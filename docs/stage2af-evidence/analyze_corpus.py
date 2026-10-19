"""Read-only 05/17 structural audit; --write regenerates only this stage's statistics."""
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
INPUTS = ('docs/compatibility-evidence/programs.json.gz',
          'docs/compatibility-evidence/compatibility-summary.json',
          'docs/compatibility-evidence/efuns.csv')


def integer(value, low, high, name):
    if type(value) is not int or not low <= value <= high:
        raise ValueError(f'invalid {name}: {value!r}')
    return value


def registry(rows):
    result = {}
    for row in rows:
        slot = int(row['index'])
        if slot in result or not row['name']:
            raise ValueError('duplicate registry slot or empty name')
        result[slot] = row['name']
    if set(result) != set(range(559)):
        raise ValueError('registry must be exactly slots 0..558')
    return result


def global_namespace(slot, candidates):
    if slot < 0:
        return 'negative'
    if slot < 559:
        return 'efun_numeric_range'
    if len(candidates) != 1:
        return 'unresolved_global_unit'
    unit = candidates[0]
    if slot < unit['function_base']:
        return 'pre_module_numeric_range'
    if slot >= unit['max_functions']:
        return 'outside_global_capacity'
    return 'module_numeric_range'


def audit(programs, efuns):
    if not isinstance(programs, list):
        raise ValueError('corpus must be a list')
    sources = set()
    units = defaultdict(list)
    for p in programs:
        source = p['source']
        if not isinstance(source, str) or not source or source in sources:
            raise ValueError('nonempty unique source required')
        sources.add(source)
        if p['mode'] not in ('object', 'global') or p['efuns_count'] != 559:
            raise ValueError('expected Magic559 object/global image')
        if p['mode'] == 'global':
            integer(p['function_base'], 559, 32767, 'function_base')
            integer(p['max_functions'], p['function_base'], 32767, 'max_functions')
            units[source.split('/')[0]].append(p)
    totals = Counter(programs=len(programs), ordinary_functions=0, hidden_functions=0, instructions=0)
    counts = defaultdict(Counter)
    rows, examples, targets = [], {}, Counter()
    for p in programs:
        source = p['source']
        service = source.split('/')[0]
        candidates = units[service]
        definitions = {f['index']: f['name'] for f in p['functions']}
        totals['ordinary_functions'] += len(p['functions'])
        totals['hidden_functions'] += len(p['hidden'])
        segments = [('init', dict(name='__init__', code=p['init']))]
        segments += [(kind, f) for kind in ('functions', 'hidden') for f in p[kind]]
        for kind, function in segments:
            code = function['code']
            offset = integer(code['nodes_offset'], 0, 2**63-1, 'nodes_offset')
            segment = integer(code['offset'], 0, offset, 'segment offset')
            if not isinstance(code['nodes'], list):
                raise ValueError('nodes must be a list')
            nodes = code['nodes']
            totals['instructions'] += len(nodes)
            for pc, node in enumerate(nodes):
                if not isinstance(node, (list, tuple)) or len(node) != 3:
                    raise ValueError('node must contain par16, par8, opcode')
                a = integer(node[0], -32768, 32767, 'par16')
                b = integer(node[1], -128, 127, 'par8')
                op = integer(node[2], 1, 85, 'opcode')
                if op not in (5, 23):
                    continue
                label = f'{op:02x}'
                raw = b & 255
                row = dict(opcode=label, source=source, member_sha256=p['sha256'],
                           mode=p['mode'], service=service, segment_kind=kind,
                           function=function['name'], function_index=function.get('index'),
                           segment_offset=segment, nodes_offset=offset, pc=pc,
                           member_node_offset=offset+4*pc, par16=a,
                           par8_signed=b, par8_unsigned=raw)
                for key, value in [('opcodes', label), ('raw_operands', f'{label}:{a}:{b}'),
                                   ('contexts', f"{label}:{p['mode']}:{kind}"),
                                   ('services', f'{label}:{service}'), ('par8_unsigned', f'{label}:{raw}')]:
                    counts[key][str(value)] += 1
                if op == 23:
                    global_bit, selector_bit, bound_count = bool(raw & 128), bool(raw & 64), raw & 63
                    namespace = (global_namespace(a, candidates) if global_bit else
                                 'negative' if a < 0 else 'object_relative')
                    row.update(global_bit=global_bit, selector_bit6=selector_bit,
                               bound_count=bound_count, numeric_namespace=namespace,
                               global_unit=candidates[0]['source'] if len(candidates) == 1 else None,
                               efun_numeric_name=efuns.get(a) if global_bit and 0 <= a < 559 else None,
                               same_image_definition_candidate=definitions.get(a) if not global_bit else None)
                    flags = f'global={global_bit}:bit6={selector_bit}:bound={bound_count}'
                    counts['create_flags'][flags] += 1
                    counts['target_numeric_namespace'][namespace] += 1
                    targets[(global_bit, selector_bit, a, row['efun_numeric_name'])] += 1
                    example_key = (label, p['mode'], kind, flags, namespace)
                else:
                    counts['call_par16'][str(a)] += 1
                    counts['call_argc_signed'][str(b)] += 1
                    row['argc_status'] = 'negative' if b < 0 else 'nonnegative'
                    counts['call_argc_status'][row['argc_status']] += 1
                    # 05's target is a runtime stack value; par16 is not a target slot.
                    row['target_resolution'] = 'runtime_stack_value_not_statically_resolved'
                    example_key = (label, p['mode'], kind, str(a), str(b))
                rows.append(row)
                if example_key not in examples:
                    examples[example_key] = row | {'snippet': [dict(pc=i, node=nodes[i], member_node_offset=offset+4*i)
                        for i in range(max(0,pc-2),min(len(nodes),pc+3))]}
    summary = dict(schema_version=1, totals=dict(totals), occurrence_count=len(rows),
                   scope='Static operands and numeric namespaces only; no execution, reachability or dynamic call graph',
                   ambiguous_global_services=sorted(k for k,v in units.items() if len(v)>1),
                   histograms={k:dict(sorted(v.items())) for k,v in sorted(counts.items())})
    inventory = [dict(global_bit=g,selector_bit6=s,par16=a,efun_numeric_name=name,occurrences=count)
                 for (g,s,a,name),count in sorted(targets.items())]
    return summary, [examples[k] for k in sorted(examples)], rows, inventory


