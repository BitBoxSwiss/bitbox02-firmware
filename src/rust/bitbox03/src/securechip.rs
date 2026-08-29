use bitbox_hal as hal;

pub struct BitBox03SecureChip;

impl hal::securechip::SecureChip for BitBox03SecureChip {
    async fn random(
        &mut self,
    ) -> Result<alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>>, bitbox_hal::securechip::Error>
    {
        todo!()
    }

    async fn init_new_password(
        &mut self,
        _random: &mut impl bitbox_hal::Random,
        _memory: &mut impl bitbox_hal::Memory,
        _password: &str,
        _password_stretch_algo: bitbox_hal::memory::PasswordStretchAlgo,
    ) -> Result<alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>>, bitbox_hal::securechip::Error>
    {
        todo!()
    }

    async fn stretch_password(
        &mut self,
        _memory: &mut impl bitbox_hal::Memory,
        _password: &str,
        _password_stretch_algo: bitbox_hal::memory::PasswordStretchAlgo,
    ) -> Result<alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>>, bitbox_hal::securechip::Error>
    {
        todo!()
    }

    async fn kdf(
        &mut self,
        _memory: &mut impl bitbox_hal::Memory,
        _msg: &[u8; 32],
    ) -> Result<alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>>, bitbox_hal::securechip::Error>
    {
        todo!()
    }

    async fn attestation_sign(
        &mut self,
        _memory: &mut impl bitbox_hal::Memory,
        _challenge: &[u8; 32],
        _signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        todo!()
    }

    async fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        todo!()
    }

    async fn model(&mut self) -> Result<bitbox_hal::securechip::Model, ()> {
        todo!()
    }

    async fn reset_keys(
        &mut self,
        _random: &mut impl bitbox_hal::Random,
        _memory: &mut impl bitbox_hal::Memory,
    ) -> Result<(), ()> {
        todo!()
    }

    #[cfg(feature = "app-u2f")]
    async fn u2f_counter_set(
        &mut self,
        _random: &mut impl bitbox_hal::Random,
        _memory: &mut impl bitbox_hal::Memory,
        _counter: u32,
    ) -> Result<(), ()> {
        todo!()
    }

    #[cfg(feature = "app-u2f")]
    async fn u2f_counter_inc(
        &mut self,
        _random: &mut impl bitbox_hal::Random,
        _memory: &mut impl bitbox_hal::Memory,
    ) -> Result<u32, ()> {
        todo!()
    }
}
