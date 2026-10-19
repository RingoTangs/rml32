"""Read-only integrity and selected instruction checks, not a proof of global storage/call semantics."""
import csv
import gzip
import hashlib
import json
from pathlib import Path

TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
PROVIDER = {'id': 'ghidra', 'name': 'Ghidra', 'version': '12.1.4'}
ENTRIES = {'vm_pushGlobalVptr': 134811856, 'vm_getNewGlobalVarOffset': 134743920, 'vm_safeNewValue': 134782840, 'vm_endNewFile': 134776948, 'vm_finalizeIdentifiersInProgram': 134763700, 'vm_destructIi': 134772288, 'vm_getGlobalValue': 134810324, 'vm_pushGlobalVar': 134811964, 'vm_unrefValue': 134791092, 'vm_copyValue': 134790692, 'vm_callInUserFunBody': 134823600, 'vm_callInUserFun': 134824064, 'vm_returnFromFun': 134826360, 'vm_pushThisObject': 134837720, 'vm_initIdentifiers': 134768248, 'vm_loadIdentifiers': 134774712, 'vm_callInEfun': 134823320, 'vm_regularProcessor': 134838468}
SITES = {'vm_pushGlobalVar': {134812035: 'CALL 0x0808be24'}, 'vm_pushGlobalVptr': {134811919: 'MOV DL, byte ptr [EBX + 0x5]', 134811925: 'INC dword ptr [EBX]', 134811931: 'MOV byte ptr [EAX + ECX*0x1 + 0x5], DL'}, 'vm_endNewFile': {134778195: 'CALL 0x080ce784', 134778495: 'CALL 0x080cf588', 134778408: 'CALL 0x08089f78', 134778599: 'CALL 0x0808bfb4'}, 'vm_destructIi': {134772476: 'CALL 0x0808bfb4', 134772508: 'MOV dword ptr [EDX + EAX*0x4], 0x0'}, 'vm_callInUserFun': {134824078: 'SHR EAX, 0x7', 134824083: 'AND EDI, 0x3f', 134824086: 'AND EAX, 0x1', 134824158: 'CALL 0x08093eb0'}, 'vm_returnFromFun': {134826907: 'CALL 0x0809789c', 134826917: 'TEST EBX, EBX', 134826919: 'JZ 0x08094bb5', 134826925: 'CALL 0x080b1804', 134826968: 'CALL 0x0809789c'}}
EXPECTED_RECORDS = [('vm_callInEfun', 'analyze_function'), ('vm_callInUserFun', 'procedure_assembly'), ('vm_callInUserFun', 'procedure_pseudo_code'), ('vm_callInUserFunBody', 'procedure_assembly'), ('vm_callInUserFunBody', 'procedure_pseudo_code'), ('vm_copyValue', 'analyze_function'), ('vm_destructIi', 'analyze_function'), ('vm_endNewFile', 'analyze_function'), ('vm_finalizeIdentifiersInProgram', 'analyze_function'), ('vm_getGlobalValue', 'analyze_function'), ('vm_getNewGlobalVarOffset', 'analyze_function'), ('vm_initIdentifiers', 'analyze_function'), ('vm_loadIdentifiers', 'analyze_function'), ('vm_pushGlobalVar', 'analyze_function'), ('vm_pushGlobalVptr', 'procedure_assembly'), ('vm_pushGlobalVptr', 'procedure_pseudo_code'), ('vm_pushThisObject', 'analyze_function'), ('vm_regularProcessor', 'analyze_function'), ('vm_returnFromFun', 'procedure_assembly'), ('vm_returnFromFun', 'procedure_pseudo_code'), ('vm_safeNewValue', 'analyze_function'), ('vm_unrefValue', 'procedure_pseudo_code')]

def require(ok, message):
    if not ok:
        raise ValueError(message)


def read_json(path):
    if path.suffix == '.gz':
        with gzip.open(path, 'rt') as stream:
            return json.load(stream)
    return json.loads(path.read_text())


