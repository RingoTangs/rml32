"""Read-only integrity checks, not an automatic proof of semantic conclusions."""
import gzip
import importlib.util
import json
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
# Load the historical ELF helper without putting its directory ahead of this
# stage's identically named analyze_corpus module.
_spec = importlib.util.spec_from_file_location('_stage2al_capture_native', ROOT / 'docs/stage2al-evidence/capture_native.py')
_native = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_native)
Elf, TARGET = _native.Elf, _native.TARGET
canonical, digest, require = _native.canonical, _native.digest, _native.require

NEW_FUNCTIONS = {'vm_callInEfun', 'vm_callInUserFunBody', 'vm_finalizeFun'}
REUSED_FUNCTIONS = {'vm_freeFunctionValue', 'vm_pushValue', 'vm_copyValue', 'vm_callFunPtr',
                    'vm_cleanupValue', 'vm_createFunctionValueEx', 'vm_doActions',
                    'vm_traceError', 'vm_createFunctionValue', 'vm_errorQuit',
                    'vm_returnFromFun', 'vm_getArgument', 'vm_returnFromEfun',
                    'vm_addEfuns', 'vm_finalizeIdentifiersInProgram'}


def safe_path(root, name):
    p = PurePosixPath(name)
    require(bool(name) and not p.is_absolute() and '..' not in p.parts and '\\' not in name,
            'unsafe source path')
    path = root / name
    require(path.resolve().is_relative_to(root.resolve()), 'source escapes root')
    return path


def load(root, name):
    raw = safe_path(root, name).read_bytes()
    return json.loads(gzip.decompress(raw) if name.endswith('.gz') else raw)


def validate_records(records, manifest, elf):
    ids = [r['evidence_id'] for r in records]
    require(len(ids) == len(set(ids)), 'duplicate Evidence ID')
    require(len(manifest) == len(records) and {m['evidence_id'] for m in manifest} == set(ids),
            'manifest partition mismatch')
    byid = {r['evidence_id']: r for r in records}
    for m in manifest:
        r = byid[m['evidence_id']]
        require(r['subject']['digest']['sha256'] == TARGET and r['subject']['format'] == 'elf'
                and r['subject']['architecture'] == 'x86', 'wrong target')
        require(r['provider']['id'] == 'ghidra' and r['provider']['version'] == '12.1.4', 'wrong provider')
        require(m['record_sha256'] == digest(canonical(r)), 'record digest differs')
        require(r['parameters']['procedure'] == m['name'] and r['operation'] == m['operation'],
                'procedure or operation differs')
        sym = elf.symbols[m['name']]
        require(int(m['address'], 16) == sym['address'] and m['size'] == sym['size'], 'ELF symbol differs')
        off, raw = elf.locate(sym['address'], sym['size'])
        require(off == m['file_offset'] and digest(raw) == m['body_sha256'], 'ELF body differs')
        require(m['limitations'] == r['limitations'] and r['raw_result'] is not None
                and r['normalized_result'], 'missing record or limitations')
        n = r['normalized_result']
        if r['operation'] == 'analyze_function':
            require(int(n['procedure']['address'], 16) == sym['address']
                    and n['procedure']['name'] == m['name'], 'dossier identity differs')
            require(n['pseudocode'] and n['assembly'] and n['procedure']['body']['available'], 'incomplete dossier')
            # A dossier must include its entry, not only an unrelated function's text.
            require(int(n['assembly'][0].split(':', 1)[0], 16) == sym['address'], 'assembly entry differs')
    return byid


def validate_origins(reused, origins, root):
    byid = {r['evidence_id']: r for r in reused}
    require(len(origins) == len(reused) and {r['evidence_id'] for r in origins} == set(byid),
            'reuse manifest differs')
    for origin in origins:
        raw = safe_path(root, origin['source']).read_bytes()
        require(digest(raw) == origin['source_sha256'], 'reuse source changed')
        record = byid[origin['evidence_id']]
        require(record in json.loads(gzip.decompress(raw))['records']
                and digest(canonical(record)) == origin['record_sha256'], 'reuse record changed')
        require(origin['name'] == record['parameters']['procedure']
                and origin['operation'] == record['operation'], 'reuse identity changed')


