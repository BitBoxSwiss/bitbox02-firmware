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
import sys

import bitbox02u2f

from bitbox02u2f.u2f import ConditionsNotSatisfiedException, WrongDataException


class App:
    """App"""

    # pylint: disable=too-few-public-methods
    APPID = "TEST"

    def __init__(self, path: bytes, debug: bool):
        self._device = bitbox02u2f.BitBox02U2F(path)
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
        except ConditionsNotSatisfiedException:
            print("Not registered")

    def _bogus(self) -> None:
        ans = input("Vendor [chromium, firefox]: ")
        try:
            self._device.u2f_register_bogus(ans)
        except ValueError as err:
            print("Invalid vendor, try again: {}".format(err))
        except ConditionsNotSatisfiedException:
            print("User not present")

    def _authenticate(self) -> None:
        try:
            self._device.u2f_authenticate(self.APPID, self._dev_keyhandle, self._dev_pubkey)
            print("User present")
        except ConditionsNotSatisfiedException:
            print("User not present")
        except WrongDataException:
            print("Keyhandle not for this key")

    def _print_menu(self) -> None:
        """Print the menu"""
        print("What would you like to do?")
        print("- (1) Wink")
        print("- (2) Ping")
        print("- (3) Register")
        print("- (4) Register with bogus appid")
        if self._dev_keyhandle == b"0" * 64:
            print("- (5) Authenticate (not registered)")
        else:
            print("- (5) Authenticate (registered)")
        print("- (q) Quit")

    def _menu(self) -> None:
        """TODO: Document
        """
        self._print_menu()
        ans = input("")
        if ans == "q":
            self._stop = True
            return
        try:
            choice = int(ans)
        except ValueError:
            print("Invalid input")
            return
        if choice == 1:
            self._wink()
        elif choice == 2:
            self._ping()
        elif choice == 3:
            self._register()
        elif choice == 4:
            self._bogus()
        elif choice == 5:
            self._authenticate()

    def run(self) -> int:
        """Main function"""
        while not self._stop:
            self._menu()
        self._device.close()
        return 0


def main() -> int:
    """Main function"""
    parser = argparse.ArgumentParser(description="Tool for communicating with bitbox device")
    parser.add_argument("--debug", action="store_true", help="Print messages sent and received")
    args = parser.parse_args()

    bitboxes = bitbox02u2f.get_bitbox02u2f_devices()

    if not bitboxes:
        print("No bitbox detected")
        return 1

    if len(bitboxes) > 1:
        print("Multiple bitboxes detected. Only one supported")
        return 1

    app = App(bitboxes[0]["path"], args.debug)
    return app.run()


if __name__ == "__main__":
    sys.exit(main())
