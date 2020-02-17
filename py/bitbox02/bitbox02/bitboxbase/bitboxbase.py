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

from typing import Optional

import serial

from bitbox02 import communication
from bitbox02.communication.devices import DeviceInfo

from bitbox02.communication.generated import bitboxbase_pb2 as bitboxbase
from bitbox02.communication.generated import hww_pb2 as hww


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
        noise_config: communication.BitBoxNoiseConfig,
    ):
        communication.BitBoxCommonAPI.__init__(self, device, device_info, noise_config)

    def _bbb_query(
        self, request: bitboxbase.BitBoxBaseRequest, _expected_response: Optional[str] = None
    ) -> hww.Response:
        # pylint: disable=no-member
        req = hww.Request()
        req.bitboxbase.CopyFrom(request)
        return self._msg_query(req)

    def set_config(
        self,
        status_led_mode: bitboxbase.BitBoxBaseSetConfigRequest.StatusLedMode,
        ip_addr: bytes,
        hostname: str,
    ) -> None:
        """Set config API call"""
        # pylint: disable=no-member
        req = bitboxbase.BitBoxBaseRequest()
        req.set_config.CopyFrom(
            bitboxbase.BitBoxBaseSetConfigRequest(
                status_led_mode=status_led_mode,
                status_screen_mode=None,
                ip=ip_addr,
                hostname=hostname,
            )
        )
        self._bbb_query(req)

    def heartbeat(
        self,
        number: bitboxbase.BitBoxBaseHeartbeatRequest.StateCode,
        description: bitboxbase.BitBoxBaseHeartbeatRequest.DescriptionCode,
    ) -> None:
        """Heartbeat API call"""
        # pylint: disable=no-member
        req = bitboxbase.BitBoxBaseRequest()
        req.heartbeat.CopyFrom(
            bitboxbase.BitBoxBaseHeartbeatRequest(state_code=number, description_code=description)
        )
        self._bbb_query(req)

    def confirm_pairing(self, msg: bytes) -> None:
        # pylint: disable=no-member
        req = bitboxbase.BitBoxBaseRequest()
        req.confirm_pairing.CopyFrom(bitboxbase.BitBoxBaseConfirmPairingRequest(msg=msg))
        self._bbb_query(req)
