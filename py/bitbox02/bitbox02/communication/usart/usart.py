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

""" U2F-over-USART module. """

from types import TracebackType
from typing import Optional, Tuple, Type
import struct

import serial

from bitbox02.communication import PhysicalLayer, TransportLayer


class SerialPort(PhysicalLayer):
    """
    Wrapper to make serial.Serial compatible with hid.device
    (i.e. to make it implement communication.PhysicalLayer).
    """

    def __init__(self, port: str):
        self.port = serial.Serial(port, baudrate=115200)

    def __enter__(self) -> "SerialPort":
        self.port.__enter__()
        return self

    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_value: Optional[BaseException],
        traceback: Optional[TracebackType],
    ) -> bool:
        result = self.port.__exit__(exc_type, exc_value, traceback)  # type: bool
        return result

    def read(self, size: int, timeout_ms: int) -> bytes:
        self.port.timeout = timeout_ms / 1000.0
        read_data = self.port.read(size)  # type: bytes
        return read_data

    def write(self, data: bytes) -> None:
        for byte in data:
            while True:
                res = self.port.write(bytes([byte]))
                if res == 1:
                    break


class U2FUsartError(Exception):
    pass


class U2FUsartTimeoutError(U2FUsartError):
    def __init__(self) -> None:
        super().__init__("Connection timed out.")


class U2FUsartErrorResponse(U2FUsartError):
    """
    Error that has been sent by the USART stack.
    """

    ENDPOINT_UNAVAILABLE = 0x01
    ENDPOINT_BUSY = 0x02
    INVALID_CMD = 0x03

    def __init__(self, error_code: int) -> None:
        super().__init__("Error response from the UART port, error code {}.".format(error_code))
        self.error_code = error_code


class U2FUsart(TransportLayer):
    """
    Implements the U2F-over-USART framing protocol.
    """

    ENDPOINT_ERROR = 0xFF

    def generate_cid(self) -> int:
        """Generate a valid CID"""
        return 0x42

    def _encode_usart_frame(self, msg: bytes) -> bytes:
        return b"\x7E" + msg.replace(b"\x7D", b"\x7D\x5D").replace(b"\x7E", b"\x7D\x5E") + b"\x7E"

    def _read_usart_frame(self) -> Tuple[bytes, bytes]:
        """
        Reads a complete frame of data.
        Returns decoded frame, raw frame bytes.
        """
        all_read = bytes()
        timeout_ms = 5000
        while True:
            read_byte = self._device.read(1, timeout_ms)
            if len(read_byte) == 0:
                return bytes(), all_read
            # print("Read {:02X}".format(r[0]))
            all_read += read_byte
            if read_byte == b"\x7E":
                break
        stuff = bytes()
        while True:
            read_byte = self._device.read(1, timeout_ms)
            if len(read_byte) == 0:
                return stuff, all_read
            # print("Read {:02X}".format(r[0]))
            all_read += read_byte
            if read_byte == b"\x7D":
                read_byte = self._device.read(1, timeout_ms)
                stuff += bytes([read_byte[0] ^ 0x20])
            elif read_byte == b"\x7E":
                return stuff, all_read
            else:
                stuff += read_byte

    def write(self, data: bytes, endpoint: int, cid: int) -> None:
        """
        Send data to the device.

        Args:
            data: Data to send
            endpoint: U2F HID command/Destination endpoint
            cid: U2F HID channel ID (will be ignored)
        Throws:
            ValueError: In case any value is out of range
        """
        if endpoint < 0 or endpoint > 0xFF:
            raise ValueError("Channel command is out of range '0 < endpoint <= 0xFF'")
        if endpoint < 0 or endpoint > 0xFFFFFFFF:
            raise ValueError("Channel id is out of range '0 < endpoint <= 0xFFFFFFFF'")
        to_write = bytes([1, endpoint]) + bytearray(data)
        checksum = self.compute_checksum(to_write)
        checksum_bytes = struct.pack("<H", checksum)
        to_write = to_write + checksum_bytes
        # print("Checksum: {} ({})".format(checksum, checksum_bytes))
        data_len = len(to_write)
        if data_len > 5000:
            raise ValueError("Data is too large 'size <= 5000'")

        to_write = self._encode_usart_frame(to_write)
        # print("Writing length {}: {}".format(len(data), data.hex()))
        # print("Writing encoded length {}: {}".format(len(to_write), to_write.hex()))

        # print("Host -> Base (unpacked): endpoint {:02X}, data (len {}): "
        # .format(endpoint, len(data)) + data.hex())
        # print("Host -> Base (raw): " + to_write.hex())
        self._device.write(to_write)

    def compute_checksum(self, data: bytes) -> int:
        """
        Computes the 1-complement sum of a chunk of bytes as 2B words.
        """
        if len(data) % 2 != 0:
            data = data + b"\x00"
        n_sums = len(data) // 2
        checksum = 0
        for i in range(n_sums):
            checksum += struct.unpack("<H", data[2 * i : 2 * i + 2])[0]
            if checksum > 0xFFFF:
                checksum -= 0xFFFF
                assert 0 <= checksum <= 0xFFFF
        return checksum

    def read(self, endpoint: int, cid: int) -> bytes:
        """
        Receive data from the device.

        Args:
            endpoint: The expected returned U2F HID command (endpoint)
            cid: The expected returned U2F CID (will be ignored).
        Returns:
            The contents of the read message.
        Throws:
            ValueError: In case any value is out of range
            Exception: In case of USART communication issues
        """
        if endpoint < 0 or endpoint > 0xFF:
            raise ValueError("Source endpoint id is out of range '0 < endpoint <= 0xFF'")
        buf, _ = self._read_usart_frame()
        # print("Base -> Host: " + raw_dump.hex(), file=dump_file)
        # print("Response ({} bytes): {}".format(len(buf), buf.hex()))
        if len(buf) < 4:
            raise U2FUsartTimeoutError()
        version, src_endpoint = buf[0], buf[1]
        if version != 1:
            raise Exception(f"- Unknown U2F-over-USART version: {version}")
        reply_checksum = struct.unpack("<H", buf[-2:])[0]
        data = buf[:-2]
        expected_checksum = self.compute_checksum(data)
        if expected_checksum != reply_checksum:
            raise Exception(
                f"- USART checksum incorrect! {reply_checksum:x} != {expected_checksum:x}"
            )
        if src_endpoint == self.ENDPOINT_ERROR:
            raise U2FUsartErrorResponse(data[0])
        if src_endpoint != endpoint:
            raise Exception(f"- USART source endpoint mismatch {endpoint:x} != {src_endpoint:x}")
        data = data[2:]
        # print("Base -> Host (unpacked): cmd {:02X}, endpoint {:02X}, data (len {}): "
        # .format(reply_cmd, src_endpoint, len(data)) + data.hex(), file=dump_file)
        return data

    def __init__(self, device: PhysicalLayer):
        self._device = device
