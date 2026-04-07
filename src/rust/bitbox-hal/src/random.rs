// SPDX-License-Identifier: Apache-2.0

pub trait Random {
    fn factory_randomness(&mut self) -> &'static [u8; 32];
    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]);
}
