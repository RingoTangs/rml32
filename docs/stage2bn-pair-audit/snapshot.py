"""Reconstruct the uncommitted 2BM authority without changing the user's index."""
import gzip,subprocess
from common import ROOT,HERE,COMMIT,digest,load,require

def materialize(destination,logs):
    b=load(HERE/'baseline-inputs.json');patch=gzip.decompress((HERE/'baseline-overlay.patch.gz').read_bytes())
    require(b['base_commit']==COMMIT and digest(patch)==b['overlay_sha256'],'snapshot overlay identity')
    def command(argv,cwd,data=None):
        r=subprocess.run(argv,cwd=cwd,input=data,capture_output=True)
        logs.append(dict(argv=argv,returncode=r.returncode,stdout=r.stdout.decode(),stderr=r.stderr.decode(),**({'stdin_sha256':digest(data)} if data is not None else {})))
        require(r.returncode==0,r.stderr.decode())
    command(['git','clone','--shared','--dissociate','--no-checkout',str(ROOT),str(destination)],ROOT)
    command(['git','checkout','--detach',COMMIT],destination)
    command(['git','apply','--binary','-'],destination,patch)
    for n,h in b['files'].items():
        p=destination/n;raw=p.read_bytes()
        if digest(raw)!=h:
            candidates=[raw.replace(b'\r\n',b'\n'),raw.replace(b'\r\n',b'\n').replace(b'\n',b'\r\n')]
            matches=[v for v in candidates if digest(v)==h]
            require(len(matches)==1,'snapshot byte mismatch '+n);p.write_bytes(matches[0])
    require(all(digest((destination/n).read_bytes())==h for n,h in b['files'].items()),'snapshot files')
    return b
