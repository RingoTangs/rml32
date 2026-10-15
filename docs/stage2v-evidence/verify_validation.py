#!/usr/bin/env python3
"""Read-only stage 2V delivery, retained evidence and regression accounting."""
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


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


def main():
    report = json.loads((ROOT / 'docs/stage2v-validation.json').read_text())
    for group in ('protected_files', 'delivery_files'):
        for name, expected in report[group].items():
            assert sha(ROOT / name) == expected, f'{group}: changed {name}'
    assert report['runtime_changed'] and not report['dependencies_changed']
    assert report['target_execution'] == 'none' and report['new_rea_queries'] == 0
    assert report['checks']['cargo test --locked']['total'] == 303
    audit = module('stage2v_audit', HERE / 'verify_completion.py')
    outputs, current = audit.build()
    for name, data in outputs.items():
        assert (HERE / name).read_text() == data, f'stale {name}'
    assert (current['supported_opcode_kinds'], current['intersection_opcode_kinds'],
            len(current['missing'])) == (68, 66, 7)
    tests = audit.test_names((ROOT / 'tests/vm_core.rs').read_text())
    unit = audit.test_names((ROOT / 'src/vm.rs').read_text())
    assert len(tests) == 265
    assert len(report['new_vm_tests']) == 20 and set(report['new_vm_tests']) <= tests
    assert len(report['new_unit_tests']) == 5 and set(report['new_unit_tests']) <= unit
    assert set(report['previous_vm_tests']) <= tests
    retained = ROOT / 'docs/vm-catch-evidence'
    evidence = module('catch_evidence_checks', retained / 'verify_evidence.py')
    fresh = json.loads(gzip.decompress((retained / 'rea.json.gz').read_bytes()))
    reused = json.loads(gzip.decompress((retained / 'reused.json.gz').read_bytes()))
    records = evidence.check_records(fresh['records'] + reused['records'])
    evidence.check_sources(reused, ROOT)
    assert len(records) == 22
    document = ROOT / 'docs/vm-catch-runtime.md'
    for identity in re.findall(r'ev_[0-9a-f]{64}', document.read_text()):
        assert identity in records
    for path in [document, HERE / 'matrix.md', ROOT / 'README.md',
                 ROOT / 'docs/magic_Linux32-rust-rewrite-roadmap.md']:
        for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
            if '://' not in link and not link.startswith('#'):
                assert (path.parent / link.split('#')[0]).exists(), f'broken link: {path}: {link}'
    corpus = json.loads((HERE / 'corpus-validation.json').read_text())
    assert corpus['status'] == 'pass' and corpus['originals_unchanged'] == 36
    assert corpus['target_execution'] == 'none'
    assert corpus['known_errors'] == ['dba/pack_data/etc.pak/etc']
    assert corpus['summary']['programs'] == 7159 and corpus['summary']['instructions'] == 6474773
    print(json.dumps(dict(status='pass', supported_bounded=68, new_rust_tests=25,
        retained_ghidra_records=22, protected_files=len(report['protected_files']),
        delivery_files=len(report['delivery_files']), target_execution='none'), sort_keys=True))


if __name__ == '__main__':
    main()
