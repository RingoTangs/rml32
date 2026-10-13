# 分派与对象调用：REA/Ghidra 证据

原始伪代码，不保证可编译；没有执行目标。

<a id="vm_callInOtherFunEx"></a>

## vm_callInOtherFunEx @ 0x80bd0fc

Evidence: `ev_13e8fe9368cca520ed154767ba3471e1ac37473f0886ab1b15365d0e8db01662`

```c

void vm_callInOtherFunEx(int callFlags,int argn)

{
  Vm_String_T *pVVar1;
  uint uVar2;
  uint uVar3;
  Vm_Value *pVVar4;
  int iVar5;
  Vm_Array_T *pArray;
  uint unaff_ESI;
  uint uVar6;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Combined_Cookie_T combinedCookie@[DW_OP_reg6(ESI)]
                        */
  uVar2 = callFlags & 1;
  uVar3 = vm_sp << 0x14;
  uVar6 = unaff_ESI & 0xfe | uVar2 | (argn & 0xfffU) << 8;
  pVVar4 = vm_stackTopValue();
  if (pVVar4->type == '\x06') {
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Combined_Cookie_T combinedCookie@[DW_OP_reg3(EBX)]
                        */
    pVVar4 = vm_stackTopValue();
    pVVar1 = (pVVar4->v).string;
    vm_pushNumber(0);
    pArray = vm_allocateEmptyArray(pVVar1->attrib);
    vm_pushArray(pArray);
    vm_callInOtherArray2((void *)(uVar6 | vm_sp << 0x14));
    if (uVar2 != 0) {
      vm_ctrlFlags = vm_ctrlFlags | 0x10;
      return;
    }
  }
  else {
    iVar5 = vm_callInOtherFun(callFlags,argn);
    if (iVar5 != 0) {
      vm_hookCallbackWhenReturn(vm_callInOtherFunExReturn,(void *)(uVar6 | uVar3));
      return;
    }
                    /* Unresolved local var: Vm_Combined_Cookie_T combinedCookie@[???] */
    if (uVar2 != 0) {
      vm_returnFromEfun();
      return;
    }
  }
  return;
}


```

<details><summary>汇编</summary>

```asm
0x80bd0fc: PUSH EBP
0x80bd0fd: MOV EBP, ESP
0x80bd0ff: PUSH EDI
0x80bd100: PUSH ESI
0x80bd101: PUSH EBX
0x80bd102: SUB ESP, 0xc
0x80bd105: MOV EDI, dword ptr [EBP + 0x8]
0x80bd108: MOV EAX, EDI
0x80bd10a: AND EAX, 0x1
0x80bd10d: MOV EBX, dword ptr [EBP + 0xc]
0x80bd110: AND ESI, 0xfffffffe
0x80bd113: OR ESI, EAX
0x80bd115: MOV EAX, EBX
0x80bd117: AND EAX, 0xfff
0x80bd11c: SHL EAX, 0x8
0x80bd11f: AND ESI, 0xfff000ff
0x80bd125: OR ESI, EAX
0x80bd127: MOV EAX, [0x086619cc]
0x80bd12c: SHL EAX, 0x14
0x80bd12f: AND ESI, 0xfffff
0x80bd135: OR ESI, EAX
0x80bd137: CALL 0x08090604
0x80bd13c: CMP byte ptr [EAX + 0x4], 0x6
0x80bd140: JZ 0x080bd178
0x80bd142: SUB ESP, 0x8
0x80bd145: PUSH EBX
0x80bd146: PUSH EDI
0x80bd147: CALL 0x08094cd8
0x80bd14c: ADD ESP, 0x10
0x80bd14f: TEST EAX, EAX
0x80bd151: JNZ 0x080bd160
0x80bd153: AND ESI, 0x1
0x80bd156: JNZ 0x080bd1cc
0x80bd158: LEA ESP, [EBP + -0xc]
0x80bd15b: POP EBX
0x80bd15c: POP ESI
0x80bd15d: POP EDI
0x80bd15e: LEAVE
0x80bd15f: RET
0x80bd160: MOV dword ptr [EBP + 0xc], ESI
0x80bd163: MOV dword ptr [EBP + 0x8], 0x80bcdd8
0x80bd16a: LEA ESP, [EBP + -0xc]
0x80bd16d: POP EBX
0x80bd16e: POP ESI
0x80bd16f: POP EDI
0x80bd170: LEAVE
0x80bd171: JMP 0x08094e70
0x80bd178: CALL 0x08090604
0x80bd17d: SUB ESP, 0xc
0x80bd180: MOV EBX, dword ptr [EAX + 0x8]
0x80bd183: PUSH 0x0
0x80bd185: CALL 0x08090b3c
0x80bd18a: POP EDX
0x80bd18b: PUSH dword ptr [EBX + 0x4]
0x80bd18e: CALL 0x0809b1d8
0x80bd193: MOV dword ptr [ESP], EAX
0x80bd196: CALL 0x08090d84
0x80bd19b: MOV EBX, ESI
0x80bd19d: MOV EAX, [0x086619cc]
0x80bd1a2: SHL EAX, 0x14
0x80bd1a5: AND EBX, 0xfffff
0x80bd1ab: OR EBX, EAX
0x80bd1ad: MOV dword ptr [ESP], EBX
0x80bd1b0: CALL 0x080bcf14
0x80bd1b5: ADD ESP, 0x10
0x80bd1b8: AND EBX, 0x1
0x80bd1bb: JZ 0x080bd158
0x80bd1bd: OR dword ptr [0x086619d4], 0x10
0x80bd1c4: LEA ESP, [EBP + -0xc]
0x80bd1c7: POP EBX
0x80bd1c8: POP ESI
0x80bd1c9: POP EDI
0x80bd1ca: LEAVE
0x80bd1cb: RET
0x80bd1cc: LEA ESP, [EBP + -0xc]
0x80bd1cf: POP EBX
0x80bd1d0: POP ESI
0x80bd1d1: POP EDI
0x80bd1d2: LEAVE
0x80bd1d3: JMP 0x08093c5c
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；Ghidra Listing reports a terminal call at 0x80bd171 to 0x8094e70 vm_hookCallbackWhenReturn (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.；Ghidra Listing reports a terminal call at 0x80bd1d3 to 0x8093c5c vm_returnFromEfun (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_createFunctionValue"></a>

## vm_createFunctionValue @ 0x8091f34

Evidence: `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`

```c

void vm_createFunctionValue(Vm_Index_T funOffset,int isGlobalTypeArgn)

{
  uint isGlobal;
  int type;
  ushort argn;
  
                    /* Unresolved local var: int isGlobal@[DW_OP_reg1(ECX)]
                       Unresolved local var: int type@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Index_T argn@[DW_OP_reg7(EDI)] */
  type = (-(uint)(((uint)isGlobalTypeArgn >> 6 & 1) == 0) & 0x18) + 0x4b;
  argn = (ushort)isGlobalTypeArgn & 0x3f;
  isGlobal = (uint)isGlobalTypeArgn >> 7 & 1;
  if (isGlobal == 0) {
    vm_createFunctionValueEx
              (0,type,funOffset + vm_currentContext->consititution->baseFunOffset,argn,
               vm_currentContext->thisObject);
    return;
  }
  vm_createFunctionValueEx(isGlobal,type,funOffset,argn,vm_currentContext->thisObject);
  return;
}


```

<details><summary>汇编</summary>

```asm
0x8091f34: PUSH EBP
0x8091f35: MOV EBP, ESP
0x8091f37: PUSH EDI
0x8091f38: PUSH ESI
0x8091f39: PUSH EBX
0x8091f3a: SUB ESP, 0xc
0x8091f3d: MOV EAX, dword ptr [EBP + 0xc]
0x8091f40: MOV EDX, EAX
0x8091f42: SHR EDX, 0x6
0x8091f45: MOV ECX, EAX
0x8091f47: AND EDX, 0x1
0x8091f4a: SHR ECX, 0x7
0x8091f4d: CMP EDX, 0x1
0x8091f50: SBB EBX, EBX
0x8091f52: AND EBX, 0x18
0x8091f55: MOV EDI, EAX
0x8091f57: ADD EBX, 0x4b
0x8091f5a: AND EDI, 0x3f
0x8091f5d: AND ECX, 0x1
0x8091f60: MOV ESI, dword ptr [EBP + 0x8]
0x8091f63: JNZ 0x08091f90
0x8091f65: MOV EDX, dword ptr [0x086619d8]
0x8091f6b: MOV EAX, dword ptr [EDX + 0x14]
0x8091f6e: SUB ESP, 0xc
0x8091f71: ADD SI, word ptr [EAX + 0x6]
0x8091f75: PUSH dword ptr [EDX + 0x20]
0x8091f78: MOVSX EAX, DI
0x8091f7b: PUSH EAX
0x8091f7c: MOVSX EAX, SI
0x8091f7f: PUSH EAX
0x8091f80: PUSH EBX
0x8091f81: PUSH ECX
0x8091f82: CALL 0x08091cd0
0x8091f87: LEA ESP, [EBP + -0xc]
0x8091f8a: POP EBX
0x8091f8b: POP ESI
0x8091f8c: POP EDI
0x8091f8d: LEAVE
0x8091f8e: RET
0x8091f90: SUB ESP, 0xc
0x8091f93: MOV EDX, dword ptr [0x086619d8]
0x8091f99: PUSH dword ptr [EDX + 0x20]
0x8091f9c: MOVSX EAX, DI
0x8091f9f: PUSH EAX
0x8091fa0: MOVSX EAX, SI
0x8091fa3: PUSH EAX
0x8091fa4: PUSH EBX
0x8091fa5: PUSH ECX
0x8091fa6: CALL 0x08091cd0
0x8091fab: LEA ESP, [EBP + -0xc]
0x8091fae: POP EBX
0x8091faf: POP ESI
0x8091fb0: POP EDI
0x8091fb1: LEAVE
0x8091fb2: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_callInOtherFun"></a>

## vm_callInOtherFun @ 0x8094cd8

Evidence: `ev_4b89eec3bfa71f6f357f508600108ff4f54e9492ffa2c51e8012d5d1a8ce8db0`

