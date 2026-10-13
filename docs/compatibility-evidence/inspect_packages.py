"""Static PAK/VM decoder grounded in Ghidra loaders. Never executes VM code."""
import csv
import gzip
import hashlib
import json
import struct
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent


def digest(data):
    return hashlib.sha256(data).hexdigest()


def text(data):
    data = data.split(b"\0", 1)[0]
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        return data.decode("gb18030", errors="replace")


def lzss(payload, size):
    ring = bytearray(4096)
    ring[:0xFEE] = b" " * 0xFEE
    cursor, flags, pos = 0xFEE, 0, 0
    result = bytearray()
    while pos < len(payload):
        flags >>= 1
        if flags & 0x100 == 0:
            flags = payload[pos] | 0xFF00
            pos += 1
        if flags & 1:
            if pos >= len(payload):
                break
            value = payload[pos]
            pos += 1
            result.append(value)
            ring[cursor] = value
            cursor = (cursor + 1) & 4095
        else:
            if pos + 1 >= len(payload):
                break
            low, high = payload[pos:pos + 2]
            pos += 2
            source = low | ((high & 0xF0) << 4)
            for i in range((high & 15) + 3):
                value = ring[(source + i) & 4095]
                result.append(value)
                ring[cursor] = value
                cursor = (cursor + 1) & 4095
        if len(result) > size:
            raise ValueError("LZSS output exceeds declared size")
    if len(result) != size:
        raise ValueError(f"LZSS size mismatch {len(result)} != {size}")
    return bytes(result)


class Reader:
    def __init__(self, data):
        self.data, self.pos = data, 0

    def take(self, count):
        if count < 0 or self.pos + count > len(self.data):
            raise ValueError(f"invalid read {count} at {self.pos:#x}")
        result = self.data[self.pos:self.pos + count]
        self.pos += count
        return result

    def number(self, fmt):
        return struct.unpack(fmt, self.take(struct.calcsize(fmt)))[0]

    def i32(self):
        return self.number("<i")

    def i16(self):
        return self.number("<h")

    def string(self):
        count = self.i32()
        return text(self.take(count)) if count > 0 else None

    def count(self):
        n = self.i32()
        if not 0 <= n <= 1000000:
            raise ValueError(f"invalid count {n} at {self.pos - 4:#x}")
        return n


def code_segment(reader):
    start = reader.pos
    symbols = []
    filename, line = None, 0
    for _ in range(reader.count()):
        flag = reader.number("<B")
        if flag & 1:
            filename = reader.string()
        if flag & 2:
            line = reader.i32()
        if not filename:
            raise ValueError("missing symbol filename")
        symbols.append([filename, line])
    count = reader.count()
    nodes_offset = reader.pos
    raw = reader.take(count * 4)
    nodes = [list(struct.unpack_from("<hbB", raw, i * 4)) for i in range(count)]
    if any(n[2] >= 87 for n in nodes):
        raise ValueError("unsupported opcode")
    meta_offset = reader.pos + 4
    metadata = reader.take(reader.count())
    runtime_count = reader.count()
    if runtime_count not in (0, count):
        raise ValueError("runtime count does not match node count")
    runtime = reader.take(runtime_count * 4)
    return dict(offset=start, nodes_offset=nodes_offset, nodes=nodes,
                metadata_offset=meta_offset, metadata_hex=metadata.hex(), symbols=symbols,
                runtime_count=runtime_count, runtime_hex=runtime.hex())


def function(reader):
    offset = reader.pos
    raw = reader.take(64)
    name = reader.string()
    params, locals_ = [], []
    for collection in (params, locals_):
        n = reader.i16()
        if n < 0:
            raise ValueError("negative argument/local count")
        for _ in range(n):
            item = reader.take(20)
            collection.append(dict(name=reader.string(), flag=item[4], type=item[5],
                                   attrib=item[6], index=struct.unpack_from("<h", item, 8)[0]))
    return dict(offset=offset, name=name, token=struct.unpack_from("<I", raw, 4)[0],
                attrib=struct.unpack_from("<h", raw, 8)[0], local_count=struct.unpack_from("<h", raw, 10)[0],
                return_type=raw[13], max_args=raw[14], min_args=raw[15],
                arguments=params, locals=locals_, code=code_segment(reader))


