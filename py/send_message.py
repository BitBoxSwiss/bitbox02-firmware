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
"""Script for interacting with bitbox v2"""


import argparse
import pprint
import sys
from typing import List

from tzlocal import get_localzone
import bitbox02
from bitbox02 import HARDENED


def change_name(device: bitbox02.BitBox02, name: str) -> None:
    """
    Invoke change name workfow.
    """
    info = device.device_info()
    print(f"\nOld device name: {info['name']}")
    try:
        device.set_device_name(name)
    except bitbox02.UserAbortException:
        print("Aborted by user")
    else:
        print("\nSetting new device name.")
        info = device.device_info()
        print(f"\nNew device name: {info['name']}")


def setup_workflow(device: bitbox02.BitBox02) -> None:
    """TODO: Document"""
    device.insert_or_remove_sdcard(insert=True)
    print("SD Card Inserted")
    change_name(device, "Shifty")
    print(
        "Please choose a password of the BitBox02. "
        + "This password will be used to unlock your BitBox02."
    )
    while not device.set_password():
        print("Passwords did not match. please try again")

    print("Your BitBox02 will now create a backup of your wallet...")
    print("Please confirm the date on your device.")
    if not device.create_backup():
        print("Creating the backup failed")
        exit()
    print("Backup created sucessfully")

    print("Please Remove SD Card")
    device.insert_or_remove_sdcard(remove=True)


def print_backups(backups: List[bitbox02.Backup]) -> None:
    local_timezone = get_localzone()
    backups = list(backups)
    if not backups:
        print("No backups found.")
        return
    fmt = "%Y-%m-%d %H:%M:%S %z"
    for (i, (backup_id, backup_name, date)) in enumerate(backups):
        date = local_timezone.localize(date)
        print(f"[{i+1}] Backup Name: {backup_name}, Time: {date.strftime(fmt)}, ID: {backup_id}")


def restore_backup_workflow(device: bitbox02.BitBox02) -> None:
    """TODO: Document"""
    backups = list(device.list_backups())
    print_backups(backups)
    if not backups:
        return
    item = int(input("Choose a backup:\n"))
    backup_id, _, _ = backups[item - 1]
    print(f"ID: {backup_id}")
    if not device.restore_backup(backup_id):
        print("Restoring backup failed")
        return
    print("Please Remove SD Card")
    device.insert_or_remove_sdcard(remove=True)


def select_init_option(device: bitbox02.BitBox02) -> bool:
    """TODO: Document

    Returns:
        bool: If the user should be prompted again
    """
    # pylint: disable=too-many-branches
    print("What would you like to do?")
    print("- (1) Set up new wallet")
    print("- (2) Restore from backup")
    print("- (3) Restore from mnemonic")
    print("- (4) List device info")
    print("- (5) Reboot into bootloader")
    print("- (6) Check if SD card inserted")
    ans = input("")
    if ans == "q":
        return False
    try:
        choice = int(ans)
    except ValueError:
        print("Invalid input")
        return True
    if choice == 1:
        setup_workflow(device)
    elif choice == 2:
        restore_backup_workflow(device)
    elif choice == 3:
        if device.restore_from_mnemonic():
            print("Restore successful")
        else:
            print("Restore was NOT successful")
    elif choice == 4:
        info = device.device_info()
        print(f"\nAll info: {info}")
    elif choice == 5:
        if device.reboot():
            print("Device rebooted")
        else:
            print("User aborted")
        return False
    elif choice == 6:
        print(f"SD Card inserted: {device.check_sdcard()}")
    else:
        print("Input unknown, please try again...")
    return True


def print_menu(mnemonic_passphrase_enabled: bool) -> None:
    """Print the menu"""

    print("What would you like to do?")
    print("- (1) List device info")
    print("- (2) Change device name")
    print("- (3) Display random number")
    print("- (4) Retrieve master xpub")
    print("- (5) Sign a BTC tx.")
    print("- (6) List backups")
    print("- (7) Check backup")
    print("- (8) Show mnemonic")
    print("- (9) Create backup")
    print("- (10) Reboot into bootloader")
    print("- (11) Check if SD card inserted")
    if mnemonic_passphrase_enabled:
        print("- (12) Disable BIP39 Mnemonic Passphrase")
    else:
        print("- (12) Enable BIP39 Mnemonic Passphrase")
    print("- (13) Retrieve Ethereum address")
    print("- (14) Reset Device")
    print("- (q) Quit")


