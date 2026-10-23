# 2AR结论账本

观察/静态推导/设计/未知分开；没有原版运行观测。完整记录见函数清单及压缩证据。

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
