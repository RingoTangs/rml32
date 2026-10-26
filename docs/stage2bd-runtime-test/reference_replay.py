"""Reexecute frozen adapter and kernel generators; save new logs only under 2BD."""
import argparse,gzip,json,re,sys
from vm_diff import ROOT,HERE,load,digest,require
sys.path.insert(0,str(ROOT/'docs/stage2bc-adapter'))
import adapter_diff as adapter
sys.path.pop(0)
sys.path.insert(0,str(ROOT/'docs/stage2bb-kernel'))
import differential as kernel
sys.path.pop(0)
def verify():
    r=load(HERE/'reference-replay.json')
    for key,basepath,logname,resultkey,prefix,source in [('adapter','docs/stage2bc-adapter/adapter-diff.json','reference-adapter-log.json.gz','replay','ADAPTER_REPLAY ',adapter),('kernel','docs/stage2bb-kernel/differential.json','reference-kernel-log.json.gz','replay','LIFECYCLE_REPLAY ',kernel)]:
        base=load(ROOT/basepath);part=r[key];packed=(HERE/logname).read_bytes();log=json.loads(gzip.decompress(packed));expected=base[resultkey]
        require(part['replay']==expected and part['input_sha256']==base['input_sha256'] and part['source_sha256']==source.sources(),'reference identity')
        hkey='log_sha256' if key=='adapter' else 'replay_log_sha256';require(part[hkey]==digest(packed) and log['returncode']==0,'reference log')
        matches=re.findall('^'+prefix+'(.+)$',log['stdout'],re.M);require(len(matches)==1 and json.loads(matches[0])==expected,'reference actual result')
    return r
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--write',action='store_true');a=p.parse_args()
    if a.write:
        ar,al=adapter.run();kr,kl=kernel.run();r=dict(stage='2BD',status='pass',adapter=ar,kernel=kr)
        (HERE/'reference-adapter-log.json.gz').write_bytes(al);(HERE/'reference-kernel-log.json.gz').write_bytes(kl);(HERE/'reference-replay.json').write_text(json.dumps(r,indent=2,sort_keys=True)+'\n')
    print(json.dumps(verify(),sort_keys=True))
