# 阶段2R：Ghidra原始证据索引

全部记录针对主ELF SHA-256 `5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新记录使用REA MCP / Ghidra 12.1.4。仅读取原始制品，导出后已关闭会话。伪代码是反编译结果，非原始源码，union字段名和恢复签名可能失真。完整limitations、函数体范围、汇编和交叉引用保留在压缩包中。

| 包 | 操作/选择器 | 地址 | Evidence ID |
|---|---|---|---|
| rea.json.gz | analyze_function / vm_condJmpForLoop | 0x80bcdc8 | `ev_2d0596b38377275abb8795f88716adc8a7d78bec1ef3c3ad3b34ba14eb6df030` |
| rea.json.gz | analyze_function / vm_caseDefault | 0x8097584 | `ev_2dce3eed228dccaaf1d1e67dc9a39c962724b0af9866210b68ecba6102befc83` |
| rea.json.gz | analyze_function / vm_cmpAndJmpInLoop | 0x80bcae0 | `ev_2f7c7986819c8b0acae9e1cddb895fded61ac3d31a2c036489d87c95592ebcce` |
| rea.json.gz | analyze_function / vm_initForLoop | 0x80bc8f0 | `ev_6493fcdaf5956b81b156c8fbb9cf5987c216a53b856561a1e6351de2ad42af92` |
| rea.json.gz | analyze_function / vm_caseCheckString | 0x80974fc | `ev_64fb48253351112793bb3171542d795d713e0d2879a2cf900617f8c6f7873a9e` |
| rea.json.gz | analyze_function / vm_copyValue | 0x808be24 | `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` |
| rea.json.gz | analyze_function / vm_caseCheckNumber | 0x80974c0 | `ev_77292c326eb595efcfe5597edb1f65f303464f58775e3435dac02ee78a721f7f` |
| rea.json.gz | analyze_function / vm_freeLoopValue | 0x80bc884 | `ev_7c730bb3ca6a220df55353b514f3cfe7e4d79e6347771e672ae459058017f177` |
| rea.json.gz | analyze_function / vm_isSameString | 0x808c8b8 | `ev_aae7e66b585e97e76f5365bf919a10c74ddeca801427441bf6bece82930403e9` |
| rea.json.gz | analyze_function / vm_takeOffValueFromStack | 0x8092008 | `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535` |
| rea.json.gz | analyze_function / vm_popNStack | 0x8091ab0 | `ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4` |
| rea.json.gz | analyze_function / vm_stepJmpForLoop | 0x80bcdb0 | `ev_e744e3e38e0be2946efa9fff437a287f867a4093ee99c36e5c78ddfbdad478bd` |
| reused.json.gz | procedure_pseudo_code / vm_regularProcessor | — | `ev_5673fa5b2edb4d5bf79e9f9431a44467493506747573d429692a62ef74c6c5de` |
| reused.json.gz | procedure_assembly / vm_regularProcessor | — | `ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597` |
| reused.json.gz | procedure_pseudo_code / vm_returnFromFun | — | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` |
| reused.json.gz | procedure_assembly / vm_returnFromFun | — | `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` |
| reused.json.gz | procedure_pseudo_code / vm_cleanupValue | — | `ev_6eecd8e699df5eefd7a3a4825b0b2c554b2844c7bf1228db97aa0803c3d8900e` |

## 新记录的完整伪代码与解码汇编

### vm_condJmpForLoop @ 0x80bcdc8

Evidence: `ev_2d0596b38377275abb8795f88716adc8a7d78bec1ef3c3ad3b34ba14eb6df030`

```c
void vm_condJmpForLoop(int breakOffset,int varNo)

{
  vm_cmpAndJmpInLoop(varNo,0,breakOffset);
  return;
}
```

```asm
0x80bcdc8: PUSH EBP
0x80bcdc9: MOV EBP, ESP
0x80bcdcb: MOV EAX, dword ptr [EBP + 0xc]
0x80bcdce: XOR EDX, EDX
0x80bcdd0: LEAVE
0x80bcdd1: JMP 0x080bcae0
```

### vm_caseDefault @ 0x8097584

Evidence: `ev_2dce3eed228dccaaf1d1e67dc9a39c962724b0af9866210b68ecba6102befc83`

```c
void vm_caseDefault(int offset)

{
  vm_popStack();
  vm_currentContext->ip = vm_currentContext->ip + offset;
  return;
}
```

```asm
0x8097584: PUSH EBP
0x8097585: MOV EBP, ESP
0x8097587: SUB ESP, 0x8
0x809758a: CALL 0x0809061c
0x809758f: MOV EDX, dword ptr [0x086619d8]
0x8097595: MOV EAX, dword ptr [EBP + 0x8]
0x8097598: ADD dword ptr [EDX + 0xc], EAX
0x809759b: LEAVE
0x809759c: RET
```

### vm_cmpAndJmpInLoop @ 0x80bcae0

Evidence: `ev_2f7c7986819c8b0acae9e1cddb895fded61ac3d31a2c036489d87c95592ebcce`

