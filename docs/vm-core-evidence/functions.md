# VM core Ghidra evidence
Original ELF SHA-256: `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`.
REA MCP / Ghidra 12.1.4; static observations only. Full canonical bundle: [rea.json.gz](rea.json.gz).

## vm_callInUserFunBody / procedure_pseudo_code
Evidence: `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_checkArgNum / procedure_assembly
Evidence: `ev_d17aa0d7a395baf8b760f4dd773008e55d6a7fc1e8c6931a9657f73663982946`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_checkArgNum / procedure_pseudo_code
Evidence: `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_copyValue / procedure_pseudo_code
Evidence: `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_divValue / procedure_assembly
Evidence: `ev_19597a6d356a5c98c32c16698ef7be5e4ad2201b7a0a22f7a714cc7918dd6f89`

```asm
0x808e1a4: PUSH EBP
0x808e1a5: MOV EBP, ESP
0x808e1a7: PUSH ESI
0x808e1a8: PUSH EBX
0x808e1a9: MOV EAX, [0x086619cc]
0x808e1ae: LEA EAX, [EAX + EAX*0x2]
0x808e1b1: MOV EDX, dword ptr [0x08661964]
0x808e1b7: LEA EBX, [EDX + EAX*0x8]
0x808e1ba: LEA ESI, [EBX + 0x18]
0x808e1bd: MOV AL, byte ptr [ESI + 0x4]
0x808e1c0: MOVZX EDX, AL
0x808e1c3: CMP EDX, 0x2
0x808e1c6: JZ 0x0808e1f4
0x808e1c8: CMP EDX, 0x3
0x808e1cb: JZ 0x0808e214
0x808e1cd: SUB ESP, 0xc
0x808e1d0: MOVZX EAX, AL
0x808e1d3: PUSH EAX
0x808e1d4: CALL 0x0808a13c
0x808e1d9: POP EBX
0x808e1da: POP ESI
0x808e1db: PUSH EAX
0x808e1dc: PUSH 0x82ad224
0x808e1e1: CALL 0x08097b24
0x808e1e6: ADD ESP, 0x10
0x808e1e9: LEA ESP, [EBP + -0x8]
0x808e1ec: POP EBX
0x808e1ed: POP ESI
0x808e1ee: LEAVE
0x808e1ef: JMP 0x0809061c
0x808e1f4: MOVZX EAX, byte ptr [EBX + 0x4]
0x808e1f8: CMP EAX, 0x2
0x808e1fb: JZ 0x0808e27a
0x808e1fd: CMP EAX, 0x3
0x808e200: JZ 0x0808e244
0x808e202: SUB ESP, 0xc
0x808e205: PUSH 0x82ad254
0x808e20a: CALL 0x08097b24
0x808e20f: JMP 0x0808e1e6
0x808e214: MOVZX EAX, byte ptr [EBX + 0x4]
0x808e218: CMP EAX, 0x2
0x808e21b: JZ 0x0808e264
0x808e21d: CMP EAX, 0x3
0x808e220: JNZ 0x0808e202
0x808e222: FLD float ptr [EBX + 0x8]
0x808e225: FLDZ
0x808e227: FXCH
0x808e229: FUCOMI ST0, ST1
0x808e22b: FSTP ST1
0x808e22d: JZ 0x0808e2bf
0x808e233: FDIVR float ptr [ESI + 0x8]
0x808e236: FSTP float ptr [ESI + 0x8]
0x808e239: LEA ESP, [EBP + -0x8]
0x808e23c: POP EBX
0x808e23d: POP ESI
0x808e23e: LEAVE
0x808e23f: JMP 0x0809061c
0x808e244: FLD float ptr [EBX + 0x8]
0x808e247: FLDZ
0x808e249: FXCH
0x808e24b: FUCOMI ST0, ST1
0x808e24d: FSTP ST1
0x808e24f: JZ 0x0808e291
0x808e251: SUB ESP, 0x10
0x808e254: FIDIVR dword ptr [ESI + 0x8]
0x808e257: FSTP float ptr [ESP]
0x808e25a: CALL 0x080a36e4
0x808e25f: MOV dword ptr [ESI + 0x8], EAX
0x808e262: JMP 0x0808e1e6
0x808e264: MOV EDX, dword ptr [EBX + 0x8]
0x808e267: TEST EDX, EDX
0x808e269: JZ 0x0808e2aa
0x808e26b: FLD float ptr [ESI + 0x8]
0x808e26e: PUSH EDX
0x808e26f: FIDIV dword ptr [ESP]
0x808e272: ADD ESP, 0x4
0x808e275: FSTP float ptr [ESI + 0x8]
0x808e278: JMP 0x0808e239
0x808e27a: MOV EDX, dword ptr [EBX + 0x8]
0x808e27d: TEST EDX, EDX
0x808e27f: JZ 0x0808e2e2
0x808e281: MOV EAX, dword ptr [ESI + 0x8]
0x808e284: MOV ECX, EDX
0x808e286: CDQ
0x808e287: IDIV ECX
0x808e289: MOV dword ptr [ESI + 0x8], EAX
0x808e28c: JMP 0x0808e1e9
0x808e291: JP 0x0808e251
0x808e293: FSTP ST0
0x808e295: SUB ESP, 0xc
0x808e298: PUSH 0x82acf88
0x808e29d: CALL 0x08097b24
0x808e2a2: ADD ESP, 0x10
0x808e2a5: FLD float ptr [EBX + 0x8]
0x808e2a8: JMP 0x0808e251
0x808e2aa: SUB ESP, 0xc
0x808e2ad: PUSH 0x82acf88
0x808e2b2: CALL 0x08097b24
0x808e2b7: ADD ESP, 0x10
0x808e2ba: MOV EDX, dword ptr [EBX + 0x8]
0x808e2bd: JMP 0x0808e26b
0x808e2bf: JP 0x0808e233
0x808e2c5: FSTP ST0
0x808e2c7: SUB ESP, 0xc
0x808e2ca: PUSH 0x82acf88
0x808e2cf: CALL 0x08097b24
0x808e2d4: FLD float ptr [EBX + 0x8]
0x808e2d7: ADD ESP, 0x10
0x808e2da: FDIVR float ptr [ESI + 0x8]
0x808e2dd: JMP 0x0808e236
0x808e2e2: SUB ESP, 0xc
0x808e2e5: PUSH 0x82acf88
0x808e2ea: CALL 0x08097b24
0x808e2ef: ADD ESP, 0x10
0x808e2f2: MOV EDX, dword ptr [EBX + 0x8]
0x808e2f5: JMP 0x0808e281
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808e1ef to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e23f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_divValue / procedure_pseudo_code
Evidence: `ev_8ae08d055f9314e0e074d8efe90e725516a1ec2a40a18b7fb1e9bc1014dc355c`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_divValue(void)

