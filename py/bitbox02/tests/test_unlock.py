# SPDX-License-Identifier: Apache-2.0

"""Unlock protocol and callback lifetimes, without a physical device."""

# pylint: disable=protected-access,no-member

import io
import queue
import threading
import unittest
from contextlib import redirect_stdout
from unittest import mock

from bitbox02.communication import bitbox_api_protocol as protocol
from bitbox02.communication.generated import hww_pb2 as hww
from bitbox02.communication.generated import keystore_pb2 as keystore


def reply(state):
    return hww.Response(unlock=keystore.UnlockResponse(state=state))


PENDING = reply(keystore.UnlockResponse.PASSPHRASE_PENDING)
READY = reply(keystore.UnlockResponse.HOST_ENTRY_READY)
ENTERED = reply(keystore.UnlockResponse.PASSPHRASE_ENTERED)
DONE = reply(keystore.UnlockResponse.DONE)


class TestUnlock(unittest.TestCase):
    """The synchronous worker owns protocol I/O; UI clicks only enqueue a request."""

    def setUp(self):
        self.api = object.__new__(protocol.BitBoxCommonAPI)
        self.api.debug = False
        self.sent = []

    def responses(self, responses):
        sequence = iter(responses)

        def query(request, expected_response):
            self.assertEqual(expected_response, "unlock")
            self.sent.append(request)
            response = next(sequence)
            if isinstance(response, Exception):
                raise response
            return response

        self.api._msg_query = mock.Mock(side_effect=query)

    def test_device_entry_notifies_availability_once(self):
        self.responses([PENDING, PENDING, PENDING, ENTERED, DONE])
        events = []
        input_callback = mock.Mock()
        config = protocol.BitBoxConfig(
            on_host_passphrase_available=events.append,
            enter_mnemonic_passphrase=input_callback,
        )
        self.api._unlock(config)
        self.assertEqual(len(events), 2)
        self.assertTrue(callable(events[0]))
        self.assertEqual(events[1:], [None])
        input_callback.assert_not_called()
        self.assertTrue(self.sent[0].HasField("unlock"))
        self.assertTrue(
            all(not request.unlock_continue.request_host_entry for request in self.sent[1:])
        )
        sent_count = len(self.sent)
        events[0]()
        self.assertEqual(len(self.sent), sent_count)

    def test_device_entry_withdraws_before_confirmation(self):
        # Callbacks are consumed synchronously within each subtest.
        # pylint: disable=cell-var-from-loop
        for click in [False, True]:
            with self.subTest(click=click):
                events = []
                entered = mock.Mock()
                self.sent.clear()

                def available(button):
                    events.append(button)
                    if button is not None and click:
                        button()

                def query(request, expected_response):
                    self.assertEqual(expected_response, "unlock")
                    self.sent.append(request)
                    if len(self.sent) == 1:
                        return PENDING
                    if len(self.sent) == 2:
                        self.assertEqual(request.unlock_continue.request_host_entry, click)
                        return ENTERED
                    self.assertEqual(len(self.sent), 3)
                    self.assertEqual(events[1:], [None])
                    # A delayed UI event cannot switch to host entry during confirmation.
                    events[0]()
                    self.assertFalse(request.unlock_continue.request_host_entry)
                    entered.assert_not_called()
                    return DONE

                self.api._msg_query = mock.Mock(side_effect=query)
                self.api._unlock(protocol.BitBoxConfig(available, entered))
                self.assertEqual(len(self.sent), 3)
                self.assertEqual(len(events), 2)

    def test_device_confirmation_rejection_offers_fresh_button(self):
        self.responses([PENDING, ENTERED, PENDING, ENTERED, DONE])
        buttons = []
        events = []

        def available(button):
            events.append(button)
            if button is not None:
                buttons.append(button)
                if len(buttons) == 2:
                    buttons[0]()

        entered = mock.Mock()
        self.api._unlock(protocol.BitBoxConfig(available, entered))
        self.assertIsNot(buttons[0], buttons[1])
        self.assertEqual(events, [buttons[0], None, buttons[1], None])
        self.assertTrue(
            all(not request.unlock_continue.request_host_entry for request in self.sent[1:])
        )
        entered.assert_not_called()

    def test_host_input_after_consent_and_empty_is_submission(self):
        # Callbacks are consumed synchronously within each subtest.
        # pylint: disable=cell-var-from-loop
        for value in ["  exact value  ", ""]:
            with self.subTest(value=value):
                self.sent.clear()
                self.responses([PENDING, READY, DONE])
                events = []

                def available(button):
                    events.append(button)
                    if button is not None:
                        count = len(self.sent)
                        button()
                        button()  # Coalesce double clicks without I/O here.
                        self.assertEqual(len(self.sent), count)

                def enter():
                    self.assertEqual(events[-1], None)
                    self.assertTrue(self.sent[-1].unlock_continue.request_host_entry)
                    return value

                self.api._unlock(protocol.BitBoxConfig(available, enter))
                self.assertEqual(events[1:], [None])
                request = self.sent[-1].unlock_host_info
                self.assertTrue(request.HasField("passphrase"))
                self.assertEqual(request.passphrase, value)
                self.assertEqual(len(self.sent), 3)

    def test_automatic_host_entry_once_per_unlock(self):
        def enter():
            self.assertEqual(len(self.sent), 2)
            self.assertTrue(self.sent[-1].unlock_continue.request_host_entry)
            return "value"

        input_callback = mock.Mock(side_effect=enter)
        config = protocol.BitBoxConfig(enter_mnemonic_passphrase=input_callback)
        # Reusing the same config must still request host entry on the next unlock.
        for _ in range(2):
            self.sent.clear()
            input_callback.reset_mock()
            self.responses([PENDING, READY, DONE])
            with mock.patch.object(protocol.time, "sleep") as sleep:
                self.api._unlock(config)
            sleep.assert_not_called()
            input_callback.assert_called_once_with()
            self.assertEqual(
                [request.WhichOneof("request") for request in self.sent],
                ["unlock", "unlock_continue", "unlock_host_info"],
            )
            self.assertEqual(self.sent[-1].unlock_host_info.passphrase, "value")

    def test_automatic_host_entry_falls_back_to_device(self):
        for after_request, value in [
            ([PENDING], None),  # Device consent rejected.
            ([READY, PENDING], None),  # Host input cancelled.
            ([READY, PENDING], "value"),  # Passphrase confirmation rejected.
            ([READY, PENDING], "~"),  # Unsupported characters.
            ([READY, PENDING], "a" * 150),  # Passphrase too long.
        ]:
            with self.subTest(after_request=after_request, value=value):
                self.sent.clear()
                self.responses([PENDING] + after_request + [PENDING, ENTERED, DONE])
                enter = mock.Mock(return_value=value)
                with mock.patch.object(protocol.time, "sleep") as sleep:
                    self.api._unlock(protocol.BitBoxConfig(enter_mnemonic_passphrase=enter))
                self.assertEqual(sleep.call_count, 2)
                self.assertEqual(
                    [
                        request.unlock_continue.request_host_entry
                        for request in self.sent
                        if request.HasField("unlock_continue")
                    ],
                    [True, False, False, False],
                )
                if READY in after_request:
                    enter.assert_called_once_with()
                    self.assertEqual(
                        self.sent[2].unlock_host_info.HasField("passphrase"), value is not None
                    )
                    if value is not None:
                        self.assertEqual(self.sent[2].unlock_host_info.passphrase, value)
                else:
                    enter.assert_not_called()

    def test_automatic_host_entry_loses_to_device_completion(self):
        # Device completion wins the automatic request; rejecting confirmation must not
        # trigger another automatic request when device entry restarts.
        self.responses([PENDING, ENTERED, PENDING, ENTERED, DONE])
        enter = mock.Mock()
        with mock.patch.object(protocol.time, "sleep"):
            self.api._unlock(protocol.BitBoxConfig(enter_mnemonic_passphrase=enter))
        enter.assert_not_called()
        self.assertEqual(
            [request.unlock_continue.request_host_entry for request in self.sent[1:]],
            [True, False, False, False],
        )

    def test_automatic_host_entry_requires_consent_response(self):
        for responses in [[READY], [PENDING, PENDING, READY]]:
            with self.subTest(responses=responses):
                self.responses(responses)
                enter = mock.Mock()
                with mock.patch.object(protocol.time, "sleep"):
                    with self.assertRaisesRegex(Exception, "Unexpected unlock phase"):
                        self.api._unlock(protocol.BitBoxConfig(enter_mnemonic_passphrase=enter))
                enter.assert_not_called()

    def test_cancel_and_rejection_offer_fresh_button(self):
        # Callbacks are consumed synchronously within each subtest.
        # pylint: disable=cell-var-from-loop
        for after_click, value in [
            ([PENDING], None),
            ([READY, PENDING], None),
            ([READY, PENDING], "~"),  # Unsupported host input also restarts device entry.
        ]:
            with self.subTest(after_click=after_click, value=value):
                self.sent.clear()
                self.responses([PENDING] + after_click + [ENTERED, DONE])
                buttons = []
                events = []

                def available(button):
                    events.append(button)
                    if button is not None:
                        buttons.append(button)
                        if len(buttons) == 1:
                            button()
                        else:
                            buttons[0]()  # Old click must not request consent in the new phase.

                enter = mock.Mock(return_value=value)
                self.api._unlock(protocol.BitBoxConfig(available, enter))
                self.assertEqual(len(buttons), 2)
                self.assertIsNot(buttons[0], buttons[1])
                self.assertEqual(events, [buttons[0], None, buttons[1], None])
                self.assertTrue(
                    all(
                        not request.unlock_continue.request_host_entry for request in self.sent[-2:]
                    )
                )
                if READY in after_click:
                    enter.assert_called_once_with()
                    self.assertEqual(
                        self.sent[2].unlock_host_info.HasField("passphrase"), value is not None
                    )
                    if value is not None:
                        self.assertEqual(self.sent[2].unlock_host_info.passphrase, value)
                else:
                    enter.assert_not_called()

    def test_stale_button_cannot_affect_later_attempt(self):
        old = []
        self.responses([PENDING, ENTERED, DONE])
        self.api._unlock(protocol.BitBoxConfig(old.append, lambda: "unused"))
        self.sent.clear()
        self.responses([PENDING, ENTERED, DONE])
        fresh = []

        def available(button):
            fresh.append(button)
            old[0]()

        self.api._unlock(protocol.BitBoxConfig(available, lambda: "unused"))
        self.assertTrue(
            all(not request.unlock_continue.request_host_entry for request in self.sent[1:])
        )
        self.assertIsNot(old[0], fresh[0])

    def test_errors_withdraw_and_invalidate(self):
        for failure in [
            OSError("disconnected"),
            hww.Response(unlock=keystore.UnlockResponse(state=99)),
        ]:
            self.responses([PENDING, failure])
            notifications = []
            with self.assertRaises(Exception):
                self.api._unlock(protocol.BitBoxConfig(notifications.append, lambda: "unused"))
            self.assertEqual(len(notifications), 2)
            self.assertIsNone(notifications[1])
            sent_count = len(self.sent)
            notifications[0]()
            self.assertEqual(len(self.sent), sent_count)

    def test_callback_failure_invalidates_button(self):
        self.responses([PENDING])
        buttons = []

        def available(button):
            buttons.append(button)
            if button is not None:
                raise RuntimeError("notification failed")

        with self.assertRaisesRegex(RuntimeError, "notification failed"):
            self.api._unlock(protocol.BitBoxConfig(available, lambda: "unused"))
        self.assertEqual(len(buttons), 2)
        self.assertIsNone(buttons[-1])
        sent_count = len(self.sent)
        buttons[0]()
        self.assertEqual(len(self.sent), sent_count)

    def test_input_exception_stops_host_submission(self):
        self.responses([PENDING, READY])
        notifications = []

        def available(button):
            notifications.append(button)
            if button is not None:
                button()

        with self.assertRaisesRegex(RuntimeError, "input failed"):
            self.api._unlock(
                protocol.BitBoxConfig(
                    available, mock.Mock(side_effect=RuntimeError("input failed"))
                )
            )
        self.assertIsNone(notifications[-1])
        self.assertEqual(len(self.sent), 2)

    def test_immediate_done_does_not_request_passphrase(self):
        for available, enter in [
            (None, None),
            (mock.Mock(), None),
            (None, mock.Mock()),
            (mock.Mock(), mock.Mock()),
        ]:
            self.responses([DONE])
            self.api._unlock(protocol.BitBoxConfig(available, enter))
            if available is not None:
                available.assert_not_called()
            if enter is not None:
                enter.assert_not_called()

    def test_device_entry_polls_without_host_callbacks(self):
        for available in [None, mock.Mock()]:
            self.sent.clear()
            self.responses([PENDING, PENDING, PENDING, ENTERED, DONE])
            with mock.patch.object(protocol.time, "sleep") as sleep:
                self.api._unlock(protocol.BitBoxConfig(on_host_passphrase_available=available))
            self.assertEqual(sleep.call_count, 3)
            self.assertEqual(
                [request.WhichOneof("request") for request in self.sent],
                ["unlock"] + ["unlock_continue"] * 4,
            )
            self.assertTrue(
                all(not request.unlock_continue.request_host_entry for request in self.sent[1:])
            )
            if available is not None:
                available.assert_not_called()

    def test_click_from_ui_thread_wakes_worker(self):
        buttons = queue.Queue()
        events = []
        errors = []
        worker_id = []

        def query(request, expected_response):
            del expected_response
            self.assertEqual(threading.get_ident(), worker_id[0])
            events.append(request.WhichOneof("request"))
            if request.HasField("unlock"):
                return PENDING
            if request.HasField("unlock_continue"):
                return READY if request.unlock_continue.request_host_entry else PENDING
            return DONE

        self.api._msg_query = mock.Mock(side_effect=query)

        def available(button):
            self.assertEqual(threading.get_ident(), worker_id[0])
            if button is not None:
                buttons.put(button)

        def run():
            worker_id.append(threading.get_ident())
            try:
                self.api._unlock(protocol.BitBoxConfig(available, lambda: "value"))
            except Exception as exc:  # pylint: disable=broad-exception-caught
                # Relay thread assertion failures to the test runner.
                errors.append(exc)

        thread = threading.Thread(target=run, daemon=True)
        thread.start()
        button = buttons.get(timeout=2)
        button()
        thread.join(timeout=2)
        self.assertFalse(thread.is_alive())
        self.assertFalse(errors)
        self.assertEqual(events[-1], "unlock_host_info")

    def test_debug_redacts_passphrase(self):
        self.api.debug = True
        self.api._bitbox_protocol = mock.Mock()
        self.api._bitbox_protocol.encrypted_query.return_value = DONE.SerializeToString()
        request = hww.Request(
            unlock_host_info=keystore.UnlockHostInfoRequest(passphrase="never-log-this")
        )
        output = io.StringIO()
        with redirect_stdout(output):
            self.api._msg_query(request)
        self.assertNotIn("never-log-this", output.getvalue())
        self.assertIn("redacted", output.getvalue())


if __name__ == "__main__":
    unittest.main()