```c
/* WARNING: Removing unreachable block (ram,0x080bcd6b) */

void __regparm2 vm_cmpAndJmpInLoop(int varNo,int continueOffset,int breakOffset)

{
  uint uVar1;
  Vm_Value_T *pValue;
  uint uVar2;
  uint uVar3;
  Vm_Program_T *pVVar4;
  int iVar5;
  uint uVar6;
  Vm_Fun_Ptr *pVVar7;
  Vm_Value *pVVar8;
  undefined4 *puVar9;
  Vm_Object *local_1c;
  int to;
  int continueOffset_local;
  
                    /* Unresolved local var: Vm_Loop_Ctrl_T * pCtrl@[???]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mapping_T * m@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mapping_Node_T * pElt@[DW_OP_reg6(ESI)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)]
                       Unresolved local var: int step@[DW_OP_reg3(EBX)]
                       Unresolved local var: int hash@[DW_OP_reg1(ECX)]
                       Unresolved local var: int subIndex@[DW_OP_reg2(EDX)]
                       Unresolved local var: int found@[???] */
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)]
                       Unresolved local var: int index@[DW_OP_reg0(EAX)] */
  iVar5 = vm_stackSize;
  if (vm_funCp != 0) {
    iVar5 = vm_funContext[vm_funCp].bp;
  }
  iVar5 = iVar5 + varNo;
  if ((iVar5 < 0) || (vm_stackSize <= iVar5)) {
    local_1c = (Vm_Object *)0x0;
  }
  else {
    local_1c = (Vm_Object *)0x0;
    if ((vm_stack[iVar5].type == '\x14') &&
       (local_1c = vm_stack[iVar5].v.object, local_1c != (Vm_Object *)0x0)) goto LAB_080bcb21;
  }
  vm_error("Bad loop ctrl, failed to loop (in step).\n");
LAB_080bcb21:
  uVar1 = local_1c->oid;
  pValue = (Vm_Value_T *)local_1c->execRef;
  if (uVar1 != 1) {
    if (1 < (int)uVar1) {
      if (uVar1 == 0x385) {
        uVar1 = *(uint *)&local_1c->initStep;
        uVar3 = local_1c->euid;
        *(uint *)&local_1c->initStep = uVar1 + 1;
        if (uVar1 < *(uint *)(uVar3 + 4)) {
          vm_currentContext->ip = vm_currentContext->ip + continueOffset;
          vm_cleanupValue(pValue);
          vm_copyValue(pValue,*(Vm_Value_T **)(*(int *)(uVar3 + 0xc) + uVar1 * 4));
          pValue->attrib = pValue->attrib | (byte)local_1c->uid;
          return;
        }
      }
      else {
        if (uVar1 != 0x386) {
          return;
        }
        pVVar4 = local_1c->pProgram;
        local_1c->pProgram = (Vm_Program_T *)((int)&pVVar4->ref + 1);
        pVVar8 = local_1c->pBindValue;
        iVar5 = *(int *)&local_1c->initStep;
        local_1c->pBindValue = (Vm_Value *)((int)&pVVar8->ref + 1);
        while (iVar5 <= *(int *)(local_1c->euid + 8)) {
          for (puVar9 = *(undefined4 **)(*(int *)(local_1c->euid + 0x14) + iVar5 * 4);
              puVar9 != (undefined4 *)0x0; puVar9 = (undefined4 *)*puVar9) {
            if ((int)pVVar8 < 1) {
              if (puVar9 != (undefined4 *)0x0) {
                if ((pValue->type != '\x06') ||
                   (pVVar7 = (pValue->v).function, iVar5._0_1_ = pVVar7->type,
                   iVar5._1_1_ = pVVar7->isGlobal, iVar5._2_2_ = pVVar7->argn, iVar5 != 3)) {
                  vm_cleanupValue(pValue);
                  pValue->attrib = pValue->attrib | (byte)local_1c->uid;
                  pVVar7 = (Vm_Fun_Ptr *)vm_allocateEmptyArray(3);
                  pValue->type = '\x06';
                  (pValue->v).function = pVVar7;
                }
                vm_cleanupValue((Vm_Value_T *)pVVar7->pObject->ref);
                vm_copyValue((Vm_Value_T *)pVVar7->pObject->ref,(Vm_Value_T *)(puVar9 + 1));
                vm_cleanupValue((Vm_Value_T *)pVVar7->pObject->execRef);
                vm_copyValue((Vm_Value_T *)pVVar7->pObject->execRef,(Vm_Value_T *)(puVar9 + 7));
                vm_cleanupValue((Vm_Value_T *)pVVar7->pObject->oid);
                *(undefined1 *)(pVVar7->pObject->oid + 4) = 2;
                *(Vm_Program_T **)(pVVar7->pObject->oid + 8) = pVVar4;
                vm_currentContext->ip = vm_currentContext->ip + continueOffset;
                return;
              }
              break;
            }
            pVVar8 = (Vm_Value *)((int)&pVVar8[-1].pPrev + 3);
          }
          iVar5 = iVar5 + 1;
          *(int *)&local_1c->initStep = iVar5;
          pVVar8 = (Vm_Value *)0x0;
          local_1c->pBindValue = (Vm_Value *)0x1;
        }
      }
      vm_currentContext->ip = vm_currentContext->ip + breakOffset;
      return;
    }
    if (uVar1 != 0xffffffff) {
      return;
    }
  }
  if (pValue->type != '\x02') {
    vm_cleanupValue(pValue);
    pValue->type = '\x02';
  }
  uVar3 = local_1c->euid;
  uVar2 = *(uint *)&local_1c->initStep;
  uVar6 = uVar3;
  if (uVar1 == 0) {
    vm_error("Bad step value, should not be zero.\n");
    uVar6 = local_1c->euid;
  }
  local_1c->euid = uVar6 + uVar1;
  (pValue->v).number = uVar3;
  if (((local_1c->flags == 0) && (((int)uVar1 < 1 || ((int)uVar3 <= (int)uVar2)))) &&
     ((-1 < (int)uVar1 || ((int)uVar2 <= (int)uVar3)))) {
    breakOffset = continueOffset;
  }
  vm_currentContext->ip = vm_currentContext->ip + breakOffset;
  if (uVar3 == uVar2) {
    local_1c->flags = 1;
  }
  return;
}
```

