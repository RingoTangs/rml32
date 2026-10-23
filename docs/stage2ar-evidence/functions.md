# 2AR函数与记录清单

符号范围不是Ghidra CFG范围；完整inclusive分段和哈希见function-manifest.json。所有provider限制保留在完整记录与清单。

| 函数 | 地址/符号字节数 | 记录来源与操作 | Evidence |
|---|---|---|---|
| vm_unrefValue | 0x808bfb4 / 115 | rea.json.gz · analyze_function | `ev_18fea6ebc3680cd0ea4726c67901206fe1299bda13ef18741e124aa8a6eb77de` |
| vm_unrefMapping | 0x809c488 / 150 | rea.json.gz · analyze_function | `ev_2aae5aa932757dd2dad4354599e352e3310dbf9d1f7637b49903392349474966` |
| vm_error | 0x8097b24 / 280 | rea.json.gz · analyze_function | `ev_f1c4dde0e0ce2a55aa3838ae691f272691496d87fcbc3d91ec17f09ea6eeaea1` |
| vm_assignValue | 0x808f634 / 108 | reused.json.gz · procedure_assembly | `ev_fd0c36e7c899550aed3734401ee4c11b3d606fd07bb637b33985c741a0822dcd` |
| vm_assignValue | 0x808f634 / 108 | reused.json.gz · procedure_pseudo_code | `ev_16636c23591e5a4b1f66e2d639b1da8c96fca5404ca5dc6a73b1eb3f8974a1c1` |
| vm_assignValueReverse | 0x808e914 / 128 | reused.json.gz · procedure_assembly | `ev_b5106f08b96c169b0aeaaf83650f4673c60f685fe9f1195bc3f5c867f90d0289` |
| vm_assignValueReverse | 0x808e914 / 128 | reused.json.gz · procedure_pseudo_code | `ev_5c9e1932bde415d90274f091f4ad7eb199e7e274a93bce3e2234d0a63fad447a` |
| vm_callFunPtr | 0x809441c / 1372 | reused.json.gz · analyze_function | `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` |
| vm_callInUserFunBody | 0x8093eb0 / 463 | reused.json.gz · analyze_function | `ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c` |
| vm_checkArgNum | 0x809521c / 652 | reused.json.gz · analyze_function | `ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126` |
| vm_cleanupValue | 0x808ad98 / 557 | reused.json.gz · analyze_function | `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc` |
| vm_convertToValue | 0x808c028 / 142 | reused.json.gz · procedure_assembly | `ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956` |
| vm_convertToValue | 0x808c028 / 142 | reused.json.gz · procedure_pseudo_code | `ev_8a351c14040cefb54204c1d1879ec273798f7579844be1503f87df0f5a5783dc` |
| vm_copyValue | 0x808be24 / 197 | reused.json.gz · analyze_function | `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` |
| vm_createFunctionValue | 0x8091f34 / 127 | reused.json.gz · analyze_function | `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444` |
| vm_createFunctionValueEx | 0x8091cd0 / 611 | reused.json.gz · analyze_function | `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` |
| vm_destructObjectValues | 0x80b11a8 / 176 | reused.json.gz · procedure_assembly | `ev_d50f92fab09f479b0532c35c9c57bb161f0484bdebcf5d1545ecb10b7ace15be` |
| vm_destructObjectValues | 0x80b11a8 / 176 | reused.json.gz · procedure_pseudo_code | `ev_d8173d61d8d105723534cddea8b9f99fadfed21f978546de6160508e896cd5ec` |
| vm_destructStaticValue | 0x808c0b8 / 89 | reused.json.gz · procedure_pseudo_code | `ev_b7275ea0d205e76bd66e12eb46ee8132bb75cd0db3113213fdd60970242bb1c2` |
| vm_doActions | 0x8097008 / 847 | reused.json.gz · analyze_function | `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` |
| vm_errorQuit | 0x80ab104 / 114 | reused.json.gz · procedure_assembly | `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` |
| vm_errorQuit | 0x80ab104 / 114 | reused.json.gz · procedure_pseudo_code | `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1` |
| vm_f_garbage_collect | 0x80f9628 / 188 | reused.json.gz · procedure_pseudo_code | `ev_8fecedeb1f644246e085db4dbc5c875cfb14fd6368045b63417b2183eab987ab` |
| vm_freeFunctionValue | 0x808c114 / 229 | reused.json.gz · analyze_function | `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` |
| vm_freeNode | 0x809c33c / 98 | reused.json.gz · procedure_assembly | `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae` |
| vm_freeNode | 0x809c33c / 98 | reused.json.gz · procedure_pseudo_code | `ev_921c6ee51c237f77c7b33c2b6e0fc80b3f2e7370fc5984d34f346c4c71cb18f3` |
| vm_freeObsoletedMapNodes | 0x809f24c / 75 | reused.json.gz · procedure_pseudo_code | `ev_aede162e24387c626fea56b29104aeb7418eda0d05ab642ae9369e131d7bd0eb` |
| vm_getArgument | 0x808a364 / 645 | reused.json.gz · analyze_function | `ev_80271c109d8b170b4dac40d9836f89a9722c8d04023f8e58b881ce2adc69c17e` |
| vm_initValue | 0x8089f10 / 101 | reused.json.gz · procedure_pseudo_code | `ev_6921332b80a7a8467156e8214b2cf335f37f0c97324f7e7737d1d6c56b44b54d` |
| vm_markActiveFromValue | 0x808d994 / 342 | reused.json.gz · procedure_assembly | `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` |
| vm_markActiveFromValue | 0x808d994 / 342 | reused.json.gz · procedure_pseudo_code | `ev_dc9d455696d05ef4b444be9dc273ee408ac79c93d4c11138399d240e3b7a82a5` |
| vm_popNStack | 0x8091ab0 / 63 | reused.json.gz · analyze_function | `ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4` |
| vm_pushGlobalVar | 0x809113c / 107 | reused.json.gz · analyze_function | `ev_f5d36761261bd90bb2139ae2bcd05e79af64a90eaffe2b978b3c0353a3ffe4a2` |
| vm_pushLocalVar | 0x80914dc / 255 | reused.json.gz · procedure_pseudo_code | `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5` |
| vm_pushObjVar | 0x8091224 / 123 | reused.json.gz · procedure_assembly | `ev_9a9cde77fb2df1b8214e667458663471ea6da5fbeea24bfb3878fcb8f7548808` |
| vm_pushObjVar | 0x8091224 / 123 | reused.json.gz · procedure_pseudo_code | `ev_03adb0f3a115b83f2497d37852e6c64ffd233cbe06b82b9e57624900a2a01c09` |
| vm_pushValue | 0x8091074 / 91 | reused.json.gz · analyze_function | `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212` |
| vm_queryMember | 0x808fa60 / 675 | reused.json.gz · procedure_assembly | `ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459` |
| vm_queryMember | 0x808fa60 / 675 | reused.json.gz · procedure_pseudo_code | `ev_2aead7613b91a786e731943745c0a2606d930e5db98d60ad0bd59ae2453e87b4` |
| vm_queryMemberPtr | 0x808f8e4 / 379 | reused.json.gz · procedure_assembly | `ev_eda665f39ff600270e7c381c4fcb9ee162e674d26a060d8afb345e0a012dc7b4` |
| vm_queryMemberPtr | 0x808f8e4 / 379 | reused.json.gz · procedure_pseudo_code | `ev_39ba42f6244debda18e95f0fa40350e37543e9ee85177f3d546be50234c8f76a` |
| vm_refreshAllValuesActive | 0x808dd0c / 408 | reused.json.gz · procedure_pseudo_code | `ev_11d1d264620ab7f5452f093971acc5d836f0364c7d522a7234d1af509587c5fd` |
| vm_returnFromEfun | 0x8093c5c / 315 | reused.json.gz · analyze_function | `ev_e86eb89e60b57e211e2efdce9ee32fdd5b9d630f548d18ca4e6b50e43675595c` |
| vm_returnFromFun | 0x8094978 / 864 | reused.json.gz · analyze_function | `ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215` |
| vm_setLocalVar | 0x808f6dc / 282 | reused.json.gz · procedure_pseudo_code | `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc` |
| vm_takeOffValueFromStack | 0x8092008 / 224 | reused.json.gz · analyze_function | `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535` |
| vm_traceError | 0x8093520 / 1850 | reused.json.gz · procedure_assembly | `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` |
| vm_traceError | 0x8093520 / 1850 | reused.json.gz · procedure_pseudo_code | `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4` |
| vm_unExecRefObject | 0x80b1804 / 70 | reused.json.gz · procedure_assembly | `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` |
| vm_unExecRefObject | 0x80b1804 / 70 | reused.json.gz · procedure_pseudo_code | `ev_a208cbdd90503fb624273316e1f15e2eee556bf230a0dee842e864b4ecb72437` |
| vm_unrefArray | 0x809b350 / 140 | reused.json.gz · procedure_assembly | `ev_8000dac56416a0239afddc630d10d5c425e7d9797f623755e77dcda7e5516c26` |
| vm_unrefArray | 0x809b350 / 140 | reused.json.gz · procedure_pseudo_code | `ev_3a18d97f17b48e830607f072dd18b5dcc20f00748883431276baba280c0ab592` |
| vm_unrefObject | 0x80b1618 / 133 | reused.json.gz · procedure_pseudo_code | `ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e` |
