# Stage 2BD: actual store and VM frame lifecycle tests

The existing ObjectStore and VM now have a test-only lifecycle mode with actual payload/creator leases, installed-frame exec leases, pending deletion, a LIFO destruction queue and explicit drain/reap. Production constructors and execution behavior remain unchanged. Host integration and the public opt-in runtime are still gated.

Read [architecture.md](architecture.md), [method.md](method.md), [trace-manifest.json](trace-manifest.json), [vm-diff.json](vm-diff.json) and [case-coverage.json](case-coverage.json). Newest-first real unwind is recorded separately from the simulated 2BB cancellation order.

Run `python3 -B docs/stage2bd-runtime-test/verify_validation.py` for read-only verification. Add `--write` to rerun all declared checks and generate only this batch's reports/logs/receipt. Historical results are explicitly pinned-baseline results, not current-source tests.
