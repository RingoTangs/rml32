# 2AI 新增原生静态函数视图

完整记录在rea.json.gz；以下是可读伪代码/汇编，不是原始C源码，也不是执行观察。

## vm_concatString @ 0x808c7fc

Evidence: `ev_0ad799c15b35d517803d7b03ec4cfdcd869b5b8720539419cd55aaca475fa1eb`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808c83a", "start": "0x808c7fc"}], "span_bytes": 63, "total_bytes": 63}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

Vm_String_T * vm_concatString(Vm_String_T *pString,Vm_String_T *pCString)

{
  uint len;
  uint uVar1;
  Vm_String_T *pVVar2;
  
                    /* Unresolved local var: Vm_String_T * dst@[DW_OP_reg6(ESI)]
                       Unresolved local var: int len@[DW_OP_reg7(EDI)]
                       Unresolved local var: int alen@[DW_OP_reg3(EBX)] */
  len = pCString->len;
  uVar1 = pString->len;
  pVVar2 = vm_extendString(pString,len);
  pVVar2->len = pVVar2->len + len;
  memcpy(pVVar2->ptr + uVar1,pCString->ptr,len + 1);
  return pVVar2;
}


```

```text
0x808c7fc: PUSH EBP
0x808c7fd: MOV EBP, ESP
0x808c7ff: PUSH EDI
0x808c800: PUSH ESI
0x808c801: PUSH EBX
0x808c802: SUB ESP, 0x14
0x808c805: MOV EDX, dword ptr [EBP + 0xc]
0x808c808: MOV EBX, dword ptr [EDX + 0x10]
0x808c80b: MOV EAX, dword ptr [EBP + 0x8]
0x808c80e: MOV EDI, dword ptr [EAX + 0x10]
0x808c811: PUSH EBX
0x808c812: PUSH EAX
0x808c813: CALL 0x0808c6dc
0x808c818: ADD dword ptr [EAX + 0x10], EBX
0x808c81b: ADD ESP, 0xc
0x808c81e: INC EBX
0x808c81f: ADD EDI, dword ptr [EAX + 0xc]
0x808c822: MOV ESI, EAX
0x808c824: PUSH EBX
0x808c825: MOV EAX, dword ptr [EBP + 0xc]
0x808c828: PUSH dword ptr [EAX + 0xc]
0x808c82b: PUSH EDI
0x808c82c: CALL 0x0804b7e4
0x808c831: LEA ESP, [EBP + -0xc]
0x808c834: POP EBX
0x808c835: MOV EAX, ESI
0x808c837: POP ESI
0x808c838: POP EDI
0x808c839: LEAVE
0x808c83a: RET
```

## pdb_allocateMemory @ 0x80ce784

Evidence: `ev_1a374a433e9a70d5821149d3a5ae4292e2b5de6701e3f2a14cc98b37bef78f48`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80ce86c", "start": "0x80ce784"}, {"end": "0x80ce98c", "start": "0x80ce870"}, {"end": "0x80ce9d0", "start": "0x80ce990"}, {"end": "0x80cea14", "start": "0x80ce9d4"}, {"end": "0x80cea31", "start": "0x80cea18"}], "span_bytes": 686, "total_bytes": 674}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

/* WARNING: Unknown calling convention */

void * pdb_allocateMemory(uint size,char *moduleName,char *file,uint line)

{
  byte bVar1;
  uint uVar2;
  uint *puVar3;
  uint *puVar4;
  int iVar5;
  uint nGrpNo;
  
                    /* Unresolved local var: Pdb_Memory_Grp_T * pGrp@[DW_OP_reg0(EAX)]
                       Unresolved local var: Pdb_Memory_Header_T * pBlock@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint i@[DW_OP_reg7(EDI)] */
  if (file == (char *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","file != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x2ca);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  if (moduleName == (char *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","moduleName != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x2cb);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  puVar3 = (uint *)0x0;
  if (_initMemMgrFlag != 0) {
    _vm_enterCriticalSection(&_memMgrSection);
    nGrpNo = 0;
    if (_pdbBlockGroups != 0) {
      iVar5 = 0;
      do {
        if (size <= *(uint *)((int)&_pMemoryGrp[0].blockSize + iVar5)) {
          puVar4 = *(uint **)((int)&_pMemoryGrp[0].pFree + iVar5);
          if (puVar4 == (uint *)0x0) {
            pdb_extendBlocksOfGroup
                      ((Pdb_Memory_Grp_T *)((int)&_pMemoryGrp[0].blockSize + iVar5),nGrpNo);
            puVar4 = *(uint **)((int)&_pMemoryGrp[0].pFree + iVar5);
            if (puVar4 == (uint *)0x0) break;
          }
          bVar1 = *(byte *)((int)puVar4 + 6);
          *(uint *)((int)&_pMemoryGrp[0].pFree + iVar5) = puVar4[2];
          *(undefined1 *)((int)puVar4 + 5) = 0;
          if ((bVar1 & 0x10) != 0) {
            pdb_syslogf(2,4,"assertion failed: %s\n","! (pBlock->flags & PDB_MEMORY_ALLOCATED)");
            pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                        "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x2f5);
            _vm_endProcess(0);
            do {
                    /* WARNING: Do nothing block with infinite loop */
            } while( true );
          }
          if (puVar4 != (uint *)0x0) goto LAB_080ce89f;
          break;
        }
        nGrpNo = nGrpNo + 1;
        iVar5 = iVar5 + 0x1c;
      } while (nGrpNo < _pdbBlockGroups);
    }
                    /* Unresolved local var: Pdb_Memory_Header_T * pBlock@[DW_OP_reg1(ECX)]
                       Unresolved local var: uint nSize@[DW_OP_reg2(EDX)] */
    puVar4 = (uint *)0x0;
    if (_initMemMgrFlag != 0) {
      uVar2 = size;
      if ((nGrpNo < _pdbBlockGroups) && (uVar2 = _pMemoryGrp[nGrpNo].blockSize, uVar2 < size)) {
        pdb_syslogf(2,4,"assertion failed: %s\n","_pMemoryGrp[nGrpNo].blockSize >= size");
        pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                    "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x82b);
        _vm_endProcess(0);
        do {
                    /* WARNING: Do nothing block with infinite loop */
        } while( true );
      }
      puVar3 = malloc(uVar2 + 0x10);
      puVar4 = (uint *)0x0;
      if (puVar3 != (uint *)0x0) {
        _pdb_totalMemSize = size + _pdb_totalMemSize + 0xc;
        if (_pdb_peakMemSize < _pdb_totalMemSize) {
          _pdb_peakMemSize = _pdb_totalMemSize;
        }
        _pdb_totalL2Size = _pdb_totalL2Size + size;
        *(undefined2 *)((int)puVar3 + 6) = 0;
        *puVar3 = size;
        *(undefined1 *)((int)puVar3 + size + 0xc) = 0xaf;
        *(char *)(puVar3 + 1) = (char)nGrpNo;
        puVar4 = puVar3;
        if (nGrpNo < _pdbBlockGroups) {
          *(ushort *)((int)puVar3 + 6) = *(ushort *)((int)puVar3 + 6) | 2;
        }
      }
    }
    if (puVar4 != (uint *)0x0) {
LAB_080ce89f:
      *(ushort *)((int)puVar4 + 6) = *(ushort *)((int)puVar4 + 6) | 0x10;
    }
    _vm_leaveCriticalSection(&_memMgrSection);
    puVar3 = (uint *)0x0;
    if (puVar4 != (uint *)0x0) {
      *puVar4 = size;
      *(undefined1 *)((int)puVar4 + size + 0xc) = 0xaf;
      _pdb_memStatAlloc = _pdb_memStatAlloc + 1;
      _pdb_totalUsedSize = _pdb_totalUsedSize + size;
      puVar3 = puVar4 + 3;
    }
  }
  return puVar3;
}


```

```text
0x80ce784: PUSH EBP
0x80ce785: MOV EBP, ESP
0x80ce787: PUSH EDI
0x80ce788: PUSH ESI
0x80ce789: PUSH EBX
0x80ce78a: SUB ESP, 0xc
0x80ce78d: MOV EDI, dword ptr [EBP + 0x10]
0x80ce790: TEST EDI, EDI
0x80ce792: JZ 0x080ce990
0x80ce798: MOV ECX, dword ptr [EBP + 0xc]
0x80ce79b: TEST ECX, ECX
0x80ce79d: JZ 0x080ce94c
0x80ce7a3: MOV EDI, dword ptr [0x0860cff0]
0x80ce7a9: XOR EAX, EAX
0x80ce7ab: TEST EDI, EDI
0x80ce7ad: JZ 0x080ce8d3
0x80ce7b3: SUB ESP, 0xc
0x80ce7b6: PUSH 0x860d158
0x80ce7bb: CALL 0x080a3c7c
0x80ce7c0: MOV EDX, dword ptr [0x083d0150]
0x80ce7c6: XOR EDI, EDI
0x80ce7c8: ADD ESP, 0x10
0x80ce7cb: TEST EDX, EDX
0x80ce7cd: JZ 0x080ce7ee
0x80ce7cf: XOR ESI, ESI
0x80ce7d1: MOV ECX, dword ptr [EBP + 0x8]
0x80ce7d4: CMP dword ptr [ESI + 0x860d040], ECX
0x80ce7da: LEA EAX, [ESI + 0x860d040]
0x80ce7e0: JNC 0x080ce870
0x80ce7e6: INC EDI
0x80ce7e7: ADD ESI, 0x1c
0x80ce7ea: CMP EDI, EDX
0x80ce7ec: JC 0x080ce7d1
0x80ce7ee: MOV ECX, dword ptr [0x0860cff0]
0x80ce7f4: XOR EAX, EAX
0x80ce7f6: TEST ECX, ECX
0x80ce7f8: JZ 0x080ce93d
0x80ce7fe: MOV EDX, dword ptr [EBP + 0x8]
0x80ce801: ADD EDX, 0xc
0x80ce804: CMP EDI, dword ptr [0x083d0150]
0x80ce80a: JNC 0x080ce8db
0x80ce810: LEA EAX, [EDI*0x8 + 0x0]
0x80ce817: SUB EAX, EDI
0x80ce819: MOV EAX, dword ptr [EAX*0x4 + 0x860d040]
0x80ce820: CMP EAX, dword ptr [EBP + 0x8]
0x80ce823: LEA EDX, [EAX + 0xc]
0x80ce826: JNC 0x080ce8db
0x80ce82c: PUSH 0x82b5b60
0x80ce831: PUSH 0x82b5ab5
0x80ce836: PUSH 0x4
0x80ce838: PUSH 0x2
0x80ce83a: CALL 0x080cc868
0x80ce83f: POP EAX
0x80ce840: POP EDX
0x80ce841: PUSH 0x82b
0x80ce846: PUSH 0x82b5b34
0x80ce84b: PUSH 0x82b5acb
0x80ce850: PUSH 0x82a825c
0x80ce855: PUSH 0x4
0x80ce857: PUSH 0x2
0x80ce859: CALL 0x080cc868
0x80ce85e: ADD ESP, 0x14
0x80ce861: PUSH 0x0
0x80ce863: CALL 0x080a3980
0x80ce868: ADD ESP, 0x10
0x80ce86b: JMP 0x080ce86b
0x80ce870: MOV EDX, dword ptr [ESI + 0x860d054]
0x80ce876: TEST EDX, EDX
0x80ce878: JZ 0x080cea18
0x80ce87e: MOV EAX, dword ptr [EDX + 0x8]
0x80ce881: TEST byte ptr [EDX + 0x6], 0x10
0x80ce885: MOV dword ptr [ESI + 0x860d054], EAX
0x80ce88b: MOV EBX, EDX
0x80ce88d: MOV byte ptr [EDX + 0x5], 0x0
0x80ce891: JNZ 0x080ce9d4
0x80ce897: TEST EDX, EDX
0x80ce899: JZ 0x080ce7ee
0x80ce89f: OR word ptr [EBX + 0x6], 0x10
0x80ce8a4: SUB ESP, 0xc
0x80ce8a7: PUSH 0x860d158
0x80ce8ac: CALL 0x080a3cc8
0x80ce8b1: ADD ESP, 0x10
0x80ce8b4: XOR EAX, EAX
0x80ce8b6: TEST EBX, EBX
0x80ce8b8: JZ 0x080ce8d3
0x80ce8ba: MOV ECX, dword ptr [EBP + 0x8]
0x80ce8bd: MOV dword ptr [EBX], ECX
0x80ce8bf: MOV byte ptr [EBX + ECX*0x1 + 0xc], 0xaf
0x80ce8c4: INC dword ptr [0x0860d01c]
0x80ce8ca: ADD dword ptr [0x0860d010], ECX
0x80ce8d0: LEA EAX, [EBX + 0xc]
0x80ce8d3: LEA ESP, [EBP + -0xc]
0x80ce8d6: POP EBX
0x80ce8d7: POP ESI
0x80ce8d8: POP EDI
0x80ce8d9: LEAVE
0x80ce8da: RET
0x80ce8db: SUB ESP, 0xc
0x80ce8de: ADD EDX, 0x4
0x80ce8e1: PUSH EDX
0x80ce8e2: CALL 0x0804b354
0x80ce8e7: MOV ECX, EAX
0x80ce8e9: ADD ESP, 0x10
0x80ce8ec: XOR EAX, EAX
0x80ce8ee: TEST ECX, ECX
0x80ce8f0: JZ 0x080ce93d
0x80ce8f2: MOV EDX, dword ptr [EBP + 0x8]
0x80ce8f5: ADD EDX, dword ptr [0x0860d014]
0x80ce8fb: MOV EAX, [0x0860d008]
0x80ce900: ADD EDX, 0xc
0x80ce903: CMP EDX, EAX
0x80ce905: CMOVA EAX, EDX
0x80ce908: MOV [0x0860d008], EAX
0x80ce90d: MOV EAX, dword ptr [EBP + 0x8]
0x80ce910: ADD dword ptr [0x0860d00c], EAX
0x80ce916: MOV dword ptr [0x0860d014], EDX
0x80ce91c: MOV word ptr [ECX + 0x6], 0x0
0x80ce922: MOV dword ptr [ECX], EAX
0x80ce924: MOV byte ptr [ECX + EAX*0x1 + 0xc], 0xaf
0x80ce929: MOV EAX, EDI
0x80ce92b: MOV byte ptr [ECX + 0x4], AL
0x80ce92e: CMP EDI, dword ptr [0x083d0150]
0x80ce934: JNC 0x080ce93b
0x80ce936: OR word ptr [ECX + 0x6], 0x2
0x80ce93b: MOV EAX, ECX
0x80ce93d: TEST EAX, EAX
0x80ce93f: MOV EBX, EAX
0x80ce941: JNZ 0x080ce89f
0x80ce947: JMP 0x080ce8a4
0x80ce94c: PUSH 0x82b5f93
0x80ce951: PUSH 0x82b5ab5
0x80ce956: PUSH 0x4
0x80ce958: PUSH 0x2
0x80ce95a: CALL 0x080cc868
0x80ce95f: POP EAX
0x80ce960: POP EDX
0x80ce961: PUSH 0x2cb
0x80ce966: PUSH 0x82b5b34
0x80ce96b: PUSH 0x82b5acb
0x80ce970: PUSH 0x82a825c
0x80ce975: PUSH 0x4
0x80ce977: PUSH 0x2
0x80ce979: CALL 0x080cc868
0x80ce97e: ADD ESP, 0x14
0x80ce981: PUSH 0x0
0x80ce983: CALL 0x080a3980
0x80ce988: ADD ESP, 0x10
0x80ce98b: JMP 0x080ce98b
0x80ce990: PUSH 0x82b5fa6
0x80ce995: PUSH 0x82b5ab5
0x80ce99a: PUSH 0x4
0x80ce99c: PUSH 0x2
0x80ce99e: CALL 0x080cc868
0x80ce9a3: POP EBX
0x80ce9a4: POP ESI
0x80ce9a5: PUSH 0x2ca
0x80ce9aa: PUSH 0x82b5b34
0x80ce9af: PUSH 0x82b5acb
0x80ce9b4: PUSH 0x82a825c
0x80ce9b9: PUSH 0x4
0x80ce9bb: PUSH 0x2
0x80ce9bd: CALL 0x080cc868
0x80ce9c2: ADD ESP, 0x14
0x80ce9c5: PUSH 0x0
0x80ce9c7: CALL 0x080a3980
0x80ce9cc: ADD ESP, 0x10
0x80ce9cf: JMP 0x080ce9cf
0x80ce9d4: PUSH 0x82b5b88
0x80ce9d9: PUSH 0x82b5ab5
0x80ce9de: PUSH 0x4
0x80ce9e0: PUSH 0x2
0x80ce9e2: CALL 0x080cc868
0x80ce9e7: POP EBX
0x80ce9e8: POP ESI
0x80ce9e9: PUSH 0x2f5
0x80ce9ee: PUSH 0x82b5b34
0x80ce9f3: PUSH 0x82b5acb
0x80ce9f8: PUSH 0x82a825c
0x80ce9fd: PUSH 0x4
0x80ce9ff: PUSH 0x2
0x80cea01: CALL 0x080cc868
0x80cea06: ADD ESP, 0x14
0x80cea09: PUSH 0x0
0x80cea0b: CALL 0x080a3980
0x80cea10: ADD ESP, 0x10
0x80cea13: JMP 0x080cea13
0x80cea18: MOV EDX, EDI
0x80cea1a: CALL 0x080ce60c
0x80cea1f: MOV EDX, dword ptr [ESI + 0x860d054]
0x80cea25: TEST EDX, EDX
0x80cea27: JZ 0x080ce7ee
0x80cea2d: JMP 0x080ce87e
```

## <EXTERNAL>::strcmp @ 0x804b064

