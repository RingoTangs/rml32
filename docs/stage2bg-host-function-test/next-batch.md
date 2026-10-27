# Remaining production decisions

Before production exposure, review resource quotas and reservation, Object-queue safe points and the D01 Host COPY versus current raw transport difference. The test adapters now cover real store/frame/Host/task/callable paths, but their private capacities and explicit drain policy are not a public production contract.

A next design batch should specify lifecycle limits independently of existing ValueLimits, failure atomicity at reservation exhaustion, and explicit cleanup-progress APIs or boundary behavior. Preserve legacy constructors and preparation permissions. Production implementation readiness requires that review and a separate non-test implementation/validation batch.

Do not infer allocator recovery, unpublished H/A cleanup or eventual scheduler/shutdown progress from these synthetic tests. Direct Object/Function capture, escaping capture References, real efuns, automatic business destructors, async/reentry, threads/JIT and GC remain gated. implementation_ready=false, stage2_complete=false and the twenty native unknowns remain unchanged.
