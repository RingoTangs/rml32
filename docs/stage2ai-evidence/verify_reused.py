"""Verify retained-record integrity and provenance, not semantic equivalence."""
import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
ADDRESSES = {'vm_addValue': '0x808e3b8', 'vm_copyValue': '0x808be24',
             'vm_eqValue': '0x808eed4', 'vm_isEq': '0x808c908',
             'vm_cleanupValue': '0x808ad98', 'vm_geValue': '0x808ef88',
             'vm_leValue': '0x808f1c0', 'vm_gtValue': '0x808f340',
             'vm_ltValue': '0x808f4c8'}


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(',', ':')).encode()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def validate_records(bundle, manifest, root):
    """Validate complete records against source bundles under root."""
    root = Path(root).resolve()
    records = bundle['records']
    entries = manifest['records']
    ids = [r['evidence_id'] for r in records]
    mids = [r['evidence_id'] for r in entries]
    require(len(ids) == len(set(ids)), 'duplicate bundle evidence ID')
    require(len(mids) == len(set(mids)), 'duplicate manifest evidence ID')
    require(set(ids) == set(mids), 'manifest/bundle bijection')
    require(manifest['record_count'] == len(records), 'record count')
    by_id = {r['evidence_id']: r for r in records}
    cache = {}
    for item in entries:
        relative = Path(item['source'])
        require(not relative.is_absolute() and '..' not in relative.parts,
                'unsafe source path')
        path = (root / relative).resolve()
        require(path.is_relative_to(root), 'source escapes root')
        if path not in cache:
            data = path.read_bytes()
            cache[path] = (digest(data), json.loads(gzip.decompress(data)))
        source_hash, source = cache[path]
        require(source_hash == item['source_sha256'], 'source hash')
        record = by_id[item['evidence_id']]
        matches = [r for r in source['records'] if r['evidence_id'] == item['evidence_id']]
        require(len(matches) == 1, 'missing or duplicate source record')
        require(record == matches[0], 'complete source record equality')
        require(digest(canonical(record)) == item['record_sha256'], 'canonical record hash')
        require(record['provider'] == item['provider'], 'manifest provider')
        require(record['provider'].get('id') == 'ghidra' and
                record['provider'].get('version') == '12.1.4', 'provider identity')
        require(record['subject'] == item['subject'], 'manifest subject')
        require(record['subject'].get('digest', {}).get('sha256') == TARGET,
                'target identity')
        require('raw_result' in record, 'missing raw result')
        require(record['operation'] == item['operation'], 'operation')
        name = record['parameters'].get('procedure')
        require(name == item['function'], 'function name')
        normalized = record['normalized_result']
        if isinstance(normalized, dict):
            procedure = normalized['procedure']
            require(procedure['name'] == name, 'dossier function name')
            address = procedure['address']
            require(address == ADDRESSES.get(name), 'dossier address')
            require(procedure['body']['contains_entry'], 'body excludes entry')
        elif record['operation'] == 'procedure_assembly':
            address = normalized.split(':', 1)[0]
            require(address == ADDRESSES.get(name), 'assembly address')
        else:
            address = None
            require(record['operation'] == 'procedure_pseudo_code' and
                    name in normalized, 'pseudocode function label')
        require(item['address'] == address, 'manifest address')
    return {'record_count': len(records), 'provenance_verified': True}


def verify(root=ROOT, here=HERE):
    here = Path(here)
    data = (here / 'reused.json.gz').read_bytes()
    require(data[:3] == b'\x1f\x8b\x08' and data[3] == 0 and
            data[4:8] == b'\0' * 4, 'deterministic gzip header')
    manifest = json.loads((here / 'reused-manifest.json').read_text())
    require(manifest['schema_version'] == 1, 'manifest schema')
    require(manifest['bundle'] == 'reused.json.gz', 'bundle path')
    require(manifest['bundle_sha256'] == digest(data), 'bundle hash')
    bundle = json.loads(gzip.decompress(data))
    result = validate_records(bundle, manifest, root)
    require(result['record_count'] == 15, 'expected 15 retained records')
    text = (here / 'reused-functions.md').read_text()
    for record in bundle['records']:
        require(text.count(record['evidence_id']) == 1, 'readable evidence coverage')
    return result


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
