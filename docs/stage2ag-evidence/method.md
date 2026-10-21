# 阶段 2AG 方法与复现

实现2AF首批Object函数值/标量捕获规格，原生依据复用已冻结2AF文件，无新增REA会话或原始程序执行。接口、语义及差异见[运行时说明](../vm-function-value-runtime.md)。Rust全部执行用内存合成程序或内部合成帧。

矩阵审计器从2AE独立延续，当前支持75种/主语料交集73种，所有行均为bounded。重新核对完整静态语料、Rust支持集合及集成/内部单元测试名称；相关测试不等于每条指令分支覆盖。--write仅刷新本目录matrix.json、matrix.md和summary.json。2AE及以前矩阵是历史快照，不能覆盖刷新。

```sh
cargo build --locked --offline
cargo test --locked --offline
cargo test --locked --offline --release
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2af-evidence/verify_evidence.py
python3 -B docs/stage2af-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2af-evidence -p 'test_*.py'
python3 -B docs/stage2ag-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2ag-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2ag-corpus-validation.json
python3 -B docs/stage2ag-evidence/verify_validation.py
```

全量只读比较覆盖12包、9527成员、7159程序、9933继承关系及36项原始制品哈希；唯一保留异常为dba/pack_data/etc.pak/etc的LZSS长度错误，新增异常必须失败。没有通过VM执行这些程序。

473项Rust测试包含本批40项（11内部故障/寿命测试及29集成测试）。2AF取证工具27项Python测试、2AG矩阵审计18项Python测试各自统计。验证记录保存实际命令、解析结果、实施前源码身份、当前交付哈希与保护文件哈希；完整性验证不替代重跑命令。旧整包验证器固定历史Rust/README哈希，本轮只调用旧证据/统计校验器及其测试，不改写旧验证记录。

Cargo依赖、错误类别、VmLimits、ReferenceKind均未扩展。原ELF、包、脚本、旧证据保持原样。日志和临时文件在target；交付保留小型摘要。
