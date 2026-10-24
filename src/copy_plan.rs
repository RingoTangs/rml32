//! Private, non-reentrant COPY/raw-transfer transactions over real stable slots.
use crate::{Value, VmError, VmErrorKind, cell::Cell};
use std::{collections::BTreeMap, rc::Rc};

#[cfg(test)]
thread_local! { static FAIL_WORKSPACE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) }; }
#[cfg(test)]
pub(crate) fn with_workspace_failure<T>(after: usize, action: impl FnOnce() -> T) -> T {
    struct Restore(Option<usize>);
    impl Drop for Restore {
        fn drop(&mut self) {
            FAIL_WORKSPACE.set(self.0);
        }
    }
    let _restore = Restore(FAIL_WORKSPACE.replace(Some(after)));
    action()
}
pub(crate) fn reserve<T>(values: &mut Vec<T>, additional: usize) -> Result<(), VmError> {
    #[cfg(test)]
    if FAIL_WORKSPACE.with(|f| match f.get() {
        Some(0) => true,
        Some(n) => {
            f.set(Some(n - 1));
            false
        }
        None => false,
    }) {
        return Err(VmError::new(
            VmErrorKind::Limit,
            "injected transaction workspace failure",
        ));
    }
    values.try_reserve(additional).map_err(|_| {
        VmError::new(
            VmErrorKind::Limit,
            "transaction workspace allocation failed",
        )
    })
}

