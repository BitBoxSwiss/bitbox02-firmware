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
"""U2F packet types"""
import binascii
import hashlib
import base64
import json
import random
from typing import Tuple
from typing_extensions import Protocol

import ecdsa

U2F_REGISTER = 0x01
U2F_AUTHENTICATE = 0x02
U2F_VERSION = 0x03

U2F_SW_NO_ERROR = 0x9000
U2F_SW_WRONG_LENGTH = 0x6700
U2F_SW_DATA_INVALID = 0x6984
U2F_SW_CONDITIONS_NOT_SATISFIED = 0x6985
U2F_SW_WRONG_DATA = 0x6A80
U2F_SW_INS_NOT_SUPPORTED = 0x6D00
U2F_SW_CLA_NOT_SUPPORTED = 0x6E00


class WrongLengthException(Exception):
    def __init__(self) -> None:
        super().__init__(self, "Wrong data")


class DataInvalidException(Exception):
    def __init__(self) -> None:
        super().__init__(self, "Wrong data")


class ConditionsNotSatisfiedException(Exception):
    def __init__(self) -> None:
        super().__init__(self, "Conditions not satisfied")


class WrongDataException(Exception):
    def __init__(self) -> None:
        super().__init__(self, "Wrong data")


def _status_code_to_exception(code_bytes: bytes) -> None:
    code = code_bytes[0] * 256 + code_bytes[1]
    if code == U2F_SW_WRONG_LENGTH:
        raise WrongLengthException()
    if code == U2F_SW_DATA_INVALID:
        raise DataInvalidException()
    if code == U2F_SW_CONDITIONS_NOT_SATISFIED:
        raise ConditionsNotSatisfiedException()
    if code == U2F_SW_WRONG_DATA:
        raise WrongDataException()


def _random_32() -> bytes:
    random_bytes = bytearray(32)
    for i in range(0, 32):
        random_bytes[i] = round(random.uniform(0, 255))
    return random_bytes


def _der_to_sig(der: bytes) -> Tuple[int, bytes]:
    """Convert from DER to raw signature"""
    # Structure is:
    #   0x30 0xNN  SEQUENCE + s_length
    #   0x02 0xNN  INTEGER + r_length
    #   0xAA 0xBB  ..   r_length bytes of "r" (offset 4)
    #   0x02 0xNN  INTEGER + s_length
    #   0xMM 0xNN  ..   s_length bytes of "s" (offset 6 + r_len)
    if len(der) < 8 or der[0] != 0x30 or der[2] != 0x02:
        raise Exception("failed to parse")

    seq_len = der[1]
    if seq_len <= 0:
        raise Exception("failed to parse")

    r_len = der[3]
    if r_len < 1 or r_len > seq_len - 5 or der[4 + r_len] != 0x02:
        raise Exception("failed to parse")

    s_len = der[5 + r_len]
    if s_len < 1 or s_len != seq_len - 4 - r_len:
        raise Exception("failed to parse")

    sig_64 = bytearray(64 * [0])
    sig_r = der[4 : 4 + r_len]
    for i in range(32):
        sig_64[31 - i] = sig_r[len(sig_r) - 1 - i]
        if len(sig_r) == i:
            break

    sig_s = der[6 + r_len : 6 + r_len + s_len]
    for i in range(32):
        sig_64[63 - i] = sig_s[len(sig_s) - 1 - i]
        if len(sig_s) == i:
            break

    return (seq_len + 2, sig_64)


class U2FSender(Protocol):
    # pylint: disable=too-few-public-methods,unused-argument,no-self-use
    def u2fhid_msg(self, msg: bytes) -> bytes:
        ...


