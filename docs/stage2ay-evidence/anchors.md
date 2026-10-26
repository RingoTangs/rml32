# 2AY 机器码锚点

字节来自原 ELF；汇编来自对应完整记录，不是执行轨迹。

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

## A086

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302509

```text
0x8091dad: TEST byte ptr [EDX + 0xd], 0x80
bytes: f6420d80
```

## A087

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302532

```text
0x8091dc4: MOV dword ptr [EAX + 0xc], EDX
bytes: 89500c
```

## A088

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302802

```text
0x8091ed2: INC dword ptr [ECX]
bytes: ff01
```

## A089

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279273

```text
0x808c2e9: CALL 0x080b1618
bytes: e82a530200
```

## A090

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279050

```text
0x808c20a: DEC EAX
bytes: 48
```

## A091

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279055

```text
0x808c20f: JLE 0x0808c21c
bytes: 7e0b
```

## A092

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279137

```text
0x808c261: DEC dword ptr [EBX]
bytes: ff0b
```

## A093

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279140

```text
0x808c264: CALL 0x0808ad98
bytes: e82febffff
```

## A094

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279148

```text
0x808c26c: DEC dword ptr [0x0847fd88]
bytes: ff0d88fd4708
```

## A095

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279167

```text
0x808c27f: MOV [0x0847fd8c], EAX
bytes: a38cfd4708
```

## A096

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279229

```text
0x808c2bd: CALL 0x080ceb94
bytes: e8d2280400
```

## A097

vm_unrefFunctionValue · `ev_0934ff13603a2c84b549e5d8f003a103db71c86b87fcc5692099ac2ab81650d4` · file offset 279253

```text
0x808c2d5: CALL 0x080ceb94
bytes: e8ba280400
```

## A098

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432288

```text
0x80b18a0: MOV EDX, dword ptr [EBX + 0x4]
bytes: 8b5304
```

## A099

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432295

```text
0x80b18a7: OR dword ptr [EBX + 0xc], 0x4000
bytes: 814b0c00400000
```

## A100

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432309

```text
0x80b18b5: RET
bytes: c389f6
```

## A101

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432446

```text
0x80b193e: MOV EAX, dword ptr [EBX]
bytes: 8b03
```

## A102

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432490

```text
0x80b196a: OR dword ptr [EBX + 0xc], 0x8000
bytes: 814b0c00800000
```

## A103

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432513

```text
0x80b1981: CALL 0x0808c0b8
bytes: e832a7fdff
```

## A104

vm_destructObject · `ev_4afb7ed3129068ae579d0107c615b2790e0c898b1d1322b7349f0b98f5f7f90b` · file offset 432603

```text
0x80b19db: MOV dword ptr [EDX], EAX
bytes: 8902
```

## A105

vm_unExecRefObject · `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` · file offset 432166

```text
0x80b1826: CALL 0x080b1728
bytes: e8fdfeffff
```

## A106

vm_unExecRefObject · `ev_c852e7916c2cdc0cda41b7ecfb7e817b8345955915dab87bdbdcc65f2ff5278f` · file offset 432186

```text
0x80b183a: MOV dword ptr [0x08609a40], EBX
bytes: 891d409a6008
```

## A107

vm_freeObjectsInDestructingList · `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63` · file offset 432669

```text
0x80b1a1d: MOV [0x08609a40], EAX
bytes: a3409a6008
```

## A108

vm_freeObjectsInDestructingList · `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63` · file offset 432682

```text
0x80b1a2a: CALL 0x080b1894
bytes: e865feffff
```

## A109

vm_freeObjectsInDestructingList · `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63` · file offset 432689

```text
0x80b1a31: DEC EAX
bytes: 48
```

## A110

vm_freeObjectsInDestructingList · `ev_1e97d9b23b4f5b64595672ccc759645ed729cbbdc52e7ea0cdf0edbdf42d3c63` · file offset 432699

```text
0x80b1a3b: TEST byte ptr [EBX + 0xd], 0x80
bytes: f6430d80
```

## A111

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278154

