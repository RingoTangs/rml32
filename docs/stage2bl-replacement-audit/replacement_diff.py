import argparse,copy,gzip,json,os,re,subprocess,tempfile,itertools
from pathlib import Path
from common import ROOT,HERE,require,digest,encoded,load
from matrix import traces,borrow_cases
from replacement_oracle import Reference,Rejected
DRIVER='boundary::lifecycle_replacement_tests::replay_replacement_traces'
def observations(path):
 states={}
 opener=gzip.open if path.suffix=='.gz'else open
 with opener(path,'rt')as f:
  for line in f:
   row=json.loads(line)
   if 'state'in row:
    require(row['state']==len(states),'state pool sequence');states[row['state']]=row['value'];continue
   require(row['before']in states and row['after']in states,'missing actual state');yield dict(row,before=states[row['before']],after=states[row['after']])
def compare(cases,rows,save_failure=False):
 it=iter(rows);cache={};counts=dict(traces=0,rows=0,setup_rejected=0,target_reached=0,accepted=0,rejected=0);families={};calibration=[]
 for t in cases:
  counts['traces']+=1;family=families.setdefault(t['family'],dict(traces=0,setup_rejected=0,prepared=0,committed=0,aborted=0,rejected=0));family['traces']+=1;oracle=Reference(t['limits']);seen=[]
  def checkpoint(step,error,index=None):
   row=next(it,None);require(row is not None and row['id']==t['id']and row['step']==step,'checkpoint inventory');seen.append(row)
   try:
    require(row['before']==before,'before semantic mismatch');require(row['error']==error and row['setup_step']==index,'result/phase mismatch');require(row['after']==oracle.projection(),'after semantic mismatch')
   except AssertionError as exc:
    finding=dict(trace=t,step=step,expected_error=error,expected_state=oracle.projection(),actual=row,observations=seen,message=str(exc))
    if save_failure:(HERE/'unclassified-counterexample.json').write_bytes(encoded(finding))
    raise AssertionError(t['id']+f' step {step}: '+str(exc)+'\n'+json.dumps(finding,sort_keys=True))from exc
   counts['rows']+=1;counts['accepted'if error is None else 'rejected']+=1;return row
  before=oracle.projection();key=(t['family'],tuple(sorted(t['limits'].items())));error=index=None
  if key not in cache:
   for i,a in enumerate(t['setup']):
    try:oracle.apply(a)
    except Rejected as e:error=e.kind;index=i;break
   cache[key]=(copy.deepcopy(oracle),error,index)
  else:oracle,error,index=copy.deepcopy(cache[key])
  checkpoint(0,error,index)
  if error:counts['setup_rejected']+=1;family['setup_rejected']+=1;continue
  for i,a in enumerate(t['actions']):
   before=oracle.projection();error=None
   try:
    if a.get('fail_after')is not None:
     # Controls are calibrated from the un-injected action. Failure cuts change
     # workspace availability only; no semantic publication is permitted.
     control=copy.deepcopy(oracle);control.apply({k:v for k,v in a.items()if k!='fail_after'})
     if a['fail_after']<t['control_calls']:raise Rejected('Limit')
    oracle.apply(a)
   except Rejected as e:error=e.kind
   row=checkpoint(i+1,error)
   if a['op']=='prepare':counts['target_reached']+=1;family['prepared']+=error is None
   if a['op']in ('commit','abort'):
    family['rejected'if error else 'committed'if a['op']=='commit'else 'aborted']+=1
    if a['op']=='commit'and error is None and 'control_calls'not in t:calibration.append((t,row['calls']))
   if error:break
 require(next(it,None)is None,'extra checkpoint')
 return dict(**counts,families=families,unclassified_differences=0),calibration

