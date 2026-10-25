# rml32 · 当前阶段 2AX

阶段2AX已补齐既有8条新增、57条复用原生静态记录的证据总账、51项历史案例映射与[T05门控规格](docs/stage2ax-evidence/runtime-design.md)。Rust运行时、公共API、版本及依赖保持2AW基线；原644项普通测试与2项doctest保持。见[本批说明](docs/stage2ax-evidence/overview.md)、[复现方法](docs/stage2ax-evidence/method.md)和[验证收据](docs/stage2ax-validation.json)。受限容器捕获能力保持；直接Object/Function捕获仍阻塞，阶段2未完成。下一批为[T05-lifecycle-contract-evidence](docs/stage2ax-evidence/next-batch.md)，先补未发布头与最终清理后的安全点等生命周期证据。未执行参考程序或业务字节码，语料结果复用冻结2AW记录。以下原文保留为历史快照。

# rml32

阶段2AW已完成 T04 显式受限容器捕获：新增 Object/Global 准备入口，支持无环 Array/Mapping 捕获及私有实际类型 profile 守卫；旧入口仍只接受标量捕获。原626项场景与断言保留，API 仅增补入口，版本与依赖不变。受限容器能力就绪，直接 Object/Function 捕获仍阻塞，阶段2未完成。见[实现说明](docs/stage2aw-evidence/overview.md)与[验证收据](docs/stage2aw-validation.json)。下一批为 T05 前置原生契约取证。以下为历史快照。


阶段2AV已完成[T03拟提交图验证](docs/stage2av-evidence/overview.md)：遍历Function捕获边，按最终边拒环并校验共享DAG、存活祖先和资源限额；公共API、版本及依赖不变。保留原606项场景与全部原断言，Host失败原子性及快照契约不变。验证见[2AV收据](docs/stage2av-validation.json)。非标量捕获仍关闭，阶段2未完成；下一批为T04显式受限容器捕获入口。以下为历史快照。


阶段2AU已完成[T02稳定捕获槽与COPY事务](docs/stage2au-evidence/overview.md)：真实源槽规范化、语义Owner唯一/共享展开、raw返回与完成槽转移；公共API、版本及依赖不变。原577项场景保留，仅按冻结证据调整两处预期。Host回调失败仍保持操作数与PC，副作用不回滚、不重放。验证见[新收据](docs/stage2au-validation.json)。非标量捕获仍关闭，阶段2未完成；下一批为T03投影图验证。以下为历史快照。


阶段2AT已完成[宿主/观察边界与语义所有者迁移](docs/stage2at-evidence/overview.md)：公共 API 源码不兼容，改用 ValueInput、不可 Clone 的 HostValue、冻结快照与借用回调。原558项断言保留，新增19项普通测试与2项文档编译负例；577项普通测试及2项doctest通过。版本和依赖不变，非标量捕获仍关闭，阶段2未完成。下一批为2AS T02稳定捕获槽与源COPY计划。验证见[2AT收据](docs/stage2at-validation.json)。以下为历史快照。


阶段2AS已完成[所有权与源复制设计及独立模型](docs/stage2as-design/overview.md)：15项决策、16项API迁移、51个2AR案例映射；三个有界枚举域合计21,837个域内去重状态、61,365次成功转换。模型不构成原生或未来Rust实现的等价证明。运行时/API/依赖与558项Rust测试不变，非标量捕获仍关闭；下一批为[2AT宿主/观察边界与语义计数迁移](docs/stage2as-design/implementation-plan.md)，验证见[收据](docs/stage2as-validation.json)。以下为历史快照。

阶段2AR已完成复制源规范化与非标量捕获所有权定向取证，见[语义报告](docs/vm-capture-ownership-semantics.md)、[结论与机器码锚点](docs/stage2ar-evidence/findings.md)、[后续Rust规格](docs/stage2ar-evidence/runtime-design.md)及[验证记录](docs/stage2ar-validation.json)。新增3条、复用50条原生记录，形成51个静态/设计案例；Rust/API/依赖与558项测试不变，非标量捕获仍关闭。下一批2AS先闭合源复制、语义持有者及Function捕获边图验证设计；阶段2仍未完成。以下为历史快照。

