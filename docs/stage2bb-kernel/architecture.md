# D02: test-only Rust lifecycle kernel

The prototype is compiled only by the new integration test target. The library, inspector binary, public APIs, dependencies and capture preparation gates are unchanged. It does not yet make the opt-in runtime implementation ready. All native claims remain those of frozen stage2AZ/2BA evidence.

## Ownership layout

`Kernel` owns an `Rc<Control>`. The control owns a single arena of Object records, VM/Host roots, execution leases and a destruction queue. Each record owns its stable variable slots and an independent `Rc<Counts>`. An Object payload's `Ordinary` lease holds only that counter, not the control or another Object record.

Function payloads hold `FunctionLease`s. Each head owns private capture cells and exactly one optional creator ordinary lease. Head semantic owners are counted separately from physical Rc views. Final semantic release clears captured payloads before releasing the creator, even if a physical observation view keeps the empty head allocation alive. The head registry is weak. Capturing an existing head creates a DAG because capture cells cannot be overwritten through the external event interface; no arbitrary container graph or GC is implemented.

Each execution lease owns the relevant counter and queue control, with no back-link to the arena. The queue is an intrusive singly linked LIFO list of counter nodes. Normal ordinary leases are limited to capacity minus one; the final count is reserved for a queue lease. Last pending exec release can therefore enqueue without allocation, overflow, arena access, callbacks or a fallible borrow. This quota is private to the prototype and is not a proposed production ValueLimits policy.

The only strong control back-links are external Host roots. Registry entries own actual payload slots; a Host root owns a preallocated retirement flag and the control. Root names can be reused, but flag identity prevents an old handle from acting on a replacement entry. Object IDs cannot be reused. IDs passed across stores and Function views are checked against origin identity.

## States, ordering and reclamation

Live ordinary refs reaching zero do not request business deletion. Delete with execution active sets pending; pending remains valid for COPY and new frames. Last execution exit invalidates and prepends a queue node with one ordinary lease.

Immediate deletion preflights all variable and final-head cleanup borrows. Commit marks invalid, clears the actual variable payloads once, then retains a shell or marks freed. Queued deletion preflights before detaching; commit detaches, clears variables while the queue lease protects storage, then releases the queue lease. A failed preflight leaves the queue intact. This is a Rust borrow policy, not a claim about native borrow behavior or hook-visible intermediate states.

Last shell ownership release marks logical freed. The end of each successful explicit event is a physical reap point: records already marked freed are removed from the arena. Reaping never selects or drains pending/queued objects. The only business queue drain is the explicit `drain` event, which processes one node. History stores frozen JSON observations of freed records, not their allocations or owning handles.

COPY reserves a new semantic owner before releasing overwritten values and normalizes invalid Object/creator payloads in the source. Move transfers the existing lease without a temporary owner, including at count capacity. Unique-head invocation raw-moves captures; shared invocation COPYs and normalizes them. Source/capture aliases are observation-only and expire as in the frozen model, including on source normalization; they are not executable References.

## Borrow, failure and close contracts

Events execute directly on Rust ownership structures, not on a cloned candidate world. They validate destinations, origins, counts and borrows, reserve new leases, and run the final fault checkpoint before committing. All released slot guards are dropped before old payloads and final heads are destroyed. Recursive final-head cleanup borrows are checked as a group, including multiple retiring owners of the same head.

Three deterministic preflight checkpoints are available for tested fallible operations. Failure can allocate and free provisional structures, but must leave published logical state and semantic ownership unchanged. System allocator OOM recovery and every actual allocation site are not proved. Commit's standard collection allocations follow Rust's allocator behavior; this prototype is not an allocator-transaction implementation.

Host drop only sets its flag. `drain-host` is explicit, checks registry and payload borrows, and releases each retired entry once. Close marks the control closed and rejects new use; it does not request business deletion or implicitly drain. Close and retirement are safe while payloads are borrowed; auditing is postponed until a no-borrow epoch. Scoped borrowing is internal and cannot be exported as an escaping guard or capture-slot Reference.

Kernel transfer moves the same control and marks the observation; it does not close the store. Cancel/restart release simulated frames in entry order (matching the frozen model), then non-Host roots, but do not implicitly drain. Entry order is an additional kernel invariant; the shared enumerated domain has only one frame. These are kernel events, not integration with VM sticky failure, initialization, constructors or actual start/cancel behavior.

Store owner drop closes access. Surviving external Host roots retain control and the arena until retirement. At final control destruction, frames release first, root and variable payloads clear, queue leases release, then arena records drop. Internal values cannot keep this control alive. Weak probes and Object/Head/Control Drop counters test physical release independently of semantic snapshots.

## Remaining integration work

Real Value/Cell adapters, projected container graph integration, exact VM safety-point placements, sticky failure unwind, Host callback single-effect preservation and pre-publication resource policy remain unimplemented. Direct Object/Function captures, escaping capture references, real efuns, business destructors, async/reentry, other threads/JIT and full GC remain gated. Native unpublished H/A final semantic cleanup and eventual scheduler progress remain unknown.
