# 2AY 后续：T05-lifecycle-contract-evidence

下一批继续选择 **T05-lifecycle-contract-evidence**，聚焦下列已定位的剩余关系。2AY 的有限路径与命名 drain 接点已经交付，不应重做同一创建、分派、allocator 或 clear/shutdown 大记录来增加数量。当前不转入 D01、不开放直接 Object/Function 捕获。

## R01：未发布 H/A 的实际恢复关系

**状态：** unresolved，阻塞 T05-L 的相关退出域。

**已有基础：** [E01 路径](e01-paths.md) 和 F41—F47 已给出合法栈、零捕获编码、创建失败前的 H/A／creator 持有以及 allocator 扩展页和显式 free 接点。下一步不能再以未知零长度分配、空直接 callers 或“尚无有限栈路径”为理由重开已闭合子题。

**取证入口：** 沿 H 的 C 局部位置在错误 longjmp 前的实际保存位置、错误恢复可访问的注册结构及 `_pdb_extendGrpList` 的读取/销毁接点追踪。只有发现具体关联时才补查 allocator shutdown，例如 `_pdb_shutdownMemMgr`；区分扩展页整体 free 与 Function 捕获／creator 的语义析构。若主张不存在恢复关系，必须说明检索了哪些读写位置、间接调用和边界，不能由局部未见 free 推成全局否定。

**退出条件：** 对命名域证明不可达，或给出 H/A 发现→相应 free→非空 creator unref 的完整证据链；只闭合存储释放时单独标记。无法闭合则保留具体未解析指针或调用边，不以 Rust 预检政策代替原生结论。

## R02：未覆盖退出域与外部持有的进展契约

**状态：** partial；可与 R01 独立取证。

**已有基础：** [E02 矩阵](e02-paths.md) 已定位“实际完成后续普通 backend”和“正常抵达 clear/shutdown 扫描后 drain”两个类别。下一批只追还影响拟支持域的外部 PCB 持有、mainLoop 停止后的实际调用者，以及明确后继 helper 的返回/再入队关系。

**取证入口：** 先分类 `vm_unrefToPCB` 已解析调用边及其发生时 PCB 存储有效性；对确需支持的路径再检查持有取得、最后退出和后继安全点。只有在证明停止后实际调用者顺序时才补查 `vm_shutdown`；间接 release/crash/end/response hooks 和其他线程/JIT 不以名称或 caller 清单视为已穷尽。

**退出条件：** 为选定域明确入队、触发排空、变量清理、队列 ref 退出及可能的壳释放条件。进展性与局部次序分别验收；需要排除的行为必须在未来入口上可明确拒绝，不能把实际依赖隐藏在“out of scope”中。

## D01 与能力开放的前置判断

只有 R01/R02 为拟支持域提供可审查的生命周期契约后，才进入 D01 的私有状态、持有事件、迁移清单与案例设计。D01 仍须分别处理 creator／载荷普通 ref、execRef、待删、失效、队列及最终壳释放；不能用物理 Rc 合并它们。

直接 Function 与直接 Object 捕获保持独立门槛，均需先通过适用的生命周期基础、再固定显式能力入口并进行真实 Rust 验收。捕获槽 Reference 逃逸、任意循环 GC、新 catch 来源、异步和重入继续排除。下一批静态结论、将来的模型或测试通过，都不能自动关闭 Q05/Q06 全域或改写历史收据。
