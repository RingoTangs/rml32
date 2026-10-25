# 2AX：T05 直接捕获的证据门控规格

本页是后续 Rust 设计约束，不是直接 Object/Function 捕获的实现声明。2AX 以 [完整新增记录](rea.json.gz)、[复用记录](reused.json.gz)和[冻结生命周期复核](lifecycle-review.md)为依据；没有执行参考程序，没有新增独立状态模型。本批公共 API、Rust 运行时、版本和依赖均不变。

`container_capture_ready=true` 仅沿用 2AW 的受限 Array/Mapping 能力；`direct_owner_capture_ready=false`、`non_scalar_ready=false`、`stage2_complete=false`。下文“必须”是 Rust 设计政策；原生观察与未闭合项分别标注。

## 1. 生命周期基础必须先于能力开放

原生静态观察支持三类不同的持有：Function 头的 creator 普通对象引用、捕获 Object 载荷的普通对象引用、活动调用帧的 execRef。即使 creator 与捕获对象相同，也不能合并其持有。Function 头自身的语义唯一性又是另一个维度；宿主快照、Rust 临时 Rc 克隆及公开句柄观察不能改变唯一 move／共享 COPY 的选择。

依据：复用 `vm_createFunctionValueEx`，`ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`；新增 `vm_unrefFunctionValue`，`ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4`。后者在最后头引用退出时逐槽 cleanup，再 unref creator，最后释放捕获数组及头；槽自身 ref 不能作为允许捕获槽外部 Reference 逃逸的依据。

