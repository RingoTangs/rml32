# 阶段 2K：继承、constitution 与函数重定位

## 总体结论

继承支持应实现为“程序目录 → 依赖链接 → constitution 与函数槽表 → 调用上下文”的独立层，不能通过复制父函数、按名字覆盖全部函数或合并字符串池替代。原生函数槽保留独立索引，函数体可以共享；调用时由槽关联的 constitution 决定函数和变量基址、所属字符串池。

本阶段只新增静态证据、统计脚本和 Rust 设计，**没有开放继承运行能力**。阶段 2J 的 55 种限定 opcode、150 项测试是既有基线，本轮未重跑或修改运行时测试。没有执行参考 ELF、启动脚本、初始化代码或业务字节码；公共 API、依赖和既有文件保持不变。

引擎为 REA MCP 接入的 Ghidra 12.1.4，分析会话已关闭。目标 `magic_Linux32` 的 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。本轮新增 41 条记录，复用 10 条记录；[索引](vm-inheritance-evidence/index.md)、[可读汇编/反编译](vm-inheritance-evidence/functions.md)和[统计方法](vm-inheritance-evidence/method.md)包含完整来源。以下区分原生确认、语料观察、推导和未来设计。

## 一、加载格式与 constitution：原生确认

### 1. 六字段的确切含义

`vm_loadInheritPrograms @0x0808729c` 读取父名称及继承列表，通过对象名称查找父程序，再调用 `vm_loadBinaryInherit @0x080a2b58`。六个 Vm_Index_T 均为 signed16，顺序为：

| 序号 | 字段 | 单位/验证 |
|---|---|---|
| 0 | baseConsititutionOffset | constitution 项索引 |
| 1 | baseFunOffset | 函数槽索引 |
| 2 | baseVarOffset | 对象变量槽索引 |
| 3 | expectedSubConsititutionNo | 父 constitution 容量 |
| 4 | expectedFunctionNo | 父函数表容量 |
| 5 | expectedVariableNo | 父变量容量 |

父程序的三项实际容量必须与后三项相等；函数和变量范围须落在子容量内。汇编 `0x080a2ceb/0x080a2cf4` 对函数区间求和并比较，`0x080a2d0e/0x080a2d14` 检查变量区间。Evidence ID：`ev_c41c22c1afd1192242c38bb59ffa5cc3402d157828307978e735721a4489e3d0`。

原生使用 `vm_lookupObjectByCStr(name)` 查找父对象，不是直接按当前工作目录读取 `.o`。本阶段同包 `.c→.o` 是显式离线目录策略，不能当作原生运行时命名空间或自动加载规则。

### 2. constitution 的布局与重定位

DWARF 结构大小12字节：+0 pProgram，+4 attrib，+6 baseFunOffset，+8 baseVarOffset。类型 Evidence ID：`ev_582a03dd695738a8d26aac5a532e68919f7c76024a231dc643f2b48941b58b84`。

`vm_buildProgramHead @0x808651c` 确认初始 constitution 0 指向自身、基址为0。Evidence ID：`ev_aa12d41f4394aa209415defcd72e5fc2cfbdee8acf9ff95fe8d33af97ed6b264`。

父表第 j 项复制到子表 `baseConstitution+j`，所属程序指针保持不变；函数及变量基址分别加上本次继承偏移。`0x080a2d73/0x080a2d7f` 使用16位 ADD，实际有截断风险。父根项标记 attrib bit0，嵌套项通过其所属直接父的整段复制导入。

**Rust 安全策略：**将算术扩大后检查 signed16 范围，拒绝溢出；显式检查 constitution 完整范围和自身槽0。这里的完整上界检查不等于原生函数内存在完全相同的防护。

初始化顺序同时重定位：

```c
for (j = 0; j < parent.constitution_count; ++j)
    child.init_order[cbase + j - 1] = parent.init_order[j] + cbase;
child.init_order[child.constitution_count - 1] = 0;
```

这是静态序列构建，不是执行初始化的授权或正确性证明。

## 二、函数槽、名称与实现绑定：原生确认

### 1. 导入槽位，不按名字扁平化

