"""
Run one unittest module, failing on skips, expected failures, or an empty suite.

Run modules in separate processes: SymForce symbolic backend/epsilon settings are global.
This wrapper leaves optional developer tests optional outside the required CI gate.
"""

import argparse
import importlib.util
import sys
import unittest
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("test_file", type=Path)
    args = parser.parse_args()
    path = args.test_file.resolve()
    if not path.is_file():
        parser.error(f"test module does not exist: {path}")
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    sys.path.insert(0, str(path.parent))
    spec = importlib.util.spec_from_file_location("required_test_module", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load required test module: {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    suite = unittest.defaultTestLoader.loadTestsFromModule(module)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    if (
        result.testsRun == 0
        or result.skipped
        or result.expectedFailures
        or not result.wasSuccessful()
    ):
        print(
            f"Required test gate FAILED: {path.name}: ran={result.testsRun}, "
            f"skipped={len(result.skipped)}, expected_failures={len(result.expectedFailures)}",
            file=sys.stderr,
        )
        return 1
    print(f"Required test gate PASSED: {path.name}: ran={result.testsRun}, skipped=0")
    return 0


if __name__ == "__main__":
    sys.exit(main())
