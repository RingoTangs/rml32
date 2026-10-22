# 阶段 2AO：受限 Global/c 函数值

按[2AN第一批规格](stage2an-evidence/function-runtime-design.md)实现普通Object／Global函数中的Global/c创建与间接调用。原生依据使用冻结的[2AN语义与证据](vm-global-function-value-semantics.md)，没有新增反编译或原程序执行。全部执行测试为内存中合成程序。

## 公共接口迁移

新增并从库根重导出 `FunctionTarget::{Object { program, function }, Global { program, function }}`；FunctionValue增加`target()`。`owner()`改为`Option<ObjectId>`，表示创建时当前对象，Global上下文创建时为None；`program()`改为`Option<ProgramId>`，本批两类目标均为Some。`function()`和`bound_count()`不变。**owner()/program()为源代码不兼容变更**，调用者须处理Option；不制造假对象／程序句柄，也没有会panic的旧必填访问器。

FunctionData私有保存origin、可选creator owner、typed target和标量bound。复制共享函数头，Eq仍比较头身份；独立创建的同字段函数值不相等。Debug只显示owner、target和捕获数量，不暴露捕获内容。Host变体未加入。

新增显式准备入口：

```rust
pub fn prepare_program_with_global_function_values(
    image: ProgramImage, globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
pub fn prepare_global_program_with_global_function_values(
    image: ProgramImage, globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
```

新入口增加独立Global目标能力，包含原Object函数值及既有全局表／07能力。旧函数值入口仍只执行Object目标，传入Global函数值不能绕过能力限制。直接06、K、非global/K、继承／hidden、init和默认表达式能力未扩展；Global映像不能创建局部Object目标。

## 17 创建与 05 调用

17先将signed par8按u8解释，0x80..0xbf为Global/c，低六位是0..63个捕获。捕获继续限Undefined／Void／Int／Real／Bytes，保持位模式和原始字节；Reference／Loop与非标量捕获继续拒绝。局部Object目标沿用已有检查顺序；Global目标先检查普通捕获，再解析当前Global绑定。它的绝对槽不能用创建程序的函数容量或本地目录验证。

准备时负槽为InvalidProgram；运行时缺绑定为InvalidState，越容量为Bounds，容量内缺已准备函数体（包括bootstrap空隙）或引用形参为UnsupportedFeature。private直接编译槽可用，不走方法名称查找。创建时保存绑定ProgramId／函数号，以及独立creator owner；在成功发布函数值后才消费捕获并推进PC。

05保留原flags=0/4、argc含目标及总参数数≤127规则；捕获在新实参之前，沿用用户函数转换、nullable、默认及variadic行为。先检查store origin和可选creator存活，再分派实际目标。Global目标要求调用程序显式启用该能力，且当前绑定仍等于保存ProgramId。

清除绑定或换成其他ProgramId后，旧值的新调用报InvalidState；重新绑定同一ProgramId即可重新调用。此为**受限Rust策略**：原生Global/c保存代码却读取当前Global constitution，现有证据不足以开放任意热替换。程序注册表不卸载代码；已经进入的Global帧继续保存原程序，后续直接07仍读取当前绑定。

Global callee使用保存程序的代码、字符串池及schema，active_object=None，41返回Int0；对象变量执行仍报错。创建者owner只用于失效检查，不变成Global当前对象。目标schema与创建者schema可以不同。返回恢复原Object／Global调用者；callee错误显示其实际program／source／function／PC／成员offset，object=None；调用预检错误属于05现场。

## 复制、失败与预算

同origin且Some(owner)已删除的函数值沿既有复制接点成为Undefined；None不会被当作失效owner。raw05检查已删除creator时为InvalidObject，flags=4不吞错误。foreign store先报InvalidState，不查本store碰撞编号；Rust Clone和宿主opaque快照不承担普通VM复制语义。完整提前源cleanup和原生ref／execRef延迟删除仍保留既有差异。

05／17每条消耗一个预算，没有内部暂停点；零预算无执行，17后可暂停在已发布值，05后可暂停在callee入口。可报告的分配／借用／栈／帧预检失败不消费操作数、不推进PC、不消耗帧ID；临时Cell ID可消耗，标准库不可恢复OOM不作回滚保证。保留引用lease前缀和最后函数头释放；into_objects后新建VM保留origin。活动执行不能用start直接替换，沿用既有状态检查；失败后的start沿原规则重新开始。

仅既有数值故障来源可被catch解栈；缺绑定、目标／owner／类型／资源错误仍终止，不新增RuntimeFaultSite。生命周期守卫在访问操作数前执行，不能从普通helper绕过任务限制。

## 验证与剩余能力

新增29项Rust测试（7内部故障／寿命测试、22集成测试），原515项保留，总544项在debug／release通过；fmt、clippy通过。覆盖全部256种17标志、绝对槽、creator与callee分离、无owner创建、独立池／schema、绑定切换、两层Global间接调用和上下文恢复、参数／捕获边界、nullable／private、数值catch及失败原子性。

复用22项2AN纯工具测试，新增5项本批完整性测试，共27项Python检查。2AO校验器单独复核20条原生记录及5,012个Global创建位置，555个既有文件保持摘要不变；历史整包校验的源码快照不被刷新。详情见[方法](stage2ao-evidence/method.md)、[能力增量](stage2ao-evidence/matrix.md)和[验证记录](stage2ao-validation.json)。

仍为75种限定操作码、主语料73种交集；G12／G18仅扩展了本批路径，阶段2未完成。Global/K、非标量捕获、完整GC／删除、热编译／卸载、继承／hidden、生命周期、字符串／OS、真实efun／bootstrap和服务仍未实现。没有执行原始ELF、脚本、init或业务字节码，不宣称原版运行等价。
