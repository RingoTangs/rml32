# 2AR机器码锚点

逐指令字节取自原ELF，汇编原文来自对应完整记录；不是原程序执行轨迹。

## A001

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302329

```text
0x8091cf9: MOV EAX, EBX
bytes: 89d8
```

## A002

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302376

```text
0x8091d28: JNZ 0x08091cf9
bytes: 75cf
```

## A003

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302397

```text
0x8091d3d: CALL 0x080ce784
bytes: e842ca0300
```

## A004

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302442

```text
0x8091d6a: CALL 0x080ce784
bytes: e815ca0300
```

## A005

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302835

```text
0x8091ef3: PUSH 0x360
bytes: 6860030000
```

## A006

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302506

```text
0x8091daa: MOV EDX, dword ptr [EBP + 0x18]
bytes: 8b5518
```

## A007

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302656

```text
0x8091e40: CALL 0x08089f10
bytes: e8cb80ffff
```

## A008

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302675

```text
0x8091e53: MOV AL, byte ptr [EDX + EAX*0x8 + 0x4]
bytes: 8a44c204
```

## A009

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302722

```text
0x8091e82: MOV byte ptr [EAX + ECX*0x1 + 0x4], 0x0
bytes: c644080400
```

## A010

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302736

```text
0x8091e90: MOV dword ptr [EBX + EDI*0x1 + 0x8], EDX
bytes: 89543b08
```

## A011

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302749

```text
0x8091e9d: MOV byte ptr [EDX + EAX*0x8 + 0x5], 0x0
bytes: c644c20500
```

## A012

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312400

```text
0x8094450: MOV DL, byte ptr [EAX + ECX*0x1 + 0x4]
bytes: 8a540804
```

## A013

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312465

```text
0x8094491: MOV dword ptr [0x086619cc], EDX
bytes: 8915cc196608
```

## A014

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312524

```text
0x80944cc: MOVSX EAX, word ptr [ESI + 0x6]
bytes: 0fbf4606
```

## A015

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312536

```text
0x80944d8: MOV EAX, dword ptr [ESI + 0xc]
bytes: 8b460c
```

## A016

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312620

```text
0x809452c: CMP dword ptr [ESI], 0x1
bytes: 833e01
```

## A017

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312662

```text
0x8094556: MOV CL, byte ptr [EBX + EDX*0x1 + 0x4]
bytes: 8a4c1304
```

## A018

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312732

```text
0x809459c: MOV byte ptr [EBX + EAX*0x1 + 0x4], 0x0
bytes: c644030400
```

## A019

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312741

```text
0x80945a5: MOV byte ptr [EBX + EAX*0x1 + 0x5], 0x0
bytes: c644030500
```

## A020

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 312758

```text
0x80945b6: CALL 0x0808c114
bytes: e8597bffff
```

## A021

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 313104

```text
0x8094710: CALL 0x0808c0b8
bytes: e8a379ffff
```

## A022

vm_callFunPtr · `ev_a50406aa6a4cd4b2b95b079a7b126ba210a8ecaa6919839ac438372500c0159d` · file offset 313142

```text
0x8094736: CALL 0x08091074
bytes: e839c9ffff
```

## A023

vm_pushValue · `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212` · file offset 299183

```text
0x80910af: CALL 0x0808be24
bytes: e870adffff
```

## A024

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278238

```text
0x808bede: MOV EAX, dword ptr [EBX + 0x8]
bytes: 8b4308
```

## A025

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278214

```text
0x808bec6: INC dword ptr [EAX]
bytes: ff00
```

## A026

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278151

```text
0x808be87: MOV EAX, dword ptr [EBX + 0x8]
bytes: 8b4308
```

## A027

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278174

```text
0x808be9e: CALL 0x0808ad98
bytes: e8f5eeffff
```

## A028

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278179

```text
0x808bea3: MOVZX ECX, byte ptr [EBX + 0x4]
bytes: 0fb64b04
```

## A029

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278195

```text
0x808beb3: MOV EAX, dword ptr [EBX + 0x8]
bytes: 8b4308
```

## A030

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278203

```text
0x808bebb: JZ 0x0808bec3
bytes: 7406
```

## A031

vm_cleanupValue · `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc` · file offset 273858

```text
0x808adc2: MOV byte ptr [EDI + 0x4], 0x0
bytes: c6470400
```

## A032

vm_cleanupValue · `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc` · file offset 273862

```text
0x808adc6: MOV byte ptr [EDI + 0x5], 0x0
bytes: c6470500
```

## A033

vm_pushValue · `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212` · file offset 299135

```text
0x809107f: TEST EAX, EAX
bytes: 85c0
```

## A034

vm_pushValue · `ev_4d6d17b9e1b09afb2790e5504711604e853f3e4e9740fbca01c397f299a00212` · file offset 299139

