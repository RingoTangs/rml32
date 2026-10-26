"""Validate 2AZ function/data evidence without executing the reference artifact."""
import importlib.util
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
DIRECTORY = 'docs/stage2az-evidence/'
PREVIOUS = 'docs/stage2ay-evidence/'


def module_at(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_old = module_at('_2az_frozen_ay_evidence', ROOT / PREVIOUS / 'verify_evidence.py')
Elf, TARGET = _old.Elf, _old.TARGET
canonical, digest, require = _old.canonical, _old.digest, _old.require
safe_path, load, describe = _old.safe_path, _old.load, _old.describe
validate_records, validate_origins = _old.validate_records, _old.validate_origins
validate_frozen_origins, validate_anchors = _old.validate_frozen_origins, _old.validate_anchors
validate_case_map, validate_dispatch_table = _old.validate_case_map, _old.validate_dispatch_table


def validate_bundles(new, reused, manifest, elf):
    # A failed acquisition remains a recorded failure, never an invented dossier.
    require(reused, 'missing retained function inventory')
    require(all(r['operation'] == 'analyze_function' for r in new), 'new evidence is not a full dossier')
    groups = {'rea.json.gz': {r['evidence_id'] for r in new},
              'reused.json.gz': {r['evidence_id'] for r in reused}}
    require(not groups['rea.json.gz'] & groups['reused.json.gz'], 'new/reused evidence overlap')
    for item in manifest:
        require(item['bundle'] in groups and item['evidence_id'] in groups[item['bundle']],
                'wrong bundle attribution')
    return validate_records(new + reused, manifest, elf)


def validate_findings(findings, cases, questions, byid, anchors):
    ids = {f['id'] for f in findings}
    anchor_ids = {a['id'] for a in anchors}
    reference_ids = {eid for eid, r in byid.items() if r['operation'] == 'xrefs'}
    require(findings and len(ids) == len(findings), 'duplicate/empty findings')
    for finding in findings:
        require(finding['kind'] in ('observation', 'static-inference', 'design', 'unknown') and
                finding['status'] in ('answered', 'bounded', 'partial', 'unresolved') and
                finding['claim'] and finding['limits'], 'invalid finding')
        require(finding['evidence'] and set(finding['evidence']) <= byid.keys(),
                'dangling finding evidence')
        require(isinstance(finding['anchors'], list) and set(finding['anchors']) <= anchor_ids,
                'dangling finding anchor')
        require(finding['anchors'] or set(finding['evidence']) & reference_ids,
                'finding lacks function anchors or retained data references')
        require(all(a['evidence_id'] in finding['evidence'] for a in anchors
                    if a['id'] in finding['anchors']), 'finding anchor provenance mismatch')
        require(not (finding['kind'] == 'unknown' and finding['status'] == 'answered'),
                'unknown finding claimed answered')
    require(cases and len({c['id'] for c in cases}) == len(cases), 'duplicate/empty cases')
    for case in cases:
        require(case['basis'] in ('static-inference', 'design', 'unresolved') and
                case['input'] and case['expected'] and case['boundary'], 'invalid expected case')
        require(case['findings'] and set(case['findings']) <= ids, 'dangling expected finding')
        require(case['runtime_executed'] is False, 'static case claimed runtime execution')
        if case['basis'] == 'static-inference':
            require(any(f['kind'] in ('observation', 'static-inference') for f in findings
                        if f['id'] in case['findings']), 'static case lacks observational basis')
    require(any(c['basis'] == 'unresolved' for c in cases), 'missing unresolved boundary cases')
    require([q['id'] for q in questions] == [f'Q{i:02}' for i in range(1, 7)],
            'question inventory mismatch')
    by_finding = {f['id']: f for f in findings}
    for question in questions:
        require(question['status'] in ('answered', 'partial', 'unresolved') and
                question['findings'] and set(question['findings']) <= ids and
                question['remaining'] and question['next_action'], 'invalid question status')
        if question['id'] in ('Q05', 'Q06'):
            require(question['status'] == 'partial', 'excluded exit/GC parent must remain partial')
        if question['status'] == 'answered':
            require(all(by_finding[f]['kind'] != 'unknown' and
                        by_finding[f]['status'] not in ('partial', 'unresolved')
                        for f in question['findings']), 'unresolved question claimed answered')
        children = question.get('subquestions', [])
        require(len({c['id'] for c in children}) == len(children), 'duplicate subquestion')
        for child in children:
            require(child['id'].startswith(question['id'] + '.') and
                    child['status'] in ('answered', 'partial', 'unresolved') and
                    child['findings'] and set(child['findings']) <= ids and
                    child['remaining'] and child['next_action'], 'invalid subquestion')
            if child['status'] == 'answered':
                require(all(by_finding[f]['kind'] != 'unknown' and
                            by_finding[f]['status'] not in ('partial', 'unresolved')
                            for f in child['findings']), 'unresolved subquestion claimed answered')
            if question['status'] == 'answered':
                require(child['status'] == 'answered', 'partial child under answered question')


def validate_session(session, new, references=()):
    require(session['before']['open'] is False and session['after']['open'] is False,
            'session lifecycle differs')
    require(session['target_execution'] == 'none', 'target execution claimed')
    require(session.get('target_sha256') == TARGET, 'session target differs')
    requests = session.get('analysis_requests', [])
    ref_requests = session.get('reference_requests', [])
    require(requests or ref_requests, 'missing analysis/reference request ledger')
    byid = {r['evidence_id']: r for r in new}
    completed = set()
    for request in requests:
        require(request.get('procedure'), 'missing request procedure')
        if request['status'] == 'complete':
            require(request.get('evidence_id') in byid, 'request evidence missing')
            record = byid[request['evidence_id']]
            require(request['procedure'] == record['parameters']['procedure'], 'request procedure differs')
            completed.add(record['evidence_id'])
        else:
            require(request['status'] in ('failed', 'transport-limited') and request.get('reason'),
                    'unexplained incomplete analysis request')
    require(completed == set(byid), 'new dossier lacks completed request')
    byid = {r['evidence_id']: r for r in references}
    completed = set()
    for request in ref_requests:
        require(request.get('operation') == 'xrefs' and request.get('parameters'),
                'invalid reference request')
        if request['status'] == 'complete':
            require(request.get('evidence_id') in byid, 'reference request evidence missing')
            record = byid[request['evidence_id']]
            require(request['parameters'] == record['parameters'], 'reference request parameters differ')
            completed.add(record['evidence_id'])
        else:
            require(request['status'] in ('failed', 'transport-limited') and request.get('reason'),
                    'unexplained incomplete reference request')
    require(completed == set(byid), 'reference record lacks completed request')


def validate_function_coverage(new, findings, anchors):
    by_anchor = {a['id']: a['evidence_id'] for a in anchors}
    for record in new:
        eid = record['evidence_id']
        require(any(eid in f['evidence'] and any(by_anchor.get(a) == eid for a in f['anchors'])
                    for f in findings), 'new dossier lacks linked finding and anchor: ' + eid)


def validate_disposition(data, byid, anchors, findings):
    require(data['stage'] == '2AZ' and data['target_sha256'] == TARGET and
            data['target_execution'] == 'none', 'allocation disposition scope differs')
    rows = data['items']
    require(rows and len({r['id'] for r in rows}) == len(rows), 'duplicate/empty disposition rows')
    by_anchor = {a['id']: a for a in anchors}
    by_finding = {f['id']: f for f in findings}
    covered = set()
    for row in rows:
        for field in ('id', 'entity', 'allocation_branch', 'acquired', 'saved', 'publication',
                      'recovery', 'physical_release', 'semantic_release', 'status',
                      'limitations', 'next_action'):
            require(isinstance(row.get(field), str) and row[field].strip(),
                    'missing disposition field: ' + field)
        require(row['entity'] in ('H', 'A', 'creator'), 'invalid disposition entity')
        if row['entity'] == 'creator':
            require(row['allocation_branch'] == 'not-applicable' and
                    row.get('creator_state') in ('present', 'absent'), 'invalid creator disposition state')
            covered.add(('creator', row['creator_state']))
        else:
            require(row['allocation_branch'] in ('extension-pool', 'fallback'),
                    'invalid allocation branch')
            covered.add((row['entity'], row['allocation_branch']))
        require(row['status'] in ('answered', 'bounded', 'partial', 'unresolved', 'not-applicable'),
                'invalid disposition status')
        require(row['evidence'] and set(row['evidence']) <= byid.keys(),
                'dangling disposition evidence')
        require(isinstance(row['anchors'], list) and set(row['anchors']) <= by_anchor.keys(),
                'dangling disposition anchor')
        require(all(by_anchor[a]['evidence_id'] in row['evidence'] for a in row['anchors']),
                'disposition anchor provenance mismatch')
        require(row['findings'] and set(row['findings']) <= by_finding.keys(),
                'dangling disposition finding')
        if row['status'] == 'answered':
            require(all(by_finding[f]['kind'] != 'unknown' and
                        by_finding[f]['status'] not in ('partial', 'unresolved')
                        for f in row['findings']), 'unresolved disposition claimed answered')
    expected = {(entity, branch) for entity in ('H', 'A') for branch in ('extension-pool', 'fallback')}
    expected |= {('creator', 'present'), ('creator', 'absent')}
    require(expected <= covered, 'allocation disposition coverage incomplete')


def validate_delivery(root, new, reused, findings, cases, anchors, references=()):
    require(reused, 'missing retained function inventory')
    for name, current, size in [('anchors.json', anchors, 273),
                                ('findings.json', findings, 54),
                                ('expected-cases.json', cases, 94)]:
        original = load(root, PREVIOUS + name)
        require(len(original) == size and current[:size] == original,
                'historical ledger changed: ' + name)
    validate_function_coverage(new, findings, anchors)
    here = root / DIRECTORY
    for name in ('overview.md', 'method.md', 'next-batch.md', 'gate-review.md'):
        require((here / name).is_file(), 'missing delivery document: ' + name)
    byid = {r['evidence_id'] for r in new + reused + list(references)}
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
    references = load(root, DIRECTORY + 'references.json.gz')['records']
    manifest = load(root, DIRECTORY + 'function-manifest.json')
    function_byid = validate_bundles(new, reused, manifest, elf)
    validate_frozen_origins(reused, load(root, DIRECTORY + 'reused-manifest.json'), root,
                            load(root, DIRECTORY + 'baseline-inputs.json'))
    data_validator = module_at('_2az_data_references', root / DIRECTORY / 'data_references.py')
    reference_byid = data_validator.validate_reference_records(references)
    require(not function_byid.keys() & reference_byid.keys(), 'function/reference evidence overlap')
    byid = dict(function_byid, **reference_byid)
    anchors = load(root, DIRECTORY + 'anchors.json')
    validate_anchors(anchors, function_byid, elf)
    findings = load(root, DIRECTORY + 'findings.json')
    cases = load(root, DIRECTORY + 'expected-cases.json')
    questions = load(root, DIRECTORY + 'questions.json')
    validate_findings(findings, cases, questions, byid, anchors)
    mapping = load(root, DIRECTORY + 'case-map.json')
    original = load(root, PREVIOUS + 'expected-cases.json')
    require(len(original) == 94, '2AY source case inventory differs')
    validate_case_map(mapping, original, cases)
    validate_session(load(root, DIRECTORY + 'session.json'), new, references)
    data_validator.validate_data_references(root, elf, references,
        load(root, DIRECTORY + 'data-references.json'), byid, findings, anchors)
    validate_disposition(load(root, DIRECTORY + 'allocation-disposition.json'), byid, anchors, findings)
    validate_delivery(root, new, reused, findings, cases, anchors, references)
    summary = dict(stage='2AZ', new_records=len(new), reused_records=len(reused),
                   reference_records=len(references), unique_records=len(byid),
                   functions=len({m['name'] for m in manifest}),
                   anchors=len(anchors), findings=len(findings), expected_cases=len(cases),
                   case_mappings=len(mapping['items']),
                   question_status={q['id']: q['status'] for q in questions},
                   subquestion_status={s['id']: s['status'] for q in questions
                                       for s in q.get('subquestions', [])},
                   target_execution='none')
    renderer = module_at('_2az_current_render', root / DIRECTORY / 'render.py')
    require(renderer.check(root) == summary, 'generated evidence summary differs')
    return summary


if __name__ == '__main__':
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))
