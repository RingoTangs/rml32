# 2AS方法与验证边界

实施前2AR完整收据通过，冻结628个既有文件与两个入口文档的原身份。Rust源码/测试、Cargo、原ELF/PAK/对象/脚本及所有历史证据保持不变。本批只增加2AS设计/模型/工具及当前入口段落；没有新REA、原版运行、业务字节码或外部应用写入。

## 模型与独立检查

model.py以显式header、稳定Cell、VM/Reference根和host root表示所有权；模型操作在候选世界上执行，图/账本校验通过才发布。cached owners/refs由各转换更新，audit独立从实际Cell载荷/边/host根重算，并用Kahn消去法检查环；主模型使用DFS投影图，因此不是把同一校验器调用两遍。Slot别名和header共享不按路径数重复计数。

候选世界deepcopy天然提供模型原子性，不能据此证明将来的Rust预留/借用/guard释放正确；未来仍需故障注入、Drop/回调和编译生命周期测试。模型中的creator/Object删除只是逻辑标记，Reference为显式根别名；它不实现原生物理Object ref/execRef、Map延迟节点链、callee/catch、JIT或GC。整数/Real/Bytes载荷只检验位/原始数据保留，不实现其全部运算。

model.apply是设计事务，不是一个opcode。零预算、单步执行和真实Host副作用由未来Rust验收保留，case-map明确列为未建模。retire不注入资源失败，表达未来Drop只设预分配状态、不返回错误；Python进程OOM不在模型保证内。普通release仍可用于显式有失败的释放事务，不能当作Rust Drop。

## 有界枚举

explore.py从ownership、graph、source-copy三个种子出发，按固定grammar枚举最多3步后继。域限制：2个header、6个slot、4个VM根、2个host根，项数/扩展深度4、模型工作节点16；状态/转换资源上限分别20000/200000。报告精确给出成功、拒绝、域外转换、深度、去重状态摘要及cutoff。触顶必须标incomplete，不把退出上限当成穷举完成。

去重只对当前活ID做alpha重命名，保留别名、root种类、creator与借用/pending信息，忽略历史事件/已释放ID和模型计数器。它用于所有权/图不变量，不证明历史快照数值ID序列；ID不复用另有手写测试。grammar没有穷举所有Value载荷、方法、数据布局或错误域，outside_profile不代表原生拒绝。

case-map.json逐项关联2AR全部51个案例，分为modeled、partial-model、native-boundary、unresolved。每条保存原案例/源文件哈希、相关测试及限制；原2AR unresolved案例不得被模型改写为已解决。模型中的原子预检、origin、retired root、资源限额和图拒绝均为Rust设计政策，不当作原生观察。

## 复现与收据

```sh
python3 -B docs/stage2as-design/explore.py --write
python3 -B docs/stage2as-design/verify_design.py --write
python3 -B docs/stage2as-design/verify_validation.py --write
python3 -B docs/stage2as-design/verify_validation.py
```

验证包含目录/出处/状态检查、模型手写与工具负向测试、声明域枚举、Rust558项debug/release、fmt及严格Clippy；真实命令输出进入新日志和收据。--write不刷新历史哈希。新材料出现后不直接重跑依赖旧全仓清单的2AR整包校验，而由新收据保护它的原文件，并复用其原生证据检查。

PAK全量比较复用2AQ冻结结果（12包、9527成员、7159程序、9933继承关系、唯一DBA解压异常），不冒称本批重跑。自动工具只验证设计一致性与有限模型，不证明原生兼容、完整GC或未来Rust实现。
