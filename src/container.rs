//! Shared acyclic containers with private stable slots and owned read snapshots.
use crate::{Value, VmError, VmErrorKind, cell::Cell};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
const MAX_ITEMS: usize = 65536;
const MAX_DEPTH: usize = 64;
#[derive(Debug)]
struct ArrayData {
    elements: Vec<Rc<Cell>>,
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
    #[cfg(test)]
    pub(crate) fn header_refs(&self) -> usize {
        Rc::strong_count(&self.0)
    }

    /// Shallow snapshot: nested containers continue to share their identity.
    pub fn elements(&self) -> Result<Vec<Value>, VmError> {
        self.0.elements.iter().map(|c| c.read()).collect()
    }
    pub fn depth(&self) -> Result<usize, VmError> {
        depth(&Value::Array(self.clone()), &mut BTreeMap::new())
    }
    pub(crate) fn len(&self) -> usize {
        self.0.elements.len()
    }
    pub(crate) fn cell(&self, index: usize) -> Result<Rc<Cell>, VmError> {
        self.0
            .elements
            .get(index)
            .cloned()
            .ok_or_else(|| VmError::new(VmErrorKind::Bounds, "array index outside container"))
    }
}
#[derive(Debug)]
struct MappingData {
    entries: Vec<(Value, Rc<Cell>)>,
    index: BTreeMap<Key, usize>,
}
impl Drop for MappingData {
    fn drop(&mut self) {
        // Native vm_freeNode clears values even when external member references
        // retain their cells. No Cell guards can be held across header drops.
        for (_, cell) in &self.entries {
            cell.clear_on_owner_drop();
        }
    }
}
#[derive(Debug, Clone)]
pub struct MappingValue(Rc<RefCell<MappingData>>);
impl PartialEq for MappingValue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for MappingValue {}
impl MappingValue {
    pub fn entries(&self) -> Result<Vec<(Value, Value)>, VmError> {
        let data = self.0.try_borrow().map_err(|_| borrow_error())?;
        data.entries
            .iter()
            .map(|(k, c)| Ok((k.clone(), c.read()?)))
            .collect()
    }
    pub fn get(&self, key: &Value) -> Result<Option<Value>, VmError> {
        self.lookup(key)?.map(|c| c.read()).transpose()
    }
    pub fn depth(&self) -> Result<usize, VmError> {
        depth(&Value::Mapping(self.clone()), &mut BTreeMap::new())
    }
    pub(crate) fn len(&self) -> Result<usize, VmError> {
        Ok(self
            .0
            .try_borrow()
            .map_err(|_| borrow_error())?
            .entries
            .len())
    }
    fn lookup(&self, key: &Value) -> Result<Option<Rc<Cell>>, VmError> {
        let key = Key::new(key)?;
        let data = self.0.try_borrow().map_err(|_| borrow_error())?;
        Ok(data.index.get(&key).map(|&i| data.entries[i].1.clone()))
    }
    pub(crate) fn insertion_needed(&self, key: &Value) -> Result<bool, VmError> {
        let missing = self.lookup(key)?.is_none();
        if missing && self.len()? >= MAX_ITEMS {
            return Err(VmError::new(
                VmErrorKind::Limit,
                "mapping item limit exceeded",
            ));
        }
        Ok(missing)
    }
    pub(crate) fn member(&self, key: &Value) -> Result<Rc<Cell>, VmError> {
        if let Some(c) = self.lookup(key)? {
            return Ok(c);
        }
        self.insertion_needed(key)?;
        let k = Key::new(key)?;
        let cell = Cell::new(Value::Undefined)?;
        let mut data = self.0.try_borrow_mut().map_err(|_| borrow_error())?;
        let i = data.entries.len();
        data.entries.push((key.clone(), cell.clone()));
        data.index.insert(k, i);
        Ok(cell)
    }
}
fn borrow_error() -> VmError {
    VmError::new(VmErrorKind::InvalidState, "mapping is borrowed")
}
fn depth(value: &Value, memo: &mut BTreeMap<(u8, usize), usize>) -> Result<usize, VmError> {
    let (key, values) = match value {
        Value::Array(a) => {
            let key = (6, Rc::as_ptr(&a.0) as usize);
            if let Some(&d) = memo.get(&key) {
                return Ok(d);
            }
            (key, a.elements()?)
        }
        Value::Mapping(m) => {
            let key = (7, Rc::as_ptr(&m.0) as usize);
            if let Some(&d) = memo.get(&key) {
                return Ok(d);
            }
            (key, m.entries()?.into_iter().map(|(_, v)| v).collect())
        }
        _ => return Ok(0),
    };
    let mut d = 1;
    for v in values {
        d = d.max(depth(&v, memo)? + 1);
    }
    validate(0, d)?;
    memo.insert(key, d);
    Ok(d)
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
fn validate_values<'a>(
    count: usize,
    values: impl Iterator<Item = &'a Value>,
) -> Result<(), VmError> {
    validate(count, 1)?;
    let mut memo = BTreeMap::new();
    for v in values {
        validate(count, depth(v, &mut memo)? + 1)?;
    }
    Ok(())
}
pub(crate) fn array(elements: Vec<Value>) -> Result<Value, VmError> {
    validate_values(elements.len(), elements.iter())?;
    Ok(Value::Array(ArrayValue(Rc::new(ArrayData {
        elements: elements
            .into_iter()
            .map(Cell::new)
            .collect::<Result<_, _>>()?,
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
    validate_values(unique.len(), unique.iter().map(|(_, v)| v))?;
    let entries = unique
        .into_iter()
        .map(|(k, v)| Ok((k, Cell::new(v)?)))
        .collect::<Result<_, VmError>>()?;
    Ok(Value::Mapping(MappingValue(Rc::new(RefCell::new(
        MappingData { entries, index },
    )))))
}
