# Ghidra证据索引与反编译摘录

原始ELF SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新查询使用REA MCP / Ghidra 12.1.4，仅读取原始制品；会话已关闭。伪代码是反编译表示，非原始源码；解码地址、汇编和原始限制保留于压缩包。

## 证据索引

| 包 | ID | 操作 | 选择器 |
|---|---|---|---|
| rea.json.gz | `ev_0f52440a23f0b659faa95e8a973f08f1b228163126ef3957b926b6685f6d9b78` | analyze_function | `vm_pushThisObject` |
| rea.json.gz | `ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215` | analyze_function | `vm_returnFromFun` |
| rea.json.gz | `ev_70d5db7423deaeb10a6ac1f00a2ab189ef9d8620f60f9eeac00dff36a31dcf66` | analyze_function | `vm_callInOtherArray3` |
| rea.json.gz | `ev_7bfea1e46d51d77201c90e9d4e594b0f6791c4bff0c33cc634a3bd46cde37c78` | analyze_function | `0x080cab20` |
| rea.json.gz | `ev_cf79f214bf5abcf504ec8eb0240a0f7d4b5abbc172ccbf5250c9ee9eecc5cfc9` | analyze_function | `vm_callInOtherFunExReturn` |
| rea.json.gz | `ev_dae2362e9944413d4a2410309483bc76294a735723624ba3a5fc2f583c2f99b4` | analyze_function | `vm_callInOtherArray2` |
| reused.json.gz | `ev_13e8fe9368cca520ed154767ba3471e1ac37473f0886ab1b15365d0e8db01662` | analyze_function | `vm_callInOtherFunEx` |
| reused.json.gz | `ev_4b89eec3bfa71f6f357f508600108ff4f54e9492ffa2c51e8012d5d1a8ce8db0` | analyze_function | `vm_callInOtherFun` |
| reused.json.gz | `ev_39ff14e286b302152ec3587aaff3135b507be423df85fb3b0c7651a2f982da7a` | procedure_assembly | `vm_findProgramFunction` |
| reused.json.gz | `ev_6bd6f758aac4e15cc09716b4e96079841eab2a3ccac331ccd49911785170d50a` | procedure_pseudo_code | `vm_lookupObject` |
| reused.json.gz | `ev_ba5b1993ec6d3677bd709e0fb78467636839e28ccd2802f1a214278e6a287928` | procedure_pseudo_code | `vm_findProgramFunction` |
| reused.json.gz | `ev_e0725c9f865c565504a4a69ce6866c428ca0be60f899fe4a27de58ea497a0c7d` | procedure_pseudo_code | `vm_findProgramFunctionByCStr` |
| reused.json.gz | `ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41` | procedure_assembly | `vm_callInUserFunBody` |
| reused.json.gz | `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad` | procedure_pseudo_code | `vm_callInUserFunBody` |
| reused.json.gz | `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2` | procedure_pseudo_code | `vm_returnFromFun` |
| reused.json.gz | `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610` | procedure_assembly | `vm_returnFromFun` |
| reused.json.gz | `ev_5673fa5b2edb4d5bf79e9f9431a44467493506747573d429692a62ef74c6c5de` | procedure_pseudo_code | `vm_regularProcessor` |
| reused.json.gz | `ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597` | procedure_assembly | `vm_regularProcessor` |

## 新查询的完整伪代码与汇编

### vm_pushThisObject @ 0x80975d8

证据：`ev_0f52440a23f0b659faa95e8a973f08f1b228163126ef3957b926b6685f6d9b78`。

```c
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_pushThisObject(void)

{
  if (vm_currentContext->thisObject == (Vm_Object_T *)0x0) {
    vm_pushNumber(0);
    return;
  }
  vm_pushObject(vm_currentContext->thisObject);
  return;
}
```

```asm
0x80975d8: PUSH EBP
0x80975d9: MOV EBP, ESP
0x80975db: SUB ESP, 0x8
0x80975de: MOV EAX, [0x086619d8]
0x80975e3: MOV EAX, dword ptr [EAX + 0x20]
0x80975e6: TEST EAX, EAX
0x80975e8: JNZ 0x080975fc
0x80975ea: SUB ESP, 0xc
0x80975ed: PUSH 0x0
0x80975ef: CALL 0x08090b3c
0x80975f4: ADD ESP, 0x10
0x80975f7: LEAVE
0x80975f8: RET
0x80975fc: SUB ESP, 0xc
0x80975ff: PUSH EAX
0x8097600: CALL 0x080912f4
0x8097605: ADD ESP, 0x10
0x8097608: LEAVE
0x8097609: RET
```

