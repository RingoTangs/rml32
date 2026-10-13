# Numeric Ghidra evidence

Static REA MCP / Ghidra 12.1.4; canonical records: [rea.json.gz](rea.json.gz).

## _vm_ftol / procedure_assembly

Evidence: `ev_88d9c6fb8cd9b4aae9e9dc4111b5a73b49b6386c133cac64b776a807a4e43f23`

```asm
0x80a36e4: PUSH EBP
0x80a36e5: MOV EBP, ESP
0x80a36e7: CVTTSS2SI EAX, dword ptr [EBP + 0x8]
0x80a36ec: LEAVE
0x80a36ed: RET
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## _vm_ftol / procedure_pseudo_code

Evidence: `ev_b38e85b15e5070aab25a097fc476ce2d7f1b68c914db56179e8a6ffdcc35c277`

```c

/* WARNING: Unknown calling convention */

int _vm_ftol(float x)

{
  return (int)x;
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

## vm_andValue / procedure_pseudo_code

Evidence: `ev_25e13022d4998eb68c1a5c4394036ca4471f6551361cd12974da4c88b9aa1c01`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_andValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  Vm_String_T *pVVar5;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Array_T * pNewArray@[DW_OP_reg3(EBX)] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 != 2) {
    if (bVar1 == 6) {
      if (vm_stack[vm_sp].type != '\x06') {
        vm_error("Bad |, right is\'t an array when left is.\n");
      }
      pVVar5 = (Vm_String_T *)
               vm_andArray((Vm_Array_T *)pVVar2[iVar3 + 1].v.string,
                           (Vm_Array_T *)pVVar2[iVar3].v.string);
      vm_unrefArray((Vm_Array_T *)pVVar2[iVar3 + 1].v.string);
      pVVar2[iVar3 + 1].v.string = pVVar5;
    }
    else {
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad &, type invalid (got %s) of left value.\n",pcVar4);
    }
    vm_popStack();
    return;
  }
  if (vm_stack[vm_sp].type != '\x02') {
    vm_error("Bad &, right isn\'t a number when left is.\n");
  }
  pVVar2[iVar3 + 1].v.number = pVVar2[iVar3 + 1].v.number & pVVar2[iVar3].v.number;
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
- Ghidra Listing reports a terminal call at 0x808ea5b to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808ea82 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_castType / procedure_assembly

Evidence: `ev_c815f914c67806419b150dbe072aeaa049045d50a62b9807321f0cee489db79e`

```asm
0x809006c: PUSH EBP
0x809006d: MOV EBP, ESP
0x809006f: PUSH EDI
0x8090070: PUSH ESI
0x8090071: PUSH EBX
0x8090072: SUB ESP, 0xc
0x8090075: MOV ECX, dword ptr [EBP + 0x8]
0x8090078: MOV EAX, [0x086619cc]
0x809007d: LEA EAX, [EAX + EAX*0x2]
0x8090080: MOV EDX, dword ptr [0x08661964]
0x8090086: CMP ECX, 0x7
0x8090089: LEA ESI, [EDX + EAX*0x8]
0x809008c: JA 0x08090098
0x809008e: JMP dword ptr [ECX*0x4 + 0x82adc9c]
0x8090098: SUB ESP, 0xc
0x809009b: PUSH 0x82ad03c
0x80900a0: CALL 0x08097b24
0x80900a5: ADD ESP, 0x10
0x80900a8: TEST byte ptr [EBP + 0xc], 0x2
0x80900ac: JZ 0x080900b2
0x80900ae: OR byte ptr [ESI + 0x5], 0x1
0x80900b2: LEA ESP, [EBP + -0xc]
0x80900b5: POP EBX
0x80900b6: POP ESI
0x80900b7: POP EDI
0x80900b8: LEAVE
0x80900b9: RET
0x80900ba: MOVZX EAX, byte ptr [ESI + 0x4]
0x80900be: CMP EAX, 0x1
0x80900c1: JLE 0x0809041c
0x80900c7: CMP EAX, 0x7
0x80900ca: JZ 0x080900a8
0x80900cc: SUB ESP, 0xc
0x80900cf: PUSH 0x82adb18
0x80900d4: CALL 0x08097b24
0x80900d9: JMP 0x080900a5
0x80900db: MOVZX EAX, byte ptr [ESI + 0x4]
0x80900df: CMP EAX, 0x5
0x80900e2: JA 0x080903d8
0x80900e8: JMP dword ptr [EAX*0x4 + 0x82adcbc]
0x80900ef: MOVZX EAX, byte ptr [ESI + 0x4]
0x80900f3: CMP EAX, 0x4
0x80900f6: JA 0x080903ea
0x80900fc: JMP dword ptr [EAX*0x4 + 0x82adcd4]
0x8090103: MOVZX EAX, byte ptr [ESI + 0x4]
0x8090107: CMP EAX, 0xa
0x809010a: JA 0x0809014f
0x809010c: JMP dword ptr [EAX*0x4 + 0x82adce8]
0x8090113: MOVZX EAX, byte ptr [ESI + 0x4]
0x8090117: CMP EAX, 0x5
0x809011a: JA 0x08090266
0x8090120: JMP dword ptr [EAX*0x4 + 0x82add14]
0x8090127: MOVZX EAX, byte ptr [ESI + 0x4]
0x809012b: CMP EAX, 0x1
0x809012e: JLE 0x080903fc
0x8090134: CMP EAX, 0x6
0x8090137: JZ 0x080900a8
0x809013d: SUB ESP, 0xc
0x8090140: PUSH 0x82adb3c
0x8090145: CALL 0x08097b24
0x809014a: JMP 0x080900a5
0x809014f: SUB ESP, 0xc
0x8090152: PUSH 0x82adb60
0x8090157: CALL 0x08097b24
0x809015c: JMP 0x080900a5
0x8090161: MOV EAX, dword ptr [ESI + 0x8]
0x8090164: MOV EBX, dword ptr [EAX + 0xc]
0x8090167: JMP 0x0809016d
0x809016c: INC EBX
0x809016d: SUB ESP, 0xc
0x8090170: MOVZX EAX, byte ptr [EBX]
0x8090173: PUSH EAX
0x8090174: CALL 0x080bd79c
0x8090179: ADD ESP, 0x10
0x809017c: TEST EAX, EAX
0x809017e: JNZ 0x0809016c
0x8090180: PUSH EDI
0x8090181: PUSH 0x0
0x8090183: PUSH 0x0
0x8090185: PUSH EBX
0x8090186: CALL 0x0804aef4
0x809018b: FSTP float ptr [EBP + -0x10]
0x809018e: MOV EBX, dword ptr [EBP + -0x10]
0x8090191: MOV dword ptr [ESP], ESI
0x8090194: CALL 0x0808ad98
0x8090199: MOV byte ptr [ESI + 0x4], 0x3
0x809019d: MOV dword ptr [ESI + 0x8], EBX
0x80901a0: JMP 0x080900a5
0x80901a5: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x80901aa: MOV byte ptr [ESI + 0x4], 0x3
0x80901ae: MOVSS dword ptr [ESI + 0x8], XMM0
0x80901b3: JMP 0x080900a8
0x80901b8: MOV EAX, 0x0
0x80901bd: MOV byte ptr [ESI + 0x4], 0x3
0x80901c1: MOV dword ptr [ESI + 0x8], EAX
0x80901c4: JMP 0x080900a8
0x80901c9: SUB ESP, 0xc
0x80901cc: PUSH ESI
0x80901cd: CALL 0x0808bd88
0x80901d2: MOV dword ptr [ESP], EAX
0x80901d5: CALL 0x0808ab6c
0x80901da: MOV EBX, EAX
0x80901dc: MOV dword ptr [ESP], ESI
0x80901df: CALL 0x0808ad98
0x80901e4: MOV dword ptr [ESI + 0x8], EBX
0x80901e7: MOV byte ptr [ESI + 0x4], 0x4
0x80901eb: JMP 0x080900a5
0x80901f0: MOV EAX, dword ptr [ESI + 0x8]
0x80901f3: MOV EBX, dword ptr [EAX + 0xc]
0x80901f6: MOV EDX, dword ptr [EAX + 0x10]
0x80901f9: MOV dword ptr [EBP + -0x14], EAX
0x80901fc: MOV dword ptr [EBP + -0x18], EBX
0x80901ff: JMP 0x0809020c
0x8090201: MOV AL, byte ptr [EDX + EBX*0x1]
0x8090204: TEST AL, AL
0x8090206: JZ 0x0809043c
0x809020c: DEC EBX
0x809020d: CMP EBX, -0x1
0x8090210: JNZ 0x08090201
0x8090212: MOV EBX, dword ptr [EBP + -0x18]
0x8090215: CMP EBX, 0x2
0x8090218: JBE 0x08090223
0x809021a: CMP byte ptr [EDX], 0xef
0x809021d: JZ 0x080903b7
0x8090223: XOR EDI, EDI
0x8090225: SUB ESP, 0xc
0x8090228: LEA EAX, [EBX + 0x1]
0x809022b: PUSH EAX
0x809022c: CALL 0x0808ac84
0x8090231: MOV dword ptr [ESI + 0x8], EAX
0x8090234: ADD ESP, 0xc
0x8090237: MOV EDX, dword ptr [EBP + -0x14]
0x809023a: ADD EDI, dword ptr [EDX + 0x10]
0x809023d: PUSH EBX
0x809023e: PUSH EDI
0x809023f: PUSH dword ptr [EAX + 0xc]
0x8090242: CALL 0x0804b7e4
0x8090247: MOV EAX, dword ptr [ESI + 0x8]
0x809024a: MOV dword ptr [EAX + 0x10], EBX
0x809024d: MOV EAX, dword ptr [EAX + 0xc]
0x8090250: MOV byte ptr [EAX + EBX*0x1], 0x0
0x8090254: MOV byte ptr [ESI + 0x4], 0x4
0x8090258: POP ECX
0x8090259: PUSH dword ptr [EBP + -0x14]
0x809025c: CALL 0x0809a580
0x8090261: JMP 0x080900a5
0x8090266: SUB ESP, 0xc
0x8090269: PUSH 0x82adb84
0x809026e: CALL 0x08097b24
0x8090273: JMP 0x080900a5
0x8090278: SUB ESP, 0xc
0x809027b: PUSH 0x0
0x809027d: CALL 0x0809aa38
0x8090282: MOV dword ptr [EBP + -0x14], EAX
0x8090285: MOV dword ptr [ESP], ESI
0x8090288: CALL 0x0808ad98
0x809028d: MOV EAX, dword ptr [EBP + -0x14]
0x8090290: MOV dword ptr [ESI + 0x8], EAX
0x8090293: MOV byte ptr [ESI + 0x4], 0x5
0x8090297: JMP 0x080900a5
0x809029c: MOV EAX, dword ptr [ESI + 0x8]
0x809029f: SUB ESP, 0xc
0x80902a2: MOV EBX, dword ptr [EAX + 0x10]
0x80902a5: PUSH EBX
0x80902a6: CALL 0x0809aa38
0x80902ab: ADD ESP, 0x10
0x80902ae: TEST EBX, EBX
0x80902b0: MOV dword ptr [EBP + -0x14], EAX
0x80902b3: JNZ 0x08090459
0x80902b9: SUB ESP, 0xc
0x80902bc: PUSH ESI
0x80902bd: CALL 0x0808ad98
0x80902c2: MOV EDX, dword ptr [EBP + -0x14]
0x80902c5: MOV dword ptr [ESI + 0x8], EDX
0x80902c8: MOV byte ptr [ESI + 0x4], 0x5
0x80902cc: JMP 0x080900a5
0x80902d1: SUB ESP, 0xc
0x80902d4: PUSH 0x4
0x80902d6: CALL 0x0809aa38
0x80902db: MOV dword ptr [EBP + -0x14], EAX
0x80902de: MOV EBX, 0x4
0x80902e3: XOR EDI, EDI
0x80902e5: MOV EAX, dword ptr [EBP + -0x14]
0x80902e8: MOV ECX, EBX
0x80902ea: SUB ECX, EDI
0x80902ec: MOV EDX, dword ptr [EAX + 0x10]
0x80902ef: MOV EAX, dword ptr [ESI + 0x8]
0x80902f2: MOV byte ptr [EDX + ECX*0x1 + -0x1], AL
0x80902f6: MOV EAX, dword ptr [ESI + 0x8]
0x80902f9: INC EDI
0x80902fa: ADD ESP, 0x10
0x80902fd: SAR EAX, 0x8
0x8090300: CMP EDI, 0x4
0x8090303: JNC 0x08090325
0x8090305: MOV dword ptr [ESI + 0x8], EAX
0x8090308: MOV EAX, dword ptr [EBP + -0x14]
0x809030b: MOV ECX, EBX
0x809030d: SUB ECX, EDI
0x809030f: MOV EDX, dword ptr [EAX + 0x10]
0x8090312: MOV EAX, dword ptr [ESI + 0x8]
0x8090315: MOV byte ptr [EDX + ECX*0x1 + -0x1], AL
0x8090319: MOV EAX, dword ptr [ESI + 0x8]
0x809031c: INC EDI
0x809031d: SAR EAX, 0x8
0x8090320: CMP EDI, 0x4
0x8090323: JC 0x08090305
0x8090325: MOV EDX, dword ptr [EBP + -0x14]
0x8090328: MOV dword ptr [ESI + 0x8], EDX
0x809032b: MOV byte ptr [ESI + 0x4], 0x5
0x809032f: JMP 0x080900a8
0x8090334: MOV byte ptr [ESI + 0x4], 0x2
0x8090338: MOV dword ptr [ESI + 0x8], 0x0
0x809033f: JMP 0x080900a8
0x8090344: MOV EAX, dword ptr [ESI + 0x8]
0x8090347: MOV ECX, dword ptr [EAX + 0xc]
0x809034a: XOR EBX, EBX
0x809034c: XOR EDI, EDI
0x809034e: CMP ECX, 0x0
0x8090351: JBE 0x08090364
0x8090353: MOV EDX, dword ptr [EAX + 0x10]
0x8090356: SHL EBX, 0x8
0x8090359: MOVZX EAX, byte ptr [EDX + EDI*0x1]
0x809035d: INC EDI
0x809035e: OR EBX, EAX
0x8090360: CMP ECX, EDI
0x8090362: JA 0x08090356
0x8090364: SUB ESP, 0xc
0x8090367: PUSH ESI
0x8090368: CALL 0x0808ad98
0x809036d: MOV dword ptr [ESI + 0x8], EBX
0x8090370: MOV byte ptr [ESI + 0x4], 0x2
0x8090374: JMP 0x080900a5
0x8090379: MOV EAX, dword ptr [ESI + 0x8]
0x809037c: PUSH 0x0
0x809037e: PUSH 0xa
0x8090380: PUSH 0x0
0x8090382: PUSH dword ptr [EAX + 0xc]
0x8090385: CALL 0x0804b5a4
0x809038a: MOV EBX, EAX
0x809038c: MOV dword ptr [ESP], ESI
0x809038f: CALL 0x0808ad98
0x8090394: MOV byte ptr [ESI + 0x4], 0x2
0x8090398: MOV dword ptr [ESI + 0x8], EBX
0x809039b: JMP 0x080900a5
0x80903a0: SUB ESP, 0xc
0x80903a3: MOV byte ptr [ESI + 0x4], 0x2
0x80903a7: PUSH dword ptr [ESI + 0x8]
0x80903aa: CALL 0x080a36e4
0x80903af: MOV dword ptr [ESI + 0x8], EAX
0x80903b2: JMP 0x080900a5
0x80903b7: CMP byte ptr [EDX + 0x1], 0xbb
0x80903bb: JNZ 0x08090223
0x80903c1: CMP byte ptr [EDX + 0x2], 0xbf
0x80903c5: JNZ 0x08090223
0x80903cb: MOV EDI, 0x3
0x80903d0: SUB EBX, 0x3
0x80903d3: JMP 0x08090225
0x80903d8: SUB ESP, 0xc
0x80903db: PUSH 0x82adba8
0x80903e0: CALL 0x08097b24
0x80903e5: JMP 0x080900a5
0x80903ea: SUB ESP, 0xc
0x80903ed: PUSH 0x82adbcc
0x80903f2: CALL 0x08097b24
0x80903f7: JMP 0x080900a5
0x80903fc: SUB ESP, 0xc
0x80903ff: PUSH 0x0
0x8090401: CALL 0x0809adf4
0x8090406: MOV EBX, EAX
0x8090408: MOV dword ptr [ESP], ESI
0x809040b: CALL 0x0808ad98
0x8090410: MOV dword ptr [ESI + 0x8], EBX
0x8090413: MOV byte ptr [ESI + 0x4], 0x6
0x8090417: JMP 0x080900a5
0x809041c: SUB ESP, 0xc
0x809041f: PUSH 0x0
0x8090421: CALL 0x0809c308
0x8090426: MOV EBX, EAX
0x8090428: MOV dword ptr [ESP], ESI
0x809042b: CALL 0x0808ad98
0x8090430: MOV dword ptr [ESI + 0x8], EBX
0x8090433: MOV byte ptr [ESI + 0x4], 0x7
0x8090437: JMP 0x080900a5
0x809043c: CLD
0x809043d: MOV ECX, 0xffffffff
0x8090442: MOV EDI, EDX
0x8090444: SCASB.REPNE ES:EDI
0x8090446: NOT ECX
0x8090448: LEA EBX, [ECX + -0x1]
0x809044b: CMP EBX, -0x1
0x809044e: JNZ 0x08090215
0x8090454: JMP 0x08090212
0x8090459: MOV EAX, dword ptr [ESI + 0x8]
0x809045c: PUSH EDX
0x809045d: PUSH EBX
0x809045e: PUSH dword ptr [EAX + 0xc]
0x8090461: MOV EAX, dword ptr [EBP + -0x14]
0x8090464: PUSH dword ptr [EAX + 0x10]
0x8090467: CALL 0x0804b7e4
0x809046c: ADD ESP, 0x10
0x809046f: JMP 0x080902b9
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_castType / procedure_pseudo_code

Evidence: `ev_af0af51b13524e56b3c21ee5bfc96e632f628813884f539bb86e9cf91834a855`

```c

