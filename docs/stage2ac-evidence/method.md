# 阶段 2AC 方法与复现

本阶段实施已冻结的 2AA 第二批规格，扩展 07/41 及内部帧上下文；不新增编号或原生会话。原生观察见[全局语义](../vm-global-semantics.md)，实现与排除范围见[运行时报告](../vm-global-call-runtime.md)。全部 VM 执行来自合成 Object/Global 映像，保留制品仅作只读解析与静态比较。

当前审计器由 2AB 独立延续，校验当前源码支持表、完整静态语料、限定语义、真实测试名及证据路径。72 种限定支持/70 种语料交集保持不变。初始化节点和全局调用位置统计不构成真实启动支持，编号数量不是功能完成率。

```sh
cargo build --locked --offline
cargo test --locked --offline
cargo test --locked --offline --release
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2aa-evidence/verify_evidence.py
python3 -B docs/stage2aa-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2aa-evidence -p 'test_*.py'
python3 -B docs/stage2ac-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2ac-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2ac-corpus-validation.json
python3 -B docs/stage2ac-evidence/verify_validation.py
```

审计器 `--write` 仅重建本阶段 matrix/summary。语料比较必须保留 `dba/pack_data/etc.pak/etc` 的已知 LZSS 长度错误，新增错误不能忽略。新验证记录固定源码、交付、依赖、原始制品和未修改历史文件；2AB 及更早记录保持原样，不将旧源码哈希替换为当前值。