`Vm_Fun_Entry` 为8字节：+0 pFun，+4 constitutionNo，+6 attrib。类型 Evidence ID：`ev_e6e788fc95d1403d2daa1f3a3db6b4e5ed24081078ff520e5395a292c3d3e8f5`。

`vm_loadIdentifiers @0x08087fb8` 分配并清零函数表；`vm_finalizeFun @0x08095794` 将本地函数按 DLT 索引写入，constitution=0。`vm_gatherProgramFunctionTable @0x080a31bc` 扫描直接父根项，将父函数槽复制至对应函数基址：

```c
child.functions[c.base_fun + k].body = parent.functions[k].body;
child.functions[c.base_fun + k].constitution =
    direct_parent_constitution_index + parent.functions[k].constitution;
child.functions[c.base_fun + k].attrib |= parent.functions[k].attrib;
```

汇编 `0x080a3212–40` 支持上述索引、constitution 累加、pFun共享和属性OR。Evidence ID：`ev_2b87bd494b66569995a2fd20b9c75f34d7fbc63d63d1b161f3aa5e6ed6113c16`。父子同名且都有代码的两个槽不会因此合并。

### 2. 只有声明的槽另行寻找实现

同一 gather 函数随后按槽号升序处理“pFun存在但代码为空”的声明。对每个声明，先从其前一个槽向0搜索，再从后一个槽向表尾搜索。同名候选必须有代码，且 `Fun.attrib & 4 == 0`、`Entry.attrib & 1 == 0`，即已经形成的别名不再次作为候选。

原生比较参数链表数量、type、varAttrib及返回type；反编译与汇编未见在该比较中检查引用flag、min/max或返回attrib。已加载默认表达式树指针为null，不能凭该路径恢复源码默认声明冲突。

匹配后声明槽的 Entry.attrib 设置bit0，pFun和constitution改用候选；槽号保留。`0x080a32ed/f3` 是候选过滤，`0x080a3388–9a` 比较参数type/attrib，`0x080a34ad–b7` 和 `0x080a3520–2a` 写回别名。原型不匹配会报告编译错误，不应任意选一个同名实现。

`vm_inheritFunctionPrototype @0x080a28fc` 还会更新标识符及可见原型，`vm_overrideFunction @0x080a21f0` 处理编译器标识符备份；这与运行时函数槽表是不同层。不能仅用 DLT 中的名字判断最终覆盖。相关 Evidence ID：`ev_f106a1ad196036705391e3d2710b9e91f713c16845951d9656ce1036e69e1c66`。

### 3. 构造/析构会更新属性

`vm_createProgram @0x080b13f4` 在编译/加载完成后将 create 的函数属性设置bit0，并保证bit2；destruct也保证bit2。这说明序列化属性不必等于最终运行态属性。Evidence ID：`ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d`。

本轮统计不模拟这些属性变化和最终名称别名绑定；非空函数体身份仍可保守确认，但不能据此推断最终参数约束和所有名称查找结果。

## 三、调用上下文与数据归属

| 路径 | 原生地址 | 观察 |
|---|---|---|
| opcode07 / vm_callInUserFun | `0x08094080` | 对当前constitution.baseFun加有符号函数操作数，取当前对象函数表 |
| vm_callInUserFunBody | `0x08093eb0` | 由目标槽constitutionNo切换上下文；cs指向实际pFun.codeSegment |
| 3d / vm_pushObjVar | `0x08091224` | 读 thisObject.values[baseVar+operand] |
| 3e / vm_pushObjVptr | `0x080911a8` | 引用同一个重定位后的对象Cell |
| 4e / vm_setObjVar | `0x0808f7f8` | 清理并写入重定位后的对象槽 |
| 40 / vm_pushSharedString | `0x08090cf8` | 字符串来自当前constitution.pProgram，不是对象根程序统一字符串池 |
| vm_regularProcessor | `0x080978c4` | metadata来自当前cs，与实际函数体代码段一致 |
| vm_returnFromFun | `0x08094978` | 清理实际参数/局部槽、弹帧并恢复上一上下文 |

