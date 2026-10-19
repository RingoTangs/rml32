# 阶段 2AD 完整证据可读视图

本文件由完整记录生成；伪代码与汇编是 Ghidra 观察，不是原始 C 源码。全部限制保留于压缩包和下列条目。

## search_procedures / search_procedures

Evidence: `ev_79ac37034107636b07d878802b6dbbe34a7813cc88652bbbbd6c9b0be3987e03`

Address: `None`; source: `docs/stage2ad-evidence/rea.json.gz`

```json
[
  {
    "address": "0x808aab0",
    "value": "vm_clearStack"
  },
  {
    "address": "0x808cbf4",
    "value": "vm_setCurrentArgument"
  },
  {
    "address": "0x809061c",
    "value": "vm_popStack"
  },
  {
    "address": "0x8090b90",
    "value": "vm_pushReal"
  },
  {
    "address": "0x8090be4",
    "value": "vm_pushRealFromWord"
  },
  {
    "address": "0x8091ab0",
    "value": "vm_popNStack"
  },
  {
    "address": "0x8092540",
    "value": "vm_pushRealOfPCB"
  },
  {
    "address": "0x80933e4",
    "value": "vm_stackInThisContext"
  },
  {
    "address": "0x8093400",
    "value": "vm_argumentInThisContext"
  },
  {
    "address": "0x80a9f1c",
    "value": "vm_popPCBStack"
  },
  {
    "address": "0x80bc84c",
    "value": "vm_popLoopContext"
  },
  {
    "address": "0x8138280",
    "value": "sqlite3VdbeFifoPop"
  },
  {
    "address": "0x813dfb0",
    "value": "sqlite3AuthContextPop"
  },
  {
    "address": "0x8156500",
    "value": "yy_pop_parser_stack"
  },
  {
    "address": "0x81607c0",
    "value": "popStack"
  },
  {
    "address": "0x8247a00",
    "value": "pop_dynamic"
  },
  {
    "address": "0x8299674",
    "value": "glPopMatrix"
  },
  {
    "address": "0x8299b14",
    "value": "glPopName"
  },
  {
    "address": "0x829b37c",
    "value": "glopPopMatrix"
  },
  {
    "address": "0x829df58",
    "value": "glopPopName"
  }
]
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Regex mode uses Java Pattern semantics. Matcher or compiler stack exhaustion returns a resource constraint without closing the session; literal mode avoids regex recursion and returns complete matching values.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## search_procedures / search_procedures

Evidence: `ev_d7b8cc47c3371f8d68aea079c0f4b3444c09f8156377ffcd272e5791a43d90eb`

Address: `None`; source: `docs/stage2ad-evidence/rea.json.gz`

```json
[
  {
    "address": "0x804c960",
    "value": "vm_initEfun"
  },
  {
    "address": "0x804c9d8",
    "value": "vm_shutdownEfun"
  },
  {
    "address": "0x8085ce4",
    "value": "vm_addEfuns"
  },
  {
    "address": "0x808600c",
    "value": "vm_updateEfuns"
  },
  {
    "address": "0x80874d4",
    "value": "vm_registerEfunModule"
  },
  {
    "address": "0x808761c",
    "value": "vm_buildEfunOffset"
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
    "address": "0x8093400",
    "value": "vm_argumentInThisContext"
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
    "address": "0x8094178",
    "value": "vm_callInFakeFun"
  },
  {
    "address": "0x8094374",
    "value": "vm_returnFromFakeFun"
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
    "address": "0x8095794",
    "value": "vm_finalizeFun"
  },
  {
    "address": "0x8095848",
    "value": "vm_saveFun"
  },
  {
    "address": "0x8095c88",
    "value": "vm_freeFun"
  },
  {
    "address": "0x80a21f0",
    "value": "vm_overrideFunction"
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
    "address": "0x80c2e94",
    "value": "vm_resourceEfunInit"
  },
  {
    "address": "0x80c2ea8",
    "value": "vm_resourceEfunDestruct"
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
    "address": "0x80edc10",
    "value": "vm_f_previous_arguments"
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
    "address": "0x8128ff0",
    "value": "vm_shutdownSqliteEfun"
  },
  {
    "address": "0x8129000",
    "value": "vm_initSqliteEfun"
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
    "address": "0x814f3c0",
    "value": "quoteFunc"
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
    "address": "0x817a260",
    "value": "vm_shutdownLanguageEfun"
  },
  {
    "address": "0x817a270",
    "value": "vm_initLanguageEfun"
  }
]
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Regex mode uses Java Pattern semantics. Matcher or compiler stack exhaustion returns a resource constraint without closing the session; literal mode avoids regex recursion and returns complete matching values.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_addEfuns / analyze_function

Evidence: `ev_a83bf890b4b1b7b1e670ffd84e2caf433106c4b1521c658ee8c18bbdb826b9a1`

Address: `0x8085ce4`; source: `docs/compatibility-evidence/main-rea.json.gz`

```c

/* WARNING: Unknown calling convention */

void vm_addEfuns(Vm_Efun_T *elist)

{
  Vm_Ident_Hash_Elem *pVVar1;
  Vm_Program_T *pVVar2;
  ushort uVar3;
  Vm_Program_T *pVVar4;
  short sVar5;
  Vm_Program_T *pVVar6;
  Vm_Ident_Hash_Elem *pVVar7;
  uint uVar8;
  int local_14;
  
                    /* Unresolved local var: int i@[???] */
  local_14 = 0;
  for (pVVar7 = (Vm_Ident_Hash_Elem *)elist; pVVar7->name != (char *)0x0;
      pVVar7 = (Vm_Ident_Hash_Elem *)&(pVVar7->ii).pInContext) {
                    /* Unresolved local var: int h@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNode@[DW_OP_reg1(ECX)] */
    uVar3 = vm_whashstr(pVVar7->name,0x20);
    pVVar2 = vm_pGlobalProgram;
    uVar8 = uVar3 & 0x3ff;
    pVVar1 = vm_pGlobalProgram->ppHashTable[uVar8];
    if (pVVar1 == (Vm_Ident_Hash_Elem *)0x0) {
      vm_pGlobalProgram->ppHashHead[uVar8] = pVVar7;
      pVVar7->pNext = pVVar7;
      pVVar1 = pVVar7;
    }
    else {
      pVVar7->pNext = pVVar1->pNext;
      pVVar1->pNext->ppPrev = &pVVar7->pNext;
      pVVar1->pNext = pVVar7;
    }
    pVVar7->ppPrev = &pVVar1->pNext;
    pVVar6 = vm_pCurrentProgram;
    pVVar2->ppHashTable[uVar8] = pVVar7;
    pVVar7->semValue = pVVar7->semValue + 1;
    pVVar4 = pVVar6;
    if (pVVar6 == (Vm_Program_T *)0x0) {
      pVVar4 = pVVar2;
    }
    sVar5 = pVVar4->maxFunctionNo;
    pVVar7->token = 0x40000000;
    if (sVar5 == 0x7fff) {
      yyerrors("Too many functions (%d) in a program (> %d).\n",(int)pVVar4->maxFunctionNo,0x7fff,
               pVVar6);
      sVar5 = -1;
    }
    else {
      if (pVVar6 == (Vm_Program_T *)0x0) {
        pVVar6 = pVVar2;
      }
      sVar5 = pVVar6->maxFunctionNo;
      pVVar6->maxFunctionNo = sVar5 + 1;
    }
    *(short *)((int)&elist->funOffset + local_14) = sVar5;
    local_14 = local_14 + 0x1c;
  }
  return;
}


```

```text
0x8085ce4: PUSH EBP
0x8085ce5: MOV EBP, ESP
0x8085ce7: PUSH EDI
0x8085ce8: PUSH ESI
0x8085ce9: PUSH EBX
0x8085cea: SUB ESP, 0xc
0x8085ced: MOV EBX, dword ptr [EBP + 0x8]
0x8085cf0: MOV dword ptr [EBP + -0x10], 0x0
0x8085cf7: JMP 0x08085d21
0x8085cfc: TEST EDX, EDX
0x8085cfe: CMOVZ EDX, ESI
0x8085d01: MOV AX, word ptr [EDX + 0x10]
0x8085d05: MOVSX ECX, AX
0x8085d08: INC EAX
0x8085d09: MOV word ptr [EDX + 0x10], AX
0x8085d0d: MOV EAX, dword ptr [EBP + -0x10]
0x8085d10: MOV EDX, dword ptr [EBP + 0x8]
0x8085d13: MOV word ptr [EAX + EDX*0x1 + 0x1a], CX
0x8085d18: ADD EAX, 0x1c
0x8085d1b: MOV dword ptr [EBP + -0x10], EAX
0x8085d1e: ADD EBX, 0x1c
0x8085d21: MOV EAX, dword ptr [EBX]
0x8085d23: TEST EAX, EAX
0x8085d25: JZ 0x08085dc2
0x8085d2b: SUB ESP, 0x8
0x8085d2e: PUSH 0x20
0x8085d30: PUSH EAX
0x8085d31: CALL 0x08089dc8
0x8085d36: MOV EDI, EAX
0x8085d38: MOV ESI, dword ptr [0x0847aad0]
0x8085d3e: AND EDI, 0x3ff
0x8085d44: MOV EAX, dword ptr [ESI + 0x54]
0x8085d47: MOV ECX, dword ptr [EAX + EDI*0x4]
0x8085d4a: ADD ESP, 0x10
0x8085d4d: TEST ECX, ECX
0x8085d4f: JZ 0x08085db4
0x8085d51: MOV EAX, dword ptr [ECX + 0xc]
0x8085d54: MOV dword ptr [EBX + 0xc], EAX
0x8085d57: MOV EDX, dword ptr [ECX + 0xc]
0x8085d5a: LEA EAX, [EBX + 0xc]
0x8085d5d: MOV dword ptr [EDX + 0x10], EAX
0x8085d60: MOV dword ptr [ECX + 0xc], EBX
0x8085d63: LEA EAX, [ECX + 0xc]
0x8085d66: MOV dword ptr [EBX + 0x10], EAX
0x8085d69: MOV EAX, dword ptr [ESI + 0x54]
0x8085d6c: MOV EDX, dword ptr [0x0847aac0]
0x8085d72: MOV dword ptr [EAX + EDI*0x4], EBX
0x8085d75: INC dword ptr [EBX + 0x8]
0x8085d78: MOV EAX, EDX
0x8085d7a: TEST EDX, EDX
0x8085d7c: CMOVZ EAX, ESI
0x8085d7f: CMP word ptr [EAX + 0x10], 0x7fff
0x8085d85: MOV dword ptr [EBX + 0x4], 0x40000000
0x8085d8c: JNZ 0x08085cfc
0x8085d92: MOVSX EAX, word ptr [EAX + 0x10]
0x8085d96: PUSH EDX
0x8085d97: PUSH 0x7fff
0x8085d9c: PUSH EAX
0x8085d9d: PUSH 0x82ab320
0x8085da2: CALL 0x080ff97c
0x8085da7: ADD ESP, 0x10
0x8085daa: MOV ECX, 0xffffffff
0x8085daf: JMP 0x08085d0d
0x8085db4: MOV EAX, dword ptr [ESI + 0x58]
0x8085db7: MOV dword ptr [EAX + EDI*0x4], EBX
0x8085dba: LEA EAX, [EBX + 0xc]
0x8085dbd: MOV dword ptr [EBX + 0xc], EBX
0x8085dc0: JMP 0x08085d66
0x8085dc2: LEA ESP, [EBP + -0xc]
0x8085dc5: POP EBX
0x8085dc6: POP ESI
0x8085dc7: POP EDI
0x8085dc8: LEAVE
0x8085dc9: RET
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_callInEfun / analyze_function

Evidence: `ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f`

Address: `0x8093d98`; source: `docs/compatibility-evidence/main-rea.json.gz`

```c

