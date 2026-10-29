import argparse,base64,gzip,json,re,subprocess,tempfile
from pathlib import Path
from common import ROOT,HERE,COMMIT,load,require,digest,encoded
from snapshot import materialize
CODE="import sys,json,base64;sys.path.insert(0,'docs/stage2bm-interleaving-audit');import historical;r,l=historical.run();print(json.dumps(dict(report=r,logs_base64=base64.b64encode(l).decode()),sort_keys=True))"
def run():
    logs=[]
    with tempfile.TemporaryDirectory(prefix='rml32-2bn-history-') as tmp:
        clone=Path(tmp)/'baseline';b=materialize(clone,logs)
        def command(argv):
            r=subprocess.run(argv,cwd=clone,text=True,capture_output=True);logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr));require(r.returncode==0,r.stdout+r.stderr);return r
        child=json.loads(command(['python3','-B','-c',CODE]).stdout);require(child['report']['historical_python_tests']==557,'nested history count')
        r=command(['python3','-B','-m','unittest','discover','-s','docs/stage2bm-interleaving-audit','-p','test_*.py'])
        require(re.findall(r'^Ran (\d+) tests? in ',r.stdout+r.stderr,re.M)==['38'] and (r.stdout+r.stderr).rstrip().endswith('OK'),'snapshot Python count')
        command(['python3','-B','docs/stage2bm-interleaving-audit/verify_validation.py'])
    raw=gzip.compress(encoded(logs),mtime=0)
    return dict(stage='2BN',status='pass',base_commit=COMMIT,overlay_sha256=b['overlay_sha256'],files_sha256=digest(encoded(b['files'])),baseline_files=len(b['files']),historical_python_tests=595,outer_python_tests=38,nested_python_tests=557,execution_location='reconstructed byte-frozen 2BM snapshot and nested fixed authorities',network_used=False,log_sha256=digest(raw)),raw
def verify():
    r=load(HERE/'historical.json');raw=(HERE/'historical-log.json.gz').read_bytes();logs=load(HERE/'historical-log.json.gz');b=load(HERE/'baseline-inputs.json')
    require(r['stage']=='2BN' and r['status']=='pass' and r['base_commit']==COMMIT and r['baseline_files']==len(b['files']) and r['historical_python_tests']==595 and r['overlay_sha256']==b['overlay_sha256'] and r['files_sha256']==digest(encoded(b['files'])) and r['log_sha256']==digest(raw),'history identity')
    require(len(logs)==6 and all(c['returncode']==0 for c in logs),'history commands')
    child=json.loads(logs[3]['stdout']);require(child['report']['historical_python_tests']==557 and digest(base64.b64decode(child['logs_base64'],validate=True))==child['report']['log_sha256'],'nested history')
    require(re.findall(r'^Ran (\d+) tests? in ',logs[4]['stdout']+logs[4]['stderr'],re.M)==['38'],'snapshot checks')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');args=p.parse_args()
    if args.write:
        r,l=run();(HERE/'historical.json').write_bytes(encoded(r));(HERE/'historical-log.json.gz').write_bytes(l)
    print(json.dumps(verify(),sort_keys=True))
