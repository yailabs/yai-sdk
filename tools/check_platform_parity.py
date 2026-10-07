#!/usr/bin/env python3
"""Compose owner-published inventories; never infer runtime support from type presence."""

from __future__ import annotations

import argparse
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import tempfile
import tomllib

from check_yvex_sdk_parity import compare as compare_yvex
from check_yvex_sdk_parity import compare_finite_schemas, compare_product_schemas

SDK_ROOT = Path(__file__).resolve().parents[1]


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def run_json(command: list[str], cwd: Path, env: dict[str, str] | None = None) -> dict:
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, check=False)
    if result.returncode:
        raise ValueError(f"{command[0]} failed ({result.returncode}): {result.stderr[:600]}")
    return json.loads(result.stdout)


def compare_yai(catalog: dict, sdk: dict) -> None:
    if catalog.get("application_protocol") != "yai.studio.application.v1":
        raise ValueError("YAI Application protocol drift")
    if sdk.get("schema") != "yai.application_capability_catalog.v1":
        raise ValueError("YAI SDK operation inventory schema drift")
    owner = {row["operation_id"]: {k: v for k, v in row.items() if k != "schema"}
             for row in catalog["operations"]}
    projection = {row["operation_id"]: row for row in sdk["operations"]}
    if len(owner) != len(catalog["operations"]) or len(projection) != len(sdk["operations"]):
        raise ValueError("duplicate YAI operation identity")
    if owner != projection:
        missing = sorted(owner.keys() - projection.keys())
        extra = sorted(projection.keys() - owner.keys())
        changed = sorted(key for key in owner.keys() & projection.keys() if owner[key] != projection[key])
        raise ValueError(f"YAI→SDK contract drift: missing={missing} extra={extra} changed={changed}")


def compare_studio_yvex(client: dict, studio: dict) -> None:
    native = studio["yvex_client"]
    repository = native.get("sdk_repository", "https://github.com/yailabs/yai-sdk.git")
    if repository == "https://github.com/yailabs/yai-sdk.git":
        if native["sdk_revision"] != studio["sdk"]["revision"]:
            raise ValueError("Studio legacy YVEX/YAI SDK revisions diverge")
    elif repository == "https://github.com/yailabs/yvex.git":
        if not re.fullmatch(r"[0-9a-f]{40}", native.get("sdk_revision", "")):
            raise ValueError("Studio canonical YVEX revision is not exact")
        # Independent published clients may have different compatible pins.
        # The schemas/operation dispositions below still compare exactly.
    else:
        raise ValueError("Studio YVEX SDK repository is unsupported")
    if native["management_request_schema"] != client["producer_request_schema"] or \
       native["management_response_schema"] != client["producer_response_schema"]:
        raise ValueError("Studio YVEX contract schema drift")
    if set(native["operation_dispositions"]) != set(client["operations"]):
        raise ValueError("YVEX SDK→Studio operation disposition gap")
    for operation, disposition in native["operation_dispositions"].items():
        if disposition["state"] not in {"supported", "deferred", "not_ui_relevant"}:
            raise ValueError(f"Studio YVEX disposition invalid: {operation}")
        if disposition["state"] == "supported" and not disposition.get("ui_evidence"):
            raise ValueError(f"Studio YVEX support lacks UI evidence: {operation}")

    product = client.get("product_management")
    if product:
        projected = native.get("product_management", {})
        if projected.get("request_schema") != product["request_schema"] or projected.get("response_schema") != product["response_schema"]:
            raise ValueError("Studio product management schema drift")
        dispositions = projected.get("operation_dispositions", {})
        if set(dispositions) != {row["operation"] for row in product["operations"]}:
            raise ValueError("YVEX product management Studio disposition gap")
        for operation, disposition in dispositions.items():
            if disposition.get("state") not in {"supported", "deferred", "not_ui_relevant"}:
                raise ValueError(f"Studio product management disposition invalid: {operation}")
            if disposition["state"] == "supported" and not disposition.get("ui_evidence"):
                raise ValueError(f"Studio product management support lacks evidence: {operation}")


