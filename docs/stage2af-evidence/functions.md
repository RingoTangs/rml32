# 阶段 2AF 原生证据可读视图

伪代码与汇编是Ghidra观察，不是原始C源码。完整dossier（含raw_result/pcode）与限制保留在压缩包。

## vm_callFunPtr / analyze_function

Evidence: `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d`

Address: `0x809441c`; source: `docs/stage2af-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention */

int vm_callFunPtr(int mayNull,int argn)

{
  uchar uVar1;
  anon_union_4_11_5000f750_for_v aVar2;
  ushort uVar3;
  char *pcVar4;
  int iVar5;
  Vm_Ident_Hash_Elem_T *pVVar6;
  int iVar7;
  Vm_Fun_T *pFun;
  int *piVar8;
  bool bVar9;
  bool bVar10;
  Vm_Object_T *pToObject;
  Vm_Index_T VVar11;
  int n;
  void *pInfo;
  int type;
  int isGlobal;
  Vm_Object_T *pObject;
  void *osPtr;
  Vm_Index_T funOffset;
  int argv [16];
  Vm_Value_T value;
  
                    /* Unresolved local var: Vm_Ident_Hash_Elem_T * ihe@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Fun_Ptr_T * function@[???]
                       Unresolved local var: int i@[DW_OP_reg7(EDI)] */
  if (argn < 1) {
    vm_error("Missed function to call.\n");
  }
  vm_initValue(&value);
  value.type = vm_stack[vm_sp].type;
  aVar2 = (anon_union_4_11_5000f750_for_v)vm_stack[vm_sp].v.string;
  value.attrib = vm_stack[vm_sp].attrib;
  n = argn + -1;
  vm_stack[vm_sp].type = '\0';
  vm_stack[vm_sp].attrib = '\0';
  vm_sp = vm_sp + 1;
  value.v = aVar2;
  if (value.type != '\x04') {
    if (value.type != '\t') {
      vm_destructStaticValue(&value);
      vm_error("Bad type of the value to call, expected a function or string.\n");
      return 1;
    }
    if (vm_sp < (aVar2.function)->argn) {
      vm_destructStaticValue(&value);
      vm_error("Stack overflow when do function call.\n");
    }
    if (((Vm_Object *)(aVar2.pPtrValue)->pNext != (Vm_Object *)0x0) &&
       ((((Vm_Object *)(aVar2.pPtrValue)->pNext)->flags & 0x8000) != 0)) {
      vm_destructStaticValue(&value);
      vm_error("Owner object is destructed, can\'t invoke function.\n");
    }
    isGlobal = (int)(aVar2.pPtrValue)->attrib;
    uVar1 = (aVar2.pPtrValue)->type;
    funOffset = (aVar2.function)->funOffset;
    pFun = (Vm_Fun_T *)((anon_union_4_3_7473fe5b_for_f *)&(aVar2.pPtrValue)->pPrev)->pEfun;
    pObject = (Vm_Object_T *)0x0;
    if (isGlobal == 0) {
      pObject = (Vm_Object_T *)(aVar2.pPtrValue)->pNext;
    }
    iVar7 = (int)(aVar2.function)->argn;
    n = n + iVar7;
    if ((aVar2.string)->ref == 1) {
      iVar7 = iVar7 + -1;
      if (iVar7 != -1) {
        iVar5 = iVar7 * 0x18;
        do {
          vm_sp = vm_sp + -1;
          vm_stack[vm_sp].type = (&(aVar2.pPtrValue)->pThis->type)[iVar5];
          vm_stack[vm_sp].attrib = (&(aVar2.pPtrValue)->pThis->attrib)[iVar5];
          vm_stack[vm_sp].v =
               *(anon_union_4_11_5000f750_for_v *)((int)&(aVar2.pPtrValue)->pThis->v + iVar5);
          (&(aVar2.pPtrValue)->pThis->type)[iVar5] = '\0';
          iVar7 = iVar7 + -1;
          (&(aVar2.pPtrValue)->pThis->attrib)[iVar5] = '\0';
          iVar5 = iVar5 + -0x18;
        } while (iVar7 != -1);
      }
      vm_freeFunctionValue((Vm_Fun_Ptr_T *)aVar2.string);
      value.type = '\0';
      value.attrib = '\0';
      vm_destructStaticValue(&value);
    }
    else {
      iVar7 = iVar7 + -1;
      vm_destructStaticValue(&value);
      if (iVar7 != -1) {
        iVar5 = iVar7 * 0x18;
        do {
          iVar7 = iVar7 + -1;
          vm_pushValue((Vm_Value_T *)((int)&(aVar2.pPtrValue)->pThis->ref + iVar5));
          iVar5 = iVar5 + -0x18;
        } while (iVar7 != -1);
      }
    }
    if (uVar1 == 'K') {
      VVar11 = *(Vm_Index_T *)((int)&(pFun->codeSegment).metaDataLen + 2);
      goto LAB_0809483e;
    }
    pToObject = (Vm_Object_T *)0x0;
    VVar11 = funOffset;
    if (vm_currentContext->thisObject != pObject) {
      pToObject = pObject;
    }
    goto LAB_0809469d;
  }
  pVVar6 = (Vm_Ident_Hash_Elem_T *)0x0;
  pFun = (Vm_Fun *)0x0;
  if ((char)((Vm_Object *)(aVar2.pPtrValue)->pNext)->ref == '$') {
    osPtr = vm_lookupOSFun((char *)((int)&((Vm_Object *)(aVar2.pPtrValue)->pNext)->ref + 1));
    if (osPtr == (void *)0x0) {
      vm_destructStaticValue(&value);
      pFun = (Vm_Fun *)0x0;
      vm_error("No such OS function.\n");
    }
    else {
      pFun = (Vm_Fun *)0x0;
    }
  }
  else {
    osPtr = (void *)0x0;
    if (vm_currentContext->thisObject != (Vm_Object *)0x0) {
      pFun = vm_findProgramFunction
                       (vm_currentContext->thisObject->pProgram,aVar2.string,1,&funOffset);
      isGlobal = 0;
      if (pFun != (Vm_Fun *)0x0) goto LAB_08094670;
    }
    aVar2 = value.v;
    if ((value.v.string)->hash == 0) {
      uVar3 = vm_whashstr((value.v.string)->ptr,0x20);
      (aVar2.string)->hash = uVar3;
    }
    pVVar6 = vm_lookupIdentInProgram
                       (vm_pGlobalProgram,(value.v.string)->ptr,(uint)(value.v.string)->hash);
    isGlobal = 1;
  }
LAB_08094670:
  vm_destructStaticValue(&value);
  VVar11 = funOffset;
  if (pVVar6 == (Vm_Ident_Hash_Elem_T *)0x0) {
    if (pFun == (Vm_Fun *)0x0) {
      if (osPtr == (void *)0x0) {
        if (mayNull != 0) {
          vm_popNStack(n);
          vm_pushVoid();
          return 0;
        }
        vm_error("No such function.\n");
      }
                    /* Unresolved local var: Vm_OS_Fun_Ptr_T funPtr@[???]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg2(EDX)]
                       Unresolved local var: int funRet@[DW_OP_reg3(EBX)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
      piVar8 = argv;
      for (iVar7 = 0x10; iVar7 != 0; iVar7 = iVar7 + -1) {
        *piVar8 = 0;
        piVar8 = piVar8 + 1;
      }
      iVar7 = 0;
      bVar10 = SBORROW4(0,n);
      bVar9 = -n < 0;
LAB_080947c1:
      if (bVar10 == bVar9) {
        iVar7 = (*osPtr)(argv[0],argv[1],argv[2],argv[3],argv[4],argv[5],argv[6],argv[7],argv[8],
                         argv[9],argv[10],argv[0xb],argv[0xc],argv[0xd],argv[0xe],argv[0xf]);
        vm_popNStack(n);
        vm_pushNumber(iVar7);
        return 1;
      }
      iVar5 = vm_sp + iVar7;
      switch((uint)vm_stack[iVar5].type) {
      case 0:
      case 1:
        argv[iVar7] = 0;
        iVar7 = iVar7 + 1;
        bVar10 = SBORROW4(iVar7,n);
        bVar9 = iVar7 - n < 0;
        goto LAB_080947c1;
      case 2:
        *(anon_union_4_11_5000f750_for_v *)(argv + iVar7) = vm_stack[iVar5].v;
        break;
      case 3:
        *(anon_union_4_11_5000f750_for_v *)(argv + iVar7) = vm_stack[iVar5].v;
        iVar7 = iVar7 + 1;
        bVar10 = SBORROW4(iVar7,n);
        bVar9 = iVar7 - n < 0;
        goto LAB_080947c1;
      case 4:
        argv[iVar7] = (int)(vm_stack[iVar5].v.string)->ptr;
        break;
      case 5:
        argv[iVar7] = (vm_stack[iVar5].v.string)->len;
        break;
      default:
        goto switchD_080947e7_default;
      }
      iVar7 = iVar7 + 1;
      bVar10 = SBORROW4(iVar7,n);
      bVar9 = iVar7 - n < 0;
      goto LAB_080947c1;
    }
  }
  else if (pFun == (Vm_Fun *)0x0) {
    if ((pVVar6->token & 0x40000000) != 0) {
      VVar11 = *(Vm_Index_T *)((int)&(pVVar6->ii).v + 6);
LAB_0809483e:
      vm_callInEfun(VVar11,n);
      return 1;
    }
    if ((pVVar6->token & 0x8000000) == 0) {
      vm_error("The \"%s\" isn\'t a function.\n",pVVar6->name);
      return 1;
    }
    pFun = (pVVar6->ii).v.uf.pFun;
    VVar11 = (pVVar6->ii).v.uf.funOffset;
  }
  pToObject = (Vm_Object_T *)0x0;
LAB_0809469d:
  vm_callInUserFunBody(isGlobal,pToObject,VVar11,pFun,n);
  return 1;
switchD_080947e7_default:
  pcVar4 = vm_valueType((uint)vm_stack[iVar5].type);
  iVar7 = iVar7 + 1;
  vm_error("Bad type (%s) of the argument for os function.\n",pcVar4);
  bVar10 = SBORROW4(iVar7,n);
  bVar9 = iVar7 - n < 0;
  goto LAB_080947c1;
}


```

