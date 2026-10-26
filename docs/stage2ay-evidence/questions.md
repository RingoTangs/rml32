# 2AY 问题与退出域

answered 仅限命名子域；普通线程的局部结论不关闭 hook、其他线程或完整 GC。

## Q01：17捕获建立、类型与局部失败顺序

partial · [F01](findings.md#f01), [F02](findings.md#f02), [F03](findings.md#f03), [F20](findings.md#f20), [F28](findings.md#f28), [F29](findings.md#f29), [F30](findings.md#f30), [F41](findings.md#f41), [F42](findings.md#f42), [F43](findings.md#f43), [F44](findings.md#f44), [F45](findings.md#f45), [F46](findings.md#f46), [F47](findings.md#f47)

剩余：已补合法entry状态、动作分派与256次Int入栈加零捕获17的有限静态路径；H/A未发布恢复根及可能的creator普通ref最终释放仍未闭合。主语料真实程序可达性与本分支推导分开。

下一步：保留发布前预检政策；下一批只追具体未发布H/A注册、恢复、语义清理及最终free关系，不把pool存储链当作载荷恢复根。

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

### Q01.e01-finite-path

answered · [F41](findings.md#f41), [F42](findings.md#f42), [F43](findings.md#f43)

边界：限定显式256槽、有效Global目标、普通processor/nohooks及分配成功；不是主语料或运行观察。

下一步：保留有限节点序列和完整机器码依据，不用direct callers空表判为不可达。

### Q01.e01-storage

answered · [F44](findings.md#f44), [F45](findings.md#f45), [F46](findings.md#f46)

边界：仅回答局部发布位置及显式块释放，不等于最终语义回收。

下一步：分别追H/A存储及可选creator普通ref，禁止合并物理存储与载荷清理。

### Q01.e01-final-release

unresolved · [F47](findings.md#f47)

边界：尚无完整恢复根、最终free和可选creator unref链；不宣称全局泄漏。

下一步：沿具体注册/恢复指针或清理调用边定向补证据。

## Q02：05唯一/共享head展开及持久源变化

answered · [F04](findings.md#f04), [F05](findings.md#f05), [F06](findings.md#f06), [F07](findings.md#f07), [F08](findings.md#f08), [F09](findings.md#f09), [F10](findings.md#f10)

剩余：结论限展开接点，后续参数/使用的行为由其各自规则决定，不推及整个调用成功。

下一步：后续实现先设计语义持有者模型；禁止使用物理Rc计数或统一clone替代两条分支。

## Q03：普通槽、引用、成员与返回接点

answered · [F07](findings.md#f07), [F08](findings.md#f08), [F12](findings.md#f12), [F13](findings.md#f13), [F16](findings.md#f16), [F22](findings.md#f22)

剩余：限已选普通读取与非const、无需转换/hook返回；完整attrib/default及其它返回分支仍排除。

下一步：建立源可改写的复制计划；明确raw-move返回不追加copy规范化。

## Q04：最后所有者、成员Ref与creator释放

partial · [F11](findings.md#f11), [F14](findings.md#f14), [F15](findings.md#f15), [F23](findings.md#f23), [F24](findings.md#f24), [F25](findings.md#f25), [F26](findings.md#f26), [F27](findings.md#f27), [F35](findings.md#f35), [F36](findings.md#f36), [F37](findings.md#f37), [F45](findings.md#f45), [F46](findings.md#f46), [F48](findings.md#f48), [F51](findings.md#f51), [F52](findings.md#f52), [F53](findings.md#f53), [F54](findings.md#f54)

剩余：普通backend完成、clearEnvironment和shutdown扫描后分别存在条件化drain；延后外部PCB ref进展、任意后继安全点与捕获槽Ref、完整生命周期仍未闭合。

下一步：保持creator/载荷普通ref、execRef、队列持有独立；后续逐有效存储域补外部持有退休和保证drain，不把shutdown存储释放当原载荷析构。

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

### Q04.e02-conditional-drains

answered · [F48](findings.md#f48), [F51](findings.md#f51), [F52](findings.md#f52)

边界：须实际抵达且正常完成相应backend/helpers/hooks；不证明外部持有者最终释放。

下一步：保留各接点前提及顺序，适用域单独重评。

### Q04.e02-retained-reference

unresolved · [F54](findings.md#f54)

边界：后继unref只在PCB存储仍有效的范围讨论；任意进展性和next drain未知。

下一步：定向审计具体外部持有/退休接点，不推广到shutdown后悬空指针。

## Q05：捕获展开后失败与解栈

partial · [F18](findings.md#f18), [F19](findings.md#f19), [F20](findings.md#f20), [F30](findings.md#f30), [F31](findings.md#f31), [F32](findings.md#f32), [F33](findings.md#f33), [F34](findings.md#f34), [F35](findings.md#f35), [F36](findings.md#f36), [F39](findings.md#f39), [F43](findings.md#f43), [F44](findings.md#f44), [F47](findings.md#f47), [F48](findings.md#f48), [F49](findings.md#f49), [F50](findings.md#f50), [F51](findings.md#f51), [F52](findings.md#f52), [F53](findings.md#f53), [F54](findings.md#f54)

剩余：普通无catch局部清理和新条件化drain已补齐；未发布头、任意停止/外部持有/间接callback与hook、其他线程/JIT及全退出域继续未闭合。

下一步：保持原catch来源不变；只对明确可达且有退出契约的子域评估，其他退出域不以普通路径代替。

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

### Q05.e02-visible-tails

answered · [F49](findings.md#f49), [F50](findings.md#f50)

边界：只描述选定结果之后的可见尾部；不把前置peek/scanner/helpers或间接callback证明为无副作用。

下一步：使用真实控制条件，避免把被scanner覆盖的变量误作scheduler返回。

### Q05.e02-progress

unresolved · [F54](findings.md#f54)

边界：stop信号不自动调用shutdown；active指针或caller列表不保证backend/helper完成。

下一步：下一批只扩展明确调用域，保持其他线程/JIT/hook/重入排除。

## Q06：图约束、循环与Rust实施边界

partial · [F17](findings.md#f17), [F20](findings.md#f20), [F21](findings.md#f21), [F23](findings.md#f23), [F37](findings.md#f37), [F38](findings.md#f38), [F39](findings.md#f39), [F40](findings.md#f40), [F45](findings.md#f45), [F46](findings.md#f46), [F47](findings.md#f47), [F53](findings.md#f53), [F54](findings.md#f54)

剩余：新增allocator页链与shutdown槽tag顺序只提供局部存储证据；未发布头语义恢复、任意外部根、Reference和循环GC仍未闭合。

下一步：保持无环图及槽不逃逸政策，直接捕获门槛不开放；先补具体根/退休与语义释放边。

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

### Q06.e01-allocator-root

partial · [F45](findings.md#f45), [F46](findings.md#f46), [F47](findings.md#f47)

边界：已见存储链但未建立Function/creator恢复及最终释放契约。

下一步：以原始指针关系追恢复根，不由allocated计数或批量存储回收推语义析构。

### Q06.e02-storage-pass

answered · [F53](findings.md#f53)

边界：只确认先清tag再destructStaticValue；不保证外部引用安全或原载荷释放。

下一步：后续生命周期设计必须分别记录载荷清理与物理slot存储清除。

### Q06.e02-external-lifetimes

unresolved · [F54](findings.md#f54)

边界：未穷尽任意mark根、间接callback、停止后退休和循环释放次序。

下一步：继续保持完整GC及任意外部图为独立unknown，不以本批静态收尾关闭。
