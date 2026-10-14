# 阶段 2R：循环与 switch 静态语义及 Rust 实施设计

## 一、总体结论

六条指令的原生主体、共享辅助函数和主要清理路径已取得REA MCP / Ghidra静态证据。**下一实现批次先补0a/0b/0c三条switch指令；整数/数组循环随后实施，mapping循环继续受原生哈希顺序约束。** 本轮没有修改Rust运行时，保持59种限定操作码、237项Rust测试；主语料仍缺16种指令，阶段2尚未完成。

两项关键修正：

- switch不是一个整块跳转表操作。每条0a/0b读取固定8字节metadata，比较当前栈顶；匹配才弹值并跳转，失配保留值继续执行。0c无条件弹值并跳转。
- 23/12/50不只是foreach。23接受步长+1/-1的整数闭区间，或模式997（0x3e5）的数组/mapping遍历；不支持buffer。它创建类型0x14的隐藏循环控制值，存放于普通局部槽，不能直接用Rust集合快照或公共Value变体替代。

**证据性质：**“确认”表示反汇编/交叉引用/反编译中的静态事实；语料观察不证明可达或执行频率；Rust接口和拒绝策略属于工程设计。全程没有执行原ELF、脚本或业务字节码。

主ELF SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新分析显式使用Ghidra 12.1.4，保留12个新dossier及5条复用记录；会话已导出并关闭。[完整证据索引、原始伪代码和汇编](vm-control-flow-evidence/rea-functions.md)，[语料方法](vm-control-flow-evidence/corpus-method.md)，[源码集成审计](vm-control-flow-evidence/runtime-design-audit.md)，[验证记录](stage2r-validation.json)。原始函数体范围和limitations保留于证据包，不能把反编译器误标的union字段当作业务结构。

## 二、六条指令及直接证据

| 指令/函数 | ELF地址 | 核心契约 | Evidence ID |
|---|---|---|---|
| 0a vm_caseCheckNumber | 0x080974c0 | metadata=(delta,number)，仅Int精确相等命中 | `ev_77292c326eb595efcfe5597edb1f65f303464f58775e3435dac02ee78a721f7f` |
| 0b vm_caseCheckString | 0x080974fc | metadata=(delta,string_index)，先验证当前constitution字符串池，再比较String | `ev_64fb48253351112793bb3171542d795d713e0d2879a2cf900617f8c6f7873a9e` |
| 0c vm_caseDefault | 0x08097584 | pop后ip+=有符号par16 | `ev_2dce3eed228dccaaf1d1e67dc9a39c962724b0af9866210b68ecba6102befc83` |
| 23 vm_initForLoop | 0x080bc8f0 | 新控制值置于bp+varNo，读取3个栈值并pop3 | `ev_6493fcdaf5956b81b156c8fbb9cf5987c216a53b856561a1e6351de2ad42af92` |
| 12 vm_condJmpForLoop | 0x080bcdc8 | 调共享helper(varNo=par8,continue=0,break=par16) | `ev_2d0596b38377275abb8795f88716adc8a7d78bec1ef3c3ad3b34ba14eb6df030` |
| 50 vm_stepJmpForLoop | 0x080bcdb0 | 调共享helper(varNo=par8,continue=par16,break=0) | `ev_e744e3e38e0be2946efa9fff437a287f867a4093ee99c36e5c78ddfbdad478bd` |

### 解码及跳转基准

动作表标记0a/0b使用metadata、参数数2。`vm_regularProcessor@0x080978c4`汇编0x08097950–0x08097951先增加IP；0x08097a3c–0x08097a46对par8/par16分别符号扩展，再计算`(par8 << 16) | par16`，不是把par16先转u16。0x08097a8c–0x08097a96与0x08097a51–0x08097a57按cdecl传入metadata的两个32位值。旧汇编证据：`ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597`。

因此五条可跳转指令的taken目标统一为 **`原pc + 1 + delta`**；23不跳转。0a/0b的delta是metadata第一个小端i32，0c/12/50的delta是par16。仅凭语料中目标没有越界不能判断这个“+1”，结论来自解释器与处理函数汇编。

