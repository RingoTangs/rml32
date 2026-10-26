# 2AY：生命周期定向取证与门槛重评

本批以 2AX 提交为冻结基线，新增 8 条完整 Ghidra 静态记录，复用 67 条记录，形成 75 条记录／59 个函数的证据库存。保留历史 180 个锚点、40 条结论、71 个案例原文，新增 93 个锚点、14 条结论和 23 个案例；全部 71 项历史案例有保留依据的映射。本批只取证和重评，不修改运行时、公共 API、版本、依赖或独立状态模型。

## E01：分支可达性与最终释放分开验收

已补齐普通 PCB 初始化、动作表间接分派及入栈动作的原始依据。给定显式 256 槽 entry、有效 Global 目标、普通解释器、无 hook/JIT 干预且分配成功，256 次 `3b(0)` 可使 sp 从 256 降到零；下一条零捕获 `17(g,0x80)` 可以抵达创建结果的栈溢出分支。这是有条件的有限静态路径，不是主语料／编译器实际产出该程序或原程序执行的证明。

函数头 H、零捕获存储 A 和可选 creator 普通 ref 的取得、结果发布位置、allocator 扩展页及自由链均已分别追踪。新 allocator 例程只给出显式指针的块校验、统计、回池或 libc free，不构成 Function/creator 的语义恢复关系。未发布 H/A 的恢复根及最终释放仍 unresolved，不据局部缺少 free 宣称全局泄漏或 GC 回收。详见 [E01 路径](e01-paths.md) 与 [F41—F47](findings.md#f41)。

## E02：新增条件化排空接点

新记录确认 clearEnvironment 和 shutdownScheduler 在 eligible PCB 删除扫描之后有明确对象队列 drain。正常完成前置 helper/hook 并抵达这些接点时，扫描期间最终 PCB 清理产生且仍在队列中的对象可以进入该次排空。

这些结论不保证外部 PCB 引用最终归零，也不保证主循环停止自动调用 shutdown。下一普通执行片段必须实际完成 `doActions`，之后才到下一 backend drain；活动 PCB 指针非空本身不构成进展保证。单次调度、主循环返回和 responding 回调路径分别记录。shutdown 的后继静态槽注销先清 type，再调用析构例程，不能误写成按原载荷 tag 再次清理。详见 [E02 退出域矩阵](e02-paths.md) 与 [F48—F54](findings.md#f48)。

## 门槛与验证

Q01/Q04/Q05/Q06 的父问题继续 partial；本轮只关闭有明确前提的子域。T05-L、T05-F、T05-O 均保持生命周期契约阻塞，下一批仍为 `T05-lifecycle-contract-evidence`，具体剩余指针、调用边和退出条件见 [门槛重评](gate-review.md) 与 [后续任务](next-batch.md)。

证据和生成视图由 [总账](findings.md)、[问题子域](questions.md)、[锚点](anchors.md)、[案例映射](case-map.md) 与 [函数清单](functions.md) 相互关联。验证覆盖原始字节、734 个冻结文件、实际本批交付与负例、既有 108 项 Python 测试、Rust debug/release 各 644 项普通测试和 2 项 doctest，以及 build、fmt、严格 clippy。正式执行结果以 [2AY 收据](../stage2ay-validation.json) 为准，复现见 [method.md](method.md)。

没有执行原 ELF 或业务字节码。完整采集会话已关闭；语料结构结果复用冻结的 2AW 记录，未重跑全 PAK 检查。`container_capture_ready=true`、`direct_owner_capture_ready=false`、`non_scalar_ready=false`、`stage2_complete=false` 保持不变。