Evidence: `ev_33be0a61bad0f87ae5aadb6450c2cfe45eff8500361f60c1846e8a19b2953a6b`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x804b069", "start": "0x804b064"}], "span_bytes": 6, "total_bytes": 6}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "Ghidra Listing reports a terminal call at 0x804b064 to 0x866d090 <EXTERNAL>::strcmp (external=false, thunk=true, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int strcmp(char *__s1,char *__s2)

{
  int iVar1;
  
  iVar1 = strcmp(__s1,__s2);
  return iVar1;
}


```

```text
0x804b064: JMP dword ptr [0x083c8888]
```

## pdb_reallocateMemory @ 0x80cf588

Evidence: `ev_342684c2c2f42c0fe982ade201a648a6b726e3faf2f9b235155693a960875967`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80cfc40", "start": "0x80cf588"}], "span_bytes": 1721, "total_bytes": 1721}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

void * pdb_reallocateMemory(void *ptr,uint size,char *moduleName,char *file,uint line)

{
  Pdb_Memory_Grp_T *pGrp;
  byte bVar1;
  uint uVar2;
  Pdb_Memory_Header_T *pPVar3;
  uint uVar4;
  Pdb_Memory_Header_T *pPVar5;
  uint uVar6;
  Pdb_Memory_Header_T *pPVar7;
  Pdb_Memory_Header_T *pPVar8;
  int iVar9;
  uint *puVar10;
  uint *puVar11;
  bool bVar12;
  uint i;
  Pdb_Memory_Header_T *pBlock;
  Pdb_Memory_Grp_T *pOldGrp;
  
                    /* Unresolved local var: Pdb_Memory_Grp_T * pNewGrp@[DW_OP_reg0(EAX)]
                       Unresolved local var: Pdb_Memory_Header_T * pNewBlock@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint prevSize@[???] */
  if (file == (char *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","file != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x354);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  if (moduleName == (char *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","moduleName != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x355);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  if (_initMemMgrFlag == 0) {
    return (void *)0x0;
  }
  _vm_enterCriticalSection(&_memMgrSection);
  if (ptr == (void *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","ptr != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x35d);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  pPVar3 = (Pdb_Memory_Header_T *)((int)ptr + -0xc);
  if (*(char *)((int)&pPVar3[1].size + *(int *)((int)ptr + -0xc)) != -0x51) {
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad block (Tail flag is error 1).\n",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x367);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  if ((*(ushort *)((int)ptr + -6) & 0x10) == 0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","pBlock->flags & PDB_MEMORY_ALLOCATED");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x36b);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  uVar2 = pPVar3->size;
  if ((*(ushort *)((int)ptr + -6) & 0xb) == 0) {
    if ((_pdbBlockGroups == 0) || ((&_pdb_extendGrpList)[_pdbBlockGroups * 7] <= size)) {
      i = _pdbBlockGroups;
    }
    else {
      i = 0;
      if (_pdbBlockGroups != 0) {
        iVar9 = 0;
LAB_080cf7fa:
        if (*(uint *)((int)&_pMemoryGrp[0].blockSize + iVar9) < size) goto code_r0x080cf809;
        pPVar5 = *(Pdb_Memory_Header_T **)((int)&_pMemoryGrp[0].pFree + iVar9);
        if (pPVar5 != (Pdb_Memory_Header_T *)0x0) {
          *(Pdb_Memory_Header **)((int)&_pMemoryGrp[0].pFree + iVar9) = pPVar5->pNext;
          uVar4 = size;
          if (uVar2 <= size) {
            uVar4 = uVar2;
          }
          puVar10 = ptr;
          pPVar8 = pPVar5 + 1;
          for (uVar6 = uVar4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            pPVar8->size = *puVar10;
            puVar10 = puVar10 + 1;
            pPVar8 = (Pdb_Memory_Header_T *)&pPVar8->serial;
          }
          puVar11 = puVar10;
          pPVar7 = pPVar8;
          if ((uVar4 & 2) != 0) {
            pPVar7 = (Pdb_Memory_Header_T *)((int)&pPVar8->size + 2);
            puVar11 = (uint *)((int)puVar10 + 2);
            *(short *)&pPVar8->size = (short)*puVar10;
          }
          if ((uVar4 & 1) != 0) {
            *(char *)&pPVar7->size = (char)*puVar11;
          }
          *(ushort *)((int)ptr + -6) = *(ushort *)((int)ptr + -6) & 0xffef;
          pdb_l2FreeMemory(pPVar3);
          if (pPVar5 == (Pdb_Memory_Header_T *)0x0) goto LAB_080cf814;
          goto LAB_080cf6fb;
        }
      }
    }
LAB_080cf814:
                    /* Unresolved local var: uint nSize@[DW_OP_reg2(EDX)]
                       Unresolved local var: Pdb_Memory_Header_T * pNewBlock@[DW_OP_reg6(ESI)] */
    if (_initMemMgrFlag == 0) {
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","PDB is not initialized.\n",
                  "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x866);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    if (pPVar3 == (Pdb_Memory_Header_T *)0x0) {
      pdb_syslogf(2,4,"assertion failed: %s\n","pBlock != NULL");
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                  "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x86a);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    if (0x7ffffff1 < size) {
      pdb_syslogf(2,4,"assertion failed: %s\n","size < 0x7FFFFFFF - sizeof(Pdb_Memory_Header_T) - 1"
                 );
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                  "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x86b);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    uVar4 = size;
    if ((i < _pdbBlockGroups) && (uVar4 = _pMemoryGrp[i].blockSize, uVar4 < size)) {
      pdb_syslogf(2,4,"assertion failed: %s\n","_pMemoryGrp[nGrpNo].blockSize >= size");
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                  "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x875);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    if (*(char *)((int)&pPVar3[1].size + pPVar3->size) != -0x51) {
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad block (Tail flag is error 3).\n"
                  ,"/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x88c);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    pPVar8 = realloc(pPVar3,uVar4 + 0x10);
    pPVar5 = pPVar3;
    if (pPVar8 != (Pdb_Memory_Header_T *)0x0) {
      iVar9 = size - pPVar8->size;
      _pdb_totalMemSize = iVar9 + _pdb_totalMemSize;
      if (_pdb_peakMemSize < _pdb_totalMemSize) {
        _pdb_peakMemSize = _pdb_totalMemSize;
      }
      _pdb_totalL2Size = _pdb_totalL2Size + iVar9;
      pPVar8->serial = (uchar)i;
      bVar12 = i < _pdbBlockGroups;
      pPVar8->size = size;
      if (bVar12) {
        pPVar8->flags = pPVar8->flags | 2;
      }
      *(undefined1 *)((int)&pPVar8[1].size + size) = 0xaf;
      pPVar5 = pPVar8;
    }
  }
  else {
    bVar1 = *(byte *)((int)ptr + -8);
    if (_pdbBlockGroups <= bVar1) {
      pdb_syslogf(2,4,"assertion failed: %s\n","pBlock->serial < _pdbBlockGroups");
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                  "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x37c);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    pPVar8 = (Pdb_Memory_Header_T *)0x0;
    i = 0;
    pPVar5 = pPVar8;
    if (_pdbBlockGroups != 0) {
      iVar9 = 0;
      do {
        pGrp = (Pdb_Memory_Grp_T *)((int)&_pMemoryGrp[0].blockSize + iVar9);
        if (size <= *(uint *)((int)&_pMemoryGrp[0].blockSize + iVar9)) {
          pPVar5 = pPVar3;
          if (_pMemoryGrp + bVar1 == pGrp) break;
          pPVar7 = *(Pdb_Memory_Header_T **)((int)&_pMemoryGrp[0].pFree + iVar9);
          if (pPVar7 == (Pdb_Memory_Header_T *)0x0) {
            pdb_extendBlocksOfGroup(pGrp,i);
            pPVar7 = *(Pdb_Memory_Header_T **)((int)&_pMemoryGrp[0].pFree + iVar9);
            pPVar5 = pPVar8;
            if (pPVar7 == (Pdb_Memory_Header_T *)0x0) break;
          }
          *(Pdb_Memory_Header **)((int)&_pMemoryGrp[0].pFree + iVar9) = pPVar7->pNext;
          pPVar5 = pPVar7;
          break;
        }
        i = i + 1;
        iVar9 = iVar9 + 0x1c;
      } while (i < _pdbBlockGroups);
    }
    if (pPVar5 != pPVar3) {
      if (pPVar5 == (Pdb_Memory_Header_T *)0x0) {
                    /* Unresolved local var: Pdb_Memory_Header_T * pBlock@[DW_OP_reg1(ECX)]
                       Unresolved local var: uint nSize@[DW_OP_reg2(EDX)] */
        pPVar5 = (Pdb_Memory_Header_T *)0x0;
        if (_initMemMgrFlag != 0) {
          uVar4 = size;
          if ((i < _pdbBlockGroups) && (uVar4 = _pMemoryGrp[i].blockSize, uVar4 < size)) {
            pdb_syslogf(2,4,"assertion failed: %s\n","_pMemoryGrp[nGrpNo].blockSize >= size");
            pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                        "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x82b);
            _vm_endProcess(0);
            do {
                    /* WARNING: Do nothing block with infinite loop */
            } while( true );
          }
          pPVar8 = malloc(uVar4 + 0x10);
          pPVar5 = (Pdb_Memory_Header_T *)0x0;
          if (pPVar8 != (Pdb_Memory_Header_T *)0x0) {
            _pdb_totalMemSize = size + _pdb_totalMemSize + 0xc;
            if (_pdb_peakMemSize < _pdb_totalMemSize) {
              _pdb_peakMemSize = _pdb_totalMemSize;
            }
            _pdb_totalL2Size = _pdb_totalL2Size + size;
            pPVar8->flags = 0;
            pPVar8->size = size;
            *(undefined1 *)((int)&pPVar8[1].size + size) = 0xaf;
            pPVar8->serial = (uchar)i;
            pPVar5 = pPVar8;
            if (i < _pdbBlockGroups) {
              pPVar8->flags = pPVar8->flags | 2;
            }
          }
        }
        if (pPVar5 == (Pdb_Memory_Header_T *)0x0) goto LAB_080cf700;
      }
      uVar4 = pPVar3->size;
      if (size < pPVar3->size) {
        uVar4 = size;
      }
      puVar10 = ptr;
      pPVar8 = pPVar5 + 1;
      for (uVar6 = uVar4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        pPVar8->size = *puVar10;
        puVar10 = puVar10 + 1;
        pPVar8 = (Pdb_Memory_Header_T *)&pPVar8->serial;
      }
      puVar11 = puVar10;
      pPVar7 = pPVar8;
      if ((uVar4 & 2) != 0) {
        pPVar7 = (Pdb_Memory_Header_T *)((int)&pPVar8->size + 2);
        puVar11 = (uint *)((int)puVar10 + 2);
        *(short *)&pPVar8->size = (short)*puVar10;
      }
      if ((uVar4 & 1) != 0) {
        *(char *)&pPVar7->size = (char)*puVar11;
      }
      *(Pdb_Memory_Header_T **)((int)ptr + -4) = _pMemoryGrp[bVar1].pFree;
      *(ushort *)((int)ptr + -6) = *(ushort *)((int)ptr + -6) & 0xffef;
      _pMemoryGrp[bVar1].pFree = pPVar3;
    }
  }
  if (pPVar5 != (Pdb_Memory_Header_T *)0x0) {
LAB_080cf6fb:
    pPVar5->flags = pPVar5->flags | 0x10;
  }
LAB_080cf700:
  _vm_leaveCriticalSection(&_memMgrSection);
  pPVar3 = (Pdb_Memory_Header_T *)0x0;
  if (pPVar5 != (Pdb_Memory_Header_T *)0x0) {
    pPVar5->size = size;
    *(undefined1 *)((int)&pPVar5[1].size + size) = 0xaf;
    _pdb_memStatAlloc = _pdb_memStatAlloc + 1;
    _pdb_memStatFree = _pdb_memStatFree + 1;
    _pdb_totalUsedSize = _pdb_totalUsedSize + (size - uVar2);
    pPVar3 = pPVar5 + 1;
  }
  return pPVar3;
code_r0x080cf809:
  i = i + 1;
  iVar9 = iVar9 + 0x1c;
  if (_pdbBlockGroups <= i) goto LAB_080cf814;
  goto LAB_080cf7fa;
}


```

```text
0x80cf588: PUSH EBP
0x80cf589: MOV EBP, ESP
0x80cf58b: PUSH EDI
0x80cf58c: PUSH ESI
0x80cf58d: PUSH EBX
0x80cf58e: SUB ESP, 0x1c
0x80cf591: MOV ECX, dword ptr [EBP + 0x14]
0x80cf594: TEST ECX, ECX
0x80cf596: MOV EBX, dword ptr [EBP + 0x8]
0x80cf599: JZ 0x080cf783
0x80cf59f: MOV EDI, dword ptr [EBP + 0x10]
0x80cf5a2: TEST EDI, EDI
0x80cf5a4: JZ 0x080cf742
0x80cf5aa: MOV ECX, dword ptr [0x0860cff0]
0x80cf5b0: XOR EAX, EAX
0x80cf5b2: TEST ECX, ECX
0x80cf5b4: JZ 0x080cf73a
0x80cf5ba: SUB ESP, 0xc
0x80cf5bd: PUSH 0x860d158
0x80cf5c2: CALL 0x080a3c7c
0x80cf5c7: ADD ESP, 0x10
0x80cf5ca: TEST EBX, EBX
0x80cf5cc: JZ 0x080cf8e9
0x80cf5d2: LEA EAX, [EBX + -0xc]
0x80cf5d5: MOV dword ptr [EBP + -0x14], EAX
0x80cf5d8: MOV EDX, dword ptr [EBP + -0x14]
0x80cf5db: MOV EAX, dword ptr [EBX + -0xc]
0x80cf5de: CMP byte ptr [EAX + EDX*0x1 + 0xc], 0xaf
0x80cf5e3: JZ 0x080cf614
0x80cf5e5: SUB ESP, 0x8
0x80cf5e8: PUSH 0x367
0x80cf5ed: PUSH 0x82b5b34
0x80cf5f2: PUSH 0x82b5ecc
0x80cf5f7: PUSH 0x82a825c
0x80cf5fc: PUSH 0x4
0x80cf5fe: PUSH 0x2
0x80cf600: CALL 0x080cc868
0x80cf605: ADD ESP, 0x14
0x80cf608: PUSH 0x0
0x80cf60a: CALL 0x080a3980
0x80cf60f: ADD ESP, 0x10
0x80cf612: JMP 0x080cf612
0x80cf614: MOV ECX, dword ptr [EBP + -0x14]
0x80cf617: MOV AX, word ptr [ECX + 0x6]
0x80cf61b: TEST AL, 0x10
0x80cf61d: JZ 0x080cf8a8
0x80cf623: MOV EDX, dword ptr [EBP + -0x14]
0x80cf626: MOV EDX, dword ptr [EDX]
0x80cf628: TEST AL, 0xb
0x80cf62a: MOV dword ptr [EBP + -0x1c], EDX
0x80cf62d: JZ 0x080cf7c4
0x80cf633: MOV ECX, dword ptr [EBP + -0x14]
0x80cf636: MOV DL, byte ptr [ECX + 0x4]
0x80cf639: MOVZX EAX, DL
0x80cf63c: MOV ECX, dword ptr [0x083d0150]
0x80cf642: CMP EAX, ECX
0x80cf644: JNC 0x080cf945
0x80cf64a: MOVZX EDX, DL
0x80cf64d: LEA EAX, [EDX*0x8 + 0x0]
0x80cf654: SUB EAX, EDX
0x80cf656: XOR EBX, EBX
0x80cf658: LEA EAX, [EAX*0x4 + 0x860d040]
0x80cf65f: CMP EBX, ECX
0x80cf661: MOV dword ptr [EBP + -0x10], EAX
0x80cf664: MOV dword ptr [EBP + -0x18], 0x0
0x80cf66b: JNC 0x080cf6af
0x80cf66d: XOR ESI, ESI
0x80cf66f: MOV EDX, dword ptr [EBP + 0xc]
0x80cf672: CMP dword ptr [ESI + 0x860d040], EDX
0x80cf678: LEA EAX, [ESI + 0x860d040]
0x80cf67e: JNC 0x080cf68d
0x80cf680: INC dword ptr [EBP + -0x18]
0x80cf683: ADD ESI, 0x1c
0x80cf686: CMP dword ptr [EBP + -0x18], ECX
0x80cf689: JC 0x080cf66f
0x80cf68b: JMP 0x080cf6af
0x80cf68d: CMP dword ptr [EBP + -0x10], EAX
0x80cf690: JZ 0x080cfb07
0x80cf696: MOV EDX, dword ptr [ESI + 0x860d054]
0x80cf69c: TEST EDX, EDX
0x80cf69e: JZ 0x080cf92a
0x80cf6a4: MOV EBX, EDX
0x80cf6a6: MOV EDX, dword ptr [EDX + 0x8]
0x80cf6a9: MOV dword ptr [ESI + 0x860d054], EDX
0x80cf6af: CMP EBX, dword ptr [EBP + -0x14]
0x80cf6b2: JZ 0x080cf6f7
0x80cf6b4: TEST EBX, EBX
0x80cf6b6: JZ 0x080cfb0f
0x80cf6bc: MOV EDX, dword ptr [EBP + -0x14]
0x80cf6bf: MOV EAX, dword ptr [EDX]
0x80cf6c1: CMP EAX, dword ptr [EBP + 0xc]
0x80cf6c4: CMOVA EAX, dword ptr [EBP + 0xc]
0x80cf6c8: MOV ESI, EDX
0x80cf6ca: MOV ECX, EAX
0x80cf6cc: ADD ESI, 0xc
0x80cf6cf: SHR ECX, 0x2
0x80cf6d2: LEA EDI, [EBX + 0xc]
0x80cf6d5: CLD
0x80cf6d6: TEST AL, 0x2
0x80cf6d8: MOVSD.REP ES:EDI, ESI
0x80cf6da: JZ 0x080cf6de
0x80cf6dc: MOVSW ES:EDI, ESI
0x80cf6de: TEST AL, 0x1
0x80cf6e0: JZ 0x080cf6e3
0x80cf6e2: MOVSB ES:EDI, ESI
0x80cf6e3: MOV ECX, dword ptr [EBP + -0x10]
0x80cf6e6: MOV EDX, dword ptr [EBP + -0x14]
0x80cf6e9: MOV EAX, dword ptr [ECX + 0x14]
0x80cf6ec: MOV dword ptr [EDX + 0x8], EAX
0x80cf6ef: AND word ptr [EDX + 0x6], 0xffef
0x80cf6f4: MOV dword ptr [ECX + 0x14], EDX
0x80cf6f7: TEST EBX, EBX
0x80cf6f9: JZ 0x080cf700
0x80cf6fb: OR word ptr [EBX + 0x6], 0x10
0x80cf700: SUB ESP, 0xc
0x80cf703: PUSH 0x860d158
0x80cf708: CALL 0x080a3cc8
0x80cf70d: ADD ESP, 0x10
0x80cf710: XOR EAX, EAX
0x80cf712: TEST EBX, EBX
0x80cf714: JZ 0x080cf73a
0x80cf716: MOV EDX, dword ptr [EBP + 0xc]
0x80cf719: MOV dword ptr [EBX], EDX
0x80cf71b: MOV byte ptr [EBX + EDX*0x1 + 0xc], 0xaf
0x80cf720: MOV ECX, dword ptr [EBP + -0x1c]
0x80cf723: SUB EDX, ECX
0x80cf725: INC dword ptr [0x0860d01c]
0x80cf72b: INC dword ptr [0x0860d018]
0x80cf731: ADD dword ptr [0x0860d010], EDX
0x80cf737: LEA EAX, [EBX + 0xc]
0x80cf73a: LEA ESP, [EBP + -0xc]
0x80cf73d: POP EBX
0x80cf73e: POP ESI
0x80cf73f: POP EDI
0x80cf740: LEAVE
0x80cf741: RET
0x80cf742: PUSH 0x82b5f93
0x80cf747: PUSH 0x82b5ab5
0x80cf74c: PUSH 0x4
0x80cf74e: PUSH 0x2
0x80cf750: CALL 0x080cc868
0x80cf755: POP EBX
0x80cf756: POP ESI
0x80cf757: PUSH 0x355
0x80cf75c: PUSH 0x82b5b34
0x80cf761: PUSH 0x82b5acb
0x80cf766: PUSH 0x82a825c
0x80cf76b: PUSH 0x4
0x80cf76d: PUSH 0x2
0x80cf76f: CALL 0x080cc868
0x80cf774: ADD ESP, 0x14
0x80cf777: PUSH 0x0
0x80cf779: CALL 0x080a3980
0x80cf77e: ADD ESP, 0x10
0x80cf781: JMP 0x080cf781
0x80cf783: PUSH 0x82b5fa6
0x80cf788: PUSH 0x82b5ab5
0x80cf78d: PUSH 0x4
0x80cf78f: PUSH 0x2
0x80cf791: CALL 0x080cc868
0x80cf796: POP EAX
0x80cf797: POP EDX
0x80cf798: PUSH 0x354
0x80cf79d: PUSH 0x82b5b34
0x80cf7a2: PUSH 0x82b5acb
0x80cf7a7: PUSH 0x82a825c
0x80cf7ac: PUSH 0x4
0x80cf7ae: PUSH 0x2
0x80cf7b0: CALL 0x080cc868
0x80cf7b5: ADD ESP, 0x14
0x80cf7b8: PUSH 0x0
0x80cf7ba: CALL 0x080a3980
0x80cf7bf: ADD ESP, 0x10
0x80cf7c2: JMP 0x080cf7c2
0x80cf7c4: MOV ECX, dword ptr [0x083d0150]
0x80cf7ca: XOR EBX, EBX
0x80cf7cc: TEST ECX, ECX
0x80cf7ce: JZ 0x080cfabe
0x80cf7d4: LEA EAX, [ECX*0x8 + 0x0]
0x80cf7db: SUB EAX, ECX
0x80cf7dd: MOV EDX, dword ptr [EBP + 0xc]
0x80cf7e0: CMP dword ptr [EAX*0x4 + 0x860d024], EDX
0x80cf7e7: JBE 0x080cfabe
0x80cf7ed: CMP EBX, ECX
0x80cf7ef: MOV dword ptr [EBP + -0x18], 0x0
0x80cf7f6: JNC 0x080cf814
0x80cf7f8: XOR EDX, EDX
0x80cf7fa: MOV EAX, dword ptr [EBP + 0xc]
0x80cf7fd: CMP dword ptr [EDX + 0x860d040], EAX
0x80cf803: JNC 0x080cf986
0x80cf809: INC dword ptr [EBP + -0x18]
0x80cf80c: ADD EDX, 0x1c
0x80cf80f: CMP dword ptr [EBP + -0x18], ECX
0x80cf812: JC 0x080cf7fa
0x80cf814: MOV EAX, [0x0860cff0]
0x80cf819: TEST EAX, EAX
0x80cf81b: MOV EBX, dword ptr [EBP + -0x14]
0x80cf81e: JZ 0x080cfa8f
0x80cf824: MOV EAX, dword ptr [EBP + -0x14]
0x80cf827: TEST EAX, EAX
0x80cf829: JZ 0x080cfbbf
0x80cf82f: CMP dword ptr [EBP + 0xc], 0x7ffffff1
0x80cf836: JA 0x080cfa4e
0x80cf83c: MOV EDX, dword ptr [EBP + 0xc]
0x80cf83f: MOV EAX, dword ptr [EBP + -0x18]
0x80cf842: ADD EDX, 0xc
0x80cf845: CMP EAX, dword ptr [0x083d0150]
0x80cf84b: JNC 0x080cf866
0x80cf84d: SHL EAX, 0x3
0x80cf850: SUB EAX, dword ptr [EBP + -0x18]
0x80cf853: MOV EAX, dword ptr [EAX*0x4 + 0x860d040]
0x80cf85a: CMP EAX, dword ptr [EBP + 0xc]
0x80cf85d: LEA EDX, [EAX + 0xc]
0x80cf860: JC 0x080cfac6
0x80cf866: MOV ECX, dword ptr [EBP + -0x14]
0x80cf869: MOV EAX, dword ptr [ECX]
0x80cf86b: ADD EDX, 0x4
0x80cf86e: CMP byte ptr [EAX + ECX*0x1 + 0xc], 0xaf
0x80cf873: JZ 0x080cf9e3
0x80cf879: SUB ESP, 0x8
0x80cf87c: PUSH 0x88c
0x80cf881: PUSH 0x82b5b34
0x80cf886: PUSH 0x82b5ef0
0x80cf88b: PUSH 0x82a825c
0x80cf890: PUSH 0x4
0x80cf892: PUSH 0x2
0x80cf894: CALL 0x080cc868
0x80cf899: ADD ESP, 0x14
0x80cf89c: PUSH 0x0
0x80cf89e: CALL 0x080a3980
0x80cf8a3: ADD ESP, 0x10
0x80cf8a6: JMP 0x080cf8a6
0x80cf8a8: PUSH 0x82b5c48
0x80cf8ad: PUSH 0x82b5ab5
0x80cf8b2: PUSH 0x4
0x80cf8b4: PUSH 0x2
0x80cf8b6: CALL 0x080cc868
0x80cf8bb: POP EDI
0x80cf8bc: POP EAX
0x80cf8bd: PUSH 0x36b
0x80cf8c2: PUSH 0x82b5b34
0x80cf8c7: PUSH 0x82b5acb
0x80cf8cc: PUSH 0x82a825c
0x80cf8d1: PUSH 0x4
0x80cf8d3: PUSH 0x2
0x80cf8d5: CALL 0x080cc868
0x80cf8da: ADD ESP, 0x14
0x80cf8dd: PUSH 0x0
0x80cf8df: CALL 0x080a3980
0x80cf8e4: ADD ESP, 0x10
0x80cf8e7: JMP 0x080cf8e7
0x80cf8e9: PUSH 0x82b5fdb
0x80cf8ee: PUSH 0x82b5ab5
0x80cf8f3: PUSH 0x4
0x80cf8f5: PUSH 0x2
0x80cf8f7: CALL 0x080cc868
0x80cf8fc: POP EAX
0x80cf8fd: POP EDX
0x80cf8fe: PUSH 0x35d
0x80cf903: PUSH 0x82b5b34
0x80cf908: PUSH 0x82b5acb
0x80cf90d: PUSH 0x82a825c
0x80cf912: PUSH 0x4
0x80cf914: PUSH 0x2
0x80cf916: CALL 0x080cc868
0x80cf91b: ADD ESP, 0x14
0x80cf91e: PUSH 0x0
0x80cf920: CALL 0x080a3980
0x80cf925: ADD ESP, 0x10
0x80cf928: JMP 0x080cf928
0x80cf92a: MOV EDX, dword ptr [EBP + -0x18]
0x80cf92d: CALL 0x080ce60c
0x80cf932: MOV EDX, dword ptr [ESI + 0x860d054]
0x80cf938: TEST EDX, EDX
0x80cf93a: JZ 0x080cf6af
0x80cf940: JMP 0x080cf6a4
0x80cf945: PUSH 0x82b5c24
0x80cf94a: PUSH 0x82b5ab5
0x80cf94f: PUSH 0x4
0x80cf951: PUSH 0x2
0x80cf953: CALL 0x080cc868
0x80cf958: POP EBX
0x80cf959: POP ESI
0x80cf95a: PUSH 0x37c
0x80cf95f: PUSH 0x82b5b34
0x80cf964: PUSH 0x82b5acb
0x80cf969: PUSH 0x82a825c
0x80cf96e: PUSH 0x4
0x80cf970: PUSH 0x2
0x80cf972: CALL 0x080cc868
0x80cf977: ADD ESP, 0x14
0x80cf97a: PUSH 0x0
0x80cf97c: CALL 0x080a3980
0x80cf981: ADD ESP, 0x10
0x80cf984: JMP 0x080cf984
0x80cf986: MOV EAX, dword ptr [EDX + 0x860d054]
0x80cf98c: TEST EAX, EAX
0x80cf98e: JZ 0x080cf814
0x80cf994: MOV EBX, EAX
0x80cf996: MOV EAX, dword ptr [EAX + 0x8]
0x80cf999: MOV dword ptr [EDX + 0x860d054], EAX
0x80cf99f: MOV EDX, dword ptr [EBP + 0xc]
0x80cf9a2: CMP dword ptr [EBP + -0x1c], EDX
0x80cf9a5: MOV EAX, EDX
0x80cf9a7: CMOVBE EAX, dword ptr [EBP + -0x1c]
0x80cf9ab: MOV ESI, dword ptr [EBP + -0x14]
0x80cf9ae: MOV ECX, EAX
0x80cf9b0: ADD ESI, 0xc
0x80cf9b3: SHR ECX, 0x2
0x80cf9b6: LEA EDI, [EBX + 0xc]
0x80cf9b9: CLD
0x80cf9ba: TEST AL, 0x2
0x80cf9bc: MOVSD.REP ES:EDI, ESI
0x80cf9be: JZ 0x080cf9c2
0x80cf9c0: MOVSW ES:EDI, ESI
0x80cf9c2: TEST AL, 0x1
0x80cf9c4: JZ 0x080cf9c7
0x80cf9c6: MOVSB ES:EDI, ESI
0x80cf9c7: MOV ECX, dword ptr [EBP + -0x14]
0x80cf9ca: AND word ptr [ECX + 0x6], 0xffef
0x80cf9cf: MOV EAX, ECX
0x80cf9d1: CALL 0x080cea34
0x80cf9d6: TEST EBX, EBX
0x80cf9d8: JNZ 0x080cf6fb
0x80cf9de: JMP 0x080cf814
0x80cf9e3: SUB ESP, 0x8
0x80cf9e6: PUSH EDX
0x80cf9e7: PUSH dword ptr [EBP + -0x14]
0x80cf9ea: CALL 0x0804b754
0x80cf9ef: ADD ESP, 0x10
0x80cf9f2: TEST EAX, EAX
0x80cf9f4: MOV ESI, EAX
0x80cf9f6: JZ 0x080cf6f7
0x80cf9fc: MOV ECX, dword ptr [EBP + 0xc]
0x80cf9ff: SUB ECX, dword ptr [EAX]
0x80cfa01: MOV EDX, ECX
0x80cfa03: ADD EDX, dword ptr [0x0860d014]
0x80cfa09: MOV EAX, [0x0860d008]
0x80cfa0e: CMP EDX, EAX
0x80cfa10: CMOVA EAX, EDX
0x80cfa13: MOV dword ptr [0x0860d014], EDX
0x80cfa19: MOV DL, byte ptr [EBP + -0x18]
0x80cfa1c: ADD dword ptr [0x0860d00c], ECX
0x80cfa22: MOV [0x0860d008], EAX
0x80cfa27: MOV EAX, dword ptr [EBP + 0xc]
0x80cfa2a: MOV byte ptr [ESI + 0x4], DL
0x80cfa2d: MOV ECX, dword ptr [EBP + -0x18]
0x80cfa30: CMP ECX, dword ptr [0x083d0150]
0x80cfa36: MOV dword ptr [ESI], EAX
0x80cfa38: JNC 0x080cfa3f
0x80cfa3a: OR word ptr [ESI + 0x6], 0x2
0x80cfa3f: MOV EAX, dword ptr [EBP + 0xc]
0x80cfa42: MOV EBX, ESI
0x80cfa44: MOV byte ptr [ESI + EAX*0x1 + 0xc], 0xaf
0x80cfa49: JMP 0x080cf6f7
0x80cfa4e: PUSH 0x82b5f14
0x80cfa53: PUSH 0x82b5ab5
0x80cfa58: PUSH 0x4
0x80cfa5a: PUSH 0x2
0x80cfa5c: CALL 0x080cc868
0x80cfa61: POP ECX
0x80cfa62: POP EBX
0x80cfa63: PUSH 0x86b
0x80cfa68: PUSH 0x82b5b34
0x80cfa6d: PUSH 0x82b5acb
0x80cfa72: PUSH 0x82a825c
0x80cfa77: PUSH 0x4
0x80cfa79: PUSH 0x2
0x80cfa7b: CALL 0x080cc868
0x80cfa80: ADD ESP, 0x14
0x80cfa83: PUSH 0x0
0x80cfa85: CALL 0x080a3980
0x80cfa8a: ADD ESP, 0x10
0x80cfa8d: JMP 0x080cfa8d
0x80cfa8f: SUB ESP, 0x8
0x80cfa92: PUSH 0x866
0x80cfa97: PUSH 0x82b5b34
0x80cfa9c: PUSH 0x82b5fb3
0x80cfaa1: PUSH 0x82a825c
0x80cfaa6: PUSH 0x4
0x80cfaa8: PUSH 0x2
0x80cfaaa: CALL 0x080cc868
0x80cfaaf: ADD ESP, 0x14
0x80cfab2: PUSH 0x0
0x80cfab4: CALL 0x080a3980
0x80cfab9: ADD ESP, 0x10
0x80cfabc: JMP 0x080cfabc
0x80cfabe: MOV dword ptr [EBP + -0x18], ECX
0x80cfac1: JMP 0x080cf814
0x80cfac6: PUSH 0x82b5b60
0x80cfacb: PUSH 0x82b5ab5
0x80cfad0: PUSH 0x4
0x80cfad2: PUSH 0x2
0x80cfad4: CALL 0x080cc868
0x80cfad9: POP EAX
0x80cfada: POP EDX
0x80cfadb: PUSH 0x875
0x80cfae0: PUSH 0x82b5b34
0x80cfae5: PUSH 0x82b5acb
0x80cfaea: PUSH 0x82a825c
0x80cfaef: PUSH 0x4
0x80cfaf1: PUSH 0x2
0x80cfaf3: CALL 0x080cc868
0x80cfaf8: ADD ESP, 0x14
0x80cfafb: PUSH 0x0
0x80cfafd: CALL 0x080a3980
0x80cfb02: ADD ESP, 0x10
0x80cfb05: JMP 0x080cfb05
0x80cfb07: MOV EBX, dword ptr [EBP + -0x14]
0x80cfb0a: JMP 0x080cf6af
0x80cfb0f: MOV ECX, dword ptr [0x0860cff0]
0x80cfb15: XOR EAX, EAX
0x80cfb17: TEST ECX, ECX
0x80cfb19: JZ 0x080cfbb0
0x80cfb1f: MOV EDX, dword ptr [EBP + 0xc]
0x80cfb22: MOV ECX, dword ptr [EBP + -0x18]
0x80cfb25: ADD EDX, 0xc
0x80cfb28: CMP ECX, dword ptr [0x083d0150]
0x80cfb2e: JNC 0x080cfb4a
0x80cfb30: MOV EAX, ECX
0x80cfb32: SHL EAX, 0x3
0x80cfb35: SUB EAX, ECX
0x80cfb37: MOV EAX, dword ptr [EAX*0x4 + 0x860d040]
0x80cfb3e: CMP EAX, dword ptr [EBP + 0xc]
0x80cfb41: LEA EDX, [EAX + 0xc]
0x80cfb44: JC 0x080cfc00
0x80cfb4a: SUB ESP, 0xc
0x80cfb4d: ADD EDX, 0x4
0x80cfb50: PUSH EDX
0x80cfb51: CALL 0x0804b354
0x80cfb56: MOV ECX, EAX
0x80cfb58: ADD ESP, 0x10
0x80cfb5b: XOR EAX, EAX
0x80cfb5d: TEST ECX, ECX
0x80cfb5f: JZ 0x080cfbb0
0x80cfb61: MOV EDX, dword ptr [EBP + 0xc]
0x80cfb64: ADD EDX, dword ptr [0x0860d014]
0x80cfb6a: MOV EAX, [0x0860d008]
0x80cfb6f: ADD EDX, 0xc
0x80cfb72: CMP EDX, EAX
0x80cfb74: CMOVA EAX, EDX
0x80cfb77: MOV [0x0860d008], EAX
0x80cfb7c: MOV EAX, dword ptr [EBP + 0xc]
0x80cfb7f: ADD dword ptr [0x0860d00c], EAX
0x80cfb85: MOV dword ptr [0x0860d014], EDX
0x80cfb8b: MOV word ptr [ECX + 0x6], 0x0
0x80cfb91: MOV dword ptr [ECX], EAX
0x80cfb93: MOV byte ptr [ECX + EAX*0x1 + 0xc], 0xaf
0x80cfb98: MOV DL, byte ptr [EBP + -0x18]
0x80cfb9b: MOV byte ptr [ECX + 0x4], DL
0x80cfb9e: MOV EAX, dword ptr [EBP + -0x18]
0x80cfba1: CMP EAX, dword ptr [0x083d0150]
0x80cfba7: JNC 0x080cfbae
0x80cfba9: OR word ptr [ECX + 0x6], 0x2
0x80cfbae: MOV EAX, ECX
0x80cfbb0: TEST EAX, EAX
0x80cfbb2: MOV EBX, EAX
0x80cfbb4: JNZ 0x080cf6bc
0x80cfbba: JMP 0x080cf700
0x80cfbbf: PUSH 0x82b5fcc
0x80cfbc4: PUSH 0x82b5ab5
0x80cfbc9: PUSH 0x4
0x80cfbcb: PUSH 0x2
0x80cfbcd: CALL 0x080cc868
0x80cfbd2: POP ESI
0x80cfbd3: POP EDI
0x80cfbd4: PUSH 0x86a
0x80cfbd9: PUSH 0x82b5b34
0x80cfbde: PUSH 0x82b5acb
0x80cfbe3: PUSH 0x82a825c
0x80cfbe8: PUSH 0x4
0x80cfbea: PUSH 0x2
0x80cfbec: CALL 0x080cc868
0x80cfbf1: ADD ESP, 0x14
0x80cfbf4: PUSH 0x0
0x80cfbf6: CALL 0x080a3980
0x80cfbfb: ADD ESP, 0x10
0x80cfbfe: JMP 0x080cfbfe
0x80cfc00: PUSH 0x82b5b60
0x80cfc05: PUSH 0x82b5ab5
0x80cfc0a: PUSH 0x4
0x80cfc0c: PUSH 0x2
0x80cfc0e: CALL 0x080cc868
0x80cfc13: POP EAX
0x80cfc14: POP EDX
0x80cfc15: PUSH 0x82b
0x80cfc1a: PUSH 0x82b5b34
0x80cfc1f: PUSH 0x82b5acb
0x80cfc24: PUSH 0x82a825c
0x80cfc29: PUSH 0x4
0x80cfc2b: PUSH 0x2
0x80cfc2d: CALL 0x080cc868
0x80cfc32: ADD ESP, 0x14
0x80cfc35: PUSH 0x0
0x80cfc37: CALL 0x080a3980
0x80cfc3c: ADD ESP, 0x10
0x80cfc3f: JMP 0x080cfc3f
```

## vm_concatStringWithCStr @ 0x808c83c

Evidence: `ev_36ddefb514f4d1eee68076fb77ba6fd169f82925498e54e17c22e4d514ce716e`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808c88c", "start": "0x808c83c"}], "span_bytes": 81, "total_bytes": 81}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

Vm_String_T * vm_concatStringWithCStr(Vm_String_T *pString,char *cstr)

{
  int len_00;
  char cVar1;
  uint uVar2;
  Vm_String_T *pVVar3;
  uint uVar4;
  char *pcVar5;
  int len;
  
                    /* Unresolved local var: Vm_String_T * dst@[DW_OP_reg3(EBX)]
                       Unresolved local var: int alen@[DW_OP_reg7(EDI)] */
  uVar2 = pString->len;
  uVar4 = 0xffffffff;
  pcVar5 = cstr;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar1 != '\0');
  len_00 = ~uVar4 - 1;
  pVVar3 = vm_extendString(pString,len_00);
  pVVar3->len = pVVar3->len + len_00;
  memcpy(pVVar3->ptr + uVar2,cstr,~uVar4);
  return pVVar3;
}


