# SPDX-License-Identifier: Apache-2.0

"""Tests for starting a fresh host session before regular connection setup."""

import unittest
from typing import Optional
from unittest import mock

import semver

from bitbox02.communication import bitbox_api_protocol as protocol
from bitbox02.communication.communication import TransportLayer
from bitbox02.communication.devices import BITBOX02MULTI, BITBOX03


class TestSession(unittest.TestCase):
    """Session startup and compatibility with older firmware."""

    def test_reset_session_version_gate_and_order(self) -> None:
        """Discover the version if needed, then reset supported devices before attestation."""
        for version in ("v9.27.2", "v9.28.0", "v9.28.0-dev", "v9.29.0"):
            for product in (BITBOX02MULTI, BITBOX03, None):
                with self.subTest(version=version, product=product):
                    self._check_session_setup(version, product)

    def _check_session_setup(self, version: str, product: Optional[str]) -> None:
        discover_version = product in (BITBOX03, None)
        events = []
        transport = mock.Mock(spec=TransportLayer)
        transport.generate_cid.return_value = 123

        def query(data: bytes, endpoint: int, cid: int) -> bytes:
            self.assertEqual(endpoint, protocol.HWW_CMD)
            self.assertEqual(cid, 123)
            events.append(data)
            if data == protocol.HwwRequestCode.REQ_INFO:
                encoded = version.encode("ascii")
                platform = 0x03 if product == BITBOX03 else 0x00
                return bytes([len(encoded)]) + encoded + bytes([platform, 0, 0, 0])
            self.assertEqual(data, protocol.HwwRequestCode.REQ_RESET)
            return protocol.HwwResponseCode.RSP_ACK

        transport.query.side_effect = query
        device_info = (
            None
            if product is None
            else {
                "serial_number": "" if product == BITBOX03 else version,
                "product_string": product,
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
        ):
            api = protocol.BitBoxCommonAPI(transport, device_info, protocol.BitBoxNoiseConfig())
        self.assertEqual(
            api.version, semver.VersionInfo.parse(version[1:]).replace(prerelease=None)
        )
        self.assertEqual(api.edition, protocol.BitBox02Edition.MULTI)
        expected = [protocol.HwwRequestCode.REQ_INFO] if discover_version else []
        if version != "v9.27.2":
            expected.append(protocol.HwwRequestCode.REQ_RESET)
        expected.extend(["attestation", "unlock", "noise"])
        self.assertEqual(events, expected)

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
