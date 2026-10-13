//! Stage-2A values. Containers, real arithmetic and mutable string operations are absent.
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramId(pub(crate) u32);
impl ProgramId {
    pub fn get(self) -> u32 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(pub(crate) u32);
impl ObjectId {
    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i32),
    Undefined,
    Void,
    Bytes(Rc<[u8]>),
    Object(ObjectId),
}
impl Value {
    pub fn bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self::Bytes(Rc::from(bytes.as_ref()))
    }
    /// vm_valueType @0x0808a13c; independent of Rust enum layout.
    pub fn tag(&self) -> u8 {
        match self {
            Self::Undefined => 0,
            Self::Void => 1,
            Self::Int(_) => 2,
            Self::Bytes(_) => 4,
            Self::Object(_) => 10,
        }
    }
}
