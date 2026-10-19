"""Read-only stage 2Y delivery and protected-file verification."""
import hashlib
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    record = json.loads((root / 'docs/stage2y-validation.json').read_text())
    if (record['stage'] != '2Y' or not record['runtime_changed']
            or record['dependencies_changed'] or record['public_api_changed']
            or record['target_execution'] != 'none'
            or record['supported_opcode_kinds'] != 69):
        raise ValueError('stage boundary mismatch')
    for group in ('protected_files', 'delivery_files'):
        for name, expected in record[group].items():
            path = (root / name).resolve()
            if not path.is_relative_to(root.resolve()):
                raise ValueError('file outside repository')
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
                raise ValueError(f'{group} hash mismatch: {name}')
    here = root / 'docs/vm-range-evidence'
    spec = importlib.util.spec_from_file_location('range_evidence', here / 'verify_evidence.py')
    evidence = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(evidence)
    native = evidence.verify_bundle(root=root, here=here)
    summary = json.loads((root / 'docs/stage2y-evidence/summary.json').read_text())
    if (summary['supported_opcode_kinds'], summary['intersection_opcode_kinds'],
            summary['missing']) != (69, 67, ['05', '06', '10', '17', '35', '36']):
        raise ValueError('matrix boundary mismatch')
    for name, expected in summary['source_sha256'].items():
        path = (root / name).resolve()
        if not path.is_relative_to(root.resolve()):
            raise ValueError('matrix source outside repository')
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f'matrix source mismatch: {name}')
    return {'status': 'pass', 'protected_files': len(record['protected_files']),
            'delivery_files': len(record['delivery_files']), 'native_evidence': native,
            'supported_opcode_kinds': 69}


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
