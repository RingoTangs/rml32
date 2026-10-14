# 阶段 2M：对象生命周期静态分析与 Rust 设计

## 一、总体结论

`magic_Linux32` 的对象创建是一个由返回回调推进的多阶段流程：**加载程序 → 分配并登记对象 → 逐层初始化 → 逐层构造 → 返回对象**。所有初始化段先执行完毕，再执行所有构造函数；正常析构按相反顺序遍历。原生对象的失效、变量释放、hash移除和对象壳释放不是同一个时刻。

本阶段使用 **REA MCP + Ghidra 12.1.4** 静态观察，新增54条记录，复用12条同目标记录。目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。没有执行ELF、脚本、初始化段或业务函数；没有修改Rust运行时、公开API或依赖。阶段2L仍为55种限定操作码、171项Rust测试。

下文“确认”指反编译与指定指令/引用支持的静态事实；“设计”是Rust实现选择；“未知”不补作原生行为。完整原始观察及局限见[证据索引](vm-object-lifecycle-evidence/index.md)、[函数原始结果](vm-object-lifecycle-evidence/functions.md)，删除路径的逐分支说明见[删除调查](vm-object-lifecycle-evidence/deletion-findings.md)。

## 二、关键函数与调用关系

| 环节 | 函数及地址 | 确认职责 |
|---|---|---|
| 程序加载 | vm_createProgram `0x080b13f4` | 编译/读取程序，查找并调整生命周期函数属性，不执行对象构造 |
| 模板加载 | vm_loadObject `0x080b1e90` | 规范化名字，复用oid0对象或加载程序并分配oid0对象 |
| 对象分配 | vm_allocateObject `0x080b1d08` | 建立hash条目、变量区及程序引用 |
| 低层克隆 | vm_cloneObject `0x080b1e4c` | 选择非零未占用oid，与模板共享程序，重新分配变量 |
| 脚本入口 | vm_f_load_object `0x080f8cbc` / vm_f_clone_object `0x080f8eac` | 检查参数，安装失败处理，启动第一段初始化 |
| 初始化帧 | vm_callInInitFun `0x08094258` | 设置来源constitution、initCodeSegment、零参数和execRef |
| 初始化返回 | vm_returnFromFakeFun `0x08094374` | opcode48，恢复调用方，释放执行引用并调用返回回调 |
| 创建推进 | vm_f_construct_object2 `0x080f90b4` | 根据initStep依次推进初始化与各层create |
| 创建失败 | vm_f_construct_object_failed `0x080f93b0` | 标记析构中，安装crash hook，进入回滚 |
| 析构入口 | vm_f_destruct_object `0x080f9420` | 检查对象与重复进入，启动逆序析构 |
| 析构推进 | vm_f_destruct_object2 `0x080f9284` | 遍历已访问层的destruct，最后低层删除 |
| 低层删除 | vm_destructObject `0x080b1894` | 根据execRef/ref延迟或释放变量、解除hash、回收对象壳 |
| 执行引用退出 | vm_unExecRefObject `0x080b1804` | 待删对象最后执行引用退出后入队并失效 |
| 延迟清理 | vm_freeObjectsInDestructingList `0x080b1a04` | 清理队列并释放队列持有的引用 |
| backend边界 | vm_endForBackend `0x08096f44` | 调用延迟清理；不是每条指令都清理 |

主链为 `load_object/clone_object → callInInitFun → opcode48 → construct_ok1 → construct_object2`。create返回经 `construct_ok2` 丢弃返回值再推进。析构成功经 `destruct_func_ok` 丢弃返回值再推进；析构异常经 `destruct_func_failed` 继续下一层。调用者查询确认 `callInInitFun` 的三个已解析入口正是两个efun入口和construct_object2（Evidence `ev_532b40a2912aa0075bd6feae89ca27ecba4c13cb04f28a6806eabe11d8501948`）；间接引用可能不完整。

## 三、分配、初值与可见性

**确认：**分配函数清零64字节对象结构，设置program、oid、uid/euid=255；oid0初始flags为`0x1000`，克隆为`0x1001`。指令 `0x080b1d98–0x080b1da7` 在变量区分配之前插入hash；变量分配失败则解除条目、释放对象并报错。成功后才增加program.ref（`0x080b1e20–0x080b1e23`）。证据：`ev_6b2366257b77aa664004a71adc8eb5f4406467f408f512967e4de1249411e062`。

