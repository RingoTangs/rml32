# Stage 2R runtime integration audit

This is a read-only audit of the existing Rust runtime. **Source facts** below are confirmed from current source and named regression tests; **design constraints** are engineering conclusions. Native loop and switch facts below are supplied by the root agent’s Ghidra audit and are indexed in the accompanying semantics report/evidence export. They are distinguished from Rust source facts and engineering decisions.

## Confirmed source facts

| Area | Current implementation and consequence |
| --- | --- |
| Preparation | `src/prepared.rs:166` decodes segment nodes into `Instruction { opcode, operands: [i32; 2], offset }`. Only 3c/3f currently dereference numeric metadata. Unsupported instructions are rejected; six proposed handlers remain unsupported. |
| Branches | Preparation validates 0f/1f/21 against `pc + 1 + delta` (`src/prepared.rs:257`). `step` increments PC before dispatch (`src/vm.rs:1360`); `jump` adds a displacement to that incremented PC (`src/vm.rs:893`). This is existing branch behavior, not proof that the six new instructions use the same displacement. |
| Frames | `Frame` stores program, object, body, PC, BP, argc and entry phase (`src/vm.rs:186`). No loop/switch state exists. Frames are cloned for each instruction's context and pre-instruction error diagnostic. |
| Dispatch context | `CodeView` supplies body-specific instructions, pool and relocation bases (`src/vm.rs:108`). Inheritance and cross-object calls must continue to use the active body, not root pools or caller-local metadata. |
| Budget | `run(0)` does no work; each successful `step` consumes one budget unit. Exhaustion preserves execution (`src/vm.rs:823`). There is no independent iteration budget or host coroutine. |
| Return | `return_function` requires zero/one result according to the prototype, then truncates arguments/locals/temporaries and pops the frame (`src/vm.rs:1190`). Stray internal loop-state values on the evaluation stack would currently cause return-shape errors. |
| Ordinary error | `run` saves a sticky failure but leaves ordinary frames and stack available for diagnostics. A subsequent permitted `start` replaces them. Committed writes survive (`src/vm.rs:823`, `tests/vm_core.rs:2602`). |
| Managed cleanup | Initialization cancellation/failure removes the owned object and clears stack/frames/reference registry. Construction error handling clears failed task frames and may execute rollback destructors; primary error and rollback errors remain distinct (`src/vm.rs:247`, `:407`, `:426`). This is not general catch/unwind support. |

## Containers and references

- **Source fact:** Arrays have an immutable vector of stable mutable cells; `elements()` returns a shallow value snapshot. Mapping storage is an insertion-order vector plus key index; existing-member writes preserve cell identity, and missing-member access can append an Undefined cell (`src/container.rs:8`, `:38`, `:60`, `:93`). No deletion/reordering or resize operation is exposed by the bounded runtime.
- **Source fact:** Mapping public `entries()` copies keys and values; it does not expose a cursor or a mutable borrow. Its vector order is current Rust construction order, not evidence of native traversal order.
- **Source fact:** Member references own a cell and a quota lease. Locals refer weakly to a cell and require a still-live owner frame; object references recheck object existence. Member writes accept only scalar values (`src/vm.rs:636`, `:653`, `:711`).
- **Source fact:** Dropping the last mapping header clears its value cells even if member references retain those cells (`src/container.rs:43`). Merely retaining a member cell does not retain the native-equivalent collection header.
- **Source fact:** Cell guards never escape the cell module; `replace_batch` releases all guards before dropping old containers (`src/cell.rs:46`). There must be no `Ref`/`RefMut` held across callbacks, nested VM calls, budget returns or header drops.

## Confirmed native constraints from the companion Ghidra audit

- Each 0a/0b instruction consumes a fixed eight-byte metadata record containing a branch displacement and comparison value. There is no aggregate variable-size case table to invent. Bounds and target validation apply separately to every record.
- Loop initialization allocates a 36-byte native type-20 controller into its designated slot and consumes three evaluation inputs: mode, source/end, and bound-variable pointer. Native `bp + varNo` and current Rust `bp - 1 - no` represent the same logical slot convention under opposite stack-growth directions; this does not justify a second numbering system.
- Numeric modes +1/-1 are inclusive loops. Input mode **997 (0x3e5)** selects collection iteration, then becomes 901 for arrays or 902 for mappings.
- The controller retains the source collection header. Array iteration increments an index and reads the current element cell; it is not a copied value snapshot.
- Mapping iteration follows native physical bucket chains and emits a reused length-three array `[key, value, ordinal]`. Current Rust insertion-vector order cannot reproduce that order.
- Conditional and step handlers both delegate to the loop helper: successful continuation updates the bound variable and applies the continuation displacement; exhaustion applies the break displacement. Normal checks/exhaustion do not themselves free the controller.
- Value cleanup of native type20 calls `vm_freeLoopValue`, releasing the retained header. Native value-copy projects a controller to Undefined. Thus overwrite/destruction of the containing slot, rather than merely frame exit, controls lifetime.

