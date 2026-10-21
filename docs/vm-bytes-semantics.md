# 阶段 2AI：Bytes 比较、拼接与下一批实施设计

本批完成七条指令的Bytes定向静态取证、125,747处指令位置复算和[普通函数受限实施规格](stage2ai-evidence/bytes-runtime-design.md)。Rust运行时/API/依赖及75种限定支持集合保持2AH状态，473项Rust测试基线不变；Bytes普通比较/拼接仍未实现。没有执行原ELF、启动脚本、原始init或包内业务字节码。

## 证据与问题状态

使用reverse-engineer-anything技能及REA/Ghidra12.1.4，目标SHA-256为`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。复用15条完整记录，新增12条完整analyze_function记录；[清单](stage2ai-evidence/function-manifest.json)索引全部27条，[新增视图](stage2ai-evidence/functions.md)与[复用视图](stage2ai-evidence/reused-functions.md)仅为阅读副本，原始完整记录保留在压缩包。来源哈希、raw_result、normalized_result、body范围、pcode和限制均保留，不将伪代码称为原始C源码。会话已关闭，未作元数据标注修改。

| 问题 | 状态及主要依据 |
|---|---|
| 01左右tag分派及操作数方向 | 已回答：vm_addValue@0x0808e3b8，`ev_42e127d62e334cc26bafbc7565df879a834dbffba8a6a59f495f385632e44e7d` |
| Bytes全长拼接及C字符串追加 | 已回答：concat@0x0808c7fc `ev_0ad799c15b35d517803d7b03ec4cfdcd869b5b8720539419cd55aaca475fa1eb`；concatWithCStr@0x0808c83c `ev_36ddefb514f4d1eee68076fb77ba6fd169f82925498e54e17c22e4d514ce716e` |
| 扩容、共享与释放 | 选定路径已回答：extend@0x0808c6dc `ev_8586e710ba1b29fd3d65723eea725f94ecb7d991e37716a47b030c78514531c4`；removeShared/free及allocator完整记录另见清单 |
| Int/Real追加格式 | Int已回答、Real外部格式等价未闭合：typeValue@0x0808bd88 `ev_5841b7654391f3ff5cb823d82e9f2e1f96f29fb0f3f21b2a0393e7fc70d7b03b`；typeValueAt@0x0808afc8 `ev_616461794c4cd7f622a095dfb402e863b4ff031428987af556db44e6bada65ba`；intToString@0x080b6304 `ev_78abb8fbaeb569a0b65639715b90654d8ae7cb48efb389df3771b99138ede0b3` |
| 相等、不同头与NUL | 已回答到所选属性分支：isEq@0x0808c908 `ev_c98c01d582d90083a8c48681f7d36bdd274bd35ed834c0ef8fe22e6ac304baf1`；eq/ne wrapper复用记录见清单 |
| 四种排序与混合类型 | 已回答到C ABI边界：四个排序wrapper及strcmp PLT@0x0804b064 `ev_33be0a61bad0f87ae5aadb6450c2cfe45eff8500361f60c1846e8a19b2953a6b`；Buffer边界helper@0x0808cda0 `ev_da709b15fb9a5c8a7024ddc3b4967ee4128d489459cc0950606350f75efc7710` |
| 原生OOM/溢出、libc/locale、任意intern头及完整GC | 部分/范围外：保留未知，不用静态成功分支或Rust资源策略冒充原生运行证明 |

完整结论账本见[findings.json](stage2ai-evidence/findings.json)，区分observation、static-inference、design及各自限制。

## 操作数、拼接和格式

原生栈向下，left=stack[sp+1]、right=stack[sp]；结果写回left并弹right。Rust上升Vec对应left=n-2、right=n-1，不能按形参编号倒置。

| 左值 | 01右值 | 原生分支 |
|---|---|---|
| String/tag4 | String/tag4 | vm_concatString |
| String/tag4 | Int/tag2、Real/tag3 | vm_typeValue后vm_concatStringWithCStr |
| String/tag4 | Undefined/Void、其他tag | 各类型错误；没有任意toString |
| Int/Real | String/tag4 | 数值分支报错，方向不对称 |
| Buffer/tag5 | String/Buffer | 单独Buffer修改路径，本批Rust不开放 |
| Array/Mapping | 同类容器 | 单独容器路径，不属于本批Bytes实现 |

vm_concatString读取两侧`len`，保存旧左长度，调用extend(right.len)，增加结果len，再从右ptr复制right.len+1字节到旧左末端。这是存储长度拼接，包括嵌入NUL及其后内容；额外1字节为原生终止符，不是Rust payload新增的字节。

concatWithCStr以REPNE SCASB测量右侧到首个NUL，复制该长度加终止符；左侧仍使用存储长度。Int经intToString的base10/sign1/upper0分支，以无符号除法输出负号后的绝对量，涵盖i32 MIN。80字节临时区足够所选i32输入；这不是任意value格式化接口。

Real提升为double交给sprintf("%g")。汇编`0x808bcbd`压入`0x82acd87`；[format-bytes.json](stage2ai-evidence/format-bytes.json)按原ELF32 PT_LOAD映射核对文件偏移2,510,215的`25 67 00`，不依赖伪代码字符串猜测。外部libc具体locale、特殊值拼写和舍入运行行为未验证；首批规格排除Real追加，不采用Rust Display近似。

## 相等与排序有意不同

isEq对String先检查同头，再检查存储长度；两个不同且都带shared bit1的头可直接不等，其他情况按len执行有界字节比较。NUL不终止这次比较。String对非String返回false，ne取反。不同shared头的优化依赖原生intern/属性约束，Rust没有该属性，不应把Rc地址不等当作内容不等。

四个排序wrapper对String/String调用strcmp(left.ptr,right.ptr)，随后只将返回值符号转为Int0/1。已核对PLT导入身份，但未分析/运行外部libc实现；首NUL终止、大小写敏感、u8字典序是该C ABI的推导，不是观察到业务执行。无需UTF8转换，也不是locale collation。

因此a\0x与a\0y按全长不相等，排序则前缀相同；a与a\0按全长长度不同，排序也可相等。String排序不自动把数字或其他类型变字符串。Buffer-left可通过compareBufWithBufStr与String比较存储长度，但String-left/Buffer-right仍报错；这条不对称的原生范围只作边界记录。

## 共享、资源与错误限制

extend长度为0时直接返回左头。其他情况ref<2且attrib bit2清零时走reallocate；否则allocate新头/内容、复制左len+1，再递减旧ref，必要时普通free或移除shared表。未对bit2赋予未经证明的语言属性名称。Rust首批以不可变Rc字节保持别名内容，不复制原生20字节头布局或危险裸指针行为。

extend的尺寸由32位ADD/LEA构成old_len+append_len+0x15，相关点未见checked_add式防护。allocator/free/reallocator记录保留初始化、块组、断言和分配边界，不能据此称全部内存管理已恢复。特别是reallocator一个路径在realloc返回NULL后保留旧block，随后仍进入更新size/返回data的路径（`0x80cf9ea`、`0x80cf9f6`至`0x80cf6f7`、`0x80cf719`）。这是静态控制流观察，未触发原生OOM，不推导它可安全扩容。

首批Rust规格采用checked长度、65,536字节拼接结果上限、可控失败原子性和现有错误种类；这些是明确的Rust策略，不是原生限额/失败现场的等价声明。所有新类型/资源错误不增加数值catch来源。原生vm_error非局部退出及完整调度/GC/后端仍属既有未闭合能力。

## 语料、期望表与交付边界

[语料方法](stage2ai-evidence/corpus-method.md)复算7,159程序、101,592函数、6,474,773指令中的125,747个选定位置；按Object/Global及init/函数区分，保存96个确定性导航样例和精确成员内offset。仅检查物理相邻指令，字符串常量邻接不是已证明的左右操作数或运行次数，没有CFG、栈流或动态可达图。

[期望表](stage2ai-evidence/expected-cases.json)给出空值、前缀、独立Rc、大小写、高位字节、NUL、Int边界及混合tag的静态推导；资源限额/Real拒绝单独标为rust-design。它不是参考程序实际运行输出，未来实现须按规格补齐原子性、预算、上下文及清理测试。

G01及阶段2仍未完成。本轮没有开放Bytes运算，当前能力矩阵与缺口清单仍以2AH冻结版本为历史基线；README/路线图新增2AI入口，不覆盖旧哈希。复现、工具测试及交付完整性见[方法](stage2ai-evidence/method.md)与[验证记录](stage2ai-validation.json)。