def program(data, object_mode):
    reader = Reader(data)
    if reader.i32() != 0x10001:
        raise ValueError("not VM bytecode")
    opaque_header, base = reader.i32(), reader.i32()
    defines = []
    for _ in range(reader.count()):
        defines.append(dict(nargs=reader.i32(), name=reader.string(), expansion=reader.string()))
    strings = [reader.string() for _ in range(reader.count())]
    names_count, names_size = reader.count(), reader.count()
    names = reader.take(names_size)
    max_vars, vars_count = reader.i16(), reader.i16()
    variables = []
    if object_mode:
        if vars_count < 0:
            raise ValueError("negative variables")
        for _ in range(vars_count):
            raw = reader.take(16)
            variables.append(dict(raw_hex=raw.hex(), name=reader.string()))
    efuns_count, max_functions = reader.i16(), reader.i16()
    if not 0 < efuns_count < 5000 or max_functions < 0:
        raise ValueError("invalid function/efun counts")
    hidden = [function(reader) for _ in range(reader.count())]
    inherits = []
    constitution_count = None
    if object_mode:
        constitution_count = reader.i16()
        if constitution_count < 1:
            raise ValueError("empty constitution table")
        for _ in range(reader.count()):
            inherits.append(dict(name=reader.string(), offsets=[reader.i16() for _ in range(6)]))
    dlts, functions = [], []
    dlt_count = reader.count()
    if names_count != dlt_count:
        raise ValueError("name/DLT count mismatch")
    for _ in range(dlt_count):
        offset = reader.pos
        raw = reader.take(20)
        name_offset, attrib = struct.unpack_from("<II", raw)
        if name_offset >= len(names):
            raise ValueError("name offset outside pool")
        name = text(names[name_offset:])
        index = struct.unpack_from("<h", raw, 16)[0]
        entry = dict(offset=offset, name=name, attrib=attrib, index=index, raw_hex=raw.hex())
        if not attrib & 0xC0000:
            if not attrib & 0x8000000:
                raise ValueError("unknown DLT kind")
            fn = function(reader)
            fn.update(dlt_name=name, index=index)
            functions.append(fn)
        dlts.append(entry)
    init = code_segment(reader)
    if reader.pos != len(data):
        raise ValueError(f"trailing bytes: {len(data) - reader.pos}")
    if not init['nodes'] or init['nodes'][-1][2] != 0x1D:
        raise ValueError("missing final finish instruction")
    return dict(mode="object" if object_mode else "global", opaque_header=opaque_header,
                function_base=base, defines=defines, strings=strings, max_vars=max_vars,
                variables=variables, efuns_count=efuns_count, max_functions=max_functions,
                hidden=hidden, constitution_count=constitution_count, inherits=inherits,
                dlts=dlts, functions=functions, init=init, bytes_consumed=reader.pos)


def decode_program(data):
    results, errors = [], []
    for mode in (False, True):
        try:
            results.append(program(data, mode))
        except (ValueError, struct.error, IndexError) as exc:
            errors.append(str(exc))
    if len(results) == 1:
        return results[0]
    raise ValueError("ambiguous modes" if results else " / ".join(errors))


