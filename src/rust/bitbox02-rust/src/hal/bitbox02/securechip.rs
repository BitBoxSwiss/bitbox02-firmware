// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::hal::SecureChip;

pub struct BitBox02SecureChip;

impl SecureChip for BitBox02SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::init_new_password(password, password_stretch_algo)
    }

    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::stretch_password(password, password_stretch_algo)
    }

    fn kdf(
        &mut self,
        msg: &[u8],
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::kdf(msg)
    }

    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        bitbox02::securechip::attestation_sign(challenge, signature)
    }

    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        bitbox02::securechip::monotonic_increments_remaining()
    }

    fn model(&mut self) -> Result<bitbox02::securechip::Model, ()> {
        bitbox02::securechip::model()
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        bitbox02::securechip::reset_keys()
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
        bitbox02::securechip::u2f_counter_set(counter)
    }
}
