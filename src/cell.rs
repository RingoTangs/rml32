//! Private stable slots. Borrow guards never leave this module.
use crate::{Value, VmError, VmErrorKind};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT_ID: AtomicU64 = AtomicU64::new(1);
#[derive(Debug)]
pub(crate) struct Cell {
    id: u64,
    value: RefCell<Value>,
}
impl Cell {
    pub(crate) fn new(value: Value) -> Result<Rc<Self>, VmError> {
        let id = NEXT_ID
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| VmError::new(VmErrorKind::Limit, "cell ID space exhausted"))?;
        Ok(Rc::new(Self {
            id,
            value: RefCell::new(value),
        }))
    }
    pub(crate) fn id(&self) -> u64 {
        self.id
    }
    pub(crate) fn read(&self) -> Result<Value, VmError> {
        self.value
            .try_borrow()
            .map(|v| v.clone())
            .map_err(|_| VmError::new(VmErrorKind::InvalidState, "value slot is borrowed"))
    }
    pub(crate) fn replace(&self, value: Value) -> Result<(), VmError> {
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
    // Only called by a final header destructor. All guards are private and are
    // released before any old Value is dropped, including in replace above.
    pub(crate) fn clear_on_owner_drop(&self) {
        let old = self.value.replace(Value::Undefined);
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
}