```c

/* WARNING: Unknown calling convention */

int vm_callInOtherFun(int callFlags,int argn)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  Vm_Fun *pFun;
  Vm_Object *unaff_EDI;
  char *str;
  Vm_Index_T funOffset;
  
  iVar2 = vm_sp;
                    /* Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Value_T * pValue2@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_stack;
  if (vm_stack[vm_sp].type == '\x04') {
    unaff_EDI = vm_lookupObjectByName(vm_stack[vm_sp].v.string);
    if ((unaff_EDI == (Vm_Object_T *)0x0) || (unaff_EDI->flags == 0x800)) goto LAB_08094d07;
  }
  else {
    if (vm_stack[vm_sp].type == '\n') {
      unaff_EDI = vm_stack[vm_sp].v.object;
      if ((unaff_EDI->flags & 0x8000) == 0) goto LAB_08094d17;
      str = "Object is not found (destructed).\n";
    }
    else {
LAB_08094d07:
      str = "Object is not found.\n";
    }
    vm_error(str);
  }
LAB_08094d17:
  if (pVVar1[iVar2 + 1].type != '\x04') {
    vm_error("Function is not found.\n");
  }
  pFun = vm_findProgramFunction
                   (unaff_EDI->pProgram,pVVar1[iVar2 + 1].v.string,callFlags & 2U,&funOffset);
  if (pFun == (Vm_Fun *)0x0) {
    pFun = vm_findProgramFunctionByCStr
                     (unaff_EDI->pProgram,"default_interface",callFlags & 2U,&funOffset);
    if (pFun != (Vm_Fun *)0x0) {
      vm_popStack();
      argn = argn + 1;
      goto LAB_08094d5d;
    }
    vm_popNStack(2);
  }
  else {
    vm_popNStack(2);
LAB_08094d5d:
    if (pFun != (Vm_Fun *)0x0) goto LAB_08094d61;
  }
  if ((callFlags & 4U) != 0) {
    vm_popNStack(argn);
    vm_pushVoid();
    return 0;
  }
  vm_error("Function is not found.\n");
LAB_08094d61:
  vm_callInUserFunBody(0,unaff_EDI,funOffset,pFun,argn);
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8094cd8: PUSH EBP
0x8094cd9: MOV EBP, ESP
0x8094cdb: PUSH EDI
0x8094cdc: PUSH ESI
0x8094cdd: PUSH EBX
0x8094cde: SUB ESP, 0xc
0x8094ce1: MOV EAX, [0x086619cc]
0x8094ce6: LEA EAX, [EAX + EAX*0x2]
0x8094ce9: MOV EDX, dword ptr [0x08661964]
0x8094cef: LEA EDX, [EDX + EAX*0x8]
0x8094cf2: MOVZX EAX, byte ptr [EDX + 0x4]
0x8094cf6: CMP EAX, 0x4
0x8094cf9: LEA EBX, [EDX + 0x18]
0x8094cfc: JZ 0x08094e04
0x8094d02: CMP EAX, 0xa
0x8094d05: JZ 0x08094d84
0x8094d07: SUB ESP, 0xc
0x8094d0a: PUSH 0x82ae0ea
0x8094d0f: CALL 0x08097b24
0x8094d14: ADD ESP, 0x10
0x8094d17: CMP byte ptr [EBX + 0x4], 0x4
0x8094d1b: JZ 0x08094d2d
0x8094d1d: SUB ESP, 0xc
0x8094d20: PUSH 0x82ae100
0x8094d25: CALL 0x08097b24
0x8094d2a: ADD ESP, 0x10
0x8094d2d: LEA EAX, [EBP + -0xe]
0x8094d30: PUSH EAX
0x8094d31: MOV ESI, dword ptr [EBP + 0x8]
0x8094d34: AND ESI, 0x2
0x8094d37: PUSH ESI
0x8094d38: PUSH dword ptr [EBX + 0x8]
0x8094d3b: PUSH dword ptr [EDI + 0x20]
0x8094d3e: CALL 0x08086b44
0x8094d43: ADD ESP, 0x10
0x8094d46: TEST EAX, EAX
0x8094d48: MOV EBX, EAX
0x8094d4a: JZ 0x08094ddc
0x8094d50: SUB ESP, 0xc
0x8094d53: PUSH 0x2
0x8094d55: CALL 0x08091ab0
0x8094d5a: ADD ESP, 0x10
0x8094d5d: TEST EBX, EBX
0x8094d5f: JZ 0x08094da8
0x8094d61: SUB ESP, 0xc
0x8094d64: PUSH dword ptr [EBP + 0xc]
0x8094d67: PUSH EBX
0x8094d68: MOVSX EAX, word ptr [EBP + -0xe]
0x8094d6c: PUSH EAX
0x8094d6d: PUSH EDI
0x8094d6e: PUSH 0x0
0x8094d70: CALL 0x08093eb0
0x8094d75: LEA ESP, [EBP + -0xc]
0x8094d78: POP EBX
0x8094d79: POP ESI
0x8094d7a: MOV EAX, 0x1
0x8094d7f: POP EDI
0x8094d80: LEAVE
0x8094d81: RET
0x8094d84: MOV EDI, dword ptr [EDX + 0x8]
0x8094d87: TEST byte ptr [EDI + 0xd], 0x80
0x8094d8b: JZ 0x08094d17
0x8094d8d: SUB ESP, 0xc
0x8094d90: PUSH 0x82ae690
0x8094d95: JMP 0x08094d0f
0x8094d9a: SUB ESP, 0xc
0x8094d9d: PUSH 0x2
0x8094d9f: CALL 0x08091ab0
0x8094da4: ADD ESP, 0x10
0x8094da7: NOP
0x8094da8: TEST byte ptr [EBP + 0x8], 0x4
0x8094dac: JZ 0x08094dc8
0x8094dae: SUB ESP, 0xc
0x8094db1: PUSH dword ptr [EBP + 0xc]
0x8094db4: CALL 0x08091ab0
0x8094db9: CALL 0x08090fc0
0x8094dbe: LEA ESP, [EBP + -0xc]
0x8094dc1: POP EBX
0x8094dc2: POP ESI
0x8094dc3: XOR EAX, EAX
0x8094dc5: POP EDI
0x8094dc6: LEAVE
0x8094dc7: RET
0x8094dc8: SUB ESP, 0xc
0x8094dcb: PUSH 0x82ae100
0x8094dd0: CALL 0x08097b24
0x8094dd5: ADD ESP, 0x10
0x8094dd8: JMP 0x08094d61
0x8094ddc: LEA EAX, [EBP + -0xe]
0x8094ddf: PUSH EAX
0x8094de0: PUSH ESI
0x8094de1: PUSH 0x82ae118
0x8094de6: PUSH dword ptr [EDI + 0x20]
0x8094de9: CALL 0x08086d68
0x8094dee: ADD ESP, 0x10
0x8094df1: TEST EAX, EAX
0x8094df3: MOV EBX, EAX
0x8094df5: JZ 0x08094d9a
0x8094df7: CALL 0x0809061c
0x8094dfc: INC dword ptr [EBP + 0xc]
0x8094dff: JMP 0x08094d5d
0x8094e04: SUB ESP, 0xc
0x8094e07: PUSH dword ptr [EDX + 0x8]
0x8094e0a: CALL 0x080b114c
0x8094e0f: ADD ESP, 0x10
0x8094e12: TEST EAX, EAX
0x8094e14: MOV EDI, EAX
0x8094e16: JZ 0x08094d07
0x8094e1c: CMP dword ptr [EAX + 0xc], 0x800
0x8094e23: JNZ 0x08094d17
0x8094e29: JMP 0x08094d07
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_pktDeclareInterface"></a>

## vm_pktDeclareInterface @ 0x80c2494

Evidence: `ev_77270df67d67e7cda8223a8169b5a62d0aecef52af7ee6847132044e05b259e2`

```c

void vm_pktDeclareInterface(int cmdFilter,char *content,Vm_Fun_Ptr_T *pFunction,int encapType)

{
  char cVar1;
  bool bVar2;
  char *pcVar3;
  Vm_Defn_T *pVVar4;
  Vm_Pkt_Interface_T *__s;
  Vm_Pkt_Member_T *pVVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  Vm_Pkt_Member_T *pVVar9;
  byte *pbVar10;
  char *pcVar11;
  byte *pbVar12;
  undefined1 uVar13;
  undefined1 uVar14;
  int msgNo;
  int receiveExtraData;
  int memberSize;
  char argDesc [256];
  char macroName [256];
  char interfaceName [256];
  Vm_Pkt_Member_T members [16];
  
                    /* Unresolved local var: Vm_Pkt_Interface_T * pPktInterface@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Defn_T * pMacroDef@[DW_OP_reg3(EBX)]
                       Unresolved local var: char * pArguments@[???]
                       Unresolved local var: char * p@[DW_OP_reg6(ESI)]
                       Unresolved local var: char * msgNoExp@[DW_OP_reg7(EDI)]
                       Unresolved local var: int i@[DW_OP_reg7(EDI)] */
  if (_pktAnalyserInited == 0) {
    vm_error("Packet analyser is not initialized yet.\n");
  }
  if (cmdFilter == 0) {
    vm_error("Bad filter (got empty mask) for packet interface.\n");
  }
  if (1 < encapType - 1U) {
    vm_error("Bad encapsulate type, expected array | mapping.\n");
  }
  pcVar3 = strchr(content,0x28);
  if (pcVar3 == (char *)0x0) {
    vm_error("Missed type name.\n");
  }
  uVar8 = (int)pcVar3 - (int)content;
  uVar13 = uVar8 < 0xff;
  uVar14 = uVar8 == 0xff;
  if (0xff < uVar8) {
    vm_error("Interface name is too long.\n");
    uVar13 = (undefined1 *)0xffffffef < &stack0xfffffa44;
    uVar14 = &stack0x00000000 == (undefined1 *)0x5ac;
  }
  memcpy(interfaceName,content,uVar8);
  interfaceName[uVar8] = '\0';
  vm_trimString(interfaceName);
  iVar6 = 5;
  pbVar12 = (byte *)interfaceName;
  pbVar10 = &DAT_082b3649;
  do {
    if (iVar6 == 0) break;
    iVar6 = iVar6 + -1;
    uVar13 = *pbVar12 < *pbVar10;
    uVar14 = *pbVar12 == *pbVar10;
    pbVar12 = pbVar12 + 1;
    pbVar10 = pbVar10 + 1;
  } while ((bool)uVar14);
  if ((!(bool)uVar13 && !(bool)uVar14) != (bool)uVar13) {
    vm_error("Bad synax of interface, expected void interface_name(...).\n");
  }
  builtin_strncpy(interfaceName,"     ",5);
  vm_trimString(interfaceName);
  if (interfaceName[0] == '\0') {
    vm_error("Bad interface name, got empty string.\n");
  }
  uVar8 = 0xffffffff;
  pcVar11 = pcVar3 + 1;
  do {
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    cVar1 = *pcVar11;
    pcVar11 = pcVar11 + 1;
  } while (cVar1 != '\0');
  if (0xff < ~uVar8 - 1) {
    vm_error("Argument description is too long.\n");
  }
  strcpy(argDesc,pcVar3 + 1);
  pcVar3 = strrchr(argDesc,0x29);
  if (pcVar3 == (char *)0x0) {
    vm_error("Missed \')\' of argument list.\n");
  }
  for (; argDesc < pcVar3; pcVar3 = pcVar3 + -1) {
    iVar6 = isspace((uint)(byte)pcVar3[-1]);
    if (iVar6 == 0) break;
  }
  *pcVar3 = '\0';
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???]
                       Unresolved local var: uchar * __s1@[???]
                       Unresolved local var: int __result@[???] */
  if ((((pcVar3 < argDesc + 3) || (pcVar3[-3] != '.')) || (pcVar3[-2] != '.')) ||
     ((pcVar3[-1] != '.' || (*pcVar3 != '\0')))) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
    pcVar3[-3] = '\0';
  }
  vm_deriveArguments(argDesc,&memberSize,members);
  if (memberSize < 0) {
    vm_error("Missed member list of interface.\n");
  }
  uVar8 = 0xffffffff;
  pcVar3 = interfaceName;
  do {
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  } while (cVar1 != '\0');
  argDesc[~uVar8 + 0xff] = '\0';
  for (iVar6 = ~uVar8 - 2; iVar6 != -1; iVar6 = iVar6 + -1) {
    iVar7 = toupper((uint)(byte)interfaceName[iVar6]);
    macroName[iVar6] = (char)iVar7;
  }
  pVVar4 = vm_lookupDefine(macroName);
  if (pVVar4 == (Vm_Defn_T *)0x0) {
    vm_error("Macro %s is not defined, can\'t resolve message no.\n");
  }
  if ((pVVar4->flags & 8) != 0) {
    vm_error("Macro %s is a function, can\'t resolve message no.\n");
  }
  if (pVVar4->nargs != -1) {
    vm_error("Can\'t expand macro %s to resolve message no since its need arguments.\n");
  }
  pbVar12 = (byte *)pVVar4->exps;
  while( true ) {
    iVar6 = vm_isspace((uint)*pbVar12);
    if (iVar6 == 0) break;
    pbVar12 = pbVar12 + 1;
  }
  if ((*pbVar12 == 0x30) && ((pbVar12[1] == 0x78 || (pbVar12[1] == 0x58)))) {
    iVar6 = 0x10;
    pbVar12 = pbVar12 + 2;
  }
  else {
    iVar6 = 10;
  }
  iVar6 = vm_strtol((char *)pbVar12,(char **)0x0,iVar6,0);
  if ((iVar6 < 1) || (_maxInterfaces <= iVar6)) {
    vm_error("Bad message no (%d), out of range.\n");
  }
  if (_allPktInterfaces[iVar6] != (Vm_Pkt_Interface_T *)0x0) {
    vm_error("The message no (%d) has already used.\n");
  }
  __s = pdb_allocateMemory(100,"VM","/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c",0x214);
  if (__s == (Vm_Pkt_Interface_T *)0x0) {
    vm_error("Failed to allocate memory for interface.\n");
  }
  memset(__s,0,100);
  __s->msgNo = iVar6;
  pVVar5 = pdb_allocateMemory(memberSize * 0x28,"VM",
                              "/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c",0x21d);
  (__s->pktDef).pMembers = pVVar5;
  if (pVVar5 == (Vm_Pkt_Member_T *)0x0) {
    pdb_freeMemory(__s,"VM","/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c",0x220);
    vm_error("Failed to allocate memory for interface packet definition.\n");
    pVVar5 = (__s->pktDef).pMembers;
  }
  pVVar9 = members;
  for (iVar7 = (memberSize * 5 & 0x1fffffffU) << 1; iVar7 != 0; iVar7 = iVar7 + -1) {
    *(undefined4 *)pVVar5->argName = *(undefined4 *)pVVar9->argName;
    pVVar9 = (Vm_Pkt_Member_T *)(pVVar9->argName + 4);
    pVVar5 = (Vm_Pkt_Member_T *)(pVVar5->argName + 4);
  }
  (__s->pktDef).encapType = encapType;
  (__s->pktDef).memberSize = memberSize;
  (__s->pktDef).matchType = 0xff;
  strncpy((__s->pktDef).defName,interfaceName,0x3f);
  (__s->pktDef).defName[0x3f] = '\0';
  __s->cmdFilter = cmdFilter;
  pFunction->ref = pFunction->ref + 1;
  __s->pFunction = pFunction;
  if (bVar2) {
    __s->attrib = __s->attrib | 1;
  }
  _allPktInterfaces[iVar6] = __s;
  return;
}


