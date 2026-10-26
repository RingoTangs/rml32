"""Fixed trace projections over frozen 2AS/2BA; executes real Rust cells."""
import argparse,gzip,hashlib,importlib.util,json,os,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent

def digest(raw):return hashlib.sha256(raw).hexdigest()
def require(v,m):
    if not v:raise AssertionError(m)
def load(p):return json.loads(p.read_bytes())
def module(name,path):
    s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m
AS=module('_2bc_as',ROOT/'docs/stage2as-design/model.py')
BA=module('_2bc_ba',ROOT/'docs/stage2ba-design/model.py')
def obj(i):return dict(op='object',id=str(i))
def put(root,i):return dict(op='put',root=root,value=['object',str(i)])
def build(root,kind,sources,creator=None):return dict(op='build',root=root,kind=kind,sources=sources,**({'creator':str(creator)} if creator is not None else {}))
def copy(source,root):return dict(op='copy',source=source,root=root)
def drop(root):return dict(op='drop',root=root)
def ref(source,root,index=0):return dict(op='member-ref',source=source,root=root,index=index)

def cases():
    seed=[obj(0),obj(1)]
    rows=[
    ('object-copy-alias-move',[put('x',0),copy('x','y'),dict(op='alias',source='x',root='r'),dict(op='move',source='y',root='z'),drop('x'),drop('r'),drop('z')]),
    ('array-shallow-copy-retained-member',[put('x',0),build('a','array',['x']),copy('a','b'),ref('a','r'),drop('a'),drop('b'),drop('r')]),
    ('mapping-retained-member-clear',[put('x',0),build('a','mapping',['x']),ref('a','r'),copy('a','b'),drop('a'),drop('b'),drop('r')]),
    ('shared-child-graph',[put('x',0),build('a','array',['x']),copy('a','b'),build('p','array',['a','b']),copy('p','q'),drop('p'),drop('q')]),
    ('same-creator-capture',[put('x',0),build('f','function',['x'],0),copy('f','g'),drop('f'),drop('g')]),
    ('distinct-creator-capture',[put('x',1),build('f','function',['x'],0),copy('f','g'),drop('g'),drop('f')]),
    ('nested-function-array',[put('x',1),build('f','function',['x'],0),build('a','array',['f']),build('g','function',['a'],1),drop('g')]),
    ('unique-invalid-capture',[put('x',1),build('f','function',['x'],0),dict(op='delete-object',id='1'),dict(op='invoke',root='f',outputs=['out']),drop('out')]),
    ('shared-invalid-capture',[put('x',1),build('f','function',['x'],0),copy('f','g'),dict(op='delete-object',id='1'),dict(op='invoke',root='f',outputs=['out']),dict(op='invoke',root='g',outputs=['out2']),drop('out'),drop('out2')]),
    ('invalid-creator-normalization',[put('x',1),build('f','function',['x'],0),dict(op='delete-object',id='0'),copy('f','out'),drop('f'),drop('out')]),
    ('actual-source-alias-normalization',[put('x',0),dict(op='alias',source='x',root='r'),dict(op='delete-object',id='0'),copy('x','out'),drop('r'),drop('x'),drop('out')]),
    ('cycle-rejected',[put('x',0),build('a','array',['x']),ref('a','r'),dict(op='write',source='a',target='r'),drop('r'),drop('a')]),
    ('conflicting-alias-batch',[put('x',0),put('y',1),dict(op='put',root='z',value=['int',7]),dict(op='alias',source='z',root='r'),dict(op='batch-copy',sources=['x','y'],targets=['z','r']),drop('x'),drop('y'),drop('z'),drop('r')]),
    ('equal-alias-batch',[put('x',0),dict(op='put',root='z',value=['int',7]),dict(op='alias',source='z',root='r'),dict(op='batch-copy',sources=['x','x'],targets=['z','r']),drop('x'),drop('z'),drop('r')]),
    ]
    result=[dict(id=name,model='2AS',actions=seed+actions) for name,actions in rows]
    for same in (False,True):
        result.append(dict(id='ba-creator-'+('same' if same else 'distinct'),model='2BA',actions=[['object','0'],['object','1'],['put','x',['object','0' if same else '1']],['function','h','0',['x'],'f'],['copy','f','g'],['drop','f'],['drop','g'],['drop','x']]))
    result.append(dict(id='ba-invalid-creator',model='2BA',actions=[['object','0'],['function','h','0',[],'f'],['delete','0'],['copy','f','out']]))
    return result

def projection_as(w):
    refs={o:0 for o in w.objects}
    for s in w.slots.values():
        if s.value[0]=='object':refs[s.value[1]]+=1
    for h in w.headers.values():
        if h.kind=='function' and h.creator is not None:refs[h.creator]+=1
    roots={n:list(w.slots[r.cell].value) for n,r in w.roots.items()}
    names=sorted(w.roots);aliases=[[a,b] for i,a in enumerate(names) for b in names[i+1:] if w.roots[a].cell==w.roots[b].cell]
    return dict(roots=roots,aliases=aliases,refs=refs,slots=len(w.slots),headers={str(i):[h.kind,h.owners] for i,h in w.headers.items()})
