# 阶段 2BJ：D08 生产契约差分审计

在 2BI 固定基线 `0c4fbf40007d0bc2f877a269ba6e1ca22ef6fcc0` 上完成受限生命周期审计，并修复两项可复现偏差：未发布 Function 的 creator 失效后仍可提交；Mapping 循环首次分配输出数组时使用了不同 origin 的旧工厂。公共 API、配额语义、版本、依赖、准备权限和旧模型保持。[偏差与修复](findings.md)给出适用范围及基线复现方法。

核心矩阵取对象/Function 头配额 `{0,1,2}`、ordinary 配额 `{2,3,4}`、exec 配额 `{1,2}`、头 owner 配额 `{1,2,3}`，共 162 个配置；每个配置运行 16 个固定动作族，共 **2,592 条 trace、10,944 个实际检查点**。debug/release 的逐行观察一致。冻结 2BH 模型核对语义状态、计数、槽和队列；新增独立维护参考模型核对实际元数据索引的分预算进展。不存在未分类差异。这是有限动作族的差分结果，不是全状态空间或原生等价证明。

正常 Rust 库另运行六个可公开表达的动作族，共 **972 条 trace、3,468 个检查点**，不依赖 `cfg(test)` 的工厂或观察器。六项新增普通集成测试覆盖多层构造和失败回滚、Global/Host Function、回调后的失败、Mapping 写入与循环、catch 展开、输出复用和峰值失败。[覆盖映射](case-coverage.json)保留此前 115 个静态案例及 20 个 native unknown；39 个模型案例逐项区分差分覆盖、实现细化和 model-only。

最终验证为 debug/release 各 **857 项普通 Rust 测试（原 851 + 新 6）、5 项 doctest**；8 个 ignored 回放驱动分别执行，不能计作普通测试。当前 Python 校验器 32 项、固定版本历史 Python 465 项、冻结配额模型 39 项及 112 项算术检查分别记账。完整结果以[收据](../stage2bj-validation.json)及压缩命令日志为准。

实际宿主为 `aarch64-apple-darwin`；真实 32 位 Rust 执行记为 `not_run`，模型检查的 32/64 位算术不能代替它。本批未运行参考 ELF、未新增原生分析或业务程序；测试中的少量字节码为合成夹具。直接 Object/Function 捕获、`implementation_ready`、`stage2_complete` 继续关闭，详见[门槛记录](gate-review.json)。

复现步骤见[方法](method.md)，后续建议见[下一批](next-batch.md)。
