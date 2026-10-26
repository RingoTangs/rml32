# 2AZ R02：PCB 退出接点与 shutdown 注册分类

本批只分类已知直接控制转移、地址取得及相邻持有事件，不递归展开任意 callback，不证明外部 PCB 持有者最终退出。下面的伪代码与机器码属于制品静态证据，不是原程序执行记录；直接 Object/Function 捕获和 T05-L/F/O 门槛继续关闭。

## 1. 六个调用者不等于六条 CALL 指令

`vm_unrefToPCB` 的既有 dossier `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` 列出六个已解析调用者函数。其 incoming reference 的七个接点均被 provider 标成 `UNCONDITIONAL_CALL`，但原始指令实际是 **三条 CALL 与四条尾部 JMP**。provider 分类保留原样，不能用它改写机器码 mnemonic。另有两个代码中的 DATA 地址取得接点及一条调试 DATA 引用。

| 函数 | 指向 unrefToPCB 的实际指令 | 地址取得与局部持有关系 |
|---|---|---|
| vm_serviceWakeupPCB | `0x080be71d: CALL` | `0x080be6ac` 增加 PCB ref；`0x080be6df` 把 unrefToPCB 地址存入节点 +8，PCB cookie 存入 +12；分配/入队失败路径直接 unref。 |
| vm_transferControlToServiceThread | `0x080be820: CALL` | 保存当前 PCB 信息后，`0x080be784` 增加 ref；`0x080be7cb` 将 unrefToPCB 地址存入节点 +8，cookie 是同一 PCB；投递失败路径直接 unref 后报错。 |
| vm_f_delete_thread | `0x080efb76: CALL` | `0x080efa90` 增加目标 PCB ref；以目标为 cookie 安装 crash、catch-failed 和 return-ok 接点；同步返回分支直接 unref。 |
| vm_f_delete_thread_ok | `0x080ef963: JMP`、`0x080ef97c: JMP` | 两个 killProcess 返回分支分别 push 结果后，尾转移到 unrefToPCB，参数为传入 cookie。不是两个额外 retain。 |
| vm_f_delete_thread_failed | `0x080efbf3: JMP` | 清当前 PCB 的对应 crash hook、push -1 后，以 cookie 尾转移到 unrefToPCB。 |
| vm_f_delete_thread_crash | `0x080efc02: JMP` | 本体把 cookie 作为 unref 参数并直接尾转移；不额外取得 PCB ref。 |

这七个指令接点不能与两处“把函数地址写进节点”的 DATA 接点相加后称为九次执行。`.debug_frame:0xffc8` 的 DATA 引用属于调试元数据，也不作为运行时调用、持有或恢复根。

六份完整新增记录分别为：

- serviceWakeupPCB：`ev_3c76db1101c3738b916aa8bca2093fcaff5ae5fb0e84d8f54f2621a9b2b1c4a7`。
- transferControlToServiceThread：`ev_85d6d3b5e1f800e18995b3afde1fe88c65a8868d87e4fb48281f59b6facdb666`。
- delete_thread：`ev_9918c7aad7104577421adf33640eebf8cbb67c3e9352ee05f6fdea9b1523ef5c`。
- delete_thread_ok：`ev_19750a0d8529d3bd1addcc87e6f3f0ddd172e6f174a81d4b8bad28ba8bed5e1b`。
- delete_thread_failed：`ev_a2c19154025b85d7b2098a80d34d3340cc92eb154e90f6b52bc4422758865d3e`。
- delete_thread_crash：`ev_2ee1efa9ea258bea8d0c461a36ef60686f0fd60451ffe5f8bfa951a71b6462ca`。

## 2. 局部 retain、投递与退出事件

### service wakeup 与投递

serviceWakeupPCB 取得 VM mutex 失败时直接返回。PCB 已置 `0x200000` 时，它保存 resumeHook/cookie、清 pending 位并发 semaphore；该分支没有上述 ref 增量。普通投递分支只在 `(flags & 0x28)==0` 时保存 hook/cookie 并增加一次 ref，然后构造节点，保存 resume 函数、unref 函数地址及 PCB cookie。入 responding queue 成功后调用 bringupMainLoop；节点分配或投递失败时，经 `0x080be71d` 退出这次局部 ref。

transferControlToServiceThread 先确定 pending 控制位并保存当前 PCB 信息，然后增加 ref。`serviceIndex==-1` 选择 responding queue，其他值选择 service queue；节点的 executionFunc 来自参数，+8 保存 unrefToPCB，+12 保存 PCB。成功投递把后续处理交给相应队列；失败释放已分配节点，再经 `0x080be820` unref 并报告错误。特定 service 状态下的 semaphore 等待、恢复信息及 resumeHook 间接转移属于另一个接点；记录仍保留 `0x080be88b` 未恢复跳转表的 provider 警告。

这些观察把“取得持有”“保存退出函数地址”“失败回退”分开。它们不证明队列消费者一定运行、destruction callback 恰好运行一次、semaphore 必然唤醒或 resumeHook 正常返回。本批也没有展开 vm_destructExecutionNode 来替代这些未知。