阶段2AQ已按2AP基线更新全量能力审计，Rust/API/依赖与558项Rust测试、75种限定操作码/73种主语料交集保持不变。见[当前完成度](docs/stage2-completion-assessment-2aq.md)、[当前矩阵](docs/stage2aq-evidence/matrix.md)与[验证记录](docs/stage2aq-validation.json)。24项稳定缺口及13类能力门槛已重核，阶段2仍未完成；下一批为[2AR复制源规范化与非标量捕获所有权取证](docs/stage2aq-evidence/next-batch.md)，不直接开放非标量捕获。以下为历史快照。

阶段2AP已实现受限同步Host/K函数值。组合准备入口同时启用普通Host调用、对象/Global函数值、Global/c和Host/K；函数值只保存Host槽号，每次05调用读取当前注册与签名。回调调用者上下文、单次调用及提交边界均有合成回归。见[Host/K运行时说明](docs/vm-host-function-value-runtime.md)、[能力增量](docs/stage2ap-evidence/matrix.md)及[验证记录](docs/stage2ap-validation.json)。**FunctionValue::owner()/program()现返回Option，属于源代码不兼容迁移**。真实efun、异步/重入、非标量捕获、完整生命周期与阶段2仍未完成。以下为历史快照。

阶段2AN已完成Global／efun函数值定向静态取证及[两批Rust实施规格](docs/stage2an-evidence/function-runtime-design.md)，见[语义报告](docs/vm-global-function-value-semantics.md)、[方法与边界](docs/stage2an-evidence/method.md)及[验证记录](docs/stage2an-validation.json)。本轮会话3条完整dossier（其中1条为同ID刷新）、复用17条历史记录；独立复算5,012处Global创建位置，与2AF逐条一致，提供30个静态／设计案例。515项Rust测试debug/release、23项Python检查及fmt/clippy通过。运行时/API/依赖和75种限定支持/73种交集不变；Global／efun函数值尚未实现，非标量捕获与阶段2仍未完成。以下为历史快照。

阶段2AM已实现普通函数受限Mapping997，包含Int/Bytes原生哈希桶链、实时遍历顺序与三元输出数组别名复用。见[运行时说明](docs/vm-mapping-runtime.md)、[能力增量](docs/stage2am-evidence/matrix.md)及[验证记录](docs/stage2am-validation.json)。515项Rust测试debug/release、19项Python模型/语料测试及fmt/clippy通过；公开API/依赖不变，75种限定支持/73种交集不变。其他键/属性、删除后引用、复杂交叉图、完整GC与生命周期循环仍排除，G05部分实现，阶段2未完成。以下为历史快照。

阶段2AL已完成Int/Bytes Mapping哈希、桶链顺序及遍历修改的静态取证与[后续实施规格](docs/stage2al-evidence/mapping-runtime-design.md)，见[语义报告](docs/vm-mapping-iteration-semantics.md)和[验证记录](docs/stage2al-validation.json)。复用13条REA记录、本地LLVM补查21个函数，复算298,369处相关位置；28项Python检查及487项Rust测试debug/release通过，fmt/clippy通过。运行时/API/依赖不变，Mapping997仍未开放，75种限定支持/73种交集不变；G05与阶段2未完成。新REA会话被客户端审批策略拦截，证据来源与剩余别名/删除引用限制已明确记录。以下为历史快照。

阶段2AK已补齐受限 Bytes `+=`（Bytes/Int右值），支持现有可写引用槽并保留表达式结果；487项Rust测试在debug/release通过，fmt及clippy通过。见[本轮语义、证据和验证](docs/vm-bytes-compound-runtime.md)。操作码集合仍为75种；Real追加、其他Bytes复合运算及生命周期新分支仍排除，阶段2未完成。以下为历史快照。

