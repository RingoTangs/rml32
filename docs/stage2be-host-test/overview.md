# Stage 2BE: actual Host boundary lifecycle tests

The real Registry, HostValue, completion extraction and direct synchronous synthetic Host callbacks now participate in a private immutable lifecycle mode. Closing a store/VM parks the actual single ObjectStore until Host roots retire; callback bindings are detached to prevent internal cycles. Production behavior, public API, preparation capabilities, dependencies and the reference executable remain unchanged.

Read [architecture.md](architecture.md), [method.md](method.md), [trace-manifest.json](trace-manifest.json), [host-diff.json](host-diff.json), [case-coverage.json](case-coverage.json) and the [validation receipt](../stage2be-validation.json).

Run `python3 -B docs/stage2be-host-test/verify_validation.py` for read-only verification. `--write` reruns all declared checks and writes only this batch's reports, logs and receipt. Historical Python checks run in pinned snapshots, not on current modified sources. The next production boundary is recorded in [next-batch.md](next-batch.md).
