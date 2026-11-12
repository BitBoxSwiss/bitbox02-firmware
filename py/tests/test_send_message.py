# SPDX-License-Identifier: Apache-2.0

# pylint: disable=missing-docstring,protected-access

import contextlib
import io
import unittest
from unittest import mock

import send_message
from bitbox02 import bitbox02
from bitbox02.communication import (
    Bitbox02Exception,
    UserAbortException,
    bitbox_api_protocol,
    devices,
)


class ManualUITestTests(unittest.TestCase):
    def setUp(self):
        self.output = io.StringIO()
        self.enterContext(contextlib.redirect_stdout(self.output))
        self.device = mock.create_autospec(bitbox02.BitBox02, instance=True)
        self.device.edition = bitbox_api_protocol.BitBox02Edition.MULTI
        self.device.device_info.return_value = {"initialized": True}
        self.connect = mock.Mock()
        self.app = send_message.ManualUITest(self.device, False, self.connect)

    def test_skip_does_not_call_device(self):
        action = mock.Mock()
        with mock.patch("builtins.input", return_value="s"):
            self.app._step("Reset", "", action)
        action.assert_not_called()
        self.assertEqual(self.app._results, [("Reset", "SKIP")])

    def test_repeat_and_explicit_operator_result(self):
        action = mock.Mock(side_effect=[UserAbortException(104, "cancelled"), None])
        with mock.patch("builtins.input", side_effect=["", "", "r", "p"]):
            self.app._step("Address", "", action)
        self.assertEqual(action.call_count, 2)
        self.assertEqual(self.app._results, [("Address", "PASS")])
        self.assertIn("Invalid input", self.output.getvalue())

    def test_request_error_can_be_recorded_and_suite_continues(self):
        failed = mock.Mock(side_effect=Bitbox02Exception(103, "error"))
        following = mock.Mock()
        with (
            mock.patch.object(
                self.app, "_cases", return_value=[("First", "", failed), ("Second", "", following)]
            ),
            mock.patch("builtins.input", side_effect=["", "f", "", "p"]),
        ):
            self.assertEqual(self.app.run(), 1)
        following.assert_called_once_with()
        self.device.close.assert_called_once_with()
        self.assertEqual(self.app._results, [("First", "FAIL"), ("Second", "PASS")])

    def test_run_closes_connection_and_reports_incomplete(self):
        for answer, expected in [("q", 130), ("s", 1)]:
            with self.subTest(answer=answer):
                self.app._results.clear()
                self.device.close.reset_mock()
                with (
                    mock.patch.object(
                        self.app, "_cases", return_value=[("Screen", "", mock.Mock())]
                    ),
                    mock.patch("builtins.input", return_value=answer),
                ):
                    self.assertEqual(self.app.run(), expected)
                self.device.close.assert_called_once_with()

    def test_uninitialized_setup_cannot_be_skipped_into_signing(self):
        self.device.device_info.return_value = {"initialized": False}
        with mock.patch("builtins.input", return_value="s"):
            self.assertEqual(self.app.run(), 1)
        self.device.btc_sign.assert_not_called()
        self.device.close.assert_called_once_with()

    def test_reconnect_retries_discovery_and_cases_use_new_device(self):
        cases = {title: action for title, _, action in self.app._cases()}
        replacement = mock.create_autospec(bitbox02.BitBox02, instance=True)
        self.connect.side_effect = [devices.NoneFoundException(), replacement]
        with mock.patch("builtins.input", return_value=""):
            self.app._reconnect()
        cases["Rename: cancellation"]()
        self.device.set_device_name.assert_not_called()
        replacement.set_device_name.assert_called_once_with("Cancelled UI test")
        self.assertEqual(self.connect.call_count, 2)

    def test_recovery_requires_reset_confirmation_and_device_acceptance(self):
        for confirmation, accepted in [("", True), ("RESET", False)]:
            with self.subTest(confirmation=confirmation):
                self.device.reset.reset_mock()
                self.device.reset.return_value = accepted
                with mock.patch("builtins.input", return_value=confirmation):
                    with self.assertRaises(ValueError):
                        self.app._recover("words")
                self.assertEqual(self.device.reset.called, confirmation == "RESET")
                self.connect.assert_not_called()
                self.device.restore_from_mnemonic.assert_not_called()

    def test_recovery_uses_fresh_connection_after_reset(self):
        replacement = mock.create_autospec(bitbox02.BitBox02, instance=True)
        replacement.device_info.return_value = {"initialized": True}
        self.connect.return_value = replacement
        self.device.reset.return_value = True
        with mock.patch("builtins.input", side_effect=["RESET", ""]):
            self.app._recover("words")
        self.device.reset.assert_called_once_with()
        self.device.restore_from_mnemonic.assert_not_called()
        replacement.restore_from_mnemonic.assert_called_once_with()

    def test_edition_and_simulator_filter_unavailable_cases(self):
        titles = [title for title, _, _ in self.app._cases()]
        self.assertIn("Ethereum: address", titles)
        self.assertIn("U2F screens", titles)
        self.device.edition = bitbox_api_protocol.BitBox02Edition.BTCONLY
        self.app._simulator = True
        titles = [title for title, _, _ in self.app._cases()]
        self.assertIn("Bitcoin: account xpub", titles)
        self.assertFalse(
            any(
                title.startswith(("Ethereum", "Cardano", "Litecoin", "Bootloader", "U2F"))
                for title in titles
            )
        )

    def test_ethereum_and_cardano_demos_do_not_prompt_for_menu_choices(self):
        self.device.eth_sign.return_value = b"signature"
        self.device.cardano_address.return_value = "test address"
        with mock.patch("builtins.input", side_effect=AssertionError("Unexpected menu prompt")):
            for index in range(1, 12):
                self.app._sign_eth_tx(str(index))
            for index in range(8):
                self.app._cardano(index)
        self.assertEqual(self.device.eth_sign.call_count, 11)
        self.assertEqual(self.device.cardano_sign_transaction.call_count, 6)

    def test_payment_request_fixture_reaches_protocol_with_invalid_signature(self):
        self.app._payment_request_ui("0.01 BTC")
        request = self.device._eth_msg_query.call_args.args[0].sign_eip1559
        self.assertEqual(request.payment_request.recipient_name, "SWAPKIT")
        self.assertEqual(request.payment_request.signature, bytes(64))
        self.assertEqual(request.payment_request.memos[1].coin_purchase_memo.amount, "0.01 BTC")
        self.assertEqual(request.gas_limit, bytes.fromhex("5208"))

    def test_cli_dispatch_and_incompatible_modes(self):
        with mock.patch.object(send_message, "connect_to_usb_bitbox", return_value=0) as usb:
            with mock.patch("sys.argv", ["send_message.py", "--test-ui"]):
                self.assertEqual(send_message.main(), 0)
            usb.assert_called_once_with(False, False, True)
        with mock.patch.object(
            send_message, "connect_to_simulator_bitbox", return_value=0
        ) as simulator:
            with mock.patch(
                "sys.argv",
                ["send_message.py", "--test-ui", "--simulator", "--simulator-port", "1234"],
            ):
                self.assertEqual(send_message.main(), 0)
            simulator.assert_called_once_with(False, 1234, True)
        with (
            mock.patch("sys.argv", ["send_message.py", "--test-ui", "--u2f"]),
            contextlib.redirect_stderr(io.StringIO()),
        ):
            with self.assertRaises(SystemExit) as exc:
                send_message.main()
            self.assertEqual(exc.exception.code, 2)


if __name__ == "__main__":
    unittest.main()
