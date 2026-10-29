import argparse,copy,gzip,hashlib,json,os,re,subprocess,tempfile
from pathlib import Path
from common import HERE,ROOT,authority,digest,encoded,load,require
from matrix import HIGH,core,interferences,lifetime_cases,metadata_cases
from pair_oracle import Reference,Rejected
maintenance=authority('_2bn_maintenance','docs/stage2bj-contract-audit/maintenance_oracle.py')
DRIVER='boundary::lifecycle_pair_tests::replay_pair_traces'

def observations(path):
    states={}
    with (gzip.open(path,'rt') if path.suffix=='.gz' else path.open()) as f:
        for line in f:
            row=json.loads(line)
            if 'state' in row:
                require(row['state']==len(states),'state pool order');states[row['state']]=row['value']
            else:
                require(row['before'] in states and row['after'] in states,'undefined state')
                yield dict(row,before=states[row['before']],after=states[row['after']])
def sem(v):return v.get('semantic',v)
def input_digest(cases):
    h=hashlib.sha256()
    for t in cases:h.update((json.dumps(t,sort_keys=True)+'\n').encode())
    return h.hexdigest()

def compare(cases,rows,save_failure=False):
    it=iter(rows);calibration=[]
    total=dict(traces=0,rows=0,setup_rejected=0,prepared=0,committed=0,aborted=0,rejected=0,workspace_blocked=0)
    families={};pairs={};state_counts={};overlap=0
    for t in cases:
        total['traces']+=1;counts=families.setdefault(t['family'],dict(traces=0,prepared=0,committed=0,aborted=0,rejected=0));counts['traces']+=1
        pair=pairs.setdefault(t['pair'],dict(traces=0,prepared=0,committed=0,aborted=0,rejected=0));pair['traces']+=1
        r=Reference(t['limits']);meta=maintenance.MaintenanceOracle();previous=None;stop=False
        for phase in ('setup','actions'):
            if stop:break
            for step,a in enumerate(t[phase]):
                row=next(it,None);require(row is not None and (row['id'],row['phase'],row['step'])==(t['id'],phase,step),'checkpoint inventory')
                before=r.projection();old_heads=copy.deepcopy(r.heads);old_physical=set(r.physical);error=None
                kind=r.transactions.get(a.get('tx'),{}).get('kind',a.get('kind'))
                try:
                    if 'fail_after' in a and a['fail_after']<t['control_calls'] and a['op']!='maintain':
                        if a['op']=='commit':r.consume(a['tx'])
                        raise Rejected('Limit')
                    r.apply(a)
                except Rejected as e:error=e.kind
                try:
                    require(sem(row['before'])==before,'before semantic mismatch')
                    if previous is not None:require(previous==row['before'],'observation continuity')
                    require(row['error']==error,'result mismatch')
                    if a['op']=='maintain' and a.get('metadata',0) and error is None:
                        b=copy.deepcopy(row['before']);b['semantic']['pending_roots']=0
                        meta.check(dict(a,budget=dict(host_roots=0,objects=0,metadata_entries=a['metadata'])),b,row['after'],row['progress'])
                        raw=row['before']['raw'];identities=row['before']['identities']
                        eligible={(x['phase'],x['key']):x['eligible'] for x in raw['metadata']}
                        for n,ident in identities['heads'].items():
                            if n in old_heads and old_heads[n]['kind']=='function' and (1,ident) in eligible:
                                require(eligible[1,ident]==(old_heads[n]['retired'] and n not in old_physical),'Function physical eligibility')
                        for n,ident in identities['cells'].items():
                            if (2,ident) in eligible:require(eligible[2,ident]==(before['cells'][n] is None),'Weak Cell eligibility')
                        for index,n in enumerate(r.known_objects):
                            if n not in r.objects:continue
                            require(eligible[0,index]==(r.objects[n]['state']=='freed'),'Object eligibility')
                            if str(index) not in row['after']['raw']['objects']:del r.objects[n]
                        for n in list(r.heads):
                            if row['after']['semantic']['headers'][n] is None:del r.heads[n]
                        r.settle()
                    require(sem(row['after'])==r.projection(),'after semantic mismatch')
                    if a['op']=='zero':require(row['before']==row['after'],'zero mutation')
                    if a['op']=='maintain' and not a.get('metadata',0) and error is None:
                        p=row['progress'];drained=len(before['queue'])-len(r.queue)
                        require(p['drained_objects']==drained and p['queued_objects']==len(r.queue),'drain progress')
                        require(all(p[k]==0 for k in ('retired_roots','reaped_objects','reclaimed_heads','pruned_weak_entries','metadata_examined','pending_roots')),'independent maintenance budgets')
                        expected='WorkspaceLimit' if a.get('workspace_blocked') else 'BorrowBlocked' if a.get('borrow') and drained<a.get('objects',0) and before['queue'] else 'BudgetExhausted'
                        require(p['stop']==expected,'maintenance stop')
                except (AssertionError,KeyError) as e:
                    finding=dict(trace=t,phase=phase,step=step,action=a,expected=r.projection(),expected_error=error,actual=row,message=str(e))
                    if save_failure:(HERE/'unclassified-counterexample.json').write_bytes(encoded(finding))
                    raise AssertionError(t['id']+f' {phase}:{step}: '+str(e)+'\n'+json.dumps(finding,sort_keys=True)) from e
                previous=row['after'];total['rows']+=1
                for obj in r.objects.values():state_counts[obj['state']]=state_counts.get(obj['state'],0)+1
                overlap+=len(r.transactions)==2
                key='prepared' if a['op']=='prepare' else 'committed' if a['op']=='commit' else 'aborted' if a['op']=='abort' else None
                if key and error is None and phase=='actions':
                    for c in (total,counts,pair):c[key]+=1
                if error:
                    for c in (total,counts,pair):c['rejected']+=1
                    total['setup_rejected']+=phase=='setup'
                if row['progress'] and row['progress']['stop']=='WorkspaceLimit':total['workspace_blocked']+=1
                if phase=='actions' and error is None and 'control_calls' not in t and (a['op'] in ('prepare','commit','delete') or a['op']=='maintain' and a.get('objects')==1 and row['progress']['drained_objects']==1):
                    signature=digest(json.dumps(before,sort_keys=True).encode())
                    calibration.append((t,step,row['calls'],kind,signature,before))
                if error and (phase=='setup' or not a.get('continue_on_error')):stop=True;break
    require(next(it,None) is None,'extra checkpoint')
    return dict(**total,families=families,pairs=pairs,states=state_counts,two_preparations_checkpoints=overlap,unclassified_differences=0),calibration

