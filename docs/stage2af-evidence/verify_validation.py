"""Verify stage 2AF files and frozen inputs; does not rerun recorded commands."""
import hashlib
import json
from pathlib import Path

from verify_evidence import verify as verify_evidence

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    root = Path(root).resolve()
    here = root / 'docs/stage2af-evidence'
    record = json.loads((root / 'docs/stage2af-validation.json').read_text())
    if (record['stage'], record['runtime_changed'], record['public_api_changed'],
            record['dependencies_changed'], record['target_execution'], record['rust_tests'],
            record['analysis_tests'], record['supported_opcode_kinds'],
            record['intersection_opcode_kinds'], record['missing']) != (
            '2AF', False, False, False, 'none', 433, 27, 73, 71, ['05', '17']):
        raise ValueError('stage boundary mismatch')
    required = {'magic_Linux32', 'Cargo.toml', 'Cargo.lock', 'src/value.rs', 'src/vm.rs',
                'src/host.rs', 'src/prepared.rs', 'src/lib.rs', 'tests/vm_core.rs',
                'docs/stage2ae-validation.json', 'docs/stage2ae-evidence/summary.json'}
    if not required <= record['protected_files'].keys():
        raise ValueError('missing protected baseline')
    expected_delivery = {str(p.relative_to(root)) for p in here.iterdir() if p.is_file()}
    expected_delivery |= {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md',
                          'docs/vm-function-value-semantics.md'}
    if set(record['delivery_files']) != expected_delivery:
        raise ValueError('delivery inventory mismatch')
    if len(record['commands']) != 8 or any(c['exit_code'] != 0 for c in record['commands']):
        raise ValueError('recorded command status')
    for group in ('protected_files', 'delivery_files'):
        for name, expected in record[group].items():
            path = (root / name).resolve()
            if Path(name).is_absolute() or not path.is_relative_to(root) or not path.is_file():
                raise ValueError('invalid validation path: ' + name)
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise ValueError(group + ' hash mismatch: ' + name)
    corpus = json.loads((here / 'corpus-validation.json').read_text())
    if (corpus['status'], corpus['originals_unchanged'], corpus['known_errors'],
            corpus['checked'], corpus['target_execution']) != (
            'pass', 36, ['dba/pack_data/etc.pak/etc'],
            dict(packages=12, members=9527, programs=7159, inheritance=9933), 'none'):
        raise ValueError('corpus boundary mismatch')
    summary = json.loads((here / 'corpus-summary.json').read_text())
    if (summary['occurrence_count'], summary['histograms']['opcodes']) != (
            35703, {'05': 3089, '17': 32614}):
        raise ValueError('05/17 corpus boundary mismatch')
    evidence = verify_evidence(root, here)
    return dict(status='pass', protected_files=len(record['protected_files']),
                delivery_files=len(record['delivery_files']), evidence_records=evidence['records'],
                rust_tests=433, analysis_tests=27, supported_opcode_kinds=73,
                target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
