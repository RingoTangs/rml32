"""Replay the existing 17 drivers on current Rust without updating old evidence."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
OLD = ROOT / 'docs/stage2bn-pair-audit'
sys.path.insert(0, str(OLD))
import common
# Only a new producer's failure evidence may be written here; old receipts stay frozen.
common.HERE = HERE
import reference_replay
import pair_diff
import public_replay

def digest(raw):
    return hashlib.sha256(raw).hexdigest()

def sources():
    return {str(p.relative_to(ROOT)): digest(p.read_bytes()) for p in sorted((ROOT/'src').glob('*.rs'))}

def semantic(v):
    if isinstance(v, dict):
        return {k: semantic(x) for k,x in v.items() if k not in
                ('source_sha256','log_sha256','replay_log_sha256')}
    if isinstance(v, list):
        return [semantic(x) for x in v]
    return v

def encode(v):
    return (json.dumps(v, sort_keys=True, indent=2)+'\n').encode()

def run():
    before = sources()
    print('Replaying the previous 15 drivers', file=sys.stderr, flush=True)
    reference, log = reference_replay.run()
    expected = common.load(OLD/'reference-replay.json')
    assert semantic(reference) == semantic(expected), 'older replay semantic results changed'
    (HERE/'reference-current-log.json.gz').write_bytes(log)
    print('Replaying all six groups of the pair driver in debug/release', file=sys.stderr, flush=True)
    pair, log, obs = pair_diff.run()
    expected = common.load(OLD/'pair-diff.json')
    assert pair['reports'] == expected['reports'], 'pair projections changed'
    assert pair['observation_sha256'] == expected['observation_sha256'], 'pair raw observations changed'
    (HERE/'pair-current-log.json.gz').write_bytes(log)
    print('Replaying the public pair driver in debug/release', file=sys.stderr, flush=True)
    public, log, obs = public_replay.run()
    expected = common.load(OLD/'public-replay.json')
    assert public['summary'] == expected['summary'], 'public projections changed'
    assert public['observations_sha256'] == expected['observations_sha256'], 'public raw observations changed'
    (HERE/'public-current-log.json.gz').write_bytes(log)
    assert sources() == before, 'Rust source changed during replay'
    result = dict(stage='2BO', status='pass', drivers=17, source_sha256=before,
                  reference=reference, pair=pair, public=public,
                  log_sha256={n:digest((HERE/n).read_bytes()) for n in
                              ('reference-current-log.json.gz','pair-current-log.json.gz','public-current-log.json.gz')})
    (HERE/'current-replay.json').write_bytes(encode(result))
    print('All 17 existing drivers reproduced their frozen semantic results', file=sys.stderr, flush=True)

def verify():
    result = common.load(HERE/'current-replay.json')
    assert result['stage']=='2BO' and result['status']=='pass' and result['drivers']==17
    assert result['source_sha256']==sources()
    for n,h in result['log_sha256'].items():
        assert digest((HERE/n).read_bytes())==h
    assert semantic(result['reference'])==semantic(common.load(OLD/'reference-replay.json'))
    expected=common.load(OLD/'pair-diff.json')
    assert result['pair']['reports']==expected['reports']
    assert result['pair']['observation_sha256']==expected['observation_sha256']
    expected=common.load(OLD/'public-replay.json')
    assert result['public']['summary']==expected['summary']
    assert result['public']['observations_sha256']==expected['observations_sha256']
    return dict(stage='2BO',status='pass',drivers=17,source_sha256=result['source_sha256'])

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--write',action='store_true')
    if parser.parse_args().write:run()
    print(json.dumps(verify(),sort_keys=True))
