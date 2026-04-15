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
