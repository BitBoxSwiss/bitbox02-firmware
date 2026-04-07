// SPDX-License-Identifier: Apache-2.0

use bitbox_hal as hal;
use rand::Rng;

pub struct BitBox03Random;

impl hal::random::Random for BitBox03Random {
    fn factory_randomness(&mut self) -> &'static [u8; 32] {
        &[0u8; 32]
    }

    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
        rand::rng().fill(out);
    }
}
