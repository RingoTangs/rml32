"""Offline provenance and consistency checks; never executes target or proves prose semantics."""
import argparse
import gzip
import importlib.util
import json
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location('_2ar_elf', ROOT/'docs/stage2al-evidence/capture_native.py')
_native = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_native)
Elf, TARGET = _native.Elf, _native.TARGET
canonical, digest, require = _native.canonical, _native.digest, _native.require


def safe_path(root, name):
    p = PurePosixPath(name)
    require(bool(name) and not p.is_absolute() and '..' not in p.parts and '\\' not in name, 'unsafe source path')
    path = root/name
    require(path.resolve().is_relative_to(root.resolve()), 'source escapes root')
    return path


def load(root, name):
    raw = safe_path(root, name).read_bytes()
    return json.loads(gzip.decompress(raw) if name.endswith('.gz') else raw)


def assembly(record):
    if record['operation'] == 'analyze_function':
        return record['normalized_result']['assembly']
    if record['operation'] == 'procedure_assembly':
        return record['normalized_result'].splitlines()
    return []


def describe(record, elf, bundle):
    name = record['parameters']['procedure']
    require(name in elf.symbols, 'unknown ELF symbol')
    sym = elf.symbols[name]
    offset, raw = elf.locate(sym['address'], sym['size'])
    n = record['normalized_result']
    ranges = []
    if record['operation'] == 'analyze_function':
        for span in n['procedure']['body'].get('ranges', []):
            start, end = int(span['start'], 16), int(span['end'], 16)
            off, body = elf.locate(start, end-start+1)
            ranges.append(dict(start=span['start'], end=span['end'], file_offset=off, sha256=digest(body)))
    return dict(name=name, operation=record['operation'], evidence_id=record['evidence_id'], bundle=bundle,
                address=hex(sym['address']), size=sym['size'], file_offset=offset, body_sha256=digest(raw),
                record_sha256=digest(canonical(record)), analyzed_ranges=ranges,
                range_basis='Ghidra inclusive body ranges' if ranges else 'ELF symbol extent only; no recovered CFG ranges',
                limitations=record['limitations'])


def validate_records(records, manifest, elf):
    ids = [r['evidence_id'] for r in records]
    require(len(ids) == len(set(ids)), 'duplicate Evidence ID')
    require(len(manifest) == len(records) and {m['evidence_id'] for m in manifest} == set(ids), 'manifest partition mismatch')
    byid = {r['evidence_id']: r for r in records}
    for m in manifest:
        r = byid[m['evidence_id']]
        require(r['subject']['digest']['sha256'] == TARGET and r['subject']['format'] == 'elf'
                and r['subject']['architecture'] == 'x86', 'wrong target')
        require(r['provider']['id'] == 'ghidra' and r['provider']['version'] == '12.1.4', 'wrong provider')
        require(r['operation'] in ('analyze_function', 'procedure_assembly', 'procedure_pseudo_code'), 'unexpected operation')
        require(r['raw_result'] is not None and r['normalized_result'] and r['limitations'], 'incomplete evidence')
        require(m == describe(r, elf, m['bundle']), 'manifest identity/range/digest differs')
        n = r['normalized_result']
        if r['operation'] == 'analyze_function':
            require(n['procedure']['name'] == m['name'] and int(n['procedure']['address'], 16) == int(m['address'], 16), 'dossier identity differs')
            require(n['pseudocode'] and n['assembly'] and n['procedure']['body']['available'] and m['analyzed_ranges'], 'incomplete dossier')
            require(int(n['assembly'][0].split(':')[0], 16) == int(m['address'], 16), 'dossier entry differs')
    return byid


def validate_origins(reused, origins, root):
    require(len(origins) == len(reused) and {x['evidence_id'] for x in origins} == {r['evidence_id'] for r in reused}, 'reuse partition mismatch')
    byid = {r['evidence_id']: r for r in reused}
    cache = {}
    for origin in origins:
        source = origin['source']
        raw = safe_path(root, source).read_bytes()
        require(digest(raw) == origin['source_sha256'], 'reuse source changed')
        if source not in cache:
            cache[source] = json.loads(gzip.decompress(raw))['records']
        r = byid[origin['evidence_id']]
        require(r in cache[source] and digest(canonical(r)) == origin['record_sha256'], 'reuse record changed')
        require(origin['name'] == r['parameters']['procedure'] and origin['operation'] == r['operation'], 'reuse identity changed')


