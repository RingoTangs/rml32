# 继承与重定位的原生静态证据

Ghidra 伪代码不是原始源代码。类型恢复、函数体范围及原始引用的限制见压缩记录。调用引用摘要不替代完整 ReferenceManager 数据。

## 本轮：0x0847aac0 — inspect_native_data_type

Evidence ID：`ev_2f85624cd94d573d180f7a152cebf952c2e43f263ef0edfd0183b1d791ecd0df`

```json
{
  "address": "0x847aac0",
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_object.h/Vm_Program_T *",
  "kind": "pointer",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Program_T *",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_object.h/Vm_Program_T",
  "size_bytes": 4,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：(Consit|onstitut|Hidden|QuickFunction) — search_procedures

Evidence ID：`ev_65ed5636b3bd823f79ae5a97a3f5dadf22691bb8def9024d87ebf3995e409295`

```json
[]
```

## 本轮：(FunctionTable|NewProgram|ProgramFunction|LoadIdentifiers) — search_procedures

Evidence ID：`ev_80eb96d6c5760e03f3718dfdb3644fdd0c5c03a78a4e4caad6c82f81d82d1099`

```json
[
  {
    "address": "0x8086b44",
    "value": "vm_findProgramFunction"
  },
  {
    "address": "0x8086d68",
    "value": "vm_findProgramFunctionByCStr"
  },
  {
    "address": "0x8087fb8",
    "value": "vm_loadIdentifiers"
  },
  {
    "address": "0x80a31bc",
    "value": "vm_gatherProgramFunctionTable"
  }
]
```

## 本轮：(NewFile|createProgram|newProgram) — search_procedures

Evidence ID：`ev_4d7a9201779ba673bbad299d7906ecdfb2892afdc238ee28ee8310c0f0f51586`

```json
[
  {
    "address": "0x8080508",
    "value": "vm_startNewFile"
  },
  {
    "address": "0x8088874",
    "value": "vm_endNewFile"
  },
  {
    "address": "0x80b13f4",
    "value": "vm_createProgram"
  },
  {
    "address": "0x819d200",
    "value": "__log_inmem_newfile"
  },
  {
    "address": "0x81a19e0",
    "value": "__log_newfile"
  }
]
```

## 本轮：vm_gatherProgramFunctionTable — procedure_references

Evidence ID：`ev_509af2c0617a2abcd8e0bef552196b3b30edb61561adb1737a054816b5a1d7cc`

函数入口：`0x80a31bc`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x8088aed",
    "target": "0x80a31bc",
    "caller": "vm_endNewFile",
    "callee": "vm_gatherProgramFunctionTable"
  }
]
```

## 本轮：vm_gatherProgramFunctionTable — procedure_references

Evidence ID：`ev_ddcf4d932d2d34b486bb39494c94b3b2da281cea97eab6a90d1db3be314d3015`

函数入口：`0x80a31bc`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x80a3308",
    "target": "0x804b064",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "<EXTERNAL>::strcmp"
  },
  {
    "source": "0x80a3369",
    "target": "0x804b064",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "<EXTERNAL>::strcmp"
  },
  {
    "source": "0x80a33c8",
    "target": "0x80ff80c",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "yyerror"
  },
  {
    "source": "0x80a33ec",
    "target": "0x80ff80c",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "yyerror"
  },
  {
    "source": "0x80a3458",
    "target": "0x80ff80c",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "yyerror"
  },
  {
    "source": "0x80a347a",
    "target": "0x80ff80c",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "yyerror"
  },
  {
    "source": "0x80a34ec",
    "target": "0x80ff97c",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "yyerrors"
  },
  {
    "source": "0x80a3510",
    "target": "0x8095c88",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "vm_freeFun"
  },
  {
    "source": "0x80a3562",
    "target": "0x80ff97c",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "yyerrors"
  },
  {
    "source": "0x80a3574",
    "target": "0x8095c88",
    "caller": "vm_gatherProgramFunctionTable",
    "callee": "vm_freeFun"
  }
]
```

## 本轮：vm_inheritFunctionPrototype — procedure_references

Evidence ID：`ev_1d4f5e7a7fd3fa71fb6c3ff5173b0360f4491cc3e130729abeca9786fa4aab4d`

函数入口：`0x80a28fc`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x80a292e",
    "target": "0x8081e40",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "vm_lookupIdentInProgram"
  },
  {
    "source": "0x80a29a0",
    "target": "0x80ff80c",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yyerror"
  },
  {
    "source": "0x80a29c0",
    "target": "0x80ff80c",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yyerror"
  },
  {
    "source": "0x80a29e6",
    "target": "0x80ff97c",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yyerrors"
  },
  {
    "source": "0x80a2a4b",
    "target": "0x8085a94",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "vm_findOrAddIdentInProgram"
  },
  {
    "source": "0x80a2abc",
    "target": "0x8085854",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "vm_backupIi"
  },
  {
    "source": "0x80a2b07",
    "target": "0x80ff97c",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yyerrors"
  },
  {
    "source": "0x80a2b1c",
    "target": "0x80ff758",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yystop"
  },
  {
    "source": "0x80a2b30",
    "target": "0x80ff774",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yywarns"
  },
  {
    "source": "0x80a2b47",
    "target": "0x80ff774",
    "caller": "vm_inheritFunctionPrototype",
    "callee": "yywarns"
  }
]
```

## 本轮：vm_overrideFunction — procedure_references

Evidence ID：`ev_8bbd3b32336295312fe983ba72a189972615c7807879ced6552cd6b0022df163`

函数入口：`0x80a21f0`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x80a2234",
    "target": "0x8085854",
    "caller": "vm_overrideFunction",
    "callee": "vm_backupIi"
  },
  {
    "source": "0x80a227d",
    "target": "0x80ff758",
    "caller": "vm_overrideFunction",
    "callee": "yystop"
  },
  {
    "source": "0x80a2292",
    "target": "0x80ff774",
    "caller": "vm_overrideFunction",
    "callee": "yywarns"
  },
  {
    "source": "0x80a22a6",
    "target": "0x80ff774",
    "caller": "vm_overrideFunction",
    "callee": "yywarns"
  }
]
```

## 本轮：vm_pushObjVar — procedure_references

Evidence ID：`ev_0123e989ef3e5ccf0ded341e9a56232ec9ac31a82567f221250b9da9a5498f51`

函数入口：`0x8091224`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x8091278",
    "target": "0x808be24",
    "caller": "vm_pushObjVar",
    "callee": "vm_copyValue"
  },
  {
    "source": "0x8091290",
    "target": "0x8097b24",
    "caller": "vm_pushObjVar",
    "callee": "vm_error"
  }
]
```

## 本轮：vm_pushObjVptr — procedure_references

Evidence ID：`ev_3457f1d49a9057b1984e47233ccdaae7d426a1bba1d6c142de5f9cdbaef906b3`

函数入口：`0x80911a8`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x8091214",
    "target": "0x8097b24",
    "caller": "vm_pushObjVptr",
    "callee": "vm_error"
  }
]
```

## 本轮：vm_pushSharedString — procedure_references

Evidence ID：`ev_6b414fe0bbcd4ccf2e6a84339a529e9cc67cdccd499fa921fddb5ea6ca99bc68`

函数入口：`0x8090cf8`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x8090d28",
    "target": "0x8097b24",
    "caller": "vm_pushSharedString",
    "callee": "vm_error"
  },
  {
    "source": "0x8090d78",
    "target": "0x8097b24",
    "caller": "vm_pushSharedString",
    "callee": "vm_error"
  }
]
```

## 本轮：vm_setObjVar — procedure_references

Evidence ID：`ev_2bccf3ddc39d8440eda9b6dc53284ec5f3daef7f0c15bfbab5e8d0c8ed84cc84`

函数入口：`0x808f7f8`；以下仅列调用，其他引用保存在原始记录中。

```json
[
  {
    "source": "0x808f81c",
    "target": "0x808ad98",
    "caller": "vm_setObjVar",
    "callee": "vm_cleanupValue"
  },
  {
    "source": "0x808f836",
    "target": "0x808be24",
    "caller": "vm_setObjVar",
    "callee": "vm_copyValue"
  }
]
```

## 本轮：vm_buildProgramHead — procedure_assembly

Evidence ID：`ev_ea5a86b9ba60013f7c3c493199e44963814f92ef6614d449a2e2de651aea591a`

```asm
0x808651c: PUSH EBP
0x808651d: MOV EBP, ESP
0x808651f: PUSH EDI
0x8086520: PUSH EBX
0x8086521: SUB ESP, 0x10
0x8086524: PUSH 0x158d
0x8086529: PUSH 0x82ab14c
0x808652e: PUSH 0x82a376b
0x8086533: PUSH 0x64
0x8086535: CALL 0x080ce784
0x808653a: MOV EBX, EAX
0x808653c: ADD ESP, 0x10
0x808653f: XOR EAX, EAX
0x8086541: TEST EBX, EBX
0x8086543: JZ 0x080865a8
0x8086545: PUSH EAX
0x8086546: PUSH 0x64
0x8086548: PUSH 0x0
0x808654a: PUSH EBX
0x808654b: CALL 0x0804ba54
0x8086550: POP ECX
0x8086551: POP EDI
0x8086552: LEA EAX, [EBX + 0x4]
0x8086555: PUSH EAX
0x8086556: PUSH dword ptr [EBP + 0x8]
0x8086559: CALL 0x0807f5b4
0x808655e: ADD ESP, 0x10
0x8086561: TEST EAX, EAX
0x8086563: JZ 0x08086666
0x8086569: PUSH 0x15a5
0x808656e: PUSH 0x82ab14c
0x8086573: PUSH 0x82a376b
0x8086578: PUSH 0x2000
0x808657d: CALL 0x080ce784
0x8086582: ADD ESP, 0x10
0x8086585: TEST EAX, EAX
0x8086587: MOV dword ptr [EBX + 0x54], EAX
0x808658a: JNZ 0x080865b0
0x808658c: PUSH 0x15d1
0x8086591: PUSH 0x82ab14c
0x8086596: PUSH 0x82a376b
0x808659b: PUSH EBX
0x808659c: CALL 0x080ceb94
0x80865a1: XOR EBX, EBX
0x80865a3: ADD ESP, 0x10
0x80865a6: MOV EAX, EBX
0x80865a8: LEA ESP, [EBP + -0x8]
0x80865ab: POP EBX
0x80865ac: POP EDI
0x80865ad: LEAVE
0x80865ae: RET
0x80865b0: PUSH EDX
0x80865b1: PUSH 0x2000
0x80865b6: PUSH 0x0
0x80865b8: PUSH EAX
0x80865b9: CALL 0x0804ba54
0x80865be: MOV EAX, dword ptr [EBX + 0x54]
0x80865c1: ADD EAX, 0x1000
0x80865c6: MOV dword ptr [EBX + 0x58], EAX
0x80865c9: MOV word ptr [EBX + 0x12], 0x4
0x80865cf: PUSH 0x15b2
0x80865d4: PUSH 0x82ab14c
0x80865d9: PUSH 0x82a376b
0x80865de: MOVSX EAX, word ptr [EBX + 0x12]
0x80865e2: LEA EAX, [EAX + EAX*0x2]
0x80865e5: SHL EAX, 0x2
0x80865e8: PUSH EAX
0x80865e9: CALL 0x080ce784
0x80865ee: ADD ESP, 0x20
0x80865f1: MOV dword ptr [EBX + 0x18], EAX
0x80865f4: PUSH 0x15b4
0x80865f9: PUSH 0x82ab14c
0x80865fe: PUSH 0x82a376b
0x8086603: MOVSX EAX, word ptr [EBX + 0x12]
0x8086607: SHL EAX, 0x1
0x8086609: PUSH EAX
0x808660a: CALL 0x080ce784
0x808660f: MOV EDI, dword ptr [EBX + 0x18]
0x8086612: ADD ESP, 0x10
0x8086615: TEST EDI, EDI
0x8086617: MOV dword ptr [EBX + 0x1c], EAX
0x808661a: JZ 0x0808663c
0x808661c: TEST EAX, EAX
0x808661e: JZ 0x0808663c
0x8086620: XOR EAX, EAX
0x8086622: CLD
0x8086623: MOV ECX, 0x3
0x8086628: MOV word ptr [EBX + 0x14], 0x1
0x808662e: STOSD.REP ES:EDI
0x8086630: MOV EAX, dword ptr [EBX + 0x18]
0x8086633: MOV dword ptr [EAX], EBX
0x8086635: JMP 0x080865a6
0x808663c: MOV EDI, dword ptr [EBX + 0x54]
0x808663f: TEST EDI, EDI
0x8086641: JZ 0x0808658c
0x8086647: PUSH 0x15cc
0x808664c: PUSH 0x82ab14c
0x8086651: PUSH 0x82a376b
0x8086656: PUSH dword ptr [EBX + 0x54]
0x8086659: CALL 0x080ceb94
0x808665e: ADD ESP, 0x10
0x8086661: JMP 0x0808658c
0x8086666: SUB ESP, 0xc
0x8086669: PUSH EBX
0x808666a: CALL 0x080862a8
0x808666f: XOR EAX, EAX
0x8086671: JMP 0x080865a8
```

## 本轮：vm_buildProgramHead — procedure_pseudo_code

Evidence ID：`ev_aa12d41f4394aa209415defcd72e5fc2cfbdee8acf9ff95fe8d33af97ed6b264`