void vm_castType(int type,int attrib)

{
  Vm_Value_T *pValue;
  uchar uVar1;
  Vm_Buffer_T *pBuffer_00;
  uchar *puVar2;
  int *piVar3;
  uint uVar4;
  char *str;
  Vm_Value *pVVar5;
  void *pvVar6;
  Vm_String_T *pVVar7;
  Vm_Mapping_T *pVVar8;
  byte *pbVar9;
  int iVar10;
  Vm_Value *pVVar11;
  uchar *puVar12;
  longdouble lVar13;
  Vm_Buffer_T *pBuffer;
  
                    /* Unresolved local var: uchar * cp@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_String_T * pString@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mapping_T * pMapping@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Real_T r@[DW_OP_reg3(EBX)]
                       Unresolved local var: int val@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint len@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint i@[DW_OP_reg7(EDI)]
                       Unresolved local var: int shift@[DW_OP_reg7(EDI)] */
  pValue = vm_stack + vm_sp;
  switch(type) {
  default:
    vm_error("Can not cast to such type.\n");
    break;
  case 2:
    switch(pValue->type) {
    case '\0':
    case '\x01':
      pValue->type = '\x02';
      (pValue->v).info = (void *)0x0;
      break;
    case '\x02':
      break;
    case '\x03':
      pValue->type = '\x02';
      iVar10 = _vm_ftol((pValue->v).real);
      (pValue->v).number = iVar10;
      break;
    case '\x04':
      pvVar6 = (void *)__strtol_internal(((pValue->v).string)->ptr,0,10,0);
      vm_cleanupValue(pValue);
      pValue->type = '\x02';
      (pValue->v).info = pvVar6;
      break;
    case '\x05':
      pVVar5 = ((pValue->v).pPtrValue)->pNext;
      uVar4 = 0;
      pVVar11 = (Vm_Value *)0x0;
      if (pVVar5 != (Vm_Value *)0x0) {
        do {
          piVar3 = &pVVar11->ref;
          pVVar11 = (Vm_Value *)((int)&pVVar11->ref + 1);
          uVar4 = uVar4 << 8 |
                  (uint)*(byte *)((int)piVar3 + (int)&((pValue->v).pPtrValue)->pThis->ref);
        } while (pVVar11 < pVVar5);
      }
      vm_cleanupValue(pValue);
      (pValue->v).number = uVar4;
      pValue->type = '\x02';
      break;
    default:
      vm_error("Can not cast the type to integer.\n");
    }
    break;
  case 3:
    switch(pValue->type) {
    case '\0':
    case '\x01':
      pValue->type = '\x03';
      (pValue->v).info = (void *)0x0;
      break;
    case '\x02':
      pValue->type = '\x03';
      (pValue->v).real = (float)(pValue->v).number;
      break;
    case '\x03':
      break;
    case '\x04':
      pbVar9 = (byte *)((pValue->v).string)->ptr;
      while (iVar10 = vm_isspace((uint)*pbVar9), iVar10 != 0) {
        pbVar9 = pbVar9 + 1;
      }
      lVar13 = (longdouble)__strtod_internal(pbVar9,0,0);
      vm_cleanupValue(pValue);
      pValue->type = '\x03';
      (pValue->v).real = (float)lVar13;
      break;
    default:
      vm_error("Can not cast the type to real.\n");
    }
    break;
  case 4:
    switch(pValue->type) {
    case '\0':
    case '\x01':
    case '\x02':
    case '\x03':
    case '\n':
      str = vm_typeValue(pValue);
      pVVar7 = vm_copyString(str);
      vm_cleanupValue(pValue);
      (pValue->v).string = pVVar7;
      pValue->type = '\x04';
      break;
    case '\x04':
      break;
    case '\x05':
      pBuffer_00 = (pValue->v).buffer;
      puVar2 = pBuffer_00->ptr;
      uVar4 = pBuffer_00->realSize;
      do {
        uVar4 = uVar4 - 1;
        if (uVar4 == 0xffffffff) goto LAB_08090212;
      } while (puVar2[uVar4] != '\0');
      uVar4 = 0xffffffff;
      puVar12 = puVar2;
      do {
        if (uVar4 == 0) break;
        uVar4 = uVar4 - 1;
        uVar1 = *puVar12;
        puVar12 = puVar12 + 1;
      } while (uVar1 != '\0');
      uVar4 = ~uVar4 - 1;
      if (uVar4 == 0xffffffff) {
LAB_08090212:
        uVar4 = pBuffer_00->realSize;
      }
      if ((((uVar4 < 3) || (*puVar2 != 0xef)) || (puVar2[1] != 0xbb)) || (puVar2[2] != 0xbf)) {
        iVar10 = 0;
      }
      else {
        iVar10 = 3;
        uVar4 = uVar4 - 3;
      }
      pVVar7 = vm_newString(uVar4 + 1);
      (pValue->v).string = pVVar7;
      memcpy(pVVar7->ptr,pBuffer_00->ptr + iVar10,uVar4);
      pVVar7 = (pValue->v).string;
      pVVar7->len = uVar4;
      pVVar7->ptr[uVar4] = '\0';
      pValue->type = '\x04';
      vm_freeBuffer(pBuffer_00);
      break;
    default:
      vm_error("Can not cast the type to string.\n");
    }
    break;
  case 5:
    switch(pValue->type) {
    case '\0':
    case '\x01':
      pVVar7 = (Vm_String_T *)vm_allocateBuffer(0);
      vm_cleanupValue(pValue);
      (pValue->v).string = pVVar7;
      pValue->type = '\x05';
      break;
    case '\x02':
      pVVar5 = (Vm_Value *)vm_allocateBuffer(4);
      *(char *)((int)&pVVar5->pThis->ref + 3) = (char)(pValue->v).number;
      iVar10 = (pValue->v).number;
      uVar4 = 1;
      do {
        (pValue->v).number = iVar10 >> 8;
        *(char *)((int)pVVar5->pThis + (3 - uVar4)) = (char)(pValue->v).number;
        iVar10 = (pValue->v).number;
        uVar4 = uVar4 + 1;
      } while (uVar4 < 4);
      (pValue->v).pPtrValue = pVVar5;
      pValue->type = '\x05';
      break;
    default:
      vm_error("Can not cast the type to buffer.\n");
      break;
    case '\x04':
      uVar4 = ((pValue->v).string)->len;
      pVVar5 = (Vm_Value *)vm_allocateBuffer(uVar4);
      if (uVar4 != 0) {
        memcpy(pVVar5->pThis,((pValue->v).string)->ptr,uVar4);
      }
      vm_cleanupValue(pValue);
      (pValue->v).pPtrValue = pVVar5;
      pValue->type = '\x05';
      break;
    case '\x05':
      break;
    }
    break;
  case 6:
    uVar4 = (uint)pValue->type;
    if (uVar4 < 2) {
      pVVar7 = (Vm_String_T *)vm_allocateArray(uVar4);
      vm_cleanupValue(pValue);
      (pValue->v).string = pVVar7;
      pValue->type = '\x06';
    }
    else if (uVar4 != 6) {
      vm_error("Can not cast the type to array.\n");
    }
    break;
  case 7:
    if (pValue->type < 2) {
      pVVar8 = vm_allocateMapping(0);
      vm_cleanupValue(pValue);
      (pValue->v).string = (Vm_String_T *)pVVar8;
      pValue->type = '\a';
    }
    else if (pValue->type != 7) {
      vm_error("Can not cast the type to mapping.\n");
    }
  }
  if ((attrib & 2U) != 0) {
    pValue->attrib = pValue->attrib | 1;
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

## vm_geValue / procedure_assembly

Evidence: `ev_5d21fedab56340ea91181a5f3d19f6bf37be94cab81f615413f75caf22ffbe9c`

```asm
0x808ef88: PUSH EBP
0x808ef89: MOV EBP, ESP
0x808ef8b: PUSH ESI
0x808ef8c: PUSH EBX
0x808ef8d: SUB ESP, 0x10
0x808ef90: MOV EAX, [0x086619cc]
0x808ef95: LEA EAX, [EAX + EAX*0x2]
0x808ef98: MOV EDX, dword ptr [0x08661964]
0x808ef9e: LEA EBX, [EDX + EAX*0x8]
0x808efa1: LEA ESI, [EBX + 0x18]
0x808efa4: MOV AL, byte ptr [ESI + 0x4]
0x808efa7: MOV CL, byte ptr [EBX + 0x4]
0x808efaa: CMP AL, CL
0x808efac: JZ 0x0808f010
0x808efae: MOVZX EDX, AL
0x808efb1: CMP EDX, 0x3
0x808efb4: JZ 0x0808f048
0x808efba: JG 0x0808efec
0x808efbc: CMP EDX, 0x2
0x808efbf: JZ 0x0808f0a3
0x808efc5: SUB ESP, 0xc
0x808efc8: MOVZX EAX, AL
0x808efcb: PUSH EAX
0x808efcc: CALL 0x0808a13c
0x808efd1: POP EDX
0x808efd2: POP ECX
0x808efd3: PUSH EAX
0x808efd4: PUSH 0x82ad76c
0x808efd9: CALL 0x08097b24
0x808efde: ADD ESP, 0x10
0x808efe1: LEA ESP, [EBP + -0x8]
0x808efe4: POP EBX
0x808efe5: POP ESI
0x808efe6: LEAVE
0x808efe7: JMP 0x0809061c
0x808efec: CMP EDX, 0x4
0x808efef: JZ 0x0808f0bb
0x808eff5: CMP EDX, 0x5
0x808eff8: JNZ 0x0808efc5
0x808effa: PUSH EAX
0x808effb: PUSH 0x82acfbf
0x808f000: PUSH EBX
0x808f001: PUSH ESI
0x808f002: CALL 0x0808cda0
0x808f007: MOV EBX, EAX
0x808f009: MOV dword ptr [ESP], ESI
0x808f00c: NOT EBX
0x808f00e: JMP 0x0808f037
0x808f010: CMP AL, 0x2
0x808f012: JZ 0x0808f098
0x808f018: CMP AL, 0x4
0x808f01a: JNZ 0x0808efae
0x808f01c: MOV EAX, dword ptr [EBX + 0x8]
0x808f01f: SUB ESP, 0x8
0x808f022: MOV EDX, dword ptr [ESI + 0x8]
0x808f025: PUSH dword ptr [EAX + 0xc]
0x808f028: PUSH dword ptr [EDX + 0xc]
0x808f02b: CALL 0x0804b064
0x808f030: MOV EBX, EAX
0x808f032: NOT EBX
0x808f034: MOV dword ptr [ESP], ESI
0x808f037: CALL 0x0808ad98
0x808f03c: SHR EBX, 0x1f
0x808f03f: MOV byte ptr [ESI + 0x4], 0x2
0x808f043: MOV dword ptr [ESI + 0x8], EBX
0x808f046: JMP 0x0808efde
0x808f048: MOVZX EAX, CL
0x808f04b: CMP EAX, 0x2
0x808f04e: JZ 0x0808f067
0x808f050: CMP EAX, 0x3
0x808f053: JZ 0x0808f0d2
0x808f055: SUB ESP, 0xc
0x808f058: PUSH 0x82ad79c
0x808f05d: CALL 0x08097b24
0x808f062: JMP 0x0808efde
0x808f067: CVTSI2SS XMM0, dword ptr [EBX + 0x8]
0x808f06c: MOV byte ptr [ESI + 0x4], 0x2
0x808f070: FLD float ptr [ESI + 0x8]
0x808f073: MOVSS dword ptr [EBP + -0xc], XMM0
0x808f078: FLD float ptr [EBP + -0xc]
0x808f07b: FXCH
0x808f07d: FUCOMIP ST0, ST1
0x808f07f: FSTP ST0
0x808f081: SETNC AL
0x808f084: MOVZX EAX, AL
0x808f087: MOV dword ptr [ESI + 0x8], EAX
0x808f08a: LEA ESP, [EBP + -0x8]
0x808f08d: POP EBX
0x808f08e: POP ESI
0x808f08f: LEAVE
0x808f090: JMP 0x0809061c
0x808f098: MOV EAX, dword ptr [EBX + 0x8]
0x808f09b: CMP dword ptr [ESI + 0x8], EAX
0x808f09e: SETGE AL
0x808f0a1: JMP 0x0808f084
0x808f0a3: MOVZX EAX, CL
0x808f0a6: CMP EAX, 0x2
0x808f0a9: JZ 0x0808f098
0x808f0ab: CMP EAX, 0x3
0x808f0ae: JNZ 0x0808f055
0x808f0b0: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808f0b5: UCOMISS XMM0, dword ptr [EBX + 0x8]
0x808f0b9: JMP 0x0808f081
0x808f0bb: CMP CL, 0x4
0x808f0be: JZ 0x0808f0e4
0x808f0c0: SUB ESP, 0xc
0x808f0c3: PUSH 0x82ad7c8
0x808f0c8: CALL 0x08097b24
0x808f0cd: JMP 0x0808efde
0x808f0d2: FLD float ptr [ESI + 0x8]
0x808f0d5: FLD float ptr [EBX + 0x8]
0x808f0d8: FXCH
0x808f0da: FUCOMIP ST0, ST1
0x808f0dc: FSTP ST0
0x808f0de: MOV byte ptr [ESI + 0x4], 0x2
0x808f0e2: JMP 0x0808f081
0x808f0e4: MOV EAX, dword ptr [EBX + 0x8]
0x808f0e7: SUB ESP, 0x8
0x808f0ea: MOV EDX, dword ptr [ESI + 0x8]
0x808f0ed: PUSH dword ptr [EAX + 0xc]
0x808f0f0: PUSH dword ptr [EDX + 0xc]
0x808f0f3: CALL 0x0804b064
0x808f0f8: JMP 0x0808f030
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808efe7 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f090 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_gtValue / procedure_assembly

Evidence: `ev_fd438ebc3f1882653512f33fc644071325a9c3e299a02f546e0887e66f20e3f7`

```asm
0x808f340: PUSH EBP
0x808f341: MOV EBP, ESP
0x808f343: PUSH ESI
0x808f344: PUSH EBX
0x808f345: SUB ESP, 0x10
0x808f348: MOV EAX, [0x086619cc]
0x808f34d: LEA EAX, [EAX + EAX*0x2]
0x808f350: MOV EDX, dword ptr [0x08661964]
0x808f356: LEA EBX, [EDX + EAX*0x8]
0x808f359: LEA ESI, [EBX + 0x18]
0x808f35c: MOV AL, byte ptr [ESI + 0x4]
0x808f35f: MOV CL, byte ptr [EBX + 0x4]
0x808f362: CMP AL, CL
0x808f364: JZ 0x0808f3d8
0x808f366: MOVZX EDX, AL
0x808f369: CMP EDX, 0x3
0x808f36c: JZ 0x0808f410
0x808f372: JG 0x0808f3a4
0x808f374: CMP EDX, 0x2
0x808f377: JZ 0x0808f46b
0x808f37d: SUB ESP, 0xc
0x808f380: MOVZX EAX, AL
0x808f383: PUSH EAX
0x808f384: CALL 0x0808a13c
0x808f389: POP EDX
0x808f38a: POP ECX
0x808f38b: PUSH EAX
0x808f38c: PUSH 0x82ad87c
0x808f391: CALL 0x08097b24
0x808f396: ADD ESP, 0x10
0x808f399: LEA ESP, [EBP + -0x8]
0x808f39c: POP EBX
0x808f39d: POP ESI
0x808f39e: LEAVE
0x808f39f: JMP 0x0809061c
0x808f3a4: CMP EDX, 0x4
0x808f3a7: JZ 0x0808f483
0x808f3ad: CMP EDX, 0x5
0x808f3b0: JNZ 0x0808f37d
0x808f3b2: PUSH EAX
0x808f3b3: PUSH 0x82cf207
0x808f3b8: PUSH EBX
0x808f3b9: PUSH ESI
0x808f3ba: CALL 0x0808cda0
0x808f3bf: MOV EBX, EAX
0x808f3c1: MOV dword ptr [ESP], ESI
0x808f3c4: CALL 0x0808ad98
0x808f3c9: ADD ESP, 0x10
0x808f3cc: TEST EBX, EBX
0x808f3ce: SETG AL
0x808f3d1: MOV byte ptr [ESI + 0x4], 0x2
0x808f3d5: JMP 0x0808f44c
0x808f3d8: CMP AL, 0x2
0x808f3da: JZ 0x0808f460
0x808f3e0: CMP AL, 0x4
0x808f3e2: JNZ 0x0808f366
0x808f3e4: MOV EAX, dword ptr [EBX + 0x8]
0x808f3e7: SUB ESP, 0x8
0x808f3ea: MOV EDX, dword ptr [ESI + 0x8]
0x808f3ed: PUSH dword ptr [EAX + 0xc]
0x808f3f0: PUSH dword ptr [EDX + 0xc]
0x808f3f3: CALL 0x0804b064
0x808f3f8: XOR EBX, EBX
0x808f3fa: MOV dword ptr [ESP], ESI
0x808f3fd: TEST EAX, EAX
0x808f3ff: SETG BL
0x808f402: CALL 0x0808ad98
0x808f407: MOV byte ptr [ESI + 0x4], 0x2
0x808f40b: MOV dword ptr [ESI + 0x8], EBX
0x808f40e: JMP 0x0808f396
0x808f410: MOVZX EAX, CL
0x808f413: CMP EAX, 0x2
0x808f416: JZ 0x0808f42f
0x808f418: CMP EAX, 0x3
0x808f41b: JZ 0x0808f49a
0x808f41d: SUB ESP, 0xc
0x808f420: PUSH 0x82ad8ac
0x808f425: CALL 0x08097b24
0x808f42a: JMP 0x0808f396
0x808f42f: CVTSI2SS XMM0, dword ptr [EBX + 0x8]
0x808f434: MOV byte ptr [ESI + 0x4], 0x2
0x808f438: FLD float ptr [ESI + 0x8]
0x808f43b: MOVSS dword ptr [EBP + -0xc], XMM0
0x808f440: FLD float ptr [EBP + -0xc]
0x808f443: FXCH
0x808f445: FUCOMIP ST0, ST1
0x808f447: FSTP ST0
0x808f449: SETA AL
0x808f44c: MOVZX EAX, AL
0x808f44f: MOV dword ptr [ESI + 0x8], EAX
0x808f452: LEA ESP, [EBP + -0x8]
0x808f455: POP EBX
0x808f456: POP ESI
0x808f457: LEAVE
0x808f458: JMP 0x0809061c
0x808f460: MOV EAX, dword ptr [EBX + 0x8]
0x808f463: CMP dword ptr [ESI + 0x8], EAX
0x808f466: SETG AL
0x808f469: JMP 0x0808f44c
0x808f46b: MOVZX EAX, CL
0x808f46e: CMP EAX, 0x2
0x808f471: JZ 0x0808f460
0x808f473: CMP EAX, 0x3
0x808f476: JNZ 0x0808f41d
0x808f478: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808f47d: UCOMISS XMM0, dword ptr [EBX + 0x8]
0x808f481: JMP 0x0808f449
0x808f483: CMP CL, 0x4
0x808f486: JZ 0x0808f4ac
0x808f488: SUB ESP, 0xc
0x808f48b: PUSH 0x82ad8d8
0x808f490: CALL 0x08097b24
0x808f495: JMP 0x0808f396
0x808f49a: FLD float ptr [ESI + 0x8]
0x808f49d: FLD float ptr [EBX + 0x8]
0x808f4a0: FXCH
0x808f4a2: FUCOMIP ST0, ST1
0x808f4a4: FSTP ST0
0x808f4a6: MOV byte ptr [ESI + 0x4], 0x2
0x808f4aa: JMP 0x0808f449
0x808f4ac: MOV EAX, dword ptr [EBX + 0x8]
0x808f4af: SUB ESP, 0x8
0x808f4b2: MOV EDX, dword ptr [ESI + 0x8]
0x808f4b5: PUSH dword ptr [EAX + 0xc]
0x808f4b8: PUSH dword ptr [EDX + 0xc]
0x808f4bb: CALL 0x0804b064
0x808f4c0: JMP 0x0808f3f8
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808f39f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f458 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_isEq / procedure_assembly

Evidence: `ev_611f55b6a7d06bc3a5a571d378a8c62a505be9c5c4c5cc08385dda151702b480`

```asm
0x808c908: PUSH EBP
0x808c909: MOV EBP, ESP
0x808c90b: PUSH EDI
0x808c90c: PUSH ESI
0x808c90d: PUSH EBX
0x808c90e: SUB ESP, 0x14
0x808c911: MOV EDI, dword ptr [EBP + 0x8]
0x808c914: MOV ESI, dword ptr [EBP + 0xc]
0x808c917: MOV DL, byte ptr [EDI + 0x4]
0x808c91a: MOV CL, byte ptr [ESI + 0x4]
0x808c91d: CMP DL, CL
0x808c91f: JZ 0x0808c940
0x808c921: MOVZX EAX, DL
0x808c924: CMP EAX, 0xa
0x808c927: JBE 0x0808c938
0x808c929: XOR EDX, EDX
0x808c92b: ADD ESP, 0x14
0x808c92e: POP EBX
0x808c92f: POP ESI
0x808c930: MOV EAX, EDX
0x808c932: POP EDI
0x808c933: LEAVE
0x808c934: RET
0x808c938: JMP dword ptr [EAX*0x4 + 0x82acf18]
0x808c940: CMP DL, 0x2
0x808c943: JZ 0x0808ca80
0x808c949: CMP DL, 0x4
0x808c94c: JNZ 0x0808c921
0x808c94e: MOV ECX, dword ptr [ESI + 0x8]
0x808c951: MOV EDX, dword ptr [EDI + 0x8]
0x808c954: CMP EDX, ECX
0x808c956: MOV ESI, 0x1
0x808c95b: JZ 0x0808c96c
0x808c95d: MOV EBX, dword ptr [EDX + 0x10]
0x808c960: XOR SI, SI
0x808c963: CMP EBX, dword ptr [ECX + 0x10]
0x808c966: JZ 0x0808caa0
0x808c96c: MOV EDX, ESI
0x808c96e: JMP 0x0808c92b
0x808c970: CMP DL, CL
0x808c972: JNZ 0x0808c929
0x808c974: MOV EAX, dword ptr [ESI + 0x8]
0x808c977: CMP dword ptr [EDI + 0x8], EAX
0x808c97a: MOV EDX, 0x1
0x808c97f: JNZ 0x0808c929
0x808c981: JMP 0x0808c92b
0x808c984: CMP CL, 0x5
0x808c987: JNZ 0x0808c929
0x808c989: MOV EDX, dword ptr [EDI + 0x8]
0x808c98c: MOV ECX, dword ptr [ESI + 0x8]
0x808c98f: MOV EAX, dword ptr [EDX + 0xc]
0x808c992: CMP EAX, dword ptr [ECX + 0xc]
0x808c995: JNZ 0x0808c929
0x808c997: XOR EBX, EBX
0x808c999: TEST EAX, EAX
0x808c99b: JZ 0x0808c9ac
0x808c99d: MOV EDI, dword ptr [ECX + 0x10]
0x808c9a0: MOV ESI, dword ptr [EDX + 0x10]
0x808c9a3: CLD
0x808c9a4: CMP EAX, EAX
0x808c9a6: MOV ECX, EAX
0x808c9a8: CMPSB.REPE ES:EDI, ESI
0x808c9aa: JNZ 0x0808c9b1
0x808c9ac: MOV EBX, 0x1
0x808c9b1: MOV EDX, EBX
0x808c9b3: JMP 0x0808c92b
0x808c9b8: XOR EDX, EDX
0x808c9ba: CMP CL, 0x4
0x808c9bd: JNZ 0x0808c92b
0x808c9c3: MOV EDX, dword ptr [EDI + 0x8]
0x808c9c6: MOV ECX, dword ptr [ESI + 0x8]
0x808c9c9: CMP EDX, ECX
0x808c9cb: MOV EBX, 0x1
0x808c9d0: JZ 0x0808c9b1
0x808c9d2: MOV EAX, dword ptr [EDX + 0x10]
0x808c9d5: XOR BL, BL
0x808c9d7: CMP EAX, dword ptr [ECX + 0x10]
0x808c9da: MOV dword ptr [EBP + -0x18], EAX
0x808c9dd: JNZ 0x0808c9b1
0x808c9df: MOV EAX, dword ptr [ECX + 0x4]
0x808c9e2: AND EAX, dword ptr [EDX + 0x4]
0x808c9e5: TEST AL, 0x1
0x808c9e7: JNZ 0x0808c9b1
0x808c9e9: MOV EAX, dword ptr [EBP + -0x18]
0x808c9ec: MOV ESI, dword ptr [EDX + 0xc]
0x808c9ef: MOV EDI, dword ptr [ECX + 0xc]
0x808c9f2: CMP EAX, EAX
0x808c9f4: MOV ECX, EAX
0x808c9f6: CLD
0x808c9f7: CMPSB.REPE ES:EDI, ESI
0x808c9f9: SETA DL
0x808c9fc: SETC AL
0x808c9ff: XOR EBX, EBX
0x808ca01: CMP DL, AL
0x808ca03: SETZ BL
0x808ca06: MOV EDX, EBX
0x808ca08: JMP 0x0808c92b
0x808ca10: MOV byte ptr [EDI + 0x4], 0x2
0x808ca14: MOVZX EAX, byte ptr [ESI + 0x4]
0x808ca18: CMP EAX, 0x2
0x808ca1b: JZ 0x0808ca8e
0x808ca1d: JLE 0x0808cac8
0x808ca23: XOR EDX, EDX
0x808ca25: CMP EAX, 0x3
0x808ca28: JNZ 0x0808c92b
0x808ca2e: FLD float ptr [EDI + 0x8]
0x808ca31: FLD float ptr [ESI + 0x8]
0x808ca34: FXCH
0x808ca36: FUCOMIP ST0, ST1
0x808ca38: FSTP ST0
0x808ca3a: JMP 0x0808ca5c
0x808ca3c: MOVZX EAX, CL
0x808ca3f: CMP EAX, 0x2
0x808ca42: JZ 0x0808ca80
0x808ca44: JLE 0x0808c929
0x808ca4a: CMP EAX, 0x3
0x808ca4d: JNZ 0x0808c929
0x808ca53: CVTSI2SS XMM0, dword ptr [EDI + 0x8]
0x808ca58: UCOMISS XMM0, dword ptr [ESI + 0x8]
0x808ca5c: SETNP DL
0x808ca5f: SETZ AL
0x808ca62: AND EAX, EDX
0x808ca64: MOVZX EDX, AL
0x808ca67: NOP
0x808ca68: ADD ESP, 0x14
0x808ca6b: POP EBX
0x808ca6c: POP ESI
0x808ca6d: MOV EAX, EDX
0x808ca6f: POP EDI
0x808ca70: LEAVE
0x808ca71: RET
0x808ca72: CMP CL, 0x1
0x808ca75: SETBE AL
0x808ca78: MOVZX EDX, AL
0x808ca7b: JMP 0x0808ca68
0x808ca80: MOV EAX, dword ptr [ESI + 0x8]
0x808ca83: CMP dword ptr [EDI + 0x8], EAX
0x808ca86: SETZ AL
0x808ca89: MOVZX EDX, AL
0x808ca8c: JMP 0x0808ca68
0x808ca8e: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808ca93: FLD float ptr [EDI + 0x8]
0x808ca96: MOVSS dword ptr [EBP + -0x20], XMM0
0x808ca9b: FLD float ptr [EBP + -0x20]
0x808ca9e: JMP 0x0808ca34
0x808caa0: MOV EAX, dword ptr [ECX + 0x4]
0x808caa3: AND EAX, dword ptr [EDX + 0x4]
0x808caa6: TEST AL, 0x1
0x808caa8: JNZ 0x0808c96c
0x808caae: MOV ESI, dword ptr [EDX + 0xc]
0x808cab1: MOV EDI, dword ptr [ECX + 0xc]
0x808cab4: CLD
0x808cab5: CMP EBX, EBX
0x808cab7: MOV ECX, EBX
0x808cab9: CMPSB.REPE ES:EDI, ESI
0x808cabb: SETZ AL
0x808cabe: MOVZX ESI, AL
0x808cac1: MOV EDX, ESI
0x808cac3: JMP 0x0808c92b
0x808cac8: FLD float ptr [EDI + 0x8]
0x808cacb: FLDZ
0x808cacd: JMP 0x0808ca34
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_isType / procedure_assembly

Evidence: `ev_98e6c9ff45230fc2a6aae46da320d253cb18a2bfbe5df25e2305c4cd36bfb3aa`

```asm
0x808f100: PUSH EBP
0x808f101: MOV EBP, ESP
0x808f103: PUSH EDI
0x808f104: PUSH ESI
0x808f105: PUSH EBX
0x808f106: SUB ESP, 0xc
0x808f109: MOV EAX, [0x086619cc]
0x808f10e: LEA EAX, [EAX + EAX*0x2]
0x808f111: MOV EDX, dword ptr [0x08661964]
0x808f117: LEA EBX, [EDX + EAX*0x8]
0x808f11a: MOV DL, byte ptr [EBX + 0x4]
0x808f11d: MOVZX EAX, DL
0x808f120: CMP EAX, 0xa
0x808f123: MOV ECX, dword ptr [EBP + 0x8]
0x808f126: JA 0x0808f157
0x808f128: JMP dword ptr [EAX*0x4 + 0x82adc50]
0x808f130: MOV EAX, dword ptr [EBX + 0x8]
0x808f133: MOV EAX, dword ptr [EAX + 0xc]
0x808f136: TEST EAX, EAX
0x808f138: JZ 0x0808f140
0x808f13a: TEST byte ptr [EAX + 0xd], 0x80
0x808f13e: JNZ 0x0808f192
0x808f140: MOVZX EAX, DL
0x808f143: CMP EAX, ECX
0x808f145: SETZ AL
0x808f148: MOVZX ESI, AL
0x808f14b: SUB ESP, 0xc
0x808f14e: PUSH EBX
0x808f14f: CALL 0x0808ad98
0x808f154: ADD ESP, 0x10
0x808f157: MOV byte ptr [EBX + 0x4], 0x2
0x808f15b: MOV dword ptr [EBX + 0x8], ESI
0x808f15e: LEA ESP, [EBP + -0xc]
0x808f161: POP EBX
0x808f162: POP ESI
0x808f163: POP EDI
0x808f164: LEAVE
0x808f165: RET
0x808f168: MOVZX EAX, DL
0x808f16b: CMP EAX, ECX
0x808f16d: SETZ AL
0x808f170: MOVZX ESI, AL
0x808f173: MOV byte ptr [EBX + 0x4], 0x2
0x808f177: MOV dword ptr [EBX + 0x8], ESI
0x808f17a: LEA ESP, [EBP + -0xc]
0x808f17d: POP EBX
0x808f17e: POP ESI
0x808f17f: POP EDI
0x808f180: LEAVE
0x808f181: RET
0x808f184: MOV EAX, dword ptr [EBX + 0x8]
0x808f187: MOV EDI, dword ptr [EAX + 0xc]
0x808f18a: TEST EDI, 0x8000
0x808f190: JZ 0x0808f196
0x808f192: TEST ECX, ECX
0x808f194: JMP 0x0808f145
0x808f196: MOVZX EAX, DL
0x808f199: CMP EAX, ECX
0x808f19b: SETZ AL
0x808f19e: MOVZX ESI, AL
0x808f1a1: TEST ESI, ESI
0x808f1a3: JZ 0x0808f14b
0x808f1a5: TEST EDI, 0x2000
0x808f1ab: MOV EAX, 0xffffffff
0x808f1b0: CMOVNZ ESI, EAX
0x808f1b3: AND EDI, 0x4000
0x808f1b9: MOV AL, 0xfe
0x808f1bb: CMOVNZ ESI, EAX
0x808f1be: JMP 0x0808f14b
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_isType / procedure_pseudo_code

Evidence: `ev_068162c07346cd0a2247f951ac416df4d75050ed3918543ecabfc7965e4ac52a`

```c

void vm_isType(int type)

{
  Vm_Value_T *pValue;
  byte bVar1;
  Vm_Value *pVVar2;
  uint uVar3;
  anon_union_4_11_5000f750_for_v unaff_ESI;
  anon_union_4_11_5000f750_for_v aVar4;
  bool bVar5;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ret@[DW_OP_reg6(ESI)] */
  pValue = vm_stack + vm_sp;
  bVar1 = pValue->type;
  switch(bVar1) {
  case 0:
  case 1:
  case 2:
  case 3:
    aVar4.number._1_3_ = 0;
    aVar4.number._0_1_ = (uint)bVar1 == type;
    pValue->type = '\x02';
    pValue->v = aVar4;
    return;
  case 4:
  case 5:
  case 6:
  case 7:
  case 8:
switchD_0808f128_caseD_4:
    bVar5 = (uint)bVar1 == type;
    break;
  case 9:
    pVVar2 = ((pValue->v).pPtrValue)->pNext;
    if ((pVVar2 == (Vm_Value *)0x0) || (((uint)pVVar2->pNext & 0x8000) == 0))
    goto switchD_0808f128_caseD_4;
LAB_0808f192:
    bVar5 = type == 0;
    break;
  case 10:
    uVar3 = ((pValue->v).buffer)->realSize;
    if ((uVar3 & 0x8000) != 0) goto LAB_0808f192;
    unaff_ESI.number._1_3_ = 0;
    unaff_ESI.number._0_1_ = (uint)bVar1 == type;
    if (unaff_ESI.info != (void *)0x0) {
      if ((uVar3 & 0x2000) != 0) {
        unaff_ESI.number = -1;
      }
      if ((uVar3 & 0x4000) != 0) {
        unaff_ESI.number = -2;
      }
    }
    goto LAB_0808f14b;
  default:
    goto switchD_0808f128_default;
  }
  unaff_ESI.number._1_3_ = 0;
  unaff_ESI.number._0_1_ = bVar5;
LAB_0808f14b:
  vm_cleanupValue(pValue);
switchD_0808f128_default:
  pValue->type = '\x02';
  pValue->v = unaff_ESI;
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

## vm_isZero / procedure_assembly

Evidence: `ev_dd7355f9ff76a95428a89da8d67e30fb22df7e858529878a656f90b9d6aa884f`

```asm
0x808cad4: PUSH EBP
0x808cad5: MOV EBP, ESP
0x808cad7: SUB ESP, 0x8
0x808cada: MOV ECX, dword ptr [EBP + 0x8]
0x808cadd: MOVZX EAX, byte ptr [ECX + 0x4]
0x808cae1: CMP EAX, 0xa
0x808cae4: JA 0x0808caf0
0x808cae6: JMP dword ptr [EAX*0x4 + 0x82acf44]
0x808caf0: XOR EDX, EDX
0x808caf2: MOV EAX, EDX
0x808caf4: LEAVE
0x808caf5: RET
0x808caf6: MOV EAX, dword ptr [ECX + 0x8]
0x808caf9: XOR EDX, EDX
0x808cafb: TEST byte ptr [EAX + 0xd], 0x80
0x808caff: JZ 0x0808caf2
0x808cb01: SUB ESP, 0xc
0x808cb04: PUSH ECX
0x808cb05: CALL 0x0808ad98
0x808cb0a: MOV ESI, ESI
0x808cb0c: MOV EDX, 0x1
0x808cb11: MOV EAX, EDX
0x808cb13: LEAVE
0x808cb14: RET
0x808cb15: MOV EAX, dword ptr [ECX + 0x8]
0x808cb18: TEST EAX, EAX
0x808cb1a: SETZ AL
0x808cb1d: MOVZX EDX, AL
0x808cb20: MOV EAX, EDX
0x808cb22: LEAVE
0x808cb23: RET
0x808cb24: FLD float ptr [ECX + 0x8]
0x808cb27: FLDZ
0x808cb29: FUCOMIP ST0, ST1
0x808cb2b: FSTP ST0
0x808cb2d: SETNP DL
0x808cb30: SETZ AL
0x808cb33: AND EAX, EDX
0x808cb35: MOVZX EDX, AL
0x808cb38: JMP 0x0808cb20
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_leValue / procedure_assembly

Evidence: `ev_e5655285c0776a4f44b0f8f21144e2a1f5912136f4b957408c2f757cf9801cf6`

```asm
0x808f1c0: PUSH EBP
0x808f1c1: MOV EBP, ESP
0x808f1c3: PUSH ESI
0x808f1c4: PUSH EBX
0x808f1c5: SUB ESP, 0x10
0x808f1c8: MOV EAX, [0x086619cc]
0x808f1cd: LEA EAX, [EAX + EAX*0x2]
0x808f1d0: MOV EDX, dword ptr [0x08661964]
0x808f1d6: LEA EBX, [EDX + EAX*0x8]
0x808f1d9: LEA ESI, [EBX + 0x18]
0x808f1dc: MOV AL, byte ptr [ESI + 0x4]
0x808f1df: MOV CL, byte ptr [EBX + 0x4]
0x808f1e2: CMP AL, CL
0x808f1e4: JZ 0x0808f258
0x808f1e6: MOVZX EDX, AL
0x808f1e9: CMP EDX, 0x3
0x808f1ec: JZ 0x0808f28c
0x808f1f2: JG 0x0808f224
0x808f1f4: CMP EDX, 0x2
0x808f1f7: JZ 0x0808f2d7
0x808f1fd: SUB ESP, 0xc
0x808f200: MOVZX EAX, AL
0x808f203: PUSH EAX
0x808f204: CALL 0x0808a13c
0x808f209: POP EDX
0x808f20a: POP ECX
0x808f20b: PUSH EAX
0x808f20c: PUSH 0x82ad7f4
0x808f211: CALL 0x08097b24
0x808f216: ADD ESP, 0x10
0x808f219: LEA ESP, [EBP + -0x8]
0x808f21c: POP EBX
0x808f21d: POP ESI
0x808f21e: LEAVE
0x808f21f: JMP 0x0809061c
0x808f224: CMP EDX, 0x4
0x808f227: JZ 0x0808f2fc
0x808f22d: CMP EDX, 0x5
0x808f230: JNZ 0x0808f1fd
0x808f232: PUSH EAX
0x808f233: PUSH 0x82acfc2
0x808f238: PUSH EBX
0x808f239: PUSH ESI
0x808f23a: CALL 0x0808cda0
0x808f23f: MOV EBX, EAX
0x808f241: MOV dword ptr [ESP], ESI
0x808f244: CALL 0x0808ad98
0x808f249: ADD ESP, 0x10
0x808f24c: TEST EBX, EBX
0x808f24e: SETLE AL
0x808f251: MOV byte ptr [ESI + 0x4], 0x2
0x808f255: JMP 0x0808f2bb
0x808f258: CMP AL, 0x2
0x808f25a: JZ 0x0808f2cc
0x808f25c: CMP AL, 0x4
0x808f25e: JNZ 0x0808f1e6
0x808f260: MOV EAX, dword ptr [EBX + 0x8]
0x808f263: SUB ESP, 0x8
0x808f266: MOV EDX, dword ptr [ESI + 0x8]
0x808f269: PUSH dword ptr [EAX + 0xc]
0x808f26c: PUSH dword ptr [EDX + 0xc]
0x808f26f: CALL 0x0804b064
0x808f274: XOR EBX, EBX
0x808f276: MOV dword ptr [ESP], ESI
0x808f279: TEST EAX, EAX
0x808f27b: SETLE BL
0x808f27e: CALL 0x0808ad98
0x808f283: MOV byte ptr [ESI + 0x4], 0x2
0x808f287: MOV dword ptr [ESI + 0x8], EBX
0x808f28a: JMP 0x0808f216
0x808f28c: MOVZX EAX, CL
0x808f28f: CMP EAX, 0x2
0x808f292: JZ 0x0808f2ab
0x808f294: CMP EAX, 0x3
0x808f297: JZ 0x0808f313
0x808f299: SUB ESP, 0xc
0x808f29c: PUSH 0x82ad824
0x808f2a1: CALL 0x08097b24
0x808f2a6: JMP 0x0808f216
0x808f2ab: CVTSI2SS XMM0, dword ptr [EBX + 0x8]
0x808f2b0: MOV byte ptr [ESI + 0x4], 0x2
0x808f2b4: UCOMISS XMM0, dword ptr [ESI + 0x8]
0x808f2b8: SETNC AL
0x808f2bb: MOVZX EAX, AL
0x808f2be: MOV dword ptr [ESI + 0x8], EAX
0x808f2c1: LEA ESP, [EBP + -0x8]
0x808f2c4: POP EBX
0x808f2c5: POP ESI
0x808f2c6: LEAVE
0x808f2c7: JMP 0x0809061c
0x808f2cc: MOV EAX, dword ptr [EBX + 0x8]
0x808f2cf: CMP dword ptr [ESI + 0x8], EAX
0x808f2d2: SETLE AL
0x808f2d5: JMP 0x0808f2bb
0x808f2d7: MOVZX EAX, CL
0x808f2da: CMP EAX, 0x2
0x808f2dd: JZ 0x0808f2cc
0x808f2df: CMP EAX, 0x3
0x808f2e2: JNZ 0x0808f299
0x808f2e4: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808f2e9: FLD float ptr [EBX + 0x8]
0x808f2ec: MOVSS dword ptr [EBP + -0xc], XMM0
0x808f2f1: FLD float ptr [EBP + -0xc]
0x808f2f4: FXCH
0x808f2f6: FUCOMIP ST0, ST1
0x808f2f8: FSTP ST0
0x808f2fa: JMP 0x0808f2b8
0x808f2fc: CMP CL, 0x4
0x808f2ff: JZ 0x0808f325
0x808f301: SUB ESP, 0xc
0x808f304: PUSH 0x82ad850
0x808f309: CALL 0x08097b24
0x808f30e: JMP 0x0808f216
0x808f313: FLD float ptr [EBX + 0x8]
0x808f316: FLD float ptr [ESI + 0x8]
0x808f319: FXCH
0x808f31b: FUCOMIP ST0, ST1
0x808f31d: FSTP ST0
0x808f31f: MOV byte ptr [ESI + 0x4], 0x2
0x808f323: JMP 0x0808f2b8
0x808f325: MOV EAX, dword ptr [EBX + 0x8]
0x808f328: SUB ESP, 0x8
0x808f32b: MOV EDX, dword ptr [ESI + 0x8]
0x808f32e: PUSH dword ptr [EAX + 0xc]
0x808f331: PUSH dword ptr [EDX + 0xc]
0x808f334: CALL 0x0804b064
0x808f339: JMP 0x0808f274
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808f21f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f2c7 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_lshValue / procedure_assembly

Evidence: `ev_21dbfb327290ca0685271bfb41c56232985c41cd73c0a3774f39312728193216`

```asm
0x808df54: PUSH EBP
0x808df55: MOV EBP, ESP
0x808df57: PUSH ESI
0x808df58: PUSH EBX
0x808df59: MOV EAX, [0x086619cc]
0x808df5e: LEA EAX, [EAX + EAX*0x2]
0x808df61: MOV EDX, dword ptr [0x08661964]
0x808df67: LEA EBX, [EDX + EAX*0x8]
0x808df6a: LEA ESI, [EBX + 0x18]
0x808df6d: MOV AL, byte ptr [ESI + 0x4]
0x808df70: MOVZX EDX, AL
0x808df73: CMP EDX, 0x2
0x808df76: JZ 0x0808dfa4
0x808df78: CMP EDX, 0x7
0x808df7b: JZ 0x0808dfd4
0x808df7d: SUB ESP, 0xc
0x808df80: MOVZX EAX, AL
0x808df83: PUSH EAX
0x808df84: CALL 0x0808a13c
0x808df89: POP EDX
0x808df8a: POP ECX
0x808df8b: PUSH EAX
0x808df8c: PUSH 0x82ad058
0x808df91: CALL 0x08097b24
0x808df96: ADD ESP, 0x10
0x808df99: LEA ESP, [EBP + -0x8]
0x808df9c: POP EBX
0x808df9d: POP ESI
0x808df9e: LEAVE
0x808df9f: JMP 0x0809061c
0x808dfa4: CMP byte ptr [EBX + 0x4], 0x2
0x808dfa8: JZ 0x0808dfba
0x808dfaa: SUB ESP, 0xc
0x808dfad: PUSH 0x82ad088
0x808dfb2: CALL 0x08097b24
0x808dfb7: ADD ESP, 0x10
0x808dfba: MOV ECX, dword ptr [EBX + 0x8]
0x808dfbd: CMP ECX, 0x1f
0x808dfc0: JBE 0x0808e000
0x808dfc2: MOV dword ptr [ESI + 0x8], 0x0
0x808dfc9: LEA ESP, [EBP + -0x8]
0x808dfcc: POP EBX
0x808dfcd: POP ESI
0x808dfce: LEAVE
0x808dfcf: JMP 0x0809061c
0x808dfd4: CMP byte ptr [EBX + 0x4], 0x7
0x808dfd8: JZ 0x0808dfea
0x808dfda: SUB ESP, 0xc
0x808dfdd: PUSH 0x82ad0b4
0x808dfe2: CALL 0x08097b24
0x808dfe7: ADD ESP, 0x10
0x808dfea: TEST byte ptr [ESI + 0x5], 0x1
0x808dfee: JNZ 0x0808e00e
0x808dff0: SUB ESP, 0x8
0x808dff3: PUSH dword ptr [EBX + 0x8]
0x808dff6: PUSH dword ptr [ESI + 0x8]
0x808dff9: CALL 0x0809db64
0x808dffe: JMP 0x0808df96
0x808e000: SHL dword ptr [ESI + 0x8], CL
0x808e003: LEA ESP, [EBP + -0x8]
0x808e006: POP EBX
0x808e007: POP ESI
0x808e008: LEAVE
0x808e009: JMP 0x0809061c
0x808e00e: SUB ESP, 0xc
0x808e011: PUSH 0x82ad0e4
0x808e016: CALL 0x08097b24
0x808e01b: ADD ESP, 0x10
0x808e01e: JMP 0x0808dff0
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808df9f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808dfcf to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e009 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_lshValue / procedure_pseudo_code

Evidence: `ev_527b853d4430d751bc5211a2ee1e4a12e71feda211e4afc2f95245822c9c3197`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_lshValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 != 2) {
    if (bVar1 == 7) {
      if (vm_stack[vm_sp].type != '\a') {
        vm_error("Bad <<, right isn\'t a mapping when left is.\n");
      }
      if ((pVVar2[iVar3 + 1].attrib & 1) != 0) {
        vm_error("Const mapping can\'t absorb other mapping.\n");
      }
      vm_concatMapping((Vm_Mapping_T *)pVVar2[iVar3 + 1].v.string,
                       (Vm_Mapping_T *)pVVar2[iVar3].v.string);
    }
    else {
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad <<, type invalid (got %s) of left value.\n",pcVar4);
    }
    vm_popStack();
    return;
  }
  if (vm_stack[vm_sp].type != '\x02') {
    vm_error("Bad <<, right isn\'t a number when left is.\n");
  }
  if (0x1f < (uint)pVVar2[iVar3].v.number) {
    pVVar2[iVar3 + 1].v.info = (void *)0x0;
    vm_popStack();
    return;
  }
  pVVar2[iVar3 + 1].v.number = pVVar2[iVar3 + 1].v.number << ((byte)pVVar2[iVar3].v.number & 0x1f);
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
- Ghidra Listing reports a terminal call at 0x808df9f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808dfcf to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e009 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_ltValue / procedure_assembly

Evidence: `ev_ec37a2191d68d1b9ae0be0762456b6e784c40b47d237bbe075457071a3086d2d`

```asm
0x808f4c8: PUSH EBP
0x808f4c9: MOV EBP, ESP
0x808f4cb: PUSH ESI
0x808f4cc: PUSH EBX
0x808f4cd: SUB ESP, 0x10
0x808f4d0: MOV EAX, [0x086619cc]
0x808f4d5: LEA EAX, [EAX + EAX*0x2]
0x808f4d8: MOV EDX, dword ptr [0x08661964]
0x808f4de: LEA EBX, [EDX + EAX*0x8]
0x808f4e1: LEA ESI, [EBX + 0x18]
0x808f4e4: MOV AL, byte ptr [ESI + 0x4]
0x808f4e7: MOV CL, byte ptr [EBX + 0x4]
0x808f4ea: CMP AL, CL
0x808f4ec: JZ 0x0808f54c
0x808f4ee: MOVZX EDX, AL
0x808f4f1: CMP EDX, 0x3
0x808f4f4: JZ 0x0808f580
0x808f4fa: JG 0x0808f52c
0x808f4fc: CMP EDX, 0x2
0x808f4ff: JZ 0x0808f5cb
0x808f505: SUB ESP, 0xc
0x808f508: MOVZX EAX, AL
0x808f50b: PUSH EAX
0x808f50c: CALL 0x0808a13c
0x808f511: POP EDX
0x808f512: POP ECX
0x808f513: PUSH EAX
0x808f514: PUSH 0x82ad904
0x808f519: CALL 0x08097b24
0x808f51e: ADD ESP, 0x10
0x808f521: LEA ESP, [EBP + -0x8]
0x808f524: POP EBX
0x808f525: POP ESI
0x808f526: LEAVE
0x808f527: JMP 0x0809061c
0x808f52c: CMP EDX, 0x4
0x808f52f: JZ 0x0808f5f0
0x808f535: CMP EDX, 0x5
0x808f538: JNZ 0x0808f505
0x808f53a: PUSH EAX
0x808f53b: PUSH 0x82e2849
0x808f540: PUSH EBX
0x808f541: PUSH ESI
0x808f542: CALL 0x0808cda0
0x808f547: JMP 0x0808f568
0x808f54c: CMP AL, 0x2
0x808f54e: JZ 0x0808f5c0
0x808f550: CMP AL, 0x4
0x808f552: JNZ 0x0808f4ee
0x808f554: MOV EAX, dword ptr [EBX + 0x8]
0x808f557: SUB ESP, 0x8
0x808f55a: MOV EDX, dword ptr [ESI + 0x8]
0x808f55d: PUSH dword ptr [EAX + 0xc]
0x808f560: PUSH dword ptr [EDX + 0xc]
0x808f563: CALL 0x0804b064
0x808f568: MOV EBX, EAX
0x808f56a: MOV dword ptr [ESP], ESI
0x808f56d: CALL 0x0808ad98
0x808f572: SHR EBX, 0x1f
0x808f575: MOV byte ptr [ESI + 0x4], 0x2
0x808f579: MOV dword ptr [ESI + 0x8], EBX
0x808f57c: JMP 0x0808f51e
0x808f580: MOVZX EAX, CL
0x808f583: CMP EAX, 0x2
0x808f586: JZ 0x0808f59c
0x808f588: CMP EAX, 0x3
0x808f58b: JZ 0x0808f607
0x808f58d: SUB ESP, 0xc
0x808f590: PUSH 0x82ad934
0x808f595: CALL 0x08097b24
0x808f59a: JMP 0x0808f51e
0x808f59c: CVTSI2SS XMM0, dword ptr [EBX + 0x8]
0x808f5a1: MOV byte ptr [ESI + 0x4], 0x2
0x808f5a5: UCOMISS XMM0, dword ptr [ESI + 0x8]
0x808f5a9: SETA AL
0x808f5ac: MOVZX EAX, AL
0x808f5af: MOV dword ptr [ESI + 0x8], EAX
0x808f5b2: LEA ESP, [EBP + -0x8]
0x808f5b5: POP EBX
0x808f5b6: POP ESI
0x808f5b7: LEAVE
0x808f5b8: JMP 0x0809061c
0x808f5c0: MOV EAX, dword ptr [EBX + 0x8]
0x808f5c3: CMP dword ptr [ESI + 0x8], EAX
0x808f5c6: SETL AL
0x808f5c9: JMP 0x0808f5ac
0x808f5cb: MOVZX EAX, CL
0x808f5ce: CMP EAX, 0x2
0x808f5d1: JZ 0x0808f5c0
0x808f5d3: CMP EAX, 0x3
0x808f5d6: JNZ 0x0808f58d
0x808f5d8: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808f5dd: FLD float ptr [EBX + 0x8]
0x808f5e0: MOVSS dword ptr [EBP + -0xc], XMM0
0x808f5e5: FLD float ptr [EBP + -0xc]
0x808f5e8: FXCH
0x808f5ea: FUCOMIP ST0, ST1
0x808f5ec: FSTP ST0
0x808f5ee: JMP 0x0808f5a9
0x808f5f0: CMP CL, 0x4
0x808f5f3: JZ 0x0808f619
0x808f5f5: SUB ESP, 0xc
0x808f5f8: PUSH 0x82ad960
0x808f5fd: CALL 0x08097b24
0x808f602: JMP 0x0808f51e
0x808f607: FLD float ptr [EBX + 0x8]
0x808f60a: FLD float ptr [ESI + 0x8]
0x808f60d: FXCH
0x808f60f: FUCOMIP ST0, ST1
0x808f611: FSTP ST0
0x808f613: MOV byte ptr [ESI + 0x4], 0x2
0x808f617: JMP 0x0808f5a9
0x808f619: MOV EAX, dword ptr [EBX + 0x8]
0x808f61c: SUB ESP, 0x8
0x808f61f: MOV EDX, dword ptr [ESI + 0x8]
0x808f622: PUSH dword ptr [EAX + 0xc]
0x808f625: PUSH dword ptr [EDX + 0xc]
0x808f628: CALL 0x0804b064
0x808f62d: JMP 0x0808f568
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808f527 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f5b8 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_neValue / procedure_pseudo_code

Evidence: `ev_3d5dfe19263e352be0e9b4cec6b9dc663f1e155cc579e8be9b46ada0e1fdcd93`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_neValue(void)

{
  Vm_Value_T *pValue;
  int iVar1;
  anon_union_4_11_5000f750_for_v aVar2;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg6(ESI)] */
  iVar1 = vm_isEq(vm_stack + vm_sp + 1,vm_stack + vm_sp);
  aVar2.number._1_3_ = 0;
  aVar2.number._0_1_ = iVar1 == 0;
  vm_popStack();
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int number@[???] */
  pValue = vm_stack + vm_sp;
  vm_cleanupValue(pValue);
  pValue->type = '\x02';
  pValue->v = aVar2;
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

## vm_notValue / procedure_pseudo_code

Evidence: `ev_702ed64639ec6581ac309f74bc113a013c52c5df268c9277d921694029c3578b`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_notValue(void)

{
  Vm_Value_T *pValue;
  int iVar1;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg3(EBX)]
                       Unresolved local var: int result@[DW_OP_reg6(ESI)] */
  pValue = vm_stack + vm_sp;
  iVar1 = vm_isZero(pValue);
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

## vm_orValue / procedure_pseudo_code

Evidence: `ev_6c05653140c74d4771951a5b89e0ce34ca3ec118eedf12db414a4e412170f307`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_orValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  Vm_String_T *pVVar5;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Array_T * pNewArray@[DW_OP_reg3(EBX)] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 != 2) {
    if (bVar1 == 6) {
      if (vm_stack[vm_sp].type != '\x06') {
        vm_error("Bad |, right is\'t an array when left is.\n");
      }
      pVVar5 = (Vm_String_T *)
               vm_orArray((Vm_Array_T *)pVVar2[iVar3 + 1].v.string,
                          (Vm_Array_T *)pVVar2[iVar3].v.string);
      vm_unrefArray((Vm_Array_T *)pVVar2[iVar3 + 1].v.string);
      pVVar2[iVar3 + 1].v.string = pVVar5;
    }
    else {
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad |, type invalid (got %s) of left value.\n",pcVar4);
    }
    vm_popStack();
    return;
  }
  if (vm_stack[vm_sp].type != '\x02') {
    vm_error("Bad |, right isn\'t a number when left is.\n");
  }
  pVVar2[iVar3 + 1].v.number = pVVar2[iVar3 + 1].v.number | pVVar2[iVar3].v.number;
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
- Ghidra Listing reports a terminal call at 0x808eb07 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808eb2e to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushRealFromWord / procedure_assembly

Evidence: `ev_cdd7939016779eff6ec0d88ba540e0f49ae57bc8c487fa746a0ec1488602dcc4`

```asm
0x8090be4: PUSH EBP
0x8090be5: MOV EAX, [0x086619cc]
0x8090bea: MOV EBP, ESP
0x8090bec: TEST EAX, EAX
0x8090bee: PUSH ESI
0x8090bef: PUSH EBX
0x8090bf0: JZ 0x08090c24
0x8090bf2: DEC EAX
0x8090bf3: MOV [0x086619cc], EAX
0x8090bf8: MOV EDX, dword ptr [0x08661964]
0x8090bfe: LEA EAX, [EAX + EAX*0x2]
0x8090c01: MOV EBX, dword ptr [EBP + 0x8]
0x8090c04: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x3
0x8090c09: MOV EAX, [0x086619cc]
0x8090c0e: MOV EDX, dword ptr [0x08661964]
0x8090c14: LEA EAX, [EAX + EAX*0x2]
0x8090c17: MOV dword ptr [EDX + EAX*0x8 + 0x8], EBX
0x8090c1b: LEA ESP, [EBP + -0x8]
0x8090c1e: POP EBX
0x8090c1f: POP ESI
0x8090c20: LEAVE
0x8090c21: RET
0x8090c24: SUB ESP, 0xc
0x8090c27: PUSH 0x82ad98c
0x8090c2c: CALL 0x08097b24
0x8090c31: ADD ESP, 0x10
0x8090c34: MOV EAX, [0x086619cc]
0x8090c39: JMP 0x08090bf2
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushRealFromWord / procedure_pseudo_code

Evidence: `ev_e72af8014b0551f5e2ffa70b50b794b66d0ab39237f4e89f63886e60198bd0a7`

```c

/* WARNING: Unknown calling convention */

void vm_pushRealFromWord(int firstWord,int secondWord)

{
                    /* Unresolved local var: Vm_Word_To_Real_T u@[DW_OP_reg3(EBX); DW_OP_piece: 4;
                       DW_OP_reg4(ESP); DW_OP_piece: 4] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x03';
  vm_stack[vm_sp].v.number = firstWord;
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

## vm_revValue / procedure_pseudo_code

Evidence: `ev_3057bfe1218374db2c82a8c5b629f83837a4e785fc9607a2c27b096cb8e945d7`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_revValue(void)

{
  char *pcVar1;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg0(EAX)] */
  if (vm_stack[vm_sp].type != 2) {
    pcVar1 = vm_valueType((uint)vm_stack[vm_sp].type);
    vm_error("Bad ~, type invalid of right value.\n",pcVar1);
    return;
  }
  vm_stack[vm_sp].v.number = ~vm_stack[vm_sp].v.number;
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

## vm_rshValue / procedure_assembly

Evidence: `ev_1c5fafbbbc3f27ea7fa86a384a6d5f136eb481c392bae9110df18e0f732626f5`

```asm
0x808e020: PUSH EBP
0x808e021: MOV EBP, ESP
0x808e023: PUSH ESI
0x808e024: PUSH EBX
0x808e025: MOV EAX, [0x086619cc]
0x808e02a: LEA EAX, [EAX + EAX*0x2]
0x808e02d: MOV EDX, dword ptr [0x08661964]
0x808e033: LEA EBX, [EDX + EAX*0x8]
0x808e036: LEA ESI, [EBX + 0x18]
0x808e039: MOV AL, byte ptr [ESI + 0x4]
0x808e03c: MOVZX EDX, AL
0x808e03f: CMP EDX, 0x2
0x808e042: JZ 0x0808e070
0x808e044: CMP EDX, 0x7
0x808e047: JZ 0x0808e0a4
0x808e049: SUB ESP, 0xc
0x808e04c: MOVZX EAX, AL
0x808e04f: PUSH EAX
0x808e050: CALL 0x0808a13c
0x808e055: POP EBX
0x808e056: POP ESI
0x808e057: PUSH EAX
0x808e058: PUSH 0x82ad110
0x808e05d: CALL 0x08097b24
0x808e062: ADD ESP, 0x10
0x808e065: LEA ESP, [EBP + -0x8]
0x808e068: POP EBX
0x808e069: POP ESI
0x808e06a: LEAVE
0x808e06b: JMP 0x0809061c
0x808e070: CMP byte ptr [EBX + 0x4], 0x2
0x808e074: JZ 0x0808e086
0x808e076: SUB ESP, 0xc
0x808e079: PUSH 0x82ad140
0x808e07e: CALL 0x08097b24
0x808e083: ADD ESP, 0x10
0x808e086: MOV ECX, dword ptr [EBX + 0x8]
0x808e089: CMP ECX, 0x1f
0x808e08c: JBE 0x0808e114
0x808e092: MOV dword ptr [ESI + 0x8], 0x0
0x808e099: LEA ESP, [EBP + -0x8]
0x808e09c: POP EBX
0x808e09d: POP ESI
0x808e09e: LEAVE
0x808e09f: JMP 0x0809061c
0x808e0a4: CMP byte ptr [EBX + 0x4], 0x7
0x808e0a8: JZ 0x0808e0ba
0x808e0aa: SUB ESP, 0xc
0x808e0ad: PUSH 0x82ad16c
0x808e0b2: CALL 0x08097b24
0x808e0b7: ADD ESP, 0x10
0x808e0ba: TEST byte ptr [EBX + 0x5], 0x1
0x808e0be: JNZ 0x0808e122
0x808e0c0: SUB ESP, 0x8
0x808e0c3: PUSH dword ptr [ESI + 0x8]
0x808e0c6: PUSH dword ptr [EBX + 0x8]
0x808e0c9: CALL 0x0809db64
0x808e0ce: POP EAX
0x808e0cf: PUSH dword ptr [ESI + 0x8]
0x808e0d2: CALL 0x0809c488
0x808e0d7: MOV EAX, dword ptr [EBX + 0x8]
0x808e0da: MOV dword ptr [ESI + 0x8], EAX
0x808e0dd: MOV EAX, [0x086619cc]
0x808e0e2: MOV EDX, dword ptr [0x08661964]
0x808e0e8: LEA EAX, [EAX + EAX*0x2]
0x808e0eb: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x808e0f0: MOV EAX, [0x086619cc]
0x808e0f5: MOV EDX, dword ptr [0x08661964]
0x808e0fb: LEA EAX, [EAX + EAX*0x2]
0x808e0fe: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x808e103: INC dword ptr [0x086619cc]
0x808e109: ADD ESP, 0x10
0x808e10c: LEA ESP, [EBP + -0x8]
0x808e10f: POP EBX
0x808e110: POP ESI
0x808e111: LEAVE
0x808e112: RET
0x808e114: SHR dword ptr [ESI + 0x8], CL
0x808e117: LEA ESP, [EBP + -0x8]
0x808e11a: POP EBX
0x808e11b: POP ESI
0x808e11c: LEAVE
0x808e11d: JMP 0x0809061c
0x808e122: SUB ESP, 0xc
0x808e125: PUSH 0x82ad19c
0x808e12a: CALL 0x08097b24
0x808e12f: ADD ESP, 0x10
0x808e132: JMP 0x0808e0c0
```

Limitations:
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808e06b to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e09f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e11d to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_rshValue / procedure_pseudo_code

Evidence: `ev_3bd7d6811bdd22c1e2bb55bb68dc4822826f2405d8c22db6fa0d352e9c336df0`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_rshValue(void)

{
  byte bVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar2 = vm_stack;
  bVar1 = vm_stack[vm_sp + 1].type;
  if (bVar1 != 2) {
    if (bVar1 != 7) {
      pcVar4 = vm_valueType((uint)bVar1);
      vm_error("Bad >>, type invalid (got %s) of left value.\n",pcVar4);
      vm_popStack();
      return;
    }
    if (vm_stack[vm_sp].type != '\a') {
      vm_error("Bad >>, right isn\'t a mapping when left is.\n");
    }
    if ((pVVar2[iVar3].attrib & 1) != 0) {
      vm_error("Const mapping can concat other mapping.\n");
    }
    vm_concatMapping((Vm_Mapping_T *)pVVar2[iVar3].v.string,
                     (Vm_Mapping_T *)pVVar2[iVar3 + 1].v.string);
    vm_unrefMapping((Vm_Mapping_T *)pVVar2[iVar3 + 1].v.string);
    pVVar2[iVar3 + 1].v = pVVar2[iVar3].v;
    vm_stack[vm_sp].type = '\0';
    vm_stack[vm_sp].attrib = '\0';
    vm_sp = vm_sp + 1;
    return;
  }
  if (vm_stack[vm_sp].type != '\x02') {
    vm_error("Bad >>, right isn\'t a number when left is.\n");
  }
  if (0x1f < (uint)pVVar2[iVar3].v.number) {
    pVVar2[iVar3 + 1].v.info = (void *)0x0;
    vm_popStack();
    return;
  }
  pVVar2[iVar3 + 1].v.number =
       (uint)pVVar2[iVar3 + 1].v.number >> ((byte)pVVar2[iVar3].v.number & 0x1f);
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
- Ghidra Listing reports a terminal call at 0x808e06b to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e09f to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e11d to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_xorValue / procedure_pseudo_code

Evidence: `ev_d3fd738bfebc02bb5abf3c3d18850204cd06896ff2aa9fc88d5fc6bf10c326b7`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_xorValue(void)

{
  anon_union_4_11_5000f750_for_v *paVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  char *pcVar4;
  
  iVar3 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
  pVVar2 = vm_stack;
  if (vm_stack[vm_sp + 1].type != 2) {
    pcVar4 = vm_valueType((uint)vm_stack[vm_sp + 1].type);
    vm_error("Bad ^, type invalid (got %s) of left value.\n",pcVar4);
    vm_popStack();
    return;
  }
  if (vm_stack[vm_sp].type != '\x02') {
    vm_error("Bad ^, right isn\'t a number when left is.\n");
  }
  paVar1 = &pVVar2[iVar3 + 1].v;
  paVar1->number = paVar1->number ^ pVVar2[iVar3].v.number;
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
- Ghidra Listing reports a terminal call at 0x808e175 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808e19e to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.
