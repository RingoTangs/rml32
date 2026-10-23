# 2AR问题状态

answered仅针对明确限定的接点；partial不代表完整兼容。

## Q01：17捕获建立、类型与局部失败顺序

partial · [F01](findings.md#f01), [F02](findings.md#f02), [F03](findings.md#f03), [F20](findings.md#f20)

剩余：最终结果栈溢出时未发布C局部函数头的恢复根/最终释放未闭合；任意畸形tag未证明合法。

下一步：先保留Rust成功发布前预检政策；如需原生失败等价，沿vm_errorQuit后的线程销毁和未发布分配跟踪补证据。

## Q02：05唯一/共享head展开及持久源变化

answered · [F04](findings.md#f04), [F05](findings.md#f05), [F06](findings.md#f06), [F07](findings.md#f07), [F08](findings.md#f08), [F09](findings.md#f09), [F10](findings.md#f10)

剩余：结论限展开接点，后续参数/使用的行为由其各自规则决定，不推及整个调用成功。

下一步：后续实现先设计语义持有者模型；禁止使用物理Rc计数或统一clone替代两条分支。

## Q03：普通槽、引用、成员与返回接点

answered · [F07](findings.md#f07), [F08](findings.md#f08), [F12](findings.md#f12), [F13](findings.md#f13), [F16](findings.md#f16), [F22](findings.md#f22)

剩余：限已选普通读取与非const、无需转换/hook返回；完整attrib/default及其它返回分支仍排除。

下一步：建立源可改写的复制计划；明确raw-move返回不追加copy规范化。

## Q04：最后所有者、成员Ref与creator释放

partial · [F11](findings.md#f11), [F14](findings.md#f14), [F15](findings.md#f15), [F23](findings.md#f23)

剩余：原生ref/execRef延迟删除、全部节点回收时机及捕获槽外部引用未闭合。

下一步：保留Array/Mapping不同最后header策略；直接Object/Function捕获暂缓，补齐生命周期契约。

## Q05：捕获展开后失败与解栈

partial · [F18](findings.md#f18), [F19](findings.md#f19), [F20](findings.md#f20)

剩余：普通线程有效script catch已有限闭合；uncaught最终PCB销毁、其他线程、JIT/callback及未发布head仍未知。

下一步：逐退出域单独取证；不得因已有脚本catch清理就扩展当前Rust可捕获错误来源。

## Q06：图约束、循环与Rust实施边界

partial · [F17](findings.md#f17), [F20](findings.md#f20), [F21](findings.md#f21), [F23](findings.md#f23)

剩余：任意Reference图、外部mark根、循环释放顺序及语义所有权表示/API影响尚未闭合。

下一步：先完成源复制/持有者设计，随后仅考虑受限无环Array/Mapping；循环和直接Object/Function捕获继续排除。
