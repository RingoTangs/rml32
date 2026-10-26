# D05: checked Host roots and terminal storage ownership

Baseline: d237bf3a55e0d37084184cc3b3efbbb6e0c1c8e3 (893 files). The private cfg(test) boundary::ObjectStore::with_test_host_lifecycle(ValueLimits) selects Host boundary mode at construction. Core 2BD mode still rejects Host programs. No public with_bounded_lifecycle API is introduced.

## Actual roots and transports

The actual Registry entries retain their OwnedValue payloads. In Host mode, raw Object payloads acquire the same checked ordinary lease used by actual VM cells; Function heads acquire semantic head ownership and exactly one creator lease per live head. No simulated Host slot table implements runtime behavior. Existing Cell/OwnedValue/CopyPlan and container implementations remain the transport machinery.

Host-created Array/Mapping members use the same context-bound factories as VM values. Unknown/freed objects, foreign executable origins and unbound legacy/foreign graphs reject without adoption. Existing ValueLimits graph/root checks remain in force; usize::MAX is the private ordinary capacity, not a proposed production resource policy.

Host retain, member export and callback results preserve existing raw boundary semantics. A new Host owning slot adds one lease, snapshots add none, and aliases to containers do not duplicate member slots. Queued/shell payloads can be transported while an existing owner keeps them present; freed IDs cannot resurrect. Script COPY performs its existing normalization. This deliberately differs from 2BA D01's future Host COPY row: source-normalizing Host retain is deferred, not silently claimed complete or proven native behavior.

Completion extraction preflights registry checks, graph/snapshot validation, root capacity, ID increment and vector reservation before take_owned. It transfers the existing completion lease, then claims once; it does not reserve a second payload owner. Borrow/capacity failure preserves completion for retry. Callback Borrowed/Owned results use temporary actual OwnedValue owners before VM transaction publication, matching the existing boundary's raw transport sequence rather than promising a zero-copy callback result.

## Direct synchronous callbacks and safe points

Only already-prepared direct opcode 0x06 executes in the new mode. Core mode still rejects it. Host Function targets/indirect Host calls, hidden Loop payloads and managed initialization/construct entry points stay gated. Object/Global Function payloads may still be retained where their existing preparation permissions allow them. No real native efun is implemented.

Callback arguments are borrowed views, not new roots. HostScope retain/value/array/mapping creates actual Registry roots. The wrapper keeps a Weak Registry backlink; the scope cannot execute or mutate a VM. Pre-callback validation/allocation failure has zero effects. After a callback returns, bad indices/origins, graph/signature errors or transaction workspace failure release provisional owners and preserve sticky failure; subsequent run does not replay the callback. A root explicitly saved by the callback remains owned until its own retirement.

Host retirement preserves the existing protocol: Drop marks a preallocated flag; a successful no-borrow registry drain releases each retired entry once. Borrow conflict defers release. Registry checks, unborrowed root Drop and boundary run continue their existing Host-root safety points. None of them progresses the Object destruction queue. Explicit test drain/reap remains required; budget, return, error, extraction and transfer do not imply object cleanup progress.

## One storage owner across close and transfer

Before close, actual storage is owned by the boundary store or VM. into_objects transfers it, preserving Registry identity, context, immutable mode, bindings and queue. Wrapper destruction after transfer sees inner=None and cannot close the recipient.

True close first retires VM frames through the existing top-down cancellation path, marks Registry and lifecycle epoch closed, and moves the sole actual ObjectStore into Registry.closed_store. Closed handles reject new import, retain, snapshot and executable reuse. Physical Object records remain while retained or borrow-deferred retired roots remain.

The store's Host callback map is taken before parking. The detached bindings are dropped after parking, outside all Registry borrows. This breaks the internal store -> user callback -> captured HostValue -> Registry -> parked store cycle. An externally retained callback/root remains an external owner; this batch does not collect arbitrary user-created Rc cycles.

Once entries are empty, registry drain removes the parked store under a short borrow and destroys it after that borrow ends. If retirement was blocked and no later drain occurs, Registry's field order destroys entries before the parked store. Terminal ObjectStore teardown closes cells and releases queue ownership using 2BD's structural close policy; it is not a scheduler/destructor progress oracle. There are no business destructor callbacks. Internal payload/creator/exec/queue leases retain metadata only, never Registry or the actual arena.

## Independent audit

The audit receives references directly to the actual Registry entry payloads, including retired entries not yet drained. It independently scans these roots plus registered actual cells, Function creators, installed frame IDs and queue ownership. It does not infer Host counts from fixture handles or cached root counts. Missing-root corruption must fail. Existing graph reachability audits inspect bound member/capture cells. Per-trace Weak probes distinguish logical state from actual record, Registry and StoreState teardown.
