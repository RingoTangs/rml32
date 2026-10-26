# Method

Baseline is c85a1c07bee399507cc3387e6177973aa822d2ef, stage2AZ, 800 tracked files. Existing evidence and prior validation logs are frozen. No new native analysis is performed. Case hashes use sorted compact JSON; source file hashes use exact bytes. Classification is conservative: modeled means a bounded ownership-policy slice, partial-model retains omissions, native-unknown always retains unresolved native status, excluded means outside this event grammar.

Enumeration performs breadth-first exploration with exact serialized state deduplication, no ID renaming or progress fairness. It counts accepted and rejected attempted transitions separately. Bounds are depth 6/4, 200000 states and 1000000 attempted transitions per domain. Depth cutoffs define the domain; capacity cutoffs mean incomplete. Negative cap tests ensure incompleteness is reported. Source hashes bind the stored report to the model and enumerator.

Validation runs the 195 frozen Python tests and current model/design/receipt tests, debug and release Rust tests (644 ordinary plus two doctests each), build, formatting and strict clippy. Historical receipt validation replays frozen stage2AZ logs without invoking its whole-delivery verifier. The 2AW corpus result is reused with rerun=false and an exact baseline hash. Compressed logs contain actual commands, exit codes and complete stdout/stderr.
