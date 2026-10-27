import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
COMMIT='5827a6387fc2fbaa142a428133316c446713c230'
def digest(raw):return hashlib.sha256(raw).hexdigest()
def load(path):return json.loads(path.read_bytes())
def require(ok,message):
    if not ok:raise AssertionError(message)
def encoded(value):return (json.dumps(value,indent=2,sort_keys=True)+'\n').encode()
