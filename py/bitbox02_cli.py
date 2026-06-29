#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Unified BitBox02 command line interface."""

# mypy: disable-error-code=attr-defined

# pylint: disable=too-many-lines,protected-access,too-many-statements,too-many-locals,too-many-return-statements

import argparse
import socket
import pprint
import sys
from time import sleep
from typing import List, Any, Optional, Callable, Union, Tuple, Sequence
import base64
import binascii
import textwrap
import json
from pathlib import Path
import os

import requests
import hid
import semver
from tzlocal import get_localzone

from bitbox02 import util
from bitbox02 import bitbox02
from bitbox02.bitbox02 import Bootloader
from bitbox02.communication import (
    devices,
    HARDENED,
    Bitbox02Exception,
    UserAbortException,
    FirmwareVersionOutdatedException,
    u2fhid,
    bitbox_api_protocol,
    PhysicalLayer,
    TransportLayer,
)

import u2f
import u2f.bitbox02

try:
    # Optional rlp dependency only needed to sign ethereum transactions.
    # pylint: disable=import-error
    import rlp
except ModuleNotFoundError:
    pass


def eprint(*args: Any, **kwargs: Any) -> None:
    """
    Like print, but defaults to stderr.
    """
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


class CliUserAbort(Exception):
    """User rejected a host-side CLI action."""


def ask_user(
    choices: Sequence[Tuple[str, Callable[[], None]]],
) -> Union[Callable[[], None], bool, None]:
    """Ask user to choose one of the choices, q quits"""
    print("What would you like to do?")
    for idx, choice in enumerate(choices):
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


def _btc_demo_inputs_outputs(
    bip44_account: int,
) -> Tuple[List[bitbox02.BTCInputType], List[bitbox02.BTCOutputType]]:
    """
    Returns a sample btc tx.
    """
    inputs: List[bitbox02.BTCInputType] = [
        {
            "prev_out_hash": binascii.unhexlify(
                "c58b7e3f1200e0c0ec9a5e81e925baface2cc1d4715514f2d8205be2508b48ee"
            ),
            "prev_out_index": 0,
            "prev_out_value": int(1e8 * 0.60005),
            "sequence": 0xFFFFFFFF,
            "keypath": [84 + HARDENED, 0 + HARDENED, bip44_account, 0, 0],
            "script_config_index": 0,
            "prev_tx": {
                "version": 1,
                "locktime": 0,
                "inputs": [
                    {
                        "prev_out_hash": b"11111111111111111111111111111111",
                        "prev_out_index": 0,
                        "signature_script": b"some signature script",
                        "sequence": 0xFFFFFFFF,
                    }
                ],
                "outputs": [
                    {
                        "value": int(1e8 * 0.60005),
                        "pubkey_script": b"some pubkey script",
                    }
                ],
            },
        },
        {
            "prev_out_hash": binascii.unhexlify(
                "c58b7e3f1200e0c0ec9a5e81e925baface2cc1d4715514f2d8205be2508b48ee"
            ),
            "prev_out_index": 0,
            "prev_out_value": int(1e8 * 0.60005),
            "sequence": 0xFFFFFFFF,
            "keypath": [49 + HARDENED, 0 + HARDENED, bip44_account, 0, 1],
            "script_config_index": 1,
            "prev_tx": {
                "version": 1,
                "locktime": 0,
                "inputs": [
                    {
                        "prev_out_hash": b"11111111111111111111111111111111",
                        "prev_out_index": 0,
                        "signature_script": b"some signature script",
                        "sequence": 0xFFFFFFFF,
                    }
                ],
                "outputs": [
                    {
                        "value": int(1e8 * 0.60005),
                        "pubkey_script": b"some pubkey script",
                    }
                ],
            },
        },
    ]
    outputs: List[bitbox02.BTCOutputType] = [
        bitbox02.BTCOutputInternal(
            keypath=[84 + HARDENED, 0 + HARDENED, bip44_account, 1, 0],
            value=int(1e8 * 1),
            script_config_index=0,
        ),
        bitbox02.BTCOutputExternal(
            output_type=bitbox02.btc.P2WSH,
            output_payload=b"11111111111111111111111111111111",
            value=int(1e8 * 0.2),
        ),
    ]
    return inputs, outputs


