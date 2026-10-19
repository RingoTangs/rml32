# Range-query corpus audit

`analyze_corpus.py` scans the retained Main Magic559 decoded program images in
`docs/compatibility-evidence/programs.json.gz`. It traverses initialization,
ordinary functions, and hidden functions without executing bytecode. Input file
SHA-256 values are embedded in `corpus-summary.json`; totals and opcode counts
must agree with the retained compatibility summary before outputs are accepted.

Reproduce from the repository root:

```sh
python3 docs/vm-range-evidence/analyze_corpus.py
python3 -m unittest discover -s docs/vm-range-evidence -p 'test_analyze_corpus.py'
```

The default command only compares bytes against the retained outputs. Explicit
`--write` refreshes the three corpus artifacts. JSON rendering and gzip timestamps
are deterministic. Unit tests cover all three segment classes, signed operand
boundaries, complete occurrence retention, representative grouping, malformed
nodes and offsets, deterministic output, unchanged inputs, and stale-output
verification without repair.

There are 3,452 opcode `0x45` instructions: 3,440 in ordinary functions and 12 in
initializers. The retained corpus has no hidden functions. The raw operand pairs
are `(0,0)` 1,525 times, `(1,0)` 1,159 times, and `(3,0)` 768 times. Package totals
are 193 AAA, 451 CCS, 265 CSA, 112 DBA, and 2,431 GS.

`corpus-occurrences.json.gz` contains every occurrence, including source member,
package, retained member hash, segment kind, function name/index, segment offset,
PC, raw operands, and member-relative instruction byte offset.
`corpus-examples.json` contains the first retained occurrence for each operand
pair and segment kind, with up to three neighboring instructions on either side
(four representatives total). Neighbors provide location context only.

`src/program.rs` reads `count * 4` instruction bytes and decodes each four-byte
node as little-endian signed i16, signed i8, and unsigned opcode. Consequently
`nodes_offset + 4 * pc` locates the instruction in the uncompressed package member.
These offsets are neither ELF addresses nor compressed-package positions.

This audit does not infer runtime value types, reachability, endpoint semantics,
or operand mode meanings. No source text or adjacent opcode is used as evidence
of runtime type. Counts describe retained decoded images only: they do not recover
members absent from the compatibility baseline, including the known rejected DBA
LZSS input, and they do not include the separate authorization corpus.
