"""Freeze/verify 2AV without changing historical receipts or executing the target."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RECORD = ROOT / 'docs/stage2av-validation.json'
CHANGED = {f'src/{n}.rs' for n in ('boundary', 'cell', 'container', 'copy_plan', 'host', 'lib', 'object', 'vm')} | {'tests/vm_boundary.rs', 'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW = {'src/graph.rs'}
RENAME = {}
COMMANDS = [
    ['python3', '-B', 'docs/stage2ar-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2as-design/explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_model.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2av-evidence', '-p', 'test_verify_validation.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
    ['cargo', 'build', '--locked', '--offline'],
    ['python3', '-B', 'tests/compare_corpus.py', '--report', 'docs/stage2av-evidence/corpus-validation.json'],
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


def test_bodies(source):
    pattern = r'^([ \t]*)#\[test\]\s*\n\1fn (\w+)\([^)]*\)(?:\s*->[^{]+)?\s*\{(.*?)^\1\}'
    return {m[1]: m[2] for m in re.findall(pattern, source, re.M | re.S)}


def assertions(body):
    # Mask Rust comments/literals while balancing delimiters, retaining literal text in receipts.
    pattern = r'(?s)//[^\n]*|/\*.*?\*/|r(\#+)?".*?"\1|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\''
    masked = re.sub(pattern, lambda m: ' ' * len(m[0]), body)
    result = []
    for m in re.finditer(r'\bassert(?:_eq|_ne)?!\s*\(', masked):
        end = m.end(); depth = 1
        while depth and end < len(masked):
            depth += (masked[end] == '(') - (masked[end] == ')')
            end += 1
        require(depth == 0, 'unbalanced assertion')
        result.append(body[m.start():end].strip())
    return result


def check_regression(before, after):
    original, current = test_bodies(before), test_bodies(after)
    pattern = r'^([ \t]*)#\[test\]\s*\n\1fn (\w+)\([^)]*\)(?:\s*->[^{]+)?\s*\{(.*?)^\1\}'
    helpers_before = re.sub(pattern, '', before, flags=re.M | re.S)
    helpers_after = re.sub(pattern, '', after, flags=re.M | re.S)
    require(assertions(helpers_before) == assertions(helpers_after), 'unregistered helper assertion changed')
    for name, body in original.items():
        require(name in current, 'lost original test: ' + name)
        require(assertions(body) == assertions(current[name]), 'unregistered original assertion changed: ' + name)


def baseline():
    data = load(HERE / 'baseline-inputs.json')
    require(data['stage'] == '2AV' and len(data['files']) == 682, 'baseline inventory differs')
    previous = data['baseline_verification']
    require(previous['stage'] == '2AU' and previous['status'] == 'pass' and previous['rust_tests_each_profile'] == 606, 'baseline verification differs')
    require(CHANGED <= data['files'].keys(), 'old source inventory differs')
    for path, expected in data['files'].items():
        if path not in CHANGED:
            require(digest((ROOT / path).read_bytes()) == expected, 'frozen input changed: ' + path)
    sources = old_sources()
    for path, text in sources.items():
        require(digest(text.encode()) == data['files'][path], 'frozen source snapshot differs: ' + path)
    require(sum(len(test_bodies(text)) for text in sources.values()) == 606, 'baseline source scenario count differs')
    for path, text in sources.items():
        check_regression(text, (ROOT / path).read_text())
    lib = (ROOT / 'src/lib.rs').read_text()
    require('\nmod graph;' in lib and 'pub mod graph' not in lib, 'private plan module differs')
    for module in ('container', 'host', 'object', 'value', 'vm'):
        require(f'\nmod {module};' in lib and f'pub mod {module};' not in lib, 'raw runtime module exposed: ' + module)
    return data


def inventory():
    rows = []
    for name in ('cell', 'container', 'copy_plan', 'vm', 'object', 'boundary', 'host'):
        path = 'src/' + name + '.rs'
        runtime = (ROOT / path).read_text().split('#[cfg(test)]\nmod ')[0]
        for i, line in enumerate(runtime.splitlines(), 1):
            if re.search(r'\bCell::(?:new|replace_payload_batch(?:_with_roots)?|transfer_batch)\b|\.replace\(|\.member\(|std::mem::replace\(', line) and 'std::cell::Cell' not in line:
                rows.append(dict(path=path, line=i, excerpt=line.strip()))
    return rows


def contacts(data=None):
    data = load(HERE / 'publication-contacts.json') if data is None else data
    require(data['stage'] == '2AV', 'contact stage differs')
    require([r['after'] for r in data['items']] == inventory(), 'missing publication contact or stale source anchor')
    tests = '\n'.join(p.read_text() for parent in ('src', 'tests') for p in (ROOT / parent).glob('*.rs'))
    for row in data['items']:
        require(row['classification'] and row['review'], 'unreviewed contact')
        require(f"fn {row['test']}(" in tests, 'missing scenario')
    return data


def names(output):
    return set(re.findall(r'^test (\S+) \.\.\. ok$', output, re.M))


def old_names():
    prior = load(ROOT / 'docs/stage2au-evidence/validation-logs.json.gz')
    command = next(c for c in prior['commands'] if c['argv'] == ['cargo', 'test', '--locked', '--offline'])
    tests = names(command['stdout'] + command['stderr'])
    require(len(tests) == 606, 'old ordinary test inventory differs')
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
    paths.discard('docs/stage2av-evidence/validation-logs.json.gz')
    return {p: digest((ROOT / p).read_bytes()) for p in sorted(paths)}


def verify():
    base = baseline()
    contacts()
    record = load(RECORD)
    require(record['stage'] == '2AV' and record['runtime_changed'] and record['source_compatible'], 'stage metadata differs')
    require(not any(record[k] for k in ('public_api_changed', 'dependencies_changed', 'non_scalar_ready', 'new_native_analysis', 'stage2_complete')), 'scope flags differ')
    require(record['target_execution'] == 'none' and record['selected_next'] == 'T04-explicit-container-captures', 'next batch differs')
    require(record['artifact_sha256'] == delivery(), 'delivery hashes differ')
    raw = (HERE / 'validation-logs.json.gz').read_bytes()
    require(digest(raw) == record['logs_sha256'], 'validation logs changed')
    passed = validate_commands(load(HERE / 'validation-logs.json.gz')['commands'])
    require(record['rust_tests_debug'] == record['rust_tests_release'] == len(passed) and record['doctests_each_profile'] == 2, 'Rust counts differ')
    require(record['model_tests'] == 56 and record['receipt_tests'] == 6, 'Python counts differ')
    require(record['baseline_test_names'] == sorted(old_names()) and record['current_test_names'] == sorted(passed), 'scenario ledger differs')
    return dict(status='pass', stage='2AV', protected_previous_files=len(base['files']) - len(CHANGED),
                publication_contacts=len(contacts()['items']), preserved_scenarios=606, rust_tests_each_profile=len(passed),
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
        record = dict(stage='2AV', runtime_changed=True, source_compatible=True, public_api_changed=False,
                      dependencies_changed=False, non_scalar_ready=False, new_native_analysis=False, stage2_complete=False,
                      target_execution='none', selected_next='T04-explicit-container-captures',
                      rust_tests_debug=len(passed), rust_tests_release=len(passed), doctests_each_profile=2,
                      model_tests=56, receipt_tests=6, baseline_test_names=sorted(old_names()), current_test_names=sorted(passed),
                      logs_sha256=digest(raw), artifact_sha256=delivery())
        RECORD.write_text(json.dumps(record, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
