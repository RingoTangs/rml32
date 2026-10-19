"""Verify stage 2AC delivery hashes and frozen history; never execute reference code."""
import hashlib
import json
from pathlib import Path

from verify_completion import build

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    record = json.loads((root / 'docs/stage2ac-validation.json').read_text())
    if (record['stage'], record['supported_opcode_kinds'], record['intersection_opcode_kinds'],
            record['missing'], record['target_execution'], record['dependencies_changed']) != (
            '2AC', 72, 70, ['05', '06', '17'], 'none', False):
        raise ValueError('stage boundary mismatch')
    for group in ('protected_files', 'delivery_files'):
        for name, expected in record[group].items():
            path = (root / name).resolve()
            if not path.is_relative_to(root.resolve()) or not path.is_file():
                raise ValueError('invalid delivery path: ' + name)
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise ValueError(group + ' hash mismatch: ' + name)
    corpus = json.loads((root / 'docs/stage2ac-evidence/corpus-validation.json').read_text())
    if (corpus['status'], corpus['originals_unchanged'], corpus['known_errors'],
            corpus['checked'], corpus['target_execution']) != (
            'pass', 36, ['dba/pack_data/etc.pak/etc'],
            dict(packages=12, members=9527, programs=7159, inheritance=9933), 'none'):
        raise ValueError('corpus boundary mismatch')
    outputs, summary = build()
    for name, expected in outputs.items():
        if (root / 'docs/stage2ac-evidence' / name).read_text() != expected:
            raise ValueError('stale matrix: ' + name)
    return dict(status='pass', protected_files=len(record['protected_files']),
                delivery_files=len(record['delivery_files']), supported_opcode_kinds=summary['supported_opcode_kinds'],
                intersection_opcode_kinds=summary['intersection_opcode_kinds'],
                target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