```text
0x809441c: PUSH EBP
0x809441d: MOV EBP, ESP
0x809441f: PUSH EDI
0x8094420: PUSH ESI
0x8094421: PUSH EBX
0x8094422: SUB ESP, 0x8c
0x8094428: MOV EAX, dword ptr [EBP + 0xc]
0x809442b: TEST EAX, EAX
0x809442d: JLE 0x0809476b
0x8094433: SUB ESP, 0xc
0x8094436: LEA EAX, [EBP + -0x38]
0x8094439: PUSH EAX
0x809443a: CALL 0x08089f10
0x809443f: MOV EAX, [0x086619cc]
0x8094444: LEA EAX, [EAX + EAX*0x2]
0x8094447: SHL EAX, 0x3
0x809444a: MOV ECX, dword ptr [0x08661964]
0x8094450: MOV DL, byte ptr [EAX + ECX*0x1 + 0x4]
0x8094454: MOV ESI, dword ptr [EAX + ECX*0x1 + 0x8]
0x8094458: MOV byte ptr [EBP + -0x34], DL
0x809445b: MOV DL, byte ptr [EAX + ECX*0x1 + 0x5]
0x809445f: DEC dword ptr [EBP + 0xc]
0x8094462: MOV byte ptr [EAX + ECX*0x1 + 0x4], 0x0
0x8094467: MOV EAX, [0x086619cc]
0x809446c: MOV byte ptr [EBP + -0x33], DL
0x809446f: LEA EAX, [EAX + EAX*0x2]
0x8094472: MOV EDX, dword ptr [0x08661964]
0x8094478: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x809447d: MOV EDX, dword ptr [0x086619cc]
0x8094483: MOVZX EAX, byte ptr [EBP + -0x34]
0x8094487: INC EDX
0x8094488: ADD ESP, 0x10
0x809448b: CMP EAX, 0x4
0x809448e: MOV dword ptr [EBP + -0x30], ESI
0x8094491: MOV dword ptr [0x086619cc], EDX
0x8094497: JZ 0x08094618
0x809449d: CMP EAX, 0x9
0x80944a0: JZ 0x080944cc
0x80944a2: SUB ESP, 0xc
0x80944a5: LEA EAX, [EBP + -0x38]
0x80944a8: PUSH EAX
0x80944a9: CALL 0x0808c0b8
0x80944ae: MOV dword ptr [ESP], 0x82ae484
0x80944b5: CALL 0x08097b24
0x80944ba: ADD ESP, 0x10
0x80944bd: MOV EAX, 0x1
0x80944c2: LEA ESP, [EBP + -0xc]
0x80944c5: POP EBX
0x80944c6: POP ESI
0x80944c7: POP EDI
0x80944c8: LEAVE
0x80944c9: RET
0x80944cc: MOVSX EAX, word ptr [ESI + 0x6]
0x80944d0: CMP EAX, EDX
0x80944d2: JG 0x0809474b
0x80944d8: MOV EAX, dword ptr [ESI + 0xc]
0x80944db: TEST EAX, EAX
0x80944dd: JNZ 0x080946b0
0x80944e3: MOVZX EDX, byte ptr [ESI + 0x5]
0x80944e7: MOV dword ptr [EBP + 0xffffff78], EDX
0x80944ed: MOVZX EAX, byte ptr [ESI + 0x4]
0x80944f1: MOV dword ptr [EBP + 0xffffff74], EAX
0x80944f7: MOV AX, word ptr [ESI + 0x8]
0x80944fb: MOV EDX, dword ptr [ESI + 0x14]
0x80944fe: MOV word ptr [EBP + -0x7a], AX
0x8094502: MOV EAX, dword ptr [EBP + 0xffffff78]
0x8094508: TEST EAX, EAX
0x809450a: MOV dword ptr [EBP + 0xffffff70], EDX
0x8094510: MOV dword ptr [EBP + 0xffffff7c], 0x0
0x809451a: JNZ 0x08094525
0x809451c: MOV EAX, dword ptr [ESI + 0xc]
0x809451f: MOV dword ptr [EBP + 0xffffff7c], EAX
0x8094525: MOVSX EDI, word ptr [ESI + 0x6]
0x8094529: ADD dword ptr [EBP + 0xc], EDI
0x809452c: CMP dword ptr [ESI], 0x1
0x809452f: JNZ 0x08094708
0x8094535: DEC EDI
0x8094536: CMP EDI, -0x1
0x8094539: JZ 0x080945b2
0x809453b: LEA EAX, [EDI + EDI*0x2]
0x809453e: LEA EBX, [EAX*0x8 + 0x0]
0x8094545: LEA ESI, [ESI]
0x8094548: MOV EAX, [0x086619cc]
0x809454d: MOV EDX, dword ptr [ESI + 0x10]
0x8094550: DEC EAX
0x8094551: MOV [0x086619cc], EAX
0x8094556: MOV CL, byte ptr [EBX + EDX*0x1 + 0x4]
0x809455a: LEA EAX, [EAX + EAX*0x2]
0x809455d: MOV EDX, dword ptr [0x08661964]
0x8094563: MOV byte ptr [EDX + EAX*0x8 + 0x4], CL
0x8094567: MOV EDX, dword ptr [ESI + 0x10]
0x809456a: MOV EAX, [0x086619cc]
0x809456f: MOV CL, byte ptr [EBX + EDX*0x1 + 0x5]
0x8094573: LEA EAX, [EAX + EAX*0x2]
0x8094576: MOV EDX, dword ptr [0x08661964]
0x809457c: MOV byte ptr [EDX + EAX*0x8 + 0x5], CL
0x8094580: MOV EDX, dword ptr [ESI + 0x10]
0x8094583: MOV EAX, [0x086619cc]
0x8094588: MOV ECX, dword ptr [EBX + EDX*0x1 + 0x8]
0x809458c: LEA EAX, [EAX + EAX*0x2]
0x809458f: MOV EDX, dword ptr [0x08661964]
0x8094595: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
0x8094599: MOV EAX, dword ptr [ESI + 0x10]
0x809459c: MOV byte ptr [EBX + EAX*0x1 + 0x4], 0x0
0x80945a1: MOV EAX, dword ptr [ESI + 0x10]
0x80945a4: DEC EDI
0x80945a5: MOV byte ptr [EBX + EAX*0x1 + 0x5], 0x0
0x80945aa: SUB EBX, 0x18
0x80945ad: CMP EDI, -0x1
0x80945b0: JNZ 0x08094548
0x80945b2: SUB ESP, 0xc
0x80945b5: PUSH ESI
0x80945b6: CALL 0x0808c114
0x80945bb: LEA EDX, [EBP + -0x38]
0x80945be: MOV dword ptr [ESP], EDX
0x80945c1: MOV byte ptr [EBP + -0x34], 0x0
0x80945c5: MOV byte ptr [EBP + -0x33], 0x0
0x80945c9: CALL 0x0808c0b8
0x80945ce: ADD ESP, 0x10
0x80945d1: CMP dword ptr [EBP + 0xffffff74], 0x4b
0x80945d8: JZ 0x0809482e
0x80945de: MOV EDX, dword ptr [EBP + 0xffffff7c]
0x80945e4: MOV EAX, [0x086619d8]
0x80945e9: CMP dword ptr [EAX + 0x20], EDX
0x80945ec: MOV EAX, 0x0
0x80945f1: CMOVNZ EAX, EDX
0x80945f4: SUB ESP, 0xc
0x80945f7: PUSH dword ptr [EBP + 0xc]
0x80945fa: MOV dword ptr [EBP + 0xffffff7c], EAX
0x8094600: PUSH dword ptr [EBP + 0xffffff70]
0x8094606: MOVSX EAX, word ptr [EBP + -0x7a]
0x809460a: PUSH EAX
0x809460b: PUSH dword ptr [EBP + 0xffffff7c]
0x8094611: JMP 0x0809469d
0x8094618: MOV EAX, dword ptr [ESI + 0xc]
0x809461b: XOR EBX, EBX
0x809461d: XOR EDI, EDI
0x809461f: CMP byte ptr [EAX], 0x24
0x8094622: JZ 0x08094873
0x8094628: MOV EAX, [0x086619d8]
0x809462d: MOV EDX, dword ptr [EAX + 0x20]
0x8094630: TEST EDX, EDX
0x8094632: MOV dword ptr [EBP + -0x80], 0x0
0x8094639: JNZ 0x080946da
0x809463f: MOV EBX, dword ptr [EBP + -0x30]
0x8094642: CMP word ptr [EBX + 0x8], 0x0
0x8094647: JZ 0x08094780
0x809464d: PUSH EAX
0x809464e: MOVZX EAX, word ptr [EBX + 0x8]
0x8094652: PUSH EAX
0x8094653: PUSH dword ptr [EBX + 0xc]
0x8094656: PUSH dword ptr [0x0847aad0]
0x809465c: CALL 0x08081e40
0x8094661: MOV EBX, EAX
0x8094663: MOV dword ptr [EBP + 0xffffff78], 0x1
0x809466d: ADD ESP, 0x10
0x8094670: SUB ESP, 0xc
0x8094673: LEA EDX, [EBP + -0x38]
0x8094676: PUSH EDX
0x8094677: CALL 0x0808c0b8
0x809467c: ADD ESP, 0x10
0x809467f: TEST EBX, EBX
0x8094681: JZ 0x0809479c
0x8094687: TEST EDI, EDI
0x8094689: JZ 0x0809484c
0x809468f: SUB ESP, 0xc
0x8094692: PUSH dword ptr [EBP + 0xc]
0x8094695: PUSH EDI
0x8094696: MOVSX EAX, word ptr [EBP + -0x7a]
0x809469a: PUSH EAX
0x809469b: PUSH 0x0
0x809469d: PUSH dword ptr [EBP + 0xffffff78]
0x80946a3: CALL 0x08093eb0
0x80946a8: ADD ESP, 0x20
0x80946ab: JMP 0x080944bd
0x80946b0: TEST byte ptr [EAX + 0xd], 0x80
0x80946b4: JZ 0x080944e3
0x80946ba: SUB ESP, 0xc
0x80946bd: LEA EAX, [EBP + -0x38]
0x80946c0: PUSH EAX
0x80946c1: CALL 0x0808c0b8
0x80946c6: MOV dword ptr [ESP], 0x82ae4c4
0x80946cd: CALL 0x08097b24
0x80946d2: ADD ESP, 0x10
0x80946d5: JMP 0x080944e3
0x80946da: LEA EAX, [EBP + -0x7a]
0x80946dd: PUSH EAX
0x80946de: PUSH 0x1
0x80946e0: PUSH ESI
0x80946e1: PUSH dword ptr [EDX + 0x20]
0x80946e4: CALL 0x08086b44
0x80946e9: ADD ESP, 0x10
0x80946ec: TEST EAX, EAX
0x80946ee: MOV EDI, EAX
0x80946f0: MOV dword ptr [EBP + 0xffffff78], 0x0
0x80946fa: JNZ 0x08094670
0x8094700: JMP 0x0809463f
0x8094708: SUB ESP, 0xc
0x809470b: LEA EAX, [EBP + -0x38]
0x809470e: PUSH EAX
0x809470f: DEC EDI
0x8094710: CALL 0x0808c0b8
0x8094715: ADD ESP, 0x10
0x8094718: CMP EDI, -0x1
0x809471b: JZ 0x080945d1
0x8094721: LEA EAX, [EDI + EDI*0x2]
0x8094724: LEA EBX, [EAX*0x8 + 0x0]
0x809472b: NOP
0x809472c: MOV EAX, dword ptr [ESI + 0x10]
0x809472f: SUB ESP, 0xc
0x8094732: ADD EAX, EBX
0x8094734: PUSH EAX
0x8094735: DEC EDI
0x8094736: CALL 0x08091074
0x809473b: SUB EBX, 0x18
0x809473e: ADD ESP, 0x10
0x8094741: CMP EDI, -0x1
0x8094744: JNZ 0x0809472c
0x8094746: JMP 0x080945d1
0x809474b: SUB ESP, 0xc
0x809474e: LEA EDX, [EBP + -0x38]
0x8094751: PUSH EDX
0x8094752: CALL 0x0808c0b8
0x8094757: MOV dword ptr [ESP], 0x82ae4f8
0x809475e: CALL 0x08097b24
0x8094763: ADD ESP, 0x10
0x8094766: JMP 0x080944d8
0x809476b: SUB ESP, 0xc
0x809476e: PUSH 0x82ae08b
0x8094773: CALL 0x08097b24
0x8094778: ADD ESP, 0x10
0x809477b: JMP 0x08094433
0x8094780: SUB ESP, 0x8
0x8094783: PUSH 0x20
0x8094785: PUSH dword ptr [EBX + 0xc]
0x8094788: CALL 0x08089dc8
0x809478d: MOV word ptr [EBX + 0x8], AX
0x8094791: ADD ESP, 0x10
0x8094794: MOV EBX, dword ptr [EBP + -0x30]
0x8094797: JMP 0x0809464d
0x809479c: TEST EDI, EDI
0x809479e: JNZ 0x0809468f
0x80947a4: MOV ESI, dword ptr [EBP + -0x80]
0x80947a7: TEST ESI, ESI
0x80947a9: JZ 0x08094945
0x80947af: XOR EAX, EAX
0x80947b1: LEA EDI, [EBP + -0x78]
0x80947b4: CLD
0x80947b5: MOV ECX, 0x10
0x80947ba: STOSD.REP ES:EDI
0x80947bc: XOR EBX, EBX
0x80947be: CMP EAX, dword ptr [EBP + 0xc]
0x80947c1: JGE 0x080948f5
0x80947c7: MOV EAX, [0x086619cc]
0x80947cc: ADD EAX, EBX
0x80947ce: LEA EAX, [EAX + EAX*0x2]
0x80947d1: MOV EDX, dword ptr [0x08661964]
0x80947d7: LEA EDX, [EDX + EAX*0x8]
0x80947da: MOVZX EAX, byte ptr [EDX + 0x4]
0x80947de: CMP EAX, 0x5
0x80947e1: JA 0x080948b0
0x80947e7: JMP dword ptr [EAX*0x4 + 0x82aeb70]
0x80947ee: MOV EAX, dword ptr [EDX + 0x8]
0x80947f1: MOV EAX, dword ptr [EAX + 0x10]
0x80947f4: MOV dword ptr [EBP + EBX*0x4 + -0x78], EAX
0x80947f8: INC EBX
0x80947f9: CMP EBX, dword ptr [EBP + 0xc]
0x80947fc: JMP 0x080947c1
0x80947fe: MOV EAX, dword ptr [EDX + 0x8]
0x8094801: MOV EAX, dword ptr [EAX + 0xc]
0x8094804: MOV dword ptr [EBP + EBX*0x4 + -0x78], EAX
0x8094808: JMP 0x080947f8
0x809480a: MOV EAX, dword ptr [EDX + 0x8]
0x809480d: MOV dword ptr [EBP + EBX*0x4 + -0x78], EAX
0x8094811: INC EBX
0x8094812: CMP EBX, dword ptr [EBP + 0xc]
0x8094815: JMP 0x080947c1
0x8094817: MOV EAX, dword ptr [EDX + 0x8]
0x809481a: MOV dword ptr [EBP + EBX*0x4 + -0x78], EAX
0x809481e: JMP 0x080947f8
0x8094820: MOV dword ptr [EBP + EBX*0x4 + -0x78], 0x0
0x8094828: INC EBX
0x8094829: CMP EBX, dword ptr [EBP + 0xc]
0x809482c: JMP 0x080947c1
0x809482e: SUB ESP, 0x8
0x8094831: PUSH dword ptr [EBP + 0xc]
0x8094834: MOV EDX, dword ptr [EBP + 0xffffff70]
0x809483a: MOVSX EAX, word ptr [EDX + 0x1a]
0x809483e: PUSH EAX
0x809483f: CALL 0x08093d98
0x8094844: ADD ESP, 0x10
0x8094847: JMP 0x080944bd
0x809484c: MOV EAX, dword ptr [EBX + 0x4]
0x809484f: TEST EAX, 0x40000000
0x8094854: JNZ 0x080948e6
0x809485a: TEST EAX, 0x8000000
0x809485f: JZ 0x080948d2
0x8094861: SUB ESP, 0xc
0x8094864: PUSH dword ptr [EBP + 0xc]
0x8094867: PUSH dword ptr [EBX + 0x14]
0x809486a: MOVSX EAX, word ptr [EBX + 0x18]
0x809486e: JMP 0x0809469a
0x8094873: SUB ESP, 0xc
0x8094876: INC EAX
0x8094877: PUSH EAX
0x8094878: CALL 0x080b0a24
0x809487d: ADD ESP, 0x10
0x8094880: TEST EAX, EAX
0x8094882: MOV dword ptr [EBP + -0x80], EAX
0x8094885: JZ 0x0809488e
0x8094887: XOR EDI, EDI
0x8094889: JMP 0x08094670
0x809488e: SUB ESP, 0xc
0x8094891: LEA EAX, [EBP + -0x38]
0x8094894: PUSH EAX
0x8094895: CALL 0x0808c0b8
0x809489a: XOR EDI, EDI
0x809489c: MOV dword ptr [ESP], 0x82ae0a5
0x80948a3: CALL 0x08097b24
0x80948a8: ADD ESP, 0x10
0x80948ab: JMP 0x08094670
0x80948b0: SUB ESP, 0xc
0x80948b3: PUSH EAX
0x80948b4: CALL 0x0808a13c
0x80948b9: POP EDX
0x80948ba: POP ECX
0x80948bb: PUSH EAX
0x80948bc: PUSH 0x82ae1bc
0x80948c1: INC EBX
0x80948c2: CALL 0x08097b24
0x80948c7: ADD ESP, 0x10
0x80948ca: CMP EBX, dword ptr [EBP + 0xc]
0x80948cd: JMP 0x080947c1
0x80948d2: SUB ESP, 0x8
0x80948d5: PUSH dword ptr [EBX]
0x80948d7: PUSH 0x82ae0bb
0x80948dc: CALL 0x08097b24
0x80948e1: JMP 0x080944ba
0x80948e6: SUB ESP, 0x8
0x80948e9: PUSH dword ptr [EBP + 0xc]
0x80948ec: MOVSX EAX, word ptr [EBX + 0x1a]
0x80948f0: JMP 0x0809483e
0x80948f5: PUSH dword ptr [EBP + -0x3c]
0x80948f8: PUSH dword ptr [EBP + -0x40]
0x80948fb: PUSH dword ptr [EBP + -0x44]
0x80948fe: PUSH dword ptr [EBP + -0x48]
0x8094901: PUSH dword ptr [EBP + -0x4c]
0x8094904: PUSH dword ptr [EBP + -0x50]
0x8094907: PUSH dword ptr [EBP + -0x54]
0x809490a: PUSH dword ptr [EBP + -0x58]
0x809490d: PUSH dword ptr [EBP + -0x5c]
0x8094910: PUSH dword ptr [EBP + -0x60]
0x8094913: PUSH dword ptr [EBP + -0x64]
0x8094916: PUSH dword ptr [EBP + -0x68]
0x8094919: PUSH dword ptr [EBP + -0x6c]
0x809491c: PUSH dword ptr [EBP + -0x70]
0x809491f: PUSH dword ptr [EBP + -0x74]
0x8094922: PUSH dword ptr [EBP + -0x78]
0x8094925: CALL dword ptr [EBP + -0x80]
0x8094928: ADD ESP, 0x34
0x809492b: PUSH dword ptr [EBP + 0xc]
0x809492e: MOV EBX, EAX
0x8094930: CALL 0x08091ab0
0x8094935: MOV dword ptr [ESP], EBX
0x8094938: CALL 0x08090b3c
0x809493d: ADD ESP, 0x10
0x8094940: JMP 0x080944bd
0x8094945: MOV EBX, dword ptr [EBP + 0x8]
0x8094948: TEST EBX, EBX
0x809494a: JZ 0x08094963
0x809494c: SUB ESP, 0xc
0x809494f: PUSH dword ptr [EBP + 0xc]
0x8094952: CALL 0x08091ab0
0x8094957: CALL 0x08090fc0
0x809495c: XOR EAX, EAX
0x809495e: JMP 0x080944c2
0x8094963: SUB ESP, 0xc
0x8094966: PUSH 0x82ae0d7
0x809496b: CALL 0x08097b24
0x8094970: ADD ESP, 0x10
0x8094973: JMP 0x080947af
```

Limitations:

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

## vm_callInEfun / analyze_function

Evidence: `ev_f7a9dddeb7edb2f09a574fb30d22a4cf7af8ca2f93f06fb113081ac12811e04f`

Address: `0x8093d98`; source: `docs/compatibility-evidence/main-rea.json.gz`

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

