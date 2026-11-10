// SPDX-License-Identifier: Apache-2.0

extern crate alloc;
use alloc::vec::Vec;

use bitbox_hal::Random;

use crate::chacha20poly1305::ChaCha20Poly1305;
use crate::sensitive::Sensitive;
use crate::sha256::Sha256;
use crate::x25519::{PrivateKey, PublicKey, X25519, genkey};

/// Specialization of noise_protocol::HandshakeState, picking the implementations for Diffie
/// Hellman, Cipher and Hash.
/// cbindgen:ignore
pub type HandshakeState = noise_protocol::HandshakeState<X25519, ChaCha20Poly1305, Sha256>;

/// Common handshake hash that can be derived by both parties. The pairing code is derived from it.
pub type HandshakeHash = [u8; 32];

/// Manages a noise communication channel, including handshake and message encryption/decryption.
///
/// The required state flow is:
///
/// `Nothing --init()--> Initialized --handshake()--> Initialized --handshake() --> Ready.`
pub enum State {
    /// Noise not in use yet.
    Nothing,
    /// Initialized, ready for handhshake messages.
    Initialized(HandshakeState),
    /// Handshake is completed. Ready to confirm the pairing and process messages.
    Ready {
        /// Defaults to true. No encryption/decryption is possible until `set_pairing_verified()` is
        /// called.
        pairing_verification_required: bool,
        /// Fetch with `get_handshake_hash()`, used to display the pairing code to verify the
        /// pairing.
        handshake_hash: HandshakeHash,
        /// Communication partner's static public key. Can be used to remember the communication
        /// partner, so the pairing verification can be skipped the next time.
        remote_static_pubkey: PublicKey,
        /// To encrypt outgoing messages.
        send: noise_protocol::CipherState<ChaCha20Poly1305>,
        /// To decrypt incoming messages.
        receive: noise_protocol::CipherState<ChaCha20Poly1305>,
    },
}

/// See documentation of `State.handshake()`.
pub enum HandshakeResult {
    Response(Vec<u8>),
    Done,
}

/// Common error returned by all noise state functions.
#[derive(Debug)]
pub enum Error {
    /// Cannot use this until `set_pairing_verified()` has been called.
    PairingVerificationRequired,
    /// Unexpected/internal errors returned by the `noise_protocol` crate.
    Noise,
    /// This function was called at the wrong time (see `State` documentation to see in which order
    /// the functions need be called).
    WrongState,
}

impl core::convert::From<Error> for () {
    fn from(_error: Error) -> Self {}
}

impl core::convert::From<noise_protocol::Error> for Error {
    fn from(_error: noise_protocol::Error) -> Self {
        Error::Noise
    }
}

impl State {
    /// Can be called at any time to reset the state.
    pub fn reset(&mut self) {
        *self = State::Nothing;
    }

    /// Can be called at any time to start waiting for a new communication channel.
    ///
    /// `static_private_key` is the local static key. It can be generated using
    /// `genkey()` with a HAL random source and then persisted. `random` is used to generate a
    /// fresh ephemeral private key for each session.
    pub fn init(&mut self, static_private_key: Sensitive<PrivateKey>, random: &mut impl Random) {
        let ephemeral_private_key = genkey(random);
        let hs = HandshakeState::new(
            noise_protocol::patterns::noise_xx(),
            false, /* is_initiator = false; the app is the initiator */
            &b"Noise_XX_25519_ChaChaPoly_SHA256"[..],
            Some(static_private_key),
            Some(ephemeral_private_key),
            None,
            None,
        );
        *self = State::Initialized(hs);
    }

    /// In `noise_XX`, there are 3 handshake messages in total (2 by remote, 1 by us):
    ///
    /// `reqA(remote -> local); respB(local -> remote); reqC(remote -> local)`
    ///
    /// This function needs to be called twice (once for `A/B`, once for `C`).
    ///
    /// See also: [noiseexplorer.com/patterns/XX](https://noiseexplorer.com/patterns/XX/).
    ///
    /// Returns: `Response(<respB>)` (see above) for the 1st handshake message, and `Done` for the
    /// 2nd handshake message.
    pub fn handshake(&mut self, msg: &[u8]) -> Result<HandshakeResult, Error> {
        use core::convert::TryInto;
        match self {
            State::Initialized(handshake_state) => {
                let payload = match handshake_state.read_message_vec(msg) {
                    Ok(payload) => payload,
                    Err(err) => {
                        self.reset();
                        return Err(err.into());
                    }
                };

                if handshake_state.completed() {
                    let (receive, send) = handshake_state.get_ciphers();
                    let remote_static_pubkey = handshake_state.get_rs().ok_or(Error::Noise)?;
                    *self = State::Ready {
                        pairing_verification_required: true,
                        handshake_hash: handshake_state.get_hash().try_into().unwrap(),
                        remote_static_pubkey,
                        send,
                        receive,
                    };
                    return Ok(HandshakeResult::Done);
                }
                match handshake_state.write_message_vec(&payload) {
                    Ok(response) => Ok(HandshakeResult::Response(response)),
                    Err(err) => {
                        self.reset();
                        Err(err.into())
                    }
                }
            }
            _ => Err(Error::WrongState),
        }
    }

