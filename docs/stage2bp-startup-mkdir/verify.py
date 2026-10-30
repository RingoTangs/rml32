"""Current full Rust tests + bounded mkdir differential; old matrices are frozen evidence."""
import argparse
from collections import Counter
import gzip,hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
RECORD=ROOT/'docs/stage2bp-validation.json'
ALLOWED={'src/prepared.rs','src/vm.rs','src/startup_probe.rs','src/startup_probe_main.rs','src/startup_vm_tests.rs',
         'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
NEW_SOURCE={'src/startup_memory_fs.rs','src/startup_mkdir_tests.rs','tests/startup_mkdir.rs'}
OUTPUTS={'validation-log.json.gz','service-observations.json.gz','focused.json','focused-log.json.gz',
         'focused-observations.jsonl.gz','corpus-validation.json'}
def require(ok,message):
    if not ok:raise AssertionError(message)
def digest(raw):return hashlib.sha256(raw).hexdigest()
def encoded(v):return (json.dumps(v,sort_keys=True,indent=2,ensure_ascii=False)+'\n').encode()
def load(p):
    b=p.read_bytes();return json.loads(gzip.decompress(b) if p.suffix=='.gz' else b)
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec)
    sys.modules[name]=m;spec.loader.exec_module(m);return m
BO=module('_2bp_bo',ROOT/'docs/stage2bo-startup-probe/verify.py')
SERVICES=BO.SERVICES
GATES=dict(stage2_complete=False,service_startup_complete=False,implementation_ready=False,
    native_equivalence_proven=False,reference_elf_execution=False,reference_scripts_execution=False,
    native_unknowns=20,native_unknowns_promoted=0,rust32_execution='not_run',
    dependencies_changed=False,existing_models_changed=False,default_prefix_changed=False,
    memory_mkdir_ready=True,host_filesystem_mkdir=False,current_large_matrix_replay=False,
    historical_large_drivers=17,current_focused_replay=True)

def boundary():
    b=load(HERE/'baseline.json');require(b['commit']=='e4d7ca6d55fa2c7e4675301fb3d9b0f384803774' and len(b['files'])==1265,'baseline identity')
    require(subprocess.run(['git','merge-base','--is-ancestor',b['commit'],'HEAD'],cwd=ROOT,capture_output=True).returncode==0,'baseline history')
    projection=module('_2bp_projection',ROOT/'docs/stage2bg-host-function-test/source_projection.py')
    changed=[];preserved=0
    for n,h in b['files'].items():
        raw=(ROOT/n).read_bytes()
        if digest(raw)==h:continue
        require(n in ALLOWED,'protected baseline changed: '+n);changed.append(n)
        old=subprocess.check_output(['git','show',b['commit']+':'+n],cwd=ROOT);require(digest(old)==h,'baseline bytes')
        if n.endswith('.md'):require(raw.endswith(old),'historical document overwritten')
        else:
            before=Counter(tuple(t) for t in projection.test_bodies(old.decode()))
            after=Counter(tuple(t) for t in projection.test_bodies(raw.decode()))
            require(before<=after,'old test body changed: '+n);preserved+=sum(before.values())
    expected={n for n in b['files'] if n.startswith(('src/','tests/'))}|NEW_SOURCE
    require(set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT,text=True).splitlines())==expected,'source inventory')
    return dict(baseline_commit=b['commit'],baseline_files=1265,byte_frozen_files=1265-len(changed),
                changed_files=sorted(changed),old_test_bodies_preserved=preserved,index_is_source_authority=False)

def snapshot():
    b=load(HERE/'baseline.json');p=ROOT/'target/stage2bp-baseline'
    if not p.exists():
        subprocess.run(['git','clone','--local','--no-hardlinks','--no-checkout',str(ROOT),str(p)],check=True,capture_output=True)
        subprocess.run(['git','checkout','--detach',b['commit']],cwd=p,check=True,capture_output=True)
    for n,h in b['files'].items():
        q=p/n;raw=q.read_bytes()
        if digest(raw)==h:continue
        variants=[raw,raw.replace(b'\r\n',b'\n'),raw.replace(b'\r\n',b'\n').replace(b'\n',b'\r\n')]
        value=next((v for v in variants if digest(v)==h),None)
        require(value is not None,'historical snapshot drift: '+n);q.write_bytes(value)
    return p

def artifacts():
    files=ALLOWED|NEW_SOURCE
    files|={str(p.relative_to(ROOT)) for p in HERE.rglob('*') if p.is_file() and '__pycache__' not in p.parts and p.name!='validation-log.json.gz'}
    return {n:digest((ROOT/n).read_bytes()) for n in sorted(files)}

def old_stdout(profile,name):
    logs=load(ROOT/'docs/stage2bo-startup-probe/validation-log.json.gz')
    argv=[f'target/{profile}/rml32-startup-probe','--service',name,'--root','.']
    return next(c['stdout'] for c in logs if c['argv']==argv)

