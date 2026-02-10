// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use bitcoin::hashes::Hash;
use hex_lit::hex;

pub struct TestingSecureChip {
    // Count how man security events happen. The numbers were obtained by reading the security
    // event counter slot (0xE0C5) on a real device. We can use this to assert how many events
    // were used in unit tests. The number is relevant due to Optiga's throttling mechanism.
    event_counter: u32,
    reset_keys_fail_once: bool,
    #[cfg(feature = "app-u2f")]
    u2f_counter: u32,
    mock_attestation_signature: [u8; 64],
    last_attestation_challenge: Option<[u8; 32]>,
}

impl TestingSecureChip {
    pub fn new() -> Self {
        TestingSecureChip {
            event_counter: 0,
            reset_keys_fail_once: false,
            #[cfg(feature = "app-u2f")]
            u2f_counter: 0,
            mock_attestation_signature: [0u8; 64],
            last_attestation_challenge: None,
        }
    }

    /// Resets the event counter.
    pub fn event_counter_reset(&mut self) {
        self.event_counter = 0;
    }

    /// Retrieves the event counter.
    pub fn get_event_counter(&self) -> u32 {
        self.event_counter
    }

    /// Make the next `reset_keys()` call return an error once. Subsequent calls succeed.
    pub fn mock_reset_keys_fails(&mut self) {
        self.reset_keys_fail_once = true;
    }

    #[cfg(feature = "app-u2f")]
    pub fn get_u2f_counter(&self) -> u32 {
        self.u2f_counter
    }

    pub fn set_mock_attestation_signature(&mut self, sig: &[u8; 64]) {
        self.mock_attestation_signature = *sig;
    }

    pub fn last_attestation_challenge(&self) -> Option<[u8; 32]> {
        self.last_attestation_challenge
    }
}

impl crate::hal::SecureChip for TestingSecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        self.event_counter += 3;

        let key: &'static [u8] = match password_stretch_algo {
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 => {
                b"unit-test-v0"
            }
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 => b"unit-test",
        };
        use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
        let mut engine = HmacEngine::<sha256::Hash>::new(key);
        engine.input(password.as_bytes());
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);
        Ok(zeroize::Zeroizing::new(
            hmac_result.to_byte_array().to_vec(),
        ))
    }

    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        self.event_counter += match password_stretch_algo {
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 => 5,
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 => 4,
        };

        let key: &'static [u8] = match password_stretch_algo {
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V0 => {
                b"unit-test-v0"
            }
            bitbox02::memory::PasswordStretchAlgo::MEMORY_PASSWORD_STRETCH_ALGO_V1 => b"unit-test",
        };

        use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
        let mut engine = HmacEngine::<sha256::Hash>::new(key);
        engine.input(password.as_bytes());
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);
        Ok(zeroize::Zeroizing::new(
            hmac_result.to_byte_array().to_vec(),
        ))
    }

    fn kdf(
        &mut self,
        msg: &[u8],
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        self.event_counter += 1;

        use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
        let mut engine = HmacEngine::<sha256::Hash>::new(&hex!(
            "d2e1e6b18b6c6b08433edbc1d168c1a0043774a4221877e79ed56684be5ac01b"
        ));
        engine.input(msg);
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);
        Ok(zeroize::Zeroizing::new(
            hmac_result.to_byte_array().to_vec(),
        ))
    }

    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        self.event_counter += 1;
        self.last_attestation_challenge = Some(*challenge);
        *signature = self.mock_attestation_signature;
        Ok(())
    }

    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        Ok(1)
    }

    fn model(&mut self) -> Result<bitbox02::securechip::Model, ()> {
        Ok(bitbox02::securechip::Model::ATECC_ATECC608B)
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        if self.reset_keys_fail_once {
            self.reset_keys_fail_once = false;
            Err(())
        } else {
            self.event_counter += 3;
            Ok(())
        }
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
        self.u2f_counter = counter;
        Ok(())
    }
}
