# 阶段 2W 验证方法

本阶段扩展既有操作码的数值错误捕获来源；限定操作码集合仍为 68 种。只复用三份历史 REA/Ghidra 证据包中的八条记录，不发起新原生查询，不执行参考 ELF、启动脚本或原始业务字节码。

`reused.json.gz` 保留八条完整原始记录及三个来源包的 SHA-256。`function-manifest.json` 索引函数、入口、操作、来源与限制。`fault-sites.json` 将五种私有来源细分为七条操作数路径，记录零判断、原生错误 CALL、复合赋值委派及写回顺序。原始目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`，提供者为 Ghidra 12.1.4。

运行：

```sh
python3 docs/stage2w-evidence/verify_evidence.py
python3 docs/stage2w-evidence/test_verify_evidence.py
```

`verify_bundle(root=仓库路径, here=本证据目录)` 返回记录、来源包、路径、私有来源数量；不通过则抛出 `ValueError`。它核对来源文件哈希、逐项原始记录、目标与提供者、清单、五种来源及操作数范围，并校验原生分支、错误 CALL 和委派 CALL。合成损坏测试只操作临时副本。该检查验证留存证据一致性，不替代人工控制流分析，也不执行或重新导入目标。

整数/实数复合赋值的失败保证仅限已确认的数值目标：原生临时栈建立早于算术，目标清理和写回晚于算术。Rust 保留其既有引用、readonly、所有权和边界检查，检查失败不标记数值来源；原生 readonly 优先级未确认，不能声称相同。NaN 除数不触发零检查；实数余数零经 `fmod`；整数 `MIN/-1` 的 `IDIV` 无显式 `vm_error`，不扩大捕获支持。生命周期、宿主回调和 JIT 不在范围内。

本阶段交付验证应检查当前代码与阶段 2W 记录，不运行要求当前源码仍匹配历史快照的旧阶段校验器。历史证据与清单保持冻结。

## 当前能力矩阵与运行验收

矩阵沿用阶段2V的18项审计规则，重新统计历史主语料并核对当前支持表、逐行限定分支、真实Rust测试名称与输入哈希。仍为73种主语料、68种限定支持、交集66种、并集75种、缺7种，额外4d/4e；不声称功能百分比或已测量分支覆盖。verify_completion.py默认只读比对，--write仅更新本目录matrix/summary生成文件。

阶段2W交付清单另外核对所有受保护原始制品、Cargo依赖、其他源码与历史快照，记录当前修改及测试重命名。Rust执行仅针对合成程序；全量只读语料比较器只运行新的Rust检查工具，不运行原版或业务字节码。已知DBA LZSS异常必须保留。运行时说明和测试结果见../vm-numeric-catch-runtime.md与../stage2w-validation.json。
