"""Read this exact ELF sample and export tables/text; never load or execute it."""
import csv
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
EXPECTED = "5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34"
data = (ROOT / "magic_Linux32").read_bytes()
assert hashlib.sha256(data).hexdigest() == EXPECTED, "Unexpected sample"
header = struct.unpack_from("<16sHHIIIIIHHHHHH", data)
sections = [struct.unpack_from("<10I", data, header[6] + i * header[11])
            for i in range(header[12])]


def read(address, length):
    section = next(s for s in sections if s[1] != 8 and s[3] <= address
                   and address + length <= s[3] + s[5])
    offset = section[4] + address - section[3]
    return data[offset:offset + length], offset


def cstring(address):
    chunk, _ = read(address, 256)
    return chunk.split(b"\0", 1)[0].decode("utf-8", errors="replace")


symbols = {}
imports = []
needed = []
for section in sections:
    if section[1] not in (6, 11):
        continue
    strings_section = sections[section[6]]
    strings = data[strings_section[4]:strings_section[4] + strings_section[5]]
    if section[1] == 6:
        for offset in range(section[4], section[4] + section[5], 8):
            tag, value = struct.unpack_from("<II", data, offset)
            if tag == 1:
                needed.append(strings[value:strings.find(b"\0", value)].decode())
    else:
        for offset in range(section[4], section[4] + section[5], section[9]):
            name, value, size, info, other, index = struct.unpack_from("<IIIBBH", data, offset)
            if name and index == 0:
                imports.append(dict(name=strings[name:strings.find(b"\0", name)].decode(),
                                    value=hex(value), symbol_type=info & 15, binding=info >> 4))
for section in sections:
    if section[1] != 2:
        continue
    strings_section = sections[section[6]]
    strings = data[strings_section[4]:strings_section[4] + strings_section[5]]
    for offset in range(section[4], section[4] + section[5], section[9]):
        name, value, size, info, other, index = struct.unpack_from("<IIIBBH", data, offset)
        if index and info & 15 == 2:
            symbols[value] = strings[name:strings.find(b"\0", name)].decode(errors="replace")


def save_csv(name, fields, rows):
    with (OUT / name).open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


save_csv("elf-imports.csv", list(imports[0]), imports)
commands = []
for base, count, table in [(0x83C8E60, 924, "client"), (0x83C8E40, 4, "server")]:
    chunk, _ = read(base, count * 8)
    for offset in range(0, len(chunk), 8):
        number, kind, function = struct.unpack_from("<HHI", chunk, offset)
        if function == 0:
            break
        commands.append(dict(table=table, record=hex(base + offset), command=hex(number),
                             type=hex(kind), function=hex(function), name=symbols.get(function, "")))
save_csv("commands.csv", list(commands[0]), commands)

actions = []
chunk, _ = read(0x83CC680, 87 * 28)
for offset in range(0, len(chunk), 28):
    opcode = chunk[offset]
    compiler, entry = struct.unpack_from("<II", chunk, offset + 4)
    memo, syntax, attrib = struct.unpack_from("<III", chunk, offset + 16)
    actions.append(dict(record=hex(0x83CC680 + offset), opcode=hex(opcode),
                        entry=hex(entry), name=symbols.get(entry, ""), compiler=hex(compiler),
                        parameters=chunk[offset + 12], memo=cstring(memo) if memo else "",
                        attrib=hex(attrib)))
save_csv("actions.csv", list(actions[0]), actions)

initializers = []
chunk, _ = read(0x83CF960, 25 * 16)
for offset in range(0, len(chunk), 16):
    name, entry, shutdown, flag = struct.unpack_from("<IIII", chunk, offset)
    initializers.append(dict(record=hex(0x83CF960 + offset), module=cstring(name),
                             entry=hex(entry), name=symbols.get(entry, ""),
                             shutdown=hex(shutdown), flag=flag))
save_csv("initializers.csv", list(initializers[0]), initializers)


def decompress(payload):
    # Mirrors lzss_decompress (0x80cb47c): 4096-byte ring, LSB-first flags.
    ring = bytearray(4096)
    ring[:0xFEE] = b" " * 0xFEE
    cursor, flags, offset = 0xFEE, 0, 0
    result = bytearray()
    while offset < len(payload):
        flags >>= 1
        if flags & 0x100 == 0:
            flags = payload[offset] | 0xFF00
            offset += 1
        if flags & 1:
            if offset == len(payload):
                break
            values = [payload[offset]]
            offset += 1
            for value in values:
                result.append(value)
                ring[cursor] = value
                cursor = (cursor + 1) & 4095
        else:
            if offset + 1 >= len(payload):
                break
            low, high = payload[offset:offset + 2]
            offset += 2
            source = low | ((high & 0xF0) << 4)
            for i in range((high & 15) + 3):
                value = ring[(source + i) & 4095]
                result.append(value)
                ring[cursor] = value
                cursor = (cursor + 1) & 4095
    return bytes(result)


embedded = []
table, _ = read(0x83CE680, 16 * 268)
for index in range(16):
    record = table[index * 268:(index + 1) * 268]
    name = record[:256].split(b"\0", 1)[0].decode()
    if not name:
        break
    address, length, compressed = struct.unpack_from("<III", record, 256)
    blob, offset = read(address, length)
    decoded = blob
    expected_size = length
    if compressed:
        kind, expected_size = struct.unpack(">II", blob[:8])
        decoded = decompress(blob[8:]) if kind else blob[8:]
        assert len(decoded) == expected_size
    obfuscation = "none"
    if decoded.startswith(b"!"):
        decoded = bytes(value ^ 0xA5 for value in decoded[1:])
        obfuscation = "marker !, XOR 0xa5"
    elif decoded.startswith(b"*"):
        key, result = 0xB6, bytearray()
        for value in decoded[1:]:
            result.append(value ^ key)
            key = ((key * 3 + 5) ^ 0x27) & 255
        decoded = bytes(result)
        obfuscation = "marker *, evolving XOR key"
    # Deliberately use .txt: this is recovered evidence, not runnable source.
    destination = OUT / (name + ".txt")
    destination.write_bytes(decoded)
    embedded.append(dict(name=name, address=hex(address), file_offset=hex(offset),
                         stored_bytes=length, decompressed_bytes=expected_size,
                         decoded_bytes=len(decoded), obfuscation=obfuscation,
                         stored_sha256=hashlib.sha256(blob).hexdigest(),
                         decoded_sha256=hashlib.sha256(decoded).hexdigest()))

(OUT / "static-extraction.json").write_text(json.dumps(dict(
    sample_sha256=EXPECTED, elf_type=header[1], machine=header[2], entry=hex(header[4]),
    needed_libraries=needed, undefined_dynamic_symbols=len(imports),
    command_records=len(commands), action_records=len(actions), embedded=embedded), indent=2) + "\n")
assert hashlib.sha256((ROOT / "magic_Linux32").read_bytes()).hexdigest() == EXPECTED
print(json.dumps(dict(commands=len(commands), actions=len(actions), embedded=embedded), indent=2))
