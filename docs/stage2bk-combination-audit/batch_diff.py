import argparse,gzip,json,os,re,subprocess,tempfile,hashlib
from pathlib import Path
from common import ROOT,HERE,require,digest,encoded,load
from matrix import batches
from batch_oracle import BatchOracle,m
DRIVER='boundary::lifecycle_combination_tests::replay_combination_batches'
def compare(cases,rows):
 it=iter(rows);counts=dict(traces=0,steps=0,setup_rejected=0,target_reached=0,accepted=0,rejected=0);layouts={}
 for t in cases:
  oracle=BatchOracle(t['limits']);previous=None;counts['traces']+=1;stat=layouts.setdefault(t['actions'][1]['layout'],dict(reached=0,committed=0,aborted=0,rejected=0))
  for step,a in enumerate(t['actions']):
   row=next(it,None);require(row is not None and row['id']==t['id']and row['step']==step,'checkpoint inventory')
   require(previous is None or previous==row['before'],'checkpoint continuity');require(row['before']==oracle.projection(),f'{t["id"]} {step} before mismatch {row["before"]} != {oracle.projection()}')
   error=None
   try:oracle.event(a)
   except m.Rejected as e:error=e.kind
   require(row['error']==error,f'{t["id"]} {step} result {row["error"]} != {error}')
   require(row['after']==oracle.projection(),f'{t["id"]} {step} after mismatch {row["after"]} != {oracle.projection()}')
   counts['steps']+=1;counts['accepted'if error is None else 'rejected']+=1
   if step==0 and error:counts['setup_rejected']+=1
   if step==1:counts['target_reached']+=1;stat['reached']+=1
   if a['op']in ('commit','abort'):
    stat['rejected'if error else 'committed'if a['op']=='commit'else 'aborted']+=1
   previous=row['after']
   if error:break
 require(next(it,None)is None,'extra checkpoint')
 require(all(s['reached']for s in layouts.values()),'unreached layout')
 return dict(**counts,layouts=layouts,unclassified_differences=0)
def rows(path):
 with gzip.open(path,'rt')as f:
  for line in f:yield json.loads(line)
def run():
 commands=[];observations=None;input_hash=None
 with tempfile.TemporaryDirectory(prefix='rml32-2bk-batch-')as tmp:
  p=Path(tmp);inp=p/'input';out=p/'output';packed=p/'observations.gz';h=hashlib.sha256()
  with inp.open('wb')as f:
   for t in batches():raw=(json.dumps(t,sort_keys=True)+'\n').encode();f.write(raw);h.update(raw)
  input_hash=h.hexdigest();env=os.environ.copy();env.update(RML32_COMBINATIONS_IN=str(inp),RML32_COMBINATIONS_OUT=str(out))
  for profile in ('debug','release'):
   argv=['cargo','test']+(['--release']if profile=='release'else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture'];r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
   with out.open('rb')as src,packed.open('wb')as dst:
    with gzip.GzipFile(fileobj=dst,mode='wb',mtime=0)as z:
     while chunk:=src.read(1048576):z.write(chunk)
   raw=packed.read_bytes();summary=compare(batches(),rows(packed));require(re.findall(r'^COMBINATION_BATCH_REPLAY (\d+)$',r.stdout,re.M)==[str(summary['traces'])],'actual trace totals');require(observations is None or observations==raw,'profile observations');observations=raw
   commands.append(dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,observations_sha256=digest(raw)))
 log=gzip.compress(encoded(commands),mtime=0)
 return dict(stage='2BK',status='pass',input_sha256=input_hash,summary=summary,log_sha256=digest(log),observations_sha256=digest(observations)),log,observations
def verify():
 r=load(HERE/'batch-diff.json');raw=(HERE/'batch-observations.jsonl.gz').read_bytes();log=(HERE/'batch-log.json.gz').read_bytes();require(r['stage']=='2BK'and r['status']=='pass'and r['observations_sha256']==digest(raw)and r['log_sha256']==digest(log),'batch identity');h=hashlib.sha256()
 for t in batches():h.update((json.dumps(t,sort_keys=True)+'\n').encode())
 require(r['input_sha256']==h.hexdigest()and r['summary']==compare(batches(),rows(HERE/'batch-observations.jsonl.gz')),'batch summary');commands=load(HERE/'batch-log.json.gz');require([x['profile']for x in commands]==['debug','release']and all(x['returncode']==0 and x['observations_sha256']==digest(raw)for x in commands),'batch logs');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l,o=run();(HERE/'batch-diff.json').write_bytes(encoded(r));(HERE/'batch-log.json.gz').write_bytes(l);(HERE/'batch-observations.jsonl.gz').write_bytes(o)
 print(json.dumps(verify(),sort_keys=True))
