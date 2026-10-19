# 阶段 2Y 复现与验证

本阶段仅实现已有2X证据支持的普通函数 Bytes/Array 范围查询。语义及差异见[运行时报告](../vm-range-runtime.md)，原生证据保持在[2X证据目录](../vm-range-evidence/method.md)，无新增原生查询。

verify_completion.py 与 test_verify_completion.py 从2W审计规则独立延续到2Y；当前矩阵只更新范围查询及相关类型限制表述，不修改历史文件。默认校验模式只读；--write只重建本目录matrix.json、matrix.md、summary.json。逐项核对动作表、原始统计、当前支持集合、限定能力、现存测试名称与证据路径；编号数量不是功能完成率，也不是分支覆盖率。

```sh
cargo test --locked --offline
cargo test --locked --offline --release
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/vm-range-evidence/verify_evidence.py
python3 -B docs/vm-range-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/vm-range-evidence -p 'test_*.py'
python3 -B docs/stage2y-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2y-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2y-corpus-validation.json
python3 -B docs/stage2y-evidence/verify_validation.py
```

阶段验证记录保存本批修改前的原始制品、依赖、历史文件及未改源码哈希，交付文件另列哈希。历史2X/2W整包验证器含当时源码和README哈希，不要求这些旧快照继续匹配当前实现。原始ELF、PAK、对象、脚本均保持不变。
