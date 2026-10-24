# 2AS后续实施任务

当前非标量捕获仍关闭；T01为下一批2AT。

## T01 · 2AT-host-boundary

specified · 前置：无

一次迁移全部公共Value出入口为Snapshot/HostValue/ValueInput/BorrowedValue，建立per-store root registry与实际槽语义计数；仍只接受标量捕获。

接口：api-migration.json全部16项及api-contract.md；私有VmValue/ValueCell，计数由copy/move/drop显式更新；RootRegistry、不可隐式Clone的HostValue及Drop退休epoch

验收：Rust编译负例拒绝Snapshot回传、HostValue Clone、BorrowedValue逃逸及scope重入；宿主retain/Drop/关闭/into_objects/foreign origin和反复完成观察有明确行为；同槽Reference别名不增加Function头owner；Array header别名不复制成员owner；原558项场景的行为断言迁移后继续通过，debug/release及fmt/clippy通过

排除：非标量捕获；全局GC/物理对象ref与execRef；新catch来源、真实efun或异步回调

## T02 · stable-captures-copy-plan

specified · 前置：T01

私有稳定捕获槽、真实源Cell复制计划、共享COPY源规范化、raw move与一次提交；捕获白名单仍保持标量。

接口：CopyPlan outputs/source_updates/semantic_owner_effects；固定bound_count与头身份；无公开可变捕获或Reference逃逸；源Cell同值合并/冲突拒绝及预留-借用-提交-释放顺序

验收：2AR各COPY/raw接点有源码及测试对应，不统一规范化返回；全部可报告分配/借用失败保留源/目标/操作数/PC；释放旧值时guard已释放；native失败顺序差异单列，不改原RuntimeFaultSite或catch范围

排除：直接Object/Function捕获；非const之外的返回/默认表达式/任务扩展

## T03 · projected-function-graph

specified · 前置：T02

所有构造、写入、Host结果、重绑与Mapping三元输出统一检查拟提交图，遍历Function捕获边，保留一般容器成员重接拒绝。

接口：带active/finished状态的图DFS与不同节点工作量计数；ValueLimits和旧container_depth诊断分离；任何可见写入之前检查完整投影图，失败不改游标/输出别名

验收：Function-Array/Mapping-Function环拒绝且共享DAG接受；旧边移除后的合法替换通过，不误判为旧图加新边；项数/深度/图工作量、borrow冲突及Host返回非法图失败原子

排除：循环GC；新的Mapping键/attrib、公开删除或Array/Mapping成员重接

## T04 · explicit-container-captures

gated · 前置：T03

仅在T01—T03真实Rust验收通过后新增显式入口，开放普通Object/Global/Host函数值的受限无环Array/Mapping捕获。

接口：prepare_program_with_container_captures(image, GlobalBankSchema)和Global twin；能力组合沿现有callable入口，但旧入口白名单不变；Function保存按实际捕获类型判定的capture_profile；旧caller拒绝间接执行container profile

验收：51个2AR案例中适用项有真实Rust对应；native-only/unknown明确保留；单步/零预算、Host签名/单次副作用、共享容器成员读取/最终释放与原故障注入回归；没有借公开Value/HostResult/成员写绕过新图验证

排除：直接Object/Function捕获；Reference/Loop捕获；init/construct/default/linked/hidden扩展

## T05 · direct-owner-captures

blocked-native-contract · 前置：T04

直接Object/Function捕获、原生延迟删除、无catch最终清理及更广生命周期组合另批取证。

接口：必须先闭合2AR Q01/Q04/Q05/Q06对应退出域，不以模型补原生未知

验收：creator/被捕获Object/execRef及最终释放契约有完整证据；唯一move与共享COPY持久源变化、无catch/其他线程/未发布头清理分别验收

排除：未闭合前禁止直接开放Value所有类型