设计要求：后续生命周期表示必须分别描述普通 ref、execRef、删除请求、失效状态、队列持有、对象变量清理和最终壳释放。`0x4000` 待删与 `0x8000` 失效不能合并；COPY 只因失效而规范化真实源槽，不能把仍在执行的待删对象提前当作 Undefined。现有 ObjectId／ObjectStore 存在性检查和 Rust Rc 不构成上述契约的替代证明。原生低层状态证据及地址见 [生命周期复核](lifecycle-review.md#延迟删除的局部状态链已存在证据)。

## 2. 普通线程退出的已知顺序与未闭合边界

原生静态观察：在所选普通解释线程、没有脚本 catch 接管且 hook 不干预的路径中，错误经 backend 恢复，`vm_doActions` 返回 `-3`；调度器先调用 `vm_endForBackend`，再保存 PCB 的 sp／funCp 等状态，摘除活动线程并进入 `vm_deleteProcess`。后者先处理 release／crash／end hooks，再减少 PCB ref；ref 归零才清理 deletionFunc、剩余值栈及符合 flags 条件的帧 execRef。若 ref 仍为正，后续 `vm_unrefToPCB` 的归零分支承担最终清理。不能把逻辑删除等同于立刻释放所有载荷。

该链依据以下完整记录：

- 复用 `vm_errorQuit`：`ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1`；`vm_doActions`：`ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e`。
- 新增 `vm_traceError`：`ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1`；`vm_scheduleProcessInternal`：`ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3`。
- 新增 `vm_endForBackend`：`ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a`；`vm_deleteProcess`：`ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab`；`vm_unrefToPCB`：`ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748`。

Unknown：本轮 `endForBackend` 在 PCB 最终清理之前，因此最终 cleanup／unExecRef 新增的待删对象不保证在这一轮已被排空。下一安全点及最终回收时机仍须取证。间接 hooks 的目标、异常、重入和额外持有也未闭合；“value-flow 未截断”不消除未解析间接调用限制。上述路径不扩展至其他线程、JIT、回调或 shutdown 全域，Q05 保持 partial。

原生静态观察：`pdb_allocateMemory` 对零长度请求可使用分组块或带管理头的非零 malloc 大小；成功取决于初始化和资源状态，并非必然失败。依据新增 `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c`。这只解除零长度分配的局部疑问；`sp=0,argn=0` 候选路径中的未发布 Function 头恢复根与最终释放仍未知，不能据此宣布泄漏、不可达或已被 GC 回收。

## 3. 冻结的复制、发布和接口约束

后续直接捕获必须继续经过 2AU—2AW 的同一发布契约：

- `17` 对真实源槽 raw transfer，`05` 按语义 Owner 选择唯一 move／共享 COPY；共享 COPY 可规范化真实持久源槽，唯一 move 不追加 COPY。Function 头身份、稳定捕获槽及固定 bound_count 不改变。
- 所有可报告分配、借用及资源检查在可见提交之前完成；失败保留源、目标、操作数和 PC。Rust 成功发布前预检政策与原生失败顺序的差异继续单列。
- 拟提交图遍历 Function 捕获边，保留共享 DAG、最终边替换、存活祖先及 ValueLimits 检查。若引入 Object 的物理持有边，必须先明确 Object 变量与 creator 边如何参与生命周期及拒环；现有 Function／容器 DFS 不证明完整 GC 等价。
- Host 回调最多执行一次；前验失败不调用，后验失败不重放，saved root 与外部副作用按现有契约保留。快照不增加语义 Owner，HostValue 的显式根和 origin 隔离继续生效。

接口政策：本批不新增准备入口、profile 变体或可变权限开关。未来若通过对应门槛，只能以显式准备入口授权新增能力，按实际直接捕获类型保存不可变 profile，并在目标分派、COPY 和 Host 回调前检查 caller 权限。旧入口不得隐式升级；被调函数体继续使用所属程序权限。未来入口名称和具体签名须在对应实施批次冻结，不能将本页当作已可调用 API。

直接 Function 与直接 Object 分别验收，不以增加统一“任意 Value”白名单实现。Reference／隐藏 Loop、捕获槽公开可变访问或 Reference 逃逸、init／construct／default／linked 扩展、新 catch 来源、异步与重入继续排除。既有容器嵌套合法成员的运输规则不等于新增直接捕获授权。

## 4. T05 的独立门槛与完成定义

| 子任务 | 依赖及状态 | 必须交付的契约与验收 |
|---|---|---|
| T05-L 生命周期基础 | T04 已完成；blocked-native-contract | 明确普通 ref／execRef／队列持有的取得、转移与释放；区分逻辑删除、失效及最终壳释放；覆盖同对象承担 creator 与捕获载荷、最后帧退出、PCB ref 延后及最终清理后新入队对象；先解决对应未发布头和安全点缺口。 |
| T05-F 直接 Function 捕获 | 依赖 T05-L 的适用退出契约；blocked-native-contract | 仅显式入口；外层／内层头身份及 creator 独立；唯一 move 与共享 COPY 对内层捕获真实源的变化分别验收；验证头最终释放、跨容器共享 DAG／拒环、失效 creator、foreign origin、失败不部分发布。无需等待直接 Object 捕获开放，但不得绕过生命周期基础。 |
| T05-O 直接 Object 捕获 | 依赖 T05-L；blocked-native-contract；不依赖 T05-F 开放 | 分离 creator 与载荷普通 ref、执行 execRef、对象变量及队列持有；覆盖同对象／不同对象／无 creator、待删仍有效与已失效、最后普通引用和最终壳释放；明确 Object 生命周期边的图政策及资源失败原子性。 |

各实施批次必须把本轮案例映射中的适用项逐一接到真实 Rust 场景，保留全部既有 644 项普通测试与 2 项 doctest，再增加对应的边界、单步／零预算和故障注入场景；新增测试数量以实际执行为准。静态案例或模型不得充当原程序运行 oracle。

本页完成意味着门槛、依赖、验收及缺口有明确归属。T05-F／T05-O 的开放仍要求相应证据与 Rust 交付分别通过；未发布头、下一安全点、hooks、任意外部根和循环释放等未知不能因文档完成被改为 answered。具体下一步见 [后续任务](next-batch.md)。
