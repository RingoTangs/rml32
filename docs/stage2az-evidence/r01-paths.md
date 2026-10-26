# 2AZ R01：保存副本、恢复元数据与逐分配处置

本页仅记录给定零捕获失败路径的原生静态证据。2AY 的有限栈路径保留；本批不重做可达性证明、不运行目标，也不新增状态模型。保存、发布、可被恢复代码发现、物理释放和语义释放分别验收。

## 已保存的 H/A 与结果发布

既有 `vm_createFunctionValueEx` 记录 `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` 的机器码已明确保存副本：

| 接点 | 原生观察 |
|---|---|
| `0x08091d47` | H 写入 C 栈槽 `[EBP-0x14]`。 |
| `0x08091d74`、`0x08091d9f` | A 留在 EDI，并写入 H 的 `+0x10` 字段。 |
| `0x08091dc4`、`0x08091ed2` | 可选 creator 保存到 H 的 `+0xc`，有效非空 creator 增加独立普通 ref。 |
| `0x08091de1` | 从 C 栈副本重读 H 到 ECX；所选零捕获末尾错误调用前仍有这些局部副本。 |
| `0x08091e0b`、`0x08091e21` | 成功分支才发布结果 tag 和头指针；零捕获错误边尚未抵达这些发布位置。 |

因此不能把“未发布”写成“没有保存”。真正未闭合的是错误恢复如何发现这些保存；寄存器值仅在所选调用接点观察，不保证跨 `vm_error` 或 longjmp 保留。零捕获跳过 `vm_initValue` 捕获槽循环，H/A 不因这条循环登记为 Value 槽。

## catch 元数据的实际内容

新增 `vm_newCatch` 完整记录 `ev_68f399017c978bc9951a6157ba0277fd79f2bdfa6733016d6ed2c3567999f037`：取得当前 ThreadInfo，检查 topCatch 小于 16，写选中 topEnv 项的 funCp/catchLevel，增加 topCatch 并返回该缓冲地址。它没有接收 H/A 参数或建立这两次分配的登记关系。

既有 `vm_doActions` 的 `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` 在普通 backend 进入 processor 之前调用 newCatch 和导入的 `_setjmp`。这说明现有恢复元数据的建立时点和字段，不等于已穷尽外部 `_setjmp`／longjmp 实现、别名或任意外部根。已有 PCB 清理仍只按其可访问的登记值和帧工作，不能据此假定它扫描未来 H 的 C 局部槽。

## 扩展页链的五个精确接点

对 `_pdb_extendGrpList @0x0860d024` 的成功 xrefs 记录 `ev_81d1554fd5d64a6256f880921e660f95ce735c39064a55299a7eed399f20a1d2` 返回五个源地址：shutdown 中 `0x080ce40e` READ、`0x080ce420` WRITE、`0x080ce42f` READ，以及 extend 中 `0x080ce67c` READ、`0x080ce68f` WRITE。

地址列表本身没有 kind；上述 READ/WRITE 来自相应完整 dossier，并与原始指令字节交叉核对。该 BSS 对象是四字节虚拟存储，无可伪造的 file-backed 内容。完整查询、对象及逐接点数据见 [data-references.json](data-references.json)。这些是 Ghidra 已分析的精确地址引用，不证明任意别名或计算访问都不存在；首次超时请求也不是阴性结果。

新增 `_pdb_shutdownMemMgr` 记录 `ev_ac794b00be49819c561c00d6b382688b5a0eae190a4833c69d176f6c996d907d` 显示：循环读取扩展页头，先把全局头推进到 page 的 next，再调用 libc free；之后释放 flat/waste 和条件存在的 stable-heap 存储，销毁临界区并清初始化状态。这是物理存储清理；该例程没有 Function/capture cleanup 或 creator unref 分派。数据引用已定位到既有 extend 与此 shutdown，无具体新 reset 关联需要扩展到 `_pdb_initMemMgr`，本批不为增加记录而补查。

## 逐分配结论

[allocation-disposition.json](allocation-disposition.json) 分别列 H/A 的 extension-pool 与 fallback 分支，以及 creator present/absent 两类，共六行：

- H/A 实际位于仍链接的扩展页时，正常抵达 allocator shutdown 可物理释放包含该块的页；这不保证该 shutdown 最终发生，也不补出 VM 语义析构。
- 独立 malloc fallback 块与扩展页链分开处理。块带有 eligible-group 序号或 flag，不足以证明它属于扩展页；没有证据时也不把 flat/waste/stable 存储与 H/A 合并。
- creator 缺省时没有该 ref 的释放义务；非空 creator 的普通 ref 仍需要明确 unref 接点，不能因页被 free 而宣称已平衡。A 的零个捕获槽没有载荷析构义务，但 A 存储、H 与 creator 的处置仍独立。

## 精确剩余前沿

本批已定位保存副本、catch 字段、五个已分析页链访问和 shutdown 存储处置。仍缺少的是：错误后可用的 H/A 发现／读取关系，以及它到适用 H/A free 和非空 creator unref 的连接。只对这些命名接点给出有界负面结论，不宣称全局泄漏、全局无根或已被 GC 回收。

后续扩展必须由具体新增别名、登记字段、恢复 reader 或导入恢复实现触发；不能重复同一 helper 来复述 unknown。没有新接点时，应先界定目标是否要求该原生失败清理行为等价，再选择进一步证据或明确差异的设计方案。本批不自行改变门槛或推进 D01。
