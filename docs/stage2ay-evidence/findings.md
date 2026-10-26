# 2AY 结论总账

观察、静态推导、Rust 设计和未知分开；没有原程序运行观测。F01—F40 保留 2AX 历史结论原文；本轮重评见后续结论及案例映射，不能据历史未知项忽略新增的限定结论。

## F01

observation · answered

17 helper explicitly scans captures for tag8 Reference before allocating; this is not a scalar-only native whitelist.

边界：Absence of other tag checks here does not legalize malformed values or all hidden tag shapes.

锚点：[A001](anchors.md#a001), [A002](anchors.md#a002)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

## F02

observation · bounded

Create allocates header then capture storage and establishes creator/target before moving captures. Second allocation failure frees the header before vm_error.

边界：Zero-size allocator policy and final result-stack failure are separate; native creation is not wholly failure-atomic.

锚点：[A003](anchors.md#a003), [A004](anchors.md#a004), [A005](anchors.md#a005), [A006](anchors.md#a006)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

## F03

observation · answered

Capture establishment initializes each 24-byte slot then moves raw type/attrib/payload and clears source type/attrib; no vm_copyValue call occurs in this transfer loop.

边界：Source means the already-evaluated raw stack value; preceding expression reads can already have normalized it.

锚点：[A007](anchors.md#a007), [A008](anchors.md#a008), [A009](anchors.md#a009), [A010](anchors.md#a010), [A011](anchors.md#a011)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

## F04

observation · answered

05 moves its raw target to a C-local value and consumes the stack slot before target/owner validation.

边界：Current Rust preflight/operand preservation is a bounded policy, not this native transaction order.

锚点：[A012](anchors.md#a012), [A013](anchors.md#a013), [A014](anchors.md#a014), [A015](anchors.md#a015)

Evidence：`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`

## F05

observation · answered

Native function head ref==1 selects reverse-index raw capture moves; clears moved slots, frees head, then dispatches saved target metadata.

边界：Test precondition is native semantic ref, not physical Rc::strong_count or diagnostic snapshots.

锚点：[A016](anchors.md#a016), [A017](anchors.md#a017), [A018](anchors.md#a018), [A019](anchors.md#a019), [A020](anchors.md#a020)

Evidence：`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`

## F06

observation · answered

Shared-head branch destructs the temporary call target first, then reverse-index vm_pushValue copies captures while another holder keeps the head alive.

边界：Does not deep-copy nested containers; not a generic restartable callback protocol.

锚点：[A021](anchors.md#a021), [A022](anchors.md#a022), [A023](anchors.md#a023)

Evidence：`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`, `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212`

## F07

observation · answered

vm_copyValue shallow-copies Array/Mapping headers and increments their reference; nested members are not recursively normalized.

边界：Header sharing does not prove cyclic reclamation or permit all graph rewiring.

锚点：[A024](anchors.md#a024), [A025](anchors.md#a025)

Evidence：`ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

## F08

observation · answered

Copy of deleted Object or Function with a nonnull deleted creator cleans the SOURCE, rereads its type, and writes Undefined; cleanup clears source attrib too.

边界：Raw truth/equality, raw moves and owner-null Function are distinct. Physical lifetime is governed by native ref/execRef.

锚点：[A026](anchors.md#a026), [A027](anchors.md#a027), [A028](anchors.md#a028), [A029](anchors.md#a029), [A030](anchors.md#a030), [A031](anchors.md#a031), [A032](anchors.md#a032)

Evidence：`ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`, `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`

## F09

static-inference · bounded

Shared 05 expansion can persistently normalize a dead Object/Function capture to Undefined in a still-live function head. Unique expansion has no copy-based normalization at that point.

边界：Describes immediate expansion before callee 0f; later argument/use rules can reject or normalize independently.

锚点：[A016](anchors.md#a016), [A018](anchors.md#a018), [A022](anchors.md#a022), [A027](anchors.md#a027)

Evidence：`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`, `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

## F10

observation · answered

vm_pushValue checks available stack space before making the destination and copying; overflow precedes source cleanup from copy.

边界：Rust Cell allocation/fault injection is separate; shallow native copy does not introduce one allocator call per capture.

锚点：[A033](anchors.md#a033), [A034](anchors.md#a034), [A023](anchors.md#a023)

Evidence：`ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212`

## F11

observation · answered

Function final release cleans each capture payload and unlinks slots, then unrefs creator, frees capture allocation and head. Moved-out Undefined slots do not release transferred payload again.

边界：Capture-slot survival cannot be inferred from ordinary external member Ref rules; function storage is freed.

锚点：[A035](anchors.md#a035), [A036](anchors.md#a036), [A037](anchors.md#a037), [A038](anchors.md#a038), [A039](anchors.md#a039)

Evidence：`ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9`

## F12

observation · answered

Explicit dereference copies the referenced value before unrefing the original target slot.

边界：Not arbitrary Reference escape; source normalization can be visible through another alias.

锚点：[A040](anchors.md#a040), [A041](anchors.md#a041), [A042](anchors.md#a042)

Evidence：`ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956`

## F13

observation · answered

Member query copies selected Array/Mapping member before unrefing consumed container header; result attrib is subsequently governed by query path.

边界：Missing Mapping lookup is not insertion; copying a header and reading a member are different normalization points.

锚点：[A043](anchors.md#a043), [A044](anchors.md#a044), [A045](anchors.md#a045), [A046](anchors.md#a046)

Evidence：`ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459`

## F14

observation · answered

Array final-header release unrefs element slots; unrefValue cleans/frees a slot only when its ref becomes exactly zero. External element references can retain payload.

边界：No blanket final-header clearing rule for all container kinds.

锚点：[A047](anchors.md#a047), [A048](anchors.md#a048), [A049](anchors.md#a049), [A050](anchors.md#a050)

Evidence：`ev_8000dac56416a0239afddc630d10d5c425e7d9797f623755e77dcda7e5516c26`, `ev_18fea6ebc3680cd0ea4726c67901206fe1299bda13ef18741e124aa8a6eb77de`

## F15

observation · answered

Mapping final-header release traverses buckets/freeNode; freeNode immediately clears key/value even when referenced slots defer physical node reclamation.

边界：Deletion execution API, arbitrary mutation and delayed-node collection schedule remain outside this bounded implementation.

锚点：[A051](anchors.md#a051), [A052](anchors.md#a052), [A053](anchors.md#a053), [A054](anchors.md#a054), [A055](anchors.md#a055), [A056](anchors.md#a056)

Evidence：`ev_2aae5aa932757dd2dad4354599e352e3310dbf9d1f7637b49903392349474966`, `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae`

## F16

observation · bounded

Ordinary nonconst mixed/already-matching return uses takeOffValueFromStack: cleans removed frame values then raw-moves retained result; no blanket copy on that path.

边界：Other return conversion, const and callback/hook paths are separate; do not universally normalize returned dead handles.

锚点：[A057](anchors.md#a057), [A058](anchors.md#a058), [A059](anchors.md#a059), [A060](anchors.md#a060)

Evidence：`ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215`, `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535`

## F17

observation · bounded

GC marker recurses through Array, Mapping, Function captures and selected Loop state after setting slot active; no general tag8 recursion branch is established.

边界：Root hooks, all Reference graphs, reentrant timing and cycle release order remain unproven; Rc is not full native GC.

锚点：[A061](anchors.md#a061), [A062](anchors.md#a062), [A063](anchors.md#a063), [A064](anchors.md#a064), [A065](anchors.md#a065), [A066](anchors.md#a066)

Evidence：`ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471`

## F18

observation · bounded

05 expansion precedes callee installation; callInUserFunBody installs bp/argNum/ip, while 0f checks count/type and can convert earlier arguments before a later error.

边界：This differs from current Rust all-argument preflight; general default expressions and const rules remain excluded.

锚点：[A067](anchors.md#a067), [A068](anchors.md#a068), [A069](anchors.md#a069), [A070](anchors.md#a070), [A071](anchors.md#a071), [A072](anchors.md#a072)

Evidence：`ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c`, `ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126`

## F19

static-inference · bounded

For ordinary interpreter threadType1 and valid script catch, vm_errorQuit/traceError unwind exiting frame values and doActions pops back to saved catch sp; moved captures are released from stack.

边界：Catch is stack recovery, not rollback of consumed Function target, source normalization, prior conversions or external writes. Callback catch and other thread types differ.

锚点：[A073](anchors.md#a073), [A074](anchors.md#a074), [A075](anchors.md#a075), [A076](anchors.md#a076), [A077](anchors.md#a077), [A078](anchors.md#a078)

Evidence：`ev_f1c4dde0e0ce2a55aa3838ae691f272691496d87fcbc3d91ec17f09ea6eeaea1`, `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2`, `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe`, `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e`

## F20

unknown · partial

Uncaught final PCB/task teardown, non-interpreter thread cleanup, JIT recovery and create-result overflow with C-local unpublished head are not closed by this evidence.

边界：Do not promise leak-free rollback or restore operands in these branches; follow main-loop/PCB destruction and allocation publication roots before claiming equivalence.

锚点：[A079](anchors.md#a079), [A075](anchors.md#a075), [A078](anchors.md#a078)

Evidence：`ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2`, `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e`, `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

## F21

design · bounded

Rust origin, allocation limits and success-only publication remain explicit policies. Native head uniqueness needs a semantic ownership model; physical Rc counts include host snapshots/temporary clones.

边界：No runtime change in 2AR. Non-scalar implementation is blocked until ownership accounting and source-mutating copy transactions have an audited design.

锚点：[A016](anchors.md#a016), [A027](anchors.md#a027)

Evidence：`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`, `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

## F22

observation · bounded

Global slot read calls vm_copyValue on the actual shared global Cell; Object and local reads also have explicit source-copy paths.

边界：Inheritance layout and all default/lifecycle combinations remain separate; this is not proof of service execution.

锚点：[A080](anchors.md#a080), [A081](anchors.md#a081)

Evidence：`ev_f5d36761261bd90bb2139ae2bcd05e79af64a90eaffe2b978b3c0353a3ffe4a2`, `ev_9a9cde77fb2df1b8214e667458663471ea6da5fbeea24bfb3878fcb8f7548808`, `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5`

## F23

unknown · partial

Function-held creator ref, captured Object payload ref and active-frame execRef are separate; native delayed deletion cannot be replaced by Rust immediate logical remove.

边界：Direct Object/Function captures remain a later batch until owner invalidation, normalization and final release interactions close.

锚点：[A039](anchors.md#a039), [A015](anchors.md#a015), [A082](anchors.md#a082), [A083](anchors.md#a083), [A084](anchors.md#a084), [A085](anchors.md#a085)

Evidence：`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`, `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9`, `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`, `ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e`

## F24

observation · bounded

A valid nonnull Function creator receives its own ordinary Object ref; capture establishment raw-moves each payload separately. Creator ref, captured Object payload ref, and active-frame execRef are distinct obligations even when identities coincide.

边界：Local create/release/execRef contacts only; no assertion that an invalid null Object target is callable or that Rust Rc implements these native obligations.

锚点：[A086](anchors.md#a086), [A087](anchors.md#a087), [A088](anchors.md#a088), [A009](anchors.md#a009), [A010](anchors.md#a010), [A089](anchors.md#a089), [A082](anchors.md#a082)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`, `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4`, `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`

## F25

observation · answered

vm_unrefFunctionValue decrements head ref and returns while positive. At nonpositive head ref it decrements each capture-slot ref, unconditionally cleans that payload, unlinks the slots, unrefs creator, then frees capture storage and Function head.

边界：This named final-release routine does not establish safe lifetime for external References to capture slots or whole-program reclaim timing.

锚点：[A090](anchors.md#a090), [A091](anchors.md#a091), [A092](anchors.md#a092), [A093](anchors.md#a093), [A094](anchors.md#a094), [A095](anchors.md#a095), [A089](anchors.md#a089), [A096](anchors.md#a096), [A097](anchors.md#a097)

Evidence：`ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4`

## F26

observation · bounded

Destruct with positive execRef sets pending bit 0x4000 and returns. Last execRef with pending bit isolates and queues the Object, sets invalid bit 0x8000 and adds a queue ordinary ref. Queue drain unlinks first, performs low-level deletion, then drops the queue ref; retained invalid shells are released by final ordinary unref.

边界：Known local state chain, not proof that every exit reaches a drain. Ordinary ref reaching zero on a live Object does not itself invoke business destruction; final unref behavior uses the frozen vm_unrefObject pseudocode.

锚点：[A098](anchors.md#a098), [A099](anchors.md#a099), [A100](anchors.md#a100), [A101](anchors.md#a101), [A102](anchors.md#a102), [A103](anchors.md#a103), [A104](anchors.md#a104), [A083](anchors.md#a083), [A105](anchors.md#a105), [A084](anchors.md#a084), [A106](anchors.md#a106), [A085](anchors.md#a085), [A107](anchors.md#a107), [A108](anchors.md#a108), [A109](anchors.md#a109), [A110](anchors.md#a110)

Evidence：`ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b`, `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`, `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63`, `ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e`

## F27

observation · answered

Object COPY and Function creator COPY test invalid bit 0x8000, not pending bit 0x4000. An invalid source is cleaned and its type reread; a merely pending live Object is not normalized solely because deletion was requested.

边界：COPY contact only; pending Object may become invalid at a later last-execRef transition.

锚点：[A111](anchors.md#a111), [A112](anchors.md#a112), [A027](anchors.md#a027), [A028](anchors.md#a028), [A113](anchors.md#a113), [A025](anchors.md#a025)

Evidence：`ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

## F28

static-inference · bounded

For legal stack bounds, nonnegative capture count and no intervening state change, every moved capture increments sp. A nonempty capture therefore supplies the result position; the final sp==0 overflow candidate is zero captures with initial sp==0.

边界：Refines the premise of historical C15; it does not prove that arbitrary malformed negative counts are legal or that the candidate is reached in an actual run.

锚点：[A114](anchors.md#a114), [A079](anchors.md#a079), [A115](anchors.md#a115), [A116](anchors.md#a116), [A117](anchors.md#a117), [A118](anchors.md#a118), [A119](anchors.md#a119)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

## F29

observation · bounded

pdb_allocateMemory has no zero-size rejection. With initialized allocator and available memory, size zero can use an eligible block group or the fallback positive-size malloc allocation with metadata; successful return is block+0xc.

边界：Conditional static allocation path, not observed runtime success, fault determinism or a reclamation/root mechanism. Allocation counters do not establish roots.

锚点：[A120](anchors.md#a120), [A121](anchors.md#a121), [A122](anchors.md#a122), [A123](anchors.md#a123), [A124](anchors.md#a124), [A125](anchors.md#a125), [A126](anchors.md#a126), [A127](anchors.md#a127)

Evidence：`ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c`

## F30

unknown · unresolved

The zero-capture, initial-sp-zero result-overflow path can retain an unpublished C-local Function head after allocations; the supplied records do not establish a recovery root or eventual release for that head.

边界：Not a demonstrated leak and not a claim that nonempty legal captures hit this branch. PCB stack cleanup and allocator statistics do not independently recover an unpublished header.

锚点：[A004](anchors.md#a004), [A115](anchors.md#a115), [A119](anchors.md#a119), [A126](anchors.md#a126), [A128](anchors.md#a128)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`, `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c`, `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`

## F31

static-inference · bounded

For ordinary interpreter threadType1, valid backend recovery environment, no script catch, and no intervening hook/nonlocal failure, errorQuit traces/unwinds to the backend boundary and longjmps; doActions returns -3 and the scheduler reaches backend teardown and logical PCB deletion.

边界：A selected static path, not a proof for other thread types, arbitrary processor/JIT targets, callbacks or all indirect hooks. Final physical PCB cleanup remains conditional on ordinary PCB ref reaching zero.

锚点：[A129](anchors.md#a129), [A075](anchors.md#a075), [A074](anchors.md#a074), [A130](anchors.md#a130), [A131](anchors.md#a131), [A132](anchors.md#a132), [A133](anchors.md#a133), [A134](anchors.md#a134), [A135](anchors.md#a135), [A136](anchors.md#a136), [A137](anchors.md#a137), [A138](anchors.md#a138), [A139](anchors.md#a139), [A140](anchors.md#a140), [A141](anchors.md#a141), [A142](anchors.md#a142), [A143](anchors.md#a143)

Evidence：`ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2`, `ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1`, `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e`, `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`

## F32

observation · answered

The public scheduler wrapper sets scheduler-active state around its internal call. On the selected ordinary scheduler path, vm_endForBackend executes immediately after vm_doActions and before vm_deleteProcess. endForBackend drains destructing Objects, obsolete JIT code and obsolete Mapping nodes, then clears the backend timer.

边界：This ordering does not promise a second drain after later PCB deletion or final unref in the same scheduler iteration.

锚点：[A144](anchors.md#a144), [A145](anchors.md#a145), [A146](anchors.md#a146), [A147](anchors.md#a147), [A148](anchors.md#a148), [A139](anchors.md#a139), [A140](anchors.md#a140), [A143](anchors.md#a143), [A149](anchors.md#a149), [A150](anchors.md#a150), [A151](anchors.md#a151), [A152](anchors.md#a152)

Evidence：`ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c`, `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`, `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a`

## F33

observation · bounded

vm_deleteProcess marks logical deletion and detaches I/O/timer/pend resources; each held Object is detached before invoking its own release hook, then the loop reads the next held Object. Release hooks, crash-list end hooks and PCB endHook precede the PCB ordinary ref decrement. Each crash-list record is unlinked/freed before its endHook call.

边界：The recorded indirect calls do not resolve every hook target, completion, retention, exception or reentrancy behavior. A logical-delete call alone does not prove final stack/frame cleanup.

锚点：[A153](anchors.md#a153), [A154](anchors.md#a154), [A155](anchors.md#a155), [A156](anchors.md#a156), [A157](anchors.md#a157), [A158](anchors.md#a158), [A159](anchors.md#a159), [A160](anchors.md#a160), [A161](anchors.md#a161)

Evidence：`ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`

## F34

observation · answered

When deleteProcess final PCB ref becomes zero after prior hooks complete, cleanup releases deletionFunc, cleans stack from saved sp to stackSize, then removes remaining contexts. Each eligible nonnull Object context drops execRef; valid flag is cleared and nonzero-tid PCB enters the free list.

边界：Conditional routine order only. Eligibility is (flags&1)==0 or (flags&0x10)!=0. Reference owners outside this PCB and any newly queued Object drain timing remain separate.

锚点：[A160](anchors.md#a160), [A161](anchors.md#a161), [A162](anchors.md#a162), [A163](anchors.md#a163), [A164](anchors.md#a164), [A165](anchors.md#a165), [A166](anchors.md#a166), [A167](anchors.md#a167), [A168](anchors.md#a168), [A169](anchors.md#a169), [A170](anchors.md#a170)

Evidence：`ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`

## F35

observation · answered

deleteProcess with nonzero remaining PCB ref skips final deletionFunc/stack/frame cleanup. Later vm_unrefToPCB performs that cleanup only when its decrement reaches exactly zero, in the same deletionFunc-before-stack-before-context order.

边界：Does not prove that every external PCB holder eventually releases its reference or determine wall-clock reclamation time.

锚点：[A161](anchors.md#a161), [A171](anchors.md#a171), [A172](anchors.md#a172), [A173](anchors.md#a173), [A174](anchors.md#a174), [A128](anchors.md#a128), [A175](anchors.md#a175), [A176](anchors.md#a176), [A177](anchors.md#a177), [A178](anchors.md#a178)

Evidence：`ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`, `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`

## F36

unknown · unresolved

Objects queued by remaining-context execRef cleanup during deleteProcess or later vm_unrefToPCB are created after the selected endForBackend drain. The next guaranteed drain and final cleanup under all scheduler-stop paths are not established by these selected records.

边界：Known local queue behavior remains valid; no claim that the objects necessarily leak or that the same iteration performs another drain.

锚点：[A140](anchors.md#a140), [A143](anchors.md#a143), [A168](anchors.md#a168), [A176](anchors.md#a176), [A106](anchors.md#a106)

Evidence：`ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`, `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`, `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`, `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`

## F37

unknown · unresolved

Function final release cleans and frees capture storage without waiting for a positive external capture-slot ref. Arbitrary external capture-slot Reference survival or safe exposure is not established.

边界：Future Rust scope must keep capture slots private and prohibit public Reference escape; ordinary Array element and Mapping node Reference rules are separate.

锚点：[A092](anchors.md#a092), [A093](anchors.md#a093), [A096](anchors.md#a096)

Evidence：`ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4`

## F38

observation · bounded

The selected marker follows Function capture slots and the root refresher marks object-hash variables, globals, PCB stacks, type-2 timers, selected pend queues and registered mark callbacks. The global allocated-value list is reset before marking; membership is not itself a live root.

边界：Indirect mark callbacks are not exhausted by their registration list. Function header recovery, arbitrary Reference/Object graph edges, cyclic release order and complete GC equivalence remain unproved.

锚点：[A179](anchors.md#a179), [A180](anchors.md#a180)

Evidence：`ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471`, `ev_11d1d264620ab7f5452f093971acc5d836f0364c7d522a7234d1af509587c5fd`, `ev_6921332b80a7a8467156e8214b2cf335f37f0c97324f7e7737d1d6c56b44b54d`, `ev_8fecedeb1f644246e085db4dbc5c875cfb14fd6368045b63417b2183eab987ab`

## F39

unknown · unresolved

Other native thread types, JIT recovery, callback-catch, arbitrary deletion hooks/reentrancy, full external roots and complete cyclic reclamation are outside the closed ordinary-thread subdomain.

边界：The eight new dossiers and untruncated local value-flow do not resolve targetless indirect calls or establish whole-lifecycle equivalence.

锚点：[A129](anchors.md#a129), [A075](anchors.md#a075), [A155](anchors.md#a155), [A158](anchors.md#a158), [A159](anchors.md#a159), [A139](anchors.md#a139)

Evidence：`ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2`, `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`, `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`

## F40

design · bounded

2AX is evidence closure and T05 gated specification only: keep runtime, public API, dependencies and existing capture permissions unchanged. Direct Object/Function capture readiness stays false until lifecycle/root/reclamation gates have evidence and an accepted bounded implementation contract.

边界：No new state model or runtime implementation here. Existing stable captures, semantic owners, unique move/shared COPY, final-graph checks, success-only publication and Host single-effect rules remain Rust acceptance policies.

锚点：[A115](anchors.md#a115), [A093](anchors.md#a093), [A161](anchors.md#a161), [A149](anchors.md#a149)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`, `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4`, `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`, `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a`

## F41

observation · bounded

Successful vm_initProcess sets sp=stackSize, funCp=0, PCB ref=1 and catchCp=0, initializes entry context with bp=stackSize/ip=0/flags=2, and clears its thisObject. Explicit nonnull attrib stack sizes are clamped to 256..65536.

边界：The null-attrib branch copies defaults without the same local clamp. The selected witness uses explicit stackSize=256, valid entry/global context and successful initialization; no universal configuration validity or runtime execution is claimed.

锚点：[A181](anchors.md#a181), [A182](anchors.md#a182), [A183](anchors.md#a183), [A184](anchors.md#a184), [A185](anchors.md#a185), [A186](anchors.md#a186), [A187](anchors.md#a187), [A188](anchors.md#a188), [A189](anchors.md#a189), [A190](anchors.md#a190), [A191](anchors.md#a191), [A192](anchors.md#a192), [A193](anchors.md#a193)

Evidence：`ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef`

## F42

observation · bounded

Ordinary no-nextActionHook dispatch indexes actionInfoTable and calls its entry without a generic sp-capacity check. Its two-parameter machine path pushes sign-extended par8 then par16; the one-parameter path pushes par16. Function-value wrapper extracts argn from low six bits, allowing Global zero-capture par8=0x80.

边界：Frozen actions.csv rows 17 and 3b were checked against their ELF table bytes in e01-paths.md. This bounded native dispatch evidence is not exhausted by direct callers; decompiler pseudocode omits the second argument and is not the authoritative calling sequence.

锚点：[A194](anchors.md#a194), [A195](anchors.md#a195), [A196](anchors.md#a196), [A197](anchors.md#a197), [A198](anchors.md#a198), [A199](anchors.md#a199), [A200](anchors.md#a200), [A201](anchors.md#a201), [A202](anchors.md#a202), [A203](anchors.md#a203), [A204](anchors.md#a204), [A205](anchors.md#a205), [A206](anchors.md#a206), [A207](anchors.md#a207), [A208](anchors.md#a208), [A209](anchors.md#a209)

Evidence：`ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`, `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`, `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a`

## F43

static-inference · bounded

A finite conditional entry path initializes 256 stack slots, executes 256 legal 3b(0) pushes, then 17(g,0x80) for an existing encodable type-matching Global target. Pushes take sp from 256 to zero within valid slots; zero captures leave sp zero and reach the final create-result overflow branch if allocations and target lookup succeed.

边界：Ordinary processor, valid entry/global environment, zero ctrlFlags, no hooks/JIT or intervening state changes are explicit premises. The 257 actions fit below its initial 0x800 timer-check interval. This is a static path witness, not main-corpus/compiler reachability, observed execution or a new model.

锚点：[A186](anchors.md#a186), [A191](anchors.md#a191), [A210](anchors.md#a210), [A211](anchors.md#a211), [A212](anchors.md#a212), [A213](anchors.md#a213), [A214](anchors.md#a214), [A215](anchors.md#a215), [A216](anchors.md#a216), [A217](anchors.md#a217), [A218](anchors.md#a218), [A206](anchors.md#a206), [A208](anchors.md#a208), [A114](anchors.md#a114), [A115](anchors.md#a115), [A119](anchors.md#a119)

Evidence：`ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef`, `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a`, `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`, `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`, `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

## F44

observation · bounded

In the selected zero-capture creation path, header H and zero-length capture allocation A exist before the result error. H points to A and possibly holds its own valid creator ref; no capture vm_initValue iteration runs. Result Value publication occurs after the error edge, so H has not been installed in the result stack slot at that contact.

边界：Tracks H, A and an optional creator ordinary ref separately. The finite Global entry witness permits null creator. Local publication stores and absent capture initialization do not exhaust every external recovery mechanism.

锚点：[A219](anchors.md#a219), [A004](anchors.md#a004), [A220](anchors.md#a220), [A087](anchors.md#a087), [A114](anchors.md#a114), [A115](anchors.md#a115), [A221](anchors.md#a221), [A222](anchors.md#a222), [A007](anchors.md#a007), [A088](anchors.md#a088), [A119](anchors.md#a119)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`, `ev_6921332b80a7a8467156e8214b2cf335f37f0c97324f7e7737d1d6c56b44b54d`

## F45

observation · bounded

pdb_extendBlocksOfGroup records a whole extension allocation in _pdb_extendGrpList with block size/count, initializes per-block manager headers and chains free blocks. allocate consumes free blocks or uses fallback allocation and updates counters/allocated bits.

边界：This is concrete allocator storage tracking, not just counters, but no Function/creator semantic recovery relation is established. Whole-page reachability or future bulk storage free is not VM payload destruction.

锚点：[A223](anchors.md#a223), [A224](anchors.md#a224), [A225](anchors.md#a225), [A226](anchors.md#a226), [A227](anchors.md#a227), [A228](anchors.md#a228), [A229](anchors.md#a229), [A230](anchors.md#a230), [A231](anchors.md#a231), [A232](anchors.md#a232), [A126](anchors.md#a126), [A127](anchors.md#a127)

Evidence：`ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d`, `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c`

## F46

observation · bounded

Given an explicit valid payload pointer, pdb_freeMemory derives its manager header, validates allocation/tail metadata, clears the allocated bit and updates statistics, then returns storage to a group or calls pdb_l2FreeMemory. l2Free returns grouped storage or calls libc free; these normal paths do not dispatch VM Value cleanup or creator unref.

边界：Selected normal allocator paths only, excluding fatal assertion behavior. Physical storage reclamation must remain separate from H/A semantic ownership and creator-ref release.

锚点：[A233](anchors.md#a233), [A234](anchors.md#a234), [A235](anchors.md#a235), [A236](anchors.md#a236), [A237](anchors.md#a237), [A238](anchors.md#a238), [A239](anchors.md#a239), [A240](anchors.md#a240), [A241](anchors.md#a241), [A242](anchors.md#a242), [A243](anchors.md#a243), [A244](anchors.md#a244)

Evidence：`ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab`, `ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a`

## F47

unknown · unresolved

Despite the finite legal-stack branch witness and newly identified allocator page tracking, recovery of unpublished H/A and eventual H/A storage plus any creator ordinary-ref release after the result error remain unclosed.

边界：The selected create/allocator/error/PCB paths do not establish the missing semantic recovery edge. Neither a global leak nor GC/allocator-shutdown recovery is claimed; actual corpus reachability is separate and not required for this bounded branch finding.

锚点：[A115](anchors.md#a115), [A119](anchors.md#a119), [A227](anchors.md#a227), [A233](anchors.md#a233), [A128](anchors.md#a128), [A176](anchors.md#a176)

Evidence：`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`, `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d`, `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab`, `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`

## F48

static-inference · bounded

An Object queued after one PCB final cleanup can be drained at a later ordinary backend only if a subsequent PCB actually enters and normally completes vm_doActions, followed by vm_endForBackend. The drain is after that backend action execution, not before the next PCB starts.

边界：An active PCB pointer alone is not a progress guarantee: resume hooks, quota, stop state, callbacks or nonlocal exits can prevent the selected backend completion.

锚点：[A139](anchors.md#a139), [A140](anchors.md#a140), [A143](anchors.md#a143), [A245](anchors.md#a245), [A246](anchors.md#a246), [A149](anchors.md#a149)

Evidence：`ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`, `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a`

## F49

observation · bounded

vm_processAllRespondingInQueue returns through a direct visible tail when its queue peek returns 0. On its nonzero-peek branch it may call a node callback, then a destruction callback after acquiring the VM mutex.

边界：The peek-return-zero visible tail has no extra drain, but the peek helper and arbitrary callbacks are not thereby proved effect-free. Nonempty response/callback progress and reentrancy remain outside the closed domain.

锚点：[A247](anchors.md#a247), [A248](anchors.md#a248), [A249](anchors.md#a249), [A250](anchors.md#a250), [A251](anchors.md#a251), [A252](anchors.md#a252), [A253](anchors.md#a253)

Evidence：`ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72`

## F50

observation · bounded

The single schedule wrapper only resets scheduler state after its internal call and returns. mainLoop has a nonnull-pPara return-zero tail and requestStop/internal-minus99 stop-signal return-one tails without an additional explicit Object drain in those visible tails.

边界：Conditions include the actual preceding scan/helper results, not an incorrectly reused scheduler return variable. Tail observations do not exhaust intervening scanner, response or hook effects and do not imply that every caller later invokes shutdown.

锚点：[A146](anchors.md#a146), [A147](anchors.md#a147), [A148](anchors.md#a148), [A254](anchors.md#a254), [A255](anchors.md#a255), [A256](anchors.md#a256), [A257](anchors.md#a257), [A258](anchors.md#a258), [A259](anchors.md#a259), [A260](anchors.md#a260)

Evidence：`ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c`, `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b`

## F51

observation · bounded

On normal completion of preceding timer/invoke/pend helpers and deletion hooks, clearEnvironment scans eligible PCBs with deleteProcess and then explicitly drains destructing Objects before destructIdentifiers and destructAllObjects(0). Newly queued Objects from in-scan final PCB cleanup can reach this drain.

边界：Conditional clearEnvironment contact only: PCB refs retained past its scan defer final values/frames, and later helper/object destruction effects or later external unrefs are not covered by this earlier drain.

锚点：[A261](anchors.md#a261), [A262](anchors.md#a262), [A263](anchors.md#a263), [A264](anchors.md#a264), [A265](anchors.md#a265), [A161](anchors.md#a161), [A168](anchors.md#a168), [A108](anchors.md#a108), [A109](anchors.md#a109)

Evidence：`ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29`, `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`, `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63`

## F52

observation · bounded

Once shutdownScheduler reaches its cleanup phase after stop/wait, scheduling and helper paths complete, it scans eligible PCBs through deleteProcess and explicitly drains destructing Objects after the scan and before freeing PCB arrays/storage.

边界：Other-task wait for alreadyStop is not proved to progress; mainLoop stop signaling does not automatically invoke shutdown. This drain covers the then-queued objects, not arbitrary retained external holders or later cleanup under all interleavings.

锚点：[A266](anchors.md#a266), [A267](anchors.md#a267), [A268](anchors.md#a268), [A269](anchors.md#a269), [A270](anchors.md#a270), [A271](anchors.md#a271), [A161](anchors.md#a161), [A168](anchors.md#a168)

Evidence：`ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5`, `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`

## F53

observation · bounded

The shutdownScheduler stack-storage pass writes each slot type byte to zero before calling vm_destructStaticValue. cleanupValue dispatches on that now-zero tag, so this pass cannot be treated as another destructor pass over the original payload tags after the explicit Object drain.

边界：Storage unlink/free and original payload semantic cleanup are distinct. This observation supplies no safe-lifetime guarantee for arbitrary external references retained across scheduler shutdown.

锚点：[A272](anchors.md#a272), [A271](anchors.md#a271), [A273](anchors.md#a273)

Evidence：`ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5`, `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`, `ev_b7275ea0d205e76bd66e12eb46ee8132bb75cd0db3113213fdd60970242bb1c2`

## F54

unknown · unresolved

E02 remains partial: some normal backend/clear/shutdown drain contacts are conditional, while next-drain guarantees for all stop paths, external retained-PCB progress, arbitrary callbacks/hooks/other threads/JIT, and complete external-root lifetime or GC are not established.

边界：No unconditional eventual reclaim or global-leak assertion. A later unref is considered only while its PCB storage is still valid; no permission to dereference PCB pointers after scheduler storage shutdown is inferred.

锚点：[A246](anchors.md#a246), [A140](anchors.md#a140), [A258](anchors.md#a258), [A259](anchors.md#a259), [A251](anchors.md#a251), [A263](anchors.md#a263), [A265](anchors.md#a265), [A269](anchors.md#a269), [A272](anchors.md#a272), [A172](anchors.md#a172), [A176](anchors.md#a176)

Evidence：`ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`, `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b`, `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72`, `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29`, `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5`, `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`
