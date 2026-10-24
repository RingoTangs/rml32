# 后续API迁移契约（2AS定稿，尚未实施）

2AS允许未来源码不兼容迁移，本批public_api_changed=false。api-migration.json逐项锁定16个现有边界、源码定位、角色和迁移方式。T01必须一次迁移所有公共可执行Value出入口，不保留偷偷登记/丢失root的旧接口适配器，不把可回传Value快照直接改成“不计数但可回传”。

## 类型与方法

以下为后续Rust接口契约；Result错误均用现有VmError/VmErrorKind，签名中的生命周期必须在T01以编译测试证明，不能把伪代码当作已编译API。

- `ValueInput<'a>`：现有标量字面量或`&'a HostValue`。不允许外部构造Function/Array/Mapping内部ID；ObjectId仍需本store校验。
- `HostValue`：opaque、不可Clone、Drop释放lease。`retain(&HostValue) -> Result<HostValue, VmError>`显式复制持有者；`snapshot(&HostValue) -> Result<ValueSnapshot, VmError>`纯观察。origin不匹配、store已关闭或lease退休均在导入前失败。
- `ValueSnapshot`/`FunctionSnapshot`：不可执行的冻结数据，Graph ID仅供身份/共享观察。无into_value/import/revive接口；Debug不retain，Function元数据不包含捕获。
- `BorrowedValue<'call>`：Host本次参数借用，不可逃逸；`HostScope<'call>::retain(&BorrowedValue<'call>) -> Result<HostValue, VmError>`是唯一跨回调提升能力。scope无VM重入接口，不能持RefCell guard跨callback。
- `HostResult`：Void、Borrowed(参数序号)、Owned(HostValue)。越界参数序号或签名/图不匹配是原现场错误；回调恰好一次，Owned失败后正常drop，外部已保存root和副作用不撤销。
- `ValueLimits`：在ObjectStore构造时指定，默认host roots=65536、图工作节点=262144、扩展深度=64、项数=65536；每个新root预留/ID溢出失败先于发布。模型不表示ID耗尽和Rust内存OOM的全部细节。

## 观察与导出分开

stack、variables、global_values、elements、entries、get均返回Result包裹的Snapshot数据，不返回HostValue。Array/Mapping的顺序和身份观察保留；选择元素产生可执行持有者必须显式`export_member`/`get_owned`，按raw opaque传输登记独立host root。缺失键返回None，不插入节点。snapshot查询不能顺带做普通VM COPY源规范化。

`run`完成后返回CompletionMeta（返回/结束/生命周期等现有完成种类），重复查询不导出root。`take_returned() -> Result<HostValue, VmError>`仅一次raw移动完成槽到host root；未完成、无返回或重复领取报InvalidState。需要多个可执行副本时retain，观察结果用snapshot。

start接收`&[ValueInput]`，预检后导入临时槽并执行entry COPY；create/register_globals沿原初始化输入的raw运输语义。`ObjectStore::array/mapping`接收ValueInput并返回HostValue，做origin/键/图检查，构造不递归规范化成员。标量返回也可装入HostValue，以统一一次领取/释放契约；标量复制本身不影响Function计数。

## 保存、关闭与迁移

root registry使用不可复用的store origin/lease ID。HostValue持有私有registry控制块，store关闭后不能重新导入，但Drop仍能结构性释放载荷；into_objects转移同一store，不关闭registry。Drop在内部epoch只设置预分配retired标志，不分配、不panic；epoch结束排空前不进入新执行/导入/retain。

T01编译测试必须拒绝：把Snapshot传入start、构造HostValue内部ID、Clone HostValue、把BorrowedValue保存到回调外、从HostScope调用VM。运行测试必须覆盖：retain/Drop、重复run/一次take_returned、错误/中止后释放、关闭/into_objects、跨store、保留回调参数、非法HostResult，以及旧标量Function身份/顺序/预算场景。

现有558项测试保留行为断言，边界helpers统一迁移。涉及真实行为差异的断言只能按已登记设计决策和原生证据更新，不能以编译迁移为理由删除。旧所有权与新类型不能在T01完成时并存为两条隐式执行通路。