```

```text
0x808c83c: PUSH EBP
0x808c83d: MOV EBP, ESP
0x808c83f: PUSH EDI
0x808c840: PUSH ESI
0x808c841: PUSH EBX
0x808c842: SUB ESP, 0x14
0x808c845: MOV EDX, dword ptr [EBP + 0x8]
0x808c848: MOV EAX, dword ptr [EDX + 0x10]
0x808c84b: MOV dword ptr [EBP + -0x10], EAX
0x808c84e: MOV EDI, dword ptr [EBP + 0xc]
0x808c851: CLD
0x808c852: XOR EAX, EAX
0x808c854: MOV ECX, 0xffffffff
0x808c859: SCASB.REPNE ES:EDI
0x808c85b: MOV ESI, ECX
0x808c85d: NOT ESI
0x808c85f: LEA EDI, [ESI + -0x1]
0x808c862: PUSH EDI
0x808c863: PUSH EDX
0x808c864: CALL 0x0808c6dc
0x808c869: ADD ESP, 0xc
0x808c86c: ADD dword ptr [EAX + 0x10], EDI
0x808c86f: MOV EBX, EAX
0x808c871: MOV EAX, dword ptr [EAX + 0xc]
0x808c874: PUSH ESI
0x808c875: PUSH dword ptr [EBP + 0xc]
0x808c878: ADD dword ptr [EBP + -0x10], EAX
0x808c87b: PUSH dword ptr [EBP + -0x10]
0x808c87e: CALL 0x0804b7e4
0x808c883: LEA ESP, [EBP + -0xc]
0x808c886: MOV EAX, EBX
0x808c888: POP EBX
0x808c889: POP ESI
0x808c88a: POP EDI
0x808c88b: LEAVE
0x808c88c: RET
```

## vm_removeSharedString @ 0x80bd460

Evidence: `ev_3b41b116e430fa00da08494f00449afb2f46ed0419a2578c4709344c21af0b0b`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80bd494", "start": "0x80bd460"}, {"end": "0x80bd4e2", "start": "0x80bd498"}], "span_bytes": 131, "total_bytes": 128}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "Ghidra Listing reports a terminal call at 0x80bd4d7 to 0x80cf214 pdb_freeTinyBlock (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

void vm_removeSharedString(Vm_String_T *pString)

{
  ushort uVar1;
  uint maxn;
  Vm_Shared_String **ppVVar2;
  Vm_Shared_String_T *p;
  
                    /* Unresolved local var: Vm_Shared_String_T * * ppNode@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Shared_String_T * pNode@[DW_OP_reg3(EBX)]
                       Unresolved local var: int hash@[DW_OP_reg0(EAX)] */
  maxn = pString->len;
  if (0x20 < maxn) {
    maxn = 0x20;
  }
  uVar1 = vm_whashstr(pString->ptr,maxn);
  ppVVar2 = _vm_ppSharedStrings + uVar1;
  p = *ppVVar2;
  while( true ) {
    if (p == (Vm_Shared_String_T *)0x0) {
      return;
    }
    if (p->pString == pString) break;
    ppVVar2 = &p->pNext;
    p = p->pNext;
  }
  if (p == (Vm_Shared_String_T *)0x0) {
    return;
  }
  *ppVVar2 = p->pNext;
  _vm_sharedStringsCount = _vm_sharedStringsCount - 1;
  pdb_freeMemory(p->pString,"VM","/home/cc/xxxx/gameshell/src/vm/vm_shared_string.c",0xf4);
  pdb_freeTinyBlock(p);
  return;
}


```

```text
0x80bd460: PUSH EBP
0x80bd461: MOV EBP, ESP
0x80bd463: PUSH ESI
0x80bd464: PUSH EBX
0x80bd465: MOV ESI, dword ptr [EBP + 0x8]
0x80bd468: MOV EAX, dword ptr [ESI + 0x10]
0x80bd46b: SUB ESP, 0x8
0x80bd46e: MOV EDX, 0x20
0x80bd473: CMP EAX, 0x21
0x80bd476: CMOVNC EAX, EDX
0x80bd479: PUSH EAX
0x80bd47a: PUSH dword ptr [ESI + 0xc]
0x80bd47d: CALL 0x08089dc8
0x80bd482: MOVZX EAX, AX
0x80bd485: MOV EDX, dword ptr [0x0860b040]
0x80bd48b: LEA EDX, [EDX + EAX*0x4]
0x80bd48e: ADD ESP, 0x10
0x80bd491: MOV EBX, dword ptr [EDX]
0x80bd493: JMP 0x080bd49e
0x80bd498: LEA EDX, [EBX + 0x4]
0x80bd49b: MOV EBX, dword ptr [EBX + 0x4]
0x80bd49e: TEST EBX, EBX
0x80bd4a0: JZ 0x080bd4dc
0x80bd4a2: CMP dword ptr [EBX], ESI
0x80bd4a4: JNZ 0x080bd498
0x80bd4a6: TEST EBX, EBX
0x80bd4a8: JZ 0x080bd4dc
0x80bd4aa: MOV EAX, dword ptr [EBX + 0x4]
0x80bd4ad: MOV dword ptr [EDX], EAX
0x80bd4af: DEC dword ptr [0x0860b044]
0x80bd4b5: PUSH 0xf4
0x80bd4ba: PUSH 0x82b2b9c
0x80bd4bf: PUSH 0x82a376b
0x80bd4c4: PUSH dword ptr [EBX]
0x80bd4c6: CALL 0x080ceb94
0x80bd4cb: MOV dword ptr [EBP + 0x8], EBX
0x80bd4ce: ADD ESP, 0x10
0x80bd4d1: LEA ESP, [EBP + -0x8]
0x80bd4d4: POP EBX
0x80bd4d5: POP ESI
0x80bd4d6: LEAVE
0x80bd4d7: JMP 0x080cf214
0x80bd4dc: LEA ESP, [EBP + -0x8]
0x80bd4df: POP EBX
0x80bd4e0: POP ESI
0x80bd4e1: LEAVE
0x80bd4e2: RET
```

## vm_typeValue @ 0x808bd88

Evidence: `ev_5841b7654391f3ff5cb823d82e9f2e1f96f29fb0f3f21b2a0393e7fc70d7b03b`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808bde1", "start": "0x808bd88"}, {"end": "0x808be1a", "start": "0x808bde4"}, {"end": "0x808be22", "start": "0x808be1c"}], "span_bytes": 155, "total_bytes": 152}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

char * vm_typeValue(Vm_Value_T *pValue)

{
  byte bVar1;
  char *pcVar2;
  Vm_Value *pVVar3;
  
                    /* Unresolved local var: char * pTo@[DW_OP_reg0(EAX)] */
  bVar1 = pValue->type;
  if ((bVar1 < 6) || ((7 < bVar1 && (bVar1 != 9)))) {
    vm_checkLoop = 0;
  }
  else {
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)] */
    vm_lastActiveFlag = vm_lastActiveFlag + 1;
    vm_checkLoop = 1;
    if (0xffff < vm_lastActiveFlag) {
      vm_lastActiveFlag = 2;
      for (pVVar3 = vm_allValues; pVVar3 != (Vm_Value_T *)0x0; pVVar3 = pVVar3->pNext) {
        pVVar3->active = 0;
      }
    }
  }
  pcVar2 = vm_typeValueAt(vm_typeValue::gbuffer,0x3fff,pValue,0);
  if (vm_typeValue::gbuffer + 0x3fdf < pcVar2) {
    builtin_strncpy(vm_typeValue::gbuffer + 0x3fe0,"...\n",5);
  }
  return vm_typeValue::gbuffer;
}


