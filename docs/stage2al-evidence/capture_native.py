"""Offline ELF32 evidence capture. Never executes the target; writes only with --write."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'
FUNCTIONS = ('vm_whashstr', 'vm_whashstrc', 'vm_whashbuf',
             'vm_safeAllocateMapping', 'vm_allocateMapping', 'vm_growMapping',
             'vm_safeFindForInsert', 'vm_findForInsert', 'vm_findInMapping',
             'vm_nodeFindInMapping', 'vm_mappingDelete', 'vm_deleteIntInMapping',
             'vm_freeNode', 'vm_emptyMapping', 'vm_unrefMapping',
             'vm_createMapping', 'vm_initForLoop', 'vm_cmpAndJmpInLoop',
             'vm_freeLoopValue', 'vm_condJmpForLoop', 'vm_stepJmpForLoop')


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


class Elf:
    def __init__(self, data):
        require(data[:7] == b'\x7fELF\x01\x01\x01', 'ELF32 little endian required')
        require(struct.unpack_from('<H', data, 18)[0] == 3, 'x86 required')
        self.data = data
        off = struct.unpack_from('<I', data, 32)[0]
        size, count = struct.unpack_from('<HH', data, 46)
        require(size == 40 and off + size * count <= len(data), 'section headers')
        self.sections = [struct.unpack_from('<10I', data, off + i * size) for i in range(count)]
        self.symbols = {}
        for section in self.sections:
            if section[1] != 2:
                continue
            strings = self.section_data(self.sections[section[6]])
            raw = self.section_data(section)
            require(section[9] == 16 and len(raw) % 16 == 0, 'symbol entries')
            for at in range(0, len(raw), 16):
                name, address, length, info, _, index = struct.unpack_from('<IIIBBH', raw, at)
                if info & 15 != 2 or not index or not length:
                    continue
                end = strings.index(b'\0', name)
                label = strings[name:end].decode()
                self.symbols[label] = dict(address=address, size=length, section=index)

    def section_data(self, section):
        off, size = section[4:6]
        require(off + size <= len(self.data), 'section file bounds')
        return self.data[off:off + size]

    def locate(self, address, size):
        matches = []
        for section in self.sections:
            if section[1] == 8:  # SHT_NOBITS is not file-backed.
                continue
            va, off, length = section[3:6]
            if va <= address and address + size <= va + length:
                matches.append(off + address - va)
        require(len(matches) == 1, 'unique file-backed address mapping')
        off = matches[0]
        require(off + size <= len(self.data), 'address file bounds')
        return off, self.data[off:off + size]


def command(args):
    p = subprocess.run(args, cwd=ROOT, capture_output=True, text=True, check=True)
    return dict(argv=args, returncode=p.returncode, stdout=p.stdout, stderr=p.stderr)


def capture():
    data = (ROOT / 'magic_Linux32').read_bytes()
    require(digest(data) == TARGET, 'target identity')
    elf = Elf(data)
    version = command(['objdump', '--version'])
    records = []
    for name in FUNCTIONS:
        symbol = elf.symbols[name]
        off, raw = elf.locate(symbol['address'], symbol['size'])
        result = command(['objdump', '-d', '--x86-asm-syntax=intel',
                          '--start-address=' + hex(symbol['address']),
                          '--stop-address=' + hex(symbol['address'] + symbol['size']),
                          '--disassemble-symbols=' + name, 'magic_Linux32'])
        record = dict(kind='local-static-disassembly', name=name, **symbol,
                      file_offset=off, bytes_hex=raw.hex(), bytes_sha256=digest(raw),
                      target_sha256=TARGET, tool_version=version, result=result,
                      limitations=['ELF symbol extent is not a recovered CFG body.',
                                   'No decompilation, runtime observation or new REA Evidence ID.',
                                   'Called allocators and error backends are not proven by this capture.'])
        record['record_id'] = 'local_sha256_' + digest(canonical(record))
        records.append(record)
    off, raw = elf.locate(0x83cc3e0, 256)
    table = dict(virtual_address='0x83cc3e0', file_offset=off, size=256,
                 bytes_hex=raw.hex(), bytes_sha256=digest(raw), target_sha256=TARGET,
                 use_function='vm_whashstr', use_sites=['0x8089de4', '0x8089e05', '0x8089e0e'])
    return dict(schema_version=1, target_sha256=TARGET, records=records, hash_table=table)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    bundle = capture()
    raw = gzip.compress(canonical(bundle), mtime=0)
    path = HERE / 'native.json.gz'
    if args.write:
        path.write_bytes(raw)
    else:
        require(json.loads(gzip.decompress(path.read_bytes())) == bundle, 'native capture changed')
    print(json.dumps(dict(functions=len(bundle['records']), target_sha256=TARGET)))


if __name__ == '__main__':
    main()
