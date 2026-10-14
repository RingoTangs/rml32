# 阶段 2F：数值更新 Ghidra 证据

目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。Ghidra 12.1.4，原版未执行。以下伪代码是反编译输出，不是原始源码。联合体/类型推导以汇编为准。

## vm_incValue / procedure_assembly

Evidence: `ev_6038bf11ed6a58f8ba545a48140a3d29d7c02fd3ebdb8b798f1cd24275cb3488`

```asm
0x808dea4: PUSH EBP
0x808dea5: MOV EAX, [0x086619cc]
0x808deaa: MOV EBP, ESP
0x808deac: LEA EAX, [EAX + EAX*0x2]
0x808deaf: PUSH ESI
0x808deb0: MOV EDX, dword ptr [0x08661964]
0x808deb6: PUSH EBX
0x808deb7: LEA ECX, [EDX + EAX*0x8]
0x808deba: MOV EBX, dword ptr [ECX + 0x8]
0x808debd: MOVZX EAX, byte ptr [EBX + 0x4]
0x808dec1: CMP EAX, 0x2
0x808dec4: MOV ESI, dword ptr [EBP + 0x8]
0x808dec7: JZ 0x0808df3c
0x808dec9: CMP EAX, 0x3
0x808decc: JZ 0x0808df44
0x808dece: TEST ESI, ESI
0x808ded0: MOV EDX, 0x82cf251
0x808ded5: PUSH EAX
0x808ded6: MOV EAX, 0x82acf70
0x808dedb: CMOVLE EAX, EDX
0x808dede: PUSH EAX
0x808dedf: PUSH ESI
0x808dee0: MOVZX EAX, byte ptr [ECX + 0x4]
0x808dee4: PUSH EAX
0x808dee5: CALL 0x0808a13c
0x808deea: POP EDX
0x808deeb: POP ECX
0x808deec: PUSH EAX
0x808deed: PUSH 0x82acf73
0x808def2: CALL 0x08097b24
0x808def7: ADD ESP, 0x10
0x808defa: SUB ESP, 0xc
0x808defd: PUSH EBX
0x808defe: CALL 0x0808bfb4
0x808df03: MOV EAX, [0x086619cc]
0x808df08: MOV EDX, dword ptr [0x08661964]
0x808df0e: LEA EAX, [EAX + EAX*0x2]
0x808df11: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x808df16: MOV EAX, [0x086619cc]
0x808df1b: MOV EDX, dword ptr [0x08661964]
0x808df21: LEA EAX, [EAX + EAX*0x2]
0x808df24: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x808df29: INC dword ptr [0x086619cc]
0x808df2f: ADD ESP, 0x10
0x808df32: LEA ESP, [EBP + -0x8]
0x808df35: POP EBX
0x808df36: POP ESI
0x808df37: LEAVE
0x808df38: RET
0x808df3c: ADD dword ptr [EBX + 0x8], ESI
0x808df3f: JMP 0x0808defa
0x808df44: FLD float ptr [EBX + 0x8]
0x808df47: PUSH ESI
0x808df48: FIADD dword ptr [ESP]
0x808df4b: ADD ESP, 0x4
0x808df4e: FSTP float ptr [EBX + 0x8]
0x808df51: JMP 0x0808defa
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-incvalue"></a>

## vm_incValue / procedure_pseudo_code

Evidence: `ev_78885cb7407e1998e0713117740b868e419ff4f1befde50c3575c204ee59a48b`

```c

void vm_incValue(int direction)

{
  Vm_Value_T *pValue;
  uint uVar1;
  undefined *puVar2;
  char *pcVar3;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg3(EBX)] */
  pValue = vm_stack[vm_sp].v.pPtrValue;
  uVar1 = (uint)pValue->type;
  if (uVar1 == 2) {
    (pValue->v).info = (void *)((int)(pValue->v).info + direction);
  }
  else if (uVar1 == 3) {
    (pValue->v).real = (pValue->v).real + (float)direction;
  }
  else {
    puVar2 = &DAT_082acf70;
    if (direction < 1) {
      puVar2 = &DAT_082cf251;
    }
    pcVar3 = vm_valueType((uint)vm_stack[vm_sp].type);
    vm_error("Bad type(%s) to %s.\n",pcVar3,puVar2,uVar1);
  }
  vm_unrefValue(pValue);
  vm_stack[vm_sp].type = '\0';
  vm_stack[vm_sp].attrib = '\0';
  vm_sp = vm_sp + 1;
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

## vm_incValue / procedure_references

