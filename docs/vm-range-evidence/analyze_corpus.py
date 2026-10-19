"""Deterministic static opcode 0x45 audit. Default verifies; --write refreshes artifacts."""
import argparse
from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INPUT = 'docs/compatibility-evidence/programs.json.gz'
BASELINE = 'docs/compatibility-evidence/compatibility-summary.json'
OPCODE = 0x45


def integer(value, minimum, maximum, label):
    if type(value) is not int or not minimum <= value <= maximum:
        raise ValueError(f'invalid {label}: {value!r}')
    return value


def contexts(program):
    yield 'init', dict(name='__init__', code=program['init'])
    for kind in ('functions', 'hidden'):
        for function in program[kind]:
            yield kind, function


def audit(programs):
    if not isinstance(programs, list):
        raise ValueError('program corpus must be a list')
    totals = Counter(programs=len(programs), ordinary_functions=0, hidden_functions=0,
                     instructions=0)
    operands, kinds, packages = Counter(), Counter(), Counter()
    occurrences, examples = [], {}
    for program in programs:
        source = program['source']
        if not isinstance(source, str) or not source:
            raise ValueError('source must be nonempty text')
        package_parts = source.split('/')
        package_end = next((i for i, part in enumerate(package_parts)
                            if part.endswith('.pak')), None)
        package = ('/'.join(package_parts[:package_end + 1])
                   if package_end is not None else '(no .pak component)')
        totals['ordinary_functions'] += len(program['functions'])
        totals['hidden_functions'] += len(program['hidden'])
        for kind, function in contexts(program):
            code = function['code']
            offset = integer(code['nodes_offset'], 0, 2**63-1, 'nodes_offset')
            nodes = code['nodes']
            if not isinstance(nodes, list):
                raise ValueError('nodes must be a list')
            totals['instructions'] += len(nodes)
            for pc, node in enumerate(nodes):
                if not isinstance(node, (list, tuple)) or len(node) != 3:
                    raise ValueError('node must contain a, b, opcode')
                a = integer(node[0], -32768, 32767, 'signed i16 a')
                b = integer(node[1], -128, 127, 'signed i8 b')
                op = integer(node[2], 1, 85, 'opcode')
                if op != OPCODE:
                    continue
                operands[(a,b)] += 1
                kinds[kind] += 1
                packages[package] += 1
                row = dict(source=source, source_package=package,
                           member_sha256=program['sha256'], function=function['name'],
                           function_index=function.get('index'), segment_kind=kind,
                           segment_offset=code['offset'], node_count=len(nodes), pc=pc,
                           member_node_offset=offset + pc*4, raw_operands=[a,b])
                occurrences.append(row)
                key = (a,b,kind)
                # First encountered in retained corpus order, never a semantic inference.
                if key not in examples:
                    examples[key] = dict(row, snippet=[dict(pc=i,
                        member_node_offset=offset + i*4, node=nodes[i])
                        for i in range(max(0,pc-3), min(len(nodes),pc+4))])
    summary = dict(schema_version=1, opcode='0x45', name='vm_queryRange',
        scope='Retained Main Magic559 decoded program images; static scan only',
        offsets='Uncompressed member-relative bytes; nodes_offset + 4*pc; src/program.rs decodes each node as little-endian i16, signed i8, u8',
        interpretation='Raw operands only; runtime types, reachability, and mode semantics are not inferred',
        totals=dict(totals), occurrence_count=len(occurrences),
        by_segment_kind=dict(sorted(kinds.items())), by_source_package=dict(sorted(packages.items())),
        raw_operand_combinations=[dict(a=a,b=b,count=n) for (a,b),n in sorted(operands.items())])
    return summary, [examples[k] for k in sorted(examples)], occurrences


def artifacts(summary, examples, occurrences):
    def encode(value):
        return (json.dumps(value, ensure_ascii=False, indent=2)+'\n').encode()
    return {'corpus-summary.json':encode(summary), 'corpus-examples.json':encode(examples),
            'corpus-occurrences.json.gz':gzip.compress(encode(occurrences), mtime=0)}


def save_or_check(outputs, directory, write=False):
    for name, data in outputs.items():
        path = directory/name
        if write:
            path.write_bytes(data)
        elif path.read_bytes() != data:
            raise ValueError(f'{name} is stale; regenerate explicitly with --write')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    raw = (ROOT/INPUT).read_bytes()
    summary, examples, occurrences = audit(json.loads(gzip.decompress(raw)))
    baseline_raw = (ROOT/BASELINE).read_bytes()
    baseline = json.loads(baseline_raw)
    expected = dict(programs=baseline['totals']['programs'],
                    ordinary_functions=baseline['totals']['functions'],
                    hidden_functions=baseline['totals']['hidden_functions'],
                    instructions=baseline['totals']['instructions'])
    if summary['totals'] != expected or summary['occurrence_count'] != baseline['opcodes']['0x45']:
        raise ValueError('corpus totals disagree with retained compatibility baseline')
    summary['input_sha256'] = {INPUT:hashlib.sha256(raw).hexdigest(),
                              BASELINE:hashlib.sha256(baseline_raw).hexdigest()}
    save_or_check(artifacts(summary, examples, occurrences), OUT, args.write)
    print(json.dumps(dict(totals=summary['totals'], occurrences=len(occurrences),
                          representatives=len(examples), mode='write' if args.write else 'verify')))


if __name__ == '__main__':
    main()
