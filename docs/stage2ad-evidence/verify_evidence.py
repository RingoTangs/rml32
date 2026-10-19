"""Verify retained evidence identity, provenance, manifest and selected instructions.

This checks integrity and specific observations, not behavioral equivalence.
"""
import csv
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
PROVIDER = dict(id='ghidra', name='Ghidra', version='12.1.4')
NEW = {'vm_returnFromEfun': '0x8093c5c', 'vm_getArgument': '0x808a364',
       'vm_f_abs': '0x80d453c', 'vm_checkArgNum': '0x809521c',
       'vm_stackInThisContext': '0x80933e4', 'vm_clearStack': '0x808aab0',
       'vm_popNStack': '0x8091ab0', 'vm_f_time': '0x80dd030'}
REUSED = {('vm_callInEfun', 'analyze_function'): '0x8093d98',
          ('vm_addEfuns', 'analyze_function'): '0x8085ce4',
          ('vm_regularProcessor', 'analyze_function'): '0x80978c4',
          ('vm_error', 'procedure_pseudo_code'): '0x8097b24',
          ('vm_errorQuit', 'procedure_pseudo_code'): '0x80ab104',
          ('vm_errorQuit', 'procedure_assembly'): '0x80ab104',
          ('vm_traceError', 'procedure_pseudo_code'): '0x8093520',
          ('vm_traceError', 'procedure_assembly'): '0x8093520',
          ('vm_doActions', 'procedure_pseudo_code'): '0x8097008',
          ('vm_doActions', 'procedure_assembly'): '0x8097008'}
