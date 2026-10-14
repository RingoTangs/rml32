# 阶段 2G：引用参数与函数边界

## 总体结论

引用参数可以在现有 Rust VM 的 Cell、Reference 和调用帧模型上实现，无需重写标准库或增加依赖。本阶段完成静态证据、语料统计和实现设计，**尚未实现引用参数**。阶段 2F 的 52 个受支持 opcode、110 项测试仅作为既有基线，本轮没有重跑或扩展运行时测试。

只新增文档和证据；没有执行原始程序、启动脚本或业务字节码，没有修改参考文件。分析引擎为 REA MCP 接入的 Ghidra 12.1.4，临时分析会话已经关闭。原程序依赖 glibc.i686、libstdc++.i686；这些是原生参考环境的要求，不意味着 Rust 重写必须保留 C++ 运行时。

证据入口：[Ghidra 索引](vm-reference-arguments-evidence/index.md)、[汇编与反编译](vm-reference-arguments-evidence/functions.md)、[统计摘要](vm-reference-arguments-evidence/summary.json)、[统计方法](vm-reference-arguments-evidence/method.md)。以下“确认”指静态指令、数据或交叉引用直接支持；“推导”指据此推演；“设计”是未来 Rust 策略。

## 一、原生调用与类型检查：确认的事实

### 核心函数与地址

| 函数 | 地址 | 作用与直接证据 |
|---|---|---|
| vm_callInUserFun | 0x08094080 | opcode 07；解码目标及参数数量，调用函数体入口 |
| vm_callInUserFunBody | 0x08093eb0 | 0x08093f2d 增加调用深度；0x08093f43–52 设置 bp、argNum、pFun |
| vm_checkArgNum | 0x0809521c | opcode 0f；引用标志、参数类型和转换 |
| vm_getValueType | 0x0808cb8c | 对 Reference 解引用一次；可清理已销毁对象 |
| vm_getVarType | 0x0808cb3c | 不对 Reference 解引用 |
| vm_pushLocalVar | 0x080914dc | opcode 37；读取值，引用参数解引用 |
| vm_pushLocalVptr | 0x0809134c | opcode 39；已有引用直接复制并转发 |
| vm_setLocalVar | 0x0808f6dc | opcode 4d；引用参数写穿透到调用者变量 |
| vm_returnFromFun | 0x08094978 | 返回值检查及参数、局部槽清理 |
| vm_takeOffValueFromStack | 0x08092008 | 清理栈项并移动保留结果 |
| vm_traceError | 0x08093520 | 错误路径中的栈与调用帧清理 |
| vm_doActions | 0x08097008 | 执行循环及 setjmp/catch 恢复 |

每项操作的完整 Evidence ID 见索引。例如参数检查汇编为 `ev_d17aa0d7a395baf8b760f4dd773008e55d6a7fc1e8c6931a9657f73663982946`；其反编译为 `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9`。

### 参数描述与调用目标

参数记录长 20 字节：偏移 4 为 flag，5 为 type，6 为 attrib，8–9 为有符号索引。`flag & 1` 表示引用参数；type 255 为 mixed；attrib 位 8 表示可空，位 2 为 const。

opcode 07：`is_global=(b>>7)&1`，`argc=b&0x3f`；局部目标使用 `signed16(a + constitution.baseFunOffset)` 查函数表。位 6 的语义尚未确认，不能擅自用于参数计数。调用包装器不会预先完成所有类型检查；检查由函数内 opcode 0f 执行。

### 类型规则与副作用

- 0x08095268 检查引用标志，0x0809527b 要求实际参数 tag 为 8（Reference）。
- 0x08095296 对引用 mixed 跳过目标类型检查；其他引用通过 0x080952a0 调用 vm_getValueType，要求精确类型相同，或 Undefined 加可空属性。引用参数不进行 Int/Real 自动转换。
- 普通 typed 参数拒绝 Reference；普通 mixed 在 0x08095350–55 跳过检查，原生实现允许其携带 Reference。不能概括为所有普通参数都拒绝引用。
- 普通数值参数在参数栈槽原地转换：0x08095429–33 Int→Real，0x0809547f–87 Real→Int。
- vm_getValueType 在 0x0808cbb4 解引用，在 0x0808cbce 检查对象销毁状态，并在 0x0808cbd8 清理引用目标。因此类型检查可能修改调用者变量，即使后续参数失败。
- vm_convertValueToConst（0x0808cc18）仅对 tag 6/7 设置运行时属性位 1；不是递归冻结。引用参数分支不调用它。

以下是据反编译和汇编整理的 C 风格语义摘要，省略默认参数、可变参数及诊断细节，并非可直接编译的完整替代：

