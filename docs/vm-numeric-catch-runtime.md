# 阶段 2W：数值错误捕获扩展

## 一、总体结果

在[阶段 2V](vm-catch-runtime.md)的受限 catch 上，补齐实数/混合除零、整数余数零及相应复合赋值错误来源。操作码仍为 **68 种限定支持**；主语料交集 66 种、缺 7 种，阶段 2 未完成。本轮没有新增操作码、公共 API、依赖或 unsafe，也没有扩大生命周期、宿主回调、线程和 JIT 的捕获范围。

所有错误仍由具体失败位置标记，不能按 `Arithmetic` 或错误字符串统一捕获。既有两阶段展开、预算、局部槽身份、共享容器和引用配额机制保持原契约。没有处理器时，原错误种类、文本、函数、PC、偏移及操作数现场保持不变。

## 二、原生证据与来源表

本轮应用 `reverse-engineer-anything` 技能，复用 8 份 REA/Ghidra 12.1.4 原始记录，来源为 3 个历史证据包；未重复查询、未打开原生会话。主 ELF SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。完整反汇编、反编译和限制见[函数证据](stage2w-evidence/functions.md)、[清单](stage2w-evidence/function-manifest.json)与[逐来源对照](stage2w-evidence/fault-sites.json)。这些是静态观察，不是原程序运行结果。

| 私有来源 | 指令与条件 | 原生定位 |
|---|---|---|
| `IntDivZero` | `19`，Int / Int(0)，沿用 2V | `vm_divValue@0x0808e1a4`，零分支 `0x0808e27f`，错误调用 `0x0808e2ea`。 |
| `NumericDivZero` | `19`，Int / Real(±0)、Real / Int(0)、Real / Real(±0) | 同函数，错误调用依次为 `0x0808e29d`、`0x0808e2b2`、`0x0808e2cf`。 |
| `IntRemZero` | `28`，Int % Int(0) | `vm_modValue@0x0808e7bc`，分支 `0x0808e848`，错误调用 `0x0808e8ae`。 |
| `CompoundDivZero` | `2f` 选择器 5，数值目标与零除数 | `vm_opEqValue@0x0808eb68`，`0x0808ecc2` 委托到 `vm_divValue` 的四条零错误分支。 |
| `CompoundIntRemZero` | `2f` 选择器 11，Int 目标与 Int(0) | `0x0808ec52` 委托到 `vm_modValue` 的整数零错误分支。 |

除法汇编 `ev_19597a6d356a5c98c32c16698ef7be5e4ad2201b7a0a22f7a714cc7918dd6f89` 与余数汇编 `ev_4066c4eef4bbbc302618e0bb8a65aeab459dde67f388ba53c0695f2616f8a31d` 确认错误调用均为 `vm_error@0x08097b24`。复合赋值汇编为 `ev_354a97d72f74e25fe049521f079d6411eef25ea6be56c075ef7da6aab3dbeb4e`，反编译为 `ev_983266641ef48cd9821c04963ca8b83351cf78fef7b46f841916bcb58d93139e`。

**已确认的边界：**浮点除数 NaN 的无序比较经 `JP` 绕过零错误；负零比较等于零。浮点/混合余数调用 `fmod`，没有整数余数的零错误检查，因此继续产生既有 NaN 结果。整数除法、余数的 `MIN/-1` 会进入 `IDIV`，未见对应软件溢出保护，Rust 继续将其作为不可捕获的安全错误。

### 复合赋值的失败前状态

原生 `vm_opEqValue` 先扩展临时栈、复制右值并调用 `vm_copyValue` 复制目标，再执行算术。成功后才在 `0x0808ebf3` 清理目标、`0x0808ebfc` 写回结果，随后解除临时引用并收栈。数值目标复制的是标量，所以已确认的除零路径不会写回目标。

