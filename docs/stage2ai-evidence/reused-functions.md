# Reused native Bytes evidence

Complete canonical records are retained in `reused.json.gz`; this view reproduces complete pseudocode and assembly for selected records. Static observations only; no reference target execution.

## vm_addValue / analyze_function

Evidence: `ev_42e127d62e334cc26bafbc7565df879a834dbffba8a6a59f495f385632e44e7d`; source: `docs/evidence/magic_Linux32-rea.json.gz`; entry: `0x808e3b8`.

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_addValue(void)

{
  char **ppcVar1;
  Vm_Value_T *pValue;
  byte bVar2;
  uint len;
  uint uVar3;
  Vm_String_T *pVVar4;
  Vm_Mapping_T *pVVar5;
  int iVar6;
  char *pcVar7;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: uint len@[DW_OP_reg3(EBX)]
                       Unresolved local var: char * str@[DW_OP_reg0(EAX)] */
  pValue = vm_stack + vm_sp;
  switch(pValue[1].type) {
  default:
    vm_valueType((uint)pValue[1].type);
    pcVar7 = "Bad +, type invalid (got %s) of left value.\n";
    break;
  case '\x02':
    if (pValue->type == '\x02') {
      pValue[1].v.info = (void *)((int)pValue[1].v.info + (int)(pValue->v).info);
    }
    else {
      if (pValue->type != '\x03') {
        pcVar7 = "Bad +, right isn\'t a number when left is.\n";
        break;
      }
      iVar6 = _vm_ftol((pValue->v).real + (float)pValue[1].v.number);
      pValue[1].v.number = iVar6;
    }
    goto LAB_0808e4cb;
  case '\x03':
    if (pValue->type == '\x02') {
      pValue[1].v.real = pValue[1].v.real + (float)(pValue->v).number;
    }
    else {
      if (pValue->type != '\x03') {
        pcVar7 = "Bad +, right isn\'t a real when left is.\n";
        break;
      }
      pValue[1].v.real = (pValue->v).real + pValue[1].v.real;
    }
LAB_0808e4cb:
    vm_popStack();
    return;
  case '\x04':
    switch(pValue->type) {
    case '\0':
      pcVar7 = "Bad +, string can not be add with undefined type.\n";
      break;
    case '\x01':
      pcVar7 = "Bad +, string can not be add with void.\n";
      break;
    case '\x02':
    case '\x03':
      pcVar7 = vm_typeValue(pValue);
      pVVar4 = vm_concatStringWithCStr(pValue[1].v.string,pcVar7);
      pValue[1].v.string = pVVar4;
      goto LAB_0808e4cb;
    case '\x04':
      pVVar4 = vm_concatString(pValue[1].v.string,(pValue->v).string);
      pValue[1].v.string = pVVar4;
      goto LAB_0808e4cb;
    default:
      pcVar7 = "Bad +, right can not be add with a string.\n";
    }
    break;
  case '\x05':
    bVar2 = pValue->type;
    if (bVar2 == 1) {
      pcVar7 = "Bad +, buffer can not be add with void.\n";
    }
    else {
      if (bVar2 < 2) {
        if (bVar2 == 0) {
          pcVar7 = "Bad +, buffer can not be add with undefined type.\n";
          break;
        }
      }
      else {
        if (bVar2 == 4) {
          vm_dupBufferToModify((Vm_Buffer_T **)&pValue[1].v.string);
          vm_addStrToBuffer((Vm_Buffer_T *)pValue[1].v.string,((pValue->v).string)->ptr);
          goto LAB_0808e4cb;
        }
        if (bVar2 == 5) {
          len = ((pValue->v).buffer)->realSize;
          if (len != 0) {
            vm_dupBufferToModify((Vm_Buffer_T **)&pValue[1].v.string);
            uVar3 = vm_extendBuffer((Vm_Buffer_T *)pValue[1].v.string,len);
            if (uVar3 != len) {
              vm_error("Can not extend buffer for add.\n");
            }
            memcpy((void *)((int)&(pValue[1].v.pPtrValue)->pThis->ref +
                           (int)&(pValue[1].v.pPtrValue)->pNext->ref),((pValue->v).pPtrValue)->pThis
                   ,len);
            ppcVar1 = &(pValue[1].v.string)->ptr;
            *ppcVar1 = *ppcVar1 + len;
          }
          goto LAB_0808e4cb;
        }
      }
      pcVar7 = "Bad +, right can not be add with a buffer.\n";
    }
    break;
  case '\x06':
                    /* Unresolved local var: Vm_Array_T * pNewArray@[DW_OP_reg3(EBX)] */
    if (pValue->type == '\x06') {
      pVVar4 = (Vm_String_T *)
               vm_addArray((Vm_Array_T *)pValue[1].v.string,(Vm_Array_T *)(pValue->v).string);
      vm_unrefArray((Vm_Array_T *)pValue[1].v.string);
      pValue[1].v.string = pVVar4;
      goto LAB_0808e4cb;
    }
    pcVar7 = "Bad +, right can not be add with an array.\n";
    break;
  case '\a':
                    /* Unresolved local var: Vm_Mapping_T * pNewMapping@[DW_OP_reg3(EBX)] */
    if (pValue->type == '\a') {
      if (1 < ((Vm_Mapping_T *)pValue[1].v.string)->ref) {
        pVVar5 = vm_copyMapping((Vm_Mapping_T *)pValue[1].v.string);
        vm_unrefMapping((Vm_Mapping_T *)pValue[1].v.string);
        pValue[1].v.string = (Vm_String_T *)pVVar5;
      }
      vm_concatMapping((Vm_Mapping_T *)pValue[1].v.string,(Vm_Mapping_T *)(pValue->v).string);
      goto LAB_0808e4cb;
    }
    pcVar7 = "Bad +, right can not be add with a mapping.\n";
  }
  vm_error(pcVar7);
  return;
}


