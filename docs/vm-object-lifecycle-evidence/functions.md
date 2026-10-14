# 阶段 2M 原始函数观察

以下为Ghidra结果，不是原始C源码。完整参数、局限和目标身份以压缩JSON记录为准。

## 本轮：vm_allocateObject — procedure_pseudo_code

Evidence ID：`ev_02d4ce740f27fd92bce43f175d7d6402a86fdbc0c55c9a7cf72664d4092c8530`

```c

Vm_Object_T * __regparm3 vm_allocateObject(Vm_Program_T *pProgram,uint oid)

{
  Vm_Object_T **ppVVar1;
  Vm_Object_T *ptr;
  uint uVar2;
  Vm_Value_T *pVVar3;
  Vm_Object *pVVar4;
  Vm_Object_T *pVVar5;
  int iVar6;
  Vm_Object **ppVVar7;
  Vm_Program_T *pProgram_local;
  
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)] */
  ptr = pdb_allocateMemory(0x40,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x406);
  pVVar5 = (Vm_Object_T *)0x0;
  if (ptr != (Vm_Object_T *)0x0) {
    pVVar5 = ptr;
    for (iVar6 = 0x10; iVar6 != 0; iVar6 = iVar6 + -1) {
      pVVar5->ref = 0;
      pVVar5 = (Vm_Object_T *)&pVVar5->execRef;
    }
    ptr->pProgram = pProgram;
    ptr->oid = oid;
    ptr->uid = 0xff;
    ptr->euid = 0xff;
    if (oid == 0) {
      ptr->flags = 0x1000;
    }
    else {
      ptr->flags = 0x1001;
    }
                    /* Unresolved local var: int idx@[DW_OP_reg0(EAX)] */
    uVar2 = pProgram->strName->hash + oid & 0x3fff;
    ptr->pEnvObject = (Vm_Object *)0x0;
    ptr->pLogderObject = (Vm_Object *)0x0;
    ptr->pNeighbourObject = (Vm_Object *)0x0;
    ptr->pTemplateObject = (Vm_Object *)0x0;
    ppVVar1 = vm_objectHashTable;
    ptr->pNextHashObject = vm_objectHashTable[uVar2];
    ppVVar1[uVar2] = ptr;
    if (pProgram->maxVariableNo < 1) {
      ptr->pObjectValues = (Vm_Value *)0x0;
    }
    else {
      pVVar3 = vm_allocateValues((int)pProgram->maxVariableNo);
      ptr->pObjectValues = pVVar3;
      if (pVVar3 == (Vm_Value_T *)0x0) {
                    /* Unresolved local var: Vm_Object_T * * ppHash@[DW_OP_reg2(EDX)]
                       Unresolved local var: int idx@[???] */
        ppVVar7 = vm_objectHashTable + ((uint)ptr->pProgram->strName->hash + ptr->oid & 0x3fff);
        for (pVVar4 = *ppVVar7; pVVar4 != (Vm_Object_T *)0x0; pVVar4 = pVVar4->pNextHashObject) {
          if (pVVar4 == ptr) {
            *ppVVar7 = ptr->pNextHashObject;
            break;
          }
          ppVVar7 = &pVVar4->pNextHashObject;
        }
        pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x437);
        vm_error("Can not allocate new object (init variabls failed).\n");
      }
    }
    pProgram->ref = pProgram->ref + 1;
    pVVar5 = ptr;
  }
  return pVVar5;
}


```

## 本轮：vm_newThreadCatch2 — procedure_pseudo_code

Evidence ID：`ev_04c0d19e992575aa438242c9b50d1dc5095f14e6645d2794b5e093f287306ba7`

```c

void vm_newThreadCatch2(_func_void_void_ptr *callbackFunc,void *pCookie)

{
  Vm_Catch_Info_T *pVVar1;
  int iVar2;
  Vm_Thread_Info_T *pVVar3;
  int iVar4;
  
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Catch_Info_T * pCatch@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_PCB_T * pPCB@[???] */
  pVVar3 = vm_getCurrentThreadInfo();
  if (_vm_pActiveProcess->catchContextSize <= _vm_pActiveProcess->catchCp) {
    vm_error("Too depth catch (%d).\n",_vm_pActiveProcess->catchCp);
  }
  pVVar1 = _vm_pActiveProcess->catchContext + _vm_pActiveProcess->catchCp;
  _vm_pActiveProcess->catchCp = _vm_pActiveProcess->catchCp + 1;
  pVVar1->catchIp = -1;
  iVar4 = vm_getCurrentFunCp();
  pVVar1->funCp = iVar4;
  pVVar1->sp = vm_sp;
  iVar2 = pVVar3->topCatch;
  pVVar1->catchCallback = callbackFunc;
  pVVar1->pCookie = pCookie;
  if (0 < iVar2) {
    pVVar3->topEnv[iVar2 + -1].funCp = iVar4;
  }
  return;
}


```

## 本轮：vm_returnFromFakeFun — procedure_assembly

Evidence ID：`ev_13bcf5ba026e9e5fa004744ae2eb642987af494bebf7a74f0a830da392e28518`

```asm
0x8094374: PUSH EBP
0x8094375: MOV EBP, ESP
0x8094377: PUSH ESI
0x8094378: PUSH EBX
0x8094379: MOV EAX, [0x086619d8]
0x809437e: MOV ECX, dword ptr [0x086619c0]
0x8094384: MOV ESI, dword ptr [EAX + 0x20]
0x8094387: LEA EDX, [ECX + ECX*0x4]
0x809438a: MOV EAX, [0x086619c8]
0x809438f: LEA EBX, [EAX + EDX*0x8]
0x8094392: MOV EAX, [0x0860ae64]
0x8094397: TEST EAX, EAX
0x8094399: JZ 0x080943e4
0x809439b: CALL 0x080bb0d4
0x80943a0: DEC dword ptr [0x086619c0]
0x80943a6: CALL 0x0809789c
0x80943ab: CALL 0x080bb0e8
0x80943b0: TEST byte ptr [EBX + 0x8], 0x8
0x80943b4: JZ 0x080943f7
0x80943b6: SUB ESP, 0xc
0x80943b9: PUSH dword ptr [EBX + 0x4]
0x80943bc: CALL 0x08091ab0
0x80943c1: ADD ESP, 0x10
0x80943c4: TEST ESI, ESI
0x80943c6: JZ 0x080943d4
0x80943c8: SUB ESP, 0xc
0x80943cb: PUSH ESI
0x80943cc: CALL 0x080b1804
0x80943d1: ADD ESP, 0x10
0x80943d4: TEST byte ptr [EBX + 0x8], 0x80
0x80943d8: JNZ 0x08094408
0x80943da: LEA ESP, [EBP + -0x8]
0x80943dd: POP EBX
0x80943de: POP ESI
0x80943df: LEAVE
0x80943e0: RET
0x80943e4: LEA EAX, [ECX + -0x1]
0x80943e7: MOV [0x086619c0], EAX
0x80943ec: CALL 0x0809789c
0x80943f1: TEST byte ptr [EBX + 0x8], 0x8
0x80943f5: JNZ 0x080943b6
0x80943f7: CALL 0x080a9310
0x80943fc: AND dword ptr [EAX + 0x28], 0xfffffeff
0x8094403: JMP 0x080943b6
0x8094408: SUB ESP, 0xc
0x809440b: PUSH dword ptr [EBX + 0x1c]
0x809440e: CALL dword ptr [EBX + 0x18]
0x8094411: ADD ESP, 0x10
0x8094414: LEA ESP, [EBP + -0x8]
0x8094417: POP EBX
0x8094418: POP ESI
0x8094419: LEAVE
0x809441a: RET
```

## 本轮：vm_freeObjectsInDestructingList — procedure_assembly

Evidence ID：`ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63`

```asm
0x80b1a04: PUSH EBP
0x80b1a05: MOV EBP, ESP
0x80b1a07: PUSH ESI
0x80b1a08: PUSH EBX
0x80b1a09: LEA ESI, [ESI]
0x80b1a0c: MOV EAX, [0x08609a40]
0x80b1a11: TEST EAX, EAX
0x80b1a13: JZ 0x080b1a70
0x80b1a15: MOV EBX, EAX
0x80b1a17: SUB ESP, 0xc
0x80b1a1a: MOV EAX, dword ptr [EAX + 0x28]
0x80b1a1d: MOV [0x08609a40], EAX
0x80b1a22: MOV dword ptr [EBX + 0x28], 0x0
0x80b1a29: PUSH EBX
0x80b1a2a: CALL 0x080b1894
0x80b1a2f: MOV EAX, dword ptr [EBX]
0x80b1a31: DEC EAX
0x80b1a32: ADD ESP, 0x10
0x80b1a35: TEST EAX, EAX
0x80b1a37: MOV dword ptr [EBX], EAX
0x80b1a39: JG 0x080b1a0c
0x80b1a3b: TEST byte ptr [EBX + 0xd], 0x80
0x80b1a3f: JZ 0x080b1a0c
0x80b1a41: MOV ESI, dword ptr [EBX + 0x20]
0x80b1a44: MOV EAX, dword ptr [ESI]
0x80b1a46: DEC EAX
0x80b1a47: TEST EAX, EAX
0x80b1a49: MOV dword ptr [ESI], EAX
0x80b1a4b: JLE 0x080b1a77
0x80b1a4d: PUSH 0x1f5
0x80b1a52: PUSH 0x82b0e28
0x80b1a57: PUSH 0x82a376b
0x80b1a5c: PUSH EBX
0x80b1a5d: CALL 0x080ceb94
0x80b1a62: ADD ESP, 0x10
0x80b1a65: MOV EAX, [0x08609a40]
0x80b1a6a: TEST EAX, EAX
0x80b1a6c: JNZ 0x080b1a15
0x80b1a6e: MOV ESI, ESI
0x80b1a70: LEA ESP, [EBP + -0x8]
0x80b1a73: POP EBX
0x80b1a74: POP ESI
0x80b1a75: LEAVE
0x80b1a76: RET
0x80b1a77: SUB ESP, 0x8
0x80b1a7a: LEA EAX, [ESI + 0x34]
0x80b1a7d: PUSH EAX
0x80b1a7e: PUSH ESI
0x80b1a7f: CALL 0x080965a4
0x80b1a84: MOV EAX, [0x0860ae64]
0x80b1a89: ADD ESP, 0x10
0x80b1a8c: TEST EAX, EAX
0x80b1a8e: JNZ 0x080b1aad
0x80b1a90: SUB ESP, 0xc
0x80b1a93: PUSH ESI
0x80b1a94: CALL 0x080862a8
0x80b1a99: MOV ECX, dword ptr [0x0860ae64]
0x80b1a9f: ADD ESP, 0x10
0x80b1aa2: TEST ECX, ECX
0x80b1aa4: JZ 0x080b1a4d
0x80b1aa6: CALL 0x080bb0c0
0x80b1aab: JMP 0x080b1a4d
0x80b1aad: CALL 0x080bb0ac
0x80b1ab2: JMP 0x080b1a90
```

## 本轮：vm_doActions — procedure_pseudo_code

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

## 本轮：vm_nullAction — procedure_pseudo_code

Evidence ID：`ev_21fbfd6bbe3870306825dbc83490032d16ecdd14f90738b427ece2604c257aa2`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_nullAction(void)

{
  return;
}


```

## 本轮：vm_loadObject — procedure_pseudo_code

Evidence ID：`ev_228dbc19a970bf00daf604f9e1f1171e1f9410d10ca94a49d063f0687b923800`

```c

/* WARNING: Unknown calling convention */

Vm_Object_T * vm_loadObject(char *pInFileName,uchar *content,int contentLen)

{
  char cVar1;
  int iVar2;
  Vm_String_T *pStrObjectName;
  Vm_Object_T *pVVar3;
  Vm_Program_T *pProgram;
  uint uVar4;
  undefined4 extraout_ECX;
  undefined4 extraout_ECX_00;
  undefined4 uVar5;
  char *pcVar6;
  char *pcVar7;
  uint oid;
  undefined4 uVar8;
  int binFileFd;
  char *builtInFileName;
  char *programFileName;
  char binFileName [256];
  char tempName [256];
  char regularName [256];
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Object_T * pRootObject@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg0(EAX)] */
  builtInFileName = (char *)0x0;
  iVar2 = vm_deriveFileName(pInFileName,".c",regularName,0x100);
  if (iVar2 == 0) {
    return (Vm_Object_T *)0x0;
  }
  oid = 0;
  uVar4 = 0xffffffff;
  pcVar6 = regularName;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  pStrObjectName = vm_findSharedString(regularName,~uVar4 - 1);
  pVVar3 = vm_lookupObject(pStrObjectName,oid);
  if (pVVar3 != (Vm_Object_T *)0x0) {
    return pVVar3;
  }
  uVar5 = extraout_ECX;
  programFileName = regularName;
  if (content != (uchar *)0x0) {
    programFileName = tempName;
    pcVar6 = "/v/::_load_object_::content";
    pcVar7 = tempName;
    for (iVar2 = 7; iVar2 != 0; iVar2 = iVar2 + -1) {
      *(undefined4 *)pcVar7 = *(undefined4 *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar7 = pcVar7 + 4;
    }
    builtInFileName = tempName + 3;
    vm_unlinkBuiltInFile(builtInFileName);
    vm_registerNewBuiltInFile(builtInFileName,(char *)content,contentLen,0);
    uVar5 = extraout_ECX_00;
  }
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg7(EDI)]
                       Unresolved local var: int fileFd@[DW_OP_reg6(ESI)] */
  uVar8 = 0x100;
  iVar2 = vm_generateBinFileName(programFileName,binFileName,0x100);
  if (((iVar2 != 0) && (iVar2 = vm_compareFileTime(programFileName,binFileName), iVar2 < 0)) &&
     (iVar2 = vm_openLocalFile(binFileName,1), 0 < iVar2)) {
    pProgram = vm_createProgram(regularName,iVar2,binFileName);
    vm_close(iVar2);
    if (pProgram != (Vm_Program_T *)0x0) goto LAB_080b1fbc;
  }
  iVar2 = vm_openLocalFile(programFileName,1);
  if (iVar2 < 0) {
    vm_errprintf("Can not open script file \"%s\" to execute.\n",programFileName,uVar8,uVar5);
    pProgram = (Vm_Program_T *)0x0;
  }
  else {
    pProgram = vm_createProgram(regularName,iVar2,programFileName);
    vm_close(iVar2);
  }
LAB_080b1fbc:
  if (builtInFileName != (char *)0x0) {
    vm_unlinkBuiltInFile(builtInFileName);
  }
  pVVar3 = (Vm_Object_T *)0x0;
  if ((pProgram != (Vm_Program_T *)0x0) &&
     (pVVar3 = vm_allocateObject(pProgram,0), pVVar3 == (Vm_Object_T *)0x0)) {
    vm_freeCodeSegment(pProgram,&pProgram->initCodeSegment);
    if (vm_gatheringCallStackInfo != 0) {
      vm_enterPPWholeCriticalSection();
    }
    vm_destructProgramHead(pProgram);
    if (vm_gatheringCallStackInfo == 0) {
      pVVar3 = (Vm_Object_T *)0x0;
    }
    else {
      vm_leavePPWholeCriticalSection();
      pVVar3 = (Vm_Object_T *)0x0;
    }
  }
  return pVVar3;
}


```

## 本轮：vm_shutdownObject — procedure_pseudo_code

Evidence ID：`ev_25d55e24d66d458ebaf2ceb6aac63cab835b100c5130c82b5138890178888577`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_shutdownObject(void)

{
  Vm_Object *pVVar1;
  Vm_Program_T *pVVar2;
  Vm_Value *pVVar3;
  Vm_Object_T **ppVVar4;
  Vm_Object_T *pVVar5;
  int iVar6;
  int iVar7;
  Vm_Program_T *pProgram;
  int i;
  Vm_Object_T *pNext;
  
  if (vm_objectHashTable != (Vm_Object_T **)0x0) {
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)] */
    i = 0;
    ppVVar4 = vm_objectHashTable;
    do {
      pVVar1 = ppVVar4[i];
      while (pVVar5 = pVVar1, pVVar5 != (Vm_Object_T *)0x0) {
        pVVar1 = pVVar5->pNextHashObject;
        vm_makeObjectAlone(pVVar5);
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
        if (pVVar5->pBindValue != (Vm_Value_T *)0x0) {
          vm_unrefValue(pVVar5->pBindValue);
          pVVar5->pBindValue = (Vm_Value *)0x0;
        }
        pVVar3 = pVVar5->pObjectValues;
        ppVVar4 = vm_objectHashTable;
        if (pVVar3 != (Vm_Value *)0x0) {
          pVVar2 = pVVar5->pProgram;
          iVar7 = 0;
          iVar6 = 0;
          if (0 < pVVar2->maxVariableNo) {
            while( true ) {
              vm_destructStaticValue((Vm_Value_T *)((int)&pVVar3->ref + iVar6));
              iVar7 = iVar7 + 1;
              iVar6 = iVar6 + 0x18;
              if (pVVar2->maxVariableNo <= iVar7) break;
              pVVar3 = pVVar5->pObjectValues;
            }
          }
          pdb_freeMemory(pVVar5->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",
                         0x1ce);
          pVVar5->pObjectValues = (Vm_Value *)0x0;
          ppVVar4 = vm_objectHashTable;
        }
      }
      i = i + 1;
    } while (i < 0x4000);
    i = 0;
    do {
      pVVar5 = ppVVar4[i];
      while (pVVar5 != (Vm_Object_T *)0x0) {
        pVVar1 = pVVar5->pNextHashObject;
        vm_destructObject(pVVar5);
        pVVar5 = pVVar1;
      }
      i = i + 1;
      ppVVar4 = vm_objectHashTable;
    } while (i < 0x4000);
    pdb_freeMemory(vm_objectHashTable,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x5b);
  }
  return;
}


```

## 本轮：vm_findProgramFunction — procedure_assembly

Evidence ID：`ev_39ff14e286b302152ec3587aaff3135b507be423df85fb3b0c7651a2f982da7a`

```asm
0x8086b44: PUSH EBP
0x8086b45: MOV EBP, ESP
0x8086b47: PUSH EDI
0x8086b48: PUSH ESI
0x8086b49: PUSH EBX
0x8086b4a: SUB ESP, 0x1c
0x8086b4d: MOV EAX, dword ptr [EBP + 0x8]
0x8086b50: MOV EAX, dword ptr [EAX + 0x54]
0x8086b53: TEST EAX, EAX
0x8086b55: MOV EDI, dword ptr [EBP + 0xc]
0x8086b58: MOV dword ptr [EBP + -0x18], EAX
0x8086b5b: JZ 0x08086c18
0x8086b61: MOV AX, word ptr [EDI + 0x8]
0x8086b65: TEST AX, AX
0x8086b68: JZ 0x08086cd0
0x8086b6e: MOV EDX, dword ptr [EBP + -0x18]
0x8086b71: TEST EDX, EDX
0x8086b73: MOV EDI, dword ptr [EDI + 0xc]
0x8086b76: MOVZX EAX, AX
0x8086b79: JZ 0x08086ca4
0x8086b7f: TEST EAX, EAX
0x8086b81: JZ 0x08086cf2
0x8086b87: AND EAX, 0x3ff
0x8086b8c: MOV dword ptr [EBP + -0x14], EAX
0x8086b8f: MOV EAX, dword ptr [EBP + -0x14]
0x8086b92: MOV EDX, dword ptr [EBP + -0x18]
0x8086b95: MOV ESI, dword ptr [EDX + EAX*0x4]
0x8086b98: TEST ESI, ESI
0x8086b9a: JZ 0x08086ca4
0x8086ba0: SUB ESP, 0x8
0x8086ba3: PUSH EDI
0x8086ba4: PUSH dword ptr [ESI]
0x8086ba6: CALL 0x0804b064
0x8086bab: ADD ESP, 0x10
0x8086bae: TEST EAX, EAX
0x8086bb0: JNZ 0x08086c7d
0x8086bb6: MOV EAX, dword ptr [ESI + 0x4]
0x8086bb9: TEST EAX, EAX
0x8086bbb: JS 0x08086bc8
0x8086bbd: MOV EDI, dword ptr [ESI + 0x8]
0x8086bc0: TEST EDI, EDI
0x8086bc2: JZ 0x08086ca4
0x8086bc8: MOV EAX, ESI
0x8086bca: TEST EAX, EAX
0x8086bcc: JZ 0x08086ca4
0x8086bd2: TEST byte ptr [EAX + 0x7], 0x8
0x8086bd6: JZ 0x08086ca4
0x8086bdc: MOV EDX, dword ptr [EAX + 0x14]
0x8086bdf: MOV dword ptr [EBP + -0x10], EDX
0x8086be2: MOV EDX, dword ptr [EBP + 0x14]
0x8086be5: TEST EDX, EDX
0x8086be7: JZ 0x08086bf4
0x8086be9: MOV AX, word ptr [EAX + 0x18]
0x8086bed: MOV ECX, dword ptr [EBP + 0x14]
0x8086bf0: MOV word ptr [ECX], AX
0x8086bf3: NOP
0x8086bf4: MOV EAX, dword ptr [EBP + 0x10]
0x8086bf7: TEST EAX, EAX
0x8086bf9: JNZ 0x08086c0a
0x8086bfb: MOV ECX, dword ptr [EBP + -0x10]
0x8086bfe: XOR EAX, EAX
0x8086c00: TEST byte ptr [ECX + 0x8], 0x4
0x8086c04: JNZ 0x08086ca6
0x8086c0a: MOV EAX, dword ptr [EBP + -0x10]
0x8086c0d: LEA ESP, [EBP + -0xc]
0x8086c10: POP EBX
0x8086c11: POP ESI
0x8086c12: POP EDI
0x8086c13: LEAVE
0x8086c14: RET
0x8086c18: TEST byte ptr [EDI + 0x4], 0x1
0x8086c1c: JZ 0x08086cae
0x8086c22: MOV EBX, dword ptr [EBP + 0x8]
0x8086c25: MOVSX EBX, word ptr [EBX + 0x5c]
0x8086c29: MOV EAX, EDI
0x8086c2b: SHR EAX, 0x3
0x8086c2e: MOV dword ptr [EBP + -0x20], EBX
0x8086c31: XOR EDX, EDX
0x8086c33: DIV EBX
0x8086c35: XOR ESI, ESI
0x8086c37: CMP dword ptr [EBP + -0x20], 0x0
0x8086c3b: MOV EBX, EDX
0x8086c3d: JLE 0x08086bf4
0x8086c3f: MOV ECX, dword ptr [EBP + 0x8]
0x8086c42: MOV ECX, dword ptr [ECX + 0x60]
0x8086c45: MOV dword ptr [EBP + -0x1c], ECX
0x8086c48: MOV ECX, dword ptr [EBP + -0x20]
0x8086c4b: NOP
0x8086c4c: MOV EDX, dword ptr [EBP + -0x1c]
0x8086c4f: MOV AX, word ptr [EDX + EBX*0x4]
0x8086c53: CMP AX, -0x1
0x8086c57: JZ 0x08086ca4
0x8086c59: MOVSX EDX, AX
0x8086c5c: MOV EAX, dword ptr [EBP + 0x8]
0x8086c5f: MOV EAX, dword ptr [EAX + 0x20]
0x8086c62: CMP dword ptr [EAX + EDX*0x4], EDI
0x8086c65: JZ 0x08086d19
0x8086c6b: LEA EAX, [EBX + 0x1]
0x8086c6e: CDQ
0x8086c6f: INC ESI
0x8086c70: IDIV ECX
0x8086c72: CMP ECX, ESI
0x8086c74: MOV EBX, EDX
0x8086c76: JG 0x08086c4c
0x8086c78: JMP 0x08086bf4
0x8086c7d: MOV EBX, dword ptr [ESI + 0xc]
0x8086c80: JMP 0x08086c9d
0x8086c84: SUB ESP, 0x8
0x8086c87: PUSH EDI
0x8086c88: PUSH dword ptr [EBX]
0x8086c8a: CALL 0x0804b064
0x8086c8f: ADD ESP, 0x10
0x8086c92: TEST EAX, EAX
0x8086c94: JZ 0x08086d46
0x8086c9a: MOV EBX, dword ptr [EBX + 0xc]
0x8086c9d: CMP EBX, ESI
0x8086c9f: JNZ 0x08086c84
0x8086ca1: LEA ESI, [ESI]
0x8086ca4: XOR EAX, EAX
0x8086ca6: LEA ESP, [EBP + -0xc]
0x8086ca9: POP EBX
0x8086caa: POP ESI
0x8086cab: POP EDI
0x8086cac: LEAVE
0x8086cad: RET
0x8086cae: SUB ESP, 0x8
0x8086cb1: PUSH dword ptr [EDI + 0x10]
0x8086cb4: PUSH dword ptr [EDI + 0xc]
0x8086cb7: CALL 0x080bd2b0
0x8086cbc: MOV EDI, EAX
0x8086cbe: ADD ESP, 0x10
0x8086cc1: XOR EAX, EAX
0x8086cc3: TEST EDI, EDI
0x8086cc5: JNZ 0x08086c22
0x8086ccb: JMP 0x08086ca6
0x8086cd0: SUB ESP, 0x8
0x8086cd3: PUSH 0x20
0x8086cd5: PUSH dword ptr [EDI + 0xc]
0x8086cd8: CALL 0x08089dc8
0x8086cdd: MOV EDX, dword ptr [EBP + 0x8]
0x8086ce0: MOV EDX, dword ptr [EDX + 0x54]
0x8086ce3: ADD ESP, 0x10
0x8086ce6: MOV word ptr [EDI + 0x8], AX
0x8086cea: MOV dword ptr [EBP + -0x18], EDX
0x8086ced: JMP 0x08086b6e
0x8086cf2: SUB ESP, 0x8
0x8086cf5: PUSH 0x20
0x8086cf7: PUSH EDI
0x8086cf8: CALL 0x08089dc8
0x8086cfd: MOV ECX, EAX
0x8086cff: MOV EBX, dword ptr [EBP + 0x8]
0x8086d02: AND ECX, 0x3ff
0x8086d08: MOV EBX, dword ptr [EBX + 0x54]
0x8086d0b: ADD ESP, 0x10
0x8086d0e: MOV dword ptr [EBP + -0x14], ECX
0x8086d11: MOV dword ptr [EBP + -0x18], EBX
0x8086d14: JMP 0x08086b8f
0x8086d19: MOV EDX, dword ptr [EBP + -0x1c]
0x8086d1c: MOV CX, word ptr [EDX + EBX*0x4 + 0x2]
0x8086d21: MOV EBX, dword ptr [EBP + 0x8]
0x8086d24: MOV EAX, dword ptr [EBX + 0x2c]
0x8086d27: MOVSX EDX, CX
0x8086d2a: MOV EDX, dword ptr [EAX + EDX*0x8]
0x8086d2d: MOV EAX, dword ptr [EBP + 0x14]
0x8086d30: TEST EAX, EAX
0x8086d32: MOV dword ptr [EBP + -0x10], EDX
0x8086d35: JZ 0x08086bf4
0x8086d3b: MOV EAX, dword ptr [EBP + 0x14]
0x8086d3e: MOV word ptr [EAX], CX
0x8086d41: JMP 0x08086bf4
0x8086d46: MOV ESI, dword ptr [EBX + 0x4]
0x8086d49: TEST ESI, ESI
0x8086d4b: JS 0x08086d58
0x8086d4d: MOV ECX, dword ptr [EBX + 0x8]
0x8086d50: TEST ECX, ECX
0x8086d52: JZ 0x08086ca4
0x8086d58: MOV EAX, dword ptr [EBP + -0x18]
0x8086d5b: MOV ECX, dword ptr [EBP + -0x14]
0x8086d5e: MOV dword ptr [EAX + ECX*0x4], EBX
0x8086d61: MOV EAX, EBX
0x8086d63: JMP 0x08086bca
```

