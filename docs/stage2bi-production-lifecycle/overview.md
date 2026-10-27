# 2BI：D08 受限生命周期生产接入

已将 [2BH D08 契约](../stage2bh-production-contract/contract.md) 接入正常 Rust 库构建：显式五项配额、峰值所有权与执行预留、pending 删除、预分配 LIFO 队列，以及 store/VM 的分预算维护入口。生产 API 为增补，`new` / `with_limits` 继续走 legacy 分支。版本、依赖、准备权限与参考二进制不变。

新增 `ObjectStore::with_bounded_lifecycle(ValueLimits, LifecycleLimits)` 与 `ObjectStore::maintain_lifecycle` / `Vm::maintain_lifecycle`，公开 `LifecycleLimits`、`LifecycleBudget`、`LifecycleProgress`、`LifecycleStop`。Limits 与 Budget 均无默认值。以下配额仅演示显式配置，应用应按自己的负载选择：

```rust
use rml32::{ObjectStore, ValueLimits, LifecycleLimits, LifecycleBudget};
let mut store = ObjectStore::with_bounded_lifecycle(
    ValueLimits::default(),
    LifecycleLimits {
        max_objects: 1024,
        max_function_heads: 4096,
        max_ordinary_refs_per_object: 4096,
        max_exec_refs_per_object: 256,
        max_owners_per_header: 8192,
    },
)?;
let progress = store.maintain_lifecycle(LifecycleBudget {
    host_roots: 16,
    objects: 8,
    metadata_entries: 64,
})?;
```

配额按峰值计入 prospective owner/执行租约，覆盖旧值或退出旧帧不能提前抵扣。raw transfer 移动原 owner，Function 副本不重复 creator；freed 记录和退休 Function 元数据在实际回收前继续占额。Host retain/export/回调保留 raw transport，脚本 COPY 规范化实际源槽。维护保留当前阻塞单位及先前成功进度，零预算不调用会退休根的 `Registry.check`，run/错误/结果提取不新增隐式对象队列 drain。

| 验证 | 结果 |
|---|---|
| 普通 Rust 测试 | debug/release 各 851 项，保留全部原 819 项，新增 32 项 |
| doctest | 每个配置 5 项，其中新增 3 项检查无 Default 及 HostScope 无维护权限 |
| 新实现案例 | 18 项通过公共 API 使用正常库构建；14 项私有借用、故障、版本、控制块及元数据探针 |
| 冻结回放 | Host Function 14/120、managed 13/85、Host 12/75、VM 10/70、adapter 18/146、kernel 14,948 次转换 |
| 独立 D08 规格 | 原 39 个模型案例与 112 行 32/64 位算术检查重新执行；无模型修改 |
| Python 验收 | 当前 10 项；固定提交副本中的历史 455 项，分别记录 |
| 构建与静态检查 | debug/release build、fmt、严格 clippy 通过 |

详细结构见 [实现说明](implementation.md)，文件责任见 [迁移表](migration.json)，证据边界与复现见 [验证方法](method.md)、[机器收据](../stage2bi-validation.json)。基线为 `f99812460c374adb49bdffd42dee84bcc1f86c8c` 的 1017 个文件：仅允许修改 11 个既有 Rust 文件、两个入口前缀，另新增三个 Rust 文件；其他 1004 个文件逐字节冻结。允许修改的 Rust 文件中，原有 156 个测试函数体按 token 核对保持。

本批标记 `production_lifecycle_implemented=true`、`bounded_lifecycle_ready=true`，仅指 D08 的受限 Rust 策略及其验收。通用 `implementation_ready=false`、直接 Object/Function 捕获门禁及 `stage2_complete=false` 保持，二十项原生未知没有提升。已存在的受限容器捕获权限保持，不将历史 `non_scalar_ready=false` 解释为撤销它。配额不是进程字节或墙钟时长上限；这些测试不证明完整分配器恢复、一般 GC、原生未发布 H/A 清理或 scheduler/shutdown 最终进展。
