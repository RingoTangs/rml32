"""Verify current delivery and frozen history; never execute reference code."""
import hashlib
import json
from pathlib import Path

from verify_completion import build

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    root = Path(root).resolve()
    here = root / 'docs/stage2ae-evidence'
    record = json.loads((root / 'docs/stage2ae-validation.json').read_text())
    if (record['stage'], record['supported_opcode_kinds'], record['intersection_opcode_kinds'],
            record['missing'], record['target_execution'], record['dependencies_changed'],
            record['rust_tests'], record['public_api_changed']) != (
            '2AE', 73, 71, ['05', '17'], 'none', False, 433, True):
        raise ValueError('stage boundary mismatch')
    expected_delivery = {str(p.relative_to(root)) for p in here.iterdir() if p.is_file()}
    expected_delivery |= {'README.md', 'docs/vm-efun-runtime.md', 'src/host.rs',
                          'src/lib.rs', 'src/object.rs', 'src/prepared.rs',
                          'src/vm.rs', 'src/cell.rs', 'tests/vm_core.rs'}
    if set(record['delivery_files']) != expected_delivery:
        raise ValueError('delivery inventory mismatch')
    protected = record['protected_files']
    if not {'magic_Linux32', 'Cargo.toml', 'Cargo.lock', 'docs/stage2ad-validation.json',
            'docs/stage2ad-evidence/host-runtime-design.md', 'src/value.rs', 'src/vm_error.rs'} <= protected.keys():
        raise ValueError('missing protected baseline')
    if len(record['commands']) != 11 or any(c['exit_code'] != 0 for c in record['commands']):
        raise ValueError('recorded command status')
    for group in ('protected_files', 'delivery_files'):
        for name, expected in record[group].items():
            path = (root / name).resolve()
            if Path(name).is_absolute() or not path.is_relative_to(root) or not path.is_file():
                raise ValueError('invalid validation path: ' + name)
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise ValueError(group + ' hash mismatch: ' + name)
    corpus = json.loads((here / 'corpus-validation.json').read_text())
    if (corpus['status'], corpus['originals_unchanged'], corpus['known_errors'], corpus['checked'],
            corpus['target_execution']) != (
            'pass', 36, ['dba/pack_data/etc.pak/etc'],
            dict(packages=12, members=9527, programs=7159, inheritance=9933), 'none'):
        raise ValueError('corpus boundary mismatch')
    outputs, summary = build()
    for name, expected in outputs.items():
        if (here / name).read_text() != expected:
            raise ValueError('stale current matrix: ' + name)
    return dict(status='pass', protected_files=len(protected),
                delivery_files=len(record['delivery_files']), rust_tests=433,
                supported_opcode_kinds=summary['supported_opcode_kinds'],
                intersection_opcode_kinds=summary['intersection_opcode_kinds'],
                target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
