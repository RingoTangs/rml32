# 阶段 2B：数值语义与验证

日期：2026-10-14。在 [阶段 2A](vm-core-semantics.md) 上增加实数、混合数值运算、整数位运算和类型判断，操作码集合扩大到 **39 种限定路径**。未增加真实包执行入口；测试仅执行人工构造的程序。

## 静态依据

REA MCP 显式选择 Ghidra 12.1.4，分析原 SHA-256 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`，保存 [24 条规范 Evidence](vm-numeric-evidence/rea.json.gz) 和 [反编译／汇编附件](vm-numeric-evidence/functions.md)，会话已关闭。算术及 typed 参数／返回转换复用 [阶段 2A 证据](vm-core-evidence/functions.md)，加法复用 [首轮证据](evidence/functions.md)。

| 例程 | 地址 | 本轮关键 Evidence |
|---|---|---|
| `vm_pushRealFromWord` 汇编 | `0x08090be4` | `ev_cdd7939016779eff6ec0d88ba540e0f49ae57bc8c487fa746a0ec1488602dcc4` |
| `_vm_ftol` 汇编 | `0x080a36e4` | `ev_88d9c6fb8cd9b4aae9e9dc4111b5a73b49b6386c133cac64b776a807a4e43f23` |
| `vm_castType` | `0x0809006c` | `ev_af0af51b13524e56b3c21ee5bfc96e632f628813884f539bb86e9cf91834a855` |
| `vm_lshValue` 汇编 | `0x0808df54` | `ev_21dbfb327290ca0685271bfb41c56232985c41cd73c0a3774f39312728193216` |
| `vm_rshValue` 汇编 | `0x0808e020` | `ev_1c5fafbbbc3f27ea7fa86a384a6d5f136eb481c392bae9110df18e0f732626f5` |
| `vm_isType` 汇编 | `0x0808f100` | `ev_98e6c9ff45230fc2a6aae46da320d253cb18a2bfbe5df25e2305c4cd36bfb3aa` |
| `vm_isEq` 汇编 | `0x0808c908` | `ev_611f55b6a7d06bc3a5a571d378a8c62a505be9c5c4c5cc08385dda151702b480` |
| `vm_isZero` 汇编 | `0x0808cad4` | `ev_dd7355f9ff76a95428a89da8d67e30fb22df7e858529878a656f90b9d6aa884f` |

反编译不能机械转为 Rust。以下结论由实际汇编纠正或确认：实数常量第二字未被读取；右移为 `SHR`；过大／负移位计数有显式保护；关系比较使用 `SETNC` / `SETA`，NaN 结果不能按伪代码中的布尔取反推定。

## 实数、转换与比较

新增 `RealBits(u32)` 和标签为 `3` 的 `Value::Real`，提供 `from_f32`、`to_f32`、`from_bits`、`bits`。结构相等按位比较，保存 signed zero 和 NaN 位；VM 数值比较独立实现。

`0x3f` 仍按动作表读取两个小端 32 位 metadata 参数并检查全部八字节边界。`vm_pushRealFromWord @0x08090c01–0x08090c17` 只将第一个参数写入实数槽，第二参数忽略，不能解释成 f64 常量。正负零与 NaN 常量位可原样保留。

`_vm_ftol @0x080a36e7` 是 `CVTTSS2SI`。typed 参数的 `0x0809547f` 和返回的 `0x08094ca4` 也使用该指令：向零截断，NaN、无穷和越界返回整数 indefinite `INT_MIN`。原生如此返回不等于 VM 检查错误。Rust 的裸浮点 `as i32` 饱和行为不同，兼容适配器显式处理该范围。

参数／返回的整数到实数使用 `CVTSI2SS`，先舍入为 binary32。数值 cast 支持 Int、Real、Undefined 和 Void 到整数／实数，其中后两者转换为零；字符串解析、buffer、容器转换及 const 属性仍明确拒绝。typed nullable 参数／返回保持 undefined，不把它隐式转换为零。

整数与实数比较先将整数转换为 binary32，例如 `16777217 == 16777216.0` 为真；该规则不适用于混合算术。NaN 下 `== >= > <= <` 均为假，`!=` 为真；正负零数值相等。逻辑非和条件分支把正负实数零视为零，NaN 非零。

## 算术矩阵

下表仅表示已实现数值路径，不覆盖原例程中的字符串、array 或 mapping 分支。

| 操作 | Int / Int | Int / Real | Real / Int | Real / Real |
|---|---|---|---|---|
| `+ - * /` | Int，整数原规则 | 精确整数参与运算→f32→截断为 Int | 精确整数参与运算→f32，Real | 运算→f32，Real |
| `%` | Int，IDIV 余数 | Int 先舍入 f32，提升 f64 取余→f32，Real | Int 先舍入 f32，提升 f64 取余→f32，Real | 两 f32 提升 f64 取余→f32，Real |

例如 `16777217 - 16777216.0` 返回整数 `1`，反向返回实数 `-1.0`；但 `16777217 % 2.0` 中整数预先舍入，返回实数 `0.0`。余数的预先舍入由 `0x0808e861` / `0x0808e87f CVTSI2SS` 确认，原伪代码直接写 `(double)int` 不准确。

整数 wrapping 与 IDIV 错误规则沿用阶段 2A。实数除零返回 VM `Arithmetic`；NaN 除数不是零。非整数余数走浮点 remainder，不套整数除零检查，零除数可产生 NaN。

实现使用标准库 f64 运算、`mul_add`、相邻 f32 值与残差校正处理有限数中间舍入，避免把整数先缩为 f32，也避免混合乘法在 binary32 中点处发生 binary64 双重舍入。不是完整 x87 模拟器，不改变宿主浮点环境。验证默认 round-to-nearest-even、正常保留 subnormal 的路径；未模拟非默认 x87/MXCSR 控制、硬件异常状态、signaling NaN 异常及算术 NaN payload 的跨平台一致性。

实数负号直接翻转 sign bit；算术产生的 NaN 只验证分类，不以宽泛误差代替有限结果的位级核对。

## 新增指令与边界

| Opcode | 实现范围 |
|---|---|
| `02 30 55 4a` | 整数 AND、OR、XOR、按位取反 |
| `26 4b` | 左移、逻辑右移；计数 0..31 有效，负数或大于 31 返回 0 |
| `0d` | 非 const 数值转换；其他转换源／目标明确未支持 |
| `24` | 支持值的标签判断；mixed `255` 不作为通配符 |
| `2a` | 数值不等 |
| `2d` | 支持值的逻辑非 |
| `3f` | 实数常量，两个字中仅第一字参与值 |

普通存活对象的 type 判断为 0／1，失效对象按已有复制规则变为 undefined。原对象特殊状态位产生 `-1/-2` 的路径没有实现，对象存储也不提供这些状态；不能据此宣称完整对象 type 语义。

准备器继续保持不可变、拒绝未支持指令与原型，错误保留函数、PC 和成员内偏移。没有新增执行真实包的 CLI，所有制品检查仍走原只读工具。

## 验收与下一步

debug/release 各通过 63 项测试：23 项解析测试、40 项 VM 测试。数值新增测试包含左右顺序、混合精度、位级常量、双重舍入反例、NaN／无穷／signed zero／subnormal、转换范围、移位保护、类型判断及预算恢复。原有基础闭环测试保留。

```sh
cargo test --offline --locked
cargo test --offline --locked --release
cargo fmt --check
cargo clippy --offline --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage2b-corpus-validation.json
```

全量回归只解析原始资源，要求 12 包、7,159 个程序、9,933 条继承匹配，保留唯一已知 DBA 异常，检查前后 36 项原始文件哈希一致。验收摘要见 [stage2b-validation.json](stage2b-validation.json)。没有新增依赖或安装运行环境。

容器、引用、共享修改、继承执行、跨对象调用、catch、全局 bootstrap、efun、调度、编译器和旧 ABI 仍未实现。下一批先补容器与引用的静态语义证据；本轮不构成原程序运行对照或完整服务兼容证明。
