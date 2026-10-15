#!/usr/bin/env python3
"""Read-only completion audit. --write regenerates only matrix/summary outputs."""
import argparse
from collections import Counter
import csv
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def supported(text):
    match = re.search(r'pub const SUPPORTED_OPCODES: &\[u8\] = &\[(.*?)\];', text, re.S)
    if not match:
        raise ValueError('SUPPORTED_OPCODES definition missing')
    values = [int(x, 16) for x in re.findall(r'0x[0-9a-fA-F]+', match[1])]
    if len(values) != len(set(values)):
        raise ValueError('duplicate supported opcode')
    return set(values)


def count_programs(programs):
    counts = Counter()
    functions = 0
    for p in programs:
        functions += len(p['functions'])
        for code in [p['init']] + [f['code'] for f in p['functions'] + p['hidden']]:
            if code:
                for node in code['nodes']:
                    if len(node) != 3 or not isinstance(node[2], int):
                        raise ValueError('malformed opcode node')
                    counts[node[2]] += 1
    return counts, functions


def test_names(text):
    return set(re.findall(r'#\[test\]\s*fn\s+(\w+)\s*\(', text))


def validate_review(rows, union, implemented, tests, root=ROOT):
    keys = [int(r['opcode'], 16) for r in rows]
    if len(keys) != len(set(keys)) or set(keys) != union:
        raise ValueError('review rows must exactly partition opcode union')
    for r in rows:
        op = int(r['opcode'], 16)
        expected = 'bounded' if op in implemented else 'unimplemented'
        if r['semantic_status'] != expected:
            raise ValueError('semantic status differs from source support')
        if not r['excluded_cases'] or not r['dependencies'] or not r['exit_criteria']:
            raise ValueError('missing boundaries/dependencies/exit criteria')
        if op in implemented and (not r['implemented_cases'] or not r['related_tests']):
            raise ValueError('bounded row needs cases and related tests')
        if op not in implemented and (r['implemented_cases'] or r['related_tests']):
            raise ValueError('unimplemented row cannot claim implementation or semantic tests')
        for name in r['related_tests']:
            if name not in tests:
                raise ValueError('unknown Rust test: '+name)
        if not r['evidence']:
            raise ValueError('missing evidence')
        for evidence in r['evidence']:
            path = root / evidence
            if Path(evidence).is_absolute() or '..' in Path(evidence).parts or not path.is_file():
                raise ValueError('invalid evidence path: '+evidence)


def render(rows, summary):
    head = '# Stage 2W opcode audit\n\n'
    head += ('Bounded means implemented selected paths, never complete native compatibility. '
             'Related tests identify supporting scenarios, not measured branch coverage. '
             'All counts include initialization and function nodes; no business code was executed.\n\n')
    head += '| Opcode / native handler | Nodes | Status / group | Implemented cases | Excluded cases | Related synthetic tests | Evidence | Dependencies / exit criterion |\n|---|---:|---|---|---|---|---|---|\n'
    for r in rows:
        def join(items):
            return '<br>'.join(str(x).replace('|', '\\|') for x in items)
        head += '| '+ ' | '.join([
            '`'+r['opcode']+'` '+r['name']+' @ '+r['native_address'],str(r['corpus_nodes']),r['semantic_status']+' / '+r['group'],join(r['implemented_cases']) or '—',join(r['excluded_cases']),join(r['related_tests']) or '—',join('['+Path(p).name+'](../../'+p+')' for p in r['evidence']),join(r['dependencies'])+'<br>'+r['exit_criteria']])+' |\n'
    return head


def build():
    source_paths = ['src/lib.rs','src/program.rs','Cargo.toml','Cargo.lock','src/prepared.rs','src/vm.rs','src/numeric.rs','src/value.rs','src/container.rs','src/object.rs','src/inheritance.rs','src/cell.rs','src/vm_error.rs','tests/vm_core.rs','docs/evidence/actions.csv','docs/compatibility-evidence/programs.json.gz','docs/compatibility-evidence/compatibility-summary.json','docs/stage2w-evidence/opcode-review.json','docs/stage2w-evidence/verify_completion.py','docs/stage2w-evidence/test_verify_completion.py','docs/stage2w-evidence/method.md']
    source_paths = [p for p in source_paths if (ROOT/p).exists()]
    implemented = supported((ROOT/'src/prepared.rs').read_text())
    with gzip.open(ROOT/'docs/compatibility-evidence/programs.json.gz','rt') as f:
        programs = json.load(f)
    counts, functions = count_programs(programs)
    baseline = json.loads((ROOT/'docs/compatibility-evidence/compatibility-summary.json').read_text())
    if dict(counts) != {int(k,16):v for k,v in baseline['opcodes'].items()}:
        raise ValueError('recomputed node counts differ from historical baseline')
    if len(programs) != baseline['totals']['programs'] or functions != baseline['totals']['functions'] or sum(counts.values()) != baseline['totals']['instructions']:
        raise ValueError('corpus totals differ')
    review = json.loads((HERE/'opcode-review.json').read_text())['rows']
    validate_review(review,set(counts)|implemented,implemented,test_names((ROOT/'tests/vm_core.rs').read_text()))
    actions = {int(r['opcode'],16):r for r in csv.DictReader((ROOT/'docs/evidence/actions.csv').open())}
    for row in review:
        a=actions[int(row['opcode'],16)]
        if row['name'] != a['name'] or row['native_address'] != a['entry']:
            raise ValueError('native action reference differs')
        source_paths.extend(row['evidence'])
    rows = [dict(r,corpus_nodes=counts[int(r['opcode'],16)],in_corpus=int(r['opcode'],16) in counts,in_supported_set=int(r['opcode'],16) in implemented,semantic_branch_coverage='not measured; no full compatibility claim') for r in review]
    summary = dict(schema_version=1,programs=len(programs),functions=functions,instructions=sum(counts.values()),corpus_opcode_kinds=len(counts),supported_opcode_kinds=len(implemented),union_opcode_kinds=len(set(counts)|implemented),intersection_opcode_kinds=len(set(counts)&implemented),missing=[f'{x:02x}' for x in sorted(set(counts)-implemented)],supported_not_in_corpus=[f'{x:02x}' for x in sorted(implemented-set(counts))],all_supported_semantic_status='bounded',full_compatibility_rows=0,branch_coverage='not measured',corpus_execution=False,missing_by_group=dict(sorted(Counter(r['group'] for r in rows if not r['in_supported_set']).items())),source_sha256={p:digest(ROOT/p) for p in sorted(set(source_paths))})
    if (summary['corpus_opcode_kinds'],summary['supported_opcode_kinds'],summary['union_opcode_kinds'],summary['intersection_opcode_kinds'],len(summary['missing']),summary['supported_not_in_corpus']) != (73,68,75,66,7,['4d','4e']):
        raise ValueError('stage 2W frozen count contract changed; review intentionally')
    return {'matrix.json': json.dumps({'schema_version':1,'rows':rows},ensure_ascii=False,indent=2)+'\n','summary.json':json.dumps(summary,ensure_ascii=False,indent=2)+'\n','matrix.md':render(rows,summary)}, summary


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write',action='store_true')
    args=parser.parse_args()
    outputs, summary=build()
    for name,data in outputs.items():
        path=HERE/name
        if args.write:
            path.write_text(data)
        elif not path.exists() or path.read_text() != data:
            raise ValueError('stale generated output: '+name+'; inspect source changes before --write')
    print(json.dumps({k:v for k,v in summary.items() if k!='source_sha256'},ensure_ascii=False))


if __name__=='__main__':
    main()
