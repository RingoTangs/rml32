"""Declared callable checkpoints; shared scalar ownership projects to frozen 2BA."""
import argparse,copy,gzip,hashlib,importlib.util,json,os,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
COMMIT='1927505f6ee5a2eb5c5c6db7eb8121e9e0632936'
def digest(raw):return hashlib.sha256(raw).hexdigest()
def require(v,m):
    if not v:raise AssertionError(m)
def load(p):return json.loads(p.read_bytes())
spec=importlib.util.spec_from_file_location('_2bg_model',ROOT/'docs/stage2ba-design/model.py');BA=importlib.util.module_from_spec(spec);sys.modules[spec.name]=BA;spec.loader.exec_module(BA)
def s(kind,owners=None,alive=True,creator=0,graph=None,roots=0,calls=0,frame=None,state='alive',cleaned=False,resident=True,closed=False):
    frames=[]if frame is None else[dict(id=i,object=o,body='fun:'+str(slot))for i,o,slot in frame]
    head=None if owners is None else dict(owners=owners,creator=creator,bound=int(kind!='none'),captures=[]if kind=='none'else[(('int:7'if kind=='scalar'else kind)if alive else'undefined')])
    objects={'0':dict(state=state,refs=int(owners is not None and owners>0 and creator==0)+int(state=='queued'),execs=sum(f['object']==0 for f in frames),cleaned=cleaned),'1':dict(state='alive',refs=int(graph is not None and graph>0),execs=sum(f['object']==1 for f in frames),cleaned=False)}
    return dict(life=dict(objects=objects,queue=[0]if state=='queued'else[],resident=[0,1]if resident else[1]),frames=frames,head=head,graph_owners=graph,roots=roots,calls=calls,closed=closed)
