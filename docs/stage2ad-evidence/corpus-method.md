# 06 专项语料统计

输入固定为compatibility-evidence下的programs.json.gz、compatibility-summary.json、efuns.csv和efun-usage.csv，四个文件的SHA-256记入corpus-summary.json。该语料来自既有独立静态提取；本分析器不执行程序，也不修改原始包。另以Rust只读解析器的全包结构比较复核输入基线。

扫描每个程序的init、functions和hidden全部节点。每个节点验证signed i16 par16、signed i8 par8、动作号和节点布局；按opcode=6记录调用。argc保留signed par8，不掩码；另保留unsigned原始字节以显示高位。槽号按当前Magic559的559槽命名空间校验，再映射固定CSV，分别记录负槽、越界及未知注册项。完整注册表验证唯一且恰为0..558；测试可向audit注入缺项以验证未知槽分支。

输出：

- corpus-summary.json：程序/函数/指令总量、06计数、槽/argc/服务/代码段/模式及组合直方图；本语料无异常，仍保留异常分类机制。
- efun-inventory.json：全部559项原CSV字段及静态调用量，包括247个未出现槽。不从返回类型列推导参数类型。
- corpus-occurrences.jsonl.gz：143,520条完整调用位置，每行JSON包含source、成员哈希、代码种类、函数及编号、段偏移、nodes_offset、PC、成员指令偏移、slot和原始par8。未保存任意运行值或凭据。
- corpus-examples.json：每个mode/segment_kind/signed argc/slot状态组合首个样例及前后最多2条节点；例子选择按输入确定，不作为所有槽的覆盖声明。

位置公式为 `member_node_offset = nodes_offset + 4*PC`。成员偏移基于解压后.o，不能用于ELF地址或PAK压缩数据寻址。静态服务分组来自source首段，不在Rust运行时引入按路径选择注册表的行为。

独立交叉核对：程序/普通及隐藏函数/指令总数和06总数等于compatibility-summary；逐efun调用总数、所在程序数、五服务调用量等于efun-usage.csv。默认重新计算后逐字节比较本目录产物；只有显式--write才重建。Gzip使用mtime=0、排序JSON键及固定换行，当前Python/zlib环境内确定；跨Python/zlib版本压缩字节可能不同，应核对解压内容后有意识地重建并更新交付哈希。

```sh
python3 -B docs/stage2ad-evidence/analyze_corpus.py
python3 -B -m unittest discover -s docs/stage2ad-evidence -p 'test_analyze_corpus.py'
# 有意更新本阶段统计时：
python3 -B docs/stage2ad-evidence/analyze_corpus.py --write
```

单元测试用合成程序覆盖0/多参数、signed负值、64/127高位、槽边界、未知槽、Global/init/hidden、偏移、重复源、错误profile、损坏节点、基线不一致及完整位置压缩往返。验证的是分析器，不是06运行时。
