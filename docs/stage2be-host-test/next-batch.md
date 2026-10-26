# Remaining production boundary

Next review managed initialize/construct rollback, ValueLimits-based lifecycle reservation and production exposure. Before adding with_bounded_lifecycle, resolve the explicit D01 Host COPY versus current raw boundary transport difference and define which no-borrow epochs can progress the Object queue. Keep legacy constructors unchanged.

Host Function values/indirect Host invocation remain excluded from the new mode. Their creator/capture expansion and single-effect failure rules require a separate real-path review. Existing production support for these operations does not by itself validate their lifecycle-mode integration.

Closed storage is now retained by actual Host roots in test mode; this does not establish production allocator recovery, arbitrary user Rc-cycle collection, managed task cleanup or native shutdown progress. Direct Object/Function capture, escaping capture References, real efuns, business destructors, async/reentry, threads/JIT and GC remain gated. implementation_ready and stage2_complete remain false; the twenty native unknowns are not promoted.
