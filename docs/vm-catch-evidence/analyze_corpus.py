#!/usr/bin/env python3
"""Readonly lexical catch analysis of retained, decoded program images."""
import argparse
from collections import Counter
import csv
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/compatibility-summary.json', 'docs/evidence/actions.csv')
OPS = (0x0e, 0x1a, 0x2c)
PATTERNS = {'contains_end': {0x1a}, 'contains_return': {0x48, 0x49},
            'contains_loop': {0x12, 0x23, 0x50}, 'contains_cross_object_call': {0x08},
            'contains_any_call': {0x05, 0x06, 0x07, 0x08, 0x09},
            'contains_nested_start': {0x2c}}


def contexts(program):
    yield 'init', {'name': '__init__', 'code': program['init']}
    for kind in ('functions', 'hidden'):
        for function in program[kind]:
            yield kind, function


def valid_node(node):
    return (isinstance(node, (list, tuple)) and len(node) == 3
            and all(isinstance(v, int) and not isinstance(v, bool) for v in node)
            and -32768 <= node[0] <= 32767 and -128 <= node[1] <= 127
            and 0 <= node[2] <= 255)


def location(program, kind, function, pc):
    code = function['code']
    return dict(source=program['source'], member_sha256=program['sha256'],
                function=function['name'], function_index=function.get('index'),
                segment_kind=kind, segment_offset=code['offset'],
                pc=pc, member_node_offset=code['nodes_offset'] + 4 * pc)


