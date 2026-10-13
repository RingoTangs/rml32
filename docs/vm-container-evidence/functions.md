# Ghidra 容器语义证据

仅静态分析，反编译不是原始源码。目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。

## vm_allocateMapping / procedure_pseudo_code

Evidence: `ev_89868107773a8596a0aab453bee289cab747793e6f77ad1bc33cd308ad0af9bd`

```c

/* WARNING: Unknown calling convention */

Vm_Mapping_T * vm_allocateMapping(uint size)

{
  Vm_Mapping_T *pVVar1;
  
                    /* Unresolved local var: Vm_Mapping_T * pNewMap@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_safeAllocateMapping(size);
  if (pVVar1 == (Vm_Mapping_T *)0x0) {
    vm_error("Allocate mapping - out of memory.\n");
  }
  return pVVar1;
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

## vm_createArray / procedure_assembly

Evidence: `ev_c64cd042d0b1c61f07afc40b63a8b11f7dea1100854de80506a653da39ccf527`

```asm
0x80918c8: PUSH EBP
0x80918c9: MOV EBP, ESP
0x80918cb: PUSH EDI
0x80918cc: PUSH ESI
0x80918cd: PUSH EBX
0x80918ce: SUB ESP, 0xc
0x80918d1: MOV ESI, dword ptr [0x086619cc]
0x80918d7: TEST ESI, ESI
0x80918d9: MOV EDI, dword ptr [EBP + 0x8]
0x80918dc: JZ 0x08091a55
0x80918e2: TEST EDI, EDI
0x80918e4: JZ 0x08091a10
0x80918ea: XOR EBX, EBX
0x80918ec: CMP EBX, EDI
0x80918ee: JNC 0x08091914
0x80918f0: MOV EAX, [0x086619cc]
0x80918f5: ADD EAX, EBX
0x80918f7: LEA EAX, [EAX + EAX*0x2]
0x80918fa: MOV EDX, dword ptr [0x08661964]
0x8091900: CMP byte ptr [EDX + EAX*0x8 + 0x4], 0x8
0x8091905: LEA ESI, [EBX + 0x1]
0x8091908: JZ 0x080919f8
0x809190e: MOV EBX, ESI
0x8091910: CMP EBX, EDI
0x8091912: JC 0x080918f0
0x8091914: SUB ESP, 0xc
0x8091917: PUSH EDI
0x8091918: XOR EBX, EBX
0x809191a: CALL 0x0809b1d8
0x809191f: ADD ESP, 0x10
0x8091922: CMP EBX, EDI
0x8091924: MOV dword ptr [EBP + -0x10], EAX
0x8091927: JNC 0x0809198b
0x8091929: MOV ESI, dword ptr [EAX + 0xc]
0x809192c: MOV EAX, [0x086619cc]
0x8091931: ADD EAX, EBX
0x8091933: MOV EDX, dword ptr [0x08661964]
0x8091939: LEA EAX, [EAX + EAX*0x2]
0x809193c: MOV ECX, dword ptr [ESI + EBX*0x4]
0x809193f: MOV AL, byte ptr [EDX + EAX*0x8 + 0x4]
0x8091943: MOV byte ptr [ECX + 0x4], AL
0x8091946: MOV EDX, dword ptr [EBP + -0x10]
0x8091949: MOV EAX, dword ptr [EDX + 0xc]
0x809194c: MOV ECX, dword ptr [EAX + EBX*0x4]
0x809194f: MOV EAX, [0x086619cc]
0x8091954: ADD EAX, EBX
0x8091956: MOV EDX, dword ptr [0x08661964]
0x809195c: LEA EAX, [EAX + EAX*0x2]
0x809195f: MOV AL, byte ptr [EDX + EAX*0x8 + 0x5]
0x8091963: MOV byte ptr [ECX + 0x5], AL
0x8091966: MOV ECX, dword ptr [EBP + -0x10]
0x8091969: MOV EAX, [0x086619cc]
0x809196e: ADD EAX, EBX
0x8091970: MOV ESI, dword ptr [ECX + 0xc]
0x8091973: MOV ECX, dword ptr [ESI + EBX*0x4]
0x8091976: MOV EDX, dword ptr [0x08661964]
0x809197c: LEA EAX, [EAX + EAX*0x2]
0x809197f: INC EBX
0x8091980: MOV EAX, dword ptr [EDX + EAX*0x8 + 0x8]
0x8091984: CMP EBX, EDI
0x8091986: MOV dword ptr [ECX + 0x8], EAX
0x8091989: JC 0x0809192c
0x809198b: SUB EDI, 0x2
0x809198e: CMP EDI, -0x1
0x8091991: JZ 0x08091a6a
0x8091997: MOV EAX, [0x086619cc]
0x809199c: LEA EDX, [EAX + EAX*0x2]
0x809199f: MOV EAX, [0x08661964]
0x80919a4: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x0
0x80919a9: MOV EAX, [0x086619cc]
0x80919ae: LEA EAX, [EAX + EAX*0x2]
0x80919b1: MOV EDX, dword ptr [0x08661964]
0x80919b7: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x80919bc: MOV EAX, [0x086619cc]
0x80919c1: DEC EDI
0x80919c2: INC EAX
0x80919c3: CMP EDI, -0x1
0x80919c6: MOV [0x086619cc], EAX
0x80919cb: JNZ 0x0809199c
0x80919cd: LEA EDX, [EAX + EAX*0x2]
0x80919d0: MOV EAX, [0x08661964]
0x80919d5: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x6
0x80919da: MOV EAX, [0x086619cc]
0x80919df: LEA EAX, [EAX + EAX*0x2]
0x80919e2: MOV EDX, dword ptr [0x08661964]
0x80919e8: MOV ECX, dword ptr [EBP + -0x10]
0x80919eb: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
0x80919ef: LEA ESP, [EBP + -0xc]
0x80919f2: POP EBX
0x80919f3: POP ESI
0x80919f4: POP EDI
0x80919f5: LEAVE
0x80919f6: RET
0x80919f8: SUB ESP, 0x8
0x80919fb: PUSH ESI
0x80919fc: PUSH 0x82addf0
0x8091a01: CALL 0x08097b24
0x8091a06: MOV EBX, ESI
0x8091a08: ADD ESP, 0x10
0x8091a0b: JMP 0x08091910
0x8091a10: SUB ESP, 0xc
0x8091a13: PUSH 0x0
0x8091a15: CALL 0x0809b1d8
0x8091a1a: MOV EBX, EAX
0x8091a1c: MOV EAX, [0x086619cc]
0x8091a21: ADD ESP, 0x10
0x8091a24: TEST EAX, EAX
0x8091a26: JZ 0x08091a96
0x8091a28: DEC EAX
0x8091a29: MOV [0x086619cc], EAX
0x8091a2e: LEA EDX, [EAX + EAX*0x2]
0x8091a31: MOV EAX, [0x08661964]
0x8091a36: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x6
0x8091a3b: MOV EAX, [0x086619cc]
0x8091a40: MOV EDX, dword ptr [0x08661964]
0x8091a46: LEA EAX, [EAX + EAX*0x2]
0x8091a49: MOV dword ptr [EDX + EAX*0x8 + 0x8], EBX
0x8091a4d: LEA ESP, [EBP + -0xc]
0x8091a50: POP EBX
0x8091a51: POP ESI
0x8091a52: POP EDI
0x8091a53: LEAVE
0x8091a54: RET
0x8091a55: SUB ESP, 0xc
0x8091a58: PUSH 0x82ad98c
0x8091a5d: CALL 0x08097b24
0x8091a62: ADD ESP, 0x10
0x8091a65: JMP 0x080918e2
0x8091a6a: MOV EAX, [0x086619cc]
0x8091a6f: LEA EDX, [EAX + EAX*0x2]
0x8091a72: MOV EAX, [0x08661964]
0x8091a77: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x6
0x8091a7c: MOV EAX, [0x086619cc]
0x8091a81: LEA EAX, [EAX + EAX*0x2]
0x8091a84: MOV EDX, dword ptr [0x08661964]
0x8091a8a: MOV ECX, dword ptr [EBP + -0x10]
0x8091a8d: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
0x8091a91: JMP 0x080919ef
0x8091a96: SUB ESP, 0xc
0x8091a99: PUSH 0x82ad98c
0x8091a9e: CALL 0x08097b24
0x8091aa3: ADD ESP, 0x10
0x8091aa6: MOV EAX, [0x086619cc]
0x8091aab: JMP 0x08091a28
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_createArray / procedure_pseudo_code

