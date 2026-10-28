# 复现偏差与最小修复

## D08-R01：失效 creator 的未发布 Function

固定基线在 `P078-F10` 第 7 步接收 commit：先创建 o0、预留 Function，再立即删除 o0。冻结模型要求 `InvalidObject`，并在组合层 abort 预留。实际旧实现只把 HeadReservation 标记 committed，没有重新检查 creator，因此接受发布。完整 actions、模型预期和修复前观察保存在 `counterexamples.json`。

在 `HeadReservation` 新增内部 validate；VM Function 的预留输出就绪后、CopyPlan 提交前调用。拒绝时先释放临时输出及 plan，再由 guard abort 归还未发布头的配额，保留初始错误。有效的 Pending creator 仍可发布，F10 后半段专门检查这一点；没有把 pending 错判为 invalid。

该失效间隔通过私有 prepared 边界确定性制造。当前正常 VM 的同步 Function 构造没有已证实可由用户插入删除的间隔，故不声称公开调用能触发此并发序列。Rust driver 直接调用同一内部 validate，生产路径在正常库构建中调用；冻结旧基线复现时只移除这条新增调用，观察器插入前后生产 token 投影相等。

## D08-R02：Mapping 循环首次输出的 origin

正常 bounded 库执行 Mapping 循环，绑定槽尚无三元素数组时，旧 `mapping_output` 使用 legacy 数组与 Cell 工厂。后续 CopyPlan 组合输出更新时，origin 校验产生 `InvalidState: mixed slot origins`，循环无法返回预期结果。它可通过公开 VM 接口触发，与私有 R01 交错边界不同。

bounded 模式下的新输出改用同源数组工厂。三个初始成员为 Undefined，待同一原子更新写入 key/value/number，避免先填 payload 再更新造成额外峰值 retain；已有三元素输出数组继续复用。正常回归覆盖首次输出、已有输出身份与别名、run(0)、catch 展开、对象成员保活/释放，以及普通引用峰值失败后的栈和先前成员创建。

原始失败测试源码压缩保存在 `counterexample-fixtures.json.gz`；在冻结 2BI 副本运行指定测试，预期退出码 101 和错误文本均验证。原始夹具内其他未选中用例后来修正了参数/配额设置；复现使用保存的原始源码，不把那些夹具问题列作生产缺陷。

## 修复边界

仅 `src/vm.rs` 与 `src/lifecycle_runtime.rs` 有上述生产改动；`src/boundary.rs` 只接入 cfg(test) 模块。没有改变公共 API、配额定义、旧模型、legacy 工厂策略或直接捕获权限。两项反例均由 `baseline_proof.py` 在固定基线重新执行，修复后由核心差分、正常库镜像及普通组合测试复核。日志和哈希见 `baseline-proof.json` 与最终收据。
