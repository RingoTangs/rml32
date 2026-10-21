"""Verify frozen delivery and unchanged baseline without executing reference artifacts."""
import hashlib
import json
from pathlib import Path
from verify_evidence import verify as verify_evidence

ROOT = Path(__file__).resolve().parents[2]
ENTRY = {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}


def check_hashes(root, entries):
    for name, expected in entries.items():
        path = Path(name)
        if path.is_absolute() or '..' in path.parts:
            raise ValueError('unsafe manifest path')
        target = (root / path).resolve()
        if not target.is_relative_to(root.resolve()):
            raise ValueError('manifest path escape')
        if hashlib.sha256(target.read_bytes()).hexdigest() != expected:
            raise ValueError('hash mismatch: ' + name)


def verify(root=ROOT):
    root = Path(root).resolve()
    here = root / 'docs/stage2ai-evidence'
    record = json.loads((root / 'docs/stage2ai-validation.json').read_text())
    boundary = dict(stage='2AI', runtime_changed=False, public_api_changed=False,
                    dependencies_changed=False, rust_tests=473, python_tests=29,
                    supported_opcode_kinds=75, intersection_opcode_kinds=73,
                    target_execution='none', stage2_complete=False,
                    new_native_analysis=True, native_session_closed=True)
    if any(record.get(k) != v for k, v in boundary.items()):
        raise ValueError('stage boundary mismatch')
    inventory = {str(p.relative_to(root)) for p in here.iterdir() if p.is_file()}
    inventory |= ENTRY | {'docs/vm-bytes-semantics.md'}
    if set(record['delivery_files']) != inventory:
        raise ValueError('delivery inventory mismatch')
    protected = record['protected_files']
    required = {'Cargo.toml', 'Cargo.lock', 'magic_Linux32', 'docs/stage2ah-validation.json'}
    required |= {str(p.relative_to(root)) for directory in ('src', 'tests', 'docs/stage2ah-evidence')
                 for p in (root / directory).rglob('*') if p.is_file() and '__pycache__' not in p.parts}
    if not required <= protected.keys() or ENTRY & protected.keys():
        raise ValueError('protected inventory mismatch')
    check_hashes(root, protected)
    check_hashes(root, record['delivery_files'])
    evidence = verify_evidence(root, here)
    if evidence != record['evidence_checks']:
        raise ValueError('evidence result mismatch')
    corpus = json.loads((here / 'corpus-validation.json').read_text())
    if corpus['status'] != 'pass' or corpus['originals_unchanged'] != 36 or corpus['target_execution'] != 'none':
        raise ValueError('corpus validation mismatch')
    if corpus['known_errors'] != ['dba/pack_data/etc.pak/etc']:
        raise ValueError('corpus known-error boundary')
    commands = record['commands']
    if not commands or any(c['exit_code'] != 0 for c in commands):
        raise ValueError('failed validation command')
    return dict(status='pass', protected_files=len(protected), delivery_files=len(inventory),
                rust_tests=473, python_tests=29, **evidence)


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
