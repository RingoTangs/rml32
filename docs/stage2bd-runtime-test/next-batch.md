# Remaining production boundary

Next review the real Host Registry adapter and store-close storage owner. Retained Host roots must preserve the required arena/tombstone lifetime without internal values owning a strong control back-link. Test retained/retired roots under registry borrowing, close, result extraction and into_objects, preserving single callback effects.

The current private mode uses explicit test inputs and safety points. It does not authenticate arbitrary private raw ObjectId values or implement the public with_bounded_lifecycle constructor. Production ValueLimits reservation, real Host no-borrow drain points, managed initialization/construct rollback and final close behavior need a separate review. Keep direct captures, escaping capture References, real efuns, business destructor hooks, async/reentry, other threads/JIT and GC gated.

Native unpublished H/A cleanup and scheduler progress remain unknown. Test-path validation is not production implementation readiness or stage completion.