```asm
0x80bcae0: PUSH EBP
0x80bcae1: MOV EBP, ESP
0x80bcae3: PUSH EDI
0x80bcae4: PUSH ESI
0x80bcae5: PUSH EBX
0x80bcae6: SUB ESP, 0x1c
0x80bcae9: MOV ECX, EAX
0x80bcaeb: MOV EAX, [0x086619c0]
0x80bcaf0: TEST EAX, EAX
0x80bcaf2: MOV dword ptr [EBP + -0x10], EDX
0x80bcaf5: JNZ 0x080bcb54
0x80bcaf7: MOV EAX, [0x08661960]
0x80bcafc: ADD EAX, ECX
0x80bcafe: TEST EAX, EAX
0x80bcb00: JS 0x080bcb0a
0x80bcb02: CMP EAX, dword ptr [0x08661960]
0x80bcb08: JL 0x080bcb64
0x80bcb0a: MOV dword ptr [EBP + -0x18], 0x0
0x80bcb11: SUB ESP, 0xc
0x80bcb14: PUSH 0x82b29f8
0x80bcb19: CALL 0x08097b24
0x80bcb1e: ADD ESP, 0x10
0x80bcb21: MOV EAX, dword ptr [EBP + -0x18]
0x80bcb24: MOV EBX, dword ptr [EAX + 0x8]
0x80bcb27: CMP EBX, 0x1
0x80bcb2a: MOV EDI, dword ptr [EAX + 0x4]
0x80bcb2d: JZ 0x080bcb90
0x80bcb2f: JLE 0x080bcb88
0x80bcb31: CMP EBX, 0x385
0x80bcb37: JZ 0x080bcc10
0x80bcb3d: CMP EBX, 0x386
0x80bcb43: JZ 0x080bcc3c
0x80bcb49: LEA ESP, [EBP + -0xc]
0x80bcb4c: POP EBX
0x80bcb4d: POP ESI
0x80bcb4e: POP EDI
0x80bcb4f: LEAVE
0x80bcb50: RET
0x80bcb54: LEA EDX, [EAX + EAX*0x4]
0x80bcb57: MOV EAX, [0x086619c8]
0x80bcb5c: MOV EAX, dword ptr [EAX + EDX*0x8]
0x80bcb5f: JMP 0x080bcafc
0x80bcb64: LEA EDX, [EAX + EAX*0x2]
0x80bcb67: MOV EAX, [0x08661964]
0x80bcb6c: LEA EAX, [EAX + EDX*0x8]
0x80bcb6f: MOV dword ptr [EBP + -0x18], 0x0
0x80bcb76: CMP byte ptr [EAX + 0x4], 0x14
0x80bcb7a: JNZ 0x080bcb11
0x80bcb7c: MOV EAX, dword ptr [EAX + 0x8]
0x80bcb7f: TEST EAX, EAX
0x80bcb81: MOV dword ptr [EBP + -0x18], EAX
0x80bcb84: JNZ 0x080bcb21
0x80bcb86: JMP 0x080bcb11
0x80bcb88: CMP EBX, -0x1
0x80bcb8b: JNZ 0x080bcb49
0x80bcb8d: LEA ESI, [ESI]
0x80bcb90: CMP byte ptr [EDI + 0x4], 0x2
0x80bcb94: JZ 0x080bcba6
0x80bcb96: SUB ESP, 0xc
0x80bcb99: PUSH EDI
0x80bcb9a: CALL 0x0808ad98
0x80bcb9f: MOV byte ptr [EDI + 0x4], 0x2
0x80bcba3: ADD ESP, 0x10
0x80bcba6: MOV EDX, dword ptr [EBP + -0x18]
0x80bcba9: MOV ESI, dword ptr [EDX + 0x14]
0x80bcbac: TEST EBX, EBX
0x80bcbae: MOV EDX, dword ptr [EDX + 0x18]
0x80bcbb1: MOV EAX, ESI
0x80bcbb3: MOV dword ptr [EBP + -0x14], EDX
0x80bcbb6: JZ 0x080bcd34
0x80bcbbc: MOV EDX, dword ptr [EBP + -0x18]
0x80bcbbf: ADD EAX, EBX
0x80bcbc1: MOV dword ptr [EDX + 0x14], EAX
0x80bcbc4: MOV dword ptr [EDI + 0x8], ESI
0x80bcbc7: MOV ECX, dword ptr [EDX + 0xc]
0x80bcbca: TEST ECX, ECX
0x80bcbcc: JNZ 0x080bcc04
0x80bcbce: TEST EBX, EBX
0x80bcbd0: JLE 0x080bcbd7
0x80bcbd2: CMP ESI, dword ptr [EBP + -0x14]
0x80bcbd5: JG 0x080bcc04
0x80bcbd7: TEST EBX, EBX
0x80bcbd9: JS 0x080bcbff
0x80bcbdb: MOV EAX, [0x086619d8]
0x80bcbe0: MOV EDX, dword ptr [EBP + -0x10]
0x80bcbe3: NOP
0x80bcbe4: ADD dword ptr [EAX + 0xc], EDX
0x80bcbe7: CMP ESI, dword ptr [EBP + -0x14]
0x80bcbea: JNZ 0x080bcb49
0x80bcbf0: MOV EAX, dword ptr [EBP + -0x18]
0x80bcbf3: MOV dword ptr [EAX + 0xc], 0x1
0x80bcbfa: JMP 0x080bcb49
0x80bcbff: CMP ESI, dword ptr [EBP + -0x14]
0x80bcc02: JGE 0x080bcbdb
0x80bcc04: MOV EAX, [0x086619d8]
0x80bcc09: MOV EDX, dword ptr [EBP + 0x8]
0x80bcc0c: JMP 0x080bcbe4
0x80bcc10: MOV EDX, dword ptr [EBP + -0x18]
0x80bcc13: MOV ESI, dword ptr [EDX + 0x18]
0x80bcc16: LEA EAX, [ESI + 0x1]
0x80bcc19: MOV EBX, dword ptr [EDX + 0x14]
0x80bcc1c: MOV dword ptr [EDX + 0x18], EAX
0x80bcc1f: CMP ESI, dword ptr [EBX + 0x4]
0x80bcc22: JC 0x080bcd70
0x80bcc28: MOV EAX, [0x086619d8]
0x80bcc2d: MOV EDX, dword ptr [EBP + 0x8]
0x80bcc30: ADD dword ptr [EAX + 0xc], EDX
0x80bcc33: LEA ESP, [EBP + -0xc]
0x80bcc36: POP EBX
0x80bcc37: POP ESI
0x80bcc38: POP EDI
0x80bcc39: LEAVE
0x80bcc3a: RET
0x80bcc3c: MOV EAX, dword ptr [EBP + -0x18]
0x80bcc3f: MOV EAX, dword ptr [EAX + 0x20]
0x80bcc42: MOV dword ptr [EBP + -0x1c], EAX
0x80bcc45: MOV EDX, dword ptr [EBP + -0x18]
0x80bcc48: INC EAX
0x80bcc49: MOV dword ptr [EDX + 0x20], EAX
0x80bcc4c: MOV EAX, dword ptr [EDX + 0x1c]
0x80bcc4f: MOV EBX, dword ptr [EDX + 0x14]
0x80bcc52: MOV ECX, dword ptr [EDX + 0x18]
0x80bcc55: MOV dword ptr [EBP + -0x24], EAX
0x80bcc58: INC EAX
0x80bcc59: MOV dword ptr [EDX + 0x1c], EAX
0x80bcc5c: MOV EDX, dword ptr [EBP + -0x24]
0x80bcc5f: CMP dword ptr [EBX + 0x8], ECX
0x80bcc62: JL 0x080bcda0
0x80bcc68: MOV EAX, dword ptr [EBX + 0x14]
0x80bcc6b: MOV ESI, dword ptr [EAX + ECX*0x4]
0x80bcc6e: JMP 0x080bcc72
0x80bcc70: MOV ESI, dword ptr [ESI]
0x80bcc72: TEST ESI, ESI
0x80bcc74: JZ 0x080bcd50
0x80bcc7a: MOV EAX, EDX
0x80bcc7c: DEC EDX
0x80bcc7d: TEST EAX, EAX
0x80bcc7f: JG 0x080bcc70
0x80bcc81: TEST ESI, ESI
0x80bcc83: JZ 0x080bcd50
0x80bcc89: CMP byte ptr [EDI + 0x4], 0x6
0x80bcc8d: JNZ 0x080bcc98
0x80bcc8f: MOV EBX, dword ptr [EDI + 0x8]
0x80bcc92: CMP dword ptr [EBX + 0x4], 0x3
0x80bcc96: JZ 0x080bccc2
0x80bcc98: SUB ESP, 0xc
0x80bcc9b: PUSH EDI
0x80bcc9c: CALL 0x0808ad98
0x80bcca1: MOV EDX, dword ptr [EBP + -0x18]
0x80bcca4: MOV AL, byte ptr [EDX + 0x10]
0x80bcca7: OR byte ptr [EDI + 0x5], AL
0x80bccaa: MOV dword ptr [ESP], 0x3
0x80bccb1: CALL 0x0809b1d8
0x80bccb6: MOV EBX, EAX
0x80bccb8: MOV byte ptr [EDI + 0x4], 0x6
0x80bccbc: MOV dword ptr [EDI + 0x8], EAX
0x80bccbf: ADD ESP, 0x10
0x80bccc2: SUB ESP, 0xc
0x80bccc5: MOV EAX, dword ptr [EBX + 0xc]
0x80bccc8: PUSH dword ptr [EAX]
0x80bccca: CALL 0x0808ad98
0x80bcccf: POP EDI
0x80bccd0: POP EAX
0x80bccd1: LEA EAX, [ESI + 0x4]
0x80bccd4: PUSH EAX
0x80bccd5: MOV EAX, dword ptr [EBX + 0xc]
0x80bccd8: PUSH dword ptr [EAX]
0x80bccda: CALL 0x0808be24
0x80bccdf: POP ECX
0x80bcce0: MOV EAX, dword ptr [EBX + 0xc]
0x80bcce3: PUSH dword ptr [EAX + 0x4]
0x80bcce6: CALL 0x0808ad98
0x80bcceb: POP EAX
0x80bccec: POP EDX
0x80bcced: LEA EAX, [ESI + 0x1c]
0x80bccf0: PUSH EAX
0x80bccf1: MOV EAX, dword ptr [EBX + 0xc]
0x80bccf4: PUSH dword ptr [EAX + 0x4]
0x80bccf7: CALL 0x0808be24
0x80bccfc: POP EAX
0x80bccfd: MOV EAX, dword ptr [EBX + 0xc]
0x80bcd00: PUSH dword ptr [EAX + 0x8]
0x80bcd03: CALL 0x0808ad98
0x80bcd08: MOV EAX, dword ptr [EBX + 0xc]
0x80bcd0b: MOV EAX, dword ptr [EAX + 0x8]
0x80bcd0e: MOV byte ptr [EAX + 0x4], 0x2
0x80bcd12: MOV EAX, dword ptr [EBX + 0xc]
0x80bcd15: MOV EAX, dword ptr [EAX + 0x8]
0x80bcd18: MOV EDX, dword ptr [EBP + -0x1c]
0x80bcd1b: MOV dword ptr [EAX + 0x8], EDX
0x80bcd1e: MOV EDX, dword ptr [EBP + -0x10]
0x80bcd21: MOV EAX, [0x086619d8]
0x80bcd26: ADD dword ptr [EAX + 0xc], EDX
0x80bcd29: ADD ESP, 0x10
0x80bcd2c: LEA ESP, [EBP + -0xc]
0x80bcd2f: POP EBX
0x80bcd30: POP ESI
0x80bcd31: POP EDI
0x80bcd32: LEAVE
0x80bcd33: RET
0x80bcd34: SUB ESP, 0xc
0x80bcd37: PUSH 0x82b2a24
0x80bcd3c: CALL 0x08097b24
0x80bcd41: MOV EDX, dword ptr [EBP + -0x18]
0x80bcd44: ADD ESP, 0x10
0x80bcd47: MOV EAX, dword ptr [EDX + 0x14]
0x80bcd4a: JMP 0x080bcbbc
0x80bcd50: MOV EDX, dword ptr [EBP + -0x18]
0x80bcd53: INC ECX
0x80bcd54: MOV dword ptr [EDX + 0x18], ECX
0x80bcd57: XOR EDX, EDX
0x80bcd59: MOV EAX, dword ptr [EBP + -0x18]
0x80bcd5c: TEST EDX, EDX
0x80bcd5e: MOV dword ptr [EAX + 0x1c], 0x1
0x80bcd65: JZ 0x080bcc5f
0x80bcd6b: JMP 0x080bcc89
0x80bcd70: MOV EAX, [0x086619d8]
0x80bcd75: MOV EDX, dword ptr [EBP + -0x10]
0x80bcd78: SUB ESP, 0xc
0x80bcd7b: ADD dword ptr [EAX + 0xc], EDX
0x80bcd7e: PUSH EDI
0x80bcd7f: CALL 0x0808ad98
0x80bcd84: POP EAX
0x80bcd85: POP EDX
0x80bcd86: MOV EAX, dword ptr [EBX + 0xc]
0x80bcd89: PUSH dword ptr [EAX + ESI*0x4]
0x80bcd8c: PUSH EDI
0x80bcd8d: CALL 0x0808be24
0x80bcd92: MOV EDX, dword ptr [EBP + -0x18]
0x80bcd95: MOV AL, byte ptr [EDX + 0x10]
0x80bcd98: OR byte ptr [EDI + 0x5], AL
0x80bcd9b: ADD ESP, 0x10
0x80bcd9e: JMP 0x080bcd2c
0x80bcda0: XOR EDX, EDX
0x80bcda2: TEST EDX, EDX
0x80bcda4: JNZ 0x080bcc89
0x80bcdaa: JMP 0x080bcc28
```

