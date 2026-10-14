#!/usr/bin/env python3
"""Read-only preservation, Ghidra provenance, links and stage 2R validation."""
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    report = json.loads((ROOT / 'docs/stage2r-validation.json').read_text())
    for group in ('protected_files', 'delivery_files'):
        for path, expected in report[group].items():
            assert sha(ROOT / path) == expected, f'{group}: changed {path}'
    records = {}
    entries = set()
    for name, count in [('rea.json.gz', 12), ('reused.json.gz', 5)]:
        bundle = json.load(gzip.open(HERE / name, 'rt'))
        assert len(bundle['records']) == count
        for r in bundle['records']:
            assert r['subject']['digest']['sha256'] == TARGET
            assert r['provider']['id'] == 'ghidra'
            assert r['analysis_profile']['provider']['id'] == 'ghidra'
            records[r['evidence_id']] = r
            if name == 'rea.json.gz':
                assert r['operation'] == 'analyze_function'
                n = r['normalized_result']
                assert n['procedure']['body']['available']
                assert n['pseudocode'] and n['assembly']
                entries.add(n['procedure']['name'])
        for source in bundle.get('sources', []):
            path = ROOT / source['path']
            assert sha(path) == source['sha256']
            original = {r['evidence_id']:r for r in json.load(gzip.open(path,'rt'))['records']}
            local = {r['evidence_id']:r for r in bundle['records']}
            for identity in source['evidence_ids']:
                assert local[identity] == original[identity]
    assert len(records) == 17
    assert set(report['rea']['primary_handlers']) <= entries
    index = (HERE / 'rea-functions.md').read_text()
    assert all(identity in index for identity in records)
    document = ROOT / 'docs/vm-control-flow-semantics.md'
    for identity in re.findall(r'ev_[0-9a-f]{64}', document.read_text()):
        assert identity in records, f'unknown evidence {identity}'
    for path in [document, ROOT / 'README.md', ROOT / 'docs/magic_Linux32-rust-rewrite-roadmap.md']:
        for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
            if '://' not in link and not link.startswith('#'):
                assert (path.parent / link.split('#')[0]).exists(), f'broken link {path}: {link}'
    summary = json.loads((HERE / 'corpus-summary.json').read_text())
    assert not summary['errors']
    for path, expected in summary['input_sha256'].items():
        assert sha(ROOT / path) == expected
    assert summary['native_layout_checks'] == dict(branch_targets_checked=83557,
        string_indexes_checked=20643, loop_slots_checked=62550)
    corpus = json.loads((HERE / 'corpus-validation.json').read_text())
    assert corpus['status'] == 'pass' and corpus['originals_unchanged'] == 36
    assert corpus['target_execution'] == 'none'
    assert corpus['known_errors'] == ['dba/pack_data/etc.pak/etc']
    import importlib.util
    spec = importlib.util.spec_from_file_location('current_audit', ROOT / 'docs/stage2q-evidence/verify_completion.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    _, current = module.build()
    assert (current['supported_opcode_kinds'], len(current['missing'])) == (59, 16)
    assert report['runtime_changed'] is False and report['target_execution'] == 'none'
    assert report['rea']['closed_after_export'] is True
    print(json.dumps(dict(status='pass', protected_files=len(report['protected_files']),
        delivery_files=len(report['delivery_files']), ghidra_records=17,
        supported_bounded=59, target_execution='none'),sort_keys=True))


if __name__ == '__main__':
    main()
