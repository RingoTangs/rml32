# 2AL 方法、来源和复现

采用 reverse-engineer-anything 技能。既有会话报告为未打开；对选定 magic_Linux32 显式请求Ghidra的open_binary返回 `MCP tool call requires approval, but approval policy is never`。未重复调用、未修复配置或请求提权；未创建原生会话，无需close。此次没有新Ghidra反编译、CFG、pcode或REA Evidence ID。

## 两类证据

1. `reused.json.gz` 保存13条完整旧REA记录；`reused-manifest.json`逐条指出原压缩包、包SHA、规范记录SHA及原Evidence ID。旧container/reference记录部分仅含原始伪代码/汇编，不把它们升级为完整dossier；6条control-flow记录为analyze_function。raw_result、normalized_result、provider/profile/limitations原样保留。
2. `native.json.gz` 保存21个函数的本地Apple LLVM17.0.0静态反汇编命令、stdout/stderr/退出码、ELF符号地址/大小/section、文件偏移及机器码。`local_sha256_*`只表示除ID外规范记录的SHA。每个范围来自ELF符号，未声称恢复CFG；校验器逐指令核对地址连续性、完整范围和原ELF字节。哈希表由ELF section文件映射提取，记录256字节及使用地址；不是执行观测。

规范JSON采用UTF-8、sort_keys、紧凑分隔符和ensure_ascii=False；gzip mtime=0。function-manifest合并34条记录，索引记录摘要和压缩来源摘要；functions.md提供阅读索引。受保护输入清单冻结工作开始时除README/路线图外的503件tracked文件，含36件参考制品、Rust源码/测试和全部旧证据；不更新历史阶段哈希。

## 问题账本与限制

findings.json有14条结论，区分observation/static-inference/design/unknown以及answered/partial/bounded/unresolved。expected-cases.json有43条人工整理的静态推导、Rust设计或未解案例。hash向量使用制品表与汇编递推，排序/修改案例来自链操作和游标算法；不是目标运行输出。mapping_model.py是仅支持Int/普通Bytes、attrib=1和成功分配的离线演算模型，不是参考VM或未来Rust实现；测试冻结预期，而非自动把模型输出当作oracle。

已经覆盖选定哈希、建表、插入、扩容、删除摘链和循环输出；没有恢复整个容器/GC。删除后被引用值槽/节点回收链再利用、交叉别名、所有属性、原生OOM与错误非局部退出仍部分/未解。LLVM观测也不证明外部allocator实现、运行可达或全部函数调用链。没有执行参考ELF、脚本、原始init或业务字节码。初始化制品中的哈希表字节不等于运行时表未被修改的动态证明。

## 复现命令

```sh
python3 -B docs/stage2al-evidence/verify_evidence.py
python3 -B docs/stage2al-evidence/analyze_corpus.py
python3 -B docs/stage2al-evidence/capture_native.py
python3 -B -m unittest discover -s docs/stage2al-evidence -p 'test_*.py'
python3 -B docs/stage2al-evidence/verify_validation.py
cargo test --locked --offline
cargo test --release --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
```

前三项与交付完整性检查默认只读；capture只启动本机objdump，不启动目标，需要同版本Apple LLVM。跨宿主可只用verify_evidence核对保留机器码/来源，不要求安装Apple LLVM。两个生成器仅显式`--write`更新本阶段对应生成物。verify_validation只校验保存的命令退出码、完整日志摘要及交付文件摘要，不替代重跑测试；本阶段变更后旧交付校验应失败，不自动刷新。

28项Python测试包含9项语料测试、9项证据完整性测试和10项静态演算测试。完整性测试覆盖错误目标/偏移/机器码、重算摘要后的错误地址、缺失指令、哈希表错配、丢失/重复函数、旧来源摘要、函数身份和非法证据链接/路径。语料方法单列于corpus-method.md。加载器和制品未变，未重跑PAK解压比较；DBA已知异常边界不变。