```c

/* WARNING: Unknown calling convention */

Vm_Program * vm_buildProgramHead(char *pProgramName)

{
  Vm_Program_T *pProgram;
  char *pcVar1;
  Vm_Ident_Hash_Elem **__s;
  Vm_Program_Consititution_T *pVVar2;
  Vm_Index_T *pVVar3;
  int iVar4;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ret@[DW_OP_reg0(EAX)] */
  pProgram = pdb_allocateMemory(100,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x158d);
  if (pProgram != (Vm_Program_T *)0x0) {
    memset(pProgram,0,100);
    pcVar1 = vm_addFileName(pProgramName,&pProgram->strName);
    if (pcVar1 == (char *)0x0) {
      vm_destructProgramHead(pProgram);
    }
    else {
      __s = pdb_allocateMemory(0x2000,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15a5);
      pProgram->ppHashTable = __s;
      if (__s != (Vm_Ident_Hash_Elem **)0x0) {
        memset(__s,0,0x2000);
        pProgram->ppHashHead = pProgram->ppHashTable + 0x400;
        pProgram->consititutionBufferSize = 4;
        pVVar2 = pdb_allocateMemory(pProgram->consititutionBufferSize * 0xc,"VM",
                                    "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15b2);
        pProgram->pConsititutions = pVVar2;
        pVVar3 = pdb_allocateMemory((int)pProgram->consititutionBufferSize << 1,"VM",
                                    "/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15b4);
        pProgram->initOrderList = pVVar3;
        if ((pProgram->pConsititutions != (Vm_Program_Consititution_T *)0x0) &&
           (pVVar3 != (Vm_Index_T *)0x0)) {
          pProgram->maxConsititutionNo = 1;
          pVVar2 = pProgram->pConsititutions;
          for (iVar4 = 3; iVar4 != 0; iVar4 = iVar4 + -1) {
            pVVar2->pProgram = (Vm_Program *)0x0;
            pVVar2 = (Vm_Program_Consititution_T *)&pVVar2->attrib;
          }
          pProgram->pConsititutions->pProgram = pProgram;
          return pProgram;
        }
        if (pProgram->ppHashTable != (Vm_Ident_Hash_Elem **)0x0) {
          pdb_freeMemory(pProgram->ppHashTable,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15cc
                        );
        }
      }
      pdb_freeMemory(pProgram,"VM","/home/cc/xxxx/gameshell/src/vm/vm_lex.c",0x15d1);
    }
  }
  return (Vm_Program *)0x0;
}
```

## 本轮：vm_createProgram — procedure_assembly

Evidence ID：`ev_7c339189f2227b0560db98446dc83bd0520ad53bb7da51083b39a547badba203`

```asm
0x80b13f4: PUSH EBP
0x80b13f5: MOV EBP, ESP
0x80b13f7: PUSH EDI
0x80b13f8: PUSH ESI
0x80b13f9: PUSH EBX
0x80b13fa: SUB ESP, 0x18
0x80b13fd: MOV EDI, dword ptr [EBP + 0x8]
0x80b1400: MOV dword ptr [EBP + -0x10], 0x0
0x80b1407: PUSH EAX
0x80b1408: MOV EBX, EDX
0x80b140a: CALL 0x0808651c
0x80b140f: ADD ESP, 0x10
0x80b1412: TEST EAX, EAX
0x80b1414: MOV ESI, EAX
0x80b1416: JZ 0x080b144b
0x80b1418: LEA EAX, [EBP + -0x10]
0x80b141b: PUSH EAX
0x80b141c: PUSH ESI
0x80b141d: PUSH EDI
0x80b141e: PUSH EBX
0x80b141f: CALL 0x080985e0
0x80b1424: ADD ESP, 0x10
0x80b1427: TEST EAX, EAX
0x80b1429: JZ 0x080b1464
0x80b142b: PUSH EDX
0x80b142c: PUSH EAX
0x80b142d: PUSH EDI
0x80b142e: PUSH 0x82b0e7c
0x80b1433: CALL 0x08097acc
0x80b1438: ADD ESP, 0x10
0x80b143b: TEST ESI, ESI
0x80b143d: JZ 0x080b144b
0x80b143f: SUB ESP, 0xc
0x80b1442: PUSH ESI
0x80b1443: CALL 0x080862a8
0x80b1448: ADD ESP, 0x10
0x80b144b: MOV EAX, dword ptr [EBP + -0x10]
0x80b144e: XOR ESI, ESI
0x80b1450: TEST EAX, EAX
0x80b1452: JNZ 0x080b14e0
0x80b1458: LEA ESP, [EBP + -0xc]
0x80b145b: POP EBX
0x80b145c: MOV EAX, ESI
0x80b145e: POP ESI
0x80b145f: POP EDI
0x80b1460: LEAVE
0x80b1461: RET
0x80b1464: LEA EAX, [ESI + 0xa]
0x80b1467: PUSH EAX
0x80b1468: PUSH 0x1
0x80b146a: PUSH 0x82d0dd9
0x80b146f: PUSH ESI
0x80b1470: CALL 0x08086d68
0x80b1475: ADD ESP, 0x10
0x80b1478: TEST EAX, EAX
0x80b147a: MOV EBX, EAX
0x80b147c: JZ 0x080b151a
0x80b1482: MOV AX, word ptr [EAX + 0x8]
0x80b1486: OR EAX, 0x1
0x80b1489: TEST AL, 0x4
0x80b148b: MOV word ptr [EBX + 0x8], AX
0x80b148f: JZ 0x080b14f6
0x80b1491: MOV EDI, dword ptr [EBX + 0x14]
0x80b1494: TEST EDI, EDI
0x80b1496: JZ 0x080b150d
0x80b1498: LEA EAX, [ESI + 0xc]
0x80b149b: PUSH EAX
0x80b149c: PUSH 0x1
0x80b149e: PUSH 0x82b0fff
0x80b14a3: PUSH ESI
0x80b14a4: CALL 0x08086d68
0x80b14a9: ADD ESP, 0x10
0x80b14ac: TEST EAX, EAX
0x80b14ae: MOV EBX, EAX
0x80b14b0: JZ 0x080b1549
0x80b14b6: TEST byte ptr [EAX + 0x8], 0x4
0x80b14ba: JZ 0x080b1525
0x80b14bc: MOV EBX, dword ptr [EBX + 0x14]
0x80b14bf: TEST EBX, EBX
0x80b14c1: JZ 0x080b153c
0x80b14c3: SUB ESP, 0xc
0x80b14c6: MOV dword ptr [ESI], 0x0
0x80b14cc: PUSH ESI
0x80b14cd: CALL 0x080c3f1c
0x80b14d2: MOV EAX, dword ptr [EBP + -0x10]
0x80b14d5: ADD ESP, 0x10
0x80b14d8: TEST EAX, EAX
0x80b14da: JZ 0x080b1458
0x80b14e0: SUB ESP, 0xc
0x80b14e3: PUSH EAX
0x80b14e4: CALL 0x08089094
0x80b14e9: ADD ESP, 0x10
0x80b14ec: LEA ESP, [EBP + -0xc]
0x80b14ef: POP EBX
0x80b14f0: MOV EAX, ESI
0x80b14f2: POP ESI
0x80b14f3: POP EDI
0x80b14f4: LEAVE
0x80b14f5: RET
0x80b14f6: SUB ESP, 0xc
0x80b14f9: PUSH 0x82b0ea4
0x80b14fe: CALL 0x08097e34
0x80b1503: OR word ptr [EBX + 0x8], 0x4
0x80b1508: ADD ESP, 0x10
0x80b150b: JMP 0x080b1491
0x80b150d: SUB ESP, 0xc
0x80b1510: PUSH 0x82b0eec
0x80b1515: JMP 0x080b1433
0x80b151a: MOV word ptr [ESI + 0xa], 0xffff
0x80b1520: JMP 0x080b1498
0x80b1525: SUB ESP, 0xc
0x80b1528: PUSH 0x82b0f20
0x80b152d: CALL 0x08097e34
0x80b1532: OR word ptr [EBX + 0x8], 0x4
0x80b1537: ADD ESP, 0x10
0x80b153a: JMP 0x080b14bc
0x80b153c: SUB ESP, 0xc
0x80b153f: PUSH 0x82b0f68
0x80b1544: JMP 0x080b1433
0x80b1549: MOV word ptr [ESI + 0xc], 0xffff
0x80b154f: JMP 0x080b14c3
```

## 本轮：vm_createProgram — procedure_pseudo_code

Evidence ID：`ev_88e6cc1c7a8abb344478148606a768df6d37a6acd2967f3aee09d0d20d04e62d`

```c

Vm_Program_T * __regparm2 vm_createProgram(char *pProgramName,int fileFd,char *pInFileName)

{
  Vm_Node_T **ppVVar1;
  ushort uVar2;
  Vm_Program *pProgram;
  Vm_Status VVar3;
  Vm_Fun *pVVar4;
  Vm_Program_T *pProgram_00;
  Vm_Program_T *pVVar5;
  Vm_Program_T *extraout_EAX;
  Vm_Index_T *extraout_EDX;
  char *str;
  Vm_Index_T *pFunOffset;
  Vm_Scratchpad_Info_T *pScratchpad;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg3(EBX)]
                       Unresolved local var: int ret@[???]
                       Unresolved local var: int errCode@[DW_OP_reg0(EAX)] */
  pScratchpad = (Vm_Scratchpad_Info_T *)0x0;
  pProgram = vm_buildProgramHead(pProgramName);
  if (pProgram != (Vm_Program *)0x0) {
    VVar3 = vm_compile(fileFd,pInFileName,pProgram,&pScratchpad);
    if (VVar3 == VM_OK) {
      pFunOffset = &pProgram->createFunOffset;
      VVar3 = ~VM_CAN_NOT_CREATE_MUTEX;
      pInFileName = "create";
      pVVar4 = vm_findProgramFunctionByCStr(pProgram,"create",1,pFunOffset);
      if (pVVar4 == (Vm_Fun *)0x0) {
        pProgram->createFunOffset = -1;
      }
      else {
        uVar2 = pVVar4->attrib;
        pVVar4->attrib = uVar2 | 1;
        if ((uVar2 & 4) == 0) {
          vm_stdprintf("Warning: Construction function \"create\" must be a private function.\n");
          pVVar4->attrib = pVVar4->attrib | 4;
        }
        if ((pVVar4->codeSegment).nodes == (Vm_Node_T *)0x0) {
          str = "Error: Construction function is not implemented.\n";
          goto LAB_080b1433;
        }
      }
      pFunOffset = &pProgram->destructFunOffset;
      VVar3 = ~VM_CAN_NOT_CREATE_MUTEX;
      pInFileName = "destruct";
      pProgram_00 = (Vm_Program_T *)vm_findProgramFunctionByCStr(pProgram,"destruct",1,pFunOffset);
      if (pProgram_00 == (Vm_Program_T *)0x0) {
        pProgram->destructFunOffset = -1;
      }
      else {
        pVVar5 = pProgram_00;
        if ((*(short *)&pProgram_00->attrib & 4) == 0) {
          vm_stdprintf("Warning: destruction function \"destruct\" must be a private function.\n");
          *(ushort *)&pProgram_00->attrib = *(ushort *)&pProgram_00->attrib | 4;
          pVVar5 = extraout_EAX;
        }
        ppVVar1 = (Vm_Node_T **)&pProgram_00->maxConsititutionNo;
        pProgram_00 = pVVar5;
        if (*ppVVar1 == (Vm_Node_T *)0x0) {
          str = "Error: Destruction function is not implemented.\n";
          goto LAB_080b1433;
        }
      }
      pProgram->ref = 0;
      vm_applyBreakpointsOnProgram(pProgram_00);
      goto joined_r0x080b14da;
    }
    str = "Failed to compile \"%s\", errCode = %d.\n";
    pFunOffset = extraout_EDX;
LAB_080b1433:
    vm_errprintf(str,pInFileName,VVar3,pFunOffset);
    if (pProgram != (Vm_Program *)0x0) {
      vm_destructProgramHead(pProgram);
    }
  }
  pProgram = (Vm_Program *)0x0;
joined_r0x080b14da:
  if (pScratchpad == (Vm_Scratchpad_Info_T *)0x0) {
    return pProgram;
  }
  vm_destructScratch(pScratchpad);
  return pProgram;
}
```

## 本轮：vm_gatherProgramFunctionTable — procedure_assembly

Evidence ID：`ev_2b87bd494b66569995a2fd20b9c75f34d7fbc63d63d1b161f3aa5e6ed6113c16`

