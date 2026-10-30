"""Replay a fixed small current-source matrix and compare with the independent model."""
import argparse,gzip,hashlib,json,os,re,subprocess,tempfile
from pathlib import Path
from model import cases,Reference
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
DRIVER='startup_probe::mkdir_tests::replay_mkdir_cases'
def digest(raw):return hashlib.sha256(raw).hexdigest()
def encode(v):return (json.dumps(v,sort_keys=True,indent=2)+'\n').encode()
def sources():
    paths=list((ROOT/'src').glob('*.rs'))+[HERE/'focused.py',HERE/'model.py']
    return {str(p.relative_to(ROOT)):digest(p.read_bytes()) for p in sorted(paths)}
def compare(ts,rows):
    assert len(ts)==len(rows),'case count';checks=0;results={}
    for t,row in zip(ts,rows):
        assert row['id']==t['id'],'case order';r=Reference(t)
        assert row['initial']==r.projection(),t['id']+' initial'
        assert len(row['steps'])==len(t['budgets']),'step count'
        for budget,step in zip(t['budgets'],row['steps']):
            error=r.advance(budget)
            assert step['error']==error,(t['id'],'error',step['error'],error)
            assert step['report']==r.projection(),(t['id'],r.pc,'state')
            checks+=1
        key=r.failure or str(r.events[-1]['call']['result'])
        results[key]=results.get(key,0)+1
    return dict(traces=len(ts),checkpoints=checks,results=results,unclassified_differences=0)
def run():
    ts=list(cases());before=sources();logs=[];observations=None;summary=None
    raw_input=''.join(json.dumps(t,sort_keys=True)+'\n' for t in ts).encode()
    for profile in ('debug','release'):
        with tempfile.TemporaryDirectory(prefix='rml32-2bp-focused-') as tmp:
            p=Path(tmp);(p/'in').write_bytes(raw_input)
            argv=['cargo','test']+(['--release'] if profile=='release' else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture']
            env=dict(os.environ,RML32_MKDIR_IN=str(p/'in'),RML32_MKDIR_OUT=str(p/'out'))
            result=subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True)
            assert result.returncode==0,result.stdout+result.stderr
            raw=(p/'out').read_bytes();rows=[json.loads(l) for l in raw.splitlines()]
            current=compare(ts,rows)
            assert summary is None or summary==current;summary=current
            assert re.findall(r'^MKDIR_REPLAY (\d+)$',result.stdout,re.M)==[str(len(ts))]
            compressed=gzip.compress(raw,mtime=0)
            assert observations is None or observations==compressed,'profile raw observations differ';observations=compressed
            logs.append(dict(profile=profile,argv=argv,returncode=result.returncode,stdout=result.stdout,stderr=result.stderr,input_sha256=digest(raw_input),observations_sha256=digest(compressed)))
    assert before==sources(),'source changed during focused replay'
    (HERE/'focused-observations.jsonl.gz').write_bytes(observations)
    log=gzip.compress(encode(logs),mtime=0);(HERE/'focused-log.json.gz').write_bytes(log)
    report=dict(stage='2BP',status='pass',summary=summary,source_sha256=before,input_sha256=digest(raw_input),observations_sha256=digest(observations),log_sha256=digest(log),scope='current small synthetic mkdir matrix; original 17 large drivers are historical only')
    (HERE/'focused.json').write_bytes(encode(report));return report
def verify():
    report=json.loads((HERE/'focused.json').read_bytes());ts=list(cases())
    raw=(HERE/'focused-observations.jsonl.gz').read_bytes();log=(HERE/'focused-log.json.gz').read_bytes()
    assert report['stage']=='2BP' and report['status']=='pass'
    assert report['source_sha256']==sources(),'focused source identity'
    assert report['observations_sha256']==digest(raw) and report['log_sha256']==digest(log)
    assert report['input_sha256']==digest(''.join(json.dumps(t,sort_keys=True)+'\n' for t in ts).encode())
    assert report['summary']==compare(ts,[json.loads(l) for l in gzip.decompress(raw).splitlines()])
    logs=json.loads(gzip.decompress(log));assert [l['profile'] for l in logs]==['debug','release']
    for l in logs:
        expected=['cargo','test']+(['--release'] if l['profile']=='release' else [])+['--locked','--offline','--lib',DRIVER,'--','--ignored','--exact','--nocapture']
        assert l['argv']==expected and l['returncode']==0
        assert l['input_sha256']==report['input_sha256'] and l['observations_sha256']==report['observations_sha256']
        assert re.findall(r'^MKDIR_REPLAY (\d+)$',l['stdout'],re.M)==[str(len(ts))]
    return report
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true')
    if p.parse_args().write:run()
    print(json.dumps(verify(),sort_keys=True))
