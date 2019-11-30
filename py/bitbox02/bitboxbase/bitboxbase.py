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
"""BitBoxBase"""

from typing import Optional, Callable


import serial

import communication
from communication.devices import DeviceInfo


def get_bitboxbase_default_device(serial_port: serial.Serial) -> DeviceInfo:
    return {
        "serial_number": "v4.2.0",
        "path": serial_port.port,
        "product_string": "bitboxbase-bootloader",
    }


class BitBoxBase(communication.BitBoxCommonAPI):
    """Class to communicate with a BitBoxBase"""

    # pylint: disable=too-many-public-methods

    def __init__(
        self,
        device: communication.TransportLayer,
        device_info: DeviceInfo,
        show_pairing_callback: Callable[[str], bool],
        attestation_check_callback: Optional[Callable[[bool], None]] = None,
    ):
        communication.BitBoxCommonAPI.__init__(
            self, device, device_info, show_pairing_callback, attestation_check_callback
        )