def compare_studio_yvex_pin(studio: dict, manifest: dict, package: dict) -> None:
    """Check both native/TS consumer pins, not an unrelated YAI repository SHA."""
    native = studio["yvex_client"]
    if native.get("sdk_repository") != "https://github.com/yailabs/yvex.git":
        return
    dependency = manifest.get("dependencies", {}).get("yvex-sdk", {})
    if not isinstance(dependency, dict) or dependency.get("git") != native["sdk_repository"] or \
       dependency.get("rev") != native["sdk_revision"]:
        raise ValueError("Studio canonical YVEX native pin diverges from component")
    expected = f"git+{native['sdk_repository']}#{native['sdk_revision']}"
    if package.get("dependencies", {}).get("@yvex/sdk") != expected:
        raise ValueError("Studio canonical YVEX TypeScript pin diverges from component")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--yai-bin", type=Path, required=True)
    parser.add_argument("--studio-root", type=Path, required=True)
    parser.add_argument("--yvex-root", type=Path, required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="platform-parity-yai-") as isolated_home:
        env = {**os.environ, "YAI_HOME": isolated_home}
        yai_result = run_json([str(args.yai_bin.resolve()), "capabilities", "--json"],
                              args.yai_bin.resolve().parent, env)
    if yai_result.get("status") != "ok":
        raise ValueError("YAI capability producer refused")
    yai = yai_result["data"]["value"]
    sdk_yai = read_json(SDK_ROOT / "contract/operations.json")
    compare_yai(yai, sdk_yai)

    yvex = read_json(args.yvex_root / "config/operator/registry.json")
    sdk_yvex = run_json(["cargo", "run", "--locked", "--quiet", "-p", "yvex-sdk@0.1.0",
                         "--example", "capabilities"], SDK_ROOT)
    compare_yvex(yvex, sdk_yvex)
    compare_finite_schemas(args.yvex_root)
    compare_product_schemas(args.yvex_root)
    studio = read_json(args.studio_root / "component.json")
    compare_studio_yvex(sdk_yvex, studio)
    if studio["yvex_client"].get("sdk_repository") == "https://github.com/yailabs/yvex.git":
        compare_studio_yvex_pin(studio,
            tomllib.loads((args.studio_root / "src-tauri/Cargo.toml").read_text()),
            read_json(args.studio_root / "package.json"))

    studio_env = {**os.environ, "YAI_STUDIO_TEST_BINARY": str(args.yai_bin.resolve())}
    check = subprocess.run(["node", "tests/studio/capability-parity.mjs"],
                           cwd=args.studio_root, env=studio_env, capture_output=True, text=True)
    if check.returncode:
        raise ValueError(f"YAI→Studio disposition drift: {check.stderr[:700]}")

    changed = json.loads(json.dumps(yvex))
    changed["catalogs"]["remote_management_operations"].append("model.load")
    try:
        compare_yvex(changed, sdk_yvex)
    except AssertionError:
        pass
    else:
        raise ValueError("YVEX producer delta negative did not fail")
    changed_yai = json.loads(json.dumps(yai))
    changed_yai["operations"][0]["output_contract"] = "changed.contract"
    try:
        compare_yai(changed_yai, sdk_yai)
    except ValueError:
        pass
    else:
        raise ValueError("YAI producer delta negative did not fail")
    print(json.dumps({"result": "PASS", "yai_operations": len(yai["operations"]),
                      "yvex_remote_operations": len(sdk_yvex["operations"]),
                      "studio_yvex_dispositions": len(studio["yvex_client"]["operation_dispositions"]),
                      "evidence": "contract_and_disposition_only_runtime_not_implied",
                      "negative_deltas": ["YAI contract", "YVEX remote operation"]}))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AssertionError, KeyError, OSError, ValueError) as error:
        print(f"FAIL platform parity: {error}", file=sys.stderr)
        raise SystemExit(1) from error
