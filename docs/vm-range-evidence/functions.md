# 阶段 2X 原生函数证据

REA/Ghidra 12.1.4 静态结果；不是原始源代码，也不是运行观察。完整记录包含原始返回、函数体范围、CFG 和限制，见压缩包及 function-manifest.json。

## vm_queryRange / analyze_function

Evidence: `ev_0491806749b3b609484ea6e46a226a7051f750aa72841dee339ffc8278e64fe5`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808fdaa", "start": "0x808fd04"}, {"end": "0x808fec9", "start": "0x808fdac"}, {"end": "0x808ff05", "start": "0x808fecc"}, {"end": "0x809005a", "start": "0x808ff08"}], "span_bytes": 855, "total_bytes": 850}`

```c

void vm_queryRange(int flag)

{
  int iVar1;
  uchar uVar2;
  Vm_Value *pVVar3;
  Vm_Value_T *pVVar4;
  int iVar5;
  byte bVar6;
  char *pcVar7;
  Vm_Buffer_T *pBuffer;
  char *pcVar8;
  Vm_String_T *pVVar9;
  uint uVar10;
  char *pcVar11;
  int iVar12;
  char *str;
  Vm_Array_T *pNewArray;
  Vm_Value_T *pValue;
  int flag_local;
  
  iVar5 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Buffer_T * pBuffer@[DW_OP_reg3(EBX)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)]
                       Unresolved local var: int ib@[DW_OP_reg7(EDI)]
                       Unresolved local var: int ie@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint len@[DW_OP_reg6(ESI)] */
  pVVar4 = vm_stack;
  if (vm_stack[vm_sp + 1].type == '\x03') {
    iVar12 = _vm_ftol(vm_stack[vm_sp + 1].v.real);
    pVVar4[iVar5 + 1].type = '\x02';
    uVar2 = pVVar4[iVar5].type;
    pVVar4[iVar5 + 1].v.number = iVar12;
  }
  else {
    uVar2 = vm_stack[vm_sp].type;
  }
  if (uVar2 == '\x03') {
    iVar12 = _vm_ftol(pVVar4[iVar5].v.real);
    pVVar4[iVar5].v.number = iVar12;
    pVVar4[iVar5].type = '\x02';
  }
  if ((pVVar4[iVar5 + 1].type == '\x02') && (pVVar4[iVar5].type == '\x02')) {
    bVar6 = pVVar4[iVar5 + 2].type;
    uVar10 = (uint)bVar6;
    iVar12 = uVar10 - 5;
    pcVar7 = (char *)pVVar4[iVar5 + 1].v.number;
    pcVar11 = (char *)pVVar4[iVar5].v.number;
    if (iVar12 == 0) {
LAB_0808fe44:
      pcVar8 = (pVVar4[iVar5 + 2].v.string)->ptr;
      if ((flag & 1U) != 0) {
        if ((int)pcVar11 < 1) {
          pcVar11 = (char *)0x1;
        }
        pcVar11 = pcVar8 + -(int)pcVar11;
      }
      if ((flag & 2U) != 0) {
        if ((int)pcVar7 < 1) {
          pcVar7 = (char *)0x1;
        }
        pcVar7 = pcVar8 + -(int)pcVar7;
      }
      if ((int)pcVar7 < 0) {
        pcVar7 = (char *)0x0;
      }
      if ((int)pcVar8 <= (int)pcVar11) {
        pcVar11 = pcVar8 + -1;
      }
      pcVar8 = (char *)0x0;
      if ((int)pcVar7 <= (int)pcVar11) {
        pcVar8 = pcVar11 + (1 - (int)pcVar7);
      }
      pBuffer = vm_allocateBuffer((uint)pcVar8);
      if (pcVar8 != (char *)0x0) {
        memcpy(pBuffer->ptr,pcVar7 + (int)(pVVar4[iVar5 + 2].v.pPtrValue)->pThis,(size_t)pcVar8);
      }
      vm_popNStack(3);
      vm_pushBuffer(pBuffer);
      return;
    }
  }
  else {
    vm_error("Index should be integer.\n");
    bVar6 = pVVar4[iVar5 + 2].type;
    uVar10 = (uint)bVar6;
    iVar12 = uVar10 - 5;
    pcVar7 = (char *)pVVar4[iVar5 + 1].v.number;
    pcVar11 = (char *)pVVar4[iVar5].v.number;
    if (uVar10 == 5) goto LAB_0808fe44;
  }
  if (SBORROW4(uVar10,5) == iVar12 < 0) {
    if (uVar10 == 6) {
      pVVar3 = (pVVar4[iVar5 + 2].v.pLoopCtrl)->pValue;
      if ((flag & 1U) != 0) {
        if ((int)pcVar11 < 1) {
          pcVar11 = (char *)0x1;
        }
        pcVar11 = (char *)((int)pVVar3 - (int)pcVar11);
      }
      if ((flag & 2U) != 0) {
        if ((int)pcVar7 < 1) {
          pcVar7 = (char *)0x1;
        }
        pcVar7 = (char *)((int)pVVar3 - (int)pcVar7);
      }
      vm_popNStack(2);
      if ((int)pcVar7 < 0) {
        pcVar7 = (char *)0x0;
      }
      if ((int)pVVar3 <= (int)pcVar11) {
        pcVar11 = (char *)((int)&pVVar3[-1].pPrev + 3);
      }
      if ((int)pcVar11 < (int)pcVar7) {
        pcVar7 = pcVar11 + 1;
      }
      pcVar8 = (char *)0x0;
      if ((int)pcVar7 <= (int)pcVar11) {
        pcVar8 = pcVar11 + (1 - (int)pcVar7);
      }
      pVVar9 = (Vm_String_T *)vm_allocateEmptyArray((uint)pcVar8);
      iVar12 = 0;
      if (0 < (int)(pcVar11 + (1 - (int)pcVar7))) {
        do {
          pcVar8 = pcVar7 + iVar12;
          iVar1 = iVar12 * 4;
          iVar12 = iVar12 + 1;
          vm_copyValue(*(Vm_Value_T **)(pVVar9->ptr + iVar1),
                       *(Vm_Value_T **)((pVVar4[iVar5 + 2].v.string)->ptr + (int)pcVar8 * 4));
        } while (iVar12 < (int)(pcVar11 + (1 - (int)pcVar7)));
      }
      vm_unrefArray((Vm_Array_T *)pVVar4[iVar5 + 2].v.string);
      pVVar4[iVar5 + 2].v.string = pVVar9;
      return;
    }
    if (uVar10 == 7) {
      vm_error("Bad type(undefined) to index for range values.\n");
      bVar6 = pVVar4[iVar5 + 2].type;
    }
  }
  else if (uVar10 == 4) {
    pVVar3 = (pVVar4[iVar5 + 2].v.pPtrValue)->pThis;
    if ((flag & 1U) != 0) {
      if ((int)pcVar11 < 1) {
        pcVar11 = (char *)0x1;
      }
      pcVar11 = (char *)((int)pVVar3 - (int)pcVar11);
    }
    if ((flag & 2U) != 0) {
      if ((int)pcVar7 < 1) {
        pcVar7 = (char *)0x1;
      }
      pcVar7 = (char *)((int)pVVar3 - (int)pcVar7);
    }
    if ((int)pcVar7 < 0) {
      pcVar7 = (char *)0x0;
    }
    if ((int)pVVar3 <= (int)pcVar11) {
      pcVar11 = (char *)((int)&pVVar3[-1].pPrev + 3);
    }
    if ((int)pcVar11 < (int)pcVar7) {
      vm_popNStack(3);
      vm_pushString("");
      return;
    }
                    /* Unresolved local var: Vm_String_T * pString@[DW_OP_reg6(ESI)]
                       Unresolved local var: uint slen@[DW_OP_reg3(EBX)] */
    pcVar11 = pcVar11 + (1 - (int)pcVar7);
    pVVar9 = vm_newString((uint)pcVar11);
    pcVar8 = pVVar9->ptr;
    strncpy(pcVar8,pcVar7 + (int)(pVVar4[iVar5 + 2].v.string)->ptr,(size_t)pcVar11);
    pcVar8[(int)pcVar11] = '\0';
    pVVar9->len = (uint)pcVar11;
    vm_popNStack(3);
    vm_pushAllocedString(pVVar9);
    return;
  }
  pcVar7 = vm_valueType((uint)bVar6);
  vm_error("Bad type(%s) to index for range values.\n",pcVar7);
  return;
}


```