{
  float fVar1;
  byte bVar2;
  Vm_Value_T *pVVar3;
  int iVar4;
  char *pcVar5;
  int iVar6;
  
  iVar4 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar3 = vm_stack;
  bVar2 = vm_stack[vm_sp + 1].type;
  if (bVar2 == 2) {
    if (vm_stack[vm_sp].type == '\x02') {
      iVar6 = vm_stack[vm_sp].v.number;
      if (iVar6 == 0) {
        vm_error("divide by zero.\n");
        iVar6 = pVVar3[iVar4].v.number;
      }
      pVVar3[iVar4 + 1].v.number = pVVar3[iVar4 + 1].v.number / iVar6;
      goto LAB_0808e1e9;
    }
    if (vm_stack[vm_sp].type == '\x03') {
      fVar1 = vm_stack[vm_sp].v.real;
      if ((fVar1 == 0.0) && (!NAN(fVar1))) {
        vm_error("divide by zero.\n");
        fVar1 = pVVar3[iVar4].v.real;
      }
      iVar6 = _vm_ftol((float)pVVar3[iVar4 + 1].v.number / fVar1);
      pVVar3[iVar4 + 1].v.number = iVar6;
      goto LAB_0808e1e9;
    }
  }
  else {
    if (bVar2 != 3) {
      pcVar5 = vm_valueType((uint)bVar2);
      vm_error("Bad /, type invalid (got %s) of left value.\n",pcVar5);
      goto LAB_0808e1e9;
    }
    if (vm_stack[vm_sp].type == '\x02') {
      iVar6 = vm_stack[vm_sp].v.number;
      if (iVar6 == 0) {
        vm_error("divide by zero.\n");
        iVar6 = pVVar3[iVar4].v.number;
      }
      pVVar3[iVar4 + 1].v.real = pVVar3[iVar4 + 1].v.real / (float)iVar6;
LAB_0808e239:
      vm_popStack();
      return;
    }
    if (vm_stack[vm_sp].type == '\x03') {
      fVar1 = vm_stack[vm_sp].v.real;
      if ((fVar1 != 0.0) || (NAN(fVar1))) {
        fVar1 = vm_stack[vm_sp + 1].v.real / fVar1;
      }
      else {
        vm_error("divide by zero.\n");
        fVar1 = pVVar3[iVar4 + 1].v.real / pVVar3[iVar4].v.real;
      }
      pVVar3[iVar4 + 1].v.real = fVar1;
      goto LAB_0808e239;
    }
  }
  vm_error("Bad /, right isn\'t a number when left is.\n");
LAB_0808e1e9:
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808e1ef to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e23f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_enterFunction / procedure_assembly
Evidence: `ev_7cfb03642ac1aba0a64ceaeab6c36dc4b3cce77e35bf7fddcddba60778df0818`

```asm
0x80955e8: PUSH EBP
0x80955e9: MOV EAX, [0x086619c0]
0x80955ee: MOV EBP, ESP
0x80955f0: MOV EDX, dword ptr [0x086619c8]
0x80955f6: PUSH ESI
0x80955f7: LEA EAX, [EAX + EAX*0x4]
0x80955fa: PUSH EBX
0x80955fb: LEA EAX, [EDX + EAX*0x8]
0x80955fe: MOV EDX, dword ptr [EAX + 0x4]
0x8095601: MOV ESI, dword ptr [EAX + 0x24]
0x8095604: MOV EAX, [0x086619d8]
0x8095609: MOV dword ptr [EAX + 0x4], EDX
0x809560c: MOV DX, word ptr [ESI + 0xa]
0x8095610: MOVSX EAX, DX
0x8095613: CMP EAX, dword ptr [0x086619cc]
0x8095619: JG 0x08095639
0x809561b: XOR EBX, EBX
0x809561d: TEST DX, DX
0x8095620: JLE 0x08095632
0x8095622: MOV ESI, ESI
0x8095624: CALL 0x08090f6c
0x8095629: INC EBX
0x809562a: MOVSX EAX, word ptr [ESI + 0xa]
0x809562e: CMP EAX, EBX
0x8095630: JG 0x08095624
0x8095632: LEA ESP, [EBP + -0x8]
0x8095635: POP EBX
0x8095636: POP ESI
0x8095637: LEAVE
0x8095638: RET
0x8095639: SUB ESP, 0x8
0x809563c: PUSH dword ptr [ESI]
0x809563e: PUSH 0x82ae8b0
0x8095643: CALL 0x08097b24
0x8095648: ADD ESP, 0x10
0x809564b: MOV DX, word ptr [ESI + 0xa]
0x809564f: JMP 0x0809561b
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_enterFunction / procedure_pseudo_code
Evidence: `ev_1782e4cf62d7942391d37161d43a2051ad3aa963b13b7ca61519cc59ae7dad43`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_eqValue / procedure_pseudo_code
Evidence: `ev_d8c29d90acdccedee7e9b0cd111d8867b1406d4758a6708a74eb9eace188f84a`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_eqValue(void)

{
  Vm_Value_T *pValue;
  int iVar1;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg6(ESI)] */
  iVar1 = vm_isEq(vm_stack + vm_sp + 1,vm_stack + vm_sp);
  vm_popStack();
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int number@[???] */
  pValue = vm_stack + vm_sp;
  vm_cleanupValue(pValue);
  pValue->type = '\x02';
  (pValue->v).number = iVar1;
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

## vm_findOrAddProgramString / procedure_pseudo_code
Evidence: `ev_4954e391cf644acf17ef3044ffce8b179f8579860125c4fdaded74e71e2412b7`

```c

/* WARNING: Unknown calling convention */

Vm_Index_T vm_findOrAddProgramString(Vm_Program_T *pProgram,char *str)

{
  char cVar1;
  short sVar2;
  int iVar3;
  Vm_String **ppVVar4;
  Vm_String_T *pVVar5;
  uint uVar6;
  short sVar7;
  short sVar8;
  Vm_String **ppVVar9;
  char *pcVar10;
  
                    /* Unresolved local var: Vm_String_T * * ppNewStrings@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_String_T * pString@[DW_OP_reg2(EDX)]
                       Unresolved local var: uint newSize@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Index_T i@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Index_T spaceIndex@[DW_OP_reg3(EBX)] */
  ppVVar4 = pProgram->ppStrings;
  if (ppVVar4 != (Vm_String **)0x0) {
    sVar2 = pProgram->maxStringNo;
    sVar7 = 0;
    if (0 < sVar2) {
      sVar8 = 0;
      ppVVar9 = ppVVar4;
      do {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
        iVar3 = strcmp(str,(*ppVVar9)->ptr);
        if (iVar3 == 0) {
          return sVar8;
        }
        sVar7 = sVar7 + 1;
        sVar8 = sVar8 + 1;
        ppVVar9 = ppVVar9 + 1;
      } while (sVar7 < sVar2);
    }
  }
  sVar2 = pProgram->maxStringNo;
  if (pProgram->stringBufSize <= sVar2) {
    uVar6 = pProgram->stringBufSize * 4 + 0x40;
    if (ppVVar4 == (Vm_String **)0x0) {
      ppVVar4 = pdb_allocateMemory(uVar6,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x285);
    }
    else {
      ppVVar4 = pdb_reallocateMemory
                          (ppVVar4,uVar6,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x289);
    }
    if (ppVVar4 == (Vm_String **)0x0) {
      return -1;
    }
    pProgram->ppStrings = ppVVar4;
    pProgram->stringBufSize = pProgram->stringBufSize + 0x10;
  }
  uVar6 = 0xffffffff;
  pcVar10 = str;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar1 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar1 != '\0');
  pVVar5 = vm_safeAddSharedString(str,~uVar6 - 1);
  sVar7 = -1;
  if (pVVar5 != (Vm_String_T *)0x0) {
    pProgram->ppStrings[sVar2] = pVVar5;
    sVar7 = sVar2;
    if (pProgram->maxStringNo <= sVar2) {
      pProgram->maxStringNo = pProgram->maxStringNo + 1;
    }
  }
  return sVar7;
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

## vm_finishAction / procedure_pseudo_code
Evidence: `ev_5785a2de03047668275b248e9579da7e78ec5779d7dc2cd2c5066806ec787e92`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_finishAction(void)

{
  vm_ctrlFlags = vm_ctrlFlags | 0x40;
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

## vm_geValue / procedure_pseudo_code
Evidence: `ev_4442d99f8563c660f950f2e4e3d6fb934504618f8904fea7b1fb0f1d470b4151`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_geValue(void)

{
  Vm_Value_T *pValue2;
  byte bVar1;
  byte bVar2;
  int iVar3;
  char *pcVar4;
  uint uVar5;
  anon_union_4_11_5000f750_for_v aVar6;
  bool bVar7;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg3(EBX)] */
  pValue2 = vm_stack + vm_sp;
  bVar1 = pValue2[1].type;
  bVar2 = pValue2->type;
  if (bVar1 == bVar2) {
    if (bVar1 == 2) {
LAB_0808f098:
      bVar7 = (pValue2->v).number <= pValue2[1].v.number;
      goto LAB_0808f084;
    }
    if (bVar1 != 4) goto LAB_0808efae;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    uVar5 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
LAB_0808f037:
    vm_cleanupValue(pValue2 + 1);
    pValue2[1].type = '\x02';
    pValue2[1].v.number = ~uVar5 >> 0x1f;
LAB_0808efde:
    vm_popStack();
    return;
  }
LAB_0808efae:
  if (bVar1 == 3) {
    if (bVar2 == 2) {
      iVar3 = (pValue2->v).number;
      pValue2[1].type = '\x02';
      bVar7 = pValue2[1].v.real < (float)iVar3;
    }
    else {
      if (bVar2 != 3) {
LAB_0808f055:
        vm_error("Bad >=, right isn\'t a number when left is.\n");
        goto LAB_0808efde;
      }
      bVar7 = pValue2[1].v.real < (pValue2->v).real;
      pValue2[1].type = '\x02';
    }
  }
  else {
    if (3 < bVar1) {
      if (bVar1 == 4) {
        if (bVar2 != 4) {
          vm_error("Bad >=, right isn\'t a string when left is.\n");
          goto LAB_0808efde;
        }
        uVar5 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
      }
      else {
        if (bVar1 != 5) goto LAB_0808efc5;
        uVar5 = vm_compareBufWithBufStr(pValue2 + 1,pValue2,">=");
      }
      goto LAB_0808f037;
    }
    if (bVar1 != 2) {
LAB_0808efc5:
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad >=, type invalid (got %s) of left value.\n",pcVar4);
      goto LAB_0808efde;
    }
    if (bVar2 == 2) goto LAB_0808f098;
    if (bVar2 != 3) goto LAB_0808f055;
    bVar7 = (float)pValue2[1].v.number < (pValue2->v).real;
  }
  bVar7 = !bVar7;
LAB_0808f084:
  aVar6.number._1_3_ = 0;
  aVar6.number._0_1_ = bVar7;
  pValue2[1].v = aVar6;
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808efe7 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f090 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_goto / procedure_pseudo_code
Evidence: `ev_b69abd763149353ae40b150969c8ff78e728a3396a1eadcc652dc563d6001ea9`

```c

/* WARNING: Unknown calling convention */

void vm_goto(int offset)

{
  vm_currentContext->ip = vm_currentContext->ip + offset;
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

## vm_gtValue / procedure_pseudo_code
Evidence: `ev_5046856efb8928064add53f8c3b8b91c64ee3e170372d0683e48cf9018ea0e36`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_gtValue(void)

{
  Vm_Value_T *pValue;
  Vm_Value_T *pValue2;
  byte bVar1;
  byte bVar2;
  char *pcVar3;
  int iVar4;
  anon_union_4_11_5000f750_for_v aVar5;
  anon_union_4_11_5000f750_for_v aVar6;
  bool bVar7;
  bool bVar8;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg3(EBX)] */
  pValue2 = vm_stack + vm_sp;
  pValue = pValue2 + 1;
  bVar1 = pValue2[1].type;
  bVar2 = pValue2->type;
  if (bVar1 == bVar2) {
    if (bVar1 == 2) {
LAB_0808f460:
      bVar8 = (pValue2->v).number < pValue2[1].v.number;
      goto LAB_0808f44c;
    }
    if (bVar1 != 4) goto LAB_0808f366;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    iVar4 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
LAB_0808f3f8:
    aVar6.number._1_3_ = 0;
    aVar6.number._0_1_ = 0 < iVar4;
    vm_cleanupValue(pValue);
    pValue2[1].type = '\x02';
    pValue2[1].v = aVar6;
    goto LAB_0808f396;
  }
LAB_0808f366:
  if (bVar1 != 3) {
    if (bVar1 < 4) {
      if (bVar1 == 2) {
        if (bVar2 == 2) goto LAB_0808f460;
        if (bVar2 != 3) goto LAB_0808f41d;
        bVar8 = (float)pValue2[1].v.number == (pValue2->v).real;
        bVar7 = (float)pValue2[1].v.number < (pValue2->v).real;
        goto LAB_0808f449;
      }
    }
    else {
      if (bVar1 == 4) {
        if (bVar2 != 4) {
          vm_error("Bad >, right isn\'t a string when left is.\n");
          goto LAB_0808f396;
        }
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
        iVar4 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
        goto LAB_0808f3f8;
      }
      if (bVar1 == 5) {
        iVar4 = vm_compareBufWithBufStr(pValue,pValue2,">");
        vm_cleanupValue(pValue);
        bVar8 = 0 < iVar4;
        pValue2[1].type = '\x02';
        goto LAB_0808f44c;
      }
    }
    pcVar3 = vm_valueType((uint)bVar1);
    vm_error("Bad >, type invalid (got %s) of left value.\n",pcVar3);
LAB_0808f396:
    vm_popStack();
    return;
  }
  if (bVar2 == 2) {
    iVar4 = (pValue2->v).number;
    pValue2[1].type = '\x02';
    bVar8 = pValue2[1].v.real == (float)iVar4;
    bVar7 = pValue2[1].v.real < (float)iVar4;
  }
  else {
    if (bVar2 != 3) {
LAB_0808f41d:
      vm_error("Bad >, right isn\'t a number when left is.\n");
      goto LAB_0808f396;
    }
    bVar8 = pValue2[1].v.real == (pValue2->v).real;
    bVar7 = pValue2[1].v.real < (pValue2->v).real;
    pValue2[1].type = '\x02';
  }
LAB_0808f449:
  bVar8 = !bVar7 && !bVar8;
LAB_0808f44c:
  aVar5.number._1_3_ = 0;
  aVar5.number._0_1_ = bVar8;
  pValue2[1].v = aVar5;
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808f39f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f458 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_if / procedure_assembly
Evidence: `ev_de36227276c0f6a1e0ff4f5ed3490b6f131a438235392dfd545dcaf35a61611f`

```asm
0x809746c: PUSH EBP
0x809746d: MOV EBP, ESP
0x809746f: PUSH ESI
0x8097470: PUSH EBX
0x8097471: SUB ESP, 0x10
0x8097474: MOV ESI, dword ptr [EBP + 0x8]
0x8097477: MOV EBX, dword ptr [EBP + 0xc]
0x809747a: CALL 0x08090604
0x809747f: MOV dword ptr [ESP], EAX
0x8097482: CALL 0x0808cad4
0x8097487: ADD ESP, 0x10
0x809748a: TEST EAX, EAX
0x809748c: JZ 0x080974a0
0x809748e: TEST BL, 0x1
0x8097491: JZ 0x080974a5
0x8097493: AND EBX, 0x2
0x8097496: JNZ 0x080974b2
0x8097498: LEA ESP, [EBP + -0x8]
0x809749b: POP EBX
0x809749c: POP ESI
0x809749d: LEAVE
0x809749e: RET
0x80974a0: TEST BL, 0x1
0x80974a3: JZ 0x08097493
0x80974a5: MOV EAX, [0x086619d8]
0x80974aa: ADD dword ptr [EAX + 0xc], ESI
0x80974ad: AND EBX, 0x2
0x80974b0: JZ 0x08097498
0x80974b2: LEA ESP, [EBP + -0x8]
0x80974b5: POP EBX
0x80974b6: POP ESI
0x80974b7: LEAVE
0x80974b8: JMP 0x0809061c
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x80974b8 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_if / procedure_pseudo_code
Evidence: `ev_bf42239829f60589ab5f10d6db7127f1ca6c92abb37e87158ef49e79cd0c4550`

```c

/* WARNING: Enum "Vm_If_Operand_T": Some values do not have unique names */

void vm_if(int elseOffset,Vm_If_Operand_T operand)

{
  Vm_Value *pValue;
  int iVar1;
  
  pValue = vm_stackTopValue();
  iVar1 = vm_isZero(pValue);
  if (iVar1 == 0) {
    if ((operand & VM_IF_TRUE) == VM_DONT_POP_COND) goto joined_r0x080974b0;
  }
  else if ((operand & VM_IF_TRUE) != VM_DONT_POP_COND) goto joined_r0x080974b0;
  vm_currentContext->ip = vm_currentContext->ip + elseOffset;
joined_r0x080974b0:
  if ((operand & VM_POP_COND) != VM_DONT_POP_COND) {
    vm_popStack();
    return;
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
- Ghidra Listing reports a terminal call at 0x80974b8 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_isEq / procedure_pseudo_code
Evidence: `ev_5c49107925fa01436b0a0bc7b43cb2be49b36a27b867f03714ba4d4eea7fc979`

```c

int vm_isEq(Vm_Value_T *pValue1,Vm_Value_T *pValue2)

{
  float fVar1;
  byte bVar2;
  byte bVar3;
  Vm_String_T *pVVar4;
  Vm_String_T *pVVar5;
  uint uVar6;
  uchar *puVar7;
  byte *pbVar8;
  char *pcVar9;
  uchar *puVar10;
  byte *pbVar11;
  char *pcVar12;
  bool bVar13;
  bool bVar14;
  float fVar15;
  
                    /* Unresolved local var: Vm_String_T * s1@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_String_T * s2@[DW_OP_reg1(ECX)] */
  bVar2 = pValue1->type;
  bVar3 = pValue2->type;
  if (bVar2 == bVar3) {
    if (bVar2 == 2) goto LAB_0808ca80;
    if (bVar2 == 4) {
      pVVar4 = (pValue2->v).string;
      pVVar5 = (pValue1->v).string;
      if (pVVar5 == pVVar4) {
        return 1;
      }
      uVar6 = pVVar5->len;
      if (uVar6 != pVVar4->len) {
        return 0;
      }
      if ((pVVar4->attrib & pVVar5->attrib & 1) != 0) {
        return 0;
      }
      bVar14 = true;
      pcVar9 = pVVar5->ptr;
      pcVar12 = pVVar4->ptr;
      do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        bVar14 = *pcVar9 == *pcVar12;
        pcVar9 = pcVar9 + 1;
        pcVar12 = pcVar12 + 1;
      } while (bVar14);
      return (uint)bVar14;
    }
  }
  switch(bVar2) {
  case 0:
  case 1:
    return (uint)(bVar3 < 2);
  case 2:
    if (bVar3 == 2) {
LAB_0808ca80:
      return (uint)((pValue1->v).info == (pValue2->v).info);
    }
    if ((2 < bVar3) && (bVar3 == 3)) {
      bVar13 = NAN((float)(pValue1->v).number) || NAN((pValue2->v).real);
      bVar14 = (float)(pValue1->v).number == (pValue2->v).real;
      goto LAB_0808ca5c;
    }
    break;
  case 3:
    pValue1->type = '\x02';
    bVar2 = pValue2->type;
    if (bVar2 == 2) {
      fVar15 = (float)(pValue2->v).number;
      fVar1 = (pValue1->v).real;
    }
    else if (bVar2 < 3) {
      fVar1 = (pValue1->v).real;
      fVar15 = 0.0;
    }
    else {
      if (bVar2 != 3) {
        return 0;
      }
      fVar1 = (pValue1->v).real;
      fVar15 = (pValue2->v).real;
    }
    bVar13 = NAN(fVar1) || NAN(fVar15);
    bVar14 = fVar1 == fVar15;
LAB_0808ca5c:
    return (uint)(byte)(bVar14 & !bVar13);
  case 4:
    if (bVar3 != 4) {
      return 0;
    }
    pVVar4 = (pValue1->v).string;
    pVVar5 = (pValue2->v).string;
    if (pVVar4 == pVVar5) {
      return 1;
    }
    uVar6 = pVVar4->len;
    if (uVar6 != pVVar5->len) {
      return 0;
    }
    if ((pVVar5->attrib & pVVar4->attrib & 1) != 0) {
      return 0;
    }
    bVar14 = false;
    bVar13 = true;
    pbVar8 = (byte *)pVVar4->ptr;
    pbVar11 = (byte *)pVVar5->ptr;
    do {
      if (uVar6 == 0) break;
      uVar6 = uVar6 - 1;
      bVar14 = *pbVar8 < *pbVar11;
      bVar13 = *pbVar8 == *pbVar11;
      pbVar8 = pbVar8 + 1;
      pbVar11 = pbVar11 + 1;
    } while (bVar13);
    return (uint)((!bVar14 && !bVar13) == bVar14);
  case 5:
    if (bVar3 == 5) {
      uVar6 = ((pValue1->v).buffer)->realSize;
      if (uVar6 == ((pValue2->v).buffer)->realSize) {
        if (uVar6 != 0) {
          bVar14 = true;
          puVar7 = ((pValue1->v).buffer)->ptr;
          puVar10 = ((pValue2->v).buffer)->ptr;
          do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            bVar14 = *puVar7 == *puVar10;
            puVar7 = puVar7 + 1;
            puVar10 = puVar10 + 1;
          } while (bVar14);
          if (!bVar14) {
            return 0;
          }
        }
        return 1;
      }
    }
    break;
  case 6:
  case 7:
  case 9:
  case 10:
    if ((bVar2 == bVar3) && ((pValue1->v).info == (pValue2->v).info)) {
      return 1;
    }
  }
  return 0;
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

## vm_isZero / procedure_pseudo_code
Evidence: `ev_174f52005ca1def1dcd28fda64fa608062b04eaeef2672493386d0ad773ab397`

```c

int vm_isZero(Vm_Value_T *pValue)

{
  bool bVar1;
  
  switch(pValue->type) {
  case '\0':
  case '\x01':
    return 1;
  case '\x02':
    bVar1 = (pValue->v).info == (void *)0x0;
    break;
  case '\x03':
    bVar1 = (pValue->v).real == 0.0;
    break;
  default:
    return 0;
  case '\n':
    if (((uint)((pValue->v).string)->ptr & 0x8000) != 0) {
      vm_cleanupValue(pValue);
      return 1;
    }
    return 0;
  }
  return (uint)bVar1;
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

## vm_leValue / procedure_pseudo_code
Evidence: `ev_0e786f83fad6944f8f5973948ba408ed0a0a5200486741ce8f534333189dc7b2`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_leValue(void)

{
  Vm_Value_T *pValue;
  Vm_Value_T *pValue2;
  byte bVar1;
  byte bVar2;
  char *pcVar3;
  int iVar4;
  anon_union_4_11_5000f750_for_v aVar5;
  anon_union_4_11_5000f750_for_v aVar6;
  bool bVar7;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg3(EBX)] */
  pValue2 = vm_stack + vm_sp;
  pValue = pValue2 + 1;
  bVar1 = pValue2[1].type;
  bVar2 = pValue2->type;
  if (bVar1 == bVar2) {
    if (bVar1 == 2) {
LAB_0808f2cc:
      bVar7 = pValue2[1].v.number <= (pValue2->v).number;
      goto LAB_0808f2bb;
    }
    if (bVar1 != 4) goto LAB_0808f1e6;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    iVar4 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
LAB_0808f274:
    aVar6.number._1_3_ = 0;
    aVar6.number._0_1_ = iVar4 < 1;
    vm_cleanupValue(pValue);
    pValue2[1].type = '\x02';
    pValue2[1].v = aVar6;
    goto LAB_0808f216;
  }
LAB_0808f1e6:
  if (bVar1 != 3) {
    if (bVar1 < 4) {
      if (bVar1 == 2) {
        if (bVar2 == 2) goto LAB_0808f2cc;
        if (bVar2 != 3) goto LAB_0808f299;
        bVar7 = (pValue2->v).real < (float)pValue2[1].v.number;
        goto LAB_0808f2b8;
      }
    }
    else {
      if (bVar1 == 4) {
        if (bVar2 != 4) {
          vm_error("Bad <=, right isn\'t a string when left is.\n");
          goto LAB_0808f216;
        }
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
        iVar4 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
        goto LAB_0808f274;
      }
      if (bVar1 == 5) {
        iVar4 = vm_compareBufWithBufStr(pValue,pValue2,"<=");
        vm_cleanupValue(pValue);
        bVar7 = iVar4 < 1;
        pValue2[1].type = '\x02';
        goto LAB_0808f2bb;
      }
    }
    pcVar3 = vm_valueType((uint)bVar1);
    vm_error("Bad <=, type invalid (got %s) of left value.\n",pcVar3);
LAB_0808f216:
    vm_popStack();
    return;
  }
  if (bVar2 == 2) {
    iVar4 = (pValue2->v).number;
    pValue2[1].type = '\x02';
    bVar7 = (float)iVar4 < pValue2[1].v.real;
  }
  else {
    if (bVar2 != 3) {
LAB_0808f299:
      vm_error("Bad <=, right isn\'t a number when left is.\n");
      goto LAB_0808f216;
    }
    bVar7 = (pValue2->v).real < pValue2[1].v.real;
    pValue2[1].type = '\x02';
  }
LAB_0808f2b8:
  bVar7 = !bVar7;
LAB_0808f2bb:
  aVar5.number._1_3_ = 0;
  aVar5.number._0_1_ = bVar7;
  pValue2[1].v = aVar5;
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808f21f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f2c7 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_loadTempString / procedure_pseudo_code
Evidence: `ev_0ae24cff7665b6db8c47eff74e11b93c12d61f3ff765f261afc703842591d404`

```c

int vm_loadTempString(Vm_Binary_Buffer_T *pInput,char **ppStr)

{
  int iVar1;
  char *__dest;
  uint uVar2;
  uint len;
  void *buf;
  char *str;
  int size;
  
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 4) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 4) {
      return 0;
    }
  }
  iVar1 = *(int *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 4);
  pInput->len = uVar2 - 4;
  if (&stack0x00000000 != (undefined1 *)0x14) {
    size = iVar1;
  }
  if (size < 1) {
    *ppStr = (char *)0x0;
  }
  else {
    __dest = vm_scratchAlloc(size);
    if ((uint)size < 0xfa1) {
      if ((pInput->len < size) && (vm_fetchBinaryBufferData(pInput), pInput->len < size)) {
LAB_080b8d45:
        vm_scratchFree(__dest);
        return 0;
      }
      memcpy(__dest,pInput->p,size);
      pInput->p = pInput->p + size;
      pInput->len = pInput->len - size;
    }
    else {
      memcpy(__dest,pInput->p,pInput->len);
      iVar1 = pInput->len;
      len = size - iVar1;
      pInput->p = pInput->data;
      pInput->len = 0;
      uVar2 = vm_read(__dest + iVar1,len,pInput->fd);
      if (uVar2 != len) goto LAB_080b8d45;
    }
    if (&stack0x00000000 == (undefined1 *)0x18) {
      vm_scratchFree(__dest);
      __dest = str;
    }
    str = __dest;
    str[size + -1] = '\0';
    if (ppStr == (char **)0x0) {
      vm_scratchFree(str);
    }
    else {
      *ppStr = str;
    }
  }
  return 1;
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

