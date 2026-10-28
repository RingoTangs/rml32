"""Normal-library mirror of exactly the six publicly representable trace families."""
import argparse,gzip,json,os,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,digest,encoded,load,require
from matrix import traces
from oracle import Oracle,m
FAMILIES={'F01','F02','F03','F04','F05','F15'}
def cases():return [t for t in traces()if t['family']in FAMILIES]
def project(oracle):
 p=oracle.projection();return dict(objects={k:v['state']in ('alive','pending')for k,v in p['objects'].items()},slots={k:v for k,v in p['slots'].items()if not k.startswith(('var:','mem:'))},function_owners={k:v['owners']for k,v in p['headers'].items()if v['kind']=='function'and v['owners']},frames=len(oracle.w.frames))
def compare(rows):
 it=iter(rows);steps=0;accepted=rejected=0
 for t in cases():
  oracle=Oracle(t['limits'])
  for step,a in enumerate(t['actions']):
   row=next(it,None);require(row is not None and row['id']==t['id']and row['step']==step,'public checkpoint inventory');error=None
   try:expected=oracle.event(a)
   except m.Rejected as e:error=e.kind
   require(row['error_kind']==error and row['accepted']==(error is None),f'public {t["id"]} step {step}: {error} != {row["error_kind"]}')
   require(row['projection']==(dict(closed=True)if a['op']=='close'and error is None else project(oracle)),f'public {t["id"]} step {step} projection {row["projection"]} != {project(oracle)}')
   if a['op']=='maintain':require(row['progress']=={k:expected[k]for k in row['progress']},'public progress')
   steps+=1;accepted+=row['accepted'];rejected+=not row['accepted']
   if not row['accepted']and not a.get('continue_on_error',False):break
 require(next(it,None)is None,'extra public checkpoint');return dict(traces=len(cases()),steps=steps,accepted=accepted,rejected=rejected,families=sorted(FAMILIES),normal_library_build=True)
def run():
 raw=b''.join((json.dumps(t,sort_keys=True)+'\n').encode()for t in cases());commands=[];observed=None
 with tempfile.TemporaryDirectory(prefix='rml32-2bj-public-')as temp:
  p=Path(temp);(p/'in').write_bytes(raw);env=os.environ.copy();env.update(RML32_D08_TRACES=str(p/'in'),RML32_D08_OBSERVATIONS=str(p/'out'))
  for profile in ('debug','release'):
   argv=['cargo','test']+(['--release']if profile=='release'else [])+['--locked','--offline','--test','lifecycle_contract_public','replay_public_contract_traces','--','--ignored','--exact','--nocapture'];r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
   current=(p/'out').read_bytes();summary=compare([json.loads(x)for x in current.splitlines()]);actual=re.findall('^PUBLIC_CONTRACT_REPLAY (.+)$',r.stdout,re.M);require(len(actual)==1 and json.loads(actual[0])=={k:summary[k]for k in ('traces','steps')},'public totals');require(observed is None or observed==current,'public profile observations differ');observed=current
   commands.append(dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,observations_sha256=digest(current)))
  log=gzip.compress(encoded(dict(profiles=commands)),mtime=0);packed=gzip.compress(observed,mtime=0)
  return dict(stage='2BJ',status='pass',profiles_checked=['debug','release'],summary=summary,input_sha256=digest(raw),observations_sha256=digest(packed),log_sha256=digest(log)),log,packed
def verify():
 r=load(HERE/'public-diff.json');raw=(HERE/'public-observations.jsonl.gz').read_bytes();log=(HERE/'public-log.json.gz').read_bytes();require(r['stage']=='2BJ'and r['status']=='pass'and r['observations_sha256']==digest(raw)and r['log_sha256']==digest(log),'public artifacts');require(r['input_sha256']==digest(b''.join((json.dumps(t,sort_keys=True)+'\n').encode()for t in cases())),'public inputs');commands=load(HERE/'public-log.json.gz')['profiles'];require([c['profile']for c in commands]==['debug','release']and all(c['returncode']==0 and c['observations_sha256']==digest(gzip.decompress(raw))for c in commands),'public profile logs');require(r['summary']==compare([json.loads(x)for x in gzip.decompress(raw).splitlines()]),'public summary');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,log,raw=run();(HERE/'public-diff.json').write_bytes(encoded(r));(HERE/'public-log.json.gz').write_bytes(log);(HERE/'public-observations.jsonl.gz').write_bytes(raw)
 print(json.dumps(verify(),sort_keys=True))
