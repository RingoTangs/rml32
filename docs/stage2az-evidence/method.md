# 2AZ 方法与复现

本批优先调查未发布 H/A 的保存、恢复和释放，R02 仅做相邻持有接点及注册／调用分类。不执行参考程序或业务字节码，不修改 Rust、公共 API、版本、依赖或独立状态模型。目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

## 基线与采集

基线为提交 `19cec02c381912224d6997c68e245fc109e33f52` 的全部 765 个已跟踪文件。README 与路线图仅允许在完整历史内容前追加本批状态；其余源文件、制品、证据、脚本和收据逐字节保护。复用 2AY 的全部 75 条函数记录及主证据包中的 `vm_waitMainLoopDone`，共 76 条；来源和完整记录哈希见 `reused-manifest.json`。

新增函数 dossier 与数据交叉引用分别保存在 `rea.json.gz` 和 `references.json.gz`。两包保留原始导出封套元数据及各自全部完整记录，按 operation 分区；不把数据对象塞入函数清单。摘要的 `new_records/reused_records` 统计函数包记录，`reference_records` 单列 xrefs，`unique_records` 是三者的联合数量，`functions` 仅统计不同函数名。

`session.json` 区分函数与数据查询请求，记录实际参数、结果 ID、失败原因及关闭状态。首次 xrefs 在 330 秒后超时，没有返回 Evidence；随后较小的 `vm_newCatch` 查询成功，会话初始化完成后原地址查询成功。失败不作为“没有引用”，也不计入成功记录。原始导出和操作分区的字节数／哈希分别注明，不能将原始导出哈希误用于过滤后的分区。

## 数据与指令的校验边界

`data-references.json` 固定 ELF OBJECT 符号、段类型、精确地址查询、实际源指令以及模块注册字段。`_pdb_extendGrpList` 位于 `.bss`／SHT_NOBITS，只有虚拟地址和大小，不提供虚构的文件偏移或原始字节；`_vmInits` 位于文件后备 `.data`，逐字节核对整个对象和 scheduler 字段。

xrefs 的 normalized/raw 结果是精确地址列表，本身不含引用种类。READ/WRITE 等元数据另取对应函数 dossier，实际指令／原始字节再独立核对；查询覆盖不等于任意别名、区间访问或计算索引的完整性。三个模块表精确地址查询为空，另一个实际消费者地址有结果；二者与原始索引指令共同保留，不能把空列表升级为表未使用。

函数机器码、inclusive 分段及文件偏移沿用既有校验。provider 把某些 tail JMP 标为 UNCONDITIONAL_CALL 时保留原 kind；R02 的 CALL／JMP 分类以实际指令为依据，不追改完整原始记录。逐分配处置表将取得、保存、发布、恢复、物理释放和语义释放分开；creator 有／无使用明确字段，避免从文字猜测状态。

## 总账与复现

原 2AY 的 273 个锚点、54 条结论、94 个案例作为完整前缀保留；新增结论和案例单列，全部 94 项映射保留原 basis/findings。精确查询范围、未解析边和物理／语义释放边界进入新结论，不把局部缺少 free 写成全局泄漏，也不靠模型关闭 native unknown。

```sh
python3 -B docs/stage2az-evidence/render.py --write
python3 -B docs/stage2az-evidence/render.py
python3 -B docs/stage2az-evidence/verify_evidence.py
python3 -B docs/stage2az-evidence/verify_validation.py --write
python3 -B docs/stage2az-evidence/verify_validation.py
```

无 `--write` 的入口只读核对。正式收据捕获实际 subprocess 日志；模拟夹具仅用于负例测试。验证保留全部 143 项旧 Python 测试以及 Rust debug/release 各 644 项普通测试和 2 项 doctest 的身份，并执行新增证据／数据／收据负例、build、fmt、严格 clippy。新增测试数以实际收据为准。旧 2AY 收据按冻结日志与基线哈希复核，不重写历史交付。

语料结构结果仍复用已核验的 2AW 记录，`rerun=false`，不重跑全 PAK 或宣称执行等价。Ghidra 数据库已关闭，原始 ELF 保持不变。范围与剩余前沿见 [overview.md](overview.md)、[gate-review.md](gate-review.md)、[next-batch.md](next-batch.md)，正式结果见 [2AZ 收据](../stage2az-validation.json)。
