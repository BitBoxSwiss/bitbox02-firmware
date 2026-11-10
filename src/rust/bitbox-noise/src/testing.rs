// SPDX-License-Identifier: Apache-2.0

use crate::noise_xx::HandshakeState;
use crate::x25519::genkey;

pub struct MockRandom;

impl bitbox_hal::Random for MockRandom {
    fn factory_randomness(&mut self) -> &'static [u8; 32] {
        unreachable!()
    }

    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
        out.copy_from_slice(b"llllllllllllllllllllllllllllllll")
    }
}

pub type TestHandshakeState = HandshakeState;

pub fn make_host(random: &mut impl bitbox_hal::Random) -> TestHandshakeState {
    let host_static_key = genkey(random);
    let host_ephemeral_key = genkey(random);
    TestHandshakeState::new(
        noise_protocol::patterns::noise_xx().clone(),
        true,
        &b"Noise_XX_25519_ChaChaPoly_SHA256"[..],
        Some(host_static_key),
        Some(host_ephemeral_key),
        None,
        None,
    )
}

pub fn make_mock_host() -> TestHandshakeState {
    let mut random = MockRandom;
    make_host(&mut random)
}

/// Low-order X25519 public keys and equivalent encodings that yield an all-zero shared secret for
/// every private key. Such keys are non-contributory: the DH result is predictable without knowing
/// either party's private key.
#[cfg(test)]
pub(crate) fn non_contributory_pubkeys() -> impl Iterator<Item = [u8; 32]> {
    use hex_lit::hex;

    [
        hex!("0000000000000000000000000000000000000000000000000000000000000000"),
        hex!("0100000000000000000000000000000000000000000000000000000000000000"),
        hex!("e0eb7a7c3b41b8ae1656e3faf19fc46ada098deb9c32b1fd866205165f49b800"),
        hex!("5f9c95bca3508c24b1d0b1559c83ef5b04445cc4581c8e86d8224eddd09f1157"),
        hex!("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
        hex!("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
        hex!("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
    ]
    .into_iter()
    .flat_map(|pubkey| {
        // X25519 ignores the high bit of the public key.
        let mut alias = pubkey;
        alias[31] |= 0x80;
        [pubkey, alias]
    })
}
