# 2AT：宿主边界与语义所有者

T01 已实现。公共 API 源码不兼容，包版本和依赖保持原值。内部 `Value`、容器头、函数头和旧 VM 接口不再公开；16 项迁移见 [映射](api-migration.json)。下一批为 2AS 的 T02：稳定捕获槽与源 COPY 计划。非标量捕获、T03 投影图和 T04 新准备入口尚未开放，阶段 2 未完成。未执行参考程序，也未新增原生分析。

## 使用公共接口

`ObjectStore::new()` 与 `with_limits(ValueLimits)` 返回 `Result`，使 store origin 的 ID 耗尽也成为可报告错误。创建、全局安装和 `Vm::start` 使用 `&[ValueInput]`；标量按值输入，可执行值通过 `ValueInput::Handle(&root)` 借用输入。

`store.value/array/mapping` 创建 `HostValue`。句柄没有 `Clone`，`root.retain()` 或 `store.retain(&root)` 显式新增一个 lease；Drop 退休该 lease。`root.snapshot()`、对象 `variables()`、`global_values()`、`Vm::stack()`、`elements/entries/get` 仅产生冻结数据。`export_variable/export_global/export_member/get_owned` 才会登记新的可执行 root。缺失 Mapping 键不插入。快照图身份不可恢复为内部头，Function 元数据不输出捕获。`owners` 是观察当时的语义持有数，不随快照继续变化。

`Vm::run` 只返回 `RunOutcome::Finished(CompletionMeta)`，重复查询不新增 root。`take_returned()` 一次领取实际返回值；领取失败保留完成槽以便重试，重复领取或非返回完成报 `InvalidState`。完成状态在领取后仍可观察。`into_objects()` 转移同一 store 与注册表；关闭 store/VM 后旧句柄不能导入、retain 或查询，已有快照仍有效，句柄 Drop 仍释放结构。

`HostCallback` 的参数顺序为 `(&HostCallContext, &HostScope<'call>, &[BorrowedValue<'call>])`。借用不能逃逸；通过 `scope.retain(&args[i])` 保存独立 root。返回使用 `HostResult::Void/Borrowed(index)/Owned(root)`。Scope 仅提供 retain 和纯值构造 `value/array/mapping`，不含 VM 执行或绑定修改接口。返回越界、跨 origin、签名或图限制错误保留原故障现场；回调不会重放，已保存 root 与外部副作用不回滚。Owned/Borrowed 返回在激活中计数持有，直到结果槽提交或错误清理。

## 所有权与原子性

`ownership::Counter` 独立于物理 Rc 引用数：实际 Cell 载荷、Host lease、完成槽、宿主返回激活及构造任务参数持有语义 Owner。临时读取的内部 Rc view 不计数；同一 Cell 的 Reference 别名不重复计数；Array/Mapping 头别名不复制成员 Cell 所有者。头身份、store origin 与 lease ID 均检查溢出，不复用。没有新增按 `Rc::strong_count` 决策的运行时分支。

HostValue 只包含注册表、lease ID 与预分配退休标志。Drop 无分配：设置标志并尝试排空；注册表短借用临界区内只标记，下一入口排空后再检查容量/导入/retain。内部 Cell/Mapping 的 guard 不跨 callback；执行值不反向持有注册表。注册表排空时不调用宿主代码。Cell 替换先预留 Owner、获取 guard、提交，再于 guard 外释放旧载荷；完成槽在清理旧栈之前预留 Owner。T02 的稳定捕获槽、共享源 COPY 规范化仍是后续工作。

默认 `ValueLimits` 为 65,536 个 host roots、262,144 次图访问、深度 64、65,536 项。单次聚合观察共享工作预算和 visited 身份集合。构造、导入和回调结果检查 origin 与当前可达图限制；现有容器深度/键/写入限制保持，Function 捕获仍只有标量。本批不宣称完成 T03 的 Function 捕获边投影图验证。

## 回归与证据

原 558 项场景继续运行。原 `tests/vm_core.rs` 的 457 项移动到私有 `src/regression.rs`，仅增加测试命名空间别名与一个回调所有权适配 helper；去掉这两处适配后文件字节与冻结原版相同。其余原 101 项仍在原模块/解析器集成测试运行。原断言未删除、未更改。这个内部语义套件不被计为新公共 API 的集成测试，旧 API 不对外保留。

新增 6 项内部所有者/退休/耗尽检查、12 项公共 VM 集成测试、1 项真实编译器契约测试（正例与 6 个诊断检查负例），另有 2 个文档编译负例。总计 577 项 Rust 普通测试、2 项 doctest；原 558 项名称在 debug/release 输出逐项核对。

[验证脚本](verify_validation.py) 的 `--write` 重跑 debug/release、fmt、clippy、56 项独立模型测试、原生静态证据检查和全 PAK 对比，生成新收据。旧 2AQ/2AR/2AS 文档、收据、模型和分析制品均冻结。旧 source-sensitive 校验器继续描述其原阶段，不能拿旧源码行定位校验已迁移代码，也不覆盖旧收据；2AT 改由新基线显式登记源码允许变化。

重跑：`python3 -B docs/stage2at-evidence/verify_validation.py --write`；只检查收据与文件：去掉 `--write`。证据仅支持静态结构和合成 VM 行为，不能证明参考程序运行等价。