def encode(value):
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2)+'\n').encode()


def artifacts(summary, examples, rows, inventory):
    lines = ''.join(json.dumps(row,ensure_ascii=False,sort_keys=True,separators=(',',':'))+'\n' for row in rows).encode()
    return {'corpus-summary.json':encode(summary),'corpus-examples.json':encode(examples),
            'corpus-occurrences.jsonl.gz':gzip.compress(lines,mtime=0),'target-inventory.json':encode(inventory)}


def check_baseline(summary, baseline):
    expected = dict(programs=baseline['totals']['programs'],ordinary_functions=baseline['totals']['functions'],
                    hidden_functions=baseline['totals']['hidden_functions'],instructions=baseline['totals']['instructions'])
    if summary['totals'] != expected:
        raise ValueError('corpus totals disagree with independent baseline')
    for op in (5,23):
        if summary['histograms'].get('opcodes',{}).get(f'{op:02x}',0) != baseline['opcodes'].get(hex(op),0):
            raise ValueError('opcode total disagrees with independent baseline')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write',action='store_true')
    args=parser.parse_args()
    raw={p:(ROOT/p).read_bytes() for p in INPUTS}
    efuns=registry(csv.DictReader(io.StringIO(raw[INPUTS[2]].decode())))
    summary,examples,rows,inventory=audit(json.loads(gzip.decompress(raw[INPUTS[0]])),efuns)
    check_baseline(summary,json.loads(raw[INPUTS[1]]))
    summary['input_sha256']={p:hashlib.sha256(data).hexdigest() for p,data in raw.items()}
    for name,data in artifacts(summary,examples,rows,inventory).items():
        path=OUT/name
        if args.write:path.write_bytes(data)
        elif path.read_bytes()!=data:raise ValueError(f'{name} stale; regenerate explicitly with --write')
    print(json.dumps(dict(totals=summary['totals'],occurrences=len(rows),opcodes=summary['histograms']['opcodes'],
                         examples=len(examples),target_shapes=len(inventory))))


if __name__=='__main__':main()