```text
0x808fd04: PUSH EBP
0x808fd05: MOV EBP, ESP
0x808fd07: PUSH EDI
0x808fd08: PUSH ESI
0x808fd09: PUSH EBX
0x808fd0a: SUB ESP, 0x1c
0x808fd0d: MOV EAX, dword ptr [EBP + 0x8]
0x808fd10: MOV dword ptr [EBP + -0x10], EAX
0x808fd13: MOV EAX, [0x086619cc]
0x808fd18: MOV EDX, dword ptr [0x08661964]
0x808fd1e: LEA EAX, [EAX + EAX*0x2]
0x808fd21: LEA EBX, [EDX + EAX*0x8]
0x808fd24: LEA EDX, [EBX + 0x30]
0x808fd27: MOV dword ptr [EBP + -0x14], EDX
0x808fd2a: LEA ESI, [EBX + 0x18]
0x808fd2d: CMP byte ptr [ESI + 0x4], 0x3
0x808fd31: JZ 0x0808fecc
0x808fd37: CMP byte ptr [EBX + 0x4], 0x3
0x808fd3b: JZ 0x0808feec
0x808fd41: CMP byte ptr [ESI + 0x4], 0x2
0x808fd45: JZ 0x0808fe20
0x808fd4b: SUB ESP, 0xc
0x808fd4e: PUSH 0x82ad00d
0x808fd53: CALL 0x08097b24
0x808fd58: MOV EDX, dword ptr [EBP + -0x14]
0x808fd5b: MOV AL, byte ptr [EDX + 0x4]
0x808fd5e: MOVZX EDX, AL
0x808fd61: ADD ESP, 0x10
0x808fd64: CMP EDX, 0x5
0x808fd67: MOV EDI, dword ptr [ESI + 0x8]
0x808fd6a: MOV EBX, dword ptr [EBX + 0x8]
0x808fd6d: JZ 0x0808fe44
0x808fd73: JLE 0x0808fdac
0x808fd75: CMP EDX, 0x6
0x808fd78: JZ 0x0808ff08
0x808fd7e: CMP EDX, 0x7
0x808fd81: JZ 0x08090040
0x808fd87: SUB ESP, 0xc
0x808fd8a: MOVZX EAX, AL
0x808fd8d: PUSH EAX
0x808fd8e: CALL 0x0808a13c
0x808fd93: POP EBX
0x808fd94: POP ESI
0x808fd95: PUSH EAX
0x808fd96: PUSH 0x82ada9c
0x808fd9b: CALL 0x08097b24
0x808fda0: ADD ESP, 0x10
0x808fda3: LEA ESP, [EBP + -0xc]
0x808fda6: POP EBX
0x808fda7: POP ESI
0x808fda8: POP EDI
0x808fda9: LEAVE
0x808fdaa: RET
0x808fdac: CMP EDX, 0x4
0x808fdaf: JNZ 0x0808fd87
0x808fdb1: MOV EDX, dword ptr [EBP + -0x14]
0x808fdb4: MOV EAX, dword ptr [EDX + 0x8]
0x808fdb7: TEST byte ptr [EBP + -0x10], 0x1
0x808fdbb: MOV ESI, dword ptr [EAX + 0x10]
0x808fdbe: JZ 0x0808fdd0
0x808fdc0: TEST EBX, EBX
0x808fdc2: MOV EAX, 0x1
0x808fdc7: CMOVLE EBX, EAX
0x808fdca: MOV EAX, ESI
0x808fdcc: SUB EAX, EBX
0x808fdce: MOV EBX, EAX
0x808fdd0: TEST byte ptr [EBP + -0x10], 0x2
0x808fdd4: JZ 0x0808fde6
0x808fdd6: TEST EDI, EDI
0x808fdd8: MOV EAX, 0x1
0x808fddd: CMOVLE EDI, EAX
0x808fde0: MOV EDX, ESI
0x808fde2: SUB EDX, EDI
0x808fde4: MOV EDI, EDX
0x808fde6: TEST EDI, EDI
0x808fde8: MOV EAX, 0x0
0x808fded: CMOVS EDI, EAX
0x808fdf0: CMP ESI, EBX
0x808fdf2: LEA EAX, [ESI + -0x1]
0x808fdf5: CMOVLE EBX, EAX
0x808fdf8: CMP EDI, EBX
0x808fdfa: JLE 0x0808ffd2
0x808fe00: SUB ESP, 0xc
0x808fe03: PUSH 0x3
0x808fe05: CALL 0x08091ab0
0x808fe0a: MOV dword ptr [EBP + 0x8], 0x82b0704
0x808fe11: ADD ESP, 0x10
0x808fe14: LEA ESP, [EBP + -0xc]
0x808fe17: POP EBX
0x808fe18: POP ESI
0x808fe19: POP EDI
0x808fe1a: LEAVE
0x808fe1b: JMP 0x08090644
0x808fe20: CMP byte ptr [EBX + 0x4], 0x2
0x808fe24: JNZ 0x0808fd4b
0x808fe2a: MOV EDX, dword ptr [EBP + -0x14]
0x808fe2d: MOV AL, byte ptr [EDX + 0x4]
0x808fe30: MOVZX EDX, AL
0x808fe33: CMP EDX, 0x5
0x808fe36: MOV EDI, dword ptr [ESI + 0x8]
0x808fe39: MOV EBX, dword ptr [EBX + 0x8]
0x808fe3c: JNZ 0x0808fd73
0x808fe42: MOV ESI, ESI
0x808fe44: MOV EDX, dword ptr [EBP + -0x14]
0x808fe47: MOV EAX, dword ptr [EDX + 0x8]
0x808fe4a: TEST byte ptr [EBP + -0x10], 0x1
0x808fe4e: MOV ESI, dword ptr [EAX + 0xc]
0x808fe51: JNZ 0x0808feb8
0x808fe53: TEST byte ptr [EBP + -0x10], 0x2
0x808fe57: JZ 0x0808fe69
0x808fe59: TEST EDI, EDI
0x808fe5b: MOV EAX, 0x1
0x808fe60: CMOVLE EDI, EAX
0x808fe63: MOV EDX, ESI
0x808fe65: SUB EDX, EDI
0x808fe67: MOV EDI, EDX
0x808fe69: TEST EDI, EDI
0x808fe6b: MOV EAX, 0x0
0x808fe70: CMOVS EDI, EAX
0x808fe73: CMP ESI, EBX
0x808fe75: LEA EAX, [ESI + -0x1]
0x808fe78: CMOVLE EBX, EAX
0x808fe7b: XOR ESI, ESI
0x808fe7d: CMP EDI, EBX
0x808fe7f: JG 0x0808fe86
0x808fe81: SUB EBX, EDI
0x808fe83: LEA ESI, [EBX + 0x1]
0x808fe86: SUB ESP, 0xc
0x808fe89: PUSH ESI
0x808fe8a: CALL 0x0809aa38
0x808fe8f: ADD ESP, 0x10
0x808fe92: TEST ESI, ESI
0x808fe94: MOV EBX, EAX
0x808fe96: JNZ 0x08090024
0x808fe9c: SUB ESP, 0xc
0x808fe9f: PUSH 0x3
0x808fea1: CALL 0x08091ab0
0x808fea6: MOV dword ptr [EBP + 0x8], EBX
0x808fea9: ADD ESP, 0x10
0x808feac: LEA ESP, [EBP + -0xc]
0x808feaf: POP EBX
0x808feb0: POP ESI
0x808feb1: POP EDI
0x808feb2: LEAVE
0x808feb3: JMP 0x08090dd8
0x808feb8: TEST EBX, EBX
0x808feba: MOV EAX, 0x1
0x808febf: CMOVLE EBX, EAX
0x808fec2: MOV EAX, ESI
0x808fec4: SUB EAX, EBX
0x808fec6: MOV EBX, EAX
0x808fec8: JMP 0x0808fe53
0x808fecc: SUB ESP, 0xc
0x808fecf: PUSH dword ptr [ESI + 0x8]
0x808fed2: CALL 0x080a36e4
0x808fed7: MOV byte ptr [ESI + 0x4], 0x2
0x808fedb: ADD ESP, 0x10
0x808fede: CMP byte ptr [EBX + 0x4], 0x3
0x808fee2: MOV dword ptr [ESI + 0x8], EAX
0x808fee5: JNZ 0x0808fd41
0x808feeb: NOP
0x808feec: SUB ESP, 0xc
0x808feef: PUSH dword ptr [EBX + 0x8]
0x808fef2: CALL 0x080a36e4
0x808fef7: ADD ESP, 0x10
0x808fefa: MOV dword ptr [EBX + 0x8], EAX
0x808fefd: MOV byte ptr [EBX + 0x4], 0x2
0x808ff01: JMP 0x0808fd41
0x808ff08: MOV EDX, dword ptr [EBP + -0x14]
0x808ff0b: MOV EAX, dword ptr [EDX + 0x8]
0x808ff0e: TEST byte ptr [EBP + -0x10], 0x1
0x808ff12: MOV ESI, dword ptr [EAX + 0x4]
0x808ff15: JZ 0x0808ff27
0x808ff17: TEST EBX, EBX
0x808ff19: MOV EAX, 0x1
0x808ff1e: CMOVLE EBX, EAX
0x808ff21: MOV EAX, ESI
0x808ff23: SUB EAX, EBX
0x808ff25: MOV EBX, EAX
0x808ff27: TEST byte ptr [EBP + -0x10], 0x2
0x808ff2b: JZ 0x0808ff3d
0x808ff2d: TEST EDI, EDI
0x808ff2f: MOV EAX, 0x1
0x808ff34: CMOVLE EDI, EAX
0x808ff37: MOV EDX, ESI
0x808ff39: SUB EDX, EDI
0x808ff3b: MOV EDI, EDX
0x808ff3d: SUB ESP, 0xc
0x808ff40: PUSH 0x2
0x808ff42: CALL 0x08091ab0
0x808ff47: TEST EDI, EDI
0x808ff49: POP EAX
0x808ff4a: MOV EAX, 0x0
0x808ff4f: CMOVS EDI, EAX
0x808ff52: LEA EAX, [ESI + -0x1]
0x808ff55: CMP ESI, EBX
0x808ff57: CMOVLE EBX, EAX
0x808ff5a: CMP EDI, EBX
0x808ff5c: LEA EAX, [EBX + 0x1]
0x808ff5f: CMOVG EDI, EAX
0x808ff62: XOR EAX, EAX
0x808ff64: CMP EDI, EBX
0x808ff66: JG 0x0808ff6d
0x808ff68: MOV EAX, EBX
0x808ff6a: SUB EAX, EDI
0x808ff6c: INC EAX
0x808ff6d: PUSH EAX
0x808ff6e: CALL 0x0809b1d8
0x808ff73: SUB EBX, EDI
0x808ff75: MOV dword ptr [EBP + -0x18], EAX
0x808ff78: LEA EAX, [EBX + 0x1]
0x808ff7b: XOR ESI, ESI
0x808ff7d: ADD ESP, 0x10
0x808ff80: CMP EAX, 0x0
0x808ff83: JLE 0x0808ffb0
0x808ff85: MOV EBX, EAX
0x808ff87: NOP
0x808ff88: MOV EDX, dword ptr [EBP + -0x14]
0x808ff8b: MOV EAX, dword ptr [EDX + 0x8]
0x808ff8e: SUB ESP, 0x8
0x808ff91: MOV EAX, dword ptr [EAX + 0xc]
0x808ff94: LEA EDX, [EDI + ESI*0x1]
0x808ff97: PUSH dword ptr [EAX + EDX*0x4]
0x808ff9a: MOV EDX, dword ptr [EBP + -0x18]
0x808ff9d: MOV EAX, dword ptr [EDX + 0xc]
0x808ffa0: PUSH dword ptr [EAX + ESI*0x4]
0x808ffa3: INC ESI
0x808ffa4: CALL 0x0808be24
0x808ffa9: ADD ESP, 0x10
0x808ffac: CMP EBX, ESI
0x808ffae: JG 0x0808ff88
0x808ffb0: MOV EAX, dword ptr [EBP + -0x14]
0x808ffb3: SUB ESP, 0xc
0x808ffb6: PUSH dword ptr [EAX + 0x8]
0x808ffb9: CALL 0x0809b350
0x808ffbe: MOV EAX, dword ptr [EBP + -0x18]
0x808ffc1: MOV EDX, dword ptr [EBP + -0x14]
0x808ffc4: MOV dword ptr [EDX + 0x8], EAX
0x808ffc7: ADD ESP, 0x10
0x808ffca: LEA ESP, [EBP + -0xc]
0x808ffcd: POP EBX
0x808ffce: POP ESI
0x808ffcf: POP EDI
0x808ffd0: LEAVE
0x808ffd1: RET
0x808ffd2: SUB EBX, EDI
0x808ffd4: INC EBX
0x808ffd5: SUB ESP, 0xc
0x808ffd8: PUSH EBX
0x808ffd9: CALL 0x0808ac84
0x808ffde: MOV ESI, EAX
0x808ffe0: ADD ESP, 0xc
0x808ffe3: MOV EAX, dword ptr [EAX + 0xc]
0x808ffe6: MOV dword ptr [EBP + -0x1c], EAX
0x808ffe9: PUSH EBX
0x808ffea: MOV EDX, dword ptr [EBP + -0x14]
0x808ffed: MOV EAX, dword ptr [EDX + 0x8]
0x808fff0: ADD EDI, dword ptr [EAX + 0xc]
0x808fff3: PUSH EDI
0x808fff4: PUSH dword ptr [EBP + -0x1c]
0x808fff7: CALL 0x0804ba94
0x808fffc: MOV EAX, dword ptr [EBP + -0x1c]
0x808ffff: MOV byte ptr [EAX + EBX*0x1], 0x0
0x8090003: MOV dword ptr [ESI + 0x10], EBX
0x8090006: MOV dword ptr [ESP], 0x3
0x809000d: CALL 0x08091ab0
0x8090012: MOV dword ptr [EBP + 0x8], ESI
0x8090015: ADD ESP, 0x10
0x8090018: LEA ESP, [EBP + -0xc]
0x809001b: POP EBX
0x809001c: POP ESI
0x809001d: POP EDI
0x809001e: LEAVE
0x809001f: JMP 0x080907c0
0x8090024: MOV EDX, dword ptr [EBP + -0x14]
0x8090027: MOV EAX, dword ptr [EDX + 0x8]
0x809002a: ADD EDI, dword ptr [EAX + 0x10]
0x809002d: PUSH EAX
0x809002e: PUSH ESI
0x809002f: PUSH EDI
0x8090030: PUSH dword ptr [EBX + 0x10]
0x8090033: CALL 0x0804b7e4
0x8090038: ADD ESP, 0x10
0x809003b: JMP 0x0808fe9c
0x8090040: SUB ESP, 0xc
0x8090043: PUSH 0x82adac8
0x8090048: CALL 0x08097b24
0x809004d: MOV EDX, dword ptr [EBP + -0x14]
0x8090050: ADD ESP, 0x10
0x8090053: MOV AL, byte ptr [EDX + 0x4]
0x8090056: JMP 0x0808fd87
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
- Ghidra Listing reports a terminal call at 0x808fe1b to 0x8090644 vm_pushString (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808feb3 to 0x8090dd8 vm_pushBuffer (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x809001f to 0x80907c0 vm_pushAllocedString (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushString / analyze_function

Evidence: `ev_0cbe07368fa3318b29bc62f0f3acdbc9e208359ce6a1ee8a5ec3cdbc689bc8e5`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80906a1", "start": "0x8090644"}, {"end": "0x80906b5", "start": "0x80906a4"}], "span_bytes": 114, "total_bytes": 112}`

