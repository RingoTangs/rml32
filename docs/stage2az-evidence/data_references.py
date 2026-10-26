"""Check data-object identity and exact-reference projections; no execution or GC proof."""
import re
import struct

TARGET = '5d4ebac58d8c415d9cf95bbcdd50f78afac91adf4f7e667ea3afc6f5336a6f34'


def require(value, message):
    if not value:
        raise ValueError(message)


def address(value):
    require(isinstance(value, str) and re.fullmatch(r'0x[0-9a-fA-F]+', value),
            'invalid data-reference address')
    return int(value, 16)


def validate_reference_records(records):
    require(isinstance(records, list), 'reference inventory must be a list')
    byid = {}
    for record in records:
        eid = record['evidence_id']
        require(re.fullmatch(r'ev_[0-9a-f]{64}', eid) and eid not in byid,
                'invalid/duplicate reference evidence ID')
        require(record['operation'] == 'xrefs' and record['authority'] == 'shipped-artifact'
                and record['confidence'] == 'observed', 'reference record is not observed static xrefs')
        subject = record['subject']
        require(subject['digest']['sha256'] == TARGET and subject['architecture'] == 'x86'
                and subject['format'] == 'elf', 'wrong reference target')
        require(record['provider']['id'] == 'ghidra' and record['provider']['version'] == '12.1.4',
                'wrong reference provider')
        require(record['analysis_profile'].get('digest') and record['limitations'],
                'incomplete reference provenance')
        address(record['parameters']['address'])
        normalized, raw = record['normalized_result'], record['raw_result']
        require(isinstance(normalized, list) and isinstance(raw, list) and normalized == raw,
                'incomplete or changed reference projection')
        for item in normalized:
            address(item)
        byid[eid] = record
    return byid


def object_symbol(elf, name):
    matches = []
    for section in elf.sections:
        if section[1] != 2:  # SHT_SYMTAB; the historical Elf helper indexes functions only.
            continue
        strings = elf.section_data(elf.sections[section[6]])
        raw = elf.section_data(section)
        require(section[9] == 16 and len(raw) % 16 == 0, 'invalid object symbol table')
        for offset in range(0, len(raw), 16):
            label, start, size, info, _, index = struct.unpack_from('<IIIBBH', raw, offset)
            if info & 15 != 1 or not index:  # STT_OBJECT, defined section.
                continue
            end = strings.index(b'\0', label)
            if strings[label:end].decode() == name:
                matches.append(dict(address=start, size=size, section=index))
    require(len(matches) == 1, 'missing/ambiguous ELF data-object symbol: ' + name)
    return matches[0]


def section_name(elf, section):
    names_index = struct.unpack_from('<H', elf.data, 50)[0]
    names = elf.section_data(elf.sections[names_index])
    end = names.index(b'\0', section[0])
    return names[section[0]:end].decode()


def typed_references(record, source, target):
    normalized = record['normalized_result']
    return [r for key in ('outgoing_references', 'incoming_references') for r in normalized[key]
            if r.get('source_address') is not None and r.get('target_address') is not None
            and re.fullmatch(r'0x[0-9a-fA-F]+', r['source_address'])
            and re.fullmatch(r'0x[0-9a-fA-F]+', r['target_address'])
            and address(r['source_address']) == source and address(r['target_address']) == target]


def function_record(byid, eid, name):
    require(eid in byid, 'missing data-access function evidence')
    record = byid[eid]
    require(record['operation'] == 'analyze_function' and
            record['normalized_result']['procedure']['name'] == name,
            'data-access function dossier differs')
    return record


