# 阶段 2I 原生证据索引

目标 `magic_Linux32`，Ghidra 12.1.4；SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

本轮新增 31 条 REA 记录，包含反编译、汇编、引用和类型检查；分析会话已关闭。类型路径查询的 unavailable 结果仅表示该精确路径未命中，并非类型不存在。

[本轮完整记录](rea.json.gz) · [复用记录及来源哈希](reused.json.gz) · [可读函数证据](functions.md)

## 本轮记录

| 函数/类型 | 操作 | Evidence ID |
|---|---|---|
| `0x08667a68` | `inspect_native_data_type` | `ev_06d68ee2779b09e24c9c4147a265a3ae799765f547578bf03c68129644793740` |
| `(Arg|arg|Default|default|Fun)` | `search_procedures` | `ev_080023dcd5f894c9e792538afcc05219bbf66429f72d148928006964dd02d179` |
| `(resort|default|arg|gen.*(Value|Action))` | `search_procedures` | `ev_3c95f227f35b62acbb04d47d5661a9036fd8985f434e03f4804a0b7533ad9564` |
| `vm_adjustArgumentNum` | `procedure_references` | `ev_e04f598e0861441e9358ac37cbb0e3466e99a4f7f58ef92fe9462739d6a5dfa3` |
| `vm_buildDefaultValueNodes` | `procedure_references` | `ev_1c650eb10db326b06b635b9d9755a9f1a7693f7545ce2fc9f47b9ea6b4c6d619` |
| `vm_adjustArgumentNum` | `procedure_references` | `ev_9f3d040916e8bf460d456d98e240b5e2d119271ef5f7d9d4a5f62cab558c8e9f` |
| `vm_buildDefaultValueNodes` | `procedure_references` | `ev_3bf6987ff1b574b94a659dfb70f74241175f9ef8f20e83188940b46f53d5ff2a` |
| `vm_createArgumentArray` | `procedure_references` | `ev_f559993057f0ecabc76210feabdbaef897b6ceff73b67ecd8e78f0d1ed22be3a` |
| `vm_pushArgNum` | `procedure_references` | `ev_0c687fad352bb734d1f0d96b9a968d94d8120a80522725456f821c6de9041937` |
| `vm_resortStack` | `procedure_references` | `ev_83867023ed9b8813c89b5908b2824066456b313ce6a7c203c040dfe4d5890306` |
| `vm_adjustArgumentNum` | `procedure_assembly` | `ev_ec468cd1bdc9e4960feb347de97932324aa4c65c6b2931c7ef08742241eddbec` |
| `vm_adjustArgumentNum` | `procedure_pseudo_code` | `ev_94eaf2ef0a5e40a1bed6d105f0a4b53eb4f09cb6819fe5694bb3b96ed5fa17da` |
| `vm_argumentInThisContext` | `procedure_assembly` | `ev_414a18bbd953f729969695ef06915f4871121b81f6814d8112df892ab3fe9a14` |
| `vm_argumentInThisContext` | `procedure_pseudo_code` | `ev_94ab3eba34c6a8ee8a84a3290d81f8b440d1c04a08d02850419510fc1d39f1d9` |
| `vm_buildDefaultValueNodes` | `procedure_assembly` | `ev_f7590616a34561205b430f9bfacdab404ef9c0c10d7447b91c9a9726ac29b0bc` |
| `vm_buildDefaultValueNodes` | `procedure_pseudo_code` | `ev_2c27ff7488e9819ae0c431411d6590807eaddf4a9037881bcf3d3e67b56ca294` |
| `vm_createArgumentArray` | `procedure_assembly` | `ev_8a50a757166bed6d254d5e96dafee266f7458421de87a7569ebbd797c419ffe0` |
| `vm_createArgumentArray` | `procedure_pseudo_code` | `ev_ad365e363694b53417408272d3fcbd105f6dbf3a8d1d0703d04ab4bd23b28021` |
| `vm_f_get_args` | `procedure_pseudo_code` | `ev_493f85df70bef526d372e8b86e91e68a4c03f5d7a54177629b294e4d19bab632` |
| `vm_finalizeFun` | `procedure_pseudo_code` | `ev_ac11d24ef0dd2c2398a02826fd57a977d4381fa3668add2ab2c2553424e05abd` |
| `vm_loadBinary` | `procedure_pseudo_code` | `ev_a8404c9fbff4bd62e5d7316362dfa2b97574e7bd2dc5b5c24ed16dff00054019` |
| `vm_pushArgNum` | `procedure_assembly` | `ev_82ac3ae7fae8a726ca218b98efe44a8ac06570221896d15273b0f53feba22c1a` |
| `vm_pushArgNum` | `procedure_pseudo_code` | `ev_ac3d0d44e6c928399cf5c31d3ae6029310624aca2d67e77651a84a51228ad183` |
| `vm_resortStack` | `procedure_assembly` | `ev_637a26f2121f5b0547c5cab5b7e08f6c925ae8aca1945de11e2008a57ed2da66` |
| `vm_resortStack` | `procedure_pseudo_code` | `ev_999d6b2ce2163dabf3dfe97c69fc89bbf621502b7efc62e552dbaefa85dda75e` |
| `vm_undefineLocalVarOrArg` | `procedure_pseudo_code` | `ev_85c2d6b33bdc10ff3f0823cf5ee7d48b21344e4330b6079891f4861ff25eae85` |
| `/DWARF/vm.h/Vm_Fun_Arg_T` | `inspect_native_data_type` | `ev_cf57bb557f0c7ff5c2de562aff56713955e79bf7b287d6e984758f39624ed688` |
| `/DWARF/vm_compiler.h/Vm_Fun_Arg_T` | `inspect_native_data_type` | `ev_a31ddd3f83ffb912741b7eb040d796c0e180fc2f999c0d4cb95f9688fb128730` |
| `/DWARF/vm_fun.h/Vm_Fun_Arg` | `inspect_native_data_type` | `ev_3a2145a4ec5bf063ceb5de533653f40d5ce70cd826ccb17d03b9892fc915df74` |
| `/DWARF/vm_fun.h/Vm_Fun_Arg_T` | `inspect_native_data_type` | `ev_81b2b91d8098c3d46320a56895d7a311cf3adfe16fef44f17543ac1f285cd0d2` |
| `/DWARF/vm_fun.h/Vm_Fun_T` | `inspect_native_data_type` | `ev_9abc40d50269c7de5b2a54455cdf0a9e946644b3a8cf3c8720fc4b213d33707d` |

