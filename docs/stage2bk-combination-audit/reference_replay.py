"""Frozen runners execute in isolated interpreters; outputs belong exclusively to 2BK."""
import argparse,base64,gzip,json,subprocess
from common import ROOT,HERE,load,require,digest,encoded
CODES={
 'six':"import reference_replay as m;r,logs=m.run();print(json.dumps(dict(report=r,logs={k:base64.b64encode(v).decode()for k,v in logs.items()}),sort_keys=True))",
 'contract':"import contract_diff as m;r,l,o=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode(),'observations':base64.b64encode(o).decode()}),sort_keys=True))",
 'public':"import public_diff as m;r,l,o=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode(),'observations':base64.b64encode(o).decode()}),sort_keys=True))",
 'model':"import model_replay as m;r,l=m.run();print(json.dumps(dict(report=r,logs={'log':base64.b64encode(l).decode()}),sort_keys=True))"}
PREFIX="import sys,json,base64;sys.path.insert(0,'docs/stage2bj-contract-audit');"
def run():
 logs=[];reports={}
 for name,code in CODES.items():
  argv=['python3','-B','-c',PREFIX+code];r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);v=json.loads(r.stdout);reports[name]=v['report'];logs.append(dict(name=name,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
 raw=gzip.compress(encoded(logs),mtime=0)
 return dict(stage='2BK',status='pass',reports=reports,drivers=8,quota_model_cases=39,arithmetic_rows=112,log_sha256=digest(raw)),raw
def verify():
 r=load(HERE/'reference-replay.json');raw=(HERE/'reference-log.json.gz').read_bytes();require(r['stage']=='2BK'and r['status']=='pass'and r['drivers']==8 and r['log_sha256']==digest(raw),'reference identity');logs=load(HERE/'reference-log.json.gz');require([c['name']for c in logs]==list(CODES)and all(c['returncode']==0 for c in logs),'reference commands')
 for c in logs:
  v=json.loads(c['stdout']);require(v['report']==r['reports'][c['name']],'reference report')
  for key,value in v['logs'].items():
   raw=base64.b64decode(value,validate=True);require(bool(raw),'empty saved replay');json.loads(gzip.decompress(raw))if key!='observations'else None
  if c['name']in ('contract','public'):
   prior=load(ROOT/('docs/stage2bj-contract-audit/'+('contract-diff.json'if c['name']=='contract'else 'public-diff.json')));require(v['report']['summary']==prior['summary']and v['report']['input_sha256']==prior['input_sha256'],'frozen 2BJ matrix');require(v['report']['log_sha256']==digest(base64.b64decode(v['logs']['log']))and v['report']['observations_sha256']==digest(base64.b64decode(v['logs']['observations'])),'reference output hashes')
 model=r['reports']['model'];authority=load(ROOT/'docs/stage2bh-production-contract/contract-checks.json');require({k:v for k,v in model['model'].items()if k!='log_sha256'}=={k:v for k,v in authority.items()if k!='log_sha256'}and model['arithmetic']==load(ROOT/'docs/stage2bh-production-contract/arithmetic.json'),'model authority')
 for name,oldpath in [('callable','stage2bg-host-function-test/callable-diff.json'),('managed','stage2bf-managed-test/managed-diff.json'),('host','stage2be-host-test/host-diff.json'),('vm','stage2bd-runtime-test/vm-diff.json'),('adapter','stage2bc-adapter/adapter-diff.json'),('kernel','stage2bb-kernel/differential.json')]:
  current=r['reports']['six'][name];old=load(ROOT/('docs/'+oldpath));require(current['input_sha256']==old['input_sha256']and current['replay']==old['replay']and all(digest((ROOT/p).read_bytes())==h for p,h in current['source_sha256'].items()),'old driver authority '+name)
 return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l=run();(HERE/'reference-replay.json').write_bytes(encoded(r));(HERE/'reference-log.json.gz').write_bytes(l)
 print(json.dumps(verify(),sort_keys=True))
