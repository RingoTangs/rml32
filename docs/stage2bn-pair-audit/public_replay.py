import argparse,gzip,itertools,json,os,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,digest,encoded,load,require
from matrix import HIGH,old
DRIVER='replay_public_pairs'
def cases():
    for qi,q in enumerate(list(old.old.old.profiles())+[HIGH]):
        for container,payload,alias in itertools.product(('array','mapping'),('object','function'),(False,True)):
            yield dict(id=f'P{qi:03}-{container}-{payload}-{alias}',kind='members',limits=q,container=container,payload=payload,alias=alias)
        for kind in ('heads','frames'):yield dict(id=f'P{qi:03}-{kind}',kind=kind,limits=q)
def compare(ts,rows):
    require(len(ts)==len(rows),'public inventory');success={};refusals={};accepted=0
    for t,row in zip(ts,rows):
        require(row['id']==t['id'],'public order')
        if 'result' in row:
            if t['kind']=='members':
                expected=dict(value=8,reaped_before_alias_drop=int(not t['alias']),reaped_after_alias_drop=int(t['alias']),frames=0)
                key='-'.join(str(t[k]) for k in ('kind','container','payload','alias'))
            elif t['kind']=='heads':expected=dict(independent_heads=2,reclaimed=2,reaped=1,frames=0);key='heads'
            else:expected=dict(peak_frames=2,drained=1,reaped=1,frames=0);key='frames'
            require(row['result']==expected,'public projection');success[key]=success.get(key,0)+1;accepted+=1
        else:
            require(t['limits']!=HIGH,'missing sufficient-quota control')
            require(row['error']=='Limit' and row['phase'] in ('setup','first','second','release','start','nested'),'public refusal classification')
            key=t['kind']+':'+row['phase'];refusals[key]=refusals.get(key,0)+1
    require(len(success)==10,'public success inventory')
    return dict(traces=len(ts),accepted=accepted,success_by_combination=success,quota_refusals=refusals,unclassified_differences=0)
def run():
    ts=list(cases());logs=[];observed=None
    for profile in ('debug','release'):
        with tempfile.TemporaryDirectory(prefix='rml32-2bn-public-') as tmp:
            p=Path(tmp);inp=p/'in';out=p/'out';inp.write_text(''.join(json.dumps(t,sort_keys=True)+'\n' for t in ts))
            env=os.environ.copy();env.update(RML32_PAIRS_IN=str(inp),RML32_PAIRS_OUT=str(out))
            argv=['cargo','test']+(['--release'] if profile=='release' else [])+['--locked','--offline','--test','lifecycle_pairs',DRIVER,'--','--ignored','--exact','--nocapture']
            r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
            raw=gzip.compress(out.read_bytes(),mtime=0);summary=compare(ts,[json.loads(l) for l in out.read_text().splitlines()])
            require(re.findall(r'^PUBLIC_PAIR_REPLAY (\d+)$',r.stdout,re.M)==[str(len(ts))],'public Rust totals')
            require(observed is None or observed==raw,'public profile observations');observed=raw
            logs.append(dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(inp.read_bytes())))
    raw=gzip.compress(encoded(logs),mtime=0)
    return dict(stage='2BN',status='pass',summary=summary,log_sha256=digest(raw),observations_sha256=digest(observed)),raw,observed
def verify():
    r=load(HERE/'public-replay.json');raw=(HERE/'public-observations.jsonl.gz').read_bytes();logs=load(HERE/'public-log.json.gz');ts=list(cases())
    require(r['stage']=='2BN' and r['status']=='pass' and r['summary']==compare(ts,[json.loads(l) for l in gzip.decompress(raw).splitlines()]),'public identity')
    require(r['observations_sha256']==digest(raw) and r['log_sha256']==digest((HERE/'public-log.json.gz').read_bytes()),'public hashes')
    require(len(logs)==2 and [l['profile'] for l in logs]==['debug','release'],'public profiles')
    expected=digest(''.join(json.dumps(t,sort_keys=True)+'\n' for t in ts).encode())
    for l in logs:
        argv=['cargo','test']+(['--release'] if l['profile']=='release' else [])+['--locked','--offline','--test','lifecycle_pairs',DRIVER,'--','--ignored','--exact','--nocapture']
        require(l['argv']==argv and l['returncode']==0 and l['input_sha256']==expected,'public commands/input')
        require(re.findall(r'^PUBLIC_PAIR_REPLAY (\d+)$',l['stdout'],re.M)==[str(len(ts))],'saved public Rust totals')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');args=p.parse_args()
    if args.write:
        r,l,o=run();(HERE/'public-replay.json').write_bytes(encoded(r));(HERE/'public-log.json.gz').write_bytes(l);(HERE/'public-observations.jsonl.gz').write_bytes(o)
    print(json.dumps(verify(),sort_keys=True))
