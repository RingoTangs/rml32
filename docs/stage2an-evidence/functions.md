# 2AN 函数证据索引

REA/Ghidra 静态观察；伪代码不是原始源码。完整记录及限制在压缩包中。新会话的 vm_callInEfun 与历史 Evidence ID 相同，是刷新观察，不计为独立新证明。

- `vm_callInEfun@0x8093d98` · analyze_function · `ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f` · rea.json.gz
- `vm_callInUserFunBody@0x8093eb0` · analyze_function · `ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c` · rea.json.gz
- `vm_finalizeFun@0x8095794` · analyze_function · `ev_7ad5b825f58978d24db93a52b2b74151085c158ff48f1957a00188e9292c6a2d` · rea.json.gz
- `vm_freeFunctionValue@0x808c114` · analyze_function · `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` · reused.json.gz
- `vm_pushValue@0x8091074` · analyze_function · `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212` · reused.json.gz
- `vm_copyValue@0x808be24` · analyze_function · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · reused.json.gz
- `vm_callFunPtr@0x809441c` · analyze_function · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · reused.json.gz
- `vm_cleanupValue@0x808ad98` · analyze_function · `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc` · reused.json.gz
- `vm_createFunctionValueEx@0x8091cd0` · analyze_function · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · reused.json.gz
- `vm_doActions@0x8097008` · procedure_pseudo_code · `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` · reused.json.gz
- `vm_traceError@0x8093520` · procedure_pseudo_code · `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4` · reused.json.gz
- `vm_createFunctionValue@0x8091f34` · analyze_function · `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444` · reused.json.gz
- `vm_errorQuit@0x80ab104` · procedure_pseudo_code · `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1` · reused.json.gz
- `vm_traceError@0x8093520` · procedure_assembly · `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` · reused.json.gz
- `vm_returnFromFun@0x8094978` · procedure_pseudo_code · `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` · reused.json.gz
- `vm_returnFromFun@0x8094978` · procedure_assembly · `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` · reused.json.gz
- `vm_getArgument@0x808a364` · analyze_function · `ev_80271c109d8b170b4dac40d9836f89a9722c8d04023f8e58b881ce2adc69c17e` · reused.json.gz
- `vm_returnFromEfun@0x8093c5c` · analyze_function · `ev_e86eb89e60b57e211e2efdce9ee32fdd5b9d630f548d18ca4e6b50e43675595c` · reused.json.gz
- `vm_addEfuns@0x8085ce4` · analyze_function · `ev_a83bf890b4b1b7b1e670ffd84e2caf433106c4b1521c658ee8c18bbdb826b9a1` · reused.json.gz
- `vm_finalizeIdentifiersInProgram@0x80854b4` · analyze_function · `ev_a62e3110c8f80b68de1c768894f66d34b9dca9778fa69b0bb7f84f2504e73e0c` · reused.json.gz
