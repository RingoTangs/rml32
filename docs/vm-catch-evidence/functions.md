# 阶段 2U Ghidra 证据索引

主 ELF SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。新增记录使用 REA MCP / Ghidra 12.1.4，原始目标未执行、未修改，会话已关闭。复用记录保持原样，并记录来源文件哈希。完整原始返回、函数体范围、交叉引用和限制保留在压缩包中。

伪代码为反编译结果，不是原始源码；恢复签名与 union 字段名可能失真。数据表入口不能仅由缺少直接 caller 推断为不可达。

| 包 | 操作／函数 | 地址 | Evidence ID |
|---|---|---|---|
| rea.json.gz | analyze_function / vm_newThreadCatch | 0x80aaf84 | `ev_446aa83f597882387e0856507a84ad5ec466a2e91a4e234c7e9c000ca98eec9a` |
| rea.json.gz | analyze_function / vm_endThreadCatch | 0x80aae08 | `ev_3c2a03d9a2a44bb053d7549ec474236c098efdaddd7ead0a8a811d91c7507797` |
| rea.json.gz | procedure_assembly / vm_caughtError | 0x80ab020 | `ev_9fb21c970e49e1948945a0156491b006ffff02c2dfe7da9caf0a8d96167cad97` |
| rea.json.gz | procedure_assembly / vm_doActions | 0x8097008 | `ev_a280841df7ce885805c45ba2b70f60cf4628279d7dbb748b1f816ba99a1ad2c4` |
| reused.json.gz | procedure_pseudo_code / vm_error | 见汇编/关联记录 | `ev_1b494a92dfe3a6c5943ad427ec0ed25c0e15c7afc6932862b5221a85cb9ef7c7` |
| reused.json.gz | procedure_pseudo_code / vm_doActions | 见汇编/关联记录 | `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f` |
| reused.json.gz | procedure_pseudo_code / vm_traceError | 见汇编/关联记录 | `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4` |
| reused.json.gz | procedure_references / vm_doActions | 0x8097008 | `ev_520af79391604c804c2a40d599235d711523bbebfdfa5c927ac8f4d5a96cc44f` |
| reused.json.gz | procedure_references / vm_traceError | 0x8093520 | `ev_757d5e071776339db4cabc56f20d2332b62632fe1fa366c3f0414c9f87edad47` |
| reused.json.gz | procedure_pseudo_code / vm_caughtError | 见汇编/关联记录 | `ev_b8cf22d050dedb8fc06d53d128ce869b74d9bfc40305dca347b34e750ee9ad50` |
| reused.json.gz | procedure_assembly / vm_errorQuit | 0x80ab104 | `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` |
| reused.json.gz | procedure_pseudo_code / vm_errorQuit | 见汇编/关联记录 | `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1` |
| reused.json.gz | procedure_assembly / vm_traceError | 0x8093520 | `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` |
| reused.json.gz | procedure_pseudo_code / vm_newThreadCatch2 | 见汇编/关联记录 | `ev_04c0d19e992575aa438242c9b50d1dc5095f14e6645d2794b5e093f287306ba7` |
| reused.json.gz | procedure_pseudo_code / vm_regularProcessor | 见汇编/关联记录 | `ev_5673fa5b2edb4d5bf79e9f9431a44467493506747573d429692a62ef74c6c5de` |
| reused.json.gz | procedure_assembly / vm_regularProcessor | 0x80978c4 | `ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597` |
| reused.json.gz | procedure_pseudo_code / vm_cleanupValue | 见汇编/关联记录 | `ev_6eecd8e699df5eefd7a3a4825b0b2c554b2844c7bf1228db97aa0803c3d8900e` |
| reused.json.gz | analyze_function / vm_returnFromFun | 0x8094978 | `ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215` |
| reused.json.gz | analyze_function / vm_freeLoopValue | 0x80bc884 | `ev_7c730bb3ca6a220df55353b514f3cfe7e4d79e6347771e672ae459058017f177` |
| reused.json.gz | analyze_function / vm_popNStack | 0x8091ab0 | `ev_e1d24b79fb397b943d57fc71f60c961852e9a969ae6f32b18bc0203f254b6ed4` |
| reused.json.gz | procedure_assembly / vm_divValue | 0x808e1a4 | `ev_19597a6d356a5c98c32c16698ef7be5e4ad2201b7a0a22f7a714cc7918dd6f89` |
| reused.json.gz | procedure_pseudo_code / vm_divValue | 见汇编/关联记录 | `ev_8ae08d055f9314e0e074d8efe90e725516a1ec2a40a18b7fb1e9bc1014dc355c` |

