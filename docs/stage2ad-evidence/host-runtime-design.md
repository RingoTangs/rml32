# 下一批：受限同步06实施规格（尚未实现）

依据[原生语义](../vm-efun-semantics.md)与阶段2AC的双上下文帧。以下均为Rust设计决策；阶段2AD只交付规格，不新增公共API、运行时或依赖。

## 公共接口与注册寿命

新类型和函数从lib.rs重导出，保持既有Vm构造/start/run/into_objects、GlobalBankSchema及所有旧准备入口签名。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostType {
    Any, Undefined, Void, Int, Real, Bytes, Object, Array, Mapping,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostReturn {
    Void,
    Exact(HostType),
    Mixed,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSignature {
    pub required: Vec<HostType>,
    pub optional: Vec<HostType>,
    pub rest: Option<HostType>,
    pub returns: HostReturn,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCallContext {
    pub slot: u16,
    pub source: String,
    pub program: ProgramId,
    pub object: Option<ObjectId>,
    pub function: i16,
    pub pc: usize,
    pub offset: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostError {
    pub kind: VmErrorKind,
    pub message: String,
}
pub type HostCallback = std::rc::Rc<
    dyn Fn(&HostCallContext, &[Value]) -> Result<Option<Value>, HostError>
>;
#[derive(Clone)]
pub struct HostFunction {
    pub name: String,
    pub signature: HostSignature,
    pub callback: HostCallback,
}
pub fn prepare_program_with_host_calls(
    image: ProgramImage,
    globals: Option<GlobalBankSchema>,
) -> Result<PreparedProgram, VmError>;
pub fn prepare_global_program_with_host_calls(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
impl ObjectStore {
    pub fn register_host_function(
        &mut self, slot: u16, function: HostFunction,
    ) -> Result<(), VmError>;
    pub fn remove_host_function(&mut self, slot: u16) -> Result<(), VmError>;
}
```

HostFunction手写Debug，只显示name/signature，不打印callback；保留ObjectStore现有Debug能力。新增私有 `BTreeMap<u16, HostFunction>`，属于store而非程序/全局变量表；into_objects保留注册。注册仅限0..558，559及以上返回Bounds，已有槽返回InvalidState，移除未注册槽返回InvalidState。name仅用于诊断，非空且不含NUL，否则InvalidProgram；不自动匹配原生名称，同名不同槽允许。合成回调可以占用任意合法槽，但文档不得称其实现了该槽的真实efun。

注册前验证required.len()+optional.len()<=127；超出返回Limit。Exact(Any)和Exact(Void)返回InvalidProgram，分别改用Mixed和Void。Any参数接受全部现有普通Value，其他类型与Value::tag精确匹配，不作int/real转换。签名是合成宿主的显式契约，不从原生CSV猜测参数原型。可选参数不补默认值；rest按同一类型检查剩余实参；总实参始终不超过127。

不新增VmErrorKind、VmLimits字段、Value或ReferenceKind变体。公开HostError允许外部回调构造失败，不需要开放当前私有VmError::new；其kind和message映射到新建VmError，其他现场由VM填入。

## 准备与执行入口

- PreparedProgram私有保存allow_host_calls。Object新入口只接受既有无继承、单constitution、无隐藏函数的Magic559 Object普通函数，globals=None时与prepare_program能力相同但开放06；Some时同时保留现有全局表与全局07能力。Global新入口沿用prepare_global_program的格式/schema/函数区间限制，同时开放06。
- 两个入口只准备普通函数，忽略init；默认表达式白名单不增加06。既有全部准备入口继续拒绝06，不因支持表新增编号而绕过显式选择。继承、初始化、构造及生命周期辅助执行仍拒绝06，包括通过内部调用绕入的情况。
- 准备时06的signed par16须在0..558，par8须在0..127；非法值为带函数、PC、成员offset的InvalidProgram。**没有标志位掩码**，64表示64个参数，负值不是标志。允许准备/注册程序而尚无宿主注册表。
- 运行时再次检查allow_host_calls/普通执行上下文，违反为UnsupportedFeature；防御性槽号或argc不在0..127时为Bounds；合法槽未注册为UnsupportedFeature。06无需绑定Global代码或安装变量表，除非程序执行其他相关指令。

## 一次同步调用的数据流

1. 固定本条指令的caller帧和Rc克隆的HostFunction；验证上下文、槽和注册存在，调用need(argc)保护参数/局部区。按现有向上增长的Vec取 `stack[len-1-n]` 作为参数n。
2. 按顺序预检：当前字节码帧数+1不超过max_frames（临时host activation也占一层）；required/optional/rest的数量；所有实参形态；精确类型及现有容器无环/深度限制；`len-argc+1<=max_stack`。数量不符为ArgumentCount，引用/隐藏Loop payload为UnsupportedFeature，类型不符为UnsupportedType，配额为Limit，借用冲突沿用InvalidState。Value::Object仅作为opaque句柄传递，不查存活、不规范化；Any也不接受Reference或隐藏payload。
3. 浅克隆参数Value，保留Bytes/Array/Mapping共享身份；不调用会规范化失效Object的copy_value，不写回源槽。预分配一个Void结果Cell，并完成提交所需栈容量的try_reserve；失败为Limit，尚未调用宿主。Cell ID已消耗不要求回滚；标准库OOM不承诺可恢复。
4. 使用私有局部HostActivation保存调用者信息，不向持久frames新增假字节码帧，不分配假ObjectId/ProgramId。HostCallContext.program/source/function/pc/offset来自当前实际执行代码，object为caller.active_object。Object保持Some，Global保持None。回调只得到context与参数切片，得不到Vm、ObjectStore、Cell、Reference或控制标志操作入口。
5. 回调在本条指令内同步调用恰好一次。返回后按下面规则检查结果并用预分配Cell.replace提交内容；失败不截断操作数、不前进PC。返回值的容器约束再次验证；Object同样opaque，不隐式规范化。无持久RefCell借用跨越回调。
6. 成功时先释放临时参数副本，再截断argc个原操作数，压入结果Cell，最后将caller.pc增加1。结果Cell已经持有结果所有权，因此返回某个Array/Mapping参数不会在丢弃旧参数时丢失内容。所有容器/成员lease清理复用现有释放规则，不持有借用guard跨析构。

HostActivation只是同步局部状态，成功和Result失败均随调用离开释放；它不进入catch的字节码frame_depth，也不生成新的脚本函数号。注册表只能在run返回后经objects_mut修改；预算暂停后新06查最新注册，已经完成的06不能重放。删除注册不改变先前返回Value的身份。

## 返回、失败和预算

| 签名 | 合法callback结果 | 栈上结果 |
|---|---|---|
| Void | Ok(None) | Value::Void |
| Exact(T) | Ok(Some(value))且精确匹配T | value |
| Mixed | Ok(None)或Ok(Some(任意现有普通Value)) | Void或value |

其他结果返回UnsupportedType，消息含槽号/name与预期；不做数值转换或补成功值。Option编码结构上排除多结果，不暴露原生栈计数接口。

回调Err按HostError.kind传播，消息前缀包含 `host slot N (name)`；回调与返回验证错误均归属于调用者06的实际program/source/object/function/PC/offset，不伪造全局函数帧。所有06错误都保持RuntimeFaultSite=None，**即使回调报告Arithmetic也不可被当前脚本catch捕获**，避免靠错误kind意外扩展异常边界。预检失败回调次数为0；回调失败或返回错误后次数为1且VM进入既有sticky failure，后续run不重复执行。

预检失败不改变操作数/根槽/帧/PC。调用后失败也保留原操作数形态与PC，但回调对共享容器、其捕获状态或外部世界的副作用不回滚；不承诺事务或恰好一次外部I/O。清理临时Value与HostActivation后走既有VM失败路径，保留现有普通失败栈快照直到start重置或VM销毁。回调必须正常返回Result；panic不转成脚本错误，不引入catch_unwind或unsafe，panic不在本规格的可恢复契约内。

06消耗一个指令预算；零预算不查注册/分配/调用，正预算可完整执行一次06再暂停。回调内部无预算暂停、无Yield/挂起结果；预算限制只约束字节码指令数，不约束宿主耗时。宿主回调不得经捕获状态递归驱动正在执行的同一Vm；本接口不提供重入入口。

## 后续实施验收清单

1. 旧准备器拒绝；新Object有/无globals、新Global普通函数可准备；init/default/linked/lifecycle继续隔离。槽0/558/559、负槽、argc0/64/127/负值均验证，64不被掩码为0。
2. 空注册、重复注册、重复移除、无效签名/名称、into_objects保留；不安装全局表/代码仍能运行独立06；新注册不改变Global 07的槽解释。
3. 用减法合成回调验证参数0/1顺序，固定/可选/rest数量和类型；不补默认参数，不做数值转换，Any仍拒绝引用和隐藏loop值。失败前计数器保持0，早期失效Object不被后续类型错误规范化。
4. Void/Mixed/每个Exact类型的成功及错误结果；共享容器身份、参数作为返回值、嵌套Object句柄原样传递；callback Err与错误返回的调用计数保持1。
5. Object→06、Object→Global→06→Global→Object恢复正确代码池/身份；catch包裹宿主Arithmetic仍失败；普通数值catch既有用例通过，错误现场指向真正的06。
6. 栈/帧/容器限额、借用冲突、可控Cell分配失败均在回调前失败，无部分写入。错误后的临时参数与结果释放、共享成员lease及start重置不泄漏，不发生借用panic。
7. 零预算、停在06前、仅一次06后暂停、重注册再恢复、sticky failure重复run；每次成功调用只扣1条预算且不重放。宿主副作用保留的测试不得断言回滚。

实施完成后才把06加入限定支持集合，预期73种支持/71种主语料交集，仍缺05/17。该数量不是功能完成率；真实559项efun、原生引用传递、调度/hook/JIT、全局init及服务启动继续排除。