### switch比较与栈效果

0a在0x080974d7检查type==2，0x080974e3比较32位payload；Real(3.0)、Bytes("3")等不会转换成Int(3)，失配不报类型错误。命中在0x080974e8弹出选择值，随后增加IP。

0b先在0x0809751d–0x08097554检查字符串索引范围与非空池项，即使选择值不是String也先检查。池来自当前constitution，继承体不能使用根对象或调用者字符串池。`vm_isSameString@0x0808c8b8`的新证据`ev_aae7e66b585e97e76f5365bf919a10c74ddeca801427441bf6bece82930403e9`确认：同一指针立即相等；长度不同不等；两个不同shared字符串指针不等；否则按长度逐字节比较（非strcmp），区分大小写，无UTF-8转换。合法共享字符串通常由驻留机制保持唯一，但本轮没有完整恢复驻留器；不得据此宣称任意伪造shared头兼容。

0c无条件调用popStack再增加IP。无default的源语法可能通过普通pop/jump处理；处理函数不要求编译器必须生成0c。原生这些case处理器未自行检查栈底，Rust实现必须保留安全的计算区栈检查。

## 三、循环状态、类型和清理

### 初始化与36字节控制块

23先清理控制槽旧值，分配0x24字节、清零并置type=0x14（0x080bc9b3–0x080bc9f9）。其后从原生向低地址生长的栈读取：`top(0)=mode`，`top(1)=end或collection`，`top(2)=绑定变量的指针值`，最后pop3。控制槽参数23取par16；12/50取有符号par8。

原生槽为`bp+varNo`；Rust栈方向相反，对应既有`slot(no)=bp-1-no`，无需改变逻辑编号。负数对应局部变量。原生低层只做整栈范围检查，不能据此开放任意参数槽/别名；后续首批将控制值限制在已分配局部槽。

按读写偏移归一化的结构（字段名为分析命名）：

| 偏移 | 含义 |
|---|---|
| +0x00 | 初始化为1的管理字段，非业务循环索引 |
| +0x04 | 绑定变量Vm_Value指针；初始化和free函数没有增减其引用 |
| +0x08 | mode/step：+1、-1、901(array)、902(mapping) |
| +0x0c | 数值循环的finished标志 |
| +0x10 | source.attrib&1，后续复制时传播的属性位 |
| +0x14 | 数值current，或被持有的集合header |
| +0x18 | 数值end，或数组index/mapping bucket |
| +0x1c | mapping桶内位置 |
| +0x20 | mapping输出序号 |

模式必须为+1、-1或997。前两者要求end和绑定变量当前值均为Int，保存初值与end。997只接受Array(type6)或Mapping(type7)，分别转为内部901/902，并对集合header加引用；源变量随后重新绑定不会替换控制块持有的集合。其他step/source类型错误；不是任意非零整数步长。

### 条件与步进共用helper

`vm_cmpAndJmpInLoop@0x080bcae0`，新证据`ev_2f7c7986819c8b0acae9e1cddb895fded61ac3d31a2c036489d87c95592ebcce`。两个包装函数通过寄存器和栈传参，helper恢复为regparm2；不能只按外层cdecl签名解读调用。

- **整数**：每次helper先读取current，保存`current+step`（x86 32位环绕），将旧current写入绑定变量，再检查finished与有符号闭区间条件。有效时应用continue位移，否则应用break位移。旧current==end时设置finished，防止整数端点环绕后重新进入。故1..3依次向循环体提供1、2、3，下一次失败检查仍将变量写为4；起点已越界时也先写入起点。循环体手动修改变量不会改变控制块自己的current。
- **数组**：每次先取index并令index++，按所持header的当前长度检查。有效时复制当前成员值到绑定变量，并传播source属性位；不是提前复制所有元素，也不是把变量绑定为成员引用。别名对尚未访问成员的修改可被下一次读取看到。普通赋值替换绑定变量会在下一次有效检查被覆盖。
- **mapping**：每次重新从当前bucket的链头走到记录的桶内位置，按物理桶/链顺序前进；有效时向绑定变量写入长度3的数组`[key,value,ordinal]`，ordinal从0开始。若绑定变量已是长度3数组，会直接重用并改写其成员（0x080bcc89–0x080bcd1b），先前保留的该数组别名会观察到更新。它不是Rust mapping的插入顺序快照。修改桶链或重哈希可能影响后续扫描，完整插入/重哈希策略尚未核实。