```text
0x808be8a: TEST byte ptr [EAX + 0xd], 0x80
bytes: f6400d80
```

## A112

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278160

```text
0x808be90: INC dword ptr [EAX]
bytes: ff00
```

## A113

vm_copyValue · `ev_69a23ae044935ae6f7450097987faa71f641c62091f885fc4b4b5f8d5b1929f4` · file offset 278205

```text
0x808bebd: TEST byte ptr [EDX + 0xd], 0x80
bytes: f6420d80
```

## A114

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302569

```text
0x8091de9: CMP SI, word ptr [EBP + -0x10]
bytes: 663b75f0
```

## A115

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302582

```text
0x8091df6: JZ 0x08091ed9
bytes: 0f84dd000000
```

## A116

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302760

```text
0x8091ea8: INC EAX
bytes: 40
```

## A117

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302768

```text
0x8091eb0: MOV [0x086619cc], EAX
bytes: a3cc196608
```

## A118

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302779

```text
0x8091ebb: JMP 0x08091df4
bytes: e934ffffff
```

## A119

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302817

```text
0x8091ee1: CALL 0x08097b24
bytes: e83e5c0000
```

## A120

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 550825

```text
0x80ce7a9: XOR EAX, EAX
bytes: 31c0
```

## A121

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 550868

```text
0x80ce7d4: CMP dword ptr [ESI + 0x860d040], ECX
bytes: 398e40d06008
```

## A122

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 550880

```text
0x80ce7e0: JNC 0x080ce870
bytes: 0f838a000000
```

## A123

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551134

```text
0x80ce8de: ADD EDX, 0x4
bytes: 83c204
```

## A124

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551138

```text
0x80ce8e2: CALL 0x0804b354
bytes: e86dcaf7ff
```

## A125

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551094

```text
0x80ce8b6: TEST EBX, EBX
bytes: 85db
```

## A126

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551108

```text
0x80ce8c4: INC dword ptr [0x0860d01c]
bytes: ff051cd06008
```

## A127

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551120

```text
0x80ce8d0: LEA EAX, [EBX + 0xc]
bytes: 8d430c
```

## A128

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407222

```text
0x80ab6b6: CALL 0x0808ad98
bytes: e8ddf6fdff
```

## A129

vm_errorQuit · `ev_bfa094c0c50a60bb231088da09294746c5850d3124a018787a398173fdb3a0c2` · file offset 405789

```text
0x80ab11d: CMP dword ptr [EBX], 0x1
bytes: 833b01
```

## A130

vm_traceError · `ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1` · file offset 309968

```text
0x8093ad0: CALL 0x08091ab0
bytes: e8dbdfffff
```

## A131

vm_traceError · `ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1` · file offset 310017

```text
0x8093b01: DEC dword ptr [0x086619c0]
bytes: ff0dc0196608
```

## A132

vm_traceError · `ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1` · file offset 310036

```text
0x8093b14: TEST AL, 0x1
bytes: a801
```

## A133

vm_traceError · `ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1` · file offset 310083

```text
0x8093b43: MOV [0x086619c0], EAX
bytes: a3c0196608
```

## A134

vm_traceError · `ev_2ca52477b7ba7dcc40e48827fff306b892f626641777eace6893b1f5acc15ba1` · file offset 310306

```text
0x8093c22: CALL 0x080b1804
bytes: e8dddb0100
```

## A135

vm_doActions · `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` · file offset 323692

```text
0x809706c: CALL 0x080aae68
bytes: e8f73d0100
```

## A136

vm_doActions · `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` · file offset 323701

```text
0x8097075: CALL 0x0804bac4
bytes: e84a4afbff
```

## A137

vm_doActions · `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` · file offset 323717

```text
0x8097085: CMP EAX, -0x3
bytes: 83f8fd
```

## A138

vm_doActions · `ev_543af7cfb6d454bc19165fe254e84721ac87776215bd08299342ec2ab9a32d9e` · file offset 323740

```text
0x809709c: JLE 0x080971e8
bytes: 0f8e46010000
```

## A139

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 415363

```text
0x80ad683: CALL 0x08097008
bytes: e88099feff
```

