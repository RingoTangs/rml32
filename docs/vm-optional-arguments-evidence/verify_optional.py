#!/usr/bin/env python3
"""Static optional-argument corpus report; default read-only verification."""
import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import struct
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/members.json.gz',
          'docs/compatibility-evidence/input-manifest.json',
          'docs/compatibility-evidence/inspect_packages.py',
          'docs/vm-reference-arguments-evidence/prototypes.json.gz')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def relation(argc, low, high):
    return 'below_min' if argc < low else 'above_max' if argc > high else 'within_bounds'


def layout(lengths, code_count, entry_pc=0):
    """Partition consecutive default instruction spans after the check opcode."""
    cursor = entry_pc + 1
    spans = []
    for length in lengths:
        end = cursor + length
        if length < 0 or end > code_count:
            return {'valid': False, 'spans': spans, 'end': end}
        spans.append([cursor, end])
        cursor = end
    return {'valid': True, 'spans': spans, 'end': cursor}


def entry_layout(f, raw_args):
    nodes = f['code']['nodes']
    lengths = [struct.unpack_from('<h', bytes.fromhex(a['raw_hex']), 10)[0] for a in raw_args]
    indices = [a['index'] for a in f['arguments']]
    if not nodes or nodes[0][2] != 15:
        return {'check_args_entry': False, 'default_lengths': lengths}
    total = sum(lengths)
    entry_total = nodes[0][0]
    argc_targets = [{'argc': argc,
                     'target': 1 + (sum(n for i, n in zip(indices, lengths) if i < argc)
                                    if argc < f['max_args'] else entry_total)}
                    for argc in range(f['min_args'], f['max_args'] + 1)]
    omitted = [{'argc': argc,
                'omitted_reference_indices': [a['index'] for a in f['arguments'] if a['index'] >= argc and a['flag'] & 1],
                'missing_zero_default_indices': [a['index'] for a, n in zip(f['arguments'], lengths) if a['index'] >= argc and n == 0],
                'missing_zero_default_nonmixed_indices': [a['index'] for a, n in zip(f['arguments'], lengths) if a['index'] >= argc and n == 0 and a['type'] != 255],
                'zero_default_hole_before_expression': any(a['index'] >= argc and n == 0 and any(b['index'] > a['index'] and m > 0 for b, m in zip(f['arguments'], lengths)) for a, n in zip(f['arguments'], lengths))}
               for argc in range(f['min_args'], f['max_args'])]
    spans = layout(lengths, len(nodes))
    after_default = 1 + total
    body_start = 1 + entry_total
    after_opcode = nodes[after_default][2] if spans['valid'] and 0 <= after_default < len(nodes) else None
    body_opcode = nodes[body_start][2] if 0 <= body_start < len(nodes) else None
    tail_matches = spans['valid'] and ((entry_total == total + 1 and after_opcode == 0x47 and body_opcode == 0x1b) if total else (entry_total == 0 and after_opcode == 0x1b))
    default_ops = Counter(op for a, b, op in nodes[1:after_default]) if spans['valid'] else Counter()
    return {'check_args_entry': True, 'omission_cases': omitted,
            'default_opcodes': {str(k): v for k, v in sorted(default_ops.items())},
            'default_uses_local_read_37': bool(default_ops[0x37]),
            'default_uses_local_reference_39': bool(default_ops[0x39]),
            'default_contains_call': any(default_ops[k] for k in range(5, 10)),
            'min_from_required_nonmixed': max((a['index'] + 1 for a, n in zip(f['arguments'], lengths) if n == 0 and a['type'] != 255), default=0), 'default_lengths': lengths,
            'entry_a_total': entry_total, 'length_sum': total,
            'entry_a_minus_length_sum': entry_total - total,
            'after_default_expressions_opcode': after_opcode,
            'body_start': body_start,
            'body_opcode': body_opcode,
            'default_tail_layout_matches': tail_matches,
            'entry_a_equals_length_sum': total == entry_total,
            'negative_length': any(n < 0 for n in lengths),
            'spans': spans,
            'argc_targets': argc_targets,
            'target_outside_code': any(t['target'] < 0 or t['target'] >= len(nodes) for t in argc_targets)}


