// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use alloc::collections::VecDeque;

#[cfg(all(feature = "simulator-graphical", not(feature = "testing")))]
use bitbox_hal::Timer;

use bitcoin::hashes::Hash;
use hex_lit::hex;

use bitbox_hal::memory::PasswordStretchAlgo;
use bitbox_hal::securechip::{Error, Model};

pub struct FakeSecureChip {
    // Count how many security events happen. The numbers were obtained by reading the security
    // event counter slot (0xE0C5) on a real device. We can use this to assert how many events
    // were used in unit tests. The number is relevant due to Optiga's throttling mechanism.
    event_counter: u32,
    reset_keys_fail_once: bool,
    #[cfg(feature = "app-u2f")]
    u2f_counter: u32,
    mock_attestation_signature: [u8; 64],
    mock_random_values: VecDeque<[u8; 32]>,
    last_attestation_challenge: Option<[u8; 32]>,
}

impl FakeSecureChip {
    pub fn new() -> Self {
        FakeSecureChip {
            event_counter: 0,
            reset_keys_fail_once: false,
            #[cfg(feature = "app-u2f")]
            u2f_counter: 0,
            mock_attestation_signature: [0u8; 64],
            mock_random_values: VecDeque::new(),
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

    pub fn mock_random(&mut self, random: [u8; 32]) {
        self.mock_random_values.push_back(random);
    }

    pub fn last_attestation_challenge(&self) -> Option<[u8; 32]> {
        self.last_attestation_challenge
    }
}

impl bitbox_hal::SecureChip for FakeSecureChip {
    fn random(&mut self) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, Error> {
        Ok(Box::new(zeroize::Zeroizing::new(
            self.mock_random_values.pop_front().unwrap_or([0u8; 32]),
        )))
    }

    async fn init_new_password(
        &mut self,
        _memory: &mut impl bitbox_hal::Memory,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, Error> {
        self.event_counter += 3;

        let key: &'static [u8] = match password_stretch_algo {
            PasswordStretchAlgo::V0 => b"unit-test-v0",
            PasswordStretchAlgo::V1 => b"unit-test",
        };
        use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
        let mut engine = HmacEngine::<sha256::Hash>::new(key);
        engine.input(password.as_bytes());
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);
        Ok(Box::new(zeroize::Zeroizing::new(
            hmac_result.to_byte_array(),
        )))
    }

    async fn stretch_password(
        &mut self,
        _memory: &mut impl bitbox_hal::Memory,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, Error> {
        self.event_counter += match password_stretch_algo {
            PasswordStretchAlgo::V0 => 5,
            PasswordStretchAlgo::V1 => 4,
        };

        let key: &'static [u8] = match password_stretch_algo {
            PasswordStretchAlgo::V0 => b"unit-test-v0",
            PasswordStretchAlgo::V1 => b"unit-test",
        };

        use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
        let mut engine = HmacEngine::<sha256::Hash>::new(key);
        engine.input(password.as_bytes());
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);
        Ok(Box::new(zeroize::Zeroizing::new(
            hmac_result.to_byte_array(),
        )))
    }

    async fn kdf(&mut self, msg: &[u8; 32]) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, Error> {
        self.event_counter += 1;

        use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
        let mut engine = HmacEngine::<sha256::Hash>::new(&hex!(
            "d2e1e6b18b6c6b08433edbc1d168c1a0043774a4221877e79ed56684be5ac01b"
        ));
        engine.input(msg);
        let hmac_result: Hmac<sha256::Hash> = Hmac::from_engine(engine);

        // Keep KDF completion visibly delayed on the host so unlock animation start/play can be
        // manually tested in the graphical simulators.
        #[cfg(all(feature = "simulator-graphical", not(feature = "testing")))]
        crate::timer::HostTimer::delay_for(core::time::Duration::from_millis(1000)).await;

        Ok(Box::new(zeroize::Zeroizing::new(
            hmac_result.to_byte_array(),
        )))
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

    async fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        Ok(1)
    }

    fn model(&mut self) -> Result<Model, ()> {
        Ok(Model::Atecc608B)
    }

    async fn reset_keys(&mut self, _memory: &mut impl bitbox_hal::Memory) -> Result<(), ()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_hal::SecureChip;
    use hex_lit::hex;

    #[test]
    fn test_mock_random() {
        let mut securechip = FakeSecureChip::new();
        let expected = hex!("00112233445566778899aabbccddeefffedcba98765432100123456789abcdef");
        securechip.mock_random(expected);
        let first = securechip.random().unwrap();
        let second = securechip.random().unwrap();
        assert_eq!(first.as_slice(), &expected);
        assert_eq!(second.as_slice(), &[0u8; 32]);
    }
}
