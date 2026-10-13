"""Validate decoded VM records and produce bounded, evidence-addressed listings."""
import csv
import gzip
import json
import struct
from collections import Counter, defaultdict
from inspect_packages import OUT, ROOT


def run():
    programs = json.loads(gzip.decompress((OUT / 'programs.json.gz').read_bytes()))
    actions = {int(row['opcode'], 16): row for row in csv.DictReader((ROOT / 'docs/evidence/actions.csv').open())}
    efuns = {int(row['index']): row for row in csv.DictReader((OUT / 'efuns.csv').open())}
    starts = {service: next(p for p in programs if p['source'].endswith('/'+service+'/start_'+service+'.o'))
              for service in ('aaa','ccs','csa','dba','gs')}
    api_usage, opcodes = Counter(), Counter()
    api_sources = defaultdict(set)
    service_apis = defaultdict(Counter)
    inheritance = []
    by_source = {p['source']:p for p in programs}
    for p in programs:
        package = p['source'].split('.pak/',1)[0]+'.pak/'
        for parent in p['inherits']:
            name = parent['name'].lstrip('/')
            if name.endswith('.c'):
                name = name[:-2]+'.o'
            resolved = by_source.get(package+name)
            expected = parent['offsets'][3:]
            observed = [resolved['constitution_count'],resolved['max_functions'],resolved['max_vars']] if resolved else None
            status = 'match' if observed == expected else ('count-mismatch' if resolved else 'missing')
            inheritance.append(dict(child=p['source'],parent=parent['name'],status=status,
                                    declared_counts=str(expected),resolved_counts=str(observed) if resolved else ''))
    with (OUT / 'inheritance.csv').open('w',newline='') as stream:
        writer = csv.DictWriter(stream,fieldnames=list(inheritance[0]))
        writer.writeheader()
        writer.writerows(inheritance)
    metadata_errors, unresolved_global_calls = [], []
    totals = Counter()
    listings = ['# 包内字节码证据\n\n由静态加载器格式解码，未执行脚本。偏移均为解压后成员内位置，'
                '不可当作 ELF 地址或压缩包内指令偏移。efun 名称由固定注册顺序恢复；'
                '调用指针及跨对象调用保留为未解析，不从附近字符串自动证明目标。\n\n']
    selected = {'cmd_login','do_when_cmd_new_connection','init_service_module','try_start_game_server_service','init_sql_module',
                'xdbc_connect','xdbc_execute','xdbc_query','post_init','init_startup_module'}
    extra_targets = {
        'gs/daemons/logind.o': {'new_user','do_login','_do_login','enter_world'},
        'gs/clone/char/login.o': {'process_command','thread_process_command','_process_command'},
        'daemons/filed.o': {'dba_comm','try_to_connect_dba','create_content','read_content','write_content'},
        'dba/daemons/dbaserver/cmd_create_content.o': {'main'},
        'dba/daemons/dbaserver/cmd_write_content.o': {'main'},
        'dba/daemons/dbaserver/cmd_read_content.o': {'main'},
        'aaa/daemons/accountd.o': {'account_login','_account_login','check_account_info','query_account_info','auth_all_protect','send_login_result'},
        'aaa/daemons/pwdprotect_authd.o': {'auth_protect','to_auth_protect','log_auth_result'},
    }

    def instructions(p, code):
        metadata = bytes.fromhex(code['metadata_hex'])
        for pc, (a, b, opcode) in enumerate(code['nodes']):
            action = actions[opcode]
            args = [a, b][:int(action['parameters'])]
            if int(action['attrib'],16) & 0x80000000:
                # Preserve the x86 sign extensions in addition to raw serialized operands.
                offset = (b << 16) | a
                count = int(action['parameters'])
                if offset < 0 or offset + count * 4 > len(metadata):
                    metadata_errors.append(dict(source=p['source'],segment=code['offset'],pc=pc,offset=offset))
                    args = ['invalid-metadata']
                else:
                    args = list(struct.unpack_from('<'+'i'*count,metadata,offset))
            note = ''
            if opcode == 64:
                assert 0 <= a < len(p['strings']), (p['source'], pc, a)
                note = repr(p['strings'][a])
            if opcode == 6:
                assert args and args[0] in efuns, (p['source'], pc, args)
                note = efuns[args[0]]['name']
            if opcode == 7 and len(args) == 2:
                context = starts[p['source'].split('/')[0]] if args[1] & 0x80 else p
                fn = next((f for f in context['functions'] if f['index'] == args[0]), None)
                note = fn['name'] if fn else 'unresolved/inherited/builtin'
            yield pc, code['nodes_offset'] + pc * 4, opcode, action['memo'], args, note

    for p in programs:
        totals['programs'] += 1
        totals['functions'] += len(p['functions'])
        totals['hidden_functions'] += len(p['hidden'])
        contexts = [dict(name='__init__',code=p['init'])] + p['functions'] + p['hidden']
        service = p['source'].split('/')[0]
        is_start = p is starts[service]
        for f in contexts:
            cs = f['code']
            decoded = list(instructions(p, cs))
            totals['instructions'] += len(decoded)
            for pc, offset, opcode, mnemonic, args, note in decoded:
                opcodes[opcode] += 1
                if opcode == 6:
                    api_usage[note] += 1
                    service_apis[service][note] += 1
                    api_sources[note].add(p['source'])
                if opcode == 7 and note == 'unresolved/inherited/builtin' and args[1] & 0x80:
                    unresolved_global_calls.append(dict(source=p['source'],function=f['name'],pc=pc,index=args[0]))
            tail = p['source'].split('.pak/',1)[1]
            if (is_start and (f['name'] == '__init__' or f['name'] in selected)) or f['name'] in extra_targets.get(tail,set()):
                anchor = service+'-'+f['name'] if is_start else service+'-'+tail.replace('/','-').replace('.','-')+'-'+f['name']
                listings.append(f'\n<a id="{anchor}"></a>\n\n## {p["source"]} :: {f["name"]}\n\n'
                                f'成员 SHA-256：`{p["sha256"]}`；函数记录偏移：`{hex(f.get("offset",cs["offset"]))}`；'
                                f'函数编号：`{f.get("index","init")}`。\n\n```text\n')
                for pc, offset, opcode, mnemonic, args, note in decoded:
                    listings.append(f'{pc:04d} @0x{offset:08x} {mnemonic} {args} {note}\n')
                listings.append('```\n')
    summary = dict(totals=totals, opcodes={hex(k):v for k,v in sorted(opcodes.items())},
                   efuns_used=len(api_usage),unused_efuns=[v['name'] for v in efuns.values() if v['name'] not in api_usage],
                   metadata_errors=metadata_errors, unresolved_global_calls=unresolved_global_calls,
                   service_efuns_used={k:len(v) for k,v in service_apis.items()})
    summary['inheritance_checks'] = dict(Counter(row['status'] for row in inheritance))
    (OUT / 'compatibility-summary.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
    with (OUT / 'efun-usage.csv').open('w',newline='') as stream:
        writer=csv.writer(stream)
        writer.writerow(['name','static_call_sites','programs','aaa','ccs','csa','dba','gs'])
        for name,count in api_usage.most_common():
            writer.writerow([name,count,len(api_sources[name])]+[service_apis[s][name] for s in starts])
    (OUT / 'bytecode-listings.md').write_text(''.join(listings))
    assert not metadata_errors
    print(json.dumps({k:v for k,v in summary.items() if k not in ['unused_efuns','unresolved_global_calls']},indent=2))
    print('UNRESOLVED GLOBAL CALL SITES',len(unresolved_global_calls))


if __name__ == '__main__':
    run()
