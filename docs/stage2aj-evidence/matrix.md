# 阶段2AJ能力增量

基线为[2AH完整矩阵](../stage2ah-evidence/matrix.md)，除下表外沿用其范围。原先G01关于这七条指令的Bytes路径排除项，由下列限定支持替代；其他缺口保留。操作码集合未改变：75种限定支持、73种主语料交集。

| 指令 | 新增限定支持 | 仍排除 |
|---|---|---|
| 01 | 普通函数Bytes+Bytes/Int | Real追加、反向数值+Bytes、容器算术、生命周期新分支 |
| 1c/2a | Bytes完整内容相等/不等、与其他普通类型比较 | 原生intern头属性、Reference/Loop投影 |
| 1e/20/25/27 | Bytes首NUL前缀u8排序 | 混合类型排序、Buffer、生命周期新分支 |

证据复用[2AI语义报告](../vm-bytes-semantics.md)，实现、测试及限制见[运行时说明](../vm-bytes-runtime.md)。所有新错误均不增加数值catch来源。482项Rust测试通过，不代表分支覆盖率或原程序运行验证。G01与阶段2尚未完成。
