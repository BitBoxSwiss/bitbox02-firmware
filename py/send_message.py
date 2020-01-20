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
from typing import List, Any, Optional, Callable, Union, Tuple, Sequence
import hashlib
import base64

import hid
from tzlocal import get_localzone


from bitboxbase import bitboxbase, BitBoxBase, get_bitboxbase_default_device
from communication import devices, HARDENED, UserAbortException, u2fhid, usart

import bitbox02
import u2f
import u2f.bitbox02


def eprint(*args: Any, **kwargs: Any) -> None:
    """
    Like print, but defaults to stderr.
    """
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


def ask_user(
    choices: Sequence[Tuple[str, Callable[[], None]]]
) -> Union[Callable[[], None], bool, None]:
    """Ask user to choose one of the choices, q quits"""
    print("What would you like to do?")
    for (idx, choice) in enumerate(choices):
        print(f"- ({idx+1}) {choice[0]}")
    print("- (q) Quit")
    ans_str = input("")
    if ans_str == "q":
        return False
    try:
        ans = int(ans_str)
        if ans < 1 or ans > len(choices):
            raise ValueError("Out of range")
    except ValueError:
        print("Invalid input")
        return None
    return choices[ans - 1][1]


class SendMessage:
    """SendMessage"""

    # pylint: disable=too-few-public-methods

    def __init__(self, device: bitbox02.BitBox02, debug: bool):
        self._device = device
        self._debug = debug
        self._stop = False

    def _change_name_workflow(self, name: Optional[str] = None) -> None:
        """
        Invoke change name workfow.
        """
        if name is None:
            name = input("Enter a name [Mia] (max 64 bytes): ")
            if not name:
                name = "Mia"
        info = self._device.device_info()
        print(f"Old device name: {info['name']}")
        try:
            self._device.set_device_name(name)
        except UserAbortException:
            eprint("Aborted by user")
        else:
            print("Setting new device name.")
            info = self._device.device_info()
            print(f"New device name: {info['name']}")

    def _setup_workflow(self) -> None:
        """TODO: Document"""
        self._device.insert_sdcard()
        print("SD Card Inserted")
        self._change_name_workflow("Shifty")
        print(
            "Please choose a password of the BitBox02. "
            + "This password will be used to unlock your BitBox02."
        )
        while not self._device.set_password():
            eprint("Passwords did not match. please try again")

        print("Your BitBox02 will now create a backup of your wallet...")
        print("Please confirm the date on your device.")
        if not self._device.create_backup():
            eprint("Creating the backup failed")
            return
        print("Backup created sucessfully")

        print("Please Remove SD Card")
        self._device.remove_sdcard()

    def _print_backups(self, backups: Optional[List[bitbox02.Backup]] = None) -> None:
        local_timezone = get_localzone()
        if backups is None:
            backups = list(self._device.list_backups())
        if not backups:
            print("No backups found.")
            return
        fmt = "%Y-%m-%d %H:%M:%S %z"
        for (i, (backup_id, backup_name, date)) in enumerate(backups):
            date = local_timezone.localize(date)
            date_str = date.strftime(fmt)
            print(f"[{i+1}] Backup Name: {backup_name}, Time: {date_str}, ID: {backup_id}")

    def _restore_backup_workflow(self) -> None:
        """TODO: Document"""
        backups = list(self._device.list_backups())
        self._print_backups(backups)
        if not backups:
            return
        ans = input(f"Choose a backup [1-{len(backups)}]: ")
        try:
            item = int(ans)
            if item < 1 or item > len(backups):
                raise ValueError("Out of range")
        except ValueError:
            eprint("Invalid input")
            return
        backup_id, _, _ = backups[item - 1]
        print(f"ID: {backup_id}")
        if not self._device.restore_backup(backup_id):
            print("Restoring backup failed")
            return
        print("Please Remove SD Card")
        self._device.remove_sdcard()

    def _restore_from_mnemonic(self) -> None:
        if self._device.restore_from_mnemonic():
            print("Restore successful")
        else:
            print("Restore was NOT successful")

    def _list_device_info(self) -> None:
        print(f"All info: {self._device.device_info()}")

    def _reboot(self) -> None:
        if self._device.reboot():
            print("Device rebooted")
            self._stop = True
        else:
            print("User aborted")

    def _check_sd_presence(self) -> None:
        print(f"SD Card inserted: {self._device.check_sdcard()}")

    def _display_random(self) -> None:
        print(f"Random number: {self._device.random_number().hex()}")

    def _get_root_fingerprint(self) -> None:
        print(f"Root fingerprint: {self._device.root_fingerprint().hex()}")

    def _display_zpub(self) -> None:
        print(
            "m/84'/0'/0' zpub: ",
            self._device.btc_pub(
                keypath=[84 + HARDENED, 0 + HARDENED, 0 + HARDENED],
                output_type=bitbox02.btc.BTCPubRequest.ZPUB,  # pylint: disable=no-member
            ),
        )

    def _btc_address(self) -> None:
        def address(display: bool) -> str:
            return self._device.btc_pub(
                keypath=[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                output_type=bitbox02.btc.BTCPubRequest.ADDRESS,  # pylint: disable=no-member,
                script_type=bitbox02.btc.SCRIPT_P2WPKH,
                display=display,
            )

        print("m/84'/0'/0'/0/0 address: ", address(False))
        address(True)

    def _sign_btc_tx(self) -> None:
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
                output_type=bitbox02.btc.P2WSH,
                output_hash=b"11111111111111111111111111111111",
                value=int(1e8 * 0.2),
            ),
        ]
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            bitbox02.btc.SCRIPT_P2WPKH,
            bip44_account=bip44_account,
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _check_backup(self) -> None:
        print("Your BitBox02 will now perform a backup check")
        backup_id = self._device.check_backup()
        if backup_id:
            print(f"Check successful. Backup with ID {backup_id} matches")
        else:
            print("No matching backup found")

    def _show_mnemnoic_seed(self) -> None:
        print("Your BitBox02 will now show the mnemonic seed phrase")
        print(self._device.show_mnemonic())

    def _create_backup(self) -> None:
        if self._device.check_backup(silent=True) is not None:
            if input("A backup already exists, continue? Y/n: ") not in ("", "Y", "y"):
                return
        if not self._device.create_backup():
            eprint("Creating the backup failed")
        else:
            print("Backup created sucessfully")

    def _reboot_bootloader(self) -> None:
        if self._device.reboot():
            print("Device rebooted")
            self._stop = True
            return
        print("User aborted")

    def _toggle_mnemonic_passphrase(self) -> None:
        enabled = self._device.device_info()["mnemonic_passphrase_enabled"]
        try:
            if enabled:
                if input("Mnemonic passprase enabled, disable? Y/n: ") not in ("", "Y", "y"):
                    return
                self._device.disable_mnemonic_passphrase()
            else:
                if input("Mnemonic passprase disabled, enable? Y/n: ") not in ("", "Y", "y"):
                    return
                self._device.enable_mnemonic_passphrase()
            enabled = not enabled
        except UserAbortException:
            print("Aborted by user")
        print("Success.")
        if enabled:
            print("You can enter a mnemonic passphrase on the next unlock.")
            print("Replug your BitBox02.")

    def _display_eth_address(self) -> None:
        def address(display: bool = False) -> str:
            return self._device.eth_pub(
                keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
                output_type=bitbox02.eth.ETHPubRequest.ADDRESS,  # pylint: disable=no-member
                display=display,
            )

        print("Ethereum address: {}".format(address(display=False)))
        address(display=True)

    def _reset_device(self) -> None:
        if self._device.reset():
            print("Device RESET")
            self._stop = True
        else:
            print("Device NOT reset")

    def _menu_notinit(self) -> None:
        """TODO: Document

        Returns:
            bool: If the user should be prompted again
        """
        choices = (
            ("Set up a new wallet", self._setup_workflow),
            ("Restore from backup", self._restore_backup_workflow),
            ("Restore from mnemonic", self._restore_from_mnemonic),
            ("List device info", self._list_device_info),
            ("Reboot into bootloader", self._reboot),
            ("Check if SD card inserted", self._check_sd_presence),
        )
        choice = ask_user(choices)
        if isinstance(choice, bool):
            self._stop = True
            return
        if choice is None:
            return
        choice()

    def _menu_init(self) -> None:
        """Print the menu"""
        choices = (
            ("List device info", self._list_device_info),
            ("Change device name", self._change_name_workflow),
            ("Display random number", self._display_random),
            ("Get root fingerprint", self._get_root_fingerprint),
            ("Retrieve zpub of first account", self._display_zpub),
            ("Retrieve a BTC address", self._btc_address),
            ("Sign a BTC tx", self._sign_btc_tx),
            ("List backups", self._print_backups),
            ("Check backup", self._check_backup),
            ("Show mnemonic", self._show_mnemnoic_seed),
            ("Create backup", self._create_backup),
            ("Reboot into bootloader", self._reboot_bootloader),
            ("Check if SD card inserted", self._check_sd_presence),
            ("Toggle BIP39 Mnemonic Passphrase", self._toggle_mnemonic_passphrase),
            ("Retrieve Ethereum address", self._display_eth_address),
            ("Reset Device", self._reset_device),
        )
        choice = ask_user(choices)
        if isinstance(choice, bool):
            self._stop = True
            return
        if choice is None:
            return
        choice()

    def _menu(self) -> None:
        if not self._device.device_info()["initialized"]:
            self._menu_notinit()
            return
        self._menu_init()

    def run(self) -> int:
        """Entry point for program"""
        if self._debug:
            self._device.debug = True

        while not self._stop:
            self._menu()
        self._device.close()
        return 0