```asm
0x80a31bc: PUSH EBP
0x80a31bd: MOV EBP, ESP
0x80a31bf: PUSH EDI
0x80a31c0: PUSH ESI
0x80a31c1: PUSH EBX
0x80a31c2: SUB ESP, 0x2c
0x80a31c5: MOV EAX, dword ptr [EBP + 0x8]
0x80a31c8: MOV CX, word ptr [EAX + 0x14]
0x80a31cc: CMP CX, 0x1
0x80a31d0: MOV word ptr [EBP + -0xe], 0x1
0x80a31d6: JMP 0x080a31e0
0x80a31d8: INC word ptr [EBP + -0xe]
0x80a31dc: CMP CX, word ptr [EBP + -0xe]
0x80a31e0: JLE 0x080a3255
0x80a31e2: MOVSX EAX, word ptr [EBP + -0xe]
0x80a31e6: MOV EBX, dword ptr [EBP + 0x8]
0x80a31e9: LEA EAX, [EAX + EAX*0x2]
0x80a31ec: MOV EDX, dword ptr [EBX + 0x18]
0x80a31ef: LEA EAX, [EDX + EAX*0x4]
0x80a31f2: MOV dword ptr [EBP + -0x30], EAX
0x80a31f5: TEST byte ptr [EAX + 0x4], 0x1
0x80a31f9: JZ 0x080a31d8
0x80a31fb: MOV EBX, dword ptr [EAX]
0x80a31fd: XOR EDI, EDI
0x80a31ff: CMP word ptr [EBX + 0x10], 0x0
0x80a3204: JLE 0x080a31d8
0x80a3206: MOV ESI, ESI
0x80a3208: MOV ESI, dword ptr [EBP + -0x30]
0x80a320b: MOVSX ECX, DI
0x80a320e: MOVSX EAX, word ptr [ESI + 0x6]
0x80a3212: LEA EAX, [ECX + EAX*0x1]
0x80a3215: MOV dword ptr [EBP + -0x28], EAX
0x80a3218: MOV EAX, dword ptr [EBP + 0x8]
0x80a321b: MOV EDX, dword ptr [EBP + -0x28]
0x80a321e: MOV EAX, dword ptr [EAX + 0x2c]
0x80a3221: MOV dword ptr [EBP + -0x2c], EAX
0x80a3224: LEA EAX, [EAX + EDX*0x8]
0x80a3227: MOV EDX, dword ptr [EBX + 0x2c]
0x80a322a: LEA ECX, [EDX + ECX*0x8]
0x80a322d: MOV DX, word ptr [EBP + -0xe]
0x80a3231: ADD DX, word ptr [ECX + 0x4]
0x80a3235: MOV word ptr [EAX + 0x4], DX
0x80a3239: MOV EDX, dword ptr [ECX]
0x80a323b: MOV dword ptr [EAX], EDX
0x80a323d: MOV DL, byte ptr [ECX + 0x6]
0x80a3240: OR byte ptr [EAX + 0x6], DL
0x80a3243: INC EDI
0x80a3244: MOV EBX, dword ptr [ESI]
0x80a3246: CMP word ptr [EBX + 0x10], DI
0x80a324a: JG 0x080a3208
0x80a324c: MOV EBX, dword ptr [EBP + 0x8]
0x80a324f: MOV CX, word ptr [EBX + 0x14]
0x80a3253: JMP 0x080a31d8
0x80a3255: MOV ESI, dword ptr [EBP + 0x8]
0x80a3258: MOV EAX, dword ptr [ESI + 0x1c]
0x80a325b: MOVSX EDX, CX
0x80a325e: MOV word ptr [EAX + EDX*0x2 + -0x2], 0x0
0x80a3265: MOV AX, word ptr [ESI + 0x14]
0x80a3269: CMP AX, 0x1
0x80a326d: JLE 0x080a3289
0x80a326f: MOV EDX, dword ptr [ESI + 0x18]
0x80a3272: DEC EAX
0x80a3273: ADD EDX, 0xc
0x80a3276: MOV word ptr [EBP + -0xe], AX
0x80a327a: MOV ESI, ESI
0x80a327c: MOV EAX, dword ptr [EDX]
0x80a327e: INC dword ptr [EAX]
0x80a3280: ADD EDX, 0xc
0x80a3283: DEC word ptr [EBP + -0xe]
0x80a3287: JNZ 0x080a327c
0x80a3289: MOV EAX, dword ptr [EBP + 0x8]
0x80a328c: MOV SI, word ptr [EAX + 0x10]
0x80a3290: OR byte ptr [EAX + 0x8], 0x1
0x80a3294: CMP SI, 0x0
0x80a3298: MOV word ptr [EBP + -0x10], 0x0
0x80a329e: JMP 0x080a32a8
0x80a32a0: INC word ptr [EBP + -0x10]
0x80a32a4: CMP SI, word ptr [EBP + -0x10]
0x80a32a8: JLE 0x080a3487
0x80a32ae: MOV ECX, dword ptr [EBP + 0x8]
0x80a32b1: MOVSX EAX, word ptr [EBP + -0x10]
0x80a32b5: MOV EDX, dword ptr [ECX + 0x2c]
0x80a32b8: LEA EAX, [EDX + EAX*0x8]
0x80a32bb: MOV dword ptr [EBP + -0x14], EAX
0x80a32be: MOV EAX, dword ptr [EAX]
0x80a32c0: TEST EAX, EAX
0x80a32c2: JZ 0x080a32a0
0x80a32c4: MOV EAX, dword ptr [EAX + 0x14]
0x80a32c7: TEST EAX, EAX
0x80a32c9: JNZ 0x080a32a0
0x80a32cb: MOV EBX, dword ptr [EBP + -0x10]
0x80a32ce: DEC BX
0x80a32d0: JNS 0x080a32da
0x80a32d2: JMP 0x080a3323
0x80a32d4: MOV ESI, dword ptr [EBP + 0x8]
0x80a32d7: MOV EDX, dword ptr [ESI + 0x2c]
0x80a32da: MOVSX EAX, BX
0x80a32dd: LEA EDI, [EDX + EAX*0x8]
0x80a32e0: MOV ESI, dword ptr [EDI]
0x80a32e2: TEST ESI, ESI
0x80a32e4: JZ 0x080a3318
0x80a32e6: MOV EAX, dword ptr [ESI + 0x14]
0x80a32e9: TEST EAX, EAX
0x80a32eb: JZ 0x080a3318
0x80a32ed: TEST byte ptr [ESI + 0x8], 0x4
0x80a32f1: JNZ 0x080a3318
0x80a32f3: TEST byte ptr [EDI + 0x6], 0x1
0x80a32f7: JNZ 0x080a3318
0x80a32f9: MOV EAX, dword ptr [EBP + -0x14]
0x80a32fc: SUB ESP, 0x8
0x80a32ff: MOV EAX, dword ptr [EAX]
0x80a3301: PUSH dword ptr [EAX]
0x80a3303: PUSH dword ptr [ESI]
0x80a3305: MOV dword ptr [EBP + -0x18], EAX
0x80a3308: CALL 0x0804b064
0x80a330d: ADD ESP, 0x10
0x80a3310: TEST EAX, EAX
0x80a3312: JZ 0x080a3400
0x80a3318: DEC BX
0x80a331a: JNS 0x080a32d4
0x80a331c: MOV EDX, dword ptr [EBP + 0x8]
0x80a331f: MOV SI, word ptr [EDX + 0x10]
0x80a3323: MOV EBX, dword ptr [EBP + -0x10]
0x80a3326: MOV ESI, ESI
0x80a3328: INC EBX
0x80a3329: CMP SI, BX
0x80a332c: JLE 0x080a32a0
0x80a3332: MOV ECX, dword ptr [EBP + 0x8]
0x80a3335: MOVSX EDX, BX
0x80a3338: MOV EAX, dword ptr [ECX + 0x2c]
0x80a333b: LEA EDX, [EAX + EDX*0x8]
0x80a333e: MOV EDI, dword ptr [EDX]
0x80a3340: TEST EDI, EDI
0x80a3342: MOV dword ptr [EBP + -0x20], EDX
0x80a3345: JZ 0x080a3328
0x80a3347: MOV EAX, dword ptr [EDI + 0x14]
0x80a334a: TEST EAX, EAX
0x80a334c: JZ 0x080a3328
0x80a334e: TEST byte ptr [EDI + 0x8], 0x4
0x80a3352: JNZ 0x080a3328
0x80a3354: TEST byte ptr [EDX + 0x6], 0x1
0x80a3358: JNZ 0x080a3328
0x80a335a: MOV EAX, dword ptr [EBP + -0x14]
0x80a335d: SUB ESP, 0x8
0x80a3360: MOV EAX, dword ptr [EAX]
0x80a3362: PUSH dword ptr [EAX]
0x80a3364: PUSH dword ptr [EDI]
0x80a3366: MOV dword ptr [EBP + -0x24], EAX
0x80a3369: CALL 0x0804b064
0x80a336e: ADD ESP, 0x10
0x80a3371: TEST EAX, EAX
0x80a3373: JNZ 0x080a3328
0x80a3375: MOV EAX, dword ptr [EBP + -0x24]
0x80a3378: XOR ESI, ESI
0x80a337a: MOV ECX, dword ptr [EDI + 0x30]
0x80a337d: MOV EDX, dword ptr [EAX + 0x30]
0x80a3380: JMP 0x080a33b8
0x80a3384: TEST EDX, EDX
0x80a3386: JZ 0x080a33c0
0x80a3388: MOV AL, byte ptr [EDX + 0x5]
0x80a338b: CMP byte ptr [ECX + 0x5], AL
0x80a338e: JNZ 0x080a3547
0x80a3394: MOV AL, byte ptr [EDX + 0x6]
0x80a3397: CMP byte ptr [ECX + 0x6], AL
0x80a339a: JNZ 0x080a353a
0x80a33a0: MOV EAX, dword ptr [ECX + 0xc]
0x80a33a3: TEST EAX, EAX
0x80a33a5: JZ 0x080a33b2
0x80a33a7: MOV EAX, dword ptr [EDX + 0xc]
0x80a33aa: TEST EAX, EAX
0x80a33ac: JNZ 0x080a3554
0x80a33b2: MOV ECX, dword ptr [ECX + 0x10]
0x80a33b5: MOV EDX, dword ptr [EDX + 0x10]
0x80a33b8: TEST ECX, ECX
0x80a33ba: JNZ 0x080a3384
0x80a33bc: TEST EDX, EDX
0x80a33be: JZ 0x080a33d5
0x80a33c0: SUB ESP, 0xc
0x80a33c3: PUSH 0x82afaac
0x80a33c8: CALL 0x080ff80c
0x80a33cd: ADD ESP, 0x10
0x80a33d0: MOV ESI, 0x1
0x80a33d5: MOV AL, byte ptr [EDI + 0xd]
0x80a33d8: MOV EDX, dword ptr [EBP + -0x24]
0x80a33db: CMP byte ptr [EDX + 0xd], AL
0x80a33de: JZ 0x080a34f6
0x80a33e4: SUB ESP, 0xc
0x80a33e7: PUSH 0x82b0063
0x80a33ec: CALL 0x080ff80c
0x80a33f1: ADD ESP, 0x10
0x80a33f4: MOV ECX, dword ptr [EBP + 0x8]
0x80a33f7: MOV SI, word ptr [ECX + 0x10]
0x80a33fb: JMP 0x080a3328
0x80a3400: MOV EAX, dword ptr [EBP + -0x18]
0x80a3403: MOV ECX, dword ptr [ESI + 0x30]
0x80a3406: MOV EDX, dword ptr [EAX + 0x30]
0x80a3409: MOV dword ptr [EBP + -0x1c], 0x0
0x80a3410: JMP 0x080a3448
0x80a3414: TEST EDX, EDX
0x80a3416: JZ 0x080a3450
0x80a3418: MOV AL, byte ptr [EDX + 0x5]
0x80a341b: CMP byte ptr [ECX + 0x5], AL
0x80a341e: JNZ 0x080a34c7
0x80a3424: MOV AL, byte ptr [EDX + 0x6]
0x80a3427: CMP byte ptr [ECX + 0x6], AL
0x80a342a: JNZ 0x080a34d1
0x80a3430: MOV EAX, dword ptr [ECX + 0xc]
0x80a3433: TEST EAX, EAX
0x80a3435: JZ 0x080a3442
0x80a3437: MOV EAX, dword ptr [EDX + 0xc]
0x80a343a: TEST EAX, EAX
0x80a343c: JNZ 0x080a34de
0x80a3442: MOV ECX, dword ptr [ECX + 0x10]
0x80a3445: MOV EDX, dword ptr [EDX + 0x10]
0x80a3448: TEST ECX, ECX
0x80a344a: JNZ 0x080a3414
0x80a344c: TEST EDX, EDX
0x80a344e: JZ 0x080a3467
0x80a3450: SUB ESP, 0xc
0x80a3453: PUSH 0x82afaac
0x80a3458: CALL 0x080ff80c
0x80a345d: ADD ESP, 0x10
0x80a3460: MOV dword ptr [EBP + -0x1c], 0x1
0x80a3467: MOV AL, byte ptr [ESI + 0xd]
0x80a346a: MOV EDX, dword ptr [EBP + -0x18]
0x80a346d: CMP byte ptr [EDX + 0xd], AL
0x80a3470: JZ 0x080a348f
0x80a3472: SUB ESP, 0xc
0x80a3475: PUSH 0x82b0063
0x80a347a: CALL 0x080ff80c
0x80a347f: ADD ESP, 0x10
0x80a3482: JMP 0x080a3318
0x80a3487: LEA ESP, [EBP + -0xc]
0x80a348a: POP EBX
0x80a348b: POP ESI
0x80a348c: POP EDI
0x80a348d: LEAVE
0x80a348e: RET
0x80a348f: MOV EAX, dword ptr [EBP + -0x1c]
0x80a3492: TEST EAX, EAX
0x80a3494: JNZ 0x080a3318
0x80a349a: MOV ECX, dword ptr [EBP + -0x14]
0x80a349d: CMP word ptr [ECX + 0x4], 0x0
0x80a34a2: JZ 0x080a356c
0x80a34a8: MOV EBX, dword ptr [EBP + -0x14]
0x80a34ab: MOV EAX, dword ptr [EDI]
0x80a34ad: OR byte ptr [EBX + 0x6], 0x1
0x80a34b1: MOV dword ptr [EBX], EAX
0x80a34b3: MOV AX, word ptr [EDI + 0x4]
0x80a34b7: MOV word ptr [EBX + 0x4], AX
0x80a34bb: MOV EAX, dword ptr [EBP + 0x8]
0x80a34be: MOV SI, word ptr [EAX + 0x10]
0x80a34c2: JMP 0x080a32a0
0x80a34c7: SUB ESP, 0xc
0x80a34ca: PUSH 0x82b007e
0x80a34cf: JMP 0x080a3458
0x80a34d1: SUB ESP, 0xc
0x80a34d4: PUSH 0x82afacc
0x80a34d9: JMP 0x080a3458
0x80a34de: MOVSX EAX, word ptr [ECX + 0x8]
0x80a34e2: SUB ESP, 0x8
0x80a34e5: INC EAX
0x80a34e6: PUSH EAX
0x80a34e7: PUSH 0x82afaf0
0x80a34ec: CALL 0x080ff97c
0x80a34f1: JMP 0x080a345d
0x80a34f6: TEST ESI, ESI
0x80a34f8: JNZ 0x080a33f4
0x80a34fe: MOV EBX, dword ptr [EBP + -0x14]
0x80a3501: CMP word ptr [EBX + 0x4], 0x0
0x80a3506: JNZ 0x080a3518
0x80a3508: PUSH ECX
0x80a3509: PUSH 0x16
0x80a350b: PUSH dword ptr [EBX]
0x80a350d: PUSH dword ptr [EBP + 0x8]
0x80a3510: CALL 0x08095c88
0x80a3515: ADD ESP, 0x10
0x80a3518: MOV ESI, dword ptr [EBP + -0x20]
0x80a351b: MOV EAX, dword ptr [ESI]
0x80a351d: MOV EDX, dword ptr [EBP + -0x14]
0x80a3520: OR byte ptr [EDX + 0x6], 0x1
0x80a3524: MOV dword ptr [EDX], EAX
0x80a3526: MOV AX, word ptr [ESI + 0x4]
0x80a352a: MOV word ptr [EDX + 0x4], AX
0x80a352e: MOV ECX, dword ptr [EBP + 0x8]
0x80a3531: MOV SI, word ptr [ECX + 0x10]
0x80a3535: JMP 0x080a32a0
0x80a353a: SUB ESP, 0xc
0x80a353d: PUSH 0x82afacc
0x80a3542: JMP 0x080a33c8
0x80a3547: SUB ESP, 0xc
0x80a354a: PUSH 0x82b007e
0x80a354f: JMP 0x080a33c8
0x80a3554: MOVSX EAX, word ptr [ECX + 0x8]
0x80a3558: SUB ESP, 0x8
0x80a355b: INC EAX
0x80a355c: PUSH EAX
0x80a355d: PUSH 0x82afaf0
0x80a3562: CALL 0x080ff97c
0x80a3567: JMP 0x080a33cd
0x80a356c: PUSH EAX
0x80a356d: PUSH 0x16
0x80a356f: PUSH dword ptr [ECX]
0x80a3571: PUSH dword ptr [EBP + 0x8]
0x80a3574: CALL 0x08095c88
0x80a3579: ADD ESP, 0x10
0x80a357c: JMP 0x080a34a8
```

