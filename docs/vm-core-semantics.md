# VM 基础语义与阶段 2A 实现

阶段 2A 在离线加载器上增加值模型、对象存储、程序准备和基础解释器。当前支持 **28 种操作码的限定类型路径**，不是完整 73 种指令或可启动的服务端。所有执行测试使用内存中人工构造的程序，没有执行原版 ELF、启动脚本或业务包字节码。

## 证据与适用范围

本轮使用 REA MCP，显式选择 Ghidra 12.1.4，对同一 ELF SHA-256 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34` 做静态反编译与汇编核对。保存 [45 条规范 Evidence](vm-core-evidence/rea.json.gz) 和 [可读反编译／汇编](vm-core-evidence/functions.md)；REA 会话已关闭。旧证据复用 [动作表](evidence/actions.csv)、[执行分派和调用](evidence/functions.md)、[二进制加载器](compatibility-evidence/main-rea-functions.md)。

**确认事实**来自原字节、Ghidra 指令及反编译；下文逐项注明。**实现策略**包括安全检查、单调句柄和测试预算。**未覆盖**的类型、路径和调度行为不作兼容结论。Ghidra 的局部变量、签名及终止调用模型存在局限，完整限制保留在附件。

### 关键证据索引

| 原函数 | 地址 | Evidence ID |
|---|---|---|
| `vm_valueType` | `0x0808a13c` | `ev_1faf74eae5f56d8dc203ea31368aeed4b1e7b857c16c42c97b00ec8937436ac8` |
| `vm_pushUndefined` | `0x08090f6c` | `ev_2c1b31fb3efde1f223d478ac5dac3ee8b3fb310407a0de9ce49c577425556416` |
| `vm_copyValue` | `0x0808be24` | `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33` |
| `vm_pushLocalVar` | `0x080914dc` | `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5` |
| `vm_setLocalVar` | `0x0808f6dc` | `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc` |
| `vm_pushObjVar` | `0x08091224` | `ev_03adb0f3a115b83f2497d37852e6c64ffd233cbe06b82b9e57624900a2a01c09` |
| `vm_setObjVar` | `0x0808f7f8` | `ev_85a706f75e7ed097afaab10b0d9474c379d806b7272d872c2ea29584a5f56438` |
| `vm_isZero` | `0x0808cad4` | `ev_174f52005ca1def1dcd28fda64fa608062b04eaeef2672493386d0ad773ab397` |
| `vm_if` | `0x0809746c` | `ev_bf42239829f60589ab5f10d6db7127f1ca6c92abb37e87158ef49e79cd0c4550` |
| `vm_if` 汇编 | `0x0809746c` | `ev_de36227276c0f6a1e0ff4f5ed3490b6f131a438235392dfd545dcaf35a61611f` |
| `vm_callInUserFunBody` | `0x08093eb0` | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` |
| `vm_checkArgNum` | `0x0809521c` | `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9` |
| `vm_enterFunction` | `0x080955e8` | `ev_1782e4cf62d7942391d37161d43a2051ad3aa963b13b7ca61519cc59ae7dad43` |
| `vm_returnFromFun` | `0x08094978` | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
| `vm_finishAction` | `0x08097460` | `ev_5785a2de03047668275b248e9579da7e78ec5779d7dc2cd2c5066806ec787e92` |
| `vm_findOrAddProgramString` | `0x080b1258` | `ev_4954e391cf644acf17ef3044ffce8b179f8579860125c4fdaded74e71e2412b7` |

其他整数、比较、常量和弹栈函数的 Evidence 与地址可在动作表及本轮附件中交叉核对。`vm_addValue` 复用首轮 `ev_42e127d62e334cc26bafbc7565df879a834dbffba8a6a59f495f385632e44e7d`；`vm_callInUserFun` 复用 `ev_1c856bf5b985ed94a90def645280c43cba5e562297da9520994866c02af54409`；`vm_regularProcessor` 复用 `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`。

## 已实现的语义

### 值与程序准备

原标签明确映射为 `Undefined=0`、`Void=1`、`Int=2`、字符串 `Bytes=4`、对象 `Object=10`；不使用 Rust 枚举布局代替标签。字符串值使用标准库 `Rc<[u8]>` 共享不可变字节。本批没有暴露原生引用计数 API，不声称 Rust 的计数或析构时机等价。

`ProgramImage` 仍保存原始字节。运行时字符串池按原加载器处理：`vm_loadBinary` 强制存储字符串的最后一字节为 NUL，`vm_findOrAddProgramString` 按 C 字符串长度建立值。因此池中的终止 NUL 和之后的内容不属于运行时字符串；保留非 UTF-8 字节，不做字符集转换。直接传入运行时的字节值没有额外编码转换。

`prepare_program` 只接受主 ABI 的单 constitution、无继承对象；拒绝全局程序、隐藏函数、默认参数、变长参数、atomic、引用参数及 const 语义。固定参数和返回类型支持本批标签及 mixed `255`，nullable 标志 `8` 允许 undefined。准备结果提供只读访问器，不能绕过验证手工修改其执行字段。

### 栈、变量与调用

原 VM 栈向低地址增长：二元运算的左值位于旧 `sp+1`，右值位于 `sp`，结果覆盖左值并弹出右值。Rust 使用向上增长的 `Vec`，左值为倒数第二项、右值为最后一项。

原调用入口 `bp=sp`，参数 0 位于栈顶。Rust 的入口 `bp=stack.len()`，局部／参数索引 `no` 对应 `bp-1-no`；正索引读取参数，负索引读取已分配的局部／栈位置。`Vm::start` 将按参数顺序传入的数组反向入栈；`0x07` 对已入栈参数不再次反转。