### vm_initForLoop @ 0x80bc8f0

Evidence: `ev_6493fcdaf5956b81b156c8fbb9cf5987c216a53b856561a1e6351de2ad42af92`

```c
/* WARNING: Unknown calling convention */

void vm_initForLoop(int varNo)

{
  Vm_Value_T *pVVar1;
  uint uVar2;
  int iVar3;
  Vm_Value *pVVar4;
  Vm_Object *pVVar5;
  Vm_Object *pVVar6;
  char *str;
  
                    /* Unresolved local var: Vm_Loop_Ctrl_T * pCtrl@[???]
                       Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Value_T * pValue2@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue3@[???] */
                    /* Unresolved local var: Vm_Loop_Ctrl_T * pCtrl@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg7(EDI)]
                       Unresolved local var: int index@[DW_OP_reg0(EAX)] */
  iVar3 = vm_stackSize;
  if (vm_funCp != 0) {
    iVar3 = vm_funContext[vm_funCp].bp;
  }
  iVar3 = iVar3 + varNo;
  if ((iVar3 < 0) || (vm_stackSize <= iVar3)) {
    pVVar5 = (Vm_Object *)0x0;
LAB_080bc91e:
    vm_error("Bad loop ctrl, failed to loop (in cond).\n");
  }
  else {
    pVVar1 = vm_stack + iVar3;
    vm_cleanupValue(pVVar1);
    pVVar5 = pdb_allocateMemory(0x24,"VM","/home/cc/xxxx/gameshell/src/vm/vm_loop.c",0x10a);
    if (pVVar5 == (Vm_Object *)0x0) {
      vm_error("Failed to create loop ctrl.\n");
    }
    (pVVar1->v).object = pVVar5;
    pVVar1->type = '\x14';
    pVVar6 = pVVar5;
    for (iVar3 = 9; iVar3 != 0; iVar3 = iVar3 + -1) {
      pVVar6->ref = 0;
      pVVar6 = (Vm_Object *)&pVVar6->execRef;
    }
    pVVar5->ref = 1;
    pVVar5->execRef = 0;
    if (pVVar5 == (Vm_Object *)0x0) goto LAB_080bc91e;
  }
  iVar3 = vm_sp;
  pVVar1 = vm_stack;
  uVar2 = vm_stack[vm_sp].v.number;
  pVVar5->oid = uVar2;
  pVVar4 = pVVar1[iVar3 + 2].v.pPtrValue;
  pVVar5->execRef = (int)pVVar4;
  *(byte *)&pVVar5->uid = pVVar1[iVar3 + 1].attrib & 1;
  if (uVar2 == 1) {
LAB_080bca18:
    if (pVVar1[iVar3 + 1].type != '\x02') {
      vm_error("Bad value to loop, expected integer (in init).\n");
      pVVar4 = (Vm_Value *)pVVar5->execRef;
    }
    *(anon_union_4_11_5000f750_for_v *)&pVVar5->initStep = pVVar1[iVar3 + 1].v;
    if (pVVar4->type != '\x02') {
      vm_error("Bad init value to loop, expected integer (in init).\n");
      pVVar4 = (Vm_Value *)pVVar5->execRef;
    }
    pVVar5->euid = (uint)pVVar4->v;
    vm_popNStack(3);
    return;
  }
  if ((int)uVar2 < 2) {
    if (uVar2 == 0xffffffff) goto LAB_080bca18;
LAB_080bc972:
    str = "Bad step type for loop.\n";
  }
  else {
    if (uVar2 != 0x3e5) goto LAB_080bc972;
    if (pVVar1[iVar3 + 1].type == '\x06') {
      pVVar5->oid = 0x385;
      pVVar5->euid = (uint)pVVar1[iVar3 + 1].v;
      (pVVar1[iVar3 + 1].v.string)->ref = (pVVar1[iVar3 + 1].v.string)->ref + 1;
      *(undefined4 *)&pVVar5->initStep = 0;
      goto LAB_080bc982;
    }
    if (pVVar1[iVar3 + 1].type == '\a') {
      pVVar5->oid = 0x386;
      pVVar5->euid = (uint)pVVar1[iVar3 + 1].v;
      (pVVar1[iVar3 + 1].v.string)->ref = (pVVar1[iVar3 + 1].v.string)->ref + 1;
      *(undefined4 *)&pVVar5->initStep = 0;
      pVVar5->pBindValue = (Vm_Value *)0x0;
      pVVar5->pProgram = (Vm_Program_T *)0x0;
      goto LAB_080bc982;
    }
    str = "Bad type of value to loop in, expected mapping or array.\n";
  }
  vm_error(str);
LAB_080bc982:
  vm_popNStack(3);
  return;
}
```