## 本轮：vm_endForBackend — procedure_callers

Evidence ID：`ev_43a949723da25ef7b139ceed352eda43fc22238fe5c47fabbd0847f52e078ee1`

```json
[
  "0x80acca8",
  "0x80ad1d4"
]
```

## 本轮：vm_f_destruct_object — procedure_assembly

Evidence ID：`ev_460c4ca1879ca5bbedea229725c71d14436b1e2f6d42e4c90983a64d2134a069`

```asm
0x80f9420: PUSH EBP
0x80f9421: MOV EBP, ESP
0x80f9423: PUSH ESI
0x80f9424: PUSH EBX
0x80f9425: MOV EAX, [0x086619d8]
0x80f942a: CMP dword ptr [EAX + 0x4], 0x1
0x80f942e: JG 0x080f94d4
0x80f9434: MOV EAX, dword ptr [EAX + 0x4]
0x80f9437: TEST EAX, EAX
0x80f9439: JLE 0x080f94f4
0x80f943f: MOV EAX, [0x08661960]
0x80f9444: CMP dword ptr [0x086619cc], EAX
0x80f944a: JGE 0x080f9518
0x80f9450: CALL 0x08090604
0x80f9455: MOV ESI, EAX
0x80f9457: MOVZX EAX, byte ptr [EAX + 0x4]
0x80f945b: CMP EAX, 0x4
0x80f945e: JZ 0x080f953c
0x80f9464: CMP EAX, 0xa
0x80f9467: JZ 0x080f95ac
0x80f946d: SUB ESP, 0xc
0x80f9470: PUSH 0x82bfd08
0x80f9475: CALL 0x08097b24
0x80f947a: ADD ESP, 0x10
0x80f947d: MOV EBX, dword ptr [ESI + 0x8]
0x80f9480: MOV EAX, dword ptr [EBX + 0xc]
0x80f9483: TEST AH, 0xe0
0x80f9486: JZ 0x080f94a0
0x80f9488: SUB ESP, 0xc
0x80f948b: MOV EAX, [0x086619d8]
0x80f9490: PUSH dword ptr [EAX + 0x4]
0x80f9493: CALL 0x08091ab0
0x80f9498: LEA ESP, [EBP + -0x8]
0x80f949b: POP EBX
0x80f949c: POP ESI
0x80f949d: LEAVE
0x80f949e: RET
0x80f94a0: OR AH, 0x20
0x80f94a3: MOV dword ptr [EBX + 0xc], EAX
0x80f94a6: PUSH EDX
0x80f94a7: PUSH EBX
0x80f94a8: PUSH 0x80f95f4
0x80f94ad: PUSH EAX
0x80f94ae: CALL 0x080a9310
0x80f94b3: MOV dword ptr [ESP], EAX
0x80f94b6: CALL 0x080a9964
0x80f94bb: ADD ESP, 0x10
0x80f94be: MOV EAX, EBX
0x80f94c0: CALL 0x080f9284
0x80f94c5: OR dword ptr [0x086619d4], 0x10
0x80f94cc: LEA ESP, [EBP + -0x8]
0x80f94cf: POP EBX
0x80f94d0: POP ESI
0x80f94d1: LEAVE
0x80f94d2: RET
0x80f94d4: SUB ESP, 0xc
0x80f94d7: PUSH 0x82bfd34
0x80f94dc: CALL 0x08097b24
0x80f94e1: MOV EAX, [0x086619d8]
0x80f94e6: MOV EAX, dword ptr [EAX + 0x4]
0x80f94e9: ADD ESP, 0x10
0x80f94ec: TEST EAX, EAX
0x80f94ee: JG 0x080f943f
0x80f94f4: SUB ESP, 0xc
0x80f94f7: PUSH 0x82bf460
0x80f94fc: CALL 0x08097b24
0x80f9501: MOV EAX, [0x08661960]
0x80f9506: ADD ESP, 0x10
0x80f9509: CMP dword ptr [0x086619cc], EAX
0x80f950f: JL 0x080f9450
0x80f9515: LEA ESI, [ESI]
0x80f9518: SUB ESP, 0xc
0x80f951b: PUSH 0x82a9240
0x80f9520: CALL 0x08097b24
0x80f9525: ADD ESP, 0x10
0x80f9528: CALL 0x08090604
0x80f952d: MOV ESI, EAX
0x80f952f: MOVZX EAX, byte ptr [EAX + 0x4]
0x80f9533: CMP EAX, 0x4
0x80f9536: JNZ 0x080f9464
0x80f953c: CALL 0x08090604
0x80f9541: MOV EBX, EAX
0x80f9543: MOV AL, byte ptr [EAX + 0x4]
0x80f9546: CMP AL, 0x4
0x80f9548: JZ 0x080f9566
0x80f954a: SUB ESP, 0xc
0x80f954d: MOVZX EAX, AL
0x80f9550: PUSH EAX
0x80f9551: CALL 0x0808a13c
0x80f9556: POP ECX
0x80f9557: POP ESI
0x80f9558: PUSH EAX
0x80f9559: PUSH 0x82bf11c
0x80f955e: CALL 0x08097b24
0x80f9563: ADD ESP, 0x10
0x80f9566: SUB ESP, 0xc
0x80f9569: PUSH dword ptr [EBX + 0x8]
0x80f956c: CALL 0x080b114c
0x80f9571: MOV EBX, EAX
0x80f9573: ADD ESP, 0x10
0x80f9576: CALL 0x0809061c
0x80f957b: TEST EBX, EBX
0x80f957d: JZ 0x080f95be
0x80f957f: CMP dword ptr [EBX + 0xc], 0x800
0x80f9586: JZ 0x080f95be
0x80f9588: SUB ESP, 0xc
0x80f958b: PUSH EBX
0x80f958c: CALL 0x080912f4
0x80f9591: ADD ESP, 0x10
0x80f9594: CALL 0x08090604
0x80f9599: CMP byte ptr [EAX + 0x4], 0xa
0x80f959d: MOV ESI, EAX
0x80f959f: JNZ 0x080f946d
0x80f95a5: JMP 0x080f947d
0x80f95ac: MOV EBX, dword ptr [ESI + 0x8]
0x80f95af: TEST byte ptr [EBX + 0xd], 0x80
0x80f95b3: JZ 0x080f9480
0x80f95b9: JMP 0x080f946d
0x80f95be: SUB ESP, 0xc
0x80f95c1: PUSH 0x0
0x80f95c3: CALL 0x08090b3c
0x80f95c8: JMP 0x080f9591
```

## 本轮：vm_f_construct_object_ok1 — procedure_pseudo_code

Evidence ID：`ev_4a7a31ef99a96b1d6fff61a35b87e55b84ae44a3e25bff0a2947f98e13f3a4e0`

```c

void vm_f_construct_object_ok1(void *pCookie)

{
  vm_endThreadCatch();
  vm_f_construct_object2(pCookie);
  return;
}


```

## 本轮：0x80aca84 — procedure_pseudo_code

Evidence ID：`ev_4a9a0054a1392bc09a3c1dc2129e02c465dc75325ec76a95e30dbebb03a82f67`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_clearEnvironment(void)

{
  uint *puVar1;
  uint uVar2;
  Vm_PCB *pVVar3;
  Vm_PCB_T *pVVar4;
  int iVar5;
  Vm_Timer_T *ptr;
  Vm_Timer_T **ppVVar6;
  Vm_PCB_T *pVVar7;
  Vm_PCB_T *pPCB;
  uint uVar8;
  Vm_Oid_T oid;
  uint uVar9;
  int iVar10;
  
  while (_vm_pTimerList != (Vm_Timer_T *)0x0) {
    vm_removeTimerFromTimerList(_vm_pTimerList->type,(_vm_pTimerList->v).pIdent);
  }
  _vm_pTimerList = (Vm_Timer_T *)0x0;
  ptr = _vm_pInvokeList;
  ppVVar6 = _vm_ppLastInvokeNode;
  uVar9 = vm_maxPendObject;
  while (_vm_ppLastInvokeNode = ppVVar6, vm_maxPendObject = uVar9, ptr != (Vm_Timer_T *)0x0) {
    _vm_pInvokeList = ptr->pNext;
    _vm_ppLastInvokeNode = &_vm_pInvokeList;
    if (_vm_pInvokeList != (Vm_Timer *)0x0) {
      _vm_ppLastInvokeNode = ppVVar6;
    }
    if (ptr->pOwnerObject == (Vm_Object_T *)0x0) {
      iVar10 = ptr->type;
      iVar5 = iVar10 + -2;
      if (iVar10 != 2) goto LAB_080acc0a;
LAB_080acc3b:
      vm_unrefValue((ptr->v).pValue);
    }
    else {
      vm_unrefObject(ptr->pOwnerObject);
      iVar10 = ptr->type;
      iVar5 = iVar10 + -2;
      if (iVar5 == 0) goto LAB_080acc3b;
LAB_080acc0a:
      if (SBORROW4(iVar10,2) == iVar5 < 0) {
        if (iVar10 != 3) {
LAB_080acc76:
          vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                     "Bad type of timer when delete all timer.\n",
                     "/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x11f6);
          _vm_endProcess(0);
          do {
                    /* WARNING: Do nothing block with infinite loop */
          } while( true );
        }
        if (ptr->finishHook != (Vm_Timer_Finish_Hook_T)0x0) {
          (*ptr->finishHook)((Vm_PCB_T *)0x0,ptr->pFinishCookie);
        }
      }
      else {
        if (iVar10 != 1) goto LAB_080acc76;
        puVar1 = &((ptr->v).pPCB)->flags;
        *puVar1 = *puVar1 & 0xffffff7f;
      }
    }
    pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0xc42);
    ptr = _vm_pInvokeList;
    ppVVar6 = _vm_ppLastInvokeNode;
    uVar9 = vm_maxPendObject;
  }
                    /* Unresolved local var: Vm_Oid_T i@[DW_OP_reg3(EBX)] */
  oid = 0;
  _vm_pInvokeList = (Vm_Timer_T *)0x0;
  if (uVar9 != 0) {
    do {
      if (_vmObject[oid] != (void *)0x0) {
        vm_deletePendObject(oid);
        uVar9 = vm_maxPendObject;
      }
      oid = oid + 1;
    } while (oid < uVar9);
  }
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg1(ECX)]
                       Unresolved local var: uint i@[DW_OP_reg6(ESI)] */
  uVar9 = 0;
  if (vm_maxProcess != 0) {
    iVar10 = 0;
    uVar8 = vm_maxProcess;
    do {
      while( true ) {
        pVVar7 = _vm_pActiveProcess;
        pPCB = (Vm_PCB_T *)(_vm_allPCB->name + iVar10);
        uVar2 = pPCB->flags;
        if (((uVar2 & 1) == 0) || ((uVar2 & 0x20) != 0)) break;
        if ((uVar2 & 6) == 0) {
          pVVar3 = pPCB->pPrev;
          pVVar3->pNext = pPCB->pNext;
          _vm_currentActiveProcess = _vm_currentActiveProcess + -1;
          pVVar4 = pPCB->pNext;
          pVVar4->pPrev = pVVar3;
          if ((pPCB == pVVar7) && (_vm_pActiveProcess = (Vm_PCB_T *)0x0, pVVar4 != pPCB)) {
            _vm_pActiveProcess = pVVar4;
          }
          pPCB->pNext = (Vm_PCB *)0x0;
          pPCB->pPrev = (Vm_PCB *)0x0;
        }
        pPCB->returnCode = -3;
        vm_deleteProcess(pPCB);
        uVar9 = uVar9 + 1;
        iVar10 = iVar10 + 0x1ec;
        uVar8 = vm_maxProcess;
        if (vm_maxProcess <= uVar9) goto LAB_080acb7e;
      }
      uVar9 = uVar9 + 1;
      iVar10 = iVar10 + 0x1ec;
    } while (uVar9 < uVar8);
  }
LAB_080acb7e:
  vm_freeObjectsInDestructingList();
  vm_destructIdentifiers();
  vm_destructAllObjects(0);
  vm_cleanupAllBreakpoints();
  vm_initIdentifiers();
  return;
}


```

## 本轮：vm_destructObject — procedure_assembly

Evidence ID：`ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b`

```asm
0x80b1894: PUSH EBP
0x80b1895: MOV EBP, ESP
0x80b1897: PUSH EDI
0x80b1898: PUSH ESI
0x80b1899: PUSH EBX
0x80b189a: SUB ESP, 0xc
0x80b189d: MOV EBX, dword ptr [EBP + 0x8]
0x80b18a0: MOV EDX, dword ptr [EBX + 0x4]
0x80b18a3: TEST EDX, EDX
0x80b18a5: JLE 0x080b18b8
0x80b18a7: OR dword ptr [EBX + 0xc], 0x4000
0x80b18ae: LEA ESP, [EBP + -0xc]
0x80b18b1: POP EBX
0x80b18b2: POP ESI
0x80b18b3: POP EDI
0x80b18b4: LEAVE
0x80b18b5: RET
0x80b18b8: SUB ESP, 0xc
0x80b18bb: PUSH EBX
0x80b18bc: CALL 0x080b1728
0x80b18c1: MOV EAX, dword ptr [EBX + 0x1c]
0x80b18c4: ADD ESP, 0x10
0x80b18c7: TEST EAX, EAX
0x80b18c9: JNZ 0x080b19e2
0x80b18cf: MOV EAX, dword ptr [EBX + 0x24]
0x80b18d2: TEST EAX, EAX
0x80b18d4: JZ 0x080b190c
0x80b18d6: MOV EDX, dword ptr [EBX + 0x20]
0x80b18d9: XOR EDI, EDI
0x80b18db: XOR ESI, ESI
0x80b18dd: CMP word ptr [EDX + 0xe], 0x0
0x80b18e2: MOV dword ptr [EBP + -0x10], EDX
0x80b18e5: JG 0x080b197b
0x80b18eb: PUSH 0x1ce
0x80b18f0: PUSH 0x82b0e28
0x80b18f5: PUSH 0x82a376b
0x80b18fa: PUSH dword ptr [EBX + 0x24]
0x80b18fd: CALL 0x080ceb94
0x80b1902: ADD ESP, 0x10
0x80b1905: MOV dword ptr [EBX + 0x24], 0x0
0x80b190c: MOV ESI, dword ptr [EBX + 0x20]
0x80b190f: MOV EAX, dword ptr [ESI + 0x4]
0x80b1912: MOVZX EAX, word ptr [EAX + 0x8]
0x80b1916: ADD EAX, dword ptr [EBX + 0x8]
0x80b1919: AND EAX, 0x3fff
0x80b191e: MOV EDX, dword ptr [0x08609a44]
0x80b1924: LEA EDX, [EDX + EAX*0x4]
0x80b1927: MOV EAX, dword ptr [EDX]
0x80b1929: JMP 0x080b193a
0x80b192c: CMP EAX, EBX
0x80b192e: JZ 0x080b19d8
0x80b1934: LEA EDX, [EAX + 0x2c]
0x80b1937: MOV EAX, dword ptr [EAX + 0x2c]
0x80b193a: TEST EAX, EAX
0x80b193c: JNZ 0x080b192c
0x80b193e: MOV EAX, dword ptr [EBX]
0x80b1940: TEST EAX, EAX
0x80b1942: JNZ 0x080b196a
0x80b1944: MOV EAX, dword ptr [ESI]
0x80b1946: DEC EAX
0x80b1947: TEST EAX, EAX
0x80b1949: MOV dword ptr [ESI], EAX
0x80b194b: JLE 0x080b199d
0x80b194d: PUSH 0x1f5
0x80b1952: PUSH 0x82b0e28
0x80b1957: PUSH 0x82a376b
0x80b195c: PUSH EBX
0x80b195d: CALL 0x080ceb94
0x80b1962: ADD ESP, 0x10
0x80b1965: JMP 0x080b18ae
0x80b196a: OR dword ptr [EBX + 0xc], 0x8000
0x80b1971: JMP 0x080b18ae
0x80b1978: MOV EAX, dword ptr [EBX + 0x24]
0x80b197b: ADD EAX, ESI
0x80b197d: SUB ESP, 0xc
0x80b1980: PUSH EAX
0x80b1981: CALL 0x0808c0b8
0x80b1986: MOV EDX, dword ptr [EBP + -0x10]
0x80b1989: INC EDI
0x80b198a: MOVSX EAX, word ptr [EDX + 0xe]
0x80b198e: ADD ESP, 0x10
0x80b1991: ADD ESI, 0x18
0x80b1994: CMP EAX, EDI
0x80b1996: JG 0x080b1978
0x80b1998: JMP 0x080b18eb
0x80b199d: SUB ESP, 0x8
0x80b19a0: LEA EAX, [ESI + 0x34]
0x80b19a3: PUSH EAX
0x80b19a4: PUSH ESI
0x80b19a5: CALL 0x080965a4
0x80b19aa: MOV EAX, [0x0860ae64]
0x80b19af: ADD ESP, 0x10
0x80b19b2: TEST EAX, EAX
0x80b19b4: JNZ 0x080b19fa
0x80b19b6: SUB ESP, 0xc
0x80b19b9: PUSH ESI
0x80b19ba: CALL 0x080862a8
0x80b19bf: MOV EAX, [0x0860ae64]
0x80b19c4: ADD ESP, 0x10
0x80b19c7: TEST EAX, EAX
0x80b19c9: JZ 0x080b194d
0x80b19cb: CALL 0x080bb0c0
0x80b19d0: JMP 0x080b194d
0x80b19d8: MOV EAX, dword ptr [EBX + 0x2c]
0x80b19db: MOV dword ptr [EDX], EAX
0x80b19dd: JMP 0x080b193e
0x80b19e2: SUB ESP, 0xc
0x80b19e5: PUSH EAX
0x80b19e6: CALL 0x0808bfb4
0x80b19eb: ADD ESP, 0x10
0x80b19ee: MOV dword ptr [EBX + 0x1c], 0x0
0x80b19f5: JMP 0x080b18cf
0x80b19fa: CALL 0x080bb0ac
0x80b19ff: JMP 0x080b19b6
```

## 本轮：vm_hookCallbackWhenReturn — procedure_pseudo_code

Evidence ID：`ev_4ff4d26fb7db2a562d3076f6865059fbb7f0d9887bb546f7044f18aaafdda955`

```c

void vm_hookCallbackWhenReturn(_func_void_void_ptr *pReturnCallback,void *pCookie)

{
  uint *puVar1;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  
  pVVar3 = vm_funContext;
  iVar2 = vm_funCp;
  vm_funContext[vm_funCp].returnCallback = pReturnCallback;
  puVar1 = &pVVar3[iVar2].flags;
  *puVar1 = *puVar1 | 0x80;
  pVVar3[iVar2].pCookie = pCookie;
  return;
}


```

## 本轮：Vm_Object_T — inspect_native_data_type

Evidence ID：`ev_52399907a9b2295d01e12ab52c5c933e0df9a5ce409ef7e5826191817dbe6ec4`

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

## 本轮：vm_callInInitFun — procedure_callers

Evidence ID：`ev_532b40a2912aa0075bd6feae89ca27ecba4c13cb04f28a6806eabe11d8501948`

```json
[
  "0x80f8cbc",
  "0x80f8eac",
  "0x80f90b4"
]
```

## 本轮：vm_destructAllObjects — procedure_pseudo_code

Evidence ID：`ev_542620fbbbb8fabe07f6b3f0e1f5f73d33a239f5aa991ca0d0a8103028909e8d`

```c

/* WARNING: Unknown calling convention */

void vm_destructAllObjects(int whenShutdown)

