# Completion audit method

This is a current-state audit, not a semantic implementation or a measured functional completion percentage. All supported rows are explicitly **bounded**; zero rows claim full native compatibility. Grouped case lists describe the reviewed subsystem envelope. They do not assert that every opcode implements every capability in that envelope. The native handler name/address identifies each individual operation.

`opcode-review.json` is the manually reviewed input: implementation envelope, excluded cases, real attributed Rust test names, existing analysis document/action-table references, dependencies and exit criteria. Related tests support scenarios and may exercise several operations together; their presence does not establish exhaustive per-opcode branch coverage. Missing opcodes have no implementation or semantic-test claim. The action table is dispatch identity evidence, not proof of missing handler semantics.

`verify_completion.py` reads the archived 7,159 parsed program records, counts initialization, ordinary and hidden function nodes independently, and compares the complete opcode histogram and totals with the historical static baseline. It extracts the actual Rust `SUPPORTED_OPCODES` declaration and requires exact equality with the 75 reviewed union rows. It cross-checks native names/addresses, evidence-file existence and actual `#[test]` names, then hashes runtime sources, tests, audit inputs, scripts and referenced evidence documents. No reference ELF, scripts or bytecode are executed. This recount relies on the archived parser output; the separate full-corpus comparison validates that output against the current Rust loader and original bytes.

Default invocation recomputes all outputs in memory and requires byte-identical `matrix.json`, `summary.json` and `matrix.md`; any source-hash change makes the saved audit stale. `--write` intentionally regenerates only those three files after review. It never updates the manual review or source files. Generated outputs deliberately omit timestamps and absolute paths.

```sh
python3 docs/stage2-completion-evidence/test_verify_completion.py
python3 docs/stage2-completion-evidence/verify_completion.py
# Only after reviewing source/boundary changes:
python3 docs/stage2-completion-evidence/verify_completion.py --write
```

Frozen stage 2P counts: 73 corpus opcode kinds, 57 source-supported kinds, 55 intersection, 75 union, 18 missing, extra supported 4d/4e. Missing groups distinguish switch (0a/0b/0c), catch (0e/1a/2c), foreach (12/23/50), function values (05/17), cross-object context (08/41), globals (35/36), host dispatch (06), reference inspection (10) and ranges (45). These group names are planning dependencies, not recovered control-flow proof.

Synthetic verifier tests check set/count boundaries, malformed nodes, stale hashes, nonexistent tests/evidence, and prohibit missing-opcode semantic claims or full-compatibility labels. Repository-wide test execution, original artifact hashes, REA evidence and full-loader comparison are recorded by the stage validation report, not inferred here.