## 本轮：vm_gatherProgramFunctionTable — procedure_pseudo_code

Evidence ID：`ev_cbeb0564e51d40f356850c8ce433d229a0063dc15122c784bcac9998341ff27d`

```c

/* WARNING: Unknown calling convention */

void vm_gatherProgramFunctionTable(Vm_Program_T *pCurrentProgram)

{
  Vm_Fun_Entry_T *pVVar1;
  Vm_Fun *pVVar2;
  Vm_Fun *pVVar3;
  short sVar4;
  int iVar5;
  short sVar6;
  Vm_Fun_Arg_T *pVVar7;
  Vm_Program_Consititution_T *pVVar8;
  Vm_Fun_Entry_T *pVVar9;
  Vm_Fun_Arg_T *pVVar10;
  short sVar11;
  Vm_Program *pVVar12;
  short sVar13;
  bool bVar14;
  bool bVar15;
  char *pcVar16;
  Vm_Program_Consititution_T *pThisConsititution;
  int notSame;
  Vm_Fun_Entry_T *pFunEntry;
  Vm_Index_T i_1;
  Vm_Index_T i;
  
                    /* Unresolved local var: Vm_Fun_Entry_T * pToFunEntry@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Fun_Entry_T * pFromFunEntry@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Index_T k@[DW_OP_reg7(EDI)] */
  sVar6 = pCurrentProgram->maxConsititutionNo;
  bVar15 = SBORROW2(sVar6,1);
  sVar13 = sVar6 + -1;
  sVar4 = 1;
  bVar14 = sVar6 == 1;
  while (!bVar14 && bVar15 == sVar13 < 0) {
    pVVar8 = pCurrentProgram->pConsititutions + sVar4;
    if ((pVVar8->attrib & 1) != 0) {
      pVVar12 = pVVar8->pProgram;
      sVar13 = 0;
      if (0 < pVVar12->maxFunctionNo) {
        do {
          pVVar1 = pCurrentProgram->pFunctions + (int)sVar13 + (int)pVVar8->baseFunOffset;
          pVVar9 = pVVar12->pFunctions + sVar13;
          pVVar1->consititutionNo = sVar4 + pVVar9->consititutionNo;
          pVVar1->pFun = pVVar9->pFun;
          pVVar1->attrib = pVVar1->attrib | pVVar9->attrib;
          sVar13 = sVar13 + 1;
          pVVar12 = pVVar8->pProgram;
        } while (sVar13 < pVVar12->maxFunctionNo);
        sVar6 = pCurrentProgram->maxConsititutionNo;
      }
    }
    sVar4 = sVar4 + 1;
    bVar15 = SBORROW2(sVar6,sVar4);
    sVar13 = sVar6 - sVar4;
    bVar14 = sVar6 == sVar4;
  }
  pCurrentProgram->initOrderList[sVar6 + -1] = 0;
  if (1 < pCurrentProgram->maxConsititutionNo) {
    pVVar8 = pCurrentProgram->pConsititutions;
    sVar4 = pCurrentProgram->maxConsititutionNo + -1;
    do {
      pVVar8 = pVVar8 + 1;
      pVVar8->pProgram->ref = pVVar8->pProgram->ref + 1;
      sVar4 = sVar4 + -1;
    } while (sVar4 != 0);
  }
                    /* Unresolved local var: Vm_Index_T k@[DW_OP_reg3(EBX)]
                       Unresolved local var: int linked@[???] */
  sVar6 = pCurrentProgram->maxFunctionNo;
  pCurrentProgram->attrib = pCurrentProgram->attrib | 1;
  bVar15 = false;
  bVar14 = sVar6 == 0;
  sVar4 = 0;
  sVar13 = sVar6;
LAB_080a32a8:
  if (bVar14 || bVar15 != sVar6 < 0) {
    return;
  }
  pVVar9 = pCurrentProgram->pFunctions;
  pVVar1 = pVVar9 + sVar4;
  if ((pVVar1->pFun != (Vm_Fun *)0x0) && ((pVVar1->pFun->codeSegment).nodes == (Vm_Node_T *)0x0)) {
    sVar11 = sVar4 + -1;
    sVar6 = sVar4;
    if (-1 < sVar11) {
      do {
        pVVar9 = pVVar9 + sVar11;
        pVVar2 = pVVar9->pFun;
        if ((((pVVar2 != (Vm_Fun *)0x0) && ((pVVar2->codeSegment).nodes != (Vm_Node_T *)0x0)) &&
            ((pVVar2->attrib & 4) == 0)) && ((pVVar9->attrib & 1) == 0)) {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
          pVVar3 = pVVar1->pFun;
          iVar5 = strcmp(pVVar2->name,pVVar3->name);
          if (iVar5 == 0) {
            pVVar10 = pVVar3->pArgs;
            bVar14 = false;
                    /* Unresolved local var: Vm_Fun_Arg_T * pFunArg@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Fun_Arg_T * pDefArg@[DW_OP_reg2(EDX)] */
            for (pVVar7 = pVVar2->pArgs; pVVar7 != (Vm_Fun_Arg_T *)0x0; pVVar7 = pVVar7->pNext) {
              if (pVVar10 == (Vm_Fun_Arg_T *)0x0) goto LAB_080a3450;
              if (pVVar7->type != pVVar10->type) {
                pcVar16 = "Argument type is mismatched.";
                goto LAB_080a3458;
              }
              if (pVVar7->varAttrib != pVVar10->varAttrib) {
                pcVar16 = "Argument attribute is mismatched.";
                goto LAB_080a3458;
              }
              if ((pVVar7->pDefaultValue != (Vm_XNode *)0x0) &&
                 (pVVar10->pDefaultValue != (Vm_XNode *)0x0)) {
                yyerrors("Default value of argument %d is already declared.\n",pVVar7->no + 1);
                goto LAB_080a345d;
              }
              pVVar10 = pVVar10->pNext;
            }
            if (pVVar10 != (Vm_Fun_Arg_T *)0x0) {
LAB_080a3450:
              pcVar16 = "Argument number is mismatched.";
LAB_080a3458:
              yyerror(pcVar16);
LAB_080a345d:
              bVar14 = true;
            }
            if (pVVar3->retType == pVVar2->retType) {
              if (!bVar14) {
                if (pVVar1->consititutionNo == 0) {
                  vm_freeFun(pCurrentProgram,pVVar1->pFun,0x16);
                }
                pVVar2 = pVVar9->pFun;
                pVVar1->attrib = pVVar1->attrib | 1;
                pVVar1->pFun = pVVar2;
                pVVar1->consititutionNo = pVVar9->consititutionNo;
                sVar13 = pCurrentProgram->maxFunctionNo;
                goto LAB_080a32a0;
              }
            }
            else {
              yyerror("Return type is mismatched.");
            }
          }
        }
        sVar11 = sVar11 + -1;
        if (sVar11 < 0) goto code_r0x080a331c;
        pVVar9 = pCurrentProgram->pFunctions;
      } while( true );
    }
    goto LAB_080a3328;
  }
  goto LAB_080a32a0;
code_r0x080a331c:
  sVar13 = pCurrentProgram->maxFunctionNo;
LAB_080a3328:
  while (sVar6 = sVar6 + 1, sVar6 < sVar13) {
    pVVar9 = pCurrentProgram->pFunctions + sVar6;
    pVVar2 = pVVar9->pFun;
    if (((pVVar2 != (Vm_Fun *)0x0) && ((pVVar2->codeSegment).nodes != (Vm_Node_T *)0x0)) &&
       (((pVVar2->attrib & 4) == 0 && ((pVVar9->attrib & 1) == 0)))) {
                    /* Unresolved local var: size_t __s1_len@[???]
                       Unresolved local var: size_t __s2_len@[???] */
      pVVar3 = pVVar1->pFun;
      iVar5 = strcmp(pVVar2->name,pVVar3->name);
      if (iVar5 == 0) {
                    /* Unresolved local var: Vm_Fun_Arg_T * pFunArg@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Fun_Arg_T * pDefArg@[DW_OP_reg2(EDX)]
                       Unresolved local var: int notSame@[DW_OP_reg6(ESI)] */
        bVar14 = false;
        pVVar10 = pVVar3->pArgs;
        for (pVVar7 = pVVar2->pArgs; pVVar7 != (Vm_Fun_Arg_T *)0x0; pVVar7 = pVVar7->pNext) {
          if (pVVar10 == (Vm_Fun_Arg_T *)0x0) goto LAB_080a33c0;
          if (pVVar7->type != pVVar10->type) {
            pcVar16 = "Argument type is mismatched.";
            goto LAB_080a33c8;
          }
          if (pVVar7->varAttrib != pVVar10->varAttrib) {
            pcVar16 = "Argument attribute is mismatched.";
            goto LAB_080a33c8;
          }
          if ((pVVar7->pDefaultValue != (Vm_XNode *)0x0) &&
             (pVVar10->pDefaultValue != (Vm_XNode *)0x0)) {
            yyerrors("Default value of argument %d is already declared.\n",pVVar7->no + 1);
            goto LAB_080a33cd;
          }
          pVVar10 = pVVar10->pNext;
        }
        if (pVVar10 != (Vm_Fun_Arg_T *)0x0) {
LAB_080a33c0:
          pcVar16 = "Argument number is mismatched.";
LAB_080a33c8:
          yyerror(pcVar16);
LAB_080a33cd:
          bVar14 = true;
        }
        if (pVVar3->retType == pVVar2->retType) {
          if (!bVar14) {
            if (pVVar1->consititutionNo == 0) {
              vm_freeFun(pCurrentProgram,pVVar1->pFun,0x16);
            }
            pVVar2 = pVVar9->pFun;
            pVVar1->attrib = pVVar1->attrib | 1;
            pVVar1->pFun = pVVar2;
            pVVar1->consititutionNo = pVVar9->consititutionNo;
            sVar13 = pCurrentProgram->maxFunctionNo;
            break;
          }
        }
        else {
          yyerror("Return type is mismatched.");
        }
        sVar13 = pCurrentProgram->maxFunctionNo;
      }
    }
  }
LAB_080a32a0:
  sVar4 = sVar4 + 1;
  bVar15 = SBORROW2(sVar13,sVar4);
  sVar6 = sVar13 - sVar4;
  bVar14 = sVar13 == sVar4;
  goto LAB_080a32a8;
}
```

## 本轮：vm_inheritFunctionPrototype — procedure_assembly

Evidence ID：`ev_c6179a67691e4485b5ca4e6a6592c20259fb54e358c2c30a060d071de2399f93`