{
  Vm_Object *pVVar1;
  Vm_Program_T *pVVar2;
  Vm_Value *pVVar3;
  Vm_Object_T **ppVVar4;
  Vm_Object_T *pVVar5;
  int iVar6;
  int iVar7;
  Vm_Program_T *pProgram;
  int i;
  Vm_Object_T *pNext;
  
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)] */
  if (vm_objectHashTable != (Vm_Object_T **)0x0) {
    i = 0;
    ppVVar4 = vm_objectHashTable;
    do {
      pVVar1 = ppVVar4[i];
      while (pVVar5 = pVVar1, pVVar5 != (Vm_Object_T *)0x0) {
        pVVar1 = pVVar5->pNextHashObject;
        vm_makeObjectAlone(pVVar5);
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
        if (pVVar5->pBindValue != (Vm_Value_T *)0x0) {
          vm_unrefValue(pVVar5->pBindValue);
          pVVar5->pBindValue = (Vm_Value *)0x0;
        }
        pVVar3 = pVVar5->pObjectValues;
        ppVVar4 = vm_objectHashTable;
        if (pVVar3 != (Vm_Value *)0x0) {
          pVVar2 = pVVar5->pProgram;
          iVar7 = 0;
          iVar6 = 0;
          if (0 < pVVar2->maxVariableNo) {
            while( true ) {
              vm_destructStaticValue((Vm_Value_T *)((int)&pVVar3->ref + iVar6));
              iVar7 = iVar7 + 1;
              iVar6 = iVar6 + 0x18;
              if (pVVar2->maxVariableNo <= iVar7) break;
              pVVar3 = pVVar5->pObjectValues;
            }
          }
          pdb_freeMemory(pVVar5->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",
                         0x1ce);
          pVVar5->pObjectValues = (Vm_Value *)0x0;
          ppVVar4 = vm_objectHashTable;
        }
      }
      i = i + 1;
    } while (i < 0x4000);
    i = 0;
    do {
      pVVar5 = ppVVar4[i];
      while (pVVar5 != (Vm_Object_T *)0x0) {
        pVVar1 = pVVar5->pNextHashObject;
        vm_destructObject(pVVar5);
        pVVar5 = pVVar1;
      }
      i = i + 1;
      ppVVar4 = vm_objectHashTable;
    } while (i < 0x4000);
  }
  return;
}


```

## 本轮：vm_freeObjectsInDestructingList — procedure_pseudo_code

Evidence ID：`ev_58e41fc7fd79b8820fddd5d3bb13ce115b1663a228e53fd381f6b10c45480218`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_freeObjectsInDestructingList(void)

{
  Vm_Program_T *pProgram;
  Vm_Object_T *pObject;
  int iVar1;
  
  do {
    pObject = pDestructingList;
    if (pDestructingList == (Vm_Object_T *)0x0) {
      return;
    }
    while( true ) {
      pDestructingList = pObject->pNext;
      pObject->pNext = (Vm_Object *)0x0;
      vm_destructObject(pObject);
      iVar1 = pObject->ref + -1;
      pObject->ref = iVar1;
      if ((0 < iVar1) || ((pObject->flags & 0x8000) == 0)) break;
      pProgram = pObject->pProgram;
      iVar1 = pProgram->ref + -1;
      pProgram->ref = iVar1;
      if (iVar1 < 1) {
        vm_freeCodeSegment(pProgram,&pProgram->initCodeSegment);
        if (vm_gatheringCallStackInfo != 0) {
          vm_enterPPWholeCriticalSection();
        }
        vm_destructProgramHead(pProgram);
        if (vm_gatheringCallStackInfo != 0) {
          vm_leavePPWholeCriticalSection();
        }
      }
      pdb_freeMemory(pObject,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1f5);
      pObject = pDestructingList;
      if (pDestructingList == (Vm_Object_T *)0x0) {
        return;
      }
    }
  } while( true );
}


```

## 本轮：vm_allocateObject — procedure_assembly

Evidence ID：`ev_6b2366257b77aa664004a71adc8eb5f4406467f408f512967e4de1249411e062`

```asm
0x80b1d08: PUSH EBP
0x80b1d09: MOV EBP, ESP
0x80b1d0b: PUSH EDI
0x80b1d0c: PUSH ESI
0x80b1d0d: PUSH EBX
0x80b1d0e: SUB ESP, 0xc
0x80b1d11: PUSH 0x406
0x80b1d16: PUSH 0x82b0e28
0x80b1d1b: PUSH 0x82a376b
0x80b1d20: PUSH 0x40
0x80b1d22: MOV dword ptr [EBP + -0x10], EAX
0x80b1d25: MOV ESI, EDX
0x80b1d27: CALL 0x080ce784
0x80b1d2c: MOV EBX, EAX
0x80b1d2e: ADD ESP, 0x10
0x80b1d31: XOR EAX, EAX
0x80b1d33: TEST EBX, EBX
0x80b1d35: JZ 0x080b1e27
0x80b1d3b: CLD
0x80b1d3c: MOV ECX, 0x10
0x80b1d41: MOV EDI, EBX
0x80b1d43: STOSD.REP ES:EDI
0x80b1d45: MOV EAX, dword ptr [EBP + -0x10]
0x80b1d48: TEST ESI, ESI
0x80b1d4a: MOV dword ptr [EBX + 0x20], EAX
0x80b1d4d: MOV dword ptr [EBX + 0x8], ESI
0x80b1d50: MOV dword ptr [EBX + 0x10], 0xff
0x80b1d57: MOV dword ptr [EBX + 0x14], 0xff
0x80b1d5e: JNZ 0x080b1e30
0x80b1d64: MOV dword ptr [EBX + 0xc], 0x1000
0x80b1d6b: MOV EDX, dword ptr [EBP + -0x10]
0x80b1d6e: MOV EAX, dword ptr [EDX + 0x4]
0x80b1d71: MOVZX EAX, word ptr [EAX + 0x8]
0x80b1d75: ADD EAX, ESI
0x80b1d77: AND EAX, 0x3fff
0x80b1d7c: MOV dword ptr [EBX + 0x30], 0x0
0x80b1d83: MOV dword ptr [EBX + 0x38], 0x0
0x80b1d8a: MOV dword ptr [EBX + 0x34], 0x0
0x80b1d91: MOV dword ptr [EBX + 0x3c], 0x0
0x80b1d98: MOV ECX, dword ptr [0x08609a44]
0x80b1d9e: MOV EDX, dword ptr [ECX + EAX*0x4]
0x80b1da1: MOV dword ptr [EBX + 0x2c], EDX
0x80b1da4: MOV EDX, dword ptr [EBP + -0x10]
0x80b1da7: MOV dword ptr [ECX + EAX*0x4], EBX
0x80b1daa: MOV AX, word ptr [EDX + 0xe]
0x80b1dae: TEST AX, AX
0x80b1db1: JLE 0x080b1e3c
0x80b1db7: SUB ESP, 0xc
0x80b1dba: CWDE
0x80b1dbb: PUSH EAX
0x80b1dbc: CALL 0x0808a088
0x80b1dc1: ADD ESP, 0x10
0x80b1dc4: TEST EAX, EAX
0x80b1dc6: MOV dword ptr [EBX + 0x24], EAX
0x80b1dc9: JNZ 0x080b1e20
0x80b1dcb: MOV EAX, dword ptr [EBX + 0x20]
0x80b1dce: MOV EAX, dword ptr [EAX + 0x4]
0x80b1dd1: MOVZX EAX, word ptr [EAX + 0x8]
0x80b1dd5: ADD EAX, dword ptr [EBX + 0x8]
0x80b1dd8: AND EAX, 0x3fff
0x80b1ddd: MOV EDX, dword ptr [0x08609a44]
0x80b1de3: LEA EDX, [EDX + EAX*0x4]
0x80b1de6: MOV EAX, dword ptr [EDX]
0x80b1de8: JMP 0x080b1df6
0x80b1dec: CMP EAX, EBX
0x80b1dee: JZ 0x080b1e45
0x80b1df0: LEA EDX, [EAX + 0x2c]
0x80b1df3: MOV EAX, dword ptr [EAX + 0x2c]
0x80b1df6: TEST EAX, EAX
0x80b1df8: JNZ 0x080b1dec
0x80b1dfa: PUSH 0x437
0x80b1dff: PUSH 0x82b0e28
0x80b1e04: PUSH 0x82a376b
0x80b1e09: PUSH EBX
0x80b1e0a: CALL 0x080ceb94
0x80b1e0f: MOV dword ptr [ESP], 0x82b0f9c
0x80b1e16: CALL 0x08097b24
0x80b1e1b: ADD ESP, 0x10
0x80b1e1e: MOV ESI, ESI
0x80b1e20: MOV EAX, dword ptr [EBP + -0x10]
0x80b1e23: INC dword ptr [EAX]
0x80b1e25: MOV EAX, EBX
0x80b1e27: LEA ESP, [EBP + -0xc]
0x80b1e2a: POP EBX
0x80b1e2b: POP ESI
0x80b1e2c: POP EDI
0x80b1e2d: LEAVE
0x80b1e2e: RET
0x80b1e30: MOV dword ptr [EBX + 0xc], 0x1001
0x80b1e37: JMP 0x080b1d6b
0x80b1e3c: MOV dword ptr [EBX + 0x24], 0x0
0x80b1e43: JMP 0x080b1e20
0x80b1e45: MOV EAX, dword ptr [EBX + 0x2c]
0x80b1e48: MOV dword ptr [EDX], EAX
0x80b1e4a: JMP 0x080b1dfa
```

## 本轮：vm_lookupObject — procedure_pseudo_code

Evidence ID：`ev_6bd6f758aac4e15cc09716b4e96079841eab2a3ccac331ccd49911785170d50a`

```c

Vm_Object_T * vm_lookupObject(Vm_String_T *pStrObjectName,uint oid)

{
  char cVar1;
  Vm_Object_T *pVVar2;
  int iVar3;
  Vm_String_T *pVVar4;
  uint uVar5;
  char *pcVar6;
  char regularName [256];
  
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_String_T * pSharedString@[DW_OP_reg1(ECX)]
                       Unresolved local var: int idx@[DW_OP_reg2(EDX)] */
  if (pStrObjectName != (Vm_String_T *)0x0) {
    pVVar4 = pStrObjectName;
    if (((pStrObjectName->attrib & 1) != 0) ||
       (pVVar4 = vm_findSharedString(pStrObjectName->ptr,pStrObjectName->len),
       pVVar4 != (Vm_String_T *)0x0)) {
      pVVar2 = vm_objectHashTable[(ushort)((short)oid + pVVar4->hash) & 0x3fff];
      while (pVVar2 != (Vm_Object_T *)0x0) {
        if ((pVVar2->pProgram->strName == pVVar4) && (pVVar2->oid == oid)) {
          if ((pVVar2->flags & 0xc000) == 0) {
            return pVVar2;
          }
          pVVar2 = pVVar2->pNextHashObject;
        }
        else {
          pVVar2 = pVVar2->pNextHashObject;
        }
      }
    }
    iVar3 = vm_deriveFileName(pStrObjectName->ptr,".c",regularName,0x100);
    if (iVar3 != 0) {
      uVar5 = 0xffffffff;
      pcVar6 = regularName;
      do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar6;
        pcVar6 = pcVar6 + 1;
      } while (cVar1 != '\0');
      pVVar4 = vm_findSharedString(regularName,~uVar5 - 1);
      if ((pVVar4 != (Vm_String_T *)0x0) && (pVVar4 != pStrObjectName)) {
        pVVar2 = vm_objectHashTable[(ushort)((short)oid + pVVar4->hash) & 0x3fff];
        while (pVVar2 != (Vm_Object_T *)0x0) {
          if ((pVVar2->pProgram->strName == pVVar4) && (pVVar2->oid == oid)) {
            if ((pVVar2->flags & 0xc000) == 0) {
              return pVVar2;
            }
            pVVar2 = pVVar2->pNextHashObject;
          }
          else {
            pVVar2 = pVVar2->pNextHashObject;
          }
        }
      }
    }
  }
  return (Vm_Object_T *)0x0;
}


```

## 本轮：0x80ae0e8 — procedure_pseudo_code

Evidence ID：`ev_86ea360aa0ccc20146300d4fada064d9d97bd8cc712b8e5d54de240c1cc4b0fd`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_shutdownScheduler(void)

{
  uint *puVar1;
  char *pcVar2;
  uint uVar3;
  Vm_PCB *pVVar4;
  Vm_PCB_T *pVVar5;
  int iVar6;
  Vm_Timer_T *ptr;
  Vm_Timer_T **ppVVar7;
  Vm_PCB_T *pVVar8;
  Vm_Timer_T *pVVar9;
  Vm_Task_Id_T VVar10;
  Vm_Tick_T VVar11;
  int iVar12;
  Vm_Tick_T VVar13;
  Vm_PCB_T *pPCB;
  uint uVar14;
  Vm_Oid_T oid;
  uint uVar15;
  int local_1c;
  uint i;
  
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint tick@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint k@[DW_OP_reg7(EDI)] */
  _vm_schedulerStatus = -1;
  VVar10 = _vm_getCurrentTaskId();
  if (VVar10 != _vm_mainTaskId) {
    _vm_requestStop = 1;
    while (_vm_alreadyStop == 0) {
      vm_releaseVmMutex();
      _vm_sleep(10);
      vm_getVmMutex();
    }
  }
  _vm_mainTaskId = 0;
  _vm_deleteSemaphore(_vm_endFlagSemId);
  _vm_endFlagSemId = (Vm_Semaphore_Id_T)0x0;
  _vm_deleteEvent(_vm_mainLoopEventId);
  _vm_mainLoopEventId = (Vm_Event_Id_T)0x0;
  VVar11 = _vm_getCurrentTick();
  do {
                    /* Unresolved local var: int ret@[DW_OP_reg0(EAX)] */
    _vm_isInSchduler = 1;
    _vm_schedulerRuntimeState = 1;
    iVar12 = vm_scheduleProcessInternal();
    _vm_schedulerRuntimeState = 0;
    _vm_isInSchduler = 0;
    if (iVar12 != 1) goto LAB_080ae1cc;
    VVar13 = _vm_getCurrentTick();
  } while (VVar13 - VVar11 < 1000);
  ptr = _vm_pInvokeList;
  pVVar9 = (Vm_Timer_T *)0x0;
  ppVVar7 = _vm_ppLastInvokeNode;
  uVar15 = vm_maxPendObject;
  if (_vm_pTimerList != (Vm_Timer_T *)0x0) {
    do {
      vm_removeTimerFromTimerList(_vm_pTimerList->type,(_vm_pTimerList->v).pIdent);
LAB_080ae1cc:
    } while (_vm_pTimerList != (Vm_Timer_T *)0x0);
    ptr = _vm_pInvokeList;
    pVVar9 = (Vm_Timer_T *)0x0;
    ppVVar7 = _vm_ppLastInvokeNode;
    uVar15 = vm_maxPendObject;
  }
  while (_vm_pTimerList = pVVar9, _vm_ppLastInvokeNode = ppVVar7, vm_maxPendObject = uVar15,
        ptr != (Vm_Timer_T *)0x0) {
    _vm_pInvokeList = ptr->pNext;
    _vm_ppLastInvokeNode = &_vm_pInvokeList;
    if (_vm_pInvokeList != (Vm_Timer *)0x0) {
      _vm_ppLastInvokeNode = ppVVar7;
    }
    if (ptr->pOwnerObject == (Vm_Object_T *)0x0) {
      iVar12 = ptr->type;
      iVar6 = iVar12 + -2;
      if (iVar12 != 2) goto LAB_080ae497;
LAB_080ae4cc:
      vm_unrefValue((ptr->v).pValue);
    }
    else {
      vm_unrefObject(ptr->pOwnerObject);
      iVar12 = ptr->type;
      iVar6 = iVar12 + -2;
      if (iVar6 == 0) goto LAB_080ae4cc;
LAB_080ae497:
      if (SBORROW4(iVar12,2) == iVar6 < 0) {
        if (iVar12 != 3) {
LAB_080ae5a6:
          vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                     "Bad type of timer when delete all timer.\n",
                     "/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x11f6);
          _vm_endProcess(0);
          do {
                    /* WARNING: Do nothing block with infinite loop */
          } while( true );
        }
        if (ptr->finishHook != (Vm_Timer_Finish_Hook_T)0x0) {
          (*ptr->finishHook)((Vm_PCB_T *)0x0,ptr->pFinishCookie);
        }
      }
      else {
        if (iVar12 != 1) goto LAB_080ae5a6;
        puVar1 = &((ptr->v).pPCB)->flags;
        *puVar1 = *puVar1 & 0xffffff7f;
      }
    }
    pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0xc42);
    ptr = _vm_pInvokeList;
    pVVar9 = _vm_pTimerList;
    ppVVar7 = _vm_ppLastInvokeNode;
    uVar15 = vm_maxPendObject;
  }
                    /* Unresolved local var: Vm_Oid_T i@[DW_OP_reg3(EBX)] */
  oid = 0;
  _vm_pInvokeList = (Vm_Timer_T *)0x0;
  if (uVar15 != 0) {
    do {
      if (_vmObject[oid] != (void *)0x0) {
        vm_deletePendObject(oid);
        uVar15 = vm_maxPendObject;
      }
      oid = oid + 1;
    } while (oid < uVar15);
  }
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg1(ECX)]
                       Unresolved local var: uint i@[DW_OP_reg6(ESI)] */
  uVar15 = 0;
  if (vm_maxProcess != 0) {
    iVar12 = 0;
    uVar14 = vm_maxProcess;
    do {
      while( true ) {
        pVVar8 = _vm_pActiveProcess;
        pPCB = (Vm_PCB_T *)(_vm_allPCB->name + iVar12);
        uVar3 = pPCB->flags;
        if (((uVar3 & 1) == 0) || ((uVar3 & 0x20) != 0)) break;
        if ((uVar3 & 6) == 0) {
          pVVar4 = pPCB->pPrev;
          pVVar4->pNext = pPCB->pNext;
          _vm_currentActiveProcess = _vm_currentActiveProcess + -1;
          pVVar5 = pPCB->pNext;
          pVVar5->pPrev = pVVar4;
          if ((pPCB == pVVar8) && (_vm_pActiveProcess = (Vm_PCB_T *)0x0, pVVar5 != pPCB)) {
            _vm_pActiveProcess = pVVar5;
          }
          pPCB->pNext = (Vm_PCB *)0x0;
          pPCB->pPrev = (Vm_PCB *)0x0;
        }
        pPCB->returnCode = -3;
        vm_deleteProcess(pPCB);
        uVar15 = uVar15 + 1;
        iVar12 = iVar12 + 0x1ec;
        uVar14 = vm_maxProcess;
        if (vm_maxProcess <= uVar15) goto LAB_080ae2b0;
      }
      uVar15 = uVar15 + 1;
      iVar12 = iVar12 + 0x1ec;
    } while (uVar15 < uVar14);
  }
