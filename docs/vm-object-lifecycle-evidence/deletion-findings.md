# 删除、析构与失败清理调查

本页是阶段 2M 的独立调查记录。引擎为 Ghidra 12.1.4，目标 SHA-256 为 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。所有结论来自静态证据；没有执行 ELF、脚本或业务字节码。

## 已确认的业务析构链

`vm_f_destruct_object`（`0x080f9420`）接受恰好一个 object/string 参数。字符串走 `vm_lookupObjectByName`，再转换为对象参数。直接对象若有 `0x8000` 标志则报未找到；其他已处于 `flags & 0xe000` 状态的对象只消费参数，避免再次进入业务析构。正常进入时置 `0x2000`、安装 crash hook，调用 `vm_f_destruct_object2`，随后设置 `vm_ctrlFlags |= 0x10`。

- 伪代码：`ev_9a05fb2e7216b29de976bc84c35804409c4b0391f4af9fdcc03e6e0ec6f9bfdf`。
- 汇编：`ev_460c4ca1879ca5bbedea229725c71d14436b1e2f6d42e4c90983a64d2134a069`。`0x080f9480–4c5` 为标志检查、hook 和调用；`0x080f95ac–5b9` 检查失效标志。
- 特别保留原样：字符串路径在 `0x080f957f` 使用 **`flags == 0x800` 的精确比较**，并非 `flags & 0x800`。不以常识替换此事实；完整语义仍需结合查找函数。
- Ghidra 将同一指针误显示为 `Vm_Buffer *`，本页依据 `[object+0xc]` 汇编和相邻函数语义称之为对象，不采用 `realSize` 这一伪字段名称。

`vm_f_destruct_object2`（`0x080f9284`）沿 `initOrder` **逆序**调用每个 constitution 的 `destruct`，每次调用参数数目为零。先将 `initStep` 减一并保存，再读取该位置对应程序的 `destructOffset`，非 `-1` 时加该 constitution 的函数基址，经根对象函数表分派。

- 伪代码：`ev_f17d1950e66d2c98c086472f49d99c9eea4f55ff75edef0ef0e659d5df2c716e`；汇编：`ev_b1334a21e40fa77529b14cafafd4ce7d1669cdf41e7d2ff014d3bf24431307f3`。
- `0x080f92a8–2e6`：有符号步数判定、递减、顺序表读取、`destructOffset` 读取及 **16 位 ADD**。Rust 后续设计应拒绝溢出，不复制回绕。
- `0x080f92ec–328`：建立 thread catch，`vm_callInUserFunBody(0, object, slot, body, 0)`，挂接成功回调。
- 成功回调 `vm_f_destruct_func_ok`（`0x080f95cc`；`ev_a4adb62dd62f7d6e50e24cd56b92f492b3bd3753a66ab3e889e8b59de3ce8db0`）结束 catch、弹出返回值并继续；失败回调 `vm_f_destruct_func_failed`（`0x080f95e8`；`ev_f4fc3a30d4d10d12d35be2daec518e2e8e8e1c601750a418d198e9540ce61de5`）直接继续。因此单个析构失败不会在此回调层阻止其余析构。
- 全部遍历完成后清 crash hook、清 `0x2000`、调用低层 `vm_destructObject`，消费 efun 参数并返回。当前 efun 返回类型若不是 void，还安排下一 action 报 `Failed to construct object`（`0x080f937b–38e`）；这区分了创建失败回滚与普通 void 析构入口。

```c
/* 归纳伪代码；不代表原始源代码。 */
void continue_destruct(Object *o) {
    while (o->init_step > 0) {
        int c = o->program->init_order[--o->init_step];
        int slot = o->program->constitutions[c].program->destruct_offset;
        if (slot == -1) continue;
        slot = native_i16_add(slot, o->program->constitutions[c].fun_base);
        if (slot == -1) continue;
        install_thread_catch(continue_destruct, o);
        call_on_same_object(o, slot, 0);
        on_return(end_catch_pop_result_and_continue, o);
        return;
    }
    clear_crash_hook(o);
    o->flags &= ~0x2000;
    destruct_object_storage_or_defer(o);
    if (current_efun_return_type() != VOID)
        report_on_next_action("Failed to construct object");
    pop_efun_arguments_and_return();
}
```

## 创建失败回滚与重入边界

创建阶段将 `initStep` **在实际 create 调用前推进**。所以 create 失败后的逆序析构包含失败 create 自己所在 constitution，也包含此前略过 create 的 constitution，而非仅成功执行过 create 的项。初始化代码阶段步数为负，调用前推进至零；失败进入 `destruct2` 时 `initStep <= 0`，不调用任何业务 destruct。

依据 `vm_f_construct_object2`（`ev_f8180e2eaf29584441f8a0ae1e5d5e7d0b3e94dbf7d2f01a2000b72058f7324c`）与 `vm_f_construct_object_failed`（`ev_b1358e0f52d40e07afc5936ce0f39b582ec457b229e0df19581b14ab1da71fc2`）。后者若已处于 `flags & 0xe000` 状态则直接返回数字零；否则置 `0x2000` 并转入同一析构链。crash hook `vm_f_destruct_object_crash`（`0x080f95f4`；`ev_8f13fecfa2ec8a0f4f108c3fed82f6a97666cedb45255c04a339049bafd5e83e`）清 `0x2000` 后调用低层删除，绕过剩余业务 destruct。具体 crash-hook 触发机制不能由这些函数独立证明。