```c

/* WARNING: Unknown calling convention */

void vm_pushString(char *str)

{
  char cVar1;
  Vm_String_T *pVVar2;
  uint uVar3;
  char *pcVar4;
  
                    /* Unresolved local var: Vm_String_T * newStr@[DW_OP_reg0(EAX)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  uVar3 = 0xffffffff;
  pcVar4 = str;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar1 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar1 != '\0');
  pVVar2 = vm_addSharedString(str,~uVar3 - 1);
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x04';
  vm_stack[vm_sp].v.string = pVVar2;
  return;
}


```

```text
0x8090644: PUSH EBP
0x8090645: MOV EBP, ESP
0x8090647: PUSH EDI
0x8090648: PUSH EAX
0x8090649: MOV EDI, dword ptr [0x086619cc]
0x809064f: TEST EDI, EDI
0x8090651: JZ 0x080906a4
0x8090653: MOV EDI, dword ptr [EBP + 0x8]
0x8090656: CLD
0x8090657: XOR EAX, EAX
0x8090659: MOV ECX, 0xffffffff
0x809065e: SCASB.REPNE ES:EDI
0x8090660: NOT ECX
0x8090662: SUB ESP, 0x8
0x8090665: DEC ECX
0x8090666: PUSH ECX
0x8090667: PUSH dword ptr [EBP + 0x8]
0x809066a: CALL 0x080bd42c
0x809066f: MOV EDX, dword ptr [0x086619cc]
0x8090675: DEC EDX
0x8090676: MOV dword ptr [0x086619cc], EDX
0x809067c: MOV ECX, dword ptr [0x08661964]
0x8090682: LEA EDX, [EDX + EDX*0x2]
0x8090685: MOV byte ptr [ECX + EDX*0x8 + 0x4], 0x4
0x809068a: MOV EDX, dword ptr [0x086619cc]
0x8090690: MOV ECX, dword ptr [0x08661964]
0x8090696: LEA EDX, [EDX + EDX*0x2]
0x8090699: MOV dword ptr [ECX + EDX*0x8 + 0x8], EAX
0x809069d: MOV EDI, dword ptr [EBP + -0x4]
0x80906a0: LEAVE
0x80906a1: RET
0x80906a4: SUB ESP, 0xc
0x80906a7: PUSH 0x82ad98c
0x80906ac: CALL 0x08097b24
0x80906b1: ADD ESP, 0x10
0x80906b4: JMP 0x08090653
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

## vm_allocateEmptyArray / analyze_function

Evidence: `ev_1201043d206cd1e999e6ed53fa0ac56132abe3d37e49ed65c881077f49a146be`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x809b220", "start": "0x809b1d8"}, {"end": "0x809b275", "start": "0x809b224"}, {"end": "0x809b2e2", "start": "0x809b278"}], "span_bytes": 267, "total_bytes": 262}`