def faults(calibration):
    seen=set()
    for t,i,calls,kind,sig,before in calibration:
        if not calls:continue
        key=(t['family'],t['actions'][i]['op'],kind,tuple(sorted(t['limits'].items())),sig)
        if key in seen:continue
        seen.add(key);require(calls<=128,'workspace calibration bound')
        for cut in range(calls+1):
            c=copy.deepcopy(t);c['id']+=f'-F{i}-{cut}';c['control_calls']=calls;c['actions']=c['actions'][:i+1];c['actions'][-1]['fail_after']=cut
            if c['actions'][-1]['op']=='maintain' and cut<calls:c['actions'][-1]['workspace_blocked']=True
            yield c

def borrows(calibration):
    seen=set()
    for t,i,_,kind,_,before in calibration:
        if t['limits']!=HIGH or t['actions'][i]['op'] not in ('commit','delete','maintain'):continue
        op=t['actions'][i]['op']
        if op=='commit' and kind!='copy':continue
        signature=(t['family'],op,kind,tuple(sorted(before['transactions'].items())),tuple((n,x['state'] if x else None) for n,x in sorted(before['objects'].items())),tuple(sorted(before['roots'])))
        if signature in seen:continue
        seen.add(signature)
        targets=['d0','m:h0'] if op=='commit' else ['v:'+t['actions'][i]['object']] if op=='delete' else ['v:'+before['queue'][0]]
        for target in targets:
            if before['cells'].get(target) is None:continue
            c=copy.deepcopy(t);c['id']+=f'-B{i}-{target}';c['actions']=c['actions'][:i+1];c['actions'][-1]['borrow']=target
            yield c

def execute(cases,profile):
    with tempfile.TemporaryDirectory(prefix='rml32-2bn-pair-') as tmp:
        p=Path(tmp);inp=p/'in';out=p/'out'
        with inp.open('w') as f:
            for t in cases:f.write(json.dumps(t,sort_keys=True)+'\n')
        env=os.environ.copy();env.update(RML32_PAIRS_IN=str(inp),RML32_PAIRS_OUT=str(out))
        argv=['cargo','test']+(['--release'] if profile=='release' else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture']
        result=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True)
        require(result.returncode==0,result.stdout+result.stderr)
        raw=gzip.compress(out.read_bytes(),mtime=0)
        cache=ROOT/'target/stage2bn-replay-cache';cache.mkdir(exist_ok=True)
        ident=input_digest(cases)+'-'+profile
        (cache/(ident+'.jsonl.gz')).write_bytes(raw)
        snapshot={str(f.relative_to(ROOT)):digest(f.read_bytes()) for f in sorted((ROOT/'src').glob('*.rs'))}
        (cache/(ident+'.json')).write_bytes(encoded(dict(profile=profile,argv=argv,returncode=result.returncode,stdout=result.stdout,stderr=result.stderr,input_sha256=input_digest(cases),source_sha256=snapshot,observations_sha256=digest(raw))))
        summary,calibration=compare(cases,observations(out),True)
        totals=re.findall(r'^PAIR_REPLAY (.+)$',result.stdout,re.M);require(len(totals)==1 and json.loads(totals[0])=={k:summary[k] for k in ('traces','rows')},'Rust totals')
        raw=gzip.compress(out.read_bytes(),mtime=0)
        return summary,calibration,dict(profile=profile,argv=argv,returncode=result.returncode,stdout=result.stdout,stderr=result.stderr,input_sha256=digest(inp.read_bytes()),observations_sha256=digest(raw),source_sha256=snapshot),raw

