"""Read-only lifecycle metadata inventory. Never executes reference artifacts or bytecode."""
import argparse
from collections import Counter
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
          'docs/vm-inheritance-evidence/graph.json.gz')


def digest(data):
    return hashlib.sha256(data).hexdigest()


def raw_string(reader):
    size = reader.i32()
    return reader.take(size) if size > 0 else b''


def raw_function(raw, fun, decoder):
    """Recover name/parameter bytes from original member at independently recorded offset."""
    r = decoder.Reader(raw)
    r.pos = fun['offset']
    header = r.take(64)
    name = raw_string(r)
    groups = {}
    for group in ('arguments', 'locals'):
        n = r.i16()
        if n != len(fun[group]):
            raise ValueError('descriptor count mismatch')
        values = []
        for _ in range(n):
            item = r.take(20)
            values.append(dict(name_hex=raw_string(r).hex(), raw_hex=item.hex(),
                               flag=item[4], type=item[5], attrib=item[6],
                               index=struct.unpack_from('<h', item, 8)[0],
                               default_action_count=struct.unpack_from('<h', item, 10)[0]))
        groups[group] = values
    if r.pos != fun['code']['offset']:
        raise ValueError('code offset mismatch')
    if decoder.text(name) != fun['name']:
        raise ValueError('display name mismatch')
    return dict(name_hex=name.hex(), c_name_hex=name.split(b'\0', 1)[0].hex(),
                header_hex=header.hex(), return_attrib=header[12], **groups)


def histogram(values):
    return dict(sorted(Counter(str(v) for v in values).items()))


def inventory(programs, graph, raw_functions):
    """Pure metadata analysis; lifecycle matching is exact C-name bytes, no alias dispatch."""
    modes = Counter()
    init_ops = Counter()
    init_rows = []
    lifecycle = []
    orders = graph['init_orders']
    issues = []
    for p in sorted(programs, key=lambda x: x['source']):
        source = p['source']
        modes[p['mode']] += 1
        nodes = p['init']['nodes']
        init_ops.update(f'{n[2]:02x}' for n in nodes)
        order = orders.get(source)
        if p['mode'] == 'object':
            count = p['constitution_count']
            if count < 1 or not order or sorted(order) != list(range(count)) or order[-1] != 0:
                issues.append(source)
        elif order is not None:
            issues.append(source)
        init_rows.append(dict(source=source, mode=p['mode'], nodes=len(nodes),
                              nodes_offset=p['init']['nodes_offset'],
                              metadata_bytes=len(bytes.fromhex(p['init']['metadata_hex'])),
                              opcode_counts=dict(sorted(Counter(f'{n[2]:02x}' for n in nodes).items())),
                              finish_only=[n[2] for n in nodes] == [0x2e, 0x48, 0x1d],
                              init_order=order))
        for group in ('functions', 'hidden'):
            for f in p[group]:
                original = raw_functions[(source, f['offset'])]
                name = bytes.fromhex(original['c_name_hex'])
                if name not in (b'create', b'destruct'):
                    continue
                lifecycle.append(dict(source=source, mode=p['mode'], group=group,
                                      name=name.decode('ascii'), index=f.get('index'), offset=f['offset'],
                                      has_code=bool(f['code']['nodes']), attrib=f['attrib'],
                                      return_type=f['return_type'], min_args=f['min_args'], max_args=f['max_args'],
                                      code_nodes=len(f['code']['nodes']), **original))
    stats = {}
    for name in ('create', 'destruct'):
        rows = [r for r in lifecycle if r['name'] == name]
        stats[name] = dict(records=len(rows), programs=len({r['source'] for r in rows}),
                           bodies=sum(r['has_code'] for r in rows), declarations=sum(not r['has_code'] for r in rows),
                           mode=histogram(r['mode'] for r in rows), group=histogram(r['group'] for r in rows),
                           attrib=histogram(r['attrib'] for r in rows),
                           return_type=histogram(r['return_type'] for r in rows),
                           return_attrib=histogram(r['return_attrib'] for r in rows),
                           argument_counts=histogram(len(r['arguments']) for r in rows),
                           min_max=histogram(f"{r['min_args']}/{r['max_args']}" for r in rows),
                           argument_flags=histogram(a['flag'] for r in rows for a in r['arguments']),
                           argument_types=histogram(a['type'] for r in rows for a in r['arguments']),
                           argument_attribs=histogram(a['attrib'] for r in rows for a in r['arguments']),
                           nonzero_default_descriptors=sum(a['default_action_count'] != 0 for r in rows for a in r['arguments']))
    return dict(init_segments=init_rows, lifecycle_records=lifecycle), dict(
        programs=len(programs), modes=dict(sorted(modes.items())),
        functions_raw_checked=len(raw_functions), init_nodes=sum(r['nodes'] for r in init_rows),
        init_opcode_counts=dict(sorted(init_ops.items())),
        init_minimal_2e_48_1d=sum(r['finish_only'] for r in init_rows),
        init_other=sum(not r['finish_only'] for r in init_rows),
        init_metadata_nonempty=sum(r['metadata_bytes'] > 0 for r in init_rows),
        object_init_orders_verified=sum(p['mode'] == 'object' for p in programs),
        init_order_issues=issues, lifecycle=stats)


