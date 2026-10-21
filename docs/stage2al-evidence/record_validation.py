"""Explicit --write reruns checks and records 2AL delivery; default verifies existing receipt."""
import argparse
import gzip
import json
import subprocess

from capture_native import HERE, ROOT, TARGET, canonical, digest
from verify_validation import COMMANDS, validate_commands, verify


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    if not args.write:
        print(json.dumps(verify(), sort_keys=True))
        return
    commands = []
    for argv in COMMANDS:
        print('Checking: ' + ' '.join(argv), flush=True)
        result = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True)
        commands.append(dict(argv=argv, returncode=result.returncode, stdout=result.stdout, stderr=result.stderr))
        if result.returncode:
            raise RuntimeError(result.stdout + result.stderr)
    validate_commands(commands)
    raw = gzip.compress(canonical(dict(commands=commands)), mtime=0)
    (HERE / 'validation-logs.json.gz').write_bytes(raw)
    files = list(p for p in HERE.iterdir() if p.is_file())
    files += [ROOT / 'README.md', ROOT / 'docs/magic_Linux32-rust-rewrite-roadmap.md',
              ROOT / 'docs/vm-mapping-iteration-semantics.md']
    receipt = dict(stage='2AL', target_sha256=TARGET, runtime_changed=False,
                   supported_opcodes=75, corpus_intersection=73, logs_sha256=digest(raw),
                   evidence=dict(local_functions=21, reused_records=13, instruction_locations=2018,
                                 findings=14, expected_cases=43, corpus_positions=298369, corpus_examples=52),
                   tests=dict(rust_debug=487, rust_release=487, python=28),
                   commands=[dict(argv=r['argv'], returncode=r['returncode']) for r in commands],
                   artifact_sha256={str(p.relative_to(ROOT)): digest(p.read_bytes()) for p in sorted(files)},
                   limitations=['No new REA session: client approval policy never rejected open_binary.',
                                'Local static disassembly and reused Evidence, not native execution.',
                                'Full Mapping/G05, alias cleanup, deleted references and GC remain incomplete.'])
    (ROOT / 'docs/stage2al-validation.json').write_text(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
    print(json.dumps(verify(), sort_keys=True))


if __name__ == '__main__':
    main()
