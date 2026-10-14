# 阶段 2I：默认参数与可变参数静态分析及 Rust 设计

## 总体结论

默认参数和可变参数可以沿用当前 Rust VM 的调用帧、稳定 Cell 和引用绑定实现，但不能只解除“参数数量必须相等”的限制。原生入口先检查已经提供的参数，再执行尚缺的默认表达式，必要时通过 `47` 重排参数栈；没有默认表达式的可选 mixed 参数则可以保持没有槽位。可变参数保留多余实参，不自动形成尾部数组。

本阶段仅新增分析文档、证据和只读统计脚本，**没有实现上述扩展**。现有阶段 2H 的 52 种限定 opcode、132 项测试保持不变，本轮没有重复运行 Rust 测试。没有执行参考 ELF、启动脚本或业务字节码；没有修改原始文件、公共 API 或 Cargo 依赖。

分析使用 REA MCP 接入 Ghidra 12.1.4，临时会话已经关闭。参考 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。本轮有 31 条新记录，并单独收录 23 条复用记录。事实依据见 [证据索引](vm-optional-arguments-evidence/index.md)、[可读反编译与汇编](vm-optional-arguments-evidence/functions.md)；统计见 [摘要](vm-optional-arguments-evidence/summary.json)与[方法](vm-optional-arguments-evidence/method.md)。下文分别标识原生确认、语料观察、推导和未来设计。

## 一、原生格式与调用规则：已确认

### 1. 参数描述符与字段来源

| 数据 | 偏移/含义 | 证据 |
|---|---|---|
| Vm_Fun 原始记录 | 64 字节；+8 属性、+14 max、+15 min | 既有加载器及入口指令 |
| Vm_Fun_Arg 原始记录 | 20 字节 | DWARF 类型与 vm_loadFun |
| no | +8，有符号 16 位 | `0x080953bc` 的 MOVSX |
| defaultActionNum | +10，有符号 16 位，默认表达式的指令节点数 | `0x080953c4` 的 MOVSX |
| pDefaultValue | +12，编译时表达式树指针 | 加载后被清零，不是序列化默认常量 |
| pNext | +16，加载时重建参数链表 | vm_loadFun |

类型记录 Evidence ID：`ev_3a2145a4ec5bf063ceb5de533653f40d5ce70cd826ccb17d03b9892fc915df74`。`vm_loadFun @0x080959b8` 读取整个 0x14 字节参数记录，重置名称和指针但保留 +10，Evidence ID：`ev_b513e6242f5447517c22636c5c877a93b397b974851b7928f195106d957a1079`。

**注意符号性：**`vm_checkArgNum` 在 `0x080952fd`、`0x0809530c` 将 min/max 字节符号扩展，不能无限制按 u8 范围推断原生语义。当前语料 min 最大 19、max 最大 21，未出现最高位置位。

### 2. 编译器如何生成入口

`vm_buildDefaultValueNodes @0x080a20d0`：

- `0x080a2112–15` 将表达式节点数写入参数 +10，并清除编译树指针。
- 按形参顺序连接默认表达式；有默认表达式时追加 `47`，然后生成 `0f`，最后追加 `1b`。
- `0x080a21a4–b3`：没有默认表达式且类型不是 mixed 的形参才提高 min；max 是形参总数。因此 `min < max` 不等于“必有显式默认值”。

入口形状是：

```text
无默认表达式：0f a=0 → 1b → 函数体
有默认表达式：0f a=S+1 → 默认表达式序列（S 个节点）→ 47 → 1b → 函数体
```

生成器伪代码 Evidence ID：`ev_2c27ff7488e9819ae0c431411d6590807eaddf4a9037881bcf3d3e67b56ca294`；汇编：`ev_f7590616a34561205b430f9bfacdab404ef9c0c10d7447b91c9a9726ac29b0bc`。调用交叉引用定位到编译器 `vm_yyparse` 的 `0x08104a4f`；这里只确认直接调用关系，不声称恢复了全部语法规则。

### 3. 入口检查与控制流

`vm_callInUserFun @0x08094080` 从调用操作数解出 `argc=b&0x3f`；`vm_callInUserFunBody @0x08093eb0` 建立帧并保存实际 argc。检查在函数内 `0f` 执行，不在包装器内预先补齐参数。

`vm_checkArgNum @0x0809521c` 依次检查已传入且有描述符的参数：值参数允许既有数值转换，引用参数使用阶段 2G 确认的精确类型规则。缺少必需参数时报错；参数链表走完后仍有多余实参，只有 `attrib & 1` 时允许。关键指令：`0x08095325` 检查可变标志，`0x08095329` 跳向超量错误分支。

令 `pc` 为 0f 的节点索引，处理器已预增 IP：

