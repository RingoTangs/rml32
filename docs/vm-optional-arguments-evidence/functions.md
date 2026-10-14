# 阶段 2I Ghidra 证据

反编译是 Ghidra 恢复结果；C 类型、局部变量和边界可能存在恢复误差。完整引用、函数体范围及工具限制以压缩原始记录为准。

## 新增：0x08667a68 — inspect_native_data_type

Evidence ID：`ev_06d68ee2779b09e24c9c4147a265a3ae799765f547578bf03c68129644793740`

```json
{
  "address": "0x8667a68",
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_fun.h/Vm_Fun_T *",
  "kind": "pointer",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Fun_T *",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_fun.h/Vm_Fun_T",
  "size_bytes": 4,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 新增：(Arg|arg|Default|default|Fun) — search_procedures

Evidence ID：`ev_080023dcd5f894c9e792538afcc05219bbf66429f72d148928006964dd02d179`

```json
[
  {
    "address": "0x804ae14",
    "value": "FUN_0804ae14"
  },
  {
    "address": "0x804bedc",
    "value": "CHSSetFunc"
  },
  {
    "address": "0x804c960",
    "value": "vm_initEfun"
  },
  {
    "address": "0x804c9d8",
    "value": "vm_shutdownEfun"
  },
  {
    "address": "0x805e880",
    "value": "vm_msgDoRecharge"
  },
  {
    "address": "0x805e9e4",
    "value": "vm_cmdRequestRecharge"
  },
  {
    "address": "0x805ea24",
    "value": "vm_cmdRecharge"
  },
  {
    "address": "0x80618b4",
    "value": "vm_msgLChargeResult"
  },
  {
    "address": "0x8062f90",
    "value": "vm_msgLDoAlipayRecharge"
  },
  {
    "address": "0x8063b58",
    "value": "vm_msgUpdateRechargeReturnCredit"
  },
  {
    "address": "0x806be60",
    "value": "vm_cmdChargeVendueOfferPrice"
  },
  {
    "address": "0x806bf0c",
    "value": "vm_cmdChargeVendueRefresh"
  },
  {
    "address": "0x806bf4c",
    "value": "vm_cmdChargeVendueFetch"
  },
  {
    "address": "0x806bfd0",
    "value": "vm_cmdChargeVendueStoreClosed"
  },
  {
    "address": "0x8076f60",
    "value": "vm_cmdCitywarGiveOrders"
  },
  {
    "address": "0x807ea8c",
    "value": "vm_getDefaultAttrib"
  },
  {
    "address": "0x807eaa0",
    "value": "vm_setDefaultAttrib"
  },
  {
    "address": "0x807eee8",
    "value": "vm_getFunName"
  },
  {
    "address": "0x808071c",
    "value": "vm_getNewFunOffset"
  },
  {
    "address": "0x8081e28",
    "value": "vm_getLocalDefaultObject"
  },
  {
    "address": "0x8081e34",
    "value": "vm_getGlobalDefaultObject"
  },
  {
    "address": "0x8085ce4",
    "value": "vm_addEfuns"
  },
  {
    "address": "0x8085e68",
    "value": "vm_tryReferToFunction"
  },
  {
    "address": "0x808600c",
    "value": "vm_updateEfuns"
  },
  {
    "address": "0x8086b44",
    "value": "vm_findProgramFunction"
  },
  {
    "address": "0x8086d68",
    "value": "vm_findProgramFunctionByCStr"
  },
  {
    "address": "0x8086df0",
    "value": "vm_getFunctionInDefaultObject"
  },
  {
    "address": "0x80874d4",
    "value": "vm_registerEfunModule"
  },
  {
    "address": "0x8087528",
    "value": "vm_pickupFunAndfreeAll"
  },
  {
    "address": "0x808761c",
    "value": "vm_buildEfunOffset"
  },
  {
    "address": "0x8089630",
    "value": "vm_scratchLargeAlloc"
  },
  {
    "address": "0x80896c8",
    "value": "vm_scratchLargeCopy"
  },
  {
    "address": "0x808a364",
    "value": "vm_getArgument"
  },
  {
    "address": "0x808a5ec",
    "value": "vm_getXIntArgument"
  },
  {
    "address": "0x808a770",
    "value": "vm_getRealArgument"
  },
  {
    "address": "0x808a8ec",
    "value": "vm_getStringArgument"
  },
  {
    "address": "0x808c114",
    "value": "vm_freeFunctionValue"
  },
  {
    "address": "0x808c1fc",
    "value": "vm_unrefFunctionValue"
  },
  {
    "address": "0x808cbf4",
    "value": "vm_setCurrentArgument"
  },
  {
    "address": "0x808ce3c",
    "value": "vm_safeGetPCBArgument"
  },
  {
    "address": "0x808d0a4",
    "value": "vm_getPCBArgument"
  },
  {
    "address": "0x808d310",
    "value": "vm_getPCBXIntArgument"
  },
  {
    "address": "0x808d474",
    "value": "vm_getPCBIntArgument"
  },
  {
    "address": "0x808d5c0",
    "value": "vm_getPCBRealArgument"
  },
  {
    "address": "0x808d730",
    "value": "vm_getPCBStringArgument"
  },
  {
    "address": "0x808d8dc",
    "value": "vm_registerMarkActiveFunction"
  },
  {
    "address": "0x808d928",
    "value": "vm_unregisterMarkActiveFunction"
  },
  {
    "address": "0x808daec",
    "value": "vm_markActiveFromFunction"
  },
  {
    "address": "0x80912a0",
    "value": "vm_pushFunction"
  },
  {
    "address": "0x80916cc",
    "value": "vm_pushArgNum"
  },
  {
    "address": "0x8091cd0",
    "value": "vm_createFunctionValueEx"
  },
  {
    "address": "0x8091f34",
    "value": "vm_createFunctionValue"
  },
  {
    "address": "0x8091fb4",
    "value": "vm_createArgumentArray"
  },
  {
    "address": "0x8092c2c",
    "value": "vm_getCurrentFunCp"
  },
  {
    "address": "0x8092d18",
    "value": "vm_showFunContext"
  },
  {
    "address": "0x8093400",
    "value": "vm_argumentInThisContext"
  },
  {
    "address": "0x8093420",
    "value": "vm_callOSFun"
  },
  {
    "address": "0x8093c5c",
    "value": "vm_returnFromEfun"
  },
  {
    "address": "0x8093d98",
    "value": "vm_callInEfun"
  },
  {
    "address": "0x8093eb0",
    "value": "vm_callInUserFunBody"
  },
  {
    "address": "0x8094080",
    "value": "vm_callInUserFun"
  },
  {
    "address": "0x8094178",
    "value": "vm_callInFakeFun"
  },
  {
    "address": "0x8094258",
    "value": "vm_callInInitFun"
  },
  {
    "address": "0x8094374",
    "value": "vm_returnFromFakeFun"
  },
  {
    "address": "0x809441c",
    "value": "vm_callFunPtr"
  },
  {
    "address": "0x8094978",
    "value": "vm_returnFromFun"
  },
  {
    "address": "0x8094cd8",
    "value": "vm_callInOtherFun"
  },
  {
    "address": "0x8094ec0",
    "value": "vm_adjustArgumentNum"
  },
  {
    "address": "0x8094ef4",
    "value": "vm_findFunction"
  },
  {
    "address": "0x809521c",
    "value": "vm_checkArgNum"
  },
  {
    "address": "0x80955e8",
    "value": "vm_enterFunction"
  },
  {
    "address": "0x8095654",
    "value": "vm_copyFunction"
  },
  {
    "address": "0x8095794",
    "value": "vm_finalizeFun"
  },
  {
    "address": "0x8095848",
    "value": "vm_saveFun"
  },
  {
    "address": "0x80959b8",
    "value": "vm_loadFun"
  },
  {
    "address": "0x8095c88",
    "value": "vm_freeFun"
  },
  {
    "address": "0x8097584",
    "value": "vm_caseDefault"
  },
  {
    "address": "0x809789c",
    "value": "vm_updateCurrentFunCallContext"
  },
  {
    "address": "0x80983fc",
    "value": "vm_createEntryFunPtrFromName"
  },
  {
    "address": "0x80984e8",
    "value": "vm_createEntryFunPtrFromFun"
  },
  {
    "address": "0x8099678",
    "value": "vm_insertArgsIntoFunction"
  },
  {
    "address": "0x80999b0",
    "value": "vm_invokeScriptFunction"
  },
  {
    "address": "0x8099f58",
    "value": "vm_applyFunInThread"
  },
  {
    "address": "0x809a09c",
    "value": "vm_applyFun"
  },
  {
    "address": "0x80a1c40",
    "value": "vm_defaultReadFromConsole"
  },
  {
    "address": "0x80a1c9c",
    "value": "vm_defaultWriteToConsole"
  },
  {
    "address": "0x80a1cb8",
    "value": "vm_defaultCntlConsole"
  },
  {
    "address": "0x80a1cd4",
    "value": "vm_combineIsGlobalTypeArgn"
  },
  {
    "address": "0x80a1d9c",
    "value": "vm_undefineLocalVarOrArg"
  },
  {
    "address": "0x80a20d0",
    "value": "vm_buildDefaultValueNodes"
  },
  {
    "address": "0x80a21f0",
    "value": "vm_overrideFunction"
  },
  {
    "address": "0x80a28fc",
    "value": "vm_inheritFunctionPrototype"
  },
  {
    "address": "0x80a31bc",
    "value": "vm_gatherProgramFunctionTable"
  },
  {
    "address": "0x80a9100",
    "value": "vm_getProcessFunContextSize"
  },
  {
    "address": "0x80a91a4",
    "value": "vm_setProcessFunContextSize"
  },
  {
    "address": "0x80aba5c",
    "value": "vm_icallScriptFunction"
  },
  {
    "address": "0x80b0a24",
    "value": "vm_lookupOSFun"
  },
  {
    "address": "0x80b622c",
    "value": "vm_registerStrwidthFunc"
  },
  {
    "address": "0x80b9728",
    "value": "vm_parseCmdLineArguments"
  },
  {
    "address": "0x80b9848",
    "value": "vm_freeCmdLineArguments"
  },
  {
    "address": "0x80b9a64",
    "value": "vm_analyseArguments"
  },
  {
    "address": "0x80bcdd8",
    "value": "vm_callInOtherFunExReturn"
  },
  {
    "address": "0x80bd0fc",
    "value": "vm_callInOtherFunEx"
  },
  {
    "address": "0x80c1698",
    "value": "vm_matchPktArguments"
  },
  {
    "address": "0x80c2178",
    "value": "vm_deriveArguments"
  },
  {
    "address": "0x80c2e94",
    "value": "vm_resourceEfunInit"
  },
  {
    "address": "0x80c2ea8",
    "value": "vm_resourceEfunDestruct"
  },
  {
    "address": "0x80c5848",
    "value": "vm_getDebugFunCp"
  },
  {
    "address": "0x80cab20",
    "value": "vm_compileToFunCall"
  },
  {
    "address": "0x80cabc4",
    "value": "vm_compileToEfun"
  },
  {
    "address": "0x80d41fc",
    "value": "vm_efunInit"
  },
  {
    "address": "0x80d4408",
    "value": "vm_efunDestruct"
  },
  {
    "address": "0x80d4414",
    "value": "vm_setArgInfo"
  },
  {
    "address": "0x80d626c",
    "value": "vm_f_find_function"
  },
  {
    "address": "0x80d62d4",
    "value": "vm_f_find_function_arg"
  },
  {
    "address": "0x80d6418",
    "value": "vm_f_get_args"
  },
  {
    "address": "0x80d6570",
    "value": "vm_f_get_function_info"
  },
  {
    "address": "0x80d6724",
    "value": "vm_f_get_function_owner"
  },
  {
    "address": "0x80d7f38",
    "value": "vm_f_is_function"
  },
  {
    "address": "0x80ed2c8",
    "value": "vm_f_get_function_report"
  },
  {
    "address": "0x80edc10",
    "value": "vm_f_previous_arguments"
  },
  {
    "address": "0x80edf94",
    "value": "vm_f_reset_function_mark"
  },
  {
    "address": "0x80f508c",
    "value": "vm_f_previous_function"
  },
  {
    "address": "0x80f5afc",
    "value": "vm_f_call_other_arg"
  },
  {
    "address": "0x80f6194",
    "value": "vm_f_find_object_function"
  },
  {
    "address": "0x80f65ec",
    "value": "vm_f_get_global_function"
  },
  {
    "address": "0x80f6958",
    "value": "vm_f_get_obj_function"
  },
  {
    "address": "0x80f95cc",
    "value": "vm_f_destruct_func_ok"
  },
  {
    "address": "0x80f95e8",
    "value": "vm_f_destruct_func_failed"
  },
  {
    "address": "0x80f960c",
    "value": "vm_efunVarInit"
  },
  {
    "address": "0x80f9620",
    "value": "vm_efunVarDestruct"
  },
  {
    "address": "0x80fa460",
    "value": "vm_statForFun"
  },
  {
    "address": "0x80fb104",
    "value": "vm_efunDebugInit"
  },
  {
    "address": "0x80fb12c",
    "value": "vm_efunDebugDestruct"
  },
  {
    "address": "0x80fb778",
    "value": "vm_efunPerformanceProfileInit"
  },
  {
    "address": "0x80fb78c",
    "value": "vm_efunPerformanceProfileDestruct"
  },
  {
    "address": "0x80fbabc",
    "value": "vm_getTableByArgument"
  },
  {
    "address": "0x80fdb4c",
    "value": "vm_getTreeByArgument"
  },
  {
    "address": "0x80ff9d8",
    "value": "vm_genCallInFun"
  },
  {
    "address": "0x811bfcc",
    "value": "desfunc"
  },
  {
    "address": "0x811ceb4",
    "value": "default_bzalloc"
  },
  {
    "address": "0x811cecc",
    "value": "default_bzfree"
  },
  {
    "address": "0x8128ff0",
    "value": "vm_shutdownSqliteEfun"
  },
  {
    "address": "0x8129000",
    "value": "vm_initSqliteEfun"
  },
  {
    "address": "0x812b9b0",
    "value": "nocaseCollatingFunc"
  },
  {
    "address": "0x812b9f0",
    "value": "binCollFunc"
  },
  {
    "address": "0x812bc60",
    "value": "sqlite3_create_function"
  },
  {
    "address": "0x812bea0",
    "value": "sqlite3_create_function16"
  },
  {
    "address": "0x812bf40",
    "value": "sqliteDefaultBusyCallback"
  },
  {
    "address": "0x8130ef0",
    "value": "targetSrcList"
  },
  {
    "address": "0x8132210",
    "value": "sqlite3ColumnDefault"
  },
  {
    "address": "0x813d700",
    "value": "sqlite3AttachFunctions"
  },
  {
    "address": "0x813d790",
    "value": "detachFunc"
  },
  {
    "address": "0x813d940",
    "value": "attachFunc"
  },
  {
    "address": "0x813e8f0",
    "value": "unlockBtreeIfUnused"
  },
  {
    "address": "0x8145480",
    "value": "sqlite3DefaultRowEst"
  },
  {
    "address": "0x8146930",
    "value": "sqlite3AddDefaultValue"
  },
  {
    "address": "0x8149df0",
    "value": "sqlite3FindFunction"
  },
  {
    "address": "0x814b250",
    "value": "sqlite3ExprIsConstantOrFunction"
  },
  {
    "address": "0x814cc60",
    "value": "sqlite3ExprFunction"
  },
  {
    "address": "0x814e8f0",
    "value": "sqlite3GetFuncCollSeq"
  },
  {
    "address": "0x814e900",
    "value": "sqlite3IsLikeFunction"
  },
  {
    "address": "0x814e9f0",
    "value": "sqlite3RegisterLikeFunctions"
  },
  {
    "address": "0x814ef60",
    "value": "likeFunc"
  },
  {
    "address": "0x814f050",
    "value": "sqlite3RegisterBuiltinFunctions"
  },
  {
    "address": "0x814f2c0",
    "value": "versionFunc"
  },
  {
    "address": "0x814f2f0",
    "value": "typeofFunc"
  },
  {
    "address": "0x814f360",
    "value": "ifnullFunc"
  },
  {
    "address": "0x814f3c0",
    "value": "quoteFunc"
  },
  {
    "address": "0x814f610",
    "value": "nullifFunc"
  },
  {
    "address": "0x814f670",
    "value": "minmaxFunc"
  },
  {
    "address": "0x814f730",
    "value": "randomFunc"
  },
  {
    "address": "0x814f760",
    "value": "lowerFunc"
  },
  {
    "address": "0x814f840",
    "value": "upperFunc"
  },
  {
    "address": "0x814f920",
    "value": "substrFunc"
  },
  {
    "address": "0x814faf0",
    "value": "roundFunc"
  },
  {
    "address": "0x814fbb0",
    "value": "lengthFunc"
  },
  {
    "address": "0x814fc50",
    "value": "absFunc"
  },
  {
    "address": "0x8150070",
    "value": "hashFunction"
  },
  {
    "address": "0x815afb0",
    "value": "finalizeAggFunctions"
  },
  {
    "address": "0x81657a0",
    "value": "sqlite3AlterFunctions"
  },
  {
    "address": "0x8165810",
    "value": "renameTriggerFunc"
  },
  {
    "address": "0x8165910",
    "value": "renameTableFunc"
  },
  {
    "address": "0x8165d90",
    "value": "sqlite3RegisterDateTimeFunctions"
  },
  {
    "address": "0x8166bc0",
    "value": "strftimeFunc"
  },
  {
    "address": "0x8167020",
    "value": "dateFunc"
  },
  {
    "address": "0x81670a0",
    "value": "cdateFunc"
  },
  {
    "address": "0x8167110",
    "value": "timeFunc"
  },
  {
    "address": "0x8167190",
    "value": "ctimeFunc"
  },
  {
    "address": "0x8167200",
    "value": "datetimeFunc"
  },
  {
    "address": "0x8167290",
    "value": "ctimestampFunc"
  },
  {
    "address": "0x8167300",
    "value": "juliandayFunc"
  },
  {
    "address": "0x81673b0",
    "value": "vm_shutdownTinyXmlEfun"
  },
  {
    "address": "0x81673c0",
    "value": "vm_initTinyXmlEfun"
  },
  {
    "address": "0x816f9a0",
    "value": "shutdownMysqlEfun"
  },
  {
    "address": "0x816f9b0",
    "value": "initMysqlEfun"
  },
  {
    "address": "0x8171620",
    "value": "vm_shutdownVerigenEfun"
  },
  {
    "address": "0x8171630",
    "value": "vm_initVerigenEfun"
  },
  {
    "address": "0x8171840",
    "value": "verigen_register_mem_funcs"
  },
  {
    "address": "0x8173940",
    "value": "alloc_func"
  },
  {
    "address": "0x8173960",
    "value": "free_func"
  },
  {
    "address": "0x8173980",
    "value": "realloc_func"
  },
  {
    "address": "0x817a260",
    "value": "vm_shutdownLanguageEfun"
  },
  {
    "address": "0x817a270",
    "value": "vm_initLanguageEfun"
  },
  {
    "address": "0x819e710",
    "value": "__cmpfunc"
  },
  {
    "address": "0x81f7980",
    "value": "__db_get_arg"
  },
  {
    "address": "0x81f8ca0",
    "value": "__db_c_get_arg"
  },
  {
    "address": "0x82028b0",
    "value": "__ham_func2"
  },
  {
    "address": "0x8202930",
    "value": "__ham_func3"
  },
  {
    "address": "0x8202a50",
    "value": "__ham_func4"
  },
  {
    "address": "0x8202b50",
    "value": "__ham_func5"
  },
  {
    "address": "0x822c6f0",
    "value": "mysql_set_local_infile_default"
  },
  {
    "address": "0x822e530",
    "value": "default_local_infile_error"
  },
  {
    "address": "0x822e7f0",
    "value": "default_local_infile_end"
  },
  {
    "address": "0x822e830",
    "value": "default_local_infile_read"
  },
  {
    "address": "0x822e8c0",
    "value": "default_local_infile_init"
  },
  {
    "address": "0x8234c90",
    "value": "reset_root_defaults"
  },
  {
    "address": "0x8240250",
    "value": "mysql_read_default_options"
  },
  {
    "address": "0x8246580",
    "value": "print_defaults"
  },
  {
    "address": "0x8246720",
    "value": "free_defaults"
  },
  {
    "address": "0x8246780",
    "value": "get_defaults_files"
  },
  {
    "address": "0x8246800",
    "value": "search_default_file_with_ext"
  },
  {
    "address": "0x8247210",
    "value": "load_defaults"
  },
  {
    "address": "0x82519f4",
    "value": "FT_Add_Default_Modules"
  },
  {
    "address": "0x825ecdc",
    "value": "T1_Size_Get_Globals_Funcs"
  },
  {
    "address": "0x826203c",
    "value": "cff_size_get_globals_funcs"
  },
  {
    "address": "0x82660b0",
    "value": "cid_size_get_globals_funcs"
  },
  {
    "address": "0x8280b68",
    "value": "pshinter_get_globals_funcs"
  },
  {
    "address": "0x8280b74",
    "value": "pshinter_get_t1_funcs"
  },
  {
    "address": "0x8280b80",
    "value": "pshinter_get_t2_funcs"
  },
  {
    "address": "0x82ca43c",
    "value": "FUN_082ca43c"
  }
]
```

## 新增：(resort|default|arg|gen.*(Value|Action)) — search_procedures

Evidence ID：`ev_3c95f227f35b62acbb04d47d5661a9036fd8985f434e03f4804a0b7533ad9564`

```json
[
  {
    "address": "0x805e880",
    "value": "vm_msgDoRecharge"
  },
  {
    "address": "0x805e9e4",
    "value": "vm_cmdRequestRecharge"
  },
  {
    "address": "0x805ea24",
    "value": "vm_cmdRecharge"
  },
  {
    "address": "0x80618b4",
    "value": "vm_msgLChargeResult"
  },
  {
    "address": "0x8062f90",
    "value": "vm_msgLDoAlipayRecharge"
  },
  {
    "address": "0x8063b58",
    "value": "vm_msgUpdateRechargeReturnCredit"
  },
  {
    "address": "0x806be60",
    "value": "vm_cmdChargeVendueOfferPrice"
  },
  {
    "address": "0x806bf0c",
    "value": "vm_cmdChargeVendueRefresh"
  },
  {
    "address": "0x806bf4c",
    "value": "vm_cmdChargeVendueFetch"
  },
  {
    "address": "0x806bfd0",
    "value": "vm_cmdChargeVendueStoreClosed"
  },
  {
    "address": "0x8076f60",
    "value": "vm_cmdCitywarGiveOrders"
  },
  {
    "address": "0x807ea8c",
    "value": "vm_getDefaultAttrib"
  },
  {
    "address": "0x807eaa0",
    "value": "vm_setDefaultAttrib"
  },
  {
    "address": "0x8081e28",
    "value": "vm_getLocalDefaultObject"
  },
  {
    "address": "0x8081e34",
    "value": "vm_getGlobalDefaultObject"
  },
  {
    "address": "0x8086df0",
    "value": "vm_getFunctionInDefaultObject"
  },
  {
    "address": "0x8089630",
    "value": "vm_scratchLargeAlloc"
  },
  {
    "address": "0x80896c8",
    "value": "vm_scratchLargeCopy"
  },
  {
    "address": "0x808a364",
    "value": "vm_getArgument"
  },
  {
    "address": "0x808a5ec",
    "value": "vm_getXIntArgument"
  },
  {
    "address": "0x808a770",
    "value": "vm_getRealArgument"
  },
  {
    "address": "0x808a8ec",
    "value": "vm_getStringArgument"
  },
  {
    "address": "0x808cbf4",
    "value": "vm_setCurrentArgument"
  },
  {
    "address": "0x808ce3c",
    "value": "vm_safeGetPCBArgument"
  },
  {
    "address": "0x808d0a4",
    "value": "vm_getPCBArgument"
  },
  {
    "address": "0x808d310",
    "value": "vm_getPCBXIntArgument"
  },
  {
    "address": "0x808d474",
    "value": "vm_getPCBIntArgument"
  },
  {
    "address": "0x808d5c0",
    "value": "vm_getPCBRealArgument"
  },
  {
    "address": "0x808d730",
    "value": "vm_getPCBStringArgument"
  },
  {
    "address": "0x80916cc",
    "value": "vm_pushArgNum"
  },
  {
    "address": "0x8091fb4",
    "value": "vm_createArgumentArray"
  },
  {
    "address": "0x8093400",
    "value": "vm_argumentInThisContext"
  },
  {
    "address": "0x8094ec0",
    "value": "vm_adjustArgumentNum"
  },
  {
    "address": "0x809521c",
    "value": "vm_checkArgNum"
  },
  {
    "address": "0x80954a8",
    "value": "vm_resortStack"
  },
  {
    "address": "0x8095e74",
    "value": "vm_genXAction"
  },
  {
    "address": "0x8097584",
    "value": "vm_caseDefault"
  },
  {
    "address": "0x8099678",
    "value": "vm_insertArgsIntoFunction"
  },
  {
    "address": "0x80a1c40",
    "value": "vm_defaultReadFromConsole"
  },
  {
    "address": "0x80a1c9c",
    "value": "vm_defaultWriteToConsole"
  },
  {
    "address": "0x80a1cb8",
    "value": "vm_defaultCntlConsole"
  },
  {
    "address": "0x80a1cd4",
    "value": "vm_combineIsGlobalTypeArgn"
  },
  {
    "address": "0x80a1d9c",
    "value": "vm_undefineLocalVarOrArg"
  },
  {
    "address": "0x80a20d0",
    "value": "vm_buildDefaultValueNodes"
  },
  {
    "address": "0x80b9728",
    "value": "vm_parseCmdLineArguments"
  },
  {
    "address": "0x80b9848",
    "value": "vm_freeCmdLineArguments"
  },
  {
    "address": "0x80b9a64",
    "value": "vm_analyseArguments"
  },
  {
    "address": "0x80c1698",
    "value": "vm_matchPktArguments"
  },
  {
    "address": "0x80c2178",
    "value": "vm_deriveArguments"
  },
  {
    "address": "0x80d4414",
    "value": "vm_setArgInfo"
  },
  {
    "address": "0x80d62d4",
    "value": "vm_f_find_function_arg"
  },
  {
    "address": "0x80d6418",
    "value": "vm_f_get_args"
  },
  {
    "address": "0x80edc10",
    "value": "vm_f_previous_arguments"
  },
  {
    "address": "0x80f5afc",
    "value": "vm_f_call_other_arg"
  },
  {
    "address": "0x80fbabc",
    "value": "vm_getTableByArgument"
  },
  {
    "address": "0x80fdb4c",
    "value": "vm_getTreeByArgument"
  },
  {
    "address": "0x811ceb4",
    "value": "default_bzalloc"
  },
  {
    "address": "0x811cecc",
    "value": "default_bzfree"
  },
  {
    "address": "0x812bf40",
    "value": "sqliteDefaultBusyCallback"
  },
  {
    "address": "0x8130ef0",
    "value": "targetSrcList"
  },
  {
    "address": "0x8132210",
    "value": "sqlite3ColumnDefault"
  },
  {
    "address": "0x8145480",
    "value": "sqlite3DefaultRowEst"
  },
  {
    "address": "0x8146930",
    "value": "sqlite3AddDefaultValue"
  },
  {
    "address": "0x81f7980",
    "value": "__db_get_arg"
  },
  {
    "address": "0x81f8ca0",
    "value": "__db_c_get_arg"
  },
  {
    "address": "0x822c6f0",
    "value": "mysql_set_local_infile_default"
  },
  {
    "address": "0x822e530",
    "value": "default_local_infile_error"
  },
  {
    "address": "0x822e7f0",
    "value": "default_local_infile_end"
  },
  {
    "address": "0x822e830",
    "value": "default_local_infile_read"
  },
  {
    "address": "0x822e8c0",
    "value": "default_local_infile_init"
  },
  {
    "address": "0x8234c90",
    "value": "reset_root_defaults"
  },
  {
    "address": "0x8240250",
    "value": "mysql_read_default_options"
  },
  {
    "address": "0x8246580",
    "value": "print_defaults"
  },
  {
    "address": "0x8246720",
    "value": "free_defaults"
  },
  {
    "address": "0x8246780",
    "value": "get_defaults_files"
  },
  {
    "address": "0x8246800",
    "value": "search_default_file_with_ext"
  },
  {
    "address": "0x8247210",
    "value": "load_defaults"
  },
  {
    "address": "0x82519f4",
    "value": "FT_Add_Default_Modules"
  }
]
```

## 新增：vm_adjustArgumentNum — procedure_references

Evidence ID：`ev_e04f598e0861441e9358ac37cbb0e3466e99a4f7f58ef92fe9462739d6a5dfa3`

函数地址：`0x8094ec0`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[
  {
    "source": "0x80dd86f",
    "target": "0x8094ec0",
    "target_function": "vm_adjustArgumentNum",
    "source_function": "vm_f_printf",
    "kind": "UNCONDITIONAL_CALL"
  }
]
```

