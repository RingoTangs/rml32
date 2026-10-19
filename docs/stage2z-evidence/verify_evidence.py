"""Read-only integrity and selected instruction checks, not a proof of reference-check semantics."""
import csv
import gzip
import hashlib
import json
from pathlib import Path

TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
PROVIDER = {'id': 'ghidra', 'name': 'Ghidra', 'version': '12.1.4'}
ENTRIES = dict(vm_checkIsRefType=0x8091750, vm_stackInThisContext=0x80933e4,
               vm_argumentInThisContext=0x8093400, vm_callInUserFunBody=0x8093eb0,
               vm_regularProcessor=0x80978c4)
SITES = {0x809175b: 'TEST ECX, ECX', 0x8091762: 'CALL 0x08093400',
         0x8091767: 'CMP EAX, EBX', 0x8091769: 'JG 0x080917c4',
         0x80917c4: 'CALL 0x080933e4', 0x80917c9: 'ADD EBX, EAX',
         0x80917d1: 'CMP EBX, EDX', 0x80917d3: 'JL 0x08091771',
         0x80917e0: 'CMP byte ptr [ECX + EAX*0x8 + 0x4], 0x8',
         0x80917e5: 'SETZ BL', 0x80917ec: 'LEA EAX, [EDX + -0x1]'}


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
    for filename, count in [('rea.json.gz', 2), ('reused.json.gz', 5)]:
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
                require((record['parameters']['procedure'], record['operation']) in
                        {('vm_stackInThisContext','analyze_function'),
                         ('vm_checkIsRefType','procedure_assembly')}, 'new operation')
                provenance[eid] = ('docs/stage2z-evidence/rea.json.gz', digest(path))
    reused = bundles['reused.json.gz']
    require(len(reused['sources']) == 4, 'source count')
    require(len({s['path'] for s in reused['sources']}) == 4, 'duplicate source')
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
    with (root / 'docs/evidence/actions.csv').open(newline='') as stream:
        actions = [r for r in csv.DictReader(stream) if int(r['opcode'], 16) == 0x10]
    require(len(actions) == 1, 'opcode binding count')
    action = actions[0]
    require(action['name'] == 'vm_checkIsRefType' and int(action['entry'], 16) == ENTRIES['vm_checkIsRefType'] and action['parameters'] == '1', 'opcode binding')
    query = [r for r in bundles['rea.json.gz']['records'] if r['parameters']['procedure'] == 'vm_checkIsRefType']
    require(len(query) == 1, 'query record')
    lines = query[0]['normalized_result'].splitlines()
    listing = dict(line.split(': ', 1) for line in lines)
    require(len(listing) == len(lines), 'duplicate assembly address')
    for address, instruction in SITES.items():
        require(listing.get(f'0x{address:x}') == instruction, f'instruction site {address:#x}')
    positions = [lines.index(f'0x{address:x}: {SITES[address]}')
                 for address in (0x809175b, 0x80917e0, 0x80917ec)]
    require(positions == sorted(positions), 'guard tag push order')
    helper = next(r for r in bundles['rea.json.gz']['records']
                  if r['parameters']['procedure'] == 'vm_stackInThisContext')
    require('0x80933fb: MOV EAX, dword ptr [EAX + EDX*0x8]' in
            helper['normalized_result']['assembly'], 'context base load')
    return {'records': len(records), 'source_bundles': len(reused['sources']), 'instruction_sites': len(SITES),
            'limitation': 'Integrity and selected instruction checks do not prove all reference-check semantics.'}


if __name__ == '__main__':
    print(json.dumps(verify_bundle(), sort_keys=True))
