# 验证来源与复现

本批基于 2BH 提交 `f99812460c374adb49bdffd42dee84bcc1f86c8c`。除显式列出的 11 个既有 Rust 文件及 README/路线图前缀外，1004 个基线文件按 SHA-256 冻结。`src/value.rs`、`src/prepared.rs`、所有旧测试/fixture、旧模型、旧报告和 `magic_Linux32` 都保持原字节。三个新增 Rust 文件及全部实际修改源码绑定到 2BI 收据哈希；新增源码集合必须精确匹配。

[verify_boundary.py](verify_boundary.py) 还使用冻结 tokenizer 对比允许修改文件里的原有 156 个测试函数体，不允许删除或修改旧断言。正常测试的 819 项旧名称必须全部保留，新增 32 项名称从 [case-coverage.json](case-coverage.json) 精确核对；该映射完整保留 2BH 历史前缀、115 项原始静态案例和二十项未知。

18 项 [公共 API 集成测试](../../tests/bounded_lifecycle.rs) 链接非 `cfg(test)` 的库，验证构造范围、legacy 行为、各类配额、creator 唯一、峰值执行交接、raw/COPY、回调后失败不重放、pending/失败 task、分预算维护、转移和关闭。14 项私有生产路径探针覆盖实际借用/工作区部分成功、根顺序、零预算、版本冲突、未发布 head abort、物理 view 延迟回收、控制块保活/外部环解除及元数据借用重试。它们没有修改准备权限或增加新 bytecode 能力。

debug/release 各运行 851 项普通测试及 5 项 doctest。旧 2 个 HostValue 编译负例保持，源码增补使其行号移动，本批按原类型身份与实际源码哈希验收；新 3 个编译负例检查 Limits/Budget 无默认值及 HostScope 无维护入口。另执行两种 build、fmt 和严格 clippy。六个 ignored 驱动分别用冻结输入与冻结模型回放，旧报告不写回，生成 [独立新报告](reference-replay.json)。这些回放证明兼容既有模式，不是生产配额与模型的逐状态等价证明。

[model_replay.py](model_replay.py) 在独立 Python 解释器中运行未修改的 2BH quota checker，将本次 39 个模型案例、112 行算术结果和日志只写入 2BI。32/64 位结果来自独立规格检查；Rust 测试执行于当前 host，未将它们声明为 32 位 Rust 二进制运行结果。没有新增 Python 状态模型，也没有将模型成功视为 Rust 内存安全或 OOM 恢复证明。

[historical.py](historical.py) 在固定 2BH 临时副本运行该批 63 项 Python 检查、只读验收及其原始嵌套历史 392 项，共 455 项。最外层 clone 使用 `--shared --dissociate` 独立保存对象，避免多批共享 alternates 链超过 Git 深度限制；内层旧脚本不变。11 个已知 CRLF 文件按冻结哈希还原，所有源文件都核验后才运行。历史检查不是当前源码检查，`historical_python_tests_run_on_current_source=false`。新目录另有 10 项验收负例测试。

2AW 语料报告仅按冻结哈希复用，`rerun=false`。本批没有运行参考程序或业务字节码，只运行合成 VM 程序；不新增原生结论，不提升二十项未知。`bounded_lifecycle_ready=true` 限定为本批明确选择的 Rust 策略，不提升通用生命周期/捕获门槛。

完整复现与只读核对：

```sh
python3 -B docs/stage2bi-production-lifecycle/verify_validation.py --write
python3 -B docs/stage2bi-production-lifecycle/verify_validation.py
```

首条重跑实际命令并生成 [stage2bi-validation.json](../stage2bi-validation.json) 与 [命令日志](validation-logs.json.gz)；第二条核对已捕获日志、当前源码/工件与冻结边界，不重新执行测试。旧验证器只能在各自固定副本运行，不通过改旧期望消除源码版本差异。
