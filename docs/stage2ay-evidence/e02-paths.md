# 2AY E02：PCB 最终清理后的对象队列路径

本页记录原生静态控制流及其条件，不是参考程序的运行轨迹。对象入队、队列排空、变量清理与最后普通引用退出分别判断；PCB 的逻辑删除不保证本轮已经释放全部载荷。直接 Object/Function 捕获仍关闭。

## 已知起点及条件化后继

普通解释线程在选定 backend 路径执行 `vm_doActions` 后，scheduler 先调用 `vm_endForBackend`，再保存 PCB 状态、摘除活动 PCB 并调用 `vm_deleteProcess`。后者在 hooks 正常完成后减少 PCB ref；只有归零才清理 deletionFunc、剩余值栈及符合 flags 条件的帧 execRef，ref 尚为正时延后至后续 `vm_unrefToPCB` 的归零分支。这些接点在 2AX 已闭合，不能重新表述为删除之后先前的 backend drain 会自动覆盖新入队对象。

原始记录与关键指令：

- `vm_scheduleProcessInternal`：`ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`；`0x080ad683` 调用 `vm_doActions`，`0x080ad68a` 调用 `vm_endForBackend`，`0x080ad519` 调用 `vm_deleteProcess`。
- `vm_deleteProcess`：`ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`；持有对象逐个摘链后调用各自 releaseHook，crash/end hooks 也先于最终 PCB ref 递减。间接 hooks 的完成、保留和重入影响仍需独立限定。
- `vm_unrefToPCB`：`ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`；`0x080ab6b6` 清理值槽，`0x080ab71f` 退出剩余对象执行引用。它们可能在先前 backend drain 之后产生待删对象。
- `vm_endForBackend`：`ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a`；`0x08096f4a` 调用 `vm_freeObjectsInDestructingList`，之后才清过期 JIT/Mapping 节点和 backend timer。

若后来另一个普通 PCB **实际进入并正常完成** `vm_doActions`，其后的 `vm_endForBackend` 提供下一排空接点。这个条件不保证另一个 PCB 一定获得执行、不保证其动作完成，也不是“下一 PCB 开始执行前已经清空”。resumeHook 非零返回、调度配额、停止状态及非局部退出必须另行判断。

## 退出域矩阵

| 域 | 有依据的静态控制流 | 不能据此推出的结论 |
|---|---|---|
| PCB ref 在 deleteProcess 中归零 | hooks 后清 deletionFunc、栈及剩余帧；其后 scheduler 回到遍历位置 | 先前 endForBackend 已覆盖这些清理新产生的待删对象 |
| PCB ref 仍为正 | 本次跳过最终栈/帧释放；后续 unrefToPCB 归零才做最终清理 | 每个外部持有者必然及时或最终释放 ref |
| 尚有活动 PCB | scheduler 可进入后续迭代；实际进入且完成下一 backend 时才到下一 drain | 仅凭 active 指针非空就证明必达该 drain |
| 最后活动 PCB 退出 | 摘链后 active 可为空，随后走返回码分支；无 timer/invoke 时可返回 0 | 最后 deleteProcess 后本函数存在无条件第二次 drain |
| 单次 schedule wrapper | 调一次 internal，清调度状态后直接返回 | wrapper 返回前还会额外调用 endForBackend |
| mainLoop 的非空 pPara 返回路径 | idle/扫描条件成立且 pPara 非空时直接返回 0 | mainLoop 返回必先执行额外对象排空 |
| mainLoop 停止路径 | requestStop 或 internal 返回 -99 后设置 alreadyStop、发结束信号并返回 1 | 信号发送本身执行对象/PCB 的最终清理，或外部调用者一定继续 shutdown |
| clearEnvironment 的扫描后接点 | 正常完成 timer/invoke/pend 处理及 eligible PCB 删除扫描后，明确调用一次对象队列 drain | 该调用是整个 clearEnvironment 最后的载荷清理，或外部 ref 延后退出已被覆盖 |
| shutdownScheduler 的扫描后接点 | 停止同步及资源处理正常完成、eligible PCB 删除扫描结束后，明确调用一次对象队列 drain | 主循环停止必然调用本函数，或其后所有静态槽/存储注销已包含在先前 drain 中 |
| responding queue 接点 | 一次 peek 返回 0 后直接返回；非空时可调用节点回调和 destruction 回调 | 任意 responding 回调都正常返回、及时释放 PCB ref，或所有回调都执行对象排空 |

上述 scheduler 的返回分支仍包含前置 helper、timer/invoke 和 resumeHook 接点。对“空调度”作排空与否判断时，必须限定这些接点的效果，不能用 caller 本体没有某个 CALL 替代完整路径证明。以下新增记录明确了 responding helper 与两类批量清理接点，但没有穷尽间接回调目标。

`vm_scheduleProcess` 的完整记录为 `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c`：`0x080ad9a2` 调用 internal，随后只有调度状态复位及 `0x080ad9bc` 返回。