class SendMessageBitBoxBase:
    """SendMessageBitBoxBase"""

    # pylint: disable=too-few-public-methods

    def __init__(self, device: BitBoxBase, debug: bool):
        self._device = device
        self._debug = debug
        self._stop = False

    def _reboot(self) -> None:
        if self._device.reboot():
            print("Device rebooted")
        else:
            print("User aborted")

    def _reboot_bootloader(self) -> None:
        if self._device.reboot():
            print("Device rebooted")
            self._stop = True
            return
        print("User aborted")

    def _set_config(self) -> None:
        # pylint: disable=no-member
        ip_str = input("Enter an IP: ")
        ip_bytes = bytes([int(x) for x in ip_str.split(".")])
        hostname = input("Enter hostname: ")
        self._device.set_config(
            bitboxbase.BitBoxBaseSetConfigRequest.LED_ALWAYS, ip_bytes, hostname
        )

    def _heartbeat(self) -> None:
        # pylint: disable=no-member
        print("States:")
        for name, state_enum in bitboxbase.BitBoxBaseHeartbeatRequest.StateCode.items():
            print(f"{state_enum}: {name}")
        state = bitboxbase.BitBoxBaseHeartbeatRequest.StateCode.items()[int(input("state: "))][1]
        for name, desc_enum in bitboxbase.BitBoxBaseHeartbeatRequest.DescriptionCode.items():
            print(f"{desc_enum}: {name}")
        description = bitboxbase.BitBoxBaseHeartbeatRequest.DescriptionCode.items()[
            int(input("description: "))
        ][1]
        self._device.heartbeat(state, description)

    def _confirm_pairing(self) -> None:
        noise = input("noise: ").encode("utf-8")
        # Always give API call 32 bytes of data
        stretch = hashlib.sha256(noise)
        hsh = hashlib.sha256(stretch.digest())
        print(base64.b32encode(hsh.digest())[:20])
        self._device.confirm_pairing(stretch.digest())

    def _menu(self) -> None:
        """Print the menu"""
        choices = (
            ("Reboot into bootloader", self._reboot_bootloader),
            ("Heartbeat", self._heartbeat),
            ("Set config", self._set_config),
            ("Confirm pairing", self._confirm_pairing),
        )
        choice = ask_user(choices)
        if isinstance(choice, bool):
            self._stop = True
            return
        if choice is None:
            return
        choice()

    def run(self) -> int:
        """Entry point for program"""
        if self._debug:
            self._device.debug = True

        while not self._stop:
            self._menu()
        self._device.close()
        return 0


