# rml32

对 `magic_Linux32` 进行静态分析，并以 Rust 重写兼容驱动。当前已实现**只读离线加载器**和阶段 2L 的**数值、共享容器、引用参数、受限默认/可变参数及继承链接 VM 库**，支持 55 种操作码的限定路径。VM 仅在人工构造程序上验证，尚无可运行服务端；`rml32-inspect` 继续只读检查制品。

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
- [服务包与 Rust 兼容性分析](docs/magic_Linux32-rust-compatibility-analysis.md)
- [Ghidra 静态分析](docs/magic_Linux32-static-analysis.md)

实现位于 `src/`，测试位于 `tests/`。原始 ELF、PAK、`.o` 和启动脚本保持只读；命令行工具不会执行目标代码或字节码，VM 测试仅执行内存中合成的程序。旧 `LegacyAuth389` 格式暂不支持。

原版 `magic_Linux32` 的运行对照需要兼容的 32 位 Linux 环境、`/lib/ld-linux.so.2`、`glibc.i686` 和 `libstdc++.i686`。这些是原版运行依赖，离线 Rust 工具不需要安装它们；本阶段不运行原版。
