# 阶段 2AG：受限 Object 函数值运行时

本阶段落实 [2AF 实施规格](stage2af-evidence/function-runtime-design.md)，新增普通 Object 函数值及受限 `05/17`。原生依据复用[2AF 静态语义](vm-function-value-semantics.md)和冻结证据，没有新开原生分析会话，也没有执行参考 ELF、脚本、初始化代码或包内业务字节码。合成程序经现有格式解析后测试。

## 公共接口及能力入口

新增公开 `Value::Function(FunctionValue)`，`tag()` 为9，下游穷尽匹配需增加该变体。`FunctionValue` 提供 `owner()`、`program()`、`function()`、`bound_count()`；无公开构造、可变捕获或序列化接口。Clone共享函数头，Eq比较Rc身份，Debug只显示上述元数据。

```rust
pub fn prepare_program_with_function_values(
    image: ProgramImage,
    globals: Option<GlobalBankSchema>,
) -> Result<PreparedProgram, VmError>;
pub fn prepare_global_program_with_function_values(
    image: ProgramImage,
    globals: GlobalBankSchema,
) -> Result<PreparedProgram, VmError>;
```

两入口及FunctionValue从库根导出。Object入口要求无继承、单constitution、无隐藏函数；Global入口只开放05，不允许17。均忽略init，不扩大默认表达式白名单，不开放06。旧准备入口仍拒绝05/17及专用tag9原型；宿主程序可通过已有调用通道与显式函数值程序互通。内部用PreparationOptions集中传递能力，避免已有准备路径意外获得新能力。

## 创建、调用和事务边界

17仅接受bit7/6均零、低6位捕获数量0..63；目标为已准备的Object函数直接槽，可以是private，禁止引用形参。待捕获项按栈顶向下顺序保存，限Undefined/Void/Int/Real/Bytes，不转换Real位模式或Bytes内容；原始Reference/Loop报UnsupportedFeature，其他捕获类型报UnsupportedType。先检查、预留容量并分配结果Cell，再发布函数值并推进PC。

05仅接受par16为0/4、signed argc为1..127，argc包括栈顶目标。先检查能力/任务、操作数、计算栈数量、目标原始形态，再检查store身份、owner存活、保存代码和原型、新参数原始形态、合并数量/类型/资源。非Function值报UnsupportedType，Reference/Loop报UnsupportedFeature，foreign store报InvalidState，已删除owner报InvalidObject。4不吞错误。

每个ObjectStore（含Default/new）独立分配origin token，into_objects保留它，编号碰撞不能跨store执行。函数头仅保存稳定owner/program/function句柄及标量捕获，不持有整个store。程序注册表继续保留代码。

逻辑参数为捕获[B0,B1]再接新增[N0,N1]，底层Vec反向安装。总量<=127，使用普通用户函数类型转换、nullable、默认及variadic契约。预检并分配全部实参Cell和帧/栈容量后才提交；保留调用者前缀、局部槽和引用lease。可恢复失败不消费操作数、不推进PC、不消耗帧ID；临时Cell ID允许已消耗。标准库OOM不承诺恢复。

成功后调用者PC前进，安装保存Object/program的普通帧，返回恢复Object或Global调用者。每条05/17消耗一条预算，无内部暂停点；零预算无执行，17后可暂停在函数值已发布状态，05后可暂停在callee入口。预检错误保留05现场，callee错误报告其自己的函数/PC/成员offset。仅callee中的既有数值故障可进入现有catch，函数值错误不增加catch来源。

## 值流动、失效和明确差异

Function可在本地、Object/显式Global槽、Array元素、Mapping值、普通参数/返回中流动，Mapping键仍限Int/Bytes。容器深度和宿主值校验将本批标量捕获Function视为叶值；不构造经捕获区回连容器的循环。HostType不变，Any/Mixed仅opaque透传，其他Exact类型拒绝。

VM普通复制将同origin且owner已删除的Function变为Undefined；foreign值保持原样，由05检查身份。read_reference先分配结果再规范化源Cell。Rust Clone及宿主快照不规范化。宿主06可返回原始失效值，但随后普通VM返回会经过复制并成为Undefined。

原始Function真值始终为真，21/2d不先规范化；1c/2a直接比较函数头身份，Function对其他普通类型不等。普通类型查询等既有复制接点继续沿用其复制行为。其他算术、排序、cast继续拒绝。

已知差异：ObjectStore::remove立刻删除逻辑对象，不模拟原生owner物理ref/execRef及活动对象延迟删除；不全面模拟原生copyValue提前清理源槽。标量捕获以共享头/浅克隆实现，不实现原生唯一所有者move优化。非标量捕获、Global/efun/string/OS目标、完整继承/异常/生命周期、hook、调度、GC/JIT及真实服务均未完成。

## 验收及复现

473项Rust测试通过（52项库单元、23项解析/CLI、398项VM），其中本批增加40项。包含标志/槽/argc边界、private槽、捕获位/字节、可选/variadic、tag9传参返回、存储/容器、Host透传、Global及跨对象恢复、删除/foreign origin、预算/重绑、数值catch、隐藏Loop/Reference拒绝、可控Cell失败原子性和最终所有者释放。

当前支持75种限定操作码，主语料73种编号均有交集；所有矩阵行仍为bounded，分支覆盖未测量，未执行主语料。编号交集不构成阶段2完成或原版行为等价证明。

[本阶段矩阵](stage2ag-evidence/matrix.md)、[方法](stage2ag-evidence/method.md)与[验证记录](stage2ag-validation.json)保存复算与边界；2AE/2AF及以前的证据和哈希保持原样。
