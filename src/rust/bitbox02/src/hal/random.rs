// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::Random;

pub struct BitBox02Random;

impl Random for BitBox02Random {
    // C simulator still uses this.
    #[cfg(any(feature = "c-unit-testing", feature = "simulator-graphical"))]
    fn factory_randomness(&mut self) -> &'static [u8; 32] {
        &[0; 32]
    }

    #[cfg(not(any(feature = "c-unit-testing", feature = "simulator-graphical")))]
    #[inline(always)]
    fn factory_randomness(&mut self) -> &'static [u8; 32] {
        let addr =
            bitbox02_sys::BITBOX02_FLASH_BOOT_START + bitbox02_sys::BITBOX02_FLASH_BOOT_LEN - 32;
        unsafe { &*(addr as *const [u8; 32]) }
    }

    #[inline(always)]
    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
        crate::random::mcu_32_bytes(out);
    }
}
