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
"""TODO: document"""


import sys
import pprint
from typing import Any, List
from time import sleep

from bitbox02 import devices
from bitbox02 import Bootloader, BitBox02


def eprint(*args: Any, **kwargs: Any) -> None:
    """
    Like print, but defaults to stderr.
    """
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


def main() -> int:
    """Main function"""
    debug = len(sys.argv) == 3 and sys.argv[2] == "debug"
    if not (len(sys.argv) == 2 or debug):
        eprint("\n\nUsage:\n\tpython load_firmware.py firmware_name.bin [debug]")
        eprint(
            "\tif debug is specified, the firmware should be unsigned, otherwise it "
            "should be signed."
        )
        return 1

    filename = sys.argv[1]
    if not debug and ".signed.bin" not in filename:
        eprint("Expecting signed firmware")
        return 1

    bootloaders = devices.get_bitbox02_devices(devices.BOOTLOADER)
    bitboxes = devices.get_bitbox02_devices()

    def _wait_for_bootloaders() -> List[devices.DeviceInfo]:
        while True:
            bootloaders = devices.get_bitbox02_devices(devices.BOOTLOADER)
            if bootloaders:
                return bootloaders
            sys.stdout.write(".")
            sleep(1)

    if not bootloaders:
        if len(bitboxes) != 1:
            eprint(
                "No bitbox02 bootloader detected. Insert exactly one bootloader or "
                "bitbox02 device."
            )
            return 1

        # bitbox02 detected -> send command to reboot into bootloader to upgrade.
        def show_pairing(code: str) -> None:
            eprint("Please compare and confirm the pairing code on your BitBox02:")
            eprint(code)

        bitbox = BitBox02(device_info=bitboxes[0], show_pairing_callback=show_pairing)
        bitbox.reboot()
        bootloaders = _wait_for_bootloaders()

    if len(bootloaders) > 1:
        eprint("Multiple bootloaders detected. Only one supported")
        return 1

    pprint.pprint(bootloaders[0])

    bootloader = Bootloader(bootloaders[0])

    with open(filename, "rb") as file:
        firmware = file.read()

    def progress(perc: float) -> None:
        sys.stdout.write(f"{perc*100:.02f}%\r")

    if bootloader.erased():
        print("device contains NO firmware")
    else:
        print("firmware version: %d\nsigning pubkeys version: %d" % bootloader.versions())
        firmware_hash, signing_keydata_hash = bootloader.get_hashes()
        print("firmware hash:", firmware_hash.hex())
        print("signing keydata hash:", signing_keydata_hash.hex())

    if debug:
        bootloader.flash_unsigned_firmware(firmware, progress)
    else:
        bootloader.flash_signed_firmware(firmware, progress)
    print()  # print a newline

    sleep(1)  # Pause to show the upgrade finished at 100%
    bootloader.reboot()
    return 0


if __name__ == "__main__":
    sys.exit(main())