```c
void check_argument(Arg desc, Value *actual) {
    if (desc.flag & 1) {
        require(actual->tag == REFERENCE);
        if (desc.type == MIXED) return;
        int type = get_value_type(actual); // 单层解引用，可清理目标
        require(type == desc.type ||
                (type == UNDEFINED && (desc.attrib & NULLABLE)));
        return;
    }
    if (desc.attrib & CONST) convert_value_to_const(actual);
    if (desc.type == MIXED) return;
    require(actual->tag != REFERENCE);
    if (is_numeric_conversion(actual->tag, desc.type))
        convert_argument_slot(actual, desc.type);
    else
        require(actual->tag == desc.type ||
                (get_var_type(actual) == UNDEFINED &&
                 (desc.attrib & NULLABLE)));
}

void push_local_pointer(Value *slot) {
    push(slot->tag == REFERENCE ? copy_reference(slot)
                                : reference_to(slot));
}
void set_local(Value *slot, Value *rhs) {
    if (slot->tag == REFERENCE) {
        require(rhs->tag != REFERENCE);
        replace_value(slot->target, rhs);
    } else {
        replace_value(slot, rhs);
    }
}
```

## 二、别名、返回与错误边界

vm_pushLocalVar 在 0x0809157c 复制槽值，再于 0x0809159d 将 Reference 转为值。vm_pushLocalVptr 对已有 Reference 在 0x08091460 调用复制；它不额外创建指向被调函数参数槽的引用。vm_setLocalVar 的引用分支在 0x0808f765/0x0808f76e 清理并复制目标值。

**推导：**同一个 Cell 传给两个参数、或者经过嵌套转发，写入应相互可见。参数声明类型在进入时检查，通用赋值路径没有重新检查原型，因此不能把声明类型当作永久写入限制。普通容器参数的值复制也不等于容器深复制。

| 边界 | 原生确认结果 | Rust 首阶段设计 |
|---|---|---|
| 正常返回 | 0x08094c4a–50 清理参数及局部槽，保留结果 | 只允许普通 Value 返回，先检查再清理 |
| 调用者变量引用 | 清理引用参数本身不会把目标改写为 Undefined | 保留原 Cell 身份和拥有者 |
| 参数检查失败 | 之前的转换、const 或死对象清理可能已生效 | 整批检查成功才提交入口转换 |
| 函数体失败 | 不撤销已发生的调用者写入 | 已提交写入保留，终止状态用于诊断 |
| 局部引用逃逸 | mixed 返回分支可能保留裸 Reference；未发现自动提升 | 明确拒绝所有引用返回及持久化 |

vm_error 调用 vm_errorQuit(-3)。脚本线程路径在 0x080ab16c 调用 vm_traceError，并通过 longjmp 恢复；0x080ab144 是 longjmp 调用点之一。vm_traceError 不只是打印：0x08093ad0 调用 vm_popNStack 清理内部帧参数和栈项，0x08093c22 释放执行对象引用，并恢复上下文。vm_doActions 的 setjmp/catch 路径可跳转到处理地址。vm_caughtError 本身只压入整数 1；vm_returnToBackend 是另外的后端跳转路径。

**未确认：**所有线程类型、跨对象调用、efun、回调和复杂 catch 嵌套的统一生命周期；逃逸栈引用及对象成员引用的原生安全保证。Rust 首阶段不模拟 catch/unwind，不能声称达到完整原生异常兼容。

## 三、全量静态语料结果与边界

输入覆盖 7,159 个程序、101,592 个函数、6,474,773 条指令节点及 175,100 个形参。发现 **4,395 个引用参数函数、4,886 个引用形参，分布于 1,710 个程序**；19 种 flag/type/attrib 组合中，引用形参全部 flag=1、attrib=0。可空属性仅见于 4 个非引用 Object 参数，const 仅见于 1 个非引用 Array 参数；样本未出现不代表 VM 不支持。

引用函数中，3,140 个参数数量固定，1,255 个非固定；4,340 个以 0f 开头，55 个为空函数。满足“固定参数、函数低两属性位为 0、首指令 0f”的有 **3,050 个**，仅是首阶段形状候选，不是可执行覆盖数。

opcode 07 共 126,466 条：全局调用 25,574 条；constitution 偏移未知的局部调用 89,945 条；单 constitution 且无继承的可精确解析调用 10,947 条，指向 5,132 个目标，其中 385 个含引用形参，对应 742 条调用。原始 argc 超 max 的 603 条、低于 min 的 5 条，不足以判定无效，仍需结合默认参数和可变参数语义。

其他调用：动态函数指针 05 为 3,089 条，跨对象 08 为 366,483 条，efun 06 为 143,520 条，OS 09 为 0 条。不能从相邻指令或目标原型直接证明实际参数引用来源。

完整逐函数原型与可解析目标保存在 [prototypes.json.gz](vm-reference-arguments-evidence/prototypes.json.gz)。可重复只读核验：

