"""Read-only source identity, retained-record and citation checks for stage 2M."""
from pathlib import Path
import gzip
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def verify():
    current = json.loads(gzip.decompress((HERE / 'rea.json.gz').read_bytes()))
    reused = json.loads(gzip.decompress((HERE / 'reused.json.gz').read_bytes()))
    ids = set()
    for r in current['records'] + reused['records']:
        assert r['subject']['digest']['sha256'] == TARGET, r['evidence_id']
        assert r['provider']['id'] == 'ghidra', r['evidence_id']
        assert r['evidence_id'] not in ids, r['evidence_id']
        ids.add(r['evidence_id'])
    for path, sha in reused['source_files_sha256'].items():
        data = (ROOT / path).read_bytes()
        assert hashlib.sha256(data).hexdigest() == sha, path
        originals = {r['evidence_id']: r for r in json.loads(gzip.decompress(data))['records']}
        for r in reused['records']:
            if reused['record_sources'][r['evidence_id']] == path:
                assert originals[r['evidence_id']] == r, r['evidence_id']
    report = ROOT / 'docs/vm-object-lifecycle.md'
    pages = [report, HERE / 'deletion-findings.md', HERE / 'index.md', HERE / 'functions.md']
    references = set()
    for page in pages:
        content = page.read_text()
        for eid in re.findall(r'ev_[a-f0-9]{64}', content):
            assert eid in ids, (page, eid)
            references.add(eid)
    raw = ROOT / 'target/vm-object-lifecycle-rea.json'
    if raw.exists():
        assert raw.read_bytes() == gzip.decompress((HERE / 'rea.json.gz').read_bytes())
    return dict(new_records=len(current['records']), reused_records=len(reused['records']),
                referenced_evidence_ids=len(references), target_sha256=TARGET,
                source_hashes_verified=len(reused['source_files_sha256']))


if __name__ == '__main__':
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))