class SendMessageBootloader:
    """Simple test application for bootloader"""

    # pylint: disable=too-few-public-methods
    def __init__(self, device: bitbox02.Bootloader):
        self._device = device
        self._stop = False

    def _boot(self) -> None:
        self._device.reboot()
        self._stop = True

    def _get_versions(self) -> None:
        if self._device.erased():
            print("No firmware on device")
        else:
            version = self._device.versions()
            print(f"Firmware version: {version[0]}, Pubkeys version: {version[1]}")

    def _erase(self) -> None:
        self._device.erase()

    def _show_fw_hash(self) -> None:
        self._device.set_show_firmware_hash(True)

    def _dont_show_fw_hash(self) -> None:
        self._device.set_show_firmware_hash(False)

    def _menu(self) -> None:
        choices = (
            ("Boot", self._boot),
            ("Print versions", self._get_versions),
            ("Erase firmware", self._erase),
            ("Show firmware hash", self._show_fw_hash),
            ("Don't show firmware hash", self._dont_show_fw_hash),
        )
        choice = ask_user(choices)
        if isinstance(choice, bool):
            self._stop = True
            return
        if choice is None:
            return
        choice()

    def run(self) -> int:
        while not self._stop:
            self._menu()
        self._device.close()
        return 0


class U2FApp:
    """App"""

    # pylint: disable=too-few-public-methods
    APPID = "http://example.com"

    def __init__(self, device: u2f.bitbox02.BitBox02U2F, debug: bool):
        self._device = device
        self._stop = False
        self._dev_keyhandle: bytes = b"0" * 64
        self._dev_pubkey: bytes = b"0" * 64
        if debug:
            self._device.debug = True

    def _wink(self) -> None:
        print("Wink")
        self._device.u2fhid_wink()

    def _ping(self) -> None:
        ans = input("Message: ")
        res = self._device.u2fhid_ping(ans.encode("utf-8"))
        print(res.decode("utf-8"))

    def _register(self) -> None:
        try:
            res = self._device.u2f_register(self.APPID)
            if res is not None:
                (self._dev_pubkey, self._dev_keyhandle) = res
        except u2f.ConditionsNotSatisfiedException:
            print("Not registered")

    def _bogus(self) -> None:
        ans = input("Vendor [chromium, firefox]: ")
        try:
            self._device.u2f_register_bogus(ans)
        except ValueError as err:
            print("Invalid vendor, try again: {}".format(err))
        except u2f.ConditionsNotSatisfiedException:
            print("User not present")

    def _authenticate(self) -> None:
        if self._dev_keyhandle == b"0" * 64:
            print("Not yet registered, authenticating anyway...")
        try:
            self._device.u2f_authenticate(self.APPID, self._dev_keyhandle, self._dev_pubkey)
            print("User present")
        except u2f.ConditionsNotSatisfiedException:
            print("User not present")
        except u2f.WrongDataException:
            print("Keyhandle not for this key")

    def _menu(self) -> None:
        """Menu"""
        print("What would you like to do?")
        choices = (
            ("Wink", self._wink),
            ("Ping", self._ping),
            ("Register", self._register),
            ("Register with bogus AppId", self._bogus),
            ("Authenticate", self._authenticate),
        )
        choice = ask_user(choices)
        if isinstance(choice, bool):
            self._stop = True
            return
        if choice is None:
            return
        choice()

    def run(self) -> int:
        """Main function"""
        while not self._stop:
            self._menu()
        self._device.close()
        return 0


