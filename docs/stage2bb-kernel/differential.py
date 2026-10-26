"""Generate temporary full-edge traces from frozen 2BA, replay real Rust leases."""
import argparse
from collections import deque
from copy import deepcopy
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
BA=ROOT/'docs/stage2ba-design'
sys.path.insert(0,str(BA))
import model
import explore
sys.path.pop(0)

def digest(raw):return hashlib.sha256(raw).hexdigest()
def require(v,m):
    if not v:raise AssertionError(m)
def load(p):return json.loads(p.read_bytes())
def sources():
    names=['docs/stage2ba-design/model.py','docs/stage2ba-design/explore.py',
           'tests/lifecycle_kernel.rs','tests/support/lifecycle_kernel.rs',
           'docs/stage2bb-kernel/differential.py']
    return {n:digest((ROOT/n).read_bytes()) for n in names}
def domain(stream,objects,depth,max_states=200000,max_transitions=1000000):
    initial=model.World()
    for i in range(objects):initial.apply('object',str(i))
    queue=deque([(initial,0,[])]);seen={initial.key()};accepted=rejected=0;complete=True
    while queue:
        w,d,trace=queue.popleft()
        if d==depth:continue
        for action in explore.actions(w):
            if accepted+rejected>=max_transitions:complete=False;break
            c=deepcopy(w);before=c.key()
            try:c.apply(*action);ok=True
            except model.Rejected:
                ok=False;require(c.key()==before,'model rejection changed state')
            model.audit(c)
            if ok:accepted+=1
            else:rejected+=1
            row=dict(objects=objects,trace=trace,action=action,accepted=ok,expected=json.loads(c.key()))
            stream.write((json.dumps(row,sort_keys=True,separators=(',',':'))+'\n').encode())
            if ok and c.key() not in seen:
                if len(seen)>=max_states:complete=False;break
                seen.add(c.key());queue.append((c,d+1,trace+[action]))
        if not complete:break
    return dict(objects=objects,depth=depth,states=len(seen),accepted_transitions=accepted,
        rejected_transitions=rejected,complete=complete,max_states=max_states,
        max_transitions=max_transitions,state_set_sha256=digest('\n'.join(sorted(seen)).encode()))

def compare_domains(domains):
    expected=load(BA/'enumeration.json')['domains']
    require(len(domains)==len(expected)==2,'domain inventory')
    for d,e in zip(domains,expected):
        require(d['complete'],'capacity cutoff: incomplete exploration')
        require(all(d[k]==e[k] for k in ('objects','depth','states','accepted_transitions',
                'rejected_transitions','state_set_sha256','max_states','max_transitions')),'frozen domain differs')

def run():
    with tempfile.TemporaryDirectory(prefix='rml32-2bb-') as temp:
        path=Path(temp)/'traces.jsonl'
        with path.open('wb') as stream:domains=[domain(stream,1,6),domain(stream,2,4)]
        compare_domains(domains);raw=path.read_bytes()
        argv=['cargo','test','--test','lifecycle_kernel','--locked','--offline',
              'kernel::tests::replay_model_traces','--','--ignored','--exact','--nocapture']
        env=dict(os.environ,RML32_LIFECYCLE_TRACE_INPUT=str(path))
        result=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True)
        require(result.returncode==0,result.stdout+result.stderr)
        matches=re.findall(r'^LIFECYCLE_REPLAY (.+)$',result.stdout,re.M)
        require(len(matches)==1,'missing replay result')
        replay=json.loads(matches[0]);accepted=sum(d['accepted_transitions'] for d in domains);rejected=sum(d['rejected_transitions'] for d in domains)
        require(replay==dict(transitions=accepted+rejected,accepted=accepted,rejected=rejected,physical_teardowns=accepted+rejected),'replay inventory')
        log=dict(argv=argv,environment=dict(RML32_LIFECYCLE_TRACE_INPUT=str(path)),returncode=result.returncode,stdout=result.stdout,stderr=result.stderr,input_sha256=digest(raw),input_bytes=len(raw))
        lograw=gzip.compress(json.dumps(log,sort_keys=True).encode(),mtime=0)
        report=dict(stage='2BB',status='pass',domains=domains,replay=replay,
                    input_sha256=digest(raw),input_bytes=len(raw),source_sha256=sources(),
                    replay_log_sha256=digest(lograw),native_execution=False,business_bytecode_execution=False,
                    rust_kernel_execution=True,full_state_space_proof=False)
        return report,lograw

def verify(data=None):
    r=load(HERE/'differential.json') if data is None else data
    require(r['stage']=='2BB' and r['status']=='pass','report scope')
    require(r['source_sha256']==sources(),'source changed')
    require(r['native_execution'] is False and r['business_bytecode_execution'] is False and r['rust_kernel_execution'] is True and r['full_state_space_proof'] is False,'execution scope')
    compare_domains(r['domains'])
    accepted=sum(d['accepted_transitions'] for d in r['domains']);rejected=sum(d['rejected_transitions'] for d in r['domains'])
    require(r['replay']==dict(accepted=accepted,rejected=rejected,transitions=accepted+rejected,physical_teardowns=accepted+rejected),'replay count')
    lograw=(HERE/'differential-log.json.gz').read_bytes();require(digest(lograw)==r['replay_log_sha256'],'replay log hash')
    log=json.loads(gzip.decompress(lograw));require(log['returncode']==0 and log['input_sha256']==r['input_sha256'] and log['input_bytes']==r['input_bytes'],'replay input')
    matches=re.findall(r'^LIFECYCLE_REPLAY (.+)$',log['stdout'],re.M)
    require(len(matches)==1 and json.loads(matches[0])==r['replay'],'replay log result')
    return r

def main():
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');p.add_argument('--verify',action='store_true');a=p.parse_args()
    if a.verify:r=verify()
    else:
        r,log=run()
        if a.write:
            (HERE/'differential-log.json.gz').write_bytes(log)
            (HERE/'differential.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n')
            verify()
    print(json.dumps(r,sort_keys=True))
if __name__=='__main__':main()
