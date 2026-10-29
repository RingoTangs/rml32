import argparse
import gzip
import hashlib
import itertools
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path
from common import HERE,ROOT,digest,encoded,load,require
from matrix import HIGH,old

DRIVER='replay_public_interleavings'
def cases():
    for qi,q in enumerate(list(old.old.profiles())+[HIGH]):
        for payload,container,state,alias in itertools.product(('object','function'),('array','mapping'),('alive','pending','queued','shell'),(False,True)):
            yield dict(id=f'P{qi:03}-{payload}-{container}-{state}-{int(alias)}',limits=q,payload=payload,container=container,state=state,alias=alias)

def compare(ts,rows):
    require(len(rows)==len(ts),'public inventory')
    phases={};success={};passed=0
    for t,row in zip(ts,rows):
        require(row['id']==t['id'],'public order')
        if 'result' in row:
            expected=dict(copy_valid=t['state'] in ('alive','pending'),drained=int(t['state'] in ('pending','queued')),reaped=1,old_id_rejected=True,frames=0)
            require(row['result']==expected,'public projection')
            key='-'.join(str(t[k]) for k in ('payload','container','state','alias'))
            success[key]=success.get(key,0)+1;passed+=1
        else:
            require(t['limits']!=HIGH,'missing sufficient-quota control')
            require(row['phase'] in ('setup','transition','start','copy','release') and row['error']=='Limit','public refusal classification')
            phases[row['phase']]=phases.get(row['phase'],0)+1
    require(len(success)==32,'every public combination needs a full success')
    return dict(traces=len(ts),accepted=passed,quota_refusals=phases,success_by_combination=success,unclassified_differences=0)

def run():
    ts=list(cases());logs=[];observed=None;summary=None
    for profile in ('debug','release'):
        with tempfile.TemporaryDirectory(prefix='rml32-2bm-public-') as tmp:
            p=Path(tmp);inp=p/'in';out=p/'out'
            inp.write_text(''.join(json.dumps(t,sort_keys=True)+'\n' for t in ts))
            env=os.environ.copy();env.update(RML32_INTERLEAVINGS_IN=str(inp),RML32_INTERLEAVINGS_OUT=str(out))
            argv=['cargo','test']+(['--release'] if profile=='release' else [])+['--locked','--offline','--test','lifecycle_interleavings',DRIVER,'--','--ignored','--exact','--nocapture']
            r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
            raw=gzip.compress(out.read_bytes(),mtime=0);rows=[json.loads(l) for l in out.read_text().splitlines()];summary=compare(ts,rows)
            require(re.findall(r'^PUBLIC_INTERLEAVING_REPLAY (\d+)$',r.stdout,re.M)==[str(len(ts))],'public driver totals')
            require(observed is None or observed==raw,'public profile observations');observed=raw
            logs.append(dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(inp.read_bytes())))
    rawlog=gzip.compress(encoded(logs),mtime=0)
    return dict(stage='2BM',status='pass',summary=summary,log_sha256=digest(rawlog),observations_sha256=digest(observed)),rawlog,observed

def verify():
    r=load(HERE/'public-replay.json');raw=(HERE/'public-observations.jsonl.gz').read_bytes();logs=load(HERE/'public-log.json.gz')
    require(r['stage']=='2BM' and r['status']=='pass' and r['summary']==compare(list(cases()),[json.loads(l) for l in gzip.decompress(raw).splitlines()]),'public report')
    require(digest(raw)==r['observations_sha256'] and digest((HERE/'public-log.json.gz').read_bytes())==r['log_sha256'],'public hash')
    require(len(logs)==2 and [l['profile'] for l in logs]==['debug','release'] and all(l['returncode']==0 for l in logs),'public commands')
    expected_input=hashlib.sha256(''.join(json.dumps(t,sort_keys=True)+'\n' for t in cases()).encode()).hexdigest()
    for log in logs:
        argv=['cargo','test']+(['--release'] if log['profile']=='release' else [])+['--locked','--offline','--test','lifecycle_interleavings',DRIVER,'--','--ignored','--exact','--nocapture']
        require(log['argv']==argv and log['input_sha256']==expected_input,'public command/input identity')
        require(re.findall(r'^PUBLIC_INTERLEAVING_REPLAY (\d+)$',log['stdout'],re.M)==[str(r['summary']['traces'])],'saved public totals')
    return r

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');args=p.parse_args()
    if args.write:
        r,l,o=run();(HERE/'public-replay.json').write_bytes(encoded(r));(HERE/'public-log.json.gz').write_bytes(l);(HERE/'public-observations.jsonl.gz').write_bytes(o)
    print(json.dumps(verify(),sort_keys=True))
