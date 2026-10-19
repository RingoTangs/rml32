# 阶段 2AF：函数值05/17静态语义与实施设计

本轮交付原生证据、全量只读统计与[首批实施规格](stage2af-evidence/function-runtime-design.md)。Rust运行时、公共API、依赖及73种限定支持集合保持2AE状态，05/17仍被拒绝；433项Rust测试基线保持通过。本轮没有执行参考ELF、启动脚本或业务字节码，不宣称完整VM或服务端兼容。

## 证据与问题清单

使用reverse-engineer-anything技能与REA/Ghidra12.1.4；目标SHA-256为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新增10条完整函数dossier，复用5个历史包的11条记录；[函数清单](stage2af-evidence/function-manifest.json)固定地址、来源及限制，[可读视图](stage2af-evidence/functions.md)保留伪代码/汇编，[结论账本](stage2af-evidence/findings.json)区分观察、推导、设计和未知。REA会话已关闭。

| 问题 | 状态及依据 |
|---|---|
| 17操作数与作用域 | 已回答：`vm_createFunctionValue@0x08091f34`，`ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`。bit7为global，bit6选择K(0x4b)/c(0x63)，低6位为捕获数量；局部槽加当前constitution.baseFunOffset，汇编ADD SI保留16位结果。 |
| 创建与捕获 | 已回答：`vm_createFunctionValueEx@0x08091cd0`，`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`。拒绝tag8引用、保存owner及函数入口，参数从栈顶顺序移入捕获区，最终压tag9。 |
| 05参数与分派 | 已回答：`vm_callFunPtr@0x0809441c`，`ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`。argn包含栈顶目标；先消费目标，随后把捕获参数前置。K进入efun，c进入用户函数；字符串走另一套名称/OS分支。 |
| 复制、释放与身份 | 已回答到选定分支：`vm_copyValue@0x0808be24`，`ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`；`vm_freeFunctionValue@0x0808c114`，`ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9`；`vm_cleanupValue@0x0808ad98`，`ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`。 |
| 相等与真值 | 已回答：`vm_isEq@0x0808c908`，`ev_c98c01d582d90083a8c48681f7d36bdd274bd35ed834c0ef8fe22e6ac304baf1`；`vm_eqValue@0x0808eed4`，`ev_9c31d92aeb045a089ff6361f0cf81330894208eb0520a78f0bf4bd4701c8d565`；复用`vm_isZero@0x0808cad4`，`ev_174f52005ca1def1dcd28fda64fa608062b04eaeef2672493386d0ad773ab397`。 |
| 返回与错误展开 | 部分回答：复用user-body、return、traceError/doActions/errorQuit路径，并结合tag9清理；完整活动对象删除、调度、hook/JIT和循环回收未恢复。 |
| 动态目标/真实可达性 | 本轮不回答：05目标是运行时栈值，静态操作数不能构成动态调用图。 |

以下结论均为静态观察或明确标出的推导，不把反编译伪代码作为原始C源码。Vm_Value/Vm_Fun_Ptr被Ghidra交叉误类型；指针别名和字段含义须结合汇编，不把伪代码中的pPtrValue/pNext等别名按Rust结构照搬。

## 创建、捕获与编号

17调用Ex时传入global、K/c选择、函数槽、捕获数量和当前thisObject。非global槽先做当前constitution函数基址重定位；global槽直接使用。K由05确认走vm_callInEfun，不能把bit6称为引用标志，也不能仅按槽小于559推断完整调用安全性。

Ex先检查待捕获项的原始tag8并报错，再分配24字节函数头与argn个原生值槽。头保存ref=1、type、isGlobal、argn、funOffset、pObject、pArgs和当时函数表中的入口f。保存的是入口快照，并非每次按名称重新查找。若owner存在且未被删除则增加其ref；已删除则置NULL。后续非global访问pObject->pProgram没有完整防御，不能据此为Rust构造NULL owner的正常语义。

捕获项0来自创建时栈顶，项1来自下一项；原值的type/attrib/payload移动到pArgs，原栈项清零并弹出。这是按值槽捕获，不是引用局部变量的闭包环境；所捕获的Array/Mapping等原生共享头仍可能共享。引用被拒绝不等于深复制所有捕获值。

函数槽、对象方法名称、efun槽和全局变量索引是不同概念。即使17的global数值落在某区间，也还要结合K/c、表项类型和完整加载状态判断；本轮统计只报告数值命名空间。

## 05调用、参数顺序与上下文

原生argn必须至少1，栈顶是目标本身，实际新增参数为argn-1。par16参数名mayNull，只在字符串名称缺失路径中决定是否消费参数并返回Void；函数值类型错误和owner删除错误仍报错。主语料par16只有0/4，不表示05操作数是函数目标编号。

