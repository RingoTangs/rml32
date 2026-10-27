# D08 生产契约（设计，尚未实现）

D08 细化 [2BA D01](../stage2ba-design/design.md)，以 [api-contract.json](api-contract.json) 为可机器核对的接口清单。本文选择未来 Rust 的受限策略，证据仍限于已冻结的原生记录与既有合成测试；没有新增原生结论。

未来构造签名为 `ObjectStore::with_bounded_lifecycle(ValueLimits, LifecycleLimits) -> Result<Self, VmError>`。它明确替代 D01 中规划的单参数 `with_bounded_lifecycle(ValueLimits)`，没有修改现有 API。`new` 与 `with_limits` 保留 legacy 行为。模式、origin、配额、arena 与维护游标在 store/VM 转移中一起保存；`ValueLimits` 继续独立校验值图、工作区等资源。启用生命周期模式不授予任何新的 preparation 权限。

## 配额与计量

未来 `LifecycleLimits` 的五个字段均为 `usize`，逐项显式提供，无 `Default`，每个 store 构造后固定。非法范围返回 `VmError` 的 `Limit` 类别，构造不得发布部分可用的 store。

| 字段 | 允许范围 | 计费单位 |
|---|---|---|
| `max_objects` | `0..=min(usize::MAX, u32::MAX)` | resident Object 生命周期记录与尚未发布的记录预留；queued、shell、freed 但尚未 reap 的记录都占额 |
| `max_function_heads` | `0..=usize::MAX` | 已登记 Function 元数据与新 head 预留；语义退休后在元数据移除前继续占额 |
| `max_ordinary_refs_per_object` | `2..=usize::MAX` | 同一 Object 的实际 payload 槽、每个 Function head 的独立 creator、队列 owner 和临时预留 |
| `max_exec_refs_per_object` | `1..=usize::MAX` | 同一 Object 已安装帧租约与 prospective 执行租约之和 |
| `max_owners_per_header` | `1..=usize::MAX` | 同一 Function/Array/Mapping header 的实际语义 owner 与 prospective owner 之和 |

`max_objects=0` 允许空存储；`max_function_heads=0` 禁止新 Function head，并不禁止 Array/Mapping。对象命名空间 exhaustion 与配额 exhaustion 分开处理，均可返回 `Limit`。回收记录归还容量不归还 identity：ID/nonce 不复用，已发放后取消可以留下空洞。不得用截断或 wrapping addition 绕过任一配额；所有计数增加先进行目标 `usize` 宽度的 checked arithmetic。

配额限制语义数量，不承诺进程总字节或墙钟时长。Function head 总额不限制 Array/Mapping header 的全局数量、所有程序的总字节或每次清理的成员数；`ValueLimits` 也不是这些资源的全局总额。Rust 容器、游标索引、弱登记与分配器的空间必须单独设计和审查，不能把本契约解释为 OOM 可恢复证明。

## 峰值预留与发布

每个事务先计算并预留全部新增责任，再替换真实槽、转移帧或释放旧 owner。计算峰值时不抵扣将被覆盖、退休或退出的资源。因此同值覆盖或尾部执行租约交接虽然最终净额不变，也可能返回 `Limit`；执行上限为 1 合法，但需临时两个租约的交接可能失败。raw move 直接转移已有 owner，不新增 owner；物理观察 view、快照及同一实际槽的别名不增加语义计数。

预留 token 属于同一 origin 和事务，记录资源集合与源/目标版本。commit 前重新校验源槽、目标槽、creator 有效性及准备权限，成功时把 prospective 责任改为已安装责任，不重复加额；abort 精确释放未消费责任。过期、重复消费和伪造 token 必须拒绝。候选事务失败不能发布目标槽、源规范化、VM 帧或 PC 的部分变化，已存在的其他成功事务不回滚。独立模型验证版本冲突与显式 abort；真实 Rust 的 origin、借用和 token 无法伪造性仍是未来实现责任。

实际 Object 槽各有一个普通 owner。每个存活 Function head 恰有一个 creator owner，即使该 Function 被复制很多次也不重复 creator；若捕获槽还保存同一 Object，它是另一个独立 payload owner。Function/容器最终语义 owner 退出时先释放成员，再释放该 head 的 creator，最多一次。prospective owner 在 commit/abort 前同样承担保持 head 的责任；纯物理 view 不承担它，不能延迟语义清理，但可延迟元数据回收。

独立租约只引用 origin/entry identity，不从记录建立指回 arena control 的强拥有边。arena 只有一个结构清理责任人；外部 Host 根可以保活 control，内部 owner 不得形成 arena 到 control 的强环。此项是生产内存布局要求，Python 计数模型不能证明它。

## 对象队列与清理责任

每条 Object 记录在发布前预分配可重用队列节点，并保留一个普通引用单位。alive/pending 的普通 owner 总数（含预留）上限为 `max_ordinary_refs_per_object - 1`；queued/shell 等 invalid 状态可使用完整上限。这样最后一个 pending 执行租约退出时能够使用已预留单位，原子地 invalid、LIFO 入队且恰好增加一个 queue owner，不能因配额或节点分配失败而丢失清理责任。释放 prospective 执行租约也适用该规则。预留节点与计数单位不意味着全部分配操作无条件成功。

存活 Object 的普通引用为零不触发业务删除。active exec 下的 remove 只置 pending，重复 remove 不增加责任；零 exec remove 先检查借用与工作区，再同步清理变量并成为 shell/freed。失败时该次 remove 不变。freed 记录仍占 `max_objects`，显式维护 reap 后才归还。托管创建/构造失败、取消的清理统一入队，包含零 exec 情形，不能把可能失败的变量清理塞入最后租约的 Drop。

队列清理把当前对象从链表脱离后，继续持有其 queue owner，完成全部变量清理后才释放 owner。一次清理为一个不可分割的工作单位；遇到借用/工作区不足，保留原队头及其责任，不跳过它。具体预算和部分成功见 [维护协议](maintenance.md)。

## Host、COPY 与既有边界

D08 明确替代 D01 事件表中“Host retain / retire: COPY into a Host slot”这一行：Host retain、export、回调输入/返回采用当前 raw transport 语义，建立或转移实际 Host 槽及相应 owner，不因已 retained 的 Object/creator invalid 而主动做脚本 COPY 规范化。raw retain 仍要校验 origin、存储状态和配额，不能凭任意旧 ID 复活 freed/reaped 对象。

脚本 COPY 在其既有边界规范化实际源槽与目标槽：invalid Object 或 invalid creator 的 Function 变为 Undefined；raw move 与唯一 head 捕获转移保留原 payload，共享 head 捕获走既有 COPY 规则。Host 回调仍只执行一次：提交验证失败保持 VM 操作数与 PC，不撤销外部副作用，也不重放回调。维持现有稳定 capture cell、投影 DAG 检查、受限类型 profile 和准备门禁。

现有 Host 根退休安全点保持，不追加隐式 Object 队列 drain。run/step、错误粘滞、cancel、结果提取及 store/VM 转移都不隐式排空对象队列；已有正预算 task step 对 pending 删除的中止规则保留，零预算不凭空推进任务。close 后新使用返回既有错误，仍存活的 Host 根保持 arena，直到退休或最终结构 teardown；终结 teardown 不伪装成一次受预算约束的维护调用，也不构成业务析构或原生退出进展承诺。

未来受影响文件及职责见 [migration.json](migration.json)。所有行 `implemented=false`；私有测试层的 `usize::MAX` 容量不得成为生产默认配额。
