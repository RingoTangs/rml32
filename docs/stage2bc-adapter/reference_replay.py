"""Rerun frozen 2BB kernel on current build; save outputs only in 2BC."""
import argparse,gzip,json,re,sys
from adapter_diff import ROOT,HERE,load,digest,require
sys.path.insert(0,str(ROOT/'docs/stage2bb-kernel'))
import differential as reference
sys.path.pop(0)
def verify():
    r=load(HERE/'reference-replay.json');base=load(ROOT/'docs/stage2bb-kernel/differential.json')
    raw=(HERE/'reference-replay-log.json.gz').read_bytes();log=json.loads(gzip.decompress(raw))
    require(r['domains']==base['domains'] and r['input_sha256']==base['input_sha256'] and r['replay']==base['replay'],'reference domain regression')
    require(r['source_sha256']==reference.sources() and r['replay_log_sha256']==digest(raw),'reference source/log')
    matches=re.findall(r'^LIFECYCLE_REPLAY (.+)$',log['stdout'],re.M)
    require(log['returncode']==0 and len(matches)==1 and json.loads(matches[0])==r['replay'],'reference actual result')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        r,raw=reference.run();(HERE/'reference-replay.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n');(HERE/'reference-replay-log.json.gz').write_bytes(raw)
    print(json.dumps(verify(),sort_keys=True))
