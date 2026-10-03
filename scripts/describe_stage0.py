#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Validate a stage0 binary's descriptor and print its information."""

import argparse
import hashlib
import struct
from pathlib import Path


# Keep in sync with bootloader-stage0.ld and src/bootloader/stage0/stage0_descriptor.h.
STAGE0_LEN = 0x2000
DESCRIPTOR_OFFSET = 0x1FF4
DESCRIPTOR_FORMAT = "<HHII"
DESCRIPTOR_MAGIC = 0x30534242
FLAG_DEVELOPMENT = 1 << 0

# Keep in sync with src/bootloader/bootloader_product.h.
PRODUCTS = {
    1: "BitBox02 Multi",
    2: "BitBox02 Bitcoin-only",
    3: "BitBox02 Nova Multi",
    4: "BitBox02 Nova Bitcoin-only",
}


def describe_stage0(stage0: bytes) -> None:
    """Validate the image size and descriptor before printing any information."""
    if len(stage0) != STAGE0_LEN:
        raise ValueError(f"invalid stage0 image size: {len(stage0)} bytes, expected {STAGE0_LEN}")

    version, product_id, flags, magic = struct.unpack_from(
        DESCRIPTOR_FORMAT, stage0, DESCRIPTOR_OFFSET
    )
    if magic != DESCRIPTOR_MAGIC:
        raise ValueError(f"invalid stage0 descriptor magic: 0x{magic:08x}")
    if product_id not in PRODUCTS:
        raise ValueError(f"invalid stage0 descriptor product ID: {product_id}")
    if flags & ~FLAG_DEVELOPMENT:
        raise ValueError(f"invalid stage0 descriptor flags: 0x{flags:08x}")

    print(f"Product: {PRODUCTS[product_id]} (ID: {product_id})")
    print(f"Stage0 version: {version}")
    print(f"Mode: {'development' if flags & FLAG_DEVELOPMENT else 'production'}")
    print(f"Flags: 0x{flags:08x}")
    print(f"Descriptor magic: 0x{magic:08x} (BBS0)")
    print(f"Image size: {len(stage0)} bytes")
    print(f"SHA-256: {hashlib.sha256(stage0).hexdigest()}")


def main() -> None:
    """Describe the stage0 binary supplied on the command line."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("stage0_bin", type=Path, help="path to a stage0 .bin image")
    args = parser.parse_args()
    try:
        describe_stage0(args.stage0_bin.read_bytes())
    except (OSError, ValueError) as err:
        parser.exit(1, f"{parser.prog}: {err}\n")


if __name__ == "__main__":
    main()