void vm_callInEfun(Vm_Index_T funOffset,int argn)

{
  Vm_Fun *pVVar1;
  Vm_FunCall_Context_T *pVVar2;
  Vm_PCB_T *pVVar3;
  int iVar4;
  Vm_FunCall_Context_T *pVVar5;
  Vm_FunCall_Context_T *pVVar6;
  bool bVar7;
  int argn_local;
  
                    /* Unresolved local var: Vm_Efun_T * pEfun@[DW_OP_reg3(EBX)] */
  if (vm_funContextSize <= vm_funCp) {
    vm_error("Too depth (%d) function call!\n",vm_funCp);
  }
  if ((funOffset < 0) || (vm_pGlobalProgram->maxFunctionNo <= funOffset)) {
    vm_error("External function (index = %d) is not implemented yet.\n",(int)funOffset);
    pVVar1 = vm_pGlobalProgram->pFunctions[funOffset].pFun;
  }
  else {
    pVVar1 = vm_pGlobalProgram->pFunctions[funOffset].pFun;
  }
  if (vm_gatheringCallStackInfo != 0) {
    vm_enterPPCopyStackCriticalSection();
  }
  vm_funCp = vm_funCp + 1;
  vm_updateCurrentFunCallContext();
  pVVar2 = vm_currentContext;
  pVVar5 = vm_currentContext + -1;
  pVVar6 = vm_currentContext;
  for (iVar4 = 10; iVar4 != 0; iVar4 = iVar4 + -1) {
    pVVar6->bp = pVVar5->bp;
    pVVar5 = (Vm_FunCall_Context_T *)&pVVar5->argNum;
    pVVar6 = (Vm_FunCall_Context_T *)&pVVar6->argNum;
  }
  pVVar2->bp = vm_sp;
  pVVar2->argNum = argn;
  bVar7 = vm_gatheringCallStackInfo != 0;
  pVVar2->flags = 1;
  (pVVar2->f).pEfun = (Vm_Efun *)pVVar1;
  if (bVar7) {
    vm_leavePPCopyStackCriticalSection();
  }
  (*(code *)(pVVar1->codeSegment).nodes)();
  if (vm_ctrlFlags != 0) {
    if ((vm_ctrlFlags & 0x10) == 0) {
      if ((vm_ctrlFlags & 4) == 0) goto LAB_08093e3c;
      pVVar3 = vm_getCurrentPCB();
      pVVar3->flags = pVVar3->flags | 0x40;
    }
    return;
  }
LAB_08093e3c:
  vm_returnFromEfun();
  return;
}


