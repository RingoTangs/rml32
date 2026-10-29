import argparse
import copy
import gzip
import hashlib
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path
from common import HERE, ROOT, authority, digest, encoded, load, require
from matrix import traces, directed, metadata_cases
from interleaving_oracle import Reference, Rejected

maintenance = authority('_2bm_maintenance', 'docs/stage2bj-contract-audit/maintenance_oracle.py')
DRIVER = 'boundary::lifecycle_interleaving_tests::replay_interleaving_traces'

def observations(path):
    states = {}
    opener = gzip.open if path.suffix == '.gz' else open
    with opener(path, 'rt') as stream:
        for line in stream:
            row = json.loads(line)
            if 'state' in row:
                require(row['state'] == len(states), 'state pool order')
                states[row['state']] = row['value']
            else:
                require(row['before'] in states and row['after'] in states, 'undefined observation')
                yield dict(row, before=states[row['before']], after=states[row['after']])

def semantic(value): return value.get('semantic', value)

def input_digest(cases):
    result=hashlib.sha256()
    for trace in cases: result.update((json.dumps(trace,sort_keys=True)+'\n').encode())
    return result.hexdigest()

def compare(cases, rows, save_failure=False):
    iterator = iter(rows)
    counts = dict(traces=0,rows=0,setup_rejected=0,prepared=0,committed=0,aborted=0,rejected=0)
    families = {}
    states = CounterStates()
    calibration = []
    for trace in cases:
        family = families.setdefault(trace['family'], dict(traces=0,setup_rejected=0,prepared=0,committed=0,aborted=0,rejected=0))
        counts['traces'] += 1; family['traces'] += 1
        oracle = Reference(trace['limits'])
        meta = maintenance.MaintenanceOracle()
        last_actual = None
        def check(step, before, error, setup_step=None, action=None):
            nonlocal last_actual
            row = next(iterator, None)
            require(row is not None and (row['id'],row['step']) == (trace['id'],step), 'checkpoint inventory')
            try:
                if last_actual is not None: require(row['before']==last_actual,'observation continuity')
                require(semantic(row['before']) == before, 'before semantic mismatch')
                require(row['error'] == error and row['setup_step'] == setup_step, 'result/phase mismatch')
                if action and action['op'] == 'maintain' and action.get('metadata',0):
                    a = dict(action,budget=dict(host_roots=0,objects=0,metadata_entries=action['metadata']))
                    b = copy.deepcopy(row['before']); b['semantic']['pending_roots'] = 0
                    meta.check(a,b,row['after'],row['progress'])
                    # Eligibility is corroborated independently for Object rows;
                    # actual physical head liveness is a concrete refinement.
                    for n, data in before['objects'].items():
                        if data is None: continue
                        identity = list(oracle.known_objects_order()).index(n)
                        eligible = next(r['eligible'] for r in row['before']['raw']['metadata'] if r['phase']==0 and r['key']==identity)
                        require(eligible == (data['state']=='freed'), 'object eligibility')
                        if str(identity) not in row['after']['raw']['objects']: del oracle.objects[n]
                    # Project validated head reclamation; labelled maps preserve
                    # tombstones, just as the real driver preserves weak labels.
                    for n,h in list(oracle.heads.items()):
                        if row['after']['semantic']['headers'].get(n) is None: del oracle.heads[n]
                require(semantic(row['after']) == oracle.projection(), 'after semantic mismatch')
                if action and action['op']=='zero': require(row['before']==row['after'], 'zero budget mutation')
                if action and action['op']=='maintain' and not action.get('metadata',0):
                    p = row['progress']; drained = len(before['queue'])-len(oracle.queue)
                    require(p['drained_objects']==drained and p['queued_objects']==len(oracle.queue), 'object maintenance progress')
                    require(all(p[k]==0 for k in ('retired_roots','reaped_objects','reclaimed_heads','pruned_weak_entries','metadata_examined','pending_roots')), 'independent budgets')
                    require(p['stop']==('BorrowBlocked' if action.get('borrow') and before['queue'] and drained<action.get('objects',0) else 'WorkspaceLimit' if action.get('workspace_blocked') else 'BudgetExhausted'), 'maintenance stop')
            except (AssertionError, KeyError, StopIteration) as exc:
                finding=dict(trace=trace,step=step,action=action,expected_error=error,expected=oracle.projection(),actual=row,message=str(exc))
                if save_failure: (HERE/'unclassified-counterexample.json').write_bytes(encoded(finding))
                raise AssertionError(trace['id']+f' step {step}: '+str(exc)+'\n'+json.dumps(finding,sort_keys=True)) from exc
            counts['rows'] += 1
            last_actual = row['after']
            for obj in semantic(row['after'])['objects'].values():
                if obj: states.add(obj['state'])
            return row
        before = oracle.projection(); error = index = None
        for i,action in enumerate(trace['setup']):
            try: oracle.apply(action)
            except Rejected as e: error=e.kind;index=i;break
        check(0,before,error,index)
        if error:
            counts['setup_rejected']+=1;family['setup_rejected']+=1
            continue
        for i,action in enumerate(trace['actions']):
            before = oracle.projection();error=None
            try:
                if 'fail_after' in action and action['fail_after'] < trace['control_calls'] and action['op']!='maintain':
                    # Only calibrated successful atomic actions are injected.
                    if action['op']=='commit': oracle.plan=None;oracle.slots.pop('tmp:move',None)
                    raise Rejected('Limit')
                oracle.apply(action)
            except Rejected as e: error=e.kind
            row = check(i+1,before,error,action=action)
            op=action['op']
            key='prepared' if op.startswith('prepare') else 'committed' if op in ('commit','commit_head','commit_exec') else 'aborted' if op in ('abort','abort_head','abort_exec') else None
            if key and error is None: counts[key]+=1;family[key]+=1
            if error: counts['rejected']+=1;family['rejected']+=1
            if error is None and (op in ('commit','delete','prepare_head') or op=='maintain' and action.get('objects')==1 and row['progress']['drained_objects']==1) and 'control_calls' not in trace:
                signature=tuple((n,None if o is None else o['state']) for n,o in sorted(before['objects'].items()))
                calibration.append((trace,i,row['calls'],signature,before['queue']))
            if error and not action.get('continue_on_error'): break
    require(next(iterator,None) is None, 'extra checkpoint')
    return dict(**counts,families=families,states=states.values,unclassified_differences=0),calibration

