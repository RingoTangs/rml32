# 阶段2AQ方法与复现

本批以2AP当前源码和冻结证据重审2AH全量目录：75行opcode、24项稳定缺口、13类能力门槛。2AI—2AP是增量依据，旧快照不改写；本批无新增反编译，无参考程序/脚本/业务字节码执行，无Rust源码、测试、公共API或依赖变更。

## 审计数据与人工边界

opcode-review.json记录当前实现、排除、源码摘录、相关测试及assertion_review；review-changes.json逐行对照2AH实现/排除/测试，并保持历史原文。gaps.json保留G01—G24，progress区分partial与unchanged；status仍表示当前剩余问题的rejected/difference/evidence_gap，不把partial当作整项关闭。capabilities.json保留C01—C13，所有阶段2门槛仍open。priorities.json比较三个方向，唯一首选P05。

所有旧源码摘录均重新定位并与当前磁盘内容核对；消失的Mapping拒绝分支由实际哈希与输出路径替换。人工增量断言复核见assertion-review.md和各opcode行；原有代表断言沿用2AH记录并检查测试仍存在、运行时路径与守卫未扩展。测试列表只是场景索引，不是完整分支覆盖。校验器验证结构、出处、引用及生成一致性，不能证明自然语言结论或测试断言充分。

全量静态矩阵直接复算冻结7,159程序的init/普通/hidden节点，与独立历史统计一致；另跑只读加载器比较器核对12包、9,527成员、7,159程序和9,933继承关系。唯一已知错误dba/pack_data/etc.pak/etc必须保留，新增错误失败。

## 基线、保护与收据

实施前运行2AP完整收据校验，将返回摘要、两个入口文档的原哈希及576个保护文件写入baseline-inputs.json。保护范围覆盖所有既有docs、src、tests、Cargo文件和原制品清单（入口文档仅允许新增当前进度）。本批只允许新增2AQ交付和更新README/路线图，不刷新历史哈希。当前入口改变后，不再要求2AP整包入口哈希匹配；原2AP静态验证和历史文件仍须通过，新收据负责当前交付。

verify_validation.py --write执行固定命令表，保存原始stdout/stderr/退出码到压缩日志并生成新收据；任一失败立即停止，不冻结成功状态。收据保存交付清单及哈希，验证时重查保护范围、原入口身份、命令结果、Rust测试名与数量、Python测试数、语料边界和生成一致性。无参数验证不重新运行Rust测试；日志证明的是记录时的运行。

```sh
python3 -B docs/stage2aq-evidence/verify_completion.py --write
python3 -B docs/stage2aq-evidence/verify_validation.py --write
python3 -B docs/stage2aq-evidence/verify_validation.py
```

固定检查包含2AP静态验证、2AQ生成一致性、Python工具负向测试、Rust debug/release、fmt、严格Clippy、只读检查器build及全量比较。缓存不可用时正常下载依赖需要按环境权限处理，不变更Cargo锁文件。工具负向测试只损坏临时副本。日志和收据不声称原生执行等价；阶段2未完成。
