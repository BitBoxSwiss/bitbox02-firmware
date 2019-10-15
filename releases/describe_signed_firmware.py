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

try:
    from bitbox02.bootloader import (
        parse_signed_firmware,
        SIGDATA_MAGIC_STANDARD,
        SIGDATA_MAGIC_BTCONLY,
        SIGNING_PUBKEYS_DATA_LEN,
        MAX_FIRMWARE_SIZE,
    )
except ModuleNotFoundError:
    print("bitbox02 module not found; please see bitbox02-firmware/py/README.md")

import sys
import hashlib
import struct


def main() -> int:
    """Main function"""
    try:
        filename = sys.argv[1]
    except IndexError:
        print("Usage: ./describe_signed_firmware.py firmware.vX.Y.Z.signed.bin")
        return 1

    with open(filename, "rb") as fileobj:
        binary = fileobj.read()

    try:
        magic, sigdata, firmware = parse_signed_firmware(binary)
    except ValueError as exception:
        print(exception)
        return 1

    print(
        "The following information assumes the provided binary was signed correctly; "
        "the signatures are not being verified."
    )
    if magic == SIGDATA_MAGIC_STANDARD:
        print("This is a Multi-edition firmware.")
    elif magic == SIGDATA_MAGIC_BTCONLY:
        print("This is a Bitcoin-only edition firmware.")
    else:
        print("Unrecognized firmware edition; magic =", magic.hex())

    firmware_padded = firmware + b"\xFF" * (MAX_FIRMWARE_SIZE - len(firmware))

    print(
        "The hash of the unsigned firmware binary is (compare with reproducible build):"
    )
    print(hashlib.sha256(firmware).hexdigest())
    version = sigdata[SIGNING_PUBKEYS_DATA_LEN:][:4]
    print("The monotonic firmware version is:", struct.unpack("<I", version)[0])
    print("The hash of the firmware as verified/shown by the bootloader is:")
    print(
        hashlib.sha256(hashlib.sha256(version + firmware_padded).digest()).hexdigest()
    )

    return 0


if __name__ == "__main__":
    sys.exit(main())
