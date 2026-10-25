# 2AX：生命周期证据收尾与 T05 门控规格

本批补齐既有 8 条新增、57 条复用静态证据的锚点、结论、案例、问题子域及历史案例映射，并形成 [T05 门控规格](runtime-design.md) 和 [后续任务](next-batch.md)。Rust 运行时、公共 API、版本及依赖保持 2AW 基线；直接 Object/Function 捕获继续关闭，阶段 2 未完成。

## 结论与边界

- creator 普通引用、被捕获 Object 载荷引用、活动帧 execRef 是独立持有关系；待删 `0x4000` 与失效 `0x8000` 不可合并。Function 最后持有者释放、COPY 的真实源规范化及延迟删除分别记录。
- 普通解释线程无 catch 的恢复、backend 和 PCB 清理可以在限定路径上串联；hook 干预仍排除。`endForBackend` 在 `deleteProcess` 之前，不能据此声称最终 PCB 清理新增的待删对象在同一轮已经排空。PCB ref 尚未归零时最终槽/帧释放延后。
- 合法非空捕获已腾出结果槽；末尾结果栈溢出的候选是初始 `sp=0,argn=0`。零长度分配可以成功，但未发布零捕获函数头的恢复根及最终释放仍无证据闭合。
- 任意 hook/重入、其他线程/JIT、捕获槽 Reference 逃逸、外部 mark 根和完整循环回收继续未知或排除。普通线程的局部结论不关闭 Q05/Q06 父问题。

逐项依据见 [结论总账](findings.md)、[问题子域](questions.md)、[机器码锚点](anchors.md) 和 [函数清单](functions.md)。原 2AR 的 51 个案例保持原文，重评结果通过 [案例映射](case-map.md) 关联新增案例，不改写历史证据或收据。静态推导和 Rust 政策均不作为原程序运行观测。

## 验证与交付

实际交付校验覆盖总账、原始字节、历史来源、全部新增记录的结论/锚点接点，以及确定性生成文档的一致性。保留既有 40 项规则/收据测试，并新增实际交付完整路径和破坏性输入负例。Rust 原有 644 项普通测试与 2 项 doctest 的场景身份保持，完整验证包含 debug/release、build、fmt、严格 clippy 和冻结的 56 项模型测试。

验证详情与实际统计以 [2AX 收据](../stage2ax-validation.json) 为准，复现方法见 [method.md](method.md)。本批复用经过哈希核验的 2AW 语料结构结果，未重跑全 PAK 检查。未执行参考程序或业务字节码；收尾也未重复采集 REA 证据。

`container_capture_ready=true` 继续仅表示 2AW 的受限无环容器捕获能力；`direct_owner_capture_ready=false`、`non_scalar_ready=false`、`stage2_complete=false`。下一批 `T05-lifecycle-contract-evidence` 先处理未发布头、hook/最终释放与下一安全点等证据门槛，不能直接扩大捕获白名单。