```

<details><summary>汇编</summary>

```asm
0x80c2494: PUSH EBP
0x80c2495: MOV EBP, ESP
0x80c2497: PUSH EDI
0x80c2498: PUSH ESI
0x80c2499: PUSH EBX
0x80c249a: SUB ESP, 0x59c
0x80c24a0: MOV EBX, dword ptr [0x0860b228]
0x80c24a6: TEST EBX, EBX
0x80c24a8: MOV ESI, dword ptr [EBP + 0xc]
0x80c24ab: JZ 0x080c288e
0x80c24b1: MOV ECX, dword ptr [EBP + 0x8]
0x80c24b4: TEST ECX, ECX
0x80c24b6: JZ 0x080c2879
0x80c24bc: MOV EAX, dword ptr [EBP + 0x14]
0x80c24bf: DEC EAX
0x80c24c0: CMP EAX, 0x1
0x80c24c3: JA 0x080c2864
0x80c24c9: SUB ESP, 0x8
0x80c24cc: PUSH 0x28
0x80c24ce: PUSH ESI
0x80c24cf: CALL 0x0804af44
0x80c24d4: ADD ESP, 0x10
0x80c24d7: TEST EAX, EAX
0x80c24d9: MOV dword ptr [EBP + 0xfffffa58], EAX
0x80c24df: JZ 0x080c2970
0x80c24e5: MOV EBX, dword ptr [EBP + 0xfffffa58]
0x80c24eb: SUB EBX, ESI
0x80c24ed: CMP EBX, 0xff
0x80c24f3: JA 0x080c284f
0x80c24f9: PUSH EDX
0x80c24fa: PUSH EBX
0x80c24fb: PUSH ESI
0x80c24fc: LEA EAX, [EBP + 0xfffffc68]
0x80c2502: PUSH EAX
0x80c2503: CALL 0x0804b7e4
0x80c2508: LEA EDX, [EBP + 0xfffffc68]
0x80c250e: MOV dword ptr [ESP], EDX
0x80c2511: MOV byte ptr [EBP + EBX*0x1 + 0xfffffc68], 0x0
0x80c2519: MOV EDI, 0x82b3649
0x80c251e: CALL 0x080b6df4
0x80c2523: LEA ESI, [EBP + 0xfffffc68]
0x80c2529: MOV ECX, 0x5
0x80c252e: CLD
0x80c252f: CMPSB.REPE ES:EDI, ESI
0x80c2531: SETA DL
0x80c2534: SETC AL
0x80c2537: ADD ESP, 0x10
0x80c253a: CMP DL, AL
0x80c253c: JNZ 0x080c283a
0x80c2542: SUB ESP, 0xc
0x80c2545: LEA EAX, [EBP + 0xfffffc68]
0x80c254b: PUSH EAX
0x80c254c: MOV dword ptr [EBP + 0xfffffc68], 0x20202020
0x80c2556: MOV byte ptr [EBP + 0xfffffc6c], 0x20
0x80c255d: CALL 0x080b6df4
0x80c2562: ADD ESP, 0x10
0x80c2565: CMP byte ptr [EBP + 0xfffffc68], 0x0
0x80c256c: JZ 0x080c2825
0x80c2572: MOV EDX, dword ptr [EBP + 0xfffffa58]
0x80c2578: LEA EBX, [EDX + 0x1]
0x80c257b: XOR EAX, EAX
0x80c257d: CLD
0x80c257e: MOV ECX, 0xffffffff
0x80c2583: MOV EDI, EBX
0x80c2585: SCASB.REPNE ES:EDI
0x80c2587: NOT ECX
0x80c2589: DEC ECX
0x80c258a: CMP ECX, 0xff
0x80c2590: JA 0x080c2810
0x80c2596: SUB ESP, 0x8
0x80c2599: PUSH EBX
0x80c259a: LEA EBX, [EBP + 0xfffffa68]
0x80c25a0: PUSH EBX
0x80c25a1: CALL 0x0804bc54
0x80c25a6: SUB ESP, 0x8
0x80c25a9: PUSH 0x29
0x80c25ab: PUSH EBX
0x80c25ac: CALL 0x0804b844
0x80c25b1: ADD ESP, 0x20
0x80c25b4: TEST EAX, EAX
0x80c25b6: MOV ESI, EAX
0x80c25b8: JNZ 0x080c25d5
0x80c25ba: JMP 0x080c295b
0x80c25c0: SUB ESP, 0xc
0x80c25c3: MOVZX EAX, byte ptr [ESI + -0x1]
0x80c25c7: PUSH EAX
0x80c25c8: CALL 0x0804b074
0x80c25cd: ADD ESP, 0x10
0x80c25d0: TEST EAX, EAX
0x80c25d2: JZ 0x080c25d9
0x80c25d4: DEC ESI
0x80c25d5: CMP EBX, ESI
0x80c25d7: JC 0x080c25c0
0x80c25d9: LEA EAX, [EBP + 0xfffffa6b]
0x80c25df: CMP EAX, ESI
0x80c25e1: MOV byte ptr [ESI], 0x0
0x80c25e4: JA 0x080c25f9
0x80c25e6: CMP byte ptr [ESI + -0x3], 0x2e
0x80c25ea: LEA EAX, [ESI + -0x3]
0x80c25ed: JNZ 0x080c25f9
0x80c25ef: CMP byte ptr [EAX + 0x1], 0x2e
0x80c25f3: JZ 0x080c28b9
0x80c25f9: MOV dword ptr [EBP + 0xfffffa60], 0x0
0x80c2603: SUB ESP, 0xc
0x80c2606: LEA ESI, [EBP + 0xfffffd68]
0x80c260c: PUSH ESI
0x80c260d: MOV EAX, EBX
0x80c260f: LEA EDX, [EBP + 0xfffffa64]
0x80c2615: CALL 0x080c2178
0x80c261a: MOV EAX, dword ptr [EBP + 0xfffffa64]
0x80c2620: ADD ESP, 0x10
0x80c2623: TEST EAX, EAX
0x80c2625: JS 0x080c2930
0x80c262b: LEA EDI, [EBP + 0xfffffc68]
0x80c2631: CLD
0x80c2632: MOV ECX, 0xffffffff
0x80c2637: XOR EAX, EAX
0x80c2639: SCASB.REPNE ES:EDI
0x80c263b: NOT ECX
0x80c263d: LEA EDI, [ECX + -0x2]
0x80c2640: MOV byte ptr [ECX + EBP*0x1 + 0xfffffb67], 0x0
0x80c2648: JMP 0x080c2668
0x80c264c: SUB ESP, 0xc
0x80c264f: MOVZX EAX, byte ptr [EBP + EDI*0x1 + 0xfffffc68]
0x80c2657: PUSH EAX
0x80c2658: CALL 0x0804b734
0x80c265d: MOV byte ptr [EBP + EDI*0x1 + 0xfffffb68], AL
0x80c2664: ADD ESP, 0x10
0x80c2667: DEC EDI
0x80c2668: CMP EDI, -0x1
0x80c266b: JNZ 0x080c264c
0x80c266d: SUB ESP, 0xc
0x80c2670: LEA EDI, [EBP + 0xfffffb68]
0x80c2676: PUSH EDI
0x80c2677: CALL 0x0807eab4
0x80c267c: ADD ESP, 0x10
0x80c267f: TEST EAX, EAX
0x80c2681: MOV EBX, EAX
0x80c2683: JZ 0x080c2945
0x80c2689: TEST byte ptr [EBX + 0xc], 0x8
0x80c268d: JNZ 0x080c28a3
0x80c2693: CMP dword ptr [EBX + 0x10], -0x1
0x80c2697: JZ 0x080c26aa
0x80c2699: SUB ESP, 0x8
0x80c269c: PUSH EDI
0x80c269d: PUSH 0x82b3170
0x80c26a2: CALL 0x08097b24
0x80c26a7: ADD ESP, 0x10
0x80c26aa: MOV EDI, dword ptr [EBX + 0x8]
0x80c26ad: JMP 0x080c26b1
0x80c26b0: INC EDI
0x80c26b1: SUB ESP, 0xc
0x80c26b4: MOVZX EAX, byte ptr [EDI]
0x80c26b7: PUSH EAX
0x80c26b8: CALL 0x080bd79c
0x80c26bd: ADD ESP, 0x10
0x80c26c0: TEST EAX, EAX
0x80c26c2: JNZ 0x080c26b0
0x80c26c4: CMP byte ptr [EDI], 0x30
0x80c26c7: JNZ 0x080c26dc
0x80c26c9: MOV AL, byte ptr [EDI + 0x1]
0x80c26cc: CMP AL, 0x78
0x80c26ce: JZ 0x080c2921
0x80c26d4: CMP AL, 0x58
0x80c26d6: JZ 0x080c2921
0x80c26dc: PUSH 0x0
0x80c26de: PUSH 0xa
0x80c26e0: PUSH 0x0
0x80c26e2: PUSH EDI
0x80c26e3: CALL 0x080b64d8
0x80c26e8: MOV dword ptr [EBP + 0xfffffa5c], EAX
0x80c26ee: MOV EAX, dword ptr [EBP + 0xfffffa5c]
0x80c26f4: ADD ESP, 0x10
0x80c26f7: TEST EAX, EAX
0x80c26f9: JLE 0x080c2709
0x80c26fb: MOV EAX, dword ptr [EBP + 0xfffffa5c]
0x80c2701: CMP EAX, dword ptr [0x083cfc44]
0x80c2707: JL 0x080c271f
0x80c2709: SUB ESP, 0x8
0x80c270c: PUSH dword ptr [EBP + 0xfffffa5c]
0x80c2712: PUSH 0x82b31b8
0x80c2717: CALL 0x08097b24
0x80c271c: ADD ESP, 0x10
0x80c271f: MOV EAX, [0x0860b224]
0x80c2724: MOV EDX, dword ptr [EBP + 0xfffffa5c]
0x80c272a: MOV EAX, dword ptr [EAX + EDX*0x4]
0x80c272d: TEST EAX, EAX
0x80c272f: JNZ 0x080c290b
0x80c2735: PUSH 0x214
0x80c273a: PUSH 0x82b2fc4
0x80c273f: PUSH 0x82a376b
0x80c2744: PUSH 0x64
0x80c2746: CALL 0x080ce784
0x80c274b: ADD ESP, 0x10
0x80c274e: TEST EAX, EAX
0x80c2750: MOV EBX, EAX
0x80c2752: JZ 0x080c2985
0x80c2758: PUSH EAX
0x80c2759: PUSH 0x64
0x80c275b: PUSH 0x0
0x80c275d: PUSH EBX
0x80c275e: CALL 0x0804ba54
0x80c2763: MOV EAX, dword ptr [EBP + 0xfffffa5c]
0x80c2769: MOV dword ptr [EBX], EAX
0x80c276b: PUSH 0x21d
0x80c2770: MOV EAX, dword ptr [EBP + 0xfffffa64]
0x80c2776: PUSH 0x82b2fc4
0x80c277b: LEA EAX, [EAX + EAX*0x4]
0x80c277e: PUSH 0x82a376b
0x80c2783: SHL EAX, 0x3
0x80c2786: PUSH EAX
0x80c2787: CALL 0x080ce784
0x80c278c: ADD ESP, 0x20
0x80c278f: TEST EAX, EAX
0x80c2791: MOV EDI, EAX
0x80c2793: MOV dword ptr [EBX + 0x5c], EAX
0x80c2796: JZ 0x080c28df
0x80c279c: MOV ECX, dword ptr [EBP + 0xfffffa64]
0x80c27a2: LEA ECX, [ECX + ECX*0x4]
0x80c27a5: SHL ECX, 0x3
0x80c27a8: CLD
0x80c27a9: SHR ECX, 0x2
0x80c27ac: MOVSD.REP ES:EDI, ESI
0x80c27ae: MOV EAX, dword ptr [EBP + 0xfffffa64]
0x80c27b4: MOV EDX, dword ptr [EBP + 0x14]
0x80c27b7: MOV dword ptr [EBX + 0x54], EDX
0x80c27ba: MOV dword ptr [EBX + 0x58], EAX
0x80c27bd: MOV dword ptr [EBX + 0x50], 0xff
0x80c27c4: PUSH EAX
0x80c27c5: PUSH 0x3f
0x80c27c7: LEA EAX, [EBP + 0xfffffc68]
0x80c27cd: PUSH EAX
0x80c27ce: LEA EAX, [EBX + 0x10]
0x80c27d1: PUSH EAX
0x80c27d2: CALL 0x0804ba94
0x80c27d7: MOV byte ptr [EBX + 0x4f], 0x0
0x80c27db: MOV EDX, dword ptr [EBP + 0x8]
0x80c27de: MOV EAX, dword ptr [EBP + 0x10]
0x80c27e1: MOV dword ptr [EBX + 0x4], EDX
0x80c27e4: INC dword ptr [EAX]
0x80c27e6: MOV dword ptr [EBX + 0xc], EAX
0x80c27e9: MOV EAX, dword ptr [EBP + 0xfffffa60]
0x80c27ef: ADD ESP, 0x10
0x80c27f2: TEST EAX, EAX
0x80c27f4: JZ 0x080c27fa
0x80c27f6: OR dword ptr [EBX + 0x8], 0x1
0x80c27fa: MOV EAX, [0x0860b224]
0x80c27ff: MOV EDX, dword ptr [EBP + 0xfffffa5c]
0x80c2805: MOV dword ptr [EAX + EDX*0x4], EBX
0x80c2808: LEA ESP, [EBP + -0xc]
0x80c280b: POP EBX
0x80c280c: POP ESI
0x80c280d: POP EDI
0x80c280e: LEAVE
0x80c280f: RET
0x80c2810: SUB ESP, 0xc
0x80c2813: PUSH 0x82b31dc
0x80c2818: CALL 0x08097b24
0x80c281d: ADD ESP, 0x10
0x80c2820: JMP 0x080c2596
0x80c2825: SUB ESP, 0xc
0x80c2828: PUSH 0x82b3200
0x80c282d: CALL 0x08097b24
0x80c2832: ADD ESP, 0x10
0x80c2835: JMP 0x080c2572
0x80c283a: SUB ESP, 0xc
0x80c283d: PUSH 0x82b3228
0x80c2842: CALL 0x08097b24
0x80c2847: ADD ESP, 0x10
0x80c284a: JMP 0x080c2542
0x80c284f: SUB ESP, 0xc
0x80c2852: PUSH 0x82b364f
0x80c2857: CALL 0x08097b24
0x80c285c: ADD ESP, 0x10
0x80c285f: JMP 0x080c24f9
0x80c2864: SUB ESP, 0xc
0x80c2867: PUSH 0x82b3264
0x80c286c: CALL 0x08097b24
0x80c2871: ADD ESP, 0x10
0x80c2874: JMP 0x080c24c9
0x80c2879: SUB ESP, 0xc
0x80c287c: PUSH 0x82b3298
0x80c2881: CALL 0x08097b24
0x80c2886: ADD ESP, 0x10
0x80c2889: JMP 0x080c24bc
0x80c288e: SUB ESP, 0xc
0x80c2891: PUSH 0x82b2ff8
0x80c2896: CALL 0x08097b24
0x80c289b: ADD ESP, 0x10
0x80c289e: JMP 0x080c24b1
0x80c28a3: SUB ESP, 0x8
0x80c28a6: PUSH EDI
0x80c28a7: PUSH 0x82b32cc
0x80c28ac: CALL 0x08097b24
0x80c28b1: ADD ESP, 0x10
0x80c28b4: JMP 0x080c2693
0x80c28b9: CMP byte ptr [EAX + 0x2], 0x2e
0x80c28bd: JNZ 0x080c25f9
0x80c28c3: CMP byte ptr [EAX + 0x3], 0x0
0x80c28c7: JNZ 0x080c25f9
0x80c28cd: MOV dword ptr [EBP + 0xfffffa60], 0x1
0x80c28d7: MOV byte ptr [EAX], 0x0
0x80c28da: JMP 0x080c2603
0x80c28df: PUSH 0x220
0x80c28e4: PUSH 0x82b2fc4
0x80c28e9: PUSH 0x82a376b
0x80c28ee: PUSH EBX
0x80c28ef: CALL 0x080ceb94
0x80c28f4: MOV dword ptr [ESP], 0x82b3300
0x80c28fb: CALL 0x08097b24
0x80c2900: ADD ESP, 0x10
0x80c2903: MOV EDI, dword ptr [EBX + 0x5c]
0x80c2906: JMP 0x080c279c
0x80c290b: SUB ESP, 0x8
0x80c290e: PUSH EDX
0x80c290f: PUSH 0x82b333c
0x80c2914: CALL 0x08097b24
0x80c2919: ADD ESP, 0x10
0x80c291c: JMP 0x080c2735
0x80c2921: PUSH 0x0
0x80c2923: PUSH 0x10
0x80c2925: PUSH 0x0
0x80c2927: LEA EAX, [EDI + 0x2]
0x80c292a: PUSH EAX
0x80c292b: JMP 0x080c26e3
0x80c2930: SUB ESP, 0xc
0x80c2933: PUSH 0x82b3364
0x80c2938: CALL 0x08097b24
0x80c293d: ADD ESP, 0x10
0x80c2940: JMP 0x080c262b
0x80c2945: SUB ESP, 0x8
0x80c2948: PUSH EDI
0x80c2949: PUSH 0x82b3388
0x80c294e: CALL 0x08097b24
0x80c2953: ADD ESP, 0x10
0x80c2956: JMP 0x080c2689
0x80c295b: SUB ESP, 0xc
0x80c295e: PUSH 0x82b366c
0x80c2963: CALL 0x08097b24
0x80c2968: ADD ESP, 0x10
0x80c296b: JMP 0x080c25d5
0x80c2970: SUB ESP, 0xc
0x80c2973: PUSH 0x82b368a
0x80c2978: CALL 0x08097b24
0x80c297d: ADD ESP, 0x10
0x80c2980: JMP 0x080c24e5
0x80c2985: SUB ESP, 0xc
0x80c2988: PUSH 0x82b33bc
0x80c298d: CALL 0x08097b24
0x80c2992: ADD ESP, 0x10
0x80c2995: JMP 0x080c2758
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_pktDeclareDefinition"></a>

