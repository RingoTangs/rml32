# 2AQ 当前能力与退出门槛

状态基于2AP当前源码；不是2P历史表的原地改写。每个open门槛仍含未解决项，故阶段2未完成。测试为相关合成场景，存在性校验不证明断言充分或原生等价。

## C01：值、数值与属性

状态：open；缺口：G01, G02, G03, G04。

当前：Bytes+Bytes/Int、Bytes全长相等及首NUL前缀排序、Bytes += Bytes/Int已实现；其他混合输入、Real拼接和一般容器运算仍受限。；Value没有Buffer；43/44只接受Array/Mapping，45只接受Bytes/Array。；cast仅数值目标；atomic、const返回、未知参数属性受限；Value不携带原生attrib。；binary32位保存和受限数值适配已测；未进行原生浮点环境/资源错误对照。

定位：[src/numeric.rs:109](../../src/numeric.rs#L109), [src/value.rs:38](../../src/value.rs#L38), [src/vm.rs:2908](../../src/vm.rs#L2908), [src/numeric.rs:27](../../src/numeric.rs#L27), [src/prepared.rs:480](../../src/prepared.rs#L480), [src/numeric.rs:46](../../src/numeric.rs#L46), [src/vm.rs:27](../../src/vm.rs#L27), [src/bytes.rs:25](../../src/bytes.rs#L25)

相关测试：`container_type_restrictions_and_access_modes_are_explicit`, `function_values_vm_equality_uses_identity_and_rejects_arithmetic`, `mixed_product_avoids_binary64_double_rounding_at_binary32_midpoint`, `numeric_casts_reject_non_numeric_sources_and_const_attributes`, `ranges_types_and_reference_operands_remain_explicit_errors`, `real_constant_uses_first_word_only_and_retains_nan_and_signed_zero_bits`, `unknown_flags_const_and_atomic_prototypes_are_rejected`, `bytes_static_evidence_cases_execute_through_vm`, `bytes_compound_add_writes_target_and_returns_full_payload`

证据：[docs/vm-container-semantics.md](../../docs/vm-container-semantics.md), [docs/vm-core-semantics.md](../../docs/vm-core-semantics.md), [docs/vm-function-value-semantics.md](../../docs/vm-function-value-semantics.md), [docs/vm-numeric-semantics.md](../../docs/vm-numeric-semantics.md), [docs/vm-range-runtime.md](../../docs/vm-range-runtime.md), [docs/vm-range-semantics.md](../../docs/vm-range-semantics.md), [docs/vm-bytes-runtime.md](../../docs/vm-bytes-runtime.md), [docs/vm-bytes-compound-runtime.md](../../docs/vm-bytes-compound-runtime.md)

退出：逐tag对恢复操作与错误；Byte成功/边界/别名/预算合成回归，未知分支逐项解决或明确调整目标。；建立独立Buffer/Bytes边界及成员操作契约并验证，不能把Bytes重命名冒充Buffer。；逐种属性和转换补证据；准备、传播与错误组合测试。；记录Rust资源策略；所需数值分支有静态依据及独立边界向量，所需运行对照单独授权。

## C02：容器及图形所有权

状态：open；缺口：G05, G06。

当前：Int/Bytes键已有原生哈希桶链及受限Mapping997实时扫描，输出[key,value,ordinal]并复用三元素数组；其他键/属性、公开删除清空及完整交叉图仍排除。；共享无环Array/Mapping及浅复制已实现；普通成员写仍拒绝Array/Mapping，函数捕获限标量；Mapping循环内部允许受检的无环容器输出。

定位：[src/container.rs:356](../../src/container.rs#L356), [src/vm.rs:795](../../src/vm.rs#L795), [src/container.rs:5](../../src/container.rs#L5), [src/container.rs:418](../../src/container.rs#L418), [src/container.rs:315](../../src/container.rs#L315)

相关测试：`depth_memoizes_shared_subgraphs_and_obeys_existing_limit`, `loops_empty_mapping_is_supported_but_member_binding_remains_unsupported`, `mapping_bytes_are_case_sensitive_non_utf8_and_length_sensitive`, `member_write_can_store_live_object_and_rejects_cycles_before_change`, `mapping_loops_emit_native_order_values_and_ordinal_under_single_step_budget`, `mapping_loops_live_growth_changes_remaining_bucket_scan`, `mapping_loop_existing_array_alias_sees_latest_output_and_zero_budget_does_not_touch_it`, `mapping_loops_normalize_dead_objects_and_allow_nested_container_values`, `mapping_loop_cross_source_output_alias_is_fatal_before_visible_writes`

证据：[docs/vm-container-semantics.md](../../docs/vm-container-semantics.md), [docs/vm-control-flow-semantics.md](../../docs/vm-control-flow-semantics.md), [docs/vm-function-value-semantics.md](../../docs/vm-function-value-semantics.md), [docs/vm-loop-runtime.md](../../docs/vm-loop-runtime.md), [docs/vm-reference-semantics.md](../../docs/vm-reference-semantics.md), [docs/vm-mapping-runtime.md](../../docs/vm-mapping-runtime.md), [docs/stage2al-evidence/mapping-runtime-design.md](../../docs/stage2al-evidence/mapping-runtime-design.md), [docs/stage2an-evidence/function-runtime-design.md](../../docs/stage2an-evidence/function-runtime-design.md)

退出：恢复桶/链/键契约和遍历mutation，确定性顺序、别名与异常清理回归。；恢复允许图形与所有权；别名、循环回收和异常最终释放均有证据及测试。

## C03：稳定槽、引用与范围

状态：open；缺口：G07, G08。

当前：成员Ref保留稳定Cell；Array/Mapping最后header释放仍有已知差异。复制在选定接点规范化死Object及同origin且有已删除creator的Function；不覆盖全部原生提前源cleanup。；45仅普通Bytes/Array只读；无范围Reference，11不在支持集合；Reference不能作为普通值逃逸。

定位：[src/vm.rs:713](../../src/vm.rs#L713), [src/vm.rs:834](../../src/vm.rs#L834), [src/container.rs:138](../../src/container.rs#L138), [src/vm.rs:2413](../../src/vm.rs#L2413), [src/vm.rs:2359](../../src/vm.rs#L2359)

相关测试：`function_values_dead_owner_reference_normalization_is_allocation_atomic`, `member_refs_keep_old_slots_after_variable_rebinding_with_native_release_difference`, `ranges_types_and_reference_operands_remain_explicit_errors`, `references_cannot_escape_via_returns_arguments_or_containers`, `global_function_values_deleted_creator_raw_call_fails_but_copy_is_undefined`, `global_function_values_ownerless_value_survives_deleted_outer_creator_object`, `global_function_values_ownerless_reference_copy_and_last_header_release`

证据：[docs/vm-function-value-runtime.md](../../docs/vm-function-value-runtime.md), [docs/vm-range-semantics.md](../../docs/vm-range-semantics.md), [docs/vm-reference-runtime.md](../../docs/vm-reference-runtime.md), [docs/vm-reference-semantics.md](../../docs/vm-reference-semantics.md), [docs/vm-reference-type-check-runtime.md](../../docs/vm-reference-type-check-runtime.md), [docs/vm-global-function-value-runtime.md](../../docs/vm-global-function-value-runtime.md), [docs/vm-host-function-value-runtime.md](../../docs/vm-host-function-value-runtime.md), [docs/vm-mapping-runtime.md](../../docs/vm-mapping-runtime.md)

退出：对差异逐接点确定保留/修正决策；分配失败与最后owner释放测试。；恢复相关指令/引用类型后实现受控范围写入及解栈清理；区分11未在主语料与语义需求。

## C04：参数、默认与返回

状态：open；缺口：G09, G15。

当前：默认代码限线性字面量、对象读及容器构造；省略引用和参数空洞等受限。；仅明确列出的除零/整数余零来源可捕获；类型/引用/资源/溢出及返回校验错误终止，未实现返回hook。

定位：[src/prepared.rs:388](../../src/prepared.rs#L388), [src/vm.rs:1093](../../src/vm.rs#L1093), [src/vm.rs:278](../../src/vm.rs#L278), [src/vm.rs:2539](../../src/vm.rs#L2539)

相关测试：`catches_return_validation_errors_remain_fatal_with_real_failure_location`, `function_values_catch_unwinds_indirect_callee_numeric_fault`, `numeric_catch_compound_integer_overflow_is_still_fatal_for_division_and_remainder`, `optional_type_check_is_atomic_before_any_default_is_evaluated`, `unsupported_default_code_and_invalid_layouts_fail_preparation`

证据：[docs/vm-catch-semantics.md](../../docs/vm-catch-semantics.md), [docs/vm-numeric-catch-runtime.md](../../docs/vm-numeric-catch-runtime.md), [docs/vm-optional-arguments-runtime.md](../../docs/vm-optional-arguments-runtime.md), [docs/vm-optional-arguments.md](../../docs/vm-optional-arguments.md)

退出：一般表达式、调用和恢复入口按证据分批开放；不得放宽白名单而无入口状态测试。；逐故障位置定性，不按VmErrorKind/文本全量捕获；跨帧清理和hook顺序有证据。

## C05：继承布局与方法查找

状态：open；缺口：G10, G11。

当前：显式目录链接，重复祖先/菱形及隐藏函数拒绝，普通新能力入口不接受继承。；08使用显式完整方法目录及直接Object；String/Array目标、其他flags与NUL名称拒绝；private筛选不是完整访问控制。

定位：[src/prepared.rs:976](../../src/prepared.rs#L976), [src/prepared.rs:989](../../src/prepared.rs#L989), [src/vm.rs:1217](../../src/vm.rs#L1217), [src/prepared.rs:145](../../src/prepared.rs#L145)

相关测试：`linked_rejects_cycles_repeated_ancestors_and_nonobject_dependencies`, `linked_rejects_diamond_even_when_intervals_are_disjoint`, `other_call_bad_targets_and_names_are_not_suppressed`, `other_call_names_are_case_sensitive_raw_bytes_and_nul_is_rejected`, `other_call_private_lookup_and_fallback_preserve_method_argument`

证据：[docs/stage2-completion-evidence/cross-object-design.md](../../docs/stage2-completion-evidence/cross-object-design.md), [docs/vm-cross-object-runtime.md](../../docs/vm-cross-object-runtime.md), [docs/vm-inheritance-linking.md](../../docs/vm-inheritance-linking.md), [docs/vm-inheritance-runtime.md](../../docs/vm-inheritance-runtime.md)

退出：恢复布局/重定位/来源池及隐藏槽契约，菱形正反测试和调用清理。；恢复原生命名及全部约定目标flags；default_interface、拒绝、返回和暂停组合回归。

## C06：调用与函数值

状态：open；缺口：G11, G12。

当前：08使用显式完整方法目录及直接Object；String/Array目标、其他flags与NUL名称拒绝；private筛选不是完整访问控制。；05/17支持显式启用的Object、Global/c、同步Host/K及标量捕获。Global/c新调用要求保存ProgramId仍绑定；Host每次读取当前槽注册/签名；非标量、引用原型、继承/hidden及string/OS仍排除。 普通Function值已有Array/Mapping值存储路径；这与把非标量值捕获进Function不同。

定位：[src/vm.rs:1217](../../src/vm.rs#L1217), [src/prepared.rs:145](../../src/prepared.rs#L145), [src/vm.rs:1492](../../src/vm.rs#L1492), [src/vm.rs:1617](../../src/vm.rs#L1617), [src/vm.rs:1875](../../src/vm.rs#L1875)

相关测试：`function_values_prepare_all_creation_flag_bytes_and_direct_private_slots`, `function_values_reject_non_function_targets_even_with_may_null`, `function_values_reject_non_scalar_captures_without_consuming_operands`, `other_call_bad_targets_and_names_are_not_suppressed`, `other_call_names_are_case_sensitive_raw_bytes_and_nul_is_rejected`, `other_call_private_lookup_and_fallback_preserve_method_argument`, `global_function_values_clear_replace_and_restore_binding_before_new_call`, `host_function_values_each_call_reads_replacement_callback_and_signature`, `host_function_values_context_comes_from_invoker_not_creator`, `global_function_values_non_scalar_captures_stay_rejected_before_binding_lookup`, `function_values_tag9_arguments_returns_containers_and_mapping_key_rejection`

证据：[docs/stage2-completion-evidence/cross-object-design.md](../../docs/stage2-completion-evidence/cross-object-design.md), [docs/stage2af-evidence/function-runtime-design.md](../../docs/stage2af-evidence/function-runtime-design.md), [docs/vm-cross-object-runtime.md](../../docs/vm-cross-object-runtime.md), [docs/vm-function-value-semantics.md](../../docs/vm-function-value-semantics.md), [docs/vm-global-function-value-runtime.md](../../docs/vm-global-function-value-runtime.md), [docs/vm-host-function-value-runtime.md](../../docs/vm-host-function-value-runtime.md), [docs/stage2an-evidence/function-runtime-design.md](../../docs/stage2an-evidence/function-runtime-design.md)

退出：恢复原生命名及全部约定目标flags；default_interface、拒绝、返回和暂停组合回归。；分目标/捕获类型建立契约，保持store身份/原子性及循环图边界后扩展。

## C07：初始化、clone及构造

状态：open；缺口：G13, G16。

当前：显式create/begin_initialize/begin_construct及绑定，尚非原生clone/load_object契约。；catch/loop/range/06/05/17等在任务内受限；不能由普通函数通过内部helper绕过。

定位：[src/object.rs:195](../../src/object.rs#L195), [src/vm.rs:489](../../src/vm.rs#L489), [src/vm.rs:1448](../../src/vm.rs#L1448), [src/vm.rs:2196](../../src/vm.rs#L2196), [src/prepared.rs:1187](../../src/prepared.rs#L1187)

相关测试：`construction_initializes_all_sources_then_runs_creates_in_order`, `function_values_lifecycle_context_guards_precede_operand_access`, `lifecycle_preparation_rejects_declarations_atomic_and_unsupported_prototypes`, `ranges_lifecycle_helper_rejection_cleans_initialization_and_construction`

证据：[docs/vm-catch-runtime.md](../../docs/vm-catch-runtime.md), [docs/vm-construction-runtime.md](../../docs/vm-construction-runtime.md), [docs/vm-function-value-runtime.md](../../docs/vm-function-value-runtime.md), [docs/vm-loop-runtime.md](../../docs/vm-loop-runtime.md), [docs/vm-object-lifecycle.md](../../docs/vm-object-lifecycle.md)

退出：恢复VM侧任务与加载边界；原生efun注册另归阶段3，不能以宿主分配冒充clone。；逐组合开放，失败保留primary错误、清理年轻帧/引用及恢复阶段。

## C08：主动析构及删除

状态：open；缺口：G14。

当前：remove立即删除逻辑对象；函数头不保活对象；只有构造失败回滚析构，无主动析构任务。

定位：[src/object.rs:265](../../src/object.rs#L265), [src/vm.rs:221](../../src/vm.rs#L221)

相关测试：`deleting_object_invalidates_pending_object_reference_and_does_not_reuse_id`, `function_values_deleted_owner_raw_call_fails_but_later_copy_is_undefined`

证据：[docs/vm-function-value-runtime.md](../../docs/vm-function-value-runtime.md), [docs/vm-object-lifecycle-evidence/deletion-findings.md](../../docs/vm-object-lifecycle-evidence/deletion-findings.md), [docs/vm-object-lifecycle.md](../../docs/vm-object-lifecycle.md)

退出：恢复主动析构状态机和延迟删除，暂停/catch/函数值保留owner组合测试。

## C09：控制流及异常

状态：open；缺口：G15, G17。

当前：仅明确列出的除零/整数余零来源可捕获；类型/引用/资源/溢出及返回校验错误终止，未实现返回hook。；循环支持整数±1、Array997及受限Mapping997；负局部控制槽与既有局部/参数/Object/Global根绑定可用，成员/临时绑定和控制别名仍拒绝；switch限Int/Bytes普通值。

定位：[src/vm.rs:278](../../src/vm.rs#L278), [src/vm.rs:2539](../../src/vm.rs#L2539), [src/vm.rs:2185](../../src/vm.rs#L2185), [src/vm.rs:2714](../../src/vm.rs#L2714)

相关测试：`catches_return_validation_errors_remain_fatal_with_real_failure_location`, `function_values_catch_unwinds_indirect_callee_numeric_fault`, `loops_empty_mapping_is_supported_but_member_binding_remains_unsupported`, `numeric_catch_compound_integer_overflow_is_still_fatal_for_division_and_remainder`, `switch_number_cases_match_only_exact_ints_and_default_consumes_selector`, `mapping_loops_support_existing_global_and_object_bindings`, `mapping_loop_helper_unwinds_through_existing_numeric_catch`

证据：[docs/vm-catch-semantics.md](../../docs/vm-catch-semantics.md), [docs/vm-control-flow-semantics.md](../../docs/vm-control-flow-semantics.md), [docs/vm-loop-runtime.md](../../docs/vm-loop-runtime.md), [docs/vm-numeric-catch-runtime.md](../../docs/vm-numeric-catch-runtime.md), [docs/vm-switch-runtime.md](../../docs/vm-switch-runtime.md), [docs/vm-mapping-runtime.md](../../docs/vm-mapping-runtime.md)

退出：逐故障位置定性，不按VmErrorKind/文本全量捕获；跨帧清理和hook顺序有证据。；补证据后覆盖绑定、属性及嵌套暂停/解栈；Mapping另见G05。

## C10：全局与能力组合

状态：open；缺口：G18。

当前：组合准备入口已同时启用Global表/07、直接06和Object/Global/Host函数值；旧入口不扩大。Global init、继承入口与任务内能力组合仍未开放。

定位：[src/prepared.rs:195](../../src/prepared.rs#L195), [src/prepared.rs:1187](../../src/prepared.rs#L1187), [src/prepared.rs:691](../../src/prepared.rs#L691)

相关测试：`function_values_prepare_opt_in_tags_and_host_isolation`, `global_calls_global_init_is_ignored_and_default_whitelist_does_not_expand`, `host_function_values_identity_opaque_forwarding_and_combined_06_object_global_paths`, `host_function_values_all_flags_and_old_entries_remain_isolated`, `global_function_values_default_and_lifecycle_preparation_remain_closed`

证据：[docs/vm-efun-runtime.md](../../docs/vm-efun-runtime.md), [docs/vm-function-value-runtime.md](../../docs/vm-function-value-runtime.md), [docs/vm-global-call-runtime.md](../../docs/vm-global-call-runtime.md), [docs/vm-host-function-value-runtime.md](../../docs/vm-host-function-value-runtime.md), [docs/vm-global-function-value-runtime.md](../../docs/vm-global-function-value-runtime.md)

退出：逐组合定义入口与初始化契约；保持上下文隔离、原子性和暂停重绑测试。

## C11：宿主VM边界与后端差异

状态：open；缺口：G15, G18, G24。

当前：仅明确列出的除零/整数余零来源可捕获；类型/引用/资源/溢出及返回校验错误终止，未实现返回hook。；组合准备入口已同时启用Global表/07、直接06和Object/Global/Host函数值；旧入口不扩大。Global init、继承入口与任务内能力组合仍未开放。；所有执行为合成测试，语料比较只证结构；没有原版运行对照或分支覆盖测量，JIT未实现。

定位：[src/vm.rs:278](../../src/vm.rs#L278), [src/vm.rs:2539](../../src/vm.rs#L2539), [src/prepared.rs:195](../../src/prepared.rs#L195), [src/prepared.rs:1187](../../src/prepared.rs#L1187), [src/vm.rs:1](../../src/vm.rs#L1), [src/main.rs:266](../../src/main.rs#L266), [src/prepared.rs:691](../../src/prepared.rs#L691)

相关测试：`catches_return_validation_errors_remain_fatal_with_real_failure_location`, `function_values_budget_creation_call_entry_and_shared_repeated_calls`, `function_values_catch_unwinds_indirect_callee_numeric_fault`, `function_values_prepare_opt_in_tags_and_host_isolation`, `global_calls_global_init_is_ignored_and_default_whitelist_does_not_expand`, `numeric_catch_compound_integer_overflow_is_still_fatal_for_division_and_remainder`, `host_function_values_identity_opaque_forwarding_and_combined_06_object_global_paths`, `host_function_values_all_flags_and_old_entries_remain_isolated`, `global_function_values_default_and_lifecycle_preparation_remain_closed`

证据：[docs/magic_Linux32-rust-rewrite-roadmap.md](../../docs/magic_Linux32-rust-rewrite-roadmap.md), [docs/stage2ag-evidence/method.md](../../docs/stage2ag-evidence/method.md), [docs/vm-catch-semantics.md](../../docs/vm-catch-semantics.md), [docs/vm-efun-runtime.md](../../docs/vm-efun-runtime.md), [docs/vm-function-value-runtime.md](../../docs/vm-function-value-runtime.md), [docs/vm-global-call-runtime.md](../../docs/vm-global-call-runtime.md), [docs/vm-numeric-catch-runtime.md](../../docs/vm-numeric-catch-runtime.md), [docs/vm-host-function-value-runtime.md](../../docs/vm-host-function-value-runtime.md), [docs/vm-global-function-value-runtime.md](../../docs/vm-global-function-value-runtime.md)

退出：逐故障位置定性，不按VmErrorKind/文本全量捕获；跨帧清理和hook顺序有证据。；逐组合定义入口与初始化契约；保持上下文隔离、原子性和暂停重绑测试。；阶段2限定解释器目标需明确说明后端差异；所需运行对照独立授权，语义稳定后测量性能，不以JIT缺失冒充某种指令缺失。

## C12：受控验证及退出口径

状态：open；缺口：G04, G24。

当前：binary32位保存和受限数值适配已测；未进行原生浮点环境/资源错误对照。；所有执行为合成测试，语料比较只证结构；没有原版运行对照或分支覆盖测量，JIT未实现。

定位：[src/numeric.rs:46](../../src/numeric.rs#L46), [src/vm.rs:27](../../src/vm.rs#L27), [src/vm.rs:1](../../src/vm.rs#L1), [src/main.rs:266](../../src/main.rs#L266)

相关测试：`function_values_budget_creation_call_entry_and_shared_repeated_calls`, `mixed_product_avoids_binary64_double_rounding_at_binary32_midpoint`, `real_constant_uses_first_word_only_and_retains_nan_and_signed_zero_bits`

证据：[docs/magic_Linux32-rust-rewrite-roadmap.md](../../docs/magic_Linux32-rust-rewrite-roadmap.md), [docs/stage2ag-evidence/method.md](../../docs/stage2ag-evidence/method.md), [docs/vm-numeric-semantics.md](../../docs/vm-numeric-semantics.md), [docs/vm-range-runtime.md](../../docs/vm-range-runtime.md)

退出：记录Rust资源策略；所需数值分支有静态依据及独立边界向量，所需运行对照单独授权。；阶段2限定解释器目标需明确说明后端差异；所需运行对照独立授权，语义稳定后测量性能，不以JIT缺失冒充某种指令缺失。

## C13：阶段外依赖

状态：outside_stage2；缺口：G19, G20, G21, G22, G23。

当前：显式同步06与间接Host/K已有合成回调；559槽容量和Host槽号函数值不代表真实efun、预置全局bootstrap或编译启动能力已经实现。；只有同步callback和调用方预算；无协作执行队列/定时器/异步等待恢复。；当前CLI是只读检查器，无可运行AAA/CCS/DBA/GS/CSA。；已有静态业务调用链，未建真实数据库/HTTP/账户运行适配。；Rust解析/VM目标为Magic559；LegacyAuth389未实现解释器，动态编译/execute_data不属于现有VM。

定位：[src/host.rs:101](../../src/host.rs#L101), [src/object.rs:57](../../src/object.rs#L57), [src/host.rs:101](../../src/host.rs#L101), [src/vm.rs:957](../../src/vm.rs#L957), [src/main.rs:266](../../src/main.rs#L266), [src/main.rs:266](../../src/main.rs#L266), [src/program.rs:6](../../src/program.rs#L6), [src/prepared.rs:763](../../src/prepared.rs#L763)

相关测试：`host_pause_rebinding_and_into_objects_keep_registry_and_never_replay`, `host_preparation_requires_opt_in_and_validates_signed_operands`, `host_function_values_identity_opaque_forwarding_and_combined_06_object_global_paths`

证据：[docs/magic_Linux32-rust-compatibility-analysis.md](../../docs/magic_Linux32-rust-compatibility-analysis.md), [docs/magic_Linux32-rust-rewrite-roadmap.md](../../docs/magic_Linux32-rust-rewrite-roadmap.md), [docs/stage2ad-evidence/host-runtime-design.md](../../docs/stage2ad-evidence/host-runtime-design.md), [docs/vm-efun-runtime.md](../../docs/vm-efun-runtime.md), [docs/vm-host-function-value-runtime.md](../../docs/vm-host-function-value-runtime.md)

退出：按首条受控链恢复真实efun/bootstrap；不得以合成回调声称原生efun兼容。；确定性调度契约和事件顺序测试，外部I/O经受控队列恢复。；按服务入口受控启动、分片/粘包/断线测试；阶段2不能背负整服完成定义。；取得外部契约后验证登录及读写故障；不猜建表或真实凭据。；ABI隔离和动态能力按阶段6恢复；启动必需编译子集属于阶段3前置。
