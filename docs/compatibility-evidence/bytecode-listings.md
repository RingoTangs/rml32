# 包内字节码证据

由静态加载器格式解码，未执行脚本。偏移均为解压后成员内位置，不可当作 ELF 地址或压缩包内指令偏移。efun 名称由固定注册顺序恢复；调用指针及跨对象调用保留为未解析，不从附近字符串自动证明目标。


<a id="aaa-__init__"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: __init__

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x6d8a8`；函数编号：`init`。

```text
0000 @0x0006de58 push_number(S) [1024] 
0001 @0x0006de5c call_in_efun [0, 1] allocate_array
0002 @0x0006de60 push_global_vptr [2] 
0003 @0x0006de64 assign_value [] 
0004 @0x0006de68 pop_stack [] 
0005 @0x0006de6c push_number(S) [2] 
0006 @0x0006de70 push_global_vptr [3] 
0007 @0x0006de74 assign_value [] 
0008 @0x0006de78 pop_stack [] 
0009 @0x0006de7c push_number(S) [0] 
0010 @0x0006de80 push_global_vptr [4] 
0011 @0x0006de84 assign_value [] 
0012 @0x0006de88 pop_stack [] 
0013 @0x0006de8c push_shared_string [574] '/xdbc/'
0014 @0x0006de90 push_global_vptr [5] 
0015 @0x0006de94 assign_value [] 
0016 @0x0006de98 pop_stack [] 
0017 @0x0006de9c create_mapping [0] 
0018 @0x0006dea0 push_global_vptr [6] 
0019 @0x0006dea4 assign_value [] 
0020 @0x0006dea8 pop_stack [] 
0021 @0x0006deac push_number(S) [0] 
0022 @0x0006deb0 push_global_vptr [7] 
0023 @0x0006deb4 assign_value [] 
0024 @0x0006deb8 pop_stack [] 
0025 @0x0006debc push_shared_string [154] '/aaa/log/'
0026 @0x0006dec0 call_in_efun [336, 1] mkdir
0027 @0x0006dec4 pop_stack [] 
0028 @0x0006dec8 push_shared_string [575] '/aaa/log/xdbc_trace'
0029 @0x0006decc push_shared_string [56] 'trace'
0030 @0x0006ded0 call_in_user_fun [583, -126] xdbc_set_output_basic_file
0031 @0x0006ded4 pop_stack [] 
0032 @0x0006ded8 call_in_user_fun [585, -128] xdbc_enable_log_trace
0033 @0x0006dedc pop_stack [] 
0034 @0x0006dee0 push_number(S) [0] 
0035 @0x0006dee4 push_shared_string [576] 'xdbc'
0036 @0x0006dee8 call_in_efun [27, 2] enable_trace
0037 @0x0006deec pop_stack [] 
0038 @0x0006def0 create_mapping [0] 
0039 @0x0006def4 push_global_vptr [8] 
0040 @0x0006def8 assign_value [] 
0041 @0x0006defc pop_stack [] 
0042 @0x0006df00 push_shared_string [577] 'xdbc_connect'
0043 @0x0006df04 push_shared_string [65] 'mysql_connect'
0044 @0x0006df08 push_shared_string [578] 'xdbc_disconnect'
0045 @0x0006df0c push_shared_string [66] 'mysql_disconnect'
0046 @0x0006df10 push_shared_string [579] 'xdbc_is_connected'
0047 @0x0006df14 push_shared_string [67] 'mysql_is_connected'
0048 @0x0006df18 push_shared_string [580] 'xdbc_get_error'
0049 @0x0006df1c push_shared_string [68] 'mysql_get_error'
0050 @0x0006df20 push_shared_string [581] 'xdbc_execute'
0051 @0x0006df24 push_shared_string [69] 'mysql_execute'
0052 @0x0006df28 push_shared_string [582] 'xdbc_query'
0053 @0x0006df2c push_shared_string [70] 'mysql_query'
0054 @0x0006df30 push_shared_string [583] 'xdbc_finalize'
0055 @0x0006df34 push_shared_string [71] 'mysql_finalize'
0056 @0x0006df38 push_shared_string [584] 'xdbc_num_records'
0057 @0x0006df3c push_shared_string [72] 'mysql_num_records'
0058 @0x0006df40 push_shared_string [585] 'xdbc_fetch_record'
0059 @0x0006df44 push_shared_string [73] 'mysql_fetch_record'
0060 @0x0006df48 push_shared_string [586] 'xdbc_this_record'
0061 @0x0006df4c push_shared_string [74] 'mysql_this_record'
0062 @0x0006df50 push_shared_string [587] 'xdbc_field_value'
0063 @0x0006df54 push_shared_string [75] 'mysql_field_value'
0064 @0x0006df58 push_shared_string [588] 'xdbc_field_name'
0065 @0x0006df5c push_shared_string [78] 'mysql_field_name'
0066 @0x0006df60 push_shared_string [589] 'xdbc_get_insert_id'
0067 @0x0006df64 push_shared_string [81] 'mysql_get_insert_id'
0068 @0x0006df68 push_shared_string [590] 'NO_IMPLEMENTED'
0069 @0x0006df6c push_shared_string [76] 'mysql_set_auto_commit'
0070 @0x0006df70 push_shared_string [590] 'NO_IMPLEMENTED'
0071 @0x0006df74 push_shared_string [77] 'mysql_select_db'
0072 @0x0006df78 push_shared_string [590] 'NO_IMPLEMENTED'
0073 @0x0006df7c push_shared_string [79] 'mysql_num_fields'
0074 @0x0006df80 push_shared_string [590] 'NO_IMPLEMENTED'
0075 @0x0006df84 push_shared_string [80] 'mysql_int_field'
0076 @0x0006df88 create_mapping [34] 
0077 @0x0006df8c push_global_vptr [9] 
0078 @0x0006df90 assign_value [] 
0079 @0x0006df94 pop_stack [] 
0080 @0x0006df98 call_in_user_fun [604, -128] init_xdbc_client
0081 @0x0006df9c pop_stack [] 
0082 @0x0006dfa0 push_shared_string [591] 'CONFIG'
0083 @0x0006dfa4 call_in_efun [22, 1] expand_macro
0084 @0x0006dfa8 if [3, 2] 
0085 @0x0006dfac push_shared_string [591] 'CONFIG'
0086 @0x0006dfb0 call_in_efun [22, 1] expand_macro
0087 @0x0006dfb4 goto [1] 
0088 @0x0006dfb8 push_shared_string [592] '/aaa/aaa.ini'
0089 @0x0006dfbc push_global_vptr [27] 
0090 @0x0006dfc0 assign_value [] 
0091 @0x0006dfc4 pop_stack [] 
0092 @0x0006dfc8 create_mapping [0] 
0093 @0x0006dfcc push_global_vptr [28] 
0094 @0x0006dfd0 assign_value [] 
0095 @0x0006dfd4 pop_stack [] 
0096 @0x0006dfd8 create_mapping [0] 
0097 @0x0006dfdc push_global_vptr [29] 
0098 @0x0006dfe0 assign_value [] 
0099 @0x0006dfe4 pop_stack [] 
0100 @0x0006dfe8 push_shared_string [593] '__BIN_DIR__'
0101 @0x0006dfec call_in_efun [22, 1] expand_macro
0102 @0x0006dff0 call_in_efun [141, 1] trim
0103 @0x0006dff4 push_global_vptr [30] 
0104 @0x0006dff8 assign_value [] 
0105 @0x0006dffc pop_stack [] 
0106 @0x0006e000 push_global_var [30] 
0107 @0x0006e004 push_number(S) [1] 
0108 @0x0006e008 push_number(S) [2] 
0109 @0x0006e00c query_range [1] 
0110 @0x0006e010 push_global_vptr [30] 
0111 @0x0006e014 assign_value [] 
0112 @0x0006e018 pop_stack [] 
0113 @0x0006e01c push_global_var [30] 
0114 @0x0006e020 push_number(S) [1] 
0115 @0x0006e024 query_member [1] 
0116 @0x0006e028 push_number(S) [47] 
0117 @0x0006e02c eq_value [] 
0118 @0x0006e030 if [7, 2] 
0119 @0x0006e034 push_global_var [30] 
0120 @0x0006e038 push_number(S) [0] 
0121 @0x0006e03c push_number(S) [2] 
0122 @0x0006e040 query_range [1] 
0123 @0x0006e044 push_global_vptr [30] 
0124 @0x0006e048 assign_value [] 
0125 @0x0006e04c pop_stack [] 
0126 @0x0006e050 create_array [0] 
0127 @0x0006e054 push_global_vptr [31] 
0128 @0x0006e058 assign_value [] 
0129 @0x0006e05c pop_stack [] 
0130 @0x0006e060 create_array [0] 
0131 @0x0006e064 push_global_vptr [32] 
0132 @0x0006e068 assign_value [] 
0133 @0x0006e06c pop_stack [] 
0134 @0x0006e070 create_array [0] 
0135 @0x0006e074 push_global_vptr [33] 
0136 @0x0006e078 assign_value [] 
0137 @0x0006e07c pop_stack [] 
0138 @0x0006e080 create_array [0] 
0139 @0x0006e084 push_global_vptr [34] 
0140 @0x0006e088 assign_value [] 
0141 @0x0006e08c pop_stack [] 
0142 @0x0006e090 create_array [0] 
0143 @0x0006e094 push_global_vptr [35] 
0144 @0x0006e098 assign_value [] 
0145 @0x0006e09c pop_stack [] 
0146 @0x0006e0a0 create_array [0] 
0147 @0x0006e0a4 push_global_vptr [36] 
0148 @0x0006e0a8 assign_value [] 
0149 @0x0006e0ac pop_stack [] 
0150 @0x0006e0b0 create_array [0] 
0151 @0x0006e0b4 push_global_vptr [37] 
0152 @0x0006e0b8 assign_value [] 
0153 @0x0006e0bc pop_stack [] 
0154 @0x0006e0c0 create_array [0] 
0155 @0x0006e0c4 push_global_vptr [38] 
0156 @0x0006e0c8 assign_value [] 
0157 @0x0006e0cc pop_stack [] 
0158 @0x0006e0d0 create_array [0] 
0159 @0x0006e0d4 push_global_vptr [39] 
0160 @0x0006e0d8 assign_value [] 
0161 @0x0006e0dc pop_stack [] 
0162 @0x0006e0e0 create_array [0] 
0163 @0x0006e0e4 push_global_vptr [40] 
0164 @0x0006e0e8 assign_value [] 
0165 @0x0006e0ec pop_stack [] 
0166 @0x0006e0f0 create_array [0] 
0167 @0x0006e0f4 push_global_vptr [41] 
0168 @0x0006e0f8 assign_value [] 
0169 @0x0006e0fc pop_stack [] 
0170 @0x0006e100 create_array [0] 
0171 @0x0006e104 push_global_vptr [42] 
0172 @0x0006e108 assign_value [] 
0173 @0x0006e10c pop_stack [] 
0174 @0x0006e110 create_mapping [0] 
0175 @0x0006e114 push_global_vptr [43] 
0176 @0x0006e118 assign_value [] 
0177 @0x0006e11c pop_stack [] 
0178 @0x0006e120 create_array [0] 
0179 @0x0006e124 push_global_vptr [44] 
0180 @0x0006e128 assign_value [] 
0181 @0x0006e12c pop_stack [] 
0182 @0x0006e130 create_array [0] 
0183 @0x0006e134 push_global_vptr [45] 
0184 @0x0006e138 assign_value [] 
0185 @0x0006e13c pop_stack [] 
0186 @0x0006e140 create_array [0] 
0187 @0x0006e144 push_global_vptr [46] 
0188 @0x0006e148 assign_value [] 
0189 @0x0006e14c pop_stack [] 
0190 @0x0006e150 create_mapping [0] 
0191 @0x0006e154 push_global_vptr [47] 
0192 @0x0006e158 assign_value [] 
0193 @0x0006e15c pop_stack [] 
0194 @0x0006e160 push_number(S) [-1] 
0195 @0x0006e164 push_global_vptr [48] 
0196 @0x0006e168 assign_value [] 
0197 @0x0006e16c pop_stack [] 
0198 @0x0006e170 push_number(S) [0] 
0199 @0x0006e174 push_global_vptr [49] 
0200 @0x0006e178 assign_value [] 
0201 @0x0006e17c pop_stack [] 
0202 @0x0006e180 push_shared_string [160] 'update_today_zero_time'
0203 @0x0006e184 push_number(S) [0] 
0204 @0x0006e188 push_number(S) [0] 
0205 @0x0006e18c call_in_efun [327, 3] set_timer
0206 @0x0006e190 pop_stack [] 
0207 @0x0006e194 push_number(S) [45] 
0208 @0x0006e198 push_global_vptr [50] 
0209 @0x0006e19c assign_value [] 
0210 @0x0006e1a0 pop_stack [] 
0211 @0x0006e1a4 push_number(S) [-1] 
0212 @0x0006e1a8 push_global_vptr [51] 
0213 @0x0006e1ac assign_value [] 
0214 @0x0006e1b0 pop_stack [] 
0215 @0x0006e1b4 push_number(S) [1] 
0216 @0x0006e1b8 push_global_vptr [52] 
0217 @0x0006e1bc assign_value [] 
0218 @0x0006e1c0 pop_stack [] 
0219 @0x0006e1c4 create_mapping [0] 
0220 @0x0006e1c8 push_global_vptr [53] 
0221 @0x0006e1cc assign_value [] 
0222 @0x0006e1d0 pop_stack [] 
0223 @0x0006e1d4 create_mapping [0] 
0224 @0x0006e1d8 push_global_vptr [54] 
0225 @0x0006e1dc assign_value [] 
0226 @0x0006e1e0 pop_stack [] 
0227 @0x0006e1e4 push_number(S) [0] 
0228 @0x0006e1e8 push_global_vptr [55] 
0229 @0x0006e1ec assign_value [] 
0230 @0x0006e1f0 pop_stack [] 
0231 @0x0006e1f4 push_shared_string [594] '癸亥'
0232 @0x0006e1f8 push_shared_string [595] '壬戌'
0233 @0x0006e1fc push_shared_string [596] '辛酉'
0234 @0x0006e200 push_shared_string [597] '庚申'
0235 @0x0006e204 push_shared_string [598] '己未'
0236 @0x0006e208 push_shared_string [599] '戊午'
0237 @0x0006e20c push_shared_string [600] '丁巳'
0238 @0x0006e210 push_shared_string [601] '丙辰'
0239 @0x0006e214 push_shared_string [602] '乙卯'
0240 @0x0006e218 push_shared_string [603] '甲寅'
0241 @0x0006e21c push_shared_string [604] '癸丑'
0242 @0x0006e220 push_shared_string [605] '壬子'
0243 @0x0006e224 push_shared_string [606] '辛亥'
0244 @0x0006e228 push_shared_string [607] '庚戌'
0245 @0x0006e22c push_shared_string [608] '己酉'
0246 @0x0006e230 push_shared_string [609] '戊申'
0247 @0x0006e234 push_shared_string [610] '丁未'
0248 @0x0006e238 push_shared_string [611] '丙午'
0249 @0x0006e23c push_shared_string [612] '乙巳'
0250 @0x0006e240 push_shared_string [613] '甲辰'
0251 @0x0006e244 push_shared_string [614] '癸卯'
0252 @0x0006e248 push_shared_string [615] '壬寅'
0253 @0x0006e24c push_shared_string [616] '辛丑'
0254 @0x0006e250 push_shared_string [617] '庚子'
0255 @0x0006e254 push_shared_string [618] '己亥'
0256 @0x0006e258 push_shared_string [619] '戊戌'
0257 @0x0006e25c push_shared_string [620] '丁酉'
0258 @0x0006e260 push_shared_string [621] '丙申'
0259 @0x0006e264 push_shared_string [622] '乙未'
0260 @0x0006e268 push_shared_string [623] '甲午'
0261 @0x0006e26c push_shared_string [624] '癸巳'
0262 @0x0006e270 push_shared_string [625] '壬辰'
0263 @0x0006e274 push_shared_string [626] '辛卯'
0264 @0x0006e278 push_shared_string [627] '庚寅'
0265 @0x0006e27c push_shared_string [628] '己丑'
0266 @0x0006e280 push_shared_string [629] '戊子'
0267 @0x0006e284 push_shared_string [630] '丁亥'
0268 @0x0006e288 push_shared_string [631] '丙戌'
0269 @0x0006e28c push_shared_string [632] '乙酉'
0270 @0x0006e290 push_shared_string [633] '甲申'
0271 @0x0006e294 push_shared_string [634] '癸未'
0272 @0x0006e298 push_shared_string [635] '壬午'
0273 @0x0006e29c push_shared_string [636] '辛巳'
0274 @0x0006e2a0 push_shared_string [637] '庚辰'
0275 @0x0006e2a4 push_shared_string [638] '己卯'
0276 @0x0006e2a8 push_shared_string [639] '戊寅'
0277 @0x0006e2ac push_shared_string [640] '丁丑'
0278 @0x0006e2b0 push_shared_string [641] '丙子'
0279 @0x0006e2b4 push_shared_string [642] '乙亥'
0280 @0x0006e2b8 push_shared_string [643] '甲戌'
0281 @0x0006e2bc push_shared_string [644] '癸酉'
0282 @0x0006e2c0 push_shared_string [645] '壬申'
0283 @0x0006e2c4 push_shared_string [646] '辛未'
0284 @0x0006e2c8 push_shared_string [647] '庚午'
0285 @0x0006e2cc push_shared_string [648] '己巳'
0286 @0x0006e2d0 push_shared_string [649] '戊辰'
0287 @0x0006e2d4 push_shared_string [650] '丁卯'
0288 @0x0006e2d8 push_shared_string [651] '丙寅'
0289 @0x0006e2dc push_shared_string [652] '乙丑'
0290 @0x0006e2e0 push_shared_string [653] '甲子'
0291 @0x0006e2e4 create_array [60] 
0292 @0x0006e2e8 push_global_vptr [56] 
0293 @0x0006e2ec assign_value [] 
0294 @0x0006e2f0 pop_stack [] 
0295 @0x0006e2f4 push_shared_string [323] '癸'
0296 @0x0006e2f8 push_shared_string [321] '壬'
0297 @0x0006e2fc push_shared_string [319] '辛'
0298 @0x0006e300 push_shared_string [317] '庚'
0299 @0x0006e304 push_shared_string [315] '己'
0300 @0x0006e308 push_shared_string [322] '戊'
0301 @0x0006e30c push_shared_string [320] '丁'
0302 @0x0006e310 push_shared_string [318] '丙'
0303 @0x0006e314 push_shared_string [316] '乙'
0304 @0x0006e318 push_shared_string [314] '甲'
0305 @0x0006e31c create_array [10] 
0306 @0x0006e320 push_global_vptr [57] 
0307 @0x0006e324 assign_value [] 
0308 @0x0006e328 pop_stack [] 
0309 @0x0006e32c push_shared_string [654] '亥'
0310 @0x0006e330 push_shared_string [655] '戌'
0311 @0x0006e334 push_shared_string [656] '酉'
0312 @0x0006e338 push_shared_string [657] '申'
0313 @0x0006e33c push_shared_string [658] 'δ'
0314 @0x0006e340 push_shared_string [659] '午'
0315 @0x0006e344 push_shared_string [660] '巳'
0316 @0x0006e348 push_shared_string [661] '辰'
0317 @0x0006e34c push_shared_string [662] 'î'
0318 @0x0006e350 push_shared_string [663] '寅'
0319 @0x0006e354 push_shared_string [664] '丑'
0320 @0x0006e358 push_shared_string [665] '子'
0321 @0x0006e35c create_array [12] 
0322 @0x0006e360 push_global_vptr [58] 
0323 @0x0006e364 assign_value [] 
0324 @0x0006e368 pop_stack [] 
0325 @0x0006e36c push_number(S) [45] 
0326 @0x0006e370 push_shared_string [231] 'year'
0327 @0x0006e374 push_number(S) [12] 
0328 @0x0006e378 push_shared_string [666] 'month'
0329 @0x0006e37c push_number(S) [17] 
0330 @0x0006e380 push_shared_string [325] 'day'
0331 @0x0006e384 push_number(S) [28] 
0332 @0x0006e388 push_shared_string [158] 'hour'
0333 @0x0006e38c create_mapping [8] 
0334 @0x0006e390 push_global_vptr [59] 
0335 @0x0006e394 assign_value [] 
0336 @0x0006e398 pop_stack [] 
0337 @0x0006e39c push_number(S) [31] 
0338 @0x0006e3a0 push_number(S) [30] 
0339 @0x0006e3a4 push_number(S) [31] 
0340 @0x0006e3a8 push_number(S) [30] 
0341 @0x0006e3ac push_number(S) [31] 
0342 @0x0006e3b0 push_number(S) [31] 
0343 @0x0006e3b4 push_number(S) [30] 
0344 @0x0006e3b8 push_number(S) [31] 
0345 @0x0006e3bc push_number(S) [30] 
0346 @0x0006e3c0 push_number(S) [31] 
0347 @0x0006e3c4 push_number(S) [28] 
0348 @0x0006e3c8 push_number(S) [31] 
0349 @0x0006e3cc create_array [12] 
0350 @0x0006e3d0 push_global_vptr [60] 
0351 @0x0006e3d4 assign_value [] 
0352 @0x0006e3d8 pop_stack [] 
0353 @0x0006e3dc push_number(S) [25] 
0354 @0x0006e3e0 push_number(S) [24] 
0355 @0x0006e3e4 push_number(S) [23] 
0356 @0x0006e3e8 push_number(S) [22] 
0357 @0x0006e3ec push_number(S) [21] 
0358 @0x0006e3f0 push_number(S) [19] 
0359 @0x0006e3f4 push_number(S) [18] 
0360 @0x0006e3f8 push_number(S) [17] 
0361 @0x0006e3fc push_number(S) [16] 
0362 @0x0006e400 push_number(S) [15] 
0363 @0x0006e404 push_number(S) [14] 
0364 @0x0006e408 push_number(S) [13] 
0365 @0x0006e40c push_number(S) [12] 
0366 @0x0006e410 push_number(S) [11] 
0367 @0x0006e414 push_number(S) [10] 
0368 @0x0006e418 push_number(S) [9] 
0369 @0x0006e41c push_number(S) [8] 
0370 @0x0006e420 push_number(S) [7] 
0371 @0x0006e424 push_number(S) [6] 
0372 @0x0006e428 push_number(S) [5] 
0373 @0x0006e42c push_number(S) [4] 
0374 @0x0006e430 push_number(S) [3] 
0375 @0x0006e434 push_number(S) [0] 
0376 @0x0006e438 create_array [23] 
0377 @0x0006e43c push_global_vptr [61] 
0378 @0x0006e440 assign_value [] 
0379 @0x0006e444 pop_stack [] 
0380 @0x0006e448 push_shared_string [260] ', '
0381 @0x0006e44c push_global_var [61] 
0382 @0x0006e450 call_in_efun [54, 2] implode
0383 @0x0006e454 push_global_vptr [62] 
0384 @0x0006e458 assign_value [] 
0385 @0x0006e45c pop_stack [] 
0386 @0x0006e460 push_number(S) [0] 
0387 @0x0006e464 push_global_vptr [63] 
0388 @0x0006e468 assign_value [] 
0389 @0x0006e46c pop_stack [] 
0390 @0x0006e470 push_number(S) [29] 
0391 @0x0006e474 push_global_vptr [64] 
0392 @0x0006e478 assign_value [] 
0393 @0x0006e47c pop_stack [] 
0394 @0x0006e480 push_number(S) [27] 
0395 @0x0006e484 push_global_vptr [65] 
0396 @0x0006e488 assign_value [] 
0397 @0x0006e48c pop_stack [] 
0398 @0x0006e490 push_number(S) [1] 
0399 @0x0006e494 push_global_vptr [66] 
0400 @0x0006e498 assign_value [] 
0401 @0x0006e49c pop_stack [] 
0402 @0x0006e4a0 push_number(S) [3] 
0403 @0x0006e4a4 push_global_vptr [67] 
0404 @0x0006e4a8 assign_value [] 
0405 @0x0006e4ac pop_stack [] 
0406 @0x0006e4b0 push_number(S) [-1] 
0407 @0x0006e4b4 push_global_vptr [68] 
0408 @0x0006e4b8 assign_value [] 
0409 @0x0006e4bc pop_stack [] 
0410 @0x0006e4c0 create_mapping [0] 
0411 @0x0006e4c4 push_global_vptr [69] 
0412 @0x0006e4c8 assign_value [] 
0413 @0x0006e4cc pop_stack [] 
0414 @0x0006e4d0 create_mapping [0] 
0415 @0x0006e4d4 push_global_vptr [70] 
0416 @0x0006e4d8 assign_value [] 
0417 @0x0006e4dc pop_stack [] 
0418 @0x0006e4e0 push_number(S) [0] 
0419 @0x0006e4e4 push_global_vptr [71] 
0420 @0x0006e4e8 assign_value [] 
0421 @0x0006e4ec pop_stack [] 
0422 @0x0006e4f0 create_mapping [0] 
0423 @0x0006e4f4 push_global_vptr [72] 
0424 @0x0006e4f8 assign_value [] 
0425 @0x0006e4fc pop_stack [] 
0426 @0x0006e500 create_mapping [0] 
0427 @0x0006e504 push_global_vptr [73] 
0428 @0x0006e508 assign_value [] 
0429 @0x0006e50c pop_stack [] 
0430 @0x0006e510 create_mapping [0] 
0431 @0x0006e514 push_global_vptr [74] 
0432 @0x0006e518 assign_value [] 
0433 @0x0006e51c pop_stack [] 
0434 @0x0006e520 push_number(S) [0] 
0435 @0x0006e524 push_global_vptr [75] 
0436 @0x0006e528 assign_value [] 
0437 @0x0006e52c pop_stack [] 
0438 @0x0006e530 push_number(S) [0] 
0439 @0x0006e534 push_global_vptr [76] 
0440 @0x0006e538 assign_value [] 
0441 @0x0006e53c pop_stack [] 
0442 @0x0006e540 create_mapping [0] 
0443 @0x0006e544 push_global_vptr [77] 
0444 @0x0006e548 assign_value [] 
0445 @0x0006e54c pop_stack [] 
0446 @0x0006e550 create_mapping [0] 
0447 @0x0006e554 push_global_vptr [78] 
0448 @0x0006e558 assign_value [] 
0449 @0x0006e55c pop_stack [] 
0450 @0x0006e560 push_shared_string [135] ''
0451 @0x0006e564 push_global_vptr [79] 
0452 @0x0006e568 assign_value [] 
0453 @0x0006e56c pop_stack [] 
0454 @0x0006e570 push_shared_string [135] ''
0455 @0x0006e574 push_global_vptr [80] 
0456 @0x0006e578 assign_value [] 
0457 @0x0006e57c pop_stack [] 
0458 @0x0006e580 push_number(S) [2] 
0459 @0x0006e584 push_global_vptr [81] 
0460 @0x0006e588 assign_value [] 
0461 @0x0006e58c pop_stack [] 
0462 @0x0006e590 push_number(S) [0] 
0463 @0x0006e594 push_global_vptr [82] 
0464 @0x0006e598 assign_value [] 
0465 @0x0006e59c pop_stack [] 
0466 @0x0006e5a0 create_mapping [0] 
0467 @0x0006e5a4 push_global_vptr [83] 
0468 @0x0006e5a8 assign_value [] 
0469 @0x0006e5ac pop_stack [] 
0470 @0x0006e5b0 push_number(S) [0] 
0471 @0x0006e5b4 push_number(S) [2] 
0472 @0x0006e5b8 push_shared_string [667] 'internal_comm'
0473 @0x0006e5bc push_number(S) [1] 
0474 @0x0006e5c0 push_shared_string [668] 'client_user'
0475 @0x0006e5c4 push_number(S) [4] 
0476 @0x0006e5c8 push_shared_string [669] 'from_server'
0477 @0x0006e5cc create_mapping [6] 
0478 @0x0006e5d0 call_in_efun [105, 2] set_case_check
0479 @0x0006e5d4 push_global_vptr [88] 
0480 @0x0006e5d8 assign_value [] 
0481 @0x0006e5dc pop_stack [] 
0482 @0x0006e5e0 create_mapping [0] 
0483 @0x0006e5e4 push_global_vptr [89] 
0484 @0x0006e5e8 assign_value [] 
0485 @0x0006e5ec pop_stack [] 
0486 @0x0006e5f0 push_number(S) [1] 
0487 @0x0006e5f4 push_shared_string [670] 'msg_intersvr_rpc_ret'
0488 @0x0006e5f8 create_mapping [2] 
0489 @0x0006e5fc push_number(S) [9] 
0490 @0x0006e600 push_number(S) [1] 
0491 @0x0006e604 push_shared_string [671] 'msg_treasure_trading_oper'
0492 @0x0006e608 push_number(S) [1] 
0493 @0x0006e60c push_shared_string [672] 'cmd_cta_to_server'
0494 @0x0006e610 create_mapping [4] 
0495 @0x0006e614 push_number(S) [16] 
0496 @0x0006e618 create_mapping [4] 
0497 @0x0006e61c push_global_vptr [90] 
0498 @0x0006e620 assign_value [] 
0499 @0x0006e624 pop_stack [] 
0500 @0x0006e628 create_array [0] 
0501 @0x0006e62c push_global_vptr [91] 
0502 @0x0006e630 assign_value [] 
0503 @0x0006e634 pop_stack [] 
0504 @0x0006e638 create_array [0] 
0505 @0x0006e63c push_global_vptr [92] 
0506 @0x0006e640 assign_value [] 
0507 @0x0006e644 pop_stack [] 
0508 @0x0006e648 create_array [0] 
0509 @0x0006e64c push_global_vptr [93] 
0510 @0x0006e650 assign_value [] 
0511 @0x0006e654 pop_stack [] 
0512 @0x0006e658 push_number(S) [0] 
0513 @0x0006e65c push_global_vptr [94] 
0514 @0x0006e660 assign_value [] 
0515 @0x0006e664 pop_stack [] 
0516 @0x0006e668 create_mapping [0] 
0517 @0x0006e66c push_global_vptr [95] 
0518 @0x0006e670 assign_value [] 
0519 @0x0006e674 pop_stack [] 
0520 @0x0006e678 call_in_efun [137, 0] time
0521 @0x0006e67c push_global_vptr [96] 
0522 @0x0006e680 assign_value [] 
0523 @0x0006e684 pop_stack [] 
0524 @0x0006e688 push_number(S) [14] 
0525 @0x0006e68c push_global_vptr [97] 
0526 @0x0006e690 assign_value [] 
0527 @0x0006e694 pop_stack [] 
0528 @0x0006e698 push_number(S) [2] 
0529 @0x0006e69c push_global_vptr [98] 
0530 @0x0006e6a0 assign_value [] 
0531 @0x0006e6a4 pop_stack [] 
0532 @0x0006e6a8 push_number(S) [12] 
0533 @0x0006e6ac push_global_vptr [99] 
0534 @0x0006e6b0 assign_value [] 
0535 @0x0006e6b4 pop_stack [] 
0536 @0x0006e6b8 push_number(S) [64] 
0537 @0x0006e6bc push_shared_string [673] 'new_account'
0538 @0x0006e6c0 push_number(S) [16] 
0539 @0x0006e6c4 push_shared_string [674] 'permit_wizard'
0540 @0x0006e6c8 push_number(S) [8] 
0541 @0x0006e6cc push_shared_string [675] 'new_char'
0542 @0x0006e6d0 push_number(S) [2048] 
0543 @0x0006e6d4 push_shared_string [676] 'new_party'
0544 @0x0006e6d8 push_number(S) [2] 
0545 @0x0006e6dc push_shared_string [677] 'login'
0546 @0x0006e6e0 push_number(S) [32] 
0547 @0x0006e6e4 push_shared_string [678] 'charge'
0548 @0x0006e6e8 push_number(L) [536870912] 
0549 @0x0006e6ec push_shared_string [679] 'user29'
0550 @0x0006e6f0 push_number(L) [1073741824] 
0551 @0x0006e6f4 push_shared_string [680] 'user30'
0552 @0x0006e6f8 push_number(L) [-2147483648] 
0553 @0x0006e6fc push_shared_string [681] 'user31'
0554 @0x0006e700 create_mapping [18] 
0555 @0x0006e704 push_global_vptr [100] 
0556 @0x0006e708 assign_value [] 
0557 @0x0006e70c pop_stack [] 
0558 @0x0006e710 create_mapping [0] 
0559 @0x0006e714 push_global_vptr [101] 
0560 @0x0006e718 assign_value [] 
0561 @0x0006e71c pop_stack [] 
0562 @0x0006e720 push_shared_string [682] 'include;aaa;aaa/include'
0563 @0x0006e724 call_in_efun [128, 1] switch_include_dir
0564 @0x0006e728 pop_stack [] 
0565 @0x0006e72c push_shared_string [683] 'full'
0566 @0x0006e730 push_shared_string [684] 'callstack_when_error'
0567 @0x0006e734 call_in_efun [164, 2] config_driver
0568 @0x0006e738 pop_stack [] 
0569 @0x0006e73c call_in_efun [137, 0] time
0570 @0x0006e740 push_global_vptr [104] 
0571 @0x0006e744 assign_value [] 
0572 @0x0006e748 pop_stack [] 
0573 @0x0006e74c push_number(S) [1] 
0574 @0x0006e750 call_in_efun [109, 1] set_log_on
0575 @0x0006e754 pop_stack [] 
0576 @0x0006e758 call_in_user_fun [656, -128] init_startup_module
0577 @0x0006e75c pop_stack [] 
0578 @0x0006e760 call_in_user_fun [796, -128] init_compile_module
0579 @0x0006e764 pop_stack [] 
0580 @0x0006e768 call_in_user_fun [762, -128] init_sql_module
0581 @0x0006e76c pop_stack [] 
0582 @0x0006e770 call_in_user_fun [763, -128] init_debug_cmds_module
0583 @0x0006e774 pop_stack [] 
0584 @0x0006e778 call_in_user_fun [840, -128] init_platform_module
0585 @0x0006e77c pop_stack [] 
0586 @0x0006e780 push_global_var [87] 
0587 @0x0006e784 push_shared_string [685] ' all global feature.\n'
0588 @0x0006e788 add [] 
0589 @0x0006e78c call_in_efun [90, 1] printf
0590 @0x0006e790 pop_stack [] 
0591 @0x0006e794 push_shared_string [686] '/feature/'
0592 @0x0006e798 push_global_var [84] 
0593 @0x0006e79c call_fun_ptr [0, 2] 
0594 @0x0006e7a0 pop_stack [] 
0595 @0x0006e7a4 push_global_var [87] 
0596 @0x0006e7a8 push_shared_string [687] ' all global inherits.\n'
0597 @0x0006e7ac add [] 
0598 @0x0006e7b0 call_in_efun [90, 1] printf
0599 @0x0006e7b4 pop_stack [] 
0600 @0x0006e7b8 push_shared_string [688] '/inherit/'
0601 @0x0006e7bc push_global_var [84] 
0602 @0x0006e7c0 call_fun_ptr [0, 2] 
0603 @0x0006e7c4 pop_stack [] 
0604 @0x0006e7c8 push_global_var [87] 
0605 @0x0006e7cc push_shared_string [689] ' all global objects.\n'
0606 @0x0006e7d0 add [] 
0607 @0x0006e7d4 call_in_efun [90, 1] printf
0608 @0x0006e7d8 pop_stack [] 
0609 @0x0006e7dc push_shared_string [690] '/clone/'
0610 @0x0006e7e0 push_global_var [84] 
0611 @0x0006e7e4 call_fun_ptr [0, 2] 
0612 @0x0006e7e8 pop_stack [] 
0613 @0x0006e7ec push_global_var [87] 
0614 @0x0006e7f0 push_shared_string [691] ' all global cmds.\n'
0615 @0x0006e7f4 add [] 
0616 @0x0006e7f8 call_in_efun [90, 1] printf
0617 @0x0006e7fc pop_stack [] 
0618 @0x0006e800 push_shared_string [692] '/cmds/'
0619 @0x0006e804 push_global_var [84] 
0620 @0x0006e808 call_fun_ptr [0, 2] 
0621 @0x0006e80c pop_stack [] 
0622 @0x0006e810 push_shared_string [693] '/daemons/'
0623 @0x0006e814 push_global_var [85] 
0624 @0x0006e818 call_fun_ptr [0, 2] 
0625 @0x0006e81c pop_stack [] 
0626 @0x0006e820 push_shared_string [694] '/aaa/daemons/'
0627 @0x0006e824 push_global_var [85] 
0628 @0x0006e828 call_fun_ptr [0, 2] 
0629 @0x0006e82c pop_stack [] 
0630 @0x0006e830 push_shared_string [695] '/aaa/clone/'
0631 @0x0006e834 push_global_var [84] 
0632 @0x0006e838 call_fun_ptr [0, 2] 
0633 @0x0006e83c pop_stack [] 
0634 @0x0006e840 call_in_efun [67, 0] is_compiling
0635 @0x0006e844 if [9, 2] 
0636 @0x0006e848 push_shared_string [693] '/daemons/'
0637 @0x0006e84c push_global_var [84] 
0638 @0x0006e850 call_fun_ptr [0, 2] 
0639 @0x0006e854 pop_stack [] 
0640 @0x0006e858 push_shared_string [694] '/aaa/daemons/'
0641 @0x0006e85c push_global_var [84] 
0642 @0x0006e860 call_fun_ptr [0, 2] 
0643 @0x0006e864 pop_stack [] 
0644 @0x0006e868 null_action [] 
0645 @0x0006e86c push_shared_string [696] 'Invoke registered initialization routines.\n'
0646 @0x0006e870 call_in_efun [90, 1] printf
0647 @0x0006e874 pop_stack [] 
0648 @0x0006e878 call_in_user_fun [629, -128] post_init
0649 @0x0006e87c pop_stack [] 
0650 @0x0006e880 push_number(S) [3] 
0651 @0x0006e884 call_in_user_fun [790, -127] set_server_status
0652 @0x0006e888 pop_stack [] 
0653 @0x0006e88c push_shared_string [697] '\x1b[1;33mDriver load ok!\n\x1b[2;37;0m'
0654 @0x0006e890 call_in_efun [90, 1] printf
0655 @0x0006e894 pop_stack [] 
0656 @0x0006e898 push_shared_string [698] 'Driver startup\n'
0657 @0x0006e89c call_in_efun [10, 1] broadcast
0658 @0x0006e8a0 pop_stack [] 
0659 @0x0006e8a4 null_action [] 
0660 @0x0006e8a8 finish_action [] 
```

<a id="aaa-post_init"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: post_init

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x3ed1e`；函数编号：`629`。

```text
0000 @0x0003ee43 check_args_num [0] 
0001 @0x0003ee47 enter_function [] 
0002 @0x0003ee4b push_global_var [31] 
0003 @0x0003ee4f sizeof [] 
0004 @0x0003ee53 if [36, 2] 
0005 @0x0003ee57 push_global_var [31] 
0006 @0x0003ee5b push_local_vptr [-2] 
0007 @0x0003ee5f assign_value [] 
0008 @0x0003ee63 pop_stack [] 
0009 @0x0003ee67 create_array [0] 
0010 @0x0003ee6b push_global_vptr [31] 
0011 @0x0003ee6f assign_value [] 
0012 @0x0003ee73 pop_stack [] 
0013 @0x0003ee77 push_local_vptr [-1] 
0014 @0x0003ee7b push_local_value [-2] 
0015 @0x0003ee7f push_number(S) [997] 
0016 @0x0003ee83 init_for_loop [-3] 
0017 @0x0003ee87 cond_jmp_for_loop [14, -3] 
0018 @0x0003ee8b push_local_value [-1] 
0019 @0x0003ee8f is_type [9] 
0020 @0x0003ee93 if [9, 2] 
0021 @0x0003ee97 new_thread_catch [6] 
0022 @0x0003ee9b push_local_value [-1] 
0023 @0x0003ee9f call_fun_ptr [4, 1] 
0024 @0x0003eea3 pop_stack [] 
0025 @0x0003eea7 push_number(S) [0] 
0026 @0x0003eeab end_thread_catch [] 
0027 @0x0003eeaf goto [1] 
0028 @0x0003eeb3 caught_error [] 
0029 @0x0003eeb7 pop_stack [] 
0030 @0x0003eebb null_action [] 
0031 @0x0003eebf step_jmp_for_loop [-14, -3] 
0032 @0x0003eec3 push_global_var [31] 
0033 @0x0003eec7 sizeof [] 
0034 @0x0003eecb if [4, 2] 
0035 @0x0003eecf push_number(S) [1] 
0036 @0x0003eed3 call_in_efun [298, 1] sleep
0037 @0x0003eed7 pop_stack [] 
0038 @0x0003eedb null_action [] 
0039 @0x0003eedf null_action [] 
0040 @0x0003eee3 goto [-39] 
0041 @0x0003eee7 push_number(S) [0] 
0042 @0x0003eeeb push_global_vptr [31] 
0043 @0x0003eeef assign_value [] 
0044 @0x0003eef3 pop_stack [] 
0045 @0x0003eef7 null_action [] 
0046 @0x0003eefb return_from_fun [] 
```

<a id="aaa-xdbc_query"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: xdbc_query

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x44e2c`；函数编号：`593`。

```text
0000 @0x00044f51 check_args_num [0] 
0001 @0x00044f55 enter_function [] 
0002 @0x00044f59 push_local_value [1] 
0003 @0x00044f5d push_local_value [0] 
0004 @0x00044f61 push_shared_string [116] 'Call xdbc_query(%d, %s)\n'
0005 @0x00044f65 push_number(S) [0] 
0006 @0x00044f69 call_in_user_fun [584, -124] xdbc_trace
0007 @0x00044f6d pop_stack [] 
0008 @0x00044f71 push_shared_string [117] 'query'
0009 @0x00044f75 push_shared_string [39] 'cmd'
0010 @0x00044f79 push_local_value [1] 
0011 @0x00044f7d push_shared_string [115] 'sql_cmd'
0012 @0x00044f81 create_mapping [4] 
0013 @0x00044f85 push_local_value [0] 
0014 @0x00044f89 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x00044f8d push_local_vptr [-1] 
0016 @0x00044f91 assign_value [] 
0017 @0x00044f95 pop_stack [] 
0018 @0x00044f99 push_local_value [-1] 
0019 @0x00044f9d push_number(S) [-1] 
0020 @0x00044fa1 eq_value [] 
0021 @0x00044fa5 if [2, 2] 
0022 @0x00044fa9 push_number(S) [-1] 
0023 @0x00044fad return_from_fun [] 
0024 @0x00044fb1 push_local_value [0] 
0025 @0x00044fb5 push_shared_string [118] 'cno'
0026 @0x00044fb9 create_mapping [2] 
0027 @0x00044fbd push_global_vptr [8] 
0028 @0x00044fc1 push_local_value [-1] 
0029 @0x00044fc5 query_member_ptr [0] 
0030 @0x00044fc9 assign_value [] 
0031 @0x00044fcd pop_stack [] 
0032 @0x00044fd1 push_local_value [-1] 
0033 @0x00044fd5 return_from_fun [] 
0034 @0x00044fd9 null_action [] 
0035 @0x00044fdd return_from_fun [] 
```

<a id="aaa-xdbc_connect"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: xdbc_connect

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x4ab55`；函数编号：`588`。

```text
0000 @0x0004add3 check_args_num [0] 
0001 @0x0004add7 enter_function [] 
0002 @0x0004addb push_number(S) [2] 
0003 @0x0004addf call_in_efun [267, 1] socket
0004 @0x0004ade3 push_local_vptr [-3] 
0005 @0x0004ade7 assign_value [] 
0006 @0x0004adeb pop_stack [] 
0007 @0x0004adef push_number(S) [9527] 
0008 @0x0004adf3 push_local_value [0] 
0009 @0x0004adf7 push_local_value [-3] 
0010 @0x0004adfb call_in_efun [254, 3] connect
0011 @0x0004adff push_number(S) [0] 
0012 @0x0004ae03 ne_value [] 
0013 @0x0004ae07 if [12, 2] 
0014 @0x0004ae0b push_number(S) [9527] 
0015 @0x0004ae0f push_local_value [0] 
0016 @0x0004ae13 push_shared_string [99] 'Failed to connect to %s:%d\n'
0017 @0x0004ae17 push_number(S) [2] 
0018 @0x0004ae1b call_in_user_fun [581, -124] xdbc_log
0019 @0x0004ae1f pop_stack [] 
0020 @0x0004ae23 push_local_value [-3] 
0021 @0x0004ae27 call_in_efun [255, 1] close_socket
0022 @0x0004ae2b pop_stack [] 
0023 @0x0004ae2f push_number(S) [-1] 
0024 @0x0004ae33 return_from_fun [] 
0025 @0x0004ae37 null_action [] 
0026 @0x0004ae3b push_local_value [-3] 
0027 @0x0004ae3f call_in_user_fun [572, -127] xdbc_allocate_client
0028 @0x0004ae43 push_local_vptr [-4] 
0029 @0x0004ae47 assign_value [] 
0030 @0x0004ae4b push_number(S) [-1] 
0031 @0x0004ae4f eq_value [] 
0032 @0x0004ae53 if [10, 2] 
0033 @0x0004ae57 push_shared_string [100] 'No more free client to allocate.\n'
0034 @0x0004ae5b push_number(S) [2] 
0035 @0x0004ae5f call_in_user_fun [581, -126] xdbc_log
0036 @0x0004ae63 pop_stack [] 
0037 @0x0004ae67 push_local_value [-3] 
0038 @0x0004ae6b call_in_efun [255, 1] close_socket
0039 @0x0004ae6f pop_stack [] 
0040 @0x0004ae73 push_number(S) [-1] 
0041 @0x0004ae77 return_from_fun [] 
0042 @0x0004ae7b null_action [] 
0043 @0x0004ae7f push_number(L) [2000000000] 
0044 @0x0004ae83 call_in_efun [94, 1] random
0045 @0x0004ae87 push_local_vptr [-5] 
0046 @0x0004ae8b assign_value [] 
0047 @0x0004ae8f pop_stack [] 
0048 @0x0004ae93 push_shared_string [101] 'set_key'
0049 @0x0004ae97 push_shared_string [39] 'cmd'
0050 @0x0004ae9b push_local_value [-5] 
0051 @0x0004ae9f push_shared_string [40] 'key'
0052 @0x0004aea3 create_mapping [4] 
0053 @0x0004aea7 push_local_value [-4] 
0054 @0x0004aeab call_in_user_fun [601, -126] xdbc_call_server
0055 @0x0004aeaf push_local_vptr [-2] 
0056 @0x0004aeb3 assign_value [] 
0057 @0x0004aeb7 pop_stack [] 
0058 @0x0004aebb push_local_value [-2] 
0059 @0x0004aebf push_number(S) [-1] 
0060 @0x0004aec3 eq_value [] 
0061 @0x0004aec7 if [7, 1] 
0062 @0x0004aecb pop_stack [] 
0063 @0x0004aecf push_local_value [-4] 
0064 @0x0004aed3 call_in_user_fun [575, -127] xdbc_get_client
0065 @0x0004aed7 push_local_vptr [-1] 
0066 @0x0004aedb assign_value [] 
0067 @0x0004aedf is_type [7] 
0068 @0x0004aee3 not_value [] 
0069 @0x0004aee7 if [12, 2] 
0070 @0x0004aeeb push_local_value [-2] 
0071 @0x0004aeef push_local_value [-4] 
0072 @0x0004aef3 push_shared_string [102] 'Failed to prepare connection %d(result = %d).\n'
0073 @0x0004aef7 push_number(S) [2] 
0074 @0x0004aefb call_in_user_fun [581, -124] xdbc_log
0075 @0x0004aeff pop_stack [] 
0076 @0x0004af03 push_local_value [-4] 
0077 @0x0004af07 call_in_user_fun [573, -127] xdbc_free_client
0078 @0x0004af0b pop_stack [] 
0079 @0x0004af0f push_number(S) [-1] 
0080 @0x0004af13 return_from_fun [] 
0081 @0x0004af17 null_action [] 
0082 @0x0004af1b push_local_value [-5] 
0083 @0x0004af1f push_shared_string [103] 'Set key: %016X\n'
0084 @0x0004af23 push_number(S) [1] 
0085 @0x0004af27 call_in_user_fun [584, -125] xdbc_trace
0086 @0x0004af2b pop_stack [] 
0087 @0x0004af2f push_local_value [-5] 
0088 @0x0004af33 push_local_vptr [-1] 
0089 @0x0004af37 push_shared_string [40] 'key'
0090 @0x0004af3b query_member_ptr [0] 
0091 @0x0004af3f assign_value [] 
0092 @0x0004af43 pop_stack [] 
0093 @0x0004af47 push_shared_string [104] 'connect'
0094 @0x0004af4b push_shared_string [39] 'cmd'
0095 @0x0004af4f push_local_value [1] 
0096 @0x0004af53 push_shared_string [105] 'user'
0097 @0x0004af57 push_local_value [2] 
0098 @0x0004af5b push_shared_string [106] 'passwd'
0099 @0x0004af5f push_local_value [3] 
0100 @0x0004af63 push_shared_string [107] 'db'
0101 @0x0004af67 create_mapping [8] 
0102 @0x0004af6b push_local_value [-4] 
0103 @0x0004af6f call_in_user_fun [601, -126] xdbc_call_server
0104 @0x0004af73 push_local_vptr [-2] 
0105 @0x0004af77 assign_value [] 
0106 @0x0004af7b pop_stack [] 
0107 @0x0004af7f push_local_value [-2] 
0108 @0x0004af83 push_number(S) [-1] 
0109 @0x0004af87 eq_value [] 
0110 @0x0004af8b if [10, 2] 
0111 @0x0004af8f push_shared_string [108] 'Failed to connect to database.\n'
0112 @0x0004af93 push_number(S) [2] 
0113 @0x0004af97 call_in_user_fun [581, -126] xdbc_log
0114 @0x0004af9b pop_stack [] 
0115 @0x0004af9f push_local_value [-4] 
0116 @0x0004afa3 call_in_user_fun [573, -127] xdbc_free_client
0117 @0x0004afa7 pop_stack [] 
0118 @0x0004afab push_number(S) [-1] 
0119 @0x0004afaf return_from_fun [] 
0120 @0x0004afb3 null_action [] 
0121 @0x0004afb7 push_local_value [-4] 
0122 @0x0004afbb return_from_fun [] 
0123 @0x0004afbf null_action [] 
0124 @0x0004afc3 return_from_fun [] 
```

<a id="aaa-xdbc_execute"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: xdbc_execute

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x4c982`；函数编号：`592`。

```text
0000 @0x0004ca5b check_args_num [0] 
0001 @0x0004ca5f enter_function [] 
0002 @0x0004ca63 push_local_value [1] 
0003 @0x0004ca67 push_local_value [0] 
0004 @0x0004ca6b push_shared_string [113] 'Call xdbc_execute(%d, %s)\n'
0005 @0x0004ca6f push_number(S) [0] 
0006 @0x0004ca73 call_in_user_fun [584, -124] xdbc_trace
0007 @0x0004ca77 pop_stack [] 
0008 @0x0004ca7b push_shared_string [114] 'execute'
0009 @0x0004ca7f push_shared_string [39] 'cmd'
0010 @0x0004ca83 push_local_value [1] 
0011 @0x0004ca87 push_shared_string [115] 'sql_cmd'
0012 @0x0004ca8b create_mapping [4] 
0013 @0x0004ca8f push_local_value [0] 
0014 @0x0004ca93 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x0004ca97 return_from_fun [] 
0016 @0x0004ca9b null_action [] 
0017 @0x0004ca9f return_from_fun [] 
```

<a id="aaa-init_startup_module"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: init_startup_module

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x524bd`；函数编号：`656`。

```text
0000 @0x0005254d check_args_num [0] 
0001 @0x00052551 enter_function [] 
0002 @0x00052555 push_shared_string [154] '/aaa/log/'
0003 @0x00052559 call_in_user_fun [613, -127] assure_file
0004 @0x0005255d pop_stack [] 
0005 @0x00052561 push_shared_string [155] 'Done of init_startup_module.\n'
0006 @0x00052565 call_in_efun [90, 1] printf
0007 @0x00052569 pop_stack [] 
0008 @0x0005256d null_action [] 
0009 @0x00052571 return_from_fun [] 
```

<a id="aaa-init_sql_module"></a>

## aaa/pack_data/lib_aaa32.pak/aaa/start_aaa.o :: init_sql_module

成员 SHA-256：`f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e`；函数记录偏移：`0x53f94`；函数编号：`762`。

```text
0000 @0x0005402e check_args_num [0] 
0001 @0x00054032 enter_function [] 
0002 @0x00054036 push_shared_string [378] ':memory:'
0003 @0x0005403a call_in_efun [423, 1] sql_open_db
0004 @0x0005403e push_global_vptr [68] 
0005 @0x00054042 assign_value [] 
0006 @0x00054046 pop_stack [] 
0007 @0x0005404a push_global_var [68] 
0008 @0x0005404e push_number(S) [-1] 
0009 @0x00054052 eq_value [] 
0010 @0x00054056 if [3, 2] 
0011 @0x0005405a push_shared_string [379] 'Failed to open memory database, startup halt.\n'
0012 @0x0005405e call_in_efun [28, 1] error
0013 @0x00054062 pop_stack [] 
0014 @0x00054066 push_shared_string [380] 'Done of init_sql_module.\n'
0015 @0x0005406a call_in_efun [90, 1] printf
0016 @0x0005406e pop_stack [] 
0017 @0x00054072 null_action [] 
0018 @0x00054076 return_from_fun [] 
```

<a id="ccs-daemons-filed-o-dba_comm"></a>

## ccs/pack_data/lib_ccs32.pak/daemons/filed.o :: dba_comm

成员 SHA-256：`745340ee35ba4b93d1ee4b34fc14785df62ad3bd499457717547a878ac815795`；函数记录偏移：`0x1cdd`；函数编号：`101`。

```text
0000 @0x00001e57 check_args_num [0] 
0001 @0x00001e5b enter_function [] 
0002 @0x00001e5f call_in_efun [290, 0] set_as_daemon
0003 @0x00001e63 pop_stack [] 
0004 @0x00001e67 push_shared_string [133] 'FILE_D.dba_comm daemon start...\n'
0005 @0x00001e6b call_in_efun [90, 1] printf
0006 @0x00001e6f pop_stack [] 
0007 @0x00001e73 push_obj_var [12] 
0008 @0x00001e77 push_number(S) [-1] 
0009 @0x00001e7b ne_value [] 
0010 @0x00001e7f if [4, 0] 
0011 @0x00001e83 pop_stack [] 
0012 @0x00001e87 push_obj_var [4] 
0013 @0x00001e8b push_number(S) [0] 
0014 @0x00001e8f lt_value [] 
0015 @0x00001e93 if [11, 2] 
0016 @0x00001e97 push_number(S) [2] 
0017 @0x00001e9b push_shared_string [132] 'wait_config'
0018 @0x00001e9f push_shared_string [29] '/daemons/configd.c'
0019 @0x00001ea3 call_in_other_fun_ex [4, 1] 
0020 @0x00001ea7 if [3, 1] 
0021 @0x00001eab pop_stack [] 
0022 @0x00001eaf push_number(S) [1] 
0023 @0x00001eb3 call_in_user_fun [660, -127] check_server_status
0024 @0x00001eb7 if [1, 2] 
0025 @0x00001ebb goto [1] 
0026 @0x00001ebf goto [-20] 
0027 @0x00001ec3 push_obj_var [10] 
0028 @0x00001ec7 if [8, 2] 
0029 @0x00001ecb call_in_user_fun [7, 0] try_to_connect_dba
0030 @0x00001ecf not_value [] 
0031 @0x00001ed3 if [4, 2] 
0032 @0x00001ed7 push_number(S) [2] 
0033 @0x00001edb call_in_efun [298, 1] sleep
0034 @0x00001edf pop_stack [] 
0035 @0x00001ee3 goto [-7] 
0036 @0x00001ee7 null_action [] 
0037 @0x00001eeb push_local_vptr [-1] 
0038 @0x00001eef push_obj_var [3] 
0039 @0x00001ef3 call_in_efun [314, 2] receive_from_queue
0040 @0x00001ef7 push_number(S) [0] 
0041 @0x00001efb ne_value [] 
0042 @0x00001eff if [1, 2] 
0043 @0x00001f03 goto [79] 
0044 @0x00001f07 push_number(S) [1] 
0045 @0x00001f0b if [76, 2] 
0046 @0x00001f0f push_local_value [-1] 
0047 @0x00001f13 push_shared_string [25] 'operate'
0048 @0x00001f17 query_member [0] 
0049 @0x00001f1b push_number(S) [0] 
0050 @0x00001f1f eq_value [] 
0051 @0x00001f23 if [5, 2] 
0052 @0x00001f27 push_shared_string [134] 'FILE_D.dba_comm done.\n'
0053 @0x00001f2b call_in_efun [90, 1] printf
0054 @0x00001f2f pop_stack [] 
0055 @0x00001f33 return_from_fun [] 
0056 @0x00001f37 null_action [] 
0057 @0x00001f3b push_obj_var [6] 
0058 @0x00001f3f push_local_value [-1] 
0059 @0x00001f43 push_shared_string [53] 'cookie'
0060 @0x00001f47 query_member [0] 
0061 @0x00001f4b query_member [0] 
0062 @0x00001f4f is_type [7] 
0063 @0x00001f53 not_value [] 
0064 @0x00001f57 if [1, 2] 
0065 @0x00001f5b goto [56] 
0066 @0x00001f5f call_in_efun [137, 0] time
0067 @0x00001f63 push_local_vptr [-1] 
0068 @0x00001f67 push_shared_string [135] 'send_time'
0069 @0x00001f6b query_member_ptr [0] 
0070 @0x00001f6f assign_value [] 
0071 @0x00001f73 pop_stack [] 
0072 @0x00001f77 push_local_value [-1] 
0073 @0x00001f7b push_obj_vptr [6] 
0074 @0x00001f7f push_local_value [-1] 
0075 @0x00001f83 push_shared_string [53] 'cookie'
0076 @0x00001f87 query_member [0] 
0077 @0x00001f8b query_member_ptr [0] 
0078 @0x00001f8f assign_value [] 
0079 @0x00001f93 pop_stack [] 
0080 @0x00001f97 push_obj_var [12] 
0081 @0x00001f9b push_number(S) [-1] 
0082 @0x00001f9f eq_value [] 
0083 @0x00001fa3 if [4, 0] 
0084 @0x00001fa7 pop_stack [] 
0085 @0x00001fab push_obj_var [4] 
0086 @0x00001faf push_number(S) [0] 
0087 @0x00001fb3 lt_value [] 
0088 @0x00001fb7 if [5, 2] 
0089 @0x00001fbb push_local_value [-1] 
0090 @0x00001fbf call_in_user_fun [109, 1] failure_routine
0091 @0x00001fc3 pop_stack [] 
0092 @0x00001fc7 goto [29] 
0093 @0x00001fcb null_action [] 
0094 @0x00001fcf push_local_value [-1] 
0095 @0x00001fd3 push_shared_string [52] 'expired'
0096 @0x00001fd7 query_member [0] 
0097 @0x00001fdb call_in_efun [137, 0] time
0098 @0x00001fdf le_value [] 
0099 @0x00001fe3 if [5, 2] 
0100 @0x00001fe7 push_local_value [-1] 
0101 @0x00001feb call_in_user_fun [109, 1] failure_routine
0102 @0x00001fef pop_stack [] 
0103 @0x00001ff3 goto [18] 
0104 @0x00001ff7 null_action [] 
0105 @0x00001ffb call_in_user_fun [7, 0] try_to_connect_dba
0106 @0x00001fff not_value [] 
0107 @0x00002003 if [5, 2] 
0108 @0x00002007 push_number(S) [2] 
0109 @0x0000200b call_in_efun [298, 1] sleep
0110 @0x0000200f pop_stack [] 
0111 @0x00002013 goto [-68] 
0112 @0x00002017 null_action [] 
0113 @0x0000201b push_local_value [-1] 
0114 @0x0000201f push_shared_string [136] 'req_buf'
0115 @0x00002023 query_member [0] 
0116 @0x00002027 push_obj_var [4] 
0117 @0x0000202b call_in_efun [527, 2] send_to_client
0118 @0x0000202f pop_stack [] 
0119 @0x00002033 goto [2] 
0120 @0x00002037 null_action [] 
0121 @0x0000203b goto [-78] 
0122 @0x0000203f null_action [] 
0123 @0x00002043 goto [-87] 
0124 @0x00002047 null_action [] 
0125 @0x0000204b return_from_fun [] 
```

<a id="ccs-daemons-filed-o-try_to_connect_dba"></a>

## ccs/pack_data/lib_ccs32.pak/daemons/filed.o :: try_to_connect_dba

成员 SHA-256：`745340ee35ba4b93d1ee4b34fc14785df62ad3bd499457717547a878ac815795`；函数记录偏移：`0x38b1`；函数编号：`7`。

```text
0000 @0x00003a33 check_args_num [0] 
0001 @0x00003a37 enter_function [] 
0002 @0x00003a3b push_obj_var [4] 
0003 @0x00003a3f push_number(S) [0] 
0004 @0x00003a43 lt_value [] 
0005 @0x00003a47 if [2, 2] 
0006 @0x00003a4b create_mapping [0] 
0007 @0x00003a4f goto [2] 
0008 @0x00003a53 push_obj_var [4] 
0009 @0x00003a57 call_in_efun [515, 1] get_client_info
0010 @0x00003a5b push_local_vptr [-2] 
0011 @0x00003a5f assign_value [] 
0012 @0x00003a63 pop_stack [] 
0013 @0x00003a67 push_local_value [-2] 
0014 @0x00003a6b push_shared_string [28] 'alive'
0015 @0x00003a6f query_member [0] 
0016 @0x00003a73 not_value [] 
0017 @0x00003a77 if [97, 2] 
0018 @0x00003a7b push_shared_string [27] 'get_dba_address'
0019 @0x00003a7f push_shared_string [29] '/daemons/configd.c'
0020 @0x00003a83 call_in_other_fun_ex [4, 0] 
0021 @0x00003a87 push_local_vptr [-1] 
0022 @0x00003a8b assign_value [] 
0023 @0x00003a8f is_type [7] 
0024 @0x00003a93 not_value [] 
0025 @0x00003a97 if [2, 2] 
0026 @0x00003a9b push_number(S) [0] 
0027 @0x00003a9f return_from_fun [] 
0028 @0x00003aa3 push_shared_string [9] 'mfiledconn'
0029 @0x00003aa7 call_in_efun [305, 1] take_mutex
0030 @0x00003aab push_number(S) [0] 
0031 @0x00003aaf ne_value [] 
0032 @0x00003ab3 if [2, 2] 
0033 @0x00003ab7 push_number(S) [0] 
0034 @0x00003abb return_from_fun [] 
0035 @0x00003abf push_local_value [-1] 
0036 @0x00003ac3 push_shared_string [30] 'port'
0037 @0x00003ac7 query_member [0] 
0038 @0x00003acb push_local_value [-1] 
0039 @0x00003acf push_shared_string [31] 'ip'
0040 @0x00003ad3 query_member [0] 
0041 @0x00003ad7 push_shared_string [32] 'Connect to DBA: %O:%O.\n'
0042 @0x00003adb call_in_efun [90, 3] printf
0043 @0x00003adf pop_stack [] 
0044 @0x00003ae3 call_in_efun [267, 0] socket
0045 @0x00003ae7 push_local_vptr [-3] 
0046 @0x00003aeb assign_value [] 
0047 @0x00003aef pop_stack [] 
0048 @0x00003af3 push_number(L) [4194304] 
0049 @0x00003af7 push_number(S) [8] 
0050 @0x00003afb push_number(S) [1] 
0051 @0x00003aff push_local_value [-3] 
0052 @0x00003b03 call_in_efun [266, 4] set_sock_opt
0053 @0x00003b07 pop_stack [] 
0054 @0x00003b0b push_number(L) [4194304] 
0055 @0x00003b0f push_number(S) [7] 
0056 @0x00003b13 push_number(S) [1] 
0057 @0x00003b17 push_local_value [-3] 
0058 @0x00003b1b call_in_efun [266, 4] set_sock_opt
0059 @0x00003b1f pop_stack [] 
0060 @0x00003b23 push_local_value [-1] 
0061 @0x00003b27 push_shared_string [30] 'port'
0062 @0x00003b2b query_member [0] 
0063 @0x00003b2f push_local_value [-1] 
0064 @0x00003b33 push_shared_string [31] 'ip'
0065 @0x00003b37 query_member [0] 
0066 @0x00003b3b push_local_value [-3] 
0067 @0x00003b3f call_in_efun [254, 3] connect
0068 @0x00003b43 push_number(S) [0] 
0069 @0x00003b47 ne_value [] 
0070 @0x00003b4b if [6, 1] 
0071 @0x00003b4f pop_stack [] 
0072 @0x00003b53 push_local_value [-3] 
0073 @0x00003b57 call_in_efun [508, 1] bind_to_client
0074 @0x00003b5b call_in_user_fun [117, 1] set_dba_client_no
0075 @0x00003b5f push_number(S) [0] 
0076 @0x00003b63 lt_value [] 
0077 @0x00003b67 if [12, 2] 
0078 @0x00003b6b push_shared_string [33] 'Connect to DBA failed.\n'
0079 @0x00003b6f call_in_efun [90, 1] printf
0080 @0x00003b73 pop_stack [] 
0081 @0x00003b77 push_local_value [-3] 
0082 @0x00003b7b call_in_efun [255, 1] close_socket
0083 @0x00003b7f pop_stack [] 
0084 @0x00003b83 push_shared_string [9] 'mfiledconn'
0085 @0x00003b87 call_in_efun [306, 1] give_mutex
0086 @0x00003b8b pop_stack [] 
0087 @0x00003b8f push_number(S) [0] 
0088 @0x00003b93 return_from_fun [] 
0089 @0x00003b97 null_action [] 
0090 @0x00003b9b push_shared_string [34] 'internal'
0091 @0x00003b9f push_shared_string [35] 'type'
0092 @0x00003ba3 push_number(S) [-1] 
0093 @0x00003ba7 push_shared_string [36] 'peek'
0094 @0x00003bab push_number(L) [131000] 
0095 @0x00003baf push_shared_string [37] 'size'
0096 @0x00003bb3 create_mapping [6] 
0097 @0x00003bb7 push_obj_var [4] 
0098 @0x00003bbb call_in_efun [510, 2] set_client_attrib
0099 @0x00003bbf pop_stack [] 
0100 @0x00003bc3 push_number(S) [0] 
0101 @0x00003bc7 call_in_efun [521, 1] make32
0102 @0x00003bcb push_global_var [80] 
0103 @0x00003bcf push_number(S) [7] 
0104 @0x00003bd3 push_number(S) [14849] 
0105 @0x00003bd7 push_obj_var [4] 
0106 @0x00003bdb call_in_efun [527, 5] send_to_client
0107 @0x00003bdf pop_stack [] 
0108 @0x00003be3 push_shared_string [38] 'Connected to DBA.\n'
0109 @0x00003be7 call_in_efun [90, 1] printf
0110 @0x00003beb pop_stack [] 
0111 @0x00003bef push_shared_string [9] 'mfiledconn'
0112 @0x00003bf3 call_in_efun [306, 1] give_mutex
0113 @0x00003bf7 pop_stack [] 
0114 @0x00003bfb null_action [] 
0115 @0x00003bff push_number(S) [1] 
0116 @0x00003c03 return_from_fun [] 
0117 @0x00003c07 null_action [] 
0118 @0x00003c0b return_from_fun [] 
```

<a id="ccs-daemons-filed-o-create_content"></a>

## ccs/pack_data/lib_ccs32.pak/daemons/filed.o :: create_content

成员 SHA-256：`745340ee35ba4b93d1ee4b34fc14785df62ad3bd499457717547a878ac815795`；函数记录偏移：`0xb358`；函数编号：`15`。

```text
0000 @0x0000b454 check_args_num [0] 
0001 @0x0000b458 enter_function [] 
0002 @0x0000b45c push_number(S) [0] 
0003 @0x0000b460 push_local_value [3] 
0004 @0x0000b464 push_local_value [2] 
0005 @0x0000b468 push_local_value [1] 
0006 @0x0000b46c push_local_value [0] 
0007 @0x0000b470 push_number(S) [0] 
0008 @0x0000b474 push_number(S) [4] 
0009 @0x0000b478 push_number(S) [1] 
0010 @0x0000b47c call_in_user_fun [111, 8] do_operate
0011 @0x0000b480 return_from_fun [] 
0012 @0x0000b484 null_action [] 
0013 @0x0000b488 return_from_fun [] 
```

<a id="ccs-daemons-filed-o-read_content"></a>

## ccs/pack_data/lib_ccs32.pak/daemons/filed.o :: read_content

成员 SHA-256：`745340ee35ba4b93d1ee4b34fc14785df62ad3bd499457717547a878ac815795`；函数记录偏移：`0xbe16`；函数编号：`9`。

```text
0000 @0x0000bf61 check_args_num [0] 
0001 @0x0000bf65 enter_function [] 
0002 @0x0000bf69 push_local_value [3] 
0003 @0x0000bf6d case_check_string [2, 45] 
0004 @0x0000bf71 case_check_string [19, 46] 
0005 @0x0000bf75 case_default [29] 
0006 @0x0000bf79 push_number(S) [0] 
0007 @0x0000bf7d push_number(S) [0] 
0008 @0x0000bf81 push_local_value [2] 
0009 @0x0000bf85 push_local_value [1] 
0010 @0x0000bf89 push_local_value [0] 
0011 @0x0000bf8d push_number(S) [0] 
0012 @0x0000bf91 push_number(S) [1] 
0013 @0x0000bf95 push_number(S) [1] 
0014 @0x0000bf99 call_in_user_fun [111, 8] do_operate
0015 @0x0000bf9d push_local_vptr [-1] 
0016 @0x0000bfa1 assign_value [] 
0017 @0x0000bfa5 pop_stack [] 
0018 @0x0000bfa9 push_local_value [-1] 
0019 @0x0000bfad is_type [4] 
0020 @0x0000bfb1 if [2, 2] 
0021 @0x0000bfb5 push_local_value [-1] 
0022 @0x0000bfb9 return_from_fun [] 
0023 @0x0000bfbd null_action [] 
0024 @0x0000bfc1 push_number(S) [0] 
0025 @0x0000bfc5 push_number(S) [0] 
0026 @0x0000bfc9 push_local_value [2] 
0027 @0x0000bfcd push_local_value [1] 
0028 @0x0000bfd1 push_local_value [0] 
0029 @0x0000bfd5 push_number(S) [0] 
0030 @0x0000bfd9 push_number(S) [22] 
0031 @0x0000bfdd push_number(S) [1] 
0032 @0x0000bfe1 call_in_user_fun [111, 8] do_operate
0033 @0x0000bfe5 return_from_fun [] 
0034 @0x0000bfe9 null_action [] 
0035 @0x0000bfed push_number(S) [0] 
0036 @0x0000bff1 push_number(S) [0] 
0037 @0x0000bff5 push_local_value [2] 
0038 @0x0000bff9 push_local_value [1] 
0039 @0x0000bffd push_local_value [0] 
0040 @0x0000c001 push_number(S) [0] 
0041 @0x0000c005 push_number(S) [1] 
0042 @0x0000c009 push_number(S) [1] 
0043 @0x0000c00d call_in_user_fun [111, 8] do_operate
0044 @0x0000c011 return_from_fun [] 
0045 @0x0000c015 null_action [] 
0046 @0x0000c019 null_action [] 
0047 @0x0000c01d return_from_fun [] 
```

<a id="ccs-daemons-filed-o-write_content"></a>

## ccs/pack_data/lib_ccs32.pak/daemons/filed.o :: write_content

成员 SHA-256：`745340ee35ba4b93d1ee4b34fc14785df62ad3bd499457717547a878ac815795`；函数记录偏移：`0x20e7f`；函数编号：`11`。

```text
0000 @0x00020f7a check_args_num [0] 
0001 @0x00020f7e enter_function [] 
0002 @0x00020f82 push_number(S) [0] 
0003 @0x00020f86 push_local_value [3] 
0004 @0x00020f8a push_local_value [2] 
0005 @0x00020f8e push_local_value [1] 
0006 @0x00020f92 push_local_value [0] 
0007 @0x00020f96 push_number(S) [0] 
0008 @0x00020f9a push_number(S) [2] 
0009 @0x00020f9e push_number(S) [1] 
0010 @0x00020fa2 call_in_user_fun [111, 8] do_operate
0011 @0x00020fa6 return_from_fun [] 
0012 @0x00020faa null_action [] 
0013 @0x00020fae return_from_fun [] 
```

<a id="ccs-__init__"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: __init__

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x87c10`；函数编号：`init`。

```text
0000 @0x00088197 push_number(S) [1024] 
0001 @0x0008819b call_in_efun [0, 1] allocate_array
0002 @0x0008819f push_global_vptr [2] 
0003 @0x000881a3 assign_value [] 
0004 @0x000881a7 pop_stack [] 
0005 @0x000881ab push_number(S) [2] 
0006 @0x000881af push_global_vptr [3] 
0007 @0x000881b3 assign_value [] 
0008 @0x000881b7 pop_stack [] 
0009 @0x000881bb push_number(S) [0] 
0010 @0x000881bf push_global_vptr [4] 
0011 @0x000881c3 assign_value [] 
0012 @0x000881c7 pop_stack [] 
0013 @0x000881cb push_shared_string [770] '/xdbc/'
0014 @0x000881cf push_global_vptr [5] 
0015 @0x000881d3 assign_value [] 
0016 @0x000881d7 pop_stack [] 
0017 @0x000881db create_mapping [0] 
0018 @0x000881df push_global_vptr [6] 
0019 @0x000881e3 assign_value [] 
0020 @0x000881e7 pop_stack [] 
0021 @0x000881eb push_number(S) [0] 
0022 @0x000881ef push_global_vptr [7] 
0023 @0x000881f3 assign_value [] 
0024 @0x000881f7 pop_stack [] 
0025 @0x000881fb push_shared_string [154] '/ccs/log/'
0026 @0x000881ff call_in_efun [336, 1] mkdir
0027 @0x00088203 pop_stack [] 
0028 @0x00088207 push_shared_string [771] '/ccs/log/xdbc_trace'
0029 @0x0008820b push_shared_string [56] 'trace'
0030 @0x0008820f call_in_user_fun [583, -126] xdbc_set_output_basic_file
0031 @0x00088213 pop_stack [] 
0032 @0x00088217 call_in_user_fun [585, -128] xdbc_enable_log_trace
0033 @0x0008821b pop_stack [] 
0034 @0x0008821f push_number(S) [0] 
0035 @0x00088223 push_shared_string [772] 'xdbc'
0036 @0x00088227 call_in_efun [27, 2] enable_trace
0037 @0x0008822b pop_stack [] 
0038 @0x0008822f create_mapping [0] 
0039 @0x00088233 push_global_vptr [8] 
0040 @0x00088237 assign_value [] 
0041 @0x0008823b pop_stack [] 
0042 @0x0008823f push_shared_string [773] 'xdbc_connect'
0043 @0x00088243 push_shared_string [65] 'mysql_connect'
0044 @0x00088247 push_shared_string [774] 'xdbc_disconnect'
0045 @0x0008824b push_shared_string [66] 'mysql_disconnect'
0046 @0x0008824f push_shared_string [775] 'xdbc_is_connected'
0047 @0x00088253 push_shared_string [67] 'mysql_is_connected'
0048 @0x00088257 push_shared_string [776] 'xdbc_get_error'
0049 @0x0008825b push_shared_string [68] 'mysql_get_error'
0050 @0x0008825f push_shared_string [777] 'xdbc_execute'
0051 @0x00088263 push_shared_string [69] 'mysql_execute'
0052 @0x00088267 push_shared_string [778] 'xdbc_query'
0053 @0x0008826b push_shared_string [70] 'mysql_query'
0054 @0x0008826f push_shared_string [779] 'xdbc_finalize'
0055 @0x00088273 push_shared_string [71] 'mysql_finalize'
0056 @0x00088277 push_shared_string [780] 'xdbc_num_records'
0057 @0x0008827b push_shared_string [72] 'mysql_num_records'
0058 @0x0008827f push_shared_string [781] 'xdbc_fetch_record'
0059 @0x00088283 push_shared_string [73] 'mysql_fetch_record'
0060 @0x00088287 push_shared_string [782] 'xdbc_this_record'
0061 @0x0008828b push_shared_string [74] 'mysql_this_record'
0062 @0x0008828f push_shared_string [783] 'xdbc_field_value'
0063 @0x00088293 push_shared_string [75] 'mysql_field_value'
0064 @0x00088297 push_shared_string [784] 'xdbc_field_name'
0065 @0x0008829b push_shared_string [78] 'mysql_field_name'
0066 @0x0008829f push_shared_string [785] 'xdbc_get_insert_id'
0067 @0x000882a3 push_shared_string [81] 'mysql_get_insert_id'
0068 @0x000882a7 push_shared_string [786] 'NO_IMPLEMENTED'
0069 @0x000882ab push_shared_string [76] 'mysql_set_auto_commit'
0070 @0x000882af push_shared_string [786] 'NO_IMPLEMENTED'
0071 @0x000882b3 push_shared_string [77] 'mysql_select_db'
0072 @0x000882b7 push_shared_string [786] 'NO_IMPLEMENTED'
0073 @0x000882bb push_shared_string [79] 'mysql_num_fields'
0074 @0x000882bf push_shared_string [786] 'NO_IMPLEMENTED'
0075 @0x000882c3 push_shared_string [80] 'mysql_int_field'
0076 @0x000882c7 create_mapping [34] 
0077 @0x000882cb push_global_vptr [9] 
0078 @0x000882cf assign_value [] 
0079 @0x000882d3 pop_stack [] 
0080 @0x000882d7 call_in_user_fun [604, -128] init_xdbc_client
0081 @0x000882db pop_stack [] 
0082 @0x000882df push_shared_string [787] 'CONFIG'
0083 @0x000882e3 call_in_efun [22, 1] expand_macro
0084 @0x000882e7 if [3, 2] 
0085 @0x000882eb push_shared_string [787] 'CONFIG'
0086 @0x000882ef call_in_efun [22, 1] expand_macro
0087 @0x000882f3 goto [1] 
0088 @0x000882f7 push_shared_string [788] '/ccs/ccs.ini'
0089 @0x000882fb push_global_vptr [27] 
0090 @0x000882ff assign_value [] 
0091 @0x00088303 pop_stack [] 
0092 @0x00088307 create_mapping [0] 
0093 @0x0008830b push_global_vptr [28] 
0094 @0x0008830f assign_value [] 
0095 @0x00088313 pop_stack [] 
0096 @0x00088317 create_mapping [0] 
0097 @0x0008831b push_global_vptr [29] 
0098 @0x0008831f assign_value [] 
0099 @0x00088323 pop_stack [] 
0100 @0x00088327 push_shared_string [789] '__BIN_DIR__'
0101 @0x0008832b call_in_efun [22, 1] expand_macro
0102 @0x0008832f call_in_efun [141, 1] trim
0103 @0x00088333 push_global_vptr [30] 
0104 @0x00088337 assign_value [] 
0105 @0x0008833b pop_stack [] 
0106 @0x0008833f push_global_var [30] 
0107 @0x00088343 push_number(S) [1] 
0108 @0x00088347 push_number(S) [2] 
0109 @0x0008834b query_range [1] 
0110 @0x0008834f push_global_vptr [30] 
0111 @0x00088353 assign_value [] 
0112 @0x00088357 pop_stack [] 
0113 @0x0008835b push_global_var [30] 
0114 @0x0008835f push_number(S) [1] 
0115 @0x00088363 query_member [1] 
0116 @0x00088367 push_number(S) [47] 
0117 @0x0008836b eq_value [] 
0118 @0x0008836f if [7, 2] 
0119 @0x00088373 push_global_var [30] 
0120 @0x00088377 push_number(S) [0] 
0121 @0x0008837b push_number(S) [2] 
0122 @0x0008837f query_range [1] 
0123 @0x00088383 push_global_vptr [30] 
0124 @0x00088387 assign_value [] 
0125 @0x0008838b pop_stack [] 
0126 @0x0008838f create_array [0] 
0127 @0x00088393 push_global_vptr [31] 
0128 @0x00088397 assign_value [] 
0129 @0x0008839b pop_stack [] 
0130 @0x0008839f create_array [0] 
0131 @0x000883a3 push_global_vptr [32] 
0132 @0x000883a7 assign_value [] 
0133 @0x000883ab pop_stack [] 
0134 @0x000883af create_array [0] 
0135 @0x000883b3 push_global_vptr [33] 
0136 @0x000883b7 assign_value [] 
0137 @0x000883bb pop_stack [] 
0138 @0x000883bf create_array [0] 
0139 @0x000883c3 push_global_vptr [34] 
0140 @0x000883c7 assign_value [] 
0141 @0x000883cb pop_stack [] 
0142 @0x000883cf create_array [0] 
0143 @0x000883d3 push_global_vptr [35] 
0144 @0x000883d7 assign_value [] 
0145 @0x000883db pop_stack [] 
0146 @0x000883df create_array [0] 
0147 @0x000883e3 push_global_vptr [36] 
0148 @0x000883e7 assign_value [] 
0149 @0x000883eb pop_stack [] 
0150 @0x000883ef create_array [0] 
0151 @0x000883f3 push_global_vptr [37] 
0152 @0x000883f7 assign_value [] 
0153 @0x000883fb pop_stack [] 
0154 @0x000883ff create_array [0] 
0155 @0x00088403 push_global_vptr [38] 
0156 @0x00088407 assign_value [] 
0157 @0x0008840b pop_stack [] 
0158 @0x0008840f create_array [0] 
0159 @0x00088413 push_global_vptr [39] 
0160 @0x00088417 assign_value [] 
0161 @0x0008841b pop_stack [] 
0162 @0x0008841f create_array [0] 
0163 @0x00088423 push_global_vptr [40] 
0164 @0x00088427 assign_value [] 
0165 @0x0008842b pop_stack [] 
0166 @0x0008842f create_array [0] 
0167 @0x00088433 push_global_vptr [41] 
0168 @0x00088437 assign_value [] 
0169 @0x0008843b pop_stack [] 
0170 @0x0008843f create_array [0] 
0171 @0x00088443 push_global_vptr [42] 
0172 @0x00088447 assign_value [] 
0173 @0x0008844b pop_stack [] 
0174 @0x0008844f create_mapping [0] 
0175 @0x00088453 push_global_vptr [43] 
0176 @0x00088457 assign_value [] 
0177 @0x0008845b pop_stack [] 
0178 @0x0008845f create_array [0] 
0179 @0x00088463 push_global_vptr [44] 
0180 @0x00088467 assign_value [] 
0181 @0x0008846b pop_stack [] 
0182 @0x0008846f create_array [0] 
0183 @0x00088473 push_global_vptr [45] 
0184 @0x00088477 assign_value [] 
0185 @0x0008847b pop_stack [] 
0186 @0x0008847f create_array [0] 
0187 @0x00088483 push_global_vptr [46] 
0188 @0x00088487 assign_value [] 
0189 @0x0008848b pop_stack [] 
0190 @0x0008848f create_mapping [0] 
0191 @0x00088493 push_global_vptr [47] 
0192 @0x00088497 assign_value [] 
0193 @0x0008849b pop_stack [] 
0194 @0x0008849f push_number(S) [-1] 
0195 @0x000884a3 push_global_vptr [48] 
0196 @0x000884a7 assign_value [] 
0197 @0x000884ab pop_stack [] 
0198 @0x000884af push_number(S) [0] 
0199 @0x000884b3 push_global_vptr [49] 
0200 @0x000884b7 assign_value [] 
0201 @0x000884bb pop_stack [] 
0202 @0x000884bf push_shared_string [160] 'update_today_zero_time'
0203 @0x000884c3 push_number(S) [0] 
0204 @0x000884c7 push_number(S) [0] 
0205 @0x000884cb call_in_efun [327, 3] set_timer
0206 @0x000884cf pop_stack [] 
0207 @0x000884d3 create_mapping [0] 
0208 @0x000884d7 push_global_vptr [50] 
0209 @0x000884db assign_value [] 
0210 @0x000884df pop_stack [] 
0211 @0x000884e3 push_number(S) [45] 
0212 @0x000884e7 push_global_vptr [51] 
0213 @0x000884eb assign_value [] 
0214 @0x000884ef pop_stack [] 
0215 @0x000884f3 push_number(S) [-1] 
0216 @0x000884f7 push_global_vptr [52] 
0217 @0x000884fb assign_value [] 
0218 @0x000884ff pop_stack [] 
0219 @0x00088503 push_number(S) [1] 
0220 @0x00088507 push_global_vptr [53] 
0221 @0x0008850b assign_value [] 
0222 @0x0008850f pop_stack [] 
0223 @0x00088513 create_mapping [0] 
0224 @0x00088517 push_global_vptr [54] 
0225 @0x0008851b assign_value [] 
0226 @0x0008851f pop_stack [] 
0227 @0x00088523 create_mapping [0] 
0228 @0x00088527 push_global_vptr [55] 
0229 @0x0008852b assign_value [] 
0230 @0x0008852f pop_stack [] 
0231 @0x00088533 push_number(S) [0] 
0232 @0x00088537 push_global_vptr [56] 
0233 @0x0008853b assign_value [] 
0234 @0x0008853f pop_stack [] 
0235 @0x00088543 push_shared_string [790] '癸亥'
0236 @0x00088547 push_shared_string [791] '壬戌'
0237 @0x0008854b push_shared_string [792] '辛酉'
0238 @0x0008854f push_shared_string [793] '庚申'
0239 @0x00088553 push_shared_string [794] '己未'
0240 @0x00088557 push_shared_string [795] '戊午'
0241 @0x0008855b push_shared_string [796] '丁巳'
0242 @0x0008855f push_shared_string [797] '丙辰'
0243 @0x00088563 push_shared_string [798] '乙卯'
0244 @0x00088567 push_shared_string [799] '甲寅'
0245 @0x0008856b push_shared_string [800] '癸丑'
0246 @0x0008856f push_shared_string [801] '壬子'
0247 @0x00088573 push_shared_string [802] '辛亥'
0248 @0x00088577 push_shared_string [803] '庚戌'
0249 @0x0008857b push_shared_string [804] '己酉'
0250 @0x0008857f push_shared_string [805] '戊申'
0251 @0x00088583 push_shared_string [806] '丁未'
0252 @0x00088587 push_shared_string [807] '丙午'
0253 @0x0008858b push_shared_string [808] '乙巳'
0254 @0x0008858f push_shared_string [809] '甲辰'
0255 @0x00088593 push_shared_string [810] '癸卯'
0256 @0x00088597 push_shared_string [811] '壬寅'
0257 @0x0008859b push_shared_string [812] '辛丑'
0258 @0x0008859f push_shared_string [813] '庚子'
0259 @0x000885a3 push_shared_string [814] '己亥'
0260 @0x000885a7 push_shared_string [815] '戊戌'
0261 @0x000885ab push_shared_string [816] '丁酉'
0262 @0x000885af push_shared_string [817] '丙申'
0263 @0x000885b3 push_shared_string [818] '乙未'
0264 @0x000885b7 push_shared_string [819] '甲午'
0265 @0x000885bb push_shared_string [820] '癸巳'
0266 @0x000885bf push_shared_string [821] '壬辰'
0267 @0x000885c3 push_shared_string [822] '辛卯'
0268 @0x000885c7 push_shared_string [823] '庚寅'
0269 @0x000885cb push_shared_string [824] '己丑'
0270 @0x000885cf push_shared_string [825] '戊子'
0271 @0x000885d3 push_shared_string [826] '丁亥'
0272 @0x000885d7 push_shared_string [827] '丙戌'
0273 @0x000885db push_shared_string [828] '乙酉'
0274 @0x000885df push_shared_string [829] '甲申'
0275 @0x000885e3 push_shared_string [830] '癸未'
0276 @0x000885e7 push_shared_string [831] '壬午'
0277 @0x000885eb push_shared_string [832] '辛巳'
0278 @0x000885ef push_shared_string [833] '庚辰'
0279 @0x000885f3 push_shared_string [834] '己卯'
0280 @0x000885f7 push_shared_string [835] '戊寅'
0281 @0x000885fb push_shared_string [836] '丁丑'
0282 @0x000885ff push_shared_string [837] '丙子'
0283 @0x00088603 push_shared_string [838] '乙亥'
0284 @0x00088607 push_shared_string [839] '甲戌'
0285 @0x0008860b push_shared_string [840] '癸酉'
0286 @0x0008860f push_shared_string [841] '壬申'
0287 @0x00088613 push_shared_string [842] '辛未'
0288 @0x00088617 push_shared_string [843] '庚午'
0289 @0x0008861b push_shared_string [844] '己巳'
0290 @0x0008861f push_shared_string [845] '戊辰'
0291 @0x00088623 push_shared_string [846] '丁卯'
0292 @0x00088627 push_shared_string [847] '丙寅'
0293 @0x0008862b push_shared_string [848] '乙丑'
0294 @0x0008862f push_shared_string [849] '甲子'
0295 @0x00088633 create_array [60] 
0296 @0x00088637 push_global_vptr [57] 
0297 @0x0008863b assign_value [] 
0298 @0x0008863f pop_stack [] 
0299 @0x00088643 push_shared_string [325] '癸'
0300 @0x00088647 push_shared_string [323] '壬'
0301 @0x0008864b push_shared_string [321] '辛'
0302 @0x0008864f push_shared_string [319] '庚'
0303 @0x00088653 push_shared_string [317] '己'
0304 @0x00088657 push_shared_string [324] '戊'
0305 @0x0008865b push_shared_string [322] '丁'
0306 @0x0008865f push_shared_string [320] '丙'
0307 @0x00088663 push_shared_string [318] '乙'
0308 @0x00088667 push_shared_string [316] '甲'
0309 @0x0008866b create_array [10] 
0310 @0x0008866f push_global_vptr [58] 
0311 @0x00088673 assign_value [] 
0312 @0x00088677 pop_stack [] 
0313 @0x0008867b push_shared_string [850] '亥'
0314 @0x0008867f push_shared_string [851] '戌'
0315 @0x00088683 push_shared_string [852] '酉'
0316 @0x00088687 push_shared_string [853] '申'
0317 @0x0008868b push_shared_string [854] 'δ'
0318 @0x0008868f push_shared_string [855] '午'
0319 @0x00088693 push_shared_string [856] '巳'
0320 @0x00088697 push_shared_string [857] '辰'
0321 @0x0008869b push_shared_string [858] 'î'
0322 @0x0008869f push_shared_string [859] '寅'
0323 @0x000886a3 push_shared_string [860] '丑'
0324 @0x000886a7 push_shared_string [861] '子'
0325 @0x000886ab create_array [12] 
0326 @0x000886af push_global_vptr [59] 
0327 @0x000886b3 assign_value [] 
0328 @0x000886b7 pop_stack [] 
0329 @0x000886bb push_number(S) [45] 
0330 @0x000886bf push_shared_string [233] 'year'
0331 @0x000886c3 push_number(S) [12] 
0332 @0x000886c7 push_shared_string [862] 'month'
0333 @0x000886cb push_number(S) [17] 
0334 @0x000886cf push_shared_string [327] 'day'
0335 @0x000886d3 push_number(S) [28] 
0336 @0x000886d7 push_shared_string [158] 'hour'
0337 @0x000886db create_mapping [8] 
0338 @0x000886df push_global_vptr [60] 
0339 @0x000886e3 assign_value [] 
0340 @0x000886e7 pop_stack [] 
0341 @0x000886eb push_number(S) [31] 
0342 @0x000886ef push_number(S) [30] 
0343 @0x000886f3 push_number(S) [31] 
0344 @0x000886f7 push_number(S) [30] 
0345 @0x000886fb push_number(S) [31] 
0346 @0x000886ff push_number(S) [31] 
0347 @0x00088703 push_number(S) [30] 
0348 @0x00088707 push_number(S) [31] 
0349 @0x0008870b push_number(S) [30] 
0350 @0x0008870f push_number(S) [31] 
0351 @0x00088713 push_number(S) [28] 
0352 @0x00088717 push_number(S) [31] 
0353 @0x0008871b create_array [12] 
0354 @0x0008871f push_global_vptr [61] 
0355 @0x00088723 assign_value [] 
0356 @0x00088727 pop_stack [] 
0357 @0x0008872b push_number(S) [25] 
0358 @0x0008872f push_number(S) [24] 
0359 @0x00088733 push_number(S) [23] 
0360 @0x00088737 push_number(S) [22] 
0361 @0x0008873b push_number(S) [21] 
0362 @0x0008873f push_number(S) [19] 
0363 @0x00088743 push_number(S) [18] 
0364 @0x00088747 push_number(S) [17] 
0365 @0x0008874b push_number(S) [16] 
0366 @0x0008874f push_number(S) [15] 
0367 @0x00088753 push_number(S) [14] 
0368 @0x00088757 push_number(S) [13] 
0369 @0x0008875b push_number(S) [12] 
0370 @0x0008875f push_number(S) [11] 
0371 @0x00088763 push_number(S) [10] 
0372 @0x00088767 push_number(S) [9] 
0373 @0x0008876b push_number(S) [8] 
0374 @0x0008876f push_number(S) [7] 
0375 @0x00088773 push_number(S) [6] 
0376 @0x00088777 push_number(S) [5] 
0377 @0x0008877b push_number(S) [4] 
0378 @0x0008877f push_number(S) [3] 
0379 @0x00088783 push_number(S) [0] 
0380 @0x00088787 create_array [23] 
0381 @0x0008878b push_global_vptr [62] 
0382 @0x0008878f assign_value [] 
0383 @0x00088793 pop_stack [] 
0384 @0x00088797 push_shared_string [262] ', '
0385 @0x0008879b push_global_var [62] 
0386 @0x0008879f call_in_efun [54, 2] implode
0387 @0x000887a3 push_global_vptr [63] 
0388 @0x000887a7 assign_value [] 
0389 @0x000887ab pop_stack [] 
0390 @0x000887af push_number(S) [0] 
0391 @0x000887b3 push_global_vptr [64] 
0392 @0x000887b7 assign_value [] 
0393 @0x000887bb pop_stack [] 
0394 @0x000887bf push_number(S) [29] 
0395 @0x000887c3 push_global_vptr [65] 
0396 @0x000887c7 assign_value [] 
0397 @0x000887cb pop_stack [] 
0398 @0x000887cf push_number(S) [27] 
0399 @0x000887d3 push_global_vptr [66] 
0400 @0x000887d7 assign_value [] 
0401 @0x000887db pop_stack [] 
0402 @0x000887df push_number(S) [1] 
0403 @0x000887e3 push_global_vptr [67] 
0404 @0x000887e7 assign_value [] 
0405 @0x000887eb pop_stack [] 
0406 @0x000887ef push_number(S) [3] 
0407 @0x000887f3 push_global_vptr [68] 
0408 @0x000887f7 assign_value [] 
0409 @0x000887fb pop_stack [] 
0410 @0x000887ff push_number(S) [-1] 
0411 @0x00088803 push_global_vptr [69] 
0412 @0x00088807 assign_value [] 
0413 @0x0008880b pop_stack [] 
0414 @0x0008880f create_mapping [0] 
0415 @0x00088813 push_global_vptr [70] 
0416 @0x00088817 assign_value [] 
0417 @0x0008881b pop_stack [] 
0418 @0x0008881f create_mapping [0] 
0419 @0x00088823 push_global_vptr [71] 
0420 @0x00088827 assign_value [] 
0421 @0x0008882b pop_stack [] 
0422 @0x0008882f push_number(S) [0] 
0423 @0x00088833 push_global_vptr [72] 
0424 @0x00088837 assign_value [] 
0425 @0x0008883b pop_stack [] 
0426 @0x0008883f create_mapping [0] 
0427 @0x00088843 push_global_vptr [73] 
0428 @0x00088847 assign_value [] 
0429 @0x0008884b pop_stack [] 
0430 @0x0008884f create_mapping [0] 
0431 @0x00088853 push_global_vptr [74] 
0432 @0x00088857 assign_value [] 
0433 @0x0008885b pop_stack [] 
0434 @0x0008885f create_mapping [0] 
0435 @0x00088863 push_global_vptr [75] 
0436 @0x00088867 assign_value [] 
0437 @0x0008886b pop_stack [] 
0438 @0x0008886f push_number(S) [0] 
0439 @0x00088873 push_global_vptr [76] 
0440 @0x00088877 assign_value [] 
0441 @0x0008887b pop_stack [] 
0442 @0x0008887f push_number(S) [0] 
0443 @0x00088883 push_global_vptr [77] 
0444 @0x00088887 assign_value [] 
0445 @0x0008888b pop_stack [] 
0446 @0x0008888f create_mapping [0] 
0447 @0x00088893 push_global_vptr [78] 
0448 @0x00088897 assign_value [] 
0449 @0x0008889b pop_stack [] 
0450 @0x0008889f create_mapping [0] 
0451 @0x000888a3 push_global_vptr [79] 
0452 @0x000888a7 assign_value [] 
0453 @0x000888ab pop_stack [] 
0454 @0x000888af push_shared_string [135] ''
0455 @0x000888b3 push_global_vptr [80] 
0456 @0x000888b7 assign_value [] 
0457 @0x000888bb pop_stack [] 
0458 @0x000888bf push_shared_string [135] ''
0459 @0x000888c3 push_global_vptr [81] 
0460 @0x000888c7 assign_value [] 
0461 @0x000888cb pop_stack [] 
0462 @0x000888cf push_number(S) [2] 
0463 @0x000888d3 push_global_vptr [82] 
0464 @0x000888d7 assign_value [] 
0465 @0x000888db pop_stack [] 
0466 @0x000888df push_number(S) [0] 
0467 @0x000888e3 push_global_vptr [83] 
0468 @0x000888e7 assign_value [] 
0469 @0x000888eb pop_stack [] 
0470 @0x000888ef create_mapping [0] 
0471 @0x000888f3 push_global_vptr [84] 
0472 @0x000888f7 assign_value [] 
0473 @0x000888fb pop_stack [] 
0474 @0x000888ff push_number(S) [0] 
0475 @0x00088903 push_global_vptr [89] 
0476 @0x00088907 assign_value [] 
0477 @0x0008890b pop_stack [] 
0478 @0x0008890f push_number(S) [0] 
0479 @0x00088913 push_global_vptr [90] 
0480 @0x00088917 assign_value [] 
0481 @0x0008891b pop_stack [] 
0482 @0x0008891f push_number(L) [256000] 
0483 @0x00088923 call_in_efun [2, 1] allocate_mapping
0484 @0x00088927 push_global_vptr [91] 
0485 @0x0008892b assign_value [] 
0486 @0x0008892f pop_stack [] 
0487 @0x00088933 push_number(S) [0] 
0488 @0x00088937 push_number(S) [2] 
0489 @0x0008893b push_shared_string [863] 'internal_comm'
0490 @0x0008893f push_number(S) [1] 
0491 @0x00088943 push_shared_string [864] 'client_user'
0492 @0x00088947 push_number(S) [4] 
0493 @0x0008894b push_shared_string [865] 'from_server'
0494 @0x0008894f create_mapping [6] 
0495 @0x00088953 call_in_efun [105, 2] set_case_check
0496 @0x00088957 push_global_vptr [92] 
0497 @0x0008895b assign_value [] 
0498 @0x0008895f pop_stack [] 
0499 @0x00088963 create_mapping [0] 
0500 @0x00088967 push_global_vptr [93] 
0501 @0x0008896b assign_value [] 
0502 @0x0008896f pop_stack [] 
0503 @0x00088973 push_number(S) [1] 
0504 @0x00088977 push_shared_string [866] 'msg_intersvr_rpc_ret'
0505 @0x0008897b create_mapping [2] 
0506 @0x0008897f push_number(S) [9] 
0507 @0x00088983 push_number(S) [1] 
0508 @0x00088987 push_shared_string [867] 'msg_treasure_trading_oper'
0509 @0x0008898b push_number(S) [1] 
0510 @0x0008898f push_shared_string [868] 'cmd_cta_to_server'
0511 @0x00088993 create_mapping [4] 
0512 @0x00088997 push_number(S) [16] 
0513 @0x0008899b create_mapping [4] 
0514 @0x0008899f push_global_vptr [94] 
0515 @0x000889a3 assign_value [] 
0516 @0x000889a7 pop_stack [] 
0517 @0x000889ab create_array [0] 
0518 @0x000889af push_global_vptr [95] 
0519 @0x000889b3 assign_value [] 
0520 @0x000889b7 pop_stack [] 
0521 @0x000889bb create_array [0] 
0522 @0x000889bf push_global_vptr [96] 
0523 @0x000889c3 assign_value [] 
0524 @0x000889c7 pop_stack [] 
0525 @0x000889cb create_array [0] 
0526 @0x000889cf push_global_vptr [97] 
0527 @0x000889d3 assign_value [] 
0528 @0x000889d7 pop_stack [] 
0529 @0x000889db push_number(S) [0] 
0530 @0x000889df push_global_vptr [98] 
0531 @0x000889e3 assign_value [] 
0532 @0x000889e7 pop_stack [] 
0533 @0x000889eb create_mapping [0] 
0534 @0x000889ef push_global_vptr [99] 
0535 @0x000889f3 assign_value [] 
0536 @0x000889f7 pop_stack [] 
0537 @0x000889fb call_in_efun [137, 0] time
0538 @0x000889ff push_global_vptr [100] 
0539 @0x00088a03 assign_value [] 
0540 @0x00088a07 pop_stack [] 
0541 @0x00088a0b push_shared_string [869] 'include;ccs;ccs/include'
0542 @0x00088a0f call_in_efun [128, 1] switch_include_dir
0543 @0x00088a13 pop_stack [] 
0544 @0x00088a17 push_shared_string [870] 'full'
0545 @0x00088a1b push_shared_string [871] 'callstack_when_error'
0546 @0x00088a1f call_in_efun [164, 2] config_driver
0547 @0x00088a23 pop_stack [] 
0548 @0x00088a27 call_in_efun [137, 0] time
0549 @0x00088a2b push_global_vptr [101] 
0550 @0x00088a2f assign_value [] 
0551 @0x00088a33 pop_stack [] 
0552 @0x00088a37 push_number(S) [1] 
0553 @0x00088a3b call_in_efun [109, 1] set_log_on
0554 @0x00088a3f pop_stack [] 
0555 @0x00088a43 call_in_user_fun [656, -128] init_startup_module
0556 @0x00088a47 pop_stack [] 
0557 @0x00088a4b call_in_user_fun [805, -128] init_compile_module
0558 @0x00088a4f pop_stack [] 
0559 @0x00088a53 call_in_user_fun [767, -128] init_sql_module
0560 @0x00088a57 pop_stack [] 
0561 @0x00088a5b call_in_user_fun [771, -128] init_debug_cmds_module
0562 @0x00088a5f pop_stack [] 
0563 @0x00088a63 call_in_user_fun [995, -128] init_platform_module
0564 @0x00088a67 pop_stack [] 
0565 @0x00088a6b push_global_var [88] 
0566 @0x00088a6f push_shared_string [872] ' all global feature.\n'
0567 @0x00088a73 add [] 
0568 @0x00088a77 call_in_efun [90, 1] printf
0569 @0x00088a7b pop_stack [] 
0570 @0x00088a7f push_shared_string [873] '/feature/'
0571 @0x00088a83 push_global_var [85] 
0572 @0x00088a87 call_fun_ptr [0, 2] 
0573 @0x00088a8b pop_stack [] 
0574 @0x00088a8f push_global_var [88] 
0575 @0x00088a93 push_shared_string [874] ' all global inherits.\n'
0576 @0x00088a97 add [] 
0577 @0x00088a9b call_in_efun [90, 1] printf
0578 @0x00088a9f pop_stack [] 
0579 @0x00088aa3 push_shared_string [875] '/inherit/'
0580 @0x00088aa7 push_global_var [85] 
0581 @0x00088aab call_fun_ptr [0, 2] 
0582 @0x00088aaf pop_stack [] 
0583 @0x00088ab3 push_global_var [88] 
0584 @0x00088ab7 push_shared_string [876] ' all global objects.\n'
0585 @0x00088abb add [] 
0586 @0x00088abf call_in_efun [90, 1] printf
0587 @0x00088ac3 pop_stack [] 
0588 @0x00088ac7 push_shared_string [877] '/clone/'
0589 @0x00088acb push_global_var [85] 
0590 @0x00088acf call_fun_ptr [0, 2] 
0591 @0x00088ad3 pop_stack [] 
0592 @0x00088ad7 push_global_var [88] 
0593 @0x00088adb push_shared_string [878] ' all global cmds.\n'
0594 @0x00088adf add [] 
0595 @0x00088ae3 call_in_efun [90, 1] printf
0596 @0x00088ae7 pop_stack [] 
0597 @0x00088aeb push_shared_string [879] '/cmds/'
0598 @0x00088aef push_global_var [85] 
0599 @0x00088af3 call_fun_ptr [0, 2] 
0600 @0x00088af7 pop_stack [] 
0601 @0x00088afb push_shared_string [880] '/ccs/cmds/'
0602 @0x00088aff push_global_var [85] 
0603 @0x00088b03 call_fun_ptr [0, 2] 
0604 @0x00088b07 pop_stack [] 
0605 @0x00088b0b push_shared_string [152] '/daemons/configd.c'
0606 @0x00088b0f push_global_var [87] 
0607 @0x00088b13 call_fun_ptr [0, 2] 
0608 @0x00088b17 pop_stack [] 
0609 @0x00088b1b push_shared_string [881] '/daemons/'
0610 @0x00088b1f push_global_var [86] 
0611 @0x00088b23 call_fun_ptr [0, 2] 
0612 @0x00088b27 pop_stack [] 
0613 @0x00088b2b push_shared_string [882] '/ccs/daemons/leavemsgd.c'
0614 @0x00088b2f push_global_var [87] 
0615 @0x00088b33 call_fun_ptr [0, 2] 
0616 @0x00088b37 pop_stack [] 
0617 @0x00088b3b push_shared_string [883] '/ccs/daemons/'
0618 @0x00088b3f push_global_var [86] 
0619 @0x00088b43 call_fun_ptr [0, 2] 
0620 @0x00088b47 pop_stack [] 
0621 @0x00088b4b push_shared_string [884] '/ccs/clone/'
0622 @0x00088b4f push_global_var [85] 
0623 @0x00088b53 call_fun_ptr [0, 2] 
0624 @0x00088b57 pop_stack [] 
0625 @0x00088b5b call_in_efun [67, 0] is_compiling
0626 @0x00088b5f if [13, 2] 
0627 @0x00088b63 push_shared_string [885] '/daemons/data_save/'
0628 @0x00088b67 push_global_var [85] 
0629 @0x00088b6b call_fun_ptr [0, 2] 
0630 @0x00088b6f pop_stack [] 
0631 @0x00088b73 push_shared_string [883] '/ccs/daemons/'
0632 @0x00088b77 push_global_var [85] 
0633 @0x00088b7b call_fun_ptr [0, 2] 
0634 @0x00088b7f pop_stack [] 
0635 @0x00088b83 push_shared_string [884] '/ccs/clone/'
0636 @0x00088b87 push_global_var [85] 
0637 @0x00088b8b call_fun_ptr [0, 2] 
0638 @0x00088b8f pop_stack [] 
0639 @0x00088b93 null_action [] 
0640 @0x00088b97 call_in_user_fun [803, -128] load_dist_start_time
0641 @0x00088b9b pop_stack [] 
0642 @0x00088b9f push_shared_string [886] 'Invoke registered initialization routines.\n'
0643 @0x00088ba3 call_in_efun [90, 1] printf
0644 @0x00088ba7 pop_stack [] 
0645 @0x00088bab call_in_user_fun [629, -128] post_init
0646 @0x00088baf pop_stack [] 
0647 @0x00088bb3 push_number(S) [3] 
0648 @0x00088bb7 call_in_user_fun [799, -127] set_server_status
0649 @0x00088bbb pop_stack [] 
0650 @0x00088bbf call_in_efun [67, 0] is_compiling
0651 @0x00088bc3 not_value [] 
0652 @0x00088bc7 if [6, 2] 
0653 @0x00088bcb push_shared_string [135] ''
0654 @0x00088bcf push_shared_string [887] 'startup'
0655 @0x00088bd3 push_shared_string [769] 'record_log'
0656 @0x00088bd7 push_shared_string [888] '/daemons/log_server_startup_shutdownd.c'
0657 @0x00088bdb call_in_other_fun_ex [4, 2] 
0658 @0x00088bdf pop_stack [] 
0659 @0x00088be3 push_shared_string [889] '\x1b[1;33mDriver load ok!\n\x1b[2;37;0m'
0660 @0x00088be7 call_in_efun [90, 1] printf
0661 @0x00088beb pop_stack [] 
0662 @0x00088bef push_shared_string [890] 'Driver startup\n'
0663 @0x00088bf3 call_in_efun [10, 1] broadcast
0664 @0x00088bf7 pop_stack [] 
0665 @0x00088bfb null_action [] 
0666 @0x00088bff finish_action [] 
```

<a id="ccs-post_init"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: post_init

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x4a073`；函数编号：`629`。

```text
0000 @0x0004a198 check_args_num [0] 
0001 @0x0004a19c enter_function [] 
0002 @0x0004a1a0 push_global_var [31] 
0003 @0x0004a1a4 sizeof [] 
0004 @0x0004a1a8 if [36, 2] 
0005 @0x0004a1ac push_global_var [31] 
0006 @0x0004a1b0 push_local_vptr [-2] 
0007 @0x0004a1b4 assign_value [] 
0008 @0x0004a1b8 pop_stack [] 
0009 @0x0004a1bc create_array [0] 
0010 @0x0004a1c0 push_global_vptr [31] 
0011 @0x0004a1c4 assign_value [] 
0012 @0x0004a1c8 pop_stack [] 
0013 @0x0004a1cc push_local_vptr [-1] 
0014 @0x0004a1d0 push_local_value [-2] 
0015 @0x0004a1d4 push_number(S) [997] 
0016 @0x0004a1d8 init_for_loop [-3] 
0017 @0x0004a1dc cond_jmp_for_loop [14, -3] 
0018 @0x0004a1e0 push_local_value [-1] 
0019 @0x0004a1e4 is_type [9] 
0020 @0x0004a1e8 if [9, 2] 
0021 @0x0004a1ec new_thread_catch [6] 
0022 @0x0004a1f0 push_local_value [-1] 
0023 @0x0004a1f4 call_fun_ptr [4, 1] 
0024 @0x0004a1f8 pop_stack [] 
0025 @0x0004a1fc push_number(S) [0] 
0026 @0x0004a200 end_thread_catch [] 
0027 @0x0004a204 goto [1] 
0028 @0x0004a208 caught_error [] 
0029 @0x0004a20c pop_stack [] 
0030 @0x0004a210 null_action [] 
0031 @0x0004a214 step_jmp_for_loop [-14, -3] 
0032 @0x0004a218 push_global_var [31] 
0033 @0x0004a21c sizeof [] 
0034 @0x0004a220 if [4, 2] 
0035 @0x0004a224 push_number(S) [1] 
0036 @0x0004a228 call_in_efun [298, 1] sleep
0037 @0x0004a22c pop_stack [] 
0038 @0x0004a230 null_action [] 
0039 @0x0004a234 null_action [] 
0040 @0x0004a238 goto [-39] 
0041 @0x0004a23c push_number(S) [0] 
0042 @0x0004a240 push_global_vptr [31] 
0043 @0x0004a244 assign_value [] 
0044 @0x0004a248 pop_stack [] 
0045 @0x0004a24c null_action [] 
0046 @0x0004a250 return_from_fun [] 
```

<a id="ccs-xdbc_query"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: xdbc_query

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x51a89`；函数编号：`593`。

```text
0000 @0x00051bae check_args_num [0] 
0001 @0x00051bb2 enter_function [] 
0002 @0x00051bb6 push_local_value [1] 
0003 @0x00051bba push_local_value [0] 
0004 @0x00051bbe push_shared_string [116] 'Call xdbc_query(%d, %s)\n'
0005 @0x00051bc2 push_number(S) [0] 
0006 @0x00051bc6 call_in_user_fun [584, -124] xdbc_trace
0007 @0x00051bca pop_stack [] 
0008 @0x00051bce push_shared_string [117] 'query'
0009 @0x00051bd2 push_shared_string [39] 'cmd'
0010 @0x00051bd6 push_local_value [1] 
0011 @0x00051bda push_shared_string [115] 'sql_cmd'
0012 @0x00051bde create_mapping [4] 
0013 @0x00051be2 push_local_value [0] 
0014 @0x00051be6 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x00051bea push_local_vptr [-1] 
0016 @0x00051bee assign_value [] 
0017 @0x00051bf2 pop_stack [] 
0018 @0x00051bf6 push_local_value [-1] 
0019 @0x00051bfa push_number(S) [-1] 
0020 @0x00051bfe eq_value [] 
0021 @0x00051c02 if [2, 2] 
0022 @0x00051c06 push_number(S) [-1] 
0023 @0x00051c0a return_from_fun [] 
0024 @0x00051c0e push_local_value [0] 
0025 @0x00051c12 push_shared_string [118] 'cno'
0026 @0x00051c16 create_mapping [2] 
0027 @0x00051c1a push_global_vptr [8] 
0028 @0x00051c1e push_local_value [-1] 
0029 @0x00051c22 query_member_ptr [0] 
0030 @0x00051c26 assign_value [] 
0031 @0x00051c2a pop_stack [] 
0032 @0x00051c2e push_local_value [-1] 
0033 @0x00051c32 return_from_fun [] 
0034 @0x00051c36 null_action [] 
0035 @0x00051c3a return_from_fun [] 
```

<a id="ccs-xdbc_connect"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: xdbc_connect

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x5b425`；函数编号：`588`。

```text
0000 @0x0005b6a3 check_args_num [0] 
0001 @0x0005b6a7 enter_function [] 
0002 @0x0005b6ab push_number(S) [2] 
0003 @0x0005b6af call_in_efun [267, 1] socket
0004 @0x0005b6b3 push_local_vptr [-3] 
0005 @0x0005b6b7 assign_value [] 
0006 @0x0005b6bb pop_stack [] 
0007 @0x0005b6bf push_number(S) [9527] 
0008 @0x0005b6c3 push_local_value [0] 
0009 @0x0005b6c7 push_local_value [-3] 
0010 @0x0005b6cb call_in_efun [254, 3] connect
0011 @0x0005b6cf push_number(S) [0] 
0012 @0x0005b6d3 ne_value [] 
0013 @0x0005b6d7 if [12, 2] 
0014 @0x0005b6db push_number(S) [9527] 
0015 @0x0005b6df push_local_value [0] 
0016 @0x0005b6e3 push_shared_string [99] 'Failed to connect to %s:%d\n'
0017 @0x0005b6e7 push_number(S) [2] 
0018 @0x0005b6eb call_in_user_fun [581, -124] xdbc_log
0019 @0x0005b6ef pop_stack [] 
0020 @0x0005b6f3 push_local_value [-3] 
0021 @0x0005b6f7 call_in_efun [255, 1] close_socket
0022 @0x0005b6fb pop_stack [] 
0023 @0x0005b6ff push_number(S) [-1] 
0024 @0x0005b703 return_from_fun [] 
0025 @0x0005b707 null_action [] 
0026 @0x0005b70b push_local_value [-3] 
0027 @0x0005b70f call_in_user_fun [572, -127] xdbc_allocate_client
0028 @0x0005b713 push_local_vptr [-4] 
0029 @0x0005b717 assign_value [] 
0030 @0x0005b71b push_number(S) [-1] 
0031 @0x0005b71f eq_value [] 
0032 @0x0005b723 if [10, 2] 
0033 @0x0005b727 push_shared_string [100] 'No more free client to allocate.\n'
0034 @0x0005b72b push_number(S) [2] 
0035 @0x0005b72f call_in_user_fun [581, -126] xdbc_log
0036 @0x0005b733 pop_stack [] 
0037 @0x0005b737 push_local_value [-3] 
0038 @0x0005b73b call_in_efun [255, 1] close_socket
0039 @0x0005b73f pop_stack [] 
0040 @0x0005b743 push_number(S) [-1] 
0041 @0x0005b747 return_from_fun [] 
0042 @0x0005b74b null_action [] 
0043 @0x0005b74f push_number(L) [2000000000] 
0044 @0x0005b753 call_in_efun [94, 1] random
0045 @0x0005b757 push_local_vptr [-5] 
0046 @0x0005b75b assign_value [] 
0047 @0x0005b75f pop_stack [] 
0048 @0x0005b763 push_shared_string [101] 'set_key'
0049 @0x0005b767 push_shared_string [39] 'cmd'
0050 @0x0005b76b push_local_value [-5] 
0051 @0x0005b76f push_shared_string [40] 'key'
0052 @0x0005b773 create_mapping [4] 
0053 @0x0005b777 push_local_value [-4] 
0054 @0x0005b77b call_in_user_fun [601, -126] xdbc_call_server
0055 @0x0005b77f push_local_vptr [-2] 
0056 @0x0005b783 assign_value [] 
0057 @0x0005b787 pop_stack [] 
0058 @0x0005b78b push_local_value [-2] 
0059 @0x0005b78f push_number(S) [-1] 
0060 @0x0005b793 eq_value [] 
0061 @0x0005b797 if [7, 1] 
0062 @0x0005b79b pop_stack [] 
0063 @0x0005b79f push_local_value [-4] 
0064 @0x0005b7a3 call_in_user_fun [575, -127] xdbc_get_client
0065 @0x0005b7a7 push_local_vptr [-1] 
0066 @0x0005b7ab assign_value [] 
0067 @0x0005b7af is_type [7] 
0068 @0x0005b7b3 not_value [] 
0069 @0x0005b7b7 if [12, 2] 
0070 @0x0005b7bb push_local_value [-2] 
0071 @0x0005b7bf push_local_value [-4] 
0072 @0x0005b7c3 push_shared_string [102] 'Failed to prepare connection %d(result = %d).\n'
0073 @0x0005b7c7 push_number(S) [2] 
0074 @0x0005b7cb call_in_user_fun [581, -124] xdbc_log
0075 @0x0005b7cf pop_stack [] 
0076 @0x0005b7d3 push_local_value [-4] 
0077 @0x0005b7d7 call_in_user_fun [573, -127] xdbc_free_client
0078 @0x0005b7db pop_stack [] 
0079 @0x0005b7df push_number(S) [-1] 
0080 @0x0005b7e3 return_from_fun [] 
0081 @0x0005b7e7 null_action [] 
0082 @0x0005b7eb push_local_value [-5] 
0083 @0x0005b7ef push_shared_string [103] 'Set key: %016X\n'
0084 @0x0005b7f3 push_number(S) [1] 
0085 @0x0005b7f7 call_in_user_fun [584, -125] xdbc_trace
0086 @0x0005b7fb pop_stack [] 
0087 @0x0005b7ff push_local_value [-5] 
0088 @0x0005b803 push_local_vptr [-1] 
0089 @0x0005b807 push_shared_string [40] 'key'
0090 @0x0005b80b query_member_ptr [0] 
0091 @0x0005b80f assign_value [] 
0092 @0x0005b813 pop_stack [] 
0093 @0x0005b817 push_shared_string [104] 'connect'
0094 @0x0005b81b push_shared_string [39] 'cmd'
0095 @0x0005b81f push_local_value [1] 
0096 @0x0005b823 push_shared_string [105] 'user'
0097 @0x0005b827 push_local_value [2] 
0098 @0x0005b82b push_shared_string [106] 'passwd'
0099 @0x0005b82f push_local_value [3] 
0100 @0x0005b833 push_shared_string [107] 'db'
0101 @0x0005b837 create_mapping [8] 
0102 @0x0005b83b push_local_value [-4] 
0103 @0x0005b83f call_in_user_fun [601, -126] xdbc_call_server
0104 @0x0005b843 push_local_vptr [-2] 
0105 @0x0005b847 assign_value [] 
0106 @0x0005b84b pop_stack [] 
0107 @0x0005b84f push_local_value [-2] 
0108 @0x0005b853 push_number(S) [-1] 
0109 @0x0005b857 eq_value [] 
0110 @0x0005b85b if [10, 2] 
0111 @0x0005b85f push_shared_string [108] 'Failed to connect to database.\n'
0112 @0x0005b863 push_number(S) [2] 
0113 @0x0005b867 call_in_user_fun [581, -126] xdbc_log
0114 @0x0005b86b pop_stack [] 
0115 @0x0005b86f push_local_value [-4] 
0116 @0x0005b873 call_in_user_fun [573, -127] xdbc_free_client
0117 @0x0005b877 pop_stack [] 
0118 @0x0005b87b push_number(S) [-1] 
0119 @0x0005b87f return_from_fun [] 
0120 @0x0005b883 null_action [] 
0121 @0x0005b887 push_local_value [-4] 
0122 @0x0005b88b return_from_fun [] 
0123 @0x0005b88f null_action [] 
0124 @0x0005b893 return_from_fun [] 
```

<a id="ccs-xdbc_execute"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: xdbc_execute

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x5dd6a`；函数编号：`592`。

```text
0000 @0x0005de43 check_args_num [0] 
0001 @0x0005de47 enter_function [] 
0002 @0x0005de4b push_local_value [1] 
0003 @0x0005de4f push_local_value [0] 
0004 @0x0005de53 push_shared_string [113] 'Call xdbc_execute(%d, %s)\n'
0005 @0x0005de57 push_number(S) [0] 
0006 @0x0005de5b call_in_user_fun [584, -124] xdbc_trace
0007 @0x0005de5f pop_stack [] 
0008 @0x0005de63 push_shared_string [114] 'execute'
0009 @0x0005de67 push_shared_string [39] 'cmd'
0010 @0x0005de6b push_local_value [1] 
0011 @0x0005de6f push_shared_string [115] 'sql_cmd'
0012 @0x0005de73 create_mapping [4] 
0013 @0x0005de77 push_local_value [0] 
0014 @0x0005de7b call_in_user_fun [601, -126] xdbc_call_server
0015 @0x0005de7f return_from_fun [] 
0016 @0x0005de83 null_action [] 
0017 @0x0005de87 return_from_fun [] 
```

<a id="ccs-init_startup_module"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: init_startup_module

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x6533a`；函数编号：`656`。

```text
0000 @0x000653ca check_args_num [0] 
0001 @0x000653ce enter_function [] 
0002 @0x000653d2 push_shared_string [154] '/ccs/log/'
0003 @0x000653d6 call_in_user_fun [613, -127] assure_file
0004 @0x000653da pop_stack [] 
0005 @0x000653de push_shared_string [155] 'Done of init_startup_module.\n'
0006 @0x000653e2 call_in_efun [90, 1] printf
0007 @0x000653e6 pop_stack [] 
0008 @0x000653ea null_action [] 
0009 @0x000653ee return_from_fun [] 
```

<a id="ccs-init_sql_module"></a>

## ccs/pack_data/lib_ccs32.pak/ccs/start_ccs.o :: init_sql_module

成员 SHA-256：`256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47`；函数记录偏移：`0x67f63`；函数编号：`767`。

```text
0000 @0x00067ffd check_args_num [0] 
0001 @0x00068001 enter_function [] 
0002 @0x00068005 push_shared_string [390] ':memory:'
0003 @0x00068009 call_in_efun [423, 1] sql_open_db
0004 @0x0006800d push_global_vptr [69] 
0005 @0x00068011 assign_value [] 
0006 @0x00068015 pop_stack [] 
0007 @0x00068019 push_global_var [69] 
0008 @0x0006801d push_number(S) [-1] 
0009 @0x00068021 eq_value [] 
0010 @0x00068025 if [3, 2] 
0011 @0x00068029 push_shared_string [391] 'Failed to open memory database, startup halt.\n'
0012 @0x0006802d call_in_efun [28, 1] error
0013 @0x00068031 pop_stack [] 
0014 @0x00068035 push_shared_string [392] 'Done of init_sql_module.\n'
0015 @0x00068039 call_in_efun [90, 1] printf
0016 @0x0006803d pop_stack [] 
0017 @0x00068041 null_action [] 
0018 @0x00068045 return_from_fun [] 
```

<a id="csa-__init__"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: __init__

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x79ca1`；函数编号：`init`。

```text
0000 @0x0007a1e9 push_number(S) [1024] 
0001 @0x0007a1ed call_in_efun [0, 1] allocate_array
0002 @0x0007a1f1 push_global_vptr [2] 
0003 @0x0007a1f5 assign_value [] 
0004 @0x0007a1f9 pop_stack [] 
0005 @0x0007a1fd push_number(S) [2] 
0006 @0x0007a201 push_global_vptr [3] 
0007 @0x0007a205 assign_value [] 
0008 @0x0007a209 pop_stack [] 
0009 @0x0007a20d push_number(S) [0] 
0010 @0x0007a211 push_global_vptr [4] 
0011 @0x0007a215 assign_value [] 
0012 @0x0007a219 pop_stack [] 
0013 @0x0007a21d push_shared_string [691] '/xdbc/'
0014 @0x0007a221 push_global_vptr [5] 
0015 @0x0007a225 assign_value [] 
0016 @0x0007a229 pop_stack [] 
0017 @0x0007a22d create_mapping [0] 
0018 @0x0007a231 push_global_vptr [6] 
0019 @0x0007a235 assign_value [] 
0020 @0x0007a239 pop_stack [] 
0021 @0x0007a23d push_number(S) [0] 
0022 @0x0007a241 push_global_vptr [7] 
0023 @0x0007a245 assign_value [] 
0024 @0x0007a249 pop_stack [] 
0025 @0x0007a24d push_shared_string [154] '/csa/log/'
0026 @0x0007a251 call_in_efun [336, 1] mkdir
0027 @0x0007a255 pop_stack [] 
0028 @0x0007a259 push_shared_string [692] '/csa/log/xdbc_trace'
0029 @0x0007a25d push_shared_string [56] 'trace'
0030 @0x0007a261 call_in_user_fun [583, -126] xdbc_set_output_basic_file
0031 @0x0007a265 pop_stack [] 
0032 @0x0007a269 call_in_user_fun [585, -128] xdbc_enable_log_trace
0033 @0x0007a26d pop_stack [] 
0034 @0x0007a271 push_number(S) [0] 
0035 @0x0007a275 push_shared_string [693] 'xdbc'
0036 @0x0007a279 call_in_efun [27, 2] enable_trace
0037 @0x0007a27d pop_stack [] 
0038 @0x0007a281 create_mapping [0] 
0039 @0x0007a285 push_global_vptr [8] 
0040 @0x0007a289 assign_value [] 
0041 @0x0007a28d pop_stack [] 
0042 @0x0007a291 push_shared_string [694] 'xdbc_connect'
0043 @0x0007a295 push_shared_string [65] 'mysql_connect'
0044 @0x0007a299 push_shared_string [695] 'xdbc_disconnect'
0045 @0x0007a29d push_shared_string [66] 'mysql_disconnect'
0046 @0x0007a2a1 push_shared_string [696] 'xdbc_is_connected'
0047 @0x0007a2a5 push_shared_string [67] 'mysql_is_connected'
0048 @0x0007a2a9 push_shared_string [697] 'xdbc_get_error'
0049 @0x0007a2ad push_shared_string [68] 'mysql_get_error'
0050 @0x0007a2b1 push_shared_string [698] 'xdbc_execute'
0051 @0x0007a2b5 push_shared_string [69] 'mysql_execute'
0052 @0x0007a2b9 push_shared_string [699] 'xdbc_query'
0053 @0x0007a2bd push_shared_string [70] 'mysql_query'
0054 @0x0007a2c1 push_shared_string [700] 'xdbc_finalize'
0055 @0x0007a2c5 push_shared_string [71] 'mysql_finalize'
0056 @0x0007a2c9 push_shared_string [701] 'xdbc_num_records'
0057 @0x0007a2cd push_shared_string [72] 'mysql_num_records'
0058 @0x0007a2d1 push_shared_string [702] 'xdbc_fetch_record'
0059 @0x0007a2d5 push_shared_string [73] 'mysql_fetch_record'
0060 @0x0007a2d9 push_shared_string [703] 'xdbc_this_record'
0061 @0x0007a2dd push_shared_string [74] 'mysql_this_record'
0062 @0x0007a2e1 push_shared_string [704] 'xdbc_field_value'
0063 @0x0007a2e5 push_shared_string [75] 'mysql_field_value'
0064 @0x0007a2e9 push_shared_string [705] 'xdbc_field_name'
0065 @0x0007a2ed push_shared_string [78] 'mysql_field_name'
0066 @0x0007a2f1 push_shared_string [706] 'xdbc_get_insert_id'
0067 @0x0007a2f5 push_shared_string [81] 'mysql_get_insert_id'
0068 @0x0007a2f9 push_shared_string [707] 'NO_IMPLEMENTED'
0069 @0x0007a2fd push_shared_string [76] 'mysql_set_auto_commit'
0070 @0x0007a301 push_shared_string [707] 'NO_IMPLEMENTED'
0071 @0x0007a305 push_shared_string [77] 'mysql_select_db'
0072 @0x0007a309 push_shared_string [707] 'NO_IMPLEMENTED'
0073 @0x0007a30d push_shared_string [79] 'mysql_num_fields'
0074 @0x0007a311 push_shared_string [707] 'NO_IMPLEMENTED'
0075 @0x0007a315 push_shared_string [80] 'mysql_int_field'
0076 @0x0007a319 create_mapping [34] 
0077 @0x0007a31d push_global_vptr [9] 
0078 @0x0007a321 assign_value [] 
0079 @0x0007a325 pop_stack [] 
0080 @0x0007a329 call_in_user_fun [604, -128] init_xdbc_client
0081 @0x0007a32d pop_stack [] 
0082 @0x0007a331 push_shared_string [708] 'CONFIG'
0083 @0x0007a335 call_in_efun [22, 1] expand_macro
0084 @0x0007a339 if [3, 2] 
0085 @0x0007a33d push_shared_string [708] 'CONFIG'
0086 @0x0007a341 call_in_efun [22, 1] expand_macro
0087 @0x0007a345 goto [1] 
0088 @0x0007a349 push_shared_string [709] '/csa/csa.ini'
0089 @0x0007a34d push_global_vptr [27] 
0090 @0x0007a351 assign_value [] 
0091 @0x0007a355 pop_stack [] 
0092 @0x0007a359 create_mapping [0] 
0093 @0x0007a35d push_global_vptr [28] 
0094 @0x0007a361 assign_value [] 
0095 @0x0007a365 pop_stack [] 
0096 @0x0007a369 create_mapping [0] 
0097 @0x0007a36d push_global_vptr [29] 
0098 @0x0007a371 assign_value [] 
0099 @0x0007a375 pop_stack [] 
0100 @0x0007a379 push_shared_string [710] '__BIN_DIR__'
0101 @0x0007a37d call_in_efun [22, 1] expand_macro
0102 @0x0007a381 call_in_efun [141, 1] trim
0103 @0x0007a385 push_global_vptr [30] 
0104 @0x0007a389 assign_value [] 
0105 @0x0007a38d pop_stack [] 
0106 @0x0007a391 push_global_var [30] 
0107 @0x0007a395 push_number(S) [1] 
0108 @0x0007a399 push_number(S) [2] 
0109 @0x0007a39d query_range [1] 
0110 @0x0007a3a1 push_global_vptr [30] 
0111 @0x0007a3a5 assign_value [] 
0112 @0x0007a3a9 pop_stack [] 
0113 @0x0007a3ad push_global_var [30] 
0114 @0x0007a3b1 push_number(S) [1] 
0115 @0x0007a3b5 query_member [1] 
0116 @0x0007a3b9 push_number(S) [47] 
0117 @0x0007a3bd eq_value [] 
0118 @0x0007a3c1 if [7, 2] 
0119 @0x0007a3c5 push_global_var [30] 
0120 @0x0007a3c9 push_number(S) [0] 
0121 @0x0007a3cd push_number(S) [2] 
0122 @0x0007a3d1 query_range [1] 
0123 @0x0007a3d5 push_global_vptr [30] 
0124 @0x0007a3d9 assign_value [] 
0125 @0x0007a3dd pop_stack [] 
0126 @0x0007a3e1 create_array [0] 
0127 @0x0007a3e5 push_global_vptr [31] 
0128 @0x0007a3e9 assign_value [] 
0129 @0x0007a3ed pop_stack [] 
0130 @0x0007a3f1 create_array [0] 
0131 @0x0007a3f5 push_global_vptr [32] 
0132 @0x0007a3f9 assign_value [] 
0133 @0x0007a3fd pop_stack [] 
0134 @0x0007a401 create_array [0] 
0135 @0x0007a405 push_global_vptr [33] 
0136 @0x0007a409 assign_value [] 
0137 @0x0007a40d pop_stack [] 
0138 @0x0007a411 create_array [0] 
0139 @0x0007a415 push_global_vptr [34] 
0140 @0x0007a419 assign_value [] 
0141 @0x0007a41d pop_stack [] 
0142 @0x0007a421 create_array [0] 
0143 @0x0007a425 push_global_vptr [35] 
0144 @0x0007a429 assign_value [] 
0145 @0x0007a42d pop_stack [] 
0146 @0x0007a431 create_array [0] 
0147 @0x0007a435 push_global_vptr [36] 
0148 @0x0007a439 assign_value [] 
0149 @0x0007a43d pop_stack [] 
0150 @0x0007a441 create_array [0] 
0151 @0x0007a445 push_global_vptr [37] 
0152 @0x0007a449 assign_value [] 
0153 @0x0007a44d pop_stack [] 
0154 @0x0007a451 create_array [0] 
0155 @0x0007a455 push_global_vptr [38] 
0156 @0x0007a459 assign_value [] 
0157 @0x0007a45d pop_stack [] 
0158 @0x0007a461 create_array [0] 
0159 @0x0007a465 push_global_vptr [39] 
0160 @0x0007a469 assign_value [] 
0161 @0x0007a46d pop_stack [] 
0162 @0x0007a471 create_array [0] 
0163 @0x0007a475 push_global_vptr [40] 
0164 @0x0007a479 assign_value [] 
0165 @0x0007a47d pop_stack [] 
0166 @0x0007a481 create_array [0] 
0167 @0x0007a485 push_global_vptr [41] 
0168 @0x0007a489 assign_value [] 
0169 @0x0007a48d pop_stack [] 
0170 @0x0007a491 create_array [0] 
0171 @0x0007a495 push_global_vptr [42] 
0172 @0x0007a499 assign_value [] 
0173 @0x0007a49d pop_stack [] 
0174 @0x0007a4a1 create_mapping [0] 
0175 @0x0007a4a5 push_global_vptr [43] 
0176 @0x0007a4a9 assign_value [] 
0177 @0x0007a4ad pop_stack [] 
0178 @0x0007a4b1 create_array [0] 
0179 @0x0007a4b5 push_global_vptr [44] 
0180 @0x0007a4b9 assign_value [] 
0181 @0x0007a4bd pop_stack [] 
0182 @0x0007a4c1 create_array [0] 
0183 @0x0007a4c5 push_global_vptr [45] 
0184 @0x0007a4c9 assign_value [] 
0185 @0x0007a4cd pop_stack [] 
0186 @0x0007a4d1 create_array [0] 
0187 @0x0007a4d5 push_global_vptr [46] 
0188 @0x0007a4d9 assign_value [] 
0189 @0x0007a4dd pop_stack [] 
0190 @0x0007a4e1 create_mapping [0] 
0191 @0x0007a4e5 push_global_vptr [47] 
0192 @0x0007a4e9 assign_value [] 
0193 @0x0007a4ed pop_stack [] 
0194 @0x0007a4f1 push_number(S) [-1] 
0195 @0x0007a4f5 push_global_vptr [48] 
0196 @0x0007a4f9 assign_value [] 
0197 @0x0007a4fd pop_stack [] 
0198 @0x0007a501 push_number(S) [0] 
0199 @0x0007a505 push_global_vptr [49] 
0200 @0x0007a509 assign_value [] 
0201 @0x0007a50d pop_stack [] 
0202 @0x0007a511 push_shared_string [160] 'update_today_zero_time'
0203 @0x0007a515 push_number(S) [0] 
0204 @0x0007a519 push_number(S) [0] 
0205 @0x0007a51d call_in_efun [327, 3] set_timer
0206 @0x0007a521 pop_stack [] 
0207 @0x0007a525 push_number(S) [45] 
0208 @0x0007a529 push_global_vptr [50] 
0209 @0x0007a52d assign_value [] 
0210 @0x0007a531 pop_stack [] 
0211 @0x0007a535 push_number(S) [-1] 
0212 @0x0007a539 push_global_vptr [51] 
0213 @0x0007a53d assign_value [] 
0214 @0x0007a541 pop_stack [] 
0215 @0x0007a545 push_number(S) [1] 
0216 @0x0007a549 push_global_vptr [52] 
0217 @0x0007a54d assign_value [] 
0218 @0x0007a551 pop_stack [] 
0219 @0x0007a555 create_mapping [0] 
0220 @0x0007a559 push_global_vptr [53] 
0221 @0x0007a55d assign_value [] 
0222 @0x0007a561 pop_stack [] 
0223 @0x0007a565 create_mapping [0] 
0224 @0x0007a569 push_global_vptr [54] 
0225 @0x0007a56d assign_value [] 
0226 @0x0007a571 pop_stack [] 
0227 @0x0007a575 push_number(S) [0] 
0228 @0x0007a579 push_global_vptr [55] 
0229 @0x0007a57d assign_value [] 
0230 @0x0007a581 pop_stack [] 
0231 @0x0007a585 push_shared_string [711] '癸亥'
0232 @0x0007a589 push_shared_string [712] '壬戌'
0233 @0x0007a58d push_shared_string [713] '辛酉'
0234 @0x0007a591 push_shared_string [714] '庚申'
0235 @0x0007a595 push_shared_string [715] '己未'
0236 @0x0007a599 push_shared_string [716] '戊午'
0237 @0x0007a59d push_shared_string [717] '丁巳'
0238 @0x0007a5a1 push_shared_string [718] '丙辰'
0239 @0x0007a5a5 push_shared_string [719] '乙卯'
0240 @0x0007a5a9 push_shared_string [720] '甲寅'
0241 @0x0007a5ad push_shared_string [721] '癸丑'
0242 @0x0007a5b1 push_shared_string [722] '壬子'
0243 @0x0007a5b5 push_shared_string [723] '辛亥'
0244 @0x0007a5b9 push_shared_string [724] '庚戌'
0245 @0x0007a5bd push_shared_string [725] '己酉'
0246 @0x0007a5c1 push_shared_string [726] '戊申'
0247 @0x0007a5c5 push_shared_string [727] '丁未'
0248 @0x0007a5c9 push_shared_string [728] '丙午'
0249 @0x0007a5cd push_shared_string [729] '乙巳'
0250 @0x0007a5d1 push_shared_string [730] '甲辰'
0251 @0x0007a5d5 push_shared_string [731] '癸卯'
0252 @0x0007a5d9 push_shared_string [732] '壬寅'
0253 @0x0007a5dd push_shared_string [733] '辛丑'
0254 @0x0007a5e1 push_shared_string [734] '庚子'
0255 @0x0007a5e5 push_shared_string [735] '己亥'
0256 @0x0007a5e9 push_shared_string [736] '戊戌'
0257 @0x0007a5ed push_shared_string [737] '丁酉'
0258 @0x0007a5f1 push_shared_string [738] '丙申'
0259 @0x0007a5f5 push_shared_string [739] '乙未'
0260 @0x0007a5f9 push_shared_string [740] '甲午'
0261 @0x0007a5fd push_shared_string [741] '癸巳'
0262 @0x0007a601 push_shared_string [742] '壬辰'
0263 @0x0007a605 push_shared_string [743] '辛卯'
0264 @0x0007a609 push_shared_string [744] '庚寅'
0265 @0x0007a60d push_shared_string [745] '己丑'
0266 @0x0007a611 push_shared_string [746] '戊子'
0267 @0x0007a615 push_shared_string [747] '丁亥'
0268 @0x0007a619 push_shared_string [748] '丙戌'
0269 @0x0007a61d push_shared_string [749] '乙酉'
0270 @0x0007a621 push_shared_string [750] '甲申'
0271 @0x0007a625 push_shared_string [751] '癸未'
0272 @0x0007a629 push_shared_string [752] '壬午'
0273 @0x0007a62d push_shared_string [753] '辛巳'
0274 @0x0007a631 push_shared_string [754] '庚辰'
0275 @0x0007a635 push_shared_string [755] '己卯'
0276 @0x0007a639 push_shared_string [756] '戊寅'
0277 @0x0007a63d push_shared_string [757] '丁丑'
0278 @0x0007a641 push_shared_string [758] '丙子'
0279 @0x0007a645 push_shared_string [759] '乙亥'
0280 @0x0007a649 push_shared_string [760] '甲戌'
0281 @0x0007a64d push_shared_string [761] '癸酉'
0282 @0x0007a651 push_shared_string [762] '壬申'
0283 @0x0007a655 push_shared_string [763] '辛未'
0284 @0x0007a659 push_shared_string [764] '庚午'
0285 @0x0007a65d push_shared_string [765] '己巳'
0286 @0x0007a661 push_shared_string [766] '戊辰'
0287 @0x0007a665 push_shared_string [767] '丁卯'
0288 @0x0007a669 push_shared_string [768] '丙寅'
0289 @0x0007a66d push_shared_string [769] '乙丑'
0290 @0x0007a671 push_shared_string [770] '甲子'
0291 @0x0007a675 create_array [60] 
0292 @0x0007a679 push_global_vptr [56] 
0293 @0x0007a67d assign_value [] 
0294 @0x0007a681 pop_stack [] 
0295 @0x0007a685 push_shared_string [323] '癸'
0296 @0x0007a689 push_shared_string [321] '壬'
0297 @0x0007a68d push_shared_string [319] '辛'
0298 @0x0007a691 push_shared_string [317] '庚'
0299 @0x0007a695 push_shared_string [315] '己'
0300 @0x0007a699 push_shared_string [322] '戊'
0301 @0x0007a69d push_shared_string [320] '丁'
0302 @0x0007a6a1 push_shared_string [318] '丙'
0303 @0x0007a6a5 push_shared_string [316] '乙'
0304 @0x0007a6a9 push_shared_string [314] '甲'
0305 @0x0007a6ad create_array [10] 
0306 @0x0007a6b1 push_global_vptr [57] 
0307 @0x0007a6b5 assign_value [] 
0308 @0x0007a6b9 pop_stack [] 
0309 @0x0007a6bd push_shared_string [771] '亥'
0310 @0x0007a6c1 push_shared_string [772] '戌'
0311 @0x0007a6c5 push_shared_string [773] '酉'
0312 @0x0007a6c9 push_shared_string [774] '申'
0313 @0x0007a6cd push_shared_string [775] 'δ'
0314 @0x0007a6d1 push_shared_string [776] '午'
0315 @0x0007a6d5 push_shared_string [777] '巳'
0316 @0x0007a6d9 push_shared_string [778] '辰'
0317 @0x0007a6dd push_shared_string [779] 'î'
0318 @0x0007a6e1 push_shared_string [780] '寅'
0319 @0x0007a6e5 push_shared_string [781] '丑'
0320 @0x0007a6e9 push_shared_string [782] '子'
0321 @0x0007a6ed create_array [12] 
0322 @0x0007a6f1 push_global_vptr [58] 
0323 @0x0007a6f5 assign_value [] 
0324 @0x0007a6f9 pop_stack [] 
0325 @0x0007a6fd push_number(S) [45] 
0326 @0x0007a701 push_shared_string [231] 'year'
0327 @0x0007a705 push_number(S) [12] 
0328 @0x0007a709 push_shared_string [783] 'month'
0329 @0x0007a70d push_number(S) [17] 
0330 @0x0007a711 push_shared_string [325] 'day'
0331 @0x0007a715 push_number(S) [28] 
0332 @0x0007a719 push_shared_string [158] 'hour'
0333 @0x0007a71d create_mapping [8] 
0334 @0x0007a721 push_global_vptr [59] 
0335 @0x0007a725 assign_value [] 
0336 @0x0007a729 pop_stack [] 
0337 @0x0007a72d push_number(S) [31] 
0338 @0x0007a731 push_number(S) [30] 
0339 @0x0007a735 push_number(S) [31] 
0340 @0x0007a739 push_number(S) [30] 
0341 @0x0007a73d push_number(S) [31] 
0342 @0x0007a741 push_number(S) [31] 
0343 @0x0007a745 push_number(S) [30] 
0344 @0x0007a749 push_number(S) [31] 
0345 @0x0007a74d push_number(S) [30] 
0346 @0x0007a751 push_number(S) [31] 
0347 @0x0007a755 push_number(S) [28] 
0348 @0x0007a759 push_number(S) [31] 
0349 @0x0007a75d create_array [12] 
0350 @0x0007a761 push_global_vptr [60] 
0351 @0x0007a765 assign_value [] 
0352 @0x0007a769 pop_stack [] 
0353 @0x0007a76d push_number(S) [25] 
0354 @0x0007a771 push_number(S) [24] 
0355 @0x0007a775 push_number(S) [23] 
0356 @0x0007a779 push_number(S) [22] 
0357 @0x0007a77d push_number(S) [21] 
0358 @0x0007a781 push_number(S) [19] 
0359 @0x0007a785 push_number(S) [18] 
0360 @0x0007a789 push_number(S) [17] 
0361 @0x0007a78d push_number(S) [16] 
0362 @0x0007a791 push_number(S) [15] 
0363 @0x0007a795 push_number(S) [14] 
0364 @0x0007a799 push_number(S) [13] 
0365 @0x0007a79d push_number(S) [12] 
0366 @0x0007a7a1 push_number(S) [11] 
0367 @0x0007a7a5 push_number(S) [10] 
0368 @0x0007a7a9 push_number(S) [9] 
0369 @0x0007a7ad push_number(S) [8] 
0370 @0x0007a7b1 push_number(S) [7] 
0371 @0x0007a7b5 push_number(S) [6] 
0372 @0x0007a7b9 push_number(S) [5] 
0373 @0x0007a7bd push_number(S) [4] 
0374 @0x0007a7c1 push_number(S) [3] 
0375 @0x0007a7c5 push_number(S) [0] 
0376 @0x0007a7c9 create_array [23] 
0377 @0x0007a7cd push_global_vptr [61] 
0378 @0x0007a7d1 assign_value [] 
0379 @0x0007a7d5 pop_stack [] 
0380 @0x0007a7d9 push_shared_string [260] ', '
0381 @0x0007a7dd push_global_var [61] 
0382 @0x0007a7e1 call_in_efun [54, 2] implode
0383 @0x0007a7e5 push_global_vptr [62] 
0384 @0x0007a7e9 assign_value [] 
0385 @0x0007a7ed pop_stack [] 
0386 @0x0007a7f1 push_number(S) [0] 
0387 @0x0007a7f5 push_global_vptr [63] 
0388 @0x0007a7f9 assign_value [] 
0389 @0x0007a7fd pop_stack [] 
0390 @0x0007a801 push_number(S) [29] 
0391 @0x0007a805 push_global_vptr [64] 
0392 @0x0007a809 assign_value [] 
0393 @0x0007a80d pop_stack [] 
0394 @0x0007a811 push_number(S) [27] 
0395 @0x0007a815 push_global_vptr [65] 
0396 @0x0007a819 assign_value [] 
0397 @0x0007a81d pop_stack [] 
0398 @0x0007a821 push_number(S) [1] 
0399 @0x0007a825 push_global_vptr [66] 
0400 @0x0007a829 assign_value [] 
0401 @0x0007a82d pop_stack [] 
0402 @0x0007a831 push_number(S) [3] 
0403 @0x0007a835 push_global_vptr [67] 
0404 @0x0007a839 assign_value [] 
0405 @0x0007a83d pop_stack [] 
0406 @0x0007a841 push_number(S) [-1] 
0407 @0x0007a845 push_global_vptr [68] 
0408 @0x0007a849 assign_value [] 
0409 @0x0007a84d pop_stack [] 
0410 @0x0007a851 create_mapping [0] 
0411 @0x0007a855 push_global_vptr [69] 
0412 @0x0007a859 assign_value [] 
0413 @0x0007a85d pop_stack [] 
0414 @0x0007a861 create_mapping [0] 
0415 @0x0007a865 push_global_vptr [70] 
0416 @0x0007a869 assign_value [] 
0417 @0x0007a86d pop_stack [] 
0418 @0x0007a871 push_number(S) [0] 
0419 @0x0007a875 push_global_vptr [71] 
0420 @0x0007a879 assign_value [] 
0421 @0x0007a87d pop_stack [] 
0422 @0x0007a881 create_mapping [0] 
0423 @0x0007a885 push_global_vptr [72] 
0424 @0x0007a889 assign_value [] 
0425 @0x0007a88d pop_stack [] 
0426 @0x0007a891 create_mapping [0] 
0427 @0x0007a895 push_global_vptr [73] 
0428 @0x0007a899 assign_value [] 
0429 @0x0007a89d pop_stack [] 
0430 @0x0007a8a1 create_mapping [0] 
0431 @0x0007a8a5 push_global_vptr [74] 
0432 @0x0007a8a9 assign_value [] 
0433 @0x0007a8ad pop_stack [] 
0434 @0x0007a8b1 push_number(S) [0] 
0435 @0x0007a8b5 push_global_vptr [75] 
0436 @0x0007a8b9 assign_value [] 
0437 @0x0007a8bd pop_stack [] 
0438 @0x0007a8c1 push_number(S) [0] 
0439 @0x0007a8c5 push_global_vptr [76] 
0440 @0x0007a8c9 assign_value [] 
0441 @0x0007a8cd pop_stack [] 
0442 @0x0007a8d1 create_mapping [0] 
0443 @0x0007a8d5 push_global_vptr [77] 
0444 @0x0007a8d9 assign_value [] 
0445 @0x0007a8dd pop_stack [] 
0446 @0x0007a8e1 create_mapping [0] 
0447 @0x0007a8e5 push_global_vptr [78] 
0448 @0x0007a8e9 assign_value [] 
0449 @0x0007a8ed pop_stack [] 
0450 @0x0007a8f1 push_shared_string [135] ''
0451 @0x0007a8f5 push_global_vptr [79] 
0452 @0x0007a8f9 assign_value [] 
0453 @0x0007a8fd pop_stack [] 
0454 @0x0007a901 push_shared_string [135] ''
0455 @0x0007a905 push_global_vptr [80] 
0456 @0x0007a909 assign_value [] 
0457 @0x0007a90d pop_stack [] 
0458 @0x0007a911 push_number(S) [2] 
0459 @0x0007a915 push_global_vptr [81] 
0460 @0x0007a919 assign_value [] 
0461 @0x0007a91d pop_stack [] 
0462 @0x0007a921 push_number(S) [0] 
0463 @0x0007a925 push_global_vptr [82] 
0464 @0x0007a929 assign_value [] 
0465 @0x0007a92d pop_stack [] 
0466 @0x0007a931 create_mapping [0] 
0467 @0x0007a935 push_global_vptr [83] 
0468 @0x0007a939 assign_value [] 
0469 @0x0007a93d pop_stack [] 
0470 @0x0007a941 push_number(S) [0] 
0471 @0x0007a945 push_number(S) [2] 
0472 @0x0007a949 push_shared_string [784] 'internal_comm'
0473 @0x0007a94d push_number(S) [1] 
0474 @0x0007a951 push_shared_string [785] 'client_user'
0475 @0x0007a955 push_number(S) [4] 
0476 @0x0007a959 push_shared_string [786] 'from_server'
0477 @0x0007a95d create_mapping [6] 
0478 @0x0007a961 call_in_efun [105, 2] set_case_check
0479 @0x0007a965 push_global_vptr [88] 
0480 @0x0007a969 assign_value [] 
0481 @0x0007a96d pop_stack [] 
0482 @0x0007a971 create_mapping [0] 
0483 @0x0007a975 push_global_vptr [89] 
0484 @0x0007a979 assign_value [] 
0485 @0x0007a97d pop_stack [] 
0486 @0x0007a981 push_number(S) [1] 
0487 @0x0007a985 push_shared_string [787] 'msg_intersvr_rpc_ret'
0488 @0x0007a989 create_mapping [2] 
0489 @0x0007a98d push_number(S) [9] 
0490 @0x0007a991 push_number(S) [1] 
0491 @0x0007a995 push_shared_string [788] 'msg_treasure_trading_oper'
0492 @0x0007a999 push_number(S) [1] 
0493 @0x0007a99d push_shared_string [789] 'cmd_cta_to_server'
0494 @0x0007a9a1 create_mapping [4] 
0495 @0x0007a9a5 push_number(S) [16] 
0496 @0x0007a9a9 create_mapping [4] 
0497 @0x0007a9ad push_global_vptr [90] 
0498 @0x0007a9b1 assign_value [] 
0499 @0x0007a9b5 pop_stack [] 
0500 @0x0007a9b9 create_array [0] 
0501 @0x0007a9bd push_global_vptr [91] 
0502 @0x0007a9c1 assign_value [] 
0503 @0x0007a9c5 pop_stack [] 
0504 @0x0007a9c9 create_array [0] 
0505 @0x0007a9cd push_global_vptr [92] 
0506 @0x0007a9d1 assign_value [] 
0507 @0x0007a9d5 pop_stack [] 
0508 @0x0007a9d9 create_array [0] 
0509 @0x0007a9dd push_global_vptr [93] 
0510 @0x0007a9e1 assign_value [] 
0511 @0x0007a9e5 pop_stack [] 
0512 @0x0007a9e9 push_number(S) [0] 
0513 @0x0007a9ed push_global_vptr [94] 
0514 @0x0007a9f1 assign_value [] 
0515 @0x0007a9f5 pop_stack [] 
0516 @0x0007a9f9 create_mapping [0] 
0517 @0x0007a9fd push_global_vptr [95] 
0518 @0x0007aa01 assign_value [] 
0519 @0x0007aa05 pop_stack [] 
0520 @0x0007aa09 call_in_efun [137, 0] time
0521 @0x0007aa0d push_global_vptr [96] 
0522 @0x0007aa11 assign_value [] 
0523 @0x0007aa15 pop_stack [] 
0524 @0x0007aa19 push_shared_string [790] 'include;csa;csa/include'
0525 @0x0007aa1d call_in_efun [128, 1] switch_include_dir
0526 @0x0007aa21 pop_stack [] 
0527 @0x0007aa25 push_shared_string [791] 'full'
0528 @0x0007aa29 push_shared_string [792] 'callstack_when_error'
0529 @0x0007aa2d call_in_efun [164, 2] config_driver
0530 @0x0007aa31 pop_stack [] 
0531 @0x0007aa35 call_in_efun [137, 0] time
0532 @0x0007aa39 push_global_vptr [97] 
0533 @0x0007aa3d assign_value [] 
0534 @0x0007aa41 pop_stack [] 
0535 @0x0007aa45 push_number(S) [1] 
0536 @0x0007aa49 call_in_efun [109, 1] set_log_on
0537 @0x0007aa4d pop_stack [] 
0538 @0x0007aa51 call_in_user_fun [656, -128] init_startup_module
0539 @0x0007aa55 pop_stack [] 
0540 @0x0007aa59 call_in_user_fun [938, -128] init_compile_module
0541 @0x0007aa5d pop_stack [] 
0542 @0x0007aa61 call_in_user_fun [762, -128] init_sql_module
0543 @0x0007aa65 pop_stack [] 
0544 @0x0007aa69 call_in_user_fun [763, -128] init_debug_cmds_module
0545 @0x0007aa6d pop_stack [] 
0546 @0x0007aa71 call_in_user_fun [978, -128] init_platform_module
0547 @0x0007aa75 pop_stack [] 
0548 @0x0007aa79 push_global_var [87] 
0549 @0x0007aa7d push_shared_string [793] ' all global feature.\n'
0550 @0x0007aa81 add [] 
0551 @0x0007aa85 call_in_efun [90, 1] printf
0552 @0x0007aa89 pop_stack [] 
0553 @0x0007aa8d push_shared_string [794] '/feature/'
0554 @0x0007aa91 push_global_var [84] 
0555 @0x0007aa95 call_fun_ptr [0, 2] 
0556 @0x0007aa99 pop_stack [] 
0557 @0x0007aa9d push_global_var [87] 
0558 @0x0007aaa1 push_shared_string [795] ' all global inherits.\n'
0559 @0x0007aaa5 add [] 
0560 @0x0007aaa9 call_in_efun [90, 1] printf
0561 @0x0007aaad pop_stack [] 
0562 @0x0007aab1 push_shared_string [796] '/inherit/'
0563 @0x0007aab5 push_global_var [84] 
0564 @0x0007aab9 call_fun_ptr [0, 2] 
0565 @0x0007aabd pop_stack [] 
0566 @0x0007aac1 push_global_var [87] 
0567 @0x0007aac5 push_shared_string [797] ' all global objects.\n'
0568 @0x0007aac9 add [] 
0569 @0x0007aacd call_in_efun [90, 1] printf
0570 @0x0007aad1 pop_stack [] 
0571 @0x0007aad5 push_shared_string [798] '/clone/'
0572 @0x0007aad9 push_global_var [84] 
0573 @0x0007aadd call_fun_ptr [0, 2] 
0574 @0x0007aae1 pop_stack [] 
0575 @0x0007aae5 push_global_var [87] 
0576 @0x0007aae9 push_shared_string [799] ' all global cmds.\n'
0577 @0x0007aaed add [] 
0578 @0x0007aaf1 call_in_efun [90, 1] printf
0579 @0x0007aaf5 pop_stack [] 
0580 @0x0007aaf9 push_shared_string [800] '/cmds/'
0581 @0x0007aafd push_global_var [84] 
0582 @0x0007ab01 call_fun_ptr [0, 2] 
0583 @0x0007ab05 pop_stack [] 
0584 @0x0007ab09 push_shared_string [801] '/daemons/'
0585 @0x0007ab0d push_global_var [85] 
0586 @0x0007ab11 call_fun_ptr [0, 2] 
0587 @0x0007ab15 pop_stack [] 
0588 @0x0007ab19 push_shared_string [802] '/csa/inherit/misc'
0589 @0x0007ab1d push_global_var [85] 
0590 @0x0007ab21 call_fun_ptr [0, 2] 
0591 @0x0007ab25 pop_stack [] 
0592 @0x0007ab29 push_shared_string [803] '/csa/daemons/logind.c'
0593 @0x0007ab2d push_global_var [86] 
0594 @0x0007ab31 call_fun_ptr [0, 2] 
0595 @0x0007ab35 pop_stack [] 
0596 @0x0007ab39 push_shared_string [804] '/csa/daemons/'
0597 @0x0007ab3d push_global_var [85] 
0598 @0x0007ab41 call_fun_ptr [0, 2] 
0599 @0x0007ab45 pop_stack [] 
0600 @0x0007ab49 push_shared_string [805] '/csa/clone/'
0601 @0x0007ab4d push_global_var [84] 
0602 @0x0007ab51 call_fun_ptr [0, 2] 
0603 @0x0007ab55 pop_stack [] 
0604 @0x0007ab59 call_in_efun [67, 0] is_compiling
0605 @0x0007ab5d if [17, 2] 
0606 @0x0007ab61 push_shared_string [801] '/daemons/'
0607 @0x0007ab65 push_global_var [84] 
0608 @0x0007ab69 call_fun_ptr [0, 2] 
0609 @0x0007ab6d pop_stack [] 
0610 @0x0007ab71 push_shared_string [804] '/csa/daemons/'
0611 @0x0007ab75 push_global_var [84] 
0612 @0x0007ab79 call_fun_ptr [0, 2] 
0613 @0x0007ab7d pop_stack [] 
0614 @0x0007ab81 push_shared_string [806] '/csa/handlers/'
0615 @0x0007ab85 push_global_var [84] 
0616 @0x0007ab89 call_fun_ptr [0, 2] 
0617 @0x0007ab8d pop_stack [] 
0618 @0x0007ab91 push_shared_string [807] '/csa/cmds/'
0619 @0x0007ab95 push_global_var [84] 
0620 @0x0007ab99 call_fun_ptr [0, 2] 
0621 @0x0007ab9d pop_stack [] 
0622 @0x0007aba1 null_action [] 
0623 @0x0007aba5 push_shared_string [808] 'Invoke registered initialization routines.\n'
0624 @0x0007aba9 call_in_efun [90, 1] printf
0625 @0x0007abad pop_stack [] 
0626 @0x0007abb1 call_in_user_fun [629, -128] post_init
0627 @0x0007abb5 pop_stack [] 
0628 @0x0007abb9 push_number(S) [3] 
0629 @0x0007abbd call_in_user_fun [790, -127] set_server_status
0630 @0x0007abc1 pop_stack [] 
0631 @0x0007abc5 push_shared_string [809] '\x1b[1;33mDriver load ok!\n\x1b[2;37;0m'
0632 @0x0007abc9 call_in_efun [90, 1] printf
0633 @0x0007abcd pop_stack [] 
0634 @0x0007abd1 push_shared_string [810] 'Driver startup\n'
0635 @0x0007abd5 call_in_efun [10, 1] broadcast
0636 @0x0007abd9 pop_stack [] 
0637 @0x0007abdd null_action [] 
0638 @0x0007abe1 finish_action [] 
```

<a id="csa-post_init"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: post_init

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x3f163`；函数编号：`629`。

```text
0000 @0x0003f288 check_args_num [0] 
0001 @0x0003f28c enter_function [] 
0002 @0x0003f290 push_global_var [31] 
0003 @0x0003f294 sizeof [] 
0004 @0x0003f298 if [36, 2] 
0005 @0x0003f29c push_global_var [31] 
0006 @0x0003f2a0 push_local_vptr [-2] 
0007 @0x0003f2a4 assign_value [] 
0008 @0x0003f2a8 pop_stack [] 
0009 @0x0003f2ac create_array [0] 
0010 @0x0003f2b0 push_global_vptr [31] 
0011 @0x0003f2b4 assign_value [] 
0012 @0x0003f2b8 pop_stack [] 
0013 @0x0003f2bc push_local_vptr [-1] 
0014 @0x0003f2c0 push_local_value [-2] 
0015 @0x0003f2c4 push_number(S) [997] 
0016 @0x0003f2c8 init_for_loop [-3] 
0017 @0x0003f2cc cond_jmp_for_loop [14, -3] 
0018 @0x0003f2d0 push_local_value [-1] 
0019 @0x0003f2d4 is_type [9] 
0020 @0x0003f2d8 if [9, 2] 
0021 @0x0003f2dc new_thread_catch [6] 
0022 @0x0003f2e0 push_local_value [-1] 
0023 @0x0003f2e4 call_fun_ptr [4, 1] 
0024 @0x0003f2e8 pop_stack [] 
0025 @0x0003f2ec push_number(S) [0] 
0026 @0x0003f2f0 end_thread_catch [] 
0027 @0x0003f2f4 goto [1] 
0028 @0x0003f2f8 caught_error [] 
0029 @0x0003f2fc pop_stack [] 
0030 @0x0003f300 null_action [] 
0031 @0x0003f304 step_jmp_for_loop [-14, -3] 
0032 @0x0003f308 push_global_var [31] 
0033 @0x0003f30c sizeof [] 
0034 @0x0003f310 if [4, 2] 
0035 @0x0003f314 push_number(S) [1] 
0036 @0x0003f318 call_in_efun [298, 1] sleep
0037 @0x0003f31c pop_stack [] 
0038 @0x0003f320 null_action [] 
0039 @0x0003f324 null_action [] 
0040 @0x0003f328 goto [-39] 
0041 @0x0003f32c push_number(S) [0] 
0042 @0x0003f330 push_global_vptr [31] 
0043 @0x0003f334 assign_value [] 
0044 @0x0003f338 pop_stack [] 
0045 @0x0003f33c null_action [] 
0046 @0x0003f340 return_from_fun [] 
```

<a id="csa-xdbc_query"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: xdbc_query

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x4689c`；函数编号：`593`。

```text
0000 @0x000469c1 check_args_num [0] 
0001 @0x000469c5 enter_function [] 
0002 @0x000469c9 push_local_value [1] 
0003 @0x000469cd push_local_value [0] 
0004 @0x000469d1 push_shared_string [116] 'Call xdbc_query(%d, %s)\n'
0005 @0x000469d5 push_number(S) [0] 
0006 @0x000469d9 call_in_user_fun [584, -124] xdbc_trace
0007 @0x000469dd pop_stack [] 
0008 @0x000469e1 push_shared_string [117] 'query'
0009 @0x000469e5 push_shared_string [39] 'cmd'
0010 @0x000469e9 push_local_value [1] 
0011 @0x000469ed push_shared_string [115] 'sql_cmd'
0012 @0x000469f1 create_mapping [4] 
0013 @0x000469f5 push_local_value [0] 
0014 @0x000469f9 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x000469fd push_local_vptr [-1] 
0016 @0x00046a01 assign_value [] 
0017 @0x00046a05 pop_stack [] 
0018 @0x00046a09 push_local_value [-1] 
0019 @0x00046a0d push_number(S) [-1] 
0020 @0x00046a11 eq_value [] 
0021 @0x00046a15 if [2, 2] 
0022 @0x00046a19 push_number(S) [-1] 
0023 @0x00046a1d return_from_fun [] 
0024 @0x00046a21 push_local_value [0] 
0025 @0x00046a25 push_shared_string [118] 'cno'
0026 @0x00046a29 create_mapping [2] 
0027 @0x00046a2d push_global_vptr [8] 
0028 @0x00046a31 push_local_value [-1] 
0029 @0x00046a35 query_member_ptr [0] 
0030 @0x00046a39 assign_value [] 
0031 @0x00046a3d pop_stack [] 
0032 @0x00046a41 push_local_value [-1] 
0033 @0x00046a45 return_from_fun [] 
0034 @0x00046a49 null_action [] 
0035 @0x00046a4d return_from_fun [] 
```

<a id="csa-xdbc_connect"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: xdbc_connect

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x4f928`；函数编号：`588`。

```text
0000 @0x0004fba6 check_args_num [0] 
0001 @0x0004fbaa enter_function [] 
0002 @0x0004fbae push_number(S) [2] 
0003 @0x0004fbb2 call_in_efun [267, 1] socket
0004 @0x0004fbb6 push_local_vptr [-3] 
0005 @0x0004fbba assign_value [] 
0006 @0x0004fbbe pop_stack [] 
0007 @0x0004fbc2 push_number(S) [9527] 
0008 @0x0004fbc6 push_local_value [0] 
0009 @0x0004fbca push_local_value [-3] 
0010 @0x0004fbce call_in_efun [254, 3] connect
0011 @0x0004fbd2 push_number(S) [0] 
0012 @0x0004fbd6 ne_value [] 
0013 @0x0004fbda if [12, 2] 
0014 @0x0004fbde push_number(S) [9527] 
0015 @0x0004fbe2 push_local_value [0] 
0016 @0x0004fbe6 push_shared_string [99] 'Failed to connect to %s:%d\n'
0017 @0x0004fbea push_number(S) [2] 
0018 @0x0004fbee call_in_user_fun [581, -124] xdbc_log
0019 @0x0004fbf2 pop_stack [] 
0020 @0x0004fbf6 push_local_value [-3] 
0021 @0x0004fbfa call_in_efun [255, 1] close_socket
0022 @0x0004fbfe pop_stack [] 
0023 @0x0004fc02 push_number(S) [-1] 
0024 @0x0004fc06 return_from_fun [] 
0025 @0x0004fc0a null_action [] 
0026 @0x0004fc0e push_local_value [-3] 
0027 @0x0004fc12 call_in_user_fun [572, -127] xdbc_allocate_client
0028 @0x0004fc16 push_local_vptr [-4] 
0029 @0x0004fc1a assign_value [] 
0030 @0x0004fc1e push_number(S) [-1] 
0031 @0x0004fc22 eq_value [] 
0032 @0x0004fc26 if [10, 2] 
0033 @0x0004fc2a push_shared_string [100] 'No more free client to allocate.\n'
0034 @0x0004fc2e push_number(S) [2] 
0035 @0x0004fc32 call_in_user_fun [581, -126] xdbc_log
0036 @0x0004fc36 pop_stack [] 
0037 @0x0004fc3a push_local_value [-3] 
0038 @0x0004fc3e call_in_efun [255, 1] close_socket
0039 @0x0004fc42 pop_stack [] 
0040 @0x0004fc46 push_number(S) [-1] 
0041 @0x0004fc4a return_from_fun [] 
0042 @0x0004fc4e null_action [] 
0043 @0x0004fc52 push_number(L) [2000000000] 
0044 @0x0004fc56 call_in_efun [94, 1] random
0045 @0x0004fc5a push_local_vptr [-5] 
0046 @0x0004fc5e assign_value [] 
0047 @0x0004fc62 pop_stack [] 
0048 @0x0004fc66 push_shared_string [101] 'set_key'
0049 @0x0004fc6a push_shared_string [39] 'cmd'
0050 @0x0004fc6e push_local_value [-5] 
0051 @0x0004fc72 push_shared_string [40] 'key'
0052 @0x0004fc76 create_mapping [4] 
0053 @0x0004fc7a push_local_value [-4] 
0054 @0x0004fc7e call_in_user_fun [601, -126] xdbc_call_server
0055 @0x0004fc82 push_local_vptr [-2] 
0056 @0x0004fc86 assign_value [] 
0057 @0x0004fc8a pop_stack [] 
0058 @0x0004fc8e push_local_value [-2] 
0059 @0x0004fc92 push_number(S) [-1] 
0060 @0x0004fc96 eq_value [] 
0061 @0x0004fc9a if [7, 1] 
0062 @0x0004fc9e pop_stack [] 
0063 @0x0004fca2 push_local_value [-4] 
0064 @0x0004fca6 call_in_user_fun [575, -127] xdbc_get_client
0065 @0x0004fcaa push_local_vptr [-1] 
0066 @0x0004fcae assign_value [] 
0067 @0x0004fcb2 is_type [7] 
0068 @0x0004fcb6 not_value [] 
0069 @0x0004fcba if [12, 2] 
0070 @0x0004fcbe push_local_value [-2] 
0071 @0x0004fcc2 push_local_value [-4] 
0072 @0x0004fcc6 push_shared_string [102] 'Failed to prepare connection %d(result = %d).\n'
0073 @0x0004fcca push_number(S) [2] 
0074 @0x0004fcce call_in_user_fun [581, -124] xdbc_log
0075 @0x0004fcd2 pop_stack [] 
0076 @0x0004fcd6 push_local_value [-4] 
0077 @0x0004fcda call_in_user_fun [573, -127] xdbc_free_client
0078 @0x0004fcde pop_stack [] 
0079 @0x0004fce2 push_number(S) [-1] 
0080 @0x0004fce6 return_from_fun [] 
0081 @0x0004fcea null_action [] 
0082 @0x0004fcee push_local_value [-5] 
0083 @0x0004fcf2 push_shared_string [103] 'Set key: %016X\n'
0084 @0x0004fcf6 push_number(S) [1] 
0085 @0x0004fcfa call_in_user_fun [584, -125] xdbc_trace
0086 @0x0004fcfe pop_stack [] 
0087 @0x0004fd02 push_local_value [-5] 
0088 @0x0004fd06 push_local_vptr [-1] 
0089 @0x0004fd0a push_shared_string [40] 'key'
0090 @0x0004fd0e query_member_ptr [0] 
0091 @0x0004fd12 assign_value [] 
0092 @0x0004fd16 pop_stack [] 
0093 @0x0004fd1a push_shared_string [104] 'connect'
0094 @0x0004fd1e push_shared_string [39] 'cmd'
0095 @0x0004fd22 push_local_value [1] 
0096 @0x0004fd26 push_shared_string [105] 'user'
0097 @0x0004fd2a push_local_value [2] 
0098 @0x0004fd2e push_shared_string [106] 'passwd'
0099 @0x0004fd32 push_local_value [3] 
0100 @0x0004fd36 push_shared_string [107] 'db'
0101 @0x0004fd3a create_mapping [8] 
0102 @0x0004fd3e push_local_value [-4] 
0103 @0x0004fd42 call_in_user_fun [601, -126] xdbc_call_server
0104 @0x0004fd46 push_local_vptr [-2] 
0105 @0x0004fd4a assign_value [] 
0106 @0x0004fd4e pop_stack [] 
0107 @0x0004fd52 push_local_value [-2] 
0108 @0x0004fd56 push_number(S) [-1] 
0109 @0x0004fd5a eq_value [] 
0110 @0x0004fd5e if [10, 2] 
0111 @0x0004fd62 push_shared_string [108] 'Failed to connect to database.\n'
0112 @0x0004fd66 push_number(S) [2] 
0113 @0x0004fd6a call_in_user_fun [581, -126] xdbc_log
0114 @0x0004fd6e pop_stack [] 
0115 @0x0004fd72 push_local_value [-4] 
0116 @0x0004fd76 call_in_user_fun [573, -127] xdbc_free_client
0117 @0x0004fd7a pop_stack [] 
0118 @0x0004fd7e push_number(S) [-1] 
0119 @0x0004fd82 return_from_fun [] 
0120 @0x0004fd86 null_action [] 
0121 @0x0004fd8a push_local_value [-4] 
0122 @0x0004fd8e return_from_fun [] 
0123 @0x0004fd92 null_action [] 
0124 @0x0004fd96 return_from_fun [] 
```

<a id="csa-xdbc_execute"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: xdbc_execute

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x5226d`；函数编号：`592`。

```text
0000 @0x00052346 check_args_num [0] 
0001 @0x0005234a enter_function [] 
0002 @0x0005234e push_local_value [1] 
0003 @0x00052352 push_local_value [0] 
0004 @0x00052356 push_shared_string [113] 'Call xdbc_execute(%d, %s)\n'
0005 @0x0005235a push_number(S) [0] 
0006 @0x0005235e call_in_user_fun [584, -124] xdbc_trace
0007 @0x00052362 pop_stack [] 
0008 @0x00052366 push_shared_string [114] 'execute'
0009 @0x0005236a push_shared_string [39] 'cmd'
0010 @0x0005236e push_local_value [1] 
0011 @0x00052372 push_shared_string [115] 'sql_cmd'
0012 @0x00052376 create_mapping [4] 
0013 @0x0005237a push_local_value [0] 
0014 @0x0005237e call_in_user_fun [601, -126] xdbc_call_server
0015 @0x00052382 return_from_fun [] 
0016 @0x00052386 null_action [] 
0017 @0x0005238a return_from_fun [] 
```

<a id="csa-init_startup_module"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: init_startup_module

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x5975f`；函数编号：`656`。

```text
0000 @0x000597ef check_args_num [0] 
0001 @0x000597f3 enter_function [] 
0002 @0x000597f7 push_shared_string [154] '/csa/log/'
0003 @0x000597fb call_in_user_fun [613, -127] assure_file
0004 @0x000597ff pop_stack [] 
0005 @0x00059803 push_shared_string [155] 'Done of init_startup_module.\n'
0006 @0x00059807 call_in_efun [90, 1] printf
0007 @0x0005980b pop_stack [] 
0008 @0x0005980f null_action [] 
0009 @0x00059813 return_from_fun [] 
```

<a id="csa-init_sql_module"></a>

## csa/pack_data/lib_csa32.pak/csa/start_csa.o :: init_sql_module

成员 SHA-256：`9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1`；函数记录偏移：`0x5c06c`；函数编号：`762`。

```text
0000 @0x0005c106 check_args_num [0] 
0001 @0x0005c10a enter_function [] 
0002 @0x0005c10e push_shared_string [378] ':memory:'
0003 @0x0005c112 call_in_efun [423, 1] sql_open_db
0004 @0x0005c116 push_global_vptr [68] 
0005 @0x0005c11a assign_value [] 
0006 @0x0005c11e pop_stack [] 
0007 @0x0005c122 push_global_var [68] 
0008 @0x0005c126 push_number(S) [-1] 
0009 @0x0005c12a eq_value [] 
0010 @0x0005c12e if [3, 2] 
0011 @0x0005c132 push_shared_string [379] 'Failed to open memory database, startup halt.\n'
0012 @0x0005c136 call_in_efun [28, 1] error
0013 @0x0005c13a pop_stack [] 
0014 @0x0005c13e push_shared_string [380] 'Done of init_sql_module.\n'
0015 @0x0005c142 call_in_efun [90, 1] printf
0016 @0x0005c146 pop_stack [] 
0017 @0x0005c14a null_action [] 
0018 @0x0005c14e return_from_fun [] 
```

<a id="dba-__init__"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: __init__

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x79d3a`；函数编号：`init`。

```text
0000 @0x0007a282 push_number(S) [1024] 
0001 @0x0007a286 call_in_efun [0, 1] allocate_array
0002 @0x0007a28a push_global_vptr [2] 
0003 @0x0007a28e assign_value [] 
0004 @0x0007a292 pop_stack [] 
0005 @0x0007a296 push_number(S) [2] 
0006 @0x0007a29a push_global_vptr [3] 
0007 @0x0007a29e assign_value [] 
0008 @0x0007a2a2 pop_stack [] 
0009 @0x0007a2a6 push_number(S) [0] 
0010 @0x0007a2aa push_global_vptr [4] 
0011 @0x0007a2ae assign_value [] 
0012 @0x0007a2b2 pop_stack [] 
0013 @0x0007a2b6 push_shared_string [701] '/xdbc/'
0014 @0x0007a2ba push_global_vptr [5] 
0015 @0x0007a2be assign_value [] 
0016 @0x0007a2c2 pop_stack [] 
0017 @0x0007a2c6 create_mapping [0] 
0018 @0x0007a2ca push_global_vptr [6] 
0019 @0x0007a2ce assign_value [] 
0020 @0x0007a2d2 pop_stack [] 
0021 @0x0007a2d6 push_number(S) [0] 
0022 @0x0007a2da push_global_vptr [7] 
0023 @0x0007a2de assign_value [] 
0024 @0x0007a2e2 pop_stack [] 
0025 @0x0007a2e6 push_shared_string [154] '/dba/log/'
0026 @0x0007a2ea call_in_efun [336, 1] mkdir
0027 @0x0007a2ee pop_stack [] 
0028 @0x0007a2f2 push_shared_string [702] '/dba/log/xdbc_trace'
0029 @0x0007a2f6 push_shared_string [56] 'trace'
0030 @0x0007a2fa call_in_user_fun [583, -126] xdbc_set_output_basic_file
0031 @0x0007a2fe pop_stack [] 
0032 @0x0007a302 call_in_user_fun [585, -128] xdbc_enable_log_trace
0033 @0x0007a306 pop_stack [] 
0034 @0x0007a30a push_number(S) [0] 
0035 @0x0007a30e push_shared_string [703] 'xdbc'
0036 @0x0007a312 call_in_efun [27, 2] enable_trace
0037 @0x0007a316 pop_stack [] 
0038 @0x0007a31a create_mapping [0] 
0039 @0x0007a31e push_global_vptr [8] 
0040 @0x0007a322 assign_value [] 
0041 @0x0007a326 pop_stack [] 
0042 @0x0007a32a push_shared_string [704] 'xdbc_connect'
0043 @0x0007a32e push_shared_string [65] 'mysql_connect'
0044 @0x0007a332 push_shared_string [705] 'xdbc_disconnect'
0045 @0x0007a336 push_shared_string [66] 'mysql_disconnect'
0046 @0x0007a33a push_shared_string [706] 'xdbc_is_connected'
0047 @0x0007a33e push_shared_string [67] 'mysql_is_connected'
0048 @0x0007a342 push_shared_string [707] 'xdbc_get_error'
0049 @0x0007a346 push_shared_string [68] 'mysql_get_error'
0050 @0x0007a34a push_shared_string [708] 'xdbc_execute'
0051 @0x0007a34e push_shared_string [69] 'mysql_execute'
0052 @0x0007a352 push_shared_string [709] 'xdbc_query'
0053 @0x0007a356 push_shared_string [70] 'mysql_query'
0054 @0x0007a35a push_shared_string [710] 'xdbc_finalize'
0055 @0x0007a35e push_shared_string [71] 'mysql_finalize'
0056 @0x0007a362 push_shared_string [711] 'xdbc_num_records'
0057 @0x0007a366 push_shared_string [72] 'mysql_num_records'
0058 @0x0007a36a push_shared_string [712] 'xdbc_fetch_record'
0059 @0x0007a36e push_shared_string [73] 'mysql_fetch_record'
0060 @0x0007a372 push_shared_string [713] 'xdbc_this_record'
0061 @0x0007a376 push_shared_string [74] 'mysql_this_record'
0062 @0x0007a37a push_shared_string [714] 'xdbc_field_value'
0063 @0x0007a37e push_shared_string [75] 'mysql_field_value'
0064 @0x0007a382 push_shared_string [715] 'xdbc_field_name'
0065 @0x0007a386 push_shared_string [78] 'mysql_field_name'
0066 @0x0007a38a push_shared_string [716] 'xdbc_get_insert_id'
0067 @0x0007a38e push_shared_string [81] 'mysql_get_insert_id'
0068 @0x0007a392 push_shared_string [717] 'NO_IMPLEMENTED'
0069 @0x0007a396 push_shared_string [76] 'mysql_set_auto_commit'
0070 @0x0007a39a push_shared_string [717] 'NO_IMPLEMENTED'
0071 @0x0007a39e push_shared_string [77] 'mysql_select_db'
0072 @0x0007a3a2 push_shared_string [717] 'NO_IMPLEMENTED'
0073 @0x0007a3a6 push_shared_string [79] 'mysql_num_fields'
0074 @0x0007a3aa push_shared_string [717] 'NO_IMPLEMENTED'
0075 @0x0007a3ae push_shared_string [80] 'mysql_int_field'
0076 @0x0007a3b2 create_mapping [34] 
0077 @0x0007a3b6 push_global_vptr [9] 
0078 @0x0007a3ba assign_value [] 
0079 @0x0007a3be pop_stack [] 
0080 @0x0007a3c2 call_in_user_fun [604, -128] init_xdbc_client
0081 @0x0007a3c6 pop_stack [] 
0082 @0x0007a3ca push_shared_string [718] 'CONFIG'
0083 @0x0007a3ce call_in_efun [22, 1] expand_macro
0084 @0x0007a3d2 if [3, 2] 
0085 @0x0007a3d6 push_shared_string [718] 'CONFIG'
0086 @0x0007a3da call_in_efun [22, 1] expand_macro
0087 @0x0007a3de goto [1] 
0088 @0x0007a3e2 push_shared_string [719] '/dba/dba.ini'
0089 @0x0007a3e6 push_global_vptr [27] 
0090 @0x0007a3ea assign_value [] 
0091 @0x0007a3ee pop_stack [] 
0092 @0x0007a3f2 create_mapping [0] 
0093 @0x0007a3f6 push_global_vptr [28] 
0094 @0x0007a3fa assign_value [] 
0095 @0x0007a3fe pop_stack [] 
0096 @0x0007a402 create_mapping [0] 
0097 @0x0007a406 push_global_vptr [29] 
0098 @0x0007a40a assign_value [] 
0099 @0x0007a40e pop_stack [] 
0100 @0x0007a412 push_shared_string [720] '__BIN_DIR__'
0101 @0x0007a416 call_in_efun [22, 1] expand_macro
0102 @0x0007a41a call_in_efun [141, 1] trim
0103 @0x0007a41e push_global_vptr [30] 
0104 @0x0007a422 assign_value [] 
0105 @0x0007a426 pop_stack [] 
0106 @0x0007a42a push_global_var [30] 
0107 @0x0007a42e push_number(S) [1] 
0108 @0x0007a432 push_number(S) [2] 
0109 @0x0007a436 query_range [1] 
0110 @0x0007a43a push_global_vptr [30] 
0111 @0x0007a43e assign_value [] 
0112 @0x0007a442 pop_stack [] 
0113 @0x0007a446 push_global_var [30] 
0114 @0x0007a44a push_number(S) [1] 
0115 @0x0007a44e query_member [1] 
0116 @0x0007a452 push_number(S) [47] 
0117 @0x0007a456 eq_value [] 
0118 @0x0007a45a if [7, 2] 
0119 @0x0007a45e push_global_var [30] 
0120 @0x0007a462 push_number(S) [0] 
0121 @0x0007a466 push_number(S) [2] 
0122 @0x0007a46a query_range [1] 
0123 @0x0007a46e push_global_vptr [30] 
0124 @0x0007a472 assign_value [] 
0125 @0x0007a476 pop_stack [] 
0126 @0x0007a47a create_array [0] 
0127 @0x0007a47e push_global_vptr [31] 
0128 @0x0007a482 assign_value [] 
0129 @0x0007a486 pop_stack [] 
0130 @0x0007a48a create_array [0] 
0131 @0x0007a48e push_global_vptr [32] 
0132 @0x0007a492 assign_value [] 
0133 @0x0007a496 pop_stack [] 
0134 @0x0007a49a create_array [0] 
0135 @0x0007a49e push_global_vptr [33] 
0136 @0x0007a4a2 assign_value [] 
0137 @0x0007a4a6 pop_stack [] 
0138 @0x0007a4aa create_array [0] 
0139 @0x0007a4ae push_global_vptr [34] 
0140 @0x0007a4b2 assign_value [] 
0141 @0x0007a4b6 pop_stack [] 
0142 @0x0007a4ba create_array [0] 
0143 @0x0007a4be push_global_vptr [35] 
0144 @0x0007a4c2 assign_value [] 
0145 @0x0007a4c6 pop_stack [] 
0146 @0x0007a4ca create_array [0] 
0147 @0x0007a4ce push_global_vptr [36] 
0148 @0x0007a4d2 assign_value [] 
0149 @0x0007a4d6 pop_stack [] 
0150 @0x0007a4da create_array [0] 
0151 @0x0007a4de push_global_vptr [37] 
0152 @0x0007a4e2 assign_value [] 
0153 @0x0007a4e6 pop_stack [] 
0154 @0x0007a4ea create_array [0] 
0155 @0x0007a4ee push_global_vptr [38] 
0156 @0x0007a4f2 assign_value [] 
0157 @0x0007a4f6 pop_stack [] 
0158 @0x0007a4fa create_array [0] 
0159 @0x0007a4fe push_global_vptr [39] 
0160 @0x0007a502 assign_value [] 
0161 @0x0007a506 pop_stack [] 
0162 @0x0007a50a create_array [0] 
0163 @0x0007a50e push_global_vptr [40] 
0164 @0x0007a512 assign_value [] 
0165 @0x0007a516 pop_stack [] 
0166 @0x0007a51a create_array [0] 
0167 @0x0007a51e push_global_vptr [41] 
0168 @0x0007a522 assign_value [] 
0169 @0x0007a526 pop_stack [] 
0170 @0x0007a52a create_array [0] 
0171 @0x0007a52e push_global_vptr [42] 
0172 @0x0007a532 assign_value [] 
0173 @0x0007a536 pop_stack [] 
0174 @0x0007a53a create_mapping [0] 
0175 @0x0007a53e push_global_vptr [43] 
0176 @0x0007a542 assign_value [] 
0177 @0x0007a546 pop_stack [] 
0178 @0x0007a54a create_array [0] 
0179 @0x0007a54e push_global_vptr [44] 
0180 @0x0007a552 assign_value [] 
0181 @0x0007a556 pop_stack [] 
0182 @0x0007a55a create_array [0] 
0183 @0x0007a55e push_global_vptr [45] 
0184 @0x0007a562 assign_value [] 
0185 @0x0007a566 pop_stack [] 
0186 @0x0007a56a create_array [0] 
0187 @0x0007a56e push_global_vptr [46] 
0188 @0x0007a572 assign_value [] 
0189 @0x0007a576 pop_stack [] 
0190 @0x0007a57a create_mapping [0] 
0191 @0x0007a57e push_global_vptr [47] 
0192 @0x0007a582 assign_value [] 
0193 @0x0007a586 pop_stack [] 
0194 @0x0007a58a push_number(S) [-1] 
0195 @0x0007a58e push_global_vptr [48] 
0196 @0x0007a592 assign_value [] 
0197 @0x0007a596 pop_stack [] 
0198 @0x0007a59a push_number(S) [0] 
0199 @0x0007a59e push_global_vptr [49] 
0200 @0x0007a5a2 assign_value [] 
0201 @0x0007a5a6 pop_stack [] 
0202 @0x0007a5aa push_shared_string [160] 'update_today_zero_time'
0203 @0x0007a5ae push_number(S) [0] 
0204 @0x0007a5b2 push_number(S) [0] 
0205 @0x0007a5b6 call_in_efun [327, 3] set_timer
0206 @0x0007a5ba pop_stack [] 
0207 @0x0007a5be push_number(S) [45] 
0208 @0x0007a5c2 push_global_vptr [50] 
0209 @0x0007a5c6 assign_value [] 
0210 @0x0007a5ca pop_stack [] 
0211 @0x0007a5ce push_number(S) [-1] 
0212 @0x0007a5d2 push_global_vptr [51] 
0213 @0x0007a5d6 assign_value [] 
0214 @0x0007a5da pop_stack [] 
0215 @0x0007a5de push_number(S) [1] 
0216 @0x0007a5e2 push_global_vptr [52] 
0217 @0x0007a5e6 assign_value [] 
0218 @0x0007a5ea pop_stack [] 
0219 @0x0007a5ee create_mapping [0] 
0220 @0x0007a5f2 push_global_vptr [53] 
0221 @0x0007a5f6 assign_value [] 
0222 @0x0007a5fa pop_stack [] 
0223 @0x0007a5fe create_mapping [0] 
0224 @0x0007a602 push_global_vptr [54] 
0225 @0x0007a606 assign_value [] 
0226 @0x0007a60a pop_stack [] 
0227 @0x0007a60e push_number(S) [0] 
0228 @0x0007a612 push_global_vptr [55] 
0229 @0x0007a616 assign_value [] 
0230 @0x0007a61a pop_stack [] 
0231 @0x0007a61e push_shared_string [721] '癸亥'
0232 @0x0007a622 push_shared_string [722] '壬戌'
0233 @0x0007a626 push_shared_string [723] '辛酉'
0234 @0x0007a62a push_shared_string [724] '庚申'
0235 @0x0007a62e push_shared_string [725] '己未'
0236 @0x0007a632 push_shared_string [726] '戊午'
0237 @0x0007a636 push_shared_string [727] '丁巳'
0238 @0x0007a63a push_shared_string [728] '丙辰'
0239 @0x0007a63e push_shared_string [729] '乙卯'
0240 @0x0007a642 push_shared_string [730] '甲寅'
0241 @0x0007a646 push_shared_string [731] '癸丑'
0242 @0x0007a64a push_shared_string [732] '壬子'
0243 @0x0007a64e push_shared_string [733] '辛亥'
0244 @0x0007a652 push_shared_string [734] '庚戌'
0245 @0x0007a656 push_shared_string [735] '己酉'
0246 @0x0007a65a push_shared_string [736] '戊申'
0247 @0x0007a65e push_shared_string [737] '丁未'
0248 @0x0007a662 push_shared_string [738] '丙午'
0249 @0x0007a666 push_shared_string [739] '乙巳'
0250 @0x0007a66a push_shared_string [740] '甲辰'
0251 @0x0007a66e push_shared_string [741] '癸卯'
0252 @0x0007a672 push_shared_string [742] '壬寅'
0253 @0x0007a676 push_shared_string [743] '辛丑'
0254 @0x0007a67a push_shared_string [744] '庚子'
0255 @0x0007a67e push_shared_string [745] '己亥'
0256 @0x0007a682 push_shared_string [746] '戊戌'
0257 @0x0007a686 push_shared_string [747] '丁酉'
0258 @0x0007a68a push_shared_string [748] '丙申'
0259 @0x0007a68e push_shared_string [749] '乙未'
0260 @0x0007a692 push_shared_string [750] '甲午'
0261 @0x0007a696 push_shared_string [751] '癸巳'
0262 @0x0007a69a push_shared_string [752] '壬辰'
0263 @0x0007a69e push_shared_string [753] '辛卯'
0264 @0x0007a6a2 push_shared_string [754] '庚寅'
0265 @0x0007a6a6 push_shared_string [755] '己丑'
0266 @0x0007a6aa push_shared_string [756] '戊子'
0267 @0x0007a6ae push_shared_string [757] '丁亥'
0268 @0x0007a6b2 push_shared_string [758] '丙戌'
0269 @0x0007a6b6 push_shared_string [759] '乙酉'
0270 @0x0007a6ba push_shared_string [760] '甲申'
0271 @0x0007a6be push_shared_string [761] '癸未'
0272 @0x0007a6c2 push_shared_string [762] '壬午'
0273 @0x0007a6c6 push_shared_string [763] '辛巳'
0274 @0x0007a6ca push_shared_string [764] '庚辰'
0275 @0x0007a6ce push_shared_string [765] '己卯'
0276 @0x0007a6d2 push_shared_string [766] '戊寅'
0277 @0x0007a6d6 push_shared_string [767] '丁丑'
0278 @0x0007a6da push_shared_string [768] '丙子'
0279 @0x0007a6de push_shared_string [769] '乙亥'
0280 @0x0007a6e2 push_shared_string [770] '甲戌'
0281 @0x0007a6e6 push_shared_string [771] '癸酉'
0282 @0x0007a6ea push_shared_string [772] '壬申'
0283 @0x0007a6ee push_shared_string [773] '辛未'
0284 @0x0007a6f2 push_shared_string [774] '庚午'
0285 @0x0007a6f6 push_shared_string [775] '己巳'
0286 @0x0007a6fa push_shared_string [776] '戊辰'
0287 @0x0007a6fe push_shared_string [777] '丁卯'
0288 @0x0007a702 push_shared_string [778] '丙寅'
0289 @0x0007a706 push_shared_string [779] '乙丑'
0290 @0x0007a70a push_shared_string [780] '甲子'
0291 @0x0007a70e create_array [60] 
0292 @0x0007a712 push_global_vptr [56] 
0293 @0x0007a716 assign_value [] 
0294 @0x0007a71a pop_stack [] 
0295 @0x0007a71e push_shared_string [323] '癸'
0296 @0x0007a722 push_shared_string [321] '壬'
0297 @0x0007a726 push_shared_string [319] '辛'
0298 @0x0007a72a push_shared_string [317] '庚'
0299 @0x0007a72e push_shared_string [315] '己'
0300 @0x0007a732 push_shared_string [322] '戊'
0301 @0x0007a736 push_shared_string [320] '丁'
0302 @0x0007a73a push_shared_string [318] '丙'
0303 @0x0007a73e push_shared_string [316] '乙'
0304 @0x0007a742 push_shared_string [314] '甲'
0305 @0x0007a746 create_array [10] 
0306 @0x0007a74a push_global_vptr [57] 
0307 @0x0007a74e assign_value [] 
0308 @0x0007a752 pop_stack [] 
0309 @0x0007a756 push_shared_string [781] '亥'
0310 @0x0007a75a push_shared_string [782] '戌'
0311 @0x0007a75e push_shared_string [783] '酉'
0312 @0x0007a762 push_shared_string [784] '申'
0313 @0x0007a766 push_shared_string [785] 'δ'
0314 @0x0007a76a push_shared_string [786] '午'
0315 @0x0007a76e push_shared_string [787] '巳'
0316 @0x0007a772 push_shared_string [788] '辰'
0317 @0x0007a776 push_shared_string [789] 'î'
0318 @0x0007a77a push_shared_string [790] '寅'
0319 @0x0007a77e push_shared_string [791] '丑'
0320 @0x0007a782 push_shared_string [792] '子'
0321 @0x0007a786 create_array [12] 
0322 @0x0007a78a push_global_vptr [58] 
0323 @0x0007a78e assign_value [] 
0324 @0x0007a792 pop_stack [] 
0325 @0x0007a796 push_number(S) [45] 
0326 @0x0007a79a push_shared_string [231] 'year'
0327 @0x0007a79e push_number(S) [12] 
0328 @0x0007a7a2 push_shared_string [793] 'month'
0329 @0x0007a7a6 push_number(S) [17] 
0330 @0x0007a7aa push_shared_string [325] 'day'
0331 @0x0007a7ae push_number(S) [28] 
0332 @0x0007a7b2 push_shared_string [158] 'hour'
0333 @0x0007a7b6 create_mapping [8] 
0334 @0x0007a7ba push_global_vptr [59] 
0335 @0x0007a7be assign_value [] 
0336 @0x0007a7c2 pop_stack [] 
0337 @0x0007a7c6 push_number(S) [31] 
0338 @0x0007a7ca push_number(S) [30] 
0339 @0x0007a7ce push_number(S) [31] 
0340 @0x0007a7d2 push_number(S) [30] 
0341 @0x0007a7d6 push_number(S) [31] 
0342 @0x0007a7da push_number(S) [31] 
0343 @0x0007a7de push_number(S) [30] 
0344 @0x0007a7e2 push_number(S) [31] 
0345 @0x0007a7e6 push_number(S) [30] 
0346 @0x0007a7ea push_number(S) [31] 
0347 @0x0007a7ee push_number(S) [28] 
0348 @0x0007a7f2 push_number(S) [31] 
0349 @0x0007a7f6 create_array [12] 
0350 @0x0007a7fa push_global_vptr [60] 
0351 @0x0007a7fe assign_value [] 
0352 @0x0007a802 pop_stack [] 
0353 @0x0007a806 push_number(S) [25] 
0354 @0x0007a80a push_number(S) [24] 
0355 @0x0007a80e push_number(S) [23] 
0356 @0x0007a812 push_number(S) [22] 
0357 @0x0007a816 push_number(S) [21] 
0358 @0x0007a81a push_number(S) [19] 
0359 @0x0007a81e push_number(S) [18] 
0360 @0x0007a822 push_number(S) [17] 
0361 @0x0007a826 push_number(S) [16] 
0362 @0x0007a82a push_number(S) [15] 
0363 @0x0007a82e push_number(S) [14] 
0364 @0x0007a832 push_number(S) [13] 
0365 @0x0007a836 push_number(S) [12] 
0366 @0x0007a83a push_number(S) [11] 
0367 @0x0007a83e push_number(S) [10] 
0368 @0x0007a842 push_number(S) [9] 
0369 @0x0007a846 push_number(S) [8] 
0370 @0x0007a84a push_number(S) [7] 
0371 @0x0007a84e push_number(S) [6] 
0372 @0x0007a852 push_number(S) [5] 
0373 @0x0007a856 push_number(S) [4] 
0374 @0x0007a85a push_number(S) [3] 
0375 @0x0007a85e push_number(S) [0] 
0376 @0x0007a862 create_array [23] 
0377 @0x0007a866 push_global_vptr [61] 
0378 @0x0007a86a assign_value [] 
0379 @0x0007a86e pop_stack [] 
0380 @0x0007a872 push_shared_string [260] ', '
0381 @0x0007a876 push_global_var [61] 
0382 @0x0007a87a call_in_efun [54, 2] implode
0383 @0x0007a87e push_global_vptr [62] 
0384 @0x0007a882 assign_value [] 
0385 @0x0007a886 pop_stack [] 
0386 @0x0007a88a push_number(S) [0] 
0387 @0x0007a88e push_global_vptr [63] 
0388 @0x0007a892 assign_value [] 
0389 @0x0007a896 pop_stack [] 
0390 @0x0007a89a push_number(S) [29] 
0391 @0x0007a89e push_global_vptr [64] 
0392 @0x0007a8a2 assign_value [] 
0393 @0x0007a8a6 pop_stack [] 
0394 @0x0007a8aa push_number(S) [27] 
0395 @0x0007a8ae push_global_vptr [65] 
0396 @0x0007a8b2 assign_value [] 
0397 @0x0007a8b6 pop_stack [] 
0398 @0x0007a8ba push_number(S) [1] 
0399 @0x0007a8be push_global_vptr [66] 
0400 @0x0007a8c2 assign_value [] 
0401 @0x0007a8c6 pop_stack [] 
0402 @0x0007a8ca push_number(S) [3] 
0403 @0x0007a8ce push_global_vptr [67] 
0404 @0x0007a8d2 assign_value [] 
0405 @0x0007a8d6 pop_stack [] 
0406 @0x0007a8da push_number(S) [-1] 
0407 @0x0007a8de push_global_vptr [68] 
0408 @0x0007a8e2 assign_value [] 
0409 @0x0007a8e6 pop_stack [] 
0410 @0x0007a8ea create_mapping [0] 
0411 @0x0007a8ee push_global_vptr [69] 
0412 @0x0007a8f2 assign_value [] 
0413 @0x0007a8f6 pop_stack [] 
0414 @0x0007a8fa create_mapping [0] 
0415 @0x0007a8fe push_global_vptr [70] 
0416 @0x0007a902 assign_value [] 
0417 @0x0007a906 pop_stack [] 
0418 @0x0007a90a push_number(S) [0] 
0419 @0x0007a90e push_global_vptr [71] 
0420 @0x0007a912 assign_value [] 
0421 @0x0007a916 pop_stack [] 
0422 @0x0007a91a create_mapping [0] 
0423 @0x0007a91e push_global_vptr [72] 
0424 @0x0007a922 assign_value [] 
0425 @0x0007a926 pop_stack [] 
0426 @0x0007a92a create_mapping [0] 
0427 @0x0007a92e push_global_vptr [73] 
0428 @0x0007a932 assign_value [] 
0429 @0x0007a936 pop_stack [] 
0430 @0x0007a93a create_mapping [0] 
0431 @0x0007a93e push_global_vptr [74] 
0432 @0x0007a942 assign_value [] 
0433 @0x0007a946 pop_stack [] 
0434 @0x0007a94a push_number(S) [0] 
0435 @0x0007a94e push_global_vptr [75] 
0436 @0x0007a952 assign_value [] 
0437 @0x0007a956 pop_stack [] 
0438 @0x0007a95a push_number(S) [0] 
0439 @0x0007a95e push_global_vptr [76] 
0440 @0x0007a962 assign_value [] 
0441 @0x0007a966 pop_stack [] 
0442 @0x0007a96a create_mapping [0] 
0443 @0x0007a96e push_global_vptr [77] 
0444 @0x0007a972 assign_value [] 
0445 @0x0007a976 pop_stack [] 
0446 @0x0007a97a create_mapping [0] 
0447 @0x0007a97e push_global_vptr [78] 
0448 @0x0007a982 assign_value [] 
0449 @0x0007a986 pop_stack [] 
0450 @0x0007a98a push_shared_string [135] ''
0451 @0x0007a98e push_global_vptr [79] 
0452 @0x0007a992 assign_value [] 
0453 @0x0007a996 pop_stack [] 
0454 @0x0007a99a push_shared_string [135] ''
0455 @0x0007a99e push_global_vptr [80] 
0456 @0x0007a9a2 assign_value [] 
0457 @0x0007a9a6 pop_stack [] 
0458 @0x0007a9aa push_number(S) [2] 
0459 @0x0007a9ae push_global_vptr [81] 
0460 @0x0007a9b2 assign_value [] 
0461 @0x0007a9b6 pop_stack [] 
0462 @0x0007a9ba push_number(S) [0] 
0463 @0x0007a9be push_global_vptr [82] 
0464 @0x0007a9c2 assign_value [] 
0465 @0x0007a9c6 pop_stack [] 
0466 @0x0007a9ca create_mapping [0] 
0467 @0x0007a9ce push_global_vptr [83] 
0468 @0x0007a9d2 assign_value [] 
0469 @0x0007a9d6 pop_stack [] 
0470 @0x0007a9da push_number(S) [0] 
0471 @0x0007a9de push_number(S) [2] 
0472 @0x0007a9e2 push_shared_string [794] 'internal_comm'
0473 @0x0007a9e6 push_number(S) [1] 
0474 @0x0007a9ea push_shared_string [795] 'client_user'
0475 @0x0007a9ee push_number(S) [4] 
0476 @0x0007a9f2 push_shared_string [796] 'from_server'
0477 @0x0007a9f6 create_mapping [6] 
0478 @0x0007a9fa call_in_efun [105, 2] set_case_check
0479 @0x0007a9fe push_global_vptr [88] 
0480 @0x0007aa02 assign_value [] 
0481 @0x0007aa06 pop_stack [] 
0482 @0x0007aa0a create_mapping [0] 
0483 @0x0007aa0e push_global_vptr [89] 
0484 @0x0007aa12 assign_value [] 
0485 @0x0007aa16 pop_stack [] 
0486 @0x0007aa1a push_number(S) [1] 
0487 @0x0007aa1e push_shared_string [797] 'msg_intersvr_rpc_ret'
0488 @0x0007aa22 create_mapping [2] 
0489 @0x0007aa26 push_number(S) [9] 
0490 @0x0007aa2a push_number(S) [1] 
0491 @0x0007aa2e push_shared_string [798] 'msg_treasure_trading_oper'
0492 @0x0007aa32 push_number(S) [1] 
0493 @0x0007aa36 push_shared_string [799] 'cmd_cta_to_server'
0494 @0x0007aa3a create_mapping [4] 
0495 @0x0007aa3e push_number(S) [16] 
0496 @0x0007aa42 create_mapping [4] 
0497 @0x0007aa46 push_global_vptr [90] 
0498 @0x0007aa4a assign_value [] 
0499 @0x0007aa4e pop_stack [] 
0500 @0x0007aa52 create_array [0] 
0501 @0x0007aa56 push_global_vptr [91] 
0502 @0x0007aa5a assign_value [] 
0503 @0x0007aa5e pop_stack [] 
0504 @0x0007aa62 create_array [0] 
0505 @0x0007aa66 push_global_vptr [92] 
0506 @0x0007aa6a assign_value [] 
0507 @0x0007aa6e pop_stack [] 
0508 @0x0007aa72 create_array [0] 
0509 @0x0007aa76 push_global_vptr [93] 
0510 @0x0007aa7a assign_value [] 
0511 @0x0007aa7e pop_stack [] 
0512 @0x0007aa82 push_number(S) [0] 
0513 @0x0007aa86 push_global_vptr [94] 
0514 @0x0007aa8a assign_value [] 
0515 @0x0007aa8e pop_stack [] 
0516 @0x0007aa92 create_mapping [0] 
0517 @0x0007aa96 push_global_vptr [95] 
0518 @0x0007aa9a assign_value [] 
0519 @0x0007aa9e pop_stack [] 
0520 @0x0007aaa2 call_in_efun [137, 0] time
0521 @0x0007aaa6 push_global_vptr [96] 
0522 @0x0007aaaa assign_value [] 
0523 @0x0007aaae pop_stack [] 
0524 @0x0007aab2 push_shared_string [800] 'include;dba;dba/include'
0525 @0x0007aab6 call_in_efun [128, 1] switch_include_dir
0526 @0x0007aaba pop_stack [] 
0527 @0x0007aabe push_shared_string [801] 'full'
0528 @0x0007aac2 push_shared_string [802] 'callstack_when_error'
0529 @0x0007aac6 call_in_efun [164, 2] config_driver
0530 @0x0007aaca pop_stack [] 
0531 @0x0007aace call_in_efun [137, 0] time
0532 @0x0007aad2 push_global_vptr [97] 
0533 @0x0007aad6 assign_value [] 
0534 @0x0007aada pop_stack [] 
0535 @0x0007aade push_number(S) [1] 
0536 @0x0007aae2 call_in_efun [109, 1] set_log_on
0537 @0x0007aae6 pop_stack [] 
0538 @0x0007aaea call_in_user_fun [656, -128] init_startup_module
0539 @0x0007aaee pop_stack [] 
0540 @0x0007aaf2 call_in_user_fun [941, -128] init_compile_module
0541 @0x0007aaf6 pop_stack [] 
0542 @0x0007aafa call_in_user_fun [762, -128] init_sql_module
0543 @0x0007aafe pop_stack [] 
0544 @0x0007ab02 call_in_user_fun [766, -128] init_debug_cmds_module
0545 @0x0007ab06 pop_stack [] 
0546 @0x0007ab0a call_in_user_fun [982, -128] init_platform_module
0547 @0x0007ab0e pop_stack [] 
0548 @0x0007ab12 push_global_var [87] 
0549 @0x0007ab16 push_shared_string [803] ' all global feature.\n'
0550 @0x0007ab1a add [] 
0551 @0x0007ab1e call_in_efun [90, 1] printf
0552 @0x0007ab22 pop_stack [] 
0553 @0x0007ab26 push_shared_string [804] '/feature/'
0554 @0x0007ab2a push_global_var [84] 
0555 @0x0007ab2e call_fun_ptr [0, 2] 
0556 @0x0007ab32 pop_stack [] 
0557 @0x0007ab36 push_global_var [87] 
0558 @0x0007ab3a push_shared_string [805] ' all global inherits.\n'
0559 @0x0007ab3e add [] 
0560 @0x0007ab42 call_in_efun [90, 1] printf
0561 @0x0007ab46 pop_stack [] 
0562 @0x0007ab4a push_shared_string [806] '/inherit/'
0563 @0x0007ab4e push_global_var [84] 
0564 @0x0007ab52 call_fun_ptr [0, 2] 
0565 @0x0007ab56 pop_stack [] 
0566 @0x0007ab5a push_global_var [87] 
0567 @0x0007ab5e push_shared_string [807] ' all global objects.\n'
0568 @0x0007ab62 add [] 
0569 @0x0007ab66 call_in_efun [90, 1] printf
0570 @0x0007ab6a pop_stack [] 
0571 @0x0007ab6e push_shared_string [808] '/clone/'
0572 @0x0007ab72 push_global_var [84] 
0573 @0x0007ab76 call_fun_ptr [0, 2] 
0574 @0x0007ab7a pop_stack [] 
0575 @0x0007ab7e push_global_var [87] 
0576 @0x0007ab82 push_shared_string [809] ' all global cmds.\n'
0577 @0x0007ab86 add [] 
0578 @0x0007ab8a call_in_efun [90, 1] printf
0579 @0x0007ab8e pop_stack [] 
0580 @0x0007ab92 push_shared_string [810] '/cmds/'
0581 @0x0007ab96 push_global_var [84] 
0582 @0x0007ab9a call_fun_ptr [0, 2] 
0583 @0x0007ab9e pop_stack [] 
0584 @0x0007aba2 push_shared_string [152] '/daemons/configd.c'
0585 @0x0007aba6 push_global_var [86] 
0586 @0x0007abaa call_fun_ptr [0, 2] 
0587 @0x0007abae pop_stack [] 
0588 @0x0007abb2 push_shared_string [811] '/daemons/'
0589 @0x0007abb6 push_global_var [85] 
0590 @0x0007abba call_fun_ptr [0, 2] 
0591 @0x0007abbe pop_stack [] 
0592 @0x0007abc2 push_shared_string [812] '/dba/inherit/misc'
0593 @0x0007abc6 push_global_var [85] 
0594 @0x0007abca call_fun_ptr [0, 2] 
0595 @0x0007abce pop_stack [] 
0596 @0x0007abd2 push_shared_string [813] '/dba/daemons/'
0597 @0x0007abd6 push_global_var [85] 
0598 @0x0007abda call_fun_ptr [0, 2] 
0599 @0x0007abde pop_stack [] 
0600 @0x0007abe2 push_shared_string [814] '/dba/clone/'
0601 @0x0007abe6 push_global_var [84] 
0602 @0x0007abea call_fun_ptr [0, 2] 
0603 @0x0007abee pop_stack [] 
0604 @0x0007abf2 push_shared_string [815] '/dba/cmds/'
0605 @0x0007abf6 push_global_var [84] 
0606 @0x0007abfa call_fun_ptr [0, 2] 
0607 @0x0007abfe pop_stack [] 
0608 @0x0007ac02 call_in_efun [67, 0] is_compiling
0609 @0x0007ac06 if [13, 2] 
0610 @0x0007ac0a push_shared_string [811] '/daemons/'
0611 @0x0007ac0e push_global_var [84] 
0612 @0x0007ac12 call_fun_ptr [0, 2] 
0613 @0x0007ac16 pop_stack [] 
0614 @0x0007ac1a push_shared_string [813] '/dba/daemons/'
0615 @0x0007ac1e push_global_var [84] 
0616 @0x0007ac22 call_fun_ptr [0, 2] 
0617 @0x0007ac26 pop_stack [] 
0618 @0x0007ac2a push_shared_string [816] '/dba/handlers/'
0619 @0x0007ac2e push_global_var [84] 
0620 @0x0007ac32 call_fun_ptr [0, 2] 
0621 @0x0007ac36 pop_stack [] 
0622 @0x0007ac3a null_action [] 
0623 @0x0007ac3e push_shared_string [817] 'Invoke registered initialization routines.\n'
0624 @0x0007ac42 call_in_efun [90, 1] printf
0625 @0x0007ac46 pop_stack [] 
0626 @0x0007ac4a call_in_user_fun [629, -128] post_init
0627 @0x0007ac4e pop_stack [] 
0628 @0x0007ac52 push_number(S) [3] 
0629 @0x0007ac56 call_in_user_fun [793, -127] set_server_status
0630 @0x0007ac5a pop_stack [] 
0631 @0x0007ac5e push_shared_string [818] '\x1b[1;33mDriver load ok!\n\x1b[2;37;0m'
0632 @0x0007ac62 call_in_efun [90, 1] printf
0633 @0x0007ac66 pop_stack [] 
0634 @0x0007ac6a push_shared_string [819] 'Driver startup\n'
0635 @0x0007ac6e call_in_efun [10, 1] broadcast
0636 @0x0007ac72 pop_stack [] 
0637 @0x0007ac76 null_action [] 
0638 @0x0007ac7a finish_action [] 
```

<a id="dba-post_init"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: post_init

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x3e89e`；函数编号：`629`。

```text
0000 @0x0003e9c3 check_args_num [0] 
0001 @0x0003e9c7 enter_function [] 
0002 @0x0003e9cb push_global_var [31] 
0003 @0x0003e9cf sizeof [] 
0004 @0x0003e9d3 if [36, 2] 
0005 @0x0003e9d7 push_global_var [31] 
0006 @0x0003e9db push_local_vptr [-2] 
0007 @0x0003e9df assign_value [] 
0008 @0x0003e9e3 pop_stack [] 
0009 @0x0003e9e7 create_array [0] 
0010 @0x0003e9eb push_global_vptr [31] 
0011 @0x0003e9ef assign_value [] 
0012 @0x0003e9f3 pop_stack [] 
0013 @0x0003e9f7 push_local_vptr [-1] 
0014 @0x0003e9fb push_local_value [-2] 
0015 @0x0003e9ff push_number(S) [997] 
0016 @0x0003ea03 init_for_loop [-3] 
0017 @0x0003ea07 cond_jmp_for_loop [14, -3] 
0018 @0x0003ea0b push_local_value [-1] 
0019 @0x0003ea0f is_type [9] 
0020 @0x0003ea13 if [9, 2] 
0021 @0x0003ea17 new_thread_catch [6] 
0022 @0x0003ea1b push_local_value [-1] 
0023 @0x0003ea1f call_fun_ptr [4, 1] 
0024 @0x0003ea23 pop_stack [] 
0025 @0x0003ea27 push_number(S) [0] 
0026 @0x0003ea2b end_thread_catch [] 
0027 @0x0003ea2f goto [1] 
0028 @0x0003ea33 caught_error [] 
0029 @0x0003ea37 pop_stack [] 
0030 @0x0003ea3b null_action [] 
0031 @0x0003ea3f step_jmp_for_loop [-14, -3] 
0032 @0x0003ea43 push_global_var [31] 
0033 @0x0003ea47 sizeof [] 
0034 @0x0003ea4b if [4, 2] 
0035 @0x0003ea4f push_number(S) [1] 
0036 @0x0003ea53 call_in_efun [298, 1] sleep
0037 @0x0003ea57 pop_stack [] 
0038 @0x0003ea5b null_action [] 
0039 @0x0003ea5f null_action [] 
0040 @0x0003ea63 goto [-39] 
0041 @0x0003ea67 push_number(S) [0] 
0042 @0x0003ea6b push_global_vptr [31] 
0043 @0x0003ea6f assign_value [] 
0044 @0x0003ea73 pop_stack [] 
0045 @0x0003ea77 null_action [] 
0046 @0x0003ea7b return_from_fun [] 
```

<a id="dba-xdbc_query"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: xdbc_query

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x46266`；函数编号：`593`。

```text
0000 @0x0004638b check_args_num [0] 
0001 @0x0004638f enter_function [] 
0002 @0x00046393 push_local_value [1] 
0003 @0x00046397 push_local_value [0] 
0004 @0x0004639b push_shared_string [116] 'Call xdbc_query(%d, %s)\n'
0005 @0x0004639f push_number(S) [0] 
0006 @0x000463a3 call_in_user_fun [584, -124] xdbc_trace
0007 @0x000463a7 pop_stack [] 
0008 @0x000463ab push_shared_string [117] 'query'
0009 @0x000463af push_shared_string [39] 'cmd'
0010 @0x000463b3 push_local_value [1] 
0011 @0x000463b7 push_shared_string [115] 'sql_cmd'
0012 @0x000463bb create_mapping [4] 
0013 @0x000463bf push_local_value [0] 
0014 @0x000463c3 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x000463c7 push_local_vptr [-1] 
0016 @0x000463cb assign_value [] 
0017 @0x000463cf pop_stack [] 
0018 @0x000463d3 push_local_value [-1] 
0019 @0x000463d7 push_number(S) [-1] 
0020 @0x000463db eq_value [] 
0021 @0x000463df if [2, 2] 
0022 @0x000463e3 push_number(S) [-1] 
0023 @0x000463e7 return_from_fun [] 
0024 @0x000463eb push_local_value [0] 
0025 @0x000463ef push_shared_string [118] 'cno'
0026 @0x000463f3 create_mapping [2] 
0027 @0x000463f7 push_global_vptr [8] 
0028 @0x000463fb push_local_value [-1] 
0029 @0x000463ff query_member_ptr [0] 
0030 @0x00046403 assign_value [] 
0031 @0x00046407 pop_stack [] 
0032 @0x0004640b push_local_value [-1] 
0033 @0x0004640f return_from_fun [] 
0034 @0x00046413 null_action [] 
0035 @0x00046417 return_from_fun [] 
```

<a id="dba-xdbc_connect"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: xdbc_connect

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x4f27d`；函数编号：`588`。

```text
0000 @0x0004f4fb check_args_num [0] 
0001 @0x0004f4ff enter_function [] 
0002 @0x0004f503 push_number(S) [2] 
0003 @0x0004f507 call_in_efun [267, 1] socket
0004 @0x0004f50b push_local_vptr [-3] 
0005 @0x0004f50f assign_value [] 
0006 @0x0004f513 pop_stack [] 
0007 @0x0004f517 push_number(S) [9527] 
0008 @0x0004f51b push_local_value [0] 
0009 @0x0004f51f push_local_value [-3] 
0010 @0x0004f523 call_in_efun [254, 3] connect
0011 @0x0004f527 push_number(S) [0] 
0012 @0x0004f52b ne_value [] 
0013 @0x0004f52f if [12, 2] 
0014 @0x0004f533 push_number(S) [9527] 
0015 @0x0004f537 push_local_value [0] 
0016 @0x0004f53b push_shared_string [99] 'Failed to connect to %s:%d\n'
0017 @0x0004f53f push_number(S) [2] 
0018 @0x0004f543 call_in_user_fun [581, -124] xdbc_log
0019 @0x0004f547 pop_stack [] 
0020 @0x0004f54b push_local_value [-3] 
0021 @0x0004f54f call_in_efun [255, 1] close_socket
0022 @0x0004f553 pop_stack [] 
0023 @0x0004f557 push_number(S) [-1] 
0024 @0x0004f55b return_from_fun [] 
0025 @0x0004f55f null_action [] 
0026 @0x0004f563 push_local_value [-3] 
0027 @0x0004f567 call_in_user_fun [572, -127] xdbc_allocate_client
0028 @0x0004f56b push_local_vptr [-4] 
0029 @0x0004f56f assign_value [] 
0030 @0x0004f573 push_number(S) [-1] 
0031 @0x0004f577 eq_value [] 
0032 @0x0004f57b if [10, 2] 
0033 @0x0004f57f push_shared_string [100] 'No more free client to allocate.\n'
0034 @0x0004f583 push_number(S) [2] 
0035 @0x0004f587 call_in_user_fun [581, -126] xdbc_log
0036 @0x0004f58b pop_stack [] 
0037 @0x0004f58f push_local_value [-3] 
0038 @0x0004f593 call_in_efun [255, 1] close_socket
0039 @0x0004f597 pop_stack [] 
0040 @0x0004f59b push_number(S) [-1] 
0041 @0x0004f59f return_from_fun [] 
0042 @0x0004f5a3 null_action [] 
0043 @0x0004f5a7 push_number(L) [2000000000] 
0044 @0x0004f5ab call_in_efun [94, 1] random
0045 @0x0004f5af push_local_vptr [-5] 
0046 @0x0004f5b3 assign_value [] 
0047 @0x0004f5b7 pop_stack [] 
0048 @0x0004f5bb push_shared_string [101] 'set_key'
0049 @0x0004f5bf push_shared_string [39] 'cmd'
0050 @0x0004f5c3 push_local_value [-5] 
0051 @0x0004f5c7 push_shared_string [40] 'key'
0052 @0x0004f5cb create_mapping [4] 
0053 @0x0004f5cf push_local_value [-4] 
0054 @0x0004f5d3 call_in_user_fun [601, -126] xdbc_call_server
0055 @0x0004f5d7 push_local_vptr [-2] 
0056 @0x0004f5db assign_value [] 
0057 @0x0004f5df pop_stack [] 
0058 @0x0004f5e3 push_local_value [-2] 
0059 @0x0004f5e7 push_number(S) [-1] 
0060 @0x0004f5eb eq_value [] 
0061 @0x0004f5ef if [7, 1] 
0062 @0x0004f5f3 pop_stack [] 
0063 @0x0004f5f7 push_local_value [-4] 
0064 @0x0004f5fb call_in_user_fun [575, -127] xdbc_get_client
0065 @0x0004f5ff push_local_vptr [-1] 
0066 @0x0004f603 assign_value [] 
0067 @0x0004f607 is_type [7] 
0068 @0x0004f60b not_value [] 
0069 @0x0004f60f if [12, 2] 
0070 @0x0004f613 push_local_value [-2] 
0071 @0x0004f617 push_local_value [-4] 
0072 @0x0004f61b push_shared_string [102] 'Failed to prepare connection %d(result = %d).\n'
0073 @0x0004f61f push_number(S) [2] 
0074 @0x0004f623 call_in_user_fun [581, -124] xdbc_log
0075 @0x0004f627 pop_stack [] 
0076 @0x0004f62b push_local_value [-4] 
0077 @0x0004f62f call_in_user_fun [573, -127] xdbc_free_client
0078 @0x0004f633 pop_stack [] 
0079 @0x0004f637 push_number(S) [-1] 
0080 @0x0004f63b return_from_fun [] 
0081 @0x0004f63f null_action [] 
0082 @0x0004f643 push_local_value [-5] 
0083 @0x0004f647 push_shared_string [103] 'Set key: %016X\n'
0084 @0x0004f64b push_number(S) [1] 
0085 @0x0004f64f call_in_user_fun [584, -125] xdbc_trace
0086 @0x0004f653 pop_stack [] 
0087 @0x0004f657 push_local_value [-5] 
0088 @0x0004f65b push_local_vptr [-1] 
0089 @0x0004f65f push_shared_string [40] 'key'
0090 @0x0004f663 query_member_ptr [0] 
0091 @0x0004f667 assign_value [] 
0092 @0x0004f66b pop_stack [] 
0093 @0x0004f66f push_shared_string [104] 'connect'
0094 @0x0004f673 push_shared_string [39] 'cmd'
0095 @0x0004f677 push_local_value [1] 
0096 @0x0004f67b push_shared_string [105] 'user'
0097 @0x0004f67f push_local_value [2] 
0098 @0x0004f683 push_shared_string [106] 'passwd'
0099 @0x0004f687 push_local_value [3] 
0100 @0x0004f68b push_shared_string [107] 'db'
0101 @0x0004f68f create_mapping [8] 
0102 @0x0004f693 push_local_value [-4] 
0103 @0x0004f697 call_in_user_fun [601, -126] xdbc_call_server
0104 @0x0004f69b push_local_vptr [-2] 
0105 @0x0004f69f assign_value [] 
0106 @0x0004f6a3 pop_stack [] 
0107 @0x0004f6a7 push_local_value [-2] 
0108 @0x0004f6ab push_number(S) [-1] 
0109 @0x0004f6af eq_value [] 
0110 @0x0004f6b3 if [10, 2] 
0111 @0x0004f6b7 push_shared_string [108] 'Failed to connect to database.\n'
0112 @0x0004f6bb push_number(S) [2] 
0113 @0x0004f6bf call_in_user_fun [581, -126] xdbc_log
0114 @0x0004f6c3 pop_stack [] 
0115 @0x0004f6c7 push_local_value [-4] 
0116 @0x0004f6cb call_in_user_fun [573, -127] xdbc_free_client
0117 @0x0004f6cf pop_stack [] 
0118 @0x0004f6d3 push_number(S) [-1] 
0119 @0x0004f6d7 return_from_fun [] 
0120 @0x0004f6db null_action [] 
0121 @0x0004f6df push_local_value [-4] 
0122 @0x0004f6e3 return_from_fun [] 
0123 @0x0004f6e7 null_action [] 
0124 @0x0004f6eb return_from_fun [] 
```

<a id="dba-xdbc_execute"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: xdbc_execute

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x51bc2`；函数编号：`592`。

```text
0000 @0x00051c9b check_args_num [0] 
0001 @0x00051c9f enter_function [] 
0002 @0x00051ca3 push_local_value [1] 
0003 @0x00051ca7 push_local_value [0] 
0004 @0x00051cab push_shared_string [113] 'Call xdbc_execute(%d, %s)\n'
0005 @0x00051caf push_number(S) [0] 
0006 @0x00051cb3 call_in_user_fun [584, -124] xdbc_trace
0007 @0x00051cb7 pop_stack [] 
0008 @0x00051cbb push_shared_string [114] 'execute'
0009 @0x00051cbf push_shared_string [39] 'cmd'
0010 @0x00051cc3 push_local_value [1] 
0011 @0x00051cc7 push_shared_string [115] 'sql_cmd'
0012 @0x00051ccb create_mapping [4] 
0013 @0x00051ccf push_local_value [0] 
0014 @0x00051cd3 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x00051cd7 return_from_fun [] 
0016 @0x00051cdb null_action [] 
0017 @0x00051cdf return_from_fun [] 
```

<a id="dba-init_startup_module"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: init_startup_module

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x590b4`；函数编号：`656`。

```text
0000 @0x00059144 check_args_num [0] 
0001 @0x00059148 enter_function [] 
0002 @0x0005914c push_shared_string [154] '/dba/log/'
0003 @0x00059150 call_in_user_fun [613, -127] assure_file
0004 @0x00059154 pop_stack [] 
0005 @0x00059158 push_shared_string [155] 'Done of init_startup_module.\n'
0006 @0x0005915c call_in_efun [90, 1] printf
0007 @0x00059160 pop_stack [] 
0008 @0x00059164 null_action [] 
0009 @0x00059168 return_from_fun [] 
```

<a id="dba-init_sql_module"></a>

## dba/pack_data/lib_dba32.pak/dba/start_dba.o :: init_sql_module

成员 SHA-256：`5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8`；函数记录偏移：`0x5b9c1`；函数编号：`762`。

```text
0000 @0x0005ba5b check_args_num [0] 
0001 @0x0005ba5f enter_function [] 
0002 @0x0005ba63 push_shared_string [378] ':memory:'
0003 @0x0005ba67 call_in_efun [423, 1] sql_open_db
0004 @0x0005ba6b push_global_vptr [68] 
0005 @0x0005ba6f assign_value [] 
0006 @0x0005ba73 pop_stack [] 
0007 @0x0005ba77 push_global_var [68] 
0008 @0x0005ba7b push_number(S) [-1] 
0009 @0x0005ba7f eq_value [] 
0010 @0x0005ba83 if [3, 2] 
0011 @0x0005ba87 push_shared_string [379] 'Failed to open memory database, startup halt.\n'
0012 @0x0005ba8b call_in_efun [28, 1] error
0013 @0x0005ba8f pop_stack [] 
0014 @0x0005ba93 push_shared_string [380] 'Done of init_sql_module.\n'
0015 @0x0005ba97 call_in_efun [90, 1] printf
0016 @0x0005ba9b pop_stack [] 
0017 @0x0005ba9f null_action [] 
0018 @0x0005baa3 return_from_fun [] 
```

<a id="dba-dba-daemons-dbaserver-cmd_create_content-o-main"></a>

## dba/pack_data/lib_dba32.pak/dba/daemons/dbaserver/cmd_create_content.o :: main

成员 SHA-256：`13307665ddc475dcb2600a14ad545cfcf4a52078f714075d21fbd07b513f0f93`；函数记录偏移：`0x253`；函数编号：`4`。

```text
0000 @0x00000483 check_args_num [0] 
0001 @0x00000487 enter_function [] 
0002 @0x0000048b push_local_value [0] 
0003 @0x0000048f is_type [10] 
0004 @0x00000493 not_value [] 
0005 @0x00000497 if [3, 2] 
0006 @0x0000049b push_number(S) [0] 
0007 @0x0000049f return_from_fun [] 
0008 @0x000004a3 null_action [] 
0009 @0x000004a7 push_shared_string [0] 'ddb'
0010 @0x000004ab push_shared_string [1] 'assure_connection'
0011 @0x000004af push_shared_string [4] '/daemons/dbd.c'
0012 @0x000004b3 call_in_other_fun_ex [4, 1] 
0013 @0x000004b7 push_local_vptr [-1] 
0014 @0x000004bb assign_value [] 
0015 @0x000004bf pop_stack [] 
0016 @0x000004c3 push_local_value [-1] 
0017 @0x000004c7 push_number(S) [-1] 
0018 @0x000004cb eq_value [] 
0019 @0x000004cf if [11, 2] 
0020 @0x000004d3 push_number(S) [0] 
0021 @0x000004d7 push_number(S) [-2] 
0022 @0x000004db push_local_value [1] 
0023 @0x000004df push_number(S) [9223] 
0024 @0x000004e3 push_shared_string [2] 'send_message'
0025 @0x000004e7 push_local_value [0] 
0026 @0x000004eb call_in_other_fun_ex [4, 4] 
0027 @0x000004ef pop_stack [] 
0028 @0x000004f3 push_number(S) [0] 
0029 @0x000004f7 return_from_fun [] 
0030 @0x000004fb null_action [] 
0031 @0x000004ff push_local_value [5] 
0032 @0x00000503 cast_type [4, 0] 
0033 @0x00000507 push_local_vptr [-4] 
0034 @0x0000050b assign_value [] 
0035 @0x0000050f pop_stack [] 
0036 @0x00000513 push_local_value [5] 
0037 @0x00000517 push_local_value [4] 
0038 @0x0000051b push_local_value [3] 
0039 @0x0000051f push_local_value [2] 
0040 @0x00000523 push_number(S) [1] 
0041 @0x00000527 call_in_efun [213, 5] create_check_sum
0042 @0x0000052b push_number(L) [-1418120497] 
0043 @0x0000052f xor_value [] 
0044 @0x00000533 call_in_efun [58, 1] int32
0045 @0x00000537 push_number(S) [3] 
0046 @0x0000053b call_in_efun [137, 0] time
0047 @0x0000053f call_in_efun [18, 2] ctime
0048 @0x00000543 push_local_value [-4] 
0049 @0x00000547 push_local_value [4] 
0050 @0x0000054b push_local_value [3] 
0051 @0x0000054f push_local_value [2] 
0052 @0x00000553 push_shared_string [5] 'insert into data (path, name, branch, content, time, checksum) values (?, ?, ?, ?, ?, ?)'
0053 @0x00000557 call_in_efun [447, 7] mysql_bind_parameters
0054 @0x0000055b push_local_vptr [-3] 
0055 @0x0000055f assign_value [] 
0056 @0x00000563 pop_stack [] 
0057 @0x00000567 push_local_value [-3] 
0058 @0x0000056b push_local_value [-1] 
0059 @0x0000056f push_global_var [14] 
0060 @0x00000573 call_fun_ptr [0, 3] 
0061 @0x00000577 push_number(S) [1] 
0062 @0x0000057b ge_value [] 
0063 @0x0000057f if [2, 2] 
0064 @0x00000583 push_number(S) [0] 
0065 @0x00000587 goto [1] 
0066 @0x0000058b push_number(S) [-1] 
0067 @0x0000058f push_local_vptr [-2] 
0068 @0x00000593 assign_value [] 
0069 @0x00000597 pop_stack [] 
0070 @0x0000059b push_number(S) [0] 
0071 @0x0000059f push_local_value [-2] 
0072 @0x000005a3 push_local_value [1] 
0073 @0x000005a7 push_number(S) [9223] 
0074 @0x000005ab push_shared_string [2] 'send_message'
0075 @0x000005af push_local_value [0] 
0076 @0x000005b3 call_in_other_fun_ex [4, 4] 
0077 @0x000005b7 pop_stack [] 
0078 @0x000005bb push_local_value [-4] 
0079 @0x000005bf push_local_value [4] 
0080 @0x000005c3 push_local_value [3] 
0081 @0x000005c7 push_local_value [2] 
0082 @0x000005cb push_shared_string [3] 'update_data_length_stat_info'
0083 @0x000005cf push_shared_string [6] '/dba/daemons/dbaserverd.c'
0084 @0x000005d3 call_in_other_fun_ex [4, 4] 
0085 @0x000005d7 pop_stack [] 
0086 @0x000005db push_number(S) [1] 
0087 @0x000005df return_from_fun [] 
0088 @0x000005e3 null_action [] 
0089 @0x000005e7 return_from_fun [] 
```

<a id="dba-dba-daemons-dbaserver-cmd_read_content-o-main"></a>

## dba/pack_data/lib_dba32.pak/dba/daemons/dbaserver/cmd_read_content.o :: main

成员 SHA-256：`3cf75021b5fc795f3f9a86b408e53862330492f83bb46f496e8f642b55e39213`；函数记录偏移：`0x405`；函数编号：`6`。

```text
0000 @0x000006b9 check_args_num [2] 
0001 @0x000006bd push_shared_string [6] 'data'
0002 @0x000006c1 resort_stack [] 
0003 @0x000006c5 enter_function [] 
0004 @0x000006c9 push_local_value [0] 
0005 @0x000006cd is_type [10] 
0006 @0x000006d1 not_value [] 
0007 @0x000006d5 if [3, 2] 
0008 @0x000006d9 push_number(S) [0] 
0009 @0x000006dd return_from_fun [] 
0010 @0x000006e1 null_action [] 
0011 @0x000006e5 push_shared_string [0] 'ddb'
0012 @0x000006e9 push_shared_string [1] 'assure_connection'
0013 @0x000006ed push_shared_string [4] '/daemons/dbd.c'
0014 @0x000006f1 call_in_other_fun_ex [4, 1] 
0015 @0x000006f5 push_local_vptr [-1] 
0016 @0x000006f9 assign_value [] 
0017 @0x000006fd pop_stack [] 
0018 @0x00000701 push_local_value [-1] 
0019 @0x00000705 push_number(S) [-1] 
0020 @0x00000709 eq_value [] 
0021 @0x0000070d if [11, 2] 
0022 @0x00000711 push_number(S) [0] 
0023 @0x00000715 push_number(S) [-2] 
0024 @0x00000719 push_local_value [1] 
0025 @0x0000071d push_number(S) [17409] 
0026 @0x00000721 push_shared_string [2] 'send_message'
0027 @0x00000725 push_local_value [0] 
0028 @0x00000729 call_in_other_fun_ex [4, 4] 
0029 @0x0000072d pop_stack [] 
0030 @0x00000731 push_number(S) [0] 
0031 @0x00000735 return_from_fun [] 
0032 @0x00000739 null_action [] 
0033 @0x0000073d push_local_value [5] 
0034 @0x00000741 push_shared_string [5] 'data1'
0035 @0x00000745 ne_value [] 
0036 @0x00000749 if [4, 2] 
0037 @0x0000074d push_shared_string [6] 'data'
0038 @0x00000751 push_local_vptr [5] 
0039 @0x00000755 assign_value [] 
0040 @0x00000759 pop_stack [] 
0041 @0x0000075d push_shared_string [7] 'select path, name, branch, content, checksum from '
0042 @0x00000761 push_local_value [5] 
0043 @0x00000765 add [] 
0044 @0x00000769 push_local_vptr [-3] 
0045 @0x0000076d assign_value [] 
0046 @0x00000771 pop_stack [] 
0047 @0x00000775 push_local_vptr [-3] 
0048 @0x00000779 push_local_value [4] 
0049 @0x0000077d push_local_value [3] 
0050 @0x00000781 push_local_value [2] 
0051 @0x00000785 push_shared_string [8] ' where path=? and name=? and branch=?'
0052 @0x00000789 call_in_efun [447, 4] mysql_bind_parameters
0053 @0x0000078d op_eq_value [2] 
0054 @0x00000791 pop_stack [] 
0055 @0x00000795 push_local_value [-3] 
0056 @0x00000799 push_local_value [-1] 
0057 @0x0000079d call_in_user_fun [955, -126] mysql_select
0058 @0x000007a1 push_local_vptr [-2] 
0059 @0x000007a5 assign_value [] 
0060 @0x000007a9 pop_stack [] 
0061 @0x000007ad push_local_value [-2] 
0062 @0x000007b1 not_value [] 
0063 @0x000007b5 if [13, 2] 
0064 @0x000007b9 null_action [] 
0065 @0x000007bd null_action [] 
0066 @0x000007c1 push_obj_vptr [2] 
0067 @0x000007c5 inc_value [1] 
0068 @0x000007c9 null_action [] 
0069 @0x000007cd push_shared_string [0] 'ddb'
0070 @0x000007d1 push_shared_string [3] 'notice_db_failed'
0071 @0x000007d5 push_shared_string [4] '/daemons/dbd.c'
0072 @0x000007d9 call_in_other_fun_ex [4, 1] 
0073 @0x000007dd pop_stack [] 
0074 @0x000007e1 push_number(S) [0] 
0075 @0x000007e5 return_from_fun [] 
0076 @0x000007e9 null_action [] 
0077 @0x000007ed push_local_value [-2] 
0078 @0x000007f1 sizeof [] 
0079 @0x000007f5 push_number(S) [1] 
0080 @0x000007f9 ne_value [] 
0081 @0x000007fd if [11, 2] 
0082 @0x00000801 push_number(S) [0] 
0083 @0x00000805 push_number(S) [-1] 
0084 @0x00000809 push_local_value [1] 
0085 @0x0000080d push_number(S) [17409] 
0086 @0x00000811 push_shared_string [2] 'send_message'
0087 @0x00000815 push_local_value [0] 
0088 @0x00000819 call_in_other_fun_ex [4, 4] 
0089 @0x0000081d pop_stack [] 
0090 @0x00000821 push_number(S) [1] 
0091 @0x00000825 return_from_fun [] 
0092 @0x00000829 null_action [] 
0093 @0x0000082d push_local_value [-2] 
0094 @0x00000831 push_number(S) [0] 
0095 @0x00000835 query_member [0] 
0096 @0x00000839 push_local_vptr [-5] 
0097 @0x0000083d assign_value [] 
0098 @0x00000841 is_type [7] 
0099 @0x00000845 not_value [] 
0100 @0x00000849 if [11, 2] 
0101 @0x0000084d push_number(S) [0] 
0102 @0x00000851 push_number(S) [-2] 
0103 @0x00000855 push_local_value [1] 
0104 @0x00000859 push_number(S) [17409] 
0105 @0x0000085d push_shared_string [2] 'send_message'
0106 @0x00000861 push_local_value [0] 
0107 @0x00000865 call_in_other_fun_ex [4, 4] 
0108 @0x00000869 pop_stack [] 
0109 @0x0000086d push_number(S) [1] 
0110 @0x00000871 return_from_fun [] 
0111 @0x00000875 null_action [] 
0112 @0x00000879 push_local_value [-5] 
0113 @0x0000087d push_shared_string [9] 'content'
0114 @0x00000881 query_member [0] 
0115 @0x00000885 cast_type [5, 0] 
0116 @0x00000889 push_local_vptr [-4] 
0117 @0x0000088d assign_value [] 
0118 @0x00000891 pop_stack [] 
0119 @0x00000895 push_local_value [-5] 
0120 @0x00000899 push_shared_string [10] 'path'
0121 @0x0000089d query_member [0] 
0122 @0x000008a1 push_local_vptr [2] 
0123 @0x000008a5 assign_value [] 
0124 @0x000008a9 pop_stack [] 
0125 @0x000008ad push_local_value [-5] 
0126 @0x000008b1 push_shared_string [11] 'name'
0127 @0x000008b5 query_member [0] 
0128 @0x000008b9 push_local_vptr [3] 
0129 @0x000008bd assign_value [] 
0130 @0x000008c1 pop_stack [] 
0131 @0x000008c5 push_local_value [-5] 
0132 @0x000008c9 push_shared_string [12] 'branch'
0133 @0x000008cd query_member [0] 
0134 @0x000008d1 push_local_vptr [4] 
0135 @0x000008d5 assign_value [] 
0136 @0x000008d9 pop_stack [] 
0137 @0x000008dd push_local_value [-4] 
0138 @0x000008e1 push_local_value [4] 
0139 @0x000008e5 push_local_value [3] 
0140 @0x000008e9 push_local_value [2] 
0141 @0x000008ed push_number(S) [1] 
0142 @0x000008f1 call_in_efun [213, 5] create_check_sum
0143 @0x000008f5 push_number(L) [-1418120497] 
0144 @0x000008f9 xor_value [] 
0145 @0x000008fd call_in_efun [58, 1] int32
0146 @0x00000901 push_local_value [-5] 
0147 @0x00000905 push_shared_string [13] 'checksum'
0148 @0x00000909 query_member [0] 
0149 @0x0000090d call_in_efun [5, 1] atoi
0150 @0x00000911 ne_value [] 
0151 @0x00000915 if [11, 2] 
0152 @0x00000919 push_number(S) [0] 
0153 @0x0000091d push_number(S) [-3] 
0154 @0x00000921 push_local_value [1] 
0155 @0x00000925 push_number(S) [17409] 
0156 @0x00000929 push_shared_string [2] 'send_message'
0157 @0x0000092d push_local_value [0] 
0158 @0x00000931 call_in_other_fun_ex [4, 4] 
0159 @0x00000935 pop_stack [] 
0160 @0x00000939 push_number(S) [1] 
0161 @0x0000093d return_from_fun [] 
0162 @0x00000941 null_action [] 
0163 @0x00000945 push_local_value [-4] 
0164 @0x00000949 push_number(S) [0] 
0165 @0x0000094d push_local_value [1] 
0166 @0x00000951 push_number(S) [17409] 
0167 @0x00000955 push_shared_string [2] 'send_message'
0168 @0x00000959 push_local_value [0] 
0169 @0x0000095d call_in_other_fun_ex [4, 4] 
0170 @0x00000961 pop_stack [] 
0171 @0x00000965 push_number(S) [1] 
0172 @0x00000969 return_from_fun [] 
0173 @0x0000096d null_action [] 
0174 @0x00000971 return_from_fun [] 
```

<a id="dba-dba-daemons-dbaserver-cmd_write_content-o-main"></a>

## dba/pack_data/lib_dba32.pak/dba/daemons/dbaserver/cmd_write_content.o :: main

成员 SHA-256：`aa739791d6573ef5c16754061537cc19fbae689ae95792d68a0617eb48a7210a`；函数记录偏移：`0x409`；函数编号：`6`。

```text
0000 @0x000006d8 check_args_num [0] 
0001 @0x000006dc enter_function [] 
0002 @0x000006e0 push_local_value [0] 
0003 @0x000006e4 is_type [10] 
0004 @0x000006e8 not_value [] 
0005 @0x000006ec if [3, 2] 
0006 @0x000006f0 push_number(S) [0] 
0007 @0x000006f4 return_from_fun [] 
0008 @0x000006f8 null_action [] 
0009 @0x000006fc push_local_value [5] 
0010 @0x00000700 cast_type [4, 0] 
0011 @0x00000704 push_local_vptr [-6] 
0012 @0x00000708 assign_value [] 
0013 @0x0000070c pop_stack [] 
0014 @0x00000710 push_local_value [2] 
0015 @0x00000714 push_shared_string [6] 'runtime'
0016 @0x00000718 eq_value [] 
0017 @0x0000071c if [2, 0] 
0018 @0x00000720 pop_stack [] 
0019 @0x00000724 push_obj_var [1] 
0020 @0x00000728 if [5, 0] 
0021 @0x0000072c pop_stack [] 
0022 @0x00000730 push_local_value [-6] 
0023 @0x00000734 push_shared_string [1] 'is_in_peeling'
0024 @0x00000738 push_shared_string [7] '/dba/daemons/peel_datad.c'
0025 @0x0000073c call_in_other_fun_ex [4, 1] 
0026 @0x00000740 if [11, 2] 
0027 @0x00000744 push_number(S) [0] 
0028 @0x00000748 push_number(S) [-2] 
0029 @0x0000074c push_local_value [1] 
0030 @0x00000750 push_number(S) [9219] 
0031 @0x00000754 push_shared_string [2] 'send_message'
0032 @0x00000758 push_local_value [0] 
0033 @0x0000075c call_in_other_fun_ex [4, 4] 
0034 @0x00000760 pop_stack [] 
0035 @0x00000764 push_number(S) [0] 
0036 @0x00000768 return_from_fun [] 
0037 @0x0000076c null_action [] 
0038 @0x00000770 push_shared_string [0] 'ddb'
0039 @0x00000774 push_shared_string [3] 'assure_connection'
0040 @0x00000778 push_shared_string [8] '/daemons/dbd.c'
0041 @0x0000077c call_in_other_fun_ex [4, 1] 
0042 @0x00000780 push_local_vptr [-1] 
0043 @0x00000784 assign_value [] 
0044 @0x00000788 pop_stack [] 
0045 @0x0000078c push_local_value [-1] 
0046 @0x00000790 push_number(S) [-1] 
0047 @0x00000794 eq_value [] 
0048 @0x00000798 if [11, 2] 
0049 @0x0000079c push_number(S) [0] 
0050 @0x000007a0 push_number(S) [-2] 
0051 @0x000007a4 push_local_value [1] 
0052 @0x000007a8 push_number(S) [9219] 
0053 @0x000007ac push_shared_string [2] 'send_message'
0054 @0x000007b0 push_local_value [0] 
0055 @0x000007b4 call_in_other_fun_ex [4, 4] 
0056 @0x000007b8 pop_stack [] 
0057 @0x000007bc push_number(S) [0] 
0058 @0x000007c0 return_from_fun [] 
0059 @0x000007c4 null_action [] 
0060 @0x000007c8 push_local_value [5] 
0061 @0x000007cc push_local_value [4] 
0062 @0x000007d0 push_local_value [3] 
0063 @0x000007d4 push_local_value [2] 
0064 @0x000007d8 push_number(S) [1] 
0065 @0x000007dc call_in_efun [213, 5] create_check_sum
0066 @0x000007e0 push_number(L) [-1418120497] 
0067 @0x000007e4 xor_value [] 
0068 @0x000007e8 call_in_efun [58, 1] int32
0069 @0x000007ec push_local_vptr [-4] 
0070 @0x000007f0 assign_value [] 
0071 @0x000007f4 pop_stack [] 
0072 @0x000007f8 push_local_value [4] 
0073 @0x000007fc push_local_value [3] 
0074 @0x00000800 push_local_value [2] 
0075 @0x00000804 push_local_value [-4] 
0076 @0x00000808 push_number(S) [3] 
0077 @0x0000080c call_in_efun [137, 0] time
0078 @0x00000810 call_in_efun [18, 2] ctime
0079 @0x00000814 push_local_value [-6] 
0080 @0x00000818 push_local_value [4] 
0081 @0x0000081c push_local_value [3] 
0082 @0x00000820 push_local_value [2] 
0083 @0x00000824 push_shared_string [9] 'update data set path=?, name=?, branch = ?, content=?, time=?, checksum=? where path=? and name=? and branch=?'
0084 @0x00000828 call_in_efun [447, 10] mysql_bind_parameters
0085 @0x0000082c push_local_vptr [-5] 
0086 @0x00000830 assign_value [] 
0087 @0x00000834 pop_stack [] 
0088 @0x00000838 push_local_value [-5] 
0089 @0x0000083c push_local_value [-1] 
0090 @0x00000840 push_global_var [14] 
0091 @0x00000844 call_fun_ptr [0, 3] 
0092 @0x00000848 push_local_vptr [-3] 
0093 @0x0000084c assign_value [] 
0094 @0x00000850 push_number(S) [1] 
0095 @0x00000854 ge_value [] 
0096 @0x00000858 if [5, 2] 
0097 @0x0000085c push_number(S) [0] 
0098 @0x00000860 push_local_vptr [-2] 
0099 @0x00000864 assign_value [] 
0100 @0x00000868 pop_stack [] 
0101 @0x0000086c goto [13] 
0102 @0x00000870 push_local_value [-3] 
0103 @0x00000874 push_number(S) [0] 
0104 @0x00000878 eq_value [] 
0105 @0x0000087c if [5, 2] 
0106 @0x00000880 push_number(S) [-1] 
0107 @0x00000884 push_local_vptr [-2] 
0108 @0x00000888 assign_value [] 
0109 @0x0000088c pop_stack [] 
0110 @0x00000890 goto [4] 
0111 @0x00000894 push_number(S) [-2] 
0112 @0x00000898 push_local_vptr [-2] 
0113 @0x0000089c assign_value [] 
0114 @0x000008a0 pop_stack [] 
0115 @0x000008a4 push_number(S) [0] 
0116 @0x000008a8 push_local_value [-2] 
0117 @0x000008ac push_local_value [1] 
0118 @0x000008b0 push_number(S) [9219] 
0119 @0x000008b4 push_shared_string [2] 'send_message'
0120 @0x000008b8 push_local_value [0] 
0121 @0x000008bc call_in_other_fun_ex [4, 4] 
0122 @0x000008c0 pop_stack [] 
0123 @0x000008c4 push_local_value [2] 
0124 @0x000008c8 push_shared_string [6] 'runtime'
0125 @0x000008cc eq_value [] 
0126 @0x000008d0 if [2, 0] 
0127 @0x000008d4 pop_stack [] 
0128 @0x000008d8 push_obj_var [1] 
0129 @0x000008dc if [6, 2] 
0130 @0x000008e0 push_local_value [-6] 
0131 @0x000008e4 push_local_value [3] 
0132 @0x000008e8 push_shared_string [4] 'add_to_online_list'
0133 @0x000008ec push_shared_string [7] '/dba/daemons/peel_datad.c'
0134 @0x000008f0 call_in_other_fun_ex [4, 2] 
0135 @0x000008f4 pop_stack [] 
0136 @0x000008f8 push_local_value [-6] 
0137 @0x000008fc push_local_value [4] 
0138 @0x00000900 push_local_value [3] 
0139 @0x00000904 push_local_value [2] 
0140 @0x00000908 push_shared_string [5] 'update_data_length_stat_info'
0141 @0x0000090c push_shared_string [10] '/dba/daemons/dbaserverd.c'
0142 @0x00000910 call_in_other_fun_ex [4, 4] 
0143 @0x00000914 pop_stack [] 
0144 @0x00000918 push_number(S) [1] 
0145 @0x0000091c return_from_fun [] 
0146 @0x00000920 null_action [] 
0147 @0x00000924 return_from_fun [] 
```

<a id="gs-daemons-filed-o-dba_comm"></a>

## gs/pack_data/lib_gs32.pak/daemons/filed.o :: dba_comm

成员 SHA-256：`48a71b20eabed90d501c07984c8fd5d9eec68160593bf191a53980942a1a1059`；函数记录偏移：`0x1e27`；函数编号：`101`。

```text
0000 @0x00001fa1 check_args_num [0] 
0001 @0x00001fa5 enter_function [] 
0002 @0x00001fa9 call_in_efun [290, 0] set_as_daemon
0003 @0x00001fad pop_stack [] 
0004 @0x00001fb1 push_shared_string [151] 'FILE_D.dba_comm daemon start...\n'
0005 @0x00001fb5 call_in_efun [90, 1] printf
0006 @0x00001fb9 pop_stack [] 
0007 @0x00001fbd push_obj_var [12] 
0008 @0x00001fc1 push_number(S) [-1] 
0009 @0x00001fc5 ne_value [] 
0010 @0x00001fc9 if [4, 0] 
0011 @0x00001fcd pop_stack [] 
0012 @0x00001fd1 push_obj_var [4] 
0013 @0x00001fd5 push_number(S) [0] 
0014 @0x00001fd9 lt_value [] 
0015 @0x00001fdd if [11, 2] 
0016 @0x00001fe1 push_number(S) [2] 
0017 @0x00001fe5 push_shared_string [2] 'wait_config'
0018 @0x00001fe9 push_shared_string [11] '/daemons/configd.c'
0019 @0x00001fed call_in_other_fun_ex [4, 1] 
0020 @0x00001ff1 if [3, 1] 
0021 @0x00001ff5 pop_stack [] 
0022 @0x00001ff9 push_number(S) [1] 
0023 @0x00001ffd call_in_user_fun [692, -127] check_server_status
0024 @0x00002001 if [1, 2] 
0025 @0x00002005 goto [1] 
0026 @0x00002009 goto [-20] 
0027 @0x0000200d push_obj_var [10] 
0028 @0x00002011 if [8, 2] 
0029 @0x00002015 call_in_user_fun [7, 0] try_to_connect_dba
0030 @0x00002019 not_value [] 
0031 @0x0000201d if [4, 2] 
0032 @0x00002021 push_number(S) [2] 
0033 @0x00002025 call_in_efun [298, 1] sleep
0034 @0x00002029 pop_stack [] 
0035 @0x0000202d goto [-7] 
0036 @0x00002031 null_action [] 
0037 @0x00002035 push_local_vptr [-1] 
0038 @0x00002039 push_obj_var [3] 
0039 @0x0000203d call_in_efun [314, 2] receive_from_queue
0040 @0x00002041 push_number(S) [0] 
0041 @0x00002045 ne_value [] 
0042 @0x00002049 if [1, 2] 
0043 @0x0000204d goto [79] 
0044 @0x00002051 push_number(S) [1] 
0045 @0x00002055 if [76, 2] 
0046 @0x00002059 push_local_value [-1] 
0047 @0x0000205d push_shared_string [33] 'operate'
0048 @0x00002061 query_member [0] 
0049 @0x00002065 push_number(S) [0] 
0050 @0x00002069 eq_value [] 
0051 @0x0000206d if [5, 2] 
0052 @0x00002071 push_shared_string [152] 'FILE_D.dba_comm done.\n'
0053 @0x00002075 call_in_efun [90, 1] printf
0054 @0x00002079 pop_stack [] 
0055 @0x0000207d return_from_fun [] 
0056 @0x00002081 null_action [] 
0057 @0x00002085 push_obj_var [6] 
0058 @0x00002089 push_local_value [-1] 
0059 @0x0000208d push_shared_string [60] 'cookie'
0060 @0x00002091 query_member [0] 
0061 @0x00002095 query_member [0] 
0062 @0x00002099 is_type [7] 
0063 @0x0000209d not_value [] 
0064 @0x000020a1 if [1, 2] 
0065 @0x000020a5 goto [56] 
0066 @0x000020a9 call_in_efun [137, 0] time
0067 @0x000020ad push_local_vptr [-1] 
0068 @0x000020b1 push_shared_string [153] 'send_time'
0069 @0x000020b5 query_member_ptr [0] 
0070 @0x000020b9 assign_value [] 
0071 @0x000020bd pop_stack [] 
0072 @0x000020c1 push_local_value [-1] 
0073 @0x000020c5 push_obj_vptr [6] 
0074 @0x000020c9 push_local_value [-1] 
0075 @0x000020cd push_shared_string [60] 'cookie'
0076 @0x000020d1 query_member [0] 
0077 @0x000020d5 query_member_ptr [0] 
0078 @0x000020d9 assign_value [] 
0079 @0x000020dd pop_stack [] 
0080 @0x000020e1 push_obj_var [12] 
0081 @0x000020e5 push_number(S) [-1] 
0082 @0x000020e9 eq_value [] 
0083 @0x000020ed if [4, 0] 
0084 @0x000020f1 pop_stack [] 
0085 @0x000020f5 push_obj_var [4] 
0086 @0x000020f9 push_number(S) [0] 
0087 @0x000020fd lt_value [] 
0088 @0x00002101 if [5, 2] 
0089 @0x00002105 push_local_value [-1] 
0090 @0x00002109 call_in_user_fun [109, 1] failure_routine
0091 @0x0000210d pop_stack [] 
0092 @0x00002111 goto [29] 
0093 @0x00002115 null_action [] 
0094 @0x00002119 push_local_value [-1] 
0095 @0x0000211d push_shared_string [59] 'expired'
0096 @0x00002121 query_member [0] 
0097 @0x00002125 call_in_efun [137, 0] time
0098 @0x00002129 le_value [] 
0099 @0x0000212d if [5, 2] 
0100 @0x00002131 push_local_value [-1] 
0101 @0x00002135 call_in_user_fun [109, 1] failure_routine
0102 @0x00002139 pop_stack [] 
0103 @0x0000213d goto [18] 
0104 @0x00002141 null_action [] 
0105 @0x00002145 call_in_user_fun [7, 0] try_to_connect_dba
0106 @0x00002149 not_value [] 
0107 @0x0000214d if [5, 2] 
0108 @0x00002151 push_number(S) [2] 
0109 @0x00002155 call_in_efun [298, 1] sleep
0110 @0x00002159 pop_stack [] 
0111 @0x0000215d goto [-68] 
0112 @0x00002161 null_action [] 
0113 @0x00002165 push_local_value [-1] 
0114 @0x00002169 push_shared_string [154] 'req_buf'
0115 @0x0000216d query_member [0] 
0116 @0x00002171 push_obj_var [4] 
0117 @0x00002175 call_in_efun [527, 2] send_to_client
0118 @0x00002179 pop_stack [] 
0119 @0x0000217d goto [2] 
0120 @0x00002181 null_action [] 
0121 @0x00002185 goto [-78] 
0122 @0x00002189 null_action [] 
0123 @0x0000218d goto [-87] 
0124 @0x00002191 null_action [] 
0125 @0x00002195 return_from_fun [] 
```

<a id="gs-daemons-filed-o-try_to_connect_dba"></a>

## gs/pack_data/lib_gs32.pak/daemons/filed.o :: try_to_connect_dba

成员 SHA-256：`48a71b20eabed90d501c07984c8fd5d9eec68160593bf191a53980942a1a1059`；函数记录偏移：`0x39fb`；函数编号：`7`。

```text
0000 @0x00003b7d check_args_num [0] 
0001 @0x00003b81 enter_function [] 
0002 @0x00003b85 push_obj_var [4] 
0003 @0x00003b89 push_number(S) [0] 
0004 @0x00003b8d lt_value [] 
0005 @0x00003b91 if [2, 2] 
0006 @0x00003b95 create_mapping [0] 
0007 @0x00003b99 goto [2] 
0008 @0x00003b9d push_obj_var [4] 
0009 @0x00003ba1 call_in_efun [515, 1] get_client_info
0010 @0x00003ba5 push_local_vptr [-2] 
0011 @0x00003ba9 assign_value [] 
0012 @0x00003bad pop_stack [] 
0013 @0x00003bb1 push_local_value [-2] 
0014 @0x00003bb5 push_shared_string [36] 'alive'
0015 @0x00003bb9 query_member [0] 
0016 @0x00003bbd not_value [] 
0017 @0x00003bc1 if [97, 2] 
0018 @0x00003bc5 push_shared_string [35] 'get_dba_address'
0019 @0x00003bc9 push_shared_string [11] '/daemons/configd.c'
0020 @0x00003bcd call_in_other_fun_ex [4, 0] 
0021 @0x00003bd1 push_local_vptr [-1] 
0022 @0x00003bd5 assign_value [] 
0023 @0x00003bd9 is_type [7] 
0024 @0x00003bdd not_value [] 
0025 @0x00003be1 if [2, 2] 
0026 @0x00003be5 push_number(S) [0] 
0027 @0x00003be9 return_from_fun [] 
0028 @0x00003bed push_shared_string [17] 'mfiledconn'
0029 @0x00003bf1 call_in_efun [305, 1] take_mutex
0030 @0x00003bf5 push_number(S) [0] 
0031 @0x00003bf9 ne_value [] 
0032 @0x00003bfd if [2, 2] 
0033 @0x00003c01 push_number(S) [0] 
0034 @0x00003c05 return_from_fun [] 
0035 @0x00003c09 push_local_value [-1] 
0036 @0x00003c0d push_shared_string [37] 'port'
0037 @0x00003c11 query_member [0] 
0038 @0x00003c15 push_local_value [-1] 
0039 @0x00003c19 push_shared_string [38] 'ip'
0040 @0x00003c1d query_member [0] 
0041 @0x00003c21 push_shared_string [39] 'Connect to DBA: %O:%O.\n'
0042 @0x00003c25 call_in_efun [90, 3] printf
0043 @0x00003c29 pop_stack [] 
0044 @0x00003c2d call_in_efun [267, 0] socket
0045 @0x00003c31 push_local_vptr [-3] 
0046 @0x00003c35 assign_value [] 
0047 @0x00003c39 pop_stack [] 
0048 @0x00003c3d push_number(L) [4194304] 
0049 @0x00003c41 push_number(S) [8] 
0050 @0x00003c45 push_number(S) [1] 
0051 @0x00003c49 push_local_value [-3] 
0052 @0x00003c4d call_in_efun [266, 4] set_sock_opt
0053 @0x00003c51 pop_stack [] 
0054 @0x00003c55 push_number(L) [4194304] 
0055 @0x00003c59 push_number(S) [7] 
0056 @0x00003c5d push_number(S) [1] 
0057 @0x00003c61 push_local_value [-3] 
0058 @0x00003c65 call_in_efun [266, 4] set_sock_opt
0059 @0x00003c69 pop_stack [] 
0060 @0x00003c6d push_local_value [-1] 
0061 @0x00003c71 push_shared_string [37] 'port'
0062 @0x00003c75 query_member [0] 
0063 @0x00003c79 push_local_value [-1] 
0064 @0x00003c7d push_shared_string [38] 'ip'
0065 @0x00003c81 query_member [0] 
0066 @0x00003c85 push_local_value [-3] 
0067 @0x00003c89 call_in_efun [254, 3] connect
0068 @0x00003c8d push_number(S) [0] 
0069 @0x00003c91 ne_value [] 
0070 @0x00003c95 if [6, 1] 
0071 @0x00003c99 pop_stack [] 
0072 @0x00003c9d push_local_value [-3] 
0073 @0x00003ca1 call_in_efun [508, 1] bind_to_client
0074 @0x00003ca5 call_in_user_fun [117, 1] set_dba_client_no
0075 @0x00003ca9 push_number(S) [0] 
0076 @0x00003cad lt_value [] 
0077 @0x00003cb1 if [12, 2] 
0078 @0x00003cb5 push_shared_string [40] 'Connect to DBA failed.\n'
0079 @0x00003cb9 call_in_efun [90, 1] printf
0080 @0x00003cbd pop_stack [] 
0081 @0x00003cc1 push_local_value [-3] 
0082 @0x00003cc5 call_in_efun [255, 1] close_socket
0083 @0x00003cc9 pop_stack [] 
0084 @0x00003ccd push_shared_string [17] 'mfiledconn'
0085 @0x00003cd1 call_in_efun [306, 1] give_mutex
0086 @0x00003cd5 pop_stack [] 
0087 @0x00003cd9 push_number(S) [0] 
0088 @0x00003cdd return_from_fun [] 
0089 @0x00003ce1 null_action [] 
0090 @0x00003ce5 push_shared_string [41] 'internal'
0091 @0x00003ce9 push_shared_string [42] 'type'
0092 @0x00003ced push_number(S) [-1] 
0093 @0x00003cf1 push_shared_string [43] 'peek'
0094 @0x00003cf5 push_number(L) [131000] 
0095 @0x00003cf9 push_shared_string [44] 'size'
0096 @0x00003cfd create_mapping [6] 
0097 @0x00003d01 push_obj_var [4] 
0098 @0x00003d05 call_in_efun [510, 2] set_client_attrib
0099 @0x00003d09 pop_stack [] 
0100 @0x00003d0d push_number(S) [0] 
0101 @0x00003d11 call_in_efun [521, 1] make32
0102 @0x00003d15 push_global_var [64] 
0103 @0x00003d19 push_number(S) [5] 
0104 @0x00003d1d push_number(S) [14849] 
0105 @0x00003d21 push_obj_var [4] 
0106 @0x00003d25 call_in_efun [527, 5] send_to_client
0107 @0x00003d29 pop_stack [] 
0108 @0x00003d2d push_shared_string [45] 'Connected to DBA.\n'
0109 @0x00003d31 call_in_efun [90, 1] printf
0110 @0x00003d35 pop_stack [] 
0111 @0x00003d39 push_shared_string [17] 'mfiledconn'
0112 @0x00003d3d call_in_efun [306, 1] give_mutex
0113 @0x00003d41 pop_stack [] 
0114 @0x00003d45 null_action [] 
0115 @0x00003d49 push_number(S) [1] 
0116 @0x00003d4d return_from_fun [] 
0117 @0x00003d51 null_action [] 
0118 @0x00003d55 return_from_fun [] 
```

<a id="gs-daemons-filed-o-create_content"></a>

## gs/pack_data/lib_gs32.pak/daemons/filed.o :: create_content

成员 SHA-256：`48a71b20eabed90d501c07984c8fd5d9eec68160593bf191a53980942a1a1059`；函数记录偏移：`0xb73d`；函数编号：`15`。

```text
0000 @0x0000b839 check_args_num [0] 
0001 @0x0000b83d enter_function [] 
0002 @0x0000b841 push_number(S) [0] 
0003 @0x0000b845 push_local_value [3] 
0004 @0x0000b849 push_local_value [2] 
0005 @0x0000b84d push_local_value [1] 
0006 @0x0000b851 push_local_value [0] 
0007 @0x0000b855 push_number(S) [0] 
0008 @0x0000b859 push_number(S) [4] 
0009 @0x0000b85d push_number(S) [1] 
0010 @0x0000b861 call_in_user_fun [111, 8] do_operate
0011 @0x0000b865 return_from_fun [] 
0012 @0x0000b869 null_action [] 
0013 @0x0000b86d return_from_fun [] 
```

<a id="gs-daemons-filed-o-read_content"></a>

## gs/pack_data/lib_gs32.pak/daemons/filed.o :: read_content

成员 SHA-256：`48a71b20eabed90d501c07984c8fd5d9eec68160593bf191a53980942a1a1059`；函数记录偏移：`0xc1fb`；函数编号：`9`。

```text
0000 @0x0000c346 check_args_num [0] 
0001 @0x0000c34a enter_function [] 
0002 @0x0000c34e push_local_value [3] 
0003 @0x0000c352 case_check_string [2, 52] 
0004 @0x0000c356 case_check_string [19, 53] 
0005 @0x0000c35a case_default [29] 
0006 @0x0000c35e push_number(S) [0] 
0007 @0x0000c362 push_number(S) [0] 
0008 @0x0000c366 push_local_value [2] 
0009 @0x0000c36a push_local_value [1] 
0010 @0x0000c36e push_local_value [0] 
0011 @0x0000c372 push_number(S) [0] 
0012 @0x0000c376 push_number(S) [1] 
0013 @0x0000c37a push_number(S) [1] 
0014 @0x0000c37e call_in_user_fun [111, 8] do_operate
0015 @0x0000c382 push_local_vptr [-1] 
0016 @0x0000c386 assign_value [] 
0017 @0x0000c38a pop_stack [] 
0018 @0x0000c38e push_local_value [-1] 
0019 @0x0000c392 is_type [4] 
0020 @0x0000c396 if [2, 2] 
0021 @0x0000c39a push_local_value [-1] 
0022 @0x0000c39e return_from_fun [] 
0023 @0x0000c3a2 null_action [] 
0024 @0x0000c3a6 push_number(S) [0] 
0025 @0x0000c3aa push_number(S) [0] 
0026 @0x0000c3ae push_local_value [2] 
0027 @0x0000c3b2 push_local_value [1] 
0028 @0x0000c3b6 push_local_value [0] 
0029 @0x0000c3ba push_number(S) [0] 
0030 @0x0000c3be push_number(S) [22] 
0031 @0x0000c3c2 push_number(S) [1] 
0032 @0x0000c3c6 call_in_user_fun [111, 8] do_operate
0033 @0x0000c3ca return_from_fun [] 
0034 @0x0000c3ce null_action [] 
0035 @0x0000c3d2 push_number(S) [0] 
0036 @0x0000c3d6 push_number(S) [0] 
0037 @0x0000c3da push_local_value [2] 
0038 @0x0000c3de push_local_value [1] 
0039 @0x0000c3e2 push_local_value [0] 
0040 @0x0000c3e6 push_number(S) [0] 
0041 @0x0000c3ea push_number(S) [1] 
0042 @0x0000c3ee push_number(S) [1] 
0043 @0x0000c3f2 call_in_user_fun [111, 8] do_operate
0044 @0x0000c3f6 return_from_fun [] 
0045 @0x0000c3fa null_action [] 
0046 @0x0000c3fe null_action [] 
0047 @0x0000c402 return_from_fun [] 
```

<a id="gs-daemons-filed-o-write_content"></a>

## gs/pack_data/lib_gs32.pak/daemons/filed.o :: write_content

成员 SHA-256：`48a71b20eabed90d501c07984c8fd5d9eec68160593bf191a53980942a1a1059`；函数记录偏移：`0x21844`；函数编号：`11`。

```text
0000 @0x0002193f check_args_num [0] 
0001 @0x00021943 enter_function [] 
0002 @0x00021947 push_number(S) [0] 
0003 @0x0002194b push_local_value [3] 
0004 @0x0002194f push_local_value [2] 
0005 @0x00021953 push_local_value [1] 
0006 @0x00021957 push_local_value [0] 
0007 @0x0002195b push_number(S) [0] 
0008 @0x0002195f push_number(S) [2] 
0009 @0x00021963 push_number(S) [1] 
0010 @0x00021967 call_in_user_fun [111, 8] do_operate
0011 @0x0002196b return_from_fun [] 
0012 @0x0002196f null_action [] 
0013 @0x00021973 return_from_fun [] 
```

<a id="gs-gs-clone-char-login-o-thread_process_command"></a>

## gs/pack_data/lib_gs32.pak/gs/clone/char/login.o :: thread_process_command

成员 SHA-256：`0e5aa235e3033a52424ee0210e1dbd1297f0b109d0bba6eee8675c7cfd628650`；函数记录偏移：`0x2c6c`；函数编号：`218`。

```text
0000 @0x00002d45 check_args_num [0] 
0001 @0x00002d49 enter_function [] 
0002 @0x00002d4d new_thread_catch [7] 
0003 @0x00002d51 push_local_value [1] 
0004 @0x00002d55 push_local_value [0] 
0005 @0x00002d59 call_in_user_fun [215, 2] _process_command
0006 @0x00002d5d pop_stack [] 
0007 @0x00002d61 push_number(S) [0] 
0008 @0x00002d65 end_thread_catch [] 
0009 @0x00002d69 goto [1] 
0010 @0x00002d6d caught_error [] 
0011 @0x00002d71 pop_stack [] 
0012 @0x00002d75 push_global_var [340] 
0013 @0x00002d79 push_local_value [0] 
0014 @0x00002d7d query_member [0] 
0015 @0x00002d81 if [5, 2] 
0016 @0x00002d85 null_action [] 
0017 @0x00002d89 null_action [] 
0018 @0x00002d8d push_global_vptr [339] 
0019 @0x00002d91 inc_value [-1] 
0020 @0x00002d95 null_action [] 
0021 @0x00002d99 null_action [] 
0022 @0x00002d9d return_from_fun [] 
```

<a id="gs-gs-clone-char-login-o-_process_command"></a>

## gs/pack_data/lib_gs32.pak/gs/clone/char/login.o :: _process_command

成员 SHA-256：`0e5aa235e3033a52424ee0210e1dbd1297f0b109d0bba6eee8675c7cfd628650`；函数记录偏移：`0x3aaa`；函数编号：`215`。

```text
0000 @0x00003d08 check_args_num [0] 
0001 @0x00003d0c enter_function [] 
0002 @0x00003d10 call_in_user_fun [6, 0] unresolved/inherited/builtin
0003 @0x00003d14 push_number(S) [-1] 
0004 @0x00003d18 eq_value [] 
0005 @0x00003d1c if [1, 2] 
0006 @0x00003d20 return_from_fun [] 
0007 @0x00003d24 push_shared_string [32] 'deny_cmd'
0008 @0x00003d28 push_shared_string [8] 'lite_query_temp'
0009 @0x00003d2c push_this_object [] 
0010 @0x00003d30 call_in_other_fun_ex [4, 1] 
0011 @0x00003d34 if [5, 2] 
0012 @0x00003d38 push_number(S) [0] 
0013 @0x00003d3c call_in_user_fun [213, 1] set_operating
0014 @0x00003d40 pop_stack [] 
0015 @0x00003d44 return_from_fun [] 
0016 @0x00003d48 null_action [] 
0017 @0x00003d4c push_local_value [0] 
0018 @0x00003d50 case_check_string [17, 19] 
0019 @0x00003d54 case_check_string [52, 10] 
0020 @0x00003d58 case_check_string [58, 20] 
0021 @0x00003d5c case_check_string [62, 21] 
0022 @0x00003d60 case_check_string [62, 22] 
0023 @0x00003d64 case_check_string [62, 23] 
0024 @0x00003d68 case_check_string [62, 24] 
0025 @0x00003d6c case_check_string [62, 25] 
0026 @0x00003d70 case_check_string [132, 11] 
0027 @0x00003d74 case_check_string [150, 26] 
0028 @0x00003d78 case_check_string [162, 27] 
0029 @0x00003d7c case_check_string [177, 28] 
0030 @0x00003d80 case_check_string [207, 15] 
0031 @0x00003d84 case_check_string [219, 29] 
0032 @0x00003d88 case_check_string [228, 30] 
0033 @0x00003d8c case_check_string [229, 17] 
0034 @0x00003d90 case_check_string [238, 31] 
0035 @0x00003d94 case_default [247] 
0036 @0x00003d98 push_local_value [1] 
0037 @0x00003d9c push_number(S) [0] 
0038 @0x00003da0 query_member [0] 
0039 @0x00003da4 push_shared_string [33] 'client_no'
0040 @0x00003da8 push_local_value [1] 
0041 @0x00003dac push_number(S) [1] 
0042 @0x00003db0 query_member [0] 
0043 @0x00003db4 push_shared_string [34] 'account'
0044 @0x00003db8 push_local_value [1] 
0045 @0x00003dbc push_number(S) [2] 
0046 @0x00003dc0 query_member [0] 
0047 @0x00003dc4 push_shared_string [35] 'auth_key'
0048 @0x00003dc8 push_local_value [1] 
0049 @0x00003dcc push_number(S) [3] 
0050 @0x00003dd0 query_member [0] 
0051 @0x00003dd4 push_shared_string [36] 'seed'
0052 @0x00003dd8 push_local_value [1] 
0053 @0x00003ddc push_number(S) [4] 
0054 @0x00003de0 query_member [0] 
0055 @0x00003de4 push_shared_string [37] 'client_type'
0056 @0x00003de8 push_local_value [1] 
0057 @0x00003dec push_number(S) [5] 
0058 @0x00003df0 query_member [0] 
0059 @0x00003df4 push_shared_string [38] 'mobile_key'
0060 @0x00003df8 push_local_value [1] 
0061 @0x00003dfc push_number(S) [6] 
0062 @0x00003e00 query_member [0] 
0063 @0x00003e04 push_shared_string [39] 'computer_id'
0064 @0x00003e08 create_mapping [14] 
0065 @0x00003e0c push_this_object [] 
0066 @0x00003e10 push_shared_string [9] 'do_login'
0067 @0x00003e14 push_shared_string [1] '/gs/daemons/logind.c'
0068 @0x00003e18 call_in_other_fun_ex [4, 2] 
0069 @0x00003e1c pop_stack [] 
0070 @0x00003e20 goto [213] 
0071 @0x00003e24 null_action [] 
0072 @0x00003e28 push_this_object [] 
0073 @0x00003e2c push_shared_string [10] 'replace_login'
0074 @0x00003e30 push_shared_string [1] '/gs/daemons/logind.c'
0075 @0x00003e34 call_in_other_fun_ex [4, 1] 
0076 @0x00003e38 pop_stack [] 
0077 @0x00003e3c goto [206] 
0078 @0x00003e40 null_action [] 
0079 @0x00003e44 push_local_value [0] 
0080 @0x00003e48 call_fun_ptr [4, 1] 
0081 @0x00003e4c pop_stack [] 
0082 @0x00003e50 goto [201] 
0083 @0x00003e54 null_action [] 
0084 @0x00003e58 null_action [] 
0085 @0x00003e5c null_action [] 
0086 @0x00003e60 null_action [] 
0087 @0x00003e64 null_action [] 
0088 @0x00003e68 push_shared_string [40] 'enabled'
0089 @0x00003e6c call_in_user_fun [96, 1] unresolved/inherited/builtin
0090 @0x00003e70 not_value [] 
0091 @0x00003e74 if [1, 2] 
0092 @0x00003e78 goto [191] 
0093 @0x00003e7c push_local_value [1] 
0094 @0x00003e80 sizeof [] 
0095 @0x00003e84 case_check_number [5, 0] 
0096 @0x00003e88 case_check_number [9, 1] 
0097 @0x00003e8c case_check_number [16, 2] 
0098 @0x00003e90 case_check_number [26, 3] 
0099 @0x00003e94 case_check_number [39, 4] 
0100 @0x00003e98 case_default [55] 
0101 @0x00003e9c push_local_value [0] 
0102 @0x00003ea0 call_fun_ptr [4, 1] 
0103 @0x00003ea4 pop_stack [] 
0104 @0x00003ea8 goto [52] 
0105 @0x00003eac null_action [] 
0106 @0x00003eb0 push_local_value [1] 
0107 @0x00003eb4 push_number(S) [0] 
0108 @0x00003eb8 query_member [0] 
0109 @0x00003ebc push_local_value [0] 
0110 @0x00003ec0 call_fun_ptr [4, 2] 
0111 @0x00003ec4 pop_stack [] 
0112 @0x00003ec8 goto [44] 
0113 @0x00003ecc null_action [] 
0114 @0x00003ed0 push_local_value [1] 
0115 @0x00003ed4 push_number(S) [1] 
0116 @0x00003ed8 query_member [0] 
0117 @0x00003edc push_local_value [1] 
0118 @0x00003ee0 push_number(S) [0] 
0119 @0x00003ee4 query_member [0] 
0120 @0x00003ee8 push_local_value [0] 
0121 @0x00003eec call_fun_ptr [4, 3] 
0122 @0x00003ef0 pop_stack [] 
0123 @0x00003ef4 goto [33] 
0124 @0x00003ef8 null_action [] 
0125 @0x00003efc push_local_value [1] 
0126 @0x00003f00 push_number(S) [2] 
0127 @0x00003f04 query_member [0] 
0128 @0x00003f08 push_local_value [1] 
0129 @0x00003f0c push_number(S) [1] 
0130 @0x00003f10 query_member [0] 
0131 @0x00003f14 push_local_value [1] 
0132 @0x00003f18 push_number(S) [0] 
0133 @0x00003f1c query_member [0] 
0134 @0x00003f20 push_local_value [0] 
0135 @0x00003f24 call_fun_ptr [4, 4] 
0136 @0x00003f28 pop_stack [] 
0137 @0x00003f2c goto [19] 
0138 @0x00003f30 null_action [] 
0139 @0x00003f34 push_local_value [1] 
0140 @0x00003f38 push_number(S) [3] 
0141 @0x00003f3c query_member [0] 
0142 @0x00003f40 push_local_value [1] 
0143 @0x00003f44 push_number(S) [2] 
0144 @0x00003f48 query_member [0] 
0145 @0x00003f4c push_local_value [1] 
0146 @0x00003f50 push_number(S) [1] 
0147 @0x00003f54 query_member [0] 
0148 @0x00003f58 push_local_value [1] 
0149 @0x00003f5c push_number(S) [0] 
0150 @0x00003f60 query_member [0] 
0151 @0x00003f64 push_local_value [0] 
0152 @0x00003f68 call_fun_ptr [4, 5] 
0153 @0x00003f6c pop_stack [] 
0154 @0x00003f70 goto [2] 
0155 @0x00003f74 null_action [] 
0156 @0x00003f78 null_action [] 
0157 @0x00003f7c goto [126] 
0158 @0x00003f80 null_action [] 
0159 @0x00003f84 push_local_value [1] 
0160 @0x00003f88 push_number(S) [0] 
0161 @0x00003f8c query_member [0] 
0162 @0x00003f90 push_local_value [1] 
0163 @0x00003f94 push_number(S) [3] 
0164 @0x00003f98 query_member [0] 
0165 @0x00003f9c push_local_value [1] 
0166 @0x00003fa0 push_number(S) [2] 
0167 @0x00003fa4 query_member [0] 
0168 @0x00003fa8 push_local_value [1] 
0169 @0x00003fac push_number(S) [1] 
0170 @0x00003fb0 query_member [0] 
0171 @0x00003fb4 push_this_object [] 
0172 @0x00003fb8 push_shared_string [11] 'open_auth_all_protect'
0173 @0x00003fbc push_shared_string [41] '/gs/daemons/auth_protectd.c'
0174 @0x00003fc0 call_in_other_fun_ex [4, 5] 
0175 @0x00003fc4 pop_stack [] 
0176 @0x00003fc8 goto [107] 
0177 @0x00003fcc null_action [] 
0178 @0x00003fd0 push_local_value [1] 
0179 @0x00003fd4 push_number(S) [1] 
0180 @0x00003fd8 query_member [0] 
0181 @0x00003fdc push_local_value [1] 
0182 @0x00003fe0 push_number(S) [0] 
0183 @0x00003fe4 query_member [0] 
0184 @0x00003fe8 push_this_object [] 
0185 @0x00003fec push_shared_string [12] 'do_cancel_trading'
0186 @0x00003ff0 push_shared_string [42] '/gs/daemons/treasure_tradingd.c'
0187 @0x00003ff4 call_in_other_fun_ex [4, 3] 
0188 @0x00003ff8 pop_stack [] 
0189 @0x00003ffc goto [94] 
0190 @0x00004000 null_action [] 
0191 @0x00004004 push_local_value [1] 
0192 @0x00004008 push_number(S) [2] 
0193 @0x0000400c query_member [0] 
0194 @0x00004010 push_local_value [1] 
0195 @0x00004014 push_number(S) [1] 
0196 @0x00004018 query_member [0] 
0197 @0x0000401c push_local_value [1] 
0198 @0x00004020 push_number(S) [0] 
0199 @0x00004024 query_member [0] 
0200 @0x00004028 push_this_object [] 
0201 @0x0000402c push_shared_string [13] 'change_price_role'
0202 @0x00004030 push_shared_string [42] '/gs/daemons/treasure_tradingd.c'
0203 @0x00004034 call_in_other_fun_ex [4, 4] 
0204 @0x00004038 pop_stack [] 
0205 @0x0000403c goto [78] 
0206 @0x00004040 null_action [] 
0207 @0x00004044 push_local_value [1] 
0208 @0x00004048 push_number(S) [7] 
0209 @0x0000404c query_member [0] 
0210 @0x00004050 push_local_value [1] 
0211 @0x00004054 push_number(S) [6] 
0212 @0x00004058 query_member [0] 
0213 @0x0000405c push_local_value [1] 
0214 @0x00004060 push_number(S) [5] 
0215 @0x00004064 query_member [0] 
0216 @0x00004068 push_local_value [1] 
0217 @0x0000406c push_number(S) [4] 
0218 @0x00004070 query_member [0] 
0219 @0x00004074 push_local_value [1] 
0220 @0x00004078 push_number(S) [3] 
0221 @0x0000407c query_member [0] 
0222 @0x00004080 push_local_value [1] 
0223 @0x00004084 push_number(S) [2] 
0224 @0x00004088 query_member [0] 
0225 @0x0000408c push_local_value [1] 
0226 @0x00004090 push_number(S) [1] 
0227 @0x00004094 query_member [0] 
0228 @0x00004098 push_local_value [1] 
0229 @0x0000409c push_number(S) [0] 
0230 @0x000040a0 query_member [0] 
0231 @0x000040a4 push_this_object [] 
0232 @0x000040a8 push_shared_string [14] 'continue_sell_role'
0233 @0x000040ac push_shared_string [42] '/gs/daemons/treasure_tradingd.c'
0234 @0x000040b0 call_in_other_fun_ex [4, 9] 
0235 @0x000040b4 pop_stack [] 
0236 @0x000040b8 goto [47] 
0237 @0x000040bc null_action [] 
0238 @0x000040c0 push_local_value [1] 
0239 @0x000040c4 push_number(S) [1] 
0240 @0x000040c8 query_member [0] 
0241 @0x000040cc push_local_value [1] 
0242 @0x000040d0 push_number(S) [0] 
0243 @0x000040d4 query_member [0] 
0244 @0x000040d8 push_this_object [] 
0245 @0x000040dc push_shared_string [15] 'buy_service'
0246 @0x000040e0 push_shared_string [43] '/gs/daemons/service_malld.c'
0247 @0x000040e4 call_in_other_fun_ex [4, 3] 
0248 @0x000040e8 pop_stack [] 
0249 @0x000040ec goto [34] 
0250 @0x000040f0 null_action [] 
0251 @0x000040f4 push_local_value [1] 
0252 @0x000040f8 push_number(S) [0] 
0253 @0x000040fc query_member [0] 
0254 @0x00004100 push_this_object [] 
0255 @0x00004104 push_shared_string [16] 'check_password'
0256 @0x00004108 push_shared_string [44] '/gs/daemons/itemlockd.c'
0257 @0x0000410c call_in_other_fun_ex [4, 2] 
0258 @0x00004110 pop_stack [] 
0259 @0x00004114 goto [24] 
0260 @0x00004118 null_action [] 
0261 @0x0000411c goto [22] 
0262 @0x00004120 null_action [] 
0263 @0x00004124 push_local_value [1] 
0264 @0x00004128 push_number(S) [0] 
0265 @0x0000412c query_member [0] 
0266 @0x00004130 push_this_object [] 
0267 @0x00004134 push_shared_string [17] 'request_server_status'
0268 @0x00004138 push_shared_string [45] '/gs/daemons/switch_serverd.c'
0269 @0x0000413c call_in_other_fun_ex [4, 2] 
0270 @0x00004140 pop_stack [] 
0271 @0x00004144 goto [12] 
0272 @0x00004148 null_action [] 
0273 @0x0000414c push_local_value [1] 
0274 @0x00004150 push_number(S) [0] 
0275 @0x00004154 query_member [0] 
0276 @0x00004158 push_this_object [] 
0277 @0x0000415c push_shared_string [18] 'login_ob_switch_server'
0278 @0x00004160 push_shared_string [45] '/gs/daemons/switch_serverd.c'
0279 @0x00004164 call_in_other_fun_ex [4, 2] 
0280 @0x00004168 pop_stack [] 
0281 @0x0000416c goto [2] 
0282 @0x00004170 null_action [] 
0283 @0x00004174 null_action [] 
0284 @0x00004178 push_number(S) [0] 
0285 @0x0000417c call_in_user_fun [213, 1] set_operating
0286 @0x00004180 pop_stack [] 
0287 @0x00004184 null_action [] 
0288 @0x00004188 return_from_fun [] 
```

<a id="gs-gs-clone-char-login-o-process_command"></a>

## gs/pack_data/lib_gs32.pak/gs/clone/char/login.o :: process_command

成员 SHA-256：`0e5aa235e3033a52424ee0210e1dbd1297f0b109d0bba6eee8675c7cfd628650`；函数记录偏移：`0x4856`；函数编号：`221`。

```text
0000 @0x00004955 check_args_num [0] 
0001 @0x00004959 enter_function [] 
0002 @0x0000495d call_in_user_fun [214, 0] get_operating
0003 @0x00004961 if [5, 2] 
0004 @0x00004965 push_shared_string [5] '请求处理中，请稍候！（原因：9101）'
0005 @0x00004969 call_in_user_fun [22, 1] unresolved/inherited/builtin
0006 @0x0000496d pop_stack [] 
0007 @0x00004971 return_from_fun [] 
0008 @0x00004975 null_action [] 
0009 @0x00004979 push_number(S) [1] 
0010 @0x0000497d call_in_user_fun [213, 1] set_operating
0011 @0x00004981 pop_stack [] 
0012 @0x00004985 call_in_efun [136, 0] tick
0013 @0x00004989 push_shared_string [6] 'recv_cmd_ti'
0014 @0x0000498d call_in_user_fun [98, 2] unresolved/inherited/builtin
0015 @0x00004991 pop_stack [] 
0016 @0x00004995 push_global_var [340] 
0017 @0x00004999 push_local_value [0] 
0018 @0x0000499d query_member [0] 
0019 @0x000049a1 if [15, 2] 
0020 @0x000049a5 null_action [] 
0021 @0x000049a9 null_action [] 
0022 @0x000049ad push_global_vptr [339] 
0023 @0x000049b1 inc_value [1] 
0024 @0x000049b5 null_action [] 
0025 @0x000049b9 push_local_value [1] 
0026 @0x000049bd push_local_value [0] 
0027 @0x000049c1 create_function_value [218, 2] 
0028 @0x000049c5 call_in_user_fun [8, 0] unresolved/inherited/builtin
0029 @0x000049c9 push_shared_string [4] 'dispatch'
0030 @0x000049cd push_shared_string [7] '/daemons/threadd.c'
0031 @0x000049d1 call_in_other_fun_ex [4, 2] 
0032 @0x000049d5 pop_stack [] 
0033 @0x000049d9 null_action [] 
0034 @0x000049dd goto [8] 
0035 @0x000049e1 push_local_value [1] 
0036 @0x000049e5 push_local_value [0] 
0037 @0x000049e9 create_function_value [215, 2] 
0038 @0x000049ed call_in_user_fun [8, 0] unresolved/inherited/builtin
0039 @0x000049f1 push_shared_string [4] 'dispatch'
0040 @0x000049f5 push_shared_string [7] '/daemons/threadd.c'
0041 @0x000049f9 call_in_other_fun_ex [4, 2] 
0042 @0x000049fd pop_stack [] 
0043 @0x00004a01 null_action [] 
0044 @0x00004a05 return_from_fun [] 
```

<a id="gs-gs-daemons-logind-o-do_login"></a>

## gs/pack_data/lib_gs32.pak/gs/daemons/logind.o :: do_login

成员 SHA-256：`51a071b4bf5411a7fdf5bddf6ae8b77a33d114ac0f530bd3822960dbb3523f79`；函数记录偏移：`0xcdb7`；函数编号：`79`。

```text
0000 @0x0000d685 check_args_num [0] 
0001 @0x0000d689 enter_function [] 
0002 @0x0000d68d push_shared_string [44] 'get_client'
0003 @0x0000d691 push_local_value [0] 
0004 @0x0000d695 call_in_other_fun_ex [4, 0] 
0005 @0x0000d699 call_in_efun [515, 1] get_client_info
0006 @0x0000d69d push_local_vptr [-9] 
0007 @0x0000d6a1 assign_value [] 
0008 @0x0000d6a5 pop_stack [] 
0009 @0x0000d6a9 push_obj_var [2] 
0010 @0x0000d6ad push_local_value [1] 
0011 @0x0000d6b1 push_shared_string [80] 'auth_key'
0012 @0x0000d6b5 query_member [0] 
0013 @0x0000d6b9 query_member [0] 
0014 @0x0000d6bd push_local_vptr [-8] 
0015 @0x0000d6c1 assign_value [] 
0016 @0x0000d6c5 is_type [7] 
0017 @0x0000d6c9 not_value [] 
0018 @0x0000d6cd if [6, 2] 
0019 @0x0000d6d1 push_local_value [1] 
0020 @0x0000d6d5 push_local_value [0] 
0021 @0x0000d6d9 call_in_user_fun [74, 2] try_quick_login
0022 @0x0000d6dd pop_stack [] 
0023 @0x0000d6e1 return_from_fun [] 
0024 @0x0000d6e5 null_action [] 
0025 @0x0000d6e9 push_shared_string [81] 'login_state'
0026 @0x0000d6ed push_local_value [-8] 
0027 @0x0000d6f1 call_in_efun [80, 2] map_delete
0028 @0x0000d6f5 pop_stack [] 
0029 @0x0000d6f9 push_number(S) [2] 
0030 @0x0000d6fd push_local_value [0] 
0031 @0x0000d701 call_in_user_fun [70, 2] set_login_state
0032 @0x0000d705 pop_stack [] 
0033 @0x0000d709 push_local_value [1] 
0034 @0x0000d70d push_shared_string [26] 'account'
0035 @0x0000d711 query_member [0] 
0036 @0x0000d715 is_type [4] 
0037 @0x0000d719 if [8, 1] 
0038 @0x0000d71d pop_stack [] 
0039 @0x0000d721 push_shared_string [27] 'login_info/account'
0040 @0x0000d725 push_shared_string [24] 'query_temp'
0041 @0x0000d729 push_local_value [1] 
0042 @0x0000d72d push_shared_string [26] 'account'
0043 @0x0000d731 query_member [0] 
0044 @0x0000d735 call_in_other_fun_ex [4, 1] 
0045 @0x0000d739 is_type [4] 
0046 @0x0000d73d if [8, 2] 
0047 @0x0000d741 push_local_value [1] 
0048 @0x0000d745 push_shared_string [26] 'account'
0049 @0x0000d749 query_member [0] 
0050 @0x0000d74d call_in_user_fun [698, -127] get_login_info_file
0051 @0x0000d751 push_shared_string [63] 'assure_log_file'
0052 @0x0000d755 push_shared_string [30] '/daemons/logd.c'
0053 @0x0000d759 call_in_other_fun_ex [4, 1] 
0054 @0x0000d75d pop_stack [] 
0055 @0x0000d761 push_local_value [1] 
0056 @0x0000d765 push_shared_string [26] 'account'
0057 @0x0000d769 query_member [0] 
0058 @0x0000d76d is_type [4] 
0059 @0x0000d771 if [8, 1] 
0060 @0x0000d775 pop_stack [] 
0061 @0x0000d779 push_shared_string [27] 'login_info/account'
0062 @0x0000d77d push_shared_string [24] 'query_temp'
0063 @0x0000d781 push_local_value [1] 
0064 @0x0000d785 push_shared_string [26] 'account'
0065 @0x0000d789 query_member [0] 
0066 @0x0000d78d call_in_other_fun_ex [4, 1] 
0067 @0x0000d791 is_type [4] 
0068 @0x0000d795 if [18, 2] 
0069 @0x0000d799 call_in_efun [136, 0] tick
0070 @0x0000d79d push_shared_string [82] 'recv_cmd_ti'
0071 @0x0000d7a1 push_shared_string [64] 'lite_query_temp'
0072 @0x0000d7a5 push_local_value [0] 
0073 @0x0000d7a9 call_in_other_fun_ex [4, 1] 
0074 @0x0000d7ad cast_type [2, 0] 
0075 @0x0000d7b1 sub_value [] 
0076 @0x0000d7b5 push_shared_string [83] '线程等待tick:%d\n'
0077 @0x0000d7b9 call_in_efun [121, 2] sprintf
0078 @0x0000d7bd push_shared_string [29] 'login'
0079 @0x0000d7c1 push_local_value [1] 
0080 @0x0000d7c5 push_shared_string [26] 'account'
0081 @0x0000d7c9 query_member [0] 
0082 @0x0000d7cd call_in_user_fun [698, -127] get_login_info_file
0083 @0x0000d7d1 push_shared_string [25] 'log_login_info'
0084 @0x0000d7d5 push_shared_string [30] '/daemons/logd.c'
0085 @0x0000d7d9 call_in_other_fun_ex [4, 3] 
0086 @0x0000d7dd pop_stack [] 
0087 @0x0000d7e1 push_local_value [1] 
0088 @0x0000d7e5 push_shared_string [26] 'account'
0089 @0x0000d7e9 query_member [0] 
0090 @0x0000d7ed is_type [4] 
0091 @0x0000d7f1 if [8, 1] 
0092 @0x0000d7f5 pop_stack [] 
0093 @0x0000d7f9 push_shared_string [27] 'login_info/account'
0094 @0x0000d7fd push_shared_string [24] 'query_temp'
0095 @0x0000d801 push_local_value [1] 
0096 @0x0000d805 push_shared_string [26] 'account'
0097 @0x0000d809 query_member [0] 
0098 @0x0000d80d call_in_other_fun_ex [4, 1] 
0099 @0x0000d811 is_type [4] 
0100 @0x0000d815 if [10, 2] 
0101 @0x0000d819 push_shared_string [84] '--------------------正常流程登录-----------------------------\n'
0102 @0x0000d81d push_shared_string [29] 'login'
0103 @0x0000d821 push_local_value [1] 
0104 @0x0000d825 push_shared_string [26] 'account'
0105 @0x0000d829 query_member [0] 
0106 @0x0000d82d call_in_user_fun [698, -127] get_login_info_file
0107 @0x0000d831 push_shared_string [25] 'log_login_info'
0108 @0x0000d835 push_shared_string [30] '/daemons/logd.c'
0109 @0x0000d839 call_in_other_fun_ex [4, 3] 
0110 @0x0000d83d pop_stack [] 
0111 @0x0000d841 push_local_value [1] 
0112 @0x0000d845 push_shared_string [26] 'account'
0113 @0x0000d849 query_member [0] 
0114 @0x0000d84d is_type [4] 
0115 @0x0000d851 if [8, 1] 
0116 @0x0000d855 pop_stack [] 
0117 @0x0000d859 push_shared_string [27] 'login_info/account'
0118 @0x0000d85d push_shared_string [24] 'query_temp'
0119 @0x0000d861 push_local_value [1] 
0120 @0x0000d865 push_shared_string [26] 'account'
0121 @0x0000d869 query_member [0] 
0122 @0x0000d86d call_in_other_fun_ex [4, 1] 
0123 @0x0000d871 is_type [4] 
0124 @0x0000d875 if [22, 2] 
0125 @0x0000d879 push_local_value [-9] 
0126 @0x0000d87d push_shared_string [85] 'remote_port'
0127 @0x0000d881 query_member [0] 
0128 @0x0000d885 cast_type [2, 0] 
0129 @0x0000d889 push_local_value [-9] 
0130 @0x0000d88d push_shared_string [86] 'remote_address'
0131 @0x0000d891 query_member [0] 
0132 @0x0000d895 cast_type [4, 0] 
0133 @0x0000d899 push_local_value [1] 
0134 @0x0000d89d push_shared_string [26] 'account'
0135 @0x0000d8a1 query_member [0] 
0136 @0x0000d8a5 push_shared_string [87] '帐号 %s(%s:%d) 正在试图登录.\n'
0137 @0x0000d8a9 call_in_efun [121, 4] sprintf
0138 @0x0000d8ad push_shared_string [29] 'login'
0139 @0x0000d8b1 push_local_value [1] 
0140 @0x0000d8b5 push_shared_string [26] 'account'
0141 @0x0000d8b9 query_member [0] 
0142 @0x0000d8bd call_in_user_fun [698, -127] get_login_info_file
0143 @0x0000d8c1 push_shared_string [25] 'log_login_info'
0144 @0x0000d8c5 push_shared_string [30] '/daemons/logd.c'
0145 @0x0000d8c9 call_in_other_fun_ex [4, 3] 
0146 @0x0000d8cd pop_stack [] 
0147 @0x0000d8d1 push_local_value [1] 
0148 @0x0000d8d5 push_shared_string [26] 'account'
0149 @0x0000d8d9 query_member [0] 
0150 @0x0000d8dd is_type [4] 
0151 @0x0000d8e1 if [8, 1] 
0152 @0x0000d8e5 pop_stack [] 
0153 @0x0000d8e9 push_shared_string [27] 'login_info/account'
0154 @0x0000d8ed push_shared_string [24] 'query_temp'
0155 @0x0000d8f1 push_local_value [1] 
0156 @0x0000d8f5 push_shared_string [26] 'account'
0157 @0x0000d8f9 query_member [0] 
0158 @0x0000d8fd call_in_other_fun_ex [4, 1] 
0159 @0x0000d901 is_type [4] 
0160 @0x0000d905 if [10, 2] 
0161 @0x0000d909 push_shared_string [88] '验证登录信息\n'
0162 @0x0000d90d push_shared_string [29] 'login'
0163 @0x0000d911 push_local_value [1] 
0164 @0x0000d915 push_shared_string [26] 'account'
0165 @0x0000d919 query_member [0] 
0166 @0x0000d91d call_in_user_fun [698, -127] get_login_info_file
0167 @0x0000d921 push_shared_string [25] 'log_login_info'
0168 @0x0000d925 push_shared_string [30] '/daemons/logd.c'
0169 @0x0000d929 call_in_other_fun_ex [4, 3] 
0170 @0x0000d92d pop_stack [] 
0171 @0x0000d931 push_local_value [1] 
0172 @0x0000d935 push_shared_string [26] 'account'
0173 @0x0000d939 query_member [0] 
0174 @0x0000d93d push_local_value [-8] 
0175 @0x0000d941 push_shared_string [26] 'account'
0176 @0x0000d945 query_member [0] 
0177 @0x0000d949 call_in_efun [125, 2] stricmp
0178 @0x0000d94d if [8, 1] 
0179 @0x0000d951 pop_stack [] 
0180 @0x0000d955 push_local_value [-8] 
0181 @0x0000d959 push_shared_string [89] 'seed'
0182 @0x0000d95d query_member [0] 
0183 @0x0000d961 push_local_value [1] 
0184 @0x0000d965 push_shared_string [89] 'seed'
0185 @0x0000d969 query_member [0] 
0186 @0x0000d96d ne_value [] 
0187 @0x0000d971 if [26, 2] 
0188 @0x0000d975 push_local_value [1] 
0189 @0x0000d979 push_shared_string [26] 'account'
0190 @0x0000d97d query_member [0] 
0191 @0x0000d981 is_type [4] 
0192 @0x0000d985 if [8, 1] 
0193 @0x0000d989 pop_stack [] 
0194 @0x0000d98d push_shared_string [27] 'login_info/account'
0195 @0x0000d991 push_shared_string [24] 'query_temp'
0196 @0x0000d995 push_local_value [1] 
0197 @0x0000d999 push_shared_string [26] 'account'
0198 @0x0000d99d query_member [0] 
0199 @0x0000d9a1 call_in_other_fun_ex [4, 1] 
0200 @0x0000d9a5 is_type [4] 
0201 @0x0000d9a9 if [10, 2] 
0202 @0x0000d9ad push_shared_string [90] '验证失败，LOGIN_D的login_info_map没有记录该帐号的对应登录信息或者对应的登录信息不匹配\n'
0203 @0x0000d9b1 push_shared_string [91] 'error'
0204 @0x0000d9b5 push_local_value [1] 
0205 @0x0000d9b9 push_shared_string [26] 'account'
0206 @0x0000d9bd query_member [0] 
0207 @0x0000d9c1 call_in_user_fun [698, -127] get_login_info_file
0208 @0x0000d9c5 push_shared_string [25] 'log_login_info'
0209 @0x0000d9c9 push_shared_string [30] '/daemons/logd.c'
0210 @0x0000d9cd call_in_other_fun_ex [4, 3] 
0211 @0x0000d9d1 pop_stack [] 
0212 @0x0000d9d5 return_from_fun [] 
0213 @0x0000d9d9 null_action [] 
0214 @0x0000d9dd push_local_value [-8] 
0215 @0x0000d9e1 push_shared_string [26] 'account'
0216 @0x0000d9e5 query_member [0] 
0217 @0x0000d9e9 push_shared_string [26] 'account'
0218 @0x0000d9ed push_shared_string [65] 'lite_set_temp'
0219 @0x0000d9f1 push_local_value [0] 
0220 @0x0000d9f5 call_in_other_fun_ex [4, 2] 
0221 @0x0000d9f9 pop_stack [] 
0222 @0x0000d9fd push_obj_var [2] 
0223 @0x0000da01 push_local_value [1] 
0224 @0x0000da05 push_shared_string [80] 'auth_key'
0225 @0x0000da09 query_member [0] 
0226 @0x0000da0d query_member [0] 
0227 @0x0000da11 call_in_efun [102, 1] save_string
0228 @0x0000da15 push_local_vptr [-8] 
0229 @0x0000da19 push_shared_string [92] 'para'
0230 @0x0000da1d query_member_ptr [0] 
0231 @0x0000da21 assign_value [] 
0232 @0x0000da25 pop_stack [] 
0233 @0x0000da29 push_local_value [1] 
0234 @0x0000da2d push_shared_string [26] 'account'
0235 @0x0000da31 query_member [0] 
0236 @0x0000da35 is_type [4] 
0237 @0x0000da39 if [8, 1] 
0238 @0x0000da3d pop_stack [] 
0239 @0x0000da41 push_shared_string [27] 'login_info/account'
0240 @0x0000da45 push_shared_string [24] 'query_temp'
0241 @0x0000da49 push_local_value [1] 
0242 @0x0000da4d push_shared_string [26] 'account'
0243 @0x0000da51 query_member [0] 
0244 @0x0000da55 call_in_other_fun_ex [4, 1] 
0245 @0x0000da59 is_type [4] 
0246 @0x0000da5d if [10, 2] 
0247 @0x0000da61 push_shared_string [93] '验证成功，从LOGIN_D中移除对应的登录信息\n'
0248 @0x0000da65 push_shared_string [29] 'login'
0249 @0x0000da69 push_local_value [1] 
0250 @0x0000da6d push_shared_string [26] 'account'
0251 @0x0000da71 query_member [0] 
0252 @0x0000da75 call_in_user_fun [698, -127] get_login_info_file
0253 @0x0000da79 push_shared_string [25] 'log_login_info'
0254 @0x0000da7d push_shared_string [30] '/daemons/logd.c'
0255 @0x0000da81 call_in_other_fun_ex [4, 3] 
0256 @0x0000da85 pop_stack [] 
0257 @0x0000da89 push_local_value [1] 
0258 @0x0000da8d push_shared_string [80] 'auth_key'
0259 @0x0000da91 query_member [0] 
0260 @0x0000da95 call_in_user_fun [28, 1] clear_login_info
0261 @0x0000da99 pop_stack [] 
0262 @0x0000da9d call_in_user_fun [39, 0] is_login_permitted
0263 @0x0000daa1 not_value [] 
0264 @0x0000daa5 if [6, 0] 
0265 @0x0000daa9 pop_stack [] 
0266 @0x0000daad push_local_value [-8] 
0267 @0x0000dab1 push_shared_string [50] 'privilege'
0268 @0x0000dab5 query_member [0] 
0269 @0x0000dab9 push_number(S) [0] 
0270 @0x0000dabd eq_value [] 
0271 @0x0000dac1 if [7, 0] 
0272 @0x0000dac5 pop_stack [] 
0273 @0x0000dac9 push_local_value [-8] 
0274 @0x0000dacd push_shared_string [94] 'service'
0275 @0x0000dad1 query_member [0] 
0276 @0x0000dad5 push_number(S) [256] 
0277 @0x0000dad9 and [] 
0278 @0x0000dadd not_value [] 
0279 @0x0000dae1 if [34, 2] 
0280 @0x0000dae5 push_local_value [1] 
0281 @0x0000dae9 push_shared_string [26] 'account'
0282 @0x0000daed query_member [0] 
0283 @0x0000daf1 is_type [4] 
0284 @0x0000daf5 if [8, 1] 
0285 @0x0000daf9 pop_stack [] 
0286 @0x0000dafd push_shared_string [27] 'login_info/account'
0287 @0x0000db01 push_shared_string [24] 'query_temp'
0288 @0x0000db05 push_local_value [1] 
0289 @0x0000db09 push_shared_string [26] 'account'
0290 @0x0000db0d query_member [0] 
0291 @0x0000db11 call_in_other_fun_ex [4, 1] 
0292 @0x0000db15 is_type [4] 
0293 @0x0000db19 if [10, 2] 
0294 @0x0000db1d push_shared_string [95] '服务器目前禁止登录，登录失败\n'
0295 @0x0000db21 push_shared_string [29] 'login'
0296 @0x0000db25 push_local_value [1] 
0297 @0x0000db29 push_shared_string [26] 'account'
0298 @0x0000db2d query_member [0] 
0299 @0x0000db31 call_in_user_fun [698, -127] get_login_info_file
0300 @0x0000db35 push_shared_string [25] 'log_login_info'
0301 @0x0000db39 push_shared_string [30] '/daemons/logd.c'
0302 @0x0000db3d call_in_other_fun_ex [4, 3] 
0303 @0x0000db41 pop_stack [] 
0304 @0x0000db45 push_shared_string [96] '服务器目前禁止登录，请稍候再尝试。（错误：9037）'
0305 @0x0000db49 push_shared_string [45] 'send_dialog_ok'
0306 @0x0000db4d push_local_value [0] 
0307 @0x0000db51 call_in_other_fun_ex [4, 1] 
0308 @0x0000db55 pop_stack [] 
0309 @0x0000db59 push_local_value [0] 
0310 @0x0000db5d call_in_efun [361, 1] destruct_object
0311 @0x0000db61 pop_stack [] 
0312 @0x0000db65 return_from_fun [] 
0313 @0x0000db69 null_action [] 
0314 @0x0000db6d push_obj_var [3] 
0315 @0x0000db71 push_local_value [1] 
0316 @0x0000db75 push_shared_string [26] 'account'
0317 @0x0000db79 query_member [0] 
0318 @0x0000db7d query_member [0] 
0319 @0x0000db81 if [34, 2] 
0320 @0x0000db85 push_local_value [1] 
0321 @0x0000db89 push_shared_string [26] 'account'
0322 @0x0000db8d query_member [0] 
0323 @0x0000db91 is_type [4] 
0324 @0x0000db95 if [8, 1] 
0325 @0x0000db99 pop_stack [] 
0326 @0x0000db9d push_shared_string [27] 'login_info/account'
0327 @0x0000dba1 push_shared_string [24] 'query_temp'
0328 @0x0000dba5 push_local_value [1] 
0329 @0x0000dba9 push_shared_string [26] 'account'
0330 @0x0000dbad query_member [0] 
0331 @0x0000dbb1 call_in_other_fun_ex [4, 1] 
0332 @0x0000dbb5 is_type [4] 
0333 @0x0000dbb9 if [10, 2] 
0334 @0x0000dbbd push_shared_string [97] '帐号在进行敏感操作，不允许登录\n'
0335 @0x0000dbc1 push_shared_string [29] 'login'
0336 @0x0000dbc5 push_local_value [1] 
0337 @0x0000dbc9 push_shared_string [26] 'account'
0338 @0x0000dbcd query_member [0] 
0339 @0x0000dbd1 call_in_user_fun [698, -127] get_login_info_file
0340 @0x0000dbd5 push_shared_string [25] 'log_login_info'
0341 @0x0000dbd9 push_shared_string [30] '/daemons/logd.c'
0342 @0x0000dbdd call_in_other_fun_ex [4, 3] 
0343 @0x0000dbe1 pop_stack [] 
0344 @0x0000dbe5 push_shared_string [98] '你的帐号现在暂时无法登录，请稍候再尝试。（错误：9026）'
0345 @0x0000dbe9 push_shared_string [45] 'send_dialog_ok'
0346 @0x0000dbed push_local_value [0] 
0347 @0x0000dbf1 call_in_other_fun_ex [4, 1] 
0348 @0x0000dbf5 pop_stack [] 
0349 @0x0000dbf9 push_local_value [0] 
0350 @0x0000dbfd call_in_efun [361, 1] destruct_object
0351 @0x0000dc01 pop_stack [] 
0352 @0x0000dc05 return_from_fun [] 
0353 @0x0000dc09 null_action [] 
0354 @0x0000dc0d push_local_value [1] 
0355 @0x0000dc11 push_shared_string [99] 'mobile_key'
0356 @0x0000dc15 query_member [0] 
0357 @0x0000dc19 cast_type [4, 0] 
0358 @0x0000dc1d call_in_efun [122, 1] strlen
0359 @0x0000dc21 if [195, 2] 
0360 @0x0000dc25 push_local_value [1] 
0361 @0x0000dc29 push_shared_string [26] 'account'
0362 @0x0000dc2d query_member [0] 
0363 @0x0000dc31 is_type [4] 
0364 @0x0000dc35 if [8, 1] 
0365 @0x0000dc39 pop_stack [] 
0366 @0x0000dc3d push_shared_string [27] 'login_info/account'
0367 @0x0000dc41 push_shared_string [24] 'query_temp'
0368 @0x0000dc45 push_local_value [1] 
0369 @0x0000dc49 push_shared_string [26] 'account'
0370 @0x0000dc4d query_member [0] 
0371 @0x0000dc51 call_in_other_fun_ex [4, 1] 
0372 @0x0000dc55 is_type [4] 
0373 @0x0000dc59 if [10, 2] 
0374 @0x0000dc5d push_shared_string [100] '尝试使用手机客户端登录\n'
0375 @0x0000dc61 push_shared_string [29] 'login'
0376 @0x0000dc65 push_local_value [1] 
0377 @0x0000dc69 push_shared_string [26] 'account'
0378 @0x0000dc6d query_member [0] 
0379 @0x0000dc71 call_in_user_fun [698, -127] get_login_info_file
0380 @0x0000dc75 push_shared_string [25] 'log_login_info'
0381 @0x0000dc79 push_shared_string [30] '/daemons/logd.c'
0382 @0x0000dc7d call_in_other_fun_ex [4, 3] 
0383 @0x0000dc81 pop_stack [] 
0384 @0x0000dc85 push_obj_var [16] 
0385 @0x0000dc89 not_value [] 
0386 @0x0000dc8d if [29, 2] 
0387 @0x0000dc91 push_local_value [1] 
0388 @0x0000dc95 push_shared_string [26] 'account'
0389 @0x0000dc99 query_member [0] 
0390 @0x0000dc9d is_type [4] 
0391 @0x0000dca1 if [8, 1] 
0392 @0x0000dca5 pop_stack [] 
0393 @0x0000dca9 push_shared_string [27] 'login_info/account'
0394 @0x0000dcad push_shared_string [24] 'query_temp'
0395 @0x0000dcb1 push_local_value [1] 
0396 @0x0000dcb5 push_shared_string [26] 'account'
0397 @0x0000dcb9 query_member [0] 
0398 @0x0000dcbd call_in_other_fun_ex [4, 1] 
0399 @0x0000dcc1 is_type [4] 
0400 @0x0000dcc5 if [10, 2] 
0401 @0x0000dcc9 push_shared_string [101] '服务器禁止手机客户端登录，断开连接\n'
0402 @0x0000dccd push_shared_string [29] 'login'
0403 @0x0000dcd1 push_local_value [1] 
0404 @0x0000dcd5 push_shared_string [26] 'account'
0405 @0x0000dcd9 query_member [0] 
0406 @0x0000dcdd call_in_user_fun [698, -127] get_login_info_file
0407 @0x0000dce1 push_shared_string [25] 'log_login_info'
0408 @0x0000dce5 push_shared_string [30] '/daemons/logd.c'
0409 @0x0000dce9 call_in_other_fun_ex [4, 3] 
0410 @0x0000dced pop_stack [] 
0411 @0x0000dcf1 push_local_value [0] 
0412 @0x0000dcf5 call_in_efun [361, 1] destruct_object
0413 @0x0000dcf9 pop_stack [] 
0414 @0x0000dcfd return_from_fun [] 
0415 @0x0000dd01 null_action [] 
0416 @0x0000dd05 push_local_value [1] 
0417 @0x0000dd09 push_shared_string [26] 'account'
0418 @0x0000dd0d query_member [0] 
0419 @0x0000dd11 push_shared_string [66] 'get_gid_in_cs'
0420 @0x0000dd15 push_shared_string [102] '/gs/daemons/cross_serverd.c'
0421 @0x0000dd19 call_in_other_fun_ex [4, 1] 
0422 @0x0000dd1d is_type [4] 
0423 @0x0000dd21 if [34, 2] 
0424 @0x0000dd25 push_local_value [1] 
0425 @0x0000dd29 push_shared_string [26] 'account'
0426 @0x0000dd2d query_member [0] 
0427 @0x0000dd31 is_type [4] 
0428 @0x0000dd35 if [8, 1] 
0429 @0x0000dd39 pop_stack [] 
0430 @0x0000dd3d push_shared_string [27] 'login_info/account'
0431 @0x0000dd41 push_shared_string [24] 'query_temp'
0432 @0x0000dd45 push_local_value [1] 
0433 @0x0000dd49 push_shared_string [26] 'account'
0434 @0x0000dd4d query_member [0] 
0435 @0x0000dd51 call_in_other_fun_ex [4, 1] 
0436 @0x0000dd55 is_type [4] 
0437 @0x0000dd59 if [10, 2] 
0438 @0x0000dd5d push_shared_string [103] '帐号已在跨服区组登录，不允许手机登录，断开连接\n'
0439 @0x0000dd61 push_shared_string [29] 'login'
0440 @0x0000dd65 push_local_value [1] 
0441 @0x0000dd69 push_shared_string [26] 'account'
0442 @0x0000dd6d query_member [0] 
0443 @0x0000dd71 call_in_user_fun [698, -127] get_login_info_file
0444 @0x0000dd75 push_shared_string [25] 'log_login_info'
0445 @0x0000dd79 push_shared_string [30] '/daemons/logd.c'
0446 @0x0000dd7d call_in_other_fun_ex [4, 3] 
0447 @0x0000dd81 pop_stack [] 
0448 @0x0000dd85 push_shared_string [104] '当前帐号正处于登录状态。'
0449 @0x0000dd89 push_shared_string [45] 'send_dialog_ok'
0450 @0x0000dd8d push_local_value [0] 
0451 @0x0000dd91 call_in_other_fun_ex [4, 1] 
0452 @0x0000dd95 pop_stack [] 
0453 @0x0000dd99 push_local_value [0] 
0454 @0x0000dd9d call_in_efun [361, 1] destruct_object
0455 @0x0000dda1 pop_stack [] 
0456 @0x0000dda5 return_from_fun [] 
0457 @0x0000dda9 null_action [] 
0458 @0x0000ddad push_shared_string [105] 'gbits'
0459 @0x0000ddb1 push_local_value [1] 
0460 @0x0000ddb5 push_shared_string [99] 'mobile_key'
0461 @0x0000ddb9 query_member [0] 
0462 @0x0000ddbd cast_type [4, 0] 
0463 @0x0000ddc1 push_shared_string [67] 'decrypt'
0464 @0x0000ddc5 push_shared_string [106] '/daemons/encryptd.c'
0465 @0x0000ddc9 call_in_other_fun_ex [4, 2] 
0466 @0x0000ddcd push_local_vptr [-6] 
0467 @0x0000ddd1 assign_value [] 
0468 @0x0000ddd5 pop_stack [] 
0469 @0x0000ddd9 push_local_value [-6] 
0470 @0x0000dddd push_local_value [1] 
0471 @0x0000dde1 push_shared_string [26] 'account'
0472 @0x0000dde5 query_member [0] 
0473 @0x0000dde9 ne_value [] 
0474 @0x0000dded if [29, 2] 
0475 @0x0000ddf1 push_local_value [1] 
0476 @0x0000ddf5 push_shared_string [26] 'account'
0477 @0x0000ddf9 query_member [0] 
0478 @0x0000ddfd is_type [4] 
0479 @0x0000de01 if [8, 1] 
0480 @0x0000de05 pop_stack [] 
0481 @0x0000de09 push_shared_string [27] 'login_info/account'
0482 @0x0000de0d push_shared_string [24] 'query_temp'
0483 @0x0000de11 push_local_value [1] 
0484 @0x0000de15 push_shared_string [26] 'account'
0485 @0x0000de19 query_member [0] 
0486 @0x0000de1d call_in_other_fun_ex [4, 1] 
0487 @0x0000de21 is_type [4] 
0488 @0x0000de25 if [10, 2] 
0489 @0x0000de29 push_shared_string [107] '验证手机客户端密匙错误，禁止登录\n'
0490 @0x0000de2d push_shared_string [29] 'login'
0491 @0x0000de31 push_local_value [1] 
0492 @0x0000de35 push_shared_string [26] 'account'
0493 @0x0000de39 query_member [0] 
0494 @0x0000de3d call_in_user_fun [698, -127] get_login_info_file
0495 @0x0000de41 push_shared_string [25] 'log_login_info'
0496 @0x0000de45 push_shared_string [30] '/daemons/logd.c'
0497 @0x0000de49 call_in_other_fun_ex [4, 3] 
0498 @0x0000de4d pop_stack [] 
0499 @0x0000de51 push_local_value [0] 
0500 @0x0000de55 call_in_efun [361, 1] destruct_object
0501 @0x0000de59 pop_stack [] 
0502 @0x0000de5d return_from_fun [] 
0503 @0x0000de61 null_action [] 
0504 @0x0000de65 push_number(S) [5] 
0505 @0x0000de69 push_number(S) [4] 
0506 @0x0000de6d create_array [2] 
0507 @0x0000de71 push_local_value [1] 
0508 @0x0000de75 push_shared_string [108] 'client_type'
0509 @0x0000de79 query_member [0] 
0510 @0x0000de7d call_in_efun [83, 2] member_array
0511 @0x0000de81 push_number(S) [-1] 
0512 @0x0000de85 gt_value [] 
0513 @0x0000de89 not_value [] 
0514 @0x0000de8d if [29, 2] 
0515 @0x0000de91 push_local_value [1] 
0516 @0x0000de95 push_shared_string [26] 'account'
0517 @0x0000de99 query_member [0] 
0518 @0x0000de9d is_type [4] 
0519 @0x0000dea1 if [8, 1] 
0520 @0x0000dea5 pop_stack [] 
0521 @0x0000dea9 push_shared_string [27] 'login_info/account'
0522 @0x0000dead push_shared_string [24] 'query_temp'
0523 @0x0000deb1 push_local_value [1] 
0524 @0x0000deb5 push_shared_string [26] 'account'
0525 @0x0000deb9 query_member [0] 
0526 @0x0000debd call_in_other_fun_ex [4, 1] 
0527 @0x0000dec1 is_type [4] 
0528 @0x0000dec5 if [10, 2] 
0529 @0x0000dec9 push_shared_string [109] '验证手机客户端类型错误，禁止登录\n'
0530 @0x0000decd push_shared_string [29] 'login'
0531 @0x0000ded1 push_local_value [1] 
0532 @0x0000ded5 push_shared_string [26] 'account'
0533 @0x0000ded9 query_member [0] 
0534 @0x0000dedd call_in_user_fun [698, -127] get_login_info_file
0535 @0x0000dee1 push_shared_string [25] 'log_login_info'
0536 @0x0000dee5 push_shared_string [30] '/daemons/logd.c'
0537 @0x0000dee9 call_in_other_fun_ex [4, 3] 
0538 @0x0000deed pop_stack [] 
0539 @0x0000def1 push_local_value [0] 
0540 @0x0000def5 call_in_efun [361, 1] destruct_object
0541 @0x0000def9 pop_stack [] 
0542 @0x0000defd return_from_fun [] 
0543 @0x0000df01 null_action [] 
0544 @0x0000df05 push_number(S) [1] 
0545 @0x0000df09 push_shared_string [44] 'get_client'
0546 @0x0000df0d push_local_value [0] 
0547 @0x0000df11 call_in_other_fun_ex [4, 0] 
0548 @0x0000df15 call_in_efun [534, 2] set_mobile_client
0549 @0x0000df19 pop_stack [] 
0550 @0x0000df1d push_shared_string [68] 'update_mobile_client_flag'
0551 @0x0000df21 push_local_value [0] 
0552 @0x0000df25 call_in_other_fun_ex [4, 0] 
0553 @0x0000df29 pop_stack [] 
0554 @0x0000df2d null_action [] 
0555 @0x0000df31 push_local_value [1] 
0556 @0x0000df35 push_shared_string [108] 'client_type'
0557 @0x0000df39 query_member [0] 
0558 @0x0000df3d cast_type [2, 0] 
0559 @0x0000df41 push_local_vptr [-8] 
0560 @0x0000df45 push_shared_string [108] 'client_type'
0561 @0x0000df49 query_member_ptr [0] 
0562 @0x0000df4d assign_value [] 
0563 @0x0000df51 pop_stack [] 
0564 @0x0000df55 push_local_value [1] 
0565 @0x0000df59 push_shared_string [110] 'computer_id'
0566 @0x0000df5d query_member [0] 
0567 @0x0000df61 cast_type [4, 0] 
0568 @0x0000df65 push_local_vptr [-8] 
0569 @0x0000df69 push_shared_string [110] 'computer_id'
0570 @0x0000df6d query_member_ptr [0] 
0571 @0x0000df71 assign_value [] 
0572 @0x0000df75 pop_stack [] 
0573 @0x0000df79 push_local_value [-8] 
0574 @0x0000df7d push_shared_string [111] 'switch_server_flag'
0575 @0x0000df81 query_member [0] 
0576 @0x0000df85 cast_type [2, 0] 
0577 @0x0000df89 push_local_vptr [-3] 
0578 @0x0000df8d assign_value [] 
0579 @0x0000df91 pop_stack [] 
0580 @0x0000df95 push_local_value [-8] 
0581 @0x0000df99 push_shared_string [94] 'service'
0582 @0x0000df9d query_member [0] 
0583 @0x0000dfa1 push_shared_string [69] 'set_service'
0584 @0x0000dfa5 push_local_value [0] 
0585 @0x0000dfa9 call_in_other_fun_ex [4, 1] 
0586 @0x0000dfad pop_stack [] 
0587 @0x0000dfb1 push_local_value [1] 
0588 @0x0000dfb5 push_shared_string [26] 'account'
0589 @0x0000dfb9 query_member [0] 
0590 @0x0000dfbd call_in_user_fun [1018, -127] find_user_by_account
0591 @0x0000dfc1 push_local_vptr [-4] 
0592 @0x0000dfc5 assign_value [] 
0593 @0x0000dfc9 pop_stack [] 
0594 @0x0000dfcd push_local_value [-4] 
0595 @0x0000dfd1 is_type [10] 
0596 @0x0000dfd5 if [450, 2] 
0597 @0x0000dfd9 push_local_value [-4] 
0598 @0x0000dfdd is_type [10] 
0599 @0x0000dfe1 push_number(S) [-1] 
0600 @0x0000dfe5 eq_value [] 
0601 @0x0000dfe9 if [33, 2] 
0602 @0x0000dfed push_local_value [1] 
0603 @0x0000dff1 push_shared_string [26] 'account'
0604 @0x0000dff5 query_member [0] 
0605 @0x0000dff9 is_type [4] 
0606 @0x0000dffd if [8, 1] 
0607 @0x0000e001 pop_stack [] 
0608 @0x0000e005 push_shared_string [27] 'login_info/account'
0609 @0x0000e009 push_shared_string [24] 'query_temp'
0610 @0x0000e00d push_local_value [1] 
0611 @0x0000e011 push_shared_string [26] 'account'
0612 @0x0000e015 query_member [0] 
0613 @0x0000e019 call_in_other_fun_ex [4, 1] 
0614 @0x0000e01d is_type [4] 
0615 @0x0000e021 if [14, 2] 
0616 @0x0000e025 push_shared_string [46] 'get_id'
0617 @0x0000e029 push_local_value [-4] 
0618 @0x0000e02d call_in_other_fun_ex [4, 0] 
0619 @0x0000e031 push_shared_string [112] '原对象(%d)正在析构中(3)，本连接无法取代之\n'
0620 @0x0000e035 call_in_efun [121, 2] sprintf
0621 @0x0000e039 push_shared_string [113] 'replace'
0622 @0x0000e03d push_local_value [1] 
0623 @0x0000e041 push_shared_string [26] 'account'
0624 @0x0000e045 query_member [0] 
0625 @0x0000e049 call_in_user_fun [698, -127] get_login_info_file
0626 @0x0000e04d push_shared_string [25] 'log_login_info'
0627 @0x0000e051 push_shared_string [30] '/daemons/logd.c'
0628 @0x0000e055 call_in_other_fun_ex [4, 3] 
0629 @0x0000e059 pop_stack [] 
0630 @0x0000e05d push_local_value [0] 
0631 @0x0000e061 call_in_efun [361, 1] destruct_object
0632 @0x0000e065 pop_stack [] 
0633 @0x0000e069 return_from_fun [] 
0634 @0x0000e06d null_action [] 
0635 @0x0000e071 push_shared_string [114] 'logout'
0636 @0x0000e075 push_shared_string [64] 'lite_query_temp'
0637 @0x0000e079 push_local_value [-4] 
0638 @0x0000e07d call_in_other_fun_ex [4, 1] 
0639 @0x0000e081 if [42, 2] 
0640 @0x0000e085 push_local_value [1] 
0641 @0x0000e089 push_shared_string [26] 'account'
0642 @0x0000e08d query_member [0] 
0643 @0x0000e091 is_type [4] 
0644 @0x0000e095 if [8, 1] 
0645 @0x0000e099 pop_stack [] 
0646 @0x0000e09d push_shared_string [27] 'login_info/account'
0647 @0x0000e0a1 push_shared_string [24] 'query_temp'
0648 @0x0000e0a5 push_local_value [1] 
0649 @0x0000e0a9 push_shared_string [26] 'account'
0650 @0x0000e0ad query_member [0] 
0651 @0x0000e0b1 call_in_other_fun_ex [4, 1] 
0652 @0x0000e0b5 is_type [4] 
0653 @0x0000e0b9 if [14, 2] 
0654 @0x0000e0bd push_shared_string [70] 'get_name'
0655 @0x0000e0c1 push_local_value [-4] 
0656 @0x0000e0c5 call_in_other_fun_ex [4, 0] 
0657 @0x0000e0c9 push_shared_string [115] '原角色(%s)正在登出中(1)，本连接无法取代之\n'
0658 @0x0000e0cd call_in_efun [121, 2] sprintf
0659 @0x0000e0d1 push_shared_string [113] 'replace'
0660 @0x0000e0d5 push_local_value [1] 
0661 @0x0000e0d9 push_shared_string [26] 'account'
0662 @0x0000e0dd query_member [0] 
0663 @0x0000e0e1 call_in_user_fun [698, -127] get_login_info_file
0664 @0x0000e0e5 push_shared_string [25] 'log_login_info'
0665 @0x0000e0e9 push_shared_string [30] '/daemons/logd.c'
0666 @0x0000e0ed call_in_other_fun_ex [4, 3] 
0667 @0x0000e0f1 pop_stack [] 
0668 @0x0000e0f5 push_shared_string [70] 'get_name'
0669 @0x0000e0f9 push_local_value [-4] 
0670 @0x0000e0fd call_in_other_fun_ex [4, 0] 
0671 @0x0000e101 push_shared_string [116] '角色%s正在进行登出操作，此时不能取代。（原因：9038）'
0672 @0x0000e105 call_in_efun [121, 2] sprintf
0673 @0x0000e109 push_shared_string [45] 'send_dialog_ok'
0674 @0x0000e10d push_local_value [0] 
0675 @0x0000e111 call_in_other_fun_ex [4, 1] 
0676 @0x0000e115 pop_stack [] 
0677 @0x0000e119 push_local_value [0] 
0678 @0x0000e11d call_in_efun [361, 1] destruct_object
0679 @0x0000e121 pop_stack [] 
0680 @0x0000e125 return_from_fun [] 
0681 @0x0000e129 null_action [] 
0682 @0x0000e12d push_obj_var [3] 
0683 @0x0000e131 push_local_value [1] 
0684 @0x0000e135 push_shared_string [26] 'account'
0685 @0x0000e139 query_member [0] 
0686 @0x0000e13d query_member [0] 
0687 @0x0000e141 if [42, 2] 
0688 @0x0000e145 push_local_value [1] 
0689 @0x0000e149 push_shared_string [26] 'account'
0690 @0x0000e14d query_member [0] 
0691 @0x0000e151 is_type [4] 
0692 @0x0000e155 if [8, 1] 
0693 @0x0000e159 pop_stack [] 
0694 @0x0000e15d push_shared_string [27] 'login_info/account'
0695 @0x0000e161 push_shared_string [24] 'query_temp'
0696 @0x0000e165 push_local_value [1] 
0697 @0x0000e169 push_shared_string [26] 'account'
0698 @0x0000e16d query_member [0] 
0699 @0x0000e171 call_in_other_fun_ex [4, 1] 
0700 @0x0000e175 is_type [4] 
0701 @0x0000e179 if [14, 2] 
0702 @0x0000e17d push_shared_string [46] 'get_id'
0703 @0x0000e181 push_local_value [-4] 
0704 @0x0000e185 call_in_other_fun_ex [4, 0] 
0705 @0x0000e189 push_shared_string [117] '原对象(%d)正在进行敏感中，本连接无法取代之\n'
0706 @0x0000e18d call_in_efun [121, 2] sprintf
0707 @0x0000e191 push_shared_string [113] 'replace'
0708 @0x0000e195 push_local_value [1] 
0709 @0x0000e199 push_shared_string [26] 'account'
0710 @0x0000e19d query_member [0] 
0711 @0x0000e1a1 call_in_user_fun [698, -127] get_login_info_file
0712 @0x0000e1a5 push_shared_string [25] 'log_login_info'
0713 @0x0000e1a9 push_shared_string [30] '/daemons/logd.c'
0714 @0x0000e1ad call_in_other_fun_ex [4, 3] 
0715 @0x0000e1b1 pop_stack [] 
0716 @0x0000e1b5 push_local_value [1] 
0717 @0x0000e1b9 push_shared_string [26] 'account'
0718 @0x0000e1bd query_member [0] 
0719 @0x0000e1c1 push_shared_string [118] '帐号%s正在进行重要操作，此时不能取代。（原因：9027）'
0720 @0x0000e1c5 call_in_efun [121, 2] sprintf
0721 @0x0000e1c9 push_shared_string [45] 'send_dialog_ok'
0722 @0x0000e1cd push_local_value [0] 
0723 @0x0000e1d1 call_in_other_fun_ex [4, 1] 
0724 @0x0000e1d5 pop_stack [] 
0725 @0x0000e1d9 push_local_value [0] 
0726 @0x0000e1dd call_in_efun [361, 1] destruct_object
0727 @0x0000e1e1 pop_stack [] 
0728 @0x0000e1e5 return_from_fun [] 
0729 @0x0000e1e9 null_action [] 
0730 @0x0000e1ed push_local_value [1] 
0731 @0x0000e1f1 push_shared_string [26] 'account'
0732 @0x0000e1f5 query_member [0] 
0733 @0x0000e1f9 is_type [4] 
0734 @0x0000e1fd if [8, 1] 
0735 @0x0000e201 pop_stack [] 
0736 @0x0000e205 push_shared_string [27] 'login_info/account'
0737 @0x0000e209 push_shared_string [24] 'query_temp'
0738 @0x0000e20d push_local_value [1] 
0739 @0x0000e211 push_shared_string [26] 'account'
0740 @0x0000e215 query_member [0] 
0741 @0x0000e219 call_in_other_fun_ex [4, 1] 
0742 @0x0000e21d is_type [4] 
0743 @0x0000e221 if [10, 2] 
0744 @0x0000e225 push_shared_string [119] '该帐号处于活动状态，正在试图重新连接或者取代连接\n'
0745 @0x0000e229 push_shared_string [113] 'replace'
0746 @0x0000e22d push_local_value [1] 
0747 @0x0000e231 push_shared_string [26] 'account'
0748 @0x0000e235 query_member [0] 
0749 @0x0000e239 call_in_user_fun [698, -127] get_login_info_file
0750 @0x0000e23d push_shared_string [25] 'log_login_info'
0751 @0x0000e241 push_shared_string [30] '/daemons/logd.c'
0752 @0x0000e245 call_in_other_fun_ex [4, 3] 
0753 @0x0000e249 pop_stack [] 
0754 @0x0000e24d push_shared_string [71] 'is_login'
0755 @0x0000e251 push_local_value [-4] 
0756 @0x0000e255 call_in_other_fun_ex [4, 0] 
0757 @0x0000e259 if [143, 2] 
0758 @0x0000e25d push_shared_string [120] 'select_char_now'
0759 @0x0000e261 push_shared_string [64] 'lite_query_temp'
0760 @0x0000e265 push_local_value [-4] 
0761 @0x0000e269 call_in_other_fun_ex [4, 1] 
0762 @0x0000e26d push_local_vptr [-7] 
0763 @0x0000e271 assign_value [] 
0764 @0x0000e275 is_type [4] 
0765 @0x0000e279 if [48, 2] 
0766 @0x0000e27d push_local_value [1] 
0767 @0x0000e281 push_shared_string [26] 'account'
0768 @0x0000e285 query_member [0] 
0769 @0x0000e289 is_type [4] 
0770 @0x0000e28d if [8, 1] 
0771 @0x0000e291 pop_stack [] 
0772 @0x0000e295 push_shared_string [27] 'login_info/account'
0773 @0x0000e299 push_shared_string [24] 'query_temp'
0774 @0x0000e29d push_local_value [1] 
0775 @0x0000e2a1 push_shared_string [26] 'account'
0776 @0x0000e2a5 query_member [0] 
0777 @0x0000e2a9 call_in_other_fun_ex [4, 1] 
0778 @0x0000e2ad is_type [4] 
0779 @0x0000e2b1 if [20, 2] 
0780 @0x0000e2b5 push_local_value [-7] 
0781 @0x0000e2b9 push_shared_string [44] 'get_client'
0782 @0x0000e2bd push_local_value [-4] 
0783 @0x0000e2c1 call_in_other_fun_ex [4, 0] 
0784 @0x0000e2c5 call_in_efun [515, 1] get_client_info
0785 @0x0000e2c9 null_action [] 
0786 @0x0000e2cd push_shared_string [86] 'remote_address'
0787 @0x0000e2d1 query_member [0] 
0788 @0x0000e2d5 cast_type [4, 0] 
0789 @0x0000e2d9 push_shared_string [121] '来自[%s]的地址已经选定角色[%s]，本连接无法取代之\n'
0790 @0x0000e2dd call_in_efun [121, 3] sprintf
0791 @0x0000e2e1 push_shared_string [113] 'replace'
0792 @0x0000e2e5 push_local_value [1] 
0793 @0x0000e2e9 push_shared_string [26] 'account'
0794 @0x0000e2ed query_member [0] 
0795 @0x0000e2f1 call_in_user_fun [698, -127] get_login_info_file
0796 @0x0000e2f5 push_shared_string [25] 'log_login_info'
0797 @0x0000e2f9 push_shared_string [30] '/daemons/logd.c'
0798 @0x0000e2fd call_in_other_fun_ex [4, 3] 
0799 @0x0000e301 pop_stack [] 
0800 @0x0000e305 push_local_value [1] 
0801 @0x0000e309 push_shared_string [26] 'account'
0802 @0x0000e30d query_member [0] 
0803 @0x0000e311 push_shared_string [118] '帐号%s正在进行重要操作，此时不能取代。（原因：9027）'
0804 @0x0000e315 call_in_efun [121, 2] sprintf
0805 @0x0000e319 push_shared_string [45] 'send_dialog_ok'
0806 @0x0000e31d push_local_value [0] 
0807 @0x0000e321 call_in_other_fun_ex [4, 1] 
0808 @0x0000e325 pop_stack [] 
0809 @0x0000e329 push_local_value [0] 
0810 @0x0000e32d call_in_efun [361, 1] destruct_object
0811 @0x0000e331 pop_stack [] 
0812 @0x0000e335 return_from_fun [] 
0813 @0x0000e339 null_action [] 
0814 @0x0000e33d push_local_value [1] 
0815 @0x0000e341 push_shared_string [26] 'account'
0816 @0x0000e345 query_member [0] 
0817 @0x0000e349 is_type [4] 
0818 @0x0000e34d if [8, 1] 
0819 @0x0000e351 pop_stack [] 
0820 @0x0000e355 push_shared_string [27] 'login_info/account'
0821 @0x0000e359 push_shared_string [24] 'query_temp'
0822 @0x0000e35d push_local_value [1] 
0823 @0x0000e361 push_shared_string [26] 'account'
0824 @0x0000e365 query_member [0] 
0825 @0x0000e369 call_in_other_fun_ex [4, 1] 
0826 @0x0000e36d is_type [4] 
0827 @0x0000e371 if [21, 2] 
0828 @0x0000e375 push_shared_string [122] '来自['
0829 @0x0000e379 push_shared_string [44] 'get_client'
0830 @0x0000e37d push_local_value [-4] 
0831 @0x0000e381 call_in_other_fun_ex [4, 0] 
0832 @0x0000e385 call_in_efun [515, 1] get_client_info
0833 @0x0000e389 null_action [] 
0834 @0x0000e38d push_shared_string [86] 'remote_address'
0835 @0x0000e391 query_member [0] 
0836 @0x0000e395 cast_type [4, 0] 
0837 @0x0000e399 add [] 
0838 @0x0000e39d push_shared_string [123] ']的地址也正在登录该帐号，本连接取代它(3)\n'
0839 @0x0000e3a1 add [] 
0840 @0x0000e3a5 push_shared_string [113] 'replace'
0841 @0x0000e3a9 push_local_value [1] 
0842 @0x0000e3ad push_shared_string [26] 'account'
0843 @0x0000e3b1 query_member [0] 
0844 @0x0000e3b5 call_in_user_fun [698, -127] get_login_info_file
0845 @0x0000e3b9 push_shared_string [25] 'log_login_info'
0846 @0x0000e3bd push_shared_string [30] '/daemons/logd.c'
0847 @0x0000e3c1 call_in_other_fun_ex [4, 3] 
0848 @0x0000e3c5 pop_stack [] 
0849 @0x0000e3c9 push_shared_string [72] 'remove_link_of_runtime_info'
0850 @0x0000e3cd push_local_value [-4] 
0851 @0x0000e3d1 call_in_other_fun_ex [4, 0] 
0852 @0x0000e3d5 pop_stack [] 
0853 @0x0000e3d9 push_shared_string [124] '其他地址试图登录本帐号，连接中断。（原因：9004）'
0854 @0x0000e3dd push_shared_string [45] 'send_dialog_ok'
0855 @0x0000e3e1 push_local_value [-4] 
0856 @0x0000e3e5 call_in_other_fun_ex [4, 1] 
0857 @0x0000e3e9 pop_stack [] 
0858 @0x0000e3ed push_local_value [-4] 
0859 @0x0000e3f1 call_in_efun [361, 1] destruct_object
0860 @0x0000e3f5 pop_stack [] 
0861 @0x0000e3f9 push_local_value [-4] 
0862 @0x0000e3fd is_type [10] 
0863 @0x0000e401 push_number(S) [-1] 
0864 @0x0000e405 eq_value [] 
0865 @0x0000e409 if [33, 2] 
0866 @0x0000e40d push_local_value [1] 
0867 @0x0000e411 push_shared_string [26] 'account'
0868 @0x0000e415 query_member [0] 
0869 @0x0000e419 is_type [4] 
0870 @0x0000e41d if [8, 1] 
0871 @0x0000e421 pop_stack [] 
0872 @0x0000e425 push_shared_string [27] 'login_info/account'
0873 @0x0000e429 push_shared_string [24] 'query_temp'
0874 @0x0000e42d push_local_value [1] 
0875 @0x0000e431 push_shared_string [26] 'account'
0876 @0x0000e435 query_member [0] 
0877 @0x0000e439 call_in_other_fun_ex [4, 1] 
0878 @0x0000e43d is_type [4] 
0879 @0x0000e441 if [14, 2] 
0880 @0x0000e445 push_shared_string [46] 'get_id'
0881 @0x0000e449 push_local_value [-4] 
0882 @0x0000e44d call_in_other_fun_ex [4, 0] 
0883 @0x0000e451 push_shared_string [125] '原对象(%d)正在析构中(4)，本连接无法取代之\n'
0884 @0x0000e455 call_in_efun [121, 2] sprintf
0885 @0x0000e459 push_shared_string [113] 'replace'
0886 @0x0000e45d push_local_value [1] 
0887 @0x0000e461 push_shared_string [26] 'account'
0888 @0x0000e465 query_member [0] 
0889 @0x0000e469 call_in_user_fun [698, -127] get_login_info_file
0890 @0x0000e46d push_shared_string [25] 'log_login_info'
0891 @0x0000e471 push_shared_string [30] '/daemons/logd.c'
0892 @0x0000e475 call_in_other_fun_ex [4, 3] 
0893 @0x0000e479 pop_stack [] 
0894 @0x0000e47d push_local_value [0] 
0895 @0x0000e481 call_in_efun [361, 1] destruct_object
0896 @0x0000e485 pop_stack [] 
0897 @0x0000e489 return_from_fun [] 
0898 @0x0000e48d null_action [] 
0899 @0x0000e491 null_action [] 
0900 @0x0000e495 goto [144] 
0901 @0x0000e499 push_shared_string [126] 'starting_game'
0902 @0x0000e49d push_shared_string [64] 'lite_query_temp'
0903 @0x0000e4a1 push_local_value [-4] 
0904 @0x0000e4a5 call_in_other_fun_ex [4, 1] 
0905 @0x0000e4a9 if [34, 2] 
0906 @0x0000e4ad push_local_value [1] 
0907 @0x0000e4b1 push_shared_string [26] 'account'
0908 @0x0000e4b5 query_member [0] 
0909 @0x0000e4b9 is_type [4] 
0910 @0x0000e4bd if [8, 1] 
0911 @0x0000e4c1 pop_stack [] 
0912 @0x0000e4c5 push_shared_string [27] 'login_info/account'
0913 @0x0000e4c9 push_shared_string [24] 'query_temp'
0914 @0x0000e4cd push_local_value [1] 
0915 @0x0000e4d1 push_shared_string [26] 'account'
0916 @0x0000e4d5 query_member [0] 
0917 @0x0000e4d9 call_in_other_fun_ex [4, 1] 
0918 @0x0000e4dd is_type [4] 
0919 @0x0000e4e1 if [14, 2] 
0920 @0x0000e4e5 push_shared_string [46] 'get_id'
0921 @0x0000e4e9 push_local_value [-4] 
0922 @0x0000e4ed call_in_other_fun_ex [4, 0] 
0923 @0x0000e4f1 push_shared_string [127] '原对象(%d)正在进入游戏过程中，本连接无法取代之\n'
0924 @0x0000e4f5 call_in_efun [121, 2] sprintf
0925 @0x0000e4f9 push_shared_string [113] 'replace'
0926 @0x0000e4fd push_local_value [1] 
0927 @0x0000e501 push_shared_string [26] 'account'
0928 @0x0000e505 query_member [0] 
0929 @0x0000e509 call_in_user_fun [698, -127] get_login_info_file
0930 @0x0000e50d push_shared_string [25] 'log_login_info'
0931 @0x0000e511 push_shared_string [30] '/daemons/logd.c'
0932 @0x0000e515 call_in_other_fun_ex [4, 3] 
0933 @0x0000e519 pop_stack [] 
0934 @0x0000e51d push_local_value [0] 
0935 @0x0000e521 call_in_efun [361, 1] destruct_object
0936 @0x0000e525 pop_stack [] 
0937 @0x0000e529 return_from_fun [] 
0938 @0x0000e52d null_action [] 
0939 @0x0000e531 goto [105] 
0940 @0x0000e535 push_shared_string [128] 'entered_world'
0941 @0x0000e539 push_shared_string [64] 'lite_query_temp'
0942 @0x0000e53d push_local_value [-4] 
0943 @0x0000e541 call_in_other_fun_ex [4, 1] 
0944 @0x0000e545 not_value [] 
0945 @0x0000e549 if [34, 2] 
0946 @0x0000e54d push_local_value [1] 
0947 @0x0000e551 push_shared_string [26] 'account'
0948 @0x0000e555 query_member [0] 
0949 @0x0000e559 is_type [4] 
0950 @0x0000e55d if [8, 1] 
0951 @0x0000e561 pop_stack [] 
0952 @0x0000e565 push_shared_string [27] 'login_info/account'
0953 @0x0000e569 push_shared_string [24] 'query_temp'
0954 @0x0000e56d push_local_value [1] 
0955 @0x0000e571 push_shared_string [26] 'account'
0956 @0x0000e575 query_member [0] 
0957 @0x0000e579 call_in_other_fun_ex [4, 1] 
0958 @0x0000e57d is_type [4] 
0959 @0x0000e581 if [14, 2] 
0960 @0x0000e585 push_shared_string [46] 'get_id'
0961 @0x0000e589 push_local_value [-4] 
0962 @0x0000e58d call_in_other_fun_ex [4, 0] 
0963 @0x0000e591 push_shared_string [129] '玩家上次没有成功完成登录过程，有残余的USER_OB(%d)，析构之\n'
0964 @0x0000e595 call_in_efun [121, 2] sprintf
0965 @0x0000e599 push_shared_string [113] 'replace'
0966 @0x0000e59d push_local_value [1] 
0967 @0x0000e5a1 push_shared_string [26] 'account'
0968 @0x0000e5a5 query_member [0] 
0969 @0x0000e5a9 call_in_user_fun [698, -127] get_login_info_file
0970 @0x0000e5ad push_shared_string [25] 'log_login_info'
0971 @0x0000e5b1 push_shared_string [30] '/daemons/logd.c'
0972 @0x0000e5b5 call_in_other_fun_ex [4, 3] 
0973 @0x0000e5b9 pop_stack [] 
0974 @0x0000e5bd push_local_value [-4] 
0975 @0x0000e5c1 call_in_efun [361, 1] destruct_object
0976 @0x0000e5c5 pop_stack [] 
0977 @0x0000e5c9 return_from_fun [] 
0978 @0x0000e5cd null_action [] 
0979 @0x0000e5d1 goto [65] 
0980 @0x0000e5d5 push_local_value [-8] 
0981 @0x0000e5d9 push_shared_string [130] 'mac'
0982 @0x0000e5dd query_member [0] 
0983 @0x0000e5e1 push_shared_string [131] 'login_info/mac'
0984 @0x0000e5e5 push_shared_string [73] 'set_temp'
0985 @0x0000e5e9 push_local_value [0] 
0986 @0x0000e5ed call_in_other_fun_ex [4, 2] 
0987 @0x0000e5f1 pop_stack [] 
0988 @0x0000e5f5 push_local_value [-8] 
0989 @0x0000e5f9 push_shared_string [108] 'client_type'
0990 @0x0000e5fd query_member [0] 
0991 @0x0000e601 push_shared_string [132] 'login_info/client_type'
0992 @0x0000e605 push_shared_string [73] 'set_temp'
0993 @0x0000e609 push_local_value [0] 
0994 @0x0000e60d call_in_other_fun_ex [4, 2] 
0995 @0x0000e611 pop_stack [] 
0996 @0x0000e615 push_local_value [-8] 
0997 @0x0000e619 push_shared_string [110] 'computer_id'
0998 @0x0000e61d query_member [0] 
0999 @0x0000e621 push_shared_string [133] 'login_info/computer_id'
1000 @0x0000e625 push_shared_string [73] 'set_temp'
1001 @0x0000e629 push_local_value [0] 
1002 @0x0000e62d call_in_other_fun_ex [4, 2] 
1003 @0x0000e631 pop_stack [] 
1004 @0x0000e635 push_local_value [-8] 
1005 @0x0000e639 push_shared_string [134] 'coin_info'
1006 @0x0000e63d query_member [0] 
1007 @0x0000e641 push_shared_string [135] 'login_info/coin_info'
1008 @0x0000e645 push_shared_string [73] 'set_temp'
1009 @0x0000e649 push_local_value [0] 
1010 @0x0000e64d call_in_other_fun_ex [4, 2] 
1011 @0x0000e651 pop_stack [] 
1012 @0x0000e655 push_local_value [-8] 
1013 @0x0000e659 push_shared_string [136] 'protects'
1014 @0x0000e65d query_member [0] 
1015 @0x0000e661 push_shared_string [136] 'protects'
1016 @0x0000e665 push_shared_string [65] 'lite_set_temp'
1017 @0x0000e669 push_local_value [0] 
1018 @0x0000e66d call_in_other_fun_ex [4, 2] 
1019 @0x0000e671 pop_stack [] 
1020 @0x0000e675 push_local_value [-8] 
1021 @0x0000e679 push_shared_string [137] 'is_bind'
1022 @0x0000e67d query_member [0] 
1023 @0x0000e681 push_shared_string [137] 'is_bind'
1024 @0x0000e685 push_shared_string [65] 'lite_set_temp'
1025 @0x0000e689 push_local_value [0] 
1026 @0x0000e68d call_in_other_fun_ex [4, 2] 
1027 @0x0000e691 pop_stack [] 
1028 @0x0000e695 push_local_value [-8] 
1029 @0x0000e699 push_shared_string [138] 'all_protects'
1030 @0x0000e69d query_member [0] 
1031 @0x0000e6a1 push_shared_string [138] 'all_protects'
1032 @0x0000e6a5 push_shared_string [65] 'lite_set_temp'
1033 @0x0000e6a9 push_local_value [0] 
1034 @0x0000e6ad call_in_other_fun_ex [4, 2] 
1035 @0x0000e6b1 pop_stack [] 
1036 @0x0000e6b5 push_local_value [0] 
1037 @0x0000e6b9 push_local_value [1] 
1038 @0x0000e6bd push_shared_string [70] 'get_name'
1039 @0x0000e6c1 push_local_value [-4] 
1040 @0x0000e6c5 call_in_other_fun_ex [4, 0] 
1041 @0x0000e6c9 call_in_user_fun [31, 3] do_replace_login
1042 @0x0000e6cd pop_stack [] 
1043 @0x0000e6d1 return_from_fun [] 
1044 @0x0000e6d5 null_action [] 
1045 @0x0000e6d9 null_action [] 
1046 @0x0000e6dd goto [48] 
1047 @0x0000e6e1 push_obj_var [15] 
1048 @0x0000e6e5 if [4, 0] 
1049 @0x0000e6e9 pop_stack [] 
1050 @0x0000e6ed push_local_value [-8] 
1051 @0x0000e6f1 push_shared_string [139] 'cs_replace_login'
1052 @0x0000e6f5 query_member [0] 
1053 @0x0000e6f9 if [6, 0] 
1054 @0x0000e6fd pop_stack [] 
1055 @0x0000e701 push_obj_var [14] 
1056 @0x0000e705 push_local_value [1] 
1057 @0x0000e709 push_shared_string [26] 'account'
1058 @0x0000e70d query_member [0] 
1059 @0x0000e711 query_member [0] 
1060 @0x0000e715 if [34, 2] 
1061 @0x0000e719 push_local_value [1] 
1062 @0x0000e71d push_shared_string [26] 'account'
1063 @0x0000e721 query_member [0] 
1064 @0x0000e725 is_type [4] 
1065 @0x0000e729 if [8, 1] 
1066 @0x0000e72d pop_stack [] 
1067 @0x0000e731 push_shared_string [27] 'login_info/account'
1068 @0x0000e735 push_shared_string [24] 'query_temp'
1069 @0x0000e739 push_local_value [1] 
1070 @0x0000e73d push_shared_string [26] 'account'
1071 @0x0000e741 query_member [0] 
1072 @0x0000e745 call_in_other_fun_ex [4, 1] 
1073 @0x0000e749 is_type [4] 
1074 @0x0000e74d if [10, 2] 
1075 @0x0000e751 push_shared_string [140] '顶号登录跨服区组失败！'
1076 @0x0000e755 push_shared_string [139] 'cs_replace_login'
1077 @0x0000e759 push_local_value [1] 
1078 @0x0000e75d push_shared_string [26] 'account'
1079 @0x0000e761 query_member [0] 
1080 @0x0000e765 call_in_user_fun [698, -127] get_login_info_file
1081 @0x0000e769 push_shared_string [25] 'log_login_info'
1082 @0x0000e76d push_shared_string [30] '/daemons/logd.c'
1083 @0x0000e771 call_in_other_fun_ex [4, 3] 
1084 @0x0000e775 pop_stack [] 
1085 @0x0000e779 push_shared_string [140] '顶号登录跨服区组失败！'
1086 @0x0000e77d push_shared_string [45] 'send_dialog_ok'
1087 @0x0000e781 push_local_value [0] 
1088 @0x0000e785 call_in_other_fun_ex [4, 1] 
1089 @0x0000e789 pop_stack [] 
1090 @0x0000e78d push_local_value [0] 
1091 @0x0000e791 call_in_efun [361, 1] destruct_object
1092 @0x0000e795 pop_stack [] 
1093 @0x0000e799 return_from_fun [] 
1094 @0x0000e79d null_action [] 
1095 @0x0000e7a1 push_local_value [-8] 
1096 @0x0000e7a5 push_shared_string [141] 'login_info'
1097 @0x0000e7a9 push_shared_string [65] 'lite_set_temp'
1098 @0x0000e7ad push_local_value [0] 
1099 @0x0000e7b1 call_in_other_fun_ex [4, 2] 
1100 @0x0000e7b5 pop_stack [] 
1101 @0x0000e7b9 push_local_value [1] 
1102 @0x0000e7bd push_shared_string [26] 'account'
1103 @0x0000e7c1 query_member [0] 
1104 @0x0000e7c5 call_in_user_fun [33, 1] do_stun_account
1105 @0x0000e7c9 pop_stack [] 
1106 @0x0000e7cd new_thread_catch [10] 
1107 @0x0000e7d1 push_local_value [-8] 
1108 @0x0000e7d5 push_local_value [1] 
1109 @0x0000e7d9 push_local_value [0] 
1110 @0x0000e7dd call_in_user_fun [78, 3] _do_login
1111 @0x0000e7e1 push_local_vptr [-1] 
1112 @0x0000e7e5 assign_value [] 
1113 @0x0000e7e9 pop_stack [] 
1114 @0x0000e7ed push_number(S) [0] 
1115 @0x0000e7f1 end_thread_catch [] 
1116 @0x0000e7f5 goto [1] 
1117 @0x0000e7f9 caught_error [] 
1118 @0x0000e7fd pop_stack [] 
1119 @0x0000e801 push_local_value [1] 
1120 @0x0000e805 push_shared_string [26] 'account'
1121 @0x0000e809 query_member [0] 
1122 @0x0000e80d call_in_user_fun [52, 1] stop_stun_account
1123 @0x0000e811 pop_stack [] 
1124 @0x0000e815 push_local_value [-1] 
1125 @0x0000e819 not_value [] 
1126 @0x0000e81d if [29, 2] 
1127 @0x0000e821 push_local_value [1] 
1128 @0x0000e825 push_shared_string [26] 'account'
1129 @0x0000e829 query_member [0] 
1130 @0x0000e82d is_type [4] 
1131 @0x0000e831 if [8, 1] 
1132 @0x0000e835 pop_stack [] 
1133 @0x0000e839 push_shared_string [27] 'login_info/account'
1134 @0x0000e83d push_shared_string [24] 'query_temp'
1135 @0x0000e841 push_local_value [1] 
1136 @0x0000e845 push_shared_string [26] 'account'
1137 @0x0000e849 query_member [0] 
1138 @0x0000e84d call_in_other_fun_ex [4, 1] 
1139 @0x0000e851 is_type [4] 
1140 @0x0000e855 if [10, 2] 
1141 @0x0000e859 push_shared_string [142] '操作失败，中止登录\n'
1142 @0x0000e85d push_shared_string [143] 'char'
1143 @0x0000e861 push_local_value [1] 
1144 @0x0000e865 push_shared_string [26] 'account'
1145 @0x0000e869 query_member [0] 
1146 @0x0000e86d call_in_user_fun [698, -127] get_login_info_file
1147 @0x0000e871 push_shared_string [25] 'log_login_info'
1148 @0x0000e875 push_shared_string [30] '/daemons/logd.c'
1149 @0x0000e879 call_in_other_fun_ex [4, 3] 
1150 @0x0000e87d pop_stack [] 
1151 @0x0000e881 push_local_value [0] 
1152 @0x0000e885 call_in_efun [361, 1] destruct_object
1153 @0x0000e889 pop_stack [] 
1154 @0x0000e88d return_from_fun [] 
1155 @0x0000e891 null_action [] 
1156 @0x0000e895 push_local_value [1] 
1157 @0x0000e899 push_shared_string [26] 'account'
1158 @0x0000e89d query_member [0] 
1159 @0x0000e8a1 is_type [4] 
1160 @0x0000e8a5 if [8, 1] 
1161 @0x0000e8a9 pop_stack [] 
1162 @0x0000e8ad push_shared_string [27] 'login_info/account'
1163 @0x0000e8b1 push_shared_string [24] 'query_temp'
1164 @0x0000e8b5 push_local_value [1] 
1165 @0x0000e8b9 push_shared_string [26] 'account'
1166 @0x0000e8bd query_member [0] 
1167 @0x0000e8c1 call_in_other_fun_ex [4, 1] 
1168 @0x0000e8c5 is_type [4] 
1169 @0x0000e8c9 if [10, 2] 
1170 @0x0000e8cd push_shared_string [144] '读取该帐号的角色列表信息\n'
1171 @0x0000e8d1 push_shared_string [143] 'char'
1172 @0x0000e8d5 push_local_value [1] 
1173 @0x0000e8d9 push_shared_string [26] 'account'
1174 @0x0000e8dd query_member [0] 
1175 @0x0000e8e1 call_in_user_fun [698, -127] get_login_info_file
1176 @0x0000e8e5 push_shared_string [25] 'log_login_info'
1177 @0x0000e8e9 push_shared_string [30] '/daemons/logd.c'
1178 @0x0000e8ed call_in_other_fun_ex [4, 3] 
1179 @0x0000e8f1 pop_stack [] 
1180 @0x0000e8f5 push_local_vptr [-2] 
1181 @0x0000e8f9 push_shared_string [74] 'restore_ex'
1182 @0x0000e8fd push_local_value [0] 
1183 @0x0000e901 call_in_other_fun_ex [4, 1] 
1184 @0x0000e905 not_value [] 
1185 @0x0000e909 if [102, 2] 
1186 @0x0000e90d push_local_value [1] 
1187 @0x0000e911 push_shared_string [26] 'account'
1188 @0x0000e915 query_member [0] 
1189 @0x0000e919 is_type [4] 
1190 @0x0000e91d if [8, 1] 
1191 @0x0000e921 pop_stack [] 
1192 @0x0000e925 push_shared_string [27] 'login_info/account'
1193 @0x0000e929 push_shared_string [24] 'query_temp'
1194 @0x0000e92d push_local_value [1] 
1195 @0x0000e931 push_shared_string [26] 'account'
1196 @0x0000e935 query_member [0] 
1197 @0x0000e939 call_in_other_fun_ex [4, 1] 
1198 @0x0000e93d is_type [4] 
1199 @0x0000e941 if [10, 2] 
1200 @0x0000e945 push_shared_string [145] '该帐号的角色列表信息读取失败，帐号可能不存在，需要创建该帐号2。\n'
1201 @0x0000e949 push_shared_string [91] 'error'
1202 @0x0000e94d push_local_value [1] 
1203 @0x0000e951 push_shared_string [26] 'account'
1204 @0x0000e955 query_member [0] 
1205 @0x0000e959 call_in_user_fun [698, -127] get_login_info_file
1206 @0x0000e95d push_shared_string [25] 'log_login_info'
1207 @0x0000e961 push_shared_string [30] '/daemons/logd.c'
1208 @0x0000e965 call_in_other_fun_ex [4, 3] 
1209 @0x0000e969 pop_stack [] 
1210 @0x0000e96d push_local_value [-2] 
1211 @0x0000e971 push_number(S) [2] 
1212 @0x0000e975 ne_value [] 
1213 @0x0000e979 if [30, 2] 
1214 @0x0000e97d push_obj_var [8] 
1215 @0x0000e981 if [18, 2] 
1216 @0x0000e985 push_local_value [0] 
1217 @0x0000e989 is_type [4] 
1218 @0x0000e98d if [6, 1] 
1219 @0x0000e991 pop_stack [] 
1220 @0x0000e995 push_shared_string [27] 'login_info/account'
1221 @0x0000e999 push_shared_string [24] 'query_temp'
1222 @0x0000e99d push_local_value [0] 
1223 @0x0000e9a1 call_in_other_fun_ex [4, 1] 
1224 @0x0000e9a5 is_type [4] 
1225 @0x0000e9a9 if [8, 2] 
1226 @0x0000e9ad push_shared_string [146] '你的帐号目前无法登录，请稍后再试。（原因：9005）'
1227 @0x0000e9b1 push_shared_string [91] 'error'
1228 @0x0000e9b5 push_local_value [0] 
1229 @0x0000e9b9 call_in_user_fun [698, -127] get_login_info_file
1230 @0x0000e9bd push_shared_string [25] 'log_login_info'
1231 @0x0000e9c1 push_shared_string [30] '/daemons/logd.c'
1232 @0x0000e9c5 call_in_other_fun_ex [4, 3] 
1233 @0x0000e9c9 pop_stack [] 
1234 @0x0000e9cd push_shared_string [146] '你的帐号目前无法登录，请稍后再试。（原因：9005）'
1235 @0x0000e9d1 push_shared_string [45] 'send_dialog_ok'
1236 @0x0000e9d5 push_local_value [0] 
1237 @0x0000e9d9 call_in_other_fun_ex [4, 1] 
1238 @0x0000e9dd pop_stack [] 
1239 @0x0000e9e1 push_local_value [0] 
1240 @0x0000e9e5 call_in_efun [361, 1] destruct_object
1241 @0x0000e9e9 pop_stack [] 
1242 @0x0000e9ed return_from_fun [] 
1243 @0x0000e9f1 null_action [] 
1244 @0x0000e9f5 push_local_value [-8] 
1245 @0x0000e9f9 push_shared_string [94] 'service'
1246 @0x0000e9fd query_member [0] 
1247 @0x0000ea01 push_number(S) [64] 
1248 @0x0000ea05 and [] 
1249 @0x0000ea09 not_value [] 
1250 @0x0000ea0d if [30, 2] 
1251 @0x0000ea11 push_obj_var [8] 
1252 @0x0000ea15 if [18, 2] 
1253 @0x0000ea19 push_local_value [0] 
1254 @0x0000ea1d is_type [4] 
1255 @0x0000ea21 if [6, 1] 
1256 @0x0000ea25 pop_stack [] 
1257 @0x0000ea29 push_shared_string [27] 'login_info/account'
1258 @0x0000ea2d push_shared_string [24] 'query_temp'
1259 @0x0000ea31 push_local_value [0] 
1260 @0x0000ea35 call_in_other_fun_ex [4, 1] 
1261 @0x0000ea39 is_type [4] 
1262 @0x0000ea3d if [8, 2] 
1263 @0x0000ea41 push_shared_string [147] '你的帐号目前不能在这个区激活。（原因：9003）'
1264 @0x0000ea45 push_shared_string [91] 'error'
1265 @0x0000ea49 push_local_value [0] 
1266 @0x0000ea4d call_in_user_fun [698, -127] get_login_info_file
1267 @0x0000ea51 push_shared_string [25] 'log_login_info'
1268 @0x0000ea55 push_shared_string [30] '/daemons/logd.c'
1269 @0x0000ea59 call_in_other_fun_ex [4, 3] 
1270 @0x0000ea5d pop_stack [] 
1271 @0x0000ea61 push_shared_string [147] '你的帐号目前不能在这个区激活。（原因：9003）'
1272 @0x0000ea65 push_shared_string [45] 'send_dialog_ok'
1273 @0x0000ea69 push_local_value [0] 
1274 @0x0000ea6d call_in_other_fun_ex [4, 1] 
1275 @0x0000ea71 pop_stack [] 
1276 @0x0000ea75 push_local_value [0] 
1277 @0x0000ea79 call_in_efun [361, 1] destruct_object
1278 @0x0000ea7d pop_stack [] 
1279 @0x0000ea81 return_from_fun [] 
1280 @0x0000ea85 null_action [] 
1281 @0x0000ea89 push_number(S) [1] 
1282 @0x0000ea8d push_shared_string [148] 'as_create'
1283 @0x0000ea91 push_shared_string [65] 'lite_set_temp'
1284 @0x0000ea95 push_local_value [0] 
1285 @0x0000ea99 call_in_other_fun_ex [4, 2] 
1286 @0x0000ea9d pop_stack [] 
1287 @0x0000eaa1 null_action [] 
1288 @0x0000eaa5 push_shared_string [42] 'create_time'
1289 @0x0000eaa9 push_shared_string [75] 'lite_query'
1290 @0x0000eaad push_local_value [0] 
1291 @0x0000eab1 call_in_other_fun_ex [4, 1] 
1292 @0x0000eab5 not_value [] 
1293 @0x0000eab9 if [28, 2] 
1294 @0x0000eabd push_local_value [0] 
1295 @0x0000eac1 is_type [4] 
1296 @0x0000eac5 if [6, 1] 
1297 @0x0000eac9 pop_stack [] 
1298 @0x0000eacd push_shared_string [27] 'login_info/account'
1299 @0x0000ead1 push_shared_string [24] 'query_temp'
1300 @0x0000ead5 push_local_value [0] 
1301 @0x0000ead9 call_in_other_fun_ex [4, 1] 
1302 @0x0000eadd is_type [4] 
1303 @0x0000eae1 if [8, 2] 
1304 @0x0000eae5 push_shared_string [149] '该帐号的登录缺少create_time字段，这是错误的数据(2)\n'
1305 @0x0000eae9 push_shared_string [91] 'error'
1306 @0x0000eaed push_local_value [0] 
1307 @0x0000eaf1 call_in_user_fun [698, -127] get_login_info_file
1308 @0x0000eaf5 push_shared_string [25] 'log_login_info'
1309 @0x0000eaf9 push_shared_string [30] '/daemons/logd.c'
1310 @0x0000eafd call_in_other_fun_ex [4, 3] 
1311 @0x0000eb01 pop_stack [] 
1312 @0x0000eb05 push_shared_string [150] '该帐号的登录信息异常，请重新登录或联系管理员。（原因：9006）'
1313 @0x0000eb09 push_shared_string [45] 'send_dialog_ok'
1314 @0x0000eb0d push_local_value [0] 
1315 @0x0000eb11 call_in_other_fun_ex [4, 1] 
1316 @0x0000eb15 pop_stack [] 
1317 @0x0000eb19 push_local_value [0] 
1318 @0x0000eb1d call_in_efun [361, 1] destruct_object
1319 @0x0000eb21 pop_stack [] 
1320 @0x0000eb25 return_from_fun [] 
1321 @0x0000eb29 null_action [] 
1322 @0x0000eb2d push_number(S) [1] 
1323 @0x0000eb31 push_shared_string [10] 'enabled'
1324 @0x0000eb35 push_shared_string [65] 'lite_set_temp'
1325 @0x0000eb39 push_local_value [0] 
1326 @0x0000eb3d call_in_other_fun_ex [4, 2] 
1327 @0x0000eb41 pop_stack [] 
1328 @0x0000eb45 push_local_value [0] 
1329 @0x0000eb49 push_shared_string [77] 'main'
1330 @0x0000eb4d push_shared_string [151] 'ask_fields'
1331 @0x0000eb51 push_shared_string [76] 'query_entry'
1332 @0x0000eb55 push_shared_string [152] '/gs/daemons/commandd.c'
1333 @0x0000eb59 call_in_other_fun_ex [4, 1] 
1334 @0x0000eb5d call_in_other_fun_ex [4, 1] 
1335 @0x0000eb61 pop_stack [] 
1336 @0x0000eb65 push_local_value [-8] 
1337 @0x0000eb69 push_shared_string [50] 'privilege'
1338 @0x0000eb6d query_member [0] 
1339 @0x0000eb71 push_shared_string [78] 'set_privilege'
1340 @0x0000eb75 push_local_value [0] 
1341 @0x0000eb79 call_in_other_fun_ex [4, 1] 
1342 @0x0000eb7d not_value [] 
1343 @0x0000eb81 if [30, 2] 
1344 @0x0000eb85 push_obj_var [8] 
1345 @0x0000eb89 if [18, 2] 
1346 @0x0000eb8d push_local_value [0] 
1347 @0x0000eb91 is_type [4] 
1348 @0x0000eb95 if [6, 1] 
1349 @0x0000eb99 pop_stack [] 
1350 @0x0000eb9d push_shared_string [27] 'login_info/account'
1351 @0x0000eba1 push_shared_string [24] 'query_temp'
1352 @0x0000eba5 push_local_value [0] 
1353 @0x0000eba9 call_in_other_fun_ex [4, 1] 
1354 @0x0000ebad is_type [4] 
1355 @0x0000ebb1 if [8, 2] 
1356 @0x0000ebb5 push_shared_string [153] '登录时发生了错误，请和GM联系。（原因：9035）'
1357 @0x0000ebb9 push_shared_string [91] 'error'
1358 @0x0000ebbd push_local_value [0] 
1359 @0x0000ebc1 call_in_user_fun [698, -127] get_login_info_file
1360 @0x0000ebc5 push_shared_string [25] 'log_login_info'
1361 @0x0000ebc9 push_shared_string [30] '/daemons/logd.c'
1362 @0x0000ebcd call_in_other_fun_ex [4, 3] 
1363 @0x0000ebd1 pop_stack [] 
1364 @0x0000ebd5 push_shared_string [153] '登录时发生了错误，请和GM联系。（原因：9035）'
1365 @0x0000ebd9 push_shared_string [45] 'send_dialog_ok'
1366 @0x0000ebdd push_local_value [0] 
1367 @0x0000ebe1 call_in_other_fun_ex [4, 1] 
1368 @0x0000ebe5 pop_stack [] 
1369 @0x0000ebe9 push_local_value [0] 
1370 @0x0000ebed call_in_efun [361, 1] destruct_object
1371 @0x0000ebf1 pop_stack [] 
1372 @0x0000ebf5 return_from_fun [] 
1373 @0x0000ebf9 null_action [] 
1374 @0x0000ebfd push_number(S) [3] 
1375 @0x0000ec01 call_in_user_fun [692, -127] check_server_status
1376 @0x0000ec05 not_value [] 
1377 @0x0000ec09 if [5, 0] 
1378 @0x0000ec0d pop_stack [] 
1379 @0x0000ec11 push_shared_string [79] 'valid_grant'
1380 @0x0000ec15 push_local_value [0] 
1381 @0x0000ec19 call_in_other_fun_ex [4, 0] 
1382 @0x0000ec1d not_value [] 
1383 @0x0000ec21 if [30, 2] 
1384 @0x0000ec25 push_obj_var [8] 
1385 @0x0000ec29 if [18, 2] 
1386 @0x0000ec2d push_local_value [0] 
1387 @0x0000ec31 is_type [4] 
1388 @0x0000ec35 if [6, 1] 
1389 @0x0000ec39 pop_stack [] 
1390 @0x0000ec3d push_shared_string [27] 'login_info/account'
1391 @0x0000ec41 push_shared_string [24] 'query_temp'
1392 @0x0000ec45 push_local_value [0] 
1393 @0x0000ec49 call_in_other_fun_ex [4, 1] 
1394 @0x0000ec4d is_type [4] 
1395 @0x0000ec51 if [8, 2] 
1396 @0x0000ec55 push_shared_string [154] '游戏服务器正在维护中，请稍候再尝试登录。（原因：9007）'
1397 @0x0000ec59 push_shared_string [91] 'error'
1398 @0x0000ec5d push_local_value [0] 
1399 @0x0000ec61 call_in_user_fun [698, -127] get_login_info_file
1400 @0x0000ec65 push_shared_string [25] 'log_login_info'
1401 @0x0000ec69 push_shared_string [30] '/daemons/logd.c'
1402 @0x0000ec6d call_in_other_fun_ex [4, 3] 
1403 @0x0000ec71 pop_stack [] 
1404 @0x0000ec75 push_shared_string [154] '游戏服务器正在维护中，请稍候再尝试登录。（原因：9007）'
1405 @0x0000ec79 push_shared_string [45] 'send_dialog_ok'
1406 @0x0000ec7d push_local_value [0] 
1407 @0x0000ec81 call_in_other_fun_ex [4, 1] 
1408 @0x0000ec85 pop_stack [] 
1409 @0x0000ec89 push_local_value [0] 
1410 @0x0000ec8d call_in_efun [361, 1] destruct_object
1411 @0x0000ec91 pop_stack [] 
1412 @0x0000ec95 return_from_fun [] 
1413 @0x0000ec99 null_action [] 
1414 @0x0000ec9d push_local_value [-8] 
1415 @0x0000eca1 push_shared_string [155] 'safe_status'
1416 @0x0000eca5 query_member [0] 
1417 @0x0000eca9 push_shared_string [155] 'safe_status'
1418 @0x0000ecad push_shared_string [38] 'lite_set'
1419 @0x0000ecb1 push_local_value [0] 
1420 @0x0000ecb5 call_in_other_fun_ex [4, 2] 
1421 @0x0000ecb9 pop_stack [] 
1422 @0x0000ecbd push_local_value [-8] 
1423 @0x0000ecc1 push_shared_string [136] 'protects'
1424 @0x0000ecc5 query_member [0] 
1425 @0x0000ecc9 push_shared_string [136] 'protects'
1426 @0x0000eccd push_shared_string [65] 'lite_set_temp'
1427 @0x0000ecd1 push_local_value [0] 
1428 @0x0000ecd5 call_in_other_fun_ex [4, 2] 
1429 @0x0000ecd9 pop_stack [] 
1430 @0x0000ecdd push_local_value [-8] 
1431 @0x0000ece1 push_shared_string [156] 'protect_range'
1432 @0x0000ece5 query_member [0] 
1433 @0x0000ece9 push_shared_string [156] 'protect_range'
1434 @0x0000eced push_shared_string [65] 'lite_set_temp'
1435 @0x0000ecf1 push_local_value [0] 
1436 @0x0000ecf5 call_in_other_fun_ex [4, 2] 
1437 @0x0000ecf9 pop_stack [] 
1438 @0x0000ecfd push_local_value [-8] 
1439 @0x0000ed01 push_shared_string [137] 'is_bind'
1440 @0x0000ed05 query_member [0] 
1441 @0x0000ed09 push_shared_string [137] 'is_bind'
1442 @0x0000ed0d push_shared_string [65] 'lite_set_temp'
1443 @0x0000ed11 push_local_value [0] 
1444 @0x0000ed15 call_in_other_fun_ex [4, 2] 
1445 @0x0000ed19 pop_stack [] 
1446 @0x0000ed1d push_local_value [-8] 
1447 @0x0000ed21 push_shared_string [157] 'passpod_type'
1448 @0x0000ed25 query_member [0] 
1449 @0x0000ed29 push_shared_string [157] 'passpod_type'
1450 @0x0000ed2d push_shared_string [65] 'lite_set_temp'
1451 @0x0000ed31 push_local_value [0] 
1452 @0x0000ed35 call_in_other_fun_ex [4, 2] 
1453 @0x0000ed39 pop_stack [] 
1454 @0x0000ed3d push_local_value [-8] 
1455 @0x0000ed41 push_shared_string [158] 'auth_result'
1456 @0x0000ed45 query_member [0] 
1457 @0x0000ed49 push_shared_string [158] 'auth_result'
1458 @0x0000ed4d push_shared_string [65] 'lite_set_temp'
1459 @0x0000ed51 push_local_value [0] 
1460 @0x0000ed55 call_in_other_fun_ex [4, 2] 
1461 @0x0000ed59 pop_stack [] 
1462 @0x0000ed5d push_local_value [-3] 
1463 @0x0000ed61 push_shared_string [111] 'switch_server_flag'
1464 @0x0000ed65 push_shared_string [65] 'lite_set_temp'
1465 @0x0000ed69 push_local_value [0] 
1466 @0x0000ed6d call_in_other_fun_ex [4, 2] 
1467 @0x0000ed71 pop_stack [] 
1468 @0x0000ed75 push_local_value [-8] 
1469 @0x0000ed79 push_shared_string [138] 'all_protects'
1470 @0x0000ed7d query_member [0] 
1471 @0x0000ed81 push_shared_string [138] 'all_protects'
1472 @0x0000ed85 push_shared_string [65] 'lite_set_temp'
1473 @0x0000ed89 push_local_value [0] 
1474 @0x0000ed8d call_in_other_fun_ex [4, 2] 
1475 @0x0000ed91 pop_stack [] 
1476 @0x0000ed95 push_local_value [-8] 
1477 @0x0000ed99 push_shared_string [159] 'vip_sms_info'
1478 @0x0000ed9d query_member [0] 
1479 @0x0000eda1 push_shared_string [159] 'vip_sms_info'
1480 @0x0000eda5 push_shared_string [65] 'lite_set_temp'
1481 @0x0000eda9 push_local_value [0] 
1482 @0x0000edad call_in_other_fun_ex [4, 2] 
1483 @0x0000edb1 pop_stack [] 
1484 @0x0000edb5 push_local_value [-8] 
1485 @0x0000edb9 push_shared_string [160] 'mp_device'
1486 @0x0000edbd query_member [0] 
1487 @0x0000edc1 push_shared_string [160] 'mp_device'
1488 @0x0000edc5 push_shared_string [65] 'lite_set_temp'
1489 @0x0000edc9 push_local_value [0] 
1490 @0x0000edcd call_in_other_fun_ex [4, 2] 
1491 @0x0000edd1 pop_stack [] 
1492 @0x0000edd5 push_local_value [-8] 
1493 @0x0000edd9 push_shared_string [161] 'temp_block_info'
1494 @0x0000eddd query_member [0] 
1495 @0x0000ede1 push_shared_string [161] 'temp_block_info'
1496 @0x0000ede5 push_shared_string [65] 'lite_set_temp'
1497 @0x0000ede9 push_local_value [0] 
1498 @0x0000eded call_in_other_fun_ex [4, 2] 
1499 @0x0000edf1 pop_stack [] 
1500 @0x0000edf5 push_local_value [-8] 
1501 @0x0000edf9 push_shared_string [162] 'char_name'
1502 @0x0000edfd query_member [0] 
1503 @0x0000ee01 if [42, 2] 
1504 @0x0000ee05 push_local_value [-8] 
1505 @0x0000ee09 push_shared_string [162] 'char_name'
1506 @0x0000ee0d query_member [0] 
1507 @0x0000ee11 call_in_efun [100, 1] restore_value
1508 @0x0000ee15 push_local_vptr [-10] 
1509 @0x0000ee19 assign_value [] 
1510 @0x0000ee1d pop_stack [] 
1511 @0x0000ee21 push_local_value [1] 
1512 @0x0000ee25 push_shared_string [26] 'account'
1513 @0x0000ee29 query_member [0] 
1514 @0x0000ee2d is_type [4] 
1515 @0x0000ee31 if [8, 1] 
1516 @0x0000ee35 pop_stack [] 
1517 @0x0000ee39 push_shared_string [27] 'login_info/account'
1518 @0x0000ee3d push_shared_string [24] 'query_temp'
1519 @0x0000ee41 push_local_value [1] 
1520 @0x0000ee45 push_shared_string [26] 'account'
1521 @0x0000ee49 query_member [0] 
1522 @0x0000ee4d call_in_other_fun_ex [4, 1] 
1523 @0x0000ee51 is_type [4] 
1524 @0x0000ee55 if [14, 2] 
1525 @0x0000ee59 push_shared_string [163] '直接登录角色：'
1526 @0x0000ee5d push_local_value [-10] 
1527 @0x0000ee61 add [] 
1528 @0x0000ee65 push_shared_string [164] '\n'
1529 @0x0000ee69 add [] 
1530 @0x0000ee6d push_shared_string [143] 'char'
1531 @0x0000ee71 push_local_value [1] 
1532 @0x0000ee75 push_shared_string [26] 'account'
1533 @0x0000ee79 query_member [0] 
1534 @0x0000ee7d call_in_user_fun [698, -127] get_login_info_file
1535 @0x0000ee81 push_shared_string [25] 'log_login_info'
1536 @0x0000ee85 push_shared_string [30] '/daemons/logd.c'
1537 @0x0000ee89 call_in_other_fun_ex [4, 3] 
1538 @0x0000ee8d pop_stack [] 
1539 @0x0000ee91 push_number(S) [0] 
1540 @0x0000ee95 push_local_value [-10] 
1541 @0x0000ee99 push_local_value [0] 
1542 @0x0000ee9d call_in_user_fun [48, 3] select_char
1543 @0x0000eea1 pop_stack [] 
1544 @0x0000eea5 return_from_fun [] 
1545 @0x0000eea9 null_action [] 
1546 @0x0000eead push_local_value [1] 
1547 @0x0000eeb1 push_shared_string [26] 'account'
1548 @0x0000eeb5 query_member [0] 
1549 @0x0000eeb9 is_type [4] 
1550 @0x0000eebd if [8, 1] 
1551 @0x0000eec1 pop_stack [] 
1552 @0x0000eec5 push_shared_string [27] 'login_info/account'
1553 @0x0000eec9 push_shared_string [24] 'query_temp'
1554 @0x0000eecd push_local_value [1] 
1555 @0x0000eed1 push_shared_string [26] 'account'
1556 @0x0000eed5 query_member [0] 
1557 @0x0000eed9 call_in_other_fun_ex [4, 1] 
1558 @0x0000eedd is_type [4] 
1559 @0x0000eee1 if [10, 2] 
1560 @0x0000eee5 push_shared_string [165] '显示帐号的所有角色\n'
1561 @0x0000eee9 push_shared_string [143] 'char'
1562 @0x0000eeed push_local_value [1] 
1563 @0x0000eef1 push_shared_string [26] 'account'
1564 @0x0000eef5 query_member [0] 
1565 @0x0000eef9 call_in_user_fun [698, -127] get_login_info_file
1566 @0x0000eefd push_shared_string [25] 'log_login_info'
1567 @0x0000ef01 push_shared_string [30] '/daemons/logd.c'
1568 @0x0000ef05 call_in_other_fun_ex [4, 3] 
1569 @0x0000ef09 pop_stack [] 
1570 @0x0000ef0d push_local_value [0] 
1571 @0x0000ef11 call_in_user_fun [51, 1] show_char_list
1572 @0x0000ef15 pop_stack [] 
1573 @0x0000ef19 push_local_value [0] 
1574 @0x0000ef1d is_type [10] 
1575 @0x0000ef21 if [6, 0] 
1576 @0x0000ef25 pop_stack [] 
1577 @0x0000ef29 push_local_value [0] 
1578 @0x0000ef2d call_in_user_fun [64, 1] get_protect_tip
1579 @0x0000ef31 push_local_vptr [-5] 
1580 @0x0000ef35 assign_value [] 
1581 @0x0000ef39 is_type [4] 
1582 @0x0000ef3d if [5, 2] 
1583 @0x0000ef41 push_local_value [-5] 
1584 @0x0000ef45 push_shared_string [45] 'send_dialog_ok'
1585 @0x0000ef49 push_local_value [0] 
1586 @0x0000ef4d call_in_other_fun_ex [4, 1] 
1587 @0x0000ef51 pop_stack [] 
1588 @0x0000ef55 push_local_value [1] 
1589 @0x0000ef59 push_shared_string [26] 'account'
1590 @0x0000ef5d query_member [0] 
1591 @0x0000ef61 is_type [4] 
1592 @0x0000ef65 if [8, 1] 
1593 @0x0000ef69 pop_stack [] 
1594 @0x0000ef6d push_shared_string [27] 'login_info/account'
1595 @0x0000ef71 push_shared_string [24] 'query_temp'
1596 @0x0000ef75 push_local_value [1] 
1597 @0x0000ef79 push_shared_string [26] 'account'
1598 @0x0000ef7d query_member [0] 
1599 @0x0000ef81 call_in_other_fun_ex [4, 1] 
1600 @0x0000ef85 is_type [4] 
1601 @0x0000ef89 if [10, 2] 
1602 @0x0000ef8d push_shared_string [166] '进入角色选择\n'
1603 @0x0000ef91 push_shared_string [143] 'char'
1604 @0x0000ef95 push_local_value [1] 
1605 @0x0000ef99 push_shared_string [26] 'account'
1606 @0x0000ef9d query_member [0] 
1607 @0x0000efa1 call_in_user_fun [698, -127] get_login_info_file
1608 @0x0000efa5 push_shared_string [25] 'log_login_info'
1609 @0x0000efa9 push_shared_string [30] '/daemons/logd.c'
1610 @0x0000efad call_in_other_fun_ex [4, 3] 
1611 @0x0000efb1 pop_stack [] 
1612 @0x0000efb5 null_action [] 
1613 @0x0000efb9 return_from_fun [] 
```

<a id="gs-gs-daemons-logind-o-_do_login"></a>

## gs/pack_data/lib_gs32.pak/gs/daemons/logind.o :: _do_login

成员 SHA-256：`51a071b4bf5411a7fdf5bddf6ae8b77a33d114ac0f530bd3822960dbb3523f79`；函数记录偏移：`0x10911`；函数编号：`78`。

```text
0000 @0x00010acf check_args_num [0] 
0001 @0x00010ad3 enter_function [] 
0002 @0x00010ad7 new_thread_catch [11] 
0003 @0x00010adb push_local_value [1] 
0004 @0x00010adf push_shared_string [26] 'account'
0005 @0x00010ae3 query_member [0] 
0006 @0x00010ae7 push_local_value [0] 
0007 @0x00010aeb call_in_user_fun [75, 2] try_recover_data
0008 @0x00010aef push_local_vptr [-1] 
0009 @0x00010af3 assign_value [] 
0010 @0x00010af7 pop_stack [] 
0011 @0x00010afb push_number(S) [0] 
0012 @0x00010aff end_thread_catch [] 
0013 @0x00010b03 goto [1] 
0014 @0x00010b07 caught_error [] 
0015 @0x00010b0b pop_stack [] 
0016 @0x00010b0f push_local_value [-1] 
0017 @0x00010b13 not_value [] 
0018 @0x00010b17 if [1, 2] 
0019 @0x00010b1b goto [156] 
0020 @0x00010b1f push_shared_string [44] 'get_client'
0021 @0x00010b23 push_local_value [0] 
0022 @0x00010b27 call_in_other_fun_ex [4, 0] 
0023 @0x00010b2b push_number(S) [-1] 
0024 @0x00010b2f eq_value [] 
0025 @0x00010b33 if [24, 2] 
0026 @0x00010b37 push_local_value [0] 
0027 @0x00010b3b is_type [4] 
0028 @0x00010b3f if [6, 1] 
0029 @0x00010b43 pop_stack [] 
0030 @0x00010b47 push_shared_string [27] 'login_info/account'
0031 @0x00010b4b push_shared_string [24] 'query_temp'
0032 @0x00010b4f push_local_value [0] 
0033 @0x00010b53 call_in_other_fun_ex [4, 1] 
0034 @0x00010b57 is_type [4] 
0035 @0x00010b5b if [8, 2] 
0036 @0x00010b5f push_shared_string [48] '恢复完毕以后玩家断线了\n'
0037 @0x00010b63 push_shared_string [49] 'runtime'
0038 @0x00010b67 push_local_value [0] 
0039 @0x00010b6b call_in_user_fun [698, -127] get_login_info_file
0040 @0x00010b6f push_shared_string [25] 'log_login_info'
0041 @0x00010b73 push_shared_string [30] '/daemons/logd.c'
0042 @0x00010b77 call_in_other_fun_ex [4, 3] 
0043 @0x00010b7b pop_stack [] 
0044 @0x00010b7f push_number(S) [0] 
0045 @0x00010b83 push_local_vptr [-1] 
0046 @0x00010b87 assign_value [] 
0047 @0x00010b8b pop_stack [] 
0048 @0x00010b8f goto [127] 
0049 @0x00010b93 null_action [] 
0050 @0x00010b97 push_global_var [110] 
0051 @0x00010b9b sizeof [] 
0052 @0x00010b9f push_obj_var [6] 
0053 @0x00010ba3 ge_value [] 
0054 @0x00010ba7 if [6, 0] 
0055 @0x00010bab pop_stack [] 
0056 @0x00010baf push_local_value [2] 
0057 @0x00010bb3 push_shared_string [50] 'privilege'
0058 @0x00010bb7 query_member [0] 
0059 @0x00010bbb push_number(S) [0] 
0060 @0x00010bbf eq_value [] 
0061 @0x00010bc3 if [5, 0] 
0062 @0x00010bc7 pop_stack [] 
0063 @0x00010bcb push_local_value [2] 
0064 @0x00010bcf push_shared_string [51] 'accepted'
0065 @0x00010bd3 query_member [0] 
0066 @0x00010bd7 not_value [] 
0067 @0x00010bdb if [42, 2] 
0068 @0x00010bdf push_local_value [1] 
0069 @0x00010be3 push_shared_string [26] 'account'
0070 @0x00010be7 query_member [0] 
0071 @0x00010beb is_type [4] 
0072 @0x00010bef if [8, 1] 
0073 @0x00010bf3 pop_stack [] 
0074 @0x00010bf7 push_shared_string [27] 'login_info/account'
0075 @0x00010bfb push_shared_string [24] 'query_temp'
0076 @0x00010bff push_local_value [1] 
0077 @0x00010c03 push_shared_string [26] 'account'
0078 @0x00010c07 query_member [0] 
0079 @0x00010c0b call_in_other_fun_ex [4, 1] 
0080 @0x00010c0f is_type [4] 
0081 @0x00010c13 if [10, 2] 
0082 @0x00010c17 push_shared_string [52] '使用者太多，拒绝登录\n'
0083 @0x00010c1b push_shared_string [53] 'reject'
0084 @0x00010c1f push_local_value [1] 
0085 @0x00010c23 push_shared_string [26] 'account'
0086 @0x00010c27 query_member [0] 
0087 @0x00010c2b call_in_user_fun [698, -127] get_login_info_file
0088 @0x00010c2f push_shared_string [25] 'log_login_info'
0089 @0x00010c33 push_shared_string [30] '/daemons/logd.c'
0090 @0x00010c37 call_in_other_fun_ex [4, 3] 
0091 @0x00010c3b pop_stack [] 
0092 @0x00010c3f push_shared_string [54] '服务器人数已满，请更换其他服务器或稍后再登录。（原因：9001）'
0093 @0x00010c43 push_shared_string [45] 'send_dialog_ok'
0094 @0x00010c47 push_local_value [0] 
0095 @0x00010c4b call_in_other_fun_ex [4, 1] 
0096 @0x00010c4f pop_stack [] 
0097 @0x00010c53 push_number(S) [3] 
0098 @0x00010c57 push_local_value [0] 
0099 @0x00010c5b call_in_user_fun [70, 2] set_login_state
0100 @0x00010c5f pop_stack [] 
0101 @0x00010c63 push_local_value [0] 
0102 @0x00010c67 call_in_efun [361, 1] destruct_object
0103 @0x00010c6b pop_stack [] 
0104 @0x00010c6f push_number(S) [0] 
0105 @0x00010c73 push_local_vptr [-1] 
0106 @0x00010c77 assign_value [] 
0107 @0x00010c7b pop_stack [] 
0108 @0x00010c7f goto [67] 
0109 @0x00010c83 null_action [] 
0110 @0x00010c87 push_shared_string [51] 'accepted'
0111 @0x00010c8b push_local_value [2] 
0112 @0x00010c8f call_in_efun [80, 2] map_delete
0113 @0x00010c93 pop_stack [] 
0114 @0x00010c97 push_local_value [0] 
0115 @0x00010c9b is_type [4] 
0116 @0x00010c9f if [6, 1] 
0117 @0x00010ca3 pop_stack [] 
0118 @0x00010ca7 push_shared_string [27] 'login_info/account'
0119 @0x00010cab push_shared_string [24] 'query_temp'
0120 @0x00010caf push_local_value [0] 
0121 @0x00010cb3 call_in_other_fun_ex [4, 1] 
0122 @0x00010cb7 is_type [4] 
0123 @0x00010cbb if [12, 2] 
0124 @0x00010cbf push_shared_string [46] 'get_id'
0125 @0x00010cc3 push_local_value [0] 
0126 @0x00010cc7 call_in_other_fun_ex [4, 0] 
0127 @0x00010ccb push_shared_string [55] '保存帐号(%d)的运行时登录信息\n'
0128 @0x00010ccf call_in_efun [121, 2] sprintf
0129 @0x00010cd3 push_shared_string [49] 'runtime'
0130 @0x00010cd7 push_local_value [0] 
0131 @0x00010cdb call_in_user_fun [698, -127] get_login_info_file
0132 @0x00010cdf push_shared_string [25] 'log_login_info'
0133 @0x00010ce3 push_shared_string [30] '/daemons/logd.c'
0134 @0x00010ce7 call_in_other_fun_ex [4, 3] 
0135 @0x00010ceb pop_stack [] 
0136 @0x00010cef push_shared_string [47] 'save_account_runtime_info'
0137 @0x00010cf3 push_local_value [0] 
0138 @0x00010cf7 call_in_other_fun_ex [4, 0] 
0139 @0x00010cfb not_value [] 
0140 @0x00010cff if [32, 2] 
0141 @0x00010d03 push_local_value [0] 
0142 @0x00010d07 is_type [4] 
0143 @0x00010d0b if [6, 1] 
0144 @0x00010d0f pop_stack [] 
0145 @0x00010d13 push_shared_string [27] 'login_info/account'
0146 @0x00010d17 push_shared_string [24] 'query_temp'
0147 @0x00010d1b push_local_value [0] 
0148 @0x00010d1f call_in_other_fun_ex [4, 1] 
0149 @0x00010d23 is_type [4] 
0150 @0x00010d27 if [8, 2] 
0151 @0x00010d2b push_shared_string [56] '保存帐号的运行时登录信息失败\n'
0152 @0x00010d2f push_shared_string [49] 'runtime'
0153 @0x00010d33 push_local_value [0] 
0154 @0x00010d37 call_in_user_fun [698, -127] get_login_info_file
0155 @0x00010d3b push_shared_string [25] 'log_login_info'
0156 @0x00010d3f push_shared_string [30] '/daemons/logd.c'
0157 @0x00010d43 call_in_other_fun_ex [4, 3] 
0158 @0x00010d47 pop_stack [] 
0159 @0x00010d4b push_shared_string [57] '无法载入角色数据，请稍候再登录。（原因：9002）'
0160 @0x00010d4f push_shared_string [45] 'send_dialog_ok'
0161 @0x00010d53 push_local_value [0] 
0162 @0x00010d57 call_in_other_fun_ex [4, 1] 
0163 @0x00010d5b pop_stack [] 
0164 @0x00010d5f push_local_value [0] 
0165 @0x00010d63 call_in_efun [361, 1] destruct_object
0166 @0x00010d67 pop_stack [] 
0167 @0x00010d6b push_number(S) [0] 
0168 @0x00010d6f push_local_vptr [-1] 
0169 @0x00010d73 assign_value [] 
0170 @0x00010d77 pop_stack [] 
0171 @0x00010d7b goto [4] 
0172 @0x00010d7f null_action [] 
0173 @0x00010d83 null_action [] 
0174 @0x00010d87 push_number(S) [0] 
0175 @0x00010d8b if [-174, 3] 
0176 @0x00010d8f push_local_value [-1] 
0177 @0x00010d93 return_from_fun [] 
0178 @0x00010d97 null_action [] 
0179 @0x00010d9b return_from_fun [] 
```

<a id="gs-gs-daemons-logind-o-new_user"></a>

## gs/pack_data/lib_gs32.pak/gs/daemons/logind.o :: new_user

成员 SHA-256：`51a071b4bf5411a7fdf5bddf6ae8b77a33d114ac0f530bd3822960dbb3523f79`；函数记录偏移：`0x1b508`；函数编号：`41`。

```text
0000 @0x0001b5d2 check_args_num [0] 
0001 @0x0001b5d6 enter_function [] 
0002 @0x0001b5da push_shared_string [40] '/gs/clone/char/login.c'
0003 @0x0001b5de call_in_efun [359, 1] clone_object
0004 @0x0001b5e2 push_local_vptr [-1] 
0005 @0x0001b5e6 assign_value [] 
0006 @0x0001b5ea pop_stack [] 
0007 @0x0001b5ee push_number(S) [0] 
0008 @0x0001b5f2 push_shared_string [41] 'register_time'
0009 @0x0001b5f6 push_shared_string [38] 'lite_set'
0010 @0x0001b5fa push_local_value [-1] 
0011 @0x0001b5fe call_in_other_fun_ex [4, 2] 
0012 @0x0001b602 pop_stack [] 
0013 @0x0001b606 call_in_efun [137, 0] time
0014 @0x0001b60a push_shared_string [42] 'create_time'
0015 @0x0001b60e push_shared_string [38] 'lite_set'
0016 @0x0001b612 push_local_value [-1] 
0017 @0x0001b616 call_in_other_fun_ex [4, 2] 
0018 @0x0001b61a pop_stack [] 
0019 @0x0001b61e push_local_value [0] 
0020 @0x0001b622 push_shared_string [43] 'client'
0021 @0x0001b626 query_member [0] 
0022 @0x0001b62a push_shared_string [39] 'set_client'
0023 @0x0001b62e push_local_value [-1] 
0024 @0x0001b632 call_in_other_fun_ex [4, 1] 
0025 @0x0001b636 pop_stack [] 
0026 @0x0001b63a push_local_value [-1] 
0027 @0x0001b63e return_from_fun [] 
0028 @0x0001b642 null_action [] 
0029 @0x0001b646 return_from_fun [] 
```

<a id="gs-__init__"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: __init__

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x1b4c1a`；函数编号：`init`。

```text
0000 @0x001b5e30 push_number(S) [1024] 
0001 @0x001b5e34 call_in_efun [0, 1] allocate_array
0002 @0x001b5e38 push_global_vptr [2] 
0003 @0x001b5e3c assign_value [] 
0004 @0x001b5e40 pop_stack [] 
0005 @0x001b5e44 push_number(S) [2] 
0006 @0x001b5e48 push_global_vptr [3] 
0007 @0x001b5e4c assign_value [] 
0008 @0x001b5e50 pop_stack [] 
0009 @0x001b5e54 push_number(S) [0] 
0010 @0x001b5e58 push_global_vptr [4] 
0011 @0x001b5e5c assign_value [] 
0012 @0x001b5e60 pop_stack [] 
0013 @0x001b5e64 push_shared_string [2554] '/xdbc/'
0014 @0x001b5e68 push_global_vptr [5] 
0015 @0x001b5e6c assign_value [] 
0016 @0x001b5e70 pop_stack [] 
0017 @0x001b5e74 create_mapping [0] 
0018 @0x001b5e78 push_global_vptr [6] 
0019 @0x001b5e7c assign_value [] 
0020 @0x001b5e80 pop_stack [] 
0021 @0x001b5e84 push_number(S) [0] 
0022 @0x001b5e88 push_global_vptr [7] 
0023 @0x001b5e8c assign_value [] 
0024 @0x001b5e90 pop_stack [] 
0025 @0x001b5e94 push_shared_string [154] '/gs/log/'
0026 @0x001b5e98 call_in_efun [336, 1] mkdir
0027 @0x001b5e9c pop_stack [] 
0028 @0x001b5ea0 push_shared_string [2555] '/gs/log/xdbc_trace'
0029 @0x001b5ea4 push_shared_string [56] 'trace'
0030 @0x001b5ea8 call_in_user_fun [583, -126] xdbc_set_output_basic_file
0031 @0x001b5eac pop_stack [] 
0032 @0x001b5eb0 call_in_user_fun [585, -128] xdbc_enable_log_trace
0033 @0x001b5eb4 pop_stack [] 
0034 @0x001b5eb8 push_number(S) [0] 
0035 @0x001b5ebc push_shared_string [2556] 'xdbc'
0036 @0x001b5ec0 call_in_efun [27, 2] enable_trace
0037 @0x001b5ec4 pop_stack [] 
0038 @0x001b5ec8 create_mapping [0] 
0039 @0x001b5ecc push_global_vptr [8] 
0040 @0x001b5ed0 assign_value [] 
0041 @0x001b5ed4 pop_stack [] 
0042 @0x001b5ed8 push_shared_string [2557] 'xdbc_connect'
0043 @0x001b5edc push_shared_string [65] 'mysql_connect'
0044 @0x001b5ee0 push_shared_string [2558] 'xdbc_disconnect'
0045 @0x001b5ee4 push_shared_string [66] 'mysql_disconnect'
0046 @0x001b5ee8 push_shared_string [2559] 'xdbc_is_connected'
0047 @0x001b5eec push_shared_string [67] 'mysql_is_connected'
0048 @0x001b5ef0 push_shared_string [2560] 'xdbc_get_error'
0049 @0x001b5ef4 push_shared_string [68] 'mysql_get_error'
0050 @0x001b5ef8 push_shared_string [2561] 'xdbc_execute'
0051 @0x001b5efc push_shared_string [69] 'mysql_execute'
0052 @0x001b5f00 push_shared_string [2562] 'xdbc_query'
0053 @0x001b5f04 push_shared_string [70] 'mysql_query'
0054 @0x001b5f08 push_shared_string [2563] 'xdbc_finalize'
0055 @0x001b5f0c push_shared_string [71] 'mysql_finalize'
0056 @0x001b5f10 push_shared_string [2564] 'xdbc_num_records'
0057 @0x001b5f14 push_shared_string [72] 'mysql_num_records'
0058 @0x001b5f18 push_shared_string [2565] 'xdbc_fetch_record'
0059 @0x001b5f1c push_shared_string [73] 'mysql_fetch_record'
0060 @0x001b5f20 push_shared_string [2566] 'xdbc_this_record'
0061 @0x001b5f24 push_shared_string [74] 'mysql_this_record'
0062 @0x001b5f28 push_shared_string [2567] 'xdbc_field_value'
0063 @0x001b5f2c push_shared_string [75] 'mysql_field_value'
0064 @0x001b5f30 push_shared_string [2568] 'xdbc_field_name'
0065 @0x001b5f34 push_shared_string [78] 'mysql_field_name'
0066 @0x001b5f38 push_shared_string [2569] 'xdbc_get_insert_id'
0067 @0x001b5f3c push_shared_string [81] 'mysql_get_insert_id'
0068 @0x001b5f40 push_shared_string [2570] 'NO_IMPLEMENTED'
0069 @0x001b5f44 push_shared_string [76] 'mysql_set_auto_commit'
0070 @0x001b5f48 push_shared_string [2570] 'NO_IMPLEMENTED'
0071 @0x001b5f4c push_shared_string [77] 'mysql_select_db'
0072 @0x001b5f50 push_shared_string [2570] 'NO_IMPLEMENTED'
0073 @0x001b5f54 push_shared_string [79] 'mysql_num_fields'
0074 @0x001b5f58 push_shared_string [2570] 'NO_IMPLEMENTED'
0075 @0x001b5f5c push_shared_string [80] 'mysql_int_field'
0076 @0x001b5f60 create_mapping [34] 
0077 @0x001b5f64 push_global_vptr [9] 
0078 @0x001b5f68 assign_value [] 
0079 @0x001b5f6c pop_stack [] 
0080 @0x001b5f70 call_in_user_fun [604, -128] init_xdbc_client
0081 @0x001b5f74 pop_stack [] 
0082 @0x001b5f78 push_shared_string [2571] 'CONFIG'
0083 @0x001b5f7c call_in_efun [22, 1] expand_macro
0084 @0x001b5f80 if [3, 2] 
0085 @0x001b5f84 push_shared_string [2571] 'CONFIG'
0086 @0x001b5f88 call_in_efun [22, 1] expand_macro
0087 @0x001b5f8c goto [1] 
0088 @0x001b5f90 push_shared_string [2572] '/gs/game_server.ini'
0089 @0x001b5f94 push_global_vptr [27] 
0090 @0x001b5f98 assign_value [] 
0091 @0x001b5f9c pop_stack [] 
0092 @0x001b5fa0 create_mapping [0] 
0093 @0x001b5fa4 push_global_vptr [28] 
0094 @0x001b5fa8 assign_value [] 
0095 @0x001b5fac pop_stack [] 
0096 @0x001b5fb0 push_shared_string [2573] '__BIN_DIR__'
0097 @0x001b5fb4 call_in_efun [22, 1] expand_macro
0098 @0x001b5fb8 call_in_efun [141, 1] trim
0099 @0x001b5fbc push_global_vptr [29] 
0100 @0x001b5fc0 assign_value [] 
0101 @0x001b5fc4 pop_stack [] 
0102 @0x001b5fc8 push_global_var [29] 
0103 @0x001b5fcc push_number(S) [1] 
0104 @0x001b5fd0 push_number(S) [2] 
0105 @0x001b5fd4 query_range [1] 
0106 @0x001b5fd8 push_global_vptr [29] 
0107 @0x001b5fdc assign_value [] 
0108 @0x001b5fe0 pop_stack [] 
0109 @0x001b5fe4 push_global_var [29] 
0110 @0x001b5fe8 push_number(S) [1] 
0111 @0x001b5fec query_member [1] 
0112 @0x001b5ff0 push_number(S) [47] 
0113 @0x001b5ff4 eq_value [] 
0114 @0x001b5ff8 if [7, 2] 
0115 @0x001b5ffc push_global_var [29] 
0116 @0x001b6000 push_number(S) [0] 
0117 @0x001b6004 push_number(S) [2] 
0118 @0x001b6008 query_range [1] 
0119 @0x001b600c push_global_vptr [29] 
0120 @0x001b6010 assign_value [] 
0121 @0x001b6014 pop_stack [] 
0122 @0x001b6018 create_array [0] 
0123 @0x001b601c push_global_vptr [30] 
0124 @0x001b6020 assign_value [] 
0125 @0x001b6024 pop_stack [] 
0126 @0x001b6028 create_array [0] 
0127 @0x001b602c push_global_vptr [31] 
0128 @0x001b6030 assign_value [] 
0129 @0x001b6034 pop_stack [] 
0130 @0x001b6038 create_array [0] 
0131 @0x001b603c push_global_vptr [32] 
0132 @0x001b6040 assign_value [] 
0133 @0x001b6044 pop_stack [] 
0134 @0x001b6048 create_array [0] 
0135 @0x001b604c push_global_vptr [33] 
0136 @0x001b6050 assign_value [] 
0137 @0x001b6054 pop_stack [] 
0138 @0x001b6058 create_array [0] 
0139 @0x001b605c push_global_vptr [34] 
0140 @0x001b6060 assign_value [] 
0141 @0x001b6064 pop_stack [] 
0142 @0x001b6068 create_array [0] 
0143 @0x001b606c push_global_vptr [35] 
0144 @0x001b6070 assign_value [] 
0145 @0x001b6074 pop_stack [] 
0146 @0x001b6078 create_array [0] 
0147 @0x001b607c push_global_vptr [36] 
0148 @0x001b6080 assign_value [] 
0149 @0x001b6084 pop_stack [] 
0150 @0x001b6088 create_array [0] 
0151 @0x001b608c push_global_vptr [37] 
0152 @0x001b6090 assign_value [] 
0153 @0x001b6094 pop_stack [] 
0154 @0x001b6098 create_array [0] 
0155 @0x001b609c push_global_vptr [38] 
0156 @0x001b60a0 assign_value [] 
0157 @0x001b60a4 pop_stack [] 
0158 @0x001b60a8 create_array [0] 
0159 @0x001b60ac push_global_vptr [39] 
0160 @0x001b60b0 assign_value [] 
0161 @0x001b60b4 pop_stack [] 
0162 @0x001b60b8 create_array [0] 
0163 @0x001b60bc push_global_vptr [40] 
0164 @0x001b60c0 assign_value [] 
0165 @0x001b60c4 pop_stack [] 
0166 @0x001b60c8 create_array [0] 
0167 @0x001b60cc push_global_vptr [41] 
0168 @0x001b60d0 assign_value [] 
0169 @0x001b60d4 pop_stack [] 
0170 @0x001b60d8 create_mapping [0] 
0171 @0x001b60dc push_global_vptr [42] 
0172 @0x001b60e0 assign_value [] 
0173 @0x001b60e4 pop_stack [] 
0174 @0x001b60e8 create_array [0] 
0175 @0x001b60ec push_global_vptr [43] 
0176 @0x001b60f0 assign_value [] 
0177 @0x001b60f4 pop_stack [] 
0178 @0x001b60f8 create_array [0] 
0179 @0x001b60fc push_global_vptr [44] 
0180 @0x001b6100 assign_value [] 
0181 @0x001b6104 pop_stack [] 
0182 @0x001b6108 create_array [0] 
0183 @0x001b610c push_global_vptr [45] 
0184 @0x001b6110 assign_value [] 
0185 @0x001b6114 pop_stack [] 
0186 @0x001b6118 create_mapping [0] 
0187 @0x001b611c push_global_vptr [46] 
0188 @0x001b6120 assign_value [] 
0189 @0x001b6124 pop_stack [] 
0190 @0x001b6128 push_number(S) [-1] 
0191 @0x001b612c push_global_vptr [47] 
0192 @0x001b6130 assign_value [] 
0193 @0x001b6134 pop_stack [] 
0194 @0x001b6138 push_number(S) [0] 
0195 @0x001b613c push_global_vptr [48] 
0196 @0x001b6140 assign_value [] 
0197 @0x001b6144 pop_stack [] 
0198 @0x001b6148 push_shared_string [160] 'update_today_zero_time'
0199 @0x001b614c push_number(S) [0] 
0200 @0x001b6150 push_number(S) [0] 
0201 @0x001b6154 call_in_efun [327, 3] set_timer
0202 @0x001b6158 pop_stack [] 
0203 @0x001b615c create_mapping [0] 
0204 @0x001b6160 push_global_vptr [49] 
0205 @0x001b6164 assign_value [] 
0206 @0x001b6168 pop_stack [] 
0207 @0x001b616c create_mapping [0] 
0208 @0x001b6170 push_global_vptr [50] 
0209 @0x001b6174 assign_value [] 
0210 @0x001b6178 pop_stack [] 
0211 @0x001b617c push_number(S) [0] 
0212 @0x001b6180 push_global_vptr [51] 
0213 @0x001b6184 assign_value [] 
0214 @0x001b6188 pop_stack [] 
0215 @0x001b618c create_mapping [0] 
0216 @0x001b6190 push_global_vptr [52] 
0217 @0x001b6194 assign_value [] 
0218 @0x001b6198 pop_stack [] 
0219 @0x001b619c push_number(S) [0] 
0220 @0x001b61a0 push_global_vptr [53] 
0221 @0x001b61a4 assign_value [] 
0222 @0x001b61a8 pop_stack [] 
0223 @0x001b61ac create_mapping [0] 
0224 @0x001b61b0 push_global_vptr [54] 
0225 @0x001b61b4 assign_value [] 
0226 @0x001b61b8 pop_stack [] 
0227 @0x001b61bc create_mapping [0] 
0228 @0x001b61c0 push_global_vptr [55] 
0229 @0x001b61c4 assign_value [] 
0230 @0x001b61c8 pop_stack [] 
0231 @0x001b61cc create_mapping [0] 
0232 @0x001b61d0 push_global_vptr [56] 
0233 @0x001b61d4 assign_value [] 
0234 @0x001b61d8 pop_stack [] 
0235 @0x001b61dc push_number(S) [0] 
0236 @0x001b61e0 push_global_vptr [57] 
0237 @0x001b61e4 assign_value [] 
0238 @0x001b61e8 pop_stack [] 
0239 @0x001b61ec push_number(S) [0] 
0240 @0x001b61f0 push_global_vptr [58] 
0241 @0x001b61f4 assign_value [] 
0242 @0x001b61f8 pop_stack [] 
0243 @0x001b61fc push_number(S) [10] 
0244 @0x001b6200 push_global_vptr [59] 
0245 @0x001b6204 assign_value [] 
0246 @0x001b6208 pop_stack [] 
0247 @0x001b620c push_number(S) [0] 
0248 @0x001b6210 push_global_vptr [60] 
0249 @0x001b6214 assign_value [] 
0250 @0x001b6218 pop_stack [] 
0251 @0x001b621c push_number(S) [0] 
0252 @0x001b6220 push_global_vptr [61] 
0253 @0x001b6224 assign_value [] 
0254 @0x001b6228 pop_stack [] 
0255 @0x001b622c create_mapping [0] 
0256 @0x001b6230 push_global_vptr [62] 
0257 @0x001b6234 assign_value [] 
0258 @0x001b6238 pop_stack [] 
0259 @0x001b623c create_mapping [0] 
0260 @0x001b6240 push_global_vptr [63] 
0261 @0x001b6244 assign_value [] 
0262 @0x001b6248 pop_stack [] 
0263 @0x001b624c push_shared_string [135] ''
0264 @0x001b6250 push_global_vptr [64] 
0265 @0x001b6254 assign_value [] 
0266 @0x001b6258 pop_stack [] 
0267 @0x001b625c push_shared_string [135] ''
0268 @0x001b6260 push_global_vptr [65] 
0269 @0x001b6264 assign_value [] 
0270 @0x001b6268 pop_stack [] 
0271 @0x001b626c push_number(S) [2] 
0272 @0x001b6270 push_global_vptr [66] 
0273 @0x001b6274 assign_value [] 
0274 @0x001b6278 pop_stack [] 
0275 @0x001b627c push_number(S) [0] 
0276 @0x001b6280 push_global_vptr [67] 
0277 @0x001b6284 assign_value [] 
0278 @0x001b6288 pop_stack [] 
0279 @0x001b628c push_number(S) [45] 
0280 @0x001b6290 push_global_vptr [68] 
0281 @0x001b6294 assign_value [] 
0282 @0x001b6298 pop_stack [] 
0283 @0x001b629c push_number(S) [-1] 
0284 @0x001b62a0 push_global_vptr [69] 
0285 @0x001b62a4 assign_value [] 
0286 @0x001b62a8 pop_stack [] 
0287 @0x001b62ac push_number(S) [1] 
0288 @0x001b62b0 push_global_vptr [70] 
0289 @0x001b62b4 assign_value [] 
0290 @0x001b62b8 pop_stack [] 
0291 @0x001b62bc create_mapping [0] 
0292 @0x001b62c0 push_global_vptr [71] 
0293 @0x001b62c4 assign_value [] 
0294 @0x001b62c8 pop_stack [] 
0295 @0x001b62cc create_mapping [0] 
0296 @0x001b62d0 push_global_vptr [72] 
0297 @0x001b62d4 assign_value [] 
0298 @0x001b62d8 pop_stack [] 
0299 @0x001b62dc call_in_user_fun [704, -128] load_ban_words
0300 @0x001b62e0 pop_stack [] 
0301 @0x001b62e4 call_in_user_fun [705, -128] load_valid_chinese
0302 @0x001b62e8 pop_stack [] 
0303 @0x001b62ec push_number(S) [0] 
0304 @0x001b62f0 push_global_vptr [73] 
0305 @0x001b62f4 assign_value [] 
0306 @0x001b62f8 pop_stack [] 
0307 @0x001b62fc push_shared_string [2574] '癸亥'
0308 @0x001b6300 push_shared_string [2575] '壬戌'
0309 @0x001b6304 push_shared_string [2576] '辛酉'
0310 @0x001b6308 push_shared_string [2577] '庚申'
0311 @0x001b630c push_shared_string [2578] '己未'
0312 @0x001b6310 push_shared_string [2579] '戊午'
0313 @0x001b6314 push_shared_string [2580] '丁巳'
0314 @0x001b6318 push_shared_string [2581] '丙辰'
0315 @0x001b631c push_shared_string [2582] '乙卯'
0316 @0x001b6320 push_shared_string [2583] '甲寅'
0317 @0x001b6324 push_shared_string [2584] '癸丑'
0318 @0x001b6328 push_shared_string [2585] '壬子'
0319 @0x001b632c push_shared_string [2586] '辛亥'
0320 @0x001b6330 push_shared_string [2587] '庚戌'
0321 @0x001b6334 push_shared_string [2588] '己酉'
0322 @0x001b6338 push_shared_string [2589] '戊申'
0323 @0x001b633c push_shared_string [2590] '丁未'
0324 @0x001b6340 push_shared_string [2591] '丙午'
0325 @0x001b6344 push_shared_string [2592] '乙巳'
0326 @0x001b6348 push_shared_string [2593] '甲辰'
0327 @0x001b634c push_shared_string [2594] '癸卯'
0328 @0x001b6350 push_shared_string [2595] '壬寅'
0329 @0x001b6354 push_shared_string [2596] '辛丑'
0330 @0x001b6358 push_shared_string [2597] '庚子'
0331 @0x001b635c push_shared_string [2598] '己亥'
0332 @0x001b6360 push_shared_string [2599] '戊戌'
0333 @0x001b6364 push_shared_string [2600] '丁酉'
0334 @0x001b6368 push_shared_string [2601] '丙申'
0335 @0x001b636c push_shared_string [2602] '乙未'
0336 @0x001b6370 push_shared_string [2603] '甲午'
0337 @0x001b6374 push_shared_string [2604] '癸巳'
0338 @0x001b6378 push_shared_string [2605] '壬辰'
0339 @0x001b637c push_shared_string [2606] '辛卯'
0340 @0x001b6380 push_shared_string [2607] '庚寅'
0341 @0x001b6384 push_shared_string [2608] '己丑'
0342 @0x001b6388 push_shared_string [2609] '戊子'
0343 @0x001b638c push_shared_string [2610] '丁亥'
0344 @0x001b6390 push_shared_string [2611] '丙戌'
0345 @0x001b6394 push_shared_string [2612] '乙酉'
0346 @0x001b6398 push_shared_string [2613] '甲申'
0347 @0x001b639c push_shared_string [2614] '癸未'
0348 @0x001b63a0 push_shared_string [2615] '壬午'
0349 @0x001b63a4 push_shared_string [2616] '辛巳'
0350 @0x001b63a8 push_shared_string [2617] '庚辰'
0351 @0x001b63ac push_shared_string [2618] '己卯'
0352 @0x001b63b0 push_shared_string [2619] '戊寅'
0353 @0x001b63b4 push_shared_string [2620] '丁丑'
0354 @0x001b63b8 push_shared_string [2621] '丙子'
0355 @0x001b63bc push_shared_string [2622] '乙亥'
0356 @0x001b63c0 push_shared_string [2623] '甲戌'
0357 @0x001b63c4 push_shared_string [2624] '癸酉'
0358 @0x001b63c8 push_shared_string [2625] '壬申'
0359 @0x001b63cc push_shared_string [2626] '辛未'
0360 @0x001b63d0 push_shared_string [2627] '庚午'
0361 @0x001b63d4 push_shared_string [2628] '己巳'
0362 @0x001b63d8 push_shared_string [2629] '戊辰'
0363 @0x001b63dc push_shared_string [2630] '丁卯'
0364 @0x001b63e0 push_shared_string [2631] '丙寅'
0365 @0x001b63e4 push_shared_string [2632] '乙丑'
0366 @0x001b63e8 push_shared_string [2633] '甲子'
0367 @0x001b63ec create_array [60] 
0368 @0x001b63f0 push_global_vptr [74] 
0369 @0x001b63f4 assign_value [] 
0370 @0x001b63f8 pop_stack [] 
0371 @0x001b63fc push_shared_string [451] '癸'
0372 @0x001b6400 push_shared_string [449] '壬'
0373 @0x001b6404 push_shared_string [447] '辛'
0374 @0x001b6408 push_shared_string [445] '庚'
0375 @0x001b640c push_shared_string [443] '己'
0376 @0x001b6410 push_shared_string [450] '戊'
0377 @0x001b6414 push_shared_string [448] '丁'
0378 @0x001b6418 push_shared_string [446] '丙'
0379 @0x001b641c push_shared_string [444] '乙'
0380 @0x001b6420 push_shared_string [442] '甲'
0381 @0x001b6424 create_array [10] 
0382 @0x001b6428 push_global_vptr [75] 
0383 @0x001b642c assign_value [] 
0384 @0x001b6430 pop_stack [] 
0385 @0x001b6434 push_shared_string [2634] '亥'
0386 @0x001b6438 push_shared_string [2635] '戌'
0387 @0x001b643c push_shared_string [2636] '酉'
0388 @0x001b6440 push_shared_string [2637] '申'
0389 @0x001b6444 push_shared_string [2638] 'δ'
0390 @0x001b6448 push_shared_string [2639] '午'
0391 @0x001b644c push_shared_string [2640] '巳'
0392 @0x001b6450 push_shared_string [2641] '辰'
0393 @0x001b6454 push_shared_string [2642] 'î'
0394 @0x001b6458 push_shared_string [2643] '寅'
0395 @0x001b645c push_shared_string [2644] '丑'
0396 @0x001b6460 push_shared_string [2645] '子'
0397 @0x001b6464 create_array [12] 
0398 @0x001b6468 push_global_vptr [76] 
0399 @0x001b646c assign_value [] 
0400 @0x001b6470 pop_stack [] 
0401 @0x001b6474 push_number(S) [45] 
0402 @0x001b6478 push_shared_string [366] 'year'
0403 @0x001b647c push_number(S) [12] 
0404 @0x001b6480 push_shared_string [2646] 'month'
0405 @0x001b6484 push_number(S) [17] 
0406 @0x001b6488 push_shared_string [453] 'day'
0407 @0x001b648c push_number(S) [28] 
0408 @0x001b6490 push_shared_string [158] 'hour'
0409 @0x001b6494 create_mapping [8] 
0410 @0x001b6498 push_global_vptr [77] 
0411 @0x001b649c assign_value [] 
0412 @0x001b64a0 pop_stack [] 
0413 @0x001b64a4 push_number(S) [31] 
0414 @0x001b64a8 push_number(S) [30] 
0415 @0x001b64ac push_number(S) [31] 
0416 @0x001b64b0 push_number(S) [30] 
0417 @0x001b64b4 push_number(S) [31] 
0418 @0x001b64b8 push_number(S) [31] 
0419 @0x001b64bc push_number(S) [30] 
0420 @0x001b64c0 push_number(S) [31] 
0421 @0x001b64c4 push_number(S) [30] 
0422 @0x001b64c8 push_number(S) [31] 
0423 @0x001b64cc push_number(S) [28] 
0424 @0x001b64d0 push_number(S) [31] 
0425 @0x001b64d4 create_array [12] 
0426 @0x001b64d8 push_global_vptr [78] 
0427 @0x001b64dc assign_value [] 
0428 @0x001b64e0 pop_stack [] 
0429 @0x001b64e4 push_number(S) [25] 
0430 @0x001b64e8 push_number(S) [24] 
0431 @0x001b64ec push_number(S) [23] 
0432 @0x001b64f0 push_number(S) [22] 
0433 @0x001b64f4 push_number(S) [21] 
0434 @0x001b64f8 push_number(S) [19] 
0435 @0x001b64fc push_number(S) [18] 
0436 @0x001b6500 push_number(S) [17] 
0437 @0x001b6504 push_number(S) [16] 
0438 @0x001b6508 push_number(S) [15] 
0439 @0x001b650c push_number(S) [14] 
0440 @0x001b6510 push_number(S) [13] 
0441 @0x001b6514 push_number(S) [12] 
0442 @0x001b6518 push_number(S) [11] 
0443 @0x001b651c push_number(S) [10] 
0444 @0x001b6520 push_number(S) [9] 
0445 @0x001b6524 push_number(S) [8] 
0446 @0x001b6528 push_number(S) [7] 
0447 @0x001b652c push_number(S) [6] 
0448 @0x001b6530 push_number(S) [5] 
0449 @0x001b6534 push_number(S) [4] 
0450 @0x001b6538 push_number(S) [3] 
0451 @0x001b653c push_number(S) [0] 
0452 @0x001b6540 create_array [23] 
0453 @0x001b6544 push_global_vptr [79] 
0454 @0x001b6548 assign_value [] 
0455 @0x001b654c pop_stack [] 
0456 @0x001b6550 push_shared_string [394] ', '
0457 @0x001b6554 push_global_var [79] 
0458 @0x001b6558 call_in_efun [54, 2] implode
0459 @0x001b655c push_global_vptr [80] 
0460 @0x001b6560 assign_value [] 
0461 @0x001b6564 pop_stack [] 
0462 @0x001b6568 push_number(S) [0] 
0463 @0x001b656c push_global_vptr [81] 
0464 @0x001b6570 assign_value [] 
0465 @0x001b6574 pop_stack [] 
0466 @0x001b6578 push_number(S) [29] 
0467 @0x001b657c push_global_vptr [82] 
0468 @0x001b6580 assign_value [] 
0469 @0x001b6584 pop_stack [] 
0470 @0x001b6588 push_number(S) [27] 
0471 @0x001b658c push_global_vptr [83] 
0472 @0x001b6590 assign_value [] 
0473 @0x001b6594 pop_stack [] 
0474 @0x001b6598 push_number(S) [1] 
0475 @0x001b659c push_global_vptr [84] 
0476 @0x001b65a0 assign_value [] 
0477 @0x001b65a4 pop_stack [] 
0478 @0x001b65a8 push_number(S) [3] 
0479 @0x001b65ac push_global_vptr [85] 
0480 @0x001b65b0 assign_value [] 
0481 @0x001b65b4 pop_stack [] 
0482 @0x001b65b8 create_mapping [0] 
0483 @0x001b65bc push_global_vptr [86] 
0484 @0x001b65c0 assign_value [] 
0485 @0x001b65c4 pop_stack [] 
0486 @0x001b65c8 push_number(S) [-1] 
0487 @0x001b65cc push_global_vptr [91] 
0488 @0x001b65d0 assign_value [] 
0489 @0x001b65d4 pop_stack [] 
0490 @0x001b65d8 create_mapping [0] 
0491 @0x001b65dc push_global_vptr [92] 
0492 @0x001b65e0 assign_value [] 
0493 @0x001b65e4 pop_stack [] 
0494 @0x001b65e8 push_number(S) [0] 
0495 @0x001b65ec push_global_vptr [93] 
0496 @0x001b65f0 assign_value [] 
0497 @0x001b65f4 pop_stack [] 
0498 @0x001b65f8 push_number(S) [0] 
0499 @0x001b65fc push_global_vptr [94] 
0500 @0x001b6600 assign_value [] 
0501 @0x001b6604 pop_stack [] 
0502 @0x001b6608 push_number(L) [256000] 
0503 @0x001b660c call_in_efun [2, 1] allocate_mapping
0504 @0x001b6610 push_global_vptr [95] 
0505 @0x001b6614 assign_value [] 
0506 @0x001b6618 pop_stack [] 
0507 @0x001b661c push_number(S) [0] 
0508 @0x001b6620 push_number(S) [2] 
0509 @0x001b6624 push_shared_string [2647] 'internal_comm'
0510 @0x001b6628 push_number(S) [1] 
0511 @0x001b662c push_shared_string [2648] 'client_user'
0512 @0x001b6630 push_number(S) [4] 
0513 @0x001b6634 push_shared_string [2649] 'from_server'
0514 @0x001b6638 create_mapping [6] 
0515 @0x001b663c call_in_efun [105, 2] set_case_check
0516 @0x001b6640 push_global_vptr [96] 
0517 @0x001b6644 assign_value [] 
0518 @0x001b6648 pop_stack [] 
0519 @0x001b664c create_mapping [0] 
0520 @0x001b6650 push_global_vptr [97] 
0521 @0x001b6654 assign_value [] 
0522 @0x001b6658 pop_stack [] 
0523 @0x001b665c push_number(S) [1] 
0524 @0x001b6660 push_shared_string [2650] 'msg_intersvr_rpc_ret'
0525 @0x001b6664 create_mapping [2] 
0526 @0x001b6668 push_number(S) [9] 
0527 @0x001b666c push_number(S) [1] 
0528 @0x001b6670 push_shared_string [2651] 'msg_treasure_trading_oper'
0529 @0x001b6674 push_number(S) [1] 
0530 @0x001b6678 push_shared_string [2652] 'cmd_cta_to_server'
0531 @0x001b667c create_mapping [4] 
0532 @0x001b6680 push_number(S) [16] 
0533 @0x001b6684 create_mapping [4] 
0534 @0x001b6688 push_global_vptr [98] 
0535 @0x001b668c assign_value [] 
0536 @0x001b6690 pop_stack [] 
0537 @0x001b6694 create_array [0] 
0538 @0x001b6698 push_global_vptr [99] 
0539 @0x001b669c assign_value [] 
0540 @0x001b66a0 pop_stack [] 
0541 @0x001b66a4 push_number(S) [0] 
0542 @0x001b66a8 push_global_vptr [100] 
0543 @0x001b66ac assign_value [] 
0544 @0x001b66b0 pop_stack [] 
0545 @0x001b66b4 create_mapping [0] 
0546 @0x001b66b8 push_global_vptr [101] 
0547 @0x001b66bc assign_value [] 
0548 @0x001b66c0 pop_stack [] 
0549 @0x001b66c4 call_in_efun [137, 0] time
0550 @0x001b66c8 push_global_vptr [102] 
0551 @0x001b66cc assign_value [] 
0552 @0x001b66d0 pop_stack [] 
0553 @0x001b66d4 push_number(S) [14] 
0554 @0x001b66d8 push_global_vptr [103] 
0555 @0x001b66dc assign_value [] 
0556 @0x001b66e0 pop_stack [] 
0557 @0x001b66e4 push_number(S) [2] 
0558 @0x001b66e8 push_global_vptr [104] 
0559 @0x001b66ec assign_value [] 
0560 @0x001b66f0 pop_stack [] 
0561 @0x001b66f4 push_number(S) [12] 
0562 @0x001b66f8 push_global_vptr [105] 
0563 @0x001b66fc assign_value [] 
0564 @0x001b6700 pop_stack [] 
0565 @0x001b6704 create_mapping [0] 
0566 @0x001b6708 push_global_vptr [106] 
0567 @0x001b670c assign_value [] 
0568 @0x001b6710 pop_stack [] 
0569 @0x001b6714 create_array [0] 
0570 @0x001b6718 push_global_vptr [107] 
0571 @0x001b671c assign_value [] 
0572 @0x001b6720 pop_stack [] 
0573 @0x001b6724 create_mapping [0] 
0574 @0x001b6728 push_global_vptr [108] 
0575 @0x001b672c assign_value [] 
0576 @0x001b6730 pop_stack [] 
0577 @0x001b6734 create_mapping [0] 
0578 @0x001b6738 push_global_vptr [109] 
0579 @0x001b673c assign_value [] 
0580 @0x001b6740 pop_stack [] 
0581 @0x001b6744 push_number(S) [0] 
0582 @0x001b6748 push_global_var [109] 
0583 @0x001b674c call_in_efun [105, 2] set_case_check
0584 @0x001b6750 pop_stack [] 
0585 @0x001b6754 create_mapping [0] 
0586 @0x001b6758 push_global_vptr [110] 
0587 @0x001b675c assign_value [] 
0588 @0x001b6760 pop_stack [] 
0589 @0x001b6764 push_number(S) [0] 
0590 @0x001b6768 push_global_var [110] 
0591 @0x001b676c call_in_efun [105, 2] set_case_check
0592 @0x001b6770 pop_stack [] 
0593 @0x001b6774 create_mapping [0] 
0594 @0x001b6778 push_global_vptr [111] 
0595 @0x001b677c assign_value [] 
0596 @0x001b6780 pop_stack [] 
0597 @0x001b6784 push_number(S) [0] 
0598 @0x001b6788 push_global_var [111] 
0599 @0x001b678c call_in_efun [105, 2] set_case_check
0600 @0x001b6790 pop_stack [] 
0601 @0x001b6794 create_mapping [0] 
0602 @0x001b6798 push_global_vptr [112] 
0603 @0x001b679c assign_value [] 
0604 @0x001b67a0 pop_stack [] 
0605 @0x001b67a4 create_array [0] 
0606 @0x001b67a8 push_global_vptr [113] 
0607 @0x001b67ac assign_value [] 
0608 @0x001b67b0 pop_stack [] 
0609 @0x001b67b4 create_array [0] 
0610 @0x001b67b8 push_global_vptr [114] 
0611 @0x001b67bc assign_value [] 
0612 @0x001b67c0 pop_stack [] 
0613 @0x001b67c4 push_number(S) [1] 
0614 @0x001b67c8 push_global_vptr [115] 
0615 @0x001b67cc assign_value [] 
0616 @0x001b67d0 pop_stack [] 
0617 @0x001b67d4 push_shared_string [859] '中国'
0618 @0x001b67d8 create_array [1] 
0619 @0x001b67dc push_global_vptr [116] 
0620 @0x001b67e0 assign_value [] 
0621 @0x001b67e4 pop_stack [] 
0622 @0x001b67e8 push_shared_string [2653] '澳门特别行政区'
0623 @0x001b67ec push_shared_string [2654] '香港特别行政区'
0624 @0x001b67f0 push_shared_string [2655] '内蒙古自治区'
0625 @0x001b67f4 push_shared_string [2656] '西藏自治区'
0626 @0x001b67f8 push_shared_string [2657] '新疆自治区'
0627 @0x001b67fc push_shared_string [2658] '宁夏自治区'
0628 @0x001b6800 push_shared_string [2659] '广西自治区'
0629 @0x001b6804 push_shared_string [2660] '黑龙江省'
0630 @0x001b6808 push_shared_string [2661] '海南省'
0631 @0x001b680c push_shared_string [2662] '山西省'
0632 @0x001b6810 push_shared_string [2663] '河南省'
0633 @0x001b6814 push_shared_string [2664] '陕西省'
0634 @0x001b6818 push_shared_string [2665] '辽宁省'
0635 @0x001b681c push_shared_string [2666] '台湾省'
0636 @0x001b6820 push_shared_string [2667] '河北省'
0637 @0x001b6824 push_shared_string [2668] '甘肃省'
0638 @0x001b6828 push_shared_string [2669] '青海省'
0639 @0x001b682c push_shared_string [2670] '浙江省'
0640 @0x001b6830 push_shared_string [2671] '江苏省'
0641 @0x001b6834 push_shared_string [2672] '安徽省'
0642 @0x001b6838 push_shared_string [2673] '云南省'
0643 @0x001b683c push_shared_string [2674] '四川省'
0644 @0x001b6840 push_shared_string [2675] '广东省'
0645 @0x001b6844 push_shared_string [2676] '福建省'
0646 @0x001b6848 push_shared_string [2677] '吉林省'
0647 @0x001b684c push_shared_string [2678] '湖南省'
0648 @0x001b6850 push_shared_string [2679] '湖北省'
0649 @0x001b6854 push_shared_string [2680] '江西省'
0650 @0x001b6858 push_shared_string [2681] '贵州省'
0651 @0x001b685c push_shared_string [2682] '山东省'
0652 @0x001b6860 push_shared_string [2683] '天津市'
0653 @0x001b6864 push_shared_string [2684] '北京市'
0654 @0x001b6868 push_shared_string [2685] '上海市'
0655 @0x001b686c push_shared_string [2686] '重庆市'
0656 @0x001b6870 create_array [34] 
0657 @0x001b6874 push_shared_string [859] '中国'
0658 @0x001b6878 create_mapping [2] 
0659 @0x001b687c push_global_vptr [117] 
0660 @0x001b6880 assign_value [] 
0661 @0x001b6884 pop_stack [] 
0662 @0x001b6888 create_mapping [0] 
0663 @0x001b688c push_global_vptr [118] 
0664 @0x001b6890 assign_value [] 
0665 @0x001b6894 pop_stack [] 
0666 @0x001b6898 push_number(S) [1] 
0667 @0x001b689c push_shared_string [2687] '金'
0668 @0x001b68a0 push_number(S) [2] 
0669 @0x001b68a4 push_shared_string [2688] 'ľ'
0670 @0x001b68a8 push_number(S) [3] 
0671 @0x001b68ac push_shared_string [2689] 'ˮ'
0672 @0x001b68b0 push_number(S) [4] 
0673 @0x001b68b4 push_shared_string [2690] '火'
0674 @0x001b68b8 push_number(S) [5] 
0675 @0x001b68bc push_shared_string [2691] '土'
0676 @0x001b68c0 create_mapping [10] 
0677 @0x001b68c4 push_global_vptr [119] 
0678 @0x001b68c8 assign_value [] 
0679 @0x001b68cc pop_stack [] 
0680 @0x001b68d0 push_shared_string [2687] '金'
0681 @0x001b68d4 push_number(S) [1] 
0682 @0x001b68d8 push_shared_string [2688] 'ľ'
0683 @0x001b68dc push_number(S) [2] 
0684 @0x001b68e0 push_shared_string [2689] 'ˮ'
0685 @0x001b68e4 push_number(S) [3] 
0686 @0x001b68e8 push_shared_string [2690] '火'
0687 @0x001b68ec push_number(S) [4] 
0688 @0x001b68f0 push_shared_string [2691] '土'
0689 @0x001b68f4 push_number(S) [5] 
0690 @0x001b68f8 create_mapping [10] 
0691 @0x001b68fc push_global_vptr [120] 
0692 @0x001b6900 assign_value [] 
0693 @0x001b6904 pop_stack [] 
0694 @0x001b6908 push_shared_string [2687] '金'
0695 @0x001b690c push_shared_string [488] '五龙山云霄洞'
0696 @0x001b6910 push_shared_string [2688] 'ľ'
0697 @0x001b6914 push_shared_string [489] '终南山玉柱洞'
0698 @0x001b6918 push_shared_string [2689] 'ˮ'
0699 @0x001b691c push_shared_string [490] '凤凰山斗阙宫'
0700 @0x001b6920 push_shared_string [2690] '火'
0701 @0x001b6924 push_shared_string [491] '乾元山金光洞'
0702 @0x001b6928 push_shared_string [2691] '土'
0703 @0x001b692c push_shared_string [492] '骷髅山白骨洞'
0704 @0x001b6930 create_mapping [10] 
0705 @0x001b6934 push_global_vptr [121] 
0706 @0x001b6938 assign_value [] 
0707 @0x001b693c pop_stack [] 
0708 @0x001b6940 push_number(S) [1] 
0709 @0x001b6944 push_shared_string [2692] 'metal'
0710 @0x001b6948 push_number(S) [2] 
0711 @0x001b694c push_shared_string [2693] 'wood'
0712 @0x001b6950 push_number(S) [3] 
0713 @0x001b6954 push_shared_string [2694] 'water'
0714 @0x001b6958 push_number(S) [4] 
0715 @0x001b695c push_shared_string [2695] 'fire'
0716 @0x001b6960 push_number(S) [5] 
0717 @0x001b6964 push_shared_string [2696] 'earth'
0718 @0x001b6968 create_mapping [10] 
0719 @0x001b696c push_global_vptr [122] 
0720 @0x001b6970 assign_value [] 
0721 @0x001b6974 pop_stack [] 
0722 @0x001b6978 push_shared_string [2692] 'metal'
0723 @0x001b697c push_number(S) [1] 
0724 @0x001b6980 push_shared_string [2693] 'wood'
0725 @0x001b6984 push_number(S) [2] 
0726 @0x001b6988 push_shared_string [2694] 'water'
0727 @0x001b698c push_number(S) [3] 
0728 @0x001b6990 push_shared_string [2695] 'fire'
0729 @0x001b6994 push_number(S) [4] 
0730 @0x001b6998 push_shared_string [2696] 'earth'
0731 @0x001b699c push_number(S) [5] 
0732 @0x001b69a0 create_mapping [10] 
0733 @0x001b69a4 push_global_vptr [123] 
0734 @0x001b69a8 assign_value [] 
0735 @0x001b69ac pop_stack [] 
0736 @0x001b69b0 push_number(S) [2] 
0737 @0x001b69b4 push_number(S) [1] 
0738 @0x001b69b8 push_number(S) [5] 
0739 @0x001b69bc push_number(S) [2] 
0740 @0x001b69c0 push_number(S) [3] 
0741 @0x001b69c4 push_number(S) [5] 
0742 @0x001b69c8 push_number(S) [4] 
0743 @0x001b69cc push_number(S) [3] 
0744 @0x001b69d0 push_number(S) [1] 
0745 @0x001b69d4 push_number(S) [4] 
0746 @0x001b69d8 create_mapping [10] 
0747 @0x001b69dc push_global_vptr [124] 
0748 @0x001b69e0 assign_value [] 
0749 @0x001b69e4 pop_stack [] 
0750 @0x001b69e8 push_number(S) [1] 
0751 @0x001b69ec push_number(S) [2] 
0752 @0x001b69f0 push_number(S) [2] 
0753 @0x001b69f4 push_number(S) [5] 
0754 @0x001b69f8 push_number(S) [5] 
0755 @0x001b69fc push_number(S) [3] 
0756 @0x001b6a00 push_number(S) [3] 
0757 @0x001b6a04 push_number(S) [4] 
0758 @0x001b6a08 push_number(S) [4] 
0759 @0x001b6a0c push_number(S) [1] 
0760 @0x001b6a10 create_mapping [10] 
0761 @0x001b6a14 push_global_vptr [125] 
0762 @0x001b6a18 assign_value [] 
0763 @0x001b6a1c pop_stack [] 
0764 @0x001b6a20 push_number(S) [3] 
0765 @0x001b6a24 push_number(S) [1] 
0766 @0x001b6a28 push_number(S) [2] 
0767 @0x001b6a2c push_number(S) [3] 
0768 @0x001b6a30 push_number(S) [4] 
0769 @0x001b6a34 push_number(S) [2] 
0770 @0x001b6a38 push_number(S) [5] 
0771 @0x001b6a3c push_number(S) [4] 
0772 @0x001b6a40 push_number(S) [1] 
0773 @0x001b6a44 push_number(S) [5] 
0774 @0x001b6a48 create_mapping [10] 
0775 @0x001b6a4c push_global_vptr [126] 
0776 @0x001b6a50 assign_value [] 
0777 @0x001b6a54 pop_stack [] 
0778 @0x001b6a58 push_number(S) [1] 
0779 @0x001b6a5c push_number(S) [3] 
0780 @0x001b6a60 push_number(S) [3] 
0781 @0x001b6a64 push_number(S) [2] 
0782 @0x001b6a68 push_number(S) [2] 
0783 @0x001b6a6c push_number(S) [4] 
0784 @0x001b6a70 push_number(S) [4] 
0785 @0x001b6a74 push_number(S) [5] 
0786 @0x001b6a78 push_number(S) [5] 
0787 @0x001b6a7c push_number(S) [1] 
0788 @0x001b6a80 create_mapping [10] 
0789 @0x001b6a84 push_global_vptr [127] 
0790 @0x001b6a88 assign_value [] 
0791 @0x001b6a8c pop_stack [] 
0792 @0x001b6a90 create_mapping [0] 
0793 @0x001b6a94 push_global_vptr [128] 
0794 @0x001b6a98 assign_value [] 
0795 @0x001b6a9c pop_stack [] 
0796 @0x001b6aa0 create_mapping [0] 
0797 @0x001b6aa4 push_global_vptr [129] 
0798 @0x001b6aa8 assign_value [] 
0799 @0x001b6aac pop_stack [] 
0800 @0x001b6ab0 create_mapping [0] 
0801 @0x001b6ab4 push_global_vptr [130] 
0802 @0x001b6ab8 assign_value [] 
0803 @0x001b6abc pop_stack [] 
0804 @0x001b6ac0 push_number(S) [1] 
0805 @0x001b6ac4 push_number(S) [17] 
0806 @0x001b6ac8 push_number(S) [6] 
0807 @0x001b6acc push_number(S) [18] 
0808 @0x001b6ad0 push_number(S) [2] 
0809 @0x001b6ad4 push_number(S) [33] 
0810 @0x001b6ad8 push_number(S) [7] 
0811 @0x001b6adc push_number(S) [34] 
0812 @0x001b6ae0 push_number(S) [3] 
0813 @0x001b6ae4 push_number(S) [49] 
0814 @0x001b6ae8 push_number(S) [8] 
0815 @0x001b6aec push_number(S) [50] 
0816 @0x001b6af0 push_number(S) [4] 
0817 @0x001b6af4 push_number(S) [65] 
0818 @0x001b6af8 push_number(S) [9] 
0819 @0x001b6afc push_number(S) [66] 
0820 @0x001b6b00 push_number(S) [5] 
0821 @0x001b6b04 push_number(S) [81] 
0822 @0x001b6b08 push_number(S) [10] 
0823 @0x001b6b0c push_number(S) [82] 
0824 @0x001b6b10 create_mapping [20] 
0825 @0x001b6b14 push_global_vptr [131] 
0826 @0x001b6b18 assign_value [] 
0827 @0x001b6b1c pop_stack [] 
0828 @0x001b6b20 push_number(S) [1] 
0829 @0x001b6b24 push_number(S) [1] 
0830 @0x001b6b28 push_number(S) [2] 
0831 @0x001b6b2c push_number(S) [2] 
0832 @0x001b6b30 push_number(S) [3] 
0833 @0x001b6b34 push_number(S) [3] 
0834 @0x001b6b38 push_number(S) [4] 
0835 @0x001b6b3c push_number(S) [4] 
0836 @0x001b6b40 push_number(S) [5] 
0837 @0x001b6b44 push_number(S) [5] 
0838 @0x001b6b48 create_mapping [10] 
0839 @0x001b6b4c push_global_vptr [132] 
0840 @0x001b6b50 assign_value [] 
0841 @0x001b6b54 pop_stack [] 
0842 @0x001b6b58 push_number(S) [1] 
0843 @0x001b6b5c push_number(S) [1] 
0844 @0x001b6b60 push_number(S) [2] 
0845 @0x001b6b64 push_number(S) [2] 
0846 @0x001b6b68 push_number(S) [3] 
0847 @0x001b6b6c push_number(S) [3] 
0848 @0x001b6b70 push_number(S) [4] 
0849 @0x001b6b74 push_number(S) [4] 
0850 @0x001b6b78 push_number(S) [5] 
0851 @0x001b6b7c push_number(S) [5] 
0852 @0x001b6b80 push_number(S) [1] 
0853 @0x001b6b84 push_number(S) [6] 
0854 @0x001b6b88 push_number(S) [2] 
0855 @0x001b6b8c push_number(S) [7] 
0856 @0x001b6b90 push_number(S) [3] 
0857 @0x001b6b94 push_number(S) [8] 
0858 @0x001b6b98 push_number(S) [4] 
0859 @0x001b6b9c push_number(S) [9] 
0860 @0x001b6ba0 push_number(S) [5] 
0861 @0x001b6ba4 push_number(S) [10] 
0862 @0x001b6ba8 create_mapping [20] 
0863 @0x001b6bac push_global_vptr [133] 
0864 @0x001b6bb0 assign_value [] 
0865 @0x001b6bb4 pop_stack [] 
0866 @0x001b6bb8 create_mapping [0] 
0867 @0x001b6bbc push_global_vptr [134] 
0868 @0x001b6bc0 assign_value [] 
0869 @0x001b6bc4 pop_stack [] 
0870 @0x001b6bc8 push_number(S) [0] 
0871 @0x001b6bcc push_global_var [134] 
0872 @0x001b6bd0 call_in_efun [105, 2] set_case_check
0873 @0x001b6bd4 pop_stack [] 
0874 @0x001b6bd8 push_number(S) [0] 
0875 @0x001b6bdc push_global_vptr [135] 
0876 @0x001b6be0 assign_value [] 
0877 @0x001b6be4 pop_stack [] 
0878 @0x001b6be8 create_mapping [0] 
0879 @0x001b6bec push_global_vptr [136] 
0880 @0x001b6bf0 assign_value [] 
0881 @0x001b6bf4 pop_stack [] 
0882 @0x001b6bf8 create_mapping [0] 
0883 @0x001b6bfc push_global_vptr [137] 
0884 @0x001b6c00 assign_value [] 
0885 @0x001b6c04 pop_stack [] 
0886 @0x001b6c08 push_number(S) [1] 
0887 @0x001b6c0c push_global_vptr [138] 
0888 @0x001b6c10 assign_value [] 
0889 @0x001b6c14 pop_stack [] 
0890 @0x001b6c18 push_shared_string [2697] 'SF_ATTACH_SKILL_TRIGGERD'
0891 @0x001b6c1c call_in_efun [139, 1] tolower
0892 @0x001b6c20 push_global_vptr [139] 
0893 @0x001b6c24 assign_value [] 
0894 @0x001b6c28 pop_stack [] 
0895 @0x001b6c2c push_global_vptr [135] 
0896 @0x001b6c30 convert_to_value [] 
0897 @0x001b6c34 push_global_vptr [136] 
0898 @0x001b6c38 push_global_var [139] 
0899 @0x001b6c3c query_member_ptr [0] 
0900 @0x001b6c40 assign_value [] 
0901 @0x001b6c44 push_global_vptr [135] 
0902 @0x001b6c48 inc_value [1] 
0903 @0x001b6c4c pop_stack [] 
0904 @0x001b6c50 push_shared_string [2698] 'SF_LOGIN_CHAR'
0905 @0x001b6c54 call_in_efun [139, 1] tolower
0906 @0x001b6c58 push_global_vptr [140] 
0907 @0x001b6c5c assign_value [] 
0908 @0x001b6c60 pop_stack [] 
0909 @0x001b6c64 push_global_vptr [135] 
0910 @0x001b6c68 convert_to_value [] 
0911 @0x001b6c6c push_global_vptr [136] 
0912 @0x001b6c70 push_global_var [140] 
0913 @0x001b6c74 query_member_ptr [0] 
0914 @0x001b6c78 assign_value [] 
0915 @0x001b6c7c push_global_vptr [135] 
0916 @0x001b6c80 inc_value [1] 
0917 @0x001b6c84 pop_stack [] 
0918 @0x001b6c88 push_shared_string [2699] 'SF_ENTER_WORLD'
0919 @0x001b6c8c call_in_efun [139, 1] tolower
0920 @0x001b6c90 push_global_vptr [141] 
0921 @0x001b6c94 assign_value [] 
0922 @0x001b6c98 pop_stack [] 
0923 @0x001b6c9c push_global_vptr [135] 
0924 @0x001b6ca0 convert_to_value [] 
0925 @0x001b6ca4 push_global_vptr [136] 
0926 @0x001b6ca8 push_global_var [141] 
0927 @0x001b6cac query_member_ptr [0] 
0928 @0x001b6cb0 assign_value [] 
0929 @0x001b6cb4 push_global_vptr [135] 
0930 @0x001b6cb8 inc_value [1] 
0931 @0x001b6cbc pop_stack [] 
0932 @0x001b6cc0 push_shared_string [2700] 'SF_LEAVE_WORLD'
0933 @0x001b6cc4 call_in_efun [139, 1] tolower
0934 @0x001b6cc8 push_global_vptr [142] 
0935 @0x001b6ccc assign_value [] 
0936 @0x001b6cd0 pop_stack [] 
0937 @0x001b6cd4 push_global_vptr [135] 
0938 @0x001b6cd8 convert_to_value [] 
0939 @0x001b6cdc push_global_vptr [136] 
0940 @0x001b6ce0 push_global_var [142] 
0941 @0x001b6ce4 query_member_ptr [0] 
0942 @0x001b6ce8 assign_value [] 
0943 @0x001b6cec push_global_vptr [135] 
0944 @0x001b6cf0 inc_value [1] 
0945 @0x001b6cf4 pop_stack [] 
0946 @0x001b6cf8 push_shared_string [2701] 'SF_INVENTORY'
0947 @0x001b6cfc call_in_efun [139, 1] tolower
0948 @0x001b6d00 push_global_vptr [143] 
0949 @0x001b6d04 assign_value [] 
0950 @0x001b6d08 pop_stack [] 
0951 @0x001b6d0c push_global_vptr [135] 
0952 @0x001b6d10 convert_to_value [] 
0953 @0x001b6d14 push_global_vptr [136] 
0954 @0x001b6d18 push_global_var [143] 
0955 @0x001b6d1c query_member_ptr [0] 
0956 @0x001b6d20 assign_value [] 
0957 @0x001b6d24 push_global_vptr [135] 
0958 @0x001b6d28 inc_value [1] 
0959 @0x001b6d2c pop_stack [] 
0960 @0x001b6d30 push_shared_string [2702] 'SF_SUB_TASK'
0961 @0x001b6d34 call_in_efun [139, 1] tolower
0962 @0x001b6d38 push_global_vptr [144] 
0963 @0x001b6d3c assign_value [] 
0964 @0x001b6d40 pop_stack [] 
0965 @0x001b6d44 push_global_vptr [135] 
0966 @0x001b6d48 convert_to_value [] 
0967 @0x001b6d4c push_global_vptr [136] 
0968 @0x001b6d50 push_global_var [144] 
0969 @0x001b6d54 query_member_ptr [0] 
0970 @0x001b6d58 assign_value [] 
0971 @0x001b6d5c push_global_vptr [135] 
0972 @0x001b6d60 inc_value [1] 
0973 @0x001b6d64 pop_stack [] 
0974 @0x001b6d68 push_shared_string [2703] 'SF_IMPROVE_LEVEL'
0975 @0x001b6d6c call_in_efun [139, 1] tolower
0976 @0x001b6d70 push_global_vptr [145] 
0977 @0x001b6d74 assign_value [] 
0978 @0x001b6d78 pop_stack [] 
0979 @0x001b6d7c push_global_vptr [135] 
0980 @0x001b6d80 convert_to_value [] 
0981 @0x001b6d84 push_global_vptr [136] 
0982 @0x001b6d88 push_global_var [145] 
0983 @0x001b6d8c query_member_ptr [0] 
0984 @0x001b6d90 assign_value [] 
0985 @0x001b6d94 push_global_vptr [135] 
0986 @0x001b6d98 inc_value [1] 
0987 @0x001b6d9c pop_stack [] 
0988 @0x001b6da0 push_shared_string [2704] 'SF_SUB_TASK_LIMIT'
0989 @0x001b6da4 call_in_efun [139, 1] tolower
0990 @0x001b6da8 push_global_vptr [146] 
0991 @0x001b6dac assign_value [] 
0992 @0x001b6db0 pop_stack [] 
0993 @0x001b6db4 push_global_vptr [135] 
0994 @0x001b6db8 convert_to_value [] 
0995 @0x001b6dbc push_global_vptr [136] 
0996 @0x001b6dc0 push_global_var [146] 
0997 @0x001b6dc4 query_member_ptr [0] 
0998 @0x001b6dc8 assign_value [] 
0999 @0x001b6dcc push_global_vptr [135] 
1000 @0x001b6dd0 inc_value [1] 
1001 @0x001b6dd4 pop_stack [] 
1002 @0x001b6dd8 push_shared_string [2705] 'SF_TOUCH_OFF_COMBAT'
1003 @0x001b6ddc call_in_efun [139, 1] tolower
1004 @0x001b6de0 push_global_vptr [147] 
1005 @0x001b6de4 assign_value [] 
1006 @0x001b6de8 pop_stack [] 
1007 @0x001b6dec push_global_vptr [135] 
1008 @0x001b6df0 convert_to_value [] 
1009 @0x001b6df4 push_global_vptr [136] 
1010 @0x001b6df8 push_global_var [147] 
1011 @0x001b6dfc query_member_ptr [0] 
1012 @0x001b6e00 assign_value [] 
1013 @0x001b6e04 push_global_vptr [135] 
1014 @0x001b6e08 inc_value [1] 
1015 @0x001b6e0c pop_stack [] 
1016 @0x001b6e10 push_shared_string [2706] 'SF_ACCEPT'
1017 @0x001b6e14 call_in_efun [139, 1] tolower
1018 @0x001b6e18 push_global_vptr [148] 
1019 @0x001b6e1c assign_value [] 
1020 @0x001b6e20 pop_stack [] 
1021 @0x001b6e24 push_global_vptr [135] 
1022 @0x001b6e28 convert_to_value [] 
1023 @0x001b6e2c push_global_vptr [136] 
1024 @0x001b6e30 push_global_var [148] 
1025 @0x001b6e34 query_member_ptr [0] 
1026 @0x001b6e38 assign_value [] 
1027 @0x001b6e3c push_global_vptr [135] 
1028 @0x001b6e40 inc_value [1] 
1029 @0x001b6e44 pop_stack [] 
1030 @0x001b6e48 push_shared_string [2707] 'SF_ACTION_TRANSLATE'
1031 @0x001b6e4c call_in_efun [139, 1] tolower
1032 @0x001b6e50 push_global_vptr [149] 
1033 @0x001b6e54 assign_value [] 
1034 @0x001b6e58 pop_stack [] 
1035 @0x001b6e5c push_global_vptr [135] 
1036 @0x001b6e60 convert_to_value [] 
1037 @0x001b6e64 push_global_vptr [136] 
1038 @0x001b6e68 push_global_var [149] 
1039 @0x001b6e6c query_member_ptr [0] 
1040 @0x001b6e70 assign_value [] 
1041 @0x001b6e74 push_global_vptr [135] 
1042 @0x001b6e78 inc_value [1] 
1043 @0x001b6e7c pop_stack [] 
1044 @0x001b6e80 push_shared_string [2708] 'SF_ALCHEMY'
1045 @0x001b6e84 call_in_efun [139, 1] tolower
1046 @0x001b6e88 push_global_vptr [150] 
1047 @0x001b6e8c assign_value [] 
1048 @0x001b6e90 pop_stack [] 
1049 @0x001b6e94 push_global_vptr [135] 
1050 @0x001b6e98 convert_to_value [] 
1051 @0x001b6e9c push_global_vptr [136] 
1052 @0x001b6ea0 push_global_var [150] 
1053 @0x001b6ea4 query_member_ptr [0] 
1054 @0x001b6ea8 assign_value [] 
1055 @0x001b6eac push_global_vptr [135] 
1056 @0x001b6eb0 inc_value [1] 
1057 @0x001b6eb4 pop_stack [] 
1058 @0x001b6eb8 push_shared_string [2709] 'SF_APPLY'
1059 @0x001b6ebc call_in_efun [139, 1] tolower
1060 @0x001b6ec0 push_global_vptr [151] 
1061 @0x001b6ec4 assign_value [] 
1062 @0x001b6ec8 pop_stack [] 
1063 @0x001b6ecc push_global_vptr [135] 
1064 @0x001b6ed0 convert_to_value [] 
1065 @0x001b6ed4 push_global_vptr [136] 
1066 @0x001b6ed8 push_global_var [151] 
1067 @0x001b6edc query_member_ptr [0] 
1068 @0x001b6ee0 assign_value [] 
1069 @0x001b6ee4 push_global_vptr [135] 
1070 @0x001b6ee8 inc_value [1] 
1071 @0x001b6eec pop_stack [] 
1072 @0x001b6ef0 push_shared_string [2710] 'SF_AUTO_DESTRUCT'
1073 @0x001b6ef4 call_in_efun [139, 1] tolower
1074 @0x001b6ef8 push_global_vptr [152] 
1075 @0x001b6efc assign_value [] 
1076 @0x001b6f00 pop_stack [] 
1077 @0x001b6f04 push_global_vptr [135] 
1078 @0x001b6f08 convert_to_value [] 
1079 @0x001b6f0c push_global_vptr [136] 
1080 @0x001b6f10 push_global_var [152] 
1081 @0x001b6f14 query_member_ptr [0] 
1082 @0x001b6f18 assign_value [] 
1083 @0x001b6f1c push_global_vptr [135] 
1084 @0x001b6f20 inc_value [1] 
1085 @0x001b6f24 pop_stack [] 
1086 @0x001b6f28 push_shared_string [2711] 'SF_BONUS'
1087 @0x001b6f2c call_in_efun [139, 1] tolower
1088 @0x001b6f30 push_global_vptr [153] 
1089 @0x001b6f34 assign_value [] 
1090 @0x001b6f38 pop_stack [] 
1091 @0x001b6f3c push_global_vptr [135] 
1092 @0x001b6f40 convert_to_value [] 
1093 @0x001b6f44 push_global_vptr [136] 
1094 @0x001b6f48 push_global_var [153] 
1095 @0x001b6f4c query_member_ptr [0] 
1096 @0x001b6f50 assign_value [] 
1097 @0x001b6f54 push_global_vptr [135] 
1098 @0x001b6f58 inc_value [1] 
1099 @0x001b6f5c pop_stack [] 
1100 @0x001b6f60 push_shared_string [2712] 'SF_CAST_ON'
1101 @0x001b6f64 call_in_efun [139, 1] tolower
1102 @0x001b6f68 push_global_vptr [154] 
1103 @0x001b6f6c assign_value [] 
1104 @0x001b6f70 pop_stack [] 
1105 @0x001b6f74 push_global_vptr [135] 
1106 @0x001b6f78 convert_to_value [] 
1107 @0x001b6f7c push_global_vptr [136] 
1108 @0x001b6f80 push_global_var [154] 
1109 @0x001b6f84 query_member_ptr [0] 
1110 @0x001b6f88 assign_value [] 
1111 @0x001b6f8c push_global_vptr [135] 
1112 @0x001b6f90 inc_value [1] 
1113 @0x001b6f94 pop_stack [] 
1114 @0x001b6f98 push_shared_string [2713] 'SF_COMBAT_FINISHED'
1115 @0x001b6f9c call_in_efun [139, 1] tolower
1116 @0x001b6fa0 push_global_vptr [155] 
1117 @0x001b6fa4 assign_value [] 
1118 @0x001b6fa8 pop_stack [] 
1119 @0x001b6fac push_global_vptr [135] 
1120 @0x001b6fb0 convert_to_value [] 
1121 @0x001b6fb4 push_global_vptr [136] 
1122 @0x001b6fb8 push_global_var [155] 
1123 @0x001b6fbc query_member_ptr [0] 
1124 @0x001b6fc0 assign_value [] 
1125 @0x001b6fc4 push_global_vptr [135] 
1126 @0x001b6fc8 inc_value [1] 
1127 @0x001b6fcc pop_stack [] 
1128 @0x001b6fd0 push_shared_string [2714] 'SF_PK_COMBAT_FINISHED'
1129 @0x001b6fd4 call_in_efun [139, 1] tolower
1130 @0x001b6fd8 push_global_vptr [156] 
1131 @0x001b6fdc assign_value [] 
1132 @0x001b6fe0 pop_stack [] 
1133 @0x001b6fe4 push_global_vptr [135] 
1134 @0x001b6fe8 convert_to_value [] 
1135 @0x001b6fec push_global_vptr [136] 
1136 @0x001b6ff0 push_global_var [156] 
1137 @0x001b6ff4 query_member_ptr [0] 
1138 @0x001b6ff8 assign_value [] 
1139 @0x001b6ffc push_global_vptr [135] 
1140 @0x001b7000 inc_value [1] 
1141 @0x001b7004 pop_stack [] 
1142 @0x001b7008 push_shared_string [2715] 'SF_CHANNEL'
1143 @0x001b700c call_in_efun [139, 1] tolower
1144 @0x001b7010 push_global_vptr [157] 
1145 @0x001b7014 assign_value [] 
1146 @0x001b7018 pop_stack [] 
1147 @0x001b701c push_global_vptr [135] 
1148 @0x001b7020 convert_to_value [] 
1149 @0x001b7024 push_global_vptr [136] 
1150 @0x001b7028 push_global_var [157] 
1151 @0x001b702c query_member_ptr [0] 
1152 @0x001b7030 assign_value [] 
1153 @0x001b7034 push_global_vptr [135] 
1154 @0x001b7038 inc_value [1] 
1155 @0x001b703c pop_stack [] 
1156 @0x001b7040 push_shared_string [2716] 'SF_USE_DUNSHU'
1157 @0x001b7044 call_in_efun [139, 1] tolower
1158 @0x001b7048 push_global_vptr [158] 
1159 @0x001b704c assign_value [] 
1160 @0x001b7050 pop_stack [] 
1161 @0x001b7054 push_global_vptr [135] 
1162 @0x001b7058 convert_to_value [] 
1163 @0x001b705c push_global_vptr [136] 
1164 @0x001b7060 push_global_var [158] 
1165 @0x001b7064 query_member_ptr [0] 
1166 @0x001b7068 assign_value [] 
1167 @0x001b706c push_global_vptr [135] 
1168 @0x001b7070 inc_value [1] 
1169 @0x001b7074 pop_stack [] 
1170 @0x001b7078 push_shared_string [2717] 'SF_CHANGE_APPELLATION'
1171 @0x001b707c call_in_efun [139, 1] tolower
1172 @0x001b7080 push_global_vptr [159] 
1173 @0x001b7084 assign_value [] 
1174 @0x001b7088 pop_stack [] 
1175 @0x001b708c push_global_vptr [135] 
1176 @0x001b7090 convert_to_value [] 
1177 @0x001b7094 push_global_vptr [136] 
1178 @0x001b7098 push_global_var [159] 
1179 @0x001b709c query_member_ptr [0] 
1180 @0x001b70a0 assign_value [] 
1181 @0x001b70a4 push_global_vptr [135] 
1182 @0x001b70a8 inc_value [1] 
1183 @0x001b70ac pop_stack [] 
1184 @0x001b70b0 push_shared_string [2718] 'SF_COMBAT_BONUS_EXP'
1185 @0x001b70b4 call_in_efun [139, 1] tolower
1186 @0x001b70b8 push_global_vptr [160] 
1187 @0x001b70bc assign_value [] 
1188 @0x001b70c0 pop_stack [] 
1189 @0x001b70c4 push_global_vptr [135] 
1190 @0x001b70c8 convert_to_value [] 
1191 @0x001b70cc push_global_vptr [136] 
1192 @0x001b70d0 push_global_var [160] 
1193 @0x001b70d4 query_member_ptr [0] 
1194 @0x001b70d8 assign_value [] 
1195 @0x001b70dc push_global_vptr [135] 
1196 @0x001b70e0 inc_value [1] 
1197 @0x001b70e4 pop_stack [] 
1198 @0x001b70e8 push_shared_string [2719] 'SF_COMBAT_BONUS_ITEM'
1199 @0x001b70ec call_in_efun [139, 1] tolower
1200 @0x001b70f0 push_global_vptr [161] 
1201 @0x001b70f4 assign_value [] 
1202 @0x001b70f8 pop_stack [] 
1203 @0x001b70fc push_global_vptr [135] 
1204 @0x001b7100 convert_to_value [] 
1205 @0x001b7104 push_global_vptr [136] 
1206 @0x001b7108 push_global_var [161] 
1207 @0x001b710c query_member_ptr [0] 
1208 @0x001b7110 assign_value [] 
1209 @0x001b7114 push_global_vptr [135] 
1210 @0x001b7118 inc_value [1] 
1211 @0x001b711c pop_stack [] 
1212 @0x001b7120 push_shared_string [2720] 'SF_CREATE_STD_MONSTER_TEAM'
1213 @0x001b7124 call_in_efun [139, 1] tolower
1214 @0x001b7128 push_global_vptr [162] 
1215 @0x001b712c assign_value [] 
1216 @0x001b7130 pop_stack [] 
1217 @0x001b7134 push_global_vptr [135] 
1218 @0x001b7138 convert_to_value [] 
1219 @0x001b713c push_global_vptr [136] 
1220 @0x001b7140 push_global_var [162] 
1221 @0x001b7144 query_member_ptr [0] 
1222 @0x001b7148 assign_value [] 
1223 @0x001b714c push_global_vptr [135] 
1224 @0x001b7150 inc_value [1] 
1225 @0x001b7154 pop_stack [] 
1226 @0x001b7158 push_shared_string [2721] 'SF_CREATE_MONSTER_TEAM'
1227 @0x001b715c call_in_efun [139, 1] tolower
1228 @0x001b7160 push_global_vptr [163] 
1229 @0x001b7164 assign_value [] 
1230 @0x001b7168 pop_stack [] 
1231 @0x001b716c push_global_vptr [135] 
1232 @0x001b7170 convert_to_value [] 
1233 @0x001b7174 push_global_vptr [136] 
1234 @0x001b7178 push_global_var [163] 
1235 @0x001b717c query_member_ptr [0] 
1236 @0x001b7180 assign_value [] 
1237 @0x001b7184 push_global_vptr [135] 
1238 @0x001b7188 inc_value [1] 
1239 @0x001b718c pop_stack [] 
1240 @0x001b7190 push_shared_string [2722] 'SF_DIE'
1241 @0x001b7194 call_in_efun [139, 1] tolower
1242 @0x001b7198 push_global_vptr [164] 
1243 @0x001b719c assign_value [] 
1244 @0x001b71a0 pop_stack [] 
1245 @0x001b71a4 push_global_vptr [135] 
1246 @0x001b71a8 convert_to_value [] 
1247 @0x001b71ac push_global_vptr [136] 
1248 @0x001b71b0 push_global_var [164] 
1249 @0x001b71b4 query_member_ptr [0] 
1250 @0x001b71b8 assign_value [] 
1251 @0x001b71bc push_global_vptr [135] 
1252 @0x001b71c0 inc_value [1] 
1253 @0x001b71c4 pop_stack [] 
1254 @0x001b71c8 push_shared_string [2723] 'SF_DIE_EX'
1255 @0x001b71cc call_in_efun [139, 1] tolower
1256 @0x001b71d0 push_global_vptr [165] 
1257 @0x001b71d4 assign_value [] 
1258 @0x001b71d8 pop_stack [] 
1259 @0x001b71dc push_global_vptr [135] 
1260 @0x001b71e0 convert_to_value [] 
1261 @0x001b71e4 push_global_vptr [136] 
1262 @0x001b71e8 push_global_var [165] 
1263 @0x001b71ec query_member_ptr [0] 
1264 @0x001b71f0 assign_value [] 
1265 @0x001b71f4 push_global_vptr [135] 
1266 @0x001b71f8 inc_value [1] 
1267 @0x001b71fc pop_stack [] 
1268 @0x001b7200 push_shared_string [2724] 'SF_REVIVE'
1269 @0x001b7204 call_in_efun [139, 1] tolower
1270 @0x001b7208 push_global_vptr [166] 
1271 @0x001b720c assign_value [] 
1272 @0x001b7210 pop_stack [] 
1273 @0x001b7214 push_global_vptr [135] 
1274 @0x001b7218 convert_to_value [] 
1275 @0x001b721c push_global_vptr [136] 
1276 @0x001b7220 push_global_var [166] 
1277 @0x001b7224 query_member_ptr [0] 
1278 @0x001b7228 assign_value [] 
1279 @0x001b722c push_global_vptr [135] 
1280 @0x001b7230 inc_value [1] 
1281 @0x001b7234 pop_stack [] 
1282 @0x001b7238 push_shared_string [2725] 'SF_MAKE_DIE'
1283 @0x001b723c call_in_efun [139, 1] tolower
1284 @0x001b7240 push_global_vptr [167] 
1285 @0x001b7244 assign_value [] 
1286 @0x001b7248 pop_stack [] 
1287 @0x001b724c push_global_vptr [135] 
1288 @0x001b7250 convert_to_value [] 
1289 @0x001b7254 push_global_vptr [136] 
1290 @0x001b7258 push_global_var [167] 
1291 @0x001b725c query_member_ptr [0] 
1292 @0x001b7260 assign_value [] 
1293 @0x001b7264 push_global_vptr [135] 
1294 @0x001b7268 inc_value [1] 
1295 @0x001b726c pop_stack [] 
1296 @0x001b7270 push_shared_string [2726] 'SF_DISCONNECTED_TIME_EXPIRED'
1297 @0x001b7274 call_in_efun [139, 1] tolower
1298 @0x001b7278 push_global_vptr [168] 
1299 @0x001b727c assign_value [] 
1300 @0x001b7280 pop_stack [] 
1301 @0x001b7284 push_global_vptr [135] 
1302 @0x001b7288 convert_to_value [] 
1303 @0x001b728c push_global_vptr [136] 
1304 @0x001b7290 push_global_var [168] 
1305 @0x001b7294 query_member_ptr [0] 
1306 @0x001b7298 assign_value [] 
1307 @0x001b729c push_global_vptr [135] 
1308 @0x001b72a0 inc_value [1] 
1309 @0x001b72a4 pop_stack [] 
1310 @0x001b72a8 push_shared_string [2727] 'SF_TRY_TO_DISMISS_TEAM'
1311 @0x001b72ac call_in_efun [139, 1] tolower
1312 @0x001b72b0 push_global_vptr [169] 
1313 @0x001b72b4 assign_value [] 
1314 @0x001b72b8 pop_stack [] 
1315 @0x001b72bc push_global_vptr [135] 
1316 @0x001b72c0 convert_to_value [] 
1317 @0x001b72c4 push_global_vptr [136] 
1318 @0x001b72c8 push_global_var [169] 
1319 @0x001b72cc query_member_ptr [0] 
1320 @0x001b72d0 assign_value [] 
1321 @0x001b72d4 push_global_vptr [135] 
1322 @0x001b72d8 inc_value [1] 
1323 @0x001b72dc pop_stack [] 
1324 @0x001b72e0 push_shared_string [2728] 'SF_TRY_TO_QUIT_TEAM'
1325 @0x001b72e4 call_in_efun [139, 1] tolower
1326 @0x001b72e8 push_global_vptr [170] 
1327 @0x001b72ec assign_value [] 
1328 @0x001b72f0 pop_stack [] 
1329 @0x001b72f4 push_global_vptr [135] 
1330 @0x001b72f8 convert_to_value [] 
1331 @0x001b72fc push_global_vptr [136] 
1332 @0x001b7300 push_global_var [170] 
1333 @0x001b7304 query_member_ptr [0] 
1334 @0x001b7308 assign_value [] 
1335 @0x001b730c push_global_vptr [135] 
1336 @0x001b7310 inc_value [1] 
1337 @0x001b7314 pop_stack [] 
1338 @0x001b7318 push_shared_string [2729] 'SF_DISMISS_TEAM_ALREADY'
1339 @0x001b731c call_in_efun [139, 1] tolower
1340 @0x001b7320 push_global_vptr [171] 
1341 @0x001b7324 assign_value [] 
1342 @0x001b7328 pop_stack [] 
1343 @0x001b732c push_global_vptr [135] 
1344 @0x001b7330 convert_to_value [] 
1345 @0x001b7334 push_global_vptr [136] 
1346 @0x001b7338 push_global_var [171] 
1347 @0x001b733c query_member_ptr [0] 
1348 @0x001b7340 assign_value [] 
1349 @0x001b7344 push_global_vptr [135] 
1350 @0x001b7348 inc_value [1] 
1351 @0x001b734c pop_stack [] 
1352 @0x001b7350 push_shared_string [2730] 'SF_DISMISS_TEAM'
1353 @0x001b7354 call_in_efun [139, 1] tolower
1354 @0x001b7358 push_global_vptr [172] 
1355 @0x001b735c assign_value [] 
1356 @0x001b7360 pop_stack [] 
1357 @0x001b7364 push_global_vptr [135] 
1358 @0x001b7368 convert_to_value [] 
1359 @0x001b736c push_global_vptr [136] 
1360 @0x001b7370 push_global_var [172] 
1361 @0x001b7374 query_member_ptr [0] 
1362 @0x001b7378 assign_value [] 
1363 @0x001b737c push_global_vptr [135] 
1364 @0x001b7380 inc_value [1] 
1365 @0x001b7384 pop_stack [] 
1366 @0x001b7388 push_shared_string [2731] 'SF_ENTER_ROOM'
1367 @0x001b738c call_in_efun [139, 1] tolower
1368 @0x001b7390 push_global_vptr [173] 
1369 @0x001b7394 assign_value [] 
1370 @0x001b7398 pop_stack [] 
1371 @0x001b739c push_global_vptr [135] 
1372 @0x001b73a0 convert_to_value [] 
1373 @0x001b73a4 push_global_vptr [136] 
1374 @0x001b73a8 push_global_var [173] 
1375 @0x001b73ac query_member_ptr [0] 
1376 @0x001b73b0 assign_value [] 
1377 @0x001b73b4 push_global_vptr [135] 
1378 @0x001b73b8 inc_value [1] 
1379 @0x001b73bc pop_stack [] 
1380 @0x001b73c0 push_shared_string [2732] 'SF_MEMBER_ENTER_ROOM'
1381 @0x001b73c4 call_in_efun [139, 1] tolower
1382 @0x001b73c8 push_global_vptr [174] 
1383 @0x001b73cc assign_value [] 
1384 @0x001b73d0 pop_stack [] 
1385 @0x001b73d4 push_global_vptr [135] 
1386 @0x001b73d8 convert_to_value [] 
1387 @0x001b73dc push_global_vptr [136] 
1388 @0x001b73e0 push_global_var [174] 
1389 @0x001b73e4 query_member_ptr [0] 
1390 @0x001b73e8 assign_value [] 
1391 @0x001b73ec push_global_vptr [135] 
1392 @0x001b73f0 inc_value [1] 
1393 @0x001b73f4 pop_stack [] 
1394 @0x001b73f8 push_shared_string [2733] 'SF_EXCHANGE'
1395 @0x001b73fc call_in_efun [139, 1] tolower
1396 @0x001b7400 push_global_vptr [175] 
1397 @0x001b7404 assign_value [] 
1398 @0x001b7408 pop_stack [] 
1399 @0x001b740c push_global_vptr [135] 
1400 @0x001b7410 convert_to_value [] 
1401 @0x001b7414 push_global_vptr [136] 
1402 @0x001b7418 push_global_var [175] 
1403 @0x001b741c query_member_ptr [0] 
1404 @0x001b7420 assign_value [] 
1405 @0x001b7424 push_global_vptr [135] 
1406 @0x001b7428 inc_value [1] 
1407 @0x001b742c pop_stack [] 
1408 @0x001b7430 push_shared_string [2734] 'SF_ESCAPE'
1409 @0x001b7434 call_in_efun [139, 1] tolower
1410 @0x001b7438 push_global_vptr [176] 
1411 @0x001b743c assign_value [] 
1412 @0x001b7440 pop_stack [] 
1413 @0x001b7444 push_global_vptr [135] 
1414 @0x001b7448 convert_to_value [] 
1415 @0x001b744c push_global_vptr [136] 
1416 @0x001b7450 push_global_var [176] 
1417 @0x001b7454 query_member_ptr [0] 
1418 @0x001b7458 assign_value [] 
1419 @0x001b745c push_global_vptr [135] 
1420 @0x001b7460 inc_value [1] 
1421 @0x001b7464 pop_stack [] 
1422 @0x001b7468 push_shared_string [2735] 'SF_FLEE'
1423 @0x001b746c call_in_efun [139, 1] tolower
1424 @0x001b7470 push_global_vptr [177] 
1425 @0x001b7474 assign_value [] 
1426 @0x001b7478 pop_stack [] 
1427 @0x001b747c push_global_vptr [135] 
1428 @0x001b7480 convert_to_value [] 
1429 @0x001b7484 push_global_vptr [136] 
1430 @0x001b7488 push_global_var [177] 
1431 @0x001b748c query_member_ptr [0] 
1432 @0x001b7490 assign_value [] 
1433 @0x001b7494 push_global_vptr [135] 
1434 @0x001b7498 inc_value [1] 
1435 @0x001b749c pop_stack [] 
1436 @0x001b74a0 push_shared_string [2736] 'SF_FLEE_FAILED'
1437 @0x001b74a4 call_in_efun [139, 1] tolower
1438 @0x001b74a8 push_global_vptr [178] 
1439 @0x001b74ac assign_value [] 
1440 @0x001b74b0 pop_stack [] 
1441 @0x001b74b4 push_global_vptr [135] 
1442 @0x001b74b8 convert_to_value [] 
1443 @0x001b74bc push_global_vptr [136] 
1444 @0x001b74c0 push_global_var [178] 
1445 @0x001b74c4 query_member_ptr [0] 
1446 @0x001b74c8 assign_value [] 
1447 @0x001b74cc push_global_vptr [135] 
1448 @0x001b74d0 inc_value [1] 
1449 @0x001b74d4 pop_stack [] 
1450 @0x001b74d8 push_shared_string [2737] 'SF_FLEE_NO_LEAVE_TEAM'
1451 @0x001b74dc call_in_efun [139, 1] tolower
1452 @0x001b74e0 push_global_vptr [179] 
1453 @0x001b74e4 assign_value [] 
1454 @0x001b74e8 pop_stack [] 
1455 @0x001b74ec push_global_vptr [135] 
1456 @0x001b74f0 convert_to_value [] 
1457 @0x001b74f4 push_global_vptr [136] 
1458 @0x001b74f8 push_global_var [179] 
1459 @0x001b74fc query_member_ptr [0] 
1460 @0x001b7500 assign_value [] 
1461 @0x001b7504 push_global_vptr [135] 
1462 @0x001b7508 inc_value [1] 
1463 @0x001b750c pop_stack [] 
1464 @0x001b7510 push_shared_string [2738] 'SF_HEART_BEAT'
1465 @0x001b7514 call_in_efun [139, 1] tolower
1466 @0x001b7518 push_global_vptr [180] 
1467 @0x001b751c assign_value [] 
1468 @0x001b7520 pop_stack [] 
1469 @0x001b7524 push_global_vptr [135] 
1470 @0x001b7528 convert_to_value [] 
1471 @0x001b752c push_global_vptr [136] 
1472 @0x001b7530 push_global_var [180] 
1473 @0x001b7534 query_member_ptr [0] 
1474 @0x001b7538 assign_value [] 
1475 @0x001b753c push_global_vptr [135] 
1476 @0x001b7540 inc_value [1] 
1477 @0x001b7544 pop_stack [] 
1478 @0x001b7548 push_shared_string [2739] 'SF_INIT_COMBAT'
1479 @0x001b754c call_in_efun [139, 1] tolower
1480 @0x001b7550 push_global_vptr [181] 
1481 @0x001b7554 assign_value [] 
1482 @0x001b7558 pop_stack [] 
1483 @0x001b755c push_global_vptr [135] 
1484 @0x001b7560 convert_to_value [] 
1485 @0x001b7564 push_global_vptr [136] 
1486 @0x001b7568 push_global_var [181] 
1487 @0x001b756c query_member_ptr [0] 
1488 @0x001b7570 assign_value [] 
1489 @0x001b7574 push_global_vptr [135] 
1490 @0x001b7578 inc_value [1] 
1491 @0x001b757c pop_stack [] 
1492 @0x001b7580 push_shared_string [2740] 'SF_INQUIRED'
1493 @0x001b7584 call_in_efun [139, 1] tolower
1494 @0x001b7588 push_global_vptr [182] 
1495 @0x001b758c assign_value [] 
1496 @0x001b7590 pop_stack [] 
1497 @0x001b7594 push_global_vptr [135] 
1498 @0x001b7598 convert_to_value [] 
1499 @0x001b759c push_global_vptr [136] 
1500 @0x001b75a0 push_global_var [182] 
1501 @0x001b75a4 query_member_ptr [0] 
1502 @0x001b75a8 assign_value [] 
1503 @0x001b75ac push_global_vptr [135] 
1504 @0x001b75b0 inc_value [1] 
1505 @0x001b75b4 pop_stack [] 
1506 @0x001b75b8 push_shared_string [2741] 'SF_JOIN_TEAM'
1507 @0x001b75bc call_in_efun [139, 1] tolower
1508 @0x001b75c0 push_global_vptr [183] 
1509 @0x001b75c4 assign_value [] 
1510 @0x001b75c8 pop_stack [] 
1511 @0x001b75cc push_global_vptr [135] 
1512 @0x001b75d0 convert_to_value [] 
1513 @0x001b75d4 push_global_vptr [136] 
1514 @0x001b75d8 push_global_var [183] 
1515 @0x001b75dc query_member_ptr [0] 
1516 @0x001b75e0 assign_value [] 
1517 @0x001b75e4 push_global_vptr [135] 
1518 @0x001b75e8 inc_value [1] 
1519 @0x001b75ec pop_stack [] 
1520 @0x001b75f0 push_shared_string [2742] 'SF_MEMBER_JOIN_TEAM'
1521 @0x001b75f4 call_in_efun [139, 1] tolower
1522 @0x001b75f8 push_global_vptr [184] 
1523 @0x001b75fc assign_value [] 
1524 @0x001b7600 pop_stack [] 
1525 @0x001b7604 push_global_vptr [135] 
1526 @0x001b7608 convert_to_value [] 
1527 @0x001b760c push_global_vptr [136] 
1528 @0x001b7610 push_global_var [184] 
1529 @0x001b7614 query_member_ptr [0] 
1530 @0x001b7618 assign_value [] 
1531 @0x001b761c push_global_vptr [135] 
1532 @0x001b7620 inc_value [1] 
1533 @0x001b7624 pop_stack [] 
1534 @0x001b7628 push_shared_string [2743] 'SF_MEMBER_JOIN_TEAM_ALREADY'
1535 @0x001b762c call_in_efun [139, 1] tolower
1536 @0x001b7630 push_global_vptr [185] 
1537 @0x001b7634 assign_value [] 
1538 @0x001b7638 pop_stack [] 
1539 @0x001b763c push_global_vptr [135] 
1540 @0x001b7640 convert_to_value [] 
1541 @0x001b7644 push_global_vptr [136] 
1542 @0x001b7648 push_global_var [185] 
1543 @0x001b764c query_member_ptr [0] 
1544 @0x001b7650 assign_value [] 
1545 @0x001b7654 push_global_vptr [135] 
1546 @0x001b7658 inc_value [1] 
1547 @0x001b765c pop_stack [] 
1548 @0x001b7660 push_shared_string [2744] 'SF_KILL'
1549 @0x001b7664 call_in_efun [139, 1] tolower
1550 @0x001b7668 push_global_vptr [186] 
1551 @0x001b766c assign_value [] 
1552 @0x001b7670 pop_stack [] 
1553 @0x001b7674 push_global_vptr [135] 
1554 @0x001b7678 convert_to_value [] 
1555 @0x001b767c push_global_vptr [136] 
1556 @0x001b7680 push_global_var [186] 
1557 @0x001b7684 query_member_ptr [0] 
1558 @0x001b7688 assign_value [] 
1559 @0x001b768c push_global_vptr [135] 
1560 @0x001b7690 inc_value [1] 
1561 @0x001b7694 pop_stack [] 
1562 @0x001b7698 push_shared_string [2745] 'SF_LOGOUT'
1563 @0x001b769c call_in_efun [139, 1] tolower
1564 @0x001b76a0 push_global_vptr [187] 
1565 @0x001b76a4 assign_value [] 
1566 @0x001b76a8 pop_stack [] 
1567 @0x001b76ac push_global_vptr [135] 
1568 @0x001b76b0 convert_to_value [] 
1569 @0x001b76b4 push_global_vptr [136] 
1570 @0x001b76b8 push_global_var [187] 
1571 @0x001b76bc query_member_ptr [0] 
1572 @0x001b76c0 assign_value [] 
1573 @0x001b76c4 push_global_vptr [135] 
1574 @0x001b76c8 inc_value [1] 
1575 @0x001b76cc pop_stack [] 
1576 @0x001b76d0 push_shared_string [2746] 'SF_MENU'
1577 @0x001b76d4 call_in_efun [139, 1] tolower
1578 @0x001b76d8 push_global_vptr [188] 
1579 @0x001b76dc assign_value [] 
1580 @0x001b76e0 pop_stack [] 
1581 @0x001b76e4 push_global_vptr [135] 
1582 @0x001b76e8 convert_to_value [] 
1583 @0x001b76ec push_global_vptr [136] 
1584 @0x001b76f0 push_global_var [188] 
1585 @0x001b76f4 query_member_ptr [0] 
1586 @0x001b76f8 assign_value [] 
1587 @0x001b76fc push_global_vptr [135] 
1588 @0x001b7700 inc_value [1] 
1589 @0x001b7704 pop_stack [] 
1590 @0x001b7708 push_shared_string [2747] 'SF_MODIFY_BONUS_AMOUNT'
1591 @0x001b770c call_in_efun [139, 1] tolower
1592 @0x001b7710 push_global_vptr [189] 
1593 @0x001b7714 assign_value [] 
1594 @0x001b7718 pop_stack [] 
1595 @0x001b771c push_global_vptr [135] 
1596 @0x001b7720 convert_to_value [] 
1597 @0x001b7724 push_global_vptr [136] 
1598 @0x001b7728 push_global_var [189] 
1599 @0x001b772c query_member_ptr [0] 
1600 @0x001b7730 assign_value [] 
1601 @0x001b7734 push_global_vptr [135] 
1602 @0x001b7738 inc_value [1] 
1603 @0x001b773c pop_stack [] 
1604 @0x001b7740 push_shared_string [2748] 'SF_NO_DESTRUCT'
1605 @0x001b7744 call_in_efun [139, 1] tolower
1606 @0x001b7748 push_global_vptr [190] 
1607 @0x001b774c assign_value [] 
1608 @0x001b7750 pop_stack [] 
1609 @0x001b7754 push_global_vptr [135] 
1610 @0x001b7758 convert_to_value [] 
1611 @0x001b775c push_global_vptr [136] 
1612 @0x001b7760 push_global_var [190] 
1613 @0x001b7764 query_member_ptr [0] 
1614 @0x001b7768 assign_value [] 
1615 @0x001b776c push_global_vptr [135] 
1616 @0x001b7770 inc_value [1] 
1617 @0x001b7774 pop_stack [] 
1618 @0x001b7778 push_shared_string [2749] 'SF_NPC_CLONE'
1619 @0x001b777c call_in_efun [139, 1] tolower
1620 @0x001b7780 push_global_vptr [191] 
1621 @0x001b7784 assign_value [] 
1622 @0x001b7788 pop_stack [] 
1623 @0x001b778c push_global_vptr [135] 
1624 @0x001b7790 convert_to_value [] 
1625 @0x001b7794 push_global_vptr [136] 
1626 @0x001b7798 push_global_var [191] 
1627 @0x001b779c query_member_ptr [0] 
1628 @0x001b77a0 assign_value [] 
1629 @0x001b77a4 push_global_vptr [135] 
1630 @0x001b77a8 inc_value [1] 
1631 @0x001b77ac pop_stack [] 
1632 @0x001b77b0 push_shared_string [2750] 'SF_PARTY_KICKOUT'
1633 @0x001b77b4 call_in_efun [139, 1] tolower
1634 @0x001b77b8 push_global_vptr [192] 
1635 @0x001b77bc assign_value [] 
1636 @0x001b77c0 pop_stack [] 
1637 @0x001b77c4 push_global_vptr [135] 
1638 @0x001b77c8 convert_to_value [] 
1639 @0x001b77cc push_global_vptr [136] 
1640 @0x001b77d0 push_global_var [192] 
1641 @0x001b77d4 query_member_ptr [0] 
1642 @0x001b77d8 assign_value [] 
1643 @0x001b77dc push_global_vptr [135] 
1644 @0x001b77e0 inc_value [1] 
1645 @0x001b77e4 pop_stack [] 
1646 @0x001b77e8 push_shared_string [2751] 'SF_PARTY_QUIT'
1647 @0x001b77ec call_in_efun [139, 1] tolower
1648 @0x001b77f0 push_global_vptr [193] 
1649 @0x001b77f4 assign_value [] 
1650 @0x001b77f8 pop_stack [] 
1651 @0x001b77fc push_global_vptr [135] 
1652 @0x001b7800 convert_to_value [] 
1653 @0x001b7804 push_global_vptr [136] 
1654 @0x001b7808 push_global_var [193] 
1655 @0x001b780c query_member_ptr [0] 
1656 @0x001b7810 assign_value [] 
1657 @0x001b7814 push_global_vptr [135] 
1658 @0x001b7818 inc_value [1] 
1659 @0x001b781c pop_stack [] 
1660 @0x001b7820 push_shared_string [2752] 'SF_PARTY_DISPUTE'
1661 @0x001b7824 call_in_efun [139, 1] tolower
1662 @0x001b7828 push_global_vptr [194] 
1663 @0x001b782c assign_value [] 
1664 @0x001b7830 pop_stack [] 
1665 @0x001b7834 push_global_vptr [135] 
1666 @0x001b7838 convert_to_value [] 
1667 @0x001b783c push_global_vptr [136] 
1668 @0x001b7840 push_global_var [194] 
1669 @0x001b7844 query_member_ptr [0] 
1670 @0x001b7848 assign_value [] 
1671 @0x001b784c push_global_vptr [135] 
1672 @0x001b7850 inc_value [1] 
1673 @0x001b7854 pop_stack [] 
1674 @0x001b7858 push_shared_string [2753] 'SF_PHYSICAL_SPICAL_DAMAGE'
1675 @0x001b785c call_in_efun [139, 1] tolower
1676 @0x001b7860 push_global_vptr [195] 
1677 @0x001b7864 assign_value [] 
1678 @0x001b7868 pop_stack [] 
1679 @0x001b786c push_global_vptr [135] 
1680 @0x001b7870 convert_to_value [] 
1681 @0x001b7874 push_global_vptr [136] 
1682 @0x001b7878 push_global_var [195] 
1683 @0x001b787c query_member_ptr [0] 
1684 @0x001b7880 assign_value [] 
1685 @0x001b7884 push_global_vptr [135] 
1686 @0x001b7888 inc_value [1] 
1687 @0x001b788c pop_stack [] 
1688 @0x001b7890 push_shared_string [2754] 'SF_QUIT_TEAM_ALREADY'
1689 @0x001b7894 call_in_efun [139, 1] tolower
1690 @0x001b7898 push_global_vptr [196] 
1691 @0x001b789c assign_value [] 
1692 @0x001b78a0 pop_stack [] 
1693 @0x001b78a4 push_global_vptr [135] 
1694 @0x001b78a8 convert_to_value [] 
1695 @0x001b78ac push_global_vptr [136] 
1696 @0x001b78b0 push_global_var [196] 
1697 @0x001b78b4 query_member_ptr [0] 
1698 @0x001b78b8 assign_value [] 
1699 @0x001b78bc push_global_vptr [135] 
1700 @0x001b78c0 inc_value [1] 
1701 @0x001b78c4 pop_stack [] 
1702 @0x001b78c8 push_shared_string [2755] 'SF_QUIT_TEAM'
1703 @0x001b78cc call_in_efun [139, 1] tolower
1704 @0x001b78d0 push_global_vptr [197] 
1705 @0x001b78d4 assign_value [] 
1706 @0x001b78d8 pop_stack [] 
1707 @0x001b78dc push_global_vptr [135] 
1708 @0x001b78e0 convert_to_value [] 
1709 @0x001b78e4 push_global_vptr [136] 
1710 @0x001b78e8 push_global_var [197] 
1711 @0x001b78ec query_member_ptr [0] 
1712 @0x001b78f0 assign_value [] 
1713 @0x001b78f4 push_global_vptr [135] 
1714 @0x001b78f8 inc_value [1] 
1715 @0x001b78fc pop_stack [] 
1716 @0x001b7900 push_shared_string [2756] 'SF_RECEIVE_DAMAGE'
1717 @0x001b7904 call_in_efun [139, 1] tolower
1718 @0x001b7908 push_global_vptr [198] 
1719 @0x001b790c assign_value [] 
1720 @0x001b7910 pop_stack [] 
1721 @0x001b7914 push_global_vptr [135] 
1722 @0x001b7918 convert_to_value [] 
1723 @0x001b791c push_global_vptr [136] 
1724 @0x001b7920 push_global_var [198] 
1725 @0x001b7924 query_member_ptr [0] 
1726 @0x001b7928 assign_value [] 
1727 @0x001b792c push_global_vptr [135] 
1728 @0x001b7930 inc_value [1] 
1729 @0x001b7934 pop_stack [] 
1730 @0x001b7938 push_shared_string [2757] 'SF_RECEIVE_DAMAGE_EX'
1731 @0x001b793c call_in_efun [139, 1] tolower
1732 @0x001b7940 push_global_vptr [199] 
1733 @0x001b7944 assign_value [] 
1734 @0x001b7948 pop_stack [] 
1735 @0x001b794c push_global_vptr [135] 
1736 @0x001b7950 convert_to_value [] 
1737 @0x001b7954 push_global_vptr [136] 
1738 @0x001b7958 push_global_var [199] 
1739 @0x001b795c query_member_ptr [0] 
1740 @0x001b7960 assign_value [] 
1741 @0x001b7964 push_global_vptr [135] 
1742 @0x001b7968 inc_value [1] 
1743 @0x001b796c pop_stack [] 
1744 @0x001b7970 push_shared_string [2758] 'SF_CHANGE_PHY_DAMAGE'
1745 @0x001b7974 call_in_efun [139, 1] tolower
1746 @0x001b7978 push_global_vptr [200] 
1747 @0x001b797c assign_value [] 
1748 @0x001b7980 pop_stack [] 
1749 @0x001b7984 push_global_vptr [135] 
1750 @0x001b7988 convert_to_value [] 
1751 @0x001b798c push_global_vptr [136] 
1752 @0x001b7990 push_global_var [200] 
1753 @0x001b7994 query_member_ptr [0] 
1754 @0x001b7998 assign_value [] 
1755 @0x001b799c push_global_vptr [135] 
1756 @0x001b79a0 inc_value [1] 
1757 @0x001b79a4 pop_stack [] 
1758 @0x001b79a8 push_shared_string [2759] 'SF_CHANGE_DAMAGE'
1759 @0x001b79ac call_in_efun [139, 1] tolower
1760 @0x001b79b0 push_global_vptr [201] 
1761 @0x001b79b4 assign_value [] 
1762 @0x001b79b8 pop_stack [] 
1763 @0x001b79bc push_global_vptr [135] 
1764 @0x001b79c0 convert_to_value [] 
1765 @0x001b79c4 push_global_vptr [136] 
1766 @0x001b79c8 push_global_var [201] 
1767 @0x001b79cc query_member_ptr [0] 
1768 @0x001b79d0 assign_value [] 
1769 @0x001b79d4 push_global_vptr [135] 
1770 @0x001b79d8 inc_value [1] 
1771 @0x001b79dc pop_stack [] 
1772 @0x001b79e0 push_shared_string [2760] 'SF_CHANGE_SKILL_MANA'
1773 @0x001b79e4 call_in_efun [139, 1] tolower
1774 @0x001b79e8 push_global_vptr [202] 
1775 @0x001b79ec assign_value [] 
1776 @0x001b79f0 pop_stack [] 
1777 @0x001b79f4 push_global_vptr [135] 
1778 @0x001b79f8 convert_to_value [] 
1779 @0x001b79fc push_global_vptr [136] 
1780 @0x001b7a00 push_global_var [202] 
1781 @0x001b7a04 query_member_ptr [0] 
1782 @0x001b7a08 assign_value [] 
1783 @0x001b7a0c push_global_vptr [135] 
1784 @0x001b7a10 inc_value [1] 
1785 @0x001b7a14 pop_stack [] 
1786 @0x001b7a18 push_shared_string [2761] 'SF_CHANGE_TO_DOUBLE_MAGIC_HIT'
1787 @0x001b7a1c call_in_efun [139, 1] tolower
1788 @0x001b7a20 push_global_vptr [203] 
1789 @0x001b7a24 assign_value [] 
1790 @0x001b7a28 pop_stack [] 
1791 @0x001b7a2c push_global_vptr [135] 
1792 @0x001b7a30 convert_to_value [] 
1793 @0x001b7a34 push_global_vptr [136] 
1794 @0x001b7a38 push_global_var [203] 
1795 @0x001b7a3c query_member_ptr [0] 
1796 @0x001b7a40 assign_value [] 
1797 @0x001b7a44 push_global_vptr [135] 
1798 @0x001b7a48 inc_value [1] 
1799 @0x001b7a4c pop_stack [] 
1800 @0x001b7a50 push_shared_string [2762] 'SF_WHEN_ACTIVE_DOUBLE_MAGIC'
1801 @0x001b7a54 call_in_efun [139, 1] tolower
1802 @0x001b7a58 push_global_vptr [204] 
1803 @0x001b7a5c assign_value [] 
1804 @0x001b7a60 pop_stack [] 
1805 @0x001b7a64 push_global_vptr [135] 
1806 @0x001b7a68 convert_to_value [] 
1807 @0x001b7a6c push_global_vptr [136] 
1808 @0x001b7a70 push_global_var [204] 
1809 @0x001b7a74 query_member_ptr [0] 
1810 @0x001b7a78 assign_value [] 
1811 @0x001b7a7c push_global_vptr [135] 
1812 @0x001b7a80 inc_value [1] 
1813 @0x001b7a84 pop_stack [] 
1814 @0x001b7a88 push_shared_string [2763] 'SF_EXCLUDE_VICTIM'
1815 @0x001b7a8c call_in_efun [139, 1] tolower
1816 @0x001b7a90 push_global_vptr [205] 
1817 @0x001b7a94 assign_value [] 
1818 @0x001b7a98 pop_stack [] 
1819 @0x001b7a9c push_global_vptr [135] 
1820 @0x001b7aa0 convert_to_value [] 
1821 @0x001b7aa4 push_global_vptr [136] 
1822 @0x001b7aa8 push_global_var [205] 
1823 @0x001b7aac query_member_ptr [0] 
1824 @0x001b7ab0 assign_value [] 
1825 @0x001b7ab4 push_global_vptr [135] 
1826 @0x001b7ab8 inc_value [1] 
1827 @0x001b7abc pop_stack [] 
1828 @0x001b7ac0 push_shared_string [2764] 'SF_ADJUST_VICTIM'
1829 @0x001b7ac4 call_in_efun [139, 1] tolower
1830 @0x001b7ac8 push_global_vptr [206] 
1831 @0x001b7acc assign_value [] 
1832 @0x001b7ad0 pop_stack [] 
1833 @0x001b7ad4 push_global_vptr [135] 
1834 @0x001b7ad8 convert_to_value [] 
1835 @0x001b7adc push_global_vptr [136] 
1836 @0x001b7ae0 push_global_var [206] 
1837 @0x001b7ae4 query_member_ptr [0] 
1838 @0x001b7ae8 assign_value [] 
1839 @0x001b7aec push_global_vptr [135] 
1840 @0x001b7af0 inc_value [1] 
1841 @0x001b7af4 pop_stack [] 
1842 @0x001b7af8 push_shared_string [2765] 'SF_CHANGE_RESIST_CONTROL'
1843 @0x001b7afc call_in_efun [139, 1] tolower
1844 @0x001b7b00 push_global_vptr [207] 
1845 @0x001b7b04 assign_value [] 
1846 @0x001b7b08 pop_stack [] 
1847 @0x001b7b0c push_global_vptr [135] 
1848 @0x001b7b10 convert_to_value [] 
1849 @0x001b7b14 push_global_vptr [136] 
1850 @0x001b7b18 push_global_var [207] 
1851 @0x001b7b1c query_member_ptr [0] 
1852 @0x001b7b20 assign_value [] 
1853 @0x001b7b24 push_global_vptr [135] 
1854 @0x001b7b28 inc_value [1] 
1855 @0x001b7b2c pop_stack [] 
1856 @0x001b7b30 push_shared_string [2766] 'SF_MAGIC_CHASE'
1857 @0x001b7b34 call_in_efun [139, 1] tolower
1858 @0x001b7b38 push_global_vptr [208] 
1859 @0x001b7b3c assign_value [] 
1860 @0x001b7b40 pop_stack [] 
1861 @0x001b7b44 push_global_vptr [135] 
1862 @0x001b7b48 convert_to_value [] 
1863 @0x001b7b4c push_global_vptr [136] 
1864 @0x001b7b50 push_global_var [208] 
1865 @0x001b7b54 query_member_ptr [0] 
1866 @0x001b7b58 assign_value [] 
1867 @0x001b7b5c push_global_vptr [135] 
1868 @0x001b7b60 inc_value [1] 
1869 @0x001b7b64 pop_stack [] 
1870 @0x001b7b68 push_shared_string [2767] 'SF_RELEASE_RELATION'
1871 @0x001b7b6c call_in_efun [139, 1] tolower
1872 @0x001b7b70 push_global_vptr [209] 
1873 @0x001b7b74 assign_value [] 
1874 @0x001b7b78 pop_stack [] 
1875 @0x001b7b7c push_global_vptr [135] 
1876 @0x001b7b80 convert_to_value [] 
1877 @0x001b7b84 push_global_vptr [136] 
1878 @0x001b7b88 push_global_var [209] 
1879 @0x001b7b8c query_member_ptr [0] 
1880 @0x001b7b90 assign_value [] 
1881 @0x001b7b94 push_global_vptr [135] 
1882 @0x001b7b98 inc_value [1] 
1883 @0x001b7b9c pop_stack [] 
1884 @0x001b7ba0 push_shared_string [2768] 'SF_REQUEST'
1885 @0x001b7ba4 call_in_efun [139, 1] tolower
1886 @0x001b7ba8 push_global_vptr [210] 
1887 @0x001b7bac assign_value [] 
1888 @0x001b7bb0 pop_stack [] 
1889 @0x001b7bb4 push_global_vptr [135] 
1890 @0x001b7bb8 convert_to_value [] 
1891 @0x001b7bbc push_global_vptr [136] 
1892 @0x001b7bc0 push_global_var [210] 
1893 @0x001b7bc4 query_member_ptr [0] 
1894 @0x001b7bc8 assign_value [] 
1895 @0x001b7bcc push_global_vptr [135] 
1896 @0x001b7bd0 inc_value [1] 
1897 @0x001b7bd4 pop_stack [] 
1898 @0x001b7bd8 push_shared_string [2769] 'SF_SELECT_TACTICS'
1899 @0x001b7bdc call_in_efun [139, 1] tolower
1900 @0x001b7be0 push_global_vptr [211] 
1901 @0x001b7be4 assign_value [] 
1902 @0x001b7be8 pop_stack [] 
1903 @0x001b7bec push_global_vptr [135] 
1904 @0x001b7bf0 convert_to_value [] 
1905 @0x001b7bf4 push_global_vptr [136] 
1906 @0x001b7bf8 push_global_var [211] 
1907 @0x001b7bfc query_member_ptr [0] 
1908 @0x001b7c00 assign_value [] 
1909 @0x001b7c04 push_global_vptr [135] 
1910 @0x001b7c08 inc_value [1] 
1911 @0x001b7c0c pop_stack [] 
1912 @0x001b7c10 push_shared_string [2770] 'SF_SELECTED_ITEM'
1913 @0x001b7c14 call_in_efun [139, 1] tolower
1914 @0x001b7c18 push_global_vptr [212] 
1915 @0x001b7c1c assign_value [] 
1916 @0x001b7c20 pop_stack [] 
1917 @0x001b7c24 push_global_vptr [135] 
1918 @0x001b7c28 convert_to_value [] 
1919 @0x001b7c2c push_global_vptr [136] 
1920 @0x001b7c30 push_global_var [212] 
1921 @0x001b7c34 query_member_ptr [0] 
1922 @0x001b7c38 assign_value [] 
1923 @0x001b7c3c push_global_vptr [135] 
1924 @0x001b7c40 inc_value [1] 
1925 @0x001b7c44 pop_stack [] 
1926 @0x001b7c48 push_shared_string [2771] 'SF_SKILL_SUCCESS_RATE'
1927 @0x001b7c4c call_in_efun [139, 1] tolower
1928 @0x001b7c50 push_global_vptr [213] 
1929 @0x001b7c54 assign_value [] 
1930 @0x001b7c58 pop_stack [] 
1931 @0x001b7c5c push_global_vptr [135] 
1932 @0x001b7c60 convert_to_value [] 
1933 @0x001b7c64 push_global_vptr [136] 
1934 @0x001b7c68 push_global_var [213] 
1935 @0x001b7c6c query_member_ptr [0] 
1936 @0x001b7c70 assign_value [] 
1937 @0x001b7c74 push_global_vptr [135] 
1938 @0x001b7c78 inc_value [1] 
1939 @0x001b7c7c pop_stack [] 
1940 @0x001b7c80 push_shared_string [2772] 'SF_SUMMON'
1941 @0x001b7c84 call_in_efun [139, 1] tolower
1942 @0x001b7c88 push_global_vptr [214] 
1943 @0x001b7c8c assign_value [] 
1944 @0x001b7c90 pop_stack [] 
1945 @0x001b7c94 push_global_vptr [135] 
1946 @0x001b7c98 convert_to_value [] 
1947 @0x001b7c9c push_global_vptr [136] 
1948 @0x001b7ca0 push_global_var [214] 
1949 @0x001b7ca4 query_member_ptr [0] 
1950 @0x001b7ca8 assign_value [] 
1951 @0x001b7cac push_global_vptr [135] 
1952 @0x001b7cb0 inc_value [1] 
1953 @0x001b7cb4 pop_stack [] 
1954 @0x001b7cb8 push_shared_string [2773] 'SF_TAKE_MENU_CONDITION'
1955 @0x001b7cbc call_in_efun [139, 1] tolower
1956 @0x001b7cc0 push_global_vptr [215] 
1957 @0x001b7cc4 assign_value [] 
1958 @0x001b7cc8 pop_stack [] 
1959 @0x001b7ccc push_global_vptr [135] 
1960 @0x001b7cd0 convert_to_value [] 
1961 @0x001b7cd4 push_global_vptr [136] 
1962 @0x001b7cd8 push_global_var [215] 
1963 @0x001b7cdc query_member_ptr [0] 
1964 @0x001b7ce0 assign_value [] 
1965 @0x001b7ce4 push_global_vptr [135] 
1966 @0x001b7ce8 inc_value [1] 
1967 @0x001b7cec pop_stack [] 
1968 @0x001b7cf0 push_shared_string [2774] 'SF_STEP_MARCH'
1969 @0x001b7cf4 call_in_efun [139, 1] tolower
1970 @0x001b7cf8 push_global_vptr [216] 
1971 @0x001b7cfc assign_value [] 
1972 @0x001b7d00 pop_stack [] 
1973 @0x001b7d04 push_global_vptr [135] 
1974 @0x001b7d08 convert_to_value [] 
1975 @0x001b7d0c push_global_vptr [136] 
1976 @0x001b7d10 push_global_var [216] 
1977 @0x001b7d14 query_member_ptr [0] 
1978 @0x001b7d18 assign_value [] 
1979 @0x001b7d1c push_global_vptr [135] 
1980 @0x001b7d20 inc_value [1] 
1981 @0x001b7d24 pop_stack [] 
1982 @0x001b7d28 push_shared_string [2775] 'SF_CHANGE_RATE'
1983 @0x001b7d2c call_in_efun [139, 1] tolower
1984 @0x001b7d30 push_global_vptr [217] 
1985 @0x001b7d34 assign_value [] 
1986 @0x001b7d38 pop_stack [] 
1987 @0x001b7d3c push_global_vptr [135] 
1988 @0x001b7d40 convert_to_value [] 
1989 @0x001b7d44 push_global_vptr [136] 
1990 @0x001b7d48 push_global_var [217] 
1991 @0x001b7d4c query_member_ptr [0] 
1992 @0x001b7d50 assign_value [] 
1993 @0x001b7d54 push_global_vptr [135] 
1994 @0x001b7d58 inc_value [1] 
1995 @0x001b7d5c pop_stack [] 
1996 @0x001b7d60 push_shared_string [2776] 'SF_HOOK_MAP_EVENT'
1997 @0x001b7d64 call_in_efun [139, 1] tolower
1998 @0x001b7d68 push_global_vptr [218] 
1999 @0x001b7d6c assign_value [] 
2000 @0x001b7d70 pop_stack [] 
2001 @0x001b7d74 push_global_vptr [135] 
2002 @0x001b7d78 convert_to_value [] 
2003 @0x001b7d7c push_global_vptr [136] 
2004 @0x001b7d80 push_global_var [218] 
2005 @0x001b7d84 query_member_ptr [0] 
2006 @0x001b7d88 assign_value [] 
2007 @0x001b7d8c push_global_vptr [135] 
2008 @0x001b7d90 inc_value [1] 
2009 @0x001b7d94 pop_stack [] 
2010 @0x001b7d98 push_shared_string [2777] 'SF_ITEM_TELEPORT'
2011 @0x001b7d9c call_in_efun [139, 1] tolower
2012 @0x001b7da0 push_global_vptr [219] 
2013 @0x001b7da4 assign_value [] 
2014 @0x001b7da8 pop_stack [] 
2015 @0x001b7dac push_global_vptr [135] 
2016 @0x001b7db0 convert_to_value [] 
2017 @0x001b7db4 push_global_vptr [136] 
2018 @0x001b7db8 push_global_var [219] 
2019 @0x001b7dbc query_member_ptr [0] 
2020 @0x001b7dc0 assign_value [] 
2021 @0x001b7dc4 push_global_vptr [135] 
2022 @0x001b7dc8 inc_value [1] 
2023 @0x001b7dcc pop_stack [] 
2024 @0x001b7dd0 push_shared_string [2778] 'SF_CANT_LOOK_ON'
2025 @0x001b7dd4 call_in_efun [139, 1] tolower
2026 @0x001b7dd8 push_global_vptr [220] 
2027 @0x001b7ddc assign_value [] 
2028 @0x001b7de0 pop_stack [] 
2029 @0x001b7de4 push_global_vptr [135] 
2030 @0x001b7de8 convert_to_value [] 
2031 @0x001b7dec push_global_vptr [136] 
2032 @0x001b7df0 push_global_var [220] 
2033 @0x001b7df4 query_member_ptr [0] 
2034 @0x001b7df8 assign_value [] 
2035 @0x001b7dfc push_global_vptr [135] 
2036 @0x001b7e00 inc_value [1] 
2037 @0x001b7e04 pop_stack [] 
2038 @0x001b7e08 push_shared_string [2779] 'SF_UPGRADE_EQUIPMENT_RATE'
2039 @0x001b7e0c call_in_efun [139, 1] tolower
2040 @0x001b7e10 push_global_vptr [221] 
2041 @0x001b7e14 assign_value [] 
2042 @0x001b7e18 pop_stack [] 
2043 @0x001b7e1c push_global_vptr [135] 
2044 @0x001b7e20 convert_to_value [] 
2045 @0x001b7e24 push_global_vptr [136] 
2046 @0x001b7e28 push_global_var [221] 
2047 @0x001b7e2c query_member_ptr [0] 
2048 @0x001b7e30 assign_value [] 
2049 @0x001b7e34 push_global_vptr [135] 
2050 @0x001b7e38 inc_value [1] 
2051 @0x001b7e3c pop_stack [] 
2052 @0x001b7e40 push_shared_string [2780] 'SF_ALCHEMY_NEIDAN_RATE'
2053 @0x001b7e44 call_in_efun [139, 1] tolower
2054 @0x001b7e48 push_global_vptr [222] 
2055 @0x001b7e4c assign_value [] 
2056 @0x001b7e50 pop_stack [] 
2057 @0x001b7e54 push_global_vptr [135] 
2058 @0x001b7e58 convert_to_value [] 
2059 @0x001b7e5c push_global_vptr [136] 
2060 @0x001b7e60 push_global_var [222] 
2061 @0x001b7e64 query_member_ptr [0] 
2062 @0x001b7e68 assign_value [] 
2063 @0x001b7e6c push_global_vptr [135] 
2064 @0x001b7e70 inc_value [1] 
2065 @0x001b7e74 pop_stack [] 
2066 @0x001b7e78 push_shared_string [2781] 'SF_UPGRADE_PET_ITEM_RATE'
2067 @0x001b7e7c call_in_efun [139, 1] tolower
2068 @0x001b7e80 push_global_vptr [223] 
2069 @0x001b7e84 assign_value [] 
2070 @0x001b7e88 pop_stack [] 
2071 @0x001b7e8c push_global_vptr [135] 
2072 @0x001b7e90 convert_to_value [] 
2073 @0x001b7e94 push_global_vptr [136] 
2074 @0x001b7e98 push_global_var [223] 
2075 @0x001b7e9c query_member_ptr [0] 
2076 @0x001b7ea0 assign_value [] 
2077 @0x001b7ea4 push_global_vptr [135] 
2078 @0x001b7ea8 inc_value [1] 
2079 @0x001b7eac pop_stack [] 
2080 @0x001b7eb0 push_shared_string [2782] 'SF_THIEF_RATE'
2081 @0x001b7eb4 call_in_efun [139, 1] tolower
2082 @0x001b7eb8 push_global_vptr [224] 
2083 @0x001b7ebc assign_value [] 
2084 @0x001b7ec0 pop_stack [] 
2085 @0x001b7ec4 push_global_vptr [135] 
2086 @0x001b7ec8 convert_to_value [] 
2087 @0x001b7ecc push_global_vptr [136] 
2088 @0x001b7ed0 push_global_var [224] 
2089 @0x001b7ed4 query_member_ptr [0] 
2090 @0x001b7ed8 assign_value [] 
2091 @0x001b7edc push_global_vptr [135] 
2092 @0x001b7ee0 inc_value [1] 
2093 @0x001b7ee4 pop_stack [] 
2094 @0x001b7ee8 push_shared_string [2783] 'SF_LEADER_RATE'
2095 @0x001b7eec call_in_efun [139, 1] tolower
2096 @0x001b7ef0 push_global_vptr [225] 
2097 @0x001b7ef4 assign_value [] 
2098 @0x001b7ef8 pop_stack [] 
2099 @0x001b7efc push_global_vptr [135] 
2100 @0x001b7f00 convert_to_value [] 
2101 @0x001b7f04 push_global_vptr [136] 
2102 @0x001b7f08 push_global_var [225] 
2103 @0x001b7f0c query_member_ptr [0] 
2104 @0x001b7f10 assign_value [] 
2105 @0x001b7f14 push_global_vptr [135] 
2106 @0x001b7f18 inc_value [1] 
2107 @0x001b7f1c pop_stack [] 
2108 @0x001b7f20 push_shared_string [2784] 'SF_BABY_RATE'
2109 @0x001b7f24 call_in_efun [139, 1] tolower
2110 @0x001b7f28 push_global_vptr [226] 
2111 @0x001b7f2c assign_value [] 
2112 @0x001b7f30 pop_stack [] 
2113 @0x001b7f34 push_global_vptr [135] 
2114 @0x001b7f38 convert_to_value [] 
2115 @0x001b7f3c push_global_vptr [136] 
2116 @0x001b7f40 push_global_var [226] 
2117 @0x001b7f44 query_member_ptr [0] 
2118 @0x001b7f48 assign_value [] 
2119 @0x001b7f4c push_global_vptr [135] 
2120 @0x001b7f50 inc_value [1] 
2121 @0x001b7f54 pop_stack [] 
2122 @0x001b7f58 push_shared_string [2785] 'SF_PET_SKILL'
2123 @0x001b7f5c call_in_efun [139, 1] tolower
2124 @0x001b7f60 push_global_vptr [227] 
2125 @0x001b7f64 assign_value [] 
2126 @0x001b7f68 pop_stack [] 
2127 @0x001b7f6c push_global_vptr [135] 
2128 @0x001b7f70 convert_to_value [] 
2129 @0x001b7f74 push_global_vptr [136] 
2130 @0x001b7f78 push_global_var [227] 
2131 @0x001b7f7c query_member_ptr [0] 
2132 @0x001b7f80 assign_value [] 
2133 @0x001b7f84 push_global_vptr [135] 
2134 @0x001b7f88 inc_value [1] 
2135 @0x001b7f8c pop_stack [] 
2136 @0x001b7f90 push_shared_string [2786] 'SF_TEMPLATE_RATE'
2137 @0x001b7f94 call_in_efun [139, 1] tolower
2138 @0x001b7f98 push_global_vptr [228] 
2139 @0x001b7f9c assign_value [] 
2140 @0x001b7fa0 pop_stack [] 
2141 @0x001b7fa4 push_global_vptr [135] 
2142 @0x001b7fa8 convert_to_value [] 
2143 @0x001b7fac push_global_vptr [136] 
2144 @0x001b7fb0 push_global_var [228] 
2145 @0x001b7fb4 query_member_ptr [0] 
2146 @0x001b7fb8 assign_value [] 
2147 @0x001b7fbc push_global_vptr [135] 
2148 @0x001b7fc0 inc_value [1] 
2149 @0x001b7fc4 pop_stack [] 
2150 @0x001b7fc8 push_shared_string [2787] 'SF_REQUEST_COUPON_BONUS'
2151 @0x001b7fcc call_in_efun [139, 1] tolower
2152 @0x001b7fd0 push_global_vptr [229] 
2153 @0x001b7fd4 assign_value [] 
2154 @0x001b7fd8 pop_stack [] 
2155 @0x001b7fdc push_global_vptr [135] 
2156 @0x001b7fe0 convert_to_value [] 
2157 @0x001b7fe4 push_global_vptr [136] 
2158 @0x001b7fe8 push_global_var [229] 
2159 @0x001b7fec query_member_ptr [0] 
2160 @0x001b7ff0 assign_value [] 
2161 @0x001b7ff4 push_global_vptr [135] 
2162 @0x001b7ff8 inc_value [1] 
2163 @0x001b7ffc pop_stack [] 
2164 @0x001b8000 push_shared_string [2788] 'SF_COST_COUPON_BONUS'
2165 @0x001b8004 call_in_efun [139, 1] tolower
2166 @0x001b8008 push_global_vptr [230] 
2167 @0x001b800c assign_value [] 
2168 @0x001b8010 pop_stack [] 
2169 @0x001b8014 push_global_vptr [135] 
2170 @0x001b8018 convert_to_value [] 
2171 @0x001b801c push_global_vptr [136] 
2172 @0x001b8020 push_global_var [230] 
2173 @0x001b8024 query_member_ptr [0] 
2174 @0x001b8028 assign_value [] 
2175 @0x001b802c push_global_vptr [135] 
2176 @0x001b8030 inc_value [1] 
2177 @0x001b8034 pop_stack [] 
2178 @0x001b8038 push_shared_string [2789] 'SF_CANNOT_OPERATE_STORE'
2179 @0x001b803c call_in_efun [139, 1] tolower
2180 @0x001b8040 push_global_vptr [231] 
2181 @0x001b8044 assign_value [] 
2182 @0x001b8048 pop_stack [] 
2183 @0x001b804c push_global_vptr [135] 
2184 @0x001b8050 convert_to_value [] 
2185 @0x001b8054 push_global_vptr [136] 
2186 @0x001b8058 push_global_var [231] 
2187 @0x001b805c query_member_ptr [0] 
2188 @0x001b8060 assign_value [] 
2189 @0x001b8064 push_global_vptr [135] 
2190 @0x001b8068 inc_value [1] 
2191 @0x001b806c pop_stack [] 
2192 @0x001b8070 push_shared_string [2790] 'SF_CANNOT_BUY'
2193 @0x001b8074 call_in_efun [139, 1] tolower
2194 @0x001b8078 push_global_vptr [232] 
2195 @0x001b807c assign_value [] 
2196 @0x001b8080 pop_stack [] 
2197 @0x001b8084 push_global_vptr [135] 
2198 @0x001b8088 convert_to_value [] 
2199 @0x001b808c push_global_vptr [136] 
2200 @0x001b8090 push_global_var [232] 
2201 @0x001b8094 query_member_ptr [0] 
2202 @0x001b8098 assign_value [] 
2203 @0x001b809c push_global_vptr [135] 
2204 @0x001b80a0 inc_value [1] 
2205 @0x001b80a4 pop_stack [] 
2206 @0x001b80a8 push_shared_string [2791] 'SF_CANNOT_SELL'
2207 @0x001b80ac call_in_efun [139, 1] tolower
2208 @0x001b80b0 push_global_vptr [233] 
2209 @0x001b80b4 assign_value [] 
2210 @0x001b80b8 pop_stack [] 
2211 @0x001b80bc push_global_vptr [135] 
2212 @0x001b80c0 convert_to_value [] 
2213 @0x001b80c4 push_global_vptr [136] 
2214 @0x001b80c8 push_global_var [233] 
2215 @0x001b80cc query_member_ptr [0] 
2216 @0x001b80d0 assign_value [] 
2217 @0x001b80d4 push_global_vptr [135] 
2218 @0x001b80d8 inc_value [1] 
2219 @0x001b80dc pop_stack [] 
2220 @0x001b80e0 push_shared_string [2792] 'SF_CANNOT_REPAIR'
2221 @0x001b80e4 call_in_efun [139, 1] tolower
2222 @0x001b80e8 push_global_vptr [234] 
2223 @0x001b80ec assign_value [] 
2224 @0x001b80f0 pop_stack [] 
2225 @0x001b80f4 push_global_vptr [135] 
2226 @0x001b80f8 convert_to_value [] 
2227 @0x001b80fc push_global_vptr [136] 
2228 @0x001b8100 push_global_var [234] 
2229 @0x001b8104 query_member_ptr [0] 
2230 @0x001b8108 assign_value [] 
2231 @0x001b810c push_global_vptr [135] 
2232 @0x001b8110 inc_value [1] 
2233 @0x001b8114 pop_stack [] 
2234 @0x001b8118 push_shared_string [2793] 'SF_CANNOT_WITHDRAW'
2235 @0x001b811c call_in_efun [139, 1] tolower
2236 @0x001b8120 push_global_vptr [235] 
2237 @0x001b8124 assign_value [] 
2238 @0x001b8128 pop_stack [] 
2239 @0x001b812c push_global_vptr [135] 
2240 @0x001b8130 convert_to_value [] 
2241 @0x001b8134 push_global_vptr [136] 
2242 @0x001b8138 push_global_var [235] 
2243 @0x001b813c query_member_ptr [0] 
2244 @0x001b8140 assign_value [] 
2245 @0x001b8144 push_global_vptr [135] 
2246 @0x001b8148 inc_value [1] 
2247 @0x001b814c pop_stack [] 
2248 @0x001b8150 push_shared_string [2794] 'SF_CANNOT_DEPOSIT'
2249 @0x001b8154 call_in_efun [139, 1] tolower
2250 @0x001b8158 push_global_vptr [236] 
2251 @0x001b815c assign_value [] 
2252 @0x001b8160 pop_stack [] 
2253 @0x001b8164 push_global_vptr [135] 
2254 @0x001b8168 convert_to_value [] 
2255 @0x001b816c push_global_vptr [136] 
2256 @0x001b8170 push_global_var [236] 
2257 @0x001b8174 query_member_ptr [0] 
2258 @0x001b8178 assign_value [] 
2259 @0x001b817c push_global_vptr [135] 
2260 @0x001b8180 inc_value [1] 
2261 @0x001b8184 pop_stack [] 
2262 @0x001b8188 push_shared_string [2795] 'SF_IGNORE_DISTANCE'
2263 @0x001b818c call_in_efun [139, 1] tolower
2264 @0x001b8190 push_global_vptr [237] 
2265 @0x001b8194 assign_value [] 
2266 @0x001b8198 pop_stack [] 
2267 @0x001b819c push_global_vptr [135] 
2268 @0x001b81a0 convert_to_value [] 
2269 @0x001b81a4 push_global_vptr [136] 
2270 @0x001b81a8 push_global_var [237] 
2271 @0x001b81ac query_member_ptr [0] 
2272 @0x001b81b0 assign_value [] 
2273 @0x001b81b4 push_global_vptr [135] 
2274 @0x001b81b8 inc_value [1] 
2275 @0x001b81bc pop_stack [] 
2276 @0x001b81c0 push_shared_string [2796] 'SF_MOVE'
2277 @0x001b81c4 call_in_efun [139, 1] tolower
2278 @0x001b81c8 push_global_vptr [238] 
2279 @0x001b81cc assign_value [] 
2280 @0x001b81d0 pop_stack [] 
2281 @0x001b81d4 push_global_vptr [135] 
2282 @0x001b81d8 convert_to_value [] 
2283 @0x001b81dc push_global_vptr [136] 
2284 @0x001b81e0 push_global_var [238] 
2285 @0x001b81e4 query_member_ptr [0] 
2286 @0x001b81e8 assign_value [] 
2287 @0x001b81ec push_global_vptr [135] 
2288 @0x001b81f0 inc_value [1] 
2289 @0x001b81f4 pop_stack [] 
2290 @0x001b81f8 push_shared_string [2797] 'SF_PREPARE_SWITCH_SERVER'
2291 @0x001b81fc call_in_efun [139, 1] tolower
2292 @0x001b8200 push_global_vptr [239] 
2293 @0x001b8204 assign_value [] 
2294 @0x001b8208 pop_stack [] 
2295 @0x001b820c push_global_vptr [135] 
2296 @0x001b8210 convert_to_value [] 
2297 @0x001b8214 push_global_vptr [136] 
2298 @0x001b8218 push_global_var [239] 
2299 @0x001b821c query_member_ptr [0] 
2300 @0x001b8220 assign_value [] 
2301 @0x001b8224 push_global_vptr [135] 
2302 @0x001b8228 inc_value [1] 
2303 @0x001b822c pop_stack [] 
2304 @0x001b8230 push_shared_string [2798] 'SF_SWITCH_SERVER'
2305 @0x001b8234 call_in_efun [139, 1] tolower
2306 @0x001b8238 push_global_vptr [240] 
2307 @0x001b823c assign_value [] 
2308 @0x001b8240 pop_stack [] 
2309 @0x001b8244 push_global_vptr [135] 
2310 @0x001b8248 convert_to_value [] 
2311 @0x001b824c push_global_vptr [136] 
2312 @0x001b8250 push_global_var [240] 
2313 @0x001b8254 query_member_ptr [0] 
2314 @0x001b8258 assign_value [] 
2315 @0x001b825c push_global_vptr [135] 
2316 @0x001b8260 inc_value [1] 
2317 @0x001b8264 pop_stack [] 
2318 @0x001b8268 push_shared_string [2799] 'SF_NOTICE_OTHERS_APPEAR'
2319 @0x001b826c call_in_efun [139, 1] tolower
2320 @0x001b8270 push_global_vptr [241] 
2321 @0x001b8274 assign_value [] 
2322 @0x001b8278 pop_stack [] 
2323 @0x001b827c push_global_vptr [135] 
2324 @0x001b8280 convert_to_value [] 
2325 @0x001b8284 push_global_vptr [136] 
2326 @0x001b8288 push_global_var [241] 
2327 @0x001b828c query_member_ptr [0] 
2328 @0x001b8290 assign_value [] 
2329 @0x001b8294 push_global_vptr [135] 
2330 @0x001b8298 inc_value [1] 
2331 @0x001b829c pop_stack [] 
2332 @0x001b82a0 push_shared_string [2800] 'SF_NOTICE_ME_APPEAR'
2333 @0x001b82a4 call_in_efun [139, 1] tolower
2334 @0x001b82a8 push_global_vptr [242] 
2335 @0x001b82ac assign_value [] 
2336 @0x001b82b0 pop_stack [] 
2337 @0x001b82b4 push_global_vptr [135] 
2338 @0x001b82b8 convert_to_value [] 
2339 @0x001b82bc push_global_vptr [136] 
2340 @0x001b82c0 push_global_var [242] 
2341 @0x001b82c4 query_member_ptr [0] 
2342 @0x001b82c8 assign_value [] 
2343 @0x001b82cc push_global_vptr [135] 
2344 @0x001b82d0 inc_value [1] 
2345 @0x001b82d4 pop_stack [] 
2346 @0x001b82d8 push_shared_string [2801] 'SF_SMELT_EQUIPMENT_RATE'
2347 @0x001b82dc call_in_efun [139, 1] tolower
2348 @0x001b82e0 push_global_vptr [243] 
2349 @0x001b82e4 assign_value [] 
2350 @0x001b82e8 pop_stack [] 
2351 @0x001b82ec push_global_vptr [135] 
2352 @0x001b82f0 convert_to_value [] 
2353 @0x001b82f4 push_global_vptr [136] 
2354 @0x001b82f8 push_global_var [243] 
2355 @0x001b82fc query_member_ptr [0] 
2356 @0x001b8300 assign_value [] 
2357 @0x001b8304 push_global_vptr [135] 
2358 @0x001b8308 inc_value [1] 
2359 @0x001b830c pop_stack [] 
2360 @0x001b8310 push_shared_string [2802] 'SF_SPLIT_EQUIPMENT_RATE'
2361 @0x001b8314 call_in_efun [139, 1] tolower
2362 @0x001b8318 push_global_vptr [244] 
2363 @0x001b831c assign_value [] 
2364 @0x001b8320 pop_stack [] 
2365 @0x001b8324 push_global_vptr [135] 
2366 @0x001b8328 convert_to_value [] 
2367 @0x001b832c push_global_vptr [136] 
2368 @0x001b8330 push_global_var [244] 
2369 @0x001b8334 query_member_ptr [0] 
2370 @0x001b8338 assign_value [] 
2371 @0x001b833c push_global_vptr [135] 
2372 @0x001b8340 inc_value [1] 
2373 @0x001b8344 pop_stack [] 
2374 @0x001b8348 push_shared_string [2803] 'SF_SWITCH_EQUIPMENT_RATE'
2375 @0x001b834c call_in_efun [139, 1] tolower
2376 @0x001b8350 push_global_vptr [245] 
2377 @0x001b8354 assign_value [] 
2378 @0x001b8358 pop_stack [] 
2379 @0x001b835c push_global_vptr [135] 
2380 @0x001b8360 convert_to_value [] 
2381 @0x001b8364 push_global_vptr [136] 
2382 @0x001b8368 push_global_var [245] 
2383 @0x001b836c query_member_ptr [0] 
2384 @0x001b8370 assign_value [] 
2385 @0x001b8374 push_global_vptr [135] 
2386 @0x001b8378 inc_value [1] 
2387 @0x001b837c pop_stack [] 
2388 @0x001b8380 push_shared_string [2804] 'SF_TRY_JOIN_RAID'
2389 @0x001b8384 call_in_efun [139, 1] tolower
2390 @0x001b8388 push_global_vptr [246] 
2391 @0x001b838c assign_value [] 
2392 @0x001b8390 pop_stack [] 
2393 @0x001b8394 push_global_vptr [135] 
2394 @0x001b8398 convert_to_value [] 
2395 @0x001b839c push_global_vptr [136] 
2396 @0x001b83a0 push_global_var [246] 
2397 @0x001b83a4 query_member_ptr [0] 
2398 @0x001b83a8 assign_value [] 
2399 @0x001b83ac push_global_vptr [135] 
2400 @0x001b83b0 inc_value [1] 
2401 @0x001b83b4 pop_stack [] 
2402 @0x001b83b8 push_shared_string [2805] 'SF_RECONNECTED'
2403 @0x001b83bc call_in_efun [139, 1] tolower
2404 @0x001b83c0 push_global_vptr [247] 
2405 @0x001b83c4 assign_value [] 
2406 @0x001b83c8 pop_stack [] 
2407 @0x001b83cc push_global_vptr [135] 
2408 @0x001b83d0 convert_to_value [] 
2409 @0x001b83d4 push_global_vptr [136] 
2410 @0x001b83d8 push_global_var [247] 
2411 @0x001b83dc query_member_ptr [0] 
2412 @0x001b83e0 assign_value [] 
2413 @0x001b83e4 push_global_vptr [135] 
2414 @0x001b83e8 inc_value [1] 
2415 @0x001b83ec pop_stack [] 
2416 @0x001b83f0 push_shared_string [2806] 'SF_SET_SETTINGS'
2417 @0x001b83f4 call_in_efun [139, 1] tolower
2418 @0x001b83f8 push_global_vptr [248] 
2419 @0x001b83fc assign_value [] 
2420 @0x001b8400 pop_stack [] 
2421 @0x001b8404 push_global_vptr [135] 
2422 @0x001b8408 convert_to_value [] 
2423 @0x001b840c push_global_vptr [136] 
2424 @0x001b8410 push_global_var [248] 
2425 @0x001b8414 query_member_ptr [0] 
2426 @0x001b8418 assign_value [] 
2427 @0x001b841c push_global_vptr [135] 
2428 @0x001b8420 inc_value [1] 
2429 @0x001b8424 pop_stack [] 
2430 @0x001b8428 push_shared_string [2807] 'SF_GOT_ARTIFACT_EXP'
2431 @0x001b842c call_in_efun [139, 1] tolower
2432 @0x001b8430 push_global_vptr [249] 
2433 @0x001b8434 assign_value [] 
2434 @0x001b8438 pop_stack [] 
2435 @0x001b843c push_global_vptr [135] 
2436 @0x001b8440 convert_to_value [] 
2437 @0x001b8444 push_global_vptr [136] 
2438 @0x001b8448 push_global_var [249] 
2439 @0x001b844c query_member_ptr [0] 
2440 @0x001b8450 assign_value [] 
2441 @0x001b8454 push_global_vptr [135] 
2442 @0x001b8458 inc_value [1] 
2443 @0x001b845c pop_stack [] 
2444 @0x001b8460 push_shared_string [2808] 'SF_GOT_EXP'
2445 @0x001b8464 call_in_efun [139, 1] tolower
2446 @0x001b8468 push_global_vptr [250] 
2447 @0x001b846c assign_value [] 
2448 @0x001b8470 pop_stack [] 
2449 @0x001b8474 push_global_vptr [135] 
2450 @0x001b8478 convert_to_value [] 
2451 @0x001b847c push_global_vptr [136] 
2452 @0x001b8480 push_global_var [250] 
2453 @0x001b8484 query_member_ptr [0] 
2454 @0x001b8488 assign_value [] 
2455 @0x001b848c push_global_vptr [135] 
2456 @0x001b8490 inc_value [1] 
2457 @0x001b8494 pop_stack [] 
2458 @0x001b8498 push_shared_string [2809] 'SF_SELECT_SKILL_TARGETS'
2459 @0x001b849c call_in_efun [139, 1] tolower
2460 @0x001b84a0 push_global_vptr [251] 
2461 @0x001b84a4 assign_value [] 
2462 @0x001b84a8 pop_stack [] 
2463 @0x001b84ac push_global_vptr [135] 
2464 @0x001b84b0 convert_to_value [] 
2465 @0x001b84b4 push_global_vptr [136] 
2466 @0x001b84b8 push_global_var [251] 
2467 @0x001b84bc query_member_ptr [0] 
2468 @0x001b84c0 assign_value [] 
2469 @0x001b84c4 push_global_vptr [135] 
2470 @0x001b84c8 inc_value [1] 
2471 @0x001b84cc pop_stack [] 
2472 @0x001b84d0 push_shared_string [2810] 'SF_SKILL_RANGE'
2473 @0x001b84d4 call_in_efun [139, 1] tolower
2474 @0x001b84d8 push_global_vptr [252] 
2475 @0x001b84dc assign_value [] 
2476 @0x001b84e0 pop_stack [] 
2477 @0x001b84e4 push_global_vptr [135] 
2478 @0x001b84e8 convert_to_value [] 
2479 @0x001b84ec push_global_vptr [136] 
2480 @0x001b84f0 push_global_var [252] 
2481 @0x001b84f4 query_member_ptr [0] 
2482 @0x001b84f8 assign_value [] 
2483 @0x001b84fc push_global_vptr [135] 
2484 @0x001b8500 inc_value [1] 
2485 @0x001b8504 pop_stack [] 
2486 @0x001b8508 push_shared_string [2811] 'SF_SKILL_ROUND'
2487 @0x001b850c call_in_efun [139, 1] tolower
2488 @0x001b8510 push_global_vptr [253] 
2489 @0x001b8514 assign_value [] 
2490 @0x001b8518 pop_stack [] 
2491 @0x001b851c push_global_vptr [135] 
2492 @0x001b8520 convert_to_value [] 
2493 @0x001b8524 push_global_vptr [136] 
2494 @0x001b8528 push_global_var [253] 
2495 @0x001b852c query_member_ptr [0] 
2496 @0x001b8530 assign_value [] 
2497 @0x001b8534 push_global_vptr [135] 
2498 @0x001b8538 inc_value [1] 
2499 @0x001b853c pop_stack [] 
2500 @0x001b8540 push_shared_string [2812] 'SF_GOD_BOOK_SUCCESS_RATE'
2501 @0x001b8544 call_in_efun [139, 1] tolower
2502 @0x001b8548 push_global_vptr [254] 
2503 @0x001b854c assign_value [] 
2504 @0x001b8550 pop_stack [] 
2505 @0x001b8554 push_global_vptr [135] 
2506 @0x001b8558 convert_to_value [] 
2507 @0x001b855c push_global_vptr [136] 
2508 @0x001b8560 push_global_var [254] 
2509 @0x001b8564 query_member_ptr [0] 
2510 @0x001b8568 assign_value [] 
2511 @0x001b856c push_global_vptr [135] 
2512 @0x001b8570 inc_value [1] 
2513 @0x001b8574 pop_stack [] 
2514 @0x001b8578 push_shared_string [2813] 'SF_COMBAT_TRIGGER_RATE'
2515 @0x001b857c call_in_efun [139, 1] tolower
2516 @0x001b8580 push_global_vptr [255] 
2517 @0x001b8584 assign_value [] 
2518 @0x001b8588 pop_stack [] 
2519 @0x001b858c push_global_vptr [135] 
2520 @0x001b8590 convert_to_value [] 
2521 @0x001b8594 push_global_vptr [136] 
2522 @0x001b8598 push_global_var [255] 
2523 @0x001b859c query_member_ptr [0] 
2524 @0x001b85a0 assign_value [] 
2525 @0x001b85a4 push_global_vptr [135] 
2526 @0x001b85a8 inc_value [1] 
2527 @0x001b85ac pop_stack [] 
2528 @0x001b85b0 push_shared_string [2814] 'SF_CHANGE_ROOM'
2529 @0x001b85b4 call_in_efun [139, 1] tolower
2530 @0x001b85b8 push_global_vptr [256] 
2531 @0x001b85bc assign_value [] 
2532 @0x001b85c0 pop_stack [] 
2533 @0x001b85c4 push_global_vptr [135] 
2534 @0x001b85c8 convert_to_value [] 
2535 @0x001b85cc push_global_vptr [136] 
2536 @0x001b85d0 push_global_var [256] 
2537 @0x001b85d4 query_member_ptr [0] 
2538 @0x001b85d8 assign_value [] 
2539 @0x001b85dc push_global_vptr [135] 
2540 @0x001b85e0 inc_value [1] 
2541 @0x001b85e4 pop_stack [] 
2542 @0x001b85e8 push_shared_string [2815] 'SF_FINISH_CHAR_UPGRADE'
2543 @0x001b85ec call_in_efun [139, 1] tolower
2544 @0x001b85f0 push_global_vptr [257] 
2545 @0x001b85f4 assign_value [] 
2546 @0x001b85f8 pop_stack [] 
2547 @0x001b85fc push_global_vptr [135] 
2548 @0x001b8600 convert_to_value [] 
2549 @0x001b8604 push_global_vptr [136] 
2550 @0x001b8608 push_global_var [257] 
2551 @0x001b860c query_member_ptr [0] 
2552 @0x001b8610 assign_value [] 
2553 @0x001b8614 push_global_vptr [135] 
2554 @0x001b8618 inc_value [1] 
2555 @0x001b861c pop_stack [] 
2556 @0x001b8620 push_shared_string [2816] 'SF_CAST_ON_EX'
2557 @0x001b8624 call_in_efun [139, 1] tolower
2558 @0x001b8628 push_global_vptr [258] 
2559 @0x001b862c assign_value [] 
2560 @0x001b8630 pop_stack [] 
2561 @0x001b8634 push_global_vptr [135] 
2562 @0x001b8638 convert_to_value [] 
2563 @0x001b863c push_global_vptr [136] 
2564 @0x001b8640 push_global_var [258] 
2565 @0x001b8644 query_member_ptr [0] 
2566 @0x001b8648 assign_value [] 
2567 @0x001b864c push_global_vptr [135] 
2568 @0x001b8650 inc_value [1] 
2569 @0x001b8654 pop_stack [] 
2570 @0x001b8658 push_shared_string [2817] 'SF_CAPTURE'
2571 @0x001b865c call_in_efun [139, 1] tolower
2572 @0x001b8660 push_global_vptr [259] 
2573 @0x001b8664 assign_value [] 
2574 @0x001b8668 pop_stack [] 
2575 @0x001b866c push_global_vptr [135] 
2576 @0x001b8670 convert_to_value [] 
2577 @0x001b8674 push_global_vptr [136] 
2578 @0x001b8678 push_global_var [259] 
2579 @0x001b867c query_member_ptr [0] 
2580 @0x001b8680 assign_value [] 
2581 @0x001b8684 push_global_vptr [135] 
2582 @0x001b8688 inc_value [1] 
2583 @0x001b868c pop_stack [] 
2584 @0x001b8690 push_shared_string [2818] 'SF_ATTRIB_ASSIGNED'
2585 @0x001b8694 call_in_efun [139, 1] tolower
2586 @0x001b8698 push_global_vptr [260] 
2587 @0x001b869c assign_value [] 
2588 @0x001b86a0 pop_stack [] 
2589 @0x001b86a4 push_global_vptr [135] 
2590 @0x001b86a8 convert_to_value [] 
2591 @0x001b86ac push_global_vptr [136] 
2592 @0x001b86b0 push_global_var [260] 
2593 @0x001b86b4 query_member_ptr [0] 
2594 @0x001b86b8 assign_value [] 
2595 @0x001b86bc push_global_vptr [135] 
2596 @0x001b86c0 inc_value [1] 
2597 @0x001b86c4 pop_stack [] 
2598 @0x001b86c8 push_shared_string [2819] 'SF_GET_UPGRADE_CHAR'
2599 @0x001b86cc call_in_efun [139, 1] tolower
2600 @0x001b86d0 push_global_vptr [261] 
2601 @0x001b86d4 assign_value [] 
2602 @0x001b86d8 pop_stack [] 
2603 @0x001b86dc push_global_vptr [135] 
2604 @0x001b86e0 convert_to_value [] 
2605 @0x001b86e4 push_global_vptr [136] 
2606 @0x001b86e8 push_global_var [261] 
2607 @0x001b86ec query_member_ptr [0] 
2608 @0x001b86f0 assign_value [] 
2609 @0x001b86f4 push_global_vptr [135] 
2610 @0x001b86f8 inc_value [1] 
2611 @0x001b86fc pop_stack [] 
2612 @0x001b8700 push_shared_string [2820] 'SF_TASK_FINISHED'
2613 @0x001b8704 call_in_efun [139, 1] tolower
2614 @0x001b8708 push_global_vptr [262] 
2615 @0x001b870c assign_value [] 
2616 @0x001b8710 pop_stack [] 
2617 @0x001b8714 push_global_vptr [135] 
2618 @0x001b8718 convert_to_value [] 
2619 @0x001b871c push_global_vptr [136] 
2620 @0x001b8720 push_global_var [262] 
2621 @0x001b8724 query_member_ptr [0] 
2622 @0x001b8728 assign_value [] 
2623 @0x001b872c push_global_vptr [135] 
2624 @0x001b8730 inc_value [1] 
2625 @0x001b8734 pop_stack [] 
2626 @0x001b8738 push_shared_string [2821] 'SF_BE_HIT'
2627 @0x001b873c call_in_efun [139, 1] tolower
2628 @0x001b8740 push_global_vptr [263] 
2629 @0x001b8744 assign_value [] 
2630 @0x001b8748 pop_stack [] 
2631 @0x001b874c push_global_vptr [135] 
2632 @0x001b8750 convert_to_value [] 
2633 @0x001b8754 push_global_vptr [136] 
2634 @0x001b8758 push_global_var [263] 
2635 @0x001b875c query_member_ptr [0] 
2636 @0x001b8760 assign_value [] 
2637 @0x001b8764 push_global_vptr [135] 
2638 @0x001b8768 inc_value [1] 
2639 @0x001b876c pop_stack [] 
2640 @0x001b8770 push_shared_string [2822] 'SF_GOT_POT'
2641 @0x001b8774 call_in_efun [139, 1] tolower
2642 @0x001b8778 push_global_vptr [264] 
2643 @0x001b877c assign_value [] 
2644 @0x001b8780 pop_stack [] 
2645 @0x001b8784 push_global_vptr [135] 
2646 @0x001b8788 convert_to_value [] 
2647 @0x001b878c push_global_vptr [136] 
2648 @0x001b8790 push_global_var [264] 
2649 @0x001b8794 query_member_ptr [0] 
2650 @0x001b8798 assign_value [] 
2651 @0x001b879c push_global_vptr [135] 
2652 @0x001b87a0 inc_value [1] 
2653 @0x001b87a4 pop_stack [] 
2654 @0x001b87a8 push_shared_string [2823] 'SF_GOT_TAO'
2655 @0x001b87ac call_in_efun [139, 1] tolower
2656 @0x001b87b0 push_global_vptr [265] 
2657 @0x001b87b4 assign_value [] 
2658 @0x001b87b8 pop_stack [] 
2659 @0x001b87bc push_global_vptr [135] 
2660 @0x001b87c0 convert_to_value [] 
2661 @0x001b87c4 push_global_vptr [136] 
2662 @0x001b87c8 push_global_var [265] 
2663 @0x001b87cc query_member_ptr [0] 
2664 @0x001b87d0 assign_value [] 
2665 @0x001b87d4 push_global_vptr [135] 
2666 @0x001b87d8 inc_value [1] 
2667 @0x001b87dc pop_stack [] 
2668 @0x001b87e0 push_shared_string [2824] 'SF_PUT_CHILD'
2669 @0x001b87e4 call_in_efun [139, 1] tolower
2670 @0x001b87e8 push_global_vptr [266] 
2671 @0x001b87ec assign_value [] 
2672 @0x001b87f0 pop_stack [] 
2673 @0x001b87f4 push_global_vptr [135] 
2674 @0x001b87f8 convert_to_value [] 
2675 @0x001b87fc push_global_vptr [136] 
2676 @0x001b8800 push_global_var [266] 
2677 @0x001b8804 query_member_ptr [0] 
2678 @0x001b8808 assign_value [] 
2679 @0x001b880c push_global_vptr [135] 
2680 @0x001b8810 inc_value [1] 
2681 @0x001b8814 pop_stack [] 
2682 @0x001b8818 push_shared_string [2825] 'SF_GOT_SCORE'
2683 @0x001b881c call_in_efun [139, 1] tolower
2684 @0x001b8820 push_global_vptr [267] 
2685 @0x001b8824 assign_value [] 
2686 @0x001b8828 pop_stack [] 
2687 @0x001b882c push_global_vptr [135] 
2688 @0x001b8830 convert_to_value [] 
2689 @0x001b8834 push_global_vptr [136] 
2690 @0x001b8838 push_global_var [267] 
2691 @0x001b883c query_member_ptr [0] 
2692 @0x001b8840 assign_value [] 
2693 @0x001b8844 push_global_vptr [135] 
2694 @0x001b8848 inc_value [1] 
2695 @0x001b884c pop_stack [] 
2696 @0x001b8850 push_shared_string [2826] 'SF_GOT_GLORY'
2697 @0x001b8854 call_in_efun [139, 1] tolower
2698 @0x001b8858 push_global_vptr [268] 
2699 @0x001b885c assign_value [] 
2700 @0x001b8860 pop_stack [] 
2701 @0x001b8864 push_global_vptr [135] 
2702 @0x001b8868 convert_to_value [] 
2703 @0x001b886c push_global_vptr [136] 
2704 @0x001b8870 push_global_var [268] 
2705 @0x001b8874 query_member_ptr [0] 
2706 @0x001b8878 assign_value [] 
2707 @0x001b887c push_global_vptr [135] 
2708 @0x001b8880 inc_value [1] 
2709 @0x001b8884 pop_stack [] 
2710 @0x001b8888 push_shared_string [2827] 'SF_USE_ITEM'
2711 @0x001b888c call_in_efun [139, 1] tolower
2712 @0x001b8890 push_global_vptr [269] 
2713 @0x001b8894 assign_value [] 
2714 @0x001b8898 pop_stack [] 
2715 @0x001b889c push_global_vptr [135] 
2716 @0x001b88a0 convert_to_value [] 
2717 @0x001b88a4 push_global_vptr [136] 
2718 @0x001b88a8 push_global_var [269] 
2719 @0x001b88ac query_member_ptr [0] 
2720 @0x001b88b0 assign_value [] 
2721 @0x001b88b4 push_global_vptr [135] 
2722 @0x001b88b8 inc_value [1] 
2723 @0x001b88bc pop_stack [] 
2724 @0x001b88c0 push_shared_string [2828] 'SF_GOT_MARTIAL'
2725 @0x001b88c4 call_in_efun [139, 1] tolower
2726 @0x001b88c8 push_global_vptr [270] 
2727 @0x001b88cc assign_value [] 
2728 @0x001b88d0 pop_stack [] 
2729 @0x001b88d4 push_global_vptr [135] 
2730 @0x001b88d8 convert_to_value [] 
2731 @0x001b88dc push_global_vptr [136] 
2732 @0x001b88e0 push_global_var [270] 
2733 @0x001b88e4 query_member_ptr [0] 
2734 @0x001b88e8 assign_value [] 
2735 @0x001b88ec push_global_vptr [135] 
2736 @0x001b88f0 inc_value [1] 
2737 @0x001b88f4 pop_stack [] 
2738 @0x001b88f8 push_shared_string [2829] 'SF_GATHER'
2739 @0x001b88fc call_in_efun [139, 1] tolower
2740 @0x001b8900 push_global_vptr [271] 
2741 @0x001b8904 assign_value [] 
2742 @0x001b8908 pop_stack [] 
2743 @0x001b890c push_global_vptr [135] 
2744 @0x001b8910 convert_to_value [] 
2745 @0x001b8914 push_global_vptr [136] 
2746 @0x001b8918 push_global_var [271] 
2747 @0x001b891c query_member_ptr [0] 
2748 @0x001b8920 assign_value [] 
2749 @0x001b8924 push_global_vptr [135] 
2750 @0x001b8928 inc_value [1] 
2751 @0x001b892c pop_stack [] 
2752 @0x001b8930 push_shared_string [2830] 'SF_QUIT_LOOK_ON'
2753 @0x001b8934 call_in_efun [139, 1] tolower
2754 @0x001b8938 push_global_vptr [272] 
2755 @0x001b893c assign_value [] 
2756 @0x001b8940 pop_stack [] 
2757 @0x001b8944 push_global_vptr [135] 
2758 @0x001b8948 convert_to_value [] 
2759 @0x001b894c push_global_vptr [136] 
2760 @0x001b8950 push_global_var [272] 
2761 @0x001b8954 query_member_ptr [0] 
2762 @0x001b8958 assign_value [] 
2763 @0x001b895c push_global_vptr [135] 
2764 @0x001b8960 inc_value [1] 
2765 @0x001b8964 pop_stack [] 
2766 @0x001b8968 push_shared_string [2831] 'SF_APPLY_STATUS'
2767 @0x001b896c call_in_efun [139, 1] tolower
2768 @0x001b8970 push_global_vptr [273] 
2769 @0x001b8974 assign_value [] 
2770 @0x001b8978 pop_stack [] 
2771 @0x001b897c push_global_vptr [135] 
2772 @0x001b8980 convert_to_value [] 
2773 @0x001b8984 push_global_vptr [136] 
2774 @0x001b8988 push_global_var [273] 
2775 @0x001b898c query_member_ptr [0] 
2776 @0x001b8990 assign_value [] 
2777 @0x001b8994 push_global_vptr [135] 
2778 @0x001b8998 inc_value [1] 
2779 @0x001b899c pop_stack [] 
2780 @0x001b89a0 push_shared_string [2832] 'SF_APPLY_STATUS_ALREADY'
2781 @0x001b89a4 call_in_efun [139, 1] tolower
2782 @0x001b89a8 push_global_vptr [274] 
2783 @0x001b89ac assign_value [] 
2784 @0x001b89b0 pop_stack [] 
2785 @0x001b89b4 push_global_vptr [135] 
2786 @0x001b89b8 convert_to_value [] 
2787 @0x001b89bc push_global_vptr [136] 
2788 @0x001b89c0 push_global_var [274] 
2789 @0x001b89c4 query_member_ptr [0] 
2790 @0x001b89c8 assign_value [] 
2791 @0x001b89cc push_global_vptr [135] 
2792 @0x001b89d0 inc_value [1] 
2793 @0x001b89d4 pop_stack [] 
2794 @0x001b89d8 push_shared_string [2833] 'SF_REMOVE_STATUS'
2795 @0x001b89dc call_in_efun [139, 1] tolower
2796 @0x001b89e0 push_global_vptr [275] 
2797 @0x001b89e4 assign_value [] 
2798 @0x001b89e8 pop_stack [] 
2799 @0x001b89ec push_global_vptr [135] 
2800 @0x001b89f0 convert_to_value [] 
2801 @0x001b89f4 push_global_vptr [136] 
2802 @0x001b89f8 push_global_var [275] 
2803 @0x001b89fc query_member_ptr [0] 
2804 @0x001b8a00 assign_value [] 
2805 @0x001b8a04 push_global_vptr [135] 
2806 @0x001b8a08 inc_value [1] 
2807 @0x001b8a0c pop_stack [] 
2808 @0x001b8a10 push_shared_string [2834] 'SF_ACTION_CUSTOM_MSG'
2809 @0x001b8a14 call_in_efun [139, 1] tolower
2810 @0x001b8a18 push_global_vptr [276] 
2811 @0x001b8a1c assign_value [] 
2812 @0x001b8a20 pop_stack [] 
2813 @0x001b8a24 push_global_vptr [135] 
2814 @0x001b8a28 convert_to_value [] 
2815 @0x001b8a2c push_global_vptr [136] 
2816 @0x001b8a30 push_global_var [276] 
2817 @0x001b8a34 query_member_ptr [0] 
2818 @0x001b8a38 assign_value [] 
2819 @0x001b8a3c push_global_vptr [135] 
2820 @0x001b8a40 inc_value [1] 
2821 @0x001b8a44 pop_stack [] 
2822 @0x001b8a48 push_shared_string [2835] 'SF_TRIGGER_CHILD_COMBAT'
2823 @0x001b8a4c call_in_efun [139, 1] tolower
2824 @0x001b8a50 push_global_vptr [277] 
2825 @0x001b8a54 assign_value [] 
2826 @0x001b8a58 pop_stack [] 
2827 @0x001b8a5c push_global_vptr [135] 
2828 @0x001b8a60 convert_to_value [] 
2829 @0x001b8a64 push_global_vptr [136] 
2830 @0x001b8a68 push_global_var [277] 
2831 @0x001b8a6c query_member_ptr [0] 
2832 @0x001b8a70 assign_value [] 
2833 @0x001b8a74 push_global_vptr [135] 
2834 @0x001b8a78 inc_value [1] 
2835 @0x001b8a7c pop_stack [] 
2836 @0x001b8a80 push_shared_string [2836] 'SF_START_STALL'
2837 @0x001b8a84 call_in_efun [139, 1] tolower
2838 @0x001b8a88 push_global_vptr [278] 
2839 @0x001b8a8c assign_value [] 
2840 @0x001b8a90 pop_stack [] 
2841 @0x001b8a94 push_global_vptr [135] 
2842 @0x001b8a98 convert_to_value [] 
2843 @0x001b8a9c push_global_vptr [136] 
2844 @0x001b8aa0 push_global_var [278] 
2845 @0x001b8aa4 query_member_ptr [0] 
2846 @0x001b8aa8 assign_value [] 
2847 @0x001b8aac push_global_vptr [135] 
2848 @0x001b8ab0 inc_value [1] 
2849 @0x001b8ab4 pop_stack [] 
2850 @0x001b8ab8 push_shared_string [2837] 'SF_COST_DAMAGE'
2851 @0x001b8abc call_in_efun [139, 1] tolower
2852 @0x001b8ac0 push_global_vptr [279] 
2853 @0x001b8ac4 assign_value [] 
2854 @0x001b8ac8 pop_stack [] 
2855 @0x001b8acc push_global_vptr [135] 
2856 @0x001b8ad0 convert_to_value [] 
2857 @0x001b8ad4 push_global_vptr [136] 
2858 @0x001b8ad8 push_global_var [279] 
2859 @0x001b8adc query_member_ptr [0] 
2860 @0x001b8ae0 assign_value [] 
2861 @0x001b8ae4 push_global_vptr [135] 
2862 @0x001b8ae8 inc_value [1] 
2863 @0x001b8aec pop_stack [] 
2864 @0x001b8af0 push_shared_string [2838] 'SF_MAKE_DAMAGE'
2865 @0x001b8af4 call_in_efun [139, 1] tolower
2866 @0x001b8af8 push_global_vptr [280] 
2867 @0x001b8afc assign_value [] 
2868 @0x001b8b00 pop_stack [] 
2869 @0x001b8b04 push_global_vptr [135] 
2870 @0x001b8b08 convert_to_value [] 
2871 @0x001b8b0c push_global_vptr [136] 
2872 @0x001b8b10 push_global_var [280] 
2873 @0x001b8b14 query_member_ptr [0] 
2874 @0x001b8b18 assign_value [] 
2875 @0x001b8b1c push_global_vptr [135] 
2876 @0x001b8b20 inc_value [1] 
2877 @0x001b8b24 pop_stack [] 
2878 @0x001b8b28 push_shared_string [2839] 'SF_PET_JOIN_IN_COMBAT'
2879 @0x001b8b2c call_in_efun [139, 1] tolower
2880 @0x001b8b30 push_global_vptr [281] 
2881 @0x001b8b34 assign_value [] 
2882 @0x001b8b38 pop_stack [] 
2883 @0x001b8b3c push_global_vptr [135] 
2884 @0x001b8b40 convert_to_value [] 
2885 @0x001b8b44 push_global_vptr [136] 
2886 @0x001b8b48 push_global_var [281] 
2887 @0x001b8b4c query_member_ptr [0] 
2888 @0x001b8b50 assign_value [] 
2889 @0x001b8b54 push_global_vptr [135] 
2890 @0x001b8b58 inc_value [1] 
2891 @0x001b8b5c pop_stack [] 
2892 @0x001b8b60 push_shared_string [2840] 'SF_PET_LEAVE_COMBAT'
2893 @0x001b8b64 call_in_efun [139, 1] tolower
2894 @0x001b8b68 push_global_vptr [282] 
2895 @0x001b8b6c assign_value [] 
2896 @0x001b8b70 pop_stack [] 
2897 @0x001b8b74 push_global_vptr [135] 
2898 @0x001b8b78 convert_to_value [] 
2899 @0x001b8b7c push_global_vptr [136] 
2900 @0x001b8b80 push_global_var [282] 
2901 @0x001b8b84 query_member_ptr [0] 
2902 @0x001b8b88 assign_value [] 
2903 @0x001b8b8c push_global_vptr [135] 
2904 @0x001b8b90 inc_value [1] 
2905 @0x001b8b94 pop_stack [] 
2906 @0x001b8b98 push_shared_string [2841] 'SF_MODIFY_COB_ATTRIB'
2907 @0x001b8b9c call_in_efun [139, 1] tolower
2908 @0x001b8ba0 push_global_vptr [283] 
2909 @0x001b8ba4 assign_value [] 
2910 @0x001b8ba8 pop_stack [] 
2911 @0x001b8bac push_global_vptr [135] 
2912 @0x001b8bb0 convert_to_value [] 
2913 @0x001b8bb4 push_global_vptr [136] 
2914 @0x001b8bb8 push_global_var [283] 
2915 @0x001b8bbc query_member_ptr [0] 
2916 @0x001b8bc0 assign_value [] 
2917 @0x001b8bc4 push_global_vptr [135] 
2918 @0x001b8bc8 inc_value [1] 
2919 @0x001b8bcc pop_stack [] 
2920 @0x001b8bd0 push_shared_string [2842] 'SF_USE_STUN_CHILD_SUCC'
2921 @0x001b8bd4 call_in_efun [139, 1] tolower
2922 @0x001b8bd8 push_global_vptr [284] 
2923 @0x001b8bdc assign_value [] 
2924 @0x001b8be0 pop_stack [] 
2925 @0x001b8be4 push_global_vptr [135] 
2926 @0x001b8be8 convert_to_value [] 
2927 @0x001b8bec push_global_vptr [136] 
2928 @0x001b8bf0 push_global_var [284] 
2929 @0x001b8bf4 query_member_ptr [0] 
2930 @0x001b8bf8 assign_value [] 
2931 @0x001b8bfc push_global_vptr [135] 
2932 @0x001b8c00 inc_value [1] 
2933 @0x001b8c04 pop_stack [] 
2934 @0x001b8c08 push_shared_string [2843] 'SF_DO_ACTION'
2935 @0x001b8c0c call_in_efun [139, 1] tolower
2936 @0x001b8c10 push_global_vptr [285] 
2937 @0x001b8c14 assign_value [] 
2938 @0x001b8c18 pop_stack [] 
2939 @0x001b8c1c push_global_vptr [135] 
2940 @0x001b8c20 convert_to_value [] 
2941 @0x001b8c24 push_global_vptr [136] 
2942 @0x001b8c28 push_global_var [285] 
2943 @0x001b8c2c query_member_ptr [0] 
2944 @0x001b8c30 assign_value [] 
2945 @0x001b8c34 push_global_vptr [135] 
2946 @0x001b8c38 inc_value [1] 
2947 @0x001b8c3c pop_stack [] 
2948 @0x001b8c40 push_shared_string [2844] 'SF_SELECT_COMBAT_TEMPLATE'
2949 @0x001b8c44 call_in_efun [139, 1] tolower
2950 @0x001b8c48 push_global_vptr [286] 
2951 @0x001b8c4c assign_value [] 
2952 @0x001b8c50 pop_stack [] 
2953 @0x001b8c54 push_global_vptr [135] 
2954 @0x001b8c58 convert_to_value [] 
2955 @0x001b8c5c push_global_vptr [136] 
2956 @0x001b8c60 push_global_var [286] 
2957 @0x001b8c64 query_member_ptr [0] 
2958 @0x001b8c68 assign_value [] 
2959 @0x001b8c6c push_global_vptr [135] 
2960 @0x001b8c70 inc_value [1] 
2961 @0x001b8c74 pop_stack [] 
2962 @0x001b8c78 push_shared_string [2845] 'SF_ADD_CHILD_SKILL_EFFECT'
2963 @0x001b8c7c call_in_efun [139, 1] tolower
2964 @0x001b8c80 push_global_vptr [287] 
2965 @0x001b8c84 assign_value [] 
2966 @0x001b8c88 pop_stack [] 
2967 @0x001b8c8c push_global_vptr [135] 
2968 @0x001b8c90 convert_to_value [] 
2969 @0x001b8c94 push_global_vptr [136] 
2970 @0x001b8c98 push_global_var [287] 
2971 @0x001b8c9c query_member_ptr [0] 
2972 @0x001b8ca0 assign_value [] 
2973 @0x001b8ca4 push_global_vptr [135] 
2974 @0x001b8ca8 inc_value [1] 
2975 @0x001b8cac pop_stack [] 
2976 @0x001b8cb0 push_shared_string [2846] 'SF_ATTACH_GOD_BOOK'
2977 @0x001b8cb4 call_in_efun [139, 1] tolower
2978 @0x001b8cb8 push_global_vptr [288] 
2979 @0x001b8cbc assign_value [] 
2980 @0x001b8cc0 pop_stack [] 
2981 @0x001b8cc4 push_global_vptr [135] 
2982 @0x001b8cc8 convert_to_value [] 
2983 @0x001b8ccc push_global_vptr [136] 
2984 @0x001b8cd0 push_global_var [288] 
2985 @0x001b8cd4 query_member_ptr [0] 
2986 @0x001b8cd8 assign_value [] 
2987 @0x001b8cdc push_global_vptr [135] 
2988 @0x001b8ce0 inc_value [1] 
2989 @0x001b8ce4 pop_stack [] 
2990 @0x001b8ce8 push_shared_string [2847] 'SF_ZHENFA_EFFECTIVE'
2991 @0x001b8cec call_in_efun [139, 1] tolower
2992 @0x001b8cf0 push_global_vptr [289] 
2993 @0x001b8cf4 assign_value [] 
2994 @0x001b8cf8 pop_stack [] 
2995 @0x001b8cfc push_global_vptr [135] 
2996 @0x001b8d00 convert_to_value [] 
2997 @0x001b8d04 push_global_vptr [136] 
2998 @0x001b8d08 push_global_var [289] 
2999 @0x001b8d0c query_member_ptr [0] 
3000 @0x001b8d10 assign_value [] 
3001 @0x001b8d14 push_global_vptr [135] 
3002 @0x001b8d18 inc_value [1] 
3003 @0x001b8d1c pop_stack [] 
3004 @0x001b8d20 push_shared_string [2848] 'SF_TASK_DROPED'
3005 @0x001b8d24 call_in_efun [139, 1] tolower
3006 @0x001b8d28 push_global_vptr [290] 
3007 @0x001b8d2c assign_value [] 
3008 @0x001b8d30 pop_stack [] 
3009 @0x001b8d34 push_global_vptr [135] 
3010 @0x001b8d38 convert_to_value [] 
3011 @0x001b8d3c push_global_vptr [136] 
3012 @0x001b8d40 push_global_var [290] 
3013 @0x001b8d44 query_member_ptr [0] 
3014 @0x001b8d48 assign_value [] 
3015 @0x001b8d4c push_global_vptr [135] 
3016 @0x001b8d50 inc_value [1] 
3017 @0x001b8d54 pop_stack [] 
3018 @0x001b8d58 push_shared_string [2849] 'SF_FIRST_HIT'
3019 @0x001b8d5c call_in_efun [139, 1] tolower
3020 @0x001b8d60 push_global_vptr [291] 
3021 @0x001b8d64 assign_value [] 
3022 @0x001b8d68 pop_stack [] 
3023 @0x001b8d6c push_global_vptr [135] 
3024 @0x001b8d70 convert_to_value [] 
3025 @0x001b8d74 push_global_vptr [136] 
3026 @0x001b8d78 push_global_var [291] 
3027 @0x001b8d7c query_member_ptr [0] 
3028 @0x001b8d80 assign_value [] 
3029 @0x001b8d84 push_global_vptr [135] 
3030 @0x001b8d88 inc_value [1] 
3031 @0x001b8d8c pop_stack [] 
3032 @0x001b8d90 push_shared_string [2850] 'SF_CHANGE_VICTIM_ID'
3033 @0x001b8d94 call_in_efun [139, 1] tolower
3034 @0x001b8d98 push_global_vptr [292] 
3035 @0x001b8d9c assign_value [] 
3036 @0x001b8da0 pop_stack [] 
3037 @0x001b8da4 push_global_vptr [135] 
3038 @0x001b8da8 convert_to_value [] 
3039 @0x001b8dac push_global_vptr [136] 
3040 @0x001b8db0 push_global_var [292] 
3041 @0x001b8db4 query_member_ptr [0] 
3042 @0x001b8db8 assign_value [] 
3043 @0x001b8dbc push_global_vptr [135] 
3044 @0x001b8dc0 inc_value [1] 
3045 @0x001b8dc4 pop_stack [] 
3046 @0x001b8dc8 push_shared_string [2851] 'SF_KEEP_ACTION'
3047 @0x001b8dcc call_in_efun [139, 1] tolower
3048 @0x001b8dd0 push_global_vptr [293] 
3049 @0x001b8dd4 assign_value [] 
3050 @0x001b8dd8 pop_stack [] 
3051 @0x001b8ddc push_global_vptr [135] 
3052 @0x001b8de0 convert_to_value [] 
3053 @0x001b8de4 push_global_vptr [136] 
3054 @0x001b8de8 push_global_var [293] 
3055 @0x001b8dec query_member_ptr [0] 
3056 @0x001b8df0 assign_value [] 
3057 @0x001b8df4 push_global_vptr [135] 
3058 @0x001b8df8 inc_value [1] 
3059 @0x001b8dfc pop_stack [] 
3060 @0x001b8e00 push_shared_string [2852] 'SF_NOT_DETECT_VICTIM'
3061 @0x001b8e04 call_in_efun [139, 1] tolower
3062 @0x001b8e08 push_global_vptr [294] 
3063 @0x001b8e0c assign_value [] 
3064 @0x001b8e10 pop_stack [] 
3065 @0x001b8e14 push_global_vptr [135] 
3066 @0x001b8e18 convert_to_value [] 
3067 @0x001b8e1c push_global_vptr [136] 
3068 @0x001b8e20 push_global_var [294] 
3069 @0x001b8e24 query_member_ptr [0] 
3070 @0x001b8e28 assign_value [] 
3071 @0x001b8e2c push_global_vptr [135] 
3072 @0x001b8e30 inc_value [1] 
3073 @0x001b8e34 pop_stack [] 
3074 @0x001b8e38 push_shared_string [2853] 'SF_IGNORE_FRIEND_PHYSICAL_DAMAGE'
3075 @0x001b8e3c call_in_efun [139, 1] tolower
3076 @0x001b8e40 push_global_vptr [295] 
3077 @0x001b8e44 assign_value [] 
3078 @0x001b8e48 pop_stack [] 
3079 @0x001b8e4c push_global_vptr [135] 
3080 @0x001b8e50 convert_to_value [] 
3081 @0x001b8e54 push_global_vptr [136] 
3082 @0x001b8e58 push_global_var [295] 
3083 @0x001b8e5c query_member_ptr [0] 
3084 @0x001b8e60 assign_value [] 
3085 @0x001b8e64 push_global_vptr [135] 
3086 @0x001b8e68 inc_value [1] 
3087 @0x001b8e6c pop_stack [] 
3088 @0x001b8e70 push_shared_string [2854] 'SF_USE_LONGTENG'
3089 @0x001b8e74 call_in_efun [139, 1] tolower
3090 @0x001b8e78 push_global_vptr [296] 
3091 @0x001b8e7c assign_value [] 
3092 @0x001b8e80 pop_stack [] 
3093 @0x001b8e84 push_global_vptr [135] 
3094 @0x001b8e88 convert_to_value [] 
3095 @0x001b8e8c push_global_vptr [136] 
3096 @0x001b8e90 push_global_var [296] 
3097 @0x001b8e94 query_member_ptr [0] 
3098 @0x001b8e98 assign_value [] 
3099 @0x001b8e9c push_global_vptr [135] 
3100 @0x001b8ea0 inc_value [1] 
3101 @0x001b8ea4 pop_stack [] 
3102 @0x001b8ea8 push_shared_string [2855] 'SF_DO_ATTACK'
3103 @0x001b8eac call_in_efun [139, 1] tolower
3104 @0x001b8eb0 push_global_vptr [297] 
3105 @0x001b8eb4 assign_value [] 
3106 @0x001b8eb8 pop_stack [] 
3107 @0x001b8ebc push_global_vptr [135] 
3108 @0x001b8ec0 convert_to_value [] 
3109 @0x001b8ec4 push_global_vptr [136] 
3110 @0x001b8ec8 push_global_var [297] 
3111 @0x001b8ecc query_member_ptr [0] 
3112 @0x001b8ed0 assign_value [] 
3113 @0x001b8ed4 push_global_vptr [135] 
3114 @0x001b8ed8 inc_value [1] 
3115 @0x001b8edc pop_stack [] 
3116 @0x001b8ee0 push_shared_string [2856] 'SF_DO_ACTION_FINISHED'
3117 @0x001b8ee4 call_in_efun [139, 1] tolower
3118 @0x001b8ee8 push_global_vptr [298] 
3119 @0x001b8eec assign_value [] 
3120 @0x001b8ef0 pop_stack [] 
3121 @0x001b8ef4 push_global_vptr [135] 
3122 @0x001b8ef8 convert_to_value [] 
3123 @0x001b8efc push_global_vptr [136] 
3124 @0x001b8f00 push_global_var [298] 
3125 @0x001b8f04 query_member_ptr [0] 
3126 @0x001b8f08 assign_value [] 
3127 @0x001b8f0c push_global_vptr [135] 
3128 @0x001b8f10 inc_value [1] 
3129 @0x001b8f14 pop_stack [] 
3130 @0x001b8f18 push_shared_string [2857] 'SF_BE_HIT_FINISHED'
3131 @0x001b8f1c call_in_efun [139, 1] tolower
3132 @0x001b8f20 push_global_vptr [299] 
3133 @0x001b8f24 assign_value [] 
3134 @0x001b8f28 pop_stack [] 
3135 @0x001b8f2c push_global_vptr [135] 
3136 @0x001b8f30 convert_to_value [] 
3137 @0x001b8f34 push_global_vptr [136] 
3138 @0x001b8f38 push_global_var [299] 
3139 @0x001b8f3c query_member_ptr [0] 
3140 @0x001b8f40 assign_value [] 
3141 @0x001b8f44 push_global_vptr [135] 
3142 @0x001b8f48 inc_value [1] 
3143 @0x001b8f4c pop_stack [] 
3144 @0x001b8f50 push_shared_string [2858] 'SF_CHANGE_MAG_DAMAGE'
3145 @0x001b8f54 call_in_efun [139, 1] tolower
3146 @0x001b8f58 push_global_vptr [300] 
3147 @0x001b8f5c assign_value [] 
3148 @0x001b8f60 pop_stack [] 
3149 @0x001b8f64 push_global_vptr [135] 
3150 @0x001b8f68 convert_to_value [] 
3151 @0x001b8f6c push_global_vptr [136] 
3152 @0x001b8f70 push_global_var [300] 
3153 @0x001b8f74 query_member_ptr [0] 
3154 @0x001b8f78 assign_value [] 
3155 @0x001b8f7c push_global_vptr [135] 
3156 @0x001b8f80 inc_value [1] 
3157 @0x001b8f84 pop_stack [] 
3158 @0x001b8f88 push_shared_string [2859] 'SF_COST_CHILD_ATTRIB'
3159 @0x001b8f8c call_in_efun [139, 1] tolower
3160 @0x001b8f90 push_global_vptr [301] 
3161 @0x001b8f94 assign_value [] 
3162 @0x001b8f98 pop_stack [] 
3163 @0x001b8f9c push_global_vptr [135] 
3164 @0x001b8fa0 convert_to_value [] 
3165 @0x001b8fa4 push_global_vptr [136] 
3166 @0x001b8fa8 push_global_var [301] 
3167 @0x001b8fac query_member_ptr [0] 
3168 @0x001b8fb0 assign_value [] 
3169 @0x001b8fb4 push_global_vptr [135] 
3170 @0x001b8fb8 inc_value [1] 
3171 @0x001b8fbc pop_stack [] 
3172 @0x001b8fc0 push_shared_string [2860] 'SF_C_SELECTED_ITEM'
3173 @0x001b8fc4 call_in_efun [139, 1] tolower
3174 @0x001b8fc8 push_global_vptr [302] 
3175 @0x001b8fcc assign_value [] 
3176 @0x001b8fd0 pop_stack [] 
3177 @0x001b8fd4 push_global_vptr [135] 
3178 @0x001b8fd8 convert_to_value [] 
3179 @0x001b8fdc push_global_vptr [136] 
3180 @0x001b8fe0 push_global_var [302] 
3181 @0x001b8fe4 query_member_ptr [0] 
3182 @0x001b8fe8 assign_value [] 
3183 @0x001b8fec push_global_vptr [135] 
3184 @0x001b8ff0 inc_value [1] 
3185 @0x001b8ff4 pop_stack [] 
3186 @0x001b8ff8 push_shared_string [2861] 'SF_HOOK_COMBAT'
3187 @0x001b8ffc call_in_efun [139, 1] tolower
3188 @0x001b9000 push_global_vptr [303] 
3189 @0x001b9004 assign_value [] 
3190 @0x001b9008 pop_stack [] 
3191 @0x001b900c push_global_vptr [135] 
3192 @0x001b9010 convert_to_value [] 
3193 @0x001b9014 push_global_vptr [136] 
3194 @0x001b9018 push_global_var [303] 
3195 @0x001b901c query_member_ptr [0] 
3196 @0x001b9020 assign_value [] 
3197 @0x001b9024 push_global_vptr [135] 
3198 @0x001b9028 inc_value [1] 
3199 @0x001b902c pop_stack [] 
3200 @0x001b9030 push_shared_string [2862] 'SF_INIT_MENU'
3201 @0x001b9034 call_in_efun [139, 1] tolower
3202 @0x001b9038 push_global_vptr [304] 
3203 @0x001b903c assign_value [] 
3204 @0x001b9040 pop_stack [] 
3205 @0x001b9044 push_global_vptr [135] 
3206 @0x001b9048 convert_to_value [] 
3207 @0x001b904c push_global_vptr [136] 
3208 @0x001b9050 push_global_var [304] 
3209 @0x001b9054 query_member_ptr [0] 
3210 @0x001b9058 assign_value [] 
3211 @0x001b905c push_global_vptr [135] 
3212 @0x001b9060 inc_value [1] 
3213 @0x001b9064 pop_stack [] 
3214 @0x001b9068 push_shared_string [2863] 'SF_POPUP_MENU'
3215 @0x001b906c call_in_efun [139, 1] tolower
3216 @0x001b9070 push_global_vptr [305] 
3217 @0x001b9074 assign_value [] 
3218 @0x001b9078 pop_stack [] 
3219 @0x001b907c push_global_vptr [135] 
3220 @0x001b9080 convert_to_value [] 
3221 @0x001b9084 push_global_vptr [136] 
3222 @0x001b9088 push_global_var [305] 
3223 @0x001b908c query_member_ptr [0] 
3224 @0x001b9090 assign_value [] 
3225 @0x001b9094 push_global_vptr [135] 
3226 @0x001b9098 inc_value [1] 
3227 @0x001b909c pop_stack [] 
3228 @0x001b90a0 push_shared_string [2864] 'SF_START_TALK'
3229 @0x001b90a4 call_in_efun [139, 1] tolower
3230 @0x001b90a8 push_global_vptr [306] 
3231 @0x001b90ac assign_value [] 
3232 @0x001b90b0 pop_stack [] 
3233 @0x001b90b4 push_global_vptr [135] 
3234 @0x001b90b8 convert_to_value [] 
3235 @0x001b90bc push_global_vptr [136] 
3236 @0x001b90c0 push_global_var [306] 
3237 @0x001b90c4 query_member_ptr [0] 
3238 @0x001b90c8 assign_value [] 
3239 @0x001b90cc push_global_vptr [135] 
3240 @0x001b90d0 inc_value [1] 
3241 @0x001b90d4 pop_stack [] 
3242 @0x001b90d8 push_shared_string [2865] 'SF_TRY_FINISHED'
3243 @0x001b90dc call_in_efun [139, 1] tolower
3244 @0x001b90e0 push_global_vptr [307] 
3245 @0x001b90e4 assign_value [] 
3246 @0x001b90e8 pop_stack [] 
3247 @0x001b90ec push_global_vptr [135] 
3248 @0x001b90f0 convert_to_value [] 
3249 @0x001b90f4 push_global_vptr [136] 
3250 @0x001b90f8 push_global_var [307] 
3251 @0x001b90fc query_member_ptr [0] 
3252 @0x001b9100 assign_value [] 
3253 @0x001b9104 push_global_vptr [135] 
3254 @0x001b9108 inc_value [1] 
3255 @0x001b910c pop_stack [] 
3256 @0x001b9110 push_shared_string [2866] 'SF_SHORT_OF_ATTACKER'
3257 @0x001b9114 call_in_efun [139, 1] tolower
3258 @0x001b9118 push_global_vptr [308] 
3259 @0x001b911c assign_value [] 
3260 @0x001b9120 pop_stack [] 
3261 @0x001b9124 push_global_vptr [135] 
3262 @0x001b9128 convert_to_value [] 
3263 @0x001b912c push_global_vptr [136] 
3264 @0x001b9130 push_global_var [308] 
3265 @0x001b9134 query_member_ptr [0] 
3266 @0x001b9138 assign_value [] 
3267 @0x001b913c push_global_vptr [135] 
3268 @0x001b9140 inc_value [1] 
3269 @0x001b9144 pop_stack [] 
3270 @0x001b9148 push_shared_string [2867] 'SF_QUIT_PARTY'
3271 @0x001b914c call_in_efun [139, 1] tolower
3272 @0x001b9150 push_global_vptr [309] 
3273 @0x001b9154 assign_value [] 
3274 @0x001b9158 pop_stack [] 
3275 @0x001b915c push_global_vptr [135] 
3276 @0x001b9160 convert_to_value [] 
3277 @0x001b9164 push_global_vptr [136] 
3278 @0x001b9168 push_global_var [309] 
3279 @0x001b916c query_member_ptr [0] 
3280 @0x001b9170 assign_value [] 
3281 @0x001b9174 push_global_vptr [135] 
3282 @0x001b9178 inc_value [1] 
3283 @0x001b917c pop_stack [] 
3284 @0x001b9180 push_shared_string [2868] 'SF_CANT_KICKOUT'
3285 @0x001b9184 call_in_efun [139, 1] tolower
3286 @0x001b9188 push_global_vptr [310] 
3287 @0x001b918c assign_value [] 
3288 @0x001b9190 pop_stack [] 
3289 @0x001b9194 push_global_vptr [135] 
3290 @0x001b9198 convert_to_value [] 
3291 @0x001b919c push_global_vptr [136] 
3292 @0x001b91a0 push_global_var [310] 
3293 @0x001b91a4 query_member_ptr [0] 
3294 @0x001b91a8 assign_value [] 
3295 @0x001b91ac push_global_vptr [135] 
3296 @0x001b91b0 inc_value [1] 
3297 @0x001b91b4 pop_stack [] 
3298 @0x001b91b8 push_shared_string [2869] 'SF_UPGRADE_YET'
3299 @0x001b91bc call_in_efun [139, 1] tolower
3300 @0x001b91c0 push_global_vptr [311] 
3301 @0x001b91c4 assign_value [] 
3302 @0x001b91c8 pop_stack [] 
3303 @0x001b91cc push_global_vptr [135] 
3304 @0x001b91d0 convert_to_value [] 
3305 @0x001b91d4 push_global_vptr [136] 
3306 @0x001b91d8 push_global_var [311] 
3307 @0x001b91dc query_member_ptr [0] 
3308 @0x001b91e0 assign_value [] 
3309 @0x001b91e4 push_global_vptr [135] 
3310 @0x001b91e8 inc_value [1] 
3311 @0x001b91ec pop_stack [] 
3312 @0x001b91f0 push_shared_string [2870] 'SF_GOT_COIN_ITEM'
3313 @0x001b91f4 call_in_efun [139, 1] tolower
3314 @0x001b91f8 push_global_vptr [312] 
3315 @0x001b91fc assign_value [] 
3316 @0x001b9200 pop_stack [] 
3317 @0x001b9204 push_global_vptr [135] 
3318 @0x001b9208 convert_to_value [] 
3319 @0x001b920c push_global_vptr [136] 
3320 @0x001b9210 push_global_var [312] 
3321 @0x001b9214 query_member_ptr [0] 
3322 @0x001b9218 assign_value [] 
3323 @0x001b921c push_global_vptr [135] 
3324 @0x001b9220 inc_value [1] 
3325 @0x001b9224 pop_stack [] 
3326 @0x001b9228 push_shared_string [2871] 'SF_FREE_TASK_DATA'
3327 @0x001b922c call_in_efun [139, 1] tolower
3328 @0x001b9230 push_global_vptr [313] 
3329 @0x001b9234 assign_value [] 
3330 @0x001b9238 pop_stack [] 
3331 @0x001b923c push_global_vptr [135] 
3332 @0x001b9240 convert_to_value [] 
3333 @0x001b9244 push_global_vptr [136] 
3334 @0x001b9248 push_global_var [313] 
3335 @0x001b924c query_member_ptr [0] 
3336 @0x001b9250 assign_value [] 
3337 @0x001b9254 push_global_vptr [135] 
3338 @0x001b9258 inc_value [1] 
3339 @0x001b925c pop_stack [] 
3340 @0x001b9260 push_shared_string [2872] 'SF_DROP'
3341 @0x001b9264 call_in_efun [139, 1] tolower
3342 @0x001b9268 push_global_vptr [314] 
3343 @0x001b926c assign_value [] 
3344 @0x001b9270 pop_stack [] 
3345 @0x001b9274 push_global_vptr [135] 
3346 @0x001b9278 convert_to_value [] 
3347 @0x001b927c push_global_vptr [136] 
3348 @0x001b9280 push_global_var [314] 
3349 @0x001b9284 query_member_ptr [0] 
3350 @0x001b9288 assign_value [] 
3351 @0x001b928c push_global_vptr [135] 
3352 @0x001b9290 inc_value [1] 
3353 @0x001b9294 pop_stack [] 
3354 @0x001b9298 push_shared_string [2873] 'SF_MODIFY_STATUS_INFO'
3355 @0x001b929c call_in_efun [139, 1] tolower
3356 @0x001b92a0 push_global_vptr [315] 
3357 @0x001b92a4 assign_value [] 
3358 @0x001b92a8 pop_stack [] 
3359 @0x001b92ac push_global_vptr [135] 
3360 @0x001b92b0 convert_to_value [] 
3361 @0x001b92b4 push_global_vptr [136] 
3362 @0x001b92b8 push_global_var [315] 
3363 @0x001b92bc query_member_ptr [0] 
3364 @0x001b92c0 assign_value [] 
3365 @0x001b92c4 push_global_vptr [135] 
3366 @0x001b92c8 inc_value [1] 
3367 @0x001b92cc pop_stack [] 
3368 @0x001b92d0 push_shared_string [2874] 'SF_OBSTRUCT_SUCCESS_RATE'
3369 @0x001b92d4 call_in_efun [139, 1] tolower
3370 @0x001b92d8 push_global_vptr [316] 
3371 @0x001b92dc assign_value [] 
3372 @0x001b92e0 pop_stack [] 
3373 @0x001b92e4 push_global_vptr [135] 
3374 @0x001b92e8 convert_to_value [] 
3375 @0x001b92ec push_global_vptr [136] 
3376 @0x001b92f0 push_global_var [316] 
3377 @0x001b92f4 query_member_ptr [0] 
3378 @0x001b92f8 assign_value [] 
3379 @0x001b92fc push_global_vptr [135] 
3380 @0x001b9300 inc_value [1] 
3381 @0x001b9304 pop_stack [] 
3382 @0x001b9308 push_shared_string [2875] 'SF_DYNAMIC_MENU'
3383 @0x001b930c call_in_efun [139, 1] tolower
3384 @0x001b9310 push_global_vptr [317] 
3385 @0x001b9314 assign_value [] 
3386 @0x001b9318 pop_stack [] 
3387 @0x001b931c push_global_vptr [135] 
3388 @0x001b9320 convert_to_value [] 
3389 @0x001b9324 push_global_vptr [136] 
3390 @0x001b9328 push_global_var [317] 
3391 @0x001b932c query_member_ptr [0] 
3392 @0x001b9330 assign_value [] 
3393 @0x001b9334 push_global_vptr [135] 
3394 @0x001b9338 inc_value [1] 
3395 @0x001b933c pop_stack [] 
3396 @0x001b9340 push_shared_string [2876] 'SF_OPPONENT_DIE'
3397 @0x001b9344 call_in_efun [139, 1] tolower
3398 @0x001b9348 push_global_vptr [318] 
3399 @0x001b934c assign_value [] 
3400 @0x001b9350 pop_stack [] 
3401 @0x001b9354 push_global_vptr [135] 
3402 @0x001b9358 convert_to_value [] 
3403 @0x001b935c push_global_vptr [136] 
3404 @0x001b9360 push_global_var [318] 
3405 @0x001b9364 query_member_ptr [0] 
3406 @0x001b9368 assign_value [] 
3407 @0x001b936c push_global_vptr [135] 
3408 @0x001b9370 inc_value [1] 
3409 @0x001b9374 pop_stack [] 
3410 @0x001b9378 push_shared_string [2722] 'SF_DIE'
3411 @0x001b937c call_in_efun [139, 1] tolower
3412 @0x001b9380 push_global_vptr [137] 
3413 @0x001b9384 push_global_var [318] 
3414 @0x001b9388 query_member_ptr [0] 
3415 @0x001b938c assign_value [] 
3416 @0x001b9390 pop_stack [] 
3417 @0x001b9394 push_shared_string [2877] 'SF_LAST_OPPONENT_DIE'
3418 @0x001b9398 call_in_efun [139, 1] tolower
3419 @0x001b939c push_global_vptr [319] 
3420 @0x001b93a0 assign_value [] 
3421 @0x001b93a4 pop_stack [] 
3422 @0x001b93a8 push_global_vptr [135] 
3423 @0x001b93ac convert_to_value [] 
3424 @0x001b93b0 push_global_vptr [136] 
3425 @0x001b93b4 push_global_var [319] 
3426 @0x001b93b8 query_member_ptr [0] 
3427 @0x001b93bc assign_value [] 
3428 @0x001b93c0 push_global_vptr [135] 
3429 @0x001b93c4 inc_value [1] 
3430 @0x001b93c8 pop_stack [] 
3431 @0x001b93cc push_shared_string [2876] 'SF_OPPONENT_DIE'
3432 @0x001b93d0 call_in_efun [139, 1] tolower
3433 @0x001b93d4 push_global_vptr [137] 
3434 @0x001b93d8 push_global_var [319] 
3435 @0x001b93dc query_member_ptr [0] 
3436 @0x001b93e0 assign_value [] 
3437 @0x001b93e4 pop_stack [] 
3438 @0x001b93e8 push_shared_string [2878] 'SF_ALLY_DIE'
3439 @0x001b93ec call_in_efun [139, 1] tolower
3440 @0x001b93f0 push_global_vptr [320] 
3441 @0x001b93f4 assign_value [] 
3442 @0x001b93f8 pop_stack [] 
3443 @0x001b93fc push_global_vptr [135] 
3444 @0x001b9400 convert_to_value [] 
3445 @0x001b9404 push_global_vptr [136] 
3446 @0x001b9408 push_global_var [320] 
3447 @0x001b940c query_member_ptr [0] 
3448 @0x001b9410 assign_value [] 
3449 @0x001b9414 push_global_vptr [135] 
3450 @0x001b9418 inc_value [1] 
3451 @0x001b941c pop_stack [] 
3452 @0x001b9420 push_shared_string [2722] 'SF_DIE'
3453 @0x001b9424 call_in_efun [139, 1] tolower
3454 @0x001b9428 push_global_vptr [137] 
3455 @0x001b942c push_global_var [320] 
3456 @0x001b9430 query_member_ptr [0] 
3457 @0x001b9434 assign_value [] 
3458 @0x001b9438 pop_stack [] 
3459 @0x001b943c push_shared_string [2879] 'SF_LAST_ALLY_DIE'
3460 @0x001b9440 call_in_efun [139, 1] tolower
3461 @0x001b9444 push_global_vptr [321] 
3462 @0x001b9448 assign_value [] 
3463 @0x001b944c pop_stack [] 
3464 @0x001b9450 push_global_vptr [135] 
3465 @0x001b9454 convert_to_value [] 
3466 @0x001b9458 push_global_vptr [136] 
3467 @0x001b945c push_global_var [321] 
3468 @0x001b9460 query_member_ptr [0] 
3469 @0x001b9464 assign_value [] 
3470 @0x001b9468 push_global_vptr [135] 
3471 @0x001b946c inc_value [1] 
3472 @0x001b9470 pop_stack [] 
3473 @0x001b9474 push_shared_string [2878] 'SF_ALLY_DIE'
3474 @0x001b9478 call_in_efun [139, 1] tolower
3475 @0x001b947c push_global_vptr [137] 
3476 @0x001b9480 push_global_var [321] 
3477 @0x001b9484 query_member_ptr [0] 
3478 @0x001b9488 assign_value [] 
3479 @0x001b948c pop_stack [] 
3480 @0x001b9490 push_shared_string [2880] 'SF_TRY_FINISHED_VICTORY'
3481 @0x001b9494 call_in_efun [139, 1] tolower
3482 @0x001b9498 push_global_vptr [322] 
3483 @0x001b949c assign_value [] 
3484 @0x001b94a0 pop_stack [] 
3485 @0x001b94a4 push_global_vptr [135] 
3486 @0x001b94a8 convert_to_value [] 
3487 @0x001b94ac push_global_vptr [136] 
3488 @0x001b94b0 push_global_var [322] 
3489 @0x001b94b4 query_member_ptr [0] 
3490 @0x001b94b8 assign_value [] 
3491 @0x001b94bc push_global_vptr [135] 
3492 @0x001b94c0 inc_value [1] 
3493 @0x001b94c4 pop_stack [] 
3494 @0x001b94c8 push_shared_string [2865] 'SF_TRY_FINISHED'
3495 @0x001b94cc call_in_efun [139, 1] tolower
3496 @0x001b94d0 push_global_vptr [137] 
3497 @0x001b94d4 push_global_var [322] 
3498 @0x001b94d8 query_member_ptr [0] 
3499 @0x001b94dc assign_value [] 
3500 @0x001b94e0 pop_stack [] 
3501 @0x001b94e4 push_shared_string [2881] 'SF_TRY_FINISHED_DEFEAT'
3502 @0x001b94e8 call_in_efun [139, 1] tolower
3503 @0x001b94ec push_global_vptr [323] 
3504 @0x001b94f0 assign_value [] 
3505 @0x001b94f4 pop_stack [] 
3506 @0x001b94f8 push_global_vptr [135] 
3507 @0x001b94fc convert_to_value [] 
3508 @0x001b9500 push_global_vptr [136] 
3509 @0x001b9504 push_global_var [323] 
3510 @0x001b9508 query_member_ptr [0] 
3511 @0x001b950c assign_value [] 
3512 @0x001b9510 push_global_vptr [135] 
3513 @0x001b9514 inc_value [1] 
3514 @0x001b9518 pop_stack [] 
3515 @0x001b951c push_shared_string [2865] 'SF_TRY_FINISHED'
3516 @0x001b9520 call_in_efun [139, 1] tolower
3517 @0x001b9524 push_global_vptr [137] 
3518 @0x001b9528 push_global_var [323] 
3519 @0x001b952c query_member_ptr [0] 
3520 @0x001b9530 assign_value [] 
3521 @0x001b9534 pop_stack [] 
3522 @0x001b9538 push_shared_string [2882] 'SF_TRY_FINISHED_DRAW'
3523 @0x001b953c call_in_efun [139, 1] tolower
3524 @0x001b9540 push_global_vptr [324] 
3525 @0x001b9544 assign_value [] 
3526 @0x001b9548 pop_stack [] 
3527 @0x001b954c push_global_vptr [135] 
3528 @0x001b9550 convert_to_value [] 
3529 @0x001b9554 push_global_vptr [136] 
3530 @0x001b9558 push_global_var [324] 
3531 @0x001b955c query_member_ptr [0] 
3532 @0x001b9560 assign_value [] 
3533 @0x001b9564 push_global_vptr [135] 
3534 @0x001b9568 inc_value [1] 
3535 @0x001b956c pop_stack [] 
3536 @0x001b9570 push_shared_string [2865] 'SF_TRY_FINISHED'
3537 @0x001b9574 call_in_efun [139, 1] tolower
3538 @0x001b9578 push_global_vptr [137] 
3539 @0x001b957c push_global_var [324] 
3540 @0x001b9580 query_member_ptr [0] 
3541 @0x001b9584 assign_value [] 
3542 @0x001b9588 pop_stack [] 
3543 @0x001b958c push_shared_string [2883] 'SF_COMBAT_FINISHED_VICTORY'
3544 @0x001b9590 call_in_efun [139, 1] tolower
3545 @0x001b9594 push_global_vptr [325] 
3546 @0x001b9598 assign_value [] 
3547 @0x001b959c pop_stack [] 
3548 @0x001b95a0 push_global_vptr [135] 
3549 @0x001b95a4 convert_to_value [] 
3550 @0x001b95a8 push_global_vptr [136] 
3551 @0x001b95ac push_global_var [325] 
3552 @0x001b95b0 query_member_ptr [0] 
3553 @0x001b95b4 assign_value [] 
3554 @0x001b95b8 push_global_vptr [135] 
3555 @0x001b95bc inc_value [1] 
3556 @0x001b95c0 pop_stack [] 
3557 @0x001b95c4 push_shared_string [2713] 'SF_COMBAT_FINISHED'
3558 @0x001b95c8 call_in_efun [139, 1] tolower
3559 @0x001b95cc push_global_vptr [137] 
3560 @0x001b95d0 push_global_var [325] 
3561 @0x001b95d4 query_member_ptr [0] 
3562 @0x001b95d8 assign_value [] 
3563 @0x001b95dc pop_stack [] 
3564 @0x001b95e0 push_shared_string [2884] 'SF_COMBAT_FINISHED_DEFEAT'
3565 @0x001b95e4 call_in_efun [139, 1] tolower
3566 @0x001b95e8 push_global_vptr [326] 
3567 @0x001b95ec assign_value [] 
3568 @0x001b95f0 pop_stack [] 
3569 @0x001b95f4 push_global_vptr [135] 
3570 @0x001b95f8 convert_to_value [] 
3571 @0x001b95fc push_global_vptr [136] 
3572 @0x001b9600 push_global_var [326] 
3573 @0x001b9604 query_member_ptr [0] 
3574 @0x001b9608 assign_value [] 
3575 @0x001b960c push_global_vptr [135] 
3576 @0x001b9610 inc_value [1] 
3577 @0x001b9614 pop_stack [] 
3578 @0x001b9618 push_shared_string [2713] 'SF_COMBAT_FINISHED'
3579 @0x001b961c call_in_efun [139, 1] tolower
3580 @0x001b9620 push_global_vptr [137] 
3581 @0x001b9624 push_global_var [326] 
3582 @0x001b9628 query_member_ptr [0] 
3583 @0x001b962c assign_value [] 
3584 @0x001b9630 pop_stack [] 
3585 @0x001b9634 push_shared_string [2885] 'SF_COMBAT_FINISHED_DRAW'
3586 @0x001b9638 call_in_efun [139, 1] tolower
3587 @0x001b963c push_global_vptr [327] 
3588 @0x001b9640 assign_value [] 
3589 @0x001b9644 pop_stack [] 
3590 @0x001b9648 push_global_vptr [135] 
3591 @0x001b964c convert_to_value [] 
3592 @0x001b9650 push_global_vptr [136] 
3593 @0x001b9654 push_global_var [327] 
3594 @0x001b9658 query_member_ptr [0] 
3595 @0x001b965c assign_value [] 
3596 @0x001b9660 push_global_vptr [135] 
3597 @0x001b9664 inc_value [1] 
3598 @0x001b9668 pop_stack [] 
3599 @0x001b966c push_shared_string [2713] 'SF_COMBAT_FINISHED'
3600 @0x001b9670 call_in_efun [139, 1] tolower
3601 @0x001b9674 push_global_vptr [137] 
3602 @0x001b9678 push_global_var [327] 
3603 @0x001b967c query_member_ptr [0] 
3604 @0x001b9680 assign_value [] 
3605 @0x001b9684 pop_stack [] 
3606 @0x001b9688 push_shared_string [2886] 'SF_LAST_OPPONENT'
3607 @0x001b968c call_in_efun [139, 1] tolower
3608 @0x001b9690 push_global_vptr [328] 
3609 @0x001b9694 assign_value [] 
3610 @0x001b9698 pop_stack [] 
3611 @0x001b969c push_global_vptr [135] 
3612 @0x001b96a0 convert_to_value [] 
3613 @0x001b96a4 push_global_vptr [136] 
3614 @0x001b96a8 push_global_var [328] 
3615 @0x001b96ac query_member_ptr [0] 
3616 @0x001b96b0 assign_value [] 
3617 @0x001b96b4 push_global_vptr [135] 
3618 @0x001b96b8 inc_value [1] 
3619 @0x001b96bc pop_stack [] 
3620 @0x001b96c0 push_shared_string [2863] 'SF_POPUP_MENU'
3621 @0x001b96c4 call_in_efun [139, 1] tolower
3622 @0x001b96c8 push_global_vptr [137] 
3623 @0x001b96cc push_global_var [328] 
3624 @0x001b96d0 query_member_ptr [0] 
3625 @0x001b96d4 assign_value [] 
3626 @0x001b96d8 pop_stack [] 
3627 @0x001b96dc push_shared_string [2887] 'SF_LAST_ALLY'
3628 @0x001b96e0 call_in_efun [139, 1] tolower
3629 @0x001b96e4 push_global_vptr [329] 
3630 @0x001b96e8 assign_value [] 
3631 @0x001b96ec pop_stack [] 
3632 @0x001b96f0 push_global_vptr [135] 
3633 @0x001b96f4 convert_to_value [] 
3634 @0x001b96f8 push_global_vptr [136] 
3635 @0x001b96fc push_global_var [329] 
3636 @0x001b9700 query_member_ptr [0] 
3637 @0x001b9704 assign_value [] 
3638 @0x001b9708 push_global_vptr [135] 
3639 @0x001b970c inc_value [1] 
3640 @0x001b9710 pop_stack [] 
3641 @0x001b9714 push_shared_string [2863] 'SF_POPUP_MENU'
3642 @0x001b9718 call_in_efun [139, 1] tolower
3643 @0x001b971c push_global_vptr [137] 
3644 @0x001b9720 push_global_var [329] 
3645 @0x001b9724 query_member_ptr [0] 
3646 @0x001b9728 assign_value [] 
3647 @0x001b972c pop_stack [] 
3648 @0x001b9730 push_shared_string [2888] 'SF_LAST_PLAYER'
3649 @0x001b9734 call_in_efun [139, 1] tolower
3650 @0x001b9738 push_global_vptr [330] 
3651 @0x001b973c assign_value [] 
3652 @0x001b9740 pop_stack [] 
3653 @0x001b9744 push_global_vptr [135] 
3654 @0x001b9748 convert_to_value [] 
3655 @0x001b974c push_global_vptr [136] 
3656 @0x001b9750 push_global_var [330] 
3657 @0x001b9754 query_member_ptr [0] 
3658 @0x001b9758 assign_value [] 
3659 @0x001b975c push_global_vptr [135] 
3660 @0x001b9760 inc_value [1] 
3661 @0x001b9764 pop_stack [] 
3662 @0x001b9768 push_shared_string [2863] 'SF_POPUP_MENU'
3663 @0x001b976c call_in_efun [139, 1] tolower
3664 @0x001b9770 push_global_vptr [137] 
3665 @0x001b9774 push_global_var [330] 
3666 @0x001b9778 query_member_ptr [0] 
3667 @0x001b977c assign_value [] 
3668 @0x001b9780 pop_stack [] 
3669 @0x001b9784 push_shared_string [2889] 'SF_NEW_ROUND'
3670 @0x001b9788 call_in_efun [139, 1] tolower
3671 @0x001b978c push_global_vptr [331] 
3672 @0x001b9790 assign_value [] 
3673 @0x001b9794 pop_stack [] 
3674 @0x001b9798 push_global_vptr [135] 
3675 @0x001b979c convert_to_value [] 
3676 @0x001b97a0 push_global_vptr [136] 
3677 @0x001b97a4 push_global_var [331] 
3678 @0x001b97a8 query_member_ptr [0] 
3679 @0x001b97ac assign_value [] 
3680 @0x001b97b0 push_global_vptr [135] 
3681 @0x001b97b4 inc_value [1] 
3682 @0x001b97b8 pop_stack [] 
3683 @0x001b97bc push_shared_string [2863] 'SF_POPUP_MENU'
3684 @0x001b97c0 call_in_efun [139, 1] tolower
3685 @0x001b97c4 push_global_vptr [137] 
3686 @0x001b97c8 push_global_var [331] 
3687 @0x001b97cc query_member_ptr [0] 
3688 @0x001b97d0 assign_value [] 
3689 @0x001b97d4 pop_stack [] 
3690 @0x001b97d8 push_shared_string [2890] 'SF_STILL_ROUND'
3691 @0x001b97dc call_in_efun [139, 1] tolower
3692 @0x001b97e0 push_global_vptr [332] 
3693 @0x001b97e4 assign_value [] 
3694 @0x001b97e8 pop_stack [] 
3695 @0x001b97ec push_global_vptr [135] 
3696 @0x001b97f0 convert_to_value [] 
3697 @0x001b97f4 push_global_vptr [136] 
3698 @0x001b97f8 push_global_var [332] 
3699 @0x001b97fc query_member_ptr [0] 
3700 @0x001b9800 assign_value [] 
3701 @0x001b9804 push_global_vptr [135] 
3702 @0x001b9808 inc_value [1] 
3703 @0x001b980c pop_stack [] 
3704 @0x001b9810 push_shared_string [2863] 'SF_POPUP_MENU'
3705 @0x001b9814 call_in_efun [139, 1] tolower
3706 @0x001b9818 push_global_vptr [137] 
3707 @0x001b981c push_global_var [332] 
3708 @0x001b9820 query_member_ptr [0] 
3709 @0x001b9824 assign_value [] 
3710 @0x001b9828 pop_stack [] 
3711 @0x001b982c push_shared_string [2891] 'SF_LAST_OPPONENT_FIRST_TIME'
3712 @0x001b9830 call_in_efun [139, 1] tolower
3713 @0x001b9834 push_global_vptr [333] 
3714 @0x001b9838 assign_value [] 
3715 @0x001b983c pop_stack [] 
3716 @0x001b9840 push_global_vptr [135] 
3717 @0x001b9844 convert_to_value [] 
3718 @0x001b9848 push_global_vptr [136] 
3719 @0x001b984c push_global_var [333] 
3720 @0x001b9850 query_member_ptr [0] 
3721 @0x001b9854 assign_value [] 
3722 @0x001b9858 push_global_vptr [135] 
3723 @0x001b985c inc_value [1] 
3724 @0x001b9860 pop_stack [] 
3725 @0x001b9864 push_shared_string [2886] 'SF_LAST_OPPONENT'
3726 @0x001b9868 call_in_efun [139, 1] tolower
3727 @0x001b986c push_global_vptr [137] 
3728 @0x001b9870 push_global_var [333] 
3729 @0x001b9874 query_member_ptr [0] 
3730 @0x001b9878 assign_value [] 
3731 @0x001b987c pop_stack [] 
3732 @0x001b9880 push_shared_string [2892] 'SF_LAST_ALLY_FIRST_TIME'
3733 @0x001b9884 call_in_efun [139, 1] tolower
3734 @0x001b9888 push_global_vptr [334] 
3735 @0x001b988c assign_value [] 
3736 @0x001b9890 pop_stack [] 
3737 @0x001b9894 push_global_vptr [135] 
3738 @0x001b9898 convert_to_value [] 
3739 @0x001b989c push_global_vptr [136] 
3740 @0x001b98a0 push_global_var [334] 
3741 @0x001b98a4 query_member_ptr [0] 
3742 @0x001b98a8 assign_value [] 
3743 @0x001b98ac push_global_vptr [135] 
3744 @0x001b98b0 inc_value [1] 
3745 @0x001b98b4 pop_stack [] 
3746 @0x001b98b8 push_shared_string [2887] 'SF_LAST_ALLY'
3747 @0x001b98bc call_in_efun [139, 1] tolower
3748 @0x001b98c0 push_global_vptr [137] 
3749 @0x001b98c4 push_global_var [334] 
3750 @0x001b98c8 query_member_ptr [0] 
3751 @0x001b98cc assign_value [] 
3752 @0x001b98d0 pop_stack [] 
3753 @0x001b98d4 create_mapping [0] 
3754 @0x001b98d8 push_global_vptr [335] 
3755 @0x001b98dc assign_value [] 
3756 @0x001b98e0 pop_stack [] 
3757 @0x001b98e4 create_array [0] 
3758 @0x001b98e8 push_global_vptr [336] 
3759 @0x001b98ec assign_value [] 
3760 @0x001b98f0 pop_stack [] 
3761 @0x001b98f4 push_shared_string [2893] 'cur_attrib_plan'
3762 @0x001b98f8 push_shared_string [2696] 'earth'
3763 @0x001b98fc push_shared_string [2695] 'fire'
3764 @0x001b9900 push_shared_string [2694] 'water'
3765 @0x001b9904 push_shared_string [2693] 'wood'
3766 @0x001b9908 push_shared_string [2692] 'metal'
3767 @0x001b990c push_shared_string [2894] 'polar_already/total'
3768 @0x001b9910 push_shared_string [2895] 'polar_already/earth'
3769 @0x001b9914 push_shared_string [2896] 'polar_already/fire'
3770 @0x001b9918 push_shared_string [2897] 'polar_already/water'
3771 @0x001b991c push_shared_string [2898] 'polar_already/wood'
3772 @0x001b9920 push_shared_string [2899] 'polar_already/metal'
3773 @0x001b9924 push_shared_string [2900] 'polar_point'
3774 @0x001b9928 push_shared_string [2901] 'upgrade_magic'
3775 @0x001b992c push_shared_string [2902] 'upgrade_immortal'
3776 @0x001b9930 push_shared_string [2903] 'upgrade/total'
3777 @0x001b9934 push_shared_string [2904] 'upgrade/dex'
3778 @0x001b9938 push_shared_string [2905] 'upgrade/con'
3779 @0x001b993c push_shared_string [2906] 'upgrade/wiz'
3780 @0x001b9940 push_shared_string [2907] 'upgrade/str'
3781 @0x001b9944 push_shared_string [2908] 'upgrade/attrib_point'
3782 @0x001b9948 push_shared_string [2909] 'attrib_already/total'
3783 @0x001b994c push_shared_string [2910] 'attrib_already/dex'
3784 @0x001b9950 push_shared_string [2911] 'attrib_already/con'
3785 @0x001b9954 push_shared_string [2912] 'attrib_already/wiz'
3786 @0x001b9958 push_shared_string [2913] 'attrib_already/str'
3787 @0x001b995c push_shared_string [2914] 'attrib_point'
3788 @0x001b9960 create_array [27] 
3789 @0x001b9964 push_global_vptr [337] 
3790 @0x001b9968 assign_value [] 
3791 @0x001b996c pop_stack [] 
3792 @0x001b9970 push_number(S) [2839] 
3793 @0x001b9974 push_number(S) [8595] 
3794 @0x001b9978 push_number(S) [8983] 
3795 @0x001b997c push_number(S) [8789] 
3796 @0x001b9980 push_number(S) [9129] 
3797 @0x001b9984 push_number(S) [4331] 
3798 @0x001b9988 push_number(S) [15891] 
3799 @0x001b998c push_number(L) [65473] 
3800 @0x001b9990 push_number(S) [12365] 
3801 @0x001b9994 push_number(S) [6631] 
3802 @0x001b9998 push_number(S) [4703] 
3803 @0x001b999c push_number(S) [8977] 
3804 @0x001b99a0 push_number(L) [61671] 
3805 @0x001b99a4 push_number(S) [15879] 
3806 @0x001b99a8 push_number(S) [8381] 
3807 @0x001b99ac push_number(S) [4965] 
3808 @0x001b99b0 push_number(S) [28929] 
3809 @0x001b99b4 push_number(S) [15859] 
3810 @0x001b99b8 push_number(S) [10509] 
3811 @0x001b99bc push_number(S) [8639] 
3812 @0x001b99c0 push_number(S) [9131] 
3813 @0x001b99c4 push_number(L) [64853] 
3814 @0x001b99c8 push_number(S) [4389] 
3815 @0x001b99cc push_number(S) [8969] 
3816 @0x001b99d0 push_number(L) [64031] 
3817 @0x001b99d4 push_number(S) [10507] 
3818 @0x001b99d8 push_number(S) [8449] 
3819 @0x001b99dc push_number(S) [24593] 
3820 @0x001b99e0 push_number(S) [2819] 
3821 @0x001b99e4 push_number(S) [8191] 
3822 @0x001b99e8 push_number(S) [12269] 
3823 @0x001b99ec push_number(S) [4617] 
3824 @0x001b99f0 push_number(L) [64005] 
3825 @0x001b99f4 push_number(S) [4635] 
3826 @0x001b99f8 push_number(L) [63755] 
3827 @0x001b99fc push_number(S) [9731] 
3828 @0x001b9a00 push_number(S) [8653] 
3829 @0x001b9a04 push_number(S) [4299] 
3830 @0x001b9a08 push_number(L) [64029] 
3831 @0x001b9a0c push_number(S) [4413] 
3832 @0x001b9a10 push_number(S) [4411] 
3833 @0x001b9a14 push_number(S) [4409] 
3834 @0x001b9a18 push_number(S) [4407] 
3835 @0x001b9a1c push_number(S) [6623] 
3836 @0x001b9a20 push_number(S) [4577] 
3837 @0x001b9a24 push_number(S) [8807] 
3838 @0x001b9a28 push_number(S) [4023] 
3839 @0x001b9a2c push_number(S) [8995] 
3840 @0x001b9a30 push_number(S) [15857] 
3841 @0x001b9a34 push_number(S) [11719] 
3842 @0x001b9a38 push_number(S) [4289] 
3843 @0x001b9a3c push_number(S) [16595] 
3844 @0x001b9a40 push_number(S) [8997] 
3845 @0x001b9a44 push_number(S) [8571] 
3846 @0x001b9a48 push_number(S) [8575] 
3847 @0x001b9a4c push_number(S) [15855] 
3848 @0x001b9a50 push_number(S) [8587] 
3849 @0x001b9a54 push_number(S) [32263] 
3850 @0x001b9a58 push_number(L) [64947] 
3851 @0x001b9a5c push_number(L) [64841] 
3852 @0x001b9a60 push_number(S) [515] 
3853 @0x001b9a64 push_number(S) [7669] 
3854 @0x001b9a68 push_number(S) [8629] 
3855 @0x001b9a6c push_number(S) [8319] 
3856 @0x001b9a70 push_number(L) [64821] 
3857 @0x001b9a74 push_number(S) [4673] 
3858 @0x001b9a78 push_number(L) [64839] 
3859 @0x001b9a7c push_number(S) [8837] 
3860 @0x001b9a80 push_number(S) [8283] 
3861 @0x001b9a84 push_number(S) [14865] 
3862 @0x001b9a88 push_number(S) [11725] 
3863 @0x001b9a8c push_number(L) [62741] 
3864 @0x001b9a90 push_number(L) [63969] 
3865 @0x001b9a94 push_number(S) [4410] 
3866 @0x001b9a98 push_number(S) [8728] 
3867 @0x001b9a9c push_number(S) [24583] 
3868 @0x001b9aa0 push_number(S) [2557] 
3869 @0x001b9aa4 push_number(S) [8759] 
3870 @0x001b9aa8 push_number(S) [9477] 
3871 @0x001b9aac push_number(S) [11715] 
3872 @0x001b9ab0 push_number(S) [11731] 
3873 @0x001b9ab4 push_number(L) [64517] 
3874 @0x001b9ab8 push_number(S) [4165] 
3875 @0x001b9abc push_number(L) [65511] 
3876 @0x001b9ac0 push_number(S) [10273] 
3877 @0x001b9ac4 push_number(L) [61619] 
3878 @0x001b9ac8 push_number(S) [12285] 
3879 @0x001b9acc push_number(S) [4441] 
3880 @0x001b9ad0 push_number(S) [8815] 
3881 @0x001b9ad4 push_number(S) [10497] 
3882 @0x001b9ad8 push_number(S) [14081] 
3883 @0x001b9adc push_number(L) [63959] 
3884 @0x001b9ae0 push_number(S) [14819] 
3885 @0x001b9ae4 push_number(L) [64936] 
3886 @0x001b9ae8 push_number(S) [8399] 
3887 @0x001b9aec push_number(S) [4463] 
3888 @0x001b9af0 push_number(S) [10691] 
3889 @0x001b9af4 push_number(S) [5031] 
3890 @0x001b9af8 push_number(S) [32747] 
3891 @0x001b9afc push_number(L) [64513] 
3892 @0x001b9b00 push_number(S) [2559] 
3893 @0x001b9b04 push_number(S) [4961] 
3894 @0x001b9b08 push_number(S) [16643] 
3895 @0x001b9b0c push_number(S) [8361] 
3896 @0x001b9b10 push_number(L) [61673] 
3897 @0x001b9b14 push_number(S) [19997] 
3898 @0x001b9b18 push_number(L) [64249] 
3899 @0x001b9b1c push_number(S) [19947] 
3900 @0x001b9b20 push_number(S) [4335] 
3901 @0x001b9b24 push_number(S) [4368] 
3902 @0x001b9b28 push_number(L) [61635] 
3903 @0x001b9b2c push_number(S) [8657] 
3904 @0x001b9b30 push_number(L) [61623] 
3905 @0x001b9b34 push_number(S) [8379] 
3906 @0x001b9b38 push_number(S) [3581] 
3907 @0x001b9b3c push_number(S) [8615] 
3908 @0x001b9b40 push_number(L) [64045] 
3909 @0x001b9b44 push_number(L) [64785] 
3910 @0x001b9b48 push_number(S) [8961] 
3911 @0x001b9b4c push_number(S) [4183] 
3912 @0x001b9b50 push_number(L) [64001] 
3913 @0x001b9b54 push_number(S) [20489] 
3914 @0x001b9b58 push_number(S) [8741] 
3915 @0x001b9b5c push_number(S) [20745] 
3916 @0x001b9b60 push_number(S) [2829] 
3917 @0x001b9b64 push_number(S) [8475] 
3918 @0x001b9b68 push_number(L) [61537] 
3919 @0x001b9b6c push_number(L) [63939] 
3920 @0x001b9b70 push_number(S) [8402] 
3921 @0x001b9b74 push_number(S) [7703] 
3922 @0x001b9b78 push_number(S) [13825] 
3923 @0x001b9b7c push_number(S) [8406] 
3924 @0x001b9b80 push_number(S) [8405] 
3925 @0x001b9b84 push_number(L) [61663] 
3926 @0x001b9b88 push_number(S) [14825] 
3927 @0x001b9b8c push_number(S) [20739] 
3928 @0x001b9b90 push_number(S) [9079] 
3929 @0x001b9b94 push_number(S) [8707] 
3930 @0x001b9b98 push_number(S) [4987] 
3931 @0x001b9b9c push_number(S) [8371] 
3932 @0x001b9ba0 push_number(S) [12377] 
3933 @0x001b9ba4 push_number(S) [8583] 
3934 @0x001b9ba8 push_number(S) [11731] 
3935 @0x001b9bac push_number(L) [65019] 
3936 @0x001b9bb0 push_number(S) [8247] 
3937 @0x001b9bb4 push_number(S) [216] 
3938 @0x001b9bb8 push_number(S) [4321] 
3939 @0x001b9bbc push_number(S) [16383] 
3940 @0x001b9bc0 push_number(S) [14833] 
3941 @0x001b9bc4 push_number(S) [8165] 
3942 @0x001b9bc8 push_number(S) [9015] 
3943 @0x001b9bcc push_number(L) [64871] 
3944 @0x001b9bd0 push_number(L) [61633] 
3945 @0x001b9bd4 push_number(S) [13143] 
3946 @0x001b9bd8 push_number(S) [17237] 
3947 @0x001b9bdc push_number(S) [21329] 
3948 @0x001b9be0 push_number(S) [11012] 
3949 @0x001b9be4 push_number(S) [6918] 
3950 @0x001b9be8 create_array [158] 
3951 @0x001b9bec push_global_vptr [338] 
3952 @0x001b9bf0 assign_value [] 
3953 @0x001b9bf4 pop_stack [] 
3954 @0x001b9bf8 push_number(S) [0] 
3955 @0x001b9bfc push_global_vptr [339] 
3956 @0x001b9c00 assign_value [] 
3957 @0x001b9c04 pop_stack [] 
3958 @0x001b9c08 push_number(S) [1] 
3959 @0x001b9c0c push_shared_string [1100] 'login'
3960 @0x001b9c10 create_mapping [2] 
3961 @0x001b9c14 push_global_vptr [340] 
3962 @0x001b9c18 assign_value [] 
3963 @0x001b9c1c pop_stack [] 
3964 @0x001b9c20 push_shared_string [2915] 'GM问道手机助手'
3965 @0x001b9c24 create_array [1] 
3966 @0x001b9c28 push_global_vptr [341] 
3967 @0x001b9c2c assign_value [] 
3968 @0x001b9c30 pop_stack [] 
3969 @0x001b9c34 push_number(S) [1] 
3970 @0x001b9c38 push_global_vptr [342] 
3971 @0x001b9c3c assign_value [] 
3972 @0x001b9c40 pop_stack [] 
3973 @0x001b9c44 push_number(S) [1] 
3974 @0x001b9c48 push_global_vptr [343] 
3975 @0x001b9c4c assign_value [] 
3976 @0x001b9c50 pop_stack [] 
3977 @0x001b9c54 push_number(S) [1] 
3978 @0x001b9c58 push_global_vptr [344] 
3979 @0x001b9c5c assign_value [] 
3980 @0x001b9c60 pop_stack [] 
3981 @0x001b9c64 push_number(S) [1] 
3982 @0x001b9c68 push_global_vptr [345] 
3983 @0x001b9c6c assign_value [] 
3984 @0x001b9c70 pop_stack [] 
3985 @0x001b9c74 push_number(S) [1] 
3986 @0x001b9c78 push_global_vptr [346] 
3987 @0x001b9c7c assign_value [] 
3988 @0x001b9c80 pop_stack [] 
3989 @0x001b9c84 push_number(S) [0] 
3990 @0x001b9c88 call_in_efun [518, 1] make8
3991 @0x001b9c8c push_global_vptr [347] 
3992 @0x001b9c90 assign_value [] 
3993 @0x001b9c94 pop_stack [] 
3994 @0x001b9c98 push_number(S) [0] 
3995 @0x001b9c9c call_in_efun [519, 1] make16
3996 @0x001b9ca0 push_global_vptr [348] 
3997 @0x001b9ca4 assign_value [] 
3998 @0x001b9ca8 pop_stack [] 
3999 @0x001b9cac push_number(S) [0] 
4000 @0x001b9cb0 call_in_efun [521, 1] make32
4001 @0x001b9cb4 push_global_vptr [349] 
4002 @0x001b9cb8 assign_value [] 
4003 @0x001b9cbc pop_stack [] 
4004 @0x001b9cc0 push_number(S) [1] 
4005 @0x001b9cc4 call_in_efun [518, 1] make8
4006 @0x001b9cc8 push_global_vptr [350] 
4007 @0x001b9ccc assign_value [] 
4008 @0x001b9cd0 pop_stack [] 
4009 @0x001b9cd4 push_number(S) [1] 
4010 @0x001b9cd8 call_in_efun [519, 1] make16
4011 @0x001b9cdc push_global_vptr [351] 
4012 @0x001b9ce0 assign_value [] 
4013 @0x001b9ce4 pop_stack [] 
4014 @0x001b9ce8 push_number(S) [1] 
4015 @0x001b9cec call_in_efun [521, 1] make32
4016 @0x001b9cf0 push_global_vptr [352] 
4017 @0x001b9cf4 assign_value [] 
4018 @0x001b9cf8 pop_stack [] 
4019 @0x001b9cfc push_number(S) [5] 
4020 @0x001b9d00 push_global_vptr [353] 
4021 @0x001b9d04 assign_value [] 
4022 @0x001b9d08 pop_stack [] 
4023 @0x001b9d0c create_mapping [0] 
4024 @0x001b9d10 push_global_vptr [354] 
4025 @0x001b9d14 assign_value [] 
4026 @0x001b9d18 pop_stack [] 
4027 @0x001b9d1c create_mapping [0] 
4028 @0x001b9d20 push_global_vptr [355] 
4029 @0x001b9d24 assign_value [] 
4030 @0x001b9d28 pop_stack [] 
4031 @0x001b9d2c create_mapping [0] 
4032 @0x001b9d30 push_global_vptr [356] 
4033 @0x001b9d34 assign_value [] 
4034 @0x001b9d38 pop_stack [] 
4035 @0x001b9d3c push_shared_string [2916] 'include;gs;gs/include'
4036 @0x001b9d40 call_in_efun [128, 1] switch_include_dir
4037 @0x001b9d44 pop_stack [] 
4038 @0x001b9d48 push_shared_string [2917] 'full'
4039 @0x001b9d4c push_shared_string [2918] 'callstack_when_error'
4040 @0x001b9d50 call_in_efun [164, 2] config_driver
4041 @0x001b9d54 pop_stack [] 
4042 @0x001b9d58 push_number(S) [-1] 
4043 @0x001b9d5c push_global_vptr [357] 
4044 @0x001b9d60 assign_value [] 
4045 @0x001b9d64 pop_stack [] 
4046 @0x001b9d68 call_in_efun [137, 0] time
4047 @0x001b9d6c push_global_vptr [358] 
4048 @0x001b9d70 assign_value [] 
4049 @0x001b9d74 pop_stack [] 
4050 @0x001b9d78 push_number(S) [1] 
4051 @0x001b9d7c call_in_efun [109, 1] set_log_on
4052 @0x001b9d80 pop_stack [] 
4053 @0x001b9d84 call_in_user_fun [655, -128] init_startup_module
4054 @0x001b9d88 pop_stack [] 
4055 @0x001b9d8c call_in_user_fun [796, -128] init_compile_module
4056 @0x001b9d90 pop_stack [] 
4057 @0x001b9d94 call_in_user_fun [814, -128] init_sql_module
4058 @0x001b9d98 pop_stack [] 
4059 @0x001b9d9c call_in_user_fun [1003, -128] init_id_module
4060 @0x001b9da0 pop_stack [] 
4061 @0x001b9da4 call_in_user_fun [818, -128] init_iid_module
4062 @0x001b9da8 pop_stack [] 
4063 @0x001b9dac call_in_user_fun [1011, -128] init_users_module
4064 @0x001b9db0 pop_stack [] 
4065 @0x001b9db4 call_in_user_fun [1082, -128] init_cmds_module
4066 @0x001b9db8 pop_stack [] 
4067 @0x001b9dbc call_in_user_fun [660, -128] init_debug_cmds_module
4068 @0x001b9dc0 pop_stack [] 
4069 @0x001b9dc4 call_in_user_fun [1801, -128] init_admin_cmds_module
4070 @0x001b9dc8 pop_stack [] 
4071 @0x001b9dcc call_in_user_fun [1031, -128] init_map_module
4072 @0x001b9dd0 pop_stack [] 
4073 @0x001b9dd4 call_in_user_fun [1796, -128] init_platform_module
4074 @0x001b9dd8 pop_stack [] 
4075 @0x001b9ddc call_in_user_fun [1831, -128] init_rpc_module
4076 @0x001b9de0 pop_stack [] 
4077 @0x001b9de4 call_in_user_fun [1865, -128] init_basic_objects_module
4078 @0x001b9de8 pop_stack [] 
4079 @0x001b9dec push_global_var [90] 
4080 @0x001b9df0 push_shared_string [2919] ' all global feature.\n'
4081 @0x001b9df4 add [] 
4082 @0x001b9df8 call_in_efun [90, 1] printf
4083 @0x001b9dfc pop_stack [] 
4084 @0x001b9e00 push_shared_string [2920] '/feature/'
4085 @0x001b9e04 push_global_var [87] 
4086 @0x001b9e08 call_fun_ptr [0, 2] 
4087 @0x001b9e0c pop_stack [] 
4088 @0x001b9e10 push_global_var [90] 
4089 @0x001b9e14 push_shared_string [2921] ' all global inherits.\n'
4090 @0x001b9e18 add [] 
4091 @0x001b9e1c call_in_efun [90, 1] printf
4092 @0x001b9e20 pop_stack [] 
4093 @0x001b9e24 push_shared_string [2922] '/inherit/'
4094 @0x001b9e28 push_global_var [87] 
4095 @0x001b9e2c call_fun_ptr [0, 2] 
4096 @0x001b9e30 pop_stack [] 
4097 @0x001b9e34 push_global_var [90] 
4098 @0x001b9e38 push_shared_string [2923] ' all global objects.\n'
4099 @0x001b9e3c add [] 
4100 @0x001b9e40 call_in_efun [90, 1] printf
4101 @0x001b9e44 pop_stack [] 
4102 @0x001b9e48 push_shared_string [2924] '/clone/'
4103 @0x001b9e4c push_global_var [87] 
4104 @0x001b9e50 call_fun_ptr [0, 2] 
4105 @0x001b9e54 pop_stack [] 
4106 @0x001b9e58 push_global_var [90] 
4107 @0x001b9e5c push_shared_string [2925] ' all global cmds.\n'
4108 @0x001b9e60 add [] 
4109 @0x001b9e64 call_in_efun [90, 1] printf
4110 @0x001b9e68 pop_stack [] 
4111 @0x001b9e6c push_shared_string [2926] '/cmds/'
4112 @0x001b9e70 push_global_var [87] 
4113 @0x001b9e74 call_fun_ptr [0, 2] 
4114 @0x001b9e78 pop_stack [] 
4115 @0x001b9e7c push_global_var [90] 
4116 @0x001b9e80 push_shared_string [2927] ' all feature.\n'
4117 @0x001b9e84 add [] 
4118 @0x001b9e88 call_in_efun [90, 1] printf
4119 @0x001b9e8c pop_stack [] 
4120 @0x001b9e90 push_shared_string [2928] '/gs/feature/'
4121 @0x001b9e94 push_global_var [87] 
4122 @0x001b9e98 call_fun_ptr [0, 2] 
4123 @0x001b9e9c pop_stack [] 
4124 @0x001b9ea0 push_shared_string [152] '/daemons/configd.c'
4125 @0x001b9ea4 push_global_var [89] 
4126 @0x001b9ea8 call_fun_ptr [0, 2] 
4127 @0x001b9eac pop_stack [] 
4128 @0x001b9eb0 push_global_var [90] 
4129 @0x001b9eb4 push_shared_string [2929] ' all inherited objects.\n'
4130 @0x001b9eb8 add [] 
4131 @0x001b9ebc call_in_efun [90, 1] printf
4132 @0x001b9ec0 pop_stack [] 
4133 @0x001b9ec4 push_shared_string [2930] '/gs/inherit/misc/'
4134 @0x001b9ec8 push_global_var [88] 
4135 @0x001b9ecc call_fun_ptr [0, 2] 
4136 @0x001b9ed0 pop_stack [] 
4137 @0x001b9ed4 push_shared_string [2931] '/gs/inherit/item/'
4138 @0x001b9ed8 push_global_var [88] 
4139 @0x001b9edc call_fun_ptr [0, 2] 
4140 @0x001b9ee0 pop_stack [] 
4141 @0x001b9ee4 push_shared_string [2932] '/gs/inherit/char/'
4142 @0x001b9ee8 push_global_var [88] 
4143 @0x001b9eec call_fun_ptr [0, 2] 
4144 @0x001b9ef0 pop_stack [] 
4145 @0x001b9ef4 push_shared_string [2933] '/gs/inherit/room/'
4146 @0x001b9ef8 push_global_var [88] 
4147 @0x001b9efc call_fun_ptr [0, 2] 
4148 @0x001b9f00 pop_stack [] 
4149 @0x001b9f04 push_shared_string [2934] '/gs/inherit/template/'
4150 @0x001b9f08 push_global_var [88] 
4151 @0x001b9f0c call_fun_ptr [0, 2] 
4152 @0x001b9f10 pop_stack [] 
4153 @0x001b9f14 push_shared_string [2935] '/daemons/'
4154 @0x001b9f18 push_global_var [88] 
4155 @0x001b9f1c call_fun_ptr [0, 2] 
4156 @0x001b9f20 pop_stack [] 
4157 @0x001b9f24 push_global_var [90] 
4158 @0x001b9f28 push_shared_string [2936] ' all daemons.\n'
4159 @0x001b9f2c add [] 
4160 @0x001b9f30 call_in_efun [90, 1] printf
4161 @0x001b9f34 pop_stack [] 
4162 @0x001b9f38 push_shared_string [2937] '/gs/daemons/importd.c'
4163 @0x001b9f3c push_global_var [89] 
4164 @0x001b9f40 call_fun_ptr [0, 2] 
4165 @0x001b9f44 pop_stack [] 
4166 @0x001b9f48 push_shared_string [2938] '/gs/daemons/partyd.c'
4167 @0x001b9f4c push_global_var [89] 
4168 @0x001b9f50 call_fun_ptr [0, 2] 
4169 @0x001b9f54 pop_stack [] 
4170 @0x001b9f58 push_shared_string [2939] '/gs/daemons/hortationd.c'
4171 @0x001b9f5c push_global_var [89] 
4172 @0x001b9f60 call_fun_ptr [0, 2] 
4173 @0x001b9f64 pop_stack [] 
4174 @0x001b9f68 push_shared_string [2940] '/gs/daemons/dungeond.c'
4175 @0x001b9f6c push_global_var [89] 
4176 @0x001b9f70 call_fun_ptr [0, 2] 
4177 @0x001b9f74 pop_stack [] 
4178 @0x001b9f78 push_shared_string [2941] '/gs/daemons/user_sortd.c'
4179 @0x001b9f7c push_global_var [89] 
4180 @0x001b9f80 call_fun_ptr [0, 2] 
4181 @0x001b9f84 pop_stack [] 
4182 @0x001b9f88 push_shared_string [2942] '/gs/daemons/apprenticed.c'
4183 @0x001b9f8c push_global_var [89] 
4184 @0x001b9f90 call_fun_ptr [0, 2] 
4185 @0x001b9f94 pop_stack [] 
4186 @0x001b9f98 push_shared_string [2943] '/gs/daemons/guessd.c'
4187 @0x001b9f9c push_global_var [89] 
4188 @0x001b9fa0 call_fun_ptr [0, 2] 
4189 @0x001b9fa4 pop_stack [] 
4190 @0x001b9fa8 push_shared_string [2944] '/gs/daemons/wrestled.c'
4191 @0x001b9fac push_global_var [89] 
4192 @0x001b9fb0 call_fun_ptr [0, 2] 
4193 @0x001b9fb4 pop_stack [] 
4194 @0x001b9fb8 push_shared_string [2945] '/gs/daemons/recommendd.c'
4195 @0x001b9fbc push_global_var [89] 
4196 @0x001b9fc0 call_fun_ptr [0, 2] 
4197 @0x001b9fc4 pop_stack [] 
4198 @0x001b9fc8 push_shared_string [2946] '/gs/daemons/backupd.c'
4199 @0x001b9fcc push_global_var [89] 
4200 @0x001b9fd0 call_fun_ptr [0, 2] 
4201 @0x001b9fd4 pop_stack [] 
4202 @0x001b9fd8 push_shared_string [2947] '/gs/daemons/topd.c'
4203 @0x001b9fdc push_global_var [89] 
4204 @0x001b9fe0 call_fun_ptr [0, 2] 
4205 @0x001b9fe4 pop_stack [] 
4206 @0x001b9fe8 push_shared_string [2948] '/gs/daemons/gardend.c'
4207 @0x001b9fec push_global_var [89] 
4208 @0x001b9ff0 call_fun_ptr [0, 2] 
4209 @0x001b9ff4 pop_stack [] 
4210 @0x001b9ff8 push_shared_string [2949] '/gs/daemons/'
4211 @0x001b9ffc push_global_var [88] 
4212 @0x001ba000 call_fun_ptr [0, 2] 
4213 @0x001ba004 pop_stack [] 
4214 @0x001ba008 push_global_var [90] 
4215 @0x001ba00c push_shared_string [2950] ' all skills.\n'
4216 @0x001ba010 add [] 
4217 @0x001ba014 call_in_efun [90, 1] printf
4218 @0x001ba018 pop_stack [] 
4219 @0x001ba01c push_shared_string [2951] '/gs/skills/'
4220 @0x001ba020 push_global_var [88] 
4221 @0x001ba024 call_fun_ptr [0, 2] 
4222 @0x001ba028 pop_stack [] 
4223 @0x001ba02c push_global_var [90] 
4224 @0x001ba030 push_shared_string [2952] ' all items, characters, misc, monsters.\n'
4225 @0x001ba034 add [] 
4226 @0x001ba038 call_in_efun [90, 1] printf
4227 @0x001ba03c pop_stack [] 
4228 @0x001ba040 push_shared_string [2546] '/gs/clone/item/'
4229 @0x001ba044 push_global_var [87] 
4230 @0x001ba048 call_fun_ptr [0, 2] 
4231 @0x001ba04c pop_stack [] 
4232 @0x001ba050 push_shared_string [2953] '/gs/clone/char/'
4233 @0x001ba054 push_global_var [88] 
4234 @0x001ba058 call_fun_ptr [0, 2] 
4235 @0x001ba05c pop_stack [] 
4236 @0x001ba060 push_shared_string [2954] '/gs/clone/misc/'
4237 @0x001ba064 push_global_var [88] 
4238 @0x001ba068 call_fun_ptr [0, 2] 
4239 @0x001ba06c pop_stack [] 
4240 @0x001ba070 push_shared_string [2955] '/gs/clone/monster/'
4241 @0x001ba074 push_global_var [88] 
4242 @0x001ba078 call_fun_ptr [0, 2] 
4243 @0x001ba07c pop_stack [] 
4244 @0x001ba080 push_shared_string [2956] '/gs/clone/mount/'
4245 @0x001ba084 push_global_var [88] 
4246 @0x001ba088 call_fun_ptr [0, 2] 
4247 @0x001ba08c pop_stack [] 
4248 @0x001ba090 push_shared_string [2957] '/gs/clone/special_monster/'
4249 @0x001ba094 push_global_var [88] 
4250 @0x001ba098 call_fun_ptr [0, 2] 
4251 @0x001ba09c pop_stack [] 
4252 @0x001ba0a0 push_shared_string [2958] '/gs/clone/boss_monster/'
4253 @0x001ba0a4 push_global_var [88] 
4254 @0x001ba0a8 call_fun_ptr [0, 2] 
4255 @0x001ba0ac pop_stack [] 
4256 @0x001ba0b0 push_shared_string [2959] '/gs/clone/hidden_boss/'
4257 @0x001ba0b4 push_global_var [88] 
4258 @0x001ba0b8 call_fun_ptr [0, 2] 
4259 @0x001ba0bc pop_stack [] 
4260 @0x001ba0c0 push_shared_string [2960] '/gs/clone/common/'
4261 @0x001ba0c4 push_global_var [88] 
4262 @0x001ba0c8 call_fun_ptr [0, 2] 
4263 @0x001ba0cc pop_stack [] 
4264 @0x001ba0d0 push_shared_string [2961] '/gs/clone/mud_monster/'
4265 @0x001ba0d4 push_global_var [88] 
4266 @0x001ba0d8 call_fun_ptr [0, 2] 
4267 @0x001ba0dc pop_stack [] 
4268 @0x001ba0e0 push_shared_string [2962] '/gs/clone/fish/'
4269 @0x001ba0e4 push_global_var [88] 
4270 @0x001ba0e8 call_fun_ptr [0, 2] 
4271 @0x001ba0ec pop_stack [] 
4272 @0x001ba0f0 push_shared_string [2963] '/gs/clone/fishing_bullet/'
4273 @0x001ba0f4 push_global_var [88] 
4274 @0x001ba0f8 call_fun_ptr [0, 2] 
4275 @0x001ba0fc pop_stack [] 
4276 @0x001ba100 push_global_var [90] 
4277 @0x001ba104 push_shared_string [2964] ' all NPCs.\n'
4278 @0x001ba108 add [] 
4279 @0x001ba10c call_in_efun [90, 1] printf
4280 @0x001ba110 pop_stack [] 
4281 @0x001ba114 push_shared_string [2965] '/gs/clone/npc/'
4282 @0x001ba118 push_global_var [87] 
4283 @0x001ba11c call_fun_ptr [0, 2] 
4284 @0x001ba120 pop_stack [] 
4285 @0x001ba124 push_global_var [90] 
4286 @0x001ba128 push_shared_string [2966] ' all init scripts.\n'
4287 @0x001ba12c add [] 
4288 @0x001ba130 call_in_efun [90, 1] printf
4289 @0x001ba134 pop_stack [] 
4290 @0x001ba138 call_in_efun [67, 0] is_compiling
4291 @0x001ba13c if [21, 2] 
4292 @0x001ba140 push_shared_string [2967] '/daemons/data_save/'
4293 @0x001ba144 push_global_var [87] 
4294 @0x001ba148 call_fun_ptr [0, 2] 
4295 @0x001ba14c pop_stack [] 
4296 @0x001ba150 push_shared_string [2949] '/gs/daemons/'
4297 @0x001ba154 push_global_var [87] 
4298 @0x001ba158 call_fun_ptr [0, 2] 
4299 @0x001ba15c pop_stack [] 
4300 @0x001ba160 push_shared_string [2968] '/gs/cmds/'
4301 @0x001ba164 push_global_var [87] 
4302 @0x001ba168 call_fun_ptr [0, 2] 
4303 @0x001ba16c pop_stack [] 
4304 @0x001ba170 push_shared_string [2969] '/gs/zone/'
4305 @0x001ba174 push_global_var [87] 
4306 @0x001ba178 call_fun_ptr [0, 2] 
4307 @0x001ba17c pop_stack [] 
4308 @0x001ba180 push_shared_string [2970] '/gs/init/'
4309 @0x001ba184 push_global_var [87] 
4310 @0x001ba188 call_fun_ptr [0, 2] 
4311 @0x001ba18c pop_stack [] 
4312 @0x001ba190 null_action [] 
4313 @0x001ba194 call_in_user_fun [696, -128] load_dist_start_time
4314 @0x001ba198 pop_stack [] 
4315 @0x001ba19c push_shared_string [2971] 'Invoke registered initialization routines.\n'
4316 @0x001ba1a0 call_in_efun [90, 1] printf
4317 @0x001ba1a4 pop_stack [] 
4318 @0x001ba1a8 call_in_user_fun [628, -128] post_init
4319 @0x001ba1ac pop_stack [] 
4320 @0x001ba1b0 push_shared_string [2972] '\x1b[1;33mDriver load GS ok!\n\x1b[2;37;0m'
4321 @0x001ba1b4 call_in_efun [90, 1] printf
4322 @0x001ba1b8 pop_stack [] 
4323 @0x001ba1bc push_shared_string [2973] 'Driver startup\n'
4324 @0x001ba1c0 call_in_efun [10, 1] broadcast
4325 @0x001ba1c4 pop_stack [] 
4326 @0x001ba1c8 call_in_efun [67, 0] is_compiling
4327 @0x001ba1cc not_value [] 
4328 @0x001ba1d0 if [26, 2] 
4329 @0x001ba1d4 call_in_user_fun [1798, -128] init_service_module
4330 @0x001ba1d8 if [23, 2] 
4331 @0x001ba1dc push_shared_string [2550] 'recover_all_user'
4332 @0x001ba1e0 push_shared_string [1132] '/gs/daemons/logind.c'
4333 @0x001ba1e4 call_in_other_fun_ex [4, 0] 
4334 @0x001ba1e8 pop_stack [] 
4335 @0x001ba1ec push_shared_string [2551] 'start_online_update'
4336 @0x001ba1f0 push_shared_string [2974] '/daemons/online_updated.c'
4337 @0x001ba1f4 call_in_other_fun_ex [4, 0] 
4338 @0x001ba1f8 pop_stack [] 
4339 @0x001ba1fc push_shared_string [2552] 'recover_all_module_data'
4340 @0x001ba200 push_shared_string [721] '/daemons/autorecoverd.c'
4341 @0x001ba204 call_in_other_fun_ex [4, 0] 
4342 @0x001ba208 not_value [] 
4343 @0x001ba20c if [4, 2] 
4344 @0x001ba210 push_shared_string [2975] '\x1b[1;31mRecovering for backup data of special modules not finished.\nPlease do check and restart if server has been reduced.\n\x1b[2;37;0m'
4345 @0x001ba214 call_in_efun [90, 1] printf
4346 @0x001ba218 pop_stack [] 
4347 @0x001ba21c goto [3] 
4348 @0x001ba220 push_shared_string [2976] '\x1b[1;32mRecovering for backup data of special modules finished.\n\x1b[2;37;0m'
4349 @0x001ba224 call_in_efun [90, 1] printf
4350 @0x001ba228 pop_stack [] 
4351 @0x001ba22c call_in_user_fun [1800, -128] try_start_game_server_service
4352 @0x001ba230 pop_stack [] 
4353 @0x001ba234 null_action [] 
4354 @0x001ba238 null_action [] 
4355 @0x001ba23c push_shared_string [2977] 'Debug'
4356 @0x001ba240 push_global_var [65] 
4357 @0x001ba244 call_in_efun [126, 2] strsrch
4358 @0x001ba248 push_number(S) [0] 
4359 @0x001ba24c ge_value [] 
4360 @0x001ba250 if [5, 2] 
4361 @0x001ba254 push_number(S) [1] 
4362 @0x001ba258 push_shared_string [2553] 'enable_debugger'
4363 @0x001ba25c push_shared_string [789] '/gs/daemons/grantd.c'
4364 @0x001ba260 call_in_other_fun_ex [4, 1] 
4365 @0x001ba264 pop_stack [] 
4366 @0x001ba268 null_action [] 
4367 @0x001ba26c finish_action [] 
```

<a id="gs-post_init"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: post_init

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0xaaeac`；函数编号：`628`。

```text
0000 @0x000aafd1 check_args_num [0] 
0001 @0x000aafd5 enter_function [] 
0002 @0x000aafd9 push_global_var [30] 
0003 @0x000aafdd sizeof [] 
0004 @0x000aafe1 if [36, 2] 
0005 @0x000aafe5 push_global_var [30] 
0006 @0x000aafe9 push_local_vptr [-2] 
0007 @0x000aafed assign_value [] 
0008 @0x000aaff1 pop_stack [] 
0009 @0x000aaff5 create_array [0] 
0010 @0x000aaff9 push_global_vptr [30] 
0011 @0x000aaffd assign_value [] 
0012 @0x000ab001 pop_stack [] 
0013 @0x000ab005 push_local_vptr [-1] 
0014 @0x000ab009 push_local_value [-2] 
0015 @0x000ab00d push_number(S) [997] 
0016 @0x000ab011 init_for_loop [-3] 
0017 @0x000ab015 cond_jmp_for_loop [14, -3] 
0018 @0x000ab019 push_local_value [-1] 
0019 @0x000ab01d is_type [9] 
0020 @0x000ab021 if [9, 2] 
0021 @0x000ab025 new_thread_catch [6] 
0022 @0x000ab029 push_local_value [-1] 
0023 @0x000ab02d call_fun_ptr [4, 1] 
0024 @0x000ab031 pop_stack [] 
0025 @0x000ab035 push_number(S) [0] 
0026 @0x000ab039 end_thread_catch [] 
0027 @0x000ab03d goto [1] 
0028 @0x000ab041 caught_error [] 
0029 @0x000ab045 pop_stack [] 
0030 @0x000ab049 null_action [] 
0031 @0x000ab04d step_jmp_for_loop [-14, -3] 
0032 @0x000ab051 push_global_var [30] 
0033 @0x000ab055 sizeof [] 
0034 @0x000ab059 if [4, 2] 
0035 @0x000ab05d push_number(S) [1] 
0036 @0x000ab061 call_in_efun [298, 1] sleep
0037 @0x000ab065 pop_stack [] 
0038 @0x000ab069 null_action [] 
0039 @0x000ab06d null_action [] 
0040 @0x000ab071 goto [-39] 
0041 @0x000ab075 push_number(S) [0] 
0042 @0x000ab079 push_global_vptr [30] 
0043 @0x000ab07d assign_value [] 
0044 @0x000ab081 pop_stack [] 
0045 @0x000ab085 null_action [] 
0046 @0x000ab089 return_from_fun [] 
```

<a id="gs-xdbc_query"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: xdbc_query

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0xcf858`；函数编号：`593`。

```text
0000 @0x000cf97d check_args_num [0] 
0001 @0x000cf981 enter_function [] 
0002 @0x000cf985 push_local_value [1] 
0003 @0x000cf989 push_local_value [0] 
0004 @0x000cf98d push_shared_string [116] 'Call xdbc_query(%d, %s)\n'
0005 @0x000cf991 push_number(S) [0] 
0006 @0x000cf995 call_in_user_fun [584, -124] xdbc_trace
0007 @0x000cf999 pop_stack [] 
0008 @0x000cf99d push_shared_string [117] 'query'
0009 @0x000cf9a1 push_shared_string [39] 'cmd'
0010 @0x000cf9a5 push_local_value [1] 
0011 @0x000cf9a9 push_shared_string [115] 'sql_cmd'
0012 @0x000cf9ad create_mapping [4] 
0013 @0x000cf9b1 push_local_value [0] 
0014 @0x000cf9b5 call_in_user_fun [601, -126] xdbc_call_server
0015 @0x000cf9b9 push_local_vptr [-1] 
0016 @0x000cf9bd assign_value [] 
0017 @0x000cf9c1 pop_stack [] 
0018 @0x000cf9c5 push_local_value [-1] 
0019 @0x000cf9c9 push_number(S) [-1] 
0020 @0x000cf9cd eq_value [] 
0021 @0x000cf9d1 if [2, 2] 
0022 @0x000cf9d5 push_number(S) [-1] 
0023 @0x000cf9d9 return_from_fun [] 
0024 @0x000cf9dd push_local_value [0] 
0025 @0x000cf9e1 push_shared_string [118] 'cno'
0026 @0x000cf9e5 create_mapping [2] 
0027 @0x000cf9e9 push_global_vptr [8] 
0028 @0x000cf9ed push_local_value [-1] 
0029 @0x000cf9f1 query_member_ptr [0] 
0030 @0x000cf9f5 assign_value [] 
0031 @0x000cf9f9 pop_stack [] 
0032 @0x000cf9fd push_local_value [-1] 
0033 @0x000cfa01 return_from_fun [] 
0034 @0x000cfa05 null_action [] 
0035 @0x000cfa09 return_from_fun [] 
```

<a id="gs-xdbc_connect"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: xdbc_connect

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x1092a2`；函数编号：`588`。

```text
0000 @0x00109520 check_args_num [0] 
0001 @0x00109524 enter_function [] 
0002 @0x00109528 push_number(S) [2] 
0003 @0x0010952c call_in_efun [267, 1] socket
0004 @0x00109530 push_local_vptr [-3] 
0005 @0x00109534 assign_value [] 
0006 @0x00109538 pop_stack [] 
0007 @0x0010953c push_number(S) [9527] 
0008 @0x00109540 push_local_value [0] 
0009 @0x00109544 push_local_value [-3] 
0010 @0x00109548 call_in_efun [254, 3] connect
0011 @0x0010954c push_number(S) [0] 
0012 @0x00109550 ne_value [] 
0013 @0x00109554 if [12, 2] 
0014 @0x00109558 push_number(S) [9527] 
0015 @0x0010955c push_local_value [0] 
0016 @0x00109560 push_shared_string [99] 'Failed to connect to %s:%d\n'
0017 @0x00109564 push_number(S) [2] 
0018 @0x00109568 call_in_user_fun [581, -124] xdbc_log
0019 @0x0010956c pop_stack [] 
0020 @0x00109570 push_local_value [-3] 
0021 @0x00109574 call_in_efun [255, 1] close_socket
0022 @0x00109578 pop_stack [] 
0023 @0x0010957c push_number(S) [-1] 
0024 @0x00109580 return_from_fun [] 
0025 @0x00109584 null_action [] 
0026 @0x00109588 push_local_value [-3] 
0027 @0x0010958c call_in_user_fun [572, -127] xdbc_allocate_client
0028 @0x00109590 push_local_vptr [-4] 
0029 @0x00109594 assign_value [] 
0030 @0x00109598 push_number(S) [-1] 
0031 @0x0010959c eq_value [] 
0032 @0x001095a0 if [10, 2] 
0033 @0x001095a4 push_shared_string [100] 'No more free client to allocate.\n'
0034 @0x001095a8 push_number(S) [2] 
0035 @0x001095ac call_in_user_fun [581, -126] xdbc_log
0036 @0x001095b0 pop_stack [] 
0037 @0x001095b4 push_local_value [-3] 
0038 @0x001095b8 call_in_efun [255, 1] close_socket
0039 @0x001095bc pop_stack [] 
0040 @0x001095c0 push_number(S) [-1] 
0041 @0x001095c4 return_from_fun [] 
0042 @0x001095c8 null_action [] 
0043 @0x001095cc push_number(L) [2000000000] 
0044 @0x001095d0 call_in_efun [94, 1] random
0045 @0x001095d4 push_local_vptr [-5] 
0046 @0x001095d8 assign_value [] 
0047 @0x001095dc pop_stack [] 
0048 @0x001095e0 push_shared_string [101] 'set_key'
0049 @0x001095e4 push_shared_string [39] 'cmd'
0050 @0x001095e8 push_local_value [-5] 
0051 @0x001095ec push_shared_string [40] 'key'
0052 @0x001095f0 create_mapping [4] 
0053 @0x001095f4 push_local_value [-4] 
0054 @0x001095f8 call_in_user_fun [601, -126] xdbc_call_server
0055 @0x001095fc push_local_vptr [-2] 
0056 @0x00109600 assign_value [] 
0057 @0x00109604 pop_stack [] 
0058 @0x00109608 push_local_value [-2] 
0059 @0x0010960c push_number(S) [-1] 
0060 @0x00109610 eq_value [] 
0061 @0x00109614 if [7, 1] 
0062 @0x00109618 pop_stack [] 
0063 @0x0010961c push_local_value [-4] 
0064 @0x00109620 call_in_user_fun [575, -127] xdbc_get_client
0065 @0x00109624 push_local_vptr [-1] 
0066 @0x00109628 assign_value [] 
0067 @0x0010962c is_type [7] 
0068 @0x00109630 not_value [] 
0069 @0x00109634 if [12, 2] 
0070 @0x00109638 push_local_value [-2] 
0071 @0x0010963c push_local_value [-4] 
0072 @0x00109640 push_shared_string [102] 'Failed to prepare connection %d(result = %d).\n'
0073 @0x00109644 push_number(S) [2] 
0074 @0x00109648 call_in_user_fun [581, -124] xdbc_log
0075 @0x0010964c pop_stack [] 
0076 @0x00109650 push_local_value [-4] 
0077 @0x00109654 call_in_user_fun [573, -127] xdbc_free_client
0078 @0x00109658 pop_stack [] 
0079 @0x0010965c push_number(S) [-1] 
0080 @0x00109660 return_from_fun [] 
0081 @0x00109664 null_action [] 
0082 @0x00109668 push_local_value [-5] 
0083 @0x0010966c push_shared_string [103] 'Set key: %016X\n'
0084 @0x00109670 push_number(S) [1] 
0085 @0x00109674 call_in_user_fun [584, -125] xdbc_trace
0086 @0x00109678 pop_stack [] 
0087 @0x0010967c push_local_value [-5] 
0088 @0x00109680 push_local_vptr [-1] 
0089 @0x00109684 push_shared_string [40] 'key'
0090 @0x00109688 query_member_ptr [0] 
0091 @0x0010968c assign_value [] 
0092 @0x00109690 pop_stack [] 
0093 @0x00109694 push_shared_string [104] 'connect'
0094 @0x00109698 push_shared_string [39] 'cmd'
0095 @0x0010969c push_local_value [1] 
0096 @0x001096a0 push_shared_string [105] 'user'
0097 @0x001096a4 push_local_value [2] 
0098 @0x001096a8 push_shared_string [106] 'passwd'
0099 @0x001096ac push_local_value [3] 
0100 @0x001096b0 push_shared_string [107] 'db'
0101 @0x001096b4 create_mapping [8] 
0102 @0x001096b8 push_local_value [-4] 
0103 @0x001096bc call_in_user_fun [601, -126] xdbc_call_server
0104 @0x001096c0 push_local_vptr [-2] 
0105 @0x001096c4 assign_value [] 
0106 @0x001096c8 pop_stack [] 
0107 @0x001096cc push_local_value [-2] 
0108 @0x001096d0 push_number(S) [-1] 
0109 @0x001096d4 eq_value [] 
0110 @0x001096d8 if [10, 2] 
0111 @0x001096dc push_shared_string [108] 'Failed to connect to database.\n'
0112 @0x001096e0 push_number(S) [2] 
0113 @0x001096e4 call_in_user_fun [581, -126] xdbc_log
0114 @0x001096e8 pop_stack [] 
0115 @0x001096ec push_local_value [-4] 
0116 @0x001096f0 call_in_user_fun [573, -127] xdbc_free_client
0117 @0x001096f4 pop_stack [] 
0118 @0x001096f8 push_number(S) [-1] 
0119 @0x001096fc return_from_fun [] 
0120 @0x00109700 null_action [] 
0121 @0x00109704 push_local_value [-4] 
0122 @0x00109708 return_from_fun [] 
0123 @0x0010970c null_action [] 
0124 @0x00109710 return_from_fun [] 
```

<a id="gs-xdbc_execute"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: xdbc_execute

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x11329a`；函数编号：`592`。

```text
0000 @0x00113373 check_args_num [0] 
0001 @0x00113377 enter_function [] 
0002 @0x0011337b push_local_value [1] 
0003 @0x0011337f push_local_value [0] 
0004 @0x00113383 push_shared_string [113] 'Call xdbc_execute(%d, %s)\n'
0005 @0x00113387 push_number(S) [0] 
0006 @0x0011338b call_in_user_fun [584, -124] xdbc_trace
0007 @0x0011338f pop_stack [] 
0008 @0x00113393 push_shared_string [114] 'execute'
0009 @0x00113397 push_shared_string [39] 'cmd'
0010 @0x0011339b push_local_value [1] 
0011 @0x0011339f push_shared_string [115] 'sql_cmd'
0012 @0x001133a3 create_mapping [4] 
0013 @0x001133a7 push_local_value [0] 
0014 @0x001133ab call_in_user_fun [601, -126] xdbc_call_server
0015 @0x001133af return_from_fun [] 
0016 @0x001133b3 null_action [] 
0017 @0x001133b7 return_from_fun [] 
```

<a id="gs-cmd_login"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: cmd_login

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x115de7`；函数编号：`1195`。

```text
0000 @0x001160ca check_args_num [0] 
0001 @0x001160ce enter_function [] 
0002 @0x001160d2 push_local_value [1] 
0003 @0x001160d6 call_in_efun [141, 1] trim
0004 @0x001160da push_local_vptr [1] 
0005 @0x001160de assign_value [] 
0006 @0x001160e2 pop_stack [] 
0007 @0x001160e6 push_local_value [1] 
0008 @0x001160ea sizeof [] 
0009 @0x001160ee not_value [] 
0010 @0x001160f2 if [1, 2] 
0011 @0x001160f6 return_from_fun [] 
0012 @0x001160fa push_local_vptr [-2] 
0013 @0x001160fe push_local_vptr [4] 
0014 @0x00116102 push_shared_string [401] '%s:%s'
0015 @0x00116106 push_local_value [4] 
0016 @0x0011610a call_in_efun [120, 4] sscanf
0017 @0x0011610e pop_stack [] 
0018 @0x00116112 push_local_value [4] 
0019 @0x00116116 call_in_efun [122, 1] strlen
0020 @0x0011611a push_number(S) [1] 
0021 @0x0011611e and [] 
0022 @0x00116122 if [9, 2] 
0023 @0x00116126 push_shared_string [1096] '你当前的客户端版本与服务器版本不匹配，无法登录游戏。\n请更新到1.60.0905版本。'
0024 @0x0011612a push_number(S) [2] 
0025 @0x0011612e push_shared_string [999] 'to_be_disconnected'
0026 @0x00116132 push_local_value [0] 
0027 @0x00116136 call_in_user_fun [1023, -127] find_user_by_client
0028 @0x0011613a call_in_other_fun_ex [4, 2] 
0029 @0x0011613e pop_stack [] 
0030 @0x00116142 return_from_fun [] 
0031 @0x00116146 null_action [] 
0032 @0x0011614a push_number(S) [0] 
0033 @0x0011614e push_local_vptr [-1] 
0034 @0x00116152 assign_value [] 
0035 @0x00116156 pop_stack [] 
0036 @0x0011615a push_local_vptr [-1] 
0037 @0x0011615e push_local_value [4] 
0038 @0x00116162 call_in_efun [122, 1] strlen
0039 @0x00116166 push_number(S) [1] 
0040 @0x0011616a sub_value [] 
0041 @0x0011616e push_number(S) [1] 
0042 @0x00116172 init_for_loop [-3] 
0043 @0x00116176 cond_jmp_for_loop [16, -3] 
0044 @0x0011617a push_local_value [4] 
0045 @0x0011617e push_local_value [-1] 
0046 @0x00116182 query_member [0] 
0047 @0x00116186 call_in_efun [64, 1] isxdigit
0048 @0x0011618a not_value [] 
0049 @0x0011618e if [8, 2] 
0050 @0x00116192 push_shared_string [1096] '你当前的客户端版本与服务器版本不匹配，无法登录游戏。\n请更新到1.60.0905版本。'
0051 @0x00116196 push_number(S) [2] 
0052 @0x0011619a push_shared_string [999] 'to_be_disconnected'
0053 @0x0011619e push_local_value [0] 
0054 @0x001161a2 call_in_user_fun [1023, -127] find_user_by_client
0055 @0x001161a6 call_in_other_fun_ex [4, 2] 
0056 @0x001161aa pop_stack [] 
0057 @0x001161ae null_action [] 
0058 @0x001161b2 null_action [] 
0059 @0x001161b6 step_jmp_for_loop [-16, -3] 
0060 @0x001161ba push_shared_string [50] 'abcdef'
0061 @0x001161be push_number(S) [1] 
0062 @0x001161c2 push_shared_string [51] ':'
0063 @0x001161c6 push_local_value [4] 
0064 @0x001161ca add [] 
0065 @0x001161ce push_shared_string [51] ':'
0066 @0x001161d2 add [] 
0067 @0x001161d6 call_in_efun [100, 1] restore_value
0068 @0x001161da call_in_efun [210, 3] des
0069 @0x001161de cast_type [4, 0] 
0070 @0x001161e2 push_local_vptr [4] 
0071 @0x001161e6 assign_value [] 
0072 @0x001161ea pop_stack [] 
0073 @0x001161ee push_local_value [6] 
0074 @0x001161f2 call_in_efun [122, 1] strlen
0075 @0x001161f6 not_value [] 
0076 @0x001161fa if [4, 0] 
0077 @0x001161fe pop_stack [] 
0078 @0x00116202 push_local_value [4] 
0079 @0x00116206 push_shared_string [1097] '1.60.0905'
0080 @0x0011620a ne_value [] 
0081 @0x0011620e if [9, 2] 
0082 @0x00116212 push_shared_string [1096] '你当前的客户端版本与服务器版本不匹配，无法登录游戏。\n请更新到1.60.0905版本。'
0083 @0x00116216 push_number(S) [2] 
0084 @0x0011621a push_shared_string [999] 'to_be_disconnected'
0085 @0x0011621e push_local_value [0] 
0086 @0x00116222 call_in_user_fun [1023, -127] find_user_by_client
0087 @0x00116226 call_in_other_fun_ex [4, 2] 
0088 @0x0011622a pop_stack [] 
0089 @0x0011622e return_from_fun [] 
0090 @0x00116232 null_action [] 
0091 @0x00116236 push_number(S) [1] 
0092 @0x0011623a push_local_value [5] 
0093 @0x0011623e eq_value [] 
0094 @0x00116242 if [7, 2] 
0095 @0x00116246 push_number(S) [1] 
0096 @0x0011624a push_shared_string [1098] 'mini_client'
0097 @0x0011624e push_shared_string [460] 'set_temp'
0098 @0x00116252 push_local_value [0] 
0099 @0x00116256 call_in_user_fun [1023, -127] find_user_by_client
0100 @0x0011625a call_in_other_fun_ex [4, 2] 
0101 @0x0011625e pop_stack [] 
0102 @0x00116262 push_global_var [60] 
0103 @0x00116266 if [11, 2] 
0104 @0x0011626a push_global_var [62] 
0105 @0x0011626e push_shared_string [1099] 'cmd_login'
0106 @0x00116272 query_member [0] 
0107 @0x00116276 cast_type [2, 0] 
0108 @0x0011627a push_number(S) [1] 
0109 @0x0011627e add [] 
0110 @0x00116282 push_global_vptr [62] 
0111 @0x00116286 push_shared_string [1099] 'cmd_login'
0112 @0x0011628a query_member_ptr [0] 
0113 @0x0011628e assign_value [] 
0114 @0x00116292 pop_stack [] 
0115 @0x00116296 push_global_var [51] 
0116 @0x0011629a if [4, 2] 
0117 @0x0011629e call_in_efun [136, 0] tick
0118 @0x001162a2 push_local_vptr [-4] 
0119 @0x001162a6 assign_value [] 
0120 @0x001162aa pop_stack [] 
0121 @0x001162ae push_local_value [0] 
0122 @0x001162b2 call_in_user_fun [1023, -127] find_user_by_client
0123 @0x001162b6 is_type [10] 
0124 @0x001162ba push_number(S) [1] 
0125 @0x001162be eq_value [] 
0126 @0x001162c2 if [14, 2] 
0127 @0x001162c6 push_local_value [7] 
0128 @0x001162ca push_local_value [6] 
0129 @0x001162ce push_local_value [5] 
0130 @0x001162d2 push_local_value [3] 
0131 @0x001162d6 push_local_value [2] 
0132 @0x001162da push_local_value [1] 
0133 @0x001162de push_local_value [0] 
0134 @0x001162e2 create_array [7] 
0135 @0x001162e6 push_shared_string [1100] 'login'
0136 @0x001162ea push_shared_string [872] 'process_command'
0137 @0x001162ee push_local_value [0] 
0138 @0x001162f2 call_in_user_fun [1023, -127] find_user_by_client
0139 @0x001162f6 call_in_other_fun_ex [4, 2] 
0140 @0x001162fa pop_stack [] 
0141 @0x001162fe push_global_var [51] 
0142 @0x00116302 if [46, 2] 
0143 @0x00116306 push_global_var [50] 
0144 @0x0011630a push_shared_string [1099] 'cmd_login'
0145 @0x0011630e query_member [0] 
0146 @0x00116312 push_local_vptr [-5] 
0147 @0x00116316 assign_value [] 
0148 @0x0011631a is_type [7] 
0149 @0x0011631e not_value [] 
0150 @0x00116322 if [8, 2] 
0151 @0x00116326 call_in_user_fun [665, -128] init_cost_stat
0152 @0x0011632a push_global_vptr [50] 
0153 @0x0011632e push_shared_string [1099] 'cmd_login'
0154 @0x00116332 query_member_ptr [0] 
0155 @0x00116336 assign_value [] 
0156 @0x0011633a push_local_vptr [-5] 
0157 @0x0011633e assign_value [] 
0158 @0x00116342 pop_stack [] 
0159 @0x00116346 null_action [] 
0160 @0x0011634a null_action [] 
0161 @0x0011634e null_action [] 
0162 @0x00116352 null_action [] 
0163 @0x00116356 null_action [] 
0164 @0x0011635a null_action [] 
0165 @0x0011635e null_action [] 
0166 @0x00116362 null_action [] 
0167 @0x00116366 push_local_vptr [-5] 
0168 @0x0011636a push_shared_string [163] 'count'
0169 @0x0011636e query_member_ptr [0] 
0170 @0x00116372 push_local_value [0] 
0171 @0x00116376 push_number(S) [10000] 
0172 @0x0011637a mod_value [] 
0173 @0x0011637e query_member_ptr [0] 
0174 @0x00116382 inc_value [1] 
0175 @0x00116386 null_action [] 
0176 @0x0011638a push_local_vptr [-5] 
0177 @0x0011638e push_shared_string [165] 'ticks'
0178 @0x00116392 query_member_ptr [0] 
0179 @0x00116396 push_local_value [0] 
0180 @0x0011639a push_number(S) [10000] 
0181 @0x0011639e mod_value [] 
0182 @0x001163a2 query_member_ptr [0] 
0183 @0x001163a6 call_in_efun [136, 0] tick
0184 @0x001163aa push_local_value [-4] 
0185 @0x001163ae sub_value [] 
0186 @0x001163b2 op_eq_value [2] 
0187 @0x001163b6 pop_stack [] 
0188 @0x001163ba null_action [] 
0189 @0x001163be push_global_var [60] 
0190 @0x001163c2 if [11, 2] 
0191 @0x001163c6 push_global_var [63] 
0192 @0x001163ca push_shared_string [1099] 'cmd_login'
0193 @0x001163ce query_member [0] 
0194 @0x001163d2 cast_type [2, 0] 
0195 @0x001163d6 push_number(S) [1] 
0196 @0x001163da add [] 
0197 @0x001163de push_global_vptr [63] 
0198 @0x001163e2 push_shared_string [1099] 'cmd_login'
0199 @0x001163e6 query_member_ptr [0] 
0200 @0x001163ea assign_value [] 
0201 @0x001163ee pop_stack [] 
0202 @0x001163f2 null_action [] 
0203 @0x001163f6 return_from_fun [] 
```

<a id="gs-init_startup_module"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: init_startup_module

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x135c90`；函数编号：`655`。

```text
0000 @0x00135d20 check_args_num [0] 
0001 @0x00135d24 enter_function [] 
0002 @0x00135d28 push_shared_string [154] '/gs/log/'
0003 @0x00135d2c call_in_user_fun [612, -127] assure_file
0004 @0x00135d30 pop_stack [] 
0005 @0x00135d34 push_shared_string [155] 'Done of init_startup_module.\n'
0006 @0x00135d38 call_in_efun [90, 1] printf
0007 @0x00135d3c pop_stack [] 
0008 @0x00135d40 null_action [] 
0009 @0x00135d44 return_from_fun [] 
```

<a id="gs-init_sql_module"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: init_sql_module

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x144fcd`；函数编号：`814`。

```text
0000 @0x00145067 check_args_num [0] 
0001 @0x0014506b enter_function [] 
0002 @0x0014506f push_shared_string [548] ':memory:'
0003 @0x00145073 call_in_efun [423, 1] sql_open_db
0004 @0x00145077 push_global_vptr [91] 
0005 @0x0014507b assign_value [] 
0006 @0x0014507f pop_stack [] 
0007 @0x00145083 push_global_var [91] 
0008 @0x00145087 push_number(S) [-1] 
0009 @0x0014508b eq_value [] 
0010 @0x0014508f if [3, 2] 
0011 @0x00145093 push_shared_string [549] 'Failed to open memory database, startup halt.\n'
0012 @0x00145097 call_in_efun [28, 1] error
0013 @0x0014509b pop_stack [] 
0014 @0x0014509f push_shared_string [550] 'Done of init_sql_module.\n'
0015 @0x001450a3 call_in_efun [90, 1] printf
0016 @0x001450a7 pop_stack [] 
0017 @0x001450ab null_action [] 
0018 @0x001450af return_from_fun [] 
```

<a id="gs-init_service_module"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: init_service_module

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x146efa`；函数编号：`1798`。

```text
0000 @0x001470d1 check_args_num [0] 
0001 @0x001470d5 enter_function [] 
0002 @0x001470d9 push_number(S) [0] 
0003 @0x001470dd push_local_vptr [-1] 
0004 @0x001470e1 assign_value [] 
0005 @0x001470e5 pop_stack [] 
0006 @0x001470e9 push_number(S) [0] 
0007 @0x001470ed push_global_var [27] 
0008 @0x001470f1 push_shared_string [2316] 'DENY_LOGIN_INTERVAL'
0009 @0x001470f5 push_shared_string [2317] 'Reboot'
0010 @0x001470f9 call_in_efun [95, 3] read_ini
0011 @0x001470fd call_in_efun [5, 1] atoi
0012 @0x00147101 ge_value [] 
0013 @0x00147105 if [8, 2] 
0014 @0x00147109 push_shared_string [2318] 'Miss field "DENY_LOGIN_INTERVAL" in section "Reboot", expected forbidden time for startup\n'
0015 @0x0014710d call_in_efun [90, 1] printf
0016 @0x00147111 pop_stack [] 
0017 @0x00147115 push_number(S) [1] 
0018 @0x00147119 push_local_vptr [-1] 
0019 @0x0014711d assign_value [] 
0020 @0x00147121 pop_stack [] 
0021 @0x00147125 null_action [] 
0022 @0x00147129 push_number(S) [0] 
0023 @0x0014712d push_global_var [27] 
0024 @0x00147131 push_shared_string [2319] 'SERVER_PORT'
0025 @0x00147135 push_shared_string [2317] 'Reboot'
0026 @0x00147139 call_in_efun [95, 3] read_ini
0027 @0x0014713d call_in_efun [5, 1] atoi
0028 @0x00147141 ge_value [] 
0029 @0x00147145 if [8, 2] 
0030 @0x00147149 push_shared_string [2320] 'Miss field "SERVER_PORT" in section "Reboot", expected a port for game server to listen on\n'
0031 @0x0014714d call_in_efun [90, 1] printf
0032 @0x00147151 pop_stack [] 
0033 @0x00147155 push_number(S) [1] 
0034 @0x00147159 push_local_vptr [-1] 
0035 @0x0014715d assign_value [] 
0036 @0x00147161 pop_stack [] 
0037 @0x00147165 null_action [] 
0038 @0x00147169 push_local_value [-1] 
0039 @0x0014716d if [6, 2] 
0040 @0x00147171 push_shared_string [2321] '\x1b[1;31mBad configuration, please edit your game_server.ini\n\x1b[2;37;0mFor example:\n\x1b[1;32m[Reboot]\nSERVER_PORT = 8000\nDENY_LOGIN_INTERVAL = 30\n\x1b[2;37;0m'
0041 @0x00147175 call_in_efun [90, 1] printf
0042 @0x00147179 pop_stack [] 
0043 @0x0014717d call_in_efun [299, 0] suspend_thread
0044 @0x00147181 pop_stack [] 
0045 @0x00147185 null_action [] 
0046 @0x00147189 push_global_var [27] 
0047 @0x0014718d push_shared_string [2322] 'NOT_CHECK_TIME'
0048 @0x00147191 push_shared_string [2323] 'Check'
0049 @0x00147195 call_in_efun [95, 3] read_ini
0050 @0x00147199 call_in_efun [5, 1] atoi
0051 @0x0014719d not_value [] 
0052 @0x001471a1 if [37, 2] 
0053 @0x001471a5 push_shared_string [2324] '开始检测游戏服务器时间和CCS时间是否同步......\n'
0054 @0x001471a9 call_in_efun [90, 1] printf
0055 @0x001471ad pop_stack [] 
0056 @0x001471b1 push_local_vptr [-4] 
0057 @0x001471b5 call_in_user_fun [1797, -127] try_to_sync_time
0058 @0x001471b9 push_local_vptr [-3] 
0059 @0x001471bd assign_value [] 
0060 @0x001471c1 pop_stack [] 
0061 @0x001471c5 push_local_value [-3] 
0062 @0x001471c9 not_value [] 
0063 @0x001471cd if [22, 2] 
0064 @0x001471d1 push_shared_string [2325] '游戏服务器时间和CCS服务器时间不同步，服务器启动失败\n'
0065 @0x001471d5 call_in_efun [90, 1] printf
0066 @0x001471d9 pop_stack [] 
0067 @0x001471dd new_thread_catch [8] 
0068 @0x001471e1 push_local_value [-4] 
0069 @0x001471e5 push_shared_string [2326] 'GS和CCS时间同步失败的具体原因:%s\n'
0070 @0x001471e9 call_in_efun [121, 2] sprintf
0071 @0x001471ed call_in_efun [28, 1] error
0072 @0x001471f1 pop_stack [] 
0073 @0x001471f5 push_number(S) [0] 
0074 @0x001471f9 end_thread_catch [] 
0075 @0x001471fd goto [1] 
0076 @0x00147201 caught_error [] 
0077 @0x00147205 pop_stack [] 
0078 @0x00147209 create_function_value [115, -64] 
0079 @0x0014720d push_number(S) [0] 
0080 @0x00147211 push_number(S) [5] 
0081 @0x00147215 call_in_efun [327, 3] set_timer
0082 @0x00147219 pop_stack [] 
0083 @0x0014721d call_in_efun [299, 0] suspend_thread
0084 @0x00147221 pop_stack [] 
0085 @0x00147225 null_action [] 
0086 @0x00147229 push_shared_string [2327] '检测游戏服务器时间和CCS时间是否同步：OK.\n'
0087 @0x0014722d call_in_efun [90, 1] printf
0088 @0x00147231 pop_stack [] 
0089 @0x00147235 null_action [] 
0090 @0x00147239 push_shared_string [632] 'my_port'
0091 @0x0014723d push_shared_string [150] 'get_config'
0092 @0x00147241 push_shared_string [152] '/daemons/configd.c'
0093 @0x00147245 call_in_other_fun_ex [4, 1] 
0094 @0x00147249 push_local_vptr [-2] 
0095 @0x0014724d assign_value [] 
0096 @0x00147251 not_value [] 
0097 @0x00147255 if [15, 2] 
0098 @0x00147259 push_global_var [27] 
0099 @0x0014725d push_shared_string [2319] 'SERVER_PORT'
0100 @0x00147261 push_shared_string [2317] 'Reboot'
0101 @0x00147265 call_in_efun [95, 3] read_ini
0102 @0x00147269 call_in_efun [5, 1] atoi
0103 @0x0014726d push_local_vptr [-2] 
0104 @0x00147271 assign_value [] 
0105 @0x00147275 pop_stack [] 
0106 @0x00147279 push_local_value [-2] 
0107 @0x0014727d push_shared_string [632] 'my_port'
0108 @0x00147281 push_shared_string [617] 'set_config'
0109 @0x00147285 push_shared_string [152] '/daemons/configd.c'
0110 @0x00147289 call_in_other_fun_ex [4, 2] 
0111 @0x0014728d pop_stack [] 
0112 @0x00147291 null_action [] 
0113 @0x00147295 push_local_value [-2] 
0114 @0x00147299 call_in_efun [529, 1] start_service_on
0115 @0x0014729d not_value [] 
0116 @0x001472a1 if [9, 2] 
0117 @0x001472a5 push_local_value [-2] 
0118 @0x001472a9 push_shared_string [2328] '\x1b[1;31mFailed to start service on port %d.\n\x1b[2;37;0m'
0119 @0x001472ad call_in_efun [10, 2] broadcast
0120 @0x001472b1 pop_stack [] 
0121 @0x001472b5 call_in_user_fun [1793, -128] reboot
0122 @0x001472b9 pop_stack [] 
0123 @0x001472bd push_number(S) [0] 
0124 @0x001472c1 return_from_fun [] 
0125 @0x001472c5 null_action [] 
0126 @0x001472c9 push_local_value [-2] 
0127 @0x001472cd push_shared_string [2329] '\x1b[1;32mStart service on port: %d\n\x1b[2;37;0m'
0128 @0x001472d1 call_in_efun [90, 2] printf
0129 @0x001472d5 pop_stack [] 
0130 @0x001472d9 push_shared_string [2330] 'Done of init_service_module.\n'
0131 @0x001472dd call_in_efun [90, 1] printf
0132 @0x001472e1 pop_stack [] 
0133 @0x001472e5 push_number(S) [1] 
0134 @0x001472e9 return_from_fun [] 
0135 @0x001472ed null_action [] 
0136 @0x001472f1 return_from_fun [] 
```

<a id="gs-try_start_game_server_service"></a>

## gs/pack_data/lib_gs32.pak/gs/start_gs.o :: try_start_game_server_service

成员 SHA-256：`2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174`；函数记录偏移：`0x17ccd6`；函数编号：`1800`。

```text
0000 @0x0017cd6a check_args_num [0] 
0001 @0x0017cd6e enter_function [] 
0002 @0x0017cd72 push_shared_string [2342] 'start_game_server_service'
0003 @0x0017cd76 push_number(S) [0] 
0004 @0x0017cd7a call_in_efun [268, 2] create_thread
0005 @0x0017cd7e pop_stack [] 
0006 @0x0017cd82 null_action [] 
0007 @0x0017cd86 return_from_fun [] 
```