```asm
0x80bc8f0: PUSH EBP
0x80bc8f1: MOV EBP, ESP
0x80bc8f3: PUSH EDI
0x80bc8f4: PUSH EBX
0x80bc8f5: MOV EAX, [0x086619c0]
0x80bc8fa: TEST EAX, EAX
0x80bc8fc: MOV ECX, dword ptr [EBP + 0x8]
0x80bc8ff: JNZ 0x080bc994
0x80bc905: MOV EAX, [0x08661960]
0x80bc90a: ADD EAX, ECX
0x80bc90c: TEST EAX, EAX
0x80bc90e: JS 0x080bc91c
0x80bc910: CMP EAX, dword ptr [0x08661960]
0x80bc916: JL 0x080bc9a4
0x80bc91c: XOR EDI, EDI
0x80bc91e: SUB ESP, 0xc
0x80bc921: PUSH 0x82b2928
0x80bc926: CALL 0x08097b24
0x80bc92b: ADD ESP, 0x10
0x80bc92e: MOV EAX, [0x086619cc]
0x80bc933: MOV EDX, dword ptr [0x08661964]
0x80bc939: LEA EAX, [EAX + EAX*0x2]
0x80bc93c: LEA EAX, [EDX + EAX*0x8]
0x80bc93f: MOV EDX, dword ptr [EAX + 0x8]
0x80bc942: MOV dword ptr [EDI + 0x8], EDX
0x80bc945: MOV ECX, dword ptr [EAX + 0x38]
0x80bc948: LEA EBX, [EAX + 0x18]
0x80bc94b: MOV dword ptr [EDI + 0x4], ECX
0x80bc94e: MOV AL, byte ptr [EBX + 0x5]
0x80bc951: AND EAX, 0x1
0x80bc954: CMP EDX, 0x1
0x80bc957: MOV byte ptr [EDI + 0x10], AL
0x80bc95a: JZ 0x080bca18
0x80bc960: JLE 0x080bca10
0x80bc966: CMP EDX, 0x3e5
0x80bc96c: JZ 0x080bca68
0x80bc972: SUB ESP, 0xc
0x80bc975: PUSH 0x82b28c6
0x80bc97a: CALL 0x08097b24
0x80bc97f: ADD ESP, 0x10
0x80bc982: MOV dword ptr [EBP + 0x8], 0x3
0x80bc989: LEA ESP, [EBP + -0x8]
0x80bc98c: POP EBX
0x80bc98d: POP EDI
0x80bc98e: LEAVE
0x80bc98f: JMP 0x08091ab0
0x80bc994: LEA EDX, [EAX + EAX*0x4]
0x80bc997: MOV EAX, [0x086619c8]
0x80bc99c: MOV EAX, dword ptr [EAX + EDX*0x8]
0x80bc99f: JMP 0x080bc90a
0x80bc9a4: LEA EDX, [EAX + EAX*0x2]
0x80bc9a7: MOV EAX, [0x08661964]
0x80bc9ac: SUB ESP, 0xc
0x80bc9af: LEA EDI, [EAX + EDX*0x8]
0x80bc9b2: PUSH EDI
0x80bc9b3: CALL 0x0808ad98
0x80bc9b8: ADD ESP, 0x10
0x80bc9bb: PUSH 0x10a
0x80bc9c0: PUSH 0x82b28fc
0x80bc9c5: PUSH 0x82a376b
0x80bc9ca: PUSH 0x24
0x80bc9cc: CALL 0x080ce784
0x80bc9d1: ADD ESP, 0x10
0x80bc9d4: TEST EAX, EAX
0x80bc9d6: MOV EBX, EAX
0x80bc9d8: JZ 0x080bcaca
0x80bc9de: MOV dword ptr [EDI + 0x8], EBX
0x80bc9e1: MOV byte ptr [EDI + 0x4], 0x14
0x80bc9e5: XOR EAX, EAX
0x80bc9e7: CLD
0x80bc9e8: MOV ECX, 0x9
0x80bc9ed: MOV EDI, EBX
0x80bc9ef: TEST EBX, EBX
0x80bc9f1: STOSD.REP ES:EDI
0x80bc9f3: MOV dword ptr [EBX], 0x1
0x80bc9f9: MOV dword ptr [EBX + 0x4], 0x0
0x80bca00: MOV EDI, EBX
0x80bca02: JNZ 0x080bc92e
0x80bca08: JMP 0x080bc91e
0x80bca10: INC EDX
0x80bca11: JNZ 0x080bc972
0x80bca17: NOP
0x80bca18: CMP byte ptr [EBX + 0x4], 0x2
0x80bca1c: JZ 0x080bca31
0x80bca1e: SUB ESP, 0xc
0x80bca21: PUSH 0x82b2954
0x80bca26: CALL 0x08097b24
0x80bca2b: ADD ESP, 0x10
0x80bca2e: MOV ECX, dword ptr [EDI + 0x4]
0x80bca31: MOV EAX, dword ptr [EBX + 0x8]
0x80bca34: MOV dword ptr [EDI + 0x18], EAX
0x80bca37: CMP byte ptr [ECX + 0x4], 0x2
0x80bca3b: JZ 0x080bca50
0x80bca3d: SUB ESP, 0xc
0x80bca40: PUSH 0x82b2984
0x80bca45: CALL 0x08097b24
0x80bca4a: ADD ESP, 0x10
0x80bca4d: MOV ECX, dword ptr [EDI + 0x4]
0x80bca50: MOV EAX, dword ptr [ECX + 0x8]
0x80bca53: MOV dword ptr [EDI + 0x14], EAX
0x80bca56: MOV dword ptr [EBP + 0x8], 0x3
0x80bca5d: LEA ESP, [EBP + -0x8]
0x80bca60: POP EBX
0x80bca61: POP EDI
0x80bca62: LEAVE
0x80bca63: JMP 0x08091ab0
0x80bca68: MOV AL, byte ptr [EBX + 0x4]
0x80bca6b: CMP AL, 0x6
0x80bca6d: JZ 0x080bcaac
0x80bca6f: CMP AL, 0x7
0x80bca71: JNZ 0x080bca9f
0x80bca73: MOV dword ptr [EDI + 0x8], 0x386
0x80bca7a: MOV EAX, dword ptr [EBX + 0x8]
0x80bca7d: MOV dword ptr [EDI + 0x14], EAX
0x80bca80: MOV EAX, dword ptr [EBX + 0x8]
0x80bca83: INC dword ptr [EAX]
0x80bca85: MOV dword ptr [EDI + 0x18], 0x0
0x80bca8c: MOV dword ptr [EDI + 0x1c], 0x0
0x80bca93: MOV dword ptr [EDI + 0x20], 0x0
0x80bca9a: JMP 0x080bc982
0x80bca9f: SUB ESP, 0xc
0x80bcaa2: PUSH 0x82b29bc
0x80bcaa7: JMP 0x080bc97a
0x80bcaac: MOV dword ptr [EDI + 0x8], 0x385
0x80bcab3: MOV EAX, dword ptr [EBX + 0x8]
0x80bcab6: MOV dword ptr [EDI + 0x14], EAX
0x80bcab9: MOV EAX, dword ptr [EBX + 0x8]
0x80bcabc: INC dword ptr [EAX]
0x80bcabe: MOV dword ptr [EDI + 0x18], 0x0
0x80bcac5: JMP 0x080bc982
0x80bcaca: SUB ESP, 0xc
0x80bcacd: PUSH 0x82b28df
0x80bcad2: CALL 0x08097b24
0x80bcad7: ADD ESP, 0x10
0x80bcada: JMP 0x080bc9de
```