class CounterStates:
    def __init__(self): self.values={}
    def add(self,n): self.values[n]=self.values.get(n,0)+1

def fault_cases(calibration):
    seen=set()
    for trace,i,calls,signature,queue in calibration:
        key=(trace['family'],trace['actions'][i]['op'],tuple(sorted(trace['limits'].items())),signature)
        if key in seen or not calls: continue
        seen.add(key)
        require(calls<=128,'workspace bound')
        for cut in range(calls+1):
            case=copy.deepcopy(trace);case['id']+=f'-fault-{i}-{cut}';case['control_calls']=calls
            case['actions']=case['actions'][:i+1];case['actions'][-1]['fail_after']=cut
            if case['actions'][-1]['op']=='maintain' and cut<calls: case['actions'][-1]['workspace_blocked']=True
            yield case

def borrow_cases(calibration):
    seen=set()
    for trace,i,_,signature,queue in calibration:
        if trace['limits'] != __import__('matrix').HIGH: continue
        action=trace['actions'][i]
        if action['op'] not in ('commit','delete','maintain'): continue
        key=(trace['family'],action['op'],signature)
        if key in seen: continue
        seen.add(key)
        for target in (['d0','m:h0'] if action['op']=='commit' else ['v:'+action['object']] if action['op']=='delete' else ['v:'+queue[0]]):
            if target=='m:h0' and not any(a.get('id')=='h0' for a in trace['setup']): continue
            case=copy.deepcopy(trace);case['id']+=f'-borrow-{i}-{target}';case['actions']=case['actions'][:i+1];case['actions'][-1]['borrow']=target
            yield case

