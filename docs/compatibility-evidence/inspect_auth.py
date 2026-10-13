"""Decode the older 1/driver bytecode ABI from its Ghidra loader evidence."""
import csv
import gzip
import json
import struct
from pathlib import Path
from inspect_packages import ROOT, OUT, Reader, digest, text


class Elf:
    def __init__(self, path):
        self.data = path.read_bytes()
        h = struct.unpack_from('<16sHHIIIIIHHHHHH', self.data)
        self.sections = [struct.unpack_from('<10I', self.data, h[6] + i * h[11]) for i in range(h[12])]
        self.symbols, self.function_names = {}, {}
        for s in self.sections:
            if s[1] != 2:
                continue
            st = self.sections[s[6]]
            strings = self.data[st[4]:st[4] + st[5]]
            for pos in range(s[4], s[4] + s[5], s[9]):
                n, v, size, info, other, index = struct.unpack_from('<IIIBBH', self.data, pos)
                name = text(strings[n:])
                if index:
                    self.symbols[name] = v
                    if info & 15 == 2:
                        self.function_names[v] = name

    def read(self, address, length):
        s = next(s for s in self.sections if s[1] != 8 and s[3] <= address
                 and address + length <= s[3] + s[5])
        offset = s[4] + address - s[3]
        return self.data[offset:offset + length]

    def cstring(self, address):
        return text(self.read(address, 256))


MAIN_ORDER = ['efuns','efunsDebug','efunsPerformanceProfile','varEfuns','itableEfuns','itreeEfuns',
              'dpEfuns','resourceEfuns','graphEfuns','socketEfuns','procEfuns','fileEfuns',
              'objectEfuns','pdbEfuns','rpcEfuns','sqliteEfuns','tinyXmlEfuns','dbMysqlEfuns',
              'verigenEfuns','languageEfuns','mapEfuns','magicEfuns','gameEfuns','kbdpwdEfuns','acMatchEfuns']
AUTH_ORDER = ['efuns','varEfuns','dpEfuns','resourceEfuns','graphEfuns','socketEfuns','procEfuns',
              'fileEfuns','objectEfuns','pdbEfuns','rpcEfuns','dbMysqlEfuns','sqliteEfuns',
              'tinyXmlEfuns','languageEfuns']


def efuns(elf, order, stride, filename):
    rows = []
    for table in order:
        address, i = elf.symbols[table], 0
        while True:
            record = elf.read(address + i * stride, stride)
            pointer = struct.unpack_from('<I', record)[0]
            if not pointer:
                break
            routine = struct.unpack_from('<I', record, 20)[0]
            rows.append(dict(index=len(rows), table=table, record=hex(address + i * stride),
                             name=elf.cstring(pointer), native_address=hex(routine),
                             native_name=elf.function_names.get(routine, ''),
                             return_type=record[24] if stride == 28 else struct.unpack_from('<I', record, 24)[0]))
            i += 1
    with (OUT / filename).open('w', newline='') as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    return rows


def old_actions(reader, instructions):
    offset = reader.pos
    nodes, filename, line = [], None, 0
    for _ in range(reader.count()):
        pos = reader.pos
        flag, parameters, instruction, unused = struct.unpack('<4B', reader.take(4))
        if flag & 1:
            filename = reader.string()
        if flag & 2:
            line = reader.i32()
        if not filename or instruction >= len(instructions) or (parameters > 4 and parameters != 255):
            raise ValueError(f'unsupported legacy node at {pos:#x}: file={filename}, '
                             f'flag={flag}, parameters={parameters}, opcode={instruction}')
        args = [reader.i32() for _ in range(min(parameters, 4))]
        routine = instructions[instruction]
        literal = None
        cases = []
        if routine in ('vm_pushString', 'vm_pushSharedString'):
            literal = reader.string()
        elif routine == 'vm_switch':
            for _ in range(reader.count()):
                raw = reader.take(16)
                item = dict(raw_hex=raw.hex())
                if struct.unpack_from('<i', raw)[0] == 3:
                    item['literal'] = reader.string()
                cases.append(item)
        nodes.append(dict(offset=pos, opcode=instruction, name=routine, parameters=parameters, args=args,
                          literal=literal, cases=cases, file=filename, line=line))
    return dict(offset=offset, nodes=nodes)