```c

Vm_Array_T * vm_allocateEmptyArray(uint size)

{
  Vm_Value_T *pVVar1;
  uint uVar2;
  Vm_Array_T *ptr;
  
                    /* Unresolved local var: Vm_Array_T * pArray@[???] */
                    /* Unresolved local var: Vm_Array_T * pArray@[???]
                       Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg2(EDX)] */
  if (size == 0) {
    _nullArray.ref = _nullArray.ref + 1;
    ptr = &_nullArray;
  }
  else {
    if (0x1000000 < size) goto LAB_0809b25d;
    uVar2 = 4;
    if (3 < size) {
      uVar2 = size;
    }
    ptr = pdb_allocateMemory(uVar2 * 4 + 0x10,"VM","/home/cc/xxxx/gameshell/src/vm/vm_array.c",100);
    if (ptr == (Vm_Array_T *)0x0) goto LAB_0809b25d;
    ptr->maxSize = uVar2;
    ptr->ref = 1;
    ptr->size = 0;
    totalArraySize = totalArraySize + uVar2;
    totalArrayNum = totalArrayNum + 1;
    ptr->pValues = (Vm_Value_T **)(ptr + 1);
  }
  uVar2 = ptr->size;
  while (uVar2 < size) {
    pVVar1 = vm_safeNewValue();
    uVar2 = ptr->size;
    ptr->pValues[uVar2] = pVVar1;
    if (pVVar1 == (Vm_Value_T *)0x0) break;
    uVar2 = uVar2 + 1;
    ptr->size = uVar2;
  }
  if (uVar2 == size) {
    if (ptr != (Vm_Array_T *)0x0) {
      return ptr;
    }
  }
  else {
    while( true ) {
      uVar2 = uVar2 - 1;
      ptr->size = uVar2;
      if (uVar2 == 0xffffffff) break;
      vm_unrefValue(ptr->pValues[uVar2]);
      uVar2 = ptr->size;
    }
    pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_array.c",400);
  }
LAB_0809b25d:
  vm_error("Not enough memory to create array.\n");
  return (Vm_Array_T *)0x0;
}


```