def validate_service(profile,name,slice_name,raw):
    old=old_stdout(profile,name);base=json.loads(old);r=json.loads(raw)
    BO.validate_probe(name,base)
    if slice_name=='allocate':
        require(raw==old,'legacy stdout changed');return
    offset=SERVICES[name][1]
    expected=json.loads(old)
    expected.update(executed_instructions=28,stop_pc=28,stop_offset=offset+112,slice='log_directory',
        next_dependency=dict(pc=30,offset=offset+120,slot=583,name='xdbc_set_output_basic_file'),
        next_dependency_kind='global_function',next_dependency_reason='atomic_function_not_enabled')
    expected['configuration']['filesystem']=dict(backend='memory',max_nodes=16,max_path_bytes=255)
    expected['host_events']=[dict(operation='allocate_array',call=base['host_calls'][0]),
        dict(operation='mkdir',call=dict(slot=336,name='mkdir',pc=26,offset=offset+104,
            argument=dict(bytes_hex=f'/{name}/log/'.encode().hex()),normalized=dict(bytes_hex=f'/{name}/log'.encode().hex()),result=1,error=None))]
    paths=['/','/aaa','/ccs','/csa','/dba','/gs',f'/{name}/log']
    expected['filesystem']=dict(entries=[dict(path=dict(bytes_hex=p.encode().hex()),directory=True)for p in sorted(paths)])
    require(r==expected,'real service state: '+name)

def native():
    b=load(HERE/'native-evidence.json.gz');records=b['records']
    expected={'vm_f_mkdir':'0x80e9110','vm_generateLocalFileName':'0x80be1fc','vm_deriveFileName':'0x80bda64',
        'vm_generateFileName':'0x80be018','vm_assureEuid':'0x80b137c','vm_isInVirtualVolume':'0x809f2e4',
        'vm_isInPackedVolume':'0x809f304','vm_isInDbfsVolume':'0x809f324'}
    require(len(records)==8 and not b['captures'] and not b['unknowns'],'native scope')
    require({r['parameters']['procedure']for r in records}==set(expected),'native function inventory')
    sha=digest((ROOT/'magic_Linux32').read_bytes())
    for r in records:
        require(r['operation']=='analyze_function' and r['authority']=='shipped-artifact','native authority')
        require(r['subject']['digest']['sha256']==sha,'native target identity')
        n=r['parameters']['procedure'];v=r['normalized_result']
        require(v['procedure']['address']==expected[n] and v['assembly'] and v['pseudocode'],'native body')
    return dict(records=8,evidence_ids=sorted(r['evidence_id']for r in records),reference_execution=False)

PREFIX='docs/stage2bp-startup-mkdir/'
COMMANDS=[('target/stage2bp-baseline',['python3','-B','docs/stage2bo-startup-probe/verify.py']),
    ('.',['cargo','test','--locked','--offline']),('.',['cargo','test','--release','--locked','--offline']),
    ('.',['cargo','build','--locked','--offline']),('.',['cargo','build','--release','--locked','--offline']),
    ('.',['cargo','fmt','--check']),('.',['cargo','clippy','--locked','--offline','--all-targets','--','-D','warnings']),
    ('.',['python3','tests/compare_corpus.py','--report','target/stage2bp-corpus-validation.json']),
    ('.',['python3','-B',PREFIX+'focused.py','--write']),
    ('.',['rustc','-Vv']),('.',['rustup','target','list','--installed'])]
for profile in ('debug','release'):
    for slice_name in ('allocate','log-dir'):
        for name in SERVICES:
            COMMANDS.append(('.',[f'target/{profile}/rml32-startup-probe','--service',name,'--root','.']+
                             ([] if slice_name=='allocate' else ['--slice',slice_name])))
COMMANDS.append(('.',['python3','-B','-m','unittest','discover','-s',PREFIX.rstrip('/'),'-p','test_*.py']))

def validate_tests(text):
    passed=re.findall(r'^test (\S+) \.\.\. ok$',text,re.M)
    old=set(load(ROOT/'docs/stage2bo-validation.json')['current_test_names'])
    require(len(passed)==len(set(passed))==920 and old<=set(passed) and len(set(passed)-old)==12,'Rust test inventory')
    docs=re.findall(r'^test (.+ \(line \d+\).+) \.\.\. ok$',text,re.M);require(len(docs)==5,'doctest inventory')
    ignored=re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',text,re.M)
    old_logs=load(ROOT/'docs/stage2bo-startup-probe/validation-log.json.gz')
    old_out=next(c for c in old_logs if c['argv']==['cargo','test','--locked','--offline'])
    expected=set(re.findall(r'^test (\S+) \.\.\. ignored(?:,.*)?$',old_out['stdout']+old_out['stderr'],re.M))|{'startup_probe::mkdir_tests::replay_mkdir_cases'}
    require(len(ignored)==18 and set(ignored)==expected,'ignored driver inventory')
    results=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)
    require(sum(int(p)for p,_,_ in results)==925 and all(f=='0'for _,f,_ in results),'Rust totals')
    return sorted(passed)