```asm
0x80a28fc: PUSH EBP
0x80a28fd: MOV EBP, ESP
0x80a28ff: PUSH EDI
0x80a2900: PUSH ESI
0x80a2901: PUSH EBX
0x80a2902: SUB ESP, 0xc
0x80a2905: MOV EBX, dword ptr [EBP + 0xc]
0x80a2908: TEST byte ptr [EDX + 0x8], 0x4
0x80a290c: MOV ESI, EAX
0x80a290e: MOV dword ptr [EBP + -0x10], EDX
0x80a2911: MOV word ptr [EBP + -0x12], BX
0x80a2915: MOV EAX, 0x1
0x80a291a: JZ 0x080a2924
0x80a291c: LEA ESP, [EBP + -0xc]
0x80a291f: POP EBX
0x80a2920: POP ESI
0x80a2921: POP EDI
0x80a2922: LEAVE
0x80a2923: RET
0x80a2924: PUSH EAX
0x80a2925: PUSH 0x0
0x80a2927: PUSH ESI
0x80a2928: PUSH dword ptr [0x0847aac0]
0x80a292e: CALL 0x08081e40
0x80a2933: ADD ESP, 0x10
0x80a2936: TEST EAX, EAX
0x80a2938: MOV EDI, EAX
0x80a293a: JZ 0x080a2a41
0x80a2940: TEST byte ptr [EAX + 0x7], 0x8
0x80a2944: JZ 0x080a29d8
0x80a294a: MOV EBX, dword ptr [EAX + 0x14]
0x80a294d: MOV EAX, dword ptr [EBP + -0x10]
0x80a2950: XOR ESI, ESI
0x80a2952: MOV ECX, dword ptr [EBX + 0x30]
0x80a2955: MOV EDX, dword ptr [EAX + 0x30]
0x80a2958: JMP 0x080a2990
0x80a295c: TEST EDX, EDX
0x80a295e: JZ 0x080a2998
0x80a2960: MOV AL, byte ptr [EDX + 0x5]
0x80a2963: CMP byte ptr [ECX + 0x5], AL
0x80a2966: JNZ 0x080a2a27
0x80a296c: MOV AL, byte ptr [EDX + 0x6]
0x80a296f: CMP byte ptr [ECX + 0x6], AL
0x80a2972: JNZ 0x080a2a34
0x80a2978: MOV EAX, dword ptr [ECX + 0xc]
0x80a297b: TEST EAX, EAX
0x80a297d: JZ 0x080a298a
0x80a297f: MOV EAX, dword ptr [EDX + 0xc]
0x80a2982: TEST EAX, EAX
0x80a2984: JNZ 0x080a2af9
0x80a298a: MOV ECX, dword ptr [ECX + 0x10]
0x80a298d: MOV EDX, dword ptr [EDX + 0x10]
0x80a2990: TEST ECX, ECX
0x80a2992: JNZ 0x080a295c
0x80a2994: TEST EDX, EDX
0x80a2996: JZ 0x080a29ad
0x80a2998: SUB ESP, 0xc
0x80a299b: PUSH 0x82afaac
0x80a29a0: CALL 0x080ff80c
0x80a29a5: ADD ESP, 0x10
0x80a29a8: MOV ESI, 0x1
0x80a29ad: MOV AL, byte ptr [EBX + 0xd]
0x80a29b0: MOV EDX, dword ptr [EBP + -0x10]
0x80a29b3: CMP byte ptr [EDX + 0xd], AL
0x80a29b6: JZ 0x080a29f5
0x80a29b8: SUB ESP, 0xc
0x80a29bb: PUSH 0x82b0063
0x80a29c0: CALL 0x080ff80c
0x80a29c5: ADD ESP, 0x10
0x80a29c8: PUSH ECX
0x80a29c9: PUSH dword ptr [EDI]
0x80a29cb: PUSH dword ptr [0x08661924]
0x80a29d1: PUSH 0x82afca0
0x80a29d6: JMP 0x080a29e6
0x80a29d8: PUSH EAX
0x80a29d9: PUSH dword ptr [EDI]
0x80a29db: PUSH dword ptr [0x08661924]
0x80a29e1: PUSH 0x82afcd4
0x80a29e6: CALL 0x080ff97c
0x80a29eb: LEA ESP, [EBP + -0xc]
0x80a29ee: POP EBX
0x80a29ef: POP ESI
0x80a29f0: XOR EAX, EAX
0x80a29f2: POP EDI
0x80a29f3: LEAVE
0x80a29f4: RET
0x80a29f5: TEST ESI, ESI
0x80a29f7: JNZ 0x080a29c8
0x80a29f9: MOV EAX, dword ptr [EDI + 0x14]
0x80a29fc: MOV EDX, dword ptr [EAX + 0x14]
0x80a29ff: TEST EDX, EDX
0x80a2a01: JZ 0x080a2a7e
0x80a2a03: MOV EAX, dword ptr [EBP + -0x10]
0x80a2a06: MOV EAX, dword ptr [EAX + 0x14]
0x80a2a09: TEST EAX, EAX
0x80a2a0b: JNZ 0x080a2a17
0x80a2a0d: MOV EAX, 0x1
0x80a2a12: JMP 0x080a291c
0x80a2a17: PUSH EAX
0x80a2a18: PUSH dword ptr [EDI]
0x80a2a1a: PUSH dword ptr [0x08661924]
0x80a2a20: PUSH 0x82afd00
0x80a2a25: JMP 0x080a29e6
0x80a2a27: SUB ESP, 0xc
0x80a2a2a: PUSH 0x82b007e
0x80a2a2f: JMP 0x080a29a0
0x80a2a34: SUB ESP, 0xc
0x80a2a37: PUSH 0x82afacc
0x80a2a3c: JMP 0x080a29a0
0x80a2a41: SUB ESP, 0x8
0x80a2a44: PUSH ESI
0x80a2a45: PUSH dword ptr [0x0847aac0]
0x80a2a4b: CALL 0x08085a94
0x80a2a50: MOV EDI, EAX
0x80a2a52: MOV dword ptr [EAX + 0x4], 0x2a000000
0x80a2a59: MOV EAX, dword ptr [EBP + -0x10]
0x80a2a5c: MOV dword ptr [EDI + 0x14], EAX
0x80a2a5f: INC dword ptr [EDI + 0x8]
0x80a2a62: INC word ptr [EDI + 0x2e]
0x80a2a66: ADD ESP, 0x10
0x80a2a69: MOV EAX, 0x1
0x80a2a6e: MOV word ptr [EDI + 0x18], BX
0x80a2a72: MOV dword ptr [EDI + 0x1c], 0x0
0x80a2a79: JMP 0x080a291c
0x80a2a7e: MOV EDX, dword ptr [EBP + -0x10]
0x80a2a81: MOV ESI, dword ptr [EDX + 0x14]
0x80a2a84: TEST ESI, ESI
0x80a2a86: JZ 0x080a2a0d
0x80a2a88: MOV EDX, dword ptr [EDI + 0x4]
0x80a2a8b: XOR EBX, EBX
0x80a2a8d: TEST EDX, 0x2000000
0x80a2a93: JNZ 0x080a2aa1
0x80a2a95: AND EDX, 0x20000000
0x80a2a9b: JNZ 0x080a2b3d
0x80a2aa1: MOV EDX, dword ptr [EAX + 0x14]
0x80a2aa4: TEST EDX, EDX
0x80a2aa6: JNZ 0x080a2b11
0x80a2aa8: MOV BX, word ptr [EDI + 0x24]
0x80a2aac: MOV EAX, EBX
0x80a2aae: TEST AL, 0x4
0x80a2ab0: JZ 0x080a2ab6
0x80a2ab2: TEST EDX, EDX
0x80a2ab4: JNZ 0x080a2b26
0x80a2ab6: SUB ESP, 0x8
0x80a2ab9: PUSH 0x0
0x80a2abb: PUSH EDI
0x80a2abc: CALL 0x08085854
0x80a2ac1: ADD ESP, 0x10
0x80a2ac4: TEST EAX, EAX
0x80a2ac6: JZ 0x080a2b17
0x80a2ac8: MOV EAX, dword ptr [EBP + -0x10]
0x80a2acb: MOV dword ptr [EDI + 0x14], EAX
0x80a2ace: MOV DX, word ptr [EBP + -0x12]
0x80a2ad2: INC dword ptr [EDI + 0x8]
0x80a2ad5: INC word ptr [EDI + 0x2e]
0x80a2ad9: MOV EAX, 0x1
0x80a2ade: MOV dword ptr [EDI + 0x1c], 0x0
0x80a2ae5: MOV word ptr [EDI + 0x24], BX
0x80a2ae9: MOV word ptr [EDI + 0x18], DX
0x80a2aed: MOV dword ptr [EDI + 0x4], 0x2a000000
0x80a2af4: JMP 0x080a291c
0x80a2af9: MOVSX EAX, word ptr [ECX + 0x8]
0x80a2afd: SUB ESP, 0x8
0x80a2b00: INC EAX
0x80a2b01: PUSH EAX
0x80a2b02: PUSH 0x82afaf0
0x80a2b07: CALL 0x080ff97c
0x80a2b0c: JMP 0x080a29a5
0x80a2b11: MOV AX, word ptr [EDI + 0x24]
0x80a2b15: JMP 0x080a2aae
0x80a2b17: SUB ESP, 0xc
0x80a2b1a: PUSH -0x2
0x80a2b1c: CALL 0x080ff758
0x80a2b21: ADD ESP, 0x10
0x80a2b24: JMP 0x080a2ac8
0x80a2b26: SUB ESP, 0x8
0x80a2b29: PUSH dword ptr [EDI]
0x80a2b2b: PUSH 0x82af960
0x80a2b30: CALL 0x080ff774
0x80a2b35: ADD ESP, 0x10
0x80a2b38: JMP 0x080a2ab6
0x80a2b3d: SUB ESP, 0x8
0x80a2b40: PUSH dword ptr [EDI]
0x80a2b42: PUSH 0x82afa44
0x80a2b47: CALL 0x080ff774
0x80a2b4c: ADD ESP, 0x10
0x80a2b4f: MOV EAX, dword ptr [EDI + 0x14]
0x80a2b52: JMP 0x080a2aa1
```

## 本轮：vm_inheritFunctionPrototype — procedure_pseudo_code

Evidence ID：`ev_f106a1ad196036705391e3d2710b9e91f713c16845951d9656ce1036e69e1c66`

```c

int __regparm2
vm_inheritFunctionPrototype
          (char *funName,Vm_Fun_T *pFromFun,Vm_Program_T *pFromProgram,Vm_Index_T newFunOffset)

{
  short *psVar1;
  Vm_Node_T *pVVar2;
  bool bVar3;
  ushort uVar4;
  Vm_Ident_Hash_Elem_T *pVVar5;
  Vm_Fun *pVVar6;
  int iVar7;
  Vm_Fun_Arg_T *pVVar8;
  Vm_Ident_Hash_Elem_T *extraout_ECX;
  Vm_Ident_Hash_Elem_T *pVVar9;
  Vm_Ident_Hash_Elem_T *extraout_ECX_00;
  Vm_Ident_Hash_Elem_T *extraout_ECX_01;
  Vm_Fun_Arg_T *pVVar10;
  ushort uVar11;
  char *pcVar12;
  char *str;
  undefined4 uVar13;
  undefined4 uVar14;
  Vm_Index_T newFunOffset_local;
  Vm_Fun_T *pFromFun_local;
  
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * toIhe@[DW_OP_reg7(EDI)] */
  if ((pFromFun->attrib & 4) == 0) {
    uVar14 = 1;
    uVar13 = 0;
    pVVar5 = vm_lookupIdentInProgram(vm_pCurrentProgram,funName,0);
    if (pVVar5 != (Vm_Ident_Hash_Elem_T *)0x0) {
      if ((pVVar5->token & 0x8000000) == 0) {
        pcVar12 = pVVar5->name;
        str = "Line %d Identifer \"%s\" is already defined.\n";
      }
      else {
        pVVar6 = (pVVar5->ii).v.uf.pFun;
                    /* Unresolved local var: Vm_Fun_Arg_T * pFunArg@[DW_OP_reg1(ECX)]
                       Unresolved local var: Vm_Fun_Arg_T * pDefArg@[DW_OP_reg2(EDX)]
                       Unresolved local var: int notSame@[DW_OP_reg6(ESI)] */
        bVar3 = false;
        pVVar10 = pFromFun->pArgs;
        for (pVVar8 = pVVar6->pArgs; pVVar8 != (Vm_Fun_Arg_T *)0x0; pVVar8 = pVVar8->pNext) {
          if (pVVar10 == (Vm_Fun_Arg_T *)0x0) goto LAB_080a2998;
          if (pVVar8->type != pVVar10->type) {
            pcVar12 = "Argument type is mismatched.";
            goto LAB_080a29a0;
          }
          if (pVVar8->varAttrib != pVVar10->varAttrib) {
            pcVar12 = "Argument attribute is mismatched.";
            goto LAB_080a29a0;
          }
          if ((pVVar8->pDefaultValue != (Vm_XNode *)0x0) &&
             (pVVar10->pDefaultValue != (Vm_XNode *)0x0)) {
            yyerrors("Default value of argument %d is already declared.\n",pVVar8->no + 1,uVar13,
                     uVar14);
            pVVar9 = extraout_ECX_01;
            goto LAB_080a29a5;
          }
          pVVar10 = pVVar10->pNext;
        }
        pVVar9 = (Vm_Ident_Hash_Elem_T *)0x0;
        if (pVVar10 != (Vm_Fun_Arg_T *)0x0) {
LAB_080a2998:
          pcVar12 = "Argument number is mismatched.";
LAB_080a29a0:
          yyerror(pcVar12);
          pVVar9 = extraout_ECX;
LAB_080a29a5:
          bVar3 = true;
        }
        if (pFromFun->retType == pVVar6->retType) {
          if (!bVar3) {
            pVVar6 = (pVVar5->ii).v.uf.pFun;
            if ((pVVar6->codeSegment).nodes == (Vm_Node_T *)0x0) {
              if ((pFromFun->codeSegment).nodes == (Vm_Node_T *)0x0) {
                return 1;
              }
                    /* Unresolved local var: ushort inheritRefered@[DW_OP_reg3(EBX)] */
              uVar11 = 0;
              if (((pVVar5->token & 0x2000000) == 0) && ((pVVar5->token & 0x20000000) != 0)) {
                yywarns("LPC8501: Override self function \"%s\" occurred, remove it.\nThe feature of override self is deprecated.\n"
                        ,pVVar5->name);
                pVVar6 = (pVVar5->ii).v.uf.pFun;
              }
              pVVar2 = (pVVar6->codeSegment).nodes;
              if (pVVar2 == (Vm_Node_T *)0x0) {
                uVar11 = (pVVar5->ii).refered;
                uVar4 = uVar11;
              }
              else {
                uVar4 = (pVVar5->ii).refered;
              }
              if (((uVar4 & 4) != 0) && (pVVar2 != (Vm_Node_T *)0x0)) {
                yywarns("LPC8502: Function calling happened before overrided function \"%s\".\nPlease move the calling statment to previous location before calling.\nThe feature to call basic-function for statments before overrided function is deprecated.\n"
                        ,pVVar5->name);
              }
              iVar7 = vm_backupIi(pVVar5,(void *)0x0);
              if (iVar7 == 0) {
                yystop(-2);
              }
              (pVVar5->ii).v.uf.pFun = pFromFun;
              pVVar5->semValue = pVVar5->semValue + 1;
              psVar1 = &(pVVar5->dn).globalNum;
              *psVar1 = *psVar1 + 1;
              (pVVar5->ii).pInContext = (void *)0x0;
              (pVVar5->ii).refered = uVar11;
              (pVVar5->ii).v.uf.funOffset = newFunOffset;
              pVVar5->token = 0x2a000000;
              return 1;
            }
            pVVar9 = (Vm_Ident_Hash_Elem_T *)(pFromFun->codeSegment).nodes;
            if (pVVar9 == (Vm_Ident_Hash_Elem_T *)0x0) {
              return 1;
            }
            pcVar12 = pVVar5->name;
            str = "Line %d Function \"%s\" is already implemented.\n";
            pVVar5 = pVVar9;
            goto LAB_080a29e6;
          }
        }
        else {
          yyerror("Return type is mismatched.");
          pVVar9 = extraout_ECX_00;
        }
        pcVar12 = pVVar5->name;
        str = "Line %d The function \"%s\" mismatch pre-defined.\n";
        pVVar5 = pVVar9;
      }
LAB_080a29e6:
      yyerrors(str,currentLine,pcVar12,pVVar5);
      return 0;
    }
    pVVar5 = vm_findOrAddIdentInProgram(vm_pCurrentProgram,funName);
    pVVar5->token = 0x2a000000;
    (pVVar5->ii).v.uf.pFun = pFromFun;
    pVVar5->semValue = pVVar5->semValue + 1;
    psVar1 = &(pVVar5->dn).globalNum;
    *psVar1 = *psVar1 + 1;
    (pVVar5->ii).v.uf.funOffset = newFunOffset;
    (pVVar5->ii).pInContext = (void *)0x0;
  }
  return 1;
}
```

## 本轮：vm_overrideFunction — procedure_pseudo_code

Evidence ID：`ev_d7402d7e06126da2991a947d90a56084eaac0ba169f9b996e21f0550d42925b6`

