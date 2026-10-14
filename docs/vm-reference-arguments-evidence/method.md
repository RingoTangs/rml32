# 引用参数原型及静态调用统计方法

## 输入与复核

本报告直接读取既有 `docs/compatibility-evidence/programs.json.gz`，不解包原始 PAK、不执行字节码或参考 ELF。`prototypes.json.gz` 保存全部输入 SHA-256，以及本目录验证脚本自身的 SHA-256。程序记录中的 `member_sha256` 是原解压成员的内容摘要，不是 JSON 对象摘要。`summary.json` 提供总数、类型组合、服务/包分布及调用分类的小型摘要，并记录完整 gzip 报告的 SHA-256 和压缩前后大小。

从仓库根目录执行：

```sh
python3 docs/vm-reference-arguments-evidence/verify_prototypes.py
```

默认仅重新计算、比较完整报告及摘要并输出验证结果，不写任何文件。显式 `--write` 只重建同目录 `prototypes.json.gz` 和 `summary.json`。完整 JSON 按键排序，以 UTF-8 编码并保留末尾换行，再用 gzip level 9、mtime=0 确定性压缩；默认复核同时比较压缩字节、解压语义及摘要。脚本仅使用 Python 标准库；生成及复核不会导入可能写入旧证据的提取脚本。统计顺序沿用输入记录。

总数分别与既有 `compatibility-summary.json` 及固定基线比较：7,159 个程序、101,592 个普通函数、0 个隐藏函数、108,751 个代码段、6,474,773 个节点。调用类操作码数量另与既有操作码直方图逐项断言一致。

## 原型字段与计数口径

每个普通/隐藏函数的 `arguments` 均计为形参，不把 `locals` 计入。参数 reference 按 `flag & 1` 识别；保留完整 flag/type/attrib 三元组，不把不同标志组合合并。nullable、const 列仅展示 attrib 的 bit3（8）及 bit1（2）是否置位；实际生效路径需结合 Ghidra，不因字段名推断引用参数也执行同样检查。

`parameter_combinations` 保存全部组合、次数及最多三个完整原型样本；按服务与包另有完整组合直方图。`function_shapes` 保留函数 attrib、min/max 参数数及引用形参数量组合。`reference_prototypes` 保存全部 4,395 个含引用参数的函数原型，并保留 source、成员摘要、函数 index/offset、参数 index 和所有参数字段。

`reference_function_shapes` 进一步按固定参数数量（min=max）、函数 attrib 低两位、首节点 opcode、参数 index 是否依次为 0..N-1 分类；空代码段的首 opcode 为 null。`descriptor_diagnostics` 检查全部形参的 flag 未知位（除 bit0）及 attrib 未知位（除 bit1/bit3），并统计首节点 `0x0f`。这些是原始结构统计，不将 attrib 低两位自动解释成已确认的变参/atomic 行为。

位置 `prototype.offset` 是**解压后成员内的函数记录起始偏移**；参数 index 是逻辑形参编号，不是成员字节偏移。调用样本另含 `code_offset`、pc 和 `member_node_offset=nodes_offset+4*pc`。这些均不是 ELF 地址或压缩包偏移。

service 为 source 路径首段，package 为 `.pak/` 之前的路径加 `.pak`。相同函数被不同服务包分发时分别累计；不是唯一源码函数数。该基线包含五个具有可解码程序的服务库包，不意味着仓库其他 PAK 不存在。

## 本地调用的保守解析

解码依据：`vm_callInUserFun @ 0x08094080`，Evidence `ev_1c856bf5b985ed94a90def645280c43cba5e562297da9520994866c02af54409`，原证据位于 `docs/evidence/magic_Linux32-rea.json.gz`，可读结果见 `docs/evidence/functions.md`。

```c
global = (b >> 7) & 1;
argc = b & 0x3f;  // bit6 不影响此函数的 argc
if (!global)
    actual_index = (short)(a + current_constitution->baseFunOffset);
target = current_object->program->pFunctions[actual_index].pFun;
```

仅当程序为 object、单 constitution、无继承、a 在声明函数范围内、该 index 具有唯一序列化函数定义时，报告才解析到该定义。绝不按函数名匹配，也不将多 constitution 的 a 直接当作当前定义索引。无法确定执行上下文的 global 程序、运行时 global 绑定、继承偏移及缺失/重复定义均保留独立原因。当前语料实际只有“明确解析”及“constitution 基址未知”两类 nonglobal 结果。

每个明确目标按 source/index 聚合，保存完整目标原型、argc 直方图、原始 min/max 范围关系及最多三个调用样本。above_max/below_min **不是非法调用判决**：变参、默认参数和运行时参数检查须另行分析。没有进行控制流、栈来源或类型推断；指向引用原型不证明实参确为引用。

其他调用保持独立：`05` 函数指针为动态调用，`08` 为跨对象调用，`06` 为 efun，`09` 为 OS 调用。对这些调用只保存原始操作数和分类，不复用 `07` 的 argc 编码。所有分类保留服务/包分布与样本。

## 结果与边界

| 项目 | 数量 |
| --- | ---: |
| 全部形参 | 175,100 |
| 含引用参数的程序 | 1,710 |
| 含引用参数的函数 | 4,395 |
| 引用形参 | 4,886 |
| `07` 明确本地调用 | 10,947 |
| `07` 本地 constitution 基址未知 | 89,945 |
| `07` global 绑定未解析 | 25,574 |
| `05` 动态函数指针 | 3,089 |
| `08` 跨对象调用 | 366,483 |
| `06` efun | 143,520 |
| `09` OS 调用 | 0 |

引用形参全部为 flag=1、attrib=0，类型分布为 String 1,849、mixed 1,457、Array 1,002、Int 298、Mapping 235、Object 27、Buffer 14、Real 4。全部形参中另有 4 个非引用 Object 使用 attrib=8，1 个非引用 Array 使用 attrib=2；**本语料没有引用与 nullable/const 组合实例**，这不是 VM 不支持该组合的证明。

明确解析的本地调用涉及 5,132 个目标，其中 385 个目标含引用形参，742 条调用指向这些原型。原始参数范围比较得到 within_bounds 10,339、above_max 603、below_min 5。所有调用类结果累计完整，但样本有限，未输出每个调用位置。

4,395 个引用函数中，3,140 个 min=max，1,255 个 min!=max；4,340 个首节点为 `0x0f`，55 个代码段为空。固定参数、attrib低两位为0且首节点为 `0x0f` 的函数有3,050个。其余形状分别为：非固定/低两位0/首`0x0f` 1,223个，非固定/低两位2/首`0x0f` 32个，固定/低两位2/首`0x0f` 35个，固定/低两位0/空代码55个。全部101,592个函数的形参index均为顺序0..N-1，87,774个函数首节点为`0x0f`；形参flag和attrib均未出现上述掩码以外的位。空代码不能据此视为可运行函数或隐式参数检查。

以上均为序列化节点及原型的静态出现次数；不表示运行频率、可达性、传参正确性或完整服务端执行覆盖。报告不从相邻节点断言引用来源，不尝试把 global、动态或跨对象目标猜成当前对象内同名函数。