Evidence: `ev_7cdd1e3195ae9d4cd44976abd598cc5e148be1b4e8697dcf26d48c781f87cc07`

```c

void vm_createArray(uint n)

{
  int iVar1;
  Vm_String_T *pVVar2;
  int iVar3;
  uint uVar4;
  char *pcVar5;
  Vm_Array_T *pArray;
  
                    /* Unresolved local var: uint i@[DW_OP_reg3(EBX)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (n == 0) {
    pVVar2 = (Vm_String_T *)vm_allocateEmptyArray(0);
    if (vm_sp == 0) {
      vm_error("Stack overflow when calculating.\n");
    }
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\x06';
    vm_stack[vm_sp].v.string = pVVar2;
    return;
  }
  uVar4 = 0;
  if (n != 0) {
    do {
      iVar1 = vm_sp + uVar4;
      uVar4 = uVar4 + 1;
      if (vm_stack[iVar1].type == '\b') {
        vm_error("Bad value (got pointer) of element %d to create array.\n",uVar4);
      }
    } while (uVar4 < n);
  }
  uVar4 = 0;
  pVVar2 = (Vm_String_T *)vm_allocateEmptyArray(n);
  if (n != 0) {
    pcVar5 = pVVar2->ptr;
    do {
      *(uchar *)(*(int *)(pcVar5 + uVar4 * 4) + 4) = vm_stack[vm_sp + uVar4].type;
      *(uchar *)(*(int *)(pVVar2->ptr + uVar4 * 4) + 5) = vm_stack[vm_sp + uVar4].attrib;
      iVar3 = vm_sp + uVar4;
      pcVar5 = pVVar2->ptr;
      iVar1 = uVar4 * 4;
      uVar4 = uVar4 + 1;
      *(anon_union_4_11_5000f750_for_v *)(*(int *)(pcVar5 + iVar1) + 8) = vm_stack[iVar3].v;
    } while (uVar4 < n);
  }
  iVar1 = n - 2;
  if (iVar1 == -1) {
    vm_stack[vm_sp].type = '\x06';
    vm_stack[vm_sp].v.string = pVVar2;
  }
  else {
    do {
      vm_stack[vm_sp].type = '\0';
      vm_stack[vm_sp].attrib = '\0';
      iVar1 = iVar1 + -1;
      vm_sp = vm_sp + 1;
    } while (iVar1 != -1);
    vm_stack[vm_sp].type = '\x06';
    vm_stack[vm_sp].v.string = pVVar2;
  }
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

## vm_createMapping / procedure_assembly

Evidence: `ev_76c678f220405160911aa5543fbdaf68e54b54a0ab78ffa9817f4b0658b68433`

```asm
0x8091af0: PUSH EBP
0x8091af1: MOV EBP, ESP
0x8091af3: PUSH EDI
0x8091af4: PUSH ESI
0x8091af5: PUSH EBX
0x8091af6: SUB ESP, 0xc
0x8091af9: MOV EAX, [0x086619cc]
0x8091afe: TEST EAX, EAX
0x8091b00: MOV EDI, dword ptr [EBP + 0x8]
0x8091b03: JZ 0x08091cb8
0x8091b09: XOR ESI, ESI
0x8091b0b: CMP ESI, EDI
0x8091b0d: JNC 0x08091b54
0x8091b0f: MOV EBX, EDI
0x8091b11: SHR EBX, 0x1
0x8091b13: JMP 0x08091b1d
0x8091b18: INC ESI
0x8091b19: CMP ESI, EDI
0x8091b1b: JNC 0x08091b54
0x8091b1d: MOV EAX, [0x086619cc]
0x8091b22: ADD EAX, ESI
0x8091b24: LEA EAX, [EAX + EAX*0x2]
0x8091b27: MOV EDX, dword ptr [0x08661964]
0x8091b2d: CMP byte ptr [EDX + EAX*0x8 + 0x4], 0x8
0x8091b32: JNZ 0x08091b18
0x8091b34: MOV EAX, ESI
0x8091b36: SHR EAX, 0x1
0x8091b38: MOV EDX, EBX
0x8091b3a: SUB ESP, 0x8
0x8091b3d: SUB EDX, EAX
0x8091b3f: PUSH EDX
0x8091b40: PUSH 0x82ade28
0x8091b45: INC ESI
0x8091b46: CALL 0x08097b24
0x8091b4b: ADD ESP, 0x10
0x8091b4e: CMP ESI, EDI
0x8091b50: JC 0x08091b1d
0x8091b52: MOV ESI, ESI
0x8091b54: MOV EAX, EDI
0x8091b56: SUB ESP, 0xc
0x8091b59: SHR EAX, 0x1
0x8091b5b: PUSH EAX
0x8091b5c: CALL 0x0809c308
0x8091b61: MOV dword ptr [EBP + -0x10], EAX
0x8091b64: MOV EAX, [0x086619cc]
0x8091b69: ADD ESP, 0x10
0x8091b6c: TEST EAX, EAX
0x8091b6e: MOV EBX, EAX
0x8091b70: JZ 0x08091c9e
0x8091b76: DEC EAX
0x8091b77: MOV [0x086619cc], EAX
0x8091b7c: LEA EDX, [EAX + EAX*0x2]
0x8091b7f: MOV EAX, [0x08661964]
0x8091b84: MOV byte ptr [EAX + EDX*0x8 + 0x4], 0x7
0x8091b89: MOV EAX, [0x086619cc]
0x8091b8e: XOR ESI, ESI
0x8091b90: LEA EAX, [EAX + EAX*0x2]
0x8091b93: MOV EDX, dword ptr [0x08661964]
0x8091b99: MOV ECX, dword ptr [EBP + -0x10]
0x8091b9c: CMP ESI, EDI
0x8091b9e: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
0x8091ba2: JNC 0x08091c0e
0x8091ba4: LEA EAX, [EBX + EBX*0x2]
0x8091ba7: LEA EBX, [EAX*0x8 + 0x0]
0x8091bae: MOV ESI, ESI
0x8091bb0: PUSH EAX
0x8091bb1: MOV EAX, EBX
0x8091bb3: PUSH 0x1
0x8091bb5: ADD EAX, dword ptr [0x08661964]
0x8091bbb: PUSH EAX
0x8091bbc: PUSH dword ptr [EBP + -0x10]
0x8091bbf: CALL 0x0809d714
0x8091bc4: MOV EDX, EBX
0x8091bc6: ADD EDX, dword ptr [0x08661964]
0x8091bcc: MOV DL, byte ptr [EDX + 0x1c]
0x8091bcf: MOV byte ptr [EAX + 0x4], DL
0x8091bd2: MOV EDX, EBX
0x8091bd4: ADD EDX, dword ptr [0x08661964]
0x8091bda: MOV DL, byte ptr [EDX + 0x1d]
0x8091bdd: MOV byte ptr [EAX + 0x5], DL
0x8091be0: MOV EDX, EBX
0x8091be2: ADD EDX, dword ptr [0x08661964]
0x8091be8: ADD EDX, 0x18
0x8091beb: MOV ECX, dword ptr [EDX + 0x8]
0x8091bee: MOV dword ptr [EAX + 0x8], ECX
0x8091bf1: MOV byte ptr [EDX + 0x4], 0x0
0x8091bf5: MOV EAX, EBX
0x8091bf7: ADD ESI, 0x2
0x8091bfa: ADD EAX, dword ptr [0x08661964]
0x8091c00: ADD ESP, 0x10
0x8091c03: ADD EBX, 0x30
0x8091c06: CMP ESI, EDI
0x8091c08: MOV byte ptr [EAX + 0x1d], 0x0
0x8091c0c: JC 0x08091bb0
0x8091c0e: MOV EAX, [0x086619cc]
0x8091c13: MOV EDX, dword ptr [0x08661964]
0x8091c19: LEA EAX, [EAX + EAX*0x2]
0x8091c1c: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x8091c21: MOV EAX, [0x086619cc]
0x8091c26: LEA EAX, [EAX + EAX*0x2]
0x8091c29: MOV EDX, dword ptr [0x08661964]
0x8091c2f: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x8091c34: MOV EAX, [0x086619cc]
0x8091c39: INC EAX
0x8091c3a: TEST EDI, EDI
0x8091c3c: MOV EDX, EAX
0x8091c3e: JLE 0x08091c6d
0x8091c40: MOV EBX, EDI
0x8091c42: MOV ESI, ESI
0x8091c44: MOV dword ptr [0x086619cc], EDX
0x8091c4a: LEA EAX, [EAX + EAX*0x2]
0x8091c4d: MOV EDX, dword ptr [0x08661964]
0x8091c53: SUB ESP, 0xc
0x8091c56: LEA EAX, [EDX + EAX*0x8]
0x8091c59: PUSH EAX
0x8091c5a: CALL 0x0808ad98
0x8091c5f: MOV EAX, [0x086619cc]
0x8091c64: INC EAX
0x8091c65: ADD ESP, 0x10
0x8091c68: DEC EBX
0x8091c69: MOV EDX, EAX
0x8091c6b: JNZ 0x08091c44
0x8091c6d: DEC EAX
0x8091c6e: MOV [0x086619cc], EAX
0x8091c73: MOV EDX, dword ptr [0x08661964]
0x8091c79: LEA EAX, [EAX + EAX*0x2]
0x8091c7c: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x7
0x8091c81: MOV EAX, [0x086619cc]
0x8091c86: MOV EDX, dword ptr [0x08661964]
0x8091c8c: MOV ECX, dword ptr [EBP + -0x10]
0x8091c8f: LEA EAX, [EAX + EAX*0x2]
0x8091c92: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
0x8091c96: LEA ESP, [EBP + -0xc]
0x8091c99: POP EBX
0x8091c9a: POP ESI
0x8091c9b: POP EDI
0x8091c9c: LEAVE
0x8091c9d: RET
0x8091c9e: SUB ESP, 0xc
0x8091ca1: PUSH 0x82ad98c
0x8091ca6: CALL 0x08097b24
0x8091cab: ADD ESP, 0x10
0x8091cae: MOV EAX, [0x086619cc]
0x8091cb3: JMP 0x08091b76
0x8091cb8: SUB ESP, 0xc
0x8091cbb: PUSH 0x82ad98c
0x8091cc0: CALL 0x08097b24
0x8091cc5: ADD ESP, 0x10
0x8091cc8: JMP 0x08091b09
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_createMapping / procedure_pseudo_code

