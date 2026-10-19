"""Read-only integrity and selected instruction checks, not a proof of range semantics."""
import csv
import gzip
import hashlib
import json
from pathlib import Path

TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
PROVIDER = {'id': 'ghidra', 'name': 'Ghidra', 'version': '12.1.4'}
ENTRIES = dict(vm_queryRange=0x808fd04, vm_pushString=0x8090644,
               vm_allocateEmptyArray=0x809b1d8, vm_pushAllocedString=0x80907c0,
               vm_copyValue=0x808be24, vm_popNStack=0x8091ab0,
               vm_unrefArray=0x809b350, vm_queryRangePtr=0x809005c,
               _vm_ftol=0x80a36e4, vm_regularProcessor=0x80978c4)
SITES = {0x808fed2: 'CALL 0x080a36e4', 0x808fef2: 'CALL 0x080a36e4',
         0x808fff7: 'CALL 0x0804ba94', 0x808ffa4: 'CALL 0x0808be24',
         0x808ffb9: 'CALL 0x0809b350', 0x808ff6e: 'CALL 0x0809b1d8',
         0x808fdb7: 'TEST byte ptr [EBP + -0x10], 0x1',
         0x808fdd0: 'TEST byte ptr [EBP + -0x10], 0x2',
         0x808fdc7: 'CMOVLE EBX, EAX', 0x808fddd: 'CMOVLE EDI, EAX',
         0x808fded: 'CMOVS EDI, EAX', 0x808fdf5: 'CMOVLE EBX, EAX',
         0x808ff4f: 'CMOVS EDI, EAX', 0x808ff57: 'CMOVLE EBX, EAX',
         0x808ff5f: 'CMOVG EDI, EAX'}


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
    for filename, count in [('rea.json.gz', 4), ('reused.json.gz', 9)]:
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
                require(record['operation'] == 'analyze_function', 'new operation')
                provenance[eid] = ('docs/vm-range-evidence/rea.json.gz', digest(path))
    reused = bundles['reused.json.gz']
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
        actions = [r for r in csv.DictReader(stream) if int(r['opcode'], 16) == 0x45]
    require(len(actions) == 1, 'opcode binding count')
    action = actions[0]
    require(action['name'] == 'vm_queryRange' and int(action['entry'], 16) == ENTRIES['vm_queryRange'] and action['parameters'] == '1', 'opcode binding')
    query = [r for r in bundles['rea.json.gz']['records'] if r['parameters']['procedure'] == 'vm_queryRange']
    require(len(query) == 1, 'query record')
    listing = dict(line.split(': ', 1) for line in query[0]['normalized_result']['assembly'])
    for address, instruction in SITES.items():
        require(listing.get(f'0x{address:x}') == instruction, f'instruction site {address:#x}')
    return {'records': len(records), 'source_bundles': len(reused['sources']), 'instruction_sites': len(SITES),
            'limitation': 'Integrity and selected instruction checks do not prove all range semantics.'}


if __name__ == '__main__':
    print(json.dumps(verify_bundle(), sort_keys=True))
