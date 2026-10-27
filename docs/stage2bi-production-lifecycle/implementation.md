# D08 生产实现与兼容边界

[公开类型](../../src/lifecycle.rs) 将五项计数配额与值图限制分开。构造前校验范围，Object 上限受 `u32` ID 域限制，普通引用至少为 2，执行引用和 header owner 至少为 1；非法配置返回 `Limit`。配额固定保存在 origin 的生命周期 context 内，公共 API 不暴露 setter。没有引入新的 dependency、feature 或自动默认配置。

## 所有权与预留

正常构建现在使用与冻结回放共享的生命周期实现；回放工厂、故障注入与独立 audit 仍受 `cfg(test)` 限制。内部 `test_life` / `test_*` 命名为兼容既有回放和测试调用而保留，它们承载的 bounded 路径已编译进正常库。普通测试模式的无配额策略继续存在，不能用它们的成功代替生产模式验证。

实际 header owner 在 `Counter` 上 checked 增减，绑定 origin 的 `Weak<AdapterContext>` 和独立 `Epoch` 通知。Object payload 与 Function creator 的租约持有独立 ObjectEntry，不拥有 arena。Function head 只保存 creator 的 entry 和函数数据的弱引用；容器状态也只保存物理 header 弱引用。内部结构不从 entry/counter 强引用 control；外部 HostValue 持有 Registry，关闭存储可停放唯一实际 store，最后根退休后释放。

`Owner`、Object/Function/Container lease 与 `ExecLease` 都在发布前取得责任，在失败路径随 RAII 释放。CopyPlan 的 output/更新为峰值计数，旧 payload 释放发生在事务全部 guard 释放之后。raw transfer 移动原 OwnedPayload 或 OwnedValue；不通过 acquire/release 配对模拟移动。每个 head 只取得一次 creator，捕获槽另有自己的 payload 责任。

新 Function 的 metadata reservation guard 在 output/workspace/commit 失败时取消未发布登记，避免耗尽 head 配额。CopyPlan 记录实际源/目标 Cell 的版本，在 commit 前再次校验；源码没有公开 token 或可伪造的 lease。Cell 版本耗尽后拒绝后续事务观察，结构释放可以无分配地标记版本不可再用，避免回绕后接受旧事务。现有 preparation、捕获类型 profile 和投影 DAG 检查继续约束所有发布。

Array/Mapping 的语义 owner 与物理 view 分离：bounded 模式最后一个语义 owner 退出会清理成员，即使观察用物理 view 仍存活；最终 Function 同样清理 captures，再释放 creator。可级联释放的实际成员会先做借用预检。Mapping 后续新增的成员槽也绑定相同 context。legacy 模式保留旧释放规则。

## 对象与执行

每个 ObjectEntry 都预先带有 queue link 和 queue-owner 存放位置。alive/pending 普通计数保留一个单位给队列，queued/shell 可使用完整上限。最后一个 pending 执行租约退出直接使用该位置 LIFO 入队，没有节点分配或 fallible 工作区申请；重复删除/托管失败不会重复入队。

零执行 remove 在检查借用与清理工作区后，同步 invalid、清空变量并成为 shell/freed；借用或工作区失败不改变此次 remove。托管失败、取消以及 pending task 在下一次正预算 step 的中止仍保留队列清理责任，包含执行帧已经退出的情况。普通引用为零不删除存活对象。freed 记录在维护 reap 前仍占 Object 配额，ID 发放、容量归还与 nonce exhaustion 独立，不复用旧 ID。

prospective 执行租约先于旧帧退出取得，因此同一 Object 的替换/调用交接可在上限为 1 时返回 `Limit`，这与 D08 的峰值规则一致。普通 Object/Global 调用、间接调用、返回、catch unwind、task 与 store/VM 转移统一保存或释放实际帧租约；Global 帧不增加 Object execRef。

## 维护与关闭

维护入口先检查 bounded 模式和 closed epoch，不调用 Registry 的普通检查安全点。Host 根按 lease ID 升序、对象按 LIFO、元数据按 Object/head/weak 索引顺序工作，分别消费三类预算。每次清理持有 queue owner 到最后，当前单位失败时队头不变，不跳过它；之前完成的工作不回滚，阻塞返回 `Ok(progress)`。

元数据游标保存在 StoreState，按 BTree 的稳定 key 和每一 pass 的固定 fence 前进，不构建全表快照。epoch 变化要求下一 pass；`u64` 回绕设置独立 force-rescan 标志，不能让 Drop 失败。物理 Counter/Cell 销毁会通知 epoch。Object/head 回收归还相应配额，弱登记按所属 context 的唯一 ID 清理线程局部图缓存；维护不会扫描或清除另一 origin 的登记。既有图缓存的周期性失效项整理保持，不承担 Object/head 配额回收。

HostValue Drop 先设置预分配的退休标志和待退休计数；已有安全点可在无借用冲突时释放根。bounded 根最终释放还预检级联成员，借用冲突时延期。关闭先拒绝新使用，停放 store 并在 Registry 借用之外拆下 callback bindings，以解除 callback 捕获 HostValue 形成的外部控制块环。最后根退休或结构 teardown 释放 arena；这不是有预算的业务清理或原生退出进展承诺。

`Complete` 表示当前根退休、对象队列与元数据扫描工作完成，仍可能保留活对象、shell 或由物理 view 延迟回收的 head。空元数据集合可以直接确认完成；若非空 pass 因回收或其他变更变脏，则仍需观察新 epoch。预算单位是根、对象和登记条目，不限制一个对象的变量数、级联 DAG 成员数、底层分配器时延或进程总内存。
