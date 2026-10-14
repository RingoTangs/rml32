#!/usr/bin/env python3
"""Read-only inheritance metadata verification; --write creates stage 2K reports."""
import argparse
from collections import Counter, defaultdict
import csv
import gzip
import hashlib
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/inheritance.csv',
          'docs/compatibility-evidence/main-rea.json.gz',
          'docs/vm-optional-arguments-evidence/rea.json.gz',
          'docs/vm-inheritance-evidence/rea.json.gz')
EVIDENCE = 'ev_c41c22c1afd1192242c38bb59ffa5cc3402d157828307978e735721a4489e3d0'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def resolve_name(source, name):
    """Explicit offline catalogue policy, not native object lookup emulation."""
    if '.pak/' not in source or not isinstance(name, str):
        return None
    name = name.lstrip('/')
    if name.endswith('.c'):
        name = name[:-2] + '.o'
    return source.split('.pak/', 1)[0] + '.pak/' + name


def intervals(offsets, parent, child):
    capacities = [parent['constitution_count'], parent['max_functions'], parent['max_vars']]
    limits = [child['constitution_count'], child['max_functions'], child['max_vars']]
    result = []
    for axis, base, expected, count, capacity in zip(('constitution', 'function', 'variable'), offsets[:3], offsets[3:], capacities, limits):
        result.append(dict(axis=axis, base=base, count=count, end=base + count,
                           expected=expected, capacity=capacity,
                           count_match=expected == count,
                           nonnegative=base >= 0 and count >= 0,
                           fits=0 <= base <= base + count <= capacity,
                           signed16_end_fits=0 <= base + count <= 32767))
    return result