def select_option(device: bitbox02.BitBox02) -> bool:
    """TODO: Document
    TODO: Refactor code so that it doesn't have too many branches

    Returns:
        bool: If the user should be prompted again
    """
    # pylint: disable=too-many-branches,too-many-statements
    mnemonic_passphrase_enabled = device.device_info()["mnemonic_passphrase_enabled"]
    print_menu(mnemonic_passphrase_enabled)
    ans = input("")
    if ans == "q":
        return False
    try:
        choice = int(ans)
    except ValueError:
        print("Invalid input")
        return True
    if choice == 1:
        info = device.device_info()
        print(f"\nAll info: {info}")
    elif choice == 2:
        name = input("Enter a name [Mia] (max 64 bytes): ")
        if not name:
            name = "Mia"
        change_name(device, name)
    elif choice == 3:
        print(f"Random number: {device.random_number().hex()}")
    elif choice == 4:
        print(
            "m/84'/0'/0' xpub: ",
            device.btc_pub(
                keypath=[84 + HARDENED, 0 + HARDENED, 0 + HARDENED],
                output_type=bitbox02.hww.BTCPubRequest.ZPUB,  # pylint: disable=no-member
            ),
        )
    elif choice == 5:
        # Dummy transaction to invoke a demo.
        bip44_account: int = 0 + HARDENED
        inputs: List[bitbox02.BTCInputType] = [
            {
                "prev_out_hash": b"11111111111111111111111111111111",
                "prev_out_index": 1,
                "prev_out_value": int(1e8 * 0.60005),
                "sequence": 0xFFFFFFFF,
                "keypath": [84 + HARDENED, 0 + HARDENED, bip44_account, 0, 0],
            },
            {
                "prev_out_hash": b"11111111111111111111111111111111",
                "prev_out_index": 1,
                "prev_out_value": int(1e8 * 0.60005),
                "sequence": 0xFFFFFFFF,
                "keypath": [84 + HARDENED, 0 + HARDENED, bip44_account, 0, 1],
            },
        ]
        outputs: List[bitbox02.BTCOutputType] = [
            bitbox02.BTCOutputInternal(
                keypath=[84 + HARDENED, 0 + HARDENED, bip44_account, 1, 0], value=int(1e8 * 1)
            ),
            bitbox02.BTCOutputExternal(
                output_type=bitbox02.hww.P2WSH,
                output_hash=b"11111111111111111111111111111111",
                value=int(1e8 * 0.2),
            ),
        ]
        sigs = device.btc_sign(
            bitbox02.hww.BTC,
            bitbox02.hww.SCRIPT_P2WPKH,
            bip44_account=bip44_account,
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))
    elif choice == 6:
        print_backups(list(device.list_backups()))
    elif choice == 7:
        print("Your BitBox02 will now perform a backup check")
        backup_id = device.check_backup()
        if backup_id:
            print(f"Check successful. Backup with ID {backup_id} matches")
        else:
            print("No matching backup found")
    elif choice == 8:
        print("Your BitBox02 will now show the mnemonic seed phrase")
        print(device.show_mnemonic())
    elif choice == 9:
        if device.check_backup(silent=True) is not None:
            if input("A backup already exists, continue? Y/n: ") not in ("", "Y"):
                return True
        if not device.create_backup():
            print("Creating the backup failed")
        else:
            print("Backup created sucessfully")
    elif choice == 10:
        if device.reboot():
            print("Device rebooted")
            return False
        print("User aborted")
    elif choice == 11:
        print(f"SD Card inserted: {device.check_sdcard()}")
    elif choice == 12:
        mnemonic_passphrase_enabled = not mnemonic_passphrase_enabled
        try:
            device.set_mnemonic_passphrase_enabled(mnemonic_passphrase_enabled)
        except bitbox02.UserAbortException:
            print("Aborted by user")
        else:
            print("Success.")
            if mnemonic_passphrase_enabled:
                print("You can enter a mnemonic passphrase on the next unlock.")
                print("Replug your BitBox02.")
    elif choice == 13:

        def address(display: bool = False) -> str:
            return device.eth_pub(
                keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
                output_type=bitbox02.hww.ETHPubRequest.ADDRESS,  # pylint: disable=no-member
                display=display,
            )

        print("Ethereum address: {}".format(address(display=False)))
        address(display=True)
    elif choice == 14:
        if device.reset():
            print("Device RESET")
        else:
            print("Device NOT reset")
    else:
        print("Input unknown, please try again...")
    return True


def menu(device: bitbox02.BitBox02) -> bool:
    if not device.device_info()["initialized"]:
        return select_init_option(device)
    return select_option(device)


def main() -> int:
    """Main function"""
    parser = argparse.ArgumentParser(description="Tool for communicating with bitbox device")
    parser.add_argument("--debug", action="store_true", help="Print messages sent and received")
    args = parser.parse_args()

    bitboxes = bitbox02.get_bitbox02_devices()

    if not bitboxes:
        print("No bitbox detected")
        exit()

    if len(bitboxes) > 1:
        print("Multiple bitboxes detected. Only one supported")
        exit()

    print("Device Info:")
    pprint.pprint(bitboxes[0])

    def show_pairing(code: str) -> None:
        print("Please compare and confirm the pairing code on your BitBox02:")
        print(code)

    def attestation_check(result: bool) -> None:
        if result:
            print("Device attestation PASSED")
        else:
            print("Device attestation FAILED")

    device = bitbox02.BitBox02(
        device_info=bitboxes[0],
        show_pairing_callback=show_pairing,
        attestation_check_callback=attestation_check,
    )
    if args.debug:
        device.debug = True

    cont = True
    while cont:
        cont = menu(device)
    device.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
