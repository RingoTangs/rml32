# 阶段 2N：受限对象初始化运行时

## 总体结果

实现显式准备、分配新对象、按继承顺序执行初始化段、预算暂停和失败清理。新增 `2e` 无操作与 `48` 初始化返回，合计57种限定操作码；本阶段不运行create/destruct，不实现全局初始化、efun、生命周期重入或局部异常恢复。

本轮复用[阶段2M静态证据](vm-object-lifecycle.md)，没有重新执行原版或业务字节码。运行验证全部使用人工构造程序。Rust测试共191项（原有171项，新增20项），静态语料比较结果见[验证记录](stage2n-validation.json)。

## 准备接口与使用方式

```rust,ignore
use rml32::{ProgramCatalog, LinkLimits, ObjectStore, Vm, RunOutcome, Completion,
            prepare_initializable_program};

let mut catalog = ProgramCatalog::new();
catalog.insert("root", image)?;
// 有继承时先插入父映像，并按完整原始名字节显式bind_inherit。
let prepared = prepare_initializable_program(&catalog, "root", LinkLimits::default())?;
assert!(prepared.is_initializable());
let mut store = ObjectStore::new();
let program = store.register_program(prepared)?;
let mut vm = Vm::new(store);
let object = vm.begin_initialize(program)?;
loop {
    match vm.run(1000)? {
        RunOutcome::BudgetExhausted => continue,
        RunOutcome::Finished(Completion::Initialized(id)) => {
            assert_eq!(id, object);
            break;
        }
        _ => unreachable!(),
    }
}
```

`prepare_initializable_program`沿用显式目录、无共享祖先继承及LinkLimits；同时准备所有普通实现和全部来源初始化段。只准备，不执行。初始化段为空、没有48、来源字符串/变量/metadata/分支越界时失败；global、hidden和未支持指令仍拒绝。所有指令均检查，包括48之后的哨兵；这不是可达性分析。

`prepare_program`、`prepare_linked_program`继续忽略初始化代码；原有映像不会因新增初始化要求而准备失败。它们的结果不具备初始化能力，`begin_initialize`返回UnsupportedFeature。

初始化直接使用参数/局部槽、普通入口或返回的指令（0f/1b/47/49/34/15/37/39/4d）明确拒绝；通过07调用的普通函数仍支持原有引用、默认和可变参数路径。48不能出现在普通函数中；2e可在两种代码体中使用。

## 帧、来源与预算

程序保存每个constitution的独立初始化代码体、来源池和基址，以及父先自身后的初始化排列。初始化不注册函数槽，也不生成伪造的PreparedFunction或负索引。内部帧身份为Function(slot)或Initialization(constitution)，共享代码视图和指令执行器；constitution运行上下文投影在对应代码体中。

07使用当前函数基址，3d/3e/4e使用当前变量基址，40使用当前来源池，数值metadata由该代码段解码。每个对象只有一份root大小的稳定Cell数组。普通被调函数返回后仍进入原初始化帧，其返回值须由初始化代码消费。

每个指令恰好消耗一次预算，包括2e和48；48要求计算栈平衡、没有尚未返回的调用帧，并切换到下一初始化段。切换不额外扣指令数，也不重置预算；最后一个48在当前调用中直接返回`Completion::Initialized(object)`。`run(0)`不执行指令，完成后重复run仍返回同一结果。

1d保持普通任务的ActionFinished语义；初始化任务中任何执行帧命中1d均受控失败，不能用它代替48。准备时允许48后的1d哨兵，正常初始化返回不会执行它。循环不会被当作成功，仍按预算暂停。

原生依据：`vm_callInInitFun@0x08094258`与`vm_returnFromFakeFun@0x08094374`确认初始化上下文及返回；`vm_f_construct_object2@0x080f90b4`确认初始化排列的消费者。关键Evidence ID分别为 `ev_e87fd215c338b444b786777608553c57b73d83dc1d3675ab73c10ec003991e6f`、`ev_13bcf5ba026e9e5fa004744ae2eb642987af494bebf7a74f0a830da392e28518`、`ev_da96a5f71d49dbc7b88d7cf809835b39c4a15c293342ba20674155e53dfa3606`，完整记录保留在阶段2M证据目录。

## 对象状态、失败与兼容性

`begin_initialize(ProgramId)`仅分配新对象，所有变量初值Undefined，返回该ObjectId供暂停期间检查。它不接收已有对象，因此失败清理不会误删手动创建对象。对象状态查询为 `Object::initialization_state()`：

| 状态 | 含义 |
|---|---|
| Manual | 通过既有ObjectStore.create显式创建，未自动初始化 |
| Initializing | 新初始化任务尚未完成，Vm::start拒绝该对象 |
| Initialized | 所有初始化段完成，允许普通函数入口；构造函数仍未执行 |

VM同一时间只接受一个活动任务。普通任务或初始化任务暂停时，其他start/begin_initialize返回InvalidState；预检查失败保留已有结果和执行。初始化期间对象可被宿主读取，但这不模拟原生全局名称查找或提前发布语义。

任务开始后任何错误先记录实际执行位置，再清理任务帧、栈及引用额度，最后删除任务创建的对象；不执行业务析构。后续run重复返回首个错误，成功开始下一任务才重置失败。普通函数任务原来的错误保留行为没有改变。

宿主提前remove任务对象时，下一指令返回InvalidObject并完成清理；清理不覆盖首个错误。`into_objects()`取回存储时会取消未完成初始化并删除该新对象，其他对象保留；成功完成的Initialized对象不会被删除。丢弃整个VM则按Rust所有权释放其存储。

`VmError.initialization: Option<usize>`表示活动任务当前初始化的constitution，即使错误发生在嵌套普通函数中也保留该任务位置。初始化代码错误的function=None，普通函数错误保留真实分派槽；program/object为root句柄，source/pc/offset指向实际代码。普通任务该新字段为None。

公开API新增Completion变体与VmError字段；下游穷举匹配Completion或直接构造VmError的代码需要补齐。既有对象创建、移除及普通函数启动签名不变。没有新增依赖或unsafe。

## 验证与边界

合成测试覆盖独立和多父初始化、三层继承、来源字符串/metadata、调用及变量重定位、逐步预算、Undefined初值、对象隔离、引用/可选/可变参数、诊断来源、首中末段失败、任务取消、提前删除、临时引用额度释放、栈/帧限制及旧接口兼容。生命周期函数可存在于程序中，但不会被初始化任务自动调用，显式调用仍受既有边界拒绝。

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --bin rml32-inspect
python3 tests/compare_corpus.py --report target/stage2n-corpus-validation.json
```

所有语料仅解析；已知DBA压缩异常仍报告。初始化“可准备”或测试通过均不能证明原有服务端能够启动。后续需独立实现局部异常恢复，再开放create/destruct及其逆序回滚和延迟删除。