```text
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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- References cover the complete Ghidra function-body AddressSet and exact entry. Dossier incoming edges omit sources owned by that function; procedure_references also retains those internal edges.
- The dossier combines Ghidra FunctionManager, Listing, ReferenceManager, BasicBlockModel, and decompiler observations; provider-specific pseudocode and assembly are not cross-provider text invariants.
- Resolved reference metadata identifies computed, indirect, external, call, jump, and data edges; unresolved targetless flows remain unknown, and function classifications distinguish thunks and externals.
- Synthetic Ghidra entry-point references without actionable memory sources are omitted.
- The Java bridge serializes one function request per Program and lets each decompilation run until it completes or the caller cancels.
- Ghidra Listing reports a terminal call at 0x8093e43 to 0x8093c5c vm_returnFromEfun (external=false, thunk=false, hasNoReturn=false); fallthrough is excluded by Ghidra's flow model. The reason for the terminal flow is not established by this observation.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_callInUserFunBody / procedure_pseudo_code

Evidence: `ev_af340516b2b29ca98a9f7fdbd69b859634edd556cde9e4d06c0db4b0f2873aad`

Address: `0x8093eb0`; source: `docs/vm-core-evidence/rea.json.gz`

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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_cleanupValue / analyze_function

Evidence: `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc`

Address: `0x808ad98`; source: `docs/stage2af-evidence/rea.json.gz`

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

```text
0x808ad98: PUSH EBP
0x808ad99: MOV EBP, ESP
0x808ad9b: PUSH EDI
0x808ad9c: PUSH ESI
0x808ad9d: PUSH EBX
0x808ad9e: SUB ESP, 0xc
0x808ada1: MOV EDI, dword ptr [EBP + 0x8]
0x808ada4: MOVZX EAX, byte ptr [EDI + 0x4]
0x808ada8: CMP EAX, 0x14
0x808adab: JA 0x0808add2
0x808adad: JMP dword ptr [EAX*0x4 + 0x82ace1c]
0x808adb4: SUB ESP, 0xc
0x808adb7: PUSH dword ptr [EDI + 0x8]
0x808adba: CALL 0x080bc884
0x808adbf: ADD ESP, 0x10
0x808adc2: MOV byte ptr [EDI + 0x4], 0x0
0x808adc6: MOV byte ptr [EDI + 0x5], 0x0
0x808adca: LEA ESP, [EBP + -0xc]
0x808adcd: POP EBX
0x808adce: POP ESI
0x808adcf: POP EDI
0x808add0: LEAVE
0x808add1: RET
0x808add2: SUB ESP, 0x8
0x808add5: PUSH 0x1bb
0x808adda: PUSH 0x82acaf8
0x808addf: PUSH 0x82acc2c
0x808ade4: PUSH 0x82a825c
0x808ade9: PUSH 0x4
0x808adeb: PUSH 0x2
0x808aded: CALL 0x080b61c4
0x808adf2: ADD ESP, 0x14
0x808adf5: PUSH 0x0
0x808adf7: CALL 0x080a3980
0x808adfc: ADD ESP, 0x10
0x808adff: JMP 0x0808adff
0x808ae01: SUB ESP, 0xc
0x808ae04: PUSH dword ptr [EDI + 0x8]
0x808ae07: CALL 0x080b1618
0x808ae0c: JMP 0x0808adbf
0x808ae0e: MOV ESI, dword ptr [EDI + 0x8]
0x808ae11: MOV EAX, dword ptr [ESI]
0x808ae13: DEC EAX
0x808ae14: TEST EAX, EAX
0x808ae16: MOV dword ptr [ESI], EAX
0x808ae18: JG 0x0808adc2
0x808ae1a: CMP word ptr [ESI + 0x6], 0x0
0x808ae1f: MOV dword ptr [EBP + -0x10], 0x0
0x808ae26: JLE 0x0808ae7f
0x808ae28: MOV dword ptr [EBP + -0x14], 0x0
0x808ae2f: MOV EBX, dword ptr [EBP + -0x14]
0x808ae32: ADD EBX, dword ptr [ESI + 0x10]
0x808ae35: SUB ESP, 0xc
0x808ae38: DEC dword ptr [EBX]
0x808ae3a: PUSH EBX
0x808ae3b: CALL 0x0808ad98
0x808ae40: ADD ESP, 0x10
0x808ae43: DEC dword ptr [0x0847fd88]
0x808ae49: CMP EBX, dword ptr [0x0847fd8c]
0x808ae4f: JZ 0x0808afa9
0x808ae55: MOV EAX, dword ptr [EBX + 0xc]
0x808ae58: MOV EDX, dword ptr [EBX + 0x14]
0x808ae5b: MOV dword ptr [EDX + 0xc], EAX
0x808ae5e: MOV EAX, dword ptr [EBX + 0xc]
0x808ae61: TEST EAX, EAX
0x808ae63: JZ 0x0808ae68
0x808ae65: MOV dword ptr [EAX + 0x14], EDX
0x808ae68: INC dword ptr [EBP + -0x10]
0x808ae6b: MOVSX EAX, word ptr [ESI + 0x6]
0x808ae6f: ADD dword ptr [EBP + -0x14], 0x18
0x808ae73: CMP EAX, dword ptr [EBP + -0x10]
0x808ae76: MOV dword ptr [EBX + 0x10], 0x0
0x808ae7d: JG 0x0808ae2f
0x808ae7f: MOV EAX, dword ptr [ESI + 0xc]
0x808ae82: TEST EAX, EAX
0x808ae84: JZ 0x0808ae92
0x808ae86: SUB ESP, 0xc
0x808ae89: PUSH EAX
0x808ae8a: CALL 0x080b1618
0x808ae8f: ADD ESP, 0x10
0x808ae92: PUSH 0x117
0x808ae97: PUSH 0x82acaf8
0x808ae9c: PUSH 0x82a376b
0x808aea1: PUSH dword ptr [ESI + 0x10]
0x808aea4: CALL 0x080ceb94
0x808aea9: ADD ESP, 0x10
0x808aeac: PUSH 0x118
0x808aeb1: PUSH 0x82acaf8
0x808aeb6: PUSH 0x82a376b
0x808aebb: PUSH ESI
0x808aebc: JMP 0x0808af83
0x808aec1: MOV EBX, dword ptr [EDI + 0x8]
0x808aec4: MOV EAX, dword ptr [EBX]
0x808aec6: DEC EAX
0x808aec7: TEST EAX, EAX
0x808aec9: MOV dword ptr [EBX], EAX
0x808aecb: JNZ 0x0808adc2
0x808aed1: TEST EBX, EBX
0x808aed3: JZ 0x0808adc2
0x808aed9: SUB ESP, 0xc
0x808aedc: PUSH EBX
0x808aedd: CALL 0x0808ad98
0x808aee2: ADD ESP, 0x10
0x808aee5: DEC dword ptr [0x0847fd88]
0x808aeeb: CMP EBX, dword ptr [0x0847fd8c]
0x808aef1: JZ 0x0808af8d
0x808aef7: MOV EAX, dword ptr [EBX + 0xc]
0x808aefa: MOV EDX, dword ptr [EBX + 0x14]
0x808aefd: MOV dword ptr [EDX + 0xc], EAX
0x808af00: MOV EAX, dword ptr [EBX + 0xc]
0x808af03: TEST EAX, EAX
0x808af05: JZ 0x0808af0a
0x808af07: MOV dword ptr [EAX + 0x14], EDX
0x808af0a: SUB ESP, 0xc
0x808af0d: MOV dword ptr [EBX + 0x10], 0x0
0x808af14: PUSH EBX
0x808af15: CALL 0x080cf214
0x808af1a: JMP 0x0808adbf
0x808af1f: SUB ESP, 0xc
0x808af22: PUSH dword ptr [EDI + 0x8]
0x808af25: CALL 0x0809c488
0x808af2a: JMP 0x0808adbf
0x808af2f: SUB ESP, 0xc
0x808af32: PUSH dword ptr [EDI + 0x8]
0x808af35: CALL 0x0809b350
0x808af3a: JMP 0x0808adbf
0x808af3f: SUB ESP, 0xc
0x808af42: PUSH dword ptr [EDI + 0x8]
0x808af45: CALL 0x0809a580
0x808af4a: JMP 0x0808adbf
0x808af4f: MOV EDX, dword ptr [EDI + 0x8]
0x808af52: MOV EAX, dword ptr [EDX]
0x808af54: DEC EAX
0x808af55: TEST EAX, EAX
0x808af57: MOV dword ptr [EDX], EAX
0x808af59: JG 0x0808adc2
0x808af5f: TEST byte ptr [EDX + 0x4], 0x1
0x808af63: JZ 0x0808af73
0x808af65: SUB ESP, 0xc
0x808af68: PUSH EDX
0x808af69: CALL 0x080bd460
0x808af6e: JMP 0x0808adbf
0x808af73: PUSH 0x515
0x808af78: PUSH 0x82acaf8
0x808af7d: PUSH 0x82a376b
0x808af82: PUSH EDX
0x808af83: CALL 0x080ceb94
0x808af88: JMP 0x0808adbf
0x808af8d: MOV EAX, dword ptr [EBX + 0xc]
0x808af90: TEST EAX, EAX
0x808af92: MOV [0x0847fd8c], EAX
0x808af97: JZ 0x0808af0a
0x808af9d: MOV dword ptr [EAX + 0x14], 0x0
0x808afa4: JMP 0x0808af0a
0x808afa9: MOV EAX, dword ptr [EBX + 0xc]
0x808afac: TEST EAX, EAX
0x808afae: MOV [0x0847fd8c], EAX
0x808afb3: JZ 0x0808ae68
0x808afb9: MOV dword ptr [EAX + 0x14], 0x0
0x808afc0: JMP 0x0808ae68
```

Limitations:

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

## vm_copyValue / analyze_function

Evidence: `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4`

Address: `0x808be24`; source: `docs/stage2af-evidence/rea.json.gz`

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

```text
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

Limitations:

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

## vm_createFunctionValue / analyze_function

Evidence: `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444`

Address: `0x8091f34`; source: `docs/compatibility-evidence/dispatch-rea.json.gz`

```c

void vm_createFunctionValue(Vm_Index_T funOffset,int isGlobalTypeArgn)

{
  uint isGlobal;
  int type;
  ushort argn;
  
                    /* Unresolved local var: int isGlobal@[DW_OP_reg1(ECX)]
                       Unresolved local var: int type@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Index_T argn@[DW_OP_reg7(EDI)] */
  type = (-(uint)(((uint)isGlobalTypeArgn >> 6 & 1) == 0) & 0x18) + 0x4b;
  argn = (ushort)isGlobalTypeArgn & 0x3f;
  isGlobal = (uint)isGlobalTypeArgn >> 7 & 1;
  if (isGlobal == 0) {
    vm_createFunctionValueEx
              (0,type,funOffset + vm_currentContext->consititution->baseFunOffset,argn,
               vm_currentContext->thisObject);
    return;
  }
  vm_createFunctionValueEx(isGlobal,type,funOffset,argn,vm_currentContext->thisObject);
  return;
}


```

```text
0x8091f34: PUSH EBP
0x8091f35: MOV EBP, ESP
0x8091f37: PUSH EDI
0x8091f38: PUSH ESI
0x8091f39: PUSH EBX
0x8091f3a: SUB ESP, 0xc
0x8091f3d: MOV EAX, dword ptr [EBP + 0xc]
0x8091f40: MOV EDX, EAX
0x8091f42: SHR EDX, 0x6
0x8091f45: MOV ECX, EAX
0x8091f47: AND EDX, 0x1
0x8091f4a: SHR ECX, 0x7
0x8091f4d: CMP EDX, 0x1
0x8091f50: SBB EBX, EBX
0x8091f52: AND EBX, 0x18
0x8091f55: MOV EDI, EAX
0x8091f57: ADD EBX, 0x4b
0x8091f5a: AND EDI, 0x3f
0x8091f5d: AND ECX, 0x1
0x8091f60: MOV ESI, dword ptr [EBP + 0x8]
0x8091f63: JNZ 0x08091f90
0x8091f65: MOV EDX, dword ptr [0x086619d8]
0x8091f6b: MOV EAX, dword ptr [EDX + 0x14]
0x8091f6e: SUB ESP, 0xc
0x8091f71: ADD SI, word ptr [EAX + 0x6]
0x8091f75: PUSH dword ptr [EDX + 0x20]
0x8091f78: MOVSX EAX, DI
0x8091f7b: PUSH EAX
0x8091f7c: MOVSX EAX, SI
0x8091f7f: PUSH EAX
0x8091f80: PUSH EBX
0x8091f81: PUSH ECX
0x8091f82: CALL 0x08091cd0
0x8091f87: LEA ESP, [EBP + -0xc]
0x8091f8a: POP EBX
0x8091f8b: POP ESI
0x8091f8c: POP EDI
0x8091f8d: LEAVE
0x8091f8e: RET
0x8091f90: SUB ESP, 0xc
0x8091f93: MOV EDX, dword ptr [0x086619d8]
0x8091f99: PUSH dword ptr [EDX + 0x20]
0x8091f9c: MOVSX EAX, DI
0x8091f9f: PUSH EAX
0x8091fa0: MOVSX EAX, SI
0x8091fa3: PUSH EAX
0x8091fa4: PUSH EBX
0x8091fa5: PUSH ECX
0x8091fa6: CALL 0x08091cd0
0x8091fab: LEA ESP, [EBP + -0xc]
0x8091fae: POP EBX
0x8091faf: POP ESI
0x8091fb0: POP EDI
0x8091fb1: LEAVE
0x8091fb2: RET
```

Limitations:

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

## vm_createFunctionValueEx / analyze_function

Evidence: `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48`

Address: `0x8091cd0`; source: `docs/stage2af-evidence/rea.json.gz`

```c

void vm_createFunctionValueEx
               (int isGlobal,int type,Vm_Index_T funOffset,Vm_Index_T argn,Vm_Object_T *pObject)

{
  Vm_Value_T *pVVar1;
  int iVar2;
  Vm_Fun_Ptr *ptr;
  Vm_Value *pVVar3;
  Vm_Program_T *pVVar4;
  int iVar5;
  Vm_Index_T VVar6;
  short sVar7;
  Vm_Fun_Ptr_T *function;
  Vm_Index_T argn_local;
  Vm_Index_T funOffset_local;
  
                    /* Unresolved local var: Vm_Value_T * pArgs@[DW_OP_reg7(EDI)]
                       Unresolved local var: Vm_Index_T i@[DW_OP_reg6(ESI)] */
  if (0 < argn) {
    iVar5 = 0;
    VVar6 = argn;
    do {
                    /* Unresolved local var: Vm_Value_T * pValue@[???] */
      while (vm_stack[iVar5 + vm_sp].type != '\b') {
        iVar5 = iVar5 + 1;
        VVar6 = VVar6 + -1;
        if (VVar6 == 0) goto LAB_08091d2c;
      }
      vm_error("Bad argument %d for function value, can\'t accept refer type.\n",iVar5 + 1);
      iVar5 = iVar5 + 1;
      VVar6 = VVar6 + -1;
    } while (VVar6 != 0);
  }
LAB_08091d2c:
  ptr = pdb_allocateMemory(0x18,"VM","/home/cc/xxxx/gameshell/src/vm/vm_stack.c",0x358);
  if (ptr == (Vm_Fun_Ptr *)0x0) {
    vm_error("Not enough memory to build function variable.\n");
  }
  pVVar3 = pdb_allocateMemory(argn * 0x18,"VM","/home/cc/xxxx/gameshell/src/vm/vm_stack.c",0x35d);
  if (pVVar3 == (Vm_Value *)0x0) {
    pdb_freeMemory(ptr,"VM","/home/cc/xxxx/gameshell/src/vm/vm_stack.c",0x360);
    vm_error("Not enough memory for arguments of function variable.\n");
  }
  ptr->type = (uchar)type;
  ptr->isGlobal = (uchar)isGlobal;
  ptr->funOffset = funOffset;
  ptr->argn = argn;
  ptr->pArgs = pVVar3;
  ptr->ref = 1;
  if (pObject != (Vm_Object_T *)0x0) {
    if ((pObject->flags & 0x8000) == 0) {
      pObject->ref = pObject->ref + 1;
    }
    else {
      pObject = (Vm_Object_T *)0x0;
    }
  }
  ptr->pObject = pObject;
  pVVar4 = vm_pGlobalProgram;
  if (isGlobal == 0) {
    pVVar4 = pObject->pProgram;
  }
  ptr->f = *(anon_union_4_3_7473fe5b_for_f *)(pVVar4->pFunctions + funOffset);
  sVar7 = 0;
  if (0 < argn) {
    do {
      vm_initValue(pVVar3 + sVar7);
      pVVar3[sVar7].type = vm_stack[vm_sp].type;
      pVVar3[sVar7].attrib = vm_stack[vm_sp].attrib;
      iVar5 = vm_sp;
      pVVar1 = vm_stack;
      vm_stack[vm_sp].type = '\0';
      iVar2 = vm_sp;
      pVVar3[sVar7].v = pVVar1[iVar5].v;
      vm_stack[iVar2].attrib = '\0';
      sVar7 = sVar7 + 1;
      vm_sp = vm_sp + 1;
    } while (sVar7 < argn);
  }
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\t';
  vm_stack[vm_sp].v.function = ptr;
  return;
}


```

