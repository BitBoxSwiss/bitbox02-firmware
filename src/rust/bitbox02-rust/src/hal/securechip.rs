// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

pub trait SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error>;
    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error>;
    fn kdf(
        &mut self,
        msg: &[u8],
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error>;
    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()>;
    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()>;
    fn model(&mut self) -> Result<bitbox02::securechip::Model, ()>;
    fn reset_keys(&mut self) -> Result<(), ()>;
    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()>;
}