LAB_080ae2b0:
  vm_freeObjectsInDestructingList();
  i = 0;
  if (vm_maxProcess != 0) {
    local_1c = 0;
    do {
      pVVar8 = _vm_allPCB;
      uVar15 = 0;
      if (*(int *)(_vm_allPCB->lastError + local_1c + -0x60) != 0) {
        iVar12 = 0;
        do {
          *(undefined1 *)(iVar12 + 4 + *(int *)(pVVar8->lastError + local_1c + -0x5c)) = 0;
          uVar15 = uVar15 + 1;
          vm_destructStaticValue
                    ((Vm_Value_T *)(*(int *)(pVVar8->lastError + local_1c + -0x5c) + iVar12));
          iVar12 = iVar12 + 0x18;
        } while (uVar15 < *(uint *)(pVVar8->lastError + local_1c + -0x60));
      }
      if (*(int *)(pVVar8->lastError + local_1c + -0x5c) != 0) {
        pdb_freeMemory(*(void **)(pVVar8->lastError + local_1c + -0x5c),"VM",
                       "/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x1ce);
        pcVar2 = pVVar8->lastError + local_1c + -0x5c;
        pcVar2[0] = '\0';
        pcVar2[1] = '\0';
        pcVar2[2] = '\0';
        pcVar2[3] = '\0';
        pcVar2 = pVVar8->lastError + local_1c + -0x60;
        pcVar2[0] = '\0';
        pcVar2[1] = '\0';
        pcVar2[2] = '\0';
        pcVar2[3] = '\0';
      }
      if (*(int *)(pVVar8->lastError + local_1c + -0x48) != 0) {
        pdb_freeMemory(*(void **)(pVVar8->lastError + local_1c + -0x48),"VM",
                       "/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x1d6);
        pcVar2 = pVVar8->lastError + local_1c + -0x48;
        pcVar2[0] = '\0';
        pcVar2[1] = '\0';
        pcVar2[2] = '\0';
        pcVar2[3] = '\0';
        pcVar2 = pVVar8->lastError + local_1c + -0x4c;
        pcVar2[0] = '\0';
        pcVar2[1] = '\0';
        pcVar2[2] = '\0';
        pcVar2[3] = '\0';
      }
      if (*(int *)(pVVar8->lastError + local_1c + -0x3c) != 0) {
        pdb_freeMemory(*(void **)(pVVar8->lastError + local_1c + -0x3c),"VM",
                       "/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x1de);
        pcVar2 = pVVar8->lastError + local_1c + -0x3c;
        pcVar2[0] = '\0';
        pcVar2[1] = '\0';
        pcVar2[2] = '\0';
        pcVar2[3] = '\0';
        pcVar2 = pVVar8->lastError + local_1c + -0x40;
        pcVar2[0] = '\0';
        pcVar2[1] = '\0';
        pcVar2[2] = '\0';
        pcVar2[3] = '\0';
      }
      i = i + 1;
      local_1c = local_1c + 0x1ec;
    } while (i < vm_maxProcess);
  }
                    /* Unresolved local var: uint i@[DW_OP_reg6(ESI)] */
  uVar15 = 0;
  pdb_freeMemory(_vmObject,"VM","/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x1e5);
  _vmObject = (void **)0x0;
  if (vm_maxProcess != 0) {
    iVar12 = 0;
    do {
      _vm_deleteSemaphore(*(Vm_Semaphore_Id_T *)(_vm_allPCB->lastError + iVar12 + -0x38));
      uVar15 = uVar15 + 1;
      _vm_deleteSemaphore(*(Vm_Semaphore_Id_T *)(_vm_allPCB->lastError + iVar12 + -0x34));
      iVar12 = iVar12 + 0x1ec;
    } while (uVar15 < vm_maxProcess);
  }
  pdb_freeMemory(_vm_allPCB,"VM","/home/cc/xxxx/gameshell/src/vm/vm_scheduler.c",0x1ec);
  _vm_allPCB = (Vm_PCB_T *)0x0;
  _vm_freePCBList = (Vm_PCB_T *)0x0;
  _vm_currentActiveProcess = 0;
  _vm_pPendingProcess = (Vm_PCB_T *)0x0;
  _vm_pActiveProcess = (Vm_PCB_T *)0x0;
  _vm_pTimerList = (Vm_Timer_T *)0x0;
  _vm_ppLastInvokeNode = (Vm_Timer_T **)0x0;
  _vm_lastTick = 0;
  _vm_schedulerStatus = 0;
  return;
}


```

## 本轮：vm_unrefObject — procedure_pseudo_code

Evidence ID：`ev_87303180daaae04da5ace6f0b3bb43f14f6414ed173f8c4b9b57a90db34f587e`

```c

void vm_unrefObject(Vm_Object_T *pObject)

{
  Vm_Program_T *pProgram;
  int iVar1;
  
  iVar1 = pObject->ref + -1;
  pObject->ref = iVar1;
  if ((iVar1 < 1) && ((pObject->flags & 0x8000) != 0)) {
    pProgram = pObject->pProgram;
    iVar1 = pProgram->ref + -1;
    pProgram->ref = iVar1;
    if (iVar1 < 1) {
      vm_freeCodeSegment(pProgram,&pProgram->initCodeSegment);
      if (vm_gatheringCallStackInfo != 0) {
        vm_enterPPWholeCriticalSection();
      }
      vm_destructProgramHead(pProgram);
      if (vm_gatheringCallStackInfo != 0) {
        vm_leavePPWholeCriticalSection();
      }
    }
    pdb_freeMemory(pObject,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1f5);
    return;
  }
  return;
}


```

## 本轮：vm_f_load_object — procedure_pseudo_code

Evidence ID：`ev_87a2b9092c28e539d262db132e5ad62f159b243caf996e464ef224049039a52e`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_load_object(void)

{
  short sVar1;
  Vm_Value_T *pVVar2;
  Vm_Object_T *pToObject;
  int iVar3;
  uint contentLen;
  Vm_Object_T *pVVar4;
  uchar *content;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pValue2@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue3@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)]
                       Unresolved local var: int contentLen@[DW_OP_reg7(EDI)] */
  contentLen = 0;
  content = (uchar *)0x0;
  if (vm_currentContext->argNum < 4) {
    iVar3 = vm_currentContext->argNum;
  }
  else {
    vm_error("Too many arguments(%d) to %s.\n",vm_currentContext->argNum,"load_object");
    iVar3 = vm_currentContext->argNum;
  }
  if (1 < iVar3) {
    pVVar2 = vm_getArgument(1,0xff);
    iVar3 = vm_isZero(pVVar2);
    if ((iVar3 == 0) && (pVVar2->type != '\a')) {
      vm_error("Expected mapping for argument 2, constructing parameter.\n");
    }
    if (2 < vm_currentContext->argNum) {
      pVVar2 = vm_getArgument(2,0xff);
      if (pVVar2->type == '\x04') {
        content = (uchar *)((pVVar2->v).string)->ptr;
        contentLen = ((pVVar2->v).string)->len;
      }
      else if (pVVar2->type == '\x05') {
        content = ((pVVar2->v).buffer)->ptr;
        contentLen = ((pVVar2->v).buffer)->realSize;
      }
      else {
        vm_error("Expected string | buffer ");
      }
    }
  }
  pVVar2 = vm_getArgument(0,4);
  pToObject = vm_lookupObject((pVVar2->v).string,0);
  if (pToObject == (Vm_Object_T *)0x0) {
    pVVar4 = pToObject;
    pToObject = vm_loadObject(((pVVar2->v).string)->ptr,content,contentLen);
    if (pToObject == (Vm_Object_T *)0x0) {
      vm_error("Can not load into object.\n",content,contentLen,pVVar4);
    }
    sVar1 = pToObject->pProgram->maxConsititutionNo;
    pToObject->flags = pToObject->flags | 0x1000;
    pToObject->initStep = -sVar1;
  }
  else {
    if (pToObject->flags != 0x800) {
      vm_popNStack(vm_currentContext->argNum);
      vm_pushObject(pToObject);
      return;
    }
                    /* Unresolved local var: Vm_Index_T consititutionToInit@[???] */
    pToObject->initStep = -pToObject->pProgram->maxConsititutionNo;
    pToObject->flags = 0x1800;
  }
                    /* Unresolved local var: Vm_Index_T consititutionToInit@[???] */
  vm_newThreadCatch2(vm_f_construct_object_failed,pToObject);
  vm_callInInitFun(pToObject,*pToObject->pProgram->initOrderList);
  pToObject->initStep = pToObject->initStep + 1;
  vm_hookCallbackWhenReturn(vm_f_construct_object_ok1,pToObject);
  vm_ctrlFlags = vm_ctrlFlags | 0x10;
  return;
}


```

## 本轮：vm_f_construct_object_ok2 — procedure_pseudo_code

Evidence ID：`ev_8de0ae51a3c83e41247c459b8b8838c17d0a9909cfed731e519d88dd05693e02`

```c

void vm_f_construct_object_ok2(void *pCookie)

{
  vm_endThreadCatch();
  vm_popStack();
  vm_f_construct_object2(pCookie);
  return;
}


```

## 本轮：vm_f_destruct_object_crash — procedure_pseudo_code

Evidence ID：`ev_8f13fecfa2ec8a0f4f108c3fed82f6a97666cedb45255c04a339049bafd5e83e`

```c

void vm_f_destruct_object_crash(Vm_PCB_T *pPCB,void *pCookie)

{
                    /* Unresolved local var: Vm_Object_T * pObject@[???] */
  *(uint *)((int)pCookie + 0xc) = *(uint *)((int)pCookie + 0xc) & 0xffffdfff;
  vm_destructObject(pCookie);
  return;
}


```

## 本轮：vm_f_clone_object — procedure_assembly

Evidence ID：`ev_95ee1759e60a50e3626d6f77afede06cdcaf36c76b1376d2b2b731116b19213f`

```asm
0x80f8eac: PUSH EBP
0x80f8ead: MOV EBP, ESP
0x80f8eaf: PUSH ESI
0x80f8eb0: PUSH EBX
0x80f8eb1: MOV EAX, [0x086619d8]
0x80f8eb6: CMP dword ptr [EAX + 0x4], 0x2
0x80f8eba: JG 0x080f8f8c
0x80f8ec0: MOV EDX, dword ptr [EAX + 0x4]
0x80f8ec3: TEST EDX, EDX
0x80f8ec5: JLE 0x080f8fb4
0x80f8ecb: CMP dword ptr [EAX + 0x4], 0x1
0x80f8ecf: JG 0x080f8fd4
0x80f8ed5: MOV EAX, [0x08661960]
0x80f8eda: CMP dword ptr [0x086619cc], EAX
0x80f8ee0: JGE 0x080f8ff4
0x80f8ee6: CALL 0x08090604
0x80f8eeb: MOV ESI, EAX
0x80f8eed: MOVZX EAX, byte ptr [EAX + 0x4]
0x80f8ef1: CMP EAX, 0x4
0x80f8ef4: JZ 0x080f9018
0x80f8efa: CMP EAX, 0xa
0x80f8efd: JZ 0x080f9080
0x80f8f03: SUB ESP, 0xc
0x80f8f06: PUSH 0x82bfce0
0x80f8f0b: CALL 0x08097b24
0x80f8f10: ADD ESP, 0x10
0x80f8f13: SUB ESP, 0xc
0x80f8f16: PUSH dword ptr [ESI + 0x8]
0x80f8f19: CALL 0x080b1e4c
0x80f8f1e: ADD ESP, 0x10
0x80f8f21: TEST EAX, EAX
0x80f8f23: MOV EBX, EAX
0x80f8f25: JZ 0x080f9092
0x80f8f2b: MOV EAX, dword ptr [ESI + 0x8]
0x80f8f2e: MOV dword ptr [EBX + 0x3c], EAX
0x80f8f31: INC dword ptr [EAX]
0x80f8f33: MOV EAX, dword ptr [EBX + 0x20]
0x80f8f36: MOV AX, word ptr [EAX + 0x14]
0x80f8f3a: SUB ESP, 0x8
0x80f8f3d: NEG EAX
0x80f8f3f: MOV word ptr [EBX + 0x18], AX
0x80f8f43: OR dword ptr [EBX + 0xc], 0x1000
0x80f8f4a: PUSH EBX
0x80f8f4b: PUSH 0x80f93b0
0x80f8f50: CALL 0x080aaef0
0x80f8f55: POP EAX
0x80f8f56: MOV EAX, dword ptr [EBX + 0x20]
0x80f8f59: MOV EAX, dword ptr [EAX + 0x1c]
0x80f8f5c: POP EDX
0x80f8f5d: MOVSX EAX, word ptr [EAX]
0x80f8f60: PUSH EAX
0x80f8f61: PUSH EBX
0x80f8f62: CALL 0x08094258
0x80f8f67: POP ESI
0x80f8f68: POP EAX
0x80f8f69: INC word ptr [EBX + 0x18]
0x80f8f6d: PUSH EBX
0x80f8f6e: PUSH 0x80f9250
0x80f8f73: CALL 0x08094e70
0x80f8f78: OR dword ptr [0x086619d4], 0x10
0x80f8f7f: ADD ESP, 0x10
0x80f8f82: LEA ESP, [EBP + -0x8]
0x80f8f85: POP EBX
0x80f8f86: POP ESI
0x80f8f87: LEAVE
0x80f8f88: RET
0x80f8f8c: PUSH ECX
0x80f8f8d: PUSH 0x82bedef
0x80f8f92: PUSH dword ptr [EAX + 0x4]
0x80f8f95: PUSH 0x82a82c4
0x80f8f9a: CALL 0x08097b24
0x80f8f9f: MOV EAX, [0x086619d8]
0x80f8fa4: MOV EDX, dword ptr [EAX + 0x4]
0x80f8fa7: ADD ESP, 0x10
0x80f8faa: TEST EDX, EDX
0x80f8fac: JG 0x080f8ecb
0x80f8fb2: MOV ESI, ESI
0x80f8fb4: SUB ESP, 0xc
0x80f8fb7: PUSH 0x82bf460
0x80f8fbc: CALL 0x08097b24
0x80f8fc1: MOV EAX, [0x086619d8]
0x80f8fc6: ADD ESP, 0x10
0x80f8fc9: CMP dword ptr [EAX + 0x4], 0x1
0x80f8fcd: JLE 0x080f8ed5
0x80f8fd3: NOP
0x80f8fd4: SUB ESP, 0x8
0x80f8fd7: PUSH 0x7
0x80f8fd9: PUSH 0x1
0x80f8fdb: CALL 0x0808a364
0x80f8fe0: MOV EAX, [0x08661960]
0x80f8fe5: ADD ESP, 0x10
0x80f8fe8: CMP dword ptr [0x086619cc], EAX
0x80f8fee: JL 0x080f8ee6
0x80f8ff4: SUB ESP, 0xc
0x80f8ff7: PUSH 0x82a9240
0x80f8ffc: CALL 0x08097b24
0x80f9001: ADD ESP, 0x10
0x80f9004: CALL 0x08090604
0x80f9009: MOV ESI, EAX
0x80f900b: MOVZX EAX, byte ptr [EAX + 0x4]
0x80f900f: CMP EAX, 0x4
0x80f9012: JNZ 0x080f8efa
0x80f9018: CALL 0x08090604
0x80f901d: MOV EBX, EAX
0x80f901f: MOV AL, byte ptr [EAX + 0x4]
0x80f9022: CMP AL, 0x4
0x80f9024: JZ 0x080f9042
0x80f9026: SUB ESP, 0xc
0x80f9029: MOVZX EAX, AL
0x80f902c: PUSH EAX
0x80f902d: CALL 0x0808a13c
0x80f9032: POP ECX
0x80f9033: POP ESI
0x80f9034: PUSH EAX
0x80f9035: PUSH 0x82bf11c
0x80f903a: CALL 0x08097b24
0x80f903f: ADD ESP, 0x10
0x80f9042: SUB ESP, 0xc
0x80f9045: PUSH dword ptr [EBX + 0x8]
0x80f9048: CALL 0x080b114c
0x80f904d: MOV EBX, EAX
0x80f904f: ADD ESP, 0x10
0x80f9052: CALL 0x0809061c
0x80f9057: TEST EBX, EBX
0x80f9059: JZ 0x080f90a7
0x80f905b: CMP dword ptr [EBX + 0xc], 0x800
0x80f9062: JZ 0x080f90a7
0x80f9064: SUB ESP, 0xc
0x80f9067: PUSH EBX
0x80f9068: CALL 0x080912f4
0x80f906d: ADD ESP, 0x10
0x80f9070: CALL 0x08090604
0x80f9075: MOV ESI, EAX
0x80f9077: CMP byte ptr [EAX + 0x4], 0xa
0x80f907b: JMP 0x080f9087
0x80f9080: MOV EAX, dword ptr [ESI + 0x8]
0x80f9083: TEST byte ptr [EAX + 0xd], 0x80
0x80f9087: JZ 0x080f8f13
0x80f908d: JMP 0x080f8f03
0x80f9092: SUB ESP, 0xc
0x80f9095: PUSH 0x82bf0e6
0x80f909a: CALL 0x08097b24
0x80f909f: ADD ESP, 0x10
0x80f90a2: JMP 0x080f8f2b
0x80f90a7: SUB ESP, 0xc
0x80f90aa: PUSH 0x0
0x80f90ac: CALL 0x08090b3c
0x80f90b1: JMP 0x080f906d
```

## 本轮：vm_f_load_object — procedure_assembly

Evidence ID：`ev_99224dc29515f385d867b034821dd999cbb2b622d551f0e8901cae93e6a4dc43`

```asm
0x80f8cbc: PUSH EBP
0x80f8cbd: MOV EBP, ESP
0x80f8cbf: PUSH EDI
0x80f8cc0: PUSH ESI
0x80f8cc1: PUSH EBX
0x80f8cc2: SUB ESP, 0xc
0x80f8cc5: MOV EAX, [0x086619d8]
0x80f8cca: XOR EDI, EDI
0x80f8ccc: CMP dword ptr [EAX + 0x4], 0x3
0x80f8cd0: MOV dword ptr [EBP + -0x10], 0x0
0x80f8cd7: JG 0x080f8d38
0x80f8cd9: CMP dword ptr [EAX + 0x4], 0x1
0x80f8cdd: JG 0x080f8d5c
0x80f8cdf: SUB ESP, 0x8
0x80f8ce2: PUSH 0x4
0x80f8ce4: PUSH 0x0
0x80f8ce6: CALL 0x0808a364
0x80f8ceb: MOV ESI, EAX
0x80f8ced: POP EAX
0x80f8cee: POP EDX
0x80f8cef: PUSH 0x0
0x80f8cf1: PUSH dword ptr [ESI + 0x8]
0x80f8cf4: CALL 0x080b0f2c
0x80f8cf9: ADD ESP, 0x10
0x80f8cfc: TEST EAX, EAX
0x80f8cfe: MOV EBX, EAX
0x80f8d00: JZ 0x080f8e43
0x80f8d06: CMP dword ptr [EAX + 0xc], 0x800
0x80f8d0d: JZ 0x080f8dcc
0x80f8d13: SUB ESP, 0xc
0x80f8d16: MOV EAX, [0x086619d8]
0x80f8d1b: PUSH dword ptr [EAX + 0x4]
0x80f8d1e: CALL 0x08091ab0
0x80f8d23: MOV dword ptr [ESP], EBX
0x80f8d26: CALL 0x080912f4
0x80f8d2b: ADD ESP, 0x10
0x80f8d2e: LEA ESP, [EBP + -0xc]
0x80f8d31: POP EBX
0x80f8d32: POP ESI
0x80f8d33: POP EDI
0x80f8d34: LEAVE
0x80f8d35: RET
0x80f8d38: PUSH ECX
0x80f8d39: PUSH 0x82beece
0x80f8d3e: PUSH dword ptr [EAX + 0x4]
0x80f8d41: PUSH 0x82a82c4
0x80f8d46: CALL 0x08097b24
0x80f8d4b: MOV EAX, [0x086619d8]
0x80f8d50: ADD ESP, 0x10
0x80f8d53: CMP dword ptr [EAX + 0x4], 0x1
0x80f8d57: JLE 0x080f8cdf
0x80f8d59: LEA ESI, [ESI]
0x80f8d5c: SUB ESP, 0x8
0x80f8d5f: PUSH 0xff
0x80f8d64: PUSH 0x1
0x80f8d66: CALL 0x0808a364
0x80f8d6b: MOV dword ptr [ESP], EAX
0x80f8d6e: MOV EBX, EAX
0x80f8d70: CALL 0x0808cad4
0x80f8d75: ADD ESP, 0x10
0x80f8d78: TEST EAX, EAX
0x80f8d7a: JZ 0x080f8e24
0x80f8d80: MOV EAX, [0x086619d8]
0x80f8d85: CMP dword ptr [EAX + 0x4], 0x2
0x80f8d89: JLE 0x080f8cdf
0x80f8d8f: SUB ESP, 0x8
0x80f8d92: PUSH 0xff
0x80f8d97: PUSH 0x2
0x80f8d99: CALL 0x0808a364
0x80f8d9e: MOV DL, byte ptr [EAX + 0x4]
0x80f8da1: ADD ESP, 0x10
0x80f8da4: CMP DL, 0x4
0x80f8da7: JZ 0x080f8e86
0x80f8dad: CMP DL, 0x5
0x80f8db0: JZ 0x080f8e75
0x80f8db6: SUB ESP, 0xc
0x80f8db9: PUSH 0x82bf0b1
0x80f8dbe: CALL 0x08097b24
0x80f8dc3: ADD ESP, 0x10
0x80f8dc6: JMP 0x080f8cdf
0x80f8dcc: MOV EAX, dword ptr [EAX + 0x20]
0x80f8dcf: MOV AX, word ptr [EAX + 0x14]
0x80f8dd3: NEG EAX
0x80f8dd5: MOV word ptr [EBX + 0x18], AX
0x80f8dd9: MOV dword ptr [EBX + 0xc], 0x1800
0x80f8de0: SUB ESP, 0x8
0x80f8de3: PUSH EBX
0x80f8de4: PUSH 0x80f93b0
0x80f8de9: CALL 0x080aaef0
0x80f8dee: POP EDI
0x80f8def: POP EAX
0x80f8df0: MOV EAX, dword ptr [EBX + 0x20]
0x80f8df3: MOV EAX, dword ptr [EAX + 0x1c]
0x80f8df6: MOVSX EAX, word ptr [EAX]
0x80f8df9: PUSH EAX
0x80f8dfa: PUSH EBX
0x80f8dfb: CALL 0x08094258
0x80f8e00: POP ECX
0x80f8e01: POP ESI
0x80f8e02: INC word ptr [EBX + 0x18]
0x80f8e06: PUSH EBX
0x80f8e07: PUSH 0x80f9250
0x80f8e0c: CALL 0x08094e70
0x80f8e11: OR dword ptr [0x086619d4], 0x10
0x80f8e18: ADD ESP, 0x10
0x80f8e1b: LEA ESP, [EBP + -0xc]
0x80f8e1e: POP EBX
0x80f8e1f: POP ESI
0x80f8e20: POP EDI
0x80f8e21: LEAVE
0x80f8e22: RET
0x80f8e24: CMP byte ptr [EBX + 0x4], 0x7
0x80f8e28: JZ 0x080f8d80
0x80f8e2e: SUB ESP, 0xc
0x80f8e31: PUSH 0x82bfca4
0x80f8e36: CALL 0x08097b24
0x80f8e3b: ADD ESP, 0x10
0x80f8e3e: JMP 0x080f8d80
0x80f8e43: PUSH EAX
0x80f8e44: PUSH EDI
0x80f8e45: PUSH dword ptr [EBP + -0x10]
0x80f8e48: MOV EAX, dword ptr [ESI + 0x8]
0x80f8e4b: PUSH dword ptr [EAX + 0xc]
0x80f8e4e: CALL 0x080b1e90
0x80f8e53: ADD ESP, 0x10
0x80f8e56: TEST EAX, EAX
0x80f8e58: MOV EBX, EAX
0x80f8e5a: JZ 0x080f8e97
0x80f8e5c: MOV EAX, dword ptr [EBX + 0x20]
0x80f8e5f: MOV AX, word ptr [EAX + 0x14]
0x80f8e63: NEG EAX
0x80f8e65: OR dword ptr [EBX + 0xc], 0x1000
0x80f8e6c: MOV word ptr [EBX + 0x18], AX
0x80f8e70: JMP 0x080f8de0
0x80f8e75: MOV EAX, dword ptr [EAX + 0x8]
0x80f8e78: MOV EDX, dword ptr [EAX + 0x10]
0x80f8e7b: MOV EDI, dword ptr [EAX + 0xc]
0x80f8e7e: MOV dword ptr [EBP + -0x10], EDX
0x80f8e81: JMP 0x080f8cdf
0x80f8e86: MOV EAX, dword ptr [EAX + 0x8]
0x80f8e89: MOV EDX, dword ptr [EAX + 0xc]
0x80f8e8c: MOV EDI, dword ptr [EAX + 0x10]
0x80f8e8f: MOV dword ptr [EBP + -0x10], EDX
0x80f8e92: JMP 0x080f8cdf
0x80f8e97: SUB ESP, 0xc
0x80f8e9a: PUSH 0x82bf0cb
0x80f8e9f: CALL 0x08097b24
0x80f8ea4: ADD ESP, 0x10
0x80f8ea7: JMP 0x080f8e5c
```

## 本轮：vm_endForBackend — procedure_assembly

Evidence ID：`ev_99ff4df3dda5ea7103a8c70bf55edc9923932a45ea27dd2afbb7aee201f001f7`

```asm
0x8096f44: PUSH EBP
0x8096f45: MOV EBP, ESP
0x8096f47: SUB ESP, 0x8
0x8096f4a: CALL 0x080b1a04
0x8096f4f: CALL 0x080c9f70
0x8096f54: CALL 0x0809f24c
0x8096f59: SUB ESP, 0xc
0x8096f5c: PUSH 0x0
0x8096f5e: CALL 0x080a391c
0x8096f63: LEAVE
0x8096f64: RET
```

## 本轮：vm_f_destruct_object — procedure_pseudo_code

Evidence ID：`ev_9a05fb2e7216b29de976bc84c35804409c4b0391f4af9fdcc03e6e0ec6f9bfdf`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_destruct_object(void)

{
  uchar uVar1;
  Vm_Value *pVVar2;
  Vm_PCB_T *pPCB;
  char *pcVar3;
  Vm_Object_T *object;
  Vm_Buffer *pCookie;
  code *endHook;
  Vm_Buffer *pCookie_00;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Object_T * pObject@[???] */
  if (vm_currentContext->argNum < 2) {
    if (vm_currentContext->argNum < 1) goto LAB_080f94f4;
LAB_080f943f:
    if (vm_stackSize <= vm_sp) goto LAB_080f9518;
LAB_080f9450:
    pVVar2 = vm_stackTopValue();
    uVar1 = pVVar2->type;
    if (uVar1 == '\x04') goto LAB_080f953c;
LAB_080f9464:
    if ((uVar1 == '\n') && (pCookie = (pVVar2->v).buffer, (pCookie->realSize & 0x8000) == 0))
    goto LAB_080f9480;
LAB_080f946d:
    vm_error("Not found the object for destruct_object.\n");
  }
  else {
    vm_error("Too many arguments to destruct_object.\n");
    if (0 < vm_currentContext->argNum) goto LAB_080f943f;
LAB_080f94f4:
    vm_error("Miss argument 1, expected object | string.\n");
    if (vm_sp < vm_stackSize) goto LAB_080f9450;
LAB_080f9518:
    vm_error("Missed argument in stack.\n");
    pVVar2 = vm_stackTopValue();
    uVar1 = pVVar2->type;
    if (uVar1 != '\x04') goto LAB_080f9464;
LAB_080f953c:
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)] */
    pVVar2 = vm_stackTopValue();
    if (pVVar2->type != 4) {
      pcVar3 = vm_valueType((uint)pVVar2->type);
      vm_error("Bad type of value, expected string, got %s\n",pcVar3);
    }
    object = vm_lookupObjectByName((pVVar2->v).string);
    vm_popStack();
    if ((object == (Vm_Object_T *)0x0) || (object->flags == 0x800)) {
      vm_pushNumber(0);
    }
    else {
      vm_pushObject(object);
    }
    pVVar2 = vm_stackTopValue();
    if (pVVar2->type != '\n') goto LAB_080f946d;
  }
  pCookie = (pVVar2->v).buffer;
LAB_080f9480:
  if ((pCookie->realSize & 0xe000) == 0) {
                    /* Unresolved local var: Vm_Object_T * pObject@[???] */
    pCookie->realSize = pCookie->realSize | 0x2000;
    endHook = vm_f_destruct_object_crash;
    pCookie_00 = pCookie;
    pPCB = vm_getCurrentPCB();
    vm_setCrashHook(pPCB,endHook,pCookie_00);
    vm_f_destruct_object2(pCookie);
    vm_ctrlFlags = vm_ctrlFlags | 0x10;
    return;
  }
  vm_popNStack(vm_currentContext->argNum);
  return;
}


```

## 本轮：vm_makeObjectAlone — procedure_pseudo_code

Evidence ID：`ev_a1391c5cfc29264f27cbda3809a9bf634f736453297c310dc23f979b12218fef`

```c

void vm_makeObjectAlone(Vm_Object_T *pObject)

{
  Vm_Object_T *pVVar1;
  Vm_Program_T *pProgram;
  Vm_Object *pVVar2;
  int iVar3;
  Vm_Object **ppVVar4;
  
                    /* Unresolved local var: Vm_Object_T * p@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Object_T * * pp@[DW_OP_reg2(EDX)] */
  pVVar2 = pObject->pEnvObject;
  if (pVVar2 != (Vm_Object *)0x0) {
    ppVVar4 = &pVVar2->pLogderObject;
    pVVar1 = pVVar2->pLogderObject;
    while (pVVar1 != pObject) {
      pVVar2 = *ppVVar4;
      ppVVar4 = &pVVar2->pNeighbourObject;
      pVVar1 = pVVar2->pNeighbourObject;
    }
    *ppVVar4 = (*ppVVar4)->pNeighbourObject;
    pObject->pEnvObject = (Vm_Object *)0x0;
  }
  ppVVar4 = &pObject->pLogderObject;
  for (pVVar2 = pObject->pLogderObject; pVVar2 != (Vm_Object *)0x0;
      pVVar2 = pVVar2->pNeighbourObject) {
    *ppVVar4 = (Vm_Object *)0x0;
    ppVVar4 = &pVVar2->pNeighbourObject;
    pVVar2->pEnvObject = (Vm_Object *)0x0;
  }
  pVVar2 = pObject->pTemplateObject;
  if (pVVar2 != (Vm_Object *)0x0) {
    iVar3 = pVVar2->ref + -1;
    pVVar2->ref = iVar3;
    if ((iVar3 < 1) && ((pVVar2->flags & 0x8000) != 0)) {
      pProgram = pVVar2->pProgram;
      iVar3 = pProgram->ref + -1;
      pProgram->ref = iVar3;
      if (iVar3 < 1) {
        vm_freeCodeSegment(pProgram,&pProgram->initCodeSegment);
        if (vm_gatheringCallStackInfo != 0) {
          vm_enterPPWholeCriticalSection();
        }
        vm_destructProgramHead(pProgram);
        if (vm_gatheringCallStackInfo != 0) {
          vm_leavePPWholeCriticalSection();
        }
      }
      pdb_freeMemory(pVVar2,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1f5);
    }
    pObject->pTemplateObject = (Vm_Object *)0x0;
  }
  return;
}


```

## 本轮：vm_unExecRefObject — procedure_pseudo_code

Evidence ID：`ev_a208cbdd90503fb624273316e1f15e2eee556bf230a0dee842e864b4ecb72437`

```c

void vm_unExecRefObject(Vm_Object_T *pObject)

{
  int iVar1;
  
  iVar1 = pObject->execRef + -1;
  pObject->execRef = iVar1;
  if ((iVar1 < 1) && ((pObject->flags & 0x4000) != 0)) {
    vm_makeObjectAlone(pObject);
    pObject->pNext = pDestructingList;
    pObject->flags = pObject->flags | 0x8000;
    pDestructingList = pObject;
    pObject->ref = pObject->ref + 1;
    return;
  }
  return;
}


```

## 本轮：vm_f_destruct_func_ok — procedure_pseudo_code

Evidence ID：`ev_a4adb62dd62f7d6e50e24cd56b92f492b3bd3753a66ab3e889e8b59de3ce8db0`

```c

void vm_f_destruct_func_ok(void *pCookie)

{
  vm_endThreadCatch();
  vm_popStack();
  vm_f_destruct_object2(pCookie);
  return;
}


```

## 本轮：vm_f_destruct_object2 — procedure_assembly

Evidence ID：`ev_b1334a21e40fa77529b14cafafd4ce7d1669cdf41e7d2ff014d3bf24431307f3`

```asm
0x80f9284: PUSH EBP
0x80f9285: MOV EBP, ESP
0x80f9287: PUSH EDI
0x80f9288: PUSH ESI
0x80f9289: PUSH EBX
0x80f928a: SUB ESP, 0xc
0x80f928d: MOV EDI, dword ptr [EAX + 0x20]
0x80f9290: MOV dword ptr [EBP + -0x10], EAX
0x80f9293: MOV ESI, 0xffffffff
0x80f9298: CMP SI, -0x1
0x80f929c: JNZ 0x080f92ec
0x80f929e: MOV EDX, dword ptr [EBP + -0x10]
0x80f92a1: MOV CX, word ptr [EDX + 0x18]
0x80f92a5: LEA ESI, [ESI]
0x80f92a8: TEST CX, CX
0x80f92ab: JLE 0x080f9338
0x80f92b1: LEA EAX, [ECX + -0x1]
0x80f92b4: MOV ECX, EAX
0x80f92b6: MOV EAX, dword ptr [EBP + -0x10]
0x80f92b9: MOV word ptr [EAX + 0x18], CX
0x80f92bd: MOVSX EDX, CX
0x80f92c0: MOV EAX, dword ptr [EDI + 0x1c]
0x80f92c3: MOVSX EAX, word ptr [EAX + EDX*0x2]
0x80f92c7: LEA EAX, [EAX + EAX*0x2]
0x80f92ca: MOV EBX, dword ptr [EDI + 0x18]
0x80f92cd: LEA EDX, [EAX*0x4 + 0x0]
0x80f92d4: MOV EAX, dword ptr [EDX + EBX*0x1]
0x80f92d7: MOV SI, word ptr [EAX + 0xc]
0x80f92db: CMP SI, -0x1
0x80f92df: JZ 0x080f92a8
0x80f92e1: ADD SI, word ptr [EDX + EBX*0x1 + 0x6]
0x80f92e6: CMP SI, -0x1
0x80f92ea: JZ 0x080f929e
0x80f92ec: SUB ESP, 0x8
0x80f92ef: PUSH dword ptr [EBP + -0x10]
0x80f92f2: PUSH 0x80f95e8
0x80f92f7: CALL 0x080aaef0
0x80f92fc: MOV dword ptr [ESP], 0x0
0x80f9303: MOV EDX, dword ptr [EBP + -0x10]
0x80f9306: MOV EAX, dword ptr [EDX + 0x20]
0x80f9309: MOV EAX, dword ptr [EAX + 0x2c]
0x80f930c: MOVSX EDX, SI
0x80f930f: PUSH dword ptr [EAX + EDX*0x8]
0x80f9312: PUSH EDX
0x80f9313: PUSH dword ptr [EBP + -0x10]
0x80f9316: PUSH 0x0
0x80f9318: CALL 0x08093eb0
0x80f931d: ADD ESP, 0x18
0x80f9320: PUSH dword ptr [EBP + -0x10]
0x80f9323: PUSH 0x80f95cc
0x80f9328: CALL 0x08094e70
0x80f932d: ADD ESP, 0x10
0x80f9330: LEA ESP, [EBP + -0xc]
0x80f9333: POP EBX
0x80f9334: POP ESI
0x80f9335: POP EDI
0x80f9336: LEAVE
0x80f9337: RET
0x80f9338: CMP SI, -0x1
0x80f933c: JNZ 0x080f92ec
0x80f933e: SUB ESP, 0x8
0x80f9341: PUSH dword ptr [EBP + -0x10]
0x80f9344: PUSH EAX
0x80f9345: CALL 0x080a9310
0x80f934a: MOV dword ptr [ESP], EAX
0x80f934d: CALL 0x080a99d4
0x80f9352: MOV EAX, dword ptr [EBP + -0x10]
0x80f9355: AND dword ptr [EAX + 0xc], 0xffffdfff
0x80f935c: MOV dword ptr [ESP], EAX
0x80f935f: CALL 0x080b1894
0x80f9364: MOV EAX, [0x086619c0]
0x80f9369: LEA EAX, [EAX + EAX*0x4]
0x80f936c: MOV EDX, dword ptr [0x086619c8]
0x80f9372: LEA EBX, [EDX + EAX*0x8]
0x80f9375: MOV EAX, dword ptr [EBX + 0x24]
0x80f9378: ADD ESP, 0x10
0x80f937b: CMP byte ptr [EAX + 0x18], 0x1
0x80f937f: JZ 0x080f9396
0x80f9381: SUB ESP, 0x8
0x80f9384: PUSH 0x82bf0fd
0x80f9389: PUSH 0x8097b24
0x80f938e: CALL 0x080a9a48
0x80f9393: ADD ESP, 0x10
0x80f9396: SUB ESP, 0xc
0x80f9399: PUSH dword ptr [EBX + 0x4]
0x80f939c: CALL 0x08091ab0
0x80f93a1: ADD ESP, 0x10
0x80f93a4: LEA ESP, [EBP + -0xc]
0x80f93a7: POP EBX
0x80f93a8: POP ESI
0x80f93a9: POP EDI
0x80f93aa: LEAVE
0x80f93ab: JMP 0x08093c5c
```

## 本轮：vm_f_construct_object_failed — procedure_pseudo_code

Evidence ID：`ev_b1358e0f52d40e07afc5936ce0f39b582ec457b229e0df19581b14ab1da71fc2`

```c

void vm_f_construct_object_failed(void *pCookie)

{
  Vm_PCB_T *pPCB;
  code *endHook;
  void *pCookie_00;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Object_T * pObject@[???] */
  if ((*(uint *)((int)pCookie + 0xc) & 0xe000) != 0) {
    vm_popNStack(vm_funContext[vm_funCp].argNum);
    vm_pushNumber(0);
    vm_returnFromEfun();
    return;
  }
                    /* Unresolved local var: Vm_Object_T * pObject@[???] */
  *(uint *)((int)pCookie + 0xc) = *(uint *)((int)pCookie + 0xc) | 0x2000;
  endHook = vm_f_destruct_object_crash;
  pCookie_00 = pCookie;
  pPCB = vm_getCurrentPCB();
  vm_setCrashHook(pPCB,endHook,pCookie_00);
  vm_f_destruct_object2(pCookie);
  return;
}


```

## 本轮：vm_f_construct_object_failed — procedure_assembly

Evidence ID：`ev_b4cb5e180fe31ab5dc405e16e7a2ebe12a00f0e4e50fc8b530dbceea37e79753`

```asm
0x80f93b0: PUSH EBP
0x80f93b1: MOV EBP, ESP
0x80f93b3: PUSH EBX
0x80f93b4: PUSH EAX
0x80f93b5: MOV EAX, [0x086619c0]
0x80f93ba: LEA EAX, [EAX + EAX*0x4]
0x80f93bd: MOV EBX, dword ptr [EBP + 0x8]
0x80f93c0: MOV EDX, dword ptr [0x086619c8]
0x80f93c6: LEA EDX, [EDX + EAX*0x8]
0x80f93c9: MOV EAX, dword ptr [EBX + 0xc]
0x80f93cc: TEST AH, 0xe0
0x80f93cf: JZ 0x080f93f4
0x80f93d1: SUB ESP, 0xc
0x80f93d4: PUSH dword ptr [EDX + 0x4]
0x80f93d7: CALL 0x08091ab0
0x80f93dc: MOV dword ptr [ESP], 0x0
0x80f93e3: CALL 0x08090b3c
0x80f93e8: ADD ESP, 0x10
0x80f93eb: MOV EBX, dword ptr [EBP + -0x4]
0x80f93ee: LEAVE
0x80f93ef: JMP 0x08093c5c
0x80f93f4: OR AH, 0x20
0x80f93f7: MOV dword ptr [EBX + 0xc], EAX
0x80f93fa: PUSH EAX
0x80f93fb: PUSH EBX
0x80f93fc: PUSH 0x80f95f4
0x80f9401: PUSH EAX
0x80f9402: CALL 0x080a9310
0x80f9407: MOV dword ptr [ESP], EAX
0x80f940a: CALL 0x080a9964
0x80f940f: ADD ESP, 0x10
0x80f9412: MOV EAX, EBX
0x80f9414: MOV EBX, dword ptr [EBP + -0x4]
0x80f9417: LEAVE
0x80f9418: JMP 0x080f9284
```

## 本轮：vm_returnFromFakeFun — procedure_pseudo_code

Evidence ID：`ev_b9baf5880477fd6709044a22da45b676e2cf123b192086b6c047577017381b6e`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_returnFromFakeFun(void)

{
  byte bVar1;
  Vm_Object_T *pObject;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  Vm_PCB_T *pVVar4;
  
  pVVar3 = vm_funContext;
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Object_T * lastObject@[DW_OP_reg6(ESI)] */
  iVar2 = vm_funCp;
  pObject = vm_currentContext->thisObject;
  if (vm_gatheringCallStackInfo == 0) {
    vm_funCp = vm_funCp + -1;
    vm_updateCurrentFunCallContext();
    bVar1 = (byte)pVVar3[iVar2].flags;
  }
  else {
    vm_enterPPCopyStackCriticalSection();
    vm_funCp = vm_funCp + -1;
    vm_updateCurrentFunCallContext();
    vm_leavePPCopyStackCriticalSection();
    bVar1 = (byte)pVVar3[iVar2].flags;
  }
  if ((bVar1 & 8) == 0) {
    pVVar4 = vm_getCurrentPCB();
    pVVar4->flags = pVVar4->flags & 0xfffffeff;
  }
  vm_popNStack(pVVar3[iVar2].argNum);
  if (pObject != (Vm_Object_T *)0x0) {
    vm_unExecRefObject(pObject);
  }
  if ((pVVar3[iVar2].flags & 0x80) != 0) {
    (*pVVar3[iVar2].returnCallback)(pVVar3[iVar2].pCookie);
    return;
  }
  return;
}


```

## 本轮：vm_findProgramFunction — procedure_pseudo_code

Evidence ID：`ev_ba5b1993ec6d3677bd709e0fb78467636839e28ccd2802f1a214278e6a287928`

```c

Vm_Fun * vm_findProgramFunction
                   (Vm_Program_T *pProgram,Vm_String_T *pString,int mayPrivate,
                   Vm_Index_T *pFunOffset)

{
  short sVar1;
  char *s;
  ushort uVar2;
  uint uVar3;
  int iVar4;
  Vm_Ident_Hash_Elem *pVVar5;
  uint uVar6;
  Vm_Ident_Hash_Elem *pVVar7;
  Vm_Ident_Hash_Elem **local_1c;
  int h;
  Vm_Fun_T *pFun;
  
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[???] */
  local_1c = pProgram->ppHashTable;
  if (local_1c == (Vm_Ident_Hash_Elem **)0x0) {
                    /* Unresolved local var: int index@[DW_OP_reg3(EBX)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)] */
    if (((pString->attrib & 1) == 0) &&
       (pString = vm_findSharedString(pString->ptr,pString->len), pString == (Vm_String_T *)0x0)) {
      return (Vm_Fun *)0x0;
    }
    uVar6 = (uint)pProgram->quickFunctionSize;
    uVar3 = ((uint)pString >> 3) % uVar6;
    iVar4 = 0;
    if (0 < (int)uVar6) {
      do {
                    /* Unresolved local var: Vm_Index_T strNo@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Index_T funOffset@[DW_OP_reg1(ECX)] */
        sVar1 = pProgram->pQuickFunction[uVar3].strNo;
        if (sVar1 == -1) {
          return (Vm_Fun *)0x0;
        }
        if (pProgram->ppStrings[sVar1] == pString) {
          sVar1 = pProgram->pQuickFunction[uVar3].funOffset;
          pFun = pProgram->pFunctions[sVar1].pFun;
          if (pFunOffset != (Vm_Index_T *)0x0) {
            *pFunOffset = sVar1;
          }
          break;
        }
        iVar4 = iVar4 + 1;
        uVar3 = (int)(uVar3 + 1) % (int)uVar6;
      } while (iVar4 < (int)uVar6);
    }
  }
  else {
    uVar2 = pString->hash;
    if (uVar2 == 0) {
      uVar2 = vm_whashstr(pString->ptr,0x20);
      local_1c = pProgram->ppHashTable;
      pString->hash = uVar2;
    }
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * hptr@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * hptr2@[DW_OP_reg3(EBX)] */
    s = pString->ptr;
    uVar3 = (uint)uVar2;
    if (local_1c == (Vm_Ident_Hash_Elem **)0x0) {
      return (Vm_Fun *)0x0;
    }
    if (uVar3 == 0) {
      uVar2 = vm_whashstr(s,0x20);
      uVar3 = (uint)uVar2;
      local_1c = pProgram->ppHashTable;
    }
    h = uVar3 & 0x3ff;
    pVVar5 = local_1c[h];
    if (pVVar5 == (Vm_Ident_Hash_Elem *)0x0) {
      return (Vm_Fun *)0x0;
    }
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    iVar4 = strcmp(pVVar5->name,s);
    if (iVar4 == 0) {
      if ((-1 < (int)pVVar5->token) && (pVVar5->semValue == 0)) {
        return (Vm_Fun *)0x0;
      }
    }
    else {
      pVVar7 = pVVar5->pNext;
      while( true ) {
        if (pVVar7 == pVVar5) {
          return (Vm_Fun *)0x0;
        }
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
        iVar4 = strcmp(pVVar7->name,s);
        if (iVar4 == 0) break;
        pVVar7 = pVVar7->pNext;
      }
      if ((-1 < (int)pVVar7->token) && (pVVar7->semValue == 0)) {
        return (Vm_Fun *)0x0;
      }
      local_1c[h] = pVVar7;
      pVVar5 = pVVar7;
    }
    if (pVVar5 == (Vm_Ident_Hash_Elem *)0x0) {
      return (Vm_Fun *)0x0;
    }
    if ((pVVar5->token & 0x8000000) == 0) {
      return (Vm_Fun *)0x0;
    }
    pFun = (pVVar5->ii).v.uf.pFun;
    if (pFunOffset != (Vm_Index_T *)0x0) {
      *pFunOffset = (pVVar5->ii).v.uf.funOffset;
    }
  }
  if ((mayPrivate == 0) && ((pFun->attrib & 4) != 0)) {
    return (Vm_Fun *)0x0;
  }
  return pFun;
}


```

## 本轮：vm_unExecRefObject — procedure_assembly

Evidence ID：`ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f`

```asm
0x80b1804: PUSH EBP
0x80b1805: MOV EBP, ESP
0x80b1807: PUSH EBX
0x80b1808: PUSH EAX
0x80b1809: MOV EBX, dword ptr [EBP + 0x8]
0x80b180c: MOV EAX, dword ptr [EBX + 0x4]
0x80b180f: DEC EAX
0x80b1810: TEST EAX, EAX
0x80b1812: MOV dword ptr [EBX + 0x4], EAX
0x80b1815: JLE 0x080b181c
0x80b1817: MOV EBX, dword ptr [EBP + -0x4]
0x80b181a: LEAVE
0x80b181b: RET
0x80b181c: TEST byte ptr [EBX + 0xd], 0x40
0x80b1820: JZ 0x080b1817
0x80b1822: SUB ESP, 0xc
0x80b1825: PUSH EBX
0x80b1826: CALL 0x080b1728
0x80b182b: MOV EAX, [0x08609a40]
0x80b1830: MOV dword ptr [EBX + 0x28], EAX
0x80b1833: OR dword ptr [EBX + 0xc], 0x8000
0x80b183a: MOV dword ptr [0x08609a40], EBX
0x80b1840: INC dword ptr [EBX]
0x80b1842: ADD ESP, 0x10
0x80b1845: MOV EBX, dword ptr [EBP + -0x4]
0x80b1848: LEAVE
0x80b1849: RET
```

## 本轮：vm_callInInitFun — procedure_pseudo_code

Evidence ID：`ev_cdee74db197772958f9b9047071e28d032beccf6d11b3ef31d290780aef303e8`

```c

void vm_callInInitFun(Vm_Object_T *pToObject,Vm_Index_T consititutionNo)

{
  Vm_Program_Consititution *pVVar1;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  Vm_Code_Segment *pVVar4;
  Vm_PCB_T *pVVar5;
  Vm_Code_Segment_T *pEntryCodeSegment;
  
                    /* Unresolved local var: Vm_Program_Consititution_T *
                       pConsititution@[DW_OP_reg7(EDI)] */
  if ((pToObject->flags & 0x8000) != 0) {
    vm_error("Object is not found (when call in fake function).\n");
  }
  pVVar1 = pToObject->pProgram->pConsititutions + consititutionNo;
  pVVar4 = &pVVar1->pProgram->initCodeSegment;
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg3(EBX)] */
  if (vm_funContextSize <= vm_funCp) {
    vm_error("Too depth (%d) function call!\n",vm_funCp);
  }
  if (vm_gatheringCallStackInfo != 0) {
    vm_enterPPCopyStackCriticalSection();
  }
  pVVar5 = vm_getCurrentPCB();
  vm_funCp = vm_funCp + 1;
  vm_updateCurrentFunCallContext();
  pVVar3 = vm_currentContext;
  iVar2 = vm_sp;
  vm_currentContext->flags = 2;
  pVVar3->bp = iVar2;
  pVVar3->argNum = 0;
  (pVVar3->f).pEfun = (Vm_Efun *)"Initialization";
  if ((pVVar5->flags & 0x100) != 0) {
    pVVar3->flags = 10;
  }
  pVVar3 = vm_currentContext;
  vm_currentContext->flags = vm_currentContext->flags | 0x10;
  pVVar3->thisObject = pToObject;
  pVVar3->consititution = pVVar1;
  if (pToObject != (Vm_Object_T *)0x0) {
    pToObject->execRef = pToObject->execRef + 1;
  }
  if (vm_gatheringCallStackInfo == 0) {
    pVVar3->ip = 0;
    pVVar3->cs = pVVar4;
    return;
  }
  vm_leavePPCopyStackCriticalSection();
  pVVar3 = vm_currentContext;
  vm_currentContext->ip = 0;
  pVVar3->cs = pVVar4;
  return;
}


```

## 本轮：vm_allocateValues — procedure_assembly

Evidence ID：`ev_d5800f267643d7ee5dde2878266b2d7345b65e5040bf3a244897aa271ce5c690`

```asm
0x808a088: PUSH EBP
0x808a089: MOV EBP, ESP
0x808a08b: PUSH EDI
0x808a08c: PUSH ESI
0x808a08d: PUSH EBX
0x808a08e: SUB ESP, 0xc
0x808a091: PUSH 0xd5
0x808a096: MOV EDI, dword ptr [EBP + 0x8]
0x808a099: PUSH 0x82acaf8
0x808a09e: LEA EAX, [EDI + EDI*0x2]
0x808a0a1: PUSH 0x82a376b
0x808a0a6: SHL EAX, 0x3
0x808a0a9: PUSH EAX
0x808a0aa: CALL 0x080ce784
0x808a0af: ADD ESP, 0x10
0x808a0b2: TEST EAX, EAX
0x808a0b4: MOV EBX, EAX
0x808a0b6: JZ 0x0808a130
0x808a0b8: TEST EDI, EDI
0x808a0ba: MOV EDX, EAX
0x808a0bc: JLE 0x0808a130
0x808a0be: CLD
0x808a0bf: MOV ESI, 0x6
0x808a0c4: MOV dword ptr [EBP + -0x10], EDI
0x808a0c7: JMP 0x0808a0eb
0x808a0cc: MOV dword ptr [0x0847fd8c], EDX
0x808a0d2: MOV dword ptr [EDX + 0x14], 0x0
0x808a0d9: MOV dword ptr [EDX + 0xc], 0x0
0x808a0e0: MOV dword ptr [EDX + 0x10], EDX
0x808a0e3: ADD EDX, 0x18
0x808a0e6: DEC dword ptr [EBP + -0x10]
0x808a0e9: JZ 0x0808a130
0x808a0eb: XOR EAX, EAX
0x808a0ed: MOV EDI, EDX
0x808a0ef: MOV ECX, ESI
0x808a0f1: STOSD.REP ES:EDI
0x808a0f3: MOV byte ptr [EDX + 0x4], 0x0
0x808a0f7: MOV byte ptr [EDX + 0x5], 0x0
0x808a0fb: MOV ECX, dword ptr [0x0847fd8c]
0x808a101: MOV dword ptr [EDX], 0x1
0x808a107: INC dword ptr [0x0847fd88]
0x808a10d: TEST ECX, ECX
0x808a10f: JZ 0x0808a0cc
0x808a111: MOV dword ptr [EDX + 0x14], 0x0
0x808a118: MOV dword ptr [EDX + 0xc], ECX
0x808a11b: MOV dword ptr [ECX + 0x14], EDX
0x808a11e: MOV dword ptr [0x0847fd8c], EDX
0x808a124: MOV dword ptr [EDX + 0x10], EDX
0x808a127: ADD EDX, 0x18
0x808a12a: DEC dword ptr [EBP + -0x10]
0x808a12d: JNZ 0x0808a0eb
0x808a12f: NOP
0x808a130: LEA ESP, [EBP + -0xc]
0x808a133: MOV EAX, EBX
0x808a135: POP EBX
0x808a136: POP ESI
0x808a137: POP EDI
0x808a138: LEAVE
0x808a139: RET
```

## 本轮：vm_f_construct_object2 — procedure_assembly

Evidence ID：`ev_da96a5f71d49dbc7b88d7cf809835b39c4a15c293342ba20674155e53dfa3606`

```asm
0x80f90b4: PUSH EBP
0x80f90b5: MOV EBP, ESP
0x80f90b7: PUSH EDI
0x80f90b8: PUSH ESI
0x80f90b9: PUSH EBX
0x80f90ba: SUB ESP, 0x1c
0x80f90bd: MOV ESI, EAX
0x80f90bf: MOV EAX, [0x086619c0]
0x80f90c4: LEA EAX, [EAX + EAX*0x4]
0x80f90c7: MOV EDX, dword ptr [0x086619c8]
0x80f90cd: LEA EAX, [EDX + EAX*0x8]
0x80f90d0: MOV dword ptr [EBP + -0x10], EAX
0x80f90d3: MOV EAX, dword ptr [ESI + 0xc]
0x80f90d6: TEST AH, 0xe0
0x80f90d9: MOV dword ptr [EBP + -0x14], EAX
0x80f90dc: JNZ 0x080f917c
0x80f90e2: MOV CX, word ptr [ESI + 0x18]
0x80f90e6: TEST CX, CX
0x80f90e9: MOV EDI, dword ptr [ESI + 0x20]
0x80f90ec: JS 0x080f91a5
0x80f90f2: CMP CX, word ptr [EDI + 0x14]
0x80f90f6: JGE 0x080f9149
0x80f90f8: MOV EBX, dword ptr [EDI + 0x1c]
0x80f90fb: MOV EAX, dword ptr [EDI + 0x18]
0x80f90fe: MOV dword ptr [EBP + -0x1c], EBX
0x80f9101: MOV dword ptr [EBP + -0x18], EAX
0x80f9104: MOV EDX, dword ptr [EBP + -0x1c]
0x80f9107: MOVSX EAX, CX
0x80f910a: MOVSX EAX, word ptr [EDX + EAX*0x2]
0x80f910e: LEA EAX, [EAX + EAX*0x2]
0x80f9111: MOV EBX, dword ptr [EBP + -0x18]
0x80f9114: LEA EDX, [EAX*0x4 + 0x0]
0x80f911b: MOV EAX, dword ptr [EDX + EBX*0x1]
0x80f911e: MOV BX, word ptr [EAX + 0xa]
0x80f9122: CMP BX, -0x1
0x80f9126: JZ 0x080f9130
0x80f9128: MOV EAX, dword ptr [EBP + -0x18]
0x80f912b: ADD BX, word ptr [EDX + EAX*0x1 + 0x6]
0x80f9130: LEA EAX, [ECX + 0x1]
0x80f9133: CMP BX, -0x1
0x80f9137: MOV ECX, EAX
0x80f9139: MOV word ptr [ESI + 0x18], AX
0x80f913d: JNZ 0x080f91e6
0x80f9143: CMP AX, word ptr [EDI + 0x14]
0x80f9147: JL 0x080f9104
0x80f9149: AND dword ptr [EBP + -0x14], 0xffffe7ff
0x80f9150: MOV EBX, dword ptr [EBP + -0x14]
0x80f9153: SUB ESP, 0xc
0x80f9156: MOV dword ptr [ESI + 0xc], EBX
0x80f9159: MOV EAX, dword ptr [EBP + -0x10]
0x80f915c: PUSH dword ptr [EAX + 0x4]
0x80f915f: CALL 0x08091ab0
0x80f9164: MOV dword ptr [ESP], ESI
0x80f9167: CALL 0x080912f4
0x80f916c: ADD ESP, 0x10
0x80f916f: LEA ESP, [EBP + -0xc]
0x80f9172: POP EBX
0x80f9173: POP ESI
0x80f9174: POP EDI
0x80f9175: LEAVE
0x80f9176: JMP 0x08093c5c
0x80f917c: SUB ESP, 0xc
0x80f917f: MOV EDX, dword ptr [EBP + -0x10]
0x80f9182: PUSH dword ptr [EDX + 0x4]
0x80f9185: CALL 0x08091ab0
0x80f918a: MOV dword ptr [ESP], 0x0
0x80f9191: CALL 0x08090b3c
0x80f9196: ADD ESP, 0x10
0x80f9199: LEA ESP, [EBP + -0xc]
0x80f919c: POP EBX
0x80f919d: POP ESI
0x80f919e: POP EDI
0x80f919f: LEAVE
0x80f91a0: JMP 0x08093c5c
0x80f91a5: SUB ESP, 0x8
0x80f91a8: PUSH ESI
0x80f91a9: PUSH 0x80f93b0
0x80f91ae: CALL 0x080aaef0
0x80f91b3: MOVSX EDX, word ptr [EDI + 0x14]
0x80f91b7: MOVSX EAX, word ptr [ESI + 0x18]
0x80f91bb: ADD EDX, EAX
0x80f91bd: POP ECX
0x80f91be: MOV EAX, dword ptr [EDI + 0x1c]
0x80f91c1: POP EBX
0x80f91c2: MOVSX EAX, word ptr [EAX + EDX*0x2]
0x80f91c6: PUSH EAX
0x80f91c7: PUSH ESI
0x80f91c8: CALL 0x08094258
0x80f91cd: INC word ptr [ESI + 0x18]
0x80f91d1: POP EBX
0x80f91d2: POP EDI
0x80f91d3: PUSH ESI
0x80f91d4: PUSH 0x80f9250
0x80f91d9: CALL 0x08094e70
0x80f91de: LEA ESP, [EBP + -0xc]
0x80f91e1: POP EBX
0x80f91e2: POP ESI
0x80f91e3: POP EDI
0x80f91e4: LEAVE
0x80f91e5: RET
0x80f91e6: SUB ESP, 0x8
0x80f91e9: PUSH ESI
0x80f91ea: PUSH 0x80f93b0
0x80f91ef: CALL 0x080aaef0
0x80f91f4: MOV EDX, dword ptr [EBP + -0x10]
0x80f91f7: ADD ESP, 0x10
0x80f91fa: XOR EDI, EDI
0x80f91fc: CMP dword ptr [EDX + 0x4], 0x1
0x80f9200: JG 0x080f922e
0x80f9202: SUB ESP, 0xc
0x80f9205: PUSH EDI
0x80f9206: MOV EAX, dword ptr [ESI + 0x20]
0x80f9209: MOV EAX, dword ptr [EAX + 0x2c]
0x80f920c: MOVSX EDX, BX
0x80f920f: PUSH dword ptr [EAX + EDX*0x8]
0x80f9212: PUSH EDX
0x80f9213: PUSH ESI
0x80f9214: PUSH 0x0
0x80f9216: CALL 0x08093eb0
0x80f921b: ADD ESP, 0x18
0x80f921e: PUSH ESI
0x80f921f: PUSH 0x80f9268
0x80f9224: CALL 0x08094e70
0x80f9229: ADD ESP, 0x10
0x80f922c: JMP 0x080f91de
0x80f922e: MOV EAX, dword ptr [EDX]
0x80f9230: LEA EAX, [EAX + EAX*0x2]
0x80f9233: MOV EDX, dword ptr [0x08661964]
0x80f9239: SUB ESP, 0xc
0x80f923c: LEA EAX, [EDX + EAX*0x8 + 0x18]
0x80f9240: PUSH EAX
0x80f9241: CALL 0x08091074
0x80f9246: MOV DI, 0x1
0x80f924a: ADD ESP, 0x10
0x80f924d: JMP 0x080f9202
```

## 本轮：vm_findProgramFunctionByCStr — procedure_pseudo_code

Evidence ID：`ev_e0725c9f865c565504a4a69ce6866c428ca0be60f899fe4a27de58ea497a0c7d`

```c

Vm_Fun * vm_findProgramFunctionByCStr
                   (Vm_Program_T *pProgram,char *str,int mayPrivate,Vm_Index_T *pFunOffset)

{
  char cVar1;
  Vm_String_T *pVVar2;
  Vm_Fun *pVVar3;
  uint uVar4;
  int iVar5;
  char *pcVar6;
  Vm_String_T string;
  
                    /* Unresolved local var: Vm_String_T * pString@[DW_OP_reg0(EAX)] */
  uVar4 = 0xffffffff;
  pcVar6 = str;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  pVVar2 = vm_findSharedString(str,~uVar4 - 1);
  if (pVVar2 == (Vm_String_T *)0x0) {
    if (pProgram != vm_pGlobalProgram) {
      return (Vm_Fun *)0x0;
    }
    pVVar2 = &string;
    for (iVar5 = 5; iVar5 != 0; iVar5 = iVar5 + -1) {
      pVVar2->ref = 0;
      pVVar2 = (Vm_String_T *)&pVVar2->attrib;
    }
    string.ref = 1;
    uVar4 = 0xffffffff;
    pcVar6 = str;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar6 + 1;
    } while (cVar1 != '\0');
    string.len = ~uVar4 - 1;
    pVVar2 = &string;
    string.attrib = 0x8002;
    string.ptr = str;
  }
  pVVar3 = vm_findProgramFunction(pProgram,pVVar2,mayPrivate,pFunOffset);
  return pVVar3;
}


```

## 本轮：vm_f_clone_object — procedure_pseudo_code

Evidence ID：`ev_e421a1294b8ef5f2e44edaf23ffeed408af95fad72a2e33232ce13d020c35748`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_clone_object(void)

{
  Vm_Object *pVVar1;
  uchar uVar2;
  Vm_Value *pVVar3;
  Vm_Object_T *pVVar4;
  char *pcVar5;
  bool bVar6;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Object_T * pNewObject@[DW_OP_reg3(EBX)] */
  if (vm_currentContext->argNum < 3) {
    if (vm_currentContext->argNum < 1) goto LAB_080f8fb4;
LAB_080f8ecb:
    if (1 < vm_currentContext->argNum) goto LAB_080f8fd4;
LAB_080f8ed5:
    if (vm_stackSize <= vm_sp) goto LAB_080f8ff4;
LAB_080f8ee6:
    pVVar3 = vm_stackTopValue();
    uVar2 = pVVar3->type;
    if (uVar2 == '\x04') goto LAB_080f9018;
LAB_080f8efa:
    if (uVar2 == '\n') {
      bVar6 = ((uint)((pVVar3->v).string)->ptr & 0x8000) == 0;
      goto LAB_080f9087;
    }
  }
  else {
    vm_error("Too many arguments(%d) to %s.\n",vm_currentContext->argNum,"clone_object");
    if (0 < vm_currentContext->argNum) goto LAB_080f8ecb;
LAB_080f8fb4:
    vm_error("Miss argument 1, expected object | string.\n");
    if (vm_currentContext->argNum < 2) goto LAB_080f8ed5;
LAB_080f8fd4:
    vm_getArgument(1,7);
    if (vm_sp < vm_stackSize) goto LAB_080f8ee6;
LAB_080f8ff4:
    vm_error("Missed argument in stack.\n");
    pVVar3 = vm_stackTopValue();
    uVar2 = pVVar3->type;
    if (uVar2 != '\x04') goto LAB_080f8efa;
LAB_080f9018:
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)] */
    pVVar3 = vm_stackTopValue();
    if (pVVar3->type != 4) {
      pcVar5 = vm_valueType((uint)pVVar3->type);
      vm_error("Bad type of value, expected string, got %s\n",pcVar5);
    }
    pVVar4 = vm_lookupObjectByName((pVVar3->v).string);
    vm_popStack();
    if ((pVVar4 == (Vm_Object_T *)0x0) || (pVVar4->flags == 0x800)) {
      vm_pushNumber(0);
    }
    else {
      vm_pushObject(pVVar4);
    }
    pVVar3 = vm_stackTopValue();
    bVar6 = pVVar3->type == '\n';
LAB_080f9087:
    if (bVar6) goto LAB_080f8f13;
  }
  vm_error("Not found the object for clone_object.\n");
