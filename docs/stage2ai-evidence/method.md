# 阶段2AI复现与交付方法

本批仅完成Bytes比较/拼接静态取证与后续Rust实施规格，没有实现运行时分支。目标为原始magic_Linux32，SHA-256见function-manifest.json；没有运行原ELF、启动脚本或包内业务程序。

REA/Ghidra 12.1.4新增12条analyze_function记录，分析配置digest为`d693cf6dff72f11d1dda1237fd566d037c1e4e694b65124902a68186a6afb717`。rea.json.gz保留完整导出记录及raw_result、normalized_result、pcode、body和限制；压缩采用mtime=0。另将历史15条完整记录原样收集到reused.json.gz，reused-manifest.json记录来源文件与记录规范化SHA-256。规范化采用UTF-8、ensure_ascii=False、sort_keys=True、separators=(',', ':')，不附换行。functions.md与reused-functions.md为阅读副本。没有元数据注释写入，会话已关闭。

verify_evidence.py核对目标、函数入口、body、选定汇编位置、配置、27条清单和结论引用；verify_reused.py逐条比对历史完整记录。format-bytes.json单独从ELF32 PT_LOAD映射确认%g及使用位置。静态观察、C ABI推导和Rust设计在findings.json、expected-cases.json中分别标记；56条期望不是原生执行输出。外部libc、原生OOM/溢出及任意intern头等价仍未知。

语料复算方法见corpus-method.md。默认analyze_corpus.py只读校验现有输出，必须显式--write才能再生成两份JSON。导航样例不代表实际操作数类型或动态可达性。

在仓库根目录执行：

```sh
python3 -B docs/stage2ai-evidence/verify_evidence.py
python3 -B docs/stage2ai-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2ai-evidence -v
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B tests/compare_corpus.py --report target/stage2ai-corpus-validation.json
python3 -B docs/stage2ai-evidence/verify_validation.py
```

stage2ai-validation.json冻结本批交付及所有既有跟踪文件（仅README与路线图允许更新）。历史2AH证据与验证文件原样保留；其动态重算依赖当时路线图内容，入口更新后不要覆盖旧输出。新验证器检查其冻结哈希。全量比较器只运行Rust检查工具，保留已知单个解压异常。未重新运行release测试；实际命令结果见新验证记录。