```text
0x809b1d8: PUSH EBP
0x809b1d9: MOV EBP, ESP
0x809b1db: PUSH ESI
0x809b1dc: PUSH EBX
0x809b1dd: MOV ESI, dword ptr [EBP + 0x8]
0x809b1e0: TEST ESI, ESI
0x809b1e2: JNZ 0x0809b278
0x809b1e8: INC dword ptr [0x083cc660]
0x809b1ee: MOV EBX, 0x83cc660
0x809b1f3: MOV ECX, dword ptr [EBX + 0x4]
0x809b1f6: CMP ECX, ESI
0x809b1f8: JNC 0x0809b216
0x809b1fa: MOV ESI, ESI
0x809b1fc: CALL 0x08089f78
0x809b201: TEST EAX, EAX
0x809b203: MOV ECX, dword ptr [EBX + 0x4]
0x809b206: MOV EDX, dword ptr [EBX + 0xc]
0x809b209: MOV dword ptr [EDX + ECX*0x4], EAX
0x809b20c: JZ 0x0809b216
0x809b20e: INC ECX
0x809b20f: MOV dword ptr [EBX + 0x4], ECX
0x809b212: CMP ECX, ESI
0x809b214: JC 0x0809b1fc
0x809b216: CMP ECX, ESI
0x809b218: JZ 0x0809b2d6
0x809b21e: DEC ECX
0x809b21f: JMP 0x0809b23b
0x809b224: SUB ESP, 0xc
0x809b227: MOV EAX, dword ptr [EBX + 0xc]
0x809b22a: PUSH dword ptr [EAX + ECX*0x4]
0x809b22d: CALL 0x0808bfb4
0x809b232: MOV EAX, dword ptr [EBX + 0x4]
0x809b235: DEC EAX
0x809b236: ADD ESP, 0x10
0x809b239: MOV ECX, EAX
0x809b23b: CMP ECX, -0x1
0x809b23e: MOV dword ptr [EBX + 0x4], ECX
0x809b241: JNZ 0x0809b224
0x809b243: PUSH 0x190
0x809b248: PUSH 0x82af390
0x809b24d: PUSH 0x82a376b
0x809b252: PUSH EBX
0x809b253: CALL 0x080ceb94
0x809b258: ADD ESP, 0x10
0x809b25b: XOR EBX, EBX
0x809b25d: SUB ESP, 0xc
0x809b260: PUSH 0x82af40c
0x809b265: CALL 0x08097b24
0x809b26a: ADD ESP, 0x10
0x809b26d: LEA ESP, [EBP + -0x8]
0x809b270: MOV EAX, EBX
0x809b272: POP EBX
0x809b273: POP ESI
0x809b274: LEAVE
0x809b275: RET
0x809b278: CMP ESI, 0x1000000
0x809b27e: JA 0x0809b25b
0x809b280: PUSH 0x64
0x809b282: CMP ESI, 0x3
0x809b285: MOV EBX, 0x4
0x809b28a: CMOVA EBX, ESI
0x809b28d: PUSH 0x82af390
0x809b292: PUSH 0x82a376b
0x809b297: LEA EAX, [EBX*0x4 + 0x10]
0x809b29e: PUSH EAX
0x809b29f: CALL 0x080ce784
0x809b2a4: ADD ESP, 0x10
0x809b2a7: TEST EAX, EAX
0x809b2a9: MOV EDX, EAX
0x809b2ab: JZ 0x0809b25b
0x809b2ad: MOV dword ptr [EAX + 0x8], EBX
0x809b2b0: MOV dword ptr [EAX], 0x1
0x809b2b6: MOV dword ptr [EAX + 0x4], 0x0
0x809b2bd: LEA EAX, [EAX + 0x10]
0x809b2c0: ADD dword ptr [0x086041a0], EBX
0x809b2c6: INC dword ptr [0x086041a4]
0x809b2cc: MOV EBX, EDX
0x809b2ce: MOV dword ptr [EDX + 0xc], EAX
0x809b2d1: JMP 0x0809b1f3
0x809b2d6: TEST EBX, EBX
0x809b2d8: JZ 0x0809b25d
0x809b2da: LEA ESP, [EBP + -0x8]
0x809b2dd: MOV EAX, EBX
0x809b2df: POP EBX
0x809b2e0: POP ESI
0x809b2e1: LEAVE
0x809b2e2: RET
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

## vm_pushAllocedString / analyze_function

Evidence: `ev_39f814faf035f624e318c788e59d9428f4863be775f5ddbf21602b881d0ef8d1`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80907f9", "start": "0x80907c0"}, {"end": "0x8090812", "start": "0x80907fc"}], "span_bytes": 83, "total_bytes": 81}`