条件12有效时continue=0，写出第一个值后落入循环体；耗尽时跳过循环体。步进50有效时跳回循环体；耗尽时break=0，落入退出路径。两者**耗尽时均未释放控制值**，也不会自动将控制槽置Undefined。

### 生命周期与非局部退出

既有`vm_cleanupValue@0x0808ad98`的type20分支调用`vm_freeLoopValue@0x080bc884`；复用证据`ev_6eecd8e699df5eefd7a3a4825b0b2c554b2844c7bf1228db97aa0803c3d8900e`与新证据`ev_7c730bb3ca6a220df55353b514f3cfe7e4d79e6347771e672ae459058017f177`确认：free按901/902释放所持集合，再释放控制块；不会清理绑定目标指针。

新`vm_copyValue@0x0808be24` dossier `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`保留完整汇编与交叉引用，type20复制结果为Undefined，不能传播成普通循环对象。新popNStack与takeOff证据确认被移除槽逐个调用cleanupValue；复用returnFromFun证据连接到正常局部值清理。因此正常返回/覆盖控制槽可释放控制值；break跳转本身不释放，寿命取决于后续覆盖或帧退出。

原生通用catch/错误后端的全部解栈路径尚未在本阶段闭合。当前Rust普通错误保留帧/栈，任务错误清理已有局部契约；后续循环不能假称两种错误模型完全一致。预算暂停在处理器边界保存控制块，不应重复init或重复推进游标。

## 四、C风格归一化伪代码

以下是依据证据整理的模型，不是原始源码或可直接编译代码；省略原生属性、错误后端和布局细节。完整反编译见证据附件。

```c
void case_number(int delta, int literal) {          // 0a
    if (top().type == INT && top().i32 == literal) {
        pop(); ip += delta;
    }
}
void case_string(int delta, int index) {           // 0b
    String *s = checked_pool(current_constitution, index);
    if (top().type == STRING && same_string(top().str, s)) {
        pop(); ip += delta;
    }
}
void case_default(int delta) { pop(); ip += delta; } // 0c

void init_loop(int slot) {                         // 23
    cleanup(control_slot(slot));
    Loop *c = new_zeroed_loop();
    control_slot(slot) = HiddenLoop(c);             // tag 0x14
    c->step = top(0).i32;
    c->bind = top(2).pointer;
    c->attrib = top(1).attrib & 1;
    if (c->step == 1 || c->step == -1) {
        require_int(top(1)); require_int(*c->bind);
        c->current = c->bind->i32; c->end = top(1).i32;
    } else if (c->step == 997 && is_array(top(1))) {
        c->step = 901; c->source = retain(top(1)); c->index = 0;
    } else if (c->step == 997 && is_mapping(top(1))) {
        c->step = 902; c->source = retain(top(1));
        c->bucket = c->subindex = c->ordinal = 0;
    } else error();
    pop_n(3);
}
void loop_cond(int break_delta, int slot) {         // 12
    loop_check(slot, 0, break_delta);
}
void loop_step(int continue_delta, int slot) {      // 50
    loop_check(slot, continue_delta, 0);
}

void loop_check(int slot, int continue_delta, int break_delta) {
    Loop *c = require_hidden_loop(control_slot(slot));
    if (c->step == 1 || c->step == -1) {
        int x = c->current;
        c->current = wrapping_add(x, c->step);
        replace_with_int(c->bind, x);
        bool valid = !c->finished && (c->step > 0 ? x <= c->end : x >= c->end);
        ip += valid ? continue_delta : break_delta;
        if (x == c->end) c->finished = true;
    } else if (c->step == 901) {
        unsigned i = c->index++;
        if (i < current_length(c->source)) {
            replace_by_copy(c->bind, current_element(c->source, i));
            apply_source_attrib(c); ip += continue_delta;
        } else ip += break_delta;
    } else if (c->step == 902) {
        unsigned ordinal = c->ordinal++;
        Node *n = next_current_bucket_chain_node(c); // current physical hash order
        if (n) {
            Array *triple = reuse_or_allocate_array3(c->bind);
            replace_by_copy(&triple[0], n->key);
            replace_by_copy(&triple[1], n->value);
            replace_with_int(&triple[2], ordinal);
            ip += continue_delta;
        } else ip += break_delta;
    }
    // Neither exhaustion nor a branch destroys control_slot(slot).
}
```

