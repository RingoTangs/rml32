# auth-rea-final：Ghidra/REA 函数证据

未执行目标。下面是引擎原始伪代码和反汇编，不保证可编译。

<a id="initGS"></a>

## initGS @ 0x804b750

Evidence: `ev_042524b4f87934c70d97325474b0d7cb35b8837fbfdc52e07e5942aa915642c0`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int initGS(void)

{
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x804b750: PUSH EBP
0x804b751: MOV EBP, ESP
0x804b753: MOV EAX, 0x1
0x804b758: LEAVE
0x804b759: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_getActionRoutine"></a>

## vm_getActionRoutine @ 0x8070cb0

Evidence: `ev_091731c63ff0c8be137f0c0ed44297d966bebccba7f957a697acf19596bdcd34`

```c

void * vm_getActionRoutine(int no)

{
  void *pvVar1;
  
  pvVar1 = (void *)0x0;
  if ((uint)no < 0x55) {
    pvVar1 = instructions[no];
  }
  return pvVar1;
}


```

<details><summary>汇编</summary>

```asm
0x8070cb0: PUSH EBP
0x8070cb1: MOV EBP, ESP
0x8070cb3: MOV EDX, dword ptr [EBP + 0x8]
0x8070cb6: XOR EAX, EAX
0x8070cb8: CMP EDX, 0x54
0x8070cbb: JA 0x08070cc4
0x8070cbd: MOV EAX, dword ptr [EDX*0x4 + 0x82bcb40]
0x8070cc4: LEAVE
0x8070cc5: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadComment"></a>

## vm_loadComment @ 0x8085d1c

Evidence: `ev_1cd93e419236808566a5cd130589effdf3d176326a93a4fb21b3f76ddf320ded`

```c

/* WARNING: Unknown calling convention */

int vm_loadComment(int fd,char *comment)

{
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8085d1c: PUSH EBP
0x8085d1d: MOV EBP, ESP
0x8085d1f: MOV EAX, 0x1
0x8085d24: LEAVE
0x8085d25: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadIdentifiers"></a>

## vm_loadIdentifiers @ 0x8063724

Evidence: `ev_210d97aaad70cc5d4a82d10efa6e68903ee178fdc854945609f9577404cb97a6`

```c

int vm_loadIdentifiers(int fd)

{
  short *psVar1;
  void *pvVar2;
  int iVar3;
  Vm_Program *pVVar4;
  char *pcVar5;
  Vm_Prog_Var_T **ppVVar6;
  int iVar7;
  Vm_Ident_Hash_Elem_T *ihe;
  bool bVar8;
  uint uVar9;
  Vm_Program_T *pProgram;
  Vm_Prog_Var_T **ppVar;
  int i;
  Vm_Fun_T *pFun;
  Vm_Dlt_T *pDlt;
  int functions;
  int count_1;
  int index;
  Vm_Prog_Var_T *pVar;
  int variables;
  char *pNames;
  int nameSize;
  int count;
  
  iVar3 = vm_loadComment(fd,"IDENTIFIERS");
  if (iVar3 == 0) {
    return 0;
  }
  iVar3 = vm_loadComment(fd,"NAMES");
  if (iVar3 == 0) {
    return 0;
  }
  iVar3 = vm_loadNumber32(fd,&count);
  if ((((iVar3 != 0) && (-1 < count)) && (iVar3 = vm_loadNumber32(fd,&nameSize), iVar3 != 0)) &&
     ((-1 < nameSize && (iVar3 = vm_loadBuffer(fd,&pNames,nameSize), iVar3 != 0)))) {
    iVar3 = vm_loadProgram(fd);
    if (iVar3 == 0) {
      return 0;
    }
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg7(EDI)] */
    pVVar4 = vm_compilingProgram();
    iVar3 = vm_loadComment(fd,"VARIABLES");
    if (((iVar3 != 0) && (iVar3 = vm_loadNumber32(fd,&variables), iVar3 != 0)) && (-1 < variables))
    {
      if (pVVar4 == pGlobalProgram) {
        if (variables < globalVar) {
          return 0;
        }
        globalVar = variables;
      }
      else {
                    /* Unresolved local var: int size@[???] */
        pVVar4->variable = variables;
        iVar3 = vm_loadComment(fd,"PROG_VARS");
        if (iVar3 == 0) {
          return 0;
        }
        ppVVar6 = &pVVar4->pProgVars;
        while (iVar3 = variables + -1, bVar8 = 0 < variables, variables = iVar3, bVar8) {
          iVar3 = vm_loadBuffer2(fd,&pVar,0x10);
          if (iVar3 == 0) {
            return 0;
          }
          iVar3 = vm_loadString2(fd,&pVar->pVarName);
          if (iVar3 == 0) {
            uVar9 = 0x197b;
LAB_080638f3:
            pdb_freeMemory(pVar,"VM","vm_lex.c",uVar9);
            return 0;
          }
          iVar3 = vm_loadNumber32(fd,&index);
          if (((iVar3 == 0) || (index < 0)) || (pVVar4->inheritProgram < index)) {
            pdb_freeMemory(pVar->pVarName,"VM","vm_lex.c",0x1984);
            uVar9 = 0x1985;
            goto LAB_080638f3;
          }
          if (index == 0) {
            pcVar5 = pVVar4->pName;
          }
          else {
            pcVar5 = pVVar4->ppInheritProgram[index + -1];
          }
          pVar->pNext = (Vm_Prog_Var *)0x0;
          *ppVVar6 = pVar;
          pVar->pProgName = pcVar5;
          ppVVar6 = &pVar->pNext;
        }
      }
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg7(EDI)] */
      pVVar4 = vm_compilingProgram();
      iVar3 = vm_loadComment(fd,"FUNCTIONS");
      if (((iVar3 != 0) && (iVar3 = vm_loadNumber32(fd,&count_1), iVar3 != 0)) &&
         ((efunsCount == count_1 &&
          ((iVar3 = vm_loadNumber32(fd,&functions), iVar3 != 0 && (-1 < functions)))))) {
        pVVar4->function = functions;
        iVar3 = vm_extendFunctionTable(pVVar4);
        if (iVar3 != 0) {
          iVar3 = vm_loadComment(fd,"DLT");
          if (iVar3 == 0) {
            return 0;
          }
          iVar3 = vm_loadNumber32(fd,&count);
          i = count;
          if (iVar3 == 0) {
            return 0;
          }
          while( true ) {
            i = i + -1;
            if (i < 0) {
              vm_scratchFree(pNames);
              return 1;
            }
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg6(ESI)] */
            pVVar4 = vm_compilingProgram();
            iVar3 = nameSize;
            pcVar5 = pNames;
            iVar7 = vm_loadBuffer(fd,&pDlt,0x14);
            if ((iVar7 == 0) || ((uint)iVar3 <= pDlt->name)) break;
            pDlt->name = (uint)(pcVar5 + pDlt->name);
            uVar9 = pDlt->attrib;
            if ((uVar9 & 0x80000) == 0) {
              if ((uVar9 & 0x40000) != 0) {
                iVar3 = (pDlt->v).funNo;
                if (iVar3 < 0) {
                  return 0;
                }
                bVar8 = SBORROW4(iVar3,pVVar4->variable);
                iVar3 = iVar3 - pVVar4->variable;
                goto LAB_08063a1c;
              }
              if ((uVar9 & 0x8000000) == 0) {
                return 0;
              }
              if ((pDlt->v).funNo < 0) {
                return 0;
              }
              if (pVVar4->function <= (pDlt->v).funNo) {
                return 0;
              }
              iVar3 = vm_loadFun(fd,&pFun);
              if (iVar3 == 0) {
                return 0;
              }
              (pDlt->v).pFun = pFun;
            }
            else {
              iVar3 = (pDlt->v).funNo;
              if (iVar3 < 0) {
                return 0;
              }
              bVar8 = SBORROW4(iVar3,globalVar);
              iVar3 = iVar3 - globalVar;
LAB_08063a1c:
              if (bVar8 == iVar3 < 0) {
                return 0;
              }
            }
            ihe = vm_findOrAddIdent((char *)pDlt->name);
            if ((ihe->token & 0xc0080000) != 0) {
              return 0;
            }
            if (((ihe->token & 0x8000000) != 0) && ((pDlt->attrib & 0x8000000) == 0)) {
              return 0;
            }
            uVar9 = 0;
            if (ihe->semValue != 0) {
              iVar3 = vm_backupIi(ihe,(void *)0x0);
              if (iVar3 == 0) {
                return 0;
              }
              uVar9 = ihe->semValue;
            }
            ihe->semValue = uVar9 + 1;
            (ihe->ii).pInContext = (void *)0x0;
            pvVar2 = (pDlt->v).pInfo;
            *(undefined4 *)((int)&(ihe->ii).v + 4) = *(undefined4 *)((int)&pDlt->v + 4);
            (ihe->ii).v.pInfo = pvVar2;
            uVar9 = pDlt->attrib | 0x20000000;
            psVar1 = &(ihe->dn).globalNum;
            *psVar1 = *psVar1 + 1;
            (ihe->ii).oldToken = uVar9;
            ihe->token = uVar9;
          }
        }
      }
    }
  }
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x8063724: PUSH EBP
0x8063725: MOV EBP, ESP
0x8063727: PUSH EDI
0x8063728: PUSH ESI
0x8063729: PUSH EBX
0x806372a: SUB ESP, 0x44
0x806372d: PUSH 0x81c7a6c
0x8063732: MOV EBX, dword ptr [EBP + 0x8]
0x8063735: PUSH EBX
0x8063736: CALL 0x08085d1c
0x806373b: ADD ESP, 0x10
0x806373e: XOR EDX, EDX
0x8063740: TEST EAX, EAX
0x8063742: JNZ 0x08063750
0x8063744: LEA ESP, [EBP + -0xc]
0x8063747: POP EBX
0x8063748: POP ESI
0x8063749: MOV EAX, EDX
0x806374b: POP EDI
0x806374c: LEAVE
0x806374d: RET
0x8063750: SUB ESP, 0x8
0x8063753: PUSH 0x81c7a78
0x8063758: PUSH EBX
0x8063759: CALL 0x08085d1c
0x806375e: ADD ESP, 0x10
0x8063761: XOR EDX, EDX
0x8063763: TEST EAX, EAX
0x8063765: JZ 0x08063744
0x8063767: SUB ESP, 0x8
0x806376a: LEA ESI, [EBP + -0x10]
0x806376d: PUSH ESI
0x806376e: PUSH EBX
0x806376f: CALL 0x08085d28
0x8063774: ADD ESP, 0x10
0x8063777: TEST EAX, EAX
0x8063779: JZ 0x08063908
0x806377f: MOV ECX, dword ptr [EBP + -0x10]
0x8063782: TEST ECX, ECX
0x8063784: JS 0x08063908
0x806378a: SUB ESP, 0x8
0x806378d: LEA EAX, [EBP + -0x14]
0x8063790: PUSH EAX
0x8063791: PUSH EBX
0x8063792: CALL 0x08085d28
0x8063797: ADD ESP, 0x10
0x806379a: TEST EAX, EAX
0x806379c: JZ 0x08063908
0x80637a2: MOV EAX, dword ptr [EBP + -0x14]
0x80637a5: TEST EAX, EAX
0x80637a7: JS 0x08063908
0x80637ad: PUSH EDX
0x80637ae: PUSH EAX
0x80637af: LEA EAX, [EBP + -0x18]
0x80637b2: PUSH EAX
0x80637b3: PUSH EBX
0x80637b4: CALL 0x08085ed8
0x80637b9: ADD ESP, 0x10
0x80637bc: TEST EAX, EAX
0x80637be: JZ 0x08063908
0x80637c4: SUB ESP, 0xc
0x80637c7: PUSH EBX
0x80637c8: CALL 0x0806351c
0x80637cd: ADD ESP, 0x10
0x80637d0: XOR EDX, EDX
0x80637d2: TEST EAX, EAX
0x80637d4: JZ 0x08063744
0x80637da: CALL 0x0805d668
0x80637df: SUB ESP, 0x8
0x80637e2: PUSH 0x81c7a7e
0x80637e7: PUSH EBX
0x80637e8: MOV EDI, EAX
0x80637ea: CALL 0x08085d1c
0x80637ef: ADD ESP, 0x10
0x80637f2: TEST EAX, EAX
0x80637f4: JZ 0x08063908
0x80637fa: SUB ESP, 0x8
0x80637fd: LEA EAX, [EBP + -0x1c]
0x8063800: PUSH EAX
0x8063801: PUSH EBX
0x8063802: CALL 0x08085d28
0x8063807: ADD ESP, 0x10
0x806380a: TEST EAX, EAX
0x806380c: JZ 0x08063908
0x8063812: MOV EAX, dword ptr [EBP + -0x1c]
0x8063815: TEST EAX, EAX
0x8063817: JS 0x08063908
0x806381d: CMP EDI, dword ptr [0x082c46b0]
0x8063823: JZ 0x08063916
0x8063829: SUB ESP, 0x8
0x806382c: MOV dword ptr [EDI + 0x8], EAX
0x806382f: PUSH 0x81c7a88
0x8063834: PUSH EBX
0x8063835: CALL 0x08085d1c
0x806383a: ADD ESP, 0x10
0x806383d: TEST EAX, EAX
0x806383f: JZ 0x08063908
0x8063845: LEA EAX, [EDI + 0x34]
0x8063848: JMP 0x08063867
0x806384c: MOV EDX, dword ptr [EBP + -0x20]
0x806384f: MOV EAX, dword ptr [EDI + 0x4]
0x8063852: MOV ECX, dword ptr [EBP + -0x3c]
0x8063855: MOV dword ptr [EDX + 0xc], 0x0
0x806385c: MOV dword ptr [ECX], EDX
0x806385e: MOV dword ptr [EDX + 0x4], EAX
0x8063861: MOV EAX, dword ptr [EBP + -0x20]
0x8063864: ADD EAX, 0xc
0x8063867: MOV EDX, dword ptr [EBP + -0x1c]
0x806386a: MOV dword ptr [EBP + -0x3c], EAX
0x806386d: TEST EDX, EDX
0x806386f: LEA EAX, [EDX + -0x1]
0x8063872: MOV dword ptr [EBP + -0x1c], EAX
0x8063875: JLE 0x08063923
0x806387b: PUSH EAX
0x806387c: PUSH 0x10
0x806387e: LEA EDX, [EBP + -0x20]
0x8063881: PUSH EDX
0x8063882: PUSH EBX
0x8063883: CALL 0x08085fb0
0x8063888: ADD ESP, 0x10
0x806388b: TEST EAX, EAX
0x806388d: JZ 0x08063908
0x806388f: SUB ESP, 0x8
0x8063892: PUSH dword ptr [EBP + -0x20]
0x8063895: PUSH EBX
0x8063896: CALL 0x08086044
0x806389b: ADD ESP, 0x10
0x806389e: TEST EAX, EAX
0x80638a0: JZ 0x0806390f
0x80638a2: SUB ESP, 0x8
0x80638a5: LEA EAX, [EBP + -0x24]
0x80638a8: PUSH EAX
0x80638a9: PUSH EBX
0x80638aa: CALL 0x08085d28
0x80638af: ADD ESP, 0x10
0x80638b2: TEST EAX, EAX
0x80638b4: JZ 0x080638d2
0x80638b6: MOV ECX, dword ptr [EBP + -0x24]
0x80638b9: TEST ECX, ECX
0x80638bb: JS 0x080638d2
0x80638bd: CMP dword ptr [EDI + 0x10], ECX
0x80638c0: JL 0x080638d2
0x80638c2: TEST ECX, ECX
0x80638c4: JZ 0x0806384c
0x80638c6: MOV EAX, dword ptr [EDI + 0x14]
0x80638c9: MOV EDX, dword ptr [EBP + -0x20]
0x80638cc: MOV EAX, dword ptr [EAX + ECX*0x4 + -0x4]
0x80638d0: JMP 0x08063852
0x80638d2: PUSH 0x1984
0x80638d7: PUSH 0x81c74e8
0x80638dc: PUSH 0x81c34ab
0x80638e1: MOV EAX, dword ptr [EBP + -0x20]
0x80638e4: PUSH dword ptr [EAX]
0x80638e6: CALL 0x080534f4
0x80638eb: ADD ESP, 0x10
0x80638ee: PUSH 0x1985
0x80638f3: PUSH 0x81c74e8
0x80638f8: PUSH 0x81c34ab
0x80638fd: PUSH dword ptr [EBP + -0x20]
0x8063900: CALL 0x080534f4
0x8063905: ADD ESP, 0x10
0x8063908: XOR EDX, EDX
0x806390a: JMP 0x08063744
0x806390f: PUSH 0x197b
0x8063914: JMP 0x080638f3
0x8063916: CMP dword ptr [0x082c46a4], EAX
0x806391c: JG 0x08063908
0x806391e: MOV [0x082c46a4], EAX
0x8063923: CALL 0x0805d668
0x8063928: SUB ESP, 0x8
0x806392b: PUSH 0x81c7a92
0x8063930: PUSH EBX
0x8063931: MOV EDI, EAX
0x8063933: CALL 0x08085d1c
0x8063938: ADD ESP, 0x10
0x806393b: TEST EAX, EAX
0x806393d: JZ 0x08063908
0x806393f: SUB ESP, 0x8
0x8063942: LEA EAX, [EBP + -0x28]
0x8063945: PUSH EAX
0x8063946: PUSH EBX
0x8063947: CALL 0x08085d28
0x806394c: ADD ESP, 0x10
0x806394f: TEST EAX, EAX
0x8063951: JZ 0x08063908
0x8063953: MOV EAX, dword ptr [EBP + -0x28]
0x8063956: CMP dword ptr [0x082c48c0], EAX
0x806395c: JNZ 0x08063908
0x806395e: SUB ESP, 0x8
0x8063961: LEA EAX, [EBP + -0x2c]
0x8063964: PUSH EAX
0x8063965: PUSH EBX
0x8063966: CALL 0x08085d28
0x806396b: ADD ESP, 0x10
0x806396e: TEST EAX, EAX
0x8063970: JZ 0x08063908
0x8063972: MOV EAX, dword ptr [EBP + -0x2c]
0x8063975: TEST EAX, EAX
0x8063977: JS 0x08063908
0x8063979: MOV dword ptr [EDI + 0xc], EAX
0x806397c: MOV EAX, EDI
0x806397e: CALL 0x0805e70c
0x8063983: TEST EAX, EAX
0x8063985: JZ 0x08063908
0x8063987: SUB ESP, 0x8
0x806398a: PUSH 0x81c7a9c
0x806398f: PUSH EBX
0x8063990: CALL 0x08085d1c
0x8063995: ADD ESP, 0x10
0x8063998: XOR EDX, EDX
0x806399a: TEST EAX, EAX
0x806399c: JZ 0x08063744
0x80639a2: SUB ESP, 0x8
0x80639a5: PUSH ESI
0x80639a6: PUSH EBX
0x80639a7: CALL 0x08085d28
0x80639ac: ADD ESP, 0x10
0x80639af: XOR EDX, EDX
0x80639b1: TEST EAX, EAX
0x80639b3: JZ 0x08063744
0x80639b9: MOV EDX, dword ptr [EBP + -0x10]
0x80639bc: DEC EDX
0x80639bd: MOV dword ptr [EBP + -0x38], EDX
0x80639c0: JS 0x08063b0e
0x80639c6: CALL 0x0805d668
0x80639cb: MOV dword ptr [EBP + -0x40], EAX
0x80639ce: PUSH EAX
0x80639cf: PUSH 0x14
0x80639d1: LEA ECX, [EBP + -0x30]
0x80639d4: PUSH ECX
0x80639d5: PUSH EBX
0x80639d6: MOV ESI, dword ptr [EBP + -0x18]
0x80639d9: MOV EDI, dword ptr [EBP + -0x14]
0x80639dc: CALL 0x08085ed8
0x80639e1: ADD ESP, 0x10
0x80639e4: TEST EAX, EAX
0x80639e6: JZ 0x08063908
0x80639ec: MOV EDX, dword ptr [EBP + -0x30]
0x80639ef: MOV EAX, dword ptr [EDX]
0x80639f1: CMP EAX, EDI
0x80639f3: JNC 0x08063908
0x80639f9: ADD EAX, ESI
0x80639fb: MOV dword ptr [EDX], EAX
0x80639fd: MOV EAX, dword ptr [EDX + 0x4]
0x8063a00: TEST EAX, 0x80000
0x8063a05: JZ 0x08063a8e
0x8063a0b: MOV EAX, dword ptr [EDX + 0xc]
0x8063a0e: TEST EAX, EAX
0x8063a10: JS 0x08063908
0x8063a16: CMP EAX, dword ptr [0x082c46a4]
0x8063a1c: JGE 0x08063908
0x8063a22: SUB ESP, 0xc
0x8063a25: PUSH dword ptr [EDX]
0x8063a27: CALL 0x080627e4
0x8063a2c: MOV ESI, EAX
0x8063a2e: MOV EAX, dword ptr [EAX + 0x4]
0x8063a31: ADD ESP, 0x10
0x8063a34: TEST EAX, 0xc0080000
0x8063a39: JNZ 0x08063908
0x8063a3f: TEST EAX, 0x8000000
0x8063a44: JZ 0x08063a53
0x8063a46: MOV EAX, dword ptr [EBP + -0x30]
0x8063a49: TEST byte ptr [EAX + 0x7], 0x8
0x8063a4d: JZ 0x08063908
0x8063a53: MOV EAX, dword ptr [ESI + 0x8]
0x8063a56: TEST EAX, EAX
0x8063a58: JNZ 0x08063aab
0x8063a5a: INC EAX
0x8063a5b: MOV ECX, dword ptr [EBP + -0x30]
0x8063a5e: MOV dword ptr [ESI + 0x8], EAX
0x8063a61: MOV dword ptr [ESI + 0x1c], 0x0
0x8063a68: MOV EAX, dword ptr [ECX + 0xc]
0x8063a6b: MOV EDX, dword ptr [ECX + 0x10]
0x8063a6e: MOV dword ptr [ESI + 0x18], EDX
0x8063a71: MOV dword ptr [ESI + 0x14], EAX
0x8063a74: MOV EAX, dword ptr [ECX + 0x4]
0x8063a77: OR EAX, 0x20000000
0x8063a7c: INC word ptr [ESI + 0x2e]
0x8063a80: DEC dword ptr [EBP + -0x38]
0x8063a83: MOV dword ptr [ESI + 0x20], EAX
0x8063a86: MOV dword ptr [ESI + 0x4], EAX
0x8063a89: JMP 0x080639c0
0x8063a8e: TEST EAX, 0x40000
0x8063a93: JZ 0x08063ac6
0x8063a95: MOV EAX, dword ptr [EDX + 0xc]
0x8063a98: TEST EAX, EAX
0x8063a9a: JS 0x08063908
0x8063aa0: MOV ECX, dword ptr [EBP + -0x40]
0x8063aa3: CMP EAX, dword ptr [ECX + 0x8]
0x8063aa6: JMP 0x08063a1c
0x8063aab: SUB ESP, 0x8
0x8063aae: PUSH 0x0
0x8063ab0: PUSH ESI
0x8063ab1: CALL 0x08062504
0x8063ab6: ADD ESP, 0x10
0x8063ab9: TEST EAX, EAX
0x8063abb: JZ 0x08063908
0x8063ac1: MOV EAX, dword ptr [ESI + 0x8]
0x8063ac4: JMP 0x08063a5a
0x8063ac6: TEST EAX, 0x8000000
0x8063acb: JZ 0x08063908
0x8063ad1: MOV EAX, dword ptr [EDX + 0xc]
0x8063ad4: TEST EAX, EAX
0x8063ad6: JS 0x08063908
0x8063adc: MOV EDX, dword ptr [EBP + -0x40]
0x8063adf: CMP EAX, dword ptr [EDX + 0xc]
0x8063ae2: JGE 0x08063908
0x8063ae8: SUB ESP, 0x8
0x8063aeb: LEA EAX, [EBP + -0x34]
0x8063aee: PUSH EAX
0x8063aef: PUSH EBX
0x8063af0: CALL 0x0806fb0c
0x8063af5: ADD ESP, 0x10
0x8063af8: TEST EAX, EAX
0x8063afa: JZ 0x08063908
0x8063b00: MOV EDX, dword ptr [EBP + -0x30]
0x8063b03: MOV EAX, dword ptr [EBP + -0x34]
0x8063b06: MOV dword ptr [EDX + 0xc], EAX
0x8063b09: JMP 0x08063a22
0x8063b0e: SUB ESP, 0xc
0x8063b11: PUSH dword ptr [EBP + -0x18]
0x8063b14: CALL 0x08064bcc
0x8063b19: MOV EDX, 0x1
0x8063b1e: JMP 0x08063744
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadFun"></a>

## vm_loadFun @ 0x806fb0c

Evidence: `ev_2b4a05effbcc877a1f94739feafa012dddb87cbec4758220c4436aad6c8dd603`

```c

int vm_loadFun(int fd,Vm_Fun_T **ppFun)

{
  Vm_Fun_Arg_T *pVVar1;
  Vm_Fun_T *pVVar2;
  int iVar3;
  char *pcVar4;
  Vm_Fun_Arg_T **ppVVar5;
  int n;
  Vm_Fun_T *pFun;
  
                    /* Unresolved local var: Vm_Fun_Arg_T * * ppArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * * ppVar@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pVar@[DW_OP_reg3(EBX)] */
  iVar3 = vm_loadBuffer(fd,&pFun,0x2c);
  if (iVar3 == 0) {
    return 0;
  }
  pFun->pArgs = (Vm_Fun_Arg_T *)0x0;
  pFun->pLabels = (Vm_Local_Label_T *)0x0;
  pFun->pNext = (Vm_Fun *)0x0;
  pFun->pNodes = (Vm_Node_T *)0x0;
  pFun->pVars = (Vm_Fun_Arg_T *)0x0;
  iVar3 = vm_isProbeDisabled();
  if (iVar3 == 0) {
    iVar3 = vm_loadString(fd,&pFun->name);
    if (iVar3 == 0) {
      return 0;
    }
  }
  else {
    iVar3 = vm_loadString(fd,(char **)0x0);
    pVVar2 = pFun;
    if (iVar3 == 0) {
      return 0;
    }
    pcVar4 = vm_scratchLargeAlloc(4);
    pVVar2->name = pcVar4;
    *pFun->name = '?';
    pFun->name[1] = '?';
    pFun->name[2] = '?';
    pFun->name[3] = '\0';
  }
  iVar3 = vm_loadComment(fd,"ARGUMENTS");
  if (iVar3 != 0) {
    iVar3 = vm_loadNumber32(fd,&n);
    if (iVar3 == 0) {
      return 0;
    }
    ppVVar5 = &pFun->pArgs;
    while( true ) {
      if (n < 1) {
        n = n + -1;
        iVar3 = vm_loadNumber32(fd,&n);
        if (iVar3 == 0) {
          return 0;
        }
        ppVVar5 = &pFun->pVars;
        while( true ) {
          if (n < 1) {
            n = n + -1;
            iVar3 = vm_isProbeDisabled();
            if (iVar3 != 0) {
              while (pVVar1 = pFun->pVars, pVVar1 != (Vm_Fun_Arg_T *)0x0) {
                pFun->pVars = pVVar1->pNext;
                vm_scratchFree(pVVar1->name);
                vm_scratchFree(pVVar1);
              }
            }
            iVar3 = vm_loadComment(fd,"ACTIONS");
            if (iVar3 != 0) {
              iVar3 = vm_loadActions(fd,pFun,&pFun->pNodes);
              if (iVar3 != 0) {
                *ppFun = pFun;
                return 1;
              }
              return 0;
            }
            return 0;
          }
          n = n + -1;
          iVar3 = vm_loadBuffer(fd,ppVVar5,0x18);
          if (iVar3 == 0) {
            return 0;
          }
          pVVar1 = *ppVVar5;
          pVVar1->pDefaultValue = (Vm_XNode *)0x0;
          pVVar1->pNext = (Vm_Fun_Arg *)0x0;
          iVar3 = vm_loadString(fd,&pVVar1->name);
          if (iVar3 == 0) break;
          ppVVar5 = &pVVar1->pNext;
        }
        return 0;
      }
      n = n + -1;
      iVar3 = vm_loadBuffer(fd,ppVVar5,0x18);
      if (iVar3 == 0) {
        return 0;
      }
      pVVar1 = *ppVVar5;
      pVVar1->pDefaultValue = (Vm_XNode *)0x0;
      pVVar1->pNext = (Vm_Fun_Arg *)0x0;
      iVar3 = vm_loadString(fd,&pVVar1->name);
      if (iVar3 == 0) break;
      ppVVar5 = &pVVar1->pNext;
    }
    return 0;
  }
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x806fb0c: PUSH EBP
0x806fb0d: MOV EBP, ESP
0x806fb0f: PUSH EDI
0x806fb10: PUSH ESI
0x806fb11: PUSH EBX
0x806fb12: SUB ESP, 0x10
0x806fb15: PUSH 0x2c
0x806fb17: LEA EAX, [EBP + -0x10]
0x806fb1a: PUSH EAX
0x806fb1b: MOV ESI, dword ptr [EBP + 0x8]
0x806fb1e: PUSH ESI
0x806fb1f: CALL 0x08085ed8
0x806fb24: ADD ESP, 0x10
0x806fb27: XOR EDX, EDX
0x806fb29: TEST EAX, EAX
0x806fb2b: JNZ 0x0806fb38
0x806fb2d: LEA ESP, [EBP + -0xc]
0x806fb30: POP EBX
0x806fb31: POP ESI
0x806fb32: MOV EAX, EDX
0x806fb34: POP EDI
0x806fb35: LEAVE
0x806fb36: RET
0x806fb38: MOV EAX, dword ptr [EBP + -0x10]
0x806fb3b: MOV dword ptr [EAX + 0x1c], 0x0
0x806fb42: MOV dword ptr [EAX + 0x24], 0x0
0x806fb49: MOV dword ptr [EAX + 0x28], 0x0
0x806fb50: MOV dword ptr [EAX + 0x18], 0x0
0x806fb57: MOV dword ptr [EAX + 0x20], 0x0
0x806fb5e: CALL 0x0804c2d4
0x806fb63: TEST EAX, EAX
0x806fb65: JNZ 0x0806fce4
0x806fb6b: SUB ESP, 0x8
0x806fb6e: PUSH dword ptr [EBP + -0x10]
0x806fb71: PUSH ESI
0x806fb72: CALL 0x08085f40
0x806fb77: ADD ESP, 0x10
0x806fb7a: XOR EDX, EDX
0x806fb7c: TEST EAX, EAX
0x806fb7e: JZ 0x0806fb2d
0x806fb80: SUB ESP, 0x8
0x806fb83: PUSH 0x81ca956
0x806fb88: PUSH ESI
0x806fb89: CALL 0x08085d1c
0x806fb8e: ADD ESP, 0x10
0x806fb91: XOR EDX, EDX
0x806fb93: TEST EAX, EAX
0x806fb95: JZ 0x0806fb2d
0x806fb97: SUB ESP, 0x8
0x806fb9a: LEA EDI, [EBP + -0x14]
0x806fb9d: PUSH EDI
0x806fb9e: PUSH ESI
0x806fb9f: CALL 0x08085d28
0x806fba4: ADD ESP, 0x10
0x806fba7: XOR EDX, EDX
0x806fba9: TEST EAX, EAX
0x806fbab: JZ 0x0806fb2d
0x806fbad: MOV EBX, dword ptr [EBP + -0x10]
0x806fbb0: ADD EBX, 0x1c
0x806fbb3: JMP 0x0806fbf2
0x806fbb5: PUSH EAX
0x806fbb6: PUSH 0x18
0x806fbb8: PUSH EBX
0x806fbb9: PUSH ESI
0x806fbba: CALL 0x08085ed8
0x806fbbf: ADD ESP, 0x10
0x806fbc2: TEST EAX, EAX
0x806fbc4: JZ 0x0806fd36
0x806fbca: MOV EBX, dword ptr [EBX]
0x806fbcc: SUB ESP, 0x8
0x806fbcf: MOV dword ptr [EBX + 0x10], 0x0
0x806fbd6: MOV dword ptr [EBX + 0x14], 0x0
0x806fbdd: PUSH EBX
0x806fbde: PUSH ESI
0x806fbdf: CALL 0x08085f40
0x806fbe4: ADD ESP, 0x10
0x806fbe7: TEST EAX, EAX
0x806fbe9: JZ 0x0806fd36
0x806fbef: ADD EBX, 0x14
0x806fbf2: MOV EDX, dword ptr [EBP + -0x14]
0x806fbf5: LEA EAX, [EDX + -0x1]
0x806fbf8: TEST EDX, EDX
0x806fbfa: MOV dword ptr [EBP + -0x14], EAX
0x806fbfd: JG 0x0806fbb5
0x806fbff: SUB ESP, 0x8
0x806fc02: PUSH EDI
0x806fc03: PUSH ESI
0x806fc04: CALL 0x08085d28
0x806fc09: ADD ESP, 0x10
0x806fc0c: XOR EDX, EDX
0x806fc0e: TEST EAX, EAX
0x806fc10: JZ 0x0806fb2d
0x806fc16: MOV EBX, dword ptr [EBP + -0x10]
0x806fc19: ADD EBX, 0x20
0x806fc1c: JMP 0x0806fc5b
0x806fc1e: PUSH ECX
0x806fc1f: PUSH 0x18
0x806fc21: PUSH EBX
0x806fc22: PUSH ESI
0x806fc23: CALL 0x08085ed8
0x806fc28: ADD ESP, 0x10
0x806fc2b: TEST EAX, EAX
0x806fc2d: JZ 0x0806fd36
0x806fc33: MOV EBX, dword ptr [EBX]
0x806fc35: SUB ESP, 0x8
0x806fc38: MOV dword ptr [EBX + 0x10], 0x0
0x806fc3f: MOV dword ptr [EBX + 0x14], 0x0
0x806fc46: PUSH EBX
0x806fc47: PUSH ESI
0x806fc48: CALL 0x08085f40
0x806fc4d: ADD ESP, 0x10
0x806fc50: TEST EAX, EAX
0x806fc52: JZ 0x0806fd36
0x806fc58: ADD EBX, 0x14
0x806fc5b: MOV EDX, dword ptr [EBP + -0x14]
0x806fc5e: LEA EAX, [EDX + -0x1]
0x806fc61: TEST EDX, EDX
0x806fc63: MOV dword ptr [EBP + -0x14], EAX
0x806fc66: JG 0x0806fc1e
0x806fc68: CALL 0x0804c2d4
0x806fc6d: TEST EAX, EAX
0x806fc6f: JZ 0x0806fc98
0x806fc71: JMP 0x0806fc8e
0x806fc73: SUB ESP, 0xc
0x806fc76: MOV EAX, dword ptr [EBX + 0x14]
0x806fc79: MOV dword ptr [EDX + 0x20], EAX
0x806fc7c: PUSH dword ptr [EBX]
0x806fc7e: CALL 0x08064bcc
0x806fc83: MOV dword ptr [ESP], EBX
0x806fc86: CALL 0x08064bcc
0x806fc8b: ADD ESP, 0x10
0x806fc8e: MOV EDX, dword ptr [EBP + -0x10]
0x806fc91: MOV EBX, dword ptr [EDX + 0x20]
0x806fc94: TEST EBX, EBX
0x806fc96: JNZ 0x0806fc73
0x806fc98: SUB ESP, 0x8
0x806fc9b: PUSH 0x81ca960
0x806fca0: PUSH ESI
0x806fca1: CALL 0x08085d1c
0x806fca6: ADD ESP, 0x10
0x806fca9: XOR EDX, EDX
0x806fcab: TEST EAX, EAX
0x806fcad: JZ 0x0806fb2d
0x806fcb3: MOV EAX, dword ptr [EBP + -0x10]
0x806fcb6: PUSH EDX
0x806fcb7: LEA EDX, [EAX + 0x18]
0x806fcba: PUSH EDX
0x806fcbb: PUSH EAX
0x806fcbc: PUSH ESI
0x806fcbd: CALL 0x08070cc8
0x806fcc2: ADD ESP, 0x10
0x806fcc5: XOR EDX, EDX
0x806fcc7: TEST EAX, EAX
0x806fcc9: JZ 0x0806fb2d
0x806fccf: MOV EDX, dword ptr [EBP + -0x10]
0x806fcd2: MOV EAX, dword ptr [EBP + 0xc]
0x806fcd5: MOV dword ptr [EAX], EDX
0x806fcd7: MOV EDX, 0x1
0x806fcdc: JMP 0x0806fb2d
0x806fce4: SUB ESP, 0x8
0x806fce7: PUSH 0x0
0x806fce9: PUSH ESI
0x806fcea: CALL 0x08085f40
0x806fcef: ADD ESP, 0x10
0x806fcf2: XOR EDX, EDX
0x806fcf4: TEST EAX, EAX
0x806fcf6: JZ 0x0806fb2d
0x806fcfc: SUB ESP, 0xc
0x806fcff: PUSH 0x4
0x806fd01: MOV EBX, dword ptr [EBP + -0x10]
0x806fd04: CALL 0x08064cd0
0x806fd09: MOV dword ptr [EBX], EAX
0x806fd0b: MOV EAX, dword ptr [EBP + -0x10]
0x806fd0e: MOV EAX, dword ptr [EAX]
0x806fd10: MOV byte ptr [EAX], 0x3f
0x806fd13: MOV EAX, dword ptr [EBP + -0x10]
0x806fd16: MOV EAX, dword ptr [EAX]
0x806fd18: MOV byte ptr [EAX + 0x1], 0x3f
0x806fd1c: MOV EAX, dword ptr [EBP + -0x10]
0x806fd1f: MOV EAX, dword ptr [EAX]
0x806fd21: MOV byte ptr [EAX + 0x2], 0x3f
0x806fd25: MOV EAX, dword ptr [EBP + -0x10]
0x806fd28: MOV EAX, dword ptr [EAX]
0x806fd2a: ADD ESP, 0x10
0x806fd2d: MOV byte ptr [EAX + 0x3], 0x0
0x806fd31: JMP 0x0806fb80
0x806fd36: XOR EDX, EDX
0x806fd38: JMP 0x0806fb2d
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_addEfuns"></a>

## vm_addEfuns @ 0x8062894

Evidence: `ev_3f1940691a0b14c7a0c2260a854889d993623a1df6651823969839f6add4a24b`

```c

void vm_addEfuns(Vm_Efun_T *elist)

{
  int iVar1;
  Vm_Efun_T *pVVar2;
  int iVar3;
  
                    /* Unresolved local var: int i@[???] */
  iVar3 = 0;
  for (pVVar2 = elist; pVVar2->word != (char *)0x0; pVVar2 = pVVar2 + 1) {
    vm_addKeyword(pVVar2->word,(Vm_Ident_Hash_Elem_T *)((int)&elist->word + iVar3));
    pVVar2->semValue = pVVar2->semValue + 1;
    pVVar2->token = 0x40000000;
    iVar1 = vm_getNewFunOffset();
    pVVar2->funOffset = iVar1;
    iVar3 = iVar3 + 0x20;
  }
  return;
}


```

<details><summary>汇编</summary>

```asm
0x8062894: PUSH EBP
0x8062895: MOV EBP, ESP
0x8062897: PUSH EDI
0x8062898: PUSH ESI
0x8062899: PUSH EBX
0x806289a: SUB ESP, 0xc
0x806289d: MOV EDI, dword ptr [EBP + 0x8]
0x80628a0: XOR ESI, ESI
0x80628a2: MOV EBX, EDI
0x80628a4: JMP 0x080628c8
0x80628a8: LEA EDX, [ESI + EDI*0x1]
0x80628ab: CALL 0x08062820
0x80628b0: INC dword ptr [EBX + 0x8]
0x80628b3: MOV dword ptr [EBX + 0x4], 0x40000000
0x80628ba: CALL 0x0805ed28
0x80628bf: MOV dword ptr [EBX + 0x1c], EAX
0x80628c2: ADD ESI, 0x20
0x80628c5: ADD EBX, 0x20
0x80628c8: MOV EAX, dword ptr [EBX]
0x80628ca: TEST EAX, EAX
0x80628cc: JNZ 0x080628a8
0x80628ce: ADD ESP, 0xc
0x80628d1: POP EBX
0x80628d2: POP ESI
0x80628d3: POP EDI
0x80628d4: LEAVE
0x80628d5: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_getNewFunOffset"></a>

## vm_getNewFunOffset @ 0x805ed28

Evidence: `ev_6ae6375b24a1fc03ab1d394843f311a18ae13dc3c3236e72d99e6361f71bf51c`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_getNewFunOffset(void)

{
  int iVar1;
  Vm_Program *pVVar2;
  
  pVVar2 = vm_compilingProgram();
  iVar1 = pVVar2->function;
  pVVar2->function = iVar1 + 1;
  return iVar1;
}


```

<details><summary>汇编</summary>

```asm
0x805ed28: PUSH EBP
0x805ed29: MOV EBP, ESP
0x805ed2b: CALL 0x0805d668
0x805ed30: MOV ECX, dword ptr [EAX + 0xc]
0x805ed33: LEA EDX, [ECX + 0x1]
0x805ed36: MOV dword ptr [EAX + 0xc], EDX
0x805ed39: LEAVE
0x805ed3a: MOV EAX, ECX
0x805ed3c: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="main"></a>

## main @ 0x804b6ec

Evidence: `ev_a3b1e59161cee014e12d15486d70cb5f2b01c1a49f5aa117f0b7361e9ab2e2f0`

```c

/* WARNING: Unknown calling convention */

int main(int argn,char **argv)

{
  int iVar1;
  int iVar2;
  
  vm_registerMysql();
  vm_registerSqlite();
  vm_registerTinyXml();
  vm_registerLanguage();
  vm_registerSubModule("Init GS",initGS,shutdownGS);
  iVar1 = vm_driverStart(argn,argv);
  iVar2 = -1;
  if (iVar1 != 0) {
    vm_driverLoop();
    vm_driverStop();
    iVar2 = vm_getReturnValue();
  }
  return iVar2;
}


```

<details><summary>汇编</summary>

```asm
0x804b6ec: PUSH EBP
0x804b6ed: MOV EBP, ESP
0x804b6ef: SUB ESP, 0x8
0x804b6f2: AND ESP, 0xfffffff0
0x804b6f5: SUB ESP, 0x10
0x804b6f8: CALL 0x080f9d94
0x804b6fd: CALL 0x080bd674
0x804b702: CALL 0x080f2d54
0x804b707: CALL 0x080fc16c
0x804b70c: PUSH EAX
0x804b70d: PUSH 0x804b75c
0x804b712: PUSH 0x804b750
0x804b717: PUSH 0x81c347e
0x804b71c: CALL 0x0804c194
0x804b721: POP EDX
0x804b722: POP ECX
0x804b723: PUSH dword ptr [EBP + 0xc]
0x804b726: PUSH dword ptr [EBP + 0x8]
0x804b729: CALL 0x0804d03c
0x804b72e: ADD ESP, 0x10
0x804b731: TEST EAX, EAX
0x804b733: MOV EDX, 0xffffffff
0x804b738: JZ 0x0804b74b
0x804b73a: CALL 0x0804c934
0x804b73f: CALL 0x0804c968
0x804b744: CALL 0x0804c6f8
0x804b749: MOV EDX, EAX
0x804b74b: MOV EAX, EDX
0x804b74d: LEAVE
0x804b74e: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadBinary"></a>

## vm_loadBinary @ 0x8085d6c

Evidence: `ev_aacec3fe6f99964270027341c1e86f2672c21c41c22466a3701b04eb88b9d984`

```c

int vm_loadBinary(int fd,Vm_Node_T **ppFinalAction)

{
  int iVar1;
  int iVar2;
  int num;
  
                    /* Unresolved local var: int ret@[DW_OP_reg7(EDI)] */
  iVar1 = vm_loadComment(fd,"HEADER");
  iVar2 = 0;
  if (iVar1 != 0) {
    iVar2 = vm_loadNumber32(fd,&num);
    if ((iVar2 == 0) || (num != 0x10001)) {
      return 0;
    }
    iVar1 = vm_loadNumber32(fd,(int *)0x0);
    iVar2 = 0;
    if (iVar1 != 0) {
      iVar2 = vm_loadNumber32(fd,&num);
      if (iVar2 == 0) {
        return 0;
      }
      if (num != functionBC) {
        return 0;
      }
      iVar1 = vm_loadComment(fd,"DEFINES");
      iVar2 = 0;
      if (iVar1 != 0) {
        iVar1 = vm_loadDefines(fd);
        iVar2 = 0;
        if (((iVar1 != 0) && (iVar1 = vm_loadIdentifiers(fd), iVar2 = 0, iVar1 != 0)) &&
           (iVar1 = vm_loadActions(fd,(Vm_Fun_T *)0x0,ppFinalAction), iVar1 != 0)) {
          iVar2 = 1;
        }
      }
    }
  }
  return iVar2;
}


```

<details><summary>汇编</summary>

```asm
0x8085d6c: PUSH EBP
0x8085d6d: MOV EBP, ESP
0x8085d6f: PUSH EDI
0x8085d70: PUSH ESI
0x8085d71: PUSH EBX
0x8085d72: SUB ESP, 0xc
0x8085d75: MOV EBX, dword ptr [EBP + 0x8]
0x8085d78: PUSH 0x81cc906
0x8085d7d: PUSH EBX
0x8085d7e: CALL 0x08085d1c
0x8085d83: POP ESI
0x8085d84: XOR EDI, EDI
0x8085d86: POP EDX
0x8085d87: XOR EDX, EDX
0x8085d89: TEST EAX, EAX
0x8085d8b: JNZ 0x08085d98
0x8085d8d: LEA ESP, [EBP + -0xc]
0x8085d90: POP EBX
0x8085d91: POP ESI
0x8085d92: MOV EAX, EDX
0x8085d94: POP EDI
0x8085d95: LEAVE
0x8085d96: RET
0x8085d98: SUB ESP, 0x8
0x8085d9b: LEA ESI, [EBP + -0x10]
0x8085d9e: PUSH ESI
0x8085d9f: PUSH EBX
0x8085da0: CALL 0x08085d28
0x8085da5: ADD ESP, 0x10
0x8085da8: TEST EAX, EAX
0x8085daa: JZ 0x08085db5
0x8085dac: CMP dword ptr [EBP + -0x10], 0x10001
0x8085db3: JZ 0x08085dc1
0x8085db5: LEA ESP, [EBP + -0xc]
0x8085db8: POP EBX
0x8085db9: POP ESI
0x8085dba: XOR EDX, EDX
0x8085dbc: MOV EAX, EDX
0x8085dbe: POP EDI
0x8085dbf: LEAVE
0x8085dc0: RET
0x8085dc1: SUB ESP, 0x8
0x8085dc4: PUSH 0x0
0x8085dc6: PUSH EBX
0x8085dc7: CALL 0x08085d28
0x8085dcc: ADD ESP, 0x10
0x8085dcf: XOR EDX, EDX
0x8085dd1: TEST EAX, EAX
0x8085dd3: JZ 0x08085d8d
0x8085dd5: SUB ESP, 0x8
0x8085dd8: PUSH ESI
0x8085dd9: PUSH EBX
0x8085dda: CALL 0x08085d28
0x8085ddf: ADD ESP, 0x10
0x8085de2: TEST EAX, EAX
0x8085de4: JZ 0x08085db5
0x8085de6: MOV EAX, [0x08313f60]
0x8085deb: CMP dword ptr [EBP + -0x10], EAX
0x8085dee: JNZ 0x08085db5
0x8085df0: PUSH 0x81cc90d
0x8085df5: PUSH EBX
0x8085df6: CALL 0x08085d1c
0x8085dfb: POP EDX
0x8085dfc: XOR EDX, EDX
0x8085dfe: TEST EAX, EAX
0x8085e00: POP ECX
0x8085e01: JZ 0x08085d8d
0x8085e03: SUB ESP, 0xc
0x8085e06: PUSH EBX
0x8085e07: CALL 0x080632ec
0x8085e0c: ADD ESP, 0x10
0x8085e0f: XOR EDX, EDX
0x8085e11: TEST EAX, EAX
0x8085e13: JZ 0x08085d8d
0x8085e19: SUB ESP, 0xc
0x8085e1c: PUSH EBX
0x8085e1d: CALL 0x08063724
0x8085e22: ADD ESP, 0x10
0x8085e25: TEST EAX, EAX
0x8085e27: JNZ 0x08085e30
0x8085e29: MOV EDX, EDI
0x8085e2b: JMP 0x08085d8d
0x8085e30: PUSH EAX
0x8085e31: PUSH dword ptr [EBP + 0xc]
0x8085e34: PUSH 0x0
0x8085e36: PUSH EBX
0x8085e37: CALL 0x08070cc8
0x8085e3c: ADD ESP, 0x10
0x8085e3f: TEST EAX, EAX
0x8085e41: JZ 0x08085e29
0x8085e43: MOV EDI, 0x1
0x8085e48: MOV EDX, EDI
0x8085e4a: JMP 0x08085d8d
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_doActions"></a>

## vm_doActions @ 0x8071478

Evidence: `ev_ae01b88208771adf11b3e281c0525fa3b653db8f9930eb5619a00ef0bee53da4`

```c

int vm_doActions(Vm_PCB *pPCB,Vm_Tick_T n)

{
  VM_FUN_PTR *pp_Var1;
  uint *puVar2;
  Vm_Catch_Info_T *pVVar3;
  Vm_Node_T *pVVar4;
  int iVar5;
  Vm_Next_Action_Hook_T p_Var6;
  void *pvVar7;
  Vm_Catch_Buf *pVVar8;
  int iVar9;
  Vm_Tick_T VVar10;
  int iVar11;
  uint uVar12;
  Vm_Catch_Info_T *pCatch;
  
                    /* Unresolved local var: int counter@[DW_OP_reg6(ESI)]
                       Unresolved local var: int errCode@[DW_OP_reg3(EBX)]
                       Unresolved local var: int funCp@[DW_OP_reg2(EDX)] */
  vm_resetBackendCatch();
  totalTicks = n;
  lastTicks = _vm_getCurrentTick();
LAB_08071498:
  iVar9 = pPCB->catchCp;
  iVar11 = 0;
  if (iVar9 < 1) goto LAB_080714b6;
  do {
    iVar11 = pPCB->catchContext[iVar9 + -1].funCp;
LAB_080714b6:
    pVVar8 = vm_newCatch(iVar11);
    iVar9 = _setjmp((__jmp_buf_tag *)pVVar8);
    while( true ) {
      if (iVar9 == 0) {
        if ((pPCB->flags & 0x40) != 0) {
          pPCB->flags = pPCB->flags & 0xffffffbf;
          vm_returnFromEfun();
        }
        if (60000 < pPCB->holdTicks) {
          vm_error("Non-delay routine (primitive) cost too much time.\n");
        }
        iVar11 = 0x40;
        switchOutThread = 0;
        if (vm_ip == &_tailNode) goto LAB_080716e7;
        goto LAB_08071602;
      }
      if (pPCB->catchCp < 1) goto LAB_080716ec;
      iVar9 = pPCB->catchCp + -1;
      pPCB->catchCp = iVar9;
      pVVar3 = pPCB->catchContext + iVar9;
      if (vm_sp < pVVar3->sp) {
        vm_popNStack(pVVar3->sp - vm_sp);
      }
      pVVar4 = pVVar3->pCatchIp;
      if (pVVar4 == (Vm_Node_T *)0x0) {
        iVar9 = 0;
        if (0 < pPCB->catchCp) {
          iVar9 = pPCB->catchContext[pPCB->catchCp + -1].funCp;
        }
        pVVar8 = vm_newCatch(iVar9);
        iVar9 = _setjmp((__jmp_buf_tag *)pVVar8);
        if (iVar9 != 0) goto LAB_080716ec;
        (*pVVar3->catchCallback)(pVVar3->pCookie);
        vm_endCatch();
        goto LAB_08071498;
      }
      iVar9 = pPCB->catchCp;
      vm_ip = pVVar4;
      if (0 < iVar9) break;
      pVVar8 = vm_newCatch(0);
      iVar9 = _setjmp((__jmp_buf_tag *)pVVar8);
    }
  } while( true );
  while( true ) {
    if (vm_ip == &_tailNode) goto LAB_080716e7;
    if (switchOutThread != 0) break;
LAB_08071602:
    iVar11 = iVar11 + -1;
    if (((iVar11 < 1) && (iVar11 = 0x40, totalTicks != 0)) &&
       (VVar10 = _vm_getCurrentTick(), totalTicks <= VVar10 - lastTicks)) break;
    p_Var6 = pPCB->nextActionHook;
    if (p_Var6 == (Vm_Next_Action_Hook_T)0x0) {
      pVVar4 = vm_ip + 1;
      iVar5 = vm_ip->parameters;
      currentNode = vm_ip;
      if (iVar5 == 3) {
LAB_08071757:
        uVar12 = vm_ip->par1;
      }
      else {
        if (3 < iVar5) {
          if (iVar5 != 4) {
            if (iVar5 == 0xff) {
              puVar2 = &vm_ip->par1;
              pp_Var1 = &vm_ip->pFunPtr;
              vm_ip = pVVar4;
              (**pp_Var1)(puVar2);
            }
            else {
LAB_080715b4:
              pp_Var1 = &vm_ip->pFunPtr;
              vm_ip = pVVar4;
              (**pp_Var1)();
            }
            goto LAB_080715bc;
          }
          goto LAB_08071757;
        }
        if (iVar5 != 1) {
          if (iVar5 != 2) goto LAB_080715b4;
          goto LAB_08071757;
        }
        uVar12 = vm_ip->par1;
      }
      pp_Var1 = &vm_ip->pFunPtr;
      vm_ip = pVVar4;
      (**pp_Var1)(uVar12);
    }
    else {
                    /* Unresolved local var: Vm_Next_Action_Hook_T _nextActionHook@[???]
                       Unresolved local var: void * _pActionCookie@[DW_OP_reg0(EAX)] */
      pvVar7 = pPCB->pNextActionCookie;
      pPCB->nextActionHook = (Vm_Next_Action_Hook_T)0x0;
      pPCB->pNextActionCookie = (void *)0x0;
      (*p_Var6)(pvVar7);
    }
LAB_080715bc:
    if (vm_ctrlFlags != 0) {
      if ((vm_ctrlFlags & 2) != 0) {
        vm_ctrlFlags = vm_ctrlFlags & 0xfffffffd;
        iVar9 = -0x67;
        break;
      }
      if ((vm_ctrlFlags & 4) != 0) {
        vm_ctrlFlags = vm_ctrlFlags & 0xfffffffb;
        iVar9 = -0x68;
        break;
      }
      if ((vm_ctrlFlags & 1) != 0) {
        pPCB->flags = pPCB->flags | 0x200;
        vm_ctrlFlags = vm_ctrlFlags & 0xfffffffe;
        break;
      }
      if ((vm_ctrlFlags & 8) != 0) {
        vm_ctrlFlags = vm_ctrlFlags & 0xfffffff7;
        iVar9 = -0x66;
        break;
      }
    }
  }
  if ((vm_ip != &_tailNode) && ((pPCB->flags & 0x200000) != 0)) {
    lastTicks = _vm_getCurrentTick();
    vm_error("Non-delay routine (callback) cost too much time.\n");
  }
LAB_080716e7:
  vm_endCatch();
LAB_080716ec:
  vm_freeObjectsInDestructingList();
  return iVar9;
}


```

<details><summary>汇编</summary>

```asm
0x8071478: PUSH EBP
0x8071479: MOV EBP, ESP
0x807147b: PUSH ESI
0x807147c: PUSH EBX
0x807147d: SUB ESP, 0x10
0x8071480: CALL 0x0807dc64
0x8071485: MOV EAX, dword ptr [EBP + 0xc]
0x8071488: MOV [0x082d3cc0], EAX
0x807148d: CALL 0x08078b58
0x8071492: MOV [0x082d3cc4], EAX
0x8071497: NOP
0x8071498: MOV EDX, dword ptr [EBP + 0x8]
0x807149b: MOV EAX, dword ptr [EDX + 0x78]
0x807149e: XOR EDX, EDX
0x80714a0: TEST EAX, EAX
0x80714a2: JLE 0x080714b6
0x80714a4: LEA EAX, [EAX + EAX*0x4]
0x80714a7: SHL EAX, 0x2
0x80714aa: MOV ECX, dword ptr [EBP + 0x8]
0x80714ad: ADD EAX, dword ptr [ECX + 0x80]
0x80714b3: MOV EDX, dword ptr [EAX + -0x10]
0x80714b6: SUB ESP, 0xc
0x80714b9: PUSH EDX
0x80714ba: CALL 0x0807dbe0
0x80714bf: MOV dword ptr [ESP], EAX
0x80714c2: CALL 0x0804b45c
0x80714c7: ADD ESP, 0x10
0x80714ca: TEST EAX, EAX
0x80714cc: MOV EBX, EAX
0x80714ce: JZ 0x0807153a
0x80714d0: MOV EDX, dword ptr [EBP + 0x8]
0x80714d3: MOV EAX, dword ptr [EDX + 0x78]
0x80714d6: TEST EAX, EAX
0x80714d8: JLE 0x080716ec
0x80714de: DEC EAX
0x80714df: MOV dword ptr [EDX + 0x78], EAX
0x80714e2: MOV EDX, dword ptr [EDX + 0x80]
0x80714e8: LEA EAX, [EAX + EAX*0x4]
0x80714eb: LEA EAX, [EDX + EAX*0x4]
0x80714ee: MOV dword ptr [EBP + -0xc], EAX
0x80714f1: MOV EDX, dword ptr [0x08313fa0]
0x80714f7: MOV EAX, dword ptr [EAX + 0x8]
0x80714fa: CMP EAX, EDX
0x80714fc: JG 0x08071698
0x8071502: MOV ECX, dword ptr [EBP + -0xc]
0x8071505: MOV EAX, dword ptr [ECX]
0x8071507: TEST EAX, EAX
0x8071509: JZ 0x08071644
0x807150f: MOV EDX, dword ptr [EBP + 0x8]
0x8071512: MOV [0x08314018], EAX
0x8071517: MOV EAX, dword ptr [EDX + 0x78]
0x807151a: XOR EDX, EDX
0x807151c: TEST EAX, EAX
0x807151e: JG 0x080714a4
0x8071520: SUB ESP, 0xc
0x8071523: PUSH EDX
0x8071524: CALL 0x0807dbe0
0x8071529: MOV dword ptr [ESP], EAX
0x807152c: CALL 0x0804b45c
0x8071531: ADD ESP, 0x10
0x8071534: TEST EAX, EAX
0x8071536: MOV EBX, EAX
0x8071538: JNZ 0x080714d0
0x807153a: MOV ECX, dword ptr [EBP + 0x8]
0x807153d: MOV EAX, dword ptr [ECX + 0x18]
0x8071540: TEST AL, 0x40
0x8071542: JNZ 0x08071774
0x8071548: MOV EAX, dword ptr [EBP + 0x8]
0x807154b: CMP dword ptr [EAX + 0x34], 0xea60
0x8071552: JA 0x0807175f
0x8071558: CMP dword ptr [0x08314018], 0x82bcb00
0x8071562: MOV ESI, 0x40
0x8071567: MOV dword ptr [0x082d3cc8], 0x0
0x8071571: JNZ 0x08071602
0x8071577: JMP 0x080716e7
0x807157c: MOV EDX, dword ptr [0x08314018]
0x8071582: LEA EAX, [EDX + 0x28]
0x8071585: MOV [0x08314018], EAX
0x807158a: MOV EAX, dword ptr [EDX + 0xc]
0x807158d: CMP EAX, 0x3
0x8071590: MOV dword ptr [0x0831401c], EDX
0x8071596: JZ 0x08071750
0x807159c: JG 0x080716fa
0x80715a2: CMP EAX, 0x1
0x80715a5: JZ 0x0807178c
0x80715ab: CMP EAX, 0x2
0x80715ae: JZ 0x08071794
0x80715b4: MOV EAX, [0x0831401c]
0x80715b9: CALL dword ptr [EAX + 0x14]
0x80715bc: MOV EAX, [0x08314014]
0x80715c1: TEST EAX, EAX
0x80715c3: JZ 0x080715e5
0x80715c5: TEST AL, 0x2
0x80715c7: JNZ 0x080717b3
0x80715cd: TEST AL, 0x4
0x80715cf: JNZ 0x080717c5
0x80715d5: TEST AL, 0x1
0x80715d7: JNZ 0x0807179c
0x80715dd: TEST AL, 0x8
0x80715df: JNZ 0x080716ab
0x80715e5: CMP dword ptr [0x08314018], 0x82bcb00
0x80715ef: JZ 0x080716e7
0x80715f5: MOV EAX, [0x082d3cc8]
0x80715fa: TEST EAX, EAX
0x80715fc: JNZ 0x080716b8
0x8071602: DEC ESI
0x8071603: TEST ESI, ESI
0x8071605: JLE 0x08071720
0x807160b: MOV ECX, dword ptr [EBP + 0x8]
0x807160e: MOV EDX, dword ptr [ECX + 0x90]
0x8071614: TEST EDX, EDX
0x8071616: JZ 0x0807157c
0x807161c: MOV EAX, dword ptr [ECX + 0x94]
0x8071622: SUB ESP, 0xc
0x8071625: MOV dword ptr [ECX + 0x90], 0x0
0x807162f: MOV dword ptr [ECX + 0x94], 0x0
0x8071639: PUSH EAX
0x807163a: CALL EDX
0x807163c: ADD ESP, 0x10
0x807163f: JMP 0x080715bc
0x8071644: MOV EDX, dword ptr [EBP + 0x8]
0x8071647: MOV EAX, dword ptr [EDX + 0x78]
0x807164a: XOR EDX, EDX
0x807164c: TEST EAX, EAX
0x807164e: JLE 0x08071662
0x8071650: LEA EAX, [EAX + EAX*0x4]
0x8071653: SHL EAX, 0x2
0x8071656: MOV ECX, dword ptr [EBP + 0x8]
0x8071659: ADD EAX, dword ptr [ECX + 0x80]
0x807165f: MOV EDX, dword ptr [EAX + -0x10]
0x8071662: SUB ESP, 0xc
0x8071665: PUSH EDX
0x8071666: CALL 0x0807dbe0
0x807166b: MOV dword ptr [ESP], EAX
0x807166e: CALL 0x0804b45c
0x8071673: ADD ESP, 0x10
0x8071676: TEST EAX, EAX
0x8071678: MOV EBX, EAX
0x807167a: JNZ 0x080716ec
0x807167c: SUB ESP, 0xc
0x807167f: MOV EAX, dword ptr [EBP + -0xc]
0x8071682: PUSH dword ptr [EAX + 0x10]
0x8071685: CALL dword ptr [EAX + 0xc]
0x8071688: CALL 0x0807dc50
0x807168d: ADD ESP, 0x10
0x8071690: JMP 0x08071498
0x8071698: SUB ESP, 0xc
0x807169b: SUB EAX, EDX
0x807169d: PUSH EAX
0x807169e: CALL 0x0806b4b4
0x80716a3: ADD ESP, 0x10
0x80716a6: JMP 0x08071502
0x80716ab: AND EAX, 0xfffffff7
0x80716ae: MOV [0x08314014], EAX
0x80716b3: MOV EBX, 0xffffff9a
0x80716b8: CMP dword ptr [0x08314018], 0x82bcb00
0x80716c2: JZ 0x080716e7
0x80716c4: MOV EAX, dword ptr [EBP + 0x8]
0x80716c7: TEST byte ptr [EAX + 0x1a], 0x20
0x80716cb: JZ 0x080716e7
0x80716cd: CALL 0x08078b58
0x80716d2: SUB ESP, 0xc
0x80716d5: PUSH 0x81cb154
0x80716da: MOV [0x082d3cc4], EAX
0x80716df: CALL 0x08071c28
0x80716e4: ADD ESP, 0x10
0x80716e7: CALL 0x0807dc50
0x80716ec: CALL 0x08080ae0
0x80716f1: LEA ESP, [EBP + -0x8]
0x80716f4: MOV EAX, EBX
0x80716f6: POP EBX
0x80716f7: POP ESI
0x80716f8: LEAVE
0x80716f9: RET
0x80716fa: CMP EAX, 0x4
0x80716fd: JZ 0x08071784
0x8071703: CMP EAX, 0xff
0x8071708: JNZ 0x080715b4
0x807170e: SUB ESP, 0xc
0x8071711: LEA EAX, [EDX + 0x18]
0x8071714: PUSH EAX
0x8071715: CALL dword ptr [EDX + 0x14]
0x8071718: ADD ESP, 0x10
0x807171b: JMP 0x080715bc
0x8071720: MOV EAX, [0x082d3cc0]
0x8071725: TEST EAX, EAX
0x8071727: MOV ESI, 0x40
0x807172c: JZ 0x0807160b
0x8071732: CALL 0x08078b58
0x8071737: SUB EAX, dword ptr [0x082d3cc4]
0x807173d: CMP EAX, dword ptr [0x082d3cc0]
0x8071743: JC 0x0807160b
0x8071749: JMP 0x080716b8
0x8071750: PUSH EAX
0x8071751: PUSH dword ptr [EDX + 0x20]
0x8071754: PUSH dword ptr [EDX + 0x1c]
0x8071757: PUSH dword ptr [EDX + 0x18]
0x807175a: CALL dword ptr [EDX + 0x14]
0x807175d: JMP 0x08071718
0x807175f: SUB ESP, 0xc
0x8071762: PUSH 0x81cb188
0x8071767: CALL 0x08071c28
0x807176c: ADD ESP, 0x10
0x807176f: JMP 0x08071558
0x8071774: AND EAX, 0xffffffbf
0x8071777: MOV dword ptr [ECX + 0x18], EAX
0x807177a: CALL 0x0806e98c
0x807177f: JMP 0x08071548
0x8071784: PUSH dword ptr [EDX + 0x24]
0x8071787: PUSH dword ptr [EDX + 0x20]
0x807178a: JMP 0x08071754
0x807178c: SUB ESP, 0xc
0x807178f: PUSH dword ptr [EDX + 0x18]
0x8071792: JMP 0x0807175a
0x8071794: SUB ESP, 0x8
0x8071797: PUSH dword ptr [EDX + 0x1c]
0x807179a: JMP 0x08071757
0x807179c: MOV EDX, dword ptr [EBP + 0x8]
0x807179f: OR dword ptr [EDX + 0x18], 0x200
0x80717a6: AND EAX, 0xfffffffe
0x80717a9: MOV [0x08314014], EAX
0x80717ae: JMP 0x080716b8
0x80717b3: AND EAX, 0xfffffffd
0x80717b6: MOV EBX, 0xffffff99
0x80717bb: MOV [0x08314014], EAX
0x80717c0: JMP 0x080716b8
0x80717c5: AND EAX, 0xfffffffb
0x80717c8: MOV EBX, 0xffffff98
0x80717cd: MOV [0x08314014], EAX
0x80717d2: JMP 0x080716b8
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadActions"></a>

## vm_loadActions @ 0x8070cc8

Evidence: `ev_c7666e4a4d210b139fa43d32efb1990b4deff53f25cc156b59bba55a747a6039`

```c

int vm_loadActions(int fd,Vm_Fun_T *pFun,Vm_Node_T **ppNode)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  Vm_Node_T *pVVar4;
  VM_FUN_PTR p_Var5;
  Vm_Program *pProgram;
  uint *ppBuf;
  uint uVar6;
  Vm_Node_T *pVVar7;
  char *str_00;
  int i;
  char *nowFile;
  Vm_Node_T *pNodes;
  int count;
  char *str;
  int nowLine;
  char *fileName;
  Vm_Save_Node_T sn;
  int n;
  
                    /* Unresolved local var: Vm_Node_T * pNode@[???] */
  nowFile = (char *)0x0;
  nowLine = 0;
  iVar3 = vm_loadNumber32(fd,&n);
  if ((iVar3 == 0) || (n < 0)) {
    return 0;
  }
  if (n == 0) {
    *ppNode = (Vm_Node_T *)0x0;
  }
  else {
    pVVar4 = vm_scratchLargeAlloc(n * 0x28);
    pVVar7 = pVVar4;
    for (iVar3 = (n * 5 & 0x1fffffffU) << 1; iVar3 != 0; iVar3 = iVar3 + -1) {
      pVVar7->pFun = (Vm_Fun *)0x0;
      pVVar7 = (Vm_Node_T *)&pVVar7->file;
    }
    i = 0;
    pVVar4->num = n & 0xff;
    pVVar7 = pVVar4;
    if (0 < n) {
      do {
        iVar3 = vm_loadNumber32(fd,&sn.data);
        if (iVar3 == 0) {
          return 0;
        }
        if ((sn.sub.flag & 1) != 0) {
          iVar3 = vm_loadString(fd,&fileName);
          if (iVar3 == 0) {
            return 0;
          }
          nowFile = vm_addFileName(fileName);
          vm_scratchFree(fileName);
          if (nowFile == (char *)0x0) {
            return 0;
          }
        }
        if (((sn.sub.flag & 2) != 0) && (iVar3 = vm_loadNumber32(fd,&nowLine), iVar3 == 0)) {
          return 0;
        }
        if (nowFile == (char *)0x0) {
          return 0;
        }
        pVVar7->file = nowFile;
        pVVar7->line = nowLine;
        uVar6 = (uint)sn.sub.para;
        pVVar7->pFun = pFun;
        pVVar7->parameters = uVar6;
        p_Var5 = vm_getActionRoutine((uint)sn.sub.instruction);
        pVVar7->pFunPtr = p_Var5;
        if (p_Var5 == (VM_FUN_PTR)0x0) {
          return 0;
        }
        pVVar7->num = n - i;
        if (uVar6 != 0) {
          iVar3 = vm_loadPointer(fd,&pVVar7->par1);
          if (iVar3 == 0) {
            return 0;
          }
          if (1 < pVVar7->parameters) {
            iVar3 = vm_loadPointer(fd,&pVVar7->par2);
            if (iVar3 == 0) {
              return 0;
            }
            if (2 < pVVar7->parameters) {
              iVar3 = vm_loadPointer(fd,&pVVar7->par3);
              if (iVar3 == 0) {
                return 0;
              }
              if ((3 < pVVar7->parameters) && (iVar3 = vm_loadPointer(fd,&pVVar7->par4), iVar3 == 0)
                 ) {
                return 0;
              }
            }
          }
          p_Var5 = pVVar7->pFunPtr;
        }
        if (p_Var5 == vm_pushString) {
          iVar3 = vm_loadString(fd,(char **)&pVVar7->par1);
          if (iVar3 == 0) {
            return 0;
          }
          pVVar7->par2 = 0;
        }
        else if (p_Var5 == vm_pushSharedString) {
                    /* Unresolved local var: int strNo@[DW_OP_reg3(EBX)] */
          iVar3 = vm_loadString(fd,&str);
          if (iVar3 == 0) {
            return 0;
          }
          str_00 = str;
          pProgram = vm_compilingProgram();
          uVar6 = vm_findOrAddProgramString(pProgram,str_00);
          vm_scratchFree(str);
          pVVar7->par1 = uVar6;
        }
        else if (p_Var5 == vm_callInFun) {
          if (pVVar7->par1 != 0) {
            pVVar7->par1 = (uint)pGlobalProgram;
          }
        }
        else if (p_Var5 == vm_switch) {
                    /* Unresolved local var: Vm_Case_T * * ppCase@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Case_T * pCase@[DW_OP_reg3(EBX)] */
          iVar3 = vm_loadNumber32(fd,&count);
          if (iVar3 == 0) {
            return 0;
          }
          if (count < 1) {
            return 0;
          }
          iVar3 = count + -1;
          ppBuf = &pVVar7->par1;
          pVVar7->par1 = 0;
          do {
            count = iVar3;
            iVar3 = vm_loadBuffer(fd,ppBuf,0x10);
            if (iVar3 == 0) {
              return 0;
            }
            piVar1 = (int *)*ppBuf;
            piVar1[3] = 0;
            if ((*piVar1 == 3) && (iVar3 = vm_loadString(fd,(char **)(piVar1 + 1)), iVar3 == 0)) {
              return 0;
            }
            ppBuf = (uint *)(piVar1 + 3);
            iVar3 = count + -1;
            bVar2 = 0 < count;
            count = iVar3;
          } while (bVar2);
        }
        i = i + 1;
        pVVar7 = pVVar7 + 1;
      } while (i < n);
    }
    *ppNode = pVVar4;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8070cc8: PUSH EBP
0x8070cc9: MOV EBP, ESP
0x8070ccb: PUSH EDI
0x8070ccc: PUSH ESI
0x8070ccd: PUSH EBX
0x8070cce: SUB ESP, 0x34
0x8070cd1: LEA EAX, [EBP + -0x10]
0x8070cd4: MOV ESI, dword ptr [EBP + 0x8]
0x8070cd7: MOV dword ptr [EBP + -0x2c], 0x0
0x8070cde: MOV dword ptr [EBP + -0x1c], 0x0
0x8070ce5: PUSH EAX
0x8070ce6: PUSH ESI
0x8070ce7: CALL 0x08085d28
0x8070cec: ADD ESP, 0x10
0x8070cef: TEST EAX, EAX
0x8070cf1: JZ 0x08070d94
0x8070cf7: MOV EAX, dword ptr [EBP + -0x10]
0x8070cfa: TEST EAX, EAX
0x8070cfc: JL 0x08070d94
0x8070d02: JNZ 0x08070da0
0x8070d08: MOV EAX, dword ptr [EBP + 0x10]
0x8070d0b: MOV dword ptr [EAX], 0x0
0x8070d11: MOV EAX, 0x1
0x8070d16: LEA ESP, [EBP + -0xc]
0x8070d19: POP EBX
0x8070d1a: POP ESI
0x8070d1b: POP EDI
0x8070d1c: LEAVE
0x8070d1d: RET
0x8070d1e: SUB ESP, 0x8
0x8070d21: LEA EAX, [EBP + -0x24]
0x8070d24: PUSH EAX
0x8070d25: PUSH ESI
0x8070d26: CALL 0x08085d28
0x8070d2b: ADD ESP, 0x10
0x8070d2e: TEST EAX, EAX
0x8070d30: JZ 0x08070d94
0x8070d32: MOV EAX, dword ptr [EBP + -0x24]
0x8070d35: TEST EAX, EAX
0x8070d37: JLE 0x08070d94
0x8070d39: DEC EAX
0x8070d3a: LEA EBX, [EDI + 0x18]
0x8070d3d: MOV dword ptr [EDI + 0x18], 0x0
0x8070d44: MOV dword ptr [EBP + -0x24], EAX
0x8070d47: JMP 0x08070d60
0x8070d4c: MOV EDX, dword ptr [EBP + -0x24]
0x8070d4f: ADD EBX, 0xc
0x8070d52: LEA EAX, [EDX + -0x1]
0x8070d55: TEST EDX, EDX
0x8070d57: MOV dword ptr [EBP + -0x24], EAX
0x8070d5a: JLE 0x08070ea8
0x8070d60: PUSH EAX
0x8070d61: PUSH 0x10
0x8070d63: PUSH EBX
0x8070d64: PUSH ESI
0x8070d65: CALL 0x08085ed8
0x8070d6a: ADD ESP, 0x10
0x8070d6d: TEST EAX, EAX
0x8070d6f: JZ 0x08070d94
0x8070d71: MOV EBX, dword ptr [EBX]
0x8070d73: CMP dword ptr [EBX], 0x3
0x8070d76: MOV dword ptr [EBX + 0xc], 0x0
0x8070d7d: JNZ 0x08070d4c
0x8070d7f: SUB ESP, 0x8
0x8070d82: LEA EAX, [EBX + 0x4]
0x8070d85: PUSH EAX
0x8070d86: PUSH ESI
0x8070d87: CALL 0x08085f40
0x8070d8c: ADD ESP, 0x10
0x8070d8f: TEST EAX, EAX
0x8070d91: JNZ 0x08070d4c
0x8070d93: NOP
0x8070d94: XOR EAX, EAX
0x8070d96: LEA ESP, [EBP + -0xc]
0x8070d99: POP EBX
0x8070d9a: POP ESI
0x8070d9b: POP EDI
0x8070d9c: LEAVE
0x8070d9d: RET
0x8070da0: LEA EAX, [EAX + EAX*0x4]
0x8070da3: SUB ESP, 0xc
0x8070da6: SHL EAX, 0x3
0x8070da9: PUSH EAX
0x8070daa: CALL 0x08064cd0
0x8070daf: MOV ECX, dword ptr [EBP + -0x10]
0x8070db2: LEA ECX, [ECX + ECX*0x4]
0x8070db5: SHL ECX, 0x3
0x8070db8: MOV dword ptr [EBP + -0x28], EAX
0x8070dbb: SHR ECX, 0x2
0x8070dbe: MOV EDI, EAX
0x8070dc0: CLD
0x8070dc1: MOV EAX, dword ptr [EBP + -0x2c]
0x8070dc4: STOSD.REP ES:EDI
0x8070dc6: MOV EDX, dword ptr [EBP + -0x10]
0x8070dc9: MOV dword ptr [EBP + -0x30], 0x0
0x8070dd0: MOVZX EAX, DL
0x8070dd3: MOV ECX, dword ptr [EBP + -0x28]
0x8070dd6: ADD ESP, 0x10
0x8070dd9: CMP dword ptr [EBP + -0x30], EDX
0x8070ddc: MOV dword ptr [ECX + 0x10], EAX
0x8070ddf: JGE 0x08070eba
0x8070de5: MOV EDI, ECX
0x8070de7: NOP
0x8070de8: SUB ESP, 0x8
0x8070deb: LEA EAX, [EBP + -0x14]
0x8070dee: PUSH EAX
0x8070def: PUSH ESI
0x8070df0: CALL 0x08085d28
0x8070df5: ADD ESP, 0x10
0x8070df8: TEST EAX, EAX
0x8070dfa: JZ 0x08070d94
0x8070dfc: MOV AL, byte ptr [EBP + -0x14]
0x8070dff: TEST AL, 0x1
0x8070e01: JNZ 0x08070f2c
0x8070e07: TEST AL, 0x2
0x8070e09: JNZ 0x08070f74
0x8070e0f: MOV EBX, dword ptr [EBP + -0x2c]
0x8070e12: TEST EBX, EBX
0x8070e14: JZ 0x08070d94
0x8070e1a: MOV EDX, dword ptr [EBP + -0x2c]
0x8070e1d: MOV EAX, dword ptr [EBP + -0x1c]
0x8070e20: MOV dword ptr [EDI + 0x4], EDX
0x8070e23: MOV dword ptr [EDI + 0x8], EAX
0x8070e26: MOVZX EBX, byte ptr [EBP + -0x13]
0x8070e2a: MOV ECX, dword ptr [EBP + 0xc]
0x8070e2d: MOV dword ptr [EDI], ECX
0x8070e2f: MOV dword ptr [EDI + 0xc], EBX
0x8070e32: MOVZX EAX, byte ptr [EBP + -0x12]
0x8070e36: PUSH EAX
0x8070e37: CALL 0x08070cb0
0x8070e3c: TEST EAX, EAX
0x8070e3e: POP ECX
0x8070e3f: MOV EDX, EAX
0x8070e41: MOV dword ptr [EDI + 0x14], EAX
0x8070e44: JZ 0x08070d94
0x8070e4a: MOV EAX, dword ptr [EBP + -0x10]
0x8070e4d: SUB EAX, dword ptr [EBP + -0x30]
0x8070e50: TEST EBX, EBX
0x8070e52: MOV dword ptr [EDI + 0x10], EAX
0x8070e55: JLE 0x08070e78
0x8070e57: SUB ESP, 0x8
0x8070e5a: LEA EAX, [EDI + 0x18]
0x8070e5d: PUSH EAX
0x8070e5e: PUSH ESI
0x8070e5f: CALL 0x08085e50
0x8070e64: ADD ESP, 0x10
0x8070e67: TEST EAX, EAX
0x8070e69: JZ 0x08070d94
0x8070e6f: CMP dword ptr [EDI + 0xc], 0x1
0x8070e73: JG 0x08070ecc
0x8070e75: MOV EDX, dword ptr [EDI + 0x14]
0x8070e78: CMP EDX, 0x806a648
0x8070e7e: JZ 0x08070fac
0x8070e84: CMP EDX, 0x806a974
0x8070e8a: JZ 0x08070fd0
0x8070e90: CMP EDX, 0x806eb94
0x8070e96: JZ 0x08070f94
0x8070e9c: CMP EDX, 0x8071acc
0x8070ea2: JZ 0x08070d1e
0x8070ea8: INC dword ptr [EBP + -0x30]
0x8070eab: MOV EAX, dword ptr [EBP + -0x30]
0x8070eae: ADD EDI, 0x28
0x8070eb1: CMP EAX, dword ptr [EBP + -0x10]
0x8070eb4: JL 0x08070de8
0x8070eba: MOV ECX, dword ptr [EBP + -0x28]
0x8070ebd: MOV EDX, dword ptr [EBP + 0x10]
0x8070ec0: MOV EAX, 0x1
0x8070ec5: MOV dword ptr [EDX], ECX
0x8070ec7: JMP 0x08070d16
0x8070ecc: SUB ESP, 0x8
0x8070ecf: LEA EAX, [EDI + 0x1c]
0x8070ed2: PUSH EAX
0x8070ed3: PUSH ESI
0x8070ed4: CALL 0x08085e50
0x8070ed9: ADD ESP, 0x10
0x8070edc: TEST EAX, EAX
0x8070ede: JZ 0x08070d94
0x8070ee4: CMP dword ptr [EDI + 0xc], 0x2
0x8070ee8: JLE 0x08070e75
0x8070eea: SUB ESP, 0x8
0x8070eed: LEA EAX, [EDI + 0x20]
0x8070ef0: PUSH EAX
0x8070ef1: PUSH ESI
0x8070ef2: CALL 0x08085e50
0x8070ef7: ADD ESP, 0x10
0x8070efa: TEST EAX, EAX
0x8070efc: JZ 0x08070d94
0x8070f02: CMP dword ptr [EDI + 0xc], 0x3
0x8070f06: JLE 0x08070e75
0x8070f0c: SUB ESP, 0x8
0x8070f0f: LEA EAX, [EDI + 0x24]
0x8070f12: PUSH EAX
0x8070f13: PUSH ESI
0x8070f14: CALL 0x08085e50
0x8070f19: ADD ESP, 0x10
0x8070f1c: TEST EAX, EAX
0x8070f1e: JNZ 0x08070e75
0x8070f24: JMP 0x08070d96
0x8070f2c: SUB ESP, 0x8
0x8070f2f: LEA EAX, [EBP + -0x18]
0x8070f32: PUSH EAX
0x8070f33: PUSH ESI
0x8070f34: CALL 0x08085f40
0x8070f39: ADD ESP, 0x10
0x8070f3c: TEST EAX, EAX
0x8070f3e: JZ 0x08070d94
0x8070f44: SUB ESP, 0xc
0x8070f47: PUSH dword ptr [EBP + -0x18]
0x8070f4a: CALL 0x0805d218
0x8070f4f: MOV dword ptr [EBP + -0x2c], EAX
0x8070f52: POP EAX
0x8070f53: PUSH dword ptr [EBP + -0x18]
0x8070f56: CALL 0x08064bcc
0x8070f5b: MOV EAX, dword ptr [EBP + -0x2c]
0x8070f5e: ADD ESP, 0x10
0x8070f61: TEST EAX, EAX
0x8070f63: JZ 0x08070d94
0x8070f69: MOV AL, byte ptr [EBP + -0x14]
0x8070f6c: TEST AL, 0x2
0x8070f6e: JZ 0x08070e0f
0x8070f74: SUB ESP, 0x8
0x8070f77: LEA EAX, [EBP + -0x1c]
0x8070f7a: PUSH EAX
0x8070f7b: PUSH ESI
0x8070f7c: CALL 0x08085d28
0x8070f81: ADD ESP, 0x10
0x8070f84: TEST EAX, EAX
0x8070f86: JNZ 0x08070e0f
0x8070f8c: JMP 0x08070d94
0x8070f94: MOV EAX, dword ptr [EDI + 0x18]
0x8070f97: TEST EAX, EAX
0x8070f99: JZ 0x08070ea8
0x8070f9f: MOV EAX, [0x082c46b0]
0x8070fa4: MOV dword ptr [EDI + 0x18], EAX
0x8070fa7: JMP 0x08070ea8
0x8070fac: SUB ESP, 0x8
0x8070faf: LEA EAX, [EDI + 0x18]
0x8070fb2: PUSH EAX
0x8070fb3: PUSH ESI
0x8070fb4: CALL 0x08085f40
0x8070fb9: ADD ESP, 0x10
0x8070fbc: TEST EAX, EAX
0x8070fbe: JZ 0x08070d94
0x8070fc4: MOV dword ptr [EDI + 0x1c], 0x0
0x8070fcb: JMP 0x08070ea8
0x8070fd0: SUB ESP, 0x8
0x8070fd3: LEA EAX, [EBP + -0x20]
0x8070fd6: PUSH EAX
0x8070fd7: PUSH ESI
0x8070fd8: CALL 0x08085f40
0x8070fdd: ADD ESP, 0x10
0x8070fe0: TEST EAX, EAX
0x8070fe2: JZ 0x08070d94
0x8070fe8: SUB ESP, 0x8
0x8070feb: PUSH dword ptr [EBP + -0x20]
0x8070fee: PUSH EDX
0x8070fef: CALL 0x0805d668
0x8070ff4: MOV dword ptr [ESP], EAX
0x8070ff7: CALL 0x080805d4
0x8070ffc: MOV EBX, EAX
0x8070ffe: POP EAX
0x8070fff: PUSH dword ptr [EBP + -0x20]
0x8071002: CALL 0x08064bcc
0x8071007: ADD ESP, 0x10
0x807100a: MOV dword ptr [EDI + 0x18], EBX
0x807100d: JMP 0x08070ea8
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadPointer"></a>

## vm_loadPointer @ 0x8085e50

Evidence: `ev_ce77dfed38324742ad66d566751fed075724c69a060bf7e60b755c2e4309ddd2`

```c

int vm_loadPointer(int fd,void *pptr)

{
  int iVar1;
  void *temp;
  
  iVar1 = vm_read(&temp,4,fd);
  if (iVar1 != 4) {
    return 0;
  }
  if (pptr != (void *)0x0) {
    *(void **)pptr = temp;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8085e50: PUSH EBP
0x8085e51: MOV EBP, ESP
0x8085e53: PUSH EBX
0x8085e54: SUB ESP, 0x8
0x8085e57: MOV EBX, dword ptr [EBP + 0xc]
0x8085e5a: PUSH dword ptr [EBP + 0x8]
0x8085e5d: PUSH 0x4
0x8085e5f: LEA EAX, [EBP + -0x8]
0x8085e62: PUSH EAX
0x8085e63: CALL 0x08078240
0x8085e68: ADD ESP, 0x10
0x8085e6b: XOR EDX, EDX
0x8085e6d: CMP EAX, 0x4
0x8085e70: JZ 0x08085e7c
0x8085e72: MOV EAX, EDX
0x8085e74: MOV EBX, dword ptr [EBP + -0x4]
0x8085e77: LEAVE
0x8085e78: RET
0x8085e7c: TEST EBX, EBX
0x8085e7e: JZ 0x08085e85
0x8085e80: MOV EAX, dword ptr [EBP + -0x8]
0x8085e83: MOV dword ptr [EBX], EAX
0x8085e85: MOV EDX, 0x1
0x8085e8a: MOV EAX, EDX
0x8085e8c: MOV EBX, dword ptr [EBP + -0x4]
0x8085e8f: LEAVE
0x8085e90: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadString"></a>

## vm_loadString @ 0x8085f40

Evidence: `ev_d4c6d03293dcdb5dc85cfcefd60d476ca57447eafca6b8a511c6cf22dec7126f`

```c

int vm_loadString(int fd,char **ppStr)

{
  int iVar1;
  char *str;
  int size;
  
  iVar1 = vm_loadNumber32(fd,&size);
  if (iVar1 != 0) {
    iVar1 = vm_loadBuffer(fd,&str,size);
    if (iVar1 != 0) {
      if (ppStr == (char **)0x0) {
        vm_scratchFree(str);
      }
      else {
        *ppStr = str;
      }
      return 1;
    }
  }
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x8085f40: PUSH EBP
0x8085f41: MOV EBP, ESP
0x8085f43: PUSH ESI
0x8085f44: PUSH EBX
0x8085f45: SUB ESP, 0x18
0x8085f48: LEA EAX, [EBP + -0xc]
0x8085f4b: MOV EBX, dword ptr [EBP + 0x8]
0x8085f4e: MOV ESI, dword ptr [EBP + 0xc]
0x8085f51: PUSH EAX
0x8085f52: PUSH EBX
0x8085f53: CALL 0x08085d28
0x8085f58: ADD ESP, 0x10
0x8085f5b: XOR EDX, EDX
0x8085f5d: TEST EAX, EAX
0x8085f5f: JNZ 0x08085f6c
0x8085f61: LEA ESP, [EBP + -0x8]
0x8085f64: POP EBX
0x8085f65: MOV EAX, EDX
0x8085f67: POP ESI
0x8085f68: LEAVE
0x8085f69: RET
0x8085f6c: PUSH ECX
0x8085f6d: PUSH dword ptr [EBP + -0xc]
0x8085f70: LEA EAX, [EBP + -0x10]
0x8085f73: PUSH EAX
0x8085f74: PUSH EBX
0x8085f75: CALL 0x08085ed8
0x8085f7a: ADD ESP, 0x10
0x8085f7d: XOR EDX, EDX
0x8085f7f: TEST EAX, EAX
0x8085f81: JZ 0x08085f61
0x8085f83: TEST ESI, ESI
0x8085f85: JZ 0x08085f9a
0x8085f87: MOV EAX, dword ptr [EBP + -0x10]
0x8085f8a: MOV dword ptr [ESI], EAX
0x8085f8c: MOV EDX, 0x1
0x8085f91: LEA ESP, [EBP + -0x8]
0x8085f94: POP EBX
0x8085f95: MOV EAX, EDX
0x8085f97: POP ESI
0x8085f98: LEAVE
0x8085f99: RET
0x8085f9a: SUB ESP, 0xc
0x8085f9d: PUSH dword ptr [EBP + -0x10]
0x8085fa0: CALL 0x08064bcc
0x8085fa5: ADD ESP, 0x10
0x8085fa8: MOV EDX, 0x1
0x8085fad: JMP 0x08085f91
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initIdentifiers"></a>

## vm_initIdentifiers @ 0x8062f14

Evidence: `ev_d86b03c08100dcbd48c229146d841a2da5ff973ea80aef957b7e4bac057188fd`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initIdentifiers(void)

{
  uint *puVar1;
  char *name;
  int iVar2;
  int iVar3;
  
                    /* Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  fileList = (Vm_File_Name_T *)0x0;
  currentFile = "__init__";
  currentLine = 0;
  pGlobalProgram = vm_buildProgramHead((char *)0x0);
  if (pGlobalProgram == (Vm_Program *)0x0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
               "No enough memory to init global identifiers.\n","vm_lex.c",0x1338);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  iVar2 = lzss_init();
  iVar3 = 0;
  if (iVar2 == 0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Failed to init LZSS module.\n",
               "vm_lex.c",0x133f);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  for (; name = *(char **)((int)&resWords[0].word + iVar3), name != (char *)0x0;
      iVar3 = iVar3 + 0x14) {
    vm_addKeyword(name,(Vm_Ident_Hash_Elem_T *)((int)&resWords[0].word + iVar3));
    puVar1 = (uint *)((int)&resWords[0].token + iVar3);
    *puVar1 = *puVar1 | 0x80000000;
  }
  vm_addPredefines();
  vm_efunInit();
  vm_efunVarInit();
  vm_dpInit();
  vm_resourceEfunInit();
  vm_graphInit();
  vm_socketInit();
  vm_procInit();
  vm_fileInit();
  vm_objectInit();
  vm_pdbInit();
  vm_rpcInit();
  for (iVar2 = 0; iVar2 < totalEfunModules; iVar2 = iVar2 + 1) {
    (*efunModule[iVar2].pInit)();
  }
  pGlobalVar = (Vm_Global_Var_T *)0x0;
  varTableSize = 0;
  globalVar = 0;
  vm_updateEfuns();
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8062f14: PUSH EBP
0x8062f15: MOV EBP, ESP
0x8062f17: PUSH EBX
0x8062f18: SUB ESP, 0x10
0x8062f1b: PUSH 0x0
0x8062f1d: MOV dword ptr [0x082c48cc], 0x0
0x8062f27: MOV dword ptr [0x08313f80], 0x81c7a1a
0x8062f31: MOV dword ptr [0x08313f68], 0x0
0x8062f3b: CALL 0x08062d7c
0x8062f40: ADD ESP, 0x10
0x8062f43: TEST EAX, EAX
0x8062f45: MOV [0x082c46b0], EAX
0x8062f4a: JNZ 0x08062f7c
0x8062f4c: SUB ESP, 0x8
0x8062f4f: PUSH 0x1338
0x8062f54: PUSH 0x81c74e8
0x8062f59: PUSH 0x81c8080
0x8062f5e: PUSH 0x81c3f74
0x8062f63: PUSH 0x4
0x8062f65: PUSH 0x2
0x8062f67: CALL 0x0804c680
0x8062f6c: ADD ESP, 0x14
0x8062f6f: PUSH 0x0
0x8062f71: CALL 0x0804b848
0x8062f76: ADD ESP, 0x10
0x8062f79: JMP 0x08062f79
0x8062f7c: CALL 0x08090ecc
0x8062f81: XOR EBX, EBX
0x8062f83: TEST EAX, EAX
0x8062f85: JNZ 0x08062fa4
0x8062f87: JMP 0x0806302b
0x8062f8c: LEA EDX, [EBX + 0x82bbdc0]
0x8062f92: CALL 0x08062820
0x8062f97: OR dword ptr [EBX + 0x82bbdc4], 0x80000000
0x8062fa1: ADD EBX, 0x14
0x8062fa4: MOV EAX, dword ptr [EBX + 0x82bbdc0]
0x8062faa: TEST EAX, EAX
0x8062fac: JNZ 0x08062f8c
0x8062fae: CALL 0x0805ea4c
0x8062fb3: XOR EBX, EBX
0x8062fb5: CALL 0x080538bc
0x8062fba: CALL 0x08086d98
0x8062fbf: CALL 0x0809b9f8
0x8062fc4: CALL 0x08051e98
0x8062fc9: CALL 0x0809e5f4
0x8062fce: CALL 0x0809ea78
0x8062fd3: CALL 0x080a52e8
0x8062fd8: CALL 0x080a2144
0x8062fdd: CALL 0x080aa718
0x8062fe2: CALL 0x080a4b7c
0x8062fe7: CALL 0x080a9a5c
0x8062fec: CMP EBX, dword ptr [0x082c48c4]
0x8062ff2: JGE 0x08062ffe
0x8062ff4: CALL dword ptr [EBX*0x8 + 0x82c95a0]
0x8062ffb: INC EBX
0x8062ffc: JMP 0x08062fec
0x8062ffe: MOV dword ptr [0x082c46ac], 0x0
0x8063008: MOV dword ptr [0x082c46a8], 0x0
0x8063012: MOV dword ptr [0x082c46a4], 0x0
0x806301c: CALL 0x08062948
0x8063021: MOV EAX, 0x1
0x8063026: MOV EBX, dword ptr [EBP + -0x4]
0x8063029: LEAVE
0x806302a: RET
0x806302b: SUB ESP, 0x8
0x806302e: PUSH 0x133f
0x8063033: PUSH 0x81c74e8
0x8063038: PUSH 0x81c7a23
0x806303d: PUSH 0x81c3f74
0x8063042: PUSH 0x4
0x8063044: PUSH 0x2
0x8063046: CALL 0x0804c680
0x806304b: ADD ESP, 0x14
0x806304e: PUSH 0x0
0x8063050: CALL 0x0804b848
0x8063055: ADD ESP, 0x10
0x8063058: JMP 0x08063058
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadProgram"></a>

## vm_loadProgram @ 0x806351c

Evidence: `ev_d8f2b70bb9361b617e607e678065ee85aec8df69ea2e67821f3ff8accfa434c4`

```c

int vm_loadProgram(int fd)

{
  bool bVar1;
  Vm_Program *pVVar2;
  int iVar3;
  char **ppcVar4;
  int iVar5;
  Vm_Fun_T *pHiddenFun;
  int count;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg3(EBX)]
                       Unresolved local var: int i@[DW_OP_reg7(EDI)] */
  pVVar2 = vm_compilingProgram();
  iVar3 = vm_loadComment(fd,"HIDDEN_FUNS");
  iVar5 = 0;
  if (iVar3 != 0) {
    iVar3 = vm_loadNumber32(fd,&count);
    if (iVar3 == 0) {
      return 0;
    }
    if (count < 0) {
      return 0;
    }
    iVar3 = count + -1;
    bVar1 = 0 < count;
    count = iVar3;
    if (bVar1) {
      do {
        iVar3 = vm_loadFun(fd,&pHiddenFun);
        if (iVar3 == 0) {
          return 0;
        }
        pHiddenFun->pNext = (Vm_Fun *)pVVar2->pHiddenFuns;
        pVVar2->pHiddenFuns = (Vm_Fun *)pHiddenFun;
        iVar3 = count + -1;
        bVar1 = 0 < count;
        count = iVar3;
      } while (bVar1);
    }
    iVar5 = 1;
    if (pVVar2 != pGlobalProgram) {
      iVar3 = vm_loadComment(fd,"INHERITS");
      iVar5 = 0;
      if (iVar3 != 0) {
        iVar3 = vm_loadNumber32(fd,&count);
        if ((iVar3 == 0) || (count < 0)) {
          return 0;
        }
        ppcVar4 = pdb_allocateMemory(count << 2,"VM","vm_lex.c",0x16ae);
        iVar5 = 0;
        pVVar2->ppInheritProgram = ppcVar4;
        if (ppcVar4 != (char **)0x0) {
          iVar3 = 0;
          if (0 < count) {
            while( true ) {
              iVar5 = vm_loadString2(fd,ppcVar4 + iVar3);
              if (iVar5 == 0) {
                return 0;
              }
              iVar3 = iVar3 + 1;
              pVVar2->inheritProgram = iVar3;
              if (count <= iVar3) break;
              ppcVar4 = pVVar2->ppInheritProgram;
            }
          }
          iVar3 = vm_loadComment(fd,"CONSTUCTION");
          iVar5 = 0;
          if (iVar3 != 0) {
            iVar3 = vm_loadNumber32(fd,&count);
            if (iVar3 == 0) {
              return 0;
            }
            if (count < 0) {
              return 0;
            }
            if ((0 < count) &&
               (iVar3 = vm_loadBuffer2(fd,&pVVar2->constructFuns,count * 4), iVar3 == 0)) {
              return 0;
            }
            pVVar2->constructFunNum = count;
            iVar3 = vm_loadComment(fd,"DESTUCTION");
            iVar5 = 0;
            if (iVar3 != 0) {
              iVar3 = vm_loadNumber32(fd,&count);
              if (iVar3 == 0) {
                return 0;
              }
              if (count < 0) {
                return 0;
              }
              if ((0 < count) &&
                 (iVar3 = vm_loadBuffer2(fd,&pVVar2->destructFuns,count * 4), iVar3 == 0)) {
                return 0;
              }
              pVVar2->destructFunNum = count;
              iVar5 = 1;
            }
          }
        }
      }
    }
  }
  return iVar5;
}


```

<details><summary>汇编</summary>

```asm
0x806351c: PUSH EBP
0x806351d: MOV EBP, ESP
0x806351f: PUSH EDI
0x8063520: PUSH ESI
0x8063521: PUSH EBX
0x8063522: SUB ESP, 0xc
0x8063525: CALL 0x0805d668
0x806352a: SUB ESP, 0x8
0x806352d: MOV ESI, dword ptr [EBP + 0x8]
0x8063530: PUSH 0x81c7a40
0x8063535: PUSH ESI
0x8063536: MOV EBX, EAX
0x8063538: CALL 0x08085d1c
0x806353d: ADD ESP, 0x10
0x8063540: XOR EDX, EDX
0x8063542: TEST EAX, EAX
0x8063544: JNZ 0x08063550
0x8063546: LEA ESP, [EBP + -0xc]
0x8063549: POP EBX
0x806354a: POP ESI
0x806354b: MOV EAX, EDX
0x806354d: POP EDI
0x806354e: LEAVE
0x806354f: RET
0x8063550: SUB ESP, 0x8
0x8063553: LEA EAX, [EBP + -0x10]
0x8063556: PUSH EAX
0x8063557: PUSH ESI
0x8063558: CALL 0x08085d28
0x806355d: ADD ESP, 0x10
0x8063560: TEST EAX, EAX
0x8063562: JZ 0x08063648
0x8063568: MOV EAX, dword ptr [EBP + -0x10]
0x806356b: TEST EAX, EAX
0x806356d: JS 0x08063648
0x8063573: LEA EDX, [EAX + -0x1]
0x8063576: TEST EAX, EAX
0x8063578: MOV dword ptr [EBP + -0x10], EDX
0x806357b: JLE 0x080635ae
0x806357d: LEA EDI, [EBP + -0x14]
0x8063580: SUB ESP, 0x8
0x8063583: PUSH EDI
0x8063584: PUSH ESI
0x8063585: CALL 0x0806fb0c
0x806358a: ADD ESP, 0x10
0x806358d: TEST EAX, EAX
0x806358f: JZ 0x08063648
0x8063595: MOV EAX, dword ptr [EBP + -0x14]
0x8063598: MOV EDX, dword ptr [EBX + 0x3c]
0x806359b: MOV dword ptr [EAX + 0x28], EDX
0x806359e: MOV dword ptr [EBX + 0x3c], EAX
0x80635a1: MOV EAX, dword ptr [EBP + -0x10]
0x80635a4: LEA EDX, [EAX + -0x1]
0x80635a7: TEST EAX, EAX
0x80635a9: MOV dword ptr [EBP + -0x10], EDX
0x80635ac: JG 0x08063580
0x80635ae: CMP EBX, dword ptr [0x082c46b0]
0x80635b4: MOV EDX, 0x1
0x80635b9: JZ 0x08063546
0x80635bb: SUB ESP, 0x8
0x80635be: PUSH 0x81c7a4c
0x80635c3: PUSH ESI
0x80635c4: CALL 0x08085d1c
0x80635c9: ADD ESP, 0x10
0x80635cc: XOR EDX, EDX
0x80635ce: TEST EAX, EAX
0x80635d0: JZ 0x08063546
0x80635d6: SUB ESP, 0x8
0x80635d9: LEA EAX, [EBP + -0x10]
0x80635dc: PUSH EAX
0x80635dd: PUSH ESI
0x80635de: CALL 0x08085d28
0x80635e3: ADD ESP, 0x10
0x80635e6: TEST EAX, EAX
0x80635e8: JZ 0x08063648
0x80635ea: MOV EAX, dword ptr [EBP + -0x10]
0x80635ed: TEST EAX, EAX
0x80635ef: JS 0x08063648
0x80635f1: PUSH 0x16ae
0x80635f6: PUSH 0x81c74e8
0x80635fb: PUSH 0x81c34ab
0x8063600: SHL EAX, 0x2
0x8063603: PUSH EAX
0x8063604: CALL 0x08053390
0x8063609: ADD ESP, 0x10
0x806360c: XOR EDX, EDX
0x806360e: TEST EAX, EAX
0x8063610: MOV dword ptr [EBX + 0x14], EAX
0x8063613: JZ 0x08063546
0x8063619: XOR EDI, EDI
0x806361b: CMP EDI, dword ptr [EBP + -0x10]
0x806361e: JL 0x08063634
0x8063620: JMP 0x08063654
0x8063624: LEA EAX, [EDI + 0x1]
0x8063627: CMP EAX, dword ptr [EBP + -0x10]
0x806362a: MOV dword ptr [EBX + 0x10], EAX
0x806362d: MOV EDI, EAX
0x806362f: JGE 0x08063654
0x8063631: MOV EAX, dword ptr [EBX + 0x14]
0x8063634: SUB ESP, 0x8
0x8063637: LEA EAX, [EAX + EDI*0x4]
0x806363a: PUSH EAX
0x806363b: PUSH ESI
0x806363c: CALL 0x08086044
0x8063641: ADD ESP, 0x10
0x8063644: TEST EAX, EAX
0x8063646: JNZ 0x08063624
0x8063648: LEA ESP, [EBP + -0xc]
0x806364b: POP EBX
0x806364c: POP ESI
0x806364d: XOR EDX, EDX
0x806364f: MOV EAX, EDX
0x8063651: POP EDI
0x8063652: LEAVE
0x8063653: RET
0x8063654: SUB ESP, 0x8
0x8063657: PUSH 0x81c7a55
0x806365c: PUSH ESI
0x806365d: CALL 0x08085d1c
0x8063662: ADD ESP, 0x10
0x8063665: XOR EDX, EDX
0x8063667: TEST EAX, EAX
0x8063669: JZ 0x08063546
0x806366f: SUB ESP, 0x8
0x8063672: LEA EAX, [EBP + -0x10]
0x8063675: PUSH EAX
0x8063676: PUSH ESI
0x8063677: CALL 0x08085d28
0x806367c: ADD ESP, 0x10
0x806367f: TEST EAX, EAX
0x8063681: JZ 0x08063648
0x8063683: MOV EDX, dword ptr [EBP + -0x10]
0x8063686: TEST EDX, EDX
0x8063688: JL 0x08063648
0x806368a: JLE 0x080636af
0x806368c: LEA EAX, [EDX*0x4 + 0x0]
0x8063693: PUSH EDI
0x8063694: PUSH EAX
0x8063695: LEA EAX, [EBX + 0x20]
0x8063698: PUSH EAX
0x8063699: PUSH ESI
0x806369a: CALL 0x08085fb0
0x806369f: ADD ESP, 0x10
0x80636a2: XOR EDX, EDX
0x80636a4: TEST EAX, EAX
0x80636a6: JZ 0x08063546
0x80636ac: MOV EDX, dword ptr [EBP + -0x10]
0x80636af: SUB ESP, 0x8
0x80636b2: MOV dword ptr [EBX + 0x18], EDX
0x80636b5: PUSH 0x81c7a61
0x80636ba: PUSH ESI
0x80636bb: CALL 0x08085d1c
0x80636c0: ADD ESP, 0x10
0x80636c3: XOR EDX, EDX
0x80636c5: TEST EAX, EAX
0x80636c7: JZ 0x08063546
0x80636cd: SUB ESP, 0x8
0x80636d0: LEA EAX, [EBP + -0x10]
0x80636d3: PUSH EAX
0x80636d4: PUSH ESI
0x80636d5: CALL 0x08085d28
0x80636da: ADD ESP, 0x10
0x80636dd: TEST EAX, EAX
0x80636df: JZ 0x08063648
0x80636e5: MOV EDX, dword ptr [EBP + -0x10]
0x80636e8: TEST EDX, EDX
0x80636ea: JL 0x08063648
0x80636f0: JLE 0x08063715
0x80636f2: LEA EAX, [EDX*0x4 + 0x0]
0x80636f9: PUSH ECX
0x80636fa: PUSH EAX
0x80636fb: LEA EAX, [EBX + 0x24]
0x80636fe: PUSH EAX
0x80636ff: PUSH ESI
0x8063700: CALL 0x08085fb0
0x8063705: ADD ESP, 0x10
0x8063708: XOR EDX, EDX
0x806370a: TEST EAX, EAX
0x806370c: JZ 0x08063546
0x8063712: MOV EDX, dword ptr [EBP + -0x10]
0x8063715: MOV dword ptr [EBX + 0x1c], EDX
0x8063718: MOV EDX, 0x1
0x806371d: JMP 0x08063546
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadDefines"></a>

## vm_loadDefines @ 0x80632ec

Evidence: `ev_ed6f60195654aef56767154253554c7ef198dd39814a6333aaaf970ceb68a5bb`

```c

int vm_loadDefines(int fd)

{
  int iVar1;
  char *exps;
  char *name;
  int nargs;
  int count;
  
  iVar1 = vm_loadNumber32(fd,&count);
  if ((iVar1 == 0) || (count < 0)) {
    return 0;
  }
  if (0 < count) {
    do {
      count = count + -1;
      iVar1 = vm_loadNumber32(fd,&nargs);
      if (iVar1 == 0) {
        return 0;
      }
      iVar1 = vm_loadString(fd,&name);
      if (iVar1 == 0) {
        return 0;
      }
      iVar1 = vm_loadString(fd,&exps);
      if (iVar1 == 0) {
        return 0;
      }
      vm_addDefine(name,nargs,exps);
      vm_scratchFree(name);
      vm_scratchFree(exps);
    } while (0 < count);
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80632ec: PUSH EBP
0x80632ed: MOV EBP, ESP
0x80632ef: PUSH ESI
0x80632f0: PUSH EBX
0x80632f1: SUB ESP, 0x18
0x80632f4: LEA EAX, [EBP + -0xc]
0x80632f7: MOV EBX, dword ptr [EBP + 0x8]
0x80632fa: PUSH EAX
0x80632fb: PUSH EBX
0x80632fc: CALL 0x08085d28
0x8063301: ADD ESP, 0x10
0x8063304: TEST EAX, EAX
0x8063306: JZ 0x0806339c
0x806330c: MOV EDX, dword ptr [EBP + -0xc]
0x806330f: TEST EDX, EDX
0x8063311: JS 0x0806339c
0x8063317: LEA EAX, [EDX + -0x1]
0x806331a: TEST EDX, EDX
0x806331c: MOV dword ptr [EBP + -0xc], EAX
0x806331f: JLE 0x0806338e
0x8063321: LEA ESI, [EBP + -0x10]
0x8063324: SUB ESP, 0x8
0x8063327: PUSH ESI
0x8063328: PUSH EBX
0x8063329: CALL 0x08085d28
0x806332e: ADD ESP, 0x10
0x8063331: TEST EAX, EAX
0x8063333: JZ 0x0806339c
0x8063335: SUB ESP, 0x8
0x8063338: LEA EAX, [EBP + -0x14]
0x806333b: PUSH EAX
0x806333c: PUSH EBX
0x806333d: CALL 0x08085f40
0x8063342: ADD ESP, 0x10
0x8063345: TEST EAX, EAX
0x8063347: JZ 0x0806339c
0x8063349: SUB ESP, 0x8
0x806334c: LEA EAX, [EBP + -0x18]
0x806334f: PUSH EAX
0x8063350: PUSH EBX
0x8063351: CALL 0x08085f40
0x8063356: ADD ESP, 0x10
0x8063359: TEST EAX, EAX
0x806335b: JZ 0x0806339c
0x806335d: PUSH EAX
0x806335e: PUSH dword ptr [EBP + -0x18]
0x8063361: PUSH dword ptr [EBP + -0x10]
0x8063364: PUSH dword ptr [EBP + -0x14]
0x8063367: CALL 0x0805e440
0x806336c: POP EAX
0x806336d: PUSH dword ptr [EBP + -0x14]
0x8063370: CALL 0x08064bcc
0x8063375: POP EAX
0x8063376: PUSH dword ptr [EBP + -0x18]
0x8063379: CALL 0x08064bcc
0x806337e: MOV EDX, dword ptr [EBP + -0xc]
0x8063381: ADD ESP, 0x10
0x8063384: LEA EAX, [EDX + -0x1]
0x8063387: TEST EDX, EDX
0x8063389: MOV dword ptr [EBP + -0xc], EAX
0x806338c: JG 0x08063324
0x806338e: LEA ESP, [EBP + -0x8]
0x8063391: POP EBX
0x8063392: MOV EAX, 0x1
0x8063397: POP ESI
0x8063398: LEAVE
0x8063399: RET
0x806339c: LEA ESP, [EBP + -0x8]
0x806339f: POP EBX
0x80633a0: XOR EAX, EAX
0x80633a2: POP ESI
0x80633a3: LEAVE
0x80633a4: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

