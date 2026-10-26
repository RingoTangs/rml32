"""Known synthetic bytecode timelines; lifecycle events use frozen 2BA model."""
import argparse,gzip,hashlib,importlib.util,json,os,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent

def digest(raw):return hashlib.sha256(raw).hexdigest()
def require(v,m):
    if not v:raise AssertionError(m)
def load(p):return json.loads(p.read_bytes())
spec=importlib.util.spec_from_file_location('_2bd_ba',ROOT/'docs/stage2ba-design/model.py');BA=importlib.util.module_from_spec(spec);sys.modules[spec.name]=BA;spec.loader.exec_module(BA)
def f(slot,nodes,args=0):return dict(slot=slot,nodes=nodes,args=args)
def start(o,slot=0,**kw):return dict(op='start',object=o,function=slot,**kw)
def run(n,**kw):return dict(op='run',budget=n,**kw)
def delete(o):return dict(op='delete',object=o)
def cmd(op):return dict(op=op)
BASIC=[f(0,[[7,0,0x3b]])]
def cases():
    rows=[]
    def add(name,functions,steps,objects=1,vars=0,**kw):rows.append(dict(id=name,functions=functions,steps=steps,objects=objects,vars=vars,**kw))
    add('pending-budget-return-drain',BASIC,[(start(0),[['enter','a','0']],[[0,0]]),(cmd('inspect'),[],[[0,0]]),(delete(0),[['delete','0']],[[0,0]]),(run(0),[],[[0,0]]),(run(4),[['exit','a']],[]),(cmd('inspect'),[],[]),(cmd('drain'),[['drain']],[])])
    add('immediate-delete-explicit-reap',BASIC,[(delete(0),[['delete','0']],[]),(cmd('reap'),[],[])],vars=1)
    add('extract-rebuild-preserves-mode',BASIC,[(start(0),[['enter','a','0']],[[0,0]]),(delete(0),[['delete','0']],[[0,0]]),(cmd('extract'),[['exit','a']],[]),(cmd('rebuild'),[],[]),(cmd('drain'),[['drain']],[])])
    recursive=[f(0,[[1,0,7]]),f(1,[[2,0,7]]),f(2,[[9,0,0x3b]])]
    add('three-recursive-real-frames',recursive,[(start(0),[['enter','a','0']],[[0,0]]),(run(3),[['enter','b','0']],[[0,0],[0,1]]),(run(3),[['enter','c','0']],[[0,0],[0,1],[0,2]]),(delete(0),[['delete','0']],[[0,0],[0,1],[0,2]]),(run(4),[['exit','c']],[[0,0],[0,1]]),(run(1),[['exit','b']],[[0,0]]),(run(1),[['exit','a']],[]),(cmd('drain'),[['drain']],[])])
    catch=[f(0,[[3,0,0x2c],[1,0,7],[0,0,0x1a],[1,0,0x1f],[0,0,0x0e]]),f(1,[[2,0,7]]),f(2,[[1,0,0x3b],[0,0,0x3b],[0,0,0x19]])]
    add('catch-exits-top-down',catch,[(start(0),[['enter','a','0']],[[0,0]]),(run(4),[['enter','b','0']],[[0,0],[0,1]]),(run(3),[['enter','c','0']],[[0,0],[0,1],[0,2]]),(delete(0),[['delete','0']],[[0,0],[0,1],[0,2]]),(run(5),[['exit','c'],['exit','b']],[[0,0]]),(run(2),[['exit','a']],[]),(cmd('drain'),[['drain']],[])])
    sticky=[f(0,[[1,0,7]]),f(1,[[1,0,0x3b],[0,0,0x3b],[0,0,0x19]]),f(2,[[7,0,0x3b]])]
    add('sticky-restart-prepares-before-retiring',sticky,[(start(0),[['enter','a','0']],[[0,0]]),(run(3),[['enter','b','0']],[[0,0],[0,1]]),(run(5,error=True),[],[[0,0],[0,1]]),(delete(0),[['delete','0']],[[0,0],[0,1]]),(run(0,error=True),[],[[0,0],[0,1]]),(start(0,2),[['enter','c','0'],['exit','b'],['exit','a']],[[0,2]]),(cmd('cancel'),[['exit','c']],[]),(cmd('drain'),[['drain']],[])])
    add('variable-function-creator-cycle',[f(0,[[5,0,0x3b],[1,1,0x17],[0,0,0x4e],[0,0,0x33],[7,0,0x3b]]),f(1,[[8,0,0x3b]],args=1)],[(start(0),[['enter','a','0']],[[0,0]]),(run(6),[['put','tmp',['int',5]],['function','h','0',['tmp'],'var:0:f'],['drop','tmp']],[[0,0]]),(delete(0),[['delete','0']],[[0,0]]),(delete(0),[['delete','0']],[[0,0]]),(run(2),[['exit','a']],[]),(cmd('drain'),[['drain']],[])],vars=1,function_values=True)
    add('global-frame-has-no-exec',[f(0,[[559,-128,7]])],[(start(0),[['enter','a','0']],[[0,0]]),(run(3),[],[[0,0],[None,559]]),(delete(0),[['delete','0']],[[0,0],[None,559]]),(run(4),[],[[0,0]]),(run(1),[['exit','a']],[]),(cmd('drain'),[['drain']],[])],global_=True)
    dual=[f(0,[[1,0,0x17]]),f(1,[[8,0,0x3b]]),f(2,[[0,0,0x37],[0,1,5]],args=1)]
    prefix=[(start(1),[['enter','ctor','1']],[[1,0]]),(run(4),[['function','h','1',[],'f'],['exit','ctor']],[]),(start(0,2,result_argument=True),[['enter','a','0']],[[0,2]]),(run(3),[['copy','f','t']],[[0,2]]),(run(1),[['enter','b','1'],['drop','t']],[[0,2],[1,1]]),(delete(0),[['delete','0']],[[0,2],[1,1]]),(delete(1),[['delete','1']],[[0,2],[1,1]])]
    add('dual-return-lifo-separate-creator',dual,prefix+[(run(4),[['exit','b']],[[0,2]]),(run(1),[['drop','f'],['exit','a']],[]),(cmd('drain'),[['drain']],[]),(cmd('drain'),[['drain']],[])],objects=2,function_values=True)
    add('dual-cancel-lifo-top-down',dual,prefix+[(cmd('cancel'),[['exit','b'],['exit','a'],['drop','f']],[]),(cmd('drain'),[['drain']],[]),(cmd('drain'),[['drain']],[])],objects=2,function_values=True)
    return rows

