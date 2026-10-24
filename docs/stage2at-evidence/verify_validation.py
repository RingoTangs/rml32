"""Freeze new 2AT receipts; never rewrite prior stages or execute the reference target."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RECORD = ROOT / 'docs/stage2at-validation.json'
CHANGED = {f'src/{n}.rs' for n in ('cell', 'container', 'host', 'lib', 'object', 'value', 'vm')} | {'tests/vm_core.rs'}
NEW = {'src/boundary.rs', 'src/ownership.rs', 'src/regression.rs', 'tests/api_compile.rs', 'tests/vm_boundary.rs', 'tests/boundary_support.rs.inc'}
ENTRIES = {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}
COMMANDS = [
    ['python3', '-B', 'docs/stage2ar-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2as-design/explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_model.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_explore.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
    ['cargo', 'build', '--locked', '--offline'],
    ['python3', '-B', 'tests/compare_corpus.py', '--report', 'docs/stage2at-evidence/corpus-validation.json'],
]

def require(value, message):
    if not value:
        raise AssertionError(message)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def load(path):
    raw = path.read_bytes()
    return json.loads(gzip.decompress(raw) if path.suffix == '.gz' else raw)


def baseline():
    data = load(HERE / 'baseline-inputs.json')
    require(data['stage'] == '2AT' and len(data['files']) == 653, 'baseline inventory differs')
    require(data['baseline_verification']['stage'] == '2AS', 'baseline stage differs')
    require(data['baseline_verification']['rust_tests_debug'] == 558, 'original count differs')
    require(set(data['entry_before']) == ENTRIES, 'entry inventory differs')
    require(CHANGED <= data['files'].keys(), 'missing old source hashes')
    for name, expected in data['files'].items():
        if name not in CHANGED:
            require(digest((ROOT / name).read_bytes()) == expected, 'frozen input changed: ' + name)
    require(not (ROOT / 'tests/vm_core.rs').exists(), 'raw integration API restored')
    old = gzip.decompress((HERE / 'regression-baseline.rs.gz').read_bytes())
    require(digest(old) == data['files']['tests/vm_core.rs'], 'regression baseline differs')
    migrated = (ROOT / 'src/regression.rs').read_text().replace('use crate::regression_api as rml32;\n', '').replace('crate::host::owned_callback(HostRc::new(callback))', 'HostRc::new(callback)')
    require(migrated.encode() == old, 'original scenario assertions changed')
    lib = (ROOT / 'src/lib.rs').read_text()
    for module in ('container', 'host', 'object', 'value', 'vm'):
        require(f'\nmod {module};' in lib and f'pub mod {module};' not in lib, 'raw module exposed: ' + module)
    require('pub(crate) use value::{FunctionValue, Value};' in lib, 'raw executable export differs')
    require('#[cfg(test)]\nmod regression_api' in lib, 'legacy namespace not test-only')
    return data


def migrations():
    data = load(HERE / 'api-migration.json')
    require(data['public_api_changed'] and not data['source_compatible'], 'migration metadata differs')
    require([x['id'] for x in data['items']] == [f'API{i:02}' for i in range(1, 17)], '16 API boundaries required')
    tests = (ROOT / 'tests/vm_boundary.rs').read_text()
    for item in data['items']:
        source = item['source']
        lines = (ROOT / source['path']).read_text().splitlines()
        require(lines[source['line'] - 1].strip() == source['excerpt'], 'stale source anchor: ' + item['id'])
        require(f"fn {item['public_integration_test']}(" in tests, 'missing public scenario: ' + item['id'])
    return data


def names(output):
    return set(re.findall(r'^test (\S+) \.\.\. ok$', output, re.M))


def original_tests():
    prior = load(ROOT / 'docs/stage2as-design/validation-logs.json.gz')
    command = next(c for c in prior['commands'] if c['argv'] == ['cargo', 'test', '--locked', '--offline'])
    result = names(command['stdout'] + command['stderr'])
    require(len(result) == 558, 'old runtime test inventory differs')
    return result


def validate_logs(commands):
    require([c['argv'] for c in commands] == COMMANDS, 'command inventory differs')
    old = original_tests()
    for command in commands:
        require(command['returncode'] == 0, 'command failed: ' + str(command['argv']))
        output = command['stdout'] + command['stderr']
        argv = command['argv']
        if argv[:2] == ['cargo', 'test']:
            passed = names(output)
            require(len(passed) == 577, 'expected 577 ordinary Rust tests')
            normalized = {n.removeprefix('regression::') for n in passed}
            require(old <= normalized, 'lost original scenarios: ' + str(sorted(old - normalized)))
            results = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', output)
            require(sum(int(n) for n, _ in results) == 579 and all(int(n) == 0 for _, n in results), 'Rust/doc counts differ')
            require('public_api_positive_and_negative_contracts_compile_against_current_library' in passed, 'compile contract missing')
        if 'unittest' in argv:
            expected = 50 if argv[-1] == 'test_model.py' else 6
            require(re.search(rf'Ran {expected} tests\b', output) and output.rstrip().endswith('OK'), 'independent model tests differ')
    corpus = load(HERE / 'corpus-validation.json')
    require(corpus['status'] == 'pass' and corpus['target_execution'] == 'none', 'corpus validation differs')
    require(corpus['summary']['programs'] == 7159 and corpus['summary']['instructions'] == 6474773, 'corpus size differs')
    # The report is generated by the captured corpus command, not a separately edited claim.
    require(json.loads(commands[-1]['stdout']) == corpus, 'corpus report/log mismatch')


def delivery():
    paths = (CHANGED - {'tests/vm_core.rs'}) | NEW | ENTRIES
    paths |= {str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file()}
    paths.discard('docs/stage2at-evidence/validation-logs.json.gz')
    return {p: digest((ROOT / p).read_bytes()) for p in sorted(paths)}


def verify():
    data = baseline()
    migrations()
    record = load(RECORD)
    require(record['stage'] == '2AT' and record['runtime_changed'] and record['public_api_changed'], 'stage flags differ')
    require(not record['non_scalar_ready'] and not record['stage2_complete'] and not record['dependencies_changed'], 'scope flags differ')
    require(record['target_execution'] == 'none' and not record['new_native_analysis'], 'evidence scope differs')
    require(record['rust_tests_debug'] == record['rust_tests_release'] == 577 and record['doctests_each_profile'] == 2, 'test record differs')
    require(record['python_tests'] == 56 and record['selected_next'] == 'T02-stable-captures-copy-plan', 'next step differs')
    require(record['artifact_sha256'] == delivery(), 'delivery hashes differ')
    raw = (HERE / 'validation-logs.json.gz').read_bytes()
    require(digest(raw) == record['logs_sha256'], 'validation logs changed')
    validate_logs(load(HERE / 'validation-logs.json.gz')['commands'])
    require(record['original_test_names'] == sorted(original_tests()), 'original test ledger differs')
    return dict(status='pass', stage='2AT', protected_previous_files=len(data['files']) - len(CHANGED),
                api_boundaries=16, original_scenarios_preserved=558, rust_tests_each_profile=577,
                doctests_each_profile=2, python_tests=56, target_execution='none', non_scalar_ready=False,
                selected_next=record['selected_next'])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        baseline()
        migrations()
        commands = []
        for argv in COMMANDS:
            print('Checking: ' + ' '.join(argv), flush=True)
            result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
            command = dict(argv=argv, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
            commands.append(command)
            require(result.returncode == 0, result.stdout + result.stderr)
        validate_logs(commands)
        baseline()
        raw = gzip.compress(json.dumps(dict(commands=commands), ensure_ascii=False, sort_keys=True).encode(), mtime=0)
        (HERE / 'validation-logs.json.gz').write_bytes(raw)
        record = dict(stage='2AT', runtime_changed=True, public_api_changed=True, source_compatible=False,
                      dependencies_changed=False, new_native_analysis=False, target_execution='none',
                      non_scalar_ready=False, stage2_complete=False, selected_next='T02-stable-captures-copy-plan',
                      rust_tests_debug=577, rust_tests_release=577, doctests_each_profile=2, python_tests=56,
                      original_test_names=sorted(original_tests()), logs_sha256=digest(raw), artifact_sha256=delivery())
        RECORD.write_text(json.dumps(record, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