变量读取、引用、写入、字符串和处理器均有本轮反编译/汇编证据，见索引。例如字符串 Evidence ID：`ev_036c7ff9e3a4c3a9d1380520976755debc306a776b18e23161519177ae073205`；metadata Evidence ID：`ev_5673fa5b2edb4d5bf79e9f9431a44467493506747573d429692a62ef74c6c5de`。

**推导：**对象存储属于最派生对象，函数体和常量属于实现来源程序，两者必须分开。把父代码移动到子程序并沿用子字符串池会改变行为。继承方法切换constitution不等于切换thisObject；引用应保留同一对象ID和绝对Cell身份。原生裸指针在对象删除、热更新等情形的完整安全性未在本阶段证明。

## 四、全量语料结果与边界

[摘要](vm-inheritance-evidence/summary.json)与[完整紧凑图](vm-inheritance-evidence/graph.json.gz)覆盖7,159个程序和9,933条继承边。所有边按离线目录策略唯一解析，父容量全部匹配；没有观察到循环、重复直接父、同一祖先的多路径、区间越界、兄弟区间重叠、嵌套基址signed16溢出或constitution空洞。

| 统计 | 结果 | 计数边界 |
|---|---:|---|
| 最大继承深度 | 5 | 最长父边数，叶为0 |
| 直接多继承程序 | 1,956 | 不等于菱形继承 |
| 深度0/1/2/3/4/5 | 1884/2034/2125/392/712/12 | 包含5个global的无父节点 |
| 重建函数槽（已填充） | 713,213 | 跨各root表，含共享父体的重复映射 |
| 非空实现槽 | 482,971 | 可确认函数体身份，不代表Rust支持该函数 |
| 未做别名解析的声明槽 | 230,242 | 保持未知，不伪造最终实现 |
| 空槽 | 2,860 | 不自动替换为其他函数 |
| 重建初始化顺序 | 7,154 | object程序全部为有效排列，自身0最后；5个global不构造 |

本轮只对各分析程序**本地functions**中的原始07节点统计，不重复枚举共享父体在所有对象中的执行上下文，也不包括init节点：总数124,759，其中具体实现目标100,119、声明别名未知773、global bit7未知23,867。该总数不能与包含init的全语料07总量126,466混用。

继承分类DLT记录15,850条；raw[8:16]中15,160条非零。原始20字节、成员内偏移及索引完整保留，但本轮不把其中旧指针字段当作有效宿主指针或最终覆盖依据。语料hidden函数数为0，不能据此证明原生不支持hidden。

真实样例：`aaa/pack_data/lib_aaa32.pak/daemons/datasaved.o` 的三个父记录分别为 `[1,0,0,1,20,2]`、`[2,20,2,1,20,0]`、`[3,40,2,1,6,1]`；子容量为constitution4、函数51、变量3。父变量区间依次为[0,2)、[2,2)、[2,3)，零长度区间不是重叠。

另一个可核查调用：`aaa/pack_data/lib_aaa32.pak/aaa/clone/account.o` 本地函数槽71，pc3，成员内节点偏移476，07操作数a=15/b=1；本轮模型确定目标槽15、constitution3。完整来源body通过图的program/body字典定位；不凭显示函数名做匹配。

脚本只重建 **名称别名解析前的表**。gather第二阶段只重写空声明，因此不发生冲突的非空实现槽可确定身份；父声明即使原生可能已经绑定，本轮仍保留未知。报告不模拟任何函数体执行、构造调用或运行态名称查询。

## 五、下一阶段 Rust 的确定链接设计

以下是未来实现方案，本轮没有修改接口。

### 1. 明确程序目录与原子链接

新增 `ProgramCatalog`：按调用方提供的String键保存ProgramImage；通过 `bind_inherit(child_key, inherited_name_bytes, parent_key)` 显式绑定父依赖，名称按原始字节匹配。不搜索文件系统、不大小写折叠、不自动跨包猜测。

新增 `prepare_linked_program(&catalog, root_key, limits) -> Result<PreparedProgram, VmError>`。既有 `prepare_program` 保持单程序入口；ObjectStore.register_program/create签名保持不变。仅链接完整成功后才提交对象存储，不部分注册程序，也不执行init/create/destruct。