## A140

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 415370

```text
0x80ad68a: CALL 0x08096f44
bytes: e8b598feff
```

## A141

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 414840

```text
0x80ad478: MOV dword ptr [EBX + 0x78], EAX
bytes: 894378
```

## A142

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 414848

```text
0x80ad480: MOV dword ptr [EBX + 0x8c], EAX
bytes: 89838c000000
```

## A143

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 415001

```text
0x80ad519: CALL 0x080ab768
bytes: e84ae2ffff
```

## A144

vm_scheduleProcess · `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c` · file offset 416142

```text
0x80ad98e: MOV dword ptr [0x08607900], 0x1
bytes: c7050079600801000000
```

## A145

vm_scheduleProcess · `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c` · file offset 416152

```text
0x80ad998: MOV dword ptr [0x086078e8], 0x1
bytes: c705e878600801000000
```

## A146

vm_scheduleProcess · `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c` · file offset 416162

```text
0x80ad9a2: CALL 0x080ad1d4
bytes: e82df8ffff
```

## A147

vm_scheduleProcess · `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c` · file offset 416167

```text
0x80ad9a7: MOV dword ptr [0x086078e8], 0x0
bytes: c705e878600800000000
```

## A148

vm_scheduleProcess · `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c` · file offset 416177

```text
0x80ad9b1: MOV dword ptr [0x08607900], 0x0
bytes: c7050079600800000000
```

## A149

vm_endForBackend · `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a` · file offset 323402

```text
0x8096f4a: CALL 0x080b1a04
bytes: e8b5aa0100
```

## A150

vm_endForBackend · `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a` · file offset 323407

```text
0x8096f4f: CALL 0x080c9f70
bytes: e81c300300
```

## A151

vm_endForBackend · `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a` · file offset 323412

```text
0x8096f54: CALL 0x0809f24c
bytes: e8f3820000
```

## A152

vm_endForBackend · `ev_f8aa49f1e43d8d933ec0710848b2cbbf20effdaafc21a17e2adb1ee0cd0cb73a` · file offset 323422

```text
0x8096f5e: CALL 0x080a391c
bytes: e8b9c90000
```

## A153

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407420

```text
0x80ab77c: OR EAX, 0x8
bytes: 83c808
```

## A154

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407554

```text
0x80ab802: MOV EAX, dword ptr [EDX + 0x30]
bytes: 8b4230
```

## A155

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407587

```text
0x80ab823: CALL EAX
bytes: ffd0
```

## A156

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407622

```text
0x80ab846: MOV dword ptr [EDI + 0xac], EDX
bytes: 8997ac000000
```

## A157

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407644

```text
0x80ab85c: CALL 0x080ceb94
bytes: e833330200
```

## A158

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407653

```text
0x80ab865: CALL ESI
bytes: ffd6
```

## A159

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 408080

```text
0x80aba10: CALL EAX
bytes: ffd0
```

## A160

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407775

```text
0x80ab8df: DEC EAX
bytes: 48
```

## A161

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407784

```text
0x80ab8e8: JNZ 0x080ab974
bytes: 0f8586000000
```

## A162

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 408125

```text
0x80aba3d: CALL 0x0808c1fc
bytes: e8ba07feff
```

## A163

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407831

```text
0x80ab917: MOV dword ptr [EDI + 0x78], EAX
bytes: 894778
```

## A164

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407834

```text
0x80ab91a: CALL 0x0808ad98
bytes: e879f4fdff
```

## A165

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407872

```text
0x80ab940: MOV dword ptr [EDI + 0x8c], ECX
bytes: 898f8c000000
```

## A166

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407884

```text
0x80ab94c: TEST AL, 0x1
bytes: a801
```

## A167

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407888

```text
0x80ab950: TEST AL, 0x10
bytes: a810
```

## A168

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407943

```text
0x80ab987: CALL 0x080b1804
bytes: e8785e0000
```

## A169

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 407901

```text
0x80ab95d: AND dword ptr [EDI + 0x28], 0xfffffffe
bytes: 836728fe
```

## A170