def connect_to_usb_bitbox(debug: bool) -> int:
    """
    Connects and runs the main menu on a BitBox02 connected
    over USB.
    """
    try:
        bitbox = devices.get_any_bitbox02()
    except devices.TooManyFoundException:
        print("Multiple bitboxes detected. Only one supported")
        return 1
    except devices.NoneFoundException:
        try:
            bootloader = devices.get_any_bitbox02_bootloader()
        except devices.TooManyFoundException:
            print("Multiple bitbox bootloaders detected. Only one supported")
            return 1
        except devices.NoneFoundException:
            print("Neither bitbox nor bootloader found.")
            return 1
        else:
            hid_device = hid.device()
            hid_device.open_path(bootloader["path"])
            bootloader_connection = bitbox02.Bootloader(u2fhid.U2FHid(hid_device), bootloader)
            boot_app = SendMessageBootloader(bootloader_connection)
            return boot_app.run()
    else:

        def show_pairing(code: str) -> bool:
            print("Please compare and confirm the pairing code on your BitBox02:")
            print(code)
            return True

        def attestation_check(result: bool) -> None:
            if result:
                print("Device attestation PASSED")
            else:
                print("Device attestation FAILED")

        hid_device = hid.device()
        hid_device.open_path(bitbox["path"])
        bitbox_connection = bitbox02.BitBox02(
            transport=u2fhid.U2FHid(hid_device),
            device_info=bitbox,
            show_pairing_callback=show_pairing,
            attestation_check_callback=attestation_check,
        )

        if debug:
            print("Device Info:")
            pprint.pprint(bitbox)
        return SendMessage(bitbox_connection, debug).run()


