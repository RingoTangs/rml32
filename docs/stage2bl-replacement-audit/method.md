# 方法、边界与复现

在仓库根执行：

```sh
python3 -B docs/stage2bl-replacement-audit/historical.py --write
python3 -B docs/stage2bl-replacement-audit/reference_replay.py --write
python3 -B docs/stage2bl-replacement-audit/verify_validation.py --write
python3 -B docs/stage2bl-replacement-audit/verify_validation.py
```

前两条执行固定基线历史验证和原 11 个驱动，保存到本批目录；主验证复核其实际日志并执行当前完整 Rust 检查、新差分与 Python 校验器。最后一条只读重算收据。Cargo locked/offline，不安装工具链，不拉镜像，不改写任何旧收据。初始历史 525 项及旧驱动已实际执行；主验证中的只读子命令不会虚报为再次执行。每个子报告保留自身实际命令和返回码。

## 固定基线

`baseline-inputs.json` 固定提交 54eb8e6 的全部 1,132 个路径及字节哈希；历史 Python 在 detached 副本运行，恢复经哈希确认、仅换行不同的 CRLF 文件。历史计数为 2BK 校验器 28 项加递归固定版本的 497 项，不能在当前已扩展源码上运行旧收据验收。2AW 语料结果只复用其冻结哈希，不重跑原程序。

## 生成域

16 个布局包括 Object 互换；Array/Mapping/Function 各自的同头重复写回、最后 owner 覆盖、外部别名保留、覆盖根与改写成员同时发生；两个父头共享子数组的退役及带 Function creator 的变体；占用 raw 目的拒绝。

沿用 2BK 的 163 个配额配置，添加 `(objects=3, heads=2, ordinary=12, exec=2, owners=12)` 成功对照，再由新参考层计算各布局在初始化及零干扰提交中的资源峰值，为 objects/heads/ordinary/header owners 添加临界值减一、等于、加一的合法配置并去重。每布局最终有 172—174 个配置，exec 保留既有有效值，不声称本批覆盖新执行租约行为。

每个配置枚举长度 0—2 的六字母干扰词：源 s0 改写、目的 d0 改写、外部 alias 释放、h0 物理视图保留／释放、全零预算维护；分别 commit/abort。总数 238,736。view 仅对未退役的现存头获取显式物理视图，重复 view 替换已有视图，不引入语义 Owner。不存在 alias/h0 时对应动作无操作。原 2BK 场景不改写。

Rust 的 setup 宏逐项执行实际对象、头和根构造；发生失败时记录精确失败子动作索引与部分完成状态。Python 对同一序列独立逐项推导并比较，前置失败不会被伪装成整体初始化回滚。成功后逐动作比较 before/after；每次动作前重复观察必须相等，每条 trace 结束后要求原 Registry/ctx Weak 失效。

## 独立参考层

`replacement_oracle.py` 新建槽、成员、头和 creator 的声明式图，没有复制或修改旧模型。每个 Slot payload 恰好贡献一次 Object/Header 引用；每个未退役 Function 独立贡献一次 creator 引用。复制头不重复计算 creator。Object 都为 Alive，ordinary 配额预留队列的一单位；没有在本批引入删除、执行、reap 或完整 metadata 等价规则。

准备阶段记录源快照和所有受关注的版本，写集合按目的去重；版本不一致先拒绝。提交先按新值总量检查峰值，保留全部旧引用；发布所有新槽值后，以当前引用图计算无 Owner 的头，清空其成员、释放 creator，再迭代至稳定。被退役头的成员即使持有外部 Rc<Cell>，也必须变成 Undefined；同一事务写入该成员的新值随后被最终清理，不能遗留新 Owner。共享子图不按父边数重复计算成员，外部语义别名保留它，纯物理视图不保留语义内容。

Rust 操作实际 Cell、CopyPlan、HeadReservation 和容器图。Function 初始捕获为允许的数组；直接改写私有 capture Cell 仅用于内部释放边界测试，不是开放公共捕获接口。已有值的批量替换使用 copy/write；负向 raw 场景直接调用内部 transfer_batch，保持“未发布空目的”前提，不新增占用目的移动 API。

29 个读借用案例分别借用写入目标或旧头成员：在待写／递归退役集合中拒绝，否则允许。workspace 校准取每个布局及配额下零干扰的成功提交，记录实际 reserve 次数 C，穷举 0..C-1 并以 C 作成功边界：共 3,169 次注入拒绝、346 次成功对照。失败仅允许弱缓存预登记等既有实现细化；实际语义状态、成员、计数、creator 与队列必须完全符合独立模型，不能用缓存解释语义偏差。

## 观察与公开验证

观察器临时读取真实状态，不获取语义 Owner，不触发 safe point，不保存新的强头视图；显式 view 动作才保存物理值。成员槽 Rc 作为测试指定的稳定地址保留，不添加其 payload 的 Owner。原始进程 ID/counter 映射成稳定逻辑身份，Int 与 Undefined 不合并。

为避免保存大重复转储，Rust 对相同完整观察状态去重，JSONL 输出先定义 state ID→完整值，再逐检查点引用它。ID 仅用于序列化，与运行时身份无关；Python 检查引用存在、顺序正确、前后连续，并从完整值复算结果。两个构建的压缩观察逐字节一致。内部弱注册数量和 mutation epoch 不被冒充为抽象图计数；本批全零维护断言保持完整实际 raw 观察不变，完整元数据维护继续由冻结旧回放验证。

正常库的 2,624 条场景通过真实 VM 赋值 opcode 替换 Array/Mapping 的占用成员，旧 payload 为 Object、Array、Mapping 或 Function，分别保留／不保留外部别名。公共 Function 经允许容器捕获的 maker VM 创建。成功写入 Int(7) 后删除 victim，再验证对象只在最后保活引用消失后被 reap 一次。它是单次成员赋值的公共投影，不声称公共 API 提供多槽原子事务。32 次完整成功，其余 2,592 次在 setup/start/prefix 因配额拒绝；它们不计为目标原子性覆盖，也不声称完整配额阈值等价。

## 完整验收

原 863 个普通测试名称和全部旧测试字节保持，新六项单独计数，doctest 允许行偏移但保留五个案例。旧 11 个驱动经冻结 runner 的 run 函数实际重跑，产物只写本批；配额模型 39 项、112 行算术由其原 runner 单独重跑。当前 32 项校验器测试涵盖临界峰值、共享退役、creator、物理视图、失败原子性、丢行、身份污染、命令/测试清单和收据篡改。收据绑定本批源码、文档、状态池、命令日志和固定基线。
