#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Create and update BitBox03 image headers."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

HEADER_LEN = 1024
MARKETING_VERSION_LEN = 32
KEY_COUNT = 3
KEY_LEN = 64
INVALID_CODE_SIZE = 0xFFFF_FFFF


def _read_json(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as infile:
        return json.load(infile)


def _encode_marketing_version(version: str) -> bytes:
    encoded = version.encode("ascii")
    if len(encoded) > MARKETING_VERSION_LEN:
        raise ValueError("marketing version too long")
    return encoded.ljust(MARKETING_VERSION_LEN, b"\x00")


def _decode_hex_entries(entries: list[str], field_name: str) -> bytes:
    if len(entries) != KEY_COUNT:
        raise ValueError(f"{field_name} must contain exactly {KEY_COUNT} entries")
    output = bytearray()
    for index, entry in enumerate(entries):
        try:
            decoded = bytes.fromhex(entry)
        except ValueError as exc:
            raise ValueError(f"{field_name}[{index}] is not valid hex") from exc
        if len(decoded) != KEY_LEN:
            raise ValueError(f"{field_name}[{index}] must be {KEY_LEN} bytes")
        output.extend(decoded)
    return bytes(output)


def _load_header_manifest(path: Path) -> dict:
    manifest = _read_json(path)
    magic = manifest.get("magic")
    if not isinstance(magic, str) or len(magic.encode("ascii")) != 4:
        raise ValueError("manifest field 'magic' must be a 4-byte ASCII string")
    product = manifest.get("product")
    if not isinstance(product, int) or not 0 <= product <= 0xFFFF_FFFF:
        raise ValueError("manifest field 'product' must be a u32")
    return {
        "magic": magic.encode("ascii"),
        "product": product,
        "signatures": _decode_hex_entries(manifest.get("signatures", []), "signatures"),
    }
def _marketing_version(versions_manifest: Path, version_key: str) -> str:
    versions = _read_json(versions_manifest)
    version = versions.get(version_key)
    if not isinstance(version, str):
        raise ValueError(f"versions manifest does not contain string key '{version_key}'")
    return version


def build_header(
    *,
    manifest: dict,
    marketing_version: str,
    monotonic_version: int,
    code_size: int,
) -> bytes:
    if not 0 <= monotonic_version <= 0xFFFF_FFFF:
        raise ValueError("monotonic_version must be a u32")
    if not 0 <= code_size <= 0xFFFF_FFFF:
        raise ValueError("code_size must be a u32")

    header = bytearray()
    header.extend(manifest["magic"])
    header.extend(HEADER_LEN.to_bytes(4, "little"))
    header.extend(code_size.to_bytes(4, "little"))
    header.extend(manifest["product"].to_bytes(4, "little"))
    header.extend(_encode_marketing_version(marketing_version))
    header.extend(monotonic_version.to_bytes(4, "little"))
    header.extend(manifest["signatures"])
    if len(header) > HEADER_LEN:
        raise ValueError("header fields exceed 1024 bytes")
    header.extend(b"\x00" * (HEADER_LEN - len(header)))
    return bytes(header)
def finalize_code_size(header_bytes: bytes, payload: bytes) -> bytes:
    if len(header_bytes) != HEADER_LEN:
        raise ValueError("header must be exactly 1024 bytes")
    updated = bytearray(header_bytes)
    updated[8:12] = len(payload).to_bytes(4, "little")
    return bytes(updated)


def cmd_render(args: argparse.Namespace, code_size: int) -> None:
    manifest = _load_header_manifest(args.manifest)
    header = build_header(
        manifest=manifest,
        marketing_version=_marketing_version(args.versions_manifest, args.version_key),
        monotonic_version=args.monotonic_version,
        code_size=code_size,
    )
    args.output.write_bytes(header)


def cmd_finalize_code_size(args: argparse.Namespace) -> None:
    header = args.header.read_bytes()
    payload = args.payload.read_bytes()
    args.output.write_bytes(finalize_code_size(header, payload))

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    for command in ("render-dev-header", "render-release-header"):
        subparser = subparsers.add_parser(command)
        subparser.add_argument("--manifest", type=Path, required=True)
        subparser.add_argument("--versions-manifest", type=Path, required=True)
        subparser.add_argument("--version-key", choices=("bootloader", "firmware"), required=True)
        subparser.add_argument("--monotonic-version", type=int, required=True)
        subparser.add_argument("--output", type=Path, required=True)

    finalize = subparsers.add_parser("finalize-code-size")
    finalize.add_argument("--header", type=Path, required=True)
    finalize.add_argument("--payload", type=Path, required=True)
    finalize.add_argument("--output", type=Path, required=True)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    if args.command == "render-dev-header":
        cmd_render(args, INVALID_CODE_SIZE)
    elif args.command == "render-release-header":
        cmd_render(args, 0)
    else:
        cmd_finalize_code_size(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