def analyze(programs):
    catalogue = defaultdict(list)
    for p in programs:
        catalogue[p['source']].append(p)
    unique = {s: v[0] for s, v in catalogue.items() if len(v) == 1}
    adjacency = {s: [] for s in unique}
    edges = []
    for source, p in sorted(unique.items()):
        for ordinal, item in enumerate(p['inherits']):
            target = resolve_name(source, item['name'])
            candidates = catalogue.get(target, [])
            status = 'resolved' if len(candidates) == 1 else 'missing' if not candidates else 'ambiguous'
            row = dict(child=source, ordinal=ordinal, parent_name=item['name'],
                       catalogue_target=target, status=status, offsets=item['offsets'])
            if status == 'resolved':
                row['intervals'] = intervals(item['offsets'], candidates[0], p)
                adjacency[source].append(target)
            edges.append(row)
    state, cache, cycles = {}, {}, []

    def walk(source, trail):
        if state.get(source) == 1:
            cycles.append(trail[trail.index(source):] + [source])
            return None
        if state.get(source) == 2:
            return cache[source]
        state[source] = 1
        ancestors = Counter()
        depths = []
        valid = True
        for parent in adjacency[source]:
            value = walk(parent, trail + [source])
            ancestors[parent] += 1
            if value is None:
                valid = False
            else:
                depths.append(value[0]); ancestors.update(value[1])
        value = (1 + max(depths, default=-1), ancestors) if valid else None
        state[source] = 2; cache[source] = value
        return value

    for source in sorted(adjacency):
        walk(source, [])
    overlaps = []
    by_child = defaultdict(list)
    for edge in edges:
        if edge['status'] == 'resolved':
            by_child[edge['child']].append(edge)
    for child, group in sorted(by_child.items()):
        for i, left in enumerate(group):
            for right in group[i + 1:]:
                for a, b in zip(left['intervals'], right['intervals']):
                    if max(a['base'], b['base']) < min(a['end'], b['end']):
                        overlaps.append(dict(child=child, axis=a['axis'], left=left['ordinal'], right=right['ordinal'], span=[max(a['base'], b['base']), min(a['end'], b['end'])]))
    # Reconstruct only constitution base relocation. Function override selection is separate.
    relocated, relocation_issues = {}, []

    def relocate(source):
        if source in relocated:
            return relocated[source]
        if cache.get(source) is None or any(e['status'] != 'resolved' for e in edges_by_child[source]):
            return None
        slots = {0: dict(program=source, base_function=0, base_variable=0)}
        for edge in edges_by_child[source]:
            parent_slots = relocate(edge['catalogue_target'])
            if parent_slots is None:
                return None
            cbase, fbase, vbase = edge['offsets'][:3]
            for index, old in sorted(parent_slots.items()):
                target = cbase + index
                item = dict(program=old['program'], base_function=fbase + old['base_function'], base_variable=vbase + old['base_variable'])
                if target in slots:
                    relocation_issues.append(dict(source=source, issue='constitution_slot_overlap', slot=target))
                if not (0 <= item['base_function'] <= 32767 and 0 <= item['base_variable'] <= 32767):
                    relocation_issues.append(dict(source=source, issue='relocated_base_signed16_overflow', slot=target))
                slots[target] = item
        count = unique[source]['constitution_count']
        if count is not None and set(slots) != set(range(count)):
            relocation_issues.append(dict(source=source, issue='constitution_slot_coverage', actual=sorted(slots), expected=count))
        relocated[source] = slots
        return slots

    edges_by_child = defaultdict(list)
    for edge in edges:
        edges_by_child[edge['child']].append(edge)
    for source in sorted(unique):
        relocate(source)
    summary = dict(programs=len(programs), edges=len(edges),
                   statuses=dict(sorted(Counter(e['status'] for e in edges).items())),
                   duplicate_source_keys=sum(len(v) > 1 for v in catalogue.values()),
                   direct_multiple_inheritance=sum(len(v) > 1 for v in adjacency.values()),
                   duplicate_direct_parent_programs=sum(len(v) != len(set(v)) for v in adjacency.values()),
                   cyclic_components_detected=len(cycles),
                   depth_histogram=dict(sorted(Counter(str(v[0]) for v in cache.values() if v is not None).items())),
                   multiple_paths_to_same_ancestor=sum(any(n > 1 for n in v[1].values()) for v in cache.values() if v is not None),
                   count_mismatch_edges=sum(any(not x['count_match'] for x in e.get('intervals', [])) for e in edges),
                   interval_out_of_capacity_edges=sum(any(not x['fits'] for x in e.get('intervals', [])) for e in edges),
                   signed16_end_overflow_edges=sum(any(not x['signed16_end_fits'] for x in e.get('intervals', [])) for e in edges),
                   overlapping_sibling_intervals=dict(sorted(Counter(x['axis'] for x in overlaps).items())),
                   relocated_programs=len(relocated),
                   relocation_issues=dict(sorted(Counter(x['issue'] for x in relocation_issues).items())))
    # Pre-alias tables: concrete bodies are shared; declarations are not resolved by name.
    tables, table_issues = {}, []

    def table(source):
        if source in tables:
            return tables[source]
        if source not in relocated:
            return None
        slots = {}
        for fun in unique[source]['functions']:
            index = fun['index']
            if index is None or not 0 <= index < unique[source]['max_functions']:
                table_issues.append(dict(source=source, issue='local_function_index_out_of_bounds', index=index))
                continue
            slots[index] = dict(body_program=source, body_function_index=index,
                                body_record_offset=fun['offset'], constitution=0,
                                has_code=bool(fun['code']['nodes']), name_display=fun['name'])
        for edge in edges_by_child[source]:
            parent_table = table(edge['catalogue_target'])
            if parent_table is None:
                return None
            cbase, fbase = edge['offsets'][:2]
            for index, item in sorted(parent_table.items()):
                target = fbase + index
                if target in slots:
                    table_issues.append(dict(source=source, issue='prealias_function_slot_collision', index=target))
                slots[target] = dict(item, constitution=cbase + item['constitution'])
        tables[source] = slots
        return slots

    for source in sorted(unique):
        table(source)
    init_orders, init_order_issues = {}, []

    def init_order(source):
        if source in init_orders:
            return init_orders[source]
        if source not in relocated:
            return None
        count = unique[source]['constitution_count']
        if count is None:
            return None
        order = [None] * count
        for edge in edges_by_child[source]:
            parent_order = init_order(edge['catalogue_target'])
            if parent_order is None:
                return None
            base = edge['offsets'][0]
            for j, value in enumerate(parent_order):
                dest = base + j - 1
                if not 0 <= dest < count:
                    init_order_issues.append(dict(source=source, issue='init_order_destination_out_of_bounds'))
                    continue
                order[dest] = base + value
        order[count - 1] = 0
        if sorted(x for x in order if x is not None) != list(range(count)):
            init_order_issues.append(dict(source=source, issue='init_order_not_permutation'))
        init_orders[source] = order
        return order

    for source in sorted(unique):
        init_order(source)
    summary['init_orders_reconstructed'] = len(init_orders)
    summary['init_order_issues'] = dict(sorted(Counter(x['issue'] for x in init_order_issues).items()))
    root_calls, root_call_counts = [], Counter()
    for source, program in sorted(unique.items()):
        entries = tables.get(source, {})
        for fun in program['functions']:
            for pc, (a, b, opcode) in enumerate(fun['code']['nodes']):
                if opcode != 7:
                    continue
                row = dict(root_object_program=source, caller_body_program=source,
                           caller_body_index=fun['index'], caller_constitution=0,
                           pc=pc, offset=fun['code']['nodes_offset'] + 4 * pc,
                           raw_a=a, raw_b=b, argc=b & 0x3f)
                if b & 0x80:
                    row['status'] = 'global_unknown'
                else:
                    index = ((a + 32768) % 65536) - 32768
                    row['target_slot'] = index
                    target = entries.get(index)
                    if target and target['has_code']:
                        row['status'] = 'concrete_resolved'
                        row['target'] = target
                    else:
                        row['status'] = 'declaration_alias_unknown' if target else 'unpopulated_unknown'
                root_call_counts[row['status']] += 1
                root_calls.append(row)
    summary['root_local_function_call07_nodes'] = len(root_calls)
    summary['root_local_function_call07_statuses'] = dict(sorted(root_call_counts.items()))
    summary['prealias_tables'] = len(tables)
    summary['prealias_populated_slots'] = sum(len(t) for t in tables.values())
    summary['prealias_concrete_slots'] = sum(x['has_code'] for t in tables.values() for x in t.values())
    summary['prealias_declaration_slots_unresolved'] = sum(not x['has_code'] for t in tables.values() for x in t.values())
    summary['prealias_unpopulated_slots'] = sum(unique[n]['max_functions'] - len(t) for n, t in tables.items())
    summary['prealias_issues'] = dict(sorted(Counter(x['issue'] for x in table_issues).items()))
    inherited_dlts = []
    for source, program in sorted(unique.items()):
        for dlt in program['dlts']:
            if dlt['attrib'] & 0xc0000:
                raw = bytes.fromhex(dlt['raw_hex'])
                inherited_dlts.append(dict(source=source, name=dlt['name'], offset=dlt['offset'], attrib=dlt['attrib'], index=dlt['index'], raw_8_16_hex=raw[8:16].hex(), raw_hex=dlt['raw_hex']))
    summary['inherited_dlt_records'] = len(inherited_dlts)
    summary['inherited_dlt_attrib_histogram'] = dict(sorted(Counter(str(d['attrib']) for d in inherited_dlts).items()))
    summary['inherited_dlt_raw_8_16_nonzero'] = sum(int(d['raw_8_16_hex'], 16) != 0 for d in inherited_dlts)
    summary['constitution_reserved_self_overlap_edges'] = sum(e['offsets'][0] < 1 and e['offsets'][3] > 0 for e in edges if e['status'] == 'resolved')
    nodes = [dict(source=s, depth=cache[s][0] if cache[s] else None,
                  direct_parent_count=len(adjacency[s]),
                  constitution_count=unique[s]['constitution_count'],
                  max_functions=unique[s]['max_functions'], max_vars=unique[s]['max_vars'],
                  relocated_constitutions=[dict(slot=i, **v) for i, v in sorted(relocated.get(s, {}).items())]) for s in sorted(unique)]
    return dict(summary=summary, nodes=nodes, edges=edges, cycles=cycles, inherited_dlts=inherited_dlts, prealias_tables={s: [dict(slot=i, **v) for i, v in sorted(t.items())] for s, t in sorted(tables.items())}, prealias_issues=table_issues, root_local_function_calls=root_calls, init_orders=init_orders, init_order_issues=init_order_issues,
                overlapping_intervals=overlaps, relocation_issues=relocation_issues)