```c

void vm_pushAllocedString(Vm_String_T *pString)

{
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x04';
  vm_stack[vm_sp].v.string = pString;
  return;
}


```

```text
0x80907c0: PUSH EBP
0x80907c1: MOV EBP, ESP
0x80907c3: SUB ESP, 0x8
0x80907c6: MOV EAX, [0x086619cc]
0x80907cb: TEST EAX, EAX
0x80907cd: JZ 0x080907fc
0x80907cf: DEC EAX
0x80907d0: MOV [0x086619cc], EAX
0x80907d5: MOV EDX, dword ptr [0x08661964]
0x80907db: LEA EAX, [EAX + EAX*0x2]
0x80907de: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x4
0x80907e3: MOV EAX, [0x086619cc]
0x80907e8: MOV ECX, dword ptr [0x08661964]
0x80907ee: MOV EDX, dword ptr [EBP + 0x8]
0x80907f1: LEA EAX, [EAX + EAX*0x2]
0x80907f4: MOV dword ptr [ECX + EAX*0x8 + 0x8], EDX
0x80907f8: LEAVE
0x80907f9: RET
0x80907fc: SUB ESP, 0xc
0x80907ff: PUSH 0x82ad98c
0x8090804: CALL 0x08097b24
0x8090809: ADD ESP, 0x10
0x809080c: MOV EAX, [0x086619cc]
0x8090811: JMP 0x080907cf
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

## vm_popNStack / analyze_function

Evidence: `ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x8091aee", "start": "0x8091ab0"}], "span_bytes": 63, "total_bytes": 63}`

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