## 新增：vm_buildDefaultValueNodes — procedure_references

Evidence ID：`ev_1c650eb10db326b06b635b9d9755a9f1a7693f7545ce2fc9f47b9ea6b4c6d619`

函数地址：`0x80a20d0`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[
  {
    "source": "0x8104a4f",
    "target": "0x80a20d0",
    "target_function": "vm_buildDefaultValueNodes",
    "source_function": "vm_yyparse",
    "kind": "UNCONDITIONAL_CALL"
  }
]
```

## 新增：vm_adjustArgumentNum — procedure_references

Evidence ID：`ev_9f3d040916e8bf460d456d98e240b5e2d119271ef5f7d9d4a5f62cab558c8e9f`

函数地址：`0x8094ec0`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[]
```

## 新增：vm_buildDefaultValueNodes — procedure_references

Evidence ID：`ev_3bf6987ff1b574b94a659dfb70f74241175f9ef8f20e83188940b46f53d5ff2a`

函数地址：`0x80a20d0`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[
  {
    "source": "0x80a2105",
    "target": "0x8096d3c",
    "target_function": "vm_appendXActions",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a2140",
    "target": "0x8095e74",
    "target_function": "vm_genXAction",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a215f",
    "target": "0x8095e74",
    "target_function": "vm_genXAction",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a2169",
    "target": "0x8096d3c",
    "target_function": "vm_appendXActions",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a2184",
    "target": "0x8095e74",
    "target_function": "vm_genXAction",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a218e",
    "target": "0x8096d3c",
    "target_function": "vm_appendXActions",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a2197",
    "target": "0x8096d3c",
    "target_function": "vm_appendXActions",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a21d8",
    "target": "0x8095e74",
    "target_function": "vm_genXAction",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x80a21e2",
    "target": "0x8096d3c",
    "target_function": "vm_appendXActions",
    "source_function": "vm_buildDefaultValueNodes",
    "kind": "UNCONDITIONAL_CALL"
  }
]
```

## 新增：vm_createArgumentArray — procedure_references

Evidence ID：`ev_f559993057f0ecabc76210feabdbaef897b6ceff73b67ecd8e78f0d1ed22be3a`

函数地址：`0x8091fb4`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[
  {
    "source": "0x8091fb9",
    "target": "0x8093400",
    "target_function": "vm_argumentInThisContext",
    "source_function": "vm_createArgumentArray",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x8091fcc",
    "target": "0x80914dc",
    "target_function": "vm_pushLocalVar",
    "source_function": "vm_createArgumentArray",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x8091fde",
    "target": "0x80918c8",
    "target_function": "vm_createArray",
    "source_function": "vm_createArgumentArray",
    "kind": "UNCONDITIONAL_CALL"
  }
]
```

## 新增：vm_pushArgNum — procedure_references

Evidence ID：`ev_0c687fad352bb734d1f0d96b9a968d94d8120a80522725456f821c6de9041937`

函数地址：`0x80916cc`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[
  {
    "source": "0x80916da",
    "target": "0x8093400",
    "target_function": "vm_argumentInThisContext",
    "source_function": "vm_pushArgNum",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x8091720",
    "target": "0x8097b24",
    "target_function": "vm_error",
    "source_function": "vm_pushArgNum",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x8091728",
    "target": "0x8093400",
    "target_function": "vm_argumentInThisContext",
    "source_function": "vm_pushArgNum",
    "kind": "UNCONDITIONAL_CALL"
  },
  {
    "source": "0x8091740",
    "target": "0x8097b24",
    "target_function": "vm_error",
    "source_function": "vm_pushArgNum",
    "kind": "UNCONDITIONAL_CALL"
  }
]
```

## 新增：vm_resortStack — procedure_references

Evidence ID：`ev_83867023ed9b8813c89b5908b2824066456b313ce6a7c203c040dfe4d5890306`

函数地址：`0x80954a8`；以下列出调用引用，完整数据和跳转引用保存在原始记录中。

```json
[]
```

## 新增：vm_adjustArgumentNum — procedure_assembly

Evidence ID：`ev_ec468cd1bdc9e4960feb347de97932324aa4c65c6b2931c7ef08742241eddbec`

```asm
0x8094ec0: MOV EAX, [0x086619c0]
0x8094ec5: PUSH EBP
0x8094ec6: LEA EAX, [EAX + EAX*0x4]
0x8094ec9: MOV EBP, ESP
0x8094ecb: SHL EAX, 0x3
0x8094ece: PUSH EBX
0x8094ecf: MOV ECX, dword ptr [0x086619c8]
0x8094ed5: MOV EBX, dword ptr [EBP + 0x8]
0x8094ed8: MOV EDX, dword ptr [EAX + ECX*0x1 + 0x4]
0x8094edc: CMP EDX, EBX
0x8094ede: JZ 0x08094ef1
0x8094ee0: SUB EDX, EBX
0x8094ee2: MOV dword ptr [EAX + ECX*0x1 + 0x4], EBX
0x8094ee6: ADD dword ptr [EAX + ECX*0x1], EDX
0x8094ee9: MOV EAX, [0x086619d8]
0x8094eee: MOV dword ptr [EAX + 0x4], EBX
0x8094ef1: POP EBX
0x8094ef2: LEAVE
0x8094ef3: RET
```

## 新增：vm_adjustArgumentNum — procedure_pseudo_code

Evidence ID：`ev_94eaf2ef0a5e40a1bed6d105f0a4b53eb4f09cb6819fe5694bb3b96ed5fa17da`

```c

void vm_adjustArgumentNum(int newArgNum)

{
  int iVar1;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  
  pVVar3 = vm_funContext;
  iVar2 = vm_funCp;
  iVar1 = vm_funContext[vm_funCp].argNum;
  if (iVar1 != newArgNum) {
    vm_funContext[vm_funCp].argNum = newArgNum;
    pVVar3[iVar2].bp = pVVar3[iVar2].bp + (iVar1 - newArgNum);
    vm_currentContext->argNum = newArgNum;
  }
  return;
}
```

## 新增：vm_argumentInThisContext — procedure_assembly

Evidence ID：`ev_414a18bbd953f729969695ef06915f4871121b81f6814d8112df892ab3fe9a14`

```asm
0x8093400: MOV EDX, dword ptr [0x086619c0]
0x8093406: PUSH EBP
0x8093407: XOR EAX, EAX
0x8093409: TEST EDX, EDX
0x809340b: MOV EBP, ESP
0x809340d: JZ 0x0809341b
0x809340f: LEA EDX, [EDX + EDX*0x4]
0x8093412: MOV EAX, [0x086619c8]
0x8093417: MOV EAX, dword ptr [EAX + EDX*0x8 + 0x4]
0x809341b: LEAVE
0x809341c: RET
```

## 新增：vm_argumentInThisContext — procedure_pseudo_code

Evidence ID：`ev_94ab3eba34c6a8ee8a84a3290d81f8b440d1c04a08d02850419510fc1d39f1d9`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_argumentInThisContext(void)

{
  int iVar1;
  
  iVar1 = 0;
  if (vm_funCp != 0) {
    iVar1 = vm_funContext[vm_funCp].argNum;
  }
  return iVar1;
}
```

## 新增：vm_buildDefaultValueNodes — procedure_assembly

Evidence ID：`ev_f7590616a34561205b430f9bfacdab404ef9c0c10d7447b91c9a9726ac29b0bc`

```asm
0x80a20d0: PUSH EBP
0x80a20d1: MOV EBP, ESP
0x80a20d3: PUSH EDI
0x80a20d4: PUSH ESI
0x80a20d5: PUSH EBX
0x80a20d6: SUB ESP, 0xc
0x80a20d9: MOV EAX, dword ptr [EBP + 0x8]
0x80a20dc: MOV EBX, dword ptr [EAX + 0x30]
0x80a20df: XOR EDI, EDI
0x80a20e1: XOR ESI, ESI
0x80a20e3: TEST EBX, EBX
0x80a20e5: MOV byte ptr [EAX + 0xf], 0x0
0x80a20e9: JZ 0x080a2127
0x80a20eb: NOP
0x80a20ec: MOV EAX, dword ptr [EBX + 0xc]
0x80a20ef: INC ESI
0x80a20f0: TEST EAX, EAX
0x80a20f2: JZ 0x080a21a4
0x80a20f8: TEST EDI, EDI
0x80a20fa: JZ 0x080a21c0
0x80a2100: SUB ESP, 0x8
0x80a2103: PUSH EAX
0x80a2104: PUSH EDI
0x80a2105: CALL 0x08096d3c
0x80a210a: MOV EDI, EAX
0x80a210c: ADD ESP, 0x10
0x80a210f: MOV EAX, dword ptr [EBX + 0xc]
0x80a2112: MOV EAX, dword ptr [EAX + 0x18]
0x80a2115: MOV word ptr [EBX + 0xa], AX
0x80a2119: MOV dword ptr [EBX + 0xc], 0x0
0x80a2120: MOV EBX, dword ptr [EBX + 0x10]
0x80a2123: TEST EBX, EBX
0x80a2125: JNZ 0x080a20ec
0x80a2127: SUB ESP, 0xc
0x80a212a: MOV EAX, dword ptr [EBP + 0x8]
0x80a212d: MOV EDX, ESI
0x80a212f: MOV byte ptr [EAX + 0xe], DL
0x80a2132: PUSH 0x0
0x80a2134: PUSH 0x0
0x80a2136: PUSH 0x1b
0x80a2138: PUSH 0x0
0x80a213a: PUSH dword ptr [0x08667a68]
0x80a2140: CALL 0x08095e74
0x80a2145: ADD ESP, 0x20
0x80a2148: TEST EDI, EDI
0x80a214a: MOV EBX, EAX
0x80a214c: JZ 0x080a21c7
0x80a214e: SUB ESP, 0xc
0x80a2151: PUSH 0x0
0x80a2153: PUSH 0x0
0x80a2155: PUSH 0x47
0x80a2157: PUSH 0x0
0x80a2159: PUSH dword ptr [0x08667a68]
0x80a215f: CALL 0x08095e74
0x80a2164: ADD ESP, 0x18
0x80a2167: PUSH EAX
0x80a2168: PUSH EDI
0x80a2169: CALL 0x08096d3c
0x80a216e: MOV dword ptr [ESP], 0x0
0x80a2175: PUSH dword ptr [EAX + 0x18]
0x80a2178: PUSH 0xf
0x80a217a: PUSH 0x1
0x80a217c: PUSH dword ptr [0x08667a68]
0x80a2182: MOV EDI, EAX
0x80a2184: CALL 0x08095e74
0x80a2189: ADD ESP, 0x18
0x80a218c: PUSH EDI
0x80a218d: PUSH EAX
0x80a218e: CALL 0x08096d3c
0x80a2193: POP EDX
0x80a2194: POP ECX
0x80a2195: PUSH EBX
0x80a2196: PUSH EAX
0x80a2197: CALL 0x08096d3c
0x80a219c: LEA ESP, [EBP + -0xc]
0x80a219f: POP EBX
0x80a21a0: POP ESI
0x80a21a1: POP EDI
0x80a21a2: LEAVE
0x80a21a3: RET
0x80a21a4: CMP byte ptr [EBX + 0x5], 0xff
0x80a21a8: JZ 0x080a2120
0x80a21ae: MOV EDX, ESI
0x80a21b0: MOV EAX, dword ptr [EBP + 0x8]
0x80a21b3: MOV byte ptr [EAX + 0xf], DL
0x80a21b6: MOV EBX, dword ptr [EBX + 0x10]
0x80a21b9: JMP 0x080a2123
0x80a21c0: MOV EDI, EAX
0x80a21c2: JMP 0x080a2112
0x80a21c7: SUB ESP, 0xc
0x80a21ca: PUSH 0x0
0x80a21cc: PUSH 0x0
0x80a21ce: PUSH 0xf
0x80a21d0: PUSH 0x1
0x80a21d2: PUSH dword ptr [0x08667a68]
0x80a21d8: CALL 0x08095e74
0x80a21dd: ADD ESP, 0x18
0x80a21e0: PUSH EBX
0x80a21e1: PUSH EAX
0x80a21e2: CALL 0x08096d3c
0x80a21e7: LEA ESP, [EBP + -0xc]
0x80a21ea: POP EBX
0x80a21eb: POP ESI
0x80a21ec: POP EDI
0x80a21ed: LEAVE
0x80a21ee: RET
```

## 新增：vm_buildDefaultValueNodes — procedure_pseudo_code

Evidence ID：`ev_2c27ff7488e9819ae0c431411d6590807eaddf4a9037881bcf3d3e67b56ca294`

```c

/* WARNING: Unknown calling convention */

Vm_XNode * vm_buildDefaultValueNodes(Vm_Fun_T *pFun)

{
  Vm_Fun_Arg_T *pVVar1;
  Vm_XNode_T *pVVar2;
  Vm_XNode_T *pVVar3;
  char cVar4;
  Vm_XNode_T *pVVar5;
  
                    /* Unresolved local var: Vm_XNode_T * pCheckArgNum@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_XNode_T * pSetDefaultValues@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_XNode_T * pResortStack@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_XNode_T * pEnterFunction@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: int argNo@[DW_OP_reg6(ESI)] */
  pVVar1 = pFun->pArgs;
  pVVar5 = (Vm_XNode_T *)0x0;
  cVar4 = '\0';
  pFun->minArgNo = '\0';
  do {
    while( true ) {
      if (pVVar1 == (Vm_Fun_Arg_T *)0x0) {
        pFun->maxArgNo = cVar4;
        pVVar3 = vm_genXAction(vm_pInFunction,0,'\x1b',0,0);
        if (pVVar5 != (Vm_XNode_T *)0x0) {
          pVVar2 = vm_genXAction(vm_pInFunction,0,'G',0,0);
          pVVar5 = vm_appendXActions(pVVar5,pVVar2);
          pVVar2 = vm_genXAction(vm_pInFunction,1,'\x0f',pVVar5->num,0);
          pVVar5 = vm_appendXActions(pVVar2,pVVar5);
          pVVar5 = vm_appendXActions(pVVar5,pVVar3);
          return pVVar5;
        }
        pVVar5 = vm_genXAction(vm_pInFunction,1,'\x0f',0,0);
        pVVar5 = vm_appendXActions(pVVar5,pVVar3);
        return pVVar5;
      }
      pVVar3 = pVVar1->pDefaultValue;
      cVar4 = cVar4 + '\x01';
      if (pVVar3 == (Vm_XNode_T *)0x0) break;
      pVVar2 = pVVar3;
      if (pVVar5 != (Vm_XNode_T *)0x0) {
        pVVar2 = vm_appendXActions(pVVar5,pVVar3);
        pVVar3 = pVVar1->pDefaultValue;
      }
      pVVar1->defaultActionNum = (Vm_Index_T)pVVar3->num;
      pVVar1->pDefaultValue = (Vm_XNode *)0x0;
      pVVar5 = pVVar2;
LAB_080a2120:
      pVVar1 = pVVar1->pNext;
    }
    if (pVVar1->type == 0xff) goto LAB_080a2120;
    pFun->minArgNo = cVar4;
    pVVar1 = pVVar1->pNext;
  } while( true );
}
```

## 新增：vm_createArgumentArray — procedure_assembly

Evidence ID：`ev_8a50a757166bed6d254d5e96dafee266f7458421de87a7569ebbd797c419ffe0`

```asm
0x8091fb4: PUSH EBP
0x8091fb5: MOV EBP, ESP
0x8091fb7: PUSH ESI
0x8091fb8: PUSH EBX
0x8091fb9: CALL 0x08093400
0x8091fbe: MOV ESI, EAX
0x8091fc0: LEA EBX, [EAX + -0x1]
0x8091fc3: JMP 0x08091fd5
0x8091fc8: SUB ESP, 0xc
0x8091fcb: PUSH EBX
0x8091fcc: CALL 0x080914dc
0x8091fd1: DEC EBX
0x8091fd2: ADD ESP, 0x10
0x8091fd5: CMP EBX, -0x1
0x8091fd8: JNZ 0x08091fc8
0x8091fda: SUB ESP, 0xc
0x8091fdd: PUSH ESI
0x8091fde: CALL 0x080918c8
0x8091fe3: LEA ESP, [EBP + -0x8]
0x8091fe6: POP EBX
0x8091fe7: POP ESI
0x8091fe8: LEAVE
0x8091fe9: RET
```

## 新增：vm_createArgumentArray — procedure_pseudo_code

Evidence ID：`ev_ad365e363694b53417408272d3fcbd105f6dbf3a8d1d0703d04ab4bd23b28021`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_createArgumentArray(void)

{
  uint n;
  uint no;
  
                    /* Unresolved local var: int n@[DW_OP_reg6(ESI)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  n = vm_argumentInThisContext();
  no = n;
  while (no = no - 1, no != 0xffffffff) {
    vm_pushLocalVar(no);
  }
  vm_createArray(n);
  return;
}
```

## 新增：vm_f_get_args — procedure_pseudo_code

Evidence ID：`ev_493f85df70bef526d372e8b86e91e68a4c03f5d7a54177629b294e4d19bab632`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_get_args(void)

{
  Vm_Array_T *pVVar1;
  int iVar2;
  
                    /* Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg3(EBX)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  if (0 < vm_currentContext->argNum) {
    vm_error("Too many arguments to args.\n");
  }
  if (vm_argn < 1) {
    pVVar1 = vm_allocateEmptyArray(0);
    vm_pushArray(pVVar1);
    return;
  }
  pVVar1 = vm_allocateArray(vm_argn - 1);
  if (pVVar1 == (Vm_Array_T *)0x0) {
    vm_error("Failed to allocate array for inputed arguments.\n");
  }
  iVar2 = 1;
  vm_pushArray(pVVar1);
  if (1 < vm_argn) {
    do {
      vm_pushString(vm_argv[iVar2]);
      iVar2 = iVar2 + 1;
      vm_addValueIntoArray();
    } while (iVar2 < vm_argn);
  }
  return;
}
```

## 新增：vm_finalizeFun — procedure_pseudo_code

Evidence ID：`ev_ac11d24ef0dd2c2398a02826fd57a977d4381fa3668add2ab2c2553424e05abd`

```c

void vm_finalizeFun(Vm_Fun_T *pFun,Vm_Index_T funOffset,int halfFinalized)

{
  Vm_Fun_Entry_T *pVVar1;
  Vm_Program *pVVar2;
  Vm_Fun_Arg_T *pVVar3;
  int halfFinalized_local;
  
                    /* Unresolved local var: Vm_Fun_Arg_T * pVar@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Entry_T * pFunEntry@[DW_OP_reg2(EDX)] */
  vm_finalizeCodeSegment(&pFun->codeSegment);
  for (pVVar3 = pFun->pVars; pVVar3 != (Vm_Fun_Arg_T *)0x0; pVVar3 = pVVar3->pNext) {
    vm_scratchFinalize(pVVar3->name);
    vm_scratchFinalize(pVVar3);
  }
  pFun->pLabels = (Vm_Local_Label_T *)0x0;
  pVVar2 = vm_compilingProgram();
  pVVar1 = pVVar2->pFunctions;
  pVVar1[funOffset].consititutionNo = 0;
  pVVar1[funOffset].pFun = pFun;
  if (halfFinalized == 0) {
    for (pVVar3 = pFun->pArgs; pVVar3 != (Vm_Fun_Arg_T *)0x0; pVVar3 = pVVar3->pNext) {
      pVVar3->pDefaultValue = (Vm_XNode *)0x0;
      vm_scratchFinalize(pVVar3->name);
      vm_scratchFinalize(pVVar3);
    }
    vm_scratchFinalize(pFun->name);
    vm_scratchFinalize(pFun);
    return;
  }
  return;
}
```

## 新增：vm_loadBinary — procedure_pseudo_code

Evidence ID：`ev_a8404c9fbff4bd62e5d7316362dfa2b97574e7bd2dc5b5c24ed16dff00054019`

```c

int vm_loadBinary(int fd,Vm_Code_Segment_T *pCodeSegment)

