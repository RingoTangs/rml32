# 阶段 2D：成员引用、赋值与 Rust 实现设计

## 1. 总体结论

本阶段完成静态证据分析及下一阶段设计，**没有实现写操作或修改现有公共 API**。当前 Rust 仍为阶段 2C：43 种限定操作码、73 项测试。原始 ELF、启动脚本、PAK 及业务字节码均未执行或修改。

分析引擎为 REA MCP / Ghidra 12.1.4，目标 `magic_Linux32` 的 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新增 68 条证据，复用 1 条原始分派器证据。完整参数、配置和局限保存在 [证据索引](vm-reference-evidence/index.md)、[反编译/汇编/引用](vm-reference-evidence/functions.md)及压缩证据包中。

关键结论：

- **已确认**：tag 8 表示指向独立 `Vm_Value_T` 值槽的引用。普通复制增加引用计数，不会深复制数组或映射；成员写入路径未观察到写时复制。
- **已确认**：成员引用需要一个容器值槽引用，而不是普通容器值。映射缺失键在取得引用时即插入，初始值为 Undefined。
- **已确认**：数组成员引用和映射成员引用的释放行为不同。数组释放递减元素槽引用计数；被外部引用的映射节点进入延迟释放路径，但键和值会先被清空。
- **推导**：多个变量持有同一容器时，成员写入对其他别名可见；变量本身重新绑定不等于替换此前取得的成员槽。
- **未知/不保证**：逃逸的栈槽引用和已销毁对象变量引用的安全性。相关路径清理或释放原存储，没有观察到统一的安全句柄失效机制。

因此建议首批 Rust 写操作采用**稳定值槽、共享容器、受限引用生命周期**，不采用 `Rc::make_mut`。首批只允许向容器成员写入现有标量值；容器作为变量赋值和嵌套读取仍允许，容器值写入成员、循环图及引用逃逸暂时拒绝。

## 2. 指令、栈布局及证据

以下 `[..., lhs, rhs]` 按 Rust 栈从底到顶描述；原版 `vm_sp` 向低地址增长。`Ref(cell)` 表示 tag 8，不表示宿主地址。操作数 `a` 为有符号 par16；无参数操作码的原始 par16/par8 不具有生效参数含义。

