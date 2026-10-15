# 阶段 2U 证据与交付方法

本轮只增加静态取证、只读语料分析和后续 Rust 设计；没有修改运行时、依赖、参考制品或历史证据。使用 `reverse-engineer-anything` 技能，目标为仓库根目录的 `magic_Linux32`，显式选择 REA MCP 的 `provider_id="ghidra"`；记录显示 Ghidra 12.1.4。会话原先关闭，打开后完成查询并关闭。没有执行目标、启动脚本或原始业务字节码。

## 取证与复用

- 新查询：`analyze_function(vm_newThreadCatch)`、`analyze_function(vm_endThreadCatch)`，以及 `procedure_assembly(vm_caughtError)`、`procedure_assembly(vm_doActions)`。前两者补齐此前缺少的处理器证据，后两者交叉核对保留伪代码的压值及错误码恢复路径；没有重跑相同分析。
- `rea.json.gz` 保存这 4 个完整 Evidence，包含原始/规范化结果、provider/profile、函数范围、汇编、交叉引用和 limitations；函数 dossier 的高层值流可能有自身截断，不能据此声称完整数据流证明。
- `reused.json.gz` 保留 18 份记录及 6 个来源文件的 SHA-256、Evidence ID；校验器验证本地记录与原来源逐项相等，不将新结论写回旧证据。
- `function-manifest.json` 和 `functions.md` 索引全部 22 份记录。注册函数没有静态直接 caller，仍有表驱动调度；不能据此推断不可达。
- 所有证据绑定同一主 ELF SHA-256。旧原始输出中的字段恢复和终端调用限制保持不变；报告只对已经检查的解释路径作结论。

## 语料与验证

语料脚本仅用 Python 标准库读取历史解码快照，记录输入哈希，默认复算并比对 JSON。其扫描范围、偏移语义和词法候选限制见 [corpus-method.md](corpus-method.md)。不做业务执行或动态捕获配对。

本阶段报告 [../stage2u-validation.json](../stage2u-validation.json) 保存开始时除 README/路线图以外全部已跟踪文件的哈希，并保存本轮交付哈希。`verify_evidence.py` 重新核对来源、原目标、原始制品/历史/源码、文档链接、清单、语料及不变的支持基线；校验器测试使用独立的坏 provider、坏目标、重复记录、来源变更等合成样本。

Rust 回归继续仅运行合成 VM 和解析/CLI 测试，278 项通过；fmt、Clippy 通过。语料输入与解析源码均未改变，因此未重复全量 PAK 比较器；保留阶段2T的已知 DBA 解压异常记录，不将其隐藏或宣称修复。旧阶段校验包含旧 README/路线图哈希，当前入口使用本轮验证记录，不修改历史快照使其迁就当前文档。

未来测试规格列在 [runtime-design.md](runtime-design.md)；它们不是本轮已经执行的 catch 测试。
