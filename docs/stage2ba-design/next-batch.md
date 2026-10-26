# Implementation review boundary

Next: review the arena/lease teardown design and explicit safety-point locations against existing Host borrowing and sticky failure contracts. Then implement the opt-in mode behind the new constructor, preserving legacy tests. Add Rust regression tests for creator/payload/exec/queue separation, pending COPY, invalid normalization, LIFO cleanup and Host roots after close/into_objects. Add deterministic preflight fault injection around future lease allocation before allowing direct captures.

Implementation readiness is false: arena lease mechanics, borrow-safe reclamation, frame failure unwind integration and exact safety-point placements have not been implemented or proved here. Capture preparation gates require a separate review; this design grants no new bytecode capability. Native unpublished H/A final semantic cleanup and eventual scheduler progress remain separate unresolved questions.