动作表来源为已有 `docs/evidence/actions.csv`。原始 [vm_regularProcessor](vm-reference-evidence/functions.md#vm-regularprocessor) 在取节点后先增加 ip，再根据 actionInfoTable 的参数数调用函数；因此不能只凭函数名或节点邻接推断栈语义。新查询捕获 `0x083cc6dc → vm_assignValue`、`0x083cccc4 → vm_pushLocalVptr`、`0x083ccdf8 → vm_queryMemberPtr` 的 DATA 引用，分别对应动作记录入口字段。分派是间接调用，直接 callers 为空不表示不可达。

| 指令 | 函数地址 | 作用/栈变化 |
|---|---|---|
| `03` | `vm_assignValue` `0x0808f634` | `[..., value, Ref(cell)] → [..., value]`，清理并写入目标，消耗引用 |
| `04` | `vm_assignValueReverse` `0x0808e914` | `[..., Ref(cell), value] → [..., value]`，结果移入原引用位置 |
| `10` | `vm_checkIsRefType` `0x08091750` | 按 a 检查参数/局部槽是否原始 tag 8，压入 Int 0/1 |
| `11` | `vm_checkIsRefTypeI` `0x08091850` | 栈顶 Int/Real 截断后减 1，消费索引，调用 `10` |
| `13` | `vm_convertToPtr` `0x0808a220` | Array/Mapping 值转为堆上临时值槽的引用；已有引用保持不变 |
| `14` | `vm_convertToValue` `0x0808c028` | `Ref(cell) → copy(cell)`，释放引用，单层解引用 |
| `22` | `vm_incValue` `0x0808dea4` | 消耗引用，目标 Int/Real 增加 a；不留下表达式结果 |
| `2f` | `vm_opEqValue` `0x0808eb68` | 目标引用在下、右值在上；读取、运算、写回并保留结果 |
| `36` | `vm_pushGlobalVptr` `0x080910d0` | 获取全局值槽引用，本阶段仅分析 |
| `39` | `vm_pushLocalVptr` `0x0809134c` | a 指定参数/局部槽；非引用槽创建引用，引用槽复制原引用 |
| `3a` | `vm_pushLocalVptrI` `0x080915dc` | 栈顶索引截断并减 1，再调用 `39` |
| `3e` | `vm_pushObjVptr` `0x080911a8` | 当前对象 constitution 的 baseVarOffset+a 值槽引用 |
| `43` | `vm_queryMember` `0x0808fa60` | `[..., container, key] → [..., copy(member)]`；缺失映射键不插入 |
| `44` | `vm_queryMemberPtr` `0x0808f8e4` | `[..., Ref(container_cell), key] → [..., Ref(member_cell)]` |
| `46` | `vm_queryRangePtr` `0x0809005c` | 无条件报错 “Can't modify range of a value.” |
| `4d/4e` | `vm_setLocalVar` `0x0808f6dc` / `vm_setObjVar` `0x0808f7f8` | 保留栈顶；局部引用参数写穿透，对象变量直接替换槽内容 |

具体伪代码和完整 Evidence ID 按表中函数名见证据文件。地址由动作表和本轮函数证据交叉定位，完整查询记录保留于证据包。

### 2.1 赋值表达式

以下为分析者整理的 C 风格伪代码，不是恢复的源文件。对应 [普通赋值](vm-reference-evidence/functions.md#vm-assignvalue)、[反向赋值](vm-reference-evidence/functions.md#vm-assignvaluereverse)。

```c
// opcode 03: top points to destination; RHS is one slot below top.
void assign_value(void) {
    Cell *dst = top().ptr;
    Value *rhs = below_top();
    cleanup(dst);
    copy_value(dst, rhs);
    unref_cell(dst);
    clear_and_pop_top();          // RHS remains the expression result
}

// opcode 04: RHS is top; destination reference is below top.
void assign_reverse(void) {
    Value *rhs = top();
    Cell *dst = below_top().ptr;
    cleanup(dst);
    copy_value(dst, rhs);
    unref_cell(dst);
    replace_two_stack_values_with(move(*rhs));
}
```

汇编证据：`0x0808f64a` 从栈顶 payload 取目标，`0x0808f651` 取下一 24 字节槽作为右值；`0x0808f654/65d/665` 依次 cleanup/copy/unref。反向赋值在 `0x0808e931` 从下一槽取目标，在 `0x0808e94b–95a` 将右值移入结果槽。两函数没有独立的 const 或目标 tag 检查；不能据此假定任意畸形字节码都有原生受控错误。

### 2.2 成员引用与提前插入

```c
Reference member_ref(Reference owner_ref, Value key, int flags) {
    if (owner_ref.attrib & 1) error("const");
    Cell *owner = owner_ref.cell;
    Cell *member;
    if (owner->type == ARRAY) {
        int i = key_is_real ? trunc_to_i32(key) : require_int(key);
        int offset = (flags & 1) ? owner->array->size - i : i;
        require(0 <= offset && offset < owner->array->size);
        member = owner->array->cells[offset];
    } else if (owner->type == MAPPING) {
        member = find_for_insert(owner->mapping, key, /*clear_old=*/0);
    } else error("not writable container");
    ref_cell(member);             // done before releasing original owner cell
    unref_cell(owner);
    consume_key();
    return Ref(member);           // retains reference attributes
}
```

[成员引用汇编](vm-reference-evidence/functions.md#vm-querymemberptr)：`0x0808f907` 检查引用 attrib bit 0；`0x0808f95b–961` 传入 doTheFree=0；数组在 `0x0808f9d3` 检查 flag bit 0，`0x0808f9e2` 计算 size-index，`0x0808f9fc–fa08` 取元素槽并增引用，之后 `0x0808fa0b` 释放原容器槽。flag=1 的 index=1 表示最后一个元素，index=0 越界；不是 Python 负索引。

[插入助手](vm-reference-evidence/functions.md#vm-safefindforinsert) 对已存在键保留值；新节点先 `vm_initValue` 再复制 key，故即使后续写入失败或只解引用，也已产生一个 Undefined 值节点。普通字符串键匹配复用阶段 2C 已确认的大小写敏感字节语义；特殊映射属性不在首批范围。

### 2.3 变量引用和函数参数

[局部引用](vm-reference-evidence/functions.md#vm-pushlocalvptr) 使用 `bp + a`，符合原版向下增长栈；非引用槽增加该槽 ref 并复制 attrib，已有 tag 8 参数直接复制引用。`vm_pushLocalVar` 则复制后对 tag 8 调用 `vm_convertToValue`，得到普通值。[vm_setLocalVar](vm-reference-evidence/functions.md#vm-setlocalvar) 在目标槽是 tag 8 时写入其引用目标，并拒绝引用右值。这是引用参数写穿透的直接证据。

[vm_checkArgNum](vm-reference-evidence/functions.md#vm-checkargnum) 的 flag bit 0 要求实参为引用，并检查被引用值类型；普通有类型参数拒绝引用。mixed 普通参数的分支并非同样拒绝，不能把 typed 参数规则泛化到 mixed。首批实现仍拒绝所有引用参数、const 参数及 const 返回值。

## 3. 别名、释放与置信度矩阵

“已确认”表示静态代码直接包含该分支；“推导”表示由已确认的地址共享及调用顺序组合推出；均不表示观测过原版运行。

| 场景 | 结论 | 级别与证据 |
|---|---|---|
| 复制数组/映射 | 同一个 header 增 ref，不深复制 | 已确认，`vm_copyValue` |
| 两变量共享容器，一方写成员 | 另一变量通过同一槽读到新值 | 推导，copyValue + queryMemberPtr + assignValue |
| 已有变量引用，变量重赋值 | 引用仍指向变量槽，随后看到新内容 | 推导，pushLocalVptr/pushObjVptr + assignValue |
| 已有成员引用，原变量改绑另一容器 | 引用不重新按变量/索引解析；目标是旧成员槽 | 已确认目标身份；旧容器释放后的值依以下两行区别 |
| 数组最后一个 header 所有者释放 | 元素槽仅 unref；尚有外部引用的元素保留内容 | 已确认，`vm_unrefArray` `0x0809b350` 调用 `vm_unrefValue` `0x0808bfb4` |
| 映射最后一个 header 所有者释放 | `vm_freeNode` 清空被引用节点的键和值，节点排入延迟释放链 | 已确认，`0x0809c349–367`；引用此时读到 Undefined 是推导 |
| 临时容器 `13 → 44` | 释放临时 owner 槽可能立即释放 header；数组元素可存活，映射值可能先被清空 | 推导，convertToPtr + queryMemberPtr + 两种析构路径 |
| 映射缺失键 `44 → 14` | 插入节点，得到 Undefined，sizeof 增加 1 | 推导，safeFindForInsert + convertToValue |
| 栈帧退出且引用逃逸 | takeOffValueFromStack 清理旧槽并搬移结果，未观察到栈槽提升或引用重定位 | 已确认清理路径；逃逸后的安全读写未知，不保证有效 |
| 对象销毁且变量引用逃逸 | 析构静态值槽后释放 pObjectValues；未观察到靠值槽 ref 延迟该存储释放 | 已确认所查路径；不能保证引用可用 |
| const 容器的成员引用 | ref.attrib bit 0 时 `44` 报错；读取 `43` 将 owner attrib 传给结果 | 已确认；不是完整 const 安全证明 |
| 字符串/Buffer/范围写入 | `44` 对 String/Buffer 报错；`46` 总是报错 | 已确认 |
| 自引用/容器环 | 赋值路径没有环检查；原版存在按 active 标记遍历及垃圾值清理 | 可构造环为推导；特定环回收完整性未验证 |

生命周期证据补充：

- [vm_unrefValue](vm-reference-evidence/functions.md#vm-unrefvalue)：ref 归零才清理、移出 allValues 并释放独立值槽。
- [vm_freeNode](vm-reference-evidence/functions.md#vm-freenode) / [延迟节点释放](vm-reference-evidence/functions.md#vm-freeobsoletedmapnodes)：没有外部引用时销毁静态槽并释放节点；否则先 cleanup 键和值、延迟释放节点。它与数组元素的独立分配策略不同。
- [vm_takeOffValueFromStack](vm-reference-evidence/functions.md#vm-takeoffvaluefromstack) `0x08092008`：`0x0809205d` 清理待移除槽，`0x08092090–d6` 搬移结果字段，没有对应外部引用重绑定步骤。[vm_returnFromFun](vm-reference-evidence/functions.md#vm-returnfromfun) 使用该函数。
- [vm_destructObjectValues](vm-reference-evidence/functions.md#vm-destructobjectvalues) `0x080b11a8` 及 `vm_destructObject`：逐槽调用 `vm_destructStaticValue` 后释放变量数组；正在执行的对象可能通过 execRef 推迟析构，不代表所有引用都保活对象。
- [垃圾值处理](vm-reference-evidence/functions.md#vm-f_garbage_collect)、`vm_refreshAllValuesActive` 和 `vm_markActiveFromValue` 确认了标记/遍历及清理入口。观察到的 marker 没有通用 tag 8 递归分支；不能宣称已证明其对任意引用图都能正确回收。

## 4. 实际制品中的优先级

[统计报告](vm-reference-evidence/corpus-usage.json)直接读取既有、带 SHA-256 的静态基线，覆盖 7,159 个程序、101,592 个普通函数和 6,474,773 个节点。方法和边界见 [统计说明](vm-reference-evidence/corpus-method.md)。

| 路径 | 节点次数 | 后续优先级 |
|---|---:|---|
| `39` 局部引用 | 278,565 | 首批 |
| `03` 普通赋值 | 250,825 | 首批 |
| `44` 成员引用 | 39,649 | 首批，mode 0=39,639，mode 1=10 |
| `3e` 对象变量引用 | 27,139 | 首批，仅当前对象、无继承 |
| `2f` 复合赋值 | 17,336 | 第二批，单独验证内部操作码和算术 |
| `22` 自增减 | 4,239 | 第二批 |
| `36` 全局引用 | 3,929 | 与全局程序生命周期一起实现 |
| `04` 反向赋值 | 169 | 首批，复用同一写入基础设施 |
| `11/3a/46/4c/4d/4e` | 各 0 | 零次不代表原版不支持；不以此删掉现有功能 |

`44 → 03` 的段内邻接有 33,193 次，说明该序列值得优先覆盖；没有进行控制流或栈来源分析，不能把这些序列直接认定为数组写或映射写。重复分发的库按各包分别计数；以上不是运行频率，也不是业务可达性证明。

## 5. 下一阶段 Rust 设计（尚未实施）

### 5.1 首批准确支持范围

在原有 43 种指令上增加 `03/04/13/14/39/3e/44` 七种限定路径，共 50 种；同时将已有 `43` 的 Array 查询扩展到 mode 1。`44` 只接受 mode 0/1：数组执行普通/size-index；普通映射按原版忽略这两个 mode 的区别。其他 flag 位明确 Unsupported，不声称原版也拒绝。

- 引用只在 VM 当前执行及仍有效的调用帧之间存在。先继续拒绝引用参数、引用返回值、引用作为容器元素/映射键值及外部实参；普通容器参数和返回保持支持。
- root 变量/临时槽可写现有普通 Value，包括 Array/Mapping；**数组/映射成员槽只接受 Int、Real、Undefined、Void、Bytes、Object**。允许沿现有嵌套容器链取得成员引用并写标量，拒绝向成员写容器值。这避免新增容器图边和循环，不需要本批实现 GC。
- 键仍限 Int/Bytes，String/Buffer 写入和 `46` 范围写明确 Unsupported。`22/2f/36/10/11/3a` 暂不放入支持集合。
- 所有读写先验证栈形状、引用目标、类型、边界及容量，再提交改变；缺失映射键在 `44` 成功时就插入 Undefined，后续赋值失败不回滚这次插入。
- 未确认的原版逃逸/悬空行为转为带位置的受控错误，不复现越界内存访问。原版析构先于复制的异常自别名情况不作为首批兼容承诺；Rust 先快照右值，再替换目标。

### 5.2 稳定值槽与存储方案

定义 `Cell = RefCell<Value>`、`CellHandle = Rc<Cell>`，使用标准库表示稳定值槽，Array header 保持共享身份并存放 Cell 列表；Mapping header 内部索引可变、键不可变，索引映射到稳定 value Cell。重新分配索引不得改变 Cell 身份。**不使用写时复制，不用数组下标作为引用的持久目标身份。**

变量存储也使用稳定 Cell。引用描述符与普通 Value 分开，VM 内部栈改为 `StackEntry::Value(Value)` / `StackEntry::Reference(Reference)`；Reference 的运行时 tag 为 8，`Value` 公共枚举不增加可供宿主构造的引用变体。

Reference 分三类：

1. `Local { frame_id, cell: Weak<Cell> }`：每次访问检查仍活跃的单调 FrameId；栈槽复用不会复活旧引用。获取已有引用参数的转发行为留待引用参数阶段。
2. `Object { object_id, cell: Weak<Cell> }`：每次访问验证对象存在；销毁后报 InvalidObject。
3. `Member { cell: Rc<Cell>, kind: Array|Mapping }` 或 `Temporary { cell: Rc<Cell> }`：独立保活目标槽，不保活原变量或 header。数组 header 释放只释放其 Cell 所有权；映射 header 最后释放时先清空所有 value Cell，仍被引用的脱离节点槽可继续承载首批标量值。

`44` 必须先保活新成员槽，再释放旧引用；这保证临时容器销毁时的数组/映射差异。引用描述符携带只读属性；首批未开放 const 输入，但内部检查点保留，不能让后续开放 const 时绕过成员写限制。

Cell/Mapping 的 RefCell 借用只存在于单次读取/更新的小作用域。先克隆所需值、释放借用，再执行清理或 drop 旧值；不得跨调用、header 释放、对象移除或 VM 步进持有借用。运行期借用失败使用 try_borrow/try_borrow_mut 转受控 InvalidState。映射最终清理需由统一的 header 释放路径完成，测试必须覆盖 retained Cell 被置 Undefined；不得仅依赖 Rc 自动释放而保留错误旧值。

成员写仅删除或替换为标量，不增加容器边。因此现有无环图仍无环，深度只可能降低。删除当前缓存 depth，改为有界递归计算（图最大深度仍 64），避免别名写入后缓存过期。新建容器仍检查 65,536 项和深度 64；映射插入前检查 distinct key 容量。另增每次执行最多 65,536 个存活临时/脱离成员槽的限制，超限为 Limit；这些限制是 Rust 策略，不是原版数值。

### 5.3 公共 API 迁移

本节是下一阶段接口规格，本轮没有实施；项目尚未发布稳定 API，直接迁移并更新调用者，不保留会失效的借用切片接口。

| 当前接口 | 下一阶段接口/规则 |
|---|---|
| `Value::array` / `Value::mapping` | 签名保持；构造内部稳定 Cell，保持原有输入顺序/重复键规则 |
| `ArrayValue::elements() -> &[Value]` | `elements() -> Result<Vec<Value>, VmError>`，返回成员值快照 |
| `MappingValue::entries()` / `get()` | 分别返回 `Result<Vec<(Value, Value)>, VmError>` / `Result<Option<Value>, VmError>` |
| 容器 `depth() -> usize` | `depth() -> Result<usize, VmError>`，按当前内容计算 |
| `Object::variables() -> &[Value]` | `variables() -> Result<Vec<Value>, VmError>` |
| `Vm::stack() -> &[Value]` | `stack() -> Result<Vec<StackSnapshot>, VmError>`；快照项为普通 Value 或引用的 kind/目标 ID/有效性信息，不暴露可写引用 |
| `Completion::Returned(Value)` | 保持；返回 Reference 时 UnsupportedFeature，不隐式解引用 |

快照克隆成员值；嵌套容器仍共享身份，不承诺深拷贝或跨 VM 步进不变。宿主只能读，写操作通过 VM 内部函数提交。Value 的容器 Eq 继续按 header 身份；不能用深比较替代。新构造单调 CellId/FrameId 只供诊断，不是原始指针、线协议或序列化地址。

### 5.4 实现顺序及任务验收

| 任务 | 交付及依据 | 验收条件 |
|---|---|---|
| E1 稳定 Cell 与快照接口 | 迁移容器、变量、栈内部表示；依据 copyValue/pushVptr | 现有 73 测试完成等价接口迁移并保持行为；内部无 unsafe |
| E2 变量引用和两种赋值 | `39/3e/03/04`、有效性校验；依据地址/栈证据 | 局部/对象写后读取正确；两种赋值保留右值；对象隔离/失效错误正确 |
| E3 临时引用与读取 | `13/14`、单层解引用；依据 convertToPtr/Value | 转换不深复制；标量不允许 `13`；引用重复转换不增一层 |
| E4 成员引用和受限写入 | `44` mode 0/1、`43` mode 1；依据 queryMemberPtr及插入助手 | 下表行为测试全部通过，缺失键提前插入；共享写对别名可见 |
| E5 释放顺序和资源审计 | 数组/映射的独立释放路径；依据 unrefArray/freeNode | 临时容器销毁差异成立；帧/对象失效受控；无环限制和资源限制成立 |
| E6 回归与文档更新 | 支持清单 50、API 调用示例、限制及验证记录 | Debug/Release、fmt、严格 clippy 与全量静态制品比较通过 |

## 6. 后续合成测试规格

以下是下一阶段要编写的测试，不是本轮已执行的功能验证。全部使用内存构造程序，禁止运行实际业务程序。

| 场景 | 预期结果/判定来源 |
|---|---|
| `03` 的 value/Ref、`04` 的 Ref/value | 正确写目标，留下一个 RHS；参考赋值汇编 |
| 两变量共享 `[1]`，经一方写元素为 9 | 两方读取都是 9，header 身份不变；别名推导 |
| 成员引用取得后原变量改绑 | 仍指向旧槽；旧数组外部引用保活元素，旧映射最终释放先清空元素 |
| `mapping_ref → missing_key → 44`，未赋值先 sizeof | 键数立即加 1；后续 `14` 得 Undefined |
| 已有映射键取引用但不写 | 值不变，键数不变，重复取引用指向同一槽 |
| 临时数组/映射经 `13 → 44 → 14` | 无其他 header 所有者时，数组得到原元素，映射得到 Undefined |
| `[nested]` 沿两次 `44` 写内层标量 | 共享内层别名看到结果；不复制父容器 |
| mode 1，size=3，index=1/0/4 | 分别选最后元素、Bounds、Bounds；Real 1.9 截断为 1 |
| 非 UTF-8 Bytes 键、大小写差异 | 按完整字节匹配，既有键不重复计数 |
| String/Buffer/非法键/flag/无引用/不足栈值 | 按支持边界返回 UnsupportedType/UnsupportedFeature/StackUnderflow，不 panic |
| 向成员写容器、构造自环、引用参数或引用返回 | 显式拒绝，不改变目标；属于 Rust 首批策略 |
| 引用跨活动嵌套调用、预算暂停恢复 | 在合法帧存活期间目标一致；暂停不释放引用 |
| 帧退出后旧 Local 引用、对象销毁后引用 | InvalidState / InvalidObject，不因槽复用或 ID 复用而复活 |
| 最大项数、深度、临时槽数量和 max_stack | 边界成功，超限 Limit；错误含 source/program/object/function/pc/offset |
| 成员由嵌套容器改成标量后 depth/再构造 | 动态深度降低，后续构造不使用旧缓存 |

## 7. 本轮验证、剩余未知与完成范围

本轮只新增本报告、证据包、语料统计和验证记录，原有源码/API/测试及旧证据不变。验证记录见 [stage2d-validation.json](stage2d-validation.json)。语料统计与旧汇总交叉核对；36 个原始输入及所有本轮开始前已跟踪文件逐个 SHA-256 校验。没有因为文档变更重复进行 VM 行为或服务启动测试；73 项测试结果是阶段 2C 基线，不伪称本轮新增测试覆盖了引用写入。

| 原计划问题 | 状态 |
|---|---|
| 指令映射、栈顺序、地址及证据 | 已回答 |
| 普通数组/映射共享写与缺失键行为 | 静态证据已回答，无运行对照 |
| 引用复制、释放、临时容器和局部/对象销毁 | 主要路径已回答；逃逸后安全性未确认 |
| const、引用参数、复合赋值及特殊模式 | 所查分支已记录；完整语言约束和特殊映射未完成 |
| 制品使用优先级 | 已完成静态节点统计，不作动态频率结论 |
| 后续实现范围、接口迁移、测试验收 | 已给出明确的受限首批设计 |

仍需独立阶段解决：容器成员写入容器值/循环图、GC 对所有引用边的完整性、引用参数与返回值、const 全路径、全局变量初始化及生命周期、compound 操作细节、跨对象/继承和实际业务执行等价。对这些未知保持 Unsupported，不能把本阶段完成解释为完整 VM 或可运行服务端已经重写完成。
