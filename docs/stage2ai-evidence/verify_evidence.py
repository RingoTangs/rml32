"""Check static evidence integrity and selected sites, not semantic equivalence."""
import gzip
import json
from pathlib import Path
import struct
import verify_reused as reuse

ROOT, OUT = reuse.ROOT, reuse.HERE
PROFILE = 'd693cf6dff72f11d1dda1237fd566d037c1e4e694b65124902a68186a6afb717'
INVENTORY = {'vm_concatString':'0x808c7fc','pdb_allocateMemory':'0x80ce784',
 '<EXTERNAL>::strcmp':'0x804b064','pdb_reallocateMemory':'0x80cf588',
 'vm_concatStringWithCStr':'0x808c83c','vm_removeSharedString':'0x80bd460',
 'vm_typeValue':'0x808bd88','vm_typeValueAt':'0x808afc8','pdb_freeMemory':'0x80ceb94',
 'vm_intToString':'0x80b6304','vm_extendString':'0x808c6dc','vm_compareBufWithBufStr':'0x808cda0'}
SITES = {'vm_concatString':['0x808c808: MOV EBX, dword ptr [EDX + 0x10]',
 '0x808c813: CALL 0x0808c6dc','0x808c818: ADD dword ptr [EAX + 0x10], EBX','0x808c82c: CALL 0x0804b7e4'],
 'vm_extendString':['0x808c6ed: CMP dword ptr [ESI], 0x1','0x808c708: CALL 0x080ce784',
 '0x808c710: TEST EAX, EAX','0x808c780: CALL 0x080cf588','0x808c788: TEST EAX, EAX',
 '0x808c7a4: TEST byte ptr [ESI + 0x4], 0x1'],
 'vm_typeValueAt':['0x808b064: CALL 0x080b6304','0x808bcc3: CALL 0x0804bb34'],
 'pdb_reallocateMemory':['0x80cf9ea: CALL 0x0804b754','0x80cf9f2: TEST EAX, EAX',
 '0x80cfb51: CALL 0x0804b354','0x80cfbb0: TEST EAX, EAX']}
require = reuse.require


def validate_fresh(records):
    names = []
    for r in records:
        n = r['normalized_result']; p = n['procedure']; names.append(p['name'])
        require(p['name'] in INVENTORY and p['address'] == INVENTORY[p['name']], 'fresh function address')
        require(r['provider']['id'] == 'ghidra' and r['provider']['version'] == '12.1.4', 'provider')
        require(r['subject']['digest']['sha256'] == reuse.TARGET, 'target')
        require(r.get('raw_result') is not None, 'raw result')
        require(r['analysis_profile']['digest'] == PROFILE, 'profile')
        require(r['operation'] == 'analyze_function', 'fresh operation')
        body = p['body']; ranges = [(int(x['start'],16),int(x['end'],16)) for x in body['ranges']]
        require(body['available'] and body['contains_entry'] and ranges, 'body')
        require(all(a <= b for a,b in ranges), 'body range')
        require(sum(b-a+1 for a,b in ranges) == body['total_bytes'], 'body total')
        require(any(a <= int(p['address'],16) <= b for a,b in ranges), 'entry range')
        for line in n['assembly']:
            address = int(line.split(':',1)[0],16)
            require(any(a <= address <= b for a,b in ranges), 'instruction outside body')
        for site in SITES.get(p['name'],[]): require(site in n['assembly'], 'selected assembly site')
    require(len(names) == len(set(names)) and set(names) == set(INVENTORY), 'fresh inventory')


def validate_references(findings, cases, ids):
    for data in [findings, cases]:
        labels = [x['id'] for x in data]
        require(len(labels) == len(set(labels)), 'duplicate finding/case')
        for x in data:
            require(x['evidence_ids'] and set(x['evidence_ids']) <= ids, 'evidence reference')
    for f in findings:
        require(f['classification'] in {'observation','inference','static-inference','design','unknown'}, 'finding classification')
        require(f['status'] in {'answered','partial','unresolved','bounded'}, 'finding status')
        require(bool(f['statement']), 'empty finding')
    types = {'Bytes','Int','Real','Undefined','Void','Array','Mapping','Object','Function','Buffer','Reference'}
    for c in cases:
        require(c['opcode'] in {'01','1c','2a','1e','20','25','27'}, 'case opcode')
        require(c['classification'] in {'static-deduction','rust-design'}, 'case classification')
        for side in ['left','right']:
            require(c[side]['type'] in types, 'case operand type')
            for key in ['hex','repeat_hex']:
                if key in c[side]: bytes.fromhex(c[side][key])
        expected=c['expected']
        require('type' in expected or 'rust_error' in expected or 'native_error' in expected, 'expected outcome')


