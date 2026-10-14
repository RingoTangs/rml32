# 成员引用与赋值：Ghidra 原始证据

目标 SHA-256：`5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34`。仅静态分析，原始制品未执行。伪代码不是原始源码；联合体类型混淆以相应汇编及调用引用为准。

## vm_.*(Garbage|garbage|Collect|collect|Reference|reference|RefValue|refValue) / search_procedures

Evidence: `ev_8177ba2a26295389b9842e60d0b8fc4ea67f984b2b3179aad5767a9252a4b898`

```json
[
  {
    "address": "0x805077c",
    "value": "vm_cmdCollectData"
  },
  {
    "address": "0x806d390",
    "value": "vm_cmdAntiHackCollect"
  },
  {
    "address": "0x808bfb4",
    "value": "vm_unrefValue"
  },
  {
    "address": "0x80f9628",
    "value": "vm_f_garbage_collect"
  }
]
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Regex mode uses Java Pattern semantics. Matcher or compiler stack exhaustion returns a resource constraint without closing the session; literal mode avoids regex recursion and returns complete matching values.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_.*(Pointer|Ptr|Member|Assign|Deref) / search_procedures

Evidence: `ev_d16b4b00f73d929ced432c867b790ea3bbacaac1dfc853d88455c37d441b4c66`

```json
[
  {
    "address": "0x80506dc",
    "value": "vm_cmdAssignAttrib"
  },
  {
    "address": "0x8050848",
    "value": "vm_cmdAssignPolar"
  },
  {
    "address": "0x8050918",
    "value": "vm_cmdAssignUpgrade"
  },
  {
    "address": "0x805099c",
    "value": "vm_cmdAssignResist"
  },
  {
    "address": "0x8055b58",
    "value": "vm_cmdMulAssignAttrib"
  },
  {
    "address": "0x8055e38",
    "value": "vm_cmdPreAssignAttrib"
  },
  {
    "address": "0x8055f08",
    "value": "vm_cmdPreAssignChildAttrib"
  },
  {
    "address": "0x80568bc",
    "value": "vm_cmdPartyMembers"
  },
  {
    "address": "0x8056a2c",
    "value": "vm_cmdPartyModifyMember"
  },
  {
    "address": "0x80598e0",
    "value": "vm_cmdTeamSeekMember"
  },
  {
    "address": "0x805d4e4",
    "value": "vm_cmdRaidKickoutMember"
  },
  {
    "address": "0x805ebb4",
    "value": "vm_cmdLocalEncryptRequest"
  },
  {
    "address": "0x806cde8",
    "value": "vm_cmdAssignChildAttrib"
  },
  {
    "address": "0x8071d0c",
    "value": "vm_cmdInviteChatGroupMember"
  },
  {
    "address": "0x8071de0",
    "value": "vm_cmdRemoveChatGroupMember"
  },
  {
    "address": "0x8085e34",
    "value": "vm_tryAssign"
  },
  {
    "address": "0x8085fd0",
    "value": "vm_tryAssignLVInfo"
  },
  {
    "address": "0x808a220",
    "value": "vm_convertToPtr"
  },
  {
    "address": "0x808e914",
    "value": "vm_assignValueReverse"
  },
  {
    "address": "0x808f634",
    "value": "vm_assignValue"
  },
  {
    "address": "0x808f8e4",
    "value": "vm_queryMemberPtr"
  },
  {
    "address": "0x808fa60",
    "value": "vm_queryMember"
  },
  {
    "address": "0x809005c",
    "value": "vm_queryRangePtr"
  },
  {
    "address": "0x8091014",
    "value": "vm_pushVptr"
  },
  {
    "address": "0x80910d0",
    "value": "vm_pushGlobalVptr"
  },
  {
    "address": "0x80911a8",
    "value": "vm_pushObjVptr"
  },
  {
    "address": "0x809134c",
    "value": "vm_pushLocalVptr"
  },
  {
    "address": "0x80915dc",
    "value": "vm_pushLocalVptrI"
  },
  {
    "address": "0x809441c",
    "value": "vm_callFunPtr"
  },
  {
    "address": "0x80983fc",
    "value": "vm_createEntryFunPtrFromName"
  },
  {
    "address": "0x80984e8",
    "value": "vm_createEntryFunPtrFromFun"
  },
  {
    "address": "0x80a3088",
    "value": "vm_makeNodesFromValuePtrInternal"
  },
  {
    "address": "0x80a318c",
    "value": "vm_makeValueNodesFromValuePtr"
  },
  {
    "address": "0x80a31a4",
    "value": "vm_makeVPtrNodesFromValuePtr"
  },
  {
    "address": "0x80a906c",
    "value": "vm_setForceScriptRelinquish"
  },
  {
    "address": "0x80a907c",
    "value": "vm_getForceScriptRelinquish"
  },
  {
    "address": "0x80d6918",
    "value": "vm_f_get_pointer"
  },
  {
    "address": "0x80d895c",
    "value": "vm_f_member_array"
  }
]
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Regex mode uses Java Pattern semantics. Matcher or compiler stack exhaustion returns a resource constraint without closing the session; literal mode avoids regex recursion and returns complete matching values.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_.*(StaticValue|MapNode|Free.*Value|ObjectValue|DeleteObject|DestructObject|clean.*Value) / search_procedures

Evidence: `ev_10ec00cdbd757643a40a3566d8ef96a160f68c5f768182dce3ebc466586294fc`

```json
[
  {
    "address": "0x808ad98",
    "value": "vm_cleanupValue"
  },
  {
    "address": "0x808bf44",
    "value": "vm_freeValue"
  },
  {
    "address": "0x808c0b8",
    "value": "vm_destructStaticValue"
  },
  {
    "address": "0x808c114",
    "value": "vm_freeFunctionValue"
  },
  {
    "address": "0x8091fec",
    "value": "vm_freeStackValue"
  },
  {
    "address": "0x8092adc",
    "value": "vm_freeStackValueOfPCB"
  },
  {
    "address": "0x809f24c",
    "value": "vm_freeObsoletedMapNodes"
  },
  {
    "address": "0x809f298",
    "value": "vm_getObsoletedMapNodesCount"
  },
  {
    "address": "0x80b11a8",
    "value": "vm_destructObjectValues"
  },
  {
    "address": "0x80b1894",
    "value": "vm_destructObject"
  },
  {
    "address": "0x80bc884",
    "value": "vm_freeLoopValue"
  },
  {
    "address": "0x80d4dfc",
    "value": "vm_f_clean_mapping_by_value"
  }
]
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Regex mode uses Java Pattern semantics. Matcher or compiler stack exhaustion returns a resource constraint without closing the session; literal mode avoids regex recursion and returns complete matching values.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_assignValue / procedure_assembly

Evidence: `ev_fd0c36e7c899550aed3734401ee4c11b3d606fd07bb637b33985c741a0822dcd`

```asm
0x808f634: PUSH EBP
0x808f635: MOV EAX, [0x086619cc]
0x808f63a: MOV EBP, ESP
0x808f63c: MOV EDX, dword ptr [0x08661964]
0x808f642: PUSH ESI
0x808f643: LEA EAX, [EAX + EAX*0x2]
0x808f646: LEA EAX, [EDX + EAX*0x8]
0x808f649: PUSH EBX
0x808f64a: MOV EBX, dword ptr [EAX + 0x8]
0x808f64d: SUB ESP, 0xc
0x808f650: PUSH EBX
0x808f651: LEA ESI, [EAX + 0x18]
0x808f654: CALL 0x0808ad98
0x808f659: POP EAX
0x808f65a: POP EDX
0x808f65b: PUSH ESI
0x808f65c: PUSH EBX
0x808f65d: CALL 0x0808be24
0x808f662: MOV dword ptr [ESP], EBX
0x808f665: CALL 0x0808bfb4
0x808f66a: MOV EAX, [0x086619cc]
0x808f66f: MOV EDX, dword ptr [0x08661964]
0x808f675: LEA EAX, [EAX + EAX*0x2]
0x808f678: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x808f67d: MOV EAX, [0x086619cc]
0x808f682: MOV EDX, dword ptr [0x08661964]
0x808f688: LEA EAX, [EAX + EAX*0x2]
0x808f68b: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x808f690: INC dword ptr [0x086619cc]
0x808f696: ADD ESP, 0x10
0x808f699: LEA ESP, [EBP + -0x8]
0x808f69c: POP EBX
0x808f69d: POP ESI
0x808f69e: LEAVE
0x808f69f: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-assignvalue"></a>

## vm_assignValue / procedure_pseudo_code

Evidence: `ev_16636c23591e5a4b1f66e2d639b1da8c96fca5404ca5dc6a73b1eb3f8974a1c1`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_assignValue(void)

{
  Vm_Value_T *pValue;
  Vm_Value_T *pValue_00;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Value_T * pValue2@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg3(EBX)] */
  pValue_00 = (Vm_Value_T *)vm_stack[vm_sp].v.string;
  pValue = vm_stack + vm_sp + 1;
  vm_cleanupValue(pValue_00);
  vm_copyValue(pValue_00,pValue);
  vm_unrefValue(pValue_00);
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

## vm_assignValue / procedure_references

Evidence: `ev_1ef9f117ba127c47d0d8cb76f713d59b197a5473effc57b7b78809b9b9da043b`

```json
{
  "direction": "incoming",
  "procedure": {
    "address": "0x808f634",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": false,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x808f69f",
          "start": "0x808f634"
        }
      ],
      "span_bytes": 108,
      "total_bytes": 108
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_assignValue"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x83cc6dc",
      "source_procedure": null,
      "target_address": "0x808f634",
      "target_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": ".debug_frame:0xb5e8",
      "source_procedure": null,
      "target_address": "0x808f634",
      "target_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
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

## vm_assignValue / procedure_references

Evidence: `ev_5de76295204efbad32f935d50b261520d84c3ee4fea3df079e8905544dcbf20d`

```json
{
  "direction": "outgoing",
  "procedure": {
    "address": "0x808f634",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": false,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x808f69f",
          "start": "0x808f634"
        }
      ],
      "span_bytes": 108,
      "total_bytes": 108
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_assignValue"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f635",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x86619cc",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f63c",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x8661964",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f654",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x808ad98",
      "target_procedure": {
        "address": "0x808ad98",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808afc4",
              "start": "0x808ad98"
            }
          ],
          "span_bytes": 557,
          "total_bytes": 557
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_cleanupValue"
      }
    },
    {
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
      },
      "source_address": "0x808f65d",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x808be24",
      "target_procedure": {
        "address": "0x808be24",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808bee8",
              "start": "0x808be24"
            }
          ],
          "span_bytes": 197,
          "total_bytes": 197
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_copyValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f662",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "stack:0xffffffffffffffe4",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f665",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x808bfb4",
      "target_procedure": {
        "address": "0x808bfb4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c005",
              "start": "0x808bfb4"
            },
            {
              "end": "0x808c026",
              "start": "0x808c008"
            }
          ],
          "span_bytes": 115,
          "total_bytes": 113
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_unrefValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f66a",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x86619cc",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f66f",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x8661964",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f67d",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x86619cc",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f682",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x8661964",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ_WRITE",
        "write": true
      },
      "source_address": "0x808f690",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "0x86619cc",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f699",
      "source_procedure": {
        "address": "0x808f634",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f69f",
              "start": "0x808f634"
            }
          ],
          "span_bytes": 108,
          "total_bytes": 108
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_assignValue"
      },
      "target_address": "stack:0xfffffffffffffff4",
      "target_procedure": null
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

## vm_assignValueReverse / procedure_assembly

Evidence: `ev_b5106f08b96c169b0aeaaf83650f4673c60f685fe9f1195bc3f5c867f90d0289`

```asm
0x808e914: PUSH EBP
0x808e915: MOV EBP, ESP
0x808e917: PUSH EDI
0x808e918: PUSH ESI
0x808e919: PUSH EBX
0x808e91a: SUB ESP, 0x18
0x808e91d: MOV EBX, dword ptr [0x086619cc]
0x808e923: MOV EAX, [0x08661964]
0x808e928: LEA EBX, [EBX + EBX*0x2]
0x808e92b: LEA EBX, [EAX + EBX*0x8]
0x808e92e: LEA EDI, [EBX + 0x18]
0x808e931: MOV ESI, dword ptr [EDI + 0x8]
0x808e934: PUSH ESI
0x808e935: CALL 0x0808ad98
0x808e93a: POP EAX
0x808e93b: POP EDX
0x808e93c: PUSH EBX
0x808e93d: PUSH ESI
0x808e93e: CALL 0x0808be24
0x808e943: MOV dword ptr [ESP], ESI
0x808e946: CALL 0x0808bfb4
0x808e94b: MOV AL, byte ptr [EBX + 0x4]
0x808e94e: MOV byte ptr [EDI + 0x4], AL
0x808e951: MOV AL, byte ptr [EBX + 0x5]
0x808e954: MOV byte ptr [EDI + 0x5], AL
0x808e957: MOV EAX, dword ptr [EBX + 0x8]
0x808e95a: MOV dword ptr [EDI + 0x8], EAX
0x808e95d: MOV EAX, [0x086619cc]
0x808e962: MOV EDX, dword ptr [0x08661964]
0x808e968: LEA EAX, [EAX + EAX*0x2]
0x808e96b: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x808e970: MOV EAX, [0x086619cc]
0x808e975: MOV EDX, dword ptr [0x08661964]
0x808e97b: LEA EAX, [EAX + EAX*0x2]
0x808e97e: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x808e983: INC dword ptr [0x086619cc]
0x808e989: ADD ESP, 0x10
0x808e98c: LEA ESP, [EBP + -0xc]
0x808e98f: POP EBX
0x808e990: POP ESI
0x808e991: POP EDI
0x808e992: LEAVE
0x808e993: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-assignvaluereverse"></a>

## vm_assignValueReverse / procedure_pseudo_code

Evidence: `ev_5c9e1932bde415d90274f091f4ad7eb199e7e274a93bce3e2234d0a63fad447a`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_assignValueReverse(void)

{
  Vm_Value_T *pValue;
  Vm_Value_T *pValue_00;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg6(ESI)] */
  pValue = vm_stack + vm_sp;
  pValue_00 = (Vm_Value_T *)pValue[1].v.string;
  vm_cleanupValue(pValue_00);
  vm_copyValue(pValue_00,pValue);
  vm_unrefValue(pValue_00);
  pValue[1].type = pValue->type;
  pValue[1].attrib = pValue->attrib;
  pValue[1].v = pValue->v;
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

<a id="vm-callinuserfunbody"></a>

## vm_callInUserFunBody / procedure_pseudo_code

Evidence: `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-checkargnum"></a>

## vm_checkArgNum / procedure_pseudo_code

