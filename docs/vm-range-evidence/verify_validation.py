"""Read-only delivery, protected baseline, and stage 2X integrity verification."""
import hashlib
import json
from pathlib import Path

from verify_evidence import verify_bundle

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    record = json.loads((root / 'docs/stage2x-validation.json').read_text())
    if (record['stage'] != '2X' or record['runtime_changed']
            or record['dependencies_changed'] or record['target_execution'] != 'none'
            or record['supported_opcode_kinds'] != 68):
        raise ValueError('stage boundary mismatch')
    for group in ('protected_files', 'delivery_files'):
        for name, expected in record[group].items():
            path = (root / name).resolve()
            if not path.is_relative_to(root.resolve()):
                raise ValueError('file outside repository')
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise ValueError(f'{group} hash mismatch: {name}')
    evidence = verify_bundle(root=root, here=root / 'docs/vm-range-evidence')
    if evidence['records'] != record['retained_records']:
        raise ValueError('record count mismatch')
    corpus = json.loads((root / 'docs/vm-range-evidence/corpus-summary.json').read_text())
    if corpus['occurrence_count'] != record['range_occurrences']:
        raise ValueError('corpus count mismatch')
    return {'status': 'pass', 'protected_files': len(record['protected_files']),
            'delivery_files': len(record['delivery_files']), 'evidence': evidence}


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