原生与 Rust 的临时布局不同：Rust 在局部变量中读取左右值并先计算，失败时保持引用、右操作数和目标不变，再由 catch 按保存边界清理。这里只承诺已限定数值路径的目标与恢复结果，不声称中间内存布局相同。`vm_copyValue` 对其他类型可能产生规范化副作用，不将数值结论扩大到对象、函数值等类型。

原生复合处理器内未见 Rust 同样的只读检查；Rust 保留现有只读、失效引用和所有者检查优先级，相关失败不标记为算术来源，不宣称原生非法引用路径的错误优先级等价。

## 三、Rust 改动与状态规则

`RuntimeFaultSite` 新增四个来源，并保留既有 `IntDivZero`。普通二元运算只在算术返回错误后，以实际指令、左右值标签和除数分类；引用读取和栈检查发生在此前。复合赋值只在解析引用、拒绝只读、验证所有者并读取操作数之后，才在选择器 5/11 的算术失败处标记来源。

流程可概括为：

```text
验证栈与引用 → 读取数值 → 执行算术
    失败且命中明确零来源 → 固定原失败位置 → 既有 catch 路由
    其他失败             → 既有粘性失败或生命周期任务错误处理
    成功                 → 分配结果 → 写回目标 → 消费操作数
```

来源仅存活于当前指令，不会使之后的类型错误或限额错误变成可捕获。恢复仍只清理执行资源：之前已经提交的变量写入或映射键插入不回滚；离开范围的成员引用释放配额，仍存活的引用继续计数。处理器已消费后再次产生本轮允许的错误，可以进入外层处理器。

浮点余数零、NaN 除数和非零除数继续走原正常数值路径；既有整数截断、实数舍入和浮点环境限制不变。`MIN/-1`、操作数类型错误、失效局部/对象引用、只读引用、配额、内部状态及返回验证错误继续不可捕获。

## 四、验证与历史兼容

新增 17 项 VM 集成测试、3 项内部单元测试；共 **323 项 Rust 测试**（18 单元、23 解析/CLI、282 VM）。覆盖数值类型组合、±0、NaN/无穷、非零除数、整数溢出反例、局部/对象/数组/映射目标、失败未写回、已插入键保留、配额回收、嵌套再失败、跨对象及逐步预算恢复。

内部测试确认只读、退出帧、已删除对象、非数值目标和引用右值先失败，来源仍为空；有效复合零来源也不改变目标和操作数。无处理器测试核对原始诊断及负零位模式。

保留既有测试用例；2V 的“实数/余数零不可捕获”反例改为本轮允许路径的正例，整数溢出和类型反例继续保留。旧测试 `catches_marker_limit_and_compound_zero_remain_uncatchable` 重命名为 `catches_marker_limit_remains_fatal_and_compound_zero_uses_handler`，明确保留 marker 限额反例并更新复合零预期。此重命名不计入新增 20 项测试。

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
python3 -B docs/stage2w-evidence/verify_evidence.py
python3 -B docs/stage2w-evidence/verify_completion.py
python3 -B -m unittest discover -s docs/stage2w-evidence -p 'test_*.py'
python3 -B docs/stage2w-evidence/verify_validation.py
```

全量只读比较器验证 12 个包、7,159 个程序和 36 个原始制品，仍仅保留已知 `dba/pack_data/etc.pak/etc` LZSS 异常，见[语料记录](stage2w-evidence/corpus-validation.json)。[当前矩阵](stage2w-evidence/matrix.md)只更新相应语义分支；[阶段验证](stage2w-validation.json)保存来源、历史和本轮交付哈希。旧阶段文档、矩阵和证据保持冻结。

## 五、剩余限制

没有执行参考 ELF、启动脚本或原始业务字节码，也没有原版动态差分证据。本轮不代表完整异常兼容或服务端可运行。剩余七种操作码、完整生命周期捕获、宿主回调/JIT/线程、任意原生错误来源及 last-error 查询继续单独取证与实施。