Evidence: `ev_e83a3872c63d02bfe316640f32f5cd3c2955f72877435e0af4e8fa0cdb14d9a9`

```c

void vm_checkArgNum(int nTotalAction)

{
  Vm_Value_T *pValue;
  Vm_Fun *pVVar1;
  Vm_Fun_Arg_T *pVVar2;
  int iVar3;
  Vm_FunCall_Context_T *pVVar4;
  uchar uVar5;
  uint type;
  char *pcVar6;
  char *pcVar7;
  int iVar8;
  int iVar9;
  uchar uVar10;
  int local_1c;
  Vm_Fun_T *pFun;
  Vm_FunCall_Context_T *pContext;
  
  pVVar4 = vm_funContext;
                    /* Unresolved local var: Vm_Fun_Arg_T * pArg@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg7(EDI)]
                       Unresolved local var: int nActionNodes@[DW_OP_reg1(ECX)]
                       Unresolved local var: int i@[DW_OP_reg6(ESI)]
                       Unresolved local var: int gotType@[DW_OP_reg2(EDX)] */
  iVar3 = vm_funCp;
  pVVar1 = vm_funContext[vm_funCp].f.pFun;
  pVVar2 = pVVar1->pArgs;
  iVar9 = 0;
  do {
    if ((pVVar2 == (Vm_Fun_Arg_T *)0x0) || (pVVar4[iVar3].argNum <= iVar9)) {
      if (iVar9 < pVVar1->minArgNo) {
        vm_error("No input argument %d for \"%s\" of %s.\n",iVar9 + 1,pVVar2->name,pVVar1->name);
      }
      iVar8 = pVVar4[iVar3].argNum;
      if (iVar8 < pVVar1->maxArgNo) {
        iVar9 = 0;
        for (pVVar2 = pVVar1->pArgs; (pVVar2 != (Vm_Fun_Arg_T *)0x0 && (pVVar2->no < iVar8));
            pVVar2 = pVVar2->pNext) {
          iVar9 = iVar9 + pVVar2->defaultActionNum;
        }
        vm_currentContext->ip = vm_currentContext->ip + iVar9;
        return;
      }
      if ((iVar9 < iVar8) && ((pVVar1->attrib & 1) == 0)) {
        vm_error("Too many arguments (%d) passed to the function %s.\n",pVVar4[iVar3].argNum,
                 pVVar1->name,iVar8);
      }
      vm_currentContext->ip = vm_currentContext->ip + nTotalAction;
      return;
    }
    pValue = vm_stack + vm_sp + iVar9;
    if ((pVVar2->flag & 1) == 0) {
      if ((pVVar2->varAttrib & 2) != 0) {
        vm_convertValueToConst(pValue);
      }
      uVar5 = pVVar2->type;
      if (uVar5 != 0xff) {
        uVar10 = pValue->type;
        if (uVar10 == '\b') {
          vm_error("Argument %d for \"%s\" of %s got a reference value.\n",iVar9 + 1,pVVar2->name,
                   pVVar1->name);
          uVar10 = pValue->type;
          uVar5 = pVVar2->type;
        }
        local_1c = iVar9 + 1;
        if (uVar10 == uVar5) {
          if (((uVar10 == '\n') && (((uint)((pValue->v).string)->ptr & 0x8000) != 0)) &&
             ((pVVar2->varAttrib & 8) == 0)) {
            vm_error("Type mismatched for argument %d \"%s\" of %s.\nExpected object, got undefined (object is destructed).\n"
                     ,local_1c,pVVar2->name,pVVar1->name);
          }
        }
        else if (uVar5 == '\x03') {
          if (uVar10 != '\x02') goto LAB_08095377;
          (pValue->v).real = (float)(pValue->v).number;
          pValue->type = '\x03';
        }
        else if ((uVar5 == '\x02') && (uVar10 == '\x03')) {
          (pValue->v).number = (int)(pValue->v).real;
          pValue->type = '\x02';
        }
        else {
LAB_08095377:
          iVar8 = vm_getVarType(pValue);
          if ((iVar8 != 0) || ((pVVar2->varAttrib & 8) == 0)) {
            type = (uint)pValue->type;
            goto LAB_080952c0;
          }
        }
      }
    }
    else {
      if (pValue->type != '\b') {
        vm_error("Argument %d for \"%s\" of %s need reference type, but got a normal value.\n",
                 iVar9 + 1,pVVar2->name,pVVar1->name);
      }
      if (((pVVar2->type != 0xff) && (type = vm_getValueType(pValue), type != pVVar2->type)) &&
         ((type != 0 || ((pVVar2->varAttrib & 8) == 0)))) {
LAB_080952c0:
        local_1c = iVar9 + 1;
        pcVar6 = vm_valueType(type);
        pcVar7 = vm_valueType((uint)pVVar2->type);
        vm_error("Type mismatched for argument %d \"%s\" of %s.\nExpected %s, got %s.\n",local_1c,
                 pVVar2->name,pVVar1->name,pcVar7,pcVar6);
      }
    }
    local_1c = iVar9 + 1;
    pVVar2 = pVVar2->pNext;
    iVar9 = local_1c;
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

<a id="vm-checkisreftype"></a>

## vm_checkIsRefType / procedure_pseudo_code

Evidence: `ev_4f2bd1197ee3cfcc577df7fb16ae4e1619badb4b73e7dda33475e7566f15ed0e`

```c

void vm_checkIsRefType(int no)

{
  int iVar1;
  anon_union_4_11_5000f750_for_v aVar2;
  
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
    iVar1 = vm_argumentInThisContext();
  }
  else {
    iVar1 = vm_argumentInThisContext();
  }
  if (no < iVar1) {
    iVar1 = vm_stackInThisContext();
    if (vm_sp <= no + iVar1) {
      aVar2.number._1_3_ = 0;
      aVar2.number._0_1_ = vm_stack[no + iVar1].type == '\b';
      if (vm_sp == 0) {
        vm_error("Stack overflow when calculating.\n");
      }
      vm_sp = vm_sp + -1;
      vm_stack[vm_sp].type = '\x02';
      vm_stack[vm_sp].v = aVar2;
      return;
    }
  }
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\x02';
  vm_stack[vm_sp].v.info = (void *)0x0;
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

<a id="vm-checkisreftypei"></a>

## vm_checkIsRefTypeI / procedure_pseudo_code

Evidence: `ev_3abfd431032fdb9b3462a59209ad2de9c1b0a205fdeec563e0f158198fa8ddef`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_checkIsRefTypeI(void)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int no@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_getValueFromStack(0,0xff);
  if (pVVar1->type == '\x03') {
    iVar2 = _vm_ftol((pVVar1->v).real);
    (pVVar1->v).number = iVar2;
    pVVar1->type = '\x02';
  }
  else if (pVVar1->type != '\x02') {
    vm_error("Bad index for argument.\n");
  }
  iVar2 = (pVVar1->v).number;
  vm_cleanupValue(vm_stack + vm_sp);
  vm_sp = vm_sp + 1;
  vm_checkIsRefType(iVar2 + -1);
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

<a id="vm-cleanupvalue"></a>

## vm_cleanupValue / procedure_pseudo_code

Evidence: `ev_6eecd8e699df5eefd7a3a4825b0b2c554b2844c7bf1228db97aa0803c3d8900e`

```c

void vm_cleanupValue(Vm_Value_T *pValue)

{
  ushort uVar1;
  Vm_Value *pVVar2;
  int iVar3;
  Vm_Value_T *pVVar4;
  Vm_Value *pVVar5;
  uint line;
  int local_18;
  int i;
  
  switch(pValue->type) {
  case '\0':
  case '\x01':
  case '\x02':
  case '\x03':
    break;
  case '\x04':
    pVVar5 = (pValue->v).pPtrValue;
    iVar3 = pVVar5->ref + -1;
    pVVar5->ref = iVar3;
    if (0 < iVar3) break;
    if ((*(uint *)&pVVar5->type & 1) != 0) {
      vm_removeSharedString((Vm_String_T *)pVVar5);
      break;
    }
    line = 0x515;
    goto LAB_0808af83;
  case '\x05':
    vm_freeBuffer((Vm_Buffer_T *)(pValue->v).string);
    break;
  case '\x06':
    vm_unrefArray((Vm_Array_T *)(pValue->v).string);
    break;
  case '\a':
    vm_unrefMapping((Vm_Mapping_T *)(pValue->v).string);
    break;
  case '\b':
    pVVar4 = (pValue->v).pPtrValue;
    iVar3 = pVVar4->ref + -1;
    pVVar4->ref = iVar3;
    if ((iVar3 == 0) && (pVVar4 != (Vm_Value_T *)0x0)) {
      vm_cleanupValue(pVVar4);
      vm_totalValues = vm_totalValues + -1;
      if (pVVar4 == vm_allValues) {
        vm_allValues = pVVar4->pNext;
        if (vm_allValues != (Vm_Value *)0x0) {
          vm_allValues->pPrev = (Vm_Value *)0x0;
        }
      }
      else {
        pVVar5 = pVVar4->pPrev;
        pVVar5->pNext = pVVar4->pNext;
        if (pVVar4->pNext != (Vm_Value *)0x0) {
          pVVar4->pNext->pPrev = pVVar5;
        }
      }
      pVVar4->pThis = (Vm_Value *)0x0;
      pdb_freeTinyBlock(pVVar4);
    }
    break;
  case '\t':
    pVVar5 = (Vm_Value *)(pValue->v).function;
    iVar3 = pVVar5->ref + -1;
    pVVar5->ref = iVar3;
    if (0 < iVar3) break;
    i = 0;
    if (0 < (short)pVVar5->active) {
      local_18 = 0;
      do {
        pVVar4 = (Vm_Value_T *)((int)&pVVar5->pThis->ref + local_18);
        pVVar4->ref = pVVar4->ref + -1;
        vm_cleanupValue(pVVar4);
        vm_totalValues = vm_totalValues + -1;
        if (pVVar4 == vm_allValues) {
          vm_allValues = pVVar4->pNext;
          if (vm_allValues != (Vm_Value_T *)0x0) {
            vm_allValues->pPrev = (Vm_Value *)0x0;
          }
        }
        else {
          pVVar2 = pVVar4->pPrev;
          pVVar2->pNext = pVVar4->pNext;
          if (pVVar4->pNext != (Vm_Value *)0x0) {
            pVVar4->pNext->pPrev = pVVar2;
          }
        }
        i = i + 1;
        uVar1 = pVVar5->active;
        local_18 = local_18 + 0x18;
        pVVar4->pThis = (Vm_Value *)0x0;
      } while (i < (short)uVar1);
    }
    if ((Vm_Object_T *)pVVar5->pNext != (Vm_Object_T *)0x0) {
      vm_unrefObject((Vm_Object_T *)pVVar5->pNext);
    }
    pdb_freeMemory(pVVar5->pThis,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x117);
    line = 0x118;
LAB_0808af83:
    pdb_freeMemory(pVVar5,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",line);
    break;
  case '\n':
    vm_unrefObject((Vm_Object_T *)(pValue->v).string);
    break;
  default:
    vm_syslogf(2,4,"Fatal error = %s\nFILE(%s)\nLINE(%d)\n","Bad type of value to destruct.\n",
               "/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x1bb);
    _vm_endProcess(0);
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  case '\x14':
    vm_freeLoopValue((Vm_Loop_Ctrl_T *)(pValue->v).string);
  }
  pValue->type = '\0';
  pValue->attrib = '\0';
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

<a id="vm-converttoptr"></a>

## vm_convertToPtr / procedure_pseudo_code

Evidence: `ev_a2b25c7c1116645bf8e17c95fb0760fd8a8046ce913401ae3ebe14c9f34c885c`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_convertToPtr(void)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  Vm_Value *pVVar3;
  int iVar4;
  Vm_Value *pVVar5;
  
  iVar2 = vm_sp;
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pNewValue@[???] */
  pVVar1 = vm_stack;
  switch(vm_stack[vm_sp].type) {
  case '\x04':
  case '\x05':
    vm_stack[vm_sp].attrib = vm_stack[vm_sp].attrib | 1;
    break;
  case '\x06':
  case '\a':
    break;
  case '\b':
    goto switchD_0808a246_caseD_8;
  default:
    vm_error("Bad type to index.\n");
  }
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pVVar3 = pdb_allocateTinyBlock(0x18);
  if (pVVar3 == (Vm_Value *)0x0) {
    vm_error("Not eough memory to create value.\n");
  }
  pVVar5 = pVVar3;
  for (iVar4 = 6; iVar4 != 0; iVar4 = iVar4 + -1) {
    pVVar5->ref = 0;
    pVVar5 = (Vm_Value *)&pVVar5->type;
  }
  pVVar3->type = '\0';
  pVVar3->attrib = '\0';
  pVVar3->ref = 1;
  vm_totalValues = vm_totalValues + 1;
  if (vm_allValues == (Vm_Value_T *)0x0) {
    vm_allValues = pVVar3;
    pVVar3->pPrev = (Vm_Value *)0x0;
    pVVar3->pNext = (Vm_Value *)0x0;
  }
  else {
    pVVar3->pPrev = (Vm_Value *)0x0;
    pVVar3->pNext = vm_allValues;
    vm_allValues->pPrev = pVVar3;
    vm_allValues = pVVar3;
  }
  pVVar3->type = pVVar1[iVar2].type;
  pVVar3->attrib = pVVar1[iVar2].attrib;
  pVVar3->v = pVVar1[iVar2].v;
  pVVar3->pThis = pVVar3;
  pVVar1[iVar2].type = '\b';
  pVVar1[iVar2].v.pPtrValue = pVVar3;
switchD_0808a246_caseD_8:
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

## vm_convertToValue / procedure_assembly

Evidence: `ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956`

```asm
0x808c028: PUSH EBP
0x808c029: MOV EBP, ESP
0x808c02b: PUSH EBX
0x808c02c: SUB ESP, 0xc
0x808c02f: MOV EAX, [0x086619cc]
0x808c034: MOV EDX, dword ptr [0x08661964]
0x808c03a: LEA EAX, [EAX + EAX*0x2]
0x808c03d: LEA EAX, [EDX + EAX*0x8]
0x808c040: MOV EBX, dword ptr [EAX + 0x8]
0x808c043: PUSH EBX
0x808c044: PUSH EAX
0x808c045: CALL 0x0808be24
0x808c04a: MOV EAX, dword ptr [EBX]
0x808c04c: DEC EAX
0x808c04d: ADD ESP, 0x10
0x808c050: TEST EAX, EAX
0x808c052: MOV dword ptr [EBX], EAX
0x808c054: JNZ 0x0808c09c
0x808c056: TEST EBX, EBX
0x808c058: JZ 0x0808c09c
0x808c05a: SUB ESP, 0xc
0x808c05d: PUSH EBX
0x808c05e: CALL 0x0808ad98
0x808c063: ADD ESP, 0x10
0x808c066: DEC dword ptr [0x0847fd88]
0x808c06c: CMP EBX, dword ptr [0x0847fd8c]
0x808c072: JZ 0x0808c0a1
0x808c074: MOV EAX, dword ptr [EBX + 0xc]
0x808c077: MOV EDX, dword ptr [EBX + 0x14]
0x808c07a: MOV dword ptr [EDX + 0xc], EAX
0x808c07d: MOV EAX, dword ptr [EBX + 0xc]
0x808c080: TEST EAX, EAX
0x808c082: JZ 0x0808c087
0x808c084: MOV dword ptr [EAX + 0x14], EDX
0x808c087: SUB ESP, 0xc
0x808c08a: MOV dword ptr [EBX + 0x10], 0x0
0x808c091: PUSH EBX
0x808c092: CALL 0x080cf214
0x808c097: ADD ESP, 0x10
0x808c09a: MOV ESI, ESI
0x808c09c: MOV EBX, dword ptr [EBP + -0x4]
0x808c09f: LEAVE
0x808c0a0: RET
0x808c0a1: MOV EAX, dword ptr [EBX + 0xc]
0x808c0a4: TEST EAX, EAX
0x808c0a6: MOV [0x0847fd8c], EAX
0x808c0ab: JZ 0x0808c087
0x808c0ad: MOV dword ptr [EAX + 0x14], 0x0
0x808c0b4: JMP 0x0808c087
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-converttovalue"></a>

