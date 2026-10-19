# 阶段 2AA：全局存储与调用上下文静态语义

## 结论与本轮边界

本轮交付静态证据、只读语料统计和[后续运行时规格](stage2aa-evidence/global-runtime-design.md)，不修改Rust运行时、公共API、依赖或支持集合。当前仍为阶段2Z的 **70种限定操作码**、359项Rust测试；主语料交集68种，缺05/06/17/35/36。没有运行参考ELF、启动脚本或业务字节码，尚无可运行服务端或原版动态差分结论。

使用 `reverse-engineer-anything` 与REA/Ghidra12.1.4：新取8条函数记录及7条导航记录，复用6个历史来源包的14条函数记录。主ELF SHA-256为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。会话已关闭。完整原始返回、伪代码、汇编、函数体范围与限制见[函数证据](stage2aa-evidence/functions.md)、[清单](stage2aa-evidence/function-manifest.json)及压缩包；导航查询另存，避免将字符串搜索结果当作语义证明。

## 全局值槽：观察与推导

| 函数 / Evidence | 已观察路径 |
|---|---|
| `vm_pushGlobalVar@0x0809113c`；`ev_f5d36761261bd90bb2139ae2bcd05e79af64a90eaffe2b978b3c0353a3ffe4a2` | 从vm_pGlobalVar[index].pValue取槽，检查值栈，压入Undefined占位后调用vm_copyValue |
| `vm_pushGlobalVptr@0x080910d0`；`ev_e0b26424bac92dc860aadf077630177dfff6e85d228ddea8d8a8e7a77fb8b7ee`、`ev_0afa29523e8e05f218ef05cec068d897509aa00f512de38f9de00d12fb0b2ff1` | 取同一槽，增加槽ref，压入tag8引用并复制**值槽attrib** |
| `vm_getGlobalValue@0x08090ad4`；`ev_b9ce656e748a1b80a2c6e5f0400f66705f486faea386f140748ad05c4527bb9d` | 返回全局表中的pValue，不做值复制 |
| `vm_getNewGlobalVarOffset@0x08080770`；`ev_2bce189c12465150bd034fd31e9af6028ea48c8031b70df5eedb7033f126abd2` | 返回旧vm_globalVar后递增；不是函数编号分配器 |
| `vm_endNewFile@0x08088874`；`ev_826e3b39ae0edc9b4bee9eabaac0dbdb7ee0500573ec5a85ea88369b044ceba6` | 成功完成文件时扩展全局表，逐槽分配值；失败时unref新增槽并退回数量基线 |
| `vm_safeNewValue@0x08089f78`；`ev_6ad4e4a31f4846ef1c2950ccd0df173f861e78b97f61852ca9fbd4a30a4bf86c` | 分配独立24字节值，type/attrib初始0、ref初始1 |
| `vm_finalizeIdentifiersInProgram@0x080854b4`；`ev_a62e3110c8f80b68de1c768894f66d34b9dca9778fa69b0bb7f84f2504e73e0c` | 全局变量标识符分支将名称和声明属性写入表条目 |
| `vm_destructIi@0x08087640`；`ev_b4ed8ce5b95fd374e42fe1750e7879f34683730bc1a71eadda3c3c2707dc450e` | 某些变量标识符销毁分支unref值槽，并清空表中的pName/pValue |

全局表指针为 `0x0847aacc`。35/36中index×3×4的寻址，以及endNewFile的分配/重分配确认条目stride=12；pValue在+0、pName在+4、表attrib在+8。值槽本身另行分配，不能把表重分配误当成值槽地址变化。

35在 `0x08091183` 调用vm_copyValue；复用记录 `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` 显示容器浅拷贝、共享引用和直接失效Object的源槽规范化。36在 `0x0809110f` 从值槽+5读取attrib、`0x08091115`增加槽ref、`0x0809111b`写入新引用的attrib；不能将这个attrib与表条目+8的声明字段视为同一字段。全部属性位在全局生命周期中的传播尚未闭合，后续首批仅支持显式合成的可写槽。

endNewFile在 `0x08088d53` 分配全局表、`0x08088e7f`重分配，在 `0x08088e28`调用safeNewValue逐槽创建Undefined值；已有pValue指针随表复制。失败分支 `0x08088ee7` 调用unrefValue并清空新增指针、恢复vm_variableBC。完整编译器事务、并发热更新和进程关闭不在已恢复范围，不能把该函数成功/失败路径当作任意卸载事务保证。

destructIi在 `0x080876fc` 调用 `vm_unrefValue@0x0808bfb4` 后清表；复用 `ev_7caa046f8081143ac8be588f8afdae8a94f7aa7a36d7146bc503150e7f9475a0` 显示仅ref减至0时清理并释放值。结合36的ref增量，**已有根引用可在表所有权释放后继续持有旧槽**。这是选定分支与引用计数的静态推导，不是所有全局卸载场景的动态证明；后续Rust不能仅因宿主移除银行就让该引用指向新槽或失效。

