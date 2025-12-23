// SPDX-License-Identifier: Apache-2.0

use crate::noise_xx::HandshakeState;
use crate::x25519::{Random32, X25519};

use noise_protocol::DH;

pub enum MockRandom32 {}
impl Random32 for MockRandom32 {
    fn mcu_32_bytes(out: &mut [u8; 32]) {
        out.copy_from_slice(b"llllllllllllllllllllllllllllllll")
    }
}

pub type TestHandshakeState = HandshakeState<MockRandom32>;

pub fn make_host() -> TestHandshakeState {
    let host_static_key = X25519::<MockRandom32>::genkey();
    TestHandshakeState::new(
        noise_protocol::patterns::noise_xx().clone(),
        true,
        &b"Noise_XX_25519_ChaChaPoly_SHA256"[..],
        Some(host_static_key),
        None,
        None,
        None,
    )
}