阶段2AJ已实现普通函数Bytes+Bytes/Int拼接、全长相等及首NUL前缀排序，482项Rust测试在debug/release通过。支持集合仍为75种、主语料交集73种。见[运行时说明](docs/vm-bytes-runtime.md)、[能力增量矩阵](docs/stage2aj-evidence/matrix.md)和[验证记录](docs/stage2aj-validation.json)。Real拼接、Buffer/Bytes写入、生命周期新分支及真实服务仍排除，阶段2未完成。以下为历史快照。

阶段2AI已完成Bytes比较/拼接定向静态取证：新增12条、复用15条完整记录，复算125,747处指令并提供56条静态/设计期望及[首批Rust实施规格](docs/stage2ai-evidence/bytes-runtime-design.md)。见[语义报告](docs/vm-bytes-semantics.md)与[验证记录](docs/stage2ai-validation.json)。本批未实现Bytes运算；473项Rust测试、75种限定支持/73种语料交集保持不变，阶段2仍未完成。下一步按规格实施普通函数受限Bytes运算。以下保留历史状态。

阶段2AH完成当前能力重审：75种限定支持、73种主语料交集与473项Rust测试保持不变。独立审计登记24项缺口（阶段2内19项、后续阶段5项），所有当前门槛仍未关闭；下一批优先进行Bytes比较/拼接定向取证。见[完成度报告](docs/stage2-completion-assessment-2ah.md)、[当前矩阵](docs/stage2ah-evidence/matrix.md)、[下一批规格](docs/stage2ah-evidence/next-batch.md)与[验证记录](docs/stage2ah-validation.json)。本批不修改运行时/API/依赖、不新增反编译、不执行原程序。以下条目保留历史状态。

阶段2AG已实现受限Object函数值与 `05/17`：显式准备入口、标量捕获、身份比较、跨Object/Global调用者及失败原子性。473项Rust测试通过，支持75种限定操作码，主语料73种编号均有交集。见[函数值运行时](docs/vm-function-value-runtime.md)、[当前矩阵](docs/stage2ag-evidence/matrix.md)和[验证记录](docs/stage2ag-validation.json)。非标量捕获、Global/efun/string/OS目标及真实服务仍未实现，阶段2尚未完成。以下条目保留历史状态。

阶段2AF已完成函数值 `05/17` 的静态取证、35,703处位置统计及[首批Object函数值/标量捕获规格](docs/stage2af-evidence/function-runtime-design.md)。确认05参数数目包含目标，17捕获参数前置；见[函数值语义](docs/vm-function-value-semantics.md)与[验证记录](docs/stage2af-validation.json)。本轮Rust/API/依赖不变，仍为2AE的433项Rust测试、73种限定操作码，05/17尚未实现。

阶段2AE已实现显式注册的受限同步 `06` 宿主调用，433项Rust测试通过。新增宿主签名/上下文/返回接口及Object/Global准备入口；支持73种限定操作码，主语料仍缺05/17。见[宿主调用运行时](docs/vm-efun-runtime.md)、[当前矩阵](docs/stage2ae-evidence/matrix.md)和[验证记录](docs/stage2ae-validation.json)。真实efun、引用参数、异步/重入、Global init及服务启动仍未实现；以下条目保留历史状态。

阶段2AD已完成 `06 / vm_callInEfun` 的静态取证、143,520处调用统计及[受限同步宿主接口规格](docs/stage2ad-evidence/host-runtime-design.md)。固定559槽中312槽被静态调用；参数顺序、原生上下文继承、返回与清理依据见[宿主分派语义](docs/vm-efun-semantics.md)。本轮未修改Rust运行时、公共API或依赖，仍为2AC的72种限定操作码、407项Rust测试，06尚未实现；合成宿主、真实efun及服务启动不能由静态统计代替。

阶段2AC已实现显式全局程序准备/绑定、全局位07及双上下文帧，407项Rust测试通过。全局代码使用独立代码池且无当前对象，41返回Int(0)，支持通过08调用对象方法并恢复上下文；暂停重绑保留活动帧旧代码。见[全局调用运行时](docs/vm-global-call-runtime.md)、[当前矩阵](docs/stage2ac-evidence/matrix.md)和[验证记录](docs/stage2ac-validation.json)。仍为72种限定操作码；06、Global init、预置bootstrap和真实服务启动尚未实现。以下条目保留历史状态。

