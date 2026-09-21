"""Negative controls for the fail-closed required-test runner (stdlib only)."""

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

RUNNER = Path(__file__).resolve().parents[1] / "tools" / "run_required_test.py"


class RequiredTestRunnerTest(unittest.TestCase):
    def run_module(self, source: str) -> subprocess.CompletedProcess:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "fixture_test.py"
            path.write_text(source)
            return subprocess.run(
                [sys.executable, str(RUNNER), str(path)],
                capture_output=True,
                text=True,
                check=False,
            )

    def test_passing_suite(self) -> None:
        result = self.run_module(
            "import unittest\nclass Test(unittest.TestCase):\n"
            "    def test_ok(self): self.assertEqual(2 + 2, 4)\n"
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_empty_suite_is_failure(self) -> None:
        self.assertNotEqual(self.run_module("import unittest\n").returncode, 0)

    def test_skip_is_failure_even_when_another_test_passes(self) -> None:
        result = self.run_module(
            "import unittest\nclass Test(unittest.TestCase):\n"
            "    def test_ok(self): pass\n"
            "    @unittest.skip('missing dependency')\n"
            "    def test_skipped(self): pass\n"
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("skipped=1", result.stderr)

    def test_expected_failure_is_failure(self) -> None:
        result = self.run_module(
            "import unittest\nclass Test(unittest.TestCase):\n"
            "    @unittest.expectedFailure\n"
            "    def test_expected(self): self.fail('not implemented')\n"
        )
        self.assertNotEqual(result.returncode, 0)

    def test_assertion_and_import_failures(self) -> None:
        for source in (
            "raise RuntimeError('broken import')\n",
            "import unittest\nclass Test(unittest.TestCase):\n"
            "    def test_failure(self): self.fail('broken contract')\n",
        ):
            with self.subTest(source=source):
                self.assertNotEqual(self.run_module(source).returncode, 0)


if __name__ == "__main__":
    unittest.main()
