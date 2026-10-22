"""Verify immutable 2AN evidence and protected inputs across the explicit 2AO source migration."""
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
CHANGED = {'src/lib.rs', 'src/prepared.rs', 'src/value.rs', 'src/vm.rs', 'tests/vm_core.rs',
           'README.md', 'docs/magic_Linux32-rust-rewrite-roadmap.md'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def safe_path(root, name):
    require(bool(name) and not Path(name).is_absolute() and '..' not in Path(name).parts
            and '\\' not in name, 'unsafe input path')
    path = root / name
    require(path.resolve().is_relative_to(root.resolve()), 'input escapes root')
    return path


def check_hashes(root, baseline, exempt):
    for name, expected in baseline.items():
        path = safe_path(root, name)
        # Even exempt baseline files must continue to exist.
        require(path.is_file(), 'missing baseline input: ' + name)
        if name not in exempt:
            require(digest(path.read_bytes()) == expected, 'protected input changed: ' + name)


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def check_opcodes(source, baseline):
    match = re.search(r'pub const SUPPORTED_OPCODES: &\[u8\] = &\[(.*?)\];', source, re.S)
    require(match is not None, 'missing supported opcode table')
    opcodes = [int(value, 16) for value in re.findall(r'0x[0-9a-f]+', match[1])]
    corpus = {int(value, 16) for value in baseline['opcodes']}
    require(len(opcodes) == len(set(opcodes)) == 75 and len(set(opcodes) & corpus) == 73,
            'opcode boundary changed')


def verify(root=ROOT):
    baseline = json.loads((root / 'docs/stage2ao-evidence/baseline-inputs.json').read_text())
    require(CHANGED <= set(baseline) and 'magic_Linux32' in baseline
            and 'docs/stage2an-validation.json' in baseline, 'baseline inventory incomplete')
    check_hashes(root, baseline, CHANGED)
    # The 2AN verifier freezes pre-migration Rust hashes. Reuse its static
    # validators directly, never rewrite that historical full-snapshot receipt.
    an = module('_stage2ao_2an_evidence', ROOT / 'docs/stage2an-evidence/verify_evidence.py')
    raw = (root / 'magic_Linux32').read_bytes()
    require(digest(raw) == an.TARGET, 'reference target changed')
    elf = an.Elf(raw)
    directory = 'docs/stage2an-evidence/'
    new = an.load(root, directory + 'rea.json.gz')['records']
    reused = an.load(root, directory + 'reused.json.gz')['records']
    records = an.validate_records(new + reused, an.load(root, directory + 'function-manifest.json'), elf)
    an.validate_origins(reused, an.load(root, directory + 'reused-manifest.json'), root)
    an.validate_anchors(an.load(root, directory + 'anchors.json'), records, elf)
    an.validate_findings(an.load(root, directory + 'findings.json'),
                         an.load(root, directory + 'expected-cases.json'), records)
    corpus = module('_stage2ao_2an_corpus', ROOT / 'docs/stage2an-evidence/analyze_corpus.py')
    for name, expected in corpus.artifacts(root).items():
        require((root / directory / name).read_bytes() == expected, '2AN corpus changed: ' + name)
    compatibility = json.loads((root / 'docs/compatibility-evidence/compatibility-summary.json').read_text())
    check_opcodes((root / 'src/prepared.rs').read_text(), compatibility)
    return dict(protected_files=len(baseline) - len(CHANGED), allowed_existing_paths=len(CHANGED),
                static_records=len(records), global_creation_locations=5012,
                supported_opcodes=75, corpus_intersection=73)


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