def validate_format(data, observations, records):
    require(data[:6] == b'\x7fELF\x01\x01', 'ELF32 little endian')
    phoff=struct.unpack_from('<I',data,28)[0]; size,count=struct.unpack_from('<HH',data,42)
    byid={r['evidence_id']:r for r in records}
    for o in observations:
        require(o['source']=='magic_Linux32', 'format source')
        i=o['segment_index']; require(0 <= i < count, 'segment index')
        kind,off,va,_,filesz,*_=struct.unpack_from('<IIIIIIII',data,phoff+i*size)
        require(kind==1 and (off,va,filesz)==(o['segment_file_offset'],o['segment_virtual_address'],o['segment_file_size']), 'load segment')
        address=int(o['virtual_address'],16); raw=bytes.fromhex(o['bytes_hex'])
        require(raw==b'%g\0' and va <= address and address+len(raw)<=va+filesz, 'format location')
        offset=off+address-va
        require(offset==o['file_offset'] and data[offset:offset+len(raw)]==raw,'format bytes')
        listing=byid[o['evidence_id']]['normalized_result']['assembly']
        require(any(x.startswith(o['native_use_site']+':') and o['virtual_address'][2:] in x for x in listing), 'format use site')


def verify(root=ROOT, here=OUT):
    root,here=Path(root),Path(here)
    retained=reuse.verify(root,here)
    fresh=json.loads(gzip.decompress((here/'rea.json.gz').read_bytes()))['records']
    validate_fresh(fresh)
    reused=json.loads(gzip.decompress((here/'reused.json.gz').read_bytes()))['records']
    records=fresh+reused; ids={r['evidence_id'] for r in records}
    require(len(ids)==27, 'combined count')
    manifest=json.loads((here/'function-manifest.json').read_text())
    require(len(manifest)==27 and {m['evidence_id'] for m in manifest}==ids,'manifest bijection')
    byid={r['evidence_id']:r for r in records}
    for m in manifest:
        r=byid[m['evidence_id']]
        require(m['record_sha256']==reuse.digest(reuse.canonical(r)), 'manifest record hash')
        path=Path(m['source']); require(not path.is_absolute() and '..' not in path.parts,'manifest source path')
        source=(root/path).resolve(); require(source.is_relative_to(root.resolve()),'manifest source escape')
        require(reuse.digest(source.read_bytes())==m['source_sha256'],'manifest source hash')
        require(m['operation']==r['operation'],'manifest operation')
        n=r['normalized_result']
        if isinstance(n,dict):
            p=n['procedure'];require(m['name']==p['name'] and m['address']==p['address'],'manifest identity')
            
            if r in fresh: require(m.get('body')==p['body'] and m.get('analysis_profile_digest')==r['analysis_profile']['digest'],'manifest body/profile')
        else: require(m['name']==r['parameters']['procedure'],'manifest function')
    findings=json.loads((here/'findings.json').read_text());cases=json.loads((here/'expected-cases.json').read_text())['cases']
    validate_references(findings,cases,ids)
    fmt=json.loads((here/'format-bytes.json').read_text());data=(root/'magic_Linux32').read_bytes()
    require(reuse.digest(data)==reuse.TARGET and fmt['target_sha256']==reuse.TARGET,'format target')
    validate_format(data,fmt['observations'],fresh)
    return {'fresh_records':12,'reused_records':retained['record_count'],'combined_records':27,
            'selected_sites':sum(map(len,SITES.values())),'findings':len(findings),'expected_cases':len(cases)}


if __name__ == '__main__': print(json.dumps(verify(),sort_keys=True))