```

```text
0x808bd88: PUSH EBP
0x808bd89: MOV EBP, ESP
0x808bd8b: SUB ESP, 0x8
0x808bd8e: MOV EDX, dword ptr [EBP + 0x8]
0x808bd91: MOVZX EAX, byte ptr [EDX + 0x4]
0x808bd95: CMP EAX, 0x6
0x808bd98: JL 0x0808bda4
0x808bd9a: CMP EAX, 0x7
0x808bd9d: JLE 0x0808bde4
0x808bd9f: CMP EAX, 0x9
0x808bda2: JZ 0x0808bde4
0x808bda4: MOV dword ptr [0x0847fd84], 0x0
0x808bdae: PUSH 0x0
0x808bdb0: PUSH EDX
0x808bdb1: PUSH 0x3fff
0x808bdb6: PUSH 0x847fda0
0x808bdbb: CALL 0x0808afc8
0x808bdc0: ADD ESP, 0x10
0x808bdc3: CMP EAX, 0x8483d80
0x808bdc8: JC 0x0808bddb
0x808bdca: MOV dword ptr [0x08483d80], 0xa2e2e2e
0x808bdd4: MOV byte ptr [0x08483d84], 0x0
0x808bddb: MOV EAX, 0x847fda0
0x808bde0: LEAVE
0x808bde1: RET
0x808bde4: MOV EAX, [0x083cc4e0]
0x808bde9: INC EAX
0x808bdea: CMP EAX, 0xffff
0x808bdef: MOV dword ptr [0x0847fd84], 0x1
0x808bdf9: JLE 0x0808be1c
0x808bdfb: MOV EAX, [0x0847fd8c]
0x808be00: MOV dword ptr [0x083cc4e0], 0x2
0x808be0a: JMP 0x0808be15
0x808be0c: MOV word ptr [EAX + 0x6], 0x0
0x808be12: MOV EAX, dword ptr [EAX + 0xc]
0x808be15: TEST EAX, EAX
0x808be17: JNZ 0x0808be0c
0x808be19: JMP 0x0808bdae
0x808be1c: MOV [0x083cc4e0], EAX
0x808be21: JMP 0x0808bdae
```

## vm_typeValueAt @ 0x808afc8

Evidence: `ev_616461794c4cd7f622a095dfb402e863b4ff031428987af556db44e6bada65ba`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808bbdd", "start": "0x808afc8"}, {"end": "0x808bd87", "start": "0x808bbe0"}], "span_bytes": 3520, "total_bytes": 3518}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

/* WARNING: Unknown calling convention */

char * vm_typeValueAt(char *pStr,uint len,Vm_Value_T *pValue,int ident)

{
  Vm_Object *pVVar1;
  Vm_Fun_Ptr *pVVar2;
  Vm_Mapping *pVVar3;
  Vm_Buffer *pVVar4;
  byte bVar5;
  Vm_Efun *pVVar6;
  char *pcVar7;
  Vm_String_T *pVVar8;
  char cVar9;
  char cVar10;
  uint uVar11;
  int iVar12;
  uint uVar13;
  int iVar14;
  byte bVar15;
  char *pcVar16;
  Vm_Mapping_Node_T *pVVar17;
  int iVar18;
  undefined4 *puVar19;
  char *pcVar20;
  int local_90;
  byte local_89;
  Vm_Fun_Ptr_T *function;
  int j;
  uchar *cp;
  uint n;
  uint i;
  char buffer [80];
  
  if (ident < 1) {
    ident = -ident;
  }
  else {
    if (len < (uint)ident) goto LAB_0808b089;
    if (ident != 0) {
      i = ident;
      do {
        *pStr = ' ';
        len = len - 1;
        pStr = pStr + 1;
        i = i - 1;
      } while (i != 0);
    }
  }
  pcVar16 = pStr;
  if (vm_checkLoop == 0) {
    local_89 = pValue->type;
  }
  else {
    local_89 = pValue->type;
    if ((5 < local_89) && ((local_89 < 8 || (local_89 == 9)))) {
      if ((uint)pValue->active == vm_lastActiveFlag) {
                    /* Unresolved local var: ulong n@[DW_OP_reg6(ESI)]
                       Unresolved local var: int ch@[DW_OP_reg0(EAX)] */
        uVar11 = (pValue->v).number;
        if (len == 0) goto LAB_0808b6ce;
        *pStr = '<';
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
        pcVar7 = pStr + 1;
        bVar5 = pValue->type;
        pcVar16 = "array";
        if (bVar5 != 6) {
          if (bVar5 < 7) {
            pcVar16 = "integer";
            if (bVar5 != 2) {
              if (bVar5 < 3) {
                pcVar16 = "undefined";
                if (bVar5 != 0) {
                  if (bVar5 != 1) goto LAB_0808b451;
                  pcVar16 = "void";
                }
              }
              else {
                pcVar16 = "string";
                if (bVar5 != 4) {
                  if (bVar5 < 5) {
                    pcVar16 = "float";
                  }
                  else {
                    pcVar16 = "buffer";
                  }
                }
              }
            }
          }
          else {
            pcVar16 = "function";
            if (bVar5 != 9) {
              if (bVar5 < 10) {
                pcVar16 = "mapping";
                if (bVar5 != 7) {
                  if (bVar5 != 8) goto LAB_0808b451;
                  pcVar16 = "pointer";
                }
              }
              else {
                pcVar16 = "loop_ctrl";
                if (bVar5 != 0x14) {
                  if (bVar5 < 0x15) {
                    if (bVar5 != 10) {
LAB_0808b451:
                      vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value.\n"
                                 ,"/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x6bf);
                      _vm_endProcess(0);
                      do {
                    /* WARNING: Do nothing block with infinite loop */
                      } while( true );
                    }
                    pcVar16 = "object";
                  }
                  else {
                    if (bVar5 != 0xff) goto LAB_0808b451;
                    pcVar16 = "mixed";
                  }
                }
              }
            }
          }
        }
        uVar13 = 0xffffffff;
        do {
          if (uVar13 == 0) break;
          uVar13 = uVar13 - 1;
          cVar10 = *pcVar16;
          pcVar16 = pcVar16 + 1;
        } while (cVar10 != '\0');
        uVar13 = ~uVar13 - 1;
        if (uVar13 <= len - 1) {
          pcVar16 = "array";
          if (bVar5 != 6) {
            if (bVar5 < 7) {
              pcVar16 = "integer";
              if (bVar5 != 2) {
                if (bVar5 < 3) {
                  pcVar16 = "undefined";
                  if (bVar5 != 0) {
                    if (bVar5 != 1) goto LAB_0808b4d2;
                    pcVar16 = "void";
                  }
                }
                else {
                  pcVar16 = "string";
                  if (bVar5 != 4) {
                    if (bVar5 < 5) {
                      pcVar16 = "float";
                    }
                    else {
                      pcVar16 = "buffer";
                    }
                  }
                }
              }
            }
            else {
              pcVar16 = "function";
              if (bVar5 != 9) {
                if (bVar5 < 10) {
                  pcVar16 = "mapping";
                  if (bVar5 != 7) {
                    if (bVar5 != 8) goto LAB_0808b4d2;
                    pcVar16 = "pointer";
                  }
                }
                else {
                  pcVar16 = "loop_ctrl";
                  if (bVar5 != 0x14) {
                    if (bVar5 < 0x15) {
                      if (bVar5 != 10) {
LAB_0808b4d2:
                        vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                                   "Bad type of value.\n","/home/cc/xxxx/gameshell/src/vm/vm_var.c",
                                   0x6bf);
                        _vm_endProcess(0);
                        do {
                    /* WARNING: Do nothing block with infinite loop */
                        } while( true );
                      }
                      pcVar16 = "object";
                    }
                    else {
                      if (bVar5 != 0xff) goto LAB_0808b4d2;
                      pcVar16 = "mixed";
                    }
                  }
                }
              }
            }
          }
          strcpy(pcVar7,pcVar16);
          pcVar16 = pcVar7 + uVar13;
          iVar12 = (len - 1) - uVar13;
          if (iVar12 == 0) goto LAB_0808b6ce;
          *pcVar16 = ':';
          len = iVar12 - 1;
          i = 0;
          bVar5 = 0x1c;
          pcVar7 = pcVar16 + 1;
          do {
            pStr = pcVar7;
            uVar13 = uVar11 >> (bVar5 & 0x1f) & 0xf;
            cVar10 = (char)uVar13;
            cVar9 = cVar10 + '0';
            if (9 < uVar13) {
              cVar9 = cVar10 + '7';
            }
            pcVar16 = pStr;
            if (len == 0) goto LAB_0808b6ce;
            *pStr = cVar9;
            i = i + 1;
            pcVar7 = pStr + 1;
            len = len - 1;
            bVar5 = bVar5 - 4;
          } while (i < 8);
          if (len != 0) {
            *pcVar7 = '>';
            pStr = pStr + 2;
            goto LAB_0808b089;
          }
          goto LAB_0808b3f0;
        }
        goto LAB_0808b103;
      }
      pValue->active = (ushort)vm_lastActiveFlag;
    }
  }
  pcVar7 = pStr;
  switch(local_89) {
  case 0:
                    /* Unresolved local var: uint _len@[???] */
    if (len < 0xb) goto LAB_0808b3f0;
    builtin_strncpy(pStr,"(undefined)",0xc);
    pcVar7 = pStr + 0xb;
    break;
  case 1:
                    /* Unresolved local var: uint _len@[???] */
    if (len < 6) goto LAB_0808b6ce;
    puVar19 = &DAT_082acd8a;
    for (iVar12 = 1; iVar12 != 0; iVar12 = iVar12 + -1) {
      *(undefined4 *)pcVar16 = *puVar19;
      puVar19 = puVar19 + 1;
      pcVar16 = pcVar16 + 4;
    }
    *(undefined2 *)pcVar16 = *(undefined2 *)puVar19;
    pcVar16[2] = *(char *)((int)puVar19 + 2);
    pcVar7 = pStr + 6;
    break;
  case 2:
    vm_intToString(buffer,0x50,(pValue->v).number,10,1,0);
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
    uVar11 = 0xffffffff;
    pcVar16 = buffer;
    do {
      if (uVar11 == 0) break;
      uVar11 = uVar11 - 1;
      cVar10 = *pcVar16;
      pcVar16 = pcVar16 + 1;
    } while (cVar10 != '\0');
    uVar11 = ~uVar11 - 1;
    if (len < uVar11) goto LAB_0808b089;
    goto LAB_0808bc3c;
  case 3:
    sprintf(buffer,"%g",(double)(pValue->v).real);
    uVar11 = 0xffffffff;
    pcVar16 = buffer;
    do {
      if (uVar11 == 0) break;
      uVar11 = uVar11 - 1;
      cVar10 = *pcVar16;
      pcVar16 = pcVar16 + 1;
    } while (cVar10 != '\0');
    uVar11 = ~uVar11 - 1;
    if (len < uVar11) break;
    goto LAB_0808bc3c;
  case 4:
    if (len != 0) {
      *pStr = '\"';
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      pcVar7 = pStr + 1;
      pcVar16 = ((pValue->v).string)->ptr;
      uVar11 = 0xffffffff;
      pcVar20 = pcVar16;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar20;
        pcVar20 = pcVar20 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (uVar11 <= len - 1) {
        strcpy(pcVar7,pcVar16);
        pcVar16 = pcVar7 + uVar11;
        if (len - 1 == uVar11) goto LAB_0808b6ce;
        *pcVar16 = '\"';
        pcVar7 = pcVar16 + 1;
      }
    }
    break;
  case 5:
                    /* Unresolved local var: int k@[DW_OP_reg6(ESI)]
                       Unresolved local var: int j@[DW_OP_reg1(ECX)]
                       Unresolved local var: uchar c@[DW_OP_reg2(EDX)]
                       Unresolved local var: uchar c1@[DW_OP_reg0(EAX)]
                       Unresolved local var: uchar c2@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_Buffer_T * pBuffer@[DW_OP_reg0(EAX)] */
    pVVar4 = (pValue->v).buffer;
    uVar11 = pVVar4->realSize;
    if (uVar11 == 0) {
                    /* Unresolved local var: uint _len@[???] */
      if (len < 0xe) goto LAB_0808b6ce;
      pcVar16 = "<Empty buffer>";
      pcVar7 = pStr;
      for (iVar12 = 3; iVar12 != 0; iVar12 = iVar12 + -1) {
        *(undefined4 *)pcVar7 = *(undefined4 *)pcVar16;
        pcVar16 = pcVar16 + 4;
        pcVar7 = pcVar7 + 4;
      }
      *(undefined2 *)pcVar7 = *(undefined2 *)pcVar16;
      pcVar7[2] = pcVar16[2];
      pcVar7 = pStr + 0xe;
      break;
    }
    if (uVar11 < 0x201) {
      iVar12 = 0;
      cp = pVVar4->ptr;
      i = 0;
      if (uVar11 == 0) break;
      do {
        bVar5 = *cp >> 4;
        bVar15 = *cp & 0xf;
        cVar10 = bVar5 + 0x30;
        if (9 < bVar5) {
          cVar10 = bVar5 + 0x37;
        }
        cVar9 = bVar15 + 0x30;
        if (9 < bVar15) {
          cVar9 = bVar15 + 0x37;
        }
        buffer[iVar12] = cVar10;
        buffer[iVar12 + 1] = cVar9;
        if ((i & 0xf) == 0xf) {
          builtin_strncpy(buffer + iVar12 + 2,"    ",4);
          iVar14 = 0;
          iVar12 = iVar12 + 6;
          do {
            iVar18 = iVar12;
            bVar5 = cp[iVar14 + -0xf];
            if (((bVar5 < 0x20) || (bVar5 == 0x7f)) || (bVar5 == 0xff)) {
              bVar5 = 0x2e;
            }
            iVar14 = iVar14 + 1;
            buffer[iVar18] = bVar5;
            iVar12 = iVar18 + 1;
          } while (iVar14 < 0x10);
          buffer[iVar18 + 1] = ' ';
          buffer[iVar18 + 2] = '\n';
          buffer[iVar18 + 3] = '\0';
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
          uVar13 = 0xffffffff;
          pcVar16 = buffer;
          do {
            if (uVar13 == 0) break;
            uVar13 = uVar13 - 1;
            cVar10 = *pcVar16;
            pcVar16 = pcVar16 + 1;
          } while (cVar10 != '\0');
          uVar13 = ~uVar13 - 1;
          pcVar7 = pStr;
          if (len < uVar13) goto LAB_0808b3f0;
          strcpy(pStr,buffer);
          pStr = pStr + uVar13;
          len = len - uVar13;
          if (i < uVar11 - 1) {
            if (len < (uint)ident) goto LAB_0808b089;
            iVar12 = ident;
            if (0 < ident) {
              do {
                *pStr = ' ';
                len = len - 1;
                pStr = pStr + 1;
                iVar12 = iVar12 + -1;
              } while (iVar12 != 0);
            }
          }
          iVar12 = 0;
        }
        else {
          iVar14 = iVar12 + 2;
          iVar12 = iVar12 + 3;
          if (((i & 7) != 7) || (cVar10 = '-', uVar11 - 1 == i)) {
            cVar10 = ' ';
          }
          buffer[iVar14] = cVar10;
        }
        i = i + 1;
        cp = cp + 1;
      } while (i < uVar11);
      pcVar7 = pStr;
      if (iVar12 == 0) break;
      for (; (i & 0xf) != 0; i = i + 1) {
        buffer[iVar12] = ' ';
        buffer[iVar12 + 1] = ' ';
        buffer[iVar12 + 2] = ' ';
        iVar12 = iVar12 + 3;
        cp = cp + 1;
      }
      buffer[iVar12] = ' ';
      buffer[iVar12 + 1] = ' ';
      buffer[iVar12 + 2] = ' ';
      iVar14 = 0;
      uVar13 = i - 0x10;
      iVar12 = iVar12 + 3;
      do {
        iVar18 = iVar12;
        bVar5 = 0x2e;
        if ((uVar13 < uVar11) &&
           (((bVar5 = cp[iVar14 + -0x10], bVar5 < 0x20 || (bVar5 == 0x7f)) || (bVar5 == 0xff)))) {
          bVar5 = 0x2e;
        }
        iVar14 = iVar14 + 1;
        buffer[iVar18] = bVar5;
        uVar13 = uVar13 + 1;
        iVar12 = iVar18 + 1;
      } while (iVar14 < 0x10);
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      buffer[iVar18 + 1] = '\n';
      buffer[iVar18 + 2] = '\0';
      uVar11 = 0xffffffff;
      pcVar16 = buffer;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar16;
        pcVar16 = pcVar16 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (len < uVar11) goto LAB_0808b3f0;
    }
    else {
      sprintf(buffer,"<Buffer:%d>",uVar11,pVVar4);
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      uVar11 = 0xffffffff;
      pcVar16 = buffer;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar16;
        pcVar16 = pcVar16 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (len < uVar11) goto LAB_0808b3f0;
    }
LAB_0808bc3c:
    strcpy(pStr,buffer);
    pcVar7 = pStr + uVar11;
    break;
  case 6:
    if (((pValue->v).string)->attrib != 0) {
                    /* Unresolved local var: uint _len@[???] */
      if (len < 0x12) goto LAB_0808b3f0;
      pcVar16 = "({ /* sizeof() == ";
      pcVar7 = pStr;
      for (iVar12 = 4; iVar12 != 0; iVar12 = iVar12 + -1) {
        *(undefined4 *)pcVar7 = *(undefined4 *)pcVar16;
        pcVar16 = pcVar16 + 4;
        pcVar7 = pcVar7 + 4;
      }
      *(undefined2 *)pcVar7 = *(undefined2 *)pcVar16;
      pcVar7[2] = pcVar16[2];
      pcVar7 = pStr + 0x12;
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      vm_intToString(buffer,0x50,((pValue->v).object)->execRef,10,1,0);
      uVar11 = 0xffffffff;
      pcVar16 = buffer;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar16;
        pcVar16 = pcVar16 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (len - 0x12 < uVar11) break;
      strcpy(pcVar7,buffer);
      uVar13 = (len - 0x12) - uVar11;
                    /* Unresolved local var: uint _len@[???] */
      pcVar16 = pcVar7 + uVar11;
      if (uVar13 < 4) {
LAB_0808b6ce:
                    /* Unresolved local var: uint _len@[???] */
        pStr = pcVar16;
        *pStr = '\0';
        return pStr;
      }
      builtin_strncpy(pStr + uVar11 + 0x12," */\n",5);
      pStr = pcVar7 + uVar11 + 4;
      i = 0;
      pVVar8 = (pValue->v).string;
      len = uVar13 - 4;
      if (pVVar8->attrib != 0) {
        do {
                    /* Unresolved local var: char * pTo@[DW_OP_reg7(EDI)] */
          pcVar7 = vm_typeValueAt(pStr,len,*(Vm_Value_T **)(pVVar8->ptr + i * 4),ident + 4);
          uVar11 = len - ((int)pcVar7 - (int)pStr);
                    /* Unresolved local var: uint _len@[???] */
          if (uVar11 < 2) goto LAB_0808b103;
          pcVar7[0] = ',';
          pcVar7[1] = '\n';
          pcVar7[2] = '\0';
          i = i + 1;
          pStr = pcVar7 + 2;
          pVVar8 = (pValue->v).string;
          len = uVar11 - 2;
        } while (i < pVVar8->attrib);
      }
      pcVar7 = pStr;
      if (len < (uint)ident) goto LAB_0808b3f0;
      if (ident != 0) {
        i = ident;
        do {
          *pStr = ' ';
          len = len - 1;
          pStr = pStr + 1;
          i = i - 1;
        } while (i != 0);
      }
                    /* Unresolved local var: uint _len@[???] */
      pcVar16 = pStr;
      if (len < 2) goto LAB_0808b6ce;
      pStr[0] = '}';
      pStr[1] = ')';
LAB_0808bb2f:
      pStr[2] = '\0';
      pcVar7 = pStr + 2;
      break;
    }
                    /* Unresolved local var: uint _len@[???] */
    if (len < 5) break;
    builtin_strncpy(pStr,"({ }",4);
    goto LAB_0808b745;
  case 7:
    pVVar3 = (pValue->v).mapping;
    if (pVVar3->count != 0) {
      j = pVVar3->tableSize;
                    /* Unresolved local var: uint _len@[???] */
      if (len < 0x12) goto LAB_0808b089;
      pcVar16 = "([ /* sizeof() == ";
      pcVar7 = pStr;
      for (iVar12 = 4; iVar12 != 0; iVar12 = iVar12 + -1) {
        *(undefined4 *)pcVar7 = *(undefined4 *)pcVar16;
        pcVar16 = pcVar16 + 4;
        pcVar7 = pcVar7 + 4;
      }
      *(undefined2 *)pcVar7 = *(undefined2 *)pcVar16;
      pcVar7[2] = pcVar16[2];
      pcVar16 = pStr + 0x12;
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      vm_intToString(buffer,0x50,pVVar3->count,10,1,0);
      uVar11 = 0xffffffff;
      pcVar7 = buffer;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (len - 0x12 < uVar11) goto LAB_0808b6ce;
      strcpy(pcVar16,buffer);
      uVar13 = (len - 0x12) - uVar11;
                    /* Unresolved local var: uint _len@[???] */
      pcVar7 = pcVar16 + uVar11;
      if (uVar13 < 4) {
LAB_0808b3f0:
        pStr = pcVar7;
        *pStr = '\0';
        return pStr;
      }
      builtin_strncpy(pStr + uVar11 + 0x12," */\n",5);
      pStr = pcVar16 + uVar11 + 4;
      len = uVar13 - 4;
      do {
        for (pVVar17 = pVVar3->table[j]; pVVar17 != (Vm_Mapping_Node_T *)0x0;
            pVVar17 = pVVar17->pNext) {
          pcVar16 = vm_typeValueAt(pStr,len,pVVar17->values,ident + 4);
          uVar11 = len - ((int)pcVar16 - (int)pStr);
          if (uVar11 < 3) goto LAB_0808b6ce;
          builtin_strncpy(pcVar16," : ",4);
          uVar11 = uVar11 - 3;
          pcVar7 = vm_typeValueAt(pcVar16 + 3,uVar11,pVVar17->values + 1,-(ident + 4));
          uVar11 = uVar11 - ((int)pcVar7 - (int)(pcVar16 + 3));
          if (uVar11 < 2) goto LAB_0808b3f0;
          pcVar7[0] = ',';
          pcVar7[1] = '\n';
          pcVar7[2] = '\0';
          len = uVar11 - 2;
          pStr = pcVar7 + 2;
        }
        j = j + -1;
      } while (j != -1);
      pcVar7 = pStr;
      if (len < (uint)ident) goto LAB_0808b3f0;
      if (ident != 0) {
        i = ident;
        do {
          *pStr = ' ';
          len = len - 1;
          pStr = pStr + 1;
          i = i - 1;
        } while (i != 0);
      }
                    /* Unresolved local var: uint _len@[???] */
      pcVar16 = pStr;
      if (len < 2) goto LAB_0808b6ce;
      pStr[0] = ']';
      pStr[1] = ')';
      goto LAB_0808bb2f;
    }
                    /* Unresolved local var: uint _len@[???] */
    if (len < 5) break;
    builtin_strncpy(pStr,"([ ]",4);
LAB_0808b745:
    pStr[4] = ')';
    pStr[5] = '\0';
    pcVar7 = pStr + 5;
    break;
  case 8:
                    /* Unresolved local var: uint _len@[???] */
    if (len < 0xb) break;
    builtin_strncpy(pStr,"(Reference)",0xc);
    goto LAB_0808b5bb;
  case 9:
    pVVar2 = (pValue->v).function;
    pVVar1 = pVVar2->pObject;
    if ((pVVar1 == (Vm_Object *)0x0) || ((pVVar1->flags & 0x8000) == 0)) {
                    /* Unresolved local var: uint _len@[???] */
      if (len < 3) goto LAB_0808b3f0;
      builtin_strncpy(pStr,"(: ",4);
      pStr = pStr + 3;
      if (pVVar2->type == 'K') {
        pVVar6 = (pVVar2->f).pEfun;
      }
      else {
        pVVar6 = (pVVar2->f).pEfun;
      }
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      uVar11 = 0xffffffff;
      pcVar16 = pVVar6->word;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar16;
        pcVar16 = pcVar16 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (len - 3 < uVar11) goto LAB_0808b089;
      strcpy(pStr,pVVar6->word);
      pStr = pStr + uVar11;
      len = (len - 3) - uVar11;
      if (pVVar1 != (Vm_Object *)0x0) {
                    /* Unresolved local var: uint _len@[???] */
        pcVar7 = pStr;
        if (len == 0) break;
        pStr[0] = '@';
        pStr[1] = '\0';
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
        pStr = pStr + 1;
        pcVar16 = pVVar1->pProgram->strName->ptr;
        uVar11 = 0xffffffff;
        pcVar7 = pcVar16;
        do {
          if (uVar11 == 0) break;
          uVar11 = uVar11 - 1;
          cVar10 = *pcVar7;
          pcVar7 = pcVar7 + 1;
        } while (cVar10 != '\0');
        uVar11 = ~uVar11 - 1;
        if (len - 1 < uVar11) goto LAB_0808b089;
        strcpy(pStr,pcVar16);
        pStr = pStr + uVar11;
        len = (len - 1) - uVar11;
        if ((pVVar1->flags & 1) != 0) {
                    /* Unresolved local var: uint _len@[???] */
          pcVar7 = pStr;
          if (len == 0) break;
          pStr[0] = '#';
          pStr[1] = '\0';
          pStr = pStr + 1;
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
          vm_intToString(buffer,0x50,pVVar1->oid,10,1,0);
          uVar11 = 0xffffffff;
          pcVar16 = buffer;
          do {
            if (uVar11 == 0) break;
            uVar11 = uVar11 - 1;
            cVar10 = *pcVar16;
            pcVar16 = pcVar16 + 1;
          } while (cVar10 != '\0');
          uVar11 = ~uVar11 - 1;
          if (len - 1 < uVar11) goto LAB_0808b089;
          strcpy(pStr,buffer);
          pStr = pStr + uVar11;
          len = (len - 1) - uVar11;
        }
      }
      iVar12 = 0;
      if (0 < pVVar2->argn) {
        local_90 = 0;
        do {
          if (len < 2) goto LAB_0808b089;
          pStr[0] = ',';
          pStr[1] = ' ';
          pStr[2] = '\0';
          pcVar16 = pStr + 2;
          pStr = vm_typeValueAt(pcVar16,len - 2,(Vm_Value_T *)((int)&pVVar2->pArgs->ref + local_90),
                                -(ident + 4));
          len = (len - 2) - ((int)pStr - (int)pcVar16);
          iVar12 = iVar12 + 1;
          local_90 = local_90 + 0x18;
        } while (iVar12 < pVVar2->argn);
      }
                    /* Unresolved local var: uint _len@[???] */
      pcVar16 = pStr;
      if (len < 3) goto LAB_0808b6ce;
      builtin_strncpy(pStr," :)",4);
      pcVar7 = pStr + 3;
      break;
    }
    goto LAB_0808b592;
  case 10:
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg6(ESI)] */
    pVVar1 = (pValue->v).object;
    if ((pVVar1->flags & 0x8000) == 0) {
                    /* Unresolved local var: uint _len@[DW_OP_reg3(EBX)] */
      pcVar16 = pVVar1->pProgram->strName->ptr;
      uVar11 = 0xffffffff;
      pcVar7 = pcVar16;
      do {
        if (uVar11 == 0) break;
        uVar11 = uVar11 - 1;
        cVar10 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar10 != '\0');
      uVar11 = ~uVar11 - 1;
      if (len < uVar11) {
LAB_0808b089:
        *pStr = '\0';
        return pStr;
      }
      strcpy(pStr,pcVar16);
      pcVar7 = pStr + uVar11;
                    /* Unresolved local var: uint _len@[???] */
      if ((pVVar1->oid != 0) && (len - uVar11 != 0)) {
        pcVar7[0] = '#';
        pcVar7[1] = '\0';
        pStr = pcVar7 + 1;
                    /* Unresolved local var: uint _len@[DW_OP_reg6(ESI)] */
        vm_intToString(buffer,0x50,pVVar1->oid,10,1,0);
        uVar13 = 0xffffffff;
        pcVar16 = buffer;
        do {
          if (uVar13 == 0) break;
          uVar13 = uVar13 - 1;
          cVar10 = *pcVar16;
          pcVar16 = pcVar16 + 1;
        } while (cVar10 != '\0');
        if ((len - uVar11) - 1 < ~uVar13 - 1) goto LAB_0808b089;
        strcpy(pStr,buffer);
        pcVar7 = pStr + (~uVar13 - 1);
      }
      break;
    }
LAB_0808b592:
    vm_cleanupValue(pValue);
                    /* Unresolved local var: uint _len@[???] */
    if (10 < len) {
      builtin_strncpy(pStr,"(undefined)",0xc);
LAB_0808b5bb:
      pcVar7 = pStr + 0xb;
    }
    break;
  default:
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x405);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  case 0x14:
                    /* Unresolved local var: uint _len@[???] */
    if (9 < len) {
      pcVar16 = "<LoopCtrl>";
      pcVar7 = pStr;
      for (iVar12 = 2; iVar12 != 0; iVar12 = iVar12 + -1) {
        *(undefined4 *)pcVar7 = *(undefined4 *)pcVar16;
        pcVar16 = pcVar16 + 4;
        pcVar7 = pcVar7 + 4;
      }
      *(undefined2 *)pcVar7 = *(undefined2 *)pcVar16;
      pcVar7[2] = pcVar16[2];
      pcVar7 = pStr + 10;
    }
  }
LAB_0808b103:
  pStr = pcVar7;
  *pStr = '\0';
  return pStr;
}


```