SEARCHES = {
    '(Efun|efun|Argument|argument|popArgs|pushReturn|checkArg)',
    '(stackInThisContext|argumentInThisContext|pop|pushInt|pushReal|setCurrentArgument|clearStack)',
}
SITES = {
    'vm_regularProcessor': {
        '0x8097986': 'MOVSX EAX, byte ptr [EBX + 0x2]',
        '0x809798d': 'PUSH EAX',
        '0x809798e': 'MOVSX EAX, word ptr [EBX]',
        '0x8097991': 'PUSH EAX',
        '0x8097992': 'CALL dword ptr [EDX + 0x8]',
    },
    'vm_callInEfun': {
        '0x8093dfd': 'MOV ECX, 0xa', '0x8093e04': 'MOVSD.REP ES:EDI, ESI',
        '0x8093e0b': 'MOV dword ptr [EDX], EAX',
        '0x8093e10': 'MOV dword ptr [EDX + 0x4], EAX',
        '0x8093e1a': 'MOV dword ptr [EDX + 0x8], 0x1',
        '0x8093e26': 'CALL dword ptr [EBX + 0x14]',
        '0x8093e32': 'TEST AL, 0x10', '0x8093e36': 'TEST AL, 0x4',
        '0x8093e83': 'OR dword ptr [EAX + 0x28], 0x40',
    },
    'vm_returnFromEfun': {
        '0x8093c77': 'ADD EDX, dword ptr [ESI + 0x4]',
        '0x8093c7e': 'SUB EDX, dword ptr [0x086619cc]',
        '0x8093c84': 'CMP EBX, 0x1', '0x8093c8d': 'CMP EBX, 0xff',
        '0x8093cc7': 'CMP EAX, EBX',
        '0x8093d10': 'DEC dword ptr [0x086619c0]',
        '0x8093d16': 'CALL 0x0809789c', '0x8093d4c': 'CALL 0x08090fc0',
    },
    'vm_getArgument': {'0x808a373': 'CALL 0x080933e4',
                       '0x808a378': 'LEA EBX, [EAX + EDI*0x1]',
                       '0x808a3ba': 'CMP ESI, 0xff'},
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    data = path.read_bytes()
    return json.loads(gzip.decompress(data) if path.suffix == '.gz' else data)


def source_path(root, relative):
    require(isinstance(relative, str) and not Path(relative).is_absolute(), 'source must be relative')
    path = (root / relative).resolve()
    require(path.is_relative_to(root.resolve()), 'source escapes repository')
    return path


def verify(root=ROOT, here=OUT):
    root, here = Path(root), Path(here)
    require(digest(root / 'magic_Linux32') == TARGET, 'target hash')
    fresh, reused = read(here / 'rea.json.gz'), read(here / 'reused.json.gz')
    require(len(fresh['records']) == 10 and len(reused['records']) == 10, 'record counts')
    records, provenance = {}, {}
    for bundle in (fresh, reused):
        require(bundle['providers'] == [PROVIDER], 'bundle provider')
        require(len(bundle['artifacts']) == 1 and
                bundle['artifacts'][0]['digest']['sha256'] == TARGET, 'bundle target')
        for record in bundle['records']:
            eid = record['evidence_id']
            require(eid not in records, 'duplicate evidence id')
            require(record['provider'] == PROVIDER and record['subject']['digest']['sha256'] == TARGET,
                    'record provider or target')
            require(record['authority'] == 'shipped-artifact', 'record authority')
            records[eid] = record
    expected = {(name, 'analyze_function') for name in NEW}
    actual = {(r['parameters'].get('procedure'), r['operation']) for r in fresh['records']
              if r['operation'] != 'search_procedures'}
    require(actual == expected, 'new procedure inventory')
    searches = [r for r in fresh['records'] if r['operation'] == 'search_procedures']
    require({r['parameters']['pattern'] for r in searches} == SEARCHES and
            all(r['parameters'] == dict(case_sensitive=False, mode='regex', pattern=r['parameters']['pattern'])
                for r in searches), 'navigation search boundary')
    require({(r['parameters'].get('procedure'), r['operation']) for r in reused['records']} == set(REUSED),
            'reused procedure inventory')
    for record in fresh['records']:
        provenance[record['evidence_id']] = ('docs/stage2ad-evidence/rea.json.gz', digest(here / 'rea.json.gz'))
    require(len(reused['sources']) == 4, 'source count')
    require(len({s['path'] for s in reused['sources']}) == 4, 'duplicate source')
    for source in reused['sources']:
        path = source_path(root, source['path'])
        require(digest(path) == source['sha256'], 'original bundle hash')
        original = {r['evidence_id']: r for r in read(path)['records']}
        for eid in source['evidence_ids']:
            require(eid not in provenance and eid in records and eid in original, 'source evidence membership')
            require(records[eid] == original[eid], 'edited reused record')
            provenance[eid] = (source['path'], source['sha256'])
    require(set(provenance) == set(records), 'missing provenance')
    manifest = read(here / 'function-manifest.json')
    require(len(manifest) == 20 and {m['evidence_id'] for m in manifest} == set(records), 'manifest bijection')
    for item in manifest:
        record = records[item['evidence_id']]
        name, operation = record['parameters'].get('procedure'), record['operation']
        require(item['name'] == name and item['operation'] == operation and
                item['parameters'] == record['parameters'], 'manifest identity')
        require(item['limitations'] == record['limitations'], 'manifest limitations')
        require((item['source'], item['source_sha256']) == provenance[item['evidence_id']], 'manifest provenance')
        address = NEW.get(name) if item['source'].endswith('stage2ad-evidence/rea.json.gz') else REUSED.get((name, operation))
        require(item['address'] == address, 'manifest address')
        if operation == 'analyze_function':
            procedure = record['normalized_result']['procedure']
            require(procedure['address'] == address and item['body'] == procedure['body'], 'procedure body/address')
            require(procedure['body']['available'] and procedure['body']['contains_entry'], 'unavailable function body')
        elif operation == 'procedure_assembly':
            require(record['normalized_result'].splitlines()[0].startswith(address + ':'), 'assembly entry')
    for name, sites in SITES.items():
        record = next(r for r in records.values() if r['parameters'].get('procedure') == name
                      and r['operation'] == 'analyze_function')
        lines = record['normalized_result']['assembly']
        listing = dict(line.split(': ', 1) for line in lines)
        require(len(listing) == len(lines), 'duplicate instruction address')
        for address, instruction in sites.items():
            require(listing.get(address) == instruction, f'instruction site {name} {address}')
    with (root / 'docs/evidence/actions.csv').open(newline='') as stream:
        action = next(r for r in csv.DictReader(stream) if int(r['opcode'], 16) == 6)
    require(action['name'] == 'vm_callInEfun' and int(action['entry'], 16) == 0x8093d98
            and action['parameters'] == '2', '06 action binding')
    findings = read(here / 'findings.json')
    require(len({f['id'] for f in findings}) == len(findings), 'duplicate finding id')
    for finding in findings:
        require(finding['classification'] in ('observation', 'static-inference', 'design', 'unknown'), 'finding classification')
        require(set(finding['evidence_ids']) <= set(records), 'unknown finding evidence')
        if finding['classification'] in ('observation', 'static-inference'):
            require(bool(finding['evidence_ids']), 'unsubstantiated finding')
    view = (here / 'functions.md').read_text()
    require(all(view.count(f"Evidence: `{eid}`") == 1 for eid in records), 'readable view evidence coverage')
    return dict(records=len(records), new_records=10, reused_records=10,
                instruction_sites=sum(map(len, SITES.values())), findings=len(findings),
                limitation='Integrity and selected instructions do not prove runtime compatibility.')


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
