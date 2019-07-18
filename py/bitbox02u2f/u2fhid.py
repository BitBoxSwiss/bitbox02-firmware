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
"""U2FHID packet types"""

CID_BROADCAST = 0xFFFFFFFF

ERR_NONE = 0x00
ERR_INVALID_CMD = 0x01
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


class InitResponse:
    """Reponse to Init"""

    # pylint: disable=too-few-public-methods

    def __init__(
        self,
        nonce: bytes,
        cid: bytes,
        version_interface: bytes,
        version_major: bytes,
        version_minor: bytes,
        version_build: bytes,
        cap_flags: bytes,
    ):
        # pylint: disable=too-many-arguments
        self.nonce = nonce
        self.cid = int.from_bytes(cid, "big")
        self.version_interface = version_interface
        self.version_major = version_major
        self.version_minor = version_minor
        self.version_build = version_build
        self.cap_flags = cap_flags