```text
0x8091cd0: PUSH EBP
0x8091cd1: MOV EBP, ESP
0x8091cd3: PUSH EDI
0x8091cd4: PUSH ESI
0x8091cd5: PUSH EBX
0x8091cd6: SUB ESP, 0xc
0x8091cd9: MOV EDX, dword ptr [EBP + 0x14]
0x8091cdc: MOV EAX, dword ptr [EBP + 0x10]
0x8091cdf: TEST DX, DX
0x8091ce2: MOV word ptr [EBP + -0xe], AX
0x8091ce6: MOV word ptr [EBP + -0x10], DX
0x8091cea: JLE 0x08091d2c
0x8091cec: XOR EBX, EBX
0x8091cee: MOV ESI, EDX
0x8091cf0: JMP 0x08091cf9
0x8091cf4: INC EBX
0x8091cf5: DEC SI
0x8091cf7: JZ 0x08091d2c
0x8091cf9: MOV EAX, EBX
0x8091cfb: ADD EAX, dword ptr [0x086619cc]
0x8091d01: LEA EAX, [EAX + EAX*0x2]
0x8091d04: MOV EDX, dword ptr [0x08661964]
0x8091d0a: CMP byte ptr [EDX + EAX*0x8 + 0x4], 0x8
0x8091d0f: JNZ 0x08091cf4
0x8091d11: SUB ESP, 0x8
0x8091d14: LEA EAX, [EBX + 0x1]
0x8091d17: PUSH EAX
0x8091d18: PUSH 0x82ade60
0x8091d1d: CALL 0x08097b24
0x8091d22: INC EBX
0x8091d23: ADD ESP, 0x10
0x8091d26: DEC SI
0x8091d28: JNZ 0x08091cf9
0x8091d2a: MOV ESI, ESI
0x8091d2c: PUSH 0x358
0x8091d31: PUSH 0x82adea0
0x8091d36: PUSH 0x82a376b
0x8091d3b: PUSH 0x18
0x8091d3d: CALL 0x080ce784
0x8091d42: ADD ESP, 0x10
0x8091d45: TEST EAX, EAX
0x8091d47: MOV dword ptr [EBP + -0x14], EAX
0x8091d4a: JZ 0x08091f1e
0x8091d50: PUSH 0x35d
0x8091d55: MOVSX EAX, word ptr [EBP + -0x10]
0x8091d59: PUSH 0x82adea0
0x8091d5e: LEA EAX, [EAX + EAX*0x2]
0x8091d61: PUSH 0x82a376b
0x8091d66: SHL EAX, 0x3
0x8091d69: PUSH EAX
0x8091d6a: CALL 0x080ce784
0x8091d6f: ADD ESP, 0x10
0x8091d72: TEST EAX, EAX
0x8091d74: MOV EDI, EAX
0x8091d76: JZ 0x08091ef3
0x8091d7c: MOV ECX, dword ptr [EBP + -0x14]
0x8091d7f: MOV EAX, dword ptr [EBP + 0xc]
0x8091d82: MOV byte ptr [ECX + 0x4], AL
0x8091d85: MOV AL, byte ptr [EBP + 0x8]
0x8091d88: MOV byte ptr [ECX + 0x5], AL
0x8091d8b: MOV AX, word ptr [EBP + -0xe]
0x8091d8f: MOV word ptr [ECX + 0x8], AX
0x8091d93: MOV EAX, dword ptr [EBP + 0x18]
0x8091d96: MOV EDX, dword ptr [EBP + -0x10]
0x8091d99: TEST EAX, EAX
0x8091d9b: MOV word ptr [ECX + 0x6], DX
0x8091d9f: MOV dword ptr [ECX + 0x10], EDI
0x8091da2: MOV dword ptr [ECX], 0x1
0x8091da8: JZ 0x08091dbe
0x8091daa: MOV EDX, dword ptr [EBP + 0x18]
0x8091dad: TEST byte ptr [EDX + 0xd], 0x80
0x8091db1: JZ 0x08091ecf
0x8091db7: MOV dword ptr [EBP + 0x18], 0x0
0x8091dbe: MOV EAX, dword ptr [EBP + -0x14]
0x8091dc1: MOV EDX, dword ptr [EBP + 0x18]
0x8091dc4: MOV dword ptr [EAX + 0xc], EDX
0x8091dc7: MOV EAX, dword ptr [EBP + 0x8]
0x8091dca: TEST EAX, EAX
0x8091dcc: JZ 0x08091ec0
0x8091dd2: MOVSX EDX, word ptr [EBP + -0xe]
0x8091dd6: MOV EAX, [0x0847aad0]
0x8091ddb: MOV EAX, dword ptr [EAX + 0x2c]
0x8091dde: MOV EAX, dword ptr [EAX + EDX*0x8]
0x8091de1: MOV ECX, dword ptr [EBP + -0x14]
0x8091de4: MOV dword ptr [ECX + 0x14], EAX
0x8091de7: XOR ESI, ESI
0x8091de9: CMP SI, word ptr [EBP + -0x10]
0x8091ded: JL 0x08091e30
0x8091def: MOV EAX, [0x086619cc]
0x8091df4: TEST EAX, EAX
0x8091df6: JZ 0x08091ed9
0x8091dfc: DEC EAX
0x8091dfd: MOV [0x086619cc], EAX
0x8091e02: MOV EDX, dword ptr [0x08661964]
0x8091e08: LEA EAX, [EAX + EAX*0x2]
0x8091e0b: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x9
0x8091e10: MOV EAX, [0x086619cc]
0x8091e15: MOV EDX, dword ptr [0x08661964]
0x8091e1b: MOV ECX, dword ptr [EBP + -0x14]
0x8091e1e: LEA EAX, [EAX + EAX*0x2]
0x8091e21: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
0x8091e25: LEA ESP, [EBP + -0xc]
0x8091e28: POP EBX
0x8091e29: POP ESI
0x8091e2a: POP EDI
0x8091e2b: LEAVE
0x8091e2c: RET
0x8091e30: MOVSX EBX, SI
0x8091e33: LEA EBX, [EBX + EBX*0x2]
0x8091e36: SHL EBX, 0x3
0x8091e39: SUB ESP, 0xc
0x8091e3c: LEA EAX, [EBX + EDI*0x1]
0x8091e3f: PUSH EAX
0x8091e40: CALL 0x08089f10
0x8091e45: MOV EAX, [0x086619cc]
0x8091e4a: MOV EDX, dword ptr [0x08661964]
0x8091e50: LEA EAX, [EAX + EAX*0x2]
0x8091e53: MOV AL, byte ptr [EDX + EAX*0x8 + 0x4]
0x8091e57: MOV byte ptr [EBX + EDI*0x1 + 0x4], AL
0x8091e5b: MOV EAX, [0x086619cc]
0x8091e60: MOV EDX, dword ptr [0x08661964]
0x8091e66: LEA EAX, [EAX + EAX*0x2]
0x8091e69: MOV AL, byte ptr [EDX + EAX*0x8 + 0x5]
0x8091e6d: MOV byte ptr [EBX + EDI*0x1 + 0x5], AL
0x8091e71: MOV EAX, [0x086619cc]
0x8091e76: LEA EAX, [EAX + EAX*0x2]
0x8091e79: MOV ECX, dword ptr [0x08661964]
0x8091e7f: SHL EAX, 0x3
0x8091e82: MOV byte ptr [EAX + ECX*0x1 + 0x4], 0x0
0x8091e87: MOV EDX, dword ptr [EAX + ECX*0x1 + 0x8]
0x8091e8b: MOV EAX, [0x086619cc]
0x8091e90: MOV dword ptr [EBX + EDI*0x1 + 0x8], EDX
0x8091e94: LEA EAX, [EAX + EAX*0x2]
0x8091e97: MOV EDX, dword ptr [0x08661964]
0x8091e9d: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
0x8091ea2: MOV EAX, [0x086619cc]
0x8091ea7: INC ESI
0x8091ea8: INC EAX
0x8091ea9: ADD ESP, 0x10
0x8091eac: CMP SI, word ptr [EBP + -0x10]
0x8091eb0: MOV [0x086619cc], EAX
0x8091eb5: JL 0x08091e30
0x8091ebb: JMP 0x08091df4
0x8091ec0: MOV EDX, dword ptr [EBP + 0x18]
0x8091ec3: MOV EAX, dword ptr [EDX + 0x20]
0x8091ec6: MOVSX EDX, word ptr [EBP + -0xe]
0x8091eca: JMP 0x08091ddb
0x8091ecf: MOV ECX, dword ptr [EBP + 0x18]
0x8091ed2: INC dword ptr [ECX]
0x8091ed4: JMP 0x08091dbe
0x8091ed9: SUB ESP, 0xc
0x8091edc: PUSH 0x82ad98c
0x8091ee1: CALL 0x08097b24
0x8091ee6: ADD ESP, 0x10
0x8091ee9: MOV EAX, [0x086619cc]
0x8091eee: JMP 0x08091dfc
0x8091ef3: PUSH 0x360
0x8091ef8: PUSH 0x82adea0
0x8091efd: PUSH 0x82a376b
0x8091f02: PUSH dword ptr [EBP + -0x14]
0x8091f05: CALL 0x080ceb94
0x8091f0a: MOV dword ptr [ESP], 0x82adecc
0x8091f11: CALL 0x08097b24
0x8091f16: ADD ESP, 0x10
0x8091f19: JMP 0x08091d7c
0x8091f1e: SUB ESP, 0xc
0x8091f21: PUSH 0x82adf04
0x8091f26: CALL 0x08097b24
0x8091f2b: ADD ESP, 0x10
0x8091f2e: JMP 0x08091d50
```

Limitations:

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

## vm_doActions / procedure_pseudo_code

Evidence: `ev_1f8d2b76898bf43dd8b24a370fca52729a822af343d566f8bc3216ef0b05c88f`

Address: `0x8097008`; source: `docs/vm-catch-evidence/reused.json.gz`

```c

int vm_doActions(Vm_PCB *pPCB,Vm_Tick_T n)

{
  Vm_Catch_Info_T *pVVar1;
  Vm_Tick_T VVar2;
  int iVar3;
  Vm_Catch_Buf *pVVar4;
  int iVar5;
  int actionState;
  Vm_Catch_Info_T *pCatch;
  
                    /* Unresolved local var: int funCp@[DW_OP_reg2(EDX)]
                       Unresolved local var: int errCode@[DW_OP_reg0(EAX)] */
  totalTicks = 10000;
  if (n != 0) {
    totalTicks = n;
  }
  VVar2 = totalTicks;
  if ((pPCB->flags & 0x200000) != 0) {
    iVar5 = vm_getCostTimeCheck();
    VVar2 = 120000;
    if (iVar5 != 0) {
      VVar2 = totalTicks;
    }
  }
  totalTicks = VVar2;
  _vm_startTimer(0,totalTicks);
LAB_08097040:
  iVar3 = pPCB->catchCp;
  iVar5 = 0;
  if (iVar3 < 1) goto LAB_08097066;
  do {
    iVar5 = pPCB->catchContext[iVar3 + -1].funCp;
LAB_08097066:
    pVVar4 = vm_newCatch(iVar5,1);
    iVar5 = _setjmp((__jmp_buf_tag *)pVVar4);
    while( true ) {
      if (iVar5 == 0) {
        if ((pPCB->flags & 0x40) != 0) {
          pPCB->flags = pPCB->flags & 0xffffffbf;
          vm_returnFromEfun();
        }
        if ((vm_confEnableJit == 0) && (600000 < pPCB->holdTicks)) {
          iVar5 = vm_getCostTimeCheck();
          if (iVar5 == 0) {
            pPCB->holdTicks = 0;
          }
          else {
            vm_error("Non-delay routine (primitive) cost too much time.\n");
          }
        }
        actionState = -0x1fab;
        if (vm_ctrlFlags == 0) goto joined_r0x0809717a;
        if ((vm_ctrlFlags & 0x10) != 0) {
          vm_ctrlFlags = vm_ctrlFlags & 0xffffffef;
          actionState = 0;
          if (vm_ctrlFlags == 0) goto joined_r0x0809717a;
        }
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
        goto joined_r0x0809717a;
      }
      if (iVar5 != -3) {
        return iVar5;
      }
      if (pPCB->catchCp < 1) {
        return -3;
      }
      iVar5 = pPCB->catchCp + -1;
      pPCB->catchCp = iVar5;
      pVVar1 = pPCB->catchContext + iVar5;
      if (vm_sp < pVVar1->sp) {
        vm_popNStack(pVVar1->sp - vm_sp);
        iVar5 = pVVar1->catchIp;
      }
      else {
        iVar5 = pVVar1->catchIp;
      }
      if (iVar5 == -1) {
        iVar5 = 0;
        if (0 < pPCB->catchCp) {
          iVar5 = pPCB->catchContext[pPCB->catchCp + -1].funCp;
        }
        pVVar4 = vm_newCatch(iVar5,1);
        iVar5 = _setjmp((__jmp_buf_tag *)pVVar4);
        if (iVar5 != 0) {
          return 0;
        }
        (*pVVar1->catchCallback)(pVVar1->pCookie);
        vm_endCatch();
        goto LAB_08097040;
      }
      vm_currentContext->ip = iVar5;
      iVar3 = pPCB->catchCp;
      if (0 < iVar3) break;
      pVVar4 = vm_newCatch(0,1);
      iVar5 = _setjmp((__jmp_buf_tag *)pVVar4);
    }
  } while( true );
joined_r0x0809717a:
  if (actionState != -0x1fab) {
LAB_080971b0:
    if (((((pPCB->flags & 0x200000) != 0) && (-1 < (int)pPCB->flags)) && (actionState != -0x1fa5))
       && (actionState != -0x1fa6)) {
      vm_error("Non-delay routine (callback) cost too much time.\n");
    }
    vm_endCatch();
    return actionState;
  }
  if ((vm_currentContext->ip < 0) || (vm_currentContext->cs->nodesCount <= vm_currentContext->ip)) {
    actionState = -0x1fa6;
    goto LAB_080971b0;
  }
  actionState = (*vm_processor)(pPCB);
  goto joined_r0x0809717a;
}


```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_eqValue / analyze_function

Evidence: `ev_9c31d92aeb045a089ff6361f0cf81330894208eb0520a78f0bf4bd4701c8d565`

Address: `0x808eed4`; source: `docs/stage2af-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_eqValue(void)

{
  Vm_Value_T *pValue;
  int iVar1;
  
                    /* Unresolved local var: Vm_Value_T * pValue1@[???]
                       Unresolved local var: Vm_Value_T * pValue2@[???]
                       Unresolved local var: int ret@[DW_OP_reg6(ESI)] */
  iVar1 = vm_isEq(vm_stack + vm_sp + 1,vm_stack + vm_sp);
  vm_popStack();
                    /* Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg3(EBX)]
                       Unresolved local var: int number@[???] */
  pValue = vm_stack + vm_sp;
  vm_cleanupValue(pValue);
  pValue->type = '\x02';
  (pValue->v).number = iVar1;
  return;
}


```

```text
0x808eed4: PUSH EBP
0x808eed5: MOV EBP, ESP
0x808eed7: MOV EAX, [0x086619cc]
0x808eedc: PUSH ESI
0x808eedd: PUSH EBX
0x808eede: MOV EDX, dword ptr [0x08661964]
0x808eee4: LEA EAX, [EAX + EAX*0x2]
0x808eee7: SUB ESP, 0x8
0x808eeea: LEA EAX, [EDX + EAX*0x8]
0x808eeed: PUSH EAX
0x808eeee: LEA EDX, [EAX + 0x18]
0x808eef1: PUSH EDX
0x808eef2: CALL 0x0808c908
0x808eef7: MOV ESI, EAX
0x808eef9: CALL 0x0809061c
0x808eefe: MOV EBX, dword ptr [0x086619cc]
0x808ef04: MOV EAX, [0x08661964]
0x808ef09: LEA EBX, [EBX + EBX*0x2]
0x808ef0c: LEA EBX, [EAX + EBX*0x8]
0x808ef0f: MOV dword ptr [ESP], EBX
0x808ef12: CALL 0x0808ad98
0x808ef17: MOV byte ptr [EBX + 0x4], 0x2
0x808ef1b: MOV dword ptr [EBX + 0x8], ESI
0x808ef1e: ADD ESP, 0x10
0x808ef21: LEA ESP, [EBP + -0x8]
0x808ef24: POP EBX
0x808ef25: POP ESI
0x808ef26: LEAVE
0x808ef27: RET
```

Limitations:

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

## vm_errorQuit / procedure_pseudo_code

Evidence: `ev_d9b9b1bd2399a379282bc11cbe8575420d3f316770e17d5a6ea5138fdee1b7a1`

Address: `0x80ab104`; source: `docs/vm-catch-evidence/reused.json.gz`

