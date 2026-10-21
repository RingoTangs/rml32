import copy
import gzip
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from capture_native import HERE, ROOT, Elf, canonical, digest
from verify_evidence import safe_source, validate_links, validate_native, verify


class IntegrityTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.bundle = json.loads(gzip.decompress((HERE / 'native.json.gz').read_bytes()))
        cls.target = (ROOT / 'magic_Linux32').read_bytes()

    def test_retained_bytes_cover_each_symbol_and_protected_inputs_match(self):
        result = verify()
        self.assertEqual(result['instruction_locations'], 2018)
        self.assertEqual(result['expected_cases'], 43)
        self.assertEqual(result['protected_files'], 503)

    def test_changed_target_rejected(self):
        with self.assertRaisesRegex(ValueError, 'target identity'):
            validate_native(self.bundle, self.target + b'!')

    def test_record_payload_cannot_change_without_digest(self):
        bundle = copy.deepcopy(self.bundle)
        bundle['records'][0]['result']['stdout'] += 'altered'
        with self.assertRaisesRegex(ValueError, 'record digest'):
            validate_native(bundle, self.target)

    def test_rehashed_wrong_offset_bytes_and_symbol_are_rejected(self):
        for field, value in [('file_offset', 0), ('bytes_hex', '00'), ('address', 1)]:
            with self.subTest(field=field):
                bundle = copy.deepcopy(self.bundle)
                record = bundle['records'][0]
                record[field] = value
                record['record_id'] = 'local_sha256_' + digest(canonical({k: v for k, v in record.items() if k != 'record_id'}))
                with self.assertRaises(ValueError):
                    validate_native(bundle, self.target)

    def test_rehashed_missing_instruction_is_not_complete_capture(self):
        bundle = copy.deepcopy(self.bundle)
        record = bundle['records'][0]
        record['result']['stdout'] = record['result']['stdout'].replace(' 8089dc8: 55', ' 8089dc9: 55')
        record['record_id'] = 'local_sha256_' + digest(canonical({k: v for k, v in record.items() if k != 'record_id'}))
        with self.assertRaisesRegex(ValueError, 'instruction gap'):
            validate_native(bundle, self.target)

    def test_table_byte_offset_and_source_reference_mismatch(self):
        for field, value in [('file_offset', 0), ('bytes_hex', '00'*256), ('use_sites', ['0x0'])]:
            bundle = copy.deepcopy(self.bundle)
            bundle['hash_table'][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                validate_native(bundle, self.target)

    def test_inventory_duplicate_and_missing_function(self):
        for records in (self.bundle['records'][:-1], self.bundle['records'] + self.bundle['records'][:1]):
            bundle = copy.deepcopy(self.bundle); bundle['records'] = records
            with self.assertRaisesRegex(ValueError, 'bijection'):
                validate_native(bundle, self.target)

    def test_reused_source_hash_function_identity_and_case_links(self):
        for name, field, value in [('reused-manifest.json', 'source_sha256', '0'*64),
                                   ('function-manifest.json', 'name', 'wrong')]:
            with tempfile.TemporaryDirectory() as tmp:
                directory = Path(tmp) / 'evidence'
                shutil.copytree(HERE, directory)
                data = json.loads((directory / name).read_text())
                data[0][field] = value
                (directory / name).write_text(json.dumps(data))
                with self.subTest(field=field), self.assertRaises(ValueError):
                    verify(ROOT, directory)
        with self.assertRaisesRegex(ValueError, 'unknown evidence'):
            validate_links([], [dict(id='x', expected='y', evidence_ids=['missing'], classification='static-deduction')], set())

    def test_path_escape_and_non_elf_rejected(self):
        for name in ('../magic_Linux32', '/tmp/x'):
            with self.assertRaises(ValueError):
                safe_source(ROOT, name)
        with self.assertRaises(ValueError):
            Elf(b'not an ELF')
        from verify_validation import COMMANDS, validate_commands
        with self.assertRaisesRegex(ValueError, 'command inventory'):
            validate_commands([])
        records = [dict(argv=c, returncode=1, stdout='', stderr='') for c in COMMANDS]
        with self.assertRaisesRegex(ValueError, 'failed command'):
            validate_commands(records)


if __name__ == '__main__':
    unittest.main()
