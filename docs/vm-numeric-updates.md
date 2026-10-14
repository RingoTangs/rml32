# 阶段 2F：数值自增减与复合赋值

## 总体结果

新增 `0x22` 自增减和 `0x2f` 数值复合赋值，支持集合从 50 增至 **52 种限定操作码**。复用阶段 2E 的稳定值槽、引用解析、对象隔离、资源上限和错误位置。公共 API、快照接口及依赖保持不变，未引入 unsafe。

本轮使用 REA MCP / Ghidra 12.1.4 分析 `magic_Linux32`，目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。原始 ELF、脚本、PAK 和业务字节码未执行或修改。执行测试全部使用内存合成程序。

## 原版静态证据

本轮保留 6 条完整记录，含 2 个汇编、2 个反编译和 2 个调用引用查询；反编译 ID 与阶段 2D 相同。完整 [Evidence 索引](vm-numeric-update-evidence/index.md)、[可读函数证据](vm-numeric-update-evidence/functions.md)及 [REA 证据包](vm-numeric-update-evidence/rea.json.gz)保留目标身份、分析配置和局限。以下结论是静态观察与代码推导，不是原版运行观测。

### 自增减：vm_incValue / 0x0808dea4

- 汇编 Evidence：`ev_6038bf11ed6a58f8ba545a48140a3d29d7c02fd3ebdb8b798f1cd24275cb3488`。
- `0x0808deba` 从栈顶 payload 取目标槽，`0x0808dec4` 读取有符号 int 增量参数。
- `0x0808df3c` 对 Int 使用 32 位 ADD，不做溢出检查；环绕由指令位宽确认。
- `0x0808df44/48/4e` 为 `FLD float / FIADD dword / FSTP float`：保留整数增量的精确值，运算后存为 binary32。
- `0x0808defe` 调用 `vm_unrefValue`，`0x0808df11–29` 清除栈顶并增加 vm_sp；因此不留下表达式结果。

增量来自单参数 action 的有符号 par16；实现接受 `-32768..32767` 的完整范围，包括 0。语料只有 ±1，不把有限使用情况误作原版参数限制。

### 复合赋值：vm_opEqValue / 0x0808eb68

- 汇编 Evidence：`ev_354a97d72f74e25fe049521f079d6411eef25ea6be56c075ef7da6aab3dbeb4e`。
- 原版栈顶为右值，下一槽为目标引用；`0x0808eb8e` 从 `ECX+0x20` 取其目标槽地址，`0x0808ebb4` 复制目标当前值参与计算。
- `0x0808ebbc–c1` 按内部操作编号分派；反编译 switch 和实际调用引用共同确认下表。内部编号不是 VM 操作码。
- 内联 Int×Int 在 `0x0808eda4` 使用 IMUL。Real×Int 在 `0x0808edb5–bb` 使用 FLD/FIMUL/FSTP；Int×Real 在 `0x0808edc0–cc` 先 FIMUL、存 binary32，再调用 `_vm_ftol`。这与阶段 2B 内核的转换顺序一致，不能把 Int 预先转换为 binary32，也不能在存回 binary32 之前截断。
- 内联异或 `0x0808ed55–58` 使用整数 XOR，左右两侧均检查 Int。
- `0x0808ebf3/ebfc/ec04` 依次清理目标、复制结果及释放引用；`0x0808ec09–41` 将结果移入原引用位置并消费一槽。

| 内部编号 | 运算 | 复用的普通数值 VM 操作码 | 原版实现 |
|---:|---|---|---|
| 2 | `+=` | `01` | 调用 vm_addValue |
| 3 | `-=` | `51` | 调用 vm_subValue |
| 4 | `*=` | `29` | 内联，汇编核对转换与乘法 |
| 5 | `/=` | `19` | 调用 vm_divValue |
| 6 | `&=` | `02` | 调用 vm_andValue |
| 7 | `\|=` | `30` | 调用 vm_orValue |
| 8 | `^=` | `55` | 内联整数 XOR |
| 9 | `>>=` | `4b` | 调用 vm_rshValue |
| 10 | `<<=` | `26` | 调用 vm_lshValue |
| 11 | `%=` | `28` | 调用 vm_modValue |