vm_deleteProcess · `ev_fd6ebcdf65115a69b9a379778148a4308b1418aee68693aba154a7ce9474feab` · file offset 408107

```text
0x80aba2b: MOV dword ptr [0x086078d0], EDI
bytes: 893dd0786008
```

## A171

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407163

```text
0x80ab67b: DEC EAX
bytes: 48
```

## A172

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407169

```text
0x80ab681: JNZ 0x080ab70c
bytes: 0f8585000000
```

## A173

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407349

```text
0x80ab735: CALL 0x0808c1fc
bytes: e8c20afeff
```

## A174

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407219

```text
0x80ab6b3: MOV dword ptr [EBX + 0x78], EAX
bytes: 894378
```

## A175

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407260

```text
0x80ab6dc: MOV dword ptr [EBX + 0x8c], ECX
bytes: 898b8c000000
```

## A176

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407327

```text
0x80ab71f: CALL 0x080b1804
bytes: e8e0600000
```

## A177

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407289

```text
0x80ab6f9: AND dword ptr [EBX + 0x28], 0xfffffffe
bytes: 836328fe
```

## A178

vm_unrefToPCB · `ev_06ea52a7afd2f826495b7a135f97a5909a8c9086ddb7d557e6dacb5dc28c5748` · file offset 407389

```text
0x80ab75d: MOV dword ptr [0x086078d0], EBX
bytes: 891dd0786008
```

## A179

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285322

```text
0x808da8a: MOV EBX, dword ptr [EDX + 0x8]
bytes: 8b5a08
```

## A180

vm_markActiveFromValue · `ev_d6bd8847412c335d9425d53073743ca82d6469ed6c69a789f996bbc7c2f39471` · file offset 285369

```text
0x808dab9: JMP 0x0808d9e1
bytes: e923ffffff
```

## A181

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 405892

```text
0x80ab184: TEST ESI, ESI
bytes: 85f6
```

## A182

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 405971

```text
0x80ab1d3: MOV dword ptr [EDX], 0x100
bytes: c70200010000
```

## A183

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406804

```text
0x80ab514: CMP EDX, 0x10000
bytes: 81fa00000100
```

## A184

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406824

```text
0x80ab528: MOV dword ptr [ESI], 0x10000
bytes: c70600000100
```

## A185

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406160

```text
0x80ab290: CALL 0x08089f10
bytes: e87becfdff
```

## A186

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406395

```text
0x80ab37b: MOV dword ptr [EBX + 0x78], EAX
bytes: 894378
```

## A187

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406398

```text
0x80ab37e: MOV dword ptr [EBX + 0x8c], 0x0
bytes: c7838c00000000000000
```

## A188

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406415

```text
0x80ab38f: MOV dword ptr [EBX + 0x30], 0x1
bytes: c7433001000000
```

## A189

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406547

```text
0x80ab413: STOSD.REP ES:EDI
bytes: f3ab
```

## A190

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406558

```text
0x80ab41e: MOV dword ptr [EDX], EAX
bytes: 8902
```

## A191

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406578

```text
0x80ab432: MOV dword ptr [EDX + 0xc], 0x0
bytes: c7420c00000000
```

## A192

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406585

```text
0x80ab439: MOV dword ptr [EDX + 0x8], 0x2
bytes: c7420802000000
```

## A193

vm_initProcess · `ev_6d29a100bf81351c9a23e7aafccf7e7b686400d183819f4c456e3c03a886c6ef` · file offset 406688

```text
0x80ab4a0: MOV dword ptr [EBX + 0x98], 0x0
bytes: c7839800000000000000
```

## A194

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325944

```text
0x8097938: JNZ 0x08097a0c
bytes: 0f85ce000000
```

## A195

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325972

```text
0x8097954: MOVZX EDX, byte ptr [EBX + 0x3]
bytes: 0fb65303
```

## A196

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325985

```text
0x8097961: LEA EDX, [EAX*0x4 + 0x83cc680]
bytes: 8d148580c63c08
```

## A197

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325992

```text
0x8097968: MOV EAX, dword ptr [EDX + 0x18]
bytes: 8b4218
```

