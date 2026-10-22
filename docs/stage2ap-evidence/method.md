# 2AP 方法与复现

本阶段以2AO实现和2AN冻结证据为输入。`baseline-inputs.json`保护参考二进制、历史
证据及未列入本批范围的文件；本批运行时、合成测试、说明和进度文件列入显式变更
清单。2AP静态验证直接复用2AN记录、来源、机器码锚点、结论和语料复算器，不调用会
校验旧Rust源码快照的完整历史验证器，也不修改2AN或2AO的历史哈希。

所有运行时覆盖均通过合成程序构造。回调测试使用本地计数器验证回调次数、替换注册
后的签名、错误提交和外部副作用；不执行 `magic_Linux32`，不调用真实efun。

复现命令：

```sh
python3 -B docs/stage2ap-evidence/verify_validation.py --write
python3 -B docs/stage2ap-evidence/verify_validation.py
```

`--write`执行本阶段检查并冻结日志与收据；不带参数只验证收据、交付文件哈希及历史
静态证据。Rust测试数、原有测试名保留数、Python检查数、opcode集合和保护文件数均
由收据验证器核对。