## 复用记录

| 函数/类型 | 操作 | Evidence ID |
|---|---|---|
| `vm_callInUserFun` | `procedure_assembly` | `ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347` |
| `vm_callInUserFun` | `procedure_pseudo_code` | `ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97` |
| `vm_callInUserFunBody` | `procedure_assembly` | `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41` |
| `vm_callInUserFunBody` | `procedure_pseudo_code` | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` |
| `vm_checkArgNum` | `procedure_assembly` | `ev_d17aa0d7a395baf8b760f4dd773008e55d6a7fc1e8c6931a9657f73663982946` |
| `vm_checkArgNum` | `procedure_pseudo_code` | `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9` |
| `vm_doActions` | `procedure_pseudo_code` | `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` |
| `vm_enterFunction` | `procedure_pseudo_code` | `ev_1782e4cf62d7942391d37161d43a2051ad3aa963b13b7ca61519cc59ae7dad43` |
| `vm_errorQuit` | `procedure_assembly` | `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` |
| `vm_errorQuit` | `procedure_pseudo_code` | `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1` |
| `vm_loadFun` | `analyze_function` | `ev_b513e6242f5447517c22636c5c877a93b397b974851b7928f195106d957a1079` |
| `vm_pushLocalVar` | `procedure_assembly` | `ev_d0c3b683bea4f35107c1c4be31af558ea2635c43a303b9009673e8de1ca083fc` |
| `vm_pushLocalVar` | `procedure_pseudo_code` | `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5` |
| `vm_pushLocalVptr` | `procedure_assembly` | `ev_9b4e2d24822946ff7ae5449f7d30b3e3b2cc1134ef9c530294dd9a840f45feed` |
| `vm_pushLocalVptr` | `procedure_pseudo_code` | `ev_cdf3f69a6cd4c53275ebfcb955c6419230978dcbe6c1af8e4d0c1dc6d6c32de2` |
| `vm_returnFromFun` | `procedure_assembly` | `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` |
| `vm_returnFromFun` | `procedure_pseudo_code` | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
| `vm_setLocalVar` | `procedure_assembly` | `ev_b4762566f69e6ef759a4c846394fb10d47cd1eebf8da295f5f40105ef3884689` |
| `vm_setLocalVar` | `procedure_pseudo_code` | `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc` |
| `vm_takeOffValueFromStack` | `procedure_assembly` | `ev_c2177a4cc3c48912f1626bc399600d302a4c708f69bf066dade3508055bf7119` |
| `vm_takeOffValueFromStack` | `procedure_pseudo_code` | `ev_99c34e4e373606ceaa48084b7cbe89c70742d22173dfb81bb36c8bddcd94b778` |
| `vm_traceError` | `procedure_assembly` | `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` |
| `vm_traceError` | `procedure_pseudo_code` | `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4` |
