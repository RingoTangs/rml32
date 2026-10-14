# 阶段 2E：受限引用写入与接口迁移

## 总体结果

在阶段 2C 的 43 种限定指令上增加 `03/04/13/14/39/3e/44`，现在共支持 **50 种操作码的限定路径**；数组普通读取 `43` 同时支持 mode 1。本阶段实现的是 VM 库，CLI 仍仅静态检查制品，没有运行完整服务端。

语义依据为[阶段 2D 的 REA/Ghidra 静态分析](vm-reference-semantics.md)及其[证据索引](vm-reference-evidence/index.md)。复用既有 68 条记录，没有重复打开分析会话，也没有执行原始 ELF、启动脚本或业务字节码。执行验证仅使用 Rust 中人工构造的程序。使用标准库 `Rc`、`Weak`、`RefCell`、`BTreeMap`，没有新增依赖或 unsafe。

## 指令与共享行为

下表中的栈顺序从底到顶；Ref 只在内部栈存在，不是公共 Value，也不是宿主地址。

| 指令 | 实现行为 | 静态依据 |
|---|---|---|
| `03` | `[value, Ref] → [value]`，写入目标并消费引用 | `vm_assignValue`，`0x0808f634` |
| `04` | `[Ref, value] → [value]`，反向赋值，同样留下右值 | `vm_assignValueReverse`，`0x0808e914` |
| `39` | 参数/局部槽引用；与原有局部槽编号方向一致 | `vm_pushLocalVptr`，`0x0809134c` |
| `3e` | 当前对象变量引用，无继承重定位 | `vm_pushObjVptr`，`0x080911a8` |
| `13` | Array/Mapping 转临时槽引用；已有引用保持原样 | `vm_convertToPtr`，`0x0808a220` |
| `14` | 单层解引用，复制普通值，释放引用 | `vm_convertToValue`，`0x0808c028` |
| `44` | `[Ref(container_slot), key] → [Ref(member_slot)]` | `vm_queryMemberPtr`，`0x0808f8e4` |

引用与普通值在 `StackEntry` 中分开表示，公共 Value 的类型标签保持不变。第一批操作要求通过 `14` 显式读取引用；数值运算、类型检查等普通值指令不会隐式解引用。引用作为函数参数、返回值或容器内容均拒绝。

容器 header 保持共享身份，数组元素和映射值存入独立稳定值槽。多个变量持有同一容器时，成员写入对所有别名可见；不使用写时复制。映射插入后，旧成员引用仍指向原槽，不依赖索引内存地址。

数组 mode 0 使用普通索引，mode 1 使用 `size-index`；因此 index 1 表示最后一个元素，index 0 越界。Real 索引使用既有的截断转换；负索引不采用 Python 语义。`43/44` 仅接受 mode 0/1；普通映射在这两个 mode 下使用相同键查找行为。键只允许 Int/Bytes，按原始字节区分大小写。

`44` 查询缺失映射键时立即插入 Undefined；后续赋值失败不回滚此前成功的插入。若 `44` 本身因键类型、容量或引用资源上限失败，则不插入键。

## 生命周期与受限写入

Local 引用带单调 FrameId 和 Weak<Cell>；访问时检查帧及槽仍有效。Object 引用同时检查对象存在性。成员和临时引用独立保活值槽，不保活原容器 header；栈快照仅暴露诊断信息，不能向 VM 注入引用。

取得新成员槽后，才消费旧引用。释放最后一个数组 header 只释放其元素槽所有权，外部引用可保留旧元素值；释放最后一个映射 header 时先清空全部值槽，脱离映射的引用随后读到 Undefined。这实现了原版 `vm_unrefArray` 与 `vm_freeNode` 的关键差异。对象失效返回 InvalidObject，失效的局部槽或帧返回 InvalidState，不复现原版悬空内存访问。

本批允许变量/临时槽绑定任意已有普通 Value；**容器成员仅可写 Int、Real、Undefined、Void、Bytes、Object**。允许沿既有嵌套容器链写入内部标量，拒绝把 Array/Mapping 写入成员，因此不会新增容器图边或循环。容器成员替换为标量可以降低深度，动态深度计算按共享身份记忆结果，避免重复遍历共享子图。

