# 阶段 2AL：Mapping 顺序与遍历静态取证

本轮补齐 Int/Bytes 键的建表、哈希、桶链、成功扩容及遍历修改规则，交付[下一批 Rust 规格](stage2al-evidence/mapping-runtime-design.md)。运行时保持 2AK：487 项 Rust 测试、75 种限定操作码、73 种主语料交集；Mapping997 仍返回 UnsupportedFeature，G05 与阶段 2 **未完成**。

没有执行参考 ELF、启动脚本或业务字节码。REA `open_binary` 被客户端审批策略 `never` 拦截，本轮没有新 REA Evidence ID，也没有打开的原生会话需要关闭。改用本地 Apple LLVM 17.0.0 对同一 ELF 做只读反汇编，取得 21 个函数、2,018 个指令位置；复用 13 条已有完整 REA 记录。[函数索引](stage2al-evidence/functions.md)、[来源清单](stage2al-evidence/function-manifest.json)、[方法与限制](stage2al-evidence/method.md)保留两种证据的区别；`local_sha256_*` 是本地记录摘要，不冒充 REA ID。

目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。校验器将全部捕获指令的机器码与 ELF 对应字节核对，函数边界来自 ELF 符号大小，不声称是新恢复的完整 CFG。

## 哈希、相等和建表

下列地址均是目标 ELF 虚拟地址；完整记录 ID 见函数索引，机器可读结论见 [findings.json](stage2al-evidence/findings.json)。

| 路径 | 静态结论 | 主要位置 |
|---|---|---|
| Int 哈希 | `i32 >> 4` 算术右移，再与 `tableSize` 相与 | safeFindForInsert `0x0809d42f–0x0809d43d`；grow `0x0809c9a1–0x0809c9aa` |
| Bytes 哈希 | attrib&1 分支读取缓存 u16；缓存零则调用 whashstr(ptr,32) | `0x0809d5aa–0x0809d5d1` |
| 字节哈希 | 首 NUL 前最多 32 字节；首字节 `hi=T[b0],lo=0`，后续 `hi=T[hi xor b],lo=T[lo xor b]`，返回 `(hi<<8)|lo` | whashstr `0x08089dc8` |
| Bytes 相等 | 同头快捷判断，否则先比较完整长度，再比较全部字节；不同 shared 头另有优化拒绝分支 | `0x0809d664–0x0809d6a8` |
| 建表 | hint≤8 分配8桶，否则分配严格大于 hint 的最小2次幂；hint 先截至0x300000 | safeAllocateMapping `0x0809c20c` |

256 字节置换表的制品地址为 `0x083cc3e0`，文件偏移 3,683,296，SHA-256 为 `aa5a5e7ca4804ae04607f40d749f988a83aa2bc2a5ad80ce8ffc18ea44fe478e`。完整表及使用地址保存在 `native.json.gz`，按 ELF 文件映射独立核对；未观察程序运行时对表的使用。

哈希不是相等判定：`a`、`a\0`、`a\0x`、`a\0y` 都得到 14336，仍是四个不同键。超过32字节的共同前缀也会碰撞。`a` 与 `A` 哈希分别为14336、59904；单字节字符串哈希低8位均为0。高位字节按无符号表索引处理，无 UTF-8 解码。Int 0至15也全部碰撞。attrib=0、Buffer、Real及其他键分支仅登记边界，不纳入首批实现。

## 插入、扩容与删除

`vm_safeFindForInsert@0x0809d404` 找到已有键时保留节点；clear 参数非零会清理旧值。缺失键头插到桶链，新节点为0x34字节。`unfilled` 不是元素数：初值 `floor(80*capacity/100)`，仅当插入目标桶原来为空时递减；减到0先扩容，再插入。大量碰撞不会按元素数触发扩容。

`vm_growMapping@0x0809c894` 将容量翻倍，超过0x400000则失败。旧桶从高到低扫描；`hash & old_capacity == 0` 的节点保留原相对顺序，其他节点依次摘下、头插到对应高位桶，故迁移的子链反转。预算重设为 `floor(80*old_capacity/100)`，新高位桶首次占用减1，旧桶被搬空加1；不能改写为常见的 count/load-factor 阈值。

