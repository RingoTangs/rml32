# 阶段 2W：复用的数值异常静态证据

目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。Ghidra 12.1.4；仅复用 REA 留存结果，未打开新会话、未运行参考程序。

## vm_divValue / procedure_assembly

Evidence: `ev_19597a6d356a5c98c32c16698ef7be5e4ad2201b7a0a22f7a714cc7918dd6f89`；来源 `docs/vm-core-evidence/rea.json.gz`。

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

## vm_modValue / procedure_pseudo_code

Evidence: `ev_2eb04ee1a9a6e84c698cbb6cc4f66fa7dfb1f53d14b2b3c0bc689e53a5db7d58`；来源 `docs/vm-core-evidence/rea.json.gz`。

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

## vm_modValue / procedure_assembly

Evidence: `ev_4066c4eef4bbbc302618e0bb8a65aeab459dde67f388ba53c0695f2616f8a31d`；来源 `docs/vm-core-evidence/rea.json.gz`。

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

## vm_divValue / procedure_pseudo_code

Evidence: `ev_8ae08d055f9314e0e074d8efe90e725516a1ec2a40a18b7fb1e9bc1014dc355c`；来源 `docs/vm-core-evidence/rea.json.gz`。

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

## vm_opEqValue / procedure_assembly

Evidence: `ev_354a97d72f74e25fe049521f079d6411eef25ea6be56c075ef7da6aab3dbeb4e`；来源 `docs/vm-numeric-update-evidence/rea.json.gz`。

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

## vm_opEqValue / procedure_references

Evidence: `ev_6d7e95f9964d461ce6bced29a53763cfe58a85e342487a05c6989477d1946dd1`；来源 `docs/vm-numeric-update-evidence/rea.json.gz`。

完整引用结果保留在 `reused.json.gz`，不省略间接流和 Ghidra 限制。

## vm_opEqValue / procedure_pseudo_code

Evidence: `ev_983266641ef48cd9821c04963ca8b83351cf78fef7b46f841916bcb58d93139e`；来源 `docs/vm-numeric-update-evidence/rea.json.gz`。

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

## vm_copyValue / procedure_pseudo_code

Evidence: `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33`；来源 `docs/vm-reference-evidence/rea.json.gz`。

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
