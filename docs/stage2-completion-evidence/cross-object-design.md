# 跨对象调用：静态证据与下一批 Rust 设计

## 一、结论及证据范围

下一实现批次优先补08 `vm_callInOtherFunEx`与41 `vm_pushThisObject`的受限对象路径。**本文件是设计，不是已实现功能。** 首批限定直接、存活的Object目标、显式完整方法绑定、普通用户函数、flags∈{0,2,4,6}；字符串名称寻址、数组分派、efun返回模式、函数值、全局及生命周期任务内跨对象调用继续明确拒绝。

原始ELF SHA-256为`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新查询由REA MCP调用Ghidra 12.1.4，仅读取制品；[完整索引、伪代码与汇编](rea-functions.md)及[新证据包](rea.json.gz)保留结果。既有证据按原记录复用到[reused.json.gz](reused.json.gz)，附原文件路径和SHA-256。没有目标运行轨迹；以下“确认”指静态代码事实，非已观察运行行为。

## 二、已确认的调用契约

| 关键函数 | 地址 | 静态结论与证据定位 |
|---|---|---|
| vm_regularProcessor | 0x080978c4 | 0x08097986–0x08097992分别MOVSX par8、par16并按cdecl压栈：08的flags=有符号par16，argc=有符号par8。旧证据`ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597`。反编译丢失第二参数，须以汇编为准。 |
| vm_callInOtherFun | 0x08094cd8 | 顶端target，其后method，再后arg0…；0x08094d2d–0x08094d3e以flags&2传mayPrivate；命中后pop2并进入用户函数。旧dossier `ev_4b89eec3bfa71f6f357f508600108ff4f54e9492ffa2c51e8012d5d1a8ce8db0`。 |
| vm_findProgramFunction | 0x08086b44 | 使用标识符哈希或quickFunction表；mayPrivate=0且函数attrib&4时视为查找失败。旧伪代码`ev_ba5b1993ec6d3677bd709e0fb78467636839e28ccd2802f1a214278e6a287928`，汇编记录同包。不能用函数槽从头扫描替代名称表。 |
| vm_callInOtherFunEx | 0x080bd0fc | 标量命中后安装ExReturn回调；数组走独立游标/结果数组续行。旧dossier `ev_13e8fe9368cca520ed154767ba3471e1ac37473f0886ab1b15365d0e8db01662`。 |
| vm_pushThisObject | 0x080975d8 | 0x080975e3读取当前上下文+0x20；非空pushObject，空指针pushNumber(0)，不是Undefined。新`ev_0f52440a23f0b659faa95e8a973f08f1b228163126ef3957b926b6685f6d9b78`。动作表数据xref为0x083ccda4。 |
| vm_callInUserFunBody | 0x08093eb0 | thisObject设为目标，以目标根程序的函数入口选择constitution，使用实际函数codeSegment并增加execRef。旧伪代码`ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`。 |
| vm_returnFromFun | 0x08094978 | 清理当前层catch/参数/局部值，恢复调用者上下文，再unExecRef旧对象，最后运行returnCallback。新dossier `ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215`，保留其非连续完整函数体。 |
| vm_callInOtherFunExReturn | 0x080bcdd8 | cookie&1为0直接return；为1跳至vm_returnFromEfun。新`ev_cf79f214bf5abcf504ec8eb0240a0f7d4b5abbc172ccbf5250c9ee9eecc5cfc9`。 |

### 缺失方法、private与回退

`vm_callInOtherFun`先验证目标；Object必须未带删除标志0x8000，String必须能经`vm_lookupObjectByName`解析（字符串分支还对flags==0x800作精确判断）。目标类型/失效错误**不受flags&4抑制**。方法名必须为String。

方法查找失败，包括不允许访问private时，继续按相同mayPrivate查找`default_interface`。回退命中只弹target，保留原method作为新的arg0，并令argc+1。若两次查找均失败，弹target与method；flags&4时再弹原参数并pushVoid，否则报Function not found。private不应一律转换成独立“拒绝访问”错误，也不应直接跳过回退。

### 栈及返回的归一化伪代码

以下为人工归一化，省略原生布局、并发栈快照和资源检查；**不可直接作为可编译源码**。原生栈向低地址增长，`top(k)`表示从顶向深处第k项。Rust Vec表现为底端`… argN-1 … arg0 method target`到顶端。

```c
bool call_other_scalar(int flags, int argc) {
    Object *target = validate_target(top(0));
    String method = require_string(top(1));
    Function f = lookup(target->program, method, flags & 2);
    if (f.found) {
        pop_n(2);                       // target, method
    } else {
        f = lookup(target->program, "default_interface", flags & 2);
        if (f.found) {
            pop_n(1);                   // target; method becomes arg0
            argc++;
        } else {
            pop_n(2);
            if (flags & 4) { pop_n(argc); push_void(); return false; }
            vm_error("Function not found");
        }
    }
    enter_user_frame(target, f.slot, f.body, argc);
    return true;
}

void call_other_ex_return(unsigned cookie) {
    if (cookie & 1) return_from_efun();
}