## vm_ltValue / procedure_pseudo_code
Evidence: `ev_deee5e8743aec13f48327236f9b0793ccd92a15e202b8459782b33ff3e514c4d`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_ltValue(void)

{
  Vm_Value_T *pValue2;
  byte bVar1;
  byte bVar2;
  int iVar3;
  char *pcVar4;
  uint uVar5;
  anon_union_4_11_5000f750_for_v aVar6;
  bool bVar7;
  bool bVar8;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg3(EBX)] */
  pValue2 = vm_stack + vm_sp;
  bVar1 = pValue2[1].type;
  bVar2 = pValue2->type;
  if (bVar1 == bVar2) {
    if (bVar1 == 2) {
LAB_0808f5c0:
      bVar8 = pValue2[1].v.number < (pValue2->v).number;
      goto LAB_0808f5ac;
    }
    if (bVar1 != 4) goto LAB_0808f4ee;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    uVar5 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
LAB_0808f568:
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    vm_cleanupValue(pValue2 + 1);
    pValue2[1].type = '\x02';
    pValue2[1].v.number = uVar5 >> 0x1f;
LAB_0808f51e:
    vm_popStack();
    return;
  }
LAB_0808f4ee:
  if (bVar1 == 3) {
    if (bVar2 == 2) {
      iVar3 = (pValue2->v).number;
      pValue2[1].type = '\x02';
      bVar8 = (float)iVar3 == pValue2[1].v.real;
      bVar7 = (float)iVar3 < pValue2[1].v.real;
    }
    else {
      if (bVar2 != 3) {
LAB_0808f58d:
        vm_error("Bad <, right isn\'t a number when left is.\n");
        goto LAB_0808f51e;
      }
      bVar8 = (pValue2->v).real == pValue2[1].v.real;
      bVar7 = (pValue2->v).real < pValue2[1].v.real;
      pValue2[1].type = '\x02';
    }
  }
  else {
    if (3 < bVar1) {
      if (bVar1 == 4) {
        if (bVar2 != 4) {
          vm_error("Bad <, right isn\'t a string when left is.\n");
          goto LAB_0808f51e;
        }
        uVar5 = strcmp((pValue2[1].v.string)->ptr,((pValue2->v).string)->ptr);
      }
      else {
        if (bVar1 != 5) goto LAB_0808f505;
        uVar5 = vm_compareBufWithBufStr(pValue2 + 1,pValue2,"<");
      }
      goto LAB_0808f568;
    }
    if (bVar1 != 2) {
LAB_0808f505:
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad <, type invalid (got %s) of left value.\n",pcVar4);
      goto LAB_0808f51e;
    }
    if (bVar2 == 2) goto LAB_0808f5c0;
    if (bVar2 != 3) goto LAB_0808f58d;
    bVar8 = (pValue2->v).real == (float)pValue2[1].v.number;
    bVar7 = (pValue2->v).real < (float)pValue2[1].v.number;
  }
  bVar8 = !bVar7 && !bVar8;
