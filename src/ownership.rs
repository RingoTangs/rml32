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
    ceiling: Cell<usize>,
    pub(crate) context:
        std::cell::RefCell<Option<std::rc::Weak<crate::lifecycle_adapter::AdapterContext>>>,
    epoch: std::cell::RefCell<Option<Rc<crate::lifecycle::Epoch>>>,
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
            ceiling: Cell::new(usize::MAX),
            context: std::cell::RefCell::new(None),
            epoch: std::cell::RefCell::new(None),
        }))
    }
    pub fn owners(&self) -> usize {
        self.owners.get()
    }
    pub(crate) fn bind(
        &self,
        ctx: &Rc<crate::lifecycle_adapter::AdapterContext>,
        ceiling: usize,
    ) -> Result<(), VmError> {
        if self
            .context
            .borrow()
            .as_ref()
            .and_then(std::rc::Weak::upgrade)
            .is_some_and(|old| !Rc::ptr_eq(&old, ctx))
        {
            return Err(VmError::new(
                VmErrorKind::InvalidState,
                "foreign counter context",
            ));
        }
        if self.owners() > ceiling {
            return Err(VmError::new(VmErrorKind::Limit, "header owner capacity"));
        }
        self.ceiling.set(ceiling);
        *self.context.borrow_mut() = Some(Rc::downgrade(ctx));
        *self.epoch.borrow_mut() = Some(ctx.epoch.clone());
        Ok(())
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        if let Some(epoch) = self.epoch.get_mut() {
            epoch.touch();
        }
    }
}
#[derive(Debug)]
pub(crate) struct Owner(Rc<Counter>);
impl Owner {
    pub fn new(counter: Option<Rc<Counter>>) -> Result<Option<Self>, VmError> {
        counter
            .map(|c| {
                let n = c
                    .owners
                    .get()
                    .checked_add(1)
                    .filter(|n| *n <= c.ceiling.get())
                    .ok_or_else(|| {
                        VmError::new(VmErrorKind::Limit, "semantic owner space exhausted")
                    })?;
                c.owners.set(n);
                if let Some(epoch) = c.epoch.borrow().as_ref() {
                    epoch.touch();
                }
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
        if let Some(epoch) = self.0.epoch.borrow().as_ref() {
            epoch.touch();
        }
    }
}
/// Owner drops before the physical value (field order is intentional).
#[derive(Debug)]
pub(crate) struct OwnedValue {
    pub(crate) _owner: Option<Owner>,
    pub(crate) lifecycle: Option<crate::lifecycle_adapter::Lease>,
    pub value: crate::Value,
}
impl OwnedValue {
    pub fn new(value: crate::Value) -> Result<Self, VmError> {
        Ok(Self {
            _owner: Owner::new(value.counter())?,
            lifecycle: None,
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
    #[test]
    fn raw_slot_and_completion_transports_do_not_reserve_another_owner() {
        let value = crate::Value::array(vec![]).unwrap();
        let counter = value.counter().unwrap();
        let source = crate::cell::Cell::new(value).unwrap();
        counter.owners.set(usize::MAX);
        let mut plan = crate::copy_plan::CopyPlan::default();
        let target = plan.transfer(source.clone()).unwrap();
        plan.commit().unwrap();
        assert_eq!(counter.owners(), usize::MAX);
        assert_eq!(source.read().unwrap(), crate::Value::Undefined);
        let owned = target.take_owned().unwrap();
        assert_eq!(counter.owners(), usize::MAX);
        counter.owners.set(1);
        drop(owned);
        assert_eq!(counter.owners(), 0);
    }
}