```c

/* WARNING: Unknown calling convention */

void vm_errorQuit(int n)

{
  int iVar1;
  Vm_Thread_Info_T *pVVar2;
  Vm_Task_Id_T VVar3;
  
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg3(EBX)] */
  pVVar2 = vm_getCurrentThreadInfo();
  VVar3 = _vm_getCurrentTaskId();
  if (VVar3 == _vm_mainTaskId) {
    vm_updateVmIpByLastJitFrame();
    iVar1 = pVVar2->threadType;
  }
  else {
    iVar1 = pVVar2->threadType;
  }
  if (iVar1 == 1) {
    vm_traceError(pVVar2->topEnv[pVVar2->topCatch + -1].funCp,(FILE *)0x0);
  }
                    /* Unresolved local var: Vm_Thread_Info_T * pThreadInfo@[DW_OP_reg0(EAX)] */
  pVVar2 = vm_getCurrentThreadInfo();
  pVVar2->topCatch = pVVar2->topCatch + -1;
                    /* WARNING: Subroutine does not return */
  longjmp((__jmp_buf_tag *)(pVVar2->topEnv + pVVar2->topCatch),n);
}


```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- Ghidra Listing reports a terminal call at 0x80ab144 to 0x804b574 <EXTERNAL>::longjmp (external=false, thunk=true, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_f_bind_parameters / analyze_function

Evidence: `ev_2367fe505024260c84916a2784345d572dd09ee9effe6fad39c28a1eb75cb9e0`

Address: `0x80df374`; source: `docs/stage2af-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_bind_parameters(void)

{
  Vm_Array_T *pArray;
  Vm_Value_T *pValue;
  char *statement;
  int n;
  
                    /* Unresolved local var: Vm_Array_T * pArray@[DW_OP_reg6(ESI)]
                       Unresolved local var: int i@[DW_OP_reg3(EBX)] */
  pArray = vm_allocateEmptyArray(vm_currentContext->argNum - 1);
  vm_pushArray(pArray);
  n = 1;
  if (1 < vm_currentContext->argNum) {
    do {
      pValue = vm_getArgument(n,0xff);
      vm_copyValue(pArray->pValues[n + -1],pValue);
      n = n + 1;
    } while (n < vm_currentContext->argNum);
  }
  statement = vm_getStringArgument(0);
  vm_bindParameters(statement,pArray);
  vm_takeOffValueFromStack(1,vm_currentContext->argNum + 1);
  return;
}


```

```text
0x80df374: PUSH EBP
0x80df375: MOV EBP, ESP
0x80df377: PUSH ESI
0x80df378: PUSH EBX
0x80df379: MOV EAX, [0x086619d8]
0x80df37e: MOV EAX, dword ptr [EAX + 0x4]
0x80df381: SUB ESP, 0xc
0x80df384: DEC EAX
0x80df385: PUSH EAX
0x80df386: CALL 0x0809b1d8
0x80df38b: MOV dword ptr [ESP], EAX
0x80df38e: MOV ESI, EAX
0x80df390: CALL 0x08090d84
0x80df395: MOV EAX, [0x086619d8]
0x80df39a: ADD ESP, 0x10
0x80df39d: CMP dword ptr [EAX + 0x4], 0x1
0x80df3a1: MOV EBX, 0x1
0x80df3a6: JLE 0x080df3d3
0x80df3a8: SUB ESP, 0x8
0x80df3ab: PUSH 0xff
0x80df3b0: PUSH EBX
0x80df3b1: CALL 0x0808a364
0x80df3b6: POP EDX
0x80df3b7: POP ECX
0x80df3b8: PUSH EAX
0x80df3b9: MOV EAX, dword ptr [ESI + 0xc]
0x80df3bc: PUSH dword ptr [EAX + EBX*0x4 + -0x4]
0x80df3c0: CALL 0x0808be24
0x80df3c5: INC EBX
0x80df3c6: MOV EAX, [0x086619d8]
0x80df3cb: ADD ESP, 0x10
0x80df3ce: CMP dword ptr [EAX + 0x4], EBX
0x80df3d1: JG 0x080df3a8
0x80df3d3: SUB ESP, 0xc
0x80df3d6: PUSH 0x0
0x80df3d8: CALL 0x0808a8ec
0x80df3dd: MOV EDX, ESI
0x80df3df: ADD ESP, 0x10
0x80df3e2: CALL 0x080defa4
0x80df3e7: MOV EAX, [0x086619d8]
0x80df3ec: MOV EAX, dword ptr [EAX + 0x4]
0x80df3ef: SUB ESP, 0x8
0x80df3f2: INC EAX
0x80df3f3: PUSH EAX
0x80df3f4: PUSH 0x1
0x80df3f6: CALL 0x08092008
0x80df3fb: LEA ESP, [EBP + -0x8]
0x80df3fe: POP EBX
0x80df3ff: POP ESI
0x80df400: LEAVE
0x80df401: RET
```

Limitations:

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

## vm_f_bind_parameters_arr / analyze_function

Evidence: `ev_6e22aee1d5df3133900f522d36c3f2841a4834926e5c9e99b05ade923bd15982`

Address: `0x80df310`; source: `docs/stage2af-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void vm_f_bind_parameters_arr(void)

{
  Vm_String_T *pArray;
  Vm_Value_T *pVVar1;
  char *statement;
  
  if (2 < vm_currentContext->argNum) {
    vm_error("Too many arguments(%d) to %s.\n",vm_currentContext->argNum,"mysql_bind_parameters_arr"
            );
  }
  pVVar1 = vm_getArgument(1,6);
  pArray = (pVVar1->v).string;
  statement = vm_getStringArgument(0);
  vm_bindParameters(statement,(Vm_Array_T *)pArray);
  vm_takeOffValueFromStack(1,vm_currentContext->argNum);
  return;
}


```

```text
0x80df310: PUSH EBP
0x80df311: MOV EBP, ESP
0x80df313: PUSH EBX
0x80df314: PUSH EAX
0x80df315: MOV EAX, [0x086619d8]
0x80df31a: CMP dword ptr [EAX + 0x4], 0x2
0x80df31e: JG 0x080df35c
0x80df320: SUB ESP, 0x8
0x80df323: PUSH 0x6
0x80df325: PUSH 0x1
0x80df327: CALL 0x0808a364
0x80df32c: MOV EBX, dword ptr [EAX + 0x8]
0x80df32f: MOV dword ptr [ESP], 0x0
0x80df336: CALL 0x0808a8ec
0x80df33b: MOV EDX, EBX
0x80df33d: ADD ESP, 0x10
0x80df340: CALL 0x080defa4
0x80df345: SUB ESP, 0x8
0x80df348: MOV EAX, [0x086619d8]
0x80df34d: PUSH dword ptr [EAX + 0x4]
0x80df350: PUSH 0x1
0x80df352: CALL 0x08092008
0x80df357: MOV EBX, dword ptr [EBP + -0x4]
0x80df35a: LEAVE
0x80df35b: RET
0x80df35c: PUSH EBX
0x80df35d: PUSH 0x82b77ff
0x80df362: PUSH dword ptr [EAX + 0x4]
0x80df365: PUSH 0x82a82c4
0x80df36a: CALL 0x08097b24
0x80df36f: ADD ESP, 0x10
0x80df372: JMP 0x080df320
```

Limitations:

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

## vm_freeFunctionValue / analyze_function

Evidence: `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9`

Address: `0x808c114`; source: `docs/stage2af-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention */

void vm_freeFunctionValue(Vm_Fun_Ptr_T *function)

{
  short sVar1;
  Vm_Value *pVVar2;
  Vm_Value_T *pValue;
  int iVar3;
  int iVar4;
  
                    /* Unresolved local var: int i@[DW_OP_reg7(EDI)] */
  iVar4 = 0;
  if (0 < function->argn) {
    iVar3 = 0;
    do {
      while( true ) {
        pValue = (Vm_Value_T *)((int)&function->pArgs->ref + iVar3);
        pValue->ref = pValue->ref + -1;
        vm_cleanupValue(pValue);
        vm_totalValues = vm_totalValues + -1;
        if (pValue == vm_allValues) break;
        pVVar2 = pValue->pPrev;
        pVVar2->pNext = pValue->pNext;
        if (pValue->pNext != (Vm_Value *)0x0) {
          pValue->pNext->pPrev = pVVar2;
        }
LAB_0808c143:
        iVar4 = iVar4 + 1;
        sVar1 = function->argn;
        iVar3 = iVar3 + 0x18;
        pValue->pThis = (Vm_Value *)0x0;
        if (sVar1 <= iVar4) goto LAB_0808c1a8;
      }
      vm_allValues = pValue->pNext;
      if (vm_allValues == (Vm_Value_T *)0x0) goto LAB_0808c143;
      vm_allValues->pPrev = (Vm_Value *)0x0;
      iVar4 = iVar4 + 1;
      sVar1 = function->argn;
      iVar3 = iVar3 + 0x18;
      pValue->pThis = (Vm_Value *)0x0;
    } while (iVar4 < sVar1);
  }
LAB_0808c1a8:
  if (function->pObject != (Vm_Object_T *)0x0) {
    vm_unrefObject(function->pObject);
  }
  pdb_freeMemory(function->pArgs,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x117);
  pdb_freeMemory(function,"VM","/home/cc/xxxx/gameshell/src/vm/vm_var.c",0x118);
  return;
}


```

```text
0x808c114: PUSH EBP
0x808c115: MOV EBP, ESP
0x808c117: PUSH EDI
0x808c118: PUSH ESI
0x808c119: PUSH EBX
0x808c11a: SUB ESP, 0xc
0x808c11d: MOV EAX, dword ptr [EBP + 0x8]
0x808c120: XOR EDI, EDI
0x808c122: CMP word ptr [EAX + 0x6], 0x0
0x808c127: JLE 0x0808c1a8
0x808c129: XOR ESI, ESI
0x808c12b: JMP 0x0808c159
0x808c130: MOV EAX, dword ptr [EBX + 0xc]
0x808c133: MOV EDX, dword ptr [EBX + 0x14]
0x808c136: MOV dword ptr [EDX + 0xc], EAX
0x808c139: MOV EAX, dword ptr [EBX + 0xc]
0x808c13c: TEST EAX, EAX
0x808c13e: JZ 0x0808c143
0x808c140: MOV dword ptr [EAX + 0x14], EDX
0x808c143: MOV EDX, dword ptr [EBP + 0x8]
0x808c146: INC EDI
0x808c147: MOVSX EAX, word ptr [EDX + 0x6]
0x808c14b: ADD ESI, 0x18
0x808c14e: CMP EAX, EDI
0x808c150: MOV dword ptr [EBX + 0x10], 0x0
0x808c157: JLE 0x0808c1a8
0x808c159: MOV EDX, dword ptr [EBP + 0x8]
0x808c15c: MOV EBX, dword ptr [EDX + 0x10]
0x808c15f: SUB ESP, 0xc
0x808c162: ADD EBX, ESI
0x808c164: DEC dword ptr [EBX]
0x808c166: PUSH EBX
0x808c167: CALL 0x0808ad98
0x808c16c: ADD ESP, 0x10
0x808c16f: DEC dword ptr [0x0847fd88]
0x808c175: CMP EBX, dword ptr [0x0847fd8c]
0x808c17b: JNZ 0x0808c130
0x808c17d: MOV EAX, dword ptr [EBX + 0xc]
0x808c180: TEST EAX, EAX
0x808c182: MOV [0x0847fd8c], EAX
0x808c187: JZ 0x0808c143
0x808c189: MOV EDX, dword ptr [EBP + 0x8]
0x808c18c: MOV dword ptr [EAX + 0x14], 0x0
0x808c193: INC EDI
0x808c194: MOVSX EAX, word ptr [EDX + 0x6]
0x808c198: ADD ESI, 0x18
0x808c19b: CMP EAX, EDI
0x808c19d: MOV dword ptr [EBX + 0x10], 0x0
0x808c1a4: JG 0x0808c159
0x808c1a6: MOV ESI, ESI
0x808c1a8: MOV EDX, dword ptr [EBP + 0x8]
0x808c1ab: MOV EAX, dword ptr [EDX + 0xc]
0x808c1ae: TEST EAX, EAX
0x808c1b0: JNZ 0x0808c1eb
0x808c1b2: PUSH 0x117
0x808c1b7: PUSH 0x82acaf8
0x808c1bc: PUSH 0x82a376b
0x808c1c1: MOV EAX, dword ptr [EBP + 0x8]
0x808c1c4: PUSH dword ptr [EAX + 0x10]
0x808c1c7: CALL 0x080ceb94
0x808c1cc: PUSH 0x118
0x808c1d1: PUSH 0x82acaf8
0x808c1d6: PUSH 0x82a376b
0x808c1db: PUSH dword ptr [EBP + 0x8]
0x808c1de: CALL 0x080ceb94
0x808c1e3: LEA ESP, [EBP + -0xc]
0x808c1e6: POP EBX
0x808c1e7: POP ESI
0x808c1e8: POP EDI
0x808c1e9: LEAVE
0x808c1ea: RET
0x808c1eb: SUB ESP, 0xc
0x808c1ee: PUSH EAX
0x808c1ef: CALL 0x080b1618
0x808c1f4: ADD ESP, 0x10
0x808c1f7: JMP 0x0808c1b2
```

Limitations:

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

## vm_isEq / analyze_function

Evidence: `ev_c98c01d582d90083a8c48681f7d36bdd274bd35ed834c0ef8fe22e6ac304baf1`

Address: `0x808c908`; source: `docs/stage2af-evidence/rea.json.gz`

```c

int vm_isEq(Vm_Value_T *pValue1,Vm_Value_T *pValue2)

{
  float fVar1;
  byte bVar2;
  byte bVar3;
  Vm_String_T *pVVar4;
  Vm_String_T *pVVar5;
  uint uVar6;
  uchar *puVar7;
  byte *pbVar8;
  char *pcVar9;
  uchar *puVar10;
  byte *pbVar11;
  char *pcVar12;
  bool bVar13;
  bool bVar14;
  float fVar15;
  
                    /* Unresolved local var: Vm_String_T * s1@[DW_OP_reg2(EDX)]
                       Unresolved local var: Vm_String_T * s2@[DW_OP_reg1(ECX)] */
  bVar2 = pValue1->type;
  bVar3 = pValue2->type;
  if (bVar2 == bVar3) {
    if (bVar2 == 2) goto LAB_0808ca80;
    if (bVar2 == 4) {
      pVVar4 = (pValue2->v).string;
      pVVar5 = (pValue1->v).string;
      if (pVVar5 == pVVar4) {
        return 1;
      }
      uVar6 = pVVar5->len;
      if (uVar6 != pVVar4->len) {
        return 0;
      }
      if ((pVVar4->attrib & pVVar5->attrib & 1) != 0) {
        return 0;
      }
      bVar14 = true;
      pcVar9 = pVVar5->ptr;
      pcVar12 = pVVar4->ptr;
      do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        bVar14 = *pcVar9 == *pcVar12;
        pcVar9 = pcVar9 + 1;
        pcVar12 = pcVar12 + 1;
      } while (bVar14);
      return (uint)bVar14;
    }
  }
  switch(bVar2) {
  case 0:
  case 1:
    return (uint)(bVar3 < 2);
  case 2:
    if (bVar3 == 2) {
LAB_0808ca80:
      return (uint)((pValue1->v).info == (pValue2->v).info);
    }
    if ((2 < bVar3) && (bVar3 == 3)) {
      bVar13 = NAN((float)(pValue1->v).number) || NAN((pValue2->v).real);
      bVar14 = (float)(pValue1->v).number == (pValue2->v).real;
      goto LAB_0808ca5c;
    }
    break;
  case 3:
    pValue1->type = '\x02';
    bVar2 = pValue2->type;
    if (bVar2 == 2) {
      fVar15 = (float)(pValue2->v).number;
      fVar1 = (pValue1->v).real;
    }
    else if (bVar2 < 3) {
      fVar1 = (pValue1->v).real;
      fVar15 = 0.0;
    }
    else {
      if (bVar2 != 3) {
        return 0;
      }
      fVar1 = (pValue1->v).real;
      fVar15 = (pValue2->v).real;
    }
    bVar13 = NAN(fVar1) || NAN(fVar15);
    bVar14 = fVar1 == fVar15;
LAB_0808ca5c:
    return (uint)(byte)(bVar14 & !bVar13);
  case 4:
    if (bVar3 != 4) {
      return 0;
    }
    pVVar4 = (pValue1->v).string;
    pVVar5 = (pValue2->v).string;
    if (pVVar4 == pVVar5) {
      return 1;
    }
    uVar6 = pVVar4->len;
    if (uVar6 != pVVar5->len) {
      return 0;
    }
    if ((pVVar5->attrib & pVVar4->attrib & 1) != 0) {
      return 0;
    }
    bVar14 = false;
    bVar13 = true;
    pbVar8 = (byte *)pVVar4->ptr;
    pbVar11 = (byte *)pVVar5->ptr;
    do {
      if (uVar6 == 0) break;
      uVar6 = uVar6 - 1;
      bVar14 = *pbVar8 < *pbVar11;
      bVar13 = *pbVar8 == *pbVar11;
      pbVar8 = pbVar8 + 1;
      pbVar11 = pbVar11 + 1;
    } while (bVar13);
    return (uint)((!bVar14 && !bVar13) == bVar14);
  case 5:
    if (bVar3 == 5) {
      uVar6 = ((pValue1->v).buffer)->realSize;
      if (uVar6 == ((pValue2->v).buffer)->realSize) {
        if (uVar6 != 0) {
          bVar14 = true;
          puVar7 = ((pValue1->v).buffer)->ptr;
          puVar10 = ((pValue2->v).buffer)->ptr;
          do {
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            bVar14 = *puVar7 == *puVar10;
            puVar7 = puVar7 + 1;
            puVar10 = puVar10 + 1;
          } while (bVar14);
          if (!bVar14) {
            return 0;
          }
        }
        return 1;
      }
    }
    break;
  case 6:
  case 7:
  case 9:
  case 10:
    if ((bVar2 == bVar3) && ((pValue1->v).info == (pValue2->v).info)) {
      return 1;
    }
  }
  return 0;
}


```

```text
0x808c908: PUSH EBP
0x808c909: MOV EBP, ESP
0x808c90b: PUSH EDI
0x808c90c: PUSH ESI
0x808c90d: PUSH EBX
0x808c90e: SUB ESP, 0x14
0x808c911: MOV EDI, dword ptr [EBP + 0x8]
0x808c914: MOV ESI, dword ptr [EBP + 0xc]
0x808c917: MOV DL, byte ptr [EDI + 0x4]
0x808c91a: MOV CL, byte ptr [ESI + 0x4]
0x808c91d: CMP DL, CL
0x808c91f: JZ 0x0808c940
0x808c921: MOVZX EAX, DL
0x808c924: CMP EAX, 0xa
0x808c927: JBE 0x0808c938
0x808c929: XOR EDX, EDX
0x808c92b: ADD ESP, 0x14
0x808c92e: POP EBX
0x808c92f: POP ESI
0x808c930: MOV EAX, EDX
0x808c932: POP EDI
0x808c933: LEAVE
0x808c934: RET
0x808c938: JMP dword ptr [EAX*0x4 + 0x82acf18]
0x808c940: CMP DL, 0x2
0x808c943: JZ 0x0808ca80
0x808c949: CMP DL, 0x4
0x808c94c: JNZ 0x0808c921
0x808c94e: MOV ECX, dword ptr [ESI + 0x8]
0x808c951: MOV EDX, dword ptr [EDI + 0x8]
0x808c954: CMP EDX, ECX
0x808c956: MOV ESI, 0x1
0x808c95b: JZ 0x0808c96c
0x808c95d: MOV EBX, dword ptr [EDX + 0x10]
0x808c960: XOR SI, SI
0x808c963: CMP EBX, dword ptr [ECX + 0x10]
0x808c966: JZ 0x0808caa0
0x808c96c: MOV EDX, ESI
0x808c96e: JMP 0x0808c92b
0x808c970: CMP DL, CL
0x808c972: JNZ 0x0808c929
0x808c974: MOV EAX, dword ptr [ESI + 0x8]
0x808c977: CMP dword ptr [EDI + 0x8], EAX
0x808c97a: MOV EDX, 0x1
0x808c97f: JNZ 0x0808c929
0x808c981: JMP 0x0808c92b
0x808c984: CMP CL, 0x5
0x808c987: JNZ 0x0808c929
0x808c989: MOV EDX, dword ptr [EDI + 0x8]
0x808c98c: MOV ECX, dword ptr [ESI + 0x8]
0x808c98f: MOV EAX, dword ptr [EDX + 0xc]
0x808c992: CMP EAX, dword ptr [ECX + 0xc]
0x808c995: JNZ 0x0808c929
0x808c997: XOR EBX, EBX
0x808c999: TEST EAX, EAX
0x808c99b: JZ 0x0808c9ac
0x808c99d: MOV EDI, dword ptr [ECX + 0x10]
0x808c9a0: MOV ESI, dword ptr [EDX + 0x10]
0x808c9a3: CLD
0x808c9a4: CMP EAX, EAX
0x808c9a6: MOV ECX, EAX
0x808c9a8: CMPSB.REPE ES:EDI, ESI
0x808c9aa: JNZ 0x0808c9b1
0x808c9ac: MOV EBX, 0x1
0x808c9b1: MOV EDX, EBX
0x808c9b3: JMP 0x0808c92b
0x808c9b8: XOR EDX, EDX
0x808c9ba: CMP CL, 0x4
0x808c9bd: JNZ 0x0808c92b
0x808c9c3: MOV EDX, dword ptr [EDI + 0x8]
0x808c9c6: MOV ECX, dword ptr [ESI + 0x8]
0x808c9c9: CMP EDX, ECX
0x808c9cb: MOV EBX, 0x1
0x808c9d0: JZ 0x0808c9b1
0x808c9d2: MOV EAX, dword ptr [EDX + 0x10]
0x808c9d5: XOR BL, BL
0x808c9d7: CMP EAX, dword ptr [ECX + 0x10]
0x808c9da: MOV dword ptr [EBP + -0x18], EAX
0x808c9dd: JNZ 0x0808c9b1
0x808c9df: MOV EAX, dword ptr [ECX + 0x4]
0x808c9e2: AND EAX, dword ptr [EDX + 0x4]
0x808c9e5: TEST AL, 0x1
0x808c9e7: JNZ 0x0808c9b1
0x808c9e9: MOV EAX, dword ptr [EBP + -0x18]
0x808c9ec: MOV ESI, dword ptr [EDX + 0xc]
0x808c9ef: MOV EDI, dword ptr [ECX + 0xc]
0x808c9f2: CMP EAX, EAX
0x808c9f4: MOV ECX, EAX
0x808c9f6: CLD
0x808c9f7: CMPSB.REPE ES:EDI, ESI
0x808c9f9: SETA DL
0x808c9fc: SETC AL
0x808c9ff: XOR EBX, EBX
0x808ca01: CMP DL, AL
0x808ca03: SETZ BL
0x808ca06: MOV EDX, EBX
0x808ca08: JMP 0x0808c92b
0x808ca10: MOV byte ptr [EDI + 0x4], 0x2
0x808ca14: MOVZX EAX, byte ptr [ESI + 0x4]
0x808ca18: CMP EAX, 0x2
0x808ca1b: JZ 0x0808ca8e
0x808ca1d: JLE 0x0808cac8
0x808ca23: XOR EDX, EDX
0x808ca25: CMP EAX, 0x3
0x808ca28: JNZ 0x0808c92b
0x808ca2e: FLD float ptr [EDI + 0x8]
0x808ca31: FLD float ptr [ESI + 0x8]
0x808ca34: FXCH
0x808ca36: FUCOMIP ST0, ST1
0x808ca38: FSTP ST0
0x808ca3a: JMP 0x0808ca5c
0x808ca3c: MOVZX EAX, CL
0x808ca3f: CMP EAX, 0x2
0x808ca42: JZ 0x0808ca80
0x808ca44: JLE 0x0808c929
0x808ca4a: CMP EAX, 0x3
0x808ca4d: JNZ 0x0808c929
0x808ca53: CVTSI2SS XMM0, dword ptr [EDI + 0x8]
0x808ca58: UCOMISS XMM0, dword ptr [ESI + 0x8]
0x808ca5c: SETNP DL
0x808ca5f: SETZ AL
0x808ca62: AND EAX, EDX
0x808ca64: MOVZX EDX, AL
0x808ca67: NOP
0x808ca68: ADD ESP, 0x14
0x808ca6b: POP EBX
0x808ca6c: POP ESI
0x808ca6d: MOV EAX, EDX
0x808ca6f: POP EDI
0x808ca70: LEAVE
0x808ca71: RET
0x808ca72: CMP CL, 0x1
0x808ca75: SETBE AL
0x808ca78: MOVZX EDX, AL
0x808ca7b: JMP 0x0808ca68
0x808ca80: MOV EAX, dword ptr [ESI + 0x8]
0x808ca83: CMP dword ptr [EDI + 0x8], EAX
0x808ca86: SETZ AL
0x808ca89: MOVZX EDX, AL
0x808ca8c: JMP 0x0808ca68
0x808ca8e: CVTSI2SS XMM0, dword ptr [ESI + 0x8]
0x808ca93: FLD float ptr [EDI + 0x8]
0x808ca96: MOVSS dword ptr [EBP + -0x20], XMM0
0x808ca9b: FLD float ptr [EBP + -0x20]
0x808ca9e: JMP 0x0808ca34
0x808caa0: MOV EAX, dword ptr [ECX + 0x4]
0x808caa3: AND EAX, dword ptr [EDX + 0x4]
0x808caa6: TEST AL, 0x1
0x808caa8: JNZ 0x0808c96c
0x808caae: MOV ESI, dword ptr [EDX + 0xc]
0x808cab1: MOV EDI, dword ptr [ECX + 0xc]
0x808cab4: CLD
0x808cab5: CMP EBX, EBX
0x808cab7: MOV ECX, EBX
0x808cab9: CMPSB.REPE ES:EDI, ESI
0x808cabb: SETZ AL
0x808cabe: MOVZX ESI, AL
0x808cac1: MOV EDX, ESI
0x808cac3: JMP 0x0808c92b
0x808cac8: FLD float ptr [EDI + 0x8]
0x808cacb: FLDZ
0x808cacd: JMP 0x0808ca34
```

Limitations:

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

## vm_isZero / procedure_pseudo_code

Evidence: `ev_174f52005ca1def1dcd28fda64fa608062b04eaeef2672493386d0ad773ab397`

Address: `0x808cad4`; source: `docs/vm-core-evidence/rea.json.gz`

```c

int vm_isZero(Vm_Value_T *pValue)

{
  bool bVar1;
  
  switch(pValue->type) {
  case '\0':
  case '\x01':
    return 1;
  case '\x02':
    bVar1 = (pValue->v).info == (void *)0x0;
    break;
  case '\x03':
    bVar1 = (pValue->v).real == 0.0;
    break;
  default:
    return 0;
  case '\n':
    if (((uint)((pValue->v).string)->ptr & 0x8000) != 0) {
      vm_cleanupValue(pValue);
      return 1;
    }
    return 0;
  }
  return (uint)bVar1;
}


```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_pushValue / analyze_function

Evidence: `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212`

Address: `0x8091074`; source: `docs/stage2af-evidence/rea.json.gz`

```c

/* WARNING: Unknown calling convention */

void vm_pushValue(Vm_Value_T *pValue)

{
  if (vm_sp == 0) {
    vm_error("Stack overflow when calculating.\n");
  }
  vm_sp = vm_sp + -1;
  vm_stack[vm_sp].type = '\0';
  vm_copyValue(vm_stack + vm_sp,pValue);
  return;
}


```

```text
0x8091074: PUSH EBP
0x8091075: MOV EBP, ESP
0x8091077: SUB ESP, 0x8
0x809107a: MOV EAX, [0x086619cc]
0x809107f: TEST EAX, EAX
0x8091081: JZ 0x080910b8
0x8091083: DEC EAX
0x8091084: MOV [0x086619cc], EAX
0x8091089: MOV EDX, dword ptr [0x08661964]
0x809108f: LEA EAX, [EAX + EAX*0x2]
0x8091092: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x0
0x8091097: MOV EAX, [0x086619cc]
0x809109c: SUB ESP, 0x8
0x809109f: MOV EDX, dword ptr [0x08661964]
0x80910a5: LEA EAX, [EAX + EAX*0x2]
0x80910a8: PUSH dword ptr [EBP + 0x8]
0x80910ab: LEA EAX, [EDX + EAX*0x8]
0x80910ae: PUSH EAX
0x80910af: CALL 0x0808be24
0x80910b4: LEAVE
0x80910b5: RET
0x80910b8: SUB ESP, 0xc
0x80910bb: PUSH 0x82ad98c
0x80910c0: CALL 0x08097b24
0x80910c5: ADD ESP, 0x10
0x80910c8: MOV EAX, [0x086619cc]
0x80910cd: JMP 0x08091083
```

Limitations:

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

## vm_regularProcessor / analyze_function

Evidence: `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3`

Address: `0x80978c4`; source: `docs/evidence/magic_Linux32-rea.json.gz`

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

```text
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

Limitations:

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

## vm_returnFromFun / procedure_assembly

Evidence: `ev_f4c403eaac43efab5b199f80f74a1e7737969061e2977d9d6e1554147b8f7610`

Address: `0x8094978`; source: `docs/vm-core-evidence/rea.json.gz`

```text
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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_returnFromFun / procedure_pseudo_code

Evidence: `ev_e9492a960f163a9c44c46efb1fdcfca92689aa3ce02ea9f075ac00e305bd3dc2`

Address: `0x8094978`; source: `docs/vm-core-evidence/rea.json.gz`

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

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_traceError / procedure_assembly

Evidence: `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe`

Address: `0x8093520`; source: `docs/vm-catch-evidence/reused.json.gz`

```text
0x8093520: PUSH EBP
0x8093521: MOV EBP, ESP
0x8093523: PUSH EDI
0x8093524: PUSH ESI
0x8093525: PUSH EBX
0x8093526: SUB ESP, 0x82c
0x809352c: CMP dword ptr [EBP + 0x8], -0x1
0x8093530: MOV dword ptr [EBP + 0xfffff7d8], 0x1
0x809353a: JZ 0x08093b4d
0x8093540: CMP dword ptr [0x083cc4e4], -0x1
0x8093547: MOV EAX, 0x0
0x809354c: CMOVNZ EAX, dword ptr [EBP + 0xfffff7d8]
0x8093553: MOV dword ptr [EBP + 0xfffff7d8], EAX
0x8093559: CALL 0x080a9310
0x809355e: SUB ESP, 0xc
0x8093561: PUSH dword ptr [EAX + 0x20]
0x8093564: PUSH EAX
0x8093565: PUSH 0x82ae01b
0x809356a: PUSH 0x800
0x809356f: MOV dword ptr [EBP + 0xfffff7dc], EAX
0x8093575: LEA EAX, [EBP + 0xfffff7e8]
0x809357b: PUSH EAX
0x809357c: CALL 0x0804b914
0x8093581: MOV EAX, dword ptr [EBP + 0xc]
0x8093584: ADD ESP, 0x20
0x8093587: TEST EAX, EAX
0x8093589: MOV byte ptr [EBP + -0x19], 0x0
0x809358d: JZ 0x08093c46
0x8093593: PUSH dword ptr [EBP + 0xc]
0x8093596: CLD
0x8093597: MOV ECX, 0xffffffff
0x809359c: XOR EAX, EAX
0x809359e: LEA EDI, [EBP + 0xfffff7e8]
0x80935a4: SCASB.REPNE ES:EDI
0x80935a6: NOT ECX
0x80935a8: DEC ECX
0x80935a9: PUSH ECX
0x80935aa: PUSH 0x1
0x80935ac: LEA ECX, [EBP + 0xfffff7e8]
0x80935b2: PUSH ECX
0x80935b3: CALL 0x0804bb44
0x80935b8: MOV EAX, [0x086619c0]
0x80935bd: ADD ESP, 0x10
0x80935c0: TEST EAX, EAX
0x80935c2: MOV dword ptr [EBP + 0xfffff7d4], EAX
0x80935c8: JLE 0x080939dc
0x80935ce: LEA EAX, [EAX + EAX*0x4]
0x80935d1: SHL EAX, 0x3
0x80935d4: MOV dword ptr [EBP + 0xfffff7cc], EAX
0x80935da: JMP 0x080936de
0x80935e0: MOV EAX, dword ptr [EBX + 0x20]
0x80935e3: TEST EAX, EAX
0x80935e5: MOV ESI, dword ptr [EBX + 0x24]
0x80935e8: MOV EDX, 0x82b7052
0x80935ed: JZ 0x0809362a
0x80935ef: MOV EDX, dword ptr [EAX + 0x8]
0x80935f2: TEST EDX, EDX
0x80935f4: JZ 0x08093b58
0x80935fa: SUB ESP, 0xc
0x80935fd: PUSH EDX
0x80935fe: MOV EAX, dword ptr [EAX + 0x20]
0x8093601: MOV EAX, dword ptr [EAX + 0x4]
0x8093604: PUSH dword ptr [EAX + 0xc]
0x8093607: PUSH 0x82adfdc
0x809360c: PUSH 0x100
0x8093611: PUSH 0x8483ec0
0x8093616: CALL 0x0804b914
0x809361b: ADD ESP, 0x20
0x809361e: MOV byte ptr [0x08483fbf], 0x0
0x8093625: MOV EDX, 0x8483ec0
0x809362a: PUSH EDI
0x809362b: PUSH EDX
0x809362c: PUSH dword ptr [ESI]
0x809362e: PUSH dword ptr [EBP + 0xfffff7e4]
0x8093634: PUSH dword ptr [EBP + 0xfffff7e0]
0x809363a: PUSH 0x82ae035
0x809363f: PUSH 0x800
0x8093644: LEA EAX, [EBP + 0xfffff7e8]
0x809364a: PUSH EAX
0x809364b: CALL 0x0804b914
0x8093650: MOV ECX, dword ptr [EBP + 0xc]
0x8093653: ADD ESP, 0x20
0x8093656: TEST ECX, ECX
0x8093658: MOV byte ptr [EBP + -0x19], 0x0
0x809365c: JZ 0x08093bcd
0x8093662: PUSH dword ptr [EBP + 0xc]
0x8093665: CLD
0x8093666: MOV ECX, 0xffffffff
0x809366b: LEA EDI, [EBP + 0xfffff7e8]
0x8093671: XOR EAX, EAX
0x8093673: SCASB.REPNE ES:EDI
0x8093675: NOT ECX
0x8093677: DEC ECX
0x8093678: PUSH ECX
0x8093679: PUSH 0x1
0x809367b: LEA EDX, [EBP + 0xfffff7e8]
0x8093681: PUSH EDX
0x8093682: CALL 0x0804bb44
0x8093687: MOV EDX, dword ptr [EBP + 0xfffff7d8]
0x809368d: ADD ESP, 0x10
0x8093690: TEST EDX, EDX
0x8093692: JZ 0x080936c3
0x8093694: MOV EAX, [0x083cc4e4]
0x8093699: TEST EAX, EAX
0x809369b: MOV EAX, 0x0
0x80936a0: CMOVNZ EAX, dword ptr [EBP + 0xfffff7d8]
0x80936a7: SUB ESP, 0xc
0x80936aa: PUSH dword ptr [EBP + 0xc]
0x80936ad: PUSH dword ptr [EBX + 0x20]
0x80936b0: PUSH ESI
0x80936b1: PUSH dword ptr [EBX + 0x14]
0x80936b4: PUSH EBX
0x80936b5: MOV dword ptr [EBP + 0xfffff7d8], EAX
0x80936bb: CALL 0x08092d18
0x80936c0: ADD ESP, 0x20
0x80936c3: DEC dword ptr [EBP + 0xfffff7d4]
0x80936c9: MOV EAX, dword ptr [EBP + 0xfffff7d4]
0x80936cf: SUB dword ptr [EBP + 0xfffff7cc], 0x28
0x80936d6: TEST EAX, EAX
0x80936d8: JLE 0x080939dc
0x80936de: LEA EDX, [EBP + 0xfffff7e4]
0x80936e4: PUSH EDX
0x80936e5: MOV EBX, dword ptr [EBP + 0xfffff7cc]
0x80936eb: LEA ECX, [EBP + 0xfffff7e0]
0x80936f1: ADD EBX, dword ptr [0x086619c8]
0x80936f7: PUSH ECX
0x80936f8: PUSH dword ptr [EBX + 0x10]
0x80936fb: PUSH dword ptr [EBX + 0xc]
0x80936fe: CALL 0x08097654
0x8093703: MOV EAX, dword ptr [EBX + 0x8]
0x8093706: ADD ESP, 0x10
0x8093709: TEST AL, 0x4
0x809370b: JNZ 0x080935e0
0x8093711: TEST AL, 0x1
0x8093713: JZ 0x08093918
0x8093719: MOV EAX, dword ptr [EBX + 0x20]
0x809371c: TEST EAX, EAX
0x809371e: MOV EDX, 0x82b7052
0x8093723: JZ 0x08093760
0x8093725: MOV EDX, dword ptr [EAX + 0x8]
0x8093728: TEST EDX, EDX
0x809372a: JZ 0x08093ba8
0x8093730: SUB ESP, 0xc
0x8093733: PUSH EDX
0x8093734: MOV EAX, dword ptr [EAX + 0x20]
0x8093737: MOV EAX, dword ptr [EAX + 0x4]
0x809373a: PUSH dword ptr [EAX + 0xc]
0x809373d: PUSH 0x82adfdc
0x8093742: PUSH 0x100
0x8093747: PUSH 0x8483ec0
0x809374c: CALL 0x0804b914
0x8093751: ADD ESP, 0x20
0x8093754: MOV byte ptr [0x08483fbf], 0x0
0x809375b: MOV EDX, 0x8483ec0
0x8093760: SUB ESP, 0xc
0x8093763: PUSH EDX
0x8093764: MOV EAX, dword ptr [EBX + 0x24]
0x8093767: PUSH dword ptr [EAX]
0x8093769: PUSH 0x82ae049
0x809376e: PUSH 0x800
0x8093773: LEA ECX, [EBP + 0xfffff7e8]
0x8093779: PUSH ECX
0x809377a: CALL 0x0804b914
0x809377f: MOV EAX, dword ptr [EBP + 0xc]
0x8093782: ADD ESP, 0x20
0x8093785: TEST EAX, EAX
0x8093787: MOV byte ptr [EBP + -0x19], 0x0
0x809378b: JZ 0x08093be1
0x8093791: PUSH dword ptr [EBP + 0xc]
0x8093794: CLD
0x8093795: MOV ECX, 0xffffffff
0x809379a: LEA EDI, [EBP + 0xfffff7e8]
0x80937a0: XOR EAX, EAX
0x80937a2: SCASB.REPNE ES:EDI
0x80937a4: NOT ECX
0x80937a6: DEC ECX
0x80937a7: PUSH ECX
0x80937a8: PUSH 0x1
0x80937aa: LEA EDX, [EBP + 0xfffff7e8]
0x80937b0: PUSH EDX
0x80937b1: CALL 0x0804bb44
0x80937b6: MOV EAX, dword ptr [EBP + 0xfffff7d8]
0x80937bc: ADD ESP, 0x10
0x80937bf: TEST EAX, EAX
0x80937c1: JZ 0x080936c3
0x80937c7: MOV EAX, dword ptr [EBX + 0x4]
0x80937ca: XOR EDI, EDI
0x80937cc: TEST EAX, EAX
0x80937ce: JG 0x08093865
0x80937d4: JMP 0x080936c3
0x80937dc: XOR EAX, EAX
0x80937de: CLD
0x80937df: MOV ECX, 0xffffffff
0x80937e4: LEA EDI, [EBP + 0xfffff7e8]
0x80937ea: SCASB.REPNE ES:EDI
0x80937ec: NOT ECX
0x80937ee: DEC ECX
0x80937ef: MOV dword ptr [EBP + ECX*0x1 + 0xfffff7e8], 0x6f726428
0x80937fa: MOV dword ptr [EBP + ECX*0x1 + 0xfffff7ec], 0x64657070
0x8093805: MOV word ptr [EBP + ECX*0x1 + 0xfffff7f0], 0x29
0x809380f: MOV EAX, dword ptr [EBP + 0xc]
0x8093812: TEST EAX, EAX
0x8093814: JZ 0x080938f8
0x809381a: PUSH dword ptr [EBP + 0xc]
0x809381d: CLD
0x809381e: XOR EAX, EAX
0x8093820: MOV ECX, 0xffffffff
0x8093825: LEA EDI, [EBP + 0xfffff7e8]
0x809382b: SCASB.REPNE ES:EDI
0x809382d: NOT ECX
0x809382f: DEC ECX
0x8093830: PUSH ECX
0x8093831: PUSH 0x1
0x8093833: LEA EDX, [EBP + 0xfffff7e8]
0x8093839: PUSH EDX
0x809383a: CALL 0x0804bb44
0x809383f: ADD ESP, 0x10
0x8093842: PUSH dword ptr [EBP + 0xc]
0x8093845: PUSH 0x1
0x8093847: PUSH 0x1
0x8093849: PUSH 0x82b0725
0x809384e: CALL 0x0804bb44
0x8093853: MOV EDI, dword ptr [EBP + 0xfffff7d0]
0x8093859: ADD ESP, 0x10
0x809385c: CMP dword ptr [EBX + 0x4], EDI
0x809385f: JLE 0x080936c3
0x8093865: MOV EAX, dword ptr [EBX]
0x8093867: ADD EAX, EDI
0x8093869: LEA EAX, [EAX + EAX*0x2]
0x809386c: MOV EDX, dword ptr [0x08661964]
0x8093872: LEA ECX, [EDI + 0x1]
0x8093875: LEA ESI, [EDX + EAX*0x8]
0x8093878: PUSH EAX
0x8093879: PUSH ECX
0x809387a: PUSH 0x82ae05c
0x809387f: LEA EAX, [EBP + 0xfffff7e8]
0x8093885: PUSH EAX
0x8093886: MOV dword ptr [EBP + 0xfffff7d0], ECX
0x809388c: CALL 0x0804bb34
0x8093891: ADD EDI, dword ptr [EBX]
0x8093893: ADD ESP, 0x10
0x8093896: CMP EDI, dword ptr [0x086619cc]
0x809389c: JL 0x080937dc
0x80938a2: LEA EDI, [EBP + 0xfffff7e8]
0x80938a8: CLD
0x80938a9: MOV ECX, 0xffffffff
0x80938ae: PUSH EAX
0x80938af: XOR EAX, EAX
0x80938b1: SCASB.REPNE ES:EDI
0x80938b3: NOT ECX
0x80938b5: MOV EAX, 0x801
0x80938ba: SUB EAX, ECX
0x80938bc: PUSH EAX
0x80938bd: PUSH EDI
0x80938be: PUSH ESI
0x80938bf: CALL 0x0808bd88
0x80938c4: POP EDX
0x80938c5: POP ECX
0x80938c6: PUSH EAX
0x80938c7: CLD
0x80938c8: XOR EAX, EAX
0x80938ca: MOV ECX, 0xffffffff
0x80938cf: LEA EDI, [EBP + 0xfffff7e8]
0x80938d5: SCASB.REPNE ES:EDI
0x80938d7: NOT ECX
0x80938d9: LEA ECX, [ECX + EBP*0x1 + 0xfffff7e7]
0x80938e0: PUSH ECX
0x80938e1: CALL 0x0804ba94
0x80938e6: MOV EAX, dword ptr [EBP + 0xc]
0x80938e9: ADD ESP, 0x10
0x80938ec: TEST EAX, EAX
0x80938ee: MOV byte ptr [EBP + -0x19], 0x0
0x80938f2: JNZ 0x0809381a
0x80938f8: SUB ESP, 0xc
0x80938fb: LEA EDX, [EBP + 0xfffff7e8]
0x8093901: PUSH EDX
0x8093902: CALL 0x08098110
0x8093907: MOV dword ptr [ESP], 0x82b0725
0x809390e: CALL 0x08098110
0x8093913: JMP 0x08093853
0x8093918: MOV EAX, dword ptr [EBX + 0x20]
0x809391b: TEST EAX, EAX
0x809391d: MOV EDX, 0x82b7052
0x8093922: JZ 0x0809395f
0x8093924: MOV EDX, dword ptr [EAX + 0x8]
0x8093927: TEST EDX, EDX
0x8093929: JZ 0x08093b80
0x809392f: SUB ESP, 0xc
0x8093932: PUSH EDX
0x8093933: MOV EAX, dword ptr [EAX + 0x20]
0x8093936: MOV EAX, dword ptr [EAX + 0x4]
0x8093939: PUSH dword ptr [EAX + 0xc]
0x809393c: PUSH 0x82adfdc
0x8093941: PUSH 0x100
0x8093946: PUSH 0x8483ec0
0x809394b: CALL 0x0804b914
0x8093950: ADD ESP, 0x20
0x8093953: MOV byte ptr [0x08483fbf], 0x0
0x809395a: MOV EDX, 0x8483ec0
0x809395f: PUSH EAX
0x8093960: PUSH EDX
0x8093961: PUSH dword ptr [EBX + 0x24]
0x8093964: PUSH dword ptr [EBP + 0xfffff7e4]
0x809396a: PUSH dword ptr [EBP + 0xfffff7e0]
0x8093970: PUSH 0x82ae035
0x8093975: PUSH 0x800
0x809397a: LEA ECX, [EBP + 0xfffff7e8]
0x8093980: PUSH ECX
0x8093981: CALL 0x0804b914
0x8093986: MOV EAX, dword ptr [EBP + 0xc]
0x8093989: ADD ESP, 0x20
0x809398c: TEST EAX, EAX
0x809398e: MOV byte ptr [EBP + -0x19], 0x0
0x8093992: JZ 0x08093bf5
0x8093998: PUSH dword ptr [EBP + 0xc]
0x809399b: CLD
0x809399c: XOR EAX, EAX
0x809399e: MOV ECX, 0xffffffff
0x80939a3: LEA EDI, [EBP + 0xfffff7e8]
0x80939a9: SCASB.REPNE ES:EDI
0x80939ab: NOT ECX
0x80939ad: DEC ECX
0x80939ae: PUSH ECX
0x80939af: PUSH 0x1
0x80939b1: LEA EDX, [EBP + 0xfffff7e8]
0x80939b7: PUSH EDX
0x80939b8: CALL 0x0804bb44
0x80939bd: ADD ESP, 0x10
0x80939c0: DEC dword ptr [EBP + 0xfffff7d4]
0x80939c6: MOV EAX, dword ptr [EBP + 0xfffff7d4]
0x80939cc: SUB dword ptr [EBP + 0xfffff7cc], 0x28
0x80939d3: TEST EAX, EAX
0x80939d5: JG 0x080936de
0x80939db: NOP
0x80939dc: LEA ECX, [EBP + 0xfffff7e4]
0x80939e2: PUSH ECX
0x80939e3: MOV ECX, dword ptr [EBP + 0xfffff7d4]
0x80939e9: LEA EDX, [ECX + ECX*0x4]
0x80939ec: LEA EAX, [EBP + 0xfffff7e0]
0x80939f2: PUSH EAX
0x80939f3: SHL EDX, 0x3
0x80939f6: MOV EAX, [0x086619c8]
0x80939fb: PUSH dword ptr [EDX + EAX*0x1 + 0x10]
0x80939ff: MOV EAX, dword ptr [EDX + EAX*0x1 + 0xc]
0x8093a03: DEC EAX
0x8093a04: PUSH EAX
0x8093a05: CALL 0x08097654
0x8093a0a: POP EAX
0x8093a0b: PUSH dword ptr [EBP + 0xfffff7e4]
0x8093a11: PUSH dword ptr [EBP + 0xfffff7e0]
0x8093a17: PUSH 0x82ae06b
0x8093a1c: PUSH 0x800
0x8093a21: LEA EAX, [EBP + 0xfffff7e8]
0x8093a27: PUSH EAX
0x8093a28: CALL 0x0804b914
0x8093a2d: MOV EDI, dword ptr [EBP + 0xc]
0x8093a30: ADD ESP, 0x20
0x8093a33: TEST EDI, EDI
0x8093a35: MOV byte ptr [EBP + -0x19], 0x0
0x8093a39: JZ 0x08093c32
0x8093a3f: PUSH dword ptr [EBP + 0xc]
0x8093a42: CLD
0x8093a43: MOV ECX, 0xffffffff
0x8093a48: XOR EAX, EAX
0x8093a4a: LEA EDI, [EBP + 0xfffff7e8]
0x8093a50: SCASB.REPNE ES:EDI
0x8093a52: NOT ECX
0x8093a54: DEC ECX
0x8093a55: PUSH ECX
0x8093a56: PUSH 0x1
0x8093a58: LEA ECX, [EBP + 0xfffff7e8]
0x8093a5e: PUSH ECX
0x8093a5f: CALL 0x0804bb44
0x8093a64: MOV EAX, dword ptr [EBP + 0x8]
0x8093a67: ADD ESP, 0x10
0x8093a6a: CMP EAX, dword ptr [0x086619c0]
0x8093a70: JGE 0x08093b40
0x8093a76: MOV EDX, dword ptr [EBP + 0xfffff7dc]
0x8093a7c: MOV EAX, dword ptr [EDX + 0x98]
0x8093a82: TEST EAX, EAX
0x8093a84: JLE 0x08093ab1
0x8093a86: MOV ESI, ESI
0x8093a88: LEA EAX, [EAX + EAX*0x4]
0x8093a8b: SHL EAX, 0x2
0x8093a8e: ADD EAX, dword ptr [EDX + 0xa0]
0x8093a94: MOV ECX, dword ptr [EBP + 0x8]
0x8093a97: CMP dword ptr [EAX + -0x10], ECX
0x8093a9a: JLE 0x08093ab1
0x8093a9c: CALL 0x080aae08
0x8093aa1: MOV EDX, dword ptr [EBP + 0xfffff7dc]
0x8093aa7: MOV EAX, dword ptr [EDX + 0x98]
0x8093aad: TEST EAX, EAX
0x8093aaf: JG 0x08093a88
0x8093ab1: MOV EDX, dword ptr [EBP + 0x8]
0x8093ab4: LEA EAX, [EDX + EDX*0x4]
0x8093ab7: SHL EAX, 0x3
0x8093aba: ADD EAX, dword ptr [0x086619c8]
0x8093ac0: MOV EDX, dword ptr [EAX + 0x2c]
0x8093ac3: ADD EDX, dword ptr [EAX + 0x28]
0x8093ac6: SUB ESP, 0xc
0x8093ac9: SUB EDX, dword ptr [0x086619cc]
0x8093acf: PUSH EDX
0x8093ad0: CALL 0x08091ab0
0x8093ad5: ADD ESP, 0x10
0x8093ad8: MOV ECX, dword ptr [0x086619c0]
0x8093ade: CMP ECX, dword ptr [EBP + 0x8]
0x8093ae1: JLE 0x08093b40
0x8093ae3: MOV ESI, dword ptr [0x0860ae64]
0x8093ae9: LEA EDX, [ECX + ECX*0x4]
0x8093aec: MOV EAX, [0x086619c8]
0x8093af1: TEST ESI, ESI
0x8093af3: LEA EBX, [EAX + EDX*0x8]
0x8093af6: JZ 0x08093c0c
0x8093afc: CALL 0x080bb0d4
0x8093b01: DEC dword ptr [0x086619c0]
0x8093b07: CALL 0x0809789c
0x8093b0c: CALL 0x080bb0e8
0x8093b11: MOV EAX, dword ptr [EBX + 0x8]
0x8093b14: TEST AL, 0x1
0x8093b16: JNZ 0x08093ad8
0x8093b18: MOV EDX, dword ptr [EBX + 0x20]
0x8093b1b: TEST EDX, EDX
0x8093b1d: JNZ 0x08093c1e
0x8093b23: TEST AL, 0x8
0x8093b25: JNZ 0x08093ad8
0x8093b27: MOV ECX, dword ptr [EBP + 0xfffff7dc]
0x8093b2d: AND dword ptr [ECX + 0x28], 0xfffffeff
0x8093b34: MOV ECX, dword ptr [0x086619c0]
0x8093b3a: CMP ECX, dword ptr [EBP + 0x8]
0x8093b3d: JG 0x08093ae3
0x8093b3f: NOP
0x8093b40: MOV EAX, dword ptr [EBP + 0x8]
0x8093b43: MOV [0x086619c0], EAX
0x8093b48: CALL 0x0809789c
0x8093b4d: LEA ESP, [EBP + -0xc]
0x8093b50: POP EBX
0x8093b51: POP ESI
0x8093b52: POP EDI
0x8093b53: LEAVE
0x8093b54: RET
0x8093b58: MOV EAX, dword ptr [EAX + 0x20]
0x8093b5b: MOV EAX, dword ptr [EAX + 0x4]
0x8093b5e: PUSH dword ptr [EAX + 0xc]
0x8093b61: PUSH 0x82d3f39
0x8093b66: PUSH 0x100
0x8093b6b: PUSH 0x8483ec0
0x8093b70: CALL 0x0804b914
0x8093b75: ADD ESP, 0x10
0x8093b78: JMP 0x0809361e
0x8093b80: MOV EAX, dword ptr [EAX + 0x20]
0x8093b83: MOV EAX, dword ptr [EAX + 0x4]
0x8093b86: PUSH dword ptr [EAX + 0xc]
0x8093b89: PUSH 0x82d3f39
0x8093b8e: PUSH 0x100
0x8093b93: PUSH 0x8483ec0
0x8093b98: CALL 0x0804b914
0x8093b9d: ADD ESP, 0x10
0x8093ba0: JMP 0x08093953
0x8093ba8: MOV EAX, dword ptr [EAX + 0x20]
0x8093bab: MOV EAX, dword ptr [EAX + 0x4]
0x8093bae: PUSH dword ptr [EAX + 0xc]
0x8093bb1: PUSH 0x82d3f39
0x8093bb6: PUSH 0x100
0x8093bbb: PUSH 0x8483ec0
0x8093bc0: CALL 0x0804b914
0x8093bc5: ADD ESP, 0x10
0x8093bc8: JMP 0x08093754
0x8093bcd: SUB ESP, 0xc
0x8093bd0: LEA EDX, [EBP + 0xfffff7e8]
0x8093bd6: PUSH EDX
0x8093bd7: CALL 0x08098110
0x8093bdc: JMP 0x08093687
0x8093be1: SUB ESP, 0xc
0x8093be4: LEA EAX, [EBP + 0xfffff7e8]
0x8093bea: PUSH EAX
0x8093beb: CALL 0x08098110
0x8093bf0: JMP 0x080937b6
0x8093bf5: SUB ESP, 0xc
0x8093bf8: LEA EAX, [EBP + 0xfffff7e8]
0x8093bfe: PUSH EAX
0x8093bff: CALL 0x08098110
0x8093c04: ADD ESP, 0x10
0x8093c07: JMP 0x080939c0
0x8093c0c: LEA EAX, [ECX + -0x1]
0x8093c0f: MOV [0x086619c0], EAX
0x8093c14: CALL 0x0809789c
0x8093c19: JMP 0x08093b11
0x8093c1e: SUB ESP, 0xc
0x8093c21: PUSH EDX
0x8093c22: CALL 0x080b1804
0x8093c27: ADD ESP, 0x10
0x8093c2a: MOV EAX, dword ptr [EBX + 0x8]
0x8093c2d: JMP 0x08093b23
0x8093c32: SUB ESP, 0xc
0x8093c35: LEA EDX, [EBP + 0xfffff7e8]
0x8093c3b: PUSH EDX
0x8093c3c: CALL 0x08098110
0x8093c41: JMP 0x08093a64
0x8093c46: SUB ESP, 0xc
0x8093c49: LEA EDX, [EBP + 0xfffff7e8]
0x8093c4f: PUSH EDX
0x8093c50: CALL 0x08098110
0x8093c55: JMP 0x080935b8
```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Assembly is Ghidra Listing text and does not claim textual equivalence with Hopper output.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.

## vm_traceError / procedure_pseudo_code

Evidence: `ev_3aa06be9e85308cb2db022171ebc8b97169c4291640f5d39b9176041544f81e4`

Address: `0x8093520`; source: `docs/vm-catch-evidence/reused.json.gz`

```c

/* WARNING: Unknown calling convention */

void vm_traceError(int toFunCp,FILE *fp)

{
  Vm_Value_T *pValue;
  char cVar1;
  Vm_Fun_T *pFun;
  Vm_Object *pVVar2;
  bool bVar3;
  Vm_PCB_T *pVVar4;
  size_t __n;
  char *__src;
  int iVar5;
  uint uVar6;
  char *pcVar7;
  Vm_FunCall_Context_T *pVVar8;
  char *unaff_EDI;
  bool bVar9;
  int local_838;
  int i;
  int flag;
  Vm_PCB_T *pPCB;
  char *file;
  int line;
  char buf [2048];
  
                    /* Unresolved local var: Vm_FunCall_Context_T * pContext@[DW_OP_reg3(EBX)]
                       Unresolved local var: Vm_Value_T * pValue@[DW_OP_reg6(ESI)] */
  if (toFunCp != -1) {
    bVar9 = _vm_showFullFrameWhenError != -1;
    pVVar4 = vm_getCurrentPCB();
    snprintf(buf,0x800,"Current thread: \"%s\"(%u)\n",pVVar4,pVVar4->tid);
    buf[0x7ff] = '\0';
    if (fp == (FILE *)0x0) {
      vm_errorOut(buf);
    }
    else {
      uVar6 = 0xffffffff;
      pcVar7 = buf;
      do {
        unaff_EDI = pcVar7;
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        unaff_EDI = pcVar7 + 1;
        cVar1 = *pcVar7;
        pcVar7 = unaff_EDI;
      } while (cVar1 != '\0');
      fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
    }
    i = vm_funCp;
    if (0 < vm_funCp) {
      local_838 = vm_funCp * 0x28;
      do {
        while( true ) {
          pVVar8 = (Vm_FunCall_Context_T *)((int)&vm_funContext->bp + local_838);
          vm_getSymbolInfoFromNode(pVVar8->ip,pVVar8->cs,&file,&line);
          if ((pVVar8->flags & 4) == 0) break;
                    /* Unresolved local var: Vm_Fun_T * pFun@[DW_OP_reg6(ESI)] */
          pVVar2 = pVVar8->thisObject;
          pFun = (Vm_Fun_T *)(pVVar8->f).pEfun;
          pcVar7 = "global";
          if (pVVar2 != (Vm_Object *)0x0) {
            if (pVVar2->oid == 0) {
              snprintf(vm_objectName::buf,0x100,"%s",pVVar2->pProgram->strName->ptr);
            }
            else {
              snprintf(vm_objectName::buf,0x100,"%s#%d",pVVar2->pProgram->strName->ptr,pVVar2->oid);
            }
            vm_objectName::buf[0xff] = '\0';
            pcVar7 = vm_objectName::buf;
          }
          snprintf(buf,0x800,"At %s:%d(%s) in %s\n",file,line,pFun->name,pcVar7,unaff_EDI);
          buf[0x7ff] = '\0';
          if (fp == (FILE *)0x0) {
            vm_errorOut(buf);
          }
          else {
            uVar6 = 0xffffffff;
            pcVar7 = buf;
            do {
              unaff_EDI = pcVar7;
              if (uVar6 == 0) break;
              uVar6 = uVar6 - 1;
              unaff_EDI = pcVar7 + 1;
              cVar1 = *pcVar7;
              pcVar7 = unaff_EDI;
            } while (cVar1 != '\0');
            fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
          }
          if (bVar9) {
            bVar3 = false;
            if (_vm_showFullFrameWhenError != 0) {
              bVar3 = bVar9;
            }
            vm_showFunContext(pVVar8,pVVar8->consititution,pFun,pVVar8->thisObject,fp);
            bVar9 = bVar3;
          }
LAB_080936c3:
          i = i + -1;
          local_838 = local_838 + -0x28;
          if (i < 1) goto LAB_080939dc;
        }
        if ((pVVar8->flags & 1) != 0) {
          pVVar2 = pVVar8->thisObject;
          pcVar7 = "global";
          if (pVVar2 != (Vm_Object *)0x0) {
            if (pVVar2->oid == 0) {
              snprintf(vm_objectName::buf,0x100,"%s",pVVar2->pProgram->strName->ptr);
            }
            else {
              snprintf(vm_objectName::buf,0x100,"%s#%d",pVVar2->pProgram->strName->ptr,pVVar2->oid);
            }
            vm_objectName::buf[0xff] = '\0';
            pcVar7 = vm_objectName::buf;
          }
          snprintf(buf,0x800,"At EFUN: %s in %s\n",((pVVar8->f).pEfun)->word,pcVar7);
          buf[0x7ff] = '\0';
          if (fp == (FILE *)0x0) {
            vm_errorOut(buf);
          }
          else {
            uVar6 = 0xffffffff;
            pcVar7 = buf;
            do {
              unaff_EDI = pcVar7;
              if (uVar6 == 0) break;
              uVar6 = uVar6 - 1;
              unaff_EDI = pcVar7 + 1;
              cVar1 = *pcVar7;
              pcVar7 = unaff_EDI;
            } while (cVar1 != '\0');
            fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
          }
                    /* Unresolved local var: int k@[DW_OP_reg7(EDI)] */
          if ((bVar9) && (unaff_EDI = (char *)0x0, pcVar7 = unaff_EDI, 0 < pVVar8->argNum)) {
            do {
              unaff_EDI = pcVar7 + 1;
              pValue = vm_stack + (int)(pcVar7 + pVVar8->bp);
              sprintf(buf,"Argument %d = ",unaff_EDI,(int)(pcVar7 + pVVar8->bp) * 3);
              if ((int)(pcVar7 + pVVar8->bp) < vm_sp) {
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                builtin_strncpy(buf + (~uVar6 - 1),"(dro",4);
                builtin_strncpy(buf + ~uVar6 + 3,"pped)",6);
                if (fp != (FILE *)0x0) goto LAB_0809381a;
LAB_080938f8:
                vm_errorOut(buf);
                vm_errorOut("\n");
              }
              else {
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                __n = 0x801 - ~uVar6;
                __src = vm_typeValue(pValue);
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                strncpy(buf + (~uVar6 - 1),__src,__n);
                buf[0x7ff] = '\0';
                if (fp == (FILE *)0x0) goto LAB_080938f8;
LAB_0809381a:
                uVar6 = 0xffffffff;
                pcVar7 = buf;
                do {
                  if (uVar6 == 0) break;
                  uVar6 = uVar6 - 1;
                  cVar1 = *pcVar7;
                  pcVar7 = pcVar7 + 1;
                } while (cVar1 != '\0');
                fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
                fwrite("\n",1,1,(FILE *)fp);
              }
              pcVar7 = unaff_EDI;
            } while ((int)unaff_EDI < pVVar8->argNum);
          }
          goto LAB_080936c3;
        }
        pVVar2 = pVVar8->thisObject;
        pcVar7 = "global";
        iVar5 = 0;
        if (pVVar2 != (Vm_Object *)0x0) {
          if (pVVar2->oid == 0) {
            iVar5 = snprintf(vm_objectName::buf,0x100,"%s",pVVar2->pProgram->strName->ptr);
          }
          else {
            iVar5 = snprintf(vm_objectName::buf,0x100,"%s#%d",pVVar2->pProgram->strName->ptr,
                             pVVar2->oid);
          }
          vm_objectName::buf[0xff] = '\0';
          pcVar7 = vm_objectName::buf;
        }
        snprintf(buf,0x800,"At %s:%d(%s) in %s\n",file,line,(pVVar8->f).pEfun,pcVar7,iVar5);
        buf[0x7ff] = '\0';
        if (fp == (FILE *)0x0) {
          vm_errorOut(buf);
        }
        else {
          uVar6 = 0xffffffff;
          pcVar7 = buf;
          do {
            unaff_EDI = pcVar7;
            if (uVar6 == 0) break;
            uVar6 = uVar6 - 1;
            unaff_EDI = pcVar7 + 1;
            cVar1 = *pcVar7;
            pcVar7 = unaff_EDI;
          } while (cVar1 != '\0');
          fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
        }
        i = i + -1;
        local_838 = local_838 + -0x28;
      } while (0 < i);
    }
LAB_080939dc:
    vm_getSymbolInfoFromNode(vm_funContext[i].ip + -1,vm_funContext[i].cs,&file,&line);
    snprintf(buf,0x800,"Call from %s:%d\n",file,line);
    buf[0x7ff] = '\0';
    if (fp == (FILE *)0x0) {
      vm_errorOut(buf);
    }
    else {
      uVar6 = 0xffffffff;
      pcVar7 = buf;
      do {
        if (uVar6 == 0) break;
        uVar6 = uVar6 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      fwrite(buf,1,~uVar6 - 1,(FILE *)fp);
    }
    if (toFunCp < vm_funCp) {
                    /* Unresolved local var: int n@[???] */
      iVar5 = pVVar4->catchCp;
      while ((0 < iVar5 && (toFunCp < pVVar4->catchContext[iVar5 + -1].funCp))) {
        vm_endThreadCatch();
        iVar5 = pVVar4->catchCp;
      }
      vm_popNStack((vm_funContext[toFunCp + 1].argNum + vm_funContext[toFunCp + 1].bp) - vm_sp);
LAB_08093ad8:
      if (toFunCp < vm_funCp) {
        while( true ) {
          pVVar8 = vm_funContext;
          iVar5 = vm_funCp;
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
          uVar6 = pVVar8[iVar5].flags;
          if ((uVar6 & 1) != 0) break;
          if (pVVar8[iVar5].thisObject != (Vm_Object_T *)0x0) {
            vm_unExecRefObject(pVVar8[iVar5].thisObject);
            uVar6 = pVVar8[iVar5].flags;
          }
          if ((uVar6 & 8) != 0) break;
          pVVar4->flags = pVVar4->flags & 0xfffffeff;
          if (vm_funCp <= toFunCp) goto LAB_08093b40;
        }
        goto LAB_08093ad8;
      }
    }
LAB_08093b40:
    vm_funCp = toFunCp;
    vm_updateCurrentFunCallContext();
  }
  return;
}


```

Limitations:

- The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.
- The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.
- Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.
- Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.
- Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.
- External functions and functions without an analyzable body return null; other decompiler failures remain explicit.
- macOS sessions require a matching executable Ghidra native decompiler; REA checks for it but does not build native components or change Gatekeeper quarantine state.
