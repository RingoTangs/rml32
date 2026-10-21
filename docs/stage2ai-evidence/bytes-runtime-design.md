# 下一批：普通函数 Bytes 比较与受限拼接（尚未实现）

本规格由2AI静态取证推导；本批未修改Rust。原生事实、设计和未闭合项见[语义报告](../vm-bytes-semantics.md)及findings.json。首批选择已闭合且不依赖外部浮点格式化的路径；不据此宣称G01整体完成。

## 公共接口、准备与生效范围

沿用公开Value::Bytes(Rc<[u8]>)、tag4及现有准备器，不新增公开类型、函数、依赖、unsafe、VmErrorKind、VmLimits或ReferenceKind。实现私有bytes模块及VM普通二元指令接点；不把Bytes分支加入numeric::binary，避免顺带开放2f复合赋值。已有Rust Value的Eq保持不变，VM运算由新helper决定。

七条指令编号已受限支持，支持集合保持75种/主语料交集73种。现有普通Object、Global及继承函数不需要新opt-in入口；现有默认表达式白名单保持不变。只有实际进入新增Bytes分支时检查initialization/construction任务并返回UnsupportedFeature，包括任务通过普通helper间接到达的情况；不能按函数名称推测生命周期。既有纯数值及含Function身份比较分支不受此新增guard影响。Global init仍不准备，不增加真实服务执行入口。

## 接受的操作数与内容契约

Rust向上Vec计算区left=n-2、right=n-1，先need(2)保护参数/局部区；原始Reference/隐藏Loop不能作为新增Bytes运算值，报UnsupportedFeature，不解引用、不把Loop投影为Undefined比较。

| 指令 | 首批接受输入 | 结果 |
|---|---|---|
| 01 | Bytes+Bytes | 按两侧完整payload长度拼接，包含嵌入NUL/高位字节，结果Bytes |
| 01 | Bytes+Int | 按有符号i32十进制ASCII追加，无引号、无前导加号、无区域化；覆盖0、MIN、MAX |
| 1c/2a | 至少一侧Bytes，另一侧为普通Value | 两侧均Bytes时按长度及全部字节相等/不等；Bytes对其他普通类型为不等；结果Int0/1 |
| 1e/20/25/27 | Bytes/Bytes | 各取第一个NUL之前的前缀（无NUL则完整payload），按u8字典序及前缀长度比较，结果Int0/1 |

含Function的1c/2a优先保留2AG既有身份分派，包括Function对Bytes不等；不新增owner规范化或生命周期限制。纯数值输入全部返回现有数值路径，保留精确Int比较、binary32混合比较及既有错误。新增helper不复制/规范化Object、Function或源槽，仅检查原始普通值的tag。

Bytes+Real虽然原生支持，但首批明确UnsupportedType：原生将binary32提升double后用sprintf("%g")，Rust默认Display并非同一契约，具体原生libc/locale及特殊值尚未验证。数值+Bytes、Bytes+Undefined/Void/Object/Array/Mapping/Function及排序混合类型均UnsupportedType。Buffer、Bytes索引/写、范围引用、容器运算、2f字符串复合赋值不开放。

相等必须区别于排序，例如a\0x与a\0y不相等，但排序前缀相同，因此>=和<=均真、>和<均假。不得改成全长词典排序，也不得用C前缀实现相等。

## 共享、长度、资源与提交

payload不额外保存原生终止NUL；C前缀使用安全切片，不调用宿主strcmp/locale。Bytes相等按内容，不照搬原生“不同shared头直接不等”的优化：Rust没有该intern属性。原生共享属性/指针可见性、任意伪造原生头不在所选兼容范围。

拼接结果最多65,536字节，沿用既有只读范围结果的量级，这是Rust资源策略，不是原生字符串限制；比较不增加长度上限。先checked_add完整两侧长度（Int为ASCII长度），再检查结果上限；即使右Bytes为空也先检查结果上限。超限或try_reserve失败为Limit。Bytes+空Bytes在合法范围内克隆左Rc即可，保持原生零扩展行为；其他拼接创建独立payload，不修改任何已有Rc内容。用11字节栈缓冲或有界整数格式化辅助函数产生Int ASCII，禁止调用native formatter或引入新依赖。

新增Bytes处理放在统一PC递增之前。仅当七条指令中出现Bytes操作数且不是已有Function相等分派时进入；不消费或改写原始栈来探测类型。先检查能力/任务、计算区数量及原始形态，再进行类型、长度及所有可恢复资源预检。结果及提交所需状态全部就绪后，先完成左Cell的单次replace，再pop右项并推进PC；replace借用失败必须在pop前返回。不新增栈/帧/引用租约。成功消耗2项产生1项；失败保留原PC、两操作数、前缀、所有源对象/容器和引用额度。

try_reserve和可控Cell失败可返回Limit；最终Rc分配的标准库OOM不承诺恢复，原生alloc/realloc布局、32位溢出和危险失败副作用不模拟。若选择预分配结果Cell，须在任何栈变更前完成，且不得使现有左槽/局部槽身份发生未说明变化。优先复用已有左Cell以避免额外Cell ID；内部借用冲突回归需覆盖该提交边界。

诊断仍由run关联原指令的source/program/object/function/PC/成员offset；不使用相邻常量或ELF地址替代成员offset。每条操作一单位预算，零预算无检查/分配，成功后暂停可看到结果；预检错误sticky，不回放。此批不增加RuntimeFaultSite；Bytes类型、长度、分配、引用和任务错误不能被数值catch吞掉。既有后续数值故障仍可按原契约捕获并释放年轻帧/结果。

## 必须覆盖的未来回归

- 空/单/多字节，左右顺序、重复同一Rc、不同Rc相同内容、大小写、高位字节、嵌入NUL与两种比较规则；以expected-cases.json的静态推导表为基础，补齐反向混合tag输入。
- i32的0、负数、MIN/MAX；65,535/65,536/65,537结果，空右值但超长左值，比较超长输入及checked_add边界；Real拼接明确拒绝，不能用近似格式通过。
- 类型/原始Reference/Loop、计算区下溢、任务直接及helper间接调用、借用/预分配失败的原子性与诊断；原来的数值/Function语义不变。
- 本地/Object/Global槽、Array/Mapping元素及捕获的Bytes保持不变；拼接结果独立、空右值允许身份共享；不引入容器重接或引用逃逸。
- Object/Global/继承调用者、跨07/08/05调用与返回、零预算/单步/连续运行相同；新增错误绕过数值catch，随后数值catch释放新结果不漏配额。
- 默认表达式、init/构造/回滚、2f及Buffer等仍按原范围拒绝；无需为新增Bytes成功场景改变旧准备入口或支持集合。

实现及上述测试通过后才更新矩阵中的Bytes受限分支；473项为2AI保留的Rust基线，不提前填写下一轮测试总数。G01中的其他非数值算术、属性、Real格式及原生失败等价仍需独立工作。
