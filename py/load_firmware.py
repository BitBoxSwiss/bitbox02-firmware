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
from typing import Any
from time import sleep

import hid
from communication import devices
from communication.devices import TooManyFoundException, NoneFoundException
from communication import u2fhid

from bitbox02 import Bootloader, BitBox02


def eprint(*args: Any, **kwargs: Any) -> None:
    """
    Like print, but defaults to stderr.
    """
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


def get_bitbox_and_reboot() -> devices.DeviceInfo:
    """Search for a bitbox and then reboot it into bootloader"""
    device = devices.get_any_bitbox02()

    # bitbox02 detected -> send command to reboot into bootloader to upgrade.
    def _show_pairing(code: str) -> None:
        print("Please compare and confirm the pairing code on your BitBox02:")
        print(code)

    hid_device = hid.device()
    hid_device.open_path(device["path"])
    bitbox = BitBox02(
        u2fhid.U2FHid(hid_device), device_info=device, show_pairing_callback=_show_pairing
    )
    bitbox.reboot()

    # wait for it to reboot
    while True:
        try:
            bootloader_device = devices.get_any_bitbox02_bootloader()
        except NoneFoundException:
            sys.stdout.write(".")
            sys.stdout.flush()
            sleep(1)
            continue
        return bootloader_device


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
        eprint("Expecting firmware to end with '.signed.bin'")
        return 1

    bootloader_device = None
    try:
        bootloader_device = devices.get_any_bitbox02_bootloader()
    except TooManyFoundException:
        eprint("Found multiple bb02 bootloader standard editions. Only one supported.")
        return 1
    except NoneFoundException:
        pass

    if bootloader_device is None:
        try:
            bootloader_device = get_bitbox_and_reboot()
        except TooManyFoundException:
            eprint("Found multiple bitboxes. Only one supported.")
            return 1
        except NoneFoundException:
            eprint("Neither bootloader nor bitbox found.")
            return 1

    pprint.pprint(bootloader_device)

    hid_dev = hid.device()
    hid_dev.open_path(bootloader_device["path"])

    bootloader = Bootloader(u2fhid.U2FHid(hid_dev), bootloader_device)

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
