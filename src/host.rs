//! Explicit synchronous synthetic host bindings; these are not native efun implementations.
use crate::{ObjectId, ProgramId, Value, VmError, VmErrorKind};
use std::{fmt, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostType {
    Any,
    Undefined,
    Void,
    Int,
    Real,
    Bytes,
    Object,
    Array,
    Mapping,
}
impl HostType {
    pub(crate) fn accepts(self, value: &Value) -> bool {
        self == Self::Any
            || matches!(
                (self, value),
                (Self::Undefined, Value::Undefined)
                    | (Self::Void, Value::Void)
                    | (Self::Int, Value::Int(_))
                    | (Self::Real, Value::Real(_))
                    | (Self::Bytes, Value::Bytes(_))
                    | (Self::Object, Value::Object(_))
                    | (Self::Array, Value::Array(_))
                    | (Self::Mapping, Value::Mapping(_))
            )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostReturn {
    Void,
    Exact(HostType),
    Mixed,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSignature {
    pub required: Vec<HostType>,
    pub optional: Vec<HostType>,
    pub rest: Option<HostType>,
    pub returns: HostReturn,
}
impl HostSignature {
    pub(crate) fn validate(&self) -> Result<(), VmError> {
        if self.required.len().saturating_add(self.optional.len()) > 127 {
            return Err(VmError::new(
                VmErrorKind::Limit,
                "host signature exceeds 127 arguments",
            ));
        }
        if matches!(
            self.returns,
            HostReturn::Exact(HostType::Any | HostType::Void)
        ) {
            return Err(VmError::new(
                VmErrorKind::InvalidProgram,
                "use Mixed or Void host return",
            ));
        }
        Ok(())
    }
    pub(crate) fn check_count(&self, argc: usize) -> Result<(), VmError> {
        if argc < self.required.len()
            || (self.rest.is_none() && argc > self.required.len() + self.optional.len())
        {
            return Err(VmError::new(
                VmErrorKind::ArgumentCount,
                "host argument count mismatch",
            ));
        }
        Ok(())
    }
    pub(crate) fn argument(&self, n: usize) -> HostType {
        self.required
            .get(n)
            .or_else(|| self.optional.get(n - self.required.len()))
            .copied()
            .or(self.rest)
            .expect("argument count was validated")
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostError {
    pub kind: VmErrorKind,
    pub message: String,
}
/// Callbacks must return normally; panics are outside the recoverable VM contract.
pub type HostCallback = Rc<dyn Fn(&HostCallContext, &[Value]) -> Result<Option<Value>, HostError>>;
#[derive(Clone)]
pub struct HostFunction {
    pub name: String,
    pub signature: HostSignature,
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

pub(crate) struct HostActivation {
    pub context: HostCallContext,
    pub function: HostFunction,
}

pub(crate) fn validate_value(value: &Value) -> Result<(), VmError> {
    match value {
        Value::Array(a) => {
            a.depth()?;
        }
        Value::Mapping(m) => {
            m.depth()?;
        }
        _ => {}
    }
    Ok(())
}
