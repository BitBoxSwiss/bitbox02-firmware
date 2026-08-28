#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Flash signed or unsigned firmware onto a BitBox device.

The input type is detected from the signed-firmware header. The supported combinations are:

* Production device, signed firmware: flash the firmware and signature data.
* Production device, unsigned firmware: warn that it will not boot, then flash the firmware.
* Development device, signed firmware: flash the firmware, attempt to flash the signature data,
  warn if the signature data is rejected, and reboot into the firmware anyway.
* Development device, unsigned firmware: flash the firmware without signature data.

Every flash requires confirmation unless ``-y``/``--yes`` is used. Edition mismatches and
combinations that are expected to fail are called out before the confirmation.
"""

import argparse
import pprint
import sys
import time
from pathlib import Path
from typing import Any, Callable, Dict, NamedTuple, Optional, Sequence, Tuple

import hid

from bitbox02.communication import devices, TransportLayer, u2fhid, bitbox_api_protocol
from bitbox02.communication.devices import TooManyFoundException, NoneFoundException

from bitbox02.bitbox02 import (
    BitBox02,
    Bootloader,
    BootloaderError,
    bootloader as bootloader_api,
)
from bitbox02 import util


BOOTLOADER_REBOOT_TIMEOUT_SECONDS = 30.0

FLASHING_HELP = """\
Flashing behavior:
  signed firmware   + production device  flash firmware and signatures
  signed firmware   + development device flash firmware, attempt signatures, and treat a
                                         signature error as a warning
  unsigned firmware + production device  flash firmware, but it will not boot
  unsigned firmware + development device flash firmware without signatures