## vm_pktDeclareDefinition @ 0x80c299c

Evidence: `ev_a402001e0d81dfa10a67890fef934bab077a58758de77637d0eab44eff96b045`

```c

void vm_pktDeclareDefinition(char *content)

{
  char *pcVar1;
  char *pcVar2;
  Vm_Pkt_Def_T *__dest;
  Vm_Pkt_Member_T *pVVar3;
  char *extraout_EAX;
  char *extraout_EAX_00;
  int iVar4;
  uint uVar5;
  Vm_Pkt_Def *pVVar6;
  Vm_Pkt_Member_T *pVVar7;
  char *pcVar8;
  char *pcVar9;
  bool bVar10;
  char *pcVar11;
  uint uVar12;
  char *pcVar13;
  Vm_Pkt_Def_T *pVVar14;
  int local_3b4;
  int encapType;
  char *defName;
  int memberSize;
  char encapTypeAndDefName [256];
  Vm_Pkt_Member_T members [16];
  
                    /* Unresolved local var: Vm_Pkt_Def_T * pPktDef@[???]
                       Unresolved local var: char * pEqual@[???]
                       Unresolved local var: char * encapTypeName@[???]
                       Unresolved local var: char * pEncapSeparator@[???] */
  if (_pktAnalyserInited == 0) {
    vm_error("Packet analyser is not initialized yet.\n");
  }
  pcVar1 = strchr(content,0x3d);
  pcVar2 = pcVar1;
  if (pcVar1 == (char *)0x0) {
    vm_error("Missed type name, expected \'=\' in definition.\n");
    pcVar2 = extraout_EAX_00;
  }
  uVar5 = (int)pcVar1 - (int)content;
  if (0xff < uVar5) {
    vm_error("Type name too long.\n");
    pcVar2 = extraout_EAX;
  }
  pcVar13 = encapTypeAndDefName;
  uVar12 = uVar5;
  memcpy(pcVar13,content,uVar5);
  encapTypeAndDefName[uVar5] = '\0';
  vm_trimString(pcVar13);
  if (encapTypeAndDefName[0] == '\0') {
    vm_error("Bad definition name, got empty string.\n",content,uVar12,pcVar2);
  }
  pcVar11 = &DAT_00000020;
  pcVar2 = strchr(pcVar13,0x20);
  if (pcVar2 == (char *)0x0) {
    vm_error("Missed encapsulation type, expected encap:none | encap:array | encap:mapping.\n");
  }
  *pcVar2 = '\0';
  pcVar2 = pcVar2 + 1;
  vm_trimString(pcVar13);
  vm_trimString(pcVar2);
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
  iVar4 = 0xe;
  bVar10 = &stack0x00000000 == (undefined1 *)0x3bc;
  pcVar8 = pcVar13;
  pcVar9 = "encap:mapping";
  do {
    if (iVar4 == 0) break;
    iVar4 = iVar4 + -1;
    bVar10 = *pcVar8 == *pcVar9;
    pcVar8 = pcVar8 + 1;
    pcVar9 = pcVar9 + 1;
  } while (bVar10);
  pVVar6 = _allPktDefs;
  if (bVar10) {
    encapType = 2;
    pcVar13 = pcVar11;
  }
  else {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    iVar4 = 0xc;
    pcVar8 = pcVar13;
    pcVar9 = "encap:array";
    do {
      if (iVar4 == 0) break;
      iVar4 = iVar4 + -1;
      bVar10 = *pcVar8 == *pcVar9;
      pcVar8 = pcVar8 + 1;
      pcVar9 = pcVar9 + 1;
    } while (bVar10);
    if (bVar10) {
      encapType = 1;
      pcVar13 = pcVar11;
    }
    else {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
      iVar4 = 0xb;
      pcVar8 = pcVar13;
      pcVar9 = "encap:none";
      do {
        if (iVar4 == 0) break;
        iVar4 = iVar4 + -1;
        bVar10 = *pcVar8 == *pcVar9;
        pcVar8 = pcVar8 + 1;
        pcVar9 = pcVar9 + 1;
      } while (bVar10);
      if (bVar10) {
        encapType = 0;
        pcVar13 = pcVar11;
      }
      else {
        vm_error("Bad encapsulation type, expected encap:none | encap:array | encap:mapping, got %s.\n"
                 ,pcVar13);
        pVVar6 = _allPktDefs;
      }
    }
  }
  for (; pVVar6 != (Vm_Pkt_Def_T *)0x0; pVVar6 = pVVar6->pNext) {
    pcVar13 = pcVar2;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
    iVar4 = strcmp(pVVar6->defName,pcVar2);
    if (iVar4 == 0) {
      if (pVVar6 != (Vm_Pkt_Def_T *)0x0) {
        pcVar13 = pcVar2;
        vm_error("Bad definition name %s, duplicated.\n",pcVar2);
      }
      break;
    }
  }
  vm_deriveArguments(pcVar1 + 1,&memberSize,members);
  if (memberSize < 1) {
    vm_error("Missed member list of definition.\n",pcVar13);
  }
  local_3b4 = memberSize;
  if ((1 < memberSize) && (encapType == 0)) {
    vm_error("Bad encapsulation type for multi-members, expected array | mapping.\n",pcVar13);
    local_3b4 = memberSize;
  }
                    /* Unresolved local var: Vm_Pkt_Def_T * pPktDef@[DW_OP_reg3(EBX)] */
  pVVar14 = (Vm_Pkt_Def_T *)0x56c;
  pcVar13 = "/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c";
  pcVar1 = "VM";
  __dest = pdb_allocateMemory(0x54,"VM","/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c",0x56c);
  if (__dest != (Vm_Pkt_Def_T *)0x0) {
    pVVar14 = __dest;
    memset(__dest,0,0x54);
    __dest->matchType = 0xff;
    pcVar13 = (char *)0x40;
    strncpy(__dest->defName,pcVar2,0x40);
    __dest->defName[0x3f] = '\0';
    __dest->encapType = 0;
    if (local_3b4 < 1) {
      __dest->pMembers = (Vm_Pkt_Member_T *)0x0;
      pcVar1 = pcVar2;
    }
    else {
      pVVar14 = (Vm_Pkt_Def_T *)0x57f;
      pcVar13 = "/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c";
      pcVar1 = "VM";
      pVVar3 = pdb_allocateMemory(local_3b4 * 0x28,"VM",
                                  "/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c",0x57f);
      __dest->pMembers = pVVar3;
      if (pVVar3 == (Vm_Pkt_Member_T *)0x0) {
        pVVar14 = (Vm_Pkt_Def_T *)0x583;
        pcVar13 = "/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c";
        pcVar1 = "VM";
        pdb_freeMemory(__dest,"VM","/home/cc/xxxx/gameshell/src/vm/vm_pkt_analyser.c",0x583);
        goto LAB_080c2c14;
      }
      pVVar7 = members;
      for (uVar5 = (uint)(local_3b4 * 0x28) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined4 *)pVVar3->argName = *(undefined4 *)pVVar7->argName;
        pVVar7 = (Vm_Pkt_Member_T *)(pVVar7->argName + 4);
        pVVar3 = (Vm_Pkt_Member_T *)(pVVar3->argName + 4);
      }
      __dest->memberSize = local_3b4;
    }
    __dest->pNext = _allPktDefs;
    _allPktDefs = __dest;
    if (__dest != (Vm_Pkt_Def_T *)0x0) {
      __dest->encapType = encapType;
      return;
    }
  }
LAB_080c2c14:
  vm_error("Failed to add into definition list, memory not enough.\n",pcVar1,pcVar13,pVVar14);
  iRam00000044 = encapType;
  return;
}


```

