# Stage 2BF: actual managed-task lifecycle tests

A private immutable managed mode now connects the existing initialization, construction and reverse rollback paths to actual frame leases, parameter ownership and deferred cleanup. The real checked boundary and ObjectStore remain in use. Core/Host modes still reject managed entry; production behavior, public API, dependencies and preparation permissions remain unchanged.

Read [architecture.md](architecture.md), [method.md](method.md), [trace-manifest.json](trace-manifest.json), [managed-diff.json](managed-diff.json), [case-coverage.json](case-coverage.json) and the [validation receipt](../stage2bf-validation.json).

Run `python3 -B docs/stage2bf-managed-test/verify_validation.py` for read-only receipt verification. `--write` reruns all declared checks and writes only this batch's reports, logs and receipt. Historical Python checks run on pinned snapshots; current checks are identified separately. Remaining production work is in [next-batch.md](next-batch.md).