- `argc < max`：目标为 `pc+1+Σ(defaultActionNum，条件 no<argc)`，只跳过已经提供参数对应的默认表达式。
- `argc >= max`：目标为 `pc+1+a`，跳过整个初始化段及 47。
- `0x080953c4/cb/d9` 分别读取长度、累计、更新 IP；`0x08095338` 是整段跳过路径。

以下 C 风格伪代码是语义整理，省略类型诊断和内部异常实现：

```c
void check_args(int total_actions) {
    Fun *f = current_frame.fun;
    int argc = current_frame.argc;
    int checked = 0;
    for (Arg *a = f->args; a && checked < argc; a = a->next) {
        check_supplied_argument(a, stack_argument(checked));
        ++checked;
    }
    require(checked >= f->min_args);
    if (argc < f->max_args) {
        int skip = 0;
        for (Arg *a = f->args; a && a->no < argc; a = a->next)
            skip += a->default_action_num;
        current_frame.ip += skip;
    } else {
        require(checked >= argc || (f->attrib & 1));
        current_frame.ip += total_actions;
    }
}
```

原生按参数逐项产生副作用；未来 Rust 继续采用阶段 2H 已选定的批量原子类型检查。这不构成整段默认表达式的事务。

## 二、栈重排、缺失参数与生命周期

### 1. `47` 调整的是实际参数槽

`vm_resortStack @0x080954a8`：原生栈向低地址生长，`defaultPushed=bp-vm_sp`，`newArgc=oldArgc+defaultPushed`。它先暂存已传参数，再按默认表达式产生的顺序取默认值，最终更新 argc 和 bp。

关键指令为 `0x080954d5/da` 计算默认值数量及新 argc，`0x08095501` 暂存已传参数，`0x08095508–62` 收集默认结果，`0x080955d3/de` 写回 argc 与 bp。Evidence ID：`ev_637a26f2121f5b0547c5cab5b7e08f6c925ae8aca1945de11e2008a57ed2da66`。

在 Rust 向上增长的栈中，等价顺序示例为：

```text
47 前：[arg1, arg0, default2, default3]
47 后：[default3, default2, arg1, arg0]  // arg0 仍由 bp-1 访问
```

这里需要将新增默认结果逆序后放到原实参之前；**只旋转整个区间会颠倒多个默认参数的逻辑顺序**。原生没有按 max 自动补空洞，也没有在 47 中再次调用参数类型检查。

**推导与限制：**Rust 应移动已有 StackEntry、保持 Cell/Reference 身份。原生回写主要是 tag、attrib、payload，未证明所有裸引用指针在移动后都会安全重定位；不能将稳定 Cell 设计描述成原生已经提供的安全保证。

### 2. 可选 mixed 不一定占槽

没有默认表达式且未提供的 mixed 参数可能始终没有槽位。`vm_pushLocalVar @0x080914dc` 对 `no >= argc` 读取 Undefined；`vm_pushLocalVptr @0x0809134c` 和 `vm_setLocalVar @0x0808f6dc` 对该情况报错。因此不能为了简化 Rust 存储而把 argc 一律扩大到 max，也不能默认允许对缺失参数取引用或写入。

在 47 执行前，默认表达式仍使用原 bp/argc；`1b` 之后才分配局部变量。参数读取不会自动切换为“已经求值的默认结果视图”。当前语料默认段没有直接 `37/39`，尚不能据此确认源码层面是否允许引用前面的形参。

### 3. 可变参数保持额外实参

| 函数/指令 | 地址 | 确认结果 |
|---|---|---|
| vm_pushArgNum / `34` | `0x080916cc` | 调用 vm_argumentInThisContext 取得当前有效 argc |
| vm_argumentInThisContext | `0x08093400` | 返回当前帧 argc；原生 funCp=0 时返回 0 |
| vm_createArgumentArray / `15` | `0x08091fb4` | 从 argc-1 到 0 调用 pushLocalVar，然后 createArray(argc) |
| vm_adjustArgumentNum | `0x08094ec0` | 调整 argc/bp；本次直接调用引用仅见于 vm_f_printf |
| vm_returnFromFun / `49` | `0x08094978` | 按更新后的实际 argc 加局部槽数量清理 |

`15` 是显式生成包含全部参数的普通数组，顺序为 arg0、arg1……；通过 pushLocalVar 读取引用形参时得到其值。它不是可变参数标志自动触发的打包，也不是只收集超额参数。`34/15` 在默认重排后看到有效 argc，而非永久保存最初传入数量。

参数计数汇编 Evidence ID：`ev_82ac3ae7fae8a726ca218b98efe44a8ac06570221896d15273b0f53feba22c1a`；数组构造汇编：`ev_8a50a757166bed6d254d5e96dafee266f7458421de87a7569ebbd797c419ffe0`。本轮也检查了名为 `vm_f_get_args` 的 efun，它读取启动命令行参数，不能把它当作当前脚本函数的可变参数数组。

