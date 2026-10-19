# 阶段 2AC：受限全局函数调用

实现阶段 2AA [第二批规格](stage2aa-evidence/global-runtime-design.md)，接续阶段 2AB 的[显式全局表](vm-global-runtime.md)。原生观察继续使用已冻结的[全局语义证据](vm-global-semantics.md)，本阶段不新增原生会话，不执行参考 ELF、启动脚本或业务字节码。

## 准备、绑定与兼容性

新增并重导出 `prepare_global_program(image, GlobalBankSchema)`，返回既有 PreparedProgram；新增 `ObjectStore::bind_global_program(ProgramId)` 和 `clear_global_program()`。不新增公共代码种类枚举、错误种类、VmLimits 字段、依赖或 unsafe。

新准备器仅接受 Magic559 Global 映像，无隐藏函数/继承；schema.slots 必须等于 max_vars，function_base 至少 559 且不超过 max_functions，已实现函数绝对编号必须落在 [function_base,max_functions)，重复、负数或越界拒绝。参数和返回原型沿用现有支持范围。只准备普通函数，init 忽略；原有默认表达式白名单不扩展，35/36/全局07 不可作为默认表达式。Global 对象变量指令仅有防御性运行时拒绝，没有对象变量执行语义。

PreparedProgram 私有保存 Object/Global 种类和函数容量，共用注册表的单调 ProgramId。Global 代码不能创建对象，返回 UnsupportedFeature。每个 store 最多一个当前绑定；重复绑定、清除未绑定状态返回 InvalidState，未知 ID 为 InvalidProgram，选择 Object 程序为 UnsupportedFeature。绑定不要求安装变量表，清除代码绑定也不移除变量表。

`prepare_program_with_globals` 现允许全局位 07；旧 prepare_program、链接及生命周期准备路径继续拒绝。Global 代码的非全局 07 和所有 06 继续拒绝。没有新的宿主 start_global 入口，合成测试仍从 Object 的 start 进入。

## 双上下文执行与寿命

帧内部使用 ExecutionSource::{ObjectProgram,GlobalProgram} 保存实际代码身份，active_object 为 Option<ObjectId>。代码身份决定指令、字符串池、引用绑定时的声明槽及错误来源。Object 帧要求当前对象存在；Global 帧要求 active_object=None，不使用假 ObjectId。

全局 07 使用 par8 bit7，低六位是 argc，bit6 忽略；par16 为绝对函数编号，不加对象函数偏移。缺少绑定返回 InvalidState；负数或达到函数容量返回 Bounds；容量内无执行体（含预模块槽）返回 UnsupportedFeature。调用先检查参数形态、原型/类型及帧资源，再安装新帧；参数规范化仍由 0f 批量提交。失败不会提前改变参数、根槽或帧深度。原有普通对象 07 的时序保持不变。

Global 的 41 返回 Int0，3d/3e/4e 返回 UnsupportedFeature。Global 可通过既有 08 调用直接 Object 方法，使用目标对象的代码、方法表和身份；返回后恢复 Global/None，再返回上层 Object 时恢复其身份。已删除的对象调用者不阻止活动 Global 代码执行，返回后下一条对象指令仍按原存活规则失败。

暂停时清除或更换绑定不会改变已进入帧的 ProgramId；后续全局 07 查当前绑定。代码注册表继续保留旧程序，不增加卸载。35/36 使用实际执行程序自身的 schema 对当前变量表检查，沿用 2AB 的强根引用保留、移除/重装和成员 lease 规则。into_objects 保留当前变量表和代码绑定。

catch 沿用保存帧身份、深度、栈长度和目标 PC 的机制；恢复时从保存帧取实际代码及可选对象，只对 Object 上下文检查对象存活。仍只捕获已有数值故障来源，生命周期及缺绑定/类型/边界/资源错误不会新增捕获路径。Global 错误的 object=None，program/source/function/PC/offset 对应真正执行的全局程序；调用预检失败则归属调用指令所在的帧。

## 验证与剩余范围

新增 20 项集成测试和 3 项内部测试；既有 384 项回归保留，阶段 2AB 的准备测试随全局07开放调整预期。新增测试包括实际序列化/解析的合成 Global 映像、绑定错误、绝对槽/标志、独立池、object→global→object、global→global、引用转发、两向 catch、参数失败原子性、清除/重绑、删除调用者、无对象错误现场、默认表达式/初始化隔离、生命周期拒绝/回滚主错误保留，以及预算暂停和单步执行。

当前为 407 项 Rust 测试（35 unit、23 parser/CLI、349 VM），支持种类仍为 72，语料交集仍为 70、缺 05/06/17。全局 07 的静态出现次数不代表真实服务可执行；预置 13 槽、559 项 efun、Global init、真实符号/声明属性装载、代码卸载、回调/JIT/宿主重入及完整原生生命周期均未实现。

复现见[阶段方法](stage2ac-evidence/method.md)，能力边界见[当前矩阵](stage2ac-evidence/matrix.md)；历史整包验证器仍要求历史源码快照，本阶段独立保留其证据/统计校验，不重写旧记录。
