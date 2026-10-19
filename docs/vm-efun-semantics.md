# 阶段 2AD：06 宿主分派静态语义

本阶段完成 06 的静态取证、全量语料统计及[同步宿主实施规格](stage2ad-evidence/host-runtime-design.md)。Rust 运行时、公共 API、Cargo 依赖和 72 种限定操作码集合保持阶段 2AC 原样；06 仍被准备器拒绝。未运行参考 ELF、启动脚本或业务字节码，也未实现真实 efun 或服务启动。

## 证据与结论

目标 `magic_Linux32` 的 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。使用 reverse-engineer-anything 技能及 REA/Ghidra 12.1.4，新增 8 条函数 dossier 和 2 条名称搜索记录，复用 10 条历史记录。完整记录、查询边界、函数体范围和限制见[清单](stage2ad-evidence/function-manifest.json)、[可读视图](stage2ad-evidence/functions.md)和[结论账本](stage2ad-evidence/findings.json)。

以下为静态观察或明确标出的静态推导；不将反编译伪代码当作原始源码。地址均为 ELF 虚拟地址。

| 问题 | 依据与结论 |
|---|---|
| 操作数 | `vm_regularProcessor@0x080978c4`，`ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`；动作表将06绑定到双参数入口。`0x08097986` 将 par8 **符号扩展**，随后压入 signed par16；不采用07的低六位 argc 掩码。伪代码漏掉第二参数，应以汇编为准。 |
| 调用与身份 | `vm_callInEfun@0x08093d98`，`ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f`；先检查调用深度及全局函数表范围，再增加调用层，复制上一层40字节上下文，将bp置为当前SP、argNum置为传入参数、flags置1、f.pEfun置为目标，直接调用原生指针。thisObject、代码池等来自调用者，区别于全局07。 |
| 槽位 | 同上及 `vm_addEfuns@0x08085ce4`，`ev_a83bf890b4b1b7b1e670ffd84e2caf433106c4b1521c658ee8c18bbdb826b9a1`；注册依序分配函数号，但06检查的是全局函数容量。**推导：容量内不等于合法efun**，Rust须额外校验注册槽类型；559..571预置字节码槽不自动成为efun。 |
| 参数位置/类型 | `vm_stackInThisContext@0x080933e4`，`ev_d908ab85c3b336c3108d0113d535ec9b5582b136bb6b35d6ca5975a896519b6d`；`vm_getArgument@0x0808a364`，`ev_80271c109d8b170b4dac40d9836f89a9722c8d04023f8e58b881ce2adc69c17e`；参数n位于bp+n。检查参数存在与原始标签；mixed(255)接受任意原始标签，没有统一的引用解引用或数值转换。 |
| 参数责任 | `vm_checkArgNum@0x0809521c`，`ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126`是用户函数入口路径，不由06调用。`abs`自行检查最多/最少参数和数值类型，`time`检查自己的0/1参数分支；不能给全部efun套用用户函数的默认参数、引用或转换契约。 |
| 栈责任 | `vm_f_abs@0x080d453c`，`ev_c4611a370e790c377051346043a5c698f4e975dc9cadbcaf69c7b8520c207983`原地修改参数作为结果；`vm_f_time@0x080dd030`，`ev_2d1bf533fc49d724997930c78837496b2b5ad806bd6bcc2f6b7c0b0977d5a0ba`先pop argc，再push结果。`vm_popNStack@0x08091ab0`，`ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4`逐项cleanup。不是分派器统一复制/清理参数。 |
| 返回 | `vm_returnFromEfun@0x08093c5c`，`ev_e86eb89e60b57e211e2efdce9ee32fdd5b9d630f548d18ca4e6b50e43675595c`；结果数按 `bp+argNum-SP`计算。Void(1)要求0项并补Void；Mixed(255)允许0项补Void或1项；具体返回类型要求恰好1项且原始标签完全相等，不作数值转换。然后递减调用层并恢复上下文。 |
| 控制标志 | 06原生返回后，ctrlFlags=0直接走返回检查；含mask0x10则跳过立即返回；无0x10但含0x04时设置PCB mask0x40并跳过立即返回；其他非零组合仍走立即返回。返回路径还有tick检查和flags mask0x80的next-action hook。mask值与分支已确认，完整挂起/恢复协议未恢复。 |

