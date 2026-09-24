# SPDX-License-Identifier: Apache-2.0

"""Compatibility tests for scripts/image_header.py."""

import argparse
import contextlib
import io
import json
import struct
import tempfile
import unittest
from pathlib import Path

from scripts import bootloader_update, image_header


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]


def stage1_manifest(**overrides: object) -> dict:
    """Return the metadata for a development stage1 image."""
    return {
        "magic": "BBS1",
        "flags": 1,
        "product_id": 1,
        "monotonic_version": 1,
        "marketing_version": "v1.2.2+dev",
        **overrides,
    }


def make_elf(header: bytes, section: str, flash_address: int) -> tuple[bytes, bytes]:
    """Make an ELF with flash code, a gap, flash-backed .data, and a RAM-only load."""
    header_offset = 0x1000
    payload = b"code" * 4 + bytes(16) + b"data"
    flash = header + payload
    data = bytearray(header_offset + len(flash) + 4)
    data[header_offset : header_offset + len(flash)] = flash
    data[-4:] = b"RAM!"
    names = b"\x00.shstrtab\x00" + section.encode("ascii") + b"\x00"
    data[0x200 : 0x200 + len(names)] = names
    struct.pack_into(
        "<16sHHIIIIIHHHHHH",
        data,
        0,
        b"\x7fELF\x01\x01\x01" + bytes(9),
        2,
        40,
        1,
        0,
        52,
        0x100,
        0,
        52,
        32,
        3,
        40,
        3,
        1,
    )
    struct.pack_into(
        "<IIIIIIII",
        data,
        52,
        1,
        header_offset,
        flash_address,
        flash_address,
        len(header) + 16,
        len(header) + 16,
        5,
        4,
    )
    struct.pack_into(
        "<IIIIIIII",
        data,
        84,
        1,
        header_offset + len(flash) - 4,
        0x20000200,
        flash_address + len(flash) - 4,
        4,
        64,
        6,
        4,
    )
    struct.pack_into("<IIIIIIII", data, 116, 1, len(data) - 4, 0x20010000, 0x20010000, 4, 4, 6, 4)
    struct.pack_into("<IIIIIIIIII", data, 0x100 + 40, 1, 3, 0, 0, 0x200, len(names), 0, 0, 1, 0)
    struct.pack_into(
        "<IIIIIIIIII",
        data,
        0x100 + 80,
        11,
        1,
        2,
        flash_address,
        header_offset,
        len(header),
        0,
        0,
        4,
        0,
    )
    return bytes(data), payload


