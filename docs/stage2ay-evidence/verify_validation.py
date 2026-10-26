"""Reproduce or read-only verify the static-only 2AY validation receipt.

--write runs declared checks and replaces only the 2AY receipt and logs.
Neither mode executes the reference executable or business bytecode.
"""
import argparse
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RECORD = ROOT / 'docs/stage2ay-validation.json'
BASELINE_COMMIT = 'e67ba838cf34415faadc8a109833e7d6d0e45655'
PREFIX_ONLY = {'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}
CORPUS = 'docs/stage2aw-evidence/corpus-validation.json'
SELECTED_NEXT = 'T05-lifecycle-contract-evidence'
COMMANDS = [
    ['python3', '-B', 'docs/stage2ay-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2ax-evidence/verify_evidence.py'],
    ['python3', '-B', 'docs/stage2as-design/explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_model.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2as-design', '-p', 'test_explore.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2ax-evidence', '-p', 'test_verify_evidence.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2ax-evidence', '-p', 'test_verify_validation.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2ay-evidence', '-p', 'test_verify_evidence.py'],
    ['python3', '-B', '-m', 'unittest', 'discover', '-s', 'docs/stage2ay-evidence', '-p', 'test_verify_validation.py'],
    ['cargo', 'test', '--locked', '--offline'],
    ['cargo', 'test', '--release', '--locked', '--offline'],
    ['cargo', 'build', '--locked', '--offline'],
    ['cargo', 'fmt', '--check'],
    ['cargo', 'clippy', '--locked', '--offline', '--all-targets', '--', '-D', 'warnings'],
]
FROZEN_PYTHON_COUNTS = {
    'docs/stage2as-design/test_model.py': 50,
    'docs/stage2as-design/test_explore.py': 6,
    'docs/stage2ax-evidence/test_verify_evidence.py': 37,
    'docs/stage2ax-evidence/test_verify_validation.py': 15,
}


def require(value, message):
    if not value:
        raise AssertionError(message)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def load(path):
    raw = path.read_bytes()
    return json.loads(gzip.decompress(raw) if path.suffix == '.gz' else raw)


def module_at(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_old = module_at('_2ay_frozen_ax_receipt', ROOT / 'docs/stage2ax-evidence/verify_validation.py')
rust_names, validate_rust = _old.rust_names, _old.validate_rust
baseline_test_names = _old.baseline_test_names


def baseline():
    data = load(HERE / 'baseline-inputs.json')
    require(data['stage'] == '2AY' and data['baseline_stage'] == '2AX', 'baseline stage differs')
    require(data['baseline_commit'] == BASELINE_COMMIT, 'baseline commit differs')
    require(data['rust_tests_each_profile'] == 644 and data['doctests_each_profile'] == 2,
            'baseline test counts differ')
    require(data['target_execution'] == 'none', 'baseline execution scope differs')
    inventory = subprocess.check_output(
        ['git', 'ls-tree', '-r', '--name-only', '-z', BASELINE_COMMIT], cwd=ROOT
    ).decode().rstrip('\0').split('\0')
    require(set(data['files']) == set(inventory) and len(inventory) == 734,
            'baseline file inventory differs')
    for path, expected in data['files'].items():
        current = (ROOT / path).read_bytes()
        if path in PREFIX_ONLY:
            original = subprocess.check_output(['git', 'show', BASELINE_COMMIT + ':' + path], cwd=ROOT)
            require(digest(original) == expected, 'historical document identity differs: ' + path)
            require(current.endswith(original), 'historical document suffix changed: ' + path)
        else:
            require(digest(current) == expected, 'frozen input changed: ' + path)
    return data


def frozen_ax():
    """Replay prior logs; never run the old receipt's whole-delivery verifier."""
    receipt_path = ROOT / 'docs/stage2ax-validation.json'
    log_path = ROOT / 'docs/stage2ax-evidence/validation-logs.json.gz'
    receipt, raw = load(receipt_path), log_path.read_bytes()
    base = load(HERE / 'baseline-inputs.json')
    require(digest(receipt_path.read_bytes()) == base['files']['docs/stage2ax-validation.json']
            and digest(raw) == base['files']['docs/stage2ax-evidence/validation-logs.json.gz'],
            '2AX frozen receipt or logs changed')
    require(digest(raw) == receipt['logs_sha256'], '2AX validation logs changed')
    logs = json.loads(gzip.decompress(raw))
    require(logs['stage'] == '2AX' and logs['target_execution'] == 'none', '2AX frozen log scope differs')
    validation = _old.validate_commands(logs['commands'])
    require(all(receipt[key] == value for key, value in validation.items()),
            '2AX frozen log summary differs')
    require(receipt['rust_tests_debug'] == receipt['rust_tests_release'] == 644
            and receipt['doctests_each_profile'] == 2, '2AX frozen counts differ')
    require(receipt['container_capture_ready'] is True and
            all(receipt[key] is False for key in ('runtime_changed', 'public_api_changed',
                'dependencies_changed', 'direct_owner_capture_ready', 'non_scalar_ready', 'stage2_complete'))
            and receipt['target_execution'] == 'none', '2AX frozen scope differs')
    return dict(stage='2AX', status='pass', rust_tests_each_profile=644,
                doctests_each_profile=2, target_execution='none',
                receipt_sha256=digest(receipt_path.read_bytes()), logs_sha256=receipt['logs_sha256'],
                method='frozen log replay plus 2AY protected input hashes; no historical writes')


def python_count(output):
    counts = re.findall(r'^Ran (\d+) tests? in ', output, re.M)
    require(len(counts) == 1 and int(counts[0]) > 0 and output.rstrip().endswith('OK'),
            'Python test result missing or failed')
    return int(counts[0])


def json_stdout(command):
    value = json.loads(command['stdout'])
    require(isinstance(value, dict) and value, 'missing JSON verification result')
    return value


def validate_evidence_summary(evidence):
    require(evidence.get('stage') == '2AY' and evidence.get('target_execution') == 'none',
            '2AY evidence scope differs')
    require(type(evidence.get('new_records')) is int and evidence['new_records'] > 0
            and type(evidence.get('reused_records')) is int and evidence['reused_records'] > 0
            and evidence.get('unique_records') == evidence['new_records'] + evidence['reused_records']
            and evidence.get('case_mappings') == 71, '2AY evidence inventory differs')


def validate_commands(commands):
    require([c['argv'] for c in commands] == COMMANDS, 'command inventory differs')
    rust, python_counts = [], {}
    for command in commands:
        require(command['returncode'] == 0, 'command failed: ' + repr(command['argv']))
        require(isinstance(command['stdout'], str) and isinstance(command['stderr'], str),
                'command log text missing')
        argv, output = command['argv'], command['stdout'] + command['stderr']
        if argv[:2] == ['cargo', 'test']:
            rust.append(validate_rust(output))
        if 'unittest' in argv:
            key = argv[argv.index('-s') + 1] + '/' + argv[argv.index('-p') + 1]
            python_counts[key] = python_count(output)
    require(len(rust) == 2 and rust[0] == rust[1], 'debug/release scenario inventory differs')
    evidence, previous, model = (json_stdout(c) for c in commands[:3])
    validate_evidence_summary(evidence)
    require(previous == load(ROOT / 'docs/stage2ax-evidence/summary.json'),
            '2AX evidence summary differs')
    require(model == load(ROOT / 'docs/stage2ax-validation.json')['model_summary'],
            'frozen model summary differs')
    require(all(python_counts[key] == count for key, count in FROZEN_PYTHON_COUNTS.items()),
            'frozen Python test counts differ')
    return dict(current_test_names=rust[0], python_test_counts=python_counts,
                evidence_summary=evidence, historical_evidence_summary=previous, model_summary=model)


def reused_corpus():
    raw = (ROOT / CORPUS).read_bytes()
    base = load(HERE / 'baseline-inputs.json')
    require(digest(raw) == base['files'][CORPUS], 'reused corpus hash differs')
    corpus = json.loads(raw)
    require(corpus['status'] == 'pass' and corpus['target_execution'] == 'none', 'corpus scope differs')
    require(corpus['checked'] == dict(packages=12, members=9527, programs=7159, inheritance=9933)
            and corpus['summary']['instructions'] == 6474773
            and corpus['known_errors'] == ['dba/pack_data/etc.pak/etc'], 'corpus inventory differs')
    return dict(source=CORPUS, sha256=digest(raw), source_stage='2AW', rerun=False,
                target_execution='none', checked=corpus['checked'], summary=corpus['summary'],
                known_errors=corpus['known_errors'], scope=corpus['scope'])


def delivery():
    paths = PREFIX_ONLY | {str(p.relative_to(ROOT)) for p in HERE.rglob('*')
                           if p.is_file() and '__pycache__' not in p.parts}
    paths.discard('docs/stage2ay-evidence/validation-logs.json.gz')
    return {path: digest((ROOT / path).read_bytes()) for path in sorted(paths)}


def current_evidence():
    return module_at('_2ay_current_evidence', HERE / 'verify_evidence.py').verify()


def validate_current_evidence(evidence):
    validate_evidence_summary(evidence)
    require(evidence == current_evidence(), 'evidence log differs from current delivery')


def capabilities():
    return dict(runtime_changed=False, public_api_changed=False, dependencies_changed=False,
                model_changed=False, source_compatible=True, container_capture_ready=True,
                direct_owner_capture_ready=False, non_scalar_ready=False, stage2_complete=False,
                new_native_analysis=True)


def validate_capabilities(record):
    require(all(type(record.get(key)) is bool and record[key] == value
                for key, value in capabilities().items()), 'receipt capability policy differs')


def build_record(commands):
    base = baseline()
    validation = validate_commands(commands)
    validate_current_evidence(validation['evidence_summary'])
    counts = validation['python_test_counts']
    return dict(stage='2AY', status='pass', baseline_stage='2AX',
                baseline_verification=frozen_ax(), protected_previous_files=len(base['files']),
                prefix_only_documents=sorted(PREFIX_ONLY), **capabilities(),
                target_execution='none', selected_next=SELECTED_NEXT,
                rust_tests_debug=644, rust_tests_release=644, doctests_each_profile=2,
                model_tests=56, preserved_python_tests=sum(FROZEN_PYTHON_COUNTS.values()),
                evidence_tests=counts['docs/stage2ay-evidence/test_verify_evidence.py'],
                receipt_tests=counts['docs/stage2ay-evidence/test_verify_validation.py'],
                baseline_test_names=sorted(baseline_test_names()), reused_corpus=reused_corpus(),
                artifact_sha256=delivery(), **validation)


def validate_record(record, commands, log_raw):
    require(digest(log_raw) == record['logs_sha256'], 'validation logs changed')
    validate_capabilities(record)
    expected = build_record(commands)
    require({key: value for key, value in record.items() if key != 'logs_sha256'} == expected,
            'receipt content or delivery hashes differ')
    return dict(stage='2AY', status='pass', protected_previous_files=record['protected_previous_files'],
                rust_tests_each_profile=644, doctests_each_profile=2, model_tests=56,
                preserved_python_tests=108, evidence_tests=record['evidence_tests'],
                receipt_tests=record['receipt_tests'], target_execution='none',
                runtime_changed=False, public_api_changed=False, direct_owner_capture_ready=False,
                non_scalar_ready=False, stage2_complete=False, selected_next=record['selected_next'],
                corpus='reused frozen 2AW result', evidence_summary=record['evidence_summary'])


def verify():
    raw = (HERE / 'validation-logs.json.gz').read_bytes()
    logs = json.loads(gzip.decompress(raw))
    require(logs['stage'] == '2AY' and logs['target_execution'] == 'none', 'log scope differs')
    return validate_record(load(RECORD), logs['commands'], raw)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if args.write:
        baseline()
        frozen_ax()
        commands = []
        for argv in COMMANDS:
            print('Checking: ' + ' '.join(argv), flush=True)
            result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
            commands.append(dict(argv=argv, returncode=result.returncode,
                                 stdout=result.stdout, stderr=result.stderr))
            require(result.returncode == 0, result.stdout + result.stderr)
        record = build_record(commands)
        raw = gzip.compress(json.dumps(dict(stage='2AY', target_execution='none', commands=commands),
                                       ensure_ascii=False, sort_keys=True).encode(), mtime=0)
        record['logs_sha256'] = digest(raw)
        (HERE / 'validation-logs.json.gz').write_bytes(raw)
        RECORD.write_text(json.dumps(record, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
