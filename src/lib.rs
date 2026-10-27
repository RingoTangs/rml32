//! Read-only parsers plus a bounded Magic559 core interpreter.
//! Reference artifacts are only parsed; execution tests use synthetic programs.

macro_rules! life_cell {
    ($owner:expr, $value:expr $(,)?) => {
        $owner.test_cell($value)
    };
}
macro_rules! life_plan {
    ($owner:expr) => {
        $owner.test_plan()
    };
}
macro_rules! life_array {
    ($owner:expr, $value:expr $(,)?) => {
        $owner.test_array($value)
    };
}
macro_rules! life_mapping {
    ($owner:expr, $value:expr $(,)?) => {
        $owner.test_mapping($value)
    };
}
mod lifecycle;
pub use lifecycle::{LifecycleBudget, LifecycleLimits, LifecycleProgress, LifecycleStop};

mod boundary;
mod bytes;
mod cell;
mod container;
mod copy_plan;
pub mod error;
mod graph;
mod host;
pub mod inheritance;
pub mod lzss;
mod numeric;
mod object;
mod ownership;
pub mod pak;
pub mod prepared;
pub mod program;
mod reader;
mod value;
mod vm;
pub mod vm_error;

pub use boundary::*;
pub use error::{ErrorKind, ParseError};
pub use host::{HostError, HostReturn, HostSignature, HostType};
pub use inheritance::{ProgramDescriptor, ValidationReport, validate_inheritance};
pub use object::{InitializationState, LifecycleState};
pub use pak::PakArchive;
pub use prepared::{
    GlobalBankSchema, LifecycleBinding, LinkLimits, MethodBinding, PreparedProgram, ProgramCatalog,
    prepare_constructible_program, prepare_global_program,
    prepare_global_program_with_callable_values, prepare_global_program_with_container_captures,
    prepare_global_program_with_function_values,
    prepare_global_program_with_global_function_values, prepare_global_program_with_host_calls,
    prepare_initializable_program, prepare_linked_program, prepare_program,
    prepare_program_with_callable_values, prepare_program_with_container_captures,
    prepare_program_with_function_values, prepare_program_with_global_function_values,
    prepare_program_with_globals, prepare_program_with_host_calls,
};
pub use program::{AbiProfile, ProgramImage, ProgramMode, parse_program};
pub use value::RealBits;
pub(crate) use value::{FunctionValue, Value};
pub use vm::{ReferenceKind, VmLimits};
pub use vm_error::{LifecycleContext, LifecyclePhase, VmError, VmErrorKind};

pub fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len().saturating_mul(2));
    for &b in bytes {
        s.push(DIGITS[(b >> 4) as usize] as char);
        s.push(DIGITS[(b & 15) as usize] as char);
    }
    s
}

pub fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex(&Sha256::digest(bytes))
}

/// Lossless byte string. JSON uses an explicit marker, never an inferred encoding.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ByteString {
    pub bytes_hex: String,
    #[serde(skip)]
    pub bytes: Vec<u8>,
}

impl ByteString {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            bytes_hex: hex(bytes),
            bytes: bytes.to_vec(),
        }
    }
    pub fn c_bytes(&self) -> &[u8] {
        self.bytes.split(|b| *b == 0).next().unwrap_or_default()
    }
}

#[cfg(test)]
mod regression_api {
    pub(crate) use crate::host::{HostCallContext, HostFunction};
    pub(crate) use crate::object::ObjectStore;
    pub(crate) use crate::value::{FunctionTarget, FunctionValue, ObjectId, ProgramId, Value};
    pub(crate) use crate::vm::{Completion, RunOutcome, StackSnapshot, Vm};
    pub(crate) use crate::*;
}
#[cfg(test)]
mod regression;

mod lifecycle_adapter;
