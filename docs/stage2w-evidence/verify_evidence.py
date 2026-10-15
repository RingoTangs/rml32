"""Read-only verification of stage 2W's retained Ghidra evidence."""
import gzip
import hashlib
import json
from pathlib import Path
import re

TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
ENTRIES = {'vm_divValue': 0x0808e1a4, 'vm_modValue': 0x0808e7bc,
           'vm_opEqValue': 0x0808eb68, 'vm_copyValue': 0x0808be24}
SOURCES = {'IntDivZero', 'NumericDivZero', 'IntRemZero',
           'CompoundDivZero', 'CompoundIntRemZero'}
DIRECT = [
    ('IntDivZero', '19', ('Int', 'Int'), 0x808e27f, 0x808e2ea),
    ('NumericDivZero', '19', ('Int', 'Real'), 0x808e24f, 0x808e29d),
    ('NumericDivZero', '19', ('Real', 'Int'), 0x808e269, 0x808e2b2),
    ('NumericDivZero', '19', ('Real', 'Real'), 0x808e22d, 0x808e2cf),
    ('IntRemZero', '28', ('Int', 'Int'), 0x808e848, 0x808e8ae),
]


def require(ok, message):
    if not ok:
        raise ValueError(message)


def read_json(path):
    if path.suffix == '.gz':
        with gzip.open(path, 'rt') as stream:
            return json.load(stream)
    return json.loads(path.read_text())


def verify_bundle(root=None, here=None):
    """Return verification counts; raise ValueError for invalid retained evidence."""
    here = Path(here) if here is not None else Path(__file__).resolve().parent
    root = Path(root) if root is not None else here.parent.parent
    bundle = read_json(here / 'reused.json.gz')
    manifest = read_json(here / 'function-manifest.json')
    faults = read_json(here / 'fault-sites.json')
    require(len(bundle['records']) == 8, 'record count')
    require(len(bundle['sources']) == 3, 'source count')
    require(bundle['artifacts'][0]['digest']['sha256'] == TARGET, 'artifact target')
    require(bundle['providers'] == [{'id': 'ghidra', 'name': 'Ghidra', 'version': '12.1.4'}], 'bundle provider')
    require(faults['target_sha256'] == TARGET, 'fault target')
    require(faults['provider'] == 'ghidra' and faults['provider_version'] == '12.1.4', 'fault provider')
    records = {r['evidence_id']: r for r in bundle['records']}
    require(len(records) == 8, 'duplicate record')
    provenance = {}
    for source in bundle['sources']:
        path = root / source['path']
        require(path.resolve().is_relative_to(root.resolve()), 'source outside root')
        require(hashlib.sha256(path.read_bytes()).hexdigest() == source['sha256'], 'source hash')
        original = read_json(path)
        original_records = {r['evidence_id']: r for r in original['records']}
        for eid in source['evidence_ids']:
            require(eid in records and eid in original_records, 'unknown source evidence ID')
            require(records[eid] == original_records[eid], 'edited retained record')
            require(eid not in provenance, 'duplicate provenance')
            provenance[eid] = source
    require(set(provenance) == set(records), 'missing record provenance')
    require(len(manifest) == 8, 'manifest count')
    require({m['evidence_id'] for m in manifest} == set(records), 'manifest evidence IDs')
    for m in manifest:
        eid = m['evidence_id']
        r = records[eid]
        require(r['subject']['digest']['sha256'] == TARGET, 'record target')
        require(r['provider']['id'] == 'ghidra' and r['provider']['version'] == '12.1.4', 'record provider')
        require(m['name'] == r['parameters']['procedure'] and m['operation'] == r['operation'], 'manifest operation')
        require(m['name'] in ENTRIES and int(m['address'], 16) == ENTRIES[m['name']], 'manifest address')
        require(m['source'] == provenance[eid]['path'] and m['source_sha256'] == provenance[eid]['sha256'], 'manifest provenance')
        require(m['limitations'] == r['limitations'], 'manifest limitations')
    def instructions(ids):
        require(all(eid in records for eid in ids), 'unknown site evidence ID')
        listing = {}
        for eid in ids:
            r = records[eid]
            if r['operation'] == 'procedure_assembly':
                for line in r['normalized_result'].splitlines():
                    match = re.match(r'(0x[0-9a-f]+): (.*)', line)
                    if match:
                        listing[int(match[1], 16)] = match[2]
        return listing
    sites = faults['sites']
    require(len(sites) == 7, 'site count')
    require({s['rust_fault_source'] for s in sites} == SOURCES, 'fault source set')
    require(sum('operand_types' in s for s in sites) == 5, 'direct type count')
    seen = set()
    for s in sites:
        listing = instructions(s['evidence_ids'])
        require(s['native_function'] in ENTRIES and int(s['native_entry'], 16) == ENTRIES[s['native_function']], 'site native entry')
        if 'operand_types' in s:
            key = (s['rust_fault_source'], s['opcode'], tuple(s['operand_types']), int(s['zero_branch'], 16), int(s['error_call'], 16))
            require(key in DIRECT and key not in seen, 'direct site type/address')
            seen.add(key)
            require(listing.get(key[3], '').startswith('JZ '), 'zero branch instruction')
            require(int(s['error_target'], 16) == 0x8097b24 and listing.get(key[4]) == 'CALL 0x08097b24', 'error CALL instruction')
            require(s['native_function'] == ('vm_divValue' if s['opcode'] == '19' else 'vm_modValue'), 'direct native function')
        else:
            is_div = s['rust_fault_source'] == 'CompoundDivZero'
            require(s['opcode'] == '2f' and s['selector'] == (5 if is_div else 11), 'compound selector')
            require(s['native_function'] == 'vm_opEqValue' and s['delegate'] == ('vm_divValue' if is_div else 'vm_modValue'), 'compound delegate')
            call = 0x808ecc2 if is_div else 0x808ec52
            target = 0x808e1a4 if is_div else 0x808e7bc
            require(int(s['delegate_call'], 16) == call and listing.get(call) == f'CALL 0x{target:08x}', 'delegate CALL instruction')
            expected = {row[4] for row in DIRECT if row[1] == ('19' if is_div else '28')}
            require({int(a, 16) for a in s['error_calls']} == expected, 'compound error sites')
            require(all(listing.get(a) == 'CALL 0x08097b24' for a in expected), 'compound error CALL instruction')
    require(seen == set(DIRECT), 'missing direct site')
    return {'records': 8, 'source_bundles': 3, 'sites': 7, 'private_sources': 5}


if __name__ == '__main__':
    print(json.dumps(verify_bundle(), sort_keys=True))
