"""Independent small byte-path/directory reference; never imports Rust observations."""
import itertools

PARENTS=('/', '/aaa', '/ccs', '/csa', '/dba', '/gs')
OFFSET=450136
SHA='f9896828c87399be4977107fbfbe3f67fd71caebd3008bda84be16cc722d0a0e'

class Rejected(Exception):pass

def regular(raw):
    value=raw.split(b'\0',1)[0]
    if not value.startswith(b'/'):raise Rejected('UnsupportedFeature')
    parts=[p for p in value.split(b'/') if p]
    if any(p in (b'.',b'..') for p in parts):raise Rejected('UnsupportedFeature')
    path=b'/'+b'/'.join(parts)
    if len(path)>255:raise Rejected('Bounds')
    classified=path+b'/'
    if classified.startswith(b'/v/') or b'.pak/' in classified or b'.dbfs/' in classified:
        raise Rejected('UnsupportedFeature')
    return path

def cases():
    paths=[b'/aaa/log/',b'//aaa///log//',b'/',b'/aaa/',b'/aaa/new',b'/missing/leaf',
           b'/aaa/file/child',b'/aaa/\xff/',b'/aaa/log/\0ignored',b'relative',b'',
           b'/aaa/./log',b'/aaa/../log',b'/v/x',b'/a.pak/x',b'/a.dbfs/x',b'/'+b'x'*254,b'/'+b'x'*255]
    nodes=[dict(path=p,directory=True) for p in PARENTS]
    envs=[('base',nodes,16),('existing',nodes+[dict(path='/aaa/log',directory=True)],16),
          ('missing',[n for n in nodes if n['path']!='/aaa'],16),
          ('parent_file',[dict(n,directory=False) if n['path']=='/aaa' else n for n in nodes],16),
          ('file',nodes+[dict(path='/aaa/log',directory=False),dict(path='/aaa/file',directory=False)],16),
          ('quota',nodes,6)]
    budgets=[[28],[0,25,1,1,1,0,100],[26,1,1,1]]
    for (pi,path),(name,seed,maximum),(bi,schedule) in itertools.product(enumerate(paths),envs,enumerate(budgets)):
        yield dict(id=f'{pi:02}-{name}-{bi}',path_hex=path.hex(),nodes=seed,max_nodes=maximum,budgets=schedule)

def byte_string(value):return dict(bytes_hex=value.hex())

class Reference:
    def __init__(self,case):
        self.case=case;self.pc=0;self.failure=None;self.calls=[];self.events=[]
        self.nodes={n['path'].encode():n['directory'] for n in case['nodes']}
    def advance(self,budget):
        if self.failure:return self.failure
        for _ in range(min(budget,28-self.pc)):
            if self.pc==1:
                call=dict(slot=0,name='allocate_array',pc=1,offset=OFFSET+4,argument=3)
                self.calls.append(call);self.events.append(dict(operation='allocate_array',call=call))
            if self.pc==26:
                raw=bytes.fromhex(self.case['path_hex']).split(b'\0',1)[0]
                call=dict(slot=336,name='mkdir',pc=26,offset=OFFSET+104,argument=byte_string(raw),normalized=None,result=None,error=None)
                self.events.append(dict(operation='mkdir',call=call))
                try:
                    path=regular(raw);call['normalized']=byte_string(path)
                    parent=path.rsplit(b'/',1)[0] or b'/'
                    if path in self.nodes or not self.nodes.get(parent,False):result=0
                    else:
                        if len(self.nodes)>=self.case['max_nodes']:raise Rejected('Limit')
                        self.nodes[path]=True;result=1
                    call['result']=result
                except Rejected as e:
                    self.failure=str(e);call['error']=self.failure;return self.failure
            self.pc+=1
        return None
    def projection(self):
        globals=[dict(type='undefined') for _ in range(105)]
        for after,slot,value in [(3,2,dict(type='array',elements=[dict(type='undefined') for _ in range(3)])),
            (7,3,dict(type='int',value=2)),(11,4,dict(type='int',value=0)),
            (15,5,dict(type='bytes',bytes_hex=b'/xdbc/'.hex())),(19,6,dict(type='mapping',entries=[])),(23,7,dict(type='int',value=0))]:
            if self.pc>after:globals[slot]=value
        return dict(source='sha256:'+SHA,sha256=SHA,function_base=572,original_init_instructions=661,
                    executed_instructions=self.pc,stop_pc=self.pc,stop_offset=OFFSET+4*self.pc,
                    stop_reason='failed' if self.failure else 'prefix_boundary' if self.pc==28 else 'budget_exhausted',
                    initialization_complete=False,host_calls=self.calls,globals=globals,
                    next_dependency=dict(pc=30,offset=OFFSET+120,slot=583,name='xdbc_set_output_basic_file'),
                    configuration=dict(values=dict(max_host_roots=32,max_graph_nodes=8192,max_depth=16,max_items=4096),
                        vm=dict(max_stack=64,max_frames=2,max_reference_cells=64),
                        lifecycle=dict(max_objects=0,max_function_heads=0,max_ordinary_refs_per_object=2,max_exec_refs_per_object=1,max_owners_per_header=8),
                        filesystem=dict(backend='memory',max_nodes=self.case['max_nodes'],max_path_bytes=255)),
                    slice='log_directory',host_events=self.events,
                    filesystem=dict(entries=[dict(path=byte_string(p),directory=self.nodes[p]) for p in sorted(self.nodes)]),
                    next_dependency_kind='global_function',next_dependency_reason='atomic_function_not_enabled')
