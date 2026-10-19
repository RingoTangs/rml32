# 阶段 2AB 方法与复现

本阶段直接实现阶段 2AA 已冻结的第一批设计，不新增原生会话或原始证据。原生事实及其限制见[全局语义](../vm-global-semantics.md)，Rust 公共接口、寿命与失败策略见[运行时报告](../vm-global-runtime.md)。只执行合成 Rust 测试和只读解析器，不执行参考 ELF 或业务字节码。

新审计器由阶段 2Z 的审计规则独立延续，核对完整静态语料、当前支持集合、每行限定语义、真实测试名及证据路径。35/36 的 init 统计不代表初始化执行支持；全局 07/06 仍拒绝。历史矩阵保留原样。

```sh
cargo build --locked --offline
cargo test --locked --offline
cargo test --locked --offline --release
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2aa-evidence/verify_evidence.py
python3 -B docs/stage2aa-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2aa-evidence -p 'test_*.py'
python3 -B docs/stage2ab-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2ab-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2ab-corpus-validation.json
python3 -B docs/stage2ab-evidence/verify_validation.py
```

`verify_completion.py --write` 仅重建本阶段 matrix/summary。语料比较必须明确报告既有 `dba/pack_data/etc.pak/etc` LZSS 长度异常；只有该异常时比较器通过，不能忽略新增错误。验证记录固定本阶段交付、源码及未修改历史/依赖/原始制品哈希。
