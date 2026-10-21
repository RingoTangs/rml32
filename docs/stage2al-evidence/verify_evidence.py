"""Verify provenance, ELF instruction bytes, links and protected inputs; no target execution."""
import gzip
import json
from pathlib import Path
import re

from capture_native import ROOT, HERE, TARGET, FUNCTIONS, Elf, canonical, digest, require

SITES = {
    'vm_whashstr': ('8089dde:', 'movzx\teax, al', '0x83cc3e0'),
    'vm_safeFindForInsert': ('sar\tebx, 0x4', '809d52b:', '809d534:', '809d624:', '809d636:'),
    'vm_growMapping': ('809c96a:', '809c980:', '809c9a4:', '809c924:'),
    'vm_mappingDelete': ('809d02e:', '809d194:'),
    'vm_cmpAndJmpInLoop': ('80bcc49:', '80bcc59:', '80bcc89:', '80bcc92:', '80bcd5e:'),
}


def instructions(text):
    for line in text.splitlines():
        if not re.match(r'^\s*[0-9a-f]+:', line):
            continue
        prefix, sep, mnemonic = line.partition('\t')
        require(bool(sep), 'instruction delimiter')
        address, raw = prefix.split(':', 1)
        yield int(address.strip(), 16), bytes.fromhex(raw), mnemonic


def validate_native(bundle, data):
    require(digest(data) == TARGET and bundle['target_sha256'] == TARGET, 'target identity')
    elf = Elf(data)
    names = []
    instruction_count = 0
    for record in bundle['records']:
        name = record['name']; names.append(name)
        require(name in FUNCTIONS and record['kind'] == 'local-static-disassembly', 'native inventory')
        content = {k: v for k, v in record.items() if k != 'record_id'}
        require(record['record_id'] == 'local_sha256_' + digest(canonical(content)), 'local record digest')
        symbol = elf.symbols[name]
        require(all(record[k] == symbol[k] for k in symbol), 'native symbol identity')
        offset, raw = elf.locate(symbol['address'], symbol['size'])
        require(record['file_offset'] == offset and record['bytes_hex'] == raw.hex()
                and record['bytes_sha256'] == digest(raw), 'native body bytes')
        require(record['target_sha256'] == TARGET, 'record target')
        result = record['result']
        expected_argv = ['objdump', '-d', '--x86-asm-syntax=intel',
                         '--start-address=' + hex(symbol['address']),
                         '--stop-address=' + hex(symbol['address'] + symbol['size']),
                         '--disassemble-symbols=' + name, 'magic_Linux32']
        require(result['argv'] == expected_argv and result['returncode'] == 0, 'capture command')
        require(record['tool_version']['returncode'] == 0
                and 'Apple LLVM version 17.0.0' in record['tool_version']['stdout'], 'capture tool version')
        pc = symbol['address']
        for address, encoded, _ in instructions(result['stdout']):
            require(encoded and address == pc, 'instruction gap or wrong address')
            require(address + len(encoded) <= symbol['address'] + symbol['size'], 'instruction outside symbol')
            require(elf.locate(address, len(encoded))[1] == encoded, 'instruction bytes disagree with ELF')
            pc += len(encoded); instruction_count += 1
        require(pc == symbol['address'] + symbol['size'], 'incomplete instruction extent')
        for site in SITES.get(name, ()):
            require(site in result['stdout'], 'selected instruction site')
    require(len(names) == len(set(names)) and set(names) == set(FUNCTIONS), 'native function bijection')
    table = bundle['hash_table']
    require(table['virtual_address'] == '0x83cc3e0' and table['size'] == 256, 'hash table identity')
    require(table['use_function'] == 'vm_whashstr'
            and table['use_sites'] == ['0x8089de4', '0x8089e05', '0x8089e0e'], 'hash table use sites')
    offset, raw = elf.locate(int(table['virtual_address'], 16), 256)
    require(table['file_offset'] == offset and table['bytes_hex'] == raw.hex()
            and table['bytes_sha256'] == digest(raw) and table['target_sha256'] == TARGET, 'hash table bytes')
    require(len(set(raw)) == 256, 'hash table permutation')
    return instruction_count


