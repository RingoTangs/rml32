# D06: actual managed frames, parameters and cleanup responsibility

Baseline: 891f55b5dae527dd8da425dcab0a6d903b7e4702, containing 921 files. All new behavior is test-only. The private ObjectStore::with_test_managed_lifecycle(ValueLimits) selects an immutable third mode, sharing the real Registry, origin and 2BE close protocol. Core and Host modes retain their existing managed-entry rejection. No public with_bounded_lifecycle constructor is added.

## Installed frames and actual task roots

The existing InitializationTask and ConstructionTask drive execution. The task object ID is metadata, not another ordinary reference or synthetic execution frame. Every installed Object initialization/function frame has one ExecLease in the real frame-ID table. Frame clones own none. Ordinary helper calls retain the existing call/return hooks.

Managed initialization preflights program eligibility, frame limits, vector workspace and the frame ID before allocating the actual object. Variable construction failure leaves prior completion/failure and records unchanged; monotonic IDs may have gaps. The fresh object's exec count is zero and capacity is at least two, so acquiring its first execution lease is infallible after registration. Only after all preparation succeeds are old execution leases and roots retired and the new root installed.

Initialization root replacement acquires the incoming lease before releasing the outgoing frame. Opcode 48 releases the final initialization lease or hands off to construction. Create/destruct entry preflights arguments, graph limits and execution lease before installing the real function frame. Task unwinding releases actual frames newest-first before clearing them; no task-level execution lease or fake model frame bridges phases.

ConstructionTask.parameter remains the actual OwnedValue. In managed mode its mapping header gains a checked semantic owner in the same context; argument Cells are bound to that context. Mapping copies share member slots, so copying the parameter does not duplicate each member's Object payload lease. Completion, failure, cancellation and extraction retire task ownership once.

The independent audit includes references to actual Registry entries and the actual task parameter, along with real variable/capture/member Cells, Function creators, queue leases and installed frame IDs. Managed-mode graph auditing independently counts actual Array/Mapping root owners against header counters. This makes omission of a task parameter detectable even when its shared members remain owned elsewhere. Negative tests separately reject an omitted parameter and an incomplete frame projection.

## Task advancement and failure

The existing validated constitution order is preserved: all initializers run before create; rollback walks the visited cursor in reverse, including layers with no create method. Explicitly bound synthetic destruct functions run only as the existing failure rollback requires. Their errors are appended while the original primary diagnostic, source, function, constitution, PC and offset are retained. Committed shared mapping writes are not rolled back.

Budget exhaustion preserves frames, cursor and roots. run(0) never advances or cancels a pending task. Host calls, Function operations and other existing task restrictions are not lifted; no real native efun or automatic business destructor is added.

An external remove request marks the executing object pending. At the next positive-budget step, before script execution, the managed path raises InvalidObject and terminates the task without executing further create/destruct bodies. During rollback, this diagnostic is appended and the original primary error is returned. Ordinary non-managed pending behavior is unchanged.

## Explicit deferred cleanup policy

Initialization failure, completed failure rollback, private cancellation and unfinished into_objects establish cleanup responsibility before destroying remaining task roots and frames. Variables are not cleared on these error paths. An active task object becomes pending; the last real frame exit enqueues it using the reserved queue owner.

A failed phase entry or completed rollback can have zero installed frames. The private enqueue_task_cleanup operation then transitions the existing live record through pending to queued immediately, with one preallocated queue node and checked reserved ordinary owner. It allocates no frame or variable workspace. Repeated requests on queued/shell/freed records do nothing. This is an explicit Rust managed-test policy, not the frozen model's immediate zero-exec deletion event or a claim about native timing.

Queued objects are invalid and retain variables until explicit drain. Borrow/workspace failure during drain preserves queue position, payloads and cleanup responsibility. Successful drain clears once, releases the queue owner and reaps freed records. Budget/return/error/cancel/extraction never implicitly drain this queue.

Extraction cancels unfinished tasks and preserves origin, mode and queue in the actual ObjectStore. Rebuilding a VM does not resurrect queued objects. True close continues 2BE's protocol: cancel frames/task roots, close the epoch, detach callback bindings and park actual storage until Host roots retire. Terminal structural teardown may clear the queue; it is not evidence of scheduler progress or implicit per-run cleanup.