写入前先验证目标和右值，分配表达式结果槽，再提交替换；右值先快照，旧值在借用结束后释放。所有运行期 Cell 借用守卫保持私有且不跨清理/释放。映射最终析构的内部清理依赖这一不变量；公共接口不会返回 RefCell 守卫。借用冲突作为 InvalidState 返回。

尚未支持：成员写入容器值、容器环及 GC、引用参数/返回、全局引用、const 扩展、复合赋值、自增减、字符串/Buffer/范围写入、继承或跨对象调用。`13` 不支持标量。旧的固定原型和 const 拒绝规则继续生效；内部保留只读引用检查点，但这不表示完整 const 语义已实现。

## 公共接口迁移

`Value::array`、`Value::mapping`、`Completion::Returned(Value)` 的签名保持不变。其余读取改为有错误返回的浅快照：

| 接口 | 当前返回类型 |
|---|---|
| `ArrayValue::elements()` | `Result<Vec<Value>, VmError>` |
| `MappingValue::entries()` | `Result<Vec<(Value, Value)>, VmError>` |
| `MappingValue::get(&Value)` | `Result<Option<Value>, VmError>` |
| 容器 `depth()` | `Result<usize, VmError>` |
| `Object::variables()` | `Result<Vec<Value>, VmError>` |
| `Vm::stack()` | `Result<Vec<StackSnapshot>, VmError>` |

`StackSnapshot::Value(Value)` 表示普通值；`StackSnapshot::Reference { kind, target_id, valid }` 表示只读引用诊断。ReferenceKind 包括 Local、Object、ArrayMember、MappingMember、Temporary。CellId/FrameId 不复用；诊断 ID 不是原版指针、序列化字段或协议值。对象被删除后快照保持相同目标 ID、valid 变为 false。

```rust
use rml32::{Value, VmError};
fn inspect() -> Result<(), VmError> {
    let value = Value::array(vec![Value::Int(7)])?;
    if let Value::Array(array) = value {
        let members = array.elements()?;
        assert_eq!(members, vec![Value::Int(7)]);
        assert_eq!(array.depth()?, 1);
    }
    Ok(())
}
```

快照复制当前成员值，嵌套容器继续共享身份；保存快照中的容器会增加其 header 所有权并推迟最后释放。它不是深拷贝或跨 VM 步进不变的快照。宿主不能直接写值槽；VM 为 Rc 所支持的单线程模型。容器 Eq 继续按 header 身份判定。

## 资源策略与验证

每容器最多 65,536 个输入项、嵌套深度 64；映射动态插入最多 65,536 个不同键。`VmLimits` 新增 `max_reference_cells`，默认 65,536，约束同时存活的临时/成员引用所指的不同槽数。同一槽的多个引用共用额度；Local/Object 引用仍受栈及帧限制。引用转换的过渡步骤同时保活新旧槽，可能需要两个额度；释放引用后额度可复用。登记表按需清理已失效 Weak 令牌，其大小保持有界。以上是离线实现策略，不是原版资源数值。

原有 73 项测试经接口迁移保持通过，新增 24 项测试：包含私有借用冲突和失效句柄单元测试，以及赋值、别名、提前插入、映射扩容、临时容器释放、嵌套写、动态深度、模式、生命周期和资源上限的合成程序测试。Debug/Release 各 **97 项**。运行命令：

```sh
cargo test --offline --locked
cargo test --offline --locked --release
cargo fmt --check
cargo clippy --offline --locked --all-targets -- -D warnings
cargo build --offline --locked
python3 tests/compare_corpus.py --report target/stage2e-corpus-validation.json
```

完整结果见 [stage2e-validation.json](stage2e-validation.json)。全量静态比较覆盖 12 个包、7,159 个程序和 9,933 条继承匹配；已知 `dba/pack_data/etc.pak/etc` 异常仍须保留。原始 36 个制品与旧证据保持不变。静态回归和合成测试不构成实际服务端执行等价证明。

下一阶段优先补充 `22/2f` 自增减及复合赋值的受限数值路径，复用本阶段稳定槽及错误定位；引用参数、容器值成员写入和 GC 仍需单独设计。
