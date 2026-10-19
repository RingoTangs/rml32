# 阶段 2AA 全局存储与调用证据

REA/Ghidra12.1.4静态结果，不是源代码或运行观察。完整raw_result、CFG和限制见压缩包。导航查询单独留存在navigation.json.gz。

## vm_pushGlobalVptr / procedure_assembly

Evidence: `ev_0afa29523e8e05f218ef05cec068d897509aa00f512de38f9de00d12fb0b2ff1`

```text
0x80910d0: PUSH EBP
0x80910d1: MOV EBP, ESP
0x80910d3: PUSH EBX
0x80910d4: PUSH ECX
0x80910d5: MOV EAX, dword ptr [EBP + 0x8]
0x80910d8: LEA EAX, [EAX + EAX*0x2]
0x80910db: MOV EDX, dword ptr [0x0847aacc]
0x80910e1: MOV EBX, dword ptr [EDX + EAX*0x4]
0x80910e4: MOV EAX, [0x086619cc]
0x80910e9: TEST EAX, EAX
0x80910eb: JZ 0x08091124
0x80910ed: DEC EAX
0x80910ee: MOV [0x086619cc], EAX
0x80910f3: MOV EDX, dword ptr [0x08661964]
0x80910f9: LEA EAX, [EAX + EAX*0x2]
0x80910fc: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x8
0x8091101: MOV EAX, [0x086619cc]
0x8091106: LEA EAX, [EAX + EAX*0x2]
0x8091109: MOV ECX, dword ptr [0x08661964]
0x809110f: MOV DL, byte ptr [EBX + 0x5]
0x8091112: SHL EAX, 0x3
0x8091115: INC dword ptr [EBX]
0x8091117: MOV dword ptr [EAX + ECX*0x1 + 0x8], EBX
0x809111b: MOV byte ptr [EAX + ECX*0x1 + 0x5], DL
0x809111f: MOV EBX, dword ptr [EBP + -0x4]
0x8091122: LEAVE
0x8091123: RET
0x8091124: SUB ESP, 0xc
0x8091127: PUSH 0x82ad98c
0x809112c: CALL 0x08097b24
0x8091131: ADD ESP, 0x10
0x8091134: MOV EAX, [0x086619cc]
0x8091139: JMP 0x080910ed
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_getNewGlobalVarOffset / analyze_function

Evidence: `ev_2bce189c12465150bd034fd31e9af6028ea48c8031b70df5eedb7033f126abd2`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8080782", "start": "0x8080770"}], "span_bytes": 19, "total_bytes": 19}`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_getNewGlobalVarOffset(void)

{
  int iVar1;
  
  iVar1 = vm_globalVar;
  vm_globalVar = vm_globalVar + 1;
  return iVar1;
}


```

```text
0x8080770: MOV EAX, [0x0847aac4]
0x8080775: PUSH EBP
0x8080776: MOV EBP, ESP
0x8080778: LEA EDX, [EAX + 0x1]
0x808077b: MOV dword ptr [0x0847aac4], EDX
0x8080781: LEAVE
0x8080782: RET
```

限制：
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

## vm_safeNewValue / analyze_function

Evidence: `ev_6ad4e4a31f4846ef1c2950ccd0df173f861e78b97f61852ca9fbd4a30a4bf86c`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8089fd5", "start": "0x8089f78"}, {"end": "0x8089fec", "start": "0x8089fd8"}], "span_bytes": 117, "total_bytes": 115}`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

Vm_Value_T * vm_safeNewValue(void)

{
  Vm_Value *pVVar1;
  int iVar2;
  Vm_Value *pVVar3;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg2(EDX)] */
  pVVar1 = pdb_allocateTinyBlock(0x18);
  if (pVVar1 != (Vm_Value *)0x0) {
    pVVar3 = pVVar1;
    for (iVar2 = 6; iVar2 != 0; iVar2 = iVar2 + -1) {
      pVVar3->ref = 0;
      pVVar3 = (Vm_Value *)&pVVar3->type;
    }
    pVVar1->type = '\0';
    pVVar1->attrib = '\0';
    pVVar1->ref = 1;
    vm_totalValues = vm_totalValues + 1;
    if (vm_allValues == (Vm_Value_T *)0x0) {
      vm_allValues = pVVar1;
      pVVar1->pPrev = (Vm_Value *)0x0;
      pVVar1->pNext = (Vm_Value *)0x0;
    }
    else {
      pVVar1->pPrev = (Vm_Value *)0x0;
      pVVar1->pNext = vm_allValues;
      vm_allValues->pPrev = pVVar1;
      vm_allValues = pVVar1;
    }
    pVVar1->pThis = pVVar1;
  }
  return pVVar1;
}


```

```text
0x8089f78: PUSH EBP
0x8089f79: MOV EBP, ESP
0x8089f7b: PUSH EDI
0x8089f7c: SUB ESP, 0x10
0x8089f7f: PUSH 0x18
0x8089f81: CALL 0x080cee94
0x8089f86: ADD ESP, 0x10
0x8089f89: TEST EAX, EAX
0x8089f8b: MOV EDX, EAX
0x8089f8d: JZ 0x08089fcf
0x8089f8f: XOR EAX, EAX
0x8089f91: CLD
0x8089f92: MOV ECX, 0x6
0x8089f97: MOV EDI, EDX
0x8089f99: STOSD.REP ES:EDI
0x8089f9b: MOV byte ptr [EDX + 0x4], 0x0
0x8089f9f: MOV byte ptr [EDX + 0x5], 0x0
0x8089fa3: MOV EAX, [0x0847fd8c]
0x8089fa8: MOV dword ptr [EDX], 0x1
0x8089fae: INC dword ptr [0x0847fd88]
0x8089fb4: TEST EAX, EAX
0x8089fb6: JNZ 0x08089fd8
0x8089fb8: MOV dword ptr [0x0847fd8c], EDX
0x8089fbe: MOV dword ptr [EDX + 0x14], 0x0
0x8089fc5: MOV dword ptr [EDX + 0xc], 0x0
0x8089fcc: MOV dword ptr [EDX + 0x10], EDX
0x8089fcf: MOV EAX, EDX
0x8089fd1: MOV EDI, dword ptr [EBP + -0x4]
0x8089fd4: LEAVE
0x8089fd5: RET
0x8089fd8: MOV dword ptr [EDX + 0x14], 0x0
0x8089fdf: MOV dword ptr [EDX + 0xc], EAX
0x8089fe2: MOV dword ptr [EAX + 0x14], EDX
0x8089fe5: MOV dword ptr [0x0847fd8c], EDX
0x8089feb: JMP 0x08089fcc
```

限制：
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

## vm_endNewFile / analyze_function

Evidence: `ev_826e3b39ae0edc9b4bee9eabaac0dbdb7ee0500573ec5a85ea88369b044ceba6`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8088e24", "start": "0x8088874"}, {"end": "0x8088f2d", "start": "0x8088e28"}], "span_bytes": 1722, "total_bytes": 1719}`

```c

/* WARNING: Unknown calling convention */

int vm_endNewFile(int succ,int isFreeDefine,Vm_Code_Segment_T *pFinalCodeSegment)

{
  short sVar1;
  uint uVar2;
  Vm_Ident_Hash_Elem_T *pVVar3;
  Vm_Ident_Hash_Elem *pVVar4;
  Vm_Defn_T *pVVar5;
  Vm_Incstate_T *ptr;
  Vm_Defn_T *ptr_00;
  Vm_Ident_Hash_Elem_T *ihe;
  Vm_Ifstate_T *ptr_01;
  char **ppcVar6;
  Vm_Defn_T *pVVar7;
  Vm_Fun_Entry_T *pVVar8;
  Vm_Quick_Function *pVVar9;
  Vm_Ident_Hash_Elem **ppVVar10;
  Vm_Global_Var_T *pVVar11;
  int iVar12;
  Vm_Value_T *pVVar13;
  int iVar14;
  short sVar15;
  Vm_Program_T *pVVar16;
  Vm_Prog_Var_T *ptr_02;
  char *pcVar17;
  int iVar18;
  char *pcVar19;
  bool bVar20;
  Vm_Program_T *pVVar21;
  Vm_Program_T *local_120;
  char binFileName [256];
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[???] */
  local_120 = vm_pCurrentProgram;
  if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
    local_120 = vm_pGlobalProgram;
  }
  ptr = incTop;
  ppcVar6 = (char **)0x0;
  if (vm_pszLocalIncList != (char **)0x0) {
    pdb_freeMemory(*vm_pszLocalIncList,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x53d);
    pdb_freeMemory(vm_pszLocalIncList,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x53e);
    vm_pszLocalIncList = (char **)0x0;
    vm_localIncListSize = 0;
    ptr = incTop;
    ppcVar6 = vm_pszLocalIncList;
  }
  while (vm_pszLocalIncList = ppcVar6, ptr != (Vm_Incstate_T *)0x0) {
    incTop = ptr;
                    /* Unresolved local var: Vm_Incstate_T * p@[???] */
    vm_close(vm_yyinDesc);
    currentFile = ptr->file;
    vm_yyinDesc = ptr->yyinDesc;
    vm_yyinCryptType = ptr->yyinCryptType;
    vm_yyinCryptCode = ptr->yyinCryptCode;
    incTop = ptr->pNext;
    vm_yyinStart = 0;
    vm_scratchFree(ptr);
    ptr = incTop;
    ppcVar6 = vm_pszLocalIncList;
  }
  incTop = (Vm_Incstate_T *)0x0;
  ptr_01 = vm_iftop;
  while (ptr_01 != (Vm_Ifstate_T *)0x0) {
                    /* Unresolved local var: Vm_Ifstate_T * p@[DW_OP_reg0(EAX)] */
    vm_iftop = ptr_01->pNext;
    vm_scratchFree(ptr_01);
    ptr_01 = vm_iftop;
  }
  vm_iftop = ptr_01;
  if (succ == 0) goto LAB_08088a76;
  sVar1 = local_120->maxFunctionNo;
  if (vm_functionBC < sVar1) {
                    /* Unresolved local var: Vm_Fun_Entry_T * pFunctions@[DW_OP_reg0(EAX)] */
    if (local_120->pFunctions == (Vm_Fun_Entry_T *)0x0) {
      pVVar8 = pdb_allocateMemory((int)sVar1 << 3,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",
                                  0x19f3);
    }
    else {
      pVVar8 = pdb_reallocateMemory
                         (local_120->pFunctions,(int)sVar1 << 3,"VM",
                          "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19f7);
    }
    if (pVVar8 != (Vm_Fun_Entry_T *)0x0) {
      iVar18 = (int)vm_functionBC;
      local_120->pFunctions = pVVar8;
      pVVar8 = pVVar8 + iVar18;
      for (iVar14 = (local_120->maxFunctionNo - iVar18 & 0x1fffffffU) << 1; iVar14 != 0;
          iVar14 = iVar14 + -1) {
        pVVar8->pFun = (Vm_Fun *)0x0;
        pVVar8 = (Vm_Fun_Entry_T *)&pVVar8->consititutionNo;
      }
      goto LAB_080889f1;
    }
  }
  else {
LAB_080889f1:
    if ((local_120 != vm_pGlobalProgram) || (vm_globalVar <= vm_variableBC)) {
LAB_08088a03:
      if (((vm_isBinaryFile == 0) && ((int)currentAttrib < 0)) &&
         (bVar20 = currentInputFile == (char *)0x0, !bVar20)) {
        iVar14 = 3;
        pcVar17 = "/v/";
        pcVar19 = currentInputFile;
        do {
          if (iVar14 == 0) break;
          iVar14 = iVar14 + -1;
          bVar20 = *pcVar17 == *pcVar19;
          pcVar17 = pcVar17 + 1;
          pcVar19 = pcVar19 + 1;
        } while (bVar20);
        if (!bVar20) {
          iVar14 = vm_generateBinFileName(currentInputFile,binFileName,0x100);
          if (iVar14 != 0) {
            vm_saveBinary(binFileName,pFinalCodeSegment,(uint)(isFreeDefine == 0));
          }
        }
      }
      if (local_120 != vm_pGlobalProgram) {
                    /* Unresolved local var: int i@[DW_OP_reg2(EDX)] */
        sVar1 = local_120->maxFunctionNo;
        sVar15 = sVar1 + 0xf;
        if (-1 < sVar1) {
          sVar15 = sVar1;
        }
        sVar1 = sVar1 + 1 + (sVar15 >> 4);
        local_120->quickFunctionSize = sVar1;
        pVVar9 = vm_scratchLargeAlloc((int)sVar1 << 2);
        local_120->pQuickFunction = pVVar9;
        if (pVVar9 == (Vm_Quick_Function *)0x0) goto LAB_08088a6f;
        iVar14 = 0;
        if (0 < local_120->quickFunctionSize) {
          do {
            pVVar9[iVar14].funOffset = -1;
            pVVar9[iVar14].strNo = -1;
            iVar14 = iVar14 + 1;
          } while (iVar14 < local_120->quickFunctionSize);
        }
      }
      pVVar21 = vm_pGlobalProgram;
      if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
        pVVar21 = vm_pCurrentProgram;
      }
      vm_finalizeIdentifiersInProgram(pVVar21);
      if (isFreeDefine == 0) {
                    /* Unresolved local var: Vm_Defn_T * p@[DW_OP_reg2(EDX)]
                       Unresolved local var: int i@[DW_OP_reg1(ECX)] */
        iVar14 = 0;
        do {
          pVVar5 = vm_defns[iVar14];
          while (pVVar5 != (Vm_Defn_T *)0x0) {
            uVar2 = pVVar5->flags;
            if (((uVar2 & 0x8000) == 0) || ((uVar2 & 0x4000) != 0)) {
              pVVar5 = pVVar5->pNext;
            }
            else {
              pVVar5->flags = uVar2 & 0xffff7fff;
              pVVar5 = pVVar5->pNext;
            }
          }
          iVar14 = iVar14 + 1;
        } while (iVar14 < 0x40);
      }
      goto LAB_08088a76;
    }
                    /* Unresolved local var: Vm_Global_Var_T * pNewVars@[DW_OP_reg0(EAX)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
    if (vm_pGlobalVar == (Vm_Global_Var_T *)0x0) {
      pVVar11 = pdb_allocateMemory(vm_globalVar * 0xc,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c"
                                   ,0x1a10);
    }
    else {
      pVVar11 = pdb_reallocateMemory
                          (vm_pGlobalVar,vm_globalVar * 0xc,"VM",
                           "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1a13);
    }
    if (pVVar11 != (Vm_Global_Var_T *)0x0) {
      iVar18 = (int)vm_variableBC;
      iVar14 = vm_globalVar;
      vm_pGlobalVar = pVVar11;
      if (iVar18 < vm_globalVar) {
        iVar12 = iVar18 * 0xc;
        do {
          pVVar11 = vm_pGlobalVar;
          (&vm_pGlobalVar->attrib)[iVar12] = '\0';
          iVar14 = vm_globalVar;
          iVar18 = iVar18 + 1;
          *(undefined4 *)((int)&pVVar11->pValue + iVar12) = 0;
          *(undefined4 *)((int)&pVVar11->pName + iVar12) = 0;
          iVar12 = iVar12 + 0xc;
        } while (iVar18 < iVar14);
      }
      iVar18 = (int)vm_variableBC;
                    /* Unresolved local var: Vm_Value_T * pNewValue@[DW_OP_reg3(EBX)] */
      while ((vm_varTableSize = iVar18, iVar18 < iVar14 &&
             (pVVar13 = vm_safeNewValue(), iVar14 = vm_globalVar, pVVar13 != (Vm_Value_T *)0x0))) {
        iVar18 = vm_varTableSize + 1;
        vm_pGlobalVar[vm_varTableSize].pValue = pVVar13;
        iVar14 = vm_globalVar;
      }
      if (iVar14 <= vm_varTableSize) goto LAB_08088a03;
    }
  }
LAB_08088a6f:
  succ = 0;
LAB_08088a76:
  pVVar21 = vm_pGlobalProgram;
  if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
    pVVar21 = vm_pCurrentProgram;
  }
  vm_freeUnusedIdentifiersInProgram(pVVar21);
                    /* Unresolved local var: Vm_Defn_T * * pp@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Defn_T * q@[DW_OP_reg7(EDI)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  iVar14 = 0;
  do {
    pVVar7 = vm_defns[iVar14];
    pVVar5 = (Vm_Defn_T *)(vm_defns + iVar14);
    while (ptr_00 = pVVar7, ptr_00 != (Vm_Defn_T *)0x0) {
      uVar2 = ptr_00->flags;
      if ((uVar2 & 0x8000) == 0) {
        pVVar7 = ptr_00->pNext;
        pVVar5 = ptr_00;
      }
      else {
        pVVar5->pNext = ptr_00->pNext;
        if ((uVar2 & 8) == 0) {
          pdb_freeMemory(ptr_00->exps,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0xf6e);
        }
        pdb_freeMemory(ptr_00->name,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0xf70);
        pdb_freeMemory(ptr_00,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0xf71);
        pVVar7 = pVVar5->pNext;
      }
    }
    iVar14 = iVar14 + 1;
  } while (iVar14 < 0x40);
  vm_nexpands = 0;
  if (succ != 0) {
    if (local_120 != vm_pGlobalProgram) {
      vm_gatherProgramFunctionTable(local_120);
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNext@[DW_OP_reg3(EBX)] */
      iVar14 = 0;
      ppVVar10 = local_120->ppHashHead;
      do {
        pVVar3 = ppVVar10[iVar14];
        ihe = pVVar3;
        while (pVVar3 != (Vm_Ident_Hash_Elem_T *)0x0) {
          pVVar4 = ihe->pNext;
          iVar18 = vm_pickupFunAndfreeAll(ihe,local_120);
          if (iVar18 == 0) {
            vm_freeIdentInProgram(local_120,ihe);
            ppVVar10 = local_120->ppHashHead;
            pVVar3 = ppVVar10[iVar14];
            ihe = pVVar4;
          }
          else {
            ppVVar10 = local_120->ppHashHead;
            if (pVVar4 == ppVVar10[iVar14]) break;
            pVVar3 = ppVVar10[iVar14];
            ihe = pVVar4;
          }
        }
        iVar14 = iVar14 + 1;
      } while (iVar14 < 0x400);
      pdb_freeMemory(local_120->ppHashTable,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1676);
      local_120->ppHashTable = (Vm_Ident_Hash_Elem **)0x0;
      local_120->ppHashHead = (Vm_Ident_Hash_Elem **)0x0;
      vm_scratchFinalize(local_120->pQuickFunction);
                    /* Unresolved local var: Vm_Prog_Var_T * pProgVar@[DW_OP_reg3(EBX)] */
      for (ptr_02 = local_120->pProgVars; ptr_02 != (Vm_Prog_Var_T *)0x0; ptr_02 = ptr_02->pNext) {
        vm_scratchFinalize(ptr_02->pVarName);
        vm_scratchFinalize(ptr_02);
      }
    }
    if (succ != 0) {
      vm_pCurrentProgram = (Vm_Program_T *)0x0;
      return succ;
    }
  }
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  pVVar21 = vm_pGlobalProgram;
  pVVar16 = vm_pGlobalProgram;
  if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
    pVVar16 = vm_pCurrentProgram;
  }
  pVVar16->pHiddenFuns = vm_hiddenFunsBC;
  pVVar16->maxFunctionNo = vm_functionBC;
  if (pVVar16 == pVVar21) {
    iVar14 = (int)vm_variableBC;
    if (iVar14 < vm_varTableSize) {
      iVar18 = iVar14 * 0xc;
      pVVar11 = vm_pGlobalVar;
      do {
        vm_unrefValue(*(Vm_Value_T **)((int)&pVVar11->pValue + iVar18));
        pVVar11 = vm_pGlobalVar;
        iVar14 = iVar14 + 1;
        *(undefined4 *)((int)&vm_pGlobalVar->pValue + iVar18) = 0;
        iVar18 = iVar18 + 0xc;
      } while (iVar14 < vm_varTableSize);
    }
    vm_globalVar = (int)vm_variableBC;
    vm_varTableSize = vm_globalVar;
  }
  vm_pCurrentProgram = (Vm_Program_T *)0x0;
  return succ;
}


```