#[derive(Debug, Default)]
pub(crate) struct CopyPlan {
    pub outputs: Vec<Rc<Cell>>,
    pub source_updates: Vec<(Rc<Cell>, Value)>,
    // Range/argument batches can contain 65,536 sources. Index deduplication
    // avoids a quadratic scan while preserving the original retirement order.
    source_indices: BTreeMap<u64, usize>,
    // Each edge transfers the existing payload AND Owner, without a retain/drop pair.
    pub semantic_owner_effects: Vec<(Rc<Cell>, Rc<Cell>)>,
}
impl CopyPlan {
    pub fn copy(
        &mut self,
        source: Rc<Cell>,
        normalize: impl FnOnce(&Value) -> Result<Value, VmError>,
    ) -> Result<Value, VmError> {
        let raw = source.read()?;
        let value = normalize(&raw)?;
        if value != raw {
            self.write(source, value.clone())?;
        }
        Ok(value)
    }
    pub fn write(&mut self, cell: Rc<Cell>, value: Value) -> Result<(), VmError> {
        if let Some(&index) = self.source_indices.get(&cell.id()) {
            let old = &self.source_updates[index].1;
            if *old != value {
                return Err(VmError::new(
                    VmErrorKind::InvalidState,
                    "conflicting writes to one source cell",
                ));
            }
            return Ok(());
        }
        reserve(&mut self.source_updates, 1)?;
        self.source_indices
            .insert(cell.id(), self.source_updates.len());
        self.source_updates.push((cell, value));
        Ok(())
    }
    pub fn output(&mut self, value: Value) -> Result<Rc<Cell>, VmError> {
        reserve(&mut self.outputs, 1)?;
        let cell = Cell::new(value)?;
        self.outputs.push(cell.clone());
        Ok(cell)
    }
    pub fn transfer(&mut self, source: Rc<Cell>) -> Result<Rc<Cell>, VmError> {
        reserve(&mut self.semantic_owner_effects, 1)?;
        let target = self.output(Value::Undefined)?;
        self.semantic_owner_effects.push((source, target.clone()));
        Ok(target)
    }
    pub fn commit_payloads(
        mut self,
        payloads: Vec<(Rc<Cell>, crate::cell::SlotPayload)>,
    ) -> Result<(), VmError> {
        let mut hidden = Vec::new();
        for (cell, payload) in payloads {
            match payload {
                crate::cell::SlotPayload::Value(value) => self.write(cell, value)?,
                value => hidden.push((cell, value)),
            }
        }
        if !self.semantic_owner_effects.is_empty() {
            return Err(VmError::new(
                VmErrorKind::InvalidState,
                "hidden payload move is unsupported",
            ));
        }
        if hidden.iter().any(|(cell, _)| {
            self.source_updates
                .iter()
                .any(|(other, _)| other.id() == cell.id())
        }) {
            return Err(VmError::new(
                VmErrorKind::InvalidState,
                "hidden payload aliases COPY update",
            ));
        }
        reserve(&mut hidden, self.source_updates.len())?;
        hidden.extend(
            self.source_updates
                .into_iter()
                .map(|(cell, value)| (cell, crate::cell::SlotPayload::Value(value))),
        );
        Cell::replace_payload_batch(hidden)
    }
    pub fn commit(self) -> Result<(), VmError> {
        Cell::transfer_batch(self.source_updates, self.semantic_owner_effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_alias_updates_merge_and_conflicting_updates_publish_nothing() {
        let source = Cell::new(Value::Int(1)).unwrap();
        let alias = source.clone();
        let mut p = CopyPlan::default();
        p.write(source.clone(), Value::Int(2)).unwrap();
        p.write(alias, Value::Int(2)).unwrap();
        assert_eq!(p.source_updates.len(), 1);
        assert_eq!(
            p.write(source.clone(), Value::Int(3)).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        assert_eq!(source.read().unwrap(), Value::Int(1));
        let mut p = CopyPlan::default();
        p.write(source.clone(), Value::Int(2)).unwrap();
        p.commit().unwrap();
        assert_eq!(source.read().unwrap(), Value::Int(2));
    }
    #[test]
    fn late_borrow_failure_preserves_both_source_updates_and_transfers() {
        let a = Cell::new(Value::Int(1)).unwrap();
        let b = Cell::new(Value::Int(2)).unwrap();
        let mut p = CopyPlan::default();
        let output = p.transfer(a.clone()).unwrap();
        p.write(b.clone(), Value::Int(3)).unwrap();
        b.with_read_borrow_for_test(|| {
            assert_eq!(p.commit().unwrap_err().kind, VmErrorKind::InvalidState)
        });
        assert_eq!(a.read().unwrap(), Value::Int(1));
        assert_eq!(b.read().unwrap(), Value::Int(2));
        assert_eq!(output.read().unwrap(), Value::Undefined);
    }
    #[test]
    fn raw_transfer_moves_one_owner_and_clears_every_alias() {
        let array = Value::array(vec![Value::Int(8)]).unwrap();
        let counter = array.counter().unwrap();
        let source = Cell::new(array.clone()).unwrap();
        let alias = source.clone();
        let mut p = CopyPlan::default();
        let output = p.transfer(source.clone()).unwrap();
        assert_eq!(counter.owners(), 1);
        p.commit().unwrap();
        assert_eq!(counter.owners(), 1);
        assert_eq!(alias.read().unwrap(), Value::Undefined);
        assert_eq!(output.read().unwrap(), array);
        drop(output);
        assert_eq!(counter.owners(), 0);
    }
    #[test]
    fn allocation_failure_leaves_a_move_source_untouched() {
        let source = Cell::new(Value::Int(7)).unwrap();
        crate::cell::with_allocation_failure(0, || {
            let mut p = CopyPlan::default();
            assert_eq!(
                p.transfer(source.clone()).unwrap_err().kind,
                VmErrorKind::Limit
            );
        });
        assert_eq!(source.read().unwrap(), Value::Int(7));
    }
    #[test]
    fn shallow_header_copy_never_visits_or_normalizes_members() {
        let nested = Value::array(vec![Value::Int(5)]).unwrap();
        let source = Cell::new(nested.clone()).unwrap();
        let mut p = CopyPlan::default();
        let v = p.copy(source.clone(), |v| Ok(v.clone())).unwrap();
        let out = p.output(v).unwrap();
        p.commit().unwrap();
        assert_eq!(source.read().unwrap(), out.read().unwrap());
        assert_eq!(nested.counter().unwrap().owners(), 2);
    }
    #[test]
    fn old_mapping_is_destroyed_after_all_member_guards_are_released() {
        let Value::Mapping(mapping) = Value::mapping(vec![(Value::Int(0), Value::Int(1))]).unwrap()
        else {
            unreachable!()
        };
        let member = mapping.lookup(&Value::Int(0)).unwrap().unwrap();
        let root = Cell::new(Value::Mapping(mapping)).unwrap();
        let mut p = CopyPlan::default();
        p.write(root, Value::Undefined).unwrap();
        p.write(member.clone(), Value::Int(9)).unwrap();
        p.commit().unwrap();
        assert_eq!(member.read().unwrap(), Value::Undefined);
    }
    #[test]
    fn old_array_release_preserves_a_retained_member_slot() {
        let Value::Array(array) = Value::array(vec![Value::Int(1)]).unwrap() else {
            unreachable!()
        };
        let member = array.cell(0).unwrap();
        let root = Cell::new(Value::Array(array)).unwrap();
        let mut p = CopyPlan::default();
        p.write(root, Value::Undefined).unwrap();
        p.write(member.clone(), Value::Int(9)).unwrap();
        p.commit().unwrap();
        assert_eq!(member.read().unwrap(), Value::Int(9));
    }
    #[test]
    fn invalid_or_duplicate_move_edges_fail_before_any_publication() {
        let source = Cell::new(Value::Int(1)).unwrap();
        let mut p = CopyPlan::default();
        let a = p.transfer(source.clone()).unwrap();
        let b = p.transfer(source.clone()).unwrap();
        assert_eq!(p.commit().unwrap_err().kind, VmErrorKind::InvalidState);
        assert_eq!(source.read().unwrap(), Value::Int(1));
        assert_eq!(a.read().unwrap(), Value::Undefined);
        assert_eq!(b.read().unwrap(), Value::Undefined);
    }
    #[test]
    fn every_workspace_reservation_failure_preserves_real_sources() {
        for after in 0..7 {
            let source = Cell::new(Value::Int(1)).unwrap();
            let target = Cell::new(Value::Int(2)).unwrap();
            let result = with_workspace_failure(after, || {
                let mut p = CopyPlan::default();
                p.write(target.clone(), Value::Int(3))?;
                p.transfer(source.clone())?;
                p.commit()
            });
            assert_eq!(result.unwrap_err().kind, VmErrorKind::Limit);
            assert_eq!(source.read().unwrap(), Value::Int(1));
            assert_eq!(target.read().unwrap(), Value::Int(2));
        }
    }
    #[test]
    fn a_read_guard_is_compatible_with_a_copy_that_needs_no_source_write() {
        let source = Cell::new(Value::Int(1)).unwrap();
        source.with_read_borrow_for_test(|| {
            let mut p = CopyPlan::default();
            let value = p.copy(source.clone(), |v| Ok(v.clone())).unwrap();
            let output = p.output(value).unwrap();
            p.commit().unwrap();
            assert_eq!(output.read().unwrap(), Value::Int(1));
        });
    }
}