### vm_caseCheckString @ 0x80974fc

Evidence: `ev_64fb48253351112793bb3171542d795d713e0d2879a2cf900617f8c6f7873a9e`

```c
void vm_caseCheckString(int offset,int n)

{
  uchar uVar1;
  Vm_Program *pVVar2;
  Vm_Value_T *pVVar3;
  int iVar4;
  Vm_String_T *unaff_ESI;
  
  iVar4 = vm_sp;
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_String_T * pString@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pVVar3 = vm_stack;
  pVVar2 = vm_currentContext->consititution->pProgram;
  if (((n < 0) || (pVVar2->maxStringNo <= n)) ||
     (unaff_ESI = pVVar2->ppStrings[n], unaff_ESI == (Vm_String_T *)0x0)) {
    vm_error("Bad shared string %d!\n",n);
    uVar1 = pVVar3[iVar4].type;
  }
  else {
    uVar1 = vm_stack[vm_sp].type;
  }
  if ((uVar1 == '\x04') && (iVar4 = vm_isSameString(pVVar3[iVar4].v.string,unaff_ESI), iVar4 != 0))
  {
    vm_popStack();
    vm_currentContext->ip = vm_currentContext->ip + offset;
  }
  return;
}
```

```asm
0x80974fc: PUSH EBP
0x80974fd: MOV EAX, [0x086619cc]
0x8097502: MOV EBP, ESP
0x8097504: LEA EAX, [EAX + EAX*0x2]
0x8097507: PUSH ESI
0x8097508: MOV EDX, dword ptr [0x08661964]
0x809750e: PUSH EBX
0x809750f: MOV ECX, dword ptr [EBP + 0xc]
0x8097512: LEA EBX, [EDX + EAX*0x8]
0x8097515: MOV EAX, [0x086619d8]
0x809751a: MOV EAX, dword ptr [EAX + 0x14]
0x809751d: TEST ECX, ECX
0x809751f: MOV EDX, dword ptr [EAX]
0x8097521: JS 0x0809752b
0x8097523: MOVSX EAX, word ptr [EDX + 0x24]
0x8097527: CMP EAX, ECX
0x8097529: JG 0x0809754c
0x809752b: SUB ESP, 0x8
0x809752e: PUSH ECX
0x809752f: PUSH 0x82adfaa
0x8097534: CALL 0x08097b24
0x8097539: ADD ESP, 0x10
0x809753c: CMP byte ptr [EBX + 0x4], 0x4
0x8097540: JZ 0x0809755c
0x8097542: LEA ESP, [EBP + -0x8]
0x8097545: POP EBX
0x8097546: POP ESI
0x8097547: LEAVE
0x8097548: RET
0x809754c: MOV EAX, dword ptr [EDX + 0x20]
0x809754f: MOV ESI, dword ptr [EAX + ECX*0x4]
0x8097552: TEST ESI, ESI
0x8097554: JZ 0x0809752b
0x8097556: CMP byte ptr [EBX + 0x4], 0x4
0x809755a: JNZ 0x08097542
0x809755c: SUB ESP, 0x8
0x809755f: PUSH ESI
0x8097560: PUSH dword ptr [EBX + 0x8]
0x8097563: CALL 0x0808c8b8
0x8097568: ADD ESP, 0x10
0x809756b: TEST EAX, EAX
0x809756d: JZ 0x08097542
0x809756f: CALL 0x0809061c
0x8097574: MOV EDX, dword ptr [0x086619d8]
0x809757a: MOV EAX, dword ptr [EBP + 0x8]
0x809757d: ADD dword ptr [EDX + 0xc], EAX
0x8097580: JMP 0x08097542
```

### vm_copyValue @ 0x808be24

