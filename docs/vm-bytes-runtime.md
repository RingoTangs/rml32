# 阶段2AJ：普通函数 Bytes 运算

按[2AI实施规格](stage2ai-evidence/bytes-runtime-design.md)实现私有 `src/bytes.rs` 和 VM 二元指令接点。原生依据复用[2AI静态语义及Evidence ID](vm-bytes-semantics.md)，本批未新增反编译或执行原程序。只执行内存中合成程序，不执行包内业务字节码。

| 指令 | 本批新增行为 |
|---|---|
| 01 | Bytes+Bytes按完整长度拼接，包括NUL后的字节；Bytes+Int追加有符号i32十进制ASCII |
| 1c/2a | Bytes按全部字节及长度相等/不等；Bytes与其他普通类型不等 |
| 1e/20/25/27 | Bytes/Bytes按首NUL之前的u8前缀排序 |

例如 `a\0x` 与 `a\0y` 不相等，但排序相同。含Function的相等判断保留既有身份分派，纯数值仍进入原有numeric模块。未改Rust Value的结构相等、公共API、依赖或操作码集合；仍为75种限定支持、73种主语料交集。编号覆盖不是功能完成率。

拼接采用checked长度及65,536字节结果上限；比较无此上限。合法空右Bytes复用左Rc，其他拼接分配独立不可变payload，源数据不变。Int格式化使用11字节栈缓冲，覆盖MIN/MAX。上限是Rust资源策略，不是原生限制；try_reserve失败为Limit，最终Rc分配的进程OOM不承诺恢复。

新增路径在统一PC递增前检查计算区、原始Reference/隐藏Loop、类型和任务状态。计算结果后先replace左Cell，成功才pop右项、推进PC，不新增Cell或引用租约。可恢复失败保留原PC、操作数及前缀；run仍关联原程序、函数、PC和成员offset。新增错误不产生RuntimeFaultSite，不会被数值catch吞掉。零预算不检查/分配；每条指令消耗一单位预算。

init/construction任务存在时，即使当前帧是普通helper，新增Bytes分支仍返回UnsupportedFeature；既有数值与Function比较分支不增加此限制。Real拼接、数值+Bytes、排序混合类型、2f字符串复合赋值、默认表达式扩展、Buffer及Bytes写入均未开放。Global init和真实服务启动仍未实现，G01与阶段2均未完成。

本批新增9项Rust测试，总数482（56库单元、23解析集成、403 VM集成），debug/release均通过。一个VM集成测试逐一执行2AI的56条静态/设计期望，包括Function及反向混合类型；它们不是参考程序动态输出。另验证边界65,535/65,536/65,537、超长比较、空追加共享、源数据不变、原始引用/Loop、计算区下溢、提交借用失败、无Cell分配、生命周期helper限制、错误sticky、预算及Object/Global/07/08/05调用、数值catch边界。既有两个负例调整为仍不支持的类型组合，继续检查删除对象及Global错误诊断。

未独立模拟系统allocator耗尽，未新增继承/捕获/成员槽组合的穷举测试，也未证明原生OOM、intern属性、libc浮点格式及GC等价。后续按[2AH缺口审计](stage2-completion-assessment-2ah.md)继续处理非数值运算、Mapping遍历及函数值/生命周期组合；保留原始历史证据与矩阵，不把本批称为完整兼容。

复现命令、计数和语料核对见[验证记录](stage2aj-validation.json)，[本批矩阵](stage2aj-evidence/matrix.md)只列增量，其他能力沿用2AH基线。