LAB_080f8f13:
  pVVar4 = vm_cloneObject((Vm_Object_T *)(pVVar3->v).string);
  if (pVVar4 == (Vm_Object_T *)0x0) {
    vm_error("Can not clone object.\n");
  }
  pVVar1 = (Vm_Object *)(pVVar3->v).string;
  pVVar4->pTemplateObject = pVVar1;
  pVVar1->ref = pVVar1->ref + 1;
                    /* Unresolved local var: Vm_Index_T consititutionToInit@[???] */
  pVVar4->initStep = -pVVar4->pProgram->maxConsititutionNo;
  pVVar4->flags = pVVar4->flags | 0x1000;
  vm_newThreadCatch2(vm_f_construct_object_failed,pVVar4);
  vm_callInInitFun(pVVar4,*pVVar4->pProgram->initOrderList);
  pVVar4->initStep = pVVar4->initStep + 1;
  vm_hookCallbackWhenReturn(vm_f_construct_object_ok1,pVVar4);
  vm_ctrlFlags = vm_ctrlFlags | 0x10;
  return;
}


```

## 本轮：vm_callInInitFun — procedure_assembly

Evidence ID：`ev_e87fd215c338b444b786777608553c57b73d83dc1d3675ab73c10ec003991e6f`

```asm
0x8094258: PUSH EBP
0x8094259: MOV EBP, ESP
0x809425b: PUSH EDI
0x809425c: PUSH ESI
0x809425d: PUSH EBX
0x809425e: SUB ESP, 0xc
0x8094261: MOV ESI, dword ptr [EBP + 0x8]
0x8094264: TEST byte ptr [ESI + 0xd], 0x80
0x8094268: MOV EBX, dword ptr [EBP + 0xc]
0x809426b: JNZ 0x0809435c
0x8094271: MOV EAX, dword ptr [ESI + 0x20]
0x8094274: MOVSX EDX, BX
0x8094277: MOV EAX, dword ptr [EAX + 0x18]
0x809427a: LEA EDX, [EDX + EDX*0x2]
0x809427d: LEA EDI, [EAX + EDX*0x4]
0x8094280: MOV EAX, dword ptr [EDI]
0x8094282: ADD EAX, 0x34
0x8094285: MOV dword ptr [EBP + -0x10], EAX
0x8094288: MOV EAX, [0x086619c0]
0x809428d: CMP EAX, dword ptr [0x086619c4]
0x8094293: JGE 0x08094344
0x8094299: MOV ECX, dword ptr [0x0860ae64]
0x809429f: TEST ECX, ECX
0x80942a1: JNZ 0x0809431c
0x80942a3: CALL 0x080a9310
0x80942a8: MOV EBX, EAX
0x80942aa: INC dword ptr [0x086619c0]
0x80942b0: CALL 0x0809789c
0x80942b5: MOV EDX, dword ptr [0x086619d8]
0x80942bb: MOV EAX, [0x086619cc]
0x80942c0: MOV dword ptr [EDX + 0x8], 0x2
0x80942c7: MOV dword ptr [EDX], EAX
0x80942c9: MOV dword ptr [EDX + 0x4], 0x0
0x80942d0: MOV dword ptr [EDX + 0x24], 0x82ae07c
0x80942d7: TEST byte ptr [EBX + 0x29], 0x1
0x80942db: JZ 0x080942e4
0x80942dd: MOV dword ptr [EDX + 0x8], 0xa
0x80942e4: MOV EAX, [0x086619d8]
0x80942e9: OR dword ptr [EAX + 0x8], 0x10
0x80942ed: TEST ESI, ESI
0x80942ef: MOV dword ptr [EAX + 0x20], ESI
0x80942f2: MOV dword ptr [EAX + 0x14], EDI
0x80942f5: JZ 0x080942fa
0x80942f7: INC dword ptr [ESI + 0x4]
0x80942fa: MOV EDX, dword ptr [0x0860ae64]
0x8094300: TEST EDX, EDX
0x8094302: JNZ 0x08094324
0x8094304: MOV EDX, dword ptr [EBP + -0x10]
0x8094307: MOV dword ptr [EAX + 0xc], 0x0
0x809430e: MOV dword ptr [EAX + 0x10], EDX
0x8094311: LEA ESP, [EBP + -0xc]
0x8094314: POP EBX
0x8094315: POP ESI
0x8094316: POP EDI
0x8094317: LEAVE
0x8094318: RET
0x809431c: CALL 0x080bb0d4
0x8094321: JMP 0x080942a3
0x8094324: CALL 0x080bb0e8
0x8094329: MOV EAX, [0x086619d8]
0x809432e: MOV EDX, dword ptr [EBP + -0x10]
0x8094331: MOV dword ptr [EAX + 0xc], 0x0
0x8094338: MOV dword ptr [EAX + 0x10], EDX
0x809433b: LEA ESP, [EBP + -0xc]
0x809433e: POP EBX
0x809433f: POP ESI
0x8094340: POP EDI
0x8094341: LEAVE
0x8094342: RET
0x8094344: SUB ESP, 0x8
0x8094347: PUSH EAX
0x8094348: PUSH 0x82ae31c
0x809434d: CALL 0x08097b24
0x8094352: ADD ESP, 0x10
0x8094355: JMP 0x08094299
0x809435c: SUB ESP, 0xc
0x809435f: PUSH 0x82ae450
0x8094364: CALL 0x08097b24
0x8094369: ADD ESP, 0x10
0x809436c: JMP 0x08094271
```

## 本轮：0x8096f44 — procedure_pseudo_code

Evidence ID：`ev_ed6d15234b6b77e70190cfcdc433e895468986122447c7f45f1bd8c78c562ba6`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_endForBackend(void)

{
  vm_freeObjectsInDestructingList();
  vm_freeObsoletedJitCodes();
  vm_freeObsoletedMapNodes();
  _vm_clearTimer(0);
  return;
}


```

