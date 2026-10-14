# 阶段 2K REA/Ghidra 证据索引

目标 `magic_Linux32`；引擎 Ghidra 12.1.4；SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

本轮 41 条记录，复用 10 条记录。类型精确路径未命中只表示该查询 unavailable；后续通过指针类型路径定位到真实结构。会话已关闭，未执行目标。

[本轮完整记录](rea.json.gz) · [复用记录及来源哈希](reused.json.gz) · [可读汇编、反编译与类型](functions.md) · [统计方法](method.md)

## 本轮记录

| 函数/类型 | 操作 | Evidence ID |
|---|---|---|
| `0x0847aac0` | `inspect_native_data_type` | `ev_2f85624cd94d573d180f7a152cebf952c2e43f263ef0edfd0183b1d791ecd0df` |
| `(Consit|onstitut|Hidden|QuickFunction)` | `search_procedures` | `ev_65ed5636b3bd823f79ae5a97a3f5dadf22691bb8def9024d87ebf3995e409295` |
| `(FunctionTable|NewProgram|ProgramFunction|LoadIdentifiers)` | `search_procedures` | `ev_80eb96d6c5760e03f3718dfdb3644fdd0c5c03a78a4e4caad6c82f81d82d1099` |
| `(NewFile|createProgram|newProgram)` | `search_procedures` | `ev_4d7a9201779ba673bbad299d7906ecdfb2892afdc238ee28ee8310c0f0f51586` |
| `vm_gatherProgramFunctionTable` | `procedure_references` | `ev_509af2c0617a2abcd8e0bef552196b3b30edb61561adb1737a054816b5a1d7cc` |
| `vm_gatherProgramFunctionTable` | `procedure_references` | `ev_ddcf4d932d2d34b486bb39494c94b3b2da281cea97eab6a90d1db3be314d3015` |
| `vm_inheritFunctionPrototype` | `procedure_references` | `ev_1d4f5e7a7fd3fa71fb6c3ff5173b0360f4491cc3e130729abeca9786fa4aab4d` |
| `vm_overrideFunction` | `procedure_references` | `ev_8bbd3b32336295312fe983ba72a189972615c7807879ced6552cd6b0022df163` |
| `vm_pushObjVar` | `procedure_references` | `ev_0123e989ef3e5ccf0ded341e9a56232ec9ac31a82567f221250b9da9a5498f51` |
| `vm_pushObjVptr` | `procedure_references` | `ev_3457f1d49a9057b1984e47233ccdaae7d426a1bba1d6c142de5f9cdbaef906b3` |
| `vm_pushSharedString` | `procedure_references` | `ev_6b414fe0bbcd4ccf2e6a84339a529e9cc67cdccd499fa921fddb5ea6ca99bc68` |
| `vm_setObjVar` | `procedure_references` | `ev_2bccf3ddc39d8440eda9b6dc53284ec5f3daef7f0c15bfbab5e8d0c8ed84cc84` |
| `vm_buildProgramHead` | `procedure_assembly` | `ev_ea5a86b9ba60013f7c3c493199e44963814f92ef6614d449a2e2de651aea591a` |
| `vm_buildProgramHead` | `procedure_pseudo_code` | `ev_aa12d41f4394aa209415defcd72e5fc2cfbdee8acf9ff95fe8d33af97ed6b264` |
| `vm_createProgram` | `procedure_assembly` | `ev_7c339189f2227b0560db98446dc83bd0520ad53bb7da51083b39a547badba203` |
| `vm_createProgram` | `procedure_pseudo_code` | `ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d` |
| `vm_gatherProgramFunctionTable` | `procedure_assembly` | `ev_2b87bd494b66569995a2fd20b9c75f34d7fbc63d63d1b161f3aa5e6ed6113c16` |
| `vm_gatherProgramFunctionTable` | `procedure_pseudo_code` | `ev_cbeb0564e51d40f356850c8ce433d229a0063dc15122c784bcac9998341ff27d` |
| `vm_inheritFunctionPrototype` | `procedure_assembly` | `ev_c6179a67691e4485b5ca4e6a6592c20259fb54e358c2c30a060d071de2399f93` |
| `vm_inheritFunctionPrototype` | `procedure_pseudo_code` | `ev_f106a1ad196036705391e3d2710b9e91f713c16845951d9656ce1036e69e1c66` |
| `vm_overrideFunction` | `procedure_pseudo_code` | `ev_d7402d7e06126da2991a947d90a56084eaac0ba169f9b996e21f0550d42925b6` |
| `vm_pushObjVar` | `procedure_assembly` | `ev_9a9cde77fb2df1b8214e667458663471ea6da5fbeea24bfb3878fcb8f7548808` |
| `vm_pushObjVar` | `procedure_pseudo_code` | `ev_03adb0f3a115b83f2497d37852e6c64ffd233cbe06b82b9e57624900a2a01c09` |
| `vm_pushObjVptr` | `procedure_assembly` | `ev_fb501c3a4e5420446efd5a59a2c8196c01d1f8540c4f0c1095c98d12e7b9f8cf` |
| `vm_pushObjVptr` | `procedure_pseudo_code` | `ev_525701540b4de9914051ebdb9af2e55a2baac12f24fdacd8fe87d50535876b6f` |
| `vm_pushSharedString` | `procedure_assembly` | `ev_734b597b55be50ee446a59e8502687b0068270e4791dcc47564e0a019eaeee7c` |
| `vm_pushSharedString` | `procedure_pseudo_code` | `ev_036c7ff9e3a4c3a9d1380520976755debc306a776b18e23161519177ae073205` |
| `vm_regularProcessor` | `procedure_assembly` | `ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597` |
| `vm_regularProcessor` | `procedure_pseudo_code` | `ev_5673fa5b2edb4d5bf79e9f9431a44467493506747573d429692a62ef74c6c5de` |
| `vm_setObjVar` | `procedure_assembly` | `ev_41e51f5930ad32612eae03bb92635d03412ccbcc621485125ad2e864f4c4f75a` |
| `vm_setObjVar` | `procedure_pseudo_code` | `ev_85a706f75e7ed097afaab10b0d9474c379d806b7272d872c2ea29584a5f56438` |
| `/DWARF/vm_action.h/Vm_FunCall_Context/Vm_Program_Consititution` | `inspect_native_data_type` | `ev_582a03dd695738a8d26aac5a532e68919f7c76024a231dc643f2b48941b58b84` |
| `/DWARF/vm_fun.h/Vm_Fun_Entry` | `inspect_native_data_type` | `ev_7501f730d5843b9d3f322072da898824ff7ffbb2f679d79475efa4826b924e66` |
| `/DWARF/vm_fun.h/Vm_Fun_Entry_T` | `inspect_native_data_type` | `ev_2b59e15dff2ae4bd55dc26e6d649415cec1e75b762c5916c7d2ebb211c9b82aa` |
| `/DWARF/vm_lex.h/Vm_FunCall_Context/Vm_Program_Consititution/Vm_Program` | `inspect_native_data_type` | `ev_47dd96b2854cbe63e0080ce518465e7ff291a29d388b4e8ea3f0712fabf620d4` |
| `/DWARF/vm_object.h/Vm_Fun_Entry` | `inspect_native_data_type` | `ev_e6e788fc95d1403d2daa1f3a3db6b4e5ed24081078ff520e5395a292c3d3e8f5` |
| `/DWARF/vm_object.h/Vm_Fun_Entry_T` | `inspect_native_data_type` | `ev_aa2011031fcd40cd856dda4d87e90d57b90648b9140112f2a8e97217afcec0da` |
| `/DWARF/vm_object.h/Vm_Program_Consititution` | `inspect_native_data_type` | `ev_6c621cfcf5bd87d3fb61919af07ef03e196b077423375c048269fb0472bf0dc0` |
| `/DWARF/vm_object.h/Vm_Program_Consititution_T` | `inspect_native_data_type` | `ev_c2f8108a74f7ee3e4de9e063bc3787ad6a10c4911726fdfbe30b7b0fd1875e59` |
| `/DWARF/vm_object.h/Vm_Program_T` | `inspect_native_data_type` | `ev_e2652d11455fff30647dbef03bae6f81c5a31d137bcbafd1e9d0a7274996329a` |
| `/DWARF/vm_program.h/Vm_Program_Consititution_T` | `inspect_native_data_type` | `ev_d857908008f3499090ddda8b2df0aa7e769f73ed525211993d9d1bbde863e1f5` |

## 复用记录

| 函数/类型 | 操作 | Evidence ID |
|---|---|---|
| `vm_callInUserFun` | `procedure_assembly` | `ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347` |
| `vm_callInUserFun` | `procedure_pseudo_code` | `ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97` |
| `vm_callInUserFunBody` | `procedure_assembly` | `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41` |
| `vm_callInUserFunBody` | `procedure_pseudo_code` | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` |
| `vm_finalizeFun` | `procedure_pseudo_code` | `ev_ac11d24ef0dd2c2398a02826fd57a977d4381fa3668add2ab2c2553424e05abd` |
| `vm_loadBinaryInherit` | `analyze_function` | `ev_c41c22c1afd1192242c38bb59ffa5cc3402d157828307978e735721a4489e3d0` |
| `vm_loadIdentifiers` | `analyze_function` | `ev_b786ac699461a76c0683ddb2f8254575187db56f002088762956ac65e57e9528` |
| `vm_loadInheritPrograms` | `analyze_function` | `ev_af5958529d07c6f0f0b9b287dbdec72c60a8ad5632f6fd5b362a60f8d3a4b489` |
| `vm_returnFromFun` | `procedure_assembly` | `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` |
| `vm_returnFromFun` | `procedure_pseudo_code` | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
