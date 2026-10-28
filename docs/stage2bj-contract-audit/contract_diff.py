"""Actions-only Rust replay versus the frozen semantic model and metadata oracle."""
import argparse,gzip,json,os,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,encoded,digest,load,require
from matrix import traces
from oracle import Oracle,m
from maintenance_oracle import MaintenanceOracle
DRIVER='boundary::lifecycle_contract_tests::replay_production_contract_traces'
SEMANTIC_PROGRESS=['retired_roots','drained_objects','reaped_objects','pending_roots','queued_objects']
def inputs():return b''.join((json.dumps(t,sort_keys=True,separators=(',',':'))+'\n').encode()for t in traces())
def compare(cases,observations):
 rows=iter(observations);counts=dict(traces=0,steps=0,accepted=0,rejected=0,semantic_checkpoints=0,metadata_checkpoints=0,observer_checks=0)
 family={};unclassified=[]
 for trace in cases:
  oracle=Oracle(trace['limits']);metadata=MaintenanceOracle();semantic=True;previous=None;counts['traces']+=1;stats=family.setdefault(trace['family'],dict(traces=0,accepted=0,rejected=0));stats['traces']+=1
  for step,a in enumerate(trace['actions']):
   row=next(rows,None);require(row is not None and row['id']==trace['id']and row['step']==step,'missing/reordered checkpoint')
   require(previous is None or row['before']==previous,'discontinuous actual observations')
   if semantic:require(row['before']['semantic']==oracle.projection(),'pre-action semantic mismatch')
   op=a['op'];expected=None;error=None
   if op=='epoch_wrap':semantic=False
   if op=='maintain'and(a['budget']['metadata_entries']or trace['family']in ('F13','F14')):
    semantic=False;require(row['accepted']and row['error_kind']is None,'metadata entry error');metadata.check(a,row['before'],row['after'],row['progress']);counts['metadata_checkpoints']+=1
   elif semantic:
    try:expected=oracle.event(a)
    except m.Rejected as e:error=e.kind
    require(row['accepted']==(error is None)and row['error_kind']==error,f'{trace["id"]} step {step} result expected={error} actual={row["error_kind"]}')
    require(row['after']['semantic']==oracle.projection(),f'{trace["id"]} step {step} semantic mismatch\nexpected={oracle.projection()}\nactual={row["after"]["semantic"]}')
    if op=='maintain':metadata.check_semantic_phase(a,row['before'],row['after'],row['progress'],expected);counts['metadata_checkpoints']+=1
    elif op=='vm_pending'and error is None:require(row['progress']==expected,'actual VM result')
    counts['semantic_checkpoints']+=1
   elif op not in ('object','drop_view','foreign','epoch_wrap'):
    unclassified.append((trace['id'],step,op))
   elif op=='object':
    full=len(row['before']['semantic']['objects'])>=trace['limits']['max_objects'];require(row['accepted']==(not full)and row['error_kind']==('Limit'if full else None),'metadata-domain object capacity')
    if not full:require(len(row['after']['semantic']['objects'])==len(row['before']['semantic']['objects'])+1,'object registration lost')
   elif op in ('drop_view','epoch_wrap'):
    require(row['accepted']and row['error_kind']is None and row['before']['semantic']==row['after']['semantic'],'physical-view/epoch event changed semantic owners')
    if op=='epoch_wrap':require(row['after']['raw']['force']and row['after']['raw']['epoch']==1,'epoch wrap notice missing')
   # All rejection paths preserve semantic state except explicit abort of a
   # staged commit: abort is a separate model transition in that macro.
   if not row['accepted']and op not in ('commit_copy','commit_head'):require(row['before']['semantic']==row['after']['semantic'],'rejection changed semantic state')
   if op=='foreign':require(not row['accepted']and row['error_kind']=='InvalidState','foreign origin accepted')
   if op=='close'and row['accepted']:require(row['before']['semantic']==row['after']['semantic'],'parked close changed ownership')
   counts['steps']+=1;counts['accepted'if row['accepted']else 'rejected']+=1;stats['accepted'if row['accepted']else 'rejected']+=1;counts['observer_checks']+=1
   previous=row['after']
   if not row['accepted']and not a.get('continue_on_error',False):break
 require(next(rows,None)is None,'extra checkpoint');require(not unclassified,'unclassified action '+str(unclassified));require(counts['traces']==2592 and all(v['traces']==162 for v in family.values()),'matrix coverage')
 return dict(**counts,families=family,unclassified_differences=0)
def run():
 raw=inputs();commands=[];observed=None
 with tempfile.TemporaryDirectory(prefix='rml32-2bj-diff-')as temp:
  path=Path(temp);(path/'actions.jsonl').write_bytes(raw);env=os.environ.copy();env.update(RML32_D08_TRACES=str(path/'actions.jsonl'),RML32_D08_OBSERVATIONS=str(path/'observations.jsonl'))
  for profile in ('debug','release'):
   argv=['cargo','test']+(['--release']if profile=='release'else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture'];r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True)
   require(r.returncode==0,r.stdout+r.stderr);current=(path/'observations.jsonl').read_bytes();summary=compare(traces(),[json.loads(line)for line in current.splitlines()])
   actual=re.findall(r'^PRODUCTION_CONTRACT_REPLAY (.+)$',r.stdout,re.M);require(len(actual)==1 and json.loads(actual[0])==dict(traces=summary['traces'],steps=summary['steps'],teardowns=summary['traces']),'driver actual totals')
   require(observed is None or observed==current,'profile observations differ');observed=current
   commands.append(dict(profile=profile,argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,observations_sha256=digest(current)))
  log=gzip.compress(encoded(dict(profiles=commands)),mtime=0);packed=gzip.compress(observed,mtime=0)
  return dict(stage='2BJ',status='pass',profiles_checked=['debug','release'],input_sha256=digest(raw),observations_sha256=digest(packed),log_sha256=digest(log),summary=summary,full_state_space_proof=False,native_equivalence_proven=False),log,packed
def verify():
 r=load(HERE/'contract-diff.json');raw=(HERE/'contract-observations.jsonl.gz').read_bytes();log=(HERE/'contract-log.json.gz').read_bytes();require(r['stage']=='2BJ'and r['status']=='pass'and r['input_sha256']==digest(inputs())and r['observations_sha256']==digest(raw)and r['log_sha256']==digest(log),'diff artifact identity')
 commands=load(HERE/'contract-log.json.gz')['profiles'];require([c['profile']for c in commands]==['debug','release']and all(c['returncode']==0 and c['observations_sha256']==digest(gzip.decompress(raw))for c in commands),'driver profile logs');rows=[json.loads(line)for line in gzip.decompress(raw).splitlines()];require(r['summary']==compare(traces(),rows),'diff summary');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,log,observed=run();(HERE/'contract-diff.json').write_bytes(encoded(r));(HERE/'contract-log.json.gz').write_bytes(log);(HERE/'contract-observations.jsonl.gz').write_bytes(observed)
 print(json.dumps(verify(),sort_keys=True))