```text
0x808afc8: PUSH EBP
0x808afc9: MOV EBP, ESP
0x808afcb: PUSH EDI
0x808afcc: PUSH ESI
0x808afcd: PUSH EBX
0x808afce: SUB ESP, 0x8c
0x808afd4: MOV EAX, dword ptr [EBP + 0x14]
0x808afd7: TEST EAX, EAX
0x808afd9: JLE 0x0808b0d4
0x808afdf: MOV EAX, dword ptr [EBP + 0xc]
0x808afe2: CMP dword ptr [EBP + 0x14], EAX
0x808afe5: JA 0x0808b086
0x808afeb: CMP dword ptr [EBP + 0x14], 0x0
0x808afef: JBE 0x0808b009
0x808aff1: MOV ECX, dword ptr [EBP + 0x14]
0x808aff4: MOV dword ptr [EBP + -0x6c], ECX
0x808aff7: MOV EBX, dword ptr [EBP + 0x8]
0x808affa: MOV byte ptr [EBX], 0x20
0x808affd: DEC dword ptr [EBP + 0xc]
0x808b000: INC EBX
0x808b001: DEC dword ptr [EBP + -0x6c]
0x808b004: MOV dword ptr [EBP + 0x8], EBX
0x808b007: JNZ 0x0808aff7
0x808b009: MOV EAX, [0x0847fd84]
0x808b00e: TEST EAX, EAX
0x808b010: JZ 0x0808b097
0x808b016: MOV EAX, dword ptr [EBP + 0x10]
0x808b019: MOV AL, byte ptr [EAX + 0x4]
0x808b01c: MOV byte ptr [EBP + 0xffffff7b], AL
0x808b022: MOVZX EAX, AL
0x808b025: CMP EAX, 0x6
0x808b028: JL 0x0808b03c
0x808b02a: CMP EAX, 0x7
0x808b02d: JLE 0x0808b10b
0x808b033: CMP EAX, 0x9
0x808b036: JZ 0x0808b10b
0x808b03c: MOVZX EAX, byte ptr [EBP + 0xffffff7b]
0x808b043: CMP EAX, 0x14
0x808b046: JA 0x0808b0a5
0x808b048: JMP dword ptr [EAX*0x4 + 0x82ace70]
0x808b04f: SUB ESP, 0x8
0x808b052: PUSH 0x0
0x808b054: PUSH 0x1
0x808b056: PUSH 0xa
0x808b058: MOV EAX, dword ptr [EBP + 0x10]
0x808b05b: PUSH dword ptr [EAX + 0x8]
0x808b05e: PUSH 0x50
0x808b060: LEA ESI, [EBP + -0x68]
0x808b063: PUSH ESI
0x808b064: CALL 0x080b6304
0x808b069: MOV EDI, ESI
0x808b06b: XOR EAX, EAX
0x808b06d: CLD
0x808b06e: MOV ECX, 0xffffffff
0x808b073: SCASB.REPNE ES:EDI
0x808b075: NOT ECX
0x808b077: LEA EBX, [ECX + -0x1]
0x808b07a: ADD ESP, 0x20
0x808b07d: CMP EBX, dword ptr [EBP + 0xc]
0x808b080: JBE 0x0808bc38
0x808b086: MOV EDX, dword ptr [EBP + 0x8]
0x808b089: MOV byte ptr [EDX], 0x0
0x808b08c: MOV EAX, dword ptr [EBP + 0x8]
0x808b08f: LEA ESP, [EBP + -0xc]
0x808b092: POP EBX
0x808b093: POP ESI
0x808b094: POP EDI
0x808b095: LEAVE
0x808b096: RET
0x808b097: MOV EBX, dword ptr [EBP + 0x10]
0x808b09a: MOV BL, byte ptr [EBX + 0x4]
0x808b09d: MOV byte ptr [EBP + 0xffffff7b], BL
0x808b0a3: JMP 0x0808b03c
0x808b0a5: SUB ESP, 0x8
0x808b0a8: PUSH 0x405
0x808b0ad: PUSH 0x82acaf8
0x808b0b2: PUSH 0x82accd1
0x808b0b7: PUSH 0x82a825c
0x808b0bc: PUSH 0x4
0x808b0be: PUSH 0x2
0x808b0c0: CALL 0x080b61c4
0x808b0c5: ADD ESP, 0x14
0x808b0c8: PUSH 0x0
0x808b0ca: CALL 0x080a3980
0x808b0cf: ADD ESP, 0x10
0x808b0d2: JMP 0x0808b0d2
0x808b0d4: NEG dword ptr [EBP + 0x14]
0x808b0d7: JMP 0x0808b009
0x808b0dc: CMP dword ptr [EBP + 0xc], 0xb
0x808b0e0: JC 0x0808b3ed
0x808b0e6: MOV EDX, dword ptr [EBP + 0x8]
0x808b0e9: MOV dword ptr [EDX], 0x646e7528
0x808b0ef: MOV dword ptr [EDX + 0x4], 0x6e696665
0x808b0f6: MOV dword ptr [EDX + 0x8], 0x296465
0x808b0fd: ADD EDX, 0xb
0x808b100: MOV dword ptr [EBP + 0x8], EDX
0x808b103: MOV ECX, dword ptr [EBP + 0x8]
0x808b106: MOV byte ptr [ECX], 0x0
0x808b109: JMP 0x0808b08c
0x808b10b: MOV EDX, dword ptr [EBP + 0x10]
0x808b10e: MOVZX EAX, word ptr [EDX + 0x6]
0x808b112: MOV EDX, dword ptr [0x083cc4e0]
0x808b118: CMP EAX, EDX
0x808b11a: JZ 0x0808b3f8
0x808b120: MOV ECX, dword ptr [EBP + 0x10]
0x808b123: MOV word ptr [ECX + 0x6], DX
0x808b127: JMP 0x0808b03c
0x808b12c: MOV EDX, dword ptr [EBP + 0x10]
0x808b12f: MOV ESI, dword ptr [EDX + 0x8]
0x808b132: TEST byte ptr [ESI + 0xd], 0x80
0x808b136: JNZ 0x0808b65c
0x808b13c: MOV EAX, dword ptr [ESI + 0x20]
0x808b13f: MOV EAX, dword ptr [EAX + 0x4]
0x808b142: MOV EDX, dword ptr [EAX + 0xc]
0x808b145: CLD
0x808b146: XOR EAX, EAX
0x808b148: MOV ECX, 0xffffffff
0x808b14d: MOV EDI, EDX
0x808b14f: SCASB.REPNE ES:EDI
0x808b151: NOT ECX
0x808b153: LEA EBX, [ECX + -0x1]
0x808b156: CMP EBX, dword ptr [EBP + 0xc]
0x808b159: JA 0x0808b086
0x808b15f: SUB ESP, 0x8
0x808b162: PUSH EDX
0x808b163: PUSH dword ptr [EBP + 0x8]
0x808b166: CALL 0x0804bc54
0x808b16b: ADD dword ptr [EBP + 0x8], EBX
0x808b16e: SUB dword ptr [EBP + 0xc], EBX
0x808b171: MOV EBX, dword ptr [ESI + 0x8]
0x808b174: ADD ESP, 0x10
0x808b177: TEST EBX, EBX
0x808b179: JZ 0x0808b103
0x808b17b: CMP dword ptr [EBP + 0xc], 0x1
0x808b17f: JC 0x0808b103
0x808b181: SUB ESP, 0x8
0x808b184: MOV EBX, dword ptr [EBP + 0x8]
0x808b187: MOV word ptr [EBX], 0x23
0x808b18c: PUSH 0x0
0x808b18e: PUSH 0x1
0x808b190: PUSH 0xa
0x808b192: PUSH dword ptr [ESI + 0x8]
0x808b195: INC EBX
0x808b196: PUSH 0x50
0x808b198: MOV dword ptr [EBP + 0x8], EBX
0x808b19b: LEA EBX, [EBP + -0x68]
0x808b19e: PUSH EBX
0x808b19f: DEC dword ptr [EBP + 0xc]
0x808b1a2: MOV EDI, EBX
0x808b1a4: CALL 0x080b6304
0x808b1a9: XOR EAX, EAX
0x808b1ab: CLD
0x808b1ac: MOV ECX, 0xffffffff
0x808b1b1: SCASB.REPNE ES:EDI
0x808b1b3: NOT ECX
0x808b1b5: LEA ESI, [ECX + -0x1]
0x808b1b8: ADD ESP, 0x20
0x808b1bb: CMP ESI, dword ptr [EBP + 0xc]
0x808b1be: JA 0x0808b086
0x808b1c4: SUB ESP, 0x8
0x808b1c7: PUSH EBX
0x808b1c8: PUSH dword ptr [EBP + 0x8]
0x808b1cb: CALL 0x0804bc54
0x808b1d0: ADD dword ptr [EBP + 0x8], ESI
0x808b1d3: ADD ESP, 0x10
0x808b1d6: JMP 0x0808b103
0x808b1db: CMP dword ptr [EBP + 0xc], 0xa
0x808b1df: JC 0x0808b103
0x808b1e5: MOV EDI, dword ptr [EBP + 0x8]
0x808b1e8: CLD
0x808b1e9: MOV ESI, 0x82acd3b
0x808b1ee: MOV ECX, 0x2
0x808b1f3: MOVSD.REP ES:EDI, ESI
0x808b1f5: MOVSW ES:EDI, ESI
0x808b1f7: MOVSB ES:EDI, ESI
0x808b1f8: ADD dword ptr [EBP + 0x8], 0xa
0x808b1fc: JMP 0x0808b103
0x808b201: MOV EDX, dword ptr [EBP + 0x10]
0x808b204: MOV EDX, dword ptr [EDX + 0x8]
0x808b207: MOV ESI, dword ptr [EDX + 0xc]
0x808b20a: TEST ESI, ESI
0x808b20c: MOV dword ptr [EBP + 0xffffff7c], EDX
0x808b212: JZ 0x0808b21e
0x808b214: TEST byte ptr [ESI + 0xd], 0x80
0x808b218: JNZ 0x0808b58c
0x808b21e: CMP dword ptr [EBP + 0xc], 0x3
0x808b222: JC 0x0808b3ed
0x808b228: MOV EDX, dword ptr [EBP + 0x8]
0x808b22b: MOV dword ptr [EDX], 0x203a28
0x808b231: MOV ECX, dword ptr [EBP + 0xffffff7c]
0x808b237: ADD EDX, 0x3
0x808b23a: SUB dword ptr [EBP + 0xc], 0x3
0x808b23e: CMP byte ptr [ECX + 0x4], 0x4b
0x808b242: MOV dword ptr [EBP + 0x8], EDX
0x808b245: JZ 0x0808b3e5
0x808b24b: MOV EBX, dword ptr [EBP + 0xffffff7c]
0x808b251: MOV EAX, dword ptr [EBX + 0x14]
0x808b254: MOV EDX, dword ptr [EAX]
0x808b256: CLD
0x808b257: XOR EAX, EAX
0x808b259: MOV ECX, 0xffffffff
0x808b25e: MOV EDI, EDX
0x808b260: SCASB.REPNE ES:EDI
0x808b262: NOT ECX
0x808b264: LEA EBX, [ECX + -0x1]
0x808b267: CMP EBX, dword ptr [EBP + 0xc]
0x808b26a: JA 0x0808b086
0x808b270: SUB ESP, 0x8
0x808b273: PUSH EDX
0x808b274: PUSH dword ptr [EBP + 0x8]
0x808b277: CALL 0x0804bc54
0x808b27c: ADD dword ptr [EBP + 0x8], EBX
0x808b27f: SUB dword ptr [EBP + 0xc], EBX
0x808b282: ADD ESP, 0x10
0x808b285: TEST ESI, ESI
0x808b287: JZ 0x0808b346
0x808b28d: CMP dword ptr [EBP + 0xc], 0x1
0x808b291: JC 0x0808b103
0x808b297: MOV EBX, dword ptr [EBP + 0x8]
0x808b29a: MOV word ptr [EBX], 0x40
0x808b29f: MOV EAX, dword ptr [ESI + 0x20]
0x808b2a2: MOV EAX, dword ptr [EAX + 0x4]
0x808b2a5: INC EBX
0x808b2a6: MOV EDX, dword ptr [EAX + 0xc]
0x808b2a9: MOV dword ptr [EBP + 0x8], EBX
0x808b2ac: DEC dword ptr [EBP + 0xc]
0x808b2af: XOR EAX, EAX
0x808b2b1: CLD
0x808b2b2: MOV ECX, 0xffffffff
0x808b2b7: MOV EDI, EDX
0x808b2b9: SCASB.REPNE ES:EDI
0x808b2bb: NOT ECX
0x808b2bd: LEA EBX, [ECX + -0x1]
0x808b2c0: CMP EBX, dword ptr [EBP + 0xc]
0x808b2c3: JA 0x0808b086
0x808b2c9: SUB ESP, 0x8
0x808b2cc: PUSH EDX
0x808b2cd: PUSH dword ptr [EBP + 0x8]
0x808b2d0: CALL 0x0804bc54
0x808b2d5: ADD dword ptr [EBP + 0x8], EBX
0x808b2d8: SUB dword ptr [EBP + 0xc], EBX
0x808b2db: ADD ESP, 0x10
0x808b2de: TEST byte ptr [ESI + 0xc], 0x1
0x808b2e2: JZ 0x0808b346
0x808b2e4: CMP dword ptr [EBP + 0xc], 0x1
0x808b2e8: JC 0x0808b103
0x808b2ee: SUB ESP, 0x8
0x808b2f1: MOV EBX, dword ptr [EBP + 0x8]
0x808b2f4: MOV word ptr [EBX], 0x23
0x808b2f9: PUSH 0x0
0x808b2fb: PUSH 0x1
0x808b2fd: PUSH 0xa
0x808b2ff: PUSH dword ptr [ESI + 0x8]
0x808b302: PUSH 0x50
0x808b304: LEA ESI, [EBP + -0x68]
0x808b307: INC EBX
0x808b308: PUSH ESI
0x808b309: MOV dword ptr [EBP + 0x8], EBX
0x808b30c: DEC dword ptr [EBP + 0xc]
0x808b30f: MOV EDI, ESI
0x808b311: CALL 0x080b6304
0x808b316: XOR EAX, EAX
0x808b318: CLD
0x808b319: MOV ECX, 0xffffffff
0x808b31e: SCASB.REPNE ES:EDI
0x808b320: NOT ECX
0x808b322: LEA EBX, [ECX + -0x1]
0x808b325: ADD ESP, 0x20
0x808b328: CMP EBX, dword ptr [EBP + 0xc]
0x808b32b: JA 0x0808b086
0x808b331: SUB ESP, 0x8
0x808b334: PUSH ESI
0x808b335: PUSH dword ptr [EBP + 0x8]
0x808b338: CALL 0x0804bc54
0x808b33d: ADD dword ptr [EBP + 0x8], EBX
0x808b340: SUB dword ptr [EBP + 0xc], EBX
0x808b343: ADD ESP, 0x10
0x808b346: MOV ECX, dword ptr [EBP + 0xffffff7c]
0x808b34c: XOR ESI, ESI
0x808b34e: CMP word ptr [ECX + 0x6], 0x0
0x808b353: JLE 0x0808b3c7
0x808b355: MOV EBX, dword ptr [EBP + 0x14]
0x808b358: ADD EBX, 0x4
0x808b35b: NEG EBX
0x808b35d: MOV dword ptr [EBP + 0xffffff74], 0x0
0x808b367: CMP dword ptr [EBP + 0xc], 0x2
0x808b36b: JC 0x0808b086
0x808b371: MOV EAX, dword ptr [EBP + 0x8]
0x808b374: MOV word ptr [EAX], 0x202c
0x808b379: MOV byte ptr [EAX + 0x2], 0x0
0x808b37d: ADD EAX, 0x2
0x808b380: PUSH EBX
0x808b381: MOV dword ptr [EBP + 0x8], EAX
0x808b384: MOV EDX, dword ptr [EBP + 0xffffff7c]
0x808b38a: MOV EAX, dword ptr [EBP + 0xffffff74]
0x808b390: ADD EAX, dword ptr [EDX + 0x10]
0x808b393: PUSH EAX
0x808b394: SUB dword ptr [EBP + 0xc], 0x2
0x808b398: PUSH dword ptr [EBP + 0xc]
0x808b39b: PUSH dword ptr [EBP + 0x8]
0x808b39e: CALL 0x0808afc8
0x808b3a3: MOV EDI, EAX
0x808b3a5: MOV ECX, dword ptr [EBP + 0xffffff7c]
0x808b3ab: SUB EAX, dword ptr [EBP + 0x8]
0x808b3ae: SUB dword ptr [EBP + 0xc], EAX
0x808b3b1: INC ESI
0x808b3b2: MOVSX EAX, word ptr [ECX + 0x6]
0x808b3b6: ADD ESP, 0x10
0x808b3b9: ADD dword ptr [EBP + 0xffffff74], 0x18
0x808b3c0: CMP EAX, ESI
0x808b3c2: MOV dword ptr [EBP + 0x8], EDI
0x808b3c5: JG 0x0808b367
0x808b3c7: CMP dword ptr [EBP + 0xc], 0x3
0x808b3cb: JC 0x0808b6ce
0x808b3d1: MOV EAX, dword ptr [EBP + 0x8]
0x808b3d4: MOV dword ptr [EAX], 0x293a20
0x808b3da: ADD EAX, 0x3
0x808b3dd: MOV dword ptr [EBP + 0x8], EAX
0x808b3e0: JMP 0x0808b103
0x808b3e5: MOV EAX, dword ptr [ECX + 0x14]
0x808b3e8: JMP 0x0808b254
0x808b3ed: MOV EAX, dword ptr [EBP + 0x8]
0x808b3f0: MOV byte ptr [EAX], 0x0
0x808b3f3: JMP 0x0808b08c
0x808b3f8: MOV EAX, dword ptr [EBP + 0xc]
0x808b3fb: MOV ECX, dword ptr [EBP + 0x10]
0x808b3fe: TEST EAX, EAX
0x808b400: MOV ESI, dword ptr [ECX + 0x8]
0x808b403: JZ 0x0808b6ce
0x808b409: MOV EAX, dword ptr [EBP + 0x8]
0x808b40c: MOV byte ptr [EAX], 0x3c
0x808b40f: MOV EDX, dword ptr [EBP + 0x10]
0x808b412: INC EAX
0x808b413: MOV DL, byte ptr [EDX + 0x4]
0x808b416: MOV dword ptr [EBP + 0x8], EAX
0x808b419: MOVZX EAX, DL
0x808b41c: DEC dword ptr [EBP + 0xc]
0x808b41f: CMP EAX, 0x6
0x808b422: MOV byte ptr [EBP + 0xffffff7b], DL
0x808b428: MOV EDX, 0x82b36b1
0x808b42d: JZ 0x0808b485
0x808b42f: JG 0x0808b665
0x808b435: CMP EAX, 0x2
0x808b438: MOV EDX, 0x82acce5
0x808b43d: JZ 0x0808b485
0x808b43f: JG 0x0808b63e
0x808b445: TEST EAX, EAX
0x808b447: MOV EDX, 0x82b04c6
0x808b44c: JZ 0x0808b485
0x808b44e: DEC EAX
0x808b44f: JZ 0x0808b480
0x808b451: SUB ESP, 0x8
0x808b454: PUSH 0x6bf
0x808b459: PUSH 0x82acaf8
0x808b45e: PUSH 0x82accd1
0x808b463: PUSH 0x82a825c
0x808b468: PUSH 0x4
0x808b46a: PUSH 0x2
0x808b46c: CALL 0x080b61c4
0x808b471: ADD ESP, 0x14
0x808b474: PUSH 0x0
0x808b476: CALL 0x080a3980
0x808b47b: ADD ESP, 0x10
0x808b47e: JMP 0x0808b47e
0x808b480: MOV EDX, 0x82b69fc
0x808b485: XOR EAX, EAX
0x808b487: CLD
0x808b488: MOV ECX, 0xffffffff
0x808b48d: MOV EDI, EDX
0x808b48f: SCASB.REPNE ES:EDI
0x808b491: NOT ECX
0x808b493: LEA EBX, [ECX + -0x1]
0x808b496: CMP EBX, dword ptr [EBP + 0xc]
0x808b499: JA 0x0808b103
0x808b49f: MOVZX EAX, byte ptr [EBP + 0xffffff7b]
0x808b4a6: CMP EAX, 0x6
0x808b4a9: MOV EDX, 0x82b36b1
0x808b4ae: JZ 0x0808b506
0x808b4b0: JG 0x0808b5e0
0x808b4b6: CMP EAX, 0x2
0x808b4b9: MOV EDX, 0x82acce5
0x808b4be: JZ 0x0808b506
0x808b4c0: JG 0x0808b5c6
0x808b4c6: TEST EAX, EAX
0x808b4c8: MOV EDX, 0x82b04c6
0x808b4cd: JZ 0x0808b506
0x808b4cf: DEC EAX
0x808b4d0: JZ 0x0808b501
0x808b4d2: SUB ESP, 0x8
0x808b4d5: PUSH 0x6bf
0x808b4da: PUSH 0x82acaf8
0x808b4df: PUSH 0x82accd1
0x808b4e4: PUSH 0x82a825c
0x808b4e9: PUSH 0x4
0x808b4eb: PUSH 0x2
0x808b4ed: CALL 0x080b61c4
0x808b4f2: ADD ESP, 0x14
0x808b4f5: PUSH 0x0
0x808b4f7: CALL 0x080a3980
0x808b4fc: ADD ESP, 0x10
0x808b4ff: JMP 0x0808b4ff
0x808b501: MOV EDX, 0x82b69fc
0x808b506: SUB ESP, 0x8
0x808b509: PUSH EDX
0x808b50a: PUSH dword ptr [EBP + 0x8]
0x808b50d: CALL 0x0804bc54
0x808b512: ADD dword ptr [EBP + 0x8], EBX
0x808b515: ADD ESP, 0x10
0x808b518: SUB dword ptr [EBP + 0xc], EBX
0x808b51b: JZ 0x0808b6ce
0x808b521: MOV EAX, dword ptr [EBP + 0x8]
0x808b524: MOV byte ptr [EAX], 0x3a
0x808b527: INC EAX
0x808b528: MOV dword ptr [EBP + 0x8], EAX
0x808b52b: DEC dword ptr [EBP + 0xc]
0x808b52e: MOV dword ptr [EBP + -0x6c], 0x0
0x808b535: MOV EBX, 0x1c
0x808b53a: MOV CL, BL
0x808b53c: MOV EAX, ESI
0x808b53e: SHR EAX, CL
0x808b540: AND EAX, 0xf
0x808b543: CMP EAX, 0x9
0x808b546: LEA ECX, [EAX + 0x30]
0x808b549: LEA EDX, [EAX + 0x37]
0x808b54c: MOV EDI, dword ptr [EBP + 0xc]
0x808b54f: CMOVG ECX, EDX
0x808b552: TEST EDI, EDI
0x808b554: JZ 0x0808b6ce
0x808b55a: MOV EAX, dword ptr [EBP + 0x8]
0x808b55d: MOV byte ptr [EAX], CL
0x808b55f: INC dword ptr [EBP + -0x6c]
0x808b562: INC EAX
0x808b563: DEC dword ptr [EBP + 0xc]
0x808b566: SUB EBX, 0x4
0x808b569: CMP dword ptr [EBP + -0x6c], 0x7
0x808b56d: MOV dword ptr [EBP + 0x8], EAX
0x808b570: JBE 0x0808b53a
0x808b572: MOV EDX, dword ptr [EBP + 0xc]
0x808b575: TEST EDX, EDX
0x808b577: JZ 0x0808b3f0
0x808b57d: MOV EDX, dword ptr [EBP + 0x8]
0x808b580: MOV byte ptr [EDX], 0x3e
0x808b583: INC EDX
0x808b584: MOV dword ptr [EBP + 0x8], EDX
0x808b587: JMP 0x0808b089
0x808b58c: SUB ESP, 0xc
0x808b58f: PUSH dword ptr [EBP + 0x10]
0x808b592: CALL 0x0808ad98
0x808b597: ADD ESP, 0x10
0x808b59a: CMP dword ptr [EBP + 0xc], 0xb
0x808b59e: JC 0x0808b103
0x808b5a4: MOV EBX, dword ptr [EBP + 0x8]
0x808b5a7: MOV dword ptr [EBX], 0x646e7528
0x808b5ad: MOV dword ptr [EBX + 0x4], 0x6e696665
0x808b5b4: MOV dword ptr [EBX + 0x8], 0x296465
0x808b5bb: ADD EBX, 0xb
0x808b5be: MOV dword ptr [EBP + 0x8], EBX
0x808b5c1: JMP 0x0808b103
0x808b5c6: CMP EAX, 0x4
0x808b5c9: MOV EDX, 0x82aae98
0x808b5ce: JZ 0x0808b506
0x808b5d4: JG 0x0808b611
0x808b5d6: MOV EDX, 0x82aaad0
0x808b5db: JMP 0x0808b506
0x808b5e0: CMP EAX, 0x9
0x808b5e3: MOV EDX, 0x82b684c
0x808b5e8: JZ 0x0808b506
0x808b5ee: JG 0x0808b61b
0x808b5f0: CMP EAX, 0x7
0x808b5f3: MOV EDX, 0x82b36a3
0x808b5f8: JZ 0x0808b506
0x808b5fe: CMP EAX, 0x8
0x808b601: JNZ 0x0808b4d2
0x808b607: MOV EDX, 0x82e1907
0x808b60c: JMP 0x0808b506
0x808b611: MOV EDX, 0x82b13db
0x808b616: JMP 0x0808b506
0x808b61b: CMP EAX, 0x14
0x808b61e: MOV EDX, 0x82acced
0x808b623: JZ 0x0808b506
0x808b629: JG 0x0808b696
0x808b62b: CMP EAX, 0xa
0x808b62e: JNZ 0x0808b4d2
0x808b634: MOV EDX, 0x82b236c
0x808b639: JMP 0x0808b506
0x808b63e: CMP EAX, 0x4
0x808b641: MOV EDX, 0x82aae98
0x808b646: JZ 0x0808b485
0x808b64c: JG 0x0808b6d9
0x808b652: MOV EDX, 0x82aaad0
0x808b657: JMP 0x0808b485
0x808b65c: SUB ESP, 0xc
0x808b65f: PUSH EDX
0x808b660: JMP 0x0808b592
0x808b665: CMP EAX, 0x9
0x808b668: MOV EDX, 0x82b684c
0x808b66d: JZ 0x0808b485
0x808b673: JG 0x0808b6ab
0x808b675: CMP EAX, 0x7
0x808b678: MOV EDX, 0x82b36a3
0x808b67d: JZ 0x0808b485
0x808b683: CMP EAX, 0x8
0x808b686: JNZ 0x0808b451
0x808b68c: MOV EDX, 0x82e1907
0x808b691: JMP 0x0808b485
0x808b696: CMP EAX, 0xff
0x808b69b: JNZ 0x0808b4d2
0x808b6a1: MOV EDX, 0x82aaaea
0x808b6a6: JMP 0x0808b506
0x808b6ab: CMP EAX, 0x14
0x808b6ae: MOV EDX, 0x82acced
0x808b6b3: JZ 0x0808b485
0x808b6b9: JG 0x0808b6e3
0x808b6bb: CMP EAX, 0xa
0x808b6be: JNZ 0x0808b451
0x808b6c4: MOV EDX, 0x82b236c
0x808b6c9: JMP 0x0808b485
0x808b6ce: MOV EBX, dword ptr [EBP + 0x8]
0x808b6d1: MOV byte ptr [EBX], 0x0
0x808b6d4: JMP 0x0808b08c
0x808b6d9: MOV EDX, 0x82b13db
0x808b6de: JMP 0x0808b485
0x808b6e3: CMP EAX, 0xff
0x808b6e8: JNZ 0x0808b451
0x808b6ee: MOV EDX, 0x82aaaea
0x808b6f3: JMP 0x0808b485
0x808b6f8: CMP dword ptr [EBP + 0xc], 0xb
0x808b6fc: JC 0x0808b103
0x808b702: MOV EBX, dword ptr [EBP + 0x8]
0x808b705: MOV dword ptr [EBX], 0x66655228
0x808b70b: MOV dword ptr [EBX + 0x4], 0x6e657265
0x808b712: MOV dword ptr [EBX + 0x8], 0x296563
0x808b719: JMP 0x0808b5bb
0x808b71e: MOV EDX, dword ptr [EBP + 0x10]
0x808b721: MOV EDX, dword ptr [EDX + 0x8]
0x808b724: MOV ESI, dword ptr [EDX + 0x10]
0x808b727: TEST ESI, ESI
0x808b729: MOV dword ptr [EBP + -0x7c], EDX
0x808b72c: JNZ 0x0808b7b3
0x808b732: CMP dword ptr [EBP + 0xc], 0x5
0x808b736: JC 0x0808b103
0x808b73c: MOV EBX, dword ptr [EBP + 0x8]
0x808b73f: MOV dword ptr [EBX], 0x5d205b28
0x808b745: MOV word ptr [EBX + 0x4], 0x29
0x808b74b: ADD EBX, 0x5
0x808b74e: JMP 0x0808b5be
0x808b753: MOV EDX, dword ptr [EBP + 0x10]
0x808b756: MOV EAX, dword ptr [EDX + 0x8]
0x808b759: MOV EAX, dword ptr [EAX + 0x4]
0x808b75c: TEST EAX, EAX
0x808b75e: JNZ 0x0808b8e7
0x808b764: CMP dword ptr [EBP + 0xc], 0x5
0x808b768: JC 0x0808b103
0x808b76e: MOV EBX, dword ptr [EBP + 0x8]
0x808b771: MOV dword ptr [EBX], 0x7d207b28
0x808b777: JMP 0x0808b745
0x808b779: MOV EDX, dword ptr [EBP + 0x10]
0x808b77c: MOV EAX, dword ptr [EDX + 0x8]
0x808b77f: MOV ECX, dword ptr [EAX + 0xc]
0x808b782: TEST ECX, ECX
0x808b784: MOV dword ptr [EBP + -0x70], ECX
0x808b787: JNZ 0x0808ba2b
0x808b78d: CMP dword ptr [EBP + 0xc], 0xe
0x808b791: JC 0x0808b6ce
0x808b797: MOV EDI, dword ptr [EBP + 0x8]
0x808b79a: CLD
0x808b79b: MOV ESI, 0x82acd46
0x808b7a0: MOV ECX, 0x3
0x808b7a5: MOVSD.REP ES:EDI, ESI
0x808b7a7: MOVSW ES:EDI, ESI
0x808b7a9: MOVSB ES:EDI, ESI
0x808b7aa: ADD dword ptr [EBP + 0x8], 0xe
0x808b7ae: JMP 0x0808b103
0x808b7b3: MOV EAX, dword ptr [EBP + -0x7c]
0x808b7b6: MOV EAX, dword ptr [EAX + 0x8]
0x808b7b9: CMP dword ptr [EBP + 0xc], 0x12
0x808b7bd: MOV dword ptr [EBP + -0x80], EAX
0x808b7c0: JC 0x0808b086
0x808b7c6: CLD
0x808b7c7: MOV EDI, dword ptr [EBP + 0x8]
0x808b7ca: MOV ESI, 0x82acd55
0x808b7cf: MOV ECX, 0x4
0x808b7d4: MOVSD.REP ES:EDI, ESI
0x808b7d6: MOVSW ES:EDI, ESI
0x808b7d8: SUB ESP, 0x8
0x808b7db: MOVSB ES:EDI, ESI
0x808b7dc: PUSH 0x0
0x808b7de: PUSH 0x1
0x808b7e0: PUSH 0xa
0x808b7e2: MOV ECX, dword ptr [EBP + -0x7c]
0x808b7e5: PUSH dword ptr [ECX + 0x10]
0x808b7e8: PUSH 0x50
0x808b7ea: LEA ESI, [EBP + -0x68]
0x808b7ed: PUSH ESI
0x808b7ee: ADD dword ptr [EBP + 0x8], 0x12
0x808b7f2: SUB dword ptr [EBP + 0xc], 0x12
0x808b7f6: MOV EDI, ESI
0x808b7f8: CALL 0x080b6304
0x808b7fd: XOR EAX, EAX
0x808b7ff: CLD
0x808b800: MOV ECX, 0xffffffff
0x808b805: SCASB.REPNE ES:EDI
0x808b807: NOT ECX
0x808b809: LEA EBX, [ECX + -0x1]
0x808b80c: ADD ESP, 0x20
0x808b80f: CMP EBX, dword ptr [EBP + 0xc]
0x808b812: JA 0x0808b6ce
0x808b818: SUB ESP, 0x8
0x808b81b: PUSH ESI
0x808b81c: PUSH dword ptr [EBP + 0x8]
0x808b81f: CALL 0x0804bc54
0x808b824: SUB dword ptr [EBP + 0xc], EBX
0x808b827: ADD dword ptr [EBP + 0x8], EBX
0x808b82a: ADD ESP, 0x10
0x808b82d: CMP dword ptr [EBP + 0xc], 0x4
0x808b831: JC 0x0808b3ed
0x808b837: MOV EDX, dword ptr [EBP + 0x8]
0x808b83a: MOV dword ptr [EDX], 0xa2f2a20
0x808b840: MOV byte ptr [EDX + 0x4], 0x0
0x808b844: MOV ECX, dword ptr [EBP + 0x14]
0x808b847: ADD EDX, 0x4
0x808b84a: MOV dword ptr [EBP + 0x8], EDX
0x808b84d: SUB dword ptr [EBP + 0xc], 0x4
0x808b851: LEA ESI, [ECX + 0x4]
0x808b854: MOV EBX, dword ptr [EBP + -0x7c]
0x808b857: MOV EAX, dword ptr [EBX + 0x14]
0x808b85a: MOV EDX, dword ptr [EBP + -0x80]
0x808b85d: MOV EBX, dword ptr [EAX + EDX*0x4]
0x808b860: TEST EBX, EBX
0x808b862: JZ 0x0808bae6
0x808b868: PUSH ESI
0x808b869: LEA EAX, [EBX + 0x4]
0x808b86c: PUSH EAX
0x808b86d: PUSH dword ptr [EBP + 0xc]
0x808b870: PUSH dword ptr [EBP + 0x8]
0x808b873: CALL 0x0808afc8
0x808b878: MOV EDI, EAX
0x808b87a: SUB EAX, dword ptr [EBP + 0x8]
0x808b87d: SUB dword ptr [EBP + 0xc], EAX
0x808b880: ADD ESP, 0x10
0x808b883: CMP dword ptr [EBP + 0xc], 0x3
0x808b887: MOV dword ptr [EBP + 0x8], EDI
0x808b88a: JC 0x0808b6ce
0x808b890: MOV EAX, ESI
0x808b892: NEG EAX
0x808b894: MOV dword ptr [EDI], 0x203a20
0x808b89a: PUSH EAX
0x808b89b: LEA EAX, [EBX + 0x1c]
0x808b89e: PUSH EAX
0x808b89f: SUB dword ptr [EBP + 0xc], 0x3
0x808b8a3: ADD dword ptr [EBP + 0x8], 0x3
0x808b8a7: PUSH dword ptr [EBP + 0xc]
0x808b8aa: PUSH dword ptr [EBP + 0x8]
0x808b8ad: CALL 0x0808afc8
0x808b8b2: MOV EDI, EAX
0x808b8b4: SUB EAX, dword ptr [EBP + 0x8]
0x808b8b7: SUB dword ptr [EBP + 0xc], EAX
0x808b8ba: ADD ESP, 0x10
0x808b8bd: CMP dword ptr [EBP + 0xc], 0x2
0x808b8c1: MOV dword ptr [EBP + 0x8], EDI
0x808b8c4: JC 0x0808b3ed
0x808b8ca: MOV word ptr [EDI], 0xa2c
0x808b8cf: MOV ECX, dword ptr [EBP + 0x8]
0x808b8d2: MOV byte ptr [ECX + 0x2], 0x0
0x808b8d6: SUB dword ptr [EBP + 0xc], 0x2
0x808b8da: ADD ECX, 0x2
0x808b8dd: MOV EBX, dword ptr [EBX]
0x808b8df: MOV dword ptr [EBP + 0x8], ECX
0x808b8e2: JMP 0x0808b860
0x808b8e7: CMP dword ptr [EBP + 0xc], 0x12
0x808b8eb: JC 0x0808b3ed
0x808b8f1: CLD
0x808b8f2: MOV EDI, dword ptr [EBP + 0x8]
0x808b8f5: MOV ESI, 0x82acd68
0x808b8fa: MOV ECX, 0x4
0x808b8ff: MOVSD.REP ES:EDI, ESI
0x808b901: MOVSW ES:EDI, ESI
0x808b903: SUB ESP, 0x8
0x808b906: MOVSB ES:EDI, ESI
0x808b907: PUSH 0x0
0x808b909: PUSH 0x1
0x808b90b: PUSH 0xa
0x808b90d: MOV EDX, dword ptr [EBP + 0x10]
0x808b910: MOV EAX, dword ptr [EDX + 0x8]
0x808b913: PUSH dword ptr [EAX + 0x4]
0x808b916: PUSH 0x50
0x808b918: LEA ESI, [EBP + -0x68]
0x808b91b: PUSH ESI
0x808b91c: ADD dword ptr [EBP + 0x8], 0x12
0x808b920: SUB dword ptr [EBP + 0xc], 0x12
0x808b924: MOV EDI, ESI
0x808b926: CALL 0x080b6304
0x808b92b: XOR EAX, EAX
0x808b92d: CLD
0x808b92e: MOV ECX, 0xffffffff
0x808b933: SCASB.REPNE ES:EDI
0x808b935: NOT ECX
0x808b937: LEA EBX, [ECX + -0x1]
0x808b93a: ADD ESP, 0x20
0x808b93d: CMP EBX, dword ptr [EBP + 0xc]
0x808b940: JA 0x0808b103
0x808b946: SUB ESP, 0x8
0x808b949: PUSH ESI
0x808b94a: PUSH dword ptr [EBP + 0x8]
0x808b94d: CALL 0x0804bc54
0x808b952: SUB dword ptr [EBP + 0xc], EBX
0x808b955: ADD dword ptr [EBP + 0x8], EBX
0x808b958: ADD ESP, 0x10
0x808b95b: CMP dword ptr [EBP + 0xc], 0x4
0x808b95f: JC 0x0808b6ce
0x808b965: MOV EAX, dword ptr [EBP + 0x8]
0x808b968: MOV byte ptr [EAX + 0x4], 0x0
0x808b96c: MOV dword ptr [EAX], 0xa2f2a20
0x808b972: MOV EDX, dword ptr [EBP + 0x10]
0x808b975: ADD EAX, 0x4
0x808b978: MOV dword ptr [EBP + -0x6c], 0x0
0x808b97f: MOV dword ptr [EBP + 0x8], EAX
0x808b982: MOV EAX, dword ptr [EDX + 0x8]
0x808b985: MOV EDI, dword ptr [EAX + 0x4]
0x808b988: SUB dword ptr [EBP + 0xc], 0x4
0x808b98c: TEST EDI, EDI
0x808b98e: JZ 0x0808b9ea
0x808b990: MOV ECX, dword ptr [EBP + 0x14]
0x808b993: LEA EBX, [ECX + 0x4]
0x808b996: PUSH EBX
0x808b997: MOV EAX, dword ptr [EAX + 0xc]
0x808b99a: MOV EDX, dword ptr [EBP + -0x6c]
0x808b99d: PUSH dword ptr [EAX + EDX*0x4]
0x808b9a0: PUSH dword ptr [EBP + 0xc]
0x808b9a3: PUSH dword ptr [EBP + 0x8]
0x808b9a6: CALL 0x0808afc8
0x808b9ab: MOV EDI, EAX
0x808b9ad: SUB EAX, dword ptr [EBP + 0x8]
0x808b9b0: SUB dword ptr [EBP + 0xc], EAX
0x808b9b3: ADD ESP, 0x10
0x808b9b6: CMP dword ptr [EBP + 0xc], 0x2
0x808b9ba: MOV dword ptr [EBP + 0x8], EDI
0x808b9bd: JC 0x0808b103
0x808b9c3: MOV word ptr [EDI], 0xa2c
0x808b9c8: MOV ECX, dword ptr [EBP + 0x8]
0x808b9cb: MOV byte ptr [ECX + 0x2], 0x0
0x808b9cf: INC dword ptr [EBP + -0x6c]
0x808b9d2: ADD ECX, 0x2
0x808b9d5: MOV EDX, dword ptr [EBP + 0x10]
0x808b9d8: MOV dword ptr [EBP + 0x8], ECX
0x808b9db: MOV EAX, dword ptr [EDX + 0x8]
0x808b9de: MOV ECX, dword ptr [EBP + -0x6c]
0x808b9e1: SUB dword ptr [EBP + 0xc], 0x2
0x808b9e5: CMP dword ptr [EAX + 0x4], ECX
0x808b9e8: JA 0x0808b996
0x808b9ea: MOV EBX, dword ptr [EBP + 0xc]
0x808b9ed: CMP dword ptr [EBP + 0x14], EBX
0x808b9f0: JA 0x0808b3ed
0x808b9f6: CMP dword ptr [EBP + 0x14], 0x0
0x808b9fa: JBE 0x0808ba14
0x808b9fc: MOV EDX, dword ptr [EBP + 0x14]
0x808b9ff: MOV dword ptr [EBP + -0x6c], EDX
0x808ba02: MOV ECX, dword ptr [EBP + 0x8]
0x808ba05: MOV byte ptr [ECX], 0x20
0x808ba08: DEC dword ptr [EBP + 0xc]
0x808ba0b: INC ECX
0x808ba0c: DEC dword ptr [EBP + -0x6c]
0x808ba0f: MOV dword ptr [EBP + 0x8], ECX
0x808ba12: JNZ 0x0808ba02
0x808ba14: CMP dword ptr [EBP + 0xc], 0x2
0x808ba18: JC 0x0808b6ce
0x808ba1e: MOV EAX, dword ptr [EBP + 0x8]
0x808ba21: MOV word ptr [EAX], 0x297d
0x808ba26: JMP 0x0808bb2f
0x808ba2b: CMP dword ptr [EBP + -0x70], 0x200
0x808ba32: JA 0x0808bc09
0x808ba38: XOR ESI, ESI
0x808ba3a: MOV EAX, dword ptr [EAX + 0x10]
0x808ba3d: CMP ESI, dword ptr [EBP + -0x70]
0x808ba40: MOV dword ptr [EBP + -0x6c], 0x0
0x808ba47: MOV dword ptr [EBP + -0x74], EAX
0x808ba4a: JNC 0x0808b103
0x808ba50: MOV EDX, dword ptr [EBP + -0x70]
0x808ba53: DEC EDX
0x808ba54: MOV dword ptr [EBP + -0x78], EDX
0x808ba57: MOV ECX, dword ptr [EBP + -0x74]
0x808ba5a: MOV DL, byte ptr [ECX]
0x808ba5c: MOV AL, DL
0x808ba5e: SHR AL, 0x4
0x808ba61: AND EDX, 0xf
0x808ba64: LEA EBX, [EAX + 0x37]
0x808ba67: CMP AL, 0x9
0x808ba69: LEA ECX, [EAX + 0x30]
0x808ba6c: CMOVA ECX, EBX
0x808ba6f: LEA EAX, [EDX + 0x30]
0x808ba72: CMP DL, 0x9
0x808ba75: LEA EBX, [EDX + 0x37]
0x808ba78: CMOVA EAX, EBX
0x808ba7b: MOV byte ptr [ESI + EBP*0x1 + -0x68], CL
0x808ba7f: INC ESI
0x808ba80: MOV byte ptr [ESI + EBP*0x1 + -0x68], AL
0x808ba84: MOV EAX, dword ptr [EBP + -0x6c]
0x808ba87: AND EAX, 0xf
0x808ba8a: INC ESI
0x808ba8b: CMP EAX, 0xf
0x808ba8e: JZ 0x0808bbc2
0x808ba94: MOV EAX, dword ptr [EBP + -0x6c]
0x808ba97: AND EAX, 0x7
0x808ba9a: LEA EDX, [ESI + EBP*0x1 + -0x68]
0x808ba9e: INC ESI
0x808ba9f: CMP EAX, 0x7
0x808baa2: JZ 0x0808bb3b
0x808baa8: MOV AL, 0x20
0x808baaa: MOV byte ptr [EDX], AL
0x808baac: INC dword ptr [EBP + -0x6c]
0x808baaf: MOV EBX, dword ptr [EBP + -0x70]
0x808bab2: INC dword ptr [EBP + -0x74]
0x808bab5: CMP dword ptr [EBP + -0x6c], EBX
0x808bab8: JC 0x0808ba57
0x808baba: TEST ESI, ESI
0x808babc: JZ 0x0808b103
0x808bac2: TEST byte ptr [EBP + -0x6c], 0xf
0x808bac6: JZ 0x0808bb4e
0x808bacc: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bad1: INC ESI
0x808bad2: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bad7: INC ESI
0x808bad8: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808badd: INC dword ptr [EBP + -0x6c]
0x808bae0: INC ESI
0x808bae1: INC dword ptr [EBP + -0x74]
0x808bae4: JMP 0x0808bac2
0x808bae6: DEC dword ptr [EBP + -0x80]
0x808bae9: CMP dword ptr [EBP + -0x80], -0x1
0x808baed: JNZ 0x0808b854
0x808baf3: MOV EBX, dword ptr [EBP + 0xc]
0x808baf6: CMP dword ptr [EBP + 0x14], EBX
0x808baf9: JA 0x0808b3ed
0x808baff: CMP dword ptr [EBP + 0x14], 0x0
0x808bb03: JBE 0x0808bb1d
0x808bb05: MOV EDX, dword ptr [EBP + 0x14]
0x808bb08: MOV dword ptr [EBP + -0x6c], EDX
0x808bb0b: MOV ECX, dword ptr [EBP + 0x8]
0x808bb0e: MOV byte ptr [ECX], 0x20
0x808bb11: DEC dword ptr [EBP + 0xc]
0x808bb14: INC ECX
0x808bb15: DEC dword ptr [EBP + -0x6c]
0x808bb18: MOV dword ptr [EBP + 0x8], ECX
0x808bb1b: JNZ 0x0808bb0b
0x808bb1d: CMP dword ptr [EBP + 0xc], 0x2
0x808bb21: JC 0x0808b6ce
0x808bb27: MOV EAX, dword ptr [EBP + 0x8]
0x808bb2a: MOV word ptr [EAX], 0x295d
0x808bb2f: MOV byte ptr [EAX + 0x2], 0x0
0x808bb33: ADD EAX, 0x2
0x808bb36: JMP 0x0808b3dd
0x808bb3b: MOV ECX, dword ptr [EBP + -0x6c]
0x808bb3e: CMP dword ptr [EBP + -0x78], ECX
0x808bb41: MOV AL, 0x2d
0x808bb43: JNZ 0x0808baaa
0x808bb49: JMP 0x0808baa8
0x808bb4e: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bb53: INC ESI
0x808bb54: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bb59: INC ESI
0x808bb5a: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bb5f: MOV EAX, dword ptr [EBP + -0x6c]
0x808bb62: INC ESI
0x808bb63: XOR ECX, ECX
0x808bb65: SUB EAX, 0x10
0x808bb68: CMP EAX, dword ptr [EBP + -0x70]
0x808bb6b: MOV DL, 0x2e
0x808bb6d: JNC 0x0808bb85
0x808bb6f: MOV EBX, dword ptr [EBP + -0x74]
0x808bb72: MOV DL, byte ptr [EBX + ECX*0x1 + -0x10]
0x808bb76: CMP DL, 0x1f
0x808bb79: JBE 0x0808bbbe
0x808bb7b: CMP DL, 0x7f
0x808bb7e: JZ 0x0808bbbe
0x808bb80: CMP DL, 0xff
0x808bb83: JZ 0x0808bbbe
0x808bb85: INC ECX
0x808bb86: MOV byte ptr [ESI + EBP*0x1 + -0x68], DL
0x808bb8a: INC EAX
0x808bb8b: INC ESI
0x808bb8c: CMP ECX, 0xf
0x808bb8f: JLE 0x0808bb68
0x808bb91: LEA EDX, [EBP + -0x68]
0x808bb94: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0xa
0x808bb99: MOV byte ptr [ESI + EBP*0x1 + -0x67], 0x0
0x808bb9e: XOR EAX, EAX
0x808bba0: CLD
0x808bba1: MOV ECX, 0xffffffff
0x808bba6: MOV EDI, EDX
0x808bba8: SCASB.REPNE ES:EDI
0x808bbaa: NOT ECX
0x808bbac: LEA EBX, [ECX + -0x1]
0x808bbaf: CMP EBX, dword ptr [EBP + 0xc]
0x808bbb2: JA 0x0808b3ed
0x808bbb8: SUB ESP, 0x8
0x808bbbb: PUSH EDX
0x808bbbc: JMP 0x0808bc3c
0x808bbbe: MOV DL, 0x2e
0x808bbc0: JMP 0x0808bb85
0x808bbc2: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bbc7: INC ESI
0x808bbc8: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bbcd: INC ESI
0x808bbce: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bbd3: INC ESI
0x808bbd4: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bbd9: XOR ECX, ECX
0x808bbdb: INC ESI
0x808bbdc: JMP 0x0808bbf9
0x808bbe0: CMP DL, 0x7f
0x808bbe3: JZ 0x0808bc05
0x808bbe5: CMP DL, 0xff
0x808bbe8: JZ 0x0808bc05
0x808bbea: INC ECX
0x808bbeb: MOV byte ptr [ESI + EBP*0x1 + -0x68], DL
0x808bbef: INC ESI
0x808bbf0: CMP ECX, 0xf
0x808bbf3: JG 0x0808bd10
0x808bbf9: MOV EBX, dword ptr [EBP + -0x74]
0x808bbfc: MOV DL, byte ptr [EBX + ECX*0x1 + -0xf]
0x808bc00: CMP DL, 0x1f
0x808bc03: JA 0x0808bbe0
0x808bc05: MOV DL, 0x2e
0x808bc07: JMP 0x0808bbea
0x808bc09: PUSH EAX
0x808bc0a: PUSH dword ptr [EBP + -0x70]
0x808bc0d: PUSH 0x82acd7b
0x808bc12: LEA ESI, [EBP + -0x68]
0x808bc15: PUSH ESI
0x808bc16: CALL 0x0804bb34
0x808bc1b: MOV EDI, ESI
0x808bc1d: XOR EAX, EAX
0x808bc1f: CLD
0x808bc20: MOV ECX, 0xffffffff
0x808bc25: SCASB.REPNE ES:EDI
0x808bc27: NOT ECX
0x808bc29: LEA EBX, [ECX + -0x1]
0x808bc2c: ADD ESP, 0x10
0x808bc2f: CMP EBX, dword ptr [EBP + 0xc]
0x808bc32: JA 0x0808b3ed
0x808bc38: SUB ESP, 0x8
0x808bc3b: PUSH ESI
0x808bc3c: PUSH dword ptr [EBP + 0x8]
0x808bc3f: CALL 0x0804bc54
0x808bc44: ADD dword ptr [EBP + 0x8], EBX
0x808bc47: JMP 0x0808b1d3
0x808bc4c: MOV EAX, dword ptr [EBP + 0xc]
0x808bc4f: TEST EAX, EAX
0x808bc51: JZ 0x0808b103
0x808bc57: MOV EBX, dword ptr [EBP + 0x8]
0x808bc5a: MOV byte ptr [EBX], 0x22
0x808bc5d: MOV EDX, dword ptr [EBP + 0x10]
0x808bc60: MOV EAX, dword ptr [EDX + 0x8]
0x808bc63: INC EBX
0x808bc64: MOV EDX, dword ptr [EAX + 0xc]
0x808bc67: MOV dword ptr [EBP + 0x8], EBX
0x808bc6a: DEC dword ptr [EBP + 0xc]
0x808bc6d: XOR EAX, EAX
0x808bc6f: CLD
0x808bc70: MOV ECX, 0xffffffff
0x808bc75: MOV EDI, EDX
0x808bc77: SCASB.REPNE ES:EDI
0x808bc79: NOT ECX
0x808bc7b: LEA EBX, [ECX + -0x1]
0x808bc7e: CMP EBX, dword ptr [EBP + 0xc]
0x808bc81: JA 0x0808b103
0x808bc87: SUB ESP, 0x8
0x808bc8a: PUSH EDX
0x808bc8b: PUSH dword ptr [EBP + 0x8]
0x808bc8e: CALL 0x0804bc54
0x808bc93: ADD dword ptr [EBP + 0x8], EBX
0x808bc96: ADD ESP, 0x10
0x808bc99: CMP dword ptr [EBP + 0xc], EBX
0x808bc9c: JZ 0x0808b6ce
0x808bca2: MOV EAX, dword ptr [EBP + 0x8]
0x808bca5: MOV byte ptr [EAX], 0x22
0x808bca8: INC EAX
0x808bca9: JMP 0x0808b3dd
0x808bcae: MOV EDX, dword ptr [EBP + 0x10]
0x808bcb1: SUB ESP, 0x8
0x808bcb4: FLD float ptr [EDX + 0x8]
0x808bcb7: FSTP double ptr [ESP]
0x808bcba: LEA ESI, [EBP + -0x68]
0x808bcbd: PUSH 0x82acd87
0x808bcc2: PUSH ESI
0x808bcc3: CALL 0x0804bb34
0x808bcc8: MOV EDI, ESI
0x808bcca: XOR EAX, EAX
0x808bccc: CLD
0x808bccd: MOV ECX, 0xffffffff
0x808bcd2: SCASB.REPNE ES:EDI
0x808bcd4: NOT ECX
0x808bcd6: LEA EBX, [ECX + -0x1]
0x808bcd9: ADD ESP, 0x10
0x808bcdc: CMP EBX, dword ptr [EBP + 0xc]
0x808bcdf: JA 0x0808b103
0x808bce5: JMP 0x0808bc38
0x808bcea: CMP dword ptr [EBP + 0xc], 0x6
0x808bcee: JC 0x0808b6ce
0x808bcf4: MOV EDI, dword ptr [EBP + 0x8]
0x808bcf7: CLD
0x808bcf8: MOV ESI, 0x82acd8a
0x808bcfd: MOV ECX, 0x1
0x808bd02: MOVSD.REP ES:EDI, ESI
0x808bd04: MOVSW ES:EDI, ESI
0x808bd06: MOVSB ES:EDI, ESI
0x808bd07: ADD dword ptr [EBP + 0x8], 0x6
0x808bd0b: JMP 0x0808b103
0x808bd10: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0x20
0x808bd15: INC ESI
0x808bd16: MOV byte ptr [ESI + EBP*0x1 + -0x68], 0xa
0x808bd1b: MOV byte ptr [ESI + EBP*0x1 + -0x67], 0x0
0x808bd20: XOR EAX, EAX
0x808bd22: CLD
0x808bd23: MOV ECX, 0xffffffff
0x808bd28: LEA EDI, [EBP + -0x68]
0x808bd2b: SCASB.REPNE ES:EDI
0x808bd2d: NOT ECX
0x808bd2f: LEA EBX, [ECX + -0x1]
0x808bd32: CMP EBX, dword ptr [EBP + 0xc]
0x808bd35: JA 0x0808b3ed
0x808bd3b: SUB ESP, 0x8
0x808bd3e: LEA EAX, [EBP + -0x68]
0x808bd41: PUSH EAX
0x808bd42: PUSH dword ptr [EBP + 0x8]
0x808bd45: CALL 0x0804bc54
0x808bd4a: MOV EDX, dword ptr [EBP + -0x6c]
0x808bd4d: ADD dword ptr [EBP + 0x8], EBX
0x808bd50: SUB dword ptr [EBP + 0xc], EBX
0x808bd53: ADD ESP, 0x10
0x808bd56: CMP dword ptr [EBP + -0x78], EDX
0x808bd59: JBE 0x0808bd81
0x808bd5b: MOV ECX, dword ptr [EBP + 0xc]
0x808bd5e: CMP dword ptr [EBP + 0x14], ECX
0x808bd61: JA 0x0808b086
0x808bd67: MOV EAX, dword ptr [EBP + 0x14]
0x808bd6a: TEST EAX, EAX
0x808bd6c: JLE 0x0808bd81
0x808bd6e: MOV ESI, dword ptr [EBP + 0x14]
0x808bd71: MOV EBX, dword ptr [EBP + 0x8]
0x808bd74: MOV byte ptr [EBX], 0x20
0x808bd77: DEC dword ptr [EBP + 0xc]
0x808bd7a: INC EBX
0x808bd7b: DEC ESI
0x808bd7c: MOV dword ptr [EBP + 0x8], EBX
0x808bd7f: JNZ 0x0808bd71
0x808bd81: XOR ESI, ESI
0x808bd83: JMP 0x0808baac
```