```c

int vm_overrideFunction(Vm_Ident_Hash_Elem_T *ihe,Vm_Fun_T *pFun,Vm_Index_T funOffset)

{
  short *psVar1;
  Vm_Node_T *pVVar2;
  ushort uVar3;
  int iVar4;
  ushort uVar5;
  
                    /* Unresolved local var: ushort inheritRefered@[DW_OP_reg6(ESI)] */
  uVar5 = 0;
  if (((ihe->token & 0x2000000) == 0) && ((ihe->token & 0x20000000) != 0)) {
    yywarns("LPC8501: Override self function \"%s\" occurred, remove it.\nThe feature of override self is deprecated.\n"
            ,ihe->name);
  }
  pVVar2 = (((ihe->ii).v.uf.pFun)->codeSegment).nodes;
  if (pVVar2 == (Vm_Node_T *)0x0) {
    uVar5 = (ihe->ii).refered;
    uVar3 = uVar5;
  }
  else {
    uVar3 = (ihe->ii).refered;
  }
  if (((uVar3 & 4) != 0) && (pVVar2 != (Vm_Node_T *)0x0)) {
    yywarns("LPC8502: Function calling happened before overrided function \"%s\".\nPlease move the calling statment to previous location before calling.\nThe feature to call basic-function for statments before overrided function is deprecated.\n"
            ,ihe->name);
  }
  iVar4 = vm_backupIi(ihe,(void *)0x0);
  if (iVar4 == 0) {
    yystop(-2);
  }
  (ihe->ii).v.uf.pFun = pFun;
  ihe->token = 0x28000000;
  ihe->semValue = ihe->semValue + 1;
  psVar1 = &(ihe->dn).globalNum;
  *psVar1 = *psVar1 + 1;
  (ihe->ii).pInContext = (void *)0x0;
  (ihe->ii).refered = uVar5;
  (ihe->ii).v.uf.funOffset = funOffset;
  return 1;
}
```

## 本轮：vm_pushObjVar — procedure_assembly

Evidence ID：`ev_9a9cde77fb2df1b8214e667458663471ea6da5fbeea24bfb3878fcb8f7548808`

```asm
0x8091224: PUSH EBP
0x8091225: MOV EBP, ESP
0x8091227: PUSH EBX
0x8091228: PUSH EAX
0x8091229: MOV EDX, dword ptr [0x086619d8]
0x809122f: MOV EAX, dword ptr [EDX + 0x14]
0x8091232: MOVSX EAX, word ptr [EAX + 0x8]
0x8091236: ADD EAX, dword ptr [EBP + 0x8]
0x8091239: MOV EDX, dword ptr [EDX + 0x20]
0x809123c: LEA EAX, [EAX + EAX*0x2]
0x809123f: MOV EDX, dword ptr [EDX + 0x24]
0x8091242: LEA EBX, [EDX + EAX*0x8]
0x8091245: MOV EAX, [0x086619cc]
0x809124a: TEST EAX, EAX
0x809124c: JZ 0x08091288
0x809124e: DEC EAX
0x809124f: MOV [0x086619cc], EAX
0x8091254: MOV EDX, dword ptr [0x08661964]
0x809125a: LEA EAX, [EAX + EAX*0x2]
0x809125d: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x8091262: MOV EAX, [0x086619cc]
0x8091267: SUB ESP, 0x8
0x809126a: MOV EDX, dword ptr [0x08661964]
0x8091270: LEA EAX, [EAX + EAX*0x2]
0x8091273: PUSH EBX
0x8091274: LEA EAX, [EDX + EAX*0x8]
0x8091277: PUSH EAX
0x8091278: CALL 0x0808be24
0x809127d: ADD ESP, 0x10
0x8091280: MOV EBX, dword ptr [EBP + -0x4]
0x8091283: LEAVE
0x8091284: RET
0x8091288: SUB ESP, 0xc
0x809128b: PUSH 0x82ad98c
0x8091290: CALL 0x08097b24
0x8091295: ADD ESP, 0x10
0x8091298: MOV EAX, [0x086619cc]
0x809129d: JMP 0x0809124e
```

## 本轮：vm_pushObjVar — procedure_pseudo_code

Evidence ID：`ev_03adb0f3a115b83f2497d37852e6c64ffd233cbe06b82b9e57624900a2a01c09`

```c

/* WARNING: Unknown calling convention */

void vm_pushObjVar(int valueNo)

{
  short sVar1;
  Vm_Value *pVVar2;
  
  sVar1 = vm_currentContext->consititution->baseVarOffset;
  pVVar2 = vm_currentContext->thisObject->pObjectValues;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_copyValue(vm_stack + vm_sp,pVVar2 + sVar1 + valueNo);
  return;
}
```

## 本轮：vm_pushObjVptr — procedure_assembly

Evidence ID：`ev_fb501c3a4e5420446efd5a59a2c8196c01d1f8540c4f0c1095c98d12e7b9f8cf`

```asm
0x80911a8: PUSH EBP
0x80911a9: MOV EBP, ESP
0x80911ab: PUSH EBX
0x80911ac: PUSH EAX
0x80911ad: MOV EDX, dword ptr [0x086619d8]
0x80911b3: MOV EAX, dword ptr [EDX + 0x14]
0x80911b6: MOVSX EAX, word ptr [EAX + 0x8]
0x80911ba: ADD EAX, dword ptr [EBP + 0x8]
0x80911bd: MOV EDX, dword ptr [EDX + 0x20]
0x80911c0: LEA EAX, [EAX + EAX*0x2]
0x80911c3: MOV EDX, dword ptr [EDX + 0x24]
0x80911c6: LEA EBX, [EDX + EAX*0x8]
0x80911c9: MOV EAX, [0x086619cc]
0x80911ce: TEST EAX, EAX
0x80911d0: JZ 0x0809120c
0x80911d2: DEC EAX
0x80911d3: MOV [0x086619cc], EAX
0x80911d8: MOV EDX, dword ptr [0x08661964]
0x80911de: LEA EAX, [EAX + EAX*0x2]
0x80911e1: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x8
0x80911e6: MOV EAX, [0x086619cc]
0x80911eb: LEA EAX, [EAX + EAX*0x2]
0x80911ee: MOV ECX, dword ptr [0x08661964]
0x80911f4: MOV DL, byte ptr [EBX + 0x5]
0x80911f7: SHL EAX, 0x3
0x80911fa: INC dword ptr [EBX]
0x80911fc: MOV dword ptr [EAX + ECX*0x1 + 0x8], EBX
0x8091200: MOV byte ptr [EAX + ECX*0x1 + 0x5], DL
0x8091204: MOV EBX, dword ptr [EBP + -0x4]
0x8091207: LEAVE
0x8091208: RET
0x809120c: SUB ESP, 0xc
0x809120f: PUSH 0x82ad98c
0x8091214: CALL 0x08097b24
0x8091219: ADD ESP, 0x10
0x809121c: MOV EAX, [0x086619cc]
0x8091221: JMP 0x080911d2
```

## 本轮：vm_pushObjVptr — procedure_pseudo_code

Evidence ID：`ev_525701540b4de9914051ebdb9af2e55a2baac12f24fdacd8fe87d50535876b6f`

```c

/* WARNING: Unknown calling convention */

void vm_pushObjVptr(int valueNo)

{
  Vm_Value *pVVar1;
  uchar uVar2;
  Vm_Value_T *pVVar3;
  int iVar4;
  
  pVVar1 = vm_currentContext->thisObject->pObjectValues +
           vm_currentContext->consititution->baseVarOffset + valueNo;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\b';
  iVar4 = vm_sp;
  pVVar3 = vm_stack;
  uVar2 = pVVar1->attrib;
  pVVar1->ref = pVVar1->ref + 1;
  pVVar3[iVar4].v.pPtrValue = pVVar1;
  pVVar3[iVar4].attrib = uVar2;
  return;
}
```

## 本轮：vm_pushSharedString — procedure_assembly

Evidence ID：`ev_734b597b55be50ee446a59e8502687b0068270e4791dcc47564e0a019eaeee7c`

```asm
0x8090cf8: PUSH EBP
0x8090cf9: MOV EAX, [0x086619cc]
0x8090cfe: MOV EBP, ESP
0x8090d00: TEST EAX, EAX
0x8090d02: PUSH ESI
0x8090d03: PUSH EBX
0x8090d04: MOV ESI, dword ptr [EBP + 0x8]
0x8090d07: JZ 0x08090d70
0x8090d09: MOV EAX, [0x086619d8]
0x8090d0e: MOV EAX, dword ptr [EAX + 0x14]
0x8090d11: TEST ESI, ESI
0x8090d13: MOV EDX, dword ptr [EAX]
0x8090d15: JS 0x08090d1f
0x8090d17: MOVSX EAX, word ptr [EDX + 0x24]
0x8090d1b: CMP EAX, ESI
0x8090d1d: JG 0x08090d64
0x8090d1f: SUB ESP, 0x8
0x8090d22: PUSH ESI
0x8090d23: PUSH 0x82adfaa
0x8090d28: CALL 0x08097b24
0x8090d2d: ADD ESP, 0x10
0x8090d30: MOV EAX, [0x086619cc]
0x8090d35: DEC EAX
0x8090d36: MOV [0x086619cc], EAX
0x8090d3b: MOV EDX, dword ptr [0x08661964]
0x8090d41: LEA EAX, [EAX + EAX*0x2]
0x8090d44: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x4
0x8090d49: MOV EAX, [0x086619cc]
0x8090d4e: MOV EDX, dword ptr [0x08661964]
0x8090d54: LEA EAX, [EAX + EAX*0x2]
0x8090d57: MOV dword ptr [EDX + EAX*0x8 + 0x8], EBX
0x8090d5b: INC dword ptr [EBX]
0x8090d5d: LEA ESP, [EBP + -0x8]
0x8090d60: POP EBX
0x8090d61: POP ESI
0x8090d62: LEAVE
0x8090d63: RET
0x8090d64: MOV EAX, dword ptr [EDX + 0x20]
0x8090d67: MOV EBX, dword ptr [EAX + ESI*0x4]
0x8090d6a: TEST EBX, EBX
0x8090d6c: JNZ 0x08090d30
0x8090d6e: JMP 0x08090d1f
0x8090d70: SUB ESP, 0xc
0x8090d73: PUSH 0x82ad98c
0x8090d78: CALL 0x08097b24
0x8090d7d: ADD ESP, 0x10
0x8090d80: JMP 0x08090d09
```

## 本轮：vm_pushSharedString — procedure_pseudo_code

Evidence ID：`ev_036c7ff9e3a4c3a9d1380520976755debc306a776b18e23161519177ae073205`

```c

void vm_pushSharedString(int n)

{
  Vm_Program *pVVar1;
  Vm_String_T *unaff_EBX;
  
                    /* Unresolved local var: Vm_Program_T * pProgram@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_String_T * pString@[DW_OP_reg3(EBX)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  pVVar1 = vm_currentContext->consititution->pProgram;
  if (((n < 0) || (pVVar1->maxStringNo <= n)) ||
     (unaff_EBX = pVVar1->ppStrings[n], unaff_EBX == (Vm_String_T *)0x0)) {
    vm_error("Bad shared string %d!\n",n);
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x04';
  vm_stack[vm_sp].v.string = unaff_EBX;
  unaff_EBX->ref = unaff_EBX->ref + 1;
  return;
}
```

## 本轮：vm_regularProcessor — procedure_assembly

Evidence ID：`ev_a53c5fa33a9f1b853cc9eca8f9c6f6c1cf304476f607c52afa2cf558d1f7f597`

```asm
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

## 本轮：vm_regularProcessor — procedure_pseudo_code

Evidence ID：`ev_5673fa5b2edb4d5bf79e9f9431a44467493506747573d429692a62ef74c6c5de`

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

## 本轮：vm_setObjVar — procedure_assembly

Evidence ID：`ev_41e51f5930ad32612eae03bb92635d03412ccbcc621485125ad2e864f4c4f75a`

```asm
0x808f7f8: PUSH EBP
0x808f7f9: MOV EBP, ESP
0x808f7fb: PUSH EBX
0x808f7fc: SUB ESP, 0x10
0x808f7ff: MOV EDX, dword ptr [0x086619d8]
0x808f805: MOV EAX, dword ptr [EDX + 0x14]
0x808f808: MOVSX EBX, word ptr [EAX + 0x8]
0x808f80c: ADD EBX, dword ptr [EBP + 0x8]
0x808f80f: MOV EAX, dword ptr [EDX + 0x20]
0x808f812: MOV EAX, dword ptr [EAX + 0x24]
0x808f815: LEA EBX, [EBX + EBX*0x2]
0x808f818: LEA EBX, [EAX + EBX*0x8]
0x808f81b: PUSH EBX
0x808f81c: CALL 0x0808ad98
0x808f821: POP EAX
0x808f822: MOV EAX, [0x086619cc]
0x808f827: POP EDX
0x808f828: LEA EAX, [EAX + EAX*0x2]
0x808f82b: MOV EDX, dword ptr [0x08661964]
0x808f831: LEA EAX, [EDX + EAX*0x8]
0x808f834: PUSH EAX
0x808f835: PUSH EBX
0x808f836: CALL 0x0808be24
0x808f83b: MOV EBX, dword ptr [EBP + -0x4]
0x808f83e: LEAVE
0x808f83f: RET
```

## 本轮：vm_setObjVar — procedure_pseudo_code

Evidence ID：`ev_85a706f75e7ed097afaab10b0d9474c379d806b7272d872c2ea29584a5f56438`

```c

/* WARNING: Unknown calling convention */

void vm_setObjVar(int valueNo)

{
  Vm_Value_T *pValue;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pValue = vm_currentContext->thisObject->pObjectValues +
           vm_currentContext->consititution->baseVarOffset + valueNo;
  vm_cleanupValue(pValue);
  vm_copyValue(pValue,vm_stack + vm_sp);
  return;
}
```

## 本轮：/DWARF/vm_action.h/Vm_FunCall_Context/Vm_Program_Consititution — inspect_native_data_type

Evidence ID：`ev_582a03dd695738a8d26aac5a532e68919f7c76024a231dc643f2b48941b58b84`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pProgram",
      "offset_bytes": 0,
      "ordinal": 0,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_FunCall_Context/Vm_Program_Consititution/Vm_Program *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "attrib",
      "offset_bytes": 4,
      "ordinal": 1,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "unused",
      "offset_bytes": 5,
      "ordinal": 2,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "baseFunOffset",
      "offset_bytes": 6,
      "ordinal": 3,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "baseVarOffset",
      "offset_bytes": 8,
      "ordinal": 4,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    }
  ],
  "id": "/DWARF/vm_action.h/Vm_FunCall_Context/Vm_Program_Consititution",
  "kind": "struct",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Program_Consititution",
  "packing_enabled": true,
  "reason": null,
  "referenced_type": null,
  "size_bytes": 12,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 5,
  "truncated": false
}
```

## 本轮：/DWARF/vm_fun.h/Vm_Fun_Entry — inspect_native_data_type

Evidence ID：`ev_7501f730d5843b9d3f322072da898824ff7ffbb2f679d79475efa4826b924e66`