```

```asm
0x808e3b8: PUSH EBP
0x808e3b9: MOV EBP, ESP
0x808e3bb: PUSH EDI
0x808e3bc: PUSH ESI
0x808e3bd: PUSH EBX
0x808e3be: SUB ESP, 0xc
0x808e3c1: MOV EAX, [0x086619cc]
0x808e3c6: LEA EAX, [EAX + EAX*0x2]
0x808e3c9: MOV EDX, dword ptr [0x08661964]
0x808e3cf: LEA ESI, [EDX + EAX*0x8]
0x808e3d2: LEA EDI, [ESI + 0x18]
0x808e3d5: MOV AL, byte ptr [EDI + 0x4]
0x808e3d8: MOVZX EDX, AL
0x808e3db: CMP EDX, 0x7
0x808e3de: JA 0x0808e3e8
0x808e3e0: JMP dword ptr [EDX*0x4 + 0x82adbec]
0x808e3e8: SUB ESP, 0xc
0x808e3eb: MOVZX EAX, AL
0x808e3ee: PUSH EAX
0x808e3ef: CALL 0x0808a13c
0x808e3f4: POP ESI
0x808e3f5: POP EDI
0x808e3f6: PUSH EAX
0x808e3f7: PUSH 0x82ad2dc
0x808e3fc: CALL 0x08097b24
0x808e401: LEA ESP, [EBP + -0xc]
0x808e404: POP EBX
0x808e405: POP ESI
0x808e406: POP EDI
0x808e407: LEAVE
0x808e408: RET
0x808e409: CMP byte ptr [ESI + 0x4], 0x7
0x808e40d: JZ 0x0808e564
0x808e413: SUB ESP, 0xc
0x808e416: PUSH 0x82ad30c
0x808e41b: JMP 0x0808e3fc
0x808e41d: MOVZX EAX, byte ptr [ESI + 0x4]
0x808e421: CMP EAX, 0x2
0x808e424: JZ 0x0808e590
0x808e42a: CMP EAX, 0x3
0x808e42d: JZ 0x0808e619
0x808e433: SUB ESP, 0xc
0x808e436: PUSH 0x82ad33c
0x808e43b: JMP 0x0808e3fc
0x808e43d: MOVZX EAX, byte ptr [ESI + 0x4]
0x808e441: CMP EAX, 0x2
0x808e444: JZ 0x0808e5bc
0x808e44a: CMP EAX, 0x3
0x808e44d: JZ 0x0808e5ca
0x808e453: SUB ESP, 0xc
0x808e456: PUSH 0x82ad368
0x808e45b: JMP 0x0808e3fc
0x808e45d: MOVZX EAX, byte ptr [ESI + 0x4]
0x808e461: CMP EAX, 0x4
0x808e464: JA 0x0808e583
0x808e46a: JMP dword ptr [EAX*0x4 + 0x82adc0c]
0x808e471: MOVZX EAX, byte ptr [ESI + 0x4]
0x808e475: CMP EAX, 0x1
0x808e478: JZ 0x0808e5ed
0x808e47e: JLE 0x0808e5d8
0x808e484: CMP EAX, 0x4
0x808e487: JZ 0x0808e632
0x808e48d: CMP EAX, 0x5
0x808e490: JZ 0x0808e50a
0x808e492: SUB ESP, 0xc
0x808e495: PUSH 0x82ad394
0x808e49a: JMP 0x0808e3fc
0x808e49f: CMP byte ptr [ESI + 0x4], 0x6
0x808e4a3: JZ 0x0808e59b
0x808e4a9: SUB ESP, 0xc
0x808e4ac: PUSH 0x82ad3c0
0x808e4b1: JMP 0x0808e3fc
0x808e4b6: SUB ESP, 0x8
0x808e4b9: PUSH dword ptr [ESI + 0x8]
0x808e4bc: PUSH dword ptr [EDI + 0x8]
0x808e4bf: CALL 0x0808c7fc
0x808e4c4: MOV dword ptr [EDI + 0x8], EAX
0x808e4c7: NOP
0x808e4c8: ADD ESP, 0x10
0x808e4cb: LEA ESP, [EBP + -0xc]
0x808e4ce: POP EBX
0x808e4cf: POP ESI
0x808e4d0: POP EDI
0x808e4d1: LEAVE
0x808e4d2: JMP 0x0809061c
0x808e4d7: SUB ESP, 0xc
0x808e4da: PUSH ESI
0x808e4db: CALL 0x0808bd88
0x808e4e0: POP ECX
0x808e4e1: POP EBX
0x808e4e2: PUSH EAX
0x808e4e3: PUSH dword ptr [EDI + 0x8]
0x808e4e6: CALL 0x0808c83c
0x808e4eb: MOV dword ptr [EDI + 0x8], EAX
0x808e4ee: JMP 0x0808e4c8
0x808e4f0: SUB ESP, 0xc
0x808e4f3: PUSH 0x82ad3ec
0x808e4f8: JMP 0x0808e3fc
0x808e4fd: SUB ESP, 0xc
0x808e500: PUSH 0x82ad418
0x808e505: JMP 0x0808e3fc
0x808e50a: MOV EAX, dword ptr [ESI + 0x8]
0x808e50d: MOV EBX, dword ptr [EAX + 0xc]
0x808e510: TEST EBX, EBX
0x808e512: JZ 0x0808e4cb
0x808e514: SUB ESP, 0xc
0x808e517: LEA EAX, [EDI + 0x8]
0x808e51a: PUSH EAX
0x808e51b: CALL 0x0808c890
0x808e520: POP ECX
0x808e521: POP EAX
0x808e522: PUSH EBX
0x808e523: PUSH dword ptr [EDI + 0x8]
0x808e526: CALL 0x0809a5e8
0x808e52b: ADD ESP, 0x10
0x808e52e: CMP EAX, EBX
0x808e530: JZ 0x0808e542
0x808e532: SUB ESP, 0xc
0x808e535: PUSH 0x82ad44c
0x808e53a: CALL 0x08097b24
0x808e53f: ADD ESP, 0x10
0x808e542: MOV EDX, dword ptr [EDI + 0x8]
0x808e545: MOV EAX, dword ptr [EDX + 0xc]
0x808e548: ADD EAX, dword ptr [EDX + 0x10]
0x808e54b: MOV ECX, dword ptr [ESI + 0x8]
0x808e54e: PUSH EDX
0x808e54f: PUSH EBX
0x808e550: PUSH dword ptr [ECX + 0x10]
0x808e553: PUSH EAX
0x808e554: CALL 0x0804b7e4
0x808e559: MOV EAX, dword ptr [EDI + 0x8]
0x808e55c: ADD dword ptr [EAX + 0xc], EBX
0x808e55f: JMP 0x0808e4c8
0x808e564: MOV EAX, dword ptr [EDI + 0x8]
0x808e567: CMP dword ptr [EAX], 0x1
0x808e56a: JG 0x0808e5fa
0x808e570: SUB ESP, 0x8
0x808e573: PUSH dword ptr [ESI + 0x8]
0x808e576: PUSH dword ptr [EDI + 0x8]
0x808e579: CALL 0x0809db64
0x808e57e: JMP 0x0808e4c8
0x808e583: SUB ESP, 0xc
0x808e586: PUSH 0x82ad46c
0x808e58b: JMP 0x0808e3fc
0x808e590: MOV EAX, dword ptr [ESI + 0x8]
0x808e593: ADD dword ptr [EDI + 0x8], EAX
0x808e596: JMP 0x0808e4cb
0x808e59b: SUB ESP, 0x8
0x808e59e: PUSH dword ptr [ESI + 0x8]
0x808e5a1: PUSH dword ptr [EDI + 0x8]
0x808e5a4: CALL 0x0809b7b8
0x808e5a9: MOV EBX, EAX
0x808e5ab: POP EAX
0x808e5ac: PUSH dword ptr [EDI + 0x8]
0x808e5af: CALL 0x0809b350
0x808e5b4: MOV dword ptr [EDI + 0x8], EBX
0x808e5b7: JMP 0x0808e4c8
0x808e5bc: FLD float ptr [EDI + 0x8]
0x808e5bf: FIADD dword ptr [ESI + 0x8]
0x808e5c2: FSTP float ptr [EDI + 0x8]
0x808e5c5: JMP 0x0808e4cb
0x808e5ca: FLD float ptr [ESI + 0x8]
0x808e5cd: FADD float ptr [EDI + 0x8]
0x808e5d0: FSTP float ptr [EDI + 0x8]
0x808e5d3: JMP 0x0808e4cb
0x808e5d8: TEST EAX, EAX
0x808e5da: JNZ 0x0808e492
0x808e5e0: SUB ESP, 0xc
0x808e5e3: PUSH 0x82ad498
0x808e5e8: JMP 0x0808e3fc
0x808e5ed: SUB ESP, 0xc
0x808e5f0: PUSH 0x82ad4cc
0x808e5f5: JMP 0x0808e3fc
0x808e5fa: SUB ESP, 0xc
0x808e5fd: PUSH EAX
0x808e5fe: CALL 0x0809c588
0x808e603: MOV EBX, EAX
0x808e605: POP EAX
0x808e606: PUSH dword ptr [EDI + 0x8]
0x808e609: CALL 0x0809c488
0x808e60e: ADD ESP, 0x10
0x808e611: MOV dword ptr [EDI + 0x8], EBX
0x808e614: JMP 0x0808e570
0x808e619: FLD float ptr [ESI + 0x8]
0x808e61c: SUB ESP, 0x10
0x808e61f: FIADD dword ptr [EDI + 0x8]
0x808e622: FSTP float ptr [ESP]
0x808e625: CALL 0x080a36e4
0x808e62a: MOV dword ptr [EDI + 0x8], EAX
0x808e62d: JMP 0x0808e4c8
0x808e632: SUB ESP, 0xc
0x808e635: LEA EAX, [EDI + 0x8]
0x808e638: PUSH EAX
0x808e639: CALL 0x0808c890
0x808e63e: POP EAX
0x808e63f: POP EDX
0x808e640: MOV EAX, dword ptr [ESI + 0x8]
0x808e643: PUSH dword ptr [EAX + 0xc]
0x808e646: PUSH dword ptr [EDI + 0x8]
0x808e649: CALL 0x0809abf0
0x808e64e: JMP 0x0808e4c8
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
- Ghidra Listing reports a terminal call at 0x808e4d2 to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_copyValue / analyze_function

