"""525 checks on the pinned committed 2BK, never on current sources."""
import argparse,base64,gzip,json,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,COMMIT,load,require,digest,encoded
CODE="import sys,json,base64;sys.path.insert(0,'docs/stage2bk-combination-audit');import historical;r,l=historical.run();print(json.dumps(dict(report=r,logs_base64=base64.b64encode(l).decode()),sort_keys=True))"
def run():
 logs=[];b=load(HERE/'baseline-inputs.json')
 with tempfile.TemporaryDirectory(prefix='rml32-2bl-history-')as tmp:
  clone=Path(tmp)/'baseline'
  def command(argv,cwd):
   r=subprocess.run(argv,cwd=cwd,text=True,capture_output=True);logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr));require(r.returncode==0,r.stdout+r.stderr);return r
  command(['git','clone','--shared','--dissociate','--no-checkout',str(ROOT),str(clone)],ROOT);command(['git','checkout','--detach',COMMIT],clone)
  for n,h in b['files'].items():
   p=clone/n;raw=p.read_bytes()
   if digest(raw)!=h:
    frozen=(ROOT/n).read_bytes();require(digest(frozen)==h and frozen.replace(b'\r\n',b'\n')==raw,'baseline line ending '+n);p.write_bytes(frozen)
  require(all(digest((clone/n).read_bytes())==h for n,h in b['files'].items()),'historical snapshot')
  child=json.loads(command(['python3','-B','-c',CODE],clone).stdout);require(child['report']['historical_python_tests']==497,'nested Python count')
  r=command(['python3','-B','-m','unittest','discover','-s','docs/stage2bk-combination-audit','-p','test_*.py'],clone);require(re.findall(r'^Ran (\d+) tests? in ',r.stdout+r.stderr,re.M)==['28']and(r.stdout+r.stderr).rstrip().endswith('OK'),'outer Python count')
  command(['python3','-B','docs/stage2bk-combination-audit/verify_validation.py'],clone)
 raw=gzip.compress(encoded(logs),mtime=0);return dict(stage='2BL',status='pass',baseline_commit=COMMIT,baseline_files=1132,files_sha256=digest(encoded(b['files'])),historical_python_tests=525,outer_python_tests=28,nested_python_tests=497,execution_location='detached pinned 2BK and its nested fixed authorities; not current source',network_used=False,log_sha256=digest(raw)),raw
def verify():
 r=load(HERE/'historical.json');raw=(HERE/'historical-log.json.gz').read_bytes();logs=load(HERE/'historical-log.json.gz');require(r['stage']=='2BL'and r['status']=='pass'and r['historical_python_tests']==525 and r['baseline_commit']==COMMIT and r['files_sha256']==digest(encoded(load(HERE/'baseline-inputs.json')['files']))and r['log_sha256']==digest(raw),'historical identity');require(len(logs)==5 and all(c['returncode']==0 for c in logs),'historical commands');v=json.loads(logs[2]['stdout']);require(v['report']['historical_python_tests']==497 and digest(base64.b64decode(v['logs_base64'],validate=True))==v['report']['log_sha256'],'nested history');require(re.findall(r'^Ran (\d+) tests? in ',logs[3]['stdout']+logs[3]['stderr'],re.M)==['28'],'historical outer checks');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l=run();(HERE/'historical.json').write_bytes(encoded(r));(HERE/'historical-log.json.gz').write_bytes(l)
 print(json.dumps(verify(),sort_keys=True))
