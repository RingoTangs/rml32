"""Read-only stage 2AA delivery/baseline checks; no global bytecode execution."""
import gzip
import hashlib
import json
from pathlib import Path
import re

from verify_evidence import verify_bundle

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    record = json.loads((root / 'docs/stage2aa-validation.json').read_text())
    if (record['stage'] != '2AA' or record['runtime_changed']
            or record['dependencies_changed'] or record['public_api_changed']
            or record['target_execution'] != 'none'
            or record['supported_opcode_kinds'] != 70):
        raise ValueError('stage boundary mismatch')
    for group in ('protected_files', 'delivery_files'):
        for name, expected in record[group].items():
            path = (root / name).resolve()
            if not path.is_relative_to(root.resolve()):
                raise ValueError('file outside repository')
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise ValueError(f'{group} hash mismatch: {name}')
    here = root / 'docs/stage2aa-evidence'
    evidence = verify_bundle(root=root, here=here)
    corpus = json.loads((here / 'corpus-summary.json').read_text())
    if (corpus['occurrence_count'], corpus['global_units']) != (47003, 5):
        raise ValueError('corpus count mismatch')
    matrix = json.loads((root / 'docs/stage2z-evidence/summary.json').read_text())
    if (matrix['supported_opcode_kinds'], matrix['intersection_opcode_kinds'],
            matrix['missing']) != (70, 68, ['05', '06', '17', '35', '36']):
        raise ValueError('current matrix boundary mismatch')
    ids = set()
    for name in ('rea.json.gz', 'reused.json.gz', 'navigation.json.gz'):
        with gzip.open(here / name, 'rt') as stream:
            ids.update(r['evidence_id'] for r in json.load(stream)['records'])
    report = (root / 'docs/vm-global-semantics.md').read_text()
    if not set(re.findall(r'ev_[a-f0-9]{64}', report)).issubset(ids):
        raise ValueError('unknown report evidence ID')
    return {'status': 'pass', 'protected_files': len(record['protected_files']),
            'delivery_files': len(record['delivery_files']), 'native_evidence': evidence,
            'supported_opcode_kinds': 70, 'runtime_changed': False}


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
