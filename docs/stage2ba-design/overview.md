# Stage 2BA: bounded lifecycle design

D01 selects a future explicit opt-in lifecycle mode while preserving legacy constructors and Rust failure atomicity. Runtime implementation and direct owner capture remain gated. Unpublished native H/A semantic cleanup remains an uncovered difference.

Read [design.md](design.md), [api-migration.md](api-migration.md), [case-map.md](case-map.md) and [enumeration.json](enumeration.json). All 115 stage2AZ cases retain their original evidence basis and findings in the machine-readable map. Native unknowns remain unknown.

Validation: `python3 -B docs/stage2ba-design/verify_validation.py` replays actual compressed command logs, verifies all 800 baseline files (README/roadmap preserve the complete old suffix), checks delivery hashes and rechecks the independent model report. `--write` reruns the declared checks and writes the receipt. No reference executable or business bytecode is run.
