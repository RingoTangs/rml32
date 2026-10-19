"""Check stage 2AD delivery and frozen inputs; does not rerun recorded commands."""
import hashlib
import json
from pathlib import Path

from verify_evidence import verify as verify_evidence

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    root = Path(root).resolve()
    here = root / 'docs/stage2ad-evidence'
    record = json.loads((root / 'docs/stage2ad-validation.json').read_text())
    if (record['stage'], record['supported_opcode_kinds'], record['intersection_opcode_kinds'],
            record['missing'], record['target_execution'], record['dependencies_changed'],
            record['runtime_changed'], record['public_api_changed'], record['rust_tests'],
            record['analysis_tests']) != (
            '2AD', 72, 70, ['05', '06', '17'], 'none', False, False, False, 407, 26):
        raise ValueError('stage boundary mismatch')
    required_protected = {'magic_Linux32', 'Cargo.toml', 'Cargo.lock', 'src/vm.rs',
                          'src/prepared.rs', 'src/lib.rs', 'tests/vm_core.rs',
                          'docs/stage2ac-validation.json'}
    if not required_protected <= record['protected_files'].keys():
        raise ValueError('missing protected baseline')
    expected_delivery = {str(p.relative_to(root)) for p in here.iterdir() if p.is_file()}
    expected_delivery |= {'README.md', 'docs/vm-efun-semantics.md'}
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
    if (corpus['status'], corpus['originals_unchanged'], corpus['known_errors'], corpus['checked'],
            corpus['target_execution']) != (
            'pass', 36, ['dba/pack_data/etc.pak/etc'],
            dict(packages=12, members=9527, programs=7159, inheritance=9933), 'none'):
        raise ValueError('corpus boundary mismatch')
    summary = json.loads((here / 'corpus-summary.json').read_text())
    if (summary['occurrence_count'], summary['registry_slots'], summary['used_slots'],
            summary['histograms']['segments'], summary['histograms']['slot_status']) != (
            143520, 559, 312, dict(functions=142702, init=818), dict(registered=143520)):
        raise ValueError('06 corpus boundary mismatch')
    evidence = verify_evidence(root, here)
    return dict(status='pass', protected_files=len(record['protected_files']),
                delivery_files=len(record['delivery_files']), evidence_records=evidence['records'],
                rust_tests=407, analysis_tests=26, supported_opcode_kinds=72,
                target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