## vm_unrefArray / procedure_pseudo_code

Evidence: `ev_3a18d97f17b48e830607f072dd18b5dcc20f00748883431276baba280c0ab592`

```text

void vm_unrefArray(Vm_Array_T *pArray)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = pArray->ref + -1;
  pArray->ref = iVar1;
  if (0 < iVar1) {
    return;
  }
  uVar2 = pArray->size - 1;
  pArray->size = uVar2;
  while (uVar2 != 0xffffffff) {
    while (pArray->pValues[uVar2] == (Vm_Value_T *)0x0) {
      uVar2 = uVar2 - 1;
      pArray->size = uVar2;
      if (uVar2 == 0xffffffff) goto LAB_0809b3ac;
    }
    vm_unrefValue(pArray->pValues[uVar2]);
    uVar2 = pArray->size - 1;
    pArray->size = uVar2;
  }
LAB_0809b3ac:
  totalArraySize = totalArraySize - pArray->maxSize;
  totalArrayNum = totalArrayNum + -1;
  pdb_freeMemory(pArray,"VM","/home/cc/xxxx/gameshell/src/vm/vm_array.c",0x1ac);
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

## vm_queryRangePtr / procedure_pseudo_code

Evidence: `ev_632b1f05dbc861527e3897578f8fd248be43de594040f3bc2a718cdf982e9969`

```text

