"""Deterministic, bounded cases. No expectations are supplied to Rust."""
import itertools
FIELDS=['max_objects','max_function_heads','max_ordinary_refs_per_object','max_exec_refs_per_object','max_owners_per_header']
ZERO=dict(host_roots=0,objects=0,metadata_entries=0)
def act(op,**kw):return dict(op=op,**kw)
def obj(n='o0'):return act('object',id=n)
def head(kind='array',n='h0',dest='a',creator=None):return act('head',id=n,kind=kind,dest=dest,creator=creator)
def put(dest,kind='scalar',identity=None,**kw):return act('put',dest=dest,value=[kind,identity],**kw)
def copy(source,dest,**kw):return act('copy',source=source,dest=dest,**kw)
def maintain(roots=0,objects=0,metadata=0,**kw):return act('maintain',budget=dict(host_roots=roots,objects=objects,metadata_entries=metadata),**kw)
def drop(slot):return act('drop',slot=slot)
def execution(n='e0',o='o0'):return act('exec',id=n,object=o)
def exit(n='e0',**kw):return act('exit',id=n,**kw)
def delete(o='o0'):return act('delete',object=o)
def queued_pair():return [obj(),obj('o1'),execution(),execution('e1','o1'),delete(),delete('o1'),exit(),exit('e1')]
def families():return [
 ('F01','allocation_identity',[obj(),obj('o1'),obj('o2')]),
 ('F02','head_creator',[obj(),head('function',creator='o0'),copy('a','b'),head('function','h1','c','o0')]),
 ('F03','ordinary_queue',[obj(),put('s','object','o0'),execution(),delete(),exit(),maintain(),maintain(objects=1),drop('s')]),
 ('F04','peak_copy',[head(),copy('a','b'),copy('a','b')]),
 ('F05','raw_move',[head(),act('move',source='a',dest='b'),act('move',source='b',dest='c'),act('move',source='c',dest='a'),drop('a')]),
 ('F06','execution_abort',[obj(),execution(),act('exec',id='e1',object='o0',continue_on_error=True),delete(),exit(),exit('e1',continue_on_error=True),maintain(objects=1)]),
 ('F07','raw_script_copy',[obj(),put('host:r','object','o0'),delete(),copy('host:r','s'),copy('s','t',script=True),drop('s'),drop('host:r')]),
 ('F08','dag_final_release',[obj(),obj('o1'),head(),head('function','h1','f','o0'),put('mem:h1:0','header','h0'),put('mem:h0:0','object','o1'),drop('a'),act('view',header='h1'),drop('f'),act('drop_view',header='h1')]),
 ('F09','lifo_task',queued_pair()+[act('task_cleanup',object='o1'),maintain(objects=1),maintain(objects=1)]),
 ('F10','version_publication',[obj(),put('s'),act('prepare_copy',token='t',source='s',dest='out'),put('s','object','o0'),act('commit_copy',token='t',continue_on_error=True),act('prepare_head',id='h0',kind='function',dest='f',creator='o0',token='h'),delete(),act('commit_head',token='h',creator='o0',continue_on_error=True),drop('s'),obj('o1'),execution('ep','o1'),act('prepare_head',id='h1',kind='function',dest='pending',creator='o1',token='p'),delete('o1'),act('commit_head',token='p',creator='o1'),exit('ep'),maintain(objects=1)]),
 ('F11','host_retirement',[obj(),put('host:r1','object','o0'),put('host:r2','object','o0'),act('retire_host',slot='host:r2'),act('retire_host',slot='host:r1'),maintain(roots=1),maintain(roots=1)]),
 ('F12','partial_maintenance',queued_pair()+[maintain(objects=2,borrow_object='o0'),maintain(objects=2,workspace_after=0),maintain(objects=2)]),
 ('F13','metadata_epoch',[obj(),head(),head('function','h1','f','o0'),drop('f'),drop('a'),maintain(metadata=1),maintain(metadata=1,borrow_index='heads'),act('epoch_wrap'),maintain(),maintain(metadata=1),obj('o1')|dict(continue_on_error=True),maintain(metadata=64),maintain(metadata=64)]),
 ('F14','physical_origin',[obj(),head('function',creator='o0'),act('view',header='h0'),delete(),drop('a'),maintain(metadata=1),act('foreign',continue_on_error=True),act('drop_view',header='h0'),maintain(metadata=64),maintain(metadata=64)]),
 ('F15','transfer_close',[obj(),put('host:r','object','o0'),act('transfer'),maintain(),act('close')]),
 ('F16','vm_combination',[act('vm_pending',id='o0')]),
]
def traces():
 result=[]
 for i,values in enumerate(itertools.product((0,1,2),(0,1,2),(2,3,4),(1,2),(1,2,3))):
  for family,name,actions in families():result.append(dict(id=f'P{i:03}-{family}',profile=i,family=family,name=name,limits=dict(zip(FIELDS,values)),actions=actions))
 assert len(result)==2592
 return result
