# 方法、边界与复现

在仓库根执行：

```sh
python3 -B docs/stage2bn-pair-audit/historical.py --write
python3 -B docs/stage2bn-pair-audit/reference_replay.py --write
python3 -B docs/stage2bn-pair-audit/pair_diff.py --write
python3 -B docs/stage2bn-pair-audit/public_replay.py --write
python3 -B docs/stage2bn-pair-audit/verify_validation.py --write
python3 -B docs/stage2bn-pair-audit/verify_validation.py
```

前四条实际执行生产者；主验证重算已保存结果并执行完整 Rust／Python 检查，最后一条独立核验收据。只读复核不虚报为再次执行 Rust。Cargo 全部 locked/offline，不安装工具链或镜像，不改写旧证据与收据。输出采用去重状态池和 gzip，输入由动作生成器及其哈希复现，不保存大型原始转储。

## 未提交基线的固定

任务开始时 HEAD 为 `072aecfb5d111003676fe21e2c34be54bab9f8d2`，完整 2BM 已暂存但未提交。`baseline-inputs.json` 记录 1,203 个文件字节哈希、原索引内容哈希及压缩补丁的解压后哈希；`baseline-overlay.patch.gz` 保存实际暂存二进制差异。快照通过本地 clone、checkout 固定父提交、apply 补丁重建，仅对已知换行差异按目标 SHA 恢复字节。

后续出现包含 2BM 的提交不改变这份快照权威；冻结校验以固定父提交与原索引内容比较，不要求 HEAD 一直停留在父提交。历史在重建的 2BM 副本运行：2BM 的 38 项加递归固定权威 557 项，共 595 项。当前源码不承担旧收据验收。原 15 个驱动通过冻结 runner 的 run 函数实际重跑，产物只写本批；D08 的 39 项模型及 112 行算术继续单独记账。2AW 语料仅核对冻结哈希，不重新执行原程序。

## 槽身份与两个独立准备

参考层声明 roots→Cell、Cell→payload、header→成员 Cell 的关系，并单独跟踪计划持有的 Cell 身份和物理 Value。多个 root 别名、两个计划的版本观察和写集合可以共同保活同一个 Cell；每个活 Cell 的 payload 只贡献一个 owner。外部 root 移除后 Cell 是否销毁，由剩余强引用可达性决定。CopyPlan 的 Value 快照不新增语义 owner，但其中的 header 可延后物理销毁与元数据回收。

COPY 准备记录真实 Cell 版本及既有规范化结果，提交前预留全部新增 payload 峰值，不抵扣旧资源。Function 准备实际取得捕获 Cell、HeadReservation 和 CopyPlan::output 槽；发布前 validate，失败先释放输出再归还 guard。执行准备持有真实 ExecLease；commit 转移到 installed 角色，abort 释放，同一租约不重复加额，不伪造 VM 帧。A/B 的未发布输出私有；槽地址及 header 标签访问均拒绝跨事务读取。只共享已发布槽，事务标识在这批四事件域中不复用。

commit 消耗计划，包括失败情况。候选写集合失败不发布源规范化或目的更新，但消费计划会合法释放它独有的最后 Cell 引用，继而释放 payload／creator，甚至使 Shell→Freed；不能把这种清理错误地要求为全部 before/after 不变。另一事务及此前成功的动作必须保持。

Mapping 与 Function 有一个明确的具体版本细化：生命周期最后 owner 清理成员一次；其物理数据析构还会再次使保留的成员 Cell 版本失效。参考层分别记录语义退休和物理销毁；第二次失效不能重复释放 owner。依据为冻结的 `container.rs` MappingData::drop、`value.rs` FunctionData::drop 与 `cell.rs` clear_on_owner_drop，不增加原生结论。

## 有限生成域

六类组合为 CC、CH、CE、HH、HE、EE；C 是 COPY 计划，H 是 Function 预留，E 是执行预留。20 个布局包括独立槽、共享源、同目的同值／异值、两向源目的交叉、根与成员退休、共享 DAG、已有 Function 成员，以及同对象／不同对象的五类混合配对。

