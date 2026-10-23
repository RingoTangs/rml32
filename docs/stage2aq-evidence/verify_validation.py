"""Run/freeze 2AQ checks with --write; otherwise verify receipt and immutable inputs."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

from audit_rules import local_file, verify_hashes
from verify_completion import build

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
ENTRY = {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}
REPORT = 'docs/stage2-completion-assessment-2aq.md'
COMMANDS = [
    ['python3', '-B', 'docs/stage2ap-evidence/verify_static_evidence.py'],
    ['python3', '-B', 'docs/stage2aq-evidence/verify_completion.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2aq-evidence', '-p', 'test_*.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
    ['cargo', 'build', '--locked', '--offline', '--bin', 'rml32-inspect'],
    ['python3', '-B', 'tests/compare_corpus.py', '--report', 'target/stage2aq-corpus-validation.json'],
]
PYTHON_TESTS = 55


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()


def delivery_files(root=ROOT):
    directory = root / 'docs/stage2aq-evidence'
    return {str(p.relative_to(root)) for p in directory.iterdir() if p.is_file()} | ENTRY | {REPORT}


def check_inventory(expected, actual):
    require(set(expected) == set(actual), 'delivery inventory mismatch')


def check_entries(baseline, previous):
    require(set(baseline['before_entry_sha256']) == ENTRY, 'entry inventory mismatch')
    require(all(previous['artifact_sha256'].get(p) == value
                for p, value in baseline['before_entry_sha256'].items()), 'entry baseline mismatch')


def check_baseline(root=ROOT):
    baseline = json.loads(local_file(root, 'docs/stage2aq-evidence/baseline-inputs.json').read_text())
    require(baseline['stage'] == '2AQ' and baseline['baseline_stage'] == '2AP', 'baseline stage mismatch')
    expected = dict(stage='2AP', commands=7, delivery_files=15, rust_tests_debug=558,
                    rust_tests_release=558, python_tests=27, protected_files=565,
                    allowed_existing_paths=7, static_records=20, global_creation_locations=5012,
                    supported_opcodes=75, corpus_intersection=73)
    require(baseline['baseline_verification'] == expected, 'baseline verification mismatch')
    protected = baseline['protected_files']
    require(len(protected) == 576, 'protected inventory size mismatch')
    prior = json.loads(local_file(root, 'docs/stage2ap-validation.json').read_text())
    check_entries(baseline, prior)
    required = {str(p.relative_to(root)) for d in ('src', 'tests', 'docs')
                for p in (root/d).rglob('*') if p.is_file() and '__pycache__' not in p.parts
                and not str(p.relative_to(root)).startswith('docs/stage2aq-evidence/')}
    required |= {'Cargo.toml', 'Cargo.lock', 'magic_Linux32', 'AGENTS.md'}
    manifest = json.loads(local_file(root, 'docs/compatibility-evidence/input-manifest.json').read_text())
    required |= {entry['path'] for entry in manifest}
    required -= ENTRY | {REPORT, 'docs/stage2aq-validation.json'}
    require(set(protected) == required, 'protected inventory mismatch')
    require(all(protected.get(p) == value for p, value in prior['artifact_sha256'].items()
                if p not in ENTRY), '2AP delivery protection mismatch')
    verify_hashes(root, protected)
    for path, old in baseline['before_entry_sha256'].items():
        current = local_file(root, path).read_text()
        lines = current.splitlines(keepends=True)
        require(len(lines) >= 3, 'entry content missing')
        # Exactly one new opening paragraph after the original title/blank line.
        first_blank = 1
        require(lines[first_blank].strip() == '', 'entry prefix changed')
        require('2AQ' in lines[2] and lines[3].strip() == '', 'entry progress prefix missing')
        original = ''.join(lines[:2] + lines[4:])
        require(digest(original.encode()) == old, 'historical entry content changed')
    return baseline


def passed_tests(output):
    return set(re.findall(r'^test (\S+) \.\.\. ok$', output, re.M))


def validate_commands(commands, original_tests):
    require([c['argv'] for c in commands] == COMMANDS, 'command inventory mismatch')
    require(len(original_tests) == 558, 'original test inventory mismatch')
    for command in commands:
        require(command['returncode'] == 0, 'failed check')
        output = command['stdout'] + command['stderr']
        if command['argv'][:2] == ['cargo', 'test']:
            counts = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', output)
            require(counts and sum(int(n) for n, _ in counts) == 558
                    and all(int(n) == 0 for _, n in counts), 'Rust result count mismatch')
            require(passed_tests(output) == original_tests, 'Rust test inventory mismatch')
        if 'unittest' in command['argv']:
            require(re.search(rf'Ran {PYTHON_TESTS} tests\b', output)
                    and output.rstrip().endswith('OK'), 'Python results mismatch')


def original_tests(root=ROOT):
    logs = json.loads(gzip.decompress(local_file(root, 'docs/stage2ap-evidence/validation-logs.json.gz').read_bytes()))
    previous = next(c for c in logs['commands'] if c['argv'] == ['cargo', 'test', '--locked', '--offline'])
    return passed_tests(previous['stdout'] + previous['stderr'])


def check_corpus(corpus):
    require((corpus['status'], corpus['originals_unchanged'], corpus['known_errors'],
             corpus['checked'], corpus['target_execution']) ==
            ('pass', 36, ['dba/pack_data/etc.pak/etc'],
             dict(packages=12, members=9527, programs=7159, inheritance=9933), 'none'), 'corpus boundary mismatch')


def verify(root=ROOT):
    baseline = check_baseline(root)
    record = json.loads(local_file(root, 'docs/stage2aq-validation.json').read_text())
    expected = dict(stage='2AQ', runtime_changed=False, public_api_changed=False,
                    dependencies_changed=False, rust_tests_debug=558, rust_tests_release=558,
                    python_tests=PYTHON_TESTS, supported_opcodes=75, corpus_intersection=73,
                    gaps=24, capability_gates=13, stage2_complete=False,
                    target_execution='none', new_native_analysis=False, selected_next='P05')
    require(all(record.get(k) == v for k, v in expected.items()), 'receipt boundary mismatch')
    check_inventory(record['artifact_sha256'], delivery_files(root))
    verify_hashes(root, record['artifact_sha256'])
    logs = local_file(root, 'docs/stage2aq-evidence/validation-logs.json.gz').read_bytes()
    require(digest(logs) == record['logs_sha256'], 'validation logs changed')
    commands = json.loads(gzip.decompress(logs))['commands']
    validate_commands(commands, original_tests(root))
    corpus = json.loads(local_file(root, 'docs/stage2aq-evidence/corpus-validation.json').read_text())
    check_corpus(corpus)
    require(json.loads(commands[-1]['stdout']) == corpus, 'recorded corpus mismatch')
    outputs, summary = build()
    require(json.loads(commands[1]['stdout']) == {k: v for k, v in summary.items() if k != 'source_sha256'},
            'recorded audit mismatch')
    for name, output in outputs.items():
        require(local_file(root, 'docs/stage2aq-evidence/' + name).read_text() == output,
                'stale generated output: ' + name)
    return dict(status='pass', stage='2AQ', protected_files=len(baseline['protected_files']),
                delivery_files=len(delivery_files(root)), rust_tests_debug=558, rust_tests_release=558,
                python_tests=PYTHON_TESTS, gaps=24, capability_gates=13, selected_next='P05',
                stage2_complete=False, target_execution='none')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        check_baseline()
        commands = []
        for argv in COMMANDS:
            print('Checking: ' + ' '.join(argv), flush=True)
            result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
            commands.append(dict(argv=argv, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr))
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)
        validate_commands(commands, original_tests())
        corpus = json.loads((ROOT / 'target/stage2aq-corpus-validation.json').read_text())
        check_corpus(corpus)
        check_baseline()
        (HERE / 'corpus-validation.json').write_text(json.dumps(corpus, ensure_ascii=False, sort_keys=True, indent=2)+'\n')
        logs = gzip.compress(canonical(dict(commands=commands)), mtime=0)
        (HERE / 'validation-logs.json.gz').write_bytes(logs)
        record = dict(stage='2AQ', runtime_changed=False, public_api_changed=False,
                      dependencies_changed=False, rust_tests_debug=558, rust_tests_release=558,
                      python_tests=PYTHON_TESTS, supported_opcodes=75, corpus_intersection=73,
                      gaps=24, capability_gates=13, stage2_complete=False, selected_next='P05',
                      target_execution='none', new_native_analysis=False, logs_sha256=digest(logs),
                      artifact_sha256={p: digest((ROOT/p).read_bytes()) for p in sorted(delivery_files())})
        (ROOT / 'docs/stage2aq-validation.json').write_text(json.dumps(record, ensure_ascii=False, sort_keys=True, indent=2)+'\n')
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