阶段2AB已实现显式共享全局表与 `35/36`，支持72种操作码的限定路径，384项Rust测试通过。新增 `GlobalBankSchema`、`prepare_program_with_globals`、全局表安装/移除/快照接口和公开枚举变体 `ReferenceKind::Global`。旧引用在移除后保留旧槽，重装不别名。见[全局运行时](docs/vm-global-runtime.md)和[当前矩阵](docs/stage2ab-evidence/matrix.md)。全局程序加载、全局07/06、真实启动、继承及生命周期内全局访问仍排除；依赖不变。

阶段2AA完成了[全局语义取证](docs/vm-global-semantics.md)与[两批实施规格](docs/stage2aa-evidence/global-runtime-design.md)，其交付保留历史快照。第一批现由阶段2AB实现，第二批全局调用与帧上下文迁移尚未实施。

阶段2Z已实现 `10` 的普通函数直接槽引用检查，共70种限定操作码、359项Rust测试通过。检查原始引用形态，不解引用或改变引用配额；`11`、生命周期及默认表达式仍排除。见[引用类型检查运行时](docs/vm-reference-type-check-runtime.md)与[当前矩阵](docs/stage2z-evidence/matrix.md)。

阶段2Y已实现 `45` 的普通函数 Bytes/Array 只读范围查询，支持闭区间、尾部计数、NUL补零及数组源槽规范化，共69种限定操作码、344项Rust测试通过。见[范围查询运行时](docs/vm-range-runtime.md)和[当前矩阵](docs/stage2y-evidence/matrix.md)。Buffer、范围写入、生命周期及默认表达式仍排除。

阶段2X已完成范围查询 `45` 的静态取证、3,452处语料统计及后续受限 Bytes/Array 实现规格，见[范围查询语义与设计](docs/vm-range-semantics.md)。本轮运行时、公共API、依赖与68种限定操作码支持集合保持不变，`45`尚未实现。

对 `magic_Linux32` 进行静态分析，并以 Rust 重写兼容驱动。当前已实现**只读离线加载器**和阶段 2AJ 的**Bytes运算、数值、共享容器、引用参数及类型检查、受限默认/可变参数、继承链接、初始化、构造回滚及受限跨对象调用、switch、整数/数组循环、范围查询、显式全局表与全局函数调用、同步宿主调用、受限Object函数值及受限 catch VM 库**，支持 75 种操作码的限定路径。VM 仅在人工构造程序上验证，尚无可运行服务端；`rml32-inspect` 继续只读检查制品。

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

- [阶段 2AH 完成度与独立能力审计](docs/stage2-completion-assessment-2ah.md)
- [阶段 2AH 方法与复现](docs/stage2ah-evidence/method.md)

- [阶段 2AG Object函数值运行时](docs/vm-function-value-runtime.md)
- [阶段 2AG 当前能力矩阵与复现](docs/stage2ag-evidence/method.md)

- [阶段 2AF 函数值创建、调用与寿命取证](docs/vm-function-value-semantics.md)
- [阶段 2AF 复现方法与首批实施设计](docs/stage2af-evidence/method.md)

- [阶段 2AE 受限同步宿主调用](docs/vm-efun-runtime.md)
- [阶段 2AE 当前能力矩阵与复现](docs/stage2ae-evidence/method.md)

- [阶段 2AD 宿主分派语义与同步设计](docs/vm-efun-semantics.md)
- [阶段 2AD 证据、统计及验证复现](docs/stage2ad-evidence/method.md)

- [阶段 2AA 全局存储与调用静态语义](docs/vm-global-semantics.md)
- [后续两批全局运行时实施规格](docs/stage2aa-evidence/global-runtime-design.md)

- [阶段 2Z 直接槽引用类型检查](docs/vm-reference-type-check-runtime.md)

- [阶段 2Y 受限范围查询运行时](docs/vm-range-runtime.md)

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