## 执行引用、延迟删除与存储释放

`vm_destructObject`（`0x080b1894`）与业务 `destruct` 不同：它负责存储和对象表清理。

- `execRef > 0`：只置 `0x4000` 并返回（`0x080b18a0–8b5`）；变量存储此时保留。
- 否则调用 `vm_makeObjectAlone`，解除绑定值引用，逐槽 `vm_destructStaticValue`，释放变量数组并置空，从对象 hash 链移除。
- `ref == 0` 时减少程序引用，必要时清 initCodeSegment/程序头，再释放对象壳；`ref != 0` 时只置 `0x8000` 保留壳。
- 复用同目标伪代码 `ev_196547cb37731a3632d63fbc4a62d83a57ac5f788fa57a3ed95657b8a8346c69`；新增汇编 `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b`。关键锚点：逐槽 `0x080b197b–996`；hash 摘除 `0x080b19d8`；引用判定 `0x080b193e–96a`。

`vm_unExecRefObject`（`0x080b1804`）先减少 execRef。如果降至 `<=0` 且有 `0x4000`，将对象隔离，加入 `pDestructingList`，置 `0x8000` 并增加一个普通引用。这证明执行引用用于推迟存储清理；普通对象引用仅能保留失效壳，不能保证业务对象有效。

- 伪代码 `ev_a208cbdd90503fb624273316e1f15e2eee556bf230a0dee842e864b4ecb72437`；汇编 `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`，入队完整锚点 `0x080b181c–840`。
- 已有同目标 `vm_callInUserFunBody` 伪代码 `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` 证实入帧增加 execRef。`vm_returnFromFun` 伪代码 `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` 和汇编 `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` 证实先恢复调用者，再 `vm_unExecRefObject`，最后执行 return callback（调用锚点 `0x08094bad`）。

`vm_freeObjectsInDestructingList`（`0x080b1a04`）摘除每个队列节点，调用低层删除，再释放队列自己的普通引用；若剩余引用 `<=0` 且已失效，释放程序引用和对象壳。

- 伪代码 `ev_58e41fc7fd79b8820fddd5d3bb13ce115b1663a228e53fd381f6b10c45480218`；汇编 `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63`，摘链/清理/减引用 `0x080b1a1a–a3f`。
- 实际后端收尾入口 `vm_endForBackend`（`0x08096f44`；`ev_ed6d15234b6b77e70190cfcdc433e895468986122447c7f45f1bd8c78c562ba6`）调用队列清理，同时清过期 JIT、映射节点与计时器。
- `vm_clearEnvironment`（`0x080aca84`；`ev_4a9a0054a1392bc09a3c1dc2129e02c465dc75325ec76a95e30dbebb03a82f67`）和 `vm_shutdownScheduler`（`0x080ae0e8`；`ev_86ea360aa0ccc20146300d4fada064d9d97bd8cc712b8e5d54de240c1cc4b0fd`）也在清进程后排空队列。

`vm_shutdownObject`（`0x080b1bd0`；`ev_25d55e24d66d458ebaf2ceb6aac63cab835b100c5130c82b5138890178888577`）首先遍历 hash 表隔离对象并清绑定值/变量数组，再遍历调用低层删除，最后释放 hash 表；其函数体未调用业务 destruct。此局部事实不说明上层关闭流程此前是否已调用业务析构。

补充确认：`vm_unrefObject`（`0x080b1618`；`ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e`）在普通引用减至 `<=0` 且 `flags & 0x8000` 时释放程序引用和失效对象壳；存活对象的普通引用归零不会在此自动执行业务析构。`vm_makeObjectAlone`（`0x080b1728`；`ev_a1391c5cfc29264f27cbda3809a9bf634f736453297c310dc23f979b12218fef`）解除环境/邻居/容纳对象关系、释放模板对象引用并清模板指针；**它自身不摘除对象 hash 链**，不能将“隔离”写作“已从名称查找表删除”。`vm_destructAllObjects`（`0x080b1ab4`；`ev_542620fbbbb8fabe07f6b3f0e1f5f73d33a239f5aa991ca0d0a8103028909e8d`）也是先清全部对象变量、再低层删除的两遍流程；未在该函数体观察到业务 destruct 调用。

## 尚未确认与后续设计约束

- 已确认生命周期析构顺序、失败继续、回滚步数、执行引用和延迟清理；程序级全局退出的完整调用链及调度器异常解栈不在本页覆盖范围。
- 对象变量引用在存储释放后不能照搬原生悬空指针；Rust 应继续按 ObjectId 验证存活并返回受控错误。
- 不应把 `0x2000/0x4000/0x8000` 简化为一个删除布尔值。待析构、执行中延迟删除、失效但仍有引用是不同状态。
- 新 Rust 生命周期执行若尚未支持 catch/回调，必须明确拒绝相关行为，不能宣称现有终止错误等价于原生“析构失败后继续”。
- 曾检索到 LegacyAuth389 的 `vm_doActions @0x08071478` 队列清理证据，因目标 SHA 不同，**已排除且不得复用**；本目标以 `vm_endForBackend` 证据为准。
