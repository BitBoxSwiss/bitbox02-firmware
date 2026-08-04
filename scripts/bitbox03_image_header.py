#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Create and update BitBox03 image headers."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, TypedDict, cast

HEADER_LEN = 1024


class HeaderManifest(TypedDict):
    magic: bytes


def _read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as infile:
        return cast(dict[str, Any], json.load(infile))


def _load_header_manifest(path: Path) -> HeaderManifest:
    manifest = _read_json(path)
    magic = manifest.get("magic")
    if not isinstance(magic, str) or len(magic.encode("ascii")) != 4:
        raise ValueError("manifest field 'magic' must be a 4-byte ASCII string")
    return {"magic": magic.encode("ascii")}


def build_header(*, manifest: HeaderManifest, code_size: int) -> bytes:
    """Build an image header."""
    if not 0 <= code_size <= 0xFFFF_FFFF:
        raise ValueError("code_size must be a u32")

    header = bytearray(HEADER_LEN)
    header[0:4] = manifest["magic"]
    header[4:8] = HEADER_LEN.to_bytes(4, "little")
    header[8:12] = code_size.to_bytes(4, "little")
    return bytes(header)


def finalize_header_code_size(header_bytes: bytes, code_size: int) -> bytes:
    if len(header_bytes) != HEADER_LEN:
        raise ValueError("header must be exactly 1024 bytes")
    if not 0 <= code_size <= 0xFFFF_FFFF:
        raise ValueError("code_size must be a u32")
    updated = bytearray(header_bytes)
    updated[8:12] = code_size.to_bytes(4, "little")
    return bytes(updated)


def finalize_code_size(header_bytes: bytes, payload: bytes) -> bytes:
    return finalize_header_code_size(header_bytes, len(payload))


def _read_u16(data: bytes, offset: int) -> int:
    return int.from_bytes(data[offset : offset + 2], "little")


def _read_u32(data: bytes, offset: int) -> int:
    return int.from_bytes(data[offset : offset + 4], "little")


def _validate_elf32_le(data: bytes, elf: Path) -> None:
    if data[:4] != b"\x7fELF":
        raise ValueError(f"{elf} is not an ELF file")
    if data[4] != 1 or data[5] != 1:
        raise ValueError(f"{elf} must be a little-endian ELF32 file")


def _elf_section(elf: Path, section_name: str) -> tuple[int, int, int]:
    data = elf.read_bytes()
    _validate_elf32_le(data, elf)

    section_header_offset = _read_u32(data, 32)
    section_header_size = _read_u16(data, 46)
    section_count = _read_u16(data, 48)
    section_names_index = _read_u16(data, 50)
    if section_header_size < 40:
        raise ValueError(f"{elf} has invalid section header size")
    if section_names_index >= section_count:
        raise ValueError(f"{elf} has invalid section name table index")

    def section_header(index: int) -> bytes:
        start = section_header_offset + index * section_header_size
        end = start + section_header_size
        if end > len(data):
            raise ValueError(f"{elf} section header table is truncated")
        return data[start:end]

    section_names_header = section_header(section_names_index)
    section_names_offset = _read_u32(section_names_header, 16)
    section_names_size = _read_u32(section_names_header, 20)
    section_names = data[section_names_offset : section_names_offset + section_names_size]

    for index in range(section_count):
        header = section_header(index)
        name_offset = _read_u32(header, 0)
        name_end = section_names.find(b"\x00", name_offset)
        if name_end == -1:
            raise ValueError(f"{elf} section name table is truncated")
        name = section_names[name_offset:name_end].decode("ascii")
        if name == section_name:
            return _read_u32(header, 12), _read_u32(header, 16), _read_u32(header, 20)

    raise ValueError(f"{elf} does not contain section {section_name}")


def _elf_flash_payload_len(elf: Path, payload_address: int) -> int:
    data = elf.read_bytes()
    _validate_elf32_le(data, elf)

    program_header_offset = _read_u32(data, 28)
    program_header_size = _read_u16(data, 42)
    program_count = _read_u16(data, 44)
    if program_header_size < 32:
        raise ValueError(f"{elf} has invalid program header size")

    payload_end = payload_address
    for index in range(program_count):
        start = program_header_offset + index * program_header_size
        end = start + program_header_size
        if end > len(data):
            raise ValueError(f"{elf} program header table is truncated")
        header = data[start:end]
        segment_type = _read_u32(header, 0)
        segment_address = _read_u32(header, 12)
        segment_file_size = _read_u32(header, 16)
        if segment_type != 1 or segment_file_size == 0:
            continue
        # RAM LOAD segments must not turn the image into a sparse flash range.
        if (segment_address ^ payload_address) & 0xFF00_0000:
            continue
        segment_end = segment_address + segment_file_size
        if segment_end > payload_address:
            payload_end = max(payload_end, segment_end)

    payload_len = payload_end - payload_address
    if payload_len == 0:
        raise ValueError(f"{elf} does not contain a flash payload after .image_header")
    return payload_len


def cmd_render(args: argparse.Namespace) -> None:
    args.output.write_bytes(
        build_header(manifest=_load_header_manifest(args.manifest), code_size=0)
    )


def cmd_finalize_code_size(args: argparse.Namespace) -> None:
    header = args.header.read_bytes()
    payload = args.payload.read_bytes()
    args.output.write_bytes(finalize_code_size(header, payload))


def cmd_finalize_elf(args: argparse.Namespace) -> None:
    elf = args.elf
    header_address, header_offset, header_size = _elf_section(elf, ".image_header")
    if header_size != HEADER_LEN:
        raise ValueError(
            f"{elf} .image_header must be exactly {HEADER_LEN} bytes, got {header_size}"
        )
    payload_len = _elf_flash_payload_len(elf, header_address + header_size)

    with elf.open("r+b") as outfile:
        outfile.seek(header_offset)
        header_bytes = outfile.read(header_size)
        outfile.seek(header_offset)
        outfile.write(finalize_header_code_size(header_bytes, payload_len))

    print(f"finalized {elf}: code_size={payload_len}")


def build_parser() -> argparse.ArgumentParser:
    """Build the command-line argument parser."""
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    for command in ("render-dev-header", "render-release-header"):
        subparser = subparsers.add_parser(command)
        subparser.add_argument("--manifest", type=Path, required=True)
        subparser.add_argument("--output", type=Path, required=True)

    finalize = subparsers.add_parser("finalize-code-size")
    finalize.add_argument("--header", type=Path, required=True)
    finalize.add_argument("--payload", type=Path, required=True)
    finalize.add_argument("--output", type=Path, required=True)

    finalize_elf = subparsers.add_parser("finalize-elf")
    finalize_elf.add_argument("elf", type=Path)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    if args.command in ("render-dev-header", "render-release-header"):
        cmd_render(args)
    elif args.command == "finalize-code-size":
        cmd_finalize_code_size(args)
    else:
        cmd_finalize_elf(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
