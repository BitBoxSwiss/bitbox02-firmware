# SPDX-License-Identifier: Apache-2.0

"""secp256k1 util functions"""
import hashlib

import ecdsa


class ECDSANonceException(Exception):
    pass


def tagged_sha256(tag: bytes, msg: bytes) -> bytes:
    tag_hash = hashlib.sha256(tag).digest()
    return hashlib.sha256(tag_hash + tag_hash + msg).digest()


def antiklepto_host_commit(host_nonce: bytes) -> bytes:
    return tagged_sha256(b"s2c/ecdsa/data", host_nonce)


def antiklepto_verify(host_nonce: bytes, signer_commitment: bytes, signature: bytes) -> None:
    """
    Verifies that hostNonce was used to tweak the nonce during signature
    generation according to k' = k + H(signerCommitment, hostNonce) by checking that
    k'*G = signerCommitment + H(signerCommitment, hostNonce)*G.
    Throws ECDSANonceException if the verification fails.
    """
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
