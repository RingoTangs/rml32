#!/usr/bin/env python3
"""Read-only stage 2Q delivery, history, native-evidence and corpus verification."""
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
TARGET_SHA = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    report = json.loads((ROOT / 'docs/stage2q-validation.json').read_text())
    for group in ('protected_files', 'delivery_files'):
        for name, expected in report[group].items():
            assert digest(ROOT / name) == expected, f'{group}: changed {name}'
    records = {}
    old = ROOT / 'docs/stage2-completion-evidence'
    for name, expected_count in [('rea.json.gz', 6), ('reused.json.gz', 12)]:
        bundle = json.load(gzip.open(old / name, 'rt'))
        assert len(bundle['records']) == expected_count
        for record in bundle['records']:
            assert record['subject']['digest']['sha256'] == TARGET_SHA
            assert record['provider']['id'] == 'ghidra'
            records[record['evidence_id']] = record
        for source in bundle.get('sources', []):
            path = ROOT / source['path']
            assert digest(path) == source['sha256']
            source_records = {r['evidence_id']: r for r in json.load(gzip.open(path, 'rt'))['records']}
            retained = {r['evidence_id']: r for r in bundle['records']}
            for identity in source['evidence_ids']:
                assert retained[identity] == source_records[identity]
    assert len(records) == 18
    corpus = json.loads((HERE / 'corpus-validation.json').read_text())
    assert corpus['status'] == 'pass' and corpus['originals_unchanged'] == 36
    assert corpus['known_errors'] == ['dba/pack_data/etc.pak/etc']
    assert corpus['target_execution'] == 'none'
    assert corpus['checked'] == dict(packages=12, members=9527, programs=7159, inheritance=9933)
    summary = json.loads((HERE / 'summary.json').read_text())
    assert (summary['supported_opcode_kinds'], summary['intersection_opcode_kinds'],
            len(summary['missing'])) == (59, 57, 16)
    assert summary['full_compatibility_rows'] == 0 and not summary['corpus_execution']
    source = (ROOT / 'tests/vm_core.rs').read_text()
    names = set(re.findall(r'#\[test\]\s*fn\s+(\w+)\s*\(', source))
    assert len(names) == 207
    assert set(report['new_rust_tests']) <= names and len(report['new_rust_tests']) == 25
    document = ROOT / 'docs/vm-cross-object-runtime.md'
    for path in [document, HERE / 'matrix.md']:
        for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
            if '://' not in link and not link.startswith('#'):
                assert (path.parent / link.split('#')[0]).exists(), f'broken link {path}: {link}'
    assert report['target_execution'] == 'none'
    assert report['new_rea_queries'] == 0
    print(json.dumps(dict(status='pass', protected_files=len(report['protected_files']),
                         delivery_files=len(report['delivery_files']), retained_ghidra_records=18,
                         new_rust_tests=25, target_execution='none'), sort_keys=True))


if __name__ == '__main__':
    main()
