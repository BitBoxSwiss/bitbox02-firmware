// Copyright 2022 Shift Crypto AG
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
