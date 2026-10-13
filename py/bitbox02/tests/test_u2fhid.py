# SPDX-License-Identifier: Apache-2.0

"""U2F HID payload size validation."""

import struct
import unittest
from unittest import mock

from bitbox02.communication import PhysicalLayer
from bitbox02.communication.u2fhid.u2fhid import U2FHid, MSG


class TestU2FHid(unittest.TestCase):
    def test_write_max_payload(self) -> None:
        device = mock.Mock(spec=PhysicalLayer)
        transport = U2FHid(device)
        payload = bytes(index % 256 for index in range(7609))
        transport.write(payload, MSG, 123)
        frames = [call[0][0] for call in device.write.call_args_list]
        self.assertEqual(len(frames), 129)
        self.assertTrue(all(len(frame) == 65 for frame in frames))
        self.assertEqual(frames[0][:8], b"\0" + struct.pack(">IBH", 123, MSG, len(payload)))
        for seq, frame in enumerate(frames[1:]):
            self.assertEqual(frame[:6], b"\0" + struct.pack(">IB", 123, seq))
        self.assertEqual(frames[0][8:] + b"".join(frame[6:] for frame in frames[1:]), payload)

    def test_write_oversized_payload(self) -> None:
        for size in (7610, 0xFFFF, 0x10000):
            with self.subTest(size=size):
                device = mock.Mock(spec=PhysicalLayer)
                transport = U2FHid(device)
                with self.assertRaisesRegex(ValueError, "Data is too large"):
                    transport.write(b"\0" * size, MSG, 123)
                device.write.assert_not_called()


if __name__ == "__main__":
    unittest.main()
