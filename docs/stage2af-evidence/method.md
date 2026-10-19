# 阶段 2AF 方法与复现

本轮仅取证、统计和实施设计，不改Rust、Cargo、公共API或支持集合。原生语义与问题状态见[报告](../vm-function-value-semantics.md)和findings.json；首批边界、确切API及验收见function-runtime-design.md。

使用reverse-engineer-anything及REA/Ghidra12.1.4直接打开原magic_Linux32。以已保留的17记录为入口，查询Ex、05、freeFunctionValue，沿共享复制/清理链补取copyValue、cleanupValue和pushValue；为身份比较补取eqValue及callee isEq。两个同名bind_parameters入口实际为SQL绑定，作为有界排除线索完整保留。没有运行原ELF或任何业务字节码，也没有修改反汇编元数据。

新增10条完整analyze_function记录导出至target/stage2af-rea.json后确定性压缩为rea.json.gz；保留全部raw_result、normalized_result、pcode及限制，不把提取的伪代码/汇编视图当作完整记录。复用包reused.json.gz保留5个历史来源的11条原文记录和来源哈希，包含17、解释器、用户调用/返回、efun调用、真值和catch清理。部分copy/cleanup查询与历史包有同名函数，目的是本轮直接检查tag9分支；不把重复观察计为独立行为证明。REA会话已关闭，原始制品保持不变。

function-manifest.json一一索引21条记录及地址/出处/限制；functions.md为可读视图。verify_evidence校验目标/provider、精确过程清单、来源原文、manifest双射、函数体、05/17动作表、26个选定汇编站点及结论引用。18项测试只损坏临时副本，确认漏记录、伪地址、改来源/指令等能被检出。校验器不自动证明每条语义推导或完整所有权/调度兼容。

```sh
python3 -B docs/stage2af-evidence/verify_evidence.py
python3 -B docs/stage2af-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2af-evidence -p 'test_*.py'
cargo test --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2ae-evidence/verify_completion.py
python3 -B tests/compare_corpus.py --report target/stage2af-corpus-validation.json
python3 -B docs/stage2af-evidence/verify_validation.py
```

本轮27项Python测试仅为工具检查，Rust仍433项、73种限定支持、主语料交集71种，缺05/17。当前2AE矩阵可直接复算，因为它的源码/审计依赖未改；2AE整包验证器固定当时README，不能通过改写历史哈希来验证本轮文档更新。全包比较必须保留dba/pack_data/etc.pak/etc已知LZSS异常，新增错误不可忽略。

stage2af-validation.json单独保存本轮命令结果、当前交付哈希和实施前历史/源码/制品哈希。verify_validation只检查这些文件和已记录结果，不会代替重跑测试；target日志/完整临时导出不作为交付。完成本轮不代表05/17已实现，也不代表阶段2完成。非标量捕获、global/efun/string/OS目标、完整对象生命周期、循环回收和调度/hook/JIT仍列为后续工作。
