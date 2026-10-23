# 下一批 2AR：复制源规范化与非标量捕获所有权取证

这是2AQ选定的P05（G06/G07/G12），仅交付证据与后续实施规格。2AR不修改Rust运行时/API/依赖，不开放非标量捕获，不执行参考ELF、启动脚本或业务字节码。当前标量Object/Global/Host函数值及旧入口边界保持。

## 固定问题与取证入口

先验证magic_Linux32身份，再盘点冻结记录；仅对缺失分支补取证据。使用REA时遵循适用技能，保留完整记录、来源、机器码锚点和限制。工具不可用或证据不足时交付明确阻塞，不用Rust行为填补原生未知。

| 问题 | 冻结入口 | 必答契约 |
|---|---|---|
| 05唯一头与共享头 | 2AF/2AN的vm_callInVarFun及其捕获转移分支 | head唯一性在何时判断、捕获搬移/复制顺序、计数变化、调用失败后谁保有捕获 |
| 17捕获建立 | 2AF的vm_newFunction及实际调用助手 | Array/Mapping/Object/Function分别如何复制、何时清理源、目标owner与捕获owner如何独立处理 |
| 普通复制与源清理 | vm_copyValue及vm_pushValue；2D引用记录和2AN owner-null分支 | 死Object/死creator Function经直接槽、引用、成员、返回、捕获的规范化；原始真值/相等与复制是否不同 |
| 最后所有者与槽释放 | vm_unrefValue、vm_unrefArray、Mapping节点释放及Function释放入口 | 函数头/容器/成员Ref的最后释放顺序；引用仍在时源槽内容；失败及解栈释放 |
| 图与可实施边界 | 上述分支实际调用的copy/cleanup/mark helper | 安全无环共享与循环形成的区别；是否存在必须先恢复GC/attrib/延迟删除才能开放的类型 |

函数名仅作导航；实际地址、调用目标和逐分支结论从冻结记录或本次工具返回取得，不预造helper地址。复用：`stage2af-evidence`、`stage2an-evidence`、`vm-reference-evidence`、`vm-container-evidence`及2AL/2AM关于Mapping输出和最后header的记录。静态记录与Rust适配说明须分栏，不能用后者证明原生行为。

## 类型、接点与案例

逐项覆盖Undefined/Void/Int/Real/Bytes基线及Array、Mapping、Object、Function候选；Reference/隐藏Loop另列形态边界。每种候选区分唯一/共享函数头、独立/共享容器、活/死Object、Function有/无creator和foreign origin。origin是Rust隔离策略，单列其与原生证据的关系。

建立“来源接点→复制/移动→源变化→目标结果→释放时刻→失败后状态”账本。优先对比普通值读取、显式解引用、成员读取、返回与05捕获展开；后续实现不能只改17类型白名单。

不要求本批实现GC、Buffer、范围引用、完整attrib、继承/hidden、主动删除、Global热更新、真实efun或异步重入；若某候选依赖它们，记录依赖并缩小未来实施范围。

## 交付与退出条件

- 观察、推导、Rust政策、未知分开记录；每项结论引用完整Evidence和具体分支，列明复用/新增来源及制品身份。
- 复用2AF/2AN指令位置作导航，必要时仅增加可复算的类型来源分类；静态无法确定的捕获类型保留unknown，计数不作为执行频率。
- 交付证据推导的期望表：唯一/共享捕获展开、源规范化、别名可见性、最后header释放、死owner、失败清理、零/单步预算及现有数值catch解栈。
- 形成后续Rust规格，锁定允许类型、图约束、表示/接口最小变更、复制接点、预检/提交、资源与失败顺序、能力入口和回归场景。若关键语义仍未知，明确列出阻塞及下一次补取动作，不宣称可直接实现。

G12对G10的依赖表示完整继承目标的退出前置，不阻止本批取证普通函数的捕获路径；本批不解决整个G12。原版运行对照另行规划。
