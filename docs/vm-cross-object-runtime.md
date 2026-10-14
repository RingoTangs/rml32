# 阶段 2Q：受限跨对象调用运行时

## 一、结果与范围

本阶段实现08 `vm_callInOtherFunEx`与41 `vm_pushThisObject`的限定路径，支持集合扩展至59种；237项Rust测试通过，其中新增25项合成回归。主语料73种指令与支持集合交集为57种，尚缺16种，另有4d/4e只在合成程序验证。上述数字是编号统计，不是功能完成率。没有可运行服务端，也没有原版差分执行证据。

原生依据复用[阶段2P跨对象静态设计](stage2-completion-evidence/cross-object-design.md)、[Ghidra证据索引](stage2-completion-evidence/rea-functions.md)及其新/旧压缩记录。关键函数：`vm_callInOtherFun@0x08094cd8`、`vm_callInOtherFunEx@0x080bd0fc`、`vm_pushThisObject@0x080975d8`、`vm_returnFromFun@0x08094978`。本轮未产生新REA查询，阶段2P证据保持原样。

当前[逐指令矩阵](stage2q-evidence/matrix.md)、[机器验证记录](stage2q-validation.json)独立保存，不覆盖2P快照。

## 二、公开接口与目录契约

```rust
pub struct MethodBinding {
    pub name: Vec<u8>,
    pub slot: i16,
}
// PreparedProgram 的消费式构建接口
pub fn with_methods(self, bindings: Vec<MethodBinding>) -> Result<Self, VmError>;
```

用法：在单体或继承程序准备成功后、注册ObjectStore之前，调用`.with_methods(vec![MethodBinding { name: b"read".to_vec(), slot: 0 }])?`。

- 方法目录必须由宿主明确提供，表示该合成环境的完整名称空间。默认未提供时，08返回UnsupportedFeature；显式空目录表示没有方法，允许进入缺失方法分支。
- 名称为非空、不含NUL、区分大小写的原始字节；非UTF-8允许。重复名称、未知/未实现槽返回InvalidProgram；不同名称可以指向同一槽。不能用函数扫描推导最终名称表。
- 槽为链接后的最终函数槽，private从实际函数体的attrib&4保留。绑定create/destruct名称、它们的别名或已登记生命周期槽均拒绝。
- 方法必须以0f `check_args`开头，保证预检后的实际数值转换、引用和默认入口检查会执行；原有固定值函数可省略0f的其他调用路径不因此扩展。
- 旧准备、注册和启动接口继续可用。作为08的目标时才要求方法目录；调用者不必提供目录，除非会被回调。

## 三、执行、返回与错误

### 08的限定语义

flags取par16，仅接受0、2、4、6；argc取有符号par8且非负。Rust栈从底到顶为`… argN-1 … arg0 method target`，与原生向低地址生长的栈对应。目标必须是直接、存活的Object，且不在Initializing/Constructing/RollingBack状态。方法值必须是Bytes；嵌入NUL明确拒绝，空方法名可进入普通查找失败/回退。

先按方法名查目标程序目录，flags&2允许private。查找失败或private不可访问时，按同一权限查`default_interface`。回退命中只移除target，保留原方法字符串作为新增arg0，再接原参数。两次均失败时，flags&4移除target、method及原参数并压入**Void**；无该位则返回InvalidProgram。坏目标、坏类型、目录缺失和参数错误不会被该位抑制。

新帧保存目标根program/object及实际函数槽，复用已有函数/变量基址和来源池。目标体中的07仍在目标对象内调用，41得到目标对象；普通返回后恢复调用者PC、对象、根程序、实际体及来源池。限制为非efun模式，原生ExReturn回调在该模式无操作，因此直接复用普通返回，无需仿造原生cookie或指针布局。

### 参数、引用与暂停

调用前检查操作数数量、目标、目录、方法、原型数量、值/引用绑定、引用owner和参数类型。类型预检只计算更新，不写槽；进入后由既有0f执行原子类型转换/规范化。跨对象引用保留原帧或原对象owner，不能改绑到目标对象。继承体和可选/可变参数继续遵守已有受限模型。

预算按原有指令边界暂停，08安装目标帧后可以尚未执行目标的第一条指令；恢复从该帧继续。普通终止错误保持既有保留帧/栈诊断，不做通用catch或事务回滚。方法查找、绑定与类型预检失败不消费操作数，IP仍已前进；原生错误现场栈可能不同。后续局部槽分配、返回类型或代码错误在目标帧报告，之前合法写入不回滚。

41在所有当前合法对象帧（包括初始化帧）压入Object，并遵守栈配额。原生无thisObject时压Int(0)，但当前全局帧不开放，不以虚构对象或Undefined代替。

## 四、测试与剩余边界

新增25项测试位于`tests/vm_core.rs`的Stage2Q区域，均为内存合成输入。代表场景包括：

- `other_call_round_trip_restores_program_pool_and_object`：A→B→A，逐层恢复字符串池与thisObject。
- `other_call_inherited_body_uses_target_bases_and_source_pool`：继承目标的变量基址与来源池；private继承和递归帧额度另有回归。
- `other_call_argument_and_fallback_orders_match_native_stack`：普通参数、fallback新增arg0及清理顺序。
- `other_call_reference_owner_survives_cross_program_and_nested_forwarding`：本地和对象引用跨程序及07转发；成员引用配额在返回后可复用。
- `other_call_entry_failures_preserve_operands_and_reference_targets`：错误位置、保留操作数、目标值与粘性错误。
- `other_call_lifecycle_preparation_rejects_direct_and_helper_calls`：初始化、构造及继承辅助函数均不能绕过08限制。

初始化/构造准备接口保守拒绝其任何已链接函数体中的08，即使当前入口未调用它；普通链接仍接受。字符串对象寻址、数组目标、efun返回模式、函数值、原生最终名称表、全局、通用catch、生命周期任务内跨对象执行、主动析构和execRef延迟删除继续不支持。宿主暂停期间调用已有remove只是立即删除，后续访问报错，不构成原生延迟删除兼容性。

## 五、复现与历史快照

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage2q-corpus-validation.json
python3 docs/stage2q-evidence/test_verify_completion.py
python3 docs/stage2q-evidence/verify_completion.py
python3 docs/stage2q-evidence/verify_validation.py
```

全量比较覆盖12包、9,527成员、7,159程序；唯一已知错误仍为`dba/pack_data/etc.pak/etc`的LZSS长度不符。检查仅验证静态结构与哈希，不执行业务字节码。

2P的矩阵、验证脚本和来源哈希保持冻结；其默认重算针对2P源码，当前运行会按预期报告过期，不能对它使用`--write`刷新。当前源码验收使用2Q目录的独立矩阵和验证记录。参考ELF、PAK、对象、脚本、Cargo依赖和全部既有分析证据保持原样。