{
  Vm_Index_T VVar1;
  Vm_Program *pProgram_00;
  Vm_String **ppVVar2;
  char *__dest;
  uint uVar3;
  int iVar4;
  char *pBuffer;
  uint len;
  Vm_Code_Segment_T *pVVar5;
  bool bVar6;
  void *buf_1;
  int i;
  Vm_Program_T *pProgram;
  int ret;
  char *str_1;
  int size;
  char *str;
  int newCount;
  int num;
  Vm_Binary_Buffer_T buf;
  
  pVVar5 = pCodeSegment;
                    /* Unresolved local var: Vm_Binary_Buffer_T * pInput@[???] */
  for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
    pVVar5->nodesCount = 0;
    pVVar5 = (Vm_Code_Segment_T *)&pVVar5->nodes;
  }
  buf.fd = fd;
  buf.p = buf.data;
  buf.len = 0;
  iVar4 = vm_fetchBinaryBufferData(&buf);
                    /* Unresolved local var: int temp@[DW_OP_reg2(EDX)] */
  if ((iVar4 != 0) && ((3 < (uint)buf.len || (vm_fetchBinaryBufferData(&buf), 3 < (uint)buf.len))))
  {
    iVar4 = *(int *)buf.p;
    buf.p = buf.p + 4;
    buf.len = buf.len - 4;
    if (&stack0x00000000 != (undefined1 *)0xfd0) {
      num = iVar4;
    }
    if (num == 0x10001) {
                    /* Unresolved local var: int temp@[???] */
      if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
        return 0;
      }
      buf.len = buf.len - 4;
      buf.p = buf.p + 4;
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
      if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
        return 0;
      }
      iVar4 = *(int *)buf.p;
      buf.p = buf.p + 4;
      buf.len = buf.len - 4;
      if (&stack0x00000000 != (undefined1 *)0xfd0) {
        num = iVar4;
      }
      if (vm_functionBC == num) {
        iVar4 = vm_loadDefines(&buf);
        if (iVar4 == 0) {
          return 0;
        }
        pProgram_00 = vm_compilingProgram();
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
        if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
          return 0;
        }
        iVar4 = *(int *)buf.p;
        buf.p = buf.p + 4;
        buf.len = buf.len + -4;
        if (&stack0x00000000 != (undefined1 *)0xfd4) {
          newCount = iVar4;
        }
        if (newCount < 0) {
          return 0;
        }
        binaryStrPoolSize = newCount;
        if (pProgram_00->maxStringNo == 0) {
          pStrOffsetMapping = (int *)0x0;
        }
        else {
          pStrOffsetMapping = vm_scratchAlloc(newCount * 4);
        }
        if (pProgram_00->stringBufSize < newCount) {
                    /* Unresolved local var: Vm_String_T * * ppNewStrings@[DW_OP_reg0(EAX)]
                       Unresolved local var: int newSize@[???] */
          if (pProgram_00->ppStrings == (Vm_String **)0x0) {
            ppVVar2 = pdb_allocateMemory(newCount * 4,"VM",
                                         "/home/cc/xxxx/gameshell/src/vm/vm_binary.c",0x2f1);
          }
          else {
            ppVVar2 = pdb_reallocateMemory
                                (pProgram_00->ppStrings,newCount * 4,"VM",
                                 "/home/cc/xxxx/gameshell/src/vm/vm_binary.c",0x2f5);
          }
          if (ppVVar2 == (Vm_String **)0x0) {
            return 0;
          }
          pProgram_00->ppStrings = ppVVar2;
          pProgram_00->stringBufSize = (Vm_Index_T)newCount;
        }
        i = 0;
        bVar6 = SBORROW4(0,newCount);
        iVar4 = -newCount;
        do {
          if (bVar6 == iVar4 < 0) {
            iVar4 = vm_loadIdentifiers(&buf);
            if (iVar4 == 0) {
              return 0;
            }
            iVar4 = vm_loadCodeSegment(&buf,(Vm_Fun_T *)0x0,pCodeSegment);
            if (iVar4 != 0) {
              return (uint)(pCodeSegment->nodes[pCodeSegment->nodesCount + -1].actionNo == '\x1d');
            }
            return 0;
          }
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
          if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
            return 0;
          }
          iVar4 = *(int *)buf.p;
          buf.p = buf.p + 4;
          buf.len = buf.len + -4;
          if (&stack0x00000000 != (undefined1 *)0xfdc) {
            size = iVar4;
          }
          if (size < 1) {
            str = (char *)0x0;
          }
          else {
                    /* Unresolved local var: int newStrNo@[DW_OP_reg3(EBX)] */
            __dest = vm_scratchAlloc(size);
            if ((uint)size < 0xfa1) {
              if ((buf.len < size) && (vm_fetchBinaryBufferData(&buf), buf.len < size)) {
LAB_080b9619:
                vm_scratchFree(__dest);
                return 0;
              }
              memcpy(__dest,buf.p,size);
              buf.p = buf.p + size;
              buf.len = buf.len - size;
            }
            else {
              memcpy(__dest,buf.p,buf.len);
              pBuffer = __dest + buf.len;
              len = size - buf.len;
              buf.p = buf.data;
              buf.len = 0;
              uVar3 = vm_read(pBuffer,len,buf.fd);
              if (uVar3 != len) goto LAB_080b9619;
            }
            if (&stack0x00000000 == (undefined1 *)0xfe0) {
              vm_scratchFree(__dest);
              __dest = str_1;
            }
            str_1 = __dest;
            str_1[size + -1] = '\0';
            if (&stack0x00000000 == (undefined1 *)0xfd8) {
              vm_scratchFree(str_1);
            }
            else {
              str = str_1;
            }
          }
          if (str == (char *)0x0) {
            return 0;
          }
          VVar1 = vm_findOrAddProgramString(pProgram_00,str);
          vm_scratchFree(str);
          if (pStrOffsetMapping != (int *)0x0) {
            pStrOffsetMapping[i] = (int)VVar1;
          }
          i = i + 1;
          bVar6 = SBORROW4(i,newCount);
          iVar4 = i - newCount;
        } while( true );
      }
    }
  }
  return 0;
}
```

## 新增：vm_pushArgNum — procedure_assembly

Evidence ID：`ev_82ac3ae7fae8a726ca218b98efe44a8ac06570221896d15273b0f53feba22c1a`

```asm
0x80916cc: PUSH EBP
0x80916cd: MOV EBP, ESP
0x80916cf: PUSH EBX
0x80916d0: PUSH EDX
0x80916d1: MOV EAX, [0x086619cc]
0x80916d6: TEST EAX, EAX
0x80916d8: JZ 0x08091718
0x80916da: CALL 0x08093400
0x80916df: MOV EBX, EAX
0x80916e1: MOV EAX, [0x086619cc]
0x80916e6: TEST EAX, EAX
0x80916e8: JZ 0x08091738
0x80916ea: DEC EAX
0x80916eb: MOV [0x086619cc], EAX
0x80916f0: MOV EDX, dword ptr [0x08661964]
0x80916f6: LEA EAX, [EAX + EAX*0x2]
0x80916f9: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x2
0x80916fe: MOV EAX, [0x086619cc]
0x8091703: MOV EDX, dword ptr [0x08661964]
0x8091709: LEA EAX, [EAX + EAX*0x2]
0x809170c: MOV dword ptr [EDX + EAX*0x8 + 0x8], EBX
0x8091710: MOV EBX, dword ptr [EBP + -0x4]
0x8091713: LEAVE
0x8091714: RET
0x8091718: SUB ESP, 0xc
0x809171b: PUSH 0x82ad98c
0x8091720: CALL 0x08097b24
0x8091725: ADD ESP, 0x10
0x8091728: CALL 0x08093400
0x809172d: MOV EBX, EAX
0x809172f: MOV EAX, [0x086619cc]
0x8091734: TEST EAX, EAX
0x8091736: JNZ 0x080916ea
0x8091738: SUB ESP, 0xc
0x809173b: PUSH 0x82ad98c
0x8091740: CALL 0x08097b24
0x8091745: ADD ESP, 0x10
0x8091748: MOV EAX, [0x086619cc]
0x809174d: JMP 0x080916ea
```

## 新增：vm_pushArgNum — procedure_pseudo_code

Evidence ID：`ev_ac3d0d44e6c928399cf5c31d3ae6029310624aca2d67e77651a84a51228ad183`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_pushArgNum(void)

{
  int iVar1;
  
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
    iVar1 = vm_argumentInThisContext();
  }
  else {
    iVar1 = vm_argumentInThisContext();
  }
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x02';
  vm_stack[vm_sp].v.number = iVar1;
  return;
}
```

## 新增：vm_resortStack — procedure_assembly

Evidence ID：`ev_637a26f2121f5b0547c5cab5b7e08f6c925ae8aca1945de11e2008a57ed2da66`

```asm
0x80954a8: PUSH EBP
0x80954a9: MOV EBP, ESP
0x80954ab: PUSH EDI
0x80954ac: PUSH ESI
0x80954ad: PUSH EBX
0x80954ae: SUB ESP, 0x10
0x80954b1: MOV EAX, [0x086619c0]
0x80954b6: MOV EDX, dword ptr [0x086619c8]
0x80954bc: LEA EAX, [EAX + EAX*0x4]
0x80954bf: LEA EAX, [EDX + EAX*0x8]
0x80954c2: MOV dword ptr [EBP + -0x10], EAX
0x80954c5: MOV ESI, dword ptr [EAX]
0x80954c7: MOV EBX, dword ptr [0x086619cc]
0x80954cd: MOV EAX, ESI
0x80954cf: MOV EDX, dword ptr [EBP + -0x10]
0x80954d2: MOV ECX, dword ptr [EDX + 0x4]
0x80954d5: SUB EAX, EBX
0x80954d7: MOV dword ptr [EBP + -0x18], EAX
0x80954da: ADD EAX, ECX
0x80954dc: LEA ECX, [ECX + ECX*0x2]
0x80954df: MOV dword ptr [EBP + -0x14], EAX
0x80954e2: LEA EDI, [EBX + EBX*0x2]
0x80954e5: MOV EAX, [0x08661964]
0x80954ea: SHL ECX, 0x3
0x80954ed: LEA ESI, [ESI + ESI*0x2]
0x80954f0: MOV dword ptr [EBP + -0x1c], EAX
0x80954f3: LEA EDI, [EDI*0x8 + 0x8483fc0]
0x80954fa: LEA ESI, [EAX + ESI*0x8]
0x80954fd: CLD
0x80954fe: SHR ECX, 0x2
0x8095501: MOVSD.REP ES:EDI, ESI
0x8095503: XOR ESI, ESI
0x8095505: JMP 0x0809555f
0x8095508: MOV ECX, dword ptr [EBP + -0x10]
0x809550b: MOV EDX, dword ptr [ECX + 0x4]
0x809550e: MOV EAX, dword ptr [ECX]
0x8095510: ADD EDX, ESI
0x8095512: SUB EAX, ESI
0x8095514: MOV ECX, dword ptr [EBP + -0x1c]
0x8095517: ADD EDX, EBX
0x8095519: LEA EAX, [EAX + EAX*0x2]
0x809551c: LEA EAX, [ECX + EAX*0x8]
0x809551f: LEA EDX, [EDX + EDX*0x2]
0x8095522: SHL EDX, 0x3
0x8095525: LEA ECX, [EAX + -0x18]
0x8095528: MOV EAX, dword ptr [EAX + -0x18]
0x809552b: MOV dword ptr [EDX + 0x8483fc0], EAX
0x8095531: MOV EAX, dword ptr [ECX + 0x4]
0x8095534: MOV dword ptr [EDX + 0x8483fc4], EAX
0x809553a: MOV EAX, dword ptr [ECX + 0x8]
0x809553d: MOV dword ptr [EDX + 0x8483fc8], EAX
0x8095543: MOV EAX, dword ptr [ECX + 0xc]
0x8095546: MOV dword ptr [EDX + 0x8483fcc], EAX
0x809554c: MOV EAX, dword ptr [ECX + 0x10]
0x809554f: MOV dword ptr [EDX + 0x8483fd0], EAX
0x8095555: MOV EAX, dword ptr [ECX + 0x14]
0x8095558: MOV dword ptr [EDX + 0x8483fd4], EAX
0x809555e: INC ESI
0x809555f: CMP ESI, dword ptr [EBP + -0x18]
0x8095562: JL 0x08095508
0x8095564: XOR ESI, ESI
0x8095566: CMP ESI, dword ptr [EBP + -0x14]
0x8095569: JL 0x08095575
0x809556b: JMP 0x080955cd
0x8095570: MOV EBX, EDI
0x8095572: MOV dword ptr [EBP + -0x1c], ECX
0x8095575: LEA EAX, [EBX + ESI*0x1]
0x8095578: LEA EAX, [EAX + EAX*0x2]
0x809557b: SHL EAX, 0x3
0x809557e: MOV DL, byte ptr [EAX + 0x8483fc4]
0x8095584: MOV ECX, dword ptr [EBP + -0x1c]
0x8095587: MOV byte ptr [EAX + ECX*0x1 + 0x4], DL
0x809558b: MOV EAX, [0x086619cc]
0x8095590: ADD EAX, ESI
0x8095592: LEA EAX, [EAX + EAX*0x2]
0x8095595: SHL EAX, 0x3
0x8095598: MOV CL, byte ptr [EAX + 0x8483fc5]
0x809559e: MOV EDX, dword ptr [0x08661964]
0x80955a4: MOV byte ptr [EAX + EDX*0x1 + 0x5], CL
0x80955a8: MOV EDI, dword ptr [0x086619cc]
0x80955ae: LEA EAX, [EDI + ESI*0x1]
0x80955b1: LEA EAX, [EAX + EAX*0x2]
0x80955b4: SHL EAX, 0x3
0x80955b7: INC ESI
0x80955b8: MOV ECX, dword ptr [0x08661964]
0x80955be: MOV EDX, dword ptr [EAX + 0x8483fc8]
0x80955c4: CMP ESI, dword ptr [EBP + -0x14]
0x80955c7: MOV dword ptr [EAX + ECX*0x1 + 0x8], EDX
0x80955cb: JL 0x08095570
0x80955cd: MOV EAX, dword ptr [EBP + -0x10]
0x80955d0: MOV EDX, dword ptr [EBP + -0x14]
0x80955d3: MOV dword ptr [EAX + 0x4], EDX
0x80955d6: MOV ECX, dword ptr [EBP + -0x10]
0x80955d9: MOV EAX, [0x086619cc]
0x80955de: MOV dword ptr [ECX], EAX
0x80955e0: ADD ESP, 0x10
0x80955e3: POP EBX
0x80955e4: POP ESI
0x80955e5: POP EDI
0x80955e6: LEAVE
0x80955e7: RET
```

## 新增：vm_resortStack — procedure_pseudo_code

Evidence ID：`ev_999d6b2ce2163dabf3dfe97c69fc89bbf621502b7efc62e552dbaefa85dda75e`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_resortStack(void)

{
  Vm_FunCall_Context_T *pVVar1;
  uchar uVar2;
  ushort uVar3;
  Vm_Value_T *pVVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  Vm_Value_T *pVVar11;
  Vm_Value_T *pVVar12;
  Vm_Value_T *local_20;
  int defaultPushed;
  int newArgNum;
  Vm_FunCall_Context_T *pContext;
  
  iVar10 = vm_sp;
                    /* Unresolved local var: Vm_Fun_T * pFun@[???]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  pVVar4 = vm_stack;
  pVVar1 = vm_funContext + vm_funCp;
  iVar5 = pVVar1->bp - vm_sp;
  iVar6 = iVar5 + pVVar1->argNum;
  local_20 = vm_stack;
  pVVar11 = vm_stack + pVVar1->bp;
  pVVar12 = vm_resortStack::tempStack + vm_sp;
  for (iVar8 = (pVVar1->argNum * 3 & 0x1fffffffU) << 1; iVar8 != 0; iVar8 = iVar8 + -1) {
    pVVar12->ref = pVVar11->ref;
    pVVar11 = (Vm_Value_T *)&pVVar11->type;
    pVVar12 = (Vm_Value_T *)&pVVar12->type;
  }
  for (iVar8 = 0; iVar8 < iVar5; iVar8 = iVar8 + 1) {
    iVar7 = pVVar1->bp - iVar8;
    iVar9 = pVVar1->argNum + iVar8 + iVar10;
    vm_resortStack::tempStack[iVar9].ref = pVVar4[iVar7 + -1].ref;
    pVVar11 = pVVar4 + iVar7 + -1;
    uVar2 = pVVar11->attrib;
    uVar3 = pVVar11->active;
    pVVar12 = vm_resortStack::tempStack + iVar9;
    pVVar12->type = pVVar11->type;
    pVVar12->attrib = uVar2;
    pVVar12->active = uVar3;
    vm_resortStack::tempStack[iVar9].v = pVVar4[iVar7 + -1].v;
    vm_resortStack::tempStack[iVar9].pNext = pVVar4[iVar7 + -1].pNext;
    vm_resortStack::tempStack[iVar9].pThis = pVVar4[iVar7 + -1].pThis;
    vm_resortStack::tempStack[iVar9].pPrev = pVVar4[iVar7 + -1].pPrev;
  }
  iVar5 = 0;
  if (0 < iVar6) {
    while( true ) {
      local_20[iVar10 + iVar5].type = vm_resortStack::tempStack[iVar10 + iVar5].type;
      vm_stack[vm_sp + iVar5].attrib = vm_resortStack::tempStack[vm_sp + iVar5].attrib;
      iVar10 = vm_sp;
      local_20 = vm_stack;
      iVar8 = vm_sp + iVar5;
      iVar5 = iVar5 + 1;
      vm_stack[iVar8].v = vm_resortStack::tempStack[iVar8].v;
      if (iVar6 <= iVar5) break;
    }
  }
  pVVar1->argNum = iVar6;
  pVVar1->bp = vm_sp;
  return;
}
```

## 新增：vm_undefineLocalVarOrArg — procedure_pseudo_code

Evidence ID：`ev_85c2d6b33bdc10ff3f0823cf5ee7d48b21344e4330b6079891f4861ff25eae85`

```c

void vm_undefineLocalVarOrArg(Vm_Fun_T *pFun,Vm_Fun_Arg_T *pVar)

{
  Vm_Fun_T *pVVar1;
  Vm_Ident_Info *ptr;
  ushort uVar2;
  Vm_Ident_Hash_Elem_T *ihe;
  uint uVar3;
  
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Ident_Info_T * pIi@[???] */
  if (pVar->name != (char *)0x0) {
    ihe = vm_lookupIdent(pVar->name);
    if ((ihe != (Vm_Ident_Hash_Elem_T *)0x0) && (pVVar1 = (ihe->ii).pInContext, pVVar1 == pFun)) {
      if (((ihe->ii).refered & 2) == 0) {
        yywarns("%s:%d: Unused variable(%s) in function(%s).\n",currentFile,currentLine,ihe->name,
                pVVar1->name);
      }
      ptr = (ihe->ii).pNext;
      if (ptr != (Vm_Ident_Info *)0x0) {
        ihe->token = ptr->oldToken;
        (ihe->ii).v.pInfo = (ptr->v).pInfo;
        *(undefined4 *)((int)&(ihe->ii).v + 4) = *(undefined4 *)((int)&ptr->v + 4);
        (ihe->ii).pInContext = ptr->pInContext;
        (ihe->ii).oldToken = ptr->oldToken;
        uVar2 = ptr->unused;
        (ihe->ii).refered = ptr->refered;
        (ihe->ii).unused = uVar2;
        (ihe->ii).pNext = ptr->pNext;
        pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_compiler.c",0x110);
      }
      uVar3 = ihe->semValue - 1;
      (ihe->dn).localNum = (ihe->dn).localNum + -1;
      ihe->semValue = uVar3;
      if (uVar3 == 0) {
        vm_freeIdent(ihe);
        return;
      }
    }
  }
  return;
}
```

## 新增：/DWARF/vm.h/Vm_Fun_Arg_T — inspect_native_data_type

Evidence ID：`ev_cf57bb557f0c7ff5c2de562aff56713955e79bf7b287d6e984758f39624ed688`

```json
{
  "address": null,
  "alignment_bytes": null,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": null,
  "kind": "unavailable",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": null,
  "packing_enabled": null,
  "reason": "No exact database type or defined typed data exists at the selected identity",
  "referenced_type": null,
  "size_bytes": null,
  "source": "analysis-database",
  "source_archive": null,
  "status": "unavailable",
  "total_fields": 0,
  "truncated": false
}
```

## 新增：/DWARF/vm_compiler.h/Vm_Fun_Arg_T — inspect_native_data_type

Evidence ID：`ev_a31ddd3f83ffb912741b7eb040d796c0e180fc2f999c0d4cb95f9688fb128730`

```json
{
  "address": null,
  "alignment_bytes": null,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": null,
  "kind": "unavailable",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": null,
  "packing_enabled": null,
  "reason": "No exact database type or defined typed data exists at the selected identity",
  "referenced_type": null,
  "size_bytes": null,
  "source": "analysis-database",
  "source_archive": null,
  "status": "unavailable",
  "total_fields": 0,
  "truncated": false
}
```

## 新增：/DWARF/vm_fun.h/Vm_Fun_Arg — inspect_native_data_type

Evidence ID：`ev_3a2145a4ec5bf063ceb5de533653f40d5ce70cd826ccb17d03b9892fc915df74`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "name",
      "offset_bytes": 0,
      "ordinal": 0,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/char *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "flag",
      "offset_bytes": 4,
      "ordinal": 1,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "type",
      "offset_bytes": 5,
      "ordinal": 2,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "varAttrib",
      "offset_bytes": 6,
      "ordinal": 3,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "no",
      "offset_bytes": 8,
      "ordinal": 4,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "defaultActionNum",
      "offset_bytes": 10,
      "ordinal": 5,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pDefaultValue",
      "offset_bytes": 12,
      "ordinal": 6,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_Ident_Unit/Vm_XNode *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pNext",
      "offset_bytes": 16,
      "ordinal": 7,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_fun.h/Vm_Fun_Arg *"
    }
  ],
  "id": "/DWARF/vm_fun.h/Vm_Fun_Arg",
  "kind": "struct",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Fun_Arg",
  "packing_enabled": true,
  "reason": null,
  "referenced_type": null,
  "size_bytes": 20,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 8,
  "truncated": false
}
```

## 新增：/DWARF/vm_fun.h/Vm_Fun_Arg_T — inspect_native_data_type

Evidence ID：`ev_81b2b91d8098c3d46320a56895d7a311cf3adfe16fef44f17543ac1f285cd0d2`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_fun.h/Vm_Fun_Arg_T",
  "kind": "typedef",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Fun_Arg_T",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_fun.h/Vm_Fun_Arg",
  "size_bytes": 20,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 新增：/DWARF/vm_fun.h/Vm_Fun_T — inspect_native_data_type

Evidence ID：`ev_9abc40d50269c7de5b2a54455cdf0a9e946644b3a8cf3c8720fc4b213d33707d`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_fun.h/Vm_Fun_T",
  "kind": "typedef",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Fun_T",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_lex.h/Vm_Ident_Unit/anon_struct_8_2_e648a62a_for_uf/Vm_Fun",
  "size_bytes": 64,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 复用：vm_callInUserFun — procedure_assembly

Evidence ID：`ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347`

```asm
0x8094080: PUSH EBP
0x8094081: MOV EBP, ESP
0x8094083: PUSH EDI
0x8094084: PUSH ESI
0x8094085: PUSH EBX
0x8094086: SUB ESP, 0xc
0x8094089: MOV EDX, dword ptr [EBP + 0xc]
0x809408c: MOV EAX, EDX
0x809408e: SHR EAX, 0x7
0x8094091: MOV EDI, EDX
0x8094093: AND EDI, 0x3f
0x8094096: AND EAX, 0x1
0x8094099: MOV ECX, dword ptr [EBP + 0x8]
0x809409c: MOV dword ptr [EBP + -0x10], EAX
0x809409f: JZ 0x080940ec
0x80940a1: TEST CX, CX
0x80940a4: JS 0x080940b4
0x80940a6: MOV EAX, [0x0847aad0]
0x80940ab: CMP word ptr [EAX + 0x10], CX
0x80940af: MOVSX EBX, CX
0x80940b2: JG 0x080940cd
0x80940b4: SUB ESP, 0x8
0x80940b7: MOVSX EBX, CX
0x80940ba: PUSH EBX
0x80940bb: PUSH 0x82ae3f0
0x80940c0: CALL 0x08097b24
0x80940c5: ADD ESP, 0x10
0x80940c8: MOV EAX, [0x0847aad0]
0x80940cd: MOV EAX, dword ptr [EAX + 0x2c]
0x80940d0: SUB ESP, 0xc
0x80940d3: MOV EAX, dword ptr [EAX + EBX*0x8]
0x80940d6: PUSH EDI
0x80940d7: PUSH EAX
0x80940d8: PUSH EBX
0x80940d9: PUSH 0x0
0x80940db: PUSH dword ptr [EBP + -0x10]
0x80940de: CALL 0x08093eb0
0x80940e3: LEA ESP, [EBP + -0xc]
0x80940e6: POP EBX
0x80940e7: POP ESI
0x80940e8: POP EDI
0x80940e9: LEAVE
0x80940ea: RET
0x80940ec: MOV EDX, dword ptr [0x086619d8]
0x80940f2: MOV EAX, dword ptr [EDX + 0x14]
0x80940f5: ADD CX, word ptr [EAX + 0x6]
0x80940f9: JS 0x08094150
0x80940fb: MOV ESI, dword ptr [EDX + 0x20]
0x80940fe: MOV EAX, dword ptr [ESI + 0x20]
0x8094101: CMP word ptr [EAX + 0x10], CX
0x8094105: MOVSX EBX, CX
0x8094108: JG 0x0809412b
0x809410a: PUSH EAX
0x809410b: MOV EAX, dword ptr [ESI + 0x20]
0x809410e: MOV EAX, dword ptr [EAX + 0x4]
0x8094111: PUSH dword ptr [EAX + 0xc]
0x8094114: MOVSX EBX, CX
0x8094117: PUSH EBX
0x8094118: PUSH 0x82ae420
0x809411d: CALL 0x08097b24
0x8094122: ADD ESP, 0x10
0x8094125: MOV EDX, dword ptr [0x086619d8]
0x809412b: MOV EAX, dword ptr [EDX + 0x20]
0x809412e: MOV EAX, dword ptr [EAX + 0x20]
0x8094131: MOV EAX, dword ptr [EAX + 0x2c]
0x8094134: SUB ESP, 0xc
0x8094137: MOV EAX, dword ptr [EAX + EBX*0x8]
0x809413a: PUSH EDI
0x809413b: PUSH EAX
0x809413c: PUSH EBX
0x809413d: PUSH 0x0
0x809413f: PUSH dword ptr [EBP + -0x10]
0x8094142: CALL 0x08093eb0
0x8094147: LEA ESP, [EBP + -0xc]
0x809414a: POP EBX
0x809414b: POP ESI
0x809414c: POP EDI
0x809414d: LEAVE
0x809414e: RET
0x8094150: MOV ESI, dword ptr [EDX + 0x20]
0x8094153: PUSH EAX
0x8094154: MOV EAX, dword ptr [ESI + 0x20]
0x8094157: MOV EAX, dword ptr [EAX + 0x4]
0x809415a: PUSH dword ptr [EAX + 0xc]
0x809415d: MOVSX EBX, CX
0x8094160: PUSH EBX
0x8094161: PUSH 0x82ae420
0x8094166: CALL 0x08097b24
0x809416b: ADD ESP, 0x10
0x809416e: MOV EDX, dword ptr [0x086619d8]
0x8094174: JMP 0x0809412b
```

## 复用：vm_callInUserFun — procedure_pseudo_code

Evidence ID：`ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97`

```c

void vm_callInUserFun(Vm_Index_T funOffset,int isGlobalTypeArgn)

{
  Vm_Program_T *pVVar1;
  uint isGlobal_00;
  short funOffset_00;
  int isGlobal;
  
                    /* Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg0(EAX)]
                       Unresolved local var: int type@[???]
                       Unresolved local var: uint argn@[DW_OP_reg7(EDI)] */
  isGlobal_00 = (uint)isGlobalTypeArgn >> 7 & 1;
  if (isGlobal_00 != 0) {
    if ((funOffset < 0) || (vm_pGlobalProgram->maxFunctionNo <= funOffset)) {
      vm_error("Global function (index = %d) is not existed!\n",(int)funOffset);
    }
    vm_callInUserFunBody
              (isGlobal_00,(Vm_Object_T *)0x0,funOffset,
               vm_pGlobalProgram->pFunctions[funOffset].pFun,isGlobalTypeArgn & 0x3fU);
    return;
  }
  funOffset_00 = funOffset + vm_currentContext->consititution->baseFunOffset;
  if (funOffset_00 < 0) {
    vm_error("Function (index = %d) in %s is not existed!\n",(int)funOffset_00,
             vm_currentContext->thisObject->pProgram->strName->ptr,vm_currentContext->consititution)
    ;
  }
  else {
    pVVar1 = vm_currentContext->thisObject->pProgram;
    if (pVVar1->maxFunctionNo <= funOffset_00) {
      vm_error("Function (index = %d) in %s is not existed!\n",(int)funOffset_00,
               vm_currentContext->thisObject->pProgram->strName->ptr,pVVar1);
    }
  }
  vm_callInUserFunBody
            (0,(Vm_Object_T *)0x0,funOffset_00,
             vm_currentContext->thisObject->pProgram->pFunctions[funOffset_00].pFun,
             isGlobalTypeArgn & 0x3fU);
  return;
}
```