Value this_object(Context ctx) {
    return ctx.this_object ? ObjectValue(ctx.this_object) : Int(0);
}
```

flags低位1从回调行为可确认是efun返回相关模式；把它直接称为字节码“尾调用”缺少依据。首批拒绝该位，因此标量回调等效无操作，但必须为以后接入保留显式返回策略。普通返回后的thisObject、根program、实际body、变量/函数基址、字符串池和PC都须恢复，不能只恢复ObjectId。

### 数组路径与编译器交叉检查

新`vm_callInOtherArray2@0x080bcf14`（`ev_dae2362e9944413d4a2410309483bc76294a735723624ba3a5fc2f583c2f99b4`）核对sp及中间值类型，跳过零元素，复制参数/方法/单个目标，以flags=4调用标量路径并安装Array3回调。`vm_callInOtherArray3@0x080bcdec`（`ev_70d5db7423deaeb10a6ac1f00a2ab189ef9d8620f60f9eeac00dff36a31dcf66`）转移返回值、推进索引，再安装nextActionHook继续遍历。不能用立即Rust for循环替代其暂停/清理契约。

这些函数的反编译将数组union字段误显示为object.execRef/string.ptr等；数组布局含义须与汇编和已有值模型交叉核对，本轮不据这些字段名实施。完整数组分派留待后续。

动作表08编译器入口0x080cab20实际名为`vm_compileToFunCall`，并非此前可能猜测的CallOther专用编译器。新`ev_7bfea1e46d51d77201c90e9d4e594b0f6791c4bff0c33cc634a3bd46cde37c78`确认它写入ip+1、调用通用编译器并检查控制标志/nextActionHook。本轮不实现JIT，也不从编译器名称推导字节码栈契约。

## 三、Rust改动设计（待实施）

1. **准备与方法目录**：为PreparedProgram增加可选显式方法表，建议`MethodBinding { name: Vec<u8>, slot: i16 }`；绑定名称必须非空、无嵌入NUL，大小写敏感，private从实际函数元数据读取。未知/未提供方法目录必须报UnsupportedFeature，不能当作“方法不存在”而返回Void。只对已声明完整的合成方法目录允许真实miss；名称重复、未知槽、未实现体、别名歧义和生命周期入口在准备时拒绝。显式绑定是合成范围的工程接口，**不是已恢复原生命名表生成算法**。
2. **静态指令验证**：08只接受flags∈{0,2,4,6}、非负argc；par8按原生有符号语义解释。未知位、efun模式、字符串/数组目标继续失败。41在当前对象帧压入当前Object；无对象全局帧将来压Int(0)，当前准备器仍拒绝全局输入。
3. **运行时分派**：从目标ObjectStore记录取得ProgramId与PreparedProgram，不能沿用调用者program。进入前检查栈、目标寿命、完整方法表、查找/回退、参数原型、帧和配额；复用现有参数/引用/default入口检查。可预检失败尽量在消费操作数或写槽前报告，这属于受限Rust错误边界，不能宣称与原生错误现场栈完全等价。
4. **帧上下文**：Frame保存目标根program/object、实际函数槽/body及返回策略；复用链接后的函数/变量基址和body字符串池。A→B→A同程序不同实例及不同程序都应正确；B内07仍调用B，41返回B；返回A恢复A。局部引用仍绑定原帧，Object引用仍绑定原owner，传参不能重标为目标对象。
5. **返回与错误**：首批正常返回按现有类型检查保留一个值并清理参数/临时槽；保持现有普通终止错误的诊断契约，不冒充通用catch。预算暂停保留完整调用状态。生命周期任务内遇08在准备时拒绝，直到跨对象错误与构造回滚组合清理有独立设计；主动删除/execRef延迟释放未实现，不开放该组合。
6. **接口与依赖**：涉及`src/prepared.rs`、`src/vm.rs`、`src/object.rs`及`tests/vm_core.rs`，按需导出方法绑定类型。继续用标准库BTreeMap/Rc与现有Cell、错误和配额模型，不新增Cargo依赖，不手写标准库。

## 四、下一批验收及阻断项

合成回归至少覆盖：A/B相同与不同program、继承目标体及来源字符串池、A→B→A、41返回值、参数顺序和引用跨对象转发、private允许/不允许及回退、fallback增加arg0、缺失抑制得到Void而非Undefined、坏目标不被抑制、方法目录缺失、重复绑定、坏标志/负argc、空栈、类型错误、帧/槽配额、预算分段续跑和返回清理。检查失败前后对象值与引用有效性，防止污染调用者。

运行全部既有212项回归及新增合成测试、fmt、clippy、全量只读语料、矩阵重算与原件/历史证据哈希检查后，才可将受限08/41计入支持集合；此后预计59种，仍不是阶段2完成。不得执行原始业务字节码来“验证”。

**阻断完整兼容的未知项**：原生最终方法名表的生成/覆盖顺序、对象名称空间及自动加载、数组hook的全部异常行为、efun模式调用上下文、活动目标删除与catch交互。首批通过显式目录与拒绝策略隔离这些未知；后续需补证据及实现，不能用猜测的首个同名函数、空返回或立即删除替代。