/* WARNING: Unknown calling convention */

void vm_queryRangePtr(int flag)

{
  vm_error("Can\'t modify range of a value.\n");
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
- Ghidra Listing reports a terminal call at 0x8090067 to 0x8097b24 vm_error (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_unrefArray / procedure_assembly

Evidence: `ev_8000dac56416a0239afddc630d10d5c425e7d9797f623755e77dcda7e5516c26`

```text
0x809b350: PUSH EBP
0x809b351: MOV EBP, ESP
0x809b353: PUSH EBX
0x809b354: PUSH EBX
0x809b355: MOV EBX, dword ptr [EBP + 0x8]
0x809b358: MOV EAX, dword ptr [EBX]
0x809b35a: DEC EAX
0x809b35b: TEST EAX, EAX
0x809b35d: MOV dword ptr [EBX], EAX
0x809b35f: JLE 0x0809b368
0x809b361: MOV EBX, dword ptr [EBP + -0x4]
0x809b364: LEAVE
0x809b365: RET
0x809b368: MOV EAX, dword ptr [EBX + 0x4]
0x809b36b: DEC EAX
0x809b36c: CMP EAX, -0x1
0x809b36f: MOV dword ptr [EBX + 0x4], EAX
0x809b372: JZ 0x0809b3ac
0x809b374: MOV EDX, EAX
0x809b376: JMP 0x0809b385
0x809b378: LEA EAX, [EDX + -0x1]
0x809b37b: CMP EAX, -0x1
0x809b37e: MOV EDX, EAX
0x809b380: MOV dword ptr [EBX + 0x4], EAX
0x809b383: JZ 0x0809b3ac
0x809b385: MOV EAX, dword ptr [EBX + 0xc]
0x809b388: MOV EAX, dword ptr [EAX + EDX*0x4]
0x809b38b: TEST EAX, EAX
0x809b38d: JZ 0x0809b378
0x809b38f: SUB ESP, 0xc
0x809b392: PUSH EAX
0x809b393: CALL 0x0808bfb4
0x809b398: MOV EDX, dword ptr [EBX + 0x4]
0x809b39b: LEA EAX, [EDX + -0x1]
0x809b39e: ADD ESP, 0x10
0x809b3a1: CMP EAX, -0x1
0x809b3a4: MOV EDX, EAX
0x809b3a6: MOV dword ptr [EBX + 0x4], EAX
0x809b3a9: JNZ 0x0809b385
0x809b3ab: NOP
0x809b3ac: MOV EAX, [0x086041a0]
0x809b3b1: SUB EAX, dword ptr [EBX + 0x8]
0x809b3b4: DEC dword ptr [0x086041a4]
0x809b3ba: MOV [0x086041a0], EAX
0x809b3bf: PUSH 0x1ac
0x809b3c4: PUSH 0x82af390
0x809b3c9: PUSH 0x82a376b
0x809b3ce: PUSH EBX
0x809b3cf: CALL 0x080ceb94
0x809b3d4: ADD ESP, 0x10
0x809b3d7: MOV EBX, dword ptr [EBP + -0x4]
0x809b3da: LEAVE
0x809b3db: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_queryRangePtr / procedure_assembly

Evidence: `ev_ee4e18126002b60c36b3820be4291ff2ef01d3d9fbf8ab2c06dea8f0d219da69`

```text
0x809005c: PUSH EBP
0x809005d: MOV EBP, ESP
0x809005f: MOV dword ptr [EBP + 0x8], 0x82adaf8
0x8090066: LEAVE
0x8090067: JMP 0x08097b24
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x8090067 to 0x8097b24 vm_error (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## _vm_ftol / procedure_assembly

Evidence: `ev_88d9c6fb8cd9b4aae9e9dc4111b5a73b49b6386c133cac64b776a807a4e43f23`

```text
0x80a36e4: PUSH EBP
0x80a36e5: MOV EBP, ESP
0x80a36e7: CVTTSS2SI EAX, dword ptr [EBP + 0x8]
0x80a36ec: LEAVE
0x80a36ed: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## _vm_ftol / procedure_pseudo_code

Evidence: `ev_b38e85b15e5070aab25a097fc476ce2d7f1b68c914db56179e8a6ffdcc35c277`

```text

/* WARNING: Unknown calling convention */

int _vm_ftol(float x)

{
  return (int)x;
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
