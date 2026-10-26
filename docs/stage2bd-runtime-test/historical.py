"""Pinned 2BC snapshot, retaining its own nested 2BB historical authority."""
import argparse,base64,gzip,json,re,subprocess,tempfile
from pathlib import Path
from vm_diff import ROOT,HERE,digest,load,require
COMMIT='1306a9bca296813f83df779fb62aa1a0f651d20c'
NESTED='7a215303c8828cfb6dd5c868499ef8f24c5d9d3d'
LOCATION='pinned 2BC snapshot plus its nested pinned 2BB replay; not current source'
CODE="import sys,json,base64;sys.path.insert(0,'docs/stage2bc-adapter');import historical;r,raw=historical.run();print(json.dumps(dict(report=r,logs_base64=base64.b64encode(raw).decode()),sort_keys=True))"
def count(out,n):require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==[str(n)] and out.rstrip().endswith('OK'),'historical Python result')
def run():
    logs=[]
    with tempfile.TemporaryDirectory(prefix='rml32-2bd-baseline-') as temp:
        clone=Path(temp)/'baseline'
        def command(argv,cwd):
            r=subprocess.run(argv,cwd=cwd,text=True,capture_output=True);logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr));require(r.returncode==0,r.stdout+r.stderr);return r
        command(['git','clone','--shared','--no-checkout',str(ROOT),str(clone)],ROOT);command(['git','checkout','--detach',COMMIT],clone)
        require(command(['git','rev-parse','HEAD'],clone).stdout.strip()==COMMIT,'historical commit')
        files=load(HERE/'baseline-inputs.json')['files'];require(len(files)==865,'historical inventory');restored=[]
        for path,h in files.items():
            target=clone/path;raw=target.read_bytes()
            if digest(raw)!=h:
                frozen=(ROOT/path).read_bytes();require(digest(frozen)==h and frozen.replace(b'\r\n',b'\n')==raw,'non-line-ending difference: '+path);target.write_bytes(frozen);restored.append(path)
        require({p:digest((clone/p).read_bytes()) for p in files}==files,'historical bytes')
        nested=json.loads(command(['python3','-B','-c',CODE],clone).stdout)
        require(nested['report']['historical_python_tests']==267 and nested['report']['source_commit']==NESTED,'nested authority')
        outer=command(['python3','-B','-m','unittest','discover','-s','docs/stage2bc-adapter','-p','test_*.py'],clone);count(outer.stdout+outer.stderr,25)
        command(['python3','-B','docs/stage2bc-adapter/verify_validation.py'],clone)
        packed=gzip.compress(json.dumps(logs,sort_keys=True).encode(),mtime=0)
        report=dict(stage='2BD',status='pass',source_commit=COMMIT,checked_files=865,historical_python_tests=292,outer_python_tests=25,nested_python_tests=267,nested_commit=NESTED,nested_report=nested['report'],checkout_line_ending_restorations=restored,baseline_files_sha256=digest(json.dumps(files,sort_keys=True).encode()),execution_location=LOCATION,network_used=False,log_sha256=digest(packed))
        return report,packed

def verify():
    r=load(HERE/'historical.json');raw=(HERE/'historical-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw))
    require(r['stage']=='2BD' and r['status']=='pass' and r['source_commit']==COMMIT and r['checked_files']==865 and r['historical_python_tests']==292,'historical scope')
    require(r['log_sha256']==digest(raw) and all(c['returncode']==0 for c in logs),'historical logs')
    require(r['execution_location']==LOCATION and r['network_used'] is False and r['nested_commit']==NESTED,'historical location')
    require(r['baseline_files_sha256']==digest(json.dumps(load(HERE/'baseline-inputs.json')['files'],sort_keys=True).encode()),'historical byte inventory')
    nested=[c for c in logs if c['argv']==['python3','-B','-c',CODE]];require(len(nested)==1,'nested invocation')
    value=json.loads(nested[0]['stdout']);require(value['report']==r['nested_report'],'nested report');nestedraw=base64.b64decode(value['logs_base64']);require(digest(nestedraw)==value['report']['log_sha256'],'nested log identity')
    children=json.loads(gzip.decompress(nestedraw));require(all(c['returncode']==0 for c in children),'nested failed command')
    nestedtests=[c for c in children if 'unittest' in c['argv']];require(len(nestedtests)==10,'nested test inventory')
    require(sum(int(re.findall(r'^Ran (\d+) tests? in ',c['stdout']+c['stderr'],re.M)[0]) for c in nestedtests)==267,'nested total')
    for c in nestedtests:
        out=c['stdout']+c['stderr'];require(out.rstrip().endswith('OK'),'nested test result')
    outer=[c for c in logs if 'unittest' in c['argv']];require(len(outer)==1,'outer test inventory');count(outer[0]['stdout']+outer[0]['stderr'],25)
    require(r['outer_python_tests']==25 and r['nested_python_tests']==267,'historical counts')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'historical.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'historical-logs.json.gz').write_bytes(raw)
    print(json.dumps(verify(),sort_keys=True))
