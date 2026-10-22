import json
from pathlib import Path
import tempfile
import unittest

from verify_static_evidence import (CHANGED, ROOT, check_hashes, check_opcodes, digest,
                                    safe_path, verify)


class StaticEvidenceTests(unittest.TestCase):
    def test_delivery_preserves_reference_and_frozen_evidence(self):
        self.assertEqual(verify()['static_records'], 20)

    def test_scope_exempts_only_declared_runtime_and_progress_paths(self):
        self.assertEqual(CHANGED, {'src/lib.rs', 'src/prepared.rs', 'src/value.rs', 'src/vm.rs',
                                  'tests/vm_core.rs', 'README.md',
                                  'docs/magic_Linux32-rust-rewrite-roadmap.md'})
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'src').mkdir()
            (root / 'src/vm.rs').write_bytes(b'new runtime')
            (root / 'magic_Linux32').write_bytes(b'reference')
            baseline = {'src/vm.rs': digest(b'old runtime'), 'magic_Linux32': digest(b'reference')}
            check_hashes(root, baseline, CHANGED)
            (root / 'magic_Linux32').write_bytes(b'tampered reference')
            with self.assertRaisesRegex(ValueError, 'protected input changed'):
                check_hashes(root, baseline, CHANGED)

    def test_exempt_source_cannot_be_deleted(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaisesRegex(ValueError, 'missing baseline input'):
                check_hashes(Path(directory), {'src/vm.rs': digest(b'old')}, CHANGED)

    def test_paths_cannot_escape_root(self):
        for name in ['', '/tmp/source', '../source', 'docs/../../source', 'docs\\source']:
            with self.subTest(name=name), self.assertRaises(ValueError):
                safe_path(ROOT, name)

    def test_opcode_boundary_rejects_duplicate_or_missing_entries(self):
        source = (ROOT / 'src/prepared.rs').read_text()
        baseline = json.loads((ROOT / 'docs/compatibility-evidence/compatibility-summary.json').read_text())
        check_opcodes(source, baseline)
        for changed in [source.replace('0x55,', '0x51,'), source.replace('0x55,', '')]:
            with self.assertRaisesRegex(ValueError, 'opcode boundary'):
                check_opcodes(changed, baseline)


if __name__ == '__main__':
    unittest.main()
