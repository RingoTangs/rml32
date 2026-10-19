"""Read-only 06 audit; --write explicitly regenerates this stage's artifacts."""
import argparse
from collections import Counter, defaultdict
import csv
import gzip
import hashlib
import io
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INPUTS = (
    'docs/compatibility-evidence/programs.json.gz',
    'docs/compatibility-evidence/compatibility-summary.json',
    'docs/compatibility-evidence/efuns.csv',
    'docs/compatibility-evidence/efun-usage.csv',
)


def integer(value, low, high, name):
    if type(value) is not int or not low <= value <= high:
        raise ValueError(f'invalid {name}: {value!r}')
    return value


def registry(rows):
    result = {}
    for row in rows:
        index = int(row['index'])
        if index in result or not row['name']:
            raise ValueError('duplicate registry slot or empty name')
        result[index] = dict(row)
    if set(result) != set(range(559)):
        raise ValueError('registry must contain exactly slots 0..558')
    return result


def audit(programs, slots):
    if not isinstance(programs, list):
        raise ValueError('corpus must be a list')
    totals = Counter(programs=len(programs), ordinary_functions=0,
                     hidden_functions=0, instructions=0)
    counts = defaultdict(Counter)
    sources, examples, occurrences = set(), {}, []
    for program in programs:
        source = program['source']
        if not isinstance(source, str) or not source or source in sources:
            raise ValueError('source must be nonempty and unique')
        sources.add(source)
        if program['mode'] not in ('object', 'global'):
            raise ValueError('unknown program mode')
        efuns = integer(program['efuns_count'], 0, 32767, 'efuns_count')
        if efuns != 559:
            raise ValueError('corpus is not Magic559')
        totals['ordinary_functions'] += len(program['functions'])
        totals['hidden_functions'] += len(program['hidden'])
        segments = [('init', dict(name='__init__', code=program['init']))]
        segments += [(kind, f) for kind in ('functions', 'hidden') for f in program[kind]]
        for kind, function in segments:
            code = function['code']
            offset = integer(code['nodes_offset'], 0, 2**63 - 1, 'nodes_offset')
            segment = integer(code['offset'], 0, offset, 'segment offset')
            nodes = code['nodes']
            if not isinstance(nodes, list):
                raise ValueError('nodes must be a list')
            totals['instructions'] += len(nodes)
            for pc, node in enumerate(nodes):
                if not isinstance(node, (list, tuple)) or len(node) != 3:
                    raise ValueError('node must contain par16, par8, opcode')
                slot = integer(node[0], -32768, 32767, 'par16')
                argc = integer(node[1], -128, 127, 'par8')
                opcode = integer(node[2], 1, 85, 'opcode')
                if opcode != 6:
                    continue
                entry = slots.get(slot)
                status = ('negative_slot' if slot < 0 else
                          'outside_efun_namespace' if slot >= efuns else
                          'unknown_registry_slot' if entry is None else 'registered')
                row = dict(source=source, member_sha256=program['sha256'],
                           mode=program['mode'], segment_kind=kind,
                           function=function['name'], function_index=function.get('index'),
                           segment_offset=segment, nodes_offset=offset, pc=pc,
                           member_node_offset=offset + 4 * pc, slot=slot,
                           par8_signed=argc, par8_unsigned=argc & 255,
                           slot_status=status, argc_status='negative' if argc < 0 else 'nonnegative',
                           efun=entry['name'] if entry else None)
                occurrences.append(row)
                for key, value in (
                    ('slots', slot), ('argc_signed', argc), ('par8_unsigned', argc & 255),
                    ('slot_status', status), ('argc_status', row['argc_status']),
                    ('services', source.split('/')[0]), ('segments', kind),
                    ('program_modes', program['mode']),
                    ('slot_argc', f'{slot}:{argc}'),
                    ('service_segment', f"{source.split('/')[0]}:{kind}"),
                ):
                    counts[key][str(value)] += 1
                example_key = f"{program['mode']}:{kind}:{argc}:{status}"
                if example_key not in examples:
                    examples[example_key] = row | {'snippet': [
                        dict(pc=i, node=nodes[i], member_node_offset=offset + 4 * i)
                        for i in range(max(0, pc - 2), min(len(nodes), pc + 3))]}
    used = Counter(r['slot'] for r in occurrences)
    inventory = [slots[i] | {'call_sites': used[i]} for i in sorted(slots)]
    summary = dict(schema_version=1, totals=dict(totals), occurrence_count=len(occurrences),
                   registry_slots=len(slots), used_slots=len(used),
                   scope='Static retained Magic559 corpus; no execution or reachability inference',
                   histograms={k: dict(sorted(v.items())) for k, v in sorted(counts.items())})
    return summary, [examples[k] for k in sorted(examples)], occurrences, inventory


def encode(value):
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + '\n').encode()


def artifacts(summary, examples, occurrences, inventory):
    lines = ''.join(json.dumps(r, ensure_ascii=False, sort_keys=True,
                              separators=(',', ':')) + '\n' for r in occurrences).encode()
    return {'corpus-summary.json': encode(summary), 'corpus-examples.json': encode(examples),
            'corpus-occurrences.jsonl.gz': gzip.compress(lines, mtime=0),
            'efun-inventory.json': encode(inventory)}


def check_baseline(summary, occurrences, baseline, usage):
    expected = dict(programs=baseline['totals']['programs'],
                    ordinary_functions=baseline['totals']['functions'],
                    hidden_functions=baseline['totals']['hidden_functions'],
                    instructions=baseline['totals']['instructions'])
    if summary['totals'] != expected or summary['occurrence_count'] != baseline['opcodes']['0x6']:
        raise ValueError('corpus totals or 06 count disagree with independent baseline')
    actual = Counter(row['efun'] for row in occurrences)
    usage = list(usage)
    expected_usage = {row['name']: int(row['static_call_sites']) for row in usage}
    if len(expected_usage) != len(usage):
        raise ValueError('duplicate efun usage name')
    if dict(actual) != expected_usage:
        raise ValueError('efun usage disagrees with independent baseline')
    for row in usage:
        calls = [r for r in occurrences if r['efun'] == row['name']]
        if 'programs' in row and len({r['source'] for r in calls}) != int(row['programs']):
            raise ValueError('efun program count disagrees with baseline')
        services = Counter(r['source'].split('/')[0] for r in calls)
        if any(services[s] != int(row[s]) for s in ('aaa', 'ccs', 'csa', 'dba', 'gs') if s in row):
            raise ValueError('efun service count disagrees with baseline')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    raw = {path: (ROOT / path).read_bytes() for path in INPUTS}
    slots = registry(csv.DictReader(io.StringIO(raw[INPUTS[2]].decode())))
    summary, examples, occurrences, inventory = audit(
        json.loads(gzip.decompress(raw[INPUTS[0]])), slots)
    check_baseline(summary, occurrences, json.loads(raw[INPUTS[1]]),
                   csv.DictReader(io.StringIO(raw[INPUTS[3]].decode())))
    summary['input_sha256'] = {p: hashlib.sha256(data).hexdigest() for p, data in raw.items()}
    for name, data in artifacts(summary, examples, occurrences, inventory).items():
        path = OUT / name
        if args.write:
            path.write_bytes(data)
        elif path.read_bytes() != data:
            raise ValueError(f'{name} stale; regenerate explicitly with --write')
    print(json.dumps({k: summary[k] for k in ('totals', 'occurrence_count', 'registry_slots', 'used_slots')}))


if __name__ == '__main__':
    main()