Evidence: `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

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

### vm_caseCheckNumber @ 0x80974c0

Evidence: `ev_77292c326eb595efcfe5597edb1f65f303464f58775e3435dac02ee78a721f7f`

```c
void vm_caseCheckNumber(int offset,int number)

{
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg2(EDX)] */
  if ((vm_stack[vm_sp].type == '\x02') && (vm_stack[vm_sp].v.info == (void *)number)) {
    vm_popStack();
    vm_currentContext->ip = vm_currentContext->ip + offset;
    return;
  }
  return;
}
```

```asm
0x80974c0: PUSH EBP
0x80974c1: MOV EBP, ESP
0x80974c3: SUB ESP, 0x8
0x80974c6: MOV EAX, [0x086619cc]
0x80974cb: LEA EAX, [EAX + EAX*0x2]
0x80974ce: MOV EDX, dword ptr [0x08661964]
0x80974d4: LEA EDX, [EDX + EAX*0x8]
0x80974d7: CMP byte ptr [EDX + 0x4], 0x2
0x80974db: JZ 0x080974e0
0x80974dd: LEAVE
0x80974de: RET
0x80974e0: MOV EAX, dword ptr [EBP + 0xc]
0x80974e3: CMP dword ptr [EDX + 0x8], EAX
0x80974e6: JNZ 0x080974dd
0x80974e8: CALL 0x0809061c
0x80974ed: MOV EDX, dword ptr [0x086619d8]
0x80974f3: MOV EAX, dword ptr [EBP + 0x8]
0x80974f6: ADD dword ptr [EDX + 0xc], EAX
0x80974f9: LEAVE
0x80974fa: RET
```

### vm_freeLoopValue @ 0x80bc884

Evidence: `ev_7c730bb3ca6a220df55353b514f3cfe7e4d79e6347771e672ae459058017f177`

```c
void vm_freeLoopValue(Vm_Loop_Ctrl_T *pLoopCtrl)

{
  if (pLoopCtrl->step == 0x385) {
    vm_unrefArray((pLoopCtrl->u).array.pArray);
  }
  else {
    if (pLoopCtrl->step != 0x386) {
      pdb_freeMemory(pLoopCtrl,"VM","/home/cc/xxxx/gameshell/src/vm/vm_loop.c",0x8d);
      return;
    }
    vm_unrefMapping((Vm_Mapping_T *)(pLoopCtrl->u).array.pArray);
  }
  pdb_freeMemory(pLoopCtrl,"VM","/home/cc/xxxx/gameshell/src/vm/vm_loop.c",0x8d);
  return;
}
```

```asm
0x80bc884: PUSH EBP
0x80bc885: MOV EBP, ESP
0x80bc887: PUSH EBX
0x80bc888: PUSH EAX
0x80bc889: MOV EBX, dword ptr [EBP + 0x8]
0x80bc88c: MOV EAX, dword ptr [EBX + 0x8]
0x80bc88f: CMP EAX, 0x385
0x80bc894: JZ 0x080bc8b8
0x80bc896: CMP EAX, 0x386
0x80bc89b: JZ 0x080bc8e0
0x80bc89d: PUSH 0x8d
0x80bc8a2: PUSH 0x82b28fc
0x80bc8a7: PUSH 0x82a376b
0x80bc8ac: PUSH EBX
0x80bc8ad: CALL 0x080ceb94
0x80bc8b2: MOV EBX, dword ptr [EBP + -0x4]
0x80bc8b5: LEAVE
0x80bc8b6: RET
0x80bc8b8: SUB ESP, 0xc
0x80bc8bb: PUSH dword ptr [EBX + 0x14]
0x80bc8be: CALL 0x0809b350
0x80bc8c3: ADD ESP, 0x10
0x80bc8c6: PUSH 0x8d
0x80bc8cb: PUSH 0x82b28fc
0x80bc8d0: PUSH 0x82a376b
0x80bc8d5: PUSH EBX
0x80bc8d6: CALL 0x080ceb94
0x80bc8db: MOV EBX, dword ptr [EBP + -0x4]
0x80bc8de: LEAVE
0x80bc8df: RET
0x80bc8e0: SUB ESP, 0xc
0x80bc8e3: PUSH dword ptr [EBX + 0x14]
0x80bc8e6: CALL 0x0809c488
0x80bc8eb: ADD ESP, 0x10
0x80bc8ee: JMP 0x080bc8c6
```

### vm_isSameString @ 0x808c8b8

Evidence: `ev_aae7e66b585e97e76f5365bf919a10c74ddeca801427441bf6bece82930403e9`

```c
int vm_isSameString(Vm_String_T *s1,Vm_String_T *s2)

{
  uint uVar1;
  int iVar2;
  char *pcVar3;
  char *pcVar4;
  bool bVar5;
  
  iVar2 = 1;
  if (s1 != s2) {
    uVar1 = s1->len;
    iVar2 = 0;
    if ((uVar1 == s2->len) && ((s2->attrib & s1->attrib & 1) == 0)) {
      bVar5 = true;
      pcVar3 = s1->ptr;
      pcVar4 = s2->ptr;
      do {
        if (uVar1 == 0) break;
        uVar1 = uVar1 - 1;
        bVar5 = *pcVar3 == *pcVar4;
        pcVar3 = pcVar3 + 1;
        pcVar4 = pcVar4 + 1;
      } while (bVar5);
      return (uint)bVar5;
    }
  }
  return iVar2;
}
```

```asm
0x808c8b8: PUSH EBP
0x808c8b9: MOV EBP, ESP
0x808c8bb: PUSH EDI
0x808c8bc: PUSH ESI
0x808c8bd: SUB ESP, 0x8
0x808c8c0: MOV ESI, dword ptr [EBP + 0x8]
0x808c8c3: MOV EDI, dword ptr [EBP + 0xc]
0x808c8c6: CMP ESI, EDI
0x808c8c8: MOV EDX, 0x1
0x808c8cd: JZ 0x0808c8d9
0x808c8cf: MOV ECX, dword ptr [ESI + 0x10]
0x808c8d2: XOR DL, DL
0x808c8d4: CMP ECX, dword ptr [EDI + 0x10]
0x808c8d7: JZ 0x0808c8e4
0x808c8d9: ADD ESP, 0x8
0x808c8dc: POP ESI
0x808c8dd: MOV EAX, EDX
0x808c8df: POP EDI
0x808c8e0: LEAVE
0x808c8e1: RET
0x808c8e4: MOV EAX, dword ptr [EDI + 0x4]
0x808c8e7: AND EAX, dword ptr [ESI + 0x4]
0x808c8ea: TEST AL, 0x1
0x808c8ec: JNZ 0x0808c8d9
0x808c8ee: MOV ESI, dword ptr [ESI + 0xc]
0x808c8f1: MOV EDI, dword ptr [EDI + 0xc]
0x808c8f4: CLD
0x808c8f5: CMP ECX, ECX
0x808c8f7: CMPSB.REPE ES:EDI, ESI
0x808c8f9: SETZ AL
0x808c8fc: ADD ESP, 0x8
0x808c8ff: POP ESI
0x808c900: MOVZX EDX, AL
0x808c903: MOV EAX, EDX
0x808c905: POP EDI
0x808c906: LEAVE
0x808c907: RET
```

### vm_takeOffValueFromStack @ 0x8092008

Evidence: `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535`

```c
void vm_takeOffValueFromStack(int spb,int spe)

