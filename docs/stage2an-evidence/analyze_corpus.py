"""Independent read-only Global/K/c audit. --write updates only 2AN statistics."""
import argparse
from collections import Counter
import csv
import gzip
import hashlib
import io
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/efuns.csv',
          'docs/compatibility-evidence/compatibility-summary.json',
          'docs/stage2af-evidence/corpus-occurrences.jsonl.gz',
          'docs/stage2af-evidence/corpus-summary.json')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def integer(value, low, high):
    require(type(value) is int and low <= value <= high, 'invalid native operand')
    return value


def scan(programs, efuns):
    require(isinstance(programs, list), 'program list required')
    require(set(efuns) == set(range(559)) and all(efuns.values()), '559 efun slots required')
    totals = Counter(programs=0, ordinary_functions=0, hidden_functions=0, instructions=0,
                     calls_05=0, creates_17=0, global_k=0, global_c=0)
    rows, shapes, seen, examples = [], Counter(), set(), {}
    for program in programs:
        source = program['source']
        require(isinstance(source, str) and source and source not in seen, 'unique source required')
        seen.add(source)
        require(program['mode'] in ('object', 'global') and program['efuns_count'] == 559,
                'Magic559 mode required')
        totals['programs'] += 1
        totals['ordinary_functions'] += len(program['functions'])
        totals['hidden_functions'] += len(program['hidden'])
        segments = [('init', {'name': '__init__', 'code': program['init']})]
        segments += [(kind, f) for kind in ('functions', 'hidden') for f in program[kind]]
        for kind, function in segments:
            code = function['code']
            offset = integer(code['nodes_offset'], 0, 2**63-1)
            require(type(code['nodes']) is list, 'node list required')
            for pc, node in enumerate(code['nodes']):
                require(isinstance(node, (list, tuple)) and len(node) == 3, 'three operands required')
                slot = integer(node[0], -32768, 32767)
                flags = integer(node[1], -128, 127) & 255
                opcode = integer(node[2], 1, 85)
                totals['instructions'] += 1
                if opcode == 5:
                    totals['calls_05'] += 1
                if opcode != 0x17:
                    continue
                totals['creates_17'] += 1
                if not flags & 128:
                    continue
                selector = 'K' if flags & 64 else 'c'
                totals['global_' + selector.lower()] += 1
                row = dict(source=source, member_sha256=program['sha256'], mode=program['mode'],
                           service=source.split('/')[0], segment_kind=kind,
                           function=function['name'], function_index=function.get('index'),
                           pc=pc, nodes_offset=offset, member_node_offset=offset+4*pc,
                           slot=slot, flags=flags, selector=selector, bound_count=flags & 63,
                           efun_numeric_name=efuns.get(slot) if 0 <= slot < 559 else None)
                rows.append(row)
                shape = (selector, slot, row['mode'], kind, row['bound_count'])
                shapes[shape] += 1
                examples.setdefault(shape, row | {'snippet': [dict(pc=i, node=code['nodes'][i])
                    for i in range(max(0, pc-2), min(len(code['nodes']), pc+3))]})
    inventory = [dict(selector=s, slot=n, mode=m, segment_kind=k, bound_count=b, occurrences=count)
                 for (s, n, m, k, b), count in sorted(shapes.items())]
    return dict(totals= dict(totals), occurrence_count=len(rows),
                limitation='Static locations and numeric names only; no inferred runtime targets or reachability'), rows, inventory, list(examples.values())


def cross_check(summary, rows, baseline, prior_summary, prior_rows):
    totals = summary['totals']
    for key, previous in [('programs', 'programs'), ('ordinary_functions', 'functions'),
                          ('hidden_functions', 'hidden_functions'), ('instructions', 'instructions')]:
        require(totals[key] == baseline['totals'][previous], 'independent corpus totals differ')
    require(totals['calls_05'] == baseline['opcodes']['0x5'] == prior_summary['histograms']['opcodes']['05'], '05 total differs')
    require(totals['creates_17'] == baseline['opcodes']['0x17'] == prior_summary['histograms']['opcodes']['17'], '17 total differs')
    def identity(row):
        return tuple(row[k] for k in ('source', 'segment_kind', 'function_index', 'pc', 'member_node_offset'))
    require(len({identity(r) for r in rows}) == len(rows), 'duplicate occurrence')
    old = {identity(r): (r['par16'], r['par8_unsigned'], r['member_sha256'])
           for r in prior_rows if r['opcode'] == '17' and r['global_bit']}
    new = {identity(r): (r['slot'], r['flags'], r['member_sha256']) for r in rows}
    require(new == old, '2AF Global location/operand mismatch')
    require(totals['global_k'] == 4902 and totals['global_c'] == 110, 'Global selector baseline differs')


def artifacts(root=ROOT):
    raw = {name: (root / name).read_bytes() for name in INPUTS}
    registry_rows = list(csv.DictReader(io.StringIO(raw[INPUTS[1]].decode())))
    efuns = {int(r['index']): r['name'] for r in registry_rows}
    require(len(registry_rows) == len(efuns), 'duplicate registry slot')
    summary, rows, inventory, examples = scan(json.loads(gzip.decompress(raw[INPUTS[0]])), efuns)
    cross_check(summary, rows, json.loads(raw[INPUTS[2]]), json.loads(raw[INPUTS[4]]),
                [json.loads(line) for line in gzip.decompress(raw[INPUTS[3]]).splitlines()])
    summary['input_sha256'] = {name: hashlib.sha256(data).hexdigest() for name, data in raw.items()}
    def encode(value):
        return (json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2)+'\n').encode()
    positions = b''.join((json.dumps(r, ensure_ascii=False, sort_keys=True, separators=(',', ':'))+'\n').encode() for r in rows)
    return {'corpus-summary.json': encode(summary), 'target-inventory.json': encode(inventory),
            'corpus-examples.json': encode(examples),
            'corpus-occurrences.jsonl.gz': gzip.compress(positions, mtime=0)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    for name, data in artifacts().items():
        if args.write:
            (HERE / name).write_bytes(data)
        else:
            require((HERE / name).read_bytes() == data, name + ' stale; use explicit --write')
    print(json.dumps(json.loads((HERE / 'corpus-summary.json').read_bytes())['totals'], sort_keys=True))


if __name__ == '__main__':
    main()