## 复用：vm_callInUserFunBody — procedure_assembly

Evidence ID：`ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41`

```asm
0x8093eb0: PUSH EBP
0x8093eb1: MOV EBP, ESP
0x8093eb3: PUSH EDI
0x8093eb4: PUSH ESI
0x8093eb5: PUSH EBX
0x8093eb6: SUB ESP, 0xc
0x8093eb9: MOV EAX, dword ptr [EBP + 0x10]
0x8093ebc: MOV word ptr [EBP + -0xe], AX
0x8093ec0: MOV EAX, [0x086619c0]
0x8093ec5: CMP EAX, dword ptr [0x086619c4]
0x8093ecb: MOV ESI, dword ptr [EBP + 0xc]
0x8093ece: MOV EDI, dword ptr [EBP + 0x14]
0x8093ed1: JGE 0x08094020
0x8093ed7: TEST ESI, ESI
0x8093ed9: JZ 0x08093ee5
0x8093edb: TEST byte ptr [ESI + 0xd], 0x80
0x8093edf: JNZ 0x08094038
0x8093ee5: MOV EAX, dword ptr [EDI + 0x14]
0x8093ee8: TEST EAX, EAX
0x8093eea: JNZ 0x08093f18
0x8093eec: MOV EBX, dword ptr [EBP + 0x8]
0x8093eef: TEST EBX, EBX
0x8093ef1: JNZ 0x08094050
0x8093ef7: TEST ESI, ESI
0x8093ef9: JZ 0x08094070
0x8093eff: PUSH ECX
0x8093f00: PUSH dword ptr [EDI]
0x8093f02: MOV EAX, dword ptr [ESI + 0x20]
0x8093f05: MOV EAX, dword ptr [EAX + 0x4]
0x8093f08: PUSH dword ptr [EAX + 0xc]
0x8093f0b: PUSH 0x82ae33c
0x8093f10: CALL 0x08097b24
0x8093f15: ADD ESP, 0x10
0x8093f18: MOV EDX, dword ptr [0x0860ae64]
0x8093f1e: TEST EDX, EDX
0x8093f20: JNZ 0x08094014
0x8093f26: CALL 0x080a9310
0x8093f2b: MOV EBX, EAX
0x8093f2d: INC dword ptr [0x086619c0]
0x8093f33: CALL 0x0809789c
0x8093f38: MOV EDX, dword ptr [0x086619d8]
0x8093f3e: MOV EAX, [0x086619cc]
0x8093f43: MOV dword ptr [EDX], EAX
0x8093f45: MOV EAX, dword ptr [EBP + 0x18]
0x8093f48: MOV dword ptr [EDX + 0x8], 0x4
0x8093f4f: MOV dword ptr [EDX + 0x4], EAX
0x8093f52: MOV dword ptr [EDX + 0x24], EDI
0x8093f55: TEST byte ptr [EBX + 0x29], 0x1
0x8093f59: JZ 0x08093f62
0x8093f5b: MOV dword ptr [EDX + 0x8], 0xc
0x8093f62: TEST byte ptr [EDI + 0x8], 0x2
0x8093f66: JZ 0x08093f6f
0x8093f68: OR dword ptr [EBX + 0x28], 0x100
0x8093f6f: MOV EAX, dword ptr [EBP + 0x8]
0x8093f72: TEST EAX, EAX
0x8093f74: JZ 0x08093fb0
0x8093f76: MOV EBX, dword ptr [0x086619d8]
0x8093f7c: MOV dword ptr [EBX + 0x20], 0x0
0x8093f83: OR dword ptr [EBX + 0x8], 0x10
0x8093f87: MOV EAX, [0x0847aad0]
0x8093f8c: MOV EAX, dword ptr [EAX + 0x18]
0x8093f8f: MOV dword ptr [EBX + 0x14], EAX
0x8093f92: MOV EAX, [0x0860ae64]
0x8093f97: TEST EAX, EAX
0x8093f99: JNZ 0x08093ff4
0x8093f9b: LEA EAX, [EDI + 0x10]
0x8093f9e: MOV dword ptr [EBX + 0xc], 0x0
0x8093fa5: MOV dword ptr [EBX + 0x10], EAX
0x8093fa8: LEA ESP, [EBP + -0xc]
0x8093fab: POP EBX
0x8093fac: POP ESI
0x8093fad: POP EDI
0x8093fae: LEAVE
0x8093faf: RET
0x8093fb0: TEST ESI, ESI
0x8093fb2: JZ 0x0809405f
0x8093fb8: MOV EBX, dword ptr [0x086619d8]
0x8093fbe: MOV dword ptr [EBX + 0x20], ESI
0x8093fc1: OR dword ptr [EBX + 0x8], 0x10
0x8093fc5: MOV ESI, dword ptr [EBX + 0x20]
0x8093fc8: TEST ESI, ESI
0x8093fca: JZ 0x08093f87
0x8093fcc: MOV ECX, dword ptr [ESI + 0x20]
0x8093fcf: MOVSX EDX, word ptr [EBP + -0xe]
0x8093fd3: MOV EAX, dword ptr [ECX + 0x2c]
0x8093fd6: MOVSX EAX, word ptr [EAX + EDX*0x8 + 0x4]
0x8093fdb: LEA EAX, [EAX + EAX*0x2]
0x8093fde: MOV EDX, dword ptr [ECX + 0x18]
0x8093fe1: LEA EAX, [EDX + EAX*0x4]
0x8093fe4: MOV dword ptr [EBX + 0x14], EAX
0x8093fe7: MOV EAX, [0x0860ae64]
0x8093fec: INC dword ptr [ESI + 0x4]
0x8093fef: TEST EAX, EAX
0x8093ff1: JZ 0x08093f9b
0x8093ff3: NOP
0x8093ff4: CALL 0x080bb0e8
0x8093ff9: MOV EBX, dword ptr [0x086619d8]
0x8093fff: LEA EAX, [EDI + 0x10]
0x8094002: MOV dword ptr [EBX + 0xc], 0x0
0x8094009: MOV dword ptr [EBX + 0x10], EAX
0x809400c: LEA ESP, [EBP + -0xc]
0x809400f: POP EBX
0x8094010: POP ESI
0x8094011: POP EDI
0x8094012: LEAVE
0x8094013: RET
0x8094014: CALL 0x080bb0d4
0x8094019: JMP 0x08093f26
0x8094020: SUB ESP, 0x8
0x8094023: PUSH EAX
0x8094024: PUSH 0x82ae31c
0x8094029: CALL 0x08097b24
0x809402e: ADD ESP, 0x10
0x8094031: JMP 0x08093ed7
0x8094038: SUB ESP, 0xc
0x809403b: PUSH 0x82ae368
0x8094040: CALL 0x08097b24
0x8094045: ADD ESP, 0x10
0x8094048: JMP 0x08093ee5
0x8094050: SUB ESP, 0x8
0x8094053: PUSH dword ptr [EDI]
0x8094055: PUSH 0x82ae398
0x809405a: JMP 0x08093f10
0x809405f: MOV EBX, dword ptr [0x086619d8]
0x8094065: MOV EAX, dword ptr [EBX + -0x8]
0x8094068: MOV dword ptr [EBX + 0x20], EAX
0x809406b: JMP 0x08093fc5
0x8094070: SUB ESP, 0x8
0x8094073: PUSH dword ptr [EDI]
0x8094075: PUSH 0x82ae3c8
0x809407a: JMP 0x08093f10
```

## 复用：vm_callInUserFunBody — procedure_pseudo_code

Evidence ID：`ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`

```c

void vm_callInUserFunBody
               (int isGlobal,Vm_Object_T *pToObject,Vm_Index_T funOffset,Vm_Fun_T *pFun,int argn)

{
  Vm_Object *pVVar1;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  Vm_PCB_T *pVVar4;
  char *str;
  char *pcVar5;
  Vm_Index_T funOffset_local;
  
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg3(EBX)] */
  if (vm_funContextSize <= vm_funCp) {
    vm_error("Too depth (%d) function call!\n",vm_funCp);
  }
  if ((pToObject != (Vm_Object_T *)0x0) && ((pToObject->flags & 0x8000) != 0)) {
    vm_error("Object is not found (when call in function).\n");
  }
  if ((pFun->codeSegment).nodes == (Vm_Node_T *)0x0) {
    if (isGlobal == 0) {
      if (pToObject == (Vm_Object_T *)0x0) {
        pcVar5 = pFun->name;
        str = "Function \"%s\" is not implemented yet!\n";
      }
      else {
        pcVar5 = pToObject->pProgram->strName->ptr;
        str = "Function \"%s::%s\" is not implemented yet!\n";
      }
    }
    else {
      pcVar5 = pFun->name;
      str = "Global function \"%s\" is not implemented yet!\n";
    }
    vm_error(str,pcVar5);
  }
  if (vm_gatheringCallStackInfo != 0) {
    vm_enterPPCopyStackCriticalSection();
  }
  pVVar4 = vm_getCurrentPCB();
  vm_funCp = vm_funCp + 1;
  vm_updateCurrentFunCallContext();
  pVVar3 = vm_currentContext;
  vm_currentContext->bp = vm_sp;
  pVVar3->flags = 4;
  pVVar3->argNum = argn;
  (pVVar3->f).pEfun = (Vm_Efun *)pFun;
  if ((pVVar4->flags & 0x100) != 0) {
    pVVar3->flags = 0xc;
  }
  if ((pFun->attrib & 2) != 0) {
    pVVar4->flags = pVVar4->flags | 0x100;
  }
  pVVar3 = vm_currentContext;
  if (isGlobal == 0) {
    if (pToObject == (Vm_Object_T *)0x0) {
      vm_currentContext->thisObject = vm_currentContext[-1].thisObject;
    }
    else {
      vm_currentContext->thisObject = pToObject;
      pVVar3->flags = pVVar3->flags | 0x10;
    }
    pVVar1 = pVVar3->thisObject;
    if (pVVar1 != (Vm_Object *)0x0) {
                    /* Unresolved local var: Vm_Fun_Entry_T * pFunEntry@[???] */
      pVVar3->consititution =
           pVVar1->pProgram->pConsititutions +
           pVVar1->pProgram->pFunctions[funOffset].consititutionNo;
      iVar2 = vm_gatheringCallStackInfo;
      pVVar1->execRef = pVVar1->execRef + 1;
      goto joined_r0x08093ff1;
    }
  }
  else {
    vm_currentContext->thisObject = (Vm_Object *)0x0;
    pVVar3->flags = pVVar3->flags | 0x10;
  }
  pVVar3->consititution = vm_pGlobalProgram->pConsititutions;
  iVar2 = vm_gatheringCallStackInfo;
joined_r0x08093ff1:
  if (iVar2 == 0) {
    pVVar3->ip = 0;
    pVVar3->cs = &pFun->codeSegment;
    return;
  }
  vm_leavePPCopyStackCriticalSection();
  pVVar3 = vm_currentContext;
  vm_currentContext->ip = 0;
  pVVar3->cs = &pFun->codeSegment;
  return;
}
```

## 复用：vm_checkArgNum — procedure_assembly

Evidence ID：`ev_d17aa0d7a395baf8b760f4dd773008e55d6a7fc1e8c6931a9657f73663982946`

```asm
0x809521c: PUSH EBP
0x809521d: MOV EBP, ESP
0x809521f: PUSH EDI
0x8095220: PUSH ESI
0x8095221: PUSH EBX
0x8095222: SUB ESP, 0xc
0x8095225: MOV EAX, [0x086619c0]
0x809522a: LEA EAX, [EAX + EAX*0x4]
0x809522d: MOV EDX, dword ptr [0x086619c8]
0x8095233: LEA EAX, [EDX + EAX*0x8]
0x8095236: MOV dword ptr [EBP + -0x10], EAX
0x8095239: MOV EAX, dword ptr [EAX + 0x24]
0x809523c: MOV EBX, dword ptr [EAX + 0x30]
0x809523f: XOR ESI, ESI
0x8095241: TEST EBX, EBX
0x8095243: MOV dword ptr [EBP + -0x14], EAX
0x8095246: JZ 0x080952fa
0x809524c: MOV EDX, dword ptr [EBP + -0x10]
0x809524f: CMP dword ptr [EDX + 0x4], ESI
0x8095252: JLE 0x080952fa
0x8095258: MOV EAX, [0x086619cc]
0x809525d: ADD EAX, ESI
0x809525f: LEA EAX, [EAX + EAX*0x2]
0x8095262: MOV EDX, dword ptr [0x08661964]
0x8095268: TEST byte ptr [EBX + 0x4], 0x1
0x809526c: LEA EDI, [EDX + EAX*0x8]
0x809526f: JZ 0x08095344
0x8095275: LEA ECX, [ESI + 0x1]
0x8095278: MOV dword ptr [EBP + -0x18], ECX
0x809527b: CMP byte ptr [EDI + 0x4], 0x8
0x809527f: JZ 0x08095296
0x8095281: MOV EAX, dword ptr [EBP + -0x14]
0x8095284: PUSH dword ptr [EAX]
0x8095286: PUSH dword ptr [EBX]
0x8095288: PUSH ECX
0x8095289: PUSH 0x82ae72c
0x809528e: CALL 0x08097b24
0x8095293: ADD ESP, 0x10
0x8095296: CMP byte ptr [EBX + 0x5], 0xff
0x809529a: JZ 0x080952ec
0x809529c: SUB ESP, 0xc
0x809529f: PUSH EDI
0x80952a0: CALL 0x0808cb8c
0x80952a5: MOV EDX, EAX
0x80952a7: MOVZX EAX, byte ptr [EBX + 0x5]
0x80952ab: ADD ESP, 0x10
0x80952ae: CMP EDX, EAX
0x80952b0: JZ 0x080952ec
0x80952b2: TEST EDX, EDX
0x80952b4: JNZ 0x080952bc
0x80952b6: TEST byte ptr [EBX + 0x6], 0x8
0x80952ba: JNZ 0x080952ec
0x80952bc: SUB ESP, 0xc
0x80952bf: PUSH EDX
0x80952c0: CALL 0x0808a13c
0x80952c5: POP ECX
0x80952c6: POP ESI
0x80952c7: PUSH EAX
0x80952c8: MOVZX EAX, byte ptr [EBX + 0x5]
0x80952cc: PUSH EAX
0x80952cd: CALL 0x0808a13c
0x80952d2: MOV dword ptr [ESP], EAX
0x80952d5: MOV EDX, dword ptr [EBP + -0x14]
0x80952d8: PUSH dword ptr [EDX]
0x80952da: PUSH dword ptr [EBX]
0x80952dc: PUSH dword ptr [EBP + -0x18]
0x80952df: PUSH 0x82ae778
0x80952e4: CALL 0x08097b24
0x80952e9: ADD ESP, 0x20
0x80952ec: MOV ESI, dword ptr [EBP + -0x18]
0x80952ef: MOV EBX, dword ptr [EBX + 0x10]
0x80952f2: TEST EBX, EBX
0x80952f4: JNZ 0x0809524c
0x80952fa: MOV EDX, dword ptr [EBP + -0x14]
0x80952fd: MOVSX EAX, byte ptr [EDX + 0xf]
0x8095301: CMP EAX, ESI
0x8095303: JG 0x0809543c
0x8095309: MOV ECX, dword ptr [EBP + -0x14]
0x809530c: MOVSX EAX, byte ptr [ECX + 0xe]
0x8095310: MOV ECX, dword ptr [EBP + -0x10]
0x8095313: MOV EDX, dword ptr [ECX + 0x4]
0x8095316: CMP EDX, EAX
0x8095318: JL 0x080953ae
0x809531e: CMP EDX, ESI
0x8095320: JLE 0x0809532f
0x8095322: MOV EAX, dword ptr [EBP + -0x14]
0x8095325: TEST byte ptr [EAX + 0x8], 0x1
0x8095329: JZ 0x08095490
0x809532f: MOV EDX, dword ptr [0x086619d8]
0x8095335: MOV EAX, dword ptr [EBP + 0x8]
0x8095338: ADD dword ptr [EDX + 0xc], EAX
0x809533b: LEA ESP, [EBP + -0xc]
0x809533e: POP EBX
0x809533f: POP ESI
0x8095340: POP EDI
0x8095341: LEAVE
0x8095342: RET
0x8095344: TEST byte ptr [EBX + 0x6], 0x2
0x8095348: JNZ 0x080953a0
0x809534a: MOV AL, byte ptr [EBX + 0x5]
0x809534d: LEA ECX, [ESI + 0x1]
0x8095350: CMP AL, 0xff
0x8095352: MOV dword ptr [EBP + -0x18], ECX
0x8095355: JZ 0x080952ec
0x8095357: MOV DL, byte ptr [EDI + 0x4]
0x809535a: CMP DL, 0x8
0x809535d: JZ 0x08095456
0x8095363: CMP DL, AL
0x8095365: JZ 0x080953e4
0x8095367: CMP AL, 0x3
0x8095369: JZ 0x08095420
0x809536f: CMP AL, 0x2
0x8095371: JZ 0x08095476
0x8095377: SUB ESP, 0xc
0x809537a: PUSH EDI
0x809537b: CALL 0x0808cb3c
0x8095380: ADD ESP, 0x10
0x8095383: TEST EAX, EAX
0x8095385: JNZ 0x08095391
0x8095387: TEST byte ptr [EBX + 0x6], 0x8
0x809538b: JNZ 0x080952ec
0x8095391: SUB ESP, 0xc
0x8095394: MOVZX EAX, byte ptr [EDI + 0x4]
0x8095398: PUSH EAX
0x8095399: JMP 0x080952c0
0x80953a0: SUB ESP, 0xc
0x80953a3: PUSH EDI
0x80953a4: CALL 0x0808cc18
0x80953a9: ADD ESP, 0x10
0x80953ac: JMP 0x0809534a
0x80953ae: MOV ECX, dword ptr [EBP + -0x14]
0x80953b1: MOV EBX, dword ptr [ECX + 0x30]
0x80953b4: XOR ECX, ECX
0x80953b6: TEST EBX, EBX
0x80953b8: JZ 0x080953d4
0x80953ba: MOV ESI, ESI
0x80953bc: MOVSX EAX, word ptr [EBX + 0x8]
0x80953c0: CMP EAX, EDX
0x80953c2: JGE 0x080953d4
0x80953c4: MOVSX EAX, word ptr [EBX + 0xa]
0x80953c8: MOV EBX, dword ptr [EBX + 0x10]
0x80953cb: ADD ECX, EAX
0x80953cd: TEST EBX, EBX
0x80953cf: JNZ 0x080953bc
0x80953d1: LEA ESI, [ESI]
0x80953d4: MOV EAX, [0x086619d8]
0x80953d9: ADD dword ptr [EAX + 0xc], ECX
0x80953dc: LEA ESP, [EBP + -0xc]
0x80953df: POP EBX
0x80953e0: POP ESI
0x80953e1: POP EDI
0x80953e2: LEAVE
0x80953e3: RET
0x80953e4: CMP DL, 0xa
0x80953e7: JNZ 0x080952ec
0x80953ed: MOV EAX, dword ptr [EDI + 0x8]
0x80953f0: TEST byte ptr [EAX + 0xd], 0x80
0x80953f4: JZ 0x080952ec
0x80953fa: TEST byte ptr [EBX + 0x6], 0x8
0x80953fe: JNZ 0x080952ec
0x8095404: MOV ECX, dword ptr [EBP + -0x14]
0x8095407: PUSH dword ptr [ECX]
0x8095409: PUSH dword ptr [EBX]
0x809540b: PUSH dword ptr [EBP + -0x18]
0x809540e: PUSH 0x82ae7bc
0x8095413: CALL 0x08097b24
0x8095418: ADD ESP, 0x10
0x809541b: JMP 0x080952ec
0x8095420: CMP DL, 0x2
0x8095423: JNZ 0x08095377
0x8095429: CVTSI2SS XMM0, dword ptr [EDI + 0x8]
0x809542e: MOVSS dword ptr [EDI + 0x8], XMM0
0x8095433: MOV byte ptr [EDI + 0x4], 0x3
0x8095437: JMP 0x080952ec
0x809543c: PUSH dword ptr [EDX]
0x809543e: PUSH dword ptr [EBX]
0x8095440: LEA EAX, [ESI + 0x1]
0x8095443: PUSH EAX
0x8095444: PUSH 0x82ae820
0x8095449: CALL 0x08097b24
0x809544e: ADD ESP, 0x10
0x8095451: JMP 0x08095309
0x8095456: MOV EAX, dword ptr [EBP + -0x14]
0x8095459: PUSH dword ptr [EAX]
0x809545b: PUSH dword ptr [EBX]
0x809545d: PUSH ECX
0x809545e: PUSH 0x82ae848
0x8095463: CALL 0x08097b24
0x8095468: ADD ESP, 0x10
0x809546b: MOV DL, byte ptr [EDI + 0x4]
0x809546e: MOV AL, byte ptr [EBX + 0x5]
0x8095471: JMP 0x08095363
0x8095476: CMP DL, 0x3
0x8095479: JNZ 0x08095377
0x809547f: CVTTSS2SI EAX, dword ptr [EDI + 0x8]
0x8095484: MOV dword ptr [EDI + 0x8], EAX
0x8095487: MOV byte ptr [EDI + 0x4], 0x2
0x809548b: JMP 0x080952ec
0x8095490: PUSH EDX
0x8095491: PUSH dword ptr [EAX]
0x8095493: PUSH dword ptr [ECX + 0x4]
0x8095496: PUSH 0x82ae87c
0x809549b: CALL 0x08097b24
0x80954a0: ADD ESP, 0x10
0x80954a3: JMP 0x0809532f
```

## 复用：vm_checkArgNum — procedure_pseudo_code

Evidence ID：`ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9`

```c

void vm_checkArgNum(int nTotalAction)

{
  Vm_Value_T *pValue;
  Vm_Fun *pVVar1;
  Vm_Fun_Arg_T *pVVar2;
  int iVar3;
  Vm_FunCall_Context_T *pVVar4;
  uchar uVar5;
  uint type;
  char *pcVar6;
  char *pcVar7;
  int iVar8;
  int iVar9;
  uchar uVar10;
  int local_1c;
  Vm_Fun_T *pFun;
  Vm_FunCall_Context_T *pContext;
  
  pVVar4 = vm_funContext;
                    /* Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg7(EDI)]
                       Unresolved local var: int nActionNodes@[DW_OP_reg1(ECX)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)]
                       Unresolved local var: int gotType@[DW_OP_reg2(EDX)] */
  iVar3 = vm_funCp;
  pVVar1 = vm_funContext[vm_funCp].f.pFun;
  pVVar2 = pVVar1->pArgs;
  iVar9 = 0;
  do {
    if ((pVVar2 == (Vm_Fun_Arg_T *)0x0) || (pVVar4[iVar3].argNum <= iVar9)) {
      if (iVar9 < pVVar1->minArgNo) {
        vm_error("No input argument %d for \"%s\" of %s.\n",iVar9 + 1,pVVar2->name,pVVar1->name);
      }
      iVar8 = pVVar4[iVar3].argNum;
      if (iVar8 < pVVar1->maxArgNo) {
        iVar9 = 0;
        for (pVVar2 = pVVar1->pArgs; (pVVar2 != (Vm_Fun_Arg_T *)0x0 && (pVVar2->no < iVar8));
            pVVar2 = pVVar2->pNext) {
          iVar9 = iVar9 + pVVar2->defaultActionNum;
        }
        vm_currentContext->ip = vm_currentContext->ip + iVar9;
        return;
      }
      if ((iVar9 < iVar8) && ((pVVar1->attrib & 1) == 0)) {
        vm_error("Too many arguments (%d) passed to the function %s.\n",pVVar4[iVar3].argNum,
                 pVVar1->name,iVar8);
      }
      vm_currentContext->ip = vm_currentContext->ip + nTotalAction;
      return;
    }
    pValue = vm_stack + vm_sp + iVar9;
    if ((pVVar2->flag & 1) == 0) {
      if ((pVVar2->varAttrib & 2) != 0) {
        vm_convertValueToConst(pValue);
      }
      uVar5 = pVVar2->type;
      if (uVar5 != 0xff) {
        uVar10 = pValue->type;
        if (uVar10 == '\b') {
          vm_error("Argument %d for \"%s\" of %s got a reference value.\n",iVar9 + 1,pVVar2->name,
                   pVVar1->name);
          uVar10 = pValue->type;
          uVar5 = pVVar2->type;
        }
        local_1c = iVar9 + 1;
        if (uVar10 == uVar5) {
          if (((uVar10 == '\n') && (((uint)((pValue->v).string)->ptr & 0x8000) != 0)) &&
             ((pVVar2->varAttrib & 8) == 0)) {
            vm_error("Type mismatched for argument %d \"%s\" of %s.\nExpected object, got undefined (object is destructed).\n"
                     ,local_1c,pVVar2->name,pVVar1->name);
          }
        }
        else if (uVar5 == '\x03') {
          if (uVar10 != '\x02') goto LAB_08095377;
          (pValue->v).real = (float)(pValue->v).number;
          pValue->type = '\x03';
        }
        else if ((uVar5 == '\x02') && (uVar10 == '\x03')) {
          (pValue->v).number = (int)(pValue->v).real;
          pValue->type = '\x02';
        }
        else {
LAB_08095377:
          iVar8 = vm_getVarType(pValue);
          if ((iVar8 != 0) || ((pVVar2->varAttrib & 8) == 0)) {
            type = (uint)pValue->type;
            goto LAB_080952c0;
          }
        }
      }
    }
    else {
      if (pValue->type != '\b') {
        vm_error("Argument %d for \"%s\" of %s need reference type, but got a normal value.\n",
                 iVar9 + 1,pVVar2->name,pVVar1->name);
      }
      if (((pVVar2->type != 0xff) && (type = vm_getValueType(pValue), type != pVVar2->type)) &&
         ((type != 0 || ((pVVar2->varAttrib & 8) == 0)))) {
LAB_080952c0:
        local_1c = iVar9 + 1;
        pcVar6 = vm_valueType(type);
        pcVar7 = vm_valueType((uint)pVVar2->type);
        vm_error("Type mismatched for argument %d \"%s\" of %s.\nExpected %s, got %s.\n",local_1c,
                 pVVar2->name,pVVar1->name,pcVar7,pcVar6);
      }
    }
    local_1c = iVar9 + 1;
    pVVar2 = pVVar2->pNext;
    iVar9 = local_1c;
  } while( true );
}
```

