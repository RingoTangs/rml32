# 后续两批全局运行时实施规格（尚未实现）

依据[原生观察](../vm-global-semantics.md)与当前ObjectStore/Cell/VM接口制定。本页为Rust设计决策，不把资源限制、宿主管理接口或失败原子性称为原生事实。本轮只写设计，不新增以下API或修改70种支持集合。

## 第一批：显式合成银行与35/36

### 公共接口与兼容边界

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalBankSchema {
    pub slots: usize,
}

pub fn prepare_program_with_globals(
    image: ProgramImage,
    schema: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;

impl ObjectStore {
    pub fn register_globals(&mut self, values: Vec<Value>) -> Result<(), VmError>;
    pub fn remove_globals(&mut self) -> Result<(), VmError>;
    pub fn global_values(&self) -> Result<Vec<Value>, VmError>;
}
```

新增类型/函数从lib.rs重导出；保持现有Vm::new、with_limits、start、into_objects、ObjectStore::register_program签名和旧准备入口。ReferenceKind新增公开Global枚举项：这是可观察API扩展，可能要求下游穷尽匹配新增分支，不能描述为“无公共API变化”。不新增VmLimits字段或VmErrorKind。

银行属于ObjectStore而非Vm、Program或Object；每个store至多一个当前银行。不同服务使用不同store，不按制品路径/服务前缀隐式选银行。容量固定为0..=32,768，这是合成资源策略和有符号par16可寻址范围，不是原生容量上限；空银行与未安装银行有区别。所有槽仅支持现有普通Value和初始attrib0的可写合成槽，不提供const/声明属性导入或真实全局符号装载。

PreparedProgram私有增加 `global_schema: Option<GlobalBankSchema>`。旧prepare_program和现有继承/初始化/构造准备器固定None，仍拒绝35/36；新入口只接受既有单constitution Object格式，显式携带Some(schema)。不顺带开放Global格式、06或全局位07；第一批不新增带银行的继承准备入口。

新入口检查schema容量，35/36的par16须非负且小于slots，par8忽略。schema越限为Limit，指令索引错误为带PC/offset的InvalidProgram。新入口只准备普通函数，映像init与旧prepare_program一样不准备、不执行；既有可执行初始化/构造准备器维持None，遇35/36仍为UnsupportedFeature，默认表达式白名单也不扩展。可注册准备程序而暂不安装银行，但每次执行35/36须确认当前银行已安装且容量与当前程序schema完全相同，否则InvalidState。运行时防御性索引检查为Bounds。schema不存在为UnsupportedFeature，不以全局支持表已含35/36绕过opt-in。

### 安装、移除与引用寿命

- 内部银行保存 `Vec<Rc<Cell>>`，没有公开Cell或银行句柄。register_globals在已安装时返回InvalidState，不原位替换；超容量返回Limit。逐值验证现有容器无环/深度约束，不把银行当作额外一层数组；先完成全部验证和Cell分配，再一次发布银行。失败时不发布部分银行，不执行脚本；不承诺恢复Cell ID消耗或标准库OOM。
- remove_globals在未安装时返回InvalidState；成功只丢弃银行的根所有权，不主动clear槽。global_values在未安装时返回InvalidState，否则提供浅快照，不规范化失效Object，沿用Object::variables的检查用途。
- 36产生内部 `Target::Global { cell: Rc<Cell> }`，readonly=false、_lease=None。Reference::kind返回Global，resolve直接取得保留的Cell，**不查询当前银行是否存在**。不得调用owned_reference，否则会错误占用成员/临时引用配额。
- 移除后已有全局根引用继续读写旧Cell；重新安装创建全部新Cell，旧引用不会访问新银行。无需generation失效校验或GlobalBankId；唯一Cell ID及强引用保证身份。此选择遵循选定原生unref分支的引用保留推导，不宣称完整热更新兼容。
- 宿主可在预算暂停时经objects_mut移除并重装银行。之后新的35/36按当前银行/schema工作，旧引用按自身Cell工作。银行容量不等同于所有历史被引用槽的总数；这些引用仍占VM栈/帧资源且不能逃逸到公开Value。
- into_objects保留当前银行；VM栈中的脱离银行根引用随VM其余执行状态销毁而释放，不自动转移进ObjectStore。

### 指令与现有机制接点

35先检查普通函数/非生命周期上下文、栈容量、schema与当前银行，再读源并执行copy_value。先分配结果Cell，随后提交直接失效Object的源槽Undefined规范化，最后压入结果；类型/借用/资源失败不提前修改源或操作数。Bytes/Array/Mapping浅复制共享，复制不递归清理嵌套值。该失败前原子策略与原生提前读指针/压栈布局的区别须写入运行时报告。

36做相同上下文、容量、schema及索引检查，只保留全局Cell并压入引用，不读取或规范化内容。已有Global引用的14解引用、赋值、复合赋值和转发不能重复要求当前银行存在；它们仅按保留Cell及既有readonly/绑定规则操作。

全局根引用不计入max_reference_cells，Array/Mapping成员引用仍使用现有Owned lease配额及成员标量写入限制。validate_binding仅对Local施加“声明槽”约束；Global可绑定既有引用形参或受限循环变量，不扩展引用逃逸和返回能力。银行不保存Reference/Loop payload，现有无环Value模型不变。

从脱离银行的根引用取得成员引用时，沿用现有释放顺序：若最后一个Mapping根所有者在44结束时释放，MappingData::drop清空节点，保留的成员引用有效但读到Undefined；Array成员槽继续保留其值。不能给成员引用增加银行存在性检查，也不能为延长Mapping内容寿命而额外保留根头。全局根槽替换容器同样复用既有释放规则。

35/36每条消耗一个预算，内部不暂停，零预算无分配或清理。错误不加入RuntimeFaultSite，catch不扩大；生命周期辅助函数运行时明确拒绝并进入既有清理/回滚。诊断使用实际对象函数的source/program/object/function/PC/offset，消息注明全局槽索引，不新增伪造对象身份或错误字段。

### 第一批退出测试

1. 旧准备器仍拒绝，新入口边界索引/par8正确；空schema、32768槽、负索引/等于容量、Global格式、全局07/06、初始化/default/lifecycle均按约定处理。
2. 两个普通对象共享同一银行而对象变量不别名；跨对象调用各帧使用自身schema。无银行、容量不匹配、重复安装/重复移除明确失败，未执行35/36的旧程序不受银行存在影响。
3. 35复制共享容器，直接失效Object返回并写回Undefined；快照和36不规范化。栈满/借用/验证或受控分配失败不产生部分源写入。
4. 36写穿透、嵌套引用转发、复合赋值、循环绑定和10形态检查；满引用配额仍能创建根引用，成员引用仍受配额限制。
5. 暂停后移除：旧引用可读写；重装后新旧槽身份/内容分离，新的35/36只访问新银行。移除后只有值副本而无根引用的容器寿命遵循既有共享规则。
6. 对脱离银行最后Mapping/Array根引用执行44，分别核对成员Undefined/保留值、lease释放及无借用panic。
7. 零预算/逐步执行/错误现场/不可捕获错误、生命周期拒绝清理；into_objects保留当前银行但不保留VM执行引用。

只在通过后将35/36加入支持表；预期72种限定编号、主语料交集70种，余05/06/17。初始化统计中的35/36仍不代表执行支持，能力矩阵必须明确opt-in、单constitution及全部排除条件。

## 第二批：受限全局函数准备与07（独立实施）

### 准备与绑定接口

```rust
pub fn prepare_global_program(
    image: ProgramImage,
    schema: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;

impl ObjectStore {
    pub fn bind_global_program(&mut self, program: ProgramId) -> Result<(), VmError>;
    pub fn clear_global_program(&mut self) -> Result<(), VmError>;
}
```

PreparedProgram私有增加Object/Global代码种类，共用单调ProgramId和现有注册表；ObjectStore::create对Global代码返回UnsupportedFeature，不伪造全局对象。global准备器仅接受Magic559 Global映像、无隐藏函数/继承、已支持原型及普通函数体；schema.slots须等于映像max_vars。function_base须至少559，函数槽保留绝对index且在[function_base,max_functions)内，重复/越界为InvalidProgram。只准备函数，不执行init，不将其宣称为全局启动装载器。

bind_global_program选择已注册Global代码；对象程序为UnsupportedFeature、缺失ProgramId为既有InvalidProgram，重复绑定为InvalidState。clear在未绑定时InvalidState。允许暂停时清除再绑定：旧帧按保存ProgramId继续旧代码，之后新的全局07查当前绑定。注册代码继续由现有store注册表持有，不引入代码卸载API。

第一批prepare_program_with_globals在本第二批才开放全局位07；旧prepare_program及无银行schema的准备路径继续拒绝。Global代码中的非全局07明确UnsupportedFeature，不猜测NULL对象上的对象调用。06仍拒绝；559..function_base之前的预置函数不自动补造。合成程序可选function_base=559；真实572之前的13槽绑定另属bootstrap阶段。

### 双上下文帧与数据流

```rust
enum ExecutionSource {
    ObjectProgram(ProgramId),
    GlobalProgram(ProgramId),
}
struct Frame {
    // 保留id/body/pc/bp/argc/entered/phase
    execution: ExecutionSource,
    active_object: Option<ObjectId>,
}
```

execution决定实际代码、字符串池、函数解析、继承偏移和错误program/source；active_object仅表示对象上下文。对象帧要求Some且保留现有存活检查；全局帧为None，不再每条step无条件objects.get。对象变量指令3d/3e/4e在无对象时UnsupportedFeature；41在None时返回Int(0)，绝不制造ObjectId(0)。

全局07按par8 bit7、低6位argc解码，忽略bit6；par16作为绝对全局槽，不加对象baseFunOffset。无绑定为InvalidState；负数/超过全局表容量为Bounds，容量内但无可执行绑定（包括未实现预置槽）为UnsupportedFeature。调用前完成参数、原型和资源检查；成功压入GlobalProgram帧、active_object=None。不能把函数编号与35/36的变量索引混用。

普通对象到全局、全局到全局及全局到既有直接Object目标08为首批组合。08创建ObjectProgram帧/Some(target)，返回后恢复调用者GlobalProgram/None；返回上层对象帧时恢复该对象与代码池。全局参数、返回值、引用绑定和执行预算沿用已支持契约；35/36仍查当前银行与该代码的schema。全局函数调用不伪造对象execRef，不把已删除调用者当作全局代码对象；恢复对象帧后按既有存活规则检查。

错误和catch保存/恢复execution与active_object，global错误object=None，function为实际绝对槽。只开放既有数值来源catch，不扩展生命周期、回调、hook、线程/JIT或宿主重入。没有新增host start_global入口，合成全局调用由现有对象start进入。

### 第二批退出测试与排除

测试object→global→object与global→global嵌套、独立池/绝对槽、None的41、对象操作拒绝、caller删除后返回、预算暂停重绑定、参数/引用/返回类型错误、受限数值catch跨双上下文恢复。全局程序的初始化/默认表达式扩展、真实13槽bootstrap、559项efun、代码卸载和主动对象析构继续排除。

第二批不新增操作码编号，只扩展07/41的限定路径；不可用编号数量宣称全局程序启动兼容。本页所有新API和帧迁移在阶段2AA均未实施。
