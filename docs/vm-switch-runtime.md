# 阶段 2S：受限 switch 运行时

## 一、结果与边界

实现0a `vm_caseCheckNumber`、0b `vm_caseCheckString`、0c `vm_caseDefault`，支持集合增至62种限定操作码；254项Rust测试通过，新增17项合成回归。主语料73种中交集60种，仍缺13种，额外4d/4e仅合成验证。编号比例不是功能完成率，阶段2尚未完成。

不新增公开API、Value变体、Frame状态或Cargo依赖。继续使用现有准备、链接、启动和预算接口。循环、通用catch、全局以及动态能力仍保留既有限制，没有可运行服务端。

证据复用[阶段2R静态报告](vm-control-flow-semantics.md)和[Ghidra原始记录](vm-control-flow-evidence/rea-functions.md)：0a入口0x080974c0（`ev_77292c326eb595efcfe5597edb1f65f303464f58775e3435dac02ee78a721f7f`）、0b入口0x080974fc（`ev_64fb48253351112793bb3171542d795d713e0d2879a2cf900617f8c6f7873a9e`）、0c入口0x08097584（`ev_2dce3eed228dccaaf1d1e67dc9a39c962724b0af9866210b68ecba6102befc83`）。本轮没有新REA查询或原版运行观察。

## 二、准备与执行契约

| 指令 | 准备结果 | 执行行为 |
|---|---|---|
| 0a | metadata两个小端i32：delta、整数literal | 仅Int且32位值相同才弹出选择值并跳转；其他Value失配并保留 |
| 0b | metadata两个小端i32：delta、字符串池索引 | 先取来源池项，再按Bytes长度/原始字节比较；匹配才弹值并跳转 |
| 0c | par16作为delta，par8忽略 | 弹出一个Value并跳转 |

0a/0b的metadata位置保留原生符号扩展表达式`(par8 << 16) | par16`，不将par16先转换u16。准备阶段读取完整8字节，允许合法未对齐、重叠和重复引用。0b检查索引非负、位于实际来源池范围且池项非空；继承函数按其实现来源校验，不能拿根程序池替代。准备器因此接收池项切片，不再仅接收池长度；此改动不影响公开接口。

五类既有/新增分支的原始pc和offset继续保留。三条case的跳转统一按`pc + 1 + delta`验证，拒绝段外目标；合法首/末节点目标不另加推测的CFG规则。即使指令不可达、选择值必然失配，非法metadata/池项/目标也在准备阶段拒绝，属于安全Rust的静态校验策略。

运行时先要求计算区至少一项，不允许吞掉参数或已声明局部值。Reference选择值对三条指令均明确返回UnsupportedFeature，不消费操作数；这是首批边界，不能称完整原生Reference兼容。0a不把Real、Bytes、Void或容器转换成Int。0b区分大小写、长度和非UTF-8字节，不用strcmp或字符串解码；池中的C终止规则沿用已有实现，含额外NUL的输入不会与短池值相等。

匹配或default执行时先验证跳转再弹栈，失配只按顺序继续。没有case表缓存、隐式switch状态或“必须有default”的约束；无default的程序可用现有pop/jump完成。每条指令占一个预算单位，零预算不变，暂停后不重复消费选择值。

Bytes与合法来源池按内容比较；不模拟原生两个不同shared字符串头的指针身份分支，不能宣称任意非规范驻留头等价。普通错误保持原来的粘性失败和帧/栈诊断契约，IP仍在派发前前进。

## 三、继承、调用与任务

运行时通过当前CodeView读取字符串池，支持继承体、07及受限08调用后的来源池切换和返回恢复。外层保留的选择值不会被内层case消费。

初始化段直接包含0a/0b/0c仍拒绝；初始化和构造可调用包含case的普通辅助函数，因为没有新增跨帧switch状态。构造失败继续通过既有回滚清理任务对象；不扩展通用catch。默认参数表达式保持原有线性白名单，不因操作码进入支持集合而开放分支默认表达式。

## 四、回归与验收

17项新增测试位于`tests/vm_core.rs`的Stage2S区域，覆盖：

- Int边界与不转换其他类型，字符串空串/大小写/长度/非UTF-8/NUL。
- 连续数值和字符串case、default、无default与嵌套选择值。
- 命中弹一项、失配保留、计算区下溢和Reference拒绝，零/单步预算恢复。
- 元数据负地址、截断、符号扩展、重叠/未对齐/共享引用、边界跳转及空/非法池项。
- 继承来源池与错误位置、08目标池和调用者case恢复、初始化直接拒绝及辅助函数执行、构造失败清理、默认表达式白名单。

254项Rust测试、格式、Clippy、全量只读语料比较和18项矩阵规则测试通过。当前[逐操作码矩阵](stage2s-evidence/matrix.md)与[机器验证记录](stage2s-validation.json)独立保存。静态语料比较仍覆盖12包、9,527成员、7,159程序；唯一已知错误为`dba/pack_data/etc.pak/etc`的LZSS长度不符。它验证加载结构，不执行业务代码，也不证明业务路径覆盖。

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage2s-corpus-validation.json
python3 docs/stage2s-evidence/test_verify_completion.py
python3 docs/stage2s-evidence/verify_completion.py
python3 docs/stage2s-evidence/verify_validation.py
```

参考ELF、PAK、对象、脚本和历史分析文件保持原样，未执行原始程序或字节码。2P/2Q/2R的来源哈希与快照不更新；旧脚本针对旧源码/入口文档的重算可能按预期报告过期，当前源码使用2S独立验证。下一步仍需先完善隐藏Cell载荷，才能实施整数/数组循环；mapping继续等待原生哈希顺序兼容。