def validate_commands(logs):
    require([(c['cwd'],c['argv'])for c in logs]==COMMANDS,'command inventory')
    require(all(c['returncode']==0 for c in logs),'failed command')
    h=json.loads(logs[0]['stdout']);require(h['stage']=='2BO' and h['status']=='pass' and h['rust_tests_debug']==908,'frozen historical result')
    names=validate_tests(logs[1]['stdout']+logs[1]['stderr'])
    require(names==validate_tests(logs[2]['stdout']+logs[2]['stderr']),'profile test inventory')
    require('host: aarch64-apple-darwin' in logs[9]['stdout'] and logs[10]['stdout'].splitlines()==['aarch64-apple-darwin'],'execution target')
    observations=load(HERE/'service-observations.json.gz');index=11
    for profile in ('debug','release'):
        for mode in ('allocate','log-dir'):
            for name in SERVICES:
                c=logs[index];validate_service(profile,name,mode,c['stdout'])
                require(observations[profile][mode][name]==c['stdout'],'saved service observations');index+=1
    require(observations['debug']==observations['release'],'profile stdout differs')
    output=logs[-1]['stdout']+logs[-1]['stderr']
    require('Ran 8 tests' in output and output.rstrip().endswith('OK'),'Python inventory')
    return names

def build(logs):
    names=validate_commands(logs);focused=module('_2bp_focused',HERE/'focused.py').verify()
    require(json.loads(logs[8]['stdout'])==focused,'focused command result')
    history=load(HERE/'historical-log.json.gz');require(history['returncode']==0 and json.loads(history['stdout'])==json.loads(logs[0]['stdout']),'historical authority')
    return dict(stage='2BP',status='pass',boundary=boundary(),gates=GATES,native=native(),
        rust_tests_debug=920,rust_tests_release=920,old_rust_tests=908,new_rust_tests=12,doctests_each_profile=5,
        ignored_drivers_each_profile=18,current_python_tests=8,current_test_names=names,
        current_focused=focused['summary'],historical_large_drivers=17,historical_run_on_current_source=False,
        default_prefix_end_pc=25,log_directory_end_pc=28,next_blocker=dict(pc=30,slot=583,name='xdbc_set_output_basic_file',reason='atomic_function_not_enabled'),
        corpus=load(HERE/'corpus-validation.json'),artifact_sha256=artifacts())

def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');args=p.parse_args()
    if args.write:
        boundary();snapshot();before={n:h for n,h in artifacts().items()if Path(n).name not in OUTPUTS}
        logs=[];observations={q:{s:{}for s in ('allocate','log-dir')}for q in ('debug','release')}
        for cwd,argv in COMMANDS:
            print('Validating: '+' '.join(argv),file=sys.stderr,flush=True)
            r=subprocess.run(argv,cwd=ROOT/cwd,capture_output=True,text=True)
            logs.append(dict(cwd=cwd,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
            # Checkpoint actual command results even if later packaging fails.
            (HERE/'validation-log.json.gz').write_bytes(gzip.compress(encoded(logs),mtime=0))
            require(r.returncode==0,r.stdout+r.stderr)
            if argv[:2]==['python3','tests/compare_corpus.py']:
                (HERE/'corpus-validation.json').write_bytes(encoded(load(ROOT/'target/stage2bp-corpus-validation.json')))
            if argv[0].endswith('/rml32-startup-probe'):
                profile=argv[0].split('/')[1];mode='log-dir'if '--slice'in argv else'allocate';name=argv[2]
                observations[profile][mode][name]=r.stdout
                if sum(len(v)for p in observations.values()for v in p.values())==20:
                    (HERE/'service-observations.json.gz').write_bytes(gzip.compress(encoded(observations),mtime=0))
        after=artifacts();require(all(after[n]==h for n,h in before.items()),'input artifacts changed during validation')
        raw=(HERE/'validation-log.json.gz').read_bytes();record=build(logs);record['log_sha256']=digest(raw);RECORD.write_bytes(encoded(record))
    else:
        record=load(RECORD);raw=(HERE/'validation-log.json.gz').read_bytes();require(record['log_sha256']==digest(raw),'log identity')
        expected=build(json.loads(gzip.decompress(raw)));expected['log_sha256']=digest(raw);require(record==expected,'receipt differs')
    print(json.dumps({k:record[k]for k in ('stage','status','rust_tests_debug','rust_tests_release','new_rust_tests','doctests_each_profile','current_focused','gates')},sort_keys=True))
if __name__=='__main__':main()