```text
0x8088874: PUSH EBP
0x8088875: MOV EBP, ESP
0x8088877: PUSH EDI
0x8088878: PUSH ESI
0x8088879: PUSH EBX
0x808887a: SUB ESP, 0x12c
0x8088880: MOV EAX, [0x0847aac0]
0x8088885: TEST EAX, EAX
0x8088887: MOV dword ptr [EBP + 0xfffffee4], EAX
0x808888d: JNZ 0x0808889b
0x808888f: MOV EDX, dword ptr [0x0847aad0]
0x8088895: MOV dword ptr [EBP + 0xfffffee4], EDX
0x808889b: MOV EAX, [0x0847df14]
0x80888a0: TEST EAX, EAX
0x80888a2: JZ 0x080888ee
0x80888a4: PUSH 0x53d
0x80888a9: PUSH 0x82ab14c
0x80888ae: PUSH 0x82a376b
0x80888b3: PUSH dword ptr [EAX]
0x80888b5: CALL 0x080ceb94
0x80888ba: ADD ESP, 0x10
0x80888bd: PUSH 0x53e
0x80888c2: PUSH 0x82ab14c
0x80888c7: PUSH 0x82a376b
0x80888cc: PUSH dword ptr [0x0847df14]
0x80888d2: CALL 0x080ceb94
0x80888d7: ADD ESP, 0x10
0x80888da: MOV dword ptr [0x0847df14], 0x0
0x80888e4: MOV dword ptr [0x0847df10], 0x0
0x80888ee: MOV EBX, dword ptr [0x0847acec]
0x80888f4: TEST EBX, EBX
0x80888f6: JZ 0x0808894c
0x80888f8: SUB ESP, 0xc
0x80888fb: PUSH dword ptr [0x0847fba0]
0x8088901: CALL 0x080a0504
0x8088906: MOV EAX, dword ptr [EBX + 0x14]
0x8088909: MOV [0x08661940], EAX
0x808890e: MOV EAX, dword ptr [EBX + 0x4]
0x8088911: MOV [0x0847fba0], EAX
0x8088916: MOV EAX, dword ptr [EBX + 0x8]
0x8088919: MOV [0x0847fba4], EAX
0x808891e: MOV EAX, dword ptr [EBX + 0xc]
0x8088921: MOV [0x0847fba8], EAX
0x8088926: MOV EAX, dword ptr [EBX]
0x8088928: MOV [0x0847acec], EAX
0x808892d: MOV dword ptr [ESP], EBX
0x8088930: MOV dword ptr [0x0847fbac], 0x0
0x808893a: CALL 0x080891d4
0x808893f: ADD ESP, 0x10
0x8088942: MOV EBX, dword ptr [0x0847acec]
0x8088948: TEST EBX, EBX
0x808894a: JNZ 0x080888f8
0x808894c: MOV EDX, dword ptr [0x0847df18]
0x8088952: TEST EDX, EDX
0x8088954: MOV dword ptr [0x0847acec], 0x0
0x808895e: JZ 0x08088980
0x8088960: MOV EAX, EDX
0x8088962: SUB ESP, 0xc
0x8088965: MOV EDX, dword ptr [EDX]
0x8088967: PUSH EAX
0x8088968: MOV dword ptr [0x0847df18], EDX
0x808896e: CALL 0x080891d4
0x8088973: ADD ESP, 0x10
0x8088976: MOV EDX, dword ptr [0x0847df18]
0x808897c: TEST EDX, EDX
0x808897e: JNZ 0x08088960
0x8088980: MOV ECX, dword ptr [EBP + 0x8]
0x8088983: TEST ECX, ECX
0x8088985: JZ 0x08088a76
0x808898b: MOV ECX, dword ptr [EBP + 0xfffffee4]
0x8088991: MOV AX, word ptr [ECX + 0x10]
0x8088995: CMP AX, word ptr [0x08660900]
0x808899c: JLE 0x080889f1
0x808899e: MOV EDX, dword ptr [ECX + 0x2c]
0x80889a1: TEST EDX, EDX
0x80889a3: JNZ 0x08088c60
0x80889a9: PUSH 0x19f3
0x80889ae: PUSH 0x82ab14c
0x80889b3: CWDE
0x80889b4: PUSH 0x82a376b
0x80889b9: SHL EAX, 0x3
0x80889bc: PUSH EAX
0x80889bd: CALL 0x080ce784
0x80889c2: ADD ESP, 0x10
0x80889c5: TEST EAX, EAX
0x80889c7: JZ 0x08088a6f
0x80889cd: MOV EDX, dword ptr [EBP + 0xfffffee4]
0x80889d3: MOVSX EDI, word ptr [0x08660900]
0x80889da: MOVSX ECX, word ptr [EDX + 0x10]
0x80889de: SUB ECX, EDI
0x80889e0: SHL ECX, 0x3
0x80889e3: MOV dword ptr [EDX + 0x2c], EAX
0x80889e6: LEA EDI, [EAX + EDI*0x8]
0x80889e9: CLD
0x80889ea: SHR ECX, 0x2
0x80889ed: XOR EAX, EAX
0x80889ef: STOSD.REP ES:EDI
0x80889f1: MOV ECX, dword ptr [EBP + 0xfffffee4]
0x80889f7: CMP ECX, dword ptr [0x0847aad0]
0x80889fd: JZ 0x08088d1a
0x8088a03: XOR EAX, EAX
0x8088a05: CMP dword ptr [EBP + 0xc], 0x0
0x8088a09: SETZ AL
0x8088a0c: MOV dword ptr [EBP + 0xfffffee0], EAX
0x8088a12: MOV EAX, [0x08660914]
0x8088a17: TEST EAX, EAX
0x8088a19: JNZ 0x08088a28
0x8088a1b: MOV EAX, [0x08661928]
0x8088a20: TEST EAX, EAX
0x8088a22: JS 0x08088db1
0x8088a28: MOV EBX, dword ptr [EBP + 0xfffffee4]
0x8088a2e: CMP EBX, dword ptr [0x0847aad0]
0x8088a34: JZ 0x08088cba
0x8088a3a: MOV AX, word ptr [EBX + 0x10]
0x8088a3e: CMP AX, -0x1
0x8088a42: LEA EDX, [EAX + 0xf]
0x8088a45: CMOVG EDX, EAX
0x8088a48: SAR DX, 0x4
0x8088a4c: LEA EAX, [EAX + EDX*0x1 + 0x1]
0x8088a50: MOV word ptr [EBX + 0x5c], AX
0x8088a54: CWDE
0x8088a55: SUB ESP, 0xc
0x8088a58: SHL EAX, 0x2
0x8088a5b: PUSH EAX
0x8088a5c: CALL 0x08089630
0x8088a61: ADD ESP, 0x10
0x8088a64: TEST EAX, EAX
0x8088a66: MOV dword ptr [EBX + 0x60], EAX
0x8088a69: JNZ 0x08088c8d
0x8088a6f: MOV dword ptr [EBP + 0x8], 0x0
0x8088a76: MOV EAX, [0x0847aac0]
0x8088a7b: TEST EAX, EAX
0x8088a7d: JZ 0x08088c52
0x8088a83: SUB ESP, 0xc
0x8088a86: PUSH EAX
0x8088a87: CALL 0x080877d0
0x8088a8c: XOR ESI, ESI
0x8088a8e: ADD ESP, 0x10
0x8088a91: LEA EAX, [ESI*0x4 + 0x0]
0x8088a98: MOV EDI, dword ptr [EAX + 0x847fbe0]
0x8088a9e: TEST EDI, EDI
0x8088aa0: LEA EBX, [EAX + 0x847fbe0]
0x8088aa6: JZ 0x08088abc
0x8088aa8: MOV EDX, dword ptr [EDI + 0xc]
0x8088aab: TEST DH, 0x80
0x8088aae: JNZ 0x08088bf4
0x8088ab4: MOV EBX, EDI
0x8088ab6: MOV EDI, dword ptr [EBX]
0x8088ab8: TEST EDI, EDI
0x8088aba: JNZ 0x08088aa8
0x8088abc: INC ESI
0x8088abd: CMP ESI, 0x3f
0x8088ac0: JLE 0x08088a91
0x8088ac2: MOV ECX, dword ptr [EBP + 0x8]
0x8088ac5: TEST ECX, ECX
0x8088ac7: MOV dword ptr [0x0847ace8], 0x0
0x8088ad1: JZ 0x08088bb0
0x8088ad7: MOV EAX, dword ptr [EBP + 0xfffffee4]
0x8088add: CMP EAX, dword ptr [0x0847aad0]
0x8088ae3: JZ 0x08088ba9
0x8088ae9: SUB ESP, 0xc
0x8088aec: PUSH EAX
0x8088aed: CALL 0x080a31bc
0x8088af2: MOV EDX, dword ptr [EBP + 0xfffffee4]
0x8088af8: ADD ESP, 0x10
0x8088afb: XOR ESI, ESI
0x8088afd: MOV EAX, dword ptr [EDX + 0x58]
0x8088b00: MOV EDI, dword ptr [EAX + ESI*0x4]
0x8088b03: TEST EDI, EDI
0x8088b05: JZ 0x08088b3c
0x8088b07: NOP
0x8088b08: SUB ESP, 0x8
0x8088b0b: MOV EBX, dword ptr [EDI + 0xc]
0x8088b0e: PUSH dword ptr [EBP + 0xfffffee4]
0x8088b14: PUSH EDI
0x8088b15: CALL 0x08087528
0x8088b1a: ADD ESP, 0x10
0x8088b1d: TEST EAX, EAX
0x8088b1f: JZ 0x08088e8c
0x8088b25: MOV EDI, EBX
0x8088b27: MOV EBX, dword ptr [EBP + 0xfffffee4]
0x8088b2d: MOV EAX, dword ptr [EBX + 0x58]
0x8088b30: CMP EDI, dword ptr [EAX + ESI*0x4]
0x8088b33: JZ 0x08088b3c
0x8088b35: MOV EDX, dword ptr [EAX + ESI*0x4]
0x8088b38: TEST EDX, EDX
0x8088b3a: JNZ 0x08088b08
0x8088b3c: INC ESI
0x8088b3d: CMP ESI, 0x3ff
0x8088b43: JLE 0x08088b00
0x8088b45: PUSH 0x1676
0x8088b4a: PUSH 0x82ab14c
0x8088b4f: PUSH 0x82a376b
0x8088b54: MOV EAX, dword ptr [EBP + 0xfffffee4]
0x8088b5a: PUSH dword ptr [EAX + 0x54]
0x8088b5d: CALL 0x080ceb94
0x8088b62: POP EAX
0x8088b63: MOV EDX, dword ptr [EBP + 0xfffffee4]
0x8088b69: MOV dword ptr [EDX + 0x54], 0x0
0x8088b70: MOV dword ptr [EDX + 0x58], 0x0
0x8088b77: PUSH dword ptr [EDX + 0x60]
0x8088b7a: CALL 0x08089b00
0x8088b7f: MOV ECX, dword ptr [EBP + 0xfffffee4]
0x8088b85: ADD ESP, 0x10
0x8088b88: MOV EBX, dword ptr [ECX + 0x28]
0x8088b8b: JMP 0x08088ba5
0x8088b8d: SUB ESP, 0xc
0x8088b90: PUSH dword ptr [EBX]
0x8088b92: CALL 0x08089b00
0x8088b97: MOV dword ptr [ESP], EBX
0x8088b9a: CALL 0x08089b00
0x8088b9f: ADD ESP, 0x10
0x8088ba2: MOV EBX, dword ptr [EBX + 0xc]
0x8088ba5: TEST EBX, EBX
0x8088ba7: JNZ 0x08088b8d
0x8088ba9: MOV EAX, dword ptr [EBP + 0x8]
0x8088bac: TEST EAX, EAX
0x8088bae: JNZ 0x08088bdf
0x8088bb0: MOV EAX, [0x0847aac0]
0x8088bb5: TEST EAX, EAX
0x8088bb7: JZ 0x08088eb3
0x8088bbd: MOV EDX, EAX
0x8088bbf: MOV ECX, dword ptr [0x0847aad0]
0x8088bc5: MOV EAX, [0x0866192c]
0x8088bca: MOV dword ptr [EDX + 0x30], EAX
0x8088bcd: CMP EDX, ECX
0x8088bcf: MOV AX, [0x08660900]
0x8088bd5: MOV word ptr [EDX + 0x10], AX
0x8088bd9: JZ 0x08088ec0
0x8088bdf: MOV EAX, dword ptr [EBP + 0x8]
0x8088be2: MOV dword ptr [0x0847aac0], 0x0
0x8088bec: LEA ESP, [EBP + -0xc]
0x8088bef: POP EBX
0x8088bf0: POP ESI
0x8088bf1: POP EDI
0x8088bf2: LEAVE
0x8088bf3: RET
0x8088bf4: MOV EAX, dword ptr [EDI]
0x8088bf6: AND EDX, 0x8
0x8088bf9: MOV dword ptr [EBX], EAX
0x8088bfb: JZ 0x08088c36
0x8088bfd: PUSH 0xf70
0x8088c02: PUSH 0x82ab14c
0x8088c07: PUSH 0x82a376b
0x8088c0c: PUSH dword ptr [EDI + 0x4]
0x8088c0f: CALL 0x080ceb94
0x8088c14: ADD ESP, 0x10
0x8088c17: PUSH 0xf71
0x8088c1c: PUSH 0x82ab14c
0x8088c21: PUSH 0x82a376b
0x8088c26: PUSH EDI
0x8088c27: CALL 0x080ceb94
0x8088c2c: ADD ESP, 0x10
0x8088c2f: MOV EDI, dword ptr [EBX]
0x8088c31: JMP 0x08088ab8
0x8088c36: PUSH 0xf6e
0x8088c3b: PUSH 0x82ab14c
0x8088c40: PUSH 0x82a376b
0x8088c45: PUSH dword ptr [EDI + 0x8]
0x8088c48: CALL 0x080ceb94
0x8088c4d: ADD ESP, 0x10
0x8088c50: JMP 0x08088bfd
0x8088c52: SUB ESP, 0xc
0x8088c55: PUSH dword ptr [0x0847aad0]
0x8088c5b: JMP 0x08088a87
0x8088c60: SUB ESP, 0xc
0x8088c63: PUSH 0x19f7
0x8088c68: PUSH 0x82ab14c
0x8088c6d: CWDE
0x8088c6e: PUSH 0x82a376b
0x8088c73: SHL EAX, 0x3
0x8088c76: PUSH EAX
0x8088c77: MOV EBX, dword ptr [EBP + 0xfffffee4]
0x8088c7d: PUSH dword ptr [EBX + 0x2c]
0x8088c80: CALL 0x080cf588
0x8088c85: ADD ESP, 0x20
0x8088c88: JMP 0x080889c5
0x8088c8d: MOV ECX, dword ptr [EBP + 0xfffffee4]
0x8088c93: XOR EDX, EDX
0x8088c95: CMP word ptr [ECX + 0x5c], 0x0
0x8088c9a: JLE 0x08088cba
0x8088c9c: MOV ECX, EAX
0x8088c9e: MOV word ptr [ECX + EDX*0x4 + 0x2], 0xffff
0x8088ca5: MOV word ptr [ECX + EDX*0x4], 0xffff
0x8088cab: MOV EBX, dword ptr [EBP + 0xfffffee4]
0x8088cb1: INC EDX
0x8088cb2: MOVSX EAX, word ptr [EBX + 0x5c]
0x8088cb6: CMP EAX, EDX
0x8088cb8: JG 0x08088c9e
0x8088cba: MOV EAX, [0x0847aac0]
0x8088cbf: TEST EAX, EAX
0x8088cc1: JZ 0x08088d0f
0x8088cc3: SUB ESP, 0xc
0x8088cc6: PUSH EAX
0x8088cc7: CALL 0x080854b4
0x8088ccc: MOV EBX, dword ptr [EBP + 0xc]
0x8088ccf: ADD ESP, 0x10
0x8088cd2: TEST EBX, EBX
0x8088cd4: JNZ 0x08088a76
0x8088cda: XOR ECX, ECX
0x8088cdc: MOV EDX, dword ptr [ECX*0x4 + 0x847fbe0]
0x8088ce3: TEST EDX, EDX
0x8088ce5: JZ 0x08088cf5
0x8088ce7: MOV EAX, dword ptr [EDX + 0xc]
0x8088cea: TEST AH, 0x80
0x8088ced: JNZ 0x08088d00
0x8088cef: MOV EDX, dword ptr [EDX]
0x8088cf1: TEST EDX, EDX
0x8088cf3: JNZ 0x08088ce7
0x8088cf5: INC ECX
0x8088cf6: CMP ECX, 0x3f
0x8088cf9: JLE 0x08088cdc
0x8088cfb: JMP 0x08088a76
0x8088d00: TEST AH, 0x40
0x8088d03: JNZ 0x08088cef
0x8088d05: AND AH, 0x7f
0x8088d08: MOV dword ptr [EDX + 0xc], EAX
0x8088d0b: MOV EDX, dword ptr [EDX]
0x8088d0d: JMP 0x08088cf1
0x8088d0f: SUB ESP, 0xc
0x8088d12: PUSH dword ptr [0x0847aad0]
0x8088d18: JMP 0x08088cc7
0x8088d1a: MOVSX EAX, word ptr [0x0866090c]
0x8088d21: MOV EDX, dword ptr [0x0847aac4]
0x8088d27: CMP EAX, EDX
0x8088d29: JGE 0x08088a03
0x8088d2f: MOV ECX, dword ptr [0x0847aacc]
0x8088d35: TEST ECX, ECX
0x8088d37: JNZ 0x08088e65
0x8088d3d: PUSH 0x1a10
0x8088d42: PUSH 0x82ab14c
0x8088d47: LEA EAX, [EDX + EDX*0x2]
0x8088d4a: PUSH 0x82a376b
0x8088d4f: SHL EAX, 0x2
0x8088d52: PUSH EAX
0x8088d53: CALL 0x080ce784
0x8088d58: ADD ESP, 0x10
0x8088d5b: TEST EAX, EAX
0x8088d5d: JZ 0x08088a6f
0x8088d63: MOV ECX, EAX
0x8088d65: MOV [0x0847aacc], EAX
0x8088d6a: MOV AX, [0x0866090c]
0x8088d70: MOVSX EBX, AX
0x8088d73: MOV EDX, dword ptr [0x0847aac4]
0x8088d79: CMP EBX, EDX
0x8088d7b: JGE 0x08088e1d
0x8088d81: LEA EAX, [EBX + EBX*0x2]
0x8088d84: SHL EAX, 0x2
0x8088d87: MOV byte ptr [EAX + ECX*0x1 + 0x8], 0x0
0x8088d8c: INC EBX
0x8088d8d: MOV EDX, dword ptr [0x0847aac4]
0x8088d93: MOV dword ptr [EAX + ECX*0x1], 0x0
0x8088d9a: MOV dword ptr [EAX + ECX*0x1 + 0x4], 0x0
0x8088da2: ADD EAX, 0xc
0x8088da5: CMP EBX, EDX
0x8088da7: JGE 0x08088e17
0x8088da9: MOV ECX, dword ptr [0x0847aacc]
0x8088daf: JMP 0x08088d87
0x8088db1: MOV EBX, dword ptr [0x08661934]
0x8088db7: TEST EBX, EBX
0x8088db9: MOV dword ptr [EBP + 0xfffffed4], EBX
0x8088dbf: JZ 0x08088a28
0x8088dc5: MOV EAX, 0x3
0x8088dca: MOV ESI, 0x82ab146
0x8088dcf: CLD
0x8088dd0: MOV EDI, EBX
0x8088dd2: MOV ECX, EAX
0x8088dd4: CMPSB.REPE ES:EDI, ESI
0x8088dd6: JZ 0x08088a28
0x8088ddc: PUSH EDI
0x8088ddd: PUSH 0x100
0x8088de2: LEA EBX, [EBP + 0xfffffee8]
0x8088de8: PUSH EBX
0x8088de9: PUSH dword ptr [EBP + 0xfffffed4]
0x8088def: CALL 0x080bdc10
0x8088df4: ADD ESP, 0x10
0x8088df7: TEST EAX, EAX
0x8088df9: JZ 0x08088a28
0x8088dff: PUSH ESI
0x8088e00: PUSH dword ptr [EBP + 0xfffffee0]
0x8088e06: PUSH dword ptr [EBP + 0x10]
0x8088e09: PUSH EBX
0x8088e0a: CALL 0x080b9018
0x8088e0f: ADD ESP, 0x10
0x8088e12: JMP 0x08088a28
0x8088e17: MOV AX, [0x0866090c]
0x8088e1d: CWDE
0x8088e1e: MOV [0x0847aac8], EAX
0x8088e23: JMP 0x08088e54
0x8088e28: CALL 0x08089f78
0x8088e2d: TEST EAX, EAX
0x8088e2f: MOV EBX, EAX
0x8088e31: JZ 0x08088f1e
0x8088e37: MOV EAX, [0x0847aac8]
0x8088e3c: LEA ECX, [EAX + EAX*0x2]
0x8088e3f: MOV EDX, dword ptr [0x0847aacc]
0x8088e45: INC EAX
0x8088e46: MOV dword ptr [EDX + ECX*0x4], EBX
0x8088e49: MOV [0x0847aac8], EAX
0x8088e4e: MOV EDX, dword ptr [0x0847aac4]
0x8088e54: CMP EAX, EDX
0x8088e56: JL 0x08088e28
0x8088e58: CMP EAX, EDX
0x8088e5a: JL 0x08088a6f
0x8088e60: JMP 0x08088a03
0x8088e65: SUB ESP, 0xc
0x8088e68: PUSH 0x1a13
0x8088e6d: PUSH 0x82ab14c
0x8088e72: LEA EAX, [EDX + EDX*0x2]
0x8088e75: PUSH 0x82a376b
0x8088e7a: SHL EAX, 0x2
0x8088e7d: PUSH EAX
0x8088e7e: PUSH ECX
0x8088e7f: CALL 0x080cf588
0x8088e84: ADD ESP, 0x20
0x8088e87: JMP 0x08088d5b
0x8088e8c: SUB ESP, 0x8
0x8088e8f: PUSH EDI
0x8088e90: PUSH dword ptr [EBP + 0xfffffee4]
0x8088e96: CALL 0x080856d8
0x8088e9b: MOV ECX, dword ptr [EBP + 0xfffffee4]
0x8088ea1: MOV EAX, dword ptr [ECX + 0x58]
0x8088ea4: MOV EDX, dword ptr [EAX + ESI*0x4]
0x8088ea7: ADD ESP, 0x10
0x8088eaa: MOV EDI, EBX
0x8088eac: TEST EDX, EDX
0x8088eae: JMP 0x08088b3a
0x8088eb3: MOV ECX, dword ptr [0x0847aad0]
0x8088eb9: MOV EDX, ECX
0x8088ebb: JMP 0x08088bc5
0x8088ec0: MOV AX, [0x0866090c]
0x8088ec6: MOVSX EBX, AX
0x8088ec9: CMP EBX, dword ptr [0x0847aac8]
0x8088ecf: JGE 0x08088f0e
0x8088ed1: LEA EAX, [EBX + EBX*0x2]
0x8088ed4: MOV ECX, dword ptr [0x0847aacc]
0x8088eda: LEA ESI, [EAX*0x4 + 0x0]
0x8088ee1: SUB ESP, 0xc
0x8088ee4: PUSH dword ptr [ESI + ECX*0x1]
0x8088ee7: CALL 0x0808bfb4
0x8088eec: INC EBX
0x8088eed: MOV ECX, dword ptr [0x0847aacc]
0x8088ef3: ADD ESP, 0x10
0x8088ef6: MOV dword ptr [ESI + ECX*0x1], 0x0
0x8088efd: ADD ESI, 0xc
0x8088f00: CMP EBX, dword ptr [0x0847aac8]
0x8088f06: JL 0x08088ee1
0x8088f08: MOV AX, [0x0866090c]
0x8088f0e: CWDE
0x8088f0f: MOV [0x0847aac8], EAX
0x8088f14: MOV [0x0847aac4], EAX
0x8088f19: JMP 0x08088bdf
0x8088f1e: MOV EDX, dword ptr [0x0847aac4]
0x8088f24: MOV EAX, [0x0847aac8]
0x8088f29: JMP 0x08088e58
```

