"""Independent product version projections must not drift from Cargo.toml."""
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

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
