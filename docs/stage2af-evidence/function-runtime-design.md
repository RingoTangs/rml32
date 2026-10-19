# 下一批：受限Object函数值与05/17（尚未实现）

本页是2AF交付的Rust实施规格，依据[原生语义](../vm-function-value-semantics.md)，不声称本轮已实现。首批取名“Object函数值/标量捕获”：目标必须为普通Object字节码函数，捕获仅限Undefined/Void/Int/Real/Bytes。global/efun目标、字符串/OS分派、非标量捕获另行实施。调用者可为Object或Global；目标代码和对象始终来自函数值。

## 公共API及身份

```rust
#[derive(Clone)]
pub struct FunctionValue { /* private Rc<FunctionData> */ }
impl FunctionValue {
    pub fn owner(&self) -> ObjectId;
    pub fn program(&self) -> ProgramId;
    pub fn function(&self) -> i16;
    pub fn bound_count(&self) -> usize;
}
// 新增既有公开Value枚举分支：
// Value::Function(FunctionValue)，Value::tag()返回9。

pub fn prepare_program_with_function_values(
    image: ProgramImage,
    globals: Option<GlobalBankSchema>,
) -> Result<PreparedProgram, VmError>;
pub fn prepare_global_program_with_function_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
```

FunctionValue和两个准备器从lib.rs重导出；Value新增分支是公共兼容变更，下游穷尽匹配须增加Function。FunctionValue无公共构造器、无可变捕获访问器、无序列化/反序列化接口；只能由VM的17创建，宿主可以保存/克隆已有值。Debug手写，仅显示owner/program/function/bound_count，不打印捕获值。Clone共享函数头；PartialEq/Eq手写Rc指针身份，不按字段或捕获内容比较。

FunctionData私有保存origin: Rc<()>、owner:ObjectId、program:ProgramId、function:i16、bound:Vec<Value>。每个ObjectStore新增独立的私有Rc<()>身份token（Default/new都必须独立分配）；into_objects保留它。创建复制token，05必须先ptr_eq当前store；来自另一store的函数值即使编号碰撞也返回InvalidState，不能执行错误代码。这是Rust执行安全决策，不是原生跨store ABI。

程序注册表继续保留代码，没有卸载API；program+function固定创建时的代码身份，不重查当前全局绑定或方法名。owner是稳定句柄，不让FunctionData持有整个store或对象变量图。ObjectStore::remove仍删除对象；函数值可继续作为Rust值存在，调用时判InvalidObject。原生保存owner物理ref而Rust保留逻辑身份，这是首批已知差异，不宣称主动析构/延迟删除兼容。

不新增VmErrorKind、VmLimits、ReferenceKind、依赖或unsafe；不新增宿主start_function入口。外部合成测试经现有Object start进入05/17。

## 准备、目标和类型范围

PreparedProgram私有增加allow_function_values。两个新入口共用现有格式/边界检查，均不准备init，默认表达式白名单不增加05/17。Object入口限无继承、单constitution、无隐藏函数；globals=None不开35/36/全局07，Some按2AC能力开放。Global入口沿用schema/函数区间校验，允许05，**拒绝所有17**，因为全局函数值创建不在首批范围。

旧准备器仍拒绝05/17及专用tag9原型。新入口允许普通函数参数/返回type9，沿用原有nullable/可选/variadic属性检查；引用形参规则不扩展。新入口不开放06；已有host入口继续单独开放06，不新增“函数值+宿主”同程序组合入口。可通过已有07/08跨程序到其他显式能力程序，不绕过各程序的准备限制。

17在准备时仅接受par8 bit7=0、bit6=0，低6位为捕获数0..63；其他位组合UnsupportedFeature。signed par16必须非负且小于映像max_functions，否则InvalidProgram；必须有已准备的实际函数体，否则UnsupportedFeature。目标可为private函数，按编译槽捕获，不走名称/private/default_interface查找，也不因此新增访问控制。目标有引用形参时首批直接UnsupportedFeature。目标可沿用既有受限可选/variadic及默认表达式，但默认表达式本身仍不得创建或间接调用函数。

