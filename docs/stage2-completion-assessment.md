# 阶段 2P：完成度评估与下一批实现设计

## 一、总体结论

**阶段 2 尚未完成，下一批先实现受限跨对象调用。** 本轮交付审计、可复算操作码矩阵和 REA/Ghidra 静态证据，不修改 Rust 运行时。现有实现仍为 57 种操作码的限定路径、212 项 Rust 测试；尚无可运行服务端，也未证明与原版行为等价。

主语料包含 7,159 个程序、101,592 个函数、6,474,773 条指令、73 种操作码。支持集合与语料交集为 55 种，另有仅合成测试覆盖的 4d/4e；语料中缺少实现的有 18 种。**这些是指令编号统计，不是功能完成率或实际执行覆盖率。** 已支持编号也可能只支持部分类型、模式、标志和程序结构。

- [逐指令矩阵](stage2-completion-evidence/matrix.md)：支持边界、排除路径、静态出现次数、测试、证据、依赖及退出条件。
- [能力独立审计](stage2-completion-evidence/capability-review.md)：按原阶段 2 目标核对值、对象、解释器和异常模型。
- [跨对象调用证据与实现设计](stage2-completion-evidence/cross-object-design.md)：下一批接口、语义边界及验收场景。
- [机器验证记录](stage2p-validation.json)。历史阶段文档保留其当时结论；本页为统一现状。

## 二、未完成范围

| 分组 | 缺失指令 | 实现重点 |
|---|---|---|
| 调用与函数值 | 05、06、08、17、41 | 跨对象上下文、方法解析、返回hook、间接调用及宿主契约 |
| 循环 | 12、23、50 | 游标、条件、步进、暂停与异常清理 |
| switch | 0a、0b、0c | 数值/字符串匹配、默认分支及跳转边界 |
| catch | 0e、1a、2c | 嵌套捕获、非局部退出、完整帧/引用/临时值清理 |
| 全局 | 35、36 | 全局值槽、引用寿命、全局调用上下文 |
| 引用与范围 | 10、45 | 引用类型判定、范围边界、读写及别名 |

此外，buffer 和函数值尚无完整 Value 表示；循环容器、原生 clone、主动析构、活动对象延迟删除、完整名称查找/private、一般默认表达式、继承共同祖先和返回hook仍有缺口。构造失败回滚不是通用catch，ObjectStore::remove不是完整destruct，链接时排除private候选不是完整访问控制。各项具体限制见独立审计及逐指令矩阵。

## 三、实施顺序与阶段边界

1. **跨对象调用**：先以显式方法绑定、直接 Object 目标及受限标志实现08/41；依据保留的反汇编恢复参数与返回上下文。字符串查找、数组目标、尾调用等未闭合路径明确拒绝。再扩展方法表、函数值和05。
2. **控制流**：循环、switch及其暂停/错误清理；语义闭合后再扩展默认表达式白名单。
3. **值与范围**：buffer、范围读写、容器类型和循环共享/释放，登记Rust实现与原生内存模型的差异。
4. **异常与对象生命周期**：通用catch、跨对象异常解栈、主动析构、重入和延迟删除；复用已验证的构造回滚，但不混淆两者契约。
5. **全局与宿主**：在合成全局银行上补全VM语义，明确06的分派、参数、返回和错误契约。阶段3再接入实际efun、真实bootstrap注册、调度和I/O。

阶段2退出门槛是主语料73种指令的实际实现、关键类型/标志分支及组合清理行为均有原生依据和合成回归，核心未知项已解除或经明确范围调整。不能用支持编号数、测试总数或受控拒绝路径宣布完成。阶段3的559项实际efun与服务启动依赖不能由VM核心测试代替。

## 四、验证与复现

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage2p-corpus-validation.json
python3 docs/stage2-completion-evidence/verify_completion.py
python3 -m unittest discover -s docs/stage2-completion-evidence -p 'test_*.py'
python3 docs/stage2-completion-evidence/verify_evidence.py
```

矩阵生成方式见[方法说明](stage2-completion-evidence/method.md)。全量只读检查覆盖12包、9,527成员，唯一已知错误为`dba/pack_data/etc.pak/etc`的LZSS长度不符；检查通过表示与保留结构基线一致，不代表程序执行成功。

参考ELF、PAK、对象、脚本和旧证据均保持原样。本轮仅启动Rust合成测试、只读解析工具及Ghidra分析引擎，没有执行原始ELF、启动脚本、原始初始化代码或业务字节码。原版的`glibc.i686`、`libstdc++.i686`依赖不改变此静态分析边界。
