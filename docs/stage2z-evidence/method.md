# 阶段2Z方法与复现

本轮实现10的普通函数直接槽引用检查，原生观察、Rust策略及剩余限制见[运行时报告](../vm-reference-type-check-runtime.md)。使用 reverse-engineer-anything 技能，目标SHA-256固定为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

REA/Ghidra12.1.4新会话查询 vm_stackInThisContext 的dossier及 vm_checkIsRefType 的汇编，导出并确定性压缩为rea.json.gz（2记录），随后关闭会话。reused.json.gz保留4个历史包的5条完整记录及原包哈希；function-manifest.json建立入口、操作、来源、体范围和限制的一一索引，functions.md提供可读摘录。没有运行目标或启动脚本。

verify_evidence.py验证目标/provider、记录唯一性、来源哈希和原文一致性、清单、动作表10绑定、11个选定指令及上下文bp读取。guard/tag/push的文本顺序检查只核对留存汇编，不独立证明控制流语义或动态行为。合成损坏测试仅修改临时副本。

语料扫描及位置定义见[统计方法](corpus-method.md)。默认只读校验；--write显式更新本目录统计生成物。matrix审计规则从2Y独立延续到2Z，重新核对完整语料、支持集合、限定分支、真实测试名称及证据路径；--write只更新本目录matrix/summary生成物。历史阶段文件不改写。

```sh
cargo test --locked --offline
cargo test --locked --offline --release
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2z-evidence/verify_evidence.py
python3 -B docs/stage2z-evidence/analyze_corpus.py
python3 -B docs/stage2z-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2z-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2z-corpus-validation.json
python3 -B docs/stage2z-evidence/verify_validation.py
```

阶段验证保存本批修改前的依赖、原始制品、历史文件及未改源码哈希，修改及新增交付另列哈希。旧阶段整包验证器要求当时源码/README快照，不用于校验当前源码；保留证据的独立验证仍有效。