### vm_returnFromFun @ 0x8094978

证据：`ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215`。

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

### vm_callInOtherArray3 @ 0x80bcdec

证据：`ev_70d5db7423deaeb10a6ac1f00a2ab189ef9d8620f60f9eeac00dff36a31dcf66`。

```c
void vm_callInOtherArray3(void *pCookie)

{
  uint uVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  Vm_Value *pVVar4;
  uint uVar5;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pTempValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pResultArray@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pResultIndex@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Combined_Cookie_T combinedCookie@[???]
                       Unresolved local var: int nArgNum@[???]
                       Unresolved local var: int sp@[DW_OP_reg2(EDX)] */
  pVVar2 = vm_stack;
  uVar5 = (uint)pCookie >> 0x14;
  if (vm_sp + 1U != uVar5) {
    vm_error("Bad sp when processing call_other array(3).\n");
  }
  if ((pVVar2[uVar5].type == '\x06') && (pVVar2[uVar5 + 1].type == '\x02')) {
    iVar3 = pVVar2[uVar5 + 1].v.number;
  }
  else {
    vm_error("Bad intermediate values in stack when processing call_other array(3).\n");
    iVar3 = pVVar2[uVar5 + 1].v.number;
  }
  if (iVar3 < 0) {
    vm_error("Bad value of result index when processing call_other array(3).\n");
    iVar3 = pVVar2[uVar5 + 1].v.number;
  }
  if (iVar3 < (pVVar2[uVar5].v.object)->execRef) {
    uVar1 = (pVVar2[uVar5].v.object)->flags;
    pVVar2[uVar5 + 1].v.info = (void *)(iVar3 + 1);
    pVVar2 = *(Vm_Value_T **)(uVar1 + iVar3 * 4);
    pVVar4 = vm_stackTopValue();
    vm_cleanupValue(pVVar2);
    pVVar2->type = pVVar4->type;
    pVVar2->attrib = pVVar4->attrib;
    pVVar2->v = pVVar4->v;
    vm_stack[vm_sp].type = '\0';
    vm_stack[vm_sp].attrib = '\0';
    vm_sp = vm_sp + 1;
    vm_hookCallbackWhenNextAction(vm_callInOtherArray2,pCookie);
    return;
  }
  vm_popStack();
  vm_hookCallbackWhenNextAction(vm_callInOtherArray2,pCookie);
  return;
}
```

