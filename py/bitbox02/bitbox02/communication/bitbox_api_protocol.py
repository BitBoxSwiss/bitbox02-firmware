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
"""BitBox02"""

from abc import ABC, abstractmethod
import os
import enum
import sys
import base64
import binascii
import hashlib
from typing import Optional, List, Dict, Tuple, Union
from typing_extensions import TypedDict

import ecdsa
from noise.connection import NoiseConnection, Keypair
import semver

from .devices import parse_device_version, DeviceInfo

from .communication import TransportLayer

try:
    from .generated import hww_pb2 as hww
    from .generated import system_pb2 as system
except ModuleNotFoundError:
    print("Run `make py` to generate the protobuf messages")
    sys.exit()


HWW_CMD = 0x80 + 0x40 + 0x01

ERR_GENERIC = 103
ERR_USER_ABORT = 104

HARDENED = 0x80000000


class AttestationPubkeyInfo(TypedDict):
    # uncompressed secp256k1 pubkey serialization
    pubkey: bytes
    # if not None, a hex-encoded bootloader hashes (of the padded
    # bootloader binary, i.e. the device bootloader area), for which
    # this attestation pubkey is
    accepted_bootloader_hash: Optional[bytes]


ATTESTATION_PUBKEYS: List[AttestationPubkeyInfo] = [
    {
        "pubkey": binascii.unhexlify(
            "04074ff1273b36c24e80fe3d59e0e897a81732d3f8e9cd07e17e9fc06319cd16b"
            "25cf74255674477b3ac9cbac2d12f0dc27a662681fcbc12955b0bccdcbbdcfd01"
        ),
        "accepted_bootloader_hash": None,
    },
    {
        "pubkey": binascii.unhexlify(
            "044c53a84f41fa7301b378bb3c260fc9b2ff1cbea7a78181279a8566797a736f1"
            "2cea25fa2b1c27a844392fe9b37547dc6fbd00a2676b816e7d2d3562be2a0cbbd"
        ),
        "accepted_bootloader_hash": None,
    },
    {
        "pubkey": binascii.unhexlify(
            "04e9c8dc929796aac65af5084eb54dc1ee482d5e0b5c58e2c93f243c5b70b2152"
            "3324bdb78d7395317da165ef1138826c3ca3c91ca95e6f490c340cf5508a4a3ec"
        ),
        "accepted_bootloader_hash": None,
    },
    {
        "pubkey": binascii.unhexlify(
            "04c2fb05889b9dff5a9fb22a59ee1d16bfc2863f0400ddcb69566e2abe8a15fa0"
            "ba1240254ca45aa310d170e724e1310ce5f611cada76c12e3c24a926a390ca4be"
        ),
        "accepted_bootloader_hash": None,
    },
    {
        "pubkey": binascii.unhexlify(
            "04c4e82d6d1b91e7853eba96a871ad31fc62620b826b0b8acf815c03de31b792a"
            "98e05bb34d3b9e0df1040eac485f03ff8bbbf7a857ef1cf2a49a60ac084efb88f"
        ),
        "accepted_bootloader_hash": None,
    },
    {
        "pubkey": binascii.unhexlify(
            "040526f5b8348a8d55e7b1cac043ce98c55bbdb3311b4d1bb2d654281edf8aeb2"
            "1f018fb027a6b08e4ddc62c919e648690722d00c6f54c668c9bd8224a1d82423a"
        ),
        "accepted_bootloader_hash": binascii.unhexlify(
            "e8fa0bd5fc80b86b9f1ea983664df33b27f6f95855d79fb43248ee4c3d3e6be6"
        ),
    },
]

ATTESTATION_PUBKEYS_MAP: Dict[bytes, AttestationPubkeyInfo] = {
    hashlib.sha256(val["pubkey"]).digest(): val for val in ATTESTATION_PUBKEYS
}

OP_ATTESTATION = b"a"
OP_UNLOCK = b"u"
OP_INFO = b"i"
OP_I_CAN_HAS_HANDSHAEK = b"h"
OP_I_CAN_HAS_PAIRIN_VERIFICASHUN = b"v"
OP_NOISE_MSG = b"n"

