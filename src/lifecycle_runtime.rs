//! Lifecycle state attached to actual ObjectStore records and VM frames.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Alive,
    Pending,
    Queued,
    Shell,
    Freed,
}

#[cfg(test)]
mod production_metadata_tests {
    use super::*;
    #[test]
    fn borrowed_metadata_preserves_scan_and_retries_without_consuming_a_unit() {
        let state = StoreState::bounded(
            Rc::new(()),
            ValueLimits::default(),
            crate::LifecycleLimits {
                max_objects: 1,
                max_function_heads: 0,
                max_ordinary_refs_per_object: 2,
                max_exec_refs_per_object: 1,
                max_owners_per_header: 1,
            },
        )
        .unwrap();
        state.register(ObjectId(0)).unwrap();
        let mut progress = crate::LifecycleProgress::new();
        let guard = state.ctx.cells.borrow_mut();
        assert!(state.metadata_one(&mut progress).is_err());
        assert_eq!(progress.metadata_examined, 0);
        assert!(!state.scan.borrow().active);
        drop(guard);
        assert!(state.metadata_one(&mut progress).unwrap().is_none());
        assert_eq!(progress.metadata_examined, 1);
        assert!(state.scan_complete());
    }
}
impl State {
    pub(crate) fn valid(self) -> bool {
        matches!(self, Self::Alive | Self::Pending)
    }
    #[cfg(test)]
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
    epoch: Rc<crate::lifecycle::Epoch>,
}
impl Life {
    pub(crate) fn released(&self, ordinary: usize) {
        if self.state.get() == State::Shell && ordinary == 0 {
            self.state.set(State::Freed);
            self.epoch.touch();
        }
    }
}
#[derive(Default, Debug)]
struct Queue {
    head: RefCell<Option<(ObjectId, Rc<ObjectEntry>)>>,
    count: Flag<usize>,
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
        l.epoch.touch();
        self.count.set(self.count.get() + 1);
        e.status.set(Status::Invalid);
    }
    fn pop(&self) -> Option<QueueHold> {
        let (id, e) = self.head.borrow_mut().take()?;
        self.count.set(self.count.get() - 1);
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
    #[cfg(test)]
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
        l.epoch.touch();
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
    pub(crate) host_function_values: bool,
    scan: RefCell<Scan>,
}
#[derive(Debug, Default)]
struct Scan {
    active: bool,
    complete: bool,
    epoch: u64,
    phase: usize,
    after: Option<u64>,
    fence: [Option<u64>; 4],
}
pub(crate) struct HeadReservation {
    ctx: Rc<AdapterContext>,
    id: u64,
    abort: bool,
}
impl HeadReservation {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.ctx.closed.get() {
            return Err(invalid("closed head reservation"));
        }
        let heads = self.ctx.heads.borrow();
        let head = heads
            .get(&self.id)
            .ok_or_else(|| invalid("expired head reservation"))?;
        if head
            .creator
            .as_ref()
            .is_some_and(|e| e.status.get() == Status::Invalid)
        {
            return Err(VmError::new(
                VmErrorKind::InvalidObject,
                "creator invalidated before publication",
            ));
        }
        Ok(())
    }
    pub(crate) fn commit(mut self) {
        self.abort = false;
    }
}
impl Drop for HeadReservation {
    fn drop(&mut self) {
        if self.abort {
            let mut heads = self.ctx.heads.borrow_mut();
            if heads.get(&self.id).is_some_and(|h| h.leases.get() == 0) {
                heads.remove(&self.id);
                self.ctx.epoch.touch();
            }
        }
    }
}
impl StoreState {
    #[cfg(test)]
    pub(crate) fn new(origin: Rc<()>, limits: ValueLimits, capacity: usize) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, capacity, false, false, false)
    }
    #[cfg(test)]
    pub(crate) fn new_host(origin: Rc<()>, limits: ValueLimits) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, usize::MAX, true, false, false)
    }
    #[cfg(test)]
    pub(crate) fn new_managed(origin: Rc<()>, limits: ValueLimits) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, usize::MAX, true, true, false)
    }
    #[cfg(test)]
    pub(crate) fn new_host_function(origin: Rc<()>, limits: ValueLimits) -> Result<Rc<Self>> {
        Self::new_mode(origin, limits, usize::MAX, true, true, true)
    }
    fn new_mode(
        origin: Rc<()>,
        limits: ValueLimits,
        capacity: usize,
        host_boundary: bool,
        managed_tasks: bool,
        host_function_values: bool,
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
            host_function_values,
            scan: RefCell::new(Scan::default()),
        }))
    }
    pub(crate) fn bounded(
        origin: Rc<()>,
        limits: ValueLimits,
        quotas: crate::LifecycleLimits,
    ) -> Result<Rc<Self>> {
        quotas.validate()?;
        let mut state = Self::new_mode(
            origin,
            limits,
            quotas.max_ordinary_refs_per_object,
            true,
            true,
            true,
        )?;
        Rc::get_mut(&mut Rc::get_mut(&mut state).unwrap().ctx)
            .unwrap()
            .quotas = Some(quotas);
        Ok(state)
    }
    pub(crate) fn register(&self, id: ObjectId) -> Result<()> {
        if self
            .ctx
            .quotas
            .is_some_and(|q| self.ctx.objects.borrow().len() >= q.max_objects)
        {
            return Err(VmError::new(VmErrorKind::Limit, "object record capacity"));
        }
        self.ctx.object(id.get())?;
        self.ctx.epoch.touch();
        let e = self.ctx.objects.borrow()[&id].clone();
        assert!(e.life.borrow().is_none());
        *e.life.borrow_mut() = Some(Rc::new(Life {
            state: Flag::new(State::Alive),
            exec: Flag::new(0),
            cleaned: Flag::new(false),
            next: RefCell::new(None),
            queue_owner: RefCell::new(None),
            capacity: self.capacity,
            epoch: self.ctx.epoch.clone(),
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
            .filter(|n| {
                *n <= self
                    .ctx
                    .quotas
                    .map_or(l.capacity, |q| q.max_exec_refs_per_object)
            })
            .ok_or_else(|| VmError::new(VmErrorKind::Limit, "execution capacity"))?;
        l.exec.set(n);
        self.ctx.epoch.touch();
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
            self.ctx.epoch.touch();
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
        self.ctx.epoch.touch();
    }
    pub(crate) fn finish(&self, id: ObjectId) {
        let e = self.entry(id).unwrap();
        let b = e.life.borrow();
        let l = b.as_ref().unwrap();
        assert!(!l.cleaned.replace(true), "duplicate cleanup");
        l.state.set(State::Shell);
        self.ctx.epoch.touch();
        l.released(e.counter.owners());
    }
    pub(crate) fn head(&self) -> Option<ObjectId> {
        self.queue.head.borrow().as_ref().map(|(id, _)| *id)
    }
    pub(crate) fn detach(&self) -> QueueHold {
        self.queue.pop().expect("preflight queue head")
    }
    pub(crate) fn register_function(&self, f: &FunctionValue) -> Result<()> {
        if matches!(f.target(), FunctionTarget::Host { .. }) && !self.host_function_values {
            return Err(VmError::new(
                VmErrorKind::UnsupportedFeature,
                "Host target excluded",
            ));
        }
        if !self.ctx.heads.borrow().contains_key(&f.0.counter.id) {
            if self
                .ctx
                .quotas
                .is_some_and(|q| self.ctx.heads.borrow().len() >= q.max_function_heads)
            {
                return Err(VmError::new(VmErrorKind::Limit, "function head capacity"));
            }
            self.ctx.epoch.touch();
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
                    epoch: self.ctx.epoch.clone(),
                }),
            );
        }
        Ok(())
    }
    pub(crate) fn reserve_function(&self, f: &FunctionValue) -> Result<HeadReservation> {
        let fresh = !self.ctx.heads.borrow().contains_key(&f.0.counter.id);
        if let Some(q) = self.ctx.quotas {
            f.0.counter.bind(&self.ctx, q.max_owners_per_header)?;
        }
        self.register_function(f)?;
        Ok(HeadReservation {
            ctx: self.ctx.clone(),
            id: f.0.counter.id,
            abort: fresh && self.ctx.quotas.is_some(),
        })
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
        if let Some(c) = v.counter()
            && let Some(q) = self.ctx.quotas
        {
            c.bind(&self.ctx, q.max_owners_per_header)?;
        }
        self.ctx.cell(&self.ctx.bound(v))
    }
    pub(crate) fn owned(&self, value: Value) -> Result<OwnedValue> {
        self.ctx.validate(&value)?;
        if let Some(c) = value.counter()
            && let Some(q) = self.ctx.quotas
        {
            c.bind(&self.ctx, q.max_owners_per_header)?;
        }
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
    #[cfg(test)]
    pub(crate) fn audit(&self, frames: &[(u64, ObjectId)], installed: &[(u64, ExecLease)]) {
        self.audit_host(frames, installed, &[]);
    }
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
    pub(crate) fn assert_host_corruption_detected(
        &self,
        f: &FunctionValue,
        roots: &[&OwnedValue],
        kind: &str,
    ) {
        let h = self.ctx.heads.borrow()[&f.0.counter.id].clone();
        let rejected = match kind {
            "missing_creator" => {
                let saved = h.creator_lease.borrow_mut().take();
                let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.audit_host(&[], &[], roots)
                }))
                .is_err();
                *h.creator_lease.borrow_mut() = saved;
                rejected
            }
            "duplicate_creator" => {
                let extra = ObjectLease::new(h.creator.as_ref().unwrap()).unwrap();
                let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.audit_host(&[], &[], roots)
                }))
                .is_err();
                drop(extra);
                rejected
            }
            "missing_capture" => {
                let cell = &f.0.bound[0];
                let saved = self.ctx.cells.borrow_mut().remove(&cell.id()).unwrap();
                let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.audit_host(&[], &[], roots)
                }))
                .is_err();
                self.ctx.cells.borrow_mut().insert(cell.id(), saved);
                rejected
            }
            "extra_exec" => {
                let extra = self.exec(f.owner().unwrap()).unwrap();
                let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.audit_host(&[], &[], roots)
                }))
                .is_err();
                drop(extra);
                rejected
            }
            _ => panic!("unknown audit corruption"),
        };
        assert!(rejected, "audit missed {kind}");
    }
    pub(crate) fn begin_close(&self) {
        self.closed.set(true);
        self.ctx.closed.set(true);
    }
    pub(crate) fn queue_count(&self) -> usize {
        self.queue.count.get()
    }
    pub(crate) fn scan_complete(&self) -> bool {
        let empty = self.ctx.objects.try_borrow().is_ok_and(|m| m.is_empty())
            && self.ctx.heads.try_borrow().is_ok_and(|m| m.is_empty())
            && self.ctx.cells.try_borrow().is_ok_and(|m| m.is_empty())
            && self.ctx.graphs.try_borrow().is_ok_and(|m| m.is_empty());
        let scan = self.scan.borrow();
        empty
            || (scan.complete
                && !self.ctx.epoch.force.get()
                && scan.epoch == self.ctx.epoch.value.get())
    }
    /// Examine one stable metadata position. No full-index snapshot is built.
    pub(crate) fn metadata_one(
        &self,
        p: &mut crate::LifecycleProgress,
    ) -> Result<Option<ObjectId>> {
        {
            let _a = self
                .ctx
                .objects
                .try_borrow_mut()
                .map_err(|_| invalid("object metadata borrowed"))?;
            let _b = self
                .ctx
                .heads
                .try_borrow_mut()
                .map_err(|_| invalid("head metadata borrowed"))?;
            let _c = self
                .ctx
                .cells
                .try_borrow_mut()
                .map_err(|_| invalid("cell metadata borrowed"))?;
            let _d = self
                .ctx
                .graphs
                .try_borrow_mut()
                .map_err(|_| invalid("graph metadata borrowed"))?;
        }
        use std::ops::Bound::{Excluded, Unbounded};
        {
            let scan = self.scan.borrow();
            if scan.complete
                && scan.epoch == self.ctx.epoch.value.get()
                && !self.ctx.epoch.force.get()
            {
                return Ok(None);
            }
        }
        let mut scan = self.scan.borrow_mut();
        if self.ctx.epoch.force.replace(false) {
            scan.active = false;
            scan.complete = false;
        }
        loop {
            if !scan.active {
                scan.active = true;
                scan.complete = false;
                scan.phase = 0;
                scan.after = None;
                scan.epoch = self.ctx.epoch.value.get();
                scan.fence = [
                    self.ctx
                        .objects
                        .borrow()
                        .last_key_value()
                        .map(|(k, _)| u64::from(k.get())),
                    self.ctx.heads.borrow().last_key_value().map(|(k, _)| *k),
                    self.ctx.cells.borrow().last_key_value().map(|(k, _)| *k),
                    self.ctx.graphs.borrow().last_key_value().map(|(k, _)| *k),
                ];
            }
            if scan.phase == 4 {
                scan.active = false;
                if scan.epoch == self.ctx.epoch.value.get() {
                    scan.complete = true;
                    return Ok(None);
                }
                continue;
            }
            let lo = scan.after.map_or(Unbounded, Excluded);
            let key = match scan.phase {
                0 => self
                    .ctx
                    .objects
                    .borrow()
                    .range((
                        scan.after
                            .map_or(Unbounded, |n| Excluded(ObjectId(n as u32))),
                        Unbounded,
                    ))
                    .next()
                    .map(|(k, _)| u64::from(k.get())),
                1 => self
                    .ctx
                    .heads
                    .borrow()
                    .range((lo, Unbounded))
                    .next()
                    .map(|(k, _)| *k),
                2 => self
                    .ctx
                    .cells
                    .borrow()
                    .range((lo, Unbounded))
                    .next()
                    .map(|(k, _)| *k),
                _ => self
                    .ctx
                    .graphs
                    .borrow()
                    .range((lo, Unbounded))
                    .next()
                    .map(|(k, _)| *k),
            };
            let Some(key) = key.filter(|key| scan.fence[scan.phase].is_some_and(|end| *key <= end))
            else {
                scan.phase += 1;
                scan.after = None;
                continue;
            };
            scan.after = Some(key);
            p.metadata_examined += 1;
            match scan.phase {
                0 => {
                    let id = ObjectId(key as u32);
                    if self.state(id) == Ok(State::Freed) {
                        self.ctx.objects.borrow_mut().remove(&id);
                        self.ctx.epoch.touch();
                        p.reaped_objects += 1;
                        return Ok(Some(id));
                    }
                }
                1 => {
                    let remove = self
                        .ctx
                        .heads
                        .borrow()
                        .get(&key)
                        .is_some_and(|h| h.leases.get() == 0 && h.function.upgrade().is_none());
                    if remove {
                        self.ctx.heads.borrow_mut().remove(&key);
                        crate::graph::prune_header(key);
                        self.ctx.epoch.touch();
                        p.reclaimed_heads += 1;
                    }
                }
                2 => {
                    if self
                        .ctx
                        .cells
                        .borrow()
                        .get(&key)
                        .is_some_and(|w| w.upgrade().is_none())
                    {
                        self.ctx.cells.borrow_mut().remove(&key);
                        crate::graph::prune_cell(key);
                        p.pruned_weak_entries += 1;
                        self.ctx.epoch.touch();
                    }
                }
                _ => {
                    if self
                        .ctx
                        .graphs
                        .borrow()
                        .get(&key)
                        .is_some_and(|w| w.upgrade().is_none())
                    {
                        self.ctx.graphs.borrow_mut().remove(&key);
                        self.ctx.containers.borrow_mut().remove(&key);
                        crate::graph::prune_header(key);
                        p.pruned_weak_entries += 1;
                        self.ctx.epoch.touch();
                    }
                }
            }
            if scan.after == scan.fence[scan.phase]
                && scan.fence[scan.phase + 1..].iter().all(Option::is_none)
            {
                scan.phase = 4;
                if scan.epoch == self.ctx.epoch.value.get() && !self.ctx.epoch.force.get() {
                    scan.complete = true;
                    scan.active = false;
                }
            }
            return Ok(None);
        }
    }
    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> Json {
        json!({"objects":self.ctx.objects.borrow().iter().map(|(id,e)|{let b=e.life.borrow();let l=b.as_ref().unwrap();(id.get().to_string(),json!({"state":l.state.get().name(),"refs":e.counter.owners(),"execs":l.exec.get(),"cleaned":l.cleaned.get()}))}).collect::<BTreeMap<_,_>>(),"queue":self.queue.ids().iter().map(|id|id.get()).collect::<Vec<_>>()})
    }
    /// Scalar-only observation; no owners, safety points or retained physical views.
    #[cfg(test)]
    pub(crate) fn contract_observation(&self) -> Json {
        let objects = self.ctx.objects.borrow().iter().map(|(id,e)| {
            let life=e.life.borrow();let life=life.as_ref().unwrap();
            (id.get().to_string(),json!({"state":life.state.get().name(),"ordinary":e.counter.owners(),"exec":life.exec.get(),"cleaned":life.cleaned.get()}))
        }).collect::<BTreeMap<_,_>>();
        let heads = self.ctx.heads.borrow().iter().map(|(id,h)|(*id,json!({"kind":"function","owners":h.leases.get(),"retired":h.retired.get()}))).chain(
            self.ctx.containers.borrow().iter().map(|(id,h)|(*id,json!({"kind":match h.header {crate::graph::WeakHeader::Array(_)=>"array",_=>"mapping"},"owners":h.leases.get(),"retired":h.retired.get()})))
        ).map(|(id,v)|(id.to_string(),v)).collect::<BTreeMap<_,_>>();
        let mut rows = Vec::new();
        for (id, e) in self.ctx.objects.borrow().iter() {
            rows.push(json!({"phase":0,"key":id.get(),"eligible":e.life.borrow().as_ref().unwrap().state.get()==State::Freed}));
        }
        for (id, h) in self.ctx.heads.borrow().iter() {
            rows.push(json!({"phase":1,"key":id,"eligible":h.leases.get()==0 && h.function.strong_count()==0}));
        }
        for (id, c) in self.ctx.cells.borrow().iter() {
            rows.push(json!({"phase":2,"key":id,"eligible":c.strong_count()==0}));
        }
        for (id, c) in self.ctx.graphs.borrow().iter() {
            rows.push(json!({"phase":3,"key":id,"eligible":c.strong_count()==0}));
        }
        let scan = self.scan.borrow();
        json!({"objects":objects,"heads":heads,"function_heads_charged":self.ctx.heads.borrow().len(),"queue":self.queue.ids().iter().map(|id|id.get()).collect::<Vec<_>>(),
            "metadata":rows,"epoch":self.ctx.epoch.value.get(),"force":self.ctx.epoch.force.get(),
            "scan":{"active":scan.active,"complete":scan.complete,"epoch":scan.epoch,"phase":scan.phase,"after":scan.after,"fence":scan.fence},"scan_complete":self.scan_complete()})
    }
    #[cfg(test)]
    pub(crate) fn contract_borrow<T>(&self, index: &str, f: impl FnOnce() -> T) -> T {
        match index {
            "objects" => {
                let _g = self.ctx.objects.borrow_mut();
                f()
            }
            "heads" => {
                let _g = self.ctx.heads.borrow_mut();
                f()
            }
            "cells" => {
                let _g = self.ctx.cells.borrow_mut();
                f()
            }
            "graphs" => {
                let _g = self.ctx.graphs.borrow_mut();
                f()
            }
            _ => f(),
        }
    }
    pub(crate) fn close(&self) {
        self.begin_close();
        while let Some(hold) = self.queue.pop() {
            self.finish(hold.id);
            drop(hold);
        }
    }
}