Evidence: `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`; source: `docs/stage2af-evidence/rea.json.gz`; entry: `0x808be24`.

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

```asm
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

## vm_eqValue / analyze_function

Evidence: `ev_9c31d92aeb045a089ff6361f0cf81330894208eb0520a78f0bf4bd4701c8d565`; source: `docs/stage2af-evidence/rea.json.gz`; entry: `0x808eed4`.

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

```asm
0x808eed4: PUSH EBP
0x808eed5: MOV EBP, ESP
0x808eed7: MOV EAX, [0x086619cc]
0x808eedc: PUSH ESI
0x808eedd: PUSH EBX
0x808eede: MOV EDX, dword ptr [0x08661964]
0x808eee4: LEA EAX, [EAX + EAX*0x2]
0x808eee7: SUB ESP, 0x8
0x808eeea: LEA EAX, [EDX + EAX*0x8]
0x808eeed: PUSH EAX
0x808eeee: LEA EDX, [EAX + 0x18]
0x808eef1: PUSH EDX
0x808eef2: CALL 0x0808c908
0x808eef7: MOV ESI, EAX
0x808eef9: CALL 0x0809061c
0x808eefe: MOV EBX, dword ptr [0x086619cc]
0x808ef04: MOV EAX, [0x08661964]
0x808ef09: LEA EBX, [EBX + EBX*0x2]
0x808ef0c: LEA EBX, [EAX + EBX*0x8]
0x808ef0f: MOV dword ptr [ESP], EBX
0x808ef12: CALL 0x0808ad98
0x808ef17: MOV byte ptr [EBX + 0x4], 0x2
0x808ef1b: MOV dword ptr [EBX + 0x8], ESI
0x808ef1e: ADD ESP, 0x10
0x808ef21: LEA ESP, [EBP + -0x8]
0x808ef24: POP EBX
0x808ef25: POP ESI
0x808ef26: LEAVE
0x808ef27: RET
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

