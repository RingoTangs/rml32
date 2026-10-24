# 2AU：稳定捕获槽与 COPY 事务

T02 已实现；下一批为 T03 投影图验证。公共 API、版本和依赖保持 2AT 契约，非标量捕获仍关闭，阶段 2 未完成。未执行参考程序、未新增原生取证。本批依据冻结的 2AR F03—F16/F22 与 2AS D01—D06/D11/D15 实施 Rust 政策，不宣称原生运行等价。

## 实现

Function 捕获使用私有稳定 Cell 和固定 `bound_count`。17 在全部预检后把真实求值槽的载荷及 Owner 移入捕获槽并清空源；函数头身份和捕获顺序不变，捕获槽不能经公共 Reference 逃逸。最后物理头释放会清理捕获载荷，即使内部计划暂时保留捕获 Cell 的 Rc。

`CopyPlan { outputs, source_updates, semantic_owner_effects }` 保存实际源 Cell 更新和载荷转移边。读取视图不计数；输出预留后才发布。同 Cell 同值写合并，不同写拒绝 `InvalidState`；转移边不能重复消费同一槽。提交前预留工作空间及新 Owner、获得全部所需 guard；guard 持有期间只交换已准备的载荷，释放 guard 后销毁旧值。原始转移携带现有 Owner，不通过先 retain 再 drop 实现，Owner 计数耗尽不阻止净零所有权转移。

Object/Global/局部读取、解引用、成员/范围读取、参数绑定、赋值及循环输出的实际 COPY 会清理直接失效 Object 或同 origin、creator 已删除的 Function 源 Cell。Array/Mapping 复制只共享头，不递归清理成员；owner=None 的 Function 不按失效处理。公共边界继续拒绝跨 store；内部旧回归使用的 foreign opaque 值不被误当本 store 死对象清理，调用时仍执行隔离检查。

05 根据调用目标移入 activation 后净值不变的语义 Owner 数选择唯一/共享分支，不使用物理 Rc 数。唯一分支 raw move 并清空捕获，共享分支从稳定捕获源 COPY。内部临时视图、Debug 和冻结快照不改变分支。普通非 const mixed 或类型已匹配返回 raw move 到调用者/完成 Cell；`take_returned` 把同一 Owner 转到宿主 lease。其他返回转换保持既有有限政策，const、一般默认表达式和生命周期能力不扩大。

全部 20 处原生产 `copy_value` 调用及 4 个补充 raw/Host 接点列于 [接点表](copy-contacts.json)。单槽类型/真值/分支运算使用只读规范化预览并最终替换同一个计算槽；Function 真值/相等行为保留。Mapping 键仍限 Int/Bytes，复制不可变键不需要失效清理；缺失键查询不插入。

## 故障政策与已批准差异

[行为变更表](behavior-changes.json) 逐项登记 F08 源清理、F16 raw 返回、D05 冲突拒绝以及 COPY/raw 指令的失败 PC 保存。原 577 项场景逐项映射保留，只有成员清理与普通 raw 返回两处断言变化；成员测试同步改名。原回归文件反向应用这两项变化后必须逐字节等于冻结 2AT 版本。

用户确认保留 Host 失败原子性：05 回调前只读预览参数并冻结分支；回调和返回校验成功后才建立最终 CopyPlan 并提交。回调期间不持 Cell/registry guard。返回错误、资源或借用失败不消费调用目标、不改捕获源和 PC；回调已经保存的 root、外部副作用与显式释放不回滚，失败后再次 run 不重放回调。结果未知的检查和最终事务资源可能在回调后失败，专门注入测试覆盖这一顺序。

这与原生先消费目标、展开/释放头、再分派及逐参转换的失败顺序不同。Rust 成功前预检仍是明确政策，临时 ID 可消耗，Rust 不可恢复 OOM 不保证回滚。沿用原 RuntimeFaultSite 与 catch 分类；普通 return 在验证前退休本帧 catch 的既有规则保留。本批没有扩展 catch 来源、回调重入或原生 ref/execRef 生命周期。

## 验证与范围

新增测试覆盖真实源槽别名、同槽冲突、只读拒绝、唯一/共享 Object/Global/Host、每个激活/捕获 Cell 及事务工作空间预留失败、借用/帧 ID/Owner 耗尽、raw 返回、一次领取、最后 Function/Array/Mapping 释放和循环游标不前进。公共集成测试验证外部 HostValue 始终保持 opaque raw，源清理仅发生在 VM 实际 COPY 槽。

部分私有测试直接构造 Object/Function/容器捕获，只检查 T02 内部事务和浅共享机制，不经过也不放宽准备器/17 白名单。这些机械测试不构成 T03/T04 验收，不关闭 2AR 原生生命周期未知项。公共测试仍拒绝 Object、Function、Array、Mapping 捕获，并接受原标量集合。

新收据保存在 `docs/stage2au-validation.json`。运行 `python3 -B docs/stage2au-evidence/verify_validation.py --write` 重跑 debug/release、fmt、严格 clippy、56 项独立模型测试、新收据校验器的负例、原生静态证据校验及全量 PAK 结构对比。去掉 `--write` 只独立检查文件与日志收据。旧 2AT/2AS/2AR 文档、日志和收据冻结，旧源码定位校验器不用于迁移后的源码，也不更新其历史结果。
