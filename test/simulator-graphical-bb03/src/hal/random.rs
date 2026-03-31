// SPDX-License-Identifier: Apache-2.0

use bitbox_hal as hal;

pub struct BitBox03Random;

impl hal::random::Random for BitBox03Random {
    fn random_32_bytes(&mut self) -> alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>> {
        todo!()
    }

    fn mcu_32_bytes(&mut self, _out: &mut [u8; 32]) {
        todo!()
    }
}