### delete_thread 的目标 cookie

delete_thread 在找到目标 PCB 且选中有效 deletionFunc 分支后，才于 `0x080efa90` 增加**目标 PCB** ref。它以目标为 cookie，在当前执行 PCB 上安装 crash hook（`0x080efaa3`）与 catch-failed 接点（`0x080efab0`）。随后增加的是 deletion Function 头的 ref（`0x080efabb`），这是另一种持有，不能计作第二次 PCB retain。

调用 deletionFunc 后，若 funCp 未改变，正常 helper 路径清 crash hook、结束 thread catch、kill 目标并 push 结果，最后在 `0x080efb76` unref 目标。funCp 改变时，`0x080efae4` 安装 return-ok callback，仍以目标 PCB 为 cookie；其后的 ok/failed/crash 本体各有上表所列的局部退出接点。

这只确认相邻的取得、cookie 传递与退出位置。pushNumber、killProcess、hook/catch 管理以及 deletionFunc 的异常或非局部退出不在本批递归展开；不能据三个 callback 的存在宣称所有路径均已平衡释放，更不能允许在 shutdown 已释放 PCB 存储后继续使用 cookie。

## 3. shutdownScheduler 的 DATA 注册与索引分派

旧 `vm_shutdownScheduler` dossier `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` 的直接 callers 为空，但 incoming DATA 位于 `0x083cfa98`。这是 `_vmInits` 表中的函数指针，不是“无调用者所以未注册”。另一条 `.debug_frame:0x102d4` DATA 仅是调试元数据。

新 `vm_shutdown` 完整记录为 `ev_170b57b39470172db518b7e0e8be6da6506c6e2795f171254705da6e8ab9b56b`。结合原 ELF 的 `_vmInits` 数据对象，形成如下静态注册/消费关系，原字节与查询边界见 [data-references.json](data-references.json) 的 D02/T01 及 X02—X05：

| 接点 | 原始数据或指令 |
|---|---|
| `_vmInits` 对象 | `0x083cf960`，400 个 file-backed 字节，即 25 个 16 字节记录。 |
| scheduler 记录 | index 19，记录基址 `0x083cfa90`；shutdown 字段 +8 位于 `0x083cfa98`，原字节 `e8 e0 0a 08`，小端值为 `0x080ae0e8`。 |
| 倒序分派起点 | `0x080b5c96` 取 index 24；`0x080b5c9b` 取 ESI=0x180，按 16 字节递减。 |
| runtime guard | `0x080b5caa` 读取 `[ESI+0x083cf96c]` 的记录 initFlag，零时跳过。 |
| 间接分派 | `0x080b5cb4` 调用 `[ESI+0x083cf968]`；ESI=0x130 时读取的恰是 scheduler slot `0x083cfa98`。 |
| 返回后的状态 | 回调正常返回后，`0x080b5cba` 清对应 initFlag，继续倒序遍历。 |

该分派还受 vm_shutdown 自身的 vm_initFlag/reentry 与 mutex guard 约束，且 add-in shutdown 和前序模块回调必须正常完成才能抵达所选记录。静态文件内 initFlag 的初值为零，不证明运行时始终未初始化，也不证明 scheduler shutdown 已实际执行。

本轮 exact-address xref 对表基址 `0x083cf960`、首 shutdown 字段 `0x083cf968` 及 scheduler slot `0x083cfa98` 均返回空列表；对 `0x083cfae8` 则返回 consumer 指令 `0x080b5cb4`。这些查询是 provider 的精确地址投影，并不穷尽索引表达式可能访问的每个表项。机器码的 ESI 初值、步长、字段偏移与原表字节共同支持分类；不能把空列表推成“没有 computed consumer”，也不能把部分恢复到的末表项引用推成已穷尽全部动态目标。

## 4. 本批确认与继续未知

本批确认六个已解析函数中的三条 direct CALL、四条尾 JMP、两个函数地址保存点，以及 scheduler 的静态 DATA 注册和有 guard 的表消费者。这些类别在完整记录、机器码和独立 reference 台账中分别保留，不以函数名、caller 数量或调试引用代替运行证据。

复用 `vm_waitMainLoopDone` 的 `ev_218395b72ac924cabe14085ef72ce27a156221b92b9de2e1d95e1152b16cb37e` 仍只支持局部行为：alreadyShutdown 非零直接返回，否则调用 mainLoop(NULL) 后返回。它的名字不证明停止后必然进入 vm_shutdown，更不证明按表到达并完成 scheduler shutdown。

继续未知的包括外部持有者的最终退出、任意队列/return/catch/crash/resume callback 的进展与重入、停止后实际外部调用顺序、PCB 存储关闭后的外部根寿命和全部后继安全点。本批没有将已找到的局部退出位置升级为 eventual reclaim，也不因存在 address-taken 注册而假定相应函数已执行；R02 仍为 partial。
