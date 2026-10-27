//! Test-only lifecycle state attached to actual ObjectStore records and VM frames.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Alive,
    Pending,
    Queued,
    Shell,
    Freed,
}
impl State {
    pub(crate) fn valid(self) -> bool {
        matches!(self, Self::Alive | Self::Pending)
    }
    fn name(self) -> &'static str {
        match self {
            Self::Alive => "alive",
            Self::Pending => "pending",
            Self::Queued => "queued",
            Self::Shell => "shell",
            Self::Freed => "freed",
        }
    }
}
#[derive(Debug)]
pub(crate) struct Life {
    pub(crate) state: Flag<State>,
    exec: Flag<usize>,
    cleaned: Flag<bool>,
    next: RefCell<Option<(ObjectId, Rc<ObjectEntry>)>>,
    queue_owner: RefCell<Option<Owner>>,
    pub(crate) capacity: usize,
}
impl Life {
    pub(crate) fn released(&self, ordinary: usize) {
        if self.state.get() == State::Shell && ordinary == 0 {
            self.state.set(State::Freed);
        }
    }
}
#[derive(Default, Debug)]
struct Queue {
    head: RefCell<Option<(ObjectId, Rc<ObjectEntry>)>>,
}
impl Queue {
    fn push(&self, id: ObjectId, e: &Rc<ObjectEntry>) {
        let binding = e.life.borrow();
        let l = binding.as_ref().unwrap();
        assert_eq!(l.state.get(), State::Pending);
        assert!(l.queue_owner.borrow().is_none());
        *l.queue_owner.borrow_mut() =
            Owner::new(Some(e.counter.clone())).expect("reserved queue reference");
        *l.next.borrow_mut() = self.head.borrow_mut().take();
        *self.head.borrow_mut() = Some((id, e.clone()));
        l.state.set(State::Queued);
        e.status.set(Status::Invalid);
    }
    fn pop(&self) -> Option<QueueHold> {
        let (id, e) = self.head.borrow_mut().take()?;
        let binding = e.life.borrow();
        let l = binding.as_ref().unwrap();
        *self.head.borrow_mut() = l.next.borrow_mut().take();
        let owner = l.queue_owner.borrow_mut().take().expect("queue reference");
        drop(binding);
        Some(QueueHold {
            id,
            entry: e,
            owner: Some(owner),
        })
    }
    fn ids(&self) -> Vec<ObjectId> {
        let mut node = self.head.borrow().clone();
        let mut ids = Vec::new();
        let mut seen = BTreeSet::new();
        while let Some((id, e)) = node {
            assert!(seen.insert(id), "queue cycle");
            ids.push(id);
            node = e.life.borrow().as_ref().unwrap().next.borrow().clone();
        }
        ids
    }
}
pub(crate) struct QueueHold {
    id: ObjectId,
    entry: Rc<ObjectEntry>,
    owner: Option<Owner>,
}
impl Drop for QueueHold {
    fn drop(&mut self) {
        drop(self.owner.take());
        self.entry
            .life
            .borrow()
            .as_ref()
            .unwrap()
            .released(self.entry.counter.owners());
    }
}
#[derive(Debug)]
pub(crate) struct ExecLease {
    id: ObjectId,
    entry: Rc<ObjectEntry>,
    queue: Rc<Queue>,
}
impl Drop for ExecLease {
    fn drop(&mut self) {
        let binding = self.entry.life.borrow();
        let l = binding.as_ref().unwrap();
        l.exec
            .set(l.exec.get().checked_sub(1).expect("execution underflow"));
        let enqueue = l.exec.get() == 0 && l.state.get() == State::Pending;
        drop(binding);
        if enqueue {
            self.queue.push(self.id, &self.entry);
        }
    }
}
#[derive(Debug)]
pub(crate) struct StoreState {
    pub(crate) ctx: Rc<AdapterContext>,
    queue: Rc<Queue>,
    capacity: usize,
    closed: Flag<bool>,
    pub(crate) host_boundary: bool,
    pub(crate) managed_tasks: bool,
}
impl StoreState {
    pub(crate) fn new(origin: Rc<()>, limits: ValueLimits, capacity: usize) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, capacity, false, false)
    }
    pub(crate) fn new_host(origin: Rc<()>, limits: ValueLimits) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, usize::MAX, true, false)
    }
    pub(crate) fn new_managed(origin: Rc<()>, limits: ValueLimits) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, usize::MAX, true, true)
    }
    fn new_mode(
        origin: Rc<()>,
        limits: ValueLimits,
        capacity: usize,
        host_boundary: bool,
        managed_tasks: bool,
    ) -> Result<Rc<Self>> {
        if capacity < 2 {
            return Err(VmError::new(
                VmErrorKind::Limit,
                "queue reserve requires capacity >= 2",
            ));
        }
        let mut ctx = AdapterContext::new(limits);
        Rc::get_mut(&mut ctx).unwrap().origin = origin;
        Ok(Rc::new(Self {
            ctx,
            queue: Rc::new(Queue::default()),
            capacity,
            closed: Flag::new(false),
            host_boundary,
            managed_tasks,
        }))
    }
    pub(crate) fn register(&self, id: ObjectId) -> Result<()> {
        self.ctx.object(id.get())?;
        let e = self.ctx.objects.borrow()[&id].clone();
        assert!(e.life.borrow().is_none());
        *e.life.borrow_mut() = Some(Rc::new(Life {
            state: Flag::new(State::Alive),
            exec: Flag::new(0),
            cleaned: Flag::new(false),
            next: RefCell::new(None),
            queue_owner: RefCell::new(None),
            capacity: self.capacity,
        }));
        Ok(())
    }
    fn entry(&self, id: ObjectId) -> Result<Rc<ObjectEntry>> {
        self.ctx
            .objects
            .borrow()
            .get(&id)
            .cloned()
            .ok_or_else(|| invalid("unknown lifecycle object"))
    }
    pub(crate) fn state(&self, id: ObjectId) -> Result<State> {
        Ok(self.entry(id)?.life.borrow().as_ref().unwrap().state.get())
    }
    pub(crate) fn valid(&self, id: ObjectId) -> bool {
        self.state(id).is_ok_and(State::valid) && !self.closed.get()
    }
    pub(crate) fn exec(&self, id: ObjectId) -> Result<ExecLease> {
        if !self.valid(id) {
            return Err(VmError::new(
                VmErrorKind::InvalidObject,
                "invalid execution object",
            ));
        }
        let e = self.entry(id)?;
        let binding = e.life.borrow();
        let l = binding.as_ref().unwrap();
        let n = l
            .exec
            .get()
            .checked_add(1)
            .filter(|n| *n <= l.capacity)
            .ok_or_else(|| VmError::new(VmErrorKind::Limit, "execution capacity"))?;
        l.exec.set(n);
        drop(binding);
        Ok(ExecLease {
            id,
            entry: e,
            queue: self.queue.clone(),
        })
    }
    pub(crate) fn request_pending(&self, id: ObjectId) -> Result<bool> {
        if !self.valid(id) {
            return Err(VmError::new(
                VmErrorKind::InvalidObject,
                "invalid deletion object",
            ));
        }
        let e = self.entry(id)?;
        let binding = e.life.borrow();
        let l = binding.as_ref().unwrap();
        if l.exec.get() > 0 {
            l.state.set(State::Pending);
            e.status.set(Status::Pending);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    // Test policy: failed/cancelled managed work retains cleanup responsibility
    // even if the last installed frame has already returned. No fake exec lease.
    pub(crate) fn enqueue_task_cleanup(&self, id: ObjectId) {
        let entry = self.entry(id).expect("registered task object");
        let binding = entry.life.borrow();
        let life = binding.as_ref().unwrap();
        if !matches!(life.state.get(), State::Alive | State::Pending) {
            return;
        }
        life.state.set(State::Pending);
        entry.status.set(Status::Pending);
        let enqueue = life.exec.get() == 0;
        drop(binding);
        if enqueue {
            self.queue.push(id, &entry);
        }
    }
    pub(crate) fn invalidate(&self, id: ObjectId) {
        let e = self.entry(id).unwrap();
        e.status.set(Status::Invalid);
        e.life.borrow().as_ref().unwrap().state.set(State::Shell);
    }
    pub(crate) fn finish(&self, id: ObjectId) {
        let e = self.entry(id).unwrap();
        let b = e.life.borrow();
        let l = b.as_ref().unwrap();
        assert!(!l.cleaned.replace(true), "duplicate cleanup");
        l.state.set(State::Shell);
        l.released(e.counter.owners());
    }
    pub(crate) fn head(&self) -> Option<ObjectId> {
        self.queue.head.borrow().as_ref().map(|(id, _)| *id)
    }
    pub(crate) fn detach(&self) -> QueueHold {
        self.queue.pop().expect("preflight queue head")
    }
    pub(crate) fn register_function(&self, f: &FunctionValue) -> Result<()> {
        if matches!(f.target(), FunctionTarget::Host { .. }) {
            return Err(VmError::new(
                VmErrorKind::UnsupportedFeature,
                "Host target excluded",
            ));
        }
        if !self.ctx.heads.borrow().contains_key(&f.0.counter.id) {
            if !Rc::ptr_eq(&f.0.origin, &self.ctx.origin) {
                return Err(invalid("foreign function origin"));
            }
            let creator = f.owner().map(|id| self.entry(id)).transpose()?;
            self.ctx.heads.borrow_mut().insert(
                f.0.counter.id,
                Rc::new(HeadState {
                    function: Rc::downgrade(&f.0),
                    creator,
                    creator_lease: RefCell::new(None),
                    leases: Flag::new(0),
                    retired: Flag::new(false),
                }),
            );
        }
        Ok(())
    }
    pub(crate) fn cell(&self, v: Value) -> Result<Rc<Cell>> {
        if self.closed.get() {
            return Err(invalid("closed lifecycle store"));
        }
        if let Value::Function(f) = &v {
            self.register_function(f)?;
        }
        let v = match v {
            Value::Object(id) if self.state(id).is_ok_and(|s| s == State::Freed) => {
                Value::Undefined
            }
            v => v,
        };
        self.ctx.cell(&self.ctx.bound(v))
    }
    pub(crate) fn owned(&self, value: Value) -> Result<OwnedValue> {
        self.ctx.validate(&value)?;
        let mut result = OwnedValue::new(value)?;
        result.lifecycle = self.ctx.acquire(&result.value)?;
        Ok(result)
    }
    pub(crate) fn plan(&self) -> CopyPlan {
        CopyPlan::with_adapter(&self.ctx)
    }
    pub(crate) fn array(&self, values: Vec<Value>) -> Result<Value> {
        self.ctx.runtime_array(values)
    }
    pub(crate) fn mapping(&self, values: Vec<(Value, Value)>) -> Result<Value> {
        self.ctx.runtime_mapping(values)
    }
    pub(crate) fn audit(&self, frames: &[(u64, ObjectId)], installed: &[(u64, ExecLease)]) {
        self.audit_host(frames, installed, &[]);
    }
    pub(crate) fn audit_host(
        &self,
        frames: &[(u64, ObjectId)],
        installed: &[(u64, ExecLease)],
        roots: &[&OwnedValue],
    ) {
        self.audit_reachable();
        if self.managed_tasks {
            self.audit_graph_roots(roots);
        }
        self.ctx.audit_roots(&self.queue.ids(), roots);
        let mut exec = BTreeMap::new();
        for (_, id) in frames {
            *exec.entry(*id).or_insert(0) += 1;
        }
        assert_eq!(frames.len(), installed.len());
        assert_eq!(
            frames.iter().copied().collect::<BTreeMap<_, _>>(),
            installed
                .iter()
                .map(|(id, lease)| (*id, lease.id))
                .collect()
        );
        assert_eq!(
            frames.iter().map(|(id, _)| *id).collect::<BTreeSet<_>>(),
            installed.iter().map(|(id, _)| *id).collect()
        );
        let q = self.queue.ids();
        for (id, e) in self.ctx.objects.borrow().iter() {
            let binding = e.life.borrow();
            let l = binding.as_ref().unwrap();
            assert_eq!(l.exec.get(), *exec.get(id).unwrap_or(&0));
            assert_eq!(l.state.get() == State::Queued, q.contains(id));
            assert_eq!(l.queue_owner.borrow().is_some(), q.contains(id));
            if l.state.get() == State::Pending {
                assert!(l.exec.get() > 0);
            }
            if l.state.get() == State::Freed {
                assert_eq!(e.counter.owners(), 0);
                assert_eq!(l.exec.get(), 0);
                assert!(l.cleaned.get());
            }
        }
    }
    fn audit_graph_roots(&self, roots: &[&OwnedValue]) {
        let mut expected = BTreeMap::<u64, usize>::new();
        let mut visit = |v: &Value| {
            if let Some(counter) = v.counter() {
                *expected.entry(counter.id).or_default() += 1;
            }
        };
        for cell in self.ctx.cells.borrow().values().filter_map(Weak::upgrade) {
            visit(&cell.read().unwrap());
        }
        for root in self.ctx.owned.borrow().iter().filter_map(Weak::upgrade) {
            visit(&root.value);
        }
        for root in roots {
            visit(&root.value);
        }
        for counter in self.ctx.graphs.borrow().values().filter_map(Weak::upgrade) {
            assert_eq!(
                counter.owners(),
                *expected.get(&counter.id).unwrap_or(&0),
                "actual graph root audit"
            );
        }
    }
    fn audit_reachable(&self) {
        let mut cells = self
            .ctx
            .cells
            .borrow()
            .values()
            .filter_map(Weak::upgrade)
            .collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        let mut headers = BTreeSet::new();
        while let Some(c) = cells.pop() {
            if !seen.insert(c.id()) {
                continue;
            }
            assert!(
                c.adapter().is_some_and(|ctx| Rc::ptr_eq(&ctx, &self.ctx)),
                "reachable slot lacks lifecycle binding"
            );
            let v = c.read().unwrap();
            if let Some(counter) = v.counter()
                && !headers.insert(counter.id)
            {
                continue;
            }
            match v {
                Value::Array(a) => cells.extend(a.0.elements.iter().cloned()),
                Value::Mapping(m) => {
                    cells.extend(m.graph_entries().unwrap().into_iter().map(|(_, c)| c))
                }
                Value::Function(f) => cells.extend(f.0.bound.iter().cloned()),
                _ => {}
            }
        }
    }
    pub(crate) fn begin_close(&self) {
        self.closed.set(true);
        self.ctx.closed.set(true);
    }
    pub(crate) fn snapshot(&self) -> Json {
        json!({"objects":self.ctx.objects.borrow().iter().map(|(id,e)|{let b=e.life.borrow();let l=b.as_ref().unwrap();(id.get().to_string(),json!({"state":l.state.get().name(),"refs":e.counter.owners(),"execs":l.exec.get(),"cleaned":l.cleaned.get()}))}).collect::<BTreeMap<_,_>>(),"queue":self.queue.ids().iter().map(|id|id.get()).collect::<Vec<_>>()})
    }
    pub(crate) fn close(&self) {
        self.begin_close();
        while let Some(hold) = self.queue.pop() {
            self.finish(hold.id);
            drop(hold);
        }
    }
}
