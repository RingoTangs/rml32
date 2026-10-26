# D03: real-type test adapter

This adapter is registered under cfg(test) only. The production Value representation, Value::counter(), public ObjectStore constructors, VM, preparation capabilities, Host Registry, dependencies and reference artifacts are unchanged. It is not a production lifecycle mode.

## Actual ownership, not mirrored slots

The existing Cell::OwnedPayload and ownership::OwnedValue contain an additional test-only Lease field. Object leases use the existing checked ownership::Owner over an independent per-Object counter; Object never becomes a graph header. Moving OwnedPayload or taking OwnedValue moves this exact lease rather than retaining a substitute. Default legacy construction stores no lease.

AdapterContext provides explicit, origin-tagged BoundValue inputs and creates real Cells, actual Array/Mapping values, and real FunctionData. It has weak cell/owned-value indexes for auditing. It does not install an ambient thread-local mode. Caller inputs must pass the bound-input interface; bare private ObjectId values cannot establish cross-store provenance. Foreign bound inputs, slots, retired heads and implicit adoption of legacy graphs reject. An expired bound context cannot become a legacy slot silently.

Actual container factories and actual stable member cells are used. New unpublished members are bound before returning a graph. The actual graph Scope and projected graph checks enforce the existing DAG and limits; no second container implementation or relaxed preparation gate is introduced. Repeated references to a shared member cell do not duplicate its Object lease. Shallow header COPY does not traverse or normalize members. Actual Array member retention and actual Mapping payload clearing remain distinct.

Function metadata is keyed by its real header counter identity and holds only a weak FunctionData reference. Each actual owning Function payload holds one test lease. The head holds one creator lease while semantic owners exist, independent of function copies and captured Object slots. Final semantic owner retirement clears real capture cells, then releases creator once, even if a physical observation keeps the empty FunctionData allocation alive. The retired metadata rejects revival. Later physical FunctionData destruction can harmlessly clear already-Undefined cells; it does not release creator twice.

Field drop order matters: the existing header Owner drops before the test lease, and both drop before the physical Value. Context indexes never own payload cells or heads strongly. Object leases retain only counter/identity metadata, not Context. Weak probes cover Context, actual cells, Object metadata and real FunctionData at trace teardown.

## Transactions and borrows

CopyPlan has an explicit test adapter constructor. Source, destination, output, transfer and retirement origins are checked, including at commit. Real Cell replacements prepare leases before overwriting; actual transfer_batch moves the entire OwnedPayload, including its lease, without a retain/release pair. take_owned transfers both owners into the actual OwnedValue.

Final-head cleanup preflights capture borrows as a group after prospective retains are reserved. Capture payloads that will move survive that group release; they must not be mistaken for final nested-head destruction. All transaction guards release before old payloads and Function leases drop. Borrow conflicts, duplicate/conflicting updates, graph cycles, graph limits and injected Cell/workspace failures leave published slots and Object/creator counts unchanged. Stale weak graph-index cache metadata may remain, as in the existing transaction contract.

Allocation fault tests use the existing real Cell and CopyPlan workspace hooks. Counter overflow protection is inherited from the existing Owner implementation. This does not establish recovery from system allocator OOM or every allocation site. Directly dropping a private capture while exporting a guard is outside the API: capture-slot Reference escape and reentry remain gated.

## Audit and bounded interpretation

The independent audit enumerates real registered Cells and explicitly registered actual OwnedValue roots. It counts each cell ID once, recounts Object payloads, derives one creator per Function head from actual owning slots, checks creator-lease presence against that derivation, and compares the real Function counter and lease count with the independent slot count. Corruption tests deliberately introduce unregistered payload ownership, a missing creator lease, and an unreleased creator after retirement. OwnedValue transport is registered by the test harness; this is not integration with Host Registry root retirement.

Alive/pending/invalid markers are supplied by test fixtures to exercise COPY and creator validation. They are not real ObjectStore state transitions. No execution references, destruction queue, scheduler, actual start/cancel/restart, shutdown or business destructor is adapted here. Ordinary zero does not imply a native deletion or physical Object record reclamation.

BoundValue is an internal physical construction/view handle, not an additional semantic owner. Its physical lifetime can retain container member cells; snapshots and shared aliases add no new payload owner. Tests release construction views before asserting final Array/Mapping destruction. Unpublished construction handles are not a general executable import interface.

## Production source boundary

Only four existing Rust files receive hooks: lib module registration, Cell payload/transactions, OwnedValue transport and CopyPlan context checks. All new behavior is cfg(test). source_projection.py compares non-test token streams against the pinned baseline, with one explicit equivalence normalization for constructing an OwnedPayload in a temporary before workspace insertion. Old test function token bodies are also compared. It is a fail-closed scanner for these four files, not a general Rust preprocessor or compiler proof.