Evidence: `ev_fdaae89f79ea8396a974ff6b3e8aec13872147eb429b5580c790128864eb6be5`

以下列出全部已解析 call 引用；完整非 call 边与函数体范围保留在压缩证据。间接调用未解析不能据此排除。

```json
{
  "procedure": "0x808dea4",
  "calls": [
    {
      "source_address": "0x808dee5",
      "target_address": "0x808a13c",
      "target_name": "vm_valueType",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808def2",
      "target_address": "0x8097b24",
      "target_name": "vm_error",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808defe",
      "target_address": "0x808bfb4",
      "target_name": "vm_unrefValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    }
  ],
  "unresolved_calls": []
}
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry, including instruction-interior destinations and embedded-data sources; an enclosing address span does not establish function ownership.
- Reference kinds are direct Ghidra ReferenceManager observations; unresolved computed flows without a target are absent and remain unknown.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_opEqValue / procedure_assembly

Evidence: `ev_354a97d72f74e25fe049521f079d6411eef25ea6be56c075ef7da6aab3dbeb4e`

```asm
0x808eb68: PUSH EBP
0x808eb69: MOV EBP, ESP
0x808eb6b: PUSH EDI
0x808eb6c: PUSH ESI
0x808eb6d: PUSH EBX
0x808eb6e: SUB ESP, 0xc
0x808eb71: MOV EDX, dword ptr [0x086619cc]
0x808eb77: TEST EDX, EDX
0x808eb79: MOV ESI, dword ptr [EBP + 0x8]
0x808eb7c: JLE 0x0808ed24
0x808eb82: MOV EAX, [0x08661964]
0x808eb87: LEA ECX, [EDX + EDX*0x2]
0x808eb8a: LEA ECX, [EAX + ECX*0x8]
0x808eb8d: DEC EDX
0x808eb8e: MOV EBX, dword ptr [ECX + 0x20]
0x808eb91: MOV dword ptr [0x086619cc], EDX
0x808eb97: LEA EDX, [EDX + EDX*0x2]
0x808eb9a: LEA EDX, [EAX + EDX*0x8]
0x808eb9d: MOV AL, byte ptr [ECX + 0x4]
0x808eba0: MOV byte ptr [EDX + 0x4], AL
0x808eba3: MOV AL, byte ptr [ECX + 0x5]
0x808eba6: SUB ESP, 0x8
0x808eba9: MOV byte ptr [EDX + 0x5], AL
0x808ebac: MOV EAX, dword ptr [ECX + 0x8]
0x808ebaf: MOV dword ptr [EDX + 0x8], EAX
0x808ebb2: PUSH EBX
0x808ebb3: PUSH ECX
0x808ebb4: CALL 0x0808be24
0x808ebb9: ADD ESP, 0x10
0x808ebbc: CMP ESI, 0xb
0x808ebbf: JA 0x0808ebc8
0x808ebc1: JMP dword ptr [ESI*0x4 + 0x82adc20]
0x808ebc8: SUB ESP, 0xc
0x808ebcb: PUSH 0x82acfaa
0x808ebd0: CALL 0x080ff80c
0x808ebd5: ADD ESP, 0x10
0x808ebd8: MOV EBX, dword ptr [0x086619cc]
0x808ebde: MOV EAX, [0x08661964]
0x808ebe3: LEA EBX, [EBX + EBX*0x2]
0x808ebe6: LEA EBX, [EAX + EBX*0x8]
0x808ebe9: LEA EDI, [EBX + 0x18]
0x808ebec: MOV ESI, dword ptr [EDI + 0x8]
0x808ebef: SUB ESP, 0xc
0x808ebf2: PUSH ESI
0x808ebf3: CALL 0x0808ad98
0x808ebf8: POP EAX
0x808ebf9: POP EDX
0x808ebfa: PUSH EBX
0x808ebfb: PUSH ESI
0x808ebfc: CALL 0x0808be24
0x808ec01: MOV dword ptr [ESP], ESI
0x808ec04: CALL 0x0808bfb4
0x808ec09: MOV AL, byte ptr [EBX + 0x4]
0x808ec0c: MOV byte ptr [EDI + 0x4], AL
0x808ec0f: MOV AL, byte ptr [EBX + 0x5]
0x808ec12: MOV byte ptr [EDI + 0x5], AL
0x808ec15: MOV EAX, dword ptr [EBX + 0x8]
0x808ec18: MOV dword ptr [EDI + 0x8], EAX
0x808ec1b: MOV EAX, [0x086619cc]
0x808ec20: MOV EDX, dword ptr [0x08661964]
0x808ec26: LEA EAX, [EAX + EAX*0x2]
0x808ec29: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x808ec2e: MOV EAX, [0x086619cc]
0x808ec33: MOV EDX, dword ptr [0x08661964]
0x808ec39: LEA EAX, [EAX + EAX*0x2]
0x808ec3c: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x808ec41: INC dword ptr [0x086619cc]
0x808ec47: ADD ESP, 0x10
0x808ec4a: LEA ESP, [EBP + -0xc]
0x808ec4d: POP EBX
0x808ec4e: POP ESI
0x808ec4f: POP EDI
0x808ec50: LEAVE
0x808ec51: RET
0x808ec52: CALL 0x0808e7bc
0x808ec57: JMP 0x0808ebd8
0x808ec5c: CALL 0x0808e3b8
0x808ec61: JMP 0x0808ebd8
0x808ec66: CALL 0x0808e654
0x808ec6b: JMP 0x0808ebd8
0x808ec70: MOV EAX, [0x086619cc]
0x808ec75: LEA EAX, [EAX + EAX*0x2]
0x808ec78: MOV EDX, dword ptr [0x08661964]
0x808ec7e: LEA ECX, [EDX + EAX*0x8]
0x808ec81: LEA EBX, [ECX + 0x18]
0x808ec84: MOV AL, byte ptr [EBX + 0x4]
0x808ec87: MOVZX EDX, AL
0x808ec8a: CMP EDX, 0x2
0x808ec8d: JZ 0x0808ed65
0x808ec93: CMP EDX, 0x3
0x808ec96: JZ 0x0808ed80
0x808ec9c: SUB ESP, 0xc
0x808ec9f: MOVZX EAX, AL
0x808eca2: PUSH EAX
0x808eca3: CALL 0x0808a13c
0x808eca8: POP ESI
0x808eca9: POP EDI
0x808ecaa: PUSH EAX
0x808ecab: PUSH 0x82ad280
0x808ecb0: CALL 0x08097b24
0x808ecb5: ADD ESP, 0x10
0x808ecb8: CALL 0x0809061c
0x808ecbd: JMP 0x0808ebd8
0x808ecc2: CALL 0x0808e1a4
0x808ecc7: JMP 0x0808ebd8
0x808eccc: CALL 0x0808ea10
0x808ecd1: JMP 0x0808ebd8
0x808ecd6: CALL 0x0808eabc
0x808ecdb: JMP 0x0808ebd8
0x808ece0: MOV EAX, [0x086619cc]
0x808ece5: LEA EAX, [EAX + EAX*0x2]
0x808ece8: MOV EDX, dword ptr [0x08661964]
0x808ecee: LEA EBX, [EDX + EAX*0x8]
0x808ecf1: LEA ESI, [EBX + 0x18]
0x808ecf4: MOVZX EAX, byte ptr [ESI + 0x4]
0x808ecf8: CMP EAX, 0x2
0x808ecfb: JZ 0x0808ed3f
0x808ecfd: SUB ESP, 0xc
0x808ed00: PUSH EAX
0x808ed01: CALL 0x0808a13c
0x808ed06: POP ECX
0x808ed07: POP EBX
0x808ed08: PUSH EAX
0x808ed09: PUSH 0x82ad1c8
0x808ed0e: JMP 0x0808ecb0
0x808ed10: CALL 0x0808e020
0x808ed15: JMP 0x0808ebd8
0x808ed1a: CALL 0x0808df54
0x808ed1f: JMP 0x0808ebd8
0x808ed24: SUB ESP, 0xc
0x808ed27: PUSH 0x82ad74c
0x808ed2c: CALL 0x08097b24
0x808ed31: ADD ESP, 0x10
0x808ed34: MOV EDX, dword ptr [0x086619cc]
0x808ed3a: JMP 0x0808eb82
0x808ed3f: CMP byte ptr [EBX + 0x4], 0x2
0x808ed43: JZ 0x0808ed55
0x808ed45: SUB ESP, 0xc
0x808ed48: PUSH 0x82ad1f8
0x808ed4d: CALL 0x08097b24
0x808ed52: ADD ESP, 0x10
0x808ed55: MOV EAX, dword ptr [EBX + 0x8]
0x808ed58: XOR dword ptr [ESI + 0x8], EAX
0x808ed5b: CALL 0x0809061c
0x808ed60: JMP 0x0808ebd8
0x808ed65: MOVZX EAX, byte ptr [ECX + 0x4]
0x808ed69: CMP EAX, 0x2
0x808ed6c: JZ 0x0808eda1
0x808ed6e: CMP EAX, 0x3
0x808ed71: JZ 0x0808edc0
0x808ed73: SUB ESP, 0xc
0x808ed76: PUSH 0x82ad2b0
0x808ed7b: JMP 0x0808ecb0
0x808ed80: MOVZX EAX, byte ptr [ECX + 0x4]
0x808ed84: CMP EAX, 0x2
0x808ed87: JZ 0x0808edb5
0x808ed89: CMP EAX, 0x3
0x808ed8c: JNZ 0x0808ed73
0x808ed8e: FLD float ptr [ECX + 0x8]
0x808ed91: FMUL float ptr [EBX + 0x8]
0x808ed94: FSTP float ptr [EBX + 0x8]
0x808ed97: CALL 0x0809061c
0x808ed9c: JMP 0x0808ebd8
0x808eda1: MOV EAX, dword ptr [ECX + 0x8]
0x808eda4: IMUL EAX, dword ptr [EBX + 0x8]
0x808eda8: MOV dword ptr [EBX + 0x8], EAX
0x808edab: CALL 0x0809061c
0x808edb0: JMP 0x0808ebd8
0x808edb5: FLD float ptr [EBX + 0x8]
0x808edb8: FIMUL dword ptr [ECX + 0x8]
0x808edbb: FSTP float ptr [EBX + 0x8]
0x808edbe: JMP 0x0808ed97
0x808edc0: FLD float ptr [ECX + 0x8]
0x808edc3: SUB ESP, 0x10
0x808edc6: FIMUL dword ptr [EBX + 0x8]
0x808edc9: FSTP float ptr [ESP]
0x808edcc: CALL 0x080a36e4
0x808edd1: ADD ESP, 0x10
0x808edd4: MOV dword ptr [EBX + 0x8], EAX
0x808edd7: JMP 0x0808edab
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-opeqvalue"></a>

