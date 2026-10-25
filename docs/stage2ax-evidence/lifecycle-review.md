# 2AX 冻结生命周期证据复核

本页独立复核既有 2AR、2M 完整记录，没有新增原生分析、执行参考程序或修改历史材料。目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。下列地址是原生虚拟地址；伪代码属于反编译观察，不是原始源码。该复核不单独关闭无 catch 最终 PCB 销毁或未发布函数头问题。

## 三种持有关系不能合并

观察：`vm_createFunctionValueEx @0x08091cd0` 为非空、未失效的 creator 增加一个普通对象引用，保存于 Function 头的 `+0xc`。随后原始搬移捕获槽，不调用 COPY，不因为捕获值恰好等于 creator 而合并持有关系。creator 已有 `0x8000` 时改存空指针；这并不证明任意 Object 目标与空 creator 的组合都合法。

依据：`docs/stage2ar-evidence/reused.json.gz` 中完整 dossier `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`。机器码观察点：`0x08091da8–0x08091dc4` 检查/保存 creator，`0x08091ecf–0x08091ed4` 增加普通 ref，`0x08091e40–0x08091eb5` 初始化并搬移捕获。

观察：`vm_freeFunctionValue @0x0808c114` 先逐捕获槽递减槽 ref、无条件清理载荷并解除全局 value 链，再 unref creator，最后释放捕获数组和头部。不能由捕获槽的 ref 字段推导“外部捕获槽引用可安全存活”：本函数没有等槽 ref 为零才释放数组的分支。

依据：同包 dossier `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9`；`0x0808c164–0x0808c167` 递减并 cleanup，`0x0808c16f–0x0808c19d` 解除槽身份，`0x0808c1a8–0x0808c1f7` creator 与两块存储释放。tag 9 的普通值清理具有同类内联路径，依据 `vm_cleanupValue` dossier `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`；tag 10 的清理调用 `vm_unrefObject`。

推导：creator 与被捕获 Object 可以指向同一对象，但其普通引用分别由 creator 字段和捕获载荷承担。活动帧的 execRef 又是独立维度；不应把语义 Function 头唯一性、普通对象 ref、execRef 或 Rust 物理 Rc 合并。

## 延迟删除的局部状态链已存在证据

| 转换 | 原生观察和机器码锚点 | 冻结依据 |
|---|---|---|
| 请求删除且 execRef > 0 | 只置 `0x4000` 并返回；`vm_destructObject @0x080b1894` 的 `0x080b18a0–0x080b18b5` | 2M `rea.json.gz` 汇编 `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` |
| 最后执行引用退出且待删 | 减 execRef、隔离对象、入 `pDestructingList`、置 `0x8000`、增加队列普通 ref；`0x080b180f–0x080b1840` | 2AR reused/2M rea 汇编 `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` |
| 队列清理 | 先摘链，再低层删除，再减少队列 ref；`vm_freeObjectsInDestructingList @0x080b1a04` 的 `0x080b1a1a–0x080b1a3f` | 2M rea 汇编 `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63` |
| 低层存储删除 | 清绑定值/变量，摘 hash；ref 为零释放壳，否则置 `0x8000`；`0x080b193e–0x080b196a`、`0x080b197b–0x080b1996`、`0x080b19d8` | `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b`；配对伪代码在 2M reused，`ev_196547cb37731a3632d63fbc4a62d83a57ac5f788fa57a3ed95657b8a8346c69` |
| 最后普通引用退出 | ref 减到 <= 0 且已有 `0x8000` 才释放失效壳/程序引用；存活对象 ref 归零不启动业务析构 | `vm_unrefObject @0x080b1618`，2AR reused 伪代码 `ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e` |

`vm_endForBackend @0x08096f44` 调用队列清理，依据 2M rea 伪代码 `ev_ed6d15234b6b77e70190cfcdc433e895468986122447c7f45f1bd8c78c562ba6`。同包 caller 记录 `ev_f5e402fe68092fc8f3da7f19e3e8f75ed5e9d36f205c394d10e109d9618d37b5` 另列 `vm_clearEnvironment @0x080aca84` 与 `vm_shutdownScheduler @0x080ae0e8`。这些是局部调用/状态证据，不证明普通线程每条错误路径一定抵达上述安全点，也不证明 reentrant 路径。