每布局枚举 prepareA<终结A、prepareB<终结B 的六种合法事件次序，乘各自 commit/abort 的四种选择，共 24 种时序。串行次序是对照，并非同时在途证明。setup 中 Z 事务只初始化已有 Function，必须先完成；A/B 的目标成功计数排除 Z。

配额层沿用旧 163 个 profile，加入显式充足配置，以及独立参考层计算的五种资源峰值减一、等于、加一并去重。干扰层使用充足配额，在五个位置插入一次源改写、目的改写、根移除、Cell 别名、零预算或适用的物理 view 保留／释放；另将同步 delete 链、exec→delete→exit→单对象维护链与事务事件做全部保序交织，分别作用于两个对象。不是所有配额与全部干扰的笛卡尔积。

存活期 64 条额外路径覆盖 Object／Array／Mapping／Function，双计划共同持有源 Cell、移除外部根、两种终结顺序、四种 commit/abort 选择及可选 Cell alias。所有状态由真实动作产生。Freed 与 owner／执行租约并存、第三个同时在途事务、共享未发布输出、任意循环头图不在合法生成域。

## 观察、故障与维护

Rust 只接收动作、配额和逻辑身份，拒绝 expected 字段；不接收参考计数。回放器持有真正的资源对象，用 Weak<Cell> 与 WeakHeader 观察，bare ID 只用于身份映射。root 绑定、Cell 存活／payload／版本、生命周期计数、header、事务及租约角色分别输出。Expired Cell 为 null，与活着的 Undefined Cell 不合并。每个 setup 子动作也单独观察，动作前双观察相等，相邻检查点连续，最终 Registry／ctx Weak 必须失效。

workspace 校准以 family、动作、配额及完整语义签名选取成功边界，记录实际 reserve 次数 C，注入 0..C-1，并用 C 作成功对照。失败的 commit 仍消费自己的准备。维护注入返回 Ok(progress)，380 次 WorkspaceLimit 与普通 Err 分开计数。读借用覆盖有效写槽、将退休的成员、删除变量及队头变量；普通回归另验证清理第二对象受阻时第一个成功单位保持、两个计划仍存在且仅已清理变量使对应计划失效。

24 条元数据路径覆盖三类头、两种物理 view、四种终结组合，以及仍有另一个计划持有槽的扫描间隔。复用冻结 2BJ 的独立 snapshot-list 扫描参考层；输入只含维护前注册事实和 epoch，不用 Rust cursor 或返回计数推导期望。Object eligibility 由独立 Freed 状态校验，Function eligibility 由退休和物理可达性校验，已命名 Weak Cell eligibility 由独立槽存活校验。其余弱登记属于具体注册事实细化，不声称完整元数据模型等价。

最终 worker 补全了未发布 header 标签隔离后，在 debug/release 重放全部组，要求输出与已独立校验的观察逐字节相等。命令记录绑定当次全部 Rust 源文件 SHA、输入 SHA、观察 SHA、实际 argv、退出码和输出；汇总从原始观察重新推导，初始化成功不充作 A/B 覆盖。

## 正常库及最终验收

公开组使用正常库构建，覆盖八种顺序成员更新组合、两个独立 Function head 的 creator 释放和两个真实嵌套帧下的 Pending 删除。十种组合均有充足配额对照。90 条完整成功；其余 1,550 条按 setup／first／second／release／nested 的 Limit 拒绝分类，不计为完整目标成功，不声称公开工厂阈值与私有模型全部等价。公开 VM 保持同步，没有双准备接口。

892 项普通测试＝原 878＋新 14，另保留五项 doctest，两个构建的名称清单严格核对。17 个 ignored 驱动按原 15＋新 2 分别执行。46 项 Python 校验器覆盖所有权方程、时序、私有输出、丢行、乱序、身份／版本／存活／计数污染、缺失成功对照、命令失败以及收据篡改。收据在源码稳定后生成，并由独立只读调用核对最终来源和全部结果。
