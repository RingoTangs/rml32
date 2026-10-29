# 受限启动探针接口

`StartupProbe::load(root, service, limits)` 从固定服务成员读取、解压、认证并完整解析真实映像。`StartupProbe::new(image, limits)` 接受调用者提供的可变 ProgramImage，因此只校验执行权限和结构，不把其 `sha256` 标签当作认证结果。CLI 始终使用 load。

`StartupProbeLimits::isolated()` 提供有名称的固定配置，不实现 Default：

- ValueLimits：32 个 Host 根、8192 个图节点、深度 16、4096 个 items。
- VmLimits：64 个栈项、2 个帧、64 个临时引用 Cell。
- LifecycleLimits：0 个对象、0 个 Function 头、每对象普通引用上限 2、执行引用上限 1、每头 owner 上限 8。

这些仍是现有配额的含义。Global 引用指向已有稳定槽，不占临时引用 Cell 额度。调用者可显式传入更严格配置；探针没有暴露内部 store、VM、HostValue 或宿主注册权限。

`run(budget)` 返回 BudgetExhausted 或 PrefixReached。预算被限定为剩余许可指令数；零预算不执行或分配 efun 结果；边界完成后重复调用仍返回 PrefixReached，不能执行尾段。任何执行失败为 sticky，后续 run 返回首个错误，不重放已完成回调。

`report()` 返回只读、独立拥有且不含可导入身份的 StartupProbeReport。包含来源、成员哈希、函数基数、原始初始化长度、已执行数、停止 PC/偏移、停止原因、全局值、实际 Host 调用、下一项依赖及配置。`initialization_complete` 始终 false；失败后 stop_reason 为 failed。Array/Mapping 内容在报告中完整保存，不用长度摘要代替元素检查。

`allocate_array` 仅支持一个 Int 参数，无可选初值。负数明确报 Bounds；超过现有 Rust items、图节点或容器上限以及分配/预留失败报 Limit，不发布伪造成功结果。成功数组元素均为 Undefined。一般原生可选初值复制、共享空数组单例身份及原生失败返回 0 均未由该受限实现声明兼容。

旧准备器仍不会准备 Global init，也不会因此开放 Object 生命周期中的 06/Global 指令。新内部初始化帧使用真实初始化代码视图，没有普通函数槽；内部 HostCallContext.function 的 -1 是该探针的初始化标记，报告及 VmError.function 不将其作为普通函数槽输出。
