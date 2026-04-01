// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use bitbox_hal as hal;
use rand::Rng;

pub struct BitBox03Random;

impl hal::random::Random for BitBox03Random {
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>> {
        let mut out = Box::new(zeroize::Zeroizing::new([0u8; 32]));
        self.mcu_32_bytes(out.as_mut());
        out
    }

    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
        rand::rng().fill(out);
    }
}