## 复用：vm_doActions — procedure_pseudo_code

Evidence ID：`ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f`

```c

int vm_doActions(Vm_PCB *pPCB,Vm_Tick_T n)

{
  Vm_Catch_Info_T *pVVar1;
  Vm_Tick_T VVar2;
  int iVar3;
  Vm_Catch_Buf *pVVar4;
  int iVar5;
  int actionState;
  Vm_Catch_Info_T *pCatch;
  
                    /* Unresolved local var: int funCp@[DW_OP_reg2(EDX)]
                       Unresolved local var: int errCode@[DW_OP_reg0(EAX)] */
  totalTicks = 10000;
  if (n != 0) {
    totalTicks = n;
  }
  VVar2 = totalTicks;
  if ((pPCB->flags & 0x200000) != 0) {
    iVar5 = vm_getCostTimeCheck();
    VVar2 = 120000;
    if (iVar5 != 0) {
      VVar2 = totalTicks;
    }
  }
  totalTicks = VVar2;
  _vm_startTimer(0,totalTicks);
LAB_08097040:
  iVar3 = pPCB->catchCp;
  iVar5 = 0;
  if (iVar3 < 1) goto LAB_08097066;
  do {
    iVar5 = pPCB->catchContext[iVar3 + -1].funCp;
LAB_08097066:
    pVVar4 = vm_newCatch(iVar5,1);
    iVar5 = _setjmp((__jmp_buf_tag *)pVVar4);
    while( true ) {
      if (iVar5 == 0) {
        if ((pPCB->flags & 0x40) != 0) {
          pPCB->flags = pPCB->flags & 0xffffffbf;
          vm_returnFromEfun();
        }
        if ((vm_confEnableJit == 0) && (600000 < pPCB->holdTicks)) {
          iVar5 = vm_getCostTimeCheck();
          if (iVar5 == 0) {
            pPCB->holdTicks = 0;
          }
          else {
            vm_error("Non-delay routine (primitive) cost too much time.\n");
          }
        }
        actionState = -0x1fab;
        if (vm_ctrlFlags == 0) goto joined_r0x0809717a;
        if ((vm_ctrlFlags & 0x10) != 0) {
          vm_ctrlFlags = vm_ctrlFlags & 0xffffffef;
          actionState = 0;
          if (vm_ctrlFlags == 0) goto joined_r0x0809717a;
        }
        if ((vm_ctrlFlags & 2) == 0) {
          if ((vm_ctrlFlags & 4) == 0) {
            if ((vm_ctrlFlags & 8) == 0) {
              if ((vm_ctrlFlags & 0x20) == 0) {
                if ((vm_ctrlFlags & 0x40) == 0) {
                  actionState = -0x1fa6;
                  if ((vm_ctrlFlags & 0x80) != 0) {
                    vm_ctrlFlags = vm_ctrlFlags & 0xffffff7f;
                    actionState = -0x1fac;
                  }
                }
                else {
                  vm_ctrlFlags = vm_ctrlFlags & 0xffffffbf;
                  actionState = -0x1fa5;
                }
              }
              else {
                vm_ctrlFlags = vm_ctrlFlags & 0xffffffdf;
                actionState = -0x1fab;
              }
            }
            else {
              vm_ctrlFlags = vm_ctrlFlags & 0xfffffff7;
              actionState = -0x1fa6;
            }
          }
          else {
            vm_ctrlFlags = vm_ctrlFlags & 0xfffffffb;
            actionState = -0x1fa8;
          }
        }
        else {
          vm_ctrlFlags = vm_ctrlFlags & 0xfffffffd;
          actionState = -0x1fa7;
        }
        goto joined_r0x0809717a;
      }
      if (iVar5 != -3) {
        return iVar5;
      }
      if (pPCB->catchCp < 1) {
        return -3;
      }
      iVar5 = pPCB->catchCp + -1;
      pPCB->catchCp = iVar5;
      pVVar1 = pPCB->catchContext + iVar5;
      if (vm_sp < pVVar1->sp) {
        vm_popNStack(pVVar1->sp - vm_sp);
        iVar5 = pVVar1->catchIp;
      }
      else {
        iVar5 = pVVar1->catchIp;
      }
      if (iVar5 == -1) {
        iVar5 = 0;
        if (0 < pPCB->catchCp) {
          iVar5 = pPCB->catchContext[pPCB->catchCp + -1].funCp;
        }
        pVVar4 = vm_newCatch(iVar5,1);
        iVar5 = _setjmp((__jmp_buf_tag *)pVVar4);
        if (iVar5 != 0) {
          return 0;
        }
        (*pVVar1->catchCallback)(pVVar1->pCookie);
        vm_endCatch();
        goto LAB_08097040;
      }
      vm_currentContext->ip = iVar5;
      iVar3 = pPCB->catchCp;
      if (0 < iVar3) break;
      pVVar4 = vm_newCatch(0,1);
      iVar5 = _setjmp((__jmp_buf_tag *)pVVar4);
    }
  } while( true );
joined_r0x0809717a:
  if (actionState != -0x1fab) {
LAB_080971b0:
    if (((((pPCB->flags & 0x200000) != 0) && (-1 < (int)pPCB->flags)) && (actionState != -0x1fa5))
       && (actionState != -0x1fa6)) {
      vm_error("Non-delay routine (callback) cost too much time.\n");
    }
    vm_endCatch();
    return actionState;
  }
  if ((vm_currentContext->ip < 0) || (vm_currentContext->cs->nodesCount <= vm_currentContext->ip)) {
    actionState = -0x1fa6;
    goto LAB_080971b0;
  }
  actionState = (*vm_processor)(pPCB);
  goto joined_r0x0809717a;
}
```

## 复用：vm_enterFunction — procedure_pseudo_code

Evidence ID：`ev_1782e4cf62d7942391d37161d43a2051ad3aa963b13b7ca61519cc59ae7dad43`

```c

/* WARNING: Unknown calling convention */

void vm_enterFunction(int minArgNum)

{
  Vm_Fun *pVVar1;
  short sVar2;
  int iVar3;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg6(ESI)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_funContext[vm_funCp].f.pFun;
  vm_currentContext->argNum = vm_funContext[vm_funCp].argNum;
  sVar2 = pVVar1->localVariables;
  if (vm_sp < sVar2) {
    vm_error("Stack overflow when calling %s.\n",pVVar1->name);
    sVar2 = pVVar1->localVariables;
  }
  iVar3 = 0;
  if (0 < sVar2) {
    do {
      vm_pushUndefined();
      iVar3 = iVar3 + 1;
    } while (iVar3 < pVVar1->localVariables);
  }
  return;
}
```

## 复用：vm_errorQuit — procedure_assembly

Evidence ID：`ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2`

```asm
0x80ab104: PUSH EBP
0x80ab105: MOV EBP, ESP
0x80ab107: PUSH EBX
0x80ab108: PUSH EAX
0x80ab109: CALL 0x080a3678
0x80ab10e: MOV EBX, EAX
0x80ab110: CALL 0x080a437c
0x80ab115: CMP EAX, dword ptr [0x086078e0]
0x80ab11b: JZ 0x080ab14c
0x80ab11d: CMP dword ptr [EBX], 0x1
0x80ab120: JZ 0x080ab158
0x80ab122: CALL 0x080a3678
0x80ab127: SUB ESP, 0x8
0x80ab12a: DEC dword ptr [EAX + 0xa48]
0x80ab130: PUSH dword ptr [EBP + 0x8]
0x80ab133: MOV EDX, dword ptr [EAX + 0xa48]
0x80ab139: LEA ECX, [EDX + EDX*0x4]
0x80ab13c: LEA ECX, [EDX + ECX*0x8]
0x80ab13f: LEA ECX, [EAX + ECX*0x4 + 0x8]
0x80ab143: PUSH ECX
0x80ab144: CALL 0x0804b574
0x80ab14c: CALL 0x080ca414
0x80ab151: CMP dword ptr [EBX], 0x1
0x80ab154: JNZ 0x080ab122
0x80ab156: MOV ESI, ESI
0x80ab158: SUB ESP, 0x8
0x80ab15b: PUSH 0x0
0x80ab15d: MOV EAX, dword ptr [EBX + 0xa48]
0x80ab163: LEA EDX, [EAX + EAX*0x4]
0x80ab166: LEA EDX, [EAX + EDX*0x8]
0x80ab169: PUSH dword ptr [EBX + EDX*0x4]
0x80ab16c: CALL 0x08093520
0x80ab171: ADD ESP, 0x10
0x80ab174: JMP 0x080ab122
```

## 复用：vm_errorQuit — procedure_pseudo_code

Evidence ID：`ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1`

```c

/* WARNING: Unknown calling convention */

void vm_errorQuit(int n)

{
  int iVar1;
  Vm_Thread_Info_T *pVVar2;
  Vm_Task_Id_T VVar3;
  
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg3(EBX)] */
  pVVar2 = vm_getCurrentThreadInfo();
  VVar3 = _vm_getCurrentTaskId();
  if (VVar3 == _vm_mainTaskId) {
    vm_updateVmIpByLastJitFrame();
    iVar1 = pVVar2->threadType;
  }
  else {
    iVar1 = pVVar2->threadType;
  }
  if (iVar1 == 1) {
    vm_traceError(pVVar2->topEnv[pVVar2->topCatch + -1].funCp,(FILE *)0x0);
  }
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg0(EAX)] */
  pVVar2 = vm_getCurrentThreadInfo();
  pVVar2->topCatch = pVVar2->topCatch + -1;
                    /* WARNING: Subroutine does not return */
  longjmp((__jmp_buf_tag *)(pVVar2->topEnv + pVVar2->topCatch),n);
}
```

## 复用：vm_loadFun — analyze_function

Evidence ID：`ev_b513e6242f5447517c22636c5c877a93b397b974851b7928f195106d957a1079`

```c

int vm_loadFun(Vm_Binary_Buffer_T *pInput,Vm_Fun_T **ppFun)

{
  Vm_Fun_Arg_T *pVVar1;
  Vm_Fun_T *pVVar2;
  int iVar3;
  Vm_Fun_Arg_T **ppVVar4;
  Vm_Code_Segment_T *pVVar5;
  char *pcVar6;
  Vm_Index_T n;
  Vm_Fun_T *pFun;
  
                    /* Unresolved local var: Vm_Fun_Arg_T * * ppArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * * ppVar@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pVar@[DW_OP_reg3(EBX)] */
  iVar3 = vm_loadBuffer(pInput,&pFun,0x40);
  if (iVar3 == 0) {
    pcVar6 = "Load function failed: structure error.\n";
    goto LAB_080959df;
  }
  pFun->name = (char *)0x0;
  pFun->pArgs = (Vm_Fun_Arg_T *)0x0;
  pFun->pLabels = (Vm_Local_Label_T *)0x0;
  pFun->pNext = (Vm_Fun *)0x0;
  pFun->pVars = (Vm_Fun_Arg_T *)0x0;
  pVVar5 = &pFun->codeSegment;
  for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
    pVVar5->nodesCount = 0;
    pVVar5 = (Vm_Code_Segment_T *)&pVVar5->nodes;
  }
  iVar3 = vm_isKeepSymbol();
  if (iVar3 == 0) {
    iVar3 = vm_loadTempString(pInput,(char **)0x0);
    pVVar2 = pFun;
    if (iVar3 != 0) {
      pcVar6 = vm_scratchLargeAlloc(4);
      pVVar2->name = pcVar6;
      *pFun->name = '?';
      pFun->name[1] = '?';
      pFun->name[2] = '?';
      pFun->name[3] = '\0';
      iVar3 = vm_loadComment(pInput,"ARGUMENTS");
joined_r0x08095a6e:
      if (iVar3 == 0) {
        pcVar6 = "Load function failed: comment \"ARGUMENTS\" mismatched.\n";
      }
      else {
        iVar3 = vm_loadVmIndex(pInput,&n);
        if (iVar3 == 0) {
          pcVar6 = "Load function failed: argument count error.\n";
        }
        else {
          ppVVar4 = &pFun->pArgs;
          while (0 < n) {
            n = n + -1;
            iVar3 = vm_loadBuffer(pInput,ppVVar4,0x14);
            if (iVar3 == 0) {
              pcVar6 = "Load function failed: argument structure error.\n";
              goto LAB_080959df;
            }
            pVVar1 = *ppVVar4;
            pVVar1->name = (char *)0x0;
            pVVar1->pDefaultValue = (Vm_XNode *)0x0;
            pVVar1->pNext = (Vm_Fun_Arg *)0x0;
            iVar3 = vm_loadString(pInput,&pVVar1->name);
            if (iVar3 == 0) {
              pcVar6 = "Load function failed: argument name error.\n";
              goto LAB_080959df;
            }
            ppVVar4 = &pVVar1->pNext;
          }
          n = n + -1;
          iVar3 = vm_loadVmIndex(pInput,&n);
          if (iVar3 == 0) {
            pcVar6 = "Load function failed: local variables count error.\n";
          }
          else {
            ppVVar4 = &pFun->pVars;
            while (0 < n) {
              n = n + -1;
              iVar3 = vm_loadBuffer(pInput,ppVVar4,0x14);
              if (iVar3 == 0) {
                pcVar6 = "Load function failed: Local variable structure error.\n";
                goto LAB_080959df;
              }
              pVVar1 = *ppVVar4;
              pVVar1->name = (char *)0x0;
              pVVar1->pDefaultValue = (Vm_XNode *)0x0;
              pVVar1->pNext = (Vm_Fun_Arg *)0x0;
              iVar3 = vm_loadString(pInput,&pVVar1->name);
              if (iVar3 == 0) {
                pcVar6 = "Load function failed: Varument name error.\n";
                goto LAB_080959df;
              }
              ppVVar4 = &pVVar1->pNext;
            }
            n = n + -1;
            iVar3 = vm_isKeepSymbol();
            if (iVar3 == 0) {
              while (pVVar1 = pFun->pVars, pVVar1 != (Vm_Fun_Arg_T *)0x0) {
                pFun->pVars = pVVar1->pNext;
                vm_scratchFree(pVVar1->name);
                vm_scratchFree(pVVar1);
              }
            }
            iVar3 = vm_loadComment(pInput,"ACTIONS");
            if (iVar3 == 0) {
              pcVar6 = "Load function failed: comment \"ACTIONS\" mismatched.\n";
            }
            else {
              iVar3 = vm_loadCodeSegment(pInput,pFun,&pFun->codeSegment);
              if (iVar3 != 0) {
                *ppFun = pFun;
                return 1;
              }
              pcVar6 = "Load function failed: actions error.\n";
            }
          }
        }
      }
      goto LAB_080959df;
    }
  }
  else {
    iVar3 = vm_loadString(pInput,&pFun->name);
    if (iVar3 != 0) {
      iVar3 = vm_loadComment(pInput,"ARGUMENTS");
      goto joined_r0x08095a6e;
    }
  }
  pcVar6 = "Load function failed: name error.\n";
LAB_080959df:
  vm_errprintf(pcVar6);
  return 0;
}
```

```asm
0x80959b8: PUSH EBP
0x80959b9: MOV EBP, ESP
0x80959bb: PUSH EDI
0x80959bc: PUSH ESI
0x80959bd: PUSH EBX
0x80959be: SUB ESP, 0x10
0x80959c1: PUSH 0x40
0x80959c3: LEA EAX, [EBP + -0x10]
0x80959c6: PUSH EAX
0x80959c7: MOV ESI, dword ptr [EBP + 0x8]
0x80959ca: PUSH ESI
0x80959cb: CALL 0x080b8a70
0x80959d0: ADD ESP, 0x10
0x80959d3: TEST EAX, EAX
0x80959d5: JNZ 0x080959f0
0x80959d7: SUB ESP, 0xc
0x80959da: PUSH 0x82ae94c
0x80959df: CALL 0x08097acc
0x80959e4: XOR EAX, EAX
0x80959e6: LEA ESP, [EBP + -0xc]
0x80959e9: POP EBX
0x80959ea: POP ESI
0x80959eb: POP EDI
0x80959ec: LEAVE
0x80959ed: RET
0x80959f0: MOV EDI, dword ptr [EBP + -0x10]
0x80959f3: MOV dword ptr [EDI], 0x0
0x80959f9: MOV dword ptr [EDI + 0x30], 0x0
0x8095a00: MOV dword ptr [EDI + 0x38], 0x0
0x8095a07: MOV dword ptr [EDI + 0x3c], 0x0
0x8095a0e: MOV dword ptr [EDI + 0x34], 0x0
0x8095a15: XOR EAX, EAX
0x8095a17: ADD EDI, 0x10
0x8095a1a: CLD
0x8095a1b: MOV ECX, 0x8
0x8095a20: STOSD.REP ES:EDI
0x8095a22: CALL 0x080b5bd0
0x8095a27: TEST EAX, EAX
0x8095a29: JNZ 0x08095a48
0x8095a2b: SUB ESP, 0x8
0x8095a2e: PUSH 0x0
0x8095a30: PUSH ESI
0x8095a31: CALL 0x080b8c4c
0x8095a36: ADD ESP, 0x10
0x8095a39: TEST EAX, EAX
0x8095a3b: JNZ 0x08095a80
0x8095a3d: SUB ESP, 0xc
0x8095a40: PUSH 0x82ae974
0x8095a45: JMP 0x080959df
0x8095a48: SUB ESP, 0x8
0x8095a4b: PUSH dword ptr [EBP + -0x10]
0x8095a4e: PUSH ESI
0x8095a4f: CALL 0x080b8dc8
0x8095a54: ADD ESP, 0x10
0x8095a57: TEST EAX, EAX
0x8095a59: JZ 0x08095a3d
0x8095a5b: SUB ESP, 0x8
0x8095a5e: PUSH 0x82ae15e
0x8095a63: PUSH ESI
0x8095a64: CALL 0x080b8800
0x8095a69: ADD ESP, 0x10
0x8095a6c: TEST EAX, EAX
0x8095a6e: JNZ 0x08095aca
0x8095a70: SUB ESP, 0xc
0x8095a73: PUSH 0x82ae998
0x8095a78: JMP 0x080959df
0x8095a80: SUB ESP, 0xc
0x8095a83: PUSH 0x4
0x8095a85: MOV EBX, dword ptr [EBP + -0x10]
0x8095a88: CALL 0x08089630
0x8095a8d: MOV dword ptr [EBX], EAX
0x8095a8f: MOV EAX, dword ptr [EBP + -0x10]
0x8095a92: MOV EAX, dword ptr [EAX]
0x8095a94: MOV byte ptr [EAX], 0x3f
0x8095a97: MOV EAX, dword ptr [EBP + -0x10]
0x8095a9a: MOV EAX, dword ptr [EAX]
0x8095a9c: MOV byte ptr [EAX + 0x1], 0x3f
0x8095aa0: MOV EAX, dword ptr [EBP + -0x10]
0x8095aa3: MOV EAX, dword ptr [EAX]
0x8095aa5: MOV byte ptr [EAX + 0x2], 0x3f
0x8095aa9: MOV EAX, dword ptr [EBP + -0x10]
0x8095aac: ADD ESP, 0x10
0x8095aaf: SUB ESP, 0x8
0x8095ab2: MOV EAX, dword ptr [EAX]
0x8095ab4: MOV byte ptr [EAX + 0x3], 0x0
0x8095ab8: PUSH 0x82ae15e
0x8095abd: PUSH ESI
0x8095abe: CALL 0x080b8800
0x8095ac3: ADD ESP, 0x10
0x8095ac6: TEST EAX, EAX
0x8095ac8: JZ 0x08095a70
0x8095aca: SUB ESP, 0x8
0x8095acd: LEA EDI, [EBP + -0x12]
0x8095ad0: PUSH EDI
0x8095ad1: PUSH ESI
0x8095ad2: CALL 0x080b8b44
0x8095ad7: ADD ESP, 0x10
0x8095ada: TEST EAX, EAX
0x8095adc: JZ 0x08095c10
0x8095ae2: MOV EBX, dword ptr [EBP + -0x10]
0x8095ae5: ADD EBX, 0x30
0x8095ae8: JMP 0x08095b2f
0x8095aec: PUSH EAX
0x8095aed: PUSH 0x14
0x8095aef: PUSH EBX
0x8095af0: PUSH ESI
0x8095af1: CALL 0x080b8a70
0x8095af6: ADD ESP, 0x10
0x8095af9: TEST EAX, EAX
0x8095afb: JZ 0x08095c1d
0x8095b01: MOV EBX, dword ptr [EBX]
0x8095b03: SUB ESP, 0x8
0x8095b06: MOV dword ptr [EBX], 0x0
0x8095b0c: MOV dword ptr [EBX + 0xc], 0x0
0x8095b13: MOV dword ptr [EBX + 0x10], 0x0
0x8095b1a: PUSH EBX
0x8095b1b: PUSH ESI
0x8095b1c: CALL 0x080b8dc8
0x8095b21: ADD ESP, 0x10
0x8095b24: TEST EAX, EAX
0x8095b26: JZ 0x08095c2a
0x8095b2c: ADD EBX, 0x10
0x8095b2f: MOV DX, word ptr [EBP + -0x12]
0x8095b33: LEA EAX, [EDX + -0x1]
0x8095b36: TEST DX, DX
0x8095b39: MOV word ptr [EBP + -0x12], AX
0x8095b3d: JG 0x08095aec
0x8095b3f: SUB ESP, 0x8
0x8095b42: PUSH EDI
0x8095b43: PUSH ESI
0x8095b44: CALL 0x080b8b44
0x8095b49: ADD ESP, 0x10
0x8095b4c: TEST EAX, EAX
0x8095b4e: JZ 0x08095c03
0x8095b54: MOV EBX, dword ptr [EBP + -0x10]
0x8095b57: ADD EBX, 0x34
0x8095b5a: JMP 0x08095b9f
0x8095b5c: PUSH EAX
0x8095b5d: PUSH 0x14
0x8095b5f: PUSH EBX
0x8095b60: PUSH ESI
0x8095b61: CALL 0x080b8a70
0x8095b66: ADD ESP, 0x10
0x8095b69: TEST EAX, EAX
0x8095b6b: JZ 0x08095c5f
0x8095b71: MOV EBX, dword ptr [EBX]
0x8095b73: SUB ESP, 0x8
0x8095b76: MOV dword ptr [EBX], 0x0
0x8095b7c: MOV dword ptr [EBX + 0xc], 0x0
0x8095b83: MOV dword ptr [EBX + 0x10], 0x0
0x8095b8a: PUSH EBX
0x8095b8b: PUSH ESI
0x8095b8c: CALL 0x080b8dc8
0x8095b91: ADD ESP, 0x10
0x8095b94: TEST EAX, EAX
0x8095b96: JZ 0x08095c6c
0x8095b9c: ADD EBX, 0x10
0x8095b9f: MOV DX, word ptr [EBP + -0x12]
0x8095ba3: LEA EAX, [EDX + -0x1]
0x8095ba6: TEST DX, DX
0x8095ba9: MOV word ptr [EBP + -0x12], AX
0x8095bad: JG 0x08095b5c
0x8095baf: CALL 0x080b5bd0
0x8095bb4: TEST EAX, EAX
0x8095bb6: JNZ 0x08095be1
0x8095bb8: JMP 0x08095bd7
0x8095bbc: SUB ESP, 0xc
0x8095bbf: MOV EAX, dword ptr [EBX + 0x10]
0x8095bc2: MOV dword ptr [EDX + 0x34], EAX
0x8095bc5: PUSH dword ptr [EBX]
0x8095bc7: CALL 0x080891d4
0x8095bcc: MOV dword ptr [ESP], EBX
0x8095bcf: CALL 0x080891d4
0x8095bd4: ADD ESP, 0x10
0x8095bd7: MOV EDX, dword ptr [EBP + -0x10]
0x8095bda: MOV EBX, dword ptr [EDX + 0x34]
0x8095bdd: TEST EBX, EBX
0x8095bdf: JNZ 0x08095bbc
0x8095be1: SUB ESP, 0x8
0x8095be4: PUSH 0x82ae168
0x8095be9: PUSH ESI
0x8095bea: CALL 0x080b8800
0x8095bef: ADD ESP, 0x10
0x8095bf2: TEST EAX, EAX
0x8095bf4: JNZ 0x08095c37
0x8095bf6: SUB ESP, 0xc
0x8095bf9: PUSH 0x82ae9d0
0x8095bfe: JMP 0x080959df
0x8095c03: SUB ESP, 0xc
0x8095c06: PUSH 0x82aea08
0x8095c0b: JMP 0x080959df
0x8095c10: SUB ESP, 0xc
0x8095c13: PUSH 0x82aea3c
0x8095c18: JMP 0x080959df
0x8095c1d: SUB ESP, 0xc
0x8095c20: PUSH 0x82aea6c
0x8095c25: JMP 0x080959df
0x8095c2a: SUB ESP, 0xc
0x8095c2d: PUSH 0x82aeaa0
0x8095c32: JMP 0x080959df
0x8095c37: PUSH EAX
0x8095c38: MOV EAX, dword ptr [EBP + -0x10]
0x8095c3b: LEA EDX, [EAX + 0x10]
0x8095c3e: PUSH EDX
0x8095c3f: PUSH EAX
0x8095c40: PUSH ESI
0x8095c41: CALL 0x08096128
0x8095c46: ADD ESP, 0x10
0x8095c49: TEST EAX, EAX
0x8095c4b: JZ 0x08095c79
0x8095c4d: MOV EAX, dword ptr [EBP + 0xc]
0x8095c50: MOV EDX, dword ptr [EBP + -0x10]
0x8095c53: MOV dword ptr [EAX], EDX
0x8095c55: MOV EAX, 0x1
0x8095c5a: JMP 0x080959e6
0x8095c5f: SUB ESP, 0xc
0x8095c62: PUSH 0x82aeacc
0x8095c67: JMP 0x080959df
0x8095c6c: SUB ESP, 0xc
0x8095c6f: PUSH 0x82aeb04
0x8095c74: JMP 0x080959df
0x8095c79: SUB ESP, 0xc
0x8095c7c: PUSH 0x82aeb30
0x8095c81: JMP 0x080959df
```

