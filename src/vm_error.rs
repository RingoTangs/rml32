use crate::value::{ObjectId, ProgramId};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmErrorKind {
    InvalidProgram,
    Bounds,
    StackUnderflow,
    UnsupportedFeature,
    UnsupportedType,
    Arithmetic,
    InvalidObject,
    ArgumentCount,
    Limit,
    InvalidState,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    Initialize,
    Create,
    RollbackDestruct,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleContext {
    pub phase: LifecyclePhase,
    pub constitution: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmError {
    pub kind: VmErrorKind,
    pub source: String,
    pub program: Option<ProgramId>,
    pub object: Option<ObjectId>,
    pub function: Option<i16>,
    /// Constitution of the active initialization task, including nested calls.
    pub initialization: Option<usize>,
    /// Managed construction phase; actual failing code is described by source/function/pc.
    pub lifecycle: Option<Box<LifecycleContext>>,
    pub pc: Option<usize>,
    pub offset: Option<usize>,
    /// Boxed text keeps the enlarged diagnostic compact on Result error paths.
    pub message: Box<str>,
}
impl VmError {
    pub(crate) fn new(kind: VmErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            source: String::new(),
            program: None,
            object: None,
            function: None,
            initialization: None,
            lifecycle: None,
            pc: None,
            offset: None,
            message: message.into().into_boxed_str(),
        }
    }
}
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} function {:?} initialization {:?} lifecycle {:?} pc {:?} offset {:?}: {:?}: {}",
            self.source,
            self.function,
            self.initialization,
            self.lifecycle,
            self.pc,
            self.offset,
            self.kind,
            self.message
        )
    }
}
impl std::error::Error for VmError {}
