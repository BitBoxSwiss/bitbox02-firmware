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
HWW_CID = 0xFF000000

FRAME_ERROR = 0x80 | 0x3F


class SupportsReadWrite(typing_extensions.Protocol):
    # pylint: disable=unused-argument,no-self-use

    def write(self, msg: bytes) -> None:
        ...

    def read(self, size: int, timeout_ms: int) -> bytes:
        ...


def hid_send_frames(hid_device: SupportsReadWrite, data: bytes, cmd: int) -> None:
    """
    Send data to the device.
    """
    data = bytearray(data)
    data_len = len(data)
    seq = 0
    idx = 0
    write = b""
    while idx < data_len:
        if idx == 0:
            # INIT frame
            write = data[idx : idx + min(data_len, USB_REPORT_SIZE - 7)]
            hid_device.write(
                b"\0"
                + struct.pack(">IBH", HWW_CID, cmd, data_len & 0xFFFF)
                + write
                + b"\xEE" * (USB_REPORT_SIZE - 7 - len(write))
            )
        else:
            # CONT frame
            write = data[idx : idx + min(data_len, USB_REPORT_SIZE - 5)]
            hid_device.write(
                b"\0"
                + struct.pack(">IB", HWW_CID, seq)
                + write
                + b"\xEE" * (USB_REPORT_SIZE - 5 - len(write))
            )
            seq += 1
        idx += len(write)


def hid_read_frames(hid_device: SupportsReadWrite, cmd: int, timeout: int = 5000) -> bytes:
    """
    Receive data from the device.
    """
    timeout_ms = timeout * 1000
    read = hid_device.read(USB_REPORT_SIZE, timeout_ms)
    if len(read) >= 3:
        cid = ((read[0] * 256 + read[1]) * 256 + read[2]) * 256 + read[3]
        reply_cmd = read[4]
        data_len = read[5] * 256 + read[6]
        data = read[7:]
        idx = len(read) - 7
        while idx < data_len:
            # CONT response
            read = hid_device.read(USB_REPORT_SIZE, timeout_ms)
            if len(read) < 3:
                raise Exception("Did not receive a continuation frame after %d seconds" % timeout)
            data += read[5:]
            idx += len(read) - 5
        assert cid == HWW_CID, "- USB command ID mismatch"
        if reply_cmd == FRAME_ERROR:
            raise Exception(
                "usb error: %s"
                % {
                    0x01: "invalid cmd",
                    0x03: "invalid len",
                    0x04: "channel seq",
                    0x06: "channel busy",
                    0x7F: "other",
                }.get(data[0], "unknown")
            )
        assert reply_cmd == cmd, "- USB command frame mismatch %s %s" % (reply_cmd, cmd)
        return data[:data_len]
    raise Exception("Did not read anything after %d seconds" % timeout)
