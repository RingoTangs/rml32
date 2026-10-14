# 阶段 2K 继承语料核验方法

## 输入与复现

`verify_inheritance.py` 仅使用 Python 标准库读取既有 `programs.json.gz`、`inheritance.csv` 和 `main-rea.json.gz`，不执行原程序或业务字节码。所有输入及脚本本身的 SHA-256 记录于 `summary.json` 和 `graph.json.gz`。默认命令只读重算并逐字节核验确定性 gzip（mtime=0）；`--write` 仅重建本目录两个新报告。

```sh
python3 docs/vm-inheritance-evidence/test_verify_inheritance.py
python3 docs/vm-inheritance-evidence/verify_inheritance.py
```

19 项合成元数据测试涵盖叶节点、多层重定位、缺失、歧义、循环、直接重复父、菱形、计数不符、区间越界、兄弟重叠、signed16 加法溢出、负偏移、自身 constitution 覆盖、同名非空实现保留不同槽及同名候选不擅自绑定空声明。合成数据不包含需执行的字节码。

## 图与名称解析

以完整成员 source 为节点标识。离线目录策略沿用既有 `summarize_programs.py`：同一个 `.pak/` 前缀内，去掉继承名开头 `/`，末尾 `.c` 改为 `.o` 后精确匹配。重复 source 为歧义，找不到为缺失；不做大小写折叠或跨包搜索。这是离线目录策略，**不是**原生 `vm_lookupObjectByCStr` 运行时命名空间的证明。

深度为已解析图中最长父边数，叶为 0；全语料没有缺失或循环，因此其深度完整。循环检测结果为 DFS 遇到的回边环列表，并非一般图强连通分量数量。祖先路径计数保留重复路径，任一祖先出现两次即归为菱形或重复祖先候选，不把多继承直接等同于菱形。

## 六字段与容量

原生 `vm_loadBinaryInherit` @ `0x080a2b58`，Evidence ID `ev_c41c22c1afd1192242c38bb59ffa5cc3402d157828307978e735721a4489e3d0` 确认六 signed16 字段依次是：constitution 基址、函数基址、变量基址、父 constitution 容量、父函数容量、父变量容量。核验父实际三容量等于声明值，分别计算半开区间；函数/变量容量检查对应 `0x080a2ceb` / `0x080a2d0e`。

constitution 以 12 字节为步长复制，索引 0 是自身。复制到 `baseConstitution+j`；嵌套函数/变量基址分别累加，原生 `0x080a2d73` / `0x080a2d7f` 使用 16 位 ADD。脚本用无溢出整数检查 signed16 范围，并重建每个程序的 constitution 槽覆盖。

constitution 区间完整上界及保留自身槽的检查属于拟议 Rust 安全策略；不能声称原生函数包含完全相同检查。兄弟区间重叠只记录事实，**不自动判非法**；函数覆盖选择规则需另有原生证据。

## 输出及边界

`graph.json.gz` 保留每边原始六字段、解析状态、三个区间、每程序重定位 constitution 表，以及所有继承 DLT 原始 20 字节、成员内记录偏移和索引。DLT 的 `attrib & 0xc0000` 仅用于选取继承分类记录，`raw[8:16]` 仅按原始字节统计，未据此推测覆盖语义。记录偏移是解压后程序成员内偏移，不是 ELF 地址。

报告不执行或模拟函数体，不根据邻近字符串猜测调用目标；构造 constitution 表并不证明函数表覆盖正确，也不等于业务运行覆盖率。所有既有 `inheritance.csv` 行的状态与声明/父实际容量核验结果交叉比较。

## 函数表与调用的保守边界

`vm_finalizeFun` @ `0x08095794`（`ev_ac11d24ef0dd2c2398a02826fd57a977d4381fa3668add2ab2c2553424e05abd`）确认本地函数按 DLT index 填表，constitution 为 0。`vm_gatherProgramFunctionTable` @ `0x080a31bc`（`ev_cbeb0564e51d40f356850c8ce433d229a0063dc15122c784bcac9998341ff27d`）确认父条目按 function 基址复制、constitution 累加并共享函数体。脚本只重建 **prealias** 表：父空声明未按名字匹配，实现未知继续传递，不把它们当最终目标。非空实现不被原生第二阶段声明补齐重写。语料 hidden 数量为 0；隐藏函数注册不在本模型范围。

仅统计每个 root 程序**本地 functions** 中的原始 opcode 07 节点，caller constitution 固定为 0；全局 bit7 调用保留未知，局部目标为 signed16(a+0)，只有 prealias 非空 body 才解析。报告未重复枚举继承 body 的执行上下文，也未计入 init 节点，因此这个计数不应与全语料 opcode 07 总数直接比较。图中每条记录保留 root、body、cons、函数索引、pc 和成员内字节偏移。

## 初始化顺序

只构造静态序列：各父的第 j 项写到 `child.initOrder[baseConstitution+j-1]`，值为 `parent.initOrder[j]+baseConstitution`；最后位置写自身 0。7154 个 object 程序均为 constitution 索引排列且自身最后；5 个 global 没有该序列。此处未执行初始化，也不代表初始化业务正确。初始化顺序规则来自 `vm_loadBinaryInherit` 和 `vm_gatherProgramFunctionTable` 的实际反编译，原始证据见同目录索引。

## 紧凑存储

压缩图采用无损字典及列式结构，顶层 `columns` 定义各数组列名，`program_dictionary` 将程序 ID 映射为完整 source，`body_dictionary` 保存函数来源、索引、记录偏移、是否有代码及显示名称。`prealias_tables` 的键为 root 程序 ID，每行 `[slot, body_id, constitution]`；调用记录通过同一 body ID 引用目标，避免重复存储共享函数体信息。调用者 body 程序等于 root、constitution 为 0 等常量保存在 `call_invariants`。统计计算仍使用可读记录，只有最终序列化压缩为列数组；未删除证据数据。

`vm_buildProgramHead` 的 `ev_aa12d41f4394aa209415defcd72e5fc2cfbdee8acf9ff95fe8d33af97ed6b264` 确认自身 constitution 0 及初始基址 0。`vm_createProgram` 的 `ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d` 显示加载后 create/destruct 还会更新函数属性；本报告未模拟这些运行时属性变化及最终 alias 绑定，因此 prealias 表不是完整原生运行结果。非空 body 的身份映射不受这些属性更新影响。上述新 Ghidra 记录全部保存在本目录 `rea.json.gz`，该文件亦纳入输入哈希。
