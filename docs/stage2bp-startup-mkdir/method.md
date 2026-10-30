# 证据与验收

基线为 `e4d7ca6d55fa2c7e4675301fb3d9b0f384803774` 的 1265 个文件，原字节清单及索引来源记录在 baseline.json。当前只允许修改五个既有 Rust 文件和两个文档前缀，新增两个内部源文件和一个集成测试文件。原测试体按 token 保留，其余文件逐字节核对。主仓库索引不作为源码权威；检查固定祖先、文件清单和最终 artifact 哈希，验证器不写主仓库索引。

## 静态原生依据

规划阶段使用 [reverse-engineer-anything 技能](/Users/ringo/.agents/skills/reverse-engineer-anything/SKILL.md) 对原 `magic_Linux32` 做 REA/Ghidra 静态分析，未执行 ELF。八份完整记录保存于 native-evidence.json.gz，绑定目标 SHA-256 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`，保留汇编、伪代码、函数范围、provider/profile 与限制。

| 函数 | 地址 | Evidence ID |
| --- | --- | --- |
| vm_f_mkdir | 0x080e9110 | ev_76a902aa7755f7bb95c23a14317018f4e98f55802a5ae5be9132216ceb46ffef |
| vm_generateLocalFileName | 0x080be1fc | ev_fd70a136d5ad27d9462528647f7512c2e13250eaf83341048976f996ea12cb51 |
| vm_deriveFileName | 0x080bda64 | ev_90cc83434942394cccb6a1fb1c2d4020cae5de89d90e1cd3f70576fd3d88a839 |
| vm_generateFileName | 0x080be018 | ev_fc03447f1e0a1e31399c2156ae938611c940dc66cdcb2b034cf4a03b4d259001 |
| vm_assureEuid | 0x080b137c | ev_34efb8f1d65ff725aac88b544ef41f2af10f2aed5a02984013e6ad047c9ee578 |
| vm_isInVirtualVolume | 0x0809f2e4 | ev_ce107980337956d4bec341a6f0169279c00f84277252e3503ee7579d2634b225 |
| vm_isInPackedVolume | 0x0809f304 | ev_ea51034fc784938c34911b1afedfa9c116b172de3a9695cb2e36b8d2ff362ef6 |
| vm_isInDbfsVolume | 0x0809f324 | ev_6fc2ea4a6667e5ce03aec5cdfe6c312f2b202440af316b69403c9dd00cb1c364 |

观察：mkdir 检查 euid、读取一个 Bytes 参数，转换本地路径，按 DBFS/packed/local 分支处理；本地路径调用 `mkdir(path,0777)`，将系统调用返回零转换为 Int 1，其余转换为 Int 0。assureEuid 对 thisObject 为 NULL 的上下文不拒绝，符合本批 Global 初始化帧的 None 对象。路径辅助函数还含挂载、相对路径和点分量规则，本批明确排除这些扩展，不以简化内存后端宣布完整文件系统兼容。

## 当前差分和失败测试

model.py 独立实现受限路径与目录状态表，不读取 Rust 结果推导期望。18 类字节路径 × 6 类初始目录/配额 × 3 种预算序列，合计 324 条场景。每构建逐个比较 324 个初态和 1296 个执行后检查点，包括完整全局状态、目录树、调用记录、返回/错误及 sticky 行为；debug/release 压缩原始输出须逐字节相等。

驱动只接收路径、初态、配额和预算，不接受 expected。它在真实 VM 中执行由 AAA 初始化前缀构造的合成 ProgramImage，数组大小改为 3 以控制故障矩阵成本；该 sha 标签不作原始成员认证。真实五服务另经 load_with_slice 从认证成员执行 1024 项原数组，并核对只增加各自的日志目录。默认 25 指令 stdout 与旧 2BO 实际命令日志逐字节比较。

新私有测试另外校准 mkdir 调用的实际 workspace reserve 次数，逐个注入失败并加成功边界，分别检查变更前失败和变更后提交失败、操作数原子性、原始错误位置和禁止重放。它们是 Host 失败协议测试，未混算为无故障独立目录模型的统计。运行期测试绕过准备故意污染已准备代码，验证权限检查和 28 指令 fence 独立生效。

## 历史权威与复现

17 个旧大矩阵结果属于冻结 2BO 权威，不重标为当前源码结果。目标缓存中重建固定 e4d7ca6 副本，逐文件恢复记录的字节；只允许能匹配哈希的 LF/CRLF 转换，然后运行旧 verify.py 的只读核验。该快照保留旧源码、原始观察、命令日志和 17 驱动记录；旧验证器不运行大矩阵生产命令。

完整本批复现：`python3 -B docs/stage2bp-startup-mkdir/verify.py --write`。当前全 Rust debug/release、build、fmt、clippy、只读语料比较、定向差分、20 次实际 CLI 和 8 项 Python 检查逐条执行。命令完成后立即落盘退出码和输出；最后绑定全部最终来源并生成新收据。只读独立核验为同一 verify.py 不带参数。旧文件与模型不刷新，重跑产生的本批输出单独保存。
