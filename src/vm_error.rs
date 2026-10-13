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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmError {
    pub kind: VmErrorKind,
    pub source: String,
    pub program: Option<ProgramId>,
    pub object: Option<ObjectId>,
    pub function: Option<i16>,
    pub pc: Option<usize>,
    pub offset: Option<usize>,
    pub message: String,
}
impl VmError {
    pub(crate) fn new(kind: VmErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            source: String::new(),
            program: None,
            object: None,
            function: None,
            pc: None,
            offset: None,
            message: message.into(),
        }
    }
}
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} function {:?} pc {:?} offset {:?}: {:?}: {}",
            self.source, self.function, self.pc, self.offset, self.kind, self.message
        )
    }
}
impl std::error::Error for VmError {}
