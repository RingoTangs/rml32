# 阶段 2C：不可变容器语义与验证

## 总体结果

Rust VM 库增加数组（tag 6）、普通单值映射（tag 7），支持操作码从 39 扩展到 43。使用标准库 `Rc`、`Vec` 和 `BTreeMap`，无需新依赖。成员、变量和函数参数可共享不可变容器；没有修改入口或循环引用。CLI 仍只读检查制品，执行测试只使用内存构造程序。

## 已确认的原始语义

本阶段使用 REA MCP 和 Ghidra 12.1.4 静态分析原始 ELF。13 条原始证据保存在 [证据包](vm-container-evidence/rea.json.gz)，完整反编译和关键汇编见 [函数证据](vm-container-evidence/functions.md)。

| 操作码 | 函数地址 | 确认行为 |
|---|---|---|
| `0x16` | `vm_createArray` / `0x080918c8` | 取 n 个栈值；元素 0 是栈顶，依次向旧输入读取；空数组合法 |
| `0x18` | `vm_createMapping` / `0x08091af0` | n 是键和值的总数；栈顶是键，下一个是值；每次前进两个槽 |
| `0x43` | `vm_queryMember` / `0x0808fa60` | 容器在下、键在上；数组索引允许整数或截断的实数；映射缺失返回 Undefined |
| `0x4f` | `vm_sizeof` / `0x0808f840` | 数组元素数、映射键数、字符串字节长度；其余当前支持的类型返回 0 |

映射汇编 `0x08091b59` 右移一位计算容量；`0x08091bbf` 调用 `vm_findForInsert(..., 1)`，`0x08091be8` 从键的下一个 24 字节槽取值，`0x08091bf7` 每轮计数加 2。`vm_safeFindForInsert` 的匹配分支先清理旧值再返回同一槽，所以后处理的重复键覆盖前者：**字节码中最早压入的一对最终获胜**。

`vm_safeAllocateMapping` 初始化 `attrib = 1`，查找与插入按长度和原始字节比较字符串，区分大小写。整数键按整数值比较。查询使用 `vm_copyValue`：选中已销毁对象时返回 Undefined，容器内的旧句柄保持不变。

## Rust 实现与明确边界

`Value::array(Vec<Value>)`、`Value::mapping(Vec<(Value, Value)>)` 返回受检查的值；`elements()`、`entries()`、`get()` 只提供读取。公共映射构造器按传入顺序处理，最后一项覆盖；VM 构造器先按栈顶顺序逆转输入。公开 entries 的顺序是 Rust 实现细节，不承诺原生哈希桶遍历顺序。

仅支持 Int/Bytes 映射键、普通 mode 0 成员查询；其他键或查询模式明确报 Unsupported。数组负索引或越界报 Bounds，不采用倒数索引。容器可以嵌套、传参、返回、检查类型和真假；空容器也为真。容器相等的 Rust API 表示共享身份，VM 容器比较、算术和转换尚不支持。

每容器最多 65,536 个输入项，嵌套深度最多 64；这些是离线实现的资源策略，不是原生限制。负数量、奇数映射输入被明确拒绝，原生构造路径未观察到对应的完整检查。尚未支持成员指针、赋值、范围访问、可变共享、Buffer、特殊映射属性与完整服务端运行。

## 验证及可行性

执行 `cargo test --offline --locked`、Release 测试、`cargo fmt --check`、严格 clippy，以及 `python3 tests/compare_corpus.py --report target/stage2c-corpus-validation.json`。结果记录于 [验证记录](stage2c-validation.json)。测试覆盖构造顺序、重复键、非 UTF-8、缺失/越界、数值索引、嵌套、资源限制、类型限制、调用、预算恢复、对象隔离与失效对象读取。

不可变容器的 Rust 重写已落地；可变引用语义仍需独立证据与实现，不能据此认定服务端已兼容。全量制品回归证明静态结构保持一致，不证明实际业务执行等价。下一阶段优先分析成员引用及赋值的别名、生命周期与错误路径，再选择受限实现范围。
