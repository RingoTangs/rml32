# 2AS后续API迁移表

本批不改API。以下接口摘要与api-contract.md共同构成T01契约，签名省略的错误均为Result/VmError。

| 当前接口 | 角色 | 后续契约 | owner变化 | 说明 |
|---|---|---|---|---|
| Vm::stack | observation | Result<Vec<StackSnapshot<ValueSnapshot>>, VmError> | none | Snapshot无Value/HostValue；保留Reference元数据，不输出捕获。 |
| Object::variables | observation | store.snapshot_variables(ObjectId) -> Vec<ValueSnapshot> | none | 删除旧可执行Value快照返回。 |
| ObjectStore::global_values | observation | snapshot_globals() -> Vec<ValueSnapshot> | none | 共享身份表现为ID，不保活header。 |
| ArrayValue::elements | observation | store.snapshot_elements(&HostValue) -> Vec<ValueSnapshot> | none | 需要取出可执行成员改用显式export_member，产生HostValue root。 |
| MappingValue::entries | observation | store.snapshot_entries(&HostValue) -> Vec<(KeySnapshot, ValueSnapshot)> | none | 顺序与原公开entries保持；快照深拷贝观察数据、别名用ID引用。 |
| MappingValue::get | observation | store.snapshot_get(&HostValue, KeyInput) -> Option<ValueSnapshot> | none | 显式export_member/get_owned方可回传VM，不静默把查询变成持有者。 |
| Value/FunctionValue::clone | host-ownership | HostValue不实现Clone；store.retain(&HostValue) -> Result<HostValue> | new-host-root | 标量ValueInput可Copy；内部VmValue不得经公共Clone越过root登记。 |
| FunctionValue访问器/Debug | observation | FunctionSnapshot metadata / HostValue.function_info() | none | owner/target/function/bound_count和身份不含捕获；HostValue debug不新增root。 |
| Completion::Returned/RunOutcome | observation | CompletionMeta + take_returned() -> Result<HostValue> | explicit-transfer-only | run重复完成观察不复制持有者，take_returned一次性转移。 |
| Vm::start | vm-input | start(object, function, &[ValueInput]) | new-vm-slots-after-preflight | 标量或借用HostValue；导入临时载荷后entry COPY，不改外部HostValue root。 |
| Value::array | host-construction | ObjectStore::array(&[ValueInput]) -> Result<HostValue> | new-host-root | 构造纳入origin/图校验，raw构造不递归规范化成员。 |
| Value::mapping | host-construction | ObjectStore::mapping(&[(KeyInput, ValueInput)]) -> Result<HostValue> | new-host-root | 键仍限Int/Bytes；不支持Function键。 |
| HostCallback | borrowed-call | Fn(&HostCallContext, &HostScope, &[BorrowedValue]) -> Result<HostResult, HostError> | explicit-retain-only | BorrowedValue生命周期限单次回调；HostResult::Borrowed/Owned/Void，Any/Mixed opaque语义保留。 |
| ObjectStore::create | vm-input | create(program, &[ValueInput]) | new-variable-slots-after-preflight | 原始初始化值导入，不虚构普通读取COPY；生命周期能力不扩展。 |
| ObjectStore::register_globals | vm-input | register_globals(&[ValueInput]) | new-global-slots-after-preflight | 保留原bank替换/旧引用语义。 |
| Vm::into_objects | lifecycle-transfer | 保留入口并转移同一ObjectStore/root registry | transfer-no-duplicate | HostValue按origin仍属同一store；关闭store后拒绝导入/retain，Drop仍可释放结构性root。 |
