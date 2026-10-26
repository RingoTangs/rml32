"""Check retained 2AY static evidence and generated views without target execution."""
import csv
import importlib.util
import io
import json
from pathlib import Path
import re
import struct

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
DIRECTORY = 'docs/stage2ay-evidence/'
PREVIOUS = 'docs/stage2ax-evidence/'


def module_at(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_old = module_at('_2ay_frozen_ax_evidence', ROOT / PREVIOUS / 'verify_evidence.py')
Elf, TARGET = _old.Elf, _old.TARGET
canonical, digest, require = _old.canonical, _old.digest, _old.require
safe_path, load, describe = _old.safe_path, _old.load, _old.describe
validate_records, validate_bundles = _old.validate_records, _old.validate_bundles
validate_origins, validate_frozen_origins = _old.validate_origins, _old.validate_frozen_origins
validate_anchors, validate_case_map = _old.validate_anchors, _old.validate_case_map


def validate_dispatch_table(root, elf):
    """Bind E01's two indirect action entries to frozen CSV and original ELF bytes."""
    source = 'docs/evidence/actions.csv'
    raw = safe_path(root, source).read_bytes()
    baseline = load(root, DIRECTORY + 'baseline-inputs.json')
    require(len(baseline['files']) == 734 and source in baseline['files'],
            'dispatch source is not in the frozen baseline')
    require(digest(raw) == baseline['files'][source], 'dispatch source hash differs')
    rows = list(csv.DictReader(io.StringIO(raw.decode('utf-8'))))
    expected = {
        0x17: ('vm_createFunctionValue', 0x08091f34, 2, 0),
        0x3b: ('vm_pushShortNumber', 0x08090ae8, 1, 8),
    }
    for opcode, (name, entry, parameters, attrib) in expected.items():
        selected = [row for row in rows if int(row['opcode'], 0) == opcode]
        require(len(selected) == 1, 'dispatch CSV row missing or duplicated')
        row = selected[0]
        record = 0x083cc680 + opcode * 28
        require(int(row['record'], 0) == record, 'dispatch record address differs')
        require(row['name'] == name and int(row['entry'], 0) == entry
                and int(row['parameters'], 0) == parameters and int(row['attrib'], 0) == attrib,
                'dispatch CSV contract differs')
        require(name in elf.symbols and elf.symbols[name]['address'] == entry,
                'dispatch ELF entry symbol differs')
        _, body = elf.locate(record, 28)
        observed = {key: struct.unpack_from('<I', body, offset)[0]
                    for key, offset in [('opcode', 0), ('compiler', 4), ('entry', 8), ('attrib', 24)]}
        observed['parameters'] = body[12]
        for field, value in observed.items():
            require(value == int(row[field], 0), 'dispatch ' + field + ' differs from original ELF')


def validate_findings(findings, cases, questions, byid, anchors):
    _old.validate_findings(findings, cases, questions, byid, anchors)
    require(all(q['status'] == 'partial' for q in questions if q['id'] in ('Q05', 'Q06')),
            'broad exit/GC parent must remain partial')


def validate_session(session, new):
    _old.validate_session(session, new)
    require(session.get('target_sha256') == TARGET, 'session target differs')


def validate_delivery(root, new, reused, findings, cases, anchors):
    require(new and reused, 'missing retained evidence inventory')
    for name, current, size in [('anchors.json', anchors, 180),
                                ('findings.json', findings, 40),
                                ('expected-cases.json', cases, 71)]:
        original = load(root, PREVIOUS + name)
        require(len(original) == size and current[:size] == original,
                'historical ledger changed: ' + name)
    by_anchor = {a['id']: a['evidence_id'] for a in anchors}
    for record in new:
        eid = record['evidence_id']
        require(any(eid in f['evidence'] and
                    any(by_anchor.get(a) == eid for a in f['anchors']) for f in findings),
                'new dossier lacks linked finding and anchor: ' + eid)
    here = root / DIRECTORY
    for name in ('overview.md', 'method.md', 'next-batch.md', 'gate-review.md'):
        require((here / name).is_file(), 'missing delivery document: ' + name)
    byid = {r['evidence_id'] for r in new + reused}
    for page in here.glob('*.md'):
        require(set(re.findall(r'ev_[a-f0-9]{64}', page.read_text())) <= byid,
                'document cites missing evidence: ' + page.name)


def verify(root=ROOT):
    raw = safe_path(root, 'magic_Linux32').read_bytes()
    require(digest(raw) == TARGET, 'reference target changed')
    elf = Elf(raw)
    validate_dispatch_table(root, elf)
    new = load(root, DIRECTORY + 'rea.json.gz')['records']
    reused = load(root, DIRECTORY + 'reused.json.gz')['records']
    manifest = load(root, DIRECTORY + 'function-manifest.json')
    byid = validate_bundles(new, reused, manifest, elf)
    validate_frozen_origins(reused, load(root, DIRECTORY + 'reused-manifest.json'), root,
                            load(root, DIRECTORY + 'baseline-inputs.json'))
    anchors = load(root, DIRECTORY + 'anchors.json')
    validate_anchors(anchors, byid, elf)
    findings = load(root, DIRECTORY + 'findings.json')
    cases = load(root, DIRECTORY + 'expected-cases.json')
    questions = load(root, DIRECTORY + 'questions.json')
    validate_findings(findings, cases, questions, byid, anchors)
    mapping = load(root, DIRECTORY + 'case-map.json')
    original = load(root, PREVIOUS + 'expected-cases.json')
    require(len(original) == 71, '2AX source case inventory differs')
    validate_case_map(mapping, original, cases)
    validate_session(load(root, DIRECTORY + 'session.json'), new)
    validate_delivery(root, new, reused, findings, cases, anchors)
    summary = dict(stage='2AY', new_records=len(new), reused_records=len(reused),
                   unique_records=len(byid), functions=len({m['name'] for m in manifest}),
                   anchors=len(anchors), findings=len(findings), expected_cases=len(cases),
                   case_mappings=len(mapping['items']),
                   question_status={q['id']: q['status'] for q in questions},
                   subquestion_status={s['id']: s['status'] for q in questions
                                       for s in q.get('subquestions', [])},
                   target_execution='none')
    renderer = module_at('_2ay_current_render', root / DIRECTORY / 'render.py')
    require(renderer.check(root) == summary, 'generated evidence summary differs')
    return summary


if __name__ == '__main__':
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))