```json
{
  "address": null,
  "alignment_bytes": null,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": null,
  "kind": "unavailable",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": null,
  "packing_enabled": null,
  "reason": "No exact database type or defined typed data exists at the selected identity",
  "referenced_type": null,
  "size_bytes": null,
  "source": "analysis-database",
  "source_archive": null,
  "status": "unavailable",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：/DWARF/vm_fun.h/Vm_Fun_Entry_T — inspect_native_data_type

Evidence ID：`ev_2b59e15dff2ae4bd55dc26e6d649415cec1e75b762c5916c7d2ebb211c9b82aa`

```json
{
  "address": null,
  "alignment_bytes": null,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": null,
  "kind": "unavailable",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": null,
  "packing_enabled": null,
  "reason": "No exact database type or defined typed data exists at the selected identity",
  "referenced_type": null,
  "size_bytes": null,
  "source": "analysis-database",
  "source_archive": null,
  "status": "unavailable",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：/DWARF/vm_lex.h/Vm_FunCall_Context/Vm_Program_Consititution/Vm_Program — inspect_native_data_type

Evidence ID：`ev_47dd96b2854cbe63e0080ce518465e7ff291a29d388b4e8ea3f0712fabf620d4`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "ref",
      "offset_bytes": 0,
      "ordinal": 0,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/int"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "strName",
      "offset_bytes": 4,
      "ordinal": 1,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_String *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "attrib",
      "offset_bytes": 8,
      "ordinal": 2,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "createFunOffset",
      "offset_bytes": 10,
      "ordinal": 3,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "destructFunOffset",
      "offset_bytes": 12,
      "ordinal": 4,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "maxVariableNo",
      "offset_bytes": 14,
      "ordinal": 5,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "maxFunctionNo",
      "offset_bytes": 16,
      "ordinal": 6,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "consititutionBufferSize",
      "offset_bytes": 18,
      "ordinal": 7,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "maxConsititutionNo",
      "offset_bytes": 20,
      "ordinal": 8,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pConsititutions",
      "offset_bytes": 24,
      "ordinal": 9,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_object.h/Vm_Program_Consititution_T *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "initOrderList",
      "offset_bytes": 28,
      "ordinal": 10,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "ppStrings",
      "offset_bytes": 32,
      "ordinal": 11,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_String * *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "maxStringNo",
      "offset_bytes": 36,
      "ordinal": 12,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "stringBufSize",
      "offset_bytes": 38,
      "ordinal": 13,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pProgVars",
      "offset_bytes": 40,
      "ordinal": 14,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_object.h/Vm_Prog_Var_T *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pFunctions",
      "offset_bytes": 44,
      "ordinal": 15,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_object.h/Vm_Fun_Entry_T *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pHiddenFuns",
      "offset_bytes": 48,
      "ordinal": 16,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_Ident_Unit/anon_struct_8_2_e648a62a_for_uf/Vm_Fun *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "initCodeSegment",
      "offset_bytes": 52,
      "ordinal": 17,
      "size_bytes": 32,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_Code_Segment"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "ppHashTable",
      "offset_bytes": 84,
      "ordinal": 18,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_Ident_Hash_Elem * *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "ppHashHead",
      "offset_bytes": 88,
      "ordinal": 19,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_Ident_Hash_Elem * *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "quickFunctionSize",
      "offset_bytes": 92,
      "ordinal": 20,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pQuickFunction",
      "offset_bytes": 96,
      "ordinal": 21,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_object.h/Vm_Quick_Function *"
    }
  ],
  "id": "/DWARF/vm_lex.h/Vm_FunCall_Context/Vm_Program_Consititution/Vm_Program",
  "kind": "struct",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Program",
  "packing_enabled": true,
  "reason": null,
  "referenced_type": null,
  "size_bytes": 100,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 22,
  "truncated": false
}
```

## 本轮：/DWARF/vm_object.h/Vm_Fun_Entry — inspect_native_data_type

Evidence ID：`ev_e6e788fc95d1403d2daa1f3a3db6b4e5ed24081078ff520e5395a292c3d3e8f5`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "pFun",
      "offset_bytes": 0,
      "ordinal": 0,
      "size_bytes": 4,
      "status": "observed",
      "type_id": "/DWARF/vm_lex.h/Vm_Ident_Unit/anon_struct_8_2_e648a62a_for_uf/Vm_Fun *"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "consititutionNo",
      "offset_bytes": 4,
      "ordinal": 1,
      "size_bytes": 2,
      "status": "observed",
      "type_id": "/DWARF/Vm_Index_T"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "attrib",
      "offset_bytes": 6,
      "ordinal": 2,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    },
    {
      "bit_offset": null,
      "bit_size": null,
      "flexible_tail": "unknown",
      "name": "unused",
      "offset_bytes": 7,
      "ordinal": 3,
      "size_bytes": 1,
      "status": "observed",
      "type_id": "/uchar"
    }
  ],
  "id": "/DWARF/vm_object.h/Vm_Fun_Entry",
  "kind": "struct",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Fun_Entry",
  "packing_enabled": true,
  "reason": null,
  "referenced_type": null,
  "size_bytes": 8,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 4,
  "truncated": false
}
```

## 本轮：/DWARF/vm_object.h/Vm_Fun_Entry_T — inspect_native_data_type

Evidence ID：`ev_aa2011031fcd40cd856dda4d87e90d57b90648b9140112f2a8e97217afcec0da`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_object.h/Vm_Fun_Entry_T",
  "kind": "typedef",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Fun_Entry_T",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_object.h/Vm_Fun_Entry",
  "size_bytes": 8,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：/DWARF/vm_object.h/Vm_Program_Consititution — inspect_native_data_type

Evidence ID：`ev_6c621cfcf5bd87d3fb61919af07ef03e196b077423375c048269fb0472bf0dc0`

```json
{
  "address": null,
  "alignment_bytes": null,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": null,
  "kind": "unavailable",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": null,
  "packing_enabled": null,
  "reason": "No exact database type or defined typed data exists at the selected identity",
  "referenced_type": null,
  "size_bytes": null,
  "source": "analysis-database",
  "source_archive": null,
  "status": "unavailable",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：/DWARF/vm_object.h/Vm_Program_Consititution_T — inspect_native_data_type

Evidence ID：`ev_c2f8108a74f7ee3e4de9e063bc3787ad6a10c4911726fdfbe30b7b0fd1875e59`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_object.h/Vm_Program_Consititution_T",
  "kind": "typedef",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Program_Consititution_T",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_action.h/Vm_FunCall_Context/Vm_Program_Consititution",
  "size_bytes": 12,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：/DWARF/vm_object.h/Vm_Program_T — inspect_native_data_type

Evidence ID：`ev_e2652d11455fff30647dbef03bae6f81c5a31d137bcbafd1e9d0a7274996329a`

```json
{
  "address": null,
  "alignment_bytes": 4,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": "/DWARF/vm_object.h/Vm_Program_T",
  "kind": "typedef",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": "Vm_Program_T",
  "packing_enabled": null,
  "reason": null,
  "referenced_type": "/DWARF/vm_lex.h/Vm_FunCall_Context/Vm_Program_Consititution/Vm_Program",
  "size_bytes": 100,
  "source": "analysis-database",
  "source_archive": "target-5d4ebac58d8c.bin",
  "status": "available",
  "total_fields": 0,
  "truncated": false
}
```

## 本轮：/DWARF/vm_program.h/Vm_Program_Consititution_T — inspect_native_data_type

Evidence ID：`ev_d857908008f3499090ddda8b2df0aa7e769f73ed525211993d9d1bbde863e1f5`

```json
{
  "address": null,
  "alignment_bytes": null,
  "array_count": null,
  "array_stride_bytes": null,
  "fields": [],
  "id": null,
  "kind": "unavailable",
  "limitations": [
    "Layouts are Ghidra data-type database observations; original debug/source authority and flexible-tail semantics are not inferred. Nested and recursive types retain category-path identities for separate inspection.",
    "At most 20000 fields or enum members are serialized per type to bound provider output. total_fields and truncated report omissions. Enum values preserve Ghidra's signed long representation."
  ],
  "members": [],
  "name": null,
  "packing_enabled": null,
  "reason": "No exact database type or defined typed data exists at the selected identity",
  "referenced_type": null,
  "size_bytes": null,
  "source": "analysis-database",
  "source_archive": null,
  "status": "unavailable",
  "total_fields": 0,
  "truncated": false
}
```

## 复用：vm_callInUserFun — procedure_assembly

Evidence ID：`ev_cc0287284ad5443dfe5a19500a3fbfcbe1369c1afa26f0a29dff5cc0d8a9f347`

```asm
0x8094080: PUSH EBP
0x8094081: MOV EBP, ESP
0x8094083: PUSH EDI
0x8094084: PUSH ESI
0x8094085: PUSH EBX
0x8094086: SUB ESP, 0xc
0x8094089: MOV EDX, dword ptr [EBP + 0xc]
0x809408c: MOV EAX, EDX
0x809408e: SHR EAX, 0x7
0x8094091: MOV EDI, EDX
0x8094093: AND EDI, 0x3f
0x8094096: AND EAX, 0x1
0x8094099: MOV ECX, dword ptr [EBP + 0x8]
0x809409c: MOV dword ptr [EBP + -0x10], EAX
0x809409f: JZ 0x080940ec
0x80940a1: TEST CX, CX
0x80940a4: JS 0x080940b4
0x80940a6: MOV EAX, [0x0847aad0]
0x80940ab: CMP word ptr [EAX + 0x10], CX
0x80940af: MOVSX EBX, CX
0x80940b2: JG 0x080940cd
0x80940b4: SUB ESP, 0x8
0x80940b7: MOVSX EBX, CX
0x80940ba: PUSH EBX
0x80940bb: PUSH 0x82ae3f0
0x80940c0: CALL 0x08097b24
0x80940c5: ADD ESP, 0x10
0x80940c8: MOV EAX, [0x0847aad0]
0x80940cd: MOV EAX, dword ptr [EAX + 0x2c]
0x80940d0: SUB ESP, 0xc
0x80940d3: MOV EAX, dword ptr [EAX + EBX*0x8]
0x80940d6: PUSH EDI
0x80940d7: PUSH EAX
0x80940d8: PUSH EBX
0x80940d9: PUSH 0x0
0x80940db: PUSH dword ptr [EBP + -0x10]
0x80940de: CALL 0x08093eb0
0x80940e3: LEA ESP, [EBP + -0xc]
0x80940e6: POP EBX
0x80940e7: POP ESI
0x80940e8: POP EDI
0x80940e9: LEAVE
0x80940ea: RET
0x80940ec: MOV EDX, dword ptr [0x086619d8]
0x80940f2: MOV EAX, dword ptr [EDX + 0x14]
0x80940f5: ADD CX, word ptr [EAX + 0x6]
0x80940f9: JS 0x08094150
0x80940fb: MOV ESI, dword ptr [EDX + 0x20]
0x80940fe: MOV EAX, dword ptr [ESI + 0x20]
0x8094101: CMP word ptr [EAX + 0x10], CX
0x8094105: MOVSX EBX, CX
0x8094108: JG 0x0809412b
0x809410a: PUSH EAX
0x809410b: MOV EAX, dword ptr [ESI + 0x20]
0x809410e: MOV EAX, dword ptr [EAX + 0x4]
0x8094111: PUSH dword ptr [EAX + 0xc]
0x8094114: MOVSX EBX, CX
0x8094117: PUSH EBX
0x8094118: PUSH 0x82ae420
0x809411d: CALL 0x08097b24
0x8094122: ADD ESP, 0x10
0x8094125: MOV EDX, dword ptr [0x086619d8]
0x809412b: MOV EAX, dword ptr [EDX + 0x20]
0x809412e: MOV EAX, dword ptr [EAX + 0x20]
0x8094131: MOV EAX, dword ptr [EAX + 0x2c]
0x8094134: SUB ESP, 0xc
0x8094137: MOV EAX, dword ptr [EAX + EBX*0x8]
0x809413a: PUSH EDI
0x809413b: PUSH EAX
0x809413c: PUSH EBX
0x809413d: PUSH 0x0
0x809413f: PUSH dword ptr [EBP + -0x10]
0x8094142: CALL 0x08093eb0
0x8094147: LEA ESP, [EBP + -0xc]
0x809414a: POP EBX
0x809414b: POP ESI
0x809414c: POP EDI
0x809414d: LEAVE
0x809414e: RET
0x8094150: MOV ESI, dword ptr [EDX + 0x20]
0x8094153: PUSH EAX
0x8094154: MOV EAX, dword ptr [ESI + 0x20]
0x8094157: MOV EAX, dword ptr [EAX + 0x4]
0x809415a: PUSH dword ptr [EAX + 0xc]
0x809415d: MOVSX EBX, CX
0x8094160: PUSH EBX
0x8094161: PUSH 0x82ae420
0x8094166: CALL 0x08097b24
0x809416b: ADD ESP, 0x10
0x809416e: MOV EDX, dword ptr [0x086619d8]
0x8094174: JMP 0x0809412b
```

## 复用：vm_callInUserFun — procedure_pseudo_code

Evidence ID：`ev_c1a3155dbe1e328a1be1bd6b5903bba9209556e7b1e8084722ed8121d520ee97`

```c

void vm_callInUserFun(Vm_Index_T funOffset,int isGlobalTypeArgn)