The input type is detected from the signed-firmware header. Recognized but malformed signed
firmware is rejected. Every flash asks for confirmation; -y/--yes skips the prompt. Edition
mismatches and combinations that are expected to fail are called out before confirmation.
"""

BOOTLOADER_ERROR_MESSAGES: Dict[int, str] = {
    ord("Z"): (
        "The bootloader rejected the signature data. The signatures may be invalid, corrupt, or "
        "for a different device."
    ),
    ord("V"): (
        "The bootloader rejected a firmware or signing-key downgrade. Use a signed firmware with "
        "versions at least as new as those already stored on the device."
    ),
    ord("N"): "The bootloader rejected an invalid firmware length or chunk number.",
    ord("M"): (
        "The loader and bootloader disagree about the firmware chunk size. Update the Python "
        "package and retry."
    ),
    ord("W"): "The device could not write the firmware to flash memory.",
    ord("C"): "The device could not verify flash memory after writing or erasing it.",
    ord("A"): "The device aborted the firmware operation.",
    ord("E"): "The device could not erase flash memory.",
    ord("L"): ("The bootloader was not ready for this operation. Reconnect the device and retry."),
    ord("I"): (
        "The bootloader does not support this command. Check that the Python package is compatible "
        "with the bootloader."
    ),
    ord("U"): "The device could not unlock flash memory for writing.",
    ord("K"): "The device could not lock flash memory after writing.",
}


class FirmwareInput(NamedTuple):
    """A validated firmware input and its parsed signed-container fields."""

    payload: bytes
    magic: Optional[bytes]
    sigdata: Optional[bytes]


class FirmwareInputError(Exception):
    """The selected firmware file is invalid."""


def eprint(*args: Any, **kwargs: Any) -> None:
    """
    Like print, but defaults to stderr.
    """
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


def _bootloader_error_message(error: BootloaderError) -> str:
    """Return an actionable description of a bootloader status."""
    return BOOTLOADER_ERROR_MESSAGES.get(
        error.code, f"The bootloader returned an unknown error status (0x{error.code:02x})."
    )


def _read_firmware(filename: str) -> FirmwareInput:
    """Read, classify, and validate a signed container or raw firmware."""
    try:
        firmware = Path(filename).read_bytes()
    except OSError as error:
        raise FirmwareInputError(f"Could not read firmware file '{filename}': {error}") from error

    if not firmware:
        raise FirmwareInputError("The firmware file is empty.")

    try:
        magic, sigdata, payload = bootloader_api.parse_signed_firmware(firmware)
    except bootloader_api.InvalidFirmwareMagic:
        payload = firmware
        magic = None
        sigdata = None
    except ValueError as error:
        raise FirmwareInputError(f"Invalid signed firmware: {error}") from error

    return FirmwareInput(payload, magic, sigdata)


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
    deadline = time.monotonic() + BOOTLOADER_REBOOT_TIMEOUT_SECONDS
    waiting = False
    while True:
        try:
            bootloader_device = devices.get_any_bitbox02_bootloader()
        except NoneFoundException:
            if time.monotonic() >= deadline:
                if waiting:
                    print()
                raise TimeoutError(
                    "The device did not enter bootloader mode within 30 seconds. "
                    "Reconnect it and retry."
                )
            sys.stdout.write(".")
            sys.stdout.flush()
            waiting = True
            time.sleep(1)
            continue
        if waiting:
            print()
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


def _confirm_flash(signed: bool, devdevice: bool, product_string: str) -> bool:
    """Ask the user to confirm the detected firmware and device combination."""
    firmware_kind = "signed" if signed else "unsigned"
    device_kind = "development" if devdevice else "production"
    try:
        response = input(
            f"Flash {firmware_kind} firmware to this {device_kind} device "
            f"({product_string})? [Y/n] "
        )
    except EOFError:
        return False
    return response.strip().lower() in ("", "y", "yes")


def _flash_firmware(
    bootloader: Bootloader,
    bootloader_device: devices.DeviceInfo,
    firmware: FirmwareInput,
    yes: bool,
    progress: Callable[[float], None],
) -> bool:
    """Flash a validated firmware input. Returns false if the user declines."""
    devdevice = bootloader.is_devdevice()
    signed = firmware.sigdata is not None

    if not signed and not devdevice:
        eprint(
            "WARNING: Unsigned firmware cannot boot on a production device; rebooting is "
            "expected to show a firmware verification error."
        )
    if signed and firmware.magic != bootloader.expected_magic:
        eprint(
            "WARNING: The signed firmware edition does not match the connected device; "
            "installing its signature data is expected to fail."
        )
    if signed and devdevice:
        eprint(
            "WARNING: Development device detected. Signature data will be attempted, but a "
            "rejection will not prevent the firmware from booting."
        )

    if not yes and not _confirm_flash(signed, devdevice, bootloader_device["product_string"]):
        return False

    bootloader.flash_unsigned_firmware(firmware.payload, progress)
    if not signed:
        return True

    assert firmware.sigdata is not None
    try:
        bootloader.flash_sigdata(firmware.sigdata)
    except Exception as error:  # pylint: disable=broad-exception-caught
        if not devdevice:
            raise
        if isinstance(error, BootloaderError):
            message = _bootloader_error_message(error)
        else:
            message = f"Could not install the signature data: {error}"
        eprint(f"WARNING: {message}")
        eprint("The firmware payload was flashed and the development device will be rebooted.")
    return True


def main(argv: Optional[Sequence[str]] = None) -> int:
    """Main function"""
    parser = argparse.ArgumentParser(
        description="Tool for flashing a new firmware on BitBox devices.",
        epilog=FLASHING_HELP,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--no-cache", action="store_true", help="Don't use cached or store noise keys"
    )
    parser.add_argument(
        "-y",
        "--yes",
        action="store_true",
        help="Skip the confirmation prompt.",
    )
    parser.add_argument(
        "--debug",
        action="store_true",
        help="Deprecated no-op; firmware type is detected automatically.",
    )
    parser.add_argument("firmware", help="Firmware to flash.")
    args = parser.parse_args(argv)

    if args.debug:
        eprint(
            "WARNING: --debug is deprecated and has no effect; firmware type is detected "
            "automatically."
        )

    try:
        firmware = _read_firmware(args.firmware)
    except FirmwareInputError as error:
        eprint(f"Error: {error}")
        return 1

    try:
        bootloader_device, transport = _find_and_open_usb_bitbox02(not args.no_cache)
    except TimeoutError as error:
        eprint(f"Error: {error}")
        return 1
    bootloader = Bootloader(transport, bootloader_device)

    def progress(perc: float) -> None:
        sys.stdout.write(f"{perc*100:.02f}%\r")

    try:
        if bootloader.erased():
            print("device contains NO firmware")
        else:
            print("firmware version: %d\nsigning pubkeys version: %d" % bootloader.versions())
            firmware_hash, signing_keydata_hash = bootloader.get_hashes()
            print("firmware hash:", firmware_hash.hex())
            print("signing keydata hash:", signing_keydata_hash.hex())

        try:
            flashed = _flash_firmware(bootloader, bootloader_device, firmware, args.yes, progress)
        finally:
            print()  # Finish the progress line, including when flashing fails.
    except BootloaderError as error:
        eprint(f"Error: {_bootloader_error_message(error)}")
        return 1
    except ValueError as error:
        eprint(f"Error: {error}")
        return 1

    if not flashed:
        eprint("Firmware flashing aborted.")
        return 1

    time.sleep(1)  # Pause to show the upgrade finished at 100%
    bootloader.reboot()
    return 0


if __name__ == "__main__":
    sys.exit(main())