def compact(graph):
    """Lossless columnar report; program strings occur once in the dictionary."""
    names = sorted(n['source'] for n in graph['nodes'])
    ids = {name: i for i, name in enumerate(names)}
    out = dict(graph)
    out['program_dictionary'] = names
    bodies = sorted({(x['body_program'], x['body_function_index'], x['body_record_offset'], int(x['has_code']), x['name_display']) for table in graph['prealias_tables'].values() for x in table})
    body_ids = {(x[0], x[1]): i for i, x in enumerate(bodies)}
    out['body_dictionary'] = [[ids[p], index, offset, code, name] for p, index, offset, code, name in bodies]
    out['columns'] = dict(body_dictionary=['program_id', 'function_index', 'record_offset', 'has_code', 'name_display'],
                          prealias_tables=['slot', 'body_id', 'constitution'],
                          root_local_function_calls=['root_program_id', 'caller_body_index', 'pc', 'offset', 'raw_a', 'raw_b', 'argc', 'status', 'target_slot', 'target_body_id', 'target_constitution'])
    out['prealias_tables'] = {str(ids[name]): [[x['slot'], body_ids[(x['body_program'], x['body_function_index'])], x['constitution']] for x in table] for name, table in graph['prealias_tables'].items()}
    calls = []
    for x in graph['root_local_function_calls']:
        t = x.get('target', {})
        calls.append([ids[x['root_object_program']], x['caller_body_index'], x['pc'], x['offset'], x['raw_a'], x['raw_b'], x['argc'], x['status'], x.get('target_slot'), body_ids[(t['body_program'], t['body_function_index'])] if t else None, t.get('constitution')])
    out['root_local_function_calls'] = calls
    out['call_invariants'] = dict(caller_body_program='same as root_program_id', caller_constitution=0, concrete_target_has_code=True)
    return out