## 参数顺序与返回模型

原生值栈向低地址增长，参数0在调用时SP，随后是参数1、2。现有Rust值栈方向相反，后续适配器必须取 `stack[len-1-n]` 为参数n，不能按底层Vec顺序传递参数。

原生C回调拿到的是当前上下文和栈，能够原地改参数、弹栈或压栈。后续Rust采用只读参数切片、受限上下文和 `Result<Option<Value>, HostError>`，由适配器统一提交单一结果。这是避免暴露内部槽的**设计选择**；不是原生同名API，也不表示已有559项C回调可以直接移植或获得引用兼容性。

原生 `vm_getArgument` 本身检查原始标签；某些efun可以另行调用其他转换或引用辅助函数。本轮只恢复分派契约和abs/time两个例子，不声称所有312个实际使用的efun具有相同参数规则。完整语义要逐项恢复。

## 错误、清理与原子性

`vm_error@0x08097b24` 的 `ev_1b494a92dfe3a6c5943ad427ec0ed25c0e15c7afc6932862b5221a85cb9ef7c7` 与 `vm_errorQuit@0x080ab104` 的 `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1` 展示 `-3` 和longjmp路径。伪代码在vm_error之后显示的访问不能当作正常成功分支。

`vm_traceError@0x08093520` 的 `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4` / `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` 显示：先移除年轻catch，pop到目标直接子调用的参数边界，再逐层退栈；flags&1的efun层跳过对象unExecRef。`vm_doActions@0x08097008` 的 `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` / `ev_a280841df7ce885805c45ba2b70f60cf4628279d7dbb748b1f816ba99a1ad2c4` 再恢复脚本catch保存栈与目标。具体依据延续[catch语义](vm-catch-semantics.md)。这支持efun上下文参与原生解栈的推导，不是所有线程类型和JIT模式的动态证明。

`vm_clearStack@0x0808aab0`，`ev_a7818c3b6ea7809db35bb023b25656cf263eb9244c344a8d13112a18e426c0fa` 为完整栈清理辅助记录，不把它误写成正常06返回必经路径。

后续Rust同步规格先拒绝引用和隐藏循环值，调用前预检资源并预分配结果Cell；错误归属于06所在代码及调用者对象。宿主失败不加入现有数值catch来源，保持可控失败，不跨Rust栈longjmp。预检失败不调用宿主、不修改操作数；已发生的宿主副作用不回滚。这个边界明确弱于原生通用catch能力。

## 语料与边界

对7,159个程序、101,592个普通函数及初始化段的6,474,773条指令重新扫描：

- 06共143,520处，普通函数142,702处、初始化818处；Global程序6,346处、Object程序137,174处。
- 固定注册表559槽，312槽被静态调用；所有调用槽都在0..558且名称可映射。
- par8实测0..30，均非负，0参数15,790处；不存在高位模式样例。负参数、高位和127的处理仍须边界测试，不能由当前语料推定原生定义良好。
- AAA/CCS/CSA/DBA/GS分别2,907/8,837/2,948/2,491/126,337处。逐efun总量、程序数和服务数与既有efun-usage.csv交叉核对。

见[统计方法](stage2ad-evidence/corpus-method.md)、[摘要](stage2ad-evidence/corpus-summary.json)、[559槽清单](stage2ad-evidence/efun-inventory.json)。完整位置为确定性压缩JSONL，偏移指解压后包成员内位置，不是ELF地址。静态出现不证明可达、参数类型正确或执行成功。

同步分派取证问题已闭合到所选受限接口设计；异步恢复、返回hook、JIT、可重入宿主、真实efun语义、原生引用参数、全局初始化和bootstrap继续排除。下一阶段只有实现并通过合成回归后才能将06加入限定支持集合；本轮仍为72种，不能用312槽的统计宣布efun实现。
