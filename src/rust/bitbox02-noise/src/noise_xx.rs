// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate alloc;
use alloc::vec::Vec;

use crate::x25519::{PrivateKey, PublicKey, Random32, X25519};
use noise_rust_crypto::{sensitive::Sensitive, ChaCha20Poly1305, Sha256};

/// Specialization of noise_protocol::HandshakeState, picking the implementations for Diffie
/// Hellman, Cipher and Hash.
pub type HandshakeState<R> = noise_protocol::HandshakeState<X25519<R>, ChaCha20Poly1305, Sha256>;

/// Common handshake hash that can be derived by both parties. The pairing code is derived from it.
pub type HandshakeHash = [u8; 32];

/// Manages a noise communication channel, including handshake and message encryption/decryption.
///
/// The required state flow is:
///
/// `Nothing --init()--> Initialized --handshake()--> Initialized --handshake() --> Ready.`
pub enum State<R: Random32> {
    /// Noise not in use yet.
    Nothing,
    /// Initialized, ready for handhshake messages.
    Initialized(HandshakeState<R>),
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

impl<R: Random32> State<R> {
    /// Can be called at any time to reset the state.
    pub fn reset(&mut self) {
        *self = State::Nothing;
    }

    /// Can be called at any time to start waiting for a new communication channel.
    ///
    /// `static_private_key` is the local static key. It can be generated using
    /// `generate_static_private_key()`.
    pub fn init(&mut self, static_private_key: Sensitive<PrivateKey>) {
        let hs = HandshakeState::new(
            noise_protocol::patterns::noise_xx(),
            false, /* is_initiator = false; the app is the initiator */
            &b"Noise_XX_25519_ChaChaPoly_SHA256"[..],
            Some(static_private_key),
            None,
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
                Ok(HandshakeResult::Response(
                    handshake_state.write_message_vec(&payload)?,
                ))
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

    enum MockRandom32 {}
    impl Random32 for MockRandom32 {
        fn mcu_32_bytes(out: &mut [u8; 32]) {
            out.copy_from_slice(b"llllllllllllllllllllllllllllllll")
        }
    }

    #[test]
    pub fn test_full() {
        use noise_protocol::DH;
        let bb02_static_key = X25519::<MockRandom32>::genkey();

        let mut host = crate::testing::make_host();
        let mut bb02 = State::<MockRandom32>::Nothing;
        bb02.init(bb02_static_key);

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
}