def build():
    spec = importlib.util.spec_from_file_location('lifecycle_decoder', ROOT / INPUTS[3])
    decoder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(decoder)
    programs = json.loads(gzip.decompress((ROOT / INPUTS[0]).read_bytes()))
    members = json.loads(gzip.decompress((ROOT / INPUTS[1]).read_bytes()))
    manifest = json.loads((ROOT / INPUTS[2]).read_text())
    originals = [r for r in manifest if r['path'] != 'AGENTS.md']
    for record in originals:
        if digest((ROOT / record['path']).read_bytes()) != record['sha256']:
            raise ValueError('original hash mismatch: ' + record['path'])
    lookup = {m['package'] + '/' + m['path']: m for m in members}
    packages, raw_functions = {}, {}
    for p in programs:
        m = lookup.get(p['source'])
        if m:
            path = m['package']
            if path not in packages:
                packages[path] = (ROOT / path).read_bytes()
            start = m['data_offset']
            raw = (packages[path][start:start + m['original_bytes']] if m['attrib'] & 0x80000000
                   else decoder.lzss(packages[path][start:start + m['stored_bytes']], m['original_bytes']))
        else:
            raw = (ROOT / p['source']).read_bytes()
        if digest(raw) != p['sha256']:
            raise ValueError('member hash mismatch: ' + p['source'])
        for group in ('functions', 'hidden'):
            for f in p[group]:
                raw_functions[p['source'], f['offset']] = raw_function(raw, f, decoder)
    graph = json.loads(gzip.decompress((ROOT / INPUTS[4]).read_bytes()))
    details, counts = inventory(programs, graph, raw_functions)
    sources = {p: digest((ROOT / p).read_bytes()) for p in INPUTS}
    sources[str(Path(__file__).relative_to(ROOT))] = digest(Path(__file__).read_bytes())
    packed = gzip.compress((json.dumps(details, sort_keys=True, separators=(',', ':')) + '\n').encode(), mtime=0)
    summary = dict(schema='stage2m-lifecycle-v1', sources=sources, counts=counts,
                   originals_verified=len(originals), details_sha256=digest(packed), details_bytes=len(packed),
                   limitations=['Counts are original local records, not final aliased or inherited dispatch slots.',
                                'Names use original bytes before first NUL; display decoding is not used for matching.',
                                'Init order is reused offline stage2K reconstruction; this does not prove native execution order.',
                                'Raw serialized attributes precede native postload lifecycle flag changes.',
                                'No original executable, startup script, init segment or business bytecode is executed.'])
    return packed, summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true', help='regenerate only this directory details and summary')
    args = parser.parse_args()
    packed, summary = build()
    rendered = (json.dumps(summary, indent=2, sort_keys=True) + '\n').encode()
    for path, data in ((HERE / 'details.json.gz', packed), (HERE / 'summary.json', rendered)):
        if args.write:
            path.write_bytes(data)
        elif path.read_bytes() != data:
            raise ValueError('generated evidence mismatch: ' + str(path))
    print(json.dumps(summary['counts'], sort_keys=True))


if __name__ == '__main__':
    main()