<details><summary>汇编</summary>

```asm
0x80c299c: PUSH EBP
0x80c299d: MOV EBP, ESP
0x80c299f: PUSH EDI
0x80c29a0: PUSH ESI
0x80c29a1: PUSH EBX
0x80c29a2: SUB ESP, 0x3ac
0x80c29a8: MOV EAX, [0x0860b228]
0x80c29ad: TEST EAX, EAX
0x80c29af: MOV EDI, dword ptr [EBP + 0x8]
0x80c29b2: JZ 0x080c2c35
0x80c29b8: SUB ESP, 0x8
0x80c29bb: PUSH 0x3d
0x80c29bd: PUSH EDI
0x80c29be: CALL 0x0804af44
0x80c29c3: ADD ESP, 0x10
0x80c29c6: TEST EAX, EAX
0x80c29c8: MOV dword ptr [EBP + 0xfffffc58], EAX
0x80c29ce: JZ 0x080c2cfb
0x80c29d4: MOV ESI, dword ptr [EBP + 0xfffffc58]
0x80c29da: SUB ESI, EDI
0x80c29dc: CMP ESI, 0xff
0x80c29e2: JA 0x080c2c5f
0x80c29e8: PUSH EAX
0x80c29e9: PUSH ESI
0x80c29ea: PUSH EDI
0x80c29eb: LEA EBX, [EBP + 0xfffffc68]
0x80c29f1: PUSH EBX
0x80c29f2: CALL 0x0804b7e4
0x80c29f7: MOV byte ptr [EBP + ESI*0x1 + 0xfffffc68], 0x0
0x80c29ff: MOV dword ptr [ESP], EBX
0x80c2a02: CALL 0x080b6df4
0x80c2a07: ADD ESP, 0x10
0x80c2a0a: CMP byte ptr [EBP + 0xfffffc68], 0x0
0x80c2a11: JZ 0x080c2c4a
0x80c2a17: SUB ESP, 0x8
0x80c2a1a: PUSH 0x20
0x80c2a1c: PUSH EBX
0x80c2a1d: CALL 0x0804af44
0x80c2a22: ADD ESP, 0x10
0x80c2a25: TEST EAX, EAX
0x80c2a27: MOV ESI, EAX
0x80c2a29: JZ 0x080c2d10
0x80c2a2f: MOV byte ptr [ESI], 0x0
0x80c2a32: SUB ESP, 0xc
0x80c2a35: INC ESI
0x80c2a36: MOV dword ptr [EBP + 0xfffffc60], ESI
0x80c2a3c: PUSH EBX
0x80c2a3d: CALL 0x080b6df4
0x80c2a42: POP EAX
0x80c2a43: PUSH dword ptr [EBP + 0xfffffc60]
0x80c2a49: CALL 0x080b6df4
0x80c2a4e: MOV EDI, 0x82b369d
0x80c2a53: MOV ECX, 0xe
0x80c2a58: CLD
0x80c2a59: MOV ESI, EBX
0x80c2a5b: ADD ESP, 0x10
0x80c2a5e: CMPSB.REPE ES:EDI, ESI
0x80c2a60: JNZ 0x080c2bb8
0x80c2a66: MOV dword ptr [EBP + 0xfffffc5c], 0x2
0x80c2a70: MOV ESI, dword ptr [0x0860b230]
0x80c2a76: JMP 0x080c2a95
0x80c2a78: SUB ESP, 0x8
0x80c2a7b: PUSH dword ptr [EBP + 0xfffffc60]
0x80c2a81: PUSH ESI
0x80c2a82: CALL 0x0804b064
0x80c2a87: ADD ESP, 0x10
0x80c2a8a: TEST EAX, EAX
0x80c2a8c: JZ 0x080c2c74
0x80c2a92: MOV ESI, dword ptr [ESI + 0x50]
0x80c2a95: TEST ESI, ESI
0x80c2a97: JNZ 0x080c2a78
0x80c2a99: SUB ESP, 0xc
0x80c2a9c: MOV EAX, dword ptr [EBP + 0xfffffc58]
0x80c2aa2: LEA ESI, [EBP + 0xfffffd68]
0x80c2aa8: INC EAX
0x80c2aa9: PUSH ESI
0x80c2aaa: LEA EDX, [EBP + 0xfffffc64]
0x80c2ab0: CALL 0x080c2178
0x80c2ab5: MOV EAX, dword ptr [EBP + 0xfffffc64]
0x80c2abb: ADD ESP, 0x10
0x80c2abe: TEST EAX, EAX
0x80c2ac0: MOV dword ptr [EBP + 0xfffffc50], EAX
0x80c2ac6: JLE 0x080c2cda
0x80c2acc: CMP dword ptr [EBP + 0xfffffc50], 0x1
0x80c2ad3: JLE 0x080c2ae3
0x80c2ad5: MOV EAX, dword ptr [EBP + 0xfffffc5c]
0x80c2adb: TEST EAX, EAX
0x80c2add: JZ 0x080c2cb9
0x80c2ae3: PUSH 0x56c
0x80c2ae8: PUSH 0x82b2fc4
0x80c2aed: PUSH 0x82a376b
0x80c2af2: PUSH 0x54
0x80c2af4: CALL 0x080ce784
0x80c2af9: XOR EDI, EDI
0x80c2afb: ADD ESP, 0x10
0x80c2afe: TEST EAX, EAX
0x80c2b00: MOV EBX, EAX
0x80c2b02: JZ 0x080c2c14
0x80c2b08: PUSH EAX
0x80c2b09: PUSH 0x54
0x80c2b0b: PUSH 0x0
0x80c2b0d: PUSH EBX
0x80c2b0e: CALL 0x0804ba54
0x80c2b13: ADD ESP, 0xc
0x80c2b16: MOV dword ptr [EBX + 0x40], 0xff
0x80c2b1d: PUSH 0x40
0x80c2b1f: PUSH dword ptr [EBP + 0xfffffc60]
0x80c2b25: PUSH EBX
0x80c2b26: CALL 0x0804ba94
0x80c2b2b: MOV byte ptr [EBX + 0x3f], 0x0
0x80c2b2f: MOV EAX, dword ptr [EBP + 0xfffffc50]
0x80c2b35: ADD ESP, 0x10
0x80c2b38: TEST EAX, EAX
0x80c2b3a: MOV dword ptr [EBX + 0x44], 0x0
0x80c2b41: JLE 0x080c2c97
0x80c2b47: PUSH 0x57f
0x80c2b4c: PUSH 0x82b2fc4
0x80c2b51: PUSH 0x82a376b
0x80c2b56: MOV EDX, dword ptr [EBP + 0xfffffc50]
0x80c2b5c: LEA EAX, [EDX + EDX*0x4]
0x80c2b5f: SHL EAX, 0x3
0x80c2b62: MOV dword ptr [EBP + 0xfffffc54], EAX
0x80c2b68: PUSH EAX
0x80c2b69: CALL 0x080ce784
0x80c2b6e: ADD ESP, 0x10
0x80c2b71: TEST EAX, EAX
0x80c2b73: MOV dword ptr [EBX + 0x4c], EAX
0x80c2b76: JZ 0x080c2bfc
0x80c2b7c: MOV ECX, dword ptr [EBP + 0xfffffc54]
0x80c2b82: MOV EDI, EAX
0x80c2b84: CLD
0x80c2b85: SHR ECX, 0x2
0x80c2b88: MOVSD.REP ES:EDI, ESI
0x80c2b8a: MOV EAX, dword ptr [EBP + 0xfffffc50]
0x80c2b90: MOV dword ptr [EBX + 0x48], EAX
0x80c2b93: MOV EAX, [0x0860b230]
0x80c2b98: TEST EBX, EBX
0x80c2b9a: MOV dword ptr [EBX + 0x50], EAX
0x80c2b9d: MOV EDI, EBX
0x80c2b9f: MOV dword ptr [0x0860b230], EBX
0x80c2ba5: JZ 0x080c2c14
0x80c2ba7: MOV EDX, dword ptr [EBP + 0xfffffc5c]
0x80c2bad: MOV dword ptr [EDI + 0x44], EDX
0x80c2bb0: LEA ESP, [EBP + -0xc]
0x80c2bb3: POP EBX
0x80c2bb4: POP ESI
0x80c2bb5: POP EDI
0x80c2bb6: LEAVE
0x80c2bb7: RET
0x80c2bb8: MOV EDI, 0x82b36ab
0x80c2bbd: MOV ECX, 0xc
0x80c2bc2: CLD
0x80c2bc3: MOV ESI, EBX
0x80c2bc5: CMPSB.REPE ES:EDI, ESI
0x80c2bc7: JNZ 0x080c2bd8
0x80c2bc9: MOV dword ptr [EBP + 0xfffffc5c], 0x1
0x80c2bd3: JMP 0x080c2a70
0x80c2bd8: MOV EDI, 0x82b36b7
0x80c2bdd: MOV ECX, 0xb
0x80c2be2: CLD
0x80c2be3: MOV ESI, EBX
0x80c2be5: CMPSB.REPE ES:EDI, ESI
0x80c2be7: JNZ 0x080c2ca3
0x80c2bed: MOV dword ptr [EBP + 0xfffffc5c], 0x0
0x80c2bf7: JMP 0x080c2a70
0x80c2bfc: PUSH 0x583
0x80c2c01: PUSH 0x82b2fc4
0x80c2c06: PUSH 0x82a376b
0x80c2c0b: PUSH EBX
0x80c2c0c: CALL 0x080ceb94
0x80c2c11: ADD ESP, 0x10
0x80c2c14: SUB ESP, 0xc
0x80c2c17: PUSH 0x82b33e8
0x80c2c1c: CALL 0x08097b24
0x80c2c21: MOV EDX, dword ptr [EBP + 0xfffffc5c]
0x80c2c27: MOV dword ptr [EDI + 0x44], EDX
0x80c2c2a: ADD ESP, 0x10
0x80c2c2d: LEA ESP, [EBP + -0xc]
0x80c2c30: POP EBX
0x80c2c31: POP ESI
0x80c2c32: POP EDI
0x80c2c33: LEAVE
0x80c2c34: RET
0x80c2c35: SUB ESP, 0xc
0x80c2c38: PUSH 0x82b2ff8
0x80c2c3d: CALL 0x08097b24
0x80c2c42: ADD ESP, 0x10
0x80c2c45: JMP 0x080c29b8
0x80c2c4a: SUB ESP, 0xc
0x80c2c4d: PUSH 0x82b3420
0x80c2c52: CALL 0x08097b24
0x80c2c57: ADD ESP, 0x10
0x80c2c5a: JMP 0x080c2a17
0x80c2c5f: SUB ESP, 0xc
0x80c2c62: PUSH 0x82b36c2
0x80c2c67: CALL 0x08097b24
0x80c2c6c: ADD ESP, 0x10
0x80c2c6f: JMP 0x080c29e8
0x80c2c74: TEST ESI, ESI
0x80c2c76: JZ 0x080c2a99
0x80c2c7c: SUB ESP, 0x8
0x80c2c7f: PUSH dword ptr [EBP + 0xfffffc60]
0x80c2c85: PUSH 0x82b3448
0x80c2c8a: CALL 0x08097b24
0x80c2c8f: ADD ESP, 0x10
0x80c2c92: JMP 0x080c2a99
0x80c2c97: MOV dword ptr [EBX + 0x4c], 0x0
0x80c2c9e: JMP 0x080c2b93
0x80c2ca3: SUB ESP, 0x8
0x80c2ca6: PUSH EBX
0x80c2ca7: PUSH 0x82b3470
0x80c2cac: CALL 0x08097b24
0x80c2cb1: ADD ESP, 0x10
0x80c2cb4: JMP 0x080c2a70
0x80c2cb9: SUB ESP, 0xc
0x80c2cbc: PUSH 0x82b34c4
0x80c2cc1: CALL 0x08097b24
0x80c2cc6: MOV EAX, dword ptr [EBP + 0xfffffc64]
0x80c2ccc: ADD ESP, 0x10
0x80c2ccf: MOV dword ptr [EBP + 0xfffffc50], EAX
0x80c2cd5: JMP 0x080c2ae3
0x80c2cda: SUB ESP, 0xc
0x80c2cdd: PUSH 0x82b350c
0x80c2ce2: CALL 0x08097b24
0x80c2ce7: MOV EDX, dword ptr [EBP + 0xfffffc64]
0x80c2ced: ADD ESP, 0x10
0x80c2cf0: MOV dword ptr [EBP + 0xfffffc50], EDX
0x80c2cf6: JMP 0x080c2acc
0x80c2cfb: SUB ESP, 0xc
0x80c2cfe: PUSH 0x82b3530
0x80c2d03: CALL 0x08097b24
0x80c2d08: ADD ESP, 0x10
0x80c2d0b: JMP 0x080c29d4
0x80c2d10: SUB ESP, 0xc
0x80c2d13: PUSH 0x82b3560
0x80c2d18: CALL 0x08097b24
0x80c2d1d: ADD ESP, 0x10
0x80c2d20: JMP 0x080c2a2f
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_pktAnalyse"></a>

## vm_pktAnalyse @ 0x80c1f18

Evidence: `ev_d5c9ac160ffdb1d78b521059303ebf16a54735bf20749764cdb4442cfc69b9d5`

```c