## A198

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326003

```text
0x8097973: MOV AL, byte ptr [EDX + 0xc]
bytes: 8a420c
```

## A199

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326022

```text
0x8097986: MOVSX EAX, byte ptr [EBX + 0x2]
bytes: 0fbe4302
```

## A200

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326029

```text
0x809798d: PUSH EAX
bytes: 50
```

## A201

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326030

```text
0x809798e: MOVSX EAX, word ptr [EBX]
bytes: 0fbf03
```

## A202

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326034

```text
0x8097992: CALL dword ptr [EDX + 0x8]
bytes: ff5208
```

## A203

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326243

```text
0x8097a63: MOVSX EAX, word ptr [EBX]
bytes: 0fbf03
```

## A204

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 326247

```text
0x8097a67: JMP 0x08097992
bytes: e926ffffff
```

## A205

vm_createFunctionValue · `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444` · file offset 302922

```text
0x8091f4a: SHR ECX, 0x7
bytes: c1e907
```

## A206

vm_createFunctionValue · `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444` · file offset 302938

```text
0x8091f5a: AND EDI, 0x3f
bytes: 83e73f
```

## A207

vm_createFunctionValue · `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444` · file offset 302941

```text
0x8091f5d: AND ECX, 0x1
bytes: 83e101
```

## A208

vm_createFunctionValue · `ev_4ae635941eda7abcc9024ccadd56fd4d6278ddcd6075acacda6b865390f65444` · file offset 303014

```text
0x8091fa6: CALL 0x08091cd0
bytes: e825fdffff
```

## A209

vm_pushShortNumber · `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a` · file offset 297704

```text
0x8090ae8: PUSH EBP
bytes: 55
```

## A210

vm_pushShortNumber · `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a` · file offset 297717

```text
0x8090af5: JZ 0x08090b24
bytes: 742d
```

## A211

vm_pushShortNumber · `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a` · file offset 297719

```text
0x8090af7: DEC EAX
bytes: 48
```

## A212

vm_pushShortNumber · `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a` · file offset 297720

```text
0x8090af8: MOV [0x086619cc], EAX
bytes: a3cc196608
```

## A213

vm_pushShortNumber · `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a` · file offset 297734

```text
0x8090b06: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x2
bytes: c644c20402
```

## A214

vm_pushShortNumber · `ev_435c34c38da3d259692fcf6ea5ca1336fb90f91e53e999c32ea0cb40d27d806a` · file offset 297756

```text
0x8090b1c: MOV dword ptr [ECX + EAX*0x8 + 0x8], EDX
bytes: 8954c108
```

## A215

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325837

```text
0x80978cd: MOV EDI, 0x800
bytes: bf00080000
```

## A216

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325924

```text
0x8097924: DEC EDI
bytes: 4f
```

## A217

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325927

```text
0x8097927: JLE 0x08097a6c
bytes: 0f8e3f010000
```

## A218

vm_regularProcessor · `ev_be433ed21232d17e29f35f068109d8dd00bd74149a9fd91468d8680cabda6fe3` · file offset 325969

```text
0x8097951: MOV dword ptr [ECX + 0xc], EAX
bytes: 89410c
```

## A219

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302407

```text
0x8091d47: MOV dword ptr [EBP + -0x14], EAX
bytes: 8945ec
```

## A220

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302495

```text
0x8091d9f: MOV dword ptr [ECX + 0x10], EDI
bytes: 897910
```

## A221

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302603

```text
0x8091e0b: MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x9
bytes: c644c20409
```

## A222

vm_createFunctionValueEx · `ev_f739f81d73d7c07779ab3fb741800726145efbc40fd8dc6c58009ac2d703aa48` · file offset 302625

```text
0x8091e21: MOV dword ptr [EDX + EAX*0x8 + 0x8], ECX
bytes: 894cc208
```

## A223

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550493

```text
0x80ce65d: CALL 0x0804b354
bytes: e8f2ccf7ff
```

## A224

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550511

```text
0x80ce66f: MOV dword ptr [EDX + 0x4], EBX
bytes: 895a04
```

