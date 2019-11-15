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

"""Implementations"""

import struct
import random
from typing_extensions import Protocol

USB_REPORT_SIZE = 64

ERR_NONE = 0x00
ERR_INVALID_CMD = 0x01
ERR_INVALID_PAR = 0x02
ERR_INVALID_LEN = 0x03
ERR_INVALID_SEQ = 0x04
ERR_MSG_TIMEOUT = 0x05
ERR_CHANNEL_BUSY = 0x06
ERR_LOCK_REQUIRED = 0x0A
ERR_INVALID_CID = 0x0B
ERR_OTHER = 0x7F

PING = 0x80 | 0x01
MSG = 0x80 | 0x03
LOCK = 0x80 | 0x04
INIT = 0x80 | 0x06
WINK = 0x80 | 0x08
SYNC = 0x80 | 0x3C
ERROR = 0x80 | 0x3F

CID_BROADCAST = 0xFFFFFFFF


class SupportsReadWrite(Protocol):
    # pylint: disable=unused-argument,no-self-use

    def write(self, msg: bytes) -> None:
        ...

    def read(self, size: int, timeout_ms: int) -> bytes:
        ...


def generate_cid() -> int:
    """Generate a valid CID"""
    # Exclude 0 and u32_max (0xffff_ffff)
    return random.randrange(1, 0xFFFFFFFF)


# TODO: Create exceptions
def _throw_error(error_code: int) -> None:
    if error_code == ERR_INVALID_CMD:
        raise Exception("Received error: invalid command")
    if error_code == ERR_INVALID_LEN:
        raise Exception("Received error: invalid length")
    if error_code == ERR_INVALID_SEQ:
        raise Exception("Received error: invalid sequence")
    if error_code == ERR_CHANNEL_BUSY:
        raise Exception("Received error: channel busy")
    if error_code == 0x7E:
        raise Exception("Received error: encryption failed")
    if error_code == ERR_OTHER:
        raise Exception("Received unknown error")
    raise Exception("Received error: %d" % error_code)


def write(hid_device: SupportsReadWrite, data: bytes, cmd: int, cid: int) -> None:
    """
    Send data to the device.

    Args:
        hid_device: An object that implements read/write functions
        data: Data to send
        cmd: U2F HID command
        cid: U2F HID channel ID
    Throws:
        ValueError: In case any value is out of range
    """
    if cmd < 0 or cmd > 0xFF:
        raise ValueError("Channel command is out of range '0 < cmd <= 0xFF'")
    if cid < 0 or cid > 0xFFFFFFFF:
        raise ValueError("Channel id is out of range '0 < cid <= 0xFFFFFFFF'")
    data = bytearray(data)
    data_len = len(data)
    if data_len > 0xFFFF:
        raise ValueError("Data is too large 'size <= 0xFFFF'")
    seq = 0
    idx = 0
    buf = b""
    # Allow to write an empty packet
    single_empty_write = data_len == 0
    while idx < data_len or single_empty_write:
        if idx == 0:
            # INIT frame
            buf = data[idx : idx + min(data_len, USB_REPORT_SIZE - 7)]
            hid_device.write(
                b"\0"
                + struct.pack(">IBH", cid, cmd, data_len)
                + buf
                + b"\xEE" * (USB_REPORT_SIZE - 7 - len(buf))
            )
        else:
            # CONT frame
            buf = data[idx : idx + min(data_len, USB_REPORT_SIZE - 5)]
            hid_device.write(
                b"\0"
                + struct.pack(">IB", cid, seq)
                + buf
                + b"\xEE" * (USB_REPORT_SIZE - 5 - len(buf))
            )
            seq += 1
        idx += len(buf)
        single_empty_write = False


def read(hid_device: SupportsReadWrite, cmd: int, cid: int, timeout: int = 5000) -> bytes:
    """
    Receive data from the device.

    Args:
        hid_device: An object that implements read/write functions
        cmd: The expected returned U2F HID command
        cid: The expected returned U2F HID channel ID
        timeout: For how long to wait for more bytes from hid_device
    Returns:
        The read message combined from the u2fhid packets
    Throws:
        ValueError: In case any value is out of range
        Exception: In case of USB communication issues
    """
    if cmd < 0 or cmd > 0xFF:
        raise ValueError("Channel command is out of range '0 < cmd <= 0xFF'")
    if cid < 0 or cid > 0xFFFFFFFF:
        raise ValueError("Channel id is out of range '0 < cid <= 0xFFFFFFFF'")
    timeout_ms = timeout * 1000
    buf = hid_device.read(USB_REPORT_SIZE, timeout_ms)
    if len(buf) >= 3:
        reply_cid = ((buf[0] * 256 + buf[1]) * 256 + buf[2]) * 256 + buf[3]
        reply_cmd = buf[4]
        data_len = buf[5] * 256 + buf[6]
        data = buf[7:]
        idx = len(buf) - 7
        if reply_cmd == ERROR:
            _throw_error(data[0])
        while idx < data_len:
            # CONT response
            buf = hid_device.read(USB_REPORT_SIZE, timeout_ms)
            if len(buf) < 3:
                raise Exception("Did not receive a continuation frame after %d seconds" % timeout)
            data += buf[5:]
            idx += len(buf) - 5
        if reply_cid != cid:
            raise Exception(f"- USB channel ID mismatch {reply_cid:x} != {cid:x}")
        if reply_cmd != cmd:
            raise Exception(f"- USB command mismatch {reply_cmd:x} != {cmd:x}")
        return data[:data_len]
    raise Exception("Did not read anything after %d seconds" % timeout)
