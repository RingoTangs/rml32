"""Thirteen frozen drivers; isolate imports, write only this batch's outputs."""
import argparse,base64,gzip,json,subprocess
from common import ROOT,HERE,load,require,digest,encoded
CODES={
 'previous':"import reference_replay as m;r,l=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode()}),sort_keys=True))",
 'replacement':"import replacement_diff as m;r,l,obs=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode(),**{k:base64.b64encode(v).decode()for k,v in obs.items()}}),sort_keys=True))",
 'public':"import public_replay as m;r,l,o=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode(),'observations':base64.b64encode(o).decode()}),sort_keys=True))"}

PREFIX="import sys,json,base64;sys.path.insert(0,'docs/stage2bl-replacement-audit');"
def run():
 logs=[];reports={}
 for n,code in CODES.items():
  argv=['python3','-B','-c',PREFIX+code];r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);v=json.loads(r.stdout);reports[n]=v['report'];logs.append(dict(name=n,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
 raw=gzip.compress(encoded(logs),mtime=0);return dict(stage='2BM',status='pass',drivers=13,reports=reports,quota_model_cases=39,arithmetic_rows=112,log_sha256=digest(raw)),raw

def verify():
 r=load(HERE/'reference-replay.json');raw=(HERE/'reference-log.json.gz').read_bytes();require(r['stage']=='2BM'and r['status']=='pass'and r['drivers']==13 and r['log_sha256']==digest(raw),'reference identity');logs=load(HERE/'reference-log.json.gz');require([c['name']for c in logs]==list(CODES)and all(c['returncode']==0 for c in logs),'reference commands')
 for c in logs:
  v=json.loads(c['stdout']);require(v['report']==r['reports'][c['name']],'reference result')
  for n,b in v['logs'].items():require(bool(gzip.decompress(base64.b64decode(b,validate=True))),'empty replay')
  if c['name']=='replacement':
   old=load(ROOT/'docs/stage2bl-replacement-audit/replacement-diff.json');require(v['report']['reports']==old['reports'],'frozen replacement coverage')
  if c['name']=='public':
   old=load(ROOT/'docs/stage2bl-replacement-audit/public-replay.json');require(v['report']['summary']==old['summary'],'frozen public coverage')
 require(r['reports']['previous']['drivers']==11 and r['reports']['previous']['quota_model_cases']==39 and r['reports']['previous']['arithmetic_rows']==112,'older model scope');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l=run();(HERE/'reference-replay.json').write_bytes(encoded(r));(HERE/'reference-log.json.gz').write_bytes(l)
 print(json.dumps(verify(),sort_keys=True))
