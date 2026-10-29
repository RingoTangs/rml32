# 方法与复现

在仓库根运行：

```sh
python3 -B docs/stage2bm-interleaving-audit/historical.py --write
python3 -B docs/stage2bm-interleaving-audit/reference_replay.py --write
python3 -B docs/stage2bm-interleaving-audit/interleaving_diff.py --write
python3 -B docs/stage2bm-interleaving-audit/public_replay.py --write
python3 -B docs/stage2bm-interleaving-audit/verify_validation.py --write
python3 -B docs/stage2bm-interleaving-audit/verify_validation.py
```

前四条实际执行生产者并保存本批产物；主验证只读重算生产者报告，并执行完整 Rust／Python 检查。最后一条独立重算最终收据。报告中的子命令和输入、观察哈希记录真实执行，不把只读验证声称为再次执行 Rust。Cargo 使用 locked/offline，不下载依赖、安装工具链或镜像，不改写旧收据。

## 固定权威

`baseline-inputs.json` 固定提交 072aecf 的 1,166 个路径与字节哈希。历史 Python 在 detached 基线副本执行：2BL 的 32 项加其递归固定权威 525 项，共 557 项；保留经哈希确认的 CRLF 字节。不能在新增源码上运行旧收据验收。旧 13 个回放驱动通过冻结 runner 的 run 函数实际重跑，日志只保存到本批；配额模型 39 项和算术 112 行继续通过原 runner 执行。2AW 语料只复用冻结哈希，不运行原程序。

## 有限生成域

核心组沿用字节冻结的 2BL 16 类布局，分别以 setup 中每个对象为转换目标。两条路径为同步 delete，以及 exec → delete → exit → 单对象维护。对路径的全部有序插入点 `begin <= end`，插入 prepare 与 commit/abort，保留路径后缀；失败按实际动作边界停止，终结动作的预期拒绝允许继续检查后缀。

每布局沿用原配额配置，并增加充足配额及独立参考层计算的 objects／Function heads／ordinary／exec／header owners 峰值减一、等于、加一，过滤非法下限并去重。配额矩阵使用无额外干扰路径；充足配额另枚举一次源改写、目的改写、外部 alias 释放、物理 view 保留／释放或全零维护。这里采用分层覆盖，不声称对所有配额与所有干扰做全笛卡尔积。

定向组覆盖 Object／Function 的 raw 与 script COPY、空目的 raw move、新 Function 预发布、prospective 执行租约 commit/abort、被清理变量作为源或目的，以及两对象的两种 LIFO 次序。配额为原 163 个 profile 加充足对照。通过真实动作抵达状态，不强写 lifecycle 枚举；带普通 owner 的对象不可能处于 Freed，预留 creator 也会阻止其提前 Freed。不可达组合不伪装为目标覆盖。

## 准备与资源责任

新参考层独立声明槽、头、creator、执行 token、版本和队列，以槽图重新计算普通引用及 header owners。Array／Mapping／Function 最后语义 owner 释放时迭代清理成员并释放 creator；物理 view 只影响可回收性。共享成员不因父边重复计算。

CopyPlan 的值／版本快照本身是物理值，不等同于冻结 D08 模型 reserve_copy 已取得的 prospective owner。新层在实际提交边界检查 COPY 的新增峰值；staged Function 输出槽与 prospective ExecLease 则在准备时已经占额。脚本规范化在既有 COPY 准备边界决定；准备后仅对象状态改变、槽版本未改变，不凭空重新执行 normalize。新 Function 发布另外复核 creator；Pending 仍有效，Queued/Shell 不可发布。

私有 Function、直接 capture Cell 改写和执行预留只证明内部边界。新 Function 的输出通过真实 CopyPlan::output 取得，HeadReservation 的 validate／commit／Drop 均实际执行；失败先释放输出槽，再由 guard 归还未发布元数据。此类确定性交错不声称可从公开同步 VM 指令中插入。

## 观察、失败与维护

Rust 输入只有动作、显式配额和逻辑标签，没有 expected 字段或参考计数。回放器读取真实 Cell、生命周期记录、队列和弱元数据；实际 ID 映射到逻辑身份，Int 与 Undefined 保持区别。观察不获得语义 owner、不触发安全点、不保存隐式强头视图。动作前双观察相等，相邻 after/before 连续；每条 trace teardown 后 Registry／ctx 的 Weak 必须失效。setup 作为宏动作报告精确失败子动作和实际部分完成状态，不声称中间每个成功子动作均单独输出观察。

校准按 family、动作、配额和对象状态签名，选择成功的 commit、同步 delete、Function 输出准备及单对象 drain，实际记录 workspace 调用数 C，注入 0..C-1，以 C 作成功对照。维护工作区阻塞返回 Ok(progress)，不能当作 Err 计数。借用覆盖不同状态签名的写槽、退役成员、同步删除变量及队头变量；另有普通 Rust 回归确认第二个对象工作区失败时保留第一个成功单位及剩余清理责任。

元数据六条定向路径覆盖三种头、物理 view 保留／释放、零预算、一条记录预算和收敛扫描。复用冻结 2BJ 的独立 snapshot-list 参考层，以维护前注册事实／eligible／epoch 推导全部进度、访问后键集和扫描代际；不以 Rust cursor 或返回计数生成期望。Object eligibility 再由独立 Freed 状态复核。头物理存活和 weak 行属于具体注册事实细化；通过验证的键移除投影到语义头／对象 tombstone，**不宣称完整模型状态等价**。

## 正常库与验收

公开组使用真实 VM 帧、HostValue、Array／Mapping、成员读取 opcode，覆盖两种 payload × 两种容器 × Alive/Pending/Queued/Shell × 有无 alias，共 32 个组合。Pending 使用受删对象的真实读取帧；Queued 使用返回后仍未 drain 的对象。验证 raw retain 保持身份、脚本读取按有效性处理实际成员、最后 raw owner 释放后恰好 reap 一次、旧 ID 拒绝及新 ID 不复用。公开失败按 setup/transition/start/copy/release 分类，只要求 Limit，完整成功投影独立核对，不声称全部公开工厂的配额阈值等价。

另有普通回归在参数入栈后删除对象，再实际执行同步 Host 回调，验证调用一次、raw 结果及完成槽一次提取；覆盖 Object 和已有 Function。完整 878 项普通测试、5 项 doctest 在两种构建保持，15 个 ignored 驱动分别记账。38 项校验器包含状态语义、丢行、乱序、计数／身份污染、缺少成功对照、缺失旧测试／doctest／驱动及失败命令。收据绑定本批源码、文档、观察、日志和固定基线。
