"""Check the 2AO receipt; --write reruns required checks and freezes this stage only."""
import argparse
import gzip
import json
from pathlib import Path
import re
import subprocess

from verify_static_evidence import ROOT, HERE, CHANGED, digest, require, safe_path, verify as verify_static

COMMANDS = [
    ['python3', '-B', 'docs/stage2ao-evidence/verify_static_evidence.py'],
    ['python3', '-B', 'docs/stage2ao-evidence/run_historical_tests.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2ao-evidence', '-p', 'test_*.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
]


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()


def files():
    return sorted(CHANGED | {'docs/vm-global-function-value-runtime.md'} |
                  {str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file()})


def passed_tests(output):
    return set(re.findall(r'^test (\S+) \.\.\. ok$', output, re.M))


def validate_commands(commands):
    require([c['argv'] for c in commands] == COMMANDS, 'command inventory differs')
    historical = json.loads(gzip.decompress((ROOT / 'docs/stage2an-evidence/validation-logs.json.gz').read_bytes()))['commands']
    baseline = next(c for c in historical if c['argv'] == ['cargo', 'test', '--locked', '--offline'])
    original = passed_tests(baseline['stdout'] + baseline['stderr'])
    require(len(original) == 515, 'historical Rust test inventory differs')
    for c in commands:
        require(c['returncode'] == 0, 'failed check')
        output = c['stdout'] + c['stderr']
        if c['argv'][:2] == ['cargo', 'test']:
            counts = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', output)
            require(counts and sum(int(n) for n, _ in counts) == 544
                    and all(int(n) == 0 for _, n in counts), 'Rust results differ')
            current = passed_tests(output)
            require(original <= current and len(current - original) == 29, 'Rust regression inventory differs')
        if 'unittest' in c['argv'] or 'docs/stage2ao-evidence/run_historical_tests.py' in c['argv']:
            count = 5 if 'unittest' in c['argv'] else 22
            require(re.search(rf'Ran {count} tests\b', output) and output.rstrip().endswith('OK'), 'Python results differ')


def verify():
    receipt = json.loads((ROOT / 'docs/stage2ao-validation.json').read_text())
    require(receipt['stage'] == '2AO' and receipt['rust_tests'] == 544 and receipt['python_tests'] == 27
            and receipt['supported_opcodes'] == 75 and receipt['corpus_intersection'] == 73,
            'receipt boundary differs')
    logs = (HERE / 'validation-logs.json.gz').read_bytes()
    require(digest(logs) == receipt['logs_sha256'], 'validation logs changed')
    validate_commands(json.loads(gzip.decompress(logs))['commands'])
    require(set(receipt['artifact_sha256']) == set(files()), 'delivery inventory differs')
    for name, expected in receipt['artifact_sha256'].items():
        require(digest(safe_path(ROOT, name).read_bytes()) == expected, 'delivery changed: ' + name)
    return dict(stage='2AO', commands=len(COMMANDS), delivery_files=len(files()),
                rust_tests_debug=544, rust_tests_release=544, python_tests=27, **verify_static())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        commands = []
        for argv in COMMANDS:
            print('Checking: ' + ' '.join(argv), flush=True)
            result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
            commands.append(dict(argv=argv, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr))
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)
        validate_commands(commands)
        logs = gzip.compress(canonical(dict(commands=commands)), mtime=0)
        (HERE / 'validation-logs.json.gz').write_bytes(logs)
        receipt = dict(stage='2AO', rust_tests=544, python_tests=27, supported_opcodes=75, corpus_intersection=73,
                       logs_sha256=digest(logs), artifact_sha256={name:digest((ROOT / name).read_bytes()) for name in files()},
                       limitations=['Synthetic execution only; no target or business bytecode execution.',
                                    'Global/c new calls require the saved program to remain bound.',
                                    'Host/K, non-scalar captures, native deferred deletion and stage 2 remain incomplete.'])
        (ROOT / 'docs/stage2ao-validation.json').write_text(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2)+'\n')
    print(json.dumps(verify(), sort_keys=True))


if __name__ == '__main__':
    main()
