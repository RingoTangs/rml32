# 2AR方法、来源与复现

本批以2AQ为基线，调查G06/G07/G12的复制源规范化及非标量捕获所有权；Rust/API/依赖与白名单保持不变。实施前2AQ收据通过，冻结601个既有文件及README/路线图原身份。历史文档、原ELF/PAK/对象/脚本均保持不变。本批未执行参考程序、业务字节码或启动脚本。

## 取证过程

按reverse-engineer-anything技能，主会话检查实际REA状态与已保留记录，使用匹配的magic_Linux32（SHA-256见清单）。初始无打开目标、无保留记录；Ghidra12.1.4可用，显式打开目标。两项独立只读审查并行复核冻结捕获证据和复制/释放证据；REA会话由主流程串行使用，没有重复导入或重复分析。

复用50条完整历史记录，覆盖创建、展开、copy/cleanup、引用/成员/变量/返回、错误恢复及GC边界。记录可能是完整dossier或旧primitive；清单明确区分，不把伪代码/汇编单项称为完整dossier。实际函数名取自ELF/证据：vm_callFunPtr、vm_createFunctionValue/Ex。旧导航名vm_callInVarFun/vm_newFunction不作为符号事实；vm_f_bind_parameters/_arr是mysql参数包装，单列排除出处。

仅新增vm_unrefValue、vm_unrefMapping与vm_error三个完整dossier，解决最后slot/header释放条件和错误分流。vm_error响应超过MCP传输预算，分析本身已完成；按工具给出的保留Evidence引用导出完整包，不重复producer，不从截断预览重建JSON。原始完整导出保留于target，规范JSON经无损压缩进入rea.json.gz；session.json保留实际传输通知和会话前后状态。导出后关闭本批打开的REA目标，未保留活动Ghidra会话。

## 验证的能力边界

function-manifest.json校验每条记录、原始制品身份、ELF符号完整范围哈希、Ghidra实际inclusive body各段和来源。非连续body不填平；终端调用导致的Ghidra范围空洞和provider限制完整保留。85条指令锚点关联精确汇编、文件偏移及机器字节，结论引用锚点与Evidence ID。

findings.json区分observation、static-inference、design、unknown；questions.json逐问题标记answered/partial/unresolved。expected-cases.json的51个案例是证据推导或设计/未知边界，均runtime_executed=false，不执行模型或参考程序。工具检查出处、结构和一致性，不证明自然语言推导或原生全图/GC等价。

语料导航复用2AF的35,703处05/17和2AN的5,012处Global创建，不做新的tag可达性推断。全量PAK比较明确复用2AQ冻结结果（12包、9,527成员、7,159程序、9,933继承关系及唯一已知DBA解压异常）；本批不冒称重跑。

## 复现与收据

```sh
python3 -B docs/stage2ar-evidence/verify_evidence.py
python3 -B docs/stage2ar-evidence/render.py --write
python3 -B docs/stage2ar-evidence/verify_validation.py --write
python3 -B docs/stage2ar-evidence/verify_validation.py
```

--write只生成本批交付或执行/冻结本批验证日志，不刷新历史哈希。证据校验无需REA在线；再次取证不是验收前置。实际验证包括证据/生成一致性、Python工具负向测试、Rust558项debug/release、fmt和严格Clippy。新收据保护所有旧材料并验证当前入口；新增docs和入口变化后不再直接调用依赖旧全仓清单的2AQ整包验证，改为保护其原记录与静态交付。

工具负向测试只修改内存/临时文件，覆盖记录缺失、错误身份、body范围、锚点、来源、悬空引用、状态夸大、静态案例冒称执行和历史哈希篡改。Rust测试数与名称须和2AQ一致。原生新记录数、复用数、函数数及案例/锚点数量均从交付复算；没有以编号交集或测试数量关闭阶段2。
