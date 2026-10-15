#!/usr/bin/env python3
"""Read-only verification of the stage 2U evidence and frozen delivery."""
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
TARGET_SHA = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_records(records):
    indexed = {}
    for record in records:
        identity = record['evidence_id']
        assert re.fullmatch(r'ev_[0-9a-f]{64}', identity), 'invalid Evidence ID'
        assert identity not in indexed, 'duplicate Evidence ID'
        assert record['provider']['id'] == 'ghidra', 'wrong analysis engine'
        assert record['subject']['digest']['sha256'] == TARGET_SHA, 'wrong target'
        assert record['authority'] == 'shipped-artifact', 'wrong authority'
        indexed[identity] = record
    return indexed


def check_sources(bundle, root):
    indexed = check_records(bundle['records'])
    covered = set()
    for source in bundle['sources']:
        path = root / source['path']
        assert sha(path) == source['sha256'], 'changed evidence source'
        original = {r['evidence_id']: r for r in json.loads(gzip.decompress(path.read_bytes()))['records']}
        for identity in source['evidence_ids']:
            assert indexed[identity] == original[identity], 'reused record differs'
            covered.add(identity)
    assert covered == set(indexed), 'unattributed reused record'


def main():
    report = json.loads((ROOT / 'docs/stage2u-validation.json').read_text())
    for group in ('protected_files', 'delivery_files'):
        for path, expected in report[group].items():
            assert sha(ROOT / path) == expected, f'{group}: changed {path}'
    assert sha(ROOT / 'magic_Linux32') == TARGET_SHA
    fresh = json.loads(gzip.decompress((HERE / 'rea.json.gz').read_bytes()))
    reused = json.loads(gzip.decompress((HERE / 'reused.json.gz').read_bytes()))
    assert (len(fresh['records']), len(reused['records']), len(reused['sources'])) == (4, 18, 6)
    records = check_records(fresh['records'] + reused['records'])
    check_sources(reused, ROOT)
    manifest = json.loads((HERE / 'function-manifest.json').read_text())
    assert len(manifest) == len(records) == 22
    assert {x['evidence_id'] for x in manifest} == set(records)
    for item in manifest:
        record = records[item['evidence_id']]
        assert item['operation'] == record['operation']
        assert item['procedure'] == record['parameters']['procedure']
        bundle = fresh if item['bundle'] == 'rea.json.gz' else reused
        assert record in bundle['records']
    fresh_names = {(r['operation'], r['parameters']['procedure']) for r in fresh['records']}
    assert fresh_names == {('analyze_function', 'vm_newThreadCatch'),
                           ('analyze_function', 'vm_endThreadCatch'),
                           ('procedure_assembly', 'vm_caughtError'),
                           ('procedure_assembly', 'vm_doActions')}
    for record in fresh['records']:
        assert record['provider']['version'] == '12.1.4'
        if record['operation'] == 'analyze_function':
            native = record['normalized_result']
            assert native['procedure']['body']['available']
            assert native['assembly'] and native['pseudocode']
            assert native['incoming_references']
    documents = [ROOT / 'README.md', ROOT / 'docs/magic_Linux32-rust-rewrite-roadmap.md',
                 ROOT / 'docs/vm-catch-semantics.md', *HERE.glob('*.md')]
    for document in documents:
        content = document.read_text()
        if document.parent == HERE or document.name == 'vm-catch-semantics.md':
            for identity in re.findall(r'ev_[0-9a-f]{64}', content):
                assert identity in records, f'unknown evidence in {document.name}'
        for link in re.findall(r'\]\(([^)]+)\)', content):
            if '://' not in link and not link.startswith('#'):
                assert (document.parent / link.split('#')[0]).exists(), f'broken link: {link}'
    subprocess.run([sys.executable, '-B', str(HERE / 'analyze_corpus.py')], cwd=ROOT, check=True,
                   stdout=subprocess.DEVNULL)
    corpus = json.loads((HERE / 'corpus-summary.json').read_text())
    assert [corpus['opcodes'][op]['count'] for op in ('0x0e', '0x1a', '0x2c')] == [3266]*3
    assert corpus['candidate_target_opcodes'] == {'0x0e': 3266} and not corpus['errors']
    for path, expected in corpus['input_sha256'].items():
        assert sha(ROOT / path) == expected
    baseline = json.loads((ROOT / 'docs/stage2t-evidence/summary.json').read_text())
    assert (baseline['supported_opcode_kinds'], baseline['intersection_opcode_kinds'],
            len(baseline['missing'])) == (65, 63, 10)
    for path, expected in baseline['source_sha256'].items():
        assert sha(ROOT / path) == expected, f'changed baseline input: {path}'
    assert not report['runtime_changed'] and not report['dependencies_changed']
    assert report['new_rea_queries'] == 4 and report['new_rust_tests'] == 0
    assert report['target_execution'] == 'none'
    assert report['checks']['cargo test --locked']['total'] == 278
    print(json.dumps(dict(status='pass', new_ghidra_records=4, reused_records=18,
        protected_files=len(report['protected_files']), delivery_files=len(report['delivery_files']),
        supported_bounded=65, target_execution='none'), sort_keys=True))


if __name__ == '__main__':
    main()
