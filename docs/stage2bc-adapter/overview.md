# Stage 2BC: real value-slot and container adapter

A test-only adapter now carries Object and creator leases through actual Cell payloads, OwnedValue and CopyPlan transactions. It exercises real FunctionData and the existing acyclic Array/Mapping implementation. Production runtime and public APIs are unchanged, and full lifecycle implementation readiness remains false.

Read [architecture.md](architecture.md), [method.md](method.md), [trace-manifest.json](trace-manifest.json), [case-coverage.json](case-coverage.json) and [next-batch.md](next-batch.md). Native unknowns retain their original evidence status.

Run `python3 -B docs/stage2bc-adapter/verify_validation.py` for read-only verification. Add `--write` to rerun current checks, pinned-baseline historical checks and both separate workers, then regenerate only 2BC results and receipt. Historical Python results are explicitly baseline-only.
