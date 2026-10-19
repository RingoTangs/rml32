# 阶段 2AE：受限同步06宿主调用

实现阶段2AD冻结的[同步接口规格](stage2ad-evidence/host-runtime-design.md)，原生观察沿用[06静态语义](vm-efun-semantics.md)。本轮只执行合成测试程序和只读解析工具，没有新增原生会话，没有运行参考ELF、启动脚本或业务字节码。

## 公共接口与准备

新增host模块及库根重导出：HostType、HostReturn、HostSignature、HostCallContext、HostError、HostCallback、HostFunction；签名与2AD规格一致。ObjectStore新增register_host_function/remove_host_function，注册属于store，into_objects保留。槽范围0..558、名称、原型和重复操作按规格检查；注册不依赖Global代码绑定或变量表。HostFunction的Debug只显示名称和原型，回调为Rc<dyn Fn>。

新增prepare_program_with_host_calls(image, Option<GlobalBankSchema>)和prepare_global_program_with_host_calls(image, GlobalBankSchema)。前者仅接受无继承单constitution Object普通函数，后者延续Global格式/schema/绝对函数区间约束。Some(schema)同时保留2AC的全局变量/全局07能力。旧准备器继续拒绝06；两个新入口忽略init，默认表达式白名单、链接和生命周期准备器不开放06。

06准备时par16须在0..558，signed par8须非负；运行时防御性检查argc0..127。不存在07的位掩码，64/127分别表示64/127个参数。只读二进制解析器原本就会拒绝越界efun槽；准备器的独立边界测试修改已解析的合成映像来验证第二层防线。

不新增VmErrorKind、VmLimits字段、Value/ReferenceKind变体、依赖或unsafe，现有Vm构造/start/run/into_objects签名保持不变。这是新增公共API，并非“没有API变化”。

## 参数、结果和上下文

参数n读取栈顶向下第n项，先检查need(argc)、临时宿主层的帧配额、数量、全部原始形态、精确类型/容器约束及结果栈配额。引用和隐藏Loop payload均拒绝，即使签名为Any；不得先通过Cell.read将Loop投影为Undefined。required/optional/rest只匹配实际参数，不补默认值、不做int/real转换。

浅克隆现有Value；Bytes/Array/Mapping保持共享身份，参数与结果均不增加虚构容器层。Object包括失效句柄原样传递，不查存活也不规范化。根值存活检查仍由后续实际使用这些句柄的指令负责；此接口不提供对象存储访问。

回调前预分配Void结果Cell，try_reserve参数向量与结果栈空间。回调获得不可变HostCallContext和参数切片，不能取得Vm、ObjectStore、Cell或Reference。Context包含当前实际执行代码的program/source/function/PC/offset及可选对象；Object为Some，Global为None，不伪造字节码宿主帧。临时HostActivation参与max_frames预检，但不进入持久frames/catch深度。

Void仅接受Ok(None)；Exact接受精确匹配的一项；Mixed接受None补Void或任意一项现有Value。结果再次验证后写入预分配Cell，释放临时参数，再截断原参数、压入结果；返回原容器参数不会因释放参数而丢失内容。

## 失败、PC与预算

解释器原来统一在分派前递增PC。06独立置于该递增之前，成功提交后只递增一次，失败保留原PC；其他操作码保持原时序。诊断仍使用run开始本条指令前保存的实际帧，宿主结果错误/回调错误消息包含槽号与名称。

预检失败不会调用回调或改写操作数/根槽；回调后失败也保留操作数和PC，但宿主捕获状态及其他副作用不回滚。所有06错误的RuntimeFaultSite为None，回调报告Arithmetic也不能被现有数值catch捕获。失败进入sticky failure，后续run不重放；start重置/VM销毁释放保留栈。临时Value/结果/回调克隆正常释放，借用guard不跨越回调或容器析构。

06只消耗一个指令预算；零预算无注册查询、分配或回调，回调内部没有挂起点。暂停发生在指令边界，之后可以通过objects_mut移除/重装注册；后续06使用新绑定，已经完成的调用不重放。指令预算不约束宿主耗时；panic不转为脚本错误，不引入catch_unwind或longjmp。

## 验证与剩余范围

新增20项集成测试、6项内部测试，合计433项Rust测试（41 unit、23 parser/CLI、369 VM）。覆盖槽边界/签名/准备隔离、参数顺序和可选/rest类型、argc0/64/127、全部现有值类型、Void/Mixed/Exact结果、容器身份、无全局绑定调用、双上下文恢复与错误现场、不可捕获宿主错误、暂停重注册、配额/借用/分配失败、隐藏Loop拒绝、失效Object不规范化和临时所有权清理。内部借用测试仅增加cfg(test)私有写借用辅助，不改变生产Cell接口。

支持集合现为73种限定编号，主语料交集71种，仍缺05/17；编号数量不是功能完成率。真实559项efun、原生引用参数、异步挂起、宿主重入、hook/JIT、Global init、bootstrap及真实服务启动继续排除。合成回调占用某槽不等于实现了该槽的真实efun。

当前[能力矩阵](stage2ae-evidence/matrix.md)、[复现方法](stage2ae-evidence/method.md)和[验证记录](stage2ae-validation.json)独立保存2AE状态；2AD及更早历史文件保持原样。
