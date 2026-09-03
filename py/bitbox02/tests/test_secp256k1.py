# SPDX-License-Identifier: Apache-2.0

"""Tests for secp256k1 signature validation and Anti-Klepto verification."""

import unittest

from bitbox02.bitbox02.secp256k1 import (
    ECDSASignatureException,
    antiklepto_verify,
    antiklepto_verify_recoverable,
    validate_compact_ecdsa_signature,
    validate_recoverable_ecdsa_signature,
)


HOST_NONCE = bytes.fromhex("8b4c26aa2695a34bdbc34235f6c91be14b93037a063b13f7c814101359561092")
SIGNER_COMMITMENT = bytes.fromhex(
    "0236ff92fe02c08d0d04851e0ce1516104085215f05a178307de60ea53e207f971"
)
VALID_SIGNATURE = bytes.fromhex(
    "7fd66b48ffea2fe048869880bbb3a1819e262af14980e8885df1e5765750cb8f"
    "47e01eca356377870356d54853573a955076228e5044cd3dd3a049abe70d5585"
)
HIGH_S_SIGNATURE = bytes.fromhex(
    "7fd66b48ffea2fe048869880bbb3a1819e262af14980e8885df1e5765750cb8f"
    "b81fe135ca9c8878fca92ab7aca8c5696a38ba585f03d2fdec3214e0e928ebbc"
)
CURVE_ORDER = bytes.fromhex("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141")


class TestECDSASignatureValidation(unittest.TestCase):
    """Tests for compact and recoverable ECDSA signature validation."""

    def test_validate_compact_ecdsa_signature(self) -> None:
        """Accept low-S and reject malformed compact signatures."""
        validate_compact_ecdsa_signature(VALID_SIGNATURE)

        with self.assertRaisesRegex(ECDSASignatureException, "high S"):
            validate_compact_ecdsa_signature(HIGH_S_SIGNATURE)

        invalid_signatures = [
            VALID_SIGNATURE[:-1],
            VALID_SIGNATURE + b"\x00",
        ]
        for offset in (0, 32):
            invalid_signatures.append(
                VALID_SIGNATURE[:offset] + bytes(32) + VALID_SIGNATURE[offset + 32 :]
            )
            invalid_signatures.append(
                VALID_SIGNATURE[:offset] + CURVE_ORDER + VALID_SIGNATURE[offset + 32 :]
            )

        for signature in invalid_signatures:
            with self.subTest(signature=signature.hex()):
                with self.assertRaises(ECDSASignatureException):
                    validate_compact_ecdsa_signature(signature)

    def test_validate_recoverable_ecdsa_signature(self) -> None:
        """Accept valid and reject malformed recoverable signatures."""
        validate_recoverable_ecdsa_signature(VALID_SIGNATURE + b"\x00")
        validate_recoverable_ecdsa_signature(VALID_SIGNATURE + b"\x03")

        for signature in (
            VALID_SIGNATURE,
            VALID_SIGNATURE + b"\x00\x00",
            VALID_SIGNATURE + b"\x04",
        ):
            with self.subTest(signature=signature.hex()):
                with self.assertRaises(ECDSASignatureException):
                    validate_recoverable_ecdsa_signature(signature)

    def test_antiklepto_verify_rejects_high_s(self) -> None:
        """Reject a high-S signature even when its nonce contribution matches."""
        antiklepto_verify(HOST_NONCE, SIGNER_COMMITMENT, VALID_SIGNATURE)

        with self.assertRaisesRegex(ECDSASignatureException, "high S"):
            antiklepto_verify(HOST_NONCE, SIGNER_COMMITMENT, HIGH_S_SIGNATURE)

    def test_antiklepto_verify_recoverable(self) -> None:
        """Validate recovery IDs while checking the nonce contribution."""
        antiklepto_verify_recoverable(HOST_NONCE, SIGNER_COMMITMENT, VALID_SIGNATURE + b"\x00")
        antiklepto_verify_recoverable(HOST_NONCE, SIGNER_COMMITMENT, VALID_SIGNATURE + b"\x03")

        with self.assertRaisesRegex(ECDSASignatureException, "recovery ID"):
            antiklepto_verify_recoverable(HOST_NONCE, SIGNER_COMMITMENT, VALID_SIGNATURE + b"\x04")


if __name__ == "__main__":
    unittest.main()
