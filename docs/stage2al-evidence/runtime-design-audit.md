# 2AL Rust Mapping / loop integration audit

本文件是当前 Rust 源码的只读审计与后续设计约束，不是运行时实现，也不证明原生运行等价。原生事实引用已保存的证据；标为 **design** 的条目是 Rust 策略。

## 当前接入点

- `src/container.rs`：`MappingData` 使用 `entries: Vec<(Value, Rc<Cell>)>` 与 `BTreeMap<Key, usize>`。`member()` 对已有键返回原 Cell，对缺失键创建 Undefined Cell 后追加；当前没有删除接口。构造器按输入顺序处理，重复键更新原位置，末次值覆盖。
- `MappingValue::entries()` 返回当前 Vec 顺序的浅值快照；`get()` 读取稳定成员 Cell。这个 Vec 顺序不能充当原生桶链遍历顺序。
- `src/vm.rs`：私有 `LoopState` 仅有 Numeric / Array；`loop_instruction()` 对 Mapping997 明确返回 `UnsupportedFeature`。`LoopControl` 持绑定引用和私有状态，不是公开 Value。
- `src/cell.rs`：`replace_payload_batch()` 先取得全部 Cell 的可变借用，再替换；重复 Cell 或借用冲突在写入前失败。所有旧 payload 在释放 guards 后销毁。

## 原生契约与设计建议

原生循环依据 `vm_cmpAndJmpInLoop @0x080bcae0`，完整既有证据 ID 为 `ev_2f7c7986819c8b0acae9e1cddb895fded61ac3d31a2c036489d87c95592ebcce`；来源说明见 `../vm-control-flow-semantics.md`。

- **native**：游标由 bucket、桶内 subindex 和 ordinal 构成。每次检查从当前桶链头重新扫描到记录位置；不能替换为持有上次节点的稳定节点迭代器，也不能提前取 `entries()` 快照。插入、删除或重哈希的影响须遵循实际扫描规则。
- **native**：每次有效检查读取绑定变量的当前值。当前值是长度 3 的 Array 时复用它，否则分配新的长度 3 Array，写出 `[key, value, ordinal]`。复用会修改该数组的既有别名；循环体若把绑定变量换成另一个长度 3 数组，下一次必须复用新数组。不能在 LoopState 永久缓存首个输出数组并强制复用。
- **native**：控制块持有集合 header，源变量重绑不改变该集合；耗尽不自动清除控制槽。ordinal 在检查开始时推进，包括耗尽检查；其溢出与耗尽后的重复调用行为应保留为案例。
- **design**：新增私有 Mapping 状态保存 owner 与上述位置计数。桶链拓扑独立于公开 `entries()` 的快照次序；保留后者当前兼容行为。节点或值槽在覆盖、插入、扩容时保留成员 `Rc<Cell>` 身份，但节点身份不是游标推进规则。
- **design**：现有 `ArrayData` 的固定形状与可变元素 Cell 已能表达三元数组原地复用，无需公共 Value 新变体。复用决定必须在每次检查重新读取绑定值后作出。

## 错误提交与别名风险

- **design**：先准备下个位置计数、选择或分配本次输出数组、计算三元值及需要的死 Object / Function 规范化，再检查目标可写及跳转范围。最后将控制槽、绑定槽、输出成员槽和需规范化的源槽一起提交。不能先修改共享数组再发现控制槽借用失败。
- **design**：批量更新的重复 Cell 不应被隐式当作末次覆盖。输出数组元素、源值槽与绑定槽可能别名时，应先按原生证据确定写入次序，再决定去重或明确拒绝；未取证组合继续保持受限边界。
- **design**：新值分配失败、映射或 Cell 借用冲突、readonly 或目标失效，不应部分更新可见三元数组或控制计数。此受控错误原子性是 Rust 策略，不宣称原生分配失败也回滚。
- **design**：释放旧值须在释放全部 RefMut 后进行。`MappingData::drop()` 会清理其值 Cell，持 guard 时释放最后一个 owner 会带来重入借用风险。
- **当前 Rust 边界**：`writable()` 拒绝向 Array/Mapping 成员引用写入容器值；23 也拒绝 Owned 绑定和控制槽别名。不能借 Mapping997 开放成员绑定、循环容器或生命周期循环。

## PC 与验收重点

`step_instruction()` 在派发 `12/23/50` 前执行 `pc += 1`；`loop_instruction()` 成功后才写入计算出的分支 PC。故失败时“数据未提交”不等于“内部 PC 回滚”。后续规格须保留这一事实，并单独验证公开诊断指向原指令；不要照搬 Bytes 普通二元操作的失败保留 PC 策略。

实施验收应覆盖：首次数组分配；同一数组别名更新；循环体替换为另一长度 3 数组；替换为非数组或其他长度数组；碰撞桶的链位置重扫；覆盖及扩容不改变既有成员引用身份；遍历期间源变量重绑；空集合与重复耗尽；控制/目标/输出元素借用冲突和分配失败；readonly、失效对象与无效分支；控制槽覆盖和帧退出释放 owner。节点删除与重哈希案例依赖本轮原生取证，不从 Vec 当前行为推导。