## 本轮：vm_f_destruct_object2 — procedure_pseudo_code

Evidence ID：`ev_f17d1950e66d2c98c086472f49d99c9eea4f55ff75edef0ef0e659d5df2c716e`

```c

void __regparm3 vm_f_destruct_object2(void *pCookie)

{
  int iVar1;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  Vm_PCB_T *pPCB;
  short sVar4;
  short funOffset;
  void *pCookie_00;
  Vm_Object_T *pObject;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[???]
                       Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Index_T funOffset@[DW_OP_reg6(ESI)] */
  funOffset = -1;
  do {
    sVar4 = *(short *)((int)pCookie + 0x18);
    do {
      if (sVar4 < 1) {
        if (funOffset == -1) {
          pCookie_00 = pCookie;
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg3(EBX)] */
          pPCB = vm_getCurrentPCB();
          vm_clearCrashHook(pPCB,pCookie_00);
          *(uint *)((int)pCookie + 0xc) = *(uint *)((int)pCookie + 0xc) & 0xffffdfff;
          vm_destructObject(pCookie);
          pVVar3 = vm_funContext;
          iVar1 = vm_funCp;
          if ((vm_funContext[vm_funCp].f.pEfun)->retType != '\x01') {
            vm_hookCallbackWhenNextAction(vm_error,"Failed to construct object.\n");
          }
          vm_popNStack(pVVar3[iVar1].argNum);
          vm_returnFromEfun();
          return;
        }
        goto LAB_080f92ec;
      }
                    /* Unresolved local var: Vm_Index_T consititutionToDestruct@[???] */
      sVar4 = sVar4 + -1;
      *(short *)((int)pCookie + 0x18) = sVar4;
      iVar2 = *(int *)(*(int *)((int)pCookie + 0x20) + 0x18);
      iVar1 = *(short *)(*(int *)(*(int *)((int)pCookie + 0x20) + 0x1c) + sVar4 * 2) * 0xc;
      funOffset = *(short *)(*(int *)(iVar1 + iVar2) + 0xc);
    } while (funOffset == -1);
    funOffset = funOffset + *(short *)(iVar1 + 6 + iVar2);
  } while (funOffset == -1);
LAB_080f92ec:
  vm_newThreadCatch2(vm_f_destruct_func_failed,pCookie);
  vm_callInUserFunBody
            (0,pCookie,funOffset,
             *(Vm_Fun_T **)(*(int *)(*(int *)((int)pCookie + 0x20) + 0x2c) + funOffset * 8),0);
  vm_hookCallbackWhenReturn(vm_f_destruct_func_ok,pCookie);
  return;
}


```

## 本轮：vm_f_destruct_func_failed — procedure_pseudo_code

Evidence ID：`ev_f4fc3a30d4d10d12d35be2daec518e2e8e8e1c601750a418d198e9540ce61de5`

```c

void vm_f_destruct_func_failed(void *pCookie)

{
  vm_f_destruct_object2(pCookie);
  return;
}


```

## 本轮：vm_freeObjectsInDestructingList — procedure_callers

Evidence ID：`ev_f5e402fe68092fc8f3da7f19e3e8f75ed5e9d36f205c394d10e109d9618d37b5`

```json
[
  "0x8096f44",
  "0x80aca84",
  "0x80ae0e8"
]
```

## 本轮：vm_f_construct_object2 — procedure_pseudo_code

Evidence ID：`ev_f8180e2eaf29584441f8a0ae1e5d5e7d0b3e94dbf7d2f01a2000b72058f7324c`

```c

void __regparm3 vm_f_construct_object2(void *pCookie)

{
  Vm_FunCall_Context_T *pVVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  bool bVar6;
  short sVar7;
  short funOffset;
  Vm_FunCall_Context_T *pContext;
  
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Index_T consititutionToInit@[???]
                       Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Index_T funOffset@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_funContext + vm_funCp;
  if ((*(uint *)((int)pCookie + 0xc) & 0xe000) != 0) {
    vm_popNStack(pVVar1->argNum);
    vm_pushNumber(0);
    vm_returnFromEfun();
    return;
  }
  sVar7 = *(short *)((int)pCookie + 0x18);
  iVar3 = *(int *)((int)pCookie + 0x20);
  if (sVar7 < 0) {
    vm_newThreadCatch2(vm_f_construct_object_failed,pCookie);
    vm_callInInitFun(pCookie,*(Vm_Index_T *)
                              (*(int *)(iVar3 + 0x1c) +
                              ((int)*(short *)(iVar3 + 0x14) + (int)*(short *)((int)pCookie + 0x18))
                              * 2));
    *(short *)((int)pCookie + 0x18) = *(short *)((int)pCookie + 0x18) + 1;
    vm_hookCallbackWhenReturn(vm_f_construct_object_ok1,pCookie);
    return;
  }
  if (sVar7 < *(short *)(iVar3 + 0x14)) {
    iVar4 = *(int *)(iVar3 + 0x1c);
    iVar5 = *(int *)(iVar3 + 0x18);
    do {
      iVar2 = *(short *)(iVar4 + sVar7 * 2) * 0xc;
      funOffset = *(short *)(*(int *)(iVar2 + iVar5) + 10);
      if (funOffset != -1) {
        funOffset = funOffset + *(short *)(iVar2 + 6 + iVar5);
      }
      sVar7 = sVar7 + 1;
      *(short *)((int)pCookie + 0x18) = sVar7;
      if (funOffset != -1) {
                    /* Unresolved local var: int initArgCount@[DW_OP_reg7(EDI)] */
        vm_newThreadCatch2(vm_f_construct_object_failed,pCookie);
        bVar6 = 1 < pVVar1->argNum;
        if (bVar6) {
          vm_pushValue(vm_stack + pVVar1->bp + 1);
        }
        vm_callInUserFunBody
                  (0,pCookie,funOffset,
                   *(Vm_Fun_T **)(*(int *)(*(int *)((int)pCookie + 0x20) + 0x2c) + funOffset * 8),
                   (uint)bVar6);
        vm_hookCallbackWhenReturn(vm_f_construct_object_ok2,pCookie);
        return;
      }
    } while (sVar7 < *(short *)(iVar3 + 0x14));
  }
  *(uint *)((int)pCookie + 0xc) = *(uint *)((int)pCookie + 0xc) & 0xffffe7ff;
  vm_popNStack(pVVar1->argNum);
  vm_pushObject(pCookie);
  vm_returnFromEfun();
  return;
}


```