35/36在取表中pValue之前未见索引边界保护，且取指针早于栈检查。Rust规格选择显式准备/运行检查及失败前不修改源，不复制非法索引的原生内存访问。Ghidra对destructIi的恢复参数/结构字段存在误类型，具体unref与清表结论以汇编寻址为依据，不把反编译中的ihe字段名当作原结构定义。

## 全局07、返回与06边界

复用07及调用体记录：`ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97` / `ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347`，`ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` / `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41`。

- 07的par8按bit7选全局，低6位为argc，bit6在该分派路径不参与；全局函数索引par16直接选择vm_pGlobalProgram.pFunctions，不增加当前对象的继承函数基址。
- 调用体保存新层bp=vm_sp、argNum=argc、代码段与函数；全局分支将thisObject设NULL并使用全局constitution。对象调用则选择或继承对象上下文。全局代码不能借用调用者ObjectId来满足当前Rust帧约束。
- `vm_returnFromFun@0x08094978` 的 `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` / `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` 展示共同的返回类型检查、局部/参数清理、恢复上一调用层，以及仅在退出对象非NULL时unExecRef。返回hook、完整原生错误恢复和生命周期组合仍另列限制。
- `vm_pushThisObject@0x080975d8`，`ev_0f52440a23f0b659faa95e8a973f08f1b228163126ef3957b926b6685f6d9b78` 明确在thisObject为NULL时压入整数0；不能伪造ObjectId(0)。

`vm_callInEfun@0x08093d98`，`ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f` 则复制上一上下文，更新bp/argNum/flags后直接调用注册函数指针；与07清空thisObject并切入字节码的契约不同。其原生范围检查使用全局函数表容量，不足以单凭编号保证目标是合法efun。后续宿主注册器必须校验槽类型；本轮不实现06。

`vm_initIdentifiers`和`vm_loadIdentifiers`的复用记录表明全局标识符/函数表与vm_globalVar是不同计数及存储。559个efun槽、启动单元function_base=572以及全局变量容量不是同一编号空间；不能拿572作为变量基数，也不能把559..571的预置全局函数槽自动映射为变量或efun。

## 只读语料结果

完整方法、重算摘要及发生位置见[语料方法](stage2aa-evidence/corpus-method.md)。主语料7,159程序、101,592普通函数、6,474,773指令中：

| 静态项目 | 数量与边界 |
|---|---|
| 35 | 17,500次：函数17,116、初始化384 |
| 36 | 3,929次：函数2,669、初始化1,260 |
| 全局位07 | 25,574次；25,524次在各启动单元定义区间，50次在预模块全局区间 |
| 06 | 143,520次，全部编号低于559；这是当前语料事实，不是完整动态分派证明 |
| 预模块调用 | 559/564/569/570分别5/10/10/25次，均在559..571；未判为越界，也未猜测目标名称或运行时绑定 |

五个mode=global启动单元的function_base均572、efuns_count均559；AAA/CCS/CSA/DBA/GS的max_vars分别105/102/98/98/359，max_functions分别846/1002/982/988/1880。其解析variables描述列表均为空，但这不表示没有全局值槽。原始DLT、函数槽及定义区间分别保留在[全局单元清单](stage2aa-evidence/global-units.json)，不从函数DLT混推变量类型/初值。

35/36在对应服务前缀唯一映射的全局容量内均无越界；此映射用于静态核对，不在Rust运行时引入按路径自动选择银行。保留47,003条35/36/全局07位置和17个样例；06另存完整计数/直方图。偏移为解压后成员内nodes_offset+4×PC，非ELF地址。统计不推断实际值类型、可达性或执行成功。

## 完成度、未知项与交付

| 问题 | 本轮结论与剩余限制 |
|---|---|
| 35/36读、引用、索引 | 选定解释路径已恢复，非法索引行为不作等价承诺 |
| 值槽分配/表扩展/释放 | 成功完成文件、失败回退、变量标识符销毁分支已恢复；完整热更新/关闭仍部分未知 |
| 属性与初值 | 初始Undefined和36值属性传播已恢复；真实全局符号/全部属性传播、服务初值仍未闭合 |
| 全局07及普通返回 | 全局代码/对象上下文分离、参数数、返回恢复已恢复；hook/回调/线程/JIT/完整catch不在范围 |
| 559..571预置绑定 | 已保留50处静态调用与数字区间，未证明13槽名称、类型和完整启动绑定 |
| 动态兼容 | 未执行原版或业务，不作运行等价或服务启动结论 |

下一批只实施显式合成银行与35/36，之后再实施受限全局函数准备及07。接口、引用保留、移除/重装、帧迁移、错误和验收用例已锁定在[实现规格](stage2aa-evidence/global-runtime-design.md)。

本轮证据/统计脚本测试、现有359项Rust测试、fmt、严格clippy、当前2Z矩阵只读重算及全量语料检查结果见[阶段记录](stage2aa-validation.json)。原始36制品、Rust、现有测试、Cargo和历史证据保持不变；全量比较继续明确报告唯一已知DBA LZSS异常。
