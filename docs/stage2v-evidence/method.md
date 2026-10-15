# Stage 2V audit method

Current snapshot derives from stage2T audit rules; all older snapshots remain immutable. Recount the retained main corpus, inspect live SUPPORTED_OPCODES, verify manual bounded cases, native action identities, evidence paths and actual Rust test names, then hash all source/evidence inputs. Default regeneration is readonly; --write updates only local matrix/summary outputs after review.

Frozen counts:73 main corpus opcode kinds,68 bounded supported,66 intersection,75 union,7 missing; extras4d/4e. Catch3 gains ordinary function primitives and opcode19 Int/Int(0) recovery only. No functional completion percentage or measured branch coverage. Other arithmetic faults, task catches, callbacks/JIT/thread behavior remain excluded.18 synthetic audit tests validate the review rules; private resource tests are recorded separately in the delivery validation report.

Reuse the22 Ghidra records in docs/vm-catch-evidence unchanged (4 acquired at2U,18 reused then); no new REA queries at2V. Validation checks source provenance and all preexisting protected originals/history. Source changes invalidate older current-source hash checks by design; do not rewrite those historical manifests. The full readonly corpus comparator runs the Rust inspection tool only, never the original binary or business bytecode, and retains the known DBA LZSS mismatch.