```text
0x8091083: DEC EAX
bytes: 48
```

## A035

vm_freeFunctionValue · `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` · file offset 278884

```text
0x808c164: DEC dword ptr [EBX]
bytes: ff0b
```

## A036

vm_freeFunctionValue · `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` · file offset 278887

```text
0x808c167: CALL 0x0808ad98
bytes: e82cecffff
```

## A037

vm_freeFunctionValue · `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` · file offset 278983

```text
0x808c1c7: CALL 0x080ceb94
bytes: e8c8290400
```

## A038

vm_freeFunctionValue · `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` · file offset 279006

```text
0x808c1de: CALL 0x080ceb94
bytes: e8b1290400
```

## A039

vm_freeFunctionValue · `ev_228d09be9663f9f70d03a59ebe1663960c6d34458f9a82d3d5e1e4cee42d44f9` · file offset 279023

```text
0x808c1ef: CALL 0x080b1618
bytes: e824540200
```

## A040

vm_convertToValue · `ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956` · file offset 278597

```text
0x808c045: CALL 0x0808be24
bytes: e8dafdffff
```

## A041

vm_convertToValue · `ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956` · file offset 278602

```text
0x808c04a: MOV EAX, dword ptr [EBX]
bytes: 8b03
```

## A042

vm_convertToValue · `ev_9d3740c765c17665b432d16e97139a12b824b1e31238f5fb1f421a36d5f5e956` · file offset 278622

```text
0x808c05e: CALL 0x0808ad98
bytes: e835edffff
```

## A043

vm_queryMember · `ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459` · file offset 293951

```text
0x808fc3f: CALL 0x0808be24
bytes: e8e0c1ffff
```

## A044

vm_queryMember · `ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459` · file offset 293959

```text
0x808fc47: CALL 0x0809b350
bytes: e804b70000
```

## A045

vm_queryMember · `ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459` · file offset 294051

```text
0x808fca3: CALL 0x0808be24
bytes: e87cc1ffff
```

## A046

vm_queryMember · `ev_ec2e63f30a5e17be4bc7568c971fc949e924bd051c8f8f5049d7793945b7c459` · file offset 294059

```text
0x808fcab: CALL 0x0809c488
bytes: e8d8c70000
```

## A047

vm_unrefArray · `ev_8000dac56416a0239afddc630d10d5c425e7d9797f623755e77dcda7e5516c26` · file offset 340883

```text
0x809b393: CALL 0x0808bfb4
bytes: e81c0cffff
```

## A048

vm_unrefValue · `ev_18fea6ebc3680cd0ea4726c67901206fe1299bda13ef18741e124aa8a6eb77de` · file offset 278462

```text
0x808bfbe: DEC EAX
bytes: 48
```

## A049

vm_unrefValue · `ev_18fea6ebc3680cd0ea4726c67901206fe1299bda13ef18741e124aa8a6eb77de` · file offset 278467

```text
0x808bfc3: JNZ 0x0808c008
bytes: 7543
```

## A050

vm_unrefValue · `ev_18fea6ebc3680cd0ea4726c67901206fe1299bda13ef18741e124aa8a6eb77de` · file offset 278477

```text
0x808bfcd: CALL 0x0808ad98
bytes: e8c6edffff
```

## A051

vm_unrefMapping · `ev_2aae5aa932757dd2dad4354599e352e3310dbf9d1f7637b49903392349474966` · file offset 345243

```text
0x809c49b: JLE 0x0809c4a5
bytes: 7e08
```

## A052

vm_unrefMapping · `ev_2aae5aa932757dd2dad4354599e352e3310dbf9d1f7637b49903392349474966` · file offset 345310

```text
0x809c4de: CALL 0x0809c33c
bytes: e859feffff
```

## A053

vm_freeNode · `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae` · file offset 344912

```text
0x809c350: CALL 0x0808ad98
bytes: e843eafeff
```

## A054

vm_freeNode · `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae` · file offset 344923

```text
0x809c35b: CALL 0x0808ad98
bytes: e838eafeff
```

## A055

vm_freeNode · `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae` · file offset 344961

```text
0x809c381: CALL 0x0808c0b8
bytes: e832fdfeff
```

## A056

vm_freeNode · `ev_ef0f48684b838cbc45dc532f24fa5985365542fc69b10337e082b6df80283dae` · file offset 344972

```text
0x809c38c: CALL 0x0808c0b8
bytes: e827fdfeff
```

## A057

vm_returnFromFun · `ev_6a8948bf446a008fa48fc2ecc8711a14fc41635f9cd05e4cfcdacf8bec7a6215` · file offset 314448

```text
0x8094c50: CALL 0x08092008
bytes: e8b3d3ffff
```

## A058

vm_takeOffValueFromStack · `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535` · file offset 303197

```text
0x809205d: CALL 0x0808ad98
bytes: e8368dffff
```

