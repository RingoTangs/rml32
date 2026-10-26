# D01: bounded lifecycle opt-in (design only)

Status: bounded policy design ready for implementation review; implementation readiness is false. No Rust code, API, dependency or capture gate changes in 2BA. Native evidence comes exclusively from the frozen 2AZ ledger. This document chooses Rust policy where required; it does not claim whole-program equivalence.

## API and compatibility

Future signature: `ObjectStore::with_bounded_lifecycle(ValueLimits) -> Result<Self, VmError>`. The mode is immutable per store. Existing `new` and `with_limits` retain legacy behavior, including immediate removal. Vm construction and `into_objects` carry the same mode and origin; transfer cannot recreate a legacy store accidentally. Preparation permissions remain independent. Direct Object/Function capture, escaping capture-slot References, general GC, real efuns, business destructor hooks, async/reentry, other threads/JIT and initialization/constructor task expansion remain gated.

| Concern | Observed / existing behavior | Future bounded policy |
|---|---|---|
| Result publication failure | Native unpublished H/A final semantic release remains unknown (2AZ C15/C61/C81/C99/C102/C103/C105) | Preserve Rust pre-publication preflight and failure atomicity; uncovered native difference |
| Object with zero ordinary refs | Native live object remains live | No implicit business deletion |
| Deletion with active execution | Native pending 0x4000, not invalid 0x8000 | Preserve distinct pending state |
| Immediate deletion | Native isolate, clear values, unlink; free or mark invalid shell | Policy invalidates before clearing for internal safety; no business hooks/reentry, so no claim about hook-visible intermediate states |
| Final exec exit | Native isolate, LIFO enqueue, invalid flag, queue ref increment | One explicit queue lease; no progress promise |
| Queue drain | Native detach, clear values, release queue ref | Same lease ordering; explicit event only |
| Host close | Existing roots survive privately until retirement | Keep arena control alive until leases retire; reject new imports/uses |
| Existing constructors | Current runtime immediate Object removal | Unchanged legacy branch |

## Ownership and storage decisions

Use origin-bound IDs and one arena owning all Object records. Independent counter leases update semantic counts without an Object record physically owning another record through Rc. A store/registry control block can own the arena; internal leases identify arena entries without strong Rc back-edges. External Host roots keep the control block alive. Define one teardown owner and prohibit internal arena-to-control strong cycles. This is a required implementation invariant, not something the Python model proves.

Ordinary count = actual Object-valued slots + one creator lease per live Function head + queue leases. Function value copies change head owners, never duplicate the creator lease. Captured Object slots are independent payload owners, including when captured Object equals creator. Active Object frames contribute only execRef. Global frames contribute none. Snapshots and aliases to one slot add no payload owner. Host retain creates a new actual owning slot; retirement marks it, and a later safe host drain releases it once.

Keep stable capture cells, existing projected graph checks, CopyPlan and Host single-effect rules. Acquire prospective payload ownership before releasing overwritten values. Preflight all fallible operations before publication. Source normalization is part of the committed COPY transaction. Borrow conflicts, foreign origins and resource failures reject atomically. Physical Rust Rc counts are never used to select unique-head expansion.

## State and event table

| State | Ordinary references | Variables | Valid for COPY/call |
|---|---|---|---|
| alive | May be zero | Present | Yes |
| pending | Independent of positive execRef | Present | Yes |
| queued (invalid) | Includes exactly one queue lease | Await cleanup | COPY normalizes; call rejects |
| shell/tombstone (invalid) | Positive remaining ordinary refs | Cleared once | COPY normalizes; call rejects |
| freed | Zero ordinary and exec refs | Cleared | No |

| Event | Required effect |
|---|---|
| Delete with execRef > 0 | Set pending only; repeated request has no extra lease |
| Delete with execRef == 0 | Isolate, clear variables once, free if count zero else shell |
| Last frame exit from pending | Isolate, prepend queue, invalidate, add queue lease |
| Drain | Detach first queue node, clear variables while queue lease holds storage, release lease, free if zero |
| COPY | Actual destination slot retains payload; invalid Object/creator normalizes actual source and destination |
| Move / unique-head invocation | Transfer raw captures, including invalid payloads, without COPY normalization; release creator once on final head destruction |
| Shared-head invocation | COPY captures into output slots; preserve head and normalize affected sources |
| Overwrite | Retain new payload before retiring old payload |
| Final head release | Clear captures then release one creator lease |
| Host retain / retire | COPY into a Host slot; mark retirement without fallible callback, release at safe no-borrow epoch |
| Cancel / restart | Release frames and VM roots once; queue drains only at separately specified safety points |
| Close / transfer | Close rejects new use but retained Host roots retire safely; transfer preserves origin/mode/state |

Safety points are explicit model events. A queued Object is not guaranteed to drain merely because some scheduler or shutdown registration exists. Native 2AZ progress-frontier unknowns remain unknown. Deferred borrow retirement in the existing Host protocol is preserved in the future design, but this lifecycle model has no borrow scheduler.

## Model interpretation and limits

`model.py` is independent of Rust and native execution. Each event runs on a candidate state, audits, then publishes; injected failure discards the candidate. Maintained counts are checked by an independent scan of actual slots, heads, frames and queue. Corruption tests target counters, queue leases and creator release. Hand tests use at most two objects, two heads, two frames and six owning slots. Alias handles here are observation aliases tied to slot lifetime, not escaping executable References.

The enumeration uses only `explore.actions`: one/two initially live objects, at most one zero-capture head, one frame and two named external slots. It allows create, delete, copy, move, overwrite, snapshot, Host retirement, frame exit, drain, close and transfer. Capture expansion and cancellation are covered by hand tests, not enumeration. Function-container DAG projection, allocator failure locations, resource-limit arithmetic, borrowed Host scheduling, bytecode dispatch and arena memory safety are outside this model. Existing 2AS graph model and Rust checks remain independent evidence; 2BA does not replace them.

No object ID reuse occurs, so tombstones cannot alias a later object. Origin checks remain a Rust implementation prerequisite; this single-store model cannot establish foreign-origin safety. No production limit is inferred from the enumeration bounds. Reaching either cap is incomplete and fails report acceptance.
