//! The executable value boundary: explicit leases and frozen observations.
use crate::{
    PreparedProgram, RealBits, VmError, VmErrorKind, host, object,
    ownership::{Counter, OwnedValue},
    value, vm,
};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
    fmt,
    rc::Rc,
};

fn invalid(message: &str) -> VmError {
    VmError::new(VmErrorKind::InvalidState, message)
}
fn limit(message: &str) -> VmError {
    VmError::new(VmErrorKind::Limit, message)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramId {
    origin: u64,
    slot: value::ProgramId,
}
impl ProgramId {
    pub fn get(self) -> u32 {
        self.slot.get()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId {
    origin: u64,
    slot: value::ObjectId,
}
impl ObjectId {
    pub fn get(self) -> u32 {
        self.slot.get()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionTarget {
    Object { program: ProgramId, function: i16 },
    Global { program: ProgramId, function: i16 },
    Host { slot: u16 },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSnapshot {
    pub id: u64,
    pub owner: Option<ObjectId>,
    pub target: FunctionTarget,
    pub bound_count: usize,
    pub program: Option<ProgramId>,
    pub function: i16,
    pub owners: usize,
}
/// Frozen data; graph identities cannot be imported or revived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueSnapshot {
    Int(i32),
    Real(RealBits),
    Undefined,
    Void,
    Bytes(Vec<u8>),
    Object(ObjectId),
    Array {
        id: u64,
        elements: Vec<ValueSnapshot>,
    },
    Mapping {
        id: u64,
        entries: Vec<(ValueSnapshot, ValueSnapshot)>,
    },
    Function(FunctionSnapshot),
    Shared {
        id: u64,
    },
}
#[derive(Debug, Clone, Copy)]
pub enum ValueInput<'a> {
    Int(i32),
    Real(RealBits),
    Undefined,
    Void,
    Bytes(&'a [u8]),
    Object(ObjectId),
    Handle(&'a HostValue),
}
#[derive(Debug, Clone, Copy)]
pub struct ValueLimits {
    pub max_host_roots: usize,
    pub max_graph_nodes: usize,
    pub max_depth: usize,
    pub max_items: usize,
}
impl Default for ValueLimits {
    fn default() -> Self {
        Self {
            max_host_roots: 65536,
            max_graph_nodes: 262144,
            max_depth: 64,
            max_items: 65536,
        }
    }
}
#[derive(Debug)]
struct Entry {
    id: u64,
    retired: Rc<Cell<bool>>,
    payload: OwnedValue,
}
#[derive(Debug)]
struct Registry {
    identity: Rc<Counter>,
    function_origin: Rc<()>,
    alive: Cell<bool>,
    next: Cell<u64>,
    entries: RefCell<Vec<Entry>>,
    limits: Rc<ValueLimits>,
    #[cfg(test)]
    test_life: Option<Rc<crate::lifecycle_adapter::runtime::StoreState>>,
    // Entries precede the parked store so final Registry teardown releases roots first.
    #[cfg(test)]
    closed_store: RefCell<Option<object::ObjectStore>>,
}
impl Registry {
    fn program(&self, slot: value::ProgramId) -> ProgramId {
        ProgramId {
            origin: self.identity.id,
            slot,
        }
    }
    fn object(&self, slot: value::ObjectId) -> ObjectId {
        ObjectId {
            origin: self.identity.id,
            slot,
        }
    }
    fn program_input(&self, id: ProgramId) -> Result<value::ProgramId, VmError> {
        self.check()?;
        if id.origin != self.identity.id {
            return Err(invalid("program belongs to another store"));
        }
        Ok(id.slot)
    }
    fn object_input(&self, id: ObjectId) -> Result<value::ObjectId, VmError> {
        self.check()?;
        if id.origin != self.identity.id {
            return Err(invalid("object belongs to another store"));
        }
        Ok(id.slot)
    }
    fn check(&self) -> Result<(), VmError> {
        if !self.alive.get() {
            return Err(invalid("store is closed"));
        }
        if !self.drain() {
            return Err(invalid("root registry is borrowed"));
        }
        Ok(())
    }
    // Drop only sets a preallocated flag if a borrow is active. No allocation,
    // callbacks or fallible borrows are required to retire a lease.
    fn drain(&self) -> bool {
        let Ok(mut entries) = self.entries.try_borrow_mut() else {
            return false;
        };
        let mut i = 0;
        while i < entries.len() {
            if entries[i].retired.get() {
                drop(entries.swap_remove(i));
            } else {
                i += 1;
            }
        }
        #[cfg(test)]
        let release_store = !self.alive.get() && entries.is_empty();
        #[cfg(test)]
        drop(entries);
        #[cfg(test)]
        if release_store {
            let store = self.closed_store.borrow_mut().take();
            drop(store);
        }
        true
    }
    fn close(&self) {
        self.alive.set(false);
        self.drain();
    }
    fn raw(&self, handle: &HostValue) -> Result<value::Value, VmError> {
        self.check()?;
        if !std::ptr::eq(self, Rc::as_ptr(&handle.registry)) || handle.retired.get() {
            return Err(invalid(
                "value lease belongs to another store or is retired",
            ));
        }
        self.entries
            .try_borrow()
            .map_err(|_| invalid("root registry is borrowed"))?
            .iter()
            .find(|e| e.id == handle.id)
            .map(|e| e.payload.value.clone())
            .ok_or_else(|| invalid("value lease is retired"))
    }
    fn input(&self, input: ValueInput<'_>) -> Result<value::Value, VmError> {
        self.check()?;
        Ok(match input {
            ValueInput::Int(n) => value::Value::Int(n),
            ValueInput::Real(n) => value::Value::Real(n),
            ValueInput::Undefined => value::Value::Undefined,
            ValueInput::Void => value::Value::Void,
            ValueInput::Bytes(b) => value::Value::bytes(b),
            ValueInput::Object(id) => value::Value::Object(self.object_input(id)?),
            ValueInput::Handle(h) => self.raw(h)?,
        })
    }
    fn inputs(&self, inputs: &[ValueInput<'_>]) -> Result<Vec<value::Value>, VmError> {
        if inputs.len() > self.limits.max_items {
            return Err(limit("input item limit exceeded"));
        }
        inputs.iter().map(|&v| self.input(v)).collect()
    }
    fn publish(self: &Rc<Self>, value: value::Value) -> Result<HostValue, VmError> {
        let _scope = crate::graph::Scope::new(*self.limits);
        self.check()?;
        crate::graph::register(&value)?;
        crate::graph::values(std::slice::from_ref(&value))?;
        // Validate before publishing. Snapshots do not normalize source values.
        self.snapshot(&value)?;
        let id = self.next.get();
        let next = id
            .checked_add(1)
            .ok_or_else(|| limit("lease ID space exhausted"))?;
        let mut entries = self
            .entries
            .try_borrow_mut()
            .map_err(|_| invalid("root registry is borrowed"))?;
        if entries.len() >= self.limits.max_host_roots {
            return Err(limit("host root limit exceeded"));
        }
        entries
            .try_reserve(1)
            .map_err(|_| limit("host root allocation failed"))?;
        let retired = Rc::new(Cell::new(false));
        #[cfg(test)]
        let payload = self.test_owned(value)?;
        #[cfg(not(test))]
        let payload = OwnedValue::new(value)?;
        entries.push(Entry {
            id,
            retired: retired.clone(),
            payload,
        });
        self.next.set(next);
        Ok(HostValue {
            registry: self.clone(),
            id,
            retired,
        })
    }
    fn publish_cell(self: &Rc<Self>, cell: Rc<crate::cell::Cell>) -> Result<HostValue, VmError> {
        let _scope = crate::graph::Scope::new(*self.limits);
        self.check()?;
        crate::graph::values(&[cell.read()?])?;
        // Validate before publishing. Snapshots do not normalize source values.
        self.snapshot(&cell.read()?)?;
        let id = self.next.get();
        let next = id
            .checked_add(1)
            .ok_or_else(|| limit("lease ID space exhausted"))?;
        let mut entries = self
            .entries
            .try_borrow_mut()
            .map_err(|_| invalid("root registry is borrowed"))?;
        if entries.len() >= self.limits.max_host_roots {
            return Err(limit("host root limit exceeded"));
        }
        entries
            .try_reserve(1)
            .map_err(|_| limit("host root allocation failed"))?;
        let retired = Rc::new(Cell::new(false));
        let payload = cell.take_owned()?;
        entries.push(Entry {
            id,
            retired: retired.clone(),
            payload,
        });
        self.next.set(next);
        Ok(HostValue {
            registry: self.clone(),
            id,
            retired,
        })
    }
    fn snapshot(&self, value: &value::Value) -> Result<ValueSnapshot, VmError> {
        self.check()?;
        self.walk(value, 0, &mut 0, &mut BTreeSet::new())
    }
    fn snapshots(&self, values: &[value::Value]) -> Result<Vec<ValueSnapshot>, VmError> {
        self.check()?;
        if values.len() > self.limits.max_items {
            return Err(limit("observation item limit exceeded"));
        }
        let (mut work, mut seen) = (0, BTreeSet::new());
        values
            .iter()
            .map(|v| self.walk(v, 0, &mut work, &mut seen))
            .collect()
    }
    fn pairs(
        &self,
        values: &[(value::Value, value::Value)],
    ) -> Result<Vec<(ValueSnapshot, ValueSnapshot)>, VmError> {
        self.check()?;
        if values.len() > self.limits.max_items {
            return Err(limit("observation item limit exceeded"));
        }
        let (mut work, mut seen) = (0, BTreeSet::new());
        values
            .iter()
            .map(|(k, v)| {
                Ok((
                    self.walk(k, 0, &mut work, &mut seen)?,
                    self.walk(v, 0, &mut work, &mut seen)?,
                ))
            })
            .collect()
    }
    fn walk(
        &self,
        v: &value::Value,
        depth: usize,
        work: &mut usize,
        seen: &mut BTreeSet<u64>,
    ) -> Result<ValueSnapshot, VmError> {
        if depth > self.limits.max_depth {
            return Err(limit("value depth limit exceeded"));
        }
        *work = work
            .checked_add(1)
            .ok_or_else(|| limit("value graph work overflow"))?;
        if *work > self.limits.max_graph_nodes {
            return Err(limit("value graph work limit exceeded"));
        }
        if let Some(counter) = v.counter()
            && !seen.insert(counter.id)
        {
            return Ok(ValueSnapshot::Shared { id: counter.id });
        }
        Ok(match v {
            value::Value::Int(n) => ValueSnapshot::Int(*n),
            value::Value::Real(n) => ValueSnapshot::Real(*n),
            value::Value::Undefined => ValueSnapshot::Undefined,
            value::Value::Void => ValueSnapshot::Void,
            value::Value::Bytes(b) => ValueSnapshot::Bytes(b.to_vec()),
            value::Value::Object(id) => ValueSnapshot::Object(self.object(*id)),
            value::Value::Function(f) => {
                if !Rc::ptr_eq(&f.0.origin, &self.function_origin) {
                    return Err(invalid("foreign function origin"));
                }
                let target = match f.target() {
                    value::FunctionTarget::Object { program, function } => FunctionTarget::Object {
                        program: self.program(program),
                        function,
                    },
                    value::FunctionTarget::Global { program, function } => FunctionTarget::Global {
                        program: self.program(program),
                        function,
                    },
                    value::FunctionTarget::Host { slot } => FunctionTarget::Host { slot },
                };
                ValueSnapshot::Function(FunctionSnapshot {
                    id: f.0.counter.id,
                    owner: f.owner().map(|id| self.object(id)),
                    target,
                    bound_count: f.bound_count(),
                    program: f.program().map(|p| self.program(p)),
                    function: f.function(),
                    owners: f.0.counter.owners(),
                })
            }
            value::Value::Array(a) => {
                let elements = a.elements()?;
                if elements.len() > self.limits.max_items {
                    return Err(limit("array item limit exceeded"));
                }
                ValueSnapshot::Array {
                    id: a.counter().id,
                    elements: elements
                        .iter()
                        .map(|v| self.walk(v, depth + 1, work, seen))
                        .collect::<Result<_, _>>()?,
                }
            }
            value::Value::Mapping(m) => {
                let entries = m.entries()?;
                if entries.len() > self.limits.max_items {
                    return Err(limit("mapping item limit exceeded"));
                }
                ValueSnapshot::Mapping {
                    id: m.counter().id,
                    entries: entries
                        .iter()
                        .map(|(k, v)| {
                            Ok((
                                self.walk(k, depth + 1, work, seen)?,
                                self.walk(v, depth + 1, work, seen)?,
                            ))
                        })
                        .collect::<Result<_, VmError>>()?,
                }
            }
        })
    }
    fn array(self: &Rc<Self>, inputs: &[ValueInput<'_>]) -> Result<HostValue, VmError> {
        let _scope = crate::graph::Scope::new(*self.limits);
        #[cfg(test)]
        if let Some(life) = &self.test_life {
            return self.publish(life.array(self.inputs(inputs)?)?);
        }
        self.publish(value::Value::array(self.inputs(inputs)?)?)
    }
    fn mapping(
        self: &Rc<Self>,
        inputs: &[(ValueInput<'_>, ValueInput<'_>)],
    ) -> Result<HostValue, VmError> {
        let _scope = crate::graph::Scope::new(*self.limits);
        if inputs.len() > self.limits.max_items {
            return Err(limit("mapping item limit exceeded"));
        }
        let values = inputs
            .iter()
            .map(|&(k, v)| Ok((self.input(k)?, self.input(v)?)))
            .collect::<Result<_, VmError>>()?;
        #[cfg(test)]
        if let Some(life) = &self.test_life {
            return self.publish(life.mapping(values)?);
        }
        self.publish(value::Value::mapping(values)?)
    }
}
/// An explicit executable root. Copying requires `retain`; Drop retires its lease.
/// ```compile_fail
/// fn duplicate(value: &rml32::HostValue) -> rml32::HostValue { value.clone() }
/// ```
/// ```compile_fail
/// let value = rml32::HostValue { id: 1 };
/// ```
pub struct HostValue {
    registry: Rc<Registry>,
    id: u64,
    retired: Rc<Cell<bool>>,
}
impl fmt::Debug for HostValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostValue")
            .field("lease", &self.id)
            .field("closed", &!self.registry.alive.get())
            .finish()
    }
}
impl Drop for HostValue {
    fn drop(&mut self) {
        self.retired.set(true);
        self.registry.drain();
    }
}
impl HostValue {
    pub fn retain(&self) -> Result<Self, VmError> {
        self.registry.publish(self.registry.raw(self)?)
    }
    pub fn snapshot(&self) -> Result<ValueSnapshot, VmError> {
        self.registry.snapshot(&self.registry.raw(self)?)
    }
    pub fn elements(&self) -> Result<Vec<ValueSnapshot>, VmError> {
        match self.registry.raw(self)? {
            value::Value::Array(a) => self.registry.snapshots(&a.elements()?),
            _ => Err(VmError::new(VmErrorKind::UnsupportedType, "expected array")),
        }
    }
    pub fn entries(&self) -> Result<Vec<(ValueSnapshot, ValueSnapshot)>, VmError> {
        match self.registry.raw(self)? {
            value::Value::Mapping(m) => self.registry.pairs(&m.entries()?),
            _ => Err(VmError::new(
                VmErrorKind::UnsupportedType,
                "expected mapping",
            )),
        }
    }
    pub fn export_member(&self, index: usize) -> Result<HostValue, VmError> {
        match self.registry.raw(self)? {
            value::Value::Array(a) => self.registry.publish(a.cell(index)?.read()?),
            _ => Err(VmError::new(VmErrorKind::UnsupportedType, "expected array")),
        }
    }
    pub fn get(&self, key: ValueInput<'_>) -> Result<Option<ValueSnapshot>, VmError> {
        match self.registry.raw(self)? {
            value::Value::Mapping(m) => m
                .get(&self.registry.input(key)?)?
                .as_ref()
                .map(|v| self.registry.snapshot(v))
                .transpose(),
            _ => Err(VmError::new(
                VmErrorKind::UnsupportedType,
                "expected mapping",
            )),
        }
    }
    pub fn get_owned(&self, key: ValueInput<'_>) -> Result<Option<HostValue>, VmError> {
        match self.registry.raw(self)? {
            value::Value::Mapping(m) => m
                .get(&self.registry.input(key)?)?
                .map(|v| self.registry.publish(v))
                .transpose(),
            _ => Err(VmError::new(
                VmErrorKind::UnsupportedType,
                "expected mapping",
            )),
        }
    }
}

#[derive(Debug)]
pub struct ObjectStore {
    inner: Option<object::ObjectStore>,
    registry: Rc<Registry>,
}
impl Drop for ObjectStore {
    fn drop(&mut self) {
        #[cfg(test)]
        if self.registry.test_life.is_some() {
            if let Some(inner) = self.inner.take() {
                self.registry.close_store_for_test(inner);
            }
            return;
        }
        if self.inner.is_some() {
            self.registry.close();
        }
    }
}
impl ObjectStore {
    pub fn new() -> Result<Self, VmError> {
        Self::with_limits(ValueLimits::default())
    }
    pub fn with_limits(limits: ValueLimits) -> Result<Self, VmError> {
        let limits = Rc::new(limits);
        let mut inner = object::ObjectStore::new();
        inner.value_limits = limits.clone();
        let registry = Rc::new(Registry {
            identity: Counter::new()?,
            function_origin: inner.function_origin.clone(),
            alive: Cell::new(true),
            next: Cell::new(1),
            entries: RefCell::new(Vec::new()),
            #[cfg(test)]
            test_life: None,
            #[cfg(test)]
            closed_store: RefCell::new(None),
            limits,
        });
        Ok(Self {
            inner: Some(inner),
            registry,
        })
    }
    pub fn view(&self) -> StoreRef<'_> {
        StoreRef {
            inner: self.inner.as_ref().unwrap(),
            registry: &self.registry,
        }
    }
    pub fn edit(&mut self) -> StoreMut<'_> {
        StoreMut {
            inner: self.inner.as_mut().unwrap(),
            registry: &self.registry,
        }
    }
}
pub struct StoreRef<'a> {
    inner: &'a object::ObjectStore,
    registry: &'a Rc<Registry>,
}
pub struct StoreMut<'a> {
    inner: &'a mut object::ObjectStore,
    registry: &'a Rc<Registry>,
}
pub struct ObjectRef<'a> {
    inner: &'a object::Object,
    registry: &'a Rc<Registry>,
}
impl ObjectRef<'_> {
    pub fn program(&self) -> ProgramId {
        self.registry.program(self.inner.program())
    }
    pub fn lifecycle_state(&self) -> object::LifecycleState {
        self.inner.lifecycle_state()
    }
    pub fn initialization_state(&self) -> object::InitializationState {
        self.inner.initialization_state()
    }
    pub fn variables(&self) -> Result<Vec<ValueSnapshot>, VmError> {
        self.registry.snapshots(&self.inner.variables()?)
    }
    pub fn export_variable(&self, index: usize) -> Result<HostValue, VmError> {
        let v = self
            .inner
            .variables()?
            .get(index)
            .cloned()
            .ok_or_else(|| VmError::new(VmErrorKind::Bounds, "variable outside object"))?;
        self.registry.publish(v)
    }
}
impl<'a> StoreRef<'a> {
    pub fn get(&self, id: ObjectId) -> Result<ObjectRef<'a>, VmError> {
        Ok(ObjectRef {
            inner: self.inner.get(self.registry.object_input(id)?)?,
            registry: self.registry,
        })
    }
    pub fn global_values(&self) -> Result<Vec<ValueSnapshot>, VmError> {
        self.registry.snapshots(&self.inner.global_values()?)
    }
    pub fn export_global(&self, index: usize) -> Result<HostValue, VmError> {
        let v = self
            .inner
            .global_values()?
            .get(index)
            .cloned()
            .ok_or_else(|| VmError::new(VmErrorKind::Bounds, "global outside bank"))?;
        self.registry.publish(v)
    }
    pub fn value(&self, input: ValueInput<'_>) -> Result<HostValue, VmError> {
        self.registry.publish(self.registry.input(input)?)
    }
    pub fn array(&self, inputs: &[ValueInput<'_>]) -> Result<HostValue, VmError> {
        self.registry.array(inputs)
    }
    pub fn mapping(
        &self,
        inputs: &[(ValueInput<'_>, ValueInput<'_>)],
    ) -> Result<HostValue, VmError> {
        self.registry.mapping(inputs)
    }
    pub fn retain(&self, value: &HostValue) -> Result<HostValue, VmError> {
        self.registry.publish(self.registry.raw(value)?)
    }
    pub fn snapshot(&self, value: &HostValue) -> Result<ValueSnapshot, VmError> {
        self.registry.snapshot(&self.registry.raw(value)?)
    }
}
impl StoreMut<'_> {
    pub fn view(&self) -> StoreRef<'_> {
        StoreRef {
            inner: self.inner,
            registry: self.registry,
        }
    }
    pub fn register_program(&mut self, program: PreparedProgram) -> Result<ProgramId, VmError> {
        self.registry.check()?;
        Ok(self.registry.program(self.inner.register_program(program)?))
    }
    pub fn create(
        &mut self,
        program: ProgramId,
        values: &[ValueInput<'_>],
    ) -> Result<ObjectId, VmError> {
        let program = self.registry.program_input(program)?;
        let values = self.registry.inputs(values)?;
        for v in &values {
            self.registry.snapshot(v)?;
        }
        Ok(self.registry.object(self.inner.create(program, values)?))
    }
    pub fn remove(&mut self, id: ObjectId) -> Result<(), VmError> {
        self.inner.remove(self.registry.object_input(id)?)
    }
    pub fn register_globals(&mut self, values: &[ValueInput<'_>]) -> Result<(), VmError> {
        let values = self.registry.inputs(values)?;
        for v in &values {
            self.registry.snapshot(v)?;
        }
        self.inner.register_globals(values)
    }
    pub fn remove_globals(&mut self) -> Result<(), VmError> {
        self.registry.check()?;
        self.inner.remove_globals()
    }
    pub fn bind_global_program(&mut self, id: ProgramId) -> Result<(), VmError> {
        self.inner
            .bind_global_program(self.registry.program_input(id)?)
    }
    pub fn clear_global_program(&mut self) -> Result<(), VmError> {
        self.registry.check()?;
        self.inner.clear_global_program()
    }
    pub fn remove_host_function(&mut self, slot: u16) -> Result<(), VmError> {
        self.registry.check()?;
        self.inner.remove_host_function(slot)
    }
    pub fn register_host_function(
        &mut self,
        slot: u16,
        function: HostFunction,
    ) -> Result<(), VmError> {
        self.registry.check()?;
        // Weak backlink prevents registry -> payload -> host callback cycles.
        let registry = Rc::downgrade(self.registry);
        let callback = function.callback;
        self.inner.register_host_function(
            slot,
            host::HostFunction {
                name: function.name,
                signature: function.signature,
                callback: Rc::new(move |context, args| {
                    let registry = registry
                        .upgrade()
                        .ok_or_else(|| host_error(invalid("store is closed")))?;
                    registry.check().map_err(host_error)?;
                    let scope = HostScope {
                        registry: &registry,
                    };
                    let borrowed: Vec<_> = args
                        .iter()
                        .map(|value| BorrowedValue {
                            value,
                            registry: &registry,
                        })
                        .collect();
                    let context = HostCallContext {
                        slot: context.slot,
                        source: context.source.clone(),
                        program: registry.program(context.program),
                        object: context.object.map(|id| registry.object(id)),
                        function: context.function,
                        pc: context.pc,
                        offset: context.offset,
                    };
                    match callback(&context, &scope, &borrowed)? {
                        HostResult::Void => Ok(None),
                        HostResult::Borrowed(index) => {
                            let raw = args.get(index).ok_or_else(|| {
                                host_error(VmError::new(
                                    VmErrorKind::Bounds,
                                    "host return argument outside call",
                                ))
                            })?;
                            registry.snapshot(raw).map_err(host_error)?;
                            #[cfg(test)]
                            return registry
                                .test_owned(raw.clone())
                                .map(Some)
                                .map_err(host_error);
                            #[cfg(not(test))]
                            OwnedValue::new(raw.clone()).map(Some).map_err(host_error)
                        }
                        HostResult::Owned(value) => {
                            let raw = registry.raw(&value).map_err(host_error)?;
                            registry.snapshot(&raw).map_err(host_error)?;
                            #[cfg(test)]
                            return registry.test_owned(raw).map(Some).map_err(host_error);
                            #[cfg(not(test))]
                            OwnedValue::new(raw).map(Some).map_err(host_error)
                        }
                    }
                }),
            },
        )
    }
}
// Convenience methods preserve the same single checked boundary.
impl ObjectStore {
    pub fn register_program(&mut self, p: PreparedProgram) -> Result<ProgramId, VmError> {
        self.edit().register_program(p)
    }
    pub fn create(&mut self, p: ProgramId, v: &[ValueInput<'_>]) -> Result<ObjectId, VmError> {
        self.edit().create(p, v)
    }
    pub fn register_globals(&mut self, v: &[ValueInput<'_>]) -> Result<(), VmError> {
        self.edit().register_globals(v)
    }
    pub fn remove_globals(&mut self) -> Result<(), VmError> {
        self.edit().remove_globals()
    }
    pub fn bind_global_program(&mut self, p: ProgramId) -> Result<(), VmError> {
        self.edit().bind_global_program(p)
    }
    pub fn clear_global_program(&mut self) -> Result<(), VmError> {
        self.edit().clear_global_program()
    }
    pub fn remove(&mut self, o: ObjectId) -> Result<(), VmError> {
        self.edit().remove(o)
    }
    pub fn register_host_function(&mut self, s: u16, f: HostFunction) -> Result<(), VmError> {
        self.edit().register_host_function(s, f)
    }
    pub fn remove_host_function(&mut self, s: u16) -> Result<(), VmError> {
        self.edit().remove_host_function(s)
    }
    pub fn get(&self, o: ObjectId) -> Result<ObjectRef<'_>, VmError> {
        self.view().get(o)
    }
    pub fn global_values(&self) -> Result<Vec<ValueSnapshot>, VmError> {
        self.view().global_values()
    }
    pub fn export_global(&self, i: usize) -> Result<HostValue, VmError> {
        self.view().export_global(i)
    }
    pub fn value(&self, v: ValueInput<'_>) -> Result<HostValue, VmError> {
        self.view().value(v)
    }
    pub fn array(&self, v: &[ValueInput<'_>]) -> Result<HostValue, VmError> {
        self.view().array(v)
    }
    pub fn mapping(&self, v: &[(ValueInput<'_>, ValueInput<'_>)]) -> Result<HostValue, VmError> {
        self.view().mapping(v)
    }
    pub fn retain(&self, v: &HostValue) -> Result<HostValue, VmError> {
        self.view().retain(v)
    }
    pub fn snapshot(&self, v: &HostValue) -> Result<ValueSnapshot, VmError> {
        self.view().snapshot(v)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionMeta {
    Returned,
    ActionFinished,
    Initialized(ObjectId),
    Constructed(ObjectId),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    Finished(CompletionMeta),
    BudgetExhausted,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackSnapshot {
    Value(ValueSnapshot),
    Reference {
        kind: vm::ReferenceKind,
        target_id: u64,
        valid: bool,
    },
}
#[derive(Debug)]
pub struct Vm {
    inner: Option<vm::Vm>,
    registry: Rc<Registry>,
    claimed: bool,
}
impl Drop for Vm {
    fn drop(&mut self) {
        #[cfg(test)]
        if self.registry.test_life.is_some() {
            if let Some(inner) = self.inner.take() {
                self.registry.close_store_for_test(inner.into_objects());
            }
            return;
        }
        if self.inner.is_some() {
            self.registry.close();
        }
    }
}
impl Vm {
    pub fn new(mut objects: ObjectStore) -> Self {
        Self {
            inner: Some(vm::Vm::new(objects.inner.take().unwrap())),
            registry: objects.registry.clone(),
            claimed: false,
        }
    }
    pub fn with_limits(mut objects: ObjectStore, limits: vm::VmLimits) -> Self {
        Self {
            inner: Some(vm::Vm::with_limits(objects.inner.take().unwrap(), limits)),
            registry: objects.registry.clone(),
            claimed: false,
        }
    }
    pub fn objects(&self) -> StoreRef<'_> {
        StoreRef {
            inner: self.inner.as_ref().unwrap().objects(),
            registry: &self.registry,
        }
    }
    pub fn objects_mut(&mut self) -> StoreMut<'_> {
        StoreMut {
            inner: self.inner.as_mut().unwrap().objects_mut(),
            registry: &self.registry,
        }
    }
    pub fn into_objects(mut self) -> ObjectStore {
        ObjectStore {
            inner: Some(self.inner.take().unwrap().into_objects()),
            registry: self.registry.clone(),
        }
    }
    pub fn start(
        &mut self,
        object: ObjectId,
        function: i16,
        arguments: &[ValueInput<'_>],
    ) -> Result<(), VmError> {
        let object = self.registry.object_input(object)?;
        let arguments = self.registry.inputs(arguments)?;
        for v in &arguments {
            self.registry.snapshot(v)?;
        }
        self.inner
            .as_mut()
            .unwrap()
            .start(object, function, arguments)?;
        self.claimed = false;
        Ok(())
    }
    pub fn begin_initialize(&mut self, program: ProgramId) -> Result<ObjectId, VmError> {
        let program = self.registry.program_input(program)?;
        let id = self.inner.as_mut().unwrap().begin_initialize(program)?;
        self.claimed = false;
        Ok(self.registry.object(id))
    }
    pub fn begin_construct(
        &mut self,
        program: ProgramId,
        parameter: Option<ValueInput<'_>>,
    ) -> Result<ObjectId, VmError> {
        let program = self.registry.program_input(program)?;
        let parameter = parameter.map(|v| self.registry.input(v)).transpose()?;
        if let Some(v) = &parameter {
            self.registry.snapshot(v)?;
        }
        let id = self
            .inner
            .as_mut()
            .unwrap()
            .begin_construct(program, parameter)?;
        self.claimed = false;
        Ok(self.registry.object(id))
    }
    pub fn run(&mut self, budget: usize) -> Result<RunOutcome, VmError> {
        self.registry.check()?;
        let result = self.inner.as_mut().unwrap().run(budget);
        self.registry.drain();
        Ok(match result? {
            vm::RunOutcome::BudgetExhausted => RunOutcome::BudgetExhausted,
            vm::RunOutcome::Finished(c) => RunOutcome::Finished(match c {
                vm::Completion::Returned(_) => CompletionMeta::Returned,
                vm::Completion::ActionFinished => CompletionMeta::ActionFinished,
                vm::Completion::Initialized(id) => {
                    CompletionMeta::Initialized(self.registry.object(id))
                }
                vm::Completion::Constructed(id) => {
                    CompletionMeta::Constructed(self.registry.object(id))
                }
            }),
        })
    }
    pub fn take_returned(&mut self) -> Result<HostValue, VmError> {
        if self.claimed {
            return Err(invalid("return value already claimed"));
        }
        let cell = self.inner.as_ref().unwrap().returned_cell()?;
        let root = self.registry.publish_cell(cell)?;
        self.inner.as_mut().unwrap().claim_returned();
        self.claimed = true;
        Ok(root)
    }
    pub fn frame_depth(&self) -> usize {
        self.inner.as_ref().unwrap().frame_depth()
    }
    pub fn rollback_errors(&self) -> &[VmError] {
        self.inner.as_ref().unwrap().rollback_errors()
    }
    pub fn stack(&self) -> Result<Vec<StackSnapshot>, VmError> {
        self.registry.check()?;
        let stack = self.inner.as_ref().unwrap().stack()?;
        if stack.len() > self.registry.limits.max_items {
            return Err(limit("stack observation item limit exceeded"));
        }
        let (mut work, mut seen) = (0, BTreeSet::new());
        stack
            .into_iter()
            .map(|v| {
                Ok(match v {
                    vm::StackSnapshot::Value(v) => {
                        StackSnapshot::Value(self.registry.walk(&v, 0, &mut work, &mut seen)?)
                    }
                    vm::StackSnapshot::Reference {
                        kind,
                        target_id,
                        valid,
                    } => StackSnapshot::Reference {
                        kind,
                        target_id,
                        valid,
                    },
                })
            })
            .collect()
    }
}
fn host_error(e: VmError) -> host::HostError {
    host::HostError {
        kind: e.kind,
        message: e.message.into(),
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCallContext {
    pub slot: u16,
    pub source: String,
    pub program: ProgramId,
    pub object: Option<ObjectId>,
    pub function: i16,
    pub pc: usize,
    pub offset: usize,
}
/// A call-scoped view. It cannot be saved outside its callback.
pub struct BorrowedValue<'call> {
    value: &'call value::Value,
    registry: &'call Rc<Registry>,
}
impl BorrowedValue<'_> {
    pub fn snapshot(&self) -> Result<ValueSnapshot, VmError> {
        self.registry.snapshot(self.value)
    }
}
/// Pure value/root operations only; this authority cannot execute or mutate a VM.
pub struct HostScope<'call> {
    registry: &'call Rc<Registry>,
}
impl<'call> HostScope<'call> {
    pub fn retain(&self, value: &BorrowedValue<'call>) -> Result<HostValue, VmError> {
        if !Rc::ptr_eq(self.registry, value.registry) {
            return Err(invalid("borrowed value belongs to another call store"));
        }
        self.registry.publish(value.value.clone())
    }
    pub fn value(&self, input: ValueInput<'_>) -> Result<HostValue, VmError> {
        self.registry.publish(self.registry.input(input)?)
    }
    pub fn array(&self, input: &[ValueInput<'_>]) -> Result<HostValue, VmError> {
        self.registry.array(input)
    }
    pub fn mapping(
        &self,
        input: &[(ValueInput<'_>, ValueInput<'_>)],
    ) -> Result<HostValue, VmError> {
        self.registry.mapping(input)
    }
}
pub enum HostResult {
    Void,
    Borrowed(usize),
    Owned(HostValue),
}
pub type HostCallback = Rc<
    dyn for<'call> Fn(
        &HostCallContext,
        &HostScope<'call>,
        &[BorrowedValue<'call>],
    ) -> Result<HostResult, host::HostError>,
>;
#[derive(Clone)]
pub struct HostFunction {
    pub name: String,
    pub signature: host::HostSignature,
    pub callback: HostCallback,
}
impl fmt::Debug for HostFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostFunction")
            .field("name", &self.name)
            .field("signature", &self.signature)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
impl Registry {
    fn test_owned(&self, value: value::Value) -> Result<OwnedValue, VmError> {
        match &self.test_life {
            Some(life) => life.owned(value),
            None => OwnedValue::new(value),
        }
    }
    fn close_store_for_test(&self, mut store: object::ObjectStore) {
        self.alive.set(false);
        self.test_life.as_ref().unwrap().begin_close();
        // A user callback can capture a HostValue. Park the sole arena owner,
        // then drop callback bindings outside every registry borrow to break that cycle.
        let callbacks = store.take_host_callbacks_for_test();
        *self.closed_store.borrow_mut() = Some(store);
        drop(callbacks);
        self.drain();
    }
}
#[cfg(test)]
impl ObjectStore {
    pub(crate) fn with_test_host_lifecycle(limits: ValueLimits) -> Result<Self, VmError> {
        let inner = object::ObjectStore::with_test_host_lifecycle(limits)?;
        let registry = Rc::new(Registry {
            identity: Counter::new()?,
            function_origin: inner.function_origin.clone(),
            alive: Cell::new(true),
            next: Cell::new(1),
            entries: RefCell::new(Vec::new()),
            limits: inner.value_limits.clone(),
            test_life: inner.test_life.clone(),
            closed_store: RefCell::new(None),
        });
        Ok(Self {
            inner: Some(inner),
            registry,
        })
    }
}
#[cfg(test)]
impl ObjectStore {
    pub(crate) fn with_test_managed_lifecycle(limits: ValueLimits) -> Result<Self, VmError> {
        let inner = object::ObjectStore::with_test_managed_lifecycle(limits)?;
        let registry = Rc::new(Registry {
            identity: Counter::new()?,
            function_origin: inner.function_origin.clone(),
            alive: Cell::new(true),
            next: Cell::new(1),
            entries: RefCell::new(Vec::new()),
            limits: inner.value_limits.clone(),
            test_life: inner.test_life.clone(),
            closed_store: RefCell::new(None),
        });
        Ok(Self {
            inner: Some(inner),
            registry,
        })
    }
}
#[cfg(test)]
impl ObjectStore {
    pub(crate) fn with_test_host_function_lifecycle(limits: ValueLimits) -> Result<Self, VmError> {
        let inner = object::ObjectStore::with_test_host_function_lifecycle(limits)?;
        let registry = Rc::new(Registry {
            identity: Counter::new()?,
            function_origin: inner.function_origin.clone(),
            alive: Cell::new(true),
            next: Cell::new(1),
            entries: RefCell::new(Vec::new()),
            limits: inner.value_limits.clone(),
            test_life: inner.test_life.clone(),
            closed_store: RefCell::new(None),
        });
        Ok(Self {
            inner: Some(inner),
            registry,
        })
    }
}
#[cfg(test)]
#[path = "lifecycle_host_function_tests.rs"]
mod lifecycle_host_function_tests;

#[cfg(test)]
#[path = "lifecycle_managed_tests.rs"]
mod lifecycle_managed_tests;

#[cfg(test)]
#[path = "lifecycle_boundary_tests.rs"]
mod lifecycle_boundary_tests;

#[cfg(test)]
mod tests {
    use super::*;
    fn function(store: &ObjectStore) -> value::Value {
        value::Value::Function(value::FunctionValue(Rc::new(value::FunctionData {
            capture_profile: crate::value::CaptureProfile::Scalar,
            origin: store.registry.function_origin.clone(),
            owner: None,
            target: value::FunctionTarget::Host { slot: 0 },
            bound: vec![],
            bound_count: 0,
            counter: Counter::new().unwrap(),
        })))
    }
    #[test]
    fn semantic_slots_not_physical_views_or_reference_aliases_own_headers() {
        let store = ObjectStore::new().unwrap();
        let f = function(&store);
        let counter = f.counter().unwrap();
        let slot = crate::cell::Cell::new(f.clone()).unwrap();
        assert_eq!(counter.owners(), 1);
        let alias = slot.clone();
        let view = slot.read().unwrap();
        assert_eq!(counter.owners(), 1);
        let root = store.registry.publish(view).unwrap();
        assert_eq!(counter.owners(), 2);
        let snapshot = root.snapshot().unwrap();
        drop(root);
        assert_eq!(counter.owners(), 1);
        drop(slot);
        assert_eq!(counter.owners(), 1);
        drop(alias);
        assert_eq!(counter.owners(), 0);
        assert!(matches!(snapshot, ValueSnapshot::Function(_)));
    }
    #[test]
    fn root_drop_while_registry_is_borrowed_defers_release_without_panicking() {
        let store = ObjectStore::new().unwrap();
        let f = function(&store);
        let counter = f.counter().unwrap();
        let root = store.registry.publish(f).unwrap();
        let guard = store.registry.entries.borrow();
        drop(root);
        assert_eq!(counter.owners(), 1);
        assert!(guard[0].retired.get());
        assert_eq!(
            store.registry.check().unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        assert!(store.registry.input(ValueInput::Int(1)).is_err());
        drop(guard);
        let root = store.value(ValueInput::Int(4)).unwrap();
        assert_eq!(counter.owners(), 0);
        assert_eq!(store.registry.entries.borrow().len(), 1);
        drop(root);
    }
    #[test]
    fn retired_roots_are_drained_before_capacity_checks_and_ids_never_wrap() {
        let store = ObjectStore::with_limits(ValueLimits {
            max_host_roots: 1,
            ..ValueLimits::default()
        })
        .unwrap();
        let root = store.value(ValueInput::Int(1)).unwrap();
        let first = root.id;
        drop(root);
        let second = store.value(ValueInput::Int(2)).unwrap();
        assert!(second.id > first);
        drop(second);
        store.registry.next.set(u64::MAX);
        assert_eq!(
            store.value(ValueInput::Int(3)).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert!(store.registry.entries.borrow().is_empty());
        assert_eq!(store.registry.next.get(), u64::MAX);
    }
    #[test]
    fn closing_registry_does_not_prevent_structural_root_release() {
        let store = ObjectStore::new().unwrap();
        let f = function(&store);
        let counter = f.counter().unwrap();
        let root = store.registry.publish(f).unwrap();
        drop(store);
        assert_eq!(counter.owners(), 1);
        assert!(root.retain().is_err());
        drop(root);
        assert_eq!(counter.owners(), 0);
    }
    #[test]
    fn failed_slot_commit_releases_provisional_owners() {
        let store = ObjectStore::new().unwrap();
        let f = function(&store);
        let counter = f.counter().unwrap();
        let slot = crate::cell::Cell::new(f.clone()).unwrap();
        slot.with_read_borrow_for_test(|| assert!(slot.replace(f.clone()).is_err()));
        assert_eq!(counter.owners(), 1);
        drop(slot);
        assert_eq!(counter.owners(), 0);
    }
    #[test]
    fn t03_function_snapshot_does_not_borrow_or_retain_hidden_capture_slots() {
        let store = ObjectStore::new().unwrap();
        let value = function(&store);
        let value::Value::Function(mut f) = value else {
            unreachable!()
        };
        let data = Rc::get_mut(&mut f.0).unwrap();
        data.bound_count = 1;
        data.bound = vec![crate::cell::Cell::new(value::Value::Int(7)).unwrap()];
        let root = store
            .registry
            .publish(value::Value::Function(f.clone()))
            .unwrap();
        let owners = f.0.counter.owners();
        f.0.bound[0].with_write_borrow_for_test(|| {
            assert!(matches!(
                root.snapshot().unwrap(),
                ValueSnapshot::Function(_)
            ));
            assert_eq!(f.0.counter.owners(), owners);
            assert_eq!(root.retain().unwrap_err().kind, VmErrorKind::InvalidState);
        });
        assert_eq!(f.0.counter.owners(), owners);
    }
}