## pdb_freeMemory @ 0x80ceb94

Evidence: `ev_6e330b55533e99e1747ee4303d288a6a5ee66bce5902b04f191a59b5cbec2386`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80cebea", "start": "0x80ceb94"}, {"end": "0x80cedfe", "start": "0x80cebec"}], "span_bytes": 619, "total_bytes": 618}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "Ghidra Listing reports a terminal call at 0x80ceca5 to 0x80a3cc8 _vm_leaveCriticalSection (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

void pdb_freeMemory(void *ptr,char *moduleName,char *file,uint line)

{
  Pdb_Memory_Header_T *pBlock;
  byte bVar1;
  ushort uVar2;
  
                    /* Unresolved local var: Pdb_Memory_Grp_T * pGrp@[DW_OP_reg2(EDX)]
                       Unresolved local var: Pdb_Memory_Header_T * pBlock@[DW_OP_reg1(ECX)] */
  if (file == (char *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","file != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x454);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  if (moduleName == (char *)0x0) {
    pdb_syslogf(2,4,"assertion failed: %s\n","moduleName != NULL");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x455);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  if (_initMemMgrFlag == 0) {
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","PDB is not initialized.\n",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x459);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  _vm_enterCriticalSection(&_memMgrSection);
  if (ptr != (void *)0x0) {
    pBlock = (Pdb_Memory_Header_T *)((int)ptr + -0xc);
    if (*(char *)((int)&pBlock[1].size + *(int *)((int)ptr + -0xc)) != -0x51) {
      pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad block (Tail flag is error 2).\n"
                  ,"/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x46c);
      _vm_endProcess(0);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    uVar2 = *(ushort *)((int)ptr + -6);
    if ((uVar2 & 0x10) != 0) {
      _pdb_totalUsedSize = _pdb_totalUsedSize - pBlock->size;
      _pdb_memStatFree = _pdb_memStatFree + 1;
      if ((uVar2 & 1) == 0) {
        *(ushort *)((int)ptr + -6) = uVar2 & 0xffef;
        pdb_l2FreeMemory(pBlock);
      }
      else {
        *(ushort *)((int)ptr + -6) = uVar2 & 0xffef;
        bVar1 = *(byte *)((int)ptr + -8);
        if (_pdbBlockGroups <= bVar1) {
          pdb_syslogf(2,4,"assertion failed: %s\n","pBlock->serial < _pdbBlockGroups");
          pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                      "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x48b);
          _vm_endProcess(0);
          do {
                    /* WARNING: Do nothing block with infinite loop */
          } while( true );
        }
        *(Pdb_Memory_Header_T **)((int)ptr + -4) = _pMemoryGrp[bVar1].pFree;
        _pMemoryGrp[bVar1].pFree = pBlock;
      }
      _vm_leaveCriticalSection(&_memMgrSection);
      return;
    }
    pdb_syslogf(2,4,"assertion failed: %s\n","pBlock->flags & PDB_MEMORY_ALLOCATED");
    pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
                "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x470);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  pdb_syslogf(2,4,"assertion failed: %s\n","ptr != NULL");
  pdb_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Assert error",
              "/home/cc/xxxx/gameshell/src/vm/pdb_memmgr.c",0x460);
  _vm_endProcess(0);
  do {
                    /* WARNING: Do nothing block with infinite loop */
  } while( true );
}


```

```text
0x80ceb94: PUSH EBP
0x80ceb95: MOV EBP, ESP
0x80ceb97: PUSH EBX
0x80ceb98: PUSH EBX
0x80ceb99: MOV ECX, dword ptr [EBP + 0x10]
0x80ceb9c: TEST ECX, ECX
0x80ceb9e: MOV EBX, dword ptr [EBP + 0x8]
0x80ceba1: MOV EAX, dword ptr [EBP + 0xc]
0x80ceba4: JZ 0x080cedbe
0x80cebaa: TEST EAX, EAX
0x80cebac: JZ 0x080ced7d
0x80cebb2: MOV ECX, dword ptr [0x0860cff0]
0x80cebb8: TEST ECX, ECX
0x80cebba: JNZ 0x080cebec
0x80cebbc: SUB ESP, 0x8
0x80cebbf: PUSH 0x459
0x80cebc4: PUSH 0x82b5b34
0x80cebc9: PUSH 0x82b5fb3
0x80cebce: PUSH 0x82a825c
0x80cebd3: PUSH 0x4
0x80cebd5: PUSH 0x2
0x80cebd7: CALL 0x080cc868
0x80cebdc: ADD ESP, 0x14
0x80cebdf: PUSH 0x0
0x80cebe1: CALL 0x080a3980
0x80cebe6: ADD ESP, 0x10
0x80cebe9: JMP 0x080cebe9
0x80cebec: SUB ESP, 0xc
0x80cebef: PUSH 0x860d158
0x80cebf4: CALL 0x080a3c7c
0x80cebf9: ADD ESP, 0x10
0x80cebfc: TEST EBX, EBX
0x80cebfe: JZ 0x080cecaa
0x80cec04: LEA ECX, [EBX + -0xc]
0x80cec07: MOV EAX, dword ptr [EBX + -0xc]
0x80cec0a: CMP byte ptr [EAX + ECX*0x1 + 0xc], 0xaf
0x80cec0f: JZ 0x080cec40
0x80cec11: SUB ESP, 0x8
0x80cec14: PUSH 0x46c
0x80cec19: PUSH 0x82b5b34
0x80cec1e: PUSH 0x82b5c00
0x80cec23: PUSH 0x82a825c
0x80cec28: PUSH 0x4
0x80cec2a: PUSH 0x2
0x80cec2c: CALL 0x080cc868
0x80cec31: ADD ESP, 0x14
0x80cec34: PUSH 0x0
0x80cec36: CALL 0x080a3980
0x80cec3b: ADD ESP, 0x10
0x80cec3e: JMP 0x080cec3e
0x80cec40: MOV AX, word ptr [ECX + 0x6]
0x80cec44: TEST AL, 0x10
0x80cec46: JZ 0x080ced3c
0x80cec4c: MOV EDX, EAX
0x80cec4e: AND EDX, 0xffffffef
0x80cec51: MOV EAX, [0x0860d010]
0x80cec56: SUB EAX, dword ptr [ECX]
0x80cec58: INC dword ptr [0x0860d018]
0x80cec5e: TEST DL, 0x1
0x80cec61: MOV [0x0860d010], EAX
0x80cec66: JZ 0x080ced2c
0x80cec6c: MOV word ptr [ECX + 0x6], DX
0x80cec70: MOV DL, byte ptr [ECX + 0x4]
0x80cec73: MOVZX EAX, DL
0x80cec76: CMP EAX, dword ptr [0x083d0150]
0x80cec7c: JNC 0x080ceceb
0x80cec7e: MOVZX EAX, DL
0x80cec81: LEA EDX, [EAX*0x8 + 0x0]
0x80cec88: SUB EDX, EAX
0x80cec8a: LEA EDX, [EDX*0x4 + 0x860d040]
0x80cec91: MOV EAX, dword ptr [EDX + 0x14]
0x80cec94: MOV dword ptr [ECX + 0x8], EAX
0x80cec97: MOV dword ptr [EDX + 0x14], ECX
0x80cec9a: MOV dword ptr [EBP + 0x8], 0x860d158
0x80ceca1: MOV EBX, dword ptr [EBP + -0x4]
0x80ceca4: LEAVE
0x80ceca5: JMP 0x080a3cc8
0x80cecaa: PUSH 0x82b5fdb
0x80cecaf: PUSH 0x82b5ab5
0x80cecb4: PUSH 0x4
0x80cecb6: PUSH 0x2
0x80cecb8: CALL 0x080cc868
0x80cecbd: POP EAX
0x80cecbe: POP EDX
0x80cecbf: PUSH 0x460
0x80cecc4: PUSH 0x82b5b34
0x80cecc9: PUSH 0x82b5acb
0x80cecce: PUSH 0x82a825c
0x80cecd3: PUSH 0x4
0x80cecd5: PUSH 0x2
0x80cecd7: CALL 0x080cc868
0x80cecdc: ADD ESP, 0x14
0x80cecdf: PUSH 0x0
0x80cece1: CALL 0x080a3980
0x80cece6: ADD ESP, 0x10
0x80cece9: JMP 0x080cece9
0x80ceceb: PUSH 0x82b5c24
0x80cecf0: PUSH 0x82b5ab5
0x80cecf5: PUSH 0x4
0x80cecf7: PUSH 0x2
0x80cecf9: CALL 0x080cc868
0x80cecfe: POP EAX
0x80cecff: POP EDX
0x80ced00: PUSH 0x48b
0x80ced05: PUSH 0x82b5b34
0x80ced0a: PUSH 0x82b5acb
0x80ced0f: PUSH 0x82a825c
0x80ced14: PUSH 0x4
0x80ced16: PUSH 0x2
0x80ced18: CALL 0x080cc868
0x80ced1d: ADD ESP, 0x14
0x80ced20: PUSH 0x0
0x80ced22: CALL 0x080a3980
0x80ced27: ADD ESP, 0x10
0x80ced2a: JMP 0x080ced2a
0x80ced2c: MOV word ptr [ECX + 0x6], DX
0x80ced30: MOV EAX, ECX
0x80ced32: CALL 0x080cea34
0x80ced37: JMP 0x080cec9a
0x80ced3c: PUSH 0x82b5c48
0x80ced41: PUSH 0x82b5ab5
0x80ced46: PUSH 0x4
0x80ced48: PUSH 0x2
0x80ced4a: CALL 0x080cc868
0x80ced4f: POP ECX
0x80ced50: POP EBX
0x80ced51: PUSH 0x470
0x80ced56: PUSH 0x82b5b34
0x80ced5b: PUSH 0x82b5acb
0x80ced60: PUSH 0x82a825c
0x80ced65: PUSH 0x4
0x80ced67: PUSH 0x2
0x80ced69: CALL 0x080cc868
0x80ced6e: ADD ESP, 0x14
0x80ced71: PUSH 0x0
0x80ced73: CALL 0x080a3980
0x80ced78: ADD ESP, 0x10
0x80ced7b: JMP 0x080ced7b
0x80ced7d: PUSH 0x82b5f93
0x80ced82: PUSH 0x82b5ab5
0x80ced87: PUSH 0x4
0x80ced89: PUSH 0x2
0x80ced8b: CALL 0x080cc868
0x80ced90: POP EBX
0x80ced91: POP EAX
0x80ced92: PUSH 0x455
0x80ced97: PUSH 0x82b5b34
0x80ced9c: PUSH 0x82b5acb
0x80ceda1: PUSH 0x82a825c
0x80ceda6: PUSH 0x4
0x80ceda8: PUSH 0x2
0x80cedaa: CALL 0x080cc868
0x80cedaf: ADD ESP, 0x14
0x80cedb2: PUSH 0x0
0x80cedb4: CALL 0x080a3980
0x80cedb9: ADD ESP, 0x10
0x80cedbc: JMP 0x080cedbc
0x80cedbe: PUSH 0x82b5fa6
0x80cedc3: PUSH 0x82b5ab5
0x80cedc8: PUSH 0x4
0x80cedca: PUSH 0x2
0x80cedcc: CALL 0x080cc868
0x80cedd1: POP EAX
0x80cedd2: POP EDX
0x80cedd3: PUSH 0x454
0x80cedd8: PUSH 0x82b5b34
0x80ceddd: PUSH 0x82b5acb
0x80cede2: PUSH 0x82a825c
0x80cede7: PUSH 0x4
0x80cede9: PUSH 0x2
0x80cedeb: CALL 0x080cc868
0x80cedf0: ADD ESP, 0x14
0x80cedf3: PUSH 0x0
0x80cedf5: CALL 0x080a3980
0x80cedfa: ADD ESP, 0x10
0x80cedfd: JMP 0x080cedfd
```

## vm_intToString @ 0x80b6304

Evidence: `ev_78abb8fbaeb569a0b65639715b90654d8ae7cb48efb389df3771b99138ede0b3`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x80b6328", "start": "0x80b6304"}, {"end": "0x80b6369", "start": "0x80b632c"}, {"end": "0x80b6391", "start": "0x80b636c"}], "span_bytes": 142, "total_bytes": 137}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

void vm_intToString(char *szTemp,uint size,int val,int base,int signFlag,int upperCase)

{
  uint uVar1;
  char cVar2;
  char cVar3;
  char *pcVar4;
  
                    /* Unresolved local var: char ch@[DW_OP_reg1(ECX)]
                       Unresolved local var: uint uval@[DW_OP_reg0(EAX)]
                       Unresolved local var: char * p@[DW_OP_reg3(EBX)] */
  if (size != 0) {
    if (size < 2) {
      *szTemp = '\0';
    }
    else {
      uVar1 = val;
      pcVar4 = szTemp;
      if ((signFlag != 0) && (val < 0)) {
        *szTemp = '-';
        pcVar4 = szTemp + 1;
        uVar1 = -val;
        val = uVar1;
      }
      do {
        uVar1 = uVar1 / (uint)base;
        pcVar4 = pcVar4 + 1;
      } while (uVar1 != 0);
      if ((int)size <= (int)pcVar4 - (int)szTemp) {
        *szTemp = '\0';
        return;
      }
      *pcVar4 = '\0';
      do {
        pcVar4 = pcVar4 + -1;
        uVar1 = (uint)val / (uint)base;
        cVar3 = (char)((uint)val % (uint)base);
        if (cVar3 < '\n') {
          cVar2 = cVar3 + '0';
        }
        else {
          cVar2 = cVar3 + 'W';
          if (upperCase != 0) {
            cVar2 = cVar3 + '7';
          }
        }
        *pcVar4 = cVar2;
        val = uVar1;
      } while (uVar1 != 0);
    }
  }
  return;
}


```

```text
0x80b6304: PUSH EBP
0x80b6305: MOV EBP, ESP
0x80b6307: PUSH EDI
0x80b6308: PUSH ESI
0x80b6309: PUSH EBX
0x80b630a: MOV ECX, dword ptr [EBP + 0xc]
0x80b630d: MOV ESI, dword ptr [EBP + 0x8]
0x80b6310: TEST ECX, ECX
0x80b6312: MOV EDI, dword ptr [EBP + 0x14]
0x80b6315: MOV EBX, ESI
0x80b6317: JZ 0x080b6324
0x80b6319: CMP ECX, 0x1
0x80b631c: JA 0x080b632c
0x80b631e: MOV byte ptr [ESI], 0x0
0x80b6321: LEA ESI, [ESI]
0x80b6324: POP EBX
0x80b6325: POP ESI
0x80b6326: POP EDI
0x80b6327: LEAVE
0x80b6328: RET
0x80b632c: MOV EDX, dword ptr [EBP + 0x18]
0x80b632f: TEST EDX, EDX
0x80b6331: JNZ 0x080b6351
0x80b6333: MOV EAX, dword ptr [EBP + 0x10]
0x80b6336: MOV ESI, ESI
0x80b6338: XOR EDX, EDX
0x80b633a: DIV EDI
0x80b633c: INC EBX
0x80b633d: TEST EAX, EAX
0x80b633f: JNZ 0x080b6338
0x80b6341: MOV EDX, EBX
0x80b6343: SUB EDX, ESI
0x80b6345: CMP EDX, ECX
0x80b6347: JL 0x080b6361
0x80b6349: MOV byte ptr [ESI], 0x0
0x80b634c: POP EBX
0x80b634d: POP ESI
0x80b634e: POP EDI
0x80b634f: LEAVE
0x80b6350: RET
0x80b6351: MOV EAX, dword ptr [EBP + 0x10]
0x80b6354: TEST EAX, EAX
0x80b6356: JNS 0x080b6333
0x80b6358: MOV byte ptr [ESI], 0x2d
0x80b635b: INC EBX
0x80b635c: NEG dword ptr [EBP + 0x10]
0x80b635f: JMP 0x080b6333
0x80b6361: MOV byte ptr [EBX], 0x0
0x80b6364: MOV EAX, dword ptr [EBP + 0x10]
0x80b6367: DEC EBX
0x80b6368: JMP 0x080b6376
0x80b636c: ADD ECX, 0x30
0x80b636f: MOV byte ptr [EBX], CL
0x80b6371: DEC EBX
0x80b6372: TEST ESI, ESI
0x80b6374: JZ 0x080b6324
0x80b6376: XOR EDX, EDX
0x80b6378: DIV EDI
0x80b637a: CMP DL, 0x9
0x80b637d: MOV ESI, EAX
0x80b637f: MOV CL, DL
0x80b6381: JLE 0x080b636c
0x80b6383: ADD ECX, 0x57
0x80b6386: LEA EDX, [EDX + 0x37]
0x80b6389: CMP dword ptr [EBP + 0x1c], 0x0
0x80b638d: CMOVNZ ECX, EDX
0x80b6390: JMP 0x080b636f
```

## vm_extendString @ 0x808c6dc

Evidence: `ev_8586e710ba1b29fd3d65723eea725f94ecb7d991e37716a47b030c78514531c4`

Body: `{"available": true, "contains_entry": true, "non_contiguous": false, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808c7fa", "start": "0x808c6dc"}], "span_bytes": 287, "total_bytes": 287}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

Vm_String_T * vm_extendString(Vm_String_T *pString,int len)

{
  uint uVar1;
  Vm_String_T *pVVar2;
  int iVar3;
  
                    /* Unresolved local var: Vm_String_T * dst@[DW_OP_reg3(EBX)] */
  pVVar2 = pString;
  if (len != 0) {
    if ((pString->ref < 2) && ((pString->attrib & 2) == 0)) {
      pVVar2 = pdb_reallocateMemory
                         (pString,len + pString->len + 0x15,"VM",
                          "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x570);
      if (pVVar2 == (Vm_String_T *)0x0) {
        vm_error("No enough memory to extend string.\n");
      }
      pVVar2->ptr = (char *)(pVVar2 + 1);
      pVVar2->hash = 0;
      pVVar2->objHash = 0;
    }
    else {
      pVVar2 = pdb_allocateMemory(len + pString->len + 0x15,"VM",
                                  "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x55e);
      if (pVVar2 == (Vm_String_T *)0x0) {
        vm_error("No enough memory to extend string.\n");
      }
      uVar1 = pString->len;
      pVVar2->len = uVar1;
      pVVar2->attrib = 0;
      pVVar2->ptr = (char *)(pVVar2 + 1);
      pVVar2->hash = 0;
      pVVar2->objHash = 0;
      pVVar2->ref = 1;
      memcpy(pVVar2 + 1,pString->ptr,uVar1 + 1);
      iVar3 = pString->ref + -1;
      pString->ref = iVar3;
      if (iVar3 < 1) {
        if ((pString->attrib & 1) == 0) {
          pdb_freeMemory(pString,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x515);
        }
        else {
          vm_removeSharedString(pString);
        }
      }
    }
  }
  return pVVar2;
}


```

```text
0x808c6dc: PUSH EBP
0x808c6dd: MOV EBP, ESP
0x808c6df: MOV EDX, dword ptr [EBP + 0xc]
0x808c6e2: PUSH ESI
0x808c6e3: TEST EDX, EDX
0x808c6e5: MOV ESI, dword ptr [EBP + 0x8]
0x808c6e8: PUSH EBX
0x808c6e9: MOV EAX, ESI
0x808c6eb: JZ 0x0808c759
0x808c6ed: CMP dword ptr [ESI], 0x1
0x808c6f0: JLE 0x0808c760
0x808c6f2: PUSH 0x55e
0x808c6f7: PUSH 0x82acaf8
0x808c6fc: PUSH 0x82a376b
0x808c701: ADD EDX, dword ptr [ESI + 0x10]
0x808c704: LEA EAX, [EDX + 0x15]
0x808c707: PUSH EAX
0x808c708: CALL 0x080ce784
0x808c70d: ADD ESP, 0x10
0x808c710: TEST EAX, EAX
0x808c712: MOV EBX, EAX
0x808c714: JZ 0x0808c7d4
0x808c71a: MOV EAX, dword ptr [ESI + 0x10]
0x808c71d: LEA EDX, [EBX + 0x14]
0x808c720: MOV dword ptr [EBX + 0x10], EAX
0x808c723: MOV dword ptr [EBX + 0x4], 0x0
0x808c72a: MOV dword ptr [EBX + 0xc], EDX
0x808c72d: MOV word ptr [EBX + 0x8], 0x0
0x808c733: MOV word ptr [EBX + 0xa], 0x0
0x808c739: MOV dword ptr [EBX], 0x1
0x808c73f: INC EAX
0x808c740: PUSH ECX
0x808c741: PUSH EAX
0x808c742: PUSH dword ptr [ESI + 0xc]
0x808c745: PUSH EDX
0x808c746: CALL 0x0804b7e4
0x808c74b: MOV EAX, dword ptr [ESI]
0x808c74d: DEC EAX
0x808c74e: ADD ESP, 0x10
0x808c751: TEST EAX, EAX
0x808c753: MOV dword ptr [ESI], EAX
0x808c755: JLE 0x0808c7a4
0x808c757: MOV EAX, EBX
0x808c759: LEA ESP, [EBP + -0x8]
0x808c75c: POP EBX
0x808c75d: POP ESI
0x808c75e: LEAVE
0x808c75f: RET
0x808c760: TEST byte ptr [ESI + 0x4], 0x2
0x808c764: JNZ 0x0808c6f2
0x808c766: SUB ESP, 0xc
0x808c769: PUSH 0x570
0x808c76e: PUSH 0x82acaf8
0x808c773: PUSH 0x82a376b
0x808c778: ADD EDX, dword ptr [ESI + 0x10]
0x808c77b: LEA EAX, [EDX + 0x15]
0x808c77e: PUSH EAX
0x808c77f: PUSH ESI
0x808c780: CALL 0x080cf588
0x808c785: ADD ESP, 0x20
0x808c788: TEST EAX, EAX
0x808c78a: MOV EBX, EAX
0x808c78c: JZ 0x0808c7e9
0x808c78e: LEA EAX, [EBX + 0x14]
0x808c791: MOV dword ptr [EBX + 0xc], EAX
0x808c794: MOV EAX, EBX
0x808c796: MOV word ptr [EBX + 0x8], 0x0
0x808c79c: MOV word ptr [EBX + 0xa], 0x0
0x808c7a2: JMP 0x0808c759
0x808c7a4: TEST byte ptr [ESI + 0x4], 0x1
0x808c7a8: JZ 0x0808c7ba
0x808c7aa: SUB ESP, 0xc
0x808c7ad: PUSH ESI
0x808c7ae: CALL 0x080bd460
0x808c7b3: ADD ESP, 0x10
0x808c7b6: MOV EAX, EBX
0x808c7b8: JMP 0x0808c759
0x808c7ba: PUSH 0x515
0x808c7bf: PUSH 0x82acaf8
0x808c7c4: PUSH 0x82a376b
0x808c7c9: PUSH ESI
0x808c7ca: CALL 0x080ceb94
0x808c7cf: ADD ESP, 0x10
0x808c7d2: JMP 0x0808c7b6
0x808c7d4: SUB ESP, 0xc
0x808c7d7: PUSH 0x82acc70
0x808c7dc: CALL 0x08097b24
0x808c7e1: ADD ESP, 0x10
0x808c7e4: JMP 0x0808c71a
0x808c7e9: SUB ESP, 0xc
0x808c7ec: PUSH 0x82acc70
0x808c7f1: CALL 0x08097b24
0x808c7f6: ADD ESP, 0x10
0x808c7f9: JMP 0x0808c78e
```

## vm_compareBufWithBufStr @ 0x808cda0

Evidence: `ev_da709b15fb9a5c8a7024ddc3b4967ee4128d489459cc0950606350f75efc7710`

Body: `{"available": true, "contains_entry": true, "non_contiguous": true, "provenance": "ghidra-function-body-address-set", "ranges": [{"end": "0x808ce24", "start": "0x808cda0"}, {"end": "0x808ce38", "start": "0x808ce28"}], "span_bytes": 153, "total_bytes": 150}`

Limitations: ["The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.", "The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.", "Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.", "Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.", "References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.", "The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.", "Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.", "Synthetic Ghidra entry-point references without actionable memory sources are omitted.", "The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.", "macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state."]

```c

int vm_compareBufWithBufStr(Vm_Value_T *pValue1,Vm_Value_T *pValue2,char *cmpType)

{
  uint uVar1;
  uint uVar2;
  byte *pbVar3;
  bool bVar4;
  bool bVar5;
  uint len2;
  uchar *pData2;
  uchar *pData1;
  
                    /* Unresolved local var: uint len1@[DW_OP_reg3(EBX)] */
  pbVar3 = ((pValue1->v).buffer)->ptr;
  uVar1 = ((pValue1->v).buffer)->realSize;
  if (pValue2->type == '\x04') {
    pData2 = (uchar *)((pValue2->v).string)->ptr;
    len2 = ((pValue2->v).string)->len;
  }
  else if (pValue2->type == '\x05') {
    pData2 = ((pValue2->v).buffer)->ptr;
    len2 = ((pValue2->v).buffer)->realSize;
  }
  else {
    vm_error("Bad %s, right isn\'t a string or buffer when left is buffer.\n",cmpType);
  }
                    /* Unresolved local var: int cmpLen@[DW_OP_reg1(ECX)]
                       Unresolved local var: int ret@[DW_OP_reg0(EAX)] */
  uVar2 = len2;
  if ((int)uVar1 < (int)len2) {
    uVar2 = uVar1;
  }
  bVar4 = false;
  bVar5 = true;
  do {
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    bVar4 = *pbVar3 < *pData2;
    bVar5 = *pbVar3 == *pData2;
    pbVar3 = pbVar3 + 1;
    pData2 = pData2 + 1;
  } while (bVar5);
  uVar2 = (uint)(char)((!bVar4 && !bVar5) - bVar4);
  if (uVar2 == 0) {
    uVar2 = 0xffffffff;
    if ((int)len2 <= (int)uVar1) {
      uVar2 = (uint)((int)len2 < (int)uVar1);
    }
  }
  return uVar2;
}


```

```text
0x808cda0: PUSH EBP
0x808cda1: MOV EBP, ESP
0x808cda3: PUSH EDI
0x808cda4: PUSH ESI
0x808cda5: PUSH EBX
0x808cda6: SUB ESP, 0x1c
0x808cda9: MOV EAX, dword ptr [EBP + 0x8]
0x808cdac: MOV EAX, dword ptr [EAX + 0x8]
0x808cdaf: MOV ECX, dword ptr [EAX + 0x10]
0x808cdb2: MOV EDX, dword ptr [EBP + 0xc]
0x808cdb5: MOV dword ptr [EBP + -0x10], ECX
0x808cdb8: MOV EBX, dword ptr [EAX + 0xc]
0x808cdbb: MOV AL, byte ptr [EDX + 0x4]
0x808cdbe: CMP AL, 0x4
0x808cdc0: JZ 0x0808ce28
0x808cdc2: CMP AL, 0x5
0x808cdc4: JZ 0x0808ce14
0x808cdc6: SUB ESP, 0x8
0x808cdc9: PUSH dword ptr [EBP + 0x10]
0x808cdcc: PUSH 0x82acc94
0x808cdd1: CALL 0x08097b24
0x808cdd6: ADD ESP, 0x10
0x808cdd9: CMP dword ptr [EBP + -0x18], EBX
0x808cddc: MOV ECX, dword ptr [EBP + -0x18]
0x808cddf: CMOVG ECX, EBX
0x808cde2: CLD
0x808cde3: CMP ECX, ECX
0x808cde5: MOV ESI, dword ptr [EBP + -0x10]
0x808cde8: MOV EDI, dword ptr [EBP + -0x14]
0x808cdeb: CMPSB.REPE ES:EDI, ESI
0x808cded: SETC AL
0x808cdf0: SETA DL
0x808cdf3: SUB DL, AL
0x808cdf5: MOVSX EAX, DL
0x808cdf8: TEST EAX, EAX
0x808cdfa: JNZ 0x0808ce0c
0x808cdfc: CMP EBX, dword ptr [EBP + -0x18]
0x808cdff: MOV EAX, 0xffffffff
0x808ce04: JL 0x0808ce0c
0x808ce06: SETG AL
0x808ce09: MOVZX EAX, AL
0x808ce0c: LEA ESP, [EBP + -0xc]
0x808ce0f: POP EBX
0x808ce10: POP ESI
0x808ce11: POP EDI
0x808ce12: LEAVE
0x808ce13: RET
0x808ce14: MOV EAX, dword ptr [EDX + 0x8]
0x808ce17: MOV ECX, dword ptr [EAX + 0x10]
0x808ce1a: MOV EAX, dword ptr [EAX + 0xc]
0x808ce1d: MOV dword ptr [EBP + -0x14], ECX
0x808ce20: MOV dword ptr [EBP + -0x18], EAX
0x808ce23: JMP 0x0808cdd9
0x808ce28: MOV EAX, dword ptr [EDX + 0x8]
0x808ce2b: MOV EDX, dword ptr [EAX + 0xc]
0x808ce2e: MOV EAX, dword ptr [EAX + 0x10]
0x808ce31: MOV dword ptr [EBP + -0x14], EDX
0x808ce34: MOV dword ptr [EBP + -0x18], EAX
0x808ce37: JMP 0x0808cdd9
```
