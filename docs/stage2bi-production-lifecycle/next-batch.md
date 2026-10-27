# 后续任务

建议下一批对生产 opt-in 做独立契约审计与差分扩展：将本批真实 Rust 配额、queue 和维护进度投影到冻结 D08 规格，覆盖每一类成功/拒绝转换及身份、游标和部分进度，而不仅回归先前无配额模式。保持本批公共 API 和现有模型冻结，用独立新证据解释发现的差异。

优先扩展多层构造/rollback、Global/Host Function 与 Mapping 修改/loop/catch 的生产模式组合，检查极小配额下的失败路径及 arena teardown；有可用环境时增加实际 32 位 Rust target 检查。当前 32/64 位算术规格不是 32 位二进制执行。

上述扩展不自动开放直接 Object/Function 捕获、escaping References、一般 GC、真实 efun、业务析构、异步/重入、线程或 JIT。未发布 H/A 及原生 scheduler/shutdown 进展仍需独立原生证据，二十项未知与阶段2完成门槛继续保持。
