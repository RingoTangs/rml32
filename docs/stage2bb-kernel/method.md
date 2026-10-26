# Validation method

The pinned baseline is 99f5626ec402499b0d007bcfc04db9e6ee57c756 (820 files). All baseline files remain byte-identical except README and roadmap, whose complete old content remains a suffix. Historical receipt validation replays frozen 2BA command logs; it never invokes the old whole-delivery verifier after document prefixes change. The 2AW corpus result is reused by exact hash with rerun=false.

`differential.py` imports the frozen 2BA model and action grammar. It performs the same breadth-first exploration, retaining a minimal parent trace for every state. Every attempted outgoing edge below depths 6/4 is emitted, including rejections, into temporary deterministic JSONL. Bounds are 200000 states and 1000000 transitions per domain; any capacity cutoff rejects acceptance. Seed creation is setup, not included in edge counts.

An explicitly ignored Rust worker test is invoked separately with the temporary input path. For each edge it creates a fresh kernel, replays the accepted parent trace, applies the candidate action, checks acceptance and the full logical snapshot, and independently audits ownership. Rejections must preserve the previous snapshot. Final kernel destruction must release all Object records and its control; Weak counter probes must expire. This validates physical Rust ownership independently of matching Python-maintained counters.

The generator checks state-set hashes and all domain counts against frozen enumeration.json. Only compact reports and actual compressed worker output are retained; the approximately 6 MB trace stream is removed with its temporary directory. Source hashes bind the report to Rust, generator and frozen model. Full state space, native equivalence and production memory safety are not claimed.

Hand tests add capture/creator distinctions, borrowed cleanup, checked count arithmetic, queue reservation, physical views, cross-origin rejection, Host root lifetimes and deterministic preflight fault injection. The shared differential grammar has zero-capture heads; additional tests are separate evidence rather than an enlargement of its bounds. All 115 original cases retain their basis/findings and unknown status in case-coverage.json.

Full acceptance requires 235 historical Python tests, new Python validators, unchanged baseline Rust scenario names in both profiles, all new ordinary kernel tests, separate worker replay, two unchanged doctests per profile, build, fmt and strict clippy. Receipts record actual outputs and distinguish test-only Rust source addition from runtime/API changes.
