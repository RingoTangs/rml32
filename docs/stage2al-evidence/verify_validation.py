"""Read-only check of the 2AL receipt; this does not rerun its commands."""
import gzip
import json
from pathlib import Path
import re

from capture_native import HERE, ROOT, TARGET, digest, require
from verify_evidence import safe_source

COMMANDS = [
    ['python3', '-B', 'docs/stage2al-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2al-evidence/analyze_corpus.py'],
    ['python3', '-B', 'docs/stage2al-evidence/capture_native.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2al-evidence', '-p', 'test_*.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
]


def validate_commands(records):
    require([r['argv'] for r in records] == COMMANDS, 'command inventory/order')
    for record in records:
        require(record['returncode'] == 0, 'failed command receipt')
        text = record['stdout'] + record['stderr']
        if record['argv'][:2] == ['cargo', 'test']:
            matches = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', text)
            require(matches and sum(int(x) for x, _ in matches) == 487
                    and all(int(y) == 0 for _, y in matches), 'Rust test count/result')
        elif 'unittest' in record['argv']:
            require(re.search(r'Ran 28 tests\b', text) and text.rstrip().endswith('OK'), 'Python test count/result')


def verify(root=ROOT):
    root = Path(root)
    receipt = json.loads((root / 'docs/stage2al-validation.json').read_text())
    require(receipt['target_sha256'] == TARGET and receipt['stage'] == '2AL', 'receipt identity')
    logs = root / 'docs/stage2al-evidence/validation-logs.json.gz'
    require(digest(logs.read_bytes()) == receipt['logs_sha256'], 'validation log digest')
    records = json.loads(gzip.decompress(logs.read_bytes()))['commands']
    validate_commands(records)
    require(receipt['runtime_changed'] is False and receipt['supported_opcodes'] == 75
            and receipt['corpus_intersection'] == 73, 'capability boundary')
    require(receipt['commands'] == [dict(argv=r['argv'], returncode=r['returncode']) for r in records], 'command receipt mismatch')
    expected = {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md',
                'docs/vm-mapping-iteration-semantics.md'}
    expected.update(str(p.relative_to(root)) for p in (root / 'docs/stage2al-evidence').iterdir() if p.is_file())
    require(set(receipt['artifact_sha256']) == expected, 'artifact inventory')
    for name, expected_hash in receipt['artifact_sha256'].items():
        require(digest(safe_source(root, name).read_bytes()) == expected_hash, 'delivery file changed: ' + name)
    protected = json.loads((root / 'docs/stage2al-evidence/protected-inputs.json').read_text())
    for name, expected_hash in protected.items():
        require(digest(safe_source(root, name).read_bytes()) == expected_hash, 'protected input changed: ' + name)
    require(digest((root / 'magic_Linux32').read_bytes()) == TARGET, 'target changed')
    return dict(commands=len(records), artifact_files=len(expected), protected_files=len(protected),
                rust_tests_debug=487, rust_tests_release=487, python_tests=28)


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
