use bitbox_hal as hal;

pub struct BitBox03Random;

#[cfg(not(target_arch = "arm"))]
const FACTORY_RANDOMNESS: [u8; 32] = [0u8; 32];

impl hal::random::Random for BitBox03Random {
    fn factory_randomness(&mut self) -> &'static [u8; 32] {
        #[cfg(target_arch = "arm")]
        {
            bitbox_platform_stm32u5::otp::randomness()
        }

        #[cfg(not(target_arch = "arm"))]
        {
            &FACTORY_RANDOMNESS
        }
    }

    fn mcu_32_bytes(&mut self, _out: &mut [u8; 32]) {
        todo!()
    }
}