## vm_opEqValue / procedure_pseudo_code

Evidence: `ev_983266641ef48cd9821c04963ca8b83351cf78fef7b46f841916bcb58d93139e`

```c

void vm_opEqValue(int opcode)

{
  Vm_Value_T *pVVar1;
  byte bVar2;
  Vm_Value_T *pVVar3;
  Vm_Value_T *pVVar4;
  int iVar5;
  int iVar6;
  char *str;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Value_T * pValueOfPtr@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg3(EBX)] */
  if (vm_sp < 1) {
    vm_error("Stack overflow when do op_eq.\n");
  }
  pVVar3 = vm_stack;
  pVVar1 = vm_stack + vm_sp;
  iVar6 = vm_sp + -1;
  pVVar4 = (Vm_Value_T *)pVVar1[1].v.string;
  vm_sp = iVar6;
  vm_stack[iVar6].type = pVVar1->type;
  pVVar3[iVar6].attrib = pVVar1->attrib;
  pVVar3[iVar6].v = pVVar1->v;
  vm_copyValue(pVVar1,pVVar4);
  iVar6 = vm_sp;
  pVVar1 = vm_stack;
  switch(opcode) {
  default:
    yyerror("Error operate code.\n");
    break;
  case 2:
    vm_addValue();
    break;
  case 3:
    vm_subValue();
    break;
  case 4:
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
    bVar2 = vm_stack[vm_sp + 1].type;
    if (bVar2 == 2) {
      if (vm_stack[vm_sp].type == '\x02') {
        vm_stack[vm_sp + 1].v.number = vm_stack[vm_sp].v.number * vm_stack[vm_sp + 1].v.number;
      }
      else {
        if (vm_stack[vm_sp].type != '\x03') {
LAB_0808ed73:
          str = "Bad *, right isn\'t a number when left is.\n";
          goto LAB_0808ecb0;
        }
        iVar5 = _vm_ftol(vm_stack[vm_sp].v.real * (float)vm_stack[vm_sp + 1].v.number);
        pVVar1[iVar6 + 1].v.number = iVar5;
      }
      vm_popStack();
      break;
    }
    if (bVar2 == 3) {
      if (vm_stack[vm_sp].type == '\x02') {
        vm_stack[vm_sp + 1].v.real = vm_stack[vm_sp + 1].v.real * (float)vm_stack[vm_sp].v.number;
      }
      else {
        if (vm_stack[vm_sp].type != '\x03') goto LAB_0808ed73;
        vm_stack[vm_sp + 1].v.real = vm_stack[vm_sp].v.real * vm_stack[vm_sp + 1].v.real;
      }
      vm_popStack();
      break;
    }
    pVVar4 = (Vm_Value_T *)vm_valueType((uint)bVar2);
    str = "Bad *, type invalid (got %s) of left value.\n";
    goto LAB_0808ecb0;
  case 5:
    vm_divValue();
    break;
  case 6:
    vm_andValue();
    break;
  case 7:
    vm_orValue();
    break;
  case 8:
    if (vm_stack[vm_sp + 1].type == 2) {
      if (vm_stack[vm_sp].type != '\x02') {
        vm_error("Bad ^, right isn\'t a number when left is.\n");
      }
      pVVar1[iVar6 + 1].v.number = pVVar1[iVar6 + 1].v.number ^ pVVar1[iVar6].v.number;
      vm_popStack();
      break;
    }
    pVVar4 = (Vm_Value_T *)vm_valueType((uint)vm_stack[vm_sp + 1].type);
    str = "Bad ^, type invalid (got %s) of left value.\n";
LAB_0808ecb0:
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???] */
    vm_error(str,pVVar4);
    vm_popStack();
    break;
  case 9:
    vm_rshValue();
    break;
  case 10:
    vm_lshValue();
    break;
  case 0xb:
    vm_modValue();
  }
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg6(ESI)] */
  pVVar1 = vm_stack + vm_sp;
  pVVar4 = (Vm_Value_T *)pVVar1[1].v.string;
  vm_cleanupValue(pVVar4);
  vm_copyValue(pVVar4,pVVar1);
  vm_unrefValue(pVVar4);
  pVVar1[1].type = pVVar1->type;
  pVVar1[1].attrib = pVVar1->attrib;
  pVVar1[1].v = pVVar1->v;
  vm_stack[vm_sp].type = '\0';
  vm_stack[vm_sp].attrib = '\0';
  vm_sp = vm_sp + 1;
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

## vm_opEqValue / procedure_references

Evidence: `ev_6d7e95f9964d461ce6bced29a53763cfe58a85e342487a05c6989477d1946dd1`

以下列出全部已解析 call 引用；完整非 call 边与函数体范围保留在压缩证据。间接调用未解析不能据此排除。

```json
{
  "procedure": "0x808eb68",
  "calls": [
    {
      "source_address": "0x808ebb4",
      "target_address": "0x808be24",
      "target_name": "vm_copyValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ebd0",
      "target_address": "0x80ff80c",
      "target_name": "yyerror",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ebf3",
      "target_address": "0x808ad98",
      "target_name": "vm_cleanupValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ebfc",
      "target_address": "0x808be24",
      "target_name": "vm_copyValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ec04",
      "target_address": "0x808bfb4",
      "target_name": "vm_unrefValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ec52",
      "target_address": "0x808e7bc",
      "target_name": "vm_modValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ec5c",
      "target_address": "0x808e3b8",
      "target_name": "vm_addValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ec66",
      "target_address": "0x808e654",
      "target_name": "vm_subValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808eca3",
      "target_address": "0x808a13c",
      "target_name": "vm_valueType",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ecb0",
      "target_address": "0x8097b24",
      "target_name": "vm_error",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ecb8",
      "target_address": "0x809061c",
      "target_name": "vm_popStack",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ecc2",
      "target_address": "0x808e1a4",
      "target_name": "vm_divValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808eccc",
      "target_address": "0x808ea10",
      "target_name": "vm_andValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ecd6",
      "target_address": "0x808eabc",
      "target_name": "vm_orValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed01",
      "target_address": "0x808a13c",
      "target_name": "vm_valueType",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed10",
      "target_address": "0x808e020",
      "target_name": "vm_rshValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed1a",
      "target_address": "0x808df54",
      "target_name": "vm_lshValue",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed2c",
      "target_address": "0x8097b24",
      "target_name": "vm_error",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed4d",
      "target_address": "0x8097b24",
      "target_name": "vm_error",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed5b",
      "target_address": "0x809061c",
      "target_name": "vm_popStack",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808ed97",
      "target_address": "0x809061c",
      "target_name": "vm_popStack",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808edab",
      "target_address": "0x809061c",
      "target_name": "vm_popStack",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    },
    {
      "source_address": "0x808edcc",
      "target_address": "0x80a36e4",
      "target_name": "_vm_ftol",
      "kind": {
        "available": true,
        "call": true,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_CALL",
        "write": false
      }
    }
  ],
  "unresolved_calls": []
}
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry, including instruction-interior destinations and embedded-data sources; an enclosing address span does not establish function ownership.
- Reference kinds are direct Ghidra ReferenceManager observations; unresolved computed flows without a target are absent and remain unknown.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.
