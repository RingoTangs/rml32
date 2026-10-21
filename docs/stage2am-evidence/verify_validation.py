"""Default verifies receipt. Explicit --write reruns checks and freezes current 2AM files."""
import argparse
import gzip
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / 'docs/stage2al-evidence'))
from capture_native import canonical, digest, require

COMMANDS = [
    ['python3', '-B', 'docs/stage2am-evidence/verify_static_evidence.py'],
    ['python3', '-B', 'docs/stage2al-evidence/analyze_corpus.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2al-evidence', '-p', 'test_mapping_model.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2al-evidence', '-p', 'test_analyze_corpus.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
]
CHANGED = ['src/container.rs', 'src/vm.rs', 'tests/vm_core.rs', 'tests/fixtures/mapping-order-2al.json',
           'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md', 'docs/vm-mapping-runtime.md']


def validate_commands(commands):
    require([r['argv'] for r in commands] == COMMANDS, 'command inventory')
    for row in commands:
        require(row['returncode'] == 0, 'failed command')
        output = row['stdout'] + row['stderr']
        if row['argv'][:2] == ['cargo', 'test']:
            matches = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', output)
            require(matches and sum(int(n) for n, _ in matches) == 515
                    and all(int(n) == 0 for _, n in matches), 'Rust results')
        elif 'unittest' in row['argv']:
            count = 10 if row['argv'][-1] == 'test_mapping_model.py' else 9
            require(re.search(rf'Ran {count} tests\b', output) and output.rstrip().endswith('OK'), 'Python results')


def files():
    return sorted(CHANGED + [str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file()])


def verify():
    receipt = json.loads((ROOT / 'docs/stage2am-validation.json').read_text())
    require(receipt['stage'] == '2AM' and receipt['rust_tests'] == 515
            and receipt['python_tests'] == 19 and receipt['supported_opcodes'] == 75
            and receipt['corpus_intersection'] == 73, 'receipt boundary')
    logs = (HERE / 'validation-logs.json.gz').read_bytes()
    require(digest(logs) == receipt['logs_sha256'], 'logs changed')
    validate_commands(json.loads(gzip.decompress(logs))['commands'])
    require(set(receipt['artifact_sha256']) == set(files()), 'delivery inventory')
    for name, expected in receipt['artifact_sha256'].items():
        require(digest((ROOT / name).read_bytes()) == expected, 'delivery changed: ' + name)
    # Recheck protected artifacts now, rather than trusting a saved successful log.
    from verify_static_evidence import verify as verify_static
    static = verify_static()
    return dict(commands=len(COMMANDS), delivery_files=len(files()), rust_tests_debug=515,
                rust_tests_release=515, python_tests=19, **static)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        commands = []
        for argv in COMMANDS:
            print('Checking: ' + ' '.join(argv), flush=True)
            result = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True)
            commands.append(dict(argv=argv, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr))
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)
        validate_commands(commands)
        logs = gzip.compress(canonical(dict(commands=commands)), mtime=0)
        (HERE / 'validation-logs.json.gz').write_bytes(logs)
        receipt = dict(stage='2AM', rust_tests=515, python_tests=19, supported_opcodes=75,
                       corpus_intersection=73, logs_sha256=digest(logs),
                       artifact_sha256={name:digest((ROOT / name).read_bytes()) for name in files()},
                       limitations=['Synthetic execution only; no reference executable or business bytecode.',
                                    'G05 remains partial; deletion references, other keys/attributes, GC and lifecycle loops excluded.'])
        (ROOT / 'docs/stage2am-validation.json').write_text(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2)+'\n')
    print(json.dumps(verify(), sort_keys=True))


if __name__ == '__main__':
    main()
