# 阶段2AS：所有权与源复制设计交付

2AS完成15项设计决策、16个现有API边界迁移契约及独立状态模型。**Rust运行时、公共API、依赖和非标量捕获白名单均未修改**，阶段2仍未完成。未新增原生取证或运行参考/业务字节码；API迁移是后续实施契约。

## 已锁定的设计

- 一个实际槽内的Function载荷贡献一份语义持有者；Reference别名只增槽refs，容器header共享不重复计算成员载荷。唯一/共享不再依赖宿主Rc计数。
- ValueSnapshot与HostValue分型：快照不可执行且不增根；可回传VM的宿主值必须有显式lease。retain显式复制，Drop在借用epoch只退休，结束后排空；内部Cell guard不能跨Host回调。
- COPY计划指向真实源Cell，成功发布前完成资源、借用和拟提交图检查；raw move接点不误加源规范化。保留Array/Mapping不同最后header语义。
- Function捕获边进入图验证，拒绝回边但接受共享DAG；一般容器成员重接和完整GC仍不开放。

完整契约见[设计定稿](design.md)、[15项决策](decisions.md)、[API迁移表](api-migration.md)和[API详细契约](api-contract.md)。私有类型、origin/lease、完成结果一次领取、Host borrowed/retained和关闭/迁移行为均有明确后续要求。

## 模型结果及限制

model.py维护转换计数，独立audit从槽/边/root重算，并用Kahn算法核对主DFS拒环。手写测试涵盖观察不变、显式宿主持有、唯一/共享展开、实际源清理、失败原子性、别名、最后释放和退休root。

三个有界枚举域合计**21,837个域内去重状态、61,365次成功转换**，均到达声明的3步深度，未触及状态/转换资源上限；它们不是跨域全局去重数。完整边界/种子/grammar和摘要见[枚举记录](enumeration.json)。live-ID alpha商空间不证明历史快照ID序列；ID不复用有单独手写测试。模型候选世界天然提供的原子性不能替代未来Rust实现的预留/借用/Drop证明。

[51项案例映射](case-map.md)分为20项modeled、18项partial-model、9项native-boundary、4项unresolved。modeled仅表示指定模型不变量；物理Object ref/execRef、原生callee/catch/GC/节点链、真实预算与Host执行不由此解决。2AR全部原生未知保持未知。

## 下一批及验收

唯一下一批是**2AT-host-boundary（T01）**：迁移公共可执行Value边界、建立root registry及语义计数，继续只允许标量捕获。随后T02稳定捕获/COPY计划、T03投影图，T04才在真实Rust验收后考虑显式入口的Array/Mapping捕获；T05直接Object/Function及完整生命周期仍阻塞。见[实施任务](implementation-plan.md)。

实际检查、日志与交付哈希见[2AS验证收据](../stage2as-validation.json)，复现见[方法](method.md)。628个既有保护文件及原制品身份保持不变；PAK比较明确复用2AQ结果，没有冒称重跑。
