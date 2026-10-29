"""Execute or independently verify the bounded real-startup delivery receipt."""
import argparse
from collections import Counter
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
RECORD=ROOT/'docs/stage2bo-validation.json'
ALLOWED={'Cargo.toml','src/boundary.rs','src/lib.rs','src/prepared.rs','src/vm.rs','src/host.rs',
         'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW_SOURCE={'src/startup_probe.rs','src/startup_probe_main.rs','src/startup_probe_tests.rs',
            'src/startup_vm_tests.rs','tests/startup_probe.rs'}
OUTPUTS={'reference-current-log.json.gz','pair-current-log.json.gz','public-current-log.json.gz',
         'current-replay.json','corpus-validation.json','probe-observations.json.gz','validation-log.json.gz'}
SERVICES={
    'aaa':(105,450136,661,'f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e'),
    'ccs':(102,557463,667,'256301c54d7c2b8bdf50aeded93c27325dad80690a2f488249d04acfe3f59b47'),
    'csa':(98,500201,639,'9d73f41b25887221e015373f262945bd907a33af16785e5ea26fcdbad7df38a1'),
    'dba':(98,500354,639,'5fd2005c124be25643ebe19169ce2a3d798fbbde106efe1aa8b36287e0aebcd8'),
    'gs':(359,1793584,4368,'2f3ce58bb54584d2ce7bb3d90852893b97fcd5fb55d455b8eccff856fc696174'),
}
GATES=dict(real_startup_prefix_ready=True,runtime_changed=True,public_api_added=True,
           dependencies_changed=False,quota_semantics_changed=False,existing_models_changed=False,
           service_startup_complete=False,implementation_ready=False,stage2_complete=False,
           native_equivalence_proven=False,reference_elf_execution=False,reference_scripts_execution=False,
           real_bytecode_execution='five_fixed_global_init_prefixes',rust32_execution='not_run',
           native_unknowns=20,native_unknowns_promoted=0)

def require(ok,message):
    if not ok:raise AssertionError(message)

def digest(raw):return hashlib.sha256(raw).hexdigest()
def encoded(v):return (json.dumps(v,sort_keys=True,indent=2,ensure_ascii=False)+'\n').encode()
def load(p):
    b=p.read_bytes();return json.loads(gzip.decompress(b) if p.suffix=='.gz' else b)
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec)
    sys.modules[name]=m;spec.loader.exec_module(m);return m

def boundary():
    base=load(HERE/'baseline.json')
    require(base['commit']=='c7a7d3b0cdb4634f18e9772493bad440921ca7bb' and len(base['files'])==1243,'baseline identity')
    require(digest(subprocess.check_output(['git','ls-files','--stage','-z'],cwd=ROOT))==base['index_sha256'],'index changed')
    projection=module('_2bo_projection',ROOT/'docs/stage2bg-host-function-test/source_projection.py')
    changes=[];bodies=0
    for name,h in base['files'].items():
        current=(ROOT/name).read_bytes()
        if digest(current)==h:continue
        require(name in ALLOWED,'protected file changed: '+name);changes.append(name)
        old=subprocess.check_output(['git','show',base['commit']+':'+name],cwd=ROOT)
        require(digest(old)==h,'changed-file baseline bytes: '+name)
        if name in ('README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'):
            require(current.endswith(old),'historical document rewritten: '+name)
        if name.startswith('src/'):
            before=Counter(tuple(b) for b in projection.test_bodies(old.decode()))
            after=Counter(tuple(b) for b in projection.test_bodies(current.decode()))
            require(before<=after,'old Rust test body changed: '+name);bodies+=sum(before.values())
    expected={n for n in base['files'] if n.startswith(('src/','tests/'))}|NEW_SOURCE
    actual=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT,text=True).splitlines())
    require(actual==expected,'source inventory changed')
    return dict(baseline_commit=base['commit'],baseline_files=1243,byte_frozen_files=1243-len(changes),
                changed_files=sorted(changes),old_test_bodies_preserved=bodies,index_unchanged=True)

def artifacts():
    files=ALLOWED|NEW_SOURCE
    files|={str(p.relative_to(ROOT)) for p in HERE.rglob('*') if p.is_file()
            and '__pycache__' not in p.parts and p.name!='validation-log.json.gz'}
    return {p:digest((ROOT/p).read_bytes()) for p in sorted(files)}

