"""Eleven frozen drivers; isolate imports, write only this batch's outputs."""
import argparse,base64,gzip,json,subprocess
from common import ROOT,HERE,load,require,digest,encoded
CODES={
 'previous':"import reference_replay as m;r,l=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode()}),sort_keys=True))",
 'batch':"import batch_diff as m;r,l,o=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode(),'observations':base64.b64encode(o).decode()}),sort_keys=True))",
 'combinations':"import combination_diff as m;r,l,obs=m.run();print(json.dumps(dict(report=r,logs=dict(log=base64.b64encode(l).decode(),**{k:base64.b64encode(v).decode()for k,v in obs.items()})),sort_keys=True))"}
PREFIX="import sys,json,base64;sys.path.insert(0,'docs/stage2bk-combination-audit');"
def run():
 logs=[];reports={}
 for n,code in CODES.items():
  argv=['python3','-B','-c',PREFIX+code];r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);v=json.loads(r.stdout);reports[n]=v['report'];logs.append(dict(name=n,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
 raw=gzip.compress(encoded(logs),mtime=0);return dict(stage='2BL',status='pass',drivers=11,reports=reports,quota_model_cases=39,arithmetic_rows=112,log_sha256=digest(raw)),raw

def verify():
 r=load(HERE/'reference-replay.json');raw=(HERE/'reference-log.json.gz').read_bytes();require(r['stage']=='2BL'and r['status']=='pass'and r['drivers']==11 and r['log_sha256']==digest(raw),'reference identity');logs=load(HERE/'reference-log.json.gz');require([c['name']for c in logs]==list(CODES)and all(c['returncode']==0 for c in logs),'reference commands')
 for c in logs:
  v=json.loads(c['stdout']);require(v['report']==r['reports'][c['name']],'reference result')
  for n,b in v['logs'].items():require(bool(gzip.decompress(base64.b64decode(b,validate=True))),'empty replay')
  if c['name']=='batch':
   old=load(ROOT/'docs/stage2bk-combination-audit/batch-diff.json');require(v['report']['summary']==old['summary']and v['report']['input_sha256']==old['input_sha256'],'frozen batch coverage')
  if c['name']=='combinations':
   old=load(ROOT/'docs/stage2bk-combination-audit/combination-diff.json');require(v['report']['summary']==old['summary']and v['report']['faults']==old['faults'],'frozen combination coverage')
 require(r['reports']['previous']['drivers']==8 and r['reports']['previous']['quota_model_cases']==39 and r['reports']['previous']['arithmetic_rows']==112,'older model scope');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l=run();(HERE/'reference-replay.json').write_bytes(encoded(r));(HERE/'reference-log.json.gz').write_bytes(l)
 print(json.dumps(verify(),sort_keys=True))
