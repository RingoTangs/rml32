# 2AS所有权与复制设计定稿

本批是设计与独立状态模型，Rust运行时/API/依赖及非标量捕获白名单均不变。依据2AR冻结的F01—F23；所有新增接口、限制和事务顺序是**后续Rust设计政策**，不构成新的原生证据。模型允许表示直接Object/Function捕获，只为分离move/COPY及删除标记；这两类捕获仍禁止进入后续首批实现。

## 1. 持有者与身份

采用私有`VmValue`、稳定`ValueCell`、Function header和per-store root registry。一个活Cell内的Function载荷贡献一份头所有权；多个Reference别名只是增加该Cell的refs。共享Array/Mapping header并不复制其成员Cell，因此不按可达路径数重复计算其中的Function。参数、局部、对象/Global变量、捕获和容器成员都遵守此规则；隐藏Loop持有者与引用lease纳入接点清单，不通过普通值逃逸。

Function使用私有稳定捕获Cell及固定bound_count。共享COPY可改写实际捕获源；唯一move先搬走载荷并清空源，再释放头。调用计算槽先移到私有activation槽，净所有权不变；此时的语义头计数决定唯一/共享。VM内部临时读取只能获得非拥有借用/内部观察，不得隐式创建语义所有者。按头身份比较，调试只显示原有target/owner/bound_count，不显示捕获。

`ValueSnapshot`保存不可调用的冻结数据与身份ID，不持有Cell/header/root，不能转换成`HostValue`。Array/Mapping观察图用ID回指表达共享；Function只输出元数据，不遍历其捕获。ID在同一store内不复用，旧快照不能重新导入。快照读失败保留状态；Debug不调用retain。类型区分替代“约定调用者不要保存Value”的隐含规则。

`HostValue`是opaque root lease，不实现Clone；`ObjectStore::retain(&HostValue)`显式返回另一lease。私有字段包含origin、不可复用lease ID和root控制块；跨store导入/retain在任何提交前失败。Scalar输入可廉价复制。root registry随ObjectStore经into_objects迁移；store关闭后不再允许导入/retain，已发lease仍可析构其结构性载荷。模型不模拟Object物理ref/execRef，不能借此关闭2AR的生命周期未知。

## 2. Drop与Host调用

Root lease的`Drop`不可返回失败，也不可临时分配。每次retain/export预先分配lease状态；Drop仅设置retired标志。若在内部借用epoch，registry暂保留这份私有根，禁止新的VM/导入/retain操作；外层释放全部guard后统一排空，再允许下一个可执行入口。retired lease立即不能再次导入；纯元数据Debug不改变分支。由此避免在析构里借用冲突panic、漏放root或半提交。模型用`retire`/`unborrow`表达这个epoch，用普通`release`检查显式可失败的事务；两者不可混淆。

Host回调改用仅限本次调用的`BorrowedValue<'call>`，不输出可保存的VmValue或RefCell guard。跨回调保存必须通过单独的`HostScope::retain`能力生成HostValue；该scope不提供重入、注册表修改或一般VM执行权限。回调元数据仍为原HostCallContext。

`HostResult`限定Void、借用某个本次参数或Owned(HostValue)。借用转结果为opaque共享，不自动规范化；Owned在签名/图/资源校验后转移为结果槽，验证失败正常释放仅本次返回持有者。callback期间显式retain或外部副作用不回滚、不重放。进入回调前不得持有槽/registry可变guard；同步单次调用规则、Any/Mixed的opaque Function语义和精确类型限制保留，真实efun/异步不纳入。

## 3. COPY计划与发布

每个实际COPY接点产生私有`CopyPlan { outputs, source_updates, semantic_owner_effects }`；`source_updates`必须指向真实Cell，不能对临时clone改写后冒称已规范化源。同Cell同目标值合并，不同值冲突返回现有InvalidState；readonly目标在任何源改写前拒绝。建立计划期间禁止可重入执行，不在计划与提交间运行Host代码。

固定顺序：读取与origin/shape检查 → 预留结果Cell、栈/帧与计划/图工作空间 → 检查拟提交图及全部必要借用 → 保留所有新载荷持有者 → 批量更新源/目标/上下文 → 释放guard → 释放旧载荷/排空retired roots。提交段不得包含可报告失败点或回调。失败保持可见源、目标、操作数及PC；临时Cell ID可消耗，宿主不可恢复OOM不保证。这是Rust预检政策，已知不同于原生先消费目标、逐参转换和catch栈恢复。

普通局部/Object/Global读取、deref、成员读取及共享头展开使用COPY。17建立、唯一头展开及选定非const普通返回使用raw move。Array/Mapping COPY只共享header，不递归清理成员；死Object/有已删除creator的同origin Function只在实际COPY源处清理，owner=None不判死。

Host export/import/retain是原始载荷所有权运输，不是COPY接点。start先导入独立临时槽，再对该槽执行entry COPY；不能把外部HostValue root当成同一可改写源。HostValue可继续opaque持有raw dead handle，实际VM普通读取/调用仍按各自规则失败或规范化。变量/Global初始注册沿原raw输入契约，不擅自追加读取COPY。

## 4. 最终图与资源

图节点为Array/Mapping/Function header及其稳定槽，边为实际载荷持有关系；Reference根指向Cell而不复制其载荷。构造、成员写、Host结果、默认容器构造、函数捕获/展开、对象/Global重绑和Mapping三元输出都必须检查拟提交后最终图，不能把旧边和替代边一起当成永久图。灰色回边拒环；已完成黑节点共享允许。一般Array/Mapping成员重接继续拒绝，即使其图无环；不新增Reference/Loop捕获。

默认既有项数65536、深度64保留。扩展深度中纯标量Function叶贡献0；有header捕获的Function贡献一层，Array/Mapping贡献一层，避免旧Array深度因标量Function叶而增加。循环检测仍遍历所有Function边。生产`ValueLimits`新增每次检查最多262144个不同header/槽节点、最多65536个host root的显式Rust预算；默认由ObjectStore::new使用，可在store创建时覆盖，不允许运行中改变。旧诊断container_depth保留原Array/Mapping度量；扩展图检查是独立策略。

模型的max_nodes对整个有限世界计数，是比生产“受影响图工作量”更保守的枚举域，不把其Limit拒绝解释为原生限额或生产实现唯一结果。Array最终header只unref元素槽；Mapping先清节点载荷再撤销header边；捕获槽从不公开Ref。原生Map延迟节点链、creator的物理释放及一般GC不在模型内。

## 5. 实施顺序与状态

T01（下一批2AT）先迁移所有观察/Host输入输出边界和语义计数，保持标量捕获；T02再落地稳定捕获槽和COPY事务；T03集成Function边图检查；T04经真实Rust失败/释放/Host回归后，才考虑显式新入口的Array/Mapping捕获。T05直接Object/Function捕获、完整删除和原生异常域继续阻塞。详细任务与退出标准见implementation-plan.json及API迁移表。

本批定稿的是有限范围架构与实施任务，不能标为“非标量捕获已准备好直接打开”。独立模型没有执行字节码、callee原型、真实Host回调、时钟预算或原生GC。2AR未知案例和未覆盖分支逐项保留在case-map.json；有界枚举仅证明所声明grammar/种子/深度中的模型不变量。
