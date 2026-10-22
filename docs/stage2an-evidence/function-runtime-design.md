# 后续 Rust 实施规格：Global／同步 Host 函数值

本页为待实施规格；2AN不修改运行时/API。依据[语义报告](../vm-global-function-value-semantics.md)、findings.json及expected-cases.json。两批独立验收，第一批不引入间接Host，第二批再组合。操作码数量仍为75，不能以编号交集宣布阶段2完成。

## 共用表示与公共接口迁移（第一批）

私有FunctionData保存origin、`owner: Option<ObjectId>`、typed target及原有标量bound。owner表示**创建时**active_object，Object目标必须Some，Global／Host可None；不用于决定Global／Host调用上下文。origin与Rc头身份比较不变，无公开构造器或可变捕获入口。

公开新增并重导出只读描述枚举：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionTarget {
    Object { program: ProgramId, function: i16 },
    Global { program: ProgramId, function: i16 },
    Host { slot: u16 }, // 第二批增加
}
impl FunctionValue {
    pub fn owner(&self) -> Option<ObjectId>;
    pub fn program(&self) -> Option<ProgramId>; // Object/Global Some，Host None
    pub fn target(&self) -> FunctionTarget;
    // function() -> i16 保留，Host返回规范槽号；bound_count()不变。
}
```

这是owner()/program()的源代码不兼容迁移，在实现批README记录并更新全部Rust调用者／测试；不保留会panic的旧必填访问器，不伪造句柄。Value标签9、身份相等、原始真值和标量bound不变。Debug仅打印owner、target和bound_count，不打印捕获。

同origin值只在Some(owner)已删除时按既有复制接点变为Undefined；None不判死。raw05仍先检查origin、保存owner，再检查目标及参数。foreign store不查碰撞编号。保持既有提前源规范化和延迟删除的限制，不扩大这些语义。

## 第一批：Global/c 用户函数

新增两个准备入口并从库根导出：

```rust
pub fn prepare_program_with_global_function_values(
    image: ProgramImage, globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
pub fn prepare_global_program_with_global_function_values(
    image: ProgramImage, globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
```

- 私有能力位在原function_values之外独立标记global targets。新入口包含原有Object函数值、全局表／07及Global/c能力；Global映像不能创建局部Object目标。旧入口维持2AG允许范围，即使传入新种类函数值，也拒绝未启用的目标种类。新入口仍拒绝06与K。
- 保持无继承、无hidden、单constitution及既有普通函数原型约束；不准备init，不扩大默认表达式。schema沿用现有Object／Global准备规则，实际目标schema在其执行时校验，不要求调用者与目标schema相同。
- 17按u8解释flags：局部c为0x00..0x3f，Global/c为0x80..0xbf。负par16在准备时报InvalidProgram。全局绝对槽不能以调用者max_functions或本地函数目录检查；运行时解析当前绑定。K和非global/K仍UnsupportedFeature。
- 17先做任务／能力守卫和普通捕获检查，解析当前Global绑定：无绑定InvalidState，越容量Bounds，容量内无已准备函数体（含559..function_base间隙）UnsupportedFeature；引用形参仍拒绝。保存目标ProgramId和绝对函数编号，以及独立创建owner。目标可以是private直接编译槽，不走名称查找。
- 新05执行Global目标前，要求当前Global绑定仍为保存ProgramId；清除或重绑其他程序为InvalidState。返回后重绑到同一ProgramId可再调用。此项是受限Rust安全政策：不推断原生热更新下保存代码和当前constitution的组合。
- 安装`GlobalProgram(saved_id)`、active_object=None的普通帧，使用保存程序的代码、字符串池、默认／variadic和schema。41返回Int0，对象变量指令继续拒绝。已进入帧不再随绑定变化；其后直接07仍查询当前绑定，旧函数值的下一次新05重新检查绑定。
- 捕获和新增参数逻辑顺序为bound后接new，总数≤127；沿用原05的类型预检与用户函数入口规则，引用／Loop不作为普通参数逃逸。调用者前缀及引用lease保留，返回恢复原Object／Global上下文。callee数值故障可被既有catch处理；目标／绑定错误不新增catch来源。

第一批验收需覆盖C01/C03/C04/C06—C15/C20—C22/C24—C30中适用部分，以及旧Object函数值全部回归。程序P被clear或替换Q：进入前拒绝，进入后暂停帧继续P，后续直接07使用Q。

## 第二批：Global/K 显式同步宿主

在第一批基础上增加Host目标枚举变体及两个组合准备入口：`prepare_program_with_callable_values(image, globals: GlobalBankSchema)`、`prepare_global_program_with_callable_values(image, globals: GlobalBankSchema)`，返回PreparedProgram。其能力为Object／Global函数值、Global表／07、直接06及间接Global/K。原有宿主和函数值入口不自动获得组合能力。

- 17只新增0xc0..0xff，par16要求0..558；结构越界准备时报InvalidProgram，防御性运行时越界Bounds。非global/K仍拒绝。捕获继续只允许Undefined/Void/Int/Real/Bytes。
- 创建时要求槽已显式注册，否则UnsupportedFeature；只保存槽，不保存HostFunction或回调快照，也不需要Global代码绑定。使用当前调用帧的可选对象作为creator owner。
- 每次05按保存槽取得当前HostFunction及签名，移除后未替换为UnsupportedFeature；remove后重注册调用新的签名和回调。当前activation内clone本次注册，不能跨次缓存。不得因ABI名称为call_other就自动实现真实efun356。
- 宿主上下文来自**执行05的帧**：slot为目标槽，source/program/object/function/pc/offset为05调用者，object可None。Global目标创建时的owner不覆盖它。
- 用bound+new构建逻辑参数切片；新参数按现有06的普通值／HostSignature规则检查，不套用用户函数数值转换。先拒绝Reference／Loop，完整校验count/type/graph与栈／frame预算，预分配结果Cell和提交所需容量，再调用一次回调。
- 提取06和05可共用的预检／激活／返回校验逻辑；不能先消费05目标、改写操作数或推进PC再调用旧call_host。成功返回并检查结果后才一次提交目标／参数替换和PC。Void／Mixed／Exact沿用现有HostReturn。Any/Mixed继续opaque透传Function，不自动调用或规范化它。
- 预检失败callback计数必须为0；callback失败或返回不合法保持VM操作数及PC、进入sticky failure，已发生的外部副作用不回滚，也不重放。错误诊断固定05现场，不制造06伪PC。无异步、重入、引用回调、新RuntimeFaultSite或一般catch。

第二批验收覆盖C02/C03/C05—C10/C16—C23/C25—C28；尤其A创建/B调用、无对象调用者、换签名／换回调、删除creator、返回失败与零重放。旧06、旧Object函数值和第一批Global/c均回归。

## 共用事务、预算与退出条件

05／17先做任务和能力守卫，失败不消费操作数、不推进PC、不消耗帧ID。所有可报告的预留、Cell分配和借用失败都在可见提交前返回；临时Cell ID不回滚，标准库不可恢复OOM不作保证。错误种类、VmLimits、ReferenceKind、依赖及unsafe策略不增加。

每条05／17计一条指令，无内部暂停点。Global/c可在callee入口暂停；Host05在一条指令内完成同步调用及结果发布。零预算无回调／分配。restart、返回、数值catch解栈及into_objects保留原清理／origin契约。

两批均运行debug／release、fmt、clippy及当前证据校验；新增合成程序测试按上述行为断言，不把静态模型当运行oracle。非标量捕获、图回收、全局热编译／卸载、继承／hidden、生命周期、字符串／OS、真实efun及服务不在退出条件内；继续列为显式限制。