def validate_links(findings, cases, ids):
    for rows in (findings, cases):
        labels = [r['id'] for r in rows]
        require(len(labels) == len(set(labels)), 'duplicate finding/case')
        for row in rows:
            require(bool(row['evidence_ids']) and set(row['evidence_ids']) <= ids, 'unknown evidence link')
            require(bool(row['statement'] if 'statement' in row else row['expected']), 'empty statement/outcome')
    for row in findings:
        require(row['classification'] in ('observation', 'static-inference', 'design', 'unknown'), 'finding classification')
        require(row['status'] in ('answered', 'partial', 'bounded', 'unresolved'), 'finding status')
    for row in cases:
        require(row['classification'] in ('static-deduction', 'rust-design', 'unresolved'), 'case classification')


def safe_source(root, name):
    path = Path(name)
    require(not path.is_absolute() and '..' not in path.parts, 'source path escape')
    result = (root / path).resolve()
    require(result.is_relative_to(root.resolve()), 'resolved source escape')
    return result


def verify(root=ROOT, here=HERE):
    root, here = Path(root), Path(here)
    native = json.loads(gzip.decompress((here / 'native.json.gz').read_bytes()))
    count = validate_native(native, (root / 'magic_Linux32').read_bytes())
    reused = json.loads(gzip.decompress((here / 'reused.json.gz').read_bytes()))['records']
    manifest = json.loads((here / 'reused-manifest.json').read_text())
    require(len(reused) == len(manifest) == 13, 'reused inventory')
    byid = {r['evidence_id']: r for r in reused}
    require(len(byid) == 13 and {m['evidence_id'] for m in manifest} == set(byid), 'reused bijection')
    sources = {}
    for item in manifest:
        raw = safe_source(root, item['source']).read_bytes()
        require(digest(raw) == item['source_sha256'], 'reused source digest')
        if item['source'] not in sources:
            sources[item['source']] = json.loads(gzip.decompress(raw))['records']
        record = byid[item['evidence_id']]
        require(record in sources[item['source']], 'reused record not in source')
        require(digest(canonical(record)) == item['record_sha256'], 'reused record hash')
        require(record['operation'] == item['operation'] and record.get('raw_result') is not None, 'reused operation/raw')
        require(record['subject']['digest']['sha256'] == TARGET, 'reused target')
        normalized = record['normalized_result']
        name = normalized['procedure']['name'] if isinstance(normalized, dict) else record['parameters']['procedure']
        require(name == item['name'], 'reused function identity')
    records = native['records'] + reused
    ids = {r.get('record_id', r.get('evidence_id')) for r in records}
    require(len(ids) == 34, 'all record IDs unique')
    functions = json.loads((here / 'function-manifest.json').read_text())
    require(len(functions) == 34 and {m['id'] for m in functions} == ids, 'function manifest bijection')
    for record in records:
        rid = record.get('record_id', record.get('evidence_id'))
        entry = next(m for m in functions if m['id'] == rid)
        n = record.get('normalized_result', {})
        p = n.get('procedure', {}) if isinstance(n, dict) else {}
        name = record.get('name', p.get('name', record.get('parameters', {}).get('procedure')))
        require(entry['name'] == name and entry['address'] == record.get('address', p.get('address')),
                'function manifest identity')
        require(entry['record_sha256'] == digest(canonical(record)), 'function manifest hash')
        require(entry['kind'] == ('local-static-disassembly' if 'record_id' in record else 'reused-rea'), 'function manifest kind')
        source = safe_source(root, entry['source']).read_bytes()
        require(entry['source_sha256'] == digest(source), 'function manifest source')
    findings = json.loads((here / 'findings.json').read_text())
    cases = json.loads((here / 'expected-cases.json').read_text())['cases']
    validate_links(findings, cases, ids)
    protected = json.loads((here / 'protected-inputs.json').read_text())
    require('magic_Linux32' in protected and 'src/vm.rs' in protected, 'protected inventory')
    for name, expected in protected.items():
        require(digest(safe_source(root, name).read_bytes()) == expected, 'protected file changed: ' + name)
    return dict(local_functions=21, reused_records=13, instruction_locations=count,
                findings=len(findings), expected_cases=len(cases), protected_files=len(protected))


if __name__ == '__main__':
    print(json.dumps(verify(), sort_keys=True))
