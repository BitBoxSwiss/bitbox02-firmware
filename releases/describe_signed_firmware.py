#!/usr/bin/env python3
# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
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

MAX_FIRMWARE_SIZE = 884736
NUM_ROOT_KEYS = 3
NUM_SIGNING_KEYS = 3
VERSION_LEN = 4
SIGNING_PUBKEYS_DATA_LEN = VERSION_LEN + NUM_SIGNING_KEYS * 64 + NUM_ROOT_KEYS * 64
FIRMWARE_DATA_LEN = VERSION_LEN + NUM_SIGNING_KEYS * 64
SIGDATA_LEN = SIGNING_PUBKEYS_DATA_LEN + FIRMWARE_DATA_LEN


def main() -> int:
    """Main function"""

    try:
        filename = sys.argv[1]
    except IndexError:
        print("Usage: ./describe_signed_firmware.py firmware.vX.Y.Z.signed.bin")
        return 1

    with open(filename, "rb") as fileobj:
        file_content = fileobj.read()

    firmware_is_signed = len(file_content) >= MAGIC_LEN and file_content[:MAGIC_LEN] in [MAGIC_MULTI, MAGIC_BTCONLY]

    if firmware_is_signed:
        # If firmware is signed
        # Split signed firmware into sigdata and firmware
        if len(file_content) < SIGDATA_LEN:
            print("firmware too small")
            return 1
        magic, rest = file_content[:MAGIC_LEN], file_content[MAGIC_LEN:]
        sigdata, firmware = rest[:SIGDATA_LEN], rest[SIGDATA_LEN:]

        print(
            "The following information assumes the provided binary was signed correctly; "
            "the signatures are not being verified."
        )
        if magic == MAGIC_MULTI:
            print("This is a Multi-edition firmware.")
        elif magic == MAGIC_BTCONLY:
            print("This is a Bitcoin-only edition firmware.")
        else:
            print("Unrecognized firmware edition; magic =", magic.hex())
    else:
        print("This firmware is not signed.")
        firmware = file_content

    firmware_padded = firmware + b"\xFF" * (MAX_FIRMWARE_SIZE - len(firmware))

    print(
        "The hash of the firmware binary is (compare with reproducible build):"
    )
    print(hashlib.sha256(firmware).hexdigest())
    
    if firmware_is_signed and len(file_content) >= SIGNING_PUBKEYS_DATA_LEN + VERSION_LEN:
        version = sigdata[SIGNING_PUBKEYS_DATA_LEN:][:VERSION_LEN]
        print("The monotonic firmware version is:", struct.unpack("<I", version)[0])
        print("The hash of the firmware as verified/shown by the bootloader is:")
        print(
            hashlib.sha256(hashlib.sha256(version + firmware_padded).digest()).hexdigest()
        )

    return 0


if __name__ == "__main__":
    sys.exit(main())
