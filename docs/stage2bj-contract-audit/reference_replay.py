"""Reexecute all six frozen drivers without changing their reports."""
import argparse,gzip,importlib.util,json,re,sys
from common import ROOT,HERE,digest,load,require
# Explicit module loading avoids the local historical/host verifier namespace.
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,ROOT/path);m=importlib.util.module_from_spec(spec);sys.modules[name]=m;spec.loader.exec_module(m);return m
sys.path.insert(0,str(ROOT/'docs/stage2bd-runtime-test'))
vm=module('_2bj_old_vm','docs/stage2bd-runtime-test/vm_diff.py')
sys.path.insert(0,str(ROOT/'docs/stage2bc-adapter'))
adapter=module('_2bj_old_adapter','docs/stage2bc-adapter/adapter_diff.py')
sys.path.insert(0,str(ROOT/'docs/stage2bb-kernel'))
kernel=module('_2bj_old_kernel','docs/stage2bb-kernel/differential.py')
for p in ('docs/stage2bb-kernel','docs/stage2bc-adapter','docs/stage2bd-runtime-test'):sys.path.remove(str(ROOT/p))
host=module('_2bj_old_host','docs/stage2be-host-test/host_diff.py')
managed=module('_2bj_old_managed','docs/stage2bf-managed-test/managed_diff.py')
callable_model=module('_2bj_old_callable','docs/stage2bg-host-function-test/callable_diff.py')
SPECS=[('callable',callable_model,'run','docs/stage2bg-host-function-test/callable-diff.json','CALLABLE_REPLAY '),('managed',managed,'run','docs/stage2bf-managed-test/managed-diff.json','MANAGED_REPLAY '),('host',host,'run','docs/stage2be-host-test/host-diff.json','HOST_REPLAY '),('vm',vm,'run_all','docs/stage2bd-runtime-test/vm-diff.json','VM_LIFECYCLE_REPLAY '),('adapter',adapter,'run','docs/stage2bc-adapter/adapter-diff.json','ADAPTER_REPLAY '),('kernel',kernel,'run','docs/stage2bb-kernel/differential.json','LIFECYCLE_REPLAY ')]
def run():
    result=dict(stage='2BJ',status='pass');logs={}
    for key,m,fn,_,_ in SPECS:
        report,raw=getattr(m,fn)();result[key]=report;logs[key]=raw
    return result,logs
def verify():
    result=load(HERE/'reference-replay.json');require(result['stage']=='2BJ' and result['status']=='pass','reference stage')
    for key,m,_,prior,prefix in SPECS:
        r=result[key];base=load(ROOT/prior);raw=(HERE/('reference-'+key+'-log.json.gz')).read_bytes();log=json.loads(gzip.decompress(raw));hkey='replay_log_sha256'if key=='kernel'else 'log_sha256'
        require(r['input_sha256']==base['input_sha256'] and r['replay']==base['replay'] and r['source_sha256']==m.sources(),'reference identity '+key)
        require(r[hkey]==digest(raw) and log['returncode']==0,'reference log '+key)
        matches=re.findall('^'+prefix+'(.+)$',log['stdout'],re.M);require(len(matches)==1 and json.loads(matches[0])=={k:v for k,v in base['replay'].items()if k!='frozen_model_traces'},'reference actual result '+key)
    return result
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,logs=run();(HERE/'reference-replay.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n')
        for key,raw in logs.items():(HERE/('reference-'+key+'-log.json.gz')).write_bytes(raw)
    print(json.dumps(verify(),sort_keys=True))
