#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

import argparse
import errno
import hashlib
import os
from pathlib import Path
import re
import signal
import socket
import struct
import subprocess
import sys
import tempfile
import threading
import time
import tty
import unittest

import ecdsa

from bitbox02.bitbox02 import bitbox02
from bitbox02.communication import PhysicalLayer, bitbox_api_protocol
from bitbox02.communication.generated import (
    antiklepto_pb2,
    btc_pb2,
    eth_pb2,
    hww_pb2,
    mnemonic_pb2,
)
from bitbox02.communication.u2fhid import u2fhid


BINARY = Path("build-build-noasan/bin/simulator").resolve()
RECORD = False
LEGACY = False


class SocketDevice(PhysicalLayer):
    def __init__(self, connection):
        self.connection = connection
        self.connection.settimeout(10)

    def write(self, data):
        self.connection.sendall(data[1:])

    def read(self, size, timeout_ms):
        result = bytearray()
        while len(result) < size:
            chunk = self.connection.recv(size - len(result))
            if not chunk:
                raise OSError("Simulator disconnected")
            result.extend(chunk)
        return bytes(result)

    def close(self):
        self.connection.close()


class Simulator:
    def __init__(self, *args, terminal=False):
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            self.port = listener.getsockname()[1]
        self.output = tempfile.TemporaryFile()
        self.errors = tempfile.TemporaryFile()
        self.reader = None
        stdout = self.output
        if terminal:
            master, slave = os.openpty()
            tty.setraw(slave)
            stdout = slave
        env = os.environ.copy()
        for name in ("DISPLAY", "WAYLAND_DISPLAY", "WAYLAND_SOCKET"):
            env.pop(name, None)
        self.process = subprocess.Popen(
            [str(BINARY), *([] if LEGACY else ["--nogui"]), "--port", str(self.port), *args],
            stdout=stdout,
            stderr=self.errors,
            env=env,
        )
        if terminal:
            os.close(slave)
            self.reader = threading.Thread(target=self.read_terminal, args=(master,))
            self.reader.start()

    def read_terminal(self, master):
        try:
            while True:
                try:
                    data = os.read(master, 4096)
                except OSError as error:
                    if error.errno == errno.EIO:
                        break
                    raise
                if not data:
                    break
                self.output.write(data)
        finally:
            os.close(master)

    def connect(self):
        deadline = time.monotonic() + 15
        while time.monotonic() < deadline:
            if self.process.poll() is not None:
                self.errors.seek(0)
                raise RuntimeError(self.errors.read().decode())
            try:
                return SocketDevice(socket.create_connection(("127.0.0.1", self.port), 1))
            except ConnectionRefusedError:
                time.sleep(0.02)
        raise TimeoutError("Simulator did not listen")

    def api(self):
        return bitbox02.BitBox02(
            transport=u2fhid.U2FHid(self.connect()),
            device_info=None,
            noise_config=bitbox_api_protocol.BitBoxNoiseConfig(),
        )

    def stop(self, signum=signal.SIGTERM):
        self.process.send_signal(signum)
        try:
            self.process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()
            raise
        if self.reader:
            self.reader.join(timeout=5)
            if self.reader.is_alive():
                raise TimeoutError("Simulator terminal did not close")
        self.output.seek(0)
        output = self.output.read().decode()
        self.errors.seek(0)
        errors = self.errors.read().decode()
        if self.process.returncode != 0:
            raise AssertionError(f"Exit {self.process.returncode}: {errors}\n{output}")
        return output.replace(f"port {self.port}", "port <port>")

    def close(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait()
        if self.reader:
            self.reader.join(timeout=5)
        self.output.close()
        self.errors.close()


def restore(api):
    request = hww_pb2.Request(
        restore_from_mnemonic=mnemonic_pb2.RestoreFromMnemonicRequest(
            timestamp=1700000000, timezone_offset=0
        )
    )
    api._msg_query(request)  # pylint: disable=protected-access


def payment_request(recipient_name="Test Merchant"):
    value = 10**18
    address = "0x" + "11" * 20
    output_hash = hashlib.sha256(
        value.to_bytes(32, "little") + bytes([len(address)]) + address.encode()
    ).digest()
    digest = hashlib.sha256(
        b"SL\x00\x24\x00"
        + bytes([len(recipient_name)])
        + recipient_name.encode()
        + b"\x00"
        + (60).to_bytes(4, "little")
        + output_hash
    ).digest()
    key = ecdsa.SigningKey.from_string(b"a" * 32, curve=ecdsa.SECP256k1)
    signature = key.sign_digest_deterministic(
        digest, sigencode=ecdsa.util.sigencode_string_canonize
    )
    return eth_pb2.ETHRequest(
        sign_eip1559=eth_pb2.ETHSignEIP1559Request(
            chain_id=1,
            keypath=[0x8000002C, 0x8000003C, 0x80000000, 0, 0],
            nonce=b"\x01",
            max_fee_per_gas=bytes.fromhex("0165a0bc00"),
            gas_limit=bytes.fromhex("5208"),
            recipient=bytes.fromhex("11" * 20),
            value=value.to_bytes(8, "big"),
            payment_request=btc_pb2.BTCPaymentRequestRequest(
                recipient_name=recipient_name, signature=signature
            ),
        )
    )


class SimulatorTests(unittest.TestCase):
    def simulator(self, *args, **kwargs):
        simulator = Simulator(*args, **kwargs)
        self.addCleanup(simulator.close)
        return simulator

    def test_version(self):
        result = subprocess.run([str(BINARY), "--version"], capture_output=True, check=True)
        self.assertRegex(
            result.stdout.decode(),
            r"\Abitbox02-multi-v\d+\.\d+\.\d+(?:[^\n]*)-simulator1\.0\.0-linux-amd64\n\Z",
        )
        self.assertEqual(result.stderr, b"")

    def test_transcript(self):
        if sys.platform != "linux":
            self.skipTest("Redirected C stdio buffering is platform-dependent")
        self.check_transcript(terminal=False)

    def test_terminal_transcript(self):
        self.check_transcript(terminal=True)

    def check_transcript(self, terminal):
        simulator = self.simulator(terminal=terminal)
        api = simulator.api()
        self.addCleanup(api.close)
        api.set_device_name("Simulator contract")
        restore(api)
        self.assertTrue(api.device_info()["initialized"])
        api.show_mnemonic()
        api.show_mnemonic()
        api._eth_msg_query(payment_request(), expected_response="sign")
        self.assertTrue(api.reset())
        self.assertFalse(api.device_info()["initialized"])
        api.set_device_name("After reset")
        output = simulator.stop()
        # The Noise pairing code is random; retain its screen and exact layout.
        output = re.sub(
            r"(TITLE: Pairing code\nBODY: )[A-Z2-7]{5} [A-Z2-7]{5}\n[A-Z2-7]{5} [A-Z2-7]{5}",
            r"\1AAAAA AAAAA\nAAAAA AAAAA",
            output,
        )
        transcript = Path(__file__).with_name(
            "transcript-tty.txt" if terminal else "transcript.txt"
        )
        if RECORD:
            transcript.write_text(output)
        self.assertEqual(output, transcript.read_text())

    def test_hangup_reconnect(self):
        simulator = self.simulator()
        api = simulator.api()
        self.addCleanup(api.close)
        api.set_device_name("Before hangup")
        simulator.process.send_signal(signal.SIGHUP)
        time.sleep(0.1)
        api.close()
        self.assertIsNone(simulator.process.poll())
        api = simulator.api()
        self.addCleanup(api.close)
        self.assertEqual(api.device_info()["name"], "Before hangup")
        simulator.stop()

    def test_shutdown(self):
        for signum in (signal.SIGINT, signal.SIGTERM):
            for connected in (False, True):
                with self.subTest(signal=signum, connected=connected):
                    simulator = self.simulator()
                    connection = simulator.connect()
                    self.addCleanup(connection.close)
                    if not connected:
                        connection.close()
                        time.sleep(0.1)
                    else:
                        connection.connection.sendall(
                            struct.pack(">IBH", 1, 0xC1, 1) + b"i" + bytes(56)
                        )
                        connection.read(64, 1000)
                        time.sleep(0.02)
                    simulator.stop(signum)
                    with socket.socket() as listener:
                        listener.bind(("0.0.0.0", simulator.port))

    def test_sign_message_and_passphrase(self):
        simulator = self.simulator()
        api = simulator.api()
        self.addCleanup(api.close)
        restore(api)
        api.enable_mnemonic_passphrase()
        api.close()
        time.sleep(0.1)
        api = simulator.api()
        self.addCleanup(api.close)
        config = btc_pb2.BTCScriptConfigWithKeypath(
            script_config=btc_pb2.BTCScriptConfig(simple_type=btc_pb2.BTCScriptConfig.P2WPKH),
            keypath=[0x80000054, 0x80000000, 0x80000000, 0, 0],
        )
        signature, _, _ = api.btc_sign_msg(btc_pb2.BTC, config, b"simulator compatibility")
        self.assertEqual(len(signature), 64)
        self.assertTrue(api.device_info()["mnemonic_passphrase_enabled"])
        simulator.stop()

    def test_preseed(self):
        if LEGACY:
            self.skipTest("Rust simulator option")
        simulator = self.simulator("--preseed")
        api = simulator.api()
        self.addCleanup(api.close)
        self.assertTrue(api.device_info()["initialized"])
        api.show_mnemonic()
        self.assertTrue(api.reset())
        restore(api)
        self.assertTrue(api.device_info()["initialized"])
        simulator.stop()

    def test_payment_request_identities(self):
        simulator = self.simulator()
        api = simulator.api()
        self.addCleanup(api.close)
        restore(api)
        response = api._eth_msg_query(payment_request(), expected_response="sign")
        self.assertEqual(len(response.sign.signature), 65)
        invalid = payment_request()
        invalid.sign_eip1559.payment_request.signature = bytes(64)
        for request in [invalid, payment_request("Unknown Merchant")]:
            with self.assertRaises(bitbox_api_protocol.Bitbox02Exception):
                api._eth_msg_query(request, expected_response="sign")
        self.assertTrue(api.device_info()["initialized"])
        simulator.stop()

    def test_hangup_during_signing(self):
        simulator = self.simulator()
        api = simulator.api()
        self.addCleanup(api.close)
        restore(api)
        config = btc_pb2.BTCScriptConfigWithKeypath(
            script_config=btc_pb2.BTCScriptConfig(simple_type=btc_pb2.BTCScriptConfig.P2WPKH),
            keypath=[0x80000054, 0x80000000, 0x80000000, 0, 0],
        )
        request = btc_pb2.BTCRequest(
            sign_message=btc_pb2.BTCSignMessageRequest(
                coin=btc_pb2.BTC,
                script_config=config,
                msg=b"Interrupted signing",
                host_nonce_commitment=antiklepto_pb2.AntiKleptoHostNonceCommitment(
                    commitment=bytes(32)
                ),
            )
        )
        api._btc_msg_query(request, expected_response="antiklepto_signer_commitment")
        simulator.process.send_signal(signal.SIGHUP)
        time.sleep(0.1)
        api.close()
        api = simulator.api()
        self.addCleanup(api.close)
        signature, _, _ = api.btc_sign_msg(btc_pb2.BTC, config, b"New session")
        self.assertEqual(len(signature), 64)
        simulator.stop()

    def test_port_in_use(self):
        simulator = self.simulator()
        device = simulator.connect()
        self.addCleanup(device.close)
        result = subprocess.run(
            [str(BINARY), *([] if LEGACY else ["--nogui"]), "--port", str(simulator.port)],
            capture_output=True,
            timeout=10,
        )
        self.assertNotEqual(result.returncode, 0)
        simulator.stop()

    def test_fragmented_reports(self):
        if LEGACY:
            self.skipTest("C simulator does not buffer partial TCP reports")
        simulator = self.simulator()
        device = simulator.connect()
        self.addCleanup(device.close)
        report = struct.pack(">IBH", 1, 0xC1, 1) + b"i" + bytes(56)
        for start, end in [(0, 1), (1, 7), (7, 31), (31, 64)]:
            device.connection.sendall(report[start:end])
            time.sleep(0.02)
        response = device.read(64, 1000)
        self.assertEqual(response[:5], report[:5])
        self.assertRegex(response[8:24], rb"v\d+\.\d+\.\d+")
        # Two HID reports in one TCP write, carrying an invalid RESET with a body.
        first = struct.pack(">IBH", 1, 0xC1, 60) + b"\x03" + bytes(56)
        second = struct.pack(">IB", 1, 0) + bytes(59)
        device.connection.sendall(first + second)
        response = device.read(64, 1000)
        self.assertEqual(response[:8], struct.pack(">IBH", 1, 0xC1, 1) + b"\x03")
        simulator.stop()

    def test_partial_frame_hangup(self):
        if LEGACY:
            self.skipTest("C simulator does not buffer partial TCP reports")
        simulator = self.simulator()
        device = simulator.connect()
        self.addCleanup(device.close)
        device.connection.sendall(b"\x00\x00\x00")
        time.sleep(0.05)
        simulator.process.send_signal(signal.SIGHUP)
        self.assertEqual(device.connection.recv(1), b"")
        api = simulator.api()
        self.addCleanup(api.close)
        api.set_device_name("New connection")
        self.assertEqual(api.device_info()["name"], "New connection")
        simulator.stop()

    def test_hangup_without_client(self):
        if LEGACY:
            self.skipTest("C accept() exits on an idle SIGHUP")
        simulator = self.simulator()
        device = simulator.connect()
        device.close()
        time.sleep(0.1)
        simulator.process.send_signal(signal.SIGHUP)
        time.sleep(0.05)
        api = simulator.api()
        self.addCleanup(api.close)
        self.assertFalse(api.device_info()["initialized"])
        simulator.stop()

    def test_single_client(self):
        simulator = self.simulator()
        first = simulator.connect()
        self.addCleanup(first.close)
        second = simulator.connect()
        self.addCleanup(second.close)
        report = struct.pack(">IBH", 1, 0xC1, 1) + b"i" + bytes(56)
        first.connection.sendall(report)
        expected = first.read(64, 1000)
        second.connection.sendall(report)
        second.connection.settimeout(0.1)
        with self.assertRaises(socket.timeout):
            second.connection.recv(64)
        first.close()
        second.connection.settimeout(10)
        self.assertEqual(second.read(64, 1000), expected)
        simulator.stop()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=BINARY)
    parser.add_argument("--record", action="store_true")
    parser.add_argument("--legacy", action="store_true")
    options, remaining = parser.parse_known_args()
    BINARY = options.binary.resolve()
    RECORD = options.record
    LEGACY = options.legacy
    unittest.main(argv=[__file__, *remaining])
