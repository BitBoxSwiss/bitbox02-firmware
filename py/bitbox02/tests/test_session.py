# SPDX-License-Identifier: Apache-2.0

"""Tests for starting a fresh host session before regular connection setup."""

import unittest
from unittest import mock

import semver

from bitbox02.communication import bitbox_api_protocol as protocol
from bitbox02.communication.communication import TransportLayer
from bitbox02.communication.devices import BITBOX02MULTI
from bitbox02.communication.generated import hww_pb2 as hww
from bitbox02.communication.generated import keystore_pb2 as keystore


class TestSession(unittest.TestCase):
    """Session startup and compatibility with older firmware."""

    def test_reset_session_version_gate_and_order(self) -> None:
        """Discover the version if needed, then reset supported devices before attestation."""
        for version in ("v9.27.2", "v9.28.0", "v9.28.0-dev", "v9.29.0"):
            for discover_version in (False, True):
                with self.subTest(version=version, discover_version=discover_version):
                    self._check_session_setup(version, discover_version)

    def _check_session_setup(self, version: str, discover_version: bool) -> None:
        events = []
        transport = mock.Mock(spec=TransportLayer)
        transport.generate_cid.return_value = 123

        def query(data: bytes, endpoint: int, cid: int) -> bytes:
            self.assertEqual(endpoint, protocol.HWW_CMD)
            self.assertEqual(cid, 123)
            events.append(data)
            if data == protocol.HwwRequestCode.REQ_INFO:
                encoded = version.encode("ascii")
                return bytes([len(encoded)]) + encoded + b"\x00\x00\x00\x01"
            self.assertEqual(data, protocol.HwwRequestCode.REQ_RESET)
            return protocol.HwwResponseCode.RSP_ACK

        transport.query.side_effect = query
        device_info = (
            None
            if discover_version
            else {
                "serial_number": version,
                "product_string": BITBOX02MULTI,
                "path": b"device",
            }
        )
        with mock.patch.object(
            protocol.BitBoxCommonAPI,
            "_perform_attestation",
            side_effect=lambda: events.append("attestation") or True,
        ), mock.patch.object(
            protocol.BitBoxProtocolV7,
            "unlock_query",
            side_effect=lambda: events.append("unlock"),
        ), mock.patch.object(
            protocol.BitBoxProtocolV7,
            "noise_connect",
            side_effect=lambda _config: events.append("noise"),
        ), mock.patch.object(
            protocol.BitBoxCommonAPI,
            "_unlock",
            side_effect=lambda _config: events.append("encrypted unlock"),
        ):
            protocol.BitBoxCommonAPI(transport, device_info, protocol.BitBoxNoiseConfig())
        expected = [protocol.HwwRequestCode.REQ_INFO] if discover_version else []
        if version != "v9.27.2":
            expected.append(protocol.HwwRequestCode.REQ_RESET)
        if version == "v9.27.2":
            expected.extend(["attestation", "unlock", "noise"])
        else:
            expected.extend(["attestation", "noise", "encrypted unlock"])
        self.assertEqual(events, expected)

    def test_unlock_without_initialization_query(self) -> None:
        """An immediate DONE completes connection setup without querying initialization."""
        # pylint: disable=no-member
        transport = mock.Mock(spec=TransportLayer)
        transport.query.side_effect = [protocol.HwwResponseCode.RSP_ACK]
        enter = mock.Mock()
        with mock.patch.object(protocol.BitBoxCommonAPI, "_perform_attestation"), mock.patch.object(
            protocol.BitBoxProtocolV7, "noise_connect"
        ), mock.patch.object(
            protocol.BitBoxCommonAPI,
            "_msg_query",
            return_value=hww.Response(
                unlock=keystore.UnlockResponse(state=keystore.UnlockResponse.DONE)
            ),
        ) as query:
            protocol.BitBoxCommonAPI(
                transport,
                {"serial_number": "v9.28.0", "product_string": BITBOX02MULTI},
                protocol.BitBoxNoiseConfig(),
                protocol.BitBoxConfig(enter_mnemonic_passphrase=enter),
            )
        query.assert_called_once_with(
            hww.Request(unlock=keystore.UnlockRequest()), expected_response="unlock"
        )
        self.assertEqual(transport.query.call_count, 1)
        enter.assert_not_called()

    def test_reset_session_busy(self) -> None:
        """Wait for another owner of the UI before continuing startup."""
        transport = mock.Mock(spec=TransportLayer)
        transport.generate_cid.return_value = 123
        transport.query.side_effect = [
            protocol.HwwResponseCode.RSP_BUSY,
            protocol.HwwResponseCode.RSP_ACK,
        ]
        with mock.patch.object(protocol.time, "sleep") as sleep:
            protocol.BitBoxCommonAPI._reset_session(  # pylint: disable=protected-access
                transport, semver.VersionInfo(9, 28, 0)
            )
        sleep.assert_called_once_with(1)
        self.assertEqual(
            transport.query.call_args_list,
            [mock.call(protocol.HwwRequestCode.REQ_RESET, protocol.HWW_CMD, 123)] * 2,
        )

    def test_reset_session_requires_ack(self) -> None:
        """Reject malformed replies and firmware that does not acknowledge cleanup."""
        for response in (b"", protocol.HwwResponseCode.RSP_NACK, b"\x00payload"):
            with self.subTest(response=response):
                transport = mock.Mock(spec=TransportLayer)
                transport.query.return_value = response
                with self.assertRaisesRegex(Exception, "Unexpected response to RESET"):
                    protocol.BitBoxCommonAPI._reset_session(  # pylint: disable=protected-access
                        transport, semver.VersionInfo(9, 28, 0)
                    )


if __name__ == "__main__":
    unittest.main()
