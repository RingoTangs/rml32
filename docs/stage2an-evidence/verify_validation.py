"""Default checks the receipt; --write reruns checks and freezes only the 2AN delivery."""
import argparse
import gzip
import json
from pathlib import Path
import re
import subprocess

from verify_evidence import ROOT, HERE, canonical, digest, require, safe_path, verify as verify_evidence
from analyze_corpus import artifacts

COMMANDS = [
    ['python3', '-B', 'docs/stage2an-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2an-evidence/analyze_corpus.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2an-evidence', '-p', 'test_*.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
]
ENTRYPOINTS = ['README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md',
               'docs/vm-global-function-value-semantics.md']


def files():
    return sorted(ENTRYPOINTS + [str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file()])


def validate_commands(commands):
    require([c['argv'] for c in commands] == COMMANDS, 'command inventory differs')
    for command in commands:
        require(command['returncode'] == 0, 'check failed')
        output = command['stdout'] + command['stderr']
        if command['argv'][:2] == ['cargo', 'test']:
            matches = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', output)
            require(matches and sum(int(n) for n, _ in matches) == 515
                    and all(int(n) == 0 for _, n in matches), 'Rust baseline differs')
        if 'unittest' in command['argv']:
            require(re.search(r'Ran 23 tests\b', output) and output.rstrip().endswith('OK'), 'Python results differ')


def verify():
    receipt = json.loads((ROOT / 'docs/stage2an-validation.json').read_text())
    require(receipt['stage'] == '2AN' and receipt['rust_tests'] == 515 and receipt['python_tests'] == 23
            and receipt['supported_opcodes'] == 75 and receipt['corpus_intersection'] == 73
            and receipt['runtime_changed'] is False, 'receipt boundary differs')
    logs = (HERE / 'validation-logs.json.gz').read_bytes()
    require(digest(logs) == receipt['logs_sha256'], 'command logs changed')
    validate_commands(json.loads(gzip.decompress(logs))['commands'])
    require(set(receipt['artifact_sha256']) == set(files()), 'delivery inventory differs')
    for name, expected in receipt['artifact_sha256'].items():
        require(digest(safe_path(ROOT, name).read_bytes()) == expected, 'delivery changed: ' + name)
    evidence = verify_evidence()
    for name, expected in artifacts().items():
        require((HERE / name).read_bytes() == expected, 'corpus changed: ' + name)
    return dict(stage='2AN', commands=len(COMMANDS), delivery_files=len(files()),
                rust_tests_debug=515, rust_tests_release=515, python_tests=23, **evidence)


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
        receipt = dict(stage='2AN', rust_tests=515, python_tests=23, runtime_changed=False,
                       supported_opcodes=75, corpus_intersection=73,
                       logs_sha256=digest(logs),
                       artifact_sha256={name: digest((ROOT / name).read_bytes()) for name in files()},
                       limitations=['Static evidence and synthetic tests only; no target or business bytecode execution.',
                                    'Global/c and Host/K designs are not implemented; G12/G18 and stage 2 remain open.',
                                    'Native arbitrary recompilation, unload, non-scalar captures and deferred deletion remain unproven.'])
        (ROOT / 'docs/stage2an-validation.json').write_text(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2)+'\n')
    print(json.dumps(verify(), sort_keys=True))


if __name__ == '__main__':
    main()