## 本轮：vm_cloneObject — procedure_pseudo_code

Evidence ID：`ev_fc3aace8758804f9f7710a10967b65590ef8e9fb8b17bd2b2a7f32d813fc0d04`

```c

Vm_Object_T * vm_cloneObject(Vm_Object_T *pObject)

{
  Vm_Object_T *pVVar1;
  uint oid;
  
                    /* Unresolved local var: Vm_Object_T * pNewObject@[???]
                       Unresolved local var: uint oid@[???] */
  do {
    do {
      oid = _vm_globalOid;
      _vm_globalOid = oid + 1;
    } while (oid == 0);
    pVVar1 = vm_lookupObject(pObject->pProgram->strName,oid);
  } while (pVVar1 != (Vm_Object_T *)0x0);
  pVVar1 = vm_allocateObject(pObject->pProgram,oid);
  return pVVar1;
}


```

## 本轮：vm_allocateValues — procedure_pseudo_code

Evidence ID：`ev_ff4d127c9992f9d1237d84027810ffe2d4684aa1253a8ec9900bb7d899010ce3`

```c

Vm_Value_T * vm_allocateValues(int n)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  Vm_Value_T *pVVar3;
  Vm_Value_T *pVVar4;
  int i;
  
                    /* Unresolved local var: Vm_Value_T * p@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Value_T * pValues@[DW_OP_reg3(EBX)] */
  pVVar1 = pdb_allocateMemory(n * 0x18,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",0xd5);
  if ((pVVar1 != (Vm_Value_T *)0x0) && (0 < n)) {
    i = n;
    pVVar3 = pVVar1;
    do {
      while( true ) {
        pVVar4 = pVVar3;
        for (iVar2 = 6; iVar2 != 0; iVar2 = iVar2 + -1) {
          pVVar4->ref = 0;
          pVVar4 = (Vm_Value_T *)&pVVar4->type;
        }
        pVVar3->type = '\0';
        pVVar3->attrib = '\0';
        pVVar3->ref = 1;
        vm_totalValues = vm_totalValues + 1;
        if (vm_allValues == (Vm_Value_T *)0x0) break;
        pVVar3->pPrev = (Vm_Value *)0x0;
        pVVar3->pNext = vm_allValues;
        vm_allValues->pPrev = pVVar3;
        vm_allValues = pVVar3;
        pVVar3->pThis = pVVar3;
        pVVar3 = pVVar3 + 1;
        i = i + -1;
        if (i == 0) {
          return pVVar1;
        }
      }
      vm_allValues = pVVar3;
      pVVar3->pPrev = (Vm_Value *)0x0;
      pVVar3->pNext = (Vm_Value *)0x0;
      pVVar3->pThis = pVVar3;
      pVVar3 = pVVar3 + 1;
      i = i + -1;
    } while (i != 0);
  }
  return pVVar1;
}


```

## 复用：vm_gatherProgramFunctionTable — procedure_assembly

Evidence ID：`ev_2b87bd494b66569995a2fd20b9c75f34d7fbc63d63d1b161f3aa5e6ed6113c16`

```asm
0x80a31bc: PUSH EBP
0x80a31bd: MOV EBP, ESP
0x80a31bf: PUSH EDI
0x80a31c0: PUSH ESI
0x80a31c1: PUSH EBX
0x80a31c2: SUB ESP, 0x2c
0x80a31c5: MOV EAX, dword ptr [EBP + 0x8]
0x80a31c8: MOV CX, word ptr [EAX + 0x14]
0x80a31cc: CMP CX, 0x1
0x80a31d0: MOV word ptr [EBP + -0xe], 0x1
0x80a31d6: JMP 0x080a31e0
0x80a31d8: INC word ptr [EBP + -0xe]
0x80a31dc: CMP CX, word ptr [EBP + -0xe]
0x80a31e0: JLE 0x080a3255
0x80a31e2: MOVSX EAX, word ptr [EBP + -0xe]
0x80a31e6: MOV EBX, dword ptr [EBP + 0x8]
0x80a31e9: LEA EAX, [EAX + EAX*0x2]
0x80a31ec: MOV EDX, dword ptr [EBX + 0x18]
0x80a31ef: LEA EAX, [EDX + EAX*0x4]
0x80a31f2: MOV dword ptr [EBP + -0x30], EAX
0x80a31f5: TEST byte ptr [EAX + 0x4], 0x1
0x80a31f9: JZ 0x080a31d8
0x80a31fb: MOV EBX, dword ptr [EAX]
0x80a31fd: XOR EDI, EDI
0x80a31ff: CMP word ptr [EBX + 0x10], 0x0
0x80a3204: JLE 0x080a31d8
0x80a3206: MOV ESI, ESI
0x80a3208: MOV ESI, dword ptr [EBP + -0x30]
0x80a320b: MOVSX ECX, DI
0x80a320e: MOVSX EAX, word ptr [ESI + 0x6]
0x80a3212: LEA EAX, [ECX + EAX*0x1]
0x80a3215: MOV dword ptr [EBP + -0x28], EAX
0x80a3218: MOV EAX, dword ptr [EBP + 0x8]
0x80a321b: MOV EDX, dword ptr [EBP + -0x28]
0x80a321e: MOV EAX, dword ptr [EAX + 0x2c]
0x80a3221: MOV dword ptr [EBP + -0x2c], EAX
0x80a3224: LEA EAX, [EAX + EDX*0x8]
0x80a3227: MOV EDX, dword ptr [EBX + 0x2c]
0x80a322a: LEA ECX, [EDX + ECX*0x8]
0x80a322d: MOV DX, word ptr [EBP + -0xe]
0x80a3231: ADD DX, word ptr [ECX + 0x4]
0x80a3235: MOV word ptr [EAX + 0x4], DX
0x80a3239: MOV EDX, dword ptr [ECX]
0x80a323b: MOV dword ptr [EAX], EDX
0x80a323d: MOV DL, byte ptr [ECX + 0x6]
0x80a3240: OR byte ptr [EAX + 0x6], DL
0x80a3243: INC EDI
0x80a3244: MOV EBX, dword ptr [ESI]
0x80a3246: CMP word ptr [EBX + 0x10], DI
0x80a324a: JG 0x080a3208
0x80a324c: MOV EBX, dword ptr [EBP + 0x8]
0x80a324f: MOV CX, word ptr [EBX + 0x14]
0x80a3253: JMP 0x080a31d8
0x80a3255: MOV ESI, dword ptr [EBP + 0x8]
0x80a3258: MOV EAX, dword ptr [ESI + 0x1c]
0x80a325b: MOVSX EDX, CX
0x80a325e: MOV word ptr [EAX + EDX*0x2 + -0x2], 0x0
0x80a3265: MOV AX, word ptr [ESI + 0x14]
0x80a3269: CMP AX, 0x1
0x80a326d: JLE 0x080a3289
0x80a326f: MOV EDX, dword ptr [ESI + 0x18]
0x80a3272: DEC EAX
0x80a3273: ADD EDX, 0xc
0x80a3276: MOV word ptr [EBP + -0xe], AX
0x80a327a: MOV ESI, ESI
0x80a327c: MOV EAX, dword ptr [EDX]
0x80a327e: INC dword ptr [EAX]
0x80a3280: ADD EDX, 0xc
0x80a3283: DEC word ptr [EBP + -0xe]
0x80a3287: JNZ 0x080a327c
0x80a3289: MOV EAX, dword ptr [EBP + 0x8]
0x80a328c: MOV SI, word ptr [EAX + 0x10]
0x80a3290: OR byte ptr [EAX + 0x8], 0x1
0x80a3294: CMP SI, 0x0
0x80a3298: MOV word ptr [EBP + -0x10], 0x0
0x80a329e: JMP 0x080a32a8
0x80a32a0: INC word ptr [EBP + -0x10]
0x80a32a4: CMP SI, word ptr [EBP + -0x10]
0x80a32a8: JLE 0x080a3487
0x80a32ae: MOV ECX, dword ptr [EBP + 0x8]
0x80a32b1: MOVSX EAX, word ptr [EBP + -0x10]
0x80a32b5: MOV EDX, dword ptr [ECX + 0x2c]
0x80a32b8: LEA EAX, [EDX + EAX*0x8]
0x80a32bb: MOV dword ptr [EBP + -0x14], EAX
0x80a32be: MOV EAX, dword ptr [EAX]
0x80a32c0: TEST EAX, EAX
0x80a32c2: JZ 0x080a32a0
0x80a32c4: MOV EAX, dword ptr [EAX + 0x14]
0x80a32c7: TEST EAX, EAX
0x80a32c9: JNZ 0x080a32a0
0x80a32cb: MOV EBX, dword ptr [EBP + -0x10]
0x80a32ce: DEC BX
0x80a32d0: JNS 0x080a32da
0x80a32d2: JMP 0x080a3323
0x80a32d4: MOV ESI, dword ptr [EBP + 0x8]
0x80a32d7: MOV EDX, dword ptr [ESI + 0x2c]
0x80a32da: MOVSX EAX, BX
0x80a32dd: LEA EDI, [EDX + EAX*0x8]
0x80a32e0: MOV ESI, dword ptr [EDI]
0x80a32e2: TEST ESI, ESI
0x80a32e4: JZ 0x080a3318
0x80a32e6: MOV EAX, dword ptr [ESI + 0x14]
0x80a32e9: TEST EAX, EAX
0x80a32eb: JZ 0x080a3318
0x80a32ed: TEST byte ptr [ESI + 0x8], 0x4
0x80a32f1: JNZ 0x080a3318
0x80a32f3: TEST byte ptr [EDI + 0x6], 0x1
0x80a32f7: JNZ 0x080a3318
0x80a32f9: MOV EAX, dword ptr [EBP + -0x14]
0x80a32fc: SUB ESP, 0x8
0x80a32ff: MOV EAX, dword ptr [EAX]
0x80a3301: PUSH dword ptr [EAX]
0x80a3303: PUSH dword ptr [ESI]
0x80a3305: MOV dword ptr [EBP + -0x18], EAX
0x80a3308: CALL 0x0804b064
0x80a330d: ADD ESP, 0x10
0x80a3310: TEST EAX, EAX
0x80a3312: JZ 0x080a3400
0x80a3318: DEC BX
0x80a331a: JNS 0x080a32d4
0x80a331c: MOV EDX, dword ptr [EBP + 0x8]
0x80a331f: MOV SI, word ptr [EDX + 0x10]
0x80a3323: MOV EBX, dword ptr [EBP + -0x10]
0x80a3326: MOV ESI, ESI
0x80a3328: INC EBX
0x80a3329: CMP SI, BX
0x80a332c: JLE 0x080a32a0
0x80a3332: MOV ECX, dword ptr [EBP + 0x8]
0x80a3335: MOVSX EDX, BX
0x80a3338: MOV EAX, dword ptr [ECX + 0x2c]
0x80a333b: LEA EDX, [EAX + EDX*0x8]
0x80a333e: MOV EDI, dword ptr [EDX]
0x80a3340: TEST EDI, EDI
0x80a3342: MOV dword ptr [EBP + -0x20], EDX
0x80a3345: JZ 0x080a3328
0x80a3347: MOV EAX, dword ptr [EDI + 0x14]
0x80a334a: TEST EAX, EAX
0x80a334c: JZ 0x080a3328
0x80a334e: TEST byte ptr [EDI + 0x8], 0x4
0x80a3352: JNZ 0x080a3328
0x80a3354: TEST byte ptr [EDX + 0x6], 0x1
0x80a3358: JNZ 0x080a3328
0x80a335a: MOV EAX, dword ptr [EBP + -0x14]
0x80a335d: SUB ESP, 0x8
0x80a3360: MOV EAX, dword ptr [EAX]
0x80a3362: PUSH dword ptr [EAX]
0x80a3364: PUSH dword ptr [EDI]
0x80a3366: MOV dword ptr [EBP + -0x24], EAX
0x80a3369: CALL 0x0804b064
0x80a336e: ADD ESP, 0x10
0x80a3371: TEST EAX, EAX
0x80a3373: JNZ 0x080a3328
0x80a3375: MOV EAX, dword ptr [EBP + -0x24]
0x80a3378: XOR ESI, ESI
0x80a337a: MOV ECX, dword ptr [EDI + 0x30]
0x80a337d: MOV EDX, dword ptr [EAX + 0x30]
0x80a3380: JMP 0x080a33b8
0x80a3384: TEST EDX, EDX
0x80a3386: JZ 0x080a33c0
0x80a3388: MOV AL, byte ptr [EDX + 0x5]
0x80a338b: CMP byte ptr [ECX + 0x5], AL
0x80a338e: JNZ 0x080a3547
0x80a3394: MOV AL, byte ptr [EDX + 0x6]
0x80a3397: CMP byte ptr [ECX + 0x6], AL
0x80a339a: JNZ 0x080a353a
0x80a33a0: MOV EAX, dword ptr [ECX + 0xc]
0x80a33a3: TEST EAX, EAX
0x80a33a5: JZ 0x080a33b2
0x80a33a7: MOV EAX, dword ptr [EDX + 0xc]
0x80a33aa: TEST EAX, EAX
0x80a33ac: JNZ 0x080a3554
0x80a33b2: MOV ECX, dword ptr [ECX + 0x10]
0x80a33b5: MOV EDX, dword ptr [EDX + 0x10]
0x80a33b8: TEST ECX, ECX
0x80a33ba: JNZ 0x080a3384
0x80a33bc: TEST EDX, EDX
0x80a33be: JZ 0x080a33d5
0x80a33c0: SUB ESP, 0xc
0x80a33c3: PUSH 0x82afaac
0x80a33c8: CALL 0x080ff80c
0x80a33cd: ADD ESP, 0x10
0x80a33d0: MOV ESI, 0x1
0x80a33d5: MOV AL, byte ptr [EDI + 0xd]
0x80a33d8: MOV EDX, dword ptr [EBP + -0x24]
0x80a33db: CMP byte ptr [EDX + 0xd], AL
0x80a33de: JZ 0x080a34f6
0x80a33e4: SUB ESP, 0xc
0x80a33e7: PUSH 0x82b0063
0x80a33ec: CALL 0x080ff80c
0x80a33f1: ADD ESP, 0x10
0x80a33f4: MOV ECX, dword ptr [EBP + 0x8]
0x80a33f7: MOV SI, word ptr [ECX + 0x10]
0x80a33fb: JMP 0x080a3328
0x80a3400: MOV EAX, dword ptr [EBP + -0x18]
0x80a3403: MOV ECX, dword ptr [ESI + 0x30]
0x80a3406: MOV EDX, dword ptr [EAX + 0x30]
0x80a3409: MOV dword ptr [EBP + -0x1c], 0x0
0x80a3410: JMP 0x080a3448
0x80a3414: TEST EDX, EDX
0x80a3416: JZ 0x080a3450
0x80a3418: MOV AL, byte ptr [EDX + 0x5]
0x80a341b: CMP byte ptr [ECX + 0x5], AL
0x80a341e: JNZ 0x080a34c7
0x80a3424: MOV AL, byte ptr [EDX + 0x6]
0x80a3427: CMP byte ptr [ECX + 0x6], AL
0x80a342a: JNZ 0x080a34d1
0x80a3430: MOV EAX, dword ptr [ECX + 0xc]
0x80a3433: TEST EAX, EAX
0x80a3435: JZ 0x080a3442
0x80a3437: MOV EAX, dword ptr [EDX + 0xc]
0x80a343a: TEST EAX, EAX
0x80a343c: JNZ 0x080a34de
0x80a3442: MOV ECX, dword ptr [ECX + 0x10]
0x80a3445: MOV EDX, dword ptr [EDX + 0x10]
0x80a3448: TEST ECX, ECX
0x80a344a: JNZ 0x080a3414
0x80a344c: TEST EDX, EDX
0x80a344e: JZ 0x080a3467
0x80a3450: SUB ESP, 0xc
0x80a3453: PUSH 0x82afaac
0x80a3458: CALL 0x080ff80c
0x80a345d: ADD ESP, 0x10
0x80a3460: MOV dword ptr [EBP + -0x1c], 0x1
0x80a3467: MOV AL, byte ptr [ESI + 0xd]
0x80a346a: MOV EDX, dword ptr [EBP + -0x18]
0x80a346d: CMP byte ptr [EDX + 0xd], AL
0x80a3470: JZ 0x080a348f
0x80a3472: SUB ESP, 0xc
0x80a3475: PUSH 0x82b0063
0x80a347a: CALL 0x080ff80c
0x80a347f: ADD ESP, 0x10
0x80a3482: JMP 0x080a3318
0x80a3487: LEA ESP, [EBP + -0xc]
0x80a348a: POP EBX
0x80a348b: POP ESI
0x80a348c: POP EDI
0x80a348d: LEAVE
0x80a348e: RET
0x80a348f: MOV EAX, dword ptr [EBP + -0x1c]
0x80a3492: TEST EAX, EAX
0x80a3494: JNZ 0x080a3318
0x80a349a: MOV ECX, dword ptr [EBP + -0x14]
0x80a349d: CMP word ptr [ECX + 0x4], 0x0
0x80a34a2: JZ 0x080a356c
0x80a34a8: MOV EBX, dword ptr [EBP + -0x14]
0x80a34ab: MOV EAX, dword ptr [EDI]
0x80a34ad: OR byte ptr [EBX + 0x6], 0x1
0x80a34b1: MOV dword ptr [EBX], EAX
0x80a34b3: MOV AX, word ptr [EDI + 0x4]
0x80a34b7: MOV word ptr [EBX + 0x4], AX
0x80a34bb: MOV EAX, dword ptr [EBP + 0x8]
0x80a34be: MOV SI, word ptr [EAX + 0x10]
0x80a34c2: JMP 0x080a32a0
0x80a34c7: SUB ESP, 0xc
0x80a34ca: PUSH 0x82b007e
0x80a34cf: JMP 0x080a3458
0x80a34d1: SUB ESP, 0xc
0x80a34d4: PUSH 0x82afacc
0x80a34d9: JMP 0x080a3458
0x80a34de: MOVSX EAX, word ptr [ECX + 0x8]
0x80a34e2: SUB ESP, 0x8
0x80a34e5: INC EAX
0x80a34e6: PUSH EAX
0x80a34e7: PUSH 0x82afaf0
0x80a34ec: CALL 0x080ff97c
0x80a34f1: JMP 0x080a345d
0x80a34f6: TEST ESI, ESI
0x80a34f8: JNZ 0x080a33f4
0x80a34fe: MOV EBX, dword ptr [EBP + -0x14]
0x80a3501: CMP word ptr [EBX + 0x4], 0x0
0x80a3506: JNZ 0x080a3518
0x80a3508: PUSH ECX
0x80a3509: PUSH 0x16
0x80a350b: PUSH dword ptr [EBX]
0x80a350d: PUSH dword ptr [EBP + 0x8]
0x80a3510: CALL 0x08095c88
0x80a3515: ADD ESP, 0x10
0x80a3518: MOV ESI, dword ptr [EBP + -0x20]
0x80a351b: MOV EAX, dword ptr [ESI]
0x80a351d: MOV EDX, dword ptr [EBP + -0x14]
0x80a3520: OR byte ptr [EDX + 0x6], 0x1
0x80a3524: MOV dword ptr [EDX], EAX
0x80a3526: MOV AX, word ptr [ESI + 0x4]
0x80a352a: MOV word ptr [EDX + 0x4], AX
0x80a352e: MOV ECX, dword ptr [EBP + 0x8]
0x80a3531: MOV SI, word ptr [ECX + 0x10]
0x80a3535: JMP 0x080a32a0
0x80a353a: SUB ESP, 0xc
0x80a353d: PUSH 0x82afacc
0x80a3542: JMP 0x080a33c8
0x80a3547: SUB ESP, 0xc
0x80a354a: PUSH 0x82b007e
0x80a354f: JMP 0x080a33c8
0x80a3554: MOVSX EAX, word ptr [ECX + 0x8]
0x80a3558: SUB ESP, 0x8
0x80a355b: INC EAX
0x80a355c: PUSH EAX
0x80a355d: PUSH 0x82afaf0
0x80a3562: CALL 0x080ff97c
0x80a3567: JMP 0x080a33cd
0x80a356c: PUSH EAX
0x80a356d: PUSH 0x16
0x80a356f: PUSH dword ptr [ECX]
0x80a3571: PUSH dword ptr [EBP + 0x8]
0x80a3574: CALL 0x08095c88
0x80a3579: ADD ESP, 0x10
0x80a357c: JMP 0x080a34a8
```

## 复用：vm_createProgram — procedure_assembly

Evidence ID：`ev_7c339189f2227b0560db98446dc83bd0520ad53bb7da51083b39a547badba203`

```asm
0x80b13f4: PUSH EBP
0x80b13f5: MOV EBP, ESP
0x80b13f7: PUSH EDI
0x80b13f8: PUSH ESI
0x80b13f9: PUSH EBX
0x80b13fa: SUB ESP, 0x18
0x80b13fd: MOV EDI, dword ptr [EBP + 0x8]
0x80b1400: MOV dword ptr [EBP + -0x10], 0x0
0x80b1407: PUSH EAX
0x80b1408: MOV EBX, EDX
0x80b140a: CALL 0x0808651c
0x80b140f: ADD ESP, 0x10
0x80b1412: TEST EAX, EAX
0x80b1414: MOV ESI, EAX
0x80b1416: JZ 0x080b144b
0x80b1418: LEA EAX, [EBP + -0x10]
0x80b141b: PUSH EAX
0x80b141c: PUSH ESI
0x80b141d: PUSH EDI
0x80b141e: PUSH EBX
0x80b141f: CALL 0x080985e0
0x80b1424: ADD ESP, 0x10
0x80b1427: TEST EAX, EAX
0x80b1429: JZ 0x080b1464
0x80b142b: PUSH EDX
0x80b142c: PUSH EAX
0x80b142d: PUSH EDI
0x80b142e: PUSH 0x82b0e7c
0x80b1433: CALL 0x08097acc
0x80b1438: ADD ESP, 0x10
0x80b143b: TEST ESI, ESI
0x80b143d: JZ 0x080b144b
0x80b143f: SUB ESP, 0xc
0x80b1442: PUSH ESI
0x80b1443: CALL 0x080862a8
0x80b1448: ADD ESP, 0x10
0x80b144b: MOV EAX, dword ptr [EBP + -0x10]
0x80b144e: XOR ESI, ESI
0x80b1450: TEST EAX, EAX
0x80b1452: JNZ 0x080b14e0
0x80b1458: LEA ESP, [EBP + -0xc]
0x80b145b: POP EBX
0x80b145c: MOV EAX, ESI
0x80b145e: POP ESI
0x80b145f: POP EDI
0x80b1460: LEAVE
0x80b1461: RET
0x80b1464: LEA EAX, [ESI + 0xa]
0x80b1467: PUSH EAX
0x80b1468: PUSH 0x1
0x80b146a: PUSH 0x82d0dd9
0x80b146f: PUSH ESI
0x80b1470: CALL 0x08086d68
0x80b1475: ADD ESP, 0x10
0x80b1478: TEST EAX, EAX
0x80b147a: MOV EBX, EAX
0x80b147c: JZ 0x080b151a
0x80b1482: MOV AX, word ptr [EAX + 0x8]
0x80b1486: OR EAX, 0x1
0x80b1489: TEST AL, 0x4
0x80b148b: MOV word ptr [EBX + 0x8], AX
0x80b148f: JZ 0x080b14f6
0x80b1491: MOV EDI, dword ptr [EBX + 0x14]
0x80b1494: TEST EDI, EDI
0x80b1496: JZ 0x080b150d
0x80b1498: LEA EAX, [ESI + 0xc]
0x80b149b: PUSH EAX
0x80b149c: PUSH 0x1
0x80b149e: PUSH 0x82b0fff
0x80b14a3: PUSH ESI
0x80b14a4: CALL 0x08086d68
0x80b14a9: ADD ESP, 0x10
0x80b14ac: TEST EAX, EAX
0x80b14ae: MOV EBX, EAX
0x80b14b0: JZ 0x080b1549
0x80b14b6: TEST byte ptr [EAX + 0x8], 0x4
0x80b14ba: JZ 0x080b1525
0x80b14bc: MOV EBX, dword ptr [EBX + 0x14]
0x80b14bf: TEST EBX, EBX
0x80b14c1: JZ 0x080b153c
0x80b14c3: SUB ESP, 0xc
0x80b14c6: MOV dword ptr [ESI], 0x0
0x80b14cc: PUSH ESI
0x80b14cd: CALL 0x080c3f1c
0x80b14d2: MOV EAX, dword ptr [EBP + -0x10]
0x80b14d5: ADD ESP, 0x10
0x80b14d8: TEST EAX, EAX
0x80b14da: JZ 0x080b1458
0x80b14e0: SUB ESP, 0xc
0x80b14e3: PUSH EAX
0x80b14e4: CALL 0x08089094
0x80b14e9: ADD ESP, 0x10
0x80b14ec: LEA ESP, [EBP + -0xc]
0x80b14ef: POP EBX
0x80b14f0: MOV EAX, ESI
0x80b14f2: POP ESI
0x80b14f3: POP EDI
0x80b14f4: LEAVE
0x80b14f5: RET
0x80b14f6: SUB ESP, 0xc
0x80b14f9: PUSH 0x82b0ea4
0x80b14fe: CALL 0x08097e34
0x80b1503: OR word ptr [EBX + 0x8], 0x4
0x80b1508: ADD ESP, 0x10
0x80b150b: JMP 0x080b1491
0x80b150d: SUB ESP, 0xc
0x80b1510: PUSH 0x82b0eec
0x80b1515: JMP 0x080b1433
0x80b151a: MOV word ptr [ESI + 0xa], 0xffff
0x80b1520: JMP 0x080b1498
0x80b1525: SUB ESP, 0xc
0x80b1528: PUSH 0x82b0f20
0x80b152d: CALL 0x08097e34
0x80b1532: OR word ptr [EBX + 0x8], 0x4
0x80b1537: ADD ESP, 0x10
0x80b153a: JMP 0x080b14bc
0x80b153c: SUB ESP, 0xc
0x80b153f: PUSH 0x82b0f68
0x80b1544: JMP 0x080b1433
0x80b1549: MOV word ptr [ESI + 0xc], 0xffff
0x80b154f: JMP 0x080b14c3
```

