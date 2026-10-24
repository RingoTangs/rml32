# 2AS对2AR案例的映射

modeled仅指独立模型内的指定不变量；partial-model明确不覆盖全部原生语义；未解决项仍未解决。

| 案例 | 原依据 | 模型范围 | 测试 | 限制 |
|---|---|---|---|---|
| [C01](../stage2ar-evidence/expected-cases.md#c01--capture-create) | static-inference | modeled | test_scalar_capture_bits_and_bytes_are_preserved | 模型只检查raw载荷转移，不模拟原生分配/指令执行。 |
| [C02](../stage2ar-evidence/expected-cases.md#c02--capture-create) | static-inference | modeled | test_scalar_capture_bits_and_bytes_are_preserved | 模型只检查raw载荷转移，不模拟原生分配/指令执行。 |
| [C03](../stage2ar-evidence/expected-cases.md#c03--capture-create) | static-inference | modeled | test_scalar_capture_bits_and_bytes_are_preserved | 模型只检查raw载荷转移，不模拟原生分配/指令执行。 |
| [C04](../stage2ar-evidence/expected-cases.md#c04--capture-create) | static-inference | modeled | test_scalar_capture_bits_and_bytes_are_preserved | 模型只检查raw载荷转移，不模拟原生分配/指令执行。 |
| [C05](../stage2ar-evidence/expected-cases.md#c05--capture-create) | static-inference | modeled | test_scalar_capture_bits_and_bytes_are_preserved | 模型只检查raw载荷转移，不模拟原生分配/指令执行。 |
| [C06](../stage2ar-evidence/expected-cases.md#c06--capture-create) | static-inference | modeled | test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 受限浅header共享与唯一展开；不代表真实VM捕获已开放。 |
| [C07](../stage2ar-evidence/expected-cases.md#c07--capture-create) | static-inference | modeled | test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 受限浅header共享与唯一展开；不代表真实VM捕获已开放。 |
| [C08](../stage2ar-evidence/expected-cases.md#c08--capture-create) | static-inference | partial-model | test_raw_create_preserves_already_dead_object_or_function_payload | 只表示raw载荷及creator标记，不模拟Object物理ref/execRef。 |
| [C09](../stage2ar-evidence/expected-cases.md#c09--capture-create) | static-inference | partial-model | test_raw_create_preserves_already_dead_object_or_function_payload | 只表示raw载荷及creator标记，不模拟Object物理ref/execRef。 |
| [C10](../stage2ar-evidence/expected-cases.md#c10--capture-create) | static-inference | partial-model | test_raw_create_preserves_already_dead_object_or_function_payload | 只表示raw载荷及creator标记，不模拟Object物理ref/execRef。 |
| [C11](../stage2ar-evidence/expected-cases.md#c11--capture-create) | static-inference | partial-model | test_raw_create_preserves_already_dead_object_or_function_payload | 只表示raw载荷及creator标记，不模拟Object物理ref/execRef。 |
| [C12](../stage2ar-evidence/expected-cases.md#c12--capture-create) | static-inference | modeled | test_function_capture_cannot_accept_reference_operand | Reference形态边界；无任意引用逃逸。 |
| [C13](../stage2ar-evidence/expected-cases.md#c13--capture-create) | design | native-boundary | — | 模型没有隐藏Loop载荷；后续运行时继续拒绝，不猜测其捕获合法性。 |
| [C14](../stage2ar-evidence/expected-cases.md#c14--capture-create) | static-inference | partial-model | test_every_create_checkpoint_preserves_state_on_failure | 只检查Rust候选发布政策；非原生allocator顺序证明。 |
| [C15](../stage2ar-evidence/expected-cases.md#c15--capture-create) | unresolved | unresolved | — | 保留2AR原生未知；模型失败原子性或拒环不替代该证据。 |
| [C16](../stage2ar-evidence/expected-cases.md#c16--head-expansion) | static-inference | modeled | test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 受限浅header共享与唯一展开；不代表真实VM捕获已开放。 |
| [C17](../stage2ar-evidence/expected-cases.md#c17--head-expansion) | static-inference | partial-model | test_array_capture_shares_header_and_retains_member_identity, test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 共享Array已有专门断言；Mapping覆盖浅identity/unique，未穷尽所有shared Map场景。 |
| [C18](../stage2ar-evidence/expected-cases.md#c18--head-expansion) | static-inference | modeled | test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 受限浅header共享与唯一展开；不代表真实VM捕获已开放。 |
| [C19](../stage2ar-evidence/expected-cases.md#c19--head-expansion) | static-inference | partial-model | test_array_capture_shares_header_and_retains_member_identity, test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 共享Array已有专门断言；Mapping覆盖浅identity/unique，未穷尽所有shared Map场景。 |
| [C20](../stage2ar-evidence/expected-cases.md#c20--head-expansion) | static-inference | partial-model | test_unique_move_does_not_normalize_dead_object_capture | 展开点raw区别已建模，Object最终生命周期未建模。 |
| [C21](../stage2ar-evidence/expected-cases.md#c21--head-expansion) | static-inference | partial-model | test_shared_copy_normalizes_actual_dead_object_capture | 存活共享捕获源规范化；无原生Object释放计数。 |
| [C22](../stage2ar-evidence/expected-cases.md#c22--head-expansion) | static-inference | partial-model | test_raw_create_preserves_already_dead_object_or_function_payload | 只表示raw载荷及creator标记，不模拟Object物理ref/execRef。 |
| [C23](../stage2ar-evidence/expected-cases.md#c23--head-expansion) | static-inference | partial-model | test_function_dead_creator_normalization_is_distinct_from_outer_creator | creator删除标记与COPY；不覆盖完整删除重入。 |
| [C24](../stage2ar-evidence/expected-cases.md#c24--head-expansion) | static-inference | partial-model | test_ownerless_function_is_not_dead, test_final_function_release_cascades_capture_cleanup_once | 头身份/释放模型；直接Function捕获仍不准实施。 |
| [C25](../stage2ar-evidence/expected-cases.md#c25--head-expansion) | static-inference | partial-model | test_ownerless_function_is_not_dead, test_final_function_release_cascades_capture_cleanup_once | 头身份/释放模型；直接Function捕获仍不准实施。 |
| [C26](../stage2ar-evidence/expected-cases.md#c26--head-expansion) | design | modeled | test_observe_and_python_deepcopy_do_not_change_semantic_branch | 纯观察不增根；明确retain则是有意所有权变化。 |
| [C27](../stage2ar-evidence/expected-cases.md#c27--head-expansion) | static-inference | partial-model | test_shared_call_copies_and_preserves_capture_order, test_bound_arguments_precede_new_arguments_and_count_is_checked | 逻辑bound-before-new已覆盖，不模拟native向下栈地址布局或所有原型。 |
| [C28](../stage2ar-evidence/expected-cases.md#c28--copy-sites) | static-inference | modeled | test_shallow_container_copy_does_not_normalize_member | header COPY不递归清理子值。 |
| [C29](../stage2ar-evidence/expected-cases.md#c29--copy-sites) | static-inference | partial-model | test_shared_copy_normalizes_actual_dead_object_capture | 统一槽复制计划，不模拟具体Global bank/继承Object定位。 |
| [C30](../stage2ar-evidence/expected-cases.md#c30--copy-sites) | static-inference | partial-model | test_shared_copy_normalizes_actual_dead_object_capture | 统一槽复制计划，不模拟具体Global bank/继承Object定位。 |
| [C31](../stage2ar-evidence/expected-cases.md#c31--copy-sites) | static-inference | modeled | test_dereference_normalizes_source_visible_through_other_alias | 源清理后释放被消费Ref root；别名仍见Undefined。 |
| [C32](../stage2ar-evidence/expected-cases.md#c32--copy-sites) | static-inference | modeled | test_selected_member_copy_normalizes_both_container_kinds | 仅现存选定成员，未模拟查询最终attrib。 |
| [C33](../stage2ar-evidence/expected-cases.md#c33--copy-sites) | static-inference | modeled | test_selected_member_copy_normalizes_both_container_kinds | 仅现存选定成员，未模拟查询最终attrib。 |
| [C34](../stage2ar-evidence/expected-cases.md#c34--copy-sites) | static-inference | native-boundary | — | 模型用成员索引而非Mapping键查找，不能验证missing-key不插入。 |
| [C35](../stage2ar-evidence/expected-cases.md#c35--release) | static-inference | modeled | test_array_final_header_preserves_referenced_element_payload | 外部Ref保留槽和payload。 |
| [C36](../stage2ar-evidence/expected-cases.md#c36--release) | static-inference | partial-model | test_mapping_final_header_clears_referenced_value | 保留空槽但不模拟原生节点延迟链。 |
| [C37](../stage2ar-evidence/expected-cases.md#c37--release) | static-inference | partial-model | test_final_function_release_cascades_capture_cleanup_once | 模型捕获/头释放一次；不证明非null creator物理unref。 |
| [C38](../stage2ar-evidence/expected-cases.md#c38--release) | static-inference | modeled | test_final_function_release_cascades_capture_cleanup_once, test_reference_alias_owns_cell_not_another_function_payload | raw move清空captured slots，头释放不双重释放payload。 |
| [C39](../stage2ar-evidence/expected-cases.md#c39--return) | static-inference | modeled | test_return_move_does_not_insert_copy_normalization | 只建模已在结果槽中的简单raw move。 |
| [C40](../stage2ar-evidence/expected-cases.md#c40--return) | unresolved | unresolved | — | 保留2AR原生未知；模型失败原子性或拒环不替代该证据。 |
| [C41](../stage2ar-evidence/expected-cases.md#c41--failure) | static-inference | native-boundary | — | 原生callee/catch/逐参转换/栈溢出顺序保留2AR证据；独立模型不模拟执行帧或扩大catch。 |
| [C42](../stage2ar-evidence/expected-cases.md#c42--failure) | static-inference | native-boundary | — | 原生callee/catch/逐参转换/栈溢出顺序保留2AR证据；独立模型不模拟执行帧或扩大catch。 |
| [C43](../stage2ar-evidence/expected-cases.md#c43--failure) | static-inference | native-boundary | — | 原生callee/catch/逐参转换/栈溢出顺序保留2AR证据；独立模型不模拟执行帧或扩大catch。 |
| [C44](../stage2ar-evidence/expected-cases.md#c44--failure) | static-inference | native-boundary | — | 原生callee/catch/逐参转换/栈溢出顺序保留2AR证据；独立模型不模拟执行帧或扩大catch。 |
| [C45](../stage2ar-evidence/expected-cases.md#c45--failure) | static-inference | native-boundary | — | 原生callee/catch/逐参转换/栈溢出顺序保留2AR证据；独立模型不模拟执行帧或扩大catch。 |
| [C46](../stage2ar-evidence/expected-cases.md#c46--failure) | unresolved | unresolved | — | 保留2AR原生未知；模型失败原子性或拒环不替代该证据。 |
| [C47](../stage2ar-evidence/expected-cases.md#c47--graph) | design | modeled | test_shared_dag_is_accepted_not_misclassified_as_cycle, test_mapping_capture_is_shallow_and_both_container_cycles_are_checked | 无环共享与Function边检查；图内header身份不深复制。 |
| [C48](../stage2ar-evidence/expected-cases.md#c48--graph) | unresolved | unresolved | — | 保留2AR原生未知；模型失败原子性或拒环不替代该证据。 |
| [C49](../stage2ar-evidence/expected-cases.md#c49--policy) | design | modeled | test_foreign_origin_is_rejected_before_publication | Rust origin政策，不是原生ABI事实。 |
| [C50](../stage2ar-evidence/expected-cases.md#c50--policy) | design | native-boundary | — | 模型无真实指令预算/Host回调；零预算和单步边界由未来Rust验收保留。 |
| [C51](../stage2ar-evidence/expected-cases.md#c51--policy) | design | native-boundary | — | 模型无真实指令预算/Host回调；零预算和单步边界由未来Rust验收保留。 |
