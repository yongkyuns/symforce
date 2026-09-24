# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Fast failure-handling contracts; real generation executes in the geometry suite."""

import hashlib
import json
import tempfile
from pathlib import Path
from unittest import mock

import symforce

symforce.set_epsilon_to_symbol()

from symforce import typing as T
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.slam.imu_preintegration import generate
from symforce.slam.imu_preintegration.generate_rust import generate_rust_imu_package
from symforce.test_util import TestCase

# Independent enumeration catches accidental changes to the generator's output contract.
FUNCTIONS = (
    "imu_manifold_preintegration_update",
    "imu_manifold_preintegration_update_auto_derivative",
    "internal_imu_factor",
    "internal_imu_unit_gravity_factor",
    "internal_imu_with_gravity_factor",
    "roll_forward_state",
)


class RustImuGenerationTest(TestCase):
    def emit_fixture(self, config: RustConfig, directory: Path) -> None:
        self.assertIs(config.algebra, RustAlgebra.STACK_ALGEBRA)
        self.assertIs(config.scalar_type, ScalarType.GENERIC)
        self.assertFalse(config.normalize_results)
        self.assertTrue(config.inline)
        self.assertEqual(config.geometry_crate, "geometry-runtime")
        for name in FUNCTIONS:
            (directory / f"{name}.rs").write_text(f"// fixture {name}\n", encoding="utf-8")

    def setUp(self) -> None:
        super().setUp()
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.output = Path(directory.name) / "package"
        patcher = mock.patch.object(
            generate, "generate_manifold_imu_preintegration", self.emit_fixture
        )
        patcher.start()
        self.addCleanup(patcher.stop)

    def generate_package(self, *, check: bool = False) -> None:
        generate_rust_imu_package(self.output, check=check, geometry_crate="geometry-runtime")

    def snapshot(self) -> T.Dict[str, T.Tuple[bytes, int]]:
        return {
            path.name: (path.read_bytes(), path.stat().st_mtime_ns)
            for path in self.output.iterdir()
        }

    def test_manifest_and_read_only_check(self) -> None:
        self.generate_package()
        before = self.snapshot()
        self.generate_package(check=True)
        self.assertEqual(self.snapshot(), before)
        manifest = json.loads((self.output / "manifest.json").read_text())
        self.assertEqual(manifest["symbolic_api"], symforce.get_symbolic_api())
        self.assertEqual(set(manifest["files"]), {f"{name}.rs" for name in FUNCTIONS})
        for name, digest in manifest["files"].items():
            self.assertEqual(hashlib.sha256((self.output / name).read_bytes()).hexdigest(), digest)

    def test_existing_destination_is_not_overwritten(self) -> None:
        self.output.mkdir()
        (self.output / "unrelated.txt").write_text("keep", encoding="utf-8")
        before = self.snapshot()
        with self.assertRaises(FileExistsError):
            self.generate_package()
        self.assertEqual(self.snapshot(), before)

    def test_check_requires_existing_directory(self) -> None:
        with self.assertRaisesRegex(ValueError, "existing package directory"):
            self.generate_package(check=True)
        self.assertFalse(self.output.exists())

    def test_corruption_and_inventory_changes_fail_without_writes(self) -> None:
        self.generate_package()
        original = self.snapshot()
        for name, content in (
            (f"{FUNCTIONS[0]}.rs", b"corrupted"),
            ("manifest.json", b"{}"),
            ("extra.rs", b"unexpected"),
            (f"{FUNCTIONS[1]}.rs", None),
        ):
            with self.subTest(name=name):
                path = self.output / name
                if content is None:
                    path.unlink()
                else:
                    path.write_bytes(content)
                before = self.snapshot()
                with self.assertRaisesRegex(ValueError, "regeneration differs"):
                    self.generate_package(check=True)
                self.assertEqual(self.snapshot(), before)
                if name in original:
                    path.write_bytes(original[name][0])
                else:
                    path.unlink()

    def test_self_consistent_modified_package_is_rejected(self) -> None:
        self.generate_package()
        name = f"{FUNCTIONS[0]}.rs"
        (self.output / name).write_bytes(b"modified")
        path = self.output / "manifest.json"
        manifest = json.loads(path.read_text())
        manifest["files"][name] = hashlib.sha256(b"modified").hexdigest()
        path.write_text(json.dumps(manifest), encoding="utf-8")
        before = self.snapshot()
        with self.assertRaisesRegex(ValueError, "regeneration differs"):
            self.generate_package(check=True)
        self.assertEqual(self.snapshot(), before)

    def test_generator_failure_does_not_publish_partial_output(self) -> None:
        with (
            mock.patch.object(
                generate, "generate_manifold_imu_preintegration", side_effect=RuntimeError("failed")
            ),
            self.assertRaisesRegex(RuntimeError, "failed"),
        ):
            self.generate_package()
        self.assertFalse(self.output.exists())

    def test_incomplete_generation_is_rejected(self) -> None:
        with (
            mock.patch.object(generate, "generate_manifold_imu_preintegration"),
            self.assertRaisesRegex(ValueError, "exactly the six"),
        ):
            self.generate_package()
        self.assertFalse(self.output.exists())

    def test_unexpected_generator_output_is_rejected(self) -> None:
        def extra_output(config: RustConfig, directory: Path) -> None:
            self.emit_fixture(config, directory)
            (directory / "unexpected.rs").write_text("extra", encoding="utf-8")

        with (
            mock.patch.object(
                generate, "generate_manifold_imu_preintegration", side_effect=extra_output
            ),
            self.assertRaisesRegex(ValueError, "exactly the six"),
        ):
            self.generate_package()
        self.assertFalse(self.output.exists())

    def test_symlink_output_is_rejected(self) -> None:
        self.generate_package()
        before = self.snapshot()
        link = self.output.parent / "link"
        link.symlink_to(self.output, target_is_directory=True)
        for check in (False, True):
            with self.subTest(check=check):
                with self.assertRaisesRegex(ValueError, "symbolic-link"):
                    generate_rust_imu_package(link, check=check)
        self.assertEqual(self.snapshot(), before)


if __name__ == "__main__":
    RustImuGenerationTest.main()
