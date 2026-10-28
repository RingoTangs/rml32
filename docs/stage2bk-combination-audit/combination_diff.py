"""Production-facing projections, independent close laws and exhaustive reserve cuts."""
import argparse,gzip,json,os,re,subprocess,tempfile,itertools
from pathlib import Path
from common import ROOT,HERE,require,digest,encoded,load
from matrix import mappings,closes
PRIVATE='boundary::lifecycle_combination_tests::production::replay_instrumented_combinations'
def cases():return list(itertools.chain(mappings(),closes()))
def project(row):
 row=json.loads(json.dumps(row))
 if row['group']=='mapping':
  for p in row.get('result',{}).get('checkpoints',[]):p.pop('calls',None)
 else:
  row.pop('final',None)
  for p in row.get('checkpoints',[]):p.pop('observation',None)
 return row
def check(t,r,internal=False):
 require(r['id']==t['id']and r['group']==t['group'],'combination inventory')
 if 'error'in r:
  require(r['error']=='Limit','unclassified entry failure '+str(r))
  if internal and r['group']=='close':require(r['final']==dict(registry_alive=False,context_alive=False),'failed setup retained control')
  return 'entry_'+r['phase']
 if r['group']=='mapping':
  result=r['result'];require(result['target']in ('accepted','rejected'),'target result')
  target=result['checkpoints'][-1]
  if result['target']=='rejected':require(target['target_error']=='Limit'and target['atomic']is True,'missing rejection atomicity')
  else:
   require(target['target_error']is None and result['frames']==0,'successful target/frame result')
   returned=result['returned']
   if t['caught']:require(returned==['int',1],'catch result')
   else:
    require(returned[0]=='array'and len(returned[1])==3,'output layout');key,value,ordinal=returned[1];require(key in (['int',0],['int',1])and ordinal==['int',1],'mapping key/ordinal')
    expected=['undefined']if t['payload']=='shell'else ['array',[['object']]]if t['payload']=='nested'else ['object'];require(value==expected,'mapping payload refinement')
  return result['target']
 closed=False;roots=2
 for p in r['checkpoints']:
  if p['op']=='maintain'and 'retired'in p:
   budget=t['budget'];drained=int(t['state']=='queued'and budget==1)
   require({k:p[k]for k in ('retired','drained','examined','queued','stop')}==dict(retired=0,drained=drained,examined=budget,queued=(2-drained if t['state']=='queued'else 0),stop='BudgetExhausted'),'maintenance phase progression')
   if not t['budget']:require(p['retired']==p['drained']==p['examined']==0,'zero maintenance')
   continue
  if p['op'].startswith('drop'):roots-=1
  if p['op']=='close':closed=True
  require(p['closed']==closed and p['roots']==roots,'close sequence projection')
  if internal:require(p['observation']==dict(registry_alive=not closed or roots>0,context_alive=not closed or roots>0),'parked/final lifetime')
 if internal:require(r['final']==dict(registry_alive=False,context_alive=False),'final Weak survived')
 return 'closed'
def compare(cases,public,private):
 counts={};eligible=[];seen=0
 for t,p,r in zip(cases,public,private,strict=True):
  require(project(p)==project(r),'normal/instrumented projection differs '+t['id']);status=check(t,r,True);check(t,p);key=t['group']+':'+status;counts[key]=counts.get(key,0)+1;seen+=1
  if t['id'].startswith(('M162','C162')):require(status in ('accepted','closed'),'success control missed target '+str(r))
  if t['group']=='mapping'and status=='accepted':
   n=r['result']['checkpoints'][-1]['calls'];require(isinstance(n,int)and 0<n<=64,'unbounded workspace count');eligible.append((t,n))
 return dict(traces=seen,classifications=counts,unclassified_differences=0),eligible
def faults(eligible):
 for t,n in eligible:
  for cut in range(n+1):yield dict(t,id=t['id']+f'-cut{cut:02}',fail_after=cut),n

def parse(raw):return [json.loads(x)for x in raw.splitlines()]
def run():
 commands=[];records={};base=cases()
 with tempfile.TemporaryDirectory(prefix='rml32-2bk-combinations-')as tmp:
  p=Path(tmp);env=os.environ.copy();env.update(RML32_COMBINATIONS_IN=str(p/'in'),RML32_COMBINATIONS_OUT=str(p/'out'))
  def execute(cases,profile,internal):
   inp=b''.join((json.dumps(t,sort_keys=True)+'\n').encode()for t in cases);(p/'in').write_bytes(inp)
   argv=['cargo','test']+(['--release']if profile=='release'else [])+['--locked','--offline']+(['--lib',PRIVATE]if internal else ['--test','lifecycle_combinations','replay_public_combinations'])+['--','--ignored','--exact','--nocapture']
   r=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);raw=(p/'out').read_bytes();require(re.findall(r'^COMBINATION_PUBLIC_REPLAY (\d+)$',r.stdout,re.M)==[str(len(cases))],'actual combination total');commands.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(inp),observations_sha256=digest(raw)));return raw
  for profile in ('debug','release'):
   pub=execute(base,profile,False);priv=execute(base,profile,True);summary,eligible=compare(base,parse(pub),parse(priv));fs=list(faults(eligible));fault=execute([t for t,_ in fs],profile,True);fault_counts=check_faults(fs,parse(fault))
   for name,raw in [('public',pub),('private',priv),('faults',fault)]:require(name not in records or records[name]==raw,'profile result differs '+name);records[name]=raw
 log=gzip.compress(encoded(commands),mtime=0);packed={n:gzip.compress(raw,mtime=0)for n,raw in records.items()}
 return dict(stage='2BK',status='pass',summary=summary,faults=fault_counts,eligible_mapping_controls=len(eligible),log_sha256=digest(log),observation_sha256={n:digest(raw)for n,raw in packed.items()}),log,packed
def check_faults(fs,rows):
 rejected=controls=0
 for (t,n),row in zip(fs,rows,strict=True):
  status=check(t,row,True);require(status==('rejected'if t['fail_after']<n else 'accepted'),'injected cut did not reach expected failure '+str(row))
  rejected+=status=='rejected';controls+=status=='accepted'
 return dict(traces=rejected+controls,injected_rejections=rejected,success_controls=controls)
def verify():
 r=load(HERE/'combination-diff.json');log=(HERE/'combination-log.json.gz').read_bytes();require(r['stage']=='2BK'and r['status']=='pass'and r['log_sha256']==digest(log),'combination identity');obs={}
 for n,h in r['observation_sha256'].items():
  raw=(HERE/(n+'-observations.jsonl.gz')).read_bytes();require(digest(raw)==h,'combination observations');obs[n]=parse(gzip.decompress(raw))
 summary,eligible=compare(cases(),obs['public'],obs['private']);require(summary==r['summary']and r['eligible_mapping_controls']==len(eligible)and check_faults(list(faults(eligible)),obs['faults'])==r['faults'],'combination reports');logs=load(HERE/'combination-log.json.gz');require(len(logs)==6 and all(c['returncode']==0 for c in logs),'combination commands');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,l,obs=run();(HERE/'combination-diff.json').write_bytes(encoded(r));(HERE/'combination-log.json.gz').write_bytes(l)
  for n,raw in obs.items():(HERE/(n+'-observations.jsonl.gz')).write_bytes(raw)
 print(json.dumps(verify(),sort_keys=True))
