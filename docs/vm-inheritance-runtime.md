# 阶段 2L：受限继承链接与 VM 执行

## 总体结果与使用方式

新增显式程序目录与离线链接器，支持单继承、多层继承及没有共同祖先的多继承。链接全部成功后返回 `PreparedProgram`，调用者再注册至 `ObjectStore`。加载、链接、注册和创建对象均不执行初始化、构造或析构。没有新增依赖，操作码仍为55种限定路径。

```rust,ignore
use rml32::{ProgramCatalog, LinkLimits, prepare_linked_program};
let mut catalog = ProgramCatalog::new();
catalog.insert("parent", parent_image)?;
catalog.insert("root", root_image)?;
// 必须传入 Inherit.name.bytes 的完整字节，包括已有的结尾 NUL。
catalog.bind_inherit("root", inherited_name_bytes, "parent");
let prepared = prepare_linked_program(&catalog, "root", LinkLimits::default())?;
let program_id = store.register_program(prepared)?;
let object_id = store.create(program_id, initial_values)?;
```

`insert` 拒绝重复键；相同名称绑定不同父键会在链接时报歧义。目录不扫描磁盘、不推断路径、不跨包搜索。已有 `prepare_program` 单程序入口及对象存储接口保持不变。

## 链接与声明绑定

依赖按父先于子递归链接。默认限制为1024个父依赖、64层继承边及累计1,000,000个函数槽。六字段容量、区间、constitution空洞、局部槽冲突、兄弟区间重叠及signed16基址累加均检查；零长度区间不占槽。循环、重复祖先及菱形继承明确拒绝，global及hidden函数不进入此路径。

内部链接表区分缺失、空声明和已绑定实现。导入父表时保留父别名标记；同名非空函数保持独立槽。空声明按低槽倒序、高槽正序查找，跳过别名和函数attrib bit2，比较原始名字节。参数数量、type、attrib及返回type冲突为 `InvalidProgram`；引用flag或返回attrib差异为 `UnsupportedFeature`，后者是Rust主动收紧的策略。采用最终实现的参数范围、默认段与可变参数属性。

未找到实现的声明保留为未实现，调用时返回 `UnsupportedFeature`；缺失槽返回 `InvalidProgram`。create/destruct声明别名和链接程序中的此类函数执行暂不支持。任何实际实现含未支持指令均使准备失败，不以空操作替代。

## 执行上下文、共享与错误

链接阶段保留完整constitution映射；准备结束后，将其来源键、函数基址、变量基址和共享字符串池投影到不可变分派槽。帧保存root ProgramId、ObjectId及分派槽，由该槽确定实际上下文，无需另存可变的constitution索引。返回弹帧即可恢复调用方。这是与2K草案不同的等价实现方式。

函数指令以 `Rc` 共享；同一来源实现只解码一次，别名仍满足 `function(slot).index() == slot`。07使用当前槽函数基址进行signed16重定位；3d/3e/4e使用变量基址定位root对象的稳定Cell；40使用实现来源字符串池，数值metadata随实际函数体解码。对象ID、引用所有者、只读属性及额度凭据继续沿用既有实现。

运行错误的program/function为root程序ID及分派槽，source/pc/offset为实际实现来源。依赖解析错误包含父路径的继承序号；布局错误保留root来源及具体原因。链接器只读取目录，失败不会部分注册或改变已有对象。

## 证据、验证与限制

实现复用[阶段2K的REA/Ghidra证据](vm-inheritance-linking.md)：加载及重定位 `vm_loadBinaryInherit@0x080a2b58`，导入与声明绑定 `vm_gatherProgramFunctionTable@0x080a31bc`，调用 `vm_callInUserFun@0x08094080`，变量处理器 `0x08091224/0x080911a8/0x0808f7f8`，字符串池 `0x08090cf8`，返回 `0x08094978`。本阶段没有重新推断原生语义，也没有执行参考程序。

验证命令及结果见[阶段2L验证记录](stage2l-validation.json)。合成测试覆盖继承调用、上下文恢复、独立父字符串池、变量写入与双对象隔离、默认metadata、可变参数、引用参数、预算暂停、对象删除、声明选择、冲突和失败原子性。全语料比较只检查加载结构，不能据此声称真实业务字节码已经可执行。

尚未支持动态加载命名空间、热更新、共同祖先、初始化/构造/析构、跨对象方法分派、全局调用及未实现efun。完整服务端重写仍需后续阶段。