def validate_anchors(anchors, byid, elf):
    require(anchors and len({(a['evidence_id'], a['address']) for a in anchors}) == len(anchors),
            'missing or duplicate anchors')
    for a in anchors:
        record = byid[a['evidence_id']]
        address = int(a['address'], 16)
        sym = elf.symbols[a['name']]
        require(record['parameters']['procedure'] == a['name']
                and sym['address'] <= address < sym['address'] + sym['size'], 'anchor outside function')
        n = record['normalized_result']
        assembly = n['assembly'] if isinstance(n, dict) else n.splitlines()
        require(a['instruction'] in assembly and a['instruction'].startswith(a['address'] + ':'),
                'anchor instruction differs')
        raw = bytes.fromhex(a['bytes_hex'])
        require(bool(raw) and len(raw) <= sym['address'] + sym['size'] - address,
                'anchor byte range invalid')
        off, expected = elf.locate(address, len(raw))
        require(off == a['file_offset'] and raw == expected, 'anchor bytes differ')


def validate_findings(findings, cases, byid):
    ids = {f['id'] for f in findings}
    require(len(ids) == len(findings) and ids, 'duplicate or empty findings')
    for f in findings:
        require(f['kind'] in ('observation', 'static-inference', 'design', 'unknown')
                and f['status'] in ('answered', 'bounded', 'partial', 'unresolved')
                and f['claim'] and f['limits'] and f['evidence'], 'invalid finding')
        require(set(f['evidence']) <= set(byid), 'dangling evidence')
    require(cases and len({c['id'] for c in cases}) == len(cases), 'duplicate or empty cases')
    for c in cases:
        require(c['basis'] in ('static-inference', 'design', 'unresolved') and c['input'] and c['expected']
                and c['findings'] and set(c['findings']) <= ids, 'invalid expected case')


def verify(root=ROOT):
    directory = 'docs/stage2an-evidence/'
    data = (root / 'magic_Linux32').read_bytes()
    require(digest(data) == TARGET, 'target bytes changed')
    elf = Elf(data)
    protected = load(root, directory + 'protected-inputs.json')
    require(protected and 'src/vm.rs' in protected and 'magic_Linux32' in protected, 'protected inventory missing')
    for name, expected in protected.items():
        require(digest(safe_path(root, name).read_bytes()) == expected, 'protected input changed: ' + name)
    new = load(root, directory + 'rea.json.gz')['records']
    reused = load(root, directory + 'reused.json.gz')['records']
    require(len(new) == 3 and {r['parameters']['procedure'] for r in new} == NEW_FUNCTIONS, 'new function scope differs')
    require(len(reused) == 17 and {r['parameters']['procedure'] for r in reused} == REUSED_FUNCTIONS, 'reuse scope differs')
    manifest = load(root, directory + 'function-manifest.json')
    for m in manifest:
        require(m['bundle'] in ('rea.json.gz', 'reused.json.gz'), 'unknown bundle')
        require(m['evidence_id'] in {r['evidence_id'] for r in (new if m['bundle'] == 'rea.json.gz' else reused)},
                'wrong bundle attribution')
    byid = validate_records(new + reused, manifest, elf)
    validate_origins(reused, load(root, directory + 'reused-manifest.json'), root)
    validate_anchors(load(root, directory + 'anchors.json'), byid, elf)
    validate_findings(load(root, directory + 'findings.json'), load(root, directory + 'expected-cases.json'), byid)
    return dict(new_session_records=len(new), reused_records=len(reused),
                protected_files=len(protected), unique_records=len(byid))


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
