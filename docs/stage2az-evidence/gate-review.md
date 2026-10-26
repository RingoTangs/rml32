# 2AZ 生命周期门槛重评

本页区分静态接点闭合与完整生命周期契约，不是运行时能力声明。

| 子题 | 本轮结论 | 仍保留的边界 |
|---|---|---|
| H/A 保存与发布 | F55：H 的 C 栈副本、A 的寄存器／头字段、可选 creator ref 及结果发布位置有机器码。 | 保存副本未自动成为恢复根；不外推寄存器跨调用或 longjmp 保留。 |
| catch 元数据 | F56：newCatch 的 funCp/catchLevel/topCatch 写入和 backend 建立顺序已定位。 | 不穷尽导入恢复实现或任意外部根，不据此证明所有可恢复副本不存在。 |
| 页链与物理处置 | F57—F59：五个精确访问与正常 shutdown 的页 free 已确认；pool 与 fallback 分开。 | 存储释放的发生仍有前提；不能把 flat/waste/stable 与独立 fallback 混同，也不替代 creator unref。 |
| 语义恢复 | F60：发现 H/A→相应 free→适用 creator unref 的完整连接未闭合。 | R01 unresolved；六行处置表保留物理与语义状态，不宣称全局泄漏。 |
| R02 接点分类 | F61—F65：CALL/JMP/DATA、相邻持有、表注册与索引消费者、wait wrapper 已分类。 | 注册不是执行；raw kind 不是实际 mnemonic；局部退出不是最终进展证明。 |
| R02 完整进展 | F66：callback 执行、外部持有退出、停止后顺序和 PCB 存储有效性未形成全域契约。 | R02 partial；本批不递归展开回调或把调试 DATA 当运行时根。 |

T05-L 仍为 `blocked-native-contract`：本轮页链物理 free 和 catch 元数据不能替代未发布分配的语义恢复契约。T05-F 与 T05-O 分别依赖适用的生命周期基础，均未开放；旧入口、稳定槽、唯一 move／共享 COPY、最终图、Host 单次副作用、origin 与观察边界保持原政策。

本批没有将模型、Rust 发布前预检或合成测试用作原生未知的证明。若后续拟把原生失败清理差异作为明确的兼容边界，必须先形成范围和行为决策；2AZ 不自动采纳该取舍，也不冻结 D01 的内部表示。

能力标志保持 `container_capture_ready=true`、`direct_owner_capture_ready=false`、`non_scalar_ready=false`、`stage2_complete=false`。门槛重评已完成与能力就绪分别验收，后续前沿见 [next-batch.md](next-batch.md)。