05在准备时仅接受par16=0或4、signed par8=1..127；不支持的par16为UnsupportedFeature，非法常量argc为InvalidProgram。二者对受支持Function目标行为相同；4不抑制错误。未来字符串分派才使用mayNull契约。每个准备错误须保留真实source/function/PC/成员offset。

所有运行时05/17先拒绝非opt-in、initialization/construction任务上下文，不允许通过内部辅助调用绕过生命周期限制。

## 17：捕获与发布

执行前保留原帧/PC，确认活的Object上下文、owner实际program、目标槽/函数体与所选代码身份一致。防御性坏槽为Bounds，缺执行体或不支持原型为UnsupportedFeature。不能用caller函数编号或对象方法目录代替目标槽。

按need(bound_count)保护参数/局部区，检查结果栈长度len-bound_count+1不超max_stack。按栈顶向下顺序检查全部捕获项：Reference/Loop为UnsupportedFeature，Object/Array/Mapping/Function为UnsupportedType；五种标量逐位/逐字节浅捕获，不转换或规范化，不复制额外引用槽。此限制刻意避免非标量捕获图、失效owner嵌套函数及原生独占/共享复制差异；不能宣称支持任意闭包环境。

使用try_reserve预留捕获Vec和提交所需栈容量，预分配结果Cell，再分配FunctionData/Rc。可控Cell分配失败或try_reserve失败为Limit，不消费捕获、不改根槽/PC。标准库OOM不承诺可恢复，已消耗的Cell ID不回滚。不新增max_function_values字段；每值捕获上限63，根存储/容器/栈/帧沿用既有限制，不把这些称为所有历史函数值的全局内存配额。

全部成功后将Value::Function写入预分配Cell，截断bound_count项，压入函数值，PC加1。bound[0]对应原栈顶。即使目标/捕获字段相同，每次17仍建立不同Rc头；复制同一结果共享头。

## 05：预检、实参排列和帧安装

signed par8的argn包含目标，所以need(argn)，新增实参为argn-1。目标必须是栈顶直接Value::Function；Bytes（含$前缀）、Object、Undefined、Reference或Loop均明确拒绝，不做名称查找或默认成功。错误类型：普通非Function为UnsupportedType，Reference/Loop为UnsupportedFeature。

检查顺序固定为：能力/任务上下文及防御性操作数（Bounds）→need(argn)→原始目标形态→origin token→owner存活→保存program与owner的代码身份→保存函数体/原型→新增实参原始形态→总量/类型/资源。foreign origin为InvalidState，已删除owner为InvalidObject，代码身份损坏为InvalidState；任何一步失败不消费函数值、参数、引用租约或PC。

合并后的逻辑参数为bound.clone()后接新增参数，新增参数n取stack[len-2-n]。首批拒绝新增Reference/Loop；新增普通Value可包括Function及现有容器，不扩大引用实参能力。捕获本身不可为Function/容器，因此不会引入Function→容器的引用环。

总argc=bound_count+argn-1，首批限制<=127；超出返回Limit，这是Rust范围限制，不是原生仅允许127。检查目标required/optional/variadic数量、参数类型及转换沿用现有普通用户函数契约（包括既有数值转换），不采用宿主06的精确类型契约。用逻辑参数视图完成预检，不先改原栈；全部数量/类型/栈/帧检查通过后才安排提交。引用参数目标在17已排除，运行时继续防御性拒绝。

在提交前分配新帧ID所需状态、所有实参Cell和Vec容量，验证容器限制。新实参区在Rust Vec中以逻辑参数的反向排列安装，替换原来的目标+新增实参区，保留前缀和caller的局部/参数槽。scalar捕获在独占/共享时均可克隆，允许不实现原生move优化；因所选捕获类型相同可观察值，这不是对任意捕获类型的等价声明。

成功后caller PC加1，安装ObjectProgram(saved program)、Some(saved owner)的新用户帧、pc=0及既有entry状态。执行目标自己的代码池/变量基址；返回走现有普通返回路径，恢复Object或Global调用者，不伪造全局对象。资源和参数预检错误指向05；进入callee后实际指令/返回错误指向callee。正常返回释放调用实参，函数值若仍有其他所有者仍可重复调用，捕获内容不被清空。

