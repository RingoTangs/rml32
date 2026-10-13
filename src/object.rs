//! Explicit test objects; creating an object never runs its initialization code.
use crate::{
    prepared::PreparedProgram,
    value::{ObjectId, ProgramId, Value},
    vm_error::{VmError, VmErrorKind},
};
use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug)]
pub struct Object {
    pub(crate) program: ProgramId,
    pub(crate) variables: Vec<Value>,
}
impl Object {
    pub fn program(&self) -> ProgramId {
        self.program
    }
    pub fn variables(&self) -> &[Value] {
        &self.variables
    }
}
#[derive(Debug, Default)]
pub struct ObjectStore {
    programs: BTreeMap<ProgramId, Rc<PreparedProgram>>,
    objects: BTreeMap<ObjectId, Object>,
    next_program: u32,
    next_object: u32,
}
impl ObjectStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register_program(&mut self, program: PreparedProgram) -> Result<ProgramId, VmError> {
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
        let p = self.program(program)?;
        if initial_values.len() != p.variable_count {
            return Err(VmError::new(
                VmErrorKind::InvalidProgram,
                "explicit initial object values do not match variable capacity",
            ));
        }
        let id = ObjectId(self.next_object);
        self.next_object = self
            .next_object
            .checked_add(1)
            .ok_or_else(|| VmError::new(VmErrorKind::Limit, "object ID space exhausted"))?;
        self.objects.insert(
            id,
            Object {
                program,
                variables: initial_values,
            },
        );
        Ok(id)
    }
    pub fn get(&self, id: ObjectId) -> Result<&Object, VmError> {
        self.objects.get(&id).ok_or_else(|| {
            let mut e = VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted");
            e.object = Some(id);
            e
        })
    }
    pub fn remove(&mut self, id: ObjectId) -> Result<(), VmError> {
        self.objects
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted"))
    }
    pub(crate) fn get_mut(&mut self, id: ObjectId) -> Result<&mut Object, VmError> {
        self.objects
            .get_mut(&id)
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidObject, "object is missing or deleted"))
    }
    pub(crate) fn program(&self, id: ProgramId) -> Result<Rc<PreparedProgram>, VmError> {
        self.programs
            .get(&id)
            .cloned()
            .ok_or_else(|| VmError::new(VmErrorKind::InvalidProgram, "program handle is unknown"))
    }
}