def validate_anchors(anchors, byid, elf):
    require(anchors and len({a['id'] for a in anchors}) == len(anchors), 'duplicate/empty anchor inventory')
    for a in anchors:
        require(a['evidence_id'] in byid, 'dangling anchor evidence')
        r = byid[a['evidence_id']]
        require(a['name'] == r['parameters']['procedure'], 'anchor function differs')
        address = int(a['address'], 16)
        sym = elf.symbols[a['name']]
        require(sym['address'] <= address < sym['address']+sym['size'], 'anchor outside function')
        require(a['instruction'] in assembly(r) and a['instruction'].startswith(a['address']+':'), 'anchor instruction differs')
        raw = bytes.fromhex(a['bytes_hex'])
        require(raw and len(raw) <= sym['address']+sym['size']-address, 'anchor byte range invalid')
        offset, body = elf.locate(address, len(raw))
        require(offset == a['file_offset'] and raw == body, 'anchor bytes differ')
        if r['operation'] == 'analyze_function':
            require(any(int(s['start'], 16) <= address <= int(s['end'], 16)
                        for s in r['normalized_result']['procedure']['body']['ranges']), 'anchor outside analyzed ranges')


def validate_findings(findings, cases, questions, byid, anchors):
    ids = {f['id'] for f in findings}
    anchor_ids = {a['id'] for a in anchors}
    require(findings and len(ids) == len(findings), 'duplicate/empty findings')
    for f in findings:
        require(f['kind'] in ('observation', 'static-inference', 'design', 'unknown') and
                f['status'] in ('answered', 'bounded', 'partial', 'unresolved') and f['claim'] and f['limits'], 'invalid finding')
        require(f['evidence'] and set(f['evidence']) <= set(byid), 'dangling finding evidence')
        require(f['anchors'] and set(f['anchors']) <= anchor_ids, 'dangling finding anchor')
        require(all(a['evidence_id'] in f['evidence'] for a in anchors if a['id'] in f['anchors']), 'finding anchor provenance mismatch')
    require(cases and len({c['id'] for c in cases}) == len(cases), 'duplicate/empty cases')
    for c in cases:
        require(c['basis'] in ('static-inference', 'design', 'unresolved') and c['input'] and c['expected'] and c['boundary'], 'invalid expected case')
        require(c['findings'] and set(c['findings']) <= ids, 'dangling expected finding')
        require(c['runtime_executed'] is False, 'static case claimed runtime execution')
        if c['basis'] == 'static-inference':
            require(any(f['kind'] in ('observation', 'static-inference') for f in findings if f['id'] in c['findings']), 'static case lacks observational basis')
    require([q['id'] for q in questions] == [f'Q{i:02}' for i in range(1, 7)], 'question inventory mismatch')
    for q in questions:
        require(q['status'] in ('answered', 'partial', 'unresolved') and q['findings'] and set(q['findings']) <= ids
                and q['remaining'] and q['next_action'], 'invalid question status')
        require(not(q['status'] == 'answered' and any((f['kind'] == 'unknown' or f['status'] in ('partial', 'unresolved')) for f in findings if f['id'] in q['findings'])), 'unresolved question claimed answered')
    require(any(c['basis'] == 'unresolved' for c in cases), 'missing unresolved boundary cases')


def verify(root=ROOT):
    directory = 'docs/stage2ar-evidence/'
    data = safe_path(root, 'magic_Linux32').read_bytes()
    require(digest(data) == TARGET, 'reference target changed')
    elf = Elf(data)
    new = load(root, directory+'rea.json.gz')['records']
    reused = load(root, directory+'reused.json.gz')['records']
    require(len(new) == 3 and {r['parameters']['procedure'] for r in new} == {'vm_unrefValue','vm_unrefMapping','vm_error'}, 'new evidence scope changed')
    require(len(reused) == 50, 'reuse evidence scope changed')
    manifest = load(root, directory+'function-manifest.json')
    for m in manifest:
        group = new if m['bundle'] == 'rea.json.gz' else reused if m['bundle'] == 'reused.json.gz' else []
        require(m['evidence_id'] in {r['evidence_id'] for r in group}, 'wrong bundle attribution')
    byid = validate_records(new+reused, manifest, elf)
    validate_origins(reused, load(root, directory+'reused-manifest.json'), root)
    anchors = load(root, directory+'anchors.json')
    validate_anchors(anchors, byid, elf)
    findings = load(root, directory+'findings.json')
    cases = load(root, directory+'expected-cases.json')
    questions = load(root, directory+'questions.json')
    validate_findings(findings, cases, questions, byid, anchors)
    session = load(root, directory+'session.json')
    require(session['before']['open'] is False and session['after']['open'] is False, 'session lifecycle differs')
    exclusions = load(root, directory+'excluded-navigation.json')
    require({x['name'] for x in exclusions} == {'vm_f_bind_parameters','vm_f_bind_parameters_arr'}, 'excluded navigation differs')
    for x in exclusions:
        require(any(r['evidence_id'] == x['evidence_id'] and r['parameters']['procedure'] == x['name']
                    for r in load(root,x['source'])['records']), 'excluded navigation provenance differs')
    return dict(stage='2AR',new_records=len(new),reused_records=len(reused),unique_records=len(byid),
                functions=len({m['name'] for m in manifest}),anchors=len(anchors),findings=len(findings),
                expected_cases=len(cases),question_status={q['id']:q['status'] for q in questions},target_execution='none')


if __name__ == '__main__':
    print(json.dumps(verify(), ensure_ascii=False, sort_keys=True))
