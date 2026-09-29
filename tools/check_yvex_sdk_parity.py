#!/usr/bin/env python3
"""Compose YVEX's canonical remote operation inventory with its SDK projection."""

from __future__ import annotations

import argparse
import json
import pathlib
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[1]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def compare(producer: dict, client: dict) -> None:
    require(producer.get("schema") == "yvex.operator.registry.v1", "YVEX registry schema changed")
    require(client.get("schema") == "platform.sdk.yvex.management.v1", "YVEX SDK projection schema changed")
    require(client.get("producer_request_schema") == "yvex.management.request.v1", "request schema drift")
    require(client.get("producer_response_schema") == "yvex.management.response.v1", "response schema drift")
    require(client.get("posture") == "contract_supported_runtime_not_implied", "evidence posture drift")
    require(client.get("openai_capacity_profile") == "yvex.openai.compat.v3", "YVEX OpenAI profile drift")
    owned = producer["catalogs"]["remote_management_operations"]
    projected = client["operations"]
    require(isinstance(owned, list) and len(owned) == len(set(owned)), "producer operation inventory invalid")
    require(isinstance(projected, list) and len(projected) == len(set(projected)), "SDK operation inventory invalid")
    require(sorted(owned) == sorted(projected), f"YVEX SDK drift: producer={sorted(owned)} SDK={sorted(projected)}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--yvex-root", type=pathlib.Path, required=True)
    args = parser.parse_args()
    producer = json.loads((args.yvex_root / "config/operator/registry.json").read_text(encoding="utf-8"))
    result = subprocess.run(
        ["cargo", "run", "--locked", "--quiet", "-p", "yvex-sdk", "--example", "capabilities"],
        cwd=ROOT, capture_output=True, text=True, check=False,
    )
    require(result.returncode == 0, f"YVEX SDK manifest failed: {result.stderr}")
    client = json.loads(result.stdout)
    compare(producer, client)
    negative = json.loads(json.dumps(producer))
    negative["catalogs"]["remote_management_operations"].append("model.load")
    try:
        compare(negative, client)
    except AssertionError:
        pass
    else:
        raise AssertionError("controlled producer delta did not fail parity")
    print(f"PASS YVEX→SDK parity: {len(client['operations'])} exact management operations; added-operation negative refused")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AssertionError, KeyError, OSError, ValueError) as error:
        print(f"FAIL YVEX→SDK parity: {error}", file=sys.stderr)
        raise SystemExit(1) from error
