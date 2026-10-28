"""497 Python tests on the sealed 2BJ overlay, including its nested 465."""
import argparse,gzip,json,base64,subprocess,tempfile,re
from pathlib import Path
from common import ROOT,HERE,restore,load,require,digest,encoded
CODE="import sys,json,base64;sys.path.insert(0,'docs/stage2bj-contract-audit');import historical;r,raw=historical.run();print(json.dumps(dict(report=r,logs_base64=base64.b64encode(raw).decode()),sort_keys=True))"
def run():
 logs=[];b=load(HERE/'baseline-inputs.json')
 with tempfile.TemporaryDirectory(prefix='rml32-2bk-history-')as tmp:
  clone=Path(tmp)/'sealed'
  def cmd(argv,cwd):
   r=subprocess.run(argv,cwd=cwd,text=True,capture_output=True);logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr));require(r.returncode==0,r.stdout+r.stderr);return r
  cmd(['git','clone','--shared','--dissociate','--no-checkout',str(ROOT),str(clone)],ROOT);cmd(['git','checkout','--detach',b['base_commit']],clone);restore(clone)
  child=json.loads(cmd(['python3','-B','-c',CODE],clone).stdout);require(child['report']['historical_python_tests']==465,'nested scope')
  r=cmd(['python3','-B','-m','unittest','discover','-s','docs/stage2bj-contract-audit','-p','test_*.py'],clone);require(re.findall(r'^Ran (\d+) tests? in ',r.stdout+r.stderr,re.M)==['32']and(r.stdout+r.stderr).rstrip().endswith('OK'),'outer checks')
  cmd(['python3','-B','docs/stage2bj-contract-audit/verify_validation.py'],clone)
 raw=gzip.compress(encoded(logs),mtime=0)
 return dict(stage='2BK',status='pass',snapshot_files=1096,base_commit=b['base_commit'],snapshot_sha256=digest(encoded(b['files'])),overlay_sha256=b['overlay_sha256'],historical_python_tests=497,outer_python_tests=32,nested_python_tests=465,execution_location='sealed 2BJ overlay on detached base plus its pinned nested authorities; not current source',network_used=False,log_sha256=digest(raw)),raw
def verify():
 r=load(HERE/'historical.json');raw=(HERE/'historical-log.json.gz').read_bytes();logs=load(HERE/'historical-log.json.gz');require(r['stage']=='2BK'and r['status']=='pass'and r['historical_python_tests']==497 and r['snapshot_files']==1096 and r['log_sha256']==digest(raw),'historical report');require(len(logs)==5 and all(c['returncode']==0 for c in logs),'historical command inventory');child=json.loads(logs[2]['stdout']);require(child['report']['historical_python_tests']==465 and digest(base64.b64decode(child['logs_base64'],validate=True))==child['report']['log_sha256'],'nested logs');require(re.findall(r'^Ran (\d+) tests? in ',logs[3]['stdout']+logs[3]['stderr'],re.M)==['32'],'outer count');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l=run();(HERE/'historical.json').write_bytes(encoded(r));(HERE/'historical-log.json.gz').write_bytes(l)
 print(json.dumps(verify(),sort_keys=True))