这些调用在 `procedure_references` 中保留准确 call 地址和目标名称。已有 [阶段 2B 数值语义](vm-numeric-semantics.md)说明被调用内核的有限兼容范围。尤其是 x87 精度控制、异常状态和任意 NaN payload 不在本次完整模拟范围内。

## Rust 实现与错误策略

整理后的 C 风格伪代码如下，表达实现顺序，不是恢复的原始源码：

```c
void numeric_update(int instruction, int operand) {
    // Stack bottom→top: increment [Ref]; compound [Ref, rhs].
    check_evaluation_stack(instruction == INC ? 1 : 2);
    Reference ref = require_reference_at_target_position();
    reject_readonly(ref);
    Cell *target = resolve_valid_target(ref);
    Value old = snapshot(target);
    Value result = instruction == INC
        ? numeric_increment(old, operand)
        : numeric_compound(operand, old, require_plain_rhs());
    StackEntry *prepared_result = instruction == COMPOUND
        ? allocate_result_before_commit(result) : NULL;
    replace_cell(target, result);              // commit after all checks
    consume_operands();
    if (prepared_result) push(prepared_result);
}
```

`0x22` 只接受 Int/Real。Int 使用 wrapping_add；Real 复用已确认的宽运算和 binary32 舍入适配。`0x2f` 在准备阶段限定编号 2–11，算术只接受 Int/Real，位运算和移位只接受 Int。非法编号报 UnsupportedFeature，非法目标/操作数类型报 UnsupportedType，引用右值等逃逸路径报 UnsupportedFeature。

加减乘除沿用左侧类型规则；混合余数沿用原版数值内核，可能将 Int 目标替换为 Real。移位在次数小于 0 或大于等于 32 时返回 0，右移为逻辑右移。整数除零、MIN/-1 除法或余数报告 Arithmetic；实数除零报告 Arithmetic；浮点取余零可能产生 NaN。Int/Real 转换和边界继续沿用阶段 2B 策略。

所有计算和结果槽分配均先于写回；失败时目标和操作数栈保持原样，错误保留 source/program/object/function/pc/offset。IP 仍按统一解释器规则预先增加，失败状态为终态，不允许继续执行导致重复提交。前一条 `44` 已成功插入的 Undefined 映射键不因后续数值更新错误回滚。

这与原版部分错误路径可能已改动临时栈的行为不同，是 Rust 的明确受控错误策略。只读引用与失效帧/对象在写前拒绝；完整 const 原型仍未开放。没有放宽引用参数、引用返回、字符串/容器运算、循环图及全局引用限制。

## 使用与测试

以下是内存构造指令序列示意，不用于执行实际业务包：

```text
3e 0      # 当前对象变量 0 的引用
22 -1     # 将变量减 1，消费引用，不留下结果

3e 0      # 目标引用
3b 3      # 右值 Int(3)
2f 4      # 内部编号 4 = 乘法；写回并留下结果
```

阶段 2E 的 97 项测试保持通过；本轮新增 12 项合成 VM 测试及 1 项私有引用保护测试，共 **110 项**（4 个单元、23 个解析、83 个 VM 集成测试）。覆盖完整有符号增量、零增量、消耗/结果形状、十种编号、混合精度及双重舍入反例、NaN/无穷/负零/subnormal、除法异常、位移边界、共享成员、局部槽、临时容器、对象隔离、提前插入、只读/失效引用、预算恢复、错误位置和失败原子性。

```sh
cargo test --offline --locked
cargo test --offline --locked --release
cargo fmt --check
cargo clippy --offline --locked --all-targets -- -D warnings
cargo build --offline --locked
python3 tests/compare_corpus.py --report target/stage2f-corpus-validation.json
```

结果见 [stage2f-validation.json](stage2f-validation.json)。静态比较继续覆盖 12 个包、7,159 个程序和 9,933 条继承匹配，保留 DBA 已知解压异常；检查 36 个原始制品及所有旧证据不变。没有原版运行对照，合成测试与结构一致不能证明完整业务执行等价。

下一阶段可以围绕引用参数与函数边界展开静态分析，先确认类型、别名和生命周期，再决定受限实现范围；不应直接放开引用逃逸或循环容器。
