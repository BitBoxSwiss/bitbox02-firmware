# SPDX-License-Identifier: Apache-2.0

"""secp256k1 util functions"""
import hashlib

import ecdsa


class ECDSANonceException(Exception):
    pass


class ECDSASignatureException(Exception):
    pass


def tagged_sha256(tag: bytes, msg: bytes) -> bytes:
    tag_hash = hashlib.sha256(tag).digest()
    return hashlib.sha256(tag_hash + tag_hash + msg).digest()


def antiklepto_host_commit(host_nonce: bytes) -> bytes:
    return tagged_sha256(b"s2c/ecdsa/data", host_nonce)


def validate_compact_ecdsa_signature(signature: bytes) -> None:
    """Validates a compact ECDSA signature encoded as r || s.

    Both scalars must be nonzero and in range, and s must use its low-S encoding.
    """
    if len(signature) != 64:
        raise ECDSASignatureException("Compact ECDSA signature must be 64 bytes")

    order = ecdsa.curves.SECP256k1.order
    sig_r = int.from_bytes(signature[:32], "big")
    sig_s = int.from_bytes(signature[32:], "big")
    if not 0 < sig_r < order or not 0 < sig_s < order:
        raise ECDSASignatureException("Invalid compact ECDSA signature scalar")
    if sig_s > order // 2:
        raise ECDSASignatureException("ECDSA signature has high S")


def validate_recoverable_ecdsa_signature(signature: bytes) -> None:
    """Validates a recoverable ECDSA signature encoded as r || s || recovery ID.

    The compact signature must be valid and the recovery ID must be in the range 0..3.
    """
    if len(signature) != 65:
        raise ECDSASignatureException("Recoverable ECDSA signature must be 65 bytes")
    validate_compact_ecdsa_signature(signature[:64])
    if signature[64] > 3:
        raise ECDSASignatureException("Invalid ECDSA recovery ID")


def antiklepto_verify(host_nonce: bytes, signer_commitment: bytes, signature: bytes) -> None:
    """
    Verifies that hostNonce was used to tweak the nonce during signature
    generation according to k' = k + H(signerCommitment, hostNonce) by checking that
    k'*G = signerCommitment + H(signerCommitment, hostNonce)*G.
    Throws ECDSASignatureException if the signature is invalid and ECDSANonceException if
    the nonce verification fails.
    """
    validate_compact_ecdsa_signature(signature)
    _antiklepto_verify_nonce(host_nonce, signer_commitment, signature)


def antiklepto_verify_recoverable(
    host_nonce: bytes, signer_commitment: bytes, signature: bytes
) -> None:
    """Validates a recoverable ECDSA signature and verifies its Anti-Klepto nonce."""
    validate_recoverable_ecdsa_signature(signature)
    _antiklepto_verify_nonce(host_nonce, signer_commitment, signature[:64])


def _antiklepto_verify_nonce(host_nonce: bytes, signer_commitment: bytes, signature: bytes) -> None:
    assert len(host_nonce) == 32
    assert len(signer_commitment) == 33, "expected compressed pubkey"
    assert len(signature) == 64
    signer_commitment_pubkey = ecdsa.VerifyingKey.from_string(
        signer_commitment, ecdsa.curves.SECP256k1
    )
    # Compute R = R1 + H(R1, host_nonce)*G. R1 is the client nonce commitment.
    tweak = tagged_sha256(b"s2c/ecdsa/point", signer_commitment + host_nonce)
    tweak_pubkey = ecdsa.SigningKey.from_string(tweak, curve=ecdsa.curves.SECP256k1).verifying_key
    tweaked_nonce = tweak_pubkey.pubkey.point + signer_commitment_pubkey.pubkey.point
    expected_sig_r = tweaked_nonce.x() % ecdsa.curves.SECP256k1.order
    sig_r = int.from_bytes(signature[:32], "big")
    if sig_r != expected_sig_r:
        raise ECDSANonceException(
            "Could not verify that the host nonce was contributed to the signature. "
            "If this happens repeatedly, the device might be attempting to "
            "leak the seed through the signature."
        )
