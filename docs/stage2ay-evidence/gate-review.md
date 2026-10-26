# 2AY：E01/E02 与 T05 门槛重评

本页是静态证据范围与后续设计门槛，不是生命周期实现。没有执行参考程序；任何模型或 Rust 合成测试都不能把本轮 unknown 自动升级为原生结论。完整结论和机器码依据见 [findings.md](findings.md)。

## 证据门槛

| 门槛 | 本轮可确认内容 | 剩余边界与状态 |
|---|---|---|
| E01 合法栈失败候选 | F41—F43：初始化、动作表、push 与零捕获 17 形成显式前提下的有限静态路径；没有统一容量检查阻止该接点。 | answered 仅限该 entry 路径；主语料／编译器是否产出此序列另论。 |
| E01 发布与 allocator | F44—F46：H/A 和可选 creator ref 在失败前存在，结果尚未发布；扩展页、自由链与显式 free 接点已识别。 | 已闭合局部持有／存储接点，不等于语义恢复根。 |
| E01 最终恢复／释放 | F47：未发布 H/A 的后继发现、H/A 最终 free 及非空 creator unref 未形成完整证据链。 | unresolved；必须继续追实际保存／恢复关系，不能由池页可达或进程终止替代 VM 语义释放。 |
| E02 明确 drain 接点 | F48、F51、F52：后续普通 backend、clearEnvironment 与 shutdownScheduler 各有命名的条件化排空位置。 | answered 仅限正常抵达且目标已在队列的子域；不保证最终物理壳释放。 |
| E02 可见返回尾部 | F49、F50：responding peek 返回零、单次 wrapper、mainLoop 的指定返回／停止尾部没有额外显式 drain。 | 不能外推 helper、扫描器、间接回调无副作用，也不证明后续调用者一定进入 shutdown。 |
| E02 后继注销与进展性 | F53、F54：shutdown 静态槽先清 type；外部持有、回调、后继清理及全部停止域仍未穷尽。 | 整体 partial；延后 unref 只在 PCB 存储仍有效时讨论，不允许在已释放 PCB 上假设安全调用。 |

## T05 实施门槛

| 任务 | 状态 | 进入下一阶段仍需满足的条件 |
|---|---|---|
| T05-L 生命周期基础 | blocked-native-contract | 把 E01 恢复根与释放事件接到明确退出域；为拟支持域给出普通 ref、execRef、队列持有和外部持有退出的契约，明确未覆盖停止／hook 分支的拒绝边界。 |
| T05-F 直接 Function 捕获 | blocked-native-contract | 先通过适用的 T05-L 契约，再冻结显式入口、内外头及 creator 身份、唯一 move／共享 COPY、最终图和失败发布要求；本批不新增入口或放宽旧 caller。 |
| T05-O 直接 Object 捕获 | blocked-native-contract | 先通过 T05-L，分离 creator、载荷普通 ref、活动 execRef、对象变量和队列；补齐待删／失效、变量图与壳释放的适用域。无需等待 T05-F 开放，但不能绕过基础门槛。 |

本轮保留稳定捕获槽、真实源 COPY 规范化、唯一 move、成功发布前预检、拟提交图、Host 单次副作用和 origin/宿主根契约；这些属于已采用的 Rust 政策，不作为未知原生恢复路径的证据。本批不冻结新的生命周期内部表示，也不实现 D01。

## 完成与未完成的区别

2AY 完成意味着新增证据、限定子域、反例边界和后续取证入口可复核。它不要求强行关闭 E01/E02 父域，也不意味着 T05 可直接实施。`container_capture_ready=true` 仍只描述 2AW 的受限容器能力；其他直接捕获及阶段完成标志继续 false。下一批选择与具体退出条件见 [next-batch.md](next-batch.md)。
