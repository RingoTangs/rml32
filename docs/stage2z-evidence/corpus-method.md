# Reference-type-check corpus audit

`analyze_corpus.py` scans retained Main Magic559 decoded program images in
`docs/compatibility-evidence/programs.json.gz`, traversing initialization,
ordinary functions, and hidden functions without executing bytecode. SHA-256
values for both input artifacts appear in `corpus-summary.json`. Program,
function, hidden-function, instruction, and opcode totals must match the retained
compatibility baseline before artifacts are accepted.

Reproduce from the repository root:

```sh
python3 docs/stage2z-evidence/analyze_corpus.py
python3 -m unittest discover -s docs/stage2z-evidence -p 'test_analyze_corpus.py'
```

The default command compares deterministic JSON bytes without repairing stale
artifacts. Explicit `--write` refreshes the three generated JSON files. Synthetic
tests exercise all segment classes, signed operands, malformed nodes/offsets,
complete occurrence retention, representative grouping, recorded prototype
metadata, deterministic output, unchanged input, and stale-output detection.

There are 11 opcode `0x10` instructions, all in ordinary functions. Raw slot
indices are 0 (once), 2 (four times), 4 (twice), and 5 (four times); all par8
values are zero. Seven occur in the AAA package and four in GS. The retained
corpus contains 7,159 programs, 101,592 ordinary functions, no hidden functions,
and 6,474,773 instructions.

`corpus-occurrences.json` retains all locations: source member, package, member
hash, function name/index, segment and node offsets, PC, raw operands, and the
recorded min/max argument count, local count, and argument flags. Initializers
have null prototype counts because no function prototype is recorded for them.
`corpus-examples.json` contains the first retained occurrence per operand pair
and segment kind, with up to three neighboring nodes on either side (four
representatives). Neighbors are location context only.

Instruction offsets are uncompressed member-relative bytes:
`nodes_offset + pc * 4`. `src/program.rs` decodes each four-byte node as a
little-endian signed i16, signed i8, and unsigned opcode. These offsets are not
ELF addresses or compressed-package offsets.

This audit records prototype metadata without interpreting argument flag bits.
It does not infer runtime value types, reference identity, reachability, or slot
semantics from metadata or neighboring instructions. Counts describe retained
decoded images only, excluding the separate authorization corpus and members
absent from the compatibility baseline, including the known rejected DBA LZSS
input.