```

```text
0x8093d98: PUSH EBP
0x8093d99: MOV EBP, ESP
0x8093d9b: PUSH EDI
0x8093d9c: PUSH ESI
0x8093d9d: PUSH EBX
0x8093d9e: SUB ESP, 0xc
0x8093da1: MOV EAX, dword ptr [EBP + 0xc]
0x8093da4: MOV dword ptr [EBP + -0x10], EAX
0x8093da7: MOV EAX, [0x086619c0]
0x8093dac: CMP EAX, dword ptr [0x086619c4]
0x8093db2: MOV EDI, dword ptr [EBP + 0x8]
0x8093db5: JGE 0x08093e98
0x8093dbb: TEST DI, DI
0x8093dbe: JS 0x08093e48
0x8093dc4: MOV EAX, [0x0847aad0]
0x8093dc9: CMP word ptr [EAX + 0x10], DI
0x8093dcd: MOVSX ESI, DI
0x8093dd0: JLE 0x08093e48
0x8093dd2: MOV EAX, dword ptr [EAX + 0x2c]
0x8093dd5: MOV EBX, dword ptr [EAX + ESI*0x8]
0x8093dd8: MOV EAX, [0x0860ae64]
0x8093ddd: TEST EAX, EAX
0x8093ddf: JNZ 0x08093e74
0x8093de5: LEA ESI, [ESI]
0x8093de8: INC dword ptr [0x086619c0]
0x8093dee: CALL 0x0809789c
0x8093df3: MOV EDX, dword ptr [0x086619d8]
0x8093df9: LEA ESI, [EDX + -0x28]
0x8093dfc: CLD
0x8093dfd: MOV ECX, 0xa
0x8093e02: MOV EDI, EDX
0x8093e04: MOVSD.REP ES:EDI, ESI
0x8093e06: MOV EAX, [0x086619cc]
0x8093e0b: MOV dword ptr [EDX], EAX
0x8093e0d: MOV EAX, dword ptr [EBP + -0x10]
0x8093e10: MOV dword ptr [EDX + 0x4], EAX
0x8093e13: MOV EAX, [0x0860ae64]
0x8093e18: TEST EAX, EAX
0x8093e1a: MOV dword ptr [EDX + 0x8], 0x1
0x8093e21: MOV dword ptr [EDX + 0x24], EBX
0x8093e24: JNZ 0x08093e90
0x8093e26: CALL dword ptr [EBX + 0x14]
0x8093e29: MOV EAX, [0x086619d4]
0x8093e2e: TEST EAX, EAX
0x8093e30: JZ 0x08093e3c
0x8093e32: TEST AL, 0x10
0x8093e34: JNZ 0x08093e88
0x8093e36: TEST AL, 0x4
0x8093e38: JNZ 0x08093e7e
0x8093e3a: MOV ESI, ESI
0x8093e3c: LEA ESP, [EBP + -0xc]
0x8093e3f: POP EBX
0x8093e40: POP ESI
0x8093e41: POP EDI
0x8093e42: LEAVE
0x8093e43: JMP 0x08093c5c
0x8093e48: SUB ESP, 0x8
0x8093e4b: MOVSX ESI, DI
0x8093e4e: PUSH ESI
0x8093e4f: PUSH 0x82ae2e4
0x8093e54: CALL 0x08097b24
0x8093e59: MOV EAX, [0x0847aad0]
0x8093e5e: MOV EAX, dword ptr [EAX + 0x2c]
0x8093e61: MOV EBX, dword ptr [EAX + ESI*0x8]
0x8093e64: MOV EAX, [0x0860ae64]
0x8093e69: ADD ESP, 0x10
0x8093e6c: TEST EAX, EAX
0x8093e6e: JZ 0x08093de8
0x8093e74: CALL 0x080bb0d4
0x8093e79: JMP 0x08093de8
0x8093e7e: CALL 0x080a9310
0x8093e83: OR dword ptr [EAX + 0x28], 0x40
0x8093e87: NOP
0x8093e88: LEA ESP, [EBP + -0xc]
0x8093e8b: POP EBX
0x8093e8c: POP ESI
0x8093e8d: POP EDI
0x8093e8e: LEAVE
0x8093e8f: RET
0x8093e90: CALL 0x080bb0e8
0x8093e95: JMP 0x08093e26
0x8093e98: SUB ESP, 0x8
0x8093e9b: PUSH EAX
0x8093e9c: PUSH 0x82ae31c
0x8093ea1: CALL 0x08097b24
0x8093ea6: ADD ESP, 0x10
0x8093ea9: JMP 0x08093dbb
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- Ghidra Listing reports a terminal call at 0x8093e43 to 0x8093c5c vm_returnFromEfun (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_checkArgNum / analyze_function

Evidence: `ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126`

Address: `0x809521c`; source: `docs/stage2ad-evidence/rea.json.gz`

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

```text
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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_clearStack / analyze_function

Evidence: `ev_a7818c3b6ea7809db35bb023b25656cf263eb9244c344a8d13112a18e426c0fa`

Address: `0x808aab0`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_clearStack(void)

{
  if (vm_sp < vm_stackSize) {
    do {
      vm_freeStackValue(vm_sp);
      vm_stack[vm_sp].type = '\0';
      vm_stack[vm_sp].attrib = '\0';
      vm_sp = vm_sp + 1;
    } while (vm_sp < vm_stackSize);
  }
  return;
}


```

```text
0x808aab0: PUSH EBP
0x808aab1: MOV EBP, ESP
0x808aab3: SUB ESP, 0x8
0x808aab6: MOV EAX, [0x086619cc]
0x808aabb: CMP EAX, dword ptr [0x08661960]
0x808aac1: JGE 0x0808ab09
0x808aac3: NOP
0x808aac4: SUB ESP, 0xc
0x808aac7: PUSH EAX
0x808aac8: CALL 0x08091fec
0x808aacd: MOV EAX, [0x086619cc]
0x808aad2: MOV EDX, dword ptr [0x08661964]
0x808aad8: LEA EAX, [EAX + EAX*0x2]
0x808aadb: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x808aae0: MOV EAX, [0x086619cc]
0x808aae5: LEA EAX, [EAX + EAX*0x2]
0x808aae8: MOV EDX, dword ptr [0x08661964]
0x808aaee: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x808aaf3: MOV EAX, [0x086619cc]
0x808aaf8: INC EAX
0x808aaf9: ADD ESP, 0x10
0x808aafc: MOV [0x086619cc], EAX
0x808ab01: CMP EAX, dword ptr [0x08661960]
0x808ab07: JL 0x0808aac4
0x808ab09: LEAVE
0x808ab0a: RET
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_doActions / procedure_assembly

Evidence: `ev_a280841df7ce885805c45ba2b70f60cf4628279d7dbb748b1f816ba99a1ad2c4`

Address: `0x8097008`; source: `docs/vm-catch-evidence/rea.json.gz`

```text
0x8097008: PUSH EBP
0x8097009: MOV EBP, ESP
0x809700b: SUB ESP, 0x8
0x809700e: MOV EDX, dword ptr [EBP + 0xc]
0x8097011: TEST EDX, EDX
0x8097013: MOV EAX, 0x2710
0x8097018: CMOVNZ EAX, EDX
0x809701b: MOV [0x083cc4ec], EAX
0x8097020: MOV EAX, dword ptr [EBP + 0x8]
0x8097023: TEST byte ptr [EAX + 0x2a], 0x20
0x8097027: JNZ 0x0809725e
0x809702d: SUB ESP, 0x8
0x8097030: PUSH dword ptr [0x083cc4ec]
0x8097036: PUSH 0x0
0x8097038: CALL 0x080a3854
0x809703d: LEA ESI, [ESI]
0x8097040: ADD ESP, 0x10
0x8097043: MOV EDX, dword ptr [EBP + 0x8]
0x8097046: MOV EAX, dword ptr [EDX + 0x98]
0x809704c: XOR EDX, EDX
0x809704e: TEST EAX, EAX
0x8097050: JLE 0x08097066
0x8097052: MOV ESI, ESI
0x8097054: LEA EAX, [EAX + EAX*0x4]
0x8097057: SHL EAX, 0x2
0x809705a: MOV ECX, dword ptr [EBP + 0x8]
0x809705d: ADD EAX, dword ptr [ECX + 0xa0]
0x8097063: MOV EDX, dword ptr [EAX + -0x10]
0x8097066: SUB ESP, 0x18
0x8097069: PUSH 0x1
0x809706b: PUSH EDX
0x809706c: CALL 0x080aae68
0x8097071: ADD ESP, 0x14
0x8097074: PUSH EAX
0x8097075: CALL 0x0804bac4
0x809707a: ADD ESP, 0x10
0x809707d: TEST EAX, EAX
0x809707f: JZ 0x08097118
0x8097085: CMP EAX, -0x3
0x8097088: MOV dword ptr [EBP + -0x8], EAX
0x809708b: JNZ 0x080971e8
0x8097091: MOV ECX, dword ptr [EBP + 0x8]
0x8097094: MOV EAX, dword ptr [ECX + 0x98]
0x809709a: TEST EAX, EAX
0x809709c: JLE 0x080971e8
0x80970a2: DEC EAX
0x80970a3: MOV dword ptr [ECX + 0x98], EAX
0x80970a9: MOV EDX, dword ptr [ECX + 0xa0]
0x80970af: LEA EAX, [EAX + EAX*0x4]
0x80970b2: LEA EAX, [EDX + EAX*0x4]
0x80970b5: MOV dword ptr [EBP + -0x4], EAX
0x80970b8: MOV EDX, dword ptr [0x086619cc]
0x80970be: MOV EAX, dword ptr [EAX + 0x8]
0x80970c1: CMP EAX, EDX
0x80970c3: MOV dword ptr [EBP + -0x8], 0x0
0x80970ca: JG 0x080971f0
0x80970d0: MOV ECX, dword ptr [EBP + -0x4]
0x80970d3: MOV EDX, dword ptr [ECX]
0x80970d5: CMP EDX, -0x1
0x80970d8: JZ 0x0809720c
0x80970de: MOV EAX, [0x086619d8]
0x80970e3: MOV dword ptr [EAX + 0xc], EDX
0x80970e6: MOV EDX, dword ptr [EBP + 0x8]
0x80970e9: MOV EAX, dword ptr [EDX + 0x98]
0x80970ef: XOR EDX, EDX
0x80970f1: TEST EAX, EAX
0x80970f3: JG 0x08097054
0x80970f9: SUB ESP, 0x18
0x80970fc: PUSH 0x1
0x80970fe: PUSH EDX
0x80970ff: CALL 0x080aae68
0x8097104: ADD ESP, 0x14
0x8097107: PUSH EAX
0x8097108: CALL 0x0804bac4
0x809710d: ADD ESP, 0x10
0x8097110: TEST EAX, EAX
0x8097112: JNZ 0x08097085
0x8097118: MOV EDX, dword ptr [EBP + 0x8]
0x809711b: MOV EAX, dword ptr [EDX + 0x28]
0x809711e: TEST AL, 0x40
0x8097120: JNZ 0x080972a4
0x8097126: MOV ECX, dword ptr [0x08604020]
0x809712c: TEST ECX, ECX
0x809712e: JZ 0x0809727c
0x8097134: MOV EAX, [0x086619d4]
0x8097139: TEST EAX, EAX
0x809713b: MOV dword ptr [EBP + -0x8], 0xffffe055
0x8097142: JZ 0x08097173
0x8097144: TEST AL, 0x10
0x8097146: JZ 0x08097156
0x8097148: AND EAX, 0xffffffef
0x809714b: XOR EDX, EDX
0x809714d: TEST EAX, EAX
0x809714f: MOV [0x086619d4], EAX
0x8097154: JZ 0x08097170
0x8097156: MOV EAX, [0x086619d4]
0x809715b: TEST AL, 0x2
0x809715d: JZ 0x080972b4
0x8097163: AND EAX, 0xfffffffd
0x8097166: MOV [0x086619d4], EAX
0x809716b: MOV EDX, 0xffffe059
0x8097170: MOV dword ptr [EBP + -0x8], EDX
0x8097173: CMP dword ptr [EBP + -0x8], 0xffffe055
0x809717a: JNZ 0x080971b0
0x809717c: MOV EAX, [0x086619d8]
0x8097181: MOV EDX, dword ptr [EAX + 0xc]
0x8097184: TEST EDX, EDX
0x8097186: JS 0x080972e6
0x809718c: MOV EAX, dword ptr [EAX + 0x10]
0x809718f: CMP EDX, dword ptr [EAX]
0x8097191: JGE 0x080972e6
0x8097197: SUB ESP, 0xc
0x809719a: PUSH dword ptr [EBP + 0x8]
0x809719d: CALL dword ptr [0x086040a0]
0x80971a3: MOV dword ptr [EBP + -0x8], EAX
0x80971a6: ADD ESP, 0x10
0x80971a9: CMP EAX, 0xffffe055
0x80971ae: JZ 0x0809717c
0x80971b0: MOV EDX, dword ptr [EBP + 0x8]
0x80971b3: MOV EAX, dword ptr [EDX + 0x28]
0x80971b6: TEST EAX, 0x200000
0x80971bb: JZ 0x080971e3
0x80971bd: TEST EAX, EAX
0x80971bf: JS 0x080971e3
0x80971c1: CMP dword ptr [EBP + -0x8], 0xffffe05b
0x80971c8: JZ 0x080971e3
0x80971ca: CMP dword ptr [EBP + -0x8], 0xffffe05a
0x80971d1: JZ 0x080971e3
0x80971d3: SUB ESP, 0xc
0x80971d6: PUSH 0x82af008
0x80971db: CALL 0x08097b24
0x80971e0: ADD ESP, 0x10
0x80971e3: CALL 0x080aae54
0x80971e8: MOV EAX, dword ptr [EBP + -0x8]
0x80971eb: LEAVE
0x80971ec: RET
0x80971f0: SUB ESP, 0xc
0x80971f3: SUB EAX, EDX
0x80971f5: PUSH EAX
0x80971f6: CALL 0x08091ab0
0x80971fb: MOV ECX, dword ptr [EBP + -0x4]
0x80971fe: MOV EDX, dword ptr [ECX]
0x8097200: ADD ESP, 0x10
0x8097203: CMP EDX, -0x1
0x8097206: JNZ 0x080970de
0x809720c: MOV EDX, dword ptr [EBP + 0x8]
0x809720f: MOV EAX, dword ptr [EDX + 0x98]
0x8097215: XOR EDX, EDX
0x8097217: TEST EAX, EAX
0x8097219: JLE 0x0809722d
0x809721b: LEA EAX, [EAX + EAX*0x4]
0x809721e: SHL EAX, 0x2
0x8097221: MOV ECX, dword ptr [EBP + 0x8]
0x8097224: ADD EAX, dword ptr [ECX + 0xa0]
0x809722a: MOV EDX, dword ptr [EAX + -0x10]
0x809722d: SUB ESP, 0x18
0x8097230: PUSH 0x1
0x8097232: PUSH EDX
0x8097233: CALL 0x080aae68
0x8097238: ADD ESP, 0x14
0x809723b: PUSH EAX
0x809723c: CALL 0x0804bac4
0x8097241: ADD ESP, 0x10
0x8097244: TEST EAX, EAX
0x8097246: JNZ 0x080971e8
0x8097248: SUB ESP, 0xc
0x809724b: MOV EAX, dword ptr [EBP + -0x4]
0x809724e: PUSH dword ptr [EAX + 0x10]
0x8097251: CALL dword ptr [EAX + 0xc]
0x8097254: CALL 0x080aae54
0x8097259: JMP 0x08097040
0x809725e: CALL 0x080a9060
0x8097263: TEST EAX, EAX
0x8097265: MOV EDX, 0x1d4c0
0x809726a: CMOVNZ EDX, dword ptr [0x083cc4ec]
0x8097271: MOV dword ptr [0x083cc4ec], EDX
0x8097277: JMP 0x0809702d
0x809727c: MOV ECX, dword ptr [EBP + 0x8]
0x809727f: CMP dword ptr [ECX + 0x44], 0x927c0
0x8097286: JBE 0x08097134
0x809728c: CALL 0x080a9060
0x8097291: TEST EAX, EAX
0x8097293: JNZ 0x0809730b
0x8097295: MOV EAX, dword ptr [EBP + 0x8]
0x8097298: MOV dword ptr [EAX + 0x44], 0x0
0x809729f: JMP 0x08097134
0x80972a4: AND EAX, 0xffffffbf
0x80972a7: MOV dword ptr [EDX + 0x28], EAX
0x80972aa: CALL 0x08093c5c
0x80972af: JMP 0x08097126
0x80972b4: TEST AL, 0x4
0x80972b6: JNZ 0x080972d1
0x80972b8: TEST AL, 0x8
0x80972ba: JZ 0x080972f2
0x80972bc: AND EAX, 0xfffffff7
0x80972bf: MOV EDX, 0xffffe05a
0x80972c4: MOV [0x086619d4], EAX
0x80972c9: MOV dword ptr [EBP + -0x8], EDX
0x80972cc: JMP 0x08097173
0x80972d1: AND EAX, 0xfffffffb
0x80972d4: MOV EDX, 0xffffe058
0x80972d9: MOV [0x086619d4], EAX
0x80972de: MOV dword ptr [EBP + -0x8], EDX
0x80972e1: JMP 0x08097173
0x80972e6: MOV dword ptr [EBP + -0x8], 0xffffe05a
0x80972ed: JMP 0x080971b0
0x80972f2: TEST AL, 0x20
0x80972f4: JZ 0x08097320
0x80972f6: AND EAX, 0xffffffdf
0x80972f9: MOV EDX, 0xffffe055
0x80972fe: MOV [0x086619d4], EAX
0x8097303: MOV dword ptr [EBP + -0x8], EDX
0x8097306: JMP 0x08097173
0x809730b: SUB ESP, 0xc
0x809730e: PUSH 0x82af03c
0x8097313: CALL 0x08097b24
0x8097318: ADD ESP, 0x10
0x809731b: JMP 0x08097134
0x8097320: TEST AL, 0x40
0x8097322: JZ 0x08097339
0x8097324: AND EAX, 0xffffffbf
0x8097327: MOV EDX, 0xffffe05b
0x809732c: MOV [0x086619d4], EAX
0x8097331: MOV dword ptr [EBP + -0x8], EDX
0x8097334: JMP 0x08097173
0x8097339: TEST AL, 0x80
0x809733b: MOV EDX, 0xffffe05a
0x8097340: JZ 0x08097170
0x8097346: AND AL, 0x7f
0x8097348: MOV DL, 0x54
0x809734a: MOV [0x086619d4], EAX
0x809734f: MOV dword ptr [EBP + -0x8], EDX
0x8097352: JMP 0x08097173
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_doActions / procedure_pseudo_code

Evidence: `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f`

Address: `0x8097008`; source: `docs/vm-catch-evidence/reused.json.gz`

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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_error / procedure_pseudo_code

Evidence: `ev_1b494a92dfe3a6c5943ad427ec0ed25c0e15c7afc6932862b5221a85cb9ef7c7`

Address: `0x8097b24`; source: `docs/vm-catch-evidence/reused.json.gz`

```c

/* WARNING: Unknown calling convention */

void vm_error(char *str,...)

{
  char cVar1;
  Vm_Object *pVVar2;
  Vm_Thread_Info_T *pVVar3;
  uint uVar4;
  char *pcVar5;
  char buf [256];
  
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Object_T * pInObject@[DW_OP_reg0(EAX)]
                       Unresolved local var: va_list args@[???]
                       Unresolved local var: int len@[DW_OP_reg1(ECX)] */
  pVVar3 = vm_getCurrentThreadInfo();
  builtin_strncpy(vm_vsprintfBuf,"Error: ",8);
  uVar4 = 0xffffffff;
  pcVar5 = vm_vsprintfBuf;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar1 != '\0');
  vm_vsnprintf((char *)((int)&_vmAddInits[0x3f].initFlag + ~uVar4 + 3),0x4000 - (~uVar4 - 1),str,
               &stack0x00000008);
  if (pVVar3->threadType != 1) {
    if (pVVar3->pPCB != (Vm_PCB *)0x0) {
      vm_setPCBLastError(pVVar3->pPCB,vm_vsprintfBuf);
    }
    vm_errorQuit(-3);
  }
  vm_errorOut(vm_vsprintfBuf);
  pVVar2 = vm_currentContext->thisObject;
  if (pVVar2 != (Vm_Object *)0x0) {
    if ((pVVar2->flags & 1) == 0) {
      snprintf(buf,0x100,"Object: %s\n",pVVar2->pProgram->strName->ptr);
    }
    else {
      snprintf(buf,0x100,"Object: %s#%u\n",pVVar2->pProgram->strName->ptr,pVVar2->oid);
    }
    buf[0xff] = '\0';
    vm_errorOut(buf);
  }
  vm_errorQuit(-3);
  return;
}


```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_errorQuit / procedure_assembly

Evidence: `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2`

Address: `0x80ab104`; source: `docs/vm-catch-evidence/reused.json.gz`

```text
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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x80ab144 to 0x804b574 <EXTERNAL>::longjmp (external=false, thunk=true, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_errorQuit / procedure_pseudo_code

Evidence: `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1`

Address: `0x80ab104`; source: `docs/vm-catch-evidence/reused.json.gz`

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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- Ghidra Listing reports a terminal call at 0x80ab144 to 0x804b574 <EXTERNAL>::longjmp (external=false, thunk=true, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_f_abs / analyze_function

Evidence: `ev_c4611a370e790c377051346043a5c698f4e975dc9cadbcaf69c7b8520c207983`

Address: `0x80d453c`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_abs(void)

{
  uchar uVar1;
  int iVar2;
  Vm_Value_T *pVVar3;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)] */
  if (vm_currentContext->argNum < 2) {
    iVar2 = vm_currentContext->argNum;
  }
  else {
    vm_error("Too many arguments to abs.\n");
    iVar2 = vm_currentContext->argNum;
  }
  if (iVar2 < 1) {
    vm_error("Miss argument 1.\n");
    pVVar3 = vm_getArgument(0,0xff);
    uVar1 = pVVar3->type;
  }
  else {
    pVVar3 = vm_getArgument(0,0xff);
    uVar1 = pVVar3->type;
  }
  if (uVar1 == '\x02') {
    if ((pVVar3->v).number < 0) {
      (pVVar3->v).number = -(pVVar3->v).number;
      return;
    }
  }
  else {
    if (uVar1 == '\x03') {
      if ((pVVar3->v).real < 0.0) {
        (pVVar3->v).real = -(pVVar3->v).real;
        return;
      }
      return;
    }
    vm_error("Bad type of argument to calculate abs value.\n");
  }
  return;
}


```

```text
0x80d453c: PUSH EBP
0x80d453d: MOV EBP, ESP
0x80d453f: SUB ESP, 0x8
0x80d4542: MOV EAX, [0x086619d8]
0x80d4547: CMP dword ptr [EAX + 0x4], 0x1
0x80d454b: JG 0x080d4588
0x80d454d: MOV EAX, dword ptr [EAX + 0x4]
0x80d4550: TEST EAX, EAX
0x80d4552: JLE 0x080d45a4
0x80d4554: SUB ESP, 0x8
0x80d4557: PUSH 0xff
0x80d455c: PUSH 0x0
0x80d455e: CALL 0x0808a364
0x80d4563: MOVZX EDX, byte ptr [EAX + 0x4]
0x80d4567: ADD ESP, 0x10
0x80d456a: CMP EDX, 0x2
0x80d456d: JZ 0x080d45d0
0x80d456f: CMP EDX, 0x3
0x80d4572: JZ 0x080d45e0
0x80d4574: SUB ESP, 0xc
0x80d4577: PUSH 0x82b7870
0x80d457c: CALL 0x08097b24
0x80d4581: ADD ESP, 0x10
0x80d4584: LEAVE
0x80d4585: RET
0x80d4588: SUB ESP, 0xc
0x80d458b: PUSH 0x82b6e9e
0x80d4590: CALL 0x08097b24
0x80d4595: MOV EAX, [0x086619d8]
0x80d459a: MOV EAX, dword ptr [EAX + 0x4]
0x80d459d: ADD ESP, 0x10
0x80d45a0: TEST EAX, EAX
0x80d45a2: JG 0x080d4554
0x80d45a4: SUB ESP, 0xc
0x80d45a7: PUSH 0x82b6eba
0x80d45ac: CALL 0x08097b24
0x80d45b1: ADD ESP, 0x10
0x80d45b4: SUB ESP, 0x8
0x80d45b7: PUSH 0xff
0x80d45bc: PUSH 0x0
0x80d45be: CALL 0x0808a364
0x80d45c3: MOVZX EDX, byte ptr [EAX + 0x4]
0x80d45c7: ADD ESP, 0x10
0x80d45ca: CMP EDX, 0x2
0x80d45cd: JNZ 0x080d456f
0x80d45cf: NOP
0x80d45d0: MOV EDX, dword ptr [EAX + 0x8]
0x80d45d3: TEST EDX, EDX
0x80d45d5: JNS 0x080d4584
0x80d45d7: NEG EDX
0x80d45d9: MOV dword ptr [EAX + 0x8], EDX
0x80d45dc: LEAVE
0x80d45dd: RET
0x80d45e0: FLD float ptr [EAX + 0x8]
0x80d45e3: FLDZ
0x80d45e5: FUCOMIP ST0, ST1
0x80d45e7: JBE 0x080d45f0
0x80d45e9: FCHS
0x80d45eb: FSTP float ptr [EAX + 0x8]
0x80d45ee: LEAVE
0x80d45ef: RET
0x80d45f0: FSTP ST0
0x80d45f2: LEAVE
0x80d45f3: RET
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_f_time / analyze_function

Evidence: `ev_2d1bf533fc49d724997930c78837496b2b5ad806bd6bcc2f6b7c0b0977d5a0ba`

Address: `0x80dd030`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_time(void)

{
  char cVar1;
  char *__src;
  int iVar2;
  char *pcVar3;
  tm *ptVar4;
  time_t t;
  char buf [32];
  tm when;
  
  if (vm_currentContext->argNum < 1) {
    _vm_getShiftTime(&t);
  }
  else {
                    /* Unresolved local var: char * str@[DW_OP_reg3(EBX)] */
    if (1 < vm_currentContext->argNum) {
      vm_error("Too many arguments to time.\n");
    }
    ptVar4 = (tm *)&when;
    for (iVar2 = 0xb; iVar2 != 0; iVar2 = iVar2 + -1) {
      ptVar4->tm_sec = 0;
      ptVar4 = (tm *)&ptVar4->tm_min;
    }
    __src = vm_getStringArgument(0);
    iVar2 = -1;
    pcVar3 = __src;
    do {
      if (iVar2 == 0) break;
      iVar2 = iVar2 + -1;
      cVar1 = *pcVar3;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    if (iVar2 == -0x10) {
      pcVar3 = buf;
      strcpy(pcVar3,__src);
      when.tm_sec = __strtol_internal(buf + 0xc,0,10,0);
      buf[0xc] = '\0';
      when.tm_min = __strtol_internal(buf + 10,0,10,0);
      buf[10] = '\0';
      when.tm_hour = __strtol_internal(buf + 8,0,10,0);
      buf[8] = '\0';
      when.tm_mday = __strtol_internal(buf + 6,0,10,0);
      buf[6] = '\0';
      iVar2 = __strtol_internal(buf + 4,0,10,0);
      buf[4] = '\0';
    }
    else {
      iVar2 = -1;
      pcVar3 = __src;
      do {
        if (iVar2 == 0) break;
        iVar2 = iVar2 + -1;
        cVar1 = *pcVar3;
        pcVar3 = pcVar3 + 1;
      } while (cVar1 != '\0');
      if (iVar2 != -0x15) {
        t = __strtol_internal(__src,0,10,0);
        vm_popNStack(vm_currentContext->argNum);
        vm_pushNumber(t);
        return;
      }
      when.tm_sec = __strtol_internal(__src + 0x11,0,10,0);
      when.tm_min = __strtol_internal(__src + 0xe,0,10,0);
      when.tm_hour = __strtol_internal(__src + 0xb,0,10,0);
      when.tm_mday = __strtol_internal(__src + 8,0,10,0);
      iVar2 = __strtol_internal(__src + 5,0,10,0);
      pcVar3 = __src;
    }
    when.tm_mon = iVar2 + -1;
    iVar2 = __strtol_internal(pcVar3,0,10,0);
    when.tm_year = iVar2 + -0x76c;
    t = mktime((tm *)&when);
  }
  vm_popNStack(vm_currentContext->argNum);
  vm_pushNumber(t);
  return;
}


```

```text
0x80dd030: PUSH EBP
0x80dd031: MOV EBP, ESP
0x80dd033: PUSH EDI
0x80dd034: PUSH ESI
0x80dd035: PUSH EBX
0x80dd036: SUB ESP, 0x6c
0x80dd039: MOV EAX, [0x086619d8]
0x80dd03e: MOV EAX, dword ptr [EAX + 0x4]
0x80dd041: TEST EAX, EAX
0x80dd043: JLE 0x080dd21c
0x80dd049: DEC EAX
0x80dd04a: JG 0x080dd0c0
0x80dd04c: LEA ESI, [EBP + -0x48]
0x80dd04f: SUB ESP, 0xc
0x80dd052: CLD
0x80dd053: XOR EAX, EAX
0x80dd055: MOV ECX, 0xb
0x80dd05a: MOV EDI, ESI
0x80dd05c: STOSD.REP ES:EDI
0x80dd05e: PUSH 0x0
0x80dd060: CALL 0x0808a8ec
0x80dd065: MOV EBX, EAX
0x80dd067: MOV EDX, 0xffffffff
0x80dd06c: XOR EAX, EAX
0x80dd06e: CLD
0x80dd06f: MOV ECX, EDX
0x80dd071: MOV EDI, EBX
0x80dd073: SCASB.REPNE ES:EDI
0x80dd075: ADD ESP, 0x10
0x80dd078: CMP ECX, -0x10
0x80dd07b: JZ 0x080dd188
0x80dd081: CLD
0x80dd082: MOV ECX, EDX
0x80dd084: MOV EDI, EBX
0x80dd086: SCASB.REPNE ES:EDI
0x80dd088: CMP ECX, -0x15
0x80dd08b: JZ 0x080dd0d8
0x80dd08d: PUSH 0x0
0x80dd08f: PUSH 0xa
0x80dd091: PUSH 0x0
0x80dd093: PUSH EBX
0x80dd094: CALL 0x0804b5a4
0x80dd099: ADD ESP, 0x10
0x80dd09c: MOV dword ptr [EBP + -0x6c], EAX
0x80dd09f: SUB ESP, 0xc
0x80dd0a2: MOV EAX, [0x086619d8]
0x80dd0a7: PUSH dword ptr [EAX + 0x4]
0x80dd0aa: CALL 0x08091ab0
0x80dd0af: POP EAX
0x80dd0b0: PUSH dword ptr [EBP + -0x6c]
0x80dd0b3: CALL 0x08090b3c
0x80dd0b8: LEA ESP, [EBP + -0xc]
0x80dd0bb: POP EBX
0x80dd0bc: POP ESI
0x80dd0bd: POP EDI
0x80dd0be: LEAVE
0x80dd0bf: RET
0x80dd0c0: SUB ESP, 0xc
0x80dd0c3: PUSH 0x82b7661
0x80dd0c8: CALL 0x08097b24
0x80dd0cd: ADD ESP, 0x10
0x80dd0d0: JMP 0x080dd04c
0x80dd0d8: PUSH 0x0
0x80dd0da: PUSH 0xa
0x80dd0dc: PUSH 0x0
0x80dd0de: LEA EAX, [EBX + 0x11]
0x80dd0e1: PUSH EAX
0x80dd0e2: CALL 0x0804b5a4
0x80dd0e7: ADD ESP, 0x10
0x80dd0ea: MOV dword ptr [EBP + -0x48], EAX
0x80dd0ed: PUSH 0x0
0x80dd0ef: PUSH 0xa
0x80dd0f1: PUSH 0x0
0x80dd0f3: LEA EAX, [EBX + 0xe]
0x80dd0f6: PUSH EAX
0x80dd0f7: CALL 0x0804b5a4
0x80dd0fc: ADD ESP, 0x10
0x80dd0ff: MOV dword ptr [EBP + -0x44], EAX
0x80dd102: PUSH 0x0
0x80dd104: PUSH 0xa
0x80dd106: PUSH 0x0
0x80dd108: LEA EAX, [EBX + 0xb]
0x80dd10b: PUSH EAX
0x80dd10c: CALL 0x0804b5a4
0x80dd111: ADD ESP, 0x10
0x80dd114: MOV dword ptr [EBP + -0x40], EAX
0x80dd117: PUSH 0x0
0x80dd119: PUSH 0xa
0x80dd11b: PUSH 0x0
0x80dd11d: LEA EAX, [EBX + 0x8]
0x80dd120: PUSH EAX
0x80dd121: CALL 0x0804b5a4
0x80dd126: ADD ESP, 0x10
0x80dd129: MOV dword ptr [EBP + -0x3c], EAX
0x80dd12c: PUSH 0x0
0x80dd12e: PUSH 0xa
0x80dd130: PUSH 0x0
0x80dd132: LEA EAX, [EBX + 0x5]
0x80dd135: PUSH EAX
0x80dd136: CALL 0x0804b5a4
0x80dd13b: DEC EAX
0x80dd13c: ADD ESP, 0x10
0x80dd13f: MOV dword ptr [EBP + -0x38], EAX
0x80dd142: PUSH 0x0
0x80dd144: PUSH 0xa
0x80dd146: PUSH 0x0
0x80dd148: PUSH EBX
0x80dd149: CALL 0x0804b5a4
0x80dd14e: SUB EAX, 0x76c
0x80dd153: MOV dword ptr [EBP + -0x34], EAX
0x80dd156: MOV dword ptr [ESP], ESI
0x80dd159: CALL 0x0804b4e4
0x80dd15e: MOV dword ptr [EBP + -0x6c], EAX
0x80dd161: ADD ESP, 0x10
0x80dd164: SUB ESP, 0xc
0x80dd167: MOV EAX, [0x086619d8]
0x80dd16c: PUSH dword ptr [EAX + 0x4]
0x80dd16f: CALL 0x08091ab0
0x80dd174: POP EAX
0x80dd175: PUSH dword ptr [EBP + -0x6c]
0x80dd178: CALL 0x08090b3c
0x80dd17d: LEA ESP, [EBP + -0xc]
0x80dd180: POP EBX
0x80dd181: POP ESI
0x80dd182: POP EDI
0x80dd183: LEAVE
0x80dd184: RET
0x80dd188: SUB ESP, 0x8
0x80dd18b: PUSH EBX
0x80dd18c: LEA EBX, [EBP + -0x68]
0x80dd18f: PUSH EBX
0x80dd190: CALL 0x0804bc54
0x80dd195: ADD ESP, 0x10
0x80dd198: PUSH 0x0
0x80dd19a: PUSH 0xa
0x80dd19c: PUSH 0x0
0x80dd19e: LEA EAX, [EBP + -0x5c]
0x80dd1a1: PUSH EAX
0x80dd1a2: CALL 0x0804b5a4
0x80dd1a7: ADD ESP, 0x10
0x80dd1aa: MOV dword ptr [EBP + -0x48], EAX
0x80dd1ad: MOV byte ptr [EBP + -0x5c], 0x0
0x80dd1b1: PUSH 0x0
0x80dd1b3: PUSH 0xa
0x80dd1b5: PUSH 0x0
0x80dd1b7: LEA EAX, [EBP + -0x5e]
0x80dd1ba: PUSH EAX
0x80dd1bb: CALL 0x0804b5a4
0x80dd1c0: ADD ESP, 0x10
0x80dd1c3: MOV dword ptr [EBP + -0x44], EAX
0x80dd1c6: MOV byte ptr [EBP + -0x5e], 0x0
0x80dd1ca: PUSH 0x0
0x80dd1cc: PUSH 0xa
0x80dd1ce: PUSH 0x0
0x80dd1d0: LEA EAX, [EBP + -0x60]
0x80dd1d3: PUSH EAX
0x80dd1d4: CALL 0x0804b5a4
0x80dd1d9: ADD ESP, 0x10
0x80dd1dc: MOV dword ptr [EBP + -0x40], EAX
0x80dd1df: MOV byte ptr [EBP + -0x60], 0x0
0x80dd1e3: PUSH 0x0
0x80dd1e5: PUSH 0xa
0x80dd1e7: PUSH 0x0
0x80dd1e9: LEA EAX, [EBP + -0x62]
0x80dd1ec: PUSH EAX
0x80dd1ed: CALL 0x0804b5a4
0x80dd1f2: ADD ESP, 0x10
0x80dd1f5: MOV dword ptr [EBP + -0x3c], EAX
0x80dd1f8: MOV byte ptr [EBP + -0x62], 0x0
0x80dd1fc: PUSH 0x0
0x80dd1fe: PUSH 0xa
0x80dd200: PUSH 0x0
0x80dd202: LEA EAX, [EBP + -0x64]
0x80dd205: PUSH EAX
0x80dd206: CALL 0x0804b5a4
0x80dd20b: DEC EAX
0x80dd20c: ADD ESP, 0x10
0x80dd20f: MOV dword ptr [EBP + -0x38], EAX
0x80dd212: MOV byte ptr [EBP + -0x64], 0x0
0x80dd216: JMP 0x080dd142
0x80dd21c: SUB ESP, 0xc
0x80dd21f: LEA EAX, [EBP + -0x6c]
0x80dd222: PUSH EAX
0x80dd223: CALL 0x080a3740
0x80dd228: ADD ESP, 0x10
0x80dd22b: JMP 0x080dd164
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_getArgument / analyze_function

Evidence: `ev_80271c109d8b170b4dac40d9836f89a9722c8d04023f8e58b881ce2adc69c17e`

Address: `0x808a364`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

Vm_Value_T * vm_getArgument(int n,int type)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  char *pcVar6;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int sn@[DW_OP_reg3(EBX)] */
  iVar4 = vm_stackInThisContext();
  iVar4 = iVar4 + n;
  if (((iVar4 < vm_sp) || (vm_stackSize <= iVar4)) || (vm_currentContext->argNum <= n)) {
    vm_error("Miss argument %d.\n",n + 1);
    pVVar2 = vm_stack + iVar4;
    bVar1 = pVVar2->type;
  }
  else {
    pVVar2 = vm_stack + iVar4;
    bVar1 = pVVar2->type;
  }
  uVar3 = (uint)bVar1;
  if ((uVar3 == type) || (type == 0xff)) {
    return pVVar2;
  }
  pcVar6 = "array";
  if (uVar3 != 6) {
    if (uVar3 < 7) {
      pcVar6 = "integer";
      if (uVar3 != 2) {
        if (uVar3 < 3) {
          pcVar6 = "undefined";
          if (uVar3 != 0) {
            if (uVar3 != 1) goto LAB_0808a402;
            pcVar6 = "void";
          }
        }
        else {
          pcVar6 = "string";
          if (uVar3 != 4) {
            if (uVar3 < 5) {
              pcVar6 = "float";
            }
            else {
              pcVar6 = "buffer";
            }
          }
        }
      }
    }
    else {
      pcVar6 = "function";
      if (uVar3 != 9) {
        if (uVar3 < 10) {
          pcVar6 = "mapping";
          if (uVar3 != 7) {
            if (uVar3 != 8) goto LAB_0808a402;
            pcVar6 = "pointer";
          }
        }
        else {
          pcVar6 = "loop_ctrl";
          if (uVar3 != 0x14) {
            if (uVar3 < 0x15) {
              if (uVar3 != 10) {
LAB_0808a402:
                vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value.\n",
                           "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x6bf);
                _vm_endProcess(0);
                do {
                    /* WARNING: Do nothing block with infinite loop */
                } while( true );
              }
              pcVar6 = "object";
            }
            else {
              if (uVar3 != 0xff) goto LAB_0808a402;
              pcVar6 = "mixed";
            }
          }
        }
      }
    }
  }
  pcVar5 = "array";
  if (type != 6) {
    if (type < 7) {
      pcVar5 = "integer";
      if (type != 2) {
        if (type < 3) {
          pcVar5 = "undefined";
          if (type != 0) {
            if (type != 1) goto LAB_0808a50f;
            pcVar5 = "void";
          }
        }
        else {
          pcVar5 = "string";
          if (type != 4) {
            if (type < 5) {
              pcVar5 = "float";
            }
            else {
              pcVar5 = "buffer";
            }
          }
        }
      }
    }
    else {
      pcVar5 = "function";
      if (type != 9) {
        if (type < 10) {
          pcVar5 = "mapping";
          if (type != 7) {
            if (type != 8) goto LAB_0808a50f;
            pcVar5 = "pointer";
          }
        }
        else {
          pcVar5 = "loop_ctrl";
          if (type != 0x14) {
            if (type < 0x15) {
              if (type != 10) {
LAB_0808a50f:
                vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value.\n",
                           "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x6bf);
                _vm_endProcess(0);
                do {
                    /* WARNING: Do nothing block with infinite loop */
                } while( true );
              }
              pcVar5 = "object";
            }
            else {
              if (type != 0xff) goto LAB_0808a50f;
              pcVar5 = "mixed";
            }
          }
        }
      }
    }
  }
  vm_error("Bad type of argument %d, expected %s, got %s.\n",n + 1,pcVar5,pcVar6);
  return pVVar2;
}


```

```text
0x808a364: PUSH EBP
0x808a365: MOV EBP, ESP
0x808a367: PUSH EDI
0x808a368: PUSH ESI
0x808a369: PUSH EBX
0x808a36a: SUB ESP, 0xc
0x808a36d: MOV EDI, dword ptr [EBP + 0x8]
0x808a370: MOV ESI, dword ptr [EBP + 0xc]
0x808a373: CALL 0x080933e4
0x808a378: LEA EBX, [EAX + EDI*0x1]
0x808a37b: CMP EBX, dword ptr [0x086619cc]
0x808a381: JL 0x0808a38f
0x808a383: CMP EBX, dword ptr [0x08661960]
0x808a389: JL 0x0808a434
0x808a38f: SUB ESP, 0x8
0x808a392: LEA EAX, [EDI + 0x1]
0x808a395: PUSH EAX
0x808a396: PUSH 0x82acd0b
0x808a39b: CALL 0x08097b24
0x808a3a0: MOV EAX, [0x08661964]
0x808a3a5: LEA EDX, [EBX + EBX*0x2]
0x808a3a8: LEA EBX, [EAX + EDX*0x8]
0x808a3ab: MOVZX EAX, byte ptr [EBX + 0x4]
0x808a3af: ADD ESP, 0x10
0x808a3b2: CMP EAX, ESI
0x808a3b4: JZ 0x0808a459
0x808a3ba: CMP ESI, 0xff
0x808a3c0: JZ 0x0808a459
0x808a3c6: CMP EAX, 0x6
0x808a3c9: MOV EDX, 0x82b36b1
0x808a3ce: JZ 0x0808a47d
0x808a3d4: JG 0x0808a4c7
0x808a3da: CMP EAX, 0x2
0x808a3dd: MOV EDX, 0x82acce5
0x808a3e2: JZ 0x0808a47d
0x808a3e8: JG 0x0808a57c
0x808a3ee: TEST EAX, EAX
0x808a3f0: MOV EDX, 0x82b04c6
0x808a3f5: JZ 0x0808a47d
0x808a3fb: DEC EAX
0x808a3fc: JZ 0x0808a5c9
0x808a402: SUB ESP, 0x8
0x808a405: PUSH 0x6bf
0x808a40a: PUSH 0x82acaf8
0x808a40f: PUSH 0x82accd1
0x808a414: PUSH 0x82a825c
0x808a419: PUSH 0x4
0x808a41b: PUSH 0x2
0x808a41d: CALL 0x080b61c4
0x808a422: ADD ESP, 0x14
0x808a425: PUSH 0x0
0x808a427: CALL 0x080a3980
0x808a42c: ADD ESP, 0x10
0x808a42f: JMP 0x0808a42f
0x808a434: MOV EAX, [0x086619d8]
0x808a439: CMP dword ptr [EAX + 0x4], EDI
0x808a43c: JLE 0x0808a38f
0x808a442: MOV EAX, [0x08661964]
0x808a447: LEA EDX, [EBX + EBX*0x2]
0x808a44a: LEA EBX, [EAX + EDX*0x8]
0x808a44d: MOVZX EAX, byte ptr [EBX + 0x4]
0x808a451: CMP EAX, ESI
0x808a453: JNZ 0x0808a3ba
0x808a459: LEA ESP, [EBP + -0xc]
0x808a45c: MOV EAX, EBX
0x808a45e: POP EBX
0x808a45f: POP ESI
0x808a460: POP EDI
0x808a461: LEAVE
0x808a462: RET
0x808a463: CMP EAX, 0x14
0x808a466: MOV EDX, 0x82acced
0x808a46b: JZ 0x0808a47d
0x808a46d: JG 0x0808a596
0x808a473: CMP EAX, 0xa
0x808a476: JNZ 0x0808a402
0x808a478: MOV EDX, 0x82b236c
0x808a47d: CMP ESI, 0x6
0x808a480: MOV EAX, 0x82b36b1
0x808a485: JZ 0x0808a4aa
0x808a487: JG 0x0808a4f0
0x808a489: CMP ESI, 0x2
0x808a48c: MOV EAX, 0x82acce5
0x808a491: JZ 0x0808a4aa
0x808a493: JG 0x0808a540
0x808a499: TEST ESI, ESI
0x808a49b: MOV EAX, 0x82b04c6
0x808a4a0: JZ 0x0808a4aa
0x808a4a2: DEC ESI
0x808a4a3: JNZ 0x0808a50f
0x808a4a5: MOV EAX, 0x82b69fc
0x808a4aa: PUSH EDX
0x808a4ab: PUSH EAX
0x808a4ac: LEA EAX, [EDI + 0x1]
0x808a4af: PUSH EAX
0x808a4b0: PUSH 0x82acb20
0x808a4b5: CALL 0x08097b24
0x808a4ba: ADD ESP, 0x10
0x808a4bd: LEA ESP, [EBP + -0xc]
0x808a4c0: MOV EAX, EBX
0x808a4c2: POP EBX
0x808a4c3: POP ESI
0x808a4c4: POP EDI
0x808a4c5: LEAVE
0x808a4c6: RET
0x808a4c7: CMP EAX, 0x9
0x808a4ca: MOV EDX, 0x82b684c
0x808a4cf: JZ 0x0808a47d
0x808a4d1: JG 0x0808a463
0x808a4d3: CMP EAX, 0x7
0x808a4d6: MOV EDX, 0x82b36a3
0x808a4db: JZ 0x0808a47d
0x808a4dd: CMP EAX, 0x8
0x808a4e0: JNZ 0x0808a402
0x808a4e6: MOV EDX, 0x82e1907
0x808a4eb: JMP 0x0808a47d
0x808a4f0: CMP ESI, 0x9
0x808a4f3: MOV EAX, 0x82b684c
0x808a4f8: JZ 0x0808a4aa
0x808a4fa: JG 0x0808a55c
0x808a4fc: CMP ESI, 0x7
0x808a4ff: MOV EAX, 0x82b36a3
0x808a504: JZ 0x0808a4aa
0x808a506: CMP ESI, 0x8
0x808a509: JZ 0x0808a5b5
0x808a50f: SUB ESP, 0x8
0x808a512: PUSH 0x6bf
0x808a517: PUSH 0x82acaf8
0x808a51c: PUSH 0x82accd1
0x808a521: PUSH 0x82a825c
0x808a526: PUSH 0x4
0x808a528: PUSH 0x2
0x808a52a: CALL 0x080b61c4
0x808a52f: ADD ESP, 0x14
0x808a532: PUSH 0x0
0x808a534: CALL 0x080a3980
0x808a539: ADD ESP, 0x10
0x808a53c: JMP 0x0808a53c
0x808a540: CMP ESI, 0x4
0x808a543: MOV EAX, 0x82aae98
0x808a548: JZ 0x0808a4aa
0x808a54e: JG 0x0808a5bf
0x808a550: MOV EAX, 0x82aaad0
0x808a555: JMP 0x0808a4aa
0x808a55c: CMP ESI, 0x14
0x808a55f: MOV EAX, 0x82acced
0x808a564: JZ 0x0808a4aa
0x808a56a: JG 0x0808a5d3
0x808a56c: CMP ESI, 0xa
0x808a56f: JNZ 0x0808a50f
0x808a571: MOV EAX, 0x82b236c
0x808a576: JMP 0x0808a4aa
0x808a57c: CMP EAX, 0x4
0x808a57f: MOV EDX, 0x82aae98
0x808a584: JZ 0x0808a47d
0x808a58a: JG 0x0808a5ab
0x808a58c: MOV EDX, 0x82aaad0
0x808a591: JMP 0x0808a47d
0x808a596: CMP EAX, 0xff
0x808a59b: JNZ 0x0808a402
0x808a5a1: MOV EDX, 0x82aaaea
0x808a5a6: JMP 0x0808a47d
0x808a5ab: MOV EDX, 0x82b13db
0x808a5b0: JMP 0x0808a47d
0x808a5b5: MOV EAX, 0x82e1907
0x808a5ba: JMP 0x0808a4aa
0x808a5bf: MOV EAX, 0x82b13db
0x808a5c4: JMP 0x0808a4aa
0x808a5c9: MOV EDX, 0x82b69fc
0x808a5ce: JMP 0x0808a47d
0x808a5d3: CMP ESI, 0xff
0x808a5d9: JNZ 0x0808a50f
0x808a5df: MOV EAX, 0x82aaaea
0x808a5e4: JMP 0x0808a4aa
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_popNStack / analyze_function

Evidence: `ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4`

Address: `0x8091ab0`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

void vm_popNStack(int n)

{
                    /* Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  if (0 < n) {
    do {
      vm_cleanupValue(vm_stack + vm_sp);
      vm_sp = vm_sp + 1;
      n = n + -1;
    } while (n != 0);
  }
  return;
}


```

```text
0x8091ab0: PUSH EBP
0x8091ab1: MOV EBP, ESP
0x8091ab3: PUSH EBX
0x8091ab4: PUSH EAX
0x8091ab5: MOV EDX, dword ptr [EBP + 0x8]
0x8091ab8: TEST EDX, EDX
0x8091aba: JLE 0x08091aea
0x8091abc: MOV EAX, [0x086619cc]
0x8091ac1: MOV EBX, EDX
0x8091ac3: NOP
0x8091ac4: MOV EDX, dword ptr [0x08661964]
0x8091aca: LEA EAX, [EAX + EAX*0x2]
0x8091acd: SUB ESP, 0xc
0x8091ad0: LEA EAX, [EDX + EAX*0x8]
0x8091ad3: PUSH EAX
0x8091ad4: CALL 0x0808ad98
0x8091ad9: MOV EAX, [0x086619cc]
0x8091ade: INC EAX
0x8091adf: ADD ESP, 0x10
0x8091ae2: DEC EBX
0x8091ae3: MOV [0x086619cc], EAX
0x8091ae8: JNZ 0x08091ac4
0x8091aea: MOV EBX, dword ptr [EBP + -0x4]
0x8091aed: LEAVE
0x8091aee: RET
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_regularProcessor / analyze_function

Evidence: `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`

Address: `0x80978c4`; source: `docs/evidence/magic_Linux32-rea.json.gz`

```c

/* WARNING: Unknown calling convention */

int vm_regularProcessor(Vm_PCB_T *pPCB)

{
  Vm_Code_Segment_T **ppVVar1;
  Vm_Node_T *pVVar2;
  byte bVar3;
  short sVar4;
  Vm_Next_Action_Hook_T p_Var5;
  void *pvVar6;
  int iVar7;
  int iVar8;
  int actionState;
  
                    /* Unresolved local var: int counter@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Action_Info_T * pActionInfo@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Node_T * pThisNode@[DW_OP_reg3(EBX)] */
  iVar8 = 0x800;
  actionState = 0;
  vm_switchOutThread = 0;
  do {
    if (vm_ctrlFlags != 0) {
      if ((vm_ctrlFlags & 0x10) == 0) {
LAB_08097902:
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
      }
      else {
        vm_ctrlFlags = vm_ctrlFlags & 0xffffffef;
        actionState = 0;
        if (vm_ctrlFlags != 0) goto LAB_08097902;
      }
      if (actionState != 0) {
        return actionState;
      }
    }
    iVar8 = iVar8 + -1;
    if (iVar8 < 1) {
      iVar7 = _vm_isTimerTimeout(0);
      iVar8 = 0x400;
      if (iVar7 != 0) {
        return actionState;
      }
    }
    p_Var5 = pPCB->nextActionHook;
    if (p_Var5 == (Vm_Next_Action_Hook_T)0x0) {
      ppVVar1 = &vm_currentContext->cs;
      pVVar2 = (*ppVVar1)->nodes + vm_currentContext->ip;
      vm_currentContext->ip = vm_currentContext->ip + 1;
      bVar3 = pVVar2->actionNo;
      if ((int)actionInfoTable[bVar3].attrib < 0) {
                    /* Unresolved local var: int metaOffset@[DW_OP_reg1(ECX)] */
        (*actionInfoTable[bVar3].entry)
                  (*(undefined4 *)
                    ((*ppVVar1)->metaData + ((int)pVVar2->par8 << 0x10 | (int)pVVar2->par16)));
      }
      else if (actionInfoTable[bVar3].parameters == '\0') {
        (*actionInfoTable[bVar3].entry)();
      }
      else {
        if (actionInfoTable[bVar3].parameters == '\x01') {
          sVar4 = pVVar2->par16;
        }
        else {
          sVar4 = pVVar2->par16;
        }
        (*actionInfoTable[bVar3].entry)((int)sVar4);
      }
    }
    else {
                    /* Unresolved local var: Vm_Next_Action_Hook_T _nextActionHook@[???]
                       Unresolved local var: void * _pActionInfoCookie@[DW_OP_reg0(EAX)] */
      pvVar6 = pPCB->pNextActionCookie;
      pPCB->nextActionHook = (Vm_Next_Action_Hook_T)0x0;
      pPCB->pNextActionCookie = (void *)0x0;
      (*p_Var5)(pvVar6);
    }
    if (vm_switchOutThread != 0) {
      return actionState;
    }
  } while( true );
}


```

```text
0x80978c4: PUSH EBP
0x80978c5: MOV EBP, ESP
0x80978c7: PUSH EDI
0x80978c8: PUSH ESI
0x80978c9: PUSH EBX
0x80978ca: SUB ESP, 0xc
0x80978cd: MOV EDI, 0x800
0x80978d2: MOV dword ptr [EBP + -0x10], 0x0
0x80978d9: MOV dword ptr [0x086619d0], 0x0
0x80978e3: NOP
0x80978e4: MOV EDX, dword ptr [0x086619d4]
0x80978ea: TEST EDX, EDX
0x80978ec: JZ 0x08097924
0x80978ee: TEST DL, 0x10
0x80978f1: JZ 0x08097902
0x80978f3: AND EDX, 0xffffffef
0x80978f6: XOR EAX, EAX
0x80978f8: TEST EDX, EDX
0x80978fa: MOV dword ptr [0x086619d4], EDX
0x8097900: JZ 0x08097919
0x8097902: TEST DL, 0x2
0x8097905: JZ 0x080979b4
0x809790b: AND EDX, 0xfffffffd
0x809790e: MOV dword ptr [0x086619d4], EDX
0x8097914: MOV EAX, 0xffffe059
0x8097919: TEST EAX, EAX
0x809791b: MOV dword ptr [EBP + -0x10], EAX
0x809791e: JNZ 0x080979a6
0x8097924: DEC EDI
0x8097925: TEST EDI, EDI
0x8097927: JLE 0x08097a6c
0x809792d: MOV EAX, dword ptr [EBP + 0x8]
0x8097930: MOV EDX, dword ptr [EAX + 0xb0]
0x8097936: TEST EDX, EDX
0x8097938: JNZ 0x08097a0c
0x809793e: MOV ECX, dword ptr [0x086619d8]
0x8097944: MOV ESI, dword ptr [ECX + 0x10]
0x8097947: MOV EAX, dword ptr [ECX + 0xc]
0x809794a: MOV EDX, dword ptr [ESI + 0x4]
0x809794d: LEA EBX, [EDX + EAX*0x4]
0x8097950: INC EAX
0x8097951: MOV dword ptr [ECX + 0xc], EAX
0x8097954: MOVZX EDX, byte ptr [EBX + 0x3]
0x8097958: LEA EAX, [EDX*0x8 + 0x0]
0x809795f: SUB EAX, EDX
0x8097961: LEA EDX, [EAX*0x4 + 0x83cc680]
0x8097968: MOV EAX, dword ptr [EDX + 0x18]
0x809796b: TEST EAX, EAX
0x809796d: JS 0x08097a3c
0x8097973: MOV AL, byte ptr [EDX + 0xc]
0x8097976: TEST AL, AL
0x8097978: JZ 0x08097a34
0x809797e: DEC AL
0x8097980: JZ 0x08097a60
0x8097986: MOVSX EAX, byte ptr [EBX + 0x2]
0x809798a: SUB ESP, 0x8
0x809798d: PUSH EAX
0x809798e: MOVSX EAX, word ptr [EBX]
0x8097991: PUSH EAX
0x8097992: CALL dword ptr [EDX + 0x8]
0x8097995: ADD ESP, 0x10
0x8097998: MOV ECX, dword ptr [0x086619d0]
0x809799e: TEST ECX, ECX
0x80979a0: JZ 0x080978e4
0x80979a6: MOV EAX, dword ptr [EBP + -0x10]
0x80979a9: LEA ESP, [EBP + -0xc]
0x80979ac: POP EBX
0x80979ad: POP ESI
0x80979ae: POP EDI
0x80979af: LEAVE
0x80979b0: RET
0x80979b4: TEST DL, 0x4
0x80979b7: JZ 0x080979cc
0x80979b9: AND EDX, 0xfffffffb
0x80979bc: MOV EAX, 0xffffe058
0x80979c1: MOV dword ptr [0x086619d4], EDX
0x80979c7: JMP 0x08097919
0x80979cc: TEST DL, 0x8
0x80979cf: JZ 0x080979e4
0x80979d1: AND EDX, 0xfffffff7
0x80979d4: MOV EAX, 0xffffe05a
0x80979d9: MOV dword ptr [0x086619d4], EDX
0x80979df: JMP 0x08097919
0x80979e4: TEST DL, 0x20
0x80979e7: JNZ 0x08097a98
0x80979ed: TEST DL, 0x40
0x80979f0: JZ 0x08097aab
0x80979f6: AND EDX, 0xffffffbf
0x80979f9: MOV EAX, 0xffffe05b
0x80979fe: MOV dword ptr [0x086619d4], EDX
0x8097a04: JMP 0x08097919
0x8097a0c: MOV ECX, EAX
0x8097a0e: SUB ESP, 0xc
0x8097a11: MOV EAX, dword ptr [EAX + 0xb4]
0x8097a17: MOV dword ptr [ECX + 0xb0], 0x0
0x8097a21: MOV dword ptr [ECX + 0xb4], 0x0
0x8097a2b: PUSH EAX
0x8097a2c: CALL EDX
0x8097a2e: JMP 0x08097995
0x8097a34: CALL dword ptr [EDX + 0x8]
0x8097a37: JMP 0x08097998
0x8097a3c: MOVSX ECX, byte ptr [EBX + 0x2]
0x8097a40: SHL ECX, 0x10
0x8097a43: MOVSX EAX, word ptr [EBX]
0x8097a46: OR ECX, EAX
0x8097a48: CMP byte ptr [EDX + 0xc], 0x2
0x8097a4c: JZ 0x08097a8c
0x8097a4e: SUB ESP, 0xc
0x8097a51: MOV EAX, dword ptr [ESI + 0xc]
0x8097a54: PUSH dword ptr [EAX + ECX*0x1]
0x8097a57: CALL dword ptr [EDX + 0x8]
0x8097a5a: JMP 0x08097995
0x8097a60: SUB ESP, 0xc
0x8097a63: MOVSX EAX, word ptr [EBX]
0x8097a66: PUSH EAX
0x8097a67: JMP 0x08097992
0x8097a6c: SUB ESP, 0xc
0x8097a6f: PUSH 0x0
0x8097a71: CALL 0x080a3904
0x8097a76: ADD ESP, 0x10
0x8097a79: TEST EAX, EAX
0x8097a7b: MOV EDI, 0x400
0x8097a80: JZ 0x0809792d
0x8097a86: JMP 0x080979a6
0x8097a8c: SUB ESP, 0x8
0x8097a8f: MOV EAX, dword ptr [ESI + 0xc]
0x8097a92: PUSH dword ptr [EAX + ECX*0x1 + 0x4]
0x8097a96: JMP 0x08097a51
0x8097a98: AND EDX, 0xffffffdf
0x8097a9b: MOV EAX, 0xffffe055
0x8097aa0: MOV dword ptr [0x086619d4], EDX
0x8097aa6: JMP 0x08097919
0x8097aab: TEST DL, 0x80
0x8097aae: MOV EAX, 0xffffe05a
0x8097ab3: JZ 0x08097919
0x8097ab9: AND DL, 0x7f
0x8097abc: MOV AL, 0x54
0x8097abe: MOV dword ptr [0x086619d4], EDX
0x8097ac4: JMP 0x08097919
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_returnFromEfun / analyze_function

Evidence: `ev_e86eb89e60b57e211e2efdce9ee32fdd5b9d630f548d18ca4e6b50e43675595c`

Address: `0x8093c5c`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_returnFromEfun(void)

{
  Vm_FunCall_Context_T *pVVar1;
  uchar uVar2;
  byte bVar3;
  int iVar4;
  char *str;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Efun_T * pEfun@[DW_OP_reg0(EAX)]
                       Unresolved local var: int retNum@[DW_OP_reg2(EDX)]
                       Unresolved local var: int retType@[DW_OP_reg3(EBX)]
                       Unresolved local var: int argn@[???] */
  pVVar1 = vm_funContext + vm_funCp;
  iVar4 = pVVar1->bp + pVVar1->argNum;
  uVar2 = ((pVVar1->f).pEfun)->retType;
  if (uVar2 == '\x01') {
    if (iVar4 != vm_sp) {
      vm_error("Implementation of efun was error! Should return void but return value.\n");
      vm_pushVoid();
      goto LAB_08093cdc;
    }
LAB_08093d4c:
    vm_pushVoid();
    goto LAB_08093cdc;
  }
  if (uVar2 == 0xff) {
    if (iVar4 == vm_sp) goto LAB_08093d4c;
    if (iVar4 - vm_sp == 1) goto LAB_08093cdc;
    str = "Implementation of efun was error! Should return only 1 value.\n";
  }
  else {
    if (iVar4 == vm_sp) {
      vm_error("Implementation of efun was error! Should return value.\n");
LAB_08093ca4:
      vm_error("Implementation of efun was error! Return type error.\n");
    }
    else if (iVar4 - vm_sp != 1) goto LAB_08093ca4;
    if (vm_stack[vm_sp].type == uVar2) goto LAB_08093cdc;
    str = "Implementation of efun was error! Return type error.\n";
  }
  vm_error(str);
LAB_08093cdc:
  if (vm_gatheringCallStackInfo == 0) {
    vm_funCp = vm_funCp + -1;
    vm_updateCurrentFunCallContext();
    iVar4 = vm_isEnabledJit();
  }
  else {
    vm_enterPPCopyStackCriticalSection();
    vm_funCp = vm_funCp + -1;
    vm_updateCurrentFunCallContext();
    vm_leavePPCopyStackCriticalSection();
    iVar4 = vm_isEnabledJit();
  }
  if (iVar4 == 0) {
    vm_checkCurrentTicks();
    bVar3 = (byte)pVVar1->flags;
  }
  else {
    bVar3 = (byte)pVVar1->flags;
  }
  if ((bVar3 & 0x80) == 0) {
    return;
  }
  vm_hookCallbackWhenNextAction(pVVar1->returnCallback,pVVar1->pCookie);
  return;
}


```

```text
0x8093c5c: PUSH EBP
0x8093c5d: MOV EBP, ESP
0x8093c5f: PUSH ESI
0x8093c60: PUSH EBX
0x8093c61: MOV EAX, [0x086619c0]
0x8093c66: LEA EAX, [EAX + EAX*0x4]
0x8093c69: MOV EDX, dword ptr [0x086619c8]
0x8093c6f: LEA ESI, [EDX + EAX*0x8]
0x8093c72: MOV EAX, dword ptr [ESI + 0x24]
0x8093c75: MOV EDX, dword ptr [ESI]
0x8093c77: ADD EDX, dword ptr [ESI + 0x4]
0x8093c7a: MOVZX EBX, byte ptr [EAX + 0x18]
0x8093c7e: SUB EDX, dword ptr [0x086619cc]
0x8093c84: CMP EBX, 0x1
0x8093c87: JZ 0x08093d48
0x8093c8d: CMP EBX, 0xff
0x8093c93: JZ 0x08093d54
0x8093c99: TEST EDX, EDX
0x8093c9b: JZ 0x08093d68
0x8093ca1: DEC EDX
0x8093ca2: JZ 0x08093cb4
0x8093ca4: SUB ESP, 0xc
0x8093ca7: PUSH 0x82ae1ec
0x8093cac: CALL 0x08097b24
0x8093cb1: ADD ESP, 0x10
0x8093cb4: MOV EAX, [0x086619cc]
0x8093cb9: LEA EAX, [EAX + EAX*0x2]
0x8093cbc: MOV EDX, dword ptr [0x08661964]
0x8093cc2: MOVZX EAX, byte ptr [EDX + EAX*0x8 + 0x4]
0x8093cc7: CMP EAX, EBX
0x8093cc9: JZ 0x08093cdc
0x8093ccb: SUB ESP, 0xc
0x8093cce: PUSH 0x82ae1ec
0x8093cd3: CALL 0x08097b24
0x8093cd8: ADD ESP, 0x10
0x8093cdb: NOP
0x8093cdc: MOV EAX, [0x0860ae64]
0x8093ce1: TEST EAX, EAX
0x8093ce3: JZ 0x08093d10
0x8093ce5: CALL 0x080bb0d4
0x8093cea: DEC dword ptr [0x086619c0]
0x8093cf0: CALL 0x0809789c
0x8093cf5: CALL 0x080bb0e8
0x8093cfa: CALL 0x08095e68
0x8093cff: TEST EAX, EAX
0x8093d01: JZ 0x08093d24
0x8093d03: TEST byte ptr [ESI + 0x8], 0x80
0x8093d07: JNZ 0x08093d30
0x8093d09: LEA ESP, [EBP + -0x8]
0x8093d0c: POP EBX
0x8093d0d: POP ESI
0x8093d0e: LEAVE
0x8093d0f: RET
0x8093d10: DEC dword ptr [0x086619c0]
0x8093d16: CALL 0x0809789c
0x8093d1b: CALL 0x08095e68
0x8093d20: TEST EAX, EAX
0x8093d22: JNZ 0x08093d03
0x8093d24: CALL 0x08096ec0
0x8093d29: TEST byte ptr [ESI + 0x8], 0x80
0x8093d2d: JZ 0x08093d09
0x8093d2f: NOP
0x8093d30: SUB ESP, 0x8
0x8093d33: PUSH dword ptr [ESI + 0x1c]
0x8093d36: PUSH dword ptr [ESI + 0x18]
0x8093d39: CALL 0x080a9a48
0x8093d3e: ADD ESP, 0x10
0x8093d41: LEA ESP, [EBP + -0x8]
0x8093d44: POP EBX
0x8093d45: POP ESI
0x8093d46: LEAVE
0x8093d47: RET
0x8093d48: TEST EDX, EDX
0x8093d4a: JNZ 0x08093d7d
0x8093d4c: CALL 0x08090fc0
0x8093d51: JMP 0x08093cdc
0x8093d54: TEST EDX, EDX
0x8093d56: JZ 0x08093d4c
0x8093d58: DEC EDX
0x8093d59: JZ 0x08093cdc
0x8093d5b: SUB ESP, 0xc
0x8093d5e: PUSH 0x82ae224
0x8093d63: JMP 0x08093cd3
0x8093d68: SUB ESP, 0xc
0x8093d6b: PUSH 0x82ae264
0x8093d70: CALL 0x08097b24
0x8093d75: ADD ESP, 0x10
0x8093d78: JMP 0x08093ca4
0x8093d7d: SUB ESP, 0xc
0x8093d80: PUSH 0x82ae29c
0x8093d85: CALL 0x08097b24
0x8093d8a: ADD ESP, 0x10
0x8093d8d: CALL 0x08090fc0
0x8093d92: JMP 0x08093cdc
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_stackInThisContext / analyze_function

Evidence: `ev_d908ab85c3b336c3108d0113d535ec9b5582b136bb6b35d6ca5975a896519b6d`

Address: `0x80933e4`; source: `docs/stage2ad-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_stackInThisContext(void)

{
  int iVar1;
  
  iVar1 = 0;
  if (vm_funCp != 0) {
    iVar1 = vm_funContext[vm_funCp].bp;
  }
  return iVar1;
}


```

```text
0x80933e4: MOV EDX, dword ptr [0x086619c0]
0x80933ea: PUSH EBP
0x80933eb: XOR EAX, EAX
0x80933ed: TEST EDX, EDX
0x80933ef: MOV EBP, ESP
0x80933f1: JZ 0x080933fe
0x80933f3: LEA EDX, [EDX + EDX*0x4]
0x80933f6: MOV EAX, [0x086619c8]
0x80933fb: MOV EAX, dword ptr [EAX + EDX*0x8]
0x80933fe: LEAVE
0x80933ff: RET
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_traceError / procedure_assembly

Evidence: `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe`

Address: `0x8093520`; source: `docs/vm-catch-evidence/reused.json.gz`

```text
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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_traceError / procedure_pseudo_code

Evidence: `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4`

Address: `0x8093520`; source: `docs/vm-catch-evidence/reused.json.gz`

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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.
