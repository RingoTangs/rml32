//! Shared acyclic containers with private stable slots and owned read snapshots.
use crate::{Value, VmError, VmErrorKind, cell::Cell};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
const MAX_ITEMS: usize = 65536;
const MAX_DEPTH: usize = 64;
#[derive(Debug)]
struct ArrayData {
    counter: Rc<crate::ownership::Counter>,
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
    pub(crate) fn counter(&self) -> Rc<crate::ownership::Counter> {
        self.0.counter.clone()
    }
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
    // Stable entry indices; public entries() order is independent of these chains.
    heads: Vec<Option<usize>>,
    next: Vec<Option<usize>>,
    unfilled: usize,
    // Container member writes remain scalar-only. This conservative bit can stay
    // true after an overwrite, but never misses a newly introduced container.
    may_have_containers: bool,
}
fn allocation_error() -> VmError {
    VmError::new(VmErrorKind::Limit, "mapping allocation failed")
}
impl MappingData {
    fn new(hint: usize) -> Result<Self, VmError> {
        let capacity = if hint <= 8 {
            8
        } else {
            (hint + 1).next_power_of_two()
        };
        let mut heads = Vec::new();
        heads
            .try_reserve_exact(capacity)
            .map_err(|_| allocation_error())?;
        heads.resize(capacity, None);
        Ok(Self {
            entries: Vec::new(),
            index: BTreeMap::new(),
            heads,
            next: Vec::new(),
            unfilled: capacity * 80 / 100,
            may_have_containers: false,
        })
    }
    fn reserve_insert(&mut self, hash: u32) -> Result<(), VmError> {
        self.entries
            .try_reserve(1)
            .map_err(|_| allocation_error())?;
        self.next.try_reserve(1).map_err(|_| allocation_error())?;
        if self.heads[hash as usize & (self.heads.len() - 1)].is_none() && self.unfilled == 1 {
            self.heads
                .try_reserve_exact(self.heads.len())
                .map_err(|_| allocation_error())?;
        }
        Ok(())
    }
    // Infallible after reserve_insert. Follows vm_growMapping, including reversal
    // of the moved high subchain; unfilled counts an empty-bucket budget, not items.
    fn insert(&mut self, key: Value, k: Key, cell: Rc<Cell>, hash: u32) {
        let mut bucket = hash as usize & (self.heads.len() - 1);
        if self.heads[bucket].is_none() {
            self.unfilled -= 1;
            if self.unfilled == 0 {
                let old = self.heads.len();
                self.heads.resize(old * 2, None);
                self.unfilled = old * 80 / 100;
                for i in (0..old).rev() {
                    let mut node = self.heads[i];
                    let mut previous = None;
                    while let Some(n) = node {
                        let next = self.next[n];
                        if native_hash(&self.entries[n].0) as usize & old != 0 {
                            if let Some(p) = previous {
                                self.next[p] = next;
                            } else {
                                self.heads[i] = next;
                            }
                            if self.heads[i + old].is_none() {
                                self.unfilled -= 1;
                            }
                            self.next[n] = self.heads[i + old];
                            self.heads[i + old] = Some(n);
                        } else {
                            previous = Some(n);
                        }
                        node = next;
                    }
                    if self.heads[i].is_none()
                        && previous.is_none()
                        && self.heads[i + old].is_some()
                    {
                        self.unfilled += 1;
                    }
                }
                bucket = hash as usize & (self.heads.len() - 1);
            }
        }
        let node = self.entries.len();
        self.entries.push((key, cell));
        self.next.push(self.heads[bucket]);
        self.heads[bucket] = Some(node);
        self.index.insert(k, node);
    }
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
pub struct MappingValue(Rc<RefCell<MappingData>>, Rc<crate::ownership::Counter>);
impl PartialEq for MappingValue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for MappingValue {}
impl MappingValue {
    pub(crate) fn counter(&self) -> Rc<crate::ownership::Counter> {
        self.1.clone()
    }
    #[cfg(test)]
    pub(crate) fn header_refs(&self) -> usize {
        Rc::strong_count(&self.0)
    }
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
        let hash = native_hash(key);
        data.reserve_insert(hash)?;
        data.insert(key.clone(), k, cell.clone(), hash);
        Ok(cell)
    }
    pub(crate) fn scan(
        &self,
        bucket: &mut u32,
        subindex: &mut u32,
    ) -> Result<Option<(Value, Rc<Cell>)>, VmError> {
        let data = self.0.try_borrow().map_err(|_| borrow_error())?;
        let mut skip = (*subindex as i32).max(0) as usize;
        *subindex = subindex.wrapping_add(1);
        while (*bucket as usize) < data.heads.len() {
            let mut node = data.heads[*bucket as usize];
            while let Some(n) = node {
                if skip == 0 {
                    return Ok(Some(data.entries[n].clone()));
                }
                skip -= 1;
                node = data.next[n];
            }
            *bucket += 1;
            skip = 0;
            *subindex = 1;
        }
        Ok(None)
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
fn identity(value: &Value) -> Option<(u8, usize)> {
    match value {
        Value::Array(a) => Some((6, Rc::as_ptr(&a.0) as usize)),
        Value::Mapping(m) => Some((7, Rc::as_ptr(&m.0) as usize)),
        _ => None,
    }
}
fn reaches(
    value: &Value,
    target: (u8, usize),
    seen: &mut std::collections::BTreeSet<(u8, usize)>,
) -> Result<bool, VmError> {
    let Some(id) = identity(value) else {
        return Ok(false);
    };
    if id == target {
        return Ok(true);
    }
    if !seen.insert(id) {
        return Ok(false);
    }
    let values = match value {
        Value::Array(a) => a.elements()?,
        Value::Mapping(m) => m.entries()?.into_iter().map(|(_, v)| v).collect(),
        _ => unreachable!(),
    };
    for value in values {
        if reaches(&value, target, seen)? {
            return Ok(true);
        }
    }
    Ok(false)
}
pub(crate) fn mapping_output(
    owner: &MappingValue,
    binding: Value,
    values: &[Value; 3],
) -> Result<ArrayValue, VmError> {
    validate_values(3, values.iter())?;
    if let Value::Array(array) = binding
        && array.len() == 3
    {
        let source = Value::Mapping(owner.clone());
        let output = Value::Array(array.clone());
        let nested_source = owner
            .0
            .try_borrow()
            .map_err(|_| borrow_error())?
            .may_have_containers;
        if (nested_source && reaches(&source, identity(&output).unwrap(), &mut Default::default())?)
            || reaches(&output, identity(&source).unwrap(), &mut Default::default())?
        {
            return Err(VmError::new(
                VmErrorKind::UnsupportedFeature,
                "mapping output crosses source container graph",
            ));
        }
        return Ok(array);
    }
    let Value::Array(array) = array(values.to_vec())? else {
        unreachable!()
    };
    Ok(array)
}
pub(crate) fn array(elements: Vec<Value>) -> Result<Value, VmError> {
    validate_values(elements.len(), elements.iter())?;
    Ok(Value::Array(ArrayValue(Rc::new(ArrayData {
        counter: crate::ownership::Counter::new()?,
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
    let hint = entries.len();
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
    let mut data = MappingData::new(hint)?;
    for (key, value) in unique {
        let k = Key::new(&key)?;
        let hash = native_hash(&key);
        data.may_have_containers |= matches!(value, Value::Array(_) | Value::Mapping(_));
        let cell = Cell::new(value)?;
        data.reserve_insert(hash)?;
        data.insert(key, k, cell, hash);
    }
    Ok(Value::Mapping(MappingValue(
        Rc::new(RefCell::new(data)),
        crate::ownership::Counter::new()?,
    )))
}

// vm_whashstr table at ELF VA 0x083cc3e0; 2AL F02 and retained machine bytes.
const HASH_TABLE: [u8; 256] = [
    0x01, 0x57, 0x31, 0x0c, 0xb0, 0xb2, 0x66, 0xa6, 0x79, 0xc1, 0x06, 0x54, 0xf9, 0xe6, 0x2c, 0xa3,
    0x0e, 0xc5, 0xd5, 0xb5, 0xa1, 0x55, 0xda, 0x50, 0x40, 0xef, 0x18, 0xe2, 0xec, 0x8e, 0x26, 0xc8,
    0x6e, 0xb1, 0x68, 0x67, 0x8d, 0xfd, 0xff, 0x32, 0x4d, 0x65, 0x51, 0x12, 0x2d, 0x60, 0x1f, 0xde,
    0x19, 0x6b, 0xbe, 0x46, 0x56, 0xed, 0xf0, 0x22, 0x48, 0xf2, 0x14, 0xd6, 0xf4, 0xe3, 0x95, 0xeb,
    0x61, 0xea, 0x39, 0x16, 0x3c, 0xfa, 0x52, 0xaf, 0xd0, 0x05, 0x7f, 0xc7, 0x6f, 0x3e, 0x87, 0xf8,
    0xae, 0xa9, 0xd3, 0x3a, 0x42, 0x9a, 0x6a, 0xc3, 0xf5, 0xab, 0x11, 0xbb, 0xb6, 0xb3, 0x00, 0xf3,
    0x84, 0x38, 0x94, 0x4b, 0x80, 0x85, 0x9e, 0x64, 0x82, 0x7e, 0x5b, 0x0d, 0x99, 0xf6, 0xd8, 0xdb,
    0x77, 0x44, 0xdf, 0x4e, 0x53, 0x58, 0xc9, 0x63, 0x7a, 0x0b, 0x5c, 0x20, 0x88, 0x72, 0x34, 0x0a,
    0x8a, 0x1e, 0x30, 0xb7, 0x9c, 0x23, 0x3d, 0x1a, 0x8f, 0x4a, 0xfb, 0x5e, 0x81, 0xa2, 0x3f, 0x98,
    0xaa, 0x07, 0x73, 0xa7, 0xf1, 0xce, 0x03, 0x96, 0x37, 0x3b, 0x97, 0xdc, 0x5a, 0x35, 0x17, 0x83,
    0x7d, 0xad, 0x0f, 0xee, 0x4f, 0x5f, 0x59, 0x10, 0x69, 0x89, 0xe1, 0xe0, 0xd9, 0xa0, 0x25, 0x7b,
    0x76, 0x49, 0x02, 0x9d, 0x2e, 0x74, 0x09, 0x91, 0x86, 0xe4, 0xcf, 0xd4, 0xca, 0xd7, 0x45, 0xe5,
    0x1b, 0xbc, 0x43, 0x7c, 0xa8, 0xfc, 0x2a, 0x04, 0x1d, 0x6c, 0x15, 0xf7, 0x13, 0xcd, 0x27, 0xcb,
    0xe9, 0x28, 0xba, 0x93, 0xc6, 0xc0, 0x9b, 0x21, 0xa4, 0xbf, 0x62, 0xcc, 0xa5, 0xb4, 0x75, 0x4c,
    0x8c, 0x24, 0xd2, 0xac, 0x29, 0x36, 0x9f, 0x08, 0xb9, 0xe8, 0x71, 0xc4, 0xe7, 0x2f, 0x92, 0x78,
    0x33, 0x41, 0x1c, 0x90, 0xfe, 0xdd, 0x5d, 0xbd, 0xc2, 0x8b, 0x70, 0x2b, 0x47, 0x6d, 0xb8, 0xd1,
];
fn native_hash(value: &Value) -> u32 {
    match value {
        Value::Int(n) => (n >> 4) as u32,
        Value::Bytes(bytes) => {
            let mut bytes = bytes.iter().take(32).copied().take_while(|b| *b != 0);
            let Some(first) = bytes.next() else {
                return 0;
            };
            let (mut hi, mut lo) = (HASH_TABLE[first as usize], 0u8);
            for b in bytes {
                hi = HASH_TABLE[(hi ^ b) as usize];
                lo = HASH_TABLE[(lo ^ b) as usize];
            }
            (u32::from(hi) << 8) | u32::from(lo)
        }
        _ => unreachable!("validated Mapping key"),
    }
}

#[cfg(test)]
mod mapping_tests {
    use super::*;
    #[test]
    fn repeated_growth_matches_frozen_independent_2al_static_model() {
        fn decode(key: &serde_json::Value) -> Value {
            if let Some(n) = key.get("int") {
                return Value::Int(n.as_i64().unwrap() as i32);
            }
            let text = key["hex"].as_str().unwrap();
            Value::bytes(
                (0..text.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
                    .collect::<Vec<_>>(),
            )
        }
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/mapping-order-2al.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let m = MappingValue(
                Rc::new(RefCell::new(
                    MappingData::new(case["hint"].as_u64().unwrap() as usize).unwrap(),
                )),
                crate::ownership::Counter::new().unwrap(),
            );
            for key in case["keys"].as_array().unwrap() {
                m.member(&decode(key)).unwrap();
            }
            assert_eq!(
                order(&m),
                case["order"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(decode)
                    .collect::<Vec<_>>()
            );
            let data = m.0.borrow();
            assert_eq!(
                data.heads.len(),
                case["capacity"].as_u64().unwrap() as usize
            );
            assert_eq!(data.unfilled, case["unfilled"].as_u64().unwrap() as usize);
        }
    }
    fn map(keys: &[i32], hint: usize) -> MappingValue {
        let mut data = MappingData::new(hint).unwrap();
        for &key in keys {
            let value = Value::Int(key);
            let hash = native_hash(&value);
            data.reserve_insert(hash).unwrap();
            data.insert(
                value.clone(),
                Key::new(&value).unwrap(),
                Cell::new(value).unwrap(),
                hash,
            );
        }
        MappingValue(
            Rc::new(RefCell::new(data)),
            crate::ownership::Counter::new().unwrap(),
        )
    }
    fn order(m: &MappingValue) -> Vec<Value> {
        let (mut bucket, mut position) = (0, 0);
        let mut keys = Vec::new();
        while let Some((key, _)) = m.scan(&mut bucket, &mut position).unwrap() {
            keys.push(key);
        }
        keys
    }
    fn ints(keys: &[i32]) -> Vec<Value> {
        keys.iter().copied().map(Value::Int).collect()
    }
    #[test]
    fn hash_vectors_cover_signed_ints_nul_prefix_high_bytes_and_long_suffix() {
        for (n, h) in [
            (0, 0),
            (15, 0),
            (16, 1),
            (-1, u32::MAX),
            (i32::MIN, 0xf8000000),
            (i32::MAX, 0x07ffffff),
        ] {
            assert_eq!(native_hash(&Value::Int(n)), h);
        }
        for (bytes, hash) in [
            (b"".as_slice(), 0),
            (b"a", 14336),
            (b"ab", 4500),
            (b"abc", 57277),
            (b"a\0x", 14336),
            (b"a\0y", 14336),
            (b"\xff", 53504),
            (b"\x80", 35328),
            (b"A", 59904),
        ] {
            assert_eq!(native_hash(&Value::bytes(bytes)), hash);
        }
        assert_eq!(native_hash(&Value::bytes([b'a'; 32])), 723);
        assert_eq!(native_hash(&Value::bytes([b'a'; 33])), 723);
    }
    #[test]
    fn initial_capacity_is_strict_above_hint_except_eight_and_uses_raw_pairs() {
        for (hint, expected) in [(0, 8), (8, 8), (9, 16), (16, 32), (65536, 131072)] {
            assert_eq!(MappingData::new(hint).unwrap().heads.len(), expected);
        }
        let Value::Mapping(m) = mapping(vec![(Value::Int(0), Value::Int(1)); 16]).unwrap() else {
            panic!()
        };
        assert_eq!(m.0.borrow().heads.len(), 32);
        assert_eq!(m.len().unwrap(), 1);
    }
    #[test]
    fn native_order_head_inserts_while_public_order_and_overwrites_stay_stable() {
        let Value::Mapping(m) = mapping(vec![
            (Value::Int(0), Value::Int(10)),
            (Value::Int(1), Value::Int(11)),
            (Value::Int(0), Value::Int(12)),
        ])
        .unwrap() else {
            panic!()
        };
        assert_eq!(order(&m), ints(&[1, 0]));
        assert_eq!(
            m.entries().unwrap(),
            vec![
                (Value::Int(0), Value::Int(12)),
                (Value::Int(1), Value::Int(11))
            ]
        );
        let old = m.member(&Value::Int(0)).unwrap();
        m.member(&Value::Int(2)).unwrap();
        assert_eq!(old.id(), m.member(&Value::Int(0)).unwrap().id());
    }
    #[test]
    fn nul_and_long_bytes_collide_but_remain_distinct_keys() {
        let keys = [b"a".as_slice(), b"a\0", b"a\0x", b"a\0y"];
        let Value::Mapping(m) = mapping(
            keys.iter()
                .enumerate()
                .map(|(i, k)| (Value::bytes(k), Value::Int(i as i32)))
                .collect(),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(
            order(&m),
            keys.iter().rev().map(Value::bytes).collect::<Vec<_>>()
        );
        for (i, k) in keys.iter().enumerate() {
            assert_eq!(m.get(&Value::bytes(k)).unwrap(), Some(Value::Int(i as i32)));
        }
    }
    #[test]
    fn growth_reverses_high_subchain_not_low_and_preserves_cells() {
        let m = map(&[0, 128, 256, 384, 640, 16, 32, 48, 64], 0);
        let cell = m.member(&Value::Int(128)).unwrap();
        m.member(&Value::Int(80)).unwrap();
        assert_eq!(
            order(&m),
            ints(&[256, 0, 16, 32, 48, 64, 80, 128, 384, 640])
        );
        assert_eq!((m.0.borrow().heads.len(), m.0.borrow().unfilled), (16, 5));
        cell.replace(Value::Int(99)).unwrap();
        assert_eq!(m.get(&Value::Int(128)).unwrap(), Some(Value::Int(99)));
        assert_eq!(m.member(&Value::Int(128)).unwrap().id(), cell.id());
    }
    #[test]
    fn collisions_do_not_grow_on_count_and_failed_member_allocation_does_not_rehash() {
        let m = map(&(0..16).collect::<Vec<_>>(), 0);
        assert_eq!(
            (
                m.len().unwrap(),
                m.0.borrow().heads.len(),
                m.0.borrow().unfilled
            ),
            (16, 8, 5)
        );
        let m = map(&[0, 16, 32, 48, 64], 0);
        let before = order(&m);
        crate::cell::with_allocation_failure(0, || {
            assert_eq!(
                m.member(&Value::Int(80)).unwrap_err().kind,
                VmErrorKind::Limit
            )
        });
        assert_eq!(order(&m), before);
        assert_eq!((m.0.borrow().heads.len(), m.0.borrow().unfilled), (8, 1));
        let _guard = m.0.borrow();
        assert_eq!(
            m.member(&Value::Int(80)).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
    }
    #[test]
    fn exhausted_scan_retains_position_and_can_resume_after_growth() {
        let m = map(&[0], 0);
        let (mut b, mut s) = (0, 0);
        assert_eq!(m.scan(&mut b, &mut s).unwrap().unwrap().0, Value::Int(0));
        assert!(m.scan(&mut b, &mut s).unwrap().is_none());
        assert_eq!((b, s), (8, 1));
        for key in [128, 384, 16, 32, 48, 64, 80] {
            m.member(&Value::Int(key)).unwrap();
        }
        assert_eq!(m.scan(&mut b, &mut s).unwrap().unwrap().0, Value::Int(384));
        b = 0;
        s = 0x80000000;
        assert_eq!(m.scan(&mut b, &mut s).unwrap().unwrap().0, Value::Int(0));
    }
}