```asm
0x80bcdec: PUSH EBP
0x80bcded: MOV EBP, ESP
0x80bcdef: PUSH EDI
0x80bcdf0: PUSH ESI
0x80bcdf1: PUSH EBX
0x80bcdf2: SUB ESP, 0xc
0x80bcdf5: MOV EDI, dword ptr [EBP + 0x8]
0x80bcdf8: MOV EDX, EDI
0x80bcdfa: SHR EDX, 0x14
0x80bcdfd: MOV EAX, [0x08661964]
0x80bce02: LEA ECX, [EDX + EDX*0x2]
0x80bce05: LEA EBX, [EAX + ECX*0x8]
0x80bce08: MOV EAX, [0x086619cc]
0x80bce0d: INC EAX
0x80bce0e: CMP EAX, EDX
0x80bce10: LEA ESI, [EBX + 0x18]
0x80bce13: JZ 0x080bce25
0x80bce15: SUB ESP, 0xc
0x80bce18: PUSH 0x82b2a4c
0x80bce1d: CALL 0x08097b24
0x80bce22: ADD ESP, 0x10
0x80bce25: CMP byte ptr [EBX + 0x4], 0x6
0x80bce29: JZ 0x080bcee4
0x80bce2f: SUB ESP, 0xc
0x80bce32: PUSH 0x82b2a7c
0x80bce37: CALL 0x08097b24
0x80bce3c: MOV EAX, dword ptr [ESI + 0x8]
0x80bce3f: ADD ESP, 0x10
0x80bce42: TEST EAX, EAX
0x80bce44: JS 0x080bcefc
0x80bce4a: MOV EDX, dword ptr [EBX + 0x8]
0x80bce4d: CMP EAX, dword ptr [EDX + 0x4]
0x80bce50: MOV ECX, EAX
0x80bce52: JGE 0x080bcec8
0x80bce54: INC EAX
0x80bce55: MOV EDX, dword ptr [EDX + 0xc]
0x80bce58: MOV dword ptr [ESI + 0x8], EAX
0x80bce5b: MOV ESI, dword ptr [EDX + ECX*0x4]
0x80bce5e: CALL 0x08090604
0x80bce63: SUB ESP, 0xc
0x80bce66: MOV EBX, EAX
0x80bce68: PUSH ESI
0x80bce69: CALL 0x0808ad98
0x80bce6e: MOV AL, byte ptr [EBX + 0x4]
0x80bce71: MOV byte ptr [ESI + 0x4], AL
0x80bce74: MOV AL, byte ptr [EBX + 0x5]
0x80bce77: MOV byte ptr [ESI + 0x5], AL
0x80bce7a: MOV EAX, dword ptr [EBX + 0x8]
0x80bce7d: MOV dword ptr [ESI + 0x8], EAX
0x80bce80: MOV EAX, [0x086619cc]
0x80bce85: MOV EDX, dword ptr [0x08661964]
0x80bce8b: LEA EAX, [EAX + EAX*0x2]
0x80bce8e: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x80bce93: MOV EAX, [0x086619cc]
0x80bce98: MOV EDX, dword ptr [0x08661964]
0x80bce9e: LEA EAX, [EAX + EAX*0x2]
0x80bcea1: ADD ESP, 0x10
0x80bcea4: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x80bcea9: SUB ESP, 0x8
0x80bceac: INC dword ptr [0x086619cc]
0x80bceb2: PUSH EDI
0x80bceb3: PUSH 0x80bcf14
0x80bceb8: CALL 0x080a9a48
0x80bcebd: LEA ESP, [EBP + -0xc]
0x80bcec0: POP EBX
0x80bcec1: POP ESI
0x80bcec2: POP EDI
0x80bcec3: LEAVE
0x80bcec4: RET
0x80bcec8: CALL 0x0809061c
0x80bcecd: SUB ESP, 0x8
0x80bced0: PUSH EDI
0x80bced1: PUSH 0x80bcf14
0x80bced6: CALL 0x080a9a48
0x80bcedb: LEA ESP, [EBP + -0xc]
0x80bcede: POP EBX
0x80bcedf: POP ESI
0x80bcee0: POP EDI
0x80bcee1: LEAVE
0x80bcee2: RET
0x80bcee4: CMP byte ptr [ESI + 0x4], 0x2
0x80bcee8: JNZ 0x080bce2f
0x80bceee: MOV EAX, dword ptr [ESI + 0x8]
0x80bcef1: TEST EAX, EAX
0x80bcef3: JNS 0x080bce4a
0x80bcef9: LEA ESI, [ESI]
0x80bcefc: SUB ESP, 0xc
0x80bceff: PUSH 0x82b2ac4
0x80bcf04: CALL 0x08097b24
0x80bcf09: ADD ESP, 0x10
0x80bcf0c: MOV EAX, dword ptr [ESI + 0x8]
0x80bcf0f: JMP 0x080bce4a
```

### vm_compileToFunCall @ 0x80cab20

证据：`ev_7bfea1e46d51d77201c90e9d4e594b0f6791c4bff0c33cc634a3bd46cde37c78`。

```c
void vm_compileToFunCall(Vm_Jit_Compiler_Env_T *pJitEnv)

{
  void **ppvVar1;
  
                    /* Unresolved local var: int insLen@[???] */
  vm_jitPutBytes(pJitEnv,0xa1,1);
  vm_jitPutBytes(pJitEnv,0x86619d8,4);
  vm_jitPutBytes(pJitEnv,0x40c7,2);
  vm_jitPutBytes(pJitEnv,0xc,1);
  vm_jitPutBytes(pJitEnv,pJitEnv->ip + 1,4);
  vm_compileToExe(pJitEnv);
  vm_insertCheckCtrlFlags(pJitEnv);
  vm_insertCheckNextActionHook(pJitEnv);
  ppvVar1 = vm_entryAddressList[0x56];
  if (pJitEnv->pNowCode != (uchar *)0x0) {
                    /* Unresolved local var: int callOffset@[???] */
    *pJitEnv->pNowCode = 0xe8;
    *(int *)(pJitEnv->pNowCode + 1) = (int)ppvVar1 + (-5 - (int)pJitEnv->pNowCode);
    pJitEnv->pNowCode = pJitEnv->pNowCode + 5;
  }
  pJitEnv->codeLen = pJitEnv->codeLen + 5;
  vm_jitPutBytes(pJitEnv,0xe0ff,2);
  return;
}
```