```sh
python3 docs/vm-reference-arguments-evidence/verify_prototypes.py
```

## 四、下一阶段 Rust 实现方案

### 支持范围与内部表示

首阶段仅实现当前对象、单 constitution、无继承、固定参数、首指令 0f 的调用；默认参数、可变参数、atomic 和 const 暂缓。允许描述符 flag 0/1、类型 0/1/2/3/4/6/7/10/255、可空位 8；拒绝未知位及 const。无需增加 opcode，接通既有 07/0f/37/39/4d；支持计数仍为 52。

公共 Value 不加入 Reference，`Vm.start(Vec<Value>)` 不增加引用注入接口。直接以引用参数函数作为根入口时返回 UnsupportedFeature，通过普通包装函数形成内部引用调用。

参数槽保存 `StackEntry::Reference(original.clone())`，保留原 Target、CellID、拥有者 FrameId/ObjectId、ReferenceKind、readonly 和配额 lease。不能改包装为被调帧的 Local 引用，否则可能丢失成员引用标识并绕过标量写限制。局部引用仅接受源帧声明的参数、局部变量 Cell；临时求值槽来源暂拒绝。

37 解引用读取普通值；39 克隆已有引用进行转发；4d 使用统一引用写入逻辑，并保持原有 RHS 栈行为。03/04/4d/22/2f 共用拥有者存活、readonly 和成员标量限制检查。不得允许引用进入返回值、容器或全局存储。入口类型不是永久写约束。

### 两阶段检查及明确的兼容差异

1. **07 预检：**检查目标形状、数量、描述符、绑定种类、拥有者及配额；完成所需分配后提交调用帧。失败不改变调用者栈、Cell 或帧集合。
2. **0f 实检：**仍在真实指令执行时检查全部参数，暂存普通数值转换和 typed 引用的死对象清理；同 Cell 的待清理项合并，全部成功后一次提交。不跨提交持有可变 Cell 借用，也不调用宿主回调。
3. **预算间隔：**07 与 0f 之间可能暂停并由宿主删除对象，0f 必须重新读取状态，不能复用陈旧的类型检查结果。
4. **失败：**0f 失败保留该指令入口的栈和目标值，保留终止调用帧用于诊断；函数体先前已经成功的写入不撤销，引用由新 start 或 VM 销毁释放。

这是有意采用的原子入口检查策略，与原生按参数逐个产生副作用不同。typed 引用指向死对象时按 Undefined 匹配：不可空则失败且不清理；可空且整批成功才提交清理；mixed 引用跳过入口清理。引用 Int/Real 不转换，普通数值参数保留转换。

原生普通 mixed 可携带 Reference；首阶段 Rust 明确拒绝 Reference 传入任何非引用参数。原生可能允许 mixed 引用返回，Rust 仍拒绝。上述限制必须在能力报告和错误中可见。

原生 vm_copyValue 在读到死对象时还会清理源值；现有 Rust 普通复制只得到 Undefined。未来在 37/14 的引用读取路径中，对成功读取分阶段提交“结果 + 源 Cell 归一化”，不顺带改变全部普通值复制行为。

### 生命周期与实现顺序

先增加描述符能力判定及内部参数绑定，再实现 0f 的批量检查、37/39/4d 的引用参数路径，最后统一写权限和清理逻辑。正常返回先验证结果为普通 Value，再清理被调帧引用槽；祖先帧仍拥有原 Cell。祖先帧失效报 InvalidState，对象销毁报 InvalidObject；成员和临时引用沿用现有持有及释放规则，禁止形成可逃逸的循环引用结构。

所有步骤沿用 Rust 标准库及现有依赖，不新增 Cargo 依赖。每一步先用合成程序验证，再扩展静态能力统计；不执行业务语料来冒充原生行为对照。

## 五、下一阶段验收清单

- 普通标量隔离、容器共享身份；引用精确类型与普通数值转换的差别。
- 同 Cell 双参数、嵌套转发、递归及引用配额；目标修改即时可见。
- nullable、死对象、后续参数失败时的批量原子性；mixed 引用不触发入口清理。
- 成员引用转发后仍禁止非标量写入，readonly 与对象销毁不可绕过。
- 07 失败、0f 失败的 PC/栈/帧边界；预算暂停后对象删除的重新检查。
- 正常返回和错误状态下引用释放；临时及 mapping 成员引用释放；失效祖先处理。
- 拒绝引用返回、持久化及宿主注入；引用声明不限制函数体后续合法赋值类型。
- 运行 cargo test、cargo fmt --check、cargo clippy --all-targets；记录实际结果后才更新能力计数与路线图。

本阶段核验结果见 [stage2g-validation.json](stage2g-validation.json)。完整重写仍需继承、跨对象调用、默认/可变参数、异常和部署环境的独立阶段，不能由本次引用参数统计推定完成。
