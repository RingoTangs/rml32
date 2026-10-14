# Stage 2T audit method

Independent current snapshot derived from stage2S rules; previous reports remain immutable. Recount the archived main-program histogram, check live SUPPORTED_OPCODES and manual bounded cases, native action identities and real VM #[test] names, and hash source/evidence inputs. The separate full-corpus comparison checks original bytes against the current loader. No business execution occurs.

Frozen counts:73 corpus kinds,65 bounded supported,63 intersection,75 union,10 missing,extra4d/4e. No functional completion percentage or measured branch coverage. 12/23/50 gain only +/-1 integer and Array paths. Mapping/source attributes/complex bindings remain excluded. Existing read/reference-call rows note hidden-slot projections and restrictions. New unit tests for private payload lifetime are recorded in the validation manifest; the matrix related_tests only references VM integration tests.

Default verify_completion.py performs read-only byte-identical regeneration; --write regenerates only local matrix.json/summary.json/matrix.md after review. test_verify_completion.py has18 synthetic audit-rule tests. verify_validation.py verifies current delivery and historical hashes, retained Ghidra evidence, actual new test names and readonly corpus results.
