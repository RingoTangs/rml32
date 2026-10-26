//! Test-only bridge to real Cell/OwnedValue/CopyPlan and acyclic containers.
use crate::{
    Value, ValueLimits, VmError, VmErrorKind,
    cell::Cell,
    copy_plan::CopyPlan,
    ownership::{Counter, OwnedValue, Owner},
    value::{CaptureProfile, FunctionData, FunctionTarget, FunctionValue, ObjectId, ProgramId},
};
use serde_json::{Value as Json, json};
use std::{
    cell::{Cell as Flag, RefCell},
    collections::{BTreeMap, BTreeSet},
    rc::{Rc, Weak},
};
fn invalid(s: &str) -> VmError {
    VmError::new(VmErrorKind::InvalidState, s)
}
type Result<T> = std::result::Result<T, VmError>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Alive,
    Pending,
    Invalid,
}
#[derive(Debug)]
struct ObjectEntry {
    counter: Rc<Counter>,
    status: Flag<Status>,
    life: RefCell<Option<Rc<runtime::Life>>>,
}
#[derive(Debug)]
pub(crate) struct ObjectLease {
    _owner: Option<Owner>,
    entry: Rc<ObjectEntry>,
}
impl ObjectLease {
    fn new(entry: &Rc<ObjectEntry>) -> Result<Self> {
        if let Some(life) = entry.life.borrow().as_ref() {
            if life.state.get() == runtime::State::Freed {
                return Err(VmError::new(
                    VmErrorKind::InvalidObject,
                    "freed object cannot acquire a lease",
                ));
            }
            let ceiling = if life.state.get().valid() {
                life.capacity - 1
            } else {
                life.capacity
            };
            if entry.counter.owners() >= ceiling {
                return Err(VmError::new(
                    VmErrorKind::Limit,
                    "ordinary capacity reserved for queue",
                ));
            }
        }
        Ok(Self {
            _owner: Owner::new(Some(entry.counter.clone()))?,
            entry: entry.clone(),
        })
    }
}
impl Drop for ObjectLease {
    fn drop(&mut self) {
        drop(self._owner.take());
        if let Some(life) = self.entry.life.borrow().as_ref() {
            life.released(self.entry.counter.owners());
        }
    }
}
pub(crate) struct HeadState {
    function: Weak<FunctionData>,
    creator: Option<Rc<ObjectEntry>>,
    creator_lease: RefCell<Option<ObjectLease>>,
    leases: Flag<usize>,
    retired: Flag<bool>,
}
impl std::fmt::Debug for HeadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeadState")
            .field("leases", &self.leases.get())
            .field("retired", &self.retired.get())
            .finish()
    }
}
#[derive(Debug)]
pub(crate) enum Lease {
    Object(ObjectLease),
    Function(Rc<HeadState>),
}
#[derive(Clone)]
pub(crate) enum LeaseView {
    Object,
    Function(Rc<HeadState>),
}
impl Lease {
    pub(crate) fn view(&self) -> LeaseView {
        match self {
            Self::Object(_) => LeaseView::Object,
            Self::Function(h) => LeaseView::Function(h.clone()),
        }
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        if let Self::Object(o) = self {
            debug_assert!(o.entry.counter.owners() > 0);
        }
        if let Self::Function(h) = self {
            h.leases
                .set(h.leases.get().checked_sub(1).expect("head lease underflow"));
            if h.leases.get() == 0 {
                assert!(!h.retired.replace(true), "head retired twice");
                if let Some(f) = h.function.upgrade() {
                    for c in &f.bound {
                        c.clear_on_owner_drop();
                    }
                }
                drop(h.creator_lease.borrow_mut().take());
            }
        }
    }
}
#[derive(Debug)]
pub(crate) struct AdapterContext {
    origin: Rc<()>,
    closed: Flag<bool>,
    objects: RefCell<BTreeMap<ObjectId, Rc<ObjectEntry>>>,
    cells: RefCell<BTreeMap<u64, Weak<Cell>>>,
    owned: RefCell<Vec<Weak<OwnedValue>>>,
    graphs: RefCell<BTreeMap<u64, Weak<Counter>>>,
    heads: RefCell<BTreeMap<u64, Rc<HeadState>>>,
    limits: ValueLimits,
}
#[derive(Clone, Debug)]
struct BoundValue {
    origin: Rc<()>,
    value: Value,
}
impl AdapterContext {
    fn new(limits: ValueLimits) -> Rc<Self> {
        Rc::new(Self {
            origin: Rc::new(()),
            closed: Flag::new(false),
            objects: RefCell::new(BTreeMap::new()),
            cells: RefCell::new(BTreeMap::new()),
            owned: RefCell::new(Vec::new()),
            graphs: RefCell::new(BTreeMap::new()),
            heads: RefCell::new(BTreeMap::new()),
            limits,
        })
    }
    fn object(self: &Rc<Self>, id: u32) -> Result<BoundValue> {
        let id = ObjectId(id);
        if !self.objects.borrow().contains_key(&id) {
            self.objects.borrow_mut().insert(
                id,
                Rc::new(ObjectEntry {
                    counter: Counter::new()?,
                    status: Flag::new(Status::Alive),
                    life: RefCell::new(None),
                }),
            );
        }
        Ok(self.bound(Value::Object(id)))
    }
    fn bound(&self, value: Value) -> BoundValue {
        BoundValue {
            origin: self.origin.clone(),
            value,
        }
    }
    fn check(&self, v: &BoundValue) -> Result<()> {
        if !Rc::ptr_eq(&self.origin, &v.origin) {
            Err(invalid("foreign bound input"))
        } else {
            self.validate(&v.value)
        }
    }
    fn validate(&self, v: &Value) -> Result<()> {
        if self.closed.get() {
            return Err(invalid("closed adapter epoch"));
        }
        match v {
            Value::Object(id) => {
                if !self.objects.borrow().contains_key(id) {
                    return Err(invalid("unknown object"));
                }
            }
            Value::Function(f) => {
                if !Rc::ptr_eq(&self.origin, &f.0.origin) {
                    return Err(invalid("foreign function"));
                }
                let heads = self.heads.borrow();
                let h = heads
                    .get(&f.0.counter.id)
                    .ok_or_else(|| invalid("unbound function"))?;
                if h.retired.get() {
                    return Err(invalid("retired function"));
                }
            }
            Value::Array(_) | Value::Mapping(_) => {
                let id = v.counter().unwrap().id;
                if !self
                    .graphs
                    .borrow()
                    .get(&id)
                    .is_some_and(|w| w.upgrade().is_some())
                {
                    return Err(invalid("legacy or foreign graph"));
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub(crate) fn register_cell(&self, c: &Rc<Cell>) {
        self.cells.borrow_mut().insert(c.id(), Rc::downgrade(c));
    }
    pub(crate) fn acquire(&self, v: &Value) -> Result<Option<Lease>> {
        self.validate(v)?;
        match v {
            Value::Object(id) => Ok(Some(Lease::Object(ObjectLease::new(
                &self.objects.borrow()[id],
            )?))),
            Value::Function(f) => {
                let h = self.heads.borrow()[&f.0.counter.id].clone();
                if f.0.counter.owners()
                    != h.leases
                        .get()
                        .checked_add(1)
                        .ok_or_else(|| invalid("head lease overflow"))?
                {
                    return Err(invalid("legacy head owners cannot be adopted"));
                }
                if h.leases.get() == 0
                    && let Some(creator) = &h.creator
                {
                    if creator.status.get() == Status::Invalid {
                        return Err(invalid("invalid creator"));
                    }
                    *h.creator_lease.borrow_mut() = Some(ObjectLease::new(creator)?);
                }
                h.leases.set(h.leases.get() + 1);
                Ok(Some(Lease::Function(h)))
            }
            _ => Ok(None),
        }
    }
    fn cell(self: &Rc<Self>, v: &BoundValue) -> Result<Rc<Cell>> {
        self.check(v)?;
        let _scope = crate::graph::Scope::new(self.limits);
        let c = Cell::new(v.value.clone())?;
        c.bind_adapter(self)?;
        Ok(c)
    }
    fn owned(self: &Rc<Self>, v: &BoundValue) -> Result<Rc<OwnedValue>> {
        self.check(v)?;
        let mut owned = OwnedValue::new(v.value.clone())?;
        owned.lifecycle = self.acquire(&owned.value)?;
        let owned = Rc::new(owned);
        self.owned.borrow_mut().push(Rc::downgrade(&owned));
        Ok(owned)
    }
    fn track_owned(&self, value: OwnedValue) -> Rc<OwnedValue> {
        let value = Rc::new(value);
        self.owned.borrow_mut().push(Rc::downgrade(&value));
        value
    }
    fn array(self: &Rc<Self>, values: &[BoundValue]) -> Result<BoundValue> {
        for v in values {
            self.check(v)?;
        }
        let _scope = crate::graph::Scope::new(self.limits);
        let value = Value::array(values.iter().map(|v| v.value.clone()).collect())?;
        let Value::Array(a) = &value else {
            unreachable!()
        };
        for c in &a.0.elements {
            c.bind_adapter(self)?;
        }
        let counter = value.counter().unwrap();
        self.graphs
            .borrow_mut()
            .insert(counter.id, Rc::downgrade(&counter));
        Ok(self.bound(value))
    }
    fn mapping(self: &Rc<Self>, values: &[(Value, BoundValue)]) -> Result<BoundValue> {
        for (_, v) in values {
            self.check(v)?;
        }
        let _scope = crate::graph::Scope::new(self.limits);
        let value = Value::mapping(
            values
                .iter()
                .map(|(k, v)| (k.clone(), v.value.clone()))
                .collect(),
        )?;
        let Value::Mapping(m) = &value else {
            unreachable!()
        };
        for (_, c) in m.graph_entries()? {
            c.bind_adapter(self)?;
        }
        let counter = value.counter().unwrap();
        self.graphs
            .borrow_mut()
            .insert(counter.id, Rc::downgrade(&counter));
        Ok(self.bound(value))
    }
    fn function(
        self: &Rc<Self>,
        creator: Option<u32>,
        values: &[BoundValue],
    ) -> Result<BoundValue> {
        let creator = creator.map(ObjectId);
        let creator_entry = creator
            .map(|id| {
                self.objects
                    .borrow()
                    .get(&id)
                    .cloned()
                    .ok_or_else(|| invalid("unknown creator"))
            })
            .transpose()?;
        if creator_entry
            .as_ref()
            .is_some_and(|e| e.status.get() == Status::Invalid)
        {
            return Err(invalid("invalid creator"));
        }
        let mut bound = Vec::new();
        for v in values {
            bound.push(self.cell(v)?);
        }
        let counter = Counter::new()?;
        let data = Rc::new(FunctionData {
            capture_profile: CaptureProfile::Container,
            origin: self.origin.clone(),
            owner: creator,
            target: if creator.is_some() {
                FunctionTarget::Object {
                    program: ProgramId(0),
                    function: 0,
                }
            } else {
                FunctionTarget::Global {
                    program: ProgramId(0),
                    function: 0,
                }
            },
            bound,
            bound_count: values.len(),
            counter: counter.clone(),
        });
        self.heads.borrow_mut().insert(
            counter.id,
            Rc::new(HeadState {
                function: Rc::downgrade(&data),
                creator: creator_entry,
                creator_lease: RefCell::new(None),
                leases: Flag::new(0),
                retired: Flag::new(false),
            }),
        );
        Ok(self.bound(Value::Function(FunctionValue(data))))
    }
    fn status(&self, id: u32, status: Status) {
        self.objects.borrow()[&ObjectId(id)].status.set(status);
    }
    fn refs(&self, id: u32) -> usize {
        self.objects.borrow()[&ObjectId(id)].counter.owners()
    }
    fn normalize(&self, v: &Value) -> Result<Value> {
        self.validate(v)?;
        let dead = match v {
            Value::Object(id) => self.objects.borrow()[id].status.get() == Status::Invalid,
            Value::Function(f) => self.heads.borrow()[&f.0.counter.id]
                .creator
                .as_ref()
                .is_some_and(|e| e.status.get() == Status::Invalid),
            _ => false,
        };
        Ok(if dead { Value::Undefined } else { v.clone() })
    }
    fn copy(self: &Rc<Self>, source: &Rc<Cell>) -> Result<Rc<Cell>> {
        let _scope = crate::graph::Scope::new(self.limits);
        let mut p = CopyPlan::with_adapter(self);
        let v = p.copy(source.clone(), |v| self.normalize(v))?;
        let out = p.output(v)?;
        p.commit()?;
        Ok(out)
    }
    fn invoke(self: &Rc<Self>, source: &Rc<Cell>) -> Result<Vec<Rc<Cell>>> {
        let Value::Function(f) = source.read()? else {
            return Err(invalid("not a function"));
        };
        self.validate(&Value::Function(f.clone()))?;
        if self.normalize(&Value::Function(f.clone()))? == Value::Undefined {
            return Err(invalid("invalid creator"));
        }
        let _scope = crate::graph::Scope::new(self.limits);
        let mut p = CopyPlan::with_adapter(self);
        let mut out = Vec::new();
        let unique = f.0.counter.owners() == 1;
        for c in &f.0.bound {
            if unique {
                out.push(p.transfer(c.clone())?);
            } else {
                let v = p.copy(c.clone(), |v| self.normalize(v))?;
                out.push(p.output(v)?);
            }
        }
        if unique {
            p.write(source.clone(), Value::Undefined)?;
        }
        p.commit()?;
        Ok(out)
    }
    fn audit(&self) {
        self.audit_extra(&[]);
    }
    fn audit_extra(&self, extra: &[ObjectId]) {
        let mut object_refs = BTreeMap::<ObjectId, usize>::new();
        let mut header_refs = BTreeMap::<u64, usize>::new();
        let mut seen = BTreeSet::new();
        let mut visit = |v: &Value| {
            if let Value::Object(id) = v {
                *object_refs.entry(*id).or_default() += 1;
            }
            if let Some(c) = v.counter() {
                *header_refs.entry(c.id).or_default() += 1;
            }
        };
        for c in self.cells.borrow().values().filter_map(Weak::upgrade) {
            if seen.insert(c.id()) {
                visit(&c.read().unwrap());
            }
        }
        for v in self.owned.borrow().iter().filter_map(Weak::upgrade) {
            visit(&v.value);
        }
        for h in self.heads.borrow().values() {
            if let Some(f) = h.function.upgrade() {
                let actual_owners = *header_refs.get(&f.counter.id).unwrap_or(&0);
                assert_eq!(f.counter.owners(), actual_owners);
                assert_eq!(h.leases.get(), actual_owners);
                assert!(Rc::ptr_eq(&f.origin, &self.origin));
                let expected_creator = actual_owners > 0 && f.owner.is_some();
                assert_eq!(
                    h.creator_lease.borrow().is_some(),
                    expected_creator,
                    "creator presence must follow actual owning slots"
                );
                if expected_creator {
                    *object_refs.entry(f.owner.unwrap()).or_default() += 1;
                }
            } else {
                assert_eq!(h.leases.get(), 0);
                assert!(h.creator_lease.borrow().is_none());
            }
        }
        for id in extra {
            *object_refs.entry(*id).or_default() += 1;
        }
        for (id, e) in self.objects.borrow().iter() {
            assert_eq!(
                e.counter.owners(),
                *object_refs.get(id).unwrap_or(&0),
                "object slot/creator audit"
            );
        }
    }
    fn snapshot(&self) -> Json {
        json!({"objects":self.objects.borrow().iter().map(|(id,e)|(id.get().to_string(),e.counter.owners())).collect::<BTreeMap<_,_>>()})
    }
}
pub(crate) fn check_cells(a: &Rc<Cell>, b: &Rc<Cell>) -> Result<()> {
    match (a.adapter(), b.adapter()) {
        (Some(a), Some(b)) if Rc::ptr_eq(&a, &b) => Ok(()),
        (None, None) if !a.adapter_bound() && !b.adapter_bound() => Ok(()),
        _ => Err(invalid("mixed transfer origins")),
    }
}
pub(crate) fn preflight_release(values: &[LeaseView], moved: &BTreeSet<u64>) -> Result<()> {
    let mut pending = BTreeMap::<u64, (Rc<HeadState>, usize)>::new();
    for v in values {
        if let LeaseView::Function(h) = v
            && let Some(f) = h.function.upgrade()
        {
            let e = pending.entry(f.counter.id).or_insert((h.clone(), 0));
            e.1 += 1;
        }
    }
    let mut visited = BTreeSet::new();
    loop {
        let next = pending
            .iter()
            .find(|(id, (h, n))| !visited.contains(*id) && h.leases.get() == *n)
            .map(|(id, (h, _))| (*id, h.clone()));
        let Some((id, h)) = next else { break };
        visited.insert(id);
        if let Some(f) = h.function.upgrade() {
            for c in &f.bound {
                if let Some(LeaseView::Function(child)) = c.adapter_release_view()? {
                    if moved.contains(&c.id()) {
                        continue;
                    }
                    if let Some(f) = child.function.upgrade() {
                        let e = pending.entry(f.counter.id).or_insert((child, 0));
                        e.1 += 1;
                    }
                }
            }
        }
    }
    Ok(())
}
// Differential driver: frozen model IDs are observation labels, never raw handles.
struct TraceDriver {
    ctx: Rc<AdapterContext>,
    roots: BTreeMap<String, Rc<Cell>>,
    headers: BTreeMap<u64, (u64, String, Weak<Counter>)>,
    next: u64,
}
impl TraceDriver {
    fn new() -> Self {
        Self {
            ctx: AdapterContext::new(ValueLimits {
                max_depth: 3,
                max_items: 4,
                max_graph_nodes: 16,
                ..ValueLimits::default()
            }),
            roots: BTreeMap::new(),
            headers: BTreeMap::new(),
            next: 1,
        }
    }
    fn root(&self, n: &str) -> Result<Rc<Cell>> {
        self.roots
            .get(n)
            .cloned()
            .ok_or_else(|| invalid("missing root"))
    }
    fn fresh(&self, n: &str) -> Result<()> {
        if self.roots.contains_key(n) {
            Err(invalid("occupied root"))
        } else {
            Ok(())
        }
    }
    fn val(&self, v: &Value) -> Json {
        match v {
            Value::Undefined => json!(["undefined", null]),
            Value::Int(n) => json!(["int", n]),
            Value::Object(id) => json!(["object", id.get().to_string()]),
            Value::Array(_) | Value::Mapping(_) | Value::Function(_) => {
                let (id, kind, _) = &self.headers[&v.counter().unwrap().id];
                json!([kind, id])
            }
            _ => panic!("outside trace grammar"),
        }
    }
    fn projection(&self) -> Json {
        self.ctx.audit();
        let roots = self
            .roots
            .iter()
            .map(|(n, c)| (n.clone(), self.val(&c.read().unwrap())))
            .collect::<BTreeMap<_, _>>();
        let names = self.roots.keys().cloned().collect::<Vec<_>>();
        let mut aliases = Vec::new();
        for (i, a) in names.iter().enumerate() {
            for b in &names[i + 1..] {
                if Rc::ptr_eq(&self.roots[a], &self.roots[b]) {
                    aliases.push(json!([a, b]));
                }
            }
        }
        let headers = self
            .headers
            .values()
            .filter_map(|(id, kind, w)| {
                w.upgrade()
                    .filter(|c| c.owners() > 0)
                    .map(|c| (id.to_string(), json!([kind, c.owners()])))
            })
            .collect::<BTreeMap<_, _>>();
        let slots = self
            .ctx
            .cells
            .borrow()
            .values()
            .filter_map(Weak::upgrade)
            .count();
        json!({"roots":roots,"aliases":aliases,"headers":headers,"refs":self.ctx.snapshot()["objects"],"slots":slots})
    }
    fn event(&mut self, a: &Json) -> Result<()> {
        let op = a["op"].as_str().unwrap();
        let name = |key: &str| a[key].as_str().unwrap();
        match op {
            "object" => {
                let id = name("id").parse::<u32>().unwrap();
                if self.ctx.objects.borrow().contains_key(&ObjectId(id)) {
                    return Err(invalid("duplicate object"));
                }
                self.ctx.object(id)?;
            }
            "delete-object" => {
                let id = name("id").parse().unwrap();
                if !self.ctx.objects.borrow().contains_key(&ObjectId(id)) {
                    return Err(invalid("missing object"));
                }
                self.ctx.status(id, Status::Invalid);
            }
            "put" => {
                let n = name("root");
                self.fresh(n)?;
                let v = match a["value"][0].as_str().unwrap() {
                    "object" => self
                        .ctx
                        .object(a["value"][1].as_str().unwrap().parse().unwrap())?,
                    "int" => self
                        .ctx
                        .bound(Value::Int(a["value"][1].as_i64().unwrap() as i32)),
                    "undefined" => self.ctx.bound(Value::Undefined),
                    _ => return Err(invalid("trace type")),
                };
                let c = self.ctx.cell(&v)?;
                self.roots.insert(n.into(), c);
            }
            "build" => {
                let n = name("root");
                self.fresh(n)?;
                let sources = a["sources"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect::<Vec<_>>();
                let mut values = Vec::new();
                for s in &sources {
                    values.push(self.ctx.bound(self.root(s)?.read()?));
                }
                let kind = name("kind");
                let v = match kind {
                    "array" => self.ctx.array(&values)?,
                    "mapping" => self.ctx.mapping(
                        &values
                            .iter()
                            .enumerate()
                            .map(|(i, v)| (Value::Int(i as i32), v.clone()))
                            .collect::<Vec<_>>(),
                    )?,
                    "function" => self.ctx.function(
                        a.get("creator")
                            .and_then(Json::as_str)
                            .map(|s| s.parse().unwrap()),
                        &values,
                    )?,
                    _ => return Err(invalid("header kind")),
                };
                let root = self.ctx.cell(&v)?;
                let counter = v.value.counter().unwrap();
                self.headers.insert(
                    counter.id,
                    (self.next, kind.into(), Rc::downgrade(&counter)),
                );
                self.next += 1;
                if a["keep_sources"] != true {
                    for s in sources {
                        self.roots.remove(s);
                    }
                }
                self.roots.insert(n.into(), root);
            }
            "copy" | "move" => {
                let n = name("root");
                self.fresh(n)?;
                let src = self.root(name("source"))?;
                let out = if op == "copy" {
                    self.ctx.copy(&src)?
                } else {
                    let mut p = CopyPlan::with_adapter(&self.ctx);
                    let out = p.transfer(src)?;
                    p.commit()?;
                    self.roots.remove(name("source"));
                    out
                };
                self.roots.insert(n.into(), out);
            }
            "alias" => {
                let n = name("root");
                self.fresh(n)?;
                self.roots.insert(n.into(), self.root(name("source"))?);
            }
            "member-ref" | "member-copy" => {
                let n = name("root");
                self.fresh(n)?;
                let v = self.root(name("source"))?.read()?;
                let i = a["index"].as_u64().unwrap() as usize;
                let c = match v {
                    Value::Array(v) => v.cell(i)?,
                    Value::Mapping(v) => v
                        .graph_entries()?
                        .get(i)
                        .map(|(_, c)| c.clone())
                        .ok_or_else(|| invalid("member index"))?,
                    _ => return Err(invalid("not a container")),
                };
                let c = if op == "member-copy" {
                    self.ctx.copy(&c)?
                } else {
                    c
                };
                self.roots.insert(n.into(), c);
            }
            "write" => {
                let mut p = CopyPlan::with_adapter(&self.ctx);
                p.write(
                    self.root(name("target"))?,
                    self.root(name("source"))?.read()?,
                )?;
                p.commit()?;
            }
            "batch-copy" => {
                let mut p = CopyPlan::with_adapter(&self.ctx);
                for (s, t) in a["sources"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .zip(a["targets"].as_array().unwrap())
                {
                    let v = p.copy(self.root(s.as_str().unwrap())?, |v| self.ctx.normalize(v))?;
                    p.write(self.root(t.as_str().unwrap())?, v)?;
                }
                p.commit()?;
            }
            "drop" => {
                if self.roots.remove(name("root")).is_none() {
                    return Err(invalid("missing root"));
                }
            }
            "invoke" => {
                let n = name("root");
                let outputs = a["outputs"].as_array().unwrap();
                for out in outputs {
                    self.fresh(out.as_str().unwrap())?;
                }
                let result = self.ctx.invoke(&self.root(n)?)?;
                if result.len() != outputs.len() {
                    return Err(invalid("output arity"));
                }
                self.roots.remove(n);
                for (name, c) in outputs.iter().zip(result) {
                    self.roots.insert(name.as_str().unwrap().into(), c);
                }
            }
            _ => return Err(invalid("outside adapter trace grammar")),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn world() -> (Rc<AdapterContext>, BoundValue, BoundValue) {
        let c = AdapterContext::new(ValueLimits::default());
        let a = c.object(0).unwrap();
        let b = c.object(1).unwrap();
        (c, a, b)
    }
    #[test]
    fn actual_object_slots_copy_move_and_take_owned() {
        let (c, a, _) = world();
        let x = c.cell(&a).unwrap();
        assert_eq!(c.refs(0), 1);
        let y = c.copy(&x).unwrap();
        assert_eq!(c.refs(0), 2);
        let mut p = CopyPlan::with_adapter(&c);
        let z = p.transfer(x.clone()).unwrap();
        p.commit().unwrap();
        assert_eq!(c.refs(0), 2);
        assert_eq!(x.read().unwrap(), Value::Undefined);
        let owned = c.track_owned(z.take_owned().unwrap());
        c.audit();
        drop(y);
        drop(owned);
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn direct_owned_value_is_a_real_lease() {
        let (c, a, _) = world();
        let x = c.owned(&a).unwrap();
        assert_eq!(c.refs(0), 1);
        c.audit();
        drop(x);
        assert_eq!(c.refs(0), 0);
    }
    #[test]
    fn creator_once_per_head_not_per_function_slot() {
        for same in [false, true] {
            let (c, a, b) = world();
            let f = c.function(Some(0), &[if same { a } else { b }]).unwrap();
            let x = c.cell(&f).unwrap();
            let y = c.copy(&x).unwrap();
            assert_eq!(c.refs(0), if same { 2 } else { 1 });
            assert_eq!(c.refs(1), if same { 0 } else { 1 });
            c.audit();
            drop(x);
            drop(y);
            assert_eq!(c.refs(0), 0);
            assert_eq!(c.refs(1), 0);
            c.audit();
        }
    }
    #[test]
    fn physical_head_view_does_not_extend_creator_or_allow_revival() {
        let (c, _, b) = world();
        let f = c.function(Some(0), &[b]).unwrap();
        let root = c.cell(&f).unwrap();
        let Value::Function(view) = f.value.clone() else {
            unreachable!()
        };
        drop(root);
        assert_eq!(c.refs(0), 0);
        assert_eq!(c.refs(1), 0);
        assert_eq!(view.0.bound[0].read().unwrap(), Value::Undefined);
        assert!(c.cell(&f).is_err());
        c.audit();
    }
    #[test]
    fn pending_is_copyable_invalid_normalizes_actual_source() {
        let (c, a, _) = world();
        let x = c.cell(&a).unwrap();
        c.status(0, Status::Pending);
        let y = c.copy(&x).unwrap();
        assert_eq!(y.read().unwrap(), Value::Object(ObjectId(0)));
        drop(y);
        c.status(0, Status::Invalid);
        let alias = x.clone();
        let y = c.copy(&x).unwrap();
        assert_eq!(alias.read().unwrap(), Value::Undefined);
        assert_eq!(y.read().unwrap(), Value::Undefined);
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn unique_expansion_moves_shared_expansion_copies_invalid_capture() {
        for shared in [false, true] {
            let (c, _, b) = world();
            let f = c.function(Some(0), &[b]).unwrap();
            let x = c.cell(&f).unwrap();
            let extra = if shared {
                Some(c.copy(&x).unwrap())
            } else {
                None
            };
            c.status(1, Status::Invalid);
            let out = c.invoke(&x).unwrap();
            assert_eq!(
                out[0].read().unwrap(),
                if shared {
                    Value::Undefined
                } else {
                    Value::Object(ObjectId(1))
                }
            );
            c.audit();
            drop(out);
            drop(extra);
            drop(x);
            assert_eq!(c.refs(0), 0);
            assert_eq!(c.refs(1), 0);
        }
    }
    #[test]
    fn shallow_container_copy_does_not_duplicate_member_object_lease() {
        let (c, a, _) = world();
        let array = c.array(&[a]).unwrap();
        let x = c.cell(&array).unwrap();
        let y = c.copy(&x).unwrap();
        assert_eq!(c.refs(0), 1);
        c.audit();
        drop(y);
        drop(x);
        drop(array);
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn shared_subcontainer_cells_are_counted_once() {
        let (c, a, _) = world();
        let child = c.array(&[a]).unwrap();
        let parent = c.array(&[child.clone(), child.clone()]).unwrap();
        let root = c.cell(&parent).unwrap();
        assert_eq!(c.refs(0), 1);
        c.audit();
        drop(root);
        drop(parent);
        drop(child);
        assert_eq!(c.refs(0), 0);
    }
    #[test]
    fn array_retained_member_keeps_payload_mapping_clears_it() {
        let (c, a, _) = world();
        let array = c.array(std::slice::from_ref(&a)).unwrap();
        let root = c.cell(&array).unwrap();
        let Value::Array(v) = &array.value else {
            unreachable!()
        };
        let member = v.cell(0).unwrap();
        drop(array);
        drop(root);
        assert_eq!(c.refs(0), 1);
        assert_eq!(member.read().unwrap(), a.value);
        drop(member);
        assert_eq!(c.refs(0), 0);
        let map = c.mapping(&[(Value::Int(2), a)]).unwrap();
        let root = c.cell(&map).unwrap();
        let Value::Mapping(m) = &map.value else {
            unreachable!()
        };
        let member = m.lookup(&Value::Int(2)).unwrap().unwrap();
        drop(map);
        drop(root);
        assert_eq!(member.read().unwrap(), Value::Undefined);
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn nested_function_container_graph_releases_in_order() {
        let (c, a, b) = world();
        let inner = c.function(Some(0), &[b]).unwrap();
        let f = c.cell(&inner).unwrap();
        let array = c.array(&[inner.clone(), a]).unwrap();
        let outer = c.function(Some(1), std::slice::from_ref(&array)).unwrap();
        let g = c.cell(&outer).unwrap();
        c.audit();
        drop(f);
        drop(inner);
        drop(array);
        drop(outer);
        drop(g);
        assert_eq!(c.refs(0), 0);
        assert_eq!(c.refs(1), 0);
        c.audit();
    }
    #[test]
    fn graph_cycle_is_rejected_before_payload_or_counts_change() {
        let (c, a, _) = world();
        let array = c.array(&[a]).unwrap();
        let root = c.cell(&array).unwrap();
        let Value::Array(v) = &array.value else {
            unreachable!()
        };
        let member = v.cell(0).unwrap();
        let before = c.snapshot();
        let mut p = CopyPlan::with_adapter(&c);
        p.write(member.clone(), array.value.clone()).unwrap();
        assert_eq!(p.commit().unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(c.snapshot(), before);
        assert_eq!(member.read().unwrap(), Value::Object(ObjectId(0)));
        drop(root);
        c.audit();
    }
    #[test]
    fn mixed_origin_and_legacy_graphs_reject() {
        let (c, a, _) = world();
        let (d, b, _) = world();
        assert!(c.cell(&b).is_err());
        let x = c.cell(&a).unwrap();
        let mut p = CopyPlan::with_adapter(&d);
        assert!(p.transfer(x).is_err());
        let legacy = c.bound(Value::array(vec![Value::Int(1)]).unwrap());
        assert!(c.cell(&legacy).is_err());
        let legacy = Cell::new(Value::Int(1)).unwrap();
        let mut p = CopyPlan::with_adapter(&c);
        assert!(p.copy(legacy, |v| Ok(v.clone())).is_err());
        c.audit();
        d.audit();
    }
    #[test]
    fn borrowed_destination_rejects_whole_copy_plan() {
        let (c, a, b) = world();
        let x = c.cell(&a).unwrap();
        let y = c.cell(&b).unwrap();
        c.status(0, Status::Invalid);
        let before = c.snapshot();
        y.with_read_borrow_for_test(|| {
            let mut p = CopyPlan::with_adapter(&c);
            let value = p.copy(x.clone(), |v| c.normalize(v)).unwrap();
            p.write(y.clone(), value).unwrap();
            assert!(p.commit().is_err());
        });
        assert_eq!(x.read().unwrap(), Value::Object(ObjectId(0)));
        assert_eq!(y.read().unwrap(), Value::Object(ObjectId(1)));
        assert_eq!(c.snapshot(), before);
        c.audit();
    }
    #[test]
    fn final_capture_borrow_rejects_before_retirement() {
        let (c, _, b) = world();
        let f = c.function(Some(0), &[b]).unwrap();
        let root = c.cell(&f).unwrap();
        let Value::Function(v) = &f.value else {
            unreachable!()
        };
        let before = c.snapshot();
        v.0.bound[0].with_read_borrow_for_test(|| {
            assert!(root.replace(Value::Undefined).is_err());
        });
        assert_eq!(c.snapshot(), before);
        c.audit();
        root.replace(Value::Undefined).unwrap();
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn duplicate_equal_writes_merge_conflicts_reject_atomically() {
        let (c, a, b) = world();
        let x = c.cell(&a).unwrap();
        let mut p = CopyPlan::with_adapter(&c);
        p.write(x.clone(), b.value.clone()).unwrap();
        p.write(x.clone(), b.value.clone()).unwrap();
        assert!(p.write(x.clone(), Value::Undefined).is_err());
        drop(p);
        assert_eq!(c.refs(0), 1);
        assert_eq!(c.refs(1), 0);
        c.audit();
    }
    #[test]
    fn allocation_and_workspace_failure_preserve_actual_sources() {
        for n in 0..7 {
            let (c, a, b) = world();
            let x = c.cell(&a).unwrap();
            let y = c.cell(&b).unwrap();
            let before = c.snapshot();
            let r = crate::copy_plan::with_workspace_failure(n, || {
                let mut p = CopyPlan::with_adapter(&c);
                p.write(y.clone(), Value::Undefined)?;
                p.transfer(x.clone())?;
                p.commit()
            });
            assert_eq!(r.unwrap_err().kind, VmErrorKind::Limit);
            assert_eq!(c.snapshot(), before);
            assert_eq!(x.read().unwrap(), a.value);
            assert_eq!(y.read().unwrap(), b.value);
            c.audit();
        }
        let (c, a, _) = world();
        let x = c.cell(&a).unwrap();
        crate::cell::with_allocation_failure(0, || {
            assert!(c.copy(&x).is_err());
        });
        assert_eq!(c.refs(0), 1);
        c.audit();
    }
    #[test]
    fn actual_graph_limits_reject_unpublished_container() {
        let c = AdapterContext::new(ValueLimits {
            max_items: 1,
            ..ValueLimits::default()
        });
        let a = c.object(0).unwrap();
        assert!(c.array(&[a.clone(), a]).is_err());
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn weak_cells_and_context_are_physically_released() {
        let (c, a, _) = world();
        let root = c.cell(&a).unwrap();
        let wc = Rc::downgrade(&c);
        let ws = Rc::downgrade(&root);
        let entry = Rc::downgrade(&c.objects.borrow()[&ObjectId(0)]);
        drop(root);
        drop(c);
        assert!(wc.upgrade().is_none());
        assert!(ws.upgrade().is_none());
        assert!(entry.upgrade().is_none());
    }
    #[test]
    fn no_adapter_context_changes_default_legacy_ownership() {
        let v = Value::Object(ObjectId(123));
        assert!(v.counter().is_none());
        let c = Cell::new(v.clone()).unwrap();
        assert!(c.adapter().is_none());
        let out = c.take_owned().unwrap();
        assert!(out.lifecycle.is_none());
        assert_eq!(out.value, v);
    }
    #[test]
    fn expired_context_cannot_be_silently_adopted_by_legacy_plan() {
        let (c, a, _) = world();
        let x = c.cell(&a).unwrap();
        drop(c);
        let mut p = CopyPlan::default();
        assert!(p.transfer(x).is_err());
    }
    #[test]
    fn raw_move_of_nested_function_does_not_require_its_capture_write_guard() {
        let (c, _, b) = world();
        let inner = c.function(Some(0), &[b]).unwrap();
        let first = c.cell(&inner).unwrap();
        let outer = c.function(Some(0), std::slice::from_ref(&inner)).unwrap();
        let root = c.cell(&outer).unwrap();
        drop(first);
        let Value::Function(view) = &inner.value else {
            unreachable!()
        };
        let out = view.0.bound[0].with_read_borrow_for_test(|| c.invoke(&root).unwrap());
        assert_eq!(c.refs(0), 1);
        c.audit();
        drop(out);
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn group_final_release_checks_all_actual_capture_borrows() {
        let (c, _, b) = world();
        let f = c.function(Some(0), &[b]).unwrap();
        let x = c.cell(&f).unwrap();
        let y = c.copy(&x).unwrap();
        let before = c.snapshot();
        let Value::Function(view) = &f.value else {
            unreachable!()
        };
        view.0.bound[0].with_read_borrow_for_test(|| {
            let mut p = CopyPlan::with_adapter(&c);
            p.write(x.clone(), Value::Undefined).unwrap();
            p.write(y.clone(), Value::Undefined).unwrap();
            assert!(p.commit().is_err());
        });
        assert_eq!(c.snapshot(), before);
        c.audit();
        drop(x);
        drop(y);
        assert_eq!(c.refs(0), 0);
        c.audit();
    }
    #[test]
    fn actual_depth_and_node_limits_reject_before_publication() {
        let c = AdapterContext::new(ValueLimits {
            max_depth: 1,
            ..ValueLimits::default()
        });
        let a = c.object(0).unwrap();
        let child = c.array(&[a]).unwrap();
        let before = c.snapshot();
        assert_eq!(
            c.array(std::slice::from_ref(&child)).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert_eq!(c.snapshot(), before);
        c.audit();
        let c = AdapterContext::new(ValueLimits {
            max_graph_nodes: 0,
            ..ValueLimits::default()
        });
        c.object(0).unwrap();
        let before = c.snapshot();
        assert_eq!(c.array(&[]).unwrap_err().kind, VmErrorKind::Limit);
        assert_eq!(c.snapshot(), before);
        c.audit();
    }
    #[test]
    fn foreign_retirement_is_rejected_at_commit() {
        let (c, a, _) = world();
        let (d, _, _) = world();
        let source = c.cell(&a).unwrap();
        let mut p = CopyPlan::with_adapter(&d);
        p.retire(&source);
        assert!(p.commit().is_err());
        assert_eq!(c.refs(0), 1);
        c.audit();
        d.audit();
    }
    #[test]
    fn independent_audit_detects_unregistered_payload_and_creator_release_corruption() {
        let (c, a, _) = world();
        let extra = c.acquire(&a.value).unwrap();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c.audit())).is_err());
        drop(extra);
        c.audit();
        let f = c.function(Some(0), &[]).unwrap();
        let root = c.cell(&f).unwrap();
        let Value::Function(view) = &f.value else {
            unreachable!()
        };
        let h = c.heads.borrow()[&view.0.counter.id].clone();
        let creator = h.creator_lease.borrow_mut().take();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c.audit())).is_err());
        *h.creator_lease.borrow_mut() = creator;
        c.audit();
        drop(root);
        *h.creator_lease.borrow_mut() =
            Some(ObjectLease::new(h.creator.as_ref().unwrap()).unwrap());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c.audit())).is_err());
        drop(h.creator_lease.borrow_mut().take());
        c.audit();
    }
    #[test]
    fn ownerless_global_head_never_acquires_a_creator_reference() {
        let (c, _, _) = world();
        let f = c.function(None, &[]).unwrap();
        let x = c.cell(&f).unwrap();
        c.status(0, Status::Invalid);
        let y = c.copy(&x).unwrap();
        assert_eq!(c.refs(0), 0);
        assert_eq!(c.refs(1), 0);
        let Value::Function(view) = &f.value else {
            unreachable!()
        };
        assert!(matches!(view.target(), FunctionTarget::Global { .. }));
        c.audit();
        drop(x);
        drop(y);
        c.audit();
    }
    #[test]
    #[ignore = "run separately by adapter_diff.py with frozen-model trace input"]
    fn replay_adapter_traces() {
        use std::io::BufRead;
        let path =
            std::env::var_os("RML32_ADAPTER_TRACES").expect("explicit temporary input required");
        let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
        let mut traces = 0;
        let mut steps = 0;
        let mut rejected = 0;
        for line in reader.lines() {
            let trace: Json = serde_json::from_str(&line.unwrap()).unwrap();
            let mut d = TraceDriver::new();
            for row in trace["steps"].as_array().unwrap() {
                let before = d.projection();
                let result = d.event(&row["action"]);
                assert_eq!(
                    result.is_ok(),
                    row["accepted"].as_bool().unwrap(),
                    "{} step {} {:?}",
                    trace["id"],
                    steps,
                    result.err()
                );
                if !row["accepted"].as_bool().unwrap() {
                    rejected += 1;
                    assert_eq!(d.projection(), before);
                }
                assert_eq!(
                    d.projection(),
                    row["expected"],
                    "{} step {}",
                    trace["id"],
                    steps
                );
                steps += 1;
            }
            let weak = Rc::downgrade(&d.ctx);
            let cells = d.ctx.cells.borrow().values().cloned().collect::<Vec<_>>();
            let entries = d
                .ctx
                .objects
                .borrow()
                .values()
                .map(Rc::downgrade)
                .collect::<Vec<_>>();
            let heads = d
                .ctx
                .heads
                .borrow()
                .values()
                .map(|h| h.function.clone())
                .collect::<Vec<_>>();
            drop(d);
            assert!(cells.iter().all(|w| w.upgrade().is_none()));
            assert!(entries.iter().all(|w| w.upgrade().is_none()));
            assert!(heads.iter().all(|w| w.upgrade().is_none()));
            assert!(weak.upgrade().is_none());
            traces += 1;
        }
        assert!(traces > 0);
        println!(
            "ADAPTER_REPLAY {}",
            json!({"traces":traces,"steps":steps,"rejected":rejected,"physical_teardowns":traces})
        );
    }
}

#[path = "lifecycle_runtime.rs"]
pub(crate) mod runtime;

impl AdapterContext {
    fn runtime_array(self: &Rc<Self>, values: Vec<Value>) -> Result<Value> {
        let _scope = crate::graph::Scope::new(self.limits);
        let cells = values
            .into_iter()
            .map(|v| self.cell(&self.bound(v)))
            .collect::<Result<Vec<_>>>()?;
        let value = crate::container::lifecycle_array(cells)?;
        let counter = value.counter().unwrap();
        self.graphs
            .borrow_mut()
            .insert(counter.id, Rc::downgrade(&counter));
        Ok(value)
    }
    fn runtime_mapping(self: &Rc<Self>, values: Vec<(Value, Value)>) -> Result<Value> {
        let _scope = crate::graph::Scope::new(self.limits);
        let cells = values
            .into_iter()
            .map(|(k, v)| Ok((k, self.cell(&self.bound(v))?)))
            .collect::<Result<Vec<_>>>()?;
        let value = crate::container::lifecycle_mapping(cells)?;
        let counter = value.counter().unwrap();
        self.graphs
            .borrow_mut()
            .insert(counter.id, Rc::downgrade(&counter));
        Ok(value)
    }
}