def validate_probe(name,r):
    slots,offset,total,sha=SERVICES[name]
    require(r['source']==f'{name}/pack_data/lib_{name}32.pak/{name}/start_{name}.o' and r['sha256']==sha,'probe identity')
    require(r['function_base']==572 and r['original_init_instructions']==total,'original Global structure')
    require((r['executed_instructions'],r['stop_pc'],r['stop_offset'])==(25,25,offset+100),'probe execution boundary')
    require(r['stop_reason']=='prefix_boundary' and r['initialization_complete'] is False,'prefix completion scope')
    undefined={'type':'undefined'}
    expected=[undefined for _ in range(slots)]
    expected[2]={'type':'array','elements':[undefined for _ in range(1024)]}
    expected[3]={'type':'int','value':2};expected[4]={'type':'int','value':0}
    expected[5]={'type':'bytes','bytes_hex':'2f786462632f'}
    expected[6]={'type':'mapping','entries':[]};expected[7]={'type':'int','value':0}
    require(r['globals']==expected,'Global values differ')
    require(r['host_calls']==[dict(slot=0,name='allocate_array',pc=1,offset=offset+4,argument=1024)],'efun call inventory')
    require(r['next_dependency']==dict(slot=336,name='mkdir',pc=26,offset=offset+104),'next dependency')
    require(r['configuration']==dict(values=dict(max_host_roots=32,max_graph_nodes=8192,max_depth=16,max_items=4096),
        vm=dict(max_stack=64,max_frames=2,max_reference_cells=64),lifecycle=dict(max_objects=0,max_function_heads=0,
        max_ordinary_refs_per_object=2,max_exec_refs_per_object=1,max_owners_per_header=8)),'probe resource configuration')

def validate_native(bundle):
    require(len(bundle['records'])==3 and not bundle['unknowns'] and not bundle['captures'],'native scope')
    expected={'vm_f_allocate_array':'0x80d58b4','vm_safeAllocateEmptyArray':'0x809b0e8','vm_safeNewValue':'0x8089f78'}
    require({r['parameters']['procedure'] for r in bundle['records']}==set(expected),'native procedure inventory')
    for r in bundle['records']:
        require(r['authority']=='shipped-artifact' and r['operation']=='analyze_function','native evidence authority')
        require(r['subject']['digest']['sha256']==digest((ROOT/'magic_Linux32').read_bytes()),'native artifact identity')
        n=r['parameters']['procedure'];v=r['normalized_result']
        require(v['procedure']['address']==expected[n] and v['assembly'] and v['pseudocode'],'native body inventory')
    return dict(records=3,reference_execution=False,evidence_ids=sorted(r['evidence_id'] for r in bundle['records']))

def snapshot():
    """Recreate the original working-tree/index authority of the 2BN receipt."""
    base=load(HERE/'baseline.json');p=ROOT/'target/stage2bo-baseline'
    if not p.exists():
        subprocess.run(['git','clone','--local','--no-hardlinks','--no-checkout',str(ROOT),str(p)],check=True,capture_output=True)
        subprocess.run(['git','checkout','--detach',base['commit']],cwd=p,check=True,capture_output=True)
        subprocess.run(['git','read-tree','13de257'],cwd=p,check=True,capture_output=True)
    restored=[]
    for n,h in base['files'].items():
        q=p/n;raw=q.read_bytes()
        if digest(raw)==h:continue
        # Git checkout may convert only recorded LF/CRLF identities. Never copy
        # a changed production source from the current working tree.
        variants=[raw,raw.replace(b'\r\n',b'\n'),raw.replace(b'\r\n',b'\n').replace(b'\n',b'\r\n')]
        match=next((b for b in variants if digest(b)==h),None)
        require(match is not None,'historical snapshot drift: '+n)
        q.write_bytes(match);restored.append(n)
    old=load(p/'docs/stage2bn-pair-audit/baseline-inputs.json')
    require(digest(subprocess.check_output(['git','ls-files','--stage','-z'],cwd=p))==old['index_sha256'],'historical index identity')
    return p

PREFIX='docs/stage2bo-startup-probe/'
COMMANDS=[
    ('target/stage2bo-baseline',['python3','-B','docs/stage2bn-pair-audit/verify_validation.py']),
    ('.',['cargo','test','--locked','--offline']),
    ('.',['cargo','test','--release','--locked','--offline']),
    ('.',['cargo','build','--locked','--offline']),
    ('.',['cargo','build','--release','--locked','--offline']),
    ('.',['cargo','fmt','--check']),
    ('.',['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings']),
    ('.',['python3','tests/compare_corpus.py','--report','target/stage2bo-corpus-validation.json']),
    ('.',['python3','-B',PREFIX+'replay_current.py','--write']),
    ('.',['python3','-B','-m','unittest','discover','-s',PREFIX.rstrip('/'),'-p','test_*.py']),
    ('.',['rustc','-Vv']),
    ('.',['rustup','target','list','--installed']),
]+[('.',[f'target/{profile}/rml32-startup-probe','--service',name,'--root','.'])
   for profile in ('debug','release') for name in SERVICES]
# Materialize authenticated observations before testing their independent validator.
COMMANDS.append(COMMANDS.pop(9))