## vm_isEq / analyze_function

Evidence: `ev_c98c01d582d90083a8c48681f7d36bdd274bd35ed834c0ef8fe22e6ac304baf1`; source: `docs/stage2af-evidence/rea.json.gz`; entry: `0x808c908`.

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
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_cleanupValue / analyze_function

Evidence: `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`; source: `docs/stage2af-evidence/rea.json.gz`; entry: `0x808ad98`.

```c

void vm_cleanupValue(Vm_Value_T *pValue)

{
  ushort uVar1;
  Vm_Value *pVVar2;
  int iVar3;
  Vm_Value_T *pVVar4;
  Vm_Value *pVVar5;
  uint line;
  int local_18;
  int i;
  
  switch(pValue->type) {
  case '\0':
  case '\x01':
  case '\x02':
  case '\x03':
    break;
  case '\x04':
    pVVar5 = (pValue->v).pPtrValue;
    iVar3 = pVVar5->ref + -1;
    pVVar5->ref = iVar3;
    if (0 < iVar3) break;
    if ((*(uint *)&pVVar5->type & 1) != 0) {
      vm_removeSharedString((Vm_String_T *)pVVar5);
      break;
    }
    line = 0x515;
    goto LAB_0808af83;
  case '\x05':
    vm_freeBuffer((Vm_Buffer_T *)(pValue->v).string);
    break;
  case '\x06':
    vm_unrefArray((Vm_Array_T *)(pValue->v).string);
    break;
  case '\a':
    vm_unrefMapping((Vm_Mapping_T *)(pValue->v).string);
    break;
  case '\b':
    pVVar4 = (pValue->v).pPtrValue;
    iVar3 = pVVar4->ref + -1;
    pVVar4->ref = iVar3;
    if ((iVar3 == 0) && (pVVar4 != (Vm_Value_T *)0x0)) {
      vm_cleanupValue(pVVar4);
      vm_totalValues = vm_totalValues + -1;
      if (pVVar4 == vm_allValues) {
        vm_allValues = pVVar4->pNext;
        if (vm_allValues != (Vm_Value *)0x0) {
          vm_allValues->pPrev = (Vm_Value *)0x0;
        }
      }
      else {
        pVVar5 = pVVar4->pPrev;
        pVVar5->pNext = pVVar4->pNext;
        if (pVVar4->pNext != (Vm_Value *)0x0) {
          pVVar4->pNext->pPrev = pVVar5;
        }
      }
      pVVar4->pThis = (Vm_Value *)0x0;
      pdb_freeTinyBlock(pVVar4);
    }
    break;
  case '\t':
    pVVar5 = (Vm_Value *)(pValue->v).function;
    iVar3 = pVVar5->ref + -1;
    pVVar5->ref = iVar3;
    if (0 < iVar3) break;
    i = 0;
    if (0 < (short)pVVar5->active) {
      local_18 = 0;
      do {
        pVVar4 = (Vm_Value_T *)((int)&pVVar5->pThis->ref + local_18);
        pVVar4->ref = pVVar4->ref + -1;
        vm_cleanupValue(pVVar4);
        vm_totalValues = vm_totalValues + -1;
        if (pVVar4 == vm_allValues) {
          vm_allValues = pVVar4->pNext;
          if (vm_allValues != (Vm_Value_T *)0x0) {
            vm_allValues->pPrev = (Vm_Value *)0x0;
          }
        }
        else {
          pVVar2 = pVVar4->pPrev;
          pVVar2->pNext = pVVar4->pNext;
          if (pVVar4->pNext != (Vm_Value *)0x0) {
            pVVar4->pNext->pPrev = pVVar2;
          }
        }
        i = i + 1;
        uVar1 = pVVar5->active;
        local_18 = local_18 + 0x18;
        pVVar4->pThis = (Vm_Value *)0x0;
      } while (i < (short)uVar1);
    }
    if ((Vm_Object_T *)pVVar5->pNext != (Vm_Object_T *)0x0) {
      vm_unrefObject((Vm_Object_T *)pVVar5->pNext);
    }
    pdb_freeMemory(pVVar5->pThis,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x117);
    line = 0x118;
LAB_0808af83:
    pdb_freeMemory(pVVar5,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",line);
    break;
  case '\n':
    vm_unrefObject((Vm_Object_T *)(pValue->v).string);
    break;
  default:
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value to destruct.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x1bb);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  case '\x14':
    vm_freeLoopValue((Vm_Loop_Ctrl_T *)(pValue->v).string);
  }
  pValue->type = '\0';
  pValue->attrib = '\0';
  return;
}


```

