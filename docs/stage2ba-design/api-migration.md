# Future migration inventory

Design only. Every row has implemented=false.

| File | Surface | Required change |
|---|---|---|
| src/boundary.rs | ObjectStore::new / with_limits | Keep legacy semantics; add separate opt-in constructor only in future implementation. |
| src/boundary.rs | Vm / into_objects | Carry immutable mode, origin and arena control block across transfer. |
| src/boundary.rs | Registry / HostValue / HostScope | Explicit leases; retirement while borrowed defers exactly one release; close rejects import but keeps retained roots safe. |
| src/object.rs | ObjectStore / Object / remove | Single arena, semantic counters, states and variable cleanup; keep legacy remove branch. |
| src/value.rs | Value::Object / FunctionData | Object slot lease and one creator lease per head; physical Rc count is not semantic count. |
| src/cell.rs | Cell / SlotPayload | Actual stable slots own payload; aliases and snapshots do not duplicate it. |
| src/ownership.rs | Counter / Owner / OwnedValue | Independent counter leases; acquire new owners before releasing overwritten payloads. |
| src/vm.rs | create_function_value / CopyPlan | Preserve preflight and atomic rejection, stable capture cells, graph checks and publication boundary. |
| src/vm.rs | frames / start / cancel / failures | Frame exec leases; unwind once; sticky failure does not imply implicit drain; preflight restart before mutation. |
| src/graph.rs | projected Function-container DAG | Retain graph restrictions; no full GC or escaping capture references. |
| src/prepared.rs | preparation capabilities | Lifecycle opt-in grants no capture permission; direct Object/Function captures remain gated. |
