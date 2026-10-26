# D04: real-store/frame lifecycle, test configuration only

The approved baseline is 1306a9bca296813f83df779fb62aa1a0f651d20c. Public constructors, production behavior, dependencies, preparation capabilities and native evidence are unchanged. with_test_lifecycle is a private test constructor; a store cannot switch modes after creation or adopt a legacy graph implicitly. This batch does not make the public opt-in runtime ready.

## One actual store, independently owned leases

The existing ObjectStore object map remains the single owner of actual Object records and variable Cells. Test-only lifecycle metadata is attached to the same Object identities through the 2BC value adapter; it is not another object arena. Initialization/construct LifecycleState remains orthogonal to Alive/Pending/Queued/Shell/Freed.

Actual Object payloads and one creator per Function head use the existing checked ordinary counter. Queue references use that same counter. ExecLease holds only counter metadata and queue control, never an Object record or store backlink. Embedded singly linked queue nodes retain only metadata; no Object/Function/arena ownership cycle is introduced.

For a live/pending object, ordinary acquisition reserves one count for later queue insertion. Last pending exec release can therefore acquire the queue reference and push the preallocated node without allocation or a fallible borrow. After invalidation no future enqueue is possible; raw retained-shell transport does not reserve a second queue count. Test capacity is private (default usize::MAX), distinct from ValueLimits and native integer widths. It is not a proposed production quota.

Zero ordinary refs do not delete a live object. With execution active, remove sets pending and leaves get/COPY valid. Without execution it preflights actual variable and final-head borrows, invalidates, clears all payloads once and forms a shell or logical freed. Last pending exec exit marks invalid and enqueues LIFO; variables remain until drain.

Drain preflights before detaching, retains the queue reference throughout actual variable destruction, finishes cleanup and drops the queue reference. Borrow/workspace failure leaves the queue and variables unchanged. Reap removes only records already logically freed. A record may remain physically resident until explicit reap; snapshots and record probes distinguish this from logical state. Freed IDs imported as unpublished test inputs normalize to Undefined and cannot acquire a new lease or resurrect a record; this is an explicit Rust test-boundary policy, not a native import oracle.

## Real installed frames, not Frame clones

Frame remains cloneable for diagnostics and context checks. VM owns a separate vector of (real frame ID, ExecLease). Only installed Object frames own leases; Global frames own none. Preparation reserves vector/frame capacity and acquires a prospective lease before publication. A failed call drops the prospective lease without installing a frame. Audit compares actual stack IDs and Object associations with the lease table and independently recomputes execRef per object.

Hooks cover actual start, local/Object/Global calls, Function-value calls, return and catch truncation. Successful restart acquires the new frame lease before retiring the old frames: restarting the same pending object must not briefly invalidate it. Uncaptured errors preserve frames and sticky failure; run(0) does not unwind them. Failed restart preserves them. Test cancel, extraction and VM destruction release the remaining frames newest-first. into_objects preserves the store origin, mode, state and queue. Test-mode finishAction retires execution frames while retained stack slots remain actual value owners.

Newest-first real stack exit differs from 2BB's simulated entry-order cancellation. This is a declared Rust test policy. The new oracle uses explicit model exit events in that order; it never treats the old cancel event as a native timing oracle.

## Actual values, scope and close

Four exact private factories select context-bound Cell, CopyPlan, Array and Mapping construction only under cfg(test). Production variants are the original constructors. Actual container members are bound incrementally, including repeated Function payloads. New Mapping member cells are bound before reference publication. Reachability audit verifies every actual reachable member/capture has the same adapter origin, then counts actual slots, creators and queue nodes independently.

The context shares actual store function_origin. Bytecode Function creation registers its real header and creator before publication; existing scalar/container capture permissions remain unchanged. Host-enabled prepared programs, Host target values, hidden Loop instructions and managed initialize/construct entry points reject before their unsupported effects. Actual Host Registry, real efuns, business destructor hooks, async/reentry, other threads/JIT, escaping capture References and GC are not integrated.

Store close invalidates the test epoch. Frames release before object teardown. Variable payloads clear where unborrowed; a private borrowed observation may retain its Cell until final Rc release, but closed-context operations reject executable reuse. Final close clears queue ownership without invoking business destructors. This terminal teardown is not a native per-object deletion/progress oracle, and does not implement the Host contract of retaining an executable arena after store close.

## Source and evidence boundary

Six existing source files receive test hooks; new runtime and fixture modules are reachable only through test-gated modules. source_projection.py supports only cfg(test), cfg(not(test)) and four exact factory definitions. It expands those factories and compares production token streams to the baseline, qualifying known aliases and normalizing an optional factory argument comma/eta closure. It is a pinned scanner, not a general compiler proof. Legacy Rust test token bodies are also frozen. The entire adapter module remains test-only.

All 115 original cases and 20 native unknowns retain their prior basis. Passing synthetic VM tests does not establish unpublished native H/A cleanup, eventual scheduler progress or full native equivalence. implementation_ready and all direct-capture/stage-completion gates remain false.