## 复用：vm_pushLocalVar — procedure_assembly

Evidence ID：`ev_d0c3b683bea4f35107c1c4be31af558ea2635c43a303b9009673e8de1ca083fc`

```asm
0x80914dc: PUSH EBP
0x80914dd: MOV EBP, ESP
0x80914df: PUSH EBX
0x80914e0: PUSH EAX
0x80914e1: MOV ECX, dword ptr [0x086619cc]
0x80914e7: TEST ECX, ECX
0x80914e9: MOV EBX, dword ptr [EBP + 0x8]
0x80914ec: JZ 0x080915a4
0x80914f2: MOV EAX, [0x086619c0]
0x80914f7: TEST EAX, EAX
0x80914f9: JZ 0x0809150c
0x80914fb: LEA EDX, [EAX + EAX*0x4]
0x80914fe: MOV EAX, [0x086619c8]
0x8091503: LEA EAX, [EAX + EDX*0x8]
0x8091506: CMP dword ptr [EAX + 0x4], EBX
0x8091509: JG 0x08091544
0x809150b: NOP
0x809150c: TEST ECX, ECX
0x809150e: JZ 0x080915c0
0x8091514: DEC ECX
0x8091515: MOV EAX, [0x08661964]
0x809151a: LEA EDX, [ECX + ECX*0x2]
0x809151d: MOV dword ptr [0x086619cc], ECX
0x8091523: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x0
0x8091528: MOV EAX, [0x086619cc]
0x809152d: LEA EAX, [EAX + EAX*0x2]
0x8091530: MOV EDX, dword ptr [0x08661964]
0x8091536: MOV dword ptr [EDX + EAX*0x8 + 0x8], 0x0
0x809153e: MOV EBX, dword ptr [EBP + -0x4]
0x8091541: LEAVE
0x8091542: RET
0x8091544: ADD EBX, dword ptr [EAX]
0x8091546: CMP EBX, ECX
0x8091548: JL 0x0809150c
0x809154a: LEA EAX, [ECX + -0x1]
0x809154d: MOV [0x086619cc], EAX
0x8091552: MOV EDX, dword ptr [0x08661964]
0x8091558: LEA EAX, [EAX + EAX*0x2]
0x809155b: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x8091560: MOV EDX, dword ptr [0x08661964]
0x8091566: LEA EAX, [EBX + EBX*0x2]
0x8091569: SUB ESP, 0x8
0x809156c: LEA EAX, [EDX + EAX*0x8]
0x809156f: PUSH EAX
0x8091570: MOV EAX, [0x086619cc]
0x8091575: LEA EAX, [EAX + EAX*0x2]
0x8091578: LEA EAX, [EDX + EAX*0x8]
0x809157b: PUSH EAX
0x809157c: CALL 0x0808be24
0x8091581: MOV EAX, [0x086619cc]
0x8091586: LEA EAX, [EAX + EAX*0x2]
0x8091589: MOV EDX, dword ptr [0x08661964]
0x809158f: ADD ESP, 0x10
0x8091592: CMP byte ptr [EDX + EAX*0x8 + 0x4], 0x8
0x8091597: JNZ 0x0809153e
0x8091599: MOV EBX, dword ptr [EBP + -0x4]
0x809159c: LEAVE
0x809159d: JMP 0x0808c028
0x80915a4: SUB ESP, 0xc
0x80915a7: PUSH 0x82ad98c
0x80915ac: CALL 0x08097b24
0x80915b1: ADD ESP, 0x10
0x80915b4: MOV ECX, dword ptr [0x086619cc]
0x80915ba: JMP 0x080914f2
0x80915c0: SUB ESP, 0xc
0x80915c3: PUSH 0x82ad98c
0x80915c8: CALL 0x08097b24
0x80915cd: ADD ESP, 0x10
0x80915d0: MOV ECX, dword ptr [0x086619cc]
0x80915d6: JMP 0x08091514
```

## 复用：vm_pushLocalVar — procedure_pseudo_code

Evidence ID：`ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5`

```c

void vm_pushLocalVar(int no)

{
  int iVar1;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg0(EAX)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (((vm_funCp == 0) || (vm_funContext[vm_funCp].argNum <= no)) ||
     (iVar1 = no + vm_funContext[vm_funCp].bp, iVar1 < vm_sp)) {
    if (vm_sp == 0) {
      vm_error("Stack overflow when calculating.\n");
    }
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\0';
    vm_stack[vm_sp].v.info = (void *)0x0;
  }
  else {
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\0';
    vm_copyValue(vm_stack + vm_sp,vm_stack + iVar1);
    if (vm_stack[vm_sp].type == '\b') {
      vm_convertToValue();
      return;
    }
  }
  return;
}
```

## 复用：vm_pushLocalVptr — procedure_assembly

Evidence ID：`ev_9b4e2d24822946ff7ae5449f7d30b3e3b2cc1134ef9c530294dd9a840f45feed`

```asm
0x809134c: PUSH EBP
0x809134d: MOV EBP, ESP
0x809134f: PUSH ESI
0x8091350: PUSH EBX
0x8091351: MOV EAX, [0x086619cc]
0x8091356: TEST EAX, EAX
0x8091358: MOV EBX, dword ptr [EBP + 0x8]
0x809135b: JZ 0x080914a4
0x8091361: MOV EAX, [0x086619c0]
0x8091366: TEST EAX, EAX
0x8091368: JZ 0x08091488
0x809136e: LEA EDX, [EAX + EAX*0x4]
0x8091371: MOV EAX, [0x086619c8]
0x8091376: LEA ESI, [EAX + EDX*0x8]
0x8091379: CMP dword ptr [ESI + 0x4], EBX
0x809137c: JLE 0x08091470
0x8091382: ADD EBX, dword ptr [ESI]
0x8091384: MOV EDX, dword ptr [0x086619cc]
0x809138a: CMP EBX, EDX
0x809138c: JL 0x08091404
0x809138e: LEA EAX, [EBX + EBX*0x2]
0x8091391: MOV ECX, dword ptr [0x08661964]
0x8091397: SHL EAX, 0x3
0x809139a: CMP byte ptr [EAX + ECX*0x1 + 0x4], 0x8
0x809139f: JZ 0x08091434
0x80913a5: TEST EDX, EDX
0x80913a7: LEA EBX, [EAX + ECX*0x1]
0x80913aa: JZ 0x080913e4
0x80913ac: LEA EAX, [EDX + -0x1]
0x80913af: MOV [0x086619cc], EAX
0x80913b4: LEA EAX, [EAX + EAX*0x2]
0x80913b7: MOV byte ptr [ECX + EAX*0x8 + 0x4], 0x8
0x80913bc: MOV EAX, [0x086619cc]
0x80913c1: LEA EAX, [EAX + EAX*0x2]
0x80913c4: MOV ECX, dword ptr [0x08661964]
0x80913ca: MOV DL, byte ptr [EBX + 0x5]
0x80913cd: SHL EAX, 0x3
0x80913d0: INC dword ptr [EBX]
0x80913d2: MOV dword ptr [EAX + ECX*0x1 + 0x8], EBX
0x80913d6: MOV byte ptr [EAX + ECX*0x1 + 0x5], DL
0x80913da: LEA ESP, [EBP + -0x8]
0x80913dd: POP EBX
0x80913de: POP ESI
0x80913df: LEAVE
0x80913e0: RET
0x80913e4: SUB ESP, 0xc
0x80913e7: PUSH 0x82ad98c
0x80913ec: CALL 0x08097b24
0x80913f1: ADD ESP, 0x10
0x80913f4: MOV EDX, dword ptr [0x086619cc]
0x80913fa: MOV ECX, dword ptr [0x08661964]
0x8091400: JMP 0x080913ac
0x8091404: SUB ESP, 0xc
0x8091407: PUSH 0x82acfc5
0x809140c: CALL 0x08097b24
0x8091411: LEA EAX, [EBX + EBX*0x2]
0x8091414: MOV ECX, dword ptr [0x08661964]
0x809141a: SHL EAX, 0x3
0x809141d: ADD ESP, 0x10
0x8091420: CMP byte ptr [EAX + ECX*0x1 + 0x4], 0x8
0x8091425: MOV EDX, dword ptr [0x086619cc]
0x809142b: JNZ 0x080913a5
0x8091431: LEA ESI, [ESI]
0x8091434: TEST EDX, EDX
0x8091436: LEA EBX, [EAX + ECX*0x1]
0x8091439: JZ 0x080914b9
0x809143b: DEC EDX
0x809143c: SUB ESP, 0x8
0x809143f: LEA EAX, [EDX + EDX*0x2]
0x8091442: MOV dword ptr [0x086619cc], EDX
0x8091448: MOV byte ptr [ECX + EAX*0x8 + 0x4], 0x0
0x809144d: PUSH EBX
0x809144e: MOV EAX, [0x086619cc]
0x8091453: MOV EDX, dword ptr [0x08661964]
0x8091459: LEA EAX, [EAX + EAX*0x2]
0x809145c: LEA EAX, [EDX + EAX*0x8]
0x809145f: PUSH EAX
0x8091460: CALL 0x0808be24
0x8091465: ADD ESP, 0x10
0x8091468: LEA ESP, [EBP + -0x8]
0x809146b: POP EBX
0x809146c: POP ESI
0x809146d: LEAVE
0x809146e: RET
0x8091470: SUB ESP, 0xc
0x8091473: PUSH 0x82acfc5
0x8091478: CALL 0x08097b24
0x809147d: ADD ESP, 0x10
0x8091480: JMP 0x08091382
0x8091488: SUB ESP, 0xc
0x809148b: PUSH 0x82acfc5
0x8091490: CALL 0x08097b24
0x8091495: ADD ESP, 0x10
0x8091498: MOV EAX, [0x086619c0]
0x809149d: JMP 0x0809136e
0x80914a4: SUB ESP, 0xc
0x80914a7: PUSH 0x82ad98c
0x80914ac: CALL 0x08097b24
0x80914b1: ADD ESP, 0x10
0x80914b4: JMP 0x08091361
0x80914b9: SUB ESP, 0xc
0x80914bc: PUSH 0x82ad98c
0x80914c1: CALL 0x08097b24
0x80914c6: ADD ESP, 0x10
0x80914c9: MOV EDX, dword ptr [0x086619cc]
0x80914cf: MOV ECX, dword ptr [0x08661964]
0x80914d5: JMP 0x0809143b
```

## 复用：vm_pushLocalVptr — procedure_pseudo_code

Evidence ID：`ev_cdf3f69a6cd4c53275ebfcb955c6419230978dcbe6c1af8e4d0c1dc6d6c32de2`

```c

void vm_pushLocalVptr(int no)

{
  Vm_FunCall_Context_T *pVVar1;
  Vm_Value_T *pVVar2;
  uchar uVar3;
  Vm_Value_T *pVVar4;
  int iVar5;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg6(ESI)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (vm_funCp == 0) {
    vm_error("No such argument.\n");
  }
  pVVar1 = vm_funContext + vm_funCp;
  if (pVVar1->argNum <= no) {
    vm_error("No such argument.\n");
  }
  iVar5 = no + pVVar1->bp;
  if (iVar5 < vm_sp) {
    vm_error("No such argument.\n");
    uVar3 = vm_stack[iVar5].type;
  }
  else {
    uVar3 = vm_stack[iVar5].type;
  }
  if (uVar3 != '\b') {
    pVVar2 = vm_stack + iVar5;
    if (vm_sp == 0) {
      vm_error("Stack overflow when calculating.\n");
    }
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\b';
    iVar5 = vm_sp;
    pVVar4 = vm_stack;
    uVar3 = pVVar2->attrib;
    pVVar2->ref = pVVar2->ref + 1;
    pVVar4[iVar5].v.pPtrValue = pVVar2;
    pVVar4[iVar5].attrib = uVar3;
    return;
  }
  pVVar2 = vm_stack + iVar5;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_copyValue(vm_stack + vm_sp,pVVar2);
  return;
}
```

## 复用：vm_returnFromFun — procedure_assembly

Evidence ID：`ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610`

```asm
0x8094978: PUSH EBP
0x8094979: MOV EBP, ESP
0x809497b: PUSH EDI
0x809497c: PUSH ESI
0x809497d: PUSH EBX
0x809497e: SUB ESP, 0xc
0x8094981: MOV ESI, dword ptr [0x086619c0]
0x8094987: TEST ESI, ESI
0x8094989: JNZ 0x080949d4
0x809498b: MOV EAX, [0x086619cc]
0x8094990: CMP EAX, dword ptr [0x08661960]
0x8094996: JGE 0x080949af
0x8094998: LEA EAX, [EAX + EAX*0x2]
0x809499b: MOV EDX, dword ptr [0x08661964]
0x80949a1: SHL EAX, 0x3
0x80949a4: CMP byte ptr [EAX + EDX*0x1 + 0x4], 0x2
0x80949a9: JZ 0x08094ccc
0x80949af: SUB ESP, 0xc
0x80949b2: PUSH 0x0
0x80949b4: CALL 0x080b970c
0x80949b9: MOV dword ptr [ESP], 0x82ae520
0x80949c0: CALL 0x08097fc0
0x80949c5: MOV dword ptr [ESP], 0xffffe05a
0x80949cc: CALL 0x080ab030
0x80949d1: ADD ESP, 0x10
0x80949d4: CALL 0x080a9310
0x80949d9: MOV EBX, EAX
0x80949db: MOV EAX, dword ptr [EBX + 0x98]
0x80949e1: TEST EAX, EAX
0x80949e3: JLE 0x08094a0b
0x80949e5: LEA EAX, [EAX + EAX*0x4]
0x80949e8: SHL EAX, 0x2
0x80949eb: ADD EAX, dword ptr [EBX + 0xa0]
0x80949f1: MOV EDX, dword ptr [0x086619c0]
0x80949f7: CMP dword ptr [EAX + -0x10], EDX
0x80949fa: JL 0x08094a11
0x80949fc: CALL 0x080aae08
0x8094a01: MOV EAX, dword ptr [EBX + 0x98]
0x8094a07: TEST EAX, EAX
0x8094a09: JG 0x080949e5
0x8094a0b: MOV EDX, dword ptr [0x086619c0]
0x8094a11: MOV EAX, [0x086619c8]
0x8094a16: LEA EDX, [EDX + EDX*0x4]
0x8094a19: LEA EDX, [EAX + EDX*0x8]
0x8094a1c: MOV EAX, dword ptr [EDX + 0x8]
0x8094a1f: TEST AL, 0x8
0x8094a21: MOV dword ptr [EBP + -0x10], EDX
0x8094a24: JZ 0x08094c33
0x8094a2a: TEST AL, 0x4
0x8094a2c: JZ 0x08094bff
0x8094a32: MOV ECX, dword ptr [EBP + -0x10]
0x8094a35: MOV ESI, dword ptr [ECX + 0x24]
0x8094a38: MOVSX EAX, word ptr [ESI + 0xa]
0x8094a3c: MOV EDX, dword ptr [ECX]
0x8094a3e: SUB EDX, EAX
0x8094a40: MOVZX EDI, byte ptr [ESI + 0xd]
0x8094a44: SUB EDX, dword ptr [0x086619cc]
0x8094a4a: CMP EDI, 0x1
0x8094a4d: JZ 0x08094bdf
0x8094a53: CMP EDI, 0xff
0x8094a59: JZ 0x08094a98
0x8094a5b: TEST EDX, EDX
0x8094a5d: JZ 0x08094cb5
0x8094a63: DEC EDX
0x8094a64: JZ 0x08094ad6
0x8094a66: SUB ESP, 0x8
0x8094a69: PUSH 0x546
0x8094a6e: PUSH 0x82ae55c
0x8094a73: PUSH 0x82ae584
0x8094a78: PUSH 0x82a825c
0x8094a7d: PUSH 0x4
0x8094a7f: PUSH 0x2
0x8094a81: CALL 0x080b61c4
0x8094a86: ADD ESP, 0x14
0x8094a89: PUSH 0x0
0x8094a8b: CALL 0x080a3980
0x8094a90: ADD ESP, 0x10
0x8094a93: JMP 0x08094a93
0x8094a98: TEST EDX, EDX
0x8094a9a: JZ 0x08094bf5
0x8094aa0: DEC EDX
0x8094aa1: JZ 0x08094b57
0x8094aa7: SUB ESP, 0x8
0x8094aaa: PUSH 0x53e
0x8094aaf: PUSH 0x82ae55c
0x8094ab4: PUSH 0x82ae584
0x8094ab9: PUSH 0x82a825c
0x8094abe: PUSH 0x4
0x8094ac0: PUSH 0x2
0x8094ac2: CALL 0x080b61c4
0x8094ac7: ADD ESP, 0x14
0x8094aca: PUSH 0x0
0x8094acc: CALL 0x080a3980
0x8094ad1: ADD ESP, 0x10
0x8094ad4: JMP 0x08094ad4
0x8094ad6: MOV EAX, [0x086619cc]
0x8094adb: LEA EAX, [EAX + EAX*0x2]
0x8094ade: MOV EDX, dword ptr [0x08661964]
0x8094ae4: MOVZX EAX, byte ptr [EDX + EAX*0x8 + 0x4]
0x8094ae9: CMP EAX, EDI
0x8094aeb: JZ 0x08094b57
0x8094aed: CALL 0x08090604
0x8094af2: SUB ESP, 0xc
0x8094af5: PUSH EAX
0x8094af6: MOV EBX, EAX
0x8094af8: CALL 0x0808cb3c
0x8094afd: ADD ESP, 0x10
0x8094b00: CMP EDI, 0x2
0x8094b03: JZ 0x08094c9b
0x8094b09: CMP EDI, 0x3
0x8094b0c: JZ 0x08094c7f
0x8094b12: MOV DL, byte ptr [ESI + 0xc]
0x8094b15: TEST DL, 0x8
0x8094b18: JZ 0x08094b1e
0x8094b1a: TEST EAX, EAX
0x8094b1c: JZ 0x08094b5a
0x8094b1e: SUB ESP, 0xc
0x8094b21: PUSH EDI
0x8094b22: CALL 0x0808a13c
0x8094b27: ADD ESP, 0x10
0x8094b2a: PUSH EAX
0x8094b2b: PUSH dword ptr [ESI]
0x8094b2d: MOV EAX, [0x086619cc]
0x8094b32: MOV EDX, dword ptr [0x08661964]
0x8094b38: LEA EAX, [EAX + EAX*0x2]
0x8094b3b: PUSH EBX
0x8094b3c: MOVZX EAX, byte ptr [EDX + EAX*0x8 + 0x4]
0x8094b41: PUSH EAX
0x8094b42: CALL 0x0808a13c
0x8094b47: POP EDX
0x8094b48: POP ECX
0x8094b49: PUSH EAX
0x8094b4a: PUSH 0x82ae5a4
0x8094b4f: CALL 0x08097b24
0x8094b54: ADD ESP, 0x10
0x8094b57: MOV DL, byte ptr [ESI + 0xc]
0x8094b5a: AND EDX, 0x2
0x8094b5d: JNZ 0x08094c5d
0x8094b63: MOV EAX, [0x086619c0]
0x8094b68: MOV ECX, dword ptr [0x086619c8]
0x8094b6e: LEA EAX, [EAX + EAX*0x4]
0x8094b71: MOVSX EDX, word ptr [ESI + 0xa]
0x8094b75: ADD EDX, dword ptr [ECX + EAX*0x8 + 0x4]
0x8094b79: JNZ 0x08094c4a
0x8094b7f: MOV EAX, [0x086619d8]
0x8094b84: MOV EBX, dword ptr [EAX + 0x20]
0x8094b87: MOV EAX, [0x0860ae64]
0x8094b8c: TEST EAX, EAX
0x8094b8e: JZ 0x08094bd2
0x8094b90: CALL 0x080bb0d4
0x8094b95: DEC dword ptr [0x086619c0]
0x8094b9b: CALL 0x0809789c
0x8094ba0: CALL 0x080bb0e8
0x8094ba5: TEST EBX, EBX
0x8094ba7: JZ 0x08094bb5
0x8094ba9: SUB ESP, 0xc
0x8094bac: PUSH EBX
0x8094bad: CALL 0x080b1804
0x8094bb2: ADD ESP, 0x10
0x8094bb5: MOV EAX, dword ptr [EBP + -0x10]
0x8094bb8: TEST byte ptr [EAX + 0x8], 0x80
0x8094bbc: JZ 0x08094bca
0x8094bbe: SUB ESP, 0xc
0x8094bc1: PUSH dword ptr [EAX + 0x1c]
0x8094bc4: CALL dword ptr [EAX + 0x18]
0x8094bc7: ADD ESP, 0x10
0x8094bca: LEA ESP, [EBP + -0xc]
0x8094bcd: POP EBX
0x8094bce: POP ESI
0x8094bcf: POP EDI
0x8094bd0: LEAVE
0x8094bd1: RET
0x8094bd2: DEC dword ptr [0x086619c0]
0x8094bd8: CALL 0x0809789c
0x8094bdd: JMP 0x08094ba5
0x8094bdf: TEST EDX, EDX
0x8094be1: JZ 0x08094bf5
0x8094be3: SUB ESP, 0x8
0x8094be6: PUSH dword ptr [ESI]
0x8094be8: PUSH 0x82ae5ec
0x8094bed: CALL 0x08097b24
0x8094bf2: ADD ESP, 0x10
0x8094bf5: CALL 0x08090fc0
0x8094bfa: JMP 0x08094b57
0x8094bff: SUB ESP, 0xc
0x8094c02: PUSH 0x82ae61c
0x8094c07: CALL 0x08097fc0
0x8094c0c: MOV EAX, [0x086619c0]
0x8094c11: MOV EDX, dword ptr [0x086619c8]
0x8094c17: LEA EAX, [EAX + EAX*0x4]
0x8094c1a: MOV EAX, dword ptr [EDX + EAX*0x8]
0x8094c1d: SUB EAX, dword ptr [0x086619cc]
0x8094c23: MOV dword ptr [ESP], EAX
0x8094c26: CALL 0x08091ab0
0x8094c2b: ADD ESP, 0x10
0x8094c2e: JMP 0x08094b7f
0x8094c33: CALL 0x080a9310
0x8094c38: AND dword ptr [EAX + 0x28], 0xfffffeff
0x8094c3f: MOV EDX, dword ptr [EBP + -0x10]
0x8094c42: MOV EAX, dword ptr [EDX + 0x8]
0x8094c45: JMP 0x08094a2a
0x8094c4a: SUB ESP, 0x8
0x8094c4d: PUSH EDX
0x8094c4e: PUSH 0x1
0x8094c50: CALL 0x08092008
0x8094c55: ADD ESP, 0x10
0x8094c58: JMP 0x08094b7f
0x8094c5d: MOV EAX, [0x086619cc]
0x8094c62: MOV EDX, dword ptr [0x08661964]
0x8094c68: LEA EAX, [EAX + EAX*0x2]
0x8094c6b: SUB ESP, 0xc
0x8094c6e: LEA EAX, [EDX + EAX*0x8]
0x8094c71: PUSH EAX
0x8094c72: CALL 0x0808cc18
0x8094c77: ADD ESP, 0x10
0x8094c7a: JMP 0x08094b63
0x8094c7f: CMP EAX, 0x2
0x8094c82: JNZ 0x08094b12
0x8094c88: CVTSI2SS XMM0, dword ptr [EBX + 0x8]
0x8094c8d: MOVSS dword ptr [EBX + 0x8], XMM0
0x8094c92: MOV byte ptr [EBX + 0x4], 0x3
0x8094c96: JMP 0x08094b57
0x8094c9b: CMP EAX, 0x3
0x8094c9e: JNZ 0x08094b12
0x8094ca4: CVTTSS2SI EAX, dword ptr [EBX + 0x8]
0x8094ca9: MOV dword ptr [EBX + 0x8], EAX
0x8094cac: MOV byte ptr [EBX + 0x4], 0x2
0x8094cb0: JMP 0x08094b57
0x8094cb5: SUB ESP, 0x8
0x8094cb8: PUSH dword ptr [ESI]
0x8094cba: PUSH 0x82ae65c
0x8094cbf: CALL 0x08097b24
0x8094cc4: ADD ESP, 0x10
0x8094cc7: JMP 0x08094a66
0x8094ccc: SUB ESP, 0xc
0x8094ccf: PUSH dword ptr [EAX + EDX*0x1 + 0x8]
0x8094cd3: JMP 0x080949b4
```

## 复用：vm_returnFromFun — procedure_pseudo_code