def old_function(reader, instructions):
    offset = reader.pos
    raw = reader.take(44)
    name = reader.string()
    args, locals_ = [], []
    for dest in (args, locals_):
        for _ in range(reader.count()):
            item = reader.take(24)
            dest.append(dict(name=reader.string(), raw_hex=item.hex()))
    return dict(offset=offset, name=name, index=struct.unpack_from('<i', raw, 20)[0],
                arguments=args, locals=locals_, code=old_actions(reader, instructions))


def old_program(data, instructions):
    reader = Reader(data)
    assert reader.i32() == 0x10001
    opaque_header, base = reader.i32(), reader.i32()
    defines = []
    for _ in range(reader.count()):
        defines.append(dict(nargs=reader.i32(), name=reader.string(), expansion=reader.string()))
    names_count, names_size = reader.count(), reader.count()
    names = reader.take(names_size)
    hidden = [old_function(reader, instructions) for _ in range(reader.count())]
    # Both supplied old .o files are global; object inheritance is deliberately unsupported here.
    variables, efuns_count, functions_count = reader.i32(), reader.i32(), reader.i32()
    dlts, functions = [], []
    n = reader.count()
    if n != names_count:
        raise ValueError('DLT/name count mismatch')
    for _ in range(n):
        pos = reader.pos
        raw = reader.take(20)
        name_offset, attrib = struct.unpack_from('<II', raw)
        if name_offset >= len(names):
            raise ValueError('bad name offset')
        name = text(names[name_offset:])
        dlts.append(dict(offset=pos, name=name, attrib=attrib, raw_hex=raw.hex()))
        if not attrib & 0xC0000:
            if not attrib & 0x8000000:
                raise ValueError('bad DLT kind')
            fn = old_function(reader, instructions)
            fn['dlt_name'] = name
            functions.append(fn)
    init = old_actions(reader, instructions)
    if reader.pos != len(data):
        raise ValueError(f'trailing bytes {len(data) - reader.pos}')
    return dict(mode='legacy-global', function_base=base, opaque_header=opaque_header,
                variables=variables, efuns_count=efuns_count, max_functions=functions_count,
                defines=defines, functions=functions, hidden=hidden, dlts=dlts,
                init=init, bytes_consumed=reader.pos)


def run():
    main = Elf(ROOT / 'magic_Linux32')
    auth = Elf(ROOT / '1/driver')
    main_efuns = efuns(main, MAIN_ORDER, 28, 'efuns.csv')
    auth_efuns = efuns(auth, AUTH_ORDER, 32, 'auth-efuns.csv')
    assert len(main_efuns) == 559
    assert len(auth_efuns) == 389
    table = auth.read(auth.symbols['instructions'], 85 * 4)
    instructions = [auth.function_names.get(struct.unpack_from('<I', table, i * 4)[0], '<unknown>') for i in range(85)]
    (OUT / 'auth-opcodes.json').write_text(json.dumps(instructions, indent=2)+'\n')
    programs = []
    for path in sorted((ROOT / '1/service').glob('*.o')):
        data = path.read_bytes()
        p = old_program(data, instructions)
        p.update(source=str(path.relative_to(ROOT)), sha256=digest(data))
        programs.append(p)
        print(p['source'], 'base',p['function_base'],'efuns',p['efuns_count'],
              'functions',len(p['functions']),'init_nodes',len(p['init']['nodes']))
    (OUT / 'auth-programs.json.gz').write_bytes(gzip.compress(json.dumps(programs,ensure_ascii=False).encode(),mtime=0))


if __name__ == '__main__':
    run()