def fault_cases(calibration):
 for t,n in calibration:
  # Calibrate one zero-interference success per family+quota and cover all real
  # workspace calls there; other successes are already in the complete matrix.
  if '-W00-'not in t['id']:continue
  require(0<n<=128,'workspace calibration bound')
  for cut in range(n+1):
   c=copy.deepcopy(t);c['id']+=f'-cut{cut:03}';c['control_calls']=n;c['actions'][-1]['fail_after']=cut;yield c

def execute(cases,profile,root=ROOT):
 with tempfile.TemporaryDirectory(prefix='rml32-2bl-replay-')as tmp:
  p=Path(tmp);inp=p/'in';out=p/'out';packed=p/'observations.gz'
  with inp.open('wb')as f:
   for t in cases:f.write((json.dumps(t,sort_keys=True)+'\n').encode())
  env=os.environ.copy();env.update(RML32_REPLACEMENTS_IN=str(inp),RML32_REPLACEMENTS_OUT=str(out));argv=['cargo','test']+(['--release']if profile=='release'else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture'];r=subprocess.run(argv,cwd=root,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
  with out.open('rb')as source,packed.open('wb')as target:
   with gzip.GzipFile(fileobj=target,mode='wb',mtime=0)as z:
    while chunk:=source.read(1048576):z.write(chunk)
  raw=packed.read_bytes();summary,calibration=compare(cases,observations(packed),save_failure=True);actual=re.findall('^REPLACEMENT_REPLAY (.+)$',r.stdout,re.M);require(len(actual)==1 and json.loads(actual[0])['traces']==summary['traces']and json.loads(actual[0])['rows']==summary['rows'],'actual Rust totals')
  return summary,calibration,dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(inp.read_bytes()),observations_sha256=digest(raw)),raw

def run(quick=False):
 cases=list(traces(quick));logs=[];observed={};reports={}
 for profile in ('debug','release'):
  summary,calibration,log,raw=execute(cases,profile);logs.append(log);require('core'not in observed or observed['core']==raw,'core profile observations');observed['core']=raw;reports['core']=summary
  if quick:continue
  for name,extra in [('borrow',list(borrow_cases())),('fault',list(fault_cases(calibration)))]:
   s,_,l,o=execute(extra,profile);logs.append(l);require(name not in observed or observed[name]==o,'profile observations '+name);observed[name]=o;reports[name]=s
 raw=gzip.compress(encoded(logs),mtime=0);return dict(stage='2BL',status='pass',quick=quick,reports=reports,log_sha256=digest(raw),observation_sha256={n:digest(o)for n,o in observed.items()}),raw,observed

def verify():
 r=load(HERE/'replacement-diff.json');raw=(HERE/'replacement-log.json.gz').read_bytes();require(r['stage']=='2BL'and r['status']=='pass'and r['quick']is False and r['log_sha256']==digest(raw),'replacement identity');core_summary,calibration=compare(list(traces()),observations(HERE/'core-observations.jsonl.gz'))
 for name,cases in [('core',list(traces())),('borrow',list(borrow_cases())),('fault',list(fault_cases(calibration)))]:
  raw=(HERE/(name+'-observations.jsonl.gz')).read_bytes();require(digest(raw)==r['observation_sha256'][name],'observation hash');summary=core_summary if name=='core'else compare(cases,observations(HERE/(name+'-observations.jsonl.gz')))[0];require(summary==r['reports'][name],'replacement summary '+name)
 logs=load(HERE/'replacement-log.json.gz');require(len(logs)==6 and [c['profile']for c in logs]==['debug']*3+['release']*3 and all(c['returncode']==0 for c in logs),'replay commands');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');p.add_argument('--quick',action='store_true');a=p.parse_args()
 if a.write or a.quick:
  r,l,obs=run(a.quick)
  if a.write:
   (HERE/'replacement-diff.json').write_bytes(encoded(r));(HERE/'replacement-log.json.gz').write_bytes(l)
   for n,raw in obs.items():(HERE/(n+'-observations.jsonl.gz')).write_bytes(raw)
  print(json.dumps(r,sort_keys=True))
 else:print(json.dumps(verify(),sort_keys=True))