## vm_convertToValue / procedure_pseudo_code

Evidence: `ev_8a351c14040cefb54204c1d1879ec273798f7579844be1503f87df0f5a5783dc`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_convertToValue(void)

{
  Vm_Value_T *pValue;
  Vm_Value *pVVar1;
  int iVar2;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg3(EBX)] */
  pValue = vm_stack[vm_sp].v.pPtrValue;
  vm_copyValue(vm_stack + vm_sp,pValue);
  iVar2 = pValue->ref + -1;
  pValue->ref = iVar2;
  if ((iVar2 == 0) && (pValue != (Vm_Value_T *)0x0)) {
    vm_cleanupValue(pValue);
    vm_totalValues = vm_totalValues + -1;
    if (pValue == vm_allValues) {
      vm_allValues = pValue->pNext;
      if (vm_allValues != (Vm_Value *)0x0) {
        vm_allValues->pPrev = (Vm_Value *)0x0;
      }
    }
    else {
      pVVar1 = pValue->pPrev;
      pVVar1->pNext = pValue->pNext;
      if (pValue->pNext != (Vm_Value *)0x0) {
        pValue->pNext->pPrev = pVVar1;
      }
    }
    pValue->pThis = (Vm_Value *)0x0;
    pdb_freeTinyBlock(pValue);
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

## vm_convertValueToConst / procedure_assembly

Evidence: `ev_27174938ea17a652d5f14712c667798de8416b58f3e9178d0f204341437021ec`

```asm
0x808cc18: PUSH EBP
0x808cc19: MOV EBP, ESP
0x808cc1b: MOV EDX, dword ptr [EBP + 0x8]
0x808cc1e: MOVZX EAX, byte ptr [EDX + 0x4]
0x808cc22: SUB EAX, 0x6
0x808cc25: CMP EAX, 0x1
0x808cc28: JA 0x0808cc2e
0x808cc2a: OR byte ptr [EDX + 0x5], 0x1
0x808cc2e: LEAVE
0x808cc2f: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-convertvaluetoconst"></a>

## vm_convertValueToConst / procedure_pseudo_code

Evidence: `ev_b6460e04232fdc6c08106ab85659c62fb0bfe8932b841606f522549d510bbd48`

```c

void vm_convertValueToConst(Vm_Value_T *pValue)

{
  if (pValue->type - 6 < 2) {
    pValue->attrib = pValue->attrib | 1;
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

<a id="vm-copyvalue"></a>

## vm_copyValue / procedure_pseudo_code

Evidence: `ev_dc24362b2fe9c37388965c9aa282e02e66f7bbe5bd7fe56dbc2c22df1fae8b33`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-destructobject"></a>

## vm_destructObject / procedure_pseudo_code

Evidence: `ev_196547cb37731a3632d63fbc4a62d83a57ac5f788fa57a3ed95657b8a8346c69`

```c

void vm_destructObject(Vm_Object_T *pObject)

{
  Vm_Program_T *pVVar1;
  Vm_Object_T *pVVar2;
  Vm_Value *pVVar3;
  Vm_Object **ppVVar4;
  int iVar5;
  int iVar6;
  Vm_Program_T *pProgram;
  
  if (pObject->execRef < 1) {
    vm_makeObjectAlone(pObject);
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
    if (pObject->pBindValue != (Vm_Value_T *)0x0) {
      vm_unrefValue(pObject->pBindValue);
      pObject->pBindValue = (Vm_Value *)0x0;
    }
    pVVar3 = pObject->pObjectValues;
    if (pVVar3 != (Vm_Value *)0x0) {
      pVVar1 = pObject->pProgram;
      iVar6 = 0;
      iVar5 = 0;
      if (0 < pVVar1->maxVariableNo) {
        while( true ) {
          vm_destructStaticValue((Vm_Value_T *)((int)&pVVar3->ref + iVar5));
          iVar6 = iVar6 + 1;
          iVar5 = iVar5 + 0x18;
          if (pVVar1->maxVariableNo <= iVar6) break;
          pVVar3 = pObject->pObjectValues;
        }
      }
      pdb_freeMemory(pObject->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1ce)
      ;
      pObject->pObjectValues = (Vm_Value *)0x0;
    }
                    /* Unresolved local var: Vm_Object_T * * ppHash@[DW_OP_reg2(EDX)]
                       Unresolved local var: int idx@[???] */
    pVVar1 = pObject->pProgram;
    ppVVar4 = vm_objectHashTable + ((uint)pVVar1->strName->hash + pObject->oid & 0x3fff);
    for (pVVar2 = *ppVVar4; pVVar2 != (Vm_Object_T *)0x0; pVVar2 = pVVar2->pNextHashObject) {
      if (pVVar2 == pObject) {
        *ppVVar4 = pObject->pNextHashObject;
        break;
      }
      ppVVar4 = &pVVar2->pNextHashObject;
    }
    if (pObject->ref == 0) {
      iVar5 = pVVar1->ref + -1;
      pVVar1->ref = iVar5;
      if (iVar5 < 1) {
        vm_freeCodeSegment(pVVar1,&pVVar1->initCodeSegment);
        if (vm_gatheringCallStackInfo != 0) {
          vm_enterPPWholeCriticalSection();
        }
        vm_destructProgramHead(pVVar1);
        if (vm_gatheringCallStackInfo != 0) {
          vm_leavePPWholeCriticalSection();
        }
      }
      pdb_freeMemory(pObject,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1f5);
    }
    else {
      pObject->flags = pObject->flags | 0x8000;
    }
  }
  else {
    pObject->flags = pObject->flags | 0x4000;
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

## vm_destructObjectValues / procedure_assembly

Evidence: `ev_d50f92fab09f479b0532c35c9c57bb161f0484bdebcf5d1545ecb10b7ace15be`

```asm
0x80b11a8: PUSH EBP
0x80b11a9: MOV EBP, ESP
0x80b11ab: PUSH EDI
0x80b11ac: PUSH ESI
0x80b11ad: PUSH EBX
0x80b11ae: SUB ESP, 0xc
0x80b11b1: MOV EDI, dword ptr [EBP + 0x8]
0x80b11b4: MOV EAX, dword ptr [EDI + 0x1c]
0x80b11b7: TEST EAX, EAX
0x80b11b9: JNZ 0x080b1240
0x80b11bf: MOV EAX, dword ptr [EDI + 0x24]
0x80b11c2: TEST EAX, EAX
0x80b11c4: JZ 0x080b11f8
0x80b11c6: MOV EDX, dword ptr [EDI + 0x20]
0x80b11c9: XOR ESI, ESI
0x80b11cb: XOR EBX, EBX
0x80b11cd: CMP word ptr [EDX + 0xe], 0x0
0x80b11d2: MOV dword ptr [EBP + -0x10], EDX
0x80b11d5: JG 0x080b1203
0x80b11d7: PUSH 0x1ce
0x80b11dc: PUSH 0x82b0e28
0x80b11e1: PUSH 0x82a376b
0x80b11e6: PUSH dword ptr [EDI + 0x24]
0x80b11e9: CALL 0x080ceb94
0x80b11ee: MOV dword ptr [EDI + 0x24], 0x0
0x80b11f5: LEA ESI, [ESI]
0x80b11f8: LEA ESP, [EBP + -0xc]
0x80b11fb: POP EBX
0x80b11fc: POP ESI
0x80b11fd: POP EDI
0x80b11fe: LEAVE
0x80b11ff: RET
0x80b1200: MOV EAX, dword ptr [EDI + 0x24]
0x80b1203: ADD EAX, EBX
0x80b1205: SUB ESP, 0xc
0x80b1208: PUSH EAX
0x80b1209: CALL 0x0808c0b8
0x80b120e: MOV EDX, dword ptr [EBP + -0x10]
0x80b1211: INC ESI
0x80b1212: MOVSX EAX, word ptr [EDX + 0xe]
0x80b1216: ADD ESP, 0x10
0x80b1219: ADD EBX, 0x18
0x80b121c: CMP EAX, ESI
0x80b121e: JG 0x080b1200
0x80b1220: PUSH 0x1ce
0x80b1225: PUSH 0x82b0e28
0x80b122a: PUSH 0x82a376b
0x80b122f: PUSH dword ptr [EDI + 0x24]
0x80b1232: CALL 0x080ceb94
0x80b1237: MOV dword ptr [EDI + 0x24], 0x0
0x80b123e: JMP 0x080b11f8
0x80b1240: SUB ESP, 0xc
0x80b1243: PUSH EAX
0x80b1244: CALL 0x0808bfb4
0x80b1249: ADD ESP, 0x10
0x80b124c: MOV dword ptr [EDI + 0x1c], 0x0
0x80b1253: JMP 0x080b11bf
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-destructobjectvalues"></a>

## vm_destructObjectValues / procedure_pseudo_code

Evidence: `ev_d8173d61d8d105723534cddea8b9f99fadfed21f978546de6160508e896cd5ec`

```c

void vm_destructObjectValues(Vm_Object_T *pObject)

{
  Vm_Program_T *pVVar1;
  Vm_Value *pVVar2;
  int iVar3;
  int iVar4;
  Vm_Program_T *pProgram;
  
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  if (pObject->pBindValue != (Vm_Value_T *)0x0) {
    vm_unrefValue(pObject->pBindValue);
    pObject->pBindValue = (Vm_Value *)0x0;
  }
  pVVar2 = pObject->pObjectValues;
  if (pVVar2 != (Vm_Value *)0x0) {
    pVVar1 = pObject->pProgram;
    iVar4 = 0;
    iVar3 = 0;
    if (pVVar1->maxVariableNo < 1) {
      pdb_freeMemory(pObject->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1ce)
      ;
      pObject->pObjectValues = (Vm_Value *)0x0;
    }
    else {
      while( true ) {
        vm_destructStaticValue((Vm_Value_T *)((int)&pVVar2->ref + iVar3));
        iVar4 = iVar4 + 1;
        iVar3 = iVar3 + 0x18;
        if (pVVar1->maxVariableNo <= iVar4) break;
        pVVar2 = pObject->pObjectValues;
      }
      pdb_freeMemory(pObject->pObjectValues,"VM","/home/cc/xxxx/gameshell/src/vm/vm_object.c",0x1ce)
      ;
      pObject->pObjectValues = (Vm_Value *)0x0;
    }
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

## vm_destructObjectValues / procedure_references

Evidence: `ev_01a65a72f2a182a9d1901125baffd24e720b82c4f3a1c8ea9d871aed1fe24189`

```json
{
  "direction": "outgoing",
  "procedure": {
    "address": "0x80b11a8",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": false,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x80b1257",
          "start": "0x80b11a8"
        }
      ],
      "span_bytes": 176,
      "total_bytes": 176
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_destructObjectValues"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x80b11b1",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "stack:0x4",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b11b9",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b1240",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b11c4",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b11f8",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x80b11d2",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "stack:0xffffffffffffffec",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b11d5",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b1203",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x80b11dc",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x82b0e28",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x80b11e1",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x82a376b",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x80b11e9",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80ceb94",
      "target_procedure": {
        "address": "0x80ceb94",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80cebea",
              "start": "0x80ceb94"
            },
            {
              "end": "0x80cedfe",
              "start": "0x80cebec"
            }
          ],
          "span_bytes": 619,
          "total_bytes": 618
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "pdb_freeMemory"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x80b11f8",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "stack:0xfffffffffffffff0",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x80b1209",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x808c0b8",
      "target_procedure": {
        "address": "0x808c0b8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c0f9",
              "start": "0x808c0b8"
            },
            {
              "end": "0x808c110",
              "start": "0x808c0fc"
            }
          ],
          "span_bytes": 89,
          "total_bytes": 87
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructStaticValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x80b120e",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "stack:0xffffffffffffffec",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b121e",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b1200",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x80b1225",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x82b0e28",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x80b122a",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x82a376b",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x80b1232",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80ceb94",
      "target_procedure": {
        "address": "0x80ceb94",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80cebea",
              "start": "0x80ceb94"
            },
            {
              "end": "0x80cedfe",
              "start": "0x80cebec"
            }
          ],
          "span_bytes": 619,
          "total_bytes": 618
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "pdb_freeMemory"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b123e",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b11f8",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
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
      },
      "source_address": "0x80b1244",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x808bfb4",
      "target_procedure": {
        "address": "0x808bfb4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c005",
              "start": "0x808bfb4"
            },
            {
              "end": "0x808c026",
              "start": "0x808c008"
            }
          ],
          "span_bytes": 115,
          "total_bytes": 113
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_unrefValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b1253",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b11bf",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
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

## vm_destructObjectValues / procedure_references

Evidence: `ev_1643799a95f82ea4e81fe0c49b2639866e471a6ab8a47eb18dad59d89ef07dd1`

```json
{
  "direction": "incoming",
  "procedure": {
    "address": "0x80b11a8",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": false,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x80b1257",
          "start": "0x80b11a8"
        }
      ],
      "span_bytes": 176,
      "total_bytes": 176
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_destructObjectValues"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b11b9",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b1240",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b11c4",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b11f8",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b11d5",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b1203",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b121e",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b1200",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b123e",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b11f8",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80b1253",
      "source_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      },
      "target_address": "0x80b11bf",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": ".debug_frame:0x10dfc",
      "source_procedure": null,
      "target_address": "0x80b11a8",
      "target_procedure": {
        "address": "0x80b11a8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80b1257",
              "start": "0x80b11a8"
            }
          ],
          "span_bytes": 176,
          "total_bytes": 176
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructObjectValues"
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

<a id="vm-destructstaticvalue"></a>

## vm_destructStaticValue / procedure_pseudo_code

Evidence: `ev_b7275ea0d205e76bd66e12eb46ee8132bb75cd0db3113213fdd60970242bb1c2`

```c

