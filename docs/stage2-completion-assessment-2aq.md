# 阶段2AQ：以2AP为基线的当前完成度重审

阶段2仍未完成。本批保留558项Rust测试、75种限定操作码、73种主语料交集；更新的是能力审计与后续排序，没有改变Rust/API/依赖，没有新增反编译或原版/业务执行。操作码编号交集不能衡量完整兼容。

## 相对2AH的变化

| 缺口 | 当前已补齐的受限路径 | 仍未闭合 |
|---|---|---|
| G01 | Bytes比较、Bytes+Bytes/Int及Bytes += Bytes/Int | Real拼接、其他非数值类型对、一般容器运算 |
| G05/G17 | Int/Bytes原生哈希桶链、Mapping997实时扫描、三元数组复用 | 其他键/属性、公开删除及删除后引用、复杂交叉图和绑定 |
| G12 | Object、Global/c、同步Host/K函数值，标量捕获 | 非标量move/copy、完整源cleanup、继承/hidden及完整生命周期 |
| G18 | 普通组合准备入口兼容Global表/07、06及三类函数值 | Global init、linked/default/任务组合 |
| G06/G07 | 受检Mapping内部容器输出、选定死Object/Function复制规范化 | 一般容器重接/循环回收、提前源清理、最后所有者与引用寿命差异 |

另发现历史2AP运行时说明对“函数值装入容器”的限制表述过宽：当前Array/Mapping值存储允许普通Function，既有tag9测试直接断言容器内容与键拒绝；仍禁止的是非标量捕获。该更正在本次审计记录，历史说明保持冻结；未据此声称所有Host目标容器组合均有专项测试。

Global/c的新调用要求当前绑定仍等于保存ProgramId，是受限Rust政策；已进入帧保留旧代码。Host/K只保存槽号，每次调用读取当前注册与签名，使用05调用者上下文，预检后单次回调；错误不撤销外部副作用，也不自动重放。真实efun尚未由这些接口实现。

[当前矩阵](stage2aq-evidence/matrix.md)逐行更新75种操作码；[变更账本](stage2aq-evidence/review-changes.json)对照2AH并保留历史原文。[缺口目录](stage2aq-evidence/gaps.md)继续使用G01—G24，其中阶段2内19项、阶段外5项；[能力门槛](stage2aq-evidence/capabilities.md)保留C01—C13，12类阶段2门槛均open。partial只标记聚合缺口有进展，不代表关闭。其余Buffer、属性、默认表达式、继承/查找、异常及生命周期等逐项重核，详见[断言复核](stage2aq-evidence/assertion-review.md)。

## 下一批及阶段边界

[优先级](stage2aq-evidence/priorities.md)的唯一首选是P05：复制源规范化与非标量捕获所有权定向取证。标量函数值三类目标已有适配，继续扩展捕获前必须恢复独占move/共享copy、源变化与释放顺序；这可先于调度和真实efun独立调查。P06继承布局/能力组合其次，P04一般异常与主动删除再次。排序是工程判断，不按静态频次推断收益。

[下一批2AR规格](stage2aq-evidence/next-batch.md)固定证据入口、候选类型、复制接点、失败/释放问题和退出条件，仅取证与设计，不直接放宽17捕获白名单。G12完整继承目标依赖G10，但不阻止隔离调查普通捕获路径。

阶段3—6继续承接bootstrap/真实efun/调度、服务协议、业务外部契约及动态编译/旧ABI。所有权、继承、异常和任务组合不能因opcode已出现而跳过。所需原版差分未完成时，只能声明相应合成范围验证。

## 验证与保护

实施前2AP完整收据验证通过，冻结576个保护文件及原入口哈希。2AQ重跑Rust debug/release、fmt、严格Clippy、Python审计测试和全量只读加载器比较；结果、完整命令和交付哈希以[2AQ收据](stage2aq-validation.json)为准。保持唯一已知DBA解压异常，新增异常失败。

复现见[方法](stage2aq-evidence/method.md)。历史收据和原制品均不刷新；新收据验证当前入口，旧2AP静态验证继续校验冻结原生材料。自动工具检查出处与一致性，不替代人工语义审计。