int vm_pktAnalyse(int cmdMask,int cookie,int msgNo,uchar *p,int mlen,int *pErrorOffset,
                 int autoInvoke)

{
  Vm_Pkt_Interface_T *pVVar1;
  uchar *puVar2;
  int iVar3;
  Vm_Catch_Buf *__env;
  int iVar4;
  anon_union_4_11_5000f750_for_v aVar5;
  Vm_String_T *pVVar6;
  Vm_Value_T *pVVar7;
  int iVar8;
  undefined4 uVar9;
  Vm_Buffer_T *pBuffer;
  int sp;
  int createdTempStack;
  uchar *pHead;
  Vm_Pkt_Def_T *pPktDef;
  Vm_Pkt_Interface_T *pPktInterface;
  Vm_Value_T value;
  
                    /* Unresolved local var: int ret@[DW_OP_reg7(EDI)] */
  puVar2 = p;
  if (_pktAnalyserInited == 0) {
    return -1;
  }
  if ((msgNo < 1) || (_maxInterfaces <= msgNo)) {
    return -2;
  }
  pVVar1 = _allPktInterfaces[msgNo];
  if (pVVar1 == (Vm_Pkt_Interface_T *)0x0) {
    return -3;
  }
  if ((pVVar1->cmdFilter & cmdMask) == 0) {
    return -4;
  }
  vm_initValue(&value);
  iVar3 = vm_runningInScheduler();
  if (iVar3 == 0) {
    vm_useTempStack();
  }
  sp = vm_sp;
  __env = vm_newCatch(-1,0);
  iVar4 = _setjmp((__jmp_buf_tag *)__env);
  iVar8 = -5;
  if (iVar4 != 0) goto LAB_080c1fca;
  iVar8 = vm_matchPktArguments(&value,&pVVar1->pktDef,&p,&mlen);
  if ((-1 < iVar8) && (mlen < 0)) {
    iVar8 = -6;
  }
  if (iVar8 != 1) {
    vm_endCatch();
    goto LAB_080c1fca;
  }
  if ((pVVar1->pktDef).encapType == 1) {
    if ((pVVar1->attrib & 1) != 0) {
                    /* Unresolved local var: Vm_Array_T * pArray@[???] */
      aVar5.info = vm_extendArray(value.v.info,1);
      value.v.string = aVar5.string;
      pVVar6 = (Vm_String_T *)vm_allocateBufferWithData(p,mlen);
      pVVar7 = vm_newValue();
      *(Vm_Value_T **)((int)(aVar5.string)->ptr + (aVar5.string)->attrib * 4) = pVVar7;
      (pVVar7->v).string = pVVar6;
      (*(Vm_Value_T **)((int)(aVar5.string)->ptr + (aVar5.string)->attrib * 4))->type = '\x05';
      (aVar5.pLoopCtrl)->pValue = (Vm_Value *)((int)&(aVar5.pLoopCtrl)->pValue->ref + 1);
    }
    if (autoInvoke == 0) goto LAB_080c20d4;
    uVar9 = 0x3a8;
  }
  else {
    if ((pVVar1->attrib & 1) != 0) {
      vm_addMappingBuffer((Vm_Mapping_T *)value.v.string,"extra_data",p,mlen);
    }
    if (autoInvoke == 0) {
LAB_080c20d4:
      vm_pushValue(&value);
      sp = vm_sp;
      vm_endCatch();
      goto LAB_080c1fca;
    }
    uVar9 = 0x1c5;
  }
  vm_applyFun(0,0,(int *)0x0,0,pVVar1->pFunction,2,0x58,cookie,uVar9,value.v.info);
  value.type = '\0';
  value.attrib = '\0';
  vm_endCatch();
LAB_080c1fca:
  vm_popNStack(sp - vm_sp);
  if (iVar3 == 0) {
    vm_freeTempStack();
  }
  vm_destructStaticValue(&value);
  if ((iVar8 != 1) && (pErrorOffset != (int *)0x0)) {
    *pErrorOffset = (int)p - (int)puVar2;
  }
  return iVar8;
}


