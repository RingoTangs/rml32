import gzip,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
COMMIT='0c4fbf40007d0bc2f877a269ba6e1ca22ef6fcc0'
def require(ok,message):
    if not ok:raise AssertionError(message)
def digest(raw):return hashlib.sha256(raw).hexdigest()
def load(path):
    raw=path.read_bytes();return json.loads(gzip.decompress(raw)if path.suffix=='.gz'else raw)
def encoded(value):return (json.dumps(value,indent=2,sort_keys=True,ensure_ascii=False)+'\n').encode()