def source_path(root, relative):
    path = (root / relative).resolve()
    require(path.is_relative_to(root.resolve()), 'source outside root')
    return path


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_bundle(root=None, here=None):
    here = Path(here) if here is not None else Path(__file__).resolve().parent
    root = Path(root) if root is not None else here.parent.parent
    require(digest(root / 'magic_Linux32') == TARGET, 'target bytes')
    records, provenance = {}, {}
    bundles = {}
    for filename, count in [('rea.json.gz', 8), ('reused.json.gz', 14)]:
        path = here / filename
        bundle = read_json(path)
        bundles[filename] = bundle
        require(bundle['providers'] == [PROVIDER], 'bundle provider')
        require(len(bundle['artifacts']) == 1 and bundle['artifacts'][0]['digest']['sha256'] == TARGET, 'bundle target')
        require(len(bundle['records']) == count, 'record count')
        for record in bundle['records']:
            eid = record['evidence_id']
            require(eid not in records, 'duplicate record')
            require(record['subject']['digest']['sha256'] == TARGET, 'record target')
            require(record['provider'] == PROVIDER, 'record provider')
            records[eid] = record
            if filename == 'rea.json.gz':
                provenance[eid] = ('docs/stage2aa-evidence/rea.json.gz', digest(path))
    require(sorted((r['parameters']['procedure'],r['operation']) for r in records.values()) == EXPECTED_RECORDS, 'reviewed function operations')
    reused = bundles['reused.json.gz']
    require(len(reused['sources']) == 6, 'source count')
    require(len({s['path'] for s in reused['sources']}) == 6, 'duplicate source')
    for source in reused['sources']:
        path = source_path(root, source['path'])
        require(digest(path) == source['sha256'], 'source hash')
        originals = read_json(path)['records']
        canonical = {r['evidence_id']: r for r in originals}
        require(len(canonical) == len(originals), 'duplicate source record')
        for eid in source['evidence_ids']:
            require(eid in records and eid in canonical, 'unknown source record')
            require(eid not in provenance, 'duplicate provenance')
            require(records[eid] == canonical[eid], 'edited retained record')
            provenance[eid] = (source['path'], source['sha256'])
    require(set(provenance) == set(records), 'missing provenance')
    manifest = read_json(here / 'function-manifest.json')
    require(len(manifest) == len(records) and {m['evidence_id'] for m in manifest} == set(records), 'manifest bijection')
    for item in manifest:
        record = records[item['evidence_id']]
        name = record['parameters']['procedure']
        require(item['name'] == name and item['operation'] == record['operation'], 'manifest operation')
        require(name in ENTRIES and int(item['address'], 16) == ENTRIES[name], 'manifest address')
        require((item['source'], item['source_sha256']) == provenance[item['evidence_id']], 'manifest provenance')
        require(item['limitations'] == record['limitations'], 'manifest limitations')
        if record['operation'] == 'analyze_function':
            proc = record['normalized_result']['procedure']
            require(int(proc['address'], 16) == ENTRIES[name], 'procedure address')
            require(item.get('body') == proc['body'], 'manifest body')
        else:
            require('body' not in item, 'unexpected manifest body')
    navigation = read_json(here / 'navigation.json.gz')
    require(navigation['providers'] == [PROVIDER], 'navigation provider')
    require(len(navigation['artifacts']) == 1 and navigation['artifacts'][0]['digest']['sha256'] == TARGET, 'navigation target')
    nav_records = navigation['records']
    nav_manifest = read_json(here / 'navigation-manifest.json')
    require(len(nav_records) == len(nav_manifest) == 7, 'navigation count')
    nav_by_id = {r['evidence_id']:r for r in nav_records}
    require(len(nav_by_id) == 7 and not set(nav_by_id) & set(records), 'navigation unique ids')
    require({m['evidence_id'] for m in nav_manifest} == set(nav_by_id), 'navigation bijection')
    expected_navigation = {('resolve_containing_procedure', address) for address in
        ('0x8093231','0x80876f3','0x80bc757','0x8088d2f','0x80855a8')} | {('xrefs','0x0847aacc'),('search_procedures','global')}
    actual_navigation = set()
    for item in nav_manifest:
        record = nav_by_id[item['evidence_id']]
        require(record['provider'] == PROVIDER and record['subject']['digest']['sha256'] == TARGET, 'navigation subject/provider')
        require(item == {k:record[k] for k in ('evidence_id','operation','parameters','limitations')}, 'navigation manifest exact')
        params = record['parameters']
        if record['operation'] == 'search_procedures':
            require(params == dict(case_sensitive=False,mode='literal',pattern='global'), 'navigation search parameters')
            actual_navigation.add((record['operation'],params['pattern']))
        else:
            require(set(params) == {'address'}, 'navigation address parameters')
            actual_navigation.add((record['operation'],params['address']))
    require(actual_navigation == expected_navigation, 'navigation operations')
    with (root / 'docs/evidence/actions.csv').open(newline='') as stream:
        actions = {int(r['opcode'],16):r for r in csv.DictReader(stream)}
    for opcode,name,parameters in ((0x35,'vm_pushGlobalVar','1'),(0x36,'vm_pushGlobalVptr','1'),
                                   (7,'vm_callInUserFun','2'),(6,'vm_callInEfun','2')):
        action=actions[opcode]
        require(action['name']==name and int(action['entry'],16)==ENTRIES[name] and
                action['parameters']==parameters, 'opcode binding')
    site_count = 0
    for name, sites in SITES.items():
        candidates=[r for r in records.values() if r['parameters']['procedure']==name and
                    r['operation'] in ('procedure_assembly','analyze_function')]
        require(len(candidates)==1, 'selected assembly record')
        result=candidates[0]['normalized_result']
        assembly=result['assembly'] if isinstance(result,dict) else result
        lines=assembly.splitlines() if isinstance(assembly,str) else assembly
        listing=dict(line.split(': ',1) for line in lines)
        require(len(listing)==len(lines), 'duplicate assembly address')
        for address,instruction in sites.items():
            require(listing.get(f'0x{address:x}')==instruction, f'instruction site {address:#x}')
            site_count += 1
    return dict(records=len(records), navigation_records=len(nav_records),source_bundles=len(reused['sources']),
        instruction_sites=site_count, limitation='Integrity and selected instruction checks do not prove all global storage/call semantic claims.')


if __name__ == '__main__':
    print(json.dumps(verify_bundle(), sort_keys=True))