`vm_mainLoop` 的完整复用记录为 `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b`：`0x080adf21` 调用 internal，`0x080adf26` 检查 -99；`0x080ae00d` 在非空 pPara 时跳到 `0x080ae0be` 返回 0；`0x080ae04b` 检查 requestStop，停止尾部始于 `0x080ae058`。后段 idle 条件中的局部变量已由 socket/telnet 扫描结果覆盖，不能误读为仍在判断 scheduler 返回值。

## 本轮新增证据与排空先后

### responding queue：空返回与间接回调分开

`vm_processAllRespondingInQueue` 的完整新增记录为 `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72`。`0x080bea74` 调用 queue peek；若它返回 0，`0x080bea7e` 跳到 `0x080beac0` 的返回尾部，没有随后显式对象 drain。非空节点可以经 `0x080beace` 的 `CALL EAX` 执行节点回调；destruction 回调在取得 VM mutex 后经 `0x080bea9e` 的 `CALL ESI` 执行。任意这些间接目标的效果与进展性并没有因 helper dossier 完整而闭合。

因此可记录“空 peek 返回后的可见尾部没有 drain”，不能把它升级为任意响应队列状态下整个 scheduler 都不会/一定会回收的结论。普通空队列路径与 callback 路径在案例和问题子域中分别重评。

### clearEnvironment：PCB 扫描之后有明确 drain

`vm_clearEnvironment` 的完整新增记录为 `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29`。它先处理 timer/invoke 和 pend 对象，再扫描 PCB；选中条件是 valid 位存在且未置已完成位 `0x20`。`0x080acb64` 调用 `vm_deleteProcess`，扫描结束后的 `0x080acb7e` 明确调用 `vm_freeObjectsInDestructingList`。

限定推导：若选中 PCB 的 hooks 正常返回且其 ref 在 deleteProcess 中归零，最终栈/帧清理产生并仍在队列中的对象，会在该次扫描后的 drain 被处理；前提还包括此前 helper 和 drain 自身正常完成。若 PCB ref 尚为正，未来外部 unref 的进展与其后新入队对象不能由此接点覆盖。

这个 drain 并不是 clearEnvironment 的最后一个清理操作：`0x080acb83` 之后还调用 identifiers 析构，`0x080acb8d` 调用 `vm_destructAllObjects(0)`，随后清断点并重建 identifiers。当前证据只关闭命名的扫描后排空接点，不能跳过这些后继例程而声称整条退出最后不会再有清理或入队。

### shutdownScheduler：先 drain，再注销静态槽和原始存储

`vm_shutdownScheduler` 的完整新增记录为 `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5`。从其他 task 调用时，它先置 requestStop 并等待 alreadyStop；该等待不是已证明的进展保证。之后存在 returnCode 为 1 时按 tick 上限继续调度的循环，`0x080ae18a` 调用 internal。它再处理 timer/invoke/pend、扫描 eligible PCB，在 `0x080ae296` 调用 deleteProcess，扫描后于 `0x080ae2b0` 调用对象 drain。

与 clearEnvironment 相同，能够限定确认的是：正常到达扫描后 drain 时已经排入队列的对象，进入这个明确接点。外部 PCB ref 未归零、hook/finishHook 的异常或重入，以及停止调用者是否实际进入 shutdownScheduler，仍是独立前提或未知。

排空之后仍有静态槽和 buffer/PCB 存储注销。这里必须保留精确顺序：`0x080ae2eb` **先把每个栈槽的 type（偏移 +4）写成 0**，然后 `0x080ae2fd` 调用 `vm_destructStaticValue`。既有 `vm_cleanupValue` 在 `0x0808ada4` 从同一 +4 偏移读取 tag，依据 `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`；`vm_destructStaticValue` 的既有记录为 `ev_b7275ea0d205e76bd66e12eb46ee8132bb75cd0db3113213fdd60970242bb1c2`。不能把该静态槽循环误写成在 drain 之后还会按原 tag 再次清理原载荷，也不能仅从 type 清零推断任意外部根和全部 ref 的最终寿命。

## 延后 unref 的调用边界

`vm_unrefToPCB` dossier 的已解析直接调用者包含 `vm_serviceWakeupPCB`、`vm_transferControlToServiceThread`、`vm_f_delete_thread`、`vm_f_delete_thread_ok`、`vm_f_delete_thread_failed` 和 `vm_f_delete_thread_crash`。该列表不穷尽未解析间接调用；函数名称也不独立证明其实际执行线程或 callback 语义。

因此本批区分“最终 ref 归零时清理的局部顺序”与“某个持有者会在何时归零”。其他线程、JIT、服务、callback、hook 和重入的进展性不能由普通解释线程路径替代。新增记录补齐了 clear/shutdown 中明确的扫描后 drain 接点；mainLoop 自动转入 shutdown、外部持有最终释放及后继批量清理全域仍未闭合。E02 只在命名子域更新结论，相关父问题和直接捕获门槛继续保留阻塞，不声称必然泄漏或无条件最终回收。
