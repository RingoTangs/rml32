"""Run frozen Python checks in a pinned local clone, never current-source hashes."""
import argparse,gzip,json,re,subprocess,tempfile
from pathlib import Path
from adapter_diff import ROOT,HERE,digest,load,require
COMMIT='7a215303c8828cfb6dd5c868499ef8f24c5d9d3d'
SUITES=[('docs/stage2as-design','test_model.py',50),('docs/stage2as-design','test_explore.py',6),
('docs/stage2ax-evidence','test_verify_evidence.py',37),('docs/stage2ax-evidence','test_verify_validation.py',15),
('docs/stage2ay-evidence','test_verify_evidence.py',20),('docs/stage2ay-evidence','test_verify_validation.py',15),
('docs/stage2az-evidence','test_verify_evidence.py',37),('docs/stage2az-evidence','test_verify_validation.py',15),
('docs/stage2ba-design','test_*.py',40),('docs/stage2bb-kernel','test_*.py',32)]
def run():
    logs=[]
    with tempfile.TemporaryDirectory(prefix='rml32-2bc-baseline-') as temp:
        clone=Path(temp)/'baseline'
        def command(argv,cwd):
            r=subprocess.run(argv,cwd=cwd,text=True,capture_output=True)
            logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout,stderr=r.stderr))
            require(r.returncode==0,r.stdout+r.stderr);return r
        command(['git','clone','--shared','--no-checkout',str(ROOT),str(clone)],ROOT)
        command(['git','checkout','--detach',COMMIT],clone)
        head=command(['git','rev-parse','HEAD'],clone).stdout.strip();require(head==COMMIT,'historical commit')
        paths=command(['git','ls-tree','-r','--name-only',COMMIT],clone).stdout.splitlines();require(len(paths)==839,'historical file inventory')
        expected_files=load(HERE/'baseline-inputs.json')['files'];restored=[]
        for path in paths:
            target=clone/path;raw=target.read_bytes()
            if digest(raw)!=expected_files[path]:
                frozen=(ROOT/path).read_bytes()
                require(digest(frozen)==expected_files[path] and frozen.replace(b'\r\n',b'\n')==raw,'non-line-ending baseline difference: '+path)
                target.write_bytes(frozen)
                restored.append(dict(path=path,checkout_sha256=digest(raw),frozen_sha256=digest(frozen),method='restore verified frozen CRLF bytes; normalized contents identical'))
        files={p:digest((clone/p).read_bytes()) for p in paths}
        require(files==expected_files,'historical byte identity')
        counts={}
        for directory,pattern,expected in SUITES:
            argv=['python3','-B','-m','unittest','discover','-s',directory,'-p',pattern]
            r=command(argv,clone);out=r.stdout+r.stderr
            n=re.findall(r'^Ran (\d+) tests? in ',out,re.M)
            require(n==[str(expected)] and out.rstrip().endswith('OK'),'historical Python result')
            counts[directory+'/'+pattern]=expected
        command(['python3','-B','docs/stage2bb-kernel/verify_validation.py'],clone)
        packed=gzip.compress(json.dumps(logs,sort_keys=True).encode(),mtime=0)
        return dict(stage='2BC',status='pass',source_commit=head,checked_files=839,baseline_files_sha256=digest(json.dumps(files,sort_keys=True).encode()),historical_python_tests=sum(counts.values()),python_counts=counts,checkout_line_ending_restorations=restored,execution_location='temporary pinned baseline clone; not current source',network_used=False,log_sha256=digest(packed)),packed

def verify():
    r=load(HERE/'historical.json');raw=(HERE/'historical-logs.json.gz').read_bytes();logs=json.loads(gzip.decompress(raw))
    require(r['source_commit']==COMMIT and r['checked_files']==839 and r['historical_python_tests']==267,'historical scope')
    require(r['log_sha256']==digest(raw) and all(c['returncode']==0 for c in logs),'historical logs')
    require(r['baseline_files_sha256']==digest(json.dumps(load(HERE/'baseline-inputs.json')['files'],sort_keys=True).encode()),'historical identity')
    for restoration in r['checkout_line_ending_restorations']:
        path=restoration['path'];raw=(ROOT/path).read_bytes()
        require(restoration['frozen_sha256']==digest(raw) and restoration['checkout_sha256']==digest(raw.replace(b'\r\n',b'\n')),'line ending restoration identity')
    require(r['python_counts']=={d+'/'+p:n for d,p,n in SUITES},'historical counts')
    observed=[c for c in logs if 'unittest' in c['argv']];require(len(observed)==len(SUITES),'historical commands')
    for c,(directory,pattern,n) in zip(observed,SUITES):
        require(c['argv']==['python3','-B','-m','unittest','discover','-s',directory,'-p',pattern],'historical argv')
        out=c['stdout']+c['stderr'];require(re.findall(r'^Ran (\d+) tests? in ',out,re.M)==[str(n)] and out.rstrip().endswith('OK'),'historical result')
    require(r['execution_location']=='temporary pinned baseline clone; not current source' and r['network_used'] is False,'historical claims')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=run();(HERE/'historical.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'historical-logs.json.gz').write_bytes(raw)
    print(json.dumps(verify(),sort_keys=True))
