# 后续：非测试生产接入

D08 的设计与独立模型已经给出配额和维护选择；它们尚未实现，`implementation_ready=false`。下一批应先审查 [迁移表](migration.json) 与实际 Rust 内存布局，再按下列顺序实现非测试接入。这里列出后续范围，不在 2BH 放宽任何门禁。

1. 建立 origin 所属的不可变 `LifecycleLimits`、checked 账本与 prepared reservation，保持 legacy 构造行为；证明内部租约没有 arena control 强回边。明确稳定索引、物理 view 与弱登记的分配及销毁责任。
2. 将实际 Cell、Function/容器 owner、CopyPlan、Object 记录及 VM 帧接入峰值预留；生产失败测试覆盖源/目标版本变化、creator 失效、配额交接、取消/abort 和回调只执行一次。
3. 让对象创建在发布前保留队列节点和普通引用单位；证明最后执行租约与 task Drop 的入队不需分配/失败。保持零 exec remove 同步清理与托管失败统一入队的区别。
4. 实现 store/VM 显式维护入口、分预算工作单位、可恢复扫描游标及 epoch wrap；真实借用和工作区冲突必须返回已完成进度，零预算不能隐式退休或排空。转移、close、Host 保活和最终 teardown 需要非测试集成回归。
5. 在非 `cfg(test)` 构建中验证 opt-in 行为，并执行公共 API 编译合同、32/64 位算术、真实 Rust 配额边界、旧行为回归与六层冻结回放。只有这些实现证据齐备才能重新评估 production implementation readiness。

采用显式小配额的生产测试，不把测试适配层的 `usize::MAX` 当默认配置。若实现调整 D08 的配额、Host raw/COPY 或维护原子边界，先记录可评审的契约修订及对应反例。

这一接入仍不能解决二十项原生未知。未发布 H/A 最终清理、完整 scheduler/shutdown 进展、直接 Object/Function 捕获、escaping References、一般 GC、真实 efun、业务析构、异步/重入、线程及 JIT 的开放各需独立证据和授权范围；不得由模型或测试接点成功自动推出。
