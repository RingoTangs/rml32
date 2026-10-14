# 阶段 2T：隐藏控制槽与受限整数、数组循环

## 一、结果与边界

实现23 `vm_initForLoop`、12 `vm_condJmpForLoop`、50 `vm_stepJmpForLoop`的整数±1闭区间及Array路径。支持集合增至65种限定操作码，278项Rust测试通过（新增24项）；主语料73种中交集63种，仍缺10种，另有4d/4e仅合成验证。编号统计不代表完整功能覆盖，阶段2尚未完成。

公开Value、启动/运行接口及Cargo依赖均不变。控制块是私有Cell载荷，不是公共脚本对象。本轮只运行合成程序及只读加载器，没有执行原ELF、启动脚本、原始初始化或业务字节码。

原生依据复用[阶段2R报告](vm-control-flow-semantics.md)与[Ghidra证据](vm-control-flow-evidence/rea-functions.md)：初始化0x080bc8f0，条件0x080bcdc8，步进0x080bcdb0，共享helper 0x080bcae0，释放0x080bc884。主要证据ID为`ev_6493fcdaf5956b81b156c8fbb9cf5987c216a53b856561a1e6351de2ad42af92`、`ev_2f7c7986819c8b0acae9e1cddb895fded61ac3d31a2c036489d87c95592ebcce`、`ev_7c730bb3ca6a220df55353b514f3cfe7e4d79e6347771e672ae459058017f177`。本轮没有新REA查询或运行对照。

## 二、隐藏槽与原子提交

Cell内部载荷现在是`SlotPayload::Value(Value)`或`Loop(Box<LoopControl>)`。控制块保存绑定Reference及`Numeric { next,end,step,finished }`或`Array { owner,index }`。读取/复制隐藏槽投影为Undefined，控制块仍在原槽；帧快照只显示投影，Frame自身没有大型游标或集合快照。

普通replace、批量replace、所有现有引用/局部写入和槽销毁统一处理载荷。覆盖控制槽立即释放旧控制块及header；普通返回、执行重启、消耗VM亦随槽销毁释放。耗尽、break或预算暂停不自行清空控制槽；普通终止错误按既有契约保留帧/栈，资源到覆盖/重启/销毁时释放。finishAction继续沿用既有保留状态契约，不宣称原生后端清理兼容。

`replace_payload_batch`先取得全部不同Cell的可变借用，再移动载荷，并在释放所有guard后销毁旧值。重复Cell或借用冲突拒绝整次提交。循环检查先计算下一控制状态、输出、目标PC和权限，再原子提交控制槽与绑定变量；复制死对象元素时，一并将源数组成员规范化为Undefined。未发生可见的半次推进，RefCell guard不会跨调用或暂停。

LoopControl只做内部短期克隆，数组header通过Rc共享，不复制元素；推进不建立集合快照；引用验证和旧值清理沿用既有帧/槽检查与释放路径。控制块数量受局部槽、max_stack及max_frames限制，不另增公开配额。header引用计数观察方法仅在cfg(test)下编译，非公开调试API。

## 三、三条指令契约

| 指令 | 操作数 | 行为 |
|---|---|---|
| 23 | par16控制槽；par8忽略 | 消费栈顶mode、end/Array、绑定Reference，安装新控制块 |
| 12 | par16退出位移、有符号par8控制槽 | 检查并写本次值；成功顺序进入body，失败跳转 |
| 50 | par16继续位移、有符号par8控制槽 | 检查并写下次值；成功回跳body，失败顺序退出 |

准备阶段要求控制槽为已声明负编号局部槽，12/50的目标为`pc+1+delta`且位于当前体内。运行时还要求局部槽已分配；不推测init/cond/step必须成某一种语法配对。缺少或已覆盖的控制值返回InvalidState。普通辅助函数中的循环可被07/08调用，但初始化/构造准备路径保守拒绝循环段及其所有已链接辅助函数。默认表达式白名单继续不开放循环。

绑定只允许有效、可写的本地/参数/对象Reference，保留原owner；拒绝Owned临时/成员引用、与控制槽同Cell、绑定其他隐藏控制槽。隐藏控制槽可以通过普通引用被覆盖，但不能作为函数引用参数传递。每次检查重新验证owner和readonly，即使数组已耗尽；这些防护是受限Rust边界，不等同原生裸指针错误现场。

### 整数模式

mode仅+1/-1，end和初值必须为Int。每次检查先把旧next写入变量，next按i32环绕加step，再按有符号闭区间和finished判断；旧next==end时置finished，防止端点环绕后重新进入。失败检查也写变量：1..3在body依次看到1、2、3，退出检查后变量为4。起点越界时不执行body，但变量写入起点。body修改变量不改变内部next。

### Array模式

输入mode为997（0x3e5）。控制块保留原Array header，从index0开始逐次复制当前成员，游标用u32环绕；不是提前elements快照，也不是成员引用绑定。空数组不写变量，单元素执行一次，退出检查保留最后输出值。body通过别名修改未访问成员，后续读取可见；源变量重绑不更换所持header。死Object成员复制为Undefined并原子清理源成员，沿用原生copyValue的规范化事实。

当前公开Value没有原生source.attrib位，首批只声明现有Array模型的限定路径，不模拟额外源属性或const传播。

## 四、错误与测试证据

可预检错误在消费操作数或写变量前返回，IP仍按既有派发规则提前递增；这比原生先清旧槽再分配/报错的现场更保守。Map模式返回UnsupportedFeature；无效整数/源类型返回明确错误。普通错误保持粘性诊断，不做事务回滚或通用catch。

新增21项VM集成测试覆盖：

- ±1空/闭区间、相等端点和i32环绕、失败检查后变量值、body修改变量。
- 空/单/多元素Array、实时成员修改、源变量重绑、死Object源规范化。
- 嵌套、递归帧限额、继承变量基址、07/08和default组合、单步及零预算。
- 控制值读取、覆盖失效、提前break/return、重启、失效owner、坏模式/槽/目标、Mapping/成员绑定拒绝、生命周期辅助体拒绝及栈限额。

新增3项单元测试验证header在读取时保留、replace/批量替换/clear/drop时释放，借用冲突及重复Cell原子拒绝，以及readonly/失效绑定不推进控制状态。既有254项回归全部保留；合计10单元、23加载器/CLI、245 VM测试，共278项。

当前[逐指令矩阵](stage2t-evidence/matrix.md)与[机器验证记录](stage2t-validation.json)独立保存。全量只读检查仍为12包、9,527成员、7,159程序，唯一已知异常为DBA etc成员LZSS长度不符；结构检查通过不是业务执行覆盖证明。

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
python3 tests/compare_corpus.py --report target/stage2t-corpus-validation.json
python3 docs/stage2t-evidence/test_verify_completion.py
python3 docs/stage2t-evidence/verify_completion.py
python3 docs/stage2t-evidence/verify_validation.py
```

## 五、剩余缺口

Mapping桶链顺序及mutation语义、buffer、额外source属性、成员/临时绑定、控制槽参数位置、通用catch及任务内循环尚未支持。控制块不泄露为公共Value，但这不等于完整原生生命周期、引用计数或非局部退出兼容。

参考制品、Cargo依赖、公共Value/API和2P—2S历史证据不变；旧来源快照不随当前代码更新，使用2T记录验证本轮交付。下一步应继续按剩余能力取证与分批实现，不能只按操作码数量宣告完成。
