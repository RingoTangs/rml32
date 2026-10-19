"""Retained evidence integrity and selected assembly checks; not execution equivalence."""
import csv
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
PROVIDER = dict(id='ghidra',name='Ghidra',version='12.1.4')
EXPECTED = {('vm_callFunPtr', 'analyze_function'): '0x809441c',
 ('vm_callInEfun', 'analyze_function'): '0x8093d98',
 ('vm_callInUserFunBody', 'procedure_pseudo_code'): '0x8093eb0',
 ('vm_cleanupValue', 'analyze_function'): '0x808ad98',
 ('vm_copyValue', 'analyze_function'): '0x808be24',
 ('vm_createFunctionValue', 'analyze_function'): '0x8091f34',
 ('vm_createFunctionValueEx', 'analyze_function'): '0x8091cd0',
 ('vm_doActions', 'procedure_pseudo_code'): '0x8097008',
 ('vm_eqValue', 'analyze_function'): '0x808eed4',
 ('vm_errorQuit', 'procedure_pseudo_code'): '0x80ab104',
 ('vm_f_bind_parameters', 'analyze_function'): '0x80df374',
 ('vm_f_bind_parameters_arr', 'analyze_function'): '0x80df310',
 ('vm_freeFunctionValue', 'analyze_function'): '0x808c114',
 ('vm_isEq', 'analyze_function'): '0x808c908',
 ('vm_isZero', 'procedure_pseudo_code'): '0x808cad4',
 ('vm_pushValue', 'analyze_function'): '0x8091074',
 ('vm_regularProcessor', 'analyze_function'): '0x80978c4',
 ('vm_returnFromFun', 'procedure_assembly'): '0x8094978',
 ('vm_returnFromFun', 'procedure_pseudo_code'): '0x8094978',
 ('vm_traceError', 'procedure_assembly'): '0x8093520',
 ('vm_traceError', 'procedure_pseudo_code'): '0x8093520'}

SITES = {
 'vm_createFunctionValue': {'0x8091f42':'SHR EDX, 0x6','0x8091f4a':'SHR ECX, 0x7',
    '0x8091f5a':'AND EDI, 0x3f','0x8091f71':'ADD SI, word ptr [EAX + 0x6]'},
 'vm_createFunctionValueEx': {'0x8091d0a':'CMP byte ptr [EDX + EAX*0x8 + 0x4], 0x8',
    '0x8091d9f':'MOV dword ptr [ECX + 0x10], EDI','0x8091da2':'MOV dword ptr [ECX], 0x1',
    '0x8091dc4':'MOV dword ptr [EAX + 0xc], EDX',
    '0x8091de4':'MOV dword ptr [ECX + 0x14], EAX',
    '0x8091e0b':'MOV byte ptr [EDX + EAX*0x8 + 0x4], 0x9',
    '0x8091ed2':'INC dword ptr [ECX]'},
 'vm_callFunPtr': {'0x809442d':'JLE 0x0809476b','0x809445f':'DEC dword ptr [EBP + 0xc]',
    '0x809449d':'CMP EAX, 0x9','0x8094529':'ADD dword ptr [EBP + 0xc], EDI',
    '0x809452c':'CMP dword ptr [ESI], 0x1','0x80945b6':'CALL 0x0808c114',
    '0x80945d1':'CMP dword ptr [EBP + 0xffffff74], 0x4b',
    '0x80946a3':'CALL 0x08093eb0','0x8094736':'CALL 0x08091074','0x809483f':'CALL 0x08093d98'},
 'vm_copyValue': {'0x808beb6':'MOV EDX, dword ptr [EAX + 0xc]',
    '0x808bebd':'TEST byte ptr [EDX + 0xd], 0x80','0x808bec6':'INC dword ptr [EAX]'},
 'vm_freeFunctionValue': {'0x808c167':'CALL 0x0808ad98','0x808c1ef':'CALL 0x080b1618'},
}


def require(condition,message):
    if not condition:raise ValueError(message)


def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    data=path.read_bytes()
    return json.loads(gzip.decompress(data) if path.suffix=='.gz' else data)


def source_path(root,name):
    require(isinstance(name,str) and not Path(name).is_absolute(),'relative source required')
    path=(root/name).resolve()
    require(path.is_relative_to(root.resolve()),'source escapes repository')
    return path


