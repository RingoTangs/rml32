"""Read-only Bytes candidate opcode audit; --write updates only this stage's outputs."""
import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/compatibility-summary.json',
          'docs/evidence/actions.csv')
OPS = (1, 28, 42, 30, 32, 37, 39)
ORIGINS = {0x34: 'argument_count', 0x35: 'global_slot', 0x37: 'local_slot',
           0x38: 'dynamic_local_slot', 0x3b: 'short_numeric_constant',
           0x3c: 'numeric_constant_metadata', 0x3d: 'object_slot',
           0x3f: 'real_constant_metadata', 0x40: 'shared_string_pool_index',
           0x41: 'this_object', 0x42: 'undefined_constant'}
LIMITATION = ('Static adjacency only: preceding instructions are navigation hints, not '
              'proven operands, runtime tags, stack flow, execution counts or reachability.')


def integer(value, low, high, name):
    if type(value) is not int or not low <= value <= high:
        raise ValueError(f'invalid {name}: {value!r}')
    return value


def origin(node):
    return 'segment_start' if node is None else ORIGINS.get(node[2], 'other_instruction')


def audit(programs):
    if not isinstance(programs, list):
        raise ValueError('corpus must be a list')
    sources = set()
    totals = Counter(programs=len(programs), ordinary_functions=0, hidden_functions=0, instructions=0)
    counts = defaultdict(Counter)
    examples = {}
    for p in sorted(programs, key=lambda p: p['source']):
        source = p['source']
        if not isinstance(source, str) or not source or source in sources:
            raise ValueError('nonempty unique source required')
        sources.add(source)
        if p['mode'] not in ('object', 'global') or p['efuns_count'] != 559:
            raise ValueError('expected Magic559 object/global image')
        if len(p['sha256']) != 64 or any(c not in '0123456789abcdef' for c in p['sha256']):
            raise ValueError('invalid member hash')
        totals['ordinary_functions'] += len(p['functions'])
        totals['hidden_functions'] += len(p['hidden'])
        segments = [('init', dict(name='__init__', code=p['init']))]
        segments += [(kind, f) for kind in ('functions', 'hidden') for f in p[kind]]
        for kind, function in segments:
            code = function['code']
            offset = integer(code['nodes_offset'], 0, 2**63-1, 'nodes_offset')
            segment = integer(code['offset'], 0, offset, 'segment offset')
            nodes = code['nodes']
            if not isinstance(nodes, list):
                raise ValueError('nodes must be a list')
            totals['instructions'] += len(nodes)
            for node in nodes:
                if not isinstance(node, (list, tuple)) or len(node) != 3:
                    raise ValueError('node must contain par16, par8, opcode')
                integer(node[0], -32768, 32767, 'par16')
                integer(node[1], -128, 127, 'par8')
                integer(node[2], 1, 85, 'opcode')
            for pc, node in enumerate(nodes):
                if node[2] not in OPS:
                    continue
                label = f'{node[2]:02x}'
                prior = nodes[pc-1] if pc else None
                prior2 = nodes[pc-2] if pc > 1 else None
                context = f"{label}:{p['mode']}:{kind}"
                for key, value in [('opcodes', label), ('contexts', context),
                                   ('preceding_origin', f'{label}:{origin(prior)}'),
                                   ('preceding_pair', f'{label}:{origin(prior2)}:{origin(prior)}')]:
                    counts[key][value] += 1
                # One lexicographically first sample per opcode/context/prior category.
                sample_key = (context, origin(prior))
                row = dict(opcode=label, source=source, member_sha256=p['sha256'],
                           mode=p['mode'], segment_kind=kind, function=function['name'],
                           function_index=function.get('index'), segment_offset=segment,
                           nodes_offset=offset, pc=pc, member_node_offset=offset+4*pc,
                           node=node, preceding_origin=origin(prior),
                           snippet=[dict(pc=i, node=nodes[i], member_node_offset=offset+4*i)
                                    for i in range(max(0, pc-2), min(len(nodes), pc+2))])
                if sample_key not in examples:
                    examples[sample_key] = row
    summary = dict(schema_version=1, totals=dict(totals),
                   occurrence_count=sum(counts['opcodes'].values()), limitation=LIMITATION,
                   selected_opcodes=[f'{op:02x}' for op in OPS],
                   sample_policy='First source (sorted), segment order init/functions/hidden, function list order, PC; one per opcode/context/prior-origin.',
                   histograms={k: dict(sorted(v.items())) for k, v in sorted(counts.items())})
    return summary, [examples[k] for k in sorted(examples)]


def check_baseline(summary, baseline):
    expected = dict(programs=baseline['totals']['programs'], ordinary_functions=baseline['totals']['functions'],
                    hidden_functions=baseline['totals']['hidden_functions'], instructions=baseline['totals']['instructions'])
    if summary['totals'] != expected:
        raise ValueError('corpus totals disagree with independent baseline')
    for op in OPS:
        if summary['histograms'].get('opcodes', {}).get(f'{op:02x}', 0) != baseline['opcodes'].get(hex(op), 0):
            raise ValueError('opcode total disagrees with independent baseline')


def artifacts(summary, examples):
    return {name: (json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2)+'\n').encode()
            for name, value in [('corpus-summary.json', summary), ('corpus-examples.json', examples)]}


def verify_outputs(outputs, directory, write=False):
    for name, data in outputs.items():
        path = directory/name
        if write:
            path.write_bytes(data)
        elif not path.exists() or path.read_bytes() != data:
            raise ValueError(f'{name} stale; regenerate explicitly with --write')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    raw = {p: (ROOT/p).read_bytes() for p in INPUTS}
    summary, examples = audit(json.loads(gzip.decompress(raw[INPUTS[0]])))
    check_baseline(summary, json.loads(raw[INPUTS[1]]))
    summary['input_sha256'] = {p: hashlib.sha256(data).hexdigest() for p, data in raw.items()}
    verify_outputs(artifacts(summary, examples), OUT, args.write)
    print(json.dumps(dict(totals=summary['totals'], occurrences=summary['occurrence_count'],
                         opcodes=summary['histograms']['opcodes'], examples=len(examples))))


if __name__ == '__main__':
    main()