Evidence ID：`ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_returnFromFun(void)

{
  Vm_FunCall_Context_T *pVVar1;
  byte bVar2;
  Vm_Fun *pVVar3;
  Vm_Object_T *pObject;
  Vm_PCB_T *pVVar4;
  Vm_Value *pValue;
  char *pcVar5;
  char *pcVar6;
  uint uVar7;
  int iVar8;
  char *pcVar9;
  Vm_FunCall_Context_T *pContext;
  
                    /* Unresolved local var: Vm_Object_T * lastObject@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg3(EBX)]
                       Unresolved local var: int to@[DW_OP_reg0(EAX)]
                       Unresolved local var: int retNum@[DW_OP_reg2(EDX)]
                       Unresolved local var: int retType@[DW_OP_reg7(EDI)] */
  if (vm_funCp == 0) {
    if ((vm_sp < vm_stackSize) && (vm_stack[vm_sp].type == '\x02')) {
      iVar8 = vm_stack[vm_sp].v.number;
    }
    else {
      iVar8 = 0;
    }
    vm_setReturnValue(iVar8);
    vm_printf("Return on the top layer of call stack. Execution abort.\n");
    vm_returnToBackend(-0x1fa6);
  }
  pVVar4 = vm_getCurrentPCB();
  iVar8 = pVVar4->catchCp;
  while ((0 < iVar8 && (vm_funCp <= pVVar4->catchContext[iVar8 + -1].funCp))) {
    vm_endThreadCatch();
    iVar8 = pVVar4->catchCp;
  }
  pVVar1 = vm_funContext + vm_funCp;
  uVar7 = pVVar1->flags;
  if ((uVar7 & 8) == 0) {
    pVVar4 = vm_getCurrentPCB();
    pVVar4->flags = pVVar4->flags & 0xfffffeff;
    uVar7 = pVVar1->flags;
  }
  if ((uVar7 & 4) == 0) {
    vm_printf("Return on the top layer when do object initialzation, abort.\n");
    vm_popNStack(vm_funContext[vm_funCp].bp - vm_sp);
    goto LAB_08094b7f;
  }
  pVVar3 = (pVVar1->f).pFun;
  iVar8 = pVVar1->bp - (int)pVVar3->localVariables;
  uVar7 = (uint)pVVar3->retType;
  if (uVar7 == 1) {
    if (iVar8 != vm_sp) {
      vm_error("The function \"%s\" shouldn\'t return any value.\n",pVVar3->name);
    }
LAB_08094bf5:
    vm_pushVoid();
LAB_08094b57:
    bVar2 = pVVar3->retVarAttrib;
  }
  else {
    if (uVar7 == 0xff) {
      if (iVar8 != vm_sp) {
        if (iVar8 - vm_sp != 1) {
          vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Stack error for function call.\n"
                     ,"/home/cc/xxxx/gameshell/src/vm/vm_fun.c",0x53e);
          _vm_endProcess(0);
          do {
                    /* WARNING: Do nothing block with infinite loop */
          } while( true );
        }
        goto LAB_08094b57;
      }
      goto LAB_08094bf5;
    }
    if (iVar8 == vm_sp) {
      vm_error("The function \"%s\" should return 1 value but not.\n",pVVar3->name);
LAB_08094a66:
      vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Stack error for function call.\n",
                 "/home/cc/xxxx/gameshell/src/vm/vm_fun.c",0x546);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    if (iVar8 - vm_sp != 1) goto LAB_08094a66;
    if (vm_stack[vm_sp].type == uVar7) goto LAB_08094b57;
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int type@[DW_OP_reg0(EAX)] */
    pValue = vm_stackTopValue();
    iVar8 = vm_getVarType(pValue);
    if (uVar7 == 2) {
      if (iVar8 != 3) goto LAB_08094b12;
      (pValue->v).number = (int)(pValue->v).real;
      pValue->type = '\x02';
      goto LAB_08094b57;
    }
    if ((uVar7 == 3) && (iVar8 == 2)) {
      (pValue->v).real = (float)(pValue->v).number;
      pValue->type = '\x03';
      goto LAB_08094b57;
    }
LAB_08094b12:
    bVar2 = pVVar3->retVarAttrib;
    if (((bVar2 & 8) == 0) || (iVar8 != 0)) {
      pcVar5 = vm_valueType(uVar7);
      pcVar9 = pVVar3->name;
      pcVar6 = vm_valueType((uint)vm_stack[vm_sp].type);
      vm_error("Return type (%s) of function \"%s\" mismatched with the prototype (%s).\n",pcVar6,
               pcVar9,pcVar5);
      goto LAB_08094b57;
    }
  }
  if ((bVar2 & 2) != 0) {
    vm_convertValueToConst(vm_stack + vm_sp);
  }
  iVar8 = (int)pVVar3->localVariables + vm_funContext[vm_funCp].argNum;
  if (iVar8 != 0) {
    vm_takeOffValueFromStack(1,iVar8);
  }
LAB_08094b7f:
  pObject = vm_currentContext->thisObject;
  if (vm_gatheringCallStackInfo == 0) {
    vm_funCp = vm_funCp + -1;
    vm_updateCurrentFunCallContext();
  }
  else {
    vm_enterPPCopyStackCriticalSection();
    vm_funCp = vm_funCp + -1;
    vm_updateCurrentFunCallContext();
    vm_leavePPCopyStackCriticalSection();
  }
  if (pObject != (Vm_Object_T *)0x0) {
    vm_unExecRefObject(pObject);
  }
  if ((pVVar1->flags & 0x80) != 0) {
    (*pVVar1->returnCallback)(pVVar1->pCookie);
  }
  return;
}
```

## 复用：vm_setLocalVar — procedure_assembly

Evidence ID：`ev_b4762566f69e6ef759a4c846394fb10d47cd1eebf8da295f5f40105ef3884689`

```asm
0x808f6dc: PUSH EBP
0x808f6dd: MOV EBP, ESP
0x808f6df: PUSH ESI
0x808f6e0: PUSH EBX
0x808f6e1: MOV EAX, [0x086619cc]
0x808f6e6: TEST EAX, EAX
0x808f6e8: MOV EBX, dword ptr [EBP + 0x8]
0x808f6eb: JZ 0x0808f7cc
0x808f6f1: MOV EAX, [0x086619c0]
0x808f6f6: TEST EAX, EAX
0x808f6f8: JZ 0x0808f7b0
0x808f6fe: LEA EDX, [EAX + EAX*0x4]
0x808f701: MOV EAX, [0x086619c8]
0x808f706: LEA ESI, [EAX + EDX*0x8]
0x808f709: CMP dword ptr [ESI + 0x4], EBX
0x808f70c: JLE 0x0808f798
0x808f712: ADD EBX, dword ptr [ESI]
0x808f714: MOV ECX, dword ptr [0x086619cc]
0x808f71a: CMP EBX, ECX
0x808f71c: JL 0x0808f780
0x808f71e: LEA EAX, [EBX + EBX*0x2]
0x808f721: MOV EDX, dword ptr [0x08661964]
0x808f727: LEA EBX, [EDX + EAX*0x8]
0x808f72a: CMP byte ptr [EBX + 0x4], 0x8
0x808f72e: LEA EAX, [ECX + ECX*0x2]
0x808f731: LEA ESI, [EDX + EAX*0x8]
0x808f734: JZ 0x0808f754
0x808f736: SUB ESP, 0xc
0x808f739: PUSH EBX
0x808f73a: CALL 0x0808ad98
0x808f73f: POP EAX
0x808f740: POP EDX
0x808f741: PUSH ESI
0x808f742: PUSH EBX
0x808f743: CALL 0x0808be24
0x808f748: ADD ESP, 0x10
0x808f74b: LEA ESP, [EBP + -0x8]
0x808f74e: POP EBX
0x808f74f: POP ESI
0x808f750: LEAVE
0x808f751: RET
0x808f754: CMP byte ptr [ESI + 0x4], 0x8
0x808f758: JZ 0x0808f7e1
0x808f75e: MOV EBX, dword ptr [EBX + 0x8]
0x808f761: SUB ESP, 0xc
0x808f764: PUSH EBX
0x808f765: CALL 0x0808ad98
0x808f76a: POP ECX
0x808f76b: POP EAX
0x808f76c: PUSH ESI
0x808f76d: PUSH EBX
0x808f76e: CALL 0x0808be24
0x808f773: ADD ESP, 0x10
0x808f776: LEA ESP, [EBP + -0x8]
0x808f779: POP EBX
0x808f77a: POP ESI
0x808f77b: LEAVE
0x808f77c: RET
0x808f780: SUB ESP, 0xc
0x808f783: PUSH 0x82acfc5
0x808f788: CALL 0x08097b24
0x808f78d: ADD ESP, 0x10
0x808f790: MOV ECX, dword ptr [0x086619cc]
0x808f796: JMP 0x0808f71e
0x808f798: SUB ESP, 0xc
0x808f79b: PUSH 0x82acfc5
0x808f7a0: CALL 0x08097b24
0x808f7a5: ADD ESP, 0x10
0x808f7a8: JMP 0x0808f712
0x808f7b0: SUB ESP, 0xc
0x808f7b3: PUSH 0x82acfc5
0x808f7b8: CALL 0x08097b24
0x808f7bd: ADD ESP, 0x10
0x808f7c0: MOV EAX, [0x086619c0]
0x808f7c5: JMP 0x0808f6fe
0x808f7cc: SUB ESP, 0xc
0x808f7cf: PUSH 0x82ad98c
0x808f7d4: CALL 0x08097b24
0x808f7d9: ADD ESP, 0x10
0x808f7dc: JMP 0x0808f6f1
0x808f7e1: SUB ESP, 0xc
0x808f7e4: PUSH 0x82ad9b0
0x808f7e9: CALL 0x08097b24
0x808f7ee: ADD ESP, 0x10
0x808f7f1: JMP 0x0808f75e
```

## 复用：vm_setLocalVar — procedure_pseudo_code

Evidence ID：`ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc`

```c

void vm_setLocalVar(int no)

{
  Vm_FunCall_Context_T *pVVar1;
  Vm_Value_T *pValue;
  Vm_Value_T *pVVar2;
  int iVar3;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue2@[DW_OP_reg6(ESI)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (vm_funCp == 0) {
    vm_error("No such argument.\n");
  }
  pVVar1 = vm_funContext + vm_funCp;
  if (pVVar1->argNum <= no) {
    vm_error("No such argument.\n");
  }
  iVar3 = no + pVVar1->bp;
  if (iVar3 < vm_sp) {
    vm_error("No such argument.\n");
  }
  pVVar2 = vm_stack + iVar3;
  pValue = vm_stack + vm_sp;
  if (pVVar2->type != '\b') {
    vm_cleanupValue(pVVar2);
    vm_copyValue(pVVar2,pValue);
    return;
  }
                    /* Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg3(EBX)] */
  if (pValue->type == '\b') {
    vm_error("Can not set a value which is reference type.\n");
  }
  pVVar2 = (Vm_Value_T *)(pVVar2->v).string;
  vm_cleanupValue(pVVar2);
  vm_copyValue(pVVar2,pValue);
  return;
}
```

## 复用：vm_takeOffValueFromStack — procedure_assembly

Evidence ID：`ev_c2177a4cc3c48912f1626bc399600d302a4c708f69bf066dade3508055bf7119`

```asm
0x8092008: PUSH EBP
0x8092009: MOV EBP, ESP
0x809200b: PUSH EDI
0x809200c: PUSH ESI
0x809200d: PUSH EBX
0x809200e: SUB ESP, 0xc
0x8092011: MOV EDI, dword ptr [EBP + 0xc]
0x8092014: CMP dword ptr [EBP + 0x8], EDI
0x8092017: JG 0x080920e0
0x809201d: MOV EAX, [0x086619cc]
0x8092022: ADD dword ptr [EBP + 0x8], EAX
0x8092025: ADD EDI, EAX
0x8092027: CMP dword ptr [EBP + 0x8], EAX
0x809202a: JL 0x080920e0
0x8092030: CMP EDI, dword ptr [0x08661960]
0x8092036: JGE 0x080920e0
0x809203c: MOV ESI, dword ptr [EBP + 0x8]
0x809203f: CMP ESI, EDI
0x8092041: JG 0x08092071
0x8092043: LEA EAX, [ESI + ESI*0x2]
0x8092046: LEA EBX, [EAX*0x8 + 0x0]
0x809204d: LEA ESI, [ESI]
0x8092050: MOV EAX, EBX
0x8092052: SUB ESP, 0xc
0x8092055: ADD EAX, dword ptr [0x08661964]
0x809205b: PUSH EAX
0x809205c: INC ESI
0x809205d: CALL 0x0808ad98
0x8092062: ADD EBX, 0x18
0x8092065: ADD ESP, 0x10
0x8092068: CMP ESI, EDI
0x809206a: JLE 0x08092050
0x809206c: MOV EAX, [0x086619cc]
0x8092071: MOV ESI, dword ptr [EBP + 0x8]
0x8092074: DEC ESI
0x8092075: INC EDI
0x8092076: CMP ESI, EAX
0x8092078: JL 0x080920d8
0x809207a: LEA EAX, [ESI + ESI*0x2]
0x809207d: LEA ECX, [EAX*0x8 + 0x0]
0x8092084: LEA EAX, [EDI + EDI*0x2]
0x8092087: LEA EBX, [EAX*0x8 + 0x0]
0x809208e: MOV ESI, ESI
0x8092090: MOV EDX, dword ptr [0x08661964]
0x8092096: MOV AL, byte ptr [ECX + EDX*0x1 + 0x4]
0x809209a: SUB EBX, 0x18
0x809209d: MOV byte ptr [EBX + EDX*0x1 + 0x4], AL
0x80920a1: MOV EDX, dword ptr [0x08661964]
0x80920a7: MOV AL, byte ptr [ECX + EDX*0x1 + 0x5]
0x80920ab: MOV byte ptr [EBX + EDX*0x1 + 0x5], AL
0x80920af: MOV EAX, [0x08661964]
0x80920b4: MOV EDX, dword ptr [ECX + EAX*0x1 + 0x8]
0x80920b8: MOV byte ptr [ECX + EAX*0x1 + 0x4], 0x0
0x80920bd: MOV dword ptr [EBX + EAX*0x1 + 0x8], EDX
0x80920c1: MOV EAX, [0x08661964]
0x80920c6: MOV byte ptr [ECX + EAX*0x1 + 0x5], 0x0
0x80920cb: DEC ESI
0x80920cc: DEC EDI
0x80920cd: SUB ECX, 0x18
0x80920d0: CMP ESI, dword ptr [0x086619cc]
0x80920d6: JGE 0x08092090
0x80920d8: MOV dword ptr [0x086619cc], EDI
0x80920de: MOV ESI, ESI
0x80920e0: LEA ESP, [EBP + -0xc]
0x80920e3: POP EBX
0x80920e4: POP ESI
0x80920e5: POP EDI
0x80920e6: LEAVE
0x80920e7: RET
```

## 复用：vm_takeOffValueFromStack — procedure_pseudo_code

Evidence ID：`ev_99c34e4e373606ceaa48084b7cbe89c70742d22173dfb81bb36c8bddcd94b778`

```c

void vm_takeOffValueFromStack(int spb,int spe)

{
  anon_union_4_11_5000f750_for_v *paVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  iVar3 = vm_sp;
  if (spb <= spe) {
    iVar5 = spb + vm_sp;
    iVar6 = spe + vm_sp;
    if ((vm_sp <= iVar5) && (iVar6 < vm_stackSize)) {
      if (iVar5 <= iVar6) {
        iVar3 = iVar5 * 0x18;
        iVar4 = iVar5;
        do {
          iVar4 = iVar4 + 1;
          vm_cleanupValue((Vm_Value_T *)((int)&vm_stack->ref + iVar3));
          iVar3 = iVar3 + 0x18;
        } while (iVar4 <= iVar6);
      }
      iVar5 = iVar5 + -1;
      iVar3 = iVar6 + 1;
      if (vm_sp <= iVar5) {
        iVar6 = iVar5 * 0x18;
        iVar4 = iVar3 * 0x18;
        do {
          *(uchar *)(iVar4 + -0x14 + (int)vm_stack) = (&vm_stack->type)[iVar6];
          *(uchar *)(iVar4 + -0x13 + (int)vm_stack) = (&vm_stack->attrib)[iVar6];
          pVVar2 = vm_stack;
          paVar1 = &vm_stack->v;
          (&vm_stack->type)[iVar6] = '\0';
          *(undefined4 *)(iVar4 + -0x10 + (int)pVVar2) = *(undefined4 *)((int)paVar1 + iVar6);
          (&vm_stack->attrib)[iVar6] = '\0';
          iVar5 = iVar5 + -1;
          iVar3 = iVar3 + -1;
          iVar6 = iVar6 + -0x18;
          iVar4 = iVar4 + -0x18;
        } while (vm_sp <= iVar5);
      }
    }
  }
  vm_sp = iVar3;
  return;
}
```

## 复用：vm_traceError — procedure_assembly

Evidence ID：`ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe`