def connect_to_usart_bitboxbase(debug: bool, serial_port: usart.SerialPort) -> int:
    """
    Connects and runs the main menu over a BitBoxBase connected
    over UART.
    """
    print("Trying to connect to BitBoxBase firmware...")
    bootloader_device: devices.DeviceInfo = get_bitboxbase_default_device(serial_port.port)

    def show_pairing(code: str) -> bool:
        print("(Pairing should be automatic) Pairing code:")
        print(code)
        return True

    def attestation_check(result: bool) -> None:
        if result:
            print("Device attestation PASSED")
        else:
            print("Device attestation FAILED")

    try:
        transport = usart.U2FUsart(serial_port)
        base_dev = BitBoxBase(
            transport,
            bootloader_device,
            show_pairing_callback=show_pairing,
            attestation_check_callback=attestation_check,
        )
        if debug:
            print("Device Info:")
            pprint.pprint(base_dev)
        return SendMessageBitBoxBase(base_dev, debug).run()
    except usart.U2FUsartErrorResponse as err:
        if err.error_code != usart.U2FUsartErrorResponse.ENDPOINT_UNAVAILABLE:
            raise
    except usart.U2FUsartTimeoutError:
        print("Timed out. Maybe the device is not connected?", file=sys.stderr)
        return 1

    print("BitBox unavailable. Starting bootloader connection.")
    transport = usart.U2FUsart(serial_port)
    bootloader = bitbox02.Bootloader(transport, bootloader_device)
    return SendMessageBootloader(bootloader).run()


def main() -> int:
    """Main function"""
    parser = argparse.ArgumentParser(description="Tool for communicating with bitbox device")
    parser.add_argument("--debug", action="store_true", help="Print messages sent and received")
    parser.add_argument("--u2f", action="store_true", help="Use u2f menu instead")
    parser.add_argument(
        "--usart", action="store", help="Use USART (BitBoxBase) on the specified serial port."
    )
    args = parser.parse_args()

    if args.u2f:
        try:
            u2fbitbox = u2f.bitbox02.get_bitbox02_u2f_device()
        except devices.TooManyFoundException:
            print("Multiple bitboxes detected. Only one supported")
        except devices.NoneFoundException:
            print("No bitboxes detected")
        else:
            hid_device = hid.device()
            hid_device.open_path(u2fbitbox["path"])
            u2fdevice = u2f.bitbox02.BitBox02U2F(hid_device)
            u2fapp = U2FApp(u2fdevice, args.debug)
            return u2fapp.run()
        return 1

    if args.usart is not None:
        with usart.SerialPort(args.usart) as serial_port:
            return connect_to_usart_bitboxbase(args.debug, serial_port)
    else:
        return connect_to_usb_bitbox(args.debug)

    return 1


if __name__ == "__main__":
    sys.exit(main())
