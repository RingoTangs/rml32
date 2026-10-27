# D07: Host heads, captures and synchronous indirect calls

Baseline: 1927505f6ee5a2eb5c5c6db7eb8121e9e0632936, containing 950 files. ObjectStore::with_test_host_function_lifecycle(ValueLimits) is private and cfg(test). Its immutable mode inherits Host boundary and managed-task support and adds Host Function registration. Core/Host/Managed modes retain their earlier Host Function rejection. No public with_bounded_lifecycle constructor is added.

## Actual creation and creator ownership

The existing prepared 0x17 path creates the real FunctionData and stable capture Cells. New mode permits its Host target in the test lifecycle metadata; it does not change instruction decoding or preparation. Flags, slots 0..558, capture count <=63, supported profiles and argument bounds retain their existing checks. Missing binding at creation rejects before publication; provisional owners are retired on failure.

A Function created in Object context holds one creator ordinary lease per live head. Copies add head ownership, not creator leases. A Global creator has none. Host target identity contains only the slot: no callback, signature, Global program or bank binding is captured. Creation/call consult current bindings; removing a binding can immediately drop the old callback while retained heads remain structurally valid.

The creator must remain alive/pending for a call. Queued/shell/freed creators reject before invoking Host code. Raw Host retain may preserve an existing invalid head under 2BE's policy; it cannot reactivate a retired head or freed Object ID. Function identity snapshots expose no capture payloads and add no owners. Physical Rc views never select unique expansion.

## Capture expansion and one transaction

Only previously prepared scalar and acyclic Array/Mapping capture profiles are admitted. Container members may include the already-supported Object/Function values, but direct Object/Function capture, cycles and escaping capture References remain closed. Nested header aliases share actual member Cells; they do not multiply each member's payload ownership.

Indirect calls validate origin, creator, caller profile, current registration/signature, argument shape and limits before callback invocation. Captures precede newly supplied arguments. Callback context comes from the invoker. Host activation consumes existing quota as before, but installs no Object frame or additional exec lease; actual caller frames remain the only execution owners.

The existing pre-callback semantic owner count fixes unique/shared selection. Unique invocation raw-transfers captures; shared invocation applies existing COPY rules to source slots. This does not recursively normalize Object members of a copied container. Callback retain/release effects and physical views cannot reselect the branch. A private stress test publishes an already-registered observed head during the callback to verify frozen selection; it grants no new public authority to obtain the unique target.

Void/Borrowed/Owned callback results retain the existing raw Host transport policy. Provisional result ownership is prepared before transaction publication. Success consumes the target slot, preserves the chosen capture effects, publishes the result and advances PC once. On final head retirement captures clear and one creator lease releases. Observation aliases to the consumed slot see Undefined.

Before-callback failure has zero Host effects. Callback failure, invalid index/origin/type, borrowed capture, graph rejection or workspace failure after invocation preserves actual operands/captures/PC, releases provisional owners and records sticky failure. Subsequent run does not replay the callback. Explicitly saved Host roots remain independent owners. Panics, reentry and asynchronous callbacks remain outside the existing recoverable contract.

## Store, task and audit integration

The real Registry entries, Cell/OwnedValue/CopyPlan implementations and frame-ID table remain in use. Host Scope operations retain their existing authority; no wrapper arena or alternate callback model implements execution. Managed tasks may own a Mapping containing a Host Function, but their Function/Host execution guards remain closed.

Independent audits enumerate actual owning Cells, Registry roots, task parameters, head creators and queue nodes. Graph header owners are independently counted. Scoped corruption tests remove a creator binding, add a duplicate creator owner, omit a capture Cell or introduce an uninstalled exec lease; each must fail audit, then restore the actual state.

Object queue drain/reap stays explicit. Pending final frame exit, shell transport and physical residency keep their prior meanings. into_objects transfers the same origin, mode, bindings and storage; reconstruction does not adopt foreign heads. True close uses 2BE's parked-store policy and detaches callback bindings outside Registry borrows, breaking internal callback/root cycles. Retained roots delay actual storage teardown; their final retirement permits structural release without running business destructors.
