# 2AS设计决策

采用的Rust设计政策，不是原生新增观察；当前运行时未修改。

## D01：按实际持有槽计数

一个活槽载荷对Function头贡献一份所有权；槽引用别名只增加该槽refs。Array/Mapping共享header不重复计算同一成员槽。

边界：原生任意ref/execRef状态机未由此恢复；Object物理保活排除。

依据：[F05](../stage2ar-evidence/findings.md#f05), [F06](../stage2ar-evidence/findings.md#f06), [F07](../stage2ar-evidence/findings.md#f07)

## D02：观察与宿主持有分型

ValueSnapshot不可调用/导入且不持有语义根；HostValue是opaque、不可隐式Clone的store root，retain显式创建新root，Drop释放。

边界：旧可回传Value快照不能静默改义，后续统一迁移，不保留会绕开计数的适配器。

依据：[F21](../stage2ar-evidence/findings.md#f21)

## D03：调用目标移动到激活持有者

05目标从计算槽raw move到临时激活槽，所有权净值不变；别名源看到Undefined。依据此时语义头计数选择唯一/共享。

边界：Rust仍在全部可报告预检后发布提交；不复刻原生失败时已消费操作数。

依据：[F04](../stage2ar-evidence/findings.md#f04), [F05](../stage2ar-evidence/findings.md#f05), [F06](../stage2ar-evidence/findings.md#f06)

## D04：私有稳定捕获槽

FunctionData使用稳定私有捕获Cell和固定bound_count；共享COPY可规范化源，唯一move先清空捕获再释放头。

边界：捕获槽不提供Reference逃逸或公开可变访问。

依据：[F03](../stage2ar-evidence/findings.md#f03), [F09](../stage2ar-evidence/findings.md#f09), [F11](../stage2ar-evidence/findings.md#f11)

## D05：源可改写复制计划

计划包含源Cell规范化及目标发布；同Cell同值合并，不同值冲突拒绝；先预留、检查全部借用及投影图，再批量提交和释放旧载荷。

边界：模型以候选世界发布实现原子性，不能据此证明Rust借用/分配实现正确。

依据：[F08](../stage2ar-evidence/findings.md#f08), [F12](../stage2ar-evidence/findings.md#f12), [F13](../stage2ar-evidence/findings.md#f13)

## D06：COPY与raw move显式区分

局部/Object/Global普通读取、deref、成员及共享展开使用COPY；17、唯一展开和选定非const普通返回raw move不追加规范化。

边界：const、转换、返回hook和一般默认表达式不纳入。

依据：[F03](../stage2ar-evidence/findings.md#f03), [F05](../stage2ar-evidence/findings.md#f05), [F08](../stage2ar-evidence/findings.md#f08), [F16](../stage2ar-evidence/findings.md#f16), [F22](../stage2ar-evidence/findings.md#f22)

## D07：Function边进入投影图

所有构造/写入/Host结果/循环输出在最终拟提交图上遍历Function捕获边；灰节点回边拒绝，黑节点共享允许。

边界：不实现循环GC。一般Array/Mapping成员重接继续拒绝，即使图无环。

依据：[F17](../stage2ar-evidence/findings.md#f17), [F21](../stage2ar-evidence/findings.md#f21)

## D08：释放规则保持容器差异

Array最终header只unref槽；Mapping先清节点载荷再解除header边，外部Ref可保留空槽。最终Function释放捕获再放creator/头。

边界：模型省略物理Map节点链和原生creator对象ref/execRef，不宣称完整生命周期等价。

依据：[F11](../stage2ar-evidence/findings.md#f11), [F14](../stage2ar-evidence/findings.md#f14), [F15](../stage2ar-evidence/findings.md#f15), [F23](../stage2ar-evidence/findings.md#f23)

## D09：资源与预算

保留65536项/64层；扩展图预算262144个不同header/槽节点，每次投影检查计数；宿主root默认上限65536，均为Rust政策。纯标量Function叶深度0，包含header捕获的Function计一层。

边界：生产图预算按受影响图而非全store；有限模型对整个小世界计数更保守。旧container depth诊断保留原度量。

依据：[F21](../stage2ar-evidence/findings.md#f21)

## D10：Host borrowed与retained

回调仅借用不可逃逸BorrowedValue；保存到回调外必须经HostScope.retain。仅该能力能登记root，不能重入VM或变更绑定。

边界：回调外部副作用仍不回滚/不重放；仅合成同步Host。

依据：[F21](../stage2ar-evidence/findings.md#f21)

## D11：导入与entry COPY分离

export/import/retain是opaque原始载荷所有权传输，不自行规范化；Vm.start先导入临时槽再按entry COPY规则规范化该槽，外部HostValue root不被当作同一源Cell。

边界：这是宿主边界政策，不是原生external root ABI；真实efun排除。

依据：[F08](../stage2ar-evidence/findings.md#f08), [F21](../stage2ar-evidence/findings.md#f21)

## D12：完成观察不隐式导出

run完成后重复返回CompletionMeta；take_returned只一次将完成槽移动成HostValue，重复取值明确失败；需要副本时显式retain。

边界：这是后续源码不兼容迁移，本批不改真实API。

依据：[F16](../stage2ar-evidence/findings.md#f16), [F21](../stage2ar-evidence/findings.md#f21)

## D13：后续分批落地

T01宿主/观察边界与计数；T02稳定捕获槽和复制事务；T03图验证；T04新入口受限容器捕获；T05直接Object/Function捕获保持阻塞。

边界：2AS只设计/模型；不可因模型通过直接开启非标量捕获。

依据：[F20](../stage2ar-evidence/findings.md#f20), [F21](../stage2ar-evidence/findings.md#f21), [F23](../stage2ar-evidence/findings.md#f23)

## D14：旧入口不扩大

未来新增prepare_program_with_container_captures及Global twin，以GlobalBankSchema签名组合既有callable能力；旧入口仍只标量捕获，间接调用按实际capture_profile守卫。

边界：T04前置T01—T03及真实Rust失败/释放/Host回归；继承/任务/default仍排除。

依据：[F21](../stage2ar-evidence/findings.md#f21)

## D15：无失败Drop与借用epoch

HostValue::Drop只置预分配lease状态retired标志；有内部借用时保留待释放registry root，外层epoch结束后排空。在排空前禁止新的VM/retain/import操作；不在Drop分配或返回错误。

边界：模型retire不注入资源失败，Python进程OOM不在保证内；这是Rust设计要求，非原生调度证明。

依据：[F21](../stage2ar-evidence/findings.md#f21)
