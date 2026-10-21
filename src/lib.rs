//! Read-only parsers plus a bounded Magic559 core interpreter.
//! Reference artifacts are only parsed; execution tests use synthetic programs.
mod bytes;
mod cell;
pub mod container;
pub mod error;
pub mod host;
pub mod inheritance;
pub mod lzss;
mod numeric;
pub mod object;
pub mod pak;
pub mod prepared;
pub mod program;
mod reader;
pub mod value;
pub mod vm;
pub mod vm_error;

pub use error::{ErrorKind, ParseError};
pub use host::{
    HostCallContext, HostCallback, HostError, HostFunction, HostReturn, HostSignature, HostType,
};
pub use inheritance::{ProgramDescriptor, ValidationReport, validate_inheritance};
pub use object::{InitializationState, LifecycleState, ObjectStore};
pub use pak::PakArchive;
pub use prepared::{
    GlobalBankSchema, LifecycleBinding, LinkLimits, MethodBinding, PreparedProgram, ProgramCatalog,
    prepare_constructible_program, prepare_global_program,
    prepare_global_program_with_function_values, prepare_global_program_with_host_calls,
    prepare_initializable_program, prepare_linked_program, prepare_program,
    prepare_program_with_function_values, prepare_program_with_globals,
    prepare_program_with_host_calls,
};
pub use program::{AbiProfile, ProgramImage, ProgramMode, parse_program};
pub use value::{FunctionValue, ObjectId, ProgramId, RealBits, Value};
pub use vm::{Completion, ReferenceKind, RunOutcome, StackSnapshot, Vm, VmLimits};
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