## 五、实际语料核对与证据边界

| 指令 | 静态节点数 |
|---|---:|
| 0a | 12,720 |
| 0b | 20,643 |
| 0c | 8,494 |
| 12 | 20,850 |
| 23 | 20,850 |
| 50 | 20,850 |

扫描全部7,159程序、101,592普通函数、0隐藏函数及所有初始化段，目标六条仅出现于普通函数。83,557个taken目标、20,643个非空字符串索引、62,550个循环槽范围检查全部合法；metadata越界0。23紧邻前一条均为3b短整数：-1有38处、+1有4,380处、997有16,432处。邻接观察不是完整栈流或类型证明，三个循环节点数量相等也不是循环配对证明。

[最小片段](vm-control-flow-evidence/examples.json)保留每条指令的来源、成员SHA、函数、PC及解压成员内偏移。例如`gs/.../feature/char/channel.o::has_refuse_channel_msg`在pc3的0a读取`(delta=1,number=3)`并命中到pc5；pc4的0c读取delta3跳到pc8。最小循环样例23在pc5控制槽-2，12在pc6的delta4跳到pc11，50在pc10的delta-4跳回pc7。不能把这些成员偏移当作ELF地址。

## 六、下一批 Rust 实现契约

### 先实现switch三条（下一批）

1. 不新增公开接口、Value变体或Frame状态。扩展准备器metadata解码到0a/0b，固定读取8字节两个i32并保留原offset；索引、非空池项及`pc+1+delta`在来源代码体内预检。保留符号扩展规则，允许合法共享/重叠metadata引用，不凭对齐或“必须成表”增加限制。
2. 运行时通过当前CodeView来源池取字符串，使用Int精确匹配及Bytes长度/原始字节匹配；其他已支持Value类型失配并保留。Reference选择值首批明确拒绝；0c同样只接受Value。Rust不模拟不合法的shared头，Bytes与合法池值之间按内容比较，保留驻留身份差异说明。
3. 所有case先验证计算区至少1项；命中时弹值并跳转，失配顺序执行。0b池校验优先于选择值类型判断。无需case表缓存、隐式switch栈或额外默认状态；07/08与暂停沿用既有帧契约。
4. 首批只开放普通函数体，初始化直接出现0a/0b/0c拒绝；初始化调用已准备普通辅助函数仍可使用switch，因为没有独立跨对象/循环任务状态。默认参数表达式继续保持现有线性白名单，不随之开放switch。
5. 支持集合仅在回归通过后增加3项，届时预计62种限定编号；不能把全部switch源语法或动态业务流程称为已兼容。

### 再实现整数与数组循环（后续批次）

采用**私有Cell载荷**，而非公开Value::Loop或单独的帧级旁表：建议内部`SlotPayload::Value(Value) | Loop(Box<LoopControl>)`，`LoopControl`保存绑定Reference及`Numeric { next,end,step,finished }`或`Array { owner,index }`。普通读取隐藏载荷投影为Undefined；替换/释放Cell自动释放控制块和所持header。这样既不污染公共标签，又能保证覆盖局部槽立即释放；仅在Frame中放游标而不接管槽替换会延迟释放。