例如依次插入 Int `[0,128,256,384,640,16,32,48,64,80]`，最后一次插入前扩到16桶。按桶升序/链头向后扫描得到 `[256,0,16,32,48,64,80,128,384,640]`，unfilled=5。`18` 以原始 pair 数作 hint，从原生栈顶键值对开始插入；重复键按这个处理顺序覆盖，不能先去重再决定容量。

`vm_mappingDelete@0x0809cf84` 摘除首个匹配节点、count减1，桶变空时预算加1；不缩表，也不调整任何循环游标。`vm_freeNode@0x0809c33c` 检查键/值槽引用计数：有额外引用的分支清理值后把节点放到回收链，另一分支销毁静态值并释放块。**任意残留成员引用、回收链再利用及完整 GC 尚未闭合**，本轮不据此设计公开删除 API。

## 游标、修改和输出别名

既有 `vm_cmpAndJmpInLoop@0x080bcae0` 记录 `ev_2f7c7986819c8b0acae9e1cddb895fded61ac3d31a2c036489d87c95592ebcce` 与本地机器码相互印证：902控制块持有 Mapping header 和 bucket/subindex/ordinal。每次检查先推进 ordinal、subindex，再从当前桶链头跳过旧 subindex 个节点。跨桶时 bucket++，本次 skip=0，存储的 subindex=1；耗尽也推进 ordinal，不设置永久 finished 标志。

- 插入 `[0,1,2]` 后先输出2；此时删除2，再检查输出0，跳过1。若改为头插3，再检查会重复输出2，之后1、0；新增3不见。
- 未访问值的覆盖在下一次读取可见。已扫过桶中的插入可能不见，后续桶中的插入可能可见；扩容按当时桶链改变后续扫描。
- 耗尽后 bucket 通常为旧容量，subindex仍为1。后续扩容可使该桶重新有效，但会跳过其首节点；这不是自动恢复从新桶头遍历。
- 成功输出 `[key,value,ordinal]`，ordinal按32位载荷环绕解释为Int。循环每次读取绑定变量的**当前值**：若为长度3数组，则原地覆盖三个成员，不检查独占引用；否则分配新数组。已有数组别名会看到更新。循环体改绑到另一个三元素数组，下一次复用的是新数组。
- 空集合/耗尽不改绑定；源变量改绑不替换控制块持有的 header。耗尽及break不自动释放控制块，覆盖控制槽或既有帧清理才释放owner。source属性位仅在新输出数组路径传播，复用路径不执行该OR。

这些是普通合法图上的静态推导。输出数组与源容器形成交叉别名、绑定为源成员、额外属性、非局部错误解栈和回收链再利用仍不能称为兼容。原生先更新计数、再分配/清理，不保证失败事务性；后续 Rust 的借用/资源失败原子性属于明确的工程策略。

## 验证与剩余工作

[43项期望案例](stage2al-evidence/expected-cases.json)区分静态推导、Rust设计和未解问题；Python模型只复核所选成功路径，不作为执行原版得到的 oracle。[语料报告](stage2al-evidence/corpus-method.md)复算7,159程序、101,592普通函数、6,474,773指令中的298,369个相关位置，保留52个定位样例；数量不证明Mapping类型或动态配对。

28项Python测试、487项Rust测试的debug/release、fmt及Clippy通过。503件受保护文件（含参考制品、Rust源码/测试、Cargo及旧证据）哈希不变。完整命令与结果见[2AL验证记录](stage2al-validation.json)。本轮未重跑包解压比较：加载器及输入制品未改，统计直接重算冻结结构语料，并核对独立兼容性计数。

下一批按实施规格接入普通函数的 Int/Bytes Mapping997，保留既有拒绝边界。完整G05仍需其他键/属性、删除后引用与复杂别名证据；本轮完成取证交付不代表这些门槛已经关闭。