原生错误可通过阶段 2G 已分析的 traceError/longjmp 清理内部帧；这些路径不撤销默认表达式之前完成的外部变量写入。Rust 首批继续保留终止错误与诊断帧，不引入原生异常恢复。

## 三、全量语料观察及真实样例

范围为 7,159 个程序、101,592 个序列化函数、175,100 个参数记录。统计脚本重新只读解压原始 PAK 成员，核对包和成员 SHA，再按原始偏移提取参数字段；不是只相信现有 JSON。

| 分类 | 数量 | 解释 |
|---|---:|---|
| min=max | 90,961 | 不排除其中带可变标志 |
| min<max | 10,631 | 全部以 0f 开头 |
| 含非零默认长度的函数 | 1,822 | 其中 92 个含引用形参 |
| min<max 且默认长度全零 | 8,809 | 可省略 mixed，不等于显式初始化 |
| 有非零默认长度的形参 | 2,538 | 长度大部分为 1；无负数 |
| 可变标志 bit0 置位 | 105 | 71 个有 0f 入口、34 个为空代码；71 个中 25 个还带 atomic |
| 允许 argc 区间涉及省略引用形参 | 1,219 | 首批 Rust 设计仍明确拒绝此调用组合 |
| 默认段含调用的函数 | 31 | 调用可能有副作用，不应常量折叠 |

87,774 个首 0f 函数全部符合已确认入口布局和 min 生成规则。1,822 个显式默认函数的 `a=Σ长度+1`，其余 85,952 个为 `a=0`。没有观察到累计默认区间越界、允许 argc 的入口越界，或无默认的 mixed 空洞位于后续默认表达式前。零默认且被省略的形参全为 mixed；这些是语料观察，不是语言禁止其他组合的证明。

8,920 个“参数描述符数不等于 max”的记录全部为空代码且 min=max=0。原生调用入口对没有代码的函数报“未实现”；不能根据这些声明的 min/max 判断真实实现的合法传参范围。

已精确解析的 10,947 条局部调用中，603 条 argc 超 max：13 条指向可变参数目标，另 590 条全部指向空代码目标。5 条 argc 低于 min；其余 10,339 条在原始范围内。这些是静态调用点分类，不能声称错误路径实际执行。全局、多 constitution、动态目标和继承偏移未知的调用不参与精确目标统计。

真实节点样例（偏移均为**解压成员内**，不是 ELF 地址）：

- `aaa/pack_data/lib_aaa32.pak/daemons/logd.o` 的 `browse_log`，函数索引 5、函数偏移 2296、节点偏移 2693，min=1/max=2、默认长度 `[0,1]`：`0f a=2; push_int 128; 47; 1b`。argc=1 跳到节点 1，argc=2 跳到节点 3。
- `ccs/pack_data/lib_ccs32.pak/ccs/clone/misc/store.o` 的 `update_last_use_time`，索引 31、函数偏移 7452、节点偏移 7623：默认段含 efun 06 槽 137。这里仅确认调用节点，不凭函数名推断该 efun 的业务行为。
- `ccs/pack_data/lib_ccs32.pak/ccs/daemons/ccs_serverd.o` 的 `send_to_server`，索引 10、函数偏移 4643、节点偏移 5254，attrib=1、min=1/max=4、默认长度全零，说明可变标志与显式默认初始化是独立维度。

完整逐函数原型、原始参数偏移、逐 argc 入口和省略情况见 [optional.json.gz](vm-optional-arguments-evidence/optional.json.gz)。执行以下命令只检查数据，不解释执行 VM 节点：

```sh
python3 docs/vm-optional-arguments-evidence/test_verify_optional.py
python3 docs/vm-optional-arguments-evidence/verify_optional.py
```

## 四、下一阶段 Rust 的确定实现方案

以下是未来阶段的设计，不是本轮已经开放的能力。

### 1. 模型、能力边界与公共接口