## A225

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550522

```text
0x80ce67a: MOV dword ptr [EDX], EAX
bytes: 8902
```

## A226

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550529

```text
0x80ce681: MOV dword ptr [EDX + 0x8], EAX
bytes: 894208
```

## A227

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550543

```text
0x80ce68f: MOV dword ptr [0x0860d024], EDX
bytes: 891524d06008
```

## A228

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550617

```text
0x80ce6d9: MOV word ptr [ECX + 0x6], AX
bytes: 66894106
```

## A229

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550634

```text
0x80ce6ea: MOV dword ptr [ECX + 0x8], EAX
bytes: 894108
```

## A230

pdb_extendBlocksOfGroup · `ev_69e5d889c7b964db3c123bbbc5f6c7a4512b0633b9d910e1a3e131018281b99d` · file offset 550638

```text
0x80ce6ee: MOV dword ptr [EDI + 0x14], ECX
bytes: 894f14
```

## A231

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551045

```text
0x80ce885: MOV dword ptr [ESI + 0x860d054], EAX
bytes: 898654d06008
```

## A232

pdb_allocateMemory · `ev_f11621ec8278f50d7b95cd012344996567569c2ad1f327a38f5d037ee8cc232c` · file offset 551071

```text
0x80ce89f: OR word ptr [EBX + 0x6], 0x10
bytes: 66834b0610
```

## A233

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 551940

```text
0x80cec04: LEA ECX, [EBX + -0xc]
bytes: 8d4bf4
```

## A234

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 552004

```text
0x80cec44: TEST AL, 0x10
bytes: a810
```

## A235

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 552024

```text
0x80cec58: INC dword ptr [0x0860d018]
bytes: ff0518d06008
```

## A236

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 552044

```text
0x80cec6c: MOV word ptr [ECX + 0x6], DX
bytes: 66895106
```

## A237

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 552084

```text
0x80cec94: MOV dword ptr [ECX + 0x8], EAX
bytes: 894108
```

## A238

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 552087

```text
0x80cec97: MOV dword ptr [EDX + 0x14], ECX
bytes: 894a14
```

## A239

pdb_freeMemory · `ev_d7cd867b7d8979108b472049c77c2ed0272efc48c431ff4071039afb7b2e15ab` · file offset 552242

```text
0x80ced32: CALL 0x080cea34
bytes: e8fdfcffff
```

## A240

pdb_l2FreeMemory · `ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a` · file offset 551564

```text
0x80cea8c: TEST AL, 0xa
bytes: a80a
```

## A241

pdb_l2FreeMemory · `ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a` · file offset 551608

```text
0x80ceab8: MOV dword ptr [ECX + 0x8], EAX
bytes: 894108
```

## A242

pdb_l2FreeMemory · `ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a` · file offset 551611

```text
0x80ceabb: MOV dword ptr [EDX + 0x14], ECX
bytes: 894a14
```

## A243

pdb_l2FreeMemory · `ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a` · file offset 551746

```text
0x80ceb42: SUB dword ptr [0x0860d00c], EDX
bytes: 29150cd06008
```

## A244

pdb_l2FreeMemory · `ev_d6e6b77318b57369520badc1b7a8bfebb3e43f1702000111099db0a94299e06a` · file offset 551753

```text
0x80ceb49: CALL 0x0804ba14
bytes: e8c6cef7ff
```

## A245

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 415012

```text
0x80ad524: DEC EDI
bytes: 4f
```

## A246

vm_scheduleProcessInternal · `ev_8afa124f5ab414820201215e5832365dfc1e5fd8f99de938704714514f785fe3` · file offset 415024

```text
0x80ad530: JNZ 0x080ad374
bytes: 0f853efeffff
```

## A247

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486004

```text
0x80bea74: CALL 0x080bf194
bytes: e81b070000
```

## A248

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486014

```text
0x80bea7e: JZ 0x080beac0
bytes: 7440
```

## A249

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486080

```text
0x80beac0: LEA ESP, [EBP + -0xc]
bytes: 8d65f4
```

## A250

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486087

