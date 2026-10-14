#!/usr/bin/env python3
"""Verify retained stage 2P evidence and protected-file hashes; never run targets."""
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / 'docs/stage2-completion-evidence'
SHA = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    report = json.loads((ROOT / 'docs/stage2p-validation.json').read_text())
    for field in ('protected_files', 'delivery_files'):
        for name, expected in report[field].items():
            assert digest(ROOT / name) == expected, f'{field}: changed {name}'
    all_records = {}
    for name in ('rea.json.gz', 'reused.json.gz'):
        bundle = json.load(gzip.open(EVIDENCE / name, 'rt'))
        records = bundle['records']
        assert len(records) == report['rea']['bundle_records'][name]
        for record in records:
            assert record['subject']['digest']['sha256'] == SHA
            assert record['provider']['id'] == 'ghidra'
            assert record['analysis_profile']['provider']['id'] == 'ghidra'
            all_records[record['evidence_id']] = record
        for source in bundle.get('sources', []):
            path = ROOT / source['path']
            assert digest(path) == source['sha256']
            original = {r['evidence_id']: r for r in json.load(gzip.open(path, 'rt'))['records']}
            retained = {r['evidence_id']: r for r in records}
            for identity in source['evidence_ids']:
                assert retained[identity] == original[identity], identity
    index = (EVIDENCE / 'rea-functions.md').read_text()
    for identity in all_records:
        assert identity in index, f'unindexed evidence {identity}'
    import re
    design = (EVIDENCE / 'cross-object-design.md').read_text()
    for identity in re.findall(r'ev_[0-9a-f]{64}', design):
        assert identity in all_records, f'unknown evidence {identity}'
    for path in [ROOT / 'docs/stage2-completion-assessment.md', EVIDENCE / 'cross-object-design.md', EVIDENCE / 'matrix.md']:
        for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
            if '://' not in link and not link.startswith('#'):
                assert (path.parent / link.split('#')[0]).exists(), f'broken link {path}: {link}'
    corpus = json.loads((EVIDENCE / 'corpus-validation.json').read_text())
    assert corpus['status'] == 'pass' and corpus['originals_unchanged'] == 36
    assert corpus['known_errors'] == ['dba/pack_data/etc.pak/etc']
    assert corpus['target_execution'] == 'none'
    assert report['runtime_changed'] is False
    assert report['rea']['closed_after_export'] is True
    print(json.dumps({'status': 'pass', 'protected_files': len(report['protected_files']),
                      'delivery_files': len(report['delivery_files']),
                      'rea_records': len(all_records), 'target_execution': 'none'}, sort_keys=True))


if __name__ == '__main__':
    main()
