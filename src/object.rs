//! Explicit test objects; creating an object never runs its initialization code.
use crate::{
    prepared::PreparedProgram,
    value::{ObjectId, ProgramId, Value},
    vm_error::{VmError, VmErrorKind},
};
use std::{collections::BTreeMap, rc::Rc};

/// Initialization status only; `Initialized` does not mean constructors ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitializationState {
    Manual,
    Initializing,
    Initialized,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Unmanaged,
    Constructing,
    RollingBack,
    Constructed,
}
#[derive(Debug)]
pub struct Object {
    pub(crate) program: ProgramId,
    pub(crate) initialization: InitializationState,
    pub(crate) lifecycle: LifecycleState,
    pub(crate) variables: Vec<Rc<crate::cell::Cell>>,
    #[cfg(test)]
    probe: Rc<()>,
}
impl Object {
    pub fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle
    }
    pub fn initialization_state(&self) -> InitializationState {
        self.initialization
    }
    pub fn program(&self) -> ProgramId {
        self.program
    }
    pub fn variables(&self) -> Result<Vec<Value>, VmError> {
        self.variables.iter().map(|c| c.read()).collect()
    }
}
#[derive(Debug, Default)]
pub struct ObjectStore {
    pub(crate) function_origin: Rc<()>,
    pub(crate) value_limits: Rc<crate::ValueLimits>,
    host_functions: BTreeMap<u16, crate::host::HostFunction>,
    programs: BTreeMap<ProgramId, Rc<PreparedProgram>>,
    objects: BTreeMap<ObjectId, Object>,
    globals: Option<Vec<Rc<crate::cell::Cell>>>,
    global_program: Option<ProgramId>,
    next_program: u32,
    next_object: u32,
    #[cfg(test)]
    pub(crate) test_life: Option<Rc<crate::lifecycle_adapter::runtime::StoreState>>,
}
impl ObjectStore {
    /// Register a synthetic synchronous callback, independently of Global code and variables.
    pub fn register_host_function(
        &mut self,
        slot: u16,
        function: crate::host::HostFunction,
    ) -> Result<(), VmError> {
        if slot >= 559 {
            return Err(VmError::new(
                VmErrorKind::Bounds,
                "host slot outside 0..558",
            ));
        }
        if self.host_functions.contains_key(&slot) {
            return Err(VmError::new(
                VmErrorKind::InvalidState,
                "host slot already registered",
            ));
        }
        if function.name.is_empty() || function.name.contains('\0') {
            return Err(VmError::new(
                VmErrorKind::InvalidProgram,
                "host name must be nonempty without NUL",
            ));
        }
        function.signature.validate()?;
        self.host_functions.insert(slot, function);
        Ok(())
    }
    pub fn remove_host_function(&mut self, slot: u16) -> Result<(), VmError> {
        if slot >= 559 {
            return Err(VmError::new(
                VmErrorKind::Bounds,
                "host slot outside 0..558",
            ));
        }
        self.host_functions
            .remove(&slot)
            .map(|_| ())
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "host slot is not registered"))
    }
    pub(crate) fn host_function(&self, slot: u16) -> Result<crate::host::HostFunction, VmError> {
        self.host_functions.get(&slot).cloned().ok_or_else(|| {
            VmError::new(
                VmErrorKind::UnsupportedFeature,
                format!("host slot {slot} is not registered"),
            )
        })
    }
    pub fn new() -> Self {
        Self::default()
    }
    /// Install writable synthetic slots atomically; no script code is executed.
    pub fn register_globals(&mut self, values: Vec<Value>) -> Result<(), VmError> {
        let _graph_scope = crate::graph::Scope::new(*self.value_limits);
        if self.globals.is_some() {
            return Err(VmError::new(
                VmErrorKind::InvalidState,
                "global bank is already installed",
            ));
        }
        if values.len() > crate::prepared::MAX_GLOBAL_SLOTS {
            return Err(VmError::new(
                VmErrorKind::Limit,
                "global bank exceeds 32768 slots",
            ));
        }
        crate::graph::values(&values)?;
        // Validate each value independently: the bank is not an Array nesting level.
        for value in &values {
            match value {
                Value::Array(a) => {
                    a.depth()?;
                }
                Value::Mapping(m) => {
                    m.depth()?;
                }
                _ => {}
            }
        }
        let cells: Vec<_> = values
            .into_iter()
            .map(|v| life_cell!(self, v))
            .collect::<Result<_, _>>()?;
        crate::graph::cells(&cells)?;
        self.globals = Some(cells);
        Ok(())
    }
    /// Release bank roots. References already held by a VM retain their own cells.
    pub fn remove_globals(&mut self) -> Result<(), VmError> {
        self.globals
            .take()
            .map(|_| ())
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "global bank is not installed"))
    }
    /// Shallow inspection snapshot; does not normalize deleted Object values.
    pub fn global_values(&self) -> Result<Vec<Value>, VmError> {
        self.global_cells()?
            .iter()
            .map(|cell| cell.read())
            .collect()
    }
    pub(crate) fn global_cells(&self) -> Result<&[Rc<crate::cell::Cell>], VmError> {
        self.globals
            .as_deref()
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "global bank is not installed"))
    }
    /// Select Global code independently of the variable bank. Existing frames keep their code.
    pub fn bind_global_program(&mut self, program: ProgramId) -> Result<(), VmError> {
        if self.global_program.is_some() {
            return Err(VmError::new(
                VmErrorKind::InvalidState,
                "global program is already bound",
            ));
        }
        if self.program(program)?.kind != crate::prepared::ProgramKind::Global {
            return Err(VmError::new(
                VmErrorKind::UnsupportedFeature,
                "global binding requires Global code",
            ));
        }
        self.global_program = Some(program);
        Ok(())
    }
    pub fn clear_global_program(&mut self) -> Result<(), VmError> {
        self.global_program
            .take()
            .map(|_| ())
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "global program is not bound"))
    }
    pub(crate) fn global_program(&self) -> Result<ProgramId, VmError> {
        self.global_program
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "global program is not bound"))
    }
    pub fn register_program(&mut self, program: PreparedProgram) -> Result<ProgramId, VmError> {
        #[cfg(test)]
        if self.test_life.is_some()
            && ((program.allow_host_function_values
                && !self.test_life.as_ref().unwrap().host_function_values)
                || (program.allow_host_calls && !self.test_life.as_ref().unwrap().host_boundary))
        {
            return Err(VmError::new(
                VmErrorKind::UnsupportedFeature,
                "test lifecycle excludes Host",
            ));
        }
        let id = ProgramId(self.next_program);
        self.next_program = self
            .next_program
            .checked_add(1)
            .ok_or_else(|| VmError::new(VmErrorKind::Limit, "program ID space exhausted"))?;
        self.programs.insert(id, Rc::new(program));
        Ok(id)
    }
    pub fn create(
        &mut self,
        program: ProgramId,
        initial_values: Vec<Value>,
    ) -> Result<ObjectId, VmError> {
        let _graph_scope = crate::graph::Scope::new(*self.value_limits);
        let p = self.program(program)?;
        if p.kind == crate::prepared::ProgramKind::Global {
            return Err(VmError::new(
                VmErrorKind::UnsupportedFeature,
                "Global code cannot create an object",
            ));
        }
        if initial_values.len() != p.variable_count {
            return Err(VmError::new(
                VmErrorKind::InvalidProgram,
                "explicit initial object values do not match variable capacity",
            ));
        }
        crate::graph::values(&initial_values)?;
        let variables = initial_values
            .into_iter()
            .map(|v| life_cell!(self, v))
            .collect::<Result<Vec<_>, _>>()?;
        crate::graph::cells(&variables)?;
        let id = ObjectId(self.next_object);
        self.next_object = self
            .next_object
            .checked_add(1)
            .ok_or_else(|| VmError::new(VmErrorKind::Limit, "object ID space exhausted"))?;
        #[cfg(test)]
        if let Some(life) = &self.test_life {
            life.register(id)?;
        }
        self.objects.insert(
            id,
            Object {
                program,
                #[cfg(test)]
                probe: Rc::new(()),
                initialization: InitializationState::Manual,
                lifecycle: LifecycleState::Unmanaged,
                variables,
            },
        );
        Ok(id)
    }
    pub fn get(&self, id: ObjectId) -> Result<&Object, VmError> {
        #[cfg(test)]
        if self.test_life.as_ref().is_some_and(|life| !life.valid(id)) {
            return Err(VmError::new(
                VmErrorKind::InvalidObject,
                "object is invalid",
            ));
        }
        self.objects.get(&id).ok_or_else(|| {
            let mut e = VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted");
            e.object = Some(id);
            e
        })
    }
    pub(crate) fn set_initialization(
        &mut self,
        id: ObjectId,
        state: InitializationState,
    ) -> Result<(), VmError> {
        self.objects
            .get_mut(&id)
            .ok_or_else(|| {
                VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted")
            })?
            .initialization = state;
        Ok(())
    }
    pub(crate) fn set_lifecycle(
        &mut self,
        id: ObjectId,
        state: LifecycleState,
    ) -> Result<(), VmError> {
        self.objects
            .get_mut(&id)
            .ok_or_else(|| {
                VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted")
            })?
            .lifecycle = state;
        Ok(())
    }
    pub fn remove(&mut self, id: ObjectId) -> Result<(), VmError> {
        #[cfg(test)]
        if let Some(life) = self.test_life.clone() {
            self.get(id)?;
            if life.request_pending(id)? {
                return Ok(());
            }
            let cells = self.objects[&id].variables.clone();
            crate::cell::Cell::lifecycle_clear(&cells, || life.invalidate(id))?;
            self.objects.get_mut(&id).unwrap().variables.clear();
            life.finish(id);
            return Ok(());
        }
        self.objects
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted"))
    }
    pub(crate) fn program(&self, id: ProgramId) -> Result<Rc<PreparedProgram>, VmError> {
        self.programs
            .get(&id)
            .cloned()
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidProgram, "program handle is unknown"))
    }
}

