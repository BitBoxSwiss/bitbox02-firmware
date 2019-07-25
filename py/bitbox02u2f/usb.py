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
""" Util functions to interact with usb/hid devices. """

import struct
import typing_extensions

USB_REPORT_SIZE = 64


class SupportsReadWrite(typing_extensions.Protocol):
    # pylint: disable=unused-argument,no-self-use

    def write(self, msg: bytes) -> None:
        ...

    def read(self, size: int, timeout_ms: int) -> bytes:
        ...


def _throw_error(error_code: int) -> None:
    if error_code == 0x01:
        raise Exception("Received error: invalid command")
    if error_code == 0x03:
        raise Exception("Received error: invalid length")
    if error_code == 0x04:
        raise Exception("Received error: invalid sequence")
    if error_code == 0x06:
        raise Exception("Received error: channel busy")
    if error_code == 0x7E:
        raise Exception("Received error: encryption failed")
    if error_code == 0x7F:
        raise Exception("Received unknown error")
    raise Exception("Received error: %d" % error_code)


def hid_send_frames(hid_device: SupportsReadWrite, data: bytes, cid: int, cmd: int) -> None:
    """
    Send data to the device.
    """
    data = bytearray(data)
    data_len = len(data)
    seq = 0
    idx = 0
    write = b""
    write_empty = data_len == 0
    while idx < data_len or write_empty:
        if idx == 0:
            # INIT frame
            write = data[idx : idx + min(data_len, USB_REPORT_SIZE - 7)]
            hid_device.write(
                b"\0"
                + struct.pack(">IBH", cid, cmd, data_len & 0xFFFF)
                + write
                + b"\xEE" * (USB_REPORT_SIZE - 7 - len(write))
            )
        else:
            # CONT frame
            write = data[idx : idx + min(data_len, USB_REPORT_SIZE - 5)]
            hid_device.write(
                b"\0"
                + struct.pack(">IB", cid, seq)
                + write
                + b"\xEE" * (USB_REPORT_SIZE - 5 - len(write))
            )
            seq += 1
        idx += len(write)
        write_empty = False


def hid_read_frames(
    hid_device: SupportsReadWrite, cid: int, cmd: int, timeout: int = 5000
) -> bytes:
    """
    Receive data from the device.
    """
    timeout_ms = timeout * 1000
    read = hid_device.read(USB_REPORT_SIZE, timeout_ms)
    if len(read) >= 3:
        reply_cid = ((read[0] * 256 + read[1]) * 256 + read[2]) * 256 + read[3]
        reply_cmd = read[4]
        data_len = read[5] * 256 + read[6]
        data = read[7:]
        idx = len(read) - 7
        if reply_cmd == 0x80 + 0x3F:
            _throw_error(data[0])

        while idx < data_len:
            # CONT response
            read = hid_device.read(USB_REPORT_SIZE, timeout_ms)
            if len(read) < 3:
                raise Exception("Did not receive a continuation frame after %d seconds" % timeout)
            data += read[5:]
            idx += len(read) - 5
        assert reply_cid == cid, "- USB channel ID mismatch"
        assert reply_cmd == cmd, "- USB command id mismatch"
        return data[:data_len]
    raise Exception("Did not read anything after %d seconds" % timeout)
