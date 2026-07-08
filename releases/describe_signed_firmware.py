#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""CLI tool to dump infos about a signed firmware binary"""

import hashlib
import struct
import sys

# A signed firmware file consists of MAGIC_LEN (4) bytes of a firmware edition marker, followed by a
# SIGDATA_LEN bytes of a signature, and ending with the actual firmware binary bytes as resulting
# from a reproducible build.

MAGIC_LEN = 4
MAGIC_MULTI = struct.pack(">I", 0x653F362B)
MAGIC_BTCONLY = struct.pack(">I", 0x11233B0B)
MAGIC_BITBOX02PLUS_MULTI = struct.pack(">I", 0x5B648CEB)
MAGIC_BITBOX02PLUS_BTCONLY = struct.pack(">I", 0x48714774)

PRODUCT_ID_BITBOX02_MULTI = 1
PRODUCT_ID_BITBOX02_BTCONLY = 2
PRODUCT_ID_BITBOX02PLUS_MULTI = 3
PRODUCT_ID_BITBOX02PLUS_BTCONLY = 4

MAX_FIRMWARE_SIZE = 884736
NUM_ROOT_KEYS = 3
NUM_SIGNING_KEYS = 3
VERSION_LEN = 4
SIGNING_PUBKEYS_DATA_LEN = VERSION_LEN + NUM_SIGNING_KEYS * 64 + NUM_ROOT_KEYS * 64
FIRMWARE_DATA_LEN = VERSION_LEN + NUM_SIGNING_KEYS * 64
SIGDATA_LEN = SIGNING_PUBKEYS_DATA_LEN + FIRMWARE_DATA_LEN
NEW_SIGHASH_VERSION_CUTOFF = 50


def main() -> int:
    """Main function"""

    try:
        filename = sys.argv[1]
    except IndexError:
        print("Usage: ./describe_signed_firmware.py firmware.vX.Y.Z.signed.bin")
        return 1

    with open(filename, "rb") as fileobj:
        signed_firmware = fileobj.read()

    # Split signed firmware into sigdata and firmware
    if len(signed_firmware) < SIGDATA_LEN:
        print("firmware too small")
        return 1
    magic, rest = signed_firmware[:MAGIC_LEN], signed_firmware[MAGIC_LEN:]
    sigdata, firmware = rest[:SIGDATA_LEN], rest[SIGDATA_LEN:]

    if magic == MAGIC_MULTI:
        print("This is a BitBox02 Multi firmware.")
        product_id = PRODUCT_ID_BITBOX02_MULTI
    elif magic == MAGIC_BTCONLY:
        print("This is a BitBox02 Bitcoin-only firmware.")
        product_id = PRODUCT_ID_BITBOX02_BTCONLY
    elif magic == MAGIC_BITBOX02PLUS_MULTI:
        print("This is a BitBox02 Nova Multi firmware")
        product_id = PRODUCT_ID_BITBOX02PLUS_MULTI
    elif magic == MAGIC_BITBOX02PLUS_BTCONLY:
        print("This is a BitBox02 Nova Bitcoin-only firmware.")
        product_id = PRODUCT_ID_BITBOX02PLUS_BTCONLY
    else:
        print(
            f"Unrecognized firmware edition; magic = f{magic.hex()}. Maybe you have accidentally invoked this script on an unsigned binary. Make sure to use a signed firmware binary."
        )
        return 1
    print(
        "The following information assumes the provided binary was signed correctly; "
        "the signatures are not being verified."
    )

    firmware_padded = firmware + b"\xff" * (MAX_FIRMWARE_SIZE - len(firmware))

    print("The hash of the unsigned firmware binary is (compare with reproducible build):")
    print(hashlib.sha256(firmware).hexdigest())
    version = sigdata[SIGNING_PUBKEYS_DATA_LEN:][:VERSION_LEN]
    monotonic_version = struct.unpack("<I", version)[0]
    print("The monotonic firmware version is:", monotonic_version)
    print("The firmware sighash as verified/shown by the bootloader is:")
    if monotonic_version >= NEW_SIGHASH_VERSION_CUTOFF:
        print(hashlib.sha256(struct.pack("<H", product_id) + version + firmware_padded).hexdigest())
    else:
        print(hashlib.sha256(hashlib.sha256(version + firmware_padded).digest()).hexdigest())

    return 0


if __name__ == "__main__":
    sys.exit(main())
