"""Independent snapshot-list maintenance oracle, distinct from Rust's range cursor.

Inputs are actual registration identities/liveness before work, never returned
counts or the implementation cursor. Internal scan fields are not oracle inputs.
"""
from common import require
class MaintenanceOracle:
 def __init__(self):self.pending=None;self.epoch=None;self.complete=False;self.seen_epoch=None
 def check_semantic_phase(self,action,before,after,actual,semantic):
  require(action['budget']['metadata_entries']==0,'semantic phase metadata budget')
  require(before['raw']['scan']==after['raw']['scan'],'non-metadata work advanced cursor')
  expected={k:semantic[k]for k in ('retired_roots','drained_objects','reaped_objects','reclaimed_heads','pruned_weak_entries','metadata_examined','pending_roots','queued_objects')}
  require(all(expected[k]==0 for k in ('reaped_objects','reclaimed_heads','pruned_weak_entries','metadata_examined')),'zero metadata budget reclaimed records')
  # These semantic traces never ran a metadata pass. Only an empty concrete
  # registration inventory can establish completion, irrespective of Rust's cursor.
  scan_complete=not after['raw']['metadata']
  stop=semantic['stop']if semantic['stop']in ('BorrowBlocked','WorkspaceLimit')else 'Complete'if not expected['pending_roots']and not expected['queued_objects']and scan_complete else 'BudgetExhausted'
  expected.update(scan_complete=scan_complete,stop=stop)
  require(actual==expected,'semantic-phase concrete progress differs')
  if not any(action['budget'].values()):require(before==after,'zero budget changed state')
 def check(self,action,before,after,actual):
  budget=action['budget'];limit=budget['metadata_entries'];b=before['raw'];end=after['raw']
  rows={(r['phase'],r['key']):r['eligible']for r in b['metadata']};epoch=b['epoch'];force=b['force']
  count=dict(retired_roots=0,drained_objects=0,reaped_objects=0,reclaimed_heads=0,pruned_weak_entries=0,metadata_examined=0,pending_roots=before['semantic']['pending_roots'],queued_objects=len(before['semantic']['queue']))
  require(budget['host_roots']==budget['objects']==0,'metadata oracle receives only metadata phase cases')
  blocked=bool(action.get('borrow_index'))and limit>0
  if not blocked:
   while count['metadata_examined']<limit:
    if self.complete and self.epoch==epoch and not force:break
    if force:self.pending=None;self.complete=False;force=False
    if self.pending is None:self.pending=sorted(rows);self.epoch=epoch;self.complete=False
    if not self.pending:
     if self.epoch==epoch:self.complete=True;break
     self.pending=None;continue
    key=self.pending.pop(0);count['metadata_examined']+=1
    if rows.get(key,False):
     del rows[key];field='reaped_objects'if key[0]==0 else 'reclaimed_heads'if key[0]==1 else 'pruned_weak_entries';count[field]+=1
     if epoch==2**64-1:epoch=0;force=True
     else:epoch+=1
    if not self.pending and self.epoch==epoch and not force:self.complete=True
  scan_complete=not rows or(self.complete and self.epoch==epoch and not force)
  count.update(scan_complete=scan_complete,stop='BorrowBlocked'if blocked else 'Complete'if not count['pending_roots']and not count['queued_objects']and scan_complete else 'BudgetExhausted')
  require(actual==count,'maintenance progress differs: expected '+str(count)+' actual '+str(actual))
  require({(r['phase'],r['key'])for r in end['metadata']}==set(rows),'metadata removed outside visited eligible entries')
  require(end['epoch']==epoch and end['force']==force,'maintenance mutation epoch/force differs')
  require(end['scan_complete']==scan_complete,'scan completion differs')
  if not any(budget.values()):require(before==after,'zero budget changed state')
  return count