- 在现有 `Argument` 中增加 `default_action_count: i16`，由 raw[10..12] 解码；这是公共解析结构的新增字段，现有序列化格式也会增加字段。更新加载器结构比较与基线适配：旧压缩证据缺失该字段时，由本阶段经哈希核验的原始记录提供独立预期，不能让测试从被测 Rust 输出反推预期。
- PreparedFunction 保存 min/max、variadic 和默认入口布局，废除以单一 argument_count 同时表示声明和实际数量。Frame.argc 始终表示当前有效槽数；增加私有默认执行状态，记录原 bp/argc、待生成结果数及 47 的位置。
- 保留单 constitution、无继承、当前对象的范围；允许 variadic 位，继续拒绝 atomic、const、未知参数位、空实现和不合法的参数索引/入口布局。min/max 限制在 0..127，描述符数必须等于 max。
- 公共 Value 不增加 Reference，`Vm::start` 签名不变；包含引用形参的根入口仍要求内部包装调用。声明之外的可变实参只允许普通 Value，普通 mixed 也继续拒绝 Reference。
- 首批显式默认表达式仅允许线性节点 `3b/3c/3f/40/42/3d/16/18`：常量、对象变量读取与容器字面量。按段静态验证栈不下溢、每段净产生一个普通值；拒绝默认段调用、分支和其他 opcode。这是有意收窄的实现范围，并非原生限制。

### 2. 调用、默认执行与重排

1. `start/07` 预检 `argc>=min`；只有 variadic 允许超过 max。逐一检查已提供形参的值/引用绑定及资源限制，省略引用形参的调用返回 UnsupportedFeature。失败不改操作数和帧集合。
2. `0f` 对已提供的描述符参数执行阶段 2H 原子类型检查，并预先验证跳转目标。argc<max 时按已提供形参累计默认长度，选择尚缺默认段；否则跳过整段。额外可变实参保留但不按不存在的描述符检查类型。
3. 不为缺失的 mixed 提前补 Undefined。没有待执行默认结果时直接到 1b，argc 保持实际值；缺失槽的 37 返回 Undefined，39/4d 报 Bounds。
4. 有默认结果时逐指令执行，消耗正常预算。省略区间中，产生默认值的形参必须连续位于第一个缺失位置之后；尾部可保留无默认 mixed。出现中间空洞则拒绝该调用。默认结果由普通指令产生，不在 47 再做声明类型转换，以符合原生路径。
5. `47` 只允许在对应入口重排点执行一次，进入局部变量区前检查新增槽数与预期相同且均为普通 Value。将新增默认区间逆序，再移到原参数区间之前；保留所有已有 Cell/Reference 身份，更新 bp/argc，标记完成。只重排该帧区域，不接触调用者求值区。
6. `1b` 在默认状态完成后分配局部槽；返回按有效 argc 清理。保留默认结果此前成功产生的副作用；任何后续错误不回滚整个初始化过程。

### 3. 参数观察、资源和失败行为

- 新增 `34` 返回当前 Frame.argc；Rust 根入口是实际脚本调用帧，因此也返回其 argc，不模拟原生后端哨兵 funCp=0。
- 新增 `15` 生成包含全部有效参数的普通数组，按 arg0 开始排列，引用形参读出值、容器共享身份。先完成全部读取、数组及结果槽分配和限制检查，再提交引用目标的死对象归一化及结果；失败不部分修改源值。
- 普通非引用的死对象复制继续沿用现有 Rust 策略，不扩大本阶段的隐式源清理范围；保留这一兼容差异。
- 不增加依赖或 unsafe；沿用已有栈、帧、容器和引用额度限制。默认段与主体分别保留准确 PC/偏移；预算可在 0f、表达式、47、1b 之间暂停恢复。
- 原子性按单条检查/指令界定：07/0f/47 失败不提交该步骤；先前默认表达式或函数体写入不回滚。继续使用终止错误和重新 start/drop 清理，不引入 catch 恢复。

若三条新 opcode 均实现，支持集合才由 52 更新为 55；不能在本轮提前增加计数。完成后再根据真实准备结果报告能力，不能把形状候选数当作可执行覆盖。

## 五、实现验收与未知项

未来合成程序测试必须覆盖：min/max 边界、全缺省/部分缺省/全部实参、多种类型和多个默认值的顺序、可选 mixed 不补槽、默认值容器共享、多余参数直接访问、34/15 的有效数量、引用转发及返回清理、缺失引用拒绝、默认阶段预算暂停、对象删除、资源耗尽及失败无部分提交。默认段调用/分支、零默认空洞、负长度/越界和重复 47 均应有明确拒绝测试；不能静默当作 Undefined 或成功。

后续扩展仍需研究默认表达式调用/控制流、源码参数引用规则、原生移动槽位的裸引用安全性，以及跨对象/全局/动态调用和异常恢复。本轮已回答加载字段、入口目标、栈序、可变数量/数组及返回清理；全部编译语法和原生生命周期安全保证仅部分回答，保持未确认状态。

本轮统计脚本的 14 项合成测试通过，默认只读重算结果为 verified=true；未重复运行未改变的 132 项 Rust 测试。详细结果、原始制品和阶段 2G/2H 文件保护结果见 [stage2i-validation.json](stage2i-validation.json)。本轮不需要安装依赖；原版 glibc.i686、libstdc++.i686 是参考运行环境要求，不是此静态分析的执行前提。