def generate():
    output=[];steps=0
    for case in cases():
        require(case['objects']<=2 and sum(c.get('budget',0) for c,_,_ in case['steps'])<=1000,'execution bounds')
        w=BA.World();resident=list(range(case['objects']))
        for i in resident:w.apply('object',str(i))
        rows=[]
        for command,events,frames in case['steps']:
            for e in events:w.apply(*e)
            BA.audit(w);require(len(frames)<=3 and len(w.heads)<=2 and len(w.slots)<=16,'trace bounds')
            if command['op']in('drain','reap'):resident=[i for i in resident if w.objects[str(i)].state!='freed']
            objects={k:dict(state=v.state,refs=v.refs,execs=v.execs,cleaned=bool(v.cleaned)) for k,v in w.objects.items()}
            rows.append(dict(command=command,expected=dict(objects=objects,queue=[int(i) for i in w.queue],resident=resident[:]),frames=frames,model_events=events));steps+=1
        row={k:v for k,v in case.items() if k!='steps'};row['global']=row.pop('global_',False);row['steps']=rows;output.append(row)
    raw=b''.join((json.dumps(row,sort_keys=True,separators=(',',':'))+'\n').encode() for row in output)
    return raw,dict(traces=len(output),steps=steps,physical_teardowns=len(output))
def sources():
    paths=['src/lib.rs','src/cell.rs','src/container.rs','src/object.rs','src/vm.rs','src/lifecycle_adapter.rs','src/lifecycle_runtime.rs','src/lifecycle_vm_tests.rs','docs/stage2ba-design/model.py','docs/stage2bd-runtime-test/vm_diff.py']
    return {p:digest((ROOT/p).read_bytes()) for p in paths}
def run_all():
    raw,expected=generate()
    with tempfile.TemporaryDirectory(prefix='rml32-2bd-vm-') as tmp:
        path=Path(tmp)/'traces.jsonl';path.write_bytes(raw);argv=['cargo','test','--lib','--locked','--offline','vm::lifecycle_vm_tests::replay_lifecycle_vm_traces','--','--ignored','--exact','--nocapture']
        r=subprocess.run(argv,cwd=ROOT,env=dict(os.environ,RML32_VM_LIFECYCLE_TRACES=str(path)),text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr)
        matches=re.findall(r'^VM_LIFECYCLE_REPLAY (.+)$',r.stdout,re.M);require(len(matches)==1 and json.loads(matches[0])==expected,'VM worker inventory')
        log=dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(raw),environment=dict(RML32_VM_LIFECYCLE_TRACES=str(path)))
        packed=gzip.compress(json.dumps(log,sort_keys=True).encode(),mtime=0)
        return dict(stage='2BD',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed),native_execution=False,business_bytecode_execution=False,synthetic_vm_execution=True,full_native_equivalence=False),packed

def verify():
    r=load(HERE/'vm-diff.json');raw,expected=generate();packed=(HERE/'vm-diff-log.json.gz').read_bytes();log=json.loads(gzip.decompress(packed))
    require(r['stage']=='2BD' and r['status']=='pass' and r['source_sha256']==sources(),'VM sources')
    require(r['input_sha256']==digest(raw) and r['replay']==expected,'VM timeline')
    require(r['log_sha256']==digest(packed) and log['returncode']==0 and log['input_sha256']==digest(raw),'VM log')
    matches=re.findall(r'^VM_LIFECYCLE_REPLAY (.+)$',log['stdout'],re.M);require(len(matches)==1 and json.loads(matches[0])==expected,'VM actual result')
    require(not r['native_execution'] and not r['business_bytecode_execution'] and r['synthetic_vm_execution'] and not r['full_native_equivalence'],'VM claims')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,log=run_all();(HERE/'vm-diff.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'vm-diff-log.json.gz').write_bytes(log)
    print(json.dumps(verify(),sort_keys=True))
