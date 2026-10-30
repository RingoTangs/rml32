# 切片、目录和提交行为

`StartupSlice::AllocationPrefix` 的终点为 25，`LogDirectory` 为 28。旧 `PREFIX_END_PC=25`、构造器和配额类型保持；`load_with_slice(root, service, limits, slice)` 继续认证原始解压成员 SHA-256 并完整解析映像。`new_with_slice(image, limits, slice)` 接受可变 ProgramImage，不能把调用者提供的 sha256 标签当作认证结果。

权限和终点保存在 PreparedInitialization 的私有 startup_slice 元数据中。普通 Object 初始化为 None，不获得此权限；两个 Global 切片分别限定自己的终点和允许的 06 操作数。终点不从指令向量长度推断，故追加指令不能绕过边界。没有新增普通函数调用能力，错误仍使用实际 source、initialization、pc 和 offset。

内存目录容量为 16 个节点，包含六个初始目录；规范化路径最多 255 字节。支持绝对字节路径，按 C 字符串首个 NUL 截断，合并重复斜杠、移除末尾斜杠，根仍为 `/`。不依赖宿主 Path/Unicode 规则。相对路径、`.`、`..`、`.pak/`、`.dbfs/` 和 `/v/` 路径明确报 UnsupportedFeature；长度越限报 Bounds。这些是受限 Rust 策略，不表示已恢复一般挂载或完整原生路径语义。

逻辑创建失败返回 Int 0，成功返回 Int 1；配额和工作区不足报 Limit。新目录节点在路径和节点预留成功后发布。Host 回调完成目录变更后，若结果根或后续 VM COPY 提交失败，目录变更保留，VM 操作数和失败 PC 保持，后续 run 返回首个错误且不重放。报告的 mkdir result 表示内存后端结果，不能单独解释为 VM 提交成功。

新模式增加可序列化、只读的类型化 `host_events`、`filesystem`、`slice`、`next_dependency_kind` 和 `next_dependency_reason`。原 `host_calls` 保留为兼容的 allocate_array 记录；新模式的完整调用顺序由 host_events 给出。字节路径复用 ByteString 的 bytes_hex 表示，不携带可导入的存储身份。文件标记仅用于私有测试初态，后端没有文件内容读写或宿主目录能力。

零预算不执行新指令。预算大于剩余切片时被截断；PC 28 后重复 run 仍返回 PrefixReached。初始化完成标记始终 false。CLI 成功到达边界退出 0，结构/认证/执行错误退出 1，用法错误退出 2；逻辑 mkdir 返回 0 不被强行转换成脚本错误。