```asm
0x808ad98: PUSH EBP
0x808ad99: MOV EBP, ESP
0x808ad9b: PUSH EDI
0x808ad9c: PUSH ESI
0x808ad9d: PUSH EBX
0x808ad9e: SUB ESP, 0xc
0x808ada1: MOV EDI, dword ptr [EBP + 0x8]
0x808ada4: MOVZX EAX, byte ptr [EDI + 0x4]
0x808ada8: CMP EAX, 0x14
0x808adab: JA 0x0808add2
0x808adad: JMP dword ptr [EAX*0x4 + 0x82ace1c]
0x808adb4: SUB ESP, 0xc
0x808adb7: PUSH dword ptr [EDI + 0x8]
0x808adba: CALL 0x080bc884
0x808adbf: ADD ESP, 0x10
0x808adc2: MOV byte ptr [EDI + 0x4], 0x0
0x808adc6: MOV byte ptr [EDI + 0x5], 0x0
0x808adca: LEA ESP, [EBP + -0xc]
0x808adcd: POP EBX
0x808adce: POP ESI
0x808adcf: POP EDI
0x808add0: LEAVE
0x808add1: RET
0x808add2: SUB ESP, 0x8
0x808add5: PUSH 0x1bb
0x808adda: PUSH 0x82acaf8
0x808addf: PUSH 0x82acc2c
0x808ade4: PUSH 0x82a825c
0x808ade9: PUSH 0x4
0x808adeb: PUSH 0x2
0x808aded: CALL 0x080b61c4
0x808adf2: ADD ESP, 0x14
0x808adf5: PUSH 0x0
0x808adf7: CALL 0x080a3980
0x808adfc: ADD ESP, 0x10
0x808adff: JMP 0x0808adff
0x808ae01: SUB ESP, 0xc
0x808ae04: PUSH dword ptr [EDI + 0x8]
0x808ae07: CALL 0x080b1618
0x808ae0c: JMP 0x0808adbf
0x808ae0e: MOV ESI, dword ptr [EDI + 0x8]
0x808ae11: MOV EAX, dword ptr [ESI]
0x808ae13: DEC EAX
0x808ae14: TEST EAX, EAX
0x808ae16: MOV dword ptr [ESI], EAX
0x808ae18: JG 0x0808adc2
0x808ae1a: CMP word ptr [ESI + 0x6], 0x0
0x808ae1f: MOV dword ptr [EBP + -0x10], 0x0
0x808ae26: JLE 0x0808ae7f
0x808ae28: MOV dword ptr [EBP + -0x14], 0x0
0x808ae2f: MOV EBX, dword ptr [EBP + -0x14]
0x808ae32: ADD EBX, dword ptr [ESI + 0x10]
0x808ae35: SUB ESP, 0xc
0x808ae38: DEC dword ptr [EBX]
0x808ae3a: PUSH EBX
0x808ae3b: CALL 0x0808ad98
0x808ae40: ADD ESP, 0x10
0x808ae43: DEC dword ptr [0x0847fd88]
0x808ae49: CMP EBX, dword ptr [0x0847fd8c]
0x808ae4f: JZ 0x0808afa9
0x808ae55: MOV EAX, dword ptr [EBX + 0xc]
0x808ae58: MOV EDX, dword ptr [EBX + 0x14]
0x808ae5b: MOV dword ptr [EDX + 0xc], EAX
0x808ae5e: MOV EAX, dword ptr [EBX + 0xc]
0x808ae61: TEST EAX, EAX
0x808ae63: JZ 0x0808ae68
0x808ae65: MOV dword ptr [EAX + 0x14], EDX
0x808ae68: INC dword ptr [EBP + -0x10]
0x808ae6b: MOVSX EAX, word ptr [ESI + 0x6]
0x808ae6f: ADD dword ptr [EBP + -0x14], 0x18
0x808ae73: CMP EAX, dword ptr [EBP + -0x10]
0x808ae76: MOV dword ptr [EBX + 0x10], 0x0
0x808ae7d: JG 0x0808ae2f
0x808ae7f: MOV EAX, dword ptr [ESI + 0xc]
0x808ae82: TEST EAX, EAX
0x808ae84: JZ 0x0808ae92
0x808ae86: SUB ESP, 0xc
0x808ae89: PUSH EAX
0x808ae8a: CALL 0x080b1618
0x808ae8f: ADD ESP, 0x10
0x808ae92: PUSH 0x117
0x808ae97: PUSH 0x82acaf8
0x808ae9c: PUSH 0x82a376b
0x808aea1: PUSH dword ptr [ESI + 0x10]
0x808aea4: CALL 0x080ceb94
0x808aea9: ADD ESP, 0x10
0x808aeac: PUSH 0x118
0x808aeb1: PUSH 0x82acaf8
0x808aeb6: PUSH 0x82a376b
0x808aebb: PUSH ESI
0x808aebc: JMP 0x0808af83
0x808aec1: MOV EBX, dword ptr [EDI + 0x8]
0x808aec4: MOV EAX, dword ptr [EBX]
0x808aec6: DEC EAX
0x808aec7: TEST EAX, EAX
0x808aec9: MOV dword ptr [EBX], EAX
0x808aecb: JNZ 0x0808adc2
0x808aed1: TEST EBX, EBX
0x808aed3: JZ 0x0808adc2
0x808aed9: SUB ESP, 0xc
0x808aedc: PUSH EBX
0x808aedd: CALL 0x0808ad98
0x808aee2: ADD ESP, 0x10
0x808aee5: DEC dword ptr [0x0847fd88]
0x808aeeb: CMP EBX, dword ptr [0x0847fd8c]
0x808aef1: JZ 0x0808af8d
0x808aef7: MOV EAX, dword ptr [EBX + 0xc]
0x808aefa: MOV EDX, dword ptr [EBX + 0x14]
0x808aefd: MOV dword ptr [EDX + 0xc], EAX
0x808af00: MOV EAX, dword ptr [EBX + 0xc]
0x808af03: TEST EAX, EAX
0x808af05: JZ 0x0808af0a
0x808af07: MOV dword ptr [EAX + 0x14], EDX
0x808af0a: SUB ESP, 0xc
0x808af0d: MOV dword ptr [EBX + 0x10], 0x0
0x808af14: PUSH EBX
0x808af15: CALL 0x080cf214
0x808af1a: JMP 0x0808adbf
0x808af1f: SUB ESP, 0xc
0x808af22: PUSH dword ptr [EDI + 0x8]
0x808af25: CALL 0x0809c488
0x808af2a: JMP 0x0808adbf
0x808af2f: SUB ESP, 0xc
0x808af32: PUSH dword ptr [EDI + 0x8]
0x808af35: CALL 0x0809b350
0x808af3a: JMP 0x0808adbf
0x808af3f: SUB ESP, 0xc
0x808af42: PUSH dword ptr [EDI + 0x8]
0x808af45: CALL 0x0809a580
0x808af4a: JMP 0x0808adbf
0x808af4f: MOV EDX, dword ptr [EDI + 0x8]
0x808af52: MOV EAX, dword ptr [EDX]
0x808af54: DEC EAX
0x808af55: TEST EAX, EAX
0x808af57: MOV dword ptr [EDX], EAX
0x808af59: JG 0x0808adc2
0x808af5f: TEST byte ptr [EDX + 0x4], 0x1
0x808af63: JZ 0x0808af73
0x808af65: SUB ESP, 0xc
0x808af68: PUSH EDX
0x808af69: CALL 0x080bd460
0x808af6e: JMP 0x0808adbf
0x808af73: PUSH 0x515
0x808af78: PUSH 0x82acaf8
0x808af7d: PUSH 0x82a376b
0x808af82: PUSH EDX
0x808af83: CALL 0x080ceb94
0x808af88: JMP 0x0808adbf
0x808af8d: MOV EAX, dword ptr [EBX + 0xc]
0x808af90: TEST EAX, EAX
0x808af92: MOV [0x0847fd8c], EAX
0x808af97: JZ 0x0808af0a
0x808af9d: MOV dword ptr [EAX + 0x14], 0x0
0x808afa4: JMP 0x0808af0a
0x808afa9: MOV EAX, dword ptr [EBX + 0xc]
0x808afac: TEST EAX, EAX
0x808afae: MOV [0x0847fd8c], EAX
0x808afb3: JZ 0x0808ae68
0x808afb9: MOV dword ptr [EAX + 0x14], 0x0
0x808afc0: JMP 0x0808ae68
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

## vm_leValue / procedure_pseudo_code

Evidence: `ev_0e786f83fad6944f8f5973948ba408ed0a0a5200486741ce8f534333189dc7b2`; source: `docs/vm-core-evidence/rea.json.gz`; entry: `see complementary assembly record`.

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

## vm_geValue / procedure_pseudo_code

Evidence: `ev_4442d99f8563c660f950f2e4e3d6fb934504618f8904fea7b1fb0f1d470b4151`; source: `docs/vm-core-evidence/rea.json.gz`; entry: `see complementary assembly record`.

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

## vm_gtValue / procedure_pseudo_code

Evidence: `ev_5046856efb8928064add53f8c3b8b91c64ee3e170372d0683e48cf9018ea0e36`; source: `docs/vm-core-evidence/rea.json.gz`; entry: `see complementary assembly record`.

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

## vm_popStack / procedure_pseudo_code

Evidence: `ev_6f8162a52b6dcead2e90f45ac594636448ad034684367dee557ea51b2cd7fc1f`; source: `docs/vm-core-evidence/rea.json.gz`; entry: `see complementary assembly record`.

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

## vm_ltValue / procedure_pseudo_code

Evidence: `ev_deee5e8743aec13f48327236f9b0793ccd92a15e202b8459782b33ff3e514c4d`; source: `docs/vm-core-evidence/rea.json.gz`; entry: `see complementary assembly record`.

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

## vm_neValue / procedure_pseudo_code

Evidence: `ev_3d5dfe19263e352be0e9b4cec6b9dc663f1e155cc579e8be9b46ada0e1fdcd93`; source: `docs/vm-numeric-evidence/rea.json.gz`; entry: `see complementary assembly record`.

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

## vm_geValue / procedure_assembly

Evidence: `ev_5d21fedab56340ea91181a5f3d19f6bf37be94cab81f615413f75caf22ffbe9c`; source: `docs/vm-numeric-evidence/rea.json.gz`; entry: `0x808ef88`.

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

## vm_leValue / procedure_assembly

Evidence: `ev_e5655285c0776a4f44b0f8f21144e2a1f5912136f4b957408c2f757cf9801cf6`; source: `docs/vm-numeric-evidence/rea.json.gz`; entry: `0x808f1c0`.

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

## vm_ltValue / procedure_assembly

Evidence: `ev_ec37a2191d68d1b9ae0be0762456b6e784c40b47d237bbe075457071a3086d2d`; source: `docs/vm-numeric-evidence/rea.json.gz`; entry: `0x808f4c8`.

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

## vm_gtValue / procedure_assembly

Evidence: `ev_fd438ebc3f1882653512f33fc644071325a9c3e299a02f546e0887e66f20e3f7`; source: `docs/vm-numeric-evidence/rea.json.gz`; entry: `0x808f340`.

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