class APDU:
    """Format that U2F is using"""

    # pylint: disable=too-few-public-methods
    def __init__(self, ins: int, p1: int, p2: int, length: int, data: bytes):
        """
        Sets up the APDU packet.
        ins: command code, 1: register, 2: authenticate, 3: version
        p1: parameters for the command, for authenticate:
            0x07: check-only
            0x03: enforce-user-presence-and-sign
            0X08: dont-enforce-user-presence-and-sign
        """
        # pylint: disable=too-many-arguments
        self._apdubytes = bytearray()
        self._apdubytes.append(0)
        self._apdubytes.append(ins)
        self._apdubytes.append(p1)
        self._apdubytes.append(p2)
        self._apdubytes.append((length >> 16) & 0xFF)
        self._apdubytes.append((length >> 8) & 0xFF)
        self._apdubytes.append((length >> 0) & 0xFF)
        self._apdubytes.extend(data)

    def __bytes__(self) -> bytes:
        return self._apdubytes


class RegistrationResponse:
    """Response to registration request"""

    # pylint: disable=too-many-instance-attributes,too-few-public-methods
    def __init__(self, response_bytes: bytes, challenge: bytes, appid: bytes):
        """
        Creates an registration response.
        """
        # Can't check this like this, to many bytes...
        # sw = response_bytes[-1] << 16
        # sw += response_bytes[-2]
        # if sw != U2F_SW_NO_ERROR:
        #    raise Exception("Device reported error")
        self._challenge = challenge
        self._appid = appid
        self.register_id = response_bytes[0]
        self.ec_public_key = response_bytes[1:66]
        # print(f"pk {len(self.ec_public_key)}: {binascii.hexlify(self.ec_public_key)}")
        self.key_handle_length = response_bytes[66]
        end = 67 + self.key_handle_length
        self.key_handle = response_bytes[67:end]
        (cert_len, self._cert) = self._get_cert(response_bytes[end:])
        # print(f"cert {cert_len}: {binascii.hexlify(self._cert)}")
        # sig_len = kh_cert_sig_max_len - self.key_handle_length - cert_len
        # print(f"sig_len {sig_len}")
        (_sig_len, self._sig) = _der_to_sig(response_bytes[end + cert_len :])
        # print(f"sig  {sig_len}: {binascii.hexlify(response_bytes[end+cert_len:])}")
        # print(f"sig  {len(self._sig)}: {binascii.hexlify(self._sig)}")
        self._subject_public_key = self._get_subject_pub_key(self._cert)
        # print(f"pk {len(self._subject_public_key)}: {binascii.hexlify(self._subject_public_key)}")

    @staticmethod
    def _get_cert(msg: bytes) -> Tuple[int, bytes]:
        """Parse certificate out of msg"""
        if msg[0] != 0x30:
            raise Exception("Failed to parse")

        if msg[1] != 0x81 and msg[1] != 0x82:
            raise Exception("Failed to parse")

        if msg[1] == 0x81:
            cert_len = msg[2]
            header_len = 3
        elif msg[1] == 0x82:
            cert_len = msg[2] * 256 + msg[3]
            header_len = 4

        return (cert_len + header_len, bytes(msg[: cert_len + header_len]))

    @staticmethod
    def _get_subject_pub_key(cert: bytes) -> bytes:
        """Parse public key out of certificate"""
        needle = binascii.unhexlify(b"3059301306072a8648ce3d020106082a8648ce3d030107034200")
        idx = cert.find(needle)
        if idx == -1:
            raise Exception("Did not find pk in cert")
        idx += len(needle)
        return bytes(cert[idx : idx + 65])

    def verify(self) -> bool:
        """Returns true if the signature matches the certificate"""
        hasher = hashlib.sha256()
        hasher.update(b"\00")  # rfu
        hasher.update(self._appid)  # appid
        if len(self._appid) < 32:
            hasher.update(b"\00" * (32 - len(self._appid)))
        hasher.update(self._challenge)  # challenge
        if len(self._challenge) < 32:
            hasher.update(b"\00" * (32 - len(self._challenge)))
        hasher.update(self.key_handle)  # keyhandle
        hasher.update(self.ec_public_key)  # pub_key
        # print(f"hash: {hasher.hexdigest()}")
        key = ecdsa.VerifyingKey.from_string(self._subject_public_key[1:], curve=ecdsa.NIST256p)

        return bool(key.verify_digest(self._sig, hasher.digest()))