限制：
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

## vm_finalizeIdentifiersInProgram / analyze_function

Evidence: `ev_a62e3110c8f80b68de1c768894f66d34b9dca9778fa69b0bb7f84f2504e73e0c`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8085599", "start": "0x80854b4"}, {"end": "0x80856a0", "start": "0x808559c"}], "span_bytes": 493, "total_bytes": 491}`

```c

/* WARNING: Unknown calling convention */

void vm_finalizeIdentifiersInProgram(Vm_Program_T *pProgram)

{
  Vm_Ident_Info_T *pVVar1;
  uchar *puVar2;
  Vm_Fun_T *pFun;
  Vm_Fun *pVVar3;
  Vm_Global_Var_T *pVVar4;
  uint uVar5;
  Vm_Ident_Hash_Elem **ppVVar6;
  uint uVar7;
  Vm_Ident_Info *pVVar8;
  Vm_Ident_Hash_Elem *pVVar9;
  int i;
  int iiProcessedFuns;
  
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Ident_Info_T * pIi@[DW_OP_reg6(ESI)] */
  i = 0;
  ppVVar6 = pProgram->ppHashHead;
  do {
    pVVar9 = ppVVar6[i];
    if (pVVar9 != (Vm_Ident_Hash_Elem *)0x0) {
      do {
        if ((pVVar9->token & 0xc0000000) == 0) {
          pVVar1 = &pVVar9->ii;
          iiProcessedFuns = 0;
          (pVVar9->ii).oldToken = pVVar9->token;
          pVVar8 = pVVar1;
          if (pVVar1 != (Vm_Ident_Info_T *)0x0) {
            do {
              while( true ) {
                pVVar4 = vm_pGlobalVar;
                uVar7 = pVVar8->oldToken;
                uVar5 = uVar7 >> 0x1b & 1;
                iiProcessedFuns = (iiProcessedFuns + 1) - (uint)(uVar5 == 0);
                if ((uVar7 & 0x30000000) != 0) break;
LAB_080854fe:
                pVVar8 = pVVar8->pNext;
                if (pVVar8 == (Vm_Ident_Info *)0x0) goto LAB_0808556d;
              }
              if ((((uVar5 == 0) || (pProgram != vm_pGlobalProgram)) ||
                  ((((pVVar8->v).uf.pFun)->attrib & 4) == 0)) &&
                 (((uVar7 & 0x8000000) == 0 || (iiProcessedFuns < 2)))) {
                if ((uVar7 & 0x80000) == 0) {
                  if ((uVar7 & 0x40000) == 0) {
                    if ((uVar7 & 0x8000000) == 0) {
                      vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                                 "Unknown type of identifer to be finalize.\n",
                                 "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x196e);
                      _vm_endProcess(0);
                      do {
                    /* WARNING: Do nothing block with infinite loop */
                      } while( true );
                    }
                    if ((uVar7 & 0x2000000) == 0) {
                      vm_finalizeFun((pVVar8->v).uf.pFun,(pVVar8->v).uf.funOffset,uVar7 & 0x10000000
                                    );
                      goto LAB_080855c1;
                    }
                  }
                }
                else {
                  vm_pGlobalVar[(pVVar8->v).value.no].pName = pVVar9->name;
                  pVVar4[(pVVar8->v).value.no].attrib = (pVVar8->v).value.attrib;
LAB_080855c1:
                  uVar7 = pVVar8->oldToken;
                }
                if (((pProgram == vm_pGlobalProgram) && ((uVar7 & 0x80000) != 0)) &&
                   (((pVVar8->v).value.attrib & 4) != 0)) {
                  pVVar8->oldToken = 0x100000;
                }
                else {
                  pVVar8->oldToken = uVar7 & 0xcfffffff;
                }
              }
              else if ((uVar7 & 0x2000000) == 0) {
                    /* Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg3(EBX)] */
                pFun = (pVVar8->v).uf.pFun;
                vm_finalizeFun(pFun,(pVVar8->v).uf.funOffset,0);
                pVVar3 = pProgram->pHiddenFuns;
                pFun->attrib = pFun->attrib | 0x100;
                pFun->pNext = pVVar3;
                pVVar8->oldToken = 0;
                pProgram->pHiddenFuns = pFun;
              }
              else {
                    /* Unresolved local var: Vm_Fun_Entry_T * pFunEntry@[???] */
                puVar2 = &pProgram->pFunctions[(pVVar8->v).uf.funOffset].attrib;
                *puVar2 = *puVar2 | 2;
              }
              if (pVVar1 != pVVar8) goto LAB_080854fe;
              pVVar8 = pVVar8->pNext;
              pVVar9->token = (pVVar9->ii).oldToken;
            } while (pVVar8 != (Vm_Ident_Info *)0x0);
LAB_0808556d:
            ppVVar6 = pProgram->ppHashHead;
          }
        }
        pVVar9 = pVVar9->pNext;
      } while (ppVVar6[i] != pVVar9);
    }
    i = i + 1;
    if (0x3ff < i) {
      return;
    }
  } while( true );
}


```

```text
0x80854b4: PUSH EBP
0x80854b5: MOV EBP, ESP
0x80854b7: PUSH EDI
0x80854b8: PUSH ESI
0x80854b9: PUSH EBX
0x80854ba: SUB ESP, 0xc
0x80854bd: MOV EAX, dword ptr [EBP + 0x8]
0x80854c0: MOV dword ptr [EBP + -0x14], 0x0
0x80854c7: MOV EDX, dword ptr [EAX + 0x58]
0x80854ca: MOV ECX, dword ptr [EBP + -0x14]
0x80854cd: MOV EDI, dword ptr [EDX + ECX*0x4]
0x80854d0: TEST EDI, EDI
0x80854d2: JZ 0x08085582
0x80854d8: MOV EAX, dword ptr [EDI + 0x4]
0x80854db: TEST EAX, 0xc0000000
0x80854e0: JNZ 0x08085573
0x80854e6: LEA ECX, [EDI + 0x14]
0x80854e9: TEST ECX, ECX
0x80854eb: MOV dword ptr [EBP + -0x18], ECX
0x80854ee: MOV ESI, ECX
0x80854f0: MOV dword ptr [EBP + -0x10], 0x0
0x80854f7: MOV dword ptr [EDI + 0x20], EAX
0x80854fa: JNZ 0x08085505
0x80854fc: JMP 0x08085573
0x80854fe: MOV ESI, dword ptr [ESI + 0x14]
0x8085501: TEST ESI, ESI
0x8085503: JZ 0x0808556d
0x8085505: MOV EDX, dword ptr [ESI + 0xc]
0x8085508: MOV EAX, EDX
0x808550a: SHR EAX, 0x1b
0x808550d: AND EAX, 0x1
0x8085510: CMP EAX, 0x1
0x8085513: SBB dword ptr [EBP + -0x10], -0x1
0x8085517: TEST EDX, 0x30000000
0x808551d: JZ 0x080854fe
0x808551f: TEST EAX, EAX
0x8085521: JZ 0x08085532
0x8085523: MOV EAX, dword ptr [EBP + 0x8]
0x8085526: CMP EAX, dword ptr [0x0847aad0]
0x808552c: JZ 0x08085629
0x8085532: TEST EDX, 0x8000000
0x8085538: JZ 0x0808559c
0x808553a: CMP dword ptr [EBP + -0x10], 0x1
0x808553e: JLE 0x0808559c
0x8085540: AND EDX, 0x2000000
0x8085546: JZ 0x08085641
0x808554c: MOV ECX, dword ptr [EBP + 0x8]
0x808554f: MOVSX EDX, word ptr [ESI + 0x4]
0x8085553: MOV EAX, dword ptr [ECX + 0x2c]
0x8085556: OR byte ptr [EAX + EDX*0x8 + 0x6], 0x2
0x808555b: CMP dword ptr [EBP + -0x18], ESI
0x808555e: JNZ 0x080854fe
0x8085560: MOV ESI, dword ptr [ESI + 0x14]
0x8085563: MOV EAX, dword ptr [EDI + 0x20]
0x8085566: TEST ESI, ESI
0x8085568: MOV dword ptr [EDI + 0x4], EAX
0x808556b: JNZ 0x08085505
0x808556d: MOV ECX, dword ptr [EBP + 0x8]
0x8085570: MOV EDX, dword ptr [ECX + 0x58]
0x8085573: MOV EDI, dword ptr [EDI + 0xc]
0x8085576: MOV EAX, dword ptr [EBP + -0x14]
0x8085579: CMP dword ptr [EDX + EAX*0x4], EDI
0x808557c: JNZ 0x080854d8
0x8085582: INC dword ptr [EBP + -0x14]
0x8085585: CMP dword ptr [EBP + -0x14], 0x3ff
0x808558c: JLE 0x080854ca
0x8085592: LEA ESP, [EBP + -0xc]
0x8085595: POP EBX
0x8085596: POP ESI
0x8085597: POP EDI
0x8085598: LEAVE
0x8085599: RET
0x808559c: TEST EDX, 0x80000
0x80855a2: JZ 0x080855db
0x80855a4: MOV EAX, dword ptr [ESI]
0x80855a6: MOV EDX, dword ptr [EDI]
0x80855a8: MOV ECX, dword ptr [0x0847aacc]
0x80855ae: LEA EAX, [EAX + EAX*0x2]
0x80855b1: MOV dword ptr [ECX + EAX*0x4 + 0x4], EDX
0x80855b5: MOV EAX, dword ptr [ESI]
0x80855b7: LEA EAX, [EAX + EAX*0x2]
0x80855ba: MOV DL, byte ptr [ESI + 0x4]
0x80855bd: MOV byte ptr [ECX + EAX*0x4 + 0x8], DL
0x80855c1: MOV EDX, dword ptr [ESI + 0xc]
0x80855c4: MOV EAX, EDX
0x80855c6: MOV EDX, dword ptr [EBP + 0x8]
0x80855c9: AND EAX, 0xcfffffff
0x80855ce: CMP EDX, dword ptr [0x0847aad0]
0x80855d4: JZ 0x08085610
0x80855d6: MOV dword ptr [ESI + 0xc], EAX
0x80855d9: JMP 0x0808555b
0x80855db: TEST EDX, 0x40000
0x80855e1: JNZ 0x080855c4
0x80855e3: TEST EDX, 0x8000000
0x80855e9: JZ 0x08085672
0x80855ef: TEST EDX, 0x2000000
0x80855f5: JNZ 0x080855c4
0x80855f7: AND EDX, 0x10000000
0x80855fd: PUSH EAX
0x80855fe: PUSH EDX
0x80855ff: MOVSX EAX, word ptr [ESI + 0x4]
0x8085603: PUSH EAX
0x8085604: PUSH dword ptr [ESI]
0x8085606: CALL 0x08095794
0x808560b: ADD ESP, 0x10
0x808560e: JMP 0x080855c1
0x8085610: TEST EAX, 0x80000
0x8085615: JZ 0x080855d6
0x8085617: TEST byte ptr [ESI + 0x4], 0x4
0x808561b: JZ 0x080855d6
0x808561d: MOV dword ptr [ESI + 0xc], 0x100000
0x8085624: JMP 0x0808555b
0x8085629: MOV EAX, dword ptr [ESI]
0x808562b: TEST byte ptr [EAX + 0x8], 0x4
0x808562f: JZ 0x08085532
0x8085635: AND EDX, 0x2000000
0x808563b: JNZ 0x0808554c
0x8085641: MOV EBX, dword ptr [ESI]
0x8085643: PUSH EAX
0x8085644: PUSH 0x0
0x8085646: MOVSX EAX, word ptr [ESI + 0x4]
0x808564a: PUSH EAX
0x808564b: PUSH EBX
0x808564c: CALL 0x08095794
0x8085651: MOV EDX, dword ptr [EBP + 0x8]
0x8085654: MOV EAX, dword ptr [EDX + 0x30]
0x8085657: OR word ptr [EBX + 0x8], 0x100
0x808565d: ADD ESP, 0x10
0x8085660: MOV dword ptr [EBX + 0x3c], EAX
0x8085663: MOV dword ptr [ESI + 0xc], 0x0
0x808566a: MOV dword ptr [EDX + 0x30], EBX
0x808566d: JMP 0x0808555b
0x8085672: SUB ESP, 0x8
0x8085675: PUSH 0x196e
0x808567a: PUSH 0x82ab14c
0x808567f: PUSH 0x82ab664
0x8085684: PUSH 0x82a825c
0x8085689: PUSH 0x4
0x808568b: PUSH 0x2
0x808568d: CALL 0x080b61c4
0x8085692: ADD ESP, 0x14
0x8085695: PUSH 0x0
0x8085697: CALL 0x080a3980
0x808569c: ADD ESP, 0x10
0x808569f: JMP 0x0808569f
```