def row(cmd,expected,events=()):return dict(command=cmd,expected=copy.deepcopy(expected),model_events=list(events))
def cases():
    rows=[]
    def add(name,kind,steps,callback='borrowed',shared=False):rows.append(dict(name=name,capture=kind,callback=callback,steps=steps,shared_model=shared))
    for kind,budget,slot in [('none',3,5),('scalar',4,3)]:
        inputs=[]if kind=='none'else['input'];creation=[]if kind=='none'else[['put','input',['int',7]]]
        add('unique-'+kind,kind,[row(['start-unique'],s(kind,frame=[(0,0,slot)]),[['enter','f0','0']]),row(['run',budget,'budget'],s(kind,1,frame=[(0,0,slot)]),creation+[['function','h','0',inputs,'activation']]),row(['run',0,'budget'],s(kind,1,frame=[(0,0,slot)])),row(['run',1,'budget'],s(kind,0,False,calls=1,frame=[(0,0,slot)]),[['invoke','activation',['tmp']if kind!='none'else[]]]+([['drop','tmp']]if kind!='none'else[])),row(['run',1,'finished'],s(kind,0,False,calls=1),[['exit','f0']]),row(['take-result'],s(kind,0,False,calls=1,roots=1)),row(['drop-result'],s(kind,0,False,calls=1))],shared=True)
    kind='scalar';make=[row(['start-make'],s(kind,frame=[(0,0,1)]),[['enter','f0','0']]),row(['run',5,'finished'],s(kind,1),[['put','input',['int',7]],['function','h','0',['input'],'completion'],['exit','f0']]),row(['take-function'],s(kind,1,roots=1),[['move','completion','host:f']])]
    add('shared-scalar-retain',kind,make+[row(['retain-function'],s(kind,2,roots=2),[['copy','host:f','host:g']]),row(['start-call',1],s(kind,3,roots=2,frame=[(1,1,2)]),[['copy','host:f','arg'],['enter','f1','1']]),row(['run',3,'budget'],s(kind,4,roots=2,frame=[(1,1,2)]),[['copy','arg','activation']]),row(['run',1,'budget'],s(kind,3,roots=2,calls=1,frame=[(1,1,2)]),[['invoke','activation',['tmp']],['drop','tmp'],['drop','activation']]),row(['run',1,'finished'],s(kind,2,roots=2,calls=1),[['drop','arg'],['exit','f1']]),row(['drop-retained'],s(kind,1,roots=1,calls=1),[['retire','host:g'],['drain-host']]),row(['drop-function'],s(kind,0,False,calls=1),[['retire','host:f'],['drain-host']])],shared=True)
    for kind in['array','mapping']:
        add('unique-'+kind+'-retain',kind,[row(['start-unique'],s(kind,graph=2,roots=1,frame=[(0,0,3)])),row(['run',4,'budget'],s(kind,1,graph=3,roots=1,frame=[(0,0,3)])),row(['run',1,'budget'],s(kind,0,False,graph=4,roots=2,calls=1,frame=[(0,0,3)])),row(['run',1,'finished'],s(kind,0,False,graph=3,roots=2,calls=1)),row(['take-result'],s(kind,0,False,graph=3,roots=3,calls=1)),row(['drop-result'],s(kind,0,False,graph=2,roots=2,calls=1)),row(['drop-saved'],s(kind,0,False,graph=1,roots=1,calls=1)),row(['drop-container'],s(kind,0,False,calls=1))],callback='save')
        add('shared-'+kind,kind,[row(['start-make'],s(kind,graph=2,roots=1,frame=[(0,0,1)])),row(['run',5,'finished'],s(kind,1,graph=2,roots=1)),row(['take-function'],s(kind,1,graph=2,roots=2)),row(['start-call',1],s(kind,2,graph=2,roots=2,frame=[(1,1,2)])),row(['run',3,'budget'],s(kind,3,graph=2,roots=2,frame=[(1,1,2)])),row(['run',1,'budget'],s(kind,2,graph=3,roots=2,calls=1,frame=[(1,1,2)])),row(['run',1,'finished'],s(kind,1,graph=3,roots=2,calls=1)),row(['take-result'],s(kind,1,graph=3,roots=3,calls=1)),row(['drop-result'],s(kind,1,graph=2,roots=2,calls=1)),row(['drop-function'],s(kind,0,False,graph=1,roots=1,calls=1)),row(['drop-container'],s(kind,0,False,calls=1))])
    add('callback-error-keeps-saved-array','array',[row(['start-unique'],s('array',graph=2,roots=1,frame=[(0,0,3)])),row(['run',4,'budget'],s('array',1,graph=3,roots=1,frame=[(0,0,3)])),row(['run',1,'error'],s('array',1,graph=4,roots=2,calls=1,frame=[(0,0,3)])),row(['run',100,'error'],s('array',1,graph=4,roots=2,calls=1,frame=[(0,0,3)])),row(['cancel'],s('array',0,False,graph=2,roots=2,calls=1)),row(['drop-saved'],s('array',0,False,graph=1,roots=1,calls=1)),row(['drop-container'],s('array',0,False,calls=1))],callback='error')
    add('pending-creator-shared-call',kind:='scalar',make+[row(['start-call',0],s(kind,2,roots=1,frame=[(1,0,2)]),[['copy','host:f','arg'],['enter','f1','0']]),row(['run',3,'budget'],s(kind,3,roots=1,frame=[(1,0,2)]),[['copy','arg','activation']]),row(['delete-creator'],s(kind,3,roots=1,frame=[(1,0,2)],state='pending'),[['delete','0']]),row(['run',1,'budget'],s(kind,2,roots=1,calls=1,frame=[(1,0,2)],state='pending'),[['invoke','activation',['tmp']],['drop','tmp'],['drop','activation']]),row(['run',1,'finished'],s(kind,1,roots=1,calls=1,state='queued'),[['drop','arg'],['exit','f1']]),row(['drain'],s(kind,1,roots=1,calls=1,state='shell',cleaned=True),[['drain']]),row(['drop-function'],s(kind,0,False,calls=1,state='freed',cleaned=True),[['retire','host:f'],['drain-host']]),row(['reap'],s(kind,0,False,calls=1,state='freed',cleaned=True,resident=False))],shared=True)
    add('binding-removal-rejects-call','scalar',make+[row(['remove-binding'],s('scalar',1,roots=1)),row(['start-call',1],s('scalar',2,roots=1,frame=[(1,1,2)])),row(['run',3,'budget'],s('scalar',3,roots=1,frame=[(1,1,2)])),row(['run',1,'error'],s('scalar',3,roots=1,frame=[(1,1,2)])),row(['cancel'],s('scalar',1,roots=1)),row(['replace-binding'],s('scalar',1,roots=1))])
    add('deleted-creator-rejects-call','scalar',make+[row(['start-call',1],s('scalar',2,roots=1,frame=[(1,1,2)])),row(['run',3,'budget'],s('scalar',3,roots=1,frame=[(1,1,2)])),row(['delete-creator'],s('scalar',3,roots=1,frame=[(1,1,2)],state='shell',cleaned=True)),row(['run',1,'error'],s('scalar',3,roots=1,frame=[(1,1,2)],state='shell',cleaned=True)),row(['cancel'],s('scalar',1,roots=1,state='shell',cleaned=True)),row(['drop-function'],s('scalar',0,False,state='freed',cleaned=True)),row(['reap'],s('scalar',0,False,state='freed',cleaned=True,resident=False))])
    add('extract-rebuild','scalar',make+[row(['extract'],s('scalar',1,roots=1)),row(['rebuild'],s('scalar',1,roots=1)),row(['start-call',1],s('scalar',2,roots=1,frame=[(0,1,2)])),row(['run',4,'budget'],s('scalar',2,roots=1,calls=1,frame=[(0,1,2)])),row(['run',1,'finished'],s('scalar',1,roots=1,calls=1))])
    add('close-retains-host-head','scalar',make+[row(['close'],s('scalar',1,roots=1,closed=True))])
    add('global-creator-host-target-independent','none',[row(['start-global'],s('none',frame=[(0,0,7)])),row(['run',3,'budget'],s('none',frame=[(0,0,7),(1,None,559)])),row(['run',3,'budget'],s('none',1,creator=None,frame=[(0,0,7),(1,None,559)])),row(['run',2,'finished'],s('none',1,creator=None)),row(['take-function'],s('none',1,creator=None,roots=1)),row(['clear-global'],s('none',1,creator=None,roots=1)),row(['start-call',1],s('none',2,creator=None,roots=1,frame=[(2,1,2)])),row(['run',4,'budget'],s('none',2,creator=None,roots=1,calls=1,frame=[(2,1,2)])),row(['run',1,'finished'],s('none',1,creator=None,roots=1,calls=1))])
    return rows