def shape(f):
    return {'attrib': f['attrib'], 'attrib_low2': f['attrib'] & 3, 'variadic': bool(f['attrib'] & 1),
            'min_args': f['min_args'], 'max_args': f['max_args'],
            'argument_count': len(f['arguments']),
            'reference_arguments': sum(bool(a['flag'] & 1) for a in f['arguments']),
            'first_opcode': f['code']['nodes'][0][2] if f['code']['nodes'] else None}


def original_descriptors(programs):
    """Read original parameter records using already validated member offsets."""
    spec = importlib.util.spec_from_file_location('static_package_decoder', ROOT / INPUTS[3])
    decoder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(decoder)
    members = json.loads(gzip.decompress((ROOT / INPUTS[1]).read_bytes()))
    by_source = {m['package'] + '/' + m['path']: m for m in members if not m['directory']}
    manifest = {m['path']: m['sha256'] for m in json.loads((ROOT / INPUTS[2]).read_text())}
    package_cache, input_hashes, records = {}, {}, {}
    for p in programs:
        source = p['source']
        if source in by_source:
            m = by_source[source]
            name = m['package']
            if name not in package_cache:
                blob = (ROOT / name).read_bytes()
                assert sha(blob) == manifest[name], name
                package_cache[name] = blob
                input_hashes[name] = sha(blob)
            blob = package_cache[name]
            start = m['data_offset']
            data = (blob[start:start + m['original_bytes']] if m['attrib'] & 0x80000000
                    else decoder.lzss(blob[start:start + m['stored_bytes']], m['original_bytes']))
        else:
            data = (ROOT / source).read_bytes()
            input_hashes[source] = sha(data)
            assert sha(data) == manifest[source], source
        assert sha(data) == p['sha256'], source
        for f in p['functions'] + p['hidden']:
            r = decoder.Reader(data)
            r.pos = f['offset'] + 64
            assert r.string() == f['name']
            n = r.i16()
            assert n == len(f['arguments'])
            args = []
            for arg in f['arguments']:
                offset = r.pos
                raw = r.take(20)
                assert r.string() == arg['name']
                assert (raw[4], raw[5], raw[6], struct.unpack_from('<h', raw, 8)[0]) == (arg['flag'], arg['type'], arg['attrib'], arg['index'])
                args.append({'offset': offset, 'raw_hex': raw.hex()})
            records[(source, f['offset'])] = args
    return records, input_hashes