class ImageHeaderTests(unittest.TestCase):
    """Keep both devices compatible with the BitBox image format."""

    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)

    def render(self, manifest: dict) -> bytes:
        manifest_path = self.directory / "manifest.json"
        output = self.directory / "header.bin"
        manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
        args = image_header.build_parser().parse_args(
            ["render-header", "--manifest", str(manifest_path), "--output", str(output)]
        )
        image_header.cmd_render(args)
        return output.read_bytes()

    def test_render_stage1_matches_shipped_headers(self) -> None:
        images = sorted((REPOSITORY_ROOT / "src/bootloader_upgrade/bin").glob("*stage1*.bin"))
        self.assertTrue(images)
        for path in images:
            with self.subTest(image=path.name):
                expected = bytearray(path.read_bytes()[:1024])
                fields = bootloader_update._unpack_header(expected)
                actual = self.render(
                    stage1_manifest(
                        flags=fields["flags"],
                        product_id=fields["product_id"],
                        monotonic_version=fields["monotonic_version"],
                        marketing_version=fields["stage1_marketing_version"],
                    )
                )
                expected[16:24] = bytes(8)
                expected[832:] = bytes(192)
                self.assertEqual(actual, expected)

    def test_finalize_stage1_unsigned_image(self) -> None:
        header = self.render(stage1_manifest())
        for payload_len in (1, 36, 0xBFE0 - 1024):
            with self.subTest(payload_len=payload_len):
                payload = b"x" * payload_len
                actual = image_header.finalize_header_code_size(header, payload_len)
                bootloader_update._validate_complete_stage1(
                    actual + payload, 1, require_signatures=False
                )
                self.assertEqual(actual[:16], header[:16])
                self.assertEqual(actual[24:], header[24:])
                self.assertEqual(
                    image_header.finalize_header_code_size(actual, payload_len), actual
                )

    def test_finalize_code_size_command_stage1(self) -> None:
        header = self.render(stage1_manifest())
        payload = self.directory / "payload.bin"
        output = self.directory / "final.bin"
        payload.write_bytes(b"payload")
        args = image_header.build_parser().parse_args(
            [
                "finalize-code-size",
                "--header",
                str(self.directory / "header.bin"),
                "--payload",
                str(payload),
                "--output",
                str(output),
            ]
        )
        image_header.cmd_finalize_code_size(args)
        self.assertEqual(
            output.read_bytes(), header[:16] + (1031).to_bytes(8, "little") + header[24:]
        )

    def test_render_bitbox03_shared_layout(self) -> None:
        for target, magic in (("bitbox03-boot1", b"BBS1"), ("bitbox03-firmware", b"BBFW")):
            with self.subTest(target=target):
                path = REPOSITORY_ROOT / "src/rust/bins" / target / "image_header.json"
                header = self.render(json.loads(path.read_text(encoding="utf-8")))
                fields = bootloader_update._unpack_header(header)
                self.assertEqual(fields["magic"], int.from_bytes(magic, "little"))
                self.assertEqual(fields["header_len"], 1024)
                self.assertEqual(fields["image_len"], 0)
                self.assertEqual(fields["header_version"], 1)
                self.assertEqual(header[64:], bytes(960))
                # BitBox03 images are not limited to the BitBox02 stage1 slot.
                for code_size in (1234, 0x200000, 2**32):
                    finalized = image_header.finalize_header_code_size(header, code_size)
                    self.assertEqual(
                        finalized,
                        header[:16] + (1024 + code_size).to_bytes(8, "little") + header[24:],
                    )

    def test_render_rejects_invalid_stage1_metadata(self) -> None:
        for field, values in {
            "flags": (-1, 2, True, "1"),
            "product_id": (-1, 65536, False, 1.5),
            "monotonic_version": (-1, 65536, True, "1"),
            "marketing_version": ("", "x" * 38, "v1 2", "v1\x00", "v1\n", "v1é", None),
        }.items():
            for value in values:
                with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                    self.render(stage1_manifest(**{field: value}))
            missing = stage1_manifest()
            del missing[field]
            with self.subTest(missing=field), self.assertRaises(ValueError):
                self.render(missing)

    def test_render_stage1_metadata_boundaries(self) -> None:
        header = self.render(stage1_manifest(monotonic_version=65535, marketing_version="x" * 37))
        parsed = bootloader_update._unpack_header(header)
        self.assertEqual(parsed["monotonic_version"], 65535)
        self.assertEqual(parsed["stage1_marketing_version"], "x" * 37)

    def test_finalize_stage1_rejects_invalid_headers(self) -> None:
        header = self.render(stage1_manifest())
        for offset, value in (
            (0, b"XXXX"),
            (4, (2).to_bytes(4, "little")),
            (8, b"\x02\x00"),
            (12, (2048).to_bytes(4, "little")),
            (16, (1234).to_bytes(8, "little")),
            (26, b"\x00"),
            (26, b"\xff"),
            (27, b"\x00"),
            (63, b"x"),
            (64, b"x"),
            (831, b"x"),
            (832, b"x"),
            (1023, b"x"),
        ):
            invalid = bytearray(header)
            invalid[offset : offset + len(value)] = value
            with self.subTest(offset=offset, value=value), self.assertRaises(ValueError):
                image_header.finalize_header_code_size(bytes(invalid), 7)
        for invalid in (header[:-1], header + b"x"):
            with self.subTest(length=len(invalid)), self.assertRaises(ValueError):
                image_header.finalize_header_code_size(invalid, 7)
        for size in (-1, 0, 0xBFE0 - 1024 + 1, 2**64):
            with self.subTest(size=size), self.assertRaises(ValueError):
                image_header.finalize_header_code_size(header, size)

    def test_finalize_elf_flash_span(self) -> None:
        for manifest, section, address in (
            (stage1_manifest(), ".stage1_header", 0x2000),
            (stage1_manifest(product_id=0), ".image_header", 0x08002000),
            (stage1_manifest(magic="BBFW", product_id=0), ".image_header", 0x08020000),
        ):
            with self.subTest(magic=manifest["magic"]):
                header = self.render(manifest)
                original, payload = make_elf(header, section, address)
                elf = self.directory / "image.elf"
                elf.write_bytes(original)
                arguments = ["finalize-elf", str(elf)]
                if section != ".image_header":
                    arguments += ["--section", section]
                args = image_header.build_parser().parse_args(arguments)
                with contextlib.redirect_stdout(io.StringIO()):
                    image_header.cmd_finalize_elf(args)
                actual = elf.read_bytes()
                offset, size, value = 16, 8, 1024 + len(payload)
                if manifest["product_id"] == 1:
                    bootloader_update._validate_complete_stage1(
                        actual[0x1000 : 0x1000 + value], 1, require_signatures=False
                    )
                expected = bytearray(original)
                expected[0x1000 + offset : 0x1000 + offset + size] = value.to_bytes(size, "little")
                self.assertEqual(actual, expected)

    def test_finalize_elf_rejects_signed_stage1_without_modifying_file(self) -> None:
        header = bytearray(self.render(stage1_manifest()))
        header[832] = 1
        original, _ = make_elf(bytes(header), ".stage1_header", 0x2000)
        elf = self.directory / "signed.elf"
        elf.write_bytes(original)
        with self.assertRaisesRegex(ValueError, "signatures"):
            image_header.cmd_finalize_elf(argparse.Namespace(elf=elf, section=".stage1_header"))
        self.assertEqual(elf.read_bytes(), original)


if __name__ == "__main__":
    unittest.main()
