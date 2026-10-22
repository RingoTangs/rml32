# 2AO 方法与复现

按2AN冻结规格实现第一批Global/c，不增加原生取证会话。原始ELF／PAK／对象及脚本保持不变；执行测试全部为合成程序，不运行原程序、init或包内业务字节码。

baseline-inputs记录开工时562个既有文件摘要。本批只允许五个既有源码／测试文件及README／路线图两个进度入口改变；555个其他文件逐一校验。Cargo依赖、锁文件、加载器、容器和全部历史交付保持不变。

2AN的verify_evidence／verify_validation固定原源码哈希，迁移后不应通过全包旧快照校验，也不应重写旧哈希。2AO使用新的verify_static_evidence：在检查本批明确允许的变更范围后，调用2AN纯记录／来源／符号／机器码锚点／结论校验，独立复算2AN语料并逐字比较输出。20条原生记录与5,012个Global创建位置均复核，静态统计不当作执行oracle。

run_historical_tests执行22项2AN可复用工具测试，只排除锁定旧源码的test_delivery_and_protected_inputs；检查原23项测试清单，不能静默少跑其他项。新增5项Python检查验证当前静态交付、变更白名单、原制品保护、文件丢失、路径逃逸及opcode集合边界。

Rust保留原515项测试名称并迁移Option接口断言，新增29项。新内部测试覆盖Cell分配／借用冲突、资源和帧ID、Reference／Loop／Function捕获拒绝、任务守卫、owner=None复制与最后头释放，以及引用lease；集成测试覆盖创建、调用、绑定、上下文、参数和错误现场。544项在debug／release通过，不以测试数量宣称完整兼容。

```sh
python3 -B docs/stage2ao-evidence/verify_static_evidence.py
python3 -B docs/stage2ao-evidence/run_historical_tests.py
python3 -B -m unittest discover -s docs/stage2ao-evidence -p 'test_*.py'
cargo test --locked --offline
cargo test --release --locked --offline
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
python3 -B docs/stage2ao-evidence/verify_validation.py
```

默认验证命令只读；verify_validation显式`--write`才重跑上述检查并冻结本轮完整stdout／stderr、退出码、交付摘要和receipt。默认receipt验证会重新检查静态证据与受保护文件，保存的测试日志只证明当时的测试结果，不代替一次新的Rust测试运行。

新增公开接口和源代码不兼容的Option迁移见../vm-global-function-value-runtime.md。Global绑定变化限制是Rust策略，非原生热更新保证；全量copy源cleanup、owner延迟删除、Host/K及非标量捕获仍有缺口。阶段2未完成。
