# 生命周期语料统计方法

## 输入与执行边界

`verify_lifecycle.py` 读取兼容性阶段保存的 `programs.json.gz`、`members.json.gz`、原始输入清单和阶段 2K 继承图。它校验清单中的 36 个原始制品 SHA-256，按成员记录解压 PAK，并校验全部 7,159 个程序的原始内容 SHA-256。只执行 Python 解析器；不启动原始 ELF、脚本、初始化段或业务函数。

```sh
python3 docs/vm-object-lifecycle-evidence/test_verify_lifecycle.py
python3 docs/vm-object-lifecycle-evidence/verify_lifecycle.py
```

默认模式重新计算并逐字节比较 `details.json.gz` 和 `summary.json`，不会修改输出。只有显式 `--write` 会重新生成这两个文件。gzip 使用固定时间戳；摘要记录输入文件和生成脚本哈希。

## 原始名字与参数核对

对 101,592 个函数记录，使用已有证据保存的成员内函数偏移，从原始成员重新读取 64 字节头、原始长度字符串、参数与局部变量描述符，确认代码段边界与显示名字一致。生命周期匹配使用原始名字在首个 NUL 之前的字节，精确匹配 ASCII `create`、`destruct`，不使用可能有损的 UTF-8/GB18030 显示文字匹配。详细记录保留完整名字字节、头和描述符；参数默认指令数按有符号 i16 读取。

统计只包括源程序本地记录，不复制继承体，不执行空声明别名绑定，不推断最终生命周期分派。序列化属性也不包含原生加载后追加的属性位。`attrib=6` 只按原值统计，不能据此声称 Rust 已支持这些函数。

## 初始化与继承排列

按每个源程序的 `init.nodes` 统计操作码及 metadata 字节数。`init_minimal_2e_48_1d` 仅表示操作码序列恰为 `[0x2e,0x48,0x1d]`，不检查操作数，也不表示初始化无副作用或可以跳过。`init_other` 为其补集。

直接复用阶段 2K 离线重建的 `init_orders`，验证每个 object 排列覆盖 `0..constitution_count` 且自身 0 最后；global 不应存在此排列。这只是结构一致性验证，真正执行顺序仍须消费者函数的 Ghidra 证据支持。详细输出为每个程序保存排列、节点数、操作码直方图和 metadata 字节数，可与原有继承图关联选择多层或多父样例。

## 输出与测试

- `summary.json`：全量计数、输入哈希和限制。
- `details.json.gz`：`init_segments`（7,159 条）和 `lifecycle_records`（6,216 条），包含程序来源、成员内偏移及原始属性。
- 19 项合成元数据测试覆盖精确名字、首个 NUL、非 ASCII/大小写/前缀排除、声明与实现、object/global 排列、非法排列、初始化结构、重复记录去重统计、参数属性与负默认值、原始读取边界。

这些统计是静态结构证据，不是构造/析构已被调用的证据，也不是运行兼容性或操作码覆盖证明。

## 原生证据保护核对

`python3 docs/vm-object-lifecycle-evidence/verify_evidence.py` 只读检查54条本轮与12条复用记录的目标SHA和Ghidra来源、复用记录与原始包逐项相等、3个来源文件哈希及报告中的Evidence引用。短类型名查询未命中的结果也被保留，不把未定位当作类型不存在。
