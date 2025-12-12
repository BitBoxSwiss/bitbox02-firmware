# SPDX-License-Identifier: Apache-2.0

"""
Common interfaces to be used for communication with BitBox devices.
"""

from typing_extensions import Protocol


class TransportLayer(Protocol):
    """
    Abstraction for the transport layer used for transmitting U2F messages.
    This class encapsulates packets on a given physical link capable of
    transmitting byte strings.
    """

    # pylint: disable=unused-argument
    def write(self, data: bytes, endpoint: int, cid: int) -> None:
        """Sends a frame of data to the specified endpoint"""

    def read(self, endpoint: int, cid: int) -> bytes: ...

    def query(self, data: bytes, endpoint: int, cid: int) -> bytes:
        self.write(data, endpoint, cid)
        return self.read(endpoint, cid)

    def generate_cid(self) -> int: ...

    def close(self) -> None: ...


class PhysicalLayer(Protocol):
    # pylint: disable=unused-argument
    def write(self, data: bytes) -> None: ...

    def read(self, size: int, timeout_ms: int) -> bytes: ...

    def close(self) -> None: ...
