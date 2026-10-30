# 阶段 2BP：真实 mkdir 与内存日志目录

新增显式 `log-dir` 切片：五个服务均执行原始 Global 初始化 PC 0—27，在 PC 28 停止。PC 26 的槽 336 `mkdir` 由独立内存目录树实现；没有创建宿主日志目录，也没有接入网络、数据库或动态编译。

```sh
cargo run --locked --offline --bin rml32-startup-probe -- --service aaa --slice log-dir
```

默认命令和原 `StartupProbe::new/load` 仍停在 PC 25。新入口为 `new_with_slice/load_with_slice`，参数增加 `StartupSlice::{AllocationPrefix, LogDirectory}`；没有任意 end-pc、存储访问或宿主回调注入权限。旧模式 JSON 的新增字段全部省略，两个构建、五个服务的默认 stdout 与冻结 2BO 日志逐字节比较。

内存目录初态包含 `/` 和 `/aaa`、`/ccs`、`/csa`、`/dba`、`/gs`，每次探针独立。成功创建目录返回 Int 1；目标已存在、父目录缺失或父节点不是目录时返回 Int 0。原始程序随后弹出结果，所以这些逻辑失败仍可到达切片边界。节点配额或分配失败为 Rust Limit，不能冒充原生分配恢复等价。

新模式完整记录两个 Host 事件：`allocate_array(1024)` 和 `mkdir('/<service>/log/')`。全局槽与 PC 25 基线相同，目录快照增加对应服务的日志目录。下一阻塞点为 **PC 30 的 Global 槽 583 `xdbc_set_output_basic_file`**，其 atomic 标记尚未开放；PC 28、29 的参数压栈也尚未执行。

准备及运行时共同检查切片元数据、权限和终点。旧模式仍只能调用槽 0；新模式只增加槽 336、argc 1，不能调用槽 27、普通 Global 函数或越过 PC 28。追加已准备指令也不能扩大终点。

当前新增 12 项普通 Rust 测试，保留原 908 项和全部原断言，debug/release 各 **920 项普通测试＋5 项 doctest**。新增一个显式小矩阵驱动，18 个 ignored 驱动分别记账。定向差分在每个构建覆盖 **324 条场景、1296 个执行后检查点和 324 个初态检查**；其结果和原始观测一致。新 Python 校验测试为 8 项。

按本批选定策略，旧 17 个大矩阵驱动的完整结果只在冻结 `e4d7ca6` 副本复核，不宣称在当前源码重新跑过。本批当前源码的全测试、小矩阵与真实服务执行分别保存来源和实际命令。最终状态见 [验证收据](../stage2bp-validation.json)，细节见 [接口与行为](api.md)、[方法](method.md) 和 [下一批](next-batch.md)。

本批没有完成初始化或服务启动；阶段2、通用 implementation_ready 与原生等价门槛保持 false，20 项既有原生未知不关闭。未执行参考 ELF 和旧启动脚本，32 位 Rust 执行仍为 not_run。
