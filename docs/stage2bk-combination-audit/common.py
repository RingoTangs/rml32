import base64,gzip,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
def require(ok,message):
 if not ok:raise AssertionError(message)
def digest(raw):return hashlib.sha256(raw).hexdigest()
def encoded(v):return (json.dumps(v,sort_keys=True,indent=2,ensure_ascii=False)+'\n').encode()
def load(p):
 raw=p.read_bytes();return json.loads(gzip.decompress(raw)if p.suffix=='.gz'else raw)
def restore(destination):
 b=load(HERE/'baseline-inputs.json');raw=(HERE/'baseline-overlay.json.gz').read_bytes();require(digest(raw)==b['overlay_sha256'],'baseline overlay hash')
 overlay=load(HERE/'baseline-overlay.json.gz');require(set(overlay)==set(b['overlay_files']),'overlay inventory')
 for n,v in overlay.items():
  path=destination/n;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(base64.b64decode(v,validate=True))
 for n,h in b['files'].items():
  path=destination/n;raw=path.read_bytes()
  if digest(raw)!=h:
   frozen=(ROOT/n).read_bytes();require(digest(frozen)==h and frozen.replace(b'\r\n',b'\n')==raw,'baseline bytes '+n);path.write_bytes(frozen)
 require(all(digest((destination/n).read_bytes())==h for n,h in b['files'].items()),'restored snapshot')
