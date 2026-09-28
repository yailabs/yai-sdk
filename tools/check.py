#!/usr/bin/env python3
"""Standalone package identity and private-boundary guard."""
import json
from pathlib import Path
import re
import tomllib

root = Path(__file__).resolve().parents[1]
manifest = tomllib.loads((root / "Cargo.toml").read_text())
version = manifest["package"]["version"]
assert re.fullmatch(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?", version)
assert manifest["package"]["license"] == "MIT"
package = json.loads((root / "package.json").read_text())
assert package["version"] == version
lock = json.loads((root / "package-lock.json").read_text())
assert lock["version"] == version and lock["packages"][""]["version"] == version
for dependency in manifest["dependencies"].values():
    assert not isinstance(dependency, dict) or "path" not in dependency
for source in (root / "src").glob("*.rs"):
    text = source.read_text()
    assert not re.search(r"(?:use|extern crate)\s+yai_(?:application|host|core_engine)", text), source
assert not (root / "studio").exists()
print(f"PASS: independent SDK {version}, MIT, synchronized package identity, no private imports")
