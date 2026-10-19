# 阶段 2X 证据方法与复现

目标、结论、后续实现决策和覆盖限制见 [语义报告](../vm-range-semantics.md)。使用 reverse-engineer-anything 技能，目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

新 REA 会话以 Ghidra 12.1.4 打开原始 ELF，仅调用 analyze_function 获取 vm_queryRange、vm_allocateEmptyArray、vm_pushAllocedString、vm_pushString；export_evidence_bundle 导出后确定性压缩为 rea.json.gz，会话已关闭。没有执行目标或启动脚本。

reused.json.gz 留存四个既有包的九条完整记录，sources 逐项保留原包路径、哈希和 Evidence IDs。function-manifest.json 对新旧十三条记录建立函数入口、操作、来源、函数体及限制索引。functions.md 是伪代码、汇编和限制的可读摘录，完整 raw_result 与 CFG 仍在压缩包中。

证据校验核对目标字节、provider、记录/清单一一对应、原包哈希与逐记录原文、入口、函数体及十五个关键指令位置。该校验证明留存数据一致和选定指令存在，不替代人工语义分析。记录中的终端尾调用、类型恢复限制和未知项必须与结论一起读取。

从仓库根目录运行：

```sh
python3 -B docs/vm-range-evidence/verify_evidence.py
python3 -B docs/vm-range-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/vm-range-evidence -p 'test_*.py'
python3 -B docs/vm-range-evidence/verify_validation.py
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
```

完整语料比较器仅启动新 Rust 检查工具：

```sh
python3 -B tests/compare_corpus.py --report target/stage2x-corpus-validation.json
```

统计默认只读核对；只有显式 --write 才更新本目录统计生成物。见 [语料方法](corpus-method.md)。原生记录不可通过重算脚本改写。阶段验证核对本批交付与受保护文件哈希；旧阶段 README/路线图快照哈希已过时，不运行要求当前文档仍匹配旧快照的整包验证。