变量区按root.maxVariableNo分配，每个24字节值被清零，type=0、attrib=0、ref=1，再登记至值链；不是按声明类型自动填充数字或字符串。证据：`vm_allocateValues` 的 `ev_ff4d127c9992f9d1237d84027810ffe2d4684aa1253a8ec9900bb7d899010ce3`，对应汇编 `ev_d5800f267643d7ee5dde2878266b2d7345b65e5040bf3a244897aa271ce5c690`。Rust映射为 `Undefined` 是基于既有值标签模型的设计。

低层clone不复制模板当前变量值。efun克隆入口另设置template指针并增加模板引用，见 `0x080f8f2b–0x080f8f31`；环境、同级及容纳关系没有在分配时复制。

**确认：**`load_object` 找到已有对象且 `flags != 0x800` 时直接返回，包括仅处于`0x1000`初始化标记的对象；`flags == 0x800` 则走重新初始化分支（`0x080f8d06`、`0x080f8dcc`）。因此不能声称原生只有构造成功后才可见。`0x800`的所有生产路径和重入影响未穷尽，本阶段仅保留精确条件。

## 四、初始化和构造顺序

**确认：**令constitution数为N、初始化排列为order。入口令 `initStep=-N`，启动order[0]的初始化后递增。返回回调不断推进负值阶段；到0之后，按order[0..N)遍历create。阶段2K已证明重建排列中自身0最后；本轮消费者证明同一排列实际用于两个完整阶段，而不是每层init后紧跟该层create。

```c
// 从 vm_f_construct_object2 简化，省略栈、catch与ABI细节。
void advance_construct(Object *o) {
    Program *root = o->program;
    if (o->flags & 0xe000) { finish_with_zero(); return; }
    if (o->initStep < 0) {
        int c = root->order[root->constitution_count + o->initStep];
        install_construct_failure(o);
        enter_init(o, c);                  // 来源程序的init段，argc=0
        ++o->initStep;                     // 在该段字节码实际执行前推进
        on_return(construct_ok1, o);
        return;
    }
    while (o->initStep < root->constitution_count) {
        Constitution *c = &root->cons[root->order[o->initStep++]];
        int slot = c->program->create_offset;
        if (slot == -1) continue;
        slot = add_i16(slot, c->function_base);
        install_construct_failure(o);
        // 原efun有第2参数时，每层create都获得同一个构造参数值。
        enter_user(o, slot, has_second_argument() ? 1 : 0);
        on_return(construct_ok2, o);         // 消费函数返回值
        return;
    }
    o->flags &= ~0x1800;
    finish_with_object(o);
}
```

证据：伪代码 `ev_f8180e2eaf29584441f8a0ae1e5d5e7d0b3e94dbf7d2f01a2000b72058f7324c`；汇编 `ev_da96a5f71d49dbc7b88d7cf809835b39c4a15c293342ba20674155e53dfa3606`。关键点：`0x080f911e`读取create偏移，`0x080f912b`加函数基址，`0x080f9139`先写initStep；`0x080f91b3–0x080f91cd`定位初始化与递增；`0x080f91fc–0x080f9246`决定0或1参数；`0x080f9149–0x080f9156`清理状态。

初始化上下文不是普通函数槽：`vm_callInInitFun`选择constitution.pProgram.initCodeSegment，置argNum=0、ip=0、thisObject保持root对象并增加execRef。`0x08094274–0x08094282`及`0x080942ef–0x0809430e`明确这些字段，证据 `ev_e87fd215c338b444b786777608553c57b73d83dc1d3675ab73c10ec003991e6f`。因此初始化的变量/字符串重定位沿用所属constitution，metadata属于实际init代码段。

opcode48先恢复调用帧、清理参数、调用unExecRefObject，再执行返回回调（`0x080943cc–0x0809440e`）；没有普通用户函数的结果类型检查。证据 `ev_13bcf5ba026e9e5fa004744ae2eb642987af494bebf7a74f0a830da392e28518`。不能用当前VM的49或1d替代48。

