use bitbox_hal as hal;

pub struct BitBox03SecureChip;

impl hal::securechip::SecureChip for BitBox03SecureChip {
    fn random(
        &mut self,
    ) -> Result<alloc::boxed::Box<zeroize::Zeroizing<[u8; 32]>>, bitbox_hal::securechip::Error>
    {
        todo!()
    }

    fn init_new_password(
        &mut self,
        _password: &str,
        _password_stretch_algo: bitbox_hal::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<alloc::vec::Vec<u8>>, bitbox_hal::securechip::Error> {
        todo!()
    }

    fn stretch_password(
        &mut self,
        _password: &str,
        _password_stretch_algo: bitbox_hal::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<alloc::vec::Vec<u8>>, bitbox_hal::securechip::Error> {
        todo!()
    }

    fn kdf(
        &mut self,
        _msg: &[u8],
    ) -> Result<zeroize::Zeroizing<alloc::vec::Vec<u8>>, bitbox_hal::securechip::Error> {
        todo!()
    }

    fn attestation_sign(
        &mut self,
        _challenge: &[u8; 32],
        _signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        todo!()
    }

    async fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        todo!()
    }

    fn model(&mut self) -> Result<bitbox_hal::securechip::Model, ()> {
        todo!()
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        todo!()
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, _counter: u32) -> Result<(), ()> {
        todo!()
    }
}
