"""Declared minimal managed bytecode checkpoints, with limited frozen-model crosschecks."""
import argparse,copy,gzip,hashlib,importlib.util,json,os,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
COMMIT='891f55b5dae527dd8da425dcab0a6d903b7e4702'
def digest(raw):return hashlib.sha256(raw).hexdigest()
def require(v,m):
    if not v:raise AssertionError(m)
def load(p):return json.loads(p.read_bytes())
spec=importlib.util.spec_from_file_location('_2bf_model',ROOT/'docs/stage2ba-design/model.py');BA=importlib.util.module_from_spec(spec);sys.modules[spec.name]=BA;spec.loader.exec_module(BA)
def checkpoint(n=3,construct=True,init=None,phase=None,pos=0,const=None,frame=None,frame_id=0,primary=None,failure=None,errors=(),state='alive',cleaned=False,resident=True,closed=False):
    order=[0]if n==1 else[1,2,0]
    init_meta=None if init is None else dict(object=1,position=init,constitution=order[init])
    owners=1+(phase is not None)+(phase=='Create')
    task=None if phase is None else dict(object=1,phase=phase,position=pos,constitution=order[0]if const is None else const,primary=primary,parameter_owners=owners)
    frames=[]if frame is None else[dict(id=frame_id,object=1,body=frame)]
    return dict(life=dict(objects={'0':dict(state='alive',refs=1,execs=0,cleaned=False),'1':dict(state=state,refs=int(state=='queued'),execs=len(frames),cleaned=cleaned)},queue=[1]if state=='queued'else[],resident=[0,1]if resident else[0]),managed=dict(init=init_meta,construct=task,frames=frames,failure=failure,rollback_errors=list(errors)),parameter_owners=owners,closed=closed,host_roots=1)
def row(command,expected,events=()):return dict(command=command,expected=copy.deepcopy(expected),model_events=list(events))
def cases():
    output=[]
    def add(name,steps,**kw):output.append(dict(name=name,layers=3,construct=True,shared_model=False,steps=steps,**kw))
    def init(i,n=3,construct=True):return checkpoint(n=n,construct=construct,init=i,phase='Initialize'if construct else None,frame='init:'+str(([0]if n==1 else[1,2,0])[i]),frame_id=i)
    def create(i):return checkpoint(phase='Create',pos=i+1,const=[1,2,0][i],frame='fun:'+str(2*i),frame_id=3+i)
    def back(i,frame_id,errors=()):return checkpoint(phase='RollbackDestruct',pos=i,const=[1,2,0][i],frame='fun:'+str(2*i+1),frame_id=frame_id,primary='StackUnderflow',errors=errors)
    queued=lambda error='StackUnderflow',errors=():checkpoint(state='queued',failure=error,errors=errors)
    begin_events=[['object','1'],['enter','f0','1']]
    first=row(['begin'],init(0),begin_events)
    second=row(['run',1,'budget'],init(1),[['exit','f0'],['enter','f1','1']])
    third=row(['run',1,'budget'],init(2),[['exit','f1'],['enter','f2','1']])
    creators=[first,second,third,row(['run',1,'budget'],create(0),[['exit','f2'],['enter','f3','1']])]
    creators2=creators+[row(['run',3,'budget'],create(1),[['exit','f3'],['enter','f4','1']])]
    creators3=creators2+[row(['run',3,'budget'],create(2),[['exit','f4'],['enter','f5','1']])]
    output.append(dict(name='initialization-root-handoffs',layers=3,construct=False,shared_model=True,steps=[row(['begin'],init(0,construct=False),begin_events),row(['run',0,'budget'],init(0,construct=False)),row(['run',1,'budget'],init(1,construct=False),[['exit','f0'],['enter','f1','1']]),row(['run',1,'budget'],init(2,construct=False),[['exit','f1'],['enter','f2','1']]),row(['run',1,'initialized'],checkpoint(),[['exit','f2']])]))
    add('construct-all-phases',creators3+[row(['run',3,'constructed'],checkpoint(),[['exit','f5']])]);output[-1]['shared_model']=True
    add('initialization-failure-queues',[first,row(['run',1,'error'],queued()),row(['run',0,'error'],queued()),row(['drain'],checkpoint(state='freed',cleaned=True,resident=False,failure='StackUnderflow'))],init_fail=True)
    add('create-first-layer-failure',creators+[row(['run',3,'budget'],back(0,4)),row(['run',3,'error'],queued()),row(['drain'],checkpoint(state='freed',cleaned=True,resident=False,failure='StackUnderflow'))],create_fail=0)
    add('create-last-layer-reverse-rollback',creators3+[row(['run',3,'budget'],back(2,6)),row(['run',3,'budget'],back(1,7)),row(['run',3,'budget'],back(0,8)),row(['run',3,'error'],queued()),row(['run',100,'error'],queued())],create_fail=2)
    add('rollback-errors-preserve-primary',creators3+[row(['run',3,'budget'],back(2,6)),row(['run',3,'budget'],back(1,7,['StackUnderflow'])),row(['run',3,'budget'],back(0,8,['StackUnderflow']*2)),row(['run',3,'error'],queued(errors=['StackUnderflow']*3))],create_fail=2,destruct_fail=True)
    pending=init(0);pending['life']['objects']['1']['state']='pending'
    add('delete-initialize-next-step',[first,row(['delete'],pending,[['delete','1']]),row(['run',0,'budget'],pending),row(['run',1,'error'],queued('InvalidObject'),[['exit','f0']]),row(['drain'],checkpoint(state='freed',cleaned=True,resident=False,failure='InvalidObject'),[['drain']])]);output[-1]['shared_model']=True
    pending_create=create(0);pending_create['life']['objects']['1']['state']='pending'
    add('delete-create-next-step',creators+[row(['delete'],pending_create),row(['run',0,'budget'],pending_create),row(['run',1,'error'],queued('InvalidObject'))])
    pending_back=back(2,6);pending_back['life']['objects']['1']['state']='pending'
    add('delete-rollback-preserves-primary',creators3+[row(['run',3,'budget'],back(2,6)),row(['delete'],pending_back),row(['run',0,'budget'],pending_back),row(['run',1,'error'],queued(errors=['InvalidObject']))],create_fail=2)
    cancelled=checkpoint(state='queued')
    add('cancel-is-idempotent',creators+[row(['cancel'],cancelled),row(['cancel'],cancelled),row(['drain'],checkpoint(state='freed',cleaned=True,resident=False))])
    add('extract-rebuild-retains-cleanup',[first,row(['extract'],cancelled),row(['rebuild'],cancelled),row(['drain'],checkpoint(state='freed',cleaned=True,resident=False))])
    add('close-retains-records-for-host-root',creators+[row(['close'],checkpoint(state='queued',closed=True))])
    output.append(dict(name='zero-exec-entry-failure-queues',layers=1,construct=True,no_destruct=True,stack_limit=0,shared_model=False,steps=[row(['begin'],init(0,n=1)),row(['run',1,'error'],queued('Limit')),row(['drain'],checkpoint(state='freed',cleaned=True,resident=False,failure='Limit'))]))
    return output

