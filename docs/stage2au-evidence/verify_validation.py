"""Freeze/verify 2AU without changing historical receipts or executing the target."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RECORD = ROOT / 'docs/stage2au-validation.json'
CHANGED = {f'src/{n}.rs' for n in ('boundary', 'cell', 'container', 'lib', 'ownership', 'regression', 'value', 'vm')} | {'tests/vm_boundary.rs', 'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW = {'src/copy_plan.rs'}
RENAME = {'regression::member_copy_normalizes_destroyed_object_without_mutating_container': 'regression::member_copy_normalizes_destroyed_object_in_source_cell'}
COMMANDS = [
    ['python3', '-B', 'docs/stage2ar-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2as-design/explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_model.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2au-evidence', '-p', 'test_verify_validation.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
    ['cargo', 'build', '--locked', '--offline'],
    ['python3', '-B', 'tests/compare_corpus.py', '--report', 'docs/stage2au-evidence/corpus-validation.json'],
]


def require(value, message):
    if not value:
        raise AssertionError(message)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def load(path):
    raw = path.read_bytes()
    return json.loads(gzip.decompress(raw) if path.suffix == '.gz' else raw)


def old_sources():
    return load(HERE / 'baseline-rust-sources.json.gz')


def check_regression(before, after):
    after = after.replace('fn member_copy_normalizes_destroyed_object_in_source_cell()', 'fn member_copy_normalizes_destroyed_object_without_mutating_container()')
    after = after.replace('    assert_eq!(a.elements().unwrap(), &[Value::Undefined]);\n}\n\n#[test]\nfn reference_assignments', '    assert_eq!(a.elements().unwrap(), &[Value::Object(dead)]);\n}\n\n#[test]\nfn reference_assignments')
    after = after.replace('    // 2AR F16: nonconst mixed/already-matching return transfers the raw result.\n    returns(&mut vm, Value::Function(f.clone()));', '    returns(&mut vm, Value::Undefined); // Ordinary return is a later VM copy boundary.')
    require(after == before, 'unregistered original regression assertion changed')


def baseline():
    data = load(HERE / 'baseline-inputs.json')
    require(data['stage'] == '2AU' and len(data['files']) == 671, 'baseline inventory differs')
    previous = data['baseline_verification']
    require(previous['stage'] == '2AT' and previous['status'] == 'pass' and previous['rust_tests_each_profile'] == 577, 'baseline verification differs')
    require(CHANGED <= data['files'].keys(), 'old source inventory differs')
    for path, expected in data['files'].items():
        if path not in CHANGED:
            require(digest((ROOT / path).read_bytes()) == expected, 'frozen input changed: ' + path)
    sources = old_sources()
    for path, text in sources.items():
        require(digest(text.encode()) == data['files'][path], 'frozen source snapshot differs: ' + path)
    check_regression(sources['src/regression.rs'], (ROOT / 'src/regression.rs').read_text())
    lib = (ROOT / 'src/lib.rs').read_text()
    require('\nmod copy_plan;' in lib and 'pub mod copy_plan' not in lib, 'private plan module differs')
    for module in ('container', 'host', 'object', 'value', 'vm'):
        require(f'\nmod {module};' in lib and f'pub mod {module};' not in lib, 'raw runtime module exposed: ' + module)
    return data


def contacts(data=None):
    data = load(HERE / 'copy-contacts.json') if data is None else data
    original = old_sources()['src/vm.rs']
    runtime = original[:original.index('#[cfg(test)]\nmod tests')]
    expected = {(i, line.strip()) for i, line in enumerate(runtime.splitlines(), 1) if '.copy_value(' in line}
    require(data['stage'] == '2AU' and data['original_copy_sites'] == len(expected) == 20, 'original COPY inventory differs')
    rows = data['items']
    require([r['id'] for r in rows] == [f'COPY{i:02}' for i in range(1, 21)] + [f'RAW{i:02}' for i in range(1, 5)], 'contact identity inventory differs')
    actual = {(r['before']['line'], r['before']['excerpt']) for r in rows if r['before']}
    require(actual == expected, 'missing or duplicated original COPY contact')
    tests = '\n'.join(p.read_text() for parent in ('src', 'tests') for p in (ROOT / parent).glob('*.rs'))
    for row in rows:
        src = row['after']
        require(src['path'] == 'src/vm.rs', 'unexpected contact source')
        lines = (ROOT / src['path']).read_text().splitlines()
        require(lines[src['line'] - 1].strip() == src['excerpt'], 'stale source anchor: ' + row['id'])
        require(f"fn {row['test']}(" in tests, 'missing scenario: ' + row['id'])
    return data


def names(output):
    return set(re.findall(r'^test (\S+) \.\.\. ok$', output, re.M))


def old_names():
    prior = load(ROOT / 'docs/stage2at-evidence/validation-logs.json.gz')
    command = next(c for c in prior['commands'] if c['argv'] == ['cargo', 'test', '--locked', '--offline'])
    tests = names(command['stdout'] + command['stderr'])
    require(len(tests) == 577, 'old ordinary test inventory differs')
    return tests


def source_test_count():
    return sum(len(re.findall(r'^\s*#\[test\]\s*\n\s*fn \w+\(', p.read_text(), re.M))
               for folder in ('src', 'tests') for p in (ROOT / folder).glob('*.rs'))


def validate_rust(output):
    passed = names(output)
    require(len(passed) == source_test_count(), 'ordinary Rust/source test count differs')
    old = {RENAME.get(n, n) for n in old_names()}
    require(old <= passed, 'lost baseline scenario: ' + str(sorted(old - passed)))
    results = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', output)
    require(sum(int(n) for n, _ in results) == len(passed) + 2 and all(int(n) == 0 for _, n in results), 'ordinary/doctest results differ')
    return passed


def validate_commands(commands):
    require([c['argv'] for c in commands] == COMMANDS, 'command inventory differs')
    rust = []
    for command in commands:
        require(command['returncode'] == 0, 'command failed: ' + str(command['argv']))
        argv = command['argv']
        output = command['stdout'] + command['stderr']
        if argv[:2] == ['cargo', 'test']:
            passed = validate_rust(output)
            for row in contacts()['items']:
                require(any(n == row['test'] or n.endswith('::' + row['test']) for n in passed), 'contact scenario not executed: ' + row['id'])
            rust.append(passed)
        if 'unittest' in argv:
            count = 50 if argv[-1] == 'test_model.py' else 6
            require(re.search(rf'Ran {count} tests\b', output) and output.rstrip().endswith('OK'), 'Python test results differ')
    require(len(rust) == 2 and rust[0] == rust[1], 'debug/release test inventory differs')
    corpus = load(HERE / 'corpus-validation.json')
    require(corpus['status'] == 'pass' and corpus['target_execution'] == 'none', 'corpus scope differs')
    require(corpus['summary']['programs'] == 7159 and corpus['summary']['instructions'] == 6474773, 'corpus inventory differs')
    require(json.loads(commands[-1]['stdout']) == corpus, 'corpus report differs from captured command')
    return rust[0]


def delivery():
    paths = CHANGED | NEW | {str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file()}
    paths.discard('docs/stage2au-evidence/validation-logs.json.gz')
    return {p: digest((ROOT / p).read_bytes()) for p in sorted(paths)}


def verify():
    base = baseline()
    contacts()
    record = load(RECORD)
    require(record['stage'] == '2AU' and record['runtime_changed'] and record['source_compatible'], 'stage metadata differs')
    require(not any(record[k] for k in ('public_api_changed', 'dependencies_changed', 'non_scalar_ready', 'new_native_analysis', 'stage2_complete')), 'scope flags differ')
    require(record['target_execution'] == 'none' and record['selected_next'] == 'T03-projected-function-graph', 'next batch differs')
    require(record['artifact_sha256'] == delivery(), 'delivery hashes differ')
    raw = (HERE / 'validation-logs.json.gz').read_bytes()
    require(digest(raw) == record['logs_sha256'], 'validation logs changed')
    passed = validate_commands(load(HERE / 'validation-logs.json.gz')['commands'])
    require(record['rust_tests_debug'] == record['rust_tests_release'] == len(passed) and record['doctests_each_profile'] == 2, 'Rust counts differ')
    require(record['model_tests'] == 56 and record['receipt_tests'] == 6, 'Python counts differ')
    require(record['baseline_test_names'] == sorted(old_names()) and record['current_test_names'] == sorted(passed), 'scenario ledger differs')
    return dict(status='pass', stage='2AU', protected_previous_files=len(base['files']) - len(CHANGED),
                original_copy_sites=20, contacts=24, preserved_scenarios=577, rust_tests_each_profile=len(passed),
                doctests_each_profile=2, model_tests=56, receipt_tests=6, target_execution='none', non_scalar_ready=False,
                selected_next=record['selected_next'])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        baseline()
        contacts()
        commands = []
        for argv in COMMANDS:
            print('Checking: ' + ' '.join(argv), flush=True)
            result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
            commands.append(dict(argv=argv, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr))
            require(result.returncode == 0, result.stdout + result.stderr)
        passed = validate_commands(commands)
        baseline()
        raw = gzip.compress(json.dumps(dict(commands=commands), ensure_ascii=False, sort_keys=True).encode(), mtime=0)
        (HERE / 'validation-logs.json.gz').write_bytes(raw)
        record = dict(stage='2AU', runtime_changed=True, source_compatible=True, public_api_changed=False,
                      dependencies_changed=False, non_scalar_ready=False, new_native_analysis=False, stage2_complete=False,
                      target_execution='none', selected_next='T03-projected-function-graph',
                      rust_tests_debug=len(passed), rust_tests_release=len(passed), doctests_each_profile=2,
                      model_tests=56, receipt_tests=6, baseline_test_names=sorted(old_names()), current_test_names=sorted(passed),
                      logs_sha256=digest(raw), artifact_sha256=delivery())
        RECORD.write_text(json.dumps(record, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
