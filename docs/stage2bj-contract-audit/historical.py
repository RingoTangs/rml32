"""465 pinned Python checks: outer 2BI's 10 plus unchanged nested 455."""
import argparse,base64,gzip,json,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,COMMIT,digest,load,require
CODE="import sys,json,base64;sys.path.insert(0,'docs/stage2bi-production-lifecycle');import historical;r,raw=historical.run();print(json.dumps(dict(report=r,logs_base64=base64.b64encode(raw).decode()),sort_keys=True))"
LOCATION='pinned 2BI snapshot plus its nested pinned 2BH/2BG/2BF/2BE/2BD/2BC/2BB authorities; not current source'
def count(out,n):require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==[str(n)] and out.rstrip().endswith('OK'),'historical test count')
def run():
    logs=[]
    with tempfile.TemporaryDirectory(prefix='rml32-2bj-baseline-')as temp:
        clone=Path(temp)/'baseline'
        def command(argv,cwd):
            r=subprocess.run(argv,cwd=cwd,text=True,capture_output=True);logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr));require(r.returncode==0,r.stdout+r.stderr);return r
        command(['git','clone','--shared','--dissociate','--no-checkout',str(ROOT),str(clone)],ROOT);command(['git','checkout','--detach',COMMIT],clone)
        require(command(['git','rev-parse','HEAD'],clone).stdout.strip()==COMMIT,'historical checkout')
        files=load(HERE/'baseline-inputs.json')['files'];restored=[]
        for p,h in files.items():
            path=clone/p;raw=path.read_bytes()
            if digest(raw)!=h:
                frozen=(ROOT/p).read_bytes();require(digest(frozen)==h and frozen.replace(b'\r\n',b'\n')==raw,'non-line-ending difference '+p);path.write_bytes(frozen);restored.append(p)
        require(len(files)==1048 and all(digest((clone/p).read_bytes())==h for p,h in files.items()),'historical identity')
        child=json.loads(command(['python3','-B','-c',CODE],clone).stdout)
        require(child['report']['historical_python_tests']==455 and child['report']['source_commit']=='f99812460c374adb49bdffd42dee84bcc1f86c8c','nested authority')
        outer=command(['python3','-B','-m','unittest','discover','-s','docs/stage2bi-production-lifecycle','-p','test_*.py'],clone);count(outer.stdout+outer.stderr,10)
        command(['python3','-B','docs/stage2bi-production-lifecycle/verify_validation.py'],clone)
        packed=gzip.compress(json.dumps(logs,sort_keys=True).encode(),mtime=0)
        return dict(stage='2BJ',status='pass',source_commit=COMMIT,checked_files=1048,historical_python_tests=465,outer_python_tests=10,nested_python_tests=455,nested_report=child['report'],execution_location=LOCATION,network_used=False,checkout_line_ending_restorations=restored,baseline_files_sha256=digest(json.dumps(files,sort_keys=True).encode()),log_sha256=digest(packed)),packed

def verify():
    r=load(HERE/'historical.json');packed=(HERE/'historical-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(packed))
    require(r['stage']=='2BJ' and r['status']=='pass' and r['source_commit']==COMMIT and r['checked_files']==1048 and r['historical_python_tests']==465,'historical scope')
    require(r['log_sha256']==digest(packed) and all(c['returncode']==0 for c in logs),'historical logs')
    require(r['execution_location']==LOCATION and r['network_used'] is False,'historical location')
    require(r['baseline_files_sha256']==digest(json.dumps(load(HERE/'baseline-inputs.json')['files'],sort_keys=True).encode()),'historical inventory')
    children=[c for c in logs if c['argv']==['python3','-B','-c',CODE]];require(len(children)==1,'historical child invocation')
    child=json.loads(children[0]['stdout']);require(child['report']==r['nested_report'] and child['report']['historical_python_tests']==455 and child['report']['source_commit']=='f99812460c374adb49bdffd42dee84bcc1f86c8c','child identity')
    raw=base64.b64decode(child['logs_base64'],validate=True);require(digest(raw)==child['report']['log_sha256'],'child log hash');nested=json.loads(gzip.decompress(raw));require(all(c['returncode']==0 for c in nested),'child command failures')
    outer=[c for c in logs if 'unittest' in c['argv']];require(len(outer)==1,'outer test command');count(outer[0]['stdout']+outer[0]['stderr'],10)
    require(r['outer_python_tests']==10 and r['nested_python_tests']==455,'historical counts')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'historical.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'historical-logs.json.gz').write_bytes(raw)
    print(json.dumps(verify(),sort_keys=True))