```

<details><summary>汇编</summary>

```asm
0x80c1f18: PUSH EBP
0x80c1f19: MOV EBP, ESP
0x80c1f1b: PUSH EDI
0x80c1f1c: PUSH ESI
0x80c1f1d: PUSH EBX
0x80c1f1e: SUB ESP, 0x4c
0x80c1f21: MOV EBX, dword ptr [0x0860b228]
0x80c1f27: MOV EAX, dword ptr [EBP + 0x14]
0x80c1f2a: TEST EBX, EBX
0x80c1f2c: MOV ECX, dword ptr [EBP + 0x10]
0x80c1f2f: MOV dword ptr [EBP + -0x44], EAX
0x80c1f32: MOV EDX, 0xffffffff
0x80c1f37: JZ 0x080c201d
0x80c1f3d: TEST ECX, ECX
0x80c1f3f: JLE 0x080c2018
0x80c1f45: CMP ECX, dword ptr [0x083cfc44]
0x80c1f4b: JGE 0x080c2018
0x80c1f51: MOV EAX, [0x0860b224]
0x80c1f56: MOV ECX, dword ptr [EAX + ECX*0x4]
0x80c1f59: TEST ECX, ECX
0x80c1f5b: MOV dword ptr [EBP + -0x3c], ECX
0x80c1f5e: MOV EDX, 0xfffffffd
0x80c1f63: JZ 0x080c201d
0x80c1f69: MOV EAX, dword ptr [EBP + 0x8]
0x80c1f6c: TEST dword ptr [ECX + 0x4], EAX
0x80c1f6f: MOV DL, 0xfc
0x80c1f71: JZ 0x080c201d
0x80c1f77: SUB ESP, 0xc
0x80c1f7a: ADD ECX, 0x10
0x80c1f7d: LEA EAX, [EBP + -0x38]
0x80c1f80: MOV dword ptr [EBP + -0x40], ECX
0x80c1f83: PUSH EAX
0x80c1f84: CALL 0x08089f10
0x80c1f89: CALL 0x080a8f94
0x80c1f8e: ADD ESP, 0x10
0x80c1f91: TEST EAX, EAX
0x80c1f93: MOV dword ptr [EBP + -0x48], 0x0
0x80c1f9a: JZ 0x080c2070
0x80c1fa0: MOV EDX, dword ptr [0x086619cc]
0x80c1fa6: SUB ESP, 0x18
0x80c1fa9: MOV dword ptr [EBP + -0x4c], EDX
0x80c1fac: PUSH 0x0
0x80c1fae: PUSH -0x1
0x80c1fb0: CALL 0x080aae68
0x80c1fb5: ADD ESP, 0x14
0x80c1fb8: PUSH EAX
0x80c1fb9: CALL 0x0804bac4
0x80c1fbe: ADD ESP, 0x10
0x80c1fc1: TEST EAX, EAX
0x80c1fc3: MOV EDI, 0xfffffffb
0x80c1fc8: JZ 0x080c2030
0x80c1fca: MOV EAX, dword ptr [EBP + -0x4c]
0x80c1fcd: SUB ESP, 0xc
0x80c1fd0: SUB EAX, dword ptr [0x086619cc]
0x80c1fd6: PUSH EAX
0x80c1fd7: CALL 0x08091ab0
0x80c1fdc: MOV EAX, dword ptr [EBP + -0x48]
0x80c1fdf: ADD ESP, 0x10
0x80c1fe2: TEST EAX, EAX
0x80c1fe4: JNZ 0x080c2028
0x80c1fe6: SUB ESP, 0xc
0x80c1fe9: LEA EAX, [EBP + -0x38]
0x80c1fec: PUSH EAX
0x80c1fed: CALL 0x0808c0b8
0x80c1ff2: ADD ESP, 0x10
0x80c1ff5: CMP EDI, 0x1
0x80c1ff8: JZ 0x080c200c
0x80c1ffa: MOV EAX, dword ptr [EBP + 0x1c]
0x80c1ffd: TEST EAX, EAX
0x80c1fff: JZ 0x080c200c
0x80c2001: MOV EAX, dword ptr [EBP + 0x14]
0x80c2004: SUB EAX, dword ptr [EBP + -0x44]
0x80c2007: MOV EDX, dword ptr [EBP + 0x1c]
0x80c200a: MOV dword ptr [EDX], EAX
0x80c200c: LEA ESP, [EBP + -0xc]
0x80c200f: POP EBX
0x80c2010: MOV EDX, EDI
0x80c2012: POP ESI
0x80c2013: MOV EAX, EDX
0x80c2015: POP EDI
0x80c2016: LEAVE
0x80c2017: RET
0x80c2018: MOV EDX, 0xfffffffe
0x80c201d: LEA ESP, [EBP + -0xc]
0x80c2020: POP EBX
0x80c2021: POP ESI
0x80c2022: MOV EAX, EDX
0x80c2024: POP EDI
0x80c2025: LEAVE
0x80c2026: RET
0x80c2028: CALL 0x080c3878
0x80c202d: JMP 0x080c1fe6
0x80c2030: SUB ESP, 0x8
0x80c2033: LEA EAX, [EBP + 0x18]
0x80c2036: PUSH EAX
0x80c2037: LEA EAX, [EBP + 0x14]
0x80c203a: PUSH EAX
0x80c203b: LEA ESI, [EBP + -0x38]
0x80c203e: MOV EDX, dword ptr [EBP + -0x40]
0x80c2041: MOV EAX, ESI
0x80c2043: CALL 0x080c1698
0x80c2048: ADD ESP, 0x10
0x80c204b: TEST EAX, EAX
0x80c204d: MOV EDI, EAX
0x80c204f: JS 0x080c205e
0x80c2051: MOV ECX, dword ptr [EBP + 0x18]
0x80c2054: TEST ECX, ECX
0x80c2056: MOV EAX, 0xfffffffa
0x80c205b: CMOVS EDI, EAX
0x80c205e: CMP EDI, 0x1
0x80c2061: JZ 0x080c2081
0x80c2063: CALL 0x080aae54
0x80c2068: JMP 0x080c1fca
0x80c2070: CALL 0x080c37fc
0x80c2075: MOV dword ptr [EBP + -0x48], 0x1
0x80c207c: JMP 0x080c1fa0
0x80c2081: MOV ECX, dword ptr [EBP + -0x40]
0x80c2084: CMP dword ptr [ECX + 0x44], 0x1
0x80c2088: JZ 0x080c20f2
0x80c208a: MOV EDX, dword ptr [EBP + -0x3c]
0x80c208d: TEST byte ptr [EDX + 0x8], 0x1
0x80c2091: JNZ 0x080c210f
0x80c2093: MOV EAX, dword ptr [EBP + 0x20]
0x80c2096: TEST EAX, EAX
0x80c2098: JZ 0x080c20d4
0x80c209a: SUB ESP, 0x8
0x80c209d: PUSH dword ptr [EBP + -0x30]
0x80c20a0: PUSH 0x1c5
0x80c20a5: PUSH dword ptr [EBP + 0xc]
0x80c20a8: PUSH 0x58
0x80c20aa: PUSH 0x2
0x80c20ac: MOV ECX, dword ptr [EBP + -0x3c]
0x80c20af: PUSH dword ptr [ECX + 0xc]
0x80c20b2: PUSH 0x0
0x80c20b4: PUSH 0x0
0x80c20b6: PUSH 0x0
0x80c20b8: PUSH 0x0
0x80c20ba: CALL 0x0809a09c
0x80c20bf: MOV byte ptr [EBP + -0x34], 0x0
0x80c20c3: MOV byte ptr [EBP + -0x33], 0x0
0x80c20c7: ADD ESP, 0x30
0x80c20ca: CALL 0x080aae54
0x80c20cf: JMP 0x080c1fca
0x80c20d4: SUB ESP, 0xc
0x80c20d7: PUSH ESI
0x80c20d8: CALL 0x08091074
0x80c20dd: MOV EAX, [0x086619cc]
0x80c20e2: MOV dword ptr [EBP + -0x4c], EAX
0x80c20e5: ADD ESP, 0x10
0x80c20e8: CALL 0x080aae54
0x80c20ed: JMP 0x080c1fca
0x80c20f2: MOV EAX, dword ptr [EBP + -0x3c]
0x80c20f5: TEST byte ptr [EAX + 0x8], 0x1
0x80c20f9: JNZ 0x080c212a
0x80c20fb: MOV EAX, dword ptr [EBP + 0x20]
0x80c20fe: TEST EAX, EAX
0x80c2100: JZ 0x080c20d4
0x80c2102: SUB ESP, 0x8
0x80c2105: PUSH dword ptr [EBP + -0x30]
0x80c2108: PUSH 0x3a8
0x80c210d: JMP 0x080c20a5
0x80c210f: PUSH dword ptr [EBP + 0x18]
0x80c2112: PUSH dword ptr [EBP + 0x14]
0x80c2115: PUSH 0x82b3632
0x80c211a: PUSH dword ptr [EBP + -0x30]
0x80c211d: CALL 0x0809ef58
0x80c2122: ADD ESP, 0x10
0x80c2125: JMP 0x080c2093
0x80c212a: SUB ESP, 0x8
0x80c212d: PUSH 0x1
0x80c212f: PUSH dword ptr [EBP + -0x30]
0x80c2132: CALL 0x0809b564
0x80c2137: MOV dword ptr [EBP + -0x30], EAX
0x80c213a: MOV EBX, EAX
0x80c213c: POP EAX
0x80c213d: POP EDX
0x80c213e: PUSH dword ptr [EBP + 0x18]
0x80c2141: PUSH dword ptr [EBP + 0x14]
0x80c2144: CALL 0x0809abc0
0x80c2149: MOV dword ptr [EBP + -0x50], EAX
0x80c214c: CALL 0x08089ff0
0x80c2151: MOV ECX, dword ptr [EBX + 0x4]
0x80c2154: MOV EDX, dword ptr [EBX + 0xc]
0x80c2157: MOV dword ptr [EDX + ECX*0x4], EAX
0x80c215a: MOV EDX, dword ptr [EBP + -0x50]
0x80c215d: MOV dword ptr [EAX + 0x8], EDX
0x80c2160: MOV EDX, dword ptr [EBX + 0x4]
0x80c2163: MOV EAX, dword ptr [EBX + 0xc]
0x80c2166: MOV EAX, dword ptr [EAX + EDX*0x4]
0x80c2169: MOV byte ptr [EAX + 0x4], 0x5
0x80c216d: INC dword ptr [EBX + 0x4]
0x80c2170: ADD ESP, 0x10
0x80c2173: JMP 0x080c20fb
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_registerEfunModule"></a>