Evidence: `ev_3f7785d6fbdad312aa5446459aa2616b97aa72c94385d825ee32adad3bf3fc65`

```c

void vm_createMapping(uint n)

{
  Vm_Value_T *pVVar1;
  uint uVar2;
  Vm_Mapping_T *m_00;
  Vm_Value_T *pVVar3;
  int iVar4;
  uint uVar5;
  Vm_Mapping_T *m;
  
                    /* Unresolved local var: Vm_Value_T * pToBeAdded@[DW_OP_reg0(EAX)]
                       Unresolved local var: uint bp@[DW_OP_reg3(EBX)]
                       Unresolved local var: uint i@[DW_OP_reg6(ESI)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  uVar5 = 0;
  if (n != 0) {
    do {
      while (vm_stack[vm_sp + uVar5].type != '\b') {
        uVar5 = uVar5 + 1;
        if (n <= uVar5) goto LAB_08091b54;
      }
      uVar2 = uVar5 >> 1;
      uVar5 = uVar5 + 1;
      vm_error("Bad value (got pointer) of pair %d to create mapping.\n",(n >> 1) - uVar2);
    } while (uVar5 < n);
  }
LAB_08091b54:
  m_00 = vm_allocateMapping(n >> 1);
  iVar4 = vm_sp;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\a';
  vm_stack[vm_sp].v.string = (Vm_String_T *)m_00;
  if (n != 0) {
    uVar5 = 0;
    iVar4 = iVar4 * 0x18;
    do {
      pVVar3 = vm_findForInsert(m_00,(Vm_Value_T *)((int)&vm_stack->ref + iVar4),1);
      pVVar3->type = (&vm_stack[1].type)[iVar4];
      pVVar3->attrib = (&vm_stack[1].attrib)[iVar4];
      pVVar1 = vm_stack;
      pVVar3->v = *(anon_union_4_11_5000f750_for_v *)((int)&vm_stack[1].v + iVar4);
      (&pVVar1[1].type)[iVar4] = '\0';
      uVar5 = uVar5 + 2;
      (&vm_stack[1].attrib)[iVar4] = '\0';
      iVar4 = iVar4 + 0x30;
    } while (uVar5 < n);
  }
  vm_stack[vm_sp].type = '\0';
  vm_stack[vm_sp].attrib = '\0';
  vm_sp = vm_sp + 1;
                    /* Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  if (0 < (int)n) {
    do {
      vm_cleanupValue(vm_stack + vm_sp);
      vm_sp = vm_sp + 1;
      n = n - 1;
    } while (n != 0);
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\a';
  vm_stack[vm_sp].v.string = (Vm_String_T *)m_00;
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

## vm_findForInsert / procedure_pseudo_code

Evidence: `ev_6cc7198eb3b6a1f5cbea87d8a19befa50be6d7d4b5aa60cdd9fd77a678a3fbcc`

```c

