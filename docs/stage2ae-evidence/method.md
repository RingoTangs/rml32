# 阶段 2AE 方法与复现

本阶段实现2AD同步06规格；原生依据继续引用已冻结的2AD证据，未新增原生分析、未执行参考ELF/脚本/业务字节码。新公共接口和限定语义见[运行时报告](../vm-efun-runtime.md)。所有VM执行来自内存中合成并经真实格式解析的程序或内部合成帧。

矩阵审计器从2AC独立延续，新增host.rs源码哈希，重新核对全部主语料和现有Rust测试名称，固定73种支持、71种交集、05/17缺失。不将静态06出现次数或312个被引用efun槽称为已执行/已实现。--write只刷新本阶段matrix.json、matrix.md与summary.json。

```sh
cargo build --locked --offline
cargo test --locked --offline
cargo test --locked --offline --release
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2ad-evidence/verify_evidence.py
python3 -B docs/stage2ad-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2ad-evidence -p 'test_*.py'
python3 -B docs/stage2ae-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2ae-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2ae-corpus-validation.json
python3 -B docs/stage2ae-evidence/verify_validation.py
```

全量只读语料比较继续保留dba/pack_data/etc.pak/etc的已知LZSS长度错误，新增错误必须失败。2AD整包验证器固定其当时Rust/README哈希，不能用于验证2AE；仅单独运行其证据、统计及工具单元测试，不改旧验证记录。2AC和更早矩阵同样是历史源码快照，不覆盖刷新。

stage2ae-validation.json记录实际命令结果、当前源码/交付哈希及实施前历史/原始制品哈希；verify_validation核对完整性、当前矩阵和全包比较边界，不会替代重新执行Rust与Python测试。构建日志和临时报告保存在target，交付只保留小型验证摘要。