## 新增注册与结束函数：完整伪代码及汇编

### vm_newThreadCatch

Evidence：`ev_446aa83f597882387e0856507a84ad5ec466a2e91a4e234c7e9c000ca98eec9a`

```c
void vm_newThreadCatch(int errCaseOffset)

{
  Vm_Catch_Info_T *pVVar1;
  int iVar2;
  Vm_PCB_T *pVVar3;
  Vm_Thread_Info_T *pVVar4;
  int iVar5;
  
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Catch_Info_T * pCatch@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_PCB_T * pPCB@[???] */
  pVVar4 = vm_getCurrentThreadInfo();
  if (_vm_pActiveProcess->catchContextSize <= _vm_pActiveProcess->catchCp) {
    vm_error("Too depth catch (%d).\n",_vm_pActiveProcess->catchCp);
  }
  pVVar3 = _vm_pActiveProcess;
  iVar2 = _vm_pActiveProcess->catchCp;
  _vm_pActiveProcess->catchCp = iVar2 + 1;
  pVVar1 = pVVar3->catchContext + iVar2;
  pVVar1->catchIp = errCaseOffset + vm_currentContext->ip;
  iVar5 = vm_getCurrentFunCp();
  iVar2 = vm_sp;
  pVVar1->funCp = iVar5;
  pVVar1->sp = iVar2;
  iVar2 = pVVar4->topCatch;
  pVVar1->catchCallback = (_func_void_void_ptr *)0x0;
  pVVar1->pCookie = (void *)0x0;
  pVVar4->topEnv[iVar2 + -1].funCp = iVar5;
  return;
}
```

```asm
0x80aaf84: PUSH EBP
0x80aaf85: MOV EBP, ESP
0x80aaf87: PUSH EDI
0x80aaf88: PUSH ESI
0x80aaf89: PUSH EBX
0x80aaf8a: SUB ESP, 0xc
0x80aaf8d: MOV ESI, dword ptr [EBP + 0x8]
0x80aaf90: CALL 0x080a3678
0x80aaf95: MOV ECX, dword ptr [0x086078ac]
0x80aaf9b: MOV EDI, EAX
0x80aaf9d: MOV EAX, dword ptr [ECX + 0x98]
0x80aafa3: CMP EAX, dword ptr [ECX + 0x9c]
0x80aafa9: JGE 0x080ab004
0x80aafab: MOV EAX, dword ptr [ECX + 0x98]
0x80aafb1: LEA EBX, [EAX + EAX*0x4]
0x80aafb4: INC EAX
0x80aafb5: MOV dword ptr [ECX + 0x98], EAX
0x80aafbb: MOV EDX, dword ptr [ECX + 0xa0]
0x80aafc1: MOV EAX, [0x086619d8]
0x80aafc6: LEA EBX, [EDX + EBX*0x4]
0x80aafc9: ADD ESI, dword ptr [EAX + 0xc]
0x80aafcc: MOV dword ptr [EBX], ESI
0x80aafce: CALL 0x08092c2c
0x80aafd3: MOV EDX, dword ptr [0x086619cc]
0x80aafd9: MOV dword ptr [EBX + 0x4], EAX
0x80aafdc: MOV dword ptr [EBX + 0x8], EDX
0x80aafdf: MOV EDX, dword ptr [EDI + 0xa48]
0x80aafe5: LEA ECX, [EDX + EDX*0x4]
0x80aafe8: LEA ECX, [EDX + ECX*0x8]
0x80aafeb: MOV dword ptr [EBX + 0xc], 0x0
0x80aaff2: MOV dword ptr [EBX + 0x10], 0x0
0x80aaff9: MOV dword ptr [EDI + ECX*0x4], EAX
0x80aaffc: LEA ESP, [EBP + -0xc]
0x80aafff: POP EBX
0x80ab000: POP ESI
0x80ab001: POP EDI
0x80ab002: LEAVE
0x80ab003: RET
0x80ab004: SUB ESP, 0x8
0x80ab007: PUSH EAX
0x80ab008: PUSH 0x82b0cad
0x80ab00d: CALL 0x08097b24
0x80ab012: ADD ESP, 0x10
0x80ab015: MOV ECX, dword ptr [0x086078ac]
0x80ab01b: JMP 0x080aafab
```