def execute(cases,profile):
    with tempfile.TemporaryDirectory(prefix='rml32-2bm-replay-') as tmp:
        p=Path(tmp);inp=p/'in';out=p/'out'
        with inp.open('w') as f:
            for trace in cases: f.write(json.dumps(trace,sort_keys=True)+'\n')
        env=os.environ.copy();env.update(RML32_INTERLEAVINGS_IN=str(inp),RML32_INTERLEAVINGS_OUT=str(out))
        argv=['cargo','test']+(['--release'] if profile=='release' else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture']
        r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True)
        require(r.returncode==0,r.stdout+r.stderr)
        summary,calibration=compare(cases,observations(out),True)
        totals=re.findall(r'^INTERLEAVING_REPLAY (.+)$',r.stdout,re.M)
        require(len(totals)==1 and json.loads(totals[0])['traces']==summary['traces'] and json.loads(totals[0])['rows']==summary['rows'],'Rust totals')
        raw=gzip.compress(out.read_bytes(),mtime=0)
        return summary,calibration,dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(inp.read_bytes()),observations_sha256=digest(raw)),raw

def run(quick=False):
    reports={};logs=[];observed={}
    for profile in ('debug','release'):
        calibration=[]
        groups=[('core',list(traces(quick))),('directed',list(directed(quick))),('metadata',list(metadata_cases()))]
        for name,cases in groups:
            s,c,l,o=execute(cases,profile);reports[name]=s;calibration+=c;logs.append(l)
            require(name not in observed or observed[name]==o,'profile observations '+name);observed[name]=o
        if not quick:
            for name,cases in [('borrow',list(borrow_cases(calibration))),('fault',list(fault_cases(calibration)))]:
                s,_,l,o=execute(cases,profile);reports[name]=s;logs.append(l)
                require(name not in observed or observed[name]==o,'profile observations '+name);observed[name]=o
    raw=gzip.compress(encoded(logs),mtime=0)
    return dict(stage='2BM',status='pass',quick=quick,reports=reports,log_sha256=digest(raw),observation_sha256={k:digest(v) for k,v in observed.items()}),raw,observed

def verify():
    r=load(HERE/'interleaving-diff.json');require(r['stage']=='2BM' and r['status']=='pass' and not r['quick'],'diff identity')
    calibration=[]
    inputs={}
    for name,cases in [('core',list(traces())),('directed',list(directed())),('metadata',list(metadata_cases()))]:
        inputs[name]=input_digest(cases)
        s,c=compare(cases,observations(HERE/(name+'-observations.jsonl.gz')));require(s==r['reports'][name],'recomputed '+name);calibration+=c
    for name,cases in [('borrow',list(borrow_cases(calibration))),('fault',list(fault_cases(calibration)))]:
        inputs[name]=input_digest(cases)
        s,_=compare(cases,observations(HERE/(name+'-observations.jsonl.gz')));require(s==r['reports'][name],'recomputed '+name)
    for name,h in r['observation_sha256'].items():require(digest((HERE/(name+'-observations.jsonl.gz')).read_bytes())==h,'observation hash')
    logs=load(HERE/'interleaving-log.json.gz');require(len(logs)==10 and all(l['returncode']==0 for l in logs),'diff commands')
    for log,(profile,name) in zip(logs,[(p,n) for p in ('debug','release') for n in ('core','directed','metadata','borrow','fault')]):
        argv=['cargo','test']+(['--release'] if profile=='release' else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture']
        require(log['profile']==profile and log['argv']==argv and log['input_sha256']==inputs[name],'replay command/input identity')
        require(log['observations_sha256']==r['observation_sha256'][name],'per-command observation hash')
        totals=re.findall(r'^INTERLEAVING_REPLAY (.+)$',log['stdout'],re.M)
        require(len(totals)==1 and all(json.loads(totals[0])[k]==r['reports'][name][k] for k in ('traces','rows')),'saved Rust totals')
    require(digest((HERE/'interleaving-log.json.gz').read_bytes())==r['log_sha256'],'diff log hash')
    return r

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');p.add_argument('--quick',action='store_true');args=p.parse_args()
    if args.write or args.quick:
        r,l,obs=run(args.quick)
        if args.write:
            (HERE/'interleaving-diff.json').write_bytes(encoded(r));(HERE/'interleaving-log.json.gz').write_bytes(l)
            for n,raw in obs.items():(HERE/(n+'-observations.jsonl.gz')).write_bytes(raw)
        print(json.dumps(r,sort_keys=True))
    else: print(json.dumps(verify(),sort_keys=True))
