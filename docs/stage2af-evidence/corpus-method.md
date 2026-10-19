# 05/17静态语料统计与复算

输入固定为compatibility-evidence/programs.json.gz、compatibility-summary.json与efuns.csv，三个文件哈希写入corpus-summary.json。扫描全部init、普通functions和hidden，核对7,159程序、101,592普通函数、6,474,773指令及05/17各自计数；不是只挑含目标操作码的程序。另用Rust只读全包解析比较核对原始输入结构。

每个节点验证signed i16 par16、signed i8 par8、操作码、段起点和nodes_offset。05保留mayNull原始par16和含目标的signed argc，不把par16当作函数槽，不猜测实际目标。17按已确认的bit7/bit6/low6分组，仍保留原始signed/unsigned字节。

17的global数值按固定559槽、同服务唯一Global单元的function_base/max_functions分类；不足以判定完整运行时表项类型。无唯一Global单元时不任选一个。Object槽只标object_relative，并提供同映像function.index对应的声明候选；不把它当成继承重定位后的必然运行目标。完整继承调用图与动态目标分析留给后续，数字区间统计不证明可达。

输出：

- corpus-summary.json：全量总数和opcode、作用域/代码段、服务、原始操作数、05参数和17创建字段直方图。
- corpus-occurrences.jsonl.gz：35,703行完整位置，含成员SHA、函数/段、nodes_offset、PC、原始操作数和分类。`member_node_offset=nodes_offset+4*PC`为解压后成员内偏移，不能用于ELF地址或PAK压缩位置。
- corpus-examples.json：各作用域/段/字段分类的首个样例及前后最多2条节点，当前68个；相邻节点不是数据流证明。
- target-inventory.json：17的global/bit6/原始槽组合及次数，当前470种；只有efun数字区间附CSV名称，不输出05动态目标列表。

默认重算后逐字节比较本阶段产物，只有显式--write重建；gzip使用mtime=0和稳定JSON排序。跨Python/zlib压缩字节差异应先核对解压内容，再有意识地重建交付哈希，不能忽略不一致。

```sh
python3 -B docs/stage2af-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2af-evidence -p 'test_analyze_corpus.py'
# 有意更新本阶段统计时：
python3 -B docs/stage2af-evidence/analyze_corpus.py --write
```

9项合成统计测试覆盖signed边界、高位/低6位、05目标数量与非槽含义、global区间/缺失/歧义、init/hidden/member偏移、Object声明候选、registry缺失/重复、损坏输入、独立基线计数及确定性压缩往返。它们验证统计器，不是05/17运行时测试。
