"""Declared actual Host timelines; frozen model only for its supported projection."""
import argparse,gzip,hashlib,importlib.util,json,os,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
COMMIT='d237bf3a55e0d37084184cc3b3efbbb6e0c1c8e3'
def digest(raw):return hashlib.sha256(raw).hexdigest()
def require(v,m):
    if not v:raise AssertionError(m)
def load(p):return json.loads(p.read_bytes())
spec=importlib.util.spec_from_file_location('_2be_model',ROOT/'docs/stage2ba-design/model.py');BA=importlib.util.module_from_spec(spec);sys.modules[spec.name]=BA;spec.loader.exec_module(BA)
def cases():
    rows=[]
    def add(name,steps,callback='borrowed'):rows.append(dict(name=name,callback=callback,steps=steps))
    def step(cmd,*events):return (cmd,list(events))
    root=step(['root','r',1],['put','host:r',['object','1']])
    start=lambda f:step(['start',f],['enter','f','0'],['put','arg',['object','1']])
    copied=step(['run',3,'budget'],['copy','arg','stack'])
    finish=step(['run',100,'finished'],['move','stack','completion'],['drop','arg'],['exit','f'])
    take=step(['take','out'],['move','completion','host:out'])
    drop=lambda n:step(['drop',n],['retire','host:'+n],['drain-host'])
    cancel=step(['transfer'],['exit','f'],['drop','arg'],['drop','stack'],['transfer'])
    add('root-retain-release',[root,step(['retain','s','r'],['copy','host:r','host:s']),drop('r'),drop('s')])
    add('borrowed-retirement',[root,step(['borrow']),step(['drop','r'],['retire','host:r']),step(['check-rejected']),step(['unborrow']),step(['host-drain'],['drain-host'])])
    add('shell-and-explicit-reap',[root,step(['delete',1],['delete','1']),drop('r'),step(['reap'])])
    add('transfer-cancels-without-object-drain',[root,start(0),step(['delete',0],['delete','0']),step(['transfer'],['exit','f'],['drop','arg'],['transfer']),step(['rebuild']),step(['drain'],['drain']),drop('r')])
    add('pending-return-root-and-drain',[step(['root','r',0],['put','host:r',['object','0']]),start(0),step(['delete',0],['delete','0']),copied,finish,take,step(['drain'],['drain']),drop('r'),drop('out'),step(['reap'])])
    add('close-retains-until-root-release',[root,step(['close'],['close']),drop('r')])
    add('close-with-borrowed-retirement',[root,step(['borrow']),step(['close'],['close']),step(['drop','r'],['retire','host:r']),step(['unborrow']),step(['host-drain'],['drain-host'])])
    add('callback-borrowed-return',[root,start(1),copied,step(['run',1,'budget']),finish,take,drop('r'),drop('out')])
    add('callback-retains-root',[root,start(1),copied,step(['run',1,'budget'],['copy','stack','host:saved']),finish,take,step(['drop-saved'],['retire','host:saved'],['drain-host']),drop('r'),drop('out')],'save')
    add('callback-error-sticky-transfer-close',[start(1),copied,step(['run',1,'error'],['copy','stack','host:saved']),step(['run',100,'error']),cancel,step(['close'],['close']),step(['drop-saved'],['retire','host:saved'],['drain-host'])],'error')
    add('callback-invalid-result-sticky',[start(1),copied,step(['run',1,'error']),step(['run',100,'error']),cancel],'invalid')
    add('callback-void-return',[start(1),copied,step(['run',1,'budget'],['drop','stack'],['put','stack',['undefined',None]]),finish,take,drop('out')],'void')
    return rows

def generate():
    output=[];total=0
    for case in cases():
        require(sum(c[1] for c,_ in case['steps'] if c[0]=='run')<=1000,'instruction request bound')
        w=BA.World();w.apply('object','0');w.apply('object','1');resident=[0,1];calls=0;commands=[];expected=[];closed=False
        for command,events in case['steps']:
            for e in events:w.apply(*e)
            BA.audit(w)
            if command[0]=='run' and command[1]==1 and case['name'].startswith('callback'):calls+=1
            # Repeated sticky runs never invoke host code.
            if command[0]in('reap','drain'):resident=[i for i in resident if w.objects[str(i)].state!='freed']
            if command[0]=='close':closed=True
            hostroots=sum(n.startswith('host:')for n in w.slots)
            if closed and hostroots==0:resident=[]
            require(len(w.slots)<=16 and len(w.frames)<=3 and hostroots<=8,'trace bounds')
            snapshot=dict(objects={k:dict(state=o.state,refs=o.refs,execs=o.execs,cleaned=bool(o.cleaned))for k,o in w.objects.items()},queue=[int(i)for i in w.queue],resident=resident[:],host_roots=hostroots,retired=len(w.retired),closed=closed,frames=len(w.frames),calls=calls)
            commands.append(command);expected.append(snapshot);total+=1
        output.append(dict(name=case['name'],callback=case['callback'],commands=commands,expected=expected,model_events=[events for _,events in case['steps']]))
    raw=b''.join((json.dumps(row,sort_keys=True,separators=(',',':'))+'\n').encode()for row in output)
    return raw,dict(traces=len(output),steps=total,physical_teardowns=len(output))
def sources():
    paths=['src/boundary.rs','src/object.rs','src/vm.rs','src/lifecycle_runtime.rs','src/lifecycle_adapter.rs','src/lifecycle_boundary_tests.rs','docs/stage2ba-design/model.py','docs/stage2be-host-test/host_diff.py','tests/boundary_support.rs.inc']
    return {p:digest((ROOT/p).read_bytes())for p in paths}
ARGV=['cargo','test','--lib','--locked','--offline','boundary::lifecycle_boundary_tests::replay_host_lifecycle_traces','--','--ignored','--exact','--nocapture']
def run():
    raw,expected=generate()
    with tempfile.TemporaryDirectory(prefix='rml32-2be-host-')as temp:
        path=Path(temp)/'traces.jsonl';path.write_bytes(raw)
        r=subprocess.run(ARGV,cwd=ROOT,env=dict(os.environ,RML32_HOST_LIFECYCLE_TRACES=str(path)),text=True,capture_output=True)
        require(r.returncode==0,r.stdout+r.stderr);actual=re.findall(r'^HOST_REPLAY (.+)$',r.stdout,re.M);require(len(actual)==1 and json.loads(actual[0])==expected,'worker result')
    packed=gzip.compress(json.dumps(dict(argv=ARGV,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(raw),environment=dict(RML32_HOST_LIFECYCLE_TRACES=str(path))),sort_keys=True).encode(),mtime=0)
    return dict(stage='2BE',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed)),packed

def verify():
    r=load(HERE/'host-diff.json');raw,expected=generate();packed=(HERE/'host-diff-log.json.gz').read_bytes();log=json.loads(gzip.decompress(packed))
    require(r==dict(stage='2BE',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed)),'Host replay identity')
    require(log['argv']==ARGV and log['returncode']==0 and log['input_sha256']==digest(raw) and set(log['environment'])=={'RML32_HOST_LIFECYCLE_TRACES'},'Host command');matches=re.findall(r'^HOST_REPLAY (.+)$',log['stdout'],re.M);require(len(matches)==1 and json.loads(matches[0])==expected,'Host actual result')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'host-diff.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'host-diff-log.json.gz').write_bytes(raw)
        (HERE/'trace-manifest.json').write_text(json.dumps(dict(stage='2BE',cases=cases(),bounds=dict(objects=2,frames=3,owning_slots=16,host_roots=8)),indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
