# SPDX-License-Identifier: Apache-2.0

"""Tests for py/load_firmware.py and its bootloader helpers."""

import contextlib
import importlib.util
import io
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
PYTHON_PACKAGE_ROOT = REPOSITORY_ROOT / "py" / "bitbox02"
sys.path.insert(0, str(PYTHON_PACKAGE_ROOT))

MODULE_PATH = REPOSITORY_ROOT / "py" / "load_firmware.py"
SPEC = importlib.util.spec_from_file_location("load_firmware_under_test", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"Could not load {MODULE_PATH}")
LOAD_FIRMWARE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = LOAD_FIRMWARE
SPEC.loader.exec_module(LOAD_FIRMWARE)

import bitbox02 as bitbox02_package  # noqa: E402
from bitbox02.bitbox02 import bootloader as bootloader_module  # noqa: E402


MAGIC = bootloader_module.SIGDATA_MAGIC_BITBOX02_MULTI
OTHER_MAGIC = bootloader_module.SIGDATA_MAGIC_BITBOX02_BTCONLY
SIGDATA = bytes(bootloader_module.SIGDATA_LEN)
PAYLOAD = b"firmware payload"


def device_info(serial_number: str = "v1.2.2") -> dict:
    """Return minimal HID device information for a Multi bootloader."""
    return {
        "serial_number": serial_number,
        "path": b"test-path",
        "product_string": bootloader_module.BB02MULTI_BOOTLOADER,
    }


def signed_input(magic: bytes = MAGIC) -> LOAD_FIRMWARE.FirmwareInput:
    """Build a parsed signed-firmware input."""
    return LOAD_FIRMWARE.FirmwareInput(PAYLOAD, magic, SIGDATA)


class DummyTransport:
    """Minimal bootloader transport for package-level tests."""

    def __init__(self, response_code: int = 0) -> None:
        self.response_code = response_code
        self.queries = []

    def generate_cid(self) -> int:
        return 1

    def query(self, data: bytes, _endpoint: int, _cid: int) -> bytes:
        self.queries.append(data)
        return bytes((data[0], self.response_code))

    def write(self, _data: bytes, _endpoint: int, _cid: int) -> None:
        pass

    def close(self) -> None:
        pass


class BootloaderPackageTests(unittest.TestCase):
    """Test the public bootloader helpers used by the loader."""

    def test_compat_package_exports_bootloader_module(self) -> None:
        self.assertIs(bitbox02_package.bootloader, bootloader_module)

    def test_parse_signed_firmware_wrong_magic(self) -> None:
        with self.assertRaises(bootloader_module.InvalidFirmwareMagic):
            bootloader_module.parse_signed_firmware(PAYLOAD)

    def test_parse_signed_firmware_rejects_invalid_payload_size(self) -> None:
        for payload in (b"", b"x" * (bootloader_module.MAX_FIRMWARE_SIZE + 1)):
            with self.subTest(size=len(payload)), self.assertRaises(ValueError):
                bootloader_module.parse_signed_firmware(MAGIC + SIGDATA + payload)

    def test_is_devdevice(self) -> None:
        for serial_number in (
            "bb02.bl:v1.0.0-dev",
            "v1.2.2+dev",
            "v1.2.2+git.123.dev",
        ):
            with self.subTest(serial_number=serial_number):
                bootloader = bootloader_module.Bootloader(
                    DummyTransport(), device_info(serial_number)
                )
                self.assertTrue(bootloader.is_devdevice())

    def test_is_devdevice_rejects_other_version_identifiers(self) -> None:
        for serial_number in (
            "v1.2.2",
            "v1.2.2+git.123",
            "v1.2.2+device",
            "v1.2.2-development",
        ):
            with self.subTest(serial_number=serial_number):
                bootloader = bootloader_module.Bootloader(
                    DummyTransport(), device_info(serial_number)
                )
                self.assertFalse(bootloader.is_devdevice())

    def test_is_devdevice_preserves_normalized_version(self) -> None:
        bootloader = bootloader_module.Bootloader(
            DummyTransport(), device_info("bb02.bl:v1.0.0-dev")
        )
        self.assertEqual(str(bootloader.version), "1.0.0")

    def test_query_raises_structured_bootloader_error(self) -> None:
        bootloader = bootloader_module.Bootloader(DummyTransport(ord("V")), device_info())
        with self.assertRaises(bootloader_module.BootloaderError) as raised:
            bootloader.versions()
        self.assertEqual(raised.exception.code, ord("V"))

    def test_flash_sigdata(self) -> None:
        transport = DummyTransport()
        bootloader = bootloader_module.Bootloader(transport, device_info())
        bootloader.flash_sigdata(SIGDATA)
        self.assertEqual(transport.queries, [b"s" + SIGDATA])

    def test_flash_sigdata_rejects_wrong_length(self) -> None:
        bootloader = bootloader_module.Bootloader(DummyTransport(), device_info())
        with self.assertRaises(ValueError):
            bootloader.flash_sigdata(SIGDATA[:-1])

    def test_flash_unsigned_firmware_rejects_oversized_payload(self) -> None:
        transport = DummyTransport()
        bootloader = bootloader_module.Bootloader(transport, device_info())
        with self.assertRaises(ValueError):
            bootloader.flash_unsigned_firmware(b"x" * (bootloader_module.MAX_FIRMWARE_SIZE + 1))
        self.assertEqual(transport.queries, [])


