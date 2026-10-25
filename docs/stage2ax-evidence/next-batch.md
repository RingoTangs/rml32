# 2AX 后续：T05-lifecycle-contract-evidence

选择的下一批是 **T05-lifecycle-contract-evidence**。2AX 已交付现有 8 条新增、57 条复用记录的证据总账及[门控规格](runtime-design.md)，不开放直接 Object／Function 捕获。下一批先补以下两个限定生命周期缺口，再冻结可实施的生命周期设计；普通线程局部结论不关闭 Q05／Q06 全域。

本页任务状态是后续工作的前置约束，不表示已经进行了新增取证、建模或 Rust 实现。本批不要求新增独立状态模型。

## E01：未发布零捕获头的恢复与最终释放

**状态：** unresolved；前置为 2AX 总账及冻结制品哈希核验。

**入口与现有结论：** 从 `vm_createFunctionValueEx` 的 `sp=0,argn=0` 末尾失败候选追踪 Function 头及捕获分配。零长度分配成功的局部可行性由 `pdb_allocateMemory` 完整记录 `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` 支持；不能继续把该缺口归因于未知 `malloc(0)` 行为。创建、全局 value 链和 marker 边界见 [生命周期复核](lifecycle-review.md)。

**所需证据：** 确认进入错误路径前头指针是否已存入可恢复位置；沿 `vm_errorQuit`、backend 恢复、PCB 最终清理及实际 allocator 追踪结构查明该分配是否可被回收。区分分配统计、全局槽链和真正恢复根；逐分配记录发布状态、所有者及 free 接点。

**退出条件：** 对限定合法输入说明失败可达条件，并以完整记录及机器码锚点证明恢复／释放路径；若证据只支持未闭合，继续标 unresolved，精确给出剩余指针或调用边。不得仅凭缺少局部 free 宣称全局泄漏，也不得借 Rust 预检政策关闭原生未知。

## E02：PCB 最终清理之后的对象队列安全点

**状态：** unresolved；可与 E01 独立取证。

**入口与现有结论：** `vm_scheduleProcessInternal` 先调用 `vm_endForBackend`，再保存 PCB 状态并执行 `vm_deleteProcess`；ref 未归零时由后续 `vm_unrefToPCB` 处理最终清理。完整记录分别为 `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`、`ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a`、`ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`、`ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`。

**所需证据：** 从两种 PCB 归零清理位置中 `vm_cleanupValue`／`vm_unExecRefObject` 新增的待删对象向后追踪，区分还有活动线程、最后线程退出、延后 unref 的发生位置及停止调度条件。以实际调用链证明下一次对象队列排空的触发条件；单纯列出 `vm_endForBackend`／clear／shutdown 的 callers 不够。

**退出条件：** 明确选定普通解释线程域的“入队→下一安全点→变量清理→队列 ref 释放→可能的壳释放”顺序及延迟条件。无法证明的终止分支保持 unresolved，不写“同轮已经清空”或无条件最终回收保证。

## D01：生命周期基础设计冻结

**状态：** gated；依赖 E01、E02 在拟支持退出域内给出可审查结论。任何未通过的退出域必须继续阻塞对应直接捕获能力。

**接口及数据流：** 以私有生命周期表示连接稳定 Cell／CopyPlan、Function creator、捕获 Object 载荷、调用帧和对象队列。分别列出每类普通 ref／execRef 的取得、move、COPY、drop、删除请求及最终释放事件；采用受控句柄与现有 origin 边界，不把原始指针或物理 Rc 计数当作语义计数。明确 Object 变量与 Function creator 的生命周期边、循环政策，以及 store 关闭／VM 提取／Host root 退休与清理的关系。

**验收：** 输出完整事件表、状态不变量、逐接点迁移清单及案例映射。覆盖 creator 等于捕获对象、失效与待删区分、唯一／共享 Function 头、PCB 额外 ref、最后帧退出、函数头及载荷最后释放；可报告失败保持源／目标／PC，Host 不重放。保留当前错误分类、catch 来源和公开观察契约。

**排除：** 任意 hook 行为、其他线程、JIT、回调／重入删除、完整 GC、捕获槽 Reference 逃逸。若拟支持路径实际上需要其中能力，不得以“排除”跳过依赖，必须补证据或缩小并显式拒绝该路径。

## F01／O01：直接捕获的后续实施批次

**状态：** 两项均为 blocked-native-contract，当前不实施。

F01 只处理直接 Function 捕获，O01 只处理直接 Object 捕获；两者分别依赖 D01 的适用生命周期契约，通过门槛后可独立排期。各批次须先固定显式准备入口与实际类型 profile 的新增接口，保留旧入口能力集合，再以真实 Rust 场景验收 [T05-F／T05-O 要求](runtime-design.md#4-t05-的独立门槛与完成定义)。不接受一次开放任意 Value 的白名单变更。

F01 的关键验收是内外 Function 头身份、各自 creator、唯一 move／共享 COPY 的持久源变化及最终图校验；O01 的关键验收是独立对象普通引用、活动 execRef、延迟删除、失效源规范化和壳释放。两项都须覆盖单步／零预算、共享 DAG、非法图、资源／借用失败、旧 caller 拒绝及 Host 前后验失败，保留原有全部场景与断言。

## 未选为下一批的扩展

release／crash／end hooks 的间接目标、异常和重入行为，其他线程/JIT 退出、任意外部 mark 根、循环回收及释放次序继续独立登记为 unknown。新记录的 value-flow 未截断不代表间接调用目标已穷尽。需要这些能力时，应另建目标明确的证据任务；本轮普通线程证据与后续合成 Rust 测试都不能替代它们。

原始 ELF、PAK 和业务 `.o` 保持只读；任何原程序运行对照另行按实际授权范围执行。下一批仍不应将限定静态闭合、Rust 设计完成或测试通过表述为完整原生行为等价。
