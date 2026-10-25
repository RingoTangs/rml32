"""Offline 2AX provenance checks, not a proof of prose or native execution."""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
DIRECTORY = 'docs/stage2ax-evidence/'
_spec = importlib.util.spec_from_file_location('_2ax_frozen_2ar', ROOT / 'docs/stage2ar-evidence/verify_evidence.py')
_old = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_old)
Elf, TARGET = _old.Elf, _old.TARGET
canonical, digest, require = _old.canonical, _old.digest, _old.require
safe_path, load, describe = _old.safe_path, _old.load, _old.describe
validate_origins, validate_anchors = _old.validate_origins, _old.validate_anchors


def validate_records(records, manifest, elf):
    require(records, 'empty evidence inventory')
    for record in records:
        require(record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed',
                'record is not an observed static artifact')
    return _old.validate_records(records, manifest, elf)


def validate_bundles(new, reused, manifest, elf):
    require(new and reused, 'new and reused evidence must both be present')
    require(all(r['operation'] == 'analyze_function' for r in new), 'new evidence is not a full dossier')
    groups = {'rea.json.gz': {r['evidence_id'] for r in new},
              'reused.json.gz': {r['evidence_id'] for r in reused}}
    require(not (groups['rea.json.gz'] & groups['reused.json.gz']), 'new/reused evidence overlap')
    for item in manifest:
        require(item['bundle'] in groups and item['evidence_id'] in groups[item['bundle']],
                'wrong bundle attribution')
    return validate_records(new + reused, manifest, elf)


def validate_frozen_origins(reused, origins, root, baseline):
    for origin in origins:
        require(origin['source'] in baseline['files'] and
                origin['source_sha256'] == baseline['files'][origin['source']],
                'reuse source is not a frozen baseline artifact')
    validate_origins(reused, origins, root)


def validate_findings(findings, cases, questions, byid, anchors):
    _old.validate_findings(findings, cases, questions, byid, anchors)
    findings_by_id = {f['id']: f for f in findings}
    for finding in findings:
        require(not (finding['kind'] == 'unknown' and finding['status'] == 'answered'),
                'unknown finding claimed answered')
    # The parent questions retain their original broad scope. Ordinary-thread
    # subquestions cannot close other-thread/JIT or complete-GC coverage.
    for question in questions:
        if question['id'] in ('Q05', 'Q06'):
            require(question['status'] != 'answered', 'excluded exit/GC domains claimed answered')
        children = question.get('subquestions', [])
        require(len({q['id'] for q in children}) == len(children), 'duplicate subquestion')
        for child in children:
            require(child['id'].startswith(question['id'] + '.') and
                    child['status'] in ('answered', 'partial', 'unresolved') and
                    child['findings'] and set(child['findings']) <= findings_by_id.keys() and
                    child['remaining'] and child['next_action'], 'invalid subquestion')
            if child['status'] == 'answered':
                require(all(findings_by_id[f]['kind'] != 'unknown' and
                            findings_by_id[f]['status'] not in ('partial', 'unresolved')
                            for f in child['findings']), 'unresolved subquestion claimed answered')
            if question['status'] == 'answered':
                require(child['status'] == 'answered', 'partial child under answered question')


def validate_case_map(mapping, original, cases):
    rows = mapping['items']
    require([r['source_case_id'] for r in rows] == [c['id'] for c in original],
            'original case inventory changed')
    current = {c['id'] for c in cases}
    for row, source in zip(rows, original):
        require(row['source_basis'] == source['basis'] and row['source_findings'] == source['findings'],
                'original case basis changed')
        require(row['case_ids'] and len(set(row['case_ids'])) == len(row['case_ids']) and
                set(row['case_ids']) <= current, 'case mapping has missing/duplicate target')
        require(row['status'] in ('reassessed', 'unchanged-boundary') and row['limitations'],
                'case mapping lacks bounded disposition')


def validate_session(session, new):
    require(session['before']['open'] is False and session['after']['open'] is False,
            'session lifecycle differs')
    require(session['target_execution'] == 'none', 'target execution claimed')
    byid = {r['evidence_id']: r for r in new}
    completed = set()
    require(session['analysis_requests'], 'missing analysis request ledger')
    for request in session['analysis_requests']:
        require(request['procedure'], 'missing request procedure')
        if request['status'] == 'complete':
            require(request.get('evidence_id') in byid, 'request evidence missing')
            evidence = byid[request['evidence_id']]
            require(request['procedure'] == evidence['parameters']['procedure'], 'request procedure differs')
            completed.add(request['evidence_id'])
        else:
            require(request['status'] in ('failed', 'transport-limited') and request.get('reason'),
                    'unexplained incomplete analysis request')
    require(completed == set(byid), 'new dossier lacks completed request')


def verify(root=ROOT):
    raw = safe_path(root, 'magic_Linux32').read_bytes()
    require(digest(raw) == TARGET, 'reference target changed')
    elf = Elf(raw)
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
    validate_case_map(mapping, load(root, 'docs/stage2ar-evidence/expected-cases.json'), cases)
    validate_session(load(root, DIRECTORY + 'session.json'), new)
    return dict(stage='2AX', new_records=len(new), reused_records=len(reused),
                unique_records=len(byid), functions=len({m['name'] for m in manifest}),
                anchors=len(anchors), findings=len(findings), expected_cases=len(cases),
                case_mappings=len(mapping['items']),
                question_status={q['id']: q['status'] for q in questions},
                subquestion_status={s['id']: s['status'] for q in questions for s in q.get('subquestions', [])},
                target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))