直接 caller（静态直接边，不含所有间接调度）：

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

### vm_endThreadCatch

Evidence：`ev_3c2a03d9a2a44bb053d7549ec474236c098efdaddd7ead0a8a811d91c7507797`

```c
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_endThreadCatch(void)

{
  int iVar1;
  Vm_PCB_T *pVVar2;
  Vm_Thread_Info_T *pVVar3;
  int iVar4;
  int iVar5;
  
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_PCB_T * pPCB@[???]
                       Unresolved local var: int funCp@[DW_OP_reg3(EBX)] */
  pVVar3 = vm_getCurrentThreadInfo();
  pVVar2 = _vm_pActiveProcess;
  iVar1 = _vm_pActiveProcess->catchCp;
  iVar4 = iVar1 + -1;
  iVar5 = 0;
  _vm_pActiveProcess->catchCp = iVar4;
  if (0 < iVar4) {
    iVar5 = pVVar2->catchContext[iVar1 + -2].funCp;
  }
  if (0 < pVVar3->topCatch) {
    pVVar3->topEnv[pVVar3->topCatch + -1].funCp = iVar5;
  }
  return;
}
```

```asm
0x80aae08: PUSH EBP
0x80aae09: MOV EBP, ESP
0x80aae0b: PUSH EBX
0x80aae0c: PUSH EAX
0x80aae0d: CALL 0x080a3678
0x80aae12: MOV EDX, dword ptr [0x086078ac]
0x80aae18: MOV ECX, EAX
0x80aae1a: MOV EAX, dword ptr [EDX + 0x98]
0x80aae20: DEC EAX
0x80aae21: XOR EBX, EBX
0x80aae23: TEST EAX, EAX
0x80aae25: MOV dword ptr [EDX + 0x98], EAX
0x80aae2b: JLE 0x080aae3c
0x80aae2d: LEA EAX, [EAX + EAX*0x4]
0x80aae30: SHL EAX, 0x2
0x80aae33: ADD EAX, dword ptr [EDX + 0xa0]
0x80aae39: MOV EBX, dword ptr [EAX + -0x10]
0x80aae3c: MOV EDX, dword ptr [ECX + 0xa48]
0x80aae42: TEST EDX, EDX
0x80aae44: JLE 0x080aae4f
0x80aae46: LEA EAX, [EDX + EDX*0x4]
0x80aae49: LEA EAX, [EDX + EAX*0x8]
0x80aae4c: MOV dword ptr [ECX + EAX*0x4], EBX
0x80aae4f: POP EAX
0x80aae50: POP EBX
0x80aae51: LEAVE
0x80aae52: RET
```

直接 caller（静态直接边，不含所有间接调度）：vm_traceError@0x8093520, vm_returnFromFun@0x8094978, vm_endOfDebugApplyScript@0x80c9e44, vm_f_delete_thread_ok@0x80ef920, vm_f_delete_thread@0x80ef984, vm_f_construct_object_ok1@0x80f9250, vm_f_construct_object_ok2@0x80f9268, vm_f_destruct_func_ok@0x80f95cc

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

### vm_caughtError

Evidence：`ev_9fb21c970e49e1948945a0156491b006ffff02c2dfe7da9caf0a8d96167cad97`

