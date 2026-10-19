"""Static global namespace audit. Default verifies artifacts; --write regenerates."""
import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INPUT = 'docs/compatibility-evidence/programs.json.gz'
BASELINE = 'docs/compatibility-evidence/compatibility-summary.json'


def integer(value, low, high, name):
    if type(value) is not int or not low <= value <= high:
        raise ValueError(f'invalid {name}: {value!r}')
    return value


def contexts(program):
    yield 'init', dict(name='__init__', code=program['init'])
    for kind in ('functions', 'hidden'):
        for function in program[kind]:
            yield kind, function


def audit(programs):
    if not isinstance(programs, list):
        raise ValueError('program corpus must be a list')
    bindings = defaultdict(list)
    inventory = []
    sources = set()
    for p in programs:
        source = p['source']
        if not isinstance(source, str) or not source or source in sources:
            raise ValueError('source must be nonempty and unique')
        sources.add(source)
        if p['mode'] == 'global':
            for key in ('function_base', 'efuns_count', 'max_vars', 'max_functions'):
                integer(p[key], 0, 2**31-1, key)
            bindings[source.split('/')[0]].append(p)
            inventory.append({k:p[k] for k in ('source','sha256','mode','function_base',
                'efuns_count','max_vars','variables','max_functions','dlts')} | {
                'dlt_raw_attrib_index_counts':dict(sorted(Counter(f"{d['attrib']}:{d['index']}" for d in p['dlts']).items())),
                'function_slots':[dict(index=f['index'], name=f['name'], offset=f['offset'])
                                  for f in p['functions']]})
    counts = defaultdict(Counter)
    totals = Counter(programs=len(programs), ordinary_functions=0, hidden_functions=0, instructions=0)
    occurrences, examples = [], {}
    for p in programs:
        source = p['source']
        prefix = source.split('/')[0]
        candidates = bindings[prefix]
        bound = candidates[0] if len(candidates) == 1 else None
        parts = source.split('/')
        end = next((i for i,v in enumerate(parts) if v.endswith('.pak')), None)
        package = '/'.join(parts[:end+1]) if end is not None else '(no .pak component)'
        totals['ordinary_functions'] += len(p['functions'])
        totals['hidden_functions'] += len(p['hidden'])
        for kind, f in contexts(p):
            code = f['code']
            offset = integer(code['nodes_offset'], 0, 2**63-1, 'nodes_offset')
            segment = integer(code['offset'], 0, offset, 'segment offset')
            nodes = code['nodes']
            if not isinstance(nodes, list):
                raise ValueError('nodes must be a list')
            totals['instructions'] += len(nodes)
            for pc, node in enumerate(nodes):
                if not isinstance(node, (list,tuple)) or len(node) != 3:
                    raise ValueError('node must contain a, b, opcode')
                a = integer(node[0], -32768, 32767, 'signed i16 a')
                b = integer(node[1], -128, 127, 'signed i8 b')
                op = integer(node[2], 1, 85, 'opcode')
                if op not in (0x35,0x36,0x06,0x07):
                    continue
                label = f'0x{op:02x}'
                counts['all_opcode_counts'][label] += 1
                raw_b = b & 255
                counts['raw_operands'][f'{label}:{a}:{b}'] += 1
                counts['raw_modes'][f'{label}:{raw_b}'] += 1
                counts['indices'][f'{label}:{a}'] += 1
                counts['contexts'][f'{label}:{kind}'] += 1
                counts['packages'][f'{label}:{package}'] += 1
                if op == 0x07:
                    counts['call_flags'][f'global={bool(raw_b & 128)}:bit6={bool(raw_b & 64)}:argc={raw_b & 63}'] += 1
                if op == 0x06:
                    status = ('unbound' if bound is None else
                              'below_efuns_count' if 0 <= a < bound['efuns_count'] else
                              'between_efuns_and_function_base' if bound['efuns_count'] <= a < bound['function_base'] else
                              'at_or_above_function_base' if a >= bound['function_base'] else 'negative')
                    counts['efun_boundary'][status] += 1
                if op not in (0x35,0x36) and not (op == 7 and raw_b & 128):
                    continue
                row = dict(opcode=label, source=source, source_package=package,
                    member_sha256=p['sha256'], segment_kind=kind, function=f['name'],
                    function_index=f.get('index'), segment_offset=segment, nodes_offset=offset,
                    pc=pc, member_node_offset=offset+4*pc, raw_operands=[a,b], raw_b_unsigned=raw_b,
                    global_unit=bound['source'] if bound else None)
                if op in (0x35,0x36):
                    status = 'unbound' if bound is None else ('in_bounds' if 0 <= a < bound['max_vars'] else 'out_of_bounds')
                    row['variable_bounds'] = status
                    row['global_max_vars'] = bound['max_vars'] if bound else None
                    counts['variable_bounds'][f'{label}:{status}'] += 1
                    counts['variable_program_bounds'][f'{source}:{label}:{status}'] += 1
                else:
                    row.update(global_function_slot=a, argc=raw_b & 63, other_flag_bits=raw_b & 64)
                    status = ('unbound' if bound is None else 'in_function_interval' if
                              bound['function_base'] <= a < bound['max_functions'] else 'outside_function_interval')
                    row['function_interval'] = status
                    counts['global_call_interval'][status] += 1
                    namespace = ('unbound' if bound is None else 'negative' if a < 0 else
                        'efun_numeric_range' if a < bound['efuns_count'] else
                        'pre_module_global_range' if a < bound['function_base'] else
                        'module_function_range' if a < bound['max_functions'] else 'at_or_above_max_functions')
                    row['numeric_namespace'] = namespace
                    counts['global_call_numeric_namespace'][namespace] += 1
                    counts['global_call_slot_by_prefix'][f'{prefix}:{a}:{namespace}'] += 1
                occurrences.append(row)
                key = (label,raw_b,kind)
                if key not in examples:
                    examples[key] = row | {'snippet':[dict(pc=i,node=nodes[i],member_node_offset=offset+4*i)
                        for i in range(max(0,pc-2),min(len(nodes),pc+3))]}
    summary = dict(schema_version=1, totals=dict(totals), occurrence_count=len(occurrences),
        global_units=len(inventory), ambiguous_prefixes=sorted(k for k,v in bindings.items() if len(v)!=1),
        scope='Retained Main Magic559 images; static observations, no execution or reachability inference',
        histograms={k:dict(sorted(v.items())) for k,v in sorted(counts.items())})
    return summary, [examples[k] for k in sorted(examples)], occurrences, sorted(inventory,key=lambda x:x['source'])


