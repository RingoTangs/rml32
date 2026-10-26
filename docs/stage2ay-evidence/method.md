# 2AY 方法与复现

本批只做 E01 未发布零捕获头与 E02 最终 PCB 清理后安全点的定向静态取证及门槛重评。没有执行 `magic_Linux32` 或业务字节码，没有实现 Rust 生命周期或新增独立状态模型。目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

## 基线和证据

基线提交为 `e67ba838cf34415faadc8a109833e7d6d0e45655`，`baseline-inputs.json` 固定全部 734 个已跟踪文件。README 和路线图只允许在完整基线内容之前追加当前状态，其他文件逐字节保护。原始 ELF、PAK、对象单元、脚本、Rust 源码与测试、旧总账和收据均不改写。

复用 2AX 的全部 65 条记录，另复用主证据包内的 `vm_mainLoop` 与 `vm_regularProcessor` 完整 dossier。`reused-manifest.json` 指向冻结来源并记录源文件与完整记录的哈希。新增完整记录放在 `rea.json.gz`，新增/复用 Evidence ID 不重叠；函数清单按原 ELF 符号、文件偏移、inclusive 分段和原始字节哈希复算。完整 `raw_result`、provider 限制和未解析间接调用均保留，不以 stdout 摘要代替原始记录。

本批 REA 仅打开同一原始 ELF 的临时 Ghidra 数据库。`session.json` 记录请求、失败原因、完整结果标识、导出及关闭状态；输入参数被拒绝与分析失败分开记述。关闭分析数据库不会修改目标制品。实际新增记录数、函数数和记录 ID 以最终清单及收据为准，不预造通过数量。

## 推理边界

E01 从普通 PCB 栈初始化与动作表间接分派核查失败候选，分别追踪函数头、捕获存储和 creator 引用。allocator 的块管理、统计、底层内存释放和 VM 语义析构分开处理。空的直接 caller 列表不能排除动作表分派；局部未见 free 也不能证明全局泄漏。

E02 将立即/延后归零、单次调度、下一普通执行片段、最后线程、主循环返回与停止/清理分支逐域记录。局部次序、条件化下一次排空、外部持有者最终释放的进展性是不同结论；服务线程、回调和任意 hook 行为不因枚举调用者而获得完整契约。

观察、静态推导、Rust 设计和未知分别登记。新总账完整保留 2AX 的 180 个锚点、40 条结论和 71 个案例作为历史前缀；重评使用新增结论/案例和全部 71 项映射，不追改历史依据。Q05/Q06 的父域不能由普通线程局部证据关闭。自然语言结论仍需人工对照完整记录；自动检查只覆盖所声明的身份、引用、字节、状态与交付一致性。

## 生成与验证

```sh
python3 -B docs/stage2ay-evidence/render.py --write
python3 -B docs/stage2ay-evidence/render.py
python3 -B docs/stage2ay-evidence/verify_evidence.py
python3 -B docs/stage2ay-evidence/verify_validation.py --write
python3 -B docs/stage2ay-evidence/verify_validation.py
```

不带 `--write` 的命令只读核对。正式收据仅捕获真实 subprocess 的返回码和完整日志，不使用单元测试里的模拟日志作为运行证据。写入只涉及本批收据和日志；历史 2AX 收据按冻结日志、场景清单及基线文件哈希复核，不用旧整批交付哈希检查已追加的新首页。

完整验证要求保留原 108 项 Python 测试（56 项冻结模型及 52 项 2AX 校验），并运行新增实际交付和负例测试。Rust debug/release 各保留全部 644 项普通测试与 2 项 doctest 的场景身份，通过 build、fmt 和严格 clippy。新增检查覆盖材料缺失、悬空引用、历史前缀改写、71 项映射遗漏、错误关闭未知/父域、来源哈希、视图过期和收据日志/场景替换。

语料结构结果继续复用经过哈希核验的 2AW 记录，明确 `rerun=false`：12 包、9,527 成员、7,159 程序、9,933 继承关系及 6,474,773 条指令仍是历史静态结构结果。本批未重跑全 PAK 检查，唯一已知异常仍为 `dba/pack_data/etc.pak/etc`。

最终结论与范围见 [本批说明](overview.md)、[门槛重评](gate-review.md) 和 [后续任务](next-batch.md)；真实执行统计见 [2AY 收据](../stage2ay-validation.json)。`container_capture_ready=true` 仅沿用 2AW 的受限容器能力；`direct_owner_capture_ready=false`、`non_scalar_ready=false`、`stage2_complete=false` 始终保留。
