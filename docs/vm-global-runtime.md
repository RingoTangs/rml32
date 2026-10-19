# 阶段 2AB：显式全局表与 35/36

本阶段实现阶段 2AA [第一批规格](stage2aa-evidence/global-runtime-design.md)。[原生观察](vm-global-semantics.md)与本页 Rust 策略分别保留；没有重新运行目标、启动脚本或真实业务字节码。当前支持 72 种限定操作码，主语料交集 70，缺项为 05、06、17；编号数量不是功能完成率。

## 接口与边界

新增 `GlobalBankSchema { slots: usize }`、`prepare_program_with_globals(image, schema)`，以及 `ObjectStore::{register_globals, remove_globals, global_values}`。每个 store 至多一个当前表，容量为 0..=32768；空表不同于未安装。程序可以先注册，执行 35/36 时才要求表存在并与当前程序 schema 容量完全相同。不同对象共享表而对象变量保持独立。

`ReferenceKind::Global` 是公共枚举扩展，下游穷尽匹配需要增加分支。其他已有方法签名、VmLimits、VmErrorKind、依赖保持不变。无新增 unsafe。

仅新入口准备单 constitution、无继承和隐藏函数的 Object 普通函数。旧入口及链接/初始化/构造准备器保持无 schema 并拒绝 35/36。init 与旧 prepare_program 一样忽略；默认表达式白名单不扩展，生命周期辅助调用仍在运行时拒绝。Global 格式、全局位 07、06、真实初始化/bootstrap、声明属性及 const 槽均未开放。

## 存储、寿命与失败策略

安装先逐值校验容器深度，再分配全部 `Rc<Cell>`，成功后一次发布。表不是额外数组层；深度 64 的合法值仍可安装。重复安装、未安装时移除或快照返回 InvalidState，超容量返回 Limit。安装不执行脚本，不承诺标准库 OOM 恢复或 Cell ID 回退。

35 检查普通函数/非生命周期上下文、栈容量、schema、当前表及索引后浅复制。直接失效 Object 返回 Undefined；先分配结果 Cell，随后提交源槽规范化，最后入栈。失败保持源槽和操作数；不递归清理嵌套容器。此顺序是 Rust 的安全策略，与原生提前读取指针/修改栈的布局有差异。

36 不读取源槽，创建强持有 Cell 的 Global 根引用，不占成员/临时引用配额。已有解引用、赋值、复合赋值、引用参数、10 形态检查和受限循环绑定复用已有机制。

移除仅释放表的根所有权，不 clear 槽。旧根引用可继续读写；重装创建新 Cell，后续 35/36 使用新表。旧引用解析不检查当前表、schema 或代际。历史槽仍受栈/帧寿命约束，不可逃逸为公开 Value；表容量不代表所有历史引用槽的总数。into_objects 只保留当前表，VM 执行中的脱离表引用随执行状态销毁。

Array/Mapping 成员仍占既有 lease 配额，仍限制标量写入。对脱离表最后一个 Mapping 根执行 44 时，根释放会清空节点，成员引用有效但值为 Undefined；Array 成员保留原值。不额外保留容器头或添加表存活检查。

每条指令消耗一份预算，零预算无副作用。错误保留现有 source/program/object/function/PC/offset，35/36 消息注明槽索引；缺 schema 为 UnsupportedFeature，缺表/容量不一致为 InvalidState，防御性运行时索引错误为 Bounds。未增加 catch 来源。

## 验证

新增 20 项集成回归和 5 项内部回归，覆盖 schema/索引边界、槽 32767、安装失败原子性、共享容器、直接失效对象、栈满/借用/受控分配失败、引用转发/写入/循环/形态检查、配额、暂停移除和重装、容器释放、跨对象 schema、生命周期拒绝与清理、零预算/分步及旧程序兼容。

测试分配注入仅在 cfg(test) 中按线程和作用域生效，不修改全局 Cell ID 计数器；不模拟标准库 OOM。生命周期内部测试构造私有任务以验证防御分支，公共准备入口仍拒绝这些指令。

复现命令见[阶段方法](stage2ab-evidence/method.md)，当前能力见[矩阵](stage2ab-evidence/matrix.md)。阶段 2AA 及更早整包验证器要求当时源码快照；当前独立验证其证据/统计，历史文件不更新哈希。