void vm_destructStaticValue(Vm_Value_T *pValue)

{
  Vm_Value *pVVar1;
  
  pValue->ref = pValue->ref + -1;
  vm_cleanupValue(pValue);
  vm_totalValues = vm_totalValues + -1;
  if (pValue == vm_allValues) {
    vm_allValues = pValue->pNext;
    if (vm_allValues != (Vm_Value_T *)0x0) {
      vm_allValues->pPrev = (Vm_Value *)0x0;
    }
  }
  else {
    pVVar1 = pValue->pPrev;
    pVVar1->pNext = pValue->pNext;
    if (pValue->pNext != (Vm_Value *)0x0) {
      pValue->pNext->pPrev = pVVar1;
    }
  }
  pValue->pThis = (Vm_Value *)0x0;
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

<a id="vm-f_garbage_collect"></a>

## vm_f_garbage_collect / procedure_pseudo_code

Evidence: `ev_8fecedeb1f644246e085db4dbc5c875cfb14fd6368045b63417b2183eab987ab`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_garbage_collect(void)

{
  ushort uVar1;
  Vm_Value *pVVar2;
  char *pcVar3;
  int num;
  Vm_Value_T *pVVar4;
  Vm_Value_T *pVVar5;
  Vm_Value_T value;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int n@[DW_OP_reg6(ESI)] */
  if (0 < vm_currentContext->argNum) {
    vm_error("Too many arguments(%d) to %s.\n",vm_currentContext->argNum,"garbage_values");
  }
  vm_refreshAllValuesActive();
  num = 0;
  if (vm_allValues != (Vm_Value_T *)0x0) {
    while (pVVar5 = vm_allValues, vm_allValues != (Vm_Value_T *)0x0) {
      uVar1 = vm_allValues->active;
      pVVar2 = vm_allValues;
      while (uVar1 != 0) {
        pVVar2 = pVVar2->pNext;
        if (pVVar2 == (Vm_Value *)0x0) goto LAB_080f96d4;
        uVar1 = pVVar2->active;
      }
      if (pVVar2 == (Vm_Value_T *)0x0) break;
      pVVar2->active = 1;
      pVVar4 = pVVar2;
      pcVar3 = vm_valueType((uint)pVVar2->type);
      vm_stdprintf("Drop value: (%s) at %p\n",pcVar3,pVVar4,pVVar5);
      value.type = pVVar2->type;
      value.attrib = pVVar2->attrib;
      value.v = pVVar2->v;
      pVVar2->type = '\0';
      pVVar2->attrib = '\0';
      num = num + 1;
      vm_cleanupValue(&value);
      if (vm_allValues == (Vm_Value_T *)0x0) break;
    }
  }
LAB_080f96d4:
  vm_pushNumber(num);
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

## vm_freeNode / procedure_assembly

Evidence: `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae`

```asm
0x809c33c: PUSH EBP
0x809c33d: MOV EBP, ESP
0x809c33f: PUSH EBX
0x809c340: PUSH EDX
0x809c341: CMP dword ptr [EAX + 0x4], 0x1
0x809c345: MOV EBX, EAX
0x809c347: JLE 0x0809c374
0x809c349: SUB ESP, 0xc
0x809c34c: LEA EAX, [EBX + 0x4]
0x809c34f: PUSH EAX
0x809c350: CALL 0x0808ad98
0x809c355: LEA EAX, [EBX + 0x1c]
0x809c358: MOV dword ptr [ESP], EAX
0x809c35b: CALL 0x0808ad98
0x809c360: MOV EAX, [0x08604220]
0x809c365: MOV dword ptr [EBX], EAX
0x809c367: MOV dword ptr [0x08604220], EBX
0x809c36d: MOV EBX, dword ptr [EBP + -0x4]
0x809c370: LEAVE
0x809c371: RET
0x809c374: CMP dword ptr [EAX + 0x1c], 0x1
0x809c378: JG 0x0809c349
0x809c37a: SUB ESP, 0xc
0x809c37d: LEA EAX, [EAX + 0x4]
0x809c380: PUSH EAX
0x809c381: CALL 0x0808c0b8
0x809c386: LEA EAX, [EBX + 0x1c]
0x809c389: MOV dword ptr [ESP], EAX
0x809c38c: CALL 0x0808c0b8
0x809c391: MOV dword ptr [ESP], EBX
0x809c394: CALL 0x080cf214
0x809c399: MOV EBX, dword ptr [EBP + -0x4]
0x809c39c: LEAVE
0x809c39d: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-freenode"></a>

## vm_freeNode / procedure_pseudo_code

Evidence: `ev_921c6ee51c237f77c7b33c2b6e0fc80b3f2e7370fc5984d34f346c4c71cb18f3`

```c

void __regparm3 vm_freeNode(Vm_Mapping_Node_T *pElt)

{
  if ((pElt->values[0].ref < 2) && (pElt->values[1].ref < 2)) {
    vm_destructStaticValue(pElt->values);
    vm_destructStaticValue(pElt->values + 1);
    pdb_freeTinyBlock(pElt);
    return;
  }
  vm_cleanupValue(pElt->values);
  vm_cleanupValue(pElt->values + 1);
  pElt->pNext = _vm_mapNodeToFree;
  _vm_mapNodeToFree = pElt;
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

## vm_freeNode / procedure_references

Evidence: `ev_a861b015bf769a528ec418daa55c88e54d8db8d6273d9deddb9cf474bd3719e0`

```json
{
  "direction": "outgoing",
  "procedure": {
    "address": "0x809c33c",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": true,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x809c371",
          "start": "0x809c33c"
        },
        {
          "end": "0x809c39d",
          "start": "0x809c374"
        }
      ],
      "span_bytes": 98,
      "total_bytes": 96
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_freeNode"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809c347",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x809c374",
      "target_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      }
    },
    {
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
      },
      "source_address": "0x809c350",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x808ad98",
      "target_procedure": {
        "address": "0x808ad98",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808afc4",
              "start": "0x808ad98"
            }
          ],
          "span_bytes": 557,
          "total_bytes": 557
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_cleanupValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x809c358",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "stack:0xffffffffffffffe4",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x809c35b",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x808ad98",
      "target_procedure": {
        "address": "0x808ad98",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808afc4",
              "start": "0x808ad98"
            }
          ],
          "span_bytes": 557,
          "total_bytes": 557
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_cleanupValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x809c360",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x8604220",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x809c367",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x8604220",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x809c36d",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "stack:0xfffffffffffffff8",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809c378",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x809c349",
      "target_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      }
    },
    {
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
      },
      "source_address": "0x809c381",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x808c0b8",
      "target_procedure": {
        "address": "0x808c0b8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c0f9",
              "start": "0x808c0b8"
            },
            {
              "end": "0x808c110",
              "start": "0x808c0fc"
            }
          ],
          "span_bytes": 89,
          "total_bytes": 87
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructStaticValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x809c389",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "stack:0xffffffffffffffe4",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x809c38c",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x808c0b8",
      "target_procedure": {
        "address": "0x808c0b8",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c0f9",
              "start": "0x808c0b8"
            },
            {
              "end": "0x808c110",
              "start": "0x808c0fc"
            }
          ],
          "span_bytes": 89,
          "total_bytes": 87
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_destructStaticValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x809c391",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "stack:0xffffffffffffffe4",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x809c394",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "0x80cf214",
      "target_procedure": {
        "address": "0x80cf214",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80cf280",
              "start": "0x80cf214"
            },
            {
              "end": "0x80cf3a6",
              "start": "0x80cf284"
            }
          ],
          "span_bytes": 403,
          "total_bytes": 400
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "pdb_freeTinyBlock"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x809c399",
      "source_procedure": {
        "address": "0x809c33c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809c371",
              "start": "0x809c33c"
            },
            {
              "end": "0x809c39d",
              "start": "0x809c374"
            }
          ],
          "span_bytes": 98,
          "total_bytes": 96
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_freeNode"
      },
      "target_address": "stack:0xfffffffffffffff8",
      "target_procedure": null
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

<a id="vm-freeobsoletedmapnodes"></a>

## vm_freeObsoletedMapNodes / procedure_pseudo_code

Evidence: `ev_aede162e24387c626fea56b29104aeb7418eda0d05ab642ae9369e131d7bd0eb`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

int vm_freeObsoletedMapNodes(void)

{
  Vm_Mapping_Node_T *pVVar1;
  Vm_Mapping_Node_T *pElt;
  
  pVVar1 = (Vm_Mapping_Node_T *)&_vm_mapNodeToFree;
  for (pElt = _vm_mapNodeToFree; pElt != (Vm_Mapping_Node_T *)0x0; pElt = pElt->pNext) {
    if ((pElt->values[0].ref < 2) && (pElt->values[1].ref < 2)) {
      pVVar1->pNext = pElt->pNext;
      vm_freeNode(pElt);
      pElt = pVVar1;
    }
    pVVar1 = pElt;
  }
  return (uint)(_vm_mapNodeToFree == (Vm_Mapping_Node_T *)0x0);
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

<a id="vm-freestackvalue"></a>

## vm_freeStackValue / procedure_pseudo_code

Evidence: `ev_31db47dcac982a1fc4e14f6cc027adc6bc4af49e1cf51b0981c819b2291a5d29`

```c

void vm_freeStackValue(int n)

{
  vm_cleanupValue(vm_stack + n);
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
- Ghidra Listing reports a terminal call at 0x8092002 to 0x808ad98 vm_cleanupValue (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-freevalue"></a>

## vm_freeValue / procedure_pseudo_code

Evidence: `ev_d4d67468e567147e05b1c04a33a2c8c38a4c356bb628f8e8ef800bc71d69e04c`

```c

void vm_freeValue(Vm_Value_T *pValue)

{
  Vm_Value *pVVar1;
  
  if ((pValue != (Vm_Value_T *)0x0) && (pValue->ref == 0)) {
    vm_cleanupValue(pValue);
    vm_totalValues = vm_totalValues + -1;
    if (pValue == vm_allValues) {
      vm_allValues = pValue->pNext;
      if (vm_allValues != (Vm_Value_T *)0x0) {
        vm_allValues->pPrev = (Vm_Value *)0x0;
      }
    }
    else {
      pVVar1 = pValue->pPrev;
      pVVar1->pNext = pValue->pNext;
      if (pValue->pNext != (Vm_Value *)0x0) {
        pValue->pNext->pPrev = pVVar1;
      }
    }
    pValue->pThis = (Vm_Value *)0x0;
    pdb_freeTinyBlock(pValue);
    return;
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
- Ghidra Listing reports a terminal call at 0x808bf94 to 0x80cf214 pdb_freeTinyBlock (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-getvaluefromstack"></a>

## vm_getValueFromStack / procedure_pseudo_code

Evidence: `ev_18ddcc6b30e7bf221782adbd2639a11d8f2748da1e50834dabfb381fe4a0951e`

```c

Vm_Value_T * vm_getValueFromStack(int n,int type)

{
  int iVar1;
  Vm_Value_T *pVVar2;
  
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg1(ECX)] */
  iVar1 = n + vm_sp;
  if ((vm_sp <= iVar1) && (iVar1 < vm_stackSize)) {
    if ((type == 0xff) || (pVVar2 = (Vm_Value_T *)0x0, (uint)vm_stack[iVar1].type == type)) {
      pVVar2 = vm_stack + iVar1;
    }
    return pVVar2;
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

<a id="vm-getvaluetype"></a>

## vm_getValueType / procedure_pseudo_code

Evidence: `ev_4b11f8b8a188ce3208935d909fe583bb7f81ba977c29dbbede57d5c0dbe4861c`

```c

int vm_getValueType(Vm_Value_T *pValue)

{
  Vm_Value *pVVar1;
  byte bVar2;
  
  bVar2 = pValue->type;
  if (bVar2 == 8) {
    pValue = (pValue->v).pPtrValue;
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg0(EAX)] */
    bVar2 = pValue->type;
    if (bVar2 != 9) goto LAB_0808cba4;
LAB_0808cbc4:
    pVVar1 = ((pValue->v).pPtrValue)->pNext;
    if (pVVar1 == (Vm_Value *)0x0) goto LAB_0808cba9;
  }
  else {
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg0(EAX)] */
    if (bVar2 == 9) goto LAB_0808cbc4;
LAB_0808cba4:
    if (bVar2 != 10) goto LAB_0808cba9;
    pVVar1 = (Vm_Value *)(pValue->v).string;
  }
  if (((uint)pVVar1->pNext & 0x8000) != 0) {
    vm_cleanupValue(pValue);
    return (uint)pValue->type;
  }
LAB_0808cba9:
  return (uint)bVar2;
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

<a id="vm-initvalue"></a>

## vm_initValue / procedure_pseudo_code

Evidence: `ev_6921332b80a7a8467156e8214b2cf335f37f0c97324f7e7737d1d6c56b44b54d`

```c

void vm_initValue(Vm_Value_T *pValue)

{
  int iVar1;
  Vm_Value_T *pVVar2;
  
  pVVar2 = pValue;
  for (iVar1 = 6; iVar1 != 0; iVar1 = iVar1 + -1) {
    pVVar2->ref = 0;
    pVVar2 = (Vm_Value_T *)&pVVar2->type;
  }
  pValue->type = '\0';
  pValue->attrib = '\0';
  pValue->ref = 1;
  vm_totalValues = vm_totalValues + 1;
  if (vm_allValues == (Vm_Value_T *)0x0) {
    vm_allValues = pValue;
    pValue->pPrev = (Vm_Value *)0x0;
    pValue->pNext = (Vm_Value *)0x0;
    pValue->pThis = pValue;
    return;
  }
  pValue->pPrev = (Vm_Value *)0x0;
  pValue->pNext = vm_allValues;
  vm_allValues->pPrev = pValue;
  vm_allValues = pValue;
  pValue->pThis = pValue;
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

## vm_markActiveFromValue / procedure_assembly

Evidence: `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471`

```asm
0x808d994: PUSH EBP
0x808d995: MOV EBP, ESP
0x808d997: PUSH EDI
0x808d998: PUSH ESI
0x808d999: PUSH EBX
0x808d99a: MOV EDX, dword ptr [EBP + 0x8]
0x808d99d: CMP word ptr [EDX + 0x6], 0x0
0x808d9a2: JNZ 0x0808d9e1
0x808d9a4: MOVZX EAX, byte ptr [EDX + 0x4]
0x808d9a8: CMP EAX, 0x7
0x808d9ab: MOV word ptr [EDX + 0x6], 0x1
0x808d9b1: JZ 0x0808da48
0x808d9b7: JG 0x0808d9e9
0x808d9b9: CMP EAX, 0x6
0x808d9bc: JNZ 0x0808d9e1
0x808d9be: MOV EBX, dword ptr [EDX + 0x8]
0x808d9c1: MOV EDX, dword ptr [EBX + 0x4]
0x808d9c4: XOR ESI, ESI
0x808d9c6: TEST EDX, EDX
0x808d9c8: JLE 0x0808d9e1
0x808d9ca: SUB ESP, 0xc
0x808d9cd: MOV EAX, dword ptr [EBX + 0xc]
0x808d9d0: PUSH dword ptr [EAX + ESI*0x4]
0x808d9d3: INC ESI
0x808d9d4: CALL 0x0808d994
0x808d9d9: ADD ESP, 0x10
0x808d9dc: CMP dword ptr [EBX + 0x4], ESI
0x808d9df: JG 0x0808d9ca
0x808d9e1: LEA ESP, [EBP + -0xc]
0x808d9e4: POP EBX
0x808d9e5: POP ESI
0x808d9e6: POP EDI
0x808d9e7: LEAVE
0x808d9e8: RET
0x808d9e9: CMP EAX, 0x9
0x808d9ec: JZ 0x0808da8a
0x808d9f2: CMP EAX, 0x14
0x808d9f5: JNZ 0x0808d9e1
0x808d9f7: MOV EAX, dword ptr [EDX + 0x8]
0x808d9fa: MOV EDX, dword ptr [EAX + 0x8]
0x808d9fd: CMP EDX, 0x385
0x808da03: JZ 0x0808dabe
0x808da09: CMP EDX, 0x386
0x808da0f: JNZ 0x0808d9e1
0x808da11: MOV EAX, dword ptr [EAX + 0x14]
0x808da14: MOV EDI, dword ptr [EAX + 0x14]
0x808da17: MOV ESI, dword ptr [EAX + 0x8]
0x808da1a: MOV EBX, dword ptr [EDI + ESI*0x4]
0x808da1d: JMP 0x0808da3b
0x808da1f: SUB ESP, 0xc
0x808da22: LEA EAX, [EBX + 0x4]
0x808da25: PUSH EAX
0x808da26: CALL 0x0808d994
0x808da2b: LEA EAX, [EBX + 0x1c]
0x808da2e: MOV dword ptr [ESP], EAX
0x808da31: CALL 0x0808d994
0x808da36: ADD ESP, 0x10
0x808da39: MOV EBX, dword ptr [EBX]
0x808da3b: TEST EBX, EBX
0x808da3d: JNZ 0x0808da1f
0x808da3f: DEC ESI
0x808da40: CMP ESI, -0x1
0x808da43: JNZ 0x0808da1a
0x808da45: JMP 0x0808d9e1
0x808da48: MOV EAX, dword ptr [EDX + 0x8]
0x808da4b: MOV EDI, dword ptr [EAX + 0x14]
0x808da4e: MOV ESI, dword ptr [EAX + 0x8]
0x808da51: LEA ESI, [ESI]
0x808da54: MOV EBX, dword ptr [EDI + ESI*0x4]
0x808da57: JMP 0x0808da78
0x808da5c: SUB ESP, 0xc
0x808da5f: LEA EAX, [EBX + 0x4]
0x808da62: PUSH EAX
0x808da63: CALL 0x0808d994
0x808da68: LEA EAX, [EBX + 0x1c]
0x808da6b: MOV dword ptr [ESP], EAX
0x808da6e: CALL 0x0808d994
0x808da73: ADD ESP, 0x10
0x808da76: MOV EBX, dword ptr [EBX]
0x808da78: TEST EBX, EBX
0x808da7a: JNZ 0x0808da5c
0x808da7c: DEC ESI
0x808da7d: CMP ESI, -0x1
0x808da80: JNZ 0x0808da54
0x808da82: LEA ESP, [EBP + -0xc]
0x808da85: POP EBX
0x808da86: POP ESI
0x808da87: POP EDI
0x808da88: LEAVE
0x808da89: RET
0x808da8a: MOV EBX, dword ptr [EDX + 0x8]
0x808da8d: XOR EDI, EDI
0x808da8f: CMP word ptr [EBX + 0x6], 0x0
0x808da94: JLE 0x0808d9e1
0x808da9a: XOR ESI, ESI
0x808da9c: MOV EAX, dword ptr [EBX + 0x10]
0x808da9f: ADD EAX, ESI
0x808daa1: SUB ESP, 0xc
0x808daa4: PUSH EAX
0x808daa5: CALL 0x0808d994
0x808daaa: INC EDI
0x808daab: MOVSX EAX, word ptr [EBX + 0x6]
0x808daaf: ADD ESP, 0x10
0x808dab2: ADD ESI, 0x18
0x808dab5: CMP EAX, EDI
0x808dab7: JG 0x0808da9c
0x808dab9: JMP 0x0808d9e1
0x808dabe: MOV EBX, dword ptr [EAX + 0x14]
0x808dac1: MOV EAX, dword ptr [EBX + 0x4]
0x808dac4: XOR ESI, ESI
0x808dac6: TEST EAX, EAX
0x808dac8: JLE 0x0808d9e1
0x808dace: SUB ESP, 0xc
0x808dad1: MOV EAX, dword ptr [EBX + 0xc]
0x808dad4: PUSH dword ptr [EAX + ESI*0x4]
0x808dad7: INC ESI
0x808dad8: CALL 0x0808d994
0x808dadd: ADD ESP, 0x10
0x808dae0: CMP dword ptr [EBX + 0x4], ESI
0x808dae3: JG 0x0808dace
0x808dae5: JMP 0x0808d9e1
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-markactivefromvalue"></a>

## vm_markActiveFromValue / procedure_pseudo_code

Evidence: `ev_dc9d455696d05ef4b444be9dc273ee408ac79c93d4c11138399d240e3b7a82a5`

```c

void vm_markActiveFromValue(Vm_Value_T *pValue)

{
  int *piVar1;
  byte bVar2;
  Vm_Object *pVVar3;
  Vm_Value *pVVar4;
  Vm_Value *pVVar5;
  Vm_Mapping_Node_T **ppVVar6;
  Vm_Fun_Ptr *pVVar7;
  int iVar8;
  undefined4 *puVar9;
  Vm_Mapping_Node_T *pVVar10;
  int iVar11;
  uint uVar12;
  int iVar13;
  
  if (pValue->active == 0) {
    bVar2 = pValue->type;
    pValue->active = 1;
    if (bVar2 == 7) {
                    /* Unresolved local var: Vm_Mapping_Node_T * pElt@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mapping_Node_T * * ppa@[DW_OP_reg7(EDI)]
                       Unresolved local var: int j@[DW_OP_reg6(ESI)] */
      ppVVar6 = ((pValue->v).mapping)->table;
      uVar12 = ((pValue->v).mapping)->tableSize;
      do {
        for (pVVar10 = ppVVar6[uVar12]; pVVar10 != (Vm_Mapping_Node_T *)0x0;
            pVVar10 = pVVar10->pNext) {
          vm_markActiveFromValue(pVVar10->values);
          vm_markActiveFromValue(pVVar10->values + 1);
        }
        uVar12 = uVar12 - 1;
      } while (uVar12 != 0xffffffff);
      return;
    }
    if (bVar2 < 8) {
      if (bVar2 == 6) {
        pVVar3 = (pValue->v).object;
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
        iVar11 = 0;
        if (0 < pVVar3->execRef) {
          do {
            iVar13 = iVar11 * 4;
            iVar11 = iVar11 + 1;
            vm_markActiveFromValue(*(Vm_Value_T **)(pVVar3->flags + iVar13));
          } while (iVar11 < pVVar3->execRef);
        }
      }
    }
    else if (bVar2 == 9) {
      pVVar7 = (pValue->v).function;
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
      iVar11 = 0;
      if (0 < pVVar7->argn) {
        iVar13 = 0;
        do {
          vm_markActiveFromValue((Vm_Value_T *)((int)&pVVar7->pArgs->ref + iVar13));
          iVar11 = iVar11 + 1;
          iVar13 = iVar13 + 0x18;
        } while (iVar11 < pVVar7->argn);
      }
    }
    else if (bVar2 == 0x14) {
                    /* Unresolved local var: Vm_Loop_Ctrl_T * pLoopCtrl@[DW_OP_reg0(EAX)] */
      pVVar4 = (pValue->v).pPtrValue;
      if ((pVVar4->v).info == (void *)0x385) {
        pVVar4 = pVVar4->pPrev;
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
        iVar11._0_1_ = pVVar4->type;
        iVar11._1_1_ = pVVar4->attrib;
        iVar11._2_2_ = pVVar4->active;
        iVar13 = 0;
        if (0 < iVar11) {
          do {
            piVar1 = &pVVar4->pNext->ref + iVar13;
            iVar13 = iVar13 + 1;
            vm_markActiveFromValue((Vm_Value_T *)*piVar1);
            iVar8._0_1_ = pVVar4->type;
            iVar8._1_1_ = pVVar4->attrib;
            iVar8._2_2_ = pVVar4->active;
          } while (iVar13 < iVar8);
        }
      }
      else if ((pVVar4->v).info == (void *)0x386) {
                    /* Unresolved local var: Vm_Mapping_Node_T * pElt@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mapping_Node_T * * ppa@[DW_OP_reg7(EDI)]
                       Unresolved local var: int j@[DW_OP_reg6(ESI)] */
        pVVar5 = pVVar4->pPrev->pPrev;
        iVar11 = (pVVar4->pPrev->v).number;
        do {
          for (puVar9 = (undefined4 *)(&pVVar5->ref)[iVar11]; puVar9 != (undefined4 *)0x0;
              puVar9 = (undefined4 *)*puVar9) {
            vm_markActiveFromValue((Vm_Value_T *)(puVar9 + 1));
            vm_markActiveFromValue((Vm_Value_T *)(puVar9 + 7));
          }
          iVar11 = iVar11 + -1;
        } while (iVar11 != -1);
      }
    }
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

<a id="vm-pushglobalvptr"></a>

## vm_pushGlobalVptr / procedure_pseudo_code

Evidence: `ev_e0b26424bac92dc860aadf077630177dfff6e85d228ddea8d8a8e7a77fb8b7ee`

```c

void vm_pushGlobalVptr(int valueNo)

{
  uchar uVar1;
  Vm_Value_T *pVVar2;
  Vm_Value_T *pVVar3;
  int iVar4;
  
  pVVar2 = vm_pGlobalVar[valueNo].pValue;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\b';
  iVar4 = vm_sp;
  pVVar3 = vm_stack;
  uVar1 = pVVar2->attrib;
  pVVar2->ref = pVVar2->ref + 1;
  pVVar3[iVar4].v.pPtrValue = pVVar2;
  pVVar3[iVar4].attrib = uVar1;
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

<a id="vm-pushlocalvar"></a>

## vm_pushLocalVar / procedure_pseudo_code

Evidence: `ev_9693ba97f8eddd0327c10164c589acb0a343f4181335c1476bc606e322e04ee5`

```c

void vm_pushLocalVar(int no)

{
  int iVar1;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg0(EAX)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (((vm_funCp == 0) || (vm_funContext[vm_funCp].argNum <= no)) ||
     (iVar1 = no + vm_funContext[vm_funCp].bp, iVar1 < vm_sp)) {
    if (vm_sp == 0) {
      vm_error("Stack overflow when calculating.\n");
    }
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\0';
    vm_stack[vm_sp].v.info = (void *)0x0;
  }
  else {
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\0';
    vm_copyValue(vm_stack + vm_sp,vm_stack + iVar1);
    if (vm_stack[vm_sp].type == '\b') {
      vm_convertToValue();
      return;
    }
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
- Ghidra Listing reports a terminal call at 0x809159d to 0x808c028 vm_convertToValue (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-pushlocalvari"></a>

## vm_pushLocalVarI / procedure_pseudo_code

Evidence: `ev_c18e607e151b8a0fd808d2d0882f548466d6d416b260f8f1deb40ac30a20e833`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_pushLocalVarI(void)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  
                    /* Unresolved local var: int no@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_getValueFromStack(0,0xff);
  if (pVVar1->type == '\x03') {
    iVar2 = _vm_ftol((pVVar1->v).real);
    (pVVar1->v).number = iVar2;
    pVVar1->type = '\x02';
  }
  else if (pVVar1->type != '\x02') {
    vm_error("Bad index for argument.\n");
  }
  iVar2 = (pVVar1->v).number;
  vm_cleanupValue(vm_stack + vm_sp);
  vm_sp = vm_sp + 1;
  vm_pushLocalVar(iVar2 + -1);
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

## vm_pushLocalVptr / procedure_assembly

Evidence: `ev_9b4e2d24822946ff7ae5449f7d30b3e3b2cc1134ef9c530294dd9a840f45feed`

```asm
0x809134c: PUSH EBP
0x809134d: MOV EBP, ESP
0x809134f: PUSH ESI
0x8091350: PUSH EBX
0x8091351: MOV EAX, [0x086619cc]
0x8091356: TEST EAX, EAX
0x8091358: MOV EBX, dword ptr [EBP + 0x8]
0x809135b: JZ 0x080914a4
0x8091361: MOV EAX, [0x086619c0]
0x8091366: TEST EAX, EAX
0x8091368: JZ 0x08091488
0x809136e: LEA EDX, [EAX + EAX*0x4]
0x8091371: MOV EAX, [0x086619c8]
0x8091376: LEA ESI, [EAX + EDX*0x8]
0x8091379: CMP dword ptr [ESI + 0x4], EBX
0x809137c: JLE 0x08091470
0x8091382: ADD EBX, dword ptr [ESI]
0x8091384: MOV EDX, dword ptr [0x086619cc]
0x809138a: CMP EBX, EDX
0x809138c: JL 0x08091404
0x809138e: LEA EAX, [EBX + EBX*0x2]
0x8091391: MOV ECX, dword ptr [0x08661964]
0x8091397: SHL EAX, 0x3
0x809139a: CMP byte ptr [EAX + ECX*0x1 + 0x4], 0x8
0x809139f: JZ 0x08091434
0x80913a5: TEST EDX, EDX
0x80913a7: LEA EBX, [EAX + ECX*0x1]
0x80913aa: JZ 0x080913e4
0x80913ac: LEA EAX, [EDX + -0x1]
0x80913af: MOV [0x086619cc], EAX
0x80913b4: LEA EAX, [EAX + EAX*0x2]
0x80913b7: MOV byte ptr [ECX + EAX*0x8 + 0x4], 0x8
0x80913bc: MOV EAX, [0x086619cc]
0x80913c1: LEA EAX, [EAX + EAX*0x2]
0x80913c4: MOV ECX, dword ptr [0x08661964]
0x80913ca: MOV DL, byte ptr [EBX + 0x5]
0x80913cd: SHL EAX, 0x3
0x80913d0: INC dword ptr [EBX]
0x80913d2: MOV dword ptr [EAX + ECX*0x1 + 0x8], EBX
0x80913d6: MOV byte ptr [EAX + ECX*0x1 + 0x5], DL
0x80913da: LEA ESP, [EBP + -0x8]
0x80913dd: POP EBX
0x80913de: POP ESI
0x80913df: LEAVE
0x80913e0: RET
0x80913e4: SUB ESP, 0xc
0x80913e7: PUSH 0x82ad98c
0x80913ec: CALL 0x08097b24
0x80913f1: ADD ESP, 0x10
0x80913f4: MOV EDX, dword ptr [0x086619cc]
0x80913fa: MOV ECX, dword ptr [0x08661964]
0x8091400: JMP 0x080913ac
0x8091404: SUB ESP, 0xc
0x8091407: PUSH 0x82acfc5
0x809140c: CALL 0x08097b24
0x8091411: LEA EAX, [EBX + EBX*0x2]
0x8091414: MOV ECX, dword ptr [0x08661964]
0x809141a: SHL EAX, 0x3
0x809141d: ADD ESP, 0x10
0x8091420: CMP byte ptr [EAX + ECX*0x1 + 0x4], 0x8
0x8091425: MOV EDX, dword ptr [0x086619cc]
0x809142b: JNZ 0x080913a5
0x8091431: LEA ESI, [ESI]
0x8091434: TEST EDX, EDX
0x8091436: LEA EBX, [EAX + ECX*0x1]
0x8091439: JZ 0x080914b9
0x809143b: DEC EDX
0x809143c: SUB ESP, 0x8
0x809143f: LEA EAX, [EDX + EDX*0x2]
0x8091442: MOV dword ptr [0x086619cc], EDX
0x8091448: MOV byte ptr [ECX + EAX*0x8 + 0x4], 0x0
0x809144d: PUSH EBX
0x809144e: MOV EAX, [0x086619cc]
0x8091453: MOV EDX, dword ptr [0x08661964]
0x8091459: LEA EAX, [EAX + EAX*0x2]
0x809145c: LEA EAX, [EDX + EAX*0x8]
0x809145f: PUSH EAX
0x8091460: CALL 0x0808be24
0x8091465: ADD ESP, 0x10
0x8091468: LEA ESP, [EBP + -0x8]
0x809146b: POP EBX
0x809146c: POP ESI
0x809146d: LEAVE
0x809146e: RET
0x8091470: SUB ESP, 0xc
0x8091473: PUSH 0x82acfc5
0x8091478: CALL 0x08097b24
0x809147d: ADD ESP, 0x10
0x8091480: JMP 0x08091382
0x8091488: SUB ESP, 0xc
0x809148b: PUSH 0x82acfc5
0x8091490: CALL 0x08097b24
0x8091495: ADD ESP, 0x10
0x8091498: MOV EAX, [0x086619c0]
0x809149d: JMP 0x0809136e
0x80914a4: SUB ESP, 0xc
0x80914a7: PUSH 0x82ad98c
0x80914ac: CALL 0x08097b24
0x80914b1: ADD ESP, 0x10
0x80914b4: JMP 0x08091361
0x80914b9: SUB ESP, 0xc
0x80914bc: PUSH 0x82ad98c
0x80914c1: CALL 0x08097b24
0x80914c6: ADD ESP, 0x10
0x80914c9: MOV EDX, dword ptr [0x086619cc]
0x80914cf: MOV ECX, dword ptr [0x08661964]
0x80914d5: JMP 0x0809143b
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-pushlocalvptr"></a>

## vm_pushLocalVptr / procedure_pseudo_code

Evidence: `ev_cdf3f69a6cd4c53275ebfcb955c6419230978dcbe6c1af8e4d0c1dc6d6c32de2`

```c

void vm_pushLocalVptr(int no)

{
  Vm_FunCall_Context_T *pVVar1;
  Vm_Value_T *pVVar2;
  uchar uVar3;
  Vm_Value_T *pVVar4;
  int iVar5;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg6(ESI)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (vm_funCp == 0) {
    vm_error("No such argument.\n");
  }
  pVVar1 = vm_funContext + vm_funCp;
  if (pVVar1->argNum <= no) {
    vm_error("No such argument.\n");
  }
  iVar5 = no + pVVar1->bp;
  if (iVar5 < vm_sp) {
    vm_error("No such argument.\n");
    uVar3 = vm_stack[iVar5].type;
  }
  else {
    uVar3 = vm_stack[iVar5].type;
  }
  if (uVar3 != '\b') {
    pVVar2 = vm_stack + iVar5;
    if (vm_sp == 0) {
      vm_error("Stack overflow when calculating.\n");
    }
    vm_sp = vm_sp + -1;
    vm_stack[vm_sp].type = '\b';
    iVar5 = vm_sp;
    pVVar4 = vm_stack;
    uVar3 = pVVar2->attrib;
    pVVar2->ref = pVVar2->ref + 1;
    pVVar4[iVar5].v.pPtrValue = pVVar2;
    pVVar4[iVar5].attrib = uVar3;
    return;
  }
  pVVar2 = vm_stack + iVar5;
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_copyValue(vm_stack + vm_sp,pVVar2);
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

## vm_pushLocalVptr / procedure_references

Evidence: `ev_bf12abf3078ae8de6c1b4870cb5ef90c0f592c3fe4560ffb37250560f1a993ec`

```json
{
  "direction": "incoming",
  "procedure": {
    "address": "0x809134c",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": true,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x80913e0",
          "start": "0x809134c"
        },
        {
          "end": "0x8091401",
          "start": "0x80913e4"
        },
        {
          "end": "0x809146e",
          "start": "0x8091404"
        },
        {
          "end": "0x8091484",
          "start": "0x8091470"
        },
        {
          "end": "0x80914a1",
          "start": "0x8091488"
        },
        {
          "end": "0x80914d9",
          "start": "0x80914a4"
        }
      ],
      "span_bytes": 398,
      "total_bytes": 387
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_pushLocalVptr"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809135b",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x80914a4",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x8091368",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x8091488",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809137c",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x8091470",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809138c",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x8091404",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809139f",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x8091434",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80913aa",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x80913e4",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x8091400",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x80913ac",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809142b",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x80913a5",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x8091439",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x80914b9",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x8091480",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x8091382",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x809149d",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x809136e",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80914b4",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x8091361",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x80914d5",
      "source_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      },
      "target_address": "0x809143b",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
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
      },
      "source_address": "0x8091636",
      "source_procedure": {
        "address": "0x80915dc",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8091653",
              "start": "0x80915dc"
            }
          ],
          "span_bytes": 120,
          "total_bytes": 120
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptrI"
      },
      "target_address": "0x809134c",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x83cccc4",
      "source_procedure": null,
      "target_address": "0x809134c",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": ".debug_frame:0xbbdc",
      "source_procedure": null,
      "target_address": "0x809134c",
      "target_procedure": {
        "address": "0x809134c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80913e0",
              "start": "0x809134c"
            },
            {
              "end": "0x8091401",
              "start": "0x80913e4"
            },
            {
              "end": "0x809146e",
              "start": "0x8091404"
            },
            {
              "end": "0x8091484",
              "start": "0x8091470"
            },
            {
              "end": "0x80914a1",
              "start": "0x8091488"
            },
            {
              "end": "0x80914d9",
              "start": "0x80914a4"
            }
          ],
          "span_bytes": 398,
          "total_bytes": 387
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_pushLocalVptr"
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

<a id="vm-pushlocalvptri"></a>

## vm_pushLocalVptrI / procedure_pseudo_code

Evidence: `ev_ec02e5266dd7cdabb5badbfbbbd1f96b4c28ebe63188afc23d23f0fff7f53866`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_pushLocalVptrI(void)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  
                    /* Unresolved local var: int no@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)] */
  pVVar1 = vm_getValueFromStack(0,0xff);
  if (pVVar1->type == '\x03') {
    iVar2 = _vm_ftol((pVVar1->v).real);
    (pVVar1->v).number = iVar2;
    pVVar1->type = '\x02';
  }
  else if (pVVar1->type != '\x02') {
    vm_error("Bad index for argument.\n");
  }
  iVar2 = (pVVar1->v).number;
  vm_cleanupValue(vm_stack + vm_sp);
  vm_sp = vm_sp + 1;
  vm_pushLocalVptr(iVar2 + -1);
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

<a id="vm-pushobjvptr"></a>

## vm_pushObjVptr / procedure_pseudo_code

Evidence: `ev_525701540b4de9914051ebdb9af2e55a2baac12f24fdacd8fe87d50535876b6f`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-pushvptr"></a>

## vm_pushVptr / procedure_pseudo_code

Evidence: `ev_84840f7531d08cddf363ae350d16487e6588a33a203d0cdbc63dd3c0ff8f68ac`

```c

void vm_pushVptr(Vm_Value_T *pValue)

{
  uchar uVar1;
  Vm_Value_T *pVVar2;
  int iVar3;
  
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\b';
  iVar3 = vm_sp;
  pVVar2 = vm_stack;
  uVar1 = pValue->attrib;
  pValue->ref = pValue->ref + 1;
  pVVar2[iVar3].v.string = (Vm_String_T *)pValue;
  pVVar2[iVar3].attrib = uVar1;
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

<a id="vm-querymember"></a>

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

## vm_queryMemberPtr / procedure_assembly

Evidence: `ev_eda665f39ff600270e7c381c4fcb9ee162e674d26a060d8afb345e0a012dc7b4`

```asm
0x808f8e4: PUSH EBP
0x808f8e5: MOV EBP, ESP
0x808f8e7: PUSH EDI
0x808f8e8: PUSH ESI
0x808f8e9: PUSH EBX
0x808f8ea: SUB ESP, 0xc
0x808f8ed: MOV EAX, dword ptr [EBP + 0x8]
0x808f8f0: MOV dword ptr [EBP + -0x10], EAX
0x808f8f3: MOV EAX, [0x086619cc]
0x808f8f8: LEA EAX, [EAX + EAX*0x2]
0x808f8fb: MOV EDX, dword ptr [0x08661964]
0x808f901: LEA EBX, [EDX + EAX*0x8]
0x808f904: LEA EDI, [EBX + 0x18]
0x808f907: TEST byte ptr [EDI + 0x5], 0x1
0x808f90b: MOV ESI, dword ptr [EDI + 0x8]
0x808f90e: JNZ 0x0808f984
0x808f910: MOV AL, byte ptr [ESI + 0x4]
0x808f913: MOVZX EDX, AL
0x808f916: CMP EDX, 0x5
0x808f919: JZ 0x0808f9a0
0x808f91f: JG 0x0808f950
0x808f921: CMP EDX, 0x4
0x808f924: JZ 0x0808fa1f
0x808f92a: SUB ESP, 0xc
0x808f92d: MOVZX EAX, AL
0x808f930: PUSH EAX
0x808f931: CALL 0x0808a13c
0x808f936: POP ECX
0x808f937: POP EBX
0x808f938: PUSH EAX
0x808f939: PUSH 0x82acfd8
0x808f93e: CALL 0x08097b24
0x808f943: ADD ESP, 0x10
0x808f946: LEA ESP, [EBP + -0xc]
0x808f949: POP EBX
0x808f94a: POP ESI
0x808f94b: POP EDI
0x808f94c: LEAVE
0x808f94d: RET
0x808f950: CMP EDX, 0x6
0x808f953: JZ 0x0808f9b4
0x808f955: CMP EDX, 0x7
0x808f958: JNZ 0x0808f92a
0x808f95a: PUSH EAX
0x808f95b: PUSH 0x0
0x808f95d: PUSH EBX
0x808f95e: PUSH dword ptr [ESI + 0x8]
0x808f961: CALL 0x0809d714
0x808f966: MOV dword ptr [EDI + 0x8], EAX
0x808f969: INC dword ptr [EAX]
0x808f96b: POP EDI
0x808f96c: PUSH ESI
0x808f96d: CALL 0x0808bfb4
0x808f972: ADD ESP, 0x10
0x808f975: LEA ESP, [EBP + -0xc]
0x808f978: POP EBX
0x808f979: POP ESI
0x808f97a: POP EDI
0x808f97b: LEAVE
0x808f97c: JMP 0x0809061c
0x808f984: SUB ESP, 0xc
0x808f987: PUSH 0x82acff0
0x808f98c: CALL 0x08097b24
0x808f991: MOV AL, byte ptr [ESI + 0x4]
0x808f994: MOVZX EDX, AL
0x808f997: ADD ESP, 0x10
0x808f99a: CMP EDX, 0x5
0x808f99d: JNZ 0x0808f91f
0x808f99f: NOP
0x808f9a0: MOV dword ptr [EBP + 0x8], 0x82ad9e0
0x808f9a7: LEA ESP, [EBP + -0xc]
0x808f9aa: POP EBX
0x808f9ab: POP ESI
0x808f9ac: POP EDI
0x808f9ad: LEAVE
0x808f9ae: JMP 0x08097b24
0x808f9b4: MOV AL, byte ptr [EBX + 0x4]
0x808f9b7: CMP AL, 0x3
0x808f9b9: JZ 0x0808fa48
0x808f9bf: CMP AL, 0x2
0x808f9c1: JZ 0x0808f9d3
0x808f9c3: SUB ESP, 0xc
0x808f9c6: PUSH 0x82ad00d
0x808f9cb: CALL 0x08097b24
0x808f9d0: ADD ESP, 0x10
0x808f9d3: TEST byte ptr [EBP + -0x10], 0x1
0x808f9d7: JZ 0x0808fa2b
0x808f9d9: MOV EAX, dword ptr [ESI + 0x8]
0x808f9dc: MOV EDX, dword ptr [EBX + 0x8]
0x808f9df: MOV EAX, dword ptr [EAX + 0x4]
0x808f9e2: SUB EAX, EDX
0x808f9e4: MOV dword ptr [EBP + -0x14], EAX
0x808f9e7: MOV EAX, dword ptr [EBP + -0x14]
0x808f9ea: TEST EAX, EAX
0x808f9ec: JS 0x0808fa33
0x808f9ee: MOV EAX, dword ptr [ESI + 0x8]
0x808f9f1: MOV EDX, dword ptr [EBP + -0x14]
0x808f9f4: CMP dword ptr [EAX + 0x4], EDX
0x808f9f7: JLE 0x0808fa33
0x808f9f9: MOV EDX, dword ptr [EBP + -0x14]
0x808f9fc: MOV EAX, dword ptr [EAX + 0xc]
0x808f9ff: MOV EAX, dword ptr [EAX + EDX*0x4]
0x808fa02: SUB ESP, 0xc
0x808fa05: MOV dword ptr [EDI + 0x8], EAX
0x808fa08: INC dword ptr [EAX]
0x808fa0a: PUSH ESI
0x808fa0b: CALL 0x0808bfb4
0x808fa10: ADD ESP, 0x10
0x808fa13: LEA ESP, [EBP + -0xc]
0x808fa16: POP EBX
0x808fa17: POP ESI
0x808fa18: POP EDI
0x808fa19: LEAVE
0x808fa1a: JMP 0x0809061c
0x808fa1f: MOV dword ptr [EBP + 0x8], 0x82ada2c
0x808fa26: JMP 0x0808f9a7
0x808fa2b: MOV EBX, dword ptr [EBX + 0x8]
0x808fa2e: MOV dword ptr [EBP + -0x14], EBX
0x808fa31: JMP 0x0808f9e7
0x808fa33: SUB ESP, 0xc
0x808fa36: PUSH 0x82ada78
0x808fa3b: CALL 0x08097b24
0x808fa40: ADD ESP, 0x10
0x808fa43: MOV EAX, dword ptr [ESI + 0x8]
0x808fa46: JMP 0x0808f9f9
0x808fa48: SUB ESP, 0xc
0x808fa4b: PUSH dword ptr [EBX + 0x8]
0x808fa4e: CALL 0x080a36e4
0x808fa53: MOV dword ptr [EBX + 0x8], EAX
0x808fa56: MOV byte ptr [EBX + 0x4], 0x2
0x808fa5a: JMP 0x0808f9d0
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x808f97c to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f9ae to 0x8097b24 vm_error (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808fa1a to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-querymemberptr"></a>

## vm_queryMemberPtr / procedure_pseudo_code

Evidence: `ev_39ba42f6244debda18e95f0fa40350e37543e9ee85177f3d546be50234c8f76a`

```c

void vm_queryMemberPtr(int flag)

{
  Vm_Value_T *lv;
  byte bVar1;
  Vm_Value_T *pValue;
  Vm_String_T *pVVar2;
  char *pcVar3;
  Vm_Value_T *pVVar4;
  Vm_Object *pVVar5;
  int iVar6;
  uint uVar7;
  int offset;
  int flag_local;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pNewIndex@[DW_OP_reg0(EAX)] */
  lv = vm_stack + vm_sp;
  pValue = lv[1].v.pPtrValue;
  if ((lv[1].attrib & 1) == 0) {
    bVar1 = pValue->type;
    uVar7 = (uint)bVar1;
    iVar6 = uVar7 - 5;
    if (uVar7 == 5) {
LAB_0808f9a0:
      flag = (int)s_Can_t_get_pointer_inside_buffer__082ad9e0;
      goto LAB_0808f9a7;
    }
  }
  else {
    vm_error("Can\'t modify a const value.\n");
    bVar1 = pValue->type;
    uVar7 = (uint)bVar1;
    iVar6 = uVar7 - 5;
    if (iVar6 == 0) goto LAB_0808f9a0;
  }
  if (SBORROW4(uVar7,5) == iVar6 < 0) {
    if (uVar7 == 6) {
      if (lv->type == '\x03') {
        iVar6 = _vm_ftol((lv->v).real);
        (lv->v).number = iVar6;
        lv->type = '\x02';
      }
      else if (lv->type != '\x02') {
        vm_error("Index should be integer.\n");
      }
      if ((flag & 1U) == 0) {
        offset = (lv->v).number;
      }
      else {
        offset = (int)((pValue->v).pLoopCtrl)->pValue - (lv->v).number;
      }
      if ((offset < 0) || (pVVar5 = (pValue->v).object, pVVar5->execRef <= offset)) {
        vm_error("Index of the array out of range.\n");
        pVVar5 = (Vm_Object *)(pValue->v).string;
      }
      pVVar2 = *(Vm_String_T **)(pVVar5->flags + offset * 4);
      lv[1].v.string = pVVar2;
      pVVar2->ref = pVVar2->ref + 1;
      vm_unrefValue(pValue);
      vm_popStack();
      return;
    }
    if (uVar7 == 7) {
      pVVar4 = vm_findForInsert((Vm_Mapping_T *)(pValue->v).string,lv,0);
      lv[1].v.string = (Vm_String_T *)pVVar4;
      pVVar4->ref = pVVar4->ref + 1;
      vm_unrefValue(pValue);
      vm_popStack();
      return;
    }
  }
  else if (uVar7 == 4) {
    flag = (int)s_Can_t_get_pointer_inside_string__082ada2c;
LAB_0808f9a7:
    vm_error((char *)flag);
    return;
  }
  pcVar3 = vm_valueType((uint)bVar1);
  vm_error("Bad type(%s) to index.\n",pcVar3);
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
- Ghidra Listing reports a terminal call at 0x808f97c to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808f9ae to 0x8097b24 vm_error (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- Ghidra Listing reports a terminal call at 0x808fa1a to 0x809061c vm_popStack (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_queryMemberPtr / procedure_references

Evidence: `ev_1870556bc9ea18617c8f4e1d0ac147a9371f14283382ea32a28f5431de2b896f`

```json
{
  "direction": "incoming",
  "procedure": {
    "address": "0x808f8e4",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": true,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x808f94d",
          "start": "0x808f8e4"
        },
        {
          "end": "0x808f980",
          "start": "0x808f950"
        },
        {
          "end": "0x808f9b2",
          "start": "0x808f984"
        },
        {
          "end": "0x808fa5e",
          "start": "0x808f9b4"
        }
      ],
      "span_bytes": 379,
      "total_bytes": 373
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_queryMemberPtr"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f90e",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f984",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f919",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9a0",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f91f",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f950",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f924",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa1f",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f953",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9b4",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f958",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f92a",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f99d",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f91f",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9b9",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa48",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9c1",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9d3",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9d7",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa2b",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9ec",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa33",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9f7",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa33",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa26",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9a7",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa31",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9e7",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa46",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9f9",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa5a",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9d0",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x83ccdf8",
      "source_procedure": null,
      "target_address": "0x808f8e4",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": ".debug_frame:0xb68c",
      "source_procedure": null,
      "target_address": "0x808f8e4",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
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

## vm_queryMemberPtr / procedure_references

Evidence: `ev_5ea261a561e7fada81b9209a8e598b9ebbaeaa5ed8a3c115c2c60d68d79157ef`

```json
{
  "direction": "outgoing",
  "procedure": {
    "address": "0x808f8e4",
    "body": {
      "available": true,
      "contains_entry": true,
      "non_contiguous": true,
      "provenance": "ghidra-function-body-address-set",
      "ranges": [
        {
          "end": "0x808f94d",
          "start": "0x808f8e4"
        },
        {
          "end": "0x808f980",
          "start": "0x808f950"
        },
        {
          "end": "0x808f9b2",
          "start": "0x808f984"
        },
        {
          "end": "0x808fa5e",
          "start": "0x808f9b4"
        }
      ],
      "span_bytes": 379,
      "total_bytes": 373
    },
    "classification": {
      "external": false,
      "provenance": "ghidra-function-manager",
      "thunk": false,
      "thunk_target": null
    },
    "name": "vm_queryMemberPtr"
  },
  "reference_kinds_available": true,
  "references": [
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f8ed",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0x4",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x808f8f0",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffec",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f8f3",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x86619cc",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f8fb",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x8661964",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f90e",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f984",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f919",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9a0",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f91f",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f950",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f924",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa1f",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
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
      },
      "source_address": "0x808f931",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808a13c",
      "target_procedure": {
        "address": "0x808a13c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808a19a",
              "start": "0x808a13c"
            },
            {
              "end": "0x808a1d2",
              "start": "0x808a19c"
            },
            {
              "end": "0x808a207",
              "start": "0x808a1d4"
            }
          ],
          "span_bytes": 204,
          "total_bytes": 202
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_valueType"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f939",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x82acfd8",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f93e",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x8097b24",
      "target_procedure": {
        "address": "0x8097b24",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8097bfd",
              "start": "0x8097b24"
            },
            {
              "end": "0x8097c3b",
              "start": "0x8097c00"
            }
          ],
          "span_bytes": 280,
          "total_bytes": 278
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_error"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f946",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xfffffffffffffff0",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f953",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9b4",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f958",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f92a",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
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
      },
      "source_address": "0x808f961",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x809d714",
      "target_procedure": {
        "address": "0x809d714",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x809d74a",
              "start": "0x809d714"
            }
          ],
          "span_bytes": 55,
          "total_bytes": 55
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_findForInsert"
      }
    },
    {
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
      },
      "source_address": "0x808f96d",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808bfb4",
      "target_procedure": {
        "address": "0x808bfb4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c005",
              "start": "0x808bfb4"
            },
            {
              "end": "0x808c026",
              "start": "0x808c008"
            }
          ],
          "span_bytes": 115,
          "total_bytes": 113
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_unrefValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f975",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xfffffffffffffff0",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f97c",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x809061c",
      "target_procedure": {
        "address": "0x809061c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8090640",
              "start": "0x809061c"
            }
          ],
          "span_bytes": 37,
          "total_bytes": 37
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_popStack"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f987",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x82acff0",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f98c",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x8097b24",
      "target_procedure": {
        "address": "0x8097b24",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8097bfd",
              "start": "0x8097b24"
            },
            {
              "end": "0x8097c3b",
              "start": "0x8097c00"
            }
          ],
          "span_bytes": 280,
          "total_bytes": 278
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_error"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f99d",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f91f",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x808f9a0",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0x4",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f9a0",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x82ad9e0",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f9a7",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xfffffffffffffff0",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f9ae",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x8097b24",
      "target_procedure": {
        "address": "0x8097b24",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8097bfd",
              "start": "0x8097b24"
            },
            {
              "end": "0x8097c3b",
              "start": "0x8097c00"
            }
          ],
          "span_bytes": 280,
          "total_bytes": 278
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_error"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9b9",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa48",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9c1",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9d3",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808f9c6",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x82ad00d",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808f9cb",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x8097b24",
      "target_procedure": {
        "address": "0x8097b24",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8097bfd",
              "start": "0x8097b24"
            },
            {
              "end": "0x8097c3b",
              "start": "0x8097c00"
            }
          ],
          "span_bytes": 280,
          "total_bytes": 278
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_error"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f9d3",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffec",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9d7",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa2b",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x808f9e4",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffe8",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f9e7",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffe8",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9ec",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa33",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f9f1",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffe8",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": true,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "CONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808f9f7",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808fa33",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": true,
        "terminal": false,
        "type": "READ",
        "write": false
      },
      "source_address": "0x808f9f9",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffe8",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808fa0b",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808bfb4",
      "target_procedure": {
        "address": "0x808bfb4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808c005",
              "start": "0x808bfb4"
            },
            {
              "end": "0x808c026",
              "start": "0x808c008"
            }
          ],
          "span_bytes": 115,
          "total_bytes": 113
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_unrefValue"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808fa13",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xfffffffffffffff0",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808fa1a",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x809061c",
      "target_procedure": {
        "address": "0x809061c",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8090640",
              "start": "0x809061c"
            }
          ],
          "span_bytes": 37,
          "total_bytes": 37
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_popStack"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x808fa1f",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0x4",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 1,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808fa1f",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x82ada2c",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa26",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9a7",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "WRITE",
        "write": true
      },
      "source_address": "0x808fa2e",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "stack:0xffffffffffffffe8",
      "target_procedure": null
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa31",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9e7",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": true,
        "external": false,
        "flow": false,
        "indirect": false,
        "jump": false,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "DATA",
        "write": false
      },
      "source_address": "0x808fa36",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x82ada78",
      "target_procedure": null
    },
    {
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
      },
      "source_address": "0x808fa3b",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x8097b24",
      "target_procedure": {
        "address": "0x8097b24",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x8097bfd",
              "start": "0x8097b24"
            },
            {
              "end": "0x8097c3b",
              "start": "0x8097c00"
            }
          ],
          "span_bytes": 280,
          "total_bytes": 278
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_error"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa46",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9f9",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      }
    },
    {
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
      },
      "source_address": "0x808fa4e",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x80a36e4",
      "target_procedure": {
        "address": "0x80a36e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": false,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x80a36ed",
              "start": "0x80a36e4"
            }
          ],
          "span_bytes": 10,
          "total_bytes": 10
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "_vm_ftol"
      }
    },
    {
      "kind": {
        "available": true,
        "call": false,
        "computed": false,
        "conditional": false,
        "data": false,
        "external": false,
        "flow": true,
        "indirect": false,
        "jump": true,
        "operand_index": 0,
        "primary": true,
        "provenance": "ghidra-reference-manager",
        "read": false,
        "terminal": false,
        "type": "UNCONDITIONAL_JUMP",
        "write": false
      },
      "source_address": "0x808fa5a",
      "source_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
      },
      "target_address": "0x808f9d0",
      "target_procedure": {
        "address": "0x808f8e4",
        "body": {
          "available": true,
          "contains_entry": true,
          "non_contiguous": true,
          "provenance": "ghidra-function-body-address-set",
          "ranges": [
            {
              "end": "0x808f94d",
              "start": "0x808f8e4"
            },
            {
              "end": "0x808f980",
              "start": "0x808f950"
            },
            {
              "end": "0x808f9b2",
              "start": "0x808f984"
            },
            {
              "end": "0x808fa5e",
              "start": "0x808f9b4"
            }
          ],
          "span_bytes": 379,
          "total_bytes": 373
        },
        "classification": {
          "external": false,
          "provenance": "ghidra-function-manager",
          "thunk": false,
          "thunk_target": null
        },
        "name": "vm_queryMemberPtr"
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

## vm_queryRangePtr / procedure_assembly

Evidence: `ev_ee4e18126002b60c36b3820be4291ff2ef01d3d9fbf8ab2c06dea8f0d219da69`

```asm
0x809005c: PUSH EBP
0x809005d: MOV EBP, ESP
0x809005f: MOV dword ptr [EBP + 0x8], 0x82adaf8
0x8090066: LEAVE
0x8090067: JMP 0x08097b24
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- Ghidra Listing reports a terminal call at 0x8090067 to 0x8097b24 vm_error (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-queryrangeptr"></a>

## vm_queryRangePtr / procedure_pseudo_code

Evidence: `ev_632b1f05dbc861527e3897578f8fd248be43de594040f3bc2a718cdf982e9969`

```c

/* WARNING: Unknown calling convention */

void vm_queryRangePtr(int flag)

{
  vm_error("Can\'t modify range of a value.\n");
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
- Ghidra Listing reports a terminal call at 0x8090067 to 0x8097b24 vm_error (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-refreshallvaluesactive"></a>

## vm_refreshAllValuesActive / procedure_pseudo_code

Evidence: `ev_11d1d264620ab7f5452f093971acc5d836f0364c7d522a7234d1af509587c5fd`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_refreshAllValuesActive(void)

{
  Vm_Value *pVVar1;
  Vm_PCB_T *pVVar2;
  ushort *puVar3;
  Vm_Timer *pVVar4;
  Vm_Pend_Object_T *pVVar5;
  int iVar6;
  Vm_Object_T *pVVar7;
  undefined4 *puVar8;
  Vm_Mark_Value_Info *pVVar9;
  int iVar10;
  uint oid;
  bool bVar11;
  Vm_Tid_T tid;
  int i;
  
                    /* Unresolved local var: Vm_Object_T * pObject@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Mark_Value_Info_T *
                       pMarkActiveInfo@[DW_OP_reg3(EBX)] */
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg0(EAX)] */
  for (pVVar1 = vm_allValues; pVVar1 != (Vm_Value_T *)0x0; pVVar1 = pVVar1->pNext) {
    pVVar1->active = 0;
  }
  i = 0;
  if (vm_objectHashTable != (Vm_Object_T **)0x0) {
    do {
      for (pVVar7 = vm_objectHashTable[i]; pVVar7 != (Vm_Object_T *)0x0;
          pVVar7 = pVVar7->pNextHashObject) {
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
        iVar10 = 0;
        if (0 < pVVar7->pProgram->maxVariableNo) {
          iVar6 = 0;
          do {
            vm_markActiveFromValue((Vm_Value_T *)((int)&pVVar7->pObjectValues->ref + iVar6));
            iVar10 = iVar10 + 1;
            iVar6 = iVar6 + 0x18;
          } while (iVar10 < pVVar7->pProgram->maxVariableNo);
        }
      }
      i = i + 1;
    } while (i < 0x4000);
  }
                    /* Unresolved local var: int i@[DW_OP_reg6(ESI)] */
  iVar10 = 0;
  if (0 < vm_globalVar) {
    iVar6 = 0;
    do {
      iVar10 = iVar10 + 1;
      vm_markActiveFromValue(*(Vm_Value_T **)((int)&vm_pGlobalVar->pValue + iVar6));
      iVar6 = iVar6 + 0xc;
    } while (iVar10 < vm_globalVar);
  }
                    /* Unresolved local var: Vm_PCB_T * pPCB@[DW_OP_reg7(EDI)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  tid = 0;
  bVar11 = vm_maxProcess != 0;
  while (bVar11) {
    pVVar2 = vm_getRawPCB(tid);
    iVar10 = pVVar2->sp;
    if (0 < iVar10) {
      puVar3 = &pVVar2->stack->active;
      iVar6 = iVar10;
      do {
        *puVar3 = 1;
        puVar3 = puVar3 + 0xc;
        iVar6 = iVar6 + -1;
      } while (iVar6 != 0);
    }
    if (iVar10 < pVVar2->stackSize) {
      iVar6 = iVar10 * 0x18;
      do {
        iVar10 = iVar10 + 1;
        vm_markActiveFromValue((Vm_Value_T *)((int)&pVVar2->stack->ref + iVar6));
        iVar6 = iVar6 + 0x18;
      } while (iVar10 < pVVar2->stackSize);
    }
    tid = tid + 1;
    bVar11 = tid < vm_maxProcess;
  }
                    /* Unresolved local var: Vm_Timer_T * pTimer@[DW_OP_reg3(EBX)] */
  for (pVVar4 = vm_getTimerList(); pVVar4 != (Vm_Timer_T *)0x0; pVVar4 = pVVar4->pNext) {
    if (pVVar4->type == 2) {
      vm_markActiveFromValue((pVVar4->v).pValue);
    }
  }
                    /* Unresolved local var: Vm_Lib_Queue_Element_T * pElement@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Pend_Object_T * pObject@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Lib_Queue_T * pQueue@[???]
                       Unresolved local var: Vm_Oid_T oid@[DW_OP_reg6(ESI)] */
  for (oid = 0; pVVar9 = _allMarkActiveInfo, oid < vm_maxPendObject; oid = oid + 1) {
    pVVar5 = vm_getPendObject(0xd,oid);
    if (pVVar5 != (Vm_Pend_Object_T *)0x0) {
      for (puVar8 = *(undefined4 **)(pVVar5[1].name + 0xc); puVar8 != (undefined4 *)0x0;
          puVar8 = (undefined4 *)puVar8[2]) {
        vm_markActiveFromValue((Vm_Value_T *)*puVar8);
      }
    }
  }
  for (; pVVar9 != (Vm_Mark_Value_Info_T *)0x0; pVVar9 = pVVar9->pNext) {
    (*pVVar9->markFunc)(pVVar9->pCookie);
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

<a id="vm-returnfromfun"></a>

## vm_returnFromFun / procedure_pseudo_code

Evidence: `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-safefindforinsert"></a>

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

<a id="vm-setlocalvar"></a>

## vm_setLocalVar / procedure_pseudo_code

Evidence: `ev_fc0b85e738fdd420e13765dad0b8b0351ca9b42d06c1a76d4f41bdc1c23297dc`

```c

void vm_setLocalVar(int no)

{
  Vm_FunCall_Context_T *pVVar1;
  Vm_Value_T *pValue;
  Vm_Value_T *pVVar2;
  int iVar3;
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg6(ESI)]
                       Unresolved local var: Vm_Value_T * pValue1@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue2@[DW_OP_reg6(ESI)] */
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  if (vm_funCp == 0) {
    vm_error("No such argument.\n");
  }
  pVVar1 = vm_funContext + vm_funCp;
  if (pVVar1->argNum <= no) {
    vm_error("No such argument.\n");
  }
  iVar3 = no + pVVar1->bp;
  if (iVar3 < vm_sp) {
    vm_error("No such argument.\n");
  }
  pVVar2 = vm_stack + iVar3;
  pValue = vm_stack + vm_sp;
  if (pVVar2->type != '\b') {
    vm_cleanupValue(pVVar2);
    vm_copyValue(pVVar2,pValue);
    return;
  }
                    /* Unresolved local var: Vm_Value_T * pIndex@[DW_OP_reg3(EBX)] */
  if (pValue->type == '\b') {
    vm_error("Can not set a value which is reference type.\n");
  }
  pVVar2 = (Vm_Value_T *)(pVVar2->v).string;
  vm_cleanupValue(pVVar2);
  vm_copyValue(pVVar2,pValue);
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

<a id="vm-setobjvar"></a>

## vm_setObjVar / procedure_pseudo_code

Evidence: `ev_85a706f75e7ed097afaab10b0d9474c379d806b7272d872c2ea29584a5f56438`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-stacktopvalue"></a>

## vm_stackTopValue / procedure_pseudo_code

Evidence: `ev_a214d11173e3b233267bdc24c3c70e7956ca4a1fc14d283c514f5e13cedd06a2`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

Vm_Value * vm_stackTopValue(void)

{
  return vm_stack + vm_sp;
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

## vm_takeOffValueFromStack / procedure_assembly

Evidence: `ev_c2177a4cc3c48912f1626bc399600d302a4c708f69bf066dade3508055bf7119`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-takeoffvaluefromstack"></a>

## vm_takeOffValueFromStack / procedure_pseudo_code

Evidence: `ev_99c34e4e373606ceaa48084b7cbe89c70742d22173dfb81bb36c8bddcd94b778`

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

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_unrefArray / procedure_assembly

Evidence: `ev_8000dac56416a0239afddc630d10d5c425e7d9797f623755e77dcda7e5516c26`

```asm
0x809b350: PUSH EBP
0x809b351: MOV EBP, ESP
0x809b353: PUSH EBX
0x809b354: PUSH EBX
0x809b355: MOV EBX, dword ptr [EBP + 0x8]
0x809b358: MOV EAX, dword ptr [EBX]
0x809b35a: DEC EAX
0x809b35b: TEST EAX, EAX
0x809b35d: MOV dword ptr [EBX], EAX
0x809b35f: JLE 0x0809b368
0x809b361: MOV EBX, dword ptr [EBP + -0x4]
0x809b364: LEAVE
0x809b365: RET
0x809b368: MOV EAX, dword ptr [EBX + 0x4]
0x809b36b: DEC EAX
0x809b36c: CMP EAX, -0x1
0x809b36f: MOV dword ptr [EBX + 0x4], EAX
0x809b372: JZ 0x0809b3ac
0x809b374: MOV EDX, EAX
0x809b376: JMP 0x0809b385
0x809b378: LEA EAX, [EDX + -0x1]
0x809b37b: CMP EAX, -0x1
0x809b37e: MOV EDX, EAX
0x809b380: MOV dword ptr [EBX + 0x4], EAX
0x809b383: JZ 0x0809b3ac
0x809b385: MOV EAX, dword ptr [EBX + 0xc]
0x809b388: MOV EAX, dword ptr [EAX + EDX*0x4]
0x809b38b: TEST EAX, EAX
0x809b38d: JZ 0x0809b378
0x809b38f: SUB ESP, 0xc
0x809b392: PUSH EAX
0x809b393: CALL 0x0808bfb4
0x809b398: MOV EDX, dword ptr [EBX + 0x4]
0x809b39b: LEA EAX, [EDX + -0x1]
0x809b39e: ADD ESP, 0x10
0x809b3a1: CMP EAX, -0x1
0x809b3a4: MOV EDX, EAX
0x809b3a6: MOV dword ptr [EBX + 0x4], EAX
0x809b3a9: JNZ 0x0809b385
0x809b3ab: NOP
0x809b3ac: MOV EAX, [0x086041a0]
0x809b3b1: SUB EAX, dword ptr [EBX + 0x8]
0x809b3b4: DEC dword ptr [0x086041a4]
0x809b3ba: MOV [0x086041a0], EAX
0x809b3bf: PUSH 0x1ac
0x809b3c4: PUSH 0x82af390
0x809b3c9: PUSH 0x82a376b
0x809b3ce: PUSH EBX
0x809b3cf: CALL 0x080ceb94
0x809b3d4: ADD ESP, 0x10
0x809b3d7: MOV EBX, dword ptr [EBP + -0x4]
0x809b3da: LEAVE
0x809b3db: RET
```

限制：
- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-unrefarray"></a>

## vm_unrefArray / procedure_pseudo_code

Evidence: `ev_3a18d97f17b48e830607f072dd18b5dcc20f00748883431276baba280c0ab592`

```c

void vm_unrefArray(Vm_Array_T *pArray)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = pArray->ref + -1;
  pArray->ref = iVar1;
  if (0 < iVar1) {
    return;
  }
  uVar2 = pArray->size - 1;
  pArray->size = uVar2;
  while (uVar2 != 0xffffffff) {
    while (pArray->pValues[uVar2] == (Vm_Value_T *)0x0) {
      uVar2 = uVar2 - 1;
      pArray->size = uVar2;
      if (uVar2 == 0xffffffff) goto LAB_0809b3ac;
    }
    vm_unrefValue(pArray->pValues[uVar2]);
    uVar2 = pArray->size - 1;
    pArray->size = uVar2;
  }
LAB_0809b3ac:
  totalArraySize = totalArraySize - pArray->maxSize;
  totalArrayNum = totalArrayNum + -1;
  pdb_freeMemory(pArray,"VM","/home/cc/xxxx/gameshell/src/vm/vm_array.c",0x1ac);
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

<a id="vm-unrefmapping"></a>

## vm_unrefMapping / procedure_pseudo_code

Evidence: `ev_d7c17baca06cfda52f8044cdba3a5aa752e6810b7a2335c3929fe6c7a82442f4`

```c

void vm_unrefMapping(Vm_Mapping_T *m)

{
  Vm_Mapping_Node_T **ptr;
  Vm_Mapping_Node *pVVar1;
  int iVar2;
  Vm_Mapping_Node_T *pElt;
  uint uVar3;
  Vm_Mapping_Node_T **ppa;
  
  iVar2 = m->ref + -1;
  m->ref = iVar2;
  if (0 < iVar2) {
    return;
  }
                    /* Unresolved local var: int j@[DW_OP_reg6(ESI)]
                       Unresolved local var: int c@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Mapping_Node_T * pElt@[DW_OP_reg0(EAX)]
                       Unresolved local var: Vm_Mapping_Node_T * pNelt@[DW_OP_reg3(EBX)] */
  ptr = m->table;
  uVar3 = m->tableSize;
  totalMappingSize = totalMappingSize + (m->count * 0xd + uVar3) * -4 + -0x1c;
  totalMappingNodes = totalMappingNodes - m->count;
  do {
    pElt = ptr[uVar3];
    while (pElt != (Vm_Mapping_Node_T *)0x0) {
      pVVar1 = pElt->pNext;
      vm_freeNode(pElt);
      pElt = pVVar1;
    }
    uVar3 = uVar3 - 1;
  } while (uVar3 != 0xffffffff);
  totalMappingNum = totalMappingNum + -1;
  pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_mapping.c",0xeb);
  pdb_freeTinyBlock(m);
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
- Ghidra Listing reports a terminal call at 0x809c519 to 0x80cf214 pdb_freeTinyBlock (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-unrefvalue"></a>

## vm_unrefValue / procedure_pseudo_code

Evidence: `ev_7caa046f8081143ac8be588f8afdae8a94f7aa7a36d7146bc503150e7f9475a0`

```c

void vm_unrefValue(Vm_Value_T *pValue)

{
  Vm_Value *pVVar1;
  int iVar2;
  
  iVar2 = pValue->ref + -1;
  pValue->ref = iVar2;
  if ((iVar2 == 0) && (pValue != (Vm_Value_T *)0x0)) {
    vm_cleanupValue(pValue);
    vm_totalValues = vm_totalValues + -1;
    if (pValue == vm_allValues) {
      vm_allValues = pValue->pNext;
      if (vm_allValues != (Vm_Value_T *)0x0) {
        vm_allValues->pPrev = (Vm_Value *)0x0;
      }
    }
    else {
      pVVar1 = pValue->pPrev;
      pVVar1->pNext = pValue->pNext;
      if (pValue->pNext != (Vm_Value *)0x0) {
        pValue->pNext->pPrev = pVVar1;
      }
    }
    pValue->pThis = (Vm_Value *)0x0;
    pdb_freeTinyBlock(pValue);
    return;
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
- Ghidra Listing reports a terminal call at 0x808c001 to 0x80cf214 pdb_freeTinyBlock (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

<a id="vm-regularprocessor"></a>
## vm_regularProcessor（复用既有证据）

Evidence: `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`

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

局限与完整汇编/引用保留于 reused-dispatch.json.gz。