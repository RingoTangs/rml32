# 阶段 2AK：受限 Bytes 复合赋值

本轮实现 `2f` 内部编号 2（`+=`）的 Bytes+Bytes/Int 分支。沿用既有准备入口、稳定引用槽及公开 API，无新依赖；限定支持集合仍为 75 种。此增量缩小 G01，不关闭阶段 2。

## 静态依据与实现边界

复用 [2F 原生证据](vm-numeric-update-evidence/functions.md)：`vm_opEqValue @0x0808eb68` 的 switch 编号 2 调用 `vm_addValue`，计算结果复制回目标并留作表达式结果。汇编 Evidence 为 `ev_354a97d72f74e25fe049521f079d6411eef25ea6be56c075ef7da6aab3dbeb4e`；反编译及调用引用保存在同目录完整证据包。拼接类型、完整长度与整数转换复用 [2AI 语义](vm-bytes-semantics.md) 及 [2AJ 实现](vm-bytes-runtime.md)。本轮只复用已保存证据，没有新增反编译或原版运行观测。

由上述原生分派推导：对受限 Bytes 加法，复合赋值可复用同一计算内核。Rust 在引用解析后选择该路径，不向 numeric::binary 添加字符串支持。目标可为当前已支持的可写引用，包括对象、局部、全局及容器成员；不新增引用种类。新增路径仍拒绝 initialization/construction 任务，包括普通 helper 帧。

左值须为 Bytes，右值只接受普通 Bytes/Int；完整 payload 参与拼接，嵌入 NUL 和非 UTF-8 均保留。整数追加有符号十进制 ASCII。合法范围内追加空 Bytes 可共享原 Rc，其他成功结果分配独立 payload，不改写旧别名。结果最长 65,536 字节，属于 Rust 资源策略。

## 写回与错误

先检查计算区、引用及只读状态，解析目标，计算结果，分配表达式结果 Cell，再提交目标替换并消费 Ref/RHS。分配或借用失败时不修改目标和操作数。与既有 2f 保持一致，内部 PC 在分派前递增；公开错误仍定位原指令，失败为 sticky 终态。此点与普通 Bytes 二元运算的提交后 PC 递增不同。

长度/分配失败为 Limit，未开放类型组合为 UnsupportedType，生命周期及引用右值/隐藏控制槽为 UnsupportedFeature。新增 Bytes 错误不创建数值 RuntimeFaultSite。Real 追加、反向 Int+Bytes、其他 Bytes 复合运算、Buffer、Bytes 成员写入、一般生命周期与服务启动仍未开放。

## 验证

新增 4 项 VM 集成测试和 1 项私有单元测试。覆盖完整字节内容、i32 极值、空追加身份共享、旧值别名、对象写回与表达式结果、共享 Array/Mapping 成员、长度上下界、失败现场/诊断/sticky、零预算及单步写回、结果 Cell 分配失败、目标借用失败和生命周期任务隔离。原有数值更新负例只移除现在合法的 Bytes+=Int 组合，其余选择器继续拒绝。

487 项 Rust 测试（57 库单元、23 解析、407 VM 集成）通过 debug/release；fmt、clippy -D warnings 通过。测试全部使用合成程序，未执行原始 ELF 或业务包。未单独重跑离线包语料比较器，因为加载器和准备器未改变；没有原版动态对照，未证明原生 OOM/GC 等价。

复现：`cargo test --offline --locked`、`cargo test --offline --locked --release`、`cargo fmt --check`、`cargo clippy --offline --locked --all-targets -- -D warnings`。