```asm
0x80cab20: PUSH EBP
0x80cab21: MOV EBP, ESP
0x80cab23: PUSH EBX
0x80cab24: SUB ESP, 0x8
0x80cab27: PUSH 0x1
0x80cab29: MOV EBX, dword ptr [EBP + 0x8]
0x80cab2c: PUSH 0xa1
0x80cab31: PUSH EBX
0x80cab32: CALL 0x080cad90
0x80cab37: ADD ESP, 0xc
0x80cab3a: PUSH 0x4
0x80cab3c: PUSH 0x86619d8
0x80cab41: PUSH EBX
0x80cab42: CALL 0x080cad90
0x80cab47: ADD ESP, 0xc
0x80cab4a: PUSH 0x2
0x80cab4c: PUSH 0x40c7
0x80cab51: PUSH EBX
0x80cab52: CALL 0x080cad90
0x80cab57: ADD ESP, 0xc
0x80cab5a: PUSH 0x1
0x80cab5c: PUSH 0xc
0x80cab5e: PUSH EBX
0x80cab5f: CALL 0x080cad90
0x80cab64: ADD ESP, 0xc
0x80cab67: PUSH 0x4
0x80cab69: MOV EAX, dword ptr [EBX + 0x18]
0x80cab6c: INC EAX
0x80cab6d: PUSH EAX
0x80cab6e: PUSH EBX
0x80cab6f: CALL 0x080cad90
0x80cab74: MOV dword ptr [ESP], EBX
0x80cab77: CALL 0x080ca554
0x80cab7c: MOV EAX, EBX
0x80cab7e: CALL 0x080ca9c4
0x80cab83: MOV EAX, EBX
0x80cab85: CALL 0x080ca8e8
0x80cab8a: MOV EDX, dword ptr [EBX + 0x8]
0x80cab8d: ADD ESP, 0x10
0x80cab90: TEST EDX, EDX
0x80cab92: MOV EAX, [0x0860cef8]
0x80cab97: JZ 0x080cabab
0x80cab99: MOV byte ptr [EDX], 0xe8
0x80cab9c: MOV EDX, dword ptr [EBX + 0x8]
0x80cab9f: SUB EAX, EDX
0x80caba1: SUB EAX, 0x5
0x80caba4: MOV dword ptr [EDX + 0x1], EAX
0x80caba7: ADD dword ptr [EBX + 0x8], 0x5
0x80cabab: ADD dword ptr [EBX + 0x10], 0x5
0x80cabaf: PUSH EAX
0x80cabb0: PUSH 0x2
0x80cabb2: PUSH 0xe0ff
0x80cabb7: PUSH EBX
0x80cabb8: CALL 0x080cad90
0x80cabbd: MOV EBX, dword ptr [EBP + -0x4]
0x80cabc0: LEAVE
0x80cabc1: RET
```

### vm_callInOtherFunExReturn @ 0x80bcdd8

证据：`ev_cf79f214bf5abcf504ec8eb0240a0f7d4b5abbc172ccbf5250c9ee9eecc5cfc9`。

```c
/* WARNING: Unknown calling convention */

void vm_callInOtherFunExReturn(void *pCookie)

{
                    /* Unresolved local var: Vm_Combined_Cookie_T combinedCookie@[???] */
  if (((uint)pCookie & 1) == 0) {
    return;
  }
  vm_returnFromEfun();
  return;
}
```

```asm
0x80bcdd8: PUSH EBP
0x80bcdd9: MOV EBP, ESP
0x80bcddb: TEST byte ptr [EBP + 0x8], 0x1
0x80bcddf: JNZ 0x080bcde4
0x80bcde1: LEAVE
0x80bcde2: RET
0x80bcde4: LEAVE
0x80bcde5: JMP 0x08093c5c
```

