# 2AZ：未发布分配追踪与外部引用分类

本批新增 9 条完整函数 dossier 和 5 条数据交叉引用记录，复用 76 条函数记录，合计 90 条 Evidence／69 个不同函数。新增 78 个锚点、12 条结论和 21 个案例；原 273A/54F/94C 完整保留，当前为 351A/66F/115C，全部 94 项历史案例映射保持依据。

## R01 的实质进展

H 确实保存在 C 栈槽，A 有寄存器及 H 字段副本；未发布与未保存不是同一判断。新增 catch 记录给出实际 topEnv/topCatch 写入，但没有建立 H/A 恢复登记。成功 xrefs 找到扩展页链的五个精确读写接点；allocator shutdown 可以条件化释放扩展页物理存储，仍不能替代 Function／creator 的语义清理，也不能自动覆盖独立 fallback 分配。

这些结果形成 [R01 路径](r01-paths.md)、[六行逐分配处置表](allocation-disposition.json) 和 [数据引用台账](data-references.json)。R01 的剩余前沿是错误后的发现／读取关系及其到 free／creator unref 的连接；局部未见该关系不等于全局泄漏。

## R02 的分类边界

六个已解析 unref 调用者实际含三条 CALL、四条尾 JMP、两个地址取得接点；provider 的七条 UNCONDITIONAL_CALL 元数据保留原样，与真实 mnemonic 分开。新记录还定位局部 retain、投递、失败回退及 callback 退出邻接。

`_vmInits` 的 scheduler 指针和 `vm_shutdown` 的条件化倒序索引消费者已关联。三处精确地址查询为空不排除计算索引访问；文件中的初始 initFlag 也不是运行时状态。分类详见 [r02-classification.md](r02-classification.md)，不递归展开任意 callback 或保证外部持有者最终退出。

## 范围与验收

本批无 Rust 运行时、API、版本、依赖或状态模型变更，未执行参考程序或业务字节码，分析会话已关闭。首次数据查询超时已进入请求日志，没有作为阴性证据；之后的成功结果与完整原始记录分别保留。

验证保护 765 个基线文件，保留全部 143 项既有 Python 测试及 Rust debug/release 各 644 项普通测试和 2 项 doctest，并检查新增数据／证据／处置／收据负例、build、fmt 和严格 clippy。正式结果以 [2AZ 收据](../stage2az-validation.json) 为准，复现见 [method.md](method.md)。语料结构结果明确复用冻结 2AW 记录，未重跑全 PAK。

Q01/Q04/Q05/Q06 父域继续 partial，直接捕获仍关闭，`stage2_complete=false`。门槛和下一步见 [gate-review.md](gate-review.md) 与 [next-batch.md](next-batch.md)；本批不推进 D01。
