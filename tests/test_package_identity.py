"""Independent product version projections must not drift from Cargo.toml."""
from pathlib import Path
import ast
import shutil
import subprocess
import sys
import tempfile
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))
from check_platform_parity import compare_studio_yvex, compare_studio_yvex_pin

ROOT = Path(__file__).resolve().parents[1]


class PackageIdentity(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for name in ("Cargo.toml", "Cargo.lock", "package.json", "package-lock.json", "tools/check.py"):
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / name, target)

    def check(self):
        return subprocess.run([sys.executable, str(self.root / "tools/check.py")],
                              capture_output=True, text=True).returncode

    def test_current_projection(self):
        self.assertEqual(self.check(), 0)

    def test_cargo_selectors_distinguish_facade_from_canonical_dependency(self):
        # Both public packages intentionally share a name. Never let a newly
        # added unversioned selector silently disable a qualification lane.
        expected = {
            "tools/check_platform_parity.py": ["yvex-sdk@0.1.0"],
            "tests/test_finite_remote.py": ["yvex-sdk@0.1.0"],
            "tests/test_management_remote.py": ["yvex-sdk@0.1.0"],
            "tests/test_finite_native.py": ["yvex-sdk@0.1.0", "yvex-sdk@0.1.0"],
        }
        for path, packages in expected.items():
            selectors = []
            for node in ast.walk(ast.parse((ROOT / path).read_text())):
                if isinstance(node, (ast.List, ast.Tuple)):
                    for index, item in enumerate(node.elts[:-1]):
                        if isinstance(item, ast.Constant) and item.value == "-p":
                            following = node.elts[index + 1]
                            self.assertIsInstance(following, ast.Constant, path)
                            selectors.append(following.value)
            self.assertCountEqual(selectors, packages, path)

    def test_independent_studio_pins_preserve_contract_and_native_typescript_identity(self):
        import copy
        client = {"producer_request_schema":"request.v1", "producer_response_schema":"response.v1", "operations":[]}
        studio = {"sdk":{"revision":"a"*40}, "yvex_client":{
            "sdk_repository":"https://github.com/yailabs/yvex.git", "sdk_revision":"b"*40,
            "management_request_schema":"request.v1", "management_response_schema":"response.v1",
            "operation_dispositions":{}}}
        manifest = {"dependencies":{"yvex-sdk":{"git":"https://github.com/yailabs/yvex.git", "rev":"b"*40}}}
        package = {"dependencies":{"@yvex/sdk":"git+https://github.com/yailabs/yvex.git#"+"b"*40}}
        compare_studio_yvex(client, studio)
        compare_studio_yvex_pin(studio, manifest, package)
        for field, value in [("sdk_repository","https://unapproved.example/sdk.git"),
                             ("sdk_revision","main"), ("management_response_schema","foreign")]:
            changed = copy.deepcopy(studio); changed["yvex_client"][field] = value
            with self.assertRaises(ValueError): compare_studio_yvex(client, changed)
        changed = copy.deepcopy(manifest); changed["dependencies"]["yvex-sdk"]["rev"] = "c"*40
        with self.assertRaises(ValueError): compare_studio_yvex_pin(studio, changed, package)
        with self.assertRaises(ValueError): compare_studio_yvex_pin(studio, manifest, {"dependencies":{}})
        legacy = copy.deepcopy(studio); del legacy["yvex_client"]["sdk_repository"]
        with self.assertRaises(ValueError): compare_studio_yvex(client, legacy)
        legacy["yvex_client"]["sdk_revision"] = legacy["sdk"]["revision"]
        compare_studio_yvex(client, legacy)

    def test_each_projection_drift_refuses(self):
        for name in ("Cargo.lock", "package.json", "package-lock.json"):
            with self.subTest(name=name):
                path = self.root / name
                original = path.read_text()
                path.write_text(original.replace('"0.1.0"', '"0.2.0"'))
                self.assertNotEqual(self.check(), 0)
                path.write_text(original)

    def test_invalid_prerelease_refuses_even_when_projections_agree(self):
        for value in ("0.1.0-01", "0.1.0-rc.01", "0.1.0-rc..1", "0.1.0+build..1"):
            originals = {}
            for name in ("Cargo.toml", "Cargo.lock", "package.json", "package-lock.json"):
                path = self.root / name
                originals[path] = path.read_text()
                path.write_text(originals[path].replace('"0.1.0"', f'"{value}"'))
            self.assertNotEqual(self.check(), 0, value)
            for path, original in originals.items():
                path.write_text(original)

    def test_standard_prerelease_is_valid(self):
        for name in ("Cargo.toml", "Cargo.lock", "package.json", "package-lock.json"):
            path = self.root / name
            path.write_text(path.read_text().replace('"0.1.0"', '"0.1.0-rc.1+build.01"'))
        self.assertEqual(self.check(), 0)


if __name__ == "__main__":
    unittest.main()
