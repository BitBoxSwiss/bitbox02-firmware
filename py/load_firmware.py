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

import argparse
import sys

import pprint
from typing import Callable, Any, Tuple
from time import sleep

import hid

from bitbox02.communication import devices, TransportLayer, u2fhid, bitbox_api_protocol
from bitbox02.communication.devices import TooManyFoundException, NoneFoundException

from bitbox02.bitbox02 import Bootloader, BitBox02
from bitbox02 import util


def eprint(*args: Any, **kwargs: Any) -> None:
    """
    Like print, but defaults to stderr.
    """
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


def _get_bitbox_and_reboot(use_cache: bool) -> devices.DeviceInfo:
    """Search for a bitbox and then reboot it into bootloader"""
    device = devices.get_any_bitbox02()

    class NoiseConfig(util.NoiseConfigUserCache):
        """NoiseConfig extends NoiseConfigUserCache"""

        def __init__(self) -> None:
            super().__init__("shift/load_firmware")

        def show_pairing(self, code: str, device_response: Callable[[], bool]) -> bool:
            print("Please compare and confirm the pairing code on your BitBox02:")
            print(code)
            return device_response()

    class NoiseConfigNoCache(bitbox_api_protocol.BitBoxNoiseConfig):
        """NoiseConfig extends BitBoxNoiseConfig"""

        def show_pairing(self, code: str, device_response: Callable[[], bool]) -> bool:
            print("Please compare and confirm the pairing code on your BitBox02:")
            print(code)
            return device_response()

    if use_cache:
        config: bitbox_api_protocol.BitBoxNoiseConfig = NoiseConfig()
    else:
        config = NoiseConfigNoCache()

    hid_device = hid.device()
    hid_device.open_path(device["path"])
    bitbox = BitBox02(transport=u2fhid.U2FHid(hid_device), device_info=device, noise_config=config)
    if not bitbox.reboot():
        raise RuntimeError("User aborted")

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


def _find_and_open_usb_bitbox02(use_cache: bool) -> Tuple[devices.DeviceInfo, TransportLayer]:
    """
    Connects to a BitBox02 bootloader over USB.
    If the BitBox02 is currently running a firmware, it will
    be rebooted and this function will connect to the bootloader
    when it shows up.
    """
    bootloader_device = None
    try:
        bootloader_device = devices.get_any_bitbox02_bootloader()
    except TooManyFoundException:
        eprint("Found multiple bb02 bootloader standard editions. Only one supported.")
        sys.exit(1)
    except NoneFoundException:
        pass

    if bootloader_device is None:
        try:
            bootloader_device = _get_bitbox_and_reboot(use_cache)
        except TooManyFoundException:
            eprint("Found multiple bitboxes. Only one supported.")
            sys.exit(1)
        except NoneFoundException:
            eprint("Neither bootloader nor bitbox found.")
            sys.exit(1)

    pprint.pprint(bootloader_device)

    hid_device = hid.device()
    hid_device.open_path(bootloader_device["path"])
    return bootloader_device, u2fhid.U2FHid(hid_device)


def main() -> int:
    """Main function"""
    parser = argparse.ArgumentParser(
        description="Tool for flashing a new firmware on BitBox devices."
    )
    parser.add_argument(
        "--no-cache", action="store_true", help="Don't use cached or store noise keys"
    )
    parser.add_argument("--debug", action="store_true", help="Flash a debug (unsigned) firmware.")
    parser.add_argument("firmware", nargs=1, help="Firmware to flash.")
    args = parser.parse_args()

    if not args.debug and ".signed.bin" not in args.firmware[0]:
        eprint("Expecting firmware to end with '.signed.bin'")
        return 1

    bootloader_device, transport = _find_and_open_usb_bitbox02(not args.no_cache)
    bootloader = Bootloader(transport, bootloader_device)

    with open(args.firmware[0], "rb") as file:
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

    if args.debug:
        bootloader.flash_unsigned_firmware(firmware, progress)
    else:
        bootloader.flash_signed_firmware(firmware, progress)
    print()  # print a newline

    sleep(1)  # Pause to show the upgrade finished at 100%
    bootloader.reboot()
    return 0


if __name__ == "__main__":
    sys.exit(main())