def audit(programs, actions=None):
    hist = {op: Counter() for op in OPS}
    kinds = {op: Counter() for op in OPS}
    patterns, checks, totals = Counter(), Counter(), Counter(programs=len(programs))
    errors, examples = [], {}
    target_ops = Counter()

    def example(label, p, kind, f, pc, extra=None):
        nodes = f['code']['nodes']
        candidate = dict(location(p, kind, f, pc), node_count=len(nodes),
                         node=nodes[pc], **(extra or {}))
        candidate['snippet'] = [dict(pc=i, member_offset=f['code']['nodes_offset'] + 4*i,
                                     node=nodes[i])
                                for i in range(max(0, pc-2), min(len(nodes), pc+5))]
        if 'candidate_target' in candidate:
            target = candidate['candidate_target']
            candidate['target_snippet'] = [
                dict(pc=i, member_offset=f['code']['nodes_offset'] + 4*i, node=nodes[i])
                for i in range(max(0, target-2), min(len(nodes), target+3))]
            if label in PATTERNS:
                witness = next(i for i in range(pc+1, target)
                               if nodes[i][2] in PATTERNS[label])
                candidate['witness_pc'] = witness
                candidate['witness_snippet'] = [
                    dict(pc=i, member_offset=f['code']['nodes_offset'] + 4*i, node=nodes[i])
                    for i in range(max(0, witness-1), min(len(nodes), witness+2))]
        key = (len(nodes), p['source'], f['name'], pc)
        if label not in examples or key < examples[label][0]:
            examples[label] = (key, candidate)

    for p in programs:
        totals['ordinary_functions'] += len(p['functions'])
        totals['hidden_functions'] += len(p['hidden'])
        for kind, f in contexts(p):
            nodes = f['code']['nodes']
            totals['instructions'] += len(nodes)
            malformed = [pc for pc, node in enumerate(nodes) if not valid_node(node)]
            for pc in malformed:
                errors.append(dict(location(p, kind, f, pc), kind='malformed-node', node=nodes[pc]))
            if malformed:
                checks['segments_skipped_malformed'] += 1
                continue
            intervals = []
            for pc, (a, b, op) in enumerate(nodes):
                if op not in OPS:
                    continue
                hist[op][a, b] += 1
                kinds[op][kind] += 1
                example(f'opcode_0x{op:02x}', p, kind, f, pc)
                if op != 0x2c:
                    continue
                target = pc + 1 + a
                checks['candidate_targets_checked'] += 1
                if not 0 <= target < len(nodes):
                    errors.append(dict(location(p, kind, f, pc), kind='candidate-target-bounds',
                                       candidate_target=target))
                    continue
                target_ops[nodes[target][2]] += 1
                if target > pc:
                    intervals.append((pc, target))
                    body = nodes[pc+1:target]
                    ops = {n[2] for n in body}
                    extra = dict(candidate_target=target,
                                 candidate_target_member_offset=f['code']['nodes_offset']+4*target,
                                 candidate_target_node=nodes[target],
                                 lexical_region=[pc+1, target],
                                 lexical_region_end_exclusive=True)
                    for label, wanted in PATTERNS.items():
                        if ops & wanted:
                            patterns[label] += 1
                            example(label, p, kind, f, pc, extra)
                    ends = [i for i in range(pc+1, target) if nodes[i][2] == 0x1a]
                    if ends and any(nodes[i][2] in (0x48, 0x49)
                                    for i in range(pc+1, ends[0])):
                        patterns['return_before_first_lexical_end'] += 1
                        example('return_before_first_lexical_end', p, kind, f, pc, extra)
                    if (target >= 2 and nodes[target-2][2] == 0x1a
                            and nodes[target-1][2] == 0x1f):
                        patterns['end_then_jump_immediately_before_target'] += 1
                        example('end_then_jump_immediately_before_target', p, kind, f, pc, extra)
                else:
                    checks['nonforward_candidate_targets'] += 1
            if intervals:
                totals['segments_with_catch_start'] += 1
                events = Counter()
                for start, end in intervals:
                    events[start] += 1
                    events[end] -= 1
                depth = maximum = 0
                for pc in sorted(events):
                    depth += events[pc]
                    maximum = max(maximum, depth)
                patterns['max_candidate_interval_overlap'] = max(
                    patterns['max_candidate_interval_overlap'], maximum)
    summary = dict(schema_version=1, scope='Main Magic559 decoded corpus; readonly lexical analysis; no execution',
                   target_formula_status='native-confirmed pc + 1 + signed a; lexical region pairing remains candidate',
                   target_formula_evidence=dict(function='vm_newThreadCatch', address='0x080aaf84',
                       evidence_id='ev_446aa83f597882387e0856507a84ad5ec466a2e91a4e234c7e9c000ca98eec9a',
                       basis='current.ip + offset; regularProcessor advances ip before signed-par16 dispatch'),
                   offsets='Uncompressed member-relative byte offsets, not ELF addresses or compressed-package offsets',
                   totals=dict(totals), checks=dict(checks), lexical_candidate_patterns=dict(patterns),
                   candidate_target_opcodes={f'0x{k:02x}': v for k, v in sorted(target_ops.items())},
                   opcodes={f'0x{op:02x}': dict(name=(actions or {}).get(op, {}).get('name'),
                       count=sum(hist[op].values()), by_segment_kind=dict(sorted(kinds[op].items())),
                       raw_operand_combinations={f'{a},{b}': n for (a,b), n in sorted(hist[op].items())})
                       for op in OPS}, errors=errors)
    return summary, {k: v[1] for k, v in sorted(examples.items())}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    programs = json.loads(gzip.decompress((ROOT / INPUTS[0]).read_bytes()))
    with (ROOT / INPUTS[2]).open() as stream:
        actions = {int(r['opcode'], 16): r for r in csv.DictReader(stream)}
    summary, examples = audit(programs, actions)
    summary['input_sha256'] = {p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in INPUTS}
    baseline = json.loads((ROOT / INPUTS[1]).read_text())
    assert len(programs) == 7159
    assert summary['totals']['ordinary_functions'] == baseline['totals']['functions']
    assert summary['totals']['instructions'] == baseline['totals']['instructions']
    assert not summary['errors'], summary['errors'][:3]
    for op, row in summary['opcodes'].items():
        assert row['count'] == baseline['opcodes'][hex(int(op, 16))]
    for name, data in [('corpus-summary.json', summary), ('corpus-examples.json', examples)]:
        rendered = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
        if args.write:
            (OUT/name).write_text(rendered)
        else:
            assert (OUT/name).read_text() == rendered, f'{name} is stale'
    print(json.dumps(dict(counts={k:v['count'] for k,v in summary['opcodes'].items()},
                          patterns=summary['lexical_candidate_patterns'], errors=len(summary['errors']))))


if __name__ == '__main__':
    main()