LAB_0808f5ac:
  aVar6.number._1_3_ = 0;
  aVar6.number._0_1_ = bVar8;
  pValue2[1].v = aVar6;
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808f527 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f5b8 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_modValue / procedure_assembly
Evidence: `ev_4066c4eef4bbbc302618e0bb8a65aeab459dde67f388ba53c0695f2616f8a31d`

```asm
0x808e7bc: PUSH EBP
0x808e7bd: MOV EBP, ESP
0x808e7bf: PUSH ESI
0x808e7c0: PUSH EBX
0x808e7c1: SUB ESP, 0x10
0x808e7c4: MOV EAX, [0x086619cc]
0x808e7c9: LEA EAX, [EAX + EAX*0x2]
0x808e7cc: MOV EDX, dword ptr [0x08661964]
0x808e7d2: LEA EBX, [EDX + EAX*0x8]
0x808e7d5: LEA ESI, [EBX + 0x18]
0x808e7d8: MOV AL, byte ptr [ESI + 0x4]
0x808e7db: MOVZX EDX, AL
0x808e7de: CMP EDX, 0x2
0x808e7e1: JZ 0x0808e810
0x808e7e3: CMP EDX, 0x3
0x808e7e6: JZ 0x0808e830
0x808e7e8: SUB ESP, 0xc
0x808e7eb: MOVZX EAX, AL
0x808e7ee: PUSH EAX
0x808e7ef: CALL 0x0808a13c
0x808e7f4: POP EDX
0x808e7f5: POP ECX
0x808e7f6: PUSH EAX
0x808e7f7: PUSH 0x82ad5bc
0x808e7fc: CALL 0x08097b24
0x808e801: ADD ESP, 0x10
0x808e804: CALL 0x0809061c
0x808e809: LEA ESP, [EBP + -0x8]
0x808e80c: POP EBX
0x808e80d: POP ESI
0x808e80e: LEAVE
0x808e80f: RET
0x808e810: MOVZX EAX, byte ptr [EBX + 0x4]
0x808e814: CMP EAX, 0x2
0x808e817: JZ 0x0808e843
0x808e819: CMP EAX, 0x3
0x808e81c: JZ 0x0808e857
0x808e81e: SUB ESP, 0xc
0x808e821: PUSH 0x82ad5ec
0x808e826: CALL 0x08097b24
0x808e82b: JMP 0x0808e801
0x808e830: MOVZX EAX, byte ptr [EBX + 0x4]
0x808e834: CMP EAX, 0x2
0x808e837: JZ 0x0808e87f
0x808e839: CMP EAX, 0x3
0x808e83c: JNZ 0x0808e81e
0x808e83e: FLD float ptr [EBX + 0x8]
0x808e841: JMP 0x0808e88c
0x808e843: MOV EDX, dword ptr [EBX + 0x8]
0x808e846: TEST EDX, EDX
0x808e848: JZ 0x0808e8a6
0x808e84a: MOV EAX, dword ptr [ESI + 0x8]
0x808e84d: MOV ECX, EDX
0x808e84f: CDQ
0x808e850: IDIV ECX
0x808e852: MOV dword ptr [ESI + 0x8], EDX
0x808e855: JMP 0x0808e804
0x808e857: FLD float ptr [EBX + 0x8]
0x808e85a: SUB ESP, 0x10
0x808e85d: FSTP double ptr [ESP + 0x8]
0x808e861: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808e866: MOVSS dword ptr [EBP + -0x10], XMM0
0x808e86b: FLD float ptr [EBP + -0x10]
0x808e86e: FSTP double ptr [ESP]
0x808e871: CALL 0x0804b0f4
0x808e876: FSTP float ptr [ESI + 0x8]
0x808e879: MOV byte ptr [ESI + 0x4], 0x3
0x808e87d: JMP 0x0808e801
0x808e87f: CVTSI2SS XMM0, dword ptr [EBX + 0x8]
0x808e884: MOVSS dword ptr [EBP + -0x10], XMM0
0x808e889: FLD float ptr [EBP + -0x10]
0x808e88c: SUB ESP, 0x10
0x808e88f: FSTP double ptr [ESP + 0x8]
0x808e893: FLD float ptr [ESI + 0x8]
0x808e896: FSTP double ptr [ESP]
0x808e899: CALL 0x0804b0f4
0x808e89e: FSTP float ptr [ESI + 0x8]
0x808e8a1: JMP 0x0808e801
0x808e8a6: SUB ESP, 0xc
0x808e8a9: PUSH 0x82acf99
0x808e8ae: CALL 0x08097b24
0x808e8b3: ADD ESP, 0x10
0x808e8b6: MOV EDX, dword ptr [EBX + 0x8]
0x808e8b9: JMP 0x0808e84a
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_modValue / procedure_pseudo_code
Evidence: `ev_2eb04ee1a9a6e84c698cbb6cc4f66fa7dfb1f53d14b2b3c0bc689e53a5db7d58`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_modValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  int iVar4;
  float fVar5;
  double dVar6;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 == 2) {
    if (vm_stack[vm_sp].type == '\x02') {
      iVar4 = vm_stack[vm_sp].v.number;
      if (iVar4 == 0) {
        vm_error("Module by zero.\n");
        iVar4 = pVVar2[iVar3].v.number;
      }
      pVVar2[iVar3 + 1].v.number = pVVar2[iVar3 + 1].v.number % iVar4;
      goto LAB_0808e804;
    }
    if (vm_stack[vm_sp].type == '\x03') {
      dVar6 = fmod((double)vm_stack[vm_sp + 1].v.number,(double)vm_stack[vm_sp].v.real);
      pVVar2[iVar3 + 1].v.real = (float)dVar6;
      pVVar2[iVar3 + 1].type = '\x03';
      goto LAB_0808e804;
    }
LAB_0808e81e:
    vm_error("Bad *, right isn\'t a real when left is.\n");
  }
  else {
    if (bVar1 != 3) {
      vm_valueType((uint)bVar1);
      vm_error("Bad %, type invalid (got %s) of left value.\n");
      goto LAB_0808e804;
    }
    if (vm_stack[vm_sp].type == '\x02') {
      fVar5 = (float)vm_stack[vm_sp].v.number;
    }
    else {
      if (vm_stack[vm_sp].type != '\x03') goto LAB_0808e81e;
      fVar5 = vm_stack[vm_sp].v.real;
    }
    dVar6 = fmod((double)vm_stack[vm_sp + 1].v.real,(double)fVar5);
    pVVar2[iVar3 + 1].v.real = (float)dVar6;
  }
LAB_0808e804:
  vm_popStack();
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

## vm_mulValue / procedure_assembly
Evidence: `ev_5feef0892ba0df07d0c16640ed85437c8c38d70cdbb82072e72d8d165d011367`

```asm
0x808e2f8: PUSH EBP
0x808e2f9: MOV EBP, ESP
0x808e2fb: PUSH EBX
0x808e2fc: PUSH EBX
0x808e2fd: MOV EAX, [0x086619cc]
0x808e302: LEA EAX, [EAX + EAX*0x2]
0x808e305: MOV EDX, dword ptr [0x08661964]
0x808e30b: LEA ECX, [EDX + EAX*0x8]
0x808e30e: LEA EBX, [ECX + 0x18]
0x808e311: MOV AL, byte ptr [EBX + 0x4]
0x808e314: MOVZX EDX, AL
0x808e317: CMP EDX, 0x2
0x808e31a: JZ 0x0808e348
0x808e31c: CMP EDX, 0x3
0x808e31f: JZ 0x0808e368
0x808e321: SUB ESP, 0xc
0x808e324: MOVZX EAX, AL
0x808e327: PUSH EAX
0x808e328: CALL 0x0808a13c
0x808e32d: POP EDX
0x808e32e: POP ECX
0x808e32f: PUSH EAX
0x808e330: PUSH 0x82ad280
0x808e335: CALL 0x08097b24
0x808e33a: ADD ESP, 0x10
0x808e33d: MOV EBX, dword ptr [EBP + -0x4]
0x808e340: LEAVE
0x808e341: JMP 0x0809061c
0x808e348: MOVZX EAX, byte ptr [ECX + 0x4]
0x808e34c: CMP EAX, 0x2
0x808e34f: JZ 0x0808e393
0x808e351: CMP EAX, 0x3
0x808e354: JZ 0x0808e39f
0x808e356: SUB ESP, 0xc
0x808e359: PUSH 0x82ad2b0
0x808e35e: CALL 0x08097b24
0x808e363: JMP 0x0808e33a
0x808e368: MOVZX EAX, byte ptr [ECX + 0x4]
0x808e36c: CMP EAX, 0x2
0x808e36f: JZ 0x0808e388
0x808e371: CMP EAX, 0x3
0x808e374: JNZ 0x0808e356
0x808e376: FLD float ptr [ECX + 0x8]
0x808e379: FMUL float ptr [EBX + 0x8]
0x808e37c: FSTP float ptr [EBX + 0x8]
0x808e37f: MOV EBX, dword ptr [EBP + -0x4]
0x808e382: LEAVE
0x808e383: JMP 0x0809061c
0x808e388: FLD float ptr [EBX + 0x8]
0x808e38b: FIMUL dword ptr [ECX + 0x8]
0x808e38e: FSTP float ptr [EBX + 0x8]
0x808e391: JMP 0x0808e37f
0x808e393: MOV EAX, dword ptr [ECX + 0x8]
0x808e396: IMUL EAX, dword ptr [EBX + 0x8]
0x808e39a: MOV dword ptr [EBX + 0x8], EAX
0x808e39d: JMP 0x0808e33d
0x808e39f: FLD float ptr [ECX + 0x8]
0x808e3a2: SUB ESP, 0x10
0x808e3a5: FIMUL dword ptr [EBX + 0x8]
0x808e3a8: FSTP float ptr [ESP]
0x808e3ab: CALL 0x080a36e4
0x808e3b0: MOV dword ptr [EBX + 0x8], EAX
0x808e3b3: JMP 0x0808e33a
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808e341 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e383 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_mulValue / procedure_pseudo_code
Evidence: `ev_eae8798244296f374fed7969671ce9b93fb83c7bb25b81f95c5b1497b8447349`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_mulValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  int iVar5;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 == 2) {
    if (vm_stack[vm_sp].type == '\x02') {
      vm_stack[vm_sp + 1].v.number = vm_stack[vm_sp].v.number * vm_stack[vm_sp + 1].v.number;
      goto LAB_0808e33d;
    }
    if (vm_stack[vm_sp].type == '\x03') {
      iVar5 = _vm_ftol(vm_stack[vm_sp].v.real * (float)vm_stack[vm_sp + 1].v.number);
      pVVar2[iVar3 + 1].v.number = iVar5;
      goto LAB_0808e33d;
    }
  }
  else {
    if (bVar1 != 3) {
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad *, type invalid (got %s) of left value.\n",pcVar4);
      goto LAB_0808e33d;
    }
    if (vm_stack[vm_sp].type == '\x02') {
      vm_stack[vm_sp + 1].v.real = vm_stack[vm_sp + 1].v.real * (float)vm_stack[vm_sp].v.number;
LAB_0808e37f:
      vm_popStack();
      return;
    }
    if (vm_stack[vm_sp].type == '\x03') {
      vm_stack[vm_sp + 1].v.real = vm_stack[vm_sp].v.real * vm_stack[vm_sp + 1].v.real;
      goto LAB_0808e37f;
    }
  }
  vm_error("Bad *, right isn\'t a number when left is.\n");
