# 2AN 当前 Rust 接入审计

本页记录2AM源码现状及后续约束；不是实现交付。536个既有文件由protected-inputs固定，包含Rust、测试、制品及历史证据；README和路线图作为进度入口允许更新。

| 当前接点 | 扩展必须处理的约束 |
|---|---|
| `src/value.rs:81` FunctionValue／FunctionData | owner与program都是必填句柄，bound限标量；需分离可空创建owner与Object／Global／Host目标，不能制造假ObjectId或ProgramId。 |
| `src/vm.rs:713` copy_value | 当前无条件用f.owner()检查同origin函数值；应仅在Some(owner)时规范化。外来store值仍由05拒绝，不能误查本store同编号对象。 |
| `src/vm.rs:1491` create_function_value | 限Object及flags<64，目标校验使用当前程序。全局17必须改查当前Global绑定，不能拿Object调用者的max_functions检查绝对全局槽。 |
| `src/vm.rs:1565` call_function_value | 目前按Object目标安装帧；需分派实际目标，独立检查creator owner，Global保持None对象，Host继承05调用者上下文。 |
| `src/prepared.rs:704` 17后检 | 当前全部按本映像函数目录检查；应仅保留局部c的本地检查，全局c延迟至运行时绑定解析，全局K先检查ABI范围。 |
| `src/object.rs:160` Global绑定 | 显式clear再bind；程序注册表保留旧代码。规格要求新Global/c调用检查原绑定，活动帧仍保存实际程序。 |
| `src/object.rs:57` 宿主注册 | 注册拒绝已有槽，remove后可重装；HostFunction的Clone冻结一次activation。K应在每次05预检时取当前注册，创建阶段只验证存在并保存槽号。 |
| `src/vm.rs:1697` call_host | 当前绑定06能力，按当前栈取argc并发布结果。不可先把05捕获改写入栈再调用它；需复用预检／调用／提交逻辑，让05目标和参数到最后才消费。 |

目标表示改变同时影响Debug、普通复制、真值／相等、Host Any/Mixed透传与容器叶值检查。首批bound继续限标量，因此Function仍可作为叶值；不得顺带开放容器捕获或改变图深度规则。

旧准备入口能力隔离、生命周期守卫、引用lease前缀、指令预算、帧ID提交、数值catch的精确来源均保留。新入口不准备init、不扩默认表达式；宿主副作用不回滚，终止错误不得在run重试时重放回调。具体取舍均在function-runtime-design.md固定。
