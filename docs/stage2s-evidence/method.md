# Stage 2S audit method

Independent current snapshot derived from stage 2Q audit rules; 2P/2Q/2R stay immutable. Recount all archived main-program nodes, compare the historical histogram, check current SUPPORTED_OPCODES, manual boundaries, native addresses and real #[test] names, then hash all source/evidence inputs. Full-corpus comparison separately verifies the current loader against original bytes. No business execution occurs.

Frozen counts: 73 corpus kinds,62 bounded supported,60 intersection,75 union,13 missing, extra4d/4e. No functional completion percentage or measured branch coverage. Only 0a/0b/0c change status; prior supported rows retain their bounds. Loop dependencies now incorporate stage2R integer/collection and hidden-slot evidence; their status remains unimplemented. Evidence includes retained stage2R Ghidra records and the current runtime report.

Default `verify_completion.py` is read-only byte-identical regeneration; `--write` updates only local matrix.json,summary.json,matrix.md after review. `test_verify_completion.py` contains18 synthetic audit-rule tests. `verify_validation.py` checks protected historical files, changed-source/delivery hashes, retained native evidence and corpus results against the2S manifest.