## vm_registerEfunModule @ 0x80874d4

Evidence: `ev_de693e0a0766b6be9f501e4cb1675f4e9e71e8ce47e771b9283ab45d321f56f1`

```c

int vm_registerEfunModule(Vm_Efun_Init_Routine_T pInit,Vm_Efun_Shutdown_Routine_T pShutdown)

{
  int iVar1;
  
                    /* Unresolved local var: int i@[DW_OP_reg2(EDX)] */
  iVar1 = 0;
  if (totalEfunModules < 0xc) {
    iVar1 = 0;
    if (0 < totalEfunModules) {
      do {
        if ((efunModule[iVar1].pInit == pInit) && (efunModule[iVar1].pShutdown == pShutdown))
        goto LAB_08087512;
        iVar1 = iVar1 + 1;
      } while (iVar1 < totalEfunModules);
    }
    efunModule[totalEfunModules].pInit = pInit;
    efunModule[totalEfunModules].pShutdown = pShutdown;
    totalEfunModules = totalEfunModules + 1;
LAB_08087512:
    iVar1 = 1;
  }
  return iVar1;
}


```

<details><summary>汇编</summary>

```asm
0x80874d4: PUSH EBP
0x80874d5: MOV EAX, [0x0847ace4]
0x80874da: MOV EBP, ESP
0x80874dc: XOR EDX, EDX
0x80874de: CMP EAX, 0xb
0x80874e1: PUSH EBX
0x80874e2: MOV ECX, dword ptr [EBP + 0x8]
0x80874e5: MOV EBX, dword ptr [EBP + 0xc]
0x80874e8: JG 0x08087517
0x80874ea: XOR EDX, EDX
0x80874ec: CMP EDX, EAX
0x80874ee: JGE 0x080874fe
0x80874f0: CMP dword ptr [EDX*0x8 + 0x847f9c0], ECX
0x80874f7: JZ 0x0808751c
0x80874f9: INC EDX
0x80874fa: CMP EDX, EAX
0x80874fc: JL 0x080874f0
0x80874fe: MOV dword ptr [EAX*0x8 + 0x847f9c0], ECX
0x8087505: MOV dword ptr [EAX*0x8 + 0x847f9c4], EBX
0x808750c: INC EAX
0x808750d: MOV [0x0847ace4], EAX
0x8087512: MOV EDX, 0x1
0x8087517: MOV EAX, EDX
0x8087519: POP EBX
0x808751a: LEAVE
0x808751b: RET
0x808751c: CMP dword ptr [EDX*0x8 + 0x847f9c4], EBX
0x8087523: JZ 0x08087512
0x8087525: INC EDX
0x8087526: JMP 0x080874fa
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm_buildProgramHead"></a>

## vm_buildProgramHead @ 0x808651c

Evidence: `ev_f36adb51c8fec4a9515bf994510f45777fefbd2aa5eefb2f00fd7aa35b354610`

```c

/* WARNING: Unknown calling convention */

Vm_Program * vm_buildProgramHead(char *pProgramName)

{
  Vm_Program_T *pProgram;
  char *pcVar1;
  Vm_Ident_Hash_Elem **__s;
  Vm_Program_Consititution_T *pVVar2;
  Vm_Index_T *pVVar3;
  int iVar4;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ret@[DW_OP_reg0(EAX)] */
  pProgram = pdb_allocateMemory(100,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x158d);
  if (pProgram != (Vm_Program_T *)0x0) {
    memset(pProgram,0,100);
    pcVar1 = vm_addFileName(pProgramName,&pProgram->strName);
    if (pcVar1 == (char *)0x0) {
      vm_destructProgramHead(pProgram);
    }
    else {
      __s = pdb_allocateMemory(0x2000,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15a5);
      pProgram->ppHashTable = __s;
      if (__s != (Vm_Ident_Hash_Elem **)0x0) {
        memset(__s,0,0x2000);
        pProgram->ppHashHead = pProgram->ppHashTable + 0x400;
        pProgram->consititutionBufferSize = 4;
        pVVar2 = pdb_allocateMemory(pProgram->consititutionBufferSize * 0xc,"VM",
                                    "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15b2);
        pProgram->pConsititutions = pVVar2;
        pVVar3 = pdb_allocateMemory((int)pProgram->consititutionBufferSize << 1,"VM",
                                    "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15b4);
        pProgram->initOrderList = pVVar3;
        if ((pProgram->pConsititutions != (Vm_Program_Consititution_T *)0x0) &&
           (pVVar3 != (Vm_Index_T *)0x0)) {
          pProgram->maxConsititutionNo = 1;
          pVVar2 = pProgram->pConsititutions;
          for (iVar4 = 3; iVar4 != 0; iVar4 = iVar4 + -1) {
            pVVar2->pProgram = (Vm_Program *)0x0;
            pVVar2 = (Vm_Program_Consititution_T *)&pVVar2->attrib;
          }
          pProgram->pConsititutions->pProgram = pProgram;
          return pProgram;
        }
        if (pProgram->ppHashTable != (Vm_Ident_Hash_Elem **)0x0) {
          pdb_freeMemory(pProgram->ppHashTable,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15cc
                        );
        }
      }
      pdb_freeMemory(pProgram,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15d1);
    }
  }
  return (Vm_Program *)0x0;
}


```

<details><summary>汇编</summary>

```asm
0x808651c: PUSH EBP
0x808651d: MOV EBP, ESP
0x808651f: PUSH EDI
0x8086520: PUSH EBX
0x8086521: SUB ESP, 0x10
0x8086524: PUSH 0x158d
0x8086529: PUSH 0x82ab14c
0x808652e: PUSH 0x82a376b
0x8086533: PUSH 0x64
0x8086535: CALL 0x080ce784
0x808653a: MOV EBX, EAX
0x808653c: ADD ESP, 0x10
0x808653f: XOR EAX, EAX
0x8086541: TEST EBX, EBX
0x8086543: JZ 0x080865a8
0x8086545: PUSH EAX
0x8086546: PUSH 0x64
0x8086548: PUSH 0x0
0x808654a: PUSH EBX
0x808654b: CALL 0x0804ba54
0x8086550: POP ECX
0x8086551: POP EDI
0x8086552: LEA EAX, [EBX + 0x4]
0x8086555: PUSH EAX
0x8086556: PUSH dword ptr [EBP + 0x8]
0x8086559: CALL 0x0807f5b4
0x808655e: ADD ESP, 0x10
0x8086561: TEST EAX, EAX
0x8086563: JZ 0x08086666
0x8086569: PUSH 0x15a5
0x808656e: PUSH 0x82ab14c
0x8086573: PUSH 0x82a376b
0x8086578: PUSH 0x2000
0x808657d: CALL 0x080ce784
0x8086582: ADD ESP, 0x10
0x8086585: TEST EAX, EAX
0x8086587: MOV dword ptr [EBX + 0x54], EAX
0x808658a: JNZ 0x080865b0
0x808658c: PUSH 0x15d1
0x8086591: PUSH 0x82ab14c
0x8086596: PUSH 0x82a376b
0x808659b: PUSH EBX
0x808659c: CALL 0x080ceb94
0x80865a1: XOR EBX, EBX
0x80865a3: ADD ESP, 0x10
0x80865a6: MOV EAX, EBX
0x80865a8: LEA ESP, [EBP + -0x8]
0x80865ab: POP EBX
0x80865ac: POP EDI
0x80865ad: LEAVE
0x80865ae: RET
0x80865b0: PUSH EDX
0x80865b1: PUSH 0x2000
0x80865b6: PUSH 0x0
0x80865b8: PUSH EAX
0x80865b9: CALL 0x0804ba54
0x80865be: MOV EAX, dword ptr [EBX + 0x54]
0x80865c1: ADD EAX, 0x1000
0x80865c6: MOV dword ptr [EBX + 0x58], EAX
0x80865c9: MOV word ptr [EBX + 0x12], 0x4
0x80865cf: PUSH 0x15b2
0x80865d4: PUSH 0x82ab14c
0x80865d9: PUSH 0x82a376b
0x80865de: MOVSX EAX, word ptr [EBX + 0x12]
0x80865e2: LEA EAX, [EAX + EAX*0x2]
0x80865e5: SHL EAX, 0x2
0x80865e8: PUSH EAX
0x80865e9: CALL 0x080ce784
0x80865ee: ADD ESP, 0x20
0x80865f1: MOV dword ptr [EBX + 0x18], EAX
0x80865f4: PUSH 0x15b4
0x80865f9: PUSH 0x82ab14c
0x80865fe: PUSH 0x82a376b
0x8086603: MOVSX EAX, word ptr [EBX + 0x12]
0x8086607: SHL EAX, 0x1
0x8086609: PUSH EAX
0x808660a: CALL 0x080ce784
0x808660f: MOV EDI, dword ptr [EBX + 0x18]
0x8086612: ADD ESP, 0x10
0x8086615: TEST EDI, EDI
0x8086617: MOV dword ptr [EBX + 0x1c], EAX
0x808661a: JZ 0x0808663c
0x808661c: TEST EAX, EAX
0x808661e: JZ 0x0808663c
0x8086620: XOR EAX, EAX
0x8086622: CLD
0x8086623: MOV ECX, 0x3
0x8086628: MOV word ptr [EBX + 0x14], 0x1
0x808662e: STOSD.REP ES:EDI
0x8086630: MOV EAX, dword ptr [EBX + 0x18]
0x8086633: MOV dword ptr [EAX], EBX
0x8086635: JMP 0x080865a6
0x808663c: MOV EDI, dword ptr [EBX + 0x54]
0x808663f: TEST EDI, EDI
0x8086641: JZ 0x0808658c
0x8086647: PUSH 0x15cc
0x808664c: PUSH 0x82ab14c
0x8086651: PUSH 0x82a376b
0x8086656: PUSH dword ptr [EBX + 0x54]
0x8086659: CALL 0x080ceb94
0x808665e: ADD ESP, 0x10
0x8086661: JMP 0x0808658c
0x8086666: SUB ESP, 0xc
0x8086669: PUSH EBX
0x808666a: CALL 0x080862a8
0x808666f: XOR EAX, EAX
0x8086671: JMP 0x080865a8
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.