### vm_callInOtherArray2 @ 0x80bcf14

证据：`ev_dae2362e9944413d4a2410309483bc76294a735723624ba3a5fc2f583c2f99b4`。

```c
void vm_callInOtherArray2(void *pCookie)

{
  Vm_Value_T *pValue;
  Vm_Value_T *pValue_00;
  Vm_Value_T *pVVar1;
  uint argn;
  int iVar2;
  Vm_Value *pVVar3;
  int iVar4;
  uchar uVar5;
  undefined1 *puVar6;
  uint uVar7;
  uint uVar8;
  bool bVar9;
  int nArgNum;
  Vm_Value_T *pCalledFunName;
  Vm_Value_T *pSrcArray;
  Vm_Value_T *pObjectValue;
  void *pCookie_local;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Value_T * pResultArray@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pResultIndex@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Combined_Cookie_T combinedCookie@[???]
                       Unresolved local var: int fromEfun@[???]
                       Unresolved local var: int sp@[DW_OP_reg3(EBX)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  pVVar1 = vm_stack;
  argn = (uint)pCookie >> 8 & 0xfff;
  uVar7 = (uint)pCookie >> 0x14;
  pValue = vm_stack + uVar7 + 3;
  if (uVar7 != vm_sp) {
    vm_error("Bad sp when processing call_other array.\n");
  }
  if ((pVVar1[uVar7].type != '\x06') || (pVVar1[uVar7 + 1].type != '\x02')) {
    vm_error("Bad intermediate values in stack when processing call_other array.\n");
  }
  puVar6 = (undefined1 *)pVVar1[uVar7 + 1].v.number;
  iVar4 = (argn + uVar7) * 0x18;
  if ((int)puVar6 < (pVVar1[uVar7].v.object)->execRef) {
    do {
      pValue_00 = *(Vm_Value_T **)((pVVar1[uVar7 + 2].v.string)->ptr + (int)puVar6 * 4);
      iVar2 = vm_isZero(pValue_00);
      uVar8 = argn;
      if (iVar2 == 0) goto LAB_080bd017;
      puVar6 = (undefined1 *)((int)&(pVVar1[uVar7 + 1].v.string)->ref + 1);
      pVVar1[uVar7 + 1].v.number = (int)puVar6;
    } while ((int)puVar6 < (pVVar1[uVar7].v.object)->execRef);
  }
  vm_takeOffValueFromStack(1,argn + 3);
  if (((uint)pCookie & 1) == 0) {
    return;
  }
  vm_returnFromEfun();
  return;
LAB_080bd017:
  for (; 0 < (int)uVar8; uVar8 = uVar8 - 1) {
    vm_pushValue((Vm_Value_T *)((int)&vm_stack[3].ref + iVar4));
    iVar4 = iVar4 + -0x18;
  }
  vm_pushValue(pValue);
  vm_pushValue(pValue_00);
  if (vm_sp < vm_stackSize) {
    pVVar3 = vm_stackTopValue();
    uVar5 = pVVar3->type;
    if (uVar5 != '\x04') goto LAB_080bd051;
LAB_080bd0d9:
    vm_findObjectByString();
    pVVar3 = vm_stackTopValue();
    bVar9 = pVVar3->type == '\n';
LAB_080bd0f0:
    if (bVar9) goto LAB_080bd06a;
  }
  else {
    vm_error("Missed argument in stack.\n");
    pVVar3 = vm_stackTopValue();
    uVar5 = pVVar3->type;
    if (uVar5 == '\x04') goto LAB_080bd0d9;
LAB_080bd051:
    if (uVar5 == '\n') {
      bVar9 = ((uint)((pVVar3->v).string)->ptr & 0x8000) == 0;
      goto LAB_080bd0f0;
    }
  }
  vm_error("Bad type of value to call in array.\n");
LAB_080bd06a:
  iVar4 = vm_callInOtherFun(4,argn);
  if (iVar4 != 0) {
    vm_hookCallbackWhenReturn(vm_callInOtherArray3,pCookie);
    return;
  }
  vm_callInOtherArray3(pCookie);
  return;
}
```

