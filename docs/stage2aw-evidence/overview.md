# 2AW：显式受限容器捕获

T04 已实现。以通过独立收据复核的 2AV 为基线，新增两个准备入口，在普通 Object/Global 执行中允许 Object、Global、Host 函数值直接捕获受限无环 Array/Mapping。旧入口不扩大白名单。包版本与依赖不变；公共 API 仅增加两个函数，现有类型、快照及入口签名不变，保持源码兼容。

## 使用与权限

新增根导出与 `prepared` 模块入口：

```rust,ignore
prepare_program_with_container_captures(image, GlobalBankSchema { slots: 0 })
prepare_global_program_with_container_captures(image, GlobalBankSchema { slots: 0 })
```

两者返回 `Result<PreparedProgram, VmError>`，能力组合分别沿用对应的 callable 入口，额外允许容器捕获。零槽 schema 不隐式安装 Global bank 或绑定 Global 程序，仍需现有显式操作。没有公开的可变权限开关。

Function 私有不可变 `CaptureProfile::{Scalar, Container}` 按创建时直接捕获槽的实际类型决定，不按创建程序的权限决定：零捕获/全标量为 Scalar，存在 Array/Mapping 为 Container。快照不增加该字段、不输出捕获、不增加 Owner。调用先保留既有 origin/owner 校验，然后在目标分派、复制及 Host 回调前阻止旧 caller 执行 Container，返回既有 UnsupportedFeature。新入口产生的 Scalar 可由兼容旧入口调用；进入被调字节码后使用该函数体所属程序自己的权限，不继承 caller 的权限。

直接 Object/Function、Reference、隐藏 Loop 捕获继续拒绝。Array/Mapping 的原有合法嵌套成员，包括 Object/Function，遵循现有容器运输规则，不递归清理、复制或新增成员权限。一般容器成员重接规则、Mapping 键、init/construct/default/linked/hidden 限制不扩大。

## 事务与生命周期

创建继续以 CopyPlan raw transfer 搬走真实源槽并按栈顺序建立稳定捕获槽，提交前检查完整最终图。展开继续使用语义 Owner 的唯一 move/共享 COPY 分支，容器 header 共享并保持成员槽身份；成员内的失效值在实际 COPY 该成员时才规范化。固定 bound_count 与 profile 不随展开清空改变。

2AV 的所有写入/发布门保持：新增 Function 捕获 Array/Mapping 后，公开成员写入若会闭合 Function→容器→Function 环，同样在提交前失败。共享 DAG 可用；工作量、项数、扩展深度、祖先校验继续由同一不可变 store ValueLimits 约束。无循环 GC、Object 物理 ref/execRef 或原生延迟删除扩展。

可报告失败不发布部分捕获/展开、不消费操作数、不推进 PC。Host 参数仍只读预览，回调只执行一次；签名失败在回调前拒绝，结果及最终投影在回调后校验。后验失败保留显式 saved root 与外部副作用，回调不重放。不增加 RuntimeFaultSite、catch 来源或恢复类别。

## 回归与证据

保留原 626 项普通测试、2 项 doctest 与全部原断言。既有私有 Function/PreparedProgram fixture 仅补充新字段，保持原场景；部分历史私有未来形状 fixture 仍标为 Scalar，仅用于原有结构/所有权测试，不能由公开创建入口产生。新增 fixture 明确使用 Container，并有公开真实创建测试独立覆盖实际分类。

新增 18 项测试（7 项内部、11 项公共边界），覆盖两类容器、五个合法创建上下文/目标组合、旧入口拒绝、标量/零捕获兼容、callee 自身权限、捕获与新参数顺序、标量位模式、共享 DAG、成员写入拒环、missing Mapping query 不插入、唯一/共享展开与成员源规范化、最后释放、全部可注入 Cell/工作区分配检查点、借用和帧/栈/图限额、Reference/Loop/生命周期守卫、Host 前后验失败与 saved root。

[51 案例映射](case-map.json)逐项保留 2AR 原依据，区分 Rust 覆盖、部分 Rust 政策对应、直接捕获排除及原生未知。不以合成 Rust 结果替代原生 allocator 顺序、attrib、catch saved-sp、creator ref/execRef、未发布头和最终生命周期证据。[发布与权限接点矩阵](publication-contacts.json)记录可复算源码锚点及执行测试；它是源码复核清单，不是 AST 完备性或原生等价证明。

完整收据为 [2AW 验证记录](../stage2aw-validation.json)，日志写入本目录，不修改历史收据。重跑完整检查：`python3 -B docs/stage2aw-evidence/verify_validation.py --write`；独立检查：去掉 `--write`。包含 debug/release、2 项 doctest、编译负例、fmt、严格 clippy、56 项冻结模型测试、9 项新收据测试、2AR 静态证据和 7,159 个程序/6,474,773 条指令的全 PAK 结构对比。

## 后续

`container_capture_ready=true` 只表示新入口的受限容器能力；`direct_owner_capture_ready=false`、通用 `non_scalar_ready=false`、`stage2_complete=false`。下一步是 T05 前置原生契约取证，闭合 2AR Q01/Q04/Q05/Q06 的相应退出域；直接 Object/Function 捕获继续阻塞。没有运行参考程序，没有新增原生分析或业务字节码执行。
