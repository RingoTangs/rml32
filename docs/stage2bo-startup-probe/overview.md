# 阶段 2BO：首个真实启动切片

新增独立 `rml32-startup-probe` 命令与 `startup_probe` Rust 模块，复用现有 VM 执行真实 Global 初始化的 PC 0—24，在 PC 25 的独立边界停止。AAA 为主验收样例，CCS、CSA、DBA、GS 同时验证这一公共前段。原 `rml32-inspect` 保持只读检查用途，裸 `cargo run` 继续默认启动它。

```sh
cargo run --locked --offline --bin rml32-startup-probe -- --service aaa --root .
```

stdout 是一个 JSON 报告；到达边界退出 0，读取、认证或执行失败退出 1，用法错误退出 2。根目录默认为当前目录，必须显式指定五个服务之一。没有扩大终点或注入回调的命令选项。

五份原始 PAK 成员均在解压后核对固定 SHA-256，完整结构解析后只准备许可的前 25 条指令。没有改写原制品、补造函数或追加初始化返回。真实执行的唯一 efun 为一次 `allocate_array(1024)`；所有服务的下一项外部依赖都是 PC 26 的槽 336 `mkdir`，尚未执行 PC 25 的路径参数压栈。

| 全局槽 | 切片结束值 |
| --- | --- |
| 2 | 长度 1024、全部为 Undefined 的 Array |
| 3、4 | Int 2、Int 0 |
| 5 | Bytes `/xdbc/` |
| 6 | 空 Mapping |
| 7 | Int 0 |

其余槽保持探针显式设置的 Undefined；这不是原生完整 bootstrap 初态的证明。五份原始全局容量依次为 105、102、98、98、359，函数基数 572 保持，559—571 的预置函数没有补造绑定。

新增 16 项 Rust 测试：10 项探针私有回归、4 项运行期防护/存活期回归、2 项正常库及 CLI 集成场景。当前 debug/release 各 **908 项普通测试＋5 项 doctest**，原 892 项普通测试和全部旧测试体保持。8 项 Python 校验测试检查报告、权限、资源、原生证据与失败命令污染。

完整当前验证与历史验证分别见 [机器收据](../stage2bo-validation.json)。旧 2BN 收据在独立、逐文件匹配的固定副本验证；17 个既有驱动在当前 Rust 上重新执行，语义结果保持。新报告来源、执行命令、源码与观察哈希分别绑定。

该结果只说明五个真实启动前缀可以执行。初始化完成、服务启动、阶段2完成、完整原生等价及通用 implementation_ready 均保持 false。参考 ELF 和旧启动脚本未执行，32 位 Rust 执行仍为 not_run，20 项既有原生未知没有关闭。详见 [实现与证据](method.md)、[接口](api.md) 及 [后续任务](next-batch.md)。
