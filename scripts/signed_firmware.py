#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Parse and hash signed BitBox02 firmware containers."""

from __future__ import annotations

import hashlib
import struct
from dataclasses import dataclass


# A signed firmware file consists of MAGIC_LEN bytes of a product marker, followed by SIGDATA_LEN
# bytes of signature data, and ending with the firmware bytes from the reproducible build.
MAGIC_LEN = 4
MAX_FIRMWARE_SIZE = 884736
NUM_ROOT_KEYS = 3
NUM_SIGNING_KEYS = 3
VERSION_LEN = 4
SIGNING_PUBKEYS_DATA_LEN = VERSION_LEN + NUM_SIGNING_KEYS * 64 + NUM_ROOT_KEYS * 64
FIRMWARE_DATA_LEN = VERSION_LEN + NUM_SIGNING_KEYS * 64
SIGDATA_LEN = SIGNING_PUBKEYS_DATA_LEN + FIRMWARE_DATA_LEN


@dataclass(frozen=True)
class Product:
    """Product values included in the signed-firmware format."""

    product_id: int
    magic: bytes


BITBOX02_MULTI = Product(product_id=1, magic=bytes.fromhex("653f362b"))
BITBOX02_BTCONLY = Product(product_id=2, magic=bytes.fromhex("11233b0b"))
BITBOX02_NOVA_MULTI = Product(product_id=3, magic=bytes.fromhex("5b648ceb"))
BITBOX02_NOVA_BTCONLY = Product(product_id=4, magic=bytes.fromhex("48714774"))

PRODUCTS = (
    BITBOX02_MULTI,
    BITBOX02_BTCONLY,
    BITBOX02_NOVA_MULTI,
    BITBOX02_NOVA_BTCONLY,
)
PRODUCT_BY_MAGIC = {product.magic: product for product in PRODUCTS}


@dataclass(frozen=True)
class SignedFirmware:
    """Parsed signed-firmware fields needed by release tooling."""

    product: Product
    sigdata: bytes
    firmware: bytes

    @property
    def version(self) -> bytes:
        """Return the encoded monotonic firmware version."""

        return self.sigdata[SIGNING_PUBKEYS_DATA_LEN : SIGNING_PUBKEYS_DATA_LEN + VERSION_LEN]


def parse(data: bytes) -> SignedFirmware:
    """Parse and validate a signed-firmware container."""

    prefix_len = MAGIC_LEN + SIGDATA_LEN
    if len(data) <= prefix_len:
        raise ValueError("firmware is too small to contain firmware data")

    magic = data[:MAGIC_LEN]
    product = PRODUCT_BY_MAGIC.get(magic)
    if product is None:
        raise ValueError(f"unrecognized firmware magic {magic.hex()}")

    sigdata = data[MAGIC_LEN:prefix_len]
    firmware = data[prefix_len:]
    if len(firmware) > MAX_FIRMWARE_SIZE:
        raise ValueError(
            f"firmware payload is {len(firmware)} bytes, exceeding the maximum "
            f"of {MAX_FIRMWARE_SIZE}"
        )
    return SignedFirmware(product=product, sigdata=sigdata, firmware=firmware)


def unsigned_sha256(signed_firmware: SignedFirmware) -> bytes:
    """Return the SHA-256 digest of the reproducible, unsigned firmware bytes."""

    return hashlib.sha256(signed_firmware.firmware).digest()


def sighash(signed_firmware: SignedFirmware) -> bytes:
    """Return the current firmware hash verified and shown by the bootloader."""

    firmware_padded = signed_firmware.firmware + b"\xff" * (
        MAX_FIRMWARE_SIZE - len(signed_firmware.firmware)
    )
    preimage = (
        struct.pack("<H", signed_firmware.product.product_id)
        + signed_firmware.version
        + firmware_padded
    )
    return hashlib.sha256(preimage).digest()
