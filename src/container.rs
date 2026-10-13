//! Immutable, acyclic container storage. Native mutation/refcount APIs are not exposed.
use crate::{Value, VmError, VmErrorKind};
use std::collections::BTreeMap;
use std::rc::Rc;

const MAX_ITEMS: usize = 65536;
const MAX_DEPTH: usize = 64;
#[derive(Debug)]
struct ArrayData {
    elements: Vec<Value>,
    depth: usize,
}
#[derive(Debug, Clone)]
pub struct ArrayValue(Rc<ArrayData>);
impl PartialEq for ArrayValue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for ArrayValue {}
impl ArrayValue {
    pub fn elements(&self) -> &[Value] {
        &self.0.elements
    }
    pub fn depth(&self) -> usize {
        self.0.depth
    }
}
#[derive(Debug)]
struct MappingData {
    entries: Vec<(Value, Value)>,
    index: BTreeMap<Key, usize>,
    depth: usize,
}
#[derive(Debug, Clone)]
pub struct MappingValue(Rc<MappingData>);
impl PartialEq for MappingValue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for MappingValue {}
impl MappingValue {
    pub fn entries(&self) -> &[(Value, Value)] {
        &self.0.entries
    }
    pub fn depth(&self) -> usize {
        self.0.depth
    }
    pub fn get(&self, key: &Value) -> Result<Option<&Value>, VmError> {
        Ok(self
            .0
            .index
            .get(&Key::new(key)?)
            .map(|&i| &self.0.entries[i].1))
    }
}
fn depth(value: &Value) -> usize {
    match value {
        Value::Array(a) => a.depth(),
        Value::Mapping(m) => m.depth(),
        _ => 0,
    }
}
fn validate(count: usize, depth: usize) -> Result<(), VmError> {
    if count > MAX_ITEMS || depth > MAX_DEPTH {
        return Err(VmError::new(
            VmErrorKind::Limit,
            "container exceeds offline core item/depth limit",
        ));
    }
    Ok(())
}
pub(crate) fn array(elements: Vec<Value>) -> Result<Value, VmError> {
    let depth = elements.iter().map(depth).max().unwrap_or(0) + 1;
    validate(elements.len(), depth)?;
    Ok(Value::Array(ArrayValue(Rc::new(ArrayData {
        elements,
        depth,
    }))))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Int(i32),
    Bytes(Rc<[u8]>),
}
impl Key {
    fn new(value: &Value) -> Result<Self, VmError> {
        match value {
            Value::Int(n) => Ok(Self::Int(*n)),
            Value::Bytes(b) => Ok(Self::Bytes(b.clone())),
            _ => Err(VmError::new(
                VmErrorKind::UnsupportedType,
                "mapping keys require integer or bytes",
            )),
        }
    }
}
/// Entries are consumed in encounter order; the last value for a key wins.
pub(crate) fn mapping(entries: Vec<(Value, Value)>) -> Result<Value, VmError> {
    validate(entries.len(), 1)?;
    let mut index = BTreeMap::new();
    let mut unique: Vec<(Value, Value)> = Vec::new();
    for (key, value) in entries {
        let k = Key::new(&key)?;
        if let Some(&i) = index.get(&k) {
            unique[i] = (key, value);
        } else {
            index.insert(k, unique.len());
            unique.push((key, value));
        }
    }
    let depth = unique.iter().map(|(_, v)| depth(v)).max().unwrap_or(0) + 1;
    validate(unique.len(), depth)?;
    Ok(Value::Mapping(MappingValue(Rc::new(MappingData {
        entries: unique,
        index,
        depth,
    }))))
}