def verify(root=ROOT,here=OUT):
    root,here=Path(root),Path(here)
    require(digest(root/'magic_Linux32')==TARGET,'target hash')
    fresh,reused=read(here/'rea.json.gz'),read(here/'reused.json.gz')
    require(len(fresh['records'])==10 and len(reused['records'])==11,'record counts')
    records,provenance={},{}
    for b in (fresh,reused):
        require(b['providers']==[PROVIDER],'bundle provider')
        require(len(b['artifacts'])==1 and b['artifacts'][0]['digest']['sha256']==TARGET,'bundle target')
        for r in b['records']:
            eid=r['evidence_id']
            require(eid not in records,'duplicate evidence id')
            require(r['provider']==PROVIDER and r['subject']['digest']['sha256']==TARGET,'record target/provider')
            require(r['authority']=='shipped-artifact','record authority')
            records[eid]=r
    require({(r['parameters'].get('procedure'),r['operation']) for r in records.values()}==set(EXPECTED),'procedure inventory')
    for r in fresh['records']:
        provenance[r['evidence_id']]=('docs/stage2af-evidence/rea.json.gz',digest(here/'rea.json.gz'))
    require(len(reused['sources'])==5 and len({s['path'] for s in reused['sources']})==5,'source count/uniqueness')
    for source in reused['sources']:
        path=source_path(root,source['path'])
        require(digest(path)==source['sha256'],'source hash')
        original={r['evidence_id']:r for r in read(path)['records']}
        for eid in source['evidence_ids']:
            require(eid not in provenance and eid in records and eid in original,'source evidence membership')
            require(records[eid]==original[eid],'edited reused record')
            provenance[eid]=(source['path'],source['sha256'])
    require(set(provenance)==set(records),'missing provenance')
    manifest=read(here/'function-manifest.json')
    require(len(manifest)==21 and {m['evidence_id'] for m in manifest}==set(records),'manifest bijection')
    for m in manifest:
        r=records[m['evidence_id']];key=(r['parameters']['procedure'],r['operation'])
        require((m['name'],m['operation'])==key and m['parameters']==r['parameters'],'manifest identity')
        require(m['limitations']==r['limitations'],'manifest limitations')
        require((m['source'],m['source_sha256'])==provenance[m['evidence_id']],'manifest provenance')
        require(m['address']==EXPECTED[key],'manifest address')
        if r['operation']=='analyze_function':
            proc=r['normalized_result']['procedure']
            require(proc['address']==m['address'] and proc['body']==m['body'],'procedure body/address')
            require(proc['body']['available'] and proc['body']['contains_entry'],'unavailable procedure body')
        elif r['operation']=='procedure_assembly':
            require(r['normalized_result'].splitlines()[0].startswith(m['address']+':'),'assembly entry')
    for name,sites in SITES.items():
        r=next(r for r in records.values() if r['parameters']['procedure']==name and r['operation']=='analyze_function')
        lines=r['normalized_result']['assembly'];listing=dict(line.split(': ',1) for line in lines)
        require(len(listing)==len(lines),'duplicate instruction address')
        for address,insn in sites.items():require(listing.get(address)==insn,f'instruction site {name} {address}')
    with (root/'docs/evidence/actions.csv').open(newline='') as f:
        actions={int(r['opcode'],16):r for r in csv.DictReader(f)}
    for opcode,name,address in ((5,'vm_callFunPtr',0x809441c),(23,'vm_createFunctionValue',0x8091f34)):
        r=actions[opcode]
        require(r['name']==name and int(r['entry'],16)==address and r['parameters']=='2','opcode action binding')
    findings=read(here/'findings.json')
    require(len({f['id'] for f in findings})==len(findings),'duplicate finding id')
    for f in findings:
        require(f['classification'] in ('observation','static-inference','design','unknown'),'finding classification')
        require(f['status'] in ('answered','partial','out_of_scope'),'finding status')
        require(set(f['evidence_ids'])<=set(records),'unknown finding evidence')
        if f['classification'] in ('observation','static-inference'):require(bool(f['evidence_ids']),'unsubstantiated finding')
    text=(here/'functions.md').read_text()
    require(all(text.count(f"Evidence: `{eid}`")==1 for eid in records),'readable evidence coverage')
    return dict(records=len(records),new_records=10,reused_records=11,instruction_sites=sum(map(len,SITES.values())),
                findings=len(findings),limitation='Integrity and selected instruction checks, not runtime equivalence.')


if __name__=='__main__':print(json.dumps(verify(),sort_keys=True))
