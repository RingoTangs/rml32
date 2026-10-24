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
    Function(FunctionValue),
}
impl Value {
    pub(crate) fn counter(&self) -> Option<Rc<crate::ownership::Counter>> {
        match self {
            Self::Function(f) => Some(f.0.counter.clone()),
            Self::Array(a) => Some(a.counter()),
            Self::Mapping(m) => Some(m.counter()),
            _ => None,
        }
    }
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
            Self::Function(_) => 9,
        }
    }
}

/// Immutable target identity, independent of the optional creator object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionTarget {
    Object { program: ProgramId, function: i16 },
    Global { program: ProgramId, function: i16 },
    Host { slot: u16 },
}

/// Opaque callable with scalar bound parameters.
/// Clones share identity; independent creations compare unequal.
#[derive(Clone)]
pub struct FunctionValue(pub(crate) Rc<FunctionData>);
pub(crate) struct FunctionData {
    pub(crate) origin: Rc<()>,
    pub(crate) owner: Option<ObjectId>,
    pub(crate) target: FunctionTarget,
    pub(crate) bound: Vec<Rc<crate::cell::Cell>>,
    pub(crate) bound_count: usize,
    pub(crate) counter: Rc<crate::ownership::Counter>,
}
impl Drop for FunctionData {
    fn drop(&mut self) {
        for cell in &self.bound {
            cell.clear_on_owner_drop();
        }
    }
}
impl FunctionValue {
    /// Object active at creation, independent of the target; absent in Global context.
    pub fn owner(&self) -> Option<ObjectId> {
        self.0.owner
    }
    /// Target bytecode identity; Host targets have no program.
    pub fn program(&self) -> Option<ProgramId> {
        match self.0.target {
            FunctionTarget::Object { program, .. } | FunctionTarget::Global { program, .. } => {
                Some(program)
            }
            FunctionTarget::Host { .. } => None,
        }
    }
    /// Saved target identity; inspection does not resolve current code or host bindings.
    pub fn target(&self) -> FunctionTarget {
        self.0.target
    }
    pub fn function(&self) -> i16 {
        match self.0.target {
            FunctionTarget::Object { function, .. } | FunctionTarget::Global { function, .. } => {
                function
            }
            FunctionTarget::Host { slot } => slot as i16,
        }
    }
    pub fn bound_count(&self) -> usize {
        self.0.bound_count
    }
}
impl PartialEq for FunctionValue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for FunctionValue {}
impl std::fmt::Debug for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionValue")
            .field("owner", &self.owner())
            .field("target", &self.target())
            .field("bound_count", &self.bound_count())
            .finish()
    }
}