限制：
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

## vm_destructIi / analyze_function

Evidence: `ev_b4ed8ce5b95fd374e42fe1750e7879f34683730bc1a71eadda3c3c2707dc450e`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80876bc", "start": "0x8087640"}, {"end": "0x8087753", "start": "0x80876c0"}], "span_bytes": 276, "total_bytes": 273}`

```c

void __regparm1 vm_destructIi(Vm_Program_T *pProgram,Vm_Ident_Hash_Elem_T *ihe,Vm_Ident_Info_T *pIi)

{
  Vm_Ident_Hash_Elem *pVVar1;
  Vm_Global_Var_T *pVVar2;
  
  pVVar1 = ihe->pNext;
  if ((pVVar1 != (Vm_Ident_Hash_Elem *)0x0) && (((uint)pVVar1 & 0x2000000) == 0)) {
    if (((uint)pVVar1 & 0x20000000) == 0) {
      if (((uint)pVVar1 & 0x180000) == 0) {
        if ((((((uint)pVVar1 & 0x40000) == 0) && (((uint)pVVar1 & 0x8000000) == 0)) &&
            (((uint)pVVar1 & 0x20000) == 0)) && ((short)pVVar1 != 0x10c)) {
          vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                     "Unknown type of identifer to be destruct.\n",
                     "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19c0);
          _vm_endProcess(0);
          do {
                    /* WARNING: Do nothing block with infinite loop */
          } while( true );
        }
      }
      else {
                    /* Unresolved local var: Vm_Value_T * pValue@[???] */
        vm_unrefValue(vm_pGlobalVar[(int)ihe->name].pValue);
        pVVar2 = vm_pGlobalVar;
        vm_pGlobalVar[(int)ihe->name].pName = (char *)0x0;
        pVVar2[(int)ihe->name].pValue = (Vm_Value_T *)0x0;
      }
    }
    else if ((((uint)pVVar1 & 0x80000) == 0) && (((uint)pVVar1 & 0x40000) == 0)) {
      if (((uint)pVVar1 & 0x8000000) == 0) {
        vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                   "Unknown type of identifer to be destruct.\n",
                   "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1992);
        _vm_endProcess(0);
        do {
                    /* WARNING: Do nothing block with infinite loop */
        } while( true );
      }
      vm_freeFun(pProgram,(Vm_Fun_T *)ihe->name,0x16);
    }
  }
  return;
}


```

```text
0x8087640: PUSH EBP
0x8087641: MOV EBP, ESP
0x8087643: PUSH EBX
0x8087644: PUSH ECX
0x8087645: MOV EBX, dword ptr [EBP + 0x8]
0x8087648: MOV EDX, dword ptr [EBX + 0xc]
0x808764b: TEST EDX, EDX
0x808764d: JZ 0x080876e6
0x8087653: TEST EDX, 0x2000000
0x8087659: JNZ 0x080876e6
0x808765f: TEST EDX, 0x20000000
0x8087665: JNZ 0x080876c0
0x8087667: TEST EDX, 0x180000
0x808766d: JNZ 0x080876eb
0x808766f: TEST EDX, 0x40000
0x8087675: JNZ 0x080876e6
0x8087677: TEST EDX, 0x8000000
0x808767d: JNZ 0x080876e6
0x808767f: TEST EDX, 0x20000
0x8087685: JNZ 0x080876e6
0x8087687: CMP DX, 0x10c
0x808768c: JZ 0x080876e6
0x808768e: SUB ESP, 0x8
0x8087691: PUSH 0x19c0
0x8087696: PUSH 0x82ab14c
0x808769b: PUSH 0x82aba48
0x80876a0: PUSH 0x82a825c
0x80876a5: PUSH 0x4
0x80876a7: PUSH 0x2
0x80876a9: CALL 0x080b61c4
0x80876ae: ADD ESP, 0x14
0x80876b1: PUSH 0x0
0x80876b3: CALL 0x080a3980
0x80876b8: ADD ESP, 0x10
0x80876bb: JMP 0x080876bb
0x80876c0: TEST EDX, 0x80000
0x80876c6: JNZ 0x080876e6
0x80876c8: TEST EDX, 0x40000
0x80876ce: JNZ 0x080876e6
0x80876d0: AND EDX, 0x8000000
0x80876d6: JZ 0x08087725
0x80876d8: PUSH EDX
0x80876d9: PUSH 0x16
0x80876db: PUSH dword ptr [EBX]
0x80876dd: PUSH EAX
0x80876de: CALL 0x08095c88
0x80876e3: ADD ESP, 0x10
0x80876e6: MOV EBX, dword ptr [EBP + -0x4]
0x80876e9: LEAVE
0x80876ea: RET
0x80876eb: MOV EAX, dword ptr [EBX]
0x80876ed: LEA EAX, [EAX + EAX*0x2]
0x80876f0: SUB ESP, 0xc
0x80876f3: MOV EDX, dword ptr [0x0847aacc]
0x80876f9: PUSH dword ptr [EDX + EAX*0x4]
0x80876fc: CALL 0x0808bfb4
0x8087701: MOV EAX, dword ptr [EBX]
0x8087703: MOV EDX, dword ptr [0x0847aacc]
0x8087709: LEA EAX, [EAX + EAX*0x2]
0x808770c: MOV dword ptr [EDX + EAX*0x4 + 0x4], 0x0
0x8087714: MOV EAX, dword ptr [EBX]
0x8087716: LEA EAX, [EAX + EAX*0x2]
0x8087719: ADD ESP, 0x10
0x808771c: MOV dword ptr [EDX + EAX*0x4], 0x0
0x8087723: JMP 0x080876e6
0x8087725: SUB ESP, 0x8
0x8087728: PUSH 0x1992
0x808772d: PUSH 0x82ab14c
0x8087732: PUSH 0x82aba48
0x8087737: PUSH 0x82a825c
0x808773c: PUSH 0x4
0x808773e: PUSH 0x2
0x8087740: CALL 0x080b61c4
0x8087745: ADD ESP, 0x14
0x8087748: PUSH 0x0
0x808774a: CALL 0x080a3980
0x808774f: ADD ESP, 0x10
0x8087752: JMP 0x08087752
```

限制：
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

## vm_getGlobalValue / analyze_function

Evidence: `ev_b9ce656e748a1b80a2c6e5f0400f66705f486faea386f140748ad05c4527bb9d`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8090ae7", "start": "0x8090ad4"}], "span_bytes": 20, "total_bytes": 20}`

```c

Vm_Value * vm_getGlobalValue(int valueNo)

{
  return vm_pGlobalVar[valueNo].pValue;
}


```

```text
0x8090ad4: PUSH EBP
0x8090ad5: MOV EBP, ESP
0x8090ad7: MOV EAX, dword ptr [EBP + 0x8]
0x8090ada: MOV EDX, dword ptr [0x0847aacc]
0x8090ae0: LEA EAX, [EAX + EAX*0x2]
0x8090ae3: MOV EAX, dword ptr [EDX + EAX*0x4]
0x8090ae6: LEAVE
0x8090ae7: RET
```

限制：
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

## vm_pushGlobalVar / analyze_function

Evidence: `ev_f5d36761261bd90bb2139ae2bcd05e79af64a90eaffe2b978b3c0353a3ffe4a2`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80911a6", "start": "0x809113c"}], "span_bytes": 107, "total_bytes": 107}`

```c

void vm_pushGlobalVar(int valueNo)

{
  Vm_Value_T *pValue;
  
  pValue = vm_pGlobalVar[valueNo].pValue;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_copyValue(vm_stack + vm_sp,pValue);
  return;
}


```

```text
0x809113c: PUSH EBP
0x809113d: MOV EBP, ESP
0x809113f: PUSH EBX
0x8091140: PUSH EBX
0x8091141: MOV EAX, dword ptr [EBP + 0x8]
0x8091144: LEA EAX, [EAX + EAX*0x2]
0x8091147: MOV EDX, dword ptr [0x0847aacc]
0x809114d: MOV EBX, dword ptr [EDX + EAX*0x4]
0x8091150: MOV EAX, [0x086619cc]
0x8091155: TEST EAX, EAX
0x8091157: JZ 0x08091190
0x8091159: DEC EAX
0x809115a: MOV [0x086619cc], EAX
0x809115f: MOV EDX, dword ptr [0x08661964]
0x8091165: LEA EAX, [EAX + EAX*0x2]
0x8091168: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x809116d: MOV EAX, [0x086619cc]
0x8091172: SUB ESP, 0x8
0x8091175: MOV EDX, dword ptr [0x08661964]
0x809117b: LEA EAX, [EAX + EAX*0x2]
0x809117e: PUSH EBX
0x809117f: LEA EAX, [EDX + EAX*0x8]
0x8091182: PUSH EAX
0x8091183: CALL 0x0808be24
0x8091188: ADD ESP, 0x10
0x809118b: MOV EBX, dword ptr [EBP + -0x4]
0x809118e: LEAVE
0x809118f: RET
0x8091190: SUB ESP, 0xc
0x8091193: PUSH 0x82ad98c
0x8091198: CALL 0x08097b24
0x809119d: ADD ESP, 0x10
0x80911a0: MOV EAX, [0x086619cc]
0x80911a5: JMP 0x08091159
```

限制：
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

## vm_unrefValue / procedure_pseudo_code

Evidence: `ev_7caa046f8081143ac8be588f8afdae8a94f7aa7a36d7146bc503150e7f9475a0`