def check_shared_model(case):
    w=BA.World();w.apply('object','0');w.apply('put','graph-member',['object','0'])
    for item in case['steps']:
        for event in item['model_events']:w.apply(*event)
        BA.audit(w);actual=dict(objects={k:dict(state=o.state,refs=o.refs,execs=o.execs,cleaned=bool(o.cleaned))for k,o in w.objects.items()},queue=[int(n)for n in w.queue]);gold=item['expected']['life'];require(actual=={k:gold[k]for k in ('objects','queue')},'frozen model projection '+case['name'])
def generate():
    rows=cases();steps=0;shared=0
    for case in rows:
        require(case['layers']in(1,3) and sum(item['command'][1]for item in case['steps']if item['command'][0]=='run')<=1000,'trace bounds')
        if case['shared_model']:check_shared_model(case);shared+=1
        for item in case['steps']:
            s=item['expected'];require(len(s['managed']['frames'])<=3 and s['host_roots']<=8 and s['parameter_owners']<=16,'ownership bounds');steps+=1
    raw=b''.join((json.dumps(row,sort_keys=True,separators=(',',':'))+'\n').encode()for row in rows)
    return raw,dict(traces=len(rows),steps=steps,physical_teardowns=len(rows),frozen_model_traces=shared)
def sources():
    paths=['src/boundary.rs','src/object.rs','src/vm.rs','src/lifecycle_runtime.rs','src/lifecycle_managed_tests.rs','docs/stage2ba-design/model.py','docs/stage2bf-managed-test/managed_diff.py','tests/boundary_support.rs.inc']
    return {p:digest((ROOT/p).read_bytes())for p in paths}
ARGV=['cargo','test','--lib','--locked','--offline','boundary::lifecycle_managed_tests::replay_managed_lifecycle_traces','--','--ignored','--exact','--nocapture']
def run():
    raw,expected=generate()
    with tempfile.TemporaryDirectory(prefix='rml32-2bf-managed-')as tmp:
        path=Path(tmp)/'traces.jsonl';path.write_bytes(raw);r=subprocess.run(ARGV,cwd=ROOT,env=dict(os.environ,RML32_MANAGED_TRACES=str(path)),text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
        matches=re.findall(r'^MANAGED_REPLAY (.+)$',r.stdout,re.M);projection={k:v for k,v in expected.items()if k!='frozen_model_traces'};require(len(matches)==1 and json.loads(matches[0])==projection,'managed worker result')
    packed=gzip.compress(json.dumps(dict(argv=ARGV,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(raw),environment=dict(RML32_MANAGED_TRACES=str(path))),sort_keys=True).encode(),mtime=0)
    return dict(stage='2BF',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed)),packed

def verify():
    r=load(HERE/'managed-diff.json');raw,expected=generate();packed=(HERE/'managed-diff-log.json.gz').read_bytes();log=json.loads(gzip.decompress(packed));require(r==dict(stage='2BF',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed)),'managed replay identity')
    require(log['argv']==ARGV and log['returncode']==0 and log['input_sha256']==digest(raw) and set(log['environment'])=={'RML32_MANAGED_TRACES'},'managed command');matches=re.findall(r'^MANAGED_REPLAY (.+)$',log['stdout'],re.M);require(len(matches)==1 and json.loads(matches[0])=={k:v for k,v in expected.items()if k!='frozen_model_traces'},'managed actual result');return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'managed-diff.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'managed-diff-log.json.gz').write_bytes(raw);(HERE/'trace-manifest.json').write_text(json.dumps(dict(stage='2BF',cases=cases(),bounds=dict(objects=2,constitutions=3,frames=3,owning_slots=16,host_roots=8,instructions=1000)),indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
