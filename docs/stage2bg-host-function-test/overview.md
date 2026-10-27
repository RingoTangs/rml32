# Stage 2BG: actual Host Function lifecycle tests

The private Host Function mode connects real prepared opcode 17 creation and opcode 05 indirect calls to actual head/creator/capture ownership. It includes the prior Host roots, managed-task and close policies without enabling Function/Host operations inside managed tasks. Existing three lifecycle modes, production behavior, public API, dependencies and preparation permissions remain unchanged.

Read [architecture.md](architecture.md), [method.md](method.md), [trace-manifest.json](trace-manifest.json), [callable-diff.json](callable-diff.json), [case-coverage.json](case-coverage.json) and the [validation receipt](../stage2bg-validation.json).

Run `python3 -B docs/stage2bg-host-function-test/verify_validation.py` for read-only receipt verification. `--write` reruns all declared checks and writes only this batch's reports, logs and receipt. Historical Python checks run in pinned snapshots, separately from the current-source checks. Remaining production decisions are in [next-batch.md](next-batch.md).
