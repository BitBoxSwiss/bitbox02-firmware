// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

use crate::hal::Random;

pub struct BitBox02Random;

impl Random for BitBox02Random {
    #[inline(always)]
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>> {
        bitbox02::random::random_32_bytes()
    }
}
