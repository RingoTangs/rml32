"""Freeze committed 2BK, including all old model/test/evidence bytes."""
import importlib.util,subprocess
from common import ROOT,HERE,COMMIT,load,require,digest

def baseline():
 b=load(HERE/'baseline-inputs.json');require(b['base_commit']==COMMIT and len(b['files'])==1132,'baseline identity');paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT,text=True).splitlines();require(set(paths)==set(b['files']),'baseline inventory')
 spec=importlib.util.spec_from_file_location('_2bl_projection',ROOT/'docs/stage2bg-host-function-test/source_projection.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);changed=[];bodies=0
 for n,h in b['files'].items():
  raw=(ROOT/n).read_bytes()
  if digest(raw)==h:continue
  changed.append(n);old=subprocess.check_output(['git','show',COMMIT+':'+n],cwd=ROOT);require(digest(old)==h,'baseline original bytes '+n)
  if n in b['allowed_prefixes']:require(raw.endswith(old),'historical prefix '+n)
  else:
   require(n in b['allowed_changes'],'frozen file changed '+n);require(m.cfg(m.tokens(raw.decode()))==m.cfg(m.tokens(old.decode())),'normal build changed '+n);before=m.test_bodies(old.decode());after=m.test_bodies(raw.decode());require(all(x in after for x in before),'old test body changed '+n);bodies+=len(before)
 require(set(changed)<=set(b['allowed_prefixes'])|{'src/boundary.rs'},'unexpected production mutation')
 expected={n for n in paths if n.startswith(('src/','tests/'))}|set(b['new_source']);actual=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT,text=True).splitlines());require(actual==expected,'source inventory')
 return dict(stage='2BL',status='pass',baseline_commit=COMMIT,baseline_files=1132,byte_frozen_files=1132-len(changed),changed_files=sorted(changed),normal_build_tokens_unchanged=True,old_rust_tests=863,old_doctests=5,preserved_test_bodies_in_changed_modules=bodies,native_unknowns=20,native_unknowns_promoted=0)
def verify():
 r=baseline();c=load(HERE/'case-coverage.json');prior=ROOT/'docs/stage2bk-combination-audit/case-coverage.json';require(c['prior_sha256']==digest(prior.read_bytes())and c['prior_coverage']==load(prior),'coverage prefix');require(len(c['new_test_names'])==len(set(c['new_test_names']))==6 and c['native_unknowns']==20 and c['native_unknowns_promoted']==0,'coverage scope');return r
if __name__=='__main__':
 import json
 print(json.dumps(verify(),sort_keys=True))