def validate_tests(text):
    names=re.findall(r'^test (\S+) \.\.\. ok$',text,re.M)
    old=set(load(ROOT/'docs/stage2bn-validation.json')['current_test_names'])
    require(len(names)==len(set(names))==908 and old<=set(names) and len(set(names)-old)==16,'ordinary Rust test inventory')
    docs=re.findall(r'^test (.+ \(line \d+\).+) \.\.\. ok$',text,re.M)
    require(len(docs)==len(set(docs))==5,'doctest inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',text,re.M)
    require(len(ignored)==len(set(ignored))==17,'ignored inventory')
    outcomes=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)
    require(sum(int(x[0]) for x in outcomes)==913 and all(x[1]=='0' for x in outcomes),'Rust aggregate results')
    return sorted(names)

def validate_commands(logs):
    require([(c['cwd'],c['argv']) for c in logs]==COMMANDS,'command inventory')
    require(all(c['returncode']==0 for c in logs),'failed command')
    history=json.loads(logs[0]['stdout'])
    require(history['stage']=='2BN' and history['status']=='pass' and history['rust_tests_debug']==892,'historical receipt result')
    names=validate_tests(logs[1]['stdout']+logs[1]['stderr'])
    require(names==validate_tests(logs[2]['stdout']+logs[2]['stderr']),'profile Rust inventory')
    require('Ran 8 tests' in logs[21]['stdout']+logs[21]['stderr'] and (logs[21]['stdout']+logs[21]['stderr']).rstrip().endswith('OK'),'Python test inventory')
    require('host: aarch64-apple-darwin' in logs[9]['stdout'] and logs[10]['stdout'].splitlines()==['aarch64-apple-darwin'],'host/target inventory')
    observed=load(HERE/'probe-observations.json.gz')
    for profile,block in zip(('debug','release'),(logs[11:16],logs[16:21])):
        for name,c in zip(SERVICES,block):validate_probe(name,json.loads(c['stdout']))
        require(observed[profile]=={name:json.loads(c['stdout']) for name,c in zip(SERVICES,block)},'saved startup observations')
    require([c['stdout'] for c in logs[11:16]]==[c['stdout'] for c in logs[16:21]],'profile raw startup observations differ')
    return names

def build(logs):
    names=validate_commands(logs)
    replay=module('_2bo_replay',HERE/'replay_current.py').verify()
    require(json.loads(logs[8]['stdout'])==replay,'current replay command result')
    corpus=load(ROOT/'target/stage2bo-corpus-validation.json') if (ROOT/'target/stage2bo-corpus-validation.json').exists() else load(HERE/'corpus-validation.json')
    require(corpus==load(HERE/'corpus-validation.json'),'saved corpus result')
    return dict(stage='2BO',status='pass',boundary=boundary(),native=validate_native(load(HERE/'native-evidence.json.gz')),
                rust_tests_debug=908,rust_tests_release=908,old_rust_tests=892,new_rust_tests=16,
                doctests_each_profile=5,ignored_drivers_each_profile=17,current_python_tests=8,
                current_test_names=names,current_replay=replay,historical_run_on_current_source=False,
                real_services=list(SERVICES),prefix_end_pc=25,first_external_dependency=dict(pc=26,slot=336,name='mkdir'),
                gates=GATES,corpus=corpus,artifact_sha256=artifacts())

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--write',action='store_true');args=parser.parse_args()
    if args.write:
        snapshot();before={n:h for n,h in artifacts().items() if Path(n).name not in OUTPUTS}
        logs=[];observed={'debug':{},'release':{}}
        for cwd,argv in COMMANDS:
            print('Validating: '+' '.join(argv),file=sys.stderr,flush=True)
            r=subprocess.run(argv,cwd=ROOT/cwd,text=True,capture_output=True)
            logs.append(dict(cwd=cwd,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
            require(r.returncode==0,r.stdout+r.stderr)
            if argv[:2]==['python3','tests/compare_corpus.py']:
                (HERE/'corpus-validation.json').write_bytes(encoded(load(ROOT/'target/stage2bo-corpus-validation.json')))
            if argv[0].endswith('/rml32-startup-probe'):
                profile=argv[0].split('/')[1];name=argv[2]
                observed[profile][name]=json.loads(r.stdout)
                if sum(map(len,observed.values()))==10:
                    (HERE/'probe-observations.json.gz').write_bytes(gzip.compress(encoded(observed),mtime=0))
        # All source, documents and evidence are stable before receipt capture;
        # the corpus result is produced by its recorded command.
        after=artifacts();require(all(after.get(n)==h for n,h in before.items()),'artifacts changed during validation')
        raw=gzip.compress(encoded(logs),mtime=0)
        record=build(logs);record['log_sha256']=digest(raw)
        (HERE/'validation-log.json.gz').write_bytes(raw);RECORD.write_bytes(encoded(record))
    else:
        record=load(RECORD);raw=(HERE/'validation-log.json.gz').read_bytes()
        require(digest(raw)==record['log_sha256'],'validation log identity')
        expected=build(json.loads(gzip.decompress(raw)));expected['log_sha256']=digest(raw)
        require(record==expected,'validation receipt differs')
    print(json.dumps({k:record[k] for k in ('stage','status','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_python_tests','real_services','gates')},sort_keys=True))

if __name__=='__main__':main()