## 普通Value接点与兼容边界

- 存储/返回：允许Function作为普通Value在本地/对象/显式全局槽、Array元素、Mapping值、混合参数及新入口的tag9参数/返回中流动。Mapping键仍只支持Int/Bytes。现有成员写入将Function视为标量可写；由于捕获只含标量，不形成经捕获区反向连回容器的环。
- 复制：Rust Clone/宿主快照是无上下文身份共享，不主动规范化。VM的copy_value扩展为：同origin且owner已删除的Function复制为Undefined；foreign函数保持原值，留给05的origin检查，不能按碰撞ID判断其owner。read_reference复用先分配后比较/写回策略规范化源槽。其他现有copy接点沿用Object同样的既有源写回边界，**不声称全面模拟原生copyValue的提前源cleanup**；列入运行时差异报告。
- 真值：原始Function为真，包括保存owner已删除的原始值。条件/逻辑指令不得先对Function调用上述owner规范化；检查全部numeric::zero调用点，保留原有Object行为。已经由复制得到的Undefined仍为假。
- 比较：Value/FunctionValue的Rust Eq为身份。VM的1c/2a增加Function指针身份相等/不等；Function对其他普通类型为不等，不规范化owner，不比较捕获结构。其他算术、排序、cast保持UnsupportedType，不因tag9加入而扩展数值语义。
- 宿主：HostType不新增变体；Any可收Function，Mixed可返回Function，其他Exact种类仍拒绝；只作opaque转发，06不调用函数、不规范化owner，origin在05执行时才校验。host/container验证器对所选Function作叶值处理，因为bound仅含标量，不递归增加虚构深度。新增回归须核对这些公开影响。

## 错误、预算与验收

05/17放在解释器统一PC递增之前处理。17成功发布才加PC；05成功安装callee前给caller加PC。预检失败和创建失败保留原PC/操作数，run生成sticky failure；资源/类型/owner等失败均不新增RuntimeFaultSite，即使mayNull=4也不可被当前数值catch吞掉。进入callee后的既有数值故障仍可经现有catch解栈，保留真实帧身份；不扩展宿主、生命周期或新的错误来源。

每次05/17各消耗1条指令预算，内部无暂停点，零预算无分配或消费。05之后预算耗尽可停在callee入口，17之后可停在函数值已发布状态。暂停期间删除owner：后续普通复制可按上述规则成为Undefined，直接原始05报InvalidObject；不复活对象、不替换目标代码。清除/重绑Global代码不改变已有Object函数值的保存program。

后续实施必须覆盖：

1. 新旧准备入口、Global只05、所有17标志组合、signed槽/argc边界、private直接槽、缺体/引用原型、默认/链接/生命周期隔离。
2. 捕获0/1/63及全部标量、拒绝Reference/Loop/Object/Array/Mapping/Function；绑定[B0,B1]加新增[N0,N1]的参数顺序；总量127/128及可选/variadic检查。
3. 同一函数值克隆相等、独立创建不等，1c/2a/原始真值、失效owner原始与复制的差异，存储/返回/Array/Mapping值及禁止作为键。
4. 跨Object、Global caller→保存Object目标→Global恢复、不同代码池、07/08嵌套、传入/返回tag9、Host Any/Mixed仅opaque转发；foreign store即使编号相同也拒绝调用。
5. 分配/栈/帧/类型/owner预检失败前原子性及PC；调用后callee错误现场、数值catch释放年轻帧、引用lease前缀保留、函数头/Bytes捕获随最后所有者释放；不能借Rust结构相等掩盖不同身份。
6. 零预算、17后暂停、05后callee入口暂停、多次共享调用、暂停删除owner、start重置/into_objects保留origin、跨Global清除重绑不改变保存代码。

只有实现并通过这些测试后，才能将05/17加入限定支持集合，预期75种支持/73种主语料交集。即便编号全部相交，非标量捕获、global/efun/string/OS目标、完整继承、完整异常/生命周期及真实服务仍未完成，不能宣布阶段2完成。