## Integration design and implementation order

1. **First implementation batch: switch only.** Decode each fixed eight-byte 0a/0b record during preparation with checked metadata ranges and target arithmetic. Preserve instruction offsets and body-specific string-pool context. Add an internal decoded-case payload or per-body indexed record; do not add public startup APIs or a synthetic runtime collection representing a table. The exact comparison, selector consumption and branch origin follow the companion native evidence.
2. Preserve existing `Vm::start`, `run` and `RunOutcome`. Each handler commits once per instruction, including one-instruction budget runs. Validate illegal metadata/targets before execution where possible. Unknown comparison/type branches remain explicit rejections rather than guessed Rust equality.
3. **Second implementation batch: integer and array loops after hidden-slot ownership.** Extend private `Cell` storage to distinguish ordinary values from an internal controller payload. Ordinary value-copy/read projection of that payload yields Undefined, while loop handlers use private typed controller access. Dropping/replacing the payload releases its held header immediately. A frame-side cursor alone is insufficient: it would retain the source after native slot overwrite.
4. Use existing logical slot addressing and bound-variable reference validation, with private typed controller access. Do not turn controller type20 into a publicly constructible `Value`, return it, or insert it into normal containers. Preserve normal return-shape checks and implement copying according to native Undefined projection.
5. Array state retains the original header and index, reads current cells each iteration, and copies the selected current value through existing normalization rules. Do not call `elements()` to construct a traversal snapshot. Existing immutable array shape is sufficient for the presently observed operations; adding resize semantics remains separate work.
6. Keep mutable controller borrows and `Cell`/mapping guards local to a helper. No guard survives nested calls, budget returns or displaced-header drops. Frame cloning must not deep-copy controller ownership/cursors. Preflight resource reservations, target slots and fallible copies before committing updates; handle aliases explicitly rather than sending duplicate cells to `replace_batch`.
7. Controller ownership must work for overwrite, normal body exit, early return, finish, construction rollback, initialization cancellation and execution replacement. Ordinary terminal errors intentionally retain frames/stack for diagnostics; retain their controllers consistently until those slots are released. Exhaustion alone must not prematurely free the source header.
8. **Mapping loops remain blocked** until the native hash algorithm, bucket layout/chain order and mutation behavior are reproduced. Current mapping insertion order and `entries()` snapshots are incompatible substitutes. The emitted reused triple also needs an explicit aliasing test before support is claimed.
9. Initialization has no ordinary function-local frame. Admit loop instructions there only after validating actual native slot/bound-reference conventions against that context. Unsupported buffer/function/catch paths remain explicit restrictions.

## Regression anchors and required additions

Existing test anchors in `tests/vm_core.rs`:

- `budget_exhaustion_preserves_loop_state_and_can_resume` (397) tests an ordinary branch loop, **not** native foreach.
- `reference_snapshot_and_budget_resume_keep_stable_target_identity` (1567), `pending_references_survive_nested_calls_without_being_passed_as_arguments` (1581), and `mapping_insertion_keeps_existing_member_reference_stable` (1740).
- `body_error_keeps_committed_writes_and_new_start_releases_frames` (2602) and `reference_parameter_cleanup_releases_member_and_temporary_quota` (2758).
- `construction_rollback_can_pause_and_host_cancellation_does_not_run_more_destructors` (4732) and `other_call_budget_pause_preserves_callee_and_restores_return_context` (6019).

Add synthetic tests, after native expected results are resolved, for empty/single/nested loops; cell aliases and header lifetime; mutation between iterations; 07/08 calls inside a loop; exact one-instruction pause/resume; break/return/error/cancellation cleanup; switch hit/miss/default and raw byte strings; malformed/truncated/overlapping tables and illegal targets; and inherited bodies with different metadata/string pools. No reference executable or shipped bytecode is needed for these tests.

## Remaining evidence and implementation gates

The root report owns exact addresses, evidence identifiers, raw instruction layouts and branch formulas; this integration audit does not replace those records. Numeric/string comparison, selector stack effects and target formulas are resolved in the companion report and must be used as the implementation contract. Before integer/array loop implementation, validate hidden-slot read/copy/overwrite behavior, bound-variable aliasing and cancellation cleanup with synthetic tests. Mapping traversal remains deferred until native hash order can be represented faithfully. Any unresolved branch becomes a precise preparation/runtime rejection condition.