```asm
0x80ab020: PUSH EBP
0x80ab021: MOV EBP, ESP
0x80ab023: SUB ESP, 0x14
0x80ab026: PUSH 0x1
0x80ab028: CALL 0x08090b3c
0x80ab02d: LEAVE
0x80ab02e: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

### vm_doActions

Evidence：`ev_a280841df7ce885805c45ba2b70f60cf4628279d7dbb748b1f816ba99a1ad2c4`

```asm
0x8097008: PUSH EBP
0x8097009: MOV EBP, ESP
0x809700b: SUB ESP, 0x8
0x809700e: MOV EDX, dword ptr [EBP + 0xc]
0x8097011: TEST EDX, EDX
0x8097013: MOV EAX, 0x2710
0x8097018: CMOVNZ EAX, EDX
0x809701b: MOV [0x083cc4ec], EAX
0x8097020: MOV EAX, dword ptr [EBP + 0x8]
0x8097023: TEST byte ptr [EAX + 0x2a], 0x20
0x8097027: JNZ 0x0809725e
0x809702d: SUB ESP, 0x8
0x8097030: PUSH dword ptr [0x083cc4ec]
0x8097036: PUSH 0x0
0x8097038: CALL 0x080a3854
0x809703d: LEA ESI, [ESI]
0x8097040: ADD ESP, 0x10
0x8097043: MOV EDX, dword ptr [EBP + 0x8]
0x8097046: MOV EAX, dword ptr [EDX + 0x98]
0x809704c: XOR EDX, EDX
0x809704e: TEST EAX, EAX
0x8097050: JLE 0x08097066
0x8097052: MOV ESI, ESI
0x8097054: LEA EAX, [EAX + EAX*0x4]
0x8097057: SHL EAX, 0x2
0x809705a: MOV ECX, dword ptr [EBP + 0x8]
0x809705d: ADD EAX, dword ptr [ECX + 0xa0]
0x8097063: MOV EDX, dword ptr [EAX + -0x10]
0x8097066: SUB ESP, 0x18
0x8097069: PUSH 0x1
0x809706b: PUSH EDX
0x809706c: CALL 0x080aae68
0x8097071: ADD ESP, 0x14
0x8097074: PUSH EAX
0x8097075: CALL 0x0804bac4
0x809707a: ADD ESP, 0x10
0x809707d: TEST EAX, EAX
0x809707f: JZ 0x08097118
0x8097085: CMP EAX, -0x3
0x8097088: MOV dword ptr [EBP + -0x8], EAX
0x809708b: JNZ 0x080971e8
0x8097091: MOV ECX, dword ptr [EBP + 0x8]
0x8097094: MOV EAX, dword ptr [ECX + 0x98]
0x809709a: TEST EAX, EAX
0x809709c: JLE 0x080971e8
0x80970a2: DEC EAX
0x80970a3: MOV dword ptr [ECX + 0x98], EAX
0x80970a9: MOV EDX, dword ptr [ECX + 0xa0]
0x80970af: LEA EAX, [EAX + EAX*0x4]
0x80970b2: LEA EAX, [EDX + EAX*0x4]
0x80970b5: MOV dword ptr [EBP + -0x4], EAX
0x80970b8: MOV EDX, dword ptr [0x086619cc]
0x80970be: MOV EAX, dword ptr [EAX + 0x8]
0x80970c1: CMP EAX, EDX
0x80970c3: MOV dword ptr [EBP + -0x8], 0x0
0x80970ca: JG 0x080971f0
0x80970d0: MOV ECX, dword ptr [EBP + -0x4]
0x80970d3: MOV EDX, dword ptr [ECX]
0x80970d5: CMP EDX, -0x1
0x80970d8: JZ 0x0809720c
0x80970de: MOV EAX, [0x086619d8]
0x80970e3: MOV dword ptr [EAX + 0xc], EDX
0x80970e6: MOV EDX, dword ptr [EBP + 0x8]
0x80970e9: MOV EAX, dword ptr [EDX + 0x98]
0x80970ef: XOR EDX, EDX
0x80970f1: TEST EAX, EAX
0x80970f3: JG 0x08097054
0x80970f9: SUB ESP, 0x18
0x80970fc: PUSH 0x1
0x80970fe: PUSH EDX
0x80970ff: CALL 0x080aae68
0x8097104: ADD ESP, 0x14
0x8097107: PUSH EAX
0x8097108: CALL 0x0804bac4
0x809710d: ADD ESP, 0x10
0x8097110: TEST EAX, EAX
0x8097112: JNZ 0x08097085
0x8097118: MOV EDX, dword ptr [EBP + 0x8]
0x809711b: MOV EAX, dword ptr [EDX + 0x28]
0x809711e: TEST AL, 0x40
0x8097120: JNZ 0x080972a4
0x8097126: MOV ECX, dword ptr [0x08604020]
0x809712c: TEST ECX, ECX
0x809712e: JZ 0x0809727c
0x8097134: MOV EAX, [0x086619d4]
0x8097139: TEST EAX, EAX
0x809713b: MOV dword ptr [EBP + -0x8], 0xffffe055
0x8097142: JZ 0x08097173
0x8097144: TEST AL, 0x10
0x8097146: JZ 0x08097156
0x8097148: AND EAX, 0xffffffef
0x809714b: XOR EDX, EDX
0x809714d: TEST EAX, EAX
0x809714f: MOV [0x086619d4], EAX
0x8097154: JZ 0x08097170
0x8097156: MOV EAX, [0x086619d4]
0x809715b: TEST AL, 0x2
0x809715d: JZ 0x080972b4
0x8097163: AND EAX, 0xfffffffd
0x8097166: MOV [0x086619d4], EAX
0x809716b: MOV EDX, 0xffffe059
0x8097170: MOV dword ptr [EBP + -0x8], EDX
0x8097173: CMP dword ptr [EBP + -0x8], 0xffffe055
0x809717a: JNZ 0x080971b0
0x809717c: MOV EAX, [0x086619d8]
0x8097181: MOV EDX, dword ptr [EAX + 0xc]
0x8097184: TEST EDX, EDX
0x8097186: JS 0x080972e6
0x809718c: MOV EAX, dword ptr [EAX + 0x10]
0x809718f: CMP EDX, dword ptr [EAX]
0x8097191: JGE 0x080972e6
0x8097197: SUB ESP, 0xc
0x809719a: PUSH dword ptr [EBP + 0x8]
0x809719d: CALL dword ptr [0x086040a0]
0x80971a3: MOV dword ptr [EBP + -0x8], EAX
0x80971a6: ADD ESP, 0x10
0x80971a9: CMP EAX, 0xffffe055
0x80971ae: JZ 0x0809717c
0x80971b0: MOV EDX, dword ptr [EBP + 0x8]
0x80971b3: MOV EAX, dword ptr [EDX + 0x28]
0x80971b6: TEST EAX, 0x200000
0x80971bb: JZ 0x080971e3
0x80971bd: TEST EAX, EAX
0x80971bf: JS 0x080971e3
0x80971c1: CMP dword ptr [EBP + -0x8], 0xffffe05b
0x80971c8: JZ 0x080971e3
0x80971ca: CMP dword ptr [EBP + -0x8], 0xffffe05a
0x80971d1: JZ 0x080971e3
0x80971d3: SUB ESP, 0xc
0x80971d6: PUSH 0x82af008
0x80971db: CALL 0x08097b24
0x80971e0: ADD ESP, 0x10
0x80971e3: CALL 0x080aae54
0x80971e8: MOV EAX, dword ptr [EBP + -0x8]
0x80971eb: LEAVE
0x80971ec: RET
0x80971f0: SUB ESP, 0xc
0x80971f3: SUB EAX, EDX
0x80971f5: PUSH EAX
0x80971f6: CALL 0x08091ab0
0x80971fb: MOV ECX, dword ptr [EBP + -0x4]
0x80971fe: MOV EDX, dword ptr [ECX]
0x8097200: ADD ESP, 0x10
0x8097203: CMP EDX, -0x1
0x8097206: JNZ 0x080970de
0x809720c: MOV EDX, dword ptr [EBP + 0x8]
0x809720f: MOV EAX, dword ptr [EDX + 0x98]
0x8097215: XOR EDX, EDX
0x8097217: TEST EAX, EAX
0x8097219: JLE 0x0809722d
0x809721b: LEA EAX, [EAX + EAX*0x4]
0x809721e: SHL EAX, 0x2
0x8097221: MOV ECX, dword ptr [EBP + 0x8]
0x8097224: ADD EAX, dword ptr [ECX + 0xa0]
0x809722a: MOV EDX, dword ptr [EAX + -0x10]
0x809722d: SUB ESP, 0x18
0x8097230: PUSH 0x1
0x8097232: PUSH EDX
0x8097233: CALL 0x080aae68
0x8097238: ADD ESP, 0x14
0x809723b: PUSH EAX
0x809723c: CALL 0x0804bac4
0x8097241: ADD ESP, 0x10
0x8097244: TEST EAX, EAX
0x8097246: JNZ 0x080971e8
0x8097248: SUB ESP, 0xc
0x809724b: MOV EAX, dword ptr [EBP + -0x4]
0x809724e: PUSH dword ptr [EAX + 0x10]
0x8097251: CALL dword ptr [EAX + 0xc]
0x8097254: CALL 0x080aae54
0x8097259: JMP 0x08097040
0x809725e: CALL 0x080a9060
0x8097263: TEST EAX, EAX
0x8097265: MOV EDX, 0x1d4c0
0x809726a: CMOVNZ EDX, dword ptr [0x083cc4ec]
0x8097271: MOV dword ptr [0x083cc4ec], EDX
0x8097277: JMP 0x0809702d
0x809727c: MOV ECX, dword ptr [EBP + 0x8]
0x809727f: CMP dword ptr [ECX + 0x44], 0x927c0
0x8097286: JBE 0x08097134
0x809728c: CALL 0x080a9060
0x8097291: TEST EAX, EAX
0x8097293: JNZ 0x0809730b
0x8097295: MOV EAX, dword ptr [EBP + 0x8]
0x8097298: MOV dword ptr [EAX + 0x44], 0x0
0x809729f: JMP 0x08097134
0x80972a4: AND EAX, 0xffffffbf
0x80972a7: MOV dword ptr [EDX + 0x28], EAX
0x80972aa: CALL 0x08093c5c
0x80972af: JMP 0x08097126
0x80972b4: TEST AL, 0x4
0x80972b6: JNZ 0x080972d1
0x80972b8: TEST AL, 0x8
0x80972ba: JZ 0x080972f2
0x80972bc: AND EAX, 0xfffffff7
0x80972bf: MOV EDX, 0xffffe05a
0x80972c4: MOV [0x086619d4], EAX
0x80972c9: MOV dword ptr [EBP + -0x8], EDX
0x80972cc: JMP 0x08097173
0x80972d1: AND EAX, 0xfffffffb
0x80972d4: MOV EDX, 0xffffe058
0x80972d9: MOV [0x086619d4], EAX
0x80972de: MOV dword ptr [EBP + -0x8], EDX
0x80972e1: JMP 0x08097173
0x80972e6: MOV dword ptr [EBP + -0x8], 0xffffe05a
0x80972ed: JMP 0x080971b0
0x80972f2: TEST AL, 0x20
0x80972f4: JZ 0x08097320
0x80972f6: AND EAX, 0xffffffdf
0x80972f9: MOV EDX, 0xffffe055
0x80972fe: MOV [0x086619d4], EAX
0x8097303: MOV dword ptr [EBP + -0x8], EDX
0x8097306: JMP 0x08097173
0x809730b: SUB ESP, 0xc
0x809730e: PUSH 0x82af03c
0x8097313: CALL 0x08097b24
0x8097318: ADD ESP, 0x10
0x809731b: JMP 0x08097134
0x8097320: TEST AL, 0x40
0x8097322: JZ 0x08097339
0x8097324: AND EAX, 0xffffffbf
0x8097327: MOV EDX, 0xffffe05b
0x809732c: MOV [0x086619d4], EAX
0x8097331: MOV dword ptr [EBP + -0x8], EDX
0x8097334: JMP 0x08097173
0x8097339: TEST AL, 0x80
0x809733b: MOV EDX, 0xffffe05a
0x8097340: JZ 0x08097170
0x8097346: AND AL, 0x7f
0x8097348: MOV DL, 0x54
0x809734a: MOV [0x086619d4], EAX
0x809734f: MOV dword ptr [EBP + -0x8], EDX
0x8097352: JMP 0x08097173
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

