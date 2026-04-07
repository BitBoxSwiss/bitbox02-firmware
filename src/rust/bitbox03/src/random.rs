use bitbox_hal as hal;

pub struct BitBox03Random;

impl hal::random::Random for BitBox03Random {
    fn factory_randomness(&mut self) -> &'static [u8; 32] {
        todo!()
    }

    fn mcu_32_bytes(&mut self, _out: &mut [u8; 32]) {
        todo!()
    }
}
