# 阶段 2AN：Global／efun 函数值静态语义

本轮接续2AM，只交付取证、独立语料复算及[后续实施规格](stage2an-evidence/function-runtime-design.md)。Rust运行时、公共API、依赖、75种限定支持与73种主语料交集不变；Global／efun函数值仍未开放，阶段2未完成。没有运行参考ELF、启动脚本、init或业务字节码。

## 证据范围

目标SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。使用reverse-engineer-anything及REA/Ghidra12.1.4，新会话取得3条完整dossier：`vm_callInEfun`、`vm_callInUserFunBody`、`vm_finalizeFun`。第一条Evidence ID与历史相同，是刷新观察；后两条是新dossier ID，不等于首次发现这些函数。另复用17条完整历史记录，其中部分原本只有伪代码或汇编，不升级其证据等级。

[函数清单](stage2an-evidence/functions.md)、[完整清单](stage2an-evidence/function-manifest.json)、[19个机器码锚点](stage2an-evidence/anchors.json)与[14项结论](stage2an-evidence/findings.json)保留出处、观察／推导／设计／未知和限制。REA会话已关闭。伪代码不是原始C源码；误类型的union字段用汇编及注册路径核对。

## 创建、owner和两种分派

| 问题 | 静态结论及依据 |
|---|---|
| 17标志与槽 | `vm_createFunctionValue@0x08091f34`，`ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`：bit7选全局表，bit6=1选K、=0选c，低6位为捕获数。全局槽为绝对编号；局部槽加constitution函数基址。 |
| 保存入口 | `vm_createFunctionValueEx@0x08091cd0`，`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`：从选定程序表复制入口到函数头`+0x14`。同时独立保存当前thisObject至`+0xc`；即使目标为全局函数，也可能保留创建者对象。全局上下文创建时可为空。 |
| owner失效 | `vm_callFunPtr@0x0809441c`，`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`：先检查保存owner的删除位，再分派K/c；global不绕过该检查。普通copyValue可能清理源并生成Undefined；原始05与先复制再调用不是同一行为。 |
| Global/c | 同上及`vm_callInUserFunBody@0x08093eb0`，`ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c`：传保存的pFun、funOffset和isGlobal。Global帧清空thisObject，从**当前**vm_pGlobalProgram取得constitution，代码段来自**保存的**pFun。 |
| Global/K | 05在`0x0809483a`读取保存描述符`+0x1a`的signed编号，再调用`vm_callInEfun@0x08093d98`，`ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f`。后者重新读当前全局表并调用其原生入口；不能把保存描述符直接当作冻结回调。 |
| K编号来源 | `vm_addEfuns@0x08085ce4`，`ev_a83bf890b4b1b7b1e670ffd84e2caf433106c4b1521c658ee8c18bbdb826b9a1`：依序分配函数号并写入efun描述符。数值落入容量不证明条目类型正确。 |
| efun上下文 | callInEfun复制上一层40字节上下文，再设置bp、argNum、flags和f。其对象、代码等来自**调用者**；函数值保存的创建者owner仅用于寿命／失效检查，不自动成为宿主当前对象。 |

例：Object A创建Global/K值，Object B调用时，删除检查针对A，efun当前对象来自B。Global/c则在callee中没有当前对象。不能用单一ObjectId兼任创建owner、目标身份和调用上下文。

## 参数、返回和绑定寿命

05的argc包含目标。捕获`[B0,B1]`和新参数`[N0,N1]`形成逻辑顺序`[B0,B1,N0,N1]`；K/c都先处理捕获，再分派。唯一函数头移动捕获并清空原槽；共享头经pushValue/copyValue复制。首批标量捕获不需要模拟非标量的全部复制源清理差异。

K走efun参数和返回契约，不能套用用户函数入口转换。`vm_returnFromEfun@0x08093c5c`、`ev_e86eb89e60b57e211e2efdce9ee32fdd5b9d630f548d18ca4e6b50e43675595c`保留Void／Mixed／Exact及上下文恢复的既有证据。最后函数头释放捕获和保存owner；一般catch、hook、异步和重入未闭合。

`vm_finalizeFun@0x08095794`，`ev_7ad5b825f58978d24db93a52b2b74151085c158ff48f1957a00188e9292c6a2d`将pFun装入编译中程序的函数表；复用finalizeIdentifiers记录可见隐藏定义处理。这些更新点不证明旧入口在任意重编译／卸载后仍安全，尤其Global/c存在保存代码与当前constitution的组合。

因此后续Rust规格采用明确受限策略：Global/c创建时保存ProgramId，新调用要求该程序仍为当前绑定；清除或重绑后拒绝旧值的新调用，已进入帧继续使用保存代码。Global/K保存规范槽号，每次调用取当前显式宿主注册及签名。后者是Rust合成注册表策略，不宣称模拟原生描述符卸载。

## 独立语料复算

新扫描器不调用2AF扫描实现，重新读取固定程序结构，逐条核对全部6,474,773条指令。结果为7,159程序、101,592普通函数、05共3,089处、17共32,614处；Global/K 4,902处、Global/c 110处。5,012个全局创建位置的成员哈希、作用域、函数、PC、偏移和操作数与2AF逐条一致。

[完整位置](stage2an-evidence/corpus-occurrences.jsonl.gz)、[分类清单](stage2an-evidence/target-inventory.json)和[代表样例](stage2an-evidence/corpus-examples.json)按选择器、槽、Object／Global、init／普通函数及捕获数分组。名称只映射固定ABI的数字槽；05动态栈目标没有被静态解析。偏移属于解压后的成员，不是ELF或PAK压缩位置。静态出现不证明执行可达或参数标签。

## 结论边界

两批规格已固定准备入口、公共接口迁移、目标表示、上下文、重绑、错误提交和[30个静态／设计案例](stage2an-evidence/expected-cases.json)。案例不是原版运行输出，也未成为新Rust运行时测试。

G12／G18仍开放；G06／G07／G14的循环回收、完整复制源清理和延迟删除仍未闭合。非标量捕获、非global/K、字符串／OS、继承／hidden、生命周期、Global init、真实efun／bootstrap及服务继续排除。复现和验证限制见[方法](stage2an-evidence/method.md)。
