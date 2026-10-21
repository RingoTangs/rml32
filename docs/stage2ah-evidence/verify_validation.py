"""Check current delivery, unchanged runtime and frozen history; no target execution."""
import json
from pathlib import Path
from audit_rules import verify_hashes
from verify_completion import build

ROOT = Path(__file__).resolve().parents[2]


def verify(root=ROOT):
    root = Path(root).resolve()
    here = root / 'docs/stage2ah-evidence'
    record = json.loads((root/'docs/stage2ah-validation.json').read_text())
    expected = dict(stage='2AH', runtime_changed=False, public_api_changed=False,
                    dependencies_changed=False, rust_tests=473, supported_opcode_kinds=75,
                    intersection_opcode_kinds=73, target_execution='none', stage2_complete=False,
                    new_native_analysis=False)
    if any(record.get(k) != v for k, v in expected.items()):
        raise ValueError('stage boundary mismatch')
    delivery = {str(p.relative_to(root)) for p in here.iterdir() if p.is_file()}
    delivery |= {'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md',
                 'docs/stage2-completion-assessment-2ah.md'}
    if set(record['delivery_files']) != delivery:
        raise ValueError('delivery inventory mismatch')
    protected = record['protected_files']
    required = {str(p.relative_to(root)) for directory in ('src', 'tests')
                for p in (root/directory).rglob('*') if p.is_file()}
    required |= {'Cargo.toml','Cargo.lock','magic_Linux32','docs/stage2ag-validation.json',
                 'docs/stage2-completion-assessment.md'}
    required |= {str(p.relative_to(root)) for p in (root/'docs/stage2ag-evidence').iterdir() if p.is_file()}
    if not required <= set(protected):
        raise ValueError('missing frozen runtime/history baseline')
    if set(record['before_entry_sha256']) != {'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}:
        raise ValueError('missing prior entry identities')
    previous = json.loads((root/'docs/stage2ag-validation.json').read_text())
    if any(value != previous['delivery_files'].get(path) for path,value in record['before_entry_sha256'].items()):
        raise ValueError('entry baseline differs from verified 2AG delivery')
    verify_hashes(root, protected)
    verify_hashes(root, record['delivery_files'])
    commands = record['commands']
    expected_commands = [
        'cargo test --locked --offline', 'cargo fmt --check',
        'cargo clippy --locked --offline --all-targets -- -D warnings',
        'python3 -B docs/stage2ag-evidence/verify_completion.py',
        'python3 -B docs/stage2ah-evidence/verify_completion.py',
        "python3 -B -m unittest discover -s docs/stage2ah-evidence -p test_*.py",
        'python3 -B tests/compare_corpus.py --report target/stage2ah-corpus-validation.json']
    if [c['command'] for c in commands] != expected_commands or any(c['exit_code'] for c in commands):
        raise ValueError('command status or inventory mismatch')
    if commands[0]['result'] != dict(unit=52, parser_cli=23, vm=398, total=473):
        raise ValueError('Rust baseline changed')
    if commands[5]['result'] != dict(tests=41):
        raise ValueError('audit test count differs')
    corpus = json.loads((here/'corpus-validation.json').read_text())
    if (corpus['status'], corpus['originals_unchanged'], corpus['known_errors'],
            corpus['checked'], corpus['target_execution']) != (
            'pass', 36, ['dba/pack_data/etc.pak/etc'],
            dict(packages=12,members=9527,programs=7159,inheritance=9933),'none'):
        raise ValueError('corpus boundary mismatch')
    if commands[6]['result'] != corpus:
        raise ValueError('recorded corpus result differs')
    outputs, summary = build()
    if commands[4]['result'] != {k: v for k, v in summary.items() if k != 'source_sha256'}:
        raise ValueError('recorded audit differs')
    for name, expected_text in outputs.items():
        if (here/name).read_text() != expected_text:
            raise ValueError('stale generated output: '+name)
    return dict(status='pass', protected_files=len(protected), delivery_files=len(delivery),
                rust_tests=473, audit_tests=41, gaps=summary['gaps'],
                stage2_complete=False, selected_next=summary['selected_next'], target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))