/* WARNING: Unknown calling convention */

Vm_Value_T * vm_findForInsert(Vm_Mapping_T *m,Vm_Value_T *lv,int doTheFree)

{
  Vm_Value_T *pVVar1;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_safeFindForInsert(m,lv,doTheFree);
  if (pVVar1 == (Vm_Value_T *)0x0) {
    vm_error("Failed to insert new mapping node into mapping.\n");
  }
  return pVVar1;
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

## vm_findInMapping / procedure_pseudo_code

Evidence: `ev_7b72033db6fecc26f1af35a5aeac0b6f57c562f2646b4ca069595c0a6005208c`

```c

/* WARNING: Unknown calling convention */

Vm_Value_T * vm_findInMapping(Vm_Mapping_T *m,Vm_Value_T *lv)

{
  byte bVar1;
  Vm_Mapping_Node_T *pVVar2;
  Vm_String_T *pVVar3;
  Vm_String_T *pVVar4;
  Vm_Buffer *pVVar5;
  ushort uVar6;
  int iVar7;
  uint uVar8;
  char *pcVar9;
  uchar *puVar10;
  char *pcVar11;
  uchar *puVar12;
  bool bVar13;
  
                    /* Unresolved local var: int i@[???]
                       Unresolved local var: Vm_Mapping_Node_T * pElt@[DW_OP_reg3(EBX)] */
  if (lv->type == '\x04') {
    if ((m->attrib & 1) == 0) {
      uVar6 = vm_whashstrc(((lv->v).string)->ptr,0x20);
      uVar8 = (uint)uVar6;
    }
    else {
      pVVar3 = (lv->v).string;
      uVar8 = (uint)pVVar3->hash;
      if (pVVar3->hash == 0) {
        uVar6 = vm_whashstr(pVVar3->ptr,0x20);
        uVar8 = (uint)uVar6;
        pVVar3->hash = uVar6;
      }
    }
  }
  else if (lv->type == '\x05') {
    uVar8 = ((lv->v).buffer)->realSize;
    if (0x20 < uVar8) {
      uVar8 = 0x20;
    }
    uVar8 = vm_whashbuf(((lv->v).buffer)->ptr,uVar8);
  }
  else {
    uVar8 = (lv->v).number >> 4;
  }
  pVVar2 = m->table[uVar8 & m->tableSize];
  do {
    if (pVVar2 == (Vm_Mapping_Node_T *)0x0) {
      return (Vm_Value_T *)0x0;
    }
                    /* Unresolved local var: Vm_String_T * s1@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_String_T * s2@[DW_OP_reg2(EDX)] */
    bVar1 = lv->type;
    if (bVar1 != pVVar2->values[0].type) goto LAB_0809d7a7;
    if (bVar1 == 3) {
      bVar13 = (lv->v).real == pVVar2->values[0].v.real;
      goto LAB_0809d7dc;
    }
    if (bVar1 < 4) {
LAB_0809d7d0:
      bVar13 = pVVar2->values[0].v.info == (lv->v).info;
LAB_0809d7dc:
      if (bVar13) {
LAB_0809d7e3:
        return pVVar2->values + 1;
      }
    }
    else {
      if (bVar1 != 4) {
        if (bVar1 == 5) {
          pVVar5 = pVVar2->values[0].v.buffer;
          uVar8 = ((lv->v).buffer)->realSize;
          if (uVar8 == pVVar5->realSize) {
            bVar13 = true;
            puVar10 = pVVar5->ptr;
            puVar12 = ((lv->v).buffer)->ptr;
            do {
              if (uVar8 == 0) break;
              uVar8 = uVar8 - 1;
              bVar13 = *puVar10 == *puVar12;
              puVar10 = puVar10 + 1;
              puVar12 = puVar12 + 1;
            } while (bVar13);
            goto LAB_0809d7dc;
          }
          goto LAB_0809d7a7;
        }
        goto LAB_0809d7d0;
      }
      pVVar3 = pVVar2->values[0].v.string;
      pVVar4 = (lv->v).string;
      if (pVVar3 == pVVar4) goto LAB_0809d7e3;
      uVar8 = pVVar3->len;
      if (uVar8 == pVVar4->len) {
        if ((m->attrib & 1) == 0) {
          iVar7 = vm_stricmp(pVVar3->ptr,pVVar4->ptr);
          bVar13 = iVar7 == 0;
        }
        else {
          if ((pVVar4->attrib & pVVar3->attrib & 1) != 0) goto LAB_0809d7a7;
          bVar13 = true;
          pcVar9 = pVVar3->ptr;
          pcVar11 = pVVar4->ptr;
          do {
                    /* Unresolved local var: uint size@[???] */
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            bVar13 = *pcVar9 == *pcVar11;
            pcVar9 = pcVar9 + 1;
            pcVar11 = pcVar11 + 1;
          } while (bVar13);
        }
        goto LAB_0809d7dc;
      }
    }
LAB_0809d7a7:
    pVVar2 = pVVar2->pNext;
  } while( true );
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

## vm_queryMember / procedure_assembly

Evidence: `ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459`

```asm
0x808fa60: PUSH EBP
0x808fa61: MOV EBP, ESP
0x808fa63: PUSH EDI
0x808fa64: PUSH ESI
0x808fa65: PUSH EBX
0x808fa66: SUB ESP, 0x1c
0x808fa69: MOV EAX, [0x086619cc]
0x808fa6e: LEA EAX, [EAX + EAX*0x2]
0x808fa71: MOV EDX, dword ptr [0x08661964]
0x808fa77: LEA EBX, [EDX + EAX*0x8]
0x808fa7a: LEA EDI, [EBX + 0x18]
0x808fa7d: MOV AL, byte ptr [EDI + 0x5]
0x808fa80: MOV byte ptr [EBP + -0x19], AL
0x808fa83: MOV AL, byte ptr [EDI + 0x4]
0x808fa86: MOVZX EDX, AL
0x808fa89: CMP EDX, 0x5
0x808fa8c: MOV ESI, dword ptr [EBP + 0x8]
0x808fa8f: JZ 0x0808fb5c
0x808fa95: JLE 0x0808fad4
0x808fa97: CMP EDX, 0x6
0x808fa9a: JZ 0x0808fbe4
0x808faa0: CMP EDX, 0x7
0x808faa3: JZ 0x0808fc81
0x808faa9: SUB ESP, 0xc
0x808faac: MOVZX EAX, AL
0x808faaf: PUSH EAX
0x808fab0: CALL 0x0808a13c
0x808fab5: POP EDX
0x808fab6: POP ECX
0x808fab7: PUSH EAX
0x808fab8: PUSH 0x82acfd8
0x808fabd: CALL 0x08097b24
0x808fac2: MOV AL, byte ptr [EBP + -0x19]
0x808fac5: MOV byte ptr [EDI + 0x5], AL
0x808fac8: ADD ESP, 0x10
0x808facb: LEA ESP, [EBP + -0xc]
0x808face: POP EBX
0x808facf: POP ESI
0x808fad0: POP EDI
0x808fad1: LEAVE
0x808fad2: RET
0x808fad4: CMP EDX, 0x4
0x808fad7: JNZ 0x0808faa9
0x808fad9: MOV AL, byte ptr [EBX + 0x4]
0x808fadc: CMP AL, 0x3
0x808fade: JZ 0x0808fccc
0x808fae4: CMP AL, 0x2
0x808fae6: JZ 0x0808faf8
0x808fae8: SUB ESP, 0xc
0x808faeb: PUSH 0x82ad00d
0x808faf0: CALL 0x08097b24
0x808faf5: ADD ESP, 0x10
0x808faf8: MOV EDX, dword ptr [EDI + 0x8]
0x808fafb: MOV EAX, dword ptr [EDX + 0x10]
0x808fafe: AND ESI, 0x1
0x808fb01: MOV dword ptr [EBP + -0x10], EDX
0x808fb04: MOV dword ptr [EBP + -0x18], EAX
0x808fb07: JZ 0x0808fc5d
0x808fb0d: MOV ESI, EAX
0x808fb0f: SUB ESI, dword ptr [EBX + 0x8]
0x808fb12: CALL 0x0809061c
0x808fb17: TEST ESI, ESI
0x808fb19: JS 0x0808fb20
0x808fb1b: CMP ESI, dword ptr [EBP + -0x18]
0x808fb1e: JL 0x0808fb30
0x808fb20: SUB ESP, 0xc
0x808fb23: PUSH 0x82ad027
0x808fb28: CALL 0x08097b24
0x808fb2d: ADD ESP, 0x10
0x808fb30: MOV EDX, dword ptr [EBP + -0x10]
0x808fb33: MOV EAX, dword ptr [EDX + 0xc]
0x808fb36: MOVZX EAX, byte ptr [EAX + ESI*0x1]
0x808fb3a: SUB ESP, 0xc
0x808fb3d: MOV dword ptr [EDI + 0x8], EAX
0x808fb40: MOV byte ptr [EDI + 0x4], 0x2
0x808fb44: PUSH EDX
0x808fb45: CALL 0x0808ad4c
0x808fb4a: MOV AL, byte ptr [EBP + -0x19]
0x808fb4d: MOV byte ptr [EDI + 0x5], AL
0x808fb50: ADD ESP, 0x10
0x808fb53: LEA ESP, [EBP + -0xc]
0x808fb56: POP EBX
0x808fb57: POP ESI
0x808fb58: POP EDI
0x808fb59: LEAVE
0x808fb5a: RET
0x808fb5c: MOV AL, byte ptr [EBX + 0x4]
0x808fb5f: CMP AL, 0x3
0x808fb61: JZ 0x0808fc6a
0x808fb67: CMP AL, 0x2
0x808fb69: JZ 0x0808fb7b
0x808fb6b: SUB ESP, 0xc
0x808fb6e: PUSH 0x82ad00d
0x808fb73: CALL 0x08097b24
0x808fb78: ADD ESP, 0x10
0x808fb7b: MOV EAX, dword ptr [EDI + 0x8]
0x808fb7e: MOV EDX, dword ptr [EAX + 0xc]
0x808fb81: AND ESI, 0x1
0x808fb84: MOV dword ptr [EBP + -0x14], EAX
0x808fb87: MOV dword ptr [EBP + -0x18], EDX
0x808fb8a: JZ 0x0808fbdc
0x808fb8c: MOV ESI, EDX
0x808fb8e: SUB ESI, dword ptr [EBX + 0x8]
0x808fb91: CALL 0x0809061c
0x808fb96: TEST ESI, ESI
0x808fb98: JS 0x0808fb9f
0x808fb9a: CMP ESI, dword ptr [EBP + -0x18]
0x808fb9d: JL 0x0808fbaf
0x808fb9f: SUB ESP, 0xc
0x808fba2: PUSH 0x82ad027
0x808fba7: CALL 0x08097b24
0x808fbac: ADD ESP, 0x10
0x808fbaf: MOV EDX, dword ptr [EBP + -0x14]
0x808fbb2: MOV EAX, dword ptr [EDX + 0x10]
0x808fbb5: MOVZX EAX, byte ptr [EAX + ESI*0x1]
0x808fbb9: SUB ESP, 0xc
0x808fbbc: MOV dword ptr [EDI + 0x8], EAX
0x808fbbf: MOV byte ptr [EDI + 0x4], 0x2
0x808fbc3: PUSH EDX
0x808fbc4: CALL 0x0809a580
0x808fbc9: MOV AL, byte ptr [EBP + -0x19]
0x808fbcc: MOV byte ptr [EDI + 0x5], AL
0x808fbcf: ADD ESP, 0x10
0x808fbd2: LEA ESP, [EBP + -0xc]
0x808fbd5: POP EBX
0x808fbd6: POP ESI
0x808fbd7: POP EDI
0x808fbd8: LEAVE
0x808fbd9: RET
0x808fbdc: MOV ESI, dword ptr [EBX + 0x8]
0x808fbdf: JMP 0x0808fb91
0x808fbe4: MOV AL, byte ptr [EBX + 0x4]
0x808fbe7: CMP AL, 0x3
0x808fbe9: JZ 0x0808fcb5
0x808fbef: CMP AL, 0x2
0x808fbf1: JZ 0x0808fc03
0x808fbf3: SUB ESP, 0xc
0x808fbf6: PUSH 0x82ad00d
0x808fbfb: CALL 0x08097b24
0x808fc00: ADD ESP, 0x10
0x808fc03: AND ESI, 0x1
0x808fc06: JZ 0x0808fc65
0x808fc08: MOV EAX, dword ptr [EDI + 0x8]
0x808fc0b: MOV ESI, dword ptr [EAX + 0x4]
0x808fc0e: SUB ESI, dword ptr [EBX + 0x8]
0x808fc11: TEST ESI, ESI
0x808fc13: JS 0x0808fc1d
0x808fc15: MOV EAX, dword ptr [EDI + 0x8]
0x808fc18: CMP ESI, dword ptr [EAX + 0x4]
0x808fc1b: JC 0x0808fc2d
0x808fc1d: SUB ESP, 0xc
0x808fc20: PUSH 0x82ada78
0x808fc25: CALL 0x08097b24
0x808fc2a: ADD ESP, 0x10
0x808fc2d: CALL 0x0809061c
0x808fc32: MOV EBX, dword ptr [EDI + 0x8]
0x808fc35: SUB ESP, 0x8
0x808fc38: MOV EAX, dword ptr [EBX + 0xc]
0x808fc3b: PUSH dword ptr [EAX + ESI*0x4]
0x808fc3e: PUSH EDI
0x808fc3f: CALL 0x0808be24
0x808fc44: MOV dword ptr [ESP], EBX
0x808fc47: CALL 0x0809b350
0x808fc4c: MOV AL, byte ptr [EBP + -0x19]
0x808fc4f: MOV byte ptr [EDI + 0x5], AL
0x808fc52: ADD ESP, 0x10
0x808fc55: LEA ESP, [EBP + -0xc]
0x808fc58: POP EBX
0x808fc59: POP ESI
0x808fc5a: POP EDI
0x808fc5b: LEAVE
0x808fc5c: RET
0x808fc5d: MOV ESI, dword ptr [EBX + 0x8]
0x808fc60: JMP 0x0808fb12
0x808fc65: MOV ESI, dword ptr [EBX + 0x8]
0x808fc68: JMP 0x0808fc11
0x808fc6a: SUB ESP, 0xc
0x808fc6d: PUSH dword ptr [EBX + 0x8]
0x808fc70: CALL 0x080a36e4
0x808fc75: MOV dword ptr [EBX + 0x8], EAX
0x808fc78: MOV byte ptr [EBX + 0x4], 0x2
0x808fc7c: JMP 0x0808fb78
0x808fc81: SUB ESP, 0x8
0x808fc84: PUSH EBX
0x808fc85: PUSH dword ptr [EDI + 0x8]
0x808fc88: CALL 0x0809d74c
0x808fc8d: ADD ESP, 0x10
0x808fc90: TEST EAX, EAX
0x808fc92: MOV ESI, EAX
0x808fc94: JZ 0x0808fce3
0x808fc96: CALL 0x0809061c
0x808fc9b: SUB ESP, 0x8
0x808fc9e: MOV EBX, dword ptr [EDI + 0x8]
0x808fca1: PUSH ESI
0x808fca2: PUSH EDI
0x808fca3: CALL 0x0808be24
0x808fca8: MOV dword ptr [ESP], EBX
0x808fcab: CALL 0x0809c488
0x808fcb0: JMP 0x0808fac2
0x808fcb5: SUB ESP, 0xc
0x808fcb8: PUSH dword ptr [EBX + 0x8]
0x808fcbb: CALL 0x080a36e4
0x808fcc0: MOV dword ptr [EBX + 0x8], EAX
0x808fcc3: MOV byte ptr [EBX + 0x4], 0x2
0x808fcc7: JMP 0x0808fc00
0x808fccc: SUB ESP, 0xc
0x808fccf: PUSH dword ptr [EBX + 0x8]
0x808fcd2: CALL 0x080a36e4
0x808fcd7: MOV dword ptr [EBX + 0x8], EAX
0x808fcda: MOV byte ptr [EBX + 0x4], 0x2
0x808fcde: JMP 0x0808faf5
0x808fce3: CALL 0x0809061c
0x808fce8: SUB ESP, 0xc
0x808fceb: PUSH dword ptr [EDI + 0x8]
0x808fcee: CALL 0x0809c488
0x808fcf3: MOV byte ptr [EDI + 0x4], 0x0
0x808fcf7: MOV dword ptr [EDI + 0x8], 0x0
0x808fcfe: JMP 0x0808fac2
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_queryMember / procedure_pseudo_code

Evidence: `ev_2aead7613b91a786e731943745c0a2606d930e5db98d60ad0bd59ae2453e87b4`

```c

void vm_queryMember(int flag)

{
  Vm_Value_T *lv;
  uchar uVar1;
  byte bVar2;
  Vm_Value *pBuffer_00;
  Vm_Value *pVVar3;
  Vm_String_T *pVVar4;
  Vm_Mapping_T *m;
  char *pcVar5;
  anon_union_4_11_5000f750_for_v aVar6;
  anon_union_4_11_5000f750_for_v aVar7;
  Vm_Value_T *pValue;
  int iVar8;
  uint uVar9;
  uchar inheritAttrib;
  int len;
  Vm_Buffer_T *pBuffer;
  Vm_String_T *pString;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Value_T * pNewValue@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mapping_T * pMapping@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ioff@[DW_OP_reg6(ESI)] */
  lv = vm_stack + vm_sp;
  uVar1 = lv[1].attrib;
  bVar2 = lv[1].type;
  if (bVar2 == 5) {
    if (lv->type == '\x03') {
      iVar8 = _vm_ftol((lv->v).real);
      (lv->v).number = iVar8;
      lv->type = '\x02';
    }
    else if (lv->type != '\x02') {
      vm_error("Index should be integer.\n");
    }
    pBuffer_00 = lv[1].v.pPtrValue;
    pVVar3 = pBuffer_00->pNext;
    if ((flag & 1U) == 0) {
      iVar8 = (lv->v).number;
    }
    else {
      iVar8 = (int)pVVar3 - (lv->v).number;
    }
    vm_popStack();
    if ((iVar8 < 0) || ((int)pVVar3 <= iVar8)) {
      vm_error("Index out of range.\n");
    }
    aVar7.number._1_3_ = 0;
    aVar7.number._0_1_ = *(byte *)((int)&pBuffer_00->pThis->ref + iVar8);
    lv[1].v = aVar7;
    lv[1].type = '\x02';
    vm_freeBuffer((Vm_Buffer_T *)pBuffer_00);
    lv[1].attrib = uVar1;
    return;
  }
  if (bVar2 < 6) {
    if (bVar2 == 4) {
      if (lv->type == '\x03') {
        iVar8 = _vm_ftol((lv->v).real);
        (lv->v).number = iVar8;
        lv->type = '\x02';
      }
      else if (lv->type != '\x02') {
        vm_error("Index should be integer.\n");
      }
      pVVar4 = lv[1].v.string;
      uVar9 = pVVar4->len;
      if ((flag & 1U) == 0) {
        iVar8 = (lv->v).number;
      }
      else {
        iVar8 = uVar9 - (lv->v).number;
      }
      vm_popStack();
      if ((iVar8 < 0) || ((int)uVar9 <= iVar8)) {
        vm_error("Index out of range.\n");
      }
      aVar6.number._1_3_ = 0;
      aVar6.number._0_1_ = pVVar4->ptr[iVar8];
      lv[1].v = aVar6;
      lv[1].type = '\x02';
      vm_unrefString(pVVar4);
      lv[1].attrib = uVar1;
      return;
    }
  }
  else {
    if (bVar2 == 6) {
      if (lv->type == '\x03') {
        iVar8 = _vm_ftol((lv->v).real);
        (lv->v).number = iVar8;
        lv->type = '\x02';
      }
      else if (lv->type != '\x02') {
        vm_error("Index should be integer.\n");
      }
      if ((flag & 1U) == 0) {
        uVar9 = (lv->v).number;
      }
      else {
        uVar9 = (int)(lv[1].v.pLoopCtrl)->pValue - (lv->v).number;
      }
      if (((int)uVar9 < 0) || ((lv[1].v.string)->attrib <= uVar9)) {
        vm_error("Index of the array out of range.\n");
      }
      vm_popStack();
      pVVar4 = lv[1].v.string;
      vm_copyValue(lv + 1,*(Vm_Value_T **)(pVVar4->ptr + uVar9 * 4));
      vm_unrefArray((Vm_Array_T *)pVVar4);
      lv[1].attrib = uVar1;
      return;
    }
    if (bVar2 == 7) {
      pValue = vm_findInMapping((Vm_Mapping_T *)lv[1].v.string,lv);
      if (pValue == (Vm_Value_T *)0x0) {
        vm_popStack();
        vm_unrefMapping((Vm_Mapping_T *)lv[1].v.string);
        lv[1].type = '\0';
        lv[1].v.info = (void *)0x0;
      }
      else {
        vm_popStack();
        m = (Vm_Mapping_T *)lv[1].v.string;
        vm_copyValue(lv + 1,pValue);
        vm_unrefMapping(m);
      }
      goto LAB_0808fac2;
    }
  }
  pcVar5 = vm_valueType((uint)bVar2);
  vm_error("Bad type(%s) to index.\n",pcVar5);
LAB_0808fac2:
  lv[1].attrib = uVar1;
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

## vm_safeAllocateMapping / procedure_pseudo_code

Evidence: `ev_8c4aee3d415edd9b0de9326ca1fb243b0d95a4cce704099135ea2da2c68628f5`

```c

Vm_Mapping_T * vm_safeAllocateMapping(uint size)

{
  Vm_Mapping_T *p;
  Vm_Mapping_Node_T **ppVVar1;
  Vm_Mapping_T *pVVar2;
  uint uVar3;
  uint uVar4;
  
                    /* Unresolved local var: Vm_Mapping_T * pNewMap@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Mapping_Node_T * * ppa@[???] */
  if (0x300000 < size) {
    size = 0x300000;
  }
  p = pdb_allocateTinyBlock(0x18);
  pVVar2 = (Vm_Mapping_T *)0x0;
  if (p != (Vm_Mapping_T *)0x0) {
    if (size < 9) {
      uVar4 = 8;
      p->tableSize = 7;
    }
    else {
      uVar3 = size | size >> 1;
      uVar3 = uVar3 | uVar3 >> 2;
      uVar3 = uVar3 | uVar3 >> 4;
      uVar3 = uVar3 | uVar3 >> 8;
      uVar3 = uVar3 | uVar3 >> 0x10;
      uVar4 = uVar3 + 1;
      p->tableSize = uVar3;
    }
    p->unfilled = (uVar4 * 0x50) / 100;
    ppVVar1 = pdb_allocateMemory(uVar4 * 4,"VM","/home/cc/xxxx/gameshell/src/vm/vm_mapping.c",0xb1);
    p->table = ppVVar1;
    if (ppVVar1 == (Vm_Mapping_Node_T **)0x0) {
      pdb_freeTinyBlock(p);
      pVVar2 = (Vm_Mapping_T *)0x0;
    }
    else {
      for (uVar3 = uVar4 & 0x3fffffff; uVar3 != 0; uVar3 = uVar3 - 1) {
        *ppVVar1 = (Vm_Mapping_Node_T *)0x0;
        ppVVar1 = ppVVar1 + 1;
      }
      totalMappingSize = uVar4 * 4 + totalMappingSize + 0x18;
      p->ref = 1;
      p->count = 0;
      p->attrib = 1;
      totalMappingNum = totalMappingNum + 1;
      pVVar2 = p;
    }
  }
  return pVVar2;
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

## vm_safeFindForInsert / procedure_pseudo_code

Evidence: `ev_f5dcab63cd814feec7e3bd3ea3639466621466cb34db0c80932a887527db2216`

```c

/* WARNING: Unknown calling convention */

Vm_Value_T * vm_safeFindForInsert(Vm_Mapping_T *m,Vm_Value_T *lv,int doTheFree)

{
  byte bVar1;
  Vm_Buffer *pVVar2;
  Vm_String_T *pVVar3;
  Vm_String_T *pVVar4;
  ushort uVar5;
  Vm_Mapping_Node_T *pVVar6;
  uint uVar7;
  int iVar8;
  Vm_Value_T *pVVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  uchar *puVar13;
  char *pcVar14;
  uchar *puVar15;
  char *pcVar16;
  bool bVar17;
  Vm_Mapping_Node_T *pElt;
  Vm_Mapping_Node_T **ppa;
  
                    /* Unresolved local var: int oi@[???]
                       Unresolved local var: uint i@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Mapping_Node_T * pNewElt@[???] */
  if (lv->type == '\x04') {
    if ((m->attrib & 1) == 0) {
      uVar5 = vm_whashstrc(((lv->v).string)->ptr,0x20);
      uVar11 = (uint)uVar5;
      goto LAB_0809d5fe;
    }
    pVVar3 = (lv->v).string;
    uVar11 = (uint)pVVar3->hash;
    if (pVVar3->hash == 0) {
      uVar5 = vm_whashstr(pVVar3->ptr,0x20);
      pVVar3->hash = uVar5;
      uVar11 = (uint)uVar5;
    }
LAB_0809d435:
    uVar10 = m->tableSize;
    uVar12 = uVar10 & uVar11;
    ppa = m->table + uVar12;
    pElt = *ppa;
    if (pElt != (Vm_Mapping_Node_T *)0x0) {
LAB_0809d464:
      do {
                    /* Unresolved local var: Vm_String_T * s1@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_String_T * s2@[DW_OP_reg2(EDX)] */
        bVar1 = pElt->values[0].type;
        if (bVar1 != lv->type) goto LAB_0809d458;
        if (bVar1 == 3) {
          bVar17 = (lv->v).real == pElt->values[0].v.real;
LAB_0809d4a5:
          if (bVar17) {
LAB_0809d4ac:
            pVVar9 = pElt->values + 1;
            if (doTheFree == 0) {
              return pVVar9;
            }
            vm_cleanupValue(pVVar9);
            return pVVar9;
          }
        }
        else {
          if (bVar1 < 4) {
            if (bVar1 == 2) {
              bVar17 = (lv->v).info == pElt->values[0].v.info;
            }
            else {
LAB_0809d499:
              bVar17 = (lv->v).info == pElt->values[0].v.info;
            }
            goto LAB_0809d4a5;
          }
          if (bVar1 == 4) {
            pVVar3 = (lv->v).string;
            pVVar4 = pElt->values[0].v.string;
            if (pVVar3 == pVVar4) goto LAB_0809d4ac;
            uVar11 = pVVar3->len;
            if (uVar11 == pVVar4->len) {
              if ((m->attrib & 1) == 0) {
                iVar8 = vm_stricmp(pVVar3->ptr,pVVar4->ptr);
                bVar17 = iVar8 == 0;
              }
              else {
                if ((pVVar4->attrib & pVVar3->attrib & 1) != 0) goto LAB_0809d458;
                bVar17 = true;
                pcVar14 = pVVar3->ptr;
                pcVar16 = pVVar4->ptr;
                do {
                  if (uVar11 == 0) break;
                  uVar11 = uVar11 - 1;
                  bVar17 = *pcVar14 == *pcVar16;
                  pcVar14 = pcVar14 + 1;
                  pcVar16 = pcVar16 + 1;
                } while (bVar17);
              }
              goto LAB_0809d4a5;
            }
          }
          else {
            if (bVar1 != 5) goto LAB_0809d499;
                    /* Unresolved local var: uint size@[???] */
            pVVar2 = pElt->values[0].v.buffer;
            uVar11 = pVVar2->realSize;
            if (uVar11 == ((lv->v).buffer)->realSize) {
              bVar17 = true;
              puVar13 = ((lv->v).buffer)->ptr;
              puVar15 = pVVar2->ptr;
              do {
                if (uVar11 == 0) break;
                uVar11 = uVar11 - 1;
                bVar17 = *puVar13 == *puVar15;
                puVar13 = puVar13 + 1;
                puVar15 = puVar15 + 1;
              } while (bVar17);
              goto LAB_0809d4a5;
            }
          }
        }
LAB_0809d458:
        pElt = pElt->pNext;
      } while (pElt != (Vm_Mapping_Node *)0x0);
      pElt = *ppa;
      goto LAB_0809d4d4;
    }
    pElt = (Vm_Mapping_Node_T *)0x0;
  }
  else {
    if (lv->type != '\x05') {
      uVar11 = (lv->v).number >> 4;
      goto LAB_0809d435;
    }
    uVar11 = ((lv->v).buffer)->realSize;
    if (0x20 < uVar11) {
      uVar11 = 0x20;
    }
    uVar11 = vm_whashbuf(((lv->v).buffer)->ptr,uVar11);
LAB_0809d5fe:
    uVar10 = m->tableSize;
    uVar12 = uVar10 & uVar11;
    ppa = m->table + uVar12;
    pElt = *ppa;
    if (pElt != (Vm_Mapping_Node_T *)0x0) goto LAB_0809d464;
  }
  uVar7 = m->unfilled - 1;
  m->unfilled = uVar7;
  if (uVar7 == 0) {
                    /* Unresolved local var: int size@[DW_OP_reg7(EDI)] */
    iVar8 = vm_growMapping(m);
    if (iVar8 == 0) {
      return (Vm_Value_T *)0x0;
    }
    if ((uVar10 + 1 & uVar11) != 0) {
      uVar12 = uVar12 | uVar10 + 1;
    }
    ppa = m->table + uVar12;
    pElt = *ppa;
  }
LAB_0809d4d4:
  uVar11 = m->count + 1;
  if (uVar11 < 0x300001) {
                    /* Unresolved local var: Vm_Mapping_Node_T * ret@[DW_OP_reg6(ESI)] */
    m->count = uVar11;
    pVVar6 = pdb_allocateTinyBlock(0x34);
    pVVar9 = (Vm_Value_T *)0x0;
    if (pVVar6 != (Vm_Mapping_Node_T *)0x0) {
      pVVar9 = pVVar6->values + 1;
      vm_initValue(pVVar6->values);
      vm_initValue(pVVar9);
      vm_copyValue(pVVar6->values,lv);
      *ppa = pVVar6;
      totalMappingSize = totalMappingSize + 0x34;
      pVVar6->pNext = pElt;
      totalMappingNodes = totalMappingNodes + 1;
    }
    return pVVar9;
  }
  return (Vm_Value_T *)0x0;
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

## vm_sizeof / procedure_assembly

Evidence: `ev_477fd7a50d14dfd66b219f1c8b7a918bcb6ca67fa0edeec1e3ff5e4027386ec7`

```asm
0x808f840: PUSH EBP
0x808f841: MOV EBP, ESP
0x808f843: PUSH ESI
0x808f844: PUSH EBX
0x808f845: MOV EAX, [0x086619cc]
0x808f84a: LEA EAX, [EAX + EAX*0x2]
0x808f84d: MOV EDX, dword ptr [0x08661964]
0x808f853: LEA EBX, [EDX + EAX*0x8]
0x808f856: MOVZX EAX, byte ptr [EBX + 0x4]
0x808f85a: CMP EAX, 0x7
0x808f85d: JA 0x0808f868
0x808f85f: JMP dword ptr [EAX*0x4 + 0x82adc7c]
0x808f868: SUB ESP, 0xc
0x808f86b: PUSH EBX
0x808f86c: XOR ESI, ESI
0x808f86e: CALL 0x0808ad98
0x808f873: MOV byte ptr [EBX + 0x4], 0x2
0x808f877: MOV dword ptr [EBX + 0x8], ESI
0x808f87a: ADD ESP, 0x10
0x808f87d: LEA ESP, [EBP + -0x8]
0x808f880: POP EBX
0x808f881: POP ESI
0x808f882: LEAVE
0x808f883: RET
0x808f884: MOV EAX, dword ptr [EBX + 0x8]
0x808f887: SUB ESP, 0xc
0x808f88a: MOV ESI, dword ptr [EAX + 0x10]
0x808f88d: PUSH EAX
0x808f88e: CALL 0x0809c488
0x808f893: JMP 0x0808f873
0x808f898: XOR ESI, ESI
0x808f89a: MOV byte ptr [EBX + 0x4], 0x2
0x808f89e: MOV dword ptr [EBX + 0x8], ESI
0x808f8a1: LEA ESP, [EBP + -0x8]
0x808f8a4: POP EBX
0x808f8a5: POP ESI
0x808f8a6: LEAVE
0x808f8a7: RET
0x808f8a8: MOV EAX, dword ptr [EBX + 0x8]
0x808f8ab: SUB ESP, 0xc
0x808f8ae: MOV ESI, dword ptr [EAX + 0x10]
0x808f8b1: PUSH EAX
0x808f8b2: CALL 0x0808ad4c
0x808f8b7: JMP 0x0808f873
0x808f8bc: MOV EAX, dword ptr [EBX + 0x8]
0x808f8bf: SUB ESP, 0xc
0x808f8c2: MOV ESI, dword ptr [EAX + 0xc]
0x808f8c5: PUSH EAX
0x808f8c6: CALL 0x0809a580
0x808f8cb: JMP 0x0808f873
0x808f8d0: MOV EAX, dword ptr [EBX + 0x8]
0x808f8d3: SUB ESP, 0xc
0x808f8d6: MOV ESI, dword ptr [EAX + 0x4]
0x808f8d9: PUSH EAX
0x808f8da: CALL 0x0809b350
0x808f8df: JMP 0x0808f873
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_sizeof / procedure_pseudo_code

Evidence: `ev_37f82993cfab325271aa168f85e14399a8fc5ce8f9b8df5cedb9e6076907b1f4`

```c

/* WARNING: Unknown calling convention */

void vm_sizeof(int type)

{
  Vm_Value_T *pValue;
  char *pcVar1;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ret@[DW_OP_reg6(ESI)] */
  pValue = vm_stack + vm_sp;
  switch(pValue->type) {
  case '\0':
  case '\x01':
  case '\x02':
  case '\x03':
    pValue->type = '\x02';
    (pValue->v).info = (void *)0x0;
    return;
  case '\x04':
    pcVar1 = (char *)((pValue->v).string)->len;
    vm_unrefString((pValue->v).string);
    break;
  case '\x05':
    pcVar1 = ((pValue->v).string)->ptr;
    vm_freeBuffer((Vm_Buffer_T *)(pValue->v).string);
    break;
  case '\x06':
    pcVar1 = (char *)((pValue->v).string)->attrib;
    vm_unrefArray((Vm_Array_T *)(pValue->v).string);
    break;
  case '\a':
    pcVar1 = (char *)((Vm_Mapping_T *)(pValue->v).string)->count;
    vm_unrefMapping((Vm_Mapping_T *)(pValue->v).string);
    break;
  default:
    pcVar1 = (char *)0x0;
    vm_cleanupValue(pValue);
  }
  pValue->type = '\x02';
  (pValue->v).info = pcVar1;
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