观察：COPY 的失效检查针对 `0x8000`，不是 `0x4000`。`vm_copyValue @0x0808be24` 在 Object 分支 `0x0808be87–0x0808be90`、Function creator 分支 `0x0808beb3–0x0808bec6` 检查失效；`0x0808be9a–0x0808bea7` 清理真实源槽后重读类型。依据 2AR reused dossier `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`。待删而尚未失效的对象不能直接按死对象规范化。

## 最终结果压栈失败需要拆分可达域

观察：创建循环每次搬移后增加 `vm_sp`，末尾只在 `vm_sp == 0` 报结果栈溢出。上述 create dossier 的 `0x08091ea2–0x08091ebb` 与 `0x08091def–0x08091df6`、`0x08091ed9–0x08091ee1` 支持此控制流。

静态推导：在合法栈范围、非负捕获数及没有其他状态修改的前提下，非空捕获已经腾出结果位置，不能触发这一末尾零栈指针分支。需要调查的合法候选是初始 `sp=0`、`argn=0` 的未发布函数头；还须先确认 `pdb_allocateMemory(0)` 是否能成功。不能把原 2AR 未知项改写为“已经搬走非空捕获后仍可结果栈溢出”，也不能仅凭当前局部 CFG 宣称分支必然可达或泄漏。

## 图标记、外部根与槽暴露边界

观察：`vm_markActiveFromValue @0x0808d994` 先将槽 active 置一，再按 tag 分派。它递归 Array、Mapping、Function 的捕获槽和部分隐藏 Loop；tag 8 Reference、tag 10 Object 没有这里的递归分支。Function 捕获循环在 `0x0808da8a–0x0808dab9`，type 9/20 判定及其他类型返回在 `0x0808d9e9–0x0808d9f5`。依据 2AR reused 汇编 `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471`；伪代码 `ev_dc9d455696d05ef4b444be9dc273ee408ac79c93d4c11138399d240e3b7a82a5`。

观察：`vm_refreshAllValuesActive @0x0808dd0c` 先清全局槽链的 active，随后从对象 hash 中的变量、全局变量、PCB 栈、type 2 timer、指定 pend queue 和 `_allMarkActiveInfo` 回调建立根。依据 2AR reused 伪代码 `ev_11d1d264620ab7f5452f093971acc5d836f0364c7d522a7234d1af509587c5fd`。最后一类是间接回调，不能把回调列表入口等同于已穷尽所有外部根。

观察：`vm_initValue @0x08089f10` 将槽 ref 初始化为一并加入 `vm_allValues`，依据 2AR reused 伪代码 `ev_6921332b80a7a8467156e8214b2cf335f37f0c97324f7e7737d1d6c56b44b54d`。全局分配链不等于存活根；它同时被 marker 重置和释放路径解除。Function 头本身并不由此证明进入该链。

设计约束：现有 Function/Array/Mapping 无环图政策不构成原生 GC 等价。直接 Object 捕获若未来引入对象物理持有关系，必须单独处理 Object 变量和 Function owner 的生命周期边；直接 Function 捕获须保持头身份、共享 COPY 源清理、唯一 move 以及内层捕获图验证。不得为闭合本批而开放捕获槽 Reference 逃逸。

## 建议新增证据与仍未闭合项

- 优先追普通解释线程无 catch 的 backend 恢复落点、PCB 逻辑删除、最终 ref 释放、crash-hook 与帧/栈清理顺序，再连接已知 `vm_endForBackend` 队列清理。已有队列 caller 列表不能替代这条路径。
- 调查 `pdb_allocateMemory @0x080ce784` 的零长度行为和分配追踪；调查未发布零捕获函数头在错误 longjmp 后是否仍有可达回收根。若只发现分配统计，不等同于根或恢复机制。
- 补充槽暴露路径或明确限定“捕获槽无公开 Reference 逃逸”的子域；现有最后头释放忽略槽 ref 的事实不能推出任意外部引用安全。
- 外部 mark 回调目标、循环释放顺序、其他线程/JIT/回调/重入删除继续单列 unknown/out-of-scope。普通线程的局部结论不关闭整个 Q04/Q05/Q06。

本独立复核只提供后续整合依据；T05 的最终门槛以 2AX 总账及新增取证为准。