```text
0x80beac7: RET
bytes: c3
```

## A251

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486094

```text
0x80beace: CALL EAX
bytes: ffd0
```

## A252

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486033

```text
0x80bea91: CALL 0x080b5bec
bytes: e85671ffff
```

## A253

vm_processAllRespondingInQueue · `ev_6e8d3bdf322cb3574f900949a394c797df34a67db5831a7a96393c9b30719a72` · file offset 486046

```text
0x80bea9e: CALL ESI
bytes: ffd6
```

## A254

vm_scheduleProcess · `ev_742151cf0a21b4697cbd895f9c6e4ae778ca4bb7c9c6a3a782fe4e7c77385d9c` · file offset 416188

```text
0x80ad9bc: RET
bytes: c3
```

## A255

vm_mainLoop · `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` · file offset 417569

```text
0x80adf21: CALL 0x080ad1d4
bytes: e8aef2ffff
```

## A256

vm_mainLoop · `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` · file offset 417574

```text
0x80adf26: CMP EAX, -0x63
bytes: 83f89d
```

## A257

vm_mainLoop · `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` · file offset 417805

```text
0x80ae00d: JNZ 0x080ae0be
bytes: 0f85ab000000
```

## A258

vm_mainLoop · `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` · file offset 417867

```text
0x80ae04b: MOV EAX, [0x08607984]
bytes: a184796008
```

## A259

vm_mainLoop · `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` · file offset 417880

```text
0x80ae058: SUB ESP, 0xc
bytes: 83ec0c
```

## A260

vm_mainLoop · `ev_daa1af7157f27407ab1c0f09a06c0a82e1245714d25bec4d5d7440c512ee7e6b` · file offset 417982

```text
0x80ae0be: LEA ESP, [EBP + -0xc]
bytes: 8d65f4
```

## A261

vm_clearEnvironment · `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29` · file offset 412516

```text
0x80acb64: CALL 0x080ab768
bytes: e8ffebffff
```

## A262

vm_clearEnvironment · `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29` · file offset 412536

```text
0x80acb78: JC 0x080acaf5
bytes: 0f8277ffffff
```

## A263

vm_clearEnvironment · `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29` · file offset 412542

```text
0x80acb7e: CALL 0x080b1a04
bytes: e8814e0000
```

## A264

vm_clearEnvironment · `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29` · file offset 412547

```text
0x80acb83: CALL 0x0808697c
bytes: e8f49dfdff
```

## A265

vm_clearEnvironment · `ev_c8ed1b0d33d59a3c3e7dd9afa1d7cdba9b5e21d7e042d4b49dcac0ae8fc24a29` · file offset 412557

```text
0x80acb8d: CALL 0x080b1ab4
bytes: e8224f0000
```

## A266

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418186

```text
0x80ae18a: CALL 0x080ad1d4
bytes: e845f0ffff
```

## A267

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418454

```text
0x80ae296: CALL 0x080ab768
bytes: e8cdd4ffff
```

## A268

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418474

```text
0x80ae2aa: JC 0x080ae227
bytes: 0f8277ffffff
```

## A269

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418480

```text
0x80ae2b0: CALL 0x080b1a04
bytes: e84f370000
```

## A270

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418522

```text
0x80ae2da: MOV EAX, dword ptr [EBX + 0x7c]
bytes: 8b437c
```

## A271

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418557

```text
0x80ae2fd: CALL 0x0808c0b8
bytes: e8b6ddfdff
```

## A272

vm_shutdownScheduler · `ev_fa72d6497d785e96f2008f5e0c88d23b56be0c6334a7128afc2e2ee91b59b9d5` · file offset 418539

```text
0x80ae2eb: MOV byte ptr [ESI + EAX*0x1 + 0x4], 0x0
bytes: c644060400
```

## A273

vm_cleanupValue · `ev_d5e0fa5ed30b20859a959e09027480b13f6d9acdd746527dd5c2dac47520cadc` · file offset 273828

```text
0x808ada4: MOVZX EAX, byte ptr [EDI + 0x4]
bytes: 0fb64704
```
