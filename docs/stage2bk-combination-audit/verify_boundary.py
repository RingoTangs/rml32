"""All 2BJ bytes frozen except test hooks and documentation prefixes."""
import base64,importlib.util,subprocess
from common import ROOT,HERE,load,require,digest,encoded

def baseline():
 b=load(HERE/'baseline-inputs.json');raw=(HERE/'baseline-overlay.json.gz').read_bytes();require(len(b['files'])==1096 and b['overlay_sha256']==digest(raw),'snapshot identity');overlay=load(HERE/'baseline-overlay.json.gz');require(set(overlay)==set(b['overlay_files'])and all(digest(base64.b64decode(v,validate=True))==b['files'][n]for n,v in overlay.items()),'sealed overlay')
 spec=importlib.util.spec_from_file_location('_2bk_projection',ROOT/'docs/stage2bg-host-function-test/source_projection.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);changed=[];bodies=0
 for n,h in b['files'].items():
  current=(ROOT/n).read_bytes()
  if digest(current)==h:continue
  old=base64.b64decode(overlay[n])if n in overlay else subprocess.check_output(['git','show',b['base_commit']+':'+n],cwd=ROOT)
  require(digest(old)==h,'old bytes '+n);changed.append(n)
  if n in b['allowed_prefixes']:require(current.endswith(old),'historical prefix '+n)
  else:
   require(n in b['allowed_changes'],'frozen bytes changed '+n);before=m.test_bodies(old.decode());after=m.test_bodies(current.decode());require(all(t in after for t in before),'original Rust test bodies '+n);bodies+=len(before)
   # No production deviation has been found in this batch: only cfg(test) hooks.
   require(m.cfg(m.tokens(old.decode()))==m.cfg(m.tokens(current.decode())),'production tokens changed '+n)
 expected={n for n in b['files']if n.startswith(('src/','tests/'))}|set(b['new_source']);actual=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines());require(expected==actual,'source inventory')
 require(set(changed)<=set(b['allowed_prefixes'])|{'src/copy_plan.rs','src/boundary.rs'},'unexpected production change')
 return dict(stage='2BK',status='pass',baseline_files=1096,byte_frozen_files=1096-len(changed),changed_files=sorted(changed),preserved_test_bodies_in_changed_modules=bodies,normal_build_tokens_unchanged=True,old_rust_tests=857,old_doctests=5,native_unknowns=20,native_unknowns_promoted=0)

def verify():
 r=baseline();c=load(HERE/'case-coverage.json');prior=ROOT/'docs/stage2bj-contract-audit/case-coverage.json';require(c['prior_sha256']==digest(prior.read_bytes())and c['prior_coverage']==load(prior),'coverage prefix');require(c['native_unknowns']==20 and c['native_unknowns_promoted']==0 and len(c['new_test_names'])==len(set(c['new_test_names']))==6,'coverage inventory');return r
if __name__=='__main__':
 import json
 print(json.dumps(verify(),sort_keys=True))
