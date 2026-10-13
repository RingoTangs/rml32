# REA Evidence 索引

完整规范记录保存于 [magic_Linux32-rea.json.gz](magic_Linux32-rea.json.gz)，解压后是 REA 原始 JSON；本索引没有改写 Evidence ID。`functions.md` 是可读投影，不用于替代原始记录。

| 操作 | 参数/对象 | Evidence ID |
|---|---|---|
| analyze_function | `{"procedure": "vm_transformXActions"}` | `ev_01e42a9cfc1c5c689799a600e23a761d1e778bcf5d43b8f80193ec081dee24a7` |
| inspect_native_data_type | `{"address": "0x83f8e00"}` | `ev_0549e18b43db3f7379881d998e2a8a56c78fc1dd5d7d8215518839d7ac20bd9e` |
| analyze_function | `{"procedure": "vm_startNewFile"}` | `ev_091710844f88d446c6ba795cdf68bea0055af14fbb18d02732b7b87a622a8ffa` |
| analyze_function | `{"procedure": "vm_buildFullPacket"}` | `ev_0c5d60d8dc7bc8cd87d1216414bed31d40677570d3d70331b9df070634fe86ba` |
| inspect_native_data_type | `{"type": "/DWARF/vm.h/Vm_Node_T"}` | `ev_108eaf94a0a4d7729a6759b88c6f3064c697b6609cda460dfb6f6535648a51d2` |
| search_procedures | `{"case_sensitive": false, "mode": "regex", "pattern": "processor&#124;backend&#124;jit&#124;action"}` | `ev_10f94eb487d1ad458b6b77efe3ed64f737de9c77fad3289b4876f7f0c9fad8fe` |
| analyze_function | `{"procedure": "main"}` | `ev_11596ff166420c38186bc7d6abbc672d88facffdcf8c8b692d814f7ccb0dd66e` |
| analyze_function | `{"procedure": "vm_f_enable_jit"}` | `ev_128424766bb0641e127ec93dcb56141b8bfd49b60e27d5e6b673e9dd664e6143` |
| resolve_containing_procedure | `{"address": "0x80b07c8"}` | `ev_1798d46841dd17000c09e484dd61c19e97fcc4d5928d3c0bf35382dea2a015d3` |
| analyze_function | `{"procedure": "vm_initScheduler"}` | `ev_1965a638cea2a5e041603b5b16f08db86671a431ef77391143b8f932008c979f` |
| analyze_function | `{"procedure": "vm_callInUserFun"}` | `ev_1c856bf5b985ed94a90def645280c43cba5e562297da9520994866c02af54409` |
| analyze_function | `{"procedure": "vm_jitEnableExecute"}` | `ev_1d9fb6c7ce28d17dfb4728537635212cc7ba3b658da7a3728e7cce0e3f2abb44` |
| analyze_function | `{"procedure": "vm_calculateCheckSum"}` | `ev_1e018f4bdaa160e8435dde86590cf2e1807bfe840410d42d2bce299bc44942f7` |
| inspect_native_data_type | `{"address": "0x83c8e60"}` | `ev_1ea02517a9f365065aaa0449b8c1034f2ae24634f698dedffbcdbde79ab3df8e` |
| read_bytes | `{"address": "0x83cc680", "length": 2436}` | `ev_1f4fb022c767a8486706fc5897b0b3d524e3ea66d5e31372afbf4896c83da6c7` |
| analyze_function | `{"procedure": "vm_initActions"}` | `ev_20fd3f1645133c68d93a3c19562ca968972df72d94cd2f21c4badce99138dedd` |
| analyze_function | `{"procedure": "vm_waitMainLoopDone"}` | `ev_218395b72ac924cabe14085ef72ce27a156221b92b9de2e1d95e1152b16cb37e` |
| analyze_function | `{"procedure": "vm_magicInit"}` | `ev_26ed1e9a3959222c06e623972588b80b2c997b29ad56919294787b5ba10ee87d` |
| analyze_function | `{"procedure": "vm_cmdLogin"}` | `ev_34f4228d36d153663747e66698896ee4e6d2a3aebde094aeb34ac4d3192c563b` |
| inspect_native_data_type | `{"type": "/DWARF/vm_efun_magic.h/Vm_Command_List_T"}` | `ev_3c69c9cef6c50e09ccccbaa402f3995a97443a8898263d4753befae400c33913` |
| analyze_function | `{"procedure": "vm_initEfun"}` | `ev_3cf4b7b4c241ad44b0d360927454293628555da474b2ebb560fa890a525285fb` |
| analyze_function | `{"procedure": "vm_compileToJit"}` | `ev_3f2a3139134d9148e77ca7c4781e487c308e7030761018699c2af97d1764a601` |
| analyze_function | `{"procedure": "vm_apply"}` | `ev_3f2d75d49aca31266911219f7b61c071d1cfa73e8847ee0504e14bcd9ea1c8ec` |
| analyze_function | `{"procedure": "vm_buildCmdIndex"}` | `ev_3f6a3b1b4aa295672b016c344c130e9243cb5bc250af2e2551bc14b507436b82` |
| analyze_function | `{"procedure": "lzss_decompress"}` | `ev_409ac996dc71b0cce45c939511e6c2e96d909adfc3ced31b97c0c177aab1b4e8` |
| analyze_function | `{"procedure": "_start"}` | `ev_420ea9ce3c8bb0b3816fe4e0d170afab61b08bf4c8ecd153b91b17f1b0836875` |
| analyze_function | `{"procedure": "vm_addValue"}` | `ev_42e127d62e334cc26bafbc7565df879a834dbffba8a6a59f495f385632e44e7d` |
| analyze_function | `{"procedure": "vm_builtInFileOpen"}` | `ev_4626d9fcf5ed44d83dd274665f0140102206b60ac180effeda739c330d71e925` |
| read_bytes | `{"address": "0x83f8e00", "length": 32}` | `ev_47d8ce3d79a1f188942d4267ee49e7ac48306f5dce3e7199c8e31e87951e900c` |
| xrefs | `{"address": "0x80adea8"}` | `ev_4a58fbe06dbae6e54d99df67ed1d21e7f97c862938f1bbcc481a250ce37b828b` |
| analyze_function | `{"procedure": "vm_loadScripts"}` | `ev_528d2e6cfdadd12e4f9fa1735c508e40354938350808cff803095e3cc70cbcf3` |
| analyze_function | `{"procedure": "vm_gameListenSocketCallback"}` | `ev_52ea6440ff8b146aee383c74cc729c765333951576ac45079d073fcf9303b74b` |
| inspect_native_data_type | `{"address": "0x83ce680"}` | `ev_5439e4f9aa2b8fdf1b2cb0f8aa652543c7868ffab70b75d610859552b0a802ec` |
| analyze_function | `{"procedure": "vm_doActions"}` | `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` |
| read_bytes | `{"address": "0x83ce780", "length": 32}` | `ev_59956e812dd88fcb458e70f2eac9d610bddce101c90bed67125e48012449ba23` |
| analyze_function | `{"procedure": "vm_driverLoop"}` | `ev_5b3319fdecc52514a9862df16351b78948bfaaa32a2e6a633ef4b761bd781fcf` |
| analyze_function | `{"procedure": "vm_bindToClient"}` | `ev_5b438686342c09b8ce0af8c4da9a3ad26fc4101f6f3d2dd5d27f2c78a5ceb81d` |
| analyze_function | `{"procedure": "vm_myConfigVm"}` | `ev_6044830af03664a057a06d80c146f7b0b160dfc5b9850b00afc43c89ed0aec7f` |
| inspect_analysis_view | `{"parent_evidence_id": "ev_bc11df9a2ac86731295fffd953bca4a9f1f963f088052c649f93071dfba7035a", "view": {"facet": "pseudocode", "kind": "native", "limit": 2500, "offset": 0}}` | `ev_60f6327eb8f8a7ebb3a6bd96a2b14a0479b984218f9f51b394e08e31d60e7b93` |
| analyze_function | `{"procedure": "vm_f_start_service_on"}` | `ev_61152e85bc03cbdcdc23cdfedf7f74f3cec2a4dfea948fe6280eb42fd4500e7b` |
| read_bytes | `{"address": "0x83cf960", "length": 400}` | `ev_61d5578e03490de5c4a0971f203403bf257f5db5f842cbec7674118bd77f89f6` |
| read_bytes | `{"address": "0x83ce680", "length": 4288}` | `ev_69a9dd6eb18e7378d3b4a872c83717953783c41b236970d55aba889bb19d2a2e` |
| inspect_native_data_type | `{"address": "0x83cc680"}` | `ev_6a549de8b50c8ec249ee158565612cd1810076f142669517e6ad2badfb9909b7` |
| read_bytes | `{"address": "0x83c8e60", "length": 7392}` | `ev_6dd68d30d2303e6ddda9285d49a1953130a8d455aa8a2e42e4c9e8a1ad3cf31f` |
| analyze_function | `{"procedure": "vm_checkAuthServer"}` | `ev_6e19c669423b79d90370b21d450bb2eb1edd584a803a8fe98089e6c41c9c7c4e` |
| read_bytes | `{"address": "0x83cd9a0", "length": 128}` | `ev_6e81d2949f9f6d95c73ae2fe4ac7ae5666c5e3acf688ce050af4edfde3898b64` |
| analyze_function | `{"procedure": "vm_clientSocketCallback"}` | `ev_7412c83a8efbac41289da3a1085cc73fc13e8f921ee29850141f2839e505d74a` |
| analyze_function | `{"procedure": "vm_registerConfigRoutine"}` | `ev_7468c46d98f6320950d7aaf6dc8be22d60dfc5a5ecc1cf6f73fe0730949aba37` |
| analyze_function | `{"procedure": "vm_registerBuiltInFiles"}` | `ev_7ad97a88c34ee06e3a23d8851be42f7b94e11919b1baef4efd937fa1f420e58b` |
| inspect_native_data_type | `{"type": "/DWARF/vm_action_table.h/Vm_Action_Info"}` | `ev_80bd2a222eacf3fd76d48c41289cbf76776fe30e735b941aae49d95a6ea7a6bb` |
| analyze_function | `{"procedure": "vm_driverStart"}` | `ev_82e207ff020838b3fa66aa7d1ba59cdb0ae38a29f625e11da964ed830b5c5b38` |
| analyze_function | `{"procedure": "vm_scheduleProcessInternal"}` | `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` |
| search_procedures | `{"case_sensitive": false, "mode": "regex", "pattern": "(^&#124;::)(main&#124;entry&#124;_start)$"}` | `ev_8cada05ba670c5f30f22fac28279a4e9b35046527f40bf7b0db07e38534464bb` |
| analyze_function | `{"procedure": "vm_refillBuffer"}` | `ev_8d9720e79dc0e51faaa6cec7ea157a066a2907aaf8c0e190cd7c046ef8507fa6` |
| analyze_function | `{"procedure": "vm_openLocalFile"}` | `ev_957152e32fa01c91e385849c187394a63e641da15285a57497c9bb63df45a081` |
| analyze_function | `{"procedure": "vm_compile"}` | `ev_95cf781aa8927dfa99280616404ea6a4666f2f030ab5a3b81feb94263470be0e` |
| analyze_function | `{"procedure": "vm_open"}` | `ev_97d9eabd7b55d38743614539d0adbb897e297fd552bad28aa3e359a3f86962c0` |
| inspect_analysis_view | `{"parent_evidence_id": "ev_bc11df9a2ac86731295fffd953bca4a9f1f963f088052c649f93071dfba7035a", "view": {"facet": "callees", "kind": "native", "limit": 30, "offset": 0}}` | `ev_9893fef8c632aac678adee5693763da140bb35202f1694c43a8a5b5590a551c8` |
| analyze_function | `{"procedure": "vm_invokeScriptFunction"}` | `ev_9d3e94067298d0475f09cf4360ba81d004261189ca8d8ae3188efd3e6f63c520` |
| analyze_function | `{"procedure": "vm_jitProcessor"}` | `ev_9e5ee0886a66f1f7fc85c78fd5fa036acb947650e8c13cb8f0f1ee8b4da907f1` |
| analyze_function | `{"procedure": "vm_checkLocalOptions"}` | `ev_a3028a6a266eab59f2e3933412667d007433fad7d0a55e782a5662052b98897d` |
| read_bytes | `{"address": "0x83cd9a0", "length": 3295}` | `ev_a4ab1a1de4a7f42fdb89dda416182235429f1c3f27b9844bd0b43895f6cf1b32` |
| analyze_function | `{"procedure": "vm_setProcessor"}` | `ev_a75ce1b0f3c99f6d4bf69abf9c54bd8404b01c683a6b774704c35f781535fc00` |
| analyze_function | `{"procedure": "vm_if"}` | `ev_b5f6eaa2b5306afbad85a9b3a712f7b17cbfa2bbe30ed8e08f200416715847a5` |
| inspect_native_data_type | `{"type": "/DWARF/vm_efun_magic.h/Vm_Packet_Header_T"}` | `ev_bc05dd0bbcf6e3391a72dbc3269d5fba2893b6dadbffe970bdf4a8a88bc3448b` |
| analyze_function | `{"procedure": "vm_yyparse"}` | `ev_bc11df9a2ac86731295fffd953bca4a9f1f963f088052c649f93071dfba7035a` |
| binary_overview | `{}` | `ev_bc218b7fb1a81fb7ab78196d70d31761a2bc4bcda25a4827820f84ab5eb823e8` |
| analyze_function | `{"procedure": "vm_regularProcessor"}` | `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` |
| analyze_function | `{"procedure": "vm_registerSubModules"}` | `ev_c172a0793b9836762cc1f66a6f0deaa3c6c4830bd013f0908f8700ab65a4fecc` |
| analyze_function | `{"procedure": "vm_run"}` | `ev_cafdf06efbda8dcdf88dd7042146a5c88423478c9317d25430f093649c51c9bf` |
| search_strings | `{"case_sensitive": false, "mode": "regex", "pattern": "usage&#124;magic&#124;version&#124;copyright"}` | `ev_cafeb40681083a7afbee3faff9c89070425f31846723c26d634c3a81fbb56905` |
| inspect_analysis_view | `{"parent_evidence_id": "ev_bc11df9a2ac86731295fffd953bca4a9f1f963f088052c649f93071dfba7035a", "view": {"facet": "procedure", "kind": "native", "limit": 30, "offset": 0}}` | `ev_cdbd2fd8813e5bdd7fb167ea6e21195551a586e21a1bc12c801a1bdb134af7aa` |
| analyze_function | `{"procedure": "vm_loadBinary"}` | `ev_d36be5e76af4173eaa042416c1354f4ac89d5b3a1f9e8e0ac7a3de77088f54d0` |
| inspect_native_data_type | `{"type": "/DWARF/vm_efun_magic.h/Vm_Packet_Header"}` | `ev_da5666013fe5b125666868d68d4621d05986d7d872b07c251dbc450275689dec` |
| analyze_function | `{"procedure": "vm_mainLoop"}` | `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` |
| list_names | `{}` | `ev_dcc8b9acde6c81246b23aaa5c12b0f8f8fb485c83555daaf174f1e452d29c31c` |
| inspect_native_data_type | `{"type": "/DWARF/vm_action_table.h/Vm_Action_Info_T"}` | `ev_df1959075d387231d15fb2e2a108e25ad1befa44512a6e967002ce2b2a7b8554` |
| xrefs | `{"address": "0x82a43f1"}` | `ev_e150b27174084a178ddb0ffd436fb9b7ebce73cc6ae8c056a467eb056e552da9` |
| analyze_function | `{"procedure": "vm_compileToExe"}` | `ev_e526f94d234771a46b207ad3f9e421d5707ac9f283b7e64f1bfe73160f0cdcb7` |
| xrefs | `{"address": "0x83cd9a0"}` | `ev_e858017cb0e915ce42026c0b6269ea808a4bfd30642c5bf20d5a2696a3348a4d` |
| analyze_function | `{"procedure": "vm_analyseArguments"}` | `ev_ec40905f4fd0261e287441a64fb12ad090a1b12bc555ea2b02f5d5e22b04eaf2` |
| analyze_function | `{"procedure": "vm_init"}` | `ev_f7cce8a4c08a768f41121ec20e46d50fefa77e3d0a95e9d5a7f4b69b22a10cbf` |
| search_strings | `{"case_sensitive": false, "mode": "regex", "pattern": "auth_1980&#124;cmd_login&#124;Gameshell&#124;start_service_on&#124;\\.pak&#124;--to-compile"}` | `ev_fa85983e91b9c05163ca1f0a6b3b31b0f4895e14a3a93f6bdc574911a8b82b96` |
| analyze_function | `{"procedure": "vm_icallScriptFunction"}` | `ev_fcfd4fa5d63c27ba2af511747d7b1766cbad817b126fff97aa26e5025e051498` |
| analyze_function | `{"procedure": "vm_configPdb"}` | `ev_fcfdd58365007d9fac73dea7876384dcfe43e7aa641f4888746395b0037cc1d1` |
| analyze_function | `{"procedure": "vm_generateLocalFileName"}` | `ev_fd70a136d5ad27d9462528647f7512c2e13250eaf83341048976f996ea12cb51` |
| xrefs | `{"address": "0x86040a0"}` | `ev_fdd6f92d102930fecb80957ab5e8529a206092bf4637842952a3afe6d93942fc` |

共 87 条记录，55 个函数 dossier。原始 JSON SHA-256：`705f1b2da331a3ed2b917ea87346f8b36613763d6589c56378ec0eb94b13ebae`。