```text

void vm_unrefValue(Vm_Value_T *pValue)

{
  Vm_Value *pVVar1;
  int iVar2;
  
  iVar2 = pValue->ref + -1;
  pValue->ref = iVar2;
  if ((iVar2 == 0) && (pValue != (Vm_Value_T *)0x0)) {
    vm_cleanupValue(pValue);
    vm_totalValues = vm_totalValues + -1;
    if (pValue == vm_allValues) {
      vm_allValues = pValue->pNext;
      if (vm_allValues != (Vm_Value_T *)0x0) {
        vm_allValues->pPrev = (Vm_Value *)0x0;
      }
    }
    else {
      pVVar1 = pValue->pPrev;
      pVVar1->pNext = pValue->pNext;
      if (pValue->pNext != (Vm_Value *)0x0) {
        pValue->pNext->pPrev = pVVar1;
      }
    }
    pValue->pThis = (Vm_Value *)0x0;
    pdb_freeTinyBlock(pValue);
    return;
  }
  return;
}


```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- Ghidra Listing reports a terminal call at 0x808c001 to 0x80cf214 pdb_freeTinyBlock (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushGlobalVptr / procedure_pseudo_code

Evidence: `ev_e0b26424bac92dc860aadf077630177dfff6e85d228ddea8d8a8e7a77fb8b7ee`

```text

void vm_pushGlobalVptr(int valueNo)

{
  uchar uVar1;
  Vm_Value_T *pVVar2;
  Vm_Value_T *pVVar3;
  int iVar4;
  
  pVVar2 = vm_pGlobalVar[valueNo].pValue;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\b';
  iVar4 = vm_sp;
  pVVar3 = vm_stack;
  uVar1 = pVVar2->attrib;
  pVVar2->ref = pVVar2->ref + 1;
  pVVar3[iVar4].v.pPtrValue = pVVar2;
  pVVar3[iVar4].attrib = uVar1;
  return;
}


```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_copyValue / analyze_function

Evidence: `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808bee8", "start": "0x808be24"}], "span_bytes": 197, "total_bytes": 197}`

```c

void vm_copyValue(Vm_Value_T *pToValue,Vm_Value_T *pValue)

{
  uchar uVar1;
  Vm_String_T *pVVar2;
  anon_union_4_11_5000f750_for_v aVar3;
  Vm_Value *pVVar4;
  uchar uVar5;
  
                    /* Unresolved local var: int newType@[DW_OP_reg1(ECX)] */
  uVar5 = pValue->type;
  switch(uVar5) {
  case '\x03':
    pToValue->v = pValue->v;
    break;
  case '\x04':
  case '\x05':
    aVar3 = (anon_union_4_11_5000f750_for_v)(pValue->v).string;
    (aVar3.string)->ref = (aVar3.string)->ref + 1;
    goto LAB_0808be95;
  case '\x06':
  case '\a':
    pToValue->v = pValue->v;
    pVVar4 = (Vm_Value *)(pValue->v).string;
    goto LAB_0808bec6;
  case '\b':
    pVVar4 = (Vm_Value *)(pValue->v).string;
    goto LAB_0808bec3;
  case '\t':
    pVVar4 = (pValue->v).pPtrValue;
    if ((pVVar4->pNext != (Vm_Value *)0x0) && (((uint)pVVar4->pNext->pNext & 0x8000) != 0)) {
LAB_0808be9a:
      vm_cleanupValue(pValue);
      uVar5 = pValue->type;
      (pToValue->v).info = (void *)0x0;
      break;
    }
LAB_0808bec3:
    (pToValue->v).pPtrValue = pVVar4;
LAB_0808bec6:
    pVVar4->ref = pVVar4->ref + 1;
    break;
  case '\n':
    pVVar2 = (pValue->v).string;
    if (((uint)pVVar2->ptr & 0x8000) != 0) goto LAB_0808be9a;
    pVVar2->ref = pVVar2->ref + 1;
  case '\x02':
    aVar3 = pValue->v;
LAB_0808be95:
    pToValue->v = aVar3;
    break;
  default:
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value to copy.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x237);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  case '\x14':
    uVar5 = '\0';
  case '\0':
  case '\x01':
    (pToValue->v).info = (void *)0x0;
  }
  uVar1 = pValue->attrib;
  pToValue->type = uVar5;
  pToValue->attrib = uVar1;
  return;
}


```

```text
0x808be24: PUSH EBP
0x808be25: MOV EBP, ESP
0x808be27: PUSH ESI
0x808be28: PUSH EBX
0x808be29: MOV EBX, dword ptr [EBP + 0xc]
0x808be2c: MOVZX ECX, byte ptr [EBX + 0x4]
0x808be30: CMP ECX, 0x14
0x808be33: MOV ESI, dword ptr [EBP + 0x8]
0x808be36: JA 0x0808be3f
0x808be38: JMP dword ptr [ECX*0x4 + 0x82acec4]
0x808be3f: SUB ESP, 0x8
0x808be42: PUSH 0x237
0x808be47: PUSH 0x82acaf8
0x808be4c: PUSH 0x82acd91
0x808be51: PUSH 0x82a825c
0x808be56: PUSH 0x4
0x808be58: PUSH 0x2
0x808be5a: CALL 0x080b61c4
0x808be5f: ADD ESP, 0x14
0x808be62: PUSH 0x0
0x808be64: CALL 0x080a3980
0x808be69: ADD ESP, 0x10
0x808be6c: JMP 0x0808be6c
0x808be6e: XOR ECX, ECX
0x808be70: MOV dword ptr [ESI + 0x8], 0x0
0x808be77: MOV AL, byte ptr [EBX + 0x5]
0x808be7a: MOV byte ptr [ESI + 0x4], CL
0x808be7d: MOV byte ptr [ESI + 0x5], AL
0x808be80: LEA ESP, [EBP + -0x8]
0x808be83: POP EBX
0x808be84: POP ESI
0x808be85: LEAVE
0x808be86: RET
0x808be87: MOV EAX, dword ptr [EBX + 0x8]
0x808be8a: TEST byte ptr [EAX + 0xd], 0x80
0x808be8e: JNZ 0x0808be9a
0x808be90: INC dword ptr [EAX]
0x808be92: MOV EAX, dword ptr [EBX + 0x8]
0x808be95: MOV dword ptr [ESI + 0x8], EAX
0x808be98: JMP 0x0808be77
0x808be9a: SUB ESP, 0xc
0x808be9d: PUSH EBX
0x808be9e: CALL 0x0808ad98
0x808bea3: MOVZX ECX, byte ptr [EBX + 0x4]
0x808bea7: ADD ESP, 0x10
0x808beaa: MOV dword ptr [ESI + 0x8], 0x0
0x808beb1: JMP 0x0808be77
0x808beb3: MOV EAX, dword ptr [EBX + 0x8]
0x808beb6: MOV EDX, dword ptr [EAX + 0xc]
0x808beb9: TEST EDX, EDX
0x808bebb: JZ 0x0808bec3
0x808bebd: TEST byte ptr [EDX + 0xd], 0x80
0x808bec1: JNZ 0x0808be9a
0x808bec3: MOV dword ptr [ESI + 0x8], EAX
0x808bec6: INC dword ptr [EAX]
0x808bec8: JMP 0x0808be77
0x808beca: MOV EAX, dword ptr [EBX + 0x8]
0x808becd: INC dword ptr [EAX]
0x808becf: JMP 0x0808be95
0x808bed1: MOV EAX, dword ptr [EBX + 0x8]
0x808bed4: MOV dword ptr [ESI + 0x8], EAX
0x808bed7: JMP 0x0808be77
0x808bed9: MOV EAX, dword ptr [EBX + 0x8]
0x808bedc: JMP 0x0808bec3
0x808bede: MOV EAX, dword ptr [EBX + 0x8]
0x808bee1: MOV dword ptr [ESI + 0x8], EAX
0x808bee4: MOV EAX, dword ptr [EBX + 0x8]
0x808bee7: JMP 0x0808bec6
```

限制：
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

## vm_callInUserFunBody / procedure_assembly

Evidence: `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41`

```text
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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_callInUserFunBody / procedure_pseudo_code

Evidence: `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`

```text

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_callInUserFun / procedure_pseudo_code

Evidence: `ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97`

```text

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_callInUserFun / procedure_assembly

Evidence: `ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347`

```text
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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_returnFromFun / procedure_pseudo_code

Evidence: `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2`

```text

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_returnFromFun / procedure_assembly

Evidence: `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610`

```text
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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushThisObject / analyze_function

Evidence: `ev_0f52440a23f0b659faa95e8a973f08f1b228163126ef3957b926b6685f6d9b78`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80975f8", "start": "0x80975d8"}, {"end": "0x8097609", "start": "0x80975fc"}], "span_bytes": 50, "total_bytes": 47}`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_pushThisObject(void)

{
  if (vm_currentContext->thisObject == (Vm_Object_T *)0x0) {
    vm_pushNumber(0);
    return;
  }
  vm_pushObject(vm_currentContext->thisObject);
  return;
}


```

```text
0x80975d8: PUSH EBP
0x80975d9: MOV EBP, ESP
0x80975db: SUB ESP, 0x8
0x80975de: MOV EAX, [0x086619d8]
0x80975e3: MOV EAX, dword ptr [EAX + 0x20]
0x80975e6: TEST EAX, EAX
0x80975e8: JNZ 0x080975fc
0x80975ea: SUB ESP, 0xc
0x80975ed: PUSH 0x0
0x80975ef: CALL 0x08090b3c
0x80975f4: ADD ESP, 0x10
0x80975f7: LEAVE
0x80975f8: RET
0x80975fc: SUB ESP, 0xc
0x80975ff: PUSH EAX
0x8097600: CALL 0x080912f4
0x8097605: ADD ESP, 0x10
0x8097608: LEAVE
0x8097609: RET
```

限制：
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

## vm_initIdentifiers / analyze_function

Evidence: `ev_22f100323748feab029789e6127317398048c5dacbd93c84c0581a7a990ca1e2`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80866d9", "start": "0x8086678"}, {"end": "0x8086842", "start": "0x80866dc"}, {"end": "0x8086889", "start": "0x8086844"}, {"end": "0x8086978", "start": "0x808688c"}], "span_bytes": 769, "total_bytes": 764}`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initIdentifiers(void)

{
  uint *puVar1;
  Vm_Ident_Hash_Elem *pVVar2;
  char *s;
  Vm_Ident_Hash_Elem_T *pVVar3;
  Vm_Ident_Hash_Elem *pVVar4;
  ushort uVar5;
  int iVar6;
  Vm_Fun_Entry_T *pVVar7;
  Vm_Ident_Hash_Elem **ppVVar8;
  uint uVar9;
  Vm_Ident_Hash_Elem_T *ihe;
  int iVar10;
  Vm_Program_T *pVVar11;
  int i;
  int h;
  
                    /* Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  currentFile = "__init__";
  currentLine = 0;
  vm_pGlobalProgram = vm_buildProgramHead("global");
  if (vm_pGlobalProgram == (Vm_Program *)0x0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
               "No enough memory to init global identifiers.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x148b);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  iVar6 = lzss_init();
  iVar10 = 0;
  if (iVar6 == 0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Failed to init LZSS module.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1492);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  for (; s = *(char **)((int)&vm_resWords[0].word + iVar10), s != (char *)0x0;
      iVar10 = iVar10 + 0x14) {
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * pNode@[DW_OP_reg1(ECX)] */
    uVar5 = vm_whashstr(s,0x20);
    pVVar11 = vm_pGlobalProgram;
    uVar9 = uVar5 & 0x3ff;
    pVVar2 = vm_pGlobalProgram->ppHashTable[uVar9];
    pVVar4 = (Vm_Ident_Hash_Elem *)((int)&vm_resWords[0].word + iVar10);
    if (pVVar2 == (Vm_Ident_Hash_Elem *)0x0) {
      vm_pGlobalProgram->ppHashHead[uVar9] = pVVar4;
      ppVVar8 = (Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10);
      *(Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10) = pVVar4;
    }
    else {
      *(Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10) = pVVar2->pNext;
      pVVar2->pNext->ppPrev = (Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10);
      pVVar2->pNext = pVVar4;
      ppVVar8 = &pVVar2->pNext;
    }
    *(Vm_Ident_Hash_Elem ***)((int)&vm_resWords[0].ppPrev + iVar10) = ppVVar8;
    ppVVar8 = pVVar11->ppHashTable;
    puVar1 = (uint *)((int)&vm_resWords[0].token + iVar10);
    *puVar1 = *puVar1 | 0x80000000;
    ppVVar8[uVar9] = pVVar4;
  }
  vm_addPredefines();
  vm_efunInit();
  vm_efunDebugInit();
  vm_efunPerformanceProfileInit();
  vm_efunVarInit();
  vm_itableInit();
  vm_itreeInit();
  vm_dpInit();
  vm_resourceEfunInit();
  vm_graphInit();
  vm_socketInit();
  vm_procInit();
  vm_fileInit();
  vm_objectInit();
  vm_pdbInit();
  vm_rpcInit();
  for (iVar6 = 0; pVVar11 = vm_pGlobalProgram, iVar6 < totalEfunModules; iVar6 = iVar6 + 1) {
    (*efunModule[iVar6].pInit)();
  }
                    /* Unresolved local var: int i@[DW_OP_reg2(EDX)] */
  vm_pGlobalVar = (Vm_Global_Var_T *)0x0;
  vm_varTableSize = 0;
  vm_globalVar = 0;
  if (vm_pGlobalProgram->pFunctions == (Vm_Fun_Entry_T *)0x0) {
    pVVar7 = pdb_allocateMemory((int)vm_pGlobalProgram->maxFunctionNo << 3,"VM",
                                "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1549);
  }
  else {
    pVVar7 = pdb_reallocateMemory
                       (vm_pGlobalProgram->pFunctions,(int)vm_pGlobalProgram->maxFunctionNo << 3,
                        "VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x154f);
  }
  pVVar11->pFunctions = pVVar7;
  if (vm_pGlobalProgram->pFunctions == (Vm_Fun_Entry_T *)0x0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
               "No enough memory to build function table.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1553);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  pVVar11 = vm_pGlobalProgram;
  for (iVar6 = (int)vm_efunsCount; iVar6 < pVVar11->maxFunctionNo; iVar6 = iVar6 + 1) {
    pVVar7 = pVVar11->pFunctions;
    pVVar7[iVar6].attrib = '\0';
    pVVar11 = vm_pGlobalProgram;
    pVVar7[iVar6].pFun = (Vm_Fun *)0x0;
    pVVar11->pFunctions[iVar6].consititutionNo = 0;
  }
  vm_efunsCount = pVVar11->maxFunctionNo;
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNext@[DW_OP_reg6(ESI)] */
  i = 0;
  ppVVar8 = pVVar11->ppHashHead;
  do {
    pVVar3 = ppVVar8[i];
    ihe = pVVar3;
    while (pVVar3 != (Vm_Ident_Hash_Elem_T *)0x0) {
      pVVar4 = ihe->pNext;
      iVar6 = vm_buildEfunOffset(ihe,(void *)0x0);
      if (iVar6 == 0) {
        vm_freeIdentInProgram(pVVar11,ihe);
        ppVVar8 = pVVar11->ppHashHead;
      }
      else {
        ppVVar8 = pVVar11->ppHashHead;
        if (pVVar4 == ppVVar8[i]) break;
      }
      ihe = pVVar4;
      pVVar3 = ppVVar8[i];
    }
    i = i + 1;
    if (0x3ff < i) {
      return 1;
    }
  } while( true );
}


```

```text
0x8086678: PUSH EBP
0x8086679: MOV EBP, ESP
0x808667b: PUSH EDI
0x808667c: PUSH ESI
0x808667d: PUSH EBX
0x808667e: SUB ESP, 0x18
0x8086681: PUSH 0x82b7052
0x8086686: MOV dword ptr [0x08661940], 0x82ab094
0x8086690: MOV dword ptr [0x08661924], 0x0
0x808669a: CALL 0x0808651c
0x808669f: ADD ESP, 0x10
0x80866a2: TEST EAX, EAX
0x80866a4: MOV [0x0847aad0], EAX
0x80866a9: JNZ 0x080866dc
0x80866ab: SUB ESP, 0x8
0x80866ae: PUSH 0x148b
0x80866b3: PUSH 0x82ab14c
0x80866b8: PUSH 0x82ab7b8
0x80866bd: PUSH 0x82a825c
0x80866c2: PUSH 0x4
0x80866c4: PUSH 0x2
0x80866c6: CALL 0x080b61c4
0x80866cb: ADD ESP, 0x14
0x80866ce: PUSH 0x0
0x80866d0: CALL 0x080a3980
0x80866d5: ADD ESP, 0x10
0x80866d8: JMP 0x080866d8
0x80866dc: CALL 0x080cb300
0x80866e1: XOR ESI, ESI
0x80866e3: TEST EAX, EAX
0x80866e5: JNZ 0x08086753
0x80866e7: JMP 0x0808692d
0x80866ec: SUB ESP, 0x8
0x80866ef: PUSH 0x20
0x80866f1: PUSH EAX
0x80866f2: CALL 0x08089dc8
0x80866f7: MOV EDX, EAX
0x80866f9: MOV EDI, dword ptr [0x0847aad0]
0x80866ff: AND EDX, 0x3ff
0x8086705: MOV EAX, dword ptr [EDI + 0x54]
0x8086708: MOV ECX, dword ptr [EAX + EDX*0x4]
0x808670b: ADD ESP, 0x10
0x808670e: TEST ECX, ECX
0x8086710: LEA EBX, [ESI + 0x83cb8c0]
0x8086716: MOV dword ptr [EBP + -0x10], EDX
0x8086719: JZ 0x080867c1
0x808671f: MOV EAX, dword ptr [ECX + 0xc]
0x8086722: MOV dword ptr [ESI + 0x83cb8cc], EAX
0x8086728: MOV EDX, dword ptr [ECX + 0xc]
0x808672b: LEA EAX, [EBX + 0xc]
0x808672e: MOV dword ptr [EDX + 0x10], EAX
0x8086731: MOV dword ptr [ECX + 0xc], EBX
0x8086734: LEA EAX, [ECX + 0xc]
0x8086737: MOV dword ptr [ESI + 0x83cb8d0], EAX
0x808673d: MOV EAX, dword ptr [EDI + 0x54]
0x8086740: MOV EDX, dword ptr [EBP + -0x10]
0x8086743: OR dword ptr [ESI + 0x83cb8c4], 0x80000000
0x808674d: MOV dword ptr [EAX + EDX*0x4], EBX
0x8086750: ADD ESI, 0x14
0x8086753: MOV EAX, dword ptr [ESI + 0x83cb8c0]
0x8086759: TEST EAX, EAX
0x808675b: JNZ 0x080866ec
0x808675d: CALL 0x08080448
0x8086762: XOR EBX, EBX
0x8086764: CALL 0x080d41fc
0x8086769: CALL 0x080fb104
0x808676e: CALL 0x080fb778
0x8086773: CALL 0x080f960c
0x8086778: CALL 0x080fb988
0x808677d: CALL 0x080fd238
0x8086782: CALL 0x080df414
0x8086787: CALL 0x080c2e94
0x808678c: CALL 0x080e2578
0x8086791: CALL 0x080e29f0
0x8086796: CALL 0x080ec7bc
0x808679b: CALL 0x080e8c54
0x80867a0: CALL 0x080f4134
0x80867a5: CALL 0x080ebcb4
0x80867aa: CALL 0x080f3474
0x80867af: CMP EBX, dword ptr [0x0847ace4]
0x80867b5: JGE 0x080867d8
0x80867b7: CALL dword ptr [EBX*0x8 + 0x847f9c0]
0x80867be: INC EBX
0x80867bf: JMP 0x080867af
0x80867c1: MOV EAX, dword ptr [EDI + 0x58]
0x80867c4: MOV EDX, dword ptr [EBP + -0x10]
0x80867c7: MOV dword ptr [EAX + EDX*0x4], EBX
0x80867ca: LEA EAX, [EBX + 0xc]
0x80867cd: MOV dword ptr [ESI + 0x83cb8cc], EBX
0x80867d3: JMP 0x08086737
0x80867d8: MOV EBX, dword ptr [0x0847aad0]
0x80867de: MOV ESI, dword ptr [EBX + 0x2c]
0x80867e1: TEST ESI, ESI
0x80867e3: MOV dword ptr [0x0847aacc], 0x0
0x80867ed: MOV dword ptr [0x0847aac8], 0x0
0x80867f7: MOV dword ptr [0x0847aac4], 0x0
0x8086801: JNZ 0x08086903
0x8086807: PUSH 0x1549
0x808680c: PUSH 0x82ab14c
0x8086811: PUSH 0x82a376b
0x8086816: MOVSX EAX, word ptr [EBX + 0x10]
0x808681a: SHL EAX, 0x3
0x808681d: PUSH EAX
0x808681e: CALL 0x080ce784
0x8086823: ADD ESP, 0x10
0x8086826: MOV dword ptr [EBX + 0x2c], EAX
0x8086829: MOV EDI, dword ptr [0x0847aad0]
0x808682f: MOV ECX, dword ptr [EDI + 0x2c]
0x8086832: TEST ECX, ECX
0x8086834: JZ 0x080868d4
0x808683a: MOVSX EDX, word ptr [0x0847ace0]
0x8086841: JMP 0x08086864
0x8086844: MOV EAX, dword ptr [EDI + 0x2c]
0x8086847: MOV byte ptr [EAX + EDX*0x8 + 0x6], 0x0
0x808684c: MOV EDI, dword ptr [0x0847aad0]
0x8086852: MOV dword ptr [EAX + EDX*0x8], 0x0
0x8086859: MOV EAX, dword ptr [EDI + 0x2c]
0x808685c: MOV word ptr [EAX + EDX*0x8 + 0x4], 0x0
0x8086863: INC EDX
0x8086864: MOVSX EAX, word ptr [EDI + 0x10]
0x8086868: CMP EAX, EDX
0x808686a: JG 0x08086844
0x808686c: MOV AX, word ptr [EDI + 0x10]
0x8086870: MOV [0x0847ace0], AX
0x8086876: MOV dword ptr [EBP + -0x14], 0x0
0x808687d: MOV EAX, dword ptr [EDI + 0x58]
0x8086880: MOV EDX, dword ptr [EBP + -0x14]
0x8086883: MOV EBX, dword ptr [EAX + EDX*0x4]
0x8086886: TEST EBX, EBX
0x8086888: JMP 0x080868a5
0x808688c: MOV EAX, dword ptr [EDI + 0x58]
0x808688f: MOV EDX, dword ptr [EBP + -0x14]
0x8086892: CMP ESI, dword ptr [EAX + EDX*0x4]
0x8086895: MOV EBX, ESI
0x8086897: JZ 0x0808695c
0x808689d: MOV EDX, dword ptr [EBP + -0x14]
0x80868a0: MOV EDX, dword ptr [EAX + EDX*0x4]
0x80868a3: TEST EDX, EDX
0x80868a5: JZ 0x0808695c
0x80868ab: SUB ESP, 0x8
0x80868ae: MOV ESI, dword ptr [EBX + 0xc]
0x80868b1: PUSH 0x0
0x80868b3: PUSH EBX
0x80868b4: CALL 0x0808761c
0x80868b9: ADD ESP, 0x10
0x80868bc: TEST EAX, EAX
0x80868be: JNZ 0x0808688c
0x80868c0: SUB ESP, 0x8
0x80868c3: PUSH EBX
0x80868c4: PUSH EDI
0x80868c5: CALL 0x080856d8
0x80868ca: MOV EBX, ESI
0x80868cc: ADD ESP, 0x10
0x80868cf: MOV EAX, dword ptr [EDI + 0x58]
0x80868d2: JMP 0x0808689d
0x80868d4: SUB ESP, 0x8
0x80868d7: PUSH 0x1553
0x80868dc: PUSH 0x82ab14c
0x80868e1: PUSH 0x82ab78c
0x80868e6: PUSH 0x82a825c
0x80868eb: PUSH 0x4
0x80868ed: PUSH 0x2
0x80868ef: CALL 0x080b61c4
0x80868f4: ADD ESP, 0x14
0x80868f7: PUSH 0x0
0x80868f9: CALL 0x080a3980
0x80868fe: ADD ESP, 0x10
0x8086901: JMP 0x08086901
0x8086903: SUB ESP, 0xc
0x8086906: PUSH 0x154f
0x808690b: PUSH 0x82ab14c
0x8086910: PUSH 0x82a376b
0x8086915: MOVSX EAX, word ptr [EBX + 0x10]
0x8086919: SHL EAX, 0x3
0x808691c: PUSH EAX
0x808691d: PUSH dword ptr [EBX + 0x2c]
0x8086920: CALL 0x080cf588
0x8086925: ADD ESP, 0x20
0x8086928: JMP 0x08086826
0x808692d: SUB ESP, 0x8
0x8086930: PUSH 0x1492
0x8086935: PUSH 0x82ab14c
0x808693a: PUSH 0x82ab09d
0x808693f: PUSH 0x82a825c
0x8086944: PUSH 0x4
0x8086946: PUSH 0x2
0x8086948: CALL 0x080b61c4
0x808694d: ADD ESP, 0x14
0x8086950: PUSH 0x0
0x8086952: CALL 0x080a3980
0x8086957: ADD ESP, 0x10
0x808695a: JMP 0x0808695a
0x808695c: INC dword ptr [EBP + -0x14]
0x808695f: CMP dword ptr [EBP + -0x14], 0x3ff
0x8086966: JLE 0x08086880
0x808696c: LEA ESP, [EBP + -0xc]
0x808696f: POP EBX
0x8086970: POP ESI
0x8086971: MOV EAX, 0x1
0x8086976: POP EDI
0x8086977: LEAVE
0x8086978: RET
```

限制：
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

## vm_loadIdentifiers / analyze_function

Evidence: `ev_b786ac699461a76c0683ddb2f8254575187db56f002088762956ac65e57e9528`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808800e", "start": "0x8087fb8"}, {"end": "0x8088074", "start": "0x8088010"}, {"end": "0x80883a0", "start": "0x8088078"}, {"end": "0x8088873", "start": "0x80883a4"}], "span_bytes": 2236, "total_bytes": 2229}`

```c

int vm_loadIdentifiers(Vm_Binary_Buffer_T *pInput)

{
  short *psVar1;
  uint uVar2;
  Vm_Ident_Hash_Elem **ppVVar3;
  bool bVar4;
  Vm_Prog_Var_T **ppVVar5;
  short sVar6;
  ushort uVar7;
  short sVar8;
  int iVar9;
  Vm_Prog_Var_T **ppVVar10;
  Vm_Fun_Entry_T *pVVar11;
  Vm_Program_T *pVVar12;
  Vm_Ident_Hash_Elem *pVVar13;
  Vm_Ident_Info *pVVar14;
  uint uVar15;
  undefined4 uVar16;
  Vm_Prog_Var_T **extraout_ECX;
  Vm_Prog_Var_T **ppVVar17;
  Vm_Binary_Buffer_T *extraout_ECX_00;
  Vm_Binary_Buffer_T *extraout_ECX_01;
  Vm_Binary_Buffer_T *extraout_ECX_02;
  Vm_Binary_Buffer_T *extraout_ECX_03;
  int iVar18;
  Vm_Ident_Hash_Elem *pVVar19;
  char *pcVar20;
  char *str_00;
  char *pcVar21;
  Vm_Prog_Var_T *pVVar22;
  Vm_Fun_T **num;
  Vm_Binary_Buffer_T *pVVar23;
  Vm_Binary_Buffer_T *pVVar24;
  int h;
  Vm_Prog_Var_T **ppVar;
  int i;
  Vm_Fun_T *pFun;
  Vm_Fun_T *pHiddenFun;
  int count_1;
  Vm_Index_T functions;
  Vm_Index_T efunCounts;
  Vm_Prog_Var_T *pVar;
  Vm_Index_T variables;
  Vm_Index_T maxVariableNo;
  char *pNames;
  int nameSize;
  int count;
  char str [256];
  Vm_Dlt_T dlt;
  
  pcVar21 = "IDENTIFIERS";
  iVar9 = vm_loadComment(pInput,"IDENTIFIERS");
  if (iVar9 == 0) {
    pcVar20 = "Load identifiers failed: comment \"IDENTIFIERS\" mismatched.\n";
    goto LAB_08087fe1;
  }
  pcVar21 = "NAMES";
  iVar9 = vm_loadComment(pInput,"NAMES");
  if (iVar9 == 0) {
    pcVar20 = "Load identifiers failed: comment \"NAME\" mismatched.\n";
    goto LAB_08087fe1;
  }
  num = (Vm_Fun_T **)&count;
  pcVar21 = (char *)num;
  iVar9 = vm_loadNumber32(pInput,(int *)num);
  if ((iVar9 == 0) || (count < 0)) {
    pcVar20 = "Load identifiers failed: count error.\n";
    goto LAB_08087fe1;
  }
  pcVar21 = (char *)&nameSize;
  iVar9 = vm_loadNumber32(pInput,(int *)pcVar21);
  if ((iVar9 != 0) && (-1 < nameSize)) {
    pcVar21 = (char *)&pNames;
    iVar9 = vm_loadTempBuffer(pInput,pcVar21,nameSize);
    if (iVar9 != 0) {
                    /* Unresolved local var: Vm_Program_T * pProgram@[???] */
      pVVar12 = vm_pGlobalProgram;
      if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
        pVVar12 = vm_pCurrentProgram;
      }
      pcVar21 = "VARIABLES";
      iVar9 = vm_loadComment(pInput,"VARIABLES");
      if (iVar9 == 0) {
        pcVar20 = "Load variables failed: comment \"VARIABLES\" mismatched.\n";
        goto LAB_080880b2;
      }
      pcVar21 = (char *)&maxVariableNo;
      iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
      if ((iVar9 == 0) || ((int)_variables < 0)) {
LAB_08088203:
        pcVar20 = "Load variables failed: read variable error.\n";
      }
      else {
        pcVar21 = (char *)&variables;
        iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
        if ((iVar9 == 0) || (variables < 0)) goto LAB_08088203;
        if (pVVar12 == vm_pGlobalProgram) {
          iVar9 = (int)maxVariableNo;
          if (vm_globalVar <= maxVariableNo) goto LAB_080881c1;
          pcVar20 = "Load variables failed: bad variable count.\n";
        }
        else {
          pVVar12->maxVariableNo = maxVariableNo;
          pcVar21 = "PROG_VARS";
          iVar9 = vm_loadComment(pInput,"PROG_VARS");
          if (iVar9 != 0) {
            ppVVar5 = &pVVar12->pProgVars;
            ppVVar17 = extraout_ECX;
            while( true ) {
              ppVVar10 = ppVVar5;
              sVar6 = (short)_variables;
              _variables = (Vm_Fun_T *)CONCAT22(maxVariableNo,sVar6 + -1);
              iVar9 = vm_globalVar;
              if (sVar6 < 1) break;
              uVar16 = 0x10;
              pcVar21 = (char *)&pVar;
              iVar9 = vm_loadBuffer(pInput,pcVar21,0x10);
              if (iVar9 == 0) {
                pcVar20 = "Load variables failed: read program\'s variable error.\n";
                goto LAB_080880b2;
              }
              pVVar22 = pVar;
              iVar9 = vm_loadString(pInput,&pVar->pVarName);
              if (iVar9 == 0) {
                vm_errprintf("Load variables failed: read program\'s variable name error.\n",pVVar22
                             ,uVar16,ppVVar17);
                pcVar21 = &DAT_082a376b;
                pdb_freeMemory(pVar,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1bb9);
                goto LAB_080880b7;
              }
              pcVar21 = pVVar12->strName->ptr;
              pVar->pNext = (Vm_Prog_Var *)0x0;
              *ppVVar10 = pVar;
              pVar->pProgName = pcVar21;
              ppVVar5 = &pVar->pNext;
              ppVVar17 = ppVVar10;
            }
LAB_080881c1:
                    /* Unresolved local var: Vm_Program_T * pProgram@[???] */
            vm_globalVar = iVar9;
            pVVar12 = vm_pCurrentProgram;
            if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
              pVVar12 = vm_pGlobalProgram;
            }
            pcVar21 = "FUNCTIONS";
            iVar9 = vm_loadComment(pInput,"FUNCTIONS");
            if (iVar9 == 0) {
              pcVar20 = "Load functions failed: comment \"FUNCTIONS\" mismatched.\n";
            }
            else {
              pcVar21 = (char *)&efunCounts;
              iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
              if ((iVar9 == 0) || (vm_efunsCount != efunCounts)) {
                pcVar20 = "Load functions failed: read efuns error.\n";
              }
              else {
                pcVar21 = (char *)&functions;
                iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
                if ((iVar9 == 0) || (sVar6 = (short)_functions, sVar6 < 0)) {
                  pcVar20 = "Load functions failed: read functions error.\n";
                }
                else {
                  pVVar12->maxFunctionNo = sVar6;
                    /* Unresolved local var: Vm_Fun_Entry_T * pFunctions@[DW_OP_reg0(EAX)] */
                  if (pVVar12->pFunctions == (Vm_Fun_Entry_T *)0x0) {
                    pcVar21 = &DAT_082a376b;
                    pVVar11 = pdb_allocateMemory((int)sVar6 << 3,"VM",
                                                 "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19f3);
                  }
                  else {
                    pVVar11 = pdb_reallocateMemory
                                        (pVVar12->pFunctions,(int)sVar6 << 3,"VM",
                                         "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19f7);
                  }
                  if (pVVar11 == (Vm_Fun_Entry_T *)0x0) {
                    pcVar20 = "Load identifiers failed: extend function table error.\n";
                  }
                  else {
                    iVar18 = (int)vm_functionBC;
                    pVVar12->pFunctions = pVVar11;
                    pVVar11 = pVVar11 + iVar18;
                    for (iVar9 = (pVVar12->maxFunctionNo - iVar18 & 0x1fffffffU) << 1; iVar9 != 0;
                        iVar9 = iVar9 + -1) {
                      pVVar11->pFun = (Vm_Fun *)0x0;
                      pVVar11 = (Vm_Fun_Entry_T *)&pVVar11->consititutionNo;
                    }
                    pcVar21 = "HIDDEN_FUNS";
                    iVar9 = vm_loadComment(pInput,"HIDDEN_FUNS");
                    if (iVar9 == 0) {
                      pcVar20 = "Load program failed: comment \"HIDDEN_FUNS\" mismatched.\n";
                    }
                    else {
                      pcVar21 = (char *)&count_1;
                      iVar9 = vm_loadNumber32(pInput,(int *)pcVar21);
                      if ((iVar9 != 0) && (-1 < count_1)) {
                        iVar9 = count_1 + -1;
                        bVar4 = 0 < count_1;
                        count_1 = iVar9;
                        if (bVar4) {
                          do {
                            pcVar21 = (char *)&pHiddenFun;
                            iVar9 = vm_loadFun(pInput,&pHiddenFun);
                            if (iVar9 == 0) {
                              pcVar20 = "Load program failed: load hidden function error.\n";
                              goto LAB_080881f2;
                            }
                            pHiddenFun->pNext = pVVar12->pHiddenFuns;
                            pVVar12->pHiddenFuns = pHiddenFun;
                            iVar9 = count_1 + -1;
                            bVar4 = 0 < count_1;
                            count_1 = iVar9;
                          } while (bVar4);
                        }
                        iVar9 = vm_loadInheritPrograms(pInput);
                        if (iVar9 == 0) {
                          pcVar20 = "Load identifiers failed: load program error.\n";
                        }
                        else {
                          pcVar21 = &DAT_082ab10b;
                          iVar9 = vm_loadComment(pInput,"DLT");
                          if (iVar9 == 0) {
                            pcVar20 = "Load identifiers failed: DLT error.\n";
                          }
                          else {
                            iVar9 = vm_loadNumber32(pInput,(int *)num);
                            i = count;
                            if (iVar9 == 0) {
                              pcVar20 = "Load identifiers failed: DLT count error.\n";
                              pcVar21 = (char *)num;
                            }
                            else {
                              while( true ) {
                                iVar9 = nameSize;
                                pcVar21 = pNames;
                                i = i + -1;
                                if (i < 0) {
                                  vm_scratchFree(pNames);
                                  return 1;
                                }
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[???]
                       Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Dlt_T * pDlt@[???] */
                                pVVar24 = (Vm_Binary_Buffer_T *)vm_pGlobalProgram;
                                if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
                                  pVVar24 = (Vm_Binary_Buffer_T *)vm_pCurrentProgram;
                                }
                                pcVar20 = (char *)0x14;
                                uVar16 = &dlt;
                                pVVar23 = (Vm_Binary_Buffer_T *)vm_pCurrentProgram;
                                iVar18 = vm_loadBufferAt(pInput,(void *)uVar16,0x14);
                                if (iVar18 == 0) break;
                                if ((uint)iVar9 <= dlt.name) {
                                  pcVar21 = "Load dlt entry failed: name error.\n";
                                  goto LAB_0808862e;
                                }
                                dlt.name = (uint)(pcVar21 + dlt.name);
                                if ((dlt.attrib & 0x80000) == 0) {
                                  if ((dlt.attrib & 0x40000) != 0) {
                                    if (dlt.v.value.no < 0) {
                                      sVar6 = *(Vm_Index_T *)((int)pVVar24->data + 2);
                                    }
                                    else {
                                      sVar6 = *(Vm_Index_T *)((int)pVVar24->data + 2);
                                      if (dlt.v.value.no < sVar6) goto LAB_0808850b;
                                    }
                                    pcVar20 = (char *)(int)sVar6;
                                    pcVar21 = "Bad offset (%d) of an object variable (max %d).\n";
                                    uVar16 = dlt.v.pInfo;
                                    pVVar23 = pInput;
                                    goto LAB_0808862e;
                                  }
                                  if ((dlt.attrib & 0x8000000) == 0) {
                                    pcVar21 = "Load dlt failed: Bad type of DLT entry.\n";
                                    goto LAB_0808862e;
                                  }
                                  sVar6 = (short)dlt.v._4_4_;
                                  if (sVar6 < 0) {
                                    sVar8 = *(Vm_Index_T *)((int)pVVar24->data + 4);
LAB_08088856:
                                    pcVar20 = (char *)(int)sVar8;
                                    uVar16 = SEXT24(sVar6);
                                    pcVar21 = "Bad offset (%d) of a function (max %d).\n";
                                    pVVar23 = extraout_ECX_00;
                                    goto LAB_0808862e;
                                  }
                                  sVar8 = *(Vm_Index_T *)((int)pVVar24->data + 4);
                                  if (sVar8 <= sVar6) goto LAB_08088856;
                                  iVar9 = vm_loadFun(pInput,&pFun);
                                  if (iVar9 != 0) {
                                    dlt.v.uf.pFun = pFun;
                                    goto LAB_0808850b;
                                  }
                                  goto LAB_08088636;
                                }
                                if ((dlt.v.value.no < 0) || (vm_globalVar <= dlt.v.value.no)) {
                                  pcVar21 = "Bad offset (%d) of a global variable (max %d).\n";
                                  uVar16 = dlt.v.pInfo;
                                  pcVar20 = (char *)vm_globalVar;
                                  pVVar23 = pVVar24;
                                  goto LAB_0808862e;
                                }
LAB_0808850b:
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[???] */
                                uVar2 = dlt.name;
                                pVVar12 = vm_pGlobalProgram;
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * hptr@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * hptr2@[DW_OP_reg7(EDI)] */
                                if (vm_pGlobalProgram->ppHashTable == (Vm_Ident_Hash_Elem **)0x0) {
LAB_08088525:
                                  pVVar12 = vm_pCurrentProgram;
                                  if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
                                    pVVar12 = vm_pGlobalProgram;
                                  }
                                  pVVar13 = vm_findOrAddIdentInProgram(pVVar12,(char *)uVar2);
                                  pVVar24 = extraout_ECX_01;
                                }
                                else {
                                  uVar7 = vm_whashstr((char *)dlt.name,0x20);
                                  ppVVar3 = pVVar12->ppHashTable;
                                  pVVar13 = ppVVar3[uVar7 & 0x3ff];
                                  if (pVVar13 == (Vm_Ident_Hash_Elem *)0x0) goto LAB_08088525;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
                                  iVar9 = strcmp(pVVar13->name,(char *)uVar2);
                                  if (iVar9 != 0) {
                                    for (pVVar19 = pVVar13->pNext; pVVar19 != pVVar13;
                                        pVVar19 = pVVar19->pNext) {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
                                      iVar9 = strcmp(pVVar19->name,(char *)uVar2);
                                      if (iVar9 == 0) {
                                        if (((int)pVVar19->token < 0) || (pVVar19->semValue != 0)) {
                                          ppVVar3[uVar7 & 0x3ff] = pVVar19;
                                          pVVar13 = pVVar19;
                                          pVVar24 = extraout_ECX_03;
                                          goto LAB_08088714;
                                        }
                                        break;
                                      }
                                    }
                                    goto LAB_08088525;
                                  }
                                  pVVar24 = extraout_ECX_02;
                                  if ((-1 < (int)pVVar13->token) && (pVVar13->semValue == 0))
                                  goto LAB_08088525;
LAB_08088714:
                                  if ((pVVar13 == (Vm_Ident_Hash_Elem *)0x0) ||
                                     (-1 < (int)pVVar13->token)) goto LAB_08088525;
                                }
                                uVar2 = pVVar13->token;
                                if ((uVar2 & 0xc0080000) != 0) {
                                  pcVar21 = "Load dlt failed: \"%s\" is already declared.\n";
                                  uVar16 = (Vm_Dlt_T *)pVVar13->name;
                                  goto LAB_0808862e;
                                }
                                if (((uVar2 & 0x8000000) != 0) && ((dlt.attrib & 0x8000000) == 0)) {
                                  pcVar21 = 
                                  "Load dlt failed: \"%s\" is already declared as function.\n";
                                  uVar16 = (Vm_Dlt_T *)pVVar13->name;
                                  goto LAB_0808862e;
                                }
                                uVar15 = 0;
                                if (pVVar13->semValue != 0) {
                    /* Unresolved local var: Vm_Ident_Info_T * pIi@[DW_OP_reg2(EDX)] */
                                  if ((int)uVar2 < 0) {
                                    pcVar21 = "The reserved word can not be redefined.";
LAB_08088689:
                                    yyerror(pcVar21);
                                  }
                                  else {
                                    if ((uVar2 & 0x40000000) != 0) {
                                      pcVar21 = "The efun can not be redefined.";
                                      goto LAB_08088689;
                                    }
                                    if (((uVar2 & 0x2ffff) == 0) ||
                                       ((pVVar13->ii).pInContext != (void *)0x0)) {
                                      pVVar24 = (Vm_Binary_Buffer_T *)0x1324;
                                      pcVar20 = "/home/cc/xxxx/gameshell/src/vm/vm_lex.c";
                                      pcVar21 = "VM";
                                      pVVar14 = pdb_allocateMemory(0x18,"VM",
                                                                                                                                      
                                                  "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1324);
                                      if (pVVar14 != (Vm_Ident_Info *)0x0) {
                                        (pVVar14->v).pInfo = (pVVar13->ii).v.pInfo;
                                        *(undefined4 *)((int)&pVVar14->v + 4) =
                                             *(undefined4 *)((int)&(pVVar13->ii).v + 4);
                                        pVVar14->pInContext = (pVVar13->ii).pInContext;
                                        pVVar14->oldToken = (pVVar13->ii).oldToken;
                                        uVar7 = (pVVar13->ii).unused;
                                        pVVar14->refered = (pVVar13->ii).refered;
                                        pVVar14->unused = uVar7;
                                        pVVar14->pNext = (pVVar13->ii).pNext;
                                        pVVar14->oldToken = pVVar13->token;
                                        (pVVar13->ii).pNext = pVVar14;
                                        uVar15 = pVVar13->semValue;
                                        goto LAB_080885e4;
                                      }
                                      str_00 = "No enough memort to backup information structure.\n"
                                      ;
                                    }
                                    else {
                                      pcVar21 = pVVar13->name;
                                      str_00 = "The name \"%s\" in line %d has been defined.\n";
                                      pcVar20 = (char *)currentLine;
                                    }
                                    yyerrors(str_00,pcVar21,pcVar20,pVVar24);
                                    pVVar23 = pVVar24;
                                  }
                                  pcVar21 = "Load dlt failed: Failed to backup II for \"%s\".\n";
                                  uVar16 = (Vm_Dlt_T *)pVVar13->name;
                                  goto LAB_0808862e;
                                }
LAB_080885e4:
                                pVVar13->semValue = uVar15 + 1;
                                (pVVar13->ii).pInContext = (void *)0x0;
                                *(undefined4 *)((int)&(pVVar13->ii).v + 4) = dlt.v._4_4_;
                                (pVVar13->ii).v.pInfo = dlt.v.pInfo;
                                psVar1 = &(pVVar13->dn).globalNum;
                                *psVar1 = *psVar1 + 1;
                                (pVVar13->ii).oldToken = dlt.attrib | 0x20000000;
                                pVVar13->token = dlt.attrib | 0x20000000;
                              }
                              pcVar21 = "Load dlt entry failed: read DLT entry error.\n";
LAB_0808862e:
                              vm_errprintf(pcVar21,uVar16,pcVar20,pVVar23);
LAB_08088636:
                              if (0xff < (uint)nameSize) {
                                nameSize = 0xff;
                              }
                              memcpy(str,pNames,nameSize);
                              pcVar20 = "Load identifier failed: %s.\n";
                              str[nameSize] = '\0';
                              pcVar21 = str;
                            }
                          }
                        }
                        goto LAB_08087fe1;
                      }
                      pcVar20 = "Load program failed: read count of hidden functions error.\n";
                    }
                  }
                }
              }
            }
LAB_080881f2:
            vm_errprintf(pcVar20);
            pcVar20 = "Load identifiers failed: load functions error.\n";
            goto LAB_08087fe1;
          }
          pcVar20 = "Load variables failed: comment \"PROG_VARS\" mismatched.\n";
        }
      }
LAB_080880b2:
      vm_errprintf(pcVar20);
LAB_080880b7:
      pcVar20 = "Load identifiers failed: load variables error.\n";
      goto LAB_08087fe1;
    }
  }
  pcVar20 = "Load identifiers failed: names error.\n";
LAB_08087fe1:
  vm_errprintf(pcVar20,pcVar21);
  return 0;
}


```

```text
0x8087fb8: PUSH EBP
0x8087fb9: MOV EBP, ESP
0x8087fbb: PUSH EDI
0x8087fbc: PUSH ESI
0x8087fbd: PUSH EBX
0x8087fbe: SUB ESP, 0x174
0x8087fc4: PUSH 0x82ab0cf
0x8087fc9: MOV EBX, dword ptr [EBP + 0x8]
0x8087fcc: PUSH EBX
0x8087fcd: CALL 0x080b8800
0x8087fd2: ADD ESP, 0x10
0x8087fd5: TEST EAX, EAX
0x8087fd7: JNZ 0x08087ff0
0x8087fd9: SUB ESP, 0xc
0x8087fdc: PUSH 0x82aba74
0x8087fe1: CALL 0x08097acc
0x8087fe6: XOR EAX, EAX
0x8087fe8: LEA ESP, [EBP + -0xc]
0x8087feb: POP EBX
0x8087fec: POP ESI
0x8087fed: POP EDI
0x8087fee: LEAVE
0x8087fef: RET
0x8087ff0: SUB ESP, 0x8
0x8087ff3: PUSH 0x82ab0db
0x8087ff8: PUSH EBX
0x8087ff9: CALL 0x080b8800
0x8087ffe: ADD ESP, 0x10
0x8088001: TEST EAX, EAX
0x8088003: JNZ 0x08088010
0x8088005: SUB ESP, 0xc
0x8088008: PUSH 0x82abab0
0x808800d: JMP 0x08087fe1
0x8088010: SUB ESP, 0x8
0x8088013: LEA ESI, [EBP + 0xfffffec4]
0x8088019: PUSH ESI
0x808801a: PUSH EBX
0x808801b: CALL 0x080b8bf4
0x8088020: ADD ESP, 0x10
0x8088023: TEST EAX, EAX
0x8088025: JZ 0x08088078
0x8088027: MOV EAX, dword ptr [EBP + 0xfffffec4]
0x808802d: TEST EAX, EAX
0x808802f: JS 0x08088078
0x8088031: SUB ESP, 0x8
0x8088034: LEA EAX, [EBP + 0xfffffec0]
0x808803a: PUSH EAX
0x808803b: PUSH EBX
0x808803c: CALL 0x080b8bf4
0x8088041: ADD ESP, 0x10
0x8088044: TEST EAX, EAX
0x8088046: JZ 0x08088068
0x8088048: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x808804e: TEST EAX, EAX
0x8088050: JS 0x08088068
0x8088052: PUSH EDI
0x8088053: PUSH EAX
0x8088054: LEA EAX, [EBP + 0xfffffebc]
0x808805a: PUSH EAX
0x808805b: PUSH EBX
0x808805c: CALL 0x080b899c
0x8088061: ADD ESP, 0x10
0x8088064: TEST EAX, EAX
0x8088066: JNZ 0x08088085
0x8088068: SUB ESP, 0xc
0x808806b: PUSH 0x82abae8
0x8088070: JMP 0x08087fe1
0x8088078: SUB ESP, 0xc
0x808807b: PUSH 0x82abb10
0x8088080: JMP 0x08087fe1
0x8088085: MOV EAX, [0x0847aac0]
0x808808a: TEST EAX, EAX
0x808808c: MOV EDI, dword ptr [0x0847aad0]
0x8088092: CMOVNZ EDI, EAX
0x8088095: SUB ESP, 0x8
0x8088098: PUSH 0x82ab0e1
0x808809d: PUSH EBX
0x808809e: CALL 0x080b8800
0x80880a3: ADD ESP, 0x10
0x80880a6: TEST EAX, EAX
0x80880a8: JNZ 0x080880c3
0x80880aa: SUB ESP, 0xc
0x80880ad: PUSH 0x82abb38
0x80880b2: CALL 0x08097acc
0x80880b7: MOV dword ptr [ESP], 0x82abb70
0x80880be: JMP 0x08087fe1
0x80880c3: SUB ESP, 0x8
0x80880c6: LEA EAX, [EBP + 0xfffffeba]
0x80880cc: PUSH EAX
0x80880cd: PUSH EBX
0x80880ce: CALL 0x080b8b44
0x80880d3: ADD ESP, 0x10
0x80880d6: TEST EAX, EAX
0x80880d8: JZ 0x08088203
0x80880de: CMP word ptr [EBP + 0xfffffeba], 0x0
0x80880e6: JS 0x08088203
0x80880ec: SUB ESP, 0x8
0x80880ef: LEA EAX, [EBP + 0xfffffeb8]
0x80880f5: PUSH EAX
0x80880f6: PUSH EBX
0x80880f7: CALL 0x080b8b44
0x80880fc: ADD ESP, 0x10
0x80880ff: TEST EAX, EAX
0x8088101: JZ 0x08088203
0x8088107: CMP word ptr [EBP + 0xfffffeb8], 0x0
0x808810f: JS 0x08088203
0x8088115: CMP EDI, dword ptr [0x0847aad0]
0x808811b: JZ 0x08088210
0x8088121: SUB ESP, 0x8
0x8088124: MOV AX, word ptr [EBP + 0xfffffeba]
0x808812b: MOV word ptr [EDI + 0xe], AX
0x808812f: PUSH 0x82ab0eb
0x8088134: PUSH EBX
0x8088135: CALL 0x080b8800
0x808813a: ADD ESP, 0x10
0x808813d: TEST EAX, EAX
0x808813f: JZ 0x08088257
0x8088145: LEA EAX, [EDI + 0x28]
0x8088148: JMP 0x080881a6
0x808814a: PUSH ECX
0x808814b: PUSH 0x10
0x808814d: LEA EDX, [EBP + 0xfffffeb4]
0x8088153: PUSH EDX
0x8088154: PUSH EBX
0x8088155: CALL 0x080b8a70
0x808815a: ADD ESP, 0x10
0x808815d: TEST EAX, EAX
0x808815f: JZ 0x080882a0
0x8088165: SUB ESP, 0x8
0x8088168: PUSH dword ptr [EBP + 0xfffffeb4]
0x808816e: PUSH EBX
0x808816f: CALL 0x080b8dc8
0x8088174: ADD ESP, 0x10
0x8088177: TEST EAX, EAX
0x8088179: JZ 0x08088271
0x808817f: MOV EDX, dword ptr [EBP + 0xfffffeb4]
0x8088185: MOV EAX, dword ptr [EDI + 0x4]
0x8088188: MOV ECX, dword ptr [EBP + 0xfffffe9c]
0x808818e: MOV EAX, dword ptr [EAX + 0xc]
0x8088191: MOV dword ptr [EDX + 0xc], 0x0
0x8088198: MOV dword ptr [ECX], EDX
0x808819a: MOV dword ptr [EDX + 0x4], EAX
0x808819d: MOV EAX, dword ptr [EBP + 0xfffffeb4]
0x80881a3: ADD EAX, 0xc
0x80881a6: MOV EDX, dword ptr [EBP + 0xfffffeb8]
0x80881ac: MOV dword ptr [EBP + 0xfffffe9c], EAX
0x80881b2: TEST DX, DX
0x80881b5: LEA EAX, [EDX + -0x1]
0x80881b8: MOV word ptr [EBP + 0xfffffeb8], AX
0x80881bf: JG 0x0808814a
0x80881c1: MOV EAX, [0x0847aac0]
0x80881c6: TEST EAX, EAX
0x80881c8: CMOVZ EAX, dword ptr [0x0847aad0]
0x80881cf: SUB ESP, 0x8
0x80881d2: PUSH 0x82ab0f5
0x80881d7: PUSH EBX
0x80881d8: MOV dword ptr [EBP + 0xfffffe98], EAX
0x80881de: CALL 0x080b8800
0x80881e3: ADD ESP, 0x10
0x80881e6: TEST EAX, EAX
0x80881e8: JNZ 0x08088226
0x80881ea: SUB ESP, 0xc
0x80881ed: PUSH 0x82abba0
0x80881f2: CALL 0x08097acc
0x80881f7: MOV dword ptr [ESP], 0x82abbd8
0x80881fe: JMP 0x08087fe1
0x8088203: SUB ESP, 0xc
0x8088206: PUSH 0x82abc08
0x808820b: JMP 0x080880b2
0x8088210: MOVSX EAX, word ptr [EBP + 0xfffffeba]
0x8088217: CMP EAX, dword ptr [0x0847aac4]
0x808821d: JL 0x08088264
0x808821f: MOV [0x0847aac4], EAX
0x8088224: JMP 0x080881c1
0x8088226: SUB ESP, 0x8
0x8088229: LEA EAX, [EBP + 0xfffffeb2]
0x808822f: PUSH EAX
0x8088230: PUSH EBX
0x8088231: CALL 0x080b8b44
0x8088236: ADD ESP, 0x10
0x8088239: TEST EAX, EAX
0x808823b: JZ 0x0808824d
0x808823d: MOV AX, word ptr [EBP + 0xfffffeb2]
0x8088244: CMP word ptr [0x0847ace0], AX
0x808824b: JZ 0x080882ad
0x808824d: SUB ESP, 0xc
0x8088250: PUSH 0x82abc38
0x8088255: JMP 0x080881f2
0x8088257: SUB ESP, 0xc
0x808825a: PUSH 0x82abc64
0x808825f: JMP 0x080880b2
0x8088264: SUB ESP, 0xc
0x8088267: PUSH 0x82abc9c
0x808826c: JMP 0x080880b2
0x8088271: SUB ESP, 0xc
0x8088274: PUSH 0x82abcc8
0x8088279: CALL 0x08097acc
0x808827e: ADD ESP, 0x10
0x8088281: PUSH 0x1bb9
0x8088286: PUSH 0x82ab14c
0x808828b: PUSH 0x82a376b
0x8088290: PUSH dword ptr [EBP + 0xfffffeb4]
0x8088296: CALL 0x080ceb94
0x808829b: JMP 0x080880b7
0x80882a0: SUB ESP, 0xc
0x80882a3: PUSH 0x82abd04
0x80882a8: JMP 0x080880b2
0x80882ad: SUB ESP, 0x8
0x80882b0: LEA EAX, [EBP + 0xfffffeb0]
0x80882b6: PUSH EAX
0x80882b7: PUSH EBX
0x80882b8: CALL 0x080b8b44
0x80882bd: ADD ESP, 0x10
0x80882c0: TEST EAX, EAX
0x80882c2: JZ 0x08088352
0x80882c8: MOV EAX, dword ptr [EBP + 0xfffffeb0]
0x80882ce: TEST AX, AX
0x80882d1: JS 0x08088352
0x80882d3: MOV EDX, dword ptr [EBP + 0xfffffe98]
0x80882d9: MOV word ptr [EDX + 0x10], AX
0x80882dd: MOV EDX, dword ptr [EDX + 0x2c]
0x80882e0: TEST EDX, EDX
0x80882e2: JNZ 0x080883f7
0x80882e8: PUSH 0x19f3
0x80882ed: PUSH 0x82ab14c
0x80882f2: CWDE
0x80882f3: PUSH 0x82a376b
0x80882f8: SHL EAX, 0x3
0x80882fb: PUSH EAX
0x80882fc: CALL 0x080ce784
0x8088301: ADD ESP, 0x10
0x8088304: TEST EAX, EAX
0x8088306: JZ 0x080883ea
0x808830c: MOV EDX, dword ptr [EBP + 0xfffffe98]
0x8088312: MOVSX ECX, word ptr [EDX + 0x10]
0x8088316: MOVSX EDI, word ptr [0x08660900]
0x808831d: SUB ECX, EDI
0x808831f: SHL ECX, 0x3
0x8088322: MOV dword ptr [EDX + 0x2c], EAX
0x8088325: SUB ESP, 0x8
0x8088328: LEA EDI, [EAX + EDI*0x8]
0x808832b: CLD
0x808832c: XOR EAX, EAX
0x808832e: SHR ECX, 0x2
0x8088331: STOSD.REP ES:EDI
0x8088333: PUSH 0x82ab0ff
0x8088338: PUSH EBX
0x8088339: CALL 0x080b8800
0x808833e: ADD ESP, 0x10
0x8088341: TEST EAX, EAX
0x8088343: JNZ 0x0808835f
0x8088345: SUB ESP, 0xc
0x8088348: PUSH 0x82abd3c
0x808834d: JMP 0x080881f2
0x8088352: SUB ESP, 0xc
0x8088355: PUSH 0x82abd74
0x808835a: JMP 0x080881f2
0x808835f: SUB ESP, 0x8
0x8088362: LEA EAX, [EBP + 0xfffffeac]
0x8088368: PUSH EAX
0x8088369: PUSH EBX
0x808836a: CALL 0x080b8bf4
0x808836f: ADD ESP, 0x10
0x8088372: TEST EAX, EAX
0x8088374: JZ 0x08088441
0x808837a: MOV EDX, dword ptr [EBP + 0xfffffeac]
0x8088380: TEST EDX, EDX
0x8088382: JS 0x08088441
0x8088388: LEA EAX, [EDX + -0x1]
0x808838b: TEST EDX, EDX
0x808838d: MOV dword ptr [EBP + 0xfffffeac], EAX
0x8088393: JLE 0x08088424
0x8088399: LEA EDI, [EBP + 0xfffffea8]
0x808839f: JMP 0x080883cc
0x80883a4: MOV ECX, dword ptr [EBP + 0xfffffe98]
0x80883aa: MOV EDX, dword ptr [EBP + 0xfffffea8]
0x80883b0: MOV EAX, dword ptr [ECX + 0x30]
0x80883b3: MOV dword ptr [EDX + 0x3c], EAX
0x80883b6: MOV dword ptr [ECX + 0x30], EDX
0x80883b9: MOV EDX, dword ptr [EBP + 0xfffffeac]
0x80883bf: LEA EAX, [EDX + -0x1]
0x80883c2: TEST EDX, EDX
0x80883c4: MOV dword ptr [EBP + 0xfffffeac], EAX
0x80883ca: JLE 0x08088424
0x80883cc: SUB ESP, 0x8
0x80883cf: PUSH EDI
0x80883d0: PUSH EBX
0x80883d1: CALL 0x080959b8
0x80883d6: ADD ESP, 0x10
0x80883d9: TEST EAX, EAX
0x80883db: JNZ 0x080883a4
0x80883dd: SUB ESP, 0xc
0x80883e0: PUSH 0x82abda4
0x80883e5: JMP 0x080881f2
0x80883ea: SUB ESP, 0xc
0x80883ed: PUSH 0x82abdd8
0x80883f2: JMP 0x080881f2
0x80883f7: SUB ESP, 0xc
0x80883fa: PUSH 0x19f7
0x80883ff: PUSH 0x82ab14c
0x8088404: CWDE
0x8088405: PUSH 0x82a376b
0x808840a: SHL EAX, 0x3
0x808840d: PUSH EAX
0x808840e: MOV ECX, dword ptr [EBP + 0xfffffe98]
0x8088414: PUSH dword ptr [ECX + 0x2c]
0x8088417: CALL 0x080cf588
0x808841c: ADD ESP, 0x20
0x808841f: JMP 0x08088304
0x8088424: SUB ESP, 0xc
0x8088427: PUSH EBX
0x8088428: CALL 0x0808729c
0x808842d: ADD ESP, 0x10
0x8088430: TEST EAX, EAX
0x8088432: JNZ 0x0808844e
0x8088434: SUB ESP, 0xc
0x8088437: PUSH 0x82abe10
0x808843c: JMP 0x08087fe1
0x8088441: SUB ESP, 0xc
0x8088444: PUSH 0x82abe40
0x8088449: JMP 0x080881f2
0x808844e: SUB ESP, 0x8
0x8088451: PUSH 0x82ab10b
0x8088456: PUSH EBX
0x8088457: CALL 0x080b8800
0x808845c: ADD ESP, 0x10
0x808845f: TEST EAX, EAX
0x8088461: JNZ 0x08088470
0x8088463: SUB ESP, 0xc
0x8088466: PUSH 0x82abe7c
0x808846b: JMP 0x08087fe1
0x8088470: SUB ESP, 0x8
0x8088473: PUSH ESI
0x8088474: PUSH EBX
0x8088475: CALL 0x080b8bf4
0x808847a: ADD ESP, 0x10
0x808847d: TEST EAX, EAX
0x808847f: JZ 0x0808881e
0x8088485: MOV EAX, dword ptr [EBP + 0xfffffec4]
0x808848b: DEC EAX
0x808848c: MOV dword ptr [EBP + 0xfffffea0], EAX
0x8088492: JS 0x0808883a
0x8088498: MOV EAX, [0x0847aac0]
0x808849d: TEST EAX, EAX
0x808849f: MOV ESI, dword ptr [0x0847aad0]
0x80884a5: CMOVNZ ESI, EAX
0x80884a8: PUSH EAX
0x80884a9: PUSH 0x14
0x80884ab: LEA ECX, [EBP + -0x38]
0x80884ae: PUSH ECX
0x80884af: MOV EDX, dword ptr [EBP + 0xfffffec0]
0x80884b5: PUSH EBX
0x80884b6: MOV EDI, dword ptr [EBP + 0xfffffebc]
0x80884bc: MOV dword ptr [EBP + 0xfffffe94], EDX
0x80884c2: CALL 0x080b8904
0x80884c7: ADD ESP, 0x10
0x80884ca: TEST EAX, EAX
0x80884cc: JZ 0x08088811
0x80884d2: MOV EAX, dword ptr [EBP + -0x38]
0x80884d5: CMP EAX, dword ptr [EBP + 0xfffffe94]
0x80884db: JNC 0x080887b9
0x80884e1: ADD EAX, EDI
0x80884e3: MOV dword ptr [EBP + -0x38], EAX
0x80884e6: MOV EAX, dword ptr [EBP + -0x34]
0x80884e9: TEST EAX, 0x80000
0x80884ee: JZ 0x0808878c
0x80884f4: MOV EAX, dword ptr [EBP + -0x2c]
0x80884f7: TEST EAX, EAX
0x80884f9: JS 0x08088755
0x80884ff: CMP EAX, dword ptr [0x0847aac4]
0x8088505: JGE 0x08088755
0x808850b: MOV ESI, dword ptr [0x0847aad0]
0x8088511: MOV EDX, dword ptr [ESI + 0x54]
0x8088514: MOV EAX, dword ptr [EBP + -0x38]
0x8088517: TEST EDX, EDX
0x8088519: MOV dword ptr [EBP + 0xfffffe90], EAX
0x808851f: JNZ 0x080886b9
0x8088525: MOV EAX, [0x0847aac0]
0x808852a: TEST EAX, EAX
0x808852c: CMOVZ EAX, dword ptr [0x0847aad0]
0x8088533: SUB ESP, 0x8
0x8088536: PUSH dword ptr [EBP + 0xfffffe90]
0x808853c: PUSH EAX
0x808853d: CALL 0x08085a94
0x8088542: MOV EDI, EAX
0x8088544: ADD ESP, 0x10
0x8088547: MOV EDX, dword ptr [EDI + 0x4]
0x808854a: TEST EDX, 0xc0080000
0x8088550: JNZ 0x0808869a
0x8088556: TEST EDX, 0x8000000
0x808855c: JZ 0x08088568
0x808855e: TEST byte ptr [EBP + -0x31], 0x8
0x8088562: JZ 0x0808882b
0x8088568: MOV EAX, dword ptr [EDI + 0x8]
0x808856b: TEST EAX, EAX
0x808856d: JZ 0x080885e4
0x808856f: TEST EDX, EDX
0x8088571: JS 0x08088690
0x8088577: TEST EDX, 0x40000000
0x808857d: JNZ 0x08088681
0x8088583: AND EDX, 0x2ffff
0x8088589: JZ 0x08088596
0x808858b: MOV ESI, dword ptr [EDI + 0x1c]
0x808858e: TEST ESI, ESI
0x8088590: JZ 0x080886a6
0x8088596: PUSH 0x1324
0x808859b: PUSH 0x82ab14c
0x80885a0: PUSH 0x82a376b
0x80885a5: PUSH 0x18
0x80885a7: CALL 0x080ce784
0x80885ac: ADD ESP, 0x10
0x80885af: TEST EAX, EAX
0x80885b1: MOV EDX, EAX
0x80885b3: JZ 0x08088618
0x80885b5: MOV EAX, dword ptr [EDI + 0x14]
0x80885b8: MOV dword ptr [EDX], EAX
0x80885ba: MOV EAX, dword ptr [EDI + 0x18]
0x80885bd: MOV dword ptr [EDX + 0x4], EAX
0x80885c0: MOV EAX, dword ptr [EDI + 0x1c]
0x80885c3: MOV dword ptr [EDX + 0x8], EAX
0x80885c6: MOV EAX, dword ptr [EDI + 0x20]
0x80885c9: MOV dword ptr [EDX + 0xc], EAX
0x80885cc: MOV EAX, dword ptr [EDI + 0x24]
0x80885cf: MOV dword ptr [EDX + 0x10], EAX
0x80885d2: MOV EAX, dword ptr [EDI + 0x28]
0x80885d5: MOV dword ptr [EDX + 0x14], EAX
0x80885d8: MOV EAX, dword ptr [EDI + 0x4]
0x80885db: MOV dword ptr [EDX + 0xc], EAX
0x80885de: MOV dword ptr [EDI + 0x28], EDX
0x80885e1: MOV EAX, dword ptr [EDI + 0x8]
0x80885e4: INC EAX
0x80885e5: MOV dword ptr [EDI + 0x8], EAX
0x80885e8: MOV dword ptr [EDI + 0x1c], 0x0
0x80885ef: MOV EAX, dword ptr [EBP + -0x2c]
0x80885f2: MOV EDX, dword ptr [EBP + -0x28]
0x80885f5: MOV dword ptr [EDI + 0x18], EDX
0x80885f8: MOV dword ptr [EDI + 0x14], EAX
0x80885fb: MOV EAX, dword ptr [EBP + -0x34]
0x80885fe: OR EAX, 0x20000000
0x8088603: INC word ptr [EDI + 0x2e]
0x8088607: DEC dword ptr [EBP + 0xfffffea0]
0x808860d: MOV dword ptr [EDI + 0x20], EAX
0x8088610: MOV dword ptr [EDI + 0x4], EAX
0x8088613: JMP 0x08088492
0x8088618: SUB ESP, 0xc
0x808861b: PUSH 0x82ab6fc
0x8088620: CALL 0x080ff97c
0x8088625: POP EAX
0x8088626: POP EDX
0x8088627: PUSH dword ptr [EDI]
0x8088629: PUSH 0x82abea4
0x808862e: CALL 0x08097acc
0x8088633: ADD ESP, 0x10
0x8088636: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x808863c: CMP EAX, 0xff
0x8088641: JBE 0x08088652
0x8088643: MOV dword ptr [EBP + 0xfffffec0], 0xff
0x808864d: MOV EAX, 0xff
0x8088652: PUSH EDI
0x8088653: PUSH EAX
0x8088654: PUSH dword ptr [EBP + 0xfffffebc]
0x808865a: LEA EBX, [EBP + 0xfffffec8]
0x8088660: PUSH EBX
0x8088661: CALL 0x0804b7e4
0x8088666: POP ECX
0x8088667: POP ESI
0x8088668: PUSH EBX
0x8088669: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x808866f: PUSH 0x82ab129
0x8088674: MOV byte ptr [EBP + EAX*0x1 + 0xfffffec8], 0x0
0x808867c: JMP 0x08087fe1
0x8088681: SUB ESP, 0xc
0x8088684: PUSH 0x82ab6b4
0x8088689: CALL 0x080ff80c
0x808868e: JMP 0x08088625
0x8088690: SUB ESP, 0xc
0x8088693: PUSH 0x82ab6d4
0x8088698: JMP 0x08088689
0x808869a: SUB ESP, 0x8
0x808869d: PUSH dword ptr [EDI]
0x808869f: PUSH 0x82abed4
0x80886a4: JMP 0x0808862e
0x80886a6: PUSH ECX
0x80886a7: PUSH dword ptr [0x08661924]
0x80886ad: PUSH dword ptr [EDI]
0x80886af: PUSH 0x82ab730
0x80886b4: JMP 0x08088620
0x80886b9: SUB ESP, 0x8
0x80886bc: PUSH 0x20
0x80886be: PUSH EAX
0x80886bf: CALL 0x08089dc8
0x80886c4: MOV EDX, EAX
0x80886c6: AND EDX, 0x3ff
0x80886cc: MOV ESI, dword ptr [ESI + 0x54]
0x80886cf: MOV dword ptr [EBP + 0xfffffe88], ESI
0x80886d5: MOV ESI, dword ptr [ESI + EDX*0x4]
0x80886d8: ADD ESP, 0x10
0x80886db: TEST ESI, ESI
0x80886dd: MOV dword ptr [EBP + 0xfffffe8c], EDX
0x80886e3: JZ 0x08088525
0x80886e9: SUB ESP, 0x8
0x80886ec: PUSH dword ptr [EBP + 0xfffffe90]
0x80886f2: PUSH dword ptr [ESI]
0x80886f4: CALL 0x0804b064
0x80886f9: ADD ESP, 0x10
0x80886fc: TEST EAX, EAX
0x80886fe: JNZ 0x0808872e
0x8088700: MOV EAX, dword ptr [ESI + 0x4]
0x8088703: TEST EAX, EAX
0x8088705: JS 0x08088712
0x8088707: MOV EAX, dword ptr [ESI + 0x8]
0x808870a: TEST EAX, EAX
0x808870c: JZ 0x08088525
0x8088712: MOV EAX, ESI
0x8088714: TEST EAX, EAX
0x8088716: JZ 0x08088525
0x808871c: MOV EDI, EAX
0x808871e: MOV EAX, dword ptr [EAX + 0x4]
0x8088721: TEST EAX, EAX
0x8088723: JNS 0x08088525
0x8088729: JMP 0x08088547
0x808872e: MOV EDI, dword ptr [ESI + 0xc]
0x8088731: CMP EDI, ESI
0x8088733: JZ 0x08088525
0x8088739: SUB ESP, 0x8
0x808873c: PUSH dword ptr [EBP + 0xfffffe90]
0x8088742: PUSH dword ptr [EDI]
0x8088744: CALL 0x0804b064
0x8088749: ADD ESP, 0x10
0x808874c: TEST EAX, EAX
0x808874e: JZ 0x08088767
0x8088750: MOV EDI, dword ptr [EDI + 0xc]
0x8088753: JMP 0x08088731
0x8088755: PUSH ESI
0x8088756: PUSH dword ptr [0x0847aac4]
0x808875c: PUSH EAX
0x808875d: PUSH 0x82abf00
0x8088762: JMP 0x0808862e
0x8088767: MOV EAX, dword ptr [EDI + 0x4]
0x808876a: TEST EAX, EAX
0x808876c: JS 0x08088779
0x808876e: MOV EAX, dword ptr [EDI + 0x8]
0x8088771: TEST EAX, EAX
0x8088773: JZ 0x08088525
0x8088779: MOV EAX, dword ptr [EBP + 0xfffffe8c]
0x808877f: MOV EDX, dword ptr [EBP + 0xfffffe88]
0x8088785: MOV dword ptr [EDX + EAX*0x4], EDI
0x8088788: MOV EAX, EDI
0x808878a: JMP 0x08088714
0x808878c: TEST EAX, 0x40000
0x8088791: JZ 0x080887cc
0x8088793: MOV ECX, dword ptr [EBP + -0x2c]
0x8088796: TEST ECX, ECX
0x8088798: JS 0x080887c6
0x808879a: MOV DX, word ptr [ESI + 0xe]
0x808879e: MOVSX EAX, DX
0x80887a1: CMP ECX, EAX
0x80887a3: JL 0x0808850b
0x80887a9: MOVSX EAX, DX
0x80887ac: PUSH EBX
0x80887ad: PUSH EAX
0x80887ae: PUSH ECX
0x80887af: PUSH 0x82abf30
0x80887b4: JMP 0x0808862e
0x80887b9: SUB ESP, 0xc
0x80887bc: PUSH 0x82abf64
0x80887c1: JMP 0x0808862e
0x80887c6: MOV DX, word ptr [ESI + 0xe]
0x80887ca: JMP 0x080887a9
0x80887cc: TEST EAX, 0x8000000
0x80887d1: JZ 0x08088867
0x80887d7: MOV EDX, dword ptr [EBP + -0x28]
0x80887da: TEST DX, DX
0x80887dd: JS 0x08088852
0x80887df: MOV AX, word ptr [ESI + 0x10]
0x80887e3: CMP DX, AX
0x80887e6: JGE 0x08088856
0x80887e8: SUB ESP, 0x8
0x80887eb: LEA EAX, [EBP + 0xfffffea4]
0x80887f1: PUSH EAX
0x80887f2: PUSH EBX
0x80887f3: CALL 0x080959b8
0x80887f8: ADD ESP, 0x10
0x80887fb: TEST EAX, EAX
0x80887fd: JZ 0x08088636
0x8088803: MOV EAX, dword ptr [EBP + 0xfffffea4]
0x8088809: MOV dword ptr [EBP + -0x2c], EAX
0x808880c: JMP 0x0808850b
0x8088811: SUB ESP, 0xc
0x8088814: PUSH 0x82abf88
0x8088819: JMP 0x0808862e
0x808881e: SUB ESP, 0xc
0x8088821: PUSH 0x82abfb8
0x8088826: JMP 0x08087fe1
0x808882b: SUB ESP, 0x8
0x808882e: PUSH dword ptr [EDI]
0x8088830: PUSH 0x82abfe4
0x8088835: JMP 0x0808862e
0x808883a: SUB ESP, 0xc
0x808883d: PUSH dword ptr [EBP + 0xfffffebc]
0x8088843: CALL 0x080891d4
0x8088848: MOV EAX, 0x1
0x808884d: JMP 0x08087fe8
0x8088852: MOV AX, word ptr [ESI + 0x10]
0x8088856: CWDE
0x8088857: PUSH ECX
0x8088858: PUSH EAX
0x8088859: MOVSX EAX, DX
0x808885c: PUSH EAX
0x808885d: PUSH 0x82ac01c
0x8088862: JMP 0x0808862e
0x8088867: SUB ESP, 0xc
0x808886a: PUSH 0x82ac048
0x808886f: JMP 0x0808862e
```

限制：
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

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8093e96", "start": "0x8093d98"}, {"end": "0x8093ead", "start": "0x8093e98"}], "span_bytes": 278, "total_bytes": 277}`

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

限制：
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

## vm_regularProcessor / analyze_function

Evidence: `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80979b0", "start": "0x80978c4"}, {"end": "0x8097a08", "start": "0x80979b4"}, {"end": "0x8097a32", "start": "0x8097a0c"}, {"end": "0x8097a5e", "start": "0x8097a34"}, {"end": "0x8097a8a", "start": "0x8097a60"}, {"end": "0x8097ac8", "start": "0x8097a8c"}], "span_bytes": 517, "total_bytes": 508}`

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

限制：
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
