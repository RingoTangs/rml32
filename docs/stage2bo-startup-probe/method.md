# 方法与证据边界

基线为 `c7a7d3b0cdb4634f18e9772493bad440921ca7bb` 的 1243 个文件，逐文件哈希与原索引哈希记录在 baseline.json。仅允许修改 Cargo 配置、五个既有 Rust 模块以及两个入口文档；实际变化清单、冻结数量与原测试体 token 核对由最终收据给出。无依赖、版本或旧模型变更。

## 指令与权限

原始映像完整解析，只有 init PC 0..25 被单独编译为初始化视图。保留每条实际 offset 和来源字符串池；普通函数与剩余初始化均未准备。没有伪造 PreparedFunction、初始化返回、预置全局槽或调用结果。`is_initializable` 继续只表示完整准备的 Object 初始化，不把 Global 前缀当作初始化完成能力。

准备和执行分别检查七种操作码：03、06、18、33、36、3b、40，06 只允许槽 0 与 argc 1。执行器先检查 PC 25 的 fence，再读取任何指令；内层测试在已准备代码中故意追加 mkdir、替换分派/返回/跳转或破坏索引，验证运行时检查独立生效。探针预算不会越过 fence；保留现有 VM 原子 COPY、06 提交和失败位置规则。

## 原生静态依据

使用 [reverse-engineer-anything 技能](/Users/ringo/.agents/skills/reverse-engineer-anything/SKILL.md) 经 REA/Ghidra 对原始 `magic_Linux32` 静态分析，目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。三份完整函数记录保存在 native-evidence.json.gz，包含汇编、伪代码、provider/profile、范围、调用关系及限制：

| 函数 | 地址 | Evidence ID |
| --- | --- | --- |
| vm_f_allocate_array | 0x080d58b4 | ev_b4bcaff8fe75bbfc39e8fd7b86daeec1e52347bf2b9cfb376d5014be0374d50b |
| vm_safeAllocateEmptyArray | 0x0809b0e8 | ev_80860ee3f2cb1ab82b73461b14770e373e09e2f8fd1e79bead11f6a0c10e3377 |
| vm_safeNewValue | 0x08089f78 | ev_6ad4e4a31f4846ef1c2950ccd0df173f861e78b97f61852ca9fbd4a30a4bf86c |

观察：allocate_array 通过 Int 参数取长度，无第二参数时逐个调用 safeNewValue；safeNewValue 汇编清零六个 DWORD 并设 type/attrib 为 0；原生分配失败路径弹出参数后压入 Int 0。由既有标签证据推导默认元素为 Undefined。Rust 显式资源策略继续返回 Limit，不能把此差异隐藏为原生等价。未执行参考 ELF，不宣称动态观察到了这些路径。

## 当前与历史验收

当前源码执行 debug/release Rust 测试、构建、fmt、严格 clippy、只读全语料比较和十次真实探针命令。两个构建的 JSON 输出逐字节相同，独立 Python 参考检查固定来源、偏移、完整全局内容和实际调用清单。新私有故障测试以小数组走相同真实路径，逐个注入实际校准到的 workspace reserve 失败；实际 1024 项数组另有真实前缀和大小边界测试。

17 个旧驱动由 replay_current.py 重新运行，保留旧断言；忽略随当前源码和日志变化的来源/日志 SHA 字段后，其他语义与输入/观察哈希须与冻结结果一致。双事务私有六组和公开组还要求原始压缩观察哈希相等，日志单独保存。旧模型不修改。

历史 2BN 收据使用独立本地 clone，检出固定 c7a7d3b 工作树，并重建其收据原本要求的 13de257 索引身份。只有能匹配记录哈希的 LF/CRLF 转换允许恢复；全部 1243 个文件字节需匹配。历史验证不是在当前源码运行，也不使用历史通过数替代当前验证。

完整复现：`python3 -B docs/stage2bo-startup-probe/verify.py --write --replay`，其中包含 17 个旧驱动的实际重跑。它们在本批最终 Rust 源码上全部完成后，可用 `--write` 单独重新生成收据；复用前必须重新核对全部 Rust 来源哈希、执行日志、冻结语义结果及观察哈希，来源不匹配即拒绝复用。仅需单独重跑旧驱动时可调用 replay_current.py --write。独立只读核验为同一 verify.py 不带参数。源码及正式输入证据稳定后才能开始收据生成；各命令产生的输出单独记账，最终每条命令、退出码和输出保存在 validation-log.json.gz。

本批长回放期间，仓库新增了包含实现的提交 `b272eb1`。原收据打包器曾因原索引哈希变化而拒绝输出，实际 22 条验证命令均已成功；随后将主仓库验证改为固定基线的祖先关系、1243 文件字节清单和原测试体检查。主仓库索引不作为源码权威，验证器不改写它；独立历史副本仍保留 2BN 收据要求的索引身份。重新打包复用了已完成且来源一致的 17 个驱动，不再执行同一大规模回放。