变量赋值 `0x4d` / `0x4e` **不弹栈**。无效局部读取 `0x37` 压入 undefined；无效写入报错。`0x1b` 按函数头的局部数量追加 undefined 槽。对象变量本批基数为 0，不执行继承偏移。

`0x07` 的第二参数 bit7 表示 global、低六位表示参数数量，bit6 在原例程未使用。本批拒绝 global 分支，仅调用当前对象的已准备函数。`0x0f` 在固定参数检查成功后，从已递增的 PC 再加 `nTotalAction`，不是无条件空操作。

返回检查计算栈相对于参数和局部存储的增量：void 必须无返回值；mixed 可以没有值或有一个值；具体类型要求一个匹配值。void 和无值 mixed 生成 `Void`。随后移除参数及局部区域，向调用者保留一个结果。公开入口使用普通用户函数帧，不实现旧 backend 的层 0 返回或对象初始化帧。

### 控制流与整数

原 `regularProcessor @0x0809794d–0x08097951` 先取 `nodes[ip]`，再 `ip++`。跳转以**下一条 PC**为基准：`goto` 增加有符号偏移；`if` 的 bit0 选择真条件跳转，bit1 独立控制弹出条件，其他位不参与这两个判断。

`vm_isZero` 把 undefined、void、整数 0 判断为零；空字符串仍为真。复制已销毁对象值会得到 undefined，本批按对象存储中的句柄有效性处理此路径；这不等同于实现原对象析构时机。

整数加、减、乘、负号使用显式 32 位 wrapping，对应观察到的 `ADD`、`SUB`、`IMUL`、`NEG`。除法及余数按有符号 `IDIV`：商向零截断，余数符号随被除数。除零及 `MIN/-1` 不使 Rust 宿主崩溃，而返回 `Arithmetic`。其中后者原生路径未见保护，x86 指令会产生商溢出异常；Rust 错误是隔离策略，不声称原 VM 返回相同错误。

`finish` 只设置原控制终止位，没有普通返回值搬运，因此表示为 `Completion::ActionFinished`，保留停止现场；不伪造 `Returned(Void)`。

## 操作码支持矩阵

| 类别 | 支持 opcode | 类型／路径限制 |
|---|---|---|
| 常量 | `3b 3c 40 42` | 有符号短整数、metadata 长整数、字符串池、undefined |
| 栈清理 | `32 33` | 仅计算区；`pop_n` 非正数量不操作 |
| 变量 | `37 3d 4d 4e` | 当前帧、当前对象；没有指针／引用和继承 |
| 算术 | `01 19 28 29 2b 51` | 仅整数；不支持字符串加法和实数运算 |
| 比较 | `1c 1e 20 25 27` | 仅整数，结果为整数 0／1 |
| 分支 | `1f 21` | 有符号相对偏移；条件仅本批支持值 |
| 调用 | `07 0f 1b 49` | 固定参数普通本地函数 |
| 结束 | `1d` | 停止整个本次执行，不作普通返回 |

代码中 `SUPPORTED_OPCODES` 是该矩阵的机器可读列表。未支持指令在准备时明确失败，包含函数、PC 及成员内偏移；不通过忽略不可达代码宣称完整支持。

## 接口与实现策略

```rust
let prepared = prepare_program(image)?.with_source("synthetic-test");
let mut objects = ObjectStore::new();
let program = objects.register_program(prepared)?;
let object = objects.create(program, initial_values)?;
let mut vm = Vm::new(objects);
vm.start(object, function_index, arguments)?;
let outcome = vm.run(instruction_budget)?;
```

`ProgramId` / `ObjectId` 为内部单调句柄，不重复使用，也不表示原版线协议或指针值。初始对象变量由调用者显式提供，不运行对象初始化代码；多个对象共享程序但变量隔离。

`RunOutcome` 区分 `Finished(Returned(value))`、`Finished(ActionFinished)` 和 `BudgetExhausted`。预算耗尽保留现场，可以再次 `run`；错误为终止状态，重复 `run` 返回同一错误，新的有效 `start` 可重置。完成后重复 `run` 返回已保存结果。

默认计算栈上限 65,536，调用帧上限 1,024，可通过 `VmLimits` 调整。预算与上限是测试／资源控制策略，不冒充原 timer、协作线程、hook 或调度语义。计算区底部保护防止非法程序弹出参数和局部存储；原弹栈例程没有同样的帧底部检查。

错误保存 source、程序／对象句柄、函数、PC、成员内偏移及类别；错误发生后可能保留已执行的状态变化，没有事务回滚承诺。范围错误、算术异常、类型限制和未支持能力明确区分。

## 验证和剩余工作

27 项 VM 集成测试覆盖本批 28 种指令、整数边界、参数顺序、负局部索引、非弹栈赋值、四种 if 标志、循环与预算恢复、嵌套调用、void/mixed 返回、错误位置、句柄失效、字符串池终止及未支持原型。23 项阶段 1 测试继续保留。debug/release 各通过 50 项测试，格式检查和 Clippy 通过。

```sh
cargo test --offline --locked
cargo test --offline --locked --release
cargo fmt --check
cargo clippy --offline --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage2a-corpus-validation.json
```

全量资源回归只解析原始包，与阶段 1 固定基线比较，不调用 VM。验收记录见 [阶段 2A 检查摘要](stage2a-validation.json)。

实数运算、容器、函数值、指针／引用、继承和跨对象调用、catch、全局 bootstrap、efun、协作调度、编译器及旧认证 ABI 尚未实现。本轮没有原程序运行对照，因此只证明人工测试与静态支持范围，不能推及真实登录、持久化、服务启动或完整行为等价。
