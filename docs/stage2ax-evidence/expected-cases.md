# 2AX 静态／设计案例

所有案例 runtime_executed=false；不是原程序运行 oracle。C01—C51 保留 2AR 原文，后续案例单列本轮重评，关联见案例映射。

## C01

capture-create · static-inference

输入：17 receives already-evaluated raw stack Undefined; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C02

capture-create · static-inference

输入：17 receives already-evaluated raw stack Void; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C03

capture-create · static-inference

输入：17 receives already-evaluated raw stack Int; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C04

capture-create · static-inference

输入：17 receives already-evaluated raw stack Real bits; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C05

capture-create · static-inference

输入：17 receives already-evaluated raw stack Bytes; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C06

capture-create · static-inference

输入：17 receives already-evaluated raw stack Array; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C07

capture-create · static-inference

输入：17 receives already-evaluated raw stack Mapping; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C08

capture-create · static-inference

输入：17 receives already-evaluated raw stack live Object; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C09

capture-create · static-inference

输入：17 receives already-evaluated raw stack Function with live or null creator; creator is valid; resources available.

期望：Move type/attrib/payload into initialized capture slot, clear source type/attrib; retain raw payload identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F03](findings.md#f03)

## C10

capture-create · static-inference

输入：17 receives raw deleted Object payload already on stack.

期望：Transfer itself does not invoke copy-based source normalization; no claim about how preceding expression produced raw value.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F03](findings.md#f03), [F08](findings.md#f08)

## C11

capture-create · static-inference

输入：17 receives raw Function whose creator is deleted, creator of outer Function valid.

期望：Raw captured Function payload is transferred at this point; outer creator validation is a separate identity.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F03](findings.md#f03), [F23](findings.md#f23)

## C12

capture-create · static-inference

输入：17 capture operand is tag8 Reference.

期望：Native helper raises argument error during reference scan before capture allocations/transfers.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F02](findings.md#f02)

## C13

capture-create · design

输入：17 capture operand is hidden Loop tag0x14.

期望：No legal capture guarantee follows from this helper; Rust continues explicit rejection.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F01](findings.md#f01), [F21](findings.md#f21)

## C14

capture-create · static-inference

输入：17 header allocation succeeds; capture storage allocation fails.

期望：Header freed then vm_error, before capture transfer; final uncaught cleanup outside this local ordering remains separate.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F02](findings.md#f02), [F20](findings.md#f20)

## C15

capture-create · unresolved

输入：17 has moved captures but final result stack has no space.

期望：Creation is not proven failure-atomic; C-local unpublished head ownership/rooting and final cleanup remain unresolved.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F02](findings.md#f02), [F20](findings.md#f20)

## C16

head-expansion · static-inference

输入：Native head ref is 1; raw bound Array; enough stack.

期望：Reverse-index raw move, clear bound source slots, free unique Function head. Container children are not recursively normalized.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F07](findings.md#f07)

## C17

head-expansion · static-inference

输入：Native head ref is 2; raw bound Array; enough stack.

期望：Release local call-target holder, push/copy bound header; surviving Function capture and argument share container. Container children are not recursively normalized.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F06](findings.md#f06), [F07](findings.md#f07)

## C18

head-expansion · static-inference

输入：Native head ref is 1; raw bound Mapping; enough stack.

期望：Reverse-index raw move, clear bound source slots, free unique Function head. Container children are not recursively normalized.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F07](findings.md#f07)

## C19

head-expansion · static-inference

输入：Native head ref is 2; raw bound Mapping; enough stack.

期望：Release local call-target holder, push/copy bound header; surviving Function capture and argument share container. Container children are not recursively normalized.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F06](findings.md#f06), [F07](findings.md#f07)

## C20

head-expansion · static-inference

输入：Native head ref==1, bound Object deleted after capture; inspect immediately after expansion, before callee 0f.

期望：Raw payload moves to argument stack without copy-based normalization; Function head/capture storage released; callee checks may subsequently reject.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F09](findings.md#f09), [F18](findings.md#f18)

## C21

head-expansion · static-inference

输入：Native head ref>1, bound Object deleted after capture; inspect expansion and surviving head.

期望：copy cleans bound SOURCE to Undefined/attrib0 and pushes Undefined. Surviving head retains normalized capture for later calls.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F06](findings.md#f06), [F08](findings.md#f08), [F09](findings.md#f09)

## C22

head-expansion · static-inference

输入：Native head ref==1, bound Function whose nonnull creator was deleted after capture; inspect immediately after expansion, before callee 0f.

期望：Raw payload moves to argument stack without copy-based normalization; Function head/capture storage released; callee checks may subsequently reject.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F09](findings.md#f09), [F18](findings.md#f18)

## C23

head-expansion · static-inference

输入：Native head ref>1, bound Function whose nonnull creator was deleted after capture; inspect expansion and surviving head.

期望：copy cleans bound SOURCE to Undefined/attrib0 and pushes Undefined. Surviving head retains normalized capture for later calls.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F06](findings.md#f06), [F08](findings.md#f08), [F09](findings.md#f09)

## C24

head-expansion · static-inference

输入：Shared head captures Function whose creator is null.

期望：copy does not classify it dead merely because creator is null; nested Function head identity is shared.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F08](findings.md#f08), [F06](findings.md#f06)

## C25

head-expansion · static-inference

输入：Shared outer head captures another live Function; later drop last outer holder.

期望：Normal copy shares nested head; final outer cleanup releases its bound payload before creator/pArgs/head.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F06](findings.md#f06), [F11](findings.md#f11)

## C26

head-expansion · design

输入：VM diagnostic snapshot or host Rc clone changes physical strong_count without a bytecode store.

期望：Do not select native unique/shared path using that physical count; semantic ownership model is a prerequisite.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F21](findings.md#f21)

## C27

head-expansion · static-inference

输入：Bound [b0,b1], new arguments [n0,n1], successful 05.

期望：Expansion visits bound indices in reverse to preserve logical bound-before-new order; no scalar conversion in raw transfer.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F06](findings.md#f06)

## C28

copy-sites · static-inference

输入：Array header contains raw deleted Object member; copy header only.

期望：Header is shared; member raw payload remains until an operation actually copies that member.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F07](findings.md#f07), [F08](findings.md#f08)

## C29

copy-sites · static-inference

输入：Shared global slot contains deleted Object or Function with deleted nonnull creator.

期望：vm_pushGlobalVar copies actual slot: source and result become Undefined at that copy point.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F08](findings.md#f08), [F22](findings.md#f22)

## C30

copy-sites · static-inference

输入：Object variable contains dead handle; ordinary Object variable read.

期望：Source-copy normalization acts on the actual relocated object slot; not a blanket return normalization.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F08](findings.md#f08), [F22](findings.md#f22)

## C31

copy-sites · static-inference

输入：Two References alias slot containing dead handle; convert one Reference to ordinary value.

期望：copy normalizes underlying source to Undefined, then consumed Reference unrefs slot; other alias can observe Undefined.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F08](findings.md#f08), [F12](findings.md#f12)

## C32

copy-sites · static-inference

输入：Query existing Array member containing deleted Object, container still shared.

期望：Copy selected member and normalize its source before consuming query container header; preserve query-specific result attribute policy.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F08](findings.md#f08), [F13](findings.md#f13)

## C33

copy-sites · static-inference

输入：Query existing Mapping member containing deleted Object, container still shared.

期望：Copy selected member and normalize its source before consuming query container header; preserve query-specific result attribute policy.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F08](findings.md#f08), [F13](findings.md#f13)

## C34

copy-sites · static-inference

输入：Mapping lookup misses a key.

期望：Query returns Undefined and consumes lookup values without inserting a missing node.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F13](findings.md#f13)

## C35

release · static-inference

输入：Last Array header released while an external element Ref remains.

期望：Header ownership unrefs slot; surviving slot reference prevents ref==0 cleanup and may retain payload.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F14](findings.md#f14)

## C36

release · static-inference

输入：Last Mapping header released while an external value Ref remains.

期望：freeNode clears key/value payload immediately; physical node reclamation can be deferred, so surviving Ref sees cleared value.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F15](findings.md#f15)

## C37

release · static-inference

输入：Last Function head dropped with nonempty captures and nonnull creator.

期望：Clean capture payloads/unlink capture slots, unref creator, free pArgs then head; external capture-slot lifetime not inferred.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F11](findings.md#f11)

## C38

release · static-inference

输入：Unique head raw-moves captures then frees head.

期望：Moved capture slots are Undefined, so head cleanup does not double-release transferred payload; argument stack later owns release.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F11](findings.md#f11)

## C39

return · static-inference

输入：Raw dead handle is already the result of ordinary nonconst mixed return; no conversion/hook required.

期望：takeOffValueFromStack cleans removed frame slots and raw-moves result; transfer itself does not invoke copy normalization.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F16](findings.md#f16)

## C40

return · unresolved

输入：Const return, conversion or return hook path.

期望：Simple return-move rule is insufficient; recover that specific branch before implementation.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F16](findings.md#f16), [F20](findings.md#f20)

## C41

failure · static-inference

输入：Shared capture copy normalized source, then callee later argument fails under interpreter threadType1 with valid script catch.

期望：Exiting argument/temporary stack is cleaned and saved catch sp restored; still-live head capture remains Undefined, not rolled back.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F09](findings.md#f09), [F18](findings.md#f18), [F19](findings.md#f19)

## C42

failure · static-inference

输入：Unique capture expansion completed; callee parameter count fails under same valid script catch conditions.

期望：Head is not recreated. Moved payloads are released from callee/unwound stack through cleanup; consumed target is not restored.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F05](findings.md#f05), [F18](findings.md#f18), [F19](findings.md#f19)

## C43

failure · static-inference

输入：Captures expanded but call depth or missing code body fails before installing callee frame; valid ordinary script catch.

期望：Catch saved-sp cleanup removes expanded calculation values even without a new callee frame; no target restoration.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F18](findings.md#f18), [F19](findings.md#f19)

## C44

failure · static-inference

输入：Earlier parameter Int converts to Real, later parameter type fails.

期望：Native 0f can mutate earlier argument before failure; current Rust all-argument preflight is separate policy.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F18](findings.md#f18), [F19](findings.md#f19)

## C45

failure · static-inference

输入：Native vm_pushValue has no free destination stack position.

期望：Stack check raises before vm_copyValue, so this push does not perform source cleanup first.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F10](findings.md#f10)

## C46

failure · unresolved

输入：No script catch, non-interpreter thread, JIT, callback-catch or reentrant deletion cleanup.

期望：Do not extend ordinary valid-script-catch cleanup proof; precise final lifecycle remains unresolved.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F19](findings.md#f19), [F20](findings.md#f20), [F23](findings.md#f23)

## C47

graph · design

输入：Function captures a shared acyclic Array/Mapping; child graph contains shared ordinary subcontainers.

期望：Native copy remains shallow and marker follows captures/children. Future Rust implementation must preserve sharing and bounded graph validation.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F07](findings.md#f07), [F17](findings.md#f17), [F21](findings.md#f21)

## C48

graph · unresolved

输入：Function/container cycle or arbitrary Reference graph; force garbage collection.

期望：Marker observations alone do not prove complete root coverage, release order or recoverability; cycles remain excluded from first implementation.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F17](findings.md#f17), [F20](findings.md#f20)

## C49

policy · design

输入：Foreign-origin Function passed as capture or call target to another Rust ObjectStore.

期望：Preserve explicit Rust store isolation; no native origin-token equivalent is established.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F21](findings.md#f21)

## C50

policy · design

输入：Zero Rust instruction budget before 05/17.

期望：Future extension keeps current inert behavior: no source edits, allocations, semantic owner change or callback. This is a Rust acceptance requirement.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F21](findings.md#f21)

## C51

policy · design

输入：One-step Rust budget around 17 publication and 05 callee entry or synchronous Host completion.

期望：Future extension preserves current single-instruction boundaries and precommit failure policy; never exposes a partial capture transfer.

边界：Only the named transfer/cleanup point; not whole-program equivalence.

依据：[F21](findings.md#f21)

## C52

creator-ownership · static-inference

输入：Creator and a raw captured Object denote the same valid Object.

期望：Header creator gains its own ordinary ref; payload is moved, not merged with creator or execRef.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F24](findings.md#f24)

## C53

delayed-delete · static-inference

输入：Destruction requested while execRef is positive.

期望：Set 0x4000 and return; do not treat this request alone as 0x8000 invalidation.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F26](findings.md#f26), [F27](findings.md#f27)

## C54

delayed-delete · static-inference

输入：Last execRef exits while 0x4000 is set.

期望：Isolate, queue, set 0x8000 and add independent queue ordinary ref.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F26](findings.md#f26)

## C55

delayed-delete · static-inference

输入：A queued Object reaches the observed drain routine.

期望：Unlink queue entry before low-level deletion and queue-ref decrement; retained invalid shell has independent final-unref lifetime.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F26](findings.md#f26)

## C56

source-copy · static-inference

输入：COPY an Object or creator-bearing Function while creator is pending 0x4000 but not invalid 0x8000.

期望：Pending bit alone does not cause source normalization; invalid 0x8000 selects source cleanup/reread.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F27](findings.md#f27)

## C57

function-release · static-inference

输入：Last Function head ref reaches nonpositive with nonempty captures and creator.

期望：Clean and unlink capture slots, unref creator, free capture storage then head.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F25](findings.md#f25)

## C58

capture-slot-reference · unresolved

输入：External Reference allegedly outlives the final owning Function head.

期望：No safe capture-slot lifetime guarantee; keep public capture-slot Reference escape excluded.

边界：Do not import Array element-reference guarantees into Function capture storage.

依据：[F25](findings.md#f25), [F37](findings.md#f37)

## C59

result-stack-reachability · static-inference

输入：Legal stack, nonnegative nonzero captures and no intervening state mutation.

期望：Each capture move increases sp; final sp==0 result-overflow premise of historical C15 is infeasible in this bounded domain.

边界：Static contact only; no target execution or Rust direct-capture implementation.

依据：[F28](findings.md#f28)

## C60

zero-allocation · static-inference

输入：Initialized pdb allocator receives size zero and resources are available.

期望：Eligible pool or positive-size fallback allocation can return block+0xc; zero size is not intrinsically rejected.

边界：Conditional static success, not runtime allocator observation or a recovery root.

依据：[F29](findings.md#f29)

## C61

unpublished-head · unresolved

输入：Zero captures, initial sp==0, both allocations succeed and final result push errors.

期望：Unpublished C-local head recovery root and eventual release remain unproved.

边界：No demonstrated leak; nonempty legal captures and allocator failure are different branches.

依据：[F28](findings.md#f28), [F29](findings.md#f29), [F30](findings.md#f30)

## C62

ordinary-uncaught · static-inference

输入：Ordinary interpreter threadType1, valid backend environment, no script catch, no interfering hook or nonlocal failure.

期望：traceError unwinds to backend boundary; errorQuit longjmps and doActions returns -3 to the scheduler.

边界：Selected ordinary path only; not whole-exit coverage.

依据：[F31](findings.md#f31)

## C63

backend-delete-order · static-inference

输入：Selected ordinary scheduler path receives uncaught-error return.

期望：endForBackend drains its queues before scheduler saves PCB sp/funCp and calls deleteProcess.

边界：No same-iteration second drain guarantee.

依据：[F31](findings.md#f31), [F32](findings.md#f32)

## C64

delete-hooks · static-inference

输入：deleteProcess has held-object release hooks, crash-list hooks or PCB endHook.

期望：Hook contacts occur before PCB ref decrement; crash records are detached/freed before callback.

边界：Target bodies, reentry, retention and exceptions remain unresolved.

依据：[F33](findings.md#f33)

## C65

pcb-final-reference · static-inference

输入：deleteProcess hooks complete and decrement changes PCB ref from one to zero.

期望：Release deletionFunc, clean saved stack range, drop eligible remaining frame execRefs and retire PCB.

边界：Does not drain newly queued objects by itself or release external owners.

依据：[F34](findings.md#f34)

## C66

pcb-retained-reference · static-inference

输入：deleteProcess leaves positive PCB ref, then a later final vm_unrefToPCB reaches exactly zero.

期望：First call skips final values/frames; final unref performs deletionFunc, stack and context cleanup.

边界：No guarantee that every external holder eventually releases.

依据：[F35](findings.md#f35)

## C67

post-delete-drain · unresolved

输入：Final remaining-frame execRef release during PCB deletion queues another Object.

期望：Queue state is known, but next guaranteed drain across scheduler-stop paths remains a gate.

边界：Backend drain precedes this deletion contact; do not assert immediate final release.

依据：[F26](findings.md#f26), [F32](findings.md#f32), [F36](findings.md#f36)

## C68

root-coverage · unresolved

输入：A zero-capture unpublished head, arbitrary Reference graph or external mark callback is proposed as GC-recoverable.

期望：Recorded allocation chain/known roots do not prove complete recovery or cyclic release order.

边界：Retain full GC and external-root unknowns.

依据：[F30](findings.md#f30), [F38](findings.md#f38), [F39](findings.md#f39)

## C69

excluded-exits · unresolved

输入：Failure occurs in another thread type, JIT, callback-catch or reentrant deletion hook.

期望：No extension of the closed ordinary-thread path is justified.

边界：Require independent per-exit evidence before implementation.

依据：[F39](findings.md#f39)

## C70

direct-capture-gate · design

输入：2AX evidence ledger and gated T05 specification are delivered.

期望：Keep direct Object/Function captures, generic non-scalar readiness and stage2 completion false; runtime/API/dependencies unchanged.

边界：Evidence/specification completion is separate from permission to open runtime capabilities.

依据：[F40](findings.md#f40)

## C71

graph-policy · design

输入：Future direct capture proposes a nested Function or Object ownership graph.

期望：Reuse semantic unique/shared COPY and final-graph acceptance rules; specify Object variable/creator edges independently and keep cycles/capture-slot Reference escape closed.

边界：A bounded Rust graph policy is not native GC or delayed-delete equivalence.

依据：[F24](findings.md#f24), [F25](findings.md#f25), [F37](findings.md#f37), [F38](findings.md#f38), [F40](findings.md#f40)
