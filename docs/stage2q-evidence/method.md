# Stage 2Q audit method

This is a new snapshot derived from the stage 2P audit rules. Stage 2P files remain immutable. The current verifier recounts archived parsed nodes, checks the live supported set, manual bounds, native action identities and real Rust test names, and hashes all audit inputs. The separate full corpus comparison checks the current loader against original bytes. Neither operation executes shipped bytecode.

Frozen counts: 73 corpus kinds, 59 bounded supported kinds, 57 intersection, 75 union, 16 missing, extra 4d/4e. No functional completion percentage or measured branch coverage is claimed. 08 and 41 reference retained Ghidra evidence plus the new implementation report. Other rows retain their prior boundaries; 07 now notes composition with bounded 08.

Run `python3 docs/stage2q-evidence/verify_completion.py` for read-only byte-identical regeneration and `python3 docs/stage2q-evidence/test_verify_completion.py` for the 18 synthetic audit-rule tests. Only after reviewing changes, `--write` regenerates matrix.json, summary.json and matrix.md in this directory. It never updates stage 2P. `verify_validation.py` independently checks protected historical files, current delivery hashes, retained native evidence and full-corpus results against the stage 2Q validation manifest.
