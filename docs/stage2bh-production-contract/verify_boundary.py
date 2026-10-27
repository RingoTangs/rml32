"""All Rust and previous evidence frozen; only two historical-prefix additions allowed."""
import subprocess
from common import ROOT,HERE,COMMIT,digest,load,require
from contract_spec import verify as contract,gates,decisions
from check_contract import verify as checks
PREFIX={'README.md','docs/magic_Linux32-rust-rewrite-roadmap.md'}
FLAGS={k:v for k,v in gates().items()if k not in ('stage','native_unknowns','target_execution','limitations')}
FLAGS['historical_python_tests_run_on_current_source']=False

def baseline():
    b=load(HERE/'baseline-inputs.json');require(b['stage']=='2BH' and b['baseline_stage']=='2BG' and b['baseline_commit']==COMMIT and b['rust_source_changes_allowed']is False,'baseline identity')
    paths=subprocess.check_output(['git','ls-tree','-r','--name-only',COMMIT],cwd=ROOT).decode().splitlines();require(len(paths)==980 and set(paths)==set(b['files']) and set(b['allowed_prefixes'])==PREFIX,'baseline inventory')
    for p,h in b['files'].items():
        raw=(ROOT/p).read_bytes()
        if p in PREFIX:
            old=subprocess.check_output(['git','show',COMMIT+':'+p],cwd=ROOT);require(digest(old)==h and raw.endswith(old),'historical suffix '+p)
        else:require(digest(raw)==h,'frozen file changed '+p)
    current=set(subprocess.check_output(['rg','--files','src','tests'],cwd=ROOT).decode().splitlines());expected={p for p in paths if p.startswith(('src/','tests/'))};require(current==expected,'new source/fixture file forbidden')
    require(not any(p.suffix=='.rs'for p in HERE.rglob('*')),'design must not add Rust code');return b

def coverage():
    r=load(HERE/'case-coverage.json');p=ROOT/'docs/stage2bg-host-function-test/case-coverage.json';prior=load(p);require(r['prior_sha256']==digest(p.read_bytes()) and r['prior_coverage']==prior,'case prefix')
    while 'prior_coverage'in prior:prior=prior['prior_coverage']
    require(len(prior['prior_cases'])==115 and r['native_unknowns']==20 and r['native_unknowns_promoted']==0,'native scope')
    names=set(checks()['model_case_names']);require(set(r['model_case_names'])==names,'contract case inventory');require({x['decision']for x in r['decisions']}=={x['id']for x in decisions()['rows']} and all(x['tests'] and set(x['tests'])<=names for x in r['decisions']),'decision tests')
    require(r['rust_implementation_validated']is False and r['native_equivalence_proven']is False,'model evidence scope');return dict(cases=115,native_unknowns=20,contract_decisions=len(r['decisions']))
def verify():
    baseline();c=contract();r=checks();scope=coverage();return dict(stage='2BH',status='pass',baseline_files=980,byte_frozen_files=978,model_cases=r['model_cases'],arithmetic_rows=r['arithmetic_rows'],future_types=c['future_types'],**scope,target_execution='none',**FLAGS)
if __name__=='__main__':
    import json
    print(json.dumps(verify(),sort_keys=True))
