# 2AN 方法与复现

采用[reverse-engineer-anything技能](/Users/ringo/.agents/skills/reverse-engineer-anything/SKILL.md)的证据复用流程。先检查REA会话未打开，打开原始magic_Linux32并核对目标哈希，再获取选定函数dossier；完成后close_binary成功。不修改分析元数据或执行目标。

完整inline Evidence分别保存为rea.json.gz（3条本轮会话记录）和reused.json.gz（17条历史记录）。vm_callInEfun本轮ID与旧记录相同，不计为独立新增证据；vm_callInUserFunBody已有旧伪代码，本轮补完整dossier；vm_finalizeFun本轮补取编译表安装路径。raw_result、normalized_result、provider、profile及limitations完整保留。来源manifest逐条保存历史压缩包与规范记录摘要。所有gzip mtime=0，规范摘要采用UTF-8／sort_keys／紧凑JSON／ensure_ascii=False。

function-manifest将20条记录与ELF符号地址、大小、文件偏移及原机器码摘要关联；anchors保存19处选定指令及其字节。验证器核对当前原ELF、来源原文、记录清单、符号身份、锚点和结论链接，不自动证明所有语义推导、CFG或完整生命周期。没有本地反汇编替代记录，也没有伪造新REA ID。

protected-inputs冻结开工时536个既有文件，排除README／路线图两个可更新入口。当前Rust和全部历史阶段记录逐文件校验；不刷新历史哈希。2AM等历史全包校验固定旧README，不能用于验证新增进度入口；本轮另设交付receipt。

## 语料方法

analyze_corpus.py独立遍历主语料结构，不调用2AF算法。验证原始signed操作数及Magic559身份，对所有05／17计数，对Global17输出逐位置数据、上下文／槽／捕获分类及每类首个代表片段。和独立compatibility-summary核对程序／函数／指令／opcode总数，和2AF完整位置逐条比较Global17的身份、成员偏移、哈希和操作数。

作用域、数字槽名称和相邻指令只是导航；不建立动态调用图，不推断捕获值tag，不把Global/K静态出现当作真实efun已实现或已执行。新扫描依赖历史结构解码，故这不是第二套PAK解码器。原制品／加载器未修改，本批不重复PAK解压比较；DBA已知异常不变。

## 命令与验证记录

```sh
python3 -B docs/stage2an-evidence/verify_evidence.py
python3 -B docs/stage2an-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2an-evidence -p 'test_*.py'
cargo test --locked --offline
cargo test --release --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2an-evidence/verify_validation.py
```

默认命令只读。统计仅显式`analyze_corpus.py --write`重生成2AN统计；`verify_validation.py --write`重跑完整检查并冻结本轮日志、交付文件摘要和receipt。receipt默认核对已保存结果与当前文件，另重查证据／保护输入和语料；不能把旧成功日志当作本次重跑结果。

23项Python测试覆盖原始flags、绝对槽、上下文、成员偏移、重复输入、非法ABI／操作数／注册表、保持总数但篡改位置，以及伪目标／过程地址、重算摘要后的错误身份、原记录缺失、来源变更、机器码锚点、路径逃逸和悬空结论引用。30个expected-cases为静态推导／Rust设计／未解案例，不是原版运行oracle。

Rust基线为515项，debug和release均须通过；本批不增加Rust测试或运行能力。原生任意重编译／卸载、非标量move/copy、owner延迟删除、hook／异步及完整GC仍未知；两批实施规格采取明确受限策略。阶段2未完成。
