//! Structural validation of a final projected graph. No VM execution or retain.
use crate::{
    Value, ValueLimits, VmError, VmErrorKind,
    cell::{Cell, SlotPayload},
    ownership::Counter,
};
use std::{
    cell::{Cell as Flag, RefCell},
    collections::{BTreeMap, BTreeSet},
    rc::{Rc, Weak},
};
fn error(kind: VmErrorKind, message: &str) -> VmError {
    VmError::new(kind, message)
}
fn limit() -> VmError {
    error(VmErrorKind::Limit, "projected graph exceeds value limits")
}
thread_local! {
    static POLICY:Flag<ValueLimits>=Flag::new(ValueLimits::default());
    static INDEX:RefCell<Index>=RefCell::new(Index::default());
}
pub(crate) struct Scope(ValueLimits);
impl Scope {
    pub fn new(limits: ValueLimits) -> Self {
        Self(POLICY.replace(limits))
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        POLICY.set(self.0);
    }
}
#[derive(Debug, Clone)]
pub(crate) enum WeakHeader {
    Array(Weak<crate::container::ArrayData>),
    Mapping(Weak<RefCell<crate::container::MappingData>>, Weak<Counter>),
    Function(Weak<crate::value::FunctionData>),
}
impl WeakHeader {
    pub(crate) fn upgrade(&self) -> Option<Value> {
        match self {
            Self::Array(a) => a
                .upgrade()
                .map(|v| Value::Array(crate::container::ArrayValue(v))),
            Self::Mapping(m, c) => Some(Value::Mapping(crate::container::MappingValue(
                m.upgrade()?,
                c.upgrade()?,
            ))),
            Self::Function(f) => f
                .upgrade()
                .map(|v| Value::Function(crate::value::FunctionValue(v))),
        }
    }
}
#[derive(Default)]
struct Index {
    headers: BTreeMap<u64, WeakHeader>,
    cells: BTreeMap<u64, Weak<Cell>>,
    parents: BTreeMap<u64, BTreeSet<u64>>,
    owners: BTreeMap<u64, BTreeMap<u64, Weak<Cell>>>,
    ticks: usize,
}
impl Index {
    fn tick(&mut self) {
        self.ticks = self.ticks.wrapping_add(1);
        if self.ticks.is_multiple_of(4096) {
            self.cells.retain(|_, c| c.upgrade().is_some());
            self.headers.retain(|_, h| h.upgrade().is_some());
            self.parents.retain(|id, parents| {
                parents.retain(|p| self.headers.contains_key(p));
                self.cells.contains_key(id) && !parents.is_empty()
            });
            self.owners.retain(|id, cells| {
                cells.retain(|_, c| c.upgrade().is_some());
                self.headers.contains_key(id) && !cells.is_empty()
            });
        }
    }
}
pub(crate) fn register_cell(cell: &Rc<Cell>, value: &Value) -> Result<(), VmError> {
    register(value)?;
    INDEX.with_borrow_mut(|i| {
        i.tick();
        i.cells.insert(cell.id(), Rc::downgrade(cell));
    });
    bind(cell.id(), value);
    Ok(())
}
pub(crate) fn prune_cell(id: u64) {
    INDEX.with_borrow_mut(|i| {
        i.cells.remove(&id);
        i.parents.remove(&id);
    });
}
pub(crate) fn prune_header(id: u64) {
    INDEX.with_borrow_mut(|i| {
        i.headers.remove(&id);
        i.owners.remove(&id);
    });
}
// Weak cache entries are reserved before mutation. A failed transaction leaves
// only stale weak metadata, filtered against the actual/projected payload.
pub(crate) fn bind(id: u64, value: &Value) {
    if let Some(header) = value.counter() {
        INDEX.with_borrow_mut(|i| {
            if let Some(cell) = i.cells.get(&id).cloned() {
                i.owners.entry(header.id).or_default().insert(id, cell);
            }
        });
    }
}
pub(crate) fn register(value: &Value) -> Result<(), VmError> {
    let Some(counter) = value.counter() else {
        return Ok(());
    };
    let weak = match value {
        Value::Array(a) => WeakHeader::Array(Rc::downgrade(&a.0)),
        Value::Mapping(m) => WeakHeader::Mapping(Rc::downgrade(&m.0), Rc::downgrade(&m.1)),
        Value::Function(f) => WeakHeader::Function(Rc::downgrade(&f.0)),
        _ => unreachable!(),
    };
    // Metadata registration is not publication; a borrowed header may be
    // registered again at its next checked boundary.
    let cells = slots(value).map(|(cells, _)| cells).unwrap_or_default();
    INDEX.with_borrow_mut(|i| {
        i.tick();
        i.headers.insert(counter.id, weak);
        for cell in cells {
            i.parents.entry(cell.id()).or_default().insert(counter.id);
        }
    });
    Ok(())
}
pub(crate) fn link(header: &Value, cell: &Rc<Cell>) -> Result<(), VmError> {
    register(header)?;
    INDEX.with_borrow_mut(|i| {
        i.parents
            .entry(cell.id())
            .or_default()
            .insert(header.counter().unwrap().id);
    });
    Ok(())
}
fn slots(value: &Value) -> Result<(Vec<Rc<Cell>>, usize), VmError> {
    match value {
        Value::Array(a) => Ok((a.0.elements.clone(), 0)),
        Value::Mapping(m) => {
            let entries = m.graph_entries()?;
            Ok((
                entries.iter().map(|(_, c)| c.clone()).collect(),
                entries.len(),
            ))
        }
        Value::Function(f) => {
            if f.0.bound.len() != f.bound_count() {
                return Err(error(
                    VmErrorKind::InvalidState,
                    "invalid stable capture layout",
                ));
            }
            Ok((f.0.bound.clone(), 0))
        }
        _ => Ok((vec![], 0)),
    }
}
#[derive(Default)]
pub(crate) struct Projection {
    values: BTreeMap<u64, Value>,
    cells: BTreeMap<u64, Rc<Cell>>,
    roots: Vec<Value>,
    extras: BTreeMap<u64, Vec<Rc<Cell>>>,
    retiring: BTreeSet<u64>,
    hidden: BTreeSet<u64>,
    owner_deltas: BTreeMap<u64, i128>,
}
impl Projection {
    pub fn root_cell(&mut self, cell: Rc<Cell>) {
        self.cells.entry(cell.id()).or_insert(cell);
    }
    pub fn write(&mut self, cell: Rc<Cell>, value: Value) -> Result<(), VmError> {
        if let Some(old) = self.values.get(&cell.id())
            && *old != value
        {
            return Err(error(
                VmErrorKind::InvalidState,
                "conflicting graph projection",
            ));
        }
        self.values.insert(cell.id(), value);
        self.cells.insert(cell.id(), cell);
        Ok(())
    }
    pub fn payload(&mut self, cell: Rc<Cell>, payload: &SlotPayload) -> Result<(), VmError> {
        let id = cell.id();
        let value = match payload {
            SlotPayload::Value(v) => v.clone(),
            SlotPayload::Loop(c) => {
                if let Some(bound) = c.graph_bound() {
                    self.root_cell(bound);
                }
                self.hidden.insert(id);
                c.graph_owner().unwrap_or(Value::Undefined)
            }
        };
        self.write(cell, value)
    }
    pub fn transfer(&mut self, src: Rc<Cell>, dst: Rc<Cell>) -> Result<(), VmError> {
        let value = src.read()?;
        self.write(src, Value::Undefined)?;
        self.write(dst, value)
    }
    pub fn retire(&mut self, cell: &Rc<Cell>) {
        self.retiring.insert(cell.id());
    }
    pub fn extra(&mut self, header: &Value, cell: Rc<Cell>) {
        self.roots.push(header.clone());
        let id = header.counter().unwrap().id;
        let owners = INDEX.with_borrow(|i| i.owners.get(&id).cloned().unwrap_or_default());
        for owner in owners.into_values().filter_map(|w| w.upgrade()) {
            if owner.graph_header() == Some(id) {
                self.cells.insert(owner.id(), owner);
            }
        }
        self.extras
            .entry(header.counter().unwrap().id)
            .or_default()
            .push(cell);
    }
    fn value(&self, cell: &Cell) -> Result<Value, VmError> {
        if self.retiring.contains(&cell.id()) {
            return Ok(Value::Undefined);
        }
        self.values
            .get(&cell.id())
            .cloned()
            .map(Ok)
            .unwrap_or_else(|| cell.graph_value())
    }
    fn header(&self, cell: &Cell) -> Option<u64> {
        if self.retiring.contains(&cell.id()) {
            None
        } else if let Some(value) = self.values.get(&cell.id()) {
            value.counter().map(|c| c.id)
        } else {
            cell.graph_header()
        }
    }
    fn alive(&self, value: &Value) -> bool {
        let counter = value.counter().unwrap();
        counter.owners() as i128 + self.owner_deltas.get(&counter.id).copied().unwrap_or(0) > 0
    }
    pub fn check(mut self) -> Result<(), VmError> {
        for cell in self.cells.values() {
            if let Some(id) = cell.graph_header() {
                *self.owner_deltas.entry(id).or_default() -= 1;
            }
            if let Some(id) = self.header(cell) {
                *self.owner_deltas.entry(id).or_default() += 1;
            }
        }
        let limits = POLICY.get();
        let mut visited = BTreeSet::new();
        let mut queue: Vec<_> = self.cells.values().cloned().collect();
        let mut seen_cells = BTreeSet::new();
        while let Some(cell) = queue.pop() {
            if !seen_cells.insert(cell.id()) {
                continue;
            }
            if seen_cells.len() + visited.len() > limits.max_graph_nodes {
                return Err(limit());
            }
            let parents =
                INDEX.with_borrow(|i| i.parents.get(&cell.id()).cloned().unwrap_or_default());
            for parent in parents {
                if visited.contains(&parent) {
                    continue;
                }
                let candidate =
                    INDEX.with_borrow(|i| i.headers.get(&parent).and_then(WeakHeader::upgrade));
                let Some(header) = candidate else { continue };
                if !self.alive(&header) {
                    continue;
                }
                let members = slots(&header)?.0;
                if !members.iter().any(|c| c.id() == cell.id())
                    && !self
                        .extras
                        .get(&parent)
                        .is_some_and(|xs| xs.iter().any(|c| c.id() == cell.id()))
                {
                    continue;
                }
                visited.insert(parent);
                self.roots.push(header);
                let owners =
                    INDEX.with_borrow(|i| i.owners.get(&parent).cloned().unwrap_or_default());
                for owner in owners.into_values().filter_map(|w| w.upgrade()) {
                    if self.header(&owner) == Some(parent) {
                        self.cells
                            .entry(owner.id())
                            .or_insert_with(|| owner.clone());
                        queue.push(owner);
                    }
                }
            }
        }
        let mut walk = Walk::new(&self, limits);
        for value in &self.roots {
            walk.run(Node::Value(value.clone()))?;
        }
        for cell in self.cells.values() {
            walk.run(Node::Cell(cell.clone()))?;
        }
        Ok(())
    }
}
#[derive(Clone)]
enum Node {
    Value(Value),
    Cell(Rc<Cell>),
    Key(u64, usize),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Id {
    Header(u64),
    Cell(u64),
    Key(u64, usize),
}
struct Walk<'a> {
    projection: &'a Projection,
    limits: ValueLimits,
    active: BTreeSet<Id>,
    memo: BTreeMap<Id, usize>,
}
impl<'a> Walk<'a> {
    fn new(projection: &'a Projection, limits: ValueLimits) -> Self {
        Self {
            projection,
            limits,
            active: BTreeSet::new(),
            memo: BTreeMap::new(),
        }
    }
    fn run(&mut self, root: Node) -> Result<(), VmError> {
        // Explicit postorder stack: even malformed very deep input cannot overflow Rust's call stack.
        let mut stack = vec![(root, false)];
        let mut results = Vec::new();
        let mut frames: Vec<(Id, usize, usize)> = Vec::new();
        while let Some((node, exit)) = stack.pop() {
            if exit {
                let (id, weight, count) = frames.pop().unwrap();
                let mut height: usize = 0;
                for _ in 0..count {
                    height = height.max(results.pop().unwrap());
                }
                let height = height.checked_add(weight).ok_or_else(limit)?;
                if height > self.limits.max_depth {
                    return Err(limit());
                }
                self.active.remove(&id);
                self.memo.insert(id, height);
                results.push(height);
                continue;
            }
            let identity = match &node {
                Node::Key(h, k) => Some(Id::Key(*h, *k)),
                Node::Cell(c) => Some(Id::Cell(c.id())),
                Node::Value(v) => v.counter().map(|c| Id::Header(c.id)),
            };
            if let Some(id) = identity {
                if self.active.contains(&id) {
                    return Err(error(
                        VmErrorKind::UnsupportedType,
                        "cycle in projected value graph",
                    ));
                }
                if let Some(&height) = self.memo.get(&id) {
                    results.push(height);
                    continue;
                }
                if self.active.len() + self.memo.len() >= self.limits.max_graph_nodes {
                    return Err(limit());
                }
            }
            let (id, children, weight) = match node {
                Node::Key(h, k) => (Id::Key(h, k), vec![], 0),
                Node::Cell(cell) => (
                    Id::Cell(cell.id()),
                    vec![Node::Value(self.projection.value(&cell)?)],
                    0,
                ),
                Node::Value(value) => {
                    let Some(counter) = value.counter() else {
                        results.push(0);
                        continue;
                    };
                    let (mut cells, keys) = slots(&value)?;
                    if let Some(extra) = self.projection.extras.get(&counter.id) {
                        cells.extend(extra.iter().cloned());
                    }
                    if cells.len() > self.limits.max_items {
                        return Err(limit());
                    }
                    for cell in &cells {
                        if self.projection.hidden.contains(&cell.id())
                            || (!self.projection.values.contains_key(&cell.id())
                                && !self.projection.retiring.contains(&cell.id())
                                && cell.is_loop()?)
                        {
                            return Err(error(
                                VmErrorKind::UnsupportedType,
                                "hidden loop is not a header member or capture",
                            ));
                        }
                    }
                    let weight = if matches!(value, Value::Function(_)) {
                        usize::from(cells.iter().any(|c| self.projection.header(c).is_some()))
                    } else {
                        1
                    };
                    let mut children: Vec<_> = cells.into_iter().map(Node::Cell).collect();
                    children.extend(
                        (0..keys + self.projection.extras.get(&counter.id).map_or(0, Vec::len))
                            .map(|k| Node::Key(counter.id, k)),
                    );
                    (Id::Header(counter.id), children, weight)
                }
            };
            if self.active.contains(&id) {
                return Err(error(
                    VmErrorKind::UnsupportedType,
                    "cycle in projected value graph",
                ));
            }
            if let Some(&height) = self.memo.get(&id) {
                results.push(height);
                continue;
            }
            if self.active.len() + self.memo.len() >= self.limits.max_graph_nodes {
                return Err(limit());
            }
            self.active.insert(id);
            crate::copy_plan::reserve(&mut stack, children.len() + 1)?;
            crate::copy_plan::reserve(&mut results, children.len() + 1)?;
            crate::copy_plan::reserve(&mut frames, 1)?;
            frames.push((id, weight, children.len()));
            stack.push((Node::Key(0, 0), true));
            stack.extend(children.into_iter().rev().map(|c| (c, false)));
        }
        Ok(())
    }
}
pub(crate) fn values(values: &[Value]) -> Result<(), VmError> {
    let p = Projection {
        roots: values.to_vec(),
        ..Default::default()
    };
    p.check()
}
pub(crate) fn replace(cell: Rc<Cell>, value: Value) -> Result<(), VmError> {
    let mut p = Projection::default();
    p.write(cell, value)?;
    p.check()
}
pub(crate) fn cell(id: u64) -> Option<Rc<Cell>> {
    INDEX.with_borrow(|i| i.cells.get(&id).and_then(Weak::upgrade))
}

