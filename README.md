# rml32

阶段2X已完成范围查询 `45` 的静态取证、3,452处语料统计及后续受限 Bytes/Array 实现规格，见[范围查询语义与设计](docs/vm-range-semantics.md)。本轮运行时、公共API、依赖与68种限定操作码支持集合保持不变，`45`尚未实现。

对 `magic_Linux32` 进行静态分析，并以 Rust 重写兼容驱动。当前已实现**只读离线加载器**和阶段 2W 的**数值、共享容器、引用参数、受限默认/可变参数、继承链接、初始化、构造回滚及受限跨对象调用、switch、整数/数组循环及受限 catch VM 库**，支持 68 种操作码的限定路径。VM 仅在人工构造程序上验证，尚无可运行服务端；`rml32-inspect` 继续只读检查制品。

阶段2W已扩展实数/混合除零、整数余数零及相应复合赋值捕获，323项Rust测试通过。见[数值错误捕获](docs/vm-numeric-catch-runtime.md)和[当前矩阵](docs/stage2w-evidence/matrix.md)。仍为68种限定操作码；生命周期内catch、其他错误来源及回调/JIT保持限制。

阶段2V已实现普通函数的2c/1a/0e及整数除零捕获，303项Rust测试通过。见[catch运行时](docs/vm-catch-runtime.md)和[2V历史矩阵](docs/stage2v-evidence/matrix.md)。主语料交集66种、缺7种；其他错误来源、生命周期内catch及回调/JIT仍未实现，编号比例不是功能完成率。

阶段2T已实现隐藏控制槽和23/12/50的整数±1闭区间、Array遍历，278项Rust测试通过。见[循环运行时](docs/vm-loop-runtime.md)与[2T历史矩阵](docs/stage2t-evidence/matrix.md)。主语料73种中交集63种、缺10种；Mapping循环及完整异常/生命周期仍未实现，编号比例不是功能完成率。2P—2S报告和证据保留历史快照。

阶段2U已完成通用catch静态取证、只读语料统计及Rust栈展开设计，见[异常捕获报告](docs/vm-catch-semantics.md)。确认2c/1a/0e各3,266次及解释模式目标公式，首批实现规格限定整数除零；该轮运行时、依赖和65种操作码支持表不变，catch当时尚未实现。

## 构建与检查

需要 Rust / Cargo；当前验证环境为 Rust 1.99.0。依赖为 `serde`、`serde_json`、`sha2`，版本由 `Cargo.lock` 固定。

```sh
cargo fetch
cargo build --locked
cargo test --locked
cargo test --locked --release
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

依赖已下载时可添加 `--offline`。工具按宿主架构构建，可在 macOS 上读取 Linux 制品。

```sh
# 检查包成员及内容哈希；输出 JSON Lines
cargo run --locked --bin rml32-inspect -- pak aaa/pack_data/etc.pak

# 直接检查包内程序，不需要解包到磁盘
cargo run --locked --bin rml32-inspect -- program \
  aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o --mode global

# 全量基线比较，只启动新写的检查工具，不运行参考程序
python3 tests/compare_corpus.py --report target/stage1-validation.json
```

比较器核对既有压缩证据中的成员、完整程序结构及继承记录。已知 `dba/pack_data/etc.pak/etc` 解压异常必须报告；直接执行 `corpus --root .` 因此返回 1，而比较器确认只有该已知异常时返回 0。新增错误不能被忽略。

## 文档与边界

- [阶段 2X 范围查询静态语义与实现设计](docs/vm-range-semantics.md)

- [阶段 2W 数值错误捕获扩展](docs/vm-numeric-catch-runtime.md)

- [阶段 2V 受限 catch 运行时](docs/vm-catch-runtime.md)

- [阶段 2U catch 静态语义与 Rust 设计](docs/vm-catch-semantics.md)
- [阶段 2T 整数与数组循环运行时](docs/vm-loop-runtime.md)
- [阶段 2S switch 运行时](docs/vm-switch-runtime.md)
- [阶段 2R 控制流静态语义与实施设计](docs/vm-control-flow-semantics.md)
- [阶段 2Q 跨对象调用运行时](docs/vm-cross-object-runtime.md)
- [阶段 2P 历史完成度评估](docs/stage2-completion-assessment.md)
- [重写路线图](docs/magic_Linux32-rust-rewrite-roadmap.md)
- [阶段 1 接口、格式策略及验收](docs/rust-offline-loader.md)
- [阶段 2A VM 语义、证据和接口](docs/vm-core-semantics.md)
- [阶段 2B 数值语义与验证](docs/vm-numeric-semantics.md)
- [阶段 2C 不可变容器语义与验证](docs/vm-container-semantics.md)
- [阶段 2D 引用静态分析与设计](docs/vm-reference-semantics.md)
- [阶段 2E 受限引用写入与接口迁移](docs/vm-reference-runtime.md)
- [阶段 2F 数值自增减与复合赋值](docs/vm-numeric-updates.md)
- [阶段 2G 引用参数静态证据与设计](docs/vm-reference-arguments.md)
- [阶段 2H 固定函数引用参数实现与限制](docs/vm-reference-arguments-runtime.md)
- [阶段 2I 默认与可变参数静态分析](docs/vm-optional-arguments.md)
- [阶段 2J 受限默认与可变参数实现](docs/vm-optional-arguments-runtime.md)
- [阶段 2K 继承静态证据与设计](docs/vm-inheritance-linking.md)
- [阶段 2L 受限继承链接与执行](docs/vm-inheritance-runtime.md)
- [阶段 2M 对象生命周期静态分析与设计](docs/vm-object-lifecycle.md)
- [阶段 2N 受限对象初始化运行时](docs/vm-initialization-runtime.md)
- [阶段 2O 受限构造任务与失败回滚](docs/vm-construction-runtime.md)
- [服务包与 Rust 兼容性分析](docs/magic_Linux32-rust-compatibility-analysis.md)
- [Ghidra 静态分析](docs/magic_Linux32-static-analysis.md)

实现位于 `src/`，测试位于 `tests/`。原始 ELF、PAK、`.o` 和启动脚本保持只读；命令行工具不会执行目标代码或字节码，VM 测试仅执行内存中合成的程序。旧 `LegacyAuth389` 格式暂不支持。

原版 `magic_Linux32` 的运行对照需要兼容的 32 位 Linux 环境、`/lib/ld-linux.so.2`、`glibc.i686` 和 `libstdc++.i686`。这些是原版运行依赖，离线 Rust 工具不需要安装它们；本阶段不运行原版。