def compute():
    programs = json.loads(gzip.decompress((ROOT / INPUTS[0]).read_bytes()))
    ref = json.loads(gzip.decompress((ROOT / INPUTS[4]).read_bytes()))
    raw_arguments, original_hashes = original_descriptors(programs)
    totals, shapes, first, ranges = Counter(), Counter(), Counter(), Counter()
    totals['hidden_functions'] = 0
    services = defaultdict(Counter)
    layouts, mismatch_shapes, default_hist = Counter(), Counter(), Counter()
    default_shapes, entry_differences = Counter(), Counter()
    layout_samples = defaultdict(list)
    samples, nonfixed = defaultdict(list), []
    resolved_calls, target_shapes, resolved_call_code_shapes = Counter(), Counter(), Counter()
    function_lookup = {(p['source'], f.get('index')): f for p in programs for f in p['functions']}
    for p in programs:
        totals['programs'] += 1
        service = p['source'].split('/')[0]
        for kind in ('functions', 'hidden'):
            for f in p[kind]:
                totals['functions' if kind == 'functions' else 'hidden_functions'] += 1
                s = shape(f)
                category = 'fixed' if s['min_args'] == s['max_args'] else 'nonfixed'
                totals[category + '_functions'] += 1
                totals['arguments'] += len(f['arguments'])
                totals['min_args_high_bit_functions'] += f['min_args'] >= 128
                totals['max_args_high_bit_functions'] += f['max_args'] >= 128
                totals['highest_min_args'] = max(totals['highest_min_args'], f['min_args'])
                totals['highest_max_args'] = max(totals['highest_max_args'], f['max_args'])
                totals['reference_arguments'] += s['reference_arguments']
                totals['reference_functions'] += bool(s['reference_arguments'])
                totals[category + '_reference_functions'] += bool(s['reference_arguments'])
                totals['min_above_max_functions'] += s['min_args'] > s['max_args']
                totals['descriptor_count_ne_max_functions'] += s['argument_count'] != s['max_args']
                services[service][category + '_functions'] += 1
                key = tuple(s.values())
                shapes[key] += 1
                first[(category, s['first_opcode'])] += 1
                ranges[(s['attrib_low2'], s['min_args'], s['max_args'])] += 1
                analysis = entry_layout(f, raw_arguments[(p['source'], f['offset'])])
                for n in analysis['default_lengths']:
                    default_hist[n] += 1
                if s['argument_count'] != s['max_args']:
                    mismatch_shapes[(category, s['attrib_low2'], s['first_opcode'], s['min_args'], s['max_args'], s['argument_count'])] += 1
                if analysis['check_args_entry']:
                    for key in ('entry_a_equals_length_sum', 'negative_length', 'target_outside_code', 'default_tail_layout_matches'):
                        layouts[key + '=' + str(analysis[key])] += 1
                    layouts['spans_valid=' + str(analysis['spans']['valid'])] += 1
                    layouts['min_matches_required_nonmixed=' + str(f['min_args'] == analysis['min_from_required_nonmixed'])] += 1
                    for key in ('default_uses_local_read_37', 'default_uses_local_reference_39', 'default_contains_call'):
                        totals[key + '_functions'] += analysis[key]
                    totals['functions_with_omitted_reference_cases'] += any(c['omitted_reference_indices'] for c in analysis['omission_cases'])
                    totals['functions_with_missing_zero_default_nonmixed_cases'] += any(c['missing_zero_default_nonmixed_indices'] for c in analysis['omission_cases'])
                    totals['functions_with_zero_default_holes_before_expression'] += any(c['zero_default_hole_before_expression'] for c in analysis['omission_cases'])
                    has_defaults = any(analysis['default_lengths'])
                    totals['functions_with_nonzero_default_lengths'] += has_defaults
                    default_shapes[(category, s['attrib_low2'], bool(s['reference_arguments']), has_defaults)] += 1
                    entry_differences[(analysis['entry_a_minus_length_sum'], analysis['after_default_expressions_opcode'])] += 1
                record = {'source': p['source'], 'member_sha256': p['sha256'], 'segment': kind,
                          'index': f.get('index'), 'name': f['name'], 'function_offset': f['offset'],
                          'code_offset': f['code']['offset'], 'nodes_offset': f['code']['nodes_offset'],
                          'node_count': len(f['code']['nodes']), 'shape': s, 'entry_layout': analysis,
                          'arguments': f['arguments'], 'raw_arguments': raw_arguments[(p['source'], f['offset'])], 'entry_nodes': f['code']['nodes'][:16]}
                if analysis.get('entry_a_equals_length_sum') is False or analysis.get('negative_length') or analysis.get('target_outside_code'):
                    label = str((analysis.get('entry_a_equals_length_sum'), analysis.get('negative_length'), analysis.get('target_outside_code')))
                    if len(layout_samples[label]) < 5:
                        layout_samples[label].append(record)
                if category == 'nonfixed':
                    nonfixed.append(record)
                sample_key = f"{category}/attrib_low2={s['attrib_low2']}/reference={bool(s['reference_arguments'])}"
                if len(samples[sample_key]) < 3:
                    samples[sample_key].append(record)
    for target in ref['resolved_local_targets']:
        f = target['prototype']
        target_has_code = bool(function_lookup[(target['source'], f['index'])]['code']['nodes'])
        category = 'fixed' if f['min_args'] == f['max_args'] else 'nonfixed'
        for argc, count in target['argc'].items():
            raw_relation = relation(int(argc), f['min_args'], f['max_args'])
            resolved_calls[(category, f['attrib'] & 3, raw_relation)] += count
            resolved_call_code_shapes[(bool(f['attrib'] & 1), target_has_code, raw_relation)] += count
        target_shapes[(category, f['attrib'] & 3)] += 1
    for key in ('programs', 'functions', 'hidden_functions', 'arguments', 'reference_functions', 'reference_arguments'):
        assert totals[key] == ref['totals'][key], (key, totals[key], ref['totals'][key])
    return {'schema': 'stage2i.optional-argument-statistics.v1',
            'inputs_sha256': {s: sha((ROOT / s).read_bytes()) for s in INPUTS},
            'method_script_sha256': sha(Path(__file__).read_bytes()), 'original_inputs_sha256': original_hashes, 'totals': totals,
            'by_service': dict(services), 'entry_layout_checks': dict(layouts),
            'default_function_shapes': [dict(category=k[0], attrib_low2=k[1], has_reference=k[2], has_nonzero_default_lengths=k[3], count=v) for k, v in sorted(default_shapes.items())],
            'entry_total_differences': [dict(entry_a_minus_length_sum=k[0], after_default_expressions_opcode=k[1], count=v) for k, v in sorted(entry_differences.items(), key=lambda kv: str(kv[0]))],
            'default_length_histogram': {str(k): v for k, v in sorted(default_hist.items())},
            'descriptor_max_mismatch_shapes': [dict(category=k[0], attrib_low2=k[1], first_opcode=k[2], min_args=k[3], max_args=k[4], descriptor_count=k[5], count=v) for k, v in sorted(mismatch_shapes.items(), key=lambda kv: str(kv[0]))],
            'layout_diagnostic_samples': dict(layout_samples),
            'default_field_evidence': {'offset': 10, 'format': 'little-endian signed16', 'evidence_id': 'ev_d17aa0d7a395baf8b760f4dd773008e55d6a7fc1e8c6931a9657f73663982946', 'read_address': '0x080953c4', 'dwarf_evidence_id': 'ev_3a2145a4ec5bf063ceb5de533653f40d5ce70cd826ccb17d03b9892fc915df74', 'builder_evidence_id': 'ev_2c27ff7488e9819ae0c431411d6590807eaddf4a9037881bcf3d3e67b56ca294', 'resort_evidence_id': 'ev_999d6b2ce2163dabf3dfe97c69fc89bbf621502b7efc62e552dbaefa85dda75e'},
            'function_shapes': [dict(zip(shape(programs[0]['functions'][0]), k), count=v) for k, v in sorted(shapes.items(), key=lambda kv: str(kv[0]))],
            'min_max_attrib': [dict(attrib_low2=k[0], min_args=k[1], max_args=k[2], count=v) for k, v in sorted(ranges.items())],
            'entry_opcodes': [dict(category=k[0], opcode=k[1], count=v) for k, v in sorted(first.items(), key=lambda kv: str(kv[0]))],
            'resolved_call_ranges': [dict(category=k[0], attrib_low2=k[1], relation=k[2], count=v) for k, v in sorted(resolved_calls.items())],
            'resolved_call_code_shapes': [dict(variadic=k[0], target_has_code=k[1], relation=k[2], count=v) for k, v in sorted(resolved_call_code_shapes.items())],
            'resolved_target_shapes': [dict(category=k[0], attrib_low2=k[1], count=v) for k, v in sorted(target_shapes.items())],
            'samples': dict(samples), 'nonfixed_functions': nonfixed,
            'limitations': ['Counts describe serialized code, not execution or reachability.',
                            'Raw min/max range comparison is not a native call validity verdict.',
                            'Resolved calls reuse stage2G unique singleton/no-inheritance target resolution.',
                            'Default spans use native signed16 descriptor+0x0a and pre-incremented entry PC; bounds checks are report diagnostics, not execution.']}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    report = compute()
    encoded = (json.dumps(report, ensure_ascii=False, sort_keys=True, indent=2) + '\n').encode()
    packed = gzip.compress(encoded, compresslevel=9, mtime=0)
    summary = {k: v for k, v in report.items() if k not in ('nonfixed_functions', 'samples')}
    summary['full_report'] = {'path': 'optional.json.gz', 'sha256': sha(packed),
                              'compressed_bytes': len(packed), 'uncompressed_bytes': len(encoded)}
    paths = (HERE / 'optional.json.gz', HERE / 'summary.json')
    if args.write:
        paths[0].write_bytes(packed)
        paths[1].write_text(json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
    else:
        assert paths[0].read_bytes() == packed, 'deterministic report mismatch'
        assert json.loads(paths[1].read_text()) == summary, 'summary mismatch'
    print(json.dumps({'verified': not args.write, 'totals': report['totals']}, sort_keys=True))


if __name__ == '__main__':
    main()