#[cfg(test)]
impl ObjectStore {
    pub(crate) fn with_test_lifecycle(limits: crate::ValueLimits) -> Result<Self, VmError> {
        Self::with_test_capacity(limits, usize::MAX)
    }
    pub(crate) fn with_test_capacity(
        limits: crate::ValueLimits,
        capacity: usize,
    ) -> Result<Self, VmError> {
        let mut store = Self::new();
        store.value_limits = Rc::new(limits);
        store.test_life = Some(crate::lifecycle_adapter::runtime::StoreState::new(
            store.function_origin.clone(),
            limits,
            capacity,
        )?);
        Ok(store)
    }
    pub(crate) fn with_test_host_lifecycle(limits: crate::ValueLimits) -> Result<Self, VmError> {
        let mut store = Self::new();
        store.value_limits = Rc::new(limits);
        store.test_life = Some(crate::lifecycle_adapter::runtime::StoreState::new_host(
            store.function_origin.clone(),
            limits,
        )?);
        Ok(store)
    }
    pub(crate) fn with_test_managed_lifecycle(limits: crate::ValueLimits) -> Result<Self, VmError> {
        let mut store = Self::new();
        store.value_limits = Rc::new(limits);
        store.test_life = Some(crate::lifecycle_adapter::runtime::StoreState::new_managed(
            store.function_origin.clone(),
            limits,
        )?);
        Ok(store)
    }
    pub(crate) fn with_test_host_function_lifecycle(
        limits: crate::ValueLimits,
    ) -> Result<Self, VmError> {
        let mut store = Self::new();
        store.value_limits = Rc::new(limits);
        store.test_life = Some(
            crate::lifecycle_adapter::runtime::StoreState::new_host_function(
                store.function_origin.clone(),
                limits,
            )?,
        );
        Ok(store)
    }
    pub(crate) fn take_host_callbacks_for_test(
        &mut self,
    ) -> BTreeMap<u16, crate::host::HostFunction> {
        std::mem::take(&mut self.host_functions)
    }
    pub(crate) fn test_cell(&self, value: Value) -> Result<Rc<crate::cell::Cell>, VmError> {
        match &self.test_life {
            Some(life) => life.cell(value),
            None => crate::cell::Cell::new(value),
        }
    }
    pub(crate) fn drain_one_for_test(&mut self) -> Result<(), VmError> {
        let life = self
            .test_life
            .clone()
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "legacy store"))?;
        let id = life
            .head()
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidState, "empty queue"))?;
        let cells = self.objects[&id].variables.clone();
        let held = crate::cell::Cell::lifecycle_clear(&cells, || life.detach())?;
        self.objects.get_mut(&id).unwrap().variables.clear();
        life.finish(id);
        drop(held);
        self.reap_for_test();
        Ok(())
    }
    pub(crate) fn reap_for_test(&mut self) {
        if let Some(life) = &self.test_life {
            self.objects.retain(|id, _| {
                life.state(*id).unwrap() != crate::lifecycle_adapter::runtime::State::Freed
            });
        }
    }
    pub(crate) fn objects_for_audit_for_test(
        &self,
        id: ObjectId,
    ) -> Option<&[Rc<crate::cell::Cell>]> {
        self.objects.get(&id).map(|o| o.variables.as_slice())
    }
    pub(crate) fn probe_for_test(&self, id: ObjectId) -> std::rc::Weak<()> {
        Rc::downgrade(&self.objects[&id].probe)
    }
    pub(crate) fn lifecycle_snapshot_for_test(&self) -> serde_json::Value {
        let mut snap = self.test_life.as_ref().unwrap().snapshot();
        snap["resident"] =
            serde_json::json!(self.objects.keys().map(|id| id.get()).collect::<Vec<_>>());
        snap
    }
}
#[cfg(test)]
impl Drop for ObjectStore {
    fn drop(&mut self) {
        if let Some(life) = &self.test_life {
            life.begin_close();
            for object in self.objects.values() {
                for cell in &object.variables {
                    cell.lifecycle_close();
                }
            }
            life.close();
        }
    }
}

#[cfg(test)]
mod global_binding_tests {
    use super::*;
    #[test]
    fn global_calls_unknown_program_binding_does_not_install_a_binding() {
        let mut store = ObjectStore::new();
        assert_eq!(
            store.bind_global_program(ProgramId(999)).unwrap_err().kind,
            VmErrorKind::InvalidProgram
        );
        assert_eq!(
            store.clear_global_program().unwrap_err().kind,
            VmErrorKind::InvalidState
        );
    }
}