def check_model(case):
    w=BA.World();w.apply('object','0');w.apply('object','1')
    for item in case['steps']:
        for event in item['model_events']:w.apply(*event)
        BA.audit(w);actual=dict(objects={k:dict(state=o.state,refs=o.refs,execs=o.execs,cleaned=bool(o.cleaned))for k,o in w.objects.items()},queue=[int(n)for n in w.queue]);gold=item['expected']['life'];require(actual=={k:gold[k]for k in ('objects','queue')},'model projection '+case['name']);h=item['expected']['head'];require((w.heads.get('h',{}).get('owners',0))==(0 if h is None else h['owners']),'model head owners')
def generate():
    rows=cases();steps=0;shared=0
    for case in rows:
        if case['shared_model']:check_model(case);shared+=1
        require(sum(x['command'][1]for x in case['steps']if x['command'][0]=='run')<=1000,'instruction bound')
        for x in case['steps']:
            require(len(x['expected']['frames'])<=3 and x['expected']['roots']<=8,'trace bounds');steps+=1
    raw=b''.join((json.dumps(r,sort_keys=True,separators=(',',':'))+'\n').encode()for r in rows);return raw,dict(traces=len(rows),steps=steps,physical_teardowns=len(rows),frozen_model_traces=shared)
def sources():
    paths=['src/boundary.rs','src/object.rs','src/vm.rs','src/lifecycle_runtime.rs','src/lifecycle_host_function_tests.rs','docs/stage2ba-design/model.py','docs/stage2bg-host-function-test/callable_diff.py','tests/boundary_support.rs.inc'];return {p:digest((ROOT/p).read_bytes())for p in paths}
ARGV=['cargo','test','--lib','--locked','--offline','boundary::lifecycle_host_function_tests::replay_host_function_traces','--','--ignored','--exact','--nocapture']
def run():
    raw,expected=generate()
    with tempfile.TemporaryDirectory(prefix='rml32-2bg-callable-')as tmp:
        path=Path(tmp)/'traces.jsonl';path.write_bytes(raw);r=subprocess.run(ARGV,cwd=ROOT,env=dict(os.environ,RML32_CALLABLE_TRACES=str(path)),text=True,capture_output=True);require(r.returncode==0,r.stdout+r.stderr);matches=re.findall(r'^CALLABLE_REPLAY (.+)$',r.stdout,re.M);require(len(matches)==1 and json.loads(matches[0])=={k:v for k,v in expected.items()if k!='frozen_model_traces'},'worker result')
    packed=gzip.compress(json.dumps(dict(argv=ARGV,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=digest(raw),environment=dict(RML32_CALLABLE_TRACES=str(path))),sort_keys=True).encode(),mtime=0);return dict(stage='2BG',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed)),packed

def verify():
    r=load(HERE/'callable-diff.json');raw,expected=generate();packed=(HERE/'callable-diff-log.json.gz').read_bytes();log=json.loads(gzip.decompress(packed));require(r==dict(stage='2BG',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed)),'callable identity');require(log['argv']==ARGV and log['returncode']==0 and log['input_sha256']==digest(raw) and set(log['environment'])=={'RML32_CALLABLE_TRACES'},'callable command');matches=re.findall(r'^CALLABLE_REPLAY (.+)$',log['stdout'],re.M);require(len(matches)==1 and json.loads(matches[0])=={k:v for k,v in expected.items()if k!='frozen_model_traces'},'callable actual result');return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'callable-diff.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'callable-diff-log.json.gz').write_bytes(raw);(HERE/'trace-manifest.json').write_text(json.dumps(dict(stage='2BG',cases=cases(),bounds=dict(objects=2,heads=2,containers=2,frames=3,owning_slots=16,host_roots=8,instructions=1000)),indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
