# 2AY E01：零捕获未发布函数头路径

本页只记录原生静态观察和带前提的有限路径推导。没有执行参考程序、业务字节码或新增独立状态模型。主 ELF、历史证据及原有 Rust 运行时保持不变。E01 的分支可达性与最终释放分别验收；前者有条件闭合不关闭后者。

## 初始化、动作表与有限满栈路径

新增 `vm_initProcess` 完整记录 `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` 表明：成功初始化后，`sp=stackSize`、`funCp=0`、`ref=1`、`catchCp=0`；entry context 的 `bp=stackSize`、`ip=0`、`flags=2`，初始化时 `thisObject` 被清零。显式非空 attrib 路径将栈尺寸夹在 256 至 65,536 之间；`pAttribs=NULL` 的路径直接使用默认值，不能把显式分支的夹限无条件套给默认分支。

原始指令包括 `0x080ab1d3` 的最小栈设置、`0x080ab514–0x080ab528` 的最大值判断、`0x080ab37b` 的 sp 写回、`0x080ab37e` 的 funCp 清零、`0x080ab38f` 的 ref 设置，以及 `0x080ab413` 对首个 context 的清零。

冻结的 [actions.csv](../evidence/actions.csv) 与 ELF 动作表各 28 字节记录已逐字段核对：

| 动作 | 表地址／文件偏移 | 入口／参数数／attrib | 原始字节 |
|---|---|---|---|
| `17` | `0x083cc904`／3684612 | `0x08091f34`／2／0 | `1700000054a50c08341f090802000000bc022b086e66756300000000` |
| `3b` | `0x083cccf4`／3685620 | `0x08090ae8`／1／8 | `3b00000054a50c08e80a09080100000050042b083a6e3a3a08000000` |

复用 `vm_regularProcessor` 完整记录 `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`：普通、无 nextActionHook 分支按 actionNo 索引动作表；`0x08097986–0x08097992` 实际先压入 par8，再压入 par16，随后间接调用两参数入口。单参数分支在 `0x08097a60–0x08097a67` 压入 par16。伪代码省略了两参数调用的第二实参，不能用它替代这些机器指令。该分派路径没有统一的 sp 余量预检。

复用 `vm_createFunctionValue` 完整记录 `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`：`0x08091f5a` 取低六位作为捕获数；par8 为 `0x80` 时是 Global、零捕获编码。par8 在普通分派时虽被符号扩展，helper 的位操作仍得到上述低六位及 Global 位。

新增 `vm_pushShortNumber` 完整记录 `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a`：`0x08090af5` 只在入栈前 sp 已为零时错误；`0x08090af7` 递减 sp；`0x08090b06` 写 Int tag，`0x08090b1c` 写数值。因此最后一次从 sp=1 的入栈成功后，sp=0 是合法满栈状态。

据此选择以下有限路径：

1. 提供有效 entry code segment，显式 attrib 的 stackSize=256，初始化和必要环境准备成功；普通解释器、ctrlFlags=0、无 nextActionHook/JIT 或其他状态干预。
2. entry 从 ip=0 执行 256 条 `3b(0)`。第 k 条前 sp=257−k，全部写入 `[0,255]` 内的合法槽，执行后 sp=0。
3. 第 257 条为 `17(g,0x80)`；g 是已存在、可由 par16 表示且类型匹配的有效 Global 目标，Global 程序及目标读取有效，创建的两次分配成功。
4. `17` 不消费捕获槽，最终仍见 sp=0，抵达 `vm_createFunctionValueEx` 的 `0x08091df6` 分支与 `0x08091ee1` 错误调用。

257 条动作少于普通 processor 初始 `0x800` 次动作的计时检查间隔；这里不需以一次外部计时器事件解释中途恢复。以上是给定合法 entry 状态和有效目标的静态有限路径，不是主语料或原编译器实际产出该序列的证明，也不是执行观察。若上游另外限制 entry 可接受集合，那个集合的可达性须另行判断。

## H、A 与 creator 的发布位置

复用 `vm_createFunctionValueEx` 完整记录 `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`：先分配 0x18 字节头 H，再分配 `argn*0x18` 的捕获存储 A；H 指向 A，可能独立增加非空、有效 creator 的普通 ref。所选 entry Global 路径的 creator 可为空；非空 creator 的持有义务须另记，不能因 H 未发布而忽略。

零捕获分支跳过整个 `vm_initValue` 循环，因此 A 不因捕获槽初始化进入 `vm_allValues`。H 本身也没有在此 helper 中作为 Value 槽登记。H 的成功结果发布位于错误分支之后的 `0x08091e0b`／`0x08091e21`；在所选错误边，头仍是 helper 的 C 局部指针，尚未写入结果栈槽。

复用 `pdb_allocateMemory` 的完整记录 `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` 已证明零字节 A 的条件成功路径。这里不能再用未知 `malloc(0)` 行为规避这一分支。

## allocator 存储链不是载荷恢复契约

新增三条完整记录：

- `pdb_extendBlocksOfGroup`：`ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d`。
- `pdb_freeMemory`：`ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab`。
- `pdb_l2FreeMemory`：`ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a`。

extend 在 `_pdb_extendGrpList` 中链接整片扩展存储，记录块尺寸／数量，并初始化各块的管理头及自由链。allocate 从自由链取块或走独立分配；计数、allocated bit 和存储页地址本身不等于 VM Function 恢复根。

free 接受显式传入的 payload 指针，校验管理头及尾标记，清 allocated bit、更新统计，再回池或进入 l2Free；l2Free 回到组自由链或调用 libc free。所选完整例程未按 VM Value tag 分派、清 Function 捕获或 unref creator。即使以后证明某个 shutdown 可批量释放扩展页，也须分别判断物理存储释放和 creator／捕获载荷语义释放。

## 仍未闭合的退出边界

2AX 已记录 errorQuit／backend／PCB 最终清理链；本轮保留其有效环境和 hook 不干预前提。PCB 最后 ref 归零会清理已登记栈值、deletionFunc 和符合 flags 的帧 execRef；没有由此证明它能找到未发布 H/A。分配统计和存储池链也尚未给出该语义恢复关系。

因此 E01 的有限分支可达性现有静态依据，但未发布头的恢复根、H/A 最终释放以及可能存在的 creator 普通 ref 释放继续 unresolved。结论只界定已检索的创建、allocator 和 PCB 清理接点；不因局部没有 free 宣称全局泄漏，不宣称已由 GC 或 allocator shutdown 回收。后续仅需继续追具体的恢复／注册关系和最终释放边；直接 Object/Function 捕获仍受生命周期门槛阻塞。
