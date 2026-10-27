//! Explicit semantic resource limits and maintenance progress for bounded stores.
use crate::{VmError, VmErrorKind};

/// Immutable per-store limits. Counts include prospective ownership at its peak.
/// These limits do not cap process bytes or maintenance latency.
/// ```compile_fail
/// let limits = rml32::LifecycleLimits::default();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleLimits {
    pub max_objects: usize,
    pub max_function_heads: usize,
    pub max_ordinary_refs_per_object: usize,
    pub max_exec_refs_per_object: usize,
    pub max_owners_per_header: usize,
}
impl LifecycleLimits {
    pub(crate) fn validate(self) -> Result<(), VmError> {
        if self.max_objects > u32::MAX as usize
            || self.max_ordinary_refs_per_object < 2
            || self.max_exec_refs_per_object < 1
            || self.max_owners_per_header < 1
        {
            return Err(VmError::new(VmErrorKind::Limit, "invalid lifecycle limits"));
        }
        Ok(())
    }
}
/// Independent work-unit budgets. Zero never grants implicit work.
/// ```compile_fail
/// let budget = rml32::LifecycleBudget::default();
/// ```
/// ```compile_fail
/// fn maintain(scope: &rml32::HostScope<'_>, budget: rml32::LifecycleBudget) {
///     scope.maintain_lifecycle(budget);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleBudget {
    pub host_roots: usize,
    pub objects: usize,
    pub metadata_entries: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleStop {
    Complete,
    BudgetExhausted,
    BorrowBlocked,
    WorkspaceLimit,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleProgress {
    pub retired_roots: usize,
    pub drained_objects: usize,
    pub reaped_objects: usize,
    pub reclaimed_heads: usize,
    pub pruned_weak_entries: usize,
    pub metadata_examined: usize,
    pub pending_roots: usize,
    pub queued_objects: usize,
    pub scan_complete: bool,
    pub stop: LifecycleStop,
}
impl LifecycleProgress {
    pub(crate) fn new() -> Self {
        Self {
            retired_roots: 0,
            drained_objects: 0,
            reaped_objects: 0,
            reclaimed_heads: 0,
            pruned_weak_entries: 0,
            metadata_examined: 0,
            pending_roots: 0,
            queued_objects: 0,
            scan_complete: false,
            stop: LifecycleStop::BudgetExhausted,
        }
    }
}

/// Shared mutation notice, with no pointer back to storage or its control block.
#[derive(Debug, Default)]
pub(crate) struct Epoch {
    pub value: std::cell::Cell<u64>,
    pub force: std::cell::Cell<bool>,
}
impl Epoch {
    pub fn touch(&self) {
        match self.value.get().checked_add(1) {
            Some(n) => self.value.set(n),
            None => {
                self.value.set(0);
                self.force.set(true);
            }
        }
    }
}
