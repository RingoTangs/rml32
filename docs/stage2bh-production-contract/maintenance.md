# 显式维护与可观察进度（D08）

未来只在 store 与 VM 层提供 `maintain_lifecycle(LifecycleBudget) -> Result<LifecycleProgress, VmError>`，不赋予 `HostScope`、`HostValue` 或 callback 维护整个 origin 的权限。legacy 模式在工作前返回 `UnsupportedFeature`；closed 的 bounded epoch 在工作前返回 `InvalidState`。转移中的 origin 不改变维护状态。

## 预算和工作顺序

`LifecycleBudget` 没有默认值，三个 `usize` 字段 `host_roots`、`objects`、`metadata_entries` 均允许零。每次调用按以下顺序消费各自独立的预算；某一类耗尽预算不挪用其他类的预算，也不禁止后续类尝试工作。

1. 按 lease ID 升序退休已标记的 Host 根，一根一个单位。尚未退休的根不消耗这项预算。
2. 按 LIFO 队头清理已入队 Object，一个对象的全部变量清理为一个单位。操作期间 queue owner 持续有效，成功后释放。
3. 按持久游标扫描本 origin 的 Object/head/weak 元数据，一条检查记录一个单位，包括检查后发现尚不能回收的条目。检查已在上次调用回收的游标位置也计一个单位，不能反复从头扫描而忽略后面的记录。

借用或工作区阻塞立即结束本次调用，不跳过被阻塞单位继续扫描。每个单位先完成必要的借用、工作区及级联成员释放预检，再提交该单位全部变化。预检失败不消费单位、不前移该单位游标或移除队头；之前已经成功的根退休、对象清理和元数据回收保持。必要的计数与释放不能靠重新调用任意用户回调完成。

这些是工作单位预算，一个对象可以有许多变量，一次语义 owner 释放也可级联释放一个受限 DAG；它们不限制指令数或时延。metadata budget 也不把 Rust 索引维护及底层分配器操作变成常数时间。

全零预算只读取当前进度，不启动扫描、不推进游标、不退休 Host 根、不清理对象；必须避免为了验证入口而隐式调用会退休根的 `Registry.check`。入口的模式/closed 检查仍然有效。正预算维护中的 Host 根退休只能按本调用的 `host_roots` 额度推进，其他已有 Host 安全点继续遵守其既有协议。

## 进度和返回值

`LifecycleProgress` 包含下列 `usize` 字段，外加 `scan_complete: bool` 与 `stop: LifecycleStop`。

| 字段 | 本次成功计数或返回时快照 |
|---|---|
| `retired_roots` | 本次实际退休的 Host 根 |
| `drained_objects` | 本次完成全部变量清理并释放 queue owner 的 Object |
| `reaped_objects` | 本次移除的 freed Object 记录，即归还的 Object 配额 |
| `reclaimed_heads` | 本次移除的退休 Function 元数据，即归还的 Function head 配额 |
| `pruned_weak_entries` | 本次移除的已失效弱登记 |
| `metadata_examined` | 本次成功完成检查的元数据条目 |
| `pending_roots` | 返回时仍标记待退休的 Host 根数量 |
| `queued_objects` | 返回时仍在 Object 队列中的对象数量 |

`drained_objects` 不表示普通引用已经归零，`reaped_objects` 也可来自之前已同步删除的对象。`reclaimed_heads` 不包括 Array/Mapping header 的元数据移除；这些检查仍计入 `metadata_examined`。pending exec Object 尚未进入队列，因此不计入 `queued_objects`。

`LifecycleStop` 只有四种变体：

| 变体 | 含义 |
|---|---|
| `Complete` | 已标记退休的根和对象队列均为空，元数据扫描观察过当前 mutation epoch |
| `BudgetExhausted` | 仍有上述工作，或尚不能确认当前 epoch 的扫描完成，而相关预算已用尽/为零 |
| `BorrowBlocked` | 当前原子单位因借用冲突不能推进，先前成功计数有效 |
| `WorkspaceLimit` | 当前原子对象清理无法取得所需工作区，先前成功计数有效 |

逐单位的借用/工作区阻塞返回 `Ok(progress)`，这样调用者不会丢失已经发生的进度。入口错误才在工作前返回 `Err`；不得在部分完成后仅返回无进度的错误。先遇到的阻塞决定 stop。全部预算为零也可在已经无待处理工作的 origin 上返回 `Complete`。

`Complete` 只表示当前退休/队列/扫描工作完成，不表示所有配额空闲。仍被持有的 shell、存活对象、物理 view 保留的退休 head 可以继续占额；也不承诺任务执行、调度器、shutdown 或业务析构最终完成。

## 元数据回收与公平扫描

每个 origin 保留 Object/head/weak 索引的稳定扫描位置、当前 pass 的固定 high-watermark，以及开始该 pass 时的 mutation epoch。新增条目不让未完 pass 从头开始，也不插入其固定范围；已到游标后的新变化由下一 pass 处理。当前 pass 结束时若 epoch 不同，必须再完成一次观察新 epoch 的 pass，之后才能将 `scan_complete` 置真。回收本身若改变登记状态，同样使旧 pass 变脏。反复修改可以一直延后完成，因此没有无条件进展保证。

可回收 Object 必须已经 freed、变量清理完成、普通与执行责任（含预留）均为零。可回收 Function head 必须已语义退休、成员和 creator 已释放、prospective owner 与物理 view 都为零。容器 head 遵循相同的生命周期条件，但不归还 Function 配额。弱登记只可检查和清理所属 origin 的条目，不得因为另一 origin 的清理使 ID 失配。带外旧 ID 和弱记录不能让已回收的对象复活。

epoch 使用 `u64` 时，wrap 必须设置无需分配的 force-rescan 标志，阻止旧 epoch 值碰巧相等而误报完成。下一次正 metadata 预算放弃旧 pass 并重新扫描；零预算保持全部扫描状态不变。禁止让 mutation epoch exhaustion 使最后 owner 的 Drop/入队失败。

实际 Rust 可使用稳定索引、游标或其他等价数据结构，但不可在每次调用无预算地构建全量扫描快照；索引增删、fence、物理 view 及弱登记的空间和借用协议必须在生产实现中独立验证。当前 Python 模型通过抽象条目列表表达扫描顺序，不证明该数据结构的生产可用性。