```asm
0x8093520: PUSH EBP
0x8093521: MOV EBP, ESP
0x8093523: PUSH EDI
0x8093524: PUSH ESI
0x8093525: PUSH EBX
0x8093526: SUB ESP, 0x82c
0x809352c: CMP dword ptr [EBP + 0x8], -0x1
0x8093530: MOV dword ptr [EBP + 0xfffff7d8], 0x1
0x809353a: JZ 0x08093b4d
0x8093540: CMP dword ptr [0x083cc4e4], -0x1
0x8093547: MOV EAX, 0x0
0x809354c: CMOVNZ EAX, dword ptr [EBP + 0xfffff7d8]
0x8093553: MOV dword ptr [EBP + 0xfffff7d8], EAX
0x8093559: CALL 0x080a9310
0x809355e: SUB ESP, 0xc
0x8093561: PUSH dword ptr [EAX + 0x20]
0x8093564: PUSH EAX
0x8093565: PUSH 0x82ae01b
0x809356a: PUSH 0x800
0x809356f: MOV dword ptr [EBP + 0xfffff7dc], EAX
0x8093575: LEA EAX, [EBP + 0xfffff7e8]
0x809357b: PUSH EAX
0x809357c: CALL 0x0804b914
0x8093581: MOV EAX, dword ptr [EBP + 0xc]
0x8093584: ADD ESP, 0x20
0x8093587: TEST EAX, EAX
0x8093589: MOV byte ptr [EBP + -0x19], 0x0
0x809358d: JZ 0x08093c46
0x8093593: PUSH dword ptr [EBP + 0xc]
0x8093596: CLD
0x8093597: MOV ECX, 0xffffffff
0x809359c: XOR EAX, EAX
0x809359e: LEA EDI, [EBP + 0xfffff7e8]
0x80935a4: SCASB.REPNE ES:EDI
0x80935a6: NOT ECX
0x80935a8: DEC ECX
0x80935a9: PUSH ECX
0x80935aa: PUSH 0x1
0x80935ac: LEA ECX, [EBP + 0xfffff7e8]
0x80935b2: PUSH ECX
0x80935b3: CALL 0x0804bb44
0x80935b8: MOV EAX, [0x086619c0]
0x80935bd: ADD ESP, 0x10
0x80935c0: TEST EAX, EAX
0x80935c2: MOV dword ptr [EBP + 0xfffff7d4], EAX
0x80935c8: JLE 0x080939dc
0x80935ce: LEA EAX, [EAX + EAX*0x4]
0x80935d1: SHL EAX, 0x3
0x80935d4: MOV dword ptr [EBP + 0xfffff7cc], EAX
0x80935da: JMP 0x080936de
0x80935e0: MOV EAX, dword ptr [EBX + 0x20]
0x80935e3: TEST EAX, EAX
0x80935e5: MOV ESI, dword ptr [EBX + 0x24]
0x80935e8: MOV EDX, 0x82b7052
0x80935ed: JZ 0x0809362a
0x80935ef: MOV EDX, dword ptr [EAX + 0x8]
0x80935f2: TEST EDX, EDX
0x80935f4: JZ 0x08093b58
0x80935fa: SUB ESP, 0xc
0x80935fd: PUSH EDX
0x80935fe: MOV EAX, dword ptr [EAX + 0x20]
0x8093601: MOV EAX, dword ptr [EAX + 0x4]
0x8093604: PUSH dword ptr [EAX + 0xc]
0x8093607: PUSH 0x82adfdc
0x809360c: PUSH 0x100
0x8093611: PUSH 0x8483ec0
0x8093616: CALL 0x0804b914
0x809361b: ADD ESP, 0x20
0x809361e: MOV byte ptr [0x08483fbf], 0x0
0x8093625: MOV EDX, 0x8483ec0
0x809362a: PUSH EDI
0x809362b: PUSH EDX
0x809362c: PUSH dword ptr [ESI]
0x809362e: PUSH dword ptr [EBP + 0xfffff7e4]
0x8093634: PUSH dword ptr [EBP + 0xfffff7e0]
0x809363a: PUSH 0x82ae035
0x809363f: PUSH 0x800
0x8093644: LEA EAX, [EBP + 0xfffff7e8]
0x809364a: PUSH EAX
0x809364b: CALL 0x0804b914
0x8093650: MOV ECX, dword ptr [EBP + 0xc]
0x8093653: ADD ESP, 0x20
0x8093656: TEST ECX, ECX
0x8093658: MOV byte ptr [EBP + -0x19], 0x0
0x809365c: JZ 0x08093bcd
0x8093662: PUSH dword ptr [EBP + 0xc]
0x8093665: CLD
0x8093666: MOV ECX, 0xffffffff
0x809366b: LEA EDI, [EBP + 0xfffff7e8]
0x8093671: XOR EAX, EAX
0x8093673: SCASB.REPNE ES:EDI
0x8093675: NOT ECX
0x8093677: DEC ECX
0x8093678: PUSH ECX
0x8093679: PUSH 0x1
0x809367b: LEA EDX, [EBP + 0xfffff7e8]
0x8093681: PUSH EDX
0x8093682: CALL 0x0804bb44
0x8093687: MOV EDX, dword ptr [EBP + 0xfffff7d8]
0x809368d: ADD ESP, 0x10
0x8093690: TEST EDX, EDX
0x8093692: JZ 0x080936c3
0x8093694: MOV EAX, [0x083cc4e4]
0x8093699: TEST EAX, EAX
0x809369b: MOV EAX, 0x0
0x80936a0: CMOVNZ EAX, dword ptr [EBP + 0xfffff7d8]
0x80936a7: SUB ESP, 0xc
0x80936aa: PUSH dword ptr [EBP + 0xc]
0x80936ad: PUSH dword ptr [EBX + 0x20]
0x80936b0: PUSH ESI
0x80936b1: PUSH dword ptr [EBX + 0x14]
0x80936b4: PUSH EBX
0x80936b5: MOV dword ptr [EBP + 0xfffff7d8], EAX
0x80936bb: CALL 0x08092d18
0x80936c0: ADD ESP, 0x20
0x80936c3: DEC dword ptr [EBP + 0xfffff7d4]
0x80936c9: MOV EAX, dword ptr [EBP + 0xfffff7d4]
0x80936cf: SUB dword ptr [EBP + 0xfffff7cc], 0x28
0x80936d6: TEST EAX, EAX
0x80936d8: JLE 0x080939dc
0x80936de: LEA EDX, [EBP + 0xfffff7e4]
0x80936e4: PUSH EDX
0x80936e5: MOV EBX, dword ptr [EBP + 0xfffff7cc]
0x80936eb: LEA ECX, [EBP + 0xfffff7e0]
0x80936f1: ADD EBX, dword ptr [0x086619c8]
0x80936f7: PUSH ECX
0x80936f8: PUSH dword ptr [EBX + 0x10]
0x80936fb: PUSH dword ptr [EBX + 0xc]
0x80936fe: CALL 0x08097654
0x8093703: MOV EAX, dword ptr [EBX + 0x8]
0x8093706: ADD ESP, 0x10
0x8093709: TEST AL, 0x4
0x809370b: JNZ 0x080935e0
0x8093711: TEST AL, 0x1
0x8093713: JZ 0x08093918
0x8093719: MOV EAX, dword ptr [EBX + 0x20]
0x809371c: TEST EAX, EAX
0x809371e: MOV EDX, 0x82b7052
0x8093723: JZ 0x08093760
0x8093725: MOV EDX, dword ptr [EAX + 0x8]
0x8093728: TEST EDX, EDX
0x809372a: JZ 0x08093ba8
0x8093730: SUB ESP, 0xc
0x8093733: PUSH EDX
0x8093734: MOV EAX, dword ptr [EAX + 0x20]
0x8093737: MOV EAX, dword ptr [EAX + 0x4]
0x809373a: PUSH dword ptr [EAX + 0xc]
0x809373d: PUSH 0x82adfdc
0x8093742: PUSH 0x100
0x8093747: PUSH 0x8483ec0
0x809374c: CALL 0x0804b914
0x8093751: ADD ESP, 0x20
0x8093754: MOV byte ptr [0x08483fbf], 0x0
0x809375b: MOV EDX, 0x8483ec0
0x8093760: SUB ESP, 0xc
0x8093763: PUSH EDX
0x8093764: MOV EAX, dword ptr [EBX + 0x24]
0x8093767: PUSH dword ptr [EAX]
0x8093769: PUSH 0x82ae049
0x809376e: PUSH 0x800
0x8093773: LEA ECX, [EBP + 0xfffff7e8]
0x8093779: PUSH ECX
0x809377a: CALL 0x0804b914
0x809377f: MOV EAX, dword ptr [EBP + 0xc]
0x8093782: ADD ESP, 0x20
0x8093785: TEST EAX, EAX
0x8093787: MOV byte ptr [EBP + -0x19], 0x0
0x809378b: JZ 0x08093be1
0x8093791: PUSH dword ptr [EBP + 0xc]
0x8093794: CLD
0x8093795: MOV ECX, 0xffffffff
0x809379a: LEA EDI, [EBP + 0xfffff7e8]
0x80937a0: XOR EAX, EAX
0x80937a2: SCASB.REPNE ES:EDI
0x80937a4: NOT ECX
0x80937a6: DEC ECX
0x80937a7: PUSH ECX
0x80937a8: PUSH 0x1
0x80937aa: LEA EDX, [EBP + 0xfffff7e8]
0x80937b0: PUSH EDX
0x80937b1: CALL 0x0804bb44
0x80937b6: MOV EAX, dword ptr [EBP + 0xfffff7d8]
0x80937bc: ADD ESP, 0x10
0x80937bf: TEST EAX, EAX
0x80937c1: JZ 0x080936c3
0x80937c7: MOV EAX, dword ptr [EBX + 0x4]
0x80937ca: XOR EDI, EDI
0x80937cc: TEST EAX, EAX
0x80937ce: JG 0x08093865
0x80937d4: JMP 0x080936c3
0x80937dc: XOR EAX, EAX
0x80937de: CLD
0x80937df: MOV ECX, 0xffffffff
0x80937e4: LEA EDI, [EBP + 0xfffff7e8]
0x80937ea: SCASB.REPNE ES:EDI
0x80937ec: NOT ECX
0x80937ee: DEC ECX
0x80937ef: MOV dword ptr [EBP + ECX*0x1 + 0xfffff7e8], 0x6f726428
0x80937fa: MOV dword ptr [EBP + ECX*0x1 + 0xfffff7ec], 0x64657070
0x8093805: MOV word ptr [EBP + ECX*0x1 + 0xfffff7f0], 0x29
0x809380f: MOV EAX, dword ptr [EBP + 0xc]
0x8093812: TEST EAX, EAX
0x8093814: JZ 0x080938f8
0x809381a: PUSH dword ptr [EBP + 0xc]
0x809381d: CLD
0x809381e: XOR EAX, EAX
0x8093820: MOV ECX, 0xffffffff
0x8093825: LEA EDI, [EBP + 0xfffff7e8]
0x809382b: SCASB.REPNE ES:EDI
0x809382d: NOT ECX
0x809382f: DEC ECX
0x8093830: PUSH ECX
0x8093831: PUSH 0x1
0x8093833: LEA EDX, [EBP + 0xfffff7e8]
0x8093839: PUSH EDX
0x809383a: CALL 0x0804bb44
0x809383f: ADD ESP, 0x10
0x8093842: PUSH dword ptr [EBP + 0xc]
0x8093845: PUSH 0x1
0x8093847: PUSH 0x1
0x8093849: PUSH 0x82b0725
0x809384e: CALL 0x0804bb44
0x8093853: MOV EDI, dword ptr [EBP + 0xfffff7d0]
0x8093859: ADD ESP, 0x10
0x809385c: CMP dword ptr [EBX + 0x4], EDI
0x809385f: JLE 0x080936c3
0x8093865: MOV EAX, dword ptr [EBX]
0x8093867: ADD EAX, EDI
0x8093869: LEA EAX, [EAX + EAX*0x2]
0x809386c: MOV EDX, dword ptr [0x08661964]
0x8093872: LEA ECX, [EDI + 0x1]
0x8093875: LEA ESI, [EDX + EAX*0x8]
0x8093878: PUSH EAX
0x8093879: PUSH ECX
0x809387a: PUSH 0x82ae05c
0x809387f: LEA EAX, [EBP + 0xfffff7e8]
0x8093885: PUSH EAX
0x8093886: MOV dword ptr [EBP + 0xfffff7d0], ECX
0x809388c: CALL 0x0804bb34
0x8093891: ADD EDI, dword ptr [EBX]
0x8093893: ADD ESP, 0x10
0x8093896: CMP EDI, dword ptr [0x086619cc]
0x809389c: JL 0x080937dc
0x80938a2: LEA EDI, [EBP + 0xfffff7e8]
0x80938a8: CLD
0x80938a9: MOV ECX, 0xffffffff
0x80938ae: PUSH EAX
0x80938af: XOR EAX, EAX
0x80938b1: SCASB.REPNE ES:EDI
0x80938b3: NOT ECX
0x80938b5: MOV EAX, 0x801
0x80938ba: SUB EAX, ECX
0x80938bc: PUSH EAX
0x80938bd: PUSH EDI
0x80938be: PUSH ESI
0x80938bf: CALL 0x0808bd88
0x80938c4: POP EDX
0x80938c5: POP ECX
0x80938c6: PUSH EAX
0x80938c7: CLD
0x80938c8: XOR EAX, EAX
0x80938ca: MOV ECX, 0xffffffff
0x80938cf: LEA EDI, [EBP + 0xfffff7e8]
0x80938d5: SCASB.REPNE ES:EDI
0x80938d7: NOT ECX
0x80938d9: LEA ECX, [ECX + EBP*0x1 + 0xfffff7e7]
0x80938e0: PUSH ECX
0x80938e1: CALL 0x0804ba94
0x80938e6: MOV EAX, dword ptr [EBP + 0xc]
0x80938e9: ADD ESP, 0x10
0x80938ec: TEST EAX, EAX
0x80938ee: MOV byte ptr [EBP + -0x19], 0x0
0x80938f2: JNZ 0x0809381a
0x80938f8: SUB ESP, 0xc
0x80938fb: LEA EDX, [EBP + 0xfffff7e8]
0x8093901: PUSH EDX
0x8093902: CALL 0x08098110
0x8093907: MOV dword ptr [ESP], 0x82b0725
0x809390e: CALL 0x08098110
0x8093913: JMP 0x08093853
0x8093918: MOV EAX, dword ptr [EBX + 0x20]
0x809391b: TEST EAX, EAX
0x809391d: MOV EDX, 0x82b7052
0x8093922: JZ 0x0809395f
0x8093924: MOV EDX, dword ptr [EAX + 0x8]
0x8093927: TEST EDX, EDX
0x8093929: JZ 0x08093b80
0x809392f: SUB ESP, 0xc
0x8093932: PUSH EDX
0x8093933: MOV EAX, dword ptr [EAX + 0x20]
0x8093936: MOV EAX, dword ptr [EAX + 0x4]
0x8093939: PUSH dword ptr [EAX + 0xc]
0x809393c: PUSH 0x82adfdc
0x8093941: PUSH 0x100
0x8093946: PUSH 0x8483ec0
0x809394b: CALL 0x0804b914
0x8093950: ADD ESP, 0x20
0x8093953: MOV byte ptr [0x08483fbf], 0x0
0x809395a: MOV EDX, 0x8483ec0
0x809395f: PUSH EAX
0x8093960: PUSH EDX
0x8093961: PUSH dword ptr [EBX + 0x24]
0x8093964: PUSH dword ptr [EBP + 0xfffff7e4]
0x809396a: PUSH dword ptr [EBP + 0xfffff7e0]
0x8093970: PUSH 0x82ae035
0x8093975: PUSH 0x800
0x809397a: LEA ECX, [EBP + 0xfffff7e8]
0x8093980: PUSH ECX
0x8093981: CALL 0x0804b914
0x8093986: MOV EAX, dword ptr [EBP + 0xc]
0x8093989: ADD ESP, 0x20
0x809398c: TEST EAX, EAX
0x809398e: MOV byte ptr [EBP + -0x19], 0x0
0x8093992: JZ 0x08093bf5
0x8093998: PUSH dword ptr [EBP + 0xc]
0x809399b: CLD
0x809399c: XOR EAX, EAX
0x809399e: MOV ECX, 0xffffffff
0x80939a3: LEA EDI, [EBP + 0xfffff7e8]
0x80939a9: SCASB.REPNE ES:EDI
0x80939ab: NOT ECX
0x80939ad: DEC ECX
0x80939ae: PUSH ECX
0x80939af: PUSH 0x1
0x80939b1: LEA EDX, [EBP + 0xfffff7e8]
0x80939b7: PUSH EDX
0x80939b8: CALL 0x0804bb44
0x80939bd: ADD ESP, 0x10
0x80939c0: DEC dword ptr [EBP + 0xfffff7d4]
0x80939c6: MOV EAX, dword ptr [EBP + 0xfffff7d4]
0x80939cc: SUB dword ptr [EBP + 0xfffff7cc], 0x28
0x80939d3: TEST EAX, EAX
0x80939d5: JG 0x080936de
0x80939db: NOP
0x80939dc: LEA ECX, [EBP + 0xfffff7e4]
0x80939e2: PUSH ECX
0x80939e3: MOV ECX, dword ptr [EBP + 0xfffff7d4]
0x80939e9: LEA EDX, [ECX + ECX*0x4]
0x80939ec: LEA EAX, [EBP + 0xfffff7e0]
0x80939f2: PUSH EAX
0x80939f3: SHL EDX, 0x3
0x80939f6: MOV EAX, [0x086619c8]
0x80939fb: PUSH dword ptr [EDX + EAX*0x1 + 0x10]
0x80939ff: MOV EAX, dword ptr [EDX + EAX*0x1 + 0xc]
0x8093a03: DEC EAX
0x8093a04: PUSH EAX
0x8093a05: CALL 0x08097654
0x8093a0a: POP EAX
0x8093a0b: PUSH dword ptr [EBP + 0xfffff7e4]
0x8093a11: PUSH dword ptr [EBP + 0xfffff7e0]
0x8093a17: PUSH 0x82ae06b
0x8093a1c: PUSH 0x800
0x8093a21: LEA EAX, [EBP + 0xfffff7e8]
0x8093a27: PUSH EAX
0x8093a28: CALL 0x0804b914
0x8093a2d: MOV EDI, dword ptr [EBP + 0xc]
0x8093a30: ADD ESP, 0x20
0x8093a33: TEST EDI, EDI
0x8093a35: MOV byte ptr [EBP + -0x19], 0x0
0x8093a39: JZ 0x08093c32
0x8093a3f: PUSH dword ptr [EBP + 0xc]
0x8093a42: CLD
0x8093a43: MOV ECX, 0xffffffff
0x8093a48: XOR EAX, EAX
0x8093a4a: LEA EDI, [EBP + 0xfffff7e8]
0x8093a50: SCASB.REPNE ES:EDI
0x8093a52: NOT ECX
0x8093a54: DEC ECX
0x8093a55: PUSH ECX
0x8093a56: PUSH 0x1
0x8093a58: LEA ECX, [EBP + 0xfffff7e8]
0x8093a5e: PUSH ECX
0x8093a5f: CALL 0x0804bb44
0x8093a64: MOV EAX, dword ptr [EBP + 0x8]
0x8093a67: ADD ESP, 0x10
0x8093a6a: CMP EAX, dword ptr [0x086619c0]
0x8093a70: JGE 0x08093b40
0x8093a76: MOV EDX, dword ptr [EBP + 0xfffff7dc]
0x8093a7c: MOV EAX, dword ptr [EDX + 0x98]
0x8093a82: TEST EAX, EAX
0x8093a84: JLE 0x08093ab1
0x8093a86: MOV ESI, ESI
0x8093a88: LEA EAX, [EAX + EAX*0x4]
0x8093a8b: SHL EAX, 0x2
0x8093a8e: ADD EAX, dword ptr [EDX + 0xa0]
0x8093a94: MOV ECX, dword ptr [EBP + 0x8]
0x8093a97: CMP dword ptr [EAX + -0x10], ECX
0x8093a9a: JLE 0x08093ab1
0x8093a9c: CALL 0x080aae08
0x8093aa1: MOV EDX, dword ptr [EBP + 0xfffff7dc]
0x8093aa7: MOV EAX, dword ptr [EDX + 0x98]
0x8093aad: TEST EAX, EAX
0x8093aaf: JG 0x08093a88
0x8093ab1: MOV EDX, dword ptr [EBP + 0x8]
0x8093ab4: LEA EAX, [EDX + EDX*0x4]
0x8093ab7: SHL EAX, 0x3
0x8093aba: ADD EAX, dword ptr [0x086619c8]
0x8093ac0: MOV EDX, dword ptr [EAX + 0x2c]
0x8093ac3: ADD EDX, dword ptr [EAX + 0x28]
0x8093ac6: SUB ESP, 0xc
0x8093ac9: SUB EDX, dword ptr [0x086619cc]
0x8093acf: PUSH EDX
0x8093ad0: CALL 0x08091ab0
0x8093ad5: ADD ESP, 0x10
0x8093ad8: MOV ECX, dword ptr [0x086619c0]
0x8093ade: CMP ECX, dword ptr [EBP + 0x8]
0x8093ae1: JLE 0x08093b40
0x8093ae3: MOV ESI, dword ptr [0x0860ae64]
0x8093ae9: LEA EDX, [ECX + ECX*0x4]
0x8093aec: MOV EAX, [0x086619c8]
0x8093af1: TEST ESI, ESI
0x8093af3: LEA EBX, [EAX + EDX*0x8]
0x8093af6: JZ 0x08093c0c
0x8093afc: CALL 0x080bb0d4
0x8093b01: DEC dword ptr [0x086619c0]
0x8093b07: CALL 0x0809789c
0x8093b0c: CALL 0x080bb0e8
0x8093b11: MOV EAX, dword ptr [EBX + 0x8]
0x8093b14: TEST AL, 0x1
0x8093b16: JNZ 0x08093ad8
0x8093b18: MOV EDX, dword ptr [EBX + 0x20]
0x8093b1b: TEST EDX, EDX
0x8093b1d: JNZ 0x08093c1e
0x8093b23: TEST AL, 0x8
0x8093b25: JNZ 0x08093ad8
0x8093b27: MOV ECX, dword ptr [EBP + 0xfffff7dc]
0x8093b2d: AND dword ptr [ECX + 0x28], 0xfffffeff
0x8093b34: MOV ECX, dword ptr [0x086619c0]
0x8093b3a: CMP ECX, dword ptr [EBP + 0x8]
0x8093b3d: JG 0x08093ae3
0x8093b3f: NOP
0x8093b40: MOV EAX, dword ptr [EBP + 0x8]
0x8093b43: MOV [0x086619c0], EAX
0x8093b48: CALL 0x0809789c
0x8093b4d: LEA ESP, [EBP + -0xc]
0x8093b50: POP EBX
0x8093b51: POP ESI
0x8093b52: POP EDI
0x8093b53: LEAVE
0x8093b54: RET
0x8093b58: MOV EAX, dword ptr [EAX + 0x20]
0x8093b5b: MOV EAX, dword ptr [EAX + 0x4]
0x8093b5e: PUSH dword ptr [EAX + 0xc]
0x8093b61: PUSH 0x82d3f39
0x8093b66: PUSH 0x100
0x8093b6b: PUSH 0x8483ec0
0x8093b70: CALL 0x0804b914
0x8093b75: ADD ESP, 0x10
0x8093b78: JMP 0x0809361e
0x8093b80: MOV EAX, dword ptr [EAX + 0x20]
0x8093b83: MOV EAX, dword ptr [EAX + 0x4]
0x8093b86: PUSH dword ptr [EAX + 0xc]
0x8093b89: PUSH 0x82d3f39
0x8093b8e: PUSH 0x100
0x8093b93: PUSH 0x8483ec0
0x8093b98: CALL 0x0804b914
0x8093b9d: ADD ESP, 0x10
0x8093ba0: JMP 0x08093953
0x8093ba8: MOV EAX, dword ptr [EAX + 0x20]
0x8093bab: MOV EAX, dword ptr [EAX + 0x4]
0x8093bae: PUSH dword ptr [EAX + 0xc]
0x8093bb1: PUSH 0x82d3f39
0x8093bb6: PUSH 0x100
0x8093bbb: PUSH 0x8483ec0
0x8093bc0: CALL 0x0804b914
0x8093bc5: ADD ESP, 0x10
0x8093bc8: JMP 0x08093754
0x8093bcd: SUB ESP, 0xc
0x8093bd0: LEA EDX, [EBP + 0xfffff7e8]
0x8093bd6: PUSH EDX
0x8093bd7: CALL 0x08098110
0x8093bdc: JMP 0x08093687
0x8093be1: SUB ESP, 0xc
0x8093be4: LEA EAX, [EBP + 0xfffff7e8]
0x8093bea: PUSH EAX
0x8093beb: CALL 0x08098110
0x8093bf0: JMP 0x080937b6
0x8093bf5: SUB ESP, 0xc
0x8093bf8: LEA EAX, [EBP + 0xfffff7e8]
0x8093bfe: PUSH EAX
0x8093bff: CALL 0x08098110
0x8093c04: ADD ESP, 0x10
0x8093c07: JMP 0x080939c0
0x8093c0c: LEA EAX, [ECX + -0x1]
0x8093c0f: MOV [0x086619c0], EAX
0x8093c14: CALL 0x0809789c
0x8093c19: JMP 0x08093b11
0x8093c1e: SUB ESP, 0xc
0x8093c21: PUSH EDX
0x8093c22: CALL 0x080b1804
0x8093c27: ADD ESP, 0x10
0x8093c2a: MOV EAX, dword ptr [EBX + 0x8]
0x8093c2d: JMP 0x08093b23
0x8093c32: SUB ESP, 0xc
0x8093c35: LEA EDX, [EBP + 0xfffff7e8]
0x8093c3b: PUSH EDX
0x8093c3c: CALL 0x08098110
0x8093c41: JMP 0x08093a64
0x8093c46: SUB ESP, 0xc
0x8093c49: LEA EDX, [EBP + 0xfffff7e8]
0x8093c4f: PUSH EDX
0x8093c50: CALL 0x08098110
0x8093c55: JMP 0x080935b8
```

## 复用：vm_traceError — procedure_pseudo_code

Evidence ID：`ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4`

```c

/* WARNING: Unknown calling convention */

void vm_traceError(int toFunCp,FILE *fp)

{
  Vm_Value_T *pValue;
  char cVar1;
  Vm_Fun_T *pFun;
  Vm_Object *pVVar2;
  bool bVar3;
  Vm_PCB_T *pVVar4;
  size_t __n;
  char *__src;
  int iVar5;
  uint uVar6;
  char *pcVar7;
  Vm_FunCall_Context_T *pVVar8;
  char *unaff_EDI;
  bool bVar9;
  int local_838;
  int i;
  int flag;
  Vm_PCB_T *pPCB;
  char *file;
  int line;
  char buf [2048];
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)] */
  if (toFunCp != -1) {
    bVar9 = _vm_showFullFrameWhenError != -1;
    pVVar4 = vm_getCurrentPCB();
    snprintf(buf,0x800,"Current thread: \"%s\"(%u)\n",pVVar4,pVVar4->tid);
    buf[0x7ff] = '\0';
    if (fp == (FILE *)0x0) {
      vm_errorOut(buf);
    }
    else {
      uVar6 = 0xffffffff;
      pcVar7 = buf;
      do {
        unaff_EDI = pcVar7;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        unaff_EDI = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = unaff_EDI;
      } while (cVar1 != '\0');
      fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
    }
    i = vm_funCp;
    if (0 < vm_funCp) {
      local_838 = vm_funCp * 0x28;
      do {
        while( true ) {
          pVVar8 = (Vm_FunCall_Context_T *)((int)&vm_funContext->bp + local_838);
          vm_getSymbolInfoFromNode(pVVar8->ip,pVVar8->cs,&file,&line);
          if ((pVVar8->flags & 4) == 0) break;
                    /* Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg6(ESI)] */
          pVVar2 = pVVar8->thisObject;
          pFun = (Vm_Fun_T *)(pVVar8->f).pEfun;
          pcVar7 = "global";
          if (pVVar2 != (Vm_Object *)0x0) {
            if (pVVar2->oid == 0) {
              snprintf(vm_objectName::buf,0x100,"%s",pVVar2->pProgram->strName->ptr);
            }
            else {
              snprintf(vm_objectName::buf,0x100,"%s#%d",pVVar2->pProgram->strName->ptr,pVVar2->oid);
            }
            vm_objectName::buf[0xff] = '\0';
            pcVar7 = vm_objectName::buf;
          }
          snprintf(buf,0x800,"At %s:%d(%s) in %s\n",file,line,pFun->name,pcVar7,unaff_EDI);
          buf[0x7ff] = '\0';
          if (fp == (FILE *)0x0) {
            vm_errorOut(buf);
          }
          else {
            uVar6 = 0xffffffff;
            pcVar7 = buf;
            do {
              unaff_EDI = pcVar7;
              if (uVar6 == 0) break;
              uVar6 = uVar6 - 1;
              unaff_EDI = pcVar7 + 1;
              cVar1 = *pcVar7;
              pcVar7 = unaff_EDI;
            } while (cVar1 != '\0');
            fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
          }
          if (bVar9) {
            bVar3 = false;
            if (_vm_showFullFrameWhenError != 0) {
              bVar3 = bVar9;
            }
            vm_showFunContext(pVVar8,pVVar8->consititution,pFun,pVVar8->thisObject,fp);
            bVar9 = bVar3;
          }
LAB_080936c3:
          i = i + -1;
          local_838 = local_838 + -0x28;
          if (i < 1) goto LAB_080939dc;
        }
        if ((pVVar8->flags & 1) != 0) {
          pVVar2 = pVVar8->thisObject;
          pcVar7 = "global";
          if (pVVar2 != (Vm_Object *)0x0) {
            if (pVVar2->oid == 0) {
              snprintf(vm_objectName::buf,0x100,"%s",pVVar2->pProgram->strName->ptr);
            }
            else {
              snprintf(vm_objectName::buf,0x100,"%s#%d",pVVar2->pProgram->strName->ptr,pVVar2->oid);
            }
            vm_objectName::buf[0xff] = '\0';
            pcVar7 = vm_objectName::buf;
          }
          snprintf(buf,0x800,"At EFUN: %s in %s\n",((pVVar8->f).pEfun)->word,pcVar7);
          buf[0x7ff] = '\0';
          if (fp == (FILE *)0x0) {
            vm_errorOut(buf);
          }
          else {
            uVar6 = 0xffffffff;
            pcVar7 = buf;
            do {
              unaff_EDI = pcVar7;
              if (uVar6 == 0) break;
              uVar6 = uVar6 - 1;
              unaff_EDI = pcVar7 + 1;
              cVar1 = *pcVar7;
              pcVar7 = unaff_EDI;
            } while (cVar1 != '\0');
            fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
          }
                    /* Unresolved local var: int k@[DW_OP_reg7(EDI)] */
          if ((bVar9) && (unaff_EDI = (char *)0x0, pcVar7 = unaff_EDI, 0 < pVVar8->argNum)) {
            do {
              unaff_EDI = pcVar7 + 1;
              pValue = vm_stack + (int)(pcVar7 + pVVar8->bp);
              sprintf(buf,"Argument %d = ",unaff_EDI,(int)(pcVar7 + pVVar8->bp) * 3);
              if ((int)(pcVar7 + pVVar8->bp) < vm_sp) {
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                builtin_strncpy(buf + (~uVar6 - 1),"(dro",4);
                builtin_strncpy(buf + ~uVar6 + 3,"pped)",6);
                if (fp != (FILE *)0x0) goto LAB_0809381a;
LAB_080938f8:
                vm_errorOut(buf);
                vm_errorOut("\n");
              }
              else {
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                __n = 0x801 - ~uVar6;
                __src = vm_typeValue(pValue);
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                strncpy(buf + (~uVar6 - 1),__src,__n);
                buf[0x7ff] = '\0';
                if (fp == (FILE *)0x0) goto LAB_080938f8;
LAB_0809381a:
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
                fwrite("\n",1,1,(FILE *)fp);
              }
              pcVar7 = unaff_EDI;
            } while ((int)unaff_EDI < pVVar8->argNum);
          }
          goto LAB_080936c3;
        }
        pVVar2 = pVVar8->thisObject;
        pcVar7 = "global";
        iVar5 = 0;
        if (pVVar2 != (Vm_Object *)0x0) {
          if (pVVar2->oid == 0) {
            iVar5 = snprintf(vm_objectName::buf,0x100,"%s",pVVar2->pProgram->strName->ptr);
          }
          else {
            iVar5 = snprintf(vm_objectName::buf,0x100,"%s#%d",pVVar2->pProgram->strName->ptr,
                             pVVar2->oid);
          }
          vm_objectName::buf[0xff] = '\0';
          pcVar7 = vm_objectName::buf;
        }
        snprintf(buf,0x800,"At %s:%d(%s) in %s\n",file,line,(pVVar8->f).pEfun,pcVar7,iVar5);
        buf[0x7ff] = '\0';
        if (fp == (FILE *)0x0) {
          vm_errorOut(buf);
        }
        else {
          uVar6 = 0xffffffff;
          pcVar7 = buf;
          do {
            unaff_EDI = pcVar7;
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            unaff_EDI = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = unaff_EDI;
          } while (cVar1 != '\0');
          fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
        }
        i = i + -1;
        local_838 = local_838 + -0x28;
      } while (0 < i);
    }
LAB_080939dc:
    vm_getSymbolInfoFromNode(vm_funContext[i].ip + -1,vm_funContext[i].cs,&file,&line);
    snprintf(buf,0x800,"Call from %s:%d\n",file,line);
    buf[0x7ff] = '\0';
    if (fp == (FILE *)0x0) {
      vm_errorOut(buf);
    }
    else {
      uVar6 = 0xffffffff;
      pcVar7 = buf;
      do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
    }
    if (toFunCp < vm_funCp) {
                    /* Unresolved local var: int n@[???] */
      iVar5 = pVVar4->catchCp;
      while ((0 < iVar5 && (toFunCp < pVVar4->catchContext[iVar5 + -1].funCp))) {
        vm_endThreadCatch();
        iVar5 = pVVar4->catchCp;
      }
      vm_popNStack((vm_funContext[toFunCp + 1].argNum + vm_funContext[toFunCp + 1].bp) - vm_sp);
LAB_08093ad8:
      if (toFunCp < vm_funCp) {
        while( true ) {
          pVVar8 = vm_funContext;
          iVar5 = vm_funCp;
          if (vm_gatheringCallStackInfo == 0) {
            vm_funCp = vm_funCp + -1;
            vm_updateCurrentFunCallContext();
          }
          else {
            vm_enterPPCopyStackCriticalSection();
            vm_funCp = vm_funCp + -1;
            vm_updateCurrentFunCallContext();
            vm_leavePPCopyStackCriticalSection();
          }
          uVar6 = pVVar8[iVar5].flags;
          if ((uVar6 & 1) != 0) break;
          if (pVVar8[iVar5].thisObject != (Vm_Object_T *)0x0) {
            vm_unExecRefObject(pVVar8[iVar5].thisObject);
            uVar6 = pVVar8[iVar5].flags;
          }
          if ((uVar6 & 8) != 0) break;
          pVVar4->flags = pVVar4->flags & 0xfffffeff;
          if (vm_funCp <= toFunCp) goto LAB_08093b40;
        }
        goto LAB_08093ad8;
      }
    }
LAB_08093b40:
    vm_funCp = toFunCp;
    vm_updateCurrentFunCallContext();
  }
  return;
}
```
