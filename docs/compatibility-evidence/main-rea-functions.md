# main-rea：Ghidra/REA 函数证据

未执行目标。下面是引擎原始伪代码和反汇编，不保证可编译。

<a id="vm_initSqliteEfun"></a>

## vm_initSqliteEfun @ 0x8129000

Evidence: `ev_11144b055a8786e11b5440df6e78342f067f062c28b9c7c4406dc30846b80b0e`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initSqliteEfun(void)

{
  vm_sqliteInit();
  vm_updateEfuns();
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8129000: PUSH EBP
0x8129001: MOV EBP, ESP
0x8129003: SUB ESP, 0x8
0x8129006: CALL 0x0812aba0
0x812900b: CALL 0x0808600c
0x8129010: MOV EAX, 0x1
0x8129015: LEAVE
0x8129016: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="fm_unpack"></a>

## fm_unpack @ 0x80b5500

Evidence: `ev_14ecbc2d3556c5e3a775b1b2fd7154aabd4a64448fa6cda902a3aedf7f8087bc`

```c

int fm_unpack(char *packedFile,char *path,void *ppRet,int *pSize,uint *pAttrib)

{
  char cVar1;
  FILE *pFVar2;
  int iVar3;
  uchar *to;
  char *__dest;
  void *__ptr;
  uint uVar4;
  uint uVar5;
  char *pcVar6;
  int nCount;
  File_List_T *pflist;
  stat st;
  char orgName [256];
  Fm_Catalog_T catalog;
  
                    /* Unresolved local var: FILE * fp@[???]
                       Unresolved local var: char * pBuffer@[DW_OP_reg6(ESI)]
                       Unresolved local var: int nSize@[DW_OP_reg7(EDI)] */
  if (_fm_unpack == 1) {
                    /* Unresolved local var: char * p@[???] */
    uVar4 = 0xffffffff;
    pcVar6 = packedFile;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar6 + 1;
    } while (cVar1 != '\0');
    uVar5 = 0xffffffff;
    pcVar6 = path;
    do {
      if (uVar5 == 0) break;
      uVar5 = uVar5 - 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar6 + 1;
    } while (cVar1 != '\0');
    pcVar6 = orgName;
    if (0xff < (~uVar5 - 2) + ~uVar4) {
      return 0;
    }
    strcpy(pcVar6,packedFile);
    __dest = strrchr(pcVar6,0x2e);
    if (__dest == (char *)0x0) {
      return 0;
    }
    strcpy(__dest,path);
                    /* Unresolved local var: FILE * fp@[DW_OP_reg6(ESI)]
                       Unresolved local var: char * pBuffer@[DW_OP_reg3(EBX)] */
    while ((orgName[0] == '/' || (orgName[0] == '\\'))) {
      pcVar6 = pcVar6 + 1;
      orgName[0] = *pcVar6;
    }
    __xstat(3,pcVar6,(stat *)&st);
    if ((st.st_mode._1_1_ & 0x40) == 0) {
      if (ppRet != (void *)0x0) {
        pFVar2 = fopen(pcVar6,"rb");
        if (pFVar2 == (FILE *)0x0) {
          return 0;
        }
        __ptr = pdb_allocateMemory(st.st_size,"VM","/home/cc/xxxx/gameshell/src/vm/vm_filem.c",0x3df
                                  );
        if (__ptr == (void *)0x0) {
          fclose(pFVar2);
          return 0;
        }
        fread(__ptr,1,st.st_size,pFVar2);
        fclose(pFVar2);
        *(void **)ppRet = __ptr;
      }
      if (pSize != (int *)0x0) {
        *pSize = st.st_size;
      }
      if (pAttrib != (uint *)0x0) {
        *pAttrib = 0;
      }
    }
    else {
      iVar3 = fm_lookupFiles(pcVar6,&nCount,&pflist);
      if (iVar3 == 0) {
        return 0;
      }
      if (ppRet == (void *)0x0) {
        pdb_freeMemory(pflist,"VM","/home/cc/xxxx/gameshell/src/vm/vm_filem.c",0x3cc);
      }
      else {
        *(File_List_T **)ppRet = pflist;
      }
      if (pSize != (int *)0x0) {
        *pSize = nCount << 6;
      }
      if (pAttrib != (uint *)0x0) {
        *pAttrib = 1;
        return 1;
      }
    }
    return 1;
  }
                    /* Unresolved local var: FILE * fp@[DW_OP_reg0(EAX)] */
  pFVar2 = fopen(packedFile,"rb");
  if (pFVar2 == (FILE *)0x0) {
    return 0;
  }
  iVar3 = fm_readCatalog((FILE *)pFVar2,path,&catalog);
  if (iVar3 != 0) {
    if (ppRet == (void *)0x0) {
      if (pSize != (int *)0x0) {
        *pSize = catalog.nOrgSize;
      }
    }
    else {
      to = pdb_allocateMemory(catalog.nOrgSize,"VM","/home/cc/xxxx/gameshell/src/vm/vm_filem.c",0xcd
                             );
      if (to == (uchar *)0x0) goto LAB_080b570b;
      fseek(pFVar2,catalog.nOffset,0);
      if ((catalog.nAttrib & 0x80000001) == 0) {
        uVar4 = lzss_decompress((FILE *)pFVar2,(uchar *)0x0,catalog.nSize,(FILE *)0x0,to,
                                catalog.nOrgSize);
        if (uVar4 != catalog.nOrgSize) {
          pdb_freeMemory(to,"VM","/home/cc/xxxx/gameshell/src/vm/vm_filem.c",0xf0);
          goto LAB_080b570b;
        }
      }
      else {
        fread(to,1,catalog.nOrgSize,pFVar2);
      }
      *(uchar **)ppRet = to;
      if (pSize != (int *)0x0) {
        *pSize = catalog.nOrgSize;
      }
    }
    if (pAttrib != (uint *)0x0) {
      *pAttrib = catalog.nAttrib;
    }
    fclose(pFVar2);
    return 1;
  }
LAB_080b570b:
                    /* Unresolved local var: int ret@[DW_OP_reg0(EAX)] */
  fclose(pFVar2);
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x80b5500: PUSH EBP
0x80b5501: MOV EBP, ESP
0x80b5503: PUSH EDI
0x80b5504: PUSH ESI
0x80b5505: PUSH EBX
0x80b5506: SUB ESP, 0x1cc
0x80b550c: CMP dword ptr [0x08609b40], 0x1
0x80b5513: MOV EBX, dword ptr [EBP + 0x8]
0x80b5516: JZ 0x080b55c8
0x80b551c: SUB ESP, 0x8
0x80b551f: PUSH 0x82a92b4
0x80b5524: PUSH EBX
0x80b5525: CALL 0x0804baa4
0x80b552a: ADD ESP, 0x10
0x80b552d: XOR EDX, EDX
0x80b552f: TEST EAX, EAX
0x80b5531: JZ 0x080b56fc
0x80b5537: SUB ESP, 0xc
0x80b553a: MOV EBX, EAX
0x80b553c: LEA EAX, [EBP + -0x58]
0x80b553f: PUSH EAX
0x80b5540: MOV EDX, dword ptr [EBP + 0xc]
0x80b5543: MOV EAX, EBX
0x80b5545: CALL 0x080b51ec
0x80b554a: ADD ESP, 0x10
0x80b554d: TEST EAX, EAX
0x80b554f: JZ 0x080b5708
0x80b5555: MOV EAX, dword ptr [EBP + 0x10]
0x80b5558: TEST EAX, EAX
0x80b555a: JZ 0x080b5728
0x80b5560: MOV EDI, dword ptr [EBP + -0x4c]
0x80b5563: PUSH 0xcd
0x80b5568: PUSH 0x82b133c
0x80b556d: PUSH 0x82a376b
0x80b5572: PUSH EDI
0x80b5573: CALL 0x080ce784
0x80b5578: ADD ESP, 0x10
0x80b557b: TEST EAX, EAX
0x80b557d: MOV ESI, EAX
0x80b557f: JZ 0x080b5708
0x80b5585: PUSH EAX
0x80b5586: PUSH 0x0
0x80b5588: PUSH dword ptr [EBP + -0x54]
0x80b558b: PUSH EBX
0x80b558c: CALL 0x0804b4b4
0x80b5591: ADD ESP, 0x10
0x80b5594: TEST dword ptr [EBP + -0x58], 0x80000001
0x80b559b: JZ 0x080b5756
0x80b55a1: PUSH EBX
0x80b55a2: PUSH EDI
0x80b55a3: PUSH 0x1
0x80b55a5: PUSH ESI
0x80b55a6: CALL 0x0804b3d4
0x80b55ab: ADD ESP, 0x10
0x80b55ae: MOV ECX, dword ptr [EBP + 0x14]
0x80b55b1: MOV EAX, dword ptr [EBP + 0x10]
0x80b55b4: TEST ECX, ECX
0x80b55b6: MOV dword ptr [EAX], ESI
0x80b55b8: JZ 0x080b5737
0x80b55be: MOV EDX, dword ptr [EBP + 0x14]
0x80b55c1: MOV dword ptr [EDX], EDI
0x80b55c3: JMP 0x080b5737
0x80b55c8: XOR EDX, EDX
0x80b55ca: MOV AL, DL
0x80b55cc: MOV EDI, EBX
0x80b55ce: CLD
0x80b55cf: MOV ECX, 0xffffffff
0x80b55d4: SCASB.REPNE ES:EDI
0x80b55d6: NOT ECX
0x80b55d8: MOV dword ptr [EBP + 0xfffffe34], ECX
0x80b55de: MOV EDI, dword ptr [EBP + 0xc]
0x80b55e1: MOV ECX, 0xffffffff
0x80b55e6: SCASB.REPNE ES:EDI
0x80b55e8: NOT ECX
0x80b55ea: MOV EAX, dword ptr [EBP + 0xfffffe34]
0x80b55f0: LEA ECX, [ECX + EAX*0x1 + -0x2]
0x80b55f4: CMP ECX, 0x100
0x80b55fa: LEA ESI, [EBP + 0xfffffea8]
0x80b5600: JNC 0x080b5714
0x80b5606: SUB ESP, 0x8
0x80b5609: PUSH EBX
0x80b560a: PUSH ESI
0x80b560b: CALL 0x0804bc54
0x80b5610: POP EDI
0x80b5611: POP EAX
0x80b5612: PUSH 0x2e
0x80b5614: PUSH ESI
0x80b5615: CALL 0x0804b844
0x80b561a: ADD ESP, 0x10
0x80b561d: TEST EAX, EAX
0x80b561f: JZ 0x080b5714
0x80b5625: SUB ESP, 0x8
0x80b5628: PUSH dword ptr [EBP + 0xc]
0x80b562b: PUSH EAX
0x80b562c: CALL 0x0804bc54
0x80b5631: MOV EDI, ESI
0x80b5633: ADD ESP, 0x10
0x80b5636: MOV AL, byte ptr [EBP + 0xfffffea8]
0x80b563c: CMP AL, 0x2f
0x80b563e: JZ 0x080b5720
0x80b5644: CMP AL, 0x5c
0x80b5646: JZ 0x080b5720
0x80b564c: LEA EAX, [EBP + 0xfffffe48]
0x80b5652: PUSH ESI
0x80b5653: PUSH EAX
0x80b5654: PUSH EDI
0x80b5655: PUSH 0x3
0x80b5657: CALL 0x0804b454
0x80b565c: ADD ESP, 0x10
0x80b565f: TEST byte ptr [EBP + 0xfffffe59], 0x40
0x80b5666: JNZ 0x080b578b
0x80b566c: MOV EAX, dword ptr [EBP + 0x10]
0x80b566f: TEST EAX, EAX
0x80b5671: JZ 0x080b56d2
0x80b5673: SUB ESP, 0x8
0x80b5676: PUSH 0x82a92b4
0x80b567b: PUSH EDI
0x80b567c: CALL 0x0804baa4
0x80b5681: ADD ESP, 0x10
0x80b5684: XOR EDX, EDX
0x80b5686: TEST EAX, EAX
0x80b5688: MOV ESI, EAX
0x80b568a: JZ 0x080b56fc
0x80b568c: PUSH 0x3df
0x80b5691: PUSH 0x82b133c
0x80b5696: PUSH 0x82a376b
0x80b569b: PUSH dword ptr [EBP + 0xfffffe74]
0x80b56a1: CALL 0x080ce784
0x80b56a6: ADD ESP, 0x10
0x80b56a9: TEST EAX, EAX
0x80b56ab: MOV EBX, EAX
0x80b56ad: JZ 0x080b57f4
0x80b56b3: PUSH ESI
0x80b56b4: PUSH dword ptr [EBP + 0xfffffe74]
0x80b56ba: PUSH 0x1
0x80b56bc: PUSH EAX
0x80b56bd: CALL 0x0804b3d4
0x80b56c2: MOV dword ptr [ESP], ESI
0x80b56c5: CALL 0x0804b814
0x80b56ca: MOV EDX, dword ptr [EBP + 0x10]
0x80b56cd: ADD ESP, 0x10
0x80b56d0: MOV dword ptr [EDX], EBX
0x80b56d2: MOV EAX, dword ptr [EBP + 0x14]
0x80b56d5: TEST EAX, EAX
0x80b56d7: JZ 0x080b56e4
0x80b56d9: MOV EAX, dword ptr [EBP + 0xfffffe74]
0x80b56df: MOV ECX, dword ptr [EBP + 0x14]
0x80b56e2: MOV dword ptr [ECX], EAX
0x80b56e4: MOV EAX, dword ptr [EBP + 0x18]
0x80b56e7: TEST EAX, EAX
0x80b56e9: JZ 0x080b56f4
0x80b56eb: MOV EAX, dword ptr [EBP + 0x18]
0x80b56ee: MOV dword ptr [EAX], 0x0
0x80b56f4: MOV EDX, 0x1
0x80b56f9: LEA ESI, [ESI]
0x80b56fc: LEA ESP, [EBP + -0xc]
0x80b56ff: POP EBX
0x80b5700: POP ESI
0x80b5701: MOV EAX, EDX
0x80b5703: POP EDI
0x80b5704: LEAVE
0x80b5705: RET
0x80b5708: SUB ESP, 0xc
0x80b570b: PUSH EBX
0x80b570c: CALL 0x0804b814
0x80b5711: LEA ESI, [ESI]
0x80b5714: XOR EDX, EDX
0x80b5716: LEA ESP, [EBP + -0xc]
0x80b5719: POP EBX
0x80b571a: POP ESI
0x80b571b: MOV EAX, EDX
0x80b571d: POP EDI
0x80b571e: LEAVE
0x80b571f: RET
0x80b5720: INC EDI
0x80b5721: MOV AL, byte ptr [EDI]
0x80b5723: JMP 0x080b563c
0x80b5728: MOV EAX, dword ptr [EBP + 0x14]
0x80b572b: TEST EAX, EAX
0x80b572d: JZ 0x080b5737
0x80b572f: MOV EAX, dword ptr [EBP + -0x4c]
0x80b5732: MOV EDX, dword ptr [EBP + 0x14]
0x80b5735: MOV dword ptr [EDX], EAX
0x80b5737: MOV EDX, dword ptr [EBP + 0x18]
0x80b573a: TEST EDX, EDX
0x80b573c: JZ 0x080b5746
0x80b573e: MOV EAX, dword ptr [EBP + -0x58]
0x80b5741: MOV ECX, dword ptr [EBP + 0x18]
0x80b5744: MOV dword ptr [ECX], EAX
0x80b5746: SUB ESP, 0xc
0x80b5749: PUSH EBX
0x80b574a: CALL 0x0804b814
0x80b574f: MOV EDX, 0x1
0x80b5754: JMP 0x080b56fc
0x80b5756: SUB ESP, 0x8
0x80b5759: PUSH EDI
0x80b575a: PUSH ESI
0x80b575b: PUSH 0x0
0x80b575d: PUSH dword ptr [EBP + -0x50]
0x80b5760: PUSH 0x0
0x80b5762: PUSH EBX
0x80b5763: CALL 0x080cb47c
0x80b5768: ADD ESP, 0x20
0x80b576b: CMP EAX, EDI
0x80b576d: JZ 0x080b55ae
0x80b5773: PUSH 0xf0
0x80b5778: PUSH 0x82b133c
0x80b577d: PUSH 0x82a376b
0x80b5782: PUSH ESI
0x80b5783: CALL 0x080ceb94
0x80b5788: POP ESI
0x80b5789: JMP 0x080b570b
0x80b578b: SUB ESP, 0xc
0x80b578e: LEA EAX, [EBP + 0xfffffe44]
0x80b5794: PUSH EAX
0x80b5795: LEA EDX, [EBP + 0xfffffe40]
0x80b579b: MOV EAX, EDI
0x80b579d: CALL 0x080b46a8
0x80b57a2: ADD ESP, 0x10
0x80b57a5: XOR EDX, EDX
0x80b57a7: TEST EAX, EAX
0x80b57a9: JZ 0x080b56fc
0x80b57af: MOV EBX, dword ptr [EBP + 0x10]
0x80b57b2: TEST EBX, EBX
0x80b57b4: JZ 0x080b5807
0x80b57b6: MOV EAX, dword ptr [EBP + 0xfffffe44]
0x80b57bc: MOV EDX, dword ptr [EBP + 0x10]
0x80b57bf: MOV dword ptr [EDX], EAX
0x80b57c1: MOV ECX, dword ptr [EBP + 0x14]
0x80b57c4: TEST ECX, ECX
0x80b57c6: JZ 0x080b57d6
0x80b57c8: MOV EAX, dword ptr [EBP + 0xfffffe40]
0x80b57ce: SHL EAX, 0x6
0x80b57d1: MOV ECX, dword ptr [EBP + 0x14]
0x80b57d4: MOV dword ptr [ECX], EAX
0x80b57d6: MOV EDX, dword ptr [EBP + 0x18]
0x80b57d9: TEST EDX, EDX
0x80b57db: JZ 0x080b56f4
0x80b57e1: MOV EAX, dword ptr [EBP + 0x18]
0x80b57e4: MOV EDX, 0x1
0x80b57e9: MOV dword ptr [EAX], 0x1
0x80b57ef: JMP 0x080b56fc
0x80b57f4: SUB ESP, 0xc
0x80b57f7: PUSH ESI
0x80b57f8: CALL 0x0804b814
0x80b57fd: ADD ESP, 0x10
0x80b5800: XOR EDX, EDX
0x80b5802: JMP 0x080b5716
0x80b5807: PUSH 0x3cc
0x80b580c: PUSH 0x82b133c
0x80b5811: PUSH 0x82a376b
0x80b5816: PUSH dword ptr [EBP + 0xfffffe44]
0x80b581c: CALL 0x080ceb94
0x80b5821: ADD ESP, 0x10
0x80b5824: JMP 0x080b57c1
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_getDirInPackedFs"></a>

## vm_getDirInPackedFs @ 0x80eb208

Evidence: `ev_1bf904a7f739f5af38dd308db493c7e0214879cb798b7acf03dcb4d922960bc8`

```c

int __regparm2 vm_getDirInPackedFs(Vm_PCB_T *pPCB,char *fullName,int flags,int maxCount)

{
  char cVar1;
  char cVar2;
  Vm_Value_T *pVVar3;
  Vm_PCB *pVVar4;
  bool bVar5;
  undefined2 uVar6;
  char *pcVar7;
  int iVar8;
  Vm_Buffer_T *pBuffer_00;
  Vm_Catch_Buf *__env;
  Vm_Array_T *pArray_00;
  Vm_Array_T *pArray_01;
  Vm_Value *pVVar9;
  Vm_Thread_Info_T *pVVar10;
  int iVar11;
  char *pcVar12;
  int local_23c;
  int local_238;
  int i;
  Vm_Buffer_T *pBuffer;
  Vm_Array_T *pArray;
  Fm_Catalog_T *pCatalog;
  int size;
  uint attrib;
  char regexppath [256];
  char temppath [256];
  
                    /* Unresolved local var: int ret@[DW_OP_reg3(EBX)]
                       Unresolved local var: int count@[???]
                       Unresolved local var: char * p@[???] */
  pcVar7 = strrchr(fullName,0x2f);
  iVar11 = 0;
  if (pcVar7 != (char *)0x0) {
    memcpy(temppath,fullName,(int)pcVar7 - (int)fullName);
    temppath[(int)pcVar7 - (int)fullName] = '\0';
    strcpy(regexppath,pcVar7 + 1);
    uVar6._0_1_ = '*';
    uVar6._1_1_ = '\0';
    if (regexppath[0] != '\0') {
      uVar6 = regexppath._0_2_;
    }
    regexppath._0_2_ = uVar6;
    iVar8 = vm_packedFsFetch(temppath,&pCatalog,&size,&attrib);
    iVar11 = 0;
    if (iVar8 != 0) {
      if (((byte)attrib & 1) == 0) {
        pdb_freeMemory(pCatalog,"VM","/home/cc/xxxx/gameshell/src/vm/vm_efun_file.c",0x785);
        iVar11 = 0;
      }
      else {
        iVar8 = vm_getVmMutex();
        iVar11 = 0;
        if (iVar8 != 0) {
          pBuffer_00 = vm_safeAllocateBuffer(size);
          if (pBuffer_00 == (Vm_Buffer_T *)0x0) {
            pdb_freeMemory(pCatalog,"VM","/home/cc/xxxx/gameshell/src/vm/vm_efun_file.c",0x793);
            vm_releaseVmMutex();
            iVar11 = 0;
          }
          else {
            memcpy(pBuffer_00->ptr,pCatalog,size);
            pdb_freeMemory(pCatalog,"VM","/home/cc/xxxx/gameshell/src/vm/vm_efun_file.c",0x79c);
            __env = vm_newCatch(-1,0);
            iVar8 = _setjmp((__jmp_buf_tag *)__env);
            iVar11 = 0;
            if (iVar8 == 0) {
              vm_pushBufferOfPCB(pBuffer_00);
              pCatalog = (Fm_Catalog_T *)pBuffer_00->ptr;
              size = (uint)size >> 6;
              pArray_00 = vm_allocateArray((uint)pCatalog);
              vm_pushArrayOfPCB(pArray_00);
              i = 0;
              if (0 < size) {
                local_238 = 0;
                local_23c = 0;
LAB_080eb3c5:
                pcVar12 = regexppath;
                pcVar7 = pCatalog->name + local_23c;
                do {
                  while( true ) {
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
                    cVar1 = *pcVar7;
                    if ((cVar1 == '\0') && (*pcVar12 == '\0')) goto LAB_080eb597;
                    cVar2 = *pcVar12;
                    if (cVar2 != '?') break;
                    if (cVar1 == '\0') goto LAB_080eb406;
                    pcVar7 = pcVar7 + 1;
                    pcVar12 = pcVar12 + 1;
                  }
                  if (cVar2 < '@') {
                    if (cVar2 == '\0') goto LAB_080eb406;
                    if (cVar2 == '*') {
                      bVar5 = true;
                      if (pcVar12[1] == '\0') goto LAB_080eb408;
                      iVar11 = 0;
                      goto joined_r0x080eb5b4;
                    }
                  }
                  else if ((cVar2 == '\\') && (pcVar12 = pcVar12 + 1, *pcVar12 == '\0'))
                  goto LAB_080eb406;
                  if (*pcVar12 != *pcVar7) goto LAB_080eb406;
                  pcVar12 = pcVar12 + 1;
                  pcVar7 = pcVar7 + 1;
                } while( true );
              }
LAB_080eb42e:
              vm_takeOffValueFromStackOfPCB(1,1);
              iVar11 = 1;
              vm_endCatch();
            }
            vm_releaseVmMutex();
          }
        }
      }
    }
  }
  return iVar11;
joined_r0x080eb5b4:
  if (cVar1 == '\0') goto LAB_080eb406;
  iVar8 = vm_matchString(pcVar12 + 1,pcVar7 + iVar11);
  if (iVar8 != 0) goto LAB_080eb597;
  iVar11 = iVar11 + 1;
  cVar1 = pcVar7[iVar11];
  goto joined_r0x080eb5b4;
LAB_080eb597:
  bVar5 = true;
  goto LAB_080eb408;
LAB_080eb406:
  bVar5 = false;
LAB_080eb408:
  if (bVar5) {
    pcVar7 = pCatalog->name + local_238 + -0x14;
    if (flags == 0) {
      vm_pushStringOfPCB(pcVar7 + 0x14);
    }
    else {
                    /* Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg6(ESI)] */
      pArray_01 = vm_allocateEmptyArray(3);
      vm_pushArrayOfPCB(pArray_01);
      vm_pushStringOfPCB(pcVar7 + 0x14);
      pVVar3 = *pArray_01->pValues;
      pVVar9 = vm_stackTopValueOfPCB();
      pVVar3->type = pVVar9->type;
      pVVar3 = *pArray_01->pValues;
      pVVar9 = vm_stackTopValueOfPCB();
      pVVar3->attrib = pVVar9->attrib;
      pVVar3 = *pArray_01->pValues;
      pVVar9 = vm_stackTopValueOfPCB();
      pVVar3->v = pVVar9->v;
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg2(EDX)] */
      pVVar10 = vm_getCurrentThreadInfo();
      pVVar4 = pVVar10->pPCB;
      pVVar4->stack[pVVar4->sp].type = '\0';
      pVVar4->stack[pVVar4->sp].attrib = '\0';
      pVVar3 = pArray_01->pValues[1];
      pVVar4->sp = pVVar4->sp + 1;
      pVVar3->type = '\x02';
      iVar11 = -2;
      if ((*pcVar7 & 1U) == 0) {
        iVar11 = *(int *)(pcVar7 + 0xc);
      }
      (pArray_01->pValues[1]->v).number = iVar11;
      pArray_01->pValues[2]->type = '\x02';
      pArray_01->pValues[2]->v = *(anon_union_4_11_5000f750_for_v *)(pcVar7 + 0x10);
    }
    vm_addValueIntoArrayOfPCB();
    if ((maxCount != -1) && (maxCount <= (int)pArray_00->size)) goto LAB_080eb42e;
  }
  i = i + 1;
  local_23c = local_23c + 0x40;
  local_238 = local_238 + 0x40;
  if (size <= i) goto LAB_080eb42e;
  goto LAB_080eb3c5;
}


```

<details><summary>汇编</summary>

```asm
0x80eb208: PUSH EBP
0x80eb209: MOV EBP, ESP
0x80eb20b: PUSH EDI
0x80eb20c: PUSH ESI
0x80eb20d: PUSH EBX
0x80eb20e: SUB ESP, 0x234
0x80eb214: PUSH 0x2f
0x80eb216: PUSH EDX
0x80eb217: MOV ESI, EDX
0x80eb219: CALL 0x0804b844
0x80eb21e: ADD ESP, 0x10
0x80eb221: XOR EDX, EDX
0x80eb223: TEST EAX, EAX
0x80eb225: MOV EDI, EAX
0x80eb227: JZ 0x080eb2c8
0x80eb22d: MOV EBX, EAX
0x80eb22f: SUB EBX, ESI
0x80eb231: PUSH ECX
0x80eb232: PUSH EBX
0x80eb233: PUSH ESI
0x80eb234: LEA EAX, [EBP + 0xfffffee8]
0x80eb23a: PUSH EAX
0x80eb23b: CALL 0x0804b7e4
0x80eb240: POP EAX
0x80eb241: POP EDX
0x80eb242: LEA EAX, [EDI + 0x1]
0x80eb245: PUSH EAX
0x80eb246: LEA EAX, [EBP + 0xfffffde8]
0x80eb24c: PUSH EAX
0x80eb24d: MOV byte ptr [EBP + EBX*0x1 + 0xfffffee8], 0x0
0x80eb255: CALL 0x0804bc54
0x80eb25a: ADD ESP, 0x10
0x80eb25d: MOV EAX, 0x2a
0x80eb262: CMP byte ptr [EBP + 0xfffffde8], 0x0
0x80eb269: CMOVNZ AX, word ptr [EBP + 0xfffffde8]
0x80eb271: MOV word ptr [EBP + 0xfffffde8], AX
0x80eb278: LEA EAX, [EBP + 0xfffffde4]
0x80eb27e: PUSH EAX
0x80eb27f: LEA EAX, [EBP + 0xfffffde0]
0x80eb285: PUSH EAX
0x80eb286: LEA EAX, [EBP + 0xfffffddc]
0x80eb28c: PUSH EAX
0x80eb28d: LEA EDX, [EBP + 0xfffffee8]
0x80eb293: PUSH EDX
0x80eb294: CALL 0x080b40c0
0x80eb299: ADD ESP, 0x10
0x80eb29c: XOR EDX, EDX
0x80eb29e: TEST EAX, EAX
0x80eb2a0: JZ 0x080eb2c8
0x80eb2a2: TEST byte ptr [EBP + 0xfffffde4], 0x1
0x80eb2a9: JNZ 0x080eb2d4
0x80eb2ab: PUSH 0x785
0x80eb2b0: PUSH 0x82bc940
0x80eb2b5: PUSH 0x82a376b
0x80eb2ba: PUSH dword ptr [EBP + 0xfffffddc]
0x80eb2c0: CALL 0x080ceb94
0x80eb2c5: XOR EDX, EDX
0x80eb2c7: NOP
0x80eb2c8: LEA ESP, [EBP + -0xc]
0x80eb2cb: POP EBX
0x80eb2cc: POP ESI
0x80eb2cd: MOV EAX, EDX
0x80eb2cf: POP EDI
0x80eb2d0: LEAVE
0x80eb2d1: RET
0x80eb2d4: CALL 0x080b5bec
0x80eb2d9: XOR EDX, EDX
0x80eb2db: TEST EAX, EAX
0x80eb2dd: JZ 0x080eb2c8
0x80eb2df: SUB ESP, 0xc
0x80eb2e2: PUSH dword ptr [EBP + 0xfffffde0]
0x80eb2e8: CALL 0x0809a698
0x80eb2ed: ADD ESP, 0x10
0x80eb2f0: TEST EAX, EAX
0x80eb2f2: MOV dword ptr [EBP + 0xfffffdd4], EAX
0x80eb2f8: JZ 0x080eb571
0x80eb2fe: PUSH EAX
0x80eb2ff: PUSH dword ptr [EBP + 0xfffffde0]
0x80eb305: PUSH dword ptr [EBP + 0xfffffddc]
0x80eb30b: MOV EAX, dword ptr [EBP + 0xfffffdd4]
0x80eb311: PUSH dword ptr [EAX + 0x10]
0x80eb314: CALL 0x0804b7e4
0x80eb319: PUSH 0x79c
0x80eb31e: PUSH 0x82bc940
0x80eb323: PUSH 0x82a376b
0x80eb328: PUSH dword ptr [EBP + 0xfffffddc]
0x80eb32e: CALL 0x080ceb94
0x80eb333: POP ESI
0x80eb334: POP EDI
0x80eb335: PUSH 0x0
0x80eb337: PUSH -0x1
0x80eb339: CALL 0x080aae68
0x80eb33e: ADD ESP, 0x14
0x80eb341: PUSH EAX
0x80eb342: CALL 0x0804bac4
0x80eb347: ADD ESP, 0x10
0x80eb34a: XOR EBX, EBX
0x80eb34c: TEST EAX, EAX
0x80eb34e: JZ 0x080eb35c
0x80eb350: CALL 0x080b5d50
0x80eb355: MOV EDX, EBX
0x80eb357: JMP 0x080eb2c8
0x80eb35c: SUB ESP, 0xc
0x80eb35f: PUSH dword ptr [EBP + 0xfffffdd4]
0x80eb365: CALL 0x0809280c
0x80eb36a: MOV EDX, dword ptr [EBP + 0xfffffdd4]
0x80eb370: MOV EAX, dword ptr [EDX + 0x10]
0x80eb373: SHR dword ptr [EBP + 0xfffffde0], 0x6
0x80eb37a: MOV dword ptr [ESP], 0x20
0x80eb381: MOV dword ptr [EBP + 0xfffffddc], EAX
0x80eb387: CALL 0x0809adf4
0x80eb38c: MOV dword ptr [ESP], EAX
0x80eb38f: MOV dword ptr [EBP + 0xfffffdd8], EAX
0x80eb395: CALL 0x0809279c
0x80eb39a: XOR EAX, EAX
0x80eb39c: ADD ESP, 0x10
0x80eb39f: CMP EAX, dword ptr [EBP + 0xfffffde0]
0x80eb3a5: MOV dword ptr [EBP + 0xfffffdd0], 0x0
0x80eb3af: JGE 0x080eb42e
0x80eb3b1: MOV dword ptr [EBP + 0xfffffdcc], 0x0
0x80eb3bb: MOV dword ptr [EBP + 0xfffffdc8], 0x0
0x80eb3c5: MOV EAX, dword ptr [EBP + 0xfffffdc8]
0x80eb3cb: ADD EAX, dword ptr [EBP + 0xfffffddc]
0x80eb3d1: LEA ESI, [EBP + 0xfffffde8]
0x80eb3d7: LEA EBX, [EAX + 0x14]
0x80eb3da: MOV DL, byte ptr [EBX]
0x80eb3dc: TEST DL, DL
0x80eb3de: JNZ 0x080eb3e9
0x80eb3e0: CMP byte ptr [ESI], 0x0
0x80eb3e3: JZ 0x080eb597
0x80eb3e9: MOVSX EAX, byte ptr [ESI]
0x80eb3ec: CMP EAX, 0x3f
0x80eb3ef: JZ 0x080eb460
0x80eb3f1: JG 0x080eb450
0x80eb3f3: TEST EAX, EAX
0x80eb3f5: JZ 0x080eb406
0x80eb3f7: CMP EAX, 0x2a
0x80eb3fa: JZ 0x080eb5a1
0x80eb400: MOV AL, byte ptr [EBX]
0x80eb402: CMP byte ptr [ESI], AL
0x80eb404: JZ 0x080eb44c
0x80eb406: XOR EAX, EAX
0x80eb408: TEST EAX, EAX
0x80eb40a: JNZ 0x080eb46b
0x80eb40c: INC dword ptr [EBP + 0xfffffdd0]
0x80eb412: MOV EDX, dword ptr [EBP + 0xfffffdd0]
0x80eb418: ADD dword ptr [EBP + 0xfffffdc8], 0x40
0x80eb41f: ADD dword ptr [EBP + 0xfffffdcc], 0x40
0x80eb426: CMP EDX, dword ptr [EBP + 0xfffffde0]
0x80eb42c: JL 0x080eb3c5
0x80eb42e: SUB ESP, 0x8
0x80eb431: PUSH 0x1
0x80eb433: PUSH 0x1
0x80eb435: CALL 0x08092b04
0x80eb43a: MOV EBX, 0x1
0x80eb43f: CALL 0x080aae54
0x80eb444: ADD ESP, 0x10
0x80eb447: JMP 0x080eb350
0x80eb44c: INC ESI
0x80eb44d: INC EBX
0x80eb44e: JMP 0x080eb3da
0x80eb450: CMP EAX, 0x5c
0x80eb453: JNZ 0x080eb400
0x80eb455: INC ESI
0x80eb456: CMP byte ptr [ESI], 0x0
0x80eb459: JNZ 0x080eb400
0x80eb45b: JMP 0x080eb406
0x80eb460: TEST DL, DL
0x80eb462: JZ 0x080eb406
0x80eb464: INC EBX
0x80eb465: INC ESI
0x80eb466: JMP 0x080eb3da
0x80eb46b: MOV EDI, dword ptr [EBP + 0xfffffdcc]
0x80eb471: MOV EBX, dword ptr [EBP + 0x8]
0x80eb474: ADD EDI, dword ptr [EBP + 0xfffffddc]
0x80eb47a: TEST EBX, EBX
0x80eb47c: JZ 0x080eb560
0x80eb482: SUB ESP, 0xc
0x80eb485: PUSH 0x3
0x80eb487: CALL 0x0809b1d8
0x80eb48c: MOV ESI, EAX
0x80eb48e: MOV dword ptr [ESP], EAX
0x80eb491: CALL 0x0809279c
0x80eb496: LEA EAX, [EDI + 0x14]
0x80eb499: MOV dword ptr [ESP], EAX
0x80eb49c: CALL 0x080925b0
0x80eb4a1: ADD ESP, 0x10
0x80eb4a4: MOV EAX, dword ptr [ESI + 0xc]
0x80eb4a7: MOV EBX, dword ptr [EAX]
0x80eb4a9: CALL 0x08092c0c
0x80eb4ae: MOV AL, byte ptr [EAX + 0x4]
0x80eb4b1: MOV byte ptr [EBX + 0x4], AL
0x80eb4b4: MOV EAX, dword ptr [ESI + 0xc]
0x80eb4b7: MOV EBX, dword ptr [EAX]
0x80eb4b9: CALL 0x08092c0c
0x80eb4be: MOV AL, byte ptr [EAX + 0x5]
0x80eb4c1: MOV byte ptr [EBX + 0x5], AL
0x80eb4c4: MOV EAX, dword ptr [ESI + 0xc]
0x80eb4c7: MOV EBX, dword ptr [EAX]
0x80eb4c9: CALL 0x08092c0c
0x80eb4ce: MOV EAX, dword ptr [EAX + 0x8]
0x80eb4d1: MOV dword ptr [EBX + 0x8], EAX
0x80eb4d4: CALL 0x080a3678
0x80eb4d9: MOV EDX, dword ptr [EAX + 0x4]
0x80eb4dc: MOV EAX, dword ptr [EDX + 0x78]
0x80eb4df: MOV ECX, dword ptr [EDX + 0x80]
0x80eb4e5: LEA EAX, [EAX + EAX*0x2]
0x80eb4e8: MOV byte ptr [ECX + EAX*0x8 + 0x4], 0x0
0x80eb4ed: MOV EAX, dword ptr [EDX + 0x78]
0x80eb4f0: MOV ECX, dword ptr [EDX + 0x80]
0x80eb4f6: LEA EAX, [EAX + EAX*0x2]
0x80eb4f9: MOV byte ptr [ECX + EAX*0x8 + 0x5], 0x0
0x80eb4fe: MOV EAX, dword ptr [ESI + 0xc]
0x80eb501: MOV EAX, dword ptr [EAX + 0x4]
0x80eb504: INC dword ptr [EDX + 0x78]
0x80eb507: MOV byte ptr [EAX + 0x4], 0x2
0x80eb50b: MOV EAX, dword ptr [ESI + 0xc]
0x80eb50e: TEST byte ptr [EDI], 0x1
0x80eb511: MOV EAX, dword ptr [EAX + 0x4]
0x80eb514: MOV EDX, 0xfffffffe
0x80eb519: JNZ 0x080eb51e
0x80eb51b: MOV EDX, dword ptr [EDI + 0xc]
0x80eb51e: MOV dword ptr [EAX + 0x8], EDX
0x80eb521: MOV EAX, dword ptr [ESI + 0xc]
0x80eb524: MOV EAX, dword ptr [EAX + 0x8]
0x80eb527: MOV byte ptr [EAX + 0x4], 0x2
0x80eb52b: MOV EAX, dword ptr [ESI + 0xc]
0x80eb52e: MOV EDX, dword ptr [EAX + 0x8]
0x80eb531: MOV EAX, dword ptr [EDI + 0x10]
0x80eb534: MOV dword ptr [EDX + 0x8], EAX
0x80eb537: CALL 0x080923fc
0x80eb53c: CMP dword ptr [EBP + 0xc], -0x1
0x80eb540: JZ 0x080eb40c
0x80eb546: MOV EAX, dword ptr [EBP + 0xc]
0x80eb549: MOV EDX, dword ptr [EBP + 0xfffffdd8]
0x80eb54f: CMP dword ptr [EDX + 0x4], EAX
0x80eb552: JL 0x080eb40c
0x80eb558: JMP 0x080eb42e
0x80eb560: SUB ESP, 0xc
0x80eb563: LEA EAX, [EDI + 0x14]
0x80eb566: PUSH EAX
0x80eb567: CALL 0x080925b0
0x80eb56c: ADD ESP, 0x10
0x80eb56f: JMP 0x080eb537
0x80eb571: PUSH 0x793
0x80eb576: PUSH 0x82bc940
0x80eb57b: PUSH 0x82a376b
0x80eb580: PUSH dword ptr [EBP + 0xfffffddc]
0x80eb586: CALL 0x080ceb94
0x80eb58b: CALL 0x080b5d50
0x80eb590: XOR EDX, EDX
0x80eb592: JMP 0x080eb2c8
0x80eb597: MOV EAX, 0x1
0x80eb59c: JMP 0x080eb408
0x80eb5a1: INC ESI
0x80eb5a2: CMP byte ptr [ESI], 0x0
0x80eb5a5: MOV EAX, 0x1
0x80eb5aa: JZ 0x080eb408
0x80eb5b0: XOR EDI, EDI
0x80eb5b2: TEST DL, DL
0x80eb5b4: JZ 0x080eb406
0x80eb5ba: LEA EDX, [EBX + EDI*0x1]
0x80eb5bd: MOV EAX, ESI
0x80eb5bf: CALL 0x080eaeb0
0x80eb5c4: TEST EAX, EAX
0x80eb5c6: JNZ 0x080eb597
0x80eb5c8: INC EDI
0x80eb5c9: CMP byte ptr [EBX + EDI*0x1], 0x0
0x80eb5cd: JNZ 0x080eb5ba
0x80eb5cf: JMP 0x080eb406
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initIdentifiers"></a>

## vm_initIdentifiers @ 0x8086678

Evidence: `ev_22f100323748feab029789e6127317398048c5dacbd93c84c0581a7a990ca1e2`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initIdentifiers(void)

{
  uint *puVar1;
  Vm_Ident_Hash_Elem *pVVar2;
  char *s;
  Vm_Ident_Hash_Elem_T *pVVar3;
  Vm_Ident_Hash_Elem *pVVar4;
  ushort uVar5;
  int iVar6;
  Vm_Fun_Entry_T *pVVar7;
  Vm_Ident_Hash_Elem **ppVVar8;
  uint uVar9;
  Vm_Ident_Hash_Elem_T *ihe;
  int iVar10;
  Vm_Program_T *pVVar11;
  int i;
  int h;
  
                    /* Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  currentFile = "__init__";
  currentLine = 0;
  vm_pGlobalProgram = vm_buildProgramHead("global");
  if (vm_pGlobalProgram == (Vm_Program *)0x0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
               "No enough memory to init global identifiers.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x148b);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  iVar6 = lzss_init();
  iVar10 = 0;
  if (iVar6 == 0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Failed to init LZSS module.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1492);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  for (; s = *(char **)((int)&vm_resWords[0].word + iVar10), s != (char *)0x0;
      iVar10 = iVar10 + 0x14) {
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * pNode@[DW_OP_reg1(ECX)] */
    uVar5 = vm_whashstr(s,0x20);
    pVVar11 = vm_pGlobalProgram;
    uVar9 = uVar5 & 0x3ff;
    pVVar2 = vm_pGlobalProgram->ppHashTable[uVar9];
    pVVar4 = (Vm_Ident_Hash_Elem *)((int)&vm_resWords[0].word + iVar10);
    if (pVVar2 == (Vm_Ident_Hash_Elem *)0x0) {
      vm_pGlobalProgram->ppHashHead[uVar9] = pVVar4;
      ppVVar8 = (Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10);
      *(Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10) = pVVar4;
    }
    else {
      *(Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10) = pVVar2->pNext;
      pVVar2->pNext->ppPrev = (Vm_Ident_Hash_Elem **)((int)&vm_resWords[0].pNext + iVar10);
      pVVar2->pNext = pVVar4;
      ppVVar8 = &pVVar2->pNext;
    }
    *(Vm_Ident_Hash_Elem ***)((int)&vm_resWords[0].ppPrev + iVar10) = ppVVar8;
    ppVVar8 = pVVar11->ppHashTable;
    puVar1 = (uint *)((int)&vm_resWords[0].token + iVar10);
    *puVar1 = *puVar1 | 0x80000000;
    ppVVar8[uVar9] = pVVar4;
  }
  vm_addPredefines();
  vm_efunInit();
  vm_efunDebugInit();
  vm_efunPerformanceProfileInit();
  vm_efunVarInit();
  vm_itableInit();
  vm_itreeInit();
  vm_dpInit();
  vm_resourceEfunInit();
  vm_graphInit();
  vm_socketInit();
  vm_procInit();
  vm_fileInit();
  vm_objectInit();
  vm_pdbInit();
  vm_rpcInit();
  for (iVar6 = 0; pVVar11 = vm_pGlobalProgram, iVar6 < totalEfunModules; iVar6 = iVar6 + 1) {
    (*efunModule[iVar6].pInit)();
  }
                    /* Unresolved local var: int i@[DW_OP_reg2(EDX)] */
  vm_pGlobalVar = (Vm_Global_Var_T *)0x0;
  vm_varTableSize = 0;
  vm_globalVar = 0;
  if (vm_pGlobalProgram->pFunctions == (Vm_Fun_Entry_T *)0x0) {
    pVVar7 = pdb_allocateMemory((int)vm_pGlobalProgram->maxFunctionNo << 3,"VM",
                                "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1549);
  }
  else {
    pVVar7 = pdb_reallocateMemory
                       (vm_pGlobalProgram->pFunctions,(int)vm_pGlobalProgram->maxFunctionNo << 3,
                        "VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x154f);
  }
  pVVar11->pFunctions = pVVar7;
  if (vm_pGlobalProgram->pFunctions == (Vm_Fun_Entry_T *)0x0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
               "No enough memory to build function table.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1553);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  pVVar11 = vm_pGlobalProgram;
  for (iVar6 = (int)vm_efunsCount; iVar6 < pVVar11->maxFunctionNo; iVar6 = iVar6 + 1) {
    pVVar7 = pVVar11->pFunctions;
    pVVar7[iVar6].attrib = '\0';
    pVVar11 = vm_pGlobalProgram;
    pVVar7[iVar6].pFun = (Vm_Fun *)0x0;
    pVVar11->pFunctions[iVar6].consititutionNo = 0;
  }
  vm_efunsCount = pVVar11->maxFunctionNo;
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNext@[DW_OP_reg6(ESI)] */
  i = 0;
  ppVVar8 = pVVar11->ppHashHead;
  do {
    pVVar3 = ppVVar8[i];
    ihe = pVVar3;
    while (pVVar3 != (Vm_Ident_Hash_Elem_T *)0x0) {
      pVVar4 = ihe->pNext;
      iVar6 = vm_buildEfunOffset(ihe,(void *)0x0);
      if (iVar6 == 0) {
        vm_freeIdentInProgram(pVVar11,ihe);
        ppVVar8 = pVVar11->ppHashHead;
      }
      else {
        ppVVar8 = pVVar11->ppHashHead;
        if (pVVar4 == ppVVar8[i]) break;
      }
      ihe = pVVar4;
      pVVar3 = ppVVar8[i];
    }
    i = i + 1;
    if (0x3ff < i) {
      return 1;
    }
  } while( true );
}


```

<details><summary>汇编</summary>

```asm
0x8086678: PUSH EBP
0x8086679: MOV EBP, ESP
0x808667b: PUSH EDI
0x808667c: PUSH ESI
0x808667d: PUSH EBX
0x808667e: SUB ESP, 0x18
0x8086681: PUSH 0x82b7052
0x8086686: MOV dword ptr [0x08661940], 0x82ab094
0x8086690: MOV dword ptr [0x08661924], 0x0
0x808669a: CALL 0x0808651c
0x808669f: ADD ESP, 0x10
0x80866a2: TEST EAX, EAX
0x80866a4: MOV [0x0847aad0], EAX
0x80866a9: JNZ 0x080866dc
0x80866ab: SUB ESP, 0x8
0x80866ae: PUSH 0x148b
0x80866b3: PUSH 0x82ab14c
0x80866b8: PUSH 0x82ab7b8
0x80866bd: PUSH 0x82a825c
0x80866c2: PUSH 0x4
0x80866c4: PUSH 0x2
0x80866c6: CALL 0x080b61c4
0x80866cb: ADD ESP, 0x14
0x80866ce: PUSH 0x0
0x80866d0: CALL 0x080a3980
0x80866d5: ADD ESP, 0x10
0x80866d8: JMP 0x080866d8
0x80866dc: CALL 0x080cb300
0x80866e1: XOR ESI, ESI
0x80866e3: TEST EAX, EAX
0x80866e5: JNZ 0x08086753
0x80866e7: JMP 0x0808692d
0x80866ec: SUB ESP, 0x8
0x80866ef: PUSH 0x20
0x80866f1: PUSH EAX
0x80866f2: CALL 0x08089dc8
0x80866f7: MOV EDX, EAX
0x80866f9: MOV EDI, dword ptr [0x0847aad0]
0x80866ff: AND EDX, 0x3ff
0x8086705: MOV EAX, dword ptr [EDI + 0x54]
0x8086708: MOV ECX, dword ptr [EAX + EDX*0x4]
0x808670b: ADD ESP, 0x10
0x808670e: TEST ECX, ECX
0x8086710: LEA EBX, [ESI + 0x83cb8c0]
0x8086716: MOV dword ptr [EBP + -0x10], EDX
0x8086719: JZ 0x080867c1
0x808671f: MOV EAX, dword ptr [ECX + 0xc]
0x8086722: MOV dword ptr [ESI + 0x83cb8cc], EAX
0x8086728: MOV EDX, dword ptr [ECX + 0xc]
0x808672b: LEA EAX, [EBX + 0xc]
0x808672e: MOV dword ptr [EDX + 0x10], EAX
0x8086731: MOV dword ptr [ECX + 0xc], EBX
0x8086734: LEA EAX, [ECX + 0xc]
0x8086737: MOV dword ptr [ESI + 0x83cb8d0], EAX
0x808673d: MOV EAX, dword ptr [EDI + 0x54]
0x8086740: MOV EDX, dword ptr [EBP + -0x10]
0x8086743: OR dword ptr [ESI + 0x83cb8c4], 0x80000000
0x808674d: MOV dword ptr [EAX + EDX*0x4], EBX
0x8086750: ADD ESI, 0x14
0x8086753: MOV EAX, dword ptr [ESI + 0x83cb8c0]
0x8086759: TEST EAX, EAX
0x808675b: JNZ 0x080866ec
0x808675d: CALL 0x08080448
0x8086762: XOR EBX, EBX
0x8086764: CALL 0x080d41fc
0x8086769: CALL 0x080fb104
0x808676e: CALL 0x080fb778
0x8086773: CALL 0x080f960c
0x8086778: CALL 0x080fb988
0x808677d: CALL 0x080fd238
0x8086782: CALL 0x080df414
0x8086787: CALL 0x080c2e94
0x808678c: CALL 0x080e2578
0x8086791: CALL 0x080e29f0
0x8086796: CALL 0x080ec7bc
0x808679b: CALL 0x080e8c54
0x80867a0: CALL 0x080f4134
0x80867a5: CALL 0x080ebcb4
0x80867aa: CALL 0x080f3474
0x80867af: CMP EBX, dword ptr [0x0847ace4]
0x80867b5: JGE 0x080867d8
0x80867b7: CALL dword ptr [EBX*0x8 + 0x847f9c0]
0x80867be: INC EBX
0x80867bf: JMP 0x080867af
0x80867c1: MOV EAX, dword ptr [EDI + 0x58]
0x80867c4: MOV EDX, dword ptr [EBP + -0x10]
0x80867c7: MOV dword ptr [EAX + EDX*0x4], EBX
0x80867ca: LEA EAX, [EBX + 0xc]
0x80867cd: MOV dword ptr [ESI + 0x83cb8cc], EBX
0x80867d3: JMP 0x08086737
0x80867d8: MOV EBX, dword ptr [0x0847aad0]
0x80867de: MOV ESI, dword ptr [EBX + 0x2c]
0x80867e1: TEST ESI, ESI
0x80867e3: MOV dword ptr [0x0847aacc], 0x0
0x80867ed: MOV dword ptr [0x0847aac8], 0x0
0x80867f7: MOV dword ptr [0x0847aac4], 0x0
0x8086801: JNZ 0x08086903
0x8086807: PUSH 0x1549
0x808680c: PUSH 0x82ab14c
0x8086811: PUSH 0x82a376b
0x8086816: MOVSX EAX, word ptr [EBX + 0x10]
0x808681a: SHL EAX, 0x3
0x808681d: PUSH EAX
0x808681e: CALL 0x080ce784
0x8086823: ADD ESP, 0x10
0x8086826: MOV dword ptr [EBX + 0x2c], EAX
0x8086829: MOV EDI, dword ptr [0x0847aad0]
0x808682f: MOV ECX, dword ptr [EDI + 0x2c]
0x8086832: TEST ECX, ECX
0x8086834: JZ 0x080868d4
0x808683a: MOVSX EDX, word ptr [0x0847ace0]
0x8086841: JMP 0x08086864
0x8086844: MOV EAX, dword ptr [EDI + 0x2c]
0x8086847: MOV byte ptr [EAX + EDX*0x8 + 0x6], 0x0
0x808684c: MOV EDI, dword ptr [0x0847aad0]
0x8086852: MOV dword ptr [EAX + EDX*0x8], 0x0
0x8086859: MOV EAX, dword ptr [EDI + 0x2c]
0x808685c: MOV word ptr [EAX + EDX*0x8 + 0x4], 0x0
0x8086863: INC EDX
0x8086864: MOVSX EAX, word ptr [EDI + 0x10]
0x8086868: CMP EAX, EDX
0x808686a: JG 0x08086844
0x808686c: MOV AX, word ptr [EDI + 0x10]
0x8086870: MOV [0x0847ace0], AX
0x8086876: MOV dword ptr [EBP + -0x14], 0x0
0x808687d: MOV EAX, dword ptr [EDI + 0x58]
0x8086880: MOV EDX, dword ptr [EBP + -0x14]
0x8086883: MOV EBX, dword ptr [EAX + EDX*0x4]
0x8086886: TEST EBX, EBX
0x8086888: JMP 0x080868a5
0x808688c: MOV EAX, dword ptr [EDI + 0x58]
0x808688f: MOV EDX, dword ptr [EBP + -0x14]
0x8086892: CMP ESI, dword ptr [EAX + EDX*0x4]
0x8086895: MOV EBX, ESI
0x8086897: JZ 0x0808695c
0x808689d: MOV EDX, dword ptr [EBP + -0x14]
0x80868a0: MOV EDX, dword ptr [EAX + EDX*0x4]
0x80868a3: TEST EDX, EDX
0x80868a5: JZ 0x0808695c
0x80868ab: SUB ESP, 0x8
0x80868ae: MOV ESI, dword ptr [EBX + 0xc]
0x80868b1: PUSH 0x0
0x80868b3: PUSH EBX
0x80868b4: CALL 0x0808761c
0x80868b9: ADD ESP, 0x10
0x80868bc: TEST EAX, EAX
0x80868be: JNZ 0x0808688c
0x80868c0: SUB ESP, 0x8
0x80868c3: PUSH EBX
0x80868c4: PUSH EDI
0x80868c5: CALL 0x080856d8
0x80868ca: MOV EBX, ESI
0x80868cc: ADD ESP, 0x10
0x80868cf: MOV EAX, dword ptr [EDI + 0x58]
0x80868d2: JMP 0x0808689d
0x80868d4: SUB ESP, 0x8
0x80868d7: PUSH 0x1553
0x80868dc: PUSH 0x82ab14c
0x80868e1: PUSH 0x82ab78c
0x80868e6: PUSH 0x82a825c
0x80868eb: PUSH 0x4
0x80868ed: PUSH 0x2
0x80868ef: CALL 0x080b61c4
0x80868f4: ADD ESP, 0x14
0x80868f7: PUSH 0x0
0x80868f9: CALL 0x080a3980
0x80868fe: ADD ESP, 0x10
0x8086901: JMP 0x08086901
0x8086903: SUB ESP, 0xc
0x8086906: PUSH 0x154f
0x808690b: PUSH 0x82ab14c
0x8086910: PUSH 0x82a376b
0x8086915: MOVSX EAX, word ptr [EBX + 0x10]
0x8086919: SHL EAX, 0x3
0x808691c: PUSH EAX
0x808691d: PUSH dword ptr [EBX + 0x2c]
0x8086920: CALL 0x080cf588
0x8086925: ADD ESP, 0x20
0x8086928: JMP 0x08086826
0x808692d: SUB ESP, 0x8
0x8086930: PUSH 0x1492
0x8086935: PUSH 0x82ab14c
0x808693a: PUSH 0x82ab09d
0x808693f: PUSH 0x82a825c
0x8086944: PUSH 0x4
0x8086946: PUSH 0x2
0x8086948: CALL 0x080b61c4
0x808694d: ADD ESP, 0x14
0x8086950: PUSH 0x0
0x8086952: CALL 0x080a3980
0x8086957: ADD ESP, 0x10
0x808695a: JMP 0x0808695a
0x808695c: INC dword ptr [EBP + -0x14]
0x808695f: CMP dword ptr [EBP + -0x14], 0x3ff
0x8086966: JLE 0x08086880
0x808696c: LEA ESP, [EBP + -0xc]
0x808696f: POP EBX
0x8086970: POP ESI
0x8086971: MOV EAX, 0x1
0x8086976: POP EDI
0x8086977: LEAVE
0x8086978: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_copyIdentsName"></a>

## vm_copyIdentsName @ 0x8087a3c

Evidence: `ev_27d4f096ce0e369419a02e49215d067382b851eec94ea52f0bfbcd31f552e7df`

```c

/* WARNING: Unknown calling convention */

int vm_copyIdentsName(Vm_Ident_Hash_Elem_T *ihe,void *pCookie)

{
  char cVar1;
  undefined4 *puVar2;
  char *__dest;
  void *pvVar3;
  uint uVar4;
  Vm_Ident_Info *pVVar5;
  char *pcVar6;
  char *pName;
  
                    /* Unresolved local var: Vm_Ident_Info_T * pIi@[DW_OP_reg6(ESI)]
                       Unresolved local var: void * * para@[???] */
  uVar4 = ihe->token;
  if ((-1 < (int)uVar4) && ((uVar4 & 0x40000000) == 0)) {
    pVVar5 = &ihe->ii;
    (ihe->ii).oldToken = uVar4;
    while (pVVar5 != (Vm_Ident_Info_T *)0x0) {
      uVar4 = pVVar5->oldToken;
      if ((((uVar4 & 0x30000000) == 0) || ((uVar4 & 0x80c0000) == 0)) || ((uVar4 & 0x2000000) != 0))
      {
        pVVar5 = pVVar5->pNext;
      }
      else {
                    /* Unresolved local var: Vm_Dlt_T * pDlt@[DW_OP_reg3(EBX)] */
        puVar2 = *(undefined4 **)pCookie;
        __dest = *(char **)((int)pCookie + 4);
        strcpy(__dest,ihe->name);
        *puVar2 = __dest;
        puVar2[1] = pVVar5->oldToken;
        puVar2[2] = 0;
        pvVar3 = (pVVar5->v).pInfo;
        puVar2[4] = *(undefined4 *)((int)&pVVar5->v + 4);
        puVar2[3] = pvVar3;
        *(undefined4 **)pCookie = puVar2 + 5;
        uVar4 = 0xffffffff;
        pcVar6 = __dest;
        do {
          if (uVar4 == 0) break;
          uVar4 = uVar4 - 1;
          cVar1 = *pcVar6;
          pcVar6 = pcVar6 + 1;
        } while (cVar1 != '\0');
        pVVar5 = pVVar5->pNext;
        *(char **)((int)pCookie + 4) = __dest + ~uVar4;
      }
    }
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8087a3c: PUSH EBP
0x8087a3d: MOV EBP, ESP
0x8087a3f: PUSH EDI
0x8087a40: PUSH ESI
0x8087a41: PUSH EBX
0x8087a42: SUB ESP, 0xc
0x8087a45: MOV EDX, dword ptr [EBP + 0x8]
0x8087a48: MOV EAX, dword ptr [EDX + 0x4]
0x8087a4b: TEST EAX, EAX
0x8087a4d: JS 0x08087a7f
0x8087a4f: TEST EAX, 0x40000000
0x8087a54: JNZ 0x08087a7f
0x8087a56: MOV ESI, EDX
0x8087a58: ADD ESI, 0x14
0x8087a5b: MOV dword ptr [EDX + 0x20], EAX
0x8087a5e: JZ 0x08087a7f
0x8087a60: MOV EAX, dword ptr [ESI + 0xc]
0x8087a63: TEST EAX, 0x30000000
0x8087a68: JZ 0x08087a78
0x8087a6a: TEST EAX, 0x80c0000
0x8087a6f: JZ 0x08087a78
0x8087a71: TEST EAX, 0x2000000
0x8087a76: JZ 0x08087a8c
0x8087a78: MOV ESI, dword ptr [ESI + 0x14]
0x8087a7b: TEST ESI, ESI
0x8087a7d: JNZ 0x08087a60
0x8087a7f: LEA ESP, [EBP + -0xc]
0x8087a82: POP EBX
0x8087a83: POP ESI
0x8087a84: MOV EAX, 0x1
0x8087a89: POP EDI
0x8087a8a: LEAVE
0x8087a8b: RET
0x8087a8c: MOV EAX, dword ptr [EBP + 0xc]
0x8087a8f: MOV EBX, dword ptr [EAX]
0x8087a91: MOV EDX, dword ptr [EAX + 0x4]
0x8087a94: SUB ESP, 0x8
0x8087a97: MOV EAX, dword ptr [EBP + 0x8]
0x8087a9a: PUSH dword ptr [EAX]
0x8087a9c: PUSH EDX
0x8087a9d: MOV dword ptr [EBP + -0x10], EDX
0x8087aa0: CALL 0x0804bc54
0x8087aa5: MOV EDX, dword ptr [EBP + -0x10]
0x8087aa8: MOV dword ptr [EBX], EDX
0x8087aaa: MOV EAX, dword ptr [ESI + 0xc]
0x8087aad: MOV dword ptr [EBX + 0x4], EAX
0x8087ab0: MOV dword ptr [EBX + 0x8], 0x0
0x8087ab7: MOV EAX, dword ptr [ESI]
0x8087ab9: MOV EDX, dword ptr [ESI + 0x4]
0x8087abc: MOV dword ptr [EBX + 0x10], EDX
0x8087abf: MOV dword ptr [EBX + 0xc], EAX
0x8087ac2: MOV EAX, dword ptr [EBP + 0xc]
0x8087ac5: ADD EBX, 0x14
0x8087ac8: MOV dword ptr [EAX], EBX
0x8087aca: CLD
0x8087acb: XOR EAX, EAX
0x8087acd: MOV ECX, 0xffffffff
0x8087ad2: MOV EDI, dword ptr [EBP + -0x10]
0x8087ad5: SCASB.REPNE ES:EDI
0x8087ad7: NOT ECX
0x8087ad9: MOV ESI, dword ptr [ESI + 0x14]
0x8087adc: ADD ECX, dword ptr [EBP + -0x10]
0x8087adf: MOV EDX, dword ptr [EBP + 0xc]
0x8087ae2: ADD ESP, 0x10
0x8087ae5: TEST ESI, ESI
0x8087ae7: MOV dword ptr [EDX + 0x4], ECX
0x8087aea: JMP 0x08087a7d
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadNumber32ToIntR"></a>

## vm_loadNumber32ToIntR @ 0x80b8b9c

Evidence: `ev_282e4e866ce9d66f64c9e2044b10a8dba0089a2c9be788e4f6a5d1fbbd77a452`

```c

int vm_loadNumber32ToIntR(Vm_Binary_Buffer_T *pInput,int *num)

{
  int iVar1;
  uint uVar2;
  
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 4) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 4) {
      return 0;
    }
  }
  iVar1 = *(int *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 4);
  pInput->len = uVar2 - 4;
  if (num != (int *)0x0) {
    *num = iVar1;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8b9c: PUSH EBP
0x80b8b9d: MOV EBP, ESP
0x80b8b9f: PUSH ESI
0x80b8ba0: PUSH EBX
0x80b8ba1: MOV EBX, dword ptr [EBP + 0x8]
0x80b8ba4: MOV EDX, dword ptr [EBX + 0x8]
0x80b8ba7: CMP EDX, 0x3
0x80b8baa: MOV ESI, dword ptr [EBP + 0xc]
0x80b8bad: JBE 0x080b8bd4
0x80b8baf: MOV EAX, dword ptr [EBX + 0x4]
0x80b8bb2: MOV ECX, dword ptr [EAX]
0x80b8bb4: ADD EAX, 0x4
0x80b8bb7: MOV dword ptr [EBX + 0x4], EAX
0x80b8bba: TEST ESI, ESI
0x80b8bbc: LEA EAX, [EDX + -0x4]
0x80b8bbf: MOV dword ptr [EBX + 0x8], EAX
0x80b8bc2: JZ 0x080b8bc6
0x80b8bc4: MOV dword ptr [ESI], ECX
0x80b8bc6: LEA ESP, [EBP + -0x8]
0x80b8bc9: POP EBX
0x80b8bca: MOV EAX, 0x1
0x80b8bcf: POP ESI
0x80b8bd0: LEAVE
0x80b8bd1: RET
0x80b8bd4: SUB ESP, 0xc
0x80b8bd7: PUSH EBX
0x80b8bd8: CALL 0x080b88a0
0x80b8bdd: MOV EDX, dword ptr [EBX + 0x8]
0x80b8be0: ADD ESP, 0x10
0x80b8be3: XOR EAX, EAX
0x80b8be5: CMP EDX, 0x3
0x80b8be8: JA 0x080b8baf
0x80b8bea: LEA ESP, [EBP + -0x8]
0x80b8bed: POP EBX
0x80b8bee: POP ESI
0x80b8bef: LEAVE
0x80b8bf0: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_countIdents"></a>

## vm_countIdents @ 0x80879d4

Evidence: `ev_306524523f9592c0f501cacdae774ec5f9c9df0f55ced600d747da959a48d9e8`

```c

int vm_countIdents(Vm_Ident_Hash_Elem_T *ihe,void *pCookie)

{
  char cVar1;
  uint uVar2;
  Vm_Ident_Info *pVVar3;
  char *pcVar4;
  
                    /* Unresolved local var: Vm_Ident_Info_T * pIi@[DW_OP_reg2(EDX)]
                       Unresolved local var: uint * pNum@[???] */
  uVar2 = ihe->token;
  if ((-1 < (int)uVar2) && ((uVar2 & 0x40000000) == 0)) {
    pVVar3 = &ihe->ii;
    (ihe->ii).oldToken = uVar2;
    for (; pVVar3 != (Vm_Ident_Info_T *)0x0; pVVar3 = pVVar3->pNext) {
      uVar2 = pVVar3->oldToken;
      if ((((uVar2 & 0x30000000) != 0) && ((uVar2 & 0x80c0000) != 0)) && ((uVar2 & 0x2000000) == 0))
      {
        *(int *)pCookie = *(int *)pCookie + 1;
        uVar2 = 0xffffffff;
        pcVar4 = ihe->name;
        do {
          if (uVar2 == 0) break;
          uVar2 = uVar2 - 1;
          cVar1 = *pcVar4;
          pcVar4 = pcVar4 + 1;
        } while (cVar1 != '\0');
        *(int *)((int)pCookie + 4) = *(int *)((int)pCookie + 4) + ~uVar2;
      }
    }
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80879d4: PUSH EBP
0x80879d5: MOV EBP, ESP
0x80879d7: PUSH EDI
0x80879d8: PUSH ESI
0x80879d9: PUSH EBX
0x80879da: PUSH EBX
0x80879db: MOV EBX, dword ptr [EBP + 0x8]
0x80879de: MOV EAX, dword ptr [EBX + 0x4]
0x80879e1: TEST EAX, EAX
0x80879e3: MOV ESI, dword ptr [EBP + 0xc]
0x80879e6: JS 0x08087a2f
0x80879e8: TEST EAX, 0x40000000
0x80879ed: JNZ 0x08087a2f
0x80879ef: MOV EDX, EBX
0x80879f1: ADD EDX, 0x14
0x80879f4: MOV dword ptr [EBX + 0x20], EAX
0x80879f7: JZ 0x08087a2f
0x80879f9: CLD
0x80879fa: MOV ESI, ESI
0x80879fc: MOV EAX, dword ptr [EDX + 0xc]
0x80879ff: TEST EAX, 0x30000000
0x8087a04: JZ 0x08087a28
0x8087a06: TEST EAX, 0x80c0000
0x8087a0b: JZ 0x08087a28
0x8087a0d: TEST EAX, 0x2000000
0x8087a12: JNZ 0x08087a28
0x8087a14: INC dword ptr [ESI]
0x8087a16: MOV EDI, dword ptr [EBX]
0x8087a18: MOV ECX, 0xffffffff
0x8087a1d: XOR EAX, EAX
0x8087a1f: SCASB.REPNE ES:EDI
0x8087a21: NOT ECX
0x8087a23: ADD dword ptr [ESI + 0x4], ECX
0x8087a26: MOV ESI, ESI
0x8087a28: MOV EDX, dword ptr [EDX + 0x14]
0x8087a2b: TEST EDX, EDX
0x8087a2d: JNZ 0x080879fc
0x8087a2f: POP ECX
0x8087a30: POP EBX
0x8087a31: POP ESI
0x8087a32: MOV EAX, 0x1
0x8087a37: POP EDI
0x8087a38: LEAVE
0x8087a39: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadString"></a>

## vm_loadString @ 0x80b8dc8

Evidence: `ev_350b1ce0544fc3fedbf29319e35770e7207764d9db261b94d05c9700049c03de`

```c

int vm_loadString(Vm_Binary_Buffer_T *pInput,char **ppStr)

{
  int iVar1;
  char *__dest;
  uint uVar2;
  uint len;
  void *buf;
  char *str;
  int size;
  
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 4) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 4) {
      return 0;
    }
  }
  iVar1 = *(int *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 4);
  pInput->len = uVar2 - 4;
  if (&stack0x00000000 != (undefined1 *)0x14) {
    size = iVar1;
  }
  if (size < 1) {
    *ppStr = (char *)0x0;
  }
  else {
    __dest = vm_scratchLargeAlloc(size);
    if ((uint)size < 0xfa1) {
      if ((pInput->len < size) && (vm_fetchBinaryBufferData(pInput), pInput->len < size)) {
LAB_080b8ec1:
        vm_scratchFree(__dest);
        return 0;
      }
      memcpy(__dest,pInput->p,size);
      pInput->p = pInput->p + size;
      pInput->len = pInput->len - size;
    }
    else {
      memcpy(__dest,pInput->p,pInput->len);
      iVar1 = pInput->len;
      len = size - iVar1;
      pInput->p = pInput->data;
      pInput->len = 0;
      uVar2 = vm_read(__dest + iVar1,len,pInput->fd);
      if (uVar2 != len) goto LAB_080b8ec1;
    }
    if (&stack0x00000000 == (undefined1 *)0x18) {
      vm_scratchFree(__dest);
      __dest = str;
    }
    str = __dest;
    str[size + -1] = '\0';
    if (ppStr == (char **)0x0) {
      vm_scratchFree(str);
    }
    else {
      *ppStr = str;
    }
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8dc8: PUSH EBP
0x80b8dc9: MOV EBP, ESP
0x80b8dcb: PUSH EDI
0x80b8dcc: PUSH ESI
0x80b8dcd: PUSH EBX
0x80b8dce: SUB ESP, 0xc
0x80b8dd1: MOV ESI, dword ptr [EBP + 0x8]
0x80b8dd4: MOV EAX, dword ptr [ESI + 0x8]
0x80b8dd7: CMP EAX, 0x3
0x80b8dda: LEA EBX, [EBP + -0x10]
0x80b8ddd: MOV EDX, EAX
0x80b8ddf: JBE 0x080b8e88
0x80b8de5: MOV EAX, dword ptr [ESI + 0x4]
0x80b8de8: MOV ECX, dword ptr [EAX]
0x80b8dea: ADD EAX, 0x4
0x80b8ded: MOV dword ptr [ESI + 0x4], EAX
0x80b8df0: TEST EBX, EBX
0x80b8df2: LEA EAX, [EDX + -0x4]
0x80b8df5: MOV dword ptr [ESI + 0x8], EAX
0x80b8df8: JZ 0x080b8dfc
0x80b8dfa: MOV dword ptr [EBX], ECX
0x80b8dfc: MOV EBX, dword ptr [EBP + -0x10]
0x80b8dff: TEST EBX, EBX
0x80b8e01: JLE 0x080b8e7c
0x80b8e03: SUB ESP, 0xc
0x80b8e06: PUSH EBX
0x80b8e07: CALL 0x08089630
0x80b8e0c: ADD ESP, 0x10
0x80b8e0f: CMP EBX, 0xfa0
0x80b8e15: MOV dword ptr [EBP + -0x18], EAX
0x80b8e18: JA 0x080b8edc
0x80b8e1e: CMP dword ptr [ESI + 0x8], EBX
0x80b8e21: JL 0x080b8eac
0x80b8e27: PUSH EDX
0x80b8e28: PUSH EBX
0x80b8e29: PUSH dword ptr [ESI + 0x4]
0x80b8e2c: PUSH dword ptr [EBP + -0x18]
0x80b8e2f: CALL 0x0804b7e4
0x80b8e34: ADD ESP, 0x10
0x80b8e37: ADD dword ptr [ESI + 0x4], EBX
0x80b8e3a: SUB dword ptr [ESI + 0x8], EBX
0x80b8e3d: LEA EDX, [EBP + -0x14]
0x80b8e40: TEST EDX, EDX
0x80b8e42: JZ 0x080b8f1c
0x80b8e48: MOV EAX, dword ptr [EBP + -0x18]
0x80b8e4b: MOV dword ptr [EBP + -0x14], EAX
0x80b8e4e: MOV EAX, dword ptr [EBP + -0x10]
0x80b8e51: MOV EDX, dword ptr [EBP + -0x14]
0x80b8e54: MOV byte ptr [EDX + EAX*0x1 + -0x1], 0x0
0x80b8e59: MOV EAX, dword ptr [EBP + 0xc]
0x80b8e5c: TEST EAX, EAX
0x80b8e5e: JZ 0x080b8f2f
0x80b8e64: MOV EAX, dword ptr [EBP + -0x14]
0x80b8e67: MOV EDX, dword ptr [EBP + 0xc]
0x80b8e6a: MOV dword ptr [EDX], EAX
0x80b8e6c: LEA ESP, [EBP + -0xc]
0x80b8e6f: POP EBX
0x80b8e70: POP ESI
0x80b8e71: MOV EAX, 0x1
0x80b8e76: POP EDI
0x80b8e77: LEAVE
0x80b8e78: RET
0x80b8e7c: MOV EAX, dword ptr [EBP + 0xc]
0x80b8e7f: MOV dword ptr [EAX], 0x0
0x80b8e85: JMP 0x080b8e6c
0x80b8e88: SUB ESP, 0xc
0x80b8e8b: PUSH ESI
0x80b8e8c: CALL 0x080b88a0
0x80b8e91: MOV EDX, dword ptr [ESI + 0x8]
0x80b8e94: ADD ESP, 0x10
0x80b8e97: XOR EAX, EAX
0x80b8e99: CMP EDX, 0x3
0x80b8e9c: JA 0x080b8de5
0x80b8ea2: LEA ESP, [EBP + -0xc]
0x80b8ea5: POP EBX
0x80b8ea6: POP ESI
0x80b8ea7: POP EDI
0x80b8ea8: LEAVE
0x80b8ea9: RET
0x80b8eac: SUB ESP, 0xc
0x80b8eaf: PUSH ESI
0x80b8eb0: CALL 0x080b88a0
0x80b8eb5: ADD ESP, 0x10
0x80b8eb8: CMP dword ptr [ESI + 0x8], EBX
0x80b8ebb: JGE 0x080b8e27
0x80b8ec1: SUB ESP, 0xc
0x80b8ec4: PUSH dword ptr [EBP + -0x18]
0x80b8ec7: CALL 0x080891d4
0x80b8ecc: ADD ESP, 0x10
0x80b8ecf: LEA ESP, [EBP + -0xc]
0x80b8ed2: POP EBX
0x80b8ed3: POP ESI
0x80b8ed4: XOR EAX, EAX
0x80b8ed6: POP EDI
0x80b8ed7: LEAVE
0x80b8ed8: RET
0x80b8edc: PUSH EAX
0x80b8edd: PUSH dword ptr [ESI + 0x8]
0x80b8ee0: PUSH dword ptr [ESI + 0x4]
0x80b8ee3: PUSH dword ptr [EBP + -0x18]
0x80b8ee6: CALL 0x0804b7e4
0x80b8eeb: MOV EDI, EBX
0x80b8eed: MOV EAX, dword ptr [ESI + 0x8]
0x80b8ef0: MOV EDX, dword ptr [EBP + -0x18]
0x80b8ef3: ADD ESP, 0xc
0x80b8ef6: ADD EDX, EAX
0x80b8ef8: SUB EDI, EAX
0x80b8efa: LEA EAX, [ESI + 0xc]
0x80b8efd: MOV dword ptr [ESI + 0x4], EAX
0x80b8f00: MOV dword ptr [ESI + 0x8], 0x0
0x80b8f07: PUSH dword ptr [ESI]
0x80b8f09: PUSH EDI
0x80b8f0a: PUSH EDX
0x80b8f0b: CALL 0x080a110c
0x80b8f10: ADD ESP, 0x10
0x80b8f13: CMP EAX, EDI
0x80b8f15: JNZ 0x080b8ec1
0x80b8f17: JMP 0x080b8e3d
0x80b8f1c: SUB ESP, 0xc
0x80b8f1f: PUSH dword ptr [EBP + -0x18]
0x80b8f22: CALL 0x080891d4
0x80b8f27: ADD ESP, 0x10
0x80b8f2a: JMP 0x080b8e4e
0x80b8f2f: SUB ESP, 0xc
0x80b8f32: PUSH dword ptr [EBP + -0x14]
0x80b8f35: CALL 0x080891d4
0x80b8f3a: ADD ESP, 0x10
0x80b8f3d: JMP 0x080b8e6c
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initEfun"></a>

## vm_initEfun @ 0x804c960

Evidence: `ev_3cf4b7b4c241ad44b0d360927454293628555da474b2ebb560fa890a525285fb`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initEfun(void)

{
  Vm_Defn_T *pVVar1;
  
  vm_mapInit();
  vm_magicInit();
  vm_gameInit();
  vm_kbdpwdInit();
  vm_AcMatchInit();
  pVVar1 = vm_lookupDefine("DRIVER_USE_APEX_PROXY");
  if (pVVar1 == (Vm_Defn_T *)0x0) {
    vm_updateEfuns();
    vm_addQuotedPredefine("MAGIC_PACKAGE","MAGIC_PACKAGE");
    return 1;
  }
  vm_apexInit();
  vm_updateEfuns();
  vm_addQuotedPredefine("MAGIC_PACKAGE","MAGIC_PACKAGE");
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x804c960: PUSH EBP
0x804c961: MOV EBP, ESP
0x804c963: SUB ESP, 0x8
0x804c966: CALL 0x080782d0
0x804c96b: CALL 0x0804cc14
0x804c970: CALL 0x0807c74c
0x804c975: CALL 0x0807e240
0x804c97a: CALL 0x0807e58c
0x804c97f: SUB ESP, 0xc
0x804c982: PUSH 0x82a35a3
0x804c987: CALL 0x0807eab4
0x804c98c: ADD ESP, 0x10
0x804c98f: TEST EAX, EAX
0x804c991: JNZ 0x0804c9b4
0x804c993: CALL 0x0808600c
0x804c998: SUB ESP, 0x8
0x804c99b: PUSH 0x82a35b9
0x804c9a0: PUSH 0x82a35b9
0x804c9a5: CALL 0x080803ac
0x804c9aa: MOV EAX, 0x1
0x804c9af: LEAVE
0x804c9b0: RET
0x804c9b4: CALL 0x0807dff8
0x804c9b9: CALL 0x0808600c
0x804c9be: SUB ESP, 0x8
0x804c9c1: PUSH 0x82a35b9
0x804c9c6: PUSH 0x82a35b9
0x804c9cb: CALL 0x080803ac
0x804c9d0: MOV EAX, 0x1
0x804c9d5: LEAVE
0x804c9d6: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initVerigenEfun"></a>

## vm_initVerigenEfun @ 0x8171630

Evidence: `ev_46c386bff284585bfc1f32e3348bdeb327a6ce7b17d40908470b83ed8584ad20`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initVerigenEfun(void)

{
  vm_verigenInit();
  vm_updateEfuns();
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x8171630: PUSH EBP
0x8171631: MOV EBP, ESP
0x8171633: SUB ESP, 0x8
0x8171636: CALL 0x08177eb0
0x817163b: CALL 0x0808600c
0x8171640: MOV EAX, 0x1
0x8171645: LEAVE
0x8171646: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadVmIndex"></a>

## vm_loadVmIndex @ 0x80b8b44

Evidence: `ev_46fdeb0c10ac16e3a6861136515a5e3f0381cec5fb3d4195a661da88ffac6dcb`

```c

int vm_loadVmIndex(Vm_Binary_Buffer_T *pInput,Vm_Index_T *index)

{
  Vm_Index_T VVar1;
  uint uVar2;
  
                    /* Unresolved local var: Vm_Index_T temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 2) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 2) {
      return 0;
    }
  }
  VVar1 = *(Vm_Index_T *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 2);
  pInput->len = uVar2 - 2;
  if (index != (Vm_Index_T *)0x0) {
    *index = VVar1;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8b44: PUSH EBP
0x80b8b45: MOV EBP, ESP
0x80b8b47: PUSH ESI
0x80b8b48: PUSH EBX
0x80b8b49: MOV EBX, dword ptr [EBP + 0x8]
0x80b8b4c: MOV EDX, dword ptr [EBX + 0x8]
0x80b8b4f: CMP EDX, 0x1
0x80b8b52: MOV ESI, dword ptr [EBP + 0xc]
0x80b8b55: JBE 0x080b8b7c
0x80b8b57: MOV EAX, dword ptr [EBX + 0x4]
0x80b8b5a: MOV CX, word ptr [EAX]
0x80b8b5d: ADD EAX, 0x2
0x80b8b60: MOV dword ptr [EBX + 0x4], EAX
0x80b8b63: TEST ESI, ESI
0x80b8b65: LEA EAX, [EDX + -0x2]
0x80b8b68: MOV dword ptr [EBX + 0x8], EAX
0x80b8b6b: JZ 0x080b8b70
0x80b8b6d: MOV word ptr [ESI], CX
0x80b8b70: LEA ESP, [EBP + -0x8]
0x80b8b73: POP EBX
0x80b8b74: MOV EAX, 0x1
0x80b8b79: POP ESI
0x80b8b7a: LEAVE
0x80b8b7b: RET
0x80b8b7c: SUB ESP, 0xc
0x80b8b7f: PUSH EBX
0x80b8b80: CALL 0x080b88a0
0x80b8b85: MOV EDX, dword ptr [EBX + 0x8]
0x80b8b88: ADD ESP, 0x10
0x80b8b8b: XOR EAX, EAX
0x80b8b8d: CMP EDX, 0x1
0x80b8b90: JA 0x080b8b57
0x80b8b92: LEA ESP, [EBP + -0x8]
0x80b8b95: POP EBX
0x80b8b96: POP ESI
0x80b8b97: LEAVE
0x80b8b98: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initDbMysql"></a>

## vm_initDbMysql @ 0x816f750

Evidence: `ev_480cdf3727c7bab9994611a03a982dcc23aba7ff40b3b7d86dd0704fa7d39423`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initDbMysql(void)

{
  char *pcVar1;
  int iVar2;
  Vm_MySql_View_Link_T *pVVar3;
  int iVar4;
  int iVar5;
  
                    /* Unresolved local var: int i@[???] */
  iVar2 = mysql_thread_safe();
  if (iVar2 == 0) {
    vm_stdprintf("MYSQL is not thread-safe mode.\n");
  }
  else {
    _mysqlConn = pdb_allocateMemory(_mysqlMaxConnection * 0x3c4,"VM",
                                    "/root/temp/gameshell/src/mysql/vm_db_mysql.c",0x45);
    if (_mysqlConn == (Vm_MySql_Conn_Link_T *)0x0) {
      vm_stdprintf("Failed to allocate memory for _mysqlConn.\n");
    }
    else {
      _mysqlView = pdb_allocateMemory(_mysqlMaxViewResult << 4,"VM",
                                      "/root/temp/gameshell/src/mysql/vm_db_mysql.c",0x4e);
      if (_mysqlView != (Vm_MySql_View_Link_T *)0x0) {
        iVar4 = 0;
        iVar2 = 0;
        if (0 < _mysqlMaxConnection) {
          do {
            iVar4 = iVar4 + 1;
            _vm_createMutex((Vm_Mutex_Id_T *)((_mysqlConn->conn).net.last_error + iVar2 + -0x5b));
            pcVar1 = (_mysqlConn->conn).net.last_error + iVar2 + -0x57;
            pcVar1[0] = '\0';
            iVar5 = _mysqlMaxConnection;
            pcVar1[1] = '\0';
            pcVar1[2] = '\0';
            pcVar1[3] = '\0';
            pcVar1 = (_mysqlConn->conn).net.last_error + iVar2 + -0x53;
            pcVar1[0] = '\0';
            pcVar1[1] = '\0';
            pcVar1[2] = '\0';
            pcVar1[3] = '\0';
            iVar2 = iVar2 + 0x3c4;
          } while (iVar4 < iVar5);
        }
        if (0 < _mysqlMaxViewResult) {
          iVar2 = 0;
          do {
            iVar5 = iVar2 + 1;
            pVVar3 = _mysqlView + iVar2;
            pVVar3->mutex = (Vm_Mutex_Id_T_conflict)0x0;
            pVVar3->cno = 0;
            pVVar3->pCurRecord = (void *)0x0;
            pVVar3->pView = (MYSQL_RES *)0x0;
            _vm_createMutex((Vm_Mutex_Id_T *)pVVar3);
            pVVar3 = _mysqlView;
            iVar4 = _mysqlMaxViewResult;
            _mysqlView[iVar2].pCurRecord = (void *)0x0;
            pVVar3[iVar2].pView = (MYSQL_RES *)0x0;
            pVVar3[iVar2].cno = -1;
            iVar2 = iVar5;
          } while (iVar5 < iVar4);
        }
        _dbMysqlActive = 1;
        return 1;
      }
      pdb_freeMemory(_mysqlConn,"VM","/root/temp/gameshell/src/mysql/vm_db_mysql.c",0x51);
      _mysqlConn = (Vm_MySql_Conn_Link_T *)0x0;
      vm_stdprintf("Failed to allocate memory for _mysqlView.\n");
    }
  }
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x816f750: PUSH EBP
0x816f751: MOV EBP, ESP
0x816f753: PUSH ESI
0x816f754: PUSH EBX
0x816f755: SUB ESP, 0x10
0x816f758: CALL 0x0822cc50
0x816f75d: TEST EAX, EAX
0x816f75f: JNZ 0x0816f776
0x816f761: MOV dword ptr [ESP], 0x82cfd00
0x816f768: CALL 0x08097e34
0x816f76d: XOR EAX, EAX
0x816f76f: ADD ESP, 0x10
0x816f772: POP EBX
0x816f773: POP ESI
0x816f774: POP EBP
0x816f775: RET
0x816f776: IMUL EAX, dword ptr [0x083d4e64], 0x3c4
0x816f780: MOV dword ptr [ESP + 0xc], 0x45
0x816f788: MOV dword ptr [ESP + 0x8], 0x82cfcd0
0x816f790: MOV dword ptr [ESP + 0x4], 0x82a376b
0x816f798: MOV dword ptr [ESP], EAX
0x816f79b: CALL 0x080ce784
0x816f7a0: TEST EAX, EAX
0x816f7a2: MOV [0x0861e0bc], EAX
0x816f7a7: JZ 0x0816f8a3
0x816f7ad: MOV EAX, [0x083d4e60]
0x816f7b2: MOV dword ptr [ESP + 0xc], 0x4e
0x816f7ba: MOV dword ptr [ESP + 0x8], 0x82cfcd0
0x816f7c2: MOV dword ptr [ESP + 0x4], 0x82a376b
0x816f7ca: SHL EAX, 0x4
0x816f7cd: MOV dword ptr [ESP], EAX
0x816f7d0: CALL 0x080ce784
0x816f7d5: TEST EAX, EAX
0x816f7d7: MOV [0x0861e0c0], EAX
0x816f7dc: JZ 0x0816f8b6
0x816f7e2: MOV EDX, dword ptr [0x083d4e64]
0x816f7e8: XOR ESI, ESI
0x816f7ea: XOR EBX, EBX
0x816f7ec: TEST EDX, EDX
0x816f7ee: JLE 0x0816f82c
0x816f7f0: MOV EAX, [0x0861e0bc]
0x816f7f5: ADD ESI, 0x1
0x816f7f8: ADD EAX, EBX
0x816f7fa: MOV dword ptr [ESP], EAX
0x816f7fd: CALL 0x080a39d0
0x816f802: MOV EAX, [0x0861e0bc]
0x816f807: MOV dword ptr [EAX + EBX*0x1 + 0x4], 0x0
0x816f80f: MOV EAX, EBX
0x816f811: ADD EBX, 0x3c4
0x816f817: ADD EAX, dword ptr [0x0861e0bc]
0x816f81d: CMP dword ptr [0x083d4e64], ESI
0x816f823: MOV dword ptr [EAX + 0x8], 0x0
0x816f82a: JG 0x0816f7f0
0x816f82c: MOV EAX, [0x083d4e60]
0x816f831: TEST EAX, EAX
0x816f833: JLE 0x0816f88d
0x816f835: XOR ESI, ESI
0x816f837: MOV EBX, ESI
0x816f839: ADD ESI, 0x1
0x816f83c: SHL EBX, 0x4
0x816f83f: MOV EAX, EBX
0x816f841: ADD EAX, dword ptr [0x0861e0c0]
0x816f847: MOV dword ptr [EAX], 0x0
0x816f84d: MOV dword ptr [EAX + 0x4], 0x0
0x816f854: MOV dword ptr [EAX + 0x8], 0x0
0x816f85b: MOV dword ptr [EAX + 0xc], 0x0
0x816f862: MOV dword ptr [ESP], EAX
0x816f865: CALL 0x080a39d0
0x816f86a: ADD EBX, dword ptr [0x0861e0c0]
0x816f870: CMP dword ptr [0x083d4e60], ESI
0x816f876: MOV dword ptr [EBX + 0x8], 0x0
0x816f87d: MOV dword ptr [EBX + 0xc], 0x0
0x816f884: MOV dword ptr [EBX + 0x4], 0xffffffff
0x816f88b: JG 0x0816f837
0x816f88d: MOV dword ptr [0x0861e0c8], 0x1
0x816f897: ADD ESP, 0x10
0x816f89a: MOV EAX, 0x1
0x816f89f: POP EBX
0x816f8a0: POP ESI
0x816f8a1: POP EBP
0x816f8a2: RET
0x816f8a3: MOV dword ptr [ESP], 0x82cfd20
0x816f8aa: CALL 0x08097e34
0x816f8af: XOR EAX, EAX
0x816f8b1: JMP 0x0816f76f
0x816f8b6: MOV EAX, [0x0861e0bc]
0x816f8bb: MOV dword ptr [ESP + 0xc], 0x51
0x816f8c3: MOV dword ptr [ESP + 0x8], 0x82cfcd0
0x816f8cb: MOV dword ptr [ESP + 0x4], 0x82a376b
0x816f8d3: MOV dword ptr [ESP], EAX
0x816f8d6: CALL 0x080ceb94
0x816f8db: MOV dword ptr [0x0861e0bc], 0x0
0x816f8e5: MOV dword ptr [ESP], 0x82cfd4c
0x816f8ec: CALL 0x08097e34
0x816f8f1: XOR EAX, EAX
0x816f8f3: JMP 0x0816f76f
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_updateEfuns"></a>

## vm_updateEfuns @ 0x808600c

Evidence: `ev_7207caf3da50797137c81e701dc99f43fd5d54403b7e8d8d6cc36d973f82013b`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_updateEfuns(void)

{
  Vm_Ident_Hash_Elem_T *pVVar1;
  Vm_Ident_Hash_Elem *pVVar2;
  Vm_Fun_Entry_T *pVVar3;
  Vm_Ident_Hash_Elem **ppVVar4;
  int iVar5;
  Vm_Ident_Hash_Elem_T *ihe;
  Vm_Program_T *pVVar6;
  int i;
  
                    /* Unresolved local var: int i@[DW_OP_reg2(EDX)] */
  pVVar6 = vm_pGlobalProgram;
  if (vm_pGlobalProgram->pFunctions == (Vm_Fun_Entry_T *)0x0) {
    pVVar3 = pdb_allocateMemory((int)vm_pGlobalProgram->maxFunctionNo << 3,"VM",
                                "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1549);
    pVVar6->pFunctions = pVVar3;
  }
  else {
    pVVar3 = pdb_reallocateMemory
                       (vm_pGlobalProgram->pFunctions,(int)vm_pGlobalProgram->maxFunctionNo << 3,
                        "VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x154f);
    pVVar6->pFunctions = pVVar3;
  }
  if (vm_pGlobalProgram->pFunctions == (Vm_Fun_Entry_T *)0x0) {
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
               "No enough memory to build function table.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1553);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  }
  pVVar6 = vm_pGlobalProgram;
  for (iVar5 = (int)vm_efunsCount; iVar5 < pVVar6->maxFunctionNo; iVar5 = iVar5 + 1) {
    pVVar3 = pVVar6->pFunctions;
    pVVar3[iVar5].attrib = '\0';
    pVVar6 = vm_pGlobalProgram;
    pVVar3[iVar5].pFun = (Vm_Fun *)0x0;
    pVVar6->pFunctions[iVar5].consititutionNo = 0;
  }
  vm_efunsCount = pVVar6->maxFunctionNo;
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNext@[DW_OP_reg6(ESI)] */
  i = 0;
  ppVVar4 = pVVar6->ppHashHead;
  do {
    pVVar1 = ppVVar4[i];
    ihe = pVVar1;
    while (pVVar1 != (Vm_Ident_Hash_Elem_T *)0x0) {
      pVVar2 = ihe->pNext;
      iVar5 = vm_buildEfunOffset(ihe,(void *)0x0);
      if (iVar5 == 0) {
        vm_freeIdentInProgram(pVVar6,ihe);
        ppVVar4 = pVVar6->ppHashHead;
      }
      else {
        ppVVar4 = pVVar6->ppHashHead;
        if (pVVar2 == ppVVar4[i]) break;
      }
      ihe = pVVar2;
      pVVar1 = ppVVar4[i];
    }
    i = i + 1;
    if (0x3ff < i) {
      return;
    }
  } while( true );
}


```

<details><summary>汇编</summary>

```asm
0x808600c: PUSH EBP
0x808600d: MOV EBP, ESP
0x808600f: PUSH EDI
0x8086010: PUSH ESI
0x8086011: PUSH EBX
0x8086012: SUB ESP, 0xc
0x8086015: MOV EBX, dword ptr [0x0847aad0]
0x808601b: MOV ESI, dword ptr [EBX + 0x2c]
0x808601e: TEST ESI, ESI
0x8086020: JNZ 0x080860f4
0x8086026: PUSH 0x1549
0x808602b: PUSH 0x82ab14c
0x8086030: PUSH 0x82a376b
0x8086035: MOVSX EAX, word ptr [EBX + 0x10]
0x8086039: SHL EAX, 0x3
0x808603c: PUSH EAX
0x808603d: CALL 0x080ce784
0x8086042: MOV dword ptr [EBX + 0x2c], EAX
0x8086045: ADD ESP, 0x10
0x8086048: MOV EDI, dword ptr [0x0847aad0]
0x808604e: MOV ECX, dword ptr [EDI + 0x2c]
0x8086051: TEST ECX, ECX
0x8086053: JZ 0x08086121
0x8086059: MOVSX EDX, word ptr [0x0847ace0]
0x8086060: JMP 0x08086082
0x8086062: MOV EAX, dword ptr [EDI + 0x2c]
0x8086065: MOV byte ptr [EAX + EDX*0x8 + 0x6], 0x0
0x808606a: MOV EDI, dword ptr [0x0847aad0]
0x8086070: MOV dword ptr [EAX + EDX*0x8], 0x0
0x8086077: MOV EAX, dword ptr [EDI + 0x2c]
0x808607a: MOV word ptr [EAX + EDX*0x8 + 0x4], 0x0
0x8086081: INC EDX
0x8086082: MOVSX EAX, word ptr [EDI + 0x10]
0x8086086: CMP EAX, EDX
0x8086088: JG 0x08086062
0x808608a: MOV AX, word ptr [EDI + 0x10]
0x808608e: MOV [0x0847ace0], AX
0x8086094: MOV dword ptr [EBP + -0x10], 0x0
0x808609b: MOV EAX, dword ptr [EDI + 0x58]
0x808609e: MOV ESI, ESI
0x80860a0: MOV EDX, dword ptr [EBP + -0x10]
0x80860a3: MOV EBX, dword ptr [EAX + EDX*0x4]
0x80860a6: TEST EBX, EBX
0x80860a8: JMP 0x080860c5
0x80860ac: MOV EAX, dword ptr [EDI + 0x58]
0x80860af: MOV EDX, dword ptr [EBP + -0x10]
0x80860b2: CMP ESI, dword ptr [EAX + EDX*0x4]
0x80860b5: MOV EBX, ESI
0x80860b7: JZ 0x08086150
0x80860bd: MOV EDX, dword ptr [EBP + -0x10]
0x80860c0: MOV EDX, dword ptr [EAX + EDX*0x4]
0x80860c3: TEST EDX, EDX
0x80860c5: JZ 0x08086150
0x80860cb: SUB ESP, 0x8
0x80860ce: MOV ESI, dword ptr [EBX + 0xc]
0x80860d1: PUSH 0x0
0x80860d3: PUSH EBX
0x80860d4: CALL 0x0808761c
0x80860d9: ADD ESP, 0x10
0x80860dc: TEST EAX, EAX
0x80860de: JNZ 0x080860ac
0x80860e0: SUB ESP, 0x8
0x80860e3: PUSH EBX
0x80860e4: PUSH EDI
0x80860e5: CALL 0x080856d8
0x80860ea: MOV EBX, ESI
0x80860ec: ADD ESP, 0x10
0x80860ef: MOV EAX, dword ptr [EDI + 0x58]
0x80860f2: JMP 0x080860bd
0x80860f4: SUB ESP, 0xc
0x80860f7: PUSH 0x154f
0x80860fc: PUSH 0x82ab14c
0x8086101: PUSH 0x82a376b
0x8086106: MOVSX EAX, word ptr [EBX + 0x10]
0x808610a: SHL EAX, 0x3
0x808610d: PUSH EAX
0x808610e: PUSH dword ptr [EBX + 0x2c]
0x8086111: CALL 0x080cf588
0x8086116: ADD ESP, 0x20
0x8086119: MOV dword ptr [EBX + 0x2c], EAX
0x808611c: JMP 0x08086048
0x8086121: SUB ESP, 0x8
0x8086124: PUSH 0x1553
0x8086129: PUSH 0x82ab14c
0x808612e: PUSH 0x82ab78c
0x8086133: PUSH 0x82a825c
0x8086138: PUSH 0x4
0x808613a: PUSH 0x2
0x808613c: CALL 0x080b61c4
0x8086141: ADD ESP, 0x14
0x8086144: PUSH 0x0
0x8086146: CALL 0x080a3980
0x808614b: ADD ESP, 0x10
0x808614e: JMP 0x0808614e
0x8086150: INC dword ptr [EBP + -0x10]
0x8086153: CMP dword ptr [EBP + -0x10], 0x3ff
0x808615a: JLE 0x080860a0
0x8086160: LEA ESP, [EBP + -0xc]
0x8086163: POP EBX
0x8086164: POP ESI
0x8086165: POP EDI
0x8086166: LEAVE
0x8086167: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadNumber32"></a>

## vm_loadNumber32 @ 0x80b8bf4

Evidence: `ev_725f935a25e45fd1f376358fb6154c704a5a08faae486db835259ab6ce73f664`

```c

int vm_loadNumber32(Vm_Binary_Buffer_T *pInput,int *num)

{
  int iVar1;
  uint uVar2;
  
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 4) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 4) {
      return 0;
    }
  }
  iVar1 = *(int *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 4);
  pInput->len = uVar2 - 4;
  if (num != (int *)0x0) {
    *num = iVar1;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8bf4: PUSH EBP
0x80b8bf5: MOV EBP, ESP
0x80b8bf7: PUSH ESI
0x80b8bf8: PUSH EBX
0x80b8bf9: MOV EBX, dword ptr [EBP + 0x8]
0x80b8bfc: MOV EDX, dword ptr [EBX + 0x8]
0x80b8bff: CMP EDX, 0x3
0x80b8c02: MOV ESI, dword ptr [EBP + 0xc]
0x80b8c05: JBE 0x080b8c2c
0x80b8c07: MOV EAX, dword ptr [EBX + 0x4]
0x80b8c0a: MOV ECX, dword ptr [EAX]
0x80b8c0c: ADD EAX, 0x4
0x80b8c0f: MOV dword ptr [EBX + 0x4], EAX
0x80b8c12: TEST ESI, ESI
0x80b8c14: LEA EAX, [EDX + -0x4]
0x80b8c17: MOV dword ptr [EBX + 0x8], EAX
0x80b8c1a: JZ 0x080b8c1e
0x80b8c1c: MOV dword ptr [ESI], ECX
0x80b8c1e: LEA ESP, [EBP + -0x8]
0x80b8c21: POP EBX
0x80b8c22: MOV EAX, 0x1
0x80b8c27: POP ESI
0x80b8c28: LEAVE
0x80b8c29: RET
0x80b8c2c: SUB ESP, 0xc
0x80b8c2f: PUSH EBX
0x80b8c30: CALL 0x080b88a0
0x80b8c35: MOV EDX, dword ptr [EBX + 0x8]
0x80b8c38: ADD ESP, 0x10
0x80b8c3b: XOR EAX, EAX
0x80b8c3d: CMP EDX, 0x3
0x80b8c40: JA 0x080b8c07
0x80b8c42: LEA ESP, [EBP + -0x8]
0x80b8c45: POP EBX
0x80b8c46: POP ESI
0x80b8c47: LEAVE
0x80b8c48: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initLanguageEfun"></a>

## vm_initLanguageEfun @ 0x817a270

Evidence: `ev_7a07253866de1f05a3e6441c47e26256781ffda62935a574ea49df39e4273fc0`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initLanguageEfun(void)

{
  vm_languageInit();
  vm_updateEfuns();
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x817a270: PUSH EBP
0x817a271: MOV EBP, ESP
0x817a273: SUB ESP, 0x8
0x817a276: CALL 0x0817b370
0x817a27b: CALL 0x0808600c
0x817a280: MOV EAX, 0x1
0x817a285: LEAVE
0x817a286: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadComment"></a>

## vm_loadComment @ 0x80b8800

Evidence: `ev_803e5760107de0c73e61fb24936c6deba731d532dbc47f36f46b37b258c23502`

```c

/* WARNING: Unknown calling convention */

int vm_loadComment(Vm_Binary_Buffer_T *pInput,char *comment)

{
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8800: PUSH EBP
0x80b8801: MOV EBP, ESP
0x80b8803: MOV EAX, 0x1
0x80b8808: LEAVE
0x80b8809: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_saveInheritPrograms"></a>

## vm_saveInheritPrograms @ 0x8087164

Evidence: `ev_8193669355a4347a34113c3b4841384f62ffab0bf23a100b32bbb58a14ca0dbd`

```c

/* WARNING: Unknown calling convention */

int vm_saveInheritPrograms(int fd)

{
  short sVar1;
  int iVar2;
  undefined4 extraout_ECX;
  int iVar3;
  uchar *puVar4;
  short sVar5;
  short consititutionNo;
  short sVar6;
  bool bVar7;
  bool bVar8;
  Vm_Program_T *pVVar9;
  undefined4 uVar10;
  Vm_Program_T *local_14;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Index_T i@[DW_OP_reg3(EBX)]
                       Unresolved local var: int count@[DW_OP_reg1(ECX)] */
  if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
    local_14 = vm_pGlobalProgram;
  }
  else {
    local_14 = vm_pCurrentProgram;
  }
  iVar3 = 1;
  if (local_14 != vm_pGlobalProgram) {
    vm_saveComment(fd,"INHERITS");
    iVar2 = vm_saveVmIndex(fd,local_14->maxConsititutionNo);
    iVar3 = 0;
    if (iVar2 != 0) {
      sVar5 = local_14->maxConsititutionNo;
      iVar3 = 0;
      if (0 < sVar5) {
        puVar4 = &local_14->pConsititutions->attrib;
        do {
          iVar3 = (iVar3 + 1) - (uint)((*puVar4 & 1) == 0);
          puVar4 = puVar4 + 0xc;
          sVar5 = sVar5 + -1;
        } while (sVar5 != 0);
      }
      iVar2 = vm_saveNumber32(fd,iVar3);
      iVar3 = 0;
      if (iVar2 != 0) {
        sVar5 = local_14->maxConsititutionNo;
        consititutionNo = 0;
        if (0 < sVar5) {
          do {
            iVar3 = (int)consititutionNo;
            if ((local_14->pConsititutions[iVar3].attrib & 1) == 0) {
              sVar6 = consititutionNo + 1;
              bVar8 = SBORROW2(sVar5,sVar6);
              sVar1 = sVar5 - sVar6;
              bVar7 = sVar5 == sVar6;
            }
            else {
              iVar2 = vm_saveString(fd,(local_14->pConsititutions[iVar3].pProgram)->strName->ptr);
              if (iVar2 == 0) {
                return 0;
              }
              pVVar9 = local_14;
              uVar10 = extraout_ECX;
              iVar2 = vm_saveBinaryInherit(fd,local_14,consititutionNo);
              if (iVar2 == 0) {
                vm_errprintf("Save program failed: write inherit program.\n",pVVar9,iVar3,uVar10);
                return 0;
              }
              sVar5 = local_14->maxConsititutionNo;
              bVar8 = SBORROW2(sVar5,consititutionNo + 1);
              sVar1 = sVar5 - (consititutionNo + 1);
              bVar7 = sVar1 == 0;
            }
            consititutionNo = consititutionNo + 1;
          } while (!bVar7 && bVar8 == sVar1 < 0);
        }
        iVar3 = 1;
      }
    }
  }
  return iVar3;
}


```

<details><summary>汇编</summary>

```asm
0x8087164: PUSH EBP
0x8087165: MOV EBP, ESP
0x8087167: PUSH EDI
0x8087168: PUSH ESI
0x8087169: PUSH EBX
0x808716a: SUB ESP, 0xc
0x808716d: MOV EAX, [0x0847aac0]
0x8087172: TEST EAX, EAX
0x8087174: JZ 0x0808723c
0x808717a: MOV dword ptr [EBP + -0x10], EAX
0x808717d: MOV EAX, [0x0847aad0]
0x8087182: CMP dword ptr [EBP + -0x10], EAX
0x8087185: MOV EDX, 0x1
0x808718a: JZ 0x08087230
0x8087190: SUB ESP, 0x8
0x8087193: PUSH 0x82ab0ba
0x8087198: PUSH dword ptr [EBP + 0x8]
0x808719b: CALL 0x080b8698
0x80871a0: MOV EDX, dword ptr [EBP + -0x10]
0x80871a3: POP EBX
0x80871a4: POP ESI
0x80871a5: MOVSX EAX, word ptr [EDX + 0x14]
0x80871a9: PUSH EAX
0x80871aa: PUSH dword ptr [EBP + 0x8]
0x80871ad: CALL 0x080b871c
0x80871b2: ADD ESP, 0x10
0x80871b5: XOR EDX, EDX
0x80871b7: TEST EAX, EAX
0x80871b9: JZ 0x08087230
0x80871bb: MOV EAX, dword ptr [EBP + -0x10]
0x80871be: MOV DI, word ptr [EAX + 0x14]
0x80871c2: XOR ECX, ECX
0x80871c4: CMP DI, 0x0
0x80871c8: JLE 0x080871e7
0x80871ca: MOV EDX, dword ptr [EAX + 0x18]
0x80871cd: ADD EDX, 0x4
0x80871d0: MOV EBX, EDI
0x80871d2: MOV ESI, ESI
0x80871d4: MOVZX EAX, byte ptr [EDX]
0x80871d7: AND EAX, 0x1
0x80871da: CMP EAX, 0x1
0x80871dd: SBB ECX, -0x1
0x80871e0: ADD EDX, 0xc
0x80871e3: DEC BX
0x80871e5: JNZ 0x080871d4
0x80871e7: SUB ESP, 0x8
0x80871ea: PUSH ECX
0x80871eb: PUSH dword ptr [EBP + 0x8]
0x80871ee: CALL 0x080b86a4
0x80871f3: ADD ESP, 0x10
0x80871f6: XOR EDX, EDX
0x80871f8: TEST EAX, EAX
0x80871fa: JZ 0x08087230
0x80871fc: MOV EDX, dword ptr [EBP + -0x10]
0x80871ff: MOV DI, word ptr [EDX + 0x14]
0x8087203: XOR EBX, EBX
0x8087205: CMP DI, 0x0
0x8087209: JLE 0x08087228
0x808720b: NOP
0x808720c: MOV EAX, dword ptr [EBP + -0x10]
0x808720f: MOVSX ESI, BX
0x8087212: MOV EDX, dword ptr [EAX + 0x18]
0x8087215: LEA EAX, [ESI + ESI*0x2]
0x8087218: SHL EAX, 0x2
0x808721b: TEST byte ptr [EAX + EDX*0x1 + 0x4], 0x1
0x8087220: JNZ 0x0808724c
0x8087222: INC EBX
0x8087223: CMP DI, BX
0x8087226: JG 0x0808720c
0x8087228: MOV EDX, 0x1
0x808722d: LEA ESI, [ESI]
0x8087230: LEA ESP, [EBP + -0xc]
0x8087233: POP EBX
0x8087234: POP ESI
0x8087235: MOV EAX, EDX
0x8087237: POP EDI
0x8087238: LEAVE
0x8087239: RET
0x808723c: MOV EAX, [0x0847aad0]
0x8087241: MOV dword ptr [EBP + -0x10], EAX
0x8087244: JMP 0x08087182
0x808724c: MOV EAX, dword ptr [EAX + EDX*0x1]
0x808724f: SUB ESP, 0x8
0x8087252: MOV EAX, dword ptr [EAX + 0x4]
0x8087255: PUSH dword ptr [EAX + 0xc]
0x8087258: PUSH dword ptr [EBP + 0x8]
0x808725b: CALL 0x080b8770
0x8087260: ADD ESP, 0x10
0x8087263: TEST EAX, EAX
0x8087265: JZ 0x08087295
0x8087267: PUSH ECX
0x8087268: PUSH ESI
0x8087269: PUSH dword ptr [EBP + -0x10]
0x808726c: PUSH dword ptr [EBP + 0x8]
0x808726f: CALL 0x080a1fd0
0x8087274: ADD ESP, 0x10
0x8087277: TEST EAX, EAX
0x8087279: JZ 0x08087288
0x808727b: MOV EDX, dword ptr [EBP + -0x10]
0x808727e: MOV DI, word ptr [EDX + 0x14]
0x8087282: INC EBX
0x8087283: CMP DI, BX
0x8087286: JMP 0x08087226
0x8087288: SUB ESP, 0xc
0x808728b: PUSH 0x82ab888
0x8087290: CALL 0x08097acc
0x8087295: XOR EDX, EDX
0x8087297: JMP 0x08087230
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadNumber16"></a>

## vm_loadNumber16 @ 0x80b8f44

Evidence: `ev_9a7a1e0aa59614769460ed8f84beaf7dfdf57e53a3b8515fc53be210fc6f5fed`

```c

int vm_loadNumber16(Vm_Binary_Buffer_T *pInput,ushort *num)

{
  ushort uVar1;
  uint uVar2;
  
                    /* Unresolved local var: ushort temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 2) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 2) {
      return 0;
    }
  }
  uVar1 = *(ushort *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 2);
  pInput->len = uVar2 - 2;
  if (num != (ushort *)0x0) {
    *num = uVar1;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8f44: PUSH EBP
0x80b8f45: MOV EBP, ESP
0x80b8f47: PUSH ESI
0x80b8f48: PUSH EBX
0x80b8f49: MOV EBX, dword ptr [EBP + 0x8]
0x80b8f4c: MOV EDX, dword ptr [EBX + 0x8]
0x80b8f4f: CMP EDX, 0x1
0x80b8f52: MOV ESI, dword ptr [EBP + 0xc]
0x80b8f55: JBE 0x080b8f7c
0x80b8f57: MOV EAX, dword ptr [EBX + 0x4]
0x80b8f5a: MOV CX, word ptr [EAX]
0x80b8f5d: ADD EAX, 0x2
0x80b8f60: MOV dword ptr [EBX + 0x4], EAX
0x80b8f63: TEST ESI, ESI
0x80b8f65: LEA EAX, [EDX + -0x2]
0x80b8f68: MOV dword ptr [EBX + 0x8], EAX
0x80b8f6b: JZ 0x080b8f70
0x80b8f6d: MOV word ptr [ESI], CX
0x80b8f70: LEA ESP, [EBP + -0x8]
0x80b8f73: POP EBX
0x80b8f74: MOV EAX, 0x1
0x80b8f79: POP ESI
0x80b8f7a: LEAVE
0x80b8f7b: RET
0x80b8f7c: SUB ESP, 0xc
0x80b8f7f: PUSH EBX
0x80b8f80: CALL 0x080b88a0
0x80b8f85: MOV EDX, dword ptr [EBX + 0x8]
0x80b8f88: ADD ESP, 0x10
0x80b8f8b: XOR EAX, EAX
0x80b8f8d: CMP EDX, 0x1
0x80b8f90: JA 0x080b8f57
0x80b8f92: LEA ESP, [EBP + -0x8]
0x80b8f95: POP EBX
0x80b8f96: POP ESI
0x80b8f97: LEAVE
0x80b8f98: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_addEfuns"></a>

## vm_addEfuns @ 0x8085ce4

Evidence: `ev_a83bf890b4b1b7b1e670ffd84e2caf433106c4b1521c658ee8c18bbdb826b9a1`

```c

/* WARNING: Unknown calling convention */

void vm_addEfuns(Vm_Efun_T *elist)

{
  Vm_Ident_Hash_Elem *pVVar1;
  Vm_Program_T *pVVar2;
  ushort uVar3;
  Vm_Program_T *pVVar4;
  short sVar5;
  Vm_Program_T *pVVar6;
  Vm_Ident_Hash_Elem *pVVar7;
  uint uVar8;
  int local_14;
  
                    /* Unresolved local var: int i@[???] */
  local_14 = 0;
  for (pVVar7 = (Vm_Ident_Hash_Elem *)elist; pVVar7->name != (char *)0x0;
      pVVar7 = (Vm_Ident_Hash_Elem *)&(pVVar7->ii).pInContext) {
                    /* Unresolved local var: int h@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNode@[DW_OP_reg1(ECX)] */
    uVar3 = vm_whashstr(pVVar7->name,0x20);
    pVVar2 = vm_pGlobalProgram;
    uVar8 = uVar3 & 0x3ff;
    pVVar1 = vm_pGlobalProgram->ppHashTable[uVar8];
    if (pVVar1 == (Vm_Ident_Hash_Elem *)0x0) {
      vm_pGlobalProgram->ppHashHead[uVar8] = pVVar7;
      pVVar7->pNext = pVVar7;
      pVVar1 = pVVar7;
    }
    else {
      pVVar7->pNext = pVVar1->pNext;
      pVVar1->pNext->ppPrev = &pVVar7->pNext;
      pVVar1->pNext = pVVar7;
    }
    pVVar7->ppPrev = &pVVar1->pNext;
    pVVar6 = vm_pCurrentProgram;
    pVVar2->ppHashTable[uVar8] = pVVar7;
    pVVar7->semValue = pVVar7->semValue + 1;
    pVVar4 = pVVar6;
    if (pVVar6 == (Vm_Program_T *)0x0) {
      pVVar4 = pVVar2;
    }
    sVar5 = pVVar4->maxFunctionNo;
    pVVar7->token = 0x40000000;
    if (sVar5 == 0x7fff) {
      yyerrors("Too many functions (%d) in a program (> %d).\n",(int)pVVar4->maxFunctionNo,0x7fff,
               pVVar6);
      sVar5 = -1;
    }
    else {
      if (pVVar6 == (Vm_Program_T *)0x0) {
        pVVar6 = pVVar2;
      }
      sVar5 = pVVar6->maxFunctionNo;
      pVVar6->maxFunctionNo = sVar5 + 1;
    }
    *(short *)((int)&elist->funOffset + local_14) = sVar5;
    local_14 = local_14 + 0x1c;
  }
  return;
}


```

<details><summary>汇编</summary>

```asm
0x8085ce4: PUSH EBP
0x8085ce5: MOV EBP, ESP
0x8085ce7: PUSH EDI
0x8085ce8: PUSH ESI
0x8085ce9: PUSH EBX
0x8085cea: SUB ESP, 0xc
0x8085ced: MOV EBX, dword ptr [EBP + 0x8]
0x8085cf0: MOV dword ptr [EBP + -0x10], 0x0
0x8085cf7: JMP 0x08085d21
0x8085cfc: TEST EDX, EDX
0x8085cfe: CMOVZ EDX, ESI
0x8085d01: MOV AX, word ptr [EDX + 0x10]
0x8085d05: MOVSX ECX, AX
0x8085d08: INC EAX
0x8085d09: MOV word ptr [EDX + 0x10], AX
0x8085d0d: MOV EAX, dword ptr [EBP + -0x10]
0x8085d10: MOV EDX, dword ptr [EBP + 0x8]
0x8085d13: MOV word ptr [EAX + EDX*0x1 + 0x1a], CX
0x8085d18: ADD EAX, 0x1c
0x8085d1b: MOV dword ptr [EBP + -0x10], EAX
0x8085d1e: ADD EBX, 0x1c
0x8085d21: MOV EAX, dword ptr [EBX]
0x8085d23: TEST EAX, EAX
0x8085d25: JZ 0x08085dc2
0x8085d2b: SUB ESP, 0x8
0x8085d2e: PUSH 0x20
0x8085d30: PUSH EAX
0x8085d31: CALL 0x08089dc8
0x8085d36: MOV EDI, EAX
0x8085d38: MOV ESI, dword ptr [0x0847aad0]
0x8085d3e: AND EDI, 0x3ff
0x8085d44: MOV EAX, dword ptr [ESI + 0x54]
0x8085d47: MOV ECX, dword ptr [EAX + EDI*0x4]
0x8085d4a: ADD ESP, 0x10
0x8085d4d: TEST ECX, ECX
0x8085d4f: JZ 0x08085db4
0x8085d51: MOV EAX, dword ptr [ECX + 0xc]
0x8085d54: MOV dword ptr [EBX + 0xc], EAX
0x8085d57: MOV EDX, dword ptr [ECX + 0xc]
0x8085d5a: LEA EAX, [EBX + 0xc]
0x8085d5d: MOV dword ptr [EDX + 0x10], EAX
0x8085d60: MOV dword ptr [ECX + 0xc], EBX
0x8085d63: LEA EAX, [ECX + 0xc]
0x8085d66: MOV dword ptr [EBX + 0x10], EAX
0x8085d69: MOV EAX, dword ptr [ESI + 0x54]
0x8085d6c: MOV EDX, dword ptr [0x0847aac0]
0x8085d72: MOV dword ptr [EAX + EDI*0x4], EBX
0x8085d75: INC dword ptr [EBX + 0x8]
0x8085d78: MOV EAX, EDX
0x8085d7a: TEST EDX, EDX
0x8085d7c: CMOVZ EAX, ESI
0x8085d7f: CMP word ptr [EAX + 0x10], 0x7fff
0x8085d85: MOV dword ptr [EBX + 0x4], 0x40000000
0x8085d8c: JNZ 0x08085cfc
0x8085d92: MOVSX EAX, word ptr [EAX + 0x10]
0x8085d96: PUSH EDX
0x8085d97: PUSH 0x7fff
0x8085d9c: PUSH EAX
0x8085d9d: PUSH 0x82ab320
0x8085da2: CALL 0x080ff97c
0x8085da7: ADD ESP, 0x10
0x8085daa: MOV ECX, 0xffffffff
0x8085daf: JMP 0x08085d0d
0x8085db4: MOV EAX, dword ptr [ESI + 0x58]
0x8085db7: MOV dword ptr [EAX + EDI*0x4], EBX
0x8085dba: LEA EAX, [EBX + 0xc]
0x8085dbd: MOV dword ptr [EBX + 0xc], EBX
0x8085dc0: JMP 0x08085d66
0x8085dc2: LEA ESP, [EBP + -0xc]
0x8085dc5: POP EBX
0x8085dc6: POP ESI
0x8085dc7: POP EDI
0x8085dc8: LEAVE
0x8085dc9: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_saveIdentifiers"></a>

## vm_saveIdentifiers @ 0x8087aec

Evidence: `ev_acf474413a54525974ac2ae0eb8283c700d2ca17b77dd68e321b3381d4c4c984`

```c

/* WARNING: Unknown calling convention */

int vm_saveIdentifiers(int fd)

{
  Vm_Ident_Hash_Elem_T *pVVar1;
  Vm_Ident_Hash_Elem *pVVar2;
  uint uVar3;
  int iVar4;
  uint size_00;
  void *ptr;
  void *ptr_00;
  Vm_Ident_Hash_Elem **ppVVar5;
  int iVar6;
  short index;
  Vm_Ident_Hash_Elem_T *pVVar7;
  uint uVar8;
  Vm_Prog_Var_T *pVVar9;
  Vm_Fun *pVVar10;
  Vm_Program_T *pVVar11;
  void *buf;
  Vm_Program_T *pVVar12;
  undefined4 uVar13;
  int i_1;
  int i;
  int size;
  int count;
  char *pNames;
  Vm_Dlt_T *pDlt;
  void *para [2];
  uint num [2];
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  num[0] = 0;
  num[1] = 0;
  vm_saveComment(fd,"IDENTIFIERS");
  pVVar12 = vm_pGlobalProgram;
  if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
    pVVar12 = vm_pCurrentProgram;
  }
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNext@[DW_OP_reg6(ESI)] */
  i = 0;
  ppVVar5 = pVVar12->ppHashHead;
  do {
    pVVar1 = ppVVar5[i];
    pVVar7 = pVVar1;
    while (pVVar1 != (Vm_Ident_Hash_Elem_T *)0x0) {
      pVVar2 = pVVar7->pNext;
      iVar4 = vm_countIdents(pVVar7,num);
      if (iVar4 == 0) {
        vm_freeIdentInProgram(pVVar12,pVVar7);
        ppVVar5 = pVVar12->ppHashHead;
      }
      else {
        ppVVar5 = pVVar12->ppHashHead;
        if (pVVar2 == ppVVar5[i]) break;
      }
      pVVar7 = pVVar2;
      pVVar1 = ppVVar5[i];
    }
    i = i + 1;
  } while (i < 0x400);
  size_00 = (num[1] | 0xf) + 1;
  ptr = pdb_allocateMemory(size_00,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x170c);
  uVar3 = num[0];
  if (ptr == (void *)0x0) {
    return 0;
  }
  ptr_00 = pdb_allocateMemory(num[0] * 0x14,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1713);
  if (ptr_00 == (void *)0x0) {
    pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1717);
    return 0;
  }
  pVVar12 = vm_pGlobalProgram;
  if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
    pVVar12 = vm_pCurrentProgram;
  }
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * pNext@[DW_OP_reg6(ESI)] */
  i_1 = 0;
  ppVVar5 = pVVar12->ppHashHead;
  para[0] = ptr_00;
  para[1] = ptr;
  do {
    pVVar1 = ppVVar5[i_1];
    pVVar7 = pVVar1;
    while (pVVar1 != (Vm_Ident_Hash_Elem_T *)0x0) {
      pVVar2 = pVVar7->pNext;
      iVar4 = vm_copyIdentsName(pVVar7,para);
      if (iVar4 == 0) {
        vm_freeIdentInProgram(pVVar12,pVVar7);
        ppVVar5 = pVVar12->ppHashHead;
      }
      else {
        ppVVar5 = pVVar12->ppHashHead;
        if (pVVar2 == ppVVar5[i_1]) break;
      }
      pVVar7 = pVVar2;
      pVVar1 = ppVVar5[i_1];
    }
    i_1 = i_1 + 1;
    if (0x3ff < i_1) {
      if (0 < (int)uVar3) {
        iVar4 = 0;
        uVar8 = uVar3;
        do {
          *(int *)(iVar4 + (int)ptr_00) = *(int *)(iVar4 + (int)ptr_00) - (int)ptr;
          iVar4 = iVar4 + 0x14;
          uVar8 = uVar8 - 1;
        } while (uVar8 != 0);
      }
      vm_saveComment(fd,"NAMES");
      iVar4 = vm_saveNumber32(fd,uVar3);
      if (((iVar4 != 0) && (iVar4 = vm_saveNumber32(fd,size_00), iVar4 != 0)) &&
         (iVar4 = vm_saveBuffer(fd,ptr,size_00), iVar4 != 0)) {
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Index_T variables@[DW_OP_reg2(EDX)] */
        pVVar11 = vm_pGlobalProgram;
        if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
          pVVar11 = vm_pCurrentProgram;
        }
        vm_saveComment(fd,"VARIABLES");
        index = (Vm_Index_T)vm_globalVar;
        if (pVVar11 == vm_pGlobalProgram) {
          iVar4 = vm_saveVmIndex(fd,(Vm_Index_T)vm_globalVar);
          if (iVar4 == 0) {
            return 0;
          }
        }
        else {
                    /* Unresolved local var: Vm_Prog_Var_T * pVar@[DW_OP_reg0(EAX)] */
          iVar4 = vm_saveVmIndex(fd,pVVar11->maxVariableNo);
          if (iVar4 == 0) {
            return 0;
          }
          index = 0;
          for (pVVar9 = pVVar11->pProgVars; pVVar9 != (Vm_Prog_Var_T *)0x0; pVVar9 = pVVar9->pNext)
          {
            index = index + 1;
          }
        }
        iVar4 = vm_saveVmIndex(fd,index);
        if (iVar4 != 0) {
          if (pVVar11 != vm_pGlobalProgram) {
                    /* Unresolved local var: Vm_Prog_Var_T * pVar@[DW_OP_reg3(EBX)] */
            vm_saveComment(fd,"PROG_VARS");
            for (pVVar9 = pVVar11->pProgVars; pVVar9 != (Vm_Prog_Var_T *)0x0; pVVar9 = pVVar9->pNext
                ) {
              iVar4 = vm_saveBuffer(fd,pVVar9,0x10);
              if (iVar4 == 0) {
                return 0;
              }
              iVar4 = vm_saveString(fd,pVVar9->pVarName);
              if (iVar4 == 0) {
                return 0;
              }
            }
          }
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Fun_T * pHiddenFun@[DW_OP_reg3(EBX)]
                       Unresolved local var: int count@[DW_OP_reg2(EDX)] */
          pVVar11 = vm_pGlobalProgram;
          if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
            pVVar11 = vm_pCurrentProgram;
          }
          vm_saveComment(fd,"FUNCTIONS");
          iVar4 = vm_saveVmIndex(fd,vm_efunsCount);
          if ((iVar4 != 0) && (iVar4 = vm_saveVmIndex(fd,pVVar11->maxFunctionNo), iVar4 != 0)) {
            vm_saveComment(fd,"HIDDEN_FUNS");
            iVar4 = 0;
            for (pVVar10 = pVVar11->pHiddenFuns; pVVar10 != vm_hiddenFunsBC;
                pVVar10 = pVVar10->pNext) {
              iVar4 = iVar4 + 1;
            }
            iVar4 = vm_saveNumber32(fd,iVar4);
            if (iVar4 != 0) {
              for (pVVar10 = pVVar11->pHiddenFuns; pVVar10 != vm_hiddenFunsBC;
                  pVVar10 = pVVar10->pNext) {
                iVar4 = vm_saveFun(fd,pVVar10);
                if (iVar4 == 0) {
                  return 0;
                }
              }
              iVar4 = vm_saveInheritPrograms(fd);
              if (iVar4 == 0) {
                return 0;
              }
              vm_saveComment(fd,"DLT");
              iVar4 = vm_saveNumber32(fd,uVar3);
              if (iVar4 == 0) {
                return 0;
              }
              iVar4 = uVar3 - 1;
              if (-1 < iVar4) {
                buf = (void *)((int)ptr_00 + iVar4 * 0x14);
                do {
                    /* Unresolved local var: Vm_Program_T * pProgram@[???] */
                  uVar13 = 0x14;
                  pVVar11 = pVVar12;
                  iVar6 = vm_saveBuffer(fd,buf,0x14);
                  if (iVar6 == 0) {
                    return 0;
                  }
                  uVar3 = *(uint *)((int)buf + 4);
                  if (((uVar3 & 0x80000) == 0) && ((uVar3 & 0x40000) == 0)) {
                    if ((uVar3 & 0x8000000) == 0) {
                      vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n",
                                 "Bad type of DLT to save.\n",
                                 "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1ab3,uVar13,pVVar11);
                      _vm_endProcess(0);
                      do {
                    /* WARNING: Do nothing block with infinite loop */
                      } while( true );
                    }
                    iVar6 = vm_saveFun(fd,*(Vm_Fun_T **)((int)buf + 0xc));
                    if (iVar6 == 0) {
                      return 0;
                    }
                  }
                  buf = (void *)((int)buf + -0x14);
                  iVar4 = iVar4 + -1;
                } while (-1 < iVar4);
              }
              pdb_freeMemory(ptr_00,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x174b);
              pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x174c);
              return 1;
            }
          }
        }
      }
      return 0;
    }
  } while( true );
}


```

<details><summary>汇编</summary>

```asm
0x8087aec: PUSH EBP
0x8087aed: MOV EBP, ESP
0x8087aef: PUSH EDI
0x8087af0: PUSH ESI
0x8087af1: PUSH EBX
0x8087af2: SUB ESP, 0x34
0x8087af5: PUSH 0x82ab0cf
0x8087afa: PUSH dword ptr [EBP + 0x8]
0x8087afd: MOV dword ptr [EBP + -0x18], 0x0
0x8087b04: MOV dword ptr [EBP + -0x14], 0x0
0x8087b0b: CALL 0x080b8698
0x8087b10: MOV EAX, [0x0847aac0]
0x8087b15: ADD ESP, 0x10
0x8087b18: MOV EDI, dword ptr [0x0847aad0]
0x8087b1e: TEST EAX, EAX
0x8087b20: CMOVNZ EDI, EAX
0x8087b23: MOV dword ptr [EBP + -0x34], 0x0
0x8087b2a: MOV EAX, dword ptr [EDI + 0x58]
0x8087b2d: LEA ESI, [ESI]
0x8087b30: MOV EDX, dword ptr [EBP + -0x34]
0x8087b33: MOV EBX, dword ptr [EAX + EDX*0x4]
0x8087b36: TEST EBX, EBX
0x8087b38: JMP 0x08087b51
0x8087b3c: MOV EAX, dword ptr [EDI + 0x58]
0x8087b3f: MOV EDX, dword ptr [EBP + -0x34]
0x8087b42: CMP ESI, dword ptr [EAX + EDX*0x4]
0x8087b45: MOV EBX, ESI
0x8087b47: JZ 0x08087b80
0x8087b49: MOV ECX, dword ptr [EBP + -0x34]
0x8087b4c: MOV EDX, dword ptr [EAX + ECX*0x4]
0x8087b4f: TEST EDX, EDX
0x8087b51: JZ 0x08087b80
0x8087b53: SUB ESP, 0x8
0x8087b56: LEA ECX, [EBP + -0x18]
0x8087b59: MOV ESI, dword ptr [EBX + 0xc]
0x8087b5c: PUSH ECX
0x8087b5d: PUSH EBX
0x8087b5e: CALL 0x080879d4
0x8087b63: ADD ESP, 0x10
0x8087b66: TEST EAX, EAX
0x8087b68: JNZ 0x08087b3c
0x8087b6a: SUB ESP, 0x8
0x8087b6d: PUSH EBX
0x8087b6e: PUSH EDI
0x8087b6f: CALL 0x080856d8
0x8087b74: MOV EBX, ESI
0x8087b76: ADD ESP, 0x10
0x8087b79: MOV EAX, dword ptr [EDI + 0x58]
0x8087b7c: JMP 0x08087b49
0x8087b80: INC dword ptr [EBP + -0x34]
0x8087b83: CMP dword ptr [EBP + -0x34], 0x3ff
0x8087b8a: JLE 0x08087b30
0x8087b8c: PUSH 0x170c
0x8087b91: MOV EAX, dword ptr [EBP + -0x14]
0x8087b94: PUSH 0x82ab14c
0x8087b99: OR EAX, 0xf
0x8087b9c: INC EAX
0x8087b9d: PUSH 0x82a376b
0x8087ba2: PUSH EAX
0x8087ba3: MOV dword ptr [EBP + -0x30], EAX
0x8087ba6: CALL 0x080ce784
0x8087bab: ADD ESP, 0x10
0x8087bae: XOR EDX, EDX
0x8087bb0: TEST EAX, EAX
0x8087bb2: MOV dword ptr [EBP + -0x28], EAX
0x8087bb5: JZ 0x08087cad
0x8087bbb: PUSH 0x1713
0x8087bc0: MOV EAX, dword ptr [EBP + -0x18]
0x8087bc3: PUSH 0x82ab14c
0x8087bc8: MOV dword ptr [EBP + -0x2c], EAX
0x8087bcb: LEA EAX, [EAX + EAX*0x4]
0x8087bce: PUSH 0x82a376b
0x8087bd3: SHL EAX, 0x2
0x8087bd6: PUSH EAX
0x8087bd7: CALL 0x080ce784
0x8087bdc: ADD ESP, 0x10
0x8087bdf: TEST EAX, EAX
0x8087be1: MOV dword ptr [EBP + -0x24], EAX
0x8087be4: JZ 0x08087daf
0x8087bea: MOV EAX, dword ptr [EBP + -0x28]
0x8087bed: MOV dword ptr [EBP + -0x1c], EAX
0x8087bf0: MOV EAX, [0x0847aac0]
0x8087bf5: TEST EAX, EAX
0x8087bf7: MOV EDI, dword ptr [0x0847aad0]
0x8087bfd: MOV ECX, dword ptr [EBP + -0x24]
0x8087c00: CMOVNZ EDI, EAX
0x8087c03: MOV dword ptr [EBP + -0x20], ECX
0x8087c06: MOV dword ptr [EBP + -0x38], 0x0
0x8087c0d: MOV EAX, dword ptr [EDI + 0x58]
0x8087c10: MOV EDX, dword ptr [EBP + -0x38]
0x8087c13: MOV EBX, dword ptr [EAX + EDX*0x4]
0x8087c16: TEST EBX, EBX
0x8087c18: JMP 0x08087c31
0x8087c1c: MOV EAX, dword ptr [EDI + 0x58]
0x8087c1f: MOV EDX, dword ptr [EBP + -0x38]
0x8087c22: CMP ESI, dword ptr [EAX + EDX*0x4]
0x8087c25: MOV EBX, ESI
0x8087c27: JZ 0x08087c60
0x8087c29: MOV ECX, dword ptr [EBP + -0x38]
0x8087c2c: MOV ESI, dword ptr [EAX + ECX*0x4]
0x8087c2f: TEST ESI, ESI
0x8087c31: JZ 0x08087c60
0x8087c33: SUB ESP, 0x8
0x8087c36: LEA ECX, [EBP + -0x20]
0x8087c39: MOV ESI, dword ptr [EBX + 0xc]
0x8087c3c: PUSH ECX
0x8087c3d: PUSH EBX
0x8087c3e: CALL 0x08087a3c
0x8087c43: ADD ESP, 0x10
0x8087c46: TEST EAX, EAX
0x8087c48: JNZ 0x08087c1c
0x8087c4a: SUB ESP, 0x8
0x8087c4d: PUSH EBX
0x8087c4e: PUSH EDI
0x8087c4f: CALL 0x080856d8
0x8087c54: MOV EBX, ESI
0x8087c56: ADD ESP, 0x10
0x8087c59: MOV EAX, dword ptr [EDI + 0x58]
0x8087c5c: JMP 0x08087c29
0x8087c60: INC dword ptr [EBP + -0x38]
0x8087c63: CMP dword ptr [EBP + -0x38], 0x3ff
0x8087c6a: JLE 0x08087c10
0x8087c6c: MOV ECX, dword ptr [EBP + -0x2c]
0x8087c6f: TEST ECX, ECX
0x8087c71: JLE 0x08087c87
0x8087c73: XOR EAX, EAX
0x8087c75: MOV EBX, dword ptr [EBP + -0x2c]
0x8087c78: MOV ECX, dword ptr [EBP + -0x28]
0x8087c7b: MOV EDX, dword ptr [EBP + -0x24]
0x8087c7e: SUB dword ptr [EAX + EDX*0x1], ECX
0x8087c81: ADD EAX, 0x14
0x8087c84: DEC EBX
0x8087c85: JNZ 0x08087c78
0x8087c87: SUB ESP, 0x8
0x8087c8a: PUSH 0x82ab0db
0x8087c8f: PUSH dword ptr [EBP + 0x8]
0x8087c92: CALL 0x080b8698
0x8087c97: POP EAX
0x8087c98: POP EDX
0x8087c99: PUSH dword ptr [EBP + -0x2c]
0x8087c9c: PUSH dword ptr [EBP + 0x8]
0x8087c9f: CALL 0x080b86a4
0x8087ca4: ADD ESP, 0x10
0x8087ca7: TEST EAX, EAX
0x8087ca9: JNZ 0x08087cb7
0x8087cab: XOR EDX, EDX
0x8087cad: LEA ESP, [EBP + -0xc]
0x8087cb0: POP EBX
0x8087cb1: POP ESI
0x8087cb2: MOV EAX, EDX
0x8087cb4: POP EDI
0x8087cb5: LEAVE
0x8087cb6: RET
0x8087cb7: SUB ESP, 0x8
0x8087cba: PUSH dword ptr [EBP + -0x30]
0x8087cbd: PUSH dword ptr [EBP + 0x8]
0x8087cc0: CALL 0x080b86a4
0x8087cc5: ADD ESP, 0x10
0x8087cc8: TEST EAX, EAX
0x8087cca: JZ 0x08087cab
0x8087ccc: PUSH EDI
0x8087ccd: PUSH dword ptr [EBP + -0x30]
0x8087cd0: PUSH dword ptr [EBP + -0x28]
0x8087cd3: PUSH dword ptr [EBP + 0x8]
0x8087cd6: CALL 0x080b8748
0x8087cdb: ADD ESP, 0x10
0x8087cde: TEST EAX, EAX
0x8087ce0: JZ 0x08087cab
0x8087ce2: MOV EAX, [0x0847aac0]
0x8087ce7: TEST EAX, EAX
0x8087ce9: MOV ESI, dword ptr [0x0847aad0]
0x8087cef: CMOVNZ ESI, EAX
0x8087cf2: SUB ESP, 0x8
0x8087cf5: PUSH 0x82ab0e1
0x8087cfa: PUSH dword ptr [EBP + 0x8]
0x8087cfd: CALL 0x080b8698
0x8087d02: ADD ESP, 0x10
0x8087d05: CMP ESI, dword ptr [0x0847aad0]
0x8087d0b: JZ 0x08087dcd
0x8087d11: SUB ESP, 0x8
0x8087d14: MOVSX EAX, word ptr [ESI + 0xe]
0x8087d18: PUSH EAX
0x8087d19: PUSH dword ptr [EBP + 0x8]
0x8087d1c: CALL 0x080b871c
0x8087d21: ADD ESP, 0x10
0x8087d24: TEST EAX, EAX
0x8087d26: JZ 0x08087cab
0x8087d28: MOV EAX, dword ptr [ESI + 0x28]
0x8087d2b: XOR EDX, EDX
0x8087d2d: XOR EBX, EBX
0x8087d2f: TEST EAX, EAX
0x8087d31: JZ 0x08087d3e
0x8087d33: MOV EAX, dword ptr [EAX + 0xc]
0x8087d36: INC EDX
0x8087d37: TEST EAX, EAX
0x8087d39: JNZ 0x08087d33
0x8087d3b: MOVSX EBX, DX
0x8087d3e: SUB ESP, 0x8
0x8087d41: PUSH EBX
0x8087d42: PUSH dword ptr [EBP + 0x8]
0x8087d45: CALL 0x080b871c
0x8087d4a: ADD ESP, 0x10
0x8087d4d: TEST EAX, EAX
0x8087d4f: JZ 0x08087cab
0x8087d55: CMP ESI, dword ptr [0x0847aad0]
0x8087d5b: JZ 0x08087df2
0x8087d61: SUB ESP, 0x8
0x8087d64: PUSH 0x82ab0eb
0x8087d69: PUSH dword ptr [EBP + 0x8]
0x8087d6c: CALL 0x080b8698
0x8087d71: ADD ESP, 0x10
0x8087d74: MOV EBX, dword ptr [ESI + 0x28]
0x8087d77: TEST EBX, EBX
0x8087d79: JZ 0x08087df2
0x8087d7b: PUSH ESI
0x8087d7c: PUSH 0x10
0x8087d7e: PUSH EBX
0x8087d7f: PUSH dword ptr [EBP + 0x8]
0x8087d82: CALL 0x080b8748
0x8087d87: ADD ESP, 0x10
0x8087d8a: TEST EAX, EAX
0x8087d8c: JZ 0x08087cab
0x8087d92: SUB ESP, 0x8
0x8087d95: PUSH dword ptr [EBX]
0x8087d97: PUSH dword ptr [EBP + 0x8]
0x8087d9a: CALL 0x080b8770
0x8087d9f: ADD ESP, 0x10
0x8087da2: TEST EAX, EAX
0x8087da4: JZ 0x08087cab
0x8087daa: MOV EBX, dword ptr [EBX + 0xc]
0x8087dad: JMP 0x08087d77
0x8087daf: PUSH 0x1717
0x8087db4: PUSH 0x82ab14c
0x8087db9: PUSH 0x82a376b
0x8087dbe: PUSH dword ptr [EBP + -0x28]
0x8087dc1: CALL 0x080ceb94
0x8087dc6: XOR EDX, EDX
0x8087dc8: JMP 0x08087cad
0x8087dcd: SUB ESP, 0x8
0x8087dd0: MOVSX EBX, word ptr [0x0847aac4]
0x8087dd7: PUSH EBX
0x8087dd8: PUSH dword ptr [EBP + 0x8]
0x8087ddb: CALL 0x080b871c
0x8087de0: ADD ESP, 0x10
0x8087de3: TEST EAX, EAX
0x8087de5: JNZ 0x08087d3e
0x8087deb: XOR EDX, EDX
0x8087ded: JMP 0x08087cad
0x8087df2: MOV EAX, [0x0847aac0]
0x8087df7: TEST EAX, EAX
0x8087df9: MOV ESI, dword ptr [0x0847aad0]
0x8087dff: CMOVNZ ESI, EAX
0x8087e02: SUB ESP, 0x8
0x8087e05: PUSH 0x82ab0f5
0x8087e0a: PUSH dword ptr [EBP + 0x8]
0x8087e0d: CALL 0x080b8698
0x8087e12: POP ECX
0x8087e13: POP EBX
0x8087e14: MOVSX EAX, word ptr [0x0847ace0]
0x8087e1b: PUSH EAX
0x8087e1c: PUSH dword ptr [EBP + 0x8]
0x8087e1f: CALL 0x080b871c
0x8087e24: ADD ESP, 0x10
0x8087e27: TEST EAX, EAX
0x8087e29: JZ 0x08087cab
0x8087e2f: SUB ESP, 0x8
0x8087e32: MOVSX EAX, word ptr [ESI + 0x10]
0x8087e36: PUSH EAX
0x8087e37: PUSH dword ptr [EBP + 0x8]
0x8087e3a: CALL 0x080b871c
0x8087e3f: ADD ESP, 0x10
0x8087e42: TEST EAX, EAX
0x8087e44: JZ 0x08087cab
0x8087e4a: SUB ESP, 0x8
0x8087e4d: PUSH 0x82ab0ff
0x8087e52: PUSH dword ptr [EBP + 0x8]
0x8087e55: CALL 0x080b8698
0x8087e5a: ADD ESP, 0x10
0x8087e5d: XOR EDX, EDX
0x8087e5f: MOV EBX, dword ptr [ESI + 0x30]
0x8087e62: MOV EAX, [0x0866192c]
0x8087e67: CMP EBX, EAX
0x8087e69: JZ 0x08087e71
0x8087e6b: INC EDX
0x8087e6c: MOV EBX, dword ptr [EBX + 0x3c]
0x8087e6f: JMP 0x08087e67
0x8087e71: SUB ESP, 0x8
0x8087e74: PUSH EDX
0x8087e75: PUSH dword ptr [EBP + 0x8]
0x8087e78: CALL 0x080b86a4
0x8087e7d: ADD ESP, 0x10
0x8087e80: TEST EAX, EAX
0x8087e82: JZ 0x08087cab
0x8087e88: MOV EBX, dword ptr [ESI + 0x30]
0x8087e8b: CMP EBX, dword ptr [0x0866192c]
0x8087e91: JZ 0x08087eaf
0x8087e93: SUB ESP, 0x8
0x8087e96: PUSH EBX
0x8087e97: PUSH dword ptr [EBP + 0x8]
0x8087e9a: CALL 0x08095848
0x8087e9f: ADD ESP, 0x10
0x8087ea2: TEST EAX, EAX
0x8087ea4: JZ 0x08087cab
0x8087eaa: MOV EBX, dword ptr [EBX + 0x3c]
0x8087ead: JMP 0x08087e8b
0x8087eaf: SUB ESP, 0xc
0x8087eb2: PUSH dword ptr [EBP + 0x8]
0x8087eb5: CALL 0x08087164
0x8087eba: ADD ESP, 0x10
0x8087ebd: XOR EDX, EDX
0x8087ebf: TEST EAX, EAX
0x8087ec1: JZ 0x08087cad
0x8087ec7: SUB ESP, 0x8
0x8087eca: PUSH 0x82ab10b
0x8087ecf: PUSH dword ptr [EBP + 0x8]
0x8087ed2: CALL 0x080b8698
0x8087ed7: POP EAX
0x8087ed8: POP EDX
0x8087ed9: PUSH dword ptr [EBP + -0x2c]
0x8087edc: PUSH dword ptr [EBP + 0x8]
0x8087edf: CALL 0x080b86a4
0x8087ee4: ADD ESP, 0x10
0x8087ee7: XOR EDX, EDX
0x8087ee9: TEST EAX, EAX
0x8087eeb: JZ 0x08087cad
0x8087ef1: MOV EBX, dword ptr [EBP + -0x2c]
0x8087ef4: DEC EBX
0x8087ef5: JS 0x08087f4e
0x8087ef7: LEA EAX, [EBX + EBX*0x4]
0x8087efa: MOV EDX, dword ptr [EBP + -0x24]
0x8087efd: LEA ESI, [EDX + EAX*0x4]
0x8087f00: PUSH EDI
0x8087f01: PUSH 0x14
0x8087f03: PUSH ESI
0x8087f04: PUSH dword ptr [EBP + 0x8]
0x8087f07: CALL 0x080b8748
0x8087f0c: ADD ESP, 0x10
0x8087f0f: TEST EAX, EAX
0x8087f11: JZ 0x08087cab
0x8087f17: MOV EAX, dword ptr [ESI + 0x4]
0x8087f1a: TEST EAX, 0x80000
0x8087f1f: JNZ 0x08087f48
0x8087f21: TEST EAX, 0x40000
0x8087f26: JNZ 0x08087f48
0x8087f28: TEST EAX, 0x8000000
0x8087f2d: JZ 0x08087f86
0x8087f2f: SUB ESP, 0x8
0x8087f32: PUSH dword ptr [ESI + 0xc]
0x8087f35: PUSH dword ptr [EBP + 0x8]
0x8087f38: CALL 0x08095848
0x8087f3d: ADD ESP, 0x10
0x8087f40: TEST EAX, EAX
0x8087f42: JZ 0x08087cab
0x8087f48: SUB ESI, 0x14
0x8087f4b: DEC EBX
0x8087f4c: JNS 0x08087f00
0x8087f4e: PUSH 0x174b
0x8087f53: PUSH 0x82ab14c
0x8087f58: PUSH 0x82a376b
0x8087f5d: PUSH dword ptr [EBP + -0x24]
0x8087f60: CALL 0x080ceb94
0x8087f65: PUSH 0x174c
0x8087f6a: PUSH 0x82ab14c
0x8087f6f: PUSH 0x82a376b
0x8087f74: PUSH dword ptr [EBP + -0x28]
0x8087f77: CALL 0x080ceb94
0x8087f7c: MOV EDX, 0x1
0x8087f81: JMP 0x08087cad
0x8087f86: SUB ESP, 0x8
0x8087f89: PUSH 0x1ab3
0x8087f8e: PUSH 0x82ab14c
0x8087f93: PUSH 0x82ab10f
0x8087f98: PUSH 0x82a825c
0x8087f9d: PUSH 0x4
0x8087f9f: PUSH 0x2
0x8087fa1: CALL 0x080b61c4
0x8087fa6: ADD ESP, 0x14
0x8087fa9: PUSH 0x0
0x8087fab: CALL 0x080a3980
0x8087fb0: ADD ESP, 0x10
0x8087fb3: JMP 0x08087fb3
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_buildEfunOffset"></a>

## vm_buildEfunOffset @ 0x808761c

Evidence: `ev_ad29fecd1081a867ce5d7533e223fa837e48dd3eaf530c6dd6564a7ee43c6fba`

```c

int vm_buildEfunOffset(Vm_Ident_Hash_Elem_T *ihe,void *pCookie)

{
  if ((ihe->token & 0x40000000) != 0) {
    vm_pGlobalProgram->pFunctions[*(short *)((int)&(ihe->ii).v + 6)].pFun = (Vm_Fun *)ihe;
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x808761c: PUSH EBP
0x808761d: MOV EBP, ESP
0x808761f: MOV ECX, dword ptr [EBP + 0x8]
0x8087622: TEST byte ptr [ECX + 0x7], 0x40
0x8087626: JZ 0x08087638
0x8087628: MOV EDX, dword ptr [0x0847aad0]
0x808762e: MOVSX EAX, word ptr [ECX + 0x1a]
0x8087632: MOV EDX, dword ptr [EDX + 0x2c]
0x8087635: MOV dword ptr [EDX + EAX*0x8], ECX
0x8087638: MOV EAX, 0x1
0x808763d: LEAVE
0x808763e: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadInheritPrograms"></a>

## vm_loadInheritPrograms @ 0x808729c

Evidence: `ev_af5958529d07c6f0f0b9b287dbdec72c60a8ad5632f6fd5b362a60f8d3a4b489`

```c

/* WARNING: Unknown calling convention */

int vm_loadInheritPrograms(Vm_Binary_Buffer_T *pInput)

{
  Vm_Program_Consititution_T *pVVar1;
  Vm_Index_T *pVVar2;
  uint uVar3;
  int iVar4;
  Vm_Program_T *pVVar5;
  short sVar6;
  char *str;
  char *num;
  Vm_String_T *strName;
  char *name;
  int count;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Index_T i@[DW_OP_reg6(ESI)] */
  pVVar5 = vm_pGlobalProgram;
  if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
    pVVar5 = vm_pCurrentProgram;
  }
  iVar4 = 1;
  if (pVVar5 != vm_pGlobalProgram) {
    num = "INHERITS";
    iVar4 = vm_loadComment(pInput,"INHERITS");
    if (iVar4 == 0) {
      str = "Load program failed: comment \"INHERITS\" mismatched.\n";
    }
    else {
      num = (char *)&pVVar5->maxConsititutionNo;
      iVar4 = vm_loadVmIndex(pInput,(Vm_Index_T *)num);
      if ((iVar4 == 0) || (pVVar5->maxConsititutionNo < 1)) {
        str = "Load program failed: read count of consititution no error.\n";
      }
      else {
                    /* Unresolved local var: Vm_Program_Consititution_T *
                       pConsititutions@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Index_T * pOrderList@[DW_OP_reg2(EDX)] */
        pVVar1 = pdb_reallocateMemory
                           (pVVar5->pConsititutions,pVVar5->maxConsititutionNo * 0xc,"VM",
                            "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19cd);
        if (pVVar1 != (Vm_Program_Consititution_T *)0x0) {
          pVVar5->consititutionBufferSize = pVVar5->maxConsititutionNo;
          pVVar5->pConsititutions = pVVar1;
          pVVar1 = pVVar1 + 1;
          for (uVar3 = pVVar5->maxConsititutionNo * 0xc - 0xcU >> 2; uVar3 != 0; uVar3 = uVar3 - 1)
          {
            pVVar1->pProgram = (Vm_Program *)0x0;
            pVVar1 = (Vm_Program_Consititution_T *)&pVVar1->attrib;
          }
          pVVar2 = pdb_reallocateMemory
                             (pVVar5->initOrderList,(int)pVVar5->maxConsititutionNo << 1,"VM",
                              "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19dc);
          if (pVVar2 != (Vm_Index_T *)0x0) {
            pVVar5->initOrderList = pVVar2;
            memset(pVVar2 + 1,0,pVVar5->maxConsititutionNo * 2 - 2);
          }
        }
        num = (char *)&count;
        iVar4 = vm_loadNumber32(pInput,(int *)num);
        if ((iVar4 == 0) || (count < 0)) {
          str = "Load program failed: read count of inherits error.\n";
        }
        else {
          if (0 < count) {
            sVar6 = 0;
            do {
                    /* Unresolved local var: char * pInheritProgramName@[DW_OP_reg7(EDI)] */
              pVVar2 = (Vm_Index_T *)&name;
              iVar4 = vm_loadTempString(pInput,(char **)pVVar2);
              if (iVar4 == 0) {
LAB_080874b8:
                num = (char *)pVVar2;
                str = "Load program failed: read inherit program\'s name error.\n";
                goto LAB_080872dd;
              }
              pVVar2 = (Vm_Index_T *)&strName;
              num = vm_addFileName(name,(Vm_String_T **)pVVar2);
              vm_scratchFree(name);
              if ((Vm_Index_T *)num == (Vm_Index_T *)0x0) goto LAB_080874b8;
              iVar4 = vm_loadBinaryInherit(pInput,num);
              if (iVar4 == 0) {
                str = "Load program failed: inherit program error.\n";
                goto LAB_080872dd;
              }
              sVar6 = sVar6 + 1;
            } while (sVar6 < count);
          }
          sVar6 = 0;
          if (0 < pVVar5->maxConsititutionNo) {
            pVVar1 = pVVar5->pConsititutions;
            num = (char *)0x0;
            do {
              if (pVVar1->pProgram == (Vm_Program *)0x0) {
                str = "Load program failed: got empty entry (%d) of consititution table.\n";
                goto LAB_080872dd;
              }
              sVar6 = sVar6 + 1;
              num = (char *)((int)num + 1);
              pVVar1 = pVVar1 + 1;
            } while (sVar6 < pVVar5->maxConsititutionNo);
          }
          num = "CONSTUCTION";
          iVar4 = vm_loadComment(pInput,"CONSTUCTION");
          if (iVar4 != 0) {
            return 1;
          }
          str = "Load program failed: comment \"CONSTUCTION\" mismatched.\n";
        }
      }
    }
LAB_080872dd:
    vm_errprintf(str,num);
    iVar4 = 0;
  }
  return iVar4;
}


```

<details><summary>汇编</summary>

```asm
0x808729c: PUSH EBP
0x808729d: MOV EBP, ESP
0x808729f: PUSH EDI
0x80872a0: PUSH ESI
0x80872a1: PUSH EBX
0x80872a2: SUB ESP, 0xc
0x80872a5: MOV EAX, [0x0847aac0]
0x80872aa: TEST EAX, EAX
0x80872ac: JZ 0x080872f0
0x80872ae: MOV EBX, EAX
0x80872b0: MOV EAX, [0x0847aad0]
0x80872b5: CMP EBX, EAX
0x80872b7: MOV EDX, 0x1
0x80872bc: JZ 0x080872e4
0x80872be: SUB ESP, 0x8
0x80872c1: PUSH 0x82ab0ba
0x80872c6: PUSH dword ptr [EBP + 0x8]
0x80872c9: CALL 0x080b8800
0x80872ce: ADD ESP, 0x10
0x80872d1: TEST EAX, EAX
0x80872d3: JNZ 0x080872fc
0x80872d5: SUB ESP, 0xc
0x80872d8: PUSH 0x82ab8b8
0x80872dd: CALL 0x08097acc
0x80872e2: XOR EDX, EDX
0x80872e4: LEA ESP, [EBP + -0xc]
0x80872e7: POP EBX
0x80872e8: POP ESI
0x80872e9: MOV EAX, EDX
0x80872eb: POP EDI
0x80872ec: LEAVE
0x80872ed: RET
0x80872f0: MOV EBX, dword ptr [0x0847aad0]
0x80872f6: MOV EAX, EBX
0x80872f8: JMP 0x080872b5
0x80872fc: SUB ESP, 0x8
0x80872ff: LEA EAX, [EBX + 0x14]
0x8087302: PUSH EAX
0x8087303: PUSH dword ptr [EBP + 0x8]
0x8087306: CALL 0x080b8b44
0x808730b: ADD ESP, 0x10
0x808730e: TEST EAX, EAX
0x8087310: JZ 0x08087490
0x8087316: MOV DX, word ptr [EBX + 0x14]
0x808731a: TEST DX, DX
0x808731d: JLE 0x08087490
0x8087323: SUB ESP, 0xc
0x8087326: PUSH 0x19cd
0x808732b: MOVSX EAX, DX
0x808732e: PUSH 0x82ab14c
0x8087333: LEA EAX, [EAX + EAX*0x2]
0x8087336: PUSH 0x82a376b
0x808733b: SHL EAX, 0x2
0x808733e: PUSH EAX
0x808733f: PUSH dword ptr [EBX + 0x18]
0x8087342: CALL 0x080cf588
0x8087347: ADD ESP, 0x20
0x808734a: TEST EAX, EAX
0x808734c: JZ 0x080873b3
0x808734e: MOVSX ECX, word ptr [EBX + 0x14]
0x8087352: MOV word ptr [EBX + 0x12], CX
0x8087356: LEA ECX, [ECX + ECX*0x2]
0x8087359: LEA ECX, [ECX*0x4 + 0xfffffff4]
0x8087360: MOV dword ptr [EBX + 0x18], EAX
0x8087363: SUB ESP, 0xc
0x8087366: LEA EDI, [EAX + 0xc]
0x8087369: CLD
0x808736a: XOR EAX, EAX
0x808736c: SHR ECX, 0x2
0x808736f: STOSD.REP ES:EDI
0x8087371: PUSH 0x19dc
0x8087376: PUSH 0x82ab14c
0x808737b: PUSH 0x82a376b
0x8087380: MOVSX EAX, word ptr [EBX + 0x14]
0x8087384: SHL EAX, 0x1
0x8087386: PUSH EAX
0x8087387: PUSH dword ptr [EBX + 0x1c]
0x808738a: CALL 0x080cf588
0x808738f: ADD ESP, 0x20
0x8087392: TEST EAX, EAX
0x8087394: MOV EDX, EAX
0x8087396: JZ 0x080873b3
0x8087398: MOV dword ptr [EBX + 0x1c], EAX
0x808739b: MOVSX EAX, word ptr [EBX + 0x14]
0x808739f: LEA EAX, [EAX + EAX*0x1 + -0x2]
0x80873a3: PUSH ECX
0x80873a4: PUSH EAX
0x80873a5: PUSH 0x0
0x80873a7: ADD EDX, 0x2
0x80873aa: PUSH EDX
0x80873ab: CALL 0x0804ba54
0x80873b0: ADD ESP, 0x10
0x80873b3: SUB ESP, 0x8
0x80873b6: LEA EAX, [EBP + -0x10]
0x80873b9: PUSH EAX
0x80873ba: PUSH dword ptr [EBP + 0x8]
0x80873bd: CALL 0x080b8bf4
0x80873c2: ADD ESP, 0x10
0x80873c5: TEST EAX, EAX
0x80873c7: JZ 0x0808749d
0x80873cd: MOV EAX, dword ptr [EBP + -0x10]
0x80873d0: TEST EAX, EAX
0x80873d2: JS 0x0808749d
0x80873d8: XOR ESI, ESI
0x80873da: TEST EAX, EAX
0x80873dc: JLE 0x0808743f
0x80873de: MOV ESI, ESI
0x80873e0: SUB ESP, 0x8
0x80873e3: LEA EAX, [EBP + -0x14]
0x80873e6: PUSH EAX
0x80873e7: PUSH dword ptr [EBP + 0x8]
0x80873ea: CALL 0x080b8c4c
0x80873ef: ADD ESP, 0x10
0x80873f2: TEST EAX, EAX
0x80873f4: JZ 0x080874b8
0x80873fa: SUB ESP, 0x8
0x80873fd: LEA EAX, [EBP + -0x18]
0x8087400: PUSH EAX
0x8087401: PUSH dword ptr [EBP + -0x14]
0x8087404: CALL 0x0807f5b4
0x8087409: MOV EDI, EAX
0x808740b: POP EAX
0x808740c: PUSH dword ptr [EBP + -0x14]
0x808740f: CALL 0x080891d4
0x8087414: ADD ESP, 0x10
0x8087417: TEST EDI, EDI
0x8087419: JZ 0x080874b8
0x808741f: SUB ESP, 0x8
0x8087422: PUSH EDI
0x8087423: PUSH dword ptr [EBP + 0x8]
0x8087426: CALL 0x080a2b58
0x808742b: ADD ESP, 0x10
0x808742e: TEST EAX, EAX
0x8087430: JZ 0x080874c5
0x8087436: INC ESI
0x8087437: MOVSX EAX, SI
0x808743a: CMP EAX, dword ptr [EBP + -0x10]
0x808743d: JL 0x080873e0
0x808743f: MOV DX, word ptr [EBX + 0x14]
0x8087443: XOR ESI, ESI
0x8087445: CMP DX, 0x0
0x8087449: JLE 0x08087460
0x808744b: MOV EAX, dword ptr [EBX + 0x18]
0x808744e: XOR ECX, ECX
0x8087450: MOV EDI, dword ptr [EAX]
0x8087452: TEST EDI, EDI
0x8087454: JZ 0x080874aa
0x8087456: INC ESI
0x8087457: INC ECX
0x8087458: ADD EAX, 0xc
0x808745b: CMP DX, SI
0x808745e: JG 0x08087450
0x8087460: SUB ESP, 0x8
0x8087463: PUSH 0x82ab0c3
0x8087468: PUSH dword ptr [EBP + 0x8]
0x808746b: CALL 0x080b8800
0x8087470: ADD ESP, 0x10
0x8087473: TEST EAX, EAX
0x8087475: MOV EDX, 0x1
0x808747a: JNZ 0x080872e4
0x8087480: SUB ESP, 0xc
0x8087483: PUSH 0x82ab8f0
0x8087488: JMP 0x080872dd
0x8087490: SUB ESP, 0xc
0x8087493: PUSH 0x82ab928
0x8087498: JMP 0x080872dd
0x808749d: SUB ESP, 0xc
0x80874a0: PUSH 0x82ab964
0x80874a5: JMP 0x080872dd
0x80874aa: SUB ESP, 0x8
0x80874ad: PUSH ECX
0x80874ae: PUSH 0x82ab998
0x80874b3: JMP 0x080872dd
0x80874b8: SUB ESP, 0xc
0x80874bb: PUSH 0x82ab9dc
0x80874c0: JMP 0x080872dd
0x80874c5: SUB ESP, 0xc
0x80874c8: PUSH 0x82aba18
0x80874cd: JMP 0x080872dd
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_initTinyXmlEfun"></a>

## vm_initTinyXmlEfun @ 0x81673c0

Evidence: `ev_af94176c9cda62c8068c39bbb3d3ca76fc01a181379c1fc27cea60d8f23a9e4c`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_initTinyXmlEfun(void)

{
  vm_tinyXmlInit();
  vm_updateEfuns();
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x81673c0: PUSH EBP
0x81673c1: MOV EBP, ESP
0x81673c3: SUB ESP, 0x8
0x81673c6: CALL 0x08168000
0x81673cb: CALL 0x0808600c
0x81673d0: MOV EAX, 0x1
0x81673d5: LEAVE
0x81673d6: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_efunInit"></a>

## vm_efunInit @ 0x80d41fc

Evidence: `ev_b2d45cb973e52b4210ddcc74678d5047c43df09614a8155ae610ba664dbfaf62`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_efunInit(void)

{
  vm_addEfuns(efuns);
  time(&vm_startTime);
  srand(vm_startTime);
  vm_addPredefineInt("LPC_DRIVER",0x71e);
  vm_addPredefineInt("VALUE_SIZE",0x20);
  vm_addPredefineInt("TYPE_UNDEFINED",0);
  vm_addPredefineInt("TYPE_VOID",1);
  vm_addPredefineInt("TYPE_NUMBER",2);
  vm_addPredefineInt("TYPE_STRING",4);
  vm_addPredefineInt("TYPE_BUFFER",5);
  vm_addPredefineInt("TYPE_ARRAY",6);
  vm_addPredefineInt("TYPE_MAPPING",7);
  vm_addPredefineInt("TYPE_REAL",3);
  vm_addPredefineInt("TYPE_VPTR",8);
  vm_addPredefineInt("TYPE_FUNCTION",9);
  vm_addPredefineInt("TYPE_OBJECT",10);
  vm_addPredefineInt("TYPE_MIXED",0xff);
  vm_addPredefineInt("ASCENDING",1);
  vm_addPredefineInt("DESCENDING",-1);
  vm_addPredefineInt("ASCENDING_STABLE",2);
  vm_addPredefineInt("DESCENDING_STABLE",-2);
  vm_addPredefineInt("TRIM_LEFT",1);
  vm_addPredefineInt("TRIM_RIGHT",2);
  vm_addPredefineInt("TRIM_BOTH",3);
  vm_addPredefineInt("INFORMATIVE",1);
  vm_addPredefineInt("MINOR",2);
  vm_addPredefineInt("MAJOR",3);
  vm_addPredefineInt("CRITICAL",4);
  vm_addPredefineInt("CTIME_REGULAR_STYLE",0);
  vm_addPredefineInt("CTIME_COMPACT_STYLE",1);
  vm_addPredefineInt("CTIME_COMPACT_DATE",2);
  vm_addPredefineInt("CTIME_NUMERIC_STYLE",3);
  vm_addPredefineInt("NO_RUNTIME_DATA",0);
  vm_addPredefineInt("INCLUDE_RUNTIME_DATA",1);
  vm_addPredefine("clean_mapping",0,-1,"clean_mapping_by_key");
  vm_addPredefine("pack_mapping",0,-1,"clean_mapping_by_value");
  return;
}


```

<details><summary>汇编</summary>

```asm
0x80d41fc: PUSH EBP
0x80d41fd: MOV EBP, ESP
0x80d41ff: SUB ESP, 0x14
0x80d4202: PUSH 0x83d01a0
0x80d4207: CALL 0x08085ce4
0x80d420c: MOV dword ptr [ESP], 0x860d2ac
0x80d4213: CALL 0x0804b4d4
0x80d4218: POP ECX
0x80d4219: PUSH dword ptr [0x0860d2ac]
0x80d421f: CALL 0x0804b8f4
0x80d4224: POP EAX
0x80d4225: POP EDX
0x80d4226: PUSH 0x71e
0x80d422b: PUSH 0x82b6cf8
0x80d4230: CALL 0x08080784
0x80d4235: POP ECX
0x80d4236: POP EAX
0x80d4237: PUSH 0x20
0x80d4239: PUSH 0x82b6d03
0x80d423e: CALL 0x08080784
0x80d4243: POP EAX
0x80d4244: POP EDX
0x80d4245: PUSH 0x0
0x80d4247: PUSH 0x82b6d0e
0x80d424c: CALL 0x08080784
0x80d4251: POP ECX
0x80d4252: POP EAX
0x80d4253: PUSH 0x1
0x80d4255: PUSH 0x82b6d1d
0x80d425a: CALL 0x08080784
0x80d425f: POP EAX
0x80d4260: POP EDX
0x80d4261: PUSH 0x2
0x80d4263: PUSH 0x82b6d27
0x80d4268: CALL 0x08080784
0x80d426d: POP ECX
0x80d426e: POP EAX
0x80d426f: PUSH 0x4
0x80d4271: PUSH 0x82b6d33
0x80d4276: CALL 0x08080784
0x80d427b: POP EAX
0x80d427c: POP EDX
0x80d427d: PUSH 0x5
0x80d427f: PUSH 0x82b6d3f
0x80d4284: CALL 0x08080784
0x80d4289: POP ECX
0x80d428a: POP EAX
0x80d428b: PUSH 0x6
0x80d428d: PUSH 0x82b6d4b
0x80d4292: CALL 0x08080784
0x80d4297: POP EAX
0x80d4298: POP EDX
0x80d4299: PUSH 0x7
0x80d429b: PUSH 0x82b6d56
0x80d42a0: CALL 0x08080784
0x80d42a5: POP ECX
0x80d42a6: POP EAX
0x80d42a7: PUSH 0x3
0x80d42a9: PUSH 0x82b6d63
0x80d42ae: CALL 0x08080784
0x80d42b3: POP EAX
0x80d42b4: POP EDX
0x80d42b5: PUSH 0x8
0x80d42b7: PUSH 0x82b6d6d
0x80d42bc: CALL 0x08080784
0x80d42c1: POP ECX
0x80d42c2: POP EAX
0x80d42c3: PUSH 0x9
0x80d42c5: PUSH 0x82b6d77
0x80d42ca: CALL 0x08080784
0x80d42cf: POP EAX
0x80d42d0: POP EDX
0x80d42d1: PUSH 0xa
0x80d42d3: PUSH 0x82b6d85
0x80d42d8: CALL 0x08080784
0x80d42dd: POP ECX
0x80d42de: POP EAX
0x80d42df: PUSH 0xff
0x80d42e4: PUSH 0x82b6d91
0x80d42e9: CALL 0x08080784
0x80d42ee: POP EAX
0x80d42ef: POP EDX
0x80d42f0: PUSH 0x1
0x80d42f2: PUSH 0x82b6d9c
0x80d42f7: CALL 0x08080784
0x80d42fc: POP ECX
0x80d42fd: POP EAX
0x80d42fe: PUSH -0x1
0x80d4300: PUSH 0x82b6da6
0x80d4305: CALL 0x08080784
0x80d430a: POP EAX
0x80d430b: POP EDX
0x80d430c: PUSH 0x2
0x80d430e: PUSH 0x82b6db1
0x80d4313: CALL 0x08080784
0x80d4318: POP ECX
0x80d4319: POP EAX
0x80d431a: PUSH -0x2
0x80d431c: PUSH 0x82b6dc2
0x80d4321: CALL 0x08080784
0x80d4326: POP EAX
0x80d4327: POP EDX
0x80d4328: PUSH 0x1
0x80d432a: PUSH 0x82b6dd4
0x80d432f: CALL 0x08080784
0x80d4334: POP ECX
0x80d4335: POP EAX
0x80d4336: PUSH 0x2
0x80d4338: PUSH 0x82b6dde
0x80d433d: CALL 0x08080784
0x80d4342: POP EAX
0x80d4343: POP EDX
0x80d4344: PUSH 0x3
0x80d4346: PUSH 0x82b6de9
0x80d434b: CALL 0x08080784
0x80d4350: POP ECX
0x80d4351: POP EAX
0x80d4352: PUSH 0x1
0x80d4354: PUSH 0x82b24eb
0x80d4359: CALL 0x08080784
0x80d435e: POP EAX
0x80d435f: POP EDX
0x80d4360: PUSH 0x2
0x80d4362: PUSH 0x82b24f7
0x80d4367: CALL 0x08080784
0x80d436c: POP ECX
0x80d436d: POP EAX
0x80d436e: PUSH 0x3
0x80d4370: PUSH 0x82b24fd
0x80d4375: CALL 0x08080784
0x80d437a: POP EAX
0x80d437b: POP EDX
0x80d437c: PUSH 0x4
0x80d437e: PUSH 0x82bd8b1
0x80d4383: CALL 0x08080784
0x80d4388: POP ECX
0x80d4389: POP EAX
0x80d438a: PUSH 0x0
0x80d438c: PUSH 0x82b6df3
0x80d4391: CALL 0x08080784
0x80d4396: POP EAX
0x80d4397: POP EDX
0x80d4398: PUSH 0x1
0x80d439a: PUSH 0x82b6e07
0x80d439f: CALL 0x08080784
0x80d43a4: POP ECX
0x80d43a5: POP EAX
0x80d43a6: PUSH 0x2
0x80d43a8: PUSH 0x82b6e1b
0x80d43ad: CALL 0x08080784
0x80d43b2: POP EAX
0x80d43b3: POP EDX
0x80d43b4: PUSH 0x3
0x80d43b6: PUSH 0x82b6e2e
0x80d43bb: CALL 0x08080784
0x80d43c0: POP ECX
0x80d43c1: POP EAX
0x80d43c2: PUSH 0x0
0x80d43c4: PUSH 0x82b6e42
0x80d43c9: CALL 0x08080784
0x80d43ce: POP EAX
0x80d43cf: POP EDX
0x80d43d0: PUSH 0x1
0x80d43d2: PUSH 0x82b6e52
0x80d43d7: CALL 0x08080784
0x80d43dc: PUSH 0x82b6756
0x80d43e1: PUSH -0x1
0x80d43e3: PUSH 0x0
0x80d43e5: PUSH 0x82b6e67
0x80d43ea: CALL 0x08080128
0x80d43ef: ADD ESP, 0x20
0x80d43f2: PUSH 0x82b676b
0x80d43f7: PUSH -0x1
0x80d43f9: PUSH 0x0
0x80d43fb: PUSH 0x82b6e75
0x80d4400: CALL 0x08080128
0x80d4405: LEAVE
0x80d4406: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_packedFsFetch"></a>

## vm_packedFsFetch @ 0x80b40c0

Evidence: `ev_b366b2140b2fa506c432a2677891e3265aeb0cf5a913f43e08a0ff23e2a104a3`

```c

int vm_packedFsFetch(char *pFileName,void *ppRet,int *pSize,uint *pAttrib)

{
  char cVar1;
  char *pcVar2;
  int iVar3;
  uint uVar4;
  char *pcVar5;
  char *p2;
  char path [256];
  char packedName [256];
  
                    /* Unresolved local var: uint fileNameLen@[DW_OP_reg3(EBX)]
                       Unresolved local var: char * p@[???] */
  pcVar2 = strstr(pFileName,".pak/");
  if (pcVar2 == (char *)0x0) {
    uVar4 = 0xffffffff;
    pcVar2 = pFileName;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    pcVar2 = (char *)(~uVar4 - 1);
    p2 = pFileName + (int)pcVar2;
  }
  else {
    p2 = pcVar2 + 5;
    pcVar2 = pcVar2 + (4 - (int)pFileName);
  }
  if (pcVar2 < (char *)0x100) {
    uVar4 = 0xffffffff;
    pcVar5 = p2;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar5;
      pcVar5 = pcVar5 + 1;
    } while (cVar1 != '\0');
    if (~uVar4 - 1 < 0x100) {
      memcpy(packedName,pFileName,(size_t)pcVar2);
      packedName[(int)pcVar2] = '\0';
      strcpy(path,p2);
      iVar3 = fm_unpack(packedName,path,ppRet,pSize,pAttrib);
      return (uint)(iVar3 != 0);
    }
  }
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x80b40c0: PUSH EBP
0x80b40c1: MOV EBP, ESP
0x80b40c3: PUSH EDI
0x80b40c4: PUSH ESI
0x80b40c5: PUSH EBX
0x80b40c6: SUB ESP, 0x224
0x80b40cc: MOV ESI, dword ptr [EBP + 0x8]
0x80b40cf: PUSH 0x82af69a
0x80b40d4: PUSH ESI
0x80b40d5: CALL 0x0804b554
0x80b40da: ADD ESP, 0x10
0x80b40dd: TEST EAX, EAX
0x80b40df: MOV EDX, EAX
0x80b40e1: JZ 0x080b4180
0x80b40e7: SUB EAX, ESI
0x80b40e9: ADD EDX, 0x5
0x80b40ec: LEA EBX, [EAX + 0x4]
0x80b40ef: MOV dword ptr [EBP + 0xfffffde4], EDX
0x80b40f5: CMP EBX, 0x100
0x80b40fb: JNC 0x080b4118
0x80b40fd: XOR EAX, EAX
0x80b40ff: CLD
0x80b4100: MOV ECX, 0xffffffff
0x80b4105: MOV EDI, dword ptr [EBP + 0xfffffde4]
0x80b410b: SCASB.REPNE ES:EDI
0x80b410d: NOT ECX
0x80b410f: DEC ECX
0x80b4110: CMP ECX, 0x100
0x80b4116: JC 0x080b4124
0x80b4118: LEA ESP, [EBP + -0xc]
0x80b411b: POP EBX
0x80b411c: POP ESI
0x80b411d: XOR EAX, EAX
0x80b411f: POP EDI
0x80b4120: LEAVE
0x80b4121: RET
0x80b4124: PUSH EDI
0x80b4125: PUSH EBX
0x80b4126: PUSH ESI
0x80b4127: LEA EAX, [EBP + 0xfffffee8]
0x80b412d: PUSH EAX
0x80b412e: CALL 0x0804b7e4
0x80b4133: POP EDX
0x80b4134: POP ECX
0x80b4135: MOV byte ptr [EBP + EBX*0x1 + 0xfffffee8], 0x0
0x80b413d: PUSH dword ptr [EBP + 0xfffffde4]
0x80b4143: LEA EAX, [EBP + 0xfffffde8]
0x80b4149: PUSH EAX
0x80b414a: CALL 0x0804bc54
0x80b414f: POP EAX
0x80b4150: PUSH dword ptr [EBP + 0x14]
0x80b4153: PUSH dword ptr [EBP + 0x10]
0x80b4156: PUSH dword ptr [EBP + 0xc]
0x80b4159: LEA EAX, [EBP + 0xfffffde8]
0x80b415f: PUSH EAX
0x80b4160: LEA EAX, [EBP + 0xfffffee8]
0x80b4166: PUSH EAX
0x80b4167: CALL 0x080b5500
0x80b416c: ADD ESP, 0x20
0x80b416f: TEST EAX, EAX
0x80b4171: SETNZ AL
0x80b4174: LEA ESP, [EBP + -0xc]
0x80b4177: POP EBX
0x80b4178: POP ESI
0x80b4179: MOVZX EAX, AL
0x80b417c: POP EDI
0x80b417d: LEAVE
0x80b417e: RET
0x80b4180: XOR EAX, EAX
0x80b4182: CLD
0x80b4183: MOV ECX, 0xffffffff
0x80b4188: MOV EDI, ESI
0x80b418a: SCASB.REPNE ES:EDI
0x80b418c: NOT ECX
0x80b418e: LEA EBX, [ECX + -0x1]
0x80b4191: LEA EAX, [ESI + EBX*0x1]
0x80b4194: MOV dword ptr [EBP + 0xfffffde4], EAX
0x80b419a: JMP 0x080b40f5
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadFun"></a>

## vm_loadFun @ 0x80959b8

Evidence: `ev_b513e6242f5447517c22636c5c877a93b397b974851b7928f195106d957a1079`

```c

int vm_loadFun(Vm_Binary_Buffer_T *pInput,Vm_Fun_T **ppFun)

{
  Vm_Fun_Arg_T *pVVar1;
  Vm_Fun_T *pVVar2;
  int iVar3;
  Vm_Fun_Arg_T **ppVVar4;
  Vm_Code_Segment_T *pVVar5;
  char *pcVar6;
  Vm_Index_T n;
  Vm_Fun_T *pFun;
  
                    /* Unresolved local var: Vm_Fun_Arg_T * * ppArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * * ppVar@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pVar@[DW_OP_reg3(EBX)] */
  iVar3 = vm_loadBuffer(pInput,&pFun,0x40);
  if (iVar3 == 0) {
    pcVar6 = "Load function failed: structure error.\n";
    goto LAB_080959df;
  }
  pFun->name = (char *)0x0;
  pFun->pArgs = (Vm_Fun_Arg_T *)0x0;
  pFun->pLabels = (Vm_Local_Label_T *)0x0;
  pFun->pNext = (Vm_Fun *)0x0;
  pFun->pVars = (Vm_Fun_Arg_T *)0x0;
  pVVar5 = &pFun->codeSegment;
  for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
    pVVar5->nodesCount = 0;
    pVVar5 = (Vm_Code_Segment_T *)&pVVar5->nodes;
  }
  iVar3 = vm_isKeepSymbol();
  if (iVar3 == 0) {
    iVar3 = vm_loadTempString(pInput,(char **)0x0);
    pVVar2 = pFun;
    if (iVar3 != 0) {
      pcVar6 = vm_scratchLargeAlloc(4);
      pVVar2->name = pcVar6;
      *pFun->name = '?';
      pFun->name[1] = '?';
      pFun->name[2] = '?';
      pFun->name[3] = '\0';
      iVar3 = vm_loadComment(pInput,"ARGUMENTS");
joined_r0x08095a6e:
      if (iVar3 == 0) {
        pcVar6 = "Load function failed: comment \"ARGUMENTS\" mismatched.\n";
      }
      else {
        iVar3 = vm_loadVmIndex(pInput,&n);
        if (iVar3 == 0) {
          pcVar6 = "Load function failed: argument count error.\n";
        }
        else {
          ppVVar4 = &pFun->pArgs;
          while (0 < n) {
            n = n + -1;
            iVar3 = vm_loadBuffer(pInput,ppVVar4,0x14);
            if (iVar3 == 0) {
              pcVar6 = "Load function failed: argument structure error.\n";
              goto LAB_080959df;
            }
            pVVar1 = *ppVVar4;
            pVVar1->name = (char *)0x0;
            pVVar1->pDefaultValue = (Vm_XNode *)0x0;
            pVVar1->pNext = (Vm_Fun_Arg *)0x0;
            iVar3 = vm_loadString(pInput,&pVVar1->name);
            if (iVar3 == 0) {
              pcVar6 = "Load function failed: argument name error.\n";
              goto LAB_080959df;
            }
            ppVVar4 = &pVVar1->pNext;
          }
          n = n + -1;
          iVar3 = vm_loadVmIndex(pInput,&n);
          if (iVar3 == 0) {
            pcVar6 = "Load function failed: local variables count error.\n";
          }
          else {
            ppVVar4 = &pFun->pVars;
            while (0 < n) {
              n = n + -1;
              iVar3 = vm_loadBuffer(pInput,ppVVar4,0x14);
              if (iVar3 == 0) {
                pcVar6 = "Load function failed: Local variable structure error.\n";
                goto LAB_080959df;
              }
              pVVar1 = *ppVVar4;
              pVVar1->name = (char *)0x0;
              pVVar1->pDefaultValue = (Vm_XNode *)0x0;
              pVVar1->pNext = (Vm_Fun_Arg *)0x0;
              iVar3 = vm_loadString(pInput,&pVVar1->name);
              if (iVar3 == 0) {
                pcVar6 = "Load function failed: Varument name error.\n";
                goto LAB_080959df;
              }
              ppVVar4 = &pVVar1->pNext;
            }
            n = n + -1;
            iVar3 = vm_isKeepSymbol();
            if (iVar3 == 0) {
              while (pVVar1 = pFun->pVars, pVVar1 != (Vm_Fun_Arg_T *)0x0) {
                pFun->pVars = pVVar1->pNext;
                vm_scratchFree(pVVar1->name);
                vm_scratchFree(pVVar1);
              }
            }
            iVar3 = vm_loadComment(pInput,"ACTIONS");
            if (iVar3 == 0) {
              pcVar6 = "Load function failed: comment \"ACTIONS\" mismatched.\n";
            }
            else {
              iVar3 = vm_loadCodeSegment(pInput,pFun,&pFun->codeSegment);
              if (iVar3 != 0) {
                *ppFun = pFun;
                return 1;
              }
              pcVar6 = "Load function failed: actions error.\n";
            }
          }
        }
      }
      goto LAB_080959df;
    }
  }
  else {
    iVar3 = vm_loadString(pInput,&pFun->name);
    if (iVar3 != 0) {
      iVar3 = vm_loadComment(pInput,"ARGUMENTS");
      goto joined_r0x08095a6e;
    }
  }
  pcVar6 = "Load function failed: name error.\n";
LAB_080959df:
  vm_errprintf(pcVar6);
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x80959b8: PUSH EBP
0x80959b9: MOV EBP, ESP
0x80959bb: PUSH EDI
0x80959bc: PUSH ESI
0x80959bd: PUSH EBX
0x80959be: SUB ESP, 0x10
0x80959c1: PUSH 0x40
0x80959c3: LEA EAX, [EBP + -0x10]
0x80959c6: PUSH EAX
0x80959c7: MOV ESI, dword ptr [EBP + 0x8]
0x80959ca: PUSH ESI
0x80959cb: CALL 0x080b8a70
0x80959d0: ADD ESP, 0x10
0x80959d3: TEST EAX, EAX
0x80959d5: JNZ 0x080959f0
0x80959d7: SUB ESP, 0xc
0x80959da: PUSH 0x82ae94c
0x80959df: CALL 0x08097acc
0x80959e4: XOR EAX, EAX
0x80959e6: LEA ESP, [EBP + -0xc]
0x80959e9: POP EBX
0x80959ea: POP ESI
0x80959eb: POP EDI
0x80959ec: LEAVE
0x80959ed: RET
0x80959f0: MOV EDI, dword ptr [EBP + -0x10]
0x80959f3: MOV dword ptr [EDI], 0x0
0x80959f9: MOV dword ptr [EDI + 0x30], 0x0
0x8095a00: MOV dword ptr [EDI + 0x38], 0x0
0x8095a07: MOV dword ptr [EDI + 0x3c], 0x0
0x8095a0e: MOV dword ptr [EDI + 0x34], 0x0
0x8095a15: XOR EAX, EAX
0x8095a17: ADD EDI, 0x10
0x8095a1a: CLD
0x8095a1b: MOV ECX, 0x8
0x8095a20: STOSD.REP ES:EDI
0x8095a22: CALL 0x080b5bd0
0x8095a27: TEST EAX, EAX
0x8095a29: JNZ 0x08095a48
0x8095a2b: SUB ESP, 0x8
0x8095a2e: PUSH 0x0
0x8095a30: PUSH ESI
0x8095a31: CALL 0x080b8c4c
0x8095a36: ADD ESP, 0x10
0x8095a39: TEST EAX, EAX
0x8095a3b: JNZ 0x08095a80
0x8095a3d: SUB ESP, 0xc
0x8095a40: PUSH 0x82ae974
0x8095a45: JMP 0x080959df
0x8095a48: SUB ESP, 0x8
0x8095a4b: PUSH dword ptr [EBP + -0x10]
0x8095a4e: PUSH ESI
0x8095a4f: CALL 0x080b8dc8
0x8095a54: ADD ESP, 0x10
0x8095a57: TEST EAX, EAX
0x8095a59: JZ 0x08095a3d
0x8095a5b: SUB ESP, 0x8
0x8095a5e: PUSH 0x82ae15e
0x8095a63: PUSH ESI
0x8095a64: CALL 0x080b8800
0x8095a69: ADD ESP, 0x10
0x8095a6c: TEST EAX, EAX
0x8095a6e: JNZ 0x08095aca
0x8095a70: SUB ESP, 0xc
0x8095a73: PUSH 0x82ae998
0x8095a78: JMP 0x080959df
0x8095a80: SUB ESP, 0xc
0x8095a83: PUSH 0x4
0x8095a85: MOV EBX, dword ptr [EBP + -0x10]
0x8095a88: CALL 0x08089630
0x8095a8d: MOV dword ptr [EBX], EAX
0x8095a8f: MOV EAX, dword ptr [EBP + -0x10]
0x8095a92: MOV EAX, dword ptr [EAX]
0x8095a94: MOV byte ptr [EAX], 0x3f
0x8095a97: MOV EAX, dword ptr [EBP + -0x10]
0x8095a9a: MOV EAX, dword ptr [EAX]
0x8095a9c: MOV byte ptr [EAX + 0x1], 0x3f
0x8095aa0: MOV EAX, dword ptr [EBP + -0x10]
0x8095aa3: MOV EAX, dword ptr [EAX]
0x8095aa5: MOV byte ptr [EAX + 0x2], 0x3f
0x8095aa9: MOV EAX, dword ptr [EBP + -0x10]
0x8095aac: ADD ESP, 0x10
0x8095aaf: SUB ESP, 0x8
0x8095ab2: MOV EAX, dword ptr [EAX]
0x8095ab4: MOV byte ptr [EAX + 0x3], 0x0
0x8095ab8: PUSH 0x82ae15e
0x8095abd: PUSH ESI
0x8095abe: CALL 0x080b8800
0x8095ac3: ADD ESP, 0x10
0x8095ac6: TEST EAX, EAX
0x8095ac8: JZ 0x08095a70
0x8095aca: SUB ESP, 0x8
0x8095acd: LEA EDI, [EBP + -0x12]
0x8095ad0: PUSH EDI
0x8095ad1: PUSH ESI
0x8095ad2: CALL 0x080b8b44
0x8095ad7: ADD ESP, 0x10
0x8095ada: TEST EAX, EAX
0x8095adc: JZ 0x08095c10
0x8095ae2: MOV EBX, dword ptr [EBP + -0x10]
0x8095ae5: ADD EBX, 0x30
0x8095ae8: JMP 0x08095b2f
0x8095aec: PUSH EAX
0x8095aed: PUSH 0x14
0x8095aef: PUSH EBX
0x8095af0: PUSH ESI
0x8095af1: CALL 0x080b8a70
0x8095af6: ADD ESP, 0x10
0x8095af9: TEST EAX, EAX
0x8095afb: JZ 0x08095c1d
0x8095b01: MOV EBX, dword ptr [EBX]
0x8095b03: SUB ESP, 0x8
0x8095b06: MOV dword ptr [EBX], 0x0
0x8095b0c: MOV dword ptr [EBX + 0xc], 0x0
0x8095b13: MOV dword ptr [EBX + 0x10], 0x0
0x8095b1a: PUSH EBX
0x8095b1b: PUSH ESI
0x8095b1c: CALL 0x080b8dc8
0x8095b21: ADD ESP, 0x10
0x8095b24: TEST EAX, EAX
0x8095b26: JZ 0x08095c2a
0x8095b2c: ADD EBX, 0x10
0x8095b2f: MOV DX, word ptr [EBP + -0x12]
0x8095b33: LEA EAX, [EDX + -0x1]
0x8095b36: TEST DX, DX
0x8095b39: MOV word ptr [EBP + -0x12], AX
0x8095b3d: JG 0x08095aec
0x8095b3f: SUB ESP, 0x8
0x8095b42: PUSH EDI
0x8095b43: PUSH ESI
0x8095b44: CALL 0x080b8b44
0x8095b49: ADD ESP, 0x10
0x8095b4c: TEST EAX, EAX
0x8095b4e: JZ 0x08095c03
0x8095b54: MOV EBX, dword ptr [EBP + -0x10]
0x8095b57: ADD EBX, 0x34
0x8095b5a: JMP 0x08095b9f
0x8095b5c: PUSH EAX
0x8095b5d: PUSH 0x14
0x8095b5f: PUSH EBX
0x8095b60: PUSH ESI
0x8095b61: CALL 0x080b8a70
0x8095b66: ADD ESP, 0x10
0x8095b69: TEST EAX, EAX
0x8095b6b: JZ 0x08095c5f
0x8095b71: MOV EBX, dword ptr [EBX]
0x8095b73: SUB ESP, 0x8
0x8095b76: MOV dword ptr [EBX], 0x0
0x8095b7c: MOV dword ptr [EBX + 0xc], 0x0
0x8095b83: MOV dword ptr [EBX + 0x10], 0x0
0x8095b8a: PUSH EBX
0x8095b8b: PUSH ESI
0x8095b8c: CALL 0x080b8dc8
0x8095b91: ADD ESP, 0x10
0x8095b94: TEST EAX, EAX
0x8095b96: JZ 0x08095c6c
0x8095b9c: ADD EBX, 0x10
0x8095b9f: MOV DX, word ptr [EBP + -0x12]
0x8095ba3: LEA EAX, [EDX + -0x1]
0x8095ba6: TEST DX, DX
0x8095ba9: MOV word ptr [EBP + -0x12], AX
0x8095bad: JG 0x08095b5c
0x8095baf: CALL 0x080b5bd0
0x8095bb4: TEST EAX, EAX
0x8095bb6: JNZ 0x08095be1
0x8095bb8: JMP 0x08095bd7
0x8095bbc: SUB ESP, 0xc
0x8095bbf: MOV EAX, dword ptr [EBX + 0x10]
0x8095bc2: MOV dword ptr [EDX + 0x34], EAX
0x8095bc5: PUSH dword ptr [EBX]
0x8095bc7: CALL 0x080891d4
0x8095bcc: MOV dword ptr [ESP], EBX
0x8095bcf: CALL 0x080891d4
0x8095bd4: ADD ESP, 0x10
0x8095bd7: MOV EDX, dword ptr [EBP + -0x10]
0x8095bda: MOV EBX, dword ptr [EDX + 0x34]
0x8095bdd: TEST EBX, EBX
0x8095bdf: JNZ 0x08095bbc
0x8095be1: SUB ESP, 0x8
0x8095be4: PUSH 0x82ae168
0x8095be9: PUSH ESI
0x8095bea: CALL 0x080b8800
0x8095bef: ADD ESP, 0x10
0x8095bf2: TEST EAX, EAX
0x8095bf4: JNZ 0x08095c37
0x8095bf6: SUB ESP, 0xc
0x8095bf9: PUSH 0x82ae9d0
0x8095bfe: JMP 0x080959df
0x8095c03: SUB ESP, 0xc
0x8095c06: PUSH 0x82aea08
0x8095c0b: JMP 0x080959df
0x8095c10: SUB ESP, 0xc
0x8095c13: PUSH 0x82aea3c
0x8095c18: JMP 0x080959df
0x8095c1d: SUB ESP, 0xc
0x8095c20: PUSH 0x82aea6c
0x8095c25: JMP 0x080959df
0x8095c2a: SUB ESP, 0xc
0x8095c2d: PUSH 0x82aeaa0
0x8095c32: JMP 0x080959df
0x8095c37: PUSH EAX
0x8095c38: MOV EAX, dword ptr [EBP + -0x10]
0x8095c3b: LEA EDX, [EAX + 0x10]
0x8095c3e: PUSH EDX
0x8095c3f: PUSH EAX
0x8095c40: PUSH ESI
0x8095c41: CALL 0x08096128
0x8095c46: ADD ESP, 0x10
0x8095c49: TEST EAX, EAX
0x8095c4b: JZ 0x08095c79
0x8095c4d: MOV EAX, dword ptr [EBP + 0xc]
0x8095c50: MOV EDX, dword ptr [EBP + -0x10]
0x8095c53: MOV dword ptr [EAX], EDX
0x8095c55: MOV EAX, 0x1
0x8095c5a: JMP 0x080959e6
0x8095c5f: SUB ESP, 0xc
0x8095c62: PUSH 0x82aeacc
0x8095c67: JMP 0x080959df
0x8095c6c: SUB ESP, 0xc
0x8095c6f: PUSH 0x82aeb04
0x8095c74: JMP 0x080959df
0x8095c79: SUB ESP, 0xc
0x8095c7c: PUSH 0x82aeb30
0x8095c81: JMP 0x080959df
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadIdentifiers"></a>

## vm_loadIdentifiers @ 0x8087fb8

Evidence: `ev_b786ac699461a76c0683ddb2f8254575187db56f002088762956ac65e57e9528`

```c

int vm_loadIdentifiers(Vm_Binary_Buffer_T *pInput)

{
  short *psVar1;
  uint uVar2;
  Vm_Ident_Hash_Elem **ppVVar3;
  bool bVar4;
  Vm_Prog_Var_T **ppVVar5;
  short sVar6;
  ushort uVar7;
  short sVar8;
  int iVar9;
  Vm_Prog_Var_T **ppVVar10;
  Vm_Fun_Entry_T *pVVar11;
  Vm_Program_T *pVVar12;
  Vm_Ident_Hash_Elem *pVVar13;
  Vm_Ident_Info *pVVar14;
  uint uVar15;
  undefined4 uVar16;
  Vm_Prog_Var_T **extraout_ECX;
  Vm_Prog_Var_T **ppVVar17;
  Vm_Binary_Buffer_T *extraout_ECX_00;
  Vm_Binary_Buffer_T *extraout_ECX_01;
  Vm_Binary_Buffer_T *extraout_ECX_02;
  Vm_Binary_Buffer_T *extraout_ECX_03;
  int iVar18;
  Vm_Ident_Hash_Elem *pVVar19;
  char *pcVar20;
  char *str_00;
  char *pcVar21;
  Vm_Prog_Var_T *pVVar22;
  Vm_Fun_T **num;
  Vm_Binary_Buffer_T *pVVar23;
  Vm_Binary_Buffer_T *pVVar24;
  int h;
  Vm_Prog_Var_T **ppVar;
  int i;
  Vm_Fun_T *pFun;
  Vm_Fun_T *pHiddenFun;
  int count_1;
  Vm_Index_T functions;
  Vm_Index_T efunCounts;
  Vm_Prog_Var_T *pVar;
  Vm_Index_T variables;
  Vm_Index_T maxVariableNo;
  char *pNames;
  int nameSize;
  int count;
  char str [256];
  Vm_Dlt_T dlt;
  
  pcVar21 = "IDENTIFIERS";
  iVar9 = vm_loadComment(pInput,"IDENTIFIERS");
  if (iVar9 == 0) {
    pcVar20 = "Load identifiers failed: comment \"IDENTIFIERS\" mismatched.\n";
    goto LAB_08087fe1;
  }
  pcVar21 = "NAMES";
  iVar9 = vm_loadComment(pInput,"NAMES");
  if (iVar9 == 0) {
    pcVar20 = "Load identifiers failed: comment \"NAME\" mismatched.\n";
    goto LAB_08087fe1;
  }
  num = (Vm_Fun_T **)&count;
  pcVar21 = (char *)num;
  iVar9 = vm_loadNumber32(pInput,(int *)num);
  if ((iVar9 == 0) || (count < 0)) {
    pcVar20 = "Load identifiers failed: count error.\n";
    goto LAB_08087fe1;
  }
  pcVar21 = (char *)&nameSize;
  iVar9 = vm_loadNumber32(pInput,(int *)pcVar21);
  if ((iVar9 != 0) && (-1 < nameSize)) {
    pcVar21 = (char *)&pNames;
    iVar9 = vm_loadTempBuffer(pInput,pcVar21,nameSize);
    if (iVar9 != 0) {
                    /* Unresolved local var: Vm_Program_T * pProgram@[???] */
      pVVar12 = vm_pGlobalProgram;
      if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
        pVVar12 = vm_pCurrentProgram;
      }
      pcVar21 = "VARIABLES";
      iVar9 = vm_loadComment(pInput,"VARIABLES");
      if (iVar9 == 0) {
        pcVar20 = "Load variables failed: comment \"VARIABLES\" mismatched.\n";
        goto LAB_080880b2;
      }
      pcVar21 = (char *)&maxVariableNo;
      iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
      if ((iVar9 == 0) || ((int)_variables < 0)) {
LAB_08088203:
        pcVar20 = "Load variables failed: read variable error.\n";
      }
      else {
        pcVar21 = (char *)&variables;
        iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
        if ((iVar9 == 0) || (variables < 0)) goto LAB_08088203;
        if (pVVar12 == vm_pGlobalProgram) {
          iVar9 = (int)maxVariableNo;
          if (vm_globalVar <= maxVariableNo) goto LAB_080881c1;
          pcVar20 = "Load variables failed: bad variable count.\n";
        }
        else {
          pVVar12->maxVariableNo = maxVariableNo;
          pcVar21 = "PROG_VARS";
          iVar9 = vm_loadComment(pInput,"PROG_VARS");
          if (iVar9 != 0) {
            ppVVar5 = &pVVar12->pProgVars;
            ppVVar17 = extraout_ECX;
            while( true ) {
              ppVVar10 = ppVVar5;
              sVar6 = (short)_variables;
              _variables = (Vm_Fun_T *)CONCAT22(maxVariableNo,sVar6 + -1);
              iVar9 = vm_globalVar;
              if (sVar6 < 1) break;
              uVar16 = 0x10;
              pcVar21 = (char *)&pVar;
              iVar9 = vm_loadBuffer(pInput,pcVar21,0x10);
              if (iVar9 == 0) {
                pcVar20 = "Load variables failed: read program\'s variable error.\n";
                goto LAB_080880b2;
              }
              pVVar22 = pVar;
              iVar9 = vm_loadString(pInput,&pVar->pVarName);
              if (iVar9 == 0) {
                vm_errprintf("Load variables failed: read program\'s variable name error.\n",pVVar22
                             ,uVar16,ppVVar17);
                pcVar21 = &DAT_082a376b;
                pdb_freeMemory(pVar,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1bb9);
                goto LAB_080880b7;
              }
              pcVar21 = pVVar12->strName->ptr;
              pVar->pNext = (Vm_Prog_Var *)0x0;
              *ppVVar10 = pVar;
              pVar->pProgName = pcVar21;
              ppVVar5 = &pVar->pNext;
              ppVVar17 = ppVVar10;
            }
LAB_080881c1:
                    /* Unresolved local var: Vm_Program_T * pProgram@[???] */
            vm_globalVar = iVar9;
            pVVar12 = vm_pCurrentProgram;
            if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
              pVVar12 = vm_pGlobalProgram;
            }
            pcVar21 = "FUNCTIONS";
            iVar9 = vm_loadComment(pInput,"FUNCTIONS");
            if (iVar9 == 0) {
              pcVar20 = "Load functions failed: comment \"FUNCTIONS\" mismatched.\n";
            }
            else {
              pcVar21 = (char *)&efunCounts;
              iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
              if ((iVar9 == 0) || (vm_efunsCount != efunCounts)) {
                pcVar20 = "Load functions failed: read efuns error.\n";
              }
              else {
                pcVar21 = (char *)&functions;
                iVar9 = vm_loadVmIndex(pInput,(Vm_Index_T *)pcVar21);
                if ((iVar9 == 0) || (sVar6 = (short)_functions, sVar6 < 0)) {
                  pcVar20 = "Load functions failed: read functions error.\n";
                }
                else {
                  pVVar12->maxFunctionNo = sVar6;
                    /* Unresolved local var: Vm_Fun_Entry_T * pFunctions@[DW_OP_reg0(EAX)] */
                  if (pVVar12->pFunctions == (Vm_Fun_Entry_T *)0x0) {
                    pcVar21 = &DAT_082a376b;
                    pVVar11 = pdb_allocateMemory((int)sVar6 << 3,"VM",
                                                 "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19f3);
                  }
                  else {
                    pVVar11 = pdb_reallocateMemory
                                        (pVVar12->pFunctions,(int)sVar6 << 3,"VM",
                                         "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x19f7);
                  }
                  if (pVVar11 == (Vm_Fun_Entry_T *)0x0) {
                    pcVar20 = "Load identifiers failed: extend function table error.\n";
                  }
                  else {
                    iVar18 = (int)vm_functionBC;
                    pVVar12->pFunctions = pVVar11;
                    pVVar11 = pVVar11 + iVar18;
                    for (iVar9 = (pVVar12->maxFunctionNo - iVar18 & 0x1fffffffU) << 1; iVar9 != 0;
                        iVar9 = iVar9 + -1) {
                      pVVar11->pFun = (Vm_Fun *)0x0;
                      pVVar11 = (Vm_Fun_Entry_T *)&pVVar11->consititutionNo;
                    }
                    pcVar21 = "HIDDEN_FUNS";
                    iVar9 = vm_loadComment(pInput,"HIDDEN_FUNS");
                    if (iVar9 == 0) {
                      pcVar20 = "Load program failed: comment \"HIDDEN_FUNS\" mismatched.\n";
                    }
                    else {
                      pcVar21 = (char *)&count_1;
                      iVar9 = vm_loadNumber32(pInput,(int *)pcVar21);
                      if ((iVar9 != 0) && (-1 < count_1)) {
                        iVar9 = count_1 + -1;
                        bVar4 = 0 < count_1;
                        count_1 = iVar9;
                        if (bVar4) {
                          do {
                            pcVar21 = (char *)&pHiddenFun;
                            iVar9 = vm_loadFun(pInput,&pHiddenFun);
                            if (iVar9 == 0) {
                              pcVar20 = "Load program failed: load hidden function error.\n";
                              goto LAB_080881f2;
                            }
                            pHiddenFun->pNext = pVVar12->pHiddenFuns;
                            pVVar12->pHiddenFuns = pHiddenFun;
                            iVar9 = count_1 + -1;
                            bVar4 = 0 < count_1;
                            count_1 = iVar9;
                          } while (bVar4);
                        }
                        iVar9 = vm_loadInheritPrograms(pInput);
                        if (iVar9 == 0) {
                          pcVar20 = "Load identifiers failed: load program error.\n";
                        }
                        else {
                          pcVar21 = &DAT_082ab10b;
                          iVar9 = vm_loadComment(pInput,"DLT");
                          if (iVar9 == 0) {
                            pcVar20 = "Load identifiers failed: DLT error.\n";
                          }
                          else {
                            iVar9 = vm_loadNumber32(pInput,(int *)num);
                            i = count;
                            if (iVar9 == 0) {
                              pcVar20 = "Load identifiers failed: DLT count error.\n";
                              pcVar21 = (char *)num;
                            }
                            else {
                              while( true ) {
                                iVar9 = nameSize;
                                pcVar21 = pNames;
                                i = i + -1;
                                if (i < 0) {
                                  vm_scratchFree(pNames);
                                  return 1;
                                }
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[???]
                       Unresolved local var: Vm_Program_T * pProgram@[???]
                       Unresolved local var: Vm_Dlt_T * pDlt@[???] */
                                pVVar24 = (Vm_Binary_Buffer_T *)vm_pGlobalProgram;
                                if (vm_pCurrentProgram != (Vm_Program_T *)0x0) {
                                  pVVar24 = (Vm_Binary_Buffer_T *)vm_pCurrentProgram;
                                }
                                pcVar20 = (char *)0x14;
                                uVar16 = &dlt;
                                pVVar23 = (Vm_Binary_Buffer_T *)vm_pCurrentProgram;
                                iVar18 = vm_loadBufferAt(pInput,(void *)uVar16,0x14);
                                if (iVar18 == 0) break;
                                if ((uint)iVar9 <= dlt.name) {
                                  pcVar21 = "Load dlt entry failed: name error.\n";
                                  goto LAB_0808862e;
                                }
                                dlt.name = (uint)(pcVar21 + dlt.name);
                                if ((dlt.attrib & 0x80000) == 0) {
                                  if ((dlt.attrib & 0x40000) != 0) {
                                    if (dlt.v.value.no < 0) {
                                      sVar6 = *(Vm_Index_T *)((int)pVVar24->data + 2);
                                    }
                                    else {
                                      sVar6 = *(Vm_Index_T *)((int)pVVar24->data + 2);
                                      if (dlt.v.value.no < sVar6) goto LAB_0808850b;
                                    }
                                    pcVar20 = (char *)(int)sVar6;
                                    pcVar21 = "Bad offset (%d) of an object variable (max %d).\n";
                                    uVar16 = dlt.v.pInfo;
                                    pVVar23 = pInput;
                                    goto LAB_0808862e;
                                  }
                                  if ((dlt.attrib & 0x8000000) == 0) {
                                    pcVar21 = "Load dlt failed: Bad type of DLT entry.\n";
                                    goto LAB_0808862e;
                                  }
                                  sVar6 = (short)dlt.v._4_4_;
                                  if (sVar6 < 0) {
                                    sVar8 = *(Vm_Index_T *)((int)pVVar24->data + 4);
LAB_08088856:
                                    pcVar20 = (char *)(int)sVar8;
                                    uVar16 = SEXT24(sVar6);
                                    pcVar21 = "Bad offset (%d) of a function (max %d).\n";
                                    pVVar23 = extraout_ECX_00;
                                    goto LAB_0808862e;
                                  }
                                  sVar8 = *(Vm_Index_T *)((int)pVVar24->data + 4);
                                  if (sVar8 <= sVar6) goto LAB_08088856;
                                  iVar9 = vm_loadFun(pInput,&pFun);
                                  if (iVar9 != 0) {
                                    dlt.v.uf.pFun = pFun;
                                    goto LAB_0808850b;
                                  }
                                  goto LAB_08088636;
                                }
                                if ((dlt.v.value.no < 0) || (vm_globalVar <= dlt.v.value.no)) {
                                  pcVar21 = "Bad offset (%d) of a global variable (max %d).\n";
                                  uVar16 = dlt.v.pInfo;
                                  pcVar20 = (char *)vm_globalVar;
                                  pVVar23 = pVVar24;
                                  goto LAB_0808862e;
                                }
LAB_0808850b:
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[???] */
                                uVar2 = dlt.name;
                                pVVar12 = vm_pGlobalProgram;
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * hptr@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Ident_Hash_Elem_T * hptr2@[DW_OP_reg7(EDI)] */
                                if (vm_pGlobalProgram->ppHashTable == (Vm_Ident_Hash_Elem **)0x0) {
LAB_08088525:
                                  pVVar12 = vm_pCurrentProgram;
                                  if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
                                    pVVar12 = vm_pGlobalProgram;
                                  }
                                  pVVar13 = vm_findOrAddIdentInProgram(pVVar12,(char *)uVar2);
                                  pVVar24 = extraout_ECX_01;
                                }
                                else {
                                  uVar7 = vm_whashstr((char *)dlt.name,0x20);
                                  ppVVar3 = pVVar12->ppHashTable;
                                  pVVar13 = ppVVar3[uVar7 & 0x3ff];
                                  if (pVVar13 == (Vm_Ident_Hash_Elem *)0x0) goto LAB_08088525;
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
                                  iVar9 = strcmp(pVVar13->name,(char *)uVar2);
                                  if (iVar9 != 0) {
                                    for (pVVar19 = pVVar13->pNext; pVVar19 != pVVar13;
                                        pVVar19 = pVVar19->pNext) {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
                                      iVar9 = strcmp(pVVar19->name,(char *)uVar2);
                                      if (iVar9 == 0) {
                                        if (((int)pVVar19->token < 0) || (pVVar19->semValue != 0)) {
                                          ppVVar3[uVar7 & 0x3ff] = pVVar19;
                                          pVVar13 = pVVar19;
                                          pVVar24 = extraout_ECX_03;
                                          goto LAB_08088714;
                                        }
                                        break;
                                      }
                                    }
                                    goto LAB_08088525;
                                  }
                                  pVVar24 = extraout_ECX_02;
                                  if ((-1 < (int)pVVar13->token) && (pVVar13->semValue == 0))
                                  goto LAB_08088525;
LAB_08088714:
                                  if ((pVVar13 == (Vm_Ident_Hash_Elem *)0x0) ||
                                     (-1 < (int)pVVar13->token)) goto LAB_08088525;
                                }
                                uVar2 = pVVar13->token;
                                if ((uVar2 & 0xc0080000) != 0) {
                                  pcVar21 = "Load dlt failed: \"%s\" is already declared.\n";
                                  uVar16 = (Vm_Dlt_T *)pVVar13->name;
                                  goto LAB_0808862e;
                                }
                                if (((uVar2 & 0x8000000) != 0) && ((dlt.attrib & 0x8000000) == 0)) {
                                  pcVar21 = 
                                  "Load dlt failed: \"%s\" is already declared as function.\n";
                                  uVar16 = (Vm_Dlt_T *)pVVar13->name;
                                  goto LAB_0808862e;
                                }
                                uVar15 = 0;
                                if (pVVar13->semValue != 0) {
                    /* Unresolved local var: Vm_Ident_Info_T * pIi@[DW_OP_reg2(EDX)] */
                                  if ((int)uVar2 < 0) {
                                    pcVar21 = "The reserved word can not be redefined.";
LAB_08088689:
                                    yyerror(pcVar21);
                                  }
                                  else {
                                    if ((uVar2 & 0x40000000) != 0) {
                                      pcVar21 = "The efun can not be redefined.";
                                      goto LAB_08088689;
                                    }
                                    if (((uVar2 & 0x2ffff) == 0) ||
                                       ((pVVar13->ii).pInContext != (void *)0x0)) {
                                      pVVar24 = (Vm_Binary_Buffer_T *)0x1324;
                                      pcVar20 = "/home/cc/xxxx/gameshell/src/vm/vm_lex.c";
                                      pcVar21 = "VM";
                                      pVVar14 = pdb_allocateMemory(0x18,"VM",
                                                                                                                                      
                                                  "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x1324);
                                      if (pVVar14 != (Vm_Ident_Info *)0x0) {
                                        (pVVar14->v).pInfo = (pVVar13->ii).v.pInfo;
                                        *(undefined4 *)((int)&pVVar14->v + 4) =
                                             *(undefined4 *)((int)&(pVVar13->ii).v + 4);
                                        pVVar14->pInContext = (pVVar13->ii).pInContext;
                                        pVVar14->oldToken = (pVVar13->ii).oldToken;
                                        uVar7 = (pVVar13->ii).unused;
                                        pVVar14->refered = (pVVar13->ii).refered;
                                        pVVar14->unused = uVar7;
                                        pVVar14->pNext = (pVVar13->ii).pNext;
                                        pVVar14->oldToken = pVVar13->token;
                                        (pVVar13->ii).pNext = pVVar14;
                                        uVar15 = pVVar13->semValue;
                                        goto LAB_080885e4;
                                      }
                                      str_00 = "No enough memort to backup information structure.\n"
                                      ;
                                    }
                                    else {
                                      pcVar21 = pVVar13->name;
                                      str_00 = "The name \"%s\" in line %d has been defined.\n";
                                      pcVar20 = (char *)currentLine;
                                    }
                                    yyerrors(str_00,pcVar21,pcVar20,pVVar24);
                                    pVVar23 = pVVar24;
                                  }
                                  pcVar21 = "Load dlt failed: Failed to backup II for \"%s\".\n";
                                  uVar16 = (Vm_Dlt_T *)pVVar13->name;
                                  goto LAB_0808862e;
                                }
LAB_080885e4:
                                pVVar13->semValue = uVar15 + 1;
                                (pVVar13->ii).pInContext = (void *)0x0;
                                *(undefined4 *)((int)&(pVVar13->ii).v + 4) = dlt.v._4_4_;
                                (pVVar13->ii).v.pInfo = dlt.v.pInfo;
                                psVar1 = &(pVVar13->dn).globalNum;
                                *psVar1 = *psVar1 + 1;
                                (pVVar13->ii).oldToken = dlt.attrib | 0x20000000;
                                pVVar13->token = dlt.attrib | 0x20000000;
                              }
                              pcVar21 = "Load dlt entry failed: read DLT entry error.\n";
LAB_0808862e:
                              vm_errprintf(pcVar21,uVar16,pcVar20,pVVar23);
LAB_08088636:
                              if (0xff < (uint)nameSize) {
                                nameSize = 0xff;
                              }
                              memcpy(str,pNames,nameSize);
                              pcVar20 = "Load identifier failed: %s.\n";
                              str[nameSize] = '\0';
                              pcVar21 = str;
                            }
                          }
                        }
                        goto LAB_08087fe1;
                      }
                      pcVar20 = "Load program failed: read count of hidden functions error.\n";
                    }
                  }
                }
              }
            }
LAB_080881f2:
            vm_errprintf(pcVar20);
            pcVar20 = "Load identifiers failed: load functions error.\n";
            goto LAB_08087fe1;
          }
          pcVar20 = "Load variables failed: comment \"PROG_VARS\" mismatched.\n";
        }
      }
LAB_080880b2:
      vm_errprintf(pcVar20);
LAB_080880b7:
      pcVar20 = "Load identifiers failed: load variables error.\n";
      goto LAB_08087fe1;
    }
  }
  pcVar20 = "Load identifiers failed: names error.\n";
LAB_08087fe1:
  vm_errprintf(pcVar20,pcVar21);
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x8087fb8: PUSH EBP
0x8087fb9: MOV EBP, ESP
0x8087fbb: PUSH EDI
0x8087fbc: PUSH ESI
0x8087fbd: PUSH EBX
0x8087fbe: SUB ESP, 0x174
0x8087fc4: PUSH 0x82ab0cf
0x8087fc9: MOV EBX, dword ptr [EBP + 0x8]
0x8087fcc: PUSH EBX
0x8087fcd: CALL 0x080b8800
0x8087fd2: ADD ESP, 0x10
0x8087fd5: TEST EAX, EAX
0x8087fd7: JNZ 0x08087ff0
0x8087fd9: SUB ESP, 0xc
0x8087fdc: PUSH 0x82aba74
0x8087fe1: CALL 0x08097acc
0x8087fe6: XOR EAX, EAX
0x8087fe8: LEA ESP, [EBP + -0xc]
0x8087feb: POP EBX
0x8087fec: POP ESI
0x8087fed: POP EDI
0x8087fee: LEAVE
0x8087fef: RET
0x8087ff0: SUB ESP, 0x8
0x8087ff3: PUSH 0x82ab0db
0x8087ff8: PUSH EBX
0x8087ff9: CALL 0x080b8800
0x8087ffe: ADD ESP, 0x10
0x8088001: TEST EAX, EAX
0x8088003: JNZ 0x08088010
0x8088005: SUB ESP, 0xc
0x8088008: PUSH 0x82abab0
0x808800d: JMP 0x08087fe1
0x8088010: SUB ESP, 0x8
0x8088013: LEA ESI, [EBP + 0xfffffec4]
0x8088019: PUSH ESI
0x808801a: PUSH EBX
0x808801b: CALL 0x080b8bf4
0x8088020: ADD ESP, 0x10
0x8088023: TEST EAX, EAX
0x8088025: JZ 0x08088078
0x8088027: MOV EAX, dword ptr [EBP + 0xfffffec4]
0x808802d: TEST EAX, EAX
0x808802f: JS 0x08088078
0x8088031: SUB ESP, 0x8
0x8088034: LEA EAX, [EBP + 0xfffffec0]
0x808803a: PUSH EAX
0x808803b: PUSH EBX
0x808803c: CALL 0x080b8bf4
0x8088041: ADD ESP, 0x10
0x8088044: TEST EAX, EAX
0x8088046: JZ 0x08088068
0x8088048: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x808804e: TEST EAX, EAX
0x8088050: JS 0x08088068
0x8088052: PUSH EDI
0x8088053: PUSH EAX
0x8088054: LEA EAX, [EBP + 0xfffffebc]
0x808805a: PUSH EAX
0x808805b: PUSH EBX
0x808805c: CALL 0x080b899c
0x8088061: ADD ESP, 0x10
0x8088064: TEST EAX, EAX
0x8088066: JNZ 0x08088085
0x8088068: SUB ESP, 0xc
0x808806b: PUSH 0x82abae8
0x8088070: JMP 0x08087fe1
0x8088078: SUB ESP, 0xc
0x808807b: PUSH 0x82abb10
0x8088080: JMP 0x08087fe1
0x8088085: MOV EAX, [0x0847aac0]
0x808808a: TEST EAX, EAX
0x808808c: MOV EDI, dword ptr [0x0847aad0]
0x8088092: CMOVNZ EDI, EAX
0x8088095: SUB ESP, 0x8
0x8088098: PUSH 0x82ab0e1
0x808809d: PUSH EBX
0x808809e: CALL 0x080b8800
0x80880a3: ADD ESP, 0x10
0x80880a6: TEST EAX, EAX
0x80880a8: JNZ 0x080880c3
0x80880aa: SUB ESP, 0xc
0x80880ad: PUSH 0x82abb38
0x80880b2: CALL 0x08097acc
0x80880b7: MOV dword ptr [ESP], 0x82abb70
0x80880be: JMP 0x08087fe1
0x80880c3: SUB ESP, 0x8
0x80880c6: LEA EAX, [EBP + 0xfffffeba]
0x80880cc: PUSH EAX
0x80880cd: PUSH EBX
0x80880ce: CALL 0x080b8b44
0x80880d3: ADD ESP, 0x10
0x80880d6: TEST EAX, EAX
0x80880d8: JZ 0x08088203
0x80880de: CMP word ptr [EBP + 0xfffffeba], 0x0
0x80880e6: JS 0x08088203
0x80880ec: SUB ESP, 0x8
0x80880ef: LEA EAX, [EBP + 0xfffffeb8]
0x80880f5: PUSH EAX
0x80880f6: PUSH EBX
0x80880f7: CALL 0x080b8b44
0x80880fc: ADD ESP, 0x10
0x80880ff: TEST EAX, EAX
0x8088101: JZ 0x08088203
0x8088107: CMP word ptr [EBP + 0xfffffeb8], 0x0
0x808810f: JS 0x08088203
0x8088115: CMP EDI, dword ptr [0x0847aad0]
0x808811b: JZ 0x08088210
0x8088121: SUB ESP, 0x8
0x8088124: MOV AX, word ptr [EBP + 0xfffffeba]
0x808812b: MOV word ptr [EDI + 0xe], AX
0x808812f: PUSH 0x82ab0eb
0x8088134: PUSH EBX
0x8088135: CALL 0x080b8800
0x808813a: ADD ESP, 0x10
0x808813d: TEST EAX, EAX
0x808813f: JZ 0x08088257
0x8088145: LEA EAX, [EDI + 0x28]
0x8088148: JMP 0x080881a6
0x808814a: PUSH ECX
0x808814b: PUSH 0x10
0x808814d: LEA EDX, [EBP + 0xfffffeb4]
0x8088153: PUSH EDX
0x8088154: PUSH EBX
0x8088155: CALL 0x080b8a70
0x808815a: ADD ESP, 0x10
0x808815d: TEST EAX, EAX
0x808815f: JZ 0x080882a0
0x8088165: SUB ESP, 0x8
0x8088168: PUSH dword ptr [EBP + 0xfffffeb4]
0x808816e: PUSH EBX
0x808816f: CALL 0x080b8dc8
0x8088174: ADD ESP, 0x10
0x8088177: TEST EAX, EAX
0x8088179: JZ 0x08088271
0x808817f: MOV EDX, dword ptr [EBP + 0xfffffeb4]
0x8088185: MOV EAX, dword ptr [EDI + 0x4]
0x8088188: MOV ECX, dword ptr [EBP + 0xfffffe9c]
0x808818e: MOV EAX, dword ptr [EAX + 0xc]
0x8088191: MOV dword ptr [EDX + 0xc], 0x0
0x8088198: MOV dword ptr [ECX], EDX
0x808819a: MOV dword ptr [EDX + 0x4], EAX
0x808819d: MOV EAX, dword ptr [EBP + 0xfffffeb4]
0x80881a3: ADD EAX, 0xc
0x80881a6: MOV EDX, dword ptr [EBP + 0xfffffeb8]
0x80881ac: MOV dword ptr [EBP + 0xfffffe9c], EAX
0x80881b2: TEST DX, DX
0x80881b5: LEA EAX, [EDX + -0x1]
0x80881b8: MOV word ptr [EBP + 0xfffffeb8], AX
0x80881bf: JG 0x0808814a
0x80881c1: MOV EAX, [0x0847aac0]
0x80881c6: TEST EAX, EAX
0x80881c8: CMOVZ EAX, dword ptr [0x0847aad0]
0x80881cf: SUB ESP, 0x8
0x80881d2: PUSH 0x82ab0f5
0x80881d7: PUSH EBX
0x80881d8: MOV dword ptr [EBP + 0xfffffe98], EAX
0x80881de: CALL 0x080b8800
0x80881e3: ADD ESP, 0x10
0x80881e6: TEST EAX, EAX
0x80881e8: JNZ 0x08088226
0x80881ea: SUB ESP, 0xc
0x80881ed: PUSH 0x82abba0
0x80881f2: CALL 0x08097acc
0x80881f7: MOV dword ptr [ESP], 0x82abbd8
0x80881fe: JMP 0x08087fe1
0x8088203: SUB ESP, 0xc
0x8088206: PUSH 0x82abc08
0x808820b: JMP 0x080880b2
0x8088210: MOVSX EAX, word ptr [EBP + 0xfffffeba]
0x8088217: CMP EAX, dword ptr [0x0847aac4]
0x808821d: JL 0x08088264
0x808821f: MOV [0x0847aac4], EAX
0x8088224: JMP 0x080881c1
0x8088226: SUB ESP, 0x8
0x8088229: LEA EAX, [EBP + 0xfffffeb2]
0x808822f: PUSH EAX
0x8088230: PUSH EBX
0x8088231: CALL 0x080b8b44
0x8088236: ADD ESP, 0x10
0x8088239: TEST EAX, EAX
0x808823b: JZ 0x0808824d
0x808823d: MOV AX, word ptr [EBP + 0xfffffeb2]
0x8088244: CMP word ptr [0x0847ace0], AX
0x808824b: JZ 0x080882ad
0x808824d: SUB ESP, 0xc
0x8088250: PUSH 0x82abc38
0x8088255: JMP 0x080881f2
0x8088257: SUB ESP, 0xc
0x808825a: PUSH 0x82abc64
0x808825f: JMP 0x080880b2
0x8088264: SUB ESP, 0xc
0x8088267: PUSH 0x82abc9c
0x808826c: JMP 0x080880b2
0x8088271: SUB ESP, 0xc
0x8088274: PUSH 0x82abcc8
0x8088279: CALL 0x08097acc
0x808827e: ADD ESP, 0x10
0x8088281: PUSH 0x1bb9
0x8088286: PUSH 0x82ab14c
0x808828b: PUSH 0x82a376b
0x8088290: PUSH dword ptr [EBP + 0xfffffeb4]
0x8088296: CALL 0x080ceb94
0x808829b: JMP 0x080880b7
0x80882a0: SUB ESP, 0xc
0x80882a3: PUSH 0x82abd04
0x80882a8: JMP 0x080880b2
0x80882ad: SUB ESP, 0x8
0x80882b0: LEA EAX, [EBP + 0xfffffeb0]
0x80882b6: PUSH EAX
0x80882b7: PUSH EBX
0x80882b8: CALL 0x080b8b44
0x80882bd: ADD ESP, 0x10
0x80882c0: TEST EAX, EAX
0x80882c2: JZ 0x08088352
0x80882c8: MOV EAX, dword ptr [EBP + 0xfffffeb0]
0x80882ce: TEST AX, AX
0x80882d1: JS 0x08088352
0x80882d3: MOV EDX, dword ptr [EBP + 0xfffffe98]
0x80882d9: MOV word ptr [EDX + 0x10], AX
0x80882dd: MOV EDX, dword ptr [EDX + 0x2c]
0x80882e0: TEST EDX, EDX
0x80882e2: JNZ 0x080883f7
0x80882e8: PUSH 0x19f3
0x80882ed: PUSH 0x82ab14c
0x80882f2: CWDE
0x80882f3: PUSH 0x82a376b
0x80882f8: SHL EAX, 0x3
0x80882fb: PUSH EAX
0x80882fc: CALL 0x080ce784
0x8088301: ADD ESP, 0x10
0x8088304: TEST EAX, EAX
0x8088306: JZ 0x080883ea
0x808830c: MOV EDX, dword ptr [EBP + 0xfffffe98]
0x8088312: MOVSX ECX, word ptr [EDX + 0x10]
0x8088316: MOVSX EDI, word ptr [0x08660900]
0x808831d: SUB ECX, EDI
0x808831f: SHL ECX, 0x3
0x8088322: MOV dword ptr [EDX + 0x2c], EAX
0x8088325: SUB ESP, 0x8
0x8088328: LEA EDI, [EAX + EDI*0x8]
0x808832b: CLD
0x808832c: XOR EAX, EAX
0x808832e: SHR ECX, 0x2
0x8088331: STOSD.REP ES:EDI
0x8088333: PUSH 0x82ab0ff
0x8088338: PUSH EBX
0x8088339: CALL 0x080b8800
0x808833e: ADD ESP, 0x10
0x8088341: TEST EAX, EAX
0x8088343: JNZ 0x0808835f
0x8088345: SUB ESP, 0xc
0x8088348: PUSH 0x82abd3c
0x808834d: JMP 0x080881f2
0x8088352: SUB ESP, 0xc
0x8088355: PUSH 0x82abd74
0x808835a: JMP 0x080881f2
0x808835f: SUB ESP, 0x8
0x8088362: LEA EAX, [EBP + 0xfffffeac]
0x8088368: PUSH EAX
0x8088369: PUSH EBX
0x808836a: CALL 0x080b8bf4
0x808836f: ADD ESP, 0x10
0x8088372: TEST EAX, EAX
0x8088374: JZ 0x08088441
0x808837a: MOV EDX, dword ptr [EBP + 0xfffffeac]
0x8088380: TEST EDX, EDX
0x8088382: JS 0x08088441
0x8088388: LEA EAX, [EDX + -0x1]
0x808838b: TEST EDX, EDX
0x808838d: MOV dword ptr [EBP + 0xfffffeac], EAX
0x8088393: JLE 0x08088424
0x8088399: LEA EDI, [EBP + 0xfffffea8]
0x808839f: JMP 0x080883cc
0x80883a4: MOV ECX, dword ptr [EBP + 0xfffffe98]
0x80883aa: MOV EDX, dword ptr [EBP + 0xfffffea8]
0x80883b0: MOV EAX, dword ptr [ECX + 0x30]
0x80883b3: MOV dword ptr [EDX + 0x3c], EAX
0x80883b6: MOV dword ptr [ECX + 0x30], EDX
0x80883b9: MOV EDX, dword ptr [EBP + 0xfffffeac]
0x80883bf: LEA EAX, [EDX + -0x1]
0x80883c2: TEST EDX, EDX
0x80883c4: MOV dword ptr [EBP + 0xfffffeac], EAX
0x80883ca: JLE 0x08088424
0x80883cc: SUB ESP, 0x8
0x80883cf: PUSH EDI
0x80883d0: PUSH EBX
0x80883d1: CALL 0x080959b8
0x80883d6: ADD ESP, 0x10
0x80883d9: TEST EAX, EAX
0x80883db: JNZ 0x080883a4
0x80883dd: SUB ESP, 0xc
0x80883e0: PUSH 0x82abda4
0x80883e5: JMP 0x080881f2
0x80883ea: SUB ESP, 0xc
0x80883ed: PUSH 0x82abdd8
0x80883f2: JMP 0x080881f2
0x80883f7: SUB ESP, 0xc
0x80883fa: PUSH 0x19f7
0x80883ff: PUSH 0x82ab14c
0x8088404: CWDE
0x8088405: PUSH 0x82a376b
0x808840a: SHL EAX, 0x3
0x808840d: PUSH EAX
0x808840e: MOV ECX, dword ptr [EBP + 0xfffffe98]
0x8088414: PUSH dword ptr [ECX + 0x2c]
0x8088417: CALL 0x080cf588
0x808841c: ADD ESP, 0x20
0x808841f: JMP 0x08088304
0x8088424: SUB ESP, 0xc
0x8088427: PUSH EBX
0x8088428: CALL 0x0808729c
0x808842d: ADD ESP, 0x10
0x8088430: TEST EAX, EAX
0x8088432: JNZ 0x0808844e
0x8088434: SUB ESP, 0xc
0x8088437: PUSH 0x82abe10
0x808843c: JMP 0x08087fe1
0x8088441: SUB ESP, 0xc
0x8088444: PUSH 0x82abe40
0x8088449: JMP 0x080881f2
0x808844e: SUB ESP, 0x8
0x8088451: PUSH 0x82ab10b
0x8088456: PUSH EBX
0x8088457: CALL 0x080b8800
0x808845c: ADD ESP, 0x10
0x808845f: TEST EAX, EAX
0x8088461: JNZ 0x08088470
0x8088463: SUB ESP, 0xc
0x8088466: PUSH 0x82abe7c
0x808846b: JMP 0x08087fe1
0x8088470: SUB ESP, 0x8
0x8088473: PUSH ESI
0x8088474: PUSH EBX
0x8088475: CALL 0x080b8bf4
0x808847a: ADD ESP, 0x10
0x808847d: TEST EAX, EAX
0x808847f: JZ 0x0808881e
0x8088485: MOV EAX, dword ptr [EBP + 0xfffffec4]
0x808848b: DEC EAX
0x808848c: MOV dword ptr [EBP + 0xfffffea0], EAX
0x8088492: JS 0x0808883a
0x8088498: MOV EAX, [0x0847aac0]
0x808849d: TEST EAX, EAX
0x808849f: MOV ESI, dword ptr [0x0847aad0]
0x80884a5: CMOVNZ ESI, EAX
0x80884a8: PUSH EAX
0x80884a9: PUSH 0x14
0x80884ab: LEA ECX, [EBP + -0x38]
0x80884ae: PUSH ECX
0x80884af: MOV EDX, dword ptr [EBP + 0xfffffec0]
0x80884b5: PUSH EBX
0x80884b6: MOV EDI, dword ptr [EBP + 0xfffffebc]
0x80884bc: MOV dword ptr [EBP + 0xfffffe94], EDX
0x80884c2: CALL 0x080b8904
0x80884c7: ADD ESP, 0x10
0x80884ca: TEST EAX, EAX
0x80884cc: JZ 0x08088811
0x80884d2: MOV EAX, dword ptr [EBP + -0x38]
0x80884d5: CMP EAX, dword ptr [EBP + 0xfffffe94]
0x80884db: JNC 0x080887b9
0x80884e1: ADD EAX, EDI
0x80884e3: MOV dword ptr [EBP + -0x38], EAX
0x80884e6: MOV EAX, dword ptr [EBP + -0x34]
0x80884e9: TEST EAX, 0x80000
0x80884ee: JZ 0x0808878c
0x80884f4: MOV EAX, dword ptr [EBP + -0x2c]
0x80884f7: TEST EAX, EAX
0x80884f9: JS 0x08088755
0x80884ff: CMP EAX, dword ptr [0x0847aac4]
0x8088505: JGE 0x08088755
0x808850b: MOV ESI, dword ptr [0x0847aad0]
0x8088511: MOV EDX, dword ptr [ESI + 0x54]
0x8088514: MOV EAX, dword ptr [EBP + -0x38]
0x8088517: TEST EDX, EDX
0x8088519: MOV dword ptr [EBP + 0xfffffe90], EAX
0x808851f: JNZ 0x080886b9
0x8088525: MOV EAX, [0x0847aac0]
0x808852a: TEST EAX, EAX
0x808852c: CMOVZ EAX, dword ptr [0x0847aad0]
0x8088533: SUB ESP, 0x8
0x8088536: PUSH dword ptr [EBP + 0xfffffe90]
0x808853c: PUSH EAX
0x808853d: CALL 0x08085a94
0x8088542: MOV EDI, EAX
0x8088544: ADD ESP, 0x10
0x8088547: MOV EDX, dword ptr [EDI + 0x4]
0x808854a: TEST EDX, 0xc0080000
0x8088550: JNZ 0x0808869a
0x8088556: TEST EDX, 0x8000000
0x808855c: JZ 0x08088568
0x808855e: TEST byte ptr [EBP + -0x31], 0x8
0x8088562: JZ 0x0808882b
0x8088568: MOV EAX, dword ptr [EDI + 0x8]
0x808856b: TEST EAX, EAX
0x808856d: JZ 0x080885e4
0x808856f: TEST EDX, EDX
0x8088571: JS 0x08088690
0x8088577: TEST EDX, 0x40000000
0x808857d: JNZ 0x08088681
0x8088583: AND EDX, 0x2ffff
0x8088589: JZ 0x08088596
0x808858b: MOV ESI, dword ptr [EDI + 0x1c]
0x808858e: TEST ESI, ESI
0x8088590: JZ 0x080886a6
0x8088596: PUSH 0x1324
0x808859b: PUSH 0x82ab14c
0x80885a0: PUSH 0x82a376b
0x80885a5: PUSH 0x18
0x80885a7: CALL 0x080ce784
0x80885ac: ADD ESP, 0x10
0x80885af: TEST EAX, EAX
0x80885b1: MOV EDX, EAX
0x80885b3: JZ 0x08088618
0x80885b5: MOV EAX, dword ptr [EDI + 0x14]
0x80885b8: MOV dword ptr [EDX], EAX
0x80885ba: MOV EAX, dword ptr [EDI + 0x18]
0x80885bd: MOV dword ptr [EDX + 0x4], EAX
0x80885c0: MOV EAX, dword ptr [EDI + 0x1c]
0x80885c3: MOV dword ptr [EDX + 0x8], EAX
0x80885c6: MOV EAX, dword ptr [EDI + 0x20]
0x80885c9: MOV dword ptr [EDX + 0xc], EAX
0x80885cc: MOV EAX, dword ptr [EDI + 0x24]
0x80885cf: MOV dword ptr [EDX + 0x10], EAX
0x80885d2: MOV EAX, dword ptr [EDI + 0x28]
0x80885d5: MOV dword ptr [EDX + 0x14], EAX
0x80885d8: MOV EAX, dword ptr [EDI + 0x4]
0x80885db: MOV dword ptr [EDX + 0xc], EAX
0x80885de: MOV dword ptr [EDI + 0x28], EDX
0x80885e1: MOV EAX, dword ptr [EDI + 0x8]
0x80885e4: INC EAX
0x80885e5: MOV dword ptr [EDI + 0x8], EAX
0x80885e8: MOV dword ptr [EDI + 0x1c], 0x0
0x80885ef: MOV EAX, dword ptr [EBP + -0x2c]
0x80885f2: MOV EDX, dword ptr [EBP + -0x28]
0x80885f5: MOV dword ptr [EDI + 0x18], EDX
0x80885f8: MOV dword ptr [EDI + 0x14], EAX
0x80885fb: MOV EAX, dword ptr [EBP + -0x34]
0x80885fe: OR EAX, 0x20000000
0x8088603: INC word ptr [EDI + 0x2e]
0x8088607: DEC dword ptr [EBP + 0xfffffea0]
0x808860d: MOV dword ptr [EDI + 0x20], EAX
0x8088610: MOV dword ptr [EDI + 0x4], EAX
0x8088613: JMP 0x08088492
0x8088618: SUB ESP, 0xc
0x808861b: PUSH 0x82ab6fc
0x8088620: CALL 0x080ff97c
0x8088625: POP EAX
0x8088626: POP EDX
0x8088627: PUSH dword ptr [EDI]
0x8088629: PUSH 0x82abea4
0x808862e: CALL 0x08097acc
0x8088633: ADD ESP, 0x10
0x8088636: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x808863c: CMP EAX, 0xff
0x8088641: JBE 0x08088652
0x8088643: MOV dword ptr [EBP + 0xfffffec0], 0xff
0x808864d: MOV EAX, 0xff
0x8088652: PUSH EDI
0x8088653: PUSH EAX
0x8088654: PUSH dword ptr [EBP + 0xfffffebc]
0x808865a: LEA EBX, [EBP + 0xfffffec8]
0x8088660: PUSH EBX
0x8088661: CALL 0x0804b7e4
0x8088666: POP ECX
0x8088667: POP ESI
0x8088668: PUSH EBX
0x8088669: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x808866f: PUSH 0x82ab129
0x8088674: MOV byte ptr [EBP + EAX*0x1 + 0xfffffec8], 0x0
0x808867c: JMP 0x08087fe1
0x8088681: SUB ESP, 0xc
0x8088684: PUSH 0x82ab6b4
0x8088689: CALL 0x080ff80c
0x808868e: JMP 0x08088625
0x8088690: SUB ESP, 0xc
0x8088693: PUSH 0x82ab6d4
0x8088698: JMP 0x08088689
0x808869a: SUB ESP, 0x8
0x808869d: PUSH dword ptr [EDI]
0x808869f: PUSH 0x82abed4
0x80886a4: JMP 0x0808862e
0x80886a6: PUSH ECX
0x80886a7: PUSH dword ptr [0x08661924]
0x80886ad: PUSH dword ptr [EDI]
0x80886af: PUSH 0x82ab730
0x80886b4: JMP 0x08088620
0x80886b9: SUB ESP, 0x8
0x80886bc: PUSH 0x20
0x80886be: PUSH EAX
0x80886bf: CALL 0x08089dc8
0x80886c4: MOV EDX, EAX
0x80886c6: AND EDX, 0x3ff
0x80886cc: MOV ESI, dword ptr [ESI + 0x54]
0x80886cf: MOV dword ptr [EBP + 0xfffffe88], ESI
0x80886d5: MOV ESI, dword ptr [ESI + EDX*0x4]
0x80886d8: ADD ESP, 0x10
0x80886db: TEST ESI, ESI
0x80886dd: MOV dword ptr [EBP + 0xfffffe8c], EDX
0x80886e3: JZ 0x08088525
0x80886e9: SUB ESP, 0x8
0x80886ec: PUSH dword ptr [EBP + 0xfffffe90]
0x80886f2: PUSH dword ptr [ESI]
0x80886f4: CALL 0x0804b064
0x80886f9: ADD ESP, 0x10
0x80886fc: TEST EAX, EAX
0x80886fe: JNZ 0x0808872e
0x8088700: MOV EAX, dword ptr [ESI + 0x4]
0x8088703: TEST EAX, EAX
0x8088705: JS 0x08088712
0x8088707: MOV EAX, dword ptr [ESI + 0x8]
0x808870a: TEST EAX, EAX
0x808870c: JZ 0x08088525
0x8088712: MOV EAX, ESI
0x8088714: TEST EAX, EAX
0x8088716: JZ 0x08088525
0x808871c: MOV EDI, EAX
0x808871e: MOV EAX, dword ptr [EAX + 0x4]
0x8088721: TEST EAX, EAX
0x8088723: JNS 0x08088525
0x8088729: JMP 0x08088547
0x808872e: MOV EDI, dword ptr [ESI + 0xc]
0x8088731: CMP EDI, ESI
0x8088733: JZ 0x08088525
0x8088739: SUB ESP, 0x8
0x808873c: PUSH dword ptr [EBP + 0xfffffe90]
0x8088742: PUSH dword ptr [EDI]
0x8088744: CALL 0x0804b064
0x8088749: ADD ESP, 0x10
0x808874c: TEST EAX, EAX
0x808874e: JZ 0x08088767
0x8088750: MOV EDI, dword ptr [EDI + 0xc]
0x8088753: JMP 0x08088731
0x8088755: PUSH ESI
0x8088756: PUSH dword ptr [0x0847aac4]
0x808875c: PUSH EAX
0x808875d: PUSH 0x82abf00
0x8088762: JMP 0x0808862e
0x8088767: MOV EAX, dword ptr [EDI + 0x4]
0x808876a: TEST EAX, EAX
0x808876c: JS 0x08088779
0x808876e: MOV EAX, dword ptr [EDI + 0x8]
0x8088771: TEST EAX, EAX
0x8088773: JZ 0x08088525
0x8088779: MOV EAX, dword ptr [EBP + 0xfffffe8c]
0x808877f: MOV EDX, dword ptr [EBP + 0xfffffe88]
0x8088785: MOV dword ptr [EDX + EAX*0x4], EDI
0x8088788: MOV EAX, EDI
0x808878a: JMP 0x08088714
0x808878c: TEST EAX, 0x40000
0x8088791: JZ 0x080887cc
0x8088793: MOV ECX, dword ptr [EBP + -0x2c]
0x8088796: TEST ECX, ECX
0x8088798: JS 0x080887c6
0x808879a: MOV DX, word ptr [ESI + 0xe]
0x808879e: MOVSX EAX, DX
0x80887a1: CMP ECX, EAX
0x80887a3: JL 0x0808850b
0x80887a9: MOVSX EAX, DX
0x80887ac: PUSH EBX
0x80887ad: PUSH EAX
0x80887ae: PUSH ECX
0x80887af: PUSH 0x82abf30
0x80887b4: JMP 0x0808862e
0x80887b9: SUB ESP, 0xc
0x80887bc: PUSH 0x82abf64
0x80887c1: JMP 0x0808862e
0x80887c6: MOV DX, word ptr [ESI + 0xe]
0x80887ca: JMP 0x080887a9
0x80887cc: TEST EAX, 0x8000000
0x80887d1: JZ 0x08088867
0x80887d7: MOV EDX, dword ptr [EBP + -0x28]
0x80887da: TEST DX, DX
0x80887dd: JS 0x08088852
0x80887df: MOV AX, word ptr [ESI + 0x10]
0x80887e3: CMP DX, AX
0x80887e6: JGE 0x08088856
0x80887e8: SUB ESP, 0x8
0x80887eb: LEA EAX, [EBP + 0xfffffea4]
0x80887f1: PUSH EAX
0x80887f2: PUSH EBX
0x80887f3: CALL 0x080959b8
0x80887f8: ADD ESP, 0x10
0x80887fb: TEST EAX, EAX
0x80887fd: JZ 0x08088636
0x8088803: MOV EAX, dword ptr [EBP + 0xfffffea4]
0x8088809: MOV dword ptr [EBP + -0x2c], EAX
0x808880c: JMP 0x0808850b
0x8088811: SUB ESP, 0xc
0x8088814: PUSH 0x82abf88
0x8088819: JMP 0x0808862e
0x808881e: SUB ESP, 0xc
0x8088821: PUSH 0x82abfb8
0x8088826: JMP 0x08087fe1
0x808882b: SUB ESP, 0x8
0x808882e: PUSH dword ptr [EDI]
0x8088830: PUSH 0x82abfe4
0x8088835: JMP 0x0808862e
0x808883a: SUB ESP, 0xc
0x808883d: PUSH dword ptr [EBP + 0xfffffebc]
0x8088843: CALL 0x080891d4
0x8088848: MOV EAX, 0x1
0x808884d: JMP 0x08087fe8
0x8088852: MOV AX, word ptr [ESI + 0x10]
0x8088856: CWDE
0x8088857: PUSH ECX
0x8088858: PUSH EAX
0x8088859: MOVSX EAX, DX
0x808885c: PUSH EAX
0x808885d: PUSH 0x82ac01c
0x8088862: JMP 0x0808862e
0x8088867: SUB ESP, 0xc
0x808886a: PUSH 0x82ac048
0x808886f: JMP 0x0808862e
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_registerMysql"></a>

## vm_registerMysql @ 0x816f900

Evidence: `ev_b90c279bf69f0154893812efeaa2a21b5093fb737f08632894669ea47534c9b4`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_registerMysql(void)

{
  int iVar1;
  
  iVar1 = vm_registerEfunModule(initMysqlEfun,shutdownMysqlEfun);
  if (iVar1 != 0) {
    vm_registerSubModule("DB MYSQL",vm_initDbMysql,vm_shutdownDbMysql);
    my_init();
    vm_registerServiceThreadInit
              (initDbMySqlForServiceThread,shutdownDbMysqlForServiceThread,(void *)0x0);
  }
  return (uint)(iVar1 != 0);
}


```

<details><summary>汇编</summary>

```asm
0x816f900: PUSH EBP
0x816f901: MOV EBP, ESP
0x816f903: SUB ESP, 0x18
0x816f906: MOV dword ptr [ESP + 0x4], 0x816f9a0
0x816f90e: MOV dword ptr [ESP], 0x816f9b0
0x816f915: CALL 0x080874d4
0x816f91a: XOR EDX, EDX
0x816f91c: TEST EAX, EAX
0x816f91e: JZ 0x0816f962
0x816f920: MOV dword ptr [ESP + 0x8], 0x816f600
0x816f928: MOV dword ptr [ESP + 0x4], 0x816f750
0x816f930: MOV dword ptr [ESP], 0x82cfd77
0x816f937: CALL 0x080b5a90
0x816f93c: CALL 0x08233290
0x816f941: MOV dword ptr [ESP + 0x8], 0x0
0x816f949: MOV dword ptr [ESP + 0x4], 0x816f970
0x816f951: MOV dword ptr [ESP], 0x816f980
0x816f958: CALL 0x080be280
0x816f95d: MOV EDX, 0x1
0x816f962: LEAVE
0x816f963: MOV EAX, EDX
0x816f965: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadDefines"></a>

## vm_loadDefines @ 0x8087080

Evidence: `ev_b99359e12a3a7ffb2aa95244d4382f00482591b3035c1ec7e358124c5f56e024`

```c

int vm_loadDefines(Vm_Binary_Buffer_T *pInput)

{
  int iVar1;
  char *str;
  char *exps;
  char *name;
  int nargs;
  int count;
  
  iVar1 = vm_loadNumber32(pInput,&count);
  if ((iVar1 == 0) || (count < 0)) {
    str = "Load defines failed: read count error.\n";
LAB_08087138:
    vm_errprintf(str);
    return 0;
  }
  if (0 < count) {
    do {
      count = count + -1;
      iVar1 = vm_loadNumber32(pInput,&nargs);
      if (iVar1 == 0) {
        str = "Load defines failed: read nargs error.\n";
        goto LAB_08087138;
      }
      iVar1 = vm_loadTempString(pInput,&name);
      if (iVar1 == 0) {
        str = "Load defines failed: read name error.\n";
        goto LAB_08087138;
      }
      iVar1 = vm_loadTempString(pInput,&exps);
      if (iVar1 == 0) {
        str = "Load defines failed: read exps error.\n";
        goto LAB_08087138;
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
0x8087080: PUSH EBP
0x8087081: MOV EBP, ESP
0x8087083: PUSH ESI
0x8087084: PUSH EBX
0x8087085: SUB ESP, 0x18
0x8087088: LEA EAX, [EBP + -0xc]
0x808708b: MOV EBX, dword ptr [EBP + 0x8]
0x808708e: PUSH EAX
0x808708f: PUSH EBX
0x8087090: CALL 0x080b8bf4
0x8087095: ADD ESP, 0x10
0x8087098: TEST EAX, EAX
0x808709a: JZ 0x08087130
0x80870a0: MOV EDX, dword ptr [EBP + -0xc]
0x80870a3: TEST EDX, EDX
0x80870a5: JS 0x08087130
0x80870ab: LEA EAX, [EDX + -0x1]
0x80870ae: TEST EDX, EDX
0x80870b0: MOV dword ptr [EBP + -0xc], EAX
0x80870b3: JLE 0x08087122
0x80870b5: LEA ESI, [EBP + -0x10]
0x80870b8: SUB ESP, 0x8
0x80870bb: PUSH ESI
0x80870bc: PUSH EBX
0x80870bd: CALL 0x080b8bf4
0x80870c2: ADD ESP, 0x10
0x80870c5: TEST EAX, EAX
0x80870c7: JZ 0x08087146
0x80870c9: SUB ESP, 0x8
0x80870cc: LEA EAX, [EBP + -0x14]
0x80870cf: PUSH EAX
0x80870d0: PUSH EBX
0x80870d1: CALL 0x080b8c4c
0x80870d6: ADD ESP, 0x10
0x80870d9: TEST EAX, EAX
0x80870db: JZ 0x08087150
0x80870dd: SUB ESP, 0x8
0x80870e0: LEA EAX, [EBP + -0x18]
0x80870e3: PUSH EAX
0x80870e4: PUSH EBX
0x80870e5: CALL 0x080b8c4c
0x80870ea: ADD ESP, 0x10
0x80870ed: TEST EAX, EAX
0x80870ef: JZ 0x0808715a
0x80870f1: PUSH EDX
0x80870f2: PUSH dword ptr [EBP + -0x18]
0x80870f5: PUSH dword ptr [EBP + -0x10]
0x80870f8: PUSH dword ptr [EBP + -0x14]
0x80870fb: CALL 0x0807fda0
0x8087100: POP EAX
0x8087101: PUSH dword ptr [EBP + -0x14]
0x8087104: CALL 0x080891d4
0x8087109: POP EAX
0x808710a: PUSH dword ptr [EBP + -0x18]
0x808710d: CALL 0x080891d4
0x8087112: MOV EDX, dword ptr [EBP + -0xc]
0x8087115: ADD ESP, 0x10
0x8087118: LEA EAX, [EDX + -0x1]
0x808711b: TEST EDX, EDX
0x808711d: MOV dword ptr [EBP + -0xc], EAX
0x8087120: JG 0x080870b8
0x8087122: LEA ESP, [EBP + -0x8]
0x8087125: POP EBX
0x8087126: MOV EAX, 0x1
0x808712b: POP ESI
0x808712c: LEAVE
0x808712d: RET
0x8087130: SUB ESP, 0xc
0x8087133: PUSH 0x82ab7e8
0x8087138: CALL 0x08097acc
0x808713d: LEA ESP, [EBP + -0x8]
0x8087140: POP EBX
0x8087141: XOR EAX, EAX
0x8087143: POP ESI
0x8087144: LEAVE
0x8087145: RET
0x8087146: SUB ESP, 0xc
0x8087149: PUSH 0x82ab810
0x808714e: JMP 0x08087138
0x8087150: SUB ESP, 0xc
0x8087153: PUSH 0x82ab838
0x8087158: JMP 0x08087138
0x808715a: SUB ESP, 0xc
0x808715d: PUSH 0x82ab860
0x8087162: JMP 0x08087138
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_getActionInfo"></a>

## vm_getActionInfo @ 0x80a369c

Evidence: `ev_bcb6905e685879e86d98526c252dafd8952d139f64a42ad75143174fb5998ef1`

```c

Vm_Action_Info_T * vm_getActionInfo(int no)

{
  Vm_Action_Info_T *pVVar1;
  
  pVVar1 = (Vm_Action_Info_T *)0x0;
  if ((uint)no < 0x57) {
    pVVar1 = actionInfoTable + no;
  }
  return pVVar1;
}


```

<details><summary>汇编</summary>

```asm
0x80a369c: PUSH EBP
0x80a369d: MOV EBP, ESP
0x80a369f: MOV EDX, dword ptr [EBP + 0x8]
0x80a36a2: XOR EAX, EAX
0x80a36a4: CMP EDX, 0x56
0x80a36a7: JA 0x080a36b9
0x80a36a9: LEA EAX, [EDX*0x8 + 0x0]
0x80a36b0: SUB EAX, EDX
0x80a36b2: LEA EAX, [EAX*0x4 + 0x83cc680]
0x80a36b9: LEAVE
0x80a36ba: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_packedFsFileOpen"></a>

## vm_packedFsFileOpen @ 0x80b41a0

Evidence: `ev_c003de595bf5c47c226e25b04bee116d1c5c27cccbb72333f60ae8a2d30feeb7`

```c

/* WARNING: Unknown calling convention */

int vm_packedFsFileOpen(char *pFileName)

{
  char cVar1;
  char *pcVar2;
  int iVar3;
  uint uVar4;
  char *__n;
  char *pcVar5;
  uint attrib;
  int srcLen;
  uchar *buf;
  char path [256];
  char packedName [256];
  
                    /* Unresolved local var: Vm_Memory_Fp_T * fi@[DW_OP_reg3(EBX)]
                       Unresolved local var: int fd@[DW_OP_reg6(ESI)] */
                    /* Unresolved local var: uint fileNameLen@[DW_OP_reg3(EBX)]
                       Unresolved local var: char * p@[???]
                       Unresolved local var: char * p2@[DW_OP_reg6(ESI)] */
  pcVar2 = strstr(pFileName,".pak/");
  if (pcVar2 == (char *)0x0) {
    uVar4 = 0xffffffff;
    pcVar2 = pFileName;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    __n = (char *)(~uVar4 - 1);
    pcVar2 = pFileName + (int)__n;
  }
  else {
    __n = pcVar2 + (4 - (int)pFileName);
    pcVar2 = pcVar2 + 5;
  }
  if (__n < (char *)0x100) {
    uVar4 = 0xffffffff;
    pcVar5 = pcVar2;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar5;
      pcVar5 = pcVar5 + 1;
    } while (cVar1 != '\0');
    if (~uVar4 - 1 < 0x100) {
      memcpy(packedName,pFileName,(size_t)__n);
      packedName[(int)__n] = '\0';
      strcpy(path,pcVar2);
      iVar3 = fm_unpack(packedName,path,&buf,&srcLen,&attrib);
      if (iVar3 != 0) {
        if (((byte)attrib & 1) == 0) {
          pcVar2 = pdb_allocateMemory(0x10c,"VM","/home/cc/xxxx/gameshell/src/vm/vm_packedfs.c",0x79
                                     );
          if (pcVar2 != (char *)0x0) {
            memset(pcVar2,0,0x10c);
            *(uchar **)(pcVar2 + 0x100) = buf;
            pcVar2[0x108] = '\0';
            pcVar2[0x109] = '\0';
            pcVar2[0x10a] = '\0';
            pcVar2[0x10b] = '\0';
            *(int *)(pcVar2 + 0x104) = srcLen;
            strncpy(pcVar2,pFileName,0x100);
            pcVar2[0xff] = '\0';
            iVar3 = vm_open("#user",3);
            if (iVar3 < 1) {
              pdb_freeMemory(*(void **)(pcVar2 + 0x100),"VM",
                             "/home/cc/xxxx/gameshell/src/vm/vm_packedfs.c",0x8f);
              pdb_freeMemory(pcVar2,"VM","/home/cc/xxxx/gameshell/src/vm/vm_packedfs.c",0x90);
              return -1;
            }
            vm_setUserIo(iVar3,pcVar2,vm_packedFsFileSeek,vm_packedFsFileRead,vm_packedFsFileWrite,
                         (Vm_Io_Read_Line_Instance_T)0x0,(Vm_Io_Flush_Instance_T)0x0,
                         vm_packedFsFileClose,(Vm_Io_Cntl_Instance_T)0x0);
            return iVar3;
          }
          uVar4 = 0x7d;
        }
        else {
          uVar4 = 0x72;
        }
        pdb_freeMemory(buf,"VM","/home/cc/xxxx/gameshell/src/vm/vm_packedfs.c",uVar4);
      }
    }
  }
  return -1;
}


```

<details><summary>汇编</summary>

```asm
0x80b41a0: PUSH EBP
0x80b41a1: MOV EBP, ESP
0x80b41a3: PUSH EDI
0x80b41a4: PUSH ESI
0x80b41a5: PUSH EBX
0x80b41a6: SUB ESP, 0x224
0x80b41ac: PUSH 0x82af69a
0x80b41b1: PUSH dword ptr [EBP + 0x8]
0x80b41b4: CALL 0x0804b554
0x80b41b9: ADD ESP, 0x10
0x80b41bc: TEST EAX, EAX
0x80b41be: MOV EDX, EAX
0x80b41c0: JZ 0x080b4310
0x80b41c6: SUB EAX, dword ptr [EBP + 0x8]
0x80b41c9: LEA EBX, [EAX + 0x4]
0x80b41cc: LEA ESI, [EDX + 0x5]
0x80b41cf: CMP EBX, 0x100
0x80b41d5: JNC 0x080b41ee
0x80b41d7: XOR EAX, EAX
0x80b41d9: CLD
0x80b41da: MOV ECX, 0xffffffff
0x80b41df: MOV EDI, ESI
0x80b41e1: SCASB.REPNE ES:EDI
0x80b41e3: NOT ECX
0x80b41e5: DEC ECX
0x80b41e6: CMP ECX, 0x100
0x80b41ec: JC 0x080b41fc
0x80b41ee: MOV EAX, 0xffffffff
0x80b41f3: LEA ESP, [EBP + -0xc]
0x80b41f6: POP EBX
0x80b41f7: POP ESI
0x80b41f8: POP EDI
0x80b41f9: LEAVE
0x80b41fa: RET
0x80b41fc: PUSH EAX
0x80b41fd: PUSH EBX
0x80b41fe: PUSH dword ptr [EBP + 0x8]
0x80b4201: LEA EAX, [EBP + 0xfffffee8]
0x80b4207: PUSH EAX
0x80b4208: CALL 0x0804b7e4
0x80b420d: POP EDI
0x80b420e: POP EAX
0x80b420f: PUSH ESI
0x80b4210: LEA EAX, [EBP + 0xfffffde8]
0x80b4216: PUSH EAX
0x80b4217: MOV byte ptr [EBP + EBX*0x1 + 0xfffffee8], 0x0
0x80b421f: CALL 0x0804bc54
0x80b4224: LEA EAX, [EBP + 0xfffffddc]
0x80b422a: MOV dword ptr [ESP], EAX
0x80b422d: LEA EAX, [EBP + 0xfffffde0]
0x80b4233: PUSH EAX
0x80b4234: LEA EAX, [EBP + 0xfffffde4]
0x80b423a: PUSH EAX
0x80b423b: LEA EAX, [EBP + 0xfffffde8]
0x80b4241: PUSH EAX
0x80b4242: LEA EAX, [EBP + 0xfffffee8]
0x80b4248: PUSH EAX
0x80b4249: CALL 0x080b5500
0x80b424e: ADD ESP, 0x20
0x80b4251: TEST EAX, EAX
0x80b4253: JZ 0x080b41ee
0x80b4255: TEST byte ptr [EBP + 0xfffffddc], 0x1
0x80b425c: JNZ 0x080b432c
0x80b4262: PUSH 0x79
0x80b4264: PUSH 0x82b130c
0x80b4269: PUSH 0x82a376b
0x80b426e: PUSH 0x10c
0x80b4273: CALL 0x080ce784
0x80b4278: ADD ESP, 0x10
0x80b427b: TEST EAX, EAX
0x80b427d: MOV EBX, EAX
0x80b427f: JZ 0x080b437c
0x80b4285: PUSH ECX
0x80b4286: PUSH 0x10c
0x80b428b: PUSH 0x0
0x80b428d: PUSH EAX
0x80b428e: CALL 0x0804ba54
0x80b4293: MOV EAX, dword ptr [EBP + 0xfffffde4]
0x80b4299: ADD ESP, 0xc
0x80b429c: MOV dword ptr [EBX + 0x100], EAX
0x80b42a2: MOV EAX, dword ptr [EBP + 0xfffffde0]
0x80b42a8: MOV dword ptr [EBX + 0x108], 0x0
0x80b42b2: MOV dword ptr [EBX + 0x104], EAX
0x80b42b8: PUSH 0x100
0x80b42bd: PUSH dword ptr [EBP + 0x8]
0x80b42c0: PUSH EBX
0x80b42c1: CALL 0x0804ba94
0x80b42c6: MOV byte ptr [EBX + 0xff], 0x0
0x80b42cd: POP EAX
0x80b42ce: POP EDX
0x80b42cf: PUSH 0x3
0x80b42d1: PUSH 0x82af0e7
0x80b42d6: CALL 0x080a1530
0x80b42db: ADD ESP, 0x10
0x80b42de: TEST EAX, EAX
0x80b42e0: MOV ESI, EAX
0x80b42e2: JLE 0x080b4348
0x80b42e4: SUB ESP, 0xc
0x80b42e7: PUSH 0x0
0x80b42e9: PUSH 0x80b4598
0x80b42ee: PUSH 0x0
0x80b42f0: PUSH 0x0
0x80b42f2: PUSH 0x80b44c0
0x80b42f7: PUSH 0x80b4460
0x80b42fc: PUSH 0x80b4450
0x80b4301: PUSH EBX
0x80b4302: PUSH EAX
0x80b4303: CALL 0x080a025c
0x80b4308: MOV EAX, ESI
0x80b430a: JMP 0x080b41f3
0x80b4310: CLD
0x80b4311: MOV ECX, 0xffffffff
0x80b4316: XOR EAX, EAX
0x80b4318: MOV EDI, dword ptr [EBP + 0x8]
0x80b431b: SCASB.REPNE ES:EDI
0x80b431d: NOT ECX
0x80b431f: LEA EBX, [ECX + -0x1]
0x80b4322: MOV ESI, dword ptr [EBP + 0x8]
0x80b4325: ADD ESI, EBX
0x80b4327: JMP 0x080b41cf
0x80b432c: PUSH 0x72
0x80b432e: PUSH 0x82b130c
0x80b4333: PUSH 0x82a376b
0x80b4338: PUSH dword ptr [EBP + 0xfffffde4]
0x80b433e: CALL 0x080ceb94
0x80b4343: JMP 0x080b41ee
0x80b4348: PUSH 0x8f
0x80b434d: PUSH 0x82b130c
0x80b4352: PUSH 0x82a376b
0x80b4357: PUSH dword ptr [EBX + 0x100]
0x80b435d: CALL 0x080ceb94
0x80b4362: PUSH 0x90
0x80b4367: PUSH 0x82b130c
0x80b436c: PUSH 0x82a376b
0x80b4371: PUSH EBX
0x80b4372: CALL 0x080ceb94
0x80b4377: JMP 0x080b41ee
0x80b437c: PUSH 0x7d
0x80b437e: JMP 0x080b432e
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadBinaryInherit"></a>

## vm_loadBinaryInherit @ 0x80a2b58

Evidence: `ev_c41c22c1afd1192242c38bb59ffa5cc3402d157828307978e735721a4489e3d0`

```c

int vm_loadBinaryInherit(Vm_Binary_Buffer_T *pInput,char *name)

{
  short sVar1;
  short sVar2;
  Vm_Program_T *pFromProgram;
  Vm_Program_T *pVVar3;
  Vm_Index_T VVar4;
  Vm_Object_T *pVVar5;
  int iVar6;
  Vm_Program_Consititution_T *pVVar7;
  bool bVar8;
  bool bVar9;
  char *str;
  Vm_Index_T i_1;
  Vm_Program_Consititution_T *pFromConsititution;
  Vm_Program_Consititution_T *pThisConsititution;
  Vm_Index_T i;
  Vm_Index_T expectedVariableNo;
  Vm_Index_T expectedFunctionNo;
  Vm_Index_T expectedSubConsititutionNo;
  Vm_Index_T baseVarOffset;
  Vm_Index_T baseFunOffset;
  Vm_Index_T baseConsititutionOffset;
  
                    /* Unresolved local var: Vm_Object_T * pFromObject@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Program_T * pFromProgram@[DW_OP_reg6(ESI)] */
  if (vm_pCurrentProgram == (Vm_Program_T *)0x0) {
    str = "Can not inherit from object outside an object.\n";
  }
  else {
    pVVar5 = vm_lookupObjectByCStr(name);
    if (pVVar5 == (Vm_Object_T *)0x0) {
      str = "Not found object \"%s\" to inherit.\n";
    }
    else {
      pFromProgram = pVVar5->pProgram;
      iVar6 = vm_loadVmIndex(pInput,&baseConsititutionOffset);
      if (iVar6 == 0) {
        str = "Inherit program failed: read baseConsititutionOffset error.\n";
      }
      else {
        iVar6 = vm_loadVmIndex(pInput,&baseFunOffset);
        if ((iVar6 == 0) || (baseFunOffset < 0)) {
          str = "Inherit program failed: read baseFunOffset error.\n";
        }
        else {
          iVar6 = vm_loadVmIndex(pInput,&baseVarOffset);
          if ((iVar6 == 0) || (_expectedSubConsititutionNo < 0)) {
            str = "Inherit program failed: read baseVarOffset error.\n";
          }
          else {
            iVar6 = vm_loadVmIndex(pInput,&expectedSubConsititutionNo);
            if (iVar6 == 0) {
              str = "Inherit program failed: read expectedSubConsititutionNo error.\n";
            }
            else {
              iVar6 = vm_loadVmIndex(pInput,&expectedFunctionNo);
              if (iVar6 == 0) {
                str = "Inherit program failed: read expectedFunctionNo error.\n";
              }
              else {
                iVar6 = vm_loadVmIndex(pInput,&expectedVariableNo);
                pVVar3 = vm_pCurrentProgram;
                if (iVar6 == 0) {
                  str = "Inherit program failed: read expectedVariableNo error.\n";
                }
                else if (pFromProgram->maxConsititutionNo == (short)_expectedSubConsititutionNo) {
                  if (pFromProgram->maxFunctionNo == expectedFunctionNo) {
                    if (pFromProgram->maxVariableNo == (short)_expectedVariableNo) {
                      if ((int)vm_pCurrentProgram->maxFunctionNo <
                          (int)(short)_baseFunOffset + (int)pFromProgram->maxFunctionNo) {
                        str = "Bad program to inherit, max functions (%d) mimstached.\n";
                      }
                      else {
                        if ((int)baseVarOffset + (int)pFromProgram->maxVariableNo <=
                            (int)vm_pCurrentProgram->maxVariableNo) {
                          pThisConsititution =
                               vm_pCurrentProgram->pConsititutions + baseConsititutionOffset;
                          pFromConsititution = pFromProgram->pConsititutions;
                          i_1 = 0;
                          if (0 < pFromProgram->maxConsititutionNo) {
                            do {
                              pVVar7 = pThisConsititution;
                              for (iVar6 = 3; iVar6 != 0; iVar6 = iVar6 + -1) {
                                pVVar7->pProgram = (Vm_Program *)0x0;
                                pVVar7 = (Vm_Program_Consititution_T *)&pVVar7->attrib;
                              }
                              if (i_1 == 0) {
                                pThisConsititution->attrib = '\x01';
                              }
                              pThisConsititution->pProgram = pFromConsititution->pProgram;
                              pThisConsititution->baseFunOffset =
                                   (short)_baseFunOffset + pFromConsititution->baseFunOffset;
                              pThisConsititution->baseVarOffset =
                                   baseVarOffset + pFromConsititution->baseVarOffset;
                              iVar6 = (int)i_1;
                              i_1 = i_1 + 1;
                              pVVar3->initOrderList[baseConsititutionOffset + iVar6 + -1] =
                                   pFromProgram->initOrderList[iVar6] + baseConsititutionOffset;
                              pThisConsititution = pThisConsititution + 1;
                              pFromConsititution = pFromConsititution + 1;
                            } while (i_1 < pFromProgram->maxConsititutionNo);
                          }
                          sVar2 = pFromProgram->quickFunctionSize;
                          bVar9 = false;
                          sVar1 = 0;
                          bVar8 = sVar2 == 0;
                          VVar4 = sVar2;
                          while (!bVar8 && bVar9 == sVar2 < 0) {
                    /* Unresolved local var: Vm_Index_T strNo@[???]
                       Unresolved local var: Vm_Index_T funOffset@[DW_OP_reg3(EBX)] */
                            sVar2 = pFromProgram->pQuickFunction[sVar1].funOffset;
                            if (sVar2 != -1) {
                              vm_inheritFunctionPrototype
                                        (pFromProgram->ppStrings
                                         [pFromProgram->pQuickFunction[sVar1].strNo]->ptr,
                                         pFromProgram->pFunctions[sVar2].pFun,pFromProgram,
                                         sVar2 + (short)_baseFunOffset);
                              VVar4 = pFromProgram->quickFunctionSize;
                            }
                            sVar1 = sVar1 + 1;
                            bVar9 = SBORROW2(VVar4,sVar1);
                            sVar2 = VVar4 - sVar1;
                            bVar8 = VVar4 == sVar1;
                          }
                          return (uint)(vm_errors == 0);
                        }
                        str = "Bad program to inherit, max variables (%d) mimstached.\n";
                      }
                    }
                    else {
                      str = "Bad program to inherit, variables (%d) mismatched to (%d).\n";
                    }
                  }
                  else {
                    str = "Bad program to inherit, functions (%d) mismatched to (%d).\n";
                  }
                }
                else {
                  str = "Bad program to inherit, consititution (%d) mismatched to (%d).\n";
                }
              }
            }
          }
        }
      }
    }
  }
  vm_errprintf(str);
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x80a2b58: PUSH EBP
0x80a2b59: MOV EBP, ESP
0x80a2b5b: PUSH EDI
0x80a2b5c: PUSH ESI
0x80a2b5d: PUSH EBX
0x80a2b5e: SUB ESP, 0x2c
0x80a2b61: MOV EAX, [0x0847aac0]
0x80a2b66: TEST EAX, EAX
0x80a2b68: MOV EDI, dword ptr [EBP + 0x8]
0x80a2b6b: MOV EBX, dword ptr [EBP + 0xc]
0x80a2b6e: JNZ 0x080a2b88
0x80a2b70: SUB ESP, 0xc
0x80a2b73: PUSH 0x82afd30
0x80a2b78: CALL 0x08097acc
0x80a2b7d: XOR EAX, EAX
0x80a2b7f: LEA ESP, [EBP + -0xc]
0x80a2b82: POP EBX
0x80a2b83: POP ESI
0x80a2b84: POP EDI
0x80a2b85: LEAVE
0x80a2b86: RET
0x80a2b88: SUB ESP, 0xc
0x80a2b8b: PUSH EBX
0x80a2b8c: CALL 0x080b1054
0x80a2b91: ADD ESP, 0x10
0x80a2b94: TEST EAX, EAX
0x80a2b96: JZ 0x080a2c13
0x80a2b98: SUB ESP, 0x8
0x80a2b9b: MOV ESI, dword ptr [EAX + 0x20]
0x80a2b9e: LEA EAX, [EBP + -0xe]
0x80a2ba1: PUSH EAX
0x80a2ba2: PUSH EDI
0x80a2ba3: CALL 0x080b8b44
0x80a2ba8: ADD ESP, 0x10
0x80a2bab: TEST EAX, EAX
0x80a2bad: JNZ 0x080a2bbc
0x80a2baf: SUB ESP, 0xc
0x80a2bb2: PUSH 0x82afd60
0x80a2bb7: JMP 0x080a2b78
0x80a2bbc: SUB ESP, 0x8
0x80a2bbf: LEA EAX, [EBP + -0x10]
0x80a2bc2: PUSH EAX
0x80a2bc3: PUSH EDI
0x80a2bc4: CALL 0x080b8b44
0x80a2bc9: ADD ESP, 0x10
0x80a2bcc: TEST EAX, EAX
0x80a2bce: JZ 0x080a2c24
0x80a2bd0: CMP word ptr [EBP + -0x10], 0x0
0x80a2bd5: JS 0x080a2c24
0x80a2bd7: SUB ESP, 0x8
0x80a2bda: LEA EAX, [EBP + -0x12]
0x80a2bdd: PUSH EAX
0x80a2bde: PUSH EDI
0x80a2bdf: CALL 0x080b8b44
0x80a2be4: ADD ESP, 0x10
0x80a2be7: TEST EAX, EAX
0x80a2be9: JZ 0x080a2c31
0x80a2beb: CMP word ptr [EBP + -0x12], 0x0
0x80a2bf0: JS 0x080a2c31
0x80a2bf2: SUB ESP, 0x8
0x80a2bf5: LEA EAX, [EBP + -0x14]
0x80a2bf8: PUSH EAX
0x80a2bf9: PUSH EDI
0x80a2bfa: CALL 0x080b8b44
0x80a2bff: ADD ESP, 0x10
0x80a2c02: TEST EAX, EAX
0x80a2c04: JNZ 0x080a2c3e
0x80a2c06: SUB ESP, 0xc
0x80a2c09: PUSH 0x82afda0
0x80a2c0e: JMP 0x080a2b78
0x80a2c13: SUB ESP, 0x8
0x80a2c16: PUSH EBX
0x80a2c17: PUSH 0x82afde0
0x80a2c1c: JMP 0x080a2b78
0x80a2c24: SUB ESP, 0xc
0x80a2c27: PUSH 0x82afe04
0x80a2c2c: JMP 0x080a2b78
0x80a2c31: SUB ESP, 0xc
0x80a2c34: PUSH 0x82afe38
0x80a2c39: JMP 0x080a2b78
0x80a2c3e: SUB ESP, 0x8
0x80a2c41: LEA EAX, [EBP + -0x16]
0x80a2c44: PUSH EAX
0x80a2c45: PUSH EDI
0x80a2c46: CALL 0x080b8b44
0x80a2c4b: ADD ESP, 0x10
0x80a2c4e: TEST EAX, EAX
0x80a2c50: JNZ 0x080a2c5f
0x80a2c52: SUB ESP, 0xc
0x80a2c55: PUSH 0x82afe6c
0x80a2c5a: JMP 0x080a2b78
0x80a2c5f: SUB ESP, 0x8
0x80a2c62: LEA EAX, [EBP + -0x18]
0x80a2c65: PUSH EAX
0x80a2c66: PUSH EDI
0x80a2c67: CALL 0x080b8b44
0x80a2c6c: ADD ESP, 0x10
0x80a2c6f: TEST EAX, EAX
0x80a2c71: JZ 0x080a2cae
0x80a2c73: MOV BX, word ptr [ESI + 0x14]
0x80a2c77: MOV EAX, dword ptr [EBP + -0x14]
0x80a2c7a: CMP BX, AX
0x80a2c7d: JZ 0x080a2c90
0x80a2c7f: CWDE
0x80a2c80: PUSH ESI
0x80a2c81: PUSH EAX
0x80a2c82: MOVSX EAX, BX
0x80a2c85: PUSH EAX
0x80a2c86: PUSH 0x82afea4
0x80a2c8b: JMP 0x080a2b78
0x80a2c90: MOV CX, word ptr [ESI + 0x10]
0x80a2c94: MOV AX, word ptr [EBP + -0x16]
0x80a2c98: CMP CX, AX
0x80a2c9b: JZ 0x080a2cbb
0x80a2c9d: CWDE
0x80a2c9e: PUSH EBX
0x80a2c9f: PUSH EAX
0x80a2ca0: MOVSX EAX, CX
0x80a2ca3: PUSH EAX
0x80a2ca4: PUSH 0x82afee4
0x80a2ca9: JMP 0x080a2b78
0x80a2cae: SUB ESP, 0xc
0x80a2cb1: PUSH 0x82aff20
0x80a2cb6: JMP 0x080a2b78
0x80a2cbb: MOV DI, word ptr [ESI + 0xe]
0x80a2cbf: MOV EAX, dword ptr [EBP + -0x18]
0x80a2cc2: CMP DI, AX
0x80a2cc5: JZ 0x080a2cd8
0x80a2cc7: CWDE
0x80a2cc8: PUSH EDX
0x80a2cc9: PUSH EAX
0x80a2cca: MOVSX EAX, DI
0x80a2ccd: PUSH EAX
0x80a2cce: PUSH 0x82aff58
0x80a2cd3: JMP 0x080a2b78
0x80a2cd8: MOV EAX, dword ptr [EBP + -0x10]
0x80a2cdb: MOVSX EDX, AX
0x80a2cde: MOV word ptr [EBP + -0x1c], AX
0x80a2ce2: MOVSX EAX, CX
0x80a2ce5: MOV ECX, dword ptr [0x0847aac0]
0x80a2ceb: ADD EDX, EAX
0x80a2ced: MOV dword ptr [EBP + -0x20], ECX
0x80a2cf0: MOVSX ECX, word ptr [ECX + 0x10]
0x80a2cf4: CMP EDX, ECX
0x80a2cf6: JG 0x080a2e33
0x80a2cfc: MOV AX, word ptr [EBP + -0x12]
0x80a2d00: MOV word ptr [EBP + -0x22], AX
0x80a2d04: MOVSX EDX, DI
0x80a2d07: MOVSX EAX, word ptr [EBP + -0x22]
0x80a2d0b: MOV ECX, dword ptr [EBP + -0x20]
0x80a2d0e: ADD EAX, EDX
0x80a2d10: MOVSX EDX, word ptr [ECX + 0xe]
0x80a2d14: CMP EAX, EDX
0x80a2d16: JG 0x080a2e25
0x80a2d1c: MOV AX, word ptr [EBP + -0xe]
0x80a2d20: MOVSX EDX, AX
0x80a2d23: MOV ECX, dword ptr [EBP + -0x20]
0x80a2d26: MOV word ptr [EBP + -0x24], AX
0x80a2d2a: MOV dword ptr [EBP + -0x34], EDX
0x80a2d2d: MOV EAX, dword ptr [ECX + 0x18]
0x80a2d30: LEA EDX, [EDX + EDX*0x2]
0x80a2d33: LEA EDX, [EAX + EDX*0x4]
0x80a2d36: CMP BX, 0x0
0x80a2d3a: MOV EAX, dword ptr [ESI + 0x18]
0x80a2d3d: MOV dword ptr [EBP + -0x28], EDX
0x80a2d40: MOV dword ptr [EBP + -0x2c], EAX
0x80a2d43: MOV word ptr [EBP + -0x2e], 0x0
0x80a2d49: JLE 0x080a2dbb
0x80a2d4b: CLD
0x80a2d4c: XOR EAX, EAX
0x80a2d4e: MOV EDI, dword ptr [EBP + -0x28]
0x80a2d51: MOV ECX, 0x3
0x80a2d56: STOSD.REP ES:EDI
0x80a2d58: CMP word ptr [EBP + -0x2e], 0x0
0x80a2d5d: JNZ 0x080a2d66
0x80a2d5f: MOV EDX, dword ptr [EBP + -0x28]
0x80a2d62: MOV byte ptr [EDX + 0x4], 0x1
0x80a2d66: MOV ECX, dword ptr [EBP + -0x2c]
0x80a2d69: MOV EDX, dword ptr [EBP + -0x28]
0x80a2d6c: MOV EAX, dword ptr [ECX]
0x80a2d6e: MOV dword ptr [EDX], EAX
0x80a2d70: MOV EAX, dword ptr [EBP + -0x1c]
0x80a2d73: ADD AX, word ptr [ECX + 0x6]
0x80a2d77: MOV word ptr [EDX + 0x6], AX
0x80a2d7b: MOV AX, word ptr [EBP + -0x22]
0x80a2d7f: ADD AX, word ptr [ECX + 0x8]
0x80a2d83: MOV word ptr [EDX + 0x8], AX
0x80a2d87: MOV EAX, dword ptr [EBP + -0x20]
0x80a2d8a: MOVSX EDX, word ptr [EBP + -0x2e]
0x80a2d8e: MOV ECX, dword ptr [EAX + 0x1c]
0x80a2d91: MOV EAX, dword ptr [ESI + 0x1c]
0x80a2d94: MOV AX, word ptr [EAX + EDX*0x2]
0x80a2d98: MOV EBX, dword ptr [EBP + -0x34]
0x80a2d9b: ADD EBX, EDX
0x80a2d9d: ADD EAX, dword ptr [EBP + -0x24]
0x80a2da0: INC word ptr [EBP + -0x2e]
0x80a2da4: MOV word ptr [ECX + EBX*0x2 + -0x2], AX
0x80a2da9: MOV DX, word ptr [EBP + -0x2e]
0x80a2dad: ADD dword ptr [EBP + -0x28], 0xc
0x80a2db1: ADD dword ptr [EBP + -0x2c], 0xc
0x80a2db5: CMP word ptr [ESI + 0x14], DX
0x80a2db9: JG 0x080a2d4c
0x80a2dbb: MOV AX, word ptr [ESI + 0x5c]
0x80a2dbf: CMP AX, 0x0
0x80a2dc3: MOV word ptr [EBP + -0x1a], 0x0
0x80a2dc9: JMP 0x080a2e12
0x80a2dcb: MOVSX ECX, word ptr [EBP + -0x1a]
0x80a2dcf: MOV EDI, dword ptr [ESI + 0x60]
0x80a2dd2: MOV BX, word ptr [EDI + ECX*0x4 + 0x2]
0x80a2dd7: CMP BX, -0x1
0x80a2ddb: JZ 0x080a2e0a
0x80a2ddd: MOV EAX, dword ptr [ESI + 0x2c]
0x80a2de0: MOVSX EDX, BX
0x80a2de3: MOVSX ECX, word ptr [EDI + ECX*0x4]
0x80a2de7: MOV EDX, dword ptr [EAX + EDX*0x8]
0x80a2dea: ADD EBX, dword ptr [EBP + -0x10]
0x80a2ded: MOV EAX, dword ptr [ESI + 0x20]
0x80a2df0: MOV EAX, dword ptr [EAX + ECX*0x4]
0x80a2df3: SUB ESP, 0x8
0x80a2df6: MOVSX ECX, BX
0x80a2df9: MOV EAX, dword ptr [EAX + 0xc]
0x80a2dfc: PUSH ECX
0x80a2dfd: PUSH ESI
0x80a2dfe: CALL 0x080a28fc
0x80a2e03: ADD ESP, 0x10
0x80a2e06: MOV AX, word ptr [ESI + 0x5c]
0x80a2e0a: INC word ptr [EBP + -0x1a]
0x80a2e0e: CMP AX, word ptr [EBP + -0x1a]
0x80a2e12: JG 0x080a2dcb
0x80a2e14: XOR EAX, EAX
0x80a2e16: CMP dword ptr [0x08667a6c], 0x0
0x80a2e1d: SETZ AL
0x80a2e20: JMP 0x080a2b7f
0x80a2e25: SUB ESP, 0x8
0x80a2e28: PUSH EDX
0x80a2e29: PUSH 0x82aff94
0x80a2e2e: JMP 0x080a2b78
0x80a2e33: SUB ESP, 0x8
0x80a2e36: PUSH ECX
0x80a2e37: PUSH 0x82affcc
0x80a2e3c: JMP 0x080a2b78
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="fm_readCatalog"></a>

## fm_readCatalog @ 0x80b51ec

Evidence: `ev_cc310fbbb5402e75d2c6ecae8ccd0be9312414b23edd2556b0d93fe81d2004fa`

```c

int __regparm2 fm_readCatalog(FILE *fp,char *file,Fm_Catalog_T *pCatalog)

{
  char cVar1;
  bool bVar2;
  size_t sVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  Fm_Catalog_T *pFVar7;
  char *pcVar8;
  char *p;
  FILE *fp_local;
  char name [48];
  Fm_Catalog_T catalog;
  Fm_Header_T hdr;
  
                    /* Unresolved local var: int len@[DW_OP_reg3(EBX)]
                       Unresolved local var: char * p2@[DW_OP_reg7(EDI)]
                       Unresolved local var: int count@[DW_OP_reg6(ESI)] */
  fseek((FILE *)fp,0,0);
  sVar3 = fread(&hdr,1,0x10,(FILE *)fp);
  if (sVar3 != 0x10) {
    return 0;
  }
  uVar5 = (uint)hdr.nRootDirSize >> 6;
  if ((*file == '\0') || ((p = file, *file == '/' && (file[1] == '\0')))) {
    pCatalog->nSize = hdr.nRootDirSize;
    pCatalog->nOrgSize = hdr.nRootDirSize;
    pCatalog->name[0] = '\0';
    pCatalog->nAttrib = 1;
    pCatalog->nOffset = 0x10;
    return 1;
  }
  do {
    cVar1 = *p;
    pcVar8 = p;
    while (((cVar1 != '\0' && (cVar1 != '/')) && (cVar1 != '\\'))) {
      pcVar8 = pcVar8 + 1;
      cVar1 = *pcVar8;
    }
    sVar3 = (int)pcVar8 - (int)p;
    if (0x2f < sVar3) {
      sVar3 = 0x2f;
    }
    memcpy(name,p,sVar3);
    uVar6 = uVar5 - 1;
    name[sVar3] = '\0';
    if (0 < (int)uVar5) {
      uVar5 = uVar6;
      do {
        sVar3 = fread(&catalog,1,0x40,(FILE *)fp);
        if (sVar3 != 0x40) {
          return 0;
        }
        iVar4 = vm_stricmp(catalog.name,name);
        uVar6 = catalog.nSize;
        if (iVar4 == 0) {
          if (*pcVar8 == '\0') {
            pFVar7 = &catalog;
            for (iVar4 = 0x10; iVar4 != 0; iVar4 = iVar4 + -1) {
              pCatalog->nAttrib = pFVar7->nAttrib;
              pFVar7 = (Fm_Catalog_T *)&pFVar7->nOffset;
              pCatalog = (Fm_Catalog_T *)&pCatalog->nOffset;
            }
            return 1;
          }
          if (((byte)catalog.nAttrib & 1) == 0) {
            return 0;
          }
          fseek((FILE *)fp,catalog.nOffset,0);
          uVar6 = uVar6 >> 6;
          p = pcVar8 + 1;
          break;
        }
        uVar6 = uVar5 - 1;
        bVar2 = 0 < (int)uVar5;
        uVar5 = uVar6;
      } while (bVar2);
    }
    uVar5 = uVar6;
    if ((int)uVar6 < 1) {
      return 0;
    }
  } while( true );
}


```

<details><summary>汇编</summary>

```asm
0x80b51ec: PUSH EBP
0x80b51ed: MOV EBP, ESP
0x80b51ef: PUSH EDI
0x80b51f0: PUSH ESI
0x80b51f1: PUSH EBX
0x80b51f2: SUB ESP, 0xa0
0x80b51f8: MOV dword ptr [EBP + 0xffffff64], EAX
0x80b51fe: PUSH 0x0
0x80b5200: PUSH 0x0
0x80b5202: PUSH EAX
0x80b5203: MOV EBX, EDX
0x80b5205: CALL 0x0804b4b4
0x80b520a: PUSH dword ptr [EBP + 0xffffff64]
0x80b5210: PUSH 0x10
0x80b5212: PUSH 0x1
0x80b5214: LEA EAX, [EBP + -0x28]
0x80b5217: PUSH EAX
0x80b5218: CALL 0x0804b3d4
0x80b521d: ADD ESP, 0x20
0x80b5220: XOR EDX, EDX
0x80b5222: CMP EAX, 0x10
0x80b5225: JZ 0x080b5234
0x80b5227: LEA ESP, [EBP + -0xc]
0x80b522a: POP EBX
0x80b522b: POP ESI
0x80b522c: MOV EAX, EDX
0x80b522e: POP EDI
0x80b522f: LEAVE
0x80b5230: RET
0x80b5234: MOV EDX, dword ptr [EBP + -0x24]
0x80b5237: MOV ESI, EDX
0x80b5239: MOV AL, byte ptr [EBX]
0x80b523b: SHR ESI, 0x6
0x80b523e: TEST AL, AL
0x80b5240: JZ 0x080b5306
0x80b5246: CMP AL, 0x2f
0x80b5248: JZ 0x080b52fc
0x80b524e: MOV dword ptr [EBP + 0xffffff60], EBX
0x80b5254: MOV EDI, dword ptr [EBP + 0xffffff60]
0x80b525a: MOV AL, byte ptr [EDI]
0x80b525c: TEST AL, AL
0x80b525e: JZ 0x080b5270
0x80b5260: CMP AL, 0x2f
0x80b5262: JZ 0x080b5270
0x80b5264: CMP AL, 0x5c
0x80b5266: JZ 0x080b5270
0x80b5268: INC EDI
0x80b5269: MOV AL, byte ptr [EDI]
0x80b526b: TEST AL, AL
0x80b526d: JNZ 0x080b5260
0x80b526f: NOP
0x80b5270: MOV EBX, EDI
0x80b5272: SUB EBX, dword ptr [EBP + 0xffffff60]
0x80b5278: CMP EBX, 0x30
0x80b527b: MOV EAX, 0x2f
0x80b5280: CMOVNC EBX, EAX
0x80b5283: PUSH EAX
0x80b5284: PUSH EBX
0x80b5285: PUSH dword ptr [EBP + 0xffffff60]
0x80b528b: LEA EDX, [EBP + 0xffffff68]
0x80b5291: PUSH EDX
0x80b5292: CALL 0x0804b7e4
0x80b5297: MOV EAX, ESI
0x80b5299: ADD ESP, 0x10
0x80b529c: DEC ESI
0x80b529d: TEST EAX, EAX
0x80b529f: MOV byte ptr [EBP + EBX*0x1 + 0xffffff68], 0x0
0x80b52a7: JLE 0x080b52e8
0x80b52a9: LEA EBX, [EDI + 0x1]
0x80b52ac: PUSH dword ptr [EBP + 0xffffff64]
0x80b52b2: PUSH 0x40
0x80b52b4: PUSH 0x1
0x80b52b6: LEA EAX, [EBP + -0x68]
0x80b52b9: PUSH EAX
0x80b52ba: CALL 0x0804b3d4
0x80b52bf: ADD ESP, 0x10
0x80b52c2: CMP EAX, 0x40
0x80b52c5: JNZ 0x080b52f0
0x80b52c7: SUB ESP, 0x8
0x80b52ca: LEA EDX, [EBP + 0xffffff68]
0x80b52d0: PUSH EDX
0x80b52d1: LEA EAX, [EBP + -0x54]
0x80b52d4: PUSH EAX
0x80b52d5: CALL 0x080b67e0
0x80b52da: ADD ESP, 0x10
0x80b52dd: TEST EAX, EAX
0x80b52df: JZ 0x080b532f
0x80b52e1: MOV EAX, ESI
0x80b52e3: DEC ESI
0x80b52e4: TEST EAX, EAX
0x80b52e6: JG 0x080b52ac
0x80b52e8: TEST ESI, ESI
0x80b52ea: JG 0x080b5254
0x80b52f0: LEA ESP, [EBP + -0xc]
0x80b52f3: POP EBX
0x80b52f4: POP ESI
0x80b52f5: XOR EDX, EDX
0x80b52f7: MOV EAX, EDX
0x80b52f9: POP EDI
0x80b52fa: LEAVE
0x80b52fb: RET
0x80b52fc: CMP byte ptr [EBX + 0x1], 0x0
0x80b5300: JNZ 0x080b524e
0x80b5306: MOV EAX, dword ptr [EBP + 0x8]
0x80b5309: MOV dword ptr [EAX + 0x8], EDX
0x80b530c: MOV dword ptr [EAX + 0xc], EDX
0x80b530f: MOV byte ptr [EAX + 0x14], 0x0
0x80b5313: MOV dword ptr [EAX], 0x1
0x80b5319: MOV dword ptr [EAX + 0x4], 0x10
0x80b5320: MOV EDX, 0x1
0x80b5325: LEA ESP, [EBP + -0xc]
0x80b5328: POP EBX
0x80b5329: POP ESI
0x80b532a: MOV EAX, EDX
0x80b532c: POP EDI
0x80b532d: LEAVE
0x80b532e: RET
0x80b532f: CMP byte ptr [EDI], 0x0
0x80b5332: JZ 0x080b535c
0x80b5334: TEST byte ptr [EBP + -0x68], 0x1
0x80b5338: JZ 0x080b52f0
0x80b533a: MOV ESI, dword ptr [EBP + -0x60]
0x80b533d: MOV dword ptr [EBP + 0xffffff60], EBX
0x80b5343: PUSH EAX
0x80b5344: PUSH 0x0
0x80b5346: PUSH dword ptr [EBP + -0x64]
0x80b5349: PUSH dword ptr [EBP + 0xffffff64]
0x80b534f: CALL 0x0804b4b4
0x80b5354: SHR ESI, 0x6
0x80b5357: ADD ESP, 0x10
0x80b535a: JMP 0x080b52e8
0x80b535c: CLD
0x80b535d: MOV ECX, 0x10
0x80b5362: MOV EDI, dword ptr [EBP + 0x8]
0x80b5365: LEA ESI, [EBP + -0x68]
0x80b5368: MOVSD.REP ES:EDI, ESI
0x80b536a: MOV EDX, 0x1
0x80b536f: JMP 0x080b5325
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_efunDebugInit"></a>

## vm_efunDebugInit @ 0x80fb104

Evidence: `ev_cfffcc2bf9e79390ad720cae98403ef18bbf49e2c69ab7c5ff062cc96461c29a`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_efunDebugInit(void)

{
  vm_addEfuns(efunsDebug);
  vm_addPredefine("_debug_mode",0,-1,"\"[#@B:_debug_mode]0[#@E]\"");
  return;
}


```

<details><summary>汇编</summary>

```asm
0x80fb104: PUSH EBP
0x80fb105: MOV EBP, ESP
0x80fb107: SUB ESP, 0x14
0x80fb10a: PUSH 0x83d2d00
0x80fb10f: CALL 0x08085ce4
0x80fb114: PUSH 0x82c0008
0x80fb119: PUSH -0x1
0x80fb11b: PUSH 0x0
0x80fb11d: PUSH 0x82b3d1f
0x80fb122: CALL 0x08080128
0x80fb127: LEAVE
0x80fb128: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_saveBinary"></a>

## vm_saveBinary @ 0x80b9018

Evidence: `ev_d34902dd8325b033fe28dc4d312b31a3c6345f983ae345459553f9f3ed0ed9b4`

```c

/* WARNING: Type propagation algorithm not settling */
/* WARNING: Unknown calling convention */

int vm_saveBinary(char *binFileName,Vm_Code_Segment_T *pCodeSegment,int saveDefine)

{
  char cVar1;
  int iVar2;
  Vm_Program *pVVar3;
  int iVar4;
  uint uVar5;
  char *pcVar6;
  char *pBuffer;
  int i;
  Vm_Program_T *pProgram;
  int ret;
  int fd;
  uint local_138;
  char local_134 [4];
  int local_130 [3];
  undefined4 local_124;
  undefined4 local_120;
  char localFileName [256];
  
  iVar2 = vm_generateLocalFileName(binFileName,localFileName,0x100);
  if (iVar2 == 0) {
    return 0;
  }
  iVar2 = vm_assurePath(localFileName);
  if (iVar2 == 0) {
    return 0;
  }
  iVar2 = vm_open(localFileName,2);
  if (iVar2 < 0) {
    return 0;
  }
                    /* Unresolved local var: int num32@[???] */
  ret = 0;
  local_120 = 0x10001;
  vm_write(&local_120,4,iVar2);
                    /* Unresolved local var: int num32@[???] */
  local_124 = 0;
  vm_write(&local_124,4,iVar2);
                    /* Unresolved local var: int num32@[???] */
  local_130[2] = (int)vm_functionBC;
  vm_write(local_130 + 2,4,iVar2);
  if (saveDefine == 0) {
                    /* Unresolved local var: int num32@[???] */
    local_130[1] = 0;
    vm_write(local_130 + 1,4,iVar2);
  }
  else {
    iVar4 = vm_saveDefines(iVar2);
    if (iVar4 == 0) goto LAB_080b91f0;
  }
  pVVar3 = vm_compilingProgram();
  local_130[0] = (int)pVVar3->maxStringNo;
                    /* Unresolved local var: int num32@[???] */
  vm_write(local_130,4,iVar2);
  i = 0;
  if (0 < pVVar3->maxStringNo) {
    do {
      pBuffer = pVVar3->ppStrings[i]->ptr;
                    /* Unresolved local var: int size@[???] */
      if (pBuffer == (char *)0x0) {
                    /* Unresolved local var: int num32@[???] */
        uVar5 = 4;
        pBuffer = local_134;
        local_134[0] = '\0';
        local_134[1] = '\0';
        local_134[2] = '\0';
        local_134[3] = '\0';
LAB_080b91cd:
        vm_write(pBuffer,uVar5,iVar2);
      }
      else {
        uVar5 = 0xffffffff;
        pcVar6 = pBuffer;
        do {
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          cVar1 = *pcVar6;
          pcVar6 = pcVar6 + 1;
        } while (cVar1 != '\0');
                    /* Unresolved local var: int num32@[???] */
        uVar5 = ~uVar5;
        local_138 = uVar5;
        iVar4 = vm_write(&local_138,4,iVar2);
        if (iVar4 == 4) goto LAB_080b91cd;
      }
      i = i + 1;
    } while (i < pVVar3->maxStringNo);
  }
  iVar4 = vm_saveIdentifiers(iVar2);
  if (iVar4 != 0) {
    iVar4 = vm_saveCodeSegment(iVar2,pCodeSegment);
    ret = (int)(iVar4 != 0);
  }
LAB_080b91f0:
  vm_close(iVar2);
  return ret;
}


```

<details><summary>汇编</summary>

```asm
0x80b9018: PUSH EBP
0x80b9019: MOV EBP, ESP
0x80b901b: PUSH EDI
0x80b901c: PUSH ESI
0x80b901d: PUSH EBX
0x80b901e: SUB ESP, 0x140
0x80b9024: PUSH 0x100
0x80b9029: LEA EBX, [EBP + 0xfffffee8]
0x80b902f: PUSH EBX
0x80b9030: PUSH dword ptr [EBP + 0x8]
0x80b9033: CALL 0x080be1fc
0x80b9038: ADD ESP, 0x10
0x80b903b: XOR EDX, EDX
0x80b903d: TEST EAX, EAX
0x80b903f: JNZ 0x080b904c
0x80b9041: LEA ESP, [EBP + -0xc]
0x80b9044: POP EBX
0x80b9045: POP ESI
0x80b9046: MOV EAX, EDX
0x80b9048: POP EDI
0x80b9049: LEAVE
0x80b904a: RET
0x80b904c: SUB ESP, 0xc
0x80b904f: PUSH EBX
0x80b9050: CALL 0x080eadac
0x80b9055: ADD ESP, 0x10
0x80b9058: XOR EDX, EDX
0x80b905a: TEST EAX, EAX
0x80b905c: JZ 0x080b9041
0x80b905e: SUB ESP, 0x8
0x80b9061: PUSH 0x2
0x80b9063: PUSH EBX
0x80b9064: CALL 0x080a1530
0x80b9069: ADD ESP, 0x10
0x80b906c: XOR EDX, EDX
0x80b906e: TEST EAX, EAX
0x80b9070: MOV dword ptr [EBP + 0xfffffec8], EAX
0x80b9076: JS 0x080b9041
0x80b9078: PUSH EDX
0x80b9079: PUSH EAX
0x80b907a: PUSH 0x4
0x80b907c: LEA EAX, [EBP + 0xfffffee4]
0x80b9082: PUSH EAX
0x80b9083: MOV dword ptr [EBP + 0xfffffec4], 0x0
0x80b908d: MOV dword ptr [EBP + 0xfffffee4], 0x10001
0x80b9097: CALL 0x080a0b00
0x80b909c: ADD ESP, 0xc
0x80b909f: PUSH dword ptr [EBP + 0xfffffec8]
0x80b90a5: PUSH 0x4
0x80b90a7: LEA EAX, [EBP + 0xfffffee0]
0x80b90ad: PUSH EAX
0x80b90ae: MOV dword ptr [EBP + 0xfffffee0], 0x0
0x80b90b8: CALL 0x080a0b00
0x80b90bd: ADD ESP, 0xc
0x80b90c0: PUSH dword ptr [EBP + 0xfffffec8]
0x80b90c6: MOVSX EAX, word ptr [0x08660900]
0x80b90cd: PUSH 0x4
0x80b90cf: MOV dword ptr [EBP + 0xfffffedc], EAX
0x80b90d5: LEA EAX, [EBP + 0xfffffedc]
0x80b90db: PUSH EAX
0x80b90dc: CALL 0x080a0b00
0x80b90e1: MOV EAX, dword ptr [EBP + 0x10]
0x80b90e4: ADD ESP, 0x10
0x80b90e7: TEST EAX, EAX
0x80b90e9: JNZ 0x080b91d7
0x80b90ef: PUSH EAX
0x80b90f0: PUSH dword ptr [EBP + 0xfffffec8]
0x80b90f6: PUSH 0x4
0x80b90f8: LEA EAX, [EBP + 0xfffffed8]
0x80b90fe: PUSH EAX
0x80b90ff: MOV dword ptr [EBP + 0xfffffed8], 0x0
0x80b9109: CALL 0x080a0b00
0x80b910e: ADD ESP, 0x10
0x80b9111: CALL 0x0807ecfc
0x80b9116: MOV dword ptr [EBP + 0xfffffec0], EAX
0x80b911c: MOVSX EAX, word ptr [EAX + 0x24]
0x80b9120: MOV dword ptr [EBP + 0xfffffed4], EAX
0x80b9126: PUSH EAX
0x80b9127: PUSH dword ptr [EBP + 0xfffffec8]
0x80b912d: PUSH 0x4
0x80b912f: LEA EAX, [EBP + 0xfffffed4]
0x80b9135: PUSH EAX
0x80b9136: CALL 0x080a0b00
0x80b913b: MOV EAX, dword ptr [EBP + 0xfffffec0]
0x80b9141: ADD ESP, 0x10
0x80b9144: CMP word ptr [EAX + 0x24], 0x0
0x80b9149: MOV dword ptr [EBP + 0xfffffebc], 0x0
0x80b9153: JG 0x080b9178
0x80b9155: JMP 0x080b9225
0x80b915c: MOV EDX, dword ptr [EBP + 0xfffffec0]
0x80b9162: INC dword ptr [EBP + 0xfffffebc]
0x80b9168: MOVSX EAX, word ptr [EDX + 0x24]
0x80b916c: CMP EAX, dword ptr [EBP + 0xfffffebc]
0x80b9172: JLE 0x080b9225
0x80b9178: MOV EDX, dword ptr [EBP + 0xfffffec0]
0x80b917e: MOV EAX, dword ptr [EDX + 0x20]
0x80b9181: MOV ECX, dword ptr [EBP + 0xfffffebc]
0x80b9187: MOV EAX, dword ptr [EAX + ECX*0x4]
0x80b918a: MOV ESI, dword ptr [EAX + 0xc]
0x80b918d: TEST ESI, ESI
0x80b918f: JZ 0x080b9209
0x80b9191: MOV EDI, ESI
0x80b9193: XOR EAX, EAX
0x80b9195: CLD
0x80b9196: MOV ECX, 0xffffffff
0x80b919b: SCASB.REPNE ES:EDI
0x80b919d: PUSH EDI
0x80b919e: PUSH dword ptr [EBP + 0xfffffec8]
0x80b91a4: PUSH 0x4
0x80b91a6: MOV EBX, ECX
0x80b91a8: LEA EAX, [EBP + 0xfffffecc]
0x80b91ae: PUSH EAX
0x80b91af: NOT EBX
0x80b91b1: MOV dword ptr [EBP + 0xfffffecc], EBX
0x80b91b7: CALL 0x080a0b00
0x80b91bc: ADD ESP, 0x10
0x80b91bf: CMP EAX, 0x4
0x80b91c2: JNZ 0x080b915c
0x80b91c4: PUSH ECX
0x80b91c5: PUSH dword ptr [EBP + 0xfffffec8]
0x80b91cb: PUSH EBX
0x80b91cc: PUSH ESI
0x80b91cd: CALL 0x080a0b00
0x80b91d2: ADD ESP, 0x10
0x80b91d5: JMP 0x080b915c
0x80b91d7: SUB ESP, 0xc
0x80b91da: PUSH dword ptr [EBP + 0xfffffec8]
0x80b91e0: CALL 0x08086fb8
0x80b91e5: ADD ESP, 0x10
0x80b91e8: TEST EAX, EAX
0x80b91ea: JNZ 0x080b9111
0x80b91f0: SUB ESP, 0xc
0x80b91f3: PUSH dword ptr [EBP + 0xfffffec8]
0x80b91f9: CALL 0x080a0504
0x80b91fe: MOV EDX, dword ptr [EBP + 0xfffffec4]
0x80b9204: JMP 0x080b9041
0x80b9209: PUSH EAX
0x80b920a: PUSH dword ptr [EBP + 0xfffffec8]
0x80b9210: PUSH 0x4
0x80b9212: LEA EAX, [EBP + 0xfffffed0]
0x80b9218: PUSH EAX
0x80b9219: MOV dword ptr [EBP + 0xfffffed0], 0x0
0x80b9223: JMP 0x080b91cd
0x80b9225: SUB ESP, 0xc
0x80b9228: PUSH dword ptr [EBP + 0xfffffec8]
0x80b922e: CALL 0x08087aec
0x80b9233: ADD ESP, 0x10
0x80b9236: TEST EAX, EAX
0x80b9238: JZ 0x080b91f0
0x80b923a: SUB ESP, 0x8
0x80b923d: PUSH dword ptr [EBP + 0xc]
0x80b9240: PUSH dword ptr [EBP + 0xfffffec8]
0x80b9246: CALL 0x08095f3c
0x80b924b: ADD ESP, 0x10
0x80b924e: TEST EAX, EAX
0x80b9250: MOV EAX, 0x1
0x80b9255: CMOVZ EAX, dword ptr [EBP + 0xfffffec4]
0x80b925c: MOV dword ptr [EBP + 0xfffffec4], EAX
0x80b9262: JMP 0x080b91f0
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadBinary"></a>

## vm_loadBinary @ 0x80b9264

Evidence: `ev_d36be5e76af4173eaa042416c1354f4ac89d5b3a1f9e8e0ac7a3de77088f54d0`

```c

int vm_loadBinary(int fd,Vm_Code_Segment_T *pCodeSegment)

{
  Vm_Index_T VVar1;
  Vm_Program *pProgram_00;
  Vm_String **ppVVar2;
  char *__dest;
  uint uVar3;
  int iVar4;
  char *pBuffer;
  uint len;
  Vm_Code_Segment_T *pVVar5;
  bool bVar6;
  void *buf_1;
  int i;
  Vm_Program_T *pProgram;
  int ret;
  char *str_1;
  int size;
  char *str;
  int newCount;
  int num;
  Vm_Binary_Buffer_T buf;
  
  pVVar5 = pCodeSegment;
                    /* Unresolved local var: Vm_Binary_Buffer_T * pInput@[???] */
  for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
    pVVar5->nodesCount = 0;
    pVVar5 = (Vm_Code_Segment_T *)&pVVar5->nodes;
  }
  buf.fd = fd;
  buf.p = buf.data;
  buf.len = 0;
  iVar4 = vm_fetchBinaryBufferData(&buf);
                    /* Unresolved local var: int temp@[DW_OP_reg2(EDX)] */
  if ((iVar4 != 0) && ((3 < (uint)buf.len || (vm_fetchBinaryBufferData(&buf), 3 < (uint)buf.len))))
  {
    iVar4 = *(int *)buf.p;
    buf.p = buf.p + 4;
    buf.len = buf.len - 4;
    if (&stack0x00000000 != (undefined1 *)0xfd0) {
      num = iVar4;
    }
    if (num == 0x10001) {
                    /* Unresolved local var: int temp@[???] */
      if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
        return 0;
      }
      buf.len = buf.len - 4;
      buf.p = buf.p + 4;
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
      if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
        return 0;
      }
      iVar4 = *(int *)buf.p;
      buf.p = buf.p + 4;
      buf.len = buf.len - 4;
      if (&stack0x00000000 != (undefined1 *)0xfd0) {
        num = iVar4;
      }
      if (vm_functionBC == num) {
        iVar4 = vm_loadDefines(&buf);
        if (iVar4 == 0) {
          return 0;
        }
        pProgram_00 = vm_compilingProgram();
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
        if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
          return 0;
        }
        iVar4 = *(int *)buf.p;
        buf.p = buf.p + 4;
        buf.len = buf.len + -4;
        if (&stack0x00000000 != (undefined1 *)0xfd4) {
          newCount = iVar4;
        }
        if (newCount < 0) {
          return 0;
        }
        binaryStrPoolSize = newCount;
        if (pProgram_00->maxStringNo == 0) {
          pStrOffsetMapping = (int *)0x0;
        }
        else {
          pStrOffsetMapping = vm_scratchAlloc(newCount * 4);
        }
        if (pProgram_00->stringBufSize < newCount) {
                    /* Unresolved local var: Vm_String_T * * ppNewStrings@[DW_OP_reg0(EAX)]
                       Unresolved local var: int newSize@[???] */
          if (pProgram_00->ppStrings == (Vm_String **)0x0) {
            ppVVar2 = pdb_allocateMemory(newCount * 4,"VM",
                                         "/home/cc/xxxx/gameshell/src/vm/vm_binary.c",0x2f1);
          }
          else {
            ppVVar2 = pdb_reallocateMemory
                                (pProgram_00->ppStrings,newCount * 4,"VM",
                                 "/home/cc/xxxx/gameshell/src/vm/vm_binary.c",0x2f5);
          }
          if (ppVVar2 == (Vm_String **)0x0) {
            return 0;
          }
          pProgram_00->ppStrings = ppVVar2;
          pProgram_00->stringBufSize = (Vm_Index_T)newCount;
        }
        i = 0;
        bVar6 = SBORROW4(0,newCount);
        iVar4 = -newCount;
        do {
          if (bVar6 == iVar4 < 0) {
            iVar4 = vm_loadIdentifiers(&buf);
            if (iVar4 == 0) {
              return 0;
            }
            iVar4 = vm_loadCodeSegment(&buf,(Vm_Fun_T *)0x0,pCodeSegment);
            if (iVar4 != 0) {
              return (uint)(pCodeSegment->nodes[pCodeSegment->nodesCount + -1].actionNo == '\x1d');
            }
            return 0;
          }
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
          if (((uint)buf.len < 4) && (vm_fetchBinaryBufferData(&buf), (uint)buf.len < 4)) {
            return 0;
          }
          iVar4 = *(int *)buf.p;
          buf.p = buf.p + 4;
          buf.len = buf.len + -4;
          if (&stack0x00000000 != (undefined1 *)0xfdc) {
            size = iVar4;
          }
          if (size < 1) {
            str = (char *)0x0;
          }
          else {
                    /* Unresolved local var: int newStrNo@[DW_OP_reg3(EBX)] */
            __dest = vm_scratchAlloc(size);
            if ((uint)size < 0xfa1) {
              if ((buf.len < size) && (vm_fetchBinaryBufferData(&buf), buf.len < size)) {
LAB_080b9619:
                vm_scratchFree(__dest);
                return 0;
              }
              memcpy(__dest,buf.p,size);
              buf.p = buf.p + size;
              buf.len = buf.len - size;
            }
            else {
              memcpy(__dest,buf.p,buf.len);
              pBuffer = __dest + buf.len;
              len = size - buf.len;
              buf.p = buf.data;
              buf.len = 0;
              uVar3 = vm_read(pBuffer,len,buf.fd);
              if (uVar3 != len) goto LAB_080b9619;
            }
            if (&stack0x00000000 == (undefined1 *)0xfe0) {
              vm_scratchFree(__dest);
              __dest = str_1;
            }
            str_1 = __dest;
            str_1[size + -1] = '\0';
            if (&stack0x00000000 == (undefined1 *)0xfd8) {
              vm_scratchFree(str_1);
            }
            else {
              str = str_1;
            }
          }
          if (str == (char *)0x0) {
            return 0;
          }
          VVar1 = vm_findOrAddProgramString(pProgram_00,str);
          vm_scratchFree(str);
          if (pStrOffsetMapping != (int *)0x0) {
            pStrOffsetMapping[i] = (int)VVar1;
          }
          i = i + 1;
          bVar6 = SBORROW4(i,newCount);
          iVar4 = i - newCount;
        } while( true );
      }
    }
  }
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x80b9264: PUSH EBP
0x80b9265: MOV EBP, ESP
0x80b9267: PUSH EDI
0x80b9268: PUSH ESI
0x80b9269: PUSH EBX
0x80b926a: SUB ESP, 0xff8
0x80b9270: XOR EAX, EAX
0x80b9272: CLD
0x80b9273: MOV EDI, dword ptr [EBP + 0xc]
0x80b9276: MOV ECX, 0x8
0x80b927b: STOSD.REP ES:EDI
0x80b927d: MOV EAX, dword ptr [EBP + 0x8]
0x80b9280: LEA EDI, [EBP + 0xfffff038]
0x80b9286: MOV dword ptr [EBP + 0xfffff038], EAX
0x80b928c: PUSH EDI
0x80b928d: LEA EAX, [EBP + 0xfffff044]
0x80b9293: MOV dword ptr [EBP + 0xfffff040], 0x0
0x80b929d: MOV dword ptr [EBP + 0xfffff03c], EAX
0x80b92a3: CALL 0x080b88a0
0x80b92a8: ADD ESP, 0x10
0x80b92ab: XOR ECX, ECX
0x80b92ad: TEST EAX, EAX
0x80b92af: JZ 0x080b92ed
0x80b92b1: CMP dword ptr [EBP + 0xfffff040], 0x3
0x80b92b8: MOV dword ptr [EBP + 0xfffff020], 0x0
0x80b92c2: LEA EBX, [EBP + 0xfffff034]
0x80b92c8: JBE 0x080b92f8
0x80b92ca: MOV EAX, dword ptr [EDI + 0x4]
0x80b92cd: MOV EDX, dword ptr [EAX]
0x80b92cf: ADD EAX, 0x4
0x80b92d2: SUB dword ptr [EDI + 0x8], 0x4
0x80b92d6: TEST EBX, EBX
0x80b92d8: MOV dword ptr [EDI + 0x4], EAX
0x80b92db: JZ 0x080b92df
0x80b92dd: MOV dword ptr [EBX], EDX
0x80b92df: CMP dword ptr [EBP + 0xfffff034], 0x10001
0x80b92e9: JZ 0x080b9314
0x80b92eb: XOR ECX, ECX
0x80b92ed: LEA ESP, [EBP + -0xc]
0x80b92f0: POP EBX
0x80b92f1: POP ESI
0x80b92f2: MOV EAX, ECX
0x80b92f4: POP EDI
0x80b92f5: LEAVE
0x80b92f6: RET
0x80b92f8: SUB ESP, 0xc
0x80b92fb: PUSH EDI
0x80b92fc: CALL 0x080b88a0
0x80b9301: ADD ESP, 0x10
0x80b9304: CMP dword ptr [EBP + 0xfffff040], 0x3
0x80b930b: JA 0x080b92ca
0x80b930d: XOR ECX, ECX
0x80b930f: JMP 0x080b92ed
0x80b9314: MOV EDX, dword ptr [EDI + 0x8]
0x80b9317: CMP EDX, 0x3
0x80b931a: JBE 0x080b9573
0x80b9320: LEA EAX, [EDX + -0x4]
0x80b9323: ADD dword ptr [EDI + 0x4], 0x4
0x80b9327: CMP EAX, 0x3
0x80b932a: MOV dword ptr [EDI + 0x8], EAX
0x80b932d: MOV EDX, EAX
0x80b932f: JBE 0x080b9592
0x80b9335: MOV EAX, dword ptr [EDI + 0x4]
0x80b9338: MOV ECX, dword ptr [EAX]
0x80b933a: ADD EAX, 0x4
0x80b933d: MOV dword ptr [EDI + 0x4], EAX
0x80b9340: TEST EBX, EBX
0x80b9342: LEA EAX, [EDX + -0x4]
0x80b9345: MOV dword ptr [EDI + 0x8], EAX
0x80b9348: JZ 0x080b934c
0x80b934a: MOV dword ptr [EBX], ECX
0x80b934c: MOVSX EAX, word ptr [0x08660900]
0x80b9353: CMP EAX, dword ptr [EBP + 0xfffff034]
0x80b9359: JNZ 0x080b92eb
0x80b935b: SUB ESP, 0xc
0x80b935e: PUSH EDI
0x80b935f: CALL 0x08087080
0x80b9364: ADD ESP, 0x10
0x80b9367: XOR ECX, ECX
0x80b9369: TEST EAX, EAX
0x80b936b: JZ 0x080b92ed
0x80b936d: CALL 0x0807ecfc
0x80b9372: MOV EDX, dword ptr [EDI + 0x8]
0x80b9375: CMP EDX, 0x3
0x80b9378: MOV dword ptr [EBP + 0xfffff01c], EAX
0x80b937e: LEA EBX, [EBP + 0xfffff030]
0x80b9384: JBE 0x080b9677
0x80b938a: MOV EAX, dword ptr [EDI + 0x4]
0x80b938d: MOV ECX, dword ptr [EAX]
0x80b938f: ADD EAX, 0x4
0x80b9392: MOV dword ptr [EDI + 0x4], EAX
0x80b9395: TEST EBX, EBX
0x80b9397: LEA EAX, [EDX + -0x4]
0x80b939a: MOV dword ptr [EDI + 0x8], EAX
0x80b939d: JZ 0x080b93a1
0x80b939f: MOV dword ptr [EBX], ECX
0x80b93a1: MOV EDX, dword ptr [EBP + 0xfffff030]
0x80b93a7: TEST EDX, EDX
0x80b93a9: JS 0x080b95c9
0x80b93af: MOV EAX, dword ptr [EBP + 0xfffff01c]
0x80b93b5: CMP word ptr [EAX + 0x24], 0x0
0x80b93ba: MOV dword ptr [0x08609d20], EDX
0x80b93c0: JNZ 0x080b9694
0x80b93c6: MOV dword ptr [0x08609ca0], 0x0
0x80b93d0: MOV ECX, dword ptr [EBP + 0xfffff01c]
0x80b93d6: MOVSX EAX, word ptr [ECX + 0x26]
0x80b93da: CMP EAX, EDX
0x80b93dc: JGE 0x080b9429
0x80b93de: LEA EAX, [EDX*0x4 + 0x0]
0x80b93e5: MOV EDX, dword ptr [ECX + 0x20]
0x80b93e8: TEST EDX, EDX
0x80b93ea: JNZ 0x080b96e3
0x80b93f0: PUSH 0x2f1
0x80b93f5: PUSH 0x82b16e4
0x80b93fa: PUSH 0x82a376b
0x80b93ff: PUSH EAX
0x80b9400: CALL 0x080ce784
0x80b9405: ADD ESP, 0x10
0x80b9408: TEST EAX, EAX
0x80b940a: JZ 0x080b95c9
0x80b9410: MOV EDX, dword ptr [EBP + 0xfffff01c]
0x80b9416: MOV dword ptr [EDX + 0x20], EAX
0x80b9419: MOV ECX, dword ptr [EBP + 0xfffff01c]
0x80b941f: MOV EDX, dword ptr [EBP + 0xfffff030]
0x80b9425: MOV word ptr [ECX + 0x26], DX
0x80b9429: MOV dword ptr [EBP + 0xfffff018], 0x0
0x80b9433: CMP dword ptr [EBP + 0xfffff018], EDX
0x80b9439: JMP 0x080b9527
0x80b943e: SUB ESP, 0xc
0x80b9441: PUSH EBX
0x80b9442: CALL 0x08089390
0x80b9447: ADD ESP, 0x10
0x80b944a: CMP EBX, 0xfa0
0x80b9450: MOV dword ptr [EBP + 0xfffff014], EAX
0x80b9456: JA 0x080b95d4
0x80b945c: CMP dword ptr [EDI + 0x8], EBX
0x80b945f: JGE 0x080b9476
0x80b9461: SUB ESP, 0xc
0x80b9464: PUSH EDI
0x80b9465: CALL 0x080b88a0
0x80b946a: ADD ESP, 0x10
0x80b946d: CMP dword ptr [EDI + 0x8], EBX
0x80b9470: JL 0x080b9619
0x80b9476: PUSH EAX
0x80b9477: PUSH EBX
0x80b9478: PUSH dword ptr [EDI + 0x4]
0x80b947b: PUSH dword ptr [EBP + 0xfffff014]
0x80b9481: CALL 0x0804b7e4
0x80b9486: ADD ESP, 0x10
0x80b9489: ADD dword ptr [EDI + 0x4], EBX
0x80b948c: SUB dword ptr [EDI + 0x8], EBX
0x80b948f: LEA EDX, [EBP + 0xfffff024]
0x80b9495: TEST EDX, EDX
0x80b9497: JZ 0x080b96cd
0x80b949d: MOV ECX, dword ptr [EBP + 0xfffff014]
0x80b94a3: MOV dword ptr [EBP + 0xfffff024], ECX
0x80b94a9: MOV EAX, dword ptr [EBP + 0xfffff028]
0x80b94af: MOV EDX, dword ptr [EBP + 0xfffff024]
0x80b94b5: MOV byte ptr [EDX + EAX*0x1 + -0x1], 0x0
0x80b94ba: LEA EAX, [EBP + 0xfffff02c]
0x80b94c0: TEST EAX, EAX
0x80b94c2: JZ 0x080b96b7
0x80b94c8: MOV EAX, dword ptr [EBP + 0xfffff024]
0x80b94ce: MOV dword ptr [EBP + 0xfffff02c], EAX
0x80b94d4: MOV EAX, dword ptr [EBP + 0xfffff02c]
0x80b94da: TEST EAX, EAX
0x80b94dc: JZ 0x080b95c9
0x80b94e2: SUB ESP, 0x8
0x80b94e5: PUSH EAX
0x80b94e6: PUSH dword ptr [EBP + 0xfffff01c]
0x80b94ec: CALL 0x080b1258
0x80b94f1: POP EBX
0x80b94f2: PUSH dword ptr [EBP + 0xfffff02c]
0x80b94f8: MOVSX EBX, AX
0x80b94fb: CALL 0x080891d4
0x80b9500: MOV EAX, [0x08609ca0]
0x80b9505: ADD ESP, 0x10
0x80b9508: TEST EAX, EAX
0x80b950a: JZ 0x080b9515
0x80b950c: MOV EDX, dword ptr [EBP + 0xfffff018]
0x80b9512: MOV dword ptr [EAX + EDX*0x4], EBX
0x80b9515: INC dword ptr [EBP + 0xfffff018]
0x80b951b: MOV ECX, dword ptr [EBP + 0xfffff018]
0x80b9521: CMP ECX, dword ptr [EBP + 0xfffff030]
0x80b9527: JGE 0x080b962c
0x80b952d: MOV EDX, dword ptr [EDI + 0x8]
0x80b9530: CMP EDX, 0x3
0x80b9533: JBE 0x080b95b1
0x80b9535: MOV EAX, dword ptr [EDI + 0x4]
0x80b9538: MOV ECX, dword ptr [EAX]
0x80b953a: ADD EAX, 0x4
0x80b953d: MOV dword ptr [EDI + 0x4], EAX
0x80b9540: LEA EAX, [EDX + -0x4]
0x80b9543: MOV dword ptr [EDI + 0x8], EAX
0x80b9546: LEA EAX, [EBP + 0xfffff028]
0x80b954c: TEST EAX, EAX
0x80b954e: JZ 0x080b9556
0x80b9550: MOV dword ptr [EBP + 0xfffff028], ECX
0x80b9556: MOV EBX, dword ptr [EBP + 0xfffff028]
0x80b955c: TEST EBX, EBX
0x80b955e: JG 0x080b943e
0x80b9564: MOV dword ptr [EBP + 0xfffff02c], 0x0
0x80b956e: JMP 0x080b94d4
0x80b9573: SUB ESP, 0xc
0x80b9576: PUSH EDI
0x80b9577: CALL 0x080b88a0
0x80b957c: MOV EDX, dword ptr [EDI + 0x8]
0x80b957f: ADD ESP, 0x10
0x80b9582: XOR ECX, ECX
0x80b9584: CMP EDX, 0x3
0x80b9587: JBE 0x080b92ed
0x80b958d: JMP 0x080b9320
0x80b9592: SUB ESP, 0xc
0x80b9595: PUSH EDI
0x80b9596: CALL 0x080b88a0
0x80b959b: MOV EDX, dword ptr [EDI + 0x8]
0x80b959e: ADD ESP, 0x10
0x80b95a1: CMP EDX, 0x3
0x80b95a4: JA 0x080b9335
0x80b95aa: XOR ECX, ECX
0x80b95ac: JMP 0x080b92ed
0x80b95b1: SUB ESP, 0xc
0x80b95b4: PUSH EDI
0x80b95b5: CALL 0x080b88a0
0x80b95ba: MOV EDX, dword ptr [EDI + 0x8]
0x80b95bd: ADD ESP, 0x10
0x80b95c0: CMP EDX, 0x3
0x80b95c3: JA 0x080b9535
0x80b95c9: MOV ECX, dword ptr [EBP + 0xfffff020]
0x80b95cf: JMP 0x080b92ed
0x80b95d4: PUSH ESI
0x80b95d5: PUSH dword ptr [EDI + 0x8]
0x80b95d8: PUSH dword ptr [EDI + 0x4]
0x80b95db: PUSH dword ptr [EBP + 0xfffff014]
0x80b95e1: CALL 0x0804b7e4
0x80b95e6: MOV ESI, EBX
0x80b95e8: MOV EAX, dword ptr [EDI + 0x8]
0x80b95eb: MOV EDX, dword ptr [EBP + 0xfffff014]
0x80b95f1: ADD ESP, 0xc
0x80b95f4: ADD EDX, EAX
0x80b95f6: SUB ESI, EAX
0x80b95f8: LEA EAX, [EDI + 0xc]
0x80b95fb: MOV dword ptr [EDI + 0x4], EAX
0x80b95fe: MOV dword ptr [EDI + 0x8], 0x0
0x80b9605: PUSH dword ptr [EDI]
0x80b9607: PUSH ESI
0x80b9608: PUSH EDX
0x80b9609: CALL 0x080a110c
0x80b960e: ADD ESP, 0x10
0x80b9611: CMP EAX, ESI
0x80b9613: JZ 0x080b948f
0x80b9619: SUB ESP, 0xc
0x80b961c: PUSH dword ptr [EBP + 0xfffff014]
0x80b9622: CALL 0x080891d4
0x80b9627: ADD ESP, 0x10
0x80b962a: JMP 0x080b95c9
0x80b962c: SUB ESP, 0xc
0x80b962f: PUSH EDI
0x80b9630: CALL 0x08087fb8
0x80b9635: ADD ESP, 0x10
0x80b9638: TEST EAX, EAX
0x80b963a: JZ 0x080b95c9
0x80b963c: PUSH ECX
0x80b963d: PUSH dword ptr [EBP + 0xc]
0x80b9640: PUSH 0x0
0x80b9642: PUSH EDI
0x80b9643: CALL 0x08096128
0x80b9648: ADD ESP, 0x10
0x80b964b: TEST EAX, EAX
0x80b964d: JZ 0x080b95c9
0x80b9653: MOV EDX, dword ptr [EBP + 0xc]
0x80b9656: MOV EAX, dword ptr [EDX]
0x80b9658: MOV EDX, dword ptr [EDX + 0x4]
0x80b965b: CMP byte ptr [EDX + EAX*0x4 + -0x1], 0x1d
0x80b9660: MOV EAX, 0x1
0x80b9665: CMOVNZ EAX, dword ptr [EBP + 0xfffff020]
0x80b966c: MOV dword ptr [EBP + 0xfffff020], EAX
0x80b9672: JMP 0x080b95c9
0x80b9677: SUB ESP, 0xc
0x80b967a: PUSH EDI
0x80b967b: CALL 0x080b88a0
0x80b9680: MOV EDX, dword ptr [EDI + 0x8]
0x80b9683: ADD ESP, 0x10
0x80b9686: CMP EDX, 0x3
0x80b9689: JBE 0x080b95c9
0x80b968f: JMP 0x080b938a
0x80b9694: SUB ESP, 0xc
0x80b9697: LEA EAX, [EDX*0x4 + 0x0]
0x80b969e: PUSH EAX
0x80b969f: CALL 0x08089390
0x80b96a4: ADD ESP, 0x10
0x80b96a7: MOV EDX, dword ptr [EBP + 0xfffff030]
0x80b96ad: MOV [0x08609ca0], EAX
0x80b96b2: JMP 0x080b93d0
0x80b96b7: SUB ESP, 0xc
0x80b96ba: PUSH dword ptr [EBP + 0xfffff024]
0x80b96c0: CALL 0x080891d4
0x80b96c5: ADD ESP, 0x10
0x80b96c8: JMP 0x080b94d4
0x80b96cd: SUB ESP, 0xc
0x80b96d0: PUSH dword ptr [EBP + 0xfffff014]
0x80b96d6: CALL 0x080891d4
0x80b96db: ADD ESP, 0x10
0x80b96de: JMP 0x080b94a9
0x80b96e3: SUB ESP, 0xc
0x80b96e6: PUSH 0x2f5
0x80b96eb: PUSH 0x82b16e4
0x80b96f0: PUSH 0x82a376b
0x80b96f5: PUSH EAX
0x80b96f6: MOV EAX, dword ptr [EBP + 0xfffff01c]
0x80b96fc: PUSH dword ptr [EAX + 0x20]
0x80b96ff: CALL 0x080cf588
0x80b9704: ADD ESP, 0x20
0x80b9707: JMP 0x080b9408
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadTempString"></a>

## vm_loadTempString @ 0x80b8c4c

Evidence: `ev_eb3f82761c65f9f0fdd6fb57ee9f5317ef2192a291ea73c6304d9eabf8ded020`

```c

int vm_loadTempString(Vm_Binary_Buffer_T *pInput,char **ppStr)

{
  int iVar1;
  char *__dest;
  uint uVar2;
  uint len;
  void *buf;
  char *str;
  int size;
  
                    /* Unresolved local var: int temp@[DW_OP_reg1(ECX)] */
  uVar2 = pInput->len;
  if (uVar2 < 4) {
    vm_fetchBinaryBufferData(pInput);
    uVar2 = pInput->len;
    if (uVar2 < 4) {
      return 0;
    }
  }
  iVar1 = *(int *)pInput->p;
  pInput->p = (char *)((int)pInput->p + 4);
  pInput->len = uVar2 - 4;
  if (&stack0x00000000 != (undefined1 *)0x14) {
    size = iVar1;
  }
  if (size < 1) {
    *ppStr = (char *)0x0;
  }
  else {
    __dest = vm_scratchAlloc(size);
    if ((uint)size < 0xfa1) {
      if ((pInput->len < size) && (vm_fetchBinaryBufferData(pInput), pInput->len < size)) {
LAB_080b8d45:
        vm_scratchFree(__dest);
        return 0;
      }
      memcpy(__dest,pInput->p,size);
      pInput->p = pInput->p + size;
      pInput->len = pInput->len - size;
    }
    else {
      memcpy(__dest,pInput->p,pInput->len);
      iVar1 = pInput->len;
      len = size - iVar1;
      pInput->p = pInput->data;
      pInput->len = 0;
      uVar2 = vm_read(__dest + iVar1,len,pInput->fd);
      if (uVar2 != len) goto LAB_080b8d45;
    }
    if (&stack0x00000000 == (undefined1 *)0x18) {
      vm_scratchFree(__dest);
      __dest = str;
    }
    str = __dest;
    str[size + -1] = '\0';
    if (ppStr == (char **)0x0) {
      vm_scratchFree(str);
    }
    else {
      *ppStr = str;
    }
  }
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x80b8c4c: PUSH EBP
0x80b8c4d: MOV EBP, ESP
0x80b8c4f: PUSH EDI
0x80b8c50: PUSH ESI
0x80b8c51: PUSH EBX
0x80b8c52: SUB ESP, 0xc
0x80b8c55: MOV ESI, dword ptr [EBP + 0x8]
0x80b8c58: MOV EAX, dword ptr [ESI + 0x8]
0x80b8c5b: CMP EAX, 0x3
0x80b8c5e: LEA EBX, [EBP + -0x10]
0x80b8c61: MOV EDX, EAX
0x80b8c63: JBE 0x080b8d0c
0x80b8c69: MOV EAX, dword ptr [ESI + 0x4]
0x80b8c6c: MOV ECX, dword ptr [EAX]
0x80b8c6e: ADD EAX, 0x4
0x80b8c71: MOV dword ptr [ESI + 0x4], EAX
0x80b8c74: TEST EBX, EBX
0x80b8c76: LEA EAX, [EDX + -0x4]
0x80b8c79: MOV dword ptr [ESI + 0x8], EAX
0x80b8c7c: JZ 0x080b8c80
0x80b8c7e: MOV dword ptr [EBX], ECX
0x80b8c80: MOV EBX, dword ptr [EBP + -0x10]
0x80b8c83: TEST EBX, EBX
0x80b8c85: JLE 0x080b8d00
0x80b8c87: SUB ESP, 0xc
0x80b8c8a: PUSH EBX
0x80b8c8b: CALL 0x08089390
0x80b8c90: ADD ESP, 0x10
0x80b8c93: CMP EBX, 0xfa0
0x80b8c99: MOV dword ptr [EBP + -0x18], EAX
0x80b8c9c: JA 0x080b8d60
0x80b8ca2: CMP dword ptr [ESI + 0x8], EBX
0x80b8ca5: JL 0x080b8d30
0x80b8cab: PUSH EAX
0x80b8cac: PUSH EBX
0x80b8cad: PUSH dword ptr [ESI + 0x4]
0x80b8cb0: PUSH dword ptr [EBP + -0x18]
0x80b8cb3: CALL 0x0804b7e4
0x80b8cb8: ADD ESP, 0x10
0x80b8cbb: ADD dword ptr [ESI + 0x4], EBX
0x80b8cbe: SUB dword ptr [ESI + 0x8], EBX
0x80b8cc1: LEA EDX, [EBP + -0x14]
0x80b8cc4: TEST EDX, EDX
0x80b8cc6: JZ 0x080b8da0
0x80b8ccc: MOV EAX, dword ptr [EBP + -0x18]
0x80b8ccf: MOV dword ptr [EBP + -0x14], EAX
0x80b8cd2: MOV EAX, dword ptr [EBP + -0x10]
0x80b8cd5: MOV EDX, dword ptr [EBP + -0x14]
0x80b8cd8: MOV byte ptr [EDX + EAX*0x1 + -0x1], 0x0
0x80b8cdd: MOV EAX, dword ptr [EBP + 0xc]
0x80b8ce0: TEST EAX, EAX
0x80b8ce2: JZ 0x080b8db3
0x80b8ce8: MOV EAX, dword ptr [EBP + -0x14]
0x80b8ceb: MOV EDX, dword ptr [EBP + 0xc]
0x80b8cee: MOV dword ptr [EDX], EAX
0x80b8cf0: LEA ESP, [EBP + -0xc]
0x80b8cf3: POP EBX
0x80b8cf4: POP ESI
0x80b8cf5: MOV EAX, 0x1
0x80b8cfa: POP EDI
0x80b8cfb: LEAVE
0x80b8cfc: RET
0x80b8d00: MOV EAX, dword ptr [EBP + 0xc]
0x80b8d03: MOV dword ptr [EAX], 0x0
0x80b8d09: JMP 0x080b8cf0
0x80b8d0c: SUB ESP, 0xc
0x80b8d0f: PUSH ESI
0x80b8d10: CALL 0x080b88a0
0x80b8d15: MOV EDX, dword ptr [ESI + 0x8]
0x80b8d18: ADD ESP, 0x10
0x80b8d1b: XOR EAX, EAX
0x80b8d1d: CMP EDX, 0x3
0x80b8d20: JA 0x080b8c69
0x80b8d26: LEA ESP, [EBP + -0xc]
0x80b8d29: POP EBX
0x80b8d2a: POP ESI
0x80b8d2b: POP EDI
0x80b8d2c: LEAVE
0x80b8d2d: RET
0x80b8d30: SUB ESP, 0xc
0x80b8d33: PUSH ESI
0x80b8d34: CALL 0x080b88a0
0x80b8d39: ADD ESP, 0x10
0x80b8d3c: CMP dword ptr [ESI + 0x8], EBX
0x80b8d3f: JGE 0x080b8cab
0x80b8d45: SUB ESP, 0xc
0x80b8d48: PUSH dword ptr [EBP + -0x18]
0x80b8d4b: CALL 0x080891d4
0x80b8d50: ADD ESP, 0x10
0x80b8d53: LEA ESP, [EBP + -0xc]
0x80b8d56: POP EBX
0x80b8d57: POP ESI
0x80b8d58: XOR EAX, EAX
0x80b8d5a: POP EDI
0x80b8d5b: LEAVE
0x80b8d5c: RET
0x80b8d60: PUSH EAX
0x80b8d61: PUSH dword ptr [ESI + 0x8]
0x80b8d64: PUSH dword ptr [ESI + 0x4]
0x80b8d67: PUSH dword ptr [EBP + -0x18]
0x80b8d6a: CALL 0x0804b7e4
0x80b8d6f: MOV EDI, EBX
0x80b8d71: MOV EAX, dword ptr [ESI + 0x8]
0x80b8d74: MOV EDX, dword ptr [EBP + -0x18]
0x80b8d77: ADD ESP, 0xc
0x80b8d7a: ADD EDX, EAX
0x80b8d7c: SUB EDI, EAX
0x80b8d7e: LEA EAX, [ESI + 0xc]
0x80b8d81: MOV dword ptr [ESI + 0x4], EAX
0x80b8d84: MOV dword ptr [ESI + 0x8], 0x0
0x80b8d8b: PUSH dword ptr [ESI]
0x80b8d8d: PUSH EDI
0x80b8d8e: PUSH EDX
0x80b8d8f: CALL 0x080a110c
0x80b8d94: ADD ESP, 0x10
0x80b8d97: CMP EAX, EDI
0x80b8d99: JNZ 0x080b8d45
0x80b8d9b: JMP 0x080b8cc1
0x80b8da0: SUB ESP, 0xc
0x80b8da3: PUSH dword ptr [EBP + -0x18]
0x80b8da6: CALL 0x080891d4
0x80b8dab: ADD ESP, 0x10
0x80b8dae: JMP 0x080b8cd2
0x80b8db3: SUB ESP, 0xc
0x80b8db6: PUSH dword ptr [EBP + -0x14]
0x80b8db9: CALL 0x080891d4
0x80b8dbe: ADD ESP, 0x10
0x80b8dc1: JMP 0x080b8cf0
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_loadCodeSegment"></a>

## vm_loadCodeSegment @ 0x8096128

Evidence: `ev_edcc9ae3360df89029b6c17c970dcbe35359f22edebb0f04fe7b3757b2f81681`

```c

int vm_loadCodeSegment(Vm_Binary_Buffer_T *pInput,Vm_Fun_T *pFun,Vm_Code_Segment_T *pCodeSegment)

{
  Vm_Symbol_T *pVVar1;
  int iVar2;
  int iVar3;
  Vm_Action_Info_T *pVVar4;
  int iVar5;
  int extraout_ECX;
  Vm_Runtime_Misc_T *pVVar6;
  int extraout_EDX;
  char *pcVar7;
  Vm_Node_T *pVVar8;
  Vm_Code_Segment_T *pVVar9;
  Vm_Symbol_T *pVVar10;
  bool bVar11;
  bool bVar12;
  Vm_Node_T **ppBuf;
  int *num;
  uchar **ppBuf_00;
  Vm_Runtime_Misc_T **num_00;
  int iVar13;
  int metaAddress;
  int i;
  Vm_Symbol_T *pAllSymbols;
  int runtimeMiscCount;
  int unused;
  int nowLine;
  char *fileName;
  Vm_Symbol_Flag_T symbolFlag;
  int symbolsCount;
  
  pVVar9 = pCodeSegment;
                    /* Unresolved local var: Vm_Node_T * pNode@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Runtime_Misc_T * pRuntime@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Symbol_T * pSymbol@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Action_Info_T * pActionInfo@[DW_OP_reg2(EDX)]
                       Unresolved local var: char * nowFile@[DW_OP_reg6(ESI)] */
  for (iVar5 = 8; iVar5 != 0; iVar5 = iVar5 + -1) {
    pVVar9->nodesCount = 0;
    pVVar9 = (Vm_Code_Segment_T *)&pVVar9->nodes;
  }
  iVar5 = vm_loadNumber32(pInput,&symbolsCount);
  if ((iVar5 == 0) || (symbolsCount < 0)) {
    vm_errprintf("Load actions failed: read symbol table error.\n");
    return 0;
  }
  if (symbolsCount != 0) {
    pVVar1 = vm_scratchLargeAlloc(symbolsCount << 3);
    pVVar10 = pVVar1;
    for (iVar5 = (symbolsCount & 0x1fffffffU) << 1; iVar5 != 0; iVar5 = iVar5 + -1) {
      pVVar10->file = (char *)0x0;
      pVVar10 = (Vm_Symbol_T *)&pVVar10->line;
    }
    pcVar7 = (char *)0x0;
    iVar5 = -symbolsCount;
    nowLine = 0;
    i = 0;
    bVar11 = SBORROW4(0,symbolsCount);
    pVVar10 = pVVar1;
    while (bVar11 != iVar5 < 0) {
      iVar5 = vm_loadNumber8(pInput,&symbolFlag);
      if (iVar5 == 0) {
        vm_errprintf("Load actions failed: read symbol flag error.\n");
        return 0;
      }
      if ((symbolFlag & 1) != 0) {
        iVar5 = vm_loadTempString(pInput,&fileName);
        if (iVar5 == 0) {
          vm_errprintf("Load actions failed: read new-file-name error.\n");
          return 0;
        }
        pcVar7 = vm_addFileName(fileName,(Vm_String_T **)0x0);
        vm_scratchFree(fileName);
        if (pcVar7 == (char *)0x0) {
          vm_errprintf("Load actions failed: add new-file-name error.\n");
          return 0;
        }
      }
      if (((symbolFlag & 2) != 0) && (iVar5 = vm_loadNumber32(pInput,&nowLine), iVar5 == 0)) {
        vm_errprintf("Load actions failed: read new-line-no error.\n");
        return 0;
      }
      if (pcVar7 == (char *)0x0) {
        vm_errprintf("Load actions failed: not got file-name.\n");
        return 0;
      }
      pVVar10->line = nowLine;
      i = i + 1;
      pVVar10->file = pcVar7;
      pVVar10 = pVVar10 + 1;
      iVar5 = i - symbolsCount;
      bVar11 = SBORROW4(i,symbolsCount);
    }
    pCodeSegment->symbolsCount = symbolsCount;
    pCodeSegment->symbols = pVVar1;
  }
  iVar5 = vm_loadNumber32ToIntR(pInput,&pCodeSegment->nodesCount);
  if (iVar5 == 0) {
LAB_08096310:
    vm_errprintf("Load actions failed: read nodes count error.\n");
    return 0;
  }
  iVar5 = pCodeSegment->nodesCount;
  if (iVar5 < 0) goto LAB_08096310;
  if (iVar5 == 0) {
    iVar5 = vm_loadNumber32(pInput,&unused);
    if ((iVar5 == 0) || (unused != 0)) {
      vm_errprintf("Load actions failed: non-empty meta data when actions is empty.\n");
      return 0;
    }
    iVar5 = vm_loadNumber32(pInput,&unused);
    if ((iVar5 != 0) && (unused == 0)) {
      return 1;
    }
    vm_errprintf(
                "Load actions failed: non-empty runtime miscellaneous information when actions is empty.\n"
                );
    return 0;
  }
  iVar5 = iVar5 << 2;
  ppBuf = &pCodeSegment->nodes;
  iVar13 = extraout_ECX;
  iVar2 = vm_loadBuffer(pInput,ppBuf,iVar5);
  if (iVar2 == 0) {
    vm_errprintf("Load actions failed: read nodes error.\n",ppBuf,iVar5,iVar13);
    return 0;
  }
  num = &pCodeSegment->metaDataLen;
  iVar2 = vm_loadNumber32ToIntR(pInput,num);
  if ((iVar2 == 0) || (iVar2 = pCodeSegment->metaDataLen, iVar2 < 0)) {
    vm_errprintf("Load actions: read meta data length (%d) error.\n",num,iVar5,iVar13);
    return 0;
  }
  if (0 < iVar2) {
    ppBuf_00 = &pCodeSegment->metaData;
    iVar13 = extraout_EDX;
    iVar3 = vm_loadBuffer(pInput,ppBuf_00,iVar2);
    iVar5 = iVar2;
    if (iVar3 == 0) {
      vm_errprintf("Load actions: read meta data error.\n",ppBuf_00,iVar2,iVar13);
      return 0;
    }
  }
  num_00 = (Vm_Runtime_Misc_T **)&runtimeMiscCount;
  iVar2 = vm_loadNumber32(pInput,(int *)num_00);
  if (iVar2 == 0) {
LAB_08096385:
    vm_errprintf("Load actions: read runtime miscellaneous count error.\n",num_00,iVar5,iVar13);
    return 0;
  }
  if (runtimeMiscCount != 0) {
    iVar2 = pCodeSegment->nodesCount;
    if (iVar2 != runtimeMiscCount) goto LAB_08096385;
    iVar3 = 0;
    if (iVar2 == 0) goto LAB_08096404;
    iVar5 = iVar2 * 4;
    num_00 = &pCodeSegment->runtimeMisc;
    iVar2 = vm_loadBuffer(pInput,num_00,iVar5);
    iVar13 = runtimeMiscCount;
    if (iVar2 == 0) {
      vm_errprintf("Load actions: read runtime information error.\n",num_00,iVar5,runtimeMiscCount);
      return 0;
    }
  }
  iVar3 = pCodeSegment->nodesCount;
LAB_08096404:
  pVVar8 = pCodeSegment->nodes;
  bVar12 = false;
  bVar11 = iVar3 == 0;
  i = 0;
  iVar2 = iVar3;
  while( true ) {
    if (bVar11 || bVar12 != iVar2 < 0) {
      pVVar6 = pCodeSegment->runtimeMisc;
      if (((pVVar6 != (Vm_Runtime_Misc_T *)0x0) && (pCodeSegment->symbols != (Vm_Symbol_T *)0x0)) &&
         (i = 0, 0 < iVar3)) {
        do {
          if (pCodeSegment->symbolsCount <= (int)pVVar6->symbolNo) {
            vm_errprintf("Load actions failed: got bad symbolNo.\n",num_00,iVar5,iVar13);
            return 0;
          }
          i = i + 1;
          pVVar6 = pVVar6 + 1;
        } while (i < iVar3);
      }
      return 1;
    }
                    /* Unresolved local var: int mappedStrNo@[DW_OP_reg0(EAX)] */
    pVVar4 = vm_getActionInfo((uint)pVVar8->actionNo);
    if (pVVar4 == (Vm_Action_Info_T *)0x0) {
      vm_errprintf("Load actions failed: got bad actionNo.\n");
      return 0;
    }
    if (((int)pVVar4->attrib < 0) &&
       (metaAddress = (int)pVVar8->par8 << 0x10 | (int)pVVar8->par16,
       pCodeSegment->metaDataLen < (int)(metaAddress + (uint)pVVar4->parameters * 4))) break;
    if (pVVar8->actionNo == '@') {
      iVar2 = vm_mapStrNoTo((int)pVVar8->par16);
      if (iVar2 < 0) {
        return 0;
      }
      pVVar8->par16 = (short)iVar2;
    }
    else if (pVVar8->actionNo == '\v') {
      iVar2 = vm_mapStrNoTo(*(int *)(pCodeSegment->metaData + metaAddress + 4));
      if (iVar2 < 0) {
        return 0;
      }
      *(int *)(pCodeSegment->metaData + metaAddress + 4) = (int)(short)iVar2;
    }
    i = i + 1;
    iVar3 = pCodeSegment->nodesCount;
    pVVar8 = pVVar8 + 1;
    bVar12 = SBORROW4(iVar3,i);
    iVar2 = iVar3 - i;
    bVar11 = iVar3 == i;
  }
  vm_errprintf("Load actions failed: got bad meta data address.");
  return 0;
}


```

<details><summary>汇编</summary>

```asm
0x8096128: PUSH EBP
0x8096129: MOV EBP, ESP
0x809612b: PUSH EDI
0x809612c: PUSH ESI
0x809612d: PUSH EBX
0x809612e: SUB ESP, 0x34
0x8096131: MOV EBX, dword ptr [EBP + 0x10]
0x8096134: XOR ESI, ESI
0x8096136: CLD
0x8096137: MOV EAX, ESI
0x8096139: MOV ECX, 0x8
0x809613e: MOV EDI, EBX
0x8096140: STOSD.REP ES:EDI
0x8096142: LEA EAX, [EBP + -0x10]
0x8096145: PUSH EAX
0x8096146: PUSH dword ptr [EBP + 0x8]
0x8096149: CALL 0x080b8bf4
0x809614e: ADD ESP, 0x10
0x8096151: TEST EAX, EAX
0x8096153: JZ 0x08096260
0x8096159: MOV EAX, dword ptr [EBP + -0x10]
0x809615c: TEST EAX, EAX
0x809615e: JL 0x08096260
0x8096164: JZ 0x080961fc
0x809616a: SUB ESP, 0xc
0x809616d: SHL EAX, 0x3
0x8096170: PUSH EAX
0x8096171: CALL 0x08089630
0x8096176: MOV ECX, dword ptr [EBP + -0x10]
0x8096179: SHL ECX, 0x3
0x809617c: MOV dword ptr [EBP + -0x28], EAX
0x809617f: SHR ECX, 0x2
0x8096182: MOV EDI, EAX
0x8096184: CLD
0x8096185: MOV EAX, ESI
0x8096187: STOSD.REP ES:EDI
0x8096189: MOV EAX, dword ptr [EBP + -0x10]
0x809618c: XOR ESI, ESI
0x809618e: ADD ESP, 0x10
0x8096191: MOV EDI, dword ptr [EBP + -0x28]
0x8096194: CMP ECX, EAX
0x8096196: MOV dword ptr [EBP + -0x1c], 0x0
0x809619d: MOV dword ptr [EBP + -0x2c], 0x0
0x80961a4: JMP 0x080961f1
0x80961a8: SUB ESP, 0x8
0x80961ab: LEA EDX, [EBP + -0x11]
0x80961ae: PUSH EDX
0x80961af: PUSH dword ptr [EBP + 0x8]
0x80961b2: CALL 0x080b8f9c
0x80961b7: ADD ESP, 0x10
0x80961ba: TEST EAX, EAX
0x80961bc: JZ 0x080963de
0x80961c2: MOV AL, byte ptr [EBP + -0x11]
0x80961c5: TEST AL, 0x1
0x80961c7: JNZ 0x08096270
0x80961cd: TEST AL, 0x2
0x80961cf: JNZ 0x080962b8
0x80961d5: TEST ESI, ESI
0x80961d7: JZ 0x080963f0
0x80961dd: MOV EAX, dword ptr [EBP + -0x1c]
0x80961e0: MOV dword ptr [EDI + 0x4], EAX
0x80961e3: INC dword ptr [EBP + -0x2c]
0x80961e6: MOV EAX, dword ptr [EBP + -0x10]
0x80961e9: MOV dword ptr [EDI], ESI
0x80961eb: ADD EDI, 0x8
0x80961ee: CMP dword ptr [EBP + -0x2c], EAX
0x80961f1: JL 0x080961a8
0x80961f3: MOV ECX, dword ptr [EBP + -0x28]
0x80961f6: MOV dword ptr [EBX + 0x14], EAX
0x80961f9: MOV dword ptr [EBX + 0x18], ECX
0x80961fc: SUB ESP, 0x8
0x80961ff: PUSH EBX
0x8096200: PUSH dword ptr [EBP + 0x8]
0x8096203: CALL 0x080b8b9c
0x8096208: ADD ESP, 0x10
0x809620b: TEST EAX, EAX
0x809620d: JZ 0x08096310
0x8096213: MOV EAX, dword ptr [EBX]
0x8096215: TEST EAX, EAX
0x8096217: JL 0x08096310
0x809621d: JNZ 0x080962e4
0x8096223: SUB ESP, 0x8
0x8096226: LEA EBX, [EBP + -0x20]
0x8096229: PUSH EBX
0x809622a: PUSH dword ptr [EBP + 0x8]
0x809622d: CALL 0x080b8bf4
0x8096232: ADD ESP, 0x10
0x8096235: TEST EAX, EAX
0x8096237: JZ 0x08096244
0x8096239: MOV ESI, dword ptr [EBP + -0x20]
0x809623c: TEST ESI, ESI
0x809623e: JZ 0x080963a9
0x8096244: SUB ESP, 0xc
0x8096247: PUSH 0x82aebb0
0x809624c: CALL 0x08097acc
0x8096251: LEA ESI, [ESI]
0x8096254: XOR EAX, EAX
0x8096256: LEA ESP, [EBP + -0xc]
0x8096259: POP EBX
0x809625a: POP ESI
0x809625b: POP EDI
0x809625c: LEAVE
0x809625d: RET
0x8096260: SUB ESP, 0xc
0x8096263: PUSH 0x82aebf4
0x8096268: CALL 0x08097acc
0x809626d: JMP 0x08096254
0x8096270: SUB ESP, 0x8
0x8096273: LEA EAX, [EBP + -0x18]
0x8096276: PUSH EAX
0x8096277: PUSH dword ptr [EBP + 0x8]
0x809627a: CALL 0x080b8c4c
0x809627f: ADD ESP, 0x10
0x8096282: TEST EAX, EAX
0x8096284: JZ 0x08096522
0x809628a: SUB ESP, 0x8
0x809628d: PUSH 0x0
0x809628f: PUSH dword ptr [EBP + -0x18]
0x8096292: CALL 0x0807f5b4
0x8096297: MOV ESI, EAX
0x8096299: POP EAX
0x809629a: PUSH dword ptr [EBP + -0x18]
0x809629d: CALL 0x080891d4
0x80962a2: ADD ESP, 0x10
0x80962a5: TEST ESI, ESI
0x80962a7: JZ 0x08096534
0x80962ad: MOV AL, byte ptr [EBP + -0x11]
0x80962b0: TEST AL, 0x2
0x80962b2: JZ 0x080961d5
0x80962b8: SUB ESP, 0x8
0x80962bb: LEA EAX, [EBP + -0x1c]
0x80962be: PUSH EAX
0x80962bf: PUSH dword ptr [EBP + 0x8]
0x80962c2: CALL 0x080b8bf4
0x80962c7: ADD ESP, 0x10
0x80962ca: TEST EAX, EAX
0x80962cc: JNZ 0x080961d5
0x80962d2: SUB ESP, 0xc
0x80962d5: PUSH 0x82aec24
0x80962da: CALL 0x08097acc
0x80962df: JMP 0x08096254
0x80962e4: SHL EAX, 0x2
0x80962e7: PUSH ECX
0x80962e8: PUSH EAX
0x80962e9: LEA EAX, [EBX + 0x4]
0x80962ec: PUSH EAX
0x80962ed: PUSH dword ptr [EBP + 0x8]
0x80962f0: CALL 0x080b8a70
0x80962f5: ADD ESP, 0x10
0x80962f8: TEST EAX, EAX
0x80962fa: JNZ 0x08096322
0x80962fc: SUB ESP, 0xc
0x80962ff: PUSH 0x82aec54
0x8096304: CALL 0x08097acc
0x8096309: JMP 0x08096254
0x8096310: SUB ESP, 0xc
0x8096313: PUSH 0x82aec7c
0x8096318: CALL 0x08097acc
0x809631d: JMP 0x08096254
0x8096322: SUB ESP, 0x8
0x8096325: LEA EAX, [EBX + 0x8]
0x8096328: PUSH EAX
0x8096329: PUSH dword ptr [EBP + 0x8]
0x809632c: CALL 0x080b8b9c
0x8096331: ADD ESP, 0x10
0x8096334: TEST EAX, EAX
0x8096336: JZ 0x08096397
0x8096338: MOV EAX, dword ptr [EBX + 0x8]
0x809633b: TEST EAX, EAX
0x809633d: JL 0x08096397
0x809633f: JLE 0x0809635a
0x8096341: PUSH EDX
0x8096342: PUSH EAX
0x8096343: LEA EAX, [EBX + 0xc]
0x8096346: PUSH EAX
0x8096347: PUSH dword ptr [EBP + 0x8]
0x809634a: CALL 0x080b8a70
0x809634f: ADD ESP, 0x10
0x8096352: TEST EAX, EAX
0x8096354: JZ 0x080964e3
0x809635a: SUB ESP, 0x8
0x809635d: LEA EAX, [EBP + -0x24]
0x8096360: PUSH EAX
0x8096361: PUSH dword ptr [EBP + 0x8]
0x8096364: CALL 0x080b8bf4
0x8096369: ADD ESP, 0x10
0x809636c: TEST EAX, EAX
0x809636e: JZ 0x08096385
0x8096370: MOV EAX, dword ptr [EBP + -0x24]
0x8096373: TEST EAX, EAX
0x8096375: JZ 0x08096402
0x809637b: MOV EDX, dword ptr [EBX]
0x809637d: CMP EDX, EAX
0x809637f: JZ 0x08096546
0x8096385: SUB ESP, 0xc
0x8096388: PUSH 0x82aecac
0x809638d: CALL 0x08097acc
0x8096392: JMP 0x08096254
0x8096397: SUB ESP, 0xc
0x809639a: PUSH 0x82aece4
0x809639f: CALL 0x08097acc
0x80963a4: JMP 0x08096254
0x80963a9: SUB ESP, 0x8
0x80963ac: PUSH EBX
0x80963ad: PUSH dword ptr [EBP + 0x8]
0x80963b0: CALL 0x080b8bf4
0x80963b5: ADD ESP, 0x10
0x80963b8: TEST EAX, EAX
0x80963ba: JZ 0x080963cc
0x80963bc: MOV EBX, dword ptr [EBP + -0x20]
0x80963bf: TEST EBX, EBX
0x80963c1: MOV EAX, 0x1
0x80963c6: JZ 0x08096256
0x80963cc: SUB ESP, 0xc
0x80963cf: PUSH 0x82aed18
0x80963d4: CALL 0x08097acc
0x80963d9: JMP 0x08096254
0x80963de: SUB ESP, 0xc
0x80963e1: PUSH 0x82aed74
0x80963e6: CALL 0x08097acc
0x80963eb: JMP 0x08096254
0x80963f0: SUB ESP, 0xc
0x80963f3: PUSH 0x82aeda4
0x80963f8: CALL 0x08097acc
0x80963fd: JMP 0x08096254
0x8096402: MOV EDX, dword ptr [EBX]
0x8096404: MOV ESI, dword ptr [EBX + 0x4]
0x8096407: CMP EDX, 0x0
0x809640a: MOV dword ptr [EBP + -0x2c], 0x0
0x8096411: JMP 0x08096452
0x8096413: SUB ESP, 0xc
0x8096416: MOVZX EAX, byte ptr [ESI + 0x3]
0x809641a: PUSH EAX
0x809641b: CALL 0x080a369c
0x8096420: ADD ESP, 0x10
0x8096423: TEST EAX, EAX
0x8096425: MOV EDX, EAX
0x8096427: JZ 0x08096580
0x809642d: MOV EDI, dword ptr [EAX + 0x18]
0x8096430: TEST EDI, EDI
0x8096432: JS 0x08096493
0x8096434: MOV AL, byte ptr [ESI + 0x3]
0x8096437: CMP AL, 0x40
0x8096439: JZ 0x080964c4
0x809643f: CMP AL, 0xb
0x8096441: JZ 0x080964f5
0x8096447: INC dword ptr [EBP + -0x2c]
0x809644a: MOV EDX, dword ptr [EBX]
0x809644c: ADD ESI, 0x4
0x809644f: CMP EDX, dword ptr [EBP + -0x2c]
0x8096452: JG 0x08096413
0x8096454: MOV EAX, dword ptr [EBX + 0x10]
0x8096457: TEST EAX, EAX
0x8096459: JZ 0x08096489
0x809645b: MOV ECX, dword ptr [EBX + 0x18]
0x809645e: TEST ECX, ECX
0x8096460: JZ 0x08096489
0x8096462: CMP EDX, 0x0
0x8096465: MOV dword ptr [EBP + -0x2c], 0x0
0x809646c: MOV ECX, EAX
0x809646e: JLE 0x08096489
0x8096470: MOV EBX, dword ptr [EBX + 0x14]
0x8096473: MOVSX EAX, word ptr [ECX]
0x8096476: CMP EAX, EBX
0x8096478: JGE 0x08096592
0x809647e: INC dword ptr [EBP + -0x2c]
0x8096481: ADD ECX, 0x4
0x8096484: CMP EDX, dword ptr [EBP + -0x2c]
0x8096487: JG 0x08096473
0x8096489: MOV EAX, 0x1
0x809648e: JMP 0x08096256
0x8096493: MOVSX EAX, byte ptr [ESI + 0x2]
0x8096497: SHL EAX, 0x10
0x809649a: MOV dword ptr [EBP + -0x30], EAX
0x809649d: MOVSX EAX, word ptr [ESI]
0x80964a0: OR dword ptr [EBP + -0x30], EAX
0x80964a3: MOVZX EAX, byte ptr [EDX + 0xc]
0x80964a7: MOV EDX, dword ptr [EBP + -0x30]
0x80964aa: LEA EAX, [EDX + EAX*0x4]
0x80964ad: CMP EAX, dword ptr [EBX + 0x8]
0x80964b0: JLE 0x08096434
0x80964b2: SUB ESP, 0xc
0x80964b5: PUSH 0x82aedd0
0x80964ba: CALL 0x08097acc
0x80964bf: JMP 0x08096254
0x80964c4: SUB ESP, 0xc
0x80964c7: MOVSX EAX, word ptr [ESI]
0x80964ca: PUSH EAX
0x80964cb: CALL 0x080b8fec
0x80964d0: ADD ESP, 0x10
0x80964d3: TEST EAX, EAX
0x80964d5: JS 0x08096254
0x80964db: MOV word ptr [ESI], AX
0x80964de: JMP 0x08096447
0x80964e3: SUB ESP, 0xc
0x80964e6: PUSH 0x82aee00
0x80964eb: CALL 0x08097acc
0x80964f0: JMP 0x08096254
0x80964f5: SUB ESP, 0xc
0x80964f8: MOV EAX, dword ptr [EBX + 0xc]
0x80964fb: MOV ECX, dword ptr [EBP + -0x30]
0x80964fe: PUSH dword ptr [EAX + ECX*0x1 + 0x4]
0x8096502: CALL 0x080b8fec
0x8096507: ADD ESP, 0x10
0x809650a: TEST EAX, EAX
0x809650c: JS 0x08096254
0x8096512: MOV EDX, dword ptr [EBX + 0xc]
0x8096515: CWDE
0x8096516: MOV ECX, dword ptr [EBP + -0x30]
0x8096519: MOV dword ptr [EDX + ECX*0x1 + 0x4], EAX
0x809651d: JMP 0x08096447
0x8096522: SUB ESP, 0xc
0x8096525: PUSH 0x82aee28
0x809652a: CALL 0x08097acc
0x809652f: JMP 0x08096254
0x8096534: SUB ESP, 0xc
0x8096537: PUSH 0x82aee58
0x809653c: CALL 0x08097acc
0x8096541: JMP 0x08096254
0x8096546: TEST EDX, EDX
0x8096548: JZ 0x08096404
0x809654e: PUSH EAX
0x809654f: LEA EAX, [EDX*0x4 + 0x0]
0x8096556: PUSH EAX
0x8096557: LEA EAX, [EBX + 0x10]
0x809655a: PUSH EAX
0x809655b: PUSH dword ptr [EBP + 0x8]
0x809655e: CALL 0x080b8a70
0x8096563: ADD ESP, 0x10
0x8096566: TEST EAX, EAX
0x8096568: JNZ 0x08096402
0x809656e: SUB ESP, 0xc
0x8096571: PUSH 0x82aee88
0x8096576: CALL 0x08097acc
0x809657b: JMP 0x08096254
0x8096580: SUB ESP, 0xc
0x8096583: PUSH 0x82aeeb8
0x8096588: CALL 0x08097acc
0x809658d: JMP 0x08096254
0x8096592: SUB ESP, 0xc
0x8096595: PUSH 0x82aeee0
0x809659a: CALL 0x08097acc
0x809659f: JMP 0x08096254
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="initMysqlEfun"></a>

## initMysqlEfun @ 0x816f9b0

Evidence: `ev_f2755060e3cc73aedf5df1a4b4154acad03373ed1c9d3778601464f92db4a127`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int initMysqlEfun(void)

{
  vm_dbMysqlInit();
  vm_updateEfuns();
  return 1;
}


```

<details><summary>汇编</summary>

```asm
0x816f9b0: PUSH EBP
0x816f9b1: MOV EBP, ESP
0x816f9b3: SUB ESP, 0x8
0x816f9b6: CALL 0x0816f9e0
0x816f9bb: CALL 0x0808600c
0x816f9c0: MOV EAX, 0x1
0x816f9c5: LEAVE
0x816f9c6: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_callInEfun"></a>

## vm_callInEfun @ 0x8093d98

Evidence: `ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f`

```c

void vm_callInEfun(Vm_Index_T funOffset,int argn)

{
  Vm_Fun *pVVar1;
  Vm_FunCall_Context_T *pVVar2;
  Vm_PCB_T *pVVar3;
  int iVar4;
  Vm_FunCall_Context_T *pVVar5;
  Vm_FunCall_Context_T *pVVar6;
  bool bVar7;
  int argn_local;
  
                    /* Unresolved local var: Vm_Efun_T * pEfun@[DW_OP_reg3(EBX)] */
  if (vm_funContextSize <= vm_funCp) {
    vm_error("Too depth (%d) function call!\n",vm_funCp);
  }
  if ((funOffset < 0) || (vm_pGlobalProgram->maxFunctionNo <= funOffset)) {
    vm_error("External function (index = %d) is not implemented yet.\n",(int)funOffset);
    pVVar1 = vm_pGlobalProgram->pFunctions[funOffset].pFun;
  }
  else {
    pVVar1 = vm_pGlobalProgram->pFunctions[funOffset].pFun;
  }
  if (vm_gatheringCallStackInfo != 0) {
    vm_enterPPCopyStackCriticalSection();
  }
  vm_funCp = vm_funCp + 1;
  vm_updateCurrentFunCallContext();
  pVVar2 = vm_currentContext;
  pVVar5 = vm_currentContext + -1;
  pVVar6 = vm_currentContext;
  for (iVar4 = 10; iVar4 != 0; iVar4 = iVar4 + -1) {
    pVVar6->bp = pVVar5->bp;
    pVVar5 = (Vm_FunCall_Context_T *)&pVVar5->argNum;
    pVVar6 = (Vm_FunCall_Context_T *)&pVVar6->argNum;
  }
  pVVar2->bp = vm_sp;
  pVVar2->argNum = argn;
  bVar7 = vm_gatheringCallStackInfo != 0;
  pVVar2->flags = 1;
  (pVVar2->f).pEfun = (Vm_Efun *)pVVar1;
  if (bVar7) {
    vm_leavePPCopyStackCriticalSection();
  }
  (*(code *)(pVVar1->codeSegment).nodes)();
  if (vm_ctrlFlags != 0) {
    if ((vm_ctrlFlags & 0x10) == 0) {
      if ((vm_ctrlFlags & 4) == 0) goto LAB_08093e3c;
      pVVar3 = vm_getCurrentPCB();
      pVVar3->flags = pVVar3->flags | 0x40;
    }
    return;
  }
LAB_08093e3c:
  vm_returnFromEfun();
  return;
}


```

<details><summary>汇编</summary>

```asm
0x8093d98: PUSH EBP
0x8093d99: MOV EBP, ESP
0x8093d9b: PUSH EDI
0x8093d9c: PUSH ESI
0x8093d9d: PUSH EBX
0x8093d9e: SUB ESP, 0xc
0x8093da1: MOV EAX, dword ptr [EBP + 0xc]
0x8093da4: MOV dword ptr [EBP + -0x10], EAX
0x8093da7: MOV EAX, [0x086619c0]
0x8093dac: CMP EAX, dword ptr [0x086619c4]
0x8093db2: MOV EDI, dword ptr [EBP + 0x8]
0x8093db5: JGE 0x08093e98
0x8093dbb: TEST DI, DI
0x8093dbe: JS 0x08093e48
0x8093dc4: MOV EAX, [0x0847aad0]
0x8093dc9: CMP word ptr [EAX + 0x10], DI
0x8093dcd: MOVSX ESI, DI
0x8093dd0: JLE 0x08093e48
0x8093dd2: MOV EAX, dword ptr [EAX + 0x2c]
0x8093dd5: MOV EBX, dword ptr [EAX + ESI*0x8]
0x8093dd8: MOV EAX, [0x0860ae64]
0x8093ddd: TEST EAX, EAX
0x8093ddf: JNZ 0x08093e74
0x8093de5: LEA ESI, [ESI]
0x8093de8: INC dword ptr [0x086619c0]
0x8093dee: CALL 0x0809789c
0x8093df3: MOV EDX, dword ptr [0x086619d8]
0x8093df9: LEA ESI, [EDX + -0x28]
0x8093dfc: CLD
0x8093dfd: MOV ECX, 0xa
0x8093e02: MOV EDI, EDX
0x8093e04: MOVSD.REP ES:EDI, ESI
0x8093e06: MOV EAX, [0x086619cc]
0x8093e0b: MOV dword ptr [EDX], EAX
0x8093e0d: MOV EAX, dword ptr [EBP + -0x10]
0x8093e10: MOV dword ptr [EDX + 0x4], EAX
0x8093e13: MOV EAX, [0x0860ae64]
0x8093e18: TEST EAX, EAX
0x8093e1a: MOV dword ptr [EDX + 0x8], 0x1
0x8093e21: MOV dword ptr [EDX + 0x24], EBX
0x8093e24: JNZ 0x08093e90
0x8093e26: CALL dword ptr [EBX + 0x14]
0x8093e29: MOV EAX, [0x086619d4]
0x8093e2e: TEST EAX, EAX
0x8093e30: JZ 0x08093e3c
0x8093e32: TEST AL, 0x10
0x8093e34: JNZ 0x08093e88
0x8093e36: TEST AL, 0x4
0x8093e38: JNZ 0x08093e7e
0x8093e3a: MOV ESI, ESI
0x8093e3c: LEA ESP, [EBP + -0xc]
0x8093e3f: POP EBX
0x8093e40: POP ESI
0x8093e41: POP EDI
0x8093e42: LEAVE
0x8093e43: JMP 0x08093c5c
0x8093e48: SUB ESP, 0x8
0x8093e4b: MOVSX ESI, DI
0x8093e4e: PUSH ESI
0x8093e4f: PUSH 0x82ae2e4
0x8093e54: CALL 0x08097b24
0x8093e59: MOV EAX, [0x0847aad0]
0x8093e5e: MOV EAX, dword ptr [EAX + 0x2c]
0x8093e61: MOV EBX, dword ptr [EAX + ESI*0x8]
0x8093e64: MOV EAX, [0x0860ae64]
0x8093e69: ADD ESP, 0x10
0x8093e6c: TEST EAX, EAX
0x8093e6e: JZ 0x08093de8
0x8093e74: CALL 0x080bb0d4
0x8093e79: JMP 0x08093de8
0x8093e7e: CALL 0x080a9310
0x8093e83: OR dword ptr [EAX + 0x28], 0x40
0x8093e87: NOP
0x8093e88: LEA ESP, [EBP + -0xc]
0x8093e8b: POP EBX
0x8093e8c: POP ESI
0x8093e8d: POP EDI
0x8093e8e: LEAVE
0x8093e8f: RET
0x8093e90: CALL 0x080bb0e8
0x8093e95: JMP 0x08093e26
0x8093e98: SUB ESP, 0x8
0x8093e9b: PUSH EAX
0x8093e9c: PUSH 0x82ae31c
0x8093ea1: CALL 0x08097b24
0x8093ea6: ADD ESP, 0x10
0x8093ea9: JMP 0x08093dbb
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；Ghidra Listing reports a terminal call at 0x8093e43 to 0x8093c5c vm_returnFromEfun (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.


<a id="vm_efunPerformanceProfileInit"></a>

## vm_efunPerformanceProfileInit @ 0x80fb778

Evidence: `ev_f80064bb3dc88680ac1e5eb8869d3f56ed19da389cdad9320405ecca784fab3b`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_efunPerformanceProfileInit(void)

{
  vm_addEfuns(efunsPerformanceProfile);
  return;
}


```

<details><summary>汇编</summary>

```asm
0x80fb778: PUSH EBP
0x80fb779: MOV EBP, ESP
0x80fb77b: SUB ESP, 0x14
0x80fb77e: PUSH 0x83d2de0
0x80fb783: CALL 0x08085ce4
0x80fb788: LEAVE
0x80fb789: RET
```

</details>

局限：The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.；The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.；Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.；Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.；References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.；The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.；Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.；Synthetic Ghidra entry-point references without actionable memory sources are omitted.；The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.；macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