class RegistrationRequest:
    """Container for request"""

    # pylint: disable=too-few-public-methods
    def __init__(self, app_id: str, bogus: str = ""):
        """
        Creates an APDU packet for request.
        """
        if bogus not in ["", "chromium", "firefox"]:
            raise ValueError("Invalid bogus keyword value")
        self._app_id = app_id
        client_data = {
            "typ": "navigator.id.finishEnrollment",
            "challenge": base64.b64encode(_random_32()).decode("utf-8"),
            "origin": app_id,
        }
        client_data_hash = hashlib.sha256()
        client_data_hash.update(json.dumps(client_data).encode("utf-8"))
        self._challenge_parameter = client_data_hash.digest()
        if bogus == "chromium":
            self._app_parameter = b"A" * 32
        elif bogus == "firefox":
            self._app_parameter = b"\00" * 32
        else:
            app_id_hash = hashlib.sha256()
            app_id_hash.update(app_id.encode("utf-8"))
            self._app_parameter = app_id_hash.digest()
        data = bytearray(64)
        for i in range(0, 32):
            data[i] = self._challenge_parameter[i]
        for i in range(0, 32):
            data[32 + i] = self._app_parameter[i]
        self._apdu = APDU(U2F_REGISTER, 0x00, 0x00, len(data), data)

    def send(self, bitbox: U2FSender) -> RegistrationResponse:
        response_bytes = bitbox.u2fhid_msg(self._apdu.__bytes__())
        _status_code_to_exception(response_bytes[-2:])

        return RegistrationResponse(response_bytes, self._challenge_parameter, self._app_parameter)


class AuthenticationResponse:
    """Reponse to an authentication request"""

    # pylint: disable=too-few-public-methods
    def __init__(self, response_bytes: bytes, appid: bytes, challenge: bytes):
        """
        Creates an authentication response.
        """
        self._appid = appid
        self._challenge = challenge
        self.flags = response_bytes[0]
        self.ctr = response_bytes[1:5]
        (_sig_len, self.sig) = _der_to_sig(response_bytes[5:])

    def verify(self, public_key: bytes) -> bool:
        """Returns true if the reponse has been signed by the public key"""
        hasher = hashlib.sha256()
        hasher.update(self._appid)  # appid
        if len(self._appid) < 32:
            hasher.update(b"\00" * (32 - len(self._appid)))
        hasher.update(b"\01")  # user presence
        hasher.update(self.ctr)  # counter
        hasher.update(self._challenge)  # challenge
        if len(self._challenge) < 32:
            hasher.update(b"\00" * (32 - len(self._challenge)))
        # print(f"hash: {hasher.hexdigest()}")
        key = ecdsa.VerifyingKey.from_string(public_key[1:], curve=ecdsa.NIST256p)

        return bool(key.verify_digest(self.sig, hasher.digest()))


class AuthenticationRequest:
    """Authentication request"""

    # pylint: disable=too-few-public-methods
    def __init__(self, app_id: str, key_handle: bytes):
        """
        Creates an APDU packet for request.
        """
        data = bytearray()
        client_data = {
            "typ": "navigator.id.getAssertion",
            "challenge": base64.b64encode(_random_32()).decode("utf-8"),
            "origin": app_id,
        }
        client_data_hash = hashlib.sha256()
        client_data_hash.update(json.dumps(client_data).encode("utf-8"))
        self._challenge_parameter = client_data_hash.digest()
        data.extend(self._challenge_parameter)

        app_id_hash = hashlib.sha256()
        app_id_hash.update(app_id.encode("utf-8"))
        self._app_parameter = app_id_hash.digest()
        data.extend(self._app_parameter)

        data.append(len(key_handle))
        data.extend(key_handle)

        # control byte 0x03: enforce-user-presence-and-sign
        self._apdu = APDU(U2F_AUTHENTICATE, 0x03, 0x00, len(data), data)

    def send(self, bitbox: U2FSender) -> AuthenticationResponse:
        response_bytes = bitbox.u2fhid_msg(self._apdu.__bytes__())
        _status_code_to_exception(response_bytes[-2:])
        return AuthenticationResponse(
            response_bytes, self._app_parameter, self._challenge_parameter
        )


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
