# 阶段 2AH 方法与复现

本批对2AG当前源码/测试、75行操作码矩阵及2P以来保留文档作独立审计，不修改Rust、公共API、依赖或支持集合，不新增REA/反编译，也不执行参考ELF、脚本和包内业务字节码。实施前2AG整包验证已通过；当前Rust基线仍为473项（52库单元、23解析/CLI、398VM）。

## 数据与人工复核边界

opcode-review.json逐行记录当前实现、排除、关联测试和源码定位，exclusion_links将每项排除精确关联稳定缺口。gaps.json独立覆盖非操作码能力，按rejected/difference/evidence_gap区分当前拒绝/已知差异/证据不足；stage字段按原路线图区分2—6阶段。capabilities.json把所有缺口映射到退出门槛，priorities.json记录工程排序及唯一首选。review-changes.json保留2AG排除描述与2AH描述的对应，历史原文件不改写。

人工复核代表测试的输入、运行入口和断言，发现并修正4a、18、22、43、4f、2e等过宽测试关联；矩阵中的相关测试是场景索引，不能据测试名存在证明覆盖。数值/位运算检查操作数表，引用检查结果/源槽与错误种类，调用/控制流检查上下文、预算及清理断言。所有75行仍为bounded，没有测量逐分支覆盖，没有把拒绝路径计作原生支持。

verify_completion.py沿2AG独立延续，重算完整静态语料统计和支持集合，并调用audit_rules检查源码行/摘录、证据/测试引用、每项排除关联、依赖无环及阶段方向、能力门槛覆盖和唯一排序。源码/证据内容哈希进入summary；缺失输入不再静默跳过。工具只验证结构/出处/一致性，不证明语义推导或断言充分。--write仅重生成matrix.json/md、summary.json和capabilities/gaps/priorities.md。

## 复现

```sh
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2ag-evidence/verify_completion.py
python3 -B docs/stage2ah-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2ah-evidence -p 'test_*.py'
python3 -B tests/compare_corpus.py --report target/stage2ah-corpus-validation.json
python3 -B docs/stage2ah-evidence/verify_validation.py
```

全量比较只核对12包、9,527成员、7,159程序及9,933继承关系，保留唯一已知dba/pack_data/etc.pak/etc的LZSS异常，新增错误失败；36项原始制品哈希必须不变。release测试结果仍是2AG历史记录，本批只重跑debug，不冒称重跑release。

2AH验证记录保存实际命令结果、当前交付哈希、实施前入口文档哈希和全部既有保护文件哈希。更新README/路线图后旧2AG整包验证会因入口文档哈希变化而不适用；2AG矩阵仍可独立复算，不覆盖旧记录。所有新工具测试只破坏临时副本，日志/临时生成器保留于target。完整性检查不替代实际执行测试。
