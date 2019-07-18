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
""" Library to interact with a BitBox02 device with U2F. """

import os
import sys
import time
import collections
import json
import base64
import hashlib
from datetime import datetime, tzinfo, timedelta
import binascii
import random
from typing import Optional, Tuple, List
from typing_extensions import TypedDict
import ecdsa

from noise.connection import NoiseConnection, Keypair
import hid

from .usb import hid_send_frames, hid_read_frames
from .u2f import RegistrationRequest, AuthenticationRequest
from .u2fhid import InitResponse, CID_BROADCAST, INIT, PING, WINK, MSG


class DeviceInfo(TypedDict):
    serial_number: str
    path: bytes


def get_bitbox02u2f_devices() -> List[DeviceInfo]:
    """List bitbos02 U2F devices"""
    # TODO: product id is 0x2403, but 0x2402 is the id of some dev
    # device bootloaders. Can be removed in time, not needed for
    # production devices.
    return [
        info
        for info in hid.enumerate()
        if info["vendor_id"] == 0x03EB
        and info["product_id"] in (0x2402, 0x2403)
        and (info["usage_page"] == 0xFFFF or info["interface_number"] == 1)
    ]


class Bitbox02Exception(Exception):
    """Custom exception"""

    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message
        super().__init__()

    def __str__(self) -> str:
        return f"error code: {self.code}, message: {self.message}"


class BitBox02U2F:
    """Interact with a bitbox for u2f purposes"""

    def __init__(self, device_path: bytes):
        self._device = hid.device()
        self._device.open_path(device_path)
        # 0 and 0xffffffff are reserved
        self._cid = random.randrange(1, 0xFFFFFFFF)
        self.debug = False

    def close(self) -> None:
        self._device.close()

    def _query(self, cid: int, cmd: int, msg: bytes) -> bytes:
        """
        Sends msg bytes and retrieves response bytes.
        """
        if self.debug:
            print(f"msg: {msg}, cid: {cid}, cmd: {cmd}")
        hid_send_frames(self._device, msg, cid, cmd)
        response_bytes = hid_read_frames(self._device, cid, cmd)
        if self.debug:
            print(f"response {len(response_bytes)}: {binascii.hexlify(bytes(response_bytes))}")
        return bytes(response_bytes)

    @staticmethod
    def _maybe_error(response_bytes: bytes, cmd: int) -> None:
        if cmd == 0x80 + 0x3F:
            raise Bitbox02Exception(response_bytes[0], "U2F error discovered")

    @staticmethod
    def _parse_u2f_init_response(response_bytes: bytes) -> InitResponse:
        return InitResponse(
            response_bytes[0:8],
            response_bytes[8:12],
            response_bytes[12:13],
            response_bytes[13:14],
            response_bytes[14:15],
            response_bytes[15:16],
            response_bytes[16:17],
        )

    def u2fhid_init(self, allocate: bool) -> None:
        """Set allocate to ask the device to choose CID"""
        nonce = bytes([1, 2, 3, 4, 5, 6, 7, 8])
        cid = self._cid
        if allocate:
            cid = CID_BROADCAST
        response_bytes = self._query(cid, INIT, nonce)
        init_response = self._parse_u2f_init_response(response_bytes)
        self._cid = init_response.cid

    def u2fhid_ping(self, msg: bytes) -> bytes:
        return self._query(self._cid, PING, msg)

    def u2fhid_wink(self) -> bool:
        _response_bytes = self._query(self._cid, WINK, bytes("", "utf-8"))
        return True

    def u2fhid_msg(self, msg: bytes) -> bytes:
        return self._query(self._cid, MSG, msg)

    def u2f_register(self, appid: str) -> Tuple[bytes, bytes]:
        self.u2fhid_init(True)
        req = RegistrationRequest(appid)
        response = req.send(self)
        if not response.verify():
            raise Exception("Did not verify")
        return (response.ec_public_key, response.key_handle)

    def u2f_authenticate(self, appid: str, key_handle: bytes, pub_key: bytes) -> bool:
        self.u2fhid_init(True)
        req = AuthenticationRequest(appid, key_handle)
        response = req.send(self)
        return response.verify(pub_key)

    def u2f_register_bogus(self, vendor: str) -> None:
        self.u2fhid_init(True)
        req = RegistrationRequest("", vendor)
        response = req.send(self)
        if not response.verify():
            raise Exception("Did not verify")