class FirmwareInputTests(unittest.TestCase):
    """Test input classification before a device is touched."""

    def _write(self, data: bytes) -> str:
        temporary = tempfile.NamedTemporaryFile(delete=False)
        self.addCleanup(Path(temporary.name).unlink, missing_ok=True)
        temporary.write(data)
        temporary.close()
        return temporary.name

    def test_read_signed_firmware(self) -> None:
        filename = self._write(MAGIC + SIGDATA + PAYLOAD)
        firmware = LOAD_FIRMWARE._read_firmware(filename)
        self.assertEqual(firmware, signed_input())

    def test_read_unsigned_firmware(self) -> None:
        for payload in (b"x", PAYLOAD):
            with self.subTest(size=len(payload)):
                filename = self._write(payload)
                firmware = LOAD_FIRMWARE._read_firmware(filename)
                self.assertEqual(firmware, LOAD_FIRMWARE.FirmwareInput(payload, None, None))

    def test_read_truncated_signed_firmware_is_rejected(self) -> None:
        filename = self._write(MAGIC + b"truncated")
        with self.assertRaises(LOAD_FIRMWARE.FirmwareInputError):
            LOAD_FIRMWARE._read_firmware(filename)

    def test_read_firmware_rejects_empty_and_oversized_payloads(self) -> None:
        for data in (
            b"",
            MAGIC + SIGDATA,
            MAGIC + SIGDATA + b"x" * (bootloader_module.MAX_FIRMWARE_SIZE + 1),
        ):
            with self.subTest(size=len(data)):
                filename = self._write(data)
                with self.assertRaises(LOAD_FIRMWARE.FirmwareInputError):
                    LOAD_FIRMWARE._read_firmware(filename)

    def test_main_reads_firmware_before_device_discovery(self) -> None:
        with mock.patch.object(LOAD_FIRMWARE, "_find_and_open_usb_bitbox02") as find_device:
            with contextlib.redirect_stderr(io.StringIO()):
                result = LOAD_FIRMWARE.main(["does-not-exist.bin"])
        self.assertEqual(result, 1)
        find_device.assert_not_called()