pub(crate) fn cells(cells: &[Rc<Cell>]) -> Result<(), VmError> {
    let mut p = Projection::default();
    for cell in cells {
        p.cells.insert(cell.id(), cell.clone());
    }
    p.check()
}

pub(crate) fn reachable(source: &Value, target: &Value) -> Result<bool, VmError> {
    let id = target.counter().unwrap().id;
    let mut stack = vec![source.clone()];
    let mut seen = BTreeSet::new();
    let mut work = 0usize;
    while let Some(value) = stack.pop() {
        let Some(counter) = value.counter() else {
            continue;
        };
        if counter.id == id {
            return Ok(true);
        }
        if !seen.insert(counter.id) {
            continue;
        }
        let (cells, keys) = slots(&value)?;
        work = work.checked_add(1 + cells.len() + keys).ok_or_else(limit)?;
        if work > POLICY.get().max_graph_nodes {
            return Err(limit());
        }
        for cell in cells {
            stack.push(cell.graph_value()?);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn function(values: Vec<Value>) -> Value {
        let value = Value::Function(crate::value::FunctionValue(Rc::new(
            crate::value::FunctionData {
                capture_profile: crate::value::CaptureProfile::Scalar,
                origin: Rc::new(()),
                owner: None,
                target: crate::value::FunctionTarget::Host { slot: 0 },
                bound_count: values.len(),
                bound: values.into_iter().map(|v| Cell::new(v).unwrap()).collect(),
                counter: Counter::new().unwrap(),
            },
        )));
        register(&value).unwrap();
        value
    }
    fn array(value: &Value) -> crate::container::ArrayValue {
        let Value::Array(a) = value else { panic!() };
        a.clone()
    }
    #[test]
    fn direct_array_cycle_is_rejected_without_publishing_or_leaking_a_cycle() {
        let value = Value::array(vec![Value::Undefined]).unwrap();
        let cell = array(&value).cell(0).unwrap();
        let mut p = Projection::default();
        p.write(cell.clone(), value.clone()).unwrap();
        assert_eq!(p.check().unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(cell.read().unwrap(), Value::Undefined);
    }
    #[test]
    fn function_array_function_cycle_uses_stable_capture_edges() {
        let value = Value::array(vec![Value::Undefined]).unwrap();
        let f = function(vec![value.clone()]);
        let root = Cell::new(f.clone()).unwrap();
        let cell = array(&value).cell(0).unwrap();
        let mut p = Projection::default();
        p.write(cell.clone(), f).unwrap();
        assert_eq!(p.check().unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(cell.read().unwrap(), Value::Undefined);
        drop(root);
    }
    #[test]
    fn longer_function_mapping_array_function_cycle_is_rejected() {
        let leaf = Value::array(vec![Value::Undefined]).unwrap();
        let map = Value::mapping(vec![(Value::Int(0), leaf.clone())]).unwrap();
        let f = function(vec![map]);
        let root = Cell::new(f.clone()).unwrap();
        let source = array(&leaf).cell(0).unwrap();
        let mut p = Projection::default();
        p.write(source.clone(), f).unwrap();
        assert_eq!(p.check().unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(source.read().unwrap(), Value::Undefined);
        drop(root);
    }
    #[test]
    fn a_shared_dag_is_accepted_and_shared_height_is_reused_at_a_deeper_path() {
        let leaf = Value::array(vec![Value::Int(0)]).unwrap();
        let inner = Value::array(vec![leaf.clone()]).unwrap();
        let outer = Value::array(vec![leaf, inner]).unwrap();
        {
            let _scope = Scope::new(ValueLimits {
                max_depth: 3,
                ..Default::default()
            });
            values(std::slice::from_ref(&outer)).unwrap();
        }
        {
            let _scope = Scope::new(ValueLimits {
                max_depth: 2,
                ..Default::default()
            });
            assert_eq!(values(&[outer]).unwrap_err().kind, VmErrorKind::Limit);
        }
    }
    #[test]
    fn replacing_an_old_edge_checks_the_final_graph_not_the_union_of_edges() {
        let value = Value::array(vec![Value::Undefined]).unwrap();
        let f = function(vec![value.clone()]);
        let aroot = Cell::new(value.clone()).unwrap();
        let froot = Cell::new(f.clone()).unwrap();
        let Value::Function(fun) = &f else {
            unreachable!()
        };
        let capture = fun.0.bound[0].clone();
        let member = array(&value).cell(0).unwrap();
        Cell::replace_payload_batch(vec![
            (capture.clone(), SlotPayload::Value(Value::Undefined)),
            (member.clone(), SlotPayload::Value(f)),
        ])
        .unwrap();
        assert_eq!(capture.read().unwrap(), Value::Undefined);
        values(&[value]).unwrap();
        drop((aroot, froot));
    }
    #[test]
    fn changing_a_member_checks_live_ancestors_instead_of_only_the_new_value() {
        let inner = Value::array(vec![Value::Undefined]).unwrap();
        let outer = Value::array(vec![inner.clone()]).unwrap();
        let root = Cell::new(outer).unwrap();
        let nested = Value::array(vec![Value::Int(1)]).unwrap();
        let member = array(&inner).cell(0).unwrap();
        let _scope = Scope::new(ValueLimits {
            max_depth: 2,
            ..Default::default()
        });
        assert_eq!(member.replace(nested).unwrap_err().kind, VmErrorKind::Limit);
        assert_eq!(member.read().unwrap(), Value::Undefined);
        drop(root);
    }
    #[test]
    fn stale_owner_cache_entries_do_not_charge_an_unrelated_live_graph() {
        let _scope = Scope::new(ValueLimits {
            max_graph_nodes: 24,
            ..Default::default()
        });
        let value = Value::mapping(vec![]).unwrap();
        let former = Cell::new(value.clone()).unwrap();
        let unrelated = Value::array(vec![Value::Int(0); 20]).unwrap();
        former.replace(unrelated).unwrap();
        let Value::Mapping(map) = value else {
            unreachable!()
        };
        map.member(&Value::Int(0)).unwrap();
        assert_eq!(map.len().unwrap(), 1);
    }
    #[test]
    fn an_insertion_checks_ancestor_budget_and_leaves_no_mapping_node() {
        let map = Value::mapping(vec![]).unwrap();
        let outer = Value::array(vec![map.clone()]).unwrap();
        let root = Cell::new(outer).unwrap();
        let Value::Mapping(map) = map else {
            unreachable!()
        };
        let _scope = Scope::new(ValueLimits {
            max_graph_nodes: 4,
            ..Default::default()
        });
        assert_eq!(
            map.member(&Value::Int(0)).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert_eq!(map.len().unwrap(), 0);
        drop(root);
    }
    #[test]
    fn structural_work_counts_each_header_and_stable_slot_once() {
        let f = function(vec![Value::Int(1)]);
        let root = Value::array(vec![f.clone(), f]).unwrap();
        {
            let _scope = Scope::new(ValueLimits {
                max_graph_nodes: 5,
                ..Default::default()
            });
            values(std::slice::from_ref(&root)).unwrap();
        }
        {
            let _scope = Scope::new(ValueLimits {
                max_graph_nodes: 4,
                ..Default::default()
            });
            assert_eq!(values(&[root]).unwrap_err().kind, VmErrorKind::Limit);
        }
    }
    #[test]
    fn scalar_function_depth_is_zero_but_header_captures_add_a_level() {
        let scalar = function(vec![Value::Int(1)]);
        {
            let _scope = Scope::new(ValueLimits {
                max_depth: 0,
                ..Default::default()
            });
            values(std::slice::from_ref(&scalar)).unwrap();
        }
        let container = Value::array(vec![scalar]).unwrap();
        let captured = function(vec![container]);
        let _scope = Scope::new(ValueLimits {
            max_depth: 1,
            ..Default::default()
        });
        assert_eq!(values(&[captured]).unwrap_err().kind, VmErrorKind::Limit);
    }
    #[test]
    fn every_graph_workspace_failure_keeps_the_real_slot_unchanged() {
        for after in 0..9 {
            let target = Cell::new(Value::Int(7)).unwrap();
            let value = Value::array(vec![Value::Int(1)]).unwrap();
            let result = crate::copy_plan::with_workspace_failure(after, || target.replace(value));
            assert_eq!(result.unwrap_err().kind, VmErrorKind::Limit);
            assert_eq!(target.read().unwrap(), Value::Int(7));
        }
    }
    #[test]
    fn weak_indexes_do_not_keep_headers_or_cells_alive() {
        let value = Value::array(vec![Value::Int(0)]).unwrap();
        let weak = Rc::downgrade(&array(&value).0);
        let root = Cell::new(value).unwrap();
        cells(std::slice::from_ref(&root)).unwrap();
        let weak_cell = Rc::downgrade(&root);
        drop(root);
        assert!(weak.upgrade().is_none());
        assert!(weak_cell.upgrade().is_none());
    }
    #[test]
    fn long_function_chains_are_checked_without_recursive_dfs() {
        let mut chain = vec![function(vec![Value::Int(0)])];
        for _ in 0..1000 {
            chain.push(function(vec![chain.last().unwrap().clone()]));
        }
        assert_eq!(
            values(&[chain.last().unwrap().clone()]).unwrap_err().kind,
            VmErrorKind::Limit
        );
        // Keep raw views until all private captures are cleared, so fixture
        // destruction itself does not recursively drop a synthetic deep chain.
        for value in &chain {
            let Value::Function(f) = value else {
                unreachable!()
            };
            for cell in &f.0.bound {
                cell.clear_on_owner_drop();
            }
        }
    }
    #[test]
    fn hidden_loop_ownership_is_checked_but_cannot_be_published_in_a_header() {
        let source = Value::array(vec![]).unwrap();
        let owner = array(&source);
        let payload = SlotPayload::Loop(Box::new(crate::vm::LoopControl::for_test(owner)));
        let local = Cell::new(Value::Undefined).unwrap();
        let mut p = Projection::default();
        p.payload(local, &payload).unwrap();
        p.check().unwrap();
        let outer = Value::array(vec![Value::Undefined]).unwrap();
        let root = Cell::new(outer.clone()).unwrap();
        let member = array(&outer).cell(0).unwrap();
        let mut p = Projection::default();
        p.payload(member.clone(), &payload).unwrap();
        assert_eq!(p.check().unwrap_err().kind, VmErrorKind::UnsupportedType);
        assert_eq!(member.read().unwrap(), Value::Undefined);
        drop(root);
    }
}
