//! Semantic payload owners. Physical Rc views never change this ledger.
use crate::{VmError, VmErrorKind};
use std::{
    cell::Cell,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(1);
#[derive(Debug)]
pub(crate) struct Counter {
    pub id: u64,
    owners: Cell<usize>,
}
impl Counter {
    pub fn new() -> Result<Rc<Self>, VmError> {
        Self::allocate(&NEXT)
    }
    fn allocate(next: &AtomicU64) -> Result<Rc<Self>, VmError> {
        let id = next
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| VmError::new(VmErrorKind::Limit, "header identity exhausted"))?;
        Ok(Rc::new(Self {
            id,
            owners: Cell::new(0),
        }))
    }
    pub fn owners(&self) -> usize {
        self.owners.get()
    }
}
#[derive(Debug)]
pub(crate) struct Owner(Rc<Counter>);
impl Owner {
    pub fn new(counter: Option<Rc<Counter>>) -> Result<Option<Self>, VmError> {
        counter
            .map(|c| {
                let n = c.owners.get().checked_add(1).ok_or_else(|| {
                    VmError::new(VmErrorKind::Limit, "semantic owner space exhausted")
                })?;
                c.owners.set(n);
                Ok(Self(c))
            })
            .transpose()
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        self.0.owners.set(
            self.0
                .owners
                .get()
                .checked_sub(1)
                .expect("semantic owner underflow"),
        );
    }
}
/// Owner drops before the physical value (field order is intentional).
#[derive(Debug)]
pub(crate) struct OwnedValue {
    _owner: Option<Owner>,
    pub value: crate::Value,
}
impl OwnedValue {
    pub fn new(value: crate::Value) -> Result<Self, VmError> {
        Ok(Self {
            _owner: Owner::new(value.counter())?,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_and_owner_exhaustion_are_controlled_without_wrapping() {
        let next = AtomicU64::new(u64::MAX);
        assert_eq!(
            Counter::allocate(&next).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert_eq!(next.load(Ordering::Relaxed), u64::MAX);
        let c = Counter::new().unwrap();
        c.owners.set(usize::MAX);
        assert_eq!(
            Owner::new(Some(c.clone())).unwrap_err().kind,
            VmErrorKind::Limit
        );
        assert_eq!(c.owners(), usize::MAX);
    }
}
