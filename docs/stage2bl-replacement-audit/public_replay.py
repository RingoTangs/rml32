import argparse,gzip,json,itertools,os,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,require,digest,encoded,load
from matrix import HIGH,old

def cases():
 for p,q in enumerate(old.profiles()+[HIGH]):
  for kind,container,alias in itertools.product(('object','array','mapping','function'),('array','mapping'),(False,True)):
   yield dict(id=f'P{p:03}-{kind}-{container}-{int(alias)}',limits=q,old=kind,container=container,alias=alias)
def compare(cases,rows):
 result={};seen=0
 for t,r in zip(cases,rows,strict=True):
  require(t['id']==r['id'],'public inventory');seen+=1
  if 'error'in r:require(r['error']=='Limit','unclassified public failure '+str(r));status='entry_'+r['phase']
  else:
   p=r['result'];status=p['target'];require(status in ('accepted','rejected'),'public target')
   if status=='rejected':require(p['atomic']is True,'public failed publication')
   else:require(p['frames']==0 and p['reaped_before_alias_drop']==int(not t['alias'])and p['reaped_after_alias_drop']==int(t['alias']),'public release count')
  if t['limits']==HIGH:require(status=='accepted','missed public success control')
  key=t['old']+':'+t['container']+':'+status;result[key]=result.get(key,0)+1
 return dict(traces=seen,classifications=result,unclassified_differences=0,normal_library_build=True)
def run():
 records=[];observed=None;ts=list(cases());rawin=b''.join((json.dumps(t,sort_keys=True)+'\n').encode()for t in ts)
 with tempfile.TemporaryDirectory(prefix='rml32-2bl-public-')as tmp:
  p=Path(tmp);(p/'in').write_bytes(rawin);env=os.environ.copy();env.update(RML32_REPLACEMENTS_IN=str(p/'in'),RML32_REPLACEMENTS_OUT=str(p/'out'))
  for profile in ('debug','release'):
   argv=['cargo','test']+(['--release']if profile=='release'else [])+['--locked','--offline','--test','lifecycle_replacements','replay_public_replacements','--','--ignored','--exact','--nocapture'];r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);raw=(p/'out').read_bytes();summary=compare(ts,[json.loads(x)for x in raw.splitlines()]);require(re.findall(r'^PUBLIC_REPLACEMENT_REPLAY (\d+)$',r.stdout,re.M)==[str(len(ts))],'public totals');require(observed is None or observed==raw,'public profile observations');observed=raw;records.append(dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,observations_sha256=digest(raw)))
 log=gzip.compress(encoded(records),mtime=0);obs=gzip.compress(observed,mtime=0);return dict(stage='2BL',status='pass',summary=summary,input_sha256=digest(rawin),log_sha256=digest(log),observations_sha256=digest(obs)),log,obs
def verify():
 r=load(HERE/'public-replay.json');log=(HERE/'public-log.json.gz').read_bytes();obs=(HERE/'public-observations.jsonl.gz').read_bytes();require(r['stage']=='2BL'and r['status']=='pass'and r['log_sha256']==digest(log)and r['observations_sha256']==digest(obs),'public artifacts');raw=b''.join((json.dumps(t,sort_keys=True)+'\n').encode()for t in cases());require(r['input_sha256']==digest(raw)and r['summary']==compare(cases(),[json.loads(x)for x in gzip.decompress(obs).splitlines()]),'public report');logs=load(HERE/'public-log.json.gz');require([c['profile']for c in logs]==['debug','release']and all(c['returncode']==0 and c['observations_sha256']==digest(gzip.decompress(obs))for c in logs),'public commands');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l,o=run();(HERE/'public-replay.json').write_bytes(encoded(r));(HERE/'public-log.json.gz').write_bytes(l);(HERE/'public-observations.jsonl.gz').write_bytes(o)
 print(json.dumps(verify(),sort_keys=True))
