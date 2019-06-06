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


import os
import sys
import time
import base64
import binascii
from datetime import datetime
import hashlib

import ecdsa
from noise.connection import NoiseConnection, Keypair
import hid

from .usb import hid_send_frames, hid_read_frames

try:
    from .generated import hww_pb2 as hww
except ModuleNotFoundError:
    print("Run `make messages` to generate the protobuf messages")
    sys.exit()

HWW_CMD = 0x80 + 0x40 + 0x01

ERR_GENERIC = 103

HARDENED = 0x80000000

# values: uncompressed secp256k1 pubkey serialization.
ATTESTATION_PUBKEYS = [
    binascii.unhexlify(
        "04074ff1273b36c24e80fe3d59e0e897a81732d3f8e9cd07e17e9fc06319cd16b25cf74255674477b3ac9cbac2d12f0dc27a662681fcbc12955b0bccdcbbdcfd01"
    )
]

ATTESTATION_PUBKEYS_MAP = {hashlib.sha256(val).digest(): val for val in ATTESTATION_PUBKEYS}

OP_I_CAN_HAS_HANDSHAEK = b"h"
OP_I_CAN_HAS_PAIRIN_VERIFICASHUN = b"v"

RESPONSE_SUCCESS = b"\x00"
RESPONSE_FAILURE = b"\x01"


class Bitbox02Exception(Exception):
    def __init__(self, code, message):
        self.code = code
        self.message = message
        super().__init__()

    def __str__(self):
        return f"error code: {self.code}, message: {self.message}"


