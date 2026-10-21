"""Verify the current delivery and frozen history; never execute reference code."""
import hashlib
import json
from pathlib import Path
from verify_completion import build

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    root = Path(root).resolve()
    here = root / 'docs/stage2ag-evidence'
    record = json.loads((root / 'docs/stage2ag-validation.json').read_text())
    expected = dict(stage='2AG', supported_opcode_kinds=75, intersection_opcode_kinds=73,
                    missing=[], target_execution='none', dependencies_changed=False,
                    public_api_changed=True, rust_tests=473)
    if any(record.get(k) != v for k, v in expected.items()):
        raise ValueError('stage boundary mismatch')
    delivery = {str(p.relative_to(root)) for p in here.iterdir() if p.is_file()}
    delivery |= {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md',
                 'docs/vm-function-value-runtime.md', 'src/lib.rs', 'src/value.rs',
                 'src/object.rs', 'src/numeric.rs', 'src/prepared.rs', 'src/vm.rs',
                 'tests/vm_core.rs'}
    if set(record['delivery_files']) != delivery:
        raise ValueError('delivery inventory mismatch')
    protected = record['protected_files']
    if not {'magic_Linux32', 'Cargo.toml', 'Cargo.lock', 'src/host.rs',
            'src/vm_error.rs', 'src/container.rs', 'src/cell.rs',
            'docs/stage2af-validation.json', 'docs/stage2ae-validation.json',
            'docs/stage2af-evidence/function-runtime-design.md'} <= protected.keys():
        raise ValueError('missing protected baseline')
    if set(record['before_source_sha256']) != {
            'src/lib.rs', 'src/value.rs', 'src/object.rs', 'src/numeric.rs',
            'src/prepared.rs', 'src/vm.rs', 'tests/vm_core.rs'}:
        raise ValueError('missing implementation baseline identities')
    if len(record['commands']) != 11 or any(c['exit_code'] != 0 for c in record['commands']):
        raise ValueError('recorded command status')
    commands = record['commands']
    for i in (1, 2):
        if commands[i]['result'] != dict(unit=52, parser_cli=23, vm=398, total=473):
            raise ValueError('Rust test counts differ')
    if commands[7]['result'] != dict(tests=27) or commands[9]['result'] != dict(tests=18):
        raise ValueError('Python test counts differ')
    for group in ('protected_files', 'delivery_files'):
        for name, expected_hash in record[group].items():
            path = (root / name).resolve()
            if Path(name).is_absolute() or not path.is_relative_to(root) or not path.is_file():
                raise ValueError('invalid validation path: ' + name)
            if hashlib.sha256(path.read_bytes()).hexdigest() != expected_hash:
                raise ValueError(group + ' hash mismatch: ' + name)
    corpus = json.loads((here / 'corpus-validation.json').read_text())
    if (corpus['status'], corpus['originals_unchanged'], corpus['known_errors'],
            corpus['checked'], corpus['target_execution']) != (
            'pass', 36, ['dba/pack_data/etc.pak/etc'],
            dict(packages=12, members=9527, programs=7159, inheritance=9933), 'none'):
        raise ValueError('corpus boundary mismatch')
    outputs, summary = build()
    if commands[8]['result'] != {k: v for k, v in summary.items() if k != 'source_sha256'}:
        raise ValueError('recorded matrix differs')
    for name, expected_text in outputs.items():
        if (here / name).read_text() != expected_text:
            raise ValueError('stale current matrix: ' + name)
    return dict(status='pass', protected_files=len(protected), delivery_files=len(delivery),
                rust_tests=473, supported_opcode_kinds=75,
                intersection_opcode_kinds=73, target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
