# 阶段 2P：源码能力独立审计

## 结论与口径

本表对照原路线图“阶段 2：实现值、对象和解释器核心”，以当前源码和合成回归测试为证据。当前是受限解释器，阶段 2 尚未完成；不能由 Rust 的 Rc、错误类型或已支持指令编号推导完整原生语义。

`src/prepared.rs::SUPPORTED_OPCODES`有57项，其中55项属于主语料的73种，4d/4e不在主语料，主语料尚缺18种。55/73是编号交集，**不是功能完成率**。212项既有Rust测试是加载器、单元和合成VM测试的总数，不是212项真实业务执行测试。原始制品不执行。

## 能力与退出条件

下表测试名均来自`tests/vm_core.rs`；引用的源码符号是审计入口，而非声称该模块全部兼容。

| 能力 | 已确认的实现与边界 | 可定位的测试证据 | 阶段2剩余门槛 |
|---|---|---|---|
| 数值与转换 | `src/value.rs::Value/RealBits`保存i32和binary32位模式；VM实现整数环绕、混合数值及受限数值cast。atomic、const返回、非数值cast等准备时拒绝。 | `integer_wrap_and_negation_match_in_debug_and_release`；`mixed_product_avoids_binary64_double_rounding_at_binary32_midpoint`；`numeric_casts_reject_non_numeric_sources_and_const_attributes` | 按原生类型、属性、特殊值及操作数分支逐项登记；拒绝路径不能当作已恢复分支。 |
| 字符串、buffer、范围 | Bytes保存原始字节；程序池按C字符串终止规则导入。Value没有Buffer变体，不能据Bytes声称buffer兼容。成员query仅支持模式0/1及数组/mapping。 | `pool_strings_use_c_termination_without_utf8_conversion`；`mapping_bytes_are_case_sensitive_non_utf8_and_length_sensitive`；`container_type_restrictions_and_access_modes_are_explicit` | 恢复buffer身份、修改/复制、字符串与buffer差异及范围操作的读写、边界和别名规则。 |
| 容器与循环 | `src/container.rs`为共享无环容器，mapping键限Int/Bytes。VM成员引用写禁止Array/Mapping值；并非支持任意嵌套重接。 | `shared_array_and_mapping_member_writes_are_visible_to_aliases`；`member_write_can_store_live_object_and_rejects_cycles_before_change`；`depth_memoizes_shared_subgraphs_and_obeys_existing_limit` | 补齐原生容器键/类型分支、循环形成/复制/释放及实际销毁时机；Rust Rc释放不是原生释放证明。 |
| 稳定槽和引用 | `Cell`与VM Reference区分本地帧、对象及owned成员/临时槽，检查owner和readonly，额度受限。成员引用保活行为有明确原生差异。 | `reference_parameter_cleanup_releases_member_and_temporary_quota`；`member_refs_keep_old_slots_after_variable_rebinding_with_native_release_difference`；`deleted_reference_owner_is_rechecked_before_argument_entry` | 补齐所有引用类型、范围引用、销毁及异常解栈组合；保留差异直到证据和实现闭合。 |
| 参数与返回 | 固定/可选/可变参数、当前对象内引用参数、默认值线性白名单和原子入口检查已有实现；混合空洞及省略引用仍限制。 | `optional_type_check_is_atomic_before_any_default_is_evaluated`；`unsupported_default_code_and_invalid_layouts_fail_preparation`；`reference_numeric_types_are_exact_while_value_parameters_coerce` | 扩展默认表达式及调用形态；补齐返回hook和非局部退出，不能把普通49返回等同全部返回机制。 |
| 继承与分派 | ProgramCatalog显式绑定，无共同祖先继承、槽/变量基址重定位、来源字符串池及metadata恢复。空声明近下后上匹配，排除private/alias候选；同名具体槽保持独立。 | `linked_alias_search_prefers_nearest_lower_and_skips_aliases_private`；`linked_calls_use_body_string_pool_and_restore_caller`；`linked_rejects_diamond_even_when_intervals_are_disjoint` | 明确菱形/重复祖先政策与原生行为；完整名称方法查找和private访问规则仍未实现，链接候选筛选不等于方法访问控制。 |
| 调用、函数值与thisObject | 07实现当前对象固定槽调用及继承基址切换。Value没有函数值，支持集合不含08、41；全局位07在准备时拒绝。 | `nested_calls_preserve_argument_order_and_cleanup_frame_storage`；`linked_multilevel_calls_add_function_bases`；`unsupported_program_shapes_fail_explicitly` | 实现跨对象上下文、thisObject、函数值及间接调用；恢复方法缺失/private/default_interface分支和参数清理，不任意返回Undefined。 |
| 分配、初始化与构造 | ObjectStore::create接收显式值、不运行代码；初始化/构造专用任务按继承顺序执行并可暂停，构造失败逆序destruct。生命周期显式绑定且原型受限。 | `initialization_multiple_parents_share_root_storage_and_keep_source_pools`；`construction_initializes_all_sources_then_runs_creates_in_order`；`construction_failure_rolls_back_failing_layer_and_previous_layers_in_reverse` | 新对象任务不是完整原生clone：仍缺原生命名/注册/克隆调用契约、自动生命周期查找及相关宿主边界。 |
| 主动析构和删除 | ObjectStore::remove立即删除。构造回滚析构可暂停、单层失败继续；宿主取消只清理任务对象。无主动析构或活动对象延迟删除任务。 | `deleting_object_invalidates_pending_object_reference_and_does_not_reuse_id`；`construction_preserves_primary_error_and_continues_after_multiple_destruct_failures`；`construction_rollback_can_pause_and_host_cancellation_does_not_run_more_destructors` | 恢复主动析构、重入、活动帧持有对象、删除排队及最终释放；不能把remove当作原生destruct。 |
| 控制流、异常和预算 | 条件分支、跳转、普通返回和预算续跑已实现；构造任务有局部错误恢复。没有通用catch、foreach/switch全套及通用异常解栈。 | `conditional_flags_choose_direction_and_pop_independently`；`budget_exhaustion_preserves_loop_state_and_can_resume`；`construction_nested_failure_unwinds_temporaries_and_allows_rollback_with_same_quota` | 补齐缺失控制流及嵌套catch；异常时帧、引用、循环游标、返回hook和预算恢复须组合验证，不跨Rust栈longjmp。 |
| 全局与宿主分派 | 准备器仅接受object路径；全局07、全局变量及06未执行。559只用于配置/ABI校验，非559项efun实现。 | `initialization_loops_obey_budget_and_global_input_is_rejected`；`linked_rejects_cycles_repeated_ancestors_and_nonobject_dependencies` | 阶段2负责全局变量/函数的VM语义及06分派、参数、返回/错误契约；阶段3负责真实efun、bootstrap槽绑定、协作调度与I/O。 |

## 审计约束与验收解释

- 支持编号必须与准备器限制和运行时类型分支同时阅读；`UnsupportedFeature/UnsupportedType`是明确边界，不是语义已完成。
- 每个剩余能力需要原生证据、正/反合成测试、暂停恢复及清理测试；跨对象、异常、继承和引用必须有组合场景。
- 全量只读语料比较验证加载结构和统计，不执行真实业务代码，也不证明运行兼容。
- 阶段2只有在主语料73种指令均有实际实现、必要分支闭合、未知核心语义解除后才可标记完成。不得以编号比例、测试总数或阶段命名计算总体完成百分比。