class BitBox02:
    """Class to communicate with a BitBox02"""

    def __init__(self, device_path, show_pairing_callback):
        self.debug = False
        self.device = hid.device()
        self.device.open_path(device_path)

        if self._query(OP_I_CAN_HAS_HANDSHAEK) != RESPONSE_SUCCESS:
            raise Exception("Couldn't kick off handshake")

        # init noise channel
        noise = NoiseConnection.from_name(b"Noise_XX_25519_ChaChaPoly_SHA256")
        noise.set_as_initiator()
        dummy_private_key = os.urandom(32)
        noise.set_keypair_from_private_bytes(Keypair.STATIC, dummy_private_key)
        noise.set_prologue(b"Noise_XX_25519_ChaChaPoly_SHA256")
        noise.start_handshake()
        noise.read_message(self._query(noise.write_message()))
        assert not noise.handshake_finished
        send_msg = noise.write_message()
        assert noise.handshake_finished
        pairing_code = base64.b32encode(noise.get_handshake_hash()).decode("ascii")
        show_pairing_callback(
            "{} {}\n{} {}".format(
                pairing_code[:5], pairing_code[5:10], pairing_code[10:15], pairing_code[15:20]
            )
        )
        response = self._query(send_msg)

        # Can be set to False if the remote static pubkey was previously confirmed.
        pairing_verification_required_by_host = True

        pairing_verification_required_by_device = response == b"\x01"
        if pairing_verification_required_by_host or pairing_verification_required_by_device:
            pairing_response = self._query(OP_I_CAN_HAS_PAIRIN_VERIFICASHUN)
            if pairing_response == RESPONSE_SUCCESS:
                pass
            elif pairing_response == RESPONSE_FAILURE:
                raise Exception("pairing rejected by the user")
            else:
                raise Exception("unexpected response")
        self.noise = noise

    def close(self):
        self.device.close()

    def _query(self, msg):
        """
        Sends msg bytes and retrieves response bytes.
        """
        hid_send_frames(self.device, msg, cmd=HWW_CMD)
        return bytes(hid_read_frames(self.device, cmd=HWW_CMD))

    def _encrypted_query(self, msg):
        """
        Sends msg bytes and reads response bytes over an encrypted channel.
        """
        return self.noise.decrypt(self._query(self.noise.encrypt(msg)))

    def _msg_query(self, request, expected_response=None):
        """
        Sends protobuf msg and retrieves protobuf response over an encrypted
        channel.
        """
        # pylint: disable=no-member
        if self.debug:
            print(request)
        response_bytes = self._encrypted_query(request.SerializeToString())
        response = hww.Response()
        response.ParseFromString(response_bytes)
        if response.WhichOneof("response") == "error":
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

    def random_number(self):
        # pylint: disable=no-member
        request = hww.Request()
        request.random_number.CopyFrom(hww.RandomNumberRequest())
        response = self._msg_query(request, expected_response="random_number")
        return response.random_number.number

    def device_info(self):
        # pylint: disable=no-member
        request = hww.Request()
        device_info_request = hww.DeviceInfoRequest()
        request.device_info.CopyFrom(device_info_request)
        response = self._msg_query(request, expected_response="device_info")
        return {
            "name": response.device_info.name,
            "version": response.device_info.version,
            "initialized": response.device_info.initialized,
            "mnemonic_passphrase_enabled": response.device_info.mnemonic_passphrase_enabled,
            "monotonic_increments_remaining": response.device_info.monotonic_increments_remaining,
        }

    def set_device_name(self, device_name):
        # pylint: disable=no-member
        request = hww.Request()
        request.device_name.name = device_name
        self._msg_query(request, expected_response="success")

    def set_password(self):
        """
        Returns True if the user entered the password correctly (passwords match).
        Returns False otherwise.
        """
        # pylint: disable=no-member
        request = hww.Request()
        request.set_password.entropy = os.urandom(32)
        try:
            self._msg_query(request, expected_response="success")
        except Bitbox02Exception as err:
            if err.code == ERR_GENERIC:
                return False
            raise
        return True

    def create_backup(self):
        """
        Returns True if the backup was created successfully.
        Returns False otherwise.
        """
        # pylint: disable=no-member
        request = hww.Request()
        request.create_backup.timestamp = int(time.time())
        request.create_backup.timezone_offset = time.localtime().tm_gmtoff
        try:
            self._msg_query(request, expected_response="success")
        except Bitbox02Exception as err:
            if err.code == ERR_GENERIC:
                return False
            raise
        return True

    def list_backups(self):
        """
        Returns a pair of id and timestamp's strings that identify the backups.
        """
        # pylint: disable=no-member
        self.insert_or_remove_sdcard(insert=True)
        request = hww.Request()
        request.list_backups.CopyFrom(hww.ListBackupsRequest())
        response = self._msg_query(request, expected_response="list_backups")
        for info in response.list_backups.info:
            utcdate = datetime.utcfromtimestamp(info.timestamp)
            yield (info.id, info.name, utcdate)

    def restore_backup(self, backup_id):
        """
        Sends a restore API call to the BitBox.
        """
        # pylint: disable=no-member
        request = hww.Request()
        request.restore_backup.id = backup_id
        try:
            self._msg_query(request, expected_response="success")
        except Bitbox02Exception as err:
            if err.code == ERR_GENERIC:
                return False
            raise
        return True

    def check_backup(self, silent=False):
        """
        Sends a check backup API call to the BitBox.
        Returns the backup ID if the backup was found and can be restored.
        Otherwise, returns None. If silent is True, the result won't be shown on the device screen.
        """
        # pylint: disable=no-member
        self.insert_or_remove_sdcard(insert=True)
        request = hww.Request()
        request.check_backup.CopyFrom(hww.CheckBackupRequest(silent=silent))
        try:
            response = self._msg_query(request, expected_response="check_backup")
        except Bitbox02Exception as err:
            if err.code == ERR_GENERIC:
                return None
            raise
        return response.check_backup.id

    def show_mnemonic(self):
        """
        Returns True if mnemonic was successfully shown and confirmed.
        Returns False otherwise.
        """
        # pylint: disable=no-member
        request = hww.Request()
        request.show_mnemonic.CopyFrom(hww.ShowMnemonicRequest())
        try:
            self._msg_query(request, expected_response="success")
        except Bitbox02Exception as err:
            if err.code == ERR_GENERIC:
                return False
            raise
        return True

    def btc_pub(
        self,
        keypath=None,
        coin=hww.BTC,
        output_type=hww.BTCPubRequest.XPUB,
        script_type=hww.SCRIPT_UNKNOWN,
        display=True,
    ):
        """
        keypath is a list of child derivation numbers.
        e.g. m/44'/0'/1'/5 corresponds to [44+HARDENED, 0+HARDENED, 1+HARDENED, 5].
        """
        # pylint: disable=no-member,too-many-arguments
        keypath = [] if keypath is None else keypath
        request = hww.Request()
        request.btc_pub.CopyFrom(
            hww.BTCPubRequest(
                coin=coin,
                keypath=keypath,
                output_type=output_type,
                script_type=script_type,
                display=display,
            )
        )
        return self._msg_query(request).pub.pub

    def check_sdcard(self):
        # pylint: disable=no-member
        request = hww.Request()
        request.check_sdcard.CopyFrom(hww.CheckSDCardRequest())
        response = self._msg_query(request, expected_response="check_sdcard")
        return response.check_sdcard.inserted

    def insert_or_remove_sdcard(self, insert=False, remove=False):
        """TODO: document"""
        # pylint: disable=no-member
        request = hww.Request()
        if insert:
            request.insert_remove_sdcard.CopyFrom(
                hww.InsertRemoveSDCardRequest(action=hww.InsertRemoveSDCardRequest.INSERT_CARD)
            )
        elif remove:
            request.insert_remove_sdcard.CopyFrom(
                hww.InsertRemoveSDCardRequest(action=hww.InsertRemoveSDCardRequest.REMOVE_CARD)
            )
        else:
            raise Exception("Invalid action")
        self._msg_query(request, expected_response="success")

    def set_mnemonic_passphrase_enabled(self, enabled):
        """
        Enable or disable the bip39 passphrase.
        """
        # pylint: disable=no-member
        request = hww.Request()
        request.set_mnemonic_passphrase_enabled.enabled = enabled
        self._msg_query(request, expected_response="success")

    def perform_attestation(self):
        """TODO: Document"""
        # pylint: disable=no-member
        request = hww.Request()
        challenge = os.urandom(32)
        request.perform_attestation.challenge = challenge
        response = self._msg_query(request, expected_response="perform_attestation")

        data = response.perform_attestation

        if data.root_pubkey_identifier not in ATTESTATION_PUBKEYS_MAP:
            # root pubkey could not be identified.
            return False

        root_pubkey_bytes_uncompressed = ATTESTATION_PUBKEYS_MAP[data.root_pubkey_identifier]
        root_pubkey = ecdsa.VerifyingKey.from_string(
            root_pubkey_bytes_uncompressed[1:], ecdsa.curves.SECP256k1
        )

        device_pubkey = ecdsa.VerifyingKey.from_string(data.device_pubkey, ecdsa.curves.NIST256p)

        try:
            # Verify certificate
            if not root_pubkey.verify(
                data.certificate, data.bootloader_hash + data.device_pubkey, hashfunc=hashlib.sha256
            ):
                return False

            # Verify challenge
            if not device_pubkey.verify(
                data.challenge_signature, challenge, hashfunc=hashlib.sha256
            ):
                return False
        except ecdsa.BadSignatureError:
            return False
        return True

    def reboot(self):
        """TODO: Document"""
        # pylint: disable=no-member
        request = hww.Request()
        request.reboot.CopyFrom(hww.RebootRequest())
        try:
            self._msg_query(request)
        except OSError:
            # In case of reboot we can't read the response.
            return True
        except Bitbox02Exception:
            return False
        return True
