# 验证方法与证据范围

本批的 [基线清单](baseline-inputs.json) 固定 2BG 提交 `5827a6387fc2fbaa142a428133316c446713c230` 的全部 980 个文件。README 与路线图仅允许前缀追加，完整旧字节串必须仍是当前后缀；其余 978 个文件的 SHA-256 必须相同。还核对 `src/`、`tests/` 文件集合，禁止通过新增 Rust 或 fixture 绕过冻结边界。参考 `magic_Linux32` 未修改、未执行。

## 新契约模型

`quota_model.py` 独立模拟资源账本、实际槽、Function creator、帧、对象队列、header 成员、物理 view、prospective 责任和元数据游标。每个操作在候选状态执行，独立 `audit` 从记录/槽/帧/队列/预留重新推导计数，成功才发布；拒绝操作不发布候选状态。维护调用内部逐单位提交，遇阻塞返回部分成功的进度。这里的 Python deepcopy 只是一种事务规格，不是建议的 Rust 实现策略。

[39 个行为案例](test_model.py) 包含峰值覆盖、执行租约交接、队列保留单位、zero-exec 托管清理、Host raw 与脚本 COPY 差异、退休 head 占额、reap 与 ID 不复用、分预算部分成功、借用重试、扫描脏 epoch/回绕及计数损坏的负例。案例采用显式的小配额，未进行全状态空间枚举。

[check_contract.py](check_contract.py) 实际运行这些 unittest 并保存输出，不以静态计数代替测试执行；再以独立算术期望检查 32/64 位 checked-add 和五字段边界，共 [112 行](arithmetic.json)。结果关联模型、测试、检查器与规格源码哈希。篡改算术实现、配额校验、机器契约或捕获日志必须导致验收失败。

本模型不执行字节码、Host 回调或 preparation，不模拟真实借用调度。根名的有序标签代表 lease 顺序，ID 与弱条目是抽象标签；实际 Rust 的 lease 编号顺序、origin 防伪、slot generation、arena 内存布局、Rc 无环、Drop 不可失败及分配器行为不能由模型推出。模型演示显式源/目标版本冲突，不覆盖任意交错的生产执行。两份 World 的隔离检查也不是多线程安全证明。

[case-coverage.json](case-coverage.json) 完整嵌入并按哈希保留 2BG 案例映射，递归保留最初 115 项静态案例和二十项未知；十条新设计决策映射到实际执行的模型案例。映射不把 Python 结果升级为 Rust 实现验证或原生等价。

## 既有 Rust 回归与六层回放

完整验收对 debug/release 各执行 819 项普通 Rust 测试及 2 项 doctest，名称必须与冻结 2BG 收据精确一致（包括 doctest 行号）。正常测试中的六个 ignored 驱动仍保持 ignored，但 [reference_replay.py](reference_replay.py) 分别执行它们，并调用冻结旧检查器的只读比较逻辑。

| 冻结层 | 当前独立回放规模 |
|---|---|
| 2BG Host Function | 14 条 trace / 120 个检查点 |
| 2BF 托管初始化与构造 | 13 条 / 85 个检查点 |
| 2BE Host | 12 条 / 75 个检查点 |
| 2BD VM/ObjectStore | 10 条 / 70 个检查点 |
| 2BC 实际值槽适配 | 18 条 / 146 个检查点 |
| 2BB 内核 | 14,948 条冻结转换 |

旧回放输入、预期、检查器及旧报告逐字节冻结，禁止对旧脚本运行 `--write`。新报告和日志只写在 2BH 目录。旧 Rust 路径的成功说明已实现测试接点继续一致，不验证本批尚未接入的生产配额或维护 API。

同时执行 debug/release build、`cargo fmt --check` 和 `cargo clippy --locked --offline --all-targets -- -D warnings`。本批无新 Rust 测试，版本与依赖也不改变。

## 当前与历史 Python 检查

当前目录共 63 项 unittest：39 项模型测试、8 项契约检查器测试、8 项交付边界测试、8 项收据负例。它们在当前工作区执行。

[historical.py](historical.py) 另从固定提交创建临时共享 clone，核验并还原已知的 CRLF 字节差异，在副本运行未修改的旧检查器。外层 2BG 的 26 项加嵌套历史 366 项，共 392 项。历史结果同时保存源码提交、命令、退出码及日志哈希；`historical_python_tests_run_on_current_source=false`。不能在当前目录强行改旧期望以消除版本边界。

2AW 主语料报告按基线 SHA-256 复用，`rerun=false`。没有执行参考程序或业务字节码；旧回放仅执行合成测试程序。压缩日志和 [stage2bh-validation.json](../stage2bh-validation.json) 将上述证据分别记录。

## 复现

先在已有 Rust 工具链和离线依赖可用的环境完成本批验收：

```sh
python3 -B docs/stage2bh-production-contract/verify_validation.py --write
python3 -B docs/stage2bh-production-contract/verify_validation.py
```

第一个命令重新执行完整检查并仅重建 2BH 结果；第二个只读核对收据、实际命令日志、当前工件和冻结文件。修改模型/规格时，先分别运行 `contract_spec.py --write`、`check_contract.py --write`，同步新案例映射后再完整验收；不得重写旧批次工件。只读验收不会重新执行 Cargo 或历史测试，其结论绑定已捕获的日志与源码哈希。