## 复用：vm_createProgram — procedure_pseudo_code

Evidence ID：`ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d`

```c

Vm_Program_T * __regparm2 vm_createProgram(char *pProgramName,int fileFd,char *pInFileName)

{
  Vm_Node_T **ppVVar1;
  ushort uVar2;
  Vm_Program *pProgram;
  Vm_Status VVar3;
  Vm_Fun *pVVar4;
  Vm_Program_T *pProgram_00;
  Vm_Program_T *pVVar5;
  Vm_Program_T *extraout_EAX;
  Vm_Index_T *extraout_EDX;
  char *str;
  Vm_Index_T *pFunOffset;
  Vm_Scratchpad_Info_T *pScratchpad;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ret@[???]
                       Unresolved local var: int errCode@[DW_OP_reg0(EAX)] */
  pScratchpad = (Vm_Scratchpad_Info_T *)0x0;
  pProgram = vm_buildProgramHead(pProgramName);
  if (pProgram != (Vm_Program *)0x0) {
    VVar3 = vm_compile(fileFd,pInFileName,pProgram,&pScratchpad);
    if (VVar3 == VM_OK) {
      pFunOffset = &pProgram->createFunOffset;
      VVar3 = ~VM_CAN_NOT_CREATE_MUTEX;
      pInFileName = "create";
      pVVar4 = vm_findProgramFunctionByCStr(pProgram,"create",1,pFunOffset);
      if (pVVar4 == (Vm_Fun *)0x0) {
        pProgram->createFunOffset = -1;
      }
      else {
        uVar2 = pVVar4->attrib;
        pVVar4->attrib = uVar2 | 1;
        if ((uVar2 & 4) == 0) {
          vm_stdprintf("Warning: Construction function \"create\" must be a private function.\n");
          pVVar4->attrib = pVVar4->attrib | 4;
        }
        if ((pVVar4->codeSegment).nodes == (Vm_Node_T *)0x0) {
          str = "Error: Construction function is not implemented.\n";
          goto LAB_080b1433;
        }
      }
      pFunOffset = &pProgram->destructFunOffset;
      VVar3 = ~VM_CAN_NOT_CREATE_MUTEX;
      pInFileName = "destruct";
      pProgram_00 = (Vm_Program_T *)vm_findProgramFunctionByCStr(pProgram,"destruct",1,pFunOffset);
      if (pProgram_00 == (Vm_Program_T *)0x0) {
        pProgram->destructFunOffset = -1;
      }
      else {
        pVVar5 = pProgram_00;
        if ((*(short *)&pProgram_00->attrib & 4) == 0) {
          vm_stdprintf("Warning: destruction function \"destruct\" must be a private function.\n");
          *(ushort *)&pProgram_00->attrib = *(ushort *)&pProgram_00->attrib | 4;
          pVVar5 = extraout_EAX;
        }
        ppVVar1 = (Vm_Node_T **)&pProgram_00->maxConsititutionNo;
        pProgram_00 = pVVar5;
        if (*ppVVar1 == (Vm_Node_T *)0x0) {
          str = "Error: Destruction function is not implemented.\n";
          goto LAB_080b1433;
        }
      }
      pProgram->ref = 0;
      vm_applyBreakpointsOnProgram(pProgram_00);
      goto joined_r0x080b14da;
    }
    str = "Failed to compile \"%s\", errCode = %d.\n";
    pFunOffset = extraout_EDX;
LAB_080b1433:
    vm_errprintf(str,pInFileName,VVar3,pFunOffset);
    if (pProgram != (Vm_Program *)0x0) {
      vm_destructProgramHead(pProgram);
    }
  }
  pProgram = (Vm_Program *)0x0;
joined_r0x080b14da:
  if (pScratchpad == (Vm_Scratchpad_Info_T *)0x0) {
    return pProgram;
  }
  vm_destructScratch(pScratchpad);
  return pProgram;
}


```

## 复用：vm_gatherProgramFunctionTable — procedure_pseudo_code

Evidence ID：`ev_cbeb0564e51d40f356850c8ce433d229a0063dc15122c784bcac9998341ff27d`

```c

/* WARNING: Unknown calling convention */

void vm_gatherProgramFunctionTable(Vm_Program_T *pCurrentProgram)

{
  Vm_Fun_Entry_T *pVVar1;
  Vm_Fun *pVVar2;
  Vm_Fun *pVVar3;
  short sVar4;
  int iVar5;
  short sVar6;
  Vm_Fun_Arg_T *pVVar7;
  Vm_Program_Consititution_T *pVVar8;
  Vm_Fun_Entry_T *pVVar9;
  Vm_Fun_Arg_T *pVVar10;
  short sVar11;
  Vm_Program *pVVar12;
  short sVar13;
  bool bVar14;
  bool bVar15;
  char *pcVar16;
  Vm_Program_Consititution_T *pThisConsititution;
  int notSame;
  Vm_Fun_Entry_T *pFunEntry;
  Vm_Index_T i_1;
  Vm_Index_T i;
  
                    /* Unresolved local var: Vm_Fun_Entry_T * pToFunEntry@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Fun_Entry_T * pFromFunEntry@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Index_T k@[DW_OP_reg7(EDI)] */
  sVar6 = pCurrentProgram->maxConsititutionNo;
  bVar15 = SBORROW2(sVar6,1);
  sVar13 = sVar6 + -1;
  sVar4 = 1;
  bVar14 = sVar6 == 1;
  while (!bVar14 && bVar15 == sVar13 < 0) {
    pVVar8 = pCurrentProgram->pConsititutions + sVar4;
    if ((pVVar8->attrib & 1) != 0) {
      pVVar12 = pVVar8->pProgram;
      sVar13 = 0;
      if (0 < pVVar12->maxFunctionNo) {
        do {
          pVVar1 = pCurrentProgram->pFunctions + (int)sVar13 + (int)pVVar8->baseFunOffset;
          pVVar9 = pVVar12->pFunctions + sVar13;
          pVVar1->consititutionNo = sVar4 + pVVar9->consititutionNo;
          pVVar1->pFun = pVVar9->pFun;
          pVVar1->attrib = pVVar1->attrib | pVVar9->attrib;
          sVar13 = sVar13 + 1;
          pVVar12 = pVVar8->pProgram;
        } while (sVar13 < pVVar12->maxFunctionNo);
        sVar6 = pCurrentProgram->maxConsititutionNo;
      }
    }
    sVar4 = sVar4 + 1;
    bVar15 = SBORROW2(sVar6,sVar4);
    sVar13 = sVar6 - sVar4;
    bVar14 = sVar6 == sVar4;
  }
  pCurrentProgram->initOrderList[sVar6 + -1] = 0;
  if (1 < pCurrentProgram->maxConsititutionNo) {
    pVVar8 = pCurrentProgram->pConsititutions;
    sVar4 = pCurrentProgram->maxConsititutionNo + -1;
    do {
      pVVar8 = pVVar8 + 1;
      pVVar8->pProgram->ref = pVVar8->pProgram->ref + 1;
      sVar4 = sVar4 + -1;
    } while (sVar4 != 0);
  }
                    /* Unresolved local var: Vm_Index_T k@[DW_OP_reg3(EBX)]
                       Unresolved local var: int linked@[???] */
  sVar6 = pCurrentProgram->maxFunctionNo;
  pCurrentProgram->attrib = pCurrentProgram->attrib | 1;
  bVar15 = false;
  bVar14 = sVar6 == 0;
  sVar4 = 0;
  sVar13 = sVar6;
LAB_080a32a8:
  if (bVar14 || bVar15 != sVar6 < 0) {
    return;
  }
  pVVar9 = pCurrentProgram->pFunctions;
  pVVar1 = pVVar9 + sVar4;
  if ((pVVar1->pFun != (Vm_Fun *)0x0) && ((pVVar1->pFun->codeSegment).nodes == (Vm_Node_T *)0x0)) {
    sVar11 = sVar4 + -1;
    sVar6 = sVar4;
    if (-1 < sVar11) {
      do {
        pVVar9 = pVVar9 + sVar11;
        pVVar2 = pVVar9->pFun;
        if ((((pVVar2 != (Vm_Fun *)0x0) && ((pVVar2->codeSegment).nodes != (Vm_Node_T *)0x0)) &&
            ((pVVar2->attrib & 4) == 0)) && ((pVVar9->attrib & 1) == 0)) {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
          pVVar3 = pVVar1->pFun;
          iVar5 = strcmp(pVVar2->name,pVVar3->name);
          if (iVar5 == 0) {
            pVVar10 = pVVar3->pArgs;
            bVar14 = false;
                    /* Unresolved local var: Vm_Fun_Arg_T * pFunArg@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Fun_Arg_T * pDefArg@[DW_OP_reg2(EDX)] */
            for (pVVar7 = pVVar2->pArgs; pVVar7 != (Vm_Fun_Arg_T *)0x0; pVVar7 = pVVar7->pNext) {
              if (pVVar10 == (Vm_Fun_Arg_T *)0x0) goto LAB_080a3450;
              if (pVVar7->type != pVVar10->type) {
                pcVar16 = "Argument type is mismatched.";
                goto LAB_080a3458;
              }
              if (pVVar7->varAttrib != pVVar10->varAttrib) {
                pcVar16 = "Argument attribute is mismatched.";
                goto LAB_080a3458;
              }
              if ((pVVar7->pDefaultValue != (Vm_XNode *)0x0) &&
                 (pVVar10->pDefaultValue != (Vm_XNode *)0x0)) {
                yyerrors("Default value of argument %d is already declared.\n",pVVar7->no + 1);
                goto LAB_080a345d;
              }
              pVVar10 = pVVar10->pNext;
            }
            if (pVVar10 != (Vm_Fun_Arg_T *)0x0) {
LAB_080a3450:
              pcVar16 = "Argument number is mismatched.";
LAB_080a3458:
              yyerror(pcVar16);
LAB_080a345d:
              bVar14 = true;
            }
            if (pVVar3->retType == pVVar2->retType) {
              if (!bVar14) {
                if (pVVar1->consititutionNo == 0) {
                  vm_freeFun(pCurrentProgram,pVVar1->pFun,0x16);
                }
                pVVar2 = pVVar9->pFun;
                pVVar1->attrib = pVVar1->attrib | 1;
                pVVar1->pFun = pVVar2;
                pVVar1->consititutionNo = pVVar9->consititutionNo;
                sVar13 = pCurrentProgram->maxFunctionNo;
                goto LAB_080a32a0;
              }
            }
            else {
              yyerror("Return type is mismatched.");
            }
          }
        }
        sVar11 = sVar11 + -1;
        if (sVar11 < 0) goto code_r0x080a331c;
        pVVar9 = pCurrentProgram->pFunctions;
      } while( true );
    }
    goto LAB_080a3328;
  }
  goto LAB_080a32a0;
code_r0x080a331c:
  sVar13 = pCurrentProgram->maxFunctionNo;
LAB_080a3328:
  while (sVar6 = sVar6 + 1, sVar6 < sVar13) {
    pVVar9 = pCurrentProgram->pFunctions + sVar6;
    pVVar2 = pVVar9->pFun;
    if (((pVVar2 != (Vm_Fun *)0x0) && ((pVVar2->codeSegment).nodes != (Vm_Node_T *)0x0)) &&
       (((pVVar2->attrib & 4) == 0 && ((pVVar9->attrib & 1) == 0)))) {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
      pVVar3 = pVVar1->pFun;
      iVar5 = strcmp(pVVar2->name,pVVar3->name);
      if (iVar5 == 0) {
                    /* Unresolved local var: Vm_Fun_Arg_T * pFunArg@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Fun_Arg_T * pDefArg@[DW_OP_reg2(EDX)]
                       Unresolved local var: int notSame@[DW_OP_reg6(ESI)] */
        bVar14 = false;
        pVVar10 = pVVar3->pArgs;
        for (pVVar7 = pVVar2->pArgs; pVVar7 != (Vm_Fun_Arg_T *)0x0; pVVar7 = pVVar7->pNext) {
          if (pVVar10 == (Vm_Fun_Arg_T *)0x0) goto LAB_080a33c0;
          if (pVVar7->type != pVVar10->type) {
            pcVar16 = "Argument type is mismatched.";
            goto LAB_080a33c8;
          }
          if (pVVar7->varAttrib != pVVar10->varAttrib) {
            pcVar16 = "Argument attribute is mismatched.";
            goto LAB_080a33c8;
          }
          if ((pVVar7->pDefaultValue != (Vm_XNode *)0x0) &&
             (pVVar10->pDefaultValue != (Vm_XNode *)0x0)) {
            yyerrors("Default value of argument %d is already declared.\n",pVVar7->no + 1);
            goto LAB_080a33cd;
          }
          pVVar10 = pVVar10->pNext;
        }
        if (pVVar10 != (Vm_Fun_Arg_T *)0x0) {
LAB_080a33c0:
          pcVar16 = "Argument number is mismatched.";
LAB_080a33c8:
          yyerror(pcVar16);
LAB_080a33cd:
          bVar14 = true;
        }
        if (pVVar3->retType == pVVar2->retType) {
          if (!bVar14) {
            if (pVVar1->consititutionNo == 0) {
              vm_freeFun(pCurrentProgram,pVVar1->pFun,0x16);
            }
            pVVar2 = pVVar9->pFun;
            pVVar1->attrib = pVVar1->attrib | 1;
            pVVar1->pFun = pVVar2;
            pVVar1->consititutionNo = pVVar9->consititutionNo;
            sVar13 = pCurrentProgram->maxFunctionNo;
            break;
          }
        }
        else {
          yyerror("Return type is mismatched.");
        }
        sVar13 = pCurrentProgram->maxFunctionNo;
      }
    }
  }
LAB_080a32a0:
  sVar4 = sVar4 + 1;
  bVar15 = SBORROW2(sVar13,sVar4);
  sVar6 = sVar13 - sVar4;
  bVar14 = sVar13 == sVar4;
  goto LAB_080a32a8;
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

## 复用：vm_destructObject — procedure_pseudo_code

Evidence ID：`ev_196547cb37731a3632d63fbc4a62d83a57ac5f788fa57a3ed95657b8a8346c69`

```c

void vm_destructObject(Vm_Object_T *pObject)

{
  Vm_Program_T *pVVar1;
  Vm_Object_T *pVVar2;
  Vm_Value *pVVar3;
  Vm_Object **ppVVar4;
  int iVar5;
  int iVar6;
  Vm_Program_T *pProgram;
  
  if (pObject->execRef < 1) {
    vm_makeObjectAlone(pObject);
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
    if (pObject->pBindValue != (Vm_Value_T *)0x0) {
      vm_unrefValue(pObject->pBindValue);
      pObject->pBindValue = (Vm_Value *)0x0;
    }
    pVVar3 = pObject->pObjectValues;
    if (pVVar3 != (Vm_Value *)0x0) {
      pVVar1 = pObject->pProgram;
      iVar6 = 0;
      iVar5 = 0;
      if (0 < pVVar1->maxVariableNo) {
        while( true ) {
          vm_destructStaticValue((Vm_Value_T *)((int)&pVVar3->ref + iVar5));
          iVar6 = iVar6 + 1;
          iVar5 = iVar5 + 0x18;
          if (pVVar1->maxVariableNo <= iVar6) break;
          pVVar3 = pObject->pObjectValues;
        }
      }
      pdb_freeMemory(pObject->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1ce)
      ;
      pObject->pObjectValues = (Vm_Value *)0x0;
    }
                    /* Unresolved local var: Vm_Object_T * * ppHash@[DW_OP_reg2(EDX)]
                       Unresolved local var: int idx@[???] */
    pVVar1 = pObject->pProgram;
    ppVVar4 = vm_objectHashTable + ((uint)pVVar1->strName->hash + pObject->oid & 0x3fff);
    for (pVVar2 = *ppVVar4; pVVar2 != (Vm_Object_T *)0x0; pVVar2 = pVVar2->pNextHashObject) {
      if (pVVar2 == pObject) {
        *ppVVar4 = pObject->pNextHashObject;
        break;
      }
      ppVVar4 = &pVVar2->pNextHashObject;
    }
    if (pObject->ref == 0) {
      iVar5 = pVVar1->ref + -1;
      pVVar1->ref = iVar5;
      if (iVar5 < 1) {
        vm_freeCodeSegment(pVVar1,&pVVar1->initCodeSegment);
        if (vm_gatheringCallStackInfo != 0) {
          vm_enterPPWholeCriticalSection();
        }
        vm_destructProgramHead(pVVar1);
        if (vm_gatheringCallStackInfo != 0) {
          vm_leavePPWholeCriticalSection();
        }
      }
      pdb_freeMemory(pObject,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1f5);
    }
    else {
      pObject->flags = pObject->flags | 0x8000;
    }
  }
  else {
    pObject->flags = pObject->flags | 0x4000;
  }
  return;
}


```

## 复用：vm_destructObjectValues — procedure_assembly

Evidence ID：`ev_d50f92fab09f479b0532c35c9c57bb161f0484bdebcf5d1545ecb10b7ace15be`

```asm
0x80b11a8: PUSH EBP
0x80b11a9: MOV EBP, ESP
0x80b11ab: PUSH EDI
0x80b11ac: PUSH ESI
0x80b11ad: PUSH EBX
0x80b11ae: SUB ESP, 0xc
0x80b11b1: MOV EDI, dword ptr [EBP + 0x8]
0x80b11b4: MOV EAX, dword ptr [EDI + 0x1c]
0x80b11b7: TEST EAX, EAX
0x80b11b9: JNZ 0x080b1240
0x80b11bf: MOV EAX, dword ptr [EDI + 0x24]
0x80b11c2: TEST EAX, EAX
0x80b11c4: JZ 0x080b11f8
0x80b11c6: MOV EDX, dword ptr [EDI + 0x20]
0x80b11c9: XOR ESI, ESI
0x80b11cb: XOR EBX, EBX
0x80b11cd: CMP word ptr [EDX + 0xe], 0x0
0x80b11d2: MOV dword ptr [EBP + -0x10], EDX
0x80b11d5: JG 0x080b1203
0x80b11d7: PUSH 0x1ce
0x80b11dc: PUSH 0x82b0e28
0x80b11e1: PUSH 0x82a376b
0x80b11e6: PUSH dword ptr [EDI + 0x24]
0x80b11e9: CALL 0x080ceb94
0x80b11ee: MOV dword ptr [EDI + 0x24], 0x0
0x80b11f5: LEA ESI, [ESI]
0x80b11f8: LEA ESP, [EBP + -0xc]
0x80b11fb: POP EBX
0x80b11fc: POP ESI
0x80b11fd: POP EDI
0x80b11fe: LEAVE
0x80b11ff: RET
0x80b1200: MOV EAX, dword ptr [EDI + 0x24]
0x80b1203: ADD EAX, EBX
0x80b1205: SUB ESP, 0xc
0x80b1208: PUSH EAX
0x80b1209: CALL 0x0808c0b8
0x80b120e: MOV EDX, dword ptr [EBP + -0x10]
0x80b1211: INC ESI
0x80b1212: MOVSX EAX, word ptr [EDX + 0xe]
0x80b1216: ADD ESP, 0x10
0x80b1219: ADD EBX, 0x18
0x80b121c: CMP EAX, ESI
0x80b121e: JG 0x080b1200
0x80b1220: PUSH 0x1ce
0x80b1225: PUSH 0x82b0e28
0x80b122a: PUSH 0x82a376b
0x80b122f: PUSH dword ptr [EDI + 0x24]
0x80b1232: CALL 0x080ceb94
0x80b1237: MOV dword ptr [EDI + 0x24], 0x0
0x80b123e: JMP 0x080b11f8
0x80b1240: SUB ESP, 0xc
0x80b1243: PUSH EAX
0x80b1244: CALL 0x0808bfb4
0x80b1249: ADD ESP, 0x10
0x80b124c: MOV dword ptr [EDI + 0x1c], 0x0
0x80b1253: JMP 0x080b11bf
```

## 复用：vm_destructObjectValues — procedure_pseudo_code

Evidence ID：`ev_d8173d61d8d105723534cddea8b9f99fadfed21f978546de6160508e896cd5ec`

```c

void vm_destructObjectValues(Vm_Object_T *pObject)

{
  Vm_Program_T *pVVar1;
  Vm_Value *pVVar2;
  int iVar3;
  int iVar4;
  Vm_Program_T *pProgram;
  
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  if (pObject->pBindValue != (Vm_Value_T *)0x0) {
    vm_unrefValue(pObject->pBindValue);
    pObject->pBindValue = (Vm_Value *)0x0;
  }
  pVVar2 = pObject->pObjectValues;
  if (pVVar2 != (Vm_Value *)0x0) {
    pVVar1 = pObject->pProgram;
    iVar4 = 0;
    iVar3 = 0;
    if (pVVar1->maxVariableNo < 1) {
      pdb_freeMemory(pObject->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1ce)
      ;
      pObject->pObjectValues = (Vm_Value *)0x0;
    }
    else {
      while( true ) {
        vm_destructStaticValue((Vm_Value_T *)((int)&pVVar2->ref + iVar3));
        iVar4 = iVar4 + 1;
        iVar3 = iVar3 + 0x18;
        if (pVVar1->maxVariableNo <= iVar4) break;
        pVVar2 = pObject->pObjectValues;
      }
      pdb_freeMemory(pObject->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1ce)
      ;
      pObject->pObjectValues = (Vm_Value *)0x0;
    }
  }
  return;
}


```

## 复用：vm_copyValue — procedure_pseudo_code

Evidence ID：`ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33`

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
