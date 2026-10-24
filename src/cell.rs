//! Private stable slots. Borrow guards never leave this module.
use crate::{Value, VmError, VmErrorKind};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};
#[derive(Debug, Clone)]
pub(crate) enum SlotPayload {
    Value(Value),
    Loop(Box<crate::vm::LoopControl>),
}
static NEXT_ID: AtomicU64 = AtomicU64::new(1);
// Thread-local, scoped fault injection never changes production allocation behavior.
#[cfg(test)]
thread_local! {
    static FAIL_AFTER: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}
#[cfg(test)]
pub(crate) fn with_allocation_failure<T>(after: usize, action: impl FnOnce() -> T) -> T {
    struct Restore(Option<usize>);
    impl Drop for Restore {
        fn drop(&mut self) {
            FAIL_AFTER.set(self.0);
        }
    }
    let _restore = Restore(FAIL_AFTER.replace(Some(after)));
    action()
}
#[derive(Debug)]
struct OwnedPayload {
    _owner: Option<crate::ownership::Owner>,
    payload: SlotPayload,
}
impl OwnedPayload {
    fn new(payload: SlotPayload) -> Result<Self, VmError> {
        let counter = match &payload {
            SlotPayload::Value(v) => v.counter(),
            SlotPayload::Loop(c) => c.counter(),
        };
        Ok(Self {
            _owner: crate::ownership::Owner::new(counter)?,
            payload,
        })
    }
}
#[derive(Debug)]
pub(crate) struct Cell {
    id: u64,
    value: RefCell<OwnedPayload>,
}
impl Cell {
    pub(crate) fn new(value: Value) -> Result<Rc<Self>, VmError> {
        #[cfg(test)]
        if FAIL_AFTER.with(|counter| match counter.get() {
            Some(0) => true,
            Some(n) => {
                counter.set(Some(n - 1));
                false
            }
            None => false,
        }) {
            return Err(VmError::new(
                VmErrorKind::Limit,
                "injected cell allocation failure",
            ));
        }
        let id = NEXT_ID
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| VmError::new(VmErrorKind::Limit, "cell ID space exhausted"))?;
        Ok(Rc::new(Self {
            id,
            value: RefCell::new(OwnedPayload::new(SlotPayload::Value(value))?),
        }))
    }
    pub(crate) fn id(&self) -> u64 {
        self.id
    }
    #[cfg(test)]
    pub(crate) fn with_read_borrow_for_test<T>(&self, f: impl FnOnce() -> T) -> T {
        let _guard = self.value.borrow();
        f()
    }
    #[cfg(test)]
    pub(crate) fn with_write_borrow_for_test<T>(&self, f: impl FnOnce() -> T) -> T {
        let _guard = self.value.borrow_mut();
        f()
    }
    pub(crate) fn read(&self) -> Result<Value, VmError> {
        self.value
            .try_borrow()
            .map(|v| match &v.payload {
                SlotPayload::Value(v) => v.clone(),
                SlotPayload::Loop(_) => Value::Undefined,
            })
            .map_err(|_| VmError::new(VmErrorKind::InvalidState, "value slot is borrowed"))
    }
    pub(crate) fn is_loop(&self) -> Result<bool, VmError> {
        self.value
            .try_borrow()
            .map(|v| matches!(v.payload, SlotPayload::Loop(_)))
            .map_err(|_| VmError::new(VmErrorKind::InvalidState, "value slot is borrowed"))
    }
    pub(crate) fn loop_control(&self) -> Result<crate::vm::LoopControl, VmError> {
        let guard = self
            .value
            .try_borrow()
            .map_err(|_| VmError::new(VmErrorKind::InvalidState, "value slot is borrowed"))?;
        match &guard.payload {
            SlotPayload::Loop(c) => Ok((**c).clone()),
            _ => Err(VmError::new(
                VmErrorKind::InvalidState,
                "slot has no loop control",
            )),
        }
    }
    pub(crate) fn replace(&self, value: Value) -> Result<(), VmError> {
        let value = OwnedPayload::new(SlotPayload::Value(value))?;
        let old = {
            let mut guard = self
                .value
                .try_borrow_mut()
                .map_err(|_| VmError::new(VmErrorKind::InvalidState, "value slot is borrowed"))?;
            std::mem::replace(&mut *guard, value)
        };
        drop(old);
        Ok(())
    }
    /// Acquire every distinct-cell guard before committing. Drop old payloads only afterwards.
    pub(crate) fn replace_payload_batch(
        updates: Vec<(Rc<Cell>, SlotPayload)>,
    ) -> Result<(), VmError> {
        let (cells, values): (Vec<_>, Vec<_>) = updates.into_iter().unzip();
        let values = values
            .into_iter()
            .map(OwnedPayload::new)
            .collect::<Result<Vec<_>, _>>()?;
        let mut old = Vec::with_capacity(cells.len());
        let mut guards = cells
            .iter()
            .map(|c| {
                c.value.try_borrow_mut().map_err(|_| {
                    VmError::new(
                        VmErrorKind::InvalidState,
                        "value slot is borrowed or duplicated",
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (guard, value) in guards.iter_mut().zip(values) {
            old.push(std::mem::replace(&mut **guard, value));
        }
        drop(guards);
        drop(old);
        Ok(())
    }
    pub(crate) fn replace_batch(updates: &[(Rc<Cell>, Value)]) -> Result<(), VmError> {
        Self::replace_payload_batch(
            updates
                .iter()
                .map(|(c, v)| (c.clone(), SlotPayload::Value(v.clone())))
                .collect(),
        )
    }
    // Only called by a final header destructor. All guards are private and are
    // released before any old Value is dropped, including in replace above.
    pub(crate) fn clear_on_owner_drop(&self) {
        let old = self.value.replace(OwnedPayload {
            _owner: None,
            payload: SlotPayload::Value(Value::Undefined),
        });
        drop(old);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conflicts_are_controlled_errors_and_failed_replace_preserves_value() {
        let cell = Cell::new(Value::Int(7)).unwrap();
        let guard = cell.value.borrow_mut();
        assert_eq!(cell.read().unwrap_err().kind, VmErrorKind::InvalidState);
        assert_eq!(
            cell.replace(Value::Int(9)).unwrap_err().kind,
            VmErrorKind::InvalidState
        );
        drop(guard);
        assert_eq!(cell.read().unwrap(), Value::Int(7));
    }
    #[test]
    fn batch_borrow_failure_does_not_commit_an_earlier_cell() {
        let first = Cell::new(Value::Int(1)).unwrap();
        let second = Cell::new(Value::Int(2)).unwrap();
        let guard = second.value.borrow();
        assert_eq!(
            Cell::replace_batch(&[
                (first.clone(), Value::Int(3)),
                (second.clone(), Value::Int(4))
            ])
            .unwrap_err()
            .kind,
            VmErrorKind::InvalidState
        );
        assert_eq!(first.read().unwrap(), Value::Int(1));
        drop(guard);
        assert_eq!(second.read().unwrap(), Value::Int(2));
    }
    #[test]
    fn hidden_payload_projection_and_all_replacement_paths_release_headers() {
        let Value::Array(owner) = Value::array(vec![Value::Int(1)]).unwrap() else {
            unreachable!()
        };
        let cell = Cell::new(Value::Undefined).unwrap();
        for path in 0..3 {
            Cell::replace_payload_batch(vec![(
                cell.clone(),
                SlotPayload::Loop(Box::new(crate::vm::LoopControl::for_test(owner.clone()))),
            )])
            .unwrap();
            assert_eq!(owner.header_refs(), 2);
            assert_eq!(cell.read().unwrap(), Value::Undefined);
            assert!(cell.is_loop().unwrap());
            {
                let _copy = cell.loop_control().unwrap();
                assert_eq!(owner.header_refs(), 3);
            }
            assert_eq!(owner.header_refs(), 2);
            match path {
                0 => cell.replace(Value::Int(7)).unwrap(),
                1 => Cell::replace_batch(&[(cell.clone(), Value::Void)]).unwrap(),
                _ => cell.clear_on_owner_drop(),
            }
            assert_eq!(owner.header_refs(), 1);
            assert!(!cell.is_loop().unwrap());
        }
        Cell::replace_payload_batch(vec![(
            cell.clone(),
            SlotPayload::Loop(Box::new(crate::vm::LoopControl::for_test(owner.clone()))),
        )])
        .unwrap();
        drop(cell);
        assert_eq!(owner.header_refs(), 1);
    }
    #[test]
    fn hidden_payload_batch_borrow_conflict_and_duplicate_cells_are_atomic() {
        let Value::Array(owner) = Value::array(vec![]).unwrap() else {
            unreachable!()
        };
        let cell = Cell::new(Value::Undefined).unwrap();
        let other = Cell::new(Value::Int(8)).unwrap();
        Cell::replace_payload_batch(vec![(
            cell.clone(),
            SlotPayload::Loop(Box::new(crate::vm::LoopControl::for_test(owner.clone()))),
        )])
        .unwrap();
        let guard = other.value.borrow();
        assert_eq!(
            Cell::replace_payload_batch(vec![
                (cell.clone(), SlotPayload::Value(Value::Int(1))),
                (other.clone(), SlotPayload::Value(Value::Int(2)))
            ])
            .unwrap_err()
            .kind,
            VmErrorKind::InvalidState
        );
        assert!(cell.is_loop().unwrap());
        assert_eq!(owner.header_refs(), 2);
        drop(guard);
        assert_eq!(other.read().unwrap(), Value::Int(8));
        assert!(
            Cell::replace_payload_batch(vec![
                (cell.clone(), SlotPayload::Value(Value::Void)),
                (cell.clone(), SlotPayload::Value(Value::Void))
            ])
            .is_err()
        );
        assert!(cell.is_loop().unwrap());
        assert_eq!(owner.header_refs(), 2);
    }
}
