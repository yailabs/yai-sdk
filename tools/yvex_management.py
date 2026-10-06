#!/usr/bin/env python3
"""Validate compatibility imports; YVEX owns generation of its public contract."""
from pathlib import Path
from yvex_owner import yvex_root
ROOT=Path(__file__).resolve().parents[1]
owner=yvex_root()
assert (owner/'contract/management.json').is_file()
assert (ROOT/'typescript/yvex.ts').read_text().strip().endswith('export type * from "@yvex/sdk";')
assert 'pub use yvex_client::*;' in (ROOT/'crates/yvex-sdk/src/lib.rs').read_text()
assert not (ROOT/'crates/yvex-sdk/contract/management.json').exists()
print('PASS: canonical YVEX owner and compatibility imports; no duplicate DTO generator')
