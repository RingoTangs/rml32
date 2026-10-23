# 2AQ断言复核记录

相关测试不是逐分支覆盖证明。75行保留各自assertion_review；下表记录重新阅读的增量断言及其证明边界。原有路径沿用2AH人工记录，核对当前源码/测试定位与守卫，并由558项debug/release回归验证。

| 路径 | 实际断言与输入 | 限制 |
|---|---|---|
| Bytes普通二元运算 | bytes_static_evidence_cases_execute_through_vm逐项解码2AI期望，核对成功返回或kind/原pc/offset/栈及sticky；bytes_concat_boundary_and_aliases_preserve_source_payloads核对65536上下界、空追加共享及源内容 | 期望是静态推导，未执行原版；不涵盖Real格式化 |
| Bytes += | bytes_compound_add_limits_and_unsupported_types_are_atomic用超长/Real/反向Int/selector3核对目标、栈、诊断及sticky；budget测试核对零预算与单步写回 | 仅selector2及既有引用；不新增catch来源 |
| Mapping实时遍历 | mapping_loops_emit_native_order_values_and_ordinal_under_single_step_budget核对[0,1,2]逆桶链及ordinal；head插入重复旧key，live_growth核对显式10项顺序 | 不是动态业务图或所有mutation覆盖；无公开删除入口 |
| Mapping输出别名 | mapping_loop_existing_array_alias_sees_latest_output_and_zero_budget_does_not_touch_it核对初始[9,9,9]零预算不变、结束后共享header内容[0,0,2]；重绑测试核对新旧数组分离 | 保留交叉可达拒绝，不证明所有图形 |
| Global/c | clear_replace_and_restore_binding测试核对清除/更换后的InvalidState与同ID恢复；entered_frame测试核对旧帧代码与后续07使用新绑定 | 此约束是Rust安全政策，未证明原生任意重绑行为 |
| Host/K注册与上下文 | each_call_reads_replacement_callback_and_signature核对A回调计数、新签名拒绝时B计数0、补参后B计数1；context_comes_from_invoker_not_creator核对slot/program/object/function/pc/offset/source | creator只作失效检查；合成同步回调不等于真实efun |
| Host/K返回与副作用 | return_modes_errors_and_side_effects_are_never_replayed枚举Void/Mixed/Exact返回，失败保持栈/pc/sticky并核对计数1；never_apply_user_numeric_coercion检查预检失败计数0 | 外部副作用不回滚；不能推广到可重入/异步 |
| 能力入口隔离 | all_flags_and_old_entries_remain_isolated枚举256个17标志，检查组合与旧入口接受集合；组合测试核对06和三类函数值 | flags测试主要证明准备阶段，不证明全部捕获/任务组合 |

另核查function_values_tag9_arguments_returns_containers_and_mapping_key_rejection：显式构造Array和Mapping值含Function，读取后身份相等、深度为1；Function作为Mapping键报UnsupportedType。2AP运行时说明中“函数值不允许装入容器”过宽，不能作为当前统一限制；该测试是Object函数值场景，未证明所有Host目标组合。源码容器构造不按FunctionTarget分支，后续捕获扩展仍需单独证明所有权。

## 全量缺口复核

G01更新Bytes剩余类型对；G02/G03核对Value无Buffer、数值cast及const/atomic守卫；G04保留数值/资源原生环境未知。

G05更新桶链及997剩余键/属性/删除边界；G06核对writable仍拒绝成员容器写，mapping_output的受检内部输出单列；G07核对copy_value的同origin/可选owner、read_reference成功分配后的源规范化及MappingData::drop；G08核对只读45与缺少11。

G09核对prepare_entry的线性默认白名单；G10核对链接器重复祖先/hidden拒绝；G11核对08目标/名称/flags与显式目录；G12核对三类目标、捕获标量白名单、Global绑定和逐次Host注册解析。

G13/G14核对显式初始化/构造及立即逻辑remove，主动析构/延迟删除仍缺；G15核对RuntimeFaultSite仅选定数值来源；G16核对任务守卫仍作用于普通helper；G17更新Mapping循环并保留绑定/属性限制；G18更新组合入口并保留linked/default/init/任务隔离。

G19—G23对照CLI、Host接口及路线图保留真实efun/bootstrap、调度、服务、业务外部契约及旧ABI/动态编译边界；G24保留合成执行、无原版差分/分支测量及无JIT的事实。每项详细来源/相关测试/退出条件见gaps.json；这些条目不是等权工作量或完成比例。