对于tag9，05检查捕获入栈空间及保存owner的删除标记，取出快照入口/作用域/槽，再把捕获参数压到新增实参之前。若捕获为[B0,B1]、新增实参为[N0,N1]，被调用者的逻辑顺序为[B0,B1,N0,N1]；Rust向上增长Vec应安装反向排列的实参区，不能直接拼接底层栈切片。

ref==1时逆序移动捕获槽到栈，并清空源槽，再释放函数头；ref>1时先释放被消费的函数值句柄，再通过vm_pushValue复制捕获项。`vm_pushValue@0x08091074`，`ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212`调用vm_copyValue。**推导：非标量捕获的观测行为可能受复制与移动路径影响**，尤其复制失效Object/Function时的源规范化；不能用无条件深复制冒充两条路径。

K调用vm_callInEfun；c调用vm_callInUserFunBody。Object函数值保存的目标对象与当前thisObject不同才显式传pToObject；相同则交由user-body继承调用者对象。global调用清空thisObject。复用`vm_callInUserFunBody@0x08093eb0`的 `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`显示函数入口代码池、constitution和对象execRef安装。返回与catch继续沿用这些实际上下文，不能把function slot当作当前caller代码的槽来执行。

字符串目标先查当前对象函数，再查全局标识符；$前缀还可走OS函数转换/调用分支。名称/private、缺失函数、OS ABI及efun完整绑定不由本轮普通Object首批规格实现；不会因为mayNull=4就把任意不支持的目标转换为Void成功。

## 复制、释放、相等与真值

普通函数值复制共享同一头并递增ref。若保存owner已被删除，vm_copyValue先cleanup源值，复制结果成为Undefined；不仅是“调用时失败”。共享捕获复制可能触发这一规则，直接移动不会经过同样检查。

最后一个tag9所有者清理时，清理全部捕获项、释放owner引用、释放捕获区和函数头。直接vm_freeFunctionValue在唯一所有者调用路径中承担对应释放；此前已移出的捕获项被清零，避免重复释放。完整原生GC/循环回收和主动析构调度不在这些局部函数的证明范围内。

vm_isEq对tag9用指针身份比较，不比较函数槽、owner或捕获参数的结构。两个独立创建但字段相同的函数值不相等，复制同一值仍相等。vm_isZero的tag9走默认非零路径，原始失效owner函数值也为真；它与先经copyValue成为Undefined后的真值不同，未来实现必须区分原始真值查询与上下文复制。

`vm_f_bind_parameters@0x080df374`和`vm_f_bind_parameters_arr@0x080df310`实际调用vm_bindParameters处理SQL statement及Array，和17的捕获区创建不是同一机制。两条完整记录保留在证据包中，作为同名搜索边界的已排除线索，不据此设计函数值绑定API。

错误展开复用 `vm_traceError` 的 `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4`、`vm_doActions` 的 `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` 和 `vm_errorQuit` 的 `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1`。它们与tag9 cleanup支持函数值参与普通栈释放的推导，不证明所有线程、暂停和JIT路径。

## 全量统计与下一步

主语料7,159程序、101,592普通函数、6,474,773条指令中：

- 05共3,089处：Object普通函数2,828，Global普通函数169，Global init92；argc含目标，实测1..11，par16=0有517处、=4有2,572处。
- 17共32,614处：Object普通函数27,633，Object init4,871，Global普通函数110。非global/c共27,602；global/K共4,902；global/c共110。非global/K未出现，但仍需边界测试。
- 17捕获数量实测0..19；global/K中call_other数字槽356有4,863处，这是注册名映射，不是已执行跨对象回调证明。
- 完整位置35,703条，68个代表样例、470种作用域/选择器/槽组合。成员位置按nodes_offset+4*PC计算，不是ELF地址或PAK压缩位置。

见[摘要](stage2af-evidence/corpus-summary.json)、[统计方法](stage2af-evidence/corpus-method.md)和[目标数字清单](stage2af-evidence/target-inventory.json)。Object槽仅给出同映像声明候选，未建立完整继承重定位或动态调用图。

首批设计限定普通Object字节码目标及0..63个标量捕获，支持身份共享、存储/返回、跨Object/Global调用者执行和既有数值catch组合；完整接口、资源/失败策略及验收见实施规格。这一限制是Rust分批决策，不是原生仅能捕获标量。非标量捕获两条所有权分支、global/K目标、字符串/OS调用、完整对象删除及循环回收留待后续。对首批所需问题已给出实施决策；这些扩展未知不以猜测解除。