    pub fn get_handshake_hash(&self) -> Result<HandshakeHash, Error> {
        match self {
            State::Ready { handshake_hash, .. } => Ok(*handshake_hash),
            _ => Err(Error::WrongState),
        }
    }

    /// The communication partner's identity.
    pub fn remote_static_pubkey(&self) -> Result<PublicKey, Error> {
        match self {
            State::Ready {
                remote_static_pubkey,
                ..
            } => Ok(*remote_static_pubkey),
            _ => Err(Error::WrongState),
        }
    }

    /// Mark the pairing as verified, unlocking encryption and decryption.
    ///
    /// This should only be called after the user verified the pairing code (see
    /// `get_handshake_hash()`), or has done so in the past for the same remote communucation
    /// partner (see `remote_static_pubkey()`).
    pub fn set_pairing_verified(&mut self) -> Result<(), Error> {
        match self {
            State::Ready {
                pairing_verification_required,
                ..
            } => {
                *pairing_verification_required = false;
                Ok(())
            }
            _ => Err(Error::WrongState),
        }
    }

    /// Decrypt an encrypted message.
    pub fn decrypt(&mut self, msg: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            State::Ready {
                pairing_verification_required: true,
                ..
            } => Err(Error::PairingVerificationRequired),

            State::Ready {
                pairing_verification_required: false,
                receive,
                ..
            } => match receive.decrypt_vec(msg) {
                Ok(r) => Ok(r),
                Err(()) => Err(Error::Noise),
            },
            _ => Err(Error::WrongState),
        }
    }

    /// Encrypt a message. The ciphertext is appended to `out`.
    pub fn encrypt(&mut self, msg: &[u8], out: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            State::Ready {
                pairing_verification_required: true,
                ..
            } => Err(Error::PairingVerificationRequired),

            State::Ready {
                pairing_verification_required: false,
                send,
                ..
            } => {
                let start = out.len();
                // Extra 16 bytes for the aead authentication tag (MAC).
                let encrypted_len = msg.len() + 16;
                // Make space for result.
                out.resize(start + encrypted_len, 0);
                // This also adds the MAC.
                send.encrypt(msg, &mut out[start..]);
                Ok(())
            }
            _ => Err(Error::WrongState),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{MockRandom, make_mock_host, non_contributory_pubkeys};
    use hex_lit::hex;
    use noise_protocol::{DH, U8Array};

    impl HandshakeResult {
        fn response(self) -> Result<Vec<u8>, ()> {
            match self {
                HandshakeResult::Response(r) => Ok(r),
                HandshakeResult::Done => Err(()),
            }
        }
        fn done(self) -> Result<(), ()> {
            match self {
                HandshakeResult::Response(_) => Err(()),
                HandshakeResult::Done => Ok(()),
            }
        }
    }

    struct MaliciousX25519;

    impl DH for MaliciousX25519 {
        type Key = Sensitive<PrivateKey>;
        type Pubkey = PublicKey;
        type Output = [u8; 32];

        fn name() -> &'static str {
            X25519::name()
        }

        fn genkey() -> Self::Key {
            unreachable!()
        }

        // Low-order values stand in for public keys with no corresponding private key.
        fn pubkey(k: &Self::Key) -> Self::Pubkey {
            if non_contributory_pubkeys().any(|pubkey| pubkey == **k) {
                **k
            } else {
                X25519::pubkey(k)
            }
        }

        fn dh(k: &Self::Key, pk: &Self::Pubkey) -> Result<Self::Output, ()> {
            if non_contributory_pubkeys().any(|pubkey| pubkey == **k) {
                Ok([0; 32])
            } else {
                X25519::dh(k, pk)
            }
        }
    }

    #[test]
    fn test_handshake_non_contributory_ephemeral() {
        for pubkey in non_contributory_pubkeys() {
            let mut bb02 = State::Nothing;
            bb02.init(genkey(&mut MockRandom), &mut MockRandom);
            assert!(
                matches!(bb02.handshake(&pubkey), Err(Error::Noise)),
                "{pubkey:02x?}"
            );
            assert!(matches!(bb02, State::Nothing));
        }
    }

    #[test]
    fn test_handshake_non_contributory_static() {
        for pubkey in non_contributory_pubkeys() {
            let mut host =
                noise_protocol::HandshakeState::<MaliciousX25519, ChaCha20Poly1305, Sha256>::new(
                    noise_protocol::patterns::noise_xx(),
                    true,
                    b"Noise_XX_25519_ChaChaPoly_SHA256",
                    Some(Sensitive::from(pubkey.into())),
                    Some(genkey(&mut MockRandom)),
                    None,
                    None,
                );
            let mut bb02 = State::Nothing;
            bb02.init(genkey(&mut MockRandom), &mut MockRandom);

            let msg = host.write_message_vec(b"").unwrap();
            let response = bb02.handshake(&msg).unwrap().response().unwrap();
            host.read_message_vec(&response).unwrap();
            let msg = host.write_message_vec(b"").unwrap();
            assert!(host.completed());
            assert!(
                matches!(bb02.handshake(&msg), Err(Error::Noise)),
                "{pubkey:02x?}"
            );
            assert!(matches!(bb02, State::Nothing));
        }
    }

    #[test]
    pub fn test_full() {
        let mut bb02_random = MockRandom;
        let bb02_static_key = genkey(&mut bb02_random);

        let mut host = make_mock_host();
        let mut bb02 = State::Nothing;
        bb02.init(bb02_static_key, &mut bb02_random);

        let host_handshake_1 = host.write_message_vec(b"").unwrap();
        let bb02_handshake_1 = bb02
            .handshake(&host_handshake_1)
            .unwrap()
            .response()
            .unwrap();

        let host_handshake_2 = {
            let payload = host.read_message_vec(&bb02_handshake_1).unwrap();
            host.write_message_vec(&payload).unwrap()
        };
        bb02.handshake(&host_handshake_2).unwrap().done().unwrap();

        assert_eq!(
            bb02.remote_static_pubkey().unwrap(),
            X25519::pubkey(&genkey(&mut MockRandom))
        );
        assert_eq!(bb02.get_handshake_hash().unwrap(), host.get_hash());
        assert!(matches!(
            bb02.encrypt(b"message", &mut Vec::new()),
            Err(Error::PairingVerificationRequired)
        ));
        assert!(matches!(
            bb02.decrypt(&[0; 16]),
            Err(Error::PairingVerificationRequired)
        ));
        bb02.set_pairing_verified().unwrap();

        let (mut host_send, mut host_recv) = host.get_ciphers();

        let encrypted = host_send.encrypt_vec(b"message from host");
        let decrypted = bb02.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, b"message from host");

        let mut encrypted = b"prefix".to_vec();
        bb02.encrypt(b"message from bb02", &mut encrypted).unwrap();
        let (prefix, encrypted) = encrypted.split_at(b"prefix".len());
        assert_eq!(&prefix, b"prefix");
        let decrypted = host_recv.decrypt_vec(encrypted).unwrap();
        assert_eq!(decrypted, b"message from bb02");
    }

    #[test]
    fn test_handshake_reference_vector() {
        // Noise_XX_25519_ChaChaPoly_SHA256 from:
        // https://github.com/rweather/noise-c/blob/cfe25410979a87391bb9ac8d4d4bef64e9f268c6/tests/vector/noise-c-basic.txt
        let mut host = HandshakeState::new(
            noise_protocol::patterns::noise_xx(),
            true,
            b"Prologue123",
            Some(Sensitive::from_slice(&hex!(
                "e61ef9919cde45dd5f82166404bd08e38bceb5dfdfded0a34c8df7ed542214d1"
            ))),
            Some(Sensitive::from_slice(&hex!(
                "893e28b9dc6ca8d611ab664754b8ceb7bac5117349a4439a6b0569da977c464a"
            ))),
            None,
            None,
        );
        assert_eq!(
            host.write_message_vec(b"Ludwig von Mises").unwrap(),
            hex!(
                "ca35def5ae56cec33dc2036731ab14896bc4c75dbb07a61f879f8e3afa4c79444c756477696720766f6e204d69736573"
            )
        );
        assert_eq!(
            host.read_message_vec(&hex!(
                "95ebc60d2b1fa672c1f46a8aa265ef51bfe38e7ccb39ec5be34069f14480884381cbad1f276e038c48378ffce2b65285e08d6b68aaa3629a5a8639392490e5b94a6d8798832d5372f220f161d9c2df035528f8982ffe09be9b5c412f8a0db5d21351f20af1370d0bf8ef1a8c59a30e"
            ))
            .unwrap(),
            b"Murray Rothbard"
        );
        assert_eq!(
            host.write_message_vec(b"F. A. Hayek").unwrap(),
            hex!(
                "c7195ffacac1307ff99046f219750fc47693e23c3cb08b89c2af808b444850a8589981dfbd651e6ff4724a781cc2aa6158c9fea0d4ec82a286427484c5b8c8123a7a6002b1de9f9775fc97"
            )
        );
        assert!(host.completed());
        assert_eq!(
            host.get_hash(),
            hex!("852a28d2146785c54bd8334f4e460c80d7fe4fd0cc5bc0abef2a24a3c4d44d5f")
        );
        let (mut send, mut receive) = host.get_ciphers();
        assert_eq!(
            receive
                .decrypt_vec(&hex!(
                    "96763ed773f8e47bb3712f0e29b3060ffc956ffc146cee53d5e1df"
                ))
                .unwrap(),
            b"Carl Menger"
        );
        assert_eq!(
            send.encrypt_vec(b"Jean-Baptiste Say"),
            hex!("3e40f15f6f3a46ae446b253bf8b1d9ffb6ed9b174d272328ff91a7e2e5c79c07f5")
        );
        assert_eq!(
            receive
                .decrypt_vec(&hex!(
                    "eb3f3515110702e047a6c9da4478b6ead94873c11c0f2d710ddb3f09fce024b3a58502ae3f"
                ))
                .unwrap(),
            b"Eugen B\xf6hm von Bawerk"
        );
    }
}
