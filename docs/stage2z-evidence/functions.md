# 阶段 2Z 原生函数证据

REA/Ghidra 12.1.4 静态结果，不是源代码或运行观察。完整 raw_result、CFG 和限制见压缩包。

## vm_checkIsRefType / procedure_assembly

Evidence: `ev_25016d21203a158ef288506e287a0e70506620d4920f2a7f1d6422a7b30343c0`

```text
0x8091750: PUSH EBP
0x8091751: MOV EBP, ESP
0x8091753: PUSH EBX
0x8091754: PUSH EBX
0x8091755: MOV ECX, dword ptr [0x086619cc]
0x809175b: TEST ECX, ECX
0x809175d: MOV EBX, dword ptr [EBP + 0x8]
0x8091760: JZ 0x080917a8
0x8091762: CALL 0x08093400
0x8091767: CMP EAX, EBX
0x8091769: JG 0x080917c4
0x809176b: MOV EDX, dword ptr [0x086619cc]
0x8091771: TEST EDX, EDX
0x8091773: JZ 0x08091814
0x8091779: DEC EDX
0x809177a: MOV dword ptr [0x086619cc], EDX
0x8091780: MOV EAX, [0x08661964]
0x8091785: LEA EDX, [EDX + EDX*0x2]
0x8091788: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x2
0x809178d: MOV EAX, [0x086619cc]
0x8091792: MOV EDX, dword ptr [0x08661964]
0x8091798: LEA EAX, [EAX + EAX*0x2]
0x809179b: MOV dword ptr [EDX + EAX*0x8 + 0x8], 0x0
0x80917a3: MOV EBX, dword ptr [EBP + -0x4]
0x80917a6: LEAVE
0x80917a7: RET
0x80917a8: SUB ESP, 0xc
0x80917ab: PUSH 0x82ad98c
0x80917b0: CALL 0x08097b24
0x80917b5: ADD ESP, 0x10
0x80917b8: CALL 0x08093400
0x80917bd: CMP EAX, EBX
0x80917bf: JLE 0x0809176b
0x80917c1: LEA ESI, [ESI]
0x80917c4: CALL 0x080933e4
0x80917c9: ADD EBX, EAX
0x80917cb: MOV EDX, dword ptr [0x086619cc]
0x80917d1: CMP EBX, EDX
0x80917d3: JL 0x08091771
0x80917d5: LEA EAX, [EBX + EBX*0x2]
0x80917d8: MOV ECX, dword ptr [0x08661964]
0x80917de: XOR EBX, EBX
0x80917e0: CMP byte ptr [ECX + EAX*0x8 + 0x4], 0x8
0x80917e5: SETZ BL
0x80917e8: TEST EDX, EDX
0x80917ea: JZ 0x0809182f
0x80917ec: LEA EAX, [EDX + -0x1]
0x80917ef: MOV [0x086619cc], EAX
0x80917f4: LEA EAX, [EAX + EAX*0x2]
0x80917f7: MOV byte ptr [ECX + EAX*0x8 + 0x4], 0x2
0x80917fc: MOV EAX, [0x086619cc]
0x8091801: MOV EDX, dword ptr [0x08661964]
0x8091807: LEA EAX, [EAX + EAX*0x2]
0x809180a: MOV dword ptr [EDX + EAX*0x8 + 0x8], EBX
0x809180e: MOV EBX, dword ptr [EBP + -0x4]
0x8091811: LEAVE
0x8091812: RET
0x8091814: SUB ESP, 0xc
0x8091817: PUSH 0x82ad98c
0x809181c: CALL 0x08097b24
0x8091821: ADD ESP, 0x10
0x8091824: MOV EDX, dword ptr [0x086619cc]
0x809182a: JMP 0x08091779
0x809182f: SUB ESP, 0xc
0x8091832: PUSH 0x82ad98c
0x8091837: CALL 0x08097b24
0x809183c: ADD ESP, 0x10
0x809183f: MOV EDX, dword ptr [0x086619cc]
0x8091845: MOV ECX, dword ptr [0x08661964]
0x809184b: JMP 0x080917ec
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_stackInThisContext / analyze_function

Evidence: `ev_d908ab85c3b336c3108d0113d535ec9b5582b136bb6b35d6ca5975a896519b6d`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80933ff", "start": "0x80933e4"}], "span_bytes": 28, "total_bytes": 28}`

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

## vm_checkIsRefType / procedure_pseudo_code

Evidence: `ev_4f2bd1197ee3cfcc577df7fb16ae4e1619badb4b73e7dda33475e7566f15ed0e`

```text

void vm_checkIsRefType(int no)

{
  int iVar1;
  anon_union_4_11_5000f750_for_v aVar2;
  
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
    iVar1 = vm_argumentInThisContext();
  }
  else {
    iVar1 = vm_argumentInThisContext();
  }
  if (no < iVar1) {
    iVar1 = vm_stackInThisContext();
    if (vm_sp <= no + iVar1) {
      aVar2.number._1_3_ = 0;
      aVar2.number._0_1_ = vm_stack[no + iVar1].type == '\b';
      if (vm_sp == 0) {
        vm_error("Stack overflow when calculating.\n");
      }
      vm_sp = vm_sp + -1;
      vm_stack[vm_sp].type = '\x02';
      vm_stack[vm_sp].v = aVar2;
      return;
    }
  }
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x02';
  vm_stack[vm_sp].v.info = (void *)0x0;
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

## vm_argumentInThisContext / procedure_assembly

Evidence: `ev_414a18bbd953f729969695ef06915f4871121b81f6814d8112df892ab3fe9a14`

```text
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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_argumentInThisContext / procedure_pseudo_code

Evidence: `ev_94ab3eba34c6a8ee8a84a3290d81f8b440d1c04a08d02850419510fc1d39f1d9`

```text

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
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
