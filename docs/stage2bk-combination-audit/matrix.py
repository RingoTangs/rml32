import itertools,importlib.util
from common import ROOT
s=importlib.util.spec_from_file_location('_2bk_matrix',ROOT/'docs/stage2bj-contract-audit/matrix.py');old=importlib.util.module_from_spec(s);s.loader.exec_module(old)
def profiles():
 p=[t['limits']for t in old.traces()if t['family']=='F01'];p.append(dict(zip(old.FIELDS,(2,2,6,2,8))));assert len(p)==163;return p
LAYOUTS=['independent','fanout','same_equal','same_conflict','copy_move_alias','moves']
INTERFERENCE=['s0','s1','d0','d1','retire','zero']
def batches():
 prefixes=[[]]+[[x]for x in INTERFERENCE]+[list(x)for x in itertools.product(INTERFERENCE,repeat=2)]
 for p,q in enumerate(profiles()):
  for layout in LAYOUTS:
   for n,prefix in enumerate(prefixes):
    for terminal in ('commit','abort'):
     yield dict(id=f'B{p:03}-{layout}-{n:02}-{terminal}',group='batch',limits=q,actions=[dict(op='setup'),dict(op='prepare',layout=layout)]+[dict(op='interfere',target=x)for x in prefix]+[dict(op=terminal)])
def mappings():
 for p,q in enumerate(profiles()):
  for reuse,payload,iteration,caught in itertools.product((False,True),('object','shell','nested'),(0,1),(False,True)):
   yield dict(id=f'M{p:03}-{int(reuse)}-{payload}-{iteration}-{int(caught)}',group='mapping',limits=q,reuse=reuse,payload=payload,iteration=iteration,caught=caught)
def closes():
 orders=[x for x in itertools.permutations(('drop0','drop1','maintain','close'))if x.index('maintain')<x.index('close')]
 for p,q in enumerate(profiles()):
  for state,budget,order in itertools.product(('queued','active'),(0,1),orders):
   yield dict(id=f'C{p:03}-{state}-{budget}-'+'.'.join(order),group='close',limits=q,state=state,budget=budget,actions=list(order))
