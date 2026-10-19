# 阶段 2AD 复现与交付边界

本轮用reverse-engineer-anything技能及REA/Ghidra12.1.4分析原始ELF。规划时未发现可用REA；实施时重新枚举工具，确认MCP可用，binary_session显示无已打开目标且Ghidra就绪，随后open_binary固定原ELF身份。没有安装CLI、改客户端配置或运行目标。

先复用2AA的vm_callInEfun与既有解释器记录，再新增vm_returnFromEfun；沿callee和名称查询找到参数/栈辅助函数，新增vm_getArgument、vm_stackInThisContext、vm_popNStack、vm_clearStack与abs/time两个不同栈处理样例。vm_checkArgNum作为用户函数对比，防止把0d入口语义误套在06上。两次名称查询的完整结果保留在rea.json.gz，搜索边界与结果不构成“不存在其他辅助函数”的证明。

会话导出target/stage2ad-rea.json后确定性gzip为rea.json.gz，10条完整新记录不删减raw_result、normalized_result、pcode或限制。reused.json.gz保存4个历史包的10条完整记录，并列原包路径、哈希与Evidence ID。function-manifest.json逐项记录原出处、操作、地址及限制；functions.md提取伪代码/汇编及名称搜索作为可读视图，完整dossier仍在压缩包。REA会话已关闭，临时导出和日志放在target，不属于交付。原始ELF、PAK、对象和历史证据保持原样。

证据验证检查目标/provider、精确函数/操作清单、搜索参数、来源原文与哈希、manifest一一对应、地址/体范围、06动作表、选定汇编站点和结论引用。测试通过损坏临时副本确认检查可检出问题。检查器证明完整性和指定指令存在，不自动证明每句话的语义推导；语义依据及未知范围仍需人工审阅报告与结论账本。

```sh
python3 -B docs/stage2ad-evidence/verify_evidence.py
python3 -B docs/stage2ad-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2ad-evidence -p 'test_*.py'
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2ac-evidence/verify_completion.py
python3 -B tests/compare_corpus.py --report target/stage2ad-corpus-validation.json
python3 -B docs/stage2ad-evidence/verify_validation.py
```

统计命令默认只读，方法见corpus-method.md。全包比较必须继续报告dba/pack_data/etc.pak/etc已知LZSS长度错误，新增异常不得忽略。当前2AC矩阵可只读复算，因为Rust及矩阵依赖未变；历史整包验证器含其当时README等哈希，不替换旧记录来适应本轮。

阶段2ad-validation.json记录命令结果、输入/历史快照哈希、交付哈希和限制。verify_validation只校验这些固定记录与文件完整性，不会替你重跑测试；重跑上面命令才是当前环境的验证。Rust仍为2AC的407项测试/72种限定编号，本轮Python测试属于取证与统计工具。新增接口仅存在于host-runtime-design.md，06仍未实现，真实efun、引用参数、异步恢复、hook/JIT、Global init、bootstrap和真实启动不在交付内。
