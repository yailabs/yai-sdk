"""Locate the exact Cargo-resolved public YVEX owner, never a sibling guess."""
from functools import lru_cache
import json
from pathlib import Path
import subprocess
ROOT=Path(__file__).resolve().parents[1]
@lru_cache(maxsize=1)
def yvex_root():
    data=json.loads(subprocess.check_output(['cargo','metadata','--format-version=1','--locked'],cwd=ROOT,text=True))
    candidates=[p for p in data['packages'] if p['name']=='yvex-sdk' and p['version']=='0.2.0']
    if len(candidates)!=1:raise RuntimeError('Canonical YVEX client dependency missing or ambiguous')
    return Path(candidates[0]['manifest_path']).parent