{
  anon_union_4_11_5000f750_for_v *paVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  iVar3 = vm_sp;
  if (spb <= spe) {
    iVar5 = spb + vm_sp;
    iVar6 = spe + vm_sp;
    if ((vm_sp <= iVar5) && (iVar6 < vm_stackSize)) {
      if (iVar5 <= iVar6) {
        iVar3 = iVar5 * 0x18;
        iVar4 = iVar5;
        do {
          iVar4 = iVar4 + 1;
          vm_cleanupValue((Vm_Value_T *)((int)&vm_stack->ref + iVar3));
          iVar3 = iVar3 + 0x18;
        } while (iVar4 <= iVar6);
      }
      iVar5 = iVar5 + -1;
      iVar3 = iVar6 + 1;
      if (vm_sp <= iVar5) {
        iVar6 = iVar5 * 0x18;
        iVar4 = iVar3 * 0x18;
        do {
          *(uchar *)(iVar4 + -0x14 + (int)vm_stack) = (&vm_stack->type)[iVar6];
          *(uchar *)(iVar4 + -0x13 + (int)vm_stack) = (&vm_stack->attrib)[iVar6];
          pVVar2 = vm_stack;
          paVar1 = &vm_stack->v;
          (&vm_stack->type)[iVar6] = '\0';
          *(undefined4 *)(iVar4 + -0x10 + (int)pVVar2) = *(undefined4 *)((int)paVar1 + iVar6);
          (&vm_stack->attrib)[iVar6] = '\0';
          iVar5 = iVar5 + -1;
          iVar3 = iVar3 + -1;
          iVar6 = iVar6 + -0x18;
          iVar4 = iVar4 + -0x18;
        } while (vm_sp <= iVar5);
      }
    }
  }
  vm_sp = iVar3;
  return;
}
```

```asm
0x8092008: PUSH EBP
0x8092009: MOV EBP, ESP
0x809200b: PUSH EDI
0x809200c: PUSH ESI
0x809200d: PUSH EBX
0x809200e: SUB ESP, 0xc
0x8092011: MOV EDI, dword ptr [EBP + 0xc]
0x8092014: CMP dword ptr [EBP + 0x8], EDI
0x8092017: JG 0x080920e0
0x809201d: MOV EAX, [0x086619cc]
0x8092022: ADD dword ptr [EBP + 0x8], EAX
0x8092025: ADD EDI, EAX
0x8092027: CMP dword ptr [EBP + 0x8], EAX
0x809202a: JL 0x080920e0
0x8092030: CMP EDI, dword ptr [0x08661960]
0x8092036: JGE 0x080920e0
0x809203c: MOV ESI, dword ptr [EBP + 0x8]
0x809203f: CMP ESI, EDI
0x8092041: JG 0x08092071
0x8092043: LEA EAX, [ESI + ESI*0x2]
0x8092046: LEA EBX, [EAX*0x8 + 0x0]
0x809204d: LEA ESI, [ESI]
0x8092050: MOV EAX, EBX
0x8092052: SUB ESP, 0xc
0x8092055: ADD EAX, dword ptr [0x08661964]
0x809205b: PUSH EAX
0x809205c: INC ESI
0x809205d: CALL 0x0808ad98
0x8092062: ADD EBX, 0x18
0x8092065: ADD ESP, 0x10
0x8092068: CMP ESI, EDI
0x809206a: JLE 0x08092050
0x809206c: MOV EAX, [0x086619cc]
0x8092071: MOV ESI, dword ptr [EBP + 0x8]
0x8092074: DEC ESI
0x8092075: INC EDI
0x8092076: CMP ESI, EAX
0x8092078: JL 0x080920d8
0x809207a: LEA EAX, [ESI + ESI*0x2]
0x809207d: LEA ECX, [EAX*0x8 + 0x0]
0x8092084: LEA EAX, [EDI + EDI*0x2]
0x8092087: LEA EBX, [EAX*0x8 + 0x0]
0x809208e: MOV ESI, ESI
0x8092090: MOV EDX, dword ptr [0x08661964]
0x8092096: MOV AL, byte ptr [ECX + EDX*0x1 + 0x4]
0x809209a: SUB EBX, 0x18
0x809209d: MOV byte ptr [EBX + EDX*0x1 + 0x4], AL
0x80920a1: MOV EDX, dword ptr [0x08661964]
0x80920a7: MOV AL, byte ptr [ECX + EDX*0x1 + 0x5]
0x80920ab: MOV byte ptr [EBX + EDX*0x1 + 0x5], AL
0x80920af: MOV EAX, [0x08661964]
0x80920b4: MOV EDX, dword ptr [ECX + EAX*0x1 + 0x8]
0x80920b8: MOV byte ptr [ECX + EAX*0x1 + 0x4], 0x0
0x80920bd: MOV dword ptr [EBX + EAX*0x1 + 0x8], EDX
0x80920c1: MOV EAX, [0x08661964]
0x80920c6: MOV byte ptr [ECX + EAX*0x1 + 0x5], 0x0
0x80920cb: DEC ESI
0x80920cc: DEC EDI
0x80920cd: SUB ECX, 0x18
0x80920d0: CMP ESI, dword ptr [0x086619cc]
0x80920d6: JGE 0x08092090
0x80920d8: MOV dword ptr [0x086619cc], EDI
0x80920de: MOV ESI, ESI
0x80920e0: LEA ESP, [EBP + -0xc]
0x80920e3: POP EBX
0x80920e4: POP ESI
0x80920e5: POP EDI
0x80920e6: LEAVE
0x80920e7: RET
```

### vm_popNStack @ 0x8091ab0

Evidence: `ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4`

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

```asm
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

### vm_stepJmpForLoop @ 0x80bcdb0

Evidence: `ev_e744e3e38e0be2946efa9fff437a287f867a4093ee99c36e5c78ddfbdad478bd`

```c
void vm_stepJmpForLoop(int continueOffset,int varNo)

{
  vm_cmpAndJmpInLoop(varNo,continueOffset,0);
  return;
}
```

```asm
0x80bcdb0: PUSH EBP
0x80bcdb1: MOV EBP, ESP
0x80bcdb3: MOV EDX, dword ptr [EBP + 0x8]
0x80bcdb6: MOV EAX, dword ptr [EBP + 0xc]
0x80bcdb9: MOV dword ptr [EBP + 0x8], 0x0
0x80bcdc0: LEAVE
0x80bcdc1: JMP 0x080bcae0
```