{
  Vm_Program_T *pVVar1;
  uint isGlobal_00;
  short funOffset_00;
  int isGlobal;
  
                    /* Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg0(EAX)]
                       Unresolved local var: int type@[???]
                       Unresolved local var: uint argn@[DW_OP_reg7(EDI)] */
  isGlobal_00 = (uint)isGlobalTypeArgn >> 7 & 1;
  if (isGlobal_00 != 0) {
    if ((funOffset < 0) || (vm_pGlobalProgram->maxFunctionNo <= funOffset)) {
      vm_error("Global function (index = %d) is not existed!\n",(int)funOffset);
    }
    vm_callInUserFunBody
              (isGlobal_00,(Vm_Object_T *)0x0,funOffset,
               vm_pGlobalProgram->pFunctions[funOffset].pFun,isGlobalTypeArgn & 0x3fU);
    return;
  }
  funOffset_00 = funOffset + vm_currentContext->consititution->baseFunOffset;
  if (funOffset_00 < 0) {
    vm_error("Function (index = %d) in %s is not existed!\n",(int)funOffset_00,
             vm_currentContext->thisObject->pProgram->strName->ptr,vm_currentContext->consititution)
    ;
  }
  else {
    pVVar1 = vm_currentContext->thisObject->pProgram;
    if (pVVar1->maxFunctionNo <= funOffset_00) {
      vm_error("Function (index = %d) in %s is not existed!\n",(int)funOffset_00,
               vm_currentContext->thisObject->pProgram->strName->ptr,pVVar1);
    }
  }
  vm_callInUserFunBody
            (0,(Vm_Object_T *)0x0,funOffset_00,
             vm_currentContext->thisObject->pProgram->pFunctions[funOffset_00].pFun,
             isGlobalTypeArgn & 0x3fU);
  return;
}
```

## 复用：vm_callInUserFunBody — procedure_assembly

Evidence ID：`ev_86daecb2fd9196b4601aa0310cad30717ea79495df131989c6e15dbaccad6a41`

```asm
0x8093eb0: PUSH EBP
0x8093eb1: MOV EBP, ESP
0x8093eb3: PUSH EDI
0x8093eb4: PUSH ESI
0x8093eb5: PUSH EBX
0x8093eb6: SUB ESP, 0xc
0x8093eb9: MOV EAX, dword ptr [EBP + 0x10]
0x8093ebc: MOV word ptr [EBP + -0xe], AX
0x8093ec0: MOV EAX, [0x086619c0]
0x8093ec5: CMP EAX, dword ptr [0x086619c4]
0x8093ecb: MOV ESI, dword ptr [EBP + 0xc]
0x8093ece: MOV EDI, dword ptr [EBP + 0x14]
0x8093ed1: JGE 0x08094020
0x8093ed7: TEST ESI, ESI
0x8093ed9: JZ 0x08093ee5
0x8093edb: TEST byte ptr [ESI + 0xd], 0x80
0x8093edf: JNZ 0x08094038
0x8093ee5: MOV EAX, dword ptr [EDI + 0x14]
0x8093ee8: TEST EAX, EAX
0x8093eea: JNZ 0x08093f18
0x8093eec: MOV EBX, dword ptr [EBP + 0x8]
0x8093eef: TEST EBX, EBX
0x8093ef1: JNZ 0x08094050
0x8093ef7: TEST ESI, ESI
0x8093ef9: JZ 0x08094070
0x8093eff: PUSH ECX
0x8093f00: PUSH dword ptr [EDI]
0x8093f02: MOV EAX, dword ptr [ESI + 0x20]
0x8093f05: MOV EAX, dword ptr [EAX + 0x4]
0x8093f08: PUSH dword ptr [EAX + 0xc]
0x8093f0b: PUSH 0x82ae33c
0x8093f10: CALL 0x08097b24
0x8093f15: ADD ESP, 0x10
0x8093f18: MOV EDX, dword ptr [0x0860ae64]
0x8093f1e: TEST EDX, EDX
0x8093f20: JNZ 0x08094014
0x8093f26: CALL 0x080a9310
0x8093f2b: MOV EBX, EAX
0x8093f2d: INC dword ptr [0x086619c0]
0x8093f33: CALL 0x0809789c
0x8093f38: MOV EDX, dword ptr [0x086619d8]
0x8093f3e: MOV EAX, [0x086619cc]
0x8093f43: MOV dword ptr [EDX], EAX
0x8093f45: MOV EAX, dword ptr [EBP + 0x18]
0x8093f48: MOV dword ptr [EDX + 0x8], 0x4
0x8093f4f: MOV dword ptr [EDX + 0x4], EAX
0x8093f52: MOV dword ptr [EDX + 0x24], EDI
0x8093f55: TEST byte ptr [EBX + 0x29], 0x1
0x8093f59: JZ 0x08093f62
0x8093f5b: MOV dword ptr [EDX + 0x8], 0xc
0x8093f62: TEST byte ptr [EDI + 0x8], 0x2
0x8093f66: JZ 0x08093f6f
0x8093f68: OR dword ptr [EBX + 0x28], 0x100
0x8093f6f: MOV EAX, dword ptr [EBP + 0x8]
0x8093f72: TEST EAX, EAX
0x8093f74: JZ 0x08093fb0
0x8093f76: MOV EBX, dword ptr [0x086619d8]
0x8093f7c: MOV dword ptr [EBX + 0x20], 0x0
0x8093f83: OR dword ptr [EBX + 0x8], 0x10
0x8093f87: MOV EAX, [0x0847aad0]
0x8093f8c: MOV EAX, dword ptr [EAX + 0x18]
0x8093f8f: MOV dword ptr [EBX + 0x14], EAX
0x8093f92: MOV EAX, [0x0860ae64]
0x8093f97: TEST EAX, EAX
0x8093f99: JNZ 0x08093ff4
0x8093f9b: LEA EAX, [EDI + 0x10]
0x8093f9e: MOV dword ptr [EBX + 0xc], 0x0
0x8093fa5: MOV dword ptr [EBX + 0x10], EAX
0x8093fa8: LEA ESP, [EBP + -0xc]
0x8093fab: POP EBX
0x8093fac: POP ESI
0x8093fad: POP EDI
0x8093fae: LEAVE
0x8093faf: RET
0x8093fb0: TEST ESI, ESI
0x8093fb2: JZ 0x0809405f
0x8093fb8: MOV EBX, dword ptr [0x086619d8]
0x8093fbe: MOV dword ptr [EBX + 0x20], ESI
0x8093fc1: OR dword ptr [EBX + 0x8], 0x10
0x8093fc5: MOV ESI, dword ptr [EBX + 0x20]
0x8093fc8: TEST ESI, ESI
0x8093fca: JZ 0x08093f87
0x8093fcc: MOV ECX, dword ptr [ESI + 0x20]
0x8093fcf: MOVSX EDX, word ptr [EBP + -0xe]
0x8093fd3: MOV EAX, dword ptr [ECX + 0x2c]
0x8093fd6: MOVSX EAX, word ptr [EAX + EDX*0x8 + 0x4]
0x8093fdb: LEA EAX, [EAX + EAX*0x2]
0x8093fde: MOV EDX, dword ptr [ECX + 0x18]
0x8093fe1: LEA EAX, [EDX + EAX*0x4]
0x8093fe4: MOV dword ptr [EBX + 0x14], EAX
0x8093fe7: MOV EAX, [0x0860ae64]
0x8093fec: INC dword ptr [ESI + 0x4]
0x8093fef: TEST EAX, EAX
0x8093ff1: JZ 0x08093f9b
0x8093ff3: NOP
0x8093ff4: CALL 0x080bb0e8
0x8093ff9: MOV EBX, dword ptr [0x086619d8]
0x8093fff: LEA EAX, [EDI + 0x10]
0x8094002: MOV dword ptr [EBX + 0xc], 0x0
0x8094009: MOV dword ptr [EBX + 0x10], EAX
0x809400c: LEA ESP, [EBP + -0xc]
0x809400f: POP EBX
0x8094010: POP ESI
0x8094011: POP EDI
0x8094012: LEAVE
0x8094013: RET
0x8094014: CALL 0x080bb0d4
0x8094019: JMP 0x08093f26
0x8094020: SUB ESP, 0x8
0x8094023: PUSH EAX
0x8094024: PUSH 0x82ae31c
0x8094029: CALL 0x08097b24
0x809402e: ADD ESP, 0x10
0x8094031: JMP 0x08093ed7
0x8094038: SUB ESP, 0xc
0x809403b: PUSH 0x82ae368
0x8094040: CALL 0x08097b24
0x8094045: ADD ESP, 0x10
0x8094048: JMP 0x08093ee5
0x8094050: SUB ESP, 0x8
0x8094053: PUSH dword ptr [EDI]
0x8094055: PUSH 0x82ae398
0x809405a: JMP 0x08093f10
0x809405f: MOV EBX, dword ptr [0x086619d8]
0x8094065: MOV EAX, dword ptr [EBX + -0x8]
0x8094068: MOV dword ptr [EBX + 0x20], EAX
0x809406b: JMP 0x08093fc5
0x8094070: SUB ESP, 0x8
0x8094073: PUSH dword ptr [EDI]
0x8094075: PUSH 0x82ae3c8
0x809407a: JMP 0x08093f10
```

## 复用：vm_callInUserFunBody — procedure_pseudo_code

Evidence ID：`ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`

```c

void vm_callInUserFunBody
               (int isGlobal,Vm_Object_T *pToObject,Vm_Index_T funOffset,Vm_Fun_T *pFun,int argn)

{
  Vm_Object *pVVar1;
  int iVar2;
  Vm_FunCall_Context_T *pVVar3;
  Vm_PCB_T *pVVar4;
  char *str;
  char *pcVar5;
  Vm_Index_T funOffset_local;
  
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg3(EBX)] */
  if (vm_funContextSize <= vm_funCp) {
    vm_error("Too depth (%d) function call!\n",vm_funCp);
  }
  if ((pToObject != (Vm_Object_T *)0x0) && ((pToObject->flags & 0x8000) != 0)) {
    vm_error("Object is not found (when call in function).\n");
  }
  if ((pFun->codeSegment).nodes == (Vm_Node_T *)0x0) {
    if (isGlobal == 0) {
      if (pToObject == (Vm_Object_T *)0x0) {
        pcVar5 = pFun->name;
        str = "Function \"%s\" is not implemented yet!\n";
      }
      else {
        pcVar5 = pToObject->pProgram->strName->ptr;
        str = "Function \"%s::%s\" is not implemented yet!\n";
      }
    }
    else {
      pcVar5 = pFun->name;
      str = "Global function \"%s\" is not implemented yet!\n";
    }
    vm_error(str,pcVar5);
  }
  if (vm_gatheringCallStackInfo != 0) {
    vm_enterPPCopyStackCriticalSection();
  }
  pVVar4 = vm_getCurrentPCB();
  vm_funCp = vm_funCp + 1;
  vm_updateCurrentFunCallContext();
  pVVar3 = vm_currentContext;
  vm_currentContext->bp = vm_sp;
  pVVar3->flags = 4;
  pVVar3->argNum = argn;
  (pVVar3->f).pEfun = (Vm_Efun *)pFun;
  if ((pVVar4->flags & 0x100) != 0) {
    pVVar3->flags = 0xc;
  }
  if ((pFun->attrib & 2) != 0) {
    pVVar4->flags = pVVar4->flags | 0x100;
  }
  pVVar3 = vm_currentContext;
  if (isGlobal == 0) {
    if (pToObject == (Vm_Object_T *)0x0) {
      vm_currentContext->thisObject = vm_currentContext[-1].thisObject;
    }
    else {
      vm_currentContext->thisObject = pToObject;
      pVVar3->flags = pVVar3->flags | 0x10;
    }
    pVVar1 = pVVar3->thisObject;
    if (pVVar1 != (Vm_Object *)0x0) {
                    /* Unresolved local var: Vm_Fun_Entry_T * pFunEntry@[???] */
      pVVar3->consititution =
           pVVar1->pProgram->pConsititutions +
           pVVar1->pProgram->pFunctions[funOffset].consititutionNo;
      iVar2 = vm_gatheringCallStackInfo;
      pVVar1->execRef = pVVar1->execRef + 1;
      goto joined_r0x08093ff1;
    }
  }
  else {
    vm_currentContext->thisObject = (Vm_Object *)0x0;
    pVVar3->flags = pVVar3->flags | 0x10;
  }
  pVVar3->consititution = vm_pGlobalProgram->pConsititutions;
  iVar2 = vm_gatheringCallStackInfo;
joined_r0x08093ff1:
  if (iVar2 == 0) {
    pVVar3->ip = 0;
    pVVar3->cs = &pFun->codeSegment;
    return;
  }
  vm_leavePPCopyStackCriticalSection();
  pVVar3 = vm_currentContext;
  vm_currentContext->ip = 0;
  pVVar3->cs = &pFun->codeSegment;
  return;
}
```

## 复用：vm_finalizeFun — procedure_pseudo_code

Evidence ID：`ev_ac11d24ef0dd2c2398a02826fd57a977d4381fa3668add2ab2c2553424e05abd`

```c

void vm_finalizeFun(Vm_Fun_T *pFun,Vm_Index_T funOffset,int halfFinalized)

{
  Vm_Fun_Entry_T *pVVar1;
  Vm_Program *pVVar2;
  Vm_Fun_Arg_T *pVVar3;
  int halfFinalized_local;
  
                    /* Unresolved local var: Vm_Fun_Arg_T * pVar@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_Entry_T * pFunEntry@[DW_OP_reg2(EDX)] */
  vm_finalizeCodeSegment(&pFun->codeSegment);
  for (pVVar3 = pFun->pVars; pVVar3 != (Vm_Fun_Arg_T *)0x0; pVVar3 = pVVar3->pNext) {
    vm_scratchFinalize(pVVar3->name);
    vm_scratchFinalize(pVVar3);
  }
  pFun->pLabels = (Vm_Local_Label_T *)0x0;
  pVVar2 = vm_compilingProgram();
  pVVar1 = pVVar2->pFunctions;
  pVVar1[funOffset].consititutionNo = 0;
  pVVar1[funOffset].pFun = pFun;
  if (halfFinalized == 0) {
    for (pVVar3 = pFun->pArgs; pVVar3 != (Vm_Fun_Arg_T *)0x0; pVVar3 = pVVar3->pNext) {
      pVVar3->pDefaultValue = (Vm_XNode *)0x0;
      vm_scratchFinalize(pVVar3->name);
      vm_scratchFinalize(pVVar3);
    }
    vm_scratchFinalize(pFun->name);
    vm_scratchFinalize(pFun);
    return;
  }
  return;
}
```

## 复用：vm_loadBinaryInherit — analyze_function

Evidence ID：`ev_c41c22c1afd1192242c38bb59ffa5cc3402d157828307978e735721a4489e3d0`

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

## 复用：vm_loadIdentifiers — analyze_function

Evidence ID：`ev_b786ac699461a76c0683ddb2f8254575187db56f002088762956ac65e57e9528`

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

## 复用：vm_loadInheritPrograms — analyze_function

Evidence ID：`ev_af5958529d07c6f0f0b9b287dbdec72c60a8ad5632f6fd5b362a60f8d3a4b489`

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

## 复用：vm_returnFromFun — procedure_assembly

Evidence ID：`ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610`

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

## 复用：vm_returnFromFun — procedure_pseudo_code

Evidence ID：`ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2`

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