def artifacts(summary, examples, occurrences, inventory):
    def encode(value):
        return (json.dumps(value, ensure_ascii=False, indent=2)+'\n').encode()
    return {'corpus-summary.json':encode(summary),'corpus-examples.json':encode(examples),
        'corpus-occurrences.json.gz':gzip.compress(encode(occurrences),mtime=0),
        'global-units.json':encode(inventory)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write',action='store_true')
    args = parser.parse_args()
    raw = (ROOT/INPUT).read_bytes()
    summary, examples, occurrences, inventory = audit(json.loads(gzip.decompress(raw)))
    baseline_raw = (ROOT/BASELINE).read_bytes()
    baseline = json.loads(baseline_raw)
    expected = dict(programs=baseline['totals']['programs'],ordinary_functions=baseline['totals']['functions'],
        hidden_functions=baseline['totals']['hidden_functions'],instructions=baseline['totals']['instructions'])
    if summary['totals'] != expected:
        raise ValueError('totals disagree with retained compatibility baseline')
    for op in ('0x35','0x36','0x06','0x07'):
        if summary['histograms']['all_opcode_counts'].get(op,0) != baseline['opcodes'].get(hex(int(op,16)),0):
            raise ValueError(f'{op} disagrees with baseline')
    summary['input_sha256'] = {INPUT:hashlib.sha256(raw).hexdigest(),BASELINE:hashlib.sha256(baseline_raw).hexdigest()}
    for name,data in artifacts(summary,examples,occurrences,inventory).items():
        path=OUT/name
        if args.write:
            path.write_bytes(data)
        elif path.read_bytes()!=data:
            raise ValueError(f'{name} stale; use explicit --write')
    print(json.dumps(dict(totals=summary['totals'],occurrences=len(occurrences),representatives=len(examples),
        counts=summary['histograms']['all_opcode_counts'], variable_bounds=summary['histograms']['variable_bounds'],
        global_call_interval=summary['histograms']['global_call_interval'])))


if __name__ == '__main__':
    main()