**确认：**程序加载阶段通过名称查询并记录create/destruct偏移，缺失置-1；存在却无实现则加载失败。create被追加attrib bit0，create/destruct均确保private bit2，见复用 `ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d`。名称查询走标识符hash或quickFunction表，而非“第一个同名函数槽”，见 `ev_ba5b1993ec6d3677bd709e0fb78467636839e28ccd2802f1a214278e6a287928`。本阶段未重建全语料最终生命周期偏移。

## 五、失败、析构与物理删除

**确认：**析构按 `order[--initStep]`逆序取各层destruct，并加该层函数基址，argc=0。正常构造结束时step=N；create调用前已推进step，因此create失败也会尝试失败层自身的destruct，且会经过此前跳过create的层。初始化失败时step<=0，直接进入低层删除，**不执行业务destruct**。

```c
void advance_destruct(Object *o) {
    while (o->initStep > 0) {
        Constitution *c = &o->program->cons[o->program->order[--o->initStep]];
        if (c->program->destruct_offset == -1) continue;
        enter_destruct_with_catch(o, add_i16(c->function_base,
                                            c->program->destruct_offset));
        // 成功或异常回调都继续本函数；成功时另丢弃返回值。
        return;
    }
    clear_crash_hook(o);
    o->flags &= ~0x2000;
    destruct_object_storage(o);
    if (current_efun_return_type() != TYPE_VOID)
        schedule_next_action_error("Failed to construct object.");
    finish_efun();
}
```

证据 `ev_b1334a21e40fa77529b14cafafd4ce7d1669cdf41e7d2ff014d3bf24431307f3`：`0x080f92b5–0x080f92ea`读取递减顺序和destruct偏移；`0x080f934d–0x080f938e`清理及延迟错误。详细分支以[删除调查](vm-object-lifecycle-evidence/deletion-findings.md)为准。

对象状态位的本轮观察：`0x1000`初始化/构造过程标记，`0x2000`析构过程中，`0x4000`有执行引用时请求删除，`0x8000`已失效。它们不是互斥枚举。重复destruct入口检查`flags&0xe000`，已有相应标志时不再启动一轮；直接传入失效对象的参数验证又可能报错，不能一概说所有重复删除均成功。

低层删除execRef>0时只置`0x4000`。最后执行引用退出后，unExecRefObject脱离环境/模板关系、入延迟队列、置`0x8000`并持有一个普通引用。队列在backend结束等边界清理；随后释放变量、解除hash，普通ref尚存则保留失效壳，最后一次unref才释放壳及程序引用。`makeObjectAlone`自身不解除hash。正常对象ref降到0不等于自动调用业务destruct。

证据分别为 `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`、`ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63`、`ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e`。队列直接调用者为endForBackend、clearEnvironment、shutdownScheduler，引用证据 `ev_f5e402fe68092fc8f3da7f19e3e8f75ed5e9d36f205c394d10e109d9618d37b5`；主驱动doActions本身不包含此调用，不能套用旧认证驱动的实现。

## 六、只读语料结果

| 项目 | 数量与边界 |
|---|---|
| 主ABI程序 | 7,159：object 7,154，global 5 |
| 初始化段指令 | 575,635，metadata非空427段 |
| 最小操作码序列2e/48/1d | 1,274段；仅形状，不宣称可跳过或无副作用 |
| init中的48 / 1c / 1d | 7,154 / 5 / 7,159 |
| init中的07 / 06 | 1,707 / 818；不据此推断调用执行次数 |
| 本地create | 5,268个实现，0空声明；34个原始attrib=6 |
| 本地destruct | 948个实现，0空声明；31个原始attrib=6 |
| create参数 | 无参数5,254；一个参数14：12个0..1 mapping、2个1..1 mixed；10个非零默认描述符 |
| 初始化排列 | 7,154个object排列有效，自身0最后，按2K离线目录策略 |

生命周期名称和参数从101,592条函数的原始成员字节重新核对，按首个NUL前的精确ASCII匹配；不依赖有损显示名字。统计是来源程序本地记录，不能当作每个root最终继承方法集合或原生加载后属性。多父样例`aaa/pack_data/lib_aaa32.pak/daemons/datasaved.o`的排列为`[1,2,3,0]`，自身init段有6条指令；构造消费顺序相同，析构为`[0,3,2,1]`（若全部构造阶段已访问）。完整结果见[摘要](vm-object-lifecycle-evidence/summary.json)、[方法](vm-object-lifecycle-evidence/method.md)。真实参数样例为`aaa/clone/account.o`、`aaa/clone/server.o`的必需mixed参数，以及`feature/common/comm.o`的可选mapping参数；完整包来源及偏移在压缩明细中。

