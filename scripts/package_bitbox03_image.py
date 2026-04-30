#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Combine a BitBox03 payload with a finalized release header."""

from __future__ import annotations

import argparse
from pathlib import Path

from bitbox03_image_header import (
    HEADER_LEN,
    build_header,
    _load_header_manifest,
    _marketing_version,
)


def _extract_payload(image_bytes: bytes, magic: bytes) -> bytes:
    if (
        len(image_bytes) >= HEADER_LEN
        and image_bytes[:4] == magic
        and int.from_bytes(image_bytes[4:8], "little") == HEADER_LEN
    ):
        return image_bytes[HEADER_LEN:]
    return image_bytes


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--versions-manifest", type=Path, required=True)
    parser.add_argument("--version-key", choices=("bootloader", "firmware"), required=True)
    parser.add_argument("--monotonic-version", type=int, required=True)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    manifest = _load_header_manifest(args.manifest)
    payload = _extract_payload(args.input.read_bytes(), manifest["magic"])
    header = build_header(
        manifest=manifest,
        marketing_version=_marketing_version(args.versions_manifest, args.version_key),
        monotonic_version=args.monotonic_version,
        code_size=len(payload),
    )
    args.output.write_bytes(header + payload)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
