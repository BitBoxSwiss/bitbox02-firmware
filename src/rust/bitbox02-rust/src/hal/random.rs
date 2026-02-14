// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

pub trait Random {
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>>;
    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]);
}