def run(quick=False):
    reports={};logs=[];obs={}
    for profile in ('debug','release'):
        calibration=[]
        for name,cases in [('core',list(core(quick))),('interference',list(interferences(quick))),('lifetime',list(lifetime_cases())),('metadata',list(metadata_cases()))]:
            print(f'2BN {profile} {name}: {len(cases)} traces',file=__import__('sys').stderr,flush=True)
            s,c,l,o=execute(cases,profile);reports[name]=s;calibration+=c;logs.append(l)
            require(name not in obs or obs[name]==o,'profile observations '+name);obs[name]=o
        if not quick:
            for name,cases in [('borrow',list(borrows(calibration))),('fault',list(faults(calibration)))]:
                print(f'2BN {profile} {name}: {len(cases)} traces',file=__import__('sys').stderr,flush=True)
                s,_,l,o=execute(cases,profile);reports[name]=s;logs.append(l);require(name not in obs or obs[name]==o,'profile observations '+name);obs[name]=o
    raw=gzip.compress(encoded(logs),mtime=0)
    return dict(stage='2BN',status='pass',quick=quick,reports=reports,log_sha256=digest(raw),observation_sha256={n:digest(o) for n,o in obs.items()}),raw,obs

def verify():
    r=load(HERE/'pair-diff.json');require(r['stage']=='2BN' and r['status']=='pass' and not r['quick'],'pair identity')
    inputs={};calibration=[]
    groups=[('core',list(core())),('interference',list(interferences())),('lifetime',list(lifetime_cases())),('metadata',list(metadata_cases()))]
    for name,cases in groups:
        s,c=compare(cases,observations(HERE/(name+'-observations.jsonl.gz')));require(s==r['reports'][name],'recomputed '+name);inputs[name]=input_digest(cases);calibration+=c
    for name,cases in [('borrow',list(borrows(calibration))),('fault',list(faults(calibration)))]:
        s,_=compare(cases,observations(HERE/(name+'-observations.jsonl.gz')));require(s==r['reports'][name],'recomputed '+name);inputs[name]=input_digest(cases)
    logs=load(HERE/'pair-log.json.gz');require(len(logs)==12 and digest((HERE/'pair-log.json.gz').read_bytes())==r['log_sha256'],'pair logs')
    source={str(f.relative_to(ROOT)):digest(f.read_bytes()) for f in sorted((ROOT/'src').glob('*.rs'))}
    for log,(p,n) in zip(logs,[(p,n) for p in ('debug','release') for n in ('core','interference','lifetime','metadata','borrow','fault')]):
        argv=['cargo','test']+(['--release'] if p=='release' else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture']
        require(log['profile']==p and log['argv']==argv and log['returncode']==0 and log['input_sha256']==inputs[n],'pair command/input')
        require(log['source_sha256']==source,'executed Rust source identity')
        totals=re.findall(r'^PAIR_REPLAY (.+)$',log['stdout'],re.M)
        require(len(totals)==1 and json.loads(totals[0])=={k:r['reports'][n][k] for k in ('traces','rows')},'saved Rust totals')
        require(log['observations_sha256']==r['observation_sha256'][n]==digest((HERE/(n+'-observations.jsonl.gz')).read_bytes()),'pair observation hashes')
    return r

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');p.add_argument('--quick',action='store_true');args=p.parse_args()
    if args.write or args.quick:
        r,l,obs=run(args.quick)
        if args.write:
            (HERE/'pair-diff.json').write_bytes(encoded(r));(HERE/'pair-log.json.gz').write_bytes(l)
            for n,o in obs.items():(HERE/(n+'-observations.jsonl.gz')).write_bytes(o)
        print(json.dumps(r,sort_keys=True))
    else:print(json.dumps(verify(),sort_keys=True))
