# Rust 离线加载器：阶段 1

阶段 1 交付 `rml32` 库和 `rml32-inspect` 命令行工具，直接读取既有 PAK / Magic559 `.o`，不执行原生代码或字节码。使用 `serde` / `serde_json` 序列化，使用 `sha2` 校验；没有自写 JSON 编码器或 SHA-256 算法。

## 接口与数据

```rust
PakArchive::parse(bytes) -> Result<PakArchive<'_>, ParseError>
PakArchive::read_member(path) -> Result<Vec<u8>, ParseError>
parse_program(bytes, ProgramMode) -> Result<ProgramImage, ParseError>
validate_inheritance(&[ProgramDescriptor]) -> ValidationReport
```

`PakArchive` 借用输入包，成员内容按需解压；目录和条目按原始路径字节排序。`read_member` 支持开头的 `/`，以 ASCII 大小写无关方式查找路径；大小写冲突报歧义，不静默选择。非 ASCII 字节保持不变，不推定 Unicode 折叠规则。

`ProgramImage` 保存对象／全局模式、未知头字段、原始记录、字符串字节、节点、metadata、runtime 和成员内偏移。旧指针字段仅保存，不转为宿主指针。`ByteString` 的 JSON 表示为 `{"bytes_hex":"..."}`，不会因展示编码而改变原始字节。

`ProgramMode::Auto` 分别尝试两种布局，要求唯一成功且消费至 EOF。`parse_program_with_profile` 可显式选择 ABI；`LegacyAuth389` 返回不支持。提供的两个旧认证制品通过精确哈希识别，不能仅用共享魔数判断任意历史 ABI。

继承验证以各自包为边界，将父路径 `.c` 映射为 `.o`，核对 constitution、函数和变量计数。它是静态一致性检查，不建立运行时对象或证明动态加载顺序。

## 命令行约定

```sh
rml32-inspect pak <package.pak>
rml32-inspect program <file.o|package.pak/member.o> --mode auto
rml32-inspect program <file.o|package.pak/member.o> --full
rml32-inspect corpus --root <repository-directory>
```

`program` 默认输出摘要，`--full` 输出完整结构。`pak` 检查所有文件内容；`corpus` 按编译时嵌入的输入清单扫描 12 包、校验包哈希、解析程序并输出继承检查。每次只保留当前包、当前程序及轻量继承描述，不积累全部解压内容。

标准输出为 JSON Lines，每行含 `kind` / `data`；类别为 `package`、`member`、`program`、`inheritance`、`error` 和 `summary`。诊断写入标准错误。批量检查遇到成员错误继续扫描；结构无法解析的包不继续解释其目录。退出码为 0 成功、1 输入或检查失败、2 命令用法错误。

## 格式检查策略

- PAK 签名严格检查是离线工具策略；原 `fm_readCatalog` 没有显式执行该检查。
- 检查目录／内容边界、循环目录、非法名称、原始存储长度和解压长度。原始存储按 `original_bytes` 读取，不误用压缩长度。
- 主格式要求 559 项 efun；保留动作表中的合法未使用指令，但拒绝无效动作和表外编号。
- 检查 metadata 的符号扩展偏移、共享字符串和 efun 索引、runtime 数量、名称池边界及初始化 finish。
- 计数字段限于 0..1,000,000，是独立工具限制；不是旧驱动执行契约。解析歧义、完整性变化及未支持 ABI 分别报告。
- 未确认字段原样保留；不执行分支、不证明调用栈、别名、调度或动态 API 行为。

## 验收与复现

```sh
cargo build --offline --locked
cargo test --offline --locked
cargo fmt --check
cargo clippy --offline --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage1-validation.json
```

比较器只读取既有 gzip / CSV 基线，启动新写的 `rml32-inspect`，逐记录核对所有已有程序字段、节点、metadata、runtime、成员哈希及继承计数；Rust 新增原始字段允许保留。文本字段按原 Python 的 UTF-8／GB18030 展示规则归一化，解析库仍保存字节。已有证据不会重生成或覆盖。

完整静态验收目标：12 包、9,527 项、8,921 份可恢复内容、7,159 个主程序、101,592 条函数记录、6,474,773 条指令和 9,933 条继承匹配。唯一已知异常是 `dba/pack_data/etc.pak/etc`；直接扫描返回 1，比较器仅在错误集合和全部结构均匹配时通过。

测试覆盖截断／尾随数据、真实布局歧义、函数记录、未使用合法指令、非法索引、LZSS 重叠与环形边界、目录循环、路径冲突及命令行错误。完整结果见 [阶段 1 静态验收记录](stage1-static-validation.json)。36 个原始制品及启动文件须在检查前后保持基线哈希不变。

原版运行对照依赖 `glibc.i686`、`libstdc++.i686` 和 `/lib/ld-linux.so.2`；本阶段不安装或调用原版环境。后续工作是值／对象模型和解释器，当前工具不能作为可运行服务端使用。
