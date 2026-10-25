# 2AX 方法与复现

本批整合已取得的原生静态记录，不执行 `magic_Linux32` 或业务字节码，不重复启动原生分析。本轮收尾没有新增 REA 调用；阶段 2AX 的 `new_native_analysis=true` 指既有新增包中的 8 条完整 dossier，不代表运行观测。目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

## 输入与证据权限

- 2AW 基线为提交 `5a888692d884cdefde9c03ceab90b0cab17e8a87`。`baseline-inputs.json` 固定其 704 个文件；除 README 和路线图允许前置追加、保持旧内容完整后缀外，全部文件逐字节保持冻结。原 ELF、PAK、对象单元、启动脚本和 Rust 测试均不改写。
- `rea.json.gz` 保留 8 条新增完整 dossier，`reused.json.gz` 保留 57 条历史记录。`function-manifest.json` 对照原 ELF 的符号、文件偏移、完整 inclusive 分段和记录哈希；`reused-manifest.json` 指回冻结来源。记录数不等于不同函数数。
- `session.json` 记录既有采集请求与关闭状态。原始 `raw_result`、provider 限制、未解析间接调用及分析范围均保留；value-flow 未截断不代表间接 hook、所有退出域或运行行为已经穷尽。
- 观察、静态推导、Rust 设计与未知分别登记。`anchors.json` 用原 ELF 字节核对记录中的指令；机器码地址不是业务字节码偏移。静态记录不是原程序运行 oracle。

## 总账与人读视图

`findings.json`、`expected-cases.json`、`questions.json`、`case-map.json` 与锚点构成机器总账。原 2AR 的 85 个锚点、23 条结论和 51 个案例作为完整前缀保留；新增结论和案例单列重评。案例映射逐项保存原 basis 和 findings，不把旧 unresolved 案例追改为已验证行为。

问题父域保留原范围，子问题只回答明确命名的接点。普通解释线程的局部清理结论不关闭 hook、其他线程、JIT 或完整 GC。后续设计见 [T05 规格](runtime-design.md) 与 [门控任务](next-batch.md)。

确定性生成人读文档：

```sh
python3 -B docs/stage2ax-evidence/render.py --write
python3 -B docs/stage2ax-evidence/render.py
python3 -B docs/stage2ax-evidence/verify_evidence.py
```

无 `--write` 的 render 只核对视图。实际证据校验入口同时核对文档存在、完整 Evidence ID 的归属、全部新增记录的结论/锚点引用及生成视图一致性。这些检查不证明自然语言推理正确或静态覆盖完整，语义仍需逐记录复核。

## 验证与收据

文档和总账定稿后运行：

```sh
python3 -B docs/stage2ax-evidence/verify_validation.py --write
python3 -B docs/stage2ax-evidence/verify_validation.py
```

写入仅涉及本批 `validation-logs.json.gz` 和 `../stage2ax-validation.json`。完整命令清单由校验脚本固定：2AX/2AR 证据校验、冻结 2AS 模型与 56 项模型测试、2AX 证据/收据测试、Rust debug/release、build、fmt 和严格 clippy。Rust 每种测试配置要求全部 644 项普通测试与 2 项 doctest 的身份保持，不接受以相同数量替换场景。公开编译负例由既有测试继续覆盖。

既有 40 项规则/收据测试保留；新增检查直接运行实际 2AX 验证，并对缺文件、悬空引用、历史案例改写、问题错误关闭、生成视图缺失/过期及日志摘要替换作负例。测试夹具中的模拟日志只用于校验器负例，正式收据仅由真实 subprocess 结果生成。收据独立校验重新执行实际证据验证，要求日志中的证据摘要等于当前交付摘要，并复算所有交付哈希。

2AW 原收据和日志仅只读复核。语料结果复用冻结的 `../stage2aw-evidence/corpus-validation.json`，明确 `rerun=false`：12 包、9,527 成员、7,159 程序、9,933 继承关系及 6,474,773 条指令是历史静态结构结果，不是本轮重新运行或执行等价证明。唯一已知异常仍为 `dba/pack_data/etc.pak/etc`。

最终能力保持 `container_capture_ready=true`、`direct_owner_capture_ready=false`、`non_scalar_ready=false`、`stage2_complete=false`。T05 门控规格完成与运行时可开放分别验收；下一批标识为 `T05-lifecycle-contract-evidence`。
