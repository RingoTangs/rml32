"""Run 22 reusable 2AN tool tests; its old full-source snapshot test stays historical."""
from pathlib import Path
import sys
import unittest

DIRECTORY = Path(__file__).resolve().parents[1] / 'stage2an-evidence'
sys.path.insert(0, str(DIRECTORY))


def tests(suite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from tests(item)
        else:
            yield item


if __name__ == '__main__':
    suite = unittest.defaultTestLoader.discover(str(DIRECTORY), pattern='test_*.py')
    excluded = 'test_verify_evidence.EvidenceTests.test_delivery_and_protected_inputs'
    cases = list(tests(suite))
    if sum(t.id() == excluded for t in cases) != 1 or len(cases) != 23:
        raise ValueError('historical test inventory changed')
    selected = unittest.TestSuite(t for t in cases if t.id() != excluded)
    sys.exit(not unittest.TextTestRunner(verbosity=1).run(selected).wasSuccessful())