def build():
    programs = json.loads(gzip.decompress((ROOT / INPUTS[0]).read_bytes()))
    graph = analyze(programs)
    rows = list(csv.DictReader((ROOT / INPUTS[1]).open()))
    assert len(rows) == len(graph['edges'])
    lookup = {(e['child'], e['parent_name']): e for e in graph['edges']}
    for row in rows:
        edge = lookup[(row['child'], row['parent'])]
        expected = 'match' if edge['status'] == 'resolved' and all(i['count_match'] for i in edge['intervals']) else 'count-mismatch' if edge['status'] == 'resolved' else edge['status']
        assert row['status'] == expected, row
    sources = {p: digest((ROOT / p).read_bytes()) for p in INPUTS}
    sources[str(Path(__file__).resolve().relative_to(ROOT))] = digest(Path(__file__).read_bytes())
    graph.update(schema='stage2k-inheritance-v1', sources=sources,
                 native_interval_evidence=EVIDENCE,
                 name_resolution='offline same package, strip leading slash, final .c -> .o; not native runtime namespace proof')
    packed = gzip.compress((json.dumps(compact(graph), ensure_ascii=False, sort_keys=True, separators=(',', ':')) + '\n').encode(), mtime=0)
    summary = dict(schema=graph['schema'], sources=sources, counts=graph['summary'],
                   graph_sha256=digest(packed), graph_bytes=len(packed),
                   inheritance_csv_rows_verified=len(rows),
                   limitations=['overlap is descriptive, not an invalidity verdict', 'prealias tables only: empty declarations remain unresolved; no name matching or bytecode execution', 'offline package catalogue is not native object namespace'])
    return graph, packed, summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    graph, packed, summary = build()
    if args.write:
        (HERE / 'graph.json.gz').write_bytes(packed)
        (HERE / 'summary.json').write_text(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True) + '\n')
    else:
        assert (HERE / 'graph.json.gz').read_bytes() == packed, 'graph mismatch'
        assert json.loads((HERE / 'summary.json').read_text()) == summary, 'summary mismatch'
    print(json.dumps(summary['counts'], ensure_ascii=False, sort_keys=True))


if __name__ == '__main__':
    main()
