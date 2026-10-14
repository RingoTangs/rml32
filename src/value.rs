//! Magic559 values with shared, acyclic containers.
use std::rc::Rc;

/// Structural equality preserves bits; VM numeric comparisons are separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RealBits(u32);
impl RealBits {
    pub fn from_f32(value: f32) -> Self {
        Self(value.to_bits())
    }
    pub fn to_f32(self) -> f32 {
        f32::from_bits(self.0)
    }
    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
    pub fn bits(self) -> u32 {
        self.0
    }
}

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
    Real(RealBits),
    Undefined,
    Void,
    Bytes(Rc<[u8]>),
    Object(ObjectId),
    Array(crate::container::ArrayValue),
    Mapping(crate::container::MappingValue),
}
impl Value {
    /// Construct a shared array in element order. Clones share identity.
    pub fn array(elements: Vec<Value>) -> Result<Self, crate::VmError> {
        crate::container::array(elements)
    }
    /// Construct an ordinary shared mapping; last duplicate key wins.
    /// Keys support integer values and case-sensitive raw bytes.
    pub fn mapping(entries: Vec<(Value, Value)>) -> Result<Self, crate::VmError> {
        crate::container::mapping(entries)
    }
    pub fn bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self::Bytes(Rc::from(bytes.as_ref()))
    }
    /// vm_valueType @0x0808a13c; independent of Rust enum layout.
    pub fn tag(&self) -> u8 {
        match self {
            Self::Undefined => 0,
            Self::Void => 1,
            Self::Int(_) => 2,
            Self::Real(_) => 3,
            Self::Bytes(_) => 4,
            Self::Object(_) => 10,
            Self::Array(_) => 6,
            Self::Mapping(_) => 7,
        }
    }
}
