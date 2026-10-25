# 2AX 问题与退出域

answered 仅限命名子域；普通线程的局部结论不关闭 hook、其他线程或完整 GC。

## Q01：17捕获建立、类型与局部失败顺序

partial · [F01](findings.md#f01), [F02](findings.md#f02), [F03](findings.md#f03), [F20](findings.md#f20), [F28](findings.md#f28), [F29](findings.md#f29), [F30](findings.md#f30)

剩余：历史创建与局部失败结论保留；合法非空捕获不能触发末尾sp==0。零捕获/初始sp=0的未发布头恢复根及最终释放仍未闭合；畸形tag/负捕获数不合法化。

下一步：保留Rust发布前预检；下一批仅追未发布头分配/注册/回收链，不以分配统计代替根。

### Q01.reachability

answered · [F28](findings.md#f28)

边界：限合法栈、非负计数且无中途状态修改，不是动态可达证明。

下一步：保留历史C15并用C59/C61明确拆分可达域。

### Q01.zero-allocation

answered · [F29](findings.md#f29)

边界：初始化和资源可用是前提；不保证每次分配成功。

下一步：采用条件成功结论，避免继续借malloc(0)作为未知理由。

### Q01.unpublished

unresolved · [F30](findings.md#f30)

边界：分配计数、PCB栈清理与已知值链不能证明该C局部头可回收。

下一步：定向追头注册、长跳转恢复和最终释放；证据前T05相关实现保持阻塞。

## Q02：05唯一/共享head展开及持久源变化

answered · [F04](findings.md#f04), [F05](findings.md#f05), [F06](findings.md#f06), [F07](findings.md#f07), [F08](findings.md#f08), [F09](findings.md#f09), [F10](findings.md#f10)

剩余：结论限展开接点，后续参数/使用的行为由其各自规则决定，不推及整个调用成功。

下一步：后续实现先设计语义持有者模型；禁止使用物理Rc计数或统一clone替代两条分支。

## Q03：普通槽、引用、成员与返回接点

answered · [F07](findings.md#f07), [F08](findings.md#f08), [F12](findings.md#f12), [F13](findings.md#f13), [F16](findings.md#f16), [F22](findings.md#f22)

剩余：限已选普通读取与非const、无需转换/hook返回；完整attrib/default及其它返回分支仍排除。

下一步：建立源可改写的复制计划；明确raw-move返回不追加copy规范化。

## Q04：最后所有者、成员Ref与creator释放

partial · [F11](findings.md#f11), [F14](findings.md#f14), [F15](findings.md#f15), [F23](findings.md#f23), [F24](findings.md#f24), [F25](findings.md#f25), [F26](findings.md#f26), [F27](findings.md#f27), [F35](findings.md#f35), [F36](findings.md#f36), [F37](findings.md#f37)

剩余：普通ref/creator/execRef及局部延迟删除链已定向补齐；最终PCB释放后新增待删对象的下一保证安全点、任意外部捕获槽Ref及完整生命周期仍未闭合。

下一步：保持三类持有与队列引用独立；明确捕获槽不公开，再补下一保证drain与外部持有退出域。

### Q04.holders

answered · [F24](findings.md#f24), [F25](findings.md#f25), [F27](findings.md#f27)

边界：限所列创建、COPY、最终释放接点，不推断物理Rc语义。

下一步：冻结三类持有和0x4000/0x8000区分为T05设计前提。

### Q04.deferred-local

answered · [F26](findings.md#f26)

边界：不保证每条退出域都到达drain。

下一步：将请求删除、最后execRef、入队、队列ref和最终壳释放分开建约束。

### Q04.later-drain

unresolved · [F36](findings.md#f36)

边界：backend drain先于deleteProcess，不得宣称本轮已二次排空。

下一步：追后续安全点及scheduler停止分支，证据未闭合前保持门控。

### Q04.capture-ref

unresolved · [F37](findings.md#f37)

边界：最后Function释放不会等待该槽ref归零。

下一步：T05继续禁止捕获槽Reference逃逸；任意原生外部引用安全另取证。

## Q05：捕获展开后失败与解栈

partial · [F18](findings.md#f18), [F19](findings.md#f19), [F20](findings.md#f20), [F30](findings.md#f30), [F31](findings.md#f31), [F32](findings.md#f32), [F33](findings.md#f33), [F34](findings.md#f34), [F35](findings.md#f35), [F36](findings.md#f36), [F39](findings.md#f39)

剩余：普通解释线程无catch、有效backend及hook不干预路径已有限补齐；最终清理取决于PCB最后ref。未发布头、其他线程/JIT/callback、任意hook/重入及删除后下一drain仍未闭合。

下一步：逐退出域保持独立门槛；不扩大Rust catch来源，后续先补未发布头和删除后保证安全点。

### Q05.ordinary-uncaught

answered · [F31](findings.md#f31), [F32](findings.md#f32)

边界：须有有效backend环境、threadType1、无hook干预；不等于最终物理释放。

下一步：保留带条件调用链和原始锚点，不外推其他线程/JIT。

### Q05.final-pcb

answered · [F34](findings.md#f34), [F35](findings.md#f35)

边界：hook须正常返回；外部持有者最终释放本身不是已证明进展性。

下一步：冻结deletionFunc→栈→剩余帧execRef顺序及ref>0延后分支。

### Q05.hooks

partial · [F33](findings.md#f33), [F39](findings.md#f39)

边界：已知先于PCB减ref，未知间接目标、保留、异常与重入。

下一步：把hook影响退出域列为后续单独取证门槛。

### Q05.unpublished

unresolved · [F30](findings.md#f30), [F36](findings.md#f36)

边界：已知PCB清理不能自行证明未发布头回收及下一drain。

下一步：分别追未发布头根和scheduler后继安全点，避免合并为已有catch结论。

## Q06：图约束、循环与Rust实施边界

partial · [F17](findings.md#f17), [F20](findings.md#f20), [F21](findings.md#f21), [F23](findings.md#f23), [F37](findings.md#f37), [F38](findings.md#f38), [F39](findings.md#f39), [F40](findings.md#f40)

剩余：已知Function捕获递归和根枚举维持局部证据；任意Reference、间接外部mark根、Object变量/creator生命周期边、循环释放顺序及完整GC仍未闭合。

下一步：保留既有受限无环图和捕获槽不逃逸政策；T05只交付证据门控规格，直接Object/Function仍关闭。

### Q06.known-roots

answered · [F38](findings.md#f38)

边界：已见根不是完整根集合，分配链不是活根。

下一步：使用限定枚举与原始证据，不宣称完整GC等价。

### Q06.complete-graph

unresolved · [F37](findings.md#f37), [F38](findings.md#f38), [F39](findings.md#f39)

边界：间接mark目标和循环回收顺序未穷尽。

下一步：维持循环/捕获槽Ref逃逸排除，独立补证后才重新评估门槛。

### Q06.rust-gate

answered · [F40](findings.md#f40)

边界：本子问题只确认设计范围，不确认原生未知已解决或运行时已实现。

下一步：交付T05门槛、接口约束和验收规格，能力标志继续false。
