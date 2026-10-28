"""Execute the unchanged D08 quota specification; write only 2BJ artifacts."""
import argparse,base64,gzip,json,subprocess
from common import ROOT,HERE,digest,load,require,encoded
CODE="import sys,json,base64;sys.path.insert(0,'docs/stage2bh-production-contract');import check_contract;r,raw,rows=check_contract.run();print(json.dumps(dict(report=r,log_base64=base64.b64encode(raw).decode(),arithmetic=rows),sort_keys=True))"
def run():
    c=subprocess.run(['python3','-B','-c',CODE],cwd=ROOT,text=True,capture_output=True);require(c.returncode==0,c.stdout+c.stderr)
    result=json.loads(c.stdout);raw=base64.b64decode(result.pop('log_base64'),validate=True)
    return dict(stage='2BJ',status='pass',model=result['report'],arithmetic=result['arithmetic'],source='frozen 2BH quota model; current execution; not Rust/model equivalence',log_sha256=digest(raw)),raw
def verify():
    r=load(HERE/'model-replay.json');old=load(ROOT/'docs/stage2bh-production-contract/contract-checks.json');require(r['stage']=='2BJ' and r['status']=='pass','model stage')
    require({k:v for k,v in r['model'].items()if k!='log_sha256'}=={k:v for k,v in old.items()if k!='log_sha256'},'model inventory')
    raw=(HERE/'model-log.json.gz').read_bytes();log=json.loads(gzip.decompress(raw));require(digest(raw)==r['log_sha256']==r['model']['log_sha256'],'model log hash');require(log['success'] and log['tests_run']==39 and log['test_names']==old['model_case_names'] and log['output'].rstrip().endswith('OK'),'model execution')
    require(r['arithmetic']==load(ROOT/'docs/stage2bh-production-contract/arithmetic.json') and len(r['arithmetic'])==112,'arithmetic identity');return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'model-replay.json').write_bytes(encoded(r));(HERE/'model-log.json.gz').write_bytes(raw)
    print(json.dumps(verify(),sort_keys=True))