def validate_data_references(root, elf, records, data, byid, _findings, _anchors):
    require(data['stage'] == '2AZ' and data['target_sha256'] == TARGET and
            data['target_execution'] == 'none', 'data-reference scope differs')
    references = validate_reference_records(records)
    objects = data['objects']
    require(objects and len({o['id'] for o in objects}) == len(objects), 'duplicate/empty data objects')
    expected = {'_pdb_extendGrpList': (0x0860d024, 4, 8), '_vmInits': (0x083cf960, 400, 1)}
    require({o['name'] for o in objects} == set(expected) and len(objects) == len(expected),
            'data-object inventory differs')
    objects_byid = {}
    for obj in objects:
        start, size, section_type = expected[obj['name']]
        symbol = object_symbol(elf, obj['name'])
        require(address(obj['address']) == start == symbol['address'] and
                obj['size'] == size == symbol['size'] and obj['elf_symbol_type'] == 'OBJECT',
                'ELF data-object identity differs')
        section = elf.sections[symbol['section']]
        require(section[1] == section_type == obj['section_type'] and
                section_name(elf, section) == obj['section_name'] and
                section[3] <= start and start + size <= section[3] + section[5],
                'ELF data-object section differs')
        if section_type == 8:
            require(obj['file_offset'] is None and obj['bytes_hex'] is None,
                    'BSS object cannot claim file-backed bytes')
        else:
            offset, body = elf.locate(start, size)
            require(obj['file_offset'] == offset and isinstance(obj['bytes_hex'], str) and
                    bytes.fromhex(obj['bytes_hex']) == body, 'data-object bytes differ')
        objects_byid[obj['id']] = obj

    queries = data['queries']
    require(len({q['id'] for q in queries}) == len(queries), 'duplicate data query ID')
    require(len(queries) == len(references) and
            {q['evidence_id'] for q in queries} == references.keys(),
            'reference records lack one-to-one data use and coverage')
    queries_byid = {}
    for query in queries:
        require(query['object_id'] in objects_byid, 'dangling data-query object')
        obj = objects_byid[query['object_id']]
        target = address(query['address'])
        require(address(obj['address']) <= target < address(obj['address']) + obj['size'],
                'data query outside named object')
        record = references[query['evidence_id']]
        require(query['operation'] == 'xrefs' and query['status'] == 'complete' and
                target == address(record['parameters']['address']), 'data query differs from retained request')
        require(query['source_addresses'] == record['normalized_result'],
                'data query dropped or changed source addresses')
        require(isinstance(query['coverage'], str) and query['coverage'].strip() and
                isinstance(query['purpose'], str) and query['purpose'].strip() and
                isinstance(query['limitations'], list) and query['limitations'] and
                all(limit in query['limitations'] for limit in record['limitations']),
                'data query lacks purpose or retained coverage limits')
        queries_byid[query['id']] = query

    accesses = data['accesses']
    require(len({r['id'] for r in accesses}) == len(accesses), 'duplicate data-access ID')
    seen = set()
    for access in accesses:
        require(access['query_id'] in queries_byid, 'dangling data-access query')
        query = queries_byid[access['query_id']]
        source, target = address(access['source_address']), address(access['target_address'])
        require(source in {address(a) for a in query['source_addresses']} and
                target == address(query['address']), 'data access is absent from exact reference projection')
        record = function_record(byid, access['function_evidence'], access['procedure'])
        normalized = record['normalized_result']
        require(any(address(span['start']) <= source <= address(span['end'])
                    for span in normalized['procedure']['body']['ranges']),
                'data-access source outside function body')
        require(access['instruction'] in normalized['assembly'] and
                address(access['instruction'].split(':', 1)[0]) == source,
                'data-access instruction differs')
        raw = bytes.fromhex(access['bytes_hex'])
        offset, body = elf.locate(source, len(raw))
        require(raw and body == raw and offset == access['file_offset'], 'data-access bytes differ')
        candidates = typed_references(record, source, target)
        require(isinstance(access['reference_kind'], dict) and
                access['reference_kind'].get('available') is True and
                any(access['reference_kind'] == r['kind'] for r in candidates),
                'data-access kind is not supplied by the named function dossier')
        require(access['role'] and access['limitations'], 'data access lacks local interpretation boundary')
        pair = (query['id'], source)
        require(pair not in seen, 'duplicate source access for data query')
        seen.add(pair)
    expected_accesses = {(q['id'], address(a)) for q in queries for a in q['source_addresses']}
    require(seen == expected_accesses, 'data access inventory does not cover exact query results')

    registrations = data['registrations']
    require(registrations and len({r['id'] for r in registrations}) == len(registrations),
            'duplicate/empty data registrations')
    table = next(o for o in objects if o['name'] == '_vmInits')
    checked = set()
    for row in registrations:
        require(row['object_id'] == table['id'] and row['record_index'] == 19 and
                row['stride'] == 16 and row['field_offset'] == 8, 'scheduler table layout differs')
        location = address(table['address']) + row['record_index'] * row['stride'] + row['field_offset']
        require(location == address(row['address']) == 0x083cfa98, 'scheduler table slot differs')
        require(row['entry'] == 'vm_shutdownScheduler' and
                address(row['entry_address']) == elf.symbols[row['entry']]['address'] == 0x080ae0e8,
                'scheduler registration target differs')
        offset, raw = elf.locate(location, 4)
        require(offset == row['file_offset'] and bytes.fromhex(row['bytes_hex']) == raw and
                struct.unpack('<I', raw)[0] == address(row['entry_address']),
                'scheduler registration pointer bytes differ')
        require(row['evidence'] and set(row['evidence']) <= byid.keys() and
                row['consumer_evidence'] in row['evidence'], 'dangling registration evidence')
        require(row['consumer'] == 'vm_shutdown', 'scheduler registration consumer differs')
        consumer = function_record(byid, row['consumer_evidence'], row['consumer'])
        source, observed = address(row['consumer_instruction_address']), address(row['observed_reference_address'])
        require(source == 0x080b5cb4 and observed == 0x083cfae8 and
                typed_references(consumer, source, observed), 'consumer indexed reference differs')
        require(any(a['function_evidence'] == row['consumer_evidence'] and
                    address(a['source_address']) == source and address(a['target_address']) == observed
                    for a in accesses), 'registration consumer lacks checked access contact')
        require(any(r['operation'] == 'analyze_function' and
                    r['normalized_result']['procedure']['name'] == row['entry'] and
                    typed_references(r, location, address(row['entry_address']))
                    for eid, r in byid.items() if eid in row['evidence']),
                'scheduler registration lacks target dossier data reference')
        require(row['guard'] and row['limitations'], 'registration lacks conditional execution boundary')
        require(location not in checked, 'duplicate scheduler registration slot')
        checked.add(location)