class FlashingMatrixTests(unittest.TestCase):
    """Test all supported device and firmware combinations."""

    def setUp(self) -> None:
        self.bootloader = mock.Mock()
        self.bootloader.expected_magic = MAGIC
        self.device = device_info()
        self.progress = mock.Mock()

    def test_unsigned_development_firmware(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        firmware = LOAD_FIRMWARE.FirmwareInput(PAYLOAD, None, None)
        self.assertTrue(
            LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, firmware, True, self.progress
            )
        )
        self.bootloader.flash_unsigned_firmware.assert_called_once_with(PAYLOAD, self.progress)
        self.bootloader.flash_sigdata.assert_not_called()

    def test_unsigned_production_firmware_is_allowed_with_warning(self) -> None:
        self.bootloader.is_devdevice.return_value = False
        firmware = LOAD_FIRMWARE.FirmwareInput(PAYLOAD, None, None)
        stderr = io.StringIO()
        with contextlib.redirect_stderr(stderr):
            result = LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, firmware, True, self.progress
            )
        self.assertTrue(result)
        self.assertIn("cannot boot", stderr.getvalue())
        self.bootloader.flash_unsigned_firmware.assert_called_once_with(PAYLOAD, self.progress)

    def test_signed_production_firmware(self) -> None:
        self.bootloader.is_devdevice.return_value = False
        firmware = signed_input()
        self.assertTrue(
            LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, firmware, True, self.progress
            )
        )
        self.bootloader.flash_unsigned_firmware.assert_called_once_with(PAYLOAD, self.progress)
        self.bootloader.flash_sigdata.assert_called_once_with(SIGDATA)

    def test_mismatched_signed_production_firmware_is_attempted(self) -> None:
        self.bootloader.is_devdevice.return_value = False
        stderr = io.StringIO()
        with contextlib.redirect_stderr(stderr):
            result = LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, signed_input(OTHER_MAGIC), True, self.progress
            )
        self.assertTrue(result)
        self.assertIn("expected to fail", stderr.getvalue())
        self.bootloader.flash_unsigned_firmware.assert_called_once_with(PAYLOAD, self.progress)
        self.bootloader.flash_sigdata.assert_called_once_with(SIGDATA)

    def test_signed_development_firmware_attempts_sigdata(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        firmware = signed_input()
        events = []
        self.bootloader.flash_unsigned_firmware.side_effect = lambda *_args: events.append(
            "firmware"
        )
        self.bootloader.flash_sigdata.side_effect = lambda *_args: events.append("sigdata")
        with contextlib.redirect_stderr(io.StringIO()):
            result = LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, firmware, True, self.progress
            )
        self.assertTrue(result)
        self.assertEqual(events, ["firmware", "sigdata"])

    def test_signed_development_sigdata_error_is_warning(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        self.bootloader.flash_sigdata.side_effect = bootloader_module.BootloaderError(ord("V"))
        stderr = io.StringIO()
        with contextlib.redirect_stderr(stderr):
            result = LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, signed_input(), True, self.progress
            )
        self.assertTrue(result)
        self.assertIn("WARNING", stderr.getvalue())
        self.assertIn("downgrade", stderr.getvalue())

    def test_signed_development_payload_error_is_fatal(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        self.bootloader.flash_unsigned_firmware.side_effect = bootloader_module.BootloaderError(
            ord("W")
        )
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(
            bootloader_module.BootloaderError
        ):
            LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, signed_input(), True, self.progress
            )
        self.bootloader.flash_sigdata.assert_not_called()

    def test_signed_development_transport_error_is_warning(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        self.bootloader.flash_sigdata.side_effect = Exception("transport failed")
        stderr = io.StringIO()
        with contextlib.redirect_stderr(stderr):
            result = LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, signed_input(), True, self.progress
            )
        self.assertTrue(result)
        self.assertIn("transport failed", stderr.getvalue())

    def test_flash_can_be_declined(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        with mock.patch.object(LOAD_FIRMWARE, "_confirm_flash", return_value=False) as confirm:
            with contextlib.redirect_stderr(io.StringIO()):
                result = LOAD_FIRMWARE._flash_firmware(
                    self.bootloader,
                    self.device,
                    signed_input(),
                    False,
                    self.progress,
                )
        self.assertFalse(result)
        confirm.assert_called_once_with(True, True, self.device["product_string"])
        self.bootloader.flash_unsigned_firmware.assert_not_called()
        self.bootloader.flash_sigdata.assert_not_called()

    def test_flash_can_be_confirmed(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        with mock.patch.object(LOAD_FIRMWARE, "_confirm_flash", return_value=True) as confirm:
            with contextlib.redirect_stderr(io.StringIO()):
                result = LOAD_FIRMWARE._flash_firmware(
                    self.bootloader,
                    self.device,
                    signed_input(),
                    False,
                    self.progress,
                )
        self.assertTrue(result)
        confirm.assert_called_once_with(True, True, self.device["product_string"])
        self.bootloader.flash_unsigned_firmware.assert_called_once()
        self.bootloader.flash_sigdata.assert_called_once_with(SIGDATA)

    def test_yes_skips_confirmation(self) -> None:
        self.bootloader.is_devdevice.return_value = True
        firmware = LOAD_FIRMWARE.FirmwareInput(PAYLOAD, None, None)
        with mock.patch.object(LOAD_FIRMWARE, "_confirm_flash") as confirm:
            LOAD_FIRMWARE._flash_firmware(
                self.bootloader, self.device, firmware, True, self.progress
            )
        confirm.assert_not_called()


class LoaderUxTests(unittest.TestCase):
    """Test user-visible failures, help, rebooting, and timeouts."""

    def test_known_bootloader_errors_are_human_readable(self) -> None:
        self.assertEqual(
            set(LOAD_FIRMWARE.BOOTLOADER_ERROR_MESSAGES),
            {ord(code) for code in "ZVNMWCAELIUK"},
        )
        for code in LOAD_FIRMWARE.BOOTLOADER_ERROR_MESSAGES:
            with self.subTest(code=code):
                message = LOAD_FIRMWARE._bootloader_error_message(
                    bootloader_module.BootloaderError(code)
                )
                self.assertNotIn("code=", message)
                self.assertTrue(message)

    def test_unknown_bootloader_error_has_fallback(self) -> None:
        message = LOAD_FIRMWARE._bootloader_error_message(bootloader_module.BootloaderError(0xFE))
        self.assertIn("unknown", message)
        self.assertIn("0xfe", message)

    def test_help_documents_flashing_matrix(self) -> None:
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout), self.assertRaises(SystemExit) as raised:
            LOAD_FIRMWARE.main(["--help"])
        self.assertEqual(raised.exception.code, 0)
        help_text = stdout.getvalue()
        self.assertIn("signed firmware   + production device", help_text)
        self.assertIn("signed firmware   + development device", help_text)
        self.assertIn("unsigned firmware + development device", help_text)
        self.assertIn("unsigned firmware + production device", help_text)
        self.assertIn("--yes", help_text)
        self.assertNotIn("--unsigned", help_text)
        self.assertIn("-y, --yes", help_text)
        self.assertIn("--debug", help_text)

    def test_debug_option_is_noop_with_warning(self) -> None:
        stderr = io.StringIO()
        with mock.patch.object(LOAD_FIRMWARE, "_find_and_open_usb_bitbox02") as find_device:
            with contextlib.redirect_stderr(stderr):
                result = LOAD_FIRMWARE.main(["--debug", "does-not-exist.bin"])
        self.assertEqual(result, 1)
        self.assertIn("--debug is deprecated and has no effect", stderr.getvalue())
        find_device.assert_not_called()

    def test_confirmation_defaults_to_yes(self) -> None:
        with mock.patch("builtins.input", return_value="") as prompt:
            self.assertTrue(LOAD_FIRMWARE._confirm_flash(True, False, "BitBox02 Multi bootloader"))
        self.assertIn("signed firmware", prompt.call_args.args[0])
        self.assertIn("production device", prompt.call_args.args[0])

    def test_confirmation_rejects_eof(self) -> None:
        with mock.patch("builtins.input", side_effect=EOFError):
            self.assertFalse(LOAD_FIRMWARE._confirm_flash(False, True, "BitBox02 Multi bootloader"))

    def test_sigdata_warning_still_reboots(self) -> None:
        bootloader = mock.Mock()
        bootloader.erased.return_value = True
        bootloader.is_devdevice.return_value = True
        bootloader.expected_magic = MAGIC
        bootloader.flash_sigdata.side_effect = bootloader_module.BootloaderError(ord("V"))
        with mock.patch.object(
            LOAD_FIRMWARE, "_read_firmware", return_value=signed_input()
        ), mock.patch.object(
            LOAD_FIRMWARE,
            "_find_and_open_usb_bitbox02",
            return_value=(device_info(), mock.Mock()),
        ), mock.patch.object(
            LOAD_FIRMWARE, "Bootloader", return_value=bootloader
        ), mock.patch.object(
            LOAD_FIRMWARE.time, "sleep"
        ), contextlib.redirect_stdout(
            io.StringIO()
        ), contextlib.redirect_stderr(
            io.StringIO()
        ):
            result = LOAD_FIRMWARE.main(["-y", "firmware.signed.bin"])
        self.assertEqual(result, 0)
        bootloader.reboot.assert_called_once_with()

    def test_production_bootloader_error_is_fatal(self) -> None:
        bootloader = mock.Mock()
        bootloader.erased.return_value = True
        bootloader.is_devdevice.return_value = False
        bootloader.expected_magic = MAGIC
        bootloader.flash_sigdata.side_effect = bootloader_module.BootloaderError(ord("V"))
        stderr = io.StringIO()
        with mock.patch.object(
            LOAD_FIRMWARE, "_read_firmware", return_value=signed_input()
        ), mock.patch.object(
            LOAD_FIRMWARE,
            "_find_and_open_usb_bitbox02",
            return_value=(device_info(), mock.Mock()),
        ), mock.patch.object(
            LOAD_FIRMWARE, "Bootloader", return_value=bootloader
        ), contextlib.redirect_stdout(
            io.StringIO()
        ), contextlib.redirect_stderr(
            stderr
        ):
            result = LOAD_FIRMWARE.main(["--yes", "firmware.signed.bin"])
        self.assertEqual(result, 1)
        self.assertIn("downgrade", stderr.getvalue())
        bootloader.reboot.assert_not_called()

    def test_wait_for_bootloader_times_out(self) -> None:
        bitbox = mock.Mock()
        bitbox.reboot.return_value = True
        with mock.patch.object(
            LOAD_FIRMWARE.devices, "get_any_bitbox02", return_value=device_info()
        ), mock.patch.object(
            LOAD_FIRMWARE.devices,
            "get_any_bitbox02_bootloader",
            side_effect=LOAD_FIRMWARE.NoneFoundException(),
        ), mock.patch.object(
            LOAD_FIRMWARE.hid, "device", return_value=mock.Mock()
        ), mock.patch.object(
            LOAD_FIRMWARE, "BitBox02", return_value=bitbox
        ), mock.patch.object(
            LOAD_FIRMWARE.time, "monotonic", side_effect=(0.0, 30.0)
        ), mock.patch.object(
            LOAD_FIRMWARE.time, "sleep"
        ), contextlib.redirect_stdout(
            io.StringIO()
        ):
            with self.assertRaises(TimeoutError):
                LOAD_FIRMWARE._get_bitbox_and_reboot(False)


if __name__ == "__main__":
    unittest.main()
