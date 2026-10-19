# 阶段2AA证据、统计和交付复现

本轮只做静态取证与设计，当前运行时仍为2Z。结论与未知项见[全局语义报告](../vm-global-semantics.md)，后续两批API/寿命/上下文及验收规格见[实施设计](global-runtime-design.md)。

使用reverse-engineer-anything与REA/Ghidra12.1.4，目标SHA-256固定为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新会话取得7条函数dossier、36汇编1条，以及global名称搜索、全局表xrefs和5个包含函数定位，共15条新记录。先按最小搜索边界发现表引用，再选择分配/销毁函数跟踪；并未把含global名称的buffer/sqlite等函数当作全局变量语义证据。

导出后保持每条完整记录原文，将8条函数记录确定性压缩至rea.json.gz，7条导航记录至navigation.json.gz；后者保留发现路径及查询边界，不算独立语义证明。会话已关闭，没有运行参考ELF。reused.json.gz保留6个历史来源包的14条完整函数记录与原包哈希；function-manifest.json、navigation-manifest.json分别索引，functions.md保留可读伪代码/汇编及限制。Ghidra误类型、原生无边界索引、完整热更新/关闭及未知bootstrap绑定不隐藏。

证据验证检查目标/provider、记录唯一性、清单一一对应、来源原文与哈希、路径边界、35/36/07/06动作表绑定、19个选定指令和导航查询参数。验证结果说明完整性与指定指令存在，不独立证明全部调用/生命周期语义。22项证据测试只修改临时副本。

语料默认只读核对，--write才刷新本目录生成统计；方法见[语料说明](corpus-method.md)。7项合成统计测试覆盖边界/标志/歧义等输入。银行/全局调用运行时尚未实现，不将这些Python测试称为全局执行测试。

```sh
python3 -B docs/stage2aa-evidence/verify_evidence.py
python3 -B docs/stage2aa-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2aa-evidence -p 'test_*.py'
python3 -B docs/stage2z-evidence/verify_completion.py
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B tests/compare_corpus.py --report target/stage2aa-corpus-validation.json
python3 -B docs/stage2aa-evidence/verify_validation.py
```

当前2Z矩阵可以直接只读重算，因为Rust和矩阵依赖未变；2Z整包交付校验含旧README哈希，不用于验证本轮新文档。2AA交付记录独立保存本轮开始前的原始制品、Rust、测试、Cargo及历史证据哈希，README/路线图的本轮更新另列交付哈希。
