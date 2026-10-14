#!/usr/bin/env python3
"""Read-only stage 2S delivery, history and retained native-evidence checks."""
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
TARGET_SHA = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    report = json.loads((ROOT / 'docs/stage2s-validation.json').read_text())
    for group in ('protected_files', 'delivery_files'):
        for name, expected in report[group].items():
            assert sha(ROOT / name) == expected, f'{group}: changed {name}'
    records = {}
    old = ROOT / 'docs/vm-control-flow-evidence'
    for name, expected in [('rea.json.gz', 12), ('reused.json.gz', 5)]:
        bundle = json.load(gzip.open(old / name, 'rt'))
        assert len(bundle['records']) == expected
        for r in bundle['records']:
            assert r['subject']['digest']['sha256'] == TARGET_SHA
            assert r['provider']['id'] == 'ghidra'
            records[r['evidence_id']] = r
        for source in bundle.get('sources', []):
            path = ROOT / source['path']
            assert sha(path) == source['sha256']
            original = {r['evidence_id']:r for r in json.load(gzip.open(path,'rt'))['records']}
            local = {r['evidence_id']:r for r in bundle['records']}
            for identity in source['evidence_ids']:
                assert local[identity] == original[identity]
    assert len(records) == 17
    spec = importlib.util.spec_from_file_location('audit', HERE / 'verify_completion.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    outputs, current = module.build()
    for name, data in outputs.items():
        assert (HERE / name).read_text() == data
    assert (current['supported_opcode_kinds'], current['intersection_opcode_kinds'],len(current['missing'])) == (62,60,13)
    tests = module.test_names((ROOT / 'tests/vm_core.rs').read_text())
    assert len(tests) == 224 and len(report['new_rust_tests']) == 17
    assert set(report['new_rust_tests']) <= tests
    document = ROOT / 'docs/vm-switch-runtime.md'
    for identity in re.findall(r'ev_[0-9a-f]{64}',document.read_text()):
        assert identity in records
    for path in [document, HERE / 'matrix.md', ROOT / 'README.md', ROOT / 'docs/magic_Linux32-rust-rewrite-roadmap.md']:
        for link in re.findall(r'\]\(([^)]+)\)',path.read_text()):
            if '://' not in link and not link.startswith('#'):
                assert (path.parent / link.split('#')[0]).exists(), f'broken link {path}: {link}'
    corpus = json.loads((HERE / 'corpus-validation.json').read_text())
    assert corpus['status'] == 'pass' and corpus['originals_unchanged'] == 36
    assert corpus['target_execution'] == 'none'
    assert corpus['known_errors'] == ['dba/pack_data/etc.pak/etc']
    assert report['runtime_changed'] and report['new_rea_queries'] == 0
    assert report['target_execution'] == 'none'
    print(json.dumps(dict(status='pass',protected_files=len(report['protected_files']),
        delivery_files=len(report['delivery_files']),new_rust_tests=17,
        supported_bounded=62,retained_ghidra_records=17,target_execution='none'),sort_keys=True))


if __name__ == '__main__':
    main()
