# 阶段 2D 证据索引

新会话 Ghidra 12.1.4，68 条记录，完整保留参数、分析配置、结果和局限。`rea.json.gz` 是 REA 原始导出文件的确定性 gzip；`reused-dispatch.json.gz` 从已有证据包保留原始分派器记录，不是本轮重分析。

| 函数/查询 | 操作 | Evidence ID |
|---|---|---|
| `vm_.*(Garbage|garbage|Collect|collect|Reference|reference|RefValue|refValue)` | search_procedures | `ev_8177ba2a26295389b9842e60d0b8fc4ea67f984b2b3179aad5767a9252a4b898` |
| `vm_.*(Pointer|Ptr|Member|Assign|Deref)` | search_procedures | `ev_d16b4b00f73d929ced432c867b790ea3bbacaac1dfc853d88455c37d441b4c66` |
| `vm_.*(StaticValue|MapNode|Free.*Value|ObjectValue|DeleteObject|DestructObject|clean.*Value)` | search_procedures | `ev_10ec00cdbd757643a40a3566d8ef96a160f68c5f768182dce3ebc466586294fc` |
| `vm_assignValue` | procedure_assembly | `ev_fd0c36e7c899550aed3734401ee4c11b3d606fd07bb637b33985c741a0822dcd` |
| `vm_assignValue` | procedure_pseudo_code | `ev_16636c23591e5a4b1f66e2d639b1da8c96fca5404ca5dc6a73b1eb3f8974a1c1` |
| `vm_assignValue` | procedure_references | `ev_1ef9f117ba127c47d0d8cb76f713d59b197a5473effc57b7b78809b9b9da043b` |
| `vm_assignValue` | procedure_references | `ev_5de76295204efbad32f935d50b261520d84c3ee4fea3df079e8905544dcbf20d` |
| `vm_assignValueReverse` | procedure_assembly | `ev_b5106f08b96c169b0aeaaf83650f4673c60f685fe9f1195bc3f5c867f90d0289` |
| `vm_assignValueReverse` | procedure_pseudo_code | `ev_5c9e1932bde415d90274f091f4ad7eb199e7e274a93bce3e2234d0a63fad447a` |
| `vm_callInUserFunBody` | procedure_pseudo_code | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` |
| `vm_checkArgNum` | procedure_pseudo_code | `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9` |
| `vm_checkIsRefType` | procedure_pseudo_code | `ev_4f2bd1197ee3cfcc577df7fb16ae4e1619badb4b73e7dda33475e7566f15ed0e` |
| `vm_checkIsRefTypeI` | procedure_pseudo_code | `ev_3abfd431032fdb9b3462a59209ad2de9c1b0a205fdeec563e0f158198fa8ddef` |
| `vm_cleanupValue` | procedure_pseudo_code | `ev_6eecd8e699df5eefd7a3a4825b0b2c554b2844c7bf1228db97aa0803c3d8900e` |
| `vm_convertToPtr` | procedure_pseudo_code | `ev_a2b25c7c1116645bf8e17c95fb0760fd8a8046ce913401ae3ebe14c9f34c885c` |
| `vm_convertToValue` | procedure_assembly | `ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956` |
| `vm_convertToValue` | procedure_pseudo_code | `ev_8a351c14040cefb54204c1d1879ec273798f7579844be1503f87df0f5a5783dc` |
| `vm_convertValueToConst` | procedure_assembly | `ev_27174938ea17a652d5f14712c667798de8416b58f3e9178d0f204341437021ec` |
| `vm_convertValueToConst` | procedure_pseudo_code | `ev_b6460e04232fdc6c08106ab85659c62fb0bfe8932b841606f522549d510bbd48` |
| `vm_copyValue` | procedure_pseudo_code | `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33` |
| `vm_destructObject` | procedure_pseudo_code | `ev_196547cb37731a3632d63fbc4a62d83a57ac5f788fa57a3ed95657b8a8346c69` |
| `vm_destructObjectValues` | procedure_assembly | `ev_d50f92fab09f479b0532c35c9c57bb161f0484bdebcf5d1545ecb10b7ace15be` |
| `vm_destructObjectValues` | procedure_pseudo_code | `ev_d8173d61d8d105723534cddea8b9f99fadfed21f978546de6160508e896cd5ec` |
| `vm_destructObjectValues` | procedure_references | `ev_01a65a72f2a182a9d1901125baffd24e720b82c4f3a1c8ea9d871aed1fe24189` |
| `vm_destructObjectValues` | procedure_references | `ev_1643799a95f82ea4e81fe0c49b2639866e471a6ab8a47eb18dad59d89ef07dd1` |
| `vm_destructStaticValue` | procedure_pseudo_code | `ev_b7275ea0d205e76bd66e12eb46ee8132bb75cd0db3113213fdd60970242bb1c2` |
| `vm_f_garbage_collect` | procedure_pseudo_code | `ev_8fecedeb1f644246e085db4dbc5c875cfb14fd6368045b63417b2183eab987ab` |
| `vm_freeNode` | procedure_assembly | `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae` |
| `vm_freeNode` | procedure_pseudo_code | `ev_921c6ee51c237f77c7b33c2b6e0fc80b3f2e7370fc5984d34f346c4c71cb18f3` |
| `vm_freeNode` | procedure_references | `ev_a861b015bf769a528ec418daa55c88e54d8db8d6273d9deddb9cf474bd3719e0` |
| `vm_freeObsoletedMapNodes` | procedure_pseudo_code | `ev_aede162e24387c626fea56b29104aeb7418eda0d05ab642ae9369e131d7bd0eb` |
| `vm_freeStackValue` | procedure_pseudo_code | `ev_31db47dcac982a1fc4e14f6cc027adc6bc4af49e1cf51b0981c819b2291a5d29` |
| `vm_freeValue` | procedure_pseudo_code | `ev_d4d67468e567147e05b1c04a33a2c8c38a4c356bb628f8e8ef800bc71d69e04c` |
| `vm_getValueFromStack` | procedure_pseudo_code | `ev_18ddcc6b30e7bf221782adbd2639a11d8f2748da1e50834dabfb381fe4a0951e` |
| `vm_getValueType` | procedure_pseudo_code | `ev_4b11f8b8a188ce3208935d909fe583bb7f81ba977c29dbbede57d5c0dbe4861c` |
| `vm_incValue` | procedure_pseudo_code | `ev_78885cb7407e1998e0713117740b868e419ff4f1befde50c3575c204ee59a48b` |
| `vm_initValue` | procedure_pseudo_code | `ev_6921332b80a7a8467156e8214b2cf335f37f0c97324f7e7737d1d6c56b44b54d` |
| `vm_markActiveFromValue` | procedure_assembly | `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` |
| `vm_markActiveFromValue` | procedure_pseudo_code | `ev_dc9d455696d05ef4b444be9dc273ee408ac79c93d4c11138399d240e3b7a82a5` |
| `vm_opEqValue` | procedure_pseudo_code | `ev_983266641ef48cd9821c04963ca8b83351cf78fef7b46f841916bcb58d93139e` |
| `vm_pushGlobalVptr` | procedure_pseudo_code | `ev_e0b26424bac92dc860aadf077630177dfff6e85d228ddea8d8a8e7a77fb8b7ee` |
| `vm_pushLocalVar` | procedure_pseudo_code | `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5` |
| `vm_pushLocalVarI` | procedure_pseudo_code | `ev_c18e607e151b8a0fd808d2d0882f548466d6d416b260f8f1deb40ac30a20e833` |
| `vm_pushLocalVptr` | procedure_assembly | `ev_9b4e2d24822946ff7ae5449f7d30b3e3b2cc1134ef9c530294dd9a840f45feed` |
| `vm_pushLocalVptr` | procedure_pseudo_code | `ev_cdf3f69a6cd4c53275ebfcb955c6419230978dcbe6c1af8e4d0c1dc6d6c32de2` |
| `vm_pushLocalVptr` | procedure_references | `ev_bf12abf3078ae8de6c1b4870cb5ef90c0f592c3fe4560ffb37250560f1a993ec` |
| `vm_pushLocalVptrI` | procedure_pseudo_code | `ev_ec02e5266dd7cdabb5badbfbbbd1f96b4c28ebe63188afc23d23f0fff7f53866` |
| `vm_pushObjVptr` | procedure_pseudo_code | `ev_525701540b4de9914051ebdb9af2e55a2baac12f24fdacd8fe87d50535876b6f` |
| `vm_pushVptr` | procedure_pseudo_code | `ev_84840f7531d08cddf363ae350d16487e6588a33a203d0cdbc63dd3c0ff8f68ac` |
| `vm_queryMember` | procedure_pseudo_code | `ev_2aead7613b91a786e731943745c0a2606d930e5db98d60ad0bd59ae2453e87b4` |
| `vm_queryMemberPtr` | procedure_assembly | `ev_eda665f39ff600270e7c381c4fcb9ee162e674d26a060d8afb345e0a012dc7b4` |
| `vm_queryMemberPtr` | procedure_pseudo_code | `ev_39ba42f6244debda18e95f0fa40350e37543e9ee85177f3d546be50234c8f76a` |
| `vm_queryMemberPtr` | procedure_references | `ev_1870556bc9ea18617c8f4e1d0ac147a9371f14283382ea32a28f5431de2b896f` |
| `vm_queryMemberPtr` | procedure_references | `ev_5ea261a561e7fada81b9209a8e598b9ebbaeaa5ed8a3c115c2c60d68d79157ef` |
| `vm_queryRangePtr` | procedure_assembly | `ev_ee4e18126002b60c36b3820be4291ff2ef01d3d9fbf8ab2c06dea8f0d219da69` |
| `vm_queryRangePtr` | procedure_pseudo_code | `ev_632b1f05dbc861527e3897578f8fd248be43de594040f3bc2a718cdf982e9969` |
| `vm_refreshAllValuesActive` | procedure_pseudo_code | `ev_11d1d264620ab7f5452f093971acc5d836f0364c7d522a7234d1af509587c5fd` |
| `vm_returnFromFun` | procedure_pseudo_code | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
| `vm_safeFindForInsert` | procedure_pseudo_code | `ev_f5dcab63cd814feec7e3bd3ea3639466621466cb34db0c80932a887527db2216` |
| `vm_setLocalVar` | procedure_pseudo_code | `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc` |
| `vm_setObjVar` | procedure_pseudo_code | `ev_85a706f75e7ed097afaab10b0d9474c379d806b7272d872c2ea29584a5f56438` |
| `vm_stackTopValue` | procedure_pseudo_code | `ev_a214d11173e3b233267bdc24c3c70e7956ca4a1fc14d283c514f5e13cedd06a2` |
| `vm_takeOffValueFromStack` | procedure_assembly | `ev_c2177a4cc3c48912f1626bc399600d302a4c708f69bf066dade3508055bf7119` |
| `vm_takeOffValueFromStack` | procedure_pseudo_code | `ev_99c34e4e373606ceaa48084b7cbe89c70742d22173dfb81bb36c8bddcd94b778` |
| `vm_unrefArray` | procedure_assembly | `ev_8000dac56416a0239afddc630d10d5c425e7d9797f623755e77dcda7e5516c26` |
| `vm_unrefArray` | procedure_pseudo_code | `ev_3a18d97f17b48e830607f072dd18b5dcc20f00748883431276baba280c0ab592` |
| `vm_unrefMapping` | procedure_pseudo_code | `ev_d7c17baca06cfda52f8044cdba3a5aa752e6810b7a2335c3929fe6c7a82442f4` |
| `vm_unrefValue` | procedure_pseudo_code | `ev_7caa046f8081143ac8be588f8afdae8a94f7aa7a36d7146bc503150e7f9475a0` |

复用分派器：`ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`；来源 `docs/evidence/magic_Linux32-rea.json.gz`，目标身份相同。
