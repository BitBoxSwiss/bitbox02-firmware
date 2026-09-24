#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Create and update headers in the BitBox image format.

Manifests specify magic (BBS1 for stage1, BBFW for firmware), flags, product_id,
monotonic_version, and marketing_version. Header format version, lengths,
reserved bytes, and empty signature slots are derived from the binary format.
The image length includes the header.

Use finalize-elf --section .stage1_header for existing BitBox02 stage1 ELFs.
"""

from __future__ import annotations

import argparse
import json
import struct
from pathlib import Path
from typing import Any, TypedDict, cast

# Must stay in sync with IMAGE_HEADER_LEN in src/rust/bitbox-boot-utils/src/image_header.rs.
HEADER_LEN = 1024

# Must stay in sync with src/bootloader_upgrade/bootloader_upgrade.h and
# src/bootloader/bootloader_product.h.
STAGE1_MAGIC = b"BBS1"
FIRMWARE_MAGIC = b"BBFW"
HEADER_VERSION = 1
BITBOX02_STAGE1_MAX_LEN = 0xBFE0
MARKETING_VERSION_MAX_LEN = 37
HEADER_PREFIX = struct.Struct("<4sIHHIQHB37s")
SIGNATURES_OFFSET = 832


class HeaderManifest(TypedDict):
    magic: bytes

    flags: int
    product_id: int
    monotonic_version: int
    marketing_version: str


def _manifest_int(manifest: dict[str, Any], name: str, minimum: int, maximum: int) -> int:
    value = manifest.get(name)
    if type(value) is not int or not minimum <= value <= maximum:
        raise ValueError(f"manifest field '{name}' must be an integer in {minimum}..{maximum}")
    return value


def _marketing_version(value: Any) -> bytes:
    if not isinstance(value, str):
        raise ValueError("manifest field 'marketing_version' must be a string")
    version = value.encode("ascii")
    if not 1 <= len(version) <= MARKETING_VERSION_MAX_LEN or any(
        char < 0x21 or char > 0x7E for char in version
    ):
        raise ValueError("marketing version must contain 1..37 printable non-space ASCII bytes")
    return version


def _load_header_manifest(path: Path) -> HeaderManifest:
    with path.open("r", encoding="utf-8") as infile:
        manifest = json.load(infile)
    if not isinstance(manifest, dict):
        raise ValueError("header manifest must be an object")
    magic = manifest.get("magic")
    if not isinstance(magic, str) or len(magic.encode("ascii")) != 4:
        raise ValueError("manifest field 'magic' must be a 4-byte ASCII string")
    return {
        "magic": magic.encode("ascii"),
        "flags": _manifest_int(manifest, "flags", 0, 1),
        "product_id": _manifest_int(manifest, "product_id", 0, 0xFFFF),
        "monotonic_version": _manifest_int(manifest, "monotonic_version", 0, 0xFFFF),
        "marketing_version": _marketing_version(manifest.get("marketing_version")).decode("ascii"),
    }


def build_header(*, manifest: HeaderManifest, code_size: int) -> bytes:
    """Build an unsigned header; a zero code size leaves image_len unset."""
    if manifest["magic"] not in (STAGE1_MAGIC, FIRMWARE_MAGIC):
        raise ValueError("unsupported image header magic")
    if not 0 <= code_size <= 0xFFFF_FFFF_FFFF_FFFF - HEADER_LEN:
        raise ValueError("image length must fit in a u64")
    fields = cast(dict[str, Any], manifest)
    version = _marketing_version(manifest.get("marketing_version"))
    header = bytearray(HEADER_LEN)
    HEADER_PREFIX.pack_into(
        header,
        0,
        manifest["magic"],
        _manifest_int(fields, "flags", 0, 1),
        HEADER_VERSION,
        _manifest_int(fields, "product_id", 0, 0xFFFF),
        HEADER_LEN,
        0,  # The raw linked image has not been sized yet.
        _manifest_int(fields, "monotonic_version", 0, 0xFFFF),
        len(version),
        version,
    )
    if code_size:
        return finalize_header_code_size(bytes(header), code_size)
    return bytes(header)


def finalize_header_code_size(header_bytes: bytes, code_size: int) -> bytes:
    """Fill the total image length in an unsigned header from its payload size."""
    if len(header_bytes) != HEADER_LEN:
        raise ValueError("header must be exactly 1024 bytes")
    image_len = HEADER_LEN + code_size
    if not HEADER_LEN < image_len <= 0xFFFF_FFFF_FFFF_FFFF:
        raise ValueError("image length is invalid")
    (
        magic,
        flags,
        header_version,
        product_id,
        header_len,
        previous_image_len,
        monotonic_version,
        version_len,
        version_field,
    ) = HEADER_PREFIX.unpack_from(header_bytes)
    # BitBox02 stage1 must end before the factory randomness. BitBox03 uses
    # its board-specific flash slots, which are enforced by the linker/loader.
    if magic == STAGE1_MAGIC and product_id in (1, 2, 3, 4) and image_len > BITBOX02_STAGE1_MAX_LEN:
        raise ValueError("BitBox02 stage1 image length is invalid")
    manifest: HeaderManifest = {
        "magic": magic,
        "flags": flags,
        "product_id": product_id,
        "monotonic_version": monotonic_version,
        "marketing_version": version_field[:version_len].decode("ascii"),
    }
    if header_version != HEADER_VERSION or header_len != HEADER_LEN:
        raise ValueError("invalid header version or length")
    if previous_image_len not in (0, image_len):
        raise ValueError("image length does not match payload")
    if any(header_bytes[SIGNATURES_OFFSET:]):
        raise ValueError("header signatures are not zero")
    updated = bytearray(header_bytes)
    updated[16:24] = bytes(8)
    if bytes(updated) != build_header(manifest=manifest, code_size=0):
        raise ValueError("invalid header padding or marketing version length")
    updated[16:24] = image_len.to_bytes(8, "little")
    return bytes(updated)


def _read_u16(data: bytes, offset: int) -> int:
    return int.from_bytes(data[offset : offset + 2], "little")


def _read_u32(data: bytes, offset: int) -> int:
    return int.from_bytes(data[offset : offset + 4], "little")


def _validate_elf32_le(data: bytes, elf: Path) -> None:
    if data[:4] != b"\x7fELF":
        raise ValueError(f"{elf} is not an ELF file")
    if len(data) < 52 or data[4] != 1 or data[5] != 1:
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
        raise ValueError(f"{elf} does not contain a flash payload after the image header")
    return payload_len


def cmd_render(args: argparse.Namespace) -> None:
    args.output.write_bytes(
        build_header(manifest=_load_header_manifest(args.manifest), code_size=0)
    )


def cmd_finalize_code_size(args: argparse.Namespace) -> None:
    header = args.header.read_bytes()
    payload = args.payload.read_bytes()
    args.output.write_bytes(finalize_header_code_size(header, len(payload)))


def cmd_finalize_elf(args: argparse.Namespace) -> None:
    elf = args.elf
    header_address, header_offset, header_size = _elf_section(elf, args.section)
    if header_size != HEADER_LEN:
        raise ValueError(
            f"{elf} {args.section} must be exactly {HEADER_LEN} bytes, got {header_size}"
        )
    payload_len = _elf_flash_payload_len(elf, header_address + header_size)

    with elf.open("r+b") as outfile:
        outfile.seek(header_offset)
        header_bytes = outfile.read(header_size)
        outfile.seek(header_offset)
        outfile.write(finalize_header_code_size(header_bytes, payload_len))

    print(f"finalized {elf}: image_len={header_size + payload_len}")


def build_parser() -> argparse.ArgumentParser:
    """Build the command-line argument parser."""
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    render = subparsers.add_parser("render-header")
    render.add_argument("--manifest", type=Path, required=True)
    render.add_argument("--output", type=Path, required=True)

    finalize = subparsers.add_parser("finalize-code-size")
    finalize.add_argument("--header", type=Path, required=True)
    finalize.add_argument("--payload", type=Path, required=True)
    finalize.add_argument("--output", type=Path, required=True)

    finalize_elf = subparsers.add_parser("finalize-elf")
    finalize_elf.add_argument("elf", type=Path)
    finalize_elf.add_argument("--section", default=".image_header", help="ELF header section name")
    return parser


def main() -> int:
    args = build_parser().parse_args()
    if args.command == "render-header":
        cmd_render(args)
    elif args.command == "finalize-code-size":
        cmd_finalize_code_size(args)
    else:
        cmd_finalize_elf(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