## A059

vm_takeOffValueFromStack · `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535` · file offset 303248

```text
0x8092090: MOV EDX, dword ptr [0x08661964]
bytes: 8b1564196608
```

## A060

vm_takeOffValueFromStack · `ev_b8168f2c625c87c5cab8326205b0560cd7db940a32c06abbbda3dcdb7a839535` · file offset 303302

```text
0x80920c6: MOV byte ptr [ECX + EAX*0x1 + 0x5], 0x0
bytes: c644010500
```

## A061

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285085

```text
0x808d99d: CMP word ptr [EDX + 0x6], 0x0
bytes: 66837a0600
```

## A062

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285096

```text
0x808d9a8: CMP EAX, 0x7
bytes: 83f807
```

## A063

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285113

```text
0x808d9b9: CMP EAX, 0x6
bytes: 83f806
```

## A064

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285161

```text
0x808d9e9: CMP EAX, 0x9
bytes: 83f809
```

## A065

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285170

```text
0x808d9f2: CMP EAX, 0x14
bytes: 83f814
```

## A066

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285349

```text
0x808daa5: CALL 0x0808d994
bytes: e8eafeffff
```

## A067

vm_callInUserFunBody · `ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c` · file offset 311107

```text
0x8093f43: MOV dword ptr [EDX], EAX
bytes: 8902
```

## A068

vm_callInUserFunBody · `ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c` · file offset 311119

```text
0x8093f4f: MOV dword ptr [EDX + 0x4], EAX
bytes: 894204
```

## A069

vm_callInUserFunBody · `ev_ea9570e30440ab119beb30143a85def63e15e87ac0485860290ba47ca3a6ae7c` · file offset 311198

```text
0x8093f9e: MOV dword ptr [EBX + 0xc], 0x0
bytes: c7430c00000000
```

## A070

vm_checkArgNum · `ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126` · file offset 316457

```text
0x8095429: CVTSI2SS XMM0, dword ptr [EDI + 0x8]
bytes: f30f2a4708
```

## A071

vm_checkArgNum · `ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126` · file offset 316543

```text
0x809547f: CVTTSS2SI EAX, dword ptr [EDI + 0x8]
bytes: f30f2c4708
```

## A072

vm_checkArgNum · `ev_aaa80e13c3c88b9a455d4ba6569e69518889b1fcc26a966c8e433f5f746c7126` · file offset 316489

```text
0x8095449: CALL 0x08097b24
bytes: e8d6260000
```

## A073

vm_error · `ev_f1c4dde0e0ce2a55aa3838ae691f272691496d87fcbc3d91ec17f09ea6eeaea1` · file offset 326436

```text
0x8097b24: PUSH EBP
bytes: 55
```

## A074

vm_errorQuit · `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` · file offset 405868

```text
0x80ab16c: CALL 0x08093520
bytes: e8af83feff
```

## A075

vm_errorQuit · `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` · file offset 405828

```text
0x80ab144: CALL 0x0804b574
bytes: e82b04faff8d7600
```

## A076

vm_traceError · `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` · file offset 309968

```text
0x8093ad0: CALL 0x08091ab0
bytes: e8dbdfffff
```

## A077

vm_traceError · `ev_e12a1e49fc37b28910632e7d22872a27b51e7c54f0c6478935e893b20f7c63fe` · file offset 310306

```text
0x8093c22: CALL 0x080b1804
bytes: e8dddb0100
```

## A078

vm_doActions · `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` · file offset 324086

```text
0x80971f6: CALL 0x08091ab0
bytes: e8b5a8ffff
```

## A079

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302580

```text
0x8091df4: TEST EAX, EAX
bytes: 85c0
```

## A080

vm_pushGlobalVar · `ev_f5d36761261bd90bb2139ae2bcd05e79af64a90eaffe2b978b3c0353a3ffe4a2` · file offset 299395

```text
0x8091183: CALL 0x0808be24
bytes: e89cacffff
```

## A081

vm_pushObjVar · `ev_9a9cde77fb2df1b8214e667458663471ea6da5fbeea24bfb3878fcb8f7548808` · file offset 299640

```text
0x8091278: CALL 0x0808be24
bytes: e8a7abffff
```

## A082

vm_unExecRefObject · `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` · file offset 432143

```text
0x80b180f: DEC EAX
bytes: 48
```

## A083

vm_unExecRefObject · `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` · file offset 432156

```text
0x80b181c: TEST byte ptr [EBX + 0xd], 0x40
bytes: f6430d40
```

## A084

vm_unExecRefObject · `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` · file offset 432179

```text
0x80b1833: OR dword ptr [EBX + 0xc], 0x8000
bytes: 814b0c00800000
```

## A085

vm_unExecRefObject · `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` · file offset 432192

```text
0x80b1840: INC dword ptr [EBX]
bytes: ff03
```
