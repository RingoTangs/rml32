# 阶段 2M REA/Ghidra 证据索引

目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。Ghidra 12.1.4；54 条本轮记录，12 条复用记录。未执行目标；分析会话已关闭。

| 来源 | 查询/函数 | 操作 | Evidence ID |
|---|---|---|---|
| 本轮 | `vm_allocateObject` | procedure_pseudo_code | `ev_02d4ce740f27fd92bce43f175d7d6402a86fdbc0c55c9a7cf72664d4092c8530` |
| 本轮 | `vm_newThreadCatch2` | procedure_pseudo_code | `ev_04c0d19e992575aa438242c9b50d1dc5095f14e6645d2794b5e093f287306ba7` |
| 本轮 | `vm_returnFromFakeFun` | procedure_assembly | `ev_13bcf5ba026e9e5fa004744ae2eb642987af494bebf7a74f0a830da392e28518` |
| 本轮 | `vm_freeObjectsInDestructingList` | procedure_assembly | `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63` |
| 本轮 | `vm_doActions` | procedure_pseudo_code | `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` |
| 本轮 | `vm_nullAction` | procedure_pseudo_code | `ev_21fbfd6bbe3870306825dbc83490032d16ecdd14f90738b427ece2604c257aa2` |
| 本轮 | `vm_loadObject` | procedure_pseudo_code | `ev_228dbc19a970bf00daf604f9e1f1171e1f9410d10ca94a49d063f0687b923800` |
| 本轮 | `vm_shutdownObject` | procedure_pseudo_code | `ev_25d55e24d66d458ebaf2ceb6aac63cab835b100c5130c82b5138890178888577` |
| 本轮 | `vm_findProgramFunction` | procedure_assembly | `ev_39ff14e286b302152ec3587aaff3135b507be423df85fb3b0c7651a2f982da7a` |
| 本轮 | `vm_endForBackend` | procedure_callers | `ev_43a949723da25ef7b139ceed352eda43fc22238fe5c47fabbd0847f52e078ee1` |
| 本轮 | `vm_f_destruct_object` | procedure_assembly | `ev_460c4ca1879ca5bbedea229725c71d14436b1e2f6d42e4c90983a64d2134a069` |
| 本轮 | `vm_f_construct_object_ok1` | procedure_pseudo_code | `ev_4a7a31ef99a96b1d6fff61a35b87e55b84ae44a3e25bff0a2947f98e13f3a4e0` |
| 本轮 | `0x80aca84` | procedure_pseudo_code | `ev_4a9a0054a1392bc09a3c1dc2129e02c465dc75325ec76a95e30dbebb03a82f67` |
| 本轮 | `vm_destructObject` | procedure_assembly | `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` |
| 本轮 | `vm_hookCallbackWhenReturn` | procedure_pseudo_code | `ev_4ff4d26fb7db2a562d3076f6865059fbb7f0d9887bb546f7044f18aaafdda955` |
| 本轮 | `Vm_Object_T` | inspect_native_data_type | `ev_52399907a9b2295d01e12ab52c5c933e0df9a5ce409ef7e5826191817dbe6ec4` |
| 本轮 | `vm_callInInitFun` | procedure_callers | `ev_532b40a2912aa0075bd6feae89ca27ecba4c13cb04f28a6806eabe11d8501948` |
| 本轮 | `vm_destructAllObjects` | procedure_pseudo_code | `ev_542620fbbbb8fabe07f6b3f0e1f5f73d33a239f5aa991ca0d0a8103028909e8d` |
| 本轮 | `vm_freeObjectsInDestructingList` | procedure_pseudo_code | `ev_58e41fc7fd79b8820fddd5d3bb13ce115b1663a228e53fd381f6b10c45480218` |
| 本轮 | `vm_allocateObject` | procedure_assembly | `ev_6b2366257b77aa664004a71adc8eb5f4406467f408f512967e4de1249411e062` |
| 本轮 | `vm_lookupObject` | procedure_pseudo_code | `ev_6bd6f758aac4e15cc09716b4e96079841eab2a3ccac331ccd49911785170d50a` |
| 本轮 | `vm_.*(Init|Init.*|[Cc]onstruct|[Ff]inish|[Ee]nd).*Fun|vm_.*[Ii]nit.*[Cc]ode|vm_.*[Ii]nit.*[Vv]ar` | search_procedures | `ev_7537affeb78c4f870f9427cfc554d438e5940600f5ac18d063143d72568aba2d` |
| 本轮 | `0x80ae0e8` | procedure_pseudo_code | `ev_86ea360aa0ccc20146300d4fada064d9d97bd8cc712b8e5d54de240c1cc4b0fd` |
| 本轮 | `vm_unrefObject` | procedure_pseudo_code | `ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e` |
| 本轮 | `vm_f_load_object` | procedure_pseudo_code | `ev_87a2b9092c28e539d262db132e5ad62f159b243caf996e464ef224049039a52e` |
| 本轮 | `vm_f_construct_object_ok2` | procedure_pseudo_code | `ev_8de0ae51a3c83e41247c459b8b8838c17d0a9909cfed731e519d88dd05693e02` |
| 本轮 | `vm_f_destruct_object_crash` | procedure_pseudo_code | `ev_8f13fecfa2ec8a0f4f108c3fed82f6a97666cedb45255c04a339049bafd5e83e` |
| 本轮 | `vm_f_clone_object` | procedure_assembly | `ev_95ee1759e60a50e3626d6f77afede06cdcaf36c76b1376d2b2b731116b19213f` |
| 本轮 | `vm_f_load_object` | procedure_assembly | `ev_99224dc29515f385d867b034821dd999cbb2b622d551f0e8901cae93e6a4dc43` |
| 本轮 | `vm_endForBackend` | procedure_assembly | `ev_99ff4df3dda5ea7103a8c70bf55edc9923932a45ea27dd2afbb7aee201f001f7` |
| 本轮 | `vm_f_destruct_object` | procedure_pseudo_code | `ev_9a05fb2e7216b29de976bc84c35804409c4b0391f4af9fdcc03e6e0ec6f9bfdf` |
| 本轮 | `vm_makeObjectAlone` | procedure_pseudo_code | `ev_a1391c5cfc29264f27cbda3809a9bf634f736453297c310dc23f979b12218fef` |
| 本轮 | `vm_unExecRefObject` | procedure_pseudo_code | `ev_a208cbdd90503fb624273316e1f15e2eee556bf230a0dee842e864b4ecb72437` |
| 本轮 | `vm_f_destruct_func_ok` | procedure_pseudo_code | `ev_a4adb62dd62f7d6e50e24cd56b92f492b3bd3753a66ab3e889e8b59de3ce8db0` |
| 本轮 | `vm_f_destruct_object2` | procedure_assembly | `ev_b1334a21e40fa77529b14cafafd4ce7d1669cdf41e7d2ff014d3bf24431307f3` |
| 本轮 | `vm_f_construct_object_failed` | procedure_pseudo_code | `ev_b1358e0f52d40e07afc5936ce0f39b582ec457b229e0df19581b14ab1da71fc2` |
| 本轮 | `vm_f_construct_object_failed` | procedure_assembly | `ev_b4cb5e180fe31ab5dc405e16e7a2ebe12a00f0e4e50fc8b530dbceea37e79753` |
| 本轮 | `vm_returnFromFakeFun` | procedure_pseudo_code | `ev_b9baf5880477fd6709044a22da45b676e2cf123b192086b6c047577017381b6e` |
| 本轮 | `vm_findProgramFunction` | procedure_pseudo_code | `ev_ba5b1993ec6d3677bd709e0fb78467636839e28ccd2802f1a214278e6a287928` |
| 本轮 | `vm_unExecRefObject` | procedure_assembly | `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` |
| 本轮 | `vm_callInInitFun` | procedure_pseudo_code | `ev_cdee74db197772958f9b9047071e28d032beccf6d11b3ef31d290780aef303e8` |
| 本轮 | `vm_allocateValues` | procedure_assembly | `ev_d5800f267643d7ee5dde2878266b2d7345b65e5040bf3a244897aa271ce5c690` |
| 本轮 | `vm_f_construct_object2` | procedure_assembly | `ev_da96a5f71d49dbc7b88d7cf809835b39c4a15c293342ba20674155e53dfa3606` |
| 本轮 | `vm_findProgramFunctionByCStr` | procedure_pseudo_code | `ev_e0725c9f865c565504a4a69ce6866c428ca0be60f899fe4a27de58ea497a0c7d` |
| 本轮 | `vm_f_clone_object` | procedure_pseudo_code | `ev_e421a1294b8ef5f2e44edaf23ffeed408af95fad72a2e33232ce13d020c35748` |
| 本轮 | `vm_callInInitFun` | procedure_assembly | `ev_e87fd215c338b444b786777608553c57b73d83dc1d3675ab73c10ec003991e6f` |
| 本轮 | `0x8096f44` | procedure_pseudo_code | `ev_ed6d15234b6b77e70190cfcdc433e895468986122447c7f45f1bd8c78c562ba6` |
| 本轮 | `vm_f_destruct_object2` | procedure_pseudo_code | `ev_f17d1950e66d2c98c086472f49d99c9eea4f55ff75edef0ef0e659d5df2c716e` |
| 本轮 | `vm_f_destruct_func_failed` | procedure_pseudo_code | `ev_f4fc3a30d4d10d12d35be2daec518e2e8e8e1c601750a418d198e9540ce61de5` |
| 本轮 | `vm_freeObjectsInDestructingList` | procedure_callers | `ev_f5e402fe68092fc8f3da7f19e3e8f75ed5e9d36f205c394d10e109d9618d37b5` |
| 本轮 | `vm_f_construct_object2` | procedure_pseudo_code | `ev_f8180e2eaf29584441f8a0ae1e5d5e7d0b3e94dbf7d2f01a2000b72058f7324c` |
| 本轮 | `vm_.*(Object|[Ii]nit|[Cc]onstruct|[Dd]estruct|[Cc]lone)` | search_procedures | `ev_faa6ea2601959ba9cca3e6727bac9e11e79c1257e9ffd739e03daa0863babb1d` |
| 本轮 | `vm_cloneObject` | procedure_pseudo_code | `ev_fc3aace8758804f9f7710a10967b65590ef8e9fb8b17bd2b2a7f32d813fc0d04` |
| 本轮 | `vm_allocateValues` | procedure_pseudo_code | `ev_ff4d127c9992f9d1237d84027810ffe2d4684aa1253a8ec9900bb7d899010ce3` |
| 复用 | `vm_gatherProgramFunctionTable` | procedure_assembly | `ev_2b87bd494b66569995a2fd20b9c75f34d7fbc63d63d1b161f3aa5e6ed6113c16` |
| 复用 | `vm_createProgram` | procedure_assembly | `ev_7c339189f2227b0560db98446dc83bd0520ad53bb7da51083b39a547badba203` |
| 复用 | `vm_createProgram` | procedure_pseudo_code | `ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d` |
| 复用 | `vm_gatherProgramFunctionTable` | procedure_pseudo_code | `ev_cbeb0564e51d40f356850c8ce433d229a0063dc15122c784bcac9998341ff27d` |
| 复用 | `vm_callInUserFunBody` | procedure_assembly | `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41` |
| 复用 | `vm_callInUserFunBody` | procedure_pseudo_code | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` |
| 复用 | `vm_returnFromFun` | procedure_pseudo_code | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
| 复用 | `vm_returnFromFun` | procedure_assembly | `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` |
| 复用 | `vm_destructObject` | procedure_pseudo_code | `ev_196547cb37731a3632d63fbc4a62d83a57ac5f788fa57a3ed95657b8a8346c69` |
| 复用 | `vm_destructObjectValues` | procedure_assembly | `ev_d50f92fab09f479b0532c35c9c57bb161f0484bdebcf5d1545ecb10b7ace15be` |
| 复用 | `vm_destructObjectValues` | procedure_pseudo_code | `ev_d8173d61d8d105723534cddea8b9f99fadfed21f978546de6160508e896cd5ec` |
| 复用 | `vm_copyValue` | procedure_pseudo_code | `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33` |

`reused.json.gz` 保留原始记录及来源文件哈希。`Vm_Object_T` 短类型名查询未定位精确数据库类型，不能据此声称没有调试类型；字段偏移以实际指令及既有声明交叉核对。
没有导入旧认证驱动证据。调用者工具只返回已解析的直接引用，间接调用不保证完整。