def run():
    members, programs, issues = [], [], []
    summary = []
    for path in sorted(ROOT.glob("*/pack_data/*.pak")):
        blob = path.read_bytes()
        label = str(path.relative_to(ROOT))
        sign, root_size, unused1, unused2 = struct.unpack_from("<4I", blob)
        if sign != 0x11223344:
            raise ValueError(f"unexpected PAK header: {label}")
        seen = set()
        package_members = []

        def walk(offset, size, prefix):
            if size % 64 or offset < 16 or offset + size > len(blob):
                raise ValueError("bad directory bounds")
            if (offset, size) in seen:
                raise ValueError("repeated/cyclic directory")
            seen.add((offset, size))
            for record in range(offset, offset + size, 64):
                attrib, data_offset, stored, original, modified = struct.unpack_from("<5I", blob, record)
                name = text(blob[record + 20:record + 64])
                if not name or name in (".", "..") or "/" in name or "\\" in name:
                    raise ValueError(f"unsafe/empty member name at {record:#x}")
                if data_offset + stored > len(blob):
                    raise ValueError("member exceeds package")
                member_name = prefix + name
                entry = dict(package=label, path=member_name, record_offset=record, attrib=attrib,
                             data_offset=data_offset, stored_bytes=stored, original_bytes=original,
                             modified=modified, directory=bool(attrib & 1))
                if attrib & 1:
                    package_members.append(entry)
                    walk(data_offset, stored, member_name + "/")
                else:
                    payload = blob[data_offset:data_offset + stored]
                    if attrib & 0x80000000:
                        if data_offset + original > len(blob):
                            raise ValueError("raw member exceeds package")
                        data = blob[data_offset:data_offset + original]
                    else:
                        try:
                            data = lzss(payload, original)
                        except ValueError as exc:
                            entry['decode_error'] = str(exc)
                            package_members.append(entry)
                            issues.append(dict(source=label + '/' + member_name,
                                               kind='member-decompression', error=str(exc)))
                            continue
                    entry['sha256'] = digest(data)
                    entry['format'] = "vm-bytecode" if data[:4] == b"\x01\x00\x01\x00" else "other"
                    package_members.append(entry)
                    if entry['format'] == "vm-bytecode":
                        try:
                            decoded = decode_program(data)
                            decoded.update(source=label + "/" + member_name, sha256=digest(data))
                            programs.append(decoded)
                        except ValueError as exc:
                            issues.append(dict(source=label + "/" + member_name, error=str(exc)))
        walk(16, root_size, "")
        members.extend(package_members)
        summary.append(dict(package=label, sha256=digest(blob), header=[sign, root_size, unused1, unused2],
                            members=len(package_members), files=sum(not m['directory'] for m in package_members),
                            bytecodes=sum(m.get('format') == 'vm-bytecode' for m in package_members)))
        print(label, summary[-1], flush=True)
    for path in sorted((ROOT / '1/service').glob('*.o')):
        data = path.read_bytes()
        try:
            decoded = decode_program(data)
            decoded.update(source=str(path.relative_to(ROOT)), sha256=digest(data))
            programs.append(decoded)
        except ValueError as exc:
            issues.append(dict(source=str(path.relative_to(ROOT)), error=str(exc)))
    with (OUT / 'members.json.gz').open('wb') as stream:
        stream.write(gzip.compress(json.dumps(members, ensure_ascii=False).encode(), mtime=0))
    with (OUT / 'programs.json.gz').open('wb') as stream:
        stream.write(gzip.compress(json.dumps(programs, ensure_ascii=False).encode(), mtime=0))
    (OUT / 'packages-summary.json').write_text(json.dumps(dict(packages=summary, programs=len(programs),
                                                            parse_issues=issues), indent=2, ensure_ascii=False)+'\n')
    fields = ['source', 'mode', 'function_base', 'efuns_count', 'max_functions', 'functions', 'hidden', 'strings', 'init_nodes']
    with (OUT / 'programs.csv').open('w', newline='') as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        for p in programs:
            writer.writerow({k: (len(p[k]) if k in ('functions', 'hidden', 'strings') else p[k])
                             for k in fields if k != 'init_nodes'} | {'init_nodes': len(p['init']['nodes'])})
    print('PARSED',len(programs),'ISSUES',len(issues), 'first',issues[:5], flush=True)


if __name__ == '__main__':
    run()
