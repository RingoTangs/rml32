import gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
COMMIT='54eb8e62d96dcc2d129abe7936aed8a4ea9595e5'
def require(ok,message):
 if not ok:raise AssertionError(message)
def digest(raw):return hashlib.sha256(raw).hexdigest()
def encoded(v):return (json.dumps(v,sort_keys=True,indent=2,ensure_ascii=False)+'\n').encode()
def load(p):
 raw=p.read_bytes();return json.loads(gzip.decompress(raw)if p.suffix=='.gz'else raw)
