# 2BH：生产生命周期契约 D08

本批完成设计与独立配额模型，承接 [2BG 的生产决策清单](../stage2bg-host-function-test/next-batch.md)。未来生产入口采用显式 `LifecycleLimits`、峰值预留和按工作单位计费的 `maintain_lifecycle`；Host 传输保留 raw 语义，脚本 COPY 继续执行源槽规范化。当前 Rust 运行时、公共 API、准备权限、版本、依赖及旧模型均未修改。

| 交付 | 内容 |
|---|---|
| [生产契约](contract.md) / [机器契约](api-contract.json) | 五项显式配额、四个未来类型、构造与维护入口、预留及失败边界 |
| [维护协议](maintenance.md) | Host 根退休、LIFO 对象队列、增量元数据扫描、部分成功及阻塞进度 |
| [决策表](decision-table.json) / [迁移表](migration.json) | 十项决策及十一个文件的未来接入责任；全部迁移仍为 `implemented=false` |
| [独立模型](quota_model.py) / [检查结果](contract-checks.json) | 39 个行为案例；32/64 位合计 112 项独立算术检查 |
| [验证方法](method.md) / [验证收据](../stage2bh-validation.json) | 当前检查、固定提交历史检查及六层旧回放分别记录 |
| [后续接入任务](next-batch.md) | 将本契约接入非测试 Rust 前必须完成的实现与验收 |

基线为 2BG 提交 `5827a6387fc2fbaa142a428133316c446713c230`，共 980 个文件。README 与路线图仅增加前缀，其余 978 个文件逐字节冻结，包含参考二进制、全部 Rust 源码、测试、旧证据及旧验证收据。新模型独立标记为 `new_quota_contract_model_added=true`；`existing_models_changed=false`。

本批复跑每个配置的 819 项普通 Rust 测试和 2 项 doctest，另行执行六个 ignored 回放驱动；63 项当前 Python 检查及固定基线副本上的 392 项历史检查通过。历史检查不声明对当前源码生效，2AW 语料报告仅按冻结哈希复用。执行命令及输出见收据和 [压缩日志](validation-logs.json.gz)。

这些结果支持 D08 的有限模型契约和既有 Rust 路径回归，不构成未来生产实现的内存安全、分配失败恢复、原生等价或最终清理进展证明。`implementation_ready=false`、`stage2_complete=false`；二十项原生未知保持，直接 Object/Function 捕获、escaping capture References、一般 GC、真实 efun、业务析构、异步/重入、其他线程及 JIT 继续受限。已有受限容器捕获能力保持原状，不能把本批 `non_scalar_ready=false` 理解为撤销该能力。完整门槛见 [gate-review.json](gate-review.json)。
