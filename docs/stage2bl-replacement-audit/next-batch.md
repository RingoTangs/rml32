# 后续建议

2BL 已补充占用目的槽、最后 owner、共享 DAG 和 Function creator 释放。下一批可独立规划对象状态转换与已准备事务交错：重点区分 Alive、Pending、Shell、Freed 的 raw 传输、script 归一化和 creator 失效，沿用原配额契约并明确失败时已预留资源的归还边界。当前 2BL 所有对象均 Alive，不能据此提升这些状态组合的覆盖。

实际 32 位 Rust 回归仍未运行；有获授权的现成环境后单独补齐。20 个 native unknown 需要另行精确取证；本批不提升直接捕获、implementation_ready 或阶段2完成状态。