只接受Magic559的object根程序及object父依赖，global程序仍不进入对象链接路径。依赖闭包按父先于子的拓扑顺序处理。首批支持单继承、多层及无共享祖先的多继承；循环、缺失、歧义、重复祖先/菱形、隐藏函数、重叠区间和溢出明确拒绝，不猜测原生特殊组合。LinkLimits默认依赖程序上限1024、深度64、累计函数槽1,000,000；各表容量及所有重定位结果必须落在0..32767。

### 2. 分离函数体与分派槽

内部保存三类不可变数据：来源程序的字符串及metadata；root的constitution表；root的函数槽表。槽使用Missing、Unimplemented(prototype)、Bound(body_id, constitution_id, alias)三态。相同函数体使用共享Rc，不复制或重写父字节码。

先初始化本地槽，再按父段导入父已经链接完成的槽和constitution，复制alias标记。随后按原生顺序处理空声明：优先向低槽倒序，再向高槽正序，按**原始函数名字节**匹配，过滤Fun.attrib bit2和Entry.alias。比较参数数量/type/attrib和返回type；额外要求引用flag与返回attrib一致，不一致返回UnsupportedFeature，明确这是Rust首批的更严格约束。min/max及默认入口采用最终实现，不拿空声明中尚未生成的0/0容量作比较。

无候选的声明保留Unimplemented，调用时明确报UnsupportedFeature；原型类型冲突报InvalidProgram。涉及create/destruct的名称别名与入口执行首批拒绝，避免忽略原生加载后的属性更新。其他构造、析构及初始化执行留给独立阶段。

最后才准备非空函数体，沿用阶段2J的opcode与参数能力检查。任何必须准备的实现含未支持指令时明确失败；不能以无操作或默认成功代替。这个链接步骤不增加opcode集合，仍为55。

### 3. 运行帧、引用与错误定位

Frame保留root对象及其ProgramId、分派槽号、实际BodyId和constitution索引。07使用当前constitution基址重定位；进入目标槽后切换body/constitution，thisObject不变，返回恢复原帧。3d/3e/4e先求绝对变量索引并检查root容量；40使用所属程序字符串，长数值和默认段使用实际body的metadata。

ObjectStore每个对象只分配root.max_vars个稳定Cell。引用绑定保存ObjectId及Cell身份，不再把相对变量操作数作为持久引用目标。默认/可变参数检查、47重排和额度凭据继续沿用阶段2J规则。

错误字段约定：program为root ProgramId、function为root分派槽，source及pc/offset指向实际函数体来源。链接错误注明root、父名称、继承序号和记录来源；数量/布局错误为InvalidProgram，超限为Limit，未开放组合为UnsupportedFeature。失败不改变已有已注册程序或对象。

## 六、验收与仍未确认的边界

本轮19项合成元数据测试及默认只读复核通过。测试包含单层/多层、直接多继承、菱形/重复父、循环、缺失/歧义、计数/范围/重叠、signed16溢出、保留自身槽、初始化顺序、同名实现不合并，以及有同名候选的空声明仍保持未知。

```sh
python3 docs/vm-inheritance-evidence/test_verify_inheritance.py
python3 docs/vm-inheritance-evidence/verify_inheritance.py
```

未来运行时验收须覆盖：继承方法调用自身及其他父方法；相对变量读写和双对象隔离；相同字符串索引在不同父程序中取不同值；声明绑定后的constitution切换；引用转发、默认参数重排、预算暂停及返回恢复；缺失实现和对象删除后的失败路径。

已回答六字段、constitution布局与累加、函数槽导入、声明绑定算法、数据池归属和初始化顺序。部分回答/未确认项包括完整原生对象命名空间、热更新、hidden注册、重复祖先语义、全部编译器标识符备份行为及裸引用安全。统计没有实现最终名称绑定，不能将100,119个具体节点目标等同于业务执行覆盖。

原始制品、阶段2I/2J和既有文件保护结果见[stage2k-validation.json](stage2k-validation.json)。本轮不需要安装依赖，也不需要启动依赖glibc.i686、libstdc++.i686的原版程序。