def projection_ba(w):
    def v(v):return [v[0],1 if v[0]=='function' else v[1]]
    return dict(roots={n:v(p) for n,p in w.slots.items() if not n.startswith('cap:')},aliases=[],refs={o:r.refs for o,r in w.objects.items()},slots=len(w.slots),headers={'1':['function',h['owners']] for h in w.heads.values()})
def ba_action(a):
    op=a[0]
    if op=='object':return obj(a[1])
    if op=='put':return dict(op='put',root=a[1],value=a[2])
    if op=='function':return dict(op='build',root=a[4],kind='function',sources=a[3],creator=a[2],keep_sources=True)
    if op=='copy':return copy(a[1],a[2])
    if op=='drop':return drop(a[1])
    if op=='delete':return dict(op='delete-object',id=a[1])
    raise AssertionError('outside frozen leaf projection')
def generate():
    output=[];rejected=steps=0
    for case in cases():
        is_as=case['model']=='2AS';m=AS if is_as else BA
        w=AS.World(max_items=4,max_depth=3,max_nodes=16) if is_as else BA.World();rows=[]
        for a in case['actions']:
            try:
                if is_as:w.apply(a)
                else:w.apply(*a)
                ok=True
            except m.Rejected:ok=False;rejected+=1
            m.audit(w)
            slots=len(w.slots);objects=len(w.objects)
            if is_as:
                heads=sum(h.kind=='function' for h in w.headers.values());containers=sum(h.kind!='function' for h in w.headers.values())
            else:heads=len(w.heads);containers=0
            require(objects<=2 and slots<=6 and heads<=2 and containers<=2,'hand trace bounds exceeded')
            rows.append(dict(action=a if is_as else ba_action(a),accepted=ok,expected=projection_as(w) if is_as else projection_ba(w)))
            steps+=1
        output.append(dict(id=case['id'],model=case['model'],steps=rows))
    raw=b''.join((json.dumps(row,sort_keys=True,separators=(',',':'))+'\n').encode() for row in output)
    return raw,dict(traces=len(output),steps=steps,rejected=rejected,physical_teardowns=len(output))
def sources():
    names=['docs/stage2as-design/model.py','docs/stage2ba-design/model.py','src/lifecycle_adapter.rs','src/cell.rs','src/copy_plan.rs','src/ownership.rs','src/lib.rs','docs/stage2bc-adapter/adapter_diff.py']
    return {p:digest((ROOT/p).read_bytes()) for p in names}
def run():
    raw,expected=generate()
    with tempfile.TemporaryDirectory(prefix='rml32-2bc-adapter-') as tmp:
        path=Path(tmp)/'traces.jsonl';path.write_bytes(raw)
        argv=['cargo','test','--lib','--locked','--offline','lifecycle_adapter::tests::replay_adapter_traces','--','--ignored','--exact','--nocapture']
        r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,env=dict(os.environ,RML32_ADAPTER_TRACES=str(path)))
        require(r.returncode==0,r.stdout+r.stderr)
        matches=re.findall(r'^ADAPTER_REPLAY (.+)$',r.stdout,re.M)
        require(len(matches)==1 and json.loads(matches[0])==expected,'adapter replay inventory')
        log=dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr,environment=dict(RML32_ADAPTER_TRACES=str(path)),input_sha256=digest(raw))
        packed=gzip.compress(json.dumps(log,sort_keys=True).encode(),mtime=0)
        report=dict(stage='2BC',status='pass',replay=expected,input_sha256=digest(raw),source_sha256=sources(),log_sha256=digest(packed),native_execution=False,business_bytecode_execution=False,real_cell_execution=True,full_model_equivalence=False)
        return report,packed

def verify():
    r=load(HERE/'adapter-diff.json');raw,expected=generate()
    require(r['stage']=='2BC' and r['status']=='pass' and r['source_sha256']==sources(),'adapter source/scope')
    require(r['input_sha256']==digest(raw) and r['replay']==expected,'adapter generated input')
    lograw=(HERE/'adapter-diff-log.json.gz').read_bytes();require(digest(lograw)==r['log_sha256'],'adapter log hash')
    log=json.loads(gzip.decompress(lograw));matches=re.findall(r'^ADAPTER_REPLAY (.+)$',log['stdout'],re.M)
    require(log['returncode']==0 and len(matches)==1 and json.loads(matches[0])==expected and log['input_sha256']==digest(raw),'adapter actual log')
    require(r['native_execution'] is False and r['business_bytecode_execution'] is False and r['real_cell_execution'] is True and r['full_model_equivalence'] is False,'adapter claims')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');p.add_argument('--verify',action='store_true');a=p.parse_args()
    if a.verify:r=verify()
    else:
        r,packed=run()
        if a.write:
            (HERE/'adapter-diff.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'adapter-diff-log.json.gz').write_bytes(packed);verify()
    print(json.dumps(r,sort_keys=True))