复用现有负局部槽编号，只允许已分配的局部控制槽；绑定变量首批限定有效本地/参数/对象引用，拒绝与控制槽同Cell及临时/成员绑定，避免原生裸指针未持有owner的别名歧义。集合首批只开Array；Mapping模式返回UnsupportedFeature，其他模式仅±1。初始化/构造准备路径保守拒绝这三条循环指令，包括辅助体，直到任务清理组合另行验证。

使用小范围借用访问并更新控制值，不跨用户调用/暂停保存RefCell guard；数组保持header而非elements快照，逐次读取成员。控制块数量受已分配局部槽及现有max_stack/max_frames限制，不引入新的公共配额API。普通错误保持既有诊断保留策略；正常返回、槽覆盖、重启和任务取消按载荷销毁释放。所有普通Value写槽、replace_batch及引用写入必须一并审计，不能只改三条handler。

### mapping循环与剩余未知

原生桶链顺序已确认，**确定性重现该顺序仍缺哈希/建表/插入/重哈希证据**；Rust当前BTreeMap索引+插入序列不等价。暂不开mapping循环，后续补其算法或明确限定无顺序观察的受控范围。原生源属性位1、绑定目标与源成员别名、空集合的绑定变量保持行为、通用catch解栈及恶意重复操作控制值需组合验证；未完成项不能用快照、自动释放或空值兜底。

## 七、后续测试预期与本轮验收

| 场景 | 后续实现必须断言的预期 |
|---|---|
| 数值case | Int(3)命中3；Real(3.0)不命中；失配选择值仍在栈，命中仅移除一项 |
| 字符串case | 相同原始字节命中；大小写/长度/非UTF-8按字节区别；无效池项即使选择值非String也被拒绝 |
| default/无default | default弹一项后按pc+1+delta跳转；其他普通pop/jump序列照常工作，不要求存在default |
| metadata/跳转 | 负offset、截断8字节、无效索引、段外目标拒绝；合法重叠metadata接受，错误定位保留pc/成员offset |
| 嵌套及07/08 | 调用体使用自己的池/基址；内层case不消费外层保留的选择值；返回恢复调用者上下文 |
| 循环端点 | ±1闭区间；起点越界不执行body；相等只一次；失败检查也写绑定变量；i32端点不重复进入 |
| 数组/空集合 | 空数组不写绑定变量；单元素一次；未访问元素的别名写入下一步可见；源变量重绑不替换所持header |
| break/return/覆盖 | break不立即释放控制槽；覆盖和返回释放header；复制控制值得到Undefined；普通错误继续保留诊断状态 |
| 预算/嵌套循环 | 0预算不变；每条指令只推进一次；嵌套控制槽、递归和跨对象调用不共享游标 |
| 暂不支持组合 | mapping、buffer、非法mode、控制槽/绑定同Cell、任务内循环明确失败；不把拒绝当作语义完成 |

本轮237项Rust测试、fmt、Clippy、既有2Q矩阵重算、全量只读语料比较和13项新统计脚本合成测试通过。脚本默认只读重算，并逐字比较结果；只在显式`--write`时更新本阶段生成统计。原始36件制品及所有旧证据、Rust源码/测试和Cargo文件哈希不变。DBA已知LZSS异常继续单独保留，完整语料比较通过只表示结构基线一致。

```sh
python3 docs/vm-control-flow-evidence/analyze_corpus.py
python3 -m unittest discover -s docs/vm-control-flow-evidence -p test_analyze_corpus.py
python3 docs/vm-control-flow-evidence/verify_validation.py
```

2Q文件和验证记录保持历史快照；其旧文件级检查包含当时README/路线图哈希，新增文档入口后应使用本阶段2R文件级验证，不刷新旧记录。

本阶段将六项问题登记为：基本操作数/跳转/主要类型与栈效果已回答；mapping完整顺序、源属性和别名极端组合、通用非局部错误清理部分回答。下一批switch实现不依赖这些循环未知项。只有实现与合成测试通过后才能更新支持矩阵，本轮不改历史2P/2Q快照或编号状态。