## 七、下一轮 Rust 的确定设计与分阶段边界

以下均为**设计**，本阶段没有新增API。

1. **先保留生命周期元数据，再开放执行。**PreparedProgram保留constitution、初始化排列、各来源init代码体及显式生命周期偏移绑定。初始化体使用独立身份，不能伪造负函数槽；函数/初始化共享指令解码和来源池。完整名称查找尚未闭合前，由调用方按来源键显式提供create/destruct槽，验证非空实现、容量和来源，不自动猜同名优先级。
2. **引入独立任务与状态。**拟议`Vm::begin_initialize(object)`、`begin_construct(object, parameter: Option<Value>)`、`begin_destruct(object)`经`run(budget)`推进；初始化与构造使用同一持久任务的两个阶段，构造结束前不返回成功。内部状态记录phase、next_init、visited_create及清理队列标志，分别表示进度和失效，不将原生重叠位硬塞成单一枚举。保留现有ObjectStore.create/remove显式测试接口，不偷偷触发回调。
3. **严格分离两批次。**下一实现批次先支持显式初始化任务，新增2e无操作与48初始化返回，支持现有55种路径可表达的init代码；其他未支持指令、global初始化和初始化中的生命周期重入明确拒绝。1d只代表finishAction，不能作为初始化成功；首批初始化任务若在48之前执行到1d则受控失败。变量默认Undefined，root只有一份稳定Cell；来源constitution控制代码、池和基址。首次失败终止该初始化任务并清理新建对象，不执行业务析构。
4. **构造/析构随后独立实现。**在生命周期局部异常恢复完成后才开放begin_construct/begin_destruct，不以当前VM终止错误模型冒充原生catch。每层create固定0或1个普通参数，同一可选mapping值按共享值规则传递，处理加载后bit0及private属性；atomic仍明确拒绝。按访问游标而非“成功构造列表”回滚，析构异常记录诊断后继续；已提交的外部对象写入不回滚。首批拒绝同对象生命周期重入、热更新及调度器重入。
5. **预算与失效。**暂停必须保留初始化/构造/析构游标和执行引用；任务转移不是新一轮预算。物理释放延后至帧退出后的受控安全点；对象失效与存储存活分别记录，不能因Cell仍被Rc保留就恢复可调用性。生命周期期间的名称可见性不仿造全局命名空间，首批仅显式句柄API，公开文档说明这是受限策略。

未来错误定位为root程序/对象、来源程序、constitution与阶段；普通函数仍保留分派槽，初始化则以独立代码体身份定位pc/原始偏移。原生初始化提前发布对象、精确0x800重入和完整名称表选择尚未复刻，须标为受限兼容差异。

后续验收用合成程序覆盖：父子与多个父的两批次顺序、变量初值/池归属、48恢复、分步预算；初始化首/中/末失败均无业务析构；create失败包含当前层的逆序回滚；缺失create但有destruct；析构异常继续；重入拒绝；活动帧与引用失效；边界清理后程序/对象资源不泄漏。这些运行测试留给对应实现阶段，不在本轮声称通过。

## 八、验证与尚未闭合项

本轮171项Rust测试、格式及Clippy检查通过；19项合成元数据测试通过；默认只读统计重算逐字节一致。全量比较覆盖12个包、9,527条成员、7,159个程序、9,933条继承；仅保留已知`dba/pack_data/etc.pak/etc`压缩异常。36个原始制品及此前证据保持不变，详见[验证记录](stage2m-validation.json)。

问题状态：分配/初值、初始化消费者与顺序、构造参数/回调、正常与失败析构顺序、执行引用延迟清理为**已确认**；生命周期名称最终选择、原生重入可见性、全部失败出口和crash hook覆盖为**部分确认**；热更新、跨线程调度、完整对象命名空间及裸引用安全仍**未解决**。`Vm_Object_T`短名类型查询未命中精确数据库类型，不代表没有DWARF；本报告关键偏移以实际指令核对。

所有统计均为静态结构，不证明原版运行行为或Rust业务执行等价。无需安装glibc.i686/libstdc++.i686；它们仍只是未来原版Linux32运行对照的依赖。
