"""Readonly, deterministic control-flow corpus audit; --write refreshes outputs."""
import argparse
from collections import Counter
import csv
import gzip
import hashlib
import json
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
OPS = (0x0a, 0x0b, 0x0c, 0x12, 0x23, 0x50)
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/compatibility-summary.json', 'docs/evidence/actions.csv')


def metadata_pair(a, b, metadata):
    # Deliberately preserve both signed extensions, matching native dispatch.
    pos = (b << 16) | a
    if pos < 0 or pos + 8 > len(metadata):
        raise ValueError('metadata pair outside segment')
    return pos, list(struct.unpack_from('<ii', metadata, pos))


def contexts(program):
    yield 'init', dict(name='__init__', code=program['init'])
    for kind in ('functions', 'hidden'):
        for function in program[kind]:
            yield kind, function


def audit(programs, actions):
    hist = {op: Counter() for op in OPS}
    decoded = {op: Counter() for op in OPS}
    kinds = {op: Counter() for op in OPS}
    errors = []
    examples = {}
    totals = Counter(programs=len(programs))
    ranges = {op: [] for op in OPS}
    checks = Counter()
    loop_prefixes = Counter()
    for p in programs:
        totals['ordinary_functions'] += len(p['functions'])
        totals['hidden_functions'] += len(p['hidden'])
        for kind, f in contexts(p):
            code = f['code']
            nodes = code['nodes']
            totals['instructions'] += len(nodes)
            metadata = bytes.fromhex(code['metadata_hex'])
            for pc, (a, b, op) in enumerate(nodes):
                if op not in hist:
                    continue
                hist[op][(a, b)] += 1
                kinds[op][kind] += 1
                operands = [a, b]
                pairpos = None
                if op in (0x0a, 0x0b):
                    try:
                        pairpos, operands = metadata_pair(a, b, metadata)
                        ranges[op].append(pairpos)
                    except ValueError:
                        errors.append(dict(source=p['source'], function=f['name'], pc=pc,
                                           kind='metadata', raw=[a, b]))
                branch_target = None
                if op != 0x23 and (op not in (0x0a, 0x0b) or pairpos is not None):
                    branch_target = pc + 1 + operands[0]
                    checks['branch_targets_checked'] += 1
                    if not 0 <= branch_target < len(nodes):
                        errors.append(dict(source=p['source'], function=f['name'], pc=pc,
                                           kind='branch-target', target=branch_target))
                if op == 0x0b and pairpos is not None:
                    index = operands[1]
                    checks['string_indexes_checked'] += 1
                    if not 0 <= index < len(p['strings']) or p['strings'][index] is None:
                        errors.append(dict(source=p['source'], function=f['name'], pc=pc,
                                           kind='string-index', index=index))
                if op == 0x23:
                    loop_prefixes[tuple(nodes[pc-1]) if pc else ()] += 1
                if op in (0x12, 0x23, 0x50):
                    slot = a if op == 0x23 else b
                    checks['loop_slots_checked'] += 1
                    # Native slots: args >=0; locals -local_count..-1.
                    if not -f.get('local_count',0) <= slot < f.get('max_args',0):
                        errors.append(dict(source=p['source'], function=f['name'], pc=pc,
                                           kind='loop-slot', slot=slot))
                decoded[op][tuple(operands)] += 1
                candidate = dict(source=p['source'], member_sha256=p['sha256'],
                                 function=f['name'], function_index=f.get('index'),
                                 segment_kind=kind, segment_offset=code['offset'],
                                 node_count=len(nodes), pc=pc,
                                 member_node_offset=code['nodes_offset'] + 4 * pc,
                                 raw_operands=[a, b], decoded_operands=operands,
                                 taken_branch_target=branch_target,
                                 metadata_pair_offset=pairpos,
                                 member_metadata_offset=(code['metadata_offset'] + pairpos
                                                         if pairpos is not None else None),
                                 metadata_pair_hex=(metadata[pairpos:pairpos+8].hex()
                                                    if pairpos is not None else None),
                                 string_pool_size=len(p['strings']),
                                 snippet=[dict(pc=i, member_offset=code['nodes_offset'] + 4*i,
                                               node=nodes[i])
                                          for i in range(max(0,pc-2),min(len(nodes),pc+3))])
                if op not in examples or (len(nodes),p['source'],pc) < (
                        examples[op]['node_count'], examples[op]['source'], examples[op]['pc']):
                    examples[op] = candidate
    summary = dict(schema_version=1, scope='Main Magic559 decoded program images only; no execution',
                   totals=dict(totals), offsets='Uncompressed member-relative bytes, never ELF addresses',
                   opcodes={f'0x{op:02x}': dict(name=actions[op]['name'],
                       count=sum(hist[op].values()), by_segment_kind=dict(sorted(kinds[op].items())),
                       raw_operand_combinations={f'{a},{b}':n for (a,b),n in sorted(hist[op].items())},
                       decoded_operand_combination_count=len(decoded[op]),
                       decoded_operand_top20=[dict(operands=list(k), count=v)
                           for k,v in sorted(decoded[op].items(), key=lambda item: (-item[1],item[0]))[:20]],
                       metadata_offset_min=min(ranges[op]) if ranges[op] else None,
                       metadata_offset_max=max(ranges[op]) if ranges[op] else None)
                       for op in OPS}, native_layout_checks=dict(checks),
                   immediate_pre_init_nodes=[dict(node=list(k), count=v) for k,v in sorted(loop_prefixes.items())],
                   errors=errors)
    return summary, {f'0x{op:02x}': examples[op] for op in OPS if op in examples}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    programs = json.loads(gzip.decompress((ROOT / INPUTS[0]).read_bytes()))
    actions = {int(r['opcode'],16):r for r in csv.DictReader((ROOT / INPUTS[2]).open())}
    summary, examples = audit(programs, actions)
    summary['input_sha256'] = {p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in INPUTS}
    baseline = json.loads((ROOT / INPUTS[1]).read_text())
    assert len(programs) == 7159
    assert summary['totals']['ordinary_functions'] == baseline['totals']['functions']
    assert summary['totals']['hidden_functions'] == 0
    assert summary['totals']['instructions'] == baseline['totals']['instructions']
    assert not summary['errors'], summary['errors'][:3]
    for op, row in summary['opcodes'].items():
        assert row['count'] == baseline['opcodes'][hex(int(op,16))], op
    for name, value in [('corpus-summary.json',summary), ('examples.json',examples)]:
        rendered = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
        path = OUT / name
        if args.write:
            path.write_text(rendered)
        else:
            assert path.read_text() == rendered, f'{name} is stale'
    print(json.dumps(dict(programs=len(programs), counts={o:r['count'] for o,r in summary['opcodes'].items()},
                          errors=len(summary['errors']), mode='write' if args.write else 'verify')))


if __name__ == '__main__':
    main()