RESPONSE_SUCCESS = b"\x00"
RESPONSE_FAILURE = b"\x01"


class Platform(enum.Enum):
    """ Available hardware platforms """

    BITBOX02 = "bitbox02"
    BITBOXBASE = "bitboxbase"


class BitBox02Edition(enum.Enum):
    """ Editions for the BitBox02 platform """

    MULTI = "multi"
    BTCONLY = "btconly"


class BitBoxBaseEdition(enum.Enum):
    """ Editions for the BitBoxBase platform """

    STANDARD = "standard"


class Bitbox02Exception(Exception):
    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message
        super().__init__()

    def __str__(self) -> str:
        return f"error code: {self.code}, message: {self.message}"


class UserAbortException(Bitbox02Exception):
    pass


class AttestationException(Exception):
    pass


class BitBoxNoiseConfig:
    """ Stores Functions required setup a noise connection """

    # pylint: disable=no-self-use,unused-argument
    def show_pairing(self, code: str) -> bool:
        return True

    def attestation_check(self, result: bool) -> None:
        return

    def contains_device_static_pubkey(self, pubkey: bytes) -> bool:
        return False

    def add_device_static_pubkey(self, pubkey: bytes) -> None:
        pass

    def get_app_static_privkey(self) -> Optional[bytes]:
        return None

    def set_app_static_privkey(self, privkey: bytes) -> None:
        pass


class NoiseProtocolHandler(ABC):
    """ Class for executing noise operations (handshake & message transmission). """

    def __init__(self, transport: TransportLayer):
        super().__init__()
        self._transport = transport
        self._noise = None

    def close(self) -> None:
        self._transport.close()

    def _raw_query(self, msg: bytes) -> bytes:
        cid = self._transport.generate_cid()
        return self._transport.query(msg, HWW_CMD, cid)

    def query(self, cmd: bytes, msg_data: bytes) -> Tuple[bytes, bytes]:
        """
        Sends encapsulated msg bytes and retrieves encapsulated response status and data.
        """
        response = self._raw_query(cmd + msg_data)
        return response[0:1], response[1:]

    @abstractmethod
    def _encode_noise_request(self, encrypted_msg: bytes) -> bytes:
        """ Encapsulates an OP_NOISE_MSG message. """
        ...

    def encrypted_query(self, msg: bytes) -> bytes:
        """
        Sends msg bytes and reads response bytes over an encrypted channel.
        """
        encrypted_msg = self._noise.encrypt(msg)
        encrypted_msg = self._encode_noise_request(encrypted_msg)

        response = self._raw_query(encrypted_msg)
        result = self._noise.decrypt(response)
        assert isinstance(result, bytes)
        return result

    # pylint: disable=too-many-branches
    def _create_noise_channel(self, noise_config: BitBoxNoiseConfig) -> NoiseConnection:
        if self._raw_query(OP_I_CAN_HAS_HANDSHAEK) != RESPONSE_SUCCESS:
            self.close()
            raise Exception("Couldn't kick off handshake")

        # init noise channel
        noise = NoiseConnection.from_name(b"Noise_XX_25519_ChaChaPoly_SHA256")
        noise.set_as_initiator()
        private_key = noise_config.get_app_static_privkey()
        if private_key is None:
            private_key = os.urandom(32)
            noise_config.set_app_static_privkey(private_key)
        noise.set_keypair_from_private_bytes(Keypair.STATIC, private_key)
        noise.set_prologue(b"Noise_XX_25519_ChaChaPoly_SHA256")
        noise.start_handshake()
        noise.read_message(self._raw_query(noise.write_message()))
        remote_static_key = noise.noise_protocol.handshake_state.rs.public_bytes
        assert not noise.handshake_finished
        send_msg = noise.write_message()
        assert noise.handshake_finished
        pairing_code = base64.b32encode(noise.get_handshake_hash()).decode("ascii")
        response = self._raw_query(send_msg)

        # Check if we recognize the device's public key
        pairing_verification_required_by_host = True
        if noise_config.contains_device_static_pubkey(remote_static_key):
            pairing_verification_required_by_host = False

        pairing_verification_required_by_device = response == b"\x01"
        if pairing_verification_required_by_host or pairing_verification_required_by_device:
            cid = self._transport.generate_cid()
            self._transport.write(OP_I_CAN_HAS_PAIRIN_VERIFICASHUN, HWW_CMD, cid)
            client_response_success = noise_config.show_pairing(
                "{} {}\n{} {}".format(
                    pairing_code[:5], pairing_code[5:10], pairing_code[10:15], pairing_code[15:20]
                )
            )
            if not client_response_success:
                self.close()
                raise Exception("pairing rejected by the user on client")
            pairing_response = self._transport.read(HWW_CMD, cid)

            if pairing_response == RESPONSE_SUCCESS:
                pass
            elif pairing_response == RESPONSE_FAILURE:
                self.close()
                raise Exception("pairing rejected by the user")
            else:
                self.close()
                raise Exception("unexpected response")
            noise_config.add_device_static_pubkey(remote_static_key)
        return noise

    def noise_connect(self, noise_config: BitBoxNoiseConfig) -> None:
        self._noise = self._create_noise_channel(noise_config)

    @abstractmethod
    def unlock_query(self) -> bytes:
        """
        Executes an unlock query.
        Returns the bytes containing the response status.
        """
        ...