class SendMessage:
    """SendMessage"""

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
        self._change_name_workflow()

        entropy_size_inp = input(
            "Choose a seed size: 32 (24 words) or 16 (12 words). Default is 32: "
        )

        if not entropy_size_inp:
            entropy_size = 32
        else:
            try:
                entropy_size = int(entropy_size_inp)
            except ValueError:
                eprint("Failed")
                return

        if entropy_size not in (16, 32):
            eprint("You must enter 16 or 32")
            return

        print(
            "Please choose a password of the BitBox02. "
            + "This password will be used to unlock your BitBox02."
        )
        while not self._device.set_password(entropy_size=entropy_size):
            eprint("Passwords did not match. please try again")

        print("Your BitBox02 will now create a backup of your wallet...")

        def backup_sd() -> None:
            if not self._device.create_backup():
                eprint("Creating the backup failed")
                return
            print("Backup created sucessfully")
            print("Please Remove SD Card")

        def backup_mnemonic() -> None:
            if self._device.version < semver.VersionInfo(9, 13, 0):
                eprint("Backing up using recovery words is supported from firmware version 9.13.0")
                return

            try:
                self._device.show_mnemonic()
            except Bitbox02Exception:
                eprint("Creating the backup failed")
                return
            print("Backup created sucessfully")

        choice = ask_user(
            (
                ("Backup onto a microSD card", backup_sd),
                ("Backup manually by writing down recovey words", backup_mnemonic),
            ),
        )
        if callable(choice):
            try:
                choice()
            except UserAbortException:
                eprint("Aborted by user")

    def _print_backups(self, backups: Optional[Sequence[bitbox02.Backup]] = None) -> None:
        local_timezone = get_localzone()
        if backups is None:
            backups = list(self._device.list_backups())
        if not backups:
            print("No backups found.")
            return
        fmt = "%Y-%m-%d %H:%M:%S %z"
        for i, (backup_id, backup_name, date) in enumerate(backups):
            date = date.astimezone(local_timezone)
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
        try:
            self._device.restore_backup(backup_id)
        except UserAbortException:
            eprint("Aborted by user")
            return
        except Bitbox02Exception:
            eprint("Restoring backup failed")
            return
        print("Please Remove SD Card")

    def _restore_from_mnemonic(self) -> None:
        try:
            self._device.restore_from_mnemonic()
            print("Restore successful")
        except UserAbortException:
            print("Aborted by user")

    def _list_device_info(self) -> None:
        pprint.pprint(self._device.device_info())

    def _reboot(self) -> None:
        inp = input("Select one of: 1=upgrade; 2=go to startup settings: ").strip()
        purpose = {
            "1": bitbox02.system.RebootRequest.Purpose.UPGRADE,  # pylint: disable=no-member
            "2": bitbox02.system.RebootRequest.Purpose.SETTINGS,  # pylint: disable=no-member
        }[inp]
        if self._device.reboot(purpose=purpose):
            print("Device rebooted")
            self._stop = True
        else:
            print("User aborted")

    def _check_sd_presence(self) -> None:
        print(f"SD Card inserted: {self._device.check_sdcard()}")

    def _insert_sdcard(self) -> None:
        try:
            self._device.insert_sdcard()
        except UserAbortException:
            print("Aborted by user")

    def _get_root_fingerprint(self) -> None:
        print(f"Root fingerprint: {self._device.root_fingerprint().hex()}")

    def _display_zpub(self) -> None:
        try:
            print(
                "m/84'/0'/0' zpub: ",
                self._device.btc_xpub(
                    keypath=[84 + HARDENED, 0 + HARDENED, 0 + HARDENED],
                    xpub_type=bitbox02.btc.BTCPubRequest.ZPUB,  # pylint: disable=no-member
                ),
            )
        except UserAbortException:
            eprint("Aborted by user")

    def _btc_xpubs(self) -> None:
        xpubs = self._device.btc_xpubs(
            keypaths=[[84 + HARDENED, 0 + HARDENED, i + HARDENED] for i in range(20)],
        )
        print("xpubs for m/84'/0'/{0'..19'}:")
        for xpub in xpubs:
            print(xpub)

    def _get_electrum_encryption_key(self) -> None:
        print(
            "Electrum wallet encryption xpub at keypath m/4541509'/1112098098':",
            self._device.electrum_encryption_key(
                keypath=[4541509 + HARDENED, 1112098098 + HARDENED]
            ),
        )

    def _bip85_bip39(self) -> None:
        try:
            self._device.bip85_bip39()
        except UserAbortException:
            print("Aborted by user")

    def _bip85_ln(self) -> None:
        try:
            entropy = self._device.bip85_ln()
            print("Derived entropy for a Breez Lightning wallet:", entropy.hex())
        except UserAbortException:
            print("Aborted by user")

    def _btc_address(self) -> None:
        def address(display: bool) -> str:
            # pylint: disable=no-member
            return self._device.btc_address(
                keypath=[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                script_config=bitbox02.btc.BTCScriptConfig(
                    simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                ),
                display=display,
            )

        print("m/84'/0'/0'/0/0 address: ", address(False))
        address(True)

    def _btc_multisig_config(self, coin: "bitbox02.btc.BTCCoin.V") -> bitbox02.btc.BTCScriptConfig:
        """
        Get a mock multisig 1-of-2 multisig with the current device and some other arbitrary xpub.
        Registers it on the device if not already registered.
        """
        account_keypath = [48 + HARDENED, 0 + HARDENED, 0 + HARDENED, 2 + HARDENED]

        my_xpub = self._device.btc_xpub(
            keypath=account_keypath,
            coin=coin,
            xpub_type=bitbox02.btc.BTCPubRequest.XPUB,
            display=False,
        )
        multisig_config = bitbox02.btc.BTCScriptConfig(
            multisig=bitbox02.btc.BTCScriptConfig.Multisig(
                threshold=1,
                xpubs=[
                    util.parse_xpub(my_xpub),
                    util.parse_xpub(
                        "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazW"
                        "HLSLi23PoBhGcLcYW6JRiJ12zXZ9Aop4LbAqsS3gtcy"
                    ),
                ],
                our_xpub_index=0,
            )
        )

        is_registered = self._device.btc_is_script_config_registered(
            coin, multisig_config, account_keypath
        )
        if is_registered:
            print("Multisig account already registered on the device.")
        else:
            multisig_name = input("Enter a name for the multisig account: ").strip()
            self._device.btc_register_script_config(
                coin=coin,
                script_config=multisig_config,
                keypath=account_keypath,
                name=multisig_name,
            )

        return multisig_config

    def _btc_policy_config(self, coin: "bitbox02.btc.BTCCoin.V") -> bitbox02.btc.BTCScriptConfig:
        account_keypath = [48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 3 + HARDENED]
        our_xpub = self._device.btc_xpub(
            keypath=account_keypath,
            coin=coin,
            xpub_type=bitbox02.btc.BTCPubRequest.XPUB,
            display=False,
        )
        our_root_fingerprint = self._device.root_fingerprint()

        policy_config = bitbox02.btc.BTCScriptConfig(
            policy=bitbox02.btc.BTCScriptConfig.Policy(
                policy="wsh(and_v(v:pk(@0/**),pk(@1/**)))",
                keys=[
                    bitbox02.common.KeyOriginInfo(
                        root_fingerprint=our_root_fingerprint,
                        keypath=account_keypath,
                        xpub=util.parse_xpub(our_xpub),
                    ),
                    bitbox02.common.KeyOriginInfo(
                        xpub=util.parse_xpub(
                            "xpub6Eq64jDihkRvLg91wnckeTFWDT5jzdoKwX24aL9MHY4pS49E9jH69zFRnHuJzZijQaLZs7t5jtUxUhywhXGtUzsCf5EjunnDUNhzJFqhowa"
                        ),
                    ),
                ],
            )
        )

        is_registered = self._device.btc_is_script_config_registered(
            coin,
            policy_config,
            [],
        )
        if is_registered:
            print("Policy account already registered on the device.")
        else:
            policy_name = input("Enter a name for the policy account: ").strip()
            self._device.btc_register_script_config(
                coin=coin,
                script_config=policy_config,
                keypath=[],
                name=policy_name,
            )

        return policy_config

    def _btc_multisig_address(self) -> None:
        try:
            coin = bitbox02.btc.BTC
            print(
                self._device.btc_address(
                    coin=coin,
                    keypath=[
                        48 + HARDENED,
                        0 + HARDENED,
                        0 + HARDENED,
                        2 + HARDENED,
                        1,
                        2,
                    ],
                    script_config=self._btc_multisig_config(coin),
                    display=True,
                )
            )
        except UserAbortException:
            print("Aborted by user")

    def _btc_policy_address(self) -> None:
        try:
            coin = bitbox02.btc.TBTC
            print(
                self._device.btc_address(
                    coin=coin,
                    keypath=[
                        48 + HARDENED,
                        1 + HARDENED,
                        0 + HARDENED,
                        3 + HARDENED,
                        1,
                        2,
                    ],
                    script_config=self._btc_policy_config(coin),
                    display=True,
                )
            )
        except UserAbortException:
            print("Aborted by user")

    def _sign_btc_normal(
        self,
        format_unit: "bitbox02.btc.BTCSignInitRequest.FormatUnit.V" = bitbox02.btc.BTCSignInitRequest.FormatUnit.DEFAULT,
    ) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
            format_unit=format_unit,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_send_to_self_same_account(
        self,
        format_unit: "bitbox02.btc.BTCSignInitRequest.FormatUnit.V" = bitbox02.btc.BTCSignInitRequest.FormatUnit.DEFAULT,
    ) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        outputs[1] = bitbox02.BTCOutputInternal(
            keypath=[84 + HARDENED, 0 + HARDENED, bip44_account, 0, 0],
            value=int(1e8 * 0.2),
            script_config_index=0,
        )
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
            format_unit=format_unit,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_send_to_self_different_account(
        self,
        format_unit: "bitbox02.btc.BTCSignInitRequest.FormatUnit.V" = bitbox02.btc.BTCSignInitRequest.FormatUnit.DEFAULT,
    ) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        outputs[1] = bitbox02.BTCOutputInternal(
            keypath=[84 + HARDENED, 0 + HARDENED, 1 + HARDENED, 0, 0],
            value=int(1e8 * 0.2),
            script_config_index=0,
            output_script_config_index=0,
        )
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
            format_unit=format_unit,
            output_script_configs=[
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, 1 + HARDENED],
                ),
            ],
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_high_fee(self) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        outputs[1].value = int(1e8 * 0.18)
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_multiple_changes(self) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        # Add a change output.
        outputs.append(
            bitbox02.BTCOutputInternal(
                keypath=[84 + HARDENED, 0 + HARDENED, bip44_account, 1, 0],
                value=int(1),
                script_config_index=0,
            )
        )
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_locktime_rbf(self) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        inputs[0]["sequence"] = 0xFFFFFFFF - 2
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
            locktime=10,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_taproot_inputs(self) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        for inp in inputs:
            inp["keypath"] = [86 + HARDENED] + list(inp["keypath"][1:])
            inp["prev_tx"] = None
            inp["script_config_index"] = 0
        for outp in outputs:
            if isinstance(outp, bitbox02.BTCOutputInternal):
                outp.keypath = [86 + HARDENED] + list(outp.keypath[1:])
        script_configs = [
            bitbox02.btc.BTCScriptConfigWithKeypath(
                script_config=bitbox02.btc.BTCScriptConfig(
                    simple_type=bitbox02.btc.BTCScriptConfig.P2TR
                ),
                keypath=[86 + HARDENED, 0 + HARDENED, bip44_account],
            ),
        ]
        assert not bitbox02.btc_sign_needs_prevtxs(script_configs)
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            script_configs,
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_taproot_output(self) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        assert isinstance(outputs[1], bitbox02.BTCOutputExternal)
        outputs[1].type = bitbox02.btc.P2TR
        outputs[1].payload = bytes.fromhex(
            "a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c"
        )
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_policy(self) -> None:
        bip44_account: int = 0 + HARDENED
        account_keypath = [48 + HARDENED, 1 + HARDENED, bip44_account, 3 + HARDENED]
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        for i, inp in enumerate(inputs):
            inp["keypath"] = account_keypath + [0, i]
            inp["script_config_index"] = 0
        assert isinstance(outputs[0], bitbox02.BTCOutputInternal)
        outputs[0].keypath = account_keypath + [1, 0]

        coin = bitbox02.btc.TBTC
        sigs = self._device.btc_sign(
            coin,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=self._btc_policy_config(coin),
                    keypath=account_keypath,
                ),
            ],
            inputs=inputs,
            outputs=outputs,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_op_return(
        self,
        format_unit: "bitbox02.btc.BTCSignInitRequest.FormatUnit.V" = bitbox02.btc.BTCSignInitRequest.FormatUnit.DEFAULT,
    ) -> None:
        # pylint: disable=no-member
        bip44_account: int = 0 + HARDENED
        inputs, outputs = _btc_demo_inputs_outputs(bip44_account)
        outputs.append(
            bitbox02.BTCOutputExternal(
                output_type=bitbox02.btc.OP_RETURN,
                output_payload=b"hello world",
                value=0,
            )
        )
        sigs = self._device.btc_sign(
            bitbox02.btc.BTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 0 + HARDENED, bip44_account],
                ),
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
                    ),
                    keypath=[49 + HARDENED, 0 + HARDENED, bip44_account],
                ),
            ],
            inputs=inputs,
            outputs=outputs,
            format_unit=format_unit,
        )
        for input_index, sig in sigs:
            print("Signature for input {}: {}".format(input_index, sig.hex()))

    def _sign_btc_tx_from_raw(self) -> None:
        """
        Experiment with testnet transactions.
        Uses blockchair.com to convert a testnet transaction to the input required by btc_sign(),
        including the previous transactions.
        """
        # pylint: disable=no-member

        def get(tx_id: str) -> Any:
            return requests.get(
                "https://api.blockchair.com/bitcoin/testnet/dashboards/transaction/{}".format(
                    tx_id
                ),
                timeout=30,
            ).json()["data"][tx_id]

        tx_id = input("Paste a btc testnet tx ID: ").strip()
        tx = get(tx_id)

        inputs: List[bitbox02.BTCInputType] = []
        outputs: List[bitbox02.BTCOutputType] = []

        bip44_account: int = 0 + HARDENED

        for inp in tx["inputs"]:
            print("Downloading prev tx")
            prev_tx = get(inp["transaction_hash"])
            print("Downloaded prev tx")
            prev_inputs: List[bitbox02.BTCPrevTxInputType] = []
            prev_outputs: List[bitbox02.BTCPrevTxOutputType] = []

            for prev_inp in prev_tx["inputs"]:
                prev_inputs.append(
                    {
                        "prev_out_hash": binascii.unhexlify(prev_inp["transaction_hash"])[::-1],
                        "prev_out_index": prev_inp["index"],
                        "signature_script": binascii.unhexlify(prev_inp["spending_signature_hex"]),
                        "sequence": prev_inp["spending_sequence"],
                    }
                )
            for prev_outp in prev_tx["outputs"]:
                prev_outputs.append(
                    {
                        "value": prev_outp["value"],
                        "pubkey_script": binascii.unhexlify(prev_outp["script_hex"]),
                    }
                )

            inputs.append(
                {
                    "prev_out_hash": binascii.unhexlify(inp["transaction_hash"])[::-1],
                    "prev_out_index": inp["index"],
                    "prev_out_value": inp["value"],
                    "sequence": inp["spending_sequence"],
                    "keypath": [84 + HARDENED, 1 + HARDENED, bip44_account, 0, 0],
                    "script_config_index": 0,
                    "prev_tx": {
                        "version": prev_tx["transaction"]["version"],
                        "locktime": prev_tx["transaction"]["lock_time"],
                        "inputs": prev_inputs,
                        "outputs": prev_outputs,
                    },
                }
            )

        for outp in tx["outputs"]:
            outputs.append(
                bitbox02.BTCOutputExternal(
                    # TODO: parse pubkey script
                    output_type=bitbox02.btc.P2WSH,
                    output_payload=b"11111111111111111111111111111111",
                    value=outp["value"],
                )
            )

        print("Start signing...")
        self._device.btc_sign(
            bitbox02.btc.TBTC,
            [
                bitbox02.btc.BTCScriptConfigWithKeypath(
                    script_config=bitbox02.btc.BTCScriptConfig(
                        simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH
                    ),
                    keypath=[84 + HARDENED, 1 + HARDENED, bip44_account],
                )
            ],
            inputs=inputs,
            outputs=outputs,
        )

    def _sign_btc_tx(self) -> None:
        """btc signing demos"""
        choices = (
            ("Normal tx", self._sign_btc_normal),
            (
                "Normal tx, formatted in sats",
                lambda: self._sign_btc_normal(
                    format_unit=bitbox02.btc.BTCSignInitRequest.FormatUnit.SAT
                ),
            ),
            ("Send to self (same account)", self._sign_btc_send_to_self_same_account),
            ("Send to self (different account)", self._sign_btc_send_to_self_different_account),
            ("High fee warning", self._sign_btc_high_fee),
            ("Multiple change outputs", self._sign_btc_multiple_changes),
            ("Locktime/RBF", self._sign_btc_locktime_rbf),
            ("Taproot inputs", self._sign_btc_taproot_inputs),
            ("Taproot output", self._sign_btc_taproot_output),
            ("Policy", self._sign_btc_policy),
            ("OP_RETURN", self._sign_btc_op_return),
            ("From testnet tx ID", self._sign_btc_tx_from_raw),
        )
        choice = ask_user(choices)
        if callable(choice):
            try:
                choice()
            except UserAbortException:
                eprint("Aborted by user")

    def _sign_btc_message(self) -> None:
        # pylint: disable=no-member

        def sign(
            coin: "bitbox02.btc.BTCCoin.V",
            keypath: Sequence[int],
            script_config: bitbox02.btc.BTCScriptConfig,
        ) -> None:
            address = self._device.btc_address(
                coin=coin, keypath=keypath, script_config=script_config, display=False
            )

            print("Address:", address)

            msg = input(r"Message to sign (\n = newline): ")
            if msg.startswith("0x"):
                msg_bytes = binascii.unhexlify(msg[2:])
            else:
                msg_bytes = msg.replace(r"\n", "\n").encode("utf-8")

            try:
                _, _, sig65 = self._device.btc_sign_msg(
                    coin,
                    bitbox02.btc.BTCScriptConfigWithKeypath(
                        script_config=script_config, keypath=keypath
                    ),
                    msg_bytes,
                )
                print("Signature:", base64.b64encode(sig65).decode("ascii"))
            except UserAbortException:
                print("Aborted by user")

        def sign_mainnet() -> None:
            keypath = [49 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0]
            script_config = bitbox02.btc.BTCScriptConfig(
                simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
            )
            sign(bitbox02.btc.BTC, keypath, script_config)

        def sign_testnet() -> None:
            keypath = [49 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0]
            script_config = bitbox02.btc.BTCScriptConfig(
                simple_type=bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH
            )
            sign(bitbox02.btc.TBTC, keypath, script_config)

        choices = (
            ("Mainnet", sign_mainnet),
            ("Testnet", sign_testnet),
        )
        choice = ask_user(choices)
        if callable(choice):
            try:
                choice()
            except UserAbortException:
                eprint("Aborted by user")

    def _check_backup(self) -> None:
        print("Your BitBox02 will now perform a backup check")
        try:
            backup_id = self._device.check_backup()
        except UserAbortException:
            print("Aborted by user")
        else:
            if backup_id:
                print(f"Check successful. Backup with ID {backup_id} matches")
            else:
                print("No matching backup found")

    def _show_mnemnoic_seed(self) -> None:
        print("Your BitBox02 will now show the mnemonic seed phrase")
        try:
            self._device.show_mnemonic()
            print("Success")
        except UserAbortException:
            print("Aborted by user")

    def _create_backup(self) -> None:
        if self._device.check_backup(silent=True) is not None:
            if input("A backup already exists, continue? Y/n: ") not in ("", "Y", "y"):
                return
        try:
            if not self._device.create_backup():
                eprint("Creating the backup failed")
            else:
                print("Backup created sucessfully")
        except UserAbortException:
            print("Aborted by user")

    def _toggle_mnemonic_passphrase(self) -> None:
        enabled = self._device.device_info()["mnemonic_passphrase_enabled"]
        try:
            if enabled:
                if input("Mnemonic passprase enabled, disable? Y/n: ") not in (
                    "",
                    "Y",
                    "y",
                ):
                    return
                self._device.disable_mnemonic_passphrase()
            else:
                if input("Mnemonic passprase disabled, enable? Y/n: ") not in (
                    "",
                    "Y",
                    "y",
                ):
                    return
                self._device.enable_mnemonic_passphrase()
            enabled = not enabled
        except UserAbortException:
            print("Aborted by user")
        print("Success.")
        if enabled:
            print("You can enter a mnemonic passphrase on the next unlock.")
            print("Replug your BitBox02.")

    def _get_eth_xpub(self) -> None:
        try:
            xpub = self._device.eth_pub(
                keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0],
                output_type=bitbox02.eth.ETHPubRequest.XPUB,  # pylint: disable=no-member
                display=False,
            )
        except UserAbortException:
            eprint("Aborted by user")

        print("Ethereum xpub: {}".format(xpub))

    def _display_eth_address(self, contract_address: bytes = b"") -> None:
        def address(display: bool = False) -> str:
            return self._device.eth_pub(
                keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
                output_type=bitbox02.eth.ETHPubRequest.ADDRESS,  # pylint: disable=no-member
                contract_address=contract_address,
                display=display,
            )

        print("Ethereum address: {}".format(address(display=False)))
        try:
            address(display=True)
        except UserAbortException:
            eprint("Aborted by user")

    def _sign_eth_tx(self) -> None:
        # pylint: disable=line-too-long,too-many-branches

        inp = input(
            "Select one of: 1=normal; 2=erc20; 3=erc721; 4=unknown erc20; 5=large data field; 6=BSC; 7=unknown network; 8=eip1559; 9=Arbitrum; 10=streaming (10KB data): "
        ).strip()

        chain_id = 1  # mainnet
        if inp == "6":
            chain_id = 56
        elif inp == "7":
            chain_id = 123456
        elif inp == "9":
            chain_id = 42161  # Arbitrum One

        if inp in ("1", "6", "7", "9"):
            # fmt: off
            tx = bytes([0xf8, 0x6e, 0x82, 0x1f, 0xdc, 0x85, 0x01, 0x65, 0xa0, 0xbc, 0x00, 0x82, 0x52,
            0x08, 0x94, 0x04, 0xf2, 0x64, 0xcf, 0x34, 0x44, 0x03, 0x13, 0xb4, 0xa0, 0x19, 0x2a,
            0x35, 0x28, 0x14, 0xfb, 0xe9, 0x27, 0xb8, 0x85, 0x88, 0x07, 0x5c, 0xf1, 0x25, 0x9e,
            0x9c, 0x40, 0x00, 0x80, 0x25, 0xa0, 0x15, 0xc9, 0x4c, 0x1a, 0x3d, 0xa0, 0xab, 0xc0,
            0xa9, 0x12, 0x4d, 0x28, 0x37, 0x80, 0x9c, 0xcc, 0x49, 0x3c, 0x41, 0x50, 0x4e, 0x45,
            0x71, 0xbc, 0xc3, 0x40, 0xee, 0xb6, 0x8a, 0x91, 0xf6, 0x41, 0xa0, 0x35, 0x99, 0x01,
            0x1d, 0x4c, 0xda, 0x2c, 0x33, 0xdd, 0x3b, 0x00, 0x07, 0x1e, 0xc1, 0x45, 0x33, 0x5e,
            0x5d, 0x2d, 0xd5, 0xed, 0x81, 0x2d, 0x5e, 0xeb, 0xee, 0xcb, 0xa5, 0x26, 0x4e, 0xd1,
            0xbf])
            # fmt: on
        elif inp == "2":
            tx = binascii.unhexlify(
                "f8ac82236785027aca1a808301d04894dac17f958d2ee523a2206206994597c13d831ec780b844a9059cbb000000000000000000000000e6ce0a092a99700cd4ccccbb1fedc39cf53e6330000000000000000000000000000000000000000000000000000000000365c0401ca0265f70103c605eaa1b64c3200d2e7934d7744a3068b377e26c4b080795c744c0a020bbcd34a306621fa8965040390bc240d6d0e3b88915ccdb309d15f1caba81b1"
            )
        elif inp == "3":
            tx = binascii.unhexlify(
                "f87282750b8502cb42fea0830927c0942cab2d282e588f00beabe2bf5577c7644972e10f808b00009470ff1c8de91c861d1ca0f07bca4f43eb1c461ca3cf208e920b60cb393dd37489a14e9f92632acb17dd7ca0694523c8b72052cf6a8b9f93719a664fbbbe59e731cefb1331b2d21ee80b5268"
            )
        elif inp == "4":
            tx = binascii.unhexlify(
                "f8aa81b9843b9aca0083010985949c23d67aea7b95d80942e3836bcdf7e708a747c180b844a9059cbb000000000000000000000000857b3d969eacb775a9f79cabc62ec4bb1d1cd60e000000000000000000000000000000000000000000000098a63cbeb859d027b026a0d3b1a9ba4aff7ebf81dca7dafdbe6d803d174f7805276f45530f2c30e74f5ffca02d86d5290f6ba2c5100e08764d8ab34cf33b03dff3f63219fd839ac9a95f7068"
            )
        elif inp == "5":
            tx = binascii.unhexlify(
                "f9016881b9843b9aca0083010985949c23d67aea7b95d80942e3836bcdf7e708a747c180b90141ef3f3d0b000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000006b175474e89094c44da98b954eedeac495271d0f0000000000000000000000009be6769ef4fc4ccda30e0e39052070d90d3f0bfe000000000000000000000000000000000000000000000000000000e8d4a51000000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000026000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000179fc8e808080"
            )
        elif inp == "8":
            tx = binascii.unhexlify(
                "02f0010184773594008502540be40082520894d61054f4456d0555dc2dd82b77f7ad6074836149865af3107a400080808080"
            )
        elif inp == "10":
            nonce = b"\x01"
            gas_price = b"\x04\xa8\x17\xc8\x00"  # 20 gwei
            gas_limit = b"\x0f\x42\x40"  # 1,000,000
            recipient = (
                b"\x11\x22\x33\x44\x55\x66\x77\x88\x99\xaa\xbb\xcc\xdd\xee\xff\x00\x11\x22\x33\x44"
            )
            value = b""  # Empty for zero value (no leading zeros allowed)
            data = bytes([i % 256 for i in range(10000)])
            v = b"\x25"  # chain_id=1
            r = b"\x01" * 32
            s = b"\x01" * 32
            tx = rlp.encode([nonce, gas_price, gas_limit, recipient, value, data, v, r, s])
            if self._debug:
                print(f"Streaming test transaction: {len(data)} bytes of data")
        else:
            print("None selected")
            return

        try:
            sig = self._device.eth_sign(
                tx,
                keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
                address_case=bitbox02.eth.ETHAddressCase.ETH_ADDRESS_CASE_MIXED,
                chain_id=chain_id,
            )
            print("Signature: {}".format(sig.hex()))
        except UserAbortException:
            eprint("Aborted by user")

    def _sign_eth_message(self) -> None:
        msg = input(r"Message to sign (\n = newline): ")
        if msg.startswith("0x"):
            msg_bytes = binascii.unhexlify(msg[2:])
        else:
            msg_bytes = msg.replace(r"\n", "\n").encode("utf-8")
        msg_hex = binascii.hexlify(msg_bytes).decode("utf-8")
        print(f"signing\nbytes: {repr(msg_bytes)}\nhex: 0x{msg_hex}")
        sig = self._device.eth_sign_msg(
            msg=msg_bytes,
            keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
        )

        print("Signature: 0x{}".format(binascii.hexlify(sig).decode("utf-8")))

    def _sign_eth_typed_message(self) -> None:
        msg = """{
    "types": {
        "EIP712Domain": [
            { "name": "name", "type": "string" },
            { "name": "version", "type": "string" },
            { "name": "chainId", "type": "uint256" },
            { "name": "verifyingContract", "type": "address" }
        ],
        "Attachment": [
            { "name": "contents", "type": "string" }
        ],
        "Person": [
            { "name": "name", "type": "string" },
            { "name": "wallet", "type": "address" },
            { "name": "age", "type": "uint8" }
        ],
        "Mail": [
            { "name": "from", "type": "Person" },
            { "name": "to", "type": "Person" },
            { "name": "contents", "type": "string" },
            { "name": "attachments", "type": "Attachment[]" }
        ]
    },
    "primaryType": "Mail",
    "domain": {
        "name": "Ether Mail",
        "version": "1",
        "chainId": 1,
        "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
    },
    "message": {
        "from": {
            "name": "Cow",
            "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
            "age": 20
        },
        "to": {
            "name": "Bob",
            "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
            "age": "0x1e"
        },
        "contents": "Hello, Bob!",
        "attachments": [{ "contents": "attachment1" }, { "contents": "attachment2" }]
    }
}"""
        print("Signing:\n{}".format(msg))
        sig = self._device.eth_sign_typed_msg(
            keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0], msg=json.loads(msg)
        )

        print("Signature: 0x{}".format(binascii.hexlify(sig).decode("utf-8")))

    def _sign_eth_typed_message_large_data(self) -> None:
        large_data = "0x" + os.urandom(50000).hex()
        msg = {
            "types": {
                "EIP712Domain": [
                    {"name": "chainId", "type": "uint256"},
                    {"name": "verifyingContract", "type": "address"},
                ],
                "SafeTx": [
                    {"name": "to", "type": "address"},
                    {"name": "value", "type": "uint256"},
                    {"name": "data", "type": "bytes"},
                    {"name": "operation", "type": "uint8"},
                    {"name": "safeTxGas", "type": "uint256"},
                    {"name": "baseGas", "type": "uint256"},
                    {"name": "gasPrice", "type": "uint256"},
                    {"name": "gasToken", "type": "address"},
                    {"name": "refundReceiver", "type": "address"},
                    {"name": "nonce", "type": "uint256"},
                ],
            },
            "primaryType": "SafeTx",
            "domain": {
                "chainId": "1",
                "verifyingContract": "0x0000000000000000000000000000000000000000",
            },
            "message": {
                "to": "0x0000000000000000000000000000000000000000",
                "value": "0",
                "data": large_data,
                "operation": "0",
                "safeTxGas": "0",
                "baseGas": "0",
                "gasPrice": "0",
                "gasToken": "0x0000000000000000000000000000000000000000",
                "refundReceiver": "0x0000000000000000000000000000000000000000",
                "nonce": "3",
            },
        }
        print(f"Signing SafeTx with {len(large_data)//2 - 1} bytes of data (streaming)")
        sig = self._device.eth_sign_typed_msg(
            keypath=[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
            msg=msg,
        )
        print("Signature: 0x{}".format(binascii.hexlify(sig).decode("utf-8")))

    def _cardano(self) -> None:
        def xpubs() -> None:
            xpubs = self._device.cardano_xpubs(
                keypaths=[
                    [1852 + HARDENED, 1815 + HARDENED, HARDENED],
                    [1852 + HARDENED, 1815 + HARDENED, HARDENED + 1],
                ]
            )
            print("m/1852'/1815'/0' xpub: ", xpubs[0].hex())
            print("m/1852'/1815'/1' xpub: ", xpubs[1].hex())

        script_config = bitbox02.cardano.CardanoScriptConfig(
            pkh_skh=bitbox02.cardano.CardanoScriptConfig.PkhSkh(
                keypath_payment=[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                keypath_stake=[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
            )
        )

        def get_address(display: bool) -> str:
            return self._device.cardano_address(
                bitbox02.cardano.CardanoAddressRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    display=display,
                    script_config=script_config,
                )
            )

        def address() -> None:
            print("m/1852'/1815'/0'/0/0 address: ", get_address(False))
            get_address(True)

        def sign() -> None:
            response = self._device.cardano_sign_transaction(
                transaction=bitbox02.cardano.CardanoSignTransactionRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    inputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "59864ee73ca5d91098a32b3ce9811bac1996dcbaefa6b6247dcaafb5779c2538"
                            ),
                            prev_out_index=0,
                        )
                    ],
                    outputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address="addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt",
                            value=1000000,
                        ),
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address=get_address(False),
                            value=4829501,
                            script_config=script_config,
                        ),
                    ],
                    fee=170499,
                    ttl=41115811,
                    certificates=[],
                    validity_interval_start=41110811,
                )
            )
            print(response)

        def sign_zero_ttl() -> None:
            response = self._device.cardano_sign_transaction(
                transaction=bitbox02.cardano.CardanoSignTransactionRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    inputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "59864ee73ca5d91098a32b3ce9811bac1996dcbaefa6b6247dcaafb5779c2538"
                            ),
                            prev_out_index=0,
                        )
                    ],
                    outputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address="addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt",
                            value=1000000,
                        ),
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address=get_address(False),
                            value=4829501,
                            script_config=script_config,
                        ),
                    ],
                    fee=170499,
                    ttl=0,
                    allow_zero_ttl=True,
                    certificates=[],
                    validity_interval_start=41110811,
                )
            )
            print(response)

        def sign_tokens() -> None:
            response = self._device.cardano_sign_transaction(
                transaction=bitbox02.cardano.CardanoSignTransactionRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    inputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "59864ee73ca5d91098a32b3ce9811bac1996dcbaefa6b6247dcaafb5779c2538"
                            ),
                            prev_out_index=0,
                        )
                    ],
                    outputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address="addr1q9qfllpxg2vu4lq6rnpel4pvpp5xnv3kvvgtxk6k6wp4ff89xrhu8jnu3p33vnctc9eklee5dtykzyag5penc6dcmakqsqqgpt",
                            value=1000000,
                            asset_groups=[
                                # Asset policy ids and asset names from: https://github.com/cardano-foundation/CIPs/blob/a2ef32d8a2b485fed7f6ffde2781dd58869ff511/CIP-0014/README.md#test-vectors
                                bitbox02.cardano.CardanoSignTransactionRequest.AssetGroup(
                                    policy_id=bytes.fromhex(
                                        "1e349c9bdea19fd6c147626a5260bc44b71635f398b67c59881df209"
                                    ),
                                    tokens=[
                                        # asset1hv4p5tv2a837mzqrst04d0dcptdjmluqvdx9k3
                                        bitbox02.cardano.CardanoSignTransactionRequest.AssetGroup.Token(
                                            asset_name=bytes.fromhex("504154415445"),
                                            value=1,
                                        ),
                                        # asset1aqrdypg669jgazruv5ah07nuyqe0wxjhe2el6f
                                        bitbox02.cardano.CardanoSignTransactionRequest.AssetGroup.Token(
                                            asset_name=bytes.fromhex(
                                                "7eae28af2208be856f7a119668ae52a49b73725e326dc16579dcc373"
                                            ),
                                            value=3,
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address=get_address(False),
                            value=4829501,
                            script_config=script_config,
                        ),
                    ],
                    fee=170499,
                    certificates=[],
                )
            )
            print(response)

        def delegate() -> None:
            response = self._device.cardano_sign_transaction(
                transaction=bitbox02.cardano.CardanoSignTransactionRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    inputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "64c39d60f9d6b4f883d05ae3585d0621d0febc06ad0ea3403bdc00bc23671615"
                            ),
                            prev_out_index=1,
                        ),
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "b7b2333e72f2670ab82051f426cc84000431975a34e71d5edf70ea6c0ddc9bf8"
                            ),
                            prev_out_index=0,
                        ),
                    ],
                    outputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address=get_address(False),
                            value=2741512,
                            script_config=script_config,
                        )
                    ],
                    fee=191681,
                    ttl=41539125,
                    certificates=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Certificate(
                            stake_registration=bitbox02.common.Keypath(
                                keypath=[2147485500, 2147485463, 2147483648, 2, 0]
                            )
                        ),
                        bitbox02.cardano.CardanoSignTransactionRequest.Certificate(
                            stake_delegation=bitbox02.cardano.CardanoSignTransactionRequest.Certificate.StakeDelegation(
                                keypath=[2147485500, 2147485463, 2147483648, 2, 0],
                                pool_keyhash=bytes.fromhex(
                                    "abababababababababababababababababababababababababababab"
                                ),
                            )
                        ),
                    ],
                )
            )
            print(response)

        def delegate_vote() -> None:
            response = self._device.cardano_sign_transaction(
                transaction=bitbox02.cardano.CardanoSignTransactionRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    inputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "b7b2333e72f2670ab82051f426cc84000431975a34e71d5edf70ea6c0ddc9bf8"
                            ),
                            prev_out_index=0,
                        ),
                    ],
                    outputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address=get_address(False),
                            value=2741512,
                            script_config=script_config,
                        )
                    ],
                    fee=191681,
                    ttl=41539125,
                    certificates=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Certificate(
                            vote_delegation=bitbox02.cardano.CardanoSignTransactionRequest.Certificate.VoteDelegation(
                                # keypath used here is the stake credential
                                keypath=[2147485500, 2147485463, 2147483648, 2, 0],
                                type=bitbox02.cardano.CardanoSignTransactionRequest.Certificate.VoteDelegation.CardanoDRepType.ALWAYS_ABSTAIN,
                            )
                        ),
                    ],
                )
            )
            print(response)

        def withdraw() -> None:
            response = self._device.cardano_sign_transaction(
                transaction=bitbox02.cardano.CardanoSignTransactionRequest(
                    network=bitbox02.cardano.CardanoMainnet,
                    inputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Input(
                            keypath=[2147485500, 2147485463, 2147483648, 0, 0],
                            prev_out_hash=bytes.fromhex(
                                "b7b2333e72f2670ab82051f426cc84000431975a34e71d5edf70ea6c0ddc9bf8"
                            ),
                            prev_out_index=0,
                        )
                    ],
                    outputs=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Output(
                            encoded_address=get_address(False),
                            value=4817591,
                            script_config=script_config,
                        )
                    ],
                    fee=175157,
                    ttl=41788708,
                    withdrawals=[
                        bitbox02.cardano.CardanoSignTransactionRequest.Withdrawal(
                            keypath=[2147485500, 2147485463, 2147483648, 2, 0],
                            value=1234567,
                        )
                    ],
                )
            )
            print([(w.public_key.hex(), w.signature.hex()) for w in response.shelley_witnesses])

        choices = (
            ("Retrieve account xpubs", xpubs),
            ("Retrieve a Shelley address", address),
            ("Sign a transaction", sign),
            ("Sign a transaction with TTL=0", sign_zero_ttl),
            ("Sign a transaction sending tokens", sign_tokens),
            ("Delegate staking to a pool", delegate),
            ("Delegate vote to a dRep", delegate_vote),
            ("Withdraw staking rewards", withdraw),
        )
        choice = ask_user(choices)
        if callable(choice):
            try:
                choice()
            except UserAbortException:
                eprint("Aborted by user")

    def _bluetooth_upgrade(self) -> None:
        filename = input("Enter path to the firmware [bitbox-da14531-firmware.bin]: ")
        if filename == "":
            filename = "bitbox-da14531-firmware.bin"
        firmware = Path(filename).read_bytes()
        try:
            self._device.bluetooth_upgrade(firmware)
        except UserAbortException:
            print("Aborted by user")

    def _bluetooth_toggle_enabled(self) -> None:
        try:
            self._device.bluetooth_toggle_enabled()
        except UserAbortException:
            print("Aborted by user")

    def _reset_device(self) -> None:
        if self._device.reset():
            print("Device RESET")
            self._stop = True
        else:
            print("Device NOT reset")

    def _change_password_workflow(self) -> None:
        """Initiate the change password workflow."""
        try:
            self._device.change_password()
            print("Change password workflow completed")
        except UserAbortException:
            eprint("Aborted by user")

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
            ("Upgrade Bluetooth firmware", self._bluetooth_upgrade),
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
            ("Get root fingerprint", self._get_root_fingerprint),
            ("Retrieve zpub of first account", self._display_zpub),
            ("Retrieve multiple xpubs", self._btc_xpubs),
            ("Retrieve a BTC address", self._btc_address),
            ("Retrieve a BTC Multisig address", self._btc_multisig_address),
            ("Retrieve a BTC policy address", self._btc_policy_address),
            ("Sign a BTC tx", self._sign_btc_tx),
            ("Sign a BTC Message", self._sign_btc_message),
            ("List backups", self._print_backups),
            ("Check backup", self._check_backup),
            ("Show mnemonic", self._show_mnemnoic_seed),
            ("Create backup", self._create_backup),
            ("Reboot into bootloader", self._reboot),
            ("Check if SD card inserted", self._check_sd_presence),
            ("Insert SD card", self._insert_sdcard),
            ("Toggle BIP39 Mnemonic Passphrase", self._toggle_mnemonic_passphrase),
            ("Retrieve Ethereum xpub", self._get_eth_xpub),
            ("Retrieve Ethereum address", self._display_eth_address),
            (
                "Retrieve ERC20 address with long token name",
                lambda: self._display_eth_address(
                    contract_address=b"\xba\x11\xd0\x0c\x5f\x74\x25\x5f\x56\xa5\xe3\x66\xf4\xf7\x7f\x5a\x18\x6d\x7f\x55"
                ),
            ),
            ("Sign Ethereum tx", self._sign_eth_tx),
            ("Sign Ethereum Message", self._sign_eth_message),
            ("Sign Ethereum Typed Message (EIP-712)", self._sign_eth_typed_message),
            (
                "Sign Ethereum Typed Message (50KB streaming) ",
                self._sign_eth_typed_message_large_data,
            ),
            ("Cardano", self._cardano),
            ("Show Electrum wallet encryption key", self._get_electrum_encryption_key),
            ("BIP85 - BIP39", self._bip85_bip39),
            ("BIP85 - LN", self._bip85_ln),
            ("Upgrade Bluetooth firmware", self._bluetooth_upgrade),
            ("Toggle bluetooth", self._bluetooth_toggle_enabled),
            ("Reset Device", self._reset_device),
            ("Change Password", self._change_password_workflow),
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

    def close(self) -> None:
        """Close the underlying device connection."""
        self._device.close()

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

    def __init__(self, device: Bootloader):
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

    def _get_hardware(self) -> None:
        secure_chip = self._device.hardware()["secure_chip_model"]
        print("Hardware variant:")
        print(f"- Secure Chip: {secure_chip.value}")

    def _erase(self) -> None:
        self._device.erase()

    def _show_fw_hash(self) -> None:
        self._device.set_show_firmware_hash(True)

    def _dont_show_fw_hash(self) -> None:
        self._device.set_show_firmware_hash(False)

    def _get_hashes(self) -> None:
        firmware_hash, sigkeys_hash = self._device.get_hashes()
        print("Firmware hash:")
        print("\n".join(textwrap.wrap(firmware_hash.hex(), 16)))
        if input("Display on device? y/[n]: ") == "y":
            self._device.get_hashes(display_firmware_hash=True)
        print("Signature keys hash:")
        print("\n".join(textwrap.wrap(sigkeys_hash.hex(), 16)))
        if input("Display on device? y/[n]: ") == "y":
            self._device.get_hashes(display_signing_keydata_hash=True)

    def _menu(self) -> None:
        choices = (
            ("Boot", self._boot),
            ("Print versions", self._get_versions),
            ("Print hardware variant", self._get_hardware),
            ("Erase firmware", self._erase),
            ("Show firmware hash at startup", self._show_fw_hash),
            ("Don't show firmware hash at startup", self._dont_show_fw_hash),
            ("Get firmware & sigkey hashes", self._get_hashes),
            ("Rotate screen", self._device.screen_rotate),
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

    def close(self) -> None:
        """Close the underlying bootloader connection."""
        self._device.close()


class U2FApp:
    """App"""

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

def _connect_to_simulator_app(debug: bool, port: int) -> SendMessage:
    """
    Connects on host computer, simulating a BitBox02 connected over USB.
    """

    class Simulator(PhysicalLayer):
        """
        Simulator class handles the communication
        with the firmware simulator
        """

        def __init__(self) -> None:
            self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.client_socket.connect(("127.0.0.1", port))
            if debug:
                print("Connected to the simulator")

        def write(self, data: bytes) -> None:
            self.client_socket.send(data[1:])
            if debug:
                print(f"Written to the simulator:\n{data.hex()[2:]}")

        def read(self, size: int, timeout_ms: int) -> bytes:
            res = self.client_socket.recv(64)
            if debug:
                print(f"Read from the simulator:\n{res.hex()}")
            return res

        def close(self) -> None:
            return None

        def __del__(self) -> None:
            print("Simulator quit")
            self.client_socket.close()

    simulator = Simulator()
    noise_config = bitbox_api_protocol.BitBoxNoiseConfig()
    bitbox_connection = bitbox02.BitBox02(
        transport=u2fhid.U2FHid(simulator),
        device_info=None,
        noise_config=noise_config,
    )
    try:
        bitbox_connection.check_min_version()
    except FirmwareVersionOutdatedException as exc:
        print("WARNING: ", exc)

    return SendMessage(bitbox_connection, debug)


def connect_to_simulator_bitbox(debug: bool, port: int) -> int:
    """
    Connects and runs the main menu on host computer,
    simulating a BitBox02 connected over USB.
    """
    return _connect_to_simulator_app(debug, port).run()


def _noise_config(
    use_cache: bool, cache_name: str, ask_host_to_accept_pairing: bool
) -> bitbox_api_protocol.BitBoxNoiseConfig:
    def show_pairing(code: str, device_response: Callable[[], bool]) -> bool:
        print("Please compare and confirm the pairing code on your BitBox02:")
        print(code)
        if not device_response():
            return False
        if ask_host_to_accept_pairing:
            return input("Accept pairing? [y]/n: ").strip() != "n"
        return True

    class NoiseConfig(util.NoiseConfigUserCache):
        """NoiseConfig extends NoiseConfigUserCache"""

        def __init__(self) -> None:
            super().__init__(cache_name)

        def show_pairing(self, code: str, device_response: Callable[[], bool]) -> bool:
            return show_pairing(code, device_response)

        def attestation_check(self, result: bool) -> None:
            if result:
                print("Device attestation PASSED")
            else:
                print("Device attestation FAILED")

    class NoiseConfigNoCache(bitbox_api_protocol.BitBoxNoiseConfig):
        """NoiseConfig extends BitBoxNoiseConfig"""

        def show_pairing(self, code: str, device_response: Callable[[], bool]) -> bool:
            return show_pairing(code, device_response)

        def attestation_check(self, result: bool) -> None:
            if result:
                print("Device attestation PASSED")
            else:
                print("Device attestation FAILED")

    if use_cache:
        return NoiseConfig()
    return NoiseConfigNoCache()


def _connect_to_usb_bitbox_app(
    debug: bool, use_cache: bool, ask_host_to_accept_pairing: bool
) -> SendMessage:
    try:
        bitbox = devices.get_any_bitbox02()
    except devices.TooManyFoundException as exc:
        raise RuntimeError("Multiple bitboxes detected. Only one supported") from exc
    except devices.NoneFoundException as exc:
        raise RuntimeError("No bitbox firmware device found") from exc

    hid_device = hid.device()
    try:
        hid_device.open_path(bitbox["path"])
    except OSError as exc:
        raise RuntimeError(
            "Could not connect to the BitBox, device may be already connected to another app."
        ) from exc

    bitbox_connection = bitbox02.BitBox02(
        transport=u2fhid.U2FHid(hid_device),
        device_info=bitbox,
        noise_config=_noise_config(
            use_cache, "shift/send_message", ask_host_to_accept_pairing
        ),
    )
    try:
        bitbox_connection.check_min_version()
    except FirmwareVersionOutdatedException as exc:
        print("WARNING: ", exc)

    if debug:
        print("Device Info:")
        pprint.pprint(bitbox)
    return SendMessage(bitbox_connection, debug)


def _connect_to_usb_bootloader_app(debug: bool) -> SendMessageBootloader:
    try:
        bootloader = devices.get_any_bitbox02_bootloader()
    except devices.TooManyFoundException as exc:
        raise RuntimeError("Multiple bitbox bootloaders detected. Only one supported") from exc
    except devices.NoneFoundException as exc:
        raise RuntimeError("No bitbox bootloader found") from exc

    hid_device = hid.device()
    try:
        hid_device.open_path(bootloader["path"])
    except OSError as exc:
        raise RuntimeError(
            "Could not connect to the BitBox, device may be already connected to another app."
        ) from exc
    bootloader_connection = Bootloader(u2fhid.U2FHid(hid_device), bootloader)
    if debug:
        print("Bootloader Device Info:")
        pprint.pprint(bootloader)
    return SendMessageBootloader(bootloader_connection)


def connect_to_usb_bitbox(debug: bool, use_cache: bool) -> int:
    """
    Connects and runs the main menu on a BitBox02 connected
    over USB.
    """
    try:
        app = _connect_to_usb_bitbox_app(debug, use_cache, ask_host_to_accept_pairing=True)
    except RuntimeError as exc:
        if str(exc) != "No bitbox firmware device found":
            print(exc)
            return 1
        try:
            bootloader = devices.get_any_bitbox02_bootloader()
        except devices.TooManyFoundException:
            print("Multiple bitbox bootloaders detected. Only one supported")
            return 1
        except devices.NoneFoundException:
            print("Neither bitbox nor bootloader found.")
            return 1
        hid_device = hid.device()
        try:
            hid_device.open_path(bootloader["path"])
        except OSError:
            print(
                "Could not connect to the BitBox, device may be already connected to another app."
            )
            return 1
        bootloader_connection = Bootloader(u2fhid.U2FHid(hid_device), bootloader)
        boot_app = SendMessageBootloader(bootloader_connection)
        return boot_app.run()
    return app.run()


def _json_default(value: Any) -> Any:
    if isinstance(value, bytes):
        return value.hex()
    if hasattr(value, "isoformat"):
        return value.isoformat()
    return str(value)


def _print_json(data: Any) -> None:
    print(json.dumps(data, default=_json_default, sort_keys=True))


def _parse_bool(value: str) -> bool:
    normalized = value.lower()
    if normalized in ("1", "true", "yes", "y", "on"):
        return True
    if normalized in ("0", "false", "no", "n", "off"):
        return False
    raise argparse.ArgumentTypeError("expected true or false")


def _parse_keypath(value: str) -> List[int]:
    keypath = value.strip()
    if keypath in ("", "m"):
        return []
    if keypath.startswith("m/"):
        keypath = keypath[2:]
    result = []
    for component in keypath.split("/"):
        hardened = component.endswith(("'", "h", "H"))
        if hardened:
            component = component[:-1]
        if component == "":
            raise argparse.ArgumentTypeError(f"invalid keypath component in {value!r}")
        try:
            child = int(component, 10)
        except ValueError as exc:
            raise argparse.ArgumentTypeError(
                f"invalid keypath component {component!r}"
            ) from exc
        if child < 0 or child >= HARDENED:
            raise argparse.ArgumentTypeError(f"keypath component out of range: {component!r}")
        result.append(child + HARDENED if hardened else child)
    return result


def _parse_hex(value: str) -> bytes:
    hex_value = value.removeprefix("0x")
    try:
        return bytes(binascii.unhexlify(hex_value))
    except binascii.Error as exc:
        raise argparse.ArgumentTypeError("invalid hex string") from exc


def _message_bytes(args: argparse.Namespace) -> bytes:
    message = args.message
    if message is not None:
        return str(message).replace(r"\n", "\n").encode("utf-8")
    message_hex = args.message_hex
    if message_hex is not None:
        return _parse_hex(str(message_hex))
    return Path(str(args.message_file)).read_bytes()


def _btc_coin(value: str) -> "bitbox02.btc.BTCCoin.V":
    return {
        "btc": bitbox02.btc.BTC,
        "tbtc": bitbox02.btc.TBTC,
    }[value]


def _btc_script_config(value: str) -> bitbox02.btc.BTCScriptConfig:
    script_types = {
        "p2wpkh": bitbox02.btc.BTCScriptConfig.P2WPKH,
        "p2wpkh-p2sh": bitbox02.btc.BTCScriptConfig.P2WPKH_P2SH,
        "p2tr": bitbox02.btc.BTCScriptConfig.P2TR,
    }
    return bitbox02.btc.BTCScriptConfig(simple_type=script_types[value])


def _btc_xpub_type(value: str) -> "bitbox02.btc.BTCPubRequest.XPubType.V":
    xpub_types = {
        "tpub": bitbox02.btc.BTCPubRequest.TPUB,
        "xpub": bitbox02.btc.BTCPubRequest.XPUB,
        "ypub": bitbox02.btc.BTCPubRequest.YPUB,
        "zpub": bitbox02.btc.BTCPubRequest.ZPUB,
        "vpub": bitbox02.btc.BTCPubRequest.VPUB,
        "upub": bitbox02.btc.BTCPubRequest.UPUB,
    }
    return xpub_types[value]


def _default_btc_keypath(
    coin: "bitbox02.btc.BTCCoin.V", script: str, account_only: bool
) -> List[int]:
    coin_index = 1 if coin == bitbox02.btc.TBTC else 0
    purpose = {
        "p2wpkh": 84,
        "p2wpkh-p2sh": 49,
        "p2tr": 86,
    }[script]
    keypath = [purpose + HARDENED, coin_index + HARDENED, 0 + HARDENED]
    if account_only:
        return keypath
    return keypath + [0, 0]


def _cmd_device_info(app: SendMessage, args: argparse.Namespace) -> None:
    info = app._device.device_info()
    if args.json:
        _print_json(info)
    else:
        pprint.pprint(info)


def _cmd_device_name_set(app: SendMessage, args: argparse.Namespace) -> None:
    app._change_name_workflow(args.name)


def _cmd_device_root_fingerprint(app: SendMessage, args: argparse.Namespace) -> None:
    fingerprint = app._device.root_fingerprint().hex()
    if args.json:
        _print_json({"root_fingerprint": fingerprint})
    else:
        print(f"Root fingerprint: {fingerprint}")


def _cmd_device_reboot(app: SendMessage, args: argparse.Namespace) -> None:
    # pylint: disable=no-member
    purpose = {
        "upgrade": bitbox02.system.RebootRequest.Purpose.UPGRADE,
        "settings": bitbox02.system.RebootRequest.Purpose.SETTINGS,
    }[args.purpose]
    if not app._device.reboot(purpose=purpose):
        raise CliUserAbort()
    print("Device rebooted")


def _cmd_sd_status(app: SendMessage, args: argparse.Namespace) -> None:
    inserted = app._device.check_sdcard()
    if args.json:
        _print_json({"inserted": inserted})
    else:
        print(f"SD Card inserted: {inserted}")


def _cmd_sd_insert(app: SendMessage, _args: argparse.Namespace) -> None:
    app._device.insert_sdcard()


def _cmd_backup_list(app: SendMessage, args: argparse.Namespace) -> None:
    backups = list(app._device.list_backups())
    if args.json:
        _print_json(
            [
                {"id": backup_id, "name": backup_name, "date": date}
                for backup_id, backup_name, date in backups
            ]
        )
    else:
        app._print_backups(backups)


def _cmd_backup_check(app: SendMessage, args: argparse.Namespace) -> None:
    backup_id = app._device.check_backup()
    if args.json:
        _print_json({"backup_id": backup_id})
    elif backup_id:
        print(f"Check successful. Backup with ID {backup_id} matches")
    else:
        print("No matching backup found")


def _cmd_backup_create(app: SendMessage, args: argparse.Namespace) -> None:
    if not args.force and app._device.check_backup(silent=True) is not None:
        raise RuntimeError("A backup already exists; pass --force to create another one")
    if not app._device.create_backup():
        raise RuntimeError("Creating the backup failed")
    print("Backup created sucessfully")


def _cmd_backup_restore(app: SendMessage, args: argparse.Namespace) -> None:
    app._device.restore_backup(args.backup_id)
    print("Please Remove SD Card")


def _cmd_btc_address(app: SendMessage, args: argparse.Namespace) -> None:
    coin = _btc_coin(args.coin)
    keypath = args.keypath or _default_btc_keypath(coin, args.script, account_only=False)
    address = app._device.btc_address(
        coin=coin,
        keypath=keypath,
        script_config=_btc_script_config(args.script),
        display=args.display,
    )
    if args.json:
        _print_json({"address": address})
    else:
        print(address)


def _cmd_btc_xpub(app: SendMessage, args: argparse.Namespace) -> None:
    xpub = app._device.btc_xpub(
        coin=_btc_coin(args.coin),
        keypath=args.keypath,
        xpub_type=_btc_xpub_type(args.type),
        display=args.display,
    )
    if args.json:
        _print_json({"xpub": xpub})
    else:
        print(xpub)


def _cmd_btc_sign_message(app: SendMessage, args: argparse.Namespace) -> None:
    coin = _btc_coin(args.coin)
    keypath = args.keypath or _default_btc_keypath(coin, args.script, account_only=False)
    script_config = _btc_script_config(args.script)
    address = app._device.btc_address(
        coin=coin, keypath=keypath, script_config=script_config, display=False
    )
    _, _, sig65 = app._device.btc_sign_msg(
        coin,
        bitbox02.btc.BTCScriptConfigWithKeypath(script_config=script_config, keypath=keypath),
        _message_bytes(args),
    )
    signature = base64.b64encode(sig65).decode("ascii")
    if args.json:
        _print_json({"address": address, "signature": signature})
    else:
        print("Address:", address)
        print("Signature:", signature)


def _default_eth_keypath(account_only: bool) -> List[int]:
    keypath = [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0]
    if account_only:
        return keypath
    return keypath + [0]


def _cmd_eth_address(app: SendMessage, args: argparse.Namespace) -> None:
    address = app._device.eth_pub(
        keypath=args.keypath or _default_eth_keypath(account_only=False),
        output_type=bitbox02.eth.ETHPubRequest.ADDRESS,
        contract_address=_parse_hex(args.contract_address) if args.contract_address else b"",
        display=args.display,
    )
    if args.json:
        _print_json({"address": address})
    else:
        print(address)


def _cmd_eth_xpub(app: SendMessage, args: argparse.Namespace) -> None:
    xpub = app._device.eth_pub(
        keypath=args.keypath or _default_eth_keypath(account_only=True),
        output_type=bitbox02.eth.ETHPubRequest.XPUB,
        display=args.display,
    )
    if args.json:
        _print_json({"xpub": xpub})
    else:
        print(xpub)


def _cmd_eth_sign_message(app: SendMessage, args: argparse.Namespace) -> None:
    sig = app._device.eth_sign_msg(
        msg=_message_bytes(args),
        keypath=args.keypath or _default_eth_keypath(account_only=False),
    )
    signature = "0x" + binascii.hexlify(sig).decode("utf-8")
    if args.json:
        _print_json({"signature": signature})
    else:
        print("Signature:", signature)


def _cmd_eth_sign_typed_message(app: SendMessage, args: argparse.Namespace) -> None:
    if args.json_file is not None:
        msg = json.loads(Path(args.json_file).read_text(encoding="utf-8"))
    else:
        msg = json.loads(args.json_string)
    sig = app._device.eth_sign_typed_msg(
        keypath=args.keypath or _default_eth_keypath(account_only=False),
        msg=msg,
    )
    signature = "0x" + binascii.hexlify(sig).decode("utf-8")
    if args.json:
        _print_json({"signature": signature})
    else:
        print("Signature:", signature)


def _cmd_bootloader_versions(app: SendMessageBootloader, args: argparse.Namespace) -> None:
    if app._device.erased():
        if args.json:
            _print_json({"erased": True})
        else:
            print("No firmware on device")
        return
    firmware_version, pubkeys_version = app._device.versions()
    if args.json:
        _print_json(
            {
                "erased": False,
                "firmware_version": firmware_version,
                "signing_pubkeys_version": pubkeys_version,
            }
        )
    else:
        print(f"Firmware version: {firmware_version}, Pubkeys version: {pubkeys_version}")


def _cmd_bootloader_hardware(app: SendMessageBootloader, args: argparse.Namespace) -> None:
    hardware = app._device.hardware()
    if args.json:
        _print_json(hardware)
    else:
        secure_chip = hardware["secure_chip_model"]
        print("Hardware variant:")
        print(f"- Secure Chip: {secure_chip.value}")


def _cmd_bootloader_hashes(app: SendMessageBootloader, args: argparse.Namespace) -> None:
    firmware_hash, sigkeys_hash = app._device.get_hashes(
        display_firmware_hash=args.display_firmware_hash,
        display_signing_keydata_hash=args.display_signing_keydata_hash,
    )
    if args.json:
        _print_json(
            {
                "firmware_hash": firmware_hash.hex(),
                "signing_keydata_hash": sigkeys_hash.hex(),
            }
        )
    else:
        print("Firmware hash:")
        print("\n".join(textwrap.wrap(firmware_hash.hex(), 16)))
        print("Signature keys hash:")
        print("\n".join(textwrap.wrap(sigkeys_hash.hex(), 16)))


def _cmd_bootloader_set_show_firmware_hash(
    app: SendMessageBootloader, args: argparse.Namespace
) -> None:
    app._device.set_show_firmware_hash(args.enabled)


def _cmd_bootloader_erase(app: SendMessageBootloader, _args: argparse.Namespace) -> None:
    app._device.erase()


def _cmd_bootloader_reboot(app: SendMessageBootloader, _args: argparse.Namespace) -> None:
    app._device.reboot()


def _get_bitbox_and_reboot(use_cache: bool) -> devices.DeviceInfo:
    device = devices.get_any_bitbox02()
    hid_device = hid.device()
    hid_device.open_path(device["path"])
    bitbox = bitbox02.BitBox02(
        transport=u2fhid.U2FHid(hid_device),
        device_info=device,
        noise_config=_noise_config(
            use_cache, "shift/load_firmware", ask_host_to_accept_pairing=False
        ),
    )
    if not bitbox.reboot():
        raise CliUserAbort()

    while True:
        try:
            return devices.get_any_bitbox02_bootloader()
        except devices.NoneFoundException:
            sys.stdout.write(".")
            sys.stdout.flush()
            sleep(1)


def _find_and_open_usb_bitbox02_bootloader(
    use_cache: bool,
) -> Tuple[devices.DeviceInfo, TransportLayer]:
    bootloader_device = None
    try:
        bootloader_device = devices.get_any_bitbox02_bootloader()
    except devices.TooManyFoundException as exc:
        raise RuntimeError(
            "Found multiple bb02 bootloader standard editions. Only one supported."
        ) from exc
    except devices.NoneFoundException:
        pass

    if bootloader_device is None:
        try:
            bootloader_device = _get_bitbox_and_reboot(use_cache)
        except devices.TooManyFoundException as exc:
            raise RuntimeError("Found multiple bitboxes. Only one supported.") from exc
        except devices.NoneFoundException as exc:
            raise RuntimeError("Neither bootloader nor bitbox found.") from exc

    pprint.pprint(bootloader_device)
    hid_device = hid.device()
    hid_device.open_path(bootloader_device["path"])
    return bootloader_device, u2fhid.U2FHid(hid_device)


def _cmd_firmware_flash(args: argparse.Namespace) -> None:
    firmware_path = Path(args.firmware)
    if not args.unsigned and ".signed.bin" not in firmware_path.name:
        raise RuntimeError("Expecting firmware to end with '.signed.bin'")

    bootloader_device, transport = _find_and_open_usb_bitbox02_bootloader(not args.no_cache)
    bootloader = Bootloader(transport, bootloader_device)
    firmware = firmware_path.read_bytes()

    def progress(perc: float) -> None:
        sys.stdout.write(f"{perc*100:.02f}%\r")

    if bootloader.erased():
        print("device contains NO firmware")
    else:
        print("firmware version: %d\nsigning pubkeys version: %d" % bootloader.versions())
        firmware_hash, signing_keydata_hash = bootloader.get_hashes()
        print("firmware hash:", firmware_hash.hex())
        print("signing keydata hash:", signing_keydata_hash.hex())

    if args.unsigned:
        bootloader.flash_unsigned_firmware(firmware, progress)
    else:
        bootloader.flash_signed_firmware(firmware, progress)
    print()

    if not args.no_reboot:
        sleep(1)
        bootloader.reboot()


def _run_firmware_command(args: argparse.Namespace) -> int:
    args.handler(args)
    return 0


def _run_bootloader_command(args: argparse.Namespace) -> int:
    app = _connect_to_usb_bootloader_app(args.debug)
    try:
        args.handler(app, args)
    finally:
        app.close()
    return 0


def _run_bitbox_command(args: argparse.Namespace) -> int:
    app = (
        _connect_to_simulator_app(args.debug, args.simulator_port)
        if args.simulator
        else _connect_to_usb_bitbox_app(
            args.debug, not args.no_cache, ask_host_to_accept_pairing=False
        )
    )
    try:
        if args.debug:
            app._device.debug = True
        args.handler(app, args)
    finally:
        app.close()
    return 0


def _add_message_input_args(parser: argparse.ArgumentParser) -> None:
    message = parser.add_mutually_exclusive_group(required=True)
    message.add_argument("--message", help=r"UTF-8 message; literal \n is converted to newline")
    message.add_argument("--message-hex", help="Hex-encoded message bytes, with or without 0x")
    message.add_argument("--message-file", help="Read message bytes from this file")


def _add_common_btc_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--coin", choices=("btc", "tbtc"), default="btc")
    parser.add_argument(
        "--script", choices=("p2wpkh", "p2wpkh-p2sh", "p2tr"), default="p2wpkh"
    )
    parser.add_argument("--keypath", type=_parse_keypath)


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Tool for communicating with BitBox02 devices")
    parser.add_argument("--debug", action="store_true", help="Print messages sent and received")
    parser.add_argument("--u2f", action="store_true", help="Use u2f menu instead")
    parser.add_argument(
        "--simulator",
        action="store_true",
        help="Connect to the BitBox02 simulator instead of a real BitBox02",
    )
    parser.add_argument("--simulator-port", default=15423, type=int, help="Simulator port")
    parser.add_argument(
        "--no-cache", action="store_true", help="Don't use cached or store noise keys"
    )
    parser.add_argument("--json", action="store_true", help="Print machine-readable JSON")
    parser.add_argument(
        "-i", "--interactive", action="store_true", help="Run the interactive menu"
    )

    subparsers = parser.add_subparsers(dest="group")

    device = subparsers.add_parser("device", help="Device commands")
    device_sub = device.add_subparsers(dest="device_command")
    device_sub.add_parser("info").set_defaults(handler=_cmd_device_info, command_target="bitbox")
    name = device_sub.add_parser("name")
    name_sub = name.add_subparsers(dest="name_command", required=True)
    name_set = name_sub.add_parser("set")
    name_set.add_argument("--name", required=True)
    name_set.set_defaults(handler=_cmd_device_name_set, command_target="bitbox")
    device_sub.add_parser("root-fingerprint").set_defaults(
        handler=_cmd_device_root_fingerprint, command_target="bitbox"
    )
    reboot = device_sub.add_parser("reboot")
    reboot.add_argument("--purpose", choices=("upgrade", "settings"), required=True)
    reboot.set_defaults(handler=_cmd_device_reboot, command_target="bitbox")

    sd = subparsers.add_parser("sd", help="SD card commands")
    sd_sub = sd.add_subparsers(dest="sd_command")
    sd_sub.add_parser("status").set_defaults(handler=_cmd_sd_status, command_target="bitbox")
    sd_sub.add_parser("insert").set_defaults(handler=_cmd_sd_insert, command_target="bitbox")

    backup = subparsers.add_parser("backup", help="Backup commands")
    backup_sub = backup.add_subparsers(dest="backup_command")
    backup_sub.add_parser("list").set_defaults(handler=_cmd_backup_list, command_target="bitbox")
    backup_sub.add_parser("check").set_defaults(handler=_cmd_backup_check, command_target="bitbox")
    backup_create = backup_sub.add_parser("create")
    backup_create.add_argument("--force", action="store_true")
    backup_create.set_defaults(handler=_cmd_backup_create, command_target="bitbox")
    backup_restore = backup_sub.add_parser("restore")
    backup_restore.add_argument("--backup-id", required=True)
    backup_restore.set_defaults(handler=_cmd_backup_restore, command_target="bitbox")

    btc = subparsers.add_parser("btc", help="Bitcoin commands")
    btc_sub = btc.add_subparsers(dest="btc_command")
    btc_address = btc_sub.add_parser("address")
    _add_common_btc_args(btc_address)
    btc_address.add_argument("--display", action="store_true")
    btc_address.set_defaults(handler=_cmd_btc_address, command_target="bitbox")
    btc_xpub = btc_sub.add_parser("xpub")
    btc_xpub.add_argument("--coin", choices=("btc", "tbtc"), default="btc")
    btc_xpub.add_argument("--keypath", type=_parse_keypath, required=True)
    btc_xpub.add_argument(
        "--type", choices=("tpub", "xpub", "ypub", "zpub", "vpub", "upub"), default="xpub"
    )
    btc_xpub.add_argument("--display", action="store_true")
    btc_xpub.set_defaults(handler=_cmd_btc_xpub, command_target="bitbox")
    btc_sign_message = btc_sub.add_parser("sign-message")
    _add_common_btc_args(btc_sign_message)
    _add_message_input_args(btc_sign_message)
    btc_sign_message.set_defaults(handler=_cmd_btc_sign_message, command_target="bitbox")

    eth = subparsers.add_parser("eth", help="Ethereum commands")
    eth_sub = eth.add_subparsers(dest="eth_command")
    eth_address = eth_sub.add_parser("address")
    eth_address.add_argument("--keypath", type=_parse_keypath)
    eth_address.add_argument("--contract-address")
    eth_address.add_argument("--display", action="store_true")
    eth_address.set_defaults(handler=_cmd_eth_address, command_target="bitbox")
    eth_xpub = eth_sub.add_parser("xpub")
    eth_xpub.add_argument("--keypath", type=_parse_keypath)
    eth_xpub.add_argument("--display", action="store_true")
    eth_xpub.set_defaults(handler=_cmd_eth_xpub, command_target="bitbox")
    eth_sign_message = eth_sub.add_parser("sign-message")
    eth_sign_message.add_argument("--keypath", type=_parse_keypath)
    _add_message_input_args(eth_sign_message)
    eth_sign_message.set_defaults(handler=_cmd_eth_sign_message, command_target="bitbox")
    eth_typed_message = eth_sub.add_parser("sign-typed-message")
    eth_typed_message.add_argument("--keypath", type=_parse_keypath)
    typed_msg = eth_typed_message.add_mutually_exclusive_group(required=True)
    typed_msg.add_argument("--json-file")
    typed_msg.add_argument("--json-string")
    eth_typed_message.set_defaults(handler=_cmd_eth_sign_typed_message, command_target="bitbox")

    bootloader = subparsers.add_parser("bootloader", help="Bootloader commands")
    bootloader_sub = bootloader.add_subparsers(dest="bootloader_command")
    bootloader_sub.add_parser("versions").set_defaults(
        handler=_cmd_bootloader_versions, command_target="bootloader"
    )
    bootloader_sub.add_parser("hardware").set_defaults(
        handler=_cmd_bootloader_hardware, command_target="bootloader"
    )
    hashes = bootloader_sub.add_parser("hashes")
    hashes.add_argument("--display-firmware-hash", action="store_true")
    hashes.add_argument("--display-signing-keydata-hash", action="store_true")
    hashes.set_defaults(handler=_cmd_bootloader_hashes, command_target="bootloader")
    set_hash = bootloader_sub.add_parser("set-show-firmware-hash")
    set_hash.add_argument("--enabled", type=_parse_bool, required=True)
    set_hash.set_defaults(
        handler=_cmd_bootloader_set_show_firmware_hash, command_target="bootloader"
    )
    bootloader_sub.add_parser("erase").set_defaults(
        handler=_cmd_bootloader_erase, command_target="bootloader"
    )
    bootloader_sub.add_parser("reboot").set_defaults(
        handler=_cmd_bootloader_reboot, command_target="bootloader"
    )

    firmware = subparsers.add_parser("firmware", help="Firmware commands")
    firmware_sub = firmware.add_subparsers(dest="firmware_command")
    flash = firmware_sub.add_parser("flash")
    flash.add_argument("firmware", help="Firmware image to flash")
    flash.add_argument("--unsigned", action="store_true", help="Flash an unsigned debug firmware")
    flash.add_argument("--no-reboot", action="store_true", help="Do not reboot after flashing")
    flash.set_defaults(handler=_cmd_firmware_flash, command_target="firmware")

    return parser


def _run_u2f_menu(args: argparse.Namespace) -> int:
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


def _run_interactive(args: argparse.Namespace) -> int:
    if args.u2f:
        return _run_u2f_menu(args)
    if args.simulator:
        return connect_to_simulator_bitbox(args.debug, args.simulator_port)
    return connect_to_usb_bitbox(args.debug, not args.no_cache)


def main(argv: Optional[Sequence[str]] = None) -> int:
    """Main function"""
    parser = _build_parser()
    args = parser.parse_args(argv)

    if args.interactive:
        return _run_interactive(args)

    if not hasattr(args, "handler"):
        parser.print_help()
        return 0

    try:
        if args.command_target == "firmware":
            return _run_firmware_command(args)
        if args.command_target == "bootloader":
            return _run_bootloader_command(args)
        if args.u2f:
            raise RuntimeError("--u2f only supports --interactive")
        return _run_bitbox_command(args)
    except (UserAbortException, CliUserAbort):
        eprint("Aborted by user")
        return 130
    except (Bitbox02Exception, OSError, RuntimeError, ValueError) as exc:
        eprint(exc)
        return 1


if __name__ == "__main__":
    sys.exit(main())