LAB_0808e33d:
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808e341 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e383 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_negValue / procedure_assembly
Evidence: `ev_37ed690e41c40d021d7342b9279e353ae0871f764be4f3c3ea43223d330a7d78`

```asm
0x808e8bc: PUSH EBP
0x808e8bd: MOV EBP, ESP
0x808e8bf: SUB ESP, 0x8
0x808e8c2: MOV EAX, [0x086619cc]
0x808e8c7: LEA EAX, [EAX + EAX*0x2]
0x808e8ca: MOV EDX, dword ptr [0x08661964]
0x808e8d0: LEA ECX, [EDX + EAX*0x8]
0x808e8d3: MOV AL, byte ptr [ECX + 0x4]
0x808e8d6: MOVZX EDX, AL
0x808e8d9: CMP EDX, 0x2
0x808e8dc: JZ 0x0808e904
0x808e8de: CMP EDX, 0x3
0x808e8e1: JZ 0x0808e90c
0x808e8e3: SUB ESP, 0xc
0x808e8e6: MOVZX EAX, AL
0x808e8e9: PUSH EAX
0x808e8ea: CALL 0x0808a13c
0x808e8ef: POP EDX
0x808e8f0: POP ECX
0x808e8f1: PUSH EAX
0x808e8f2: PUSH 0x82ad618
0x808e8f7: CALL 0x08097b24
0x808e8fc: ADD ESP, 0x10
0x808e8ff: LEAVE
0x808e900: RET
0x808e904: NEG dword ptr [ECX + 0x8]
0x808e907: LEAVE
0x808e908: RET
0x808e90c: XOR byte ptr [ECX + 0xb], 0x80
0x808e910: LEAVE
0x808e911: RET
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_negValue / procedure_pseudo_code
Evidence: `ev_fcdf023d9edf9971d84cf2ec610aead4d298bc2cb2dc5599ca81ba8f89e14478`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_negValue(void)

{
  byte *pbVar1;
  byte bVar2;
  char *pcVar3;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg1(ECX)] */
  bVar2 = vm_stack[vm_sp].type;
  if (bVar2 == 2) {
    vm_stack[vm_sp].v.number = -vm_stack[vm_sp].v.number;
    return;
  }
  if (bVar2 != 3) {
    pcVar3 = vm_valueType((uint)bVar2);
    vm_error("Bad -, type invalid of right value.\n",pcVar3);
    return;
  }
  pbVar1 = (byte *)((int)&vm_stack[vm_sp].v + 3);
  *pbVar1 = *pbVar1 ^ 0x80;
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

## vm_popNStack / procedure_pseudo_code
Evidence: `ev_e7527a7ca28726089a3908bfa29a6bbe1052e75faaf4c81247826fe8dc45597c`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_popStack / procedure_pseudo_code
Evidence: `ev_6f8162a52b6dcead2e90f45ac594636448ad034684367dee557ea51b2cd7fc1f`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_popStack(void)

{
  vm_cleanupValue(vm_stack + vm_sp);
  vm_sp = vm_sp + 1;
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

## vm_pushLocalVar / procedure_assembly
Evidence: `ev_d0c3b683bea4f35107c1c4be31af558ea2635c43a303b9009673e8de1ca083fc`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x809159d to 0x808c028 vm_convertToValue (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushLocalVar / procedure_pseudo_code
Evidence: `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- Ghidra Listing reports a terminal call at 0x809159d to 0x808c028 vm_convertToValue (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushNumber / procedure_pseudo_code
Evidence: `ev_13cc97c095cc20b0ce78acbb8560fd9b067806a9cff8156d4eadc2014279a737`

```c

void vm_pushNumber(int num)

{
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x02';
  vm_stack[vm_sp].v.number = num;
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

## vm_pushObjVar / procedure_pseudo_code
Evidence: `ev_03adb0f3a115b83f2497d37852e6c64ffd233cbe06b82b9e57624900a2a01c09`

```c

/* WARNING: Unknown calling convention */

void vm_pushObjVar(int valueNo)

{
  short sVar1;
  Vm_Value *pVVar2;
  
  sVar1 = vm_currentContext->consititution->baseVarOffset;
  pVVar2 = vm_currentContext->thisObject->pObjectValues;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_copyValue(vm_stack + vm_sp,pVVar2 + sVar1 + valueNo);
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

## vm_pushSharedString / procedure_assembly
Evidence: `ev_734b597b55be50ee446a59e8502687b0068270e4791dcc47564e0a019eaeee7c`

```asm
0x8090cf8: PUSH EBP
0x8090cf9: MOV EAX, [0x086619cc]
0x8090cfe: MOV EBP, ESP
0x8090d00: TEST EAX, EAX
0x8090d02: PUSH ESI
0x8090d03: PUSH EBX
0x8090d04: MOV ESI, dword ptr [EBP + 0x8]
0x8090d07: JZ 0x08090d70
0x8090d09: MOV EAX, [0x086619d8]
0x8090d0e: MOV EAX, dword ptr [EAX + 0x14]
0x8090d11: TEST ESI, ESI
0x8090d13: MOV EDX, dword ptr [EAX]
0x8090d15: JS 0x08090d1f
0x8090d17: MOVSX EAX, word ptr [EDX + 0x24]
0x8090d1b: CMP EAX, ESI
0x8090d1d: JG 0x08090d64
0x8090d1f: SUB ESP, 0x8
0x8090d22: PUSH ESI
0x8090d23: PUSH 0x82adfaa
0x8090d28: CALL 0x08097b24
0x8090d2d: ADD ESP, 0x10
0x8090d30: MOV EAX, [0x086619cc]
0x8090d35: DEC EAX
0x8090d36: MOV [0x086619cc], EAX
0x8090d3b: MOV EDX, dword ptr [0x08661964]
0x8090d41: LEA EAX, [EAX + EAX*0x2]
0x8090d44: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x4
0x8090d49: MOV EAX, [0x086619cc]
0x8090d4e: MOV EDX, dword ptr [0x08661964]
0x8090d54: LEA EAX, [EAX + EAX*0x2]
0x8090d57: MOV dword ptr [EDX + EAX*0x8 + 0x8], EBX
0x8090d5b: INC dword ptr [EBX]
0x8090d5d: LEA ESP, [EBP + -0x8]
0x8090d60: POP EBX
0x8090d61: POP ESI
0x8090d62: LEAVE
0x8090d63: RET
0x8090d64: MOV EAX, dword ptr [EDX + 0x20]
0x8090d67: MOV EBX, dword ptr [EAX + ESI*0x4]
0x8090d6a: TEST EBX, EBX
0x8090d6c: JNZ 0x08090d30
0x8090d6e: JMP 0x08090d1f
0x8090d70: SUB ESP, 0xc
0x8090d73: PUSH 0x82ad98c
0x8090d78: CALL 0x08097b24
0x8090d7d: ADD ESP, 0x10
0x8090d80: JMP 0x08090d09
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushSharedString / procedure_pseudo_code
Evidence: `ev_036c7ff9e3a4c3a9d1380520976755debc306a776b18e23161519177ae073205`

```c

void vm_pushSharedString(int n)

{
  Vm_Program *pVVar1;
  Vm_String_T *unaff_EBX;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_String_T * pString@[DW_OP_reg3(EBX)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  pVVar1 = vm_currentContext->consititution->pProgram;
  if (((n < 0) || (pVVar1->maxStringNo <= n)) ||
     (unaff_EBX = pVVar1->ppStrings[n], unaff_EBX == (Vm_String_T *)0x0)) {
    vm_error("Bad shared string %d!\n",n);
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x04';
  vm_stack[vm_sp].v.string = unaff_EBX;
  unaff_EBX->ref = unaff_EBX->ref + 1;
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

## vm_pushShortNumber / procedure_pseudo_code
Evidence: `ev_4a1f1eabff94d1d4b1a2edcdf34c51d50d7c3ee62ff70c51d3e1d68fcd66a0cc`

```c

void vm_pushShortNumber(int num)

{
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x02';
  vm_stack[vm_sp].v.number = num;
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

## vm_pushUndefined / procedure_pseudo_code
Evidence: `ev_2c1b31fb3efde1f223d478ac5dac3ee8b3fb310407a0de9ce49c577425556416`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_pushUndefined(void)

{
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_stack[vm_sp].v.info = (void *)0x0;
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

## vm_returnFromFun / procedure_assembly
Evidence: `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_returnFromFun / procedure_pseudo_code
Evidence: `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_setLocalVar / procedure_assembly
Evidence: `ev_b4762566f69e6ef759a4c846394fb10d47cd1eebf8da295f5f40105ef3884689`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_setLocalVar / procedure_pseudo_code
Evidence: `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc`

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

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_setObjVar / procedure_pseudo_code
Evidence: `ev_85a706f75e7ed097afaab10b0d9474c379d806b7272d872c2ea29584a5f56438`

```c

/* WARNING: Unknown calling convention */

void vm_setObjVar(int valueNo)

{
  Vm_Value_T *pValue;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pValue = vm_currentContext->thisObject->pObjectValues +
           vm_currentContext->consititution->baseVarOffset + valueNo;
  vm_cleanupValue(pValue);
  vm_copyValue(pValue,vm_stack + vm_sp);
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

## vm_subValue / procedure_assembly
Evidence: `ev_c05cfbed9ca9bde5d9502bdb4b76690fae96ad7d90969915c664c33ea7a38220`

```asm
0x808e654: PUSH EBP
0x808e655: MOV EBP, ESP
0x808e657: PUSH ESI
0x808e658: PUSH EBX
0x808e659: MOV EAX, [0x086619cc]
0x808e65e: LEA EAX, [EAX + EAX*0x2]
0x808e661: MOV EDX, dword ptr [0x08661964]
0x808e667: LEA ECX, [EDX + EAX*0x8]
0x808e66a: LEA ESI, [ECX + 0x18]
0x808e66d: MOV AL, byte ptr [ESI + 0x4]
0x808e670: MOVZX EDX, AL
0x808e673: CMP EDX, 0x3
0x808e676: JZ 0x0808e6dc
0x808e678: JG 0x0808e6ac
0x808e67a: CMP EDX, 0x2
0x808e67d: JZ 0x0808e728
0x808e683: SUB ESP, 0xc
0x808e686: MOVZX EAX, AL
0x808e689: PUSH EAX
0x808e68a: CALL 0x0808a13c
0x808e68f: POP ESI
0x808e690: POP EDX
0x808e691: PUSH EAX
0x808e692: PUSH 0x82ad4f8
0x808e697: CALL 0x08097b24
0x808e69c: ADD ESP, 0x10
0x808e69f: LEA ESP, [EBP + -0x8]
0x808e6a2: POP EBX
0x808e6a3: POP ESI
0x808e6a4: LEAVE
0x808e6a5: JMP 0x0809061c
0x808e6ac: CMP EDX, 0x6
0x808e6af: JZ 0x0808e710
0x808e6b1: CMP EDX, 0x7
0x808e6b4: JNZ 0x0808e683
0x808e6b6: MOVZX EAX, byte ptr [ECX + 0x4]
0x808e6ba: CMP EAX, 0x6
0x808e6bd: JZ 0x0808e7aa
0x808e6c3: CMP EAX, 0x7
0x808e6c6: JZ 0x0808e75a
0x808e6cc: SUB ESP, 0xc
0x808e6cf: PUSH 0x82ad528
0x808e6d4: CALL 0x08097b24
0x808e6d9: JMP 0x0808e69c
0x808e6dc: MOVZX EAX, byte ptr [ECX + 0x4]
0x808e6e0: CMP EAX, 0x2
0x808e6e3: JZ 0x0808e74f
0x808e6e5: CMP EAX, 0x3
0x808e6e8: JZ 0x0808e6f9
0x808e6ea: SUB ESP, 0xc
0x808e6ed: PUSH 0x82ad564
0x808e6f2: CALL 0x08097b24
0x808e6f7: JMP 0x0808e69c
0x808e6f9: FLD float ptr [ECX + 0x8]
0x808e6fc: FSUBR float ptr [ESI + 0x8]
0x808e6ff: FSTP float ptr [ESI + 0x8]
0x808e702: LEA ESP, [EBP + -0x8]
0x808e705: POP EBX
0x808e706: POP ESI
0x808e707: LEAVE
0x808e708: JMP 0x0809061c
0x808e710: CMP byte ptr [ECX + 0x4], 0x6
0x808e714: JZ 0x0808e789
0x808e716: SUB ESP, 0xc
0x808e719: PUSH 0x82ad590
0x808e71e: CALL 0x08097b24
0x808e723: JMP 0x0808e69c
0x808e728: MOVZX EAX, byte ptr [ECX + 0x4]
0x808e72c: CMP EAX, 0x2
0x808e72f: JZ 0x0808e77b
0x808e731: CMP EAX, 0x3
0x808e734: JNZ 0x0808e6ea
0x808e736: FLD float ptr [ECX + 0x8]
0x808e739: SUB ESP, 0x10
0x808e73c: FISUBR dword ptr [ESI + 0x8]
0x808e73f: FSTP float ptr [ESP]
0x808e742: CALL 0x080a36e4
0x808e747: MOV dword ptr [ESI + 0x8], EAX
0x808e74a: JMP 0x0808e69c
0x808e74f: FLD float ptr [ESI + 0x8]
0x808e752: FISUB dword ptr [ECX + 0x8]
0x808e755: FSTP float ptr [ESI + 0x8]
0x808e758: JMP 0x0808e702
0x808e75a: PUSH EAX
0x808e75b: PUSH 0x1
0x808e75d: PUSH dword ptr [ECX + 0x8]
0x808e760: PUSH dword ptr [ESI + 0x8]
0x808e763: CALL 0x0809df64
0x808e768: MOV EBX, EAX
0x808e76a: POP EAX
0x808e76b: PUSH dword ptr [ESI + 0x8]
0x808e76e: CALL 0x0809c488
0x808e773: MOV dword ptr [ESI + 0x8], EBX
0x808e776: JMP 0x0808e69c
0x808e77b: MOV EAX, dword ptr [ESI + 0x8]
0x808e77e: SUB EAX, dword ptr [ECX + 0x8]
0x808e781: MOV dword ptr [ESI + 0x8], EAX
0x808e784: JMP 0x0808e69f
0x808e789: SUB ESP, 0x8
0x808e78c: PUSH dword ptr [ECX + 0x8]
0x808e78f: PUSH dword ptr [ESI + 0x8]
0x808e792: CALL 0x0809bfb8
0x808e797: MOV EBX, EAX
0x808e799: POP EAX
0x808e79a: PUSH dword ptr [ESI + 0x8]
0x808e79d: CALL 0x0809b350
0x808e7a2: MOV dword ptr [ESI + 0x8], EBX
0x808e7a5: JMP 0x0808e69c
0x808e7aa: PUSH EBX
0x808e7ab: PUSH 0x1
0x808e7ad: PUSH dword ptr [ECX + 0x8]
0x808e7b0: PUSH dword ptr [ESI + 0x8]
0x808e7b3: CALL 0x0809e234
0x808e7b8: JMP 0x0808e768
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808e6a5 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e708 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_subValue / procedure_pseudo_code
Evidence: `ev_26f07bada739486910ffe50a01f0d7f9cda886d834b4dc02ca1f63165cfbdd9f`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_subValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  int iVar5;
  Vm_Mapping_T *pVVar6;
  Vm_String_T *pVVar7;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 == 3) {
    if (vm_stack[vm_sp].type == '\x02') {
      vm_stack[vm_sp + 1].v.real = vm_stack[vm_sp + 1].v.real - (float)vm_stack[vm_sp].v.number;
LAB_0808e702:
      vm_popStack();
      return;
    }
    if (vm_stack[vm_sp].type == '\x03') {
      vm_stack[vm_sp + 1].v.real = vm_stack[vm_sp + 1].v.real - vm_stack[vm_sp].v.real;
      goto LAB_0808e702;
    }
LAB_0808e6ea:
    vm_error("Bad -, right isn\'t a number when left is.\n");
  }
  else {
    if (bVar1 < 4) {
      if (bVar1 == 2) {
        if (vm_stack[vm_sp].type == '\x02') {
          vm_stack[vm_sp + 1].v.info =
               (void *)((int)vm_stack[vm_sp + 1].v.info - vm_stack[vm_sp].v.number);
          goto LAB_0808e69f;
        }
        if (vm_stack[vm_sp].type == '\x03') {
          iVar5 = _vm_ftol((float)vm_stack[vm_sp + 1].v.number - vm_stack[vm_sp].v.real);
          pVVar2[iVar3 + 1].v.number = iVar5;
          goto LAB_0808e69f;
        }
        goto LAB_0808e6ea;
      }
    }
    else {
      if (bVar1 == 6) {
                    /* Unresolved local var: Vm_Array_T * pNewArray@[DW_OP_reg3(EBX)] */
        if (vm_stack[vm_sp].type == '\x06') {
          pVVar7 = (Vm_String_T *)
                   vm_subArray((Vm_Array_T *)vm_stack[vm_sp + 1].v.string,
                               (Vm_Array_T *)vm_stack[vm_sp].v.string);
          vm_unrefArray((Vm_Array_T *)pVVar2[iVar3 + 1].v.string);
          pVVar2[iVar3 + 1].v.string = pVVar7;
        }
        else {
          vm_error("Bad -, right can not be sub with an array.\n");
        }
        goto LAB_0808e69f;
      }
      if (bVar1 == 7) {
                    /* Unresolved local var: Vm_Mapping_T * pNewMapping@[DW_OP_reg3(EBX)] */
        if (vm_stack[vm_sp].type == '\x06') {
          pVVar6 = vm_subMappingWithArray
                             ((Vm_Mapping_T *)vm_stack[vm_sp + 1].v.string,
                              (Vm_Array_T *)vm_stack[vm_sp].v.string,1);
        }
        else {
          if (vm_stack[vm_sp].type != '\a') {
            vm_error("Bad -, right can not be sub with a mapping or an array.\n");
            goto LAB_0808e69f;
          }
          pVVar6 = vm_subMapping((Vm_Mapping_T *)vm_stack[vm_sp + 1].v.string,
                                 (Vm_Mapping_T *)vm_stack[vm_sp].v.string,1);
        }
        vm_unrefMapping((Vm_Mapping_T *)pVVar2[iVar3 + 1].v.string);
        pVVar2[iVar3 + 1].v.string = (Vm_String_T *)pVVar6;
        goto LAB_0808e69f;
      }
    }
    pcVar4 = vm_valueType((uint)bVar1);
    vm_error("Bad -, type invalid (got %s) of left value.\n",pcVar4);
  }
LAB_0808e69f:
  vm_popStack();
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
- Ghidra Listing reports a terminal call at 0x808e6a5 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e708 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_valueType / procedure_pseudo_code
Evidence: `ev_1faf74eae5f56d8dc203ea31368aeed4b1e7b857c16c42c97b00ec8937436ac8`

```c

char * vm_valueType(int type)

{
  char *pcVar1;
  
  pcVar1 = "array";
  if (type != 6) {
    if (type < 7) {
      pcVar1 = "integer";
      if (type != 2) {
        if (type < 3) {
          pcVar1 = "undefined";
          if (type != 0) {
            if (type != 1) goto LAB_0808a16c;
            pcVar1 = "void";
          }
        }
        else {
          pcVar1 = "string";
          if (type != 4) {
            if (4 < type) {
              return "buffer";
            }
            return "float";
          }
        }
      }
    }
    else {
      pcVar1 = "function";
      if (type != 9) {
        if (type < 10) {
          if (type == 7) {
            return "mapping";
          }
          if (type == 8) {
            return "pointer";
          }
        }
        else {
          if (type == 0x14) {
            return "loop_ctrl";
          }
          if (type < 0x15) {
            if (type == 10) {
              return "object";
            }
          }
          else if (type == 0xff) {
            return "mixed";
          }
        }
LAB_0808a16c:
        vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value.\n",
                   "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x6bf);
        _vm_endProcess(0);
        do {
                    /* WARNING: Do nothing block with infinite loop */
        } while( true );
      }
    }
  }
  return pcVar1;
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
