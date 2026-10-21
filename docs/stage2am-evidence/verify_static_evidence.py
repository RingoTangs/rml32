"""Read-only 2AM evidence check; 2AL runtime hashes remain a historical snapshot."""
import gzip
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'docs/stage2al-evidence'))
from capture_native import canonical, digest, require
from mapping_model import Mapping
from verify_evidence import safe_source, validate_native

BASELINE_SHA = '180405090b675fe4ed071c97bd9810e584afb5fdb5803d2c4bbedd764288d437'
CHANGED = {'src/container.rs', 'src/vm.rs', 'tests/vm_core.rs'}
ENTRYPOINTS = {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}


def verify(root=ROOT):
    root = Path(root)
    raw = (root / 'docs/stage2al-validation.json').read_bytes()
    require(digest(raw) == BASELINE_SHA, '2AL historical receipt changed')
    receipt = json.loads(raw)
    historical = 0
    for name, expected in receipt['artifact_sha256'].items():
        if name in ENTRYPOINTS:
            continue
        require(digest(safe_source(root, name).read_bytes()) == expected, '2AL artifact changed: ' + name)
        historical += 1
    protected = json.loads((root / 'docs/stage2al-evidence/protected-inputs.json').read_text())
    for name, expected in protected.items():
        if name not in CHANGED:
            require(digest(safe_source(root, name).read_bytes()) == expected, 'protected file changed: ' + name)
    bundle_raw = (root / 'docs/stage2al-evidence/native.json.gz').read_bytes()
    bundle = json.loads(gzip.decompress(bundle_raw))
    instructions = validate_native(bundle, (root / 'magic_Linux32').read_bytes())
    # Preserve original source-to-record identity for reused REA Evidence.
    reused = json.loads(gzip.decompress((root / 'docs/stage2al-evidence/reused.json.gz').read_bytes()))['records']
    byid = {r['evidence_id']: r for r in reused}
    for entry in json.loads((root / 'docs/stage2al-evidence/reused-manifest.json').read_text()):
        source = safe_source(root, entry['source']).read_bytes()
        require(digest(source) == entry['source_sha256'], 'reused source changed')
        record = byid[entry['evidence_id']]
        require(record in json.loads(gzip.decompress(source))['records']
                and digest(canonical(record)) == entry['record_sha256'], 'reused record changed')
    fixture = json.loads((root / 'tests/fixtures/mapping-order-2al.json').read_text())
    require(fixture['model_sha256'] == digest((root / 'docs/stage2al-evidence/mapping_model.py').read_bytes()), 'fixture model source')
    require(fixture['native_bundle_sha256'] == digest(bundle_raw)
            and fixture['target_sha256'] == bundle['target_sha256'], 'fixture target/source')
    table = bytes.fromhex(bundle['hash_table']['bytes_hex'])
    for case in fixture['cases']:
        mapping = Mapping(table, case['hint'])
        for row in case['keys']:
            key = row['int'] if 'int' in row else bytes.fromhex(row['hex'])
            mapping.insert(key, None)
        order = [dict(int=k) if type(k) is int else dict(hex=k.hex()) for k in mapping.order()]
        require(order == case['order'] and len(mapping.buckets) == case['capacity']
                and mapping.unfilled == case['unfilled'], 'static fixture result')
    return dict(historical_artifacts=historical, unchanged_protected_files=len(protected)-len(CHANGED),
                native_instruction_locations=instructions, reused_records=len(reused), fixture_cases=len(fixture['cases']))


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
