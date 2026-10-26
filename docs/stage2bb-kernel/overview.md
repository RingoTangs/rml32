# Stage 2BB: isolated Rust lifecycle kernel

A test-only Rust implementation of D01 now exercises real semantic leases, Drop, borrow rejection, LIFO destruction and arena reclamation. It is loaded by an integration test and is not linked into the library or inspector. Runtime implementation readiness remains false.

Read [architecture.md](architecture.md) for ownership and failure ordering, [method.md](method.md) for the differential boundary, [differential.json](differential.json) for actual counts, and [case-coverage.json](case-coverage.json) for retained evidence classifications. No native unknown is promoted by matching a model.

Run `python3 -B docs/stage2bb-kernel/verify_validation.py` for read-only receipt verification, or add `--write` to rerun all declared checks and replace only this batch's reports/logs/receipt. The full temporary trace stream is reproducible and is not committed.
