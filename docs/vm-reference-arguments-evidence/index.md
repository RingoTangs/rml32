# Ghidra 静态证据索引

目标：`magic_Linux32`；引擎：Ghidra 12.1.4；完整记录 45 条。

目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

所有反编译均为静态解释，变量名和类型可能存在恢复误差。原始证据包含函数体范围、限制及完整交叉引用。

[完整原始记录](rea.json.gz) · [可读反编译与汇编](functions.md) · [统计方法](method.md)

| 函数 | 操作 | Evidence ID |
|---|---|---|
| `vm_callInUserFun` | `procedure_assembly` | `ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347` |
| `vm_callInUserFun` | `procedure_pseudo_code` | `ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97` |
| `vm_callInUserFun` | `procedure_references` | `ev_ec24937983df149b2a6071baf778392b3e10f4cfbac994804282c6be6d30341e` |
| `vm_callInUserFunBody` | `procedure_assembly` | `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41` |
| `vm_callInUserFunBody` | `procedure_pseudo_code` | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` |
| `vm_callInUserFunBody` | `procedure_references` | `ev_a41035d7f8f629d8a34298016d9624295c4c86e2f5c476e3c459c6f9582b9959` |
| `vm_caughtError` | `procedure_pseudo_code` | `ev_b8cf22d050dedb8fc06d53d128ce869b74d9bfc40305dca347b34e750ee9ad50` |
| `vm_checkArgNum` | `procedure_assembly` | `ev_d17aa0d7a395baf8b760f4dd773008e55d6a7fc1e8c6931a9657f73663982946` |
| `vm_checkArgNum` | `procedure_pseudo_code` | `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9` |
| `vm_checkArgNum` | `procedure_references` | `ev_0ed414de826af49dbadb7f26fc53cdc07c2d0227f0342b5dd8417ce6d020b662` |
| `vm_convertToValue` | `procedure_pseudo_code` | `ev_8a351c14040cefb54204c1d1879ec273798f7579844be1503f87df0f5a5783dc` |
| `vm_convertValueToConst` | `procedure_assembly` | `ev_27174938ea17a652d5f14712c667798de8416b58f3e9178d0f204341437021ec` |
| `vm_convertValueToConst` | `procedure_pseudo_code` | `ev_b6460e04232fdc6c08106ab85659c62fb0bfe8932b841606f522549d510bbd48` |
| `vm_copyValue` | `procedure_assembly` | `ev_423da2c3a22de7bda0a4607843d7318f489c845a044cb1ae3efe37c0e6862630` |
| `vm_copyValue` | `procedure_pseudo_code` | `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33` |
| `vm_doActions` | `procedure_pseudo_code` | `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` |
| `vm_doActions` | `procedure_references` | `ev_520af79391604c804c2a40d599235d711523bbebfdfa5c927ac8f4d5a96cc44f` |
| `vm_enterFunction` | `procedure_pseudo_code` | `ev_1782e4cf62d7942391d37161d43a2051ad3aa963b13b7ca61519cc59ae7dad43` |
| `vm_error` | `procedure_pseudo_code` | `ev_1b494a92dfe3a6c5943ad427ec0ed25c0e15c7afc6932862b5221a85cb9ef7c7` |
| `vm_errorQuit` | `procedure_assembly` | `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` |
| `vm_errorQuit` | `procedure_pseudo_code` | `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1` |
| `vm_errorQuit` | `procedure_references` | `ev_e54bf8d3fa81ea23438c1549ccb9cfd5f2d2dfbd7c29686f20088d6b68c014d4` |
| `vm_getValueType` | `procedure_assembly` | `ev_c80699a3a1f8f4bf323e4b383924439d4b12f2e7690a1e5715dde49bcf319546` |
| `vm_getValueType` | `procedure_pseudo_code` | `ev_4b11f8b8a188ce3208935d909fe583bb7f81ba977c29dbbede57d5c0dbe4861c` |
| `vm_getValueType` | `procedure_references` | `ev_42be364b90ac0e163a87f9db5c8d57e1dc405ea418da16aa79bcc05ee1a5a4db` |
| `vm_getVarType` | `procedure_assembly` | `ev_538ec0dc0ef72f4f093d9c31c2e74f1fdc4f83d41d2e53d2e73b7d2db14d4d67` |
| `vm_getVarType` | `procedure_pseudo_code` | `ev_f613eb5dabf38b4c00ad89d950aa023ce066cbfe213a48b832691e6655a8a6ac` |
| `vm_pushLocalVar` | `procedure_assembly` | `ev_d0c3b683bea4f35107c1c4be31af558ea2635c43a303b9009673e8de1ca083fc` |
| `vm_pushLocalVar` | `procedure_pseudo_code` | `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5` |
| `vm_pushLocalVar` | `procedure_references` | `ev_408a9e6b5f4acc2cd9c0d58b366519c7bebf0a8e9c196c560f282bbd3a6adc9e` |
| `vm_pushLocalVptr` | `procedure_assembly` | `ev_9b4e2d24822946ff7ae5449f7d30b3e3b2cc1134ef9c530294dd9a840f45feed` |
| `vm_pushLocalVptr` | `procedure_pseudo_code` | `ev_cdf3f69a6cd4c53275ebfcb955c6419230978dcbe6c1af8e4d0c1dc6d6c32de2` |
| `vm_pushLocalVptr` | `procedure_references` | `ev_41adff86548bebe965803ed1613b8429cb029ce9fc72d17da1166bfa9aecf13a` |
| `vm_returnFromFun` | `procedure_assembly` | `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` |
| `vm_returnFromFun` | `procedure_pseudo_code` | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
| `vm_returnFromFun` | `procedure_references` | `ev_afb24a32fef84ca16adde43d0d60a0fddd1bdb98ecf3b02f908f38827dfa4d3f` |
| `vm_returnToBackend` | `procedure_pseudo_code` | `ev_15229c3a25b5ee5f8e5403626edbcb2859e31a9d6353095683206d71fd94338c` |
| `vm_setLocalVar` | `procedure_assembly` | `ev_b4762566f69e6ef759a4c846394fb10d47cd1eebf8da295f5f40105ef3884689` |
| `vm_setLocalVar` | `procedure_pseudo_code` | `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc` |
| `vm_setLocalVar` | `procedure_references` | `ev_fbd42bebd7cb8d30f00cd2817fed56d20d93b522c4697ff503576460e1772a63` |
| `vm_takeOffValueFromStack` | `procedure_assembly` | `ev_c2177a4cc3c48912f1626bc399600d302a4c708f69bf066dade3508055bf7119` |
| `vm_takeOffValueFromStack` | `procedure_pseudo_code` | `ev_99c34e4e373606ceaa48084b7cbe89c70742d22173dfb81bb36c8bddcd94b778` |
| `vm_traceError` | `procedure_assembly` | `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` |
| `vm_traceError` | `procedure_pseudo_code` | `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4` |
| `vm_traceError` | `procedure_references` | `ev_757d5e071776339db4cabc56f20d2332b62632fe1fa366c3f0414c9f87edad47` |