class NoiseProtocolHandlerV1(NoiseProtocolHandler):
    """ Noise Protocol from firmware V1.0.0 onwards. """

    def unlock_query(self) -> bytes:
        unlock_reply = self._raw_query(OP_UNLOCK)
        if len(unlock_reply) != 0:
            raise ValueError(f"OP_UNLOCK (V1) replied with wrong length.")
        return RESPONSE_SUCCESS

    def _encode_noise_request(self, encrypted_msg: bytes) -> bytes:
        return encrypted_msg


class NoiseProtocolHandlerV3(NoiseProtocolHandlerV1):
    """ Noise Protocol from firmware V3.0.0 onwards. """

    def unlock_query(self) -> bytes:
        unlock_result, unlock_data = self.query(OP_UNLOCK, b"")
        if len(unlock_data) != 0:
            raise ValueError(f"OP_UNLOCK (V3) replied with wrong length.")
        return unlock_result


class NoiseProtocolHandlerV4(NoiseProtocolHandlerV3):
    """ Noise Protocol from firmware V4.0.0 onwards. """

    def _encode_noise_request(self, encrypted_msg: bytes) -> bytes:
        return OP_NOISE_MSG + encrypted_msg


class BitBoxCommonAPI:
    """Class to communicate with a BitBox device"""

    # pylint: disable=too-many-public-methods,too-many-arguments
    def __init__(
        self, transport: TransportLayer, device_info: DeviceInfo, noise_config: BitBoxNoiseConfig
    ):
        self.debug = False
        serial_number = device_info["serial_number"]

        self.version = parse_device_version(serial_number)
        if self.version is None:
            transport.close()
            raise ValueError(f"Could not parse version from {serial_number}")
        # Delete the prelease part, as it messes with the comparison (e.g. 3.0.0-pre < 3.0.0 is
        # True, but the 3.0.0-pre has already the same API breaking changes like 3.0.0...).
        self.version = semver.VersionInfo(
            self.version.major, self.version.minor, self.version.patch, build=self.version.build
        )

        self._noise_handler: NoiseProtocolHandler
        if self.version >= semver.VersionInfo(4, 0, 0):
            self._noise_handler = NoiseProtocolHandlerV4(transport)
        elif self.version >= semver.VersionInfo(3, 0, 0):
            self._noise_handler = NoiseProtocolHandlerV3(transport)
        else:
            self._noise_handler = NoiseProtocolHandlerV1(transport)

        if self.version >= semver.VersionInfo(2, 0, 0):
            noise_config.attestation_check(self._perform_attestation())
            unlock_result = self._noise_handler.unlock_query()
            if unlock_result != RESPONSE_SUCCESS:
                self.close()
                raise Exception("Unlock process aborted")

        self._noise_handler.noise_connect(noise_config)

    # pylint: disable=too-many-return-statements
    def _perform_attestation(self) -> bool:
        """Sends a random challenge and verifies that the response can be verified with
        Shift's root attestation pubkeys. Returns True if the verification is successful."""

        challenge = os.urandom(32)
        response_status, response = self._noise_handler.query(OP_ATTESTATION, challenge)
        if response_status != RESPONSE_SUCCESS:
            return False

        # parse data
        bootloader_hash, response = response[:32], response[32:]
        device_pubkey_bytes, response = response[:64], response[64:]
        certificate, response = response[:64], response[64:]
        root_pubkey_identifier, response = response[:32], response[32:]
        challenge_signature, response = response[:64], response[64:]

        # check attestation
        if root_pubkey_identifier not in ATTESTATION_PUBKEYS_MAP:
            # root pubkey could not be identified.
            return False

        root_pubkey_info = ATTESTATION_PUBKEYS_MAP[root_pubkey_identifier]
        root_pubkey_bytes_uncompressed = root_pubkey_info["pubkey"]
        if (
            root_pubkey_info["accepted_bootloader_hash"] is not None
            and root_pubkey_info["accepted_bootloader_hash"] != bootloader_hash
        ):
            return False

        root_pubkey = ecdsa.VerifyingKey.from_string(
            root_pubkey_bytes_uncompressed[1:], ecdsa.curves.SECP256k1
        )

        device_pubkey = ecdsa.VerifyingKey.from_string(device_pubkey_bytes, ecdsa.curves.NIST256p)

        try:
            # Verify certificate
            if not root_pubkey.verify(
                certificate, bootloader_hash + device_pubkey_bytes, hashfunc=hashlib.sha256
            ):
                return False

            # Verify challenge
            if not device_pubkey.verify(challenge_signature, challenge, hashfunc=hashlib.sha256):
                return False
        except ecdsa.BadSignatureError:
            return False
        return True

    def _msg_query(
        self, request: hww.Request, expected_response: Optional[str] = None
    ) -> hww.Response:
        """
        Sends protobuf msg and retrieves protobuf response over an encrypted
        channel.
        """
        # pylint: disable=no-member
        if self.debug:
            print(request)
        response_bytes = self._noise_handler.encrypted_query(request.SerializeToString())
        response = hww.Response()
        response.ParseFromString(response_bytes)
        if response.WhichOneof("response") == "error":
            if response.error.code == ERR_USER_ABORT:
                raise UserAbortException(response.error.code, response.error.message)
            raise Bitbox02Exception(response.error.code, response.error.message)
        if expected_response is not None and response.WhichOneof("response") != expected_response:
            raise Exception(
                "Unexpected response: {}, expected: {}".format(
                    response.WhichOneof("response"), expected_response
                )
            )
        if self.debug:
            print(response)
        return response

    def reboot(self) -> bool:
        """TODO: Document"""
        # pylint: disable=no-member
        request = hww.Request()
        request.reboot.CopyFrom(system.RebootRequest())
        try:
            self._msg_query(request)
        except OSError:
            # In case of reboot we can't read the response.
            return True
        except Bitbox02Exception:
            return False
        return True

    def get_info(self) -> Tuple[str, Platform, Union[BitBox02Edition, BitBoxBaseEdition], bool]:
        """
        Returns (version, platform, edition, unlocked).
        """
        response_status, response = self._noise_handler.query(OP_INFO, b"")

        version_str_len = int(response_status[0])
        version, response = response[:version_str_len], response[version_str_len:]
        version_str = version.rstrip(b"\0").decode("ascii")

        platform_byte, response = response[0], response[1:]
        platform = {0x00: Platform.BITBOX02}[platform_byte]

        edition_byte, response = response[0], response[1:]
        edition: Union[BitBox02Edition, BitBoxBaseEdition]
        if platform == Platform.BITBOX02:
            edition = {0x00: BitBox02Edition.MULTI, 0x01: BitBox02Edition.BTCONLY}[edition_byte]
        else:
            edition = {0x00: BitBoxBaseEdition.STANDARD}[edition_byte]

        unlocked_byte = response[0]
        unlocked = {0x00: False, 0x01: True}[unlocked_byte]
        return (version_str, platform, edition, unlocked)

    def close(self) -> None:
        self._noise_handler.close()
