"""Run actual design-model cases and independent unsigned arithmetic boundary rows."""
import argparse,gzip,io,json,unittest
from common import ROOT,HERE,digest,load,require,encoded
from quota_model import LifecycleLimits,Rejected,checked_add
from contract_spec import verify as contract
LIMIT_FIELDS=list(LifecycleLimits.__dataclass_fields__)
def arithmetic():
 rows=[]
 for bits in (32,64):
  maximum=(1<<bits)-1
  for a in [0,1,maximum-1,maximum]:
   for b in [0,1,maximum-1,maximum]:
    expected=a+b if a+b<=maximum else None
    try:actual=checked_add(a,b,bits)
    except Rejected as error:require(error.kind=='Limit','arithmetic failure kind');actual=None
    require(actual==expected,'checked arithmetic disagreement')
    rows.append(dict(width=bits,kind='checked_add',a=str(a),b=str(b),expected=None if expected is None else str(expected)))
  for field in LIMIT_FIELDS:
   for value in [0,1,2,3,maximum,maximum+1,-1,True]:
    values=dict(max_objects=2,max_function_heads=2,max_ordinary_refs_per_object=4,max_exec_refs_per_object=2,max_owners_per_header=3);values[field]=value
    expected=type(value)is int and 0<=value<=maximum and (field!='max_objects' or value<=2**32-1) and (field!='max_ordinary_refs_per_object' or value>=2) and (field not in ('max_exec_refs_per_object','max_owners_per_header')or value>=1)
    try:LifecycleLimits(**values).validate(bits);actual=True
    except Rejected:actual=False
    require(actual==expected,'limit validation disagreement')
    rows.append(dict(width=bits,kind='limit',field=field,value=repr(value),accepted=expected))
 return rows

def sources():return {str(p.relative_to(ROOT)):digest(p.read_bytes())for p in [HERE/'quota_model.py',HERE/'test_model.py',HERE/'check_contract.py',HERE/'contract_spec.py']}
def run():
 contract();rows=arithmetic();suite=unittest.defaultTestLoader.discover(str(HERE),pattern='test_model.py')
 def names(test):
  if isinstance(test,unittest.TestSuite):return [n for child in test for n in names(child)]
  return [test.id()]
 expected=sorted(names(suite));stream=io.StringIO();result=unittest.TextTestRunner(stream=stream,verbosity=2).run(suite);require(result.wasSuccessful() and not result.skipped and result.testsRun==len(expected),stream.getvalue());packed=gzip.compress(encoded(dict(stage='2BH',runner='unittest.TextTestRunner',test_names=expected,tests_run=result.testsRun,success=True,output=stream.getvalue())),mtime=0)
 return dict(stage='2BH',status='pass',model_cases=len(expected),model_case_names=expected,arithmetic_rows=len(rows),arithmetic_sha256=digest(encoded(rows)),source_sha256=sources(),log_sha256=digest(packed),rust_implementation_validated=False,full_state_space_enumeration=False),packed,rows

def verify():
 contract();r=load(HERE/'contract-checks.json');rows=arithmetic();raw=(HERE/'contract-checks-log.json.gz').read_bytes();log=json.loads(gzip.decompress(raw));require(r['stage']=='2BH' and r['status']=='pass' and r['source_sha256']==sources() and r['log_sha256']==digest(raw),'contract log identity');require(log['stage']=='2BH' and log['success']is True and log['test_names']==r['model_case_names'] and log['tests_run']==r['model_cases']==len(set(r['model_case_names'])),'case inventory');require(all((' ('+n+') ... ok')in log['output']for n in r['model_case_names']) and log['output'].rstrip().endswith('OK'),'actual case results');require(r['arithmetic_rows']==len(rows) and r['arithmetic_sha256']==digest(encoded(rows)) and (HERE/'arithmetic.json').read_bytes()==encoded(rows),'arithmetic report');require(r['rust_implementation_validated']is False and r['full_state_space_enumeration']is False,'model scope');return r
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
 if a.write:
  r,raw,rows=run();(HERE/'contract-checks.json').write_bytes(encoded(r));(HERE/'contract-checks-log.json.gz').write_bytes(raw);(HERE/'arithmetic.json').write_bytes(encoded(rows))
 print(json.dumps(verify(),sort_keys=True))