```asm
0x80bcf14: PUSH EBP
0x80bcf15: MOV EBP, ESP
0x80bcf17: PUSH EDI
0x80bcf18: PUSH ESI
0x80bcf19: PUSH EBX
0x80bcf1a: SUB ESP, 0x1c
0x80bcf1d: MOV EAX, dword ptr [EBP + 0x8]
0x80bcf20: MOV dword ptr [EBP + -0x10], EAX
0x80bcf23: MOV EBX, dword ptr [EBP + -0x10]
0x80bcf26: SHR EAX, 0x8
0x80bcf29: AND EAX, 0xfff
0x80bcf2e: SHR EBX, 0x14
0x80bcf31: MOV dword ptr [EBP + -0x20], EAX
0x80bcf34: LEA EDX, [EBX + EBX*0x2]
0x80bcf37: MOV EAX, [0x08661964]
0x80bcf3c: LEA ESI, [EAX + EDX*0x8]
0x80bcf3f: LEA ECX, [ESI + 0x30]
0x80bcf42: LEA EAX, [ESI + 0x48]
0x80bcf45: CMP EBX, dword ptr [0x086619cc]
0x80bcf4b: LEA EDI, [ESI + 0x18]
0x80bcf4e: MOV dword ptr [EBP + -0x18], ECX
0x80bcf51: MOV dword ptr [EBP + -0x1c], EAX
0x80bcf54: JZ 0x080bcf66
0x80bcf56: SUB ESP, 0xc
0x80bcf59: PUSH 0x82b2b04
0x80bcf5e: CALL 0x08097b24
0x80bcf63: ADD ESP, 0x10
0x80bcf66: CMP byte ptr [ESI + 0x4], 0x6
0x80bcf6a: JZ 0x080bd090
0x80bcf70: SUB ESP, 0xc
0x80bcf73: PUSH 0x82b2b30
0x80bcf78: CALL 0x08097b24
0x80bcf7d: ADD ESP, 0x10
0x80bcf80: MOV EAX, dword ptr [EBP + -0x20]
0x80bcf83: ADD EAX, EBX
0x80bcf85: LEA EAX, [EAX + EAX*0x2]
0x80bcf88: MOV EDX, dword ptr [EDI + 0x8]
0x80bcf8b: LEA EBX, [EAX*0x8 + 0x0]
0x80bcf92: MOV EAX, dword ptr [ESI + 0x8]
0x80bcf95: CMP EDX, dword ptr [EAX + 0x4]
0x80bcf98: JGE 0x080bcfca
0x80bcf9a: MOV ECX, dword ptr [EBP + -0x18]
0x80bcf9d: MOV EAX, dword ptr [ECX + 0x8]
0x80bcfa0: MOV EAX, dword ptr [EAX + 0xc]
0x80bcfa3: SUB ESP, 0xc
0x80bcfa6: MOV EDX, dword ptr [EAX + EDX*0x4]
0x80bcfa9: MOV dword ptr [EBP + -0x14], EDX
0x80bcfac: PUSH EDX
0x80bcfad: CALL 0x0808cad4
0x80bcfb2: ADD ESP, 0x10
0x80bcfb5: TEST EAX, EAX
0x80bcfb7: JZ 0x080bcff4
0x80bcfb9: MOV EAX, dword ptr [EDI + 0x8]
0x80bcfbc: INC EAX
0x80bcfbd: MOV dword ptr [EDI + 0x8], EAX
0x80bcfc0: MOV EDX, EAX
0x80bcfc2: MOV EAX, dword ptr [ESI + 0x8]
0x80bcfc5: CMP EDX, dword ptr [EAX + 0x4]
0x80bcfc8: JL 0x080bcf9a
0x80bcfca: MOV EAX, dword ptr [EBP + -0x20]
0x80bcfcd: SUB ESP, 0x8
0x80bcfd0: ADD EAX, 0x3
0x80bcfd3: PUSH EAX
0x80bcfd4: PUSH 0x1
0x80bcfd6: CALL 0x08092008
0x80bcfdb: ADD ESP, 0x10
0x80bcfde: TEST byte ptr [EBP + -0x10], 0x1
0x80bcfe2: JZ 0x080bd0af
0x80bcfe8: LEA ESP, [EBP + -0xc]
0x80bcfeb: POP EBX
0x80bcfec: POP ESI
0x80bcfed: POP EDI
0x80bcfee: LEAVE
0x80bcfef: JMP 0x08093c5c
0x80bcff4: MOV ESI, dword ptr [EBP + -0x20]
0x80bcff7: JMP 0x080bd017
0x80bcffc: MOV EAX, EBX
0x80bcffe: ADD EAX, dword ptr [0x08661964]
0x80bd004: SUB ESP, 0xc
0x80bd007: ADD EAX, 0x48
0x80bd00a: PUSH EAX
0x80bd00b: CALL 0x08091074
0x80bd010: DEC ESI
0x80bd011: ADD ESP, 0x10
0x80bd014: SUB EBX, 0x18
0x80bd017: TEST ESI, ESI
0x80bd019: JG 0x080bcffc
0x80bd01b: SUB ESP, 0xc
0x80bd01e: PUSH dword ptr [EBP + -0x1c]
0x80bd021: CALL 0x08091074
0x80bd026: POP EAX
0x80bd027: PUSH dword ptr [EBP + -0x14]
0x80bd02a: CALL 0x08091074
0x80bd02f: MOV EAX, [0x08661960]
0x80bd034: ADD ESP, 0x10
0x80bd037: CMP dword ptr [0x086619cc], EAX
0x80bd03d: JGE 0x080bd0b7
0x80bd03f: CALL 0x08090604
0x80bd044: MOVZX EDX, byte ptr [EAX + 0x4]
0x80bd048: CMP EDX, 0x4
0x80bd04b: JZ 0x080bd0d9
0x80bd051: CMP EDX, 0xa
0x80bd054: JZ 0x080bd0e9
0x80bd05a: SUB ESP, 0xc
0x80bd05d: PUSH 0x82b2b74
0x80bd062: CALL 0x08097b24
0x80bd067: ADD ESP, 0x10
0x80bd06a: SUB ESP, 0x8
0x80bd06d: PUSH dword ptr [EBP + -0x20]
0x80bd070: PUSH 0x4
0x80bd072: CALL 0x08094cd8
0x80bd077: ADD ESP, 0x10
0x80bd07a: TEST EAX, EAX
0x80bd07c: JNZ 0x080bd09f
0x80bd07e: MOV EAX, dword ptr [EBP + -0x10]
0x80bd081: MOV dword ptr [EBP + 0x8], EAX
0x80bd084: LEA ESP, [EBP + -0xc]
0x80bd087: POP EBX
0x80bd088: POP ESI
0x80bd089: POP EDI
0x80bd08a: LEAVE
0x80bd08b: JMP 0x080bcdec
0x80bd090: CMP byte ptr [EDI + 0x4], 0x2
0x80bd094: JNZ 0x080bcf70
0x80bd09a: JMP 0x080bcf80
0x80bd09f: SUB ESP, 0x8
0x80bd0a2: PUSH dword ptr [EBP + -0x10]
0x80bd0a5: PUSH 0x80bcdec
0x80bd0aa: CALL 0x08094e70
0x80bd0af: LEA ESP, [EBP + -0xc]
0x80bd0b2: POP EBX
0x80bd0b3: POP ESI
0x80bd0b4: POP EDI
0x80bd0b5: LEAVE
0x80bd0b6: RET
0x80bd0b7: SUB ESP, 0xc
0x80bd0ba: PUSH 0x82a9240
0x80bd0bf: CALL 0x08097b24
0x80bd0c4: ADD ESP, 0x10
0x80bd0c7: CALL 0x08090604
0x80bd0cc: MOVZX EDX, byte ptr [EAX + 0x4]
0x80bd0d0: CMP EDX, 0x4
0x80bd0d3: JNZ 0x080bd051
0x80bd0d9: CALL 0x080f4150
0x80bd0de: CALL 0x08090604
0x80bd0e3: CMP byte ptr [EAX + 0x4], 0xa
0x80bd0e7: JMP 0x080bd0f0
0x80bd0e9: MOV EAX, dword ptr [EAX + 0x8]
0x80bd0ec: TEST byte ptr [EAX + 0xd], 0x80
0x80bd0f0: JZ 0x080bd06a
0x80bd0f6: JMP 0x080bd05a
```

