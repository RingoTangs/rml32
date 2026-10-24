# 2AV：拟提交图验证

T03 已实现。以冻结的 2AU 为基线，公共 API、包版本、依赖和捕获白名单保持不变。下一批是 T04 显式受限容器捕获入口；本批仍为 `non_scalar_ready=false`，阶段 2 未完成。没有执行参考程序，没有新增原生取证。

## 校验语义

私有图内核使用显式 postorder DFS 栈，active 回边拒环，finished 节点允许共享并复用高度。Header、实际 Cell、Mapping 的稳定键槽采用分型身份；每次检查只对不同节点计费。遍历 Array/Mapping 成员及 Function 的稳定捕获槽，不沿 ObjectId 遍历对象变量或模拟 ref/execRef。Reference 仍指向既有 Cell，不增加该载荷的语义所有者；Loop 只纳入已有集合持有边与已验证绑定的 Cell 根，不能成为容器成员或捕获。

`Projection` 描述全部最终写入、raw move 的源清空、待插入 Mapping 成员、临时 activation 的退休和待发布输出槽。验证最终边，不把已删除旧边与新边叠加。纯标量 Function 深度为 0，有 header 捕获时贡献一层；Array/Mapping 各贡献一层。旧容器深度诊断及 65,536 项/64 层硬限制独立保留。

Registry 和 ObjectStore/VM 共享同一不可变 `Rc<ValueLimits>`。默认图工作量 262,144 个不同 header/槽节点；配置的项数和扩展深度在 VM 内部写入也生效。工作量包括本次投影涉及的源、输出和祖先槽，不是仅对最终输出大小计数，也不等于 2AS 模型的全世界节点数。Root lease 数量继续由独立宿主限额约束。

Weak 反向索引连接成员槽、所属 header 和持有 header 的 Cell，从改写槽收集存活祖先，避免只看新值漏检深度增长，也不扫描整个 store。索引不保活载荷、不增加 Owner；槽头标记随实际交换更新，用于过滤失败预检或重绑留下的陈旧 Weak 缓存。缓存预留发生在发布前，失败至多留下不可观察的 Weak 元数据；安全点周期清理失效索引，Drop 不运行图校验或分配。

## 发布与失败

发布接点及可复算的原始 Cell/Mapping 修改调用清单见 [发布矩阵](publication-contacts.json)。普通写入、CopyPlan 转移、输出根、Loop 控制/绑定/成员输出、对象与 Global 安装、Mapping 插入及三元数组复用均在提交前验证。临时私有 Cell 构造只登记索引，不表示执行发布；消费者必须通过已列出的提交门。最后释放与纯移除边不要求可失败校验。

权限、类型、既有成员重接和 Mapping source/output 限制保留；无环不是放宽这些规则的理由。Mapping 别名检查现在遍历 Function 边。新增图失败使用现有 UnsupportedType/Limit/InvalidState，不新增错误枚举、RuntimeFaultSite 或 catch 来源。数字运算结果先校验/提交再消费右操作数，避免资源错误留下半消费栈。

Host 保留已确认的失败政策：回调前预检最小输出槽，参数仍只读预览并冻结唯一/共享分支；未知结果和最终投影在回调后验证。错误不发布计划、不消费操作数、不前进 PC；已保存 root、外部副作用及显式释放不回滚，回调不会重放。普通 return 验证前退休本帧 catch 的既有行为保持。临时 ID 可消耗；不可恢复 Rust OOM 不承诺回滚。

快照与结构校验分离。Function 快照仍只输出元数据，不读取捕获，不 retain；旧观察预算和序列化规则保持。COPY 才按原规则规范化真实源，图遍历不规范化 raw 失效值，不改变唯一/共享分支或源 origin 隔离策略。

## 验证与后续

原 606 项普通测试与 2 项 doctest 全部保留，不更改既有断言。两处私有 fixture 改用测试专用原始注入来建立非法前态，继续测试被测操作的拒绝与原子性；该通路不进入生产编译。新测试通过投影构造环，不发布真实 Rc 环。

新增覆盖 Function/Array/Mapping 环、共享 DAG 与较深共享路径、旧边移除、存活祖先、陈旧索引、精确节点预算、长链非递归检查、工作空间失败、Weak 最后释放、隐藏 Loop、纯函数快照、store 转移后限额、Mapping 不插入、三元输出祖先失败、Host 单次副作用以及图失败不成为数值 catch fault。

部分私有结构测试使用未来捕获形状，只验证图内核，不授予执行能力。公共测试继续拒绝 Object、Function、Array、Mapping 捕获；T04 入口、一般容器成员重接、循环 GC 和原生生命周期未知项未开放。

完整收据为 `docs/stage2av-validation.json`。重跑：`python3 -B docs/stage2av-evidence/verify_validation.py --write`；去掉 `--write` 独立检查哈希、原断言、接点和日志。debug/release、fmt、严格 clippy、56 项冻结模型测试、新收据负例、原生静态证据与全 PAK 结构对比使用新日志，不覆盖旧收据。所有结果只支持受控 Rust/静态验证，不证明参考程序运行等价。
