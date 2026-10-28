# 方法与复现

在仓库根运行：

```sh
python3 -B docs/stage2bj-contract-audit/verify_validation.py --write
python3 -B docs/stage2bj-contract-audit/verify_validation.py
```

第一条执行全部命令并生成本批收据，第二条只读核验保存的日志、来源、观察和哈希。Cargo 使用 `--locked --offline`；不拉取镜像或安装工具链。`validation-logs.json.gz` 保存逐命令 argv、退出码、stdout、stderr；基线反例中唯一预期失败是旧版 Mapping 回归测试的退出码 101，单独保存在 `baseline-proof-log.json.gz`，不会当成当前验证成功。

## 动作与观察

`matrix.py` 只生成动作、显式配额和逻辑标签；传给 Rust 的 JSONL 没有预期状态、计数或 oracle 结果。`src/lifecycle_contract_tests.rs` 操作实际 bounded StoreState、Cell、CopyPlan、ExecLease、头预留和 HostValue，不另建所有权账本。逻辑标签将实际 ObjectId、头 counter ID 映射到可比较身份，标量仅保留 scalar 类别。低层 exec 动作仅证明执行租约，不据此声称调用过真实 VM 帧；F16 和六项普通组合测试另使用真实 VM。

观察器从实际对象、头、队列及 weak 注册表读取数值，临时读取的 Value 在返回前释放；不获取语义 Owner、不触发 safe point、不保存新的物理视图。每次动作前连续观察两次要求相等；Python 再要求上一步 after 与下一步 before 完全相等。显式 `view` 动作才保存测试指定的物理视图。每条 trace 结束后要求原 Registry 与生命周期 ctx 的 Weak 均失效。测试保留实际槽 Rc 以提供稳定目的地址，它不增加槽 payload 的语义 owner，但会影响 weak 元数据存活，因此按实际注册事实检查维护。

冻结 `docs/stage2bh-production-contract/quota_model.py` 作为语义权威。`oracle.py` 是动作组合及身份细化层：公开原子 create 对应模型 reserve+commit；带失败的原子构造在候选模型上计算，失败不发布；staged commit 失败显式 abort 后再观察。拆分 Object reservation 和可伪造 token 协议并不是新增 Rust API，详见逐案例分类。

## 元数据细化

Rust 实现有四种实际元数据行：对象、Function 头、weak Cell、weak graph；模型的抽象 rows 与实际索引数量不同，不能直接比较其 cursor 或计数。本批新增 `maintenance_oracle.py`，旧模型不修改。它使用排序快照的待访问列表，区别于 Rust 的 BTree range cursor；输入只有维护前实际注册 identity、eligible 事实及 mutation epoch/force。它不读取 Rust cursor 来生成预期，也不使用返回的进度计数推导预期。

该参考模型独立推进预算和扫描代际，比较全部 progress 字段、剩余元数据键、mutation epoch/force 和 scan_complete。F13/F14 的纯元数据阶段 host/object 预算为零，避免将两个不同抽象混算。零 metadata 预算阶段的 root/drain 语义由冻结模型核对，cursor 必须原样保留；纯零预算要求完整 before/after 一致。借用元数据表时采用既有保守预检规则，返回 BorrowBlocked 并不消耗 metadata 单位。原始 scan 字段只用于观察及“不推进”断言，不作为参考模型的扫描进度输入。

当前核心报告有 10,008 个语义检查点、1,518 个维护检查点，其中部分同时核对语义清理和实际维护结果，两个数字不可相加作动作数。进入元数据细化后的 object 插入、显式物理视图释放、foreign 拒绝和 epoch wrap 各有专门断言。没有把这些动作声称为完整模型状态等价。

## 公开镜像与旧验证

`tests/lifecycle_contract_public.rs` 在正常库编译下镜像 F01/F02/F03/F04/F05/F15，仅比较公共可观察投影。HostValue 根替代稳定私有槽；Function 通过真正 maker VM 创建，不添加无所有权的标量捕获，因此可对应私有模型的 scalar-only 成员。公开投影不声称看见全部内部 ordinary/exec/queue/metadata；闭合存储只输出 closed。其余十个动作族依赖内部预留、强制借用或 epoch 注入，不虚构公共表达方式。

六个冻结旧回放驱动分别复跑，并把日志写到本批目录；旧驱动、输入和收据不改写。历史 465 项 Python 在 2BI detached 副本及其递归固定基线运行，当前 32 项校验器测试在工作树运行；另复跑冻结配额模型 39 项及 112 行算术。2AW 语料结果只核验冻结哈希并复用，未重跑原生语料。

`baseline-inputs.json` 固定 1,048 个旧文件；1,043 个逐字节冻结，README/roadmap 只加前缀，三个既有 Rust 文件允许本批改动。变更模块中的 92 个旧测试函数体保持，851 个旧测试名称逐项核验。只新增三个 Rust 文件。包元数据、公开类型和导出、准备门禁及所有旧模型均冻结。收据同时绑定新增文档、代码、观察数据和日志的 SHA-256；负向测试检查漏行、乱序、身份/状态/计数污染、维护结果、缺失测试、失败命令及收据篡改。
