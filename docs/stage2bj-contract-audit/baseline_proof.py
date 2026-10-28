"""Reproduce both findings on the detached 2BI production source, then verify fixes."""
import argparse,gzip,importlib.util,json,os,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,COMMIT,digest,load,require,encoded
def tokenizer():
 spec=importlib.util.spec_from_file_location('_2bj_projection_proof',ROOT/'docs/stage2bg-host-function-test/source_projection.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def repaired():
 finding=load(HERE/'counterexamples.json')['findings'][0];rows=[json.loads(x)for x in gzip.decompress((HERE/'contract-observations.jsonl.gz').read_bytes()).splitlines()]
 actual=next(r for r in rows if r['id']==finding['trace']['id']and r['step']==7);require(actual['error_kind']=='InvalidObject'and actual['after']['semantic']==finding['expected'][7]['semantic'],'R01 repair no longer matches frozen oracle')
 return dict(id='D08-R01',trace=finding['trace']['id'],step=7,error_kind=actual['error_kind'])
def run():
 findings=load(HERE/'counterexamples.json')['findings'];fixture=load(HERE/'counterexample-fixtures.json.gz')['fixtures']['D08-R02'];require(digest(fixture['source'].encode())==findings[1]['fixture_sha256'],'R02 original fixture')
 projection=tokenizer();commands=[]
 with tempfile.TemporaryDirectory(prefix='rml32-2bj-counterexamples-')as temp:
  base=Path(temp)/'baseline';env=os.environ.copy();env['CARGO_TARGET_DIR']=str(Path(temp)/'target')
  def command(argv,cwd,expected=0):
   r=subprocess.run(argv,cwd=cwd,env=env,text=True,capture_output=True);commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr));require(r.returncode==expected,r.stdout+r.stderr);return r
  command(['git','clone','--shared','--dissociate','--no-checkout',str(ROOT),str(base)],ROOT);command(['git','checkout','--detach',COMMIT],base)
  files=load(HERE/'baseline-inputs.json')['files']
  for name,h in files.items():
   p=base/name;raw=p.read_bytes()
   if digest(raw)!=h:
    frozen=(ROOT/name).read_bytes();require(digest(frozen)==h and frozen.replace(b'\r\n',b'\n')==raw,'baseline line ending '+name);p.write_bytes(frozen)
  require(all(digest((base/p).read_bytes())==h for p,h in files.items()),'detached baseline identity')
  runtime=(ROOT/'src/lifecycle_runtime.rs').read_text();start=runtime.index('    /// Scalar-only observation;');hooks=runtime[start:runtime.index('    pub(crate) fn close(',start)]
  for name,addition in [('src/lifecycle_runtime.rs',hooks),('src/boundary.rs','\n#[cfg(test)]\n#[path = "lifecycle_contract_tests.rs"]\nmod lifecycle_contract_tests;\n')]:
   p=base/name;old=p.read_text();new=old.replace('    pub(crate) fn close(',addition+'    pub(crate) fn close(',1)if name.endswith('runtime.rs')else old+addition
   require(projection.cfg(projection.tokens(old))==projection.cfg(projection.tokens(new)),'observer changed baseline production tokens');p.write_text(new)
  driver=(ROOT/'src/lifecycle_contract_tests.rs').read_text();require(driver.count('pending.reservation.validate()?;')==1,'baseline driver validation seam');(base/'src/lifecycle_contract_tests.rs').write_text(driver.replace('pending.reservation.validate()?;',''))
  input_path=Path(temp)/'actions.jsonl';output_path=Path(temp)/'observations.jsonl';input_path.write_text(json.dumps(findings[0]['trace'])+'\n');env.update(RML32_D08_TRACES=str(input_path),RML32_D08_OBSERVATIONS=str(output_path))
  command(['cargo','test','--locked','--offline','--lib','boundary::lifecycle_contract_tests::replay_production_contract_traces','--','--ignored','--exact','--nocapture'],base)
  rows=[json.loads(x)for x in output_path.read_text().splitlines()];require(rows==findings[0]['pre_fix_observations'],'R01 baseline observations changed')
  (base/fixture['path']).write_text(fixture['source']);r=command(['cargo','test','--locked','--offline','--test','lifecycle_contract_combinations',findings[1]['test'],'--','--exact','--nocapture'],base,101);require('mixed slot origins'in r.stdout+r.stderr and 'InvalidState'in r.stdout+r.stderr,'R02 baseline failure changed')
  raw=gzip.compress(encoded(dict(stage='2BJ',commands=commands)),mtime=0)
  return dict(stage='2BJ',status='pass',baseline_commit=COMMIT,baseline_files_verified=len(files),production_tokens_unchanged_by_observers=True,findings_reproduced=['D08-R01','D08-R02'],expected_baseline_failure_commands=1,repaired=repaired(),log_sha256=digest(raw)),raw
def verify():
 r=load(HERE/'baseline-proof.json');raw=(HERE/'baseline-proof-log.json.gz').read_bytes();logs=load(HERE/'baseline-proof-log.json.gz')['commands'];require(r['stage']=='2BJ'and r['status']=='pass'and r['baseline_commit']==COMMIT and r['log_sha256']==digest(raw),'baseline proof identity')
 require(r['production_tokens_unchanged_by_observers']and r['findings_reproduced']==['D08-R01','D08-R02']and r['repaired']==repaired(),'baseline proof scope');require(len(logs)==4 and [c['returncode']for c in logs]==[0,0,0,101]and 'mixed slot origins'in logs[-1]['stdout']+logs[-1]['stderr'],'actual baseline commands');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,raw=run();(HERE/'baseline-proof.json').write_bytes(encoded(r));(HERE/'baseline-proof-log.json.gz').write_bytes(raw)
 print(json.dumps(verify(),sort_keys=True))
