# 阶段2AR：复制源规范化与捕获所有权静态语义

2AR完成所选接点的定向调查，**非标量捕获仍未开放，阶段2仍未完成**。交付53条原生记录（新增3、复用50），涉及40个函数、85条机器码锚点、23项结论和51个静态/设计案例。六项问题中Q02展开和Q03复制接点已回答限定范围，其余四项仍部分回答。Rust/API/依赖和558项测试保持不变；所有原生结论均来自静态分析，没有执行原版或业务字节码。

## 已确认的核心区别

| 接点 | 静态结论 | 证据账本 |
|---|---|---|
| 17建立捕获 | 先扫描Reference并分配头/捕获存储，再raw move type/attrib/payload并清空源；此处不调用copy | F01—F03 |
| 05展开唯一head | 原生ref==1时倒序raw move捕获，清空旧槽后释放函数头 | F04/F05/F11 |
| 05展开共享head | 先析构临时调用目标，再经pushValue/copyValue展开；存活函数头中的捕获源可被规范化 | F06/F08/F09 |
| Array/Mapping COPY | 浅共享header，不递归规范化成员；具体成员读取才触发该成员copy | F07/F13 |
| 死Object/Function COPY | 清理实际源槽，随后目的与源均成为Undefined；无creator的Function不因None判死 | F08/F12/F22 |
| 最后header | Array只unref元素槽，外部成员Ref可保留payload；Mapping节点即使延迟回收也先清key/value | F14/F15 |
| 选定普通返回 | 非const、无转换/hook的mixed/已匹配返回原始移动结果，不统一追加copy规范化 | F16 |

逐项结论、原生函数与完整Evidence ID见[结论账本](stage2ar-evidence/findings.md)，每项链接[指令锚点](stage2ar-evidence/anchors.md)；函数与范围见[清单](stage2ar-evidence/functions.md)。例如vm_copyValue的删除分支直接调用cleanup(source)，因此共享函数值首次调用可以永久改写被捕获源；唯一move分支在展开接点不做同样规范化。不能把两者统一为Rc::clone或全部提前copy。

## 失败清理与未知

05先消费原调用目标，再检查/展开捕获；参数数量/类型检查发生在callee入口0f，逐参数转换可能早于后参错误。普通解释器线程type1且有效script catch下，traceError退出帧、doActions恢复保存sp，已有静态栈清理链。它不会恢复函数头、原操作数、先前转换、已提交外部写或共享捕获源规范化（F18/F19）。这与当前Rust成功发布前预检政策不同，应明确登记而非冒称失败现场等价。

无script catch时的最终PCB/任务销毁、其他线程、JIT/callback恢复，以及17捕获移动后结果栈溢出时C局部未发布函数头的最终清理仍未闭合。创建分配失败发生在移动前的局部顺序已确认，不能因此把整个创建过程说成原生事务（F02/F20）。

GC marker确实跟进Array/Mapping/Function捕获及部分Loop状态，但没有据此证明一般Reference图、外部mark根和循环释放顺序。native ref、execRef、creator引用与捕获Object引用也不能直接等同Rust逻辑remove或宿主Rc计数（F17/F23）。

## 后续实施门槛

[后续Rust规格](stage2ar-evidence/runtime-design.md)明确：下一批2AS先闭合源可改写复制计划、VM语义持有者/稳定捕获槽与Function捕获边图验证设计，再考虑受限无环Array/Mapping。直接Object/Function捕获继续单列；当前不将任何非标量扩展标为可直接实施。

一个新增风险是Function→容器→Function环：当前成员写允许普通Function，而容器深度验证此前可以把Function视为叶子。放开容器捕获后，仅禁止Array/Mapping成员重接已不足够；所有构造/写入/Host结果及循环输出接点都需纳入图验证设计。该项是Rust安全设计要求，不是对原生任意图回收能力的证明。

普通Function可存入Array/Mapping与“非标量值不能捕获进Function”仍是两件事。本批不新增公共类型或修改准备入口，不更改旧2AQ及更早快照，不因原生17仅显式拒绝tag8就放宽全部Value。

## 验证与复现

见[方法与来源](stage2ar-evidence/method.md)、[问题状态](stage2ar-evidence/questions.md)、[期望案例](stage2ar-evidence/expected-cases.md)和[验证收据](stage2ar-validation.json)。收据保存实际命令日志和当前交付哈希；601个旧材料与原制品保持身份不变。全量PAK比较与静态位置统计复用冻结结果并注明出处，不当作本批原生运行覆盖。
