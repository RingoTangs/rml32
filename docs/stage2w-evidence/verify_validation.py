#!/usr/bin/env python3
"""Read-only verification of current stage 2W delivery and preserved history."""
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
    report = json.loads((ROOT / 'docs/stage2w-validation.json').read_text())
    for group in ('protected_files', 'delivery_files'):
        for path, expected in report[group].items():
            assert sha(ROOT / path) == expected, f'{group}: changed {path}'
    assert report['runtime_changed'] and not report['dependencies_changed']
    assert report['target_execution'] == 'none' and report['new_rea_queries'] == 0
    assert report['checks']['cargo test --locked']['total'] == 323
    audit = module('stage2w_audit', HERE / 'verify_completion.py')
    outputs, current = audit.build()
    for name, data in outputs.items():
        assert (HERE / name).read_text() == data, f'stale {name}'
    assert (current['supported_opcode_kinds'], current['intersection_opcode_kinds'],
            len(current['missing'])) == (68, 66, 7)
    tests = audit.test_names((ROOT / 'tests/vm_core.rs').read_text())
    unit = audit.test_names((ROOT / 'src/vm.rs').read_text())
    assert len(tests) == 282
    assert len(report['new_vm_tests']) == 17 and set(report['new_vm_tests']) <= tests
    assert len(report['new_unit_tests']) == 3 and set(report['new_unit_tests']) <= unit
    renamed = report['renamed_vm_tests']
    assert renamed == {'catches_marker_limit_and_compound_zero_remain_uncatchable':
                       'catches_marker_limit_remains_fatal_and_compound_zero_uses_handler'}
    previous = {renamed.get(name, name) for name in report['previous_vm_tests']}
    assert previous <= tests and tests - previous == set(report['new_vm_tests'])
    evidence = module('numeric_evidence_checks', HERE / 'verify_evidence.py')
    observed = evidence.verify_bundle(ROOT, HERE)
    assert observed == dict(records=8, source_bundles=3, sites=7, private_sources=5)
    source = (ROOT / 'src/vm.rs').read_text()
    variants = re.search(r'enum RuntimeFaultSite\s*\{(.*?)\}', source, re.S).group(1)
    assert set(re.findall(r'^\s*(\w+),\s*$', variants, re.M)) == evidence.SOURCES
    records = {r['evidence_id']:r for r in json.loads(gzip.decompress((HERE / 'reused.json.gz').read_bytes()))['records']}
    document = ROOT / 'docs/vm-numeric-catch-runtime.md'
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
    print(json.dumps(dict(status='pass', supported_bounded=68, new_rust_tests=20,
        retained_numeric_ghidra_records=8, fault_paths=7,
        protected_files=len(report['protected_files']), delivery_files=len(report['delivery_files']),
        target_execution='none'), sort_keys=True))


if __name__ == '__main__':
    main()
