// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::workflow::RealWorkflows;
pub use crate::workflow::Workflows as Ui;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use futures_lite::future::yield_now;

#[allow(async_fn_in_trait)]
pub trait Sd {
    async fn sdcard_inserted(&mut self) -> bool;
    async fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()>;
    async fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()>;
    async fn load_bin(
        &mut self,
        filename: &str,
        dir: &str,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, ()>;
    async fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()>;
}

pub trait Random {
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>>;
}

pub trait SecureChip {
    fn init_new_password(&mut self, password: &str) -> Result<(), bitbox02::securechip::Error>;
    fn stretch_password(
        &mut self,
        password: &str,
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

pub trait Memory {
    fn get_securechip_type(&mut self) -> Result<bitbox02::memory::SecurechipType, ()>;
    fn get_platform(&mut self) -> Result<bitbox02::memory::Platform, ()>;
    fn get_device_name(&mut self) -> String;
    fn set_device_name(&mut self, name: &str) -> Result<(), bitbox02::memory::Error>;
    fn is_mnemonic_passphrase_enabled(&mut self) -> bool;
    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()>;
    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()>;
    fn get_seed_birthdate(&mut self) -> u32;
    fn is_seeded(&mut self) -> bool;
    fn is_initialized(&mut self) -> bool;
    fn set_initialized(&mut self) -> Result<(), ()>;
    fn get_encrypted_seed_and_hmac(&mut self) -> Result<alloc::vec::Vec<u8>, ()>;
    fn set_encrypted_seed_and_hmac(&mut self, data: &[u8]) -> Result<(), ()>;
    fn reset_hww(&mut self) -> Result<(), ()>;
    fn get_unlock_attempts(&mut self) -> u8;
    fn increment_unlock_attempts(&mut self);
    fn reset_unlock_attempts(&mut self);
    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()>;
    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()>;
    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32];
}

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    fn ui(&mut self) -> &mut impl Ui;
    fn sd(&mut self) -> &mut impl Sd;
    fn random(&mut self) -> &mut impl Random;
    fn securechip(&mut self) -> &mut impl SecureChip;
    fn memory(&mut self) -> &mut impl Memory;
}

pub struct BitBox02Sd;

impl Sd for BitBox02Sd {
    #[inline(always)]
    async fn sdcard_inserted(&mut self) -> bool {
        let result = bitbox02::sd::sdcard_inserted();
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()> {
        let result = bitbox02::sd::list_subdir(subdir);
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()> {
        let result = bitbox02::sd::erase_file_in_subdir(filename, dir);
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn load_bin(
        &mut self,
        filename: &str,
        dir: &str,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        let result = bitbox02::sd::load_bin(filename, dir);
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()> {
        let result = bitbox02::sd::write_bin(filename, dir, data);
        yield_now().await;
        result
    }
}

pub struct BitBox02Random;

impl Random for BitBox02Random {
    #[inline(always)]
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>> {
        bitbox02::random::random_32_bytes()
    }
}

pub struct BitBox02SecureChip;

impl SecureChip for BitBox02SecureChip {
    fn init_new_password(&mut self, password: &str) -> Result<(), bitbox02::securechip::Error> {
        bitbox02::securechip::init_new_password(password)
    }

    fn stretch_password(
        &mut self,
        password: &str,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::stretch_password(password)
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

pub struct BitBox02Memory;

impl Memory for BitBox02Memory {
    fn get_securechip_type(&mut self) -> Result<bitbox02::memory::SecurechipType, ()> {
        bitbox02::memory::get_securechip_type()
    }

    fn get_platform(&mut self) -> Result<bitbox02::memory::Platform, ()> {
        bitbox02::memory::get_platform()
    }

    fn get_device_name(&mut self) -> String {
        bitbox02::memory::get_device_name()
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), bitbox02::memory::Error> {
        bitbox02::memory::set_device_name(name)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        bitbox02::memory::is_mnemonic_passphrase_enabled()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        bitbox02::memory::set_mnemonic_passphrase_enabled(enabled)
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        bitbox02::memory::set_seed_birthdate(timestamp)
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        bitbox02::memory::get_seed_birthdate()
    }

    fn is_seeded(&mut self) -> bool {
        bitbox02::memory::is_seeded()
    }

    fn is_initialized(&mut self) -> bool {
        bitbox02::memory::is_initialized()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        bitbox02::memory::set_initialized()
    }

    fn get_encrypted_seed_and_hmac(&mut self) -> Result<alloc::vec::Vec<u8>, ()> {
        bitbox02::memory::get_encrypted_seed_and_hmac()
    }

    fn set_encrypted_seed_and_hmac(&mut self, data: &[u8]) -> Result<(), ()> {
        bitbox02::memory::set_encrypted_seed_and_hmac(data)
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        bitbox02::memory::reset_hww()
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        bitbox02::memory::smarteeprom_get_unlock_attempts()
    }
    fn increment_unlock_attempts(&mut self) {
        bitbox02::memory::smarteeprom_increment_unlock_attempts()
    }
    fn reset_unlock_attempts(&mut self) {
        bitbox02::memory::smarteeprom_reset_unlock_attempts()
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        bitbox02::memory::get_salt_root()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        bitbox02::memory::get_attestation_pubkey_and_certificate(
            pubkey_out,
            certificate_out,
            root_pubkey_identifier_out,
        )
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        bitbox02::memory::get_attestation_bootloader_hash()
    }
}

pub struct BitBox02Hal {
    ui: RealWorkflows,
    sd: BitBox02Sd,
    random: BitBox02Random,
    securechip: BitBox02SecureChip,
    memory: BitBox02Memory,
}

impl BitBox02Hal {
    pub const fn new() -> Self {
        Self {
            ui: crate::workflow::RealWorkflows,
            sd: BitBox02Sd,
            random: BitBox02Random,
            securechip: BitBox02SecureChip,
            memory: BitBox02Memory,
        }
    }
}

impl Hal for BitBox02Hal {
    fn ui(&mut self) -> &mut impl Ui {
        &mut self.ui
    }
    fn sd(&mut self) -> &mut impl Sd {
        &mut self.sd
    }
    fn random(&mut self) -> &mut impl Random {
        &mut self.random
    }
    fn securechip(&mut self) -> &mut impl SecureChip {
        &mut self.securechip
    }
    fn memory(&mut self) -> &mut impl Memory {
        &mut self.memory
    }
}

#[cfg(feature = "testing")]
pub mod testing {
    use alloc::boxed::Box;
    use alloc::collections::{BTreeMap, VecDeque};
    use alloc::string::String;
    use alloc::vec::Vec;

    use bitcoin::hashes::{Hash, sha256};

    use bitbox02::memory::SecurechipType;
    use hex_lit::hex;

    pub struct TestingRandom {
        mock_next_values: VecDeque<[u8; 32]>,
        counter: u32,
    }

    impl TestingRandom {
        pub fn new() -> Self {
            Self {
                mock_next_values: VecDeque::new(),
                counter: 0,
            }
        }

        pub fn mock_next(&mut self, value: [u8; 32]) {
            self.mock_next_values.push_back(value)
        }
    }

    impl super::Random for TestingRandom {
        fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>> {
            self.counter += 1;
            let value = if let Some(value) = self.mock_next_values.pop_front() {
                value
            } else {
                let hash = sha256::Hash::hash(&self.counter.to_be_bytes());
                hash.to_byte_array()
            };
            Box::new(zeroize::Zeroizing::new(value))
        }
    }

    pub struct TestingSd {
        pub inserted: Option<bool>,
        files: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
    }

    impl TestingSd {
        pub fn new() -> Self {
            Self {
                inserted: None,
                files: BTreeMap::new(),
            }
        }
    }

    impl super::Sd for TestingSd {
        async fn sdcard_inserted(&mut self) -> bool {
            self.inserted.unwrap()
        }

        async fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()> {
            match subdir {
                Some(key) => Ok(self
                    .files
                    .get(key)
                    .map(|files| files.keys().cloned().collect())
                    .unwrap_or_default()),
                None => Ok(self.files.keys().cloned().collect()),
            }
        }

        async fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()> {
            self.files
                .get_mut(dir)
                .and_then(|files| files.remove(filename).map(|_| ()))
                .ok_or(())
        }

        async fn load_bin(
            &mut self,
            filename: &str,
            dir: &str,
        ) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
            self.files
                .get(dir)
                .and_then(|files| files.get(filename))
                .map(|data| zeroize::Zeroizing::new(data.clone()))
                .ok_or(())
        }

        async fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()> {
            self.files
                .entry(dir.into())
                .or_default()
                .insert(filename.into(), data.to_vec());
            Ok(())
        }
    }

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

    pub struct TestingMemory {
        securechip_type: SecurechipType,
        platform: bitbox02::memory::Platform,
        initialized: bool,
        is_seeded: bool,
        mnemonic_passphrase_enabled: bool,
        seed_birthdate: u32,
        encrypted_seed_and_hmac: Option<Vec<u8>>,
        device_name: Option<String>,
        unlock_attempts: u8,
        salt_root: [u8; 32],
        attestation_device_pubkey: Option<[u8; 64]>,
        attestation_certificate: Option<[u8; 64]>,
        attestation_root_pubkey_identifier: Option<[u8; 32]>,
        attestation_bootloader_hash: [u8; 32],
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

    impl super::SecureChip for TestingSecureChip {
        fn init_new_password(
            &mut self,
            _password: &str,
        ) -> Result<(), bitbox02::securechip::Error> {
            self.event_counter += 1;
            Ok(())
        }

        fn stretch_password(
            &mut self,
            password: &str,
        ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
            self.event_counter += 5;

            use bitcoin::hashes::{HashEngine, Hmac, HmacEngine, sha256};
            let mut engine = HmacEngine::<sha256::Hash>::new(b"unit-test");
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
                self.event_counter += 1;
                Ok(())
            }
        }

        #[cfg(feature = "app-u2f")]
        fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
            self.u2f_counter = counter;
            Ok(())
        }
    }

    impl TestingMemory {
        pub fn new() -> Self {
            Self {
                securechip_type: SecurechipType::Atecc,
                platform: bitbox02::memory::Platform::BitBox02,
                initialized: false,
                is_seeded: false,
                mnemonic_passphrase_enabled: false,
                seed_birthdate: 0,
                encrypted_seed_and_hmac: None,
                device_name: None,
                unlock_attempts: 0,
                salt_root: *b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                attestation_device_pubkey: None,
                attestation_certificate: None,
                attestation_root_pubkey_identifier: None,
                attestation_bootloader_hash: [0; 32],
            }
        }

        pub fn set_securechip_type(&mut self, securechip_type: SecurechipType) {
            self.securechip_type = securechip_type;
        }

        pub fn set_platform(&mut self, platform: bitbox02::memory::Platform) {
            self.platform = platform;
        }

        pub fn set_unlock_attempts_for_testing(&mut self, attempts: u8) {
            self.unlock_attempts = attempts;
        }

        pub fn set_salt_root(&mut self, salt_root: &[u8; 32]) {
            self.salt_root = *salt_root;
        }

        pub fn set_attestation_certificate(
            &mut self,
            pubkey: &[u8; 64],
            certificate: &[u8; 64],
            root_pubkey_identifier: &[u8; 32],
        ) {
            self.attestation_device_pubkey = Some(*pubkey);
            self.attestation_certificate = Some(*certificate);
            self.attestation_root_pubkey_identifier = Some(*root_pubkey_identifier);
        }

        pub fn set_attestation_bootloader_hash(&mut self, hash: &[u8; 32]) {
            self.attestation_bootloader_hash = *hash;
        }
    }

    impl super::Memory for TestingMemory {
        fn get_securechip_type(&mut self) -> Result<SecurechipType, ()> {
            Ok(self.securechip_type)
        }

        fn get_platform(&mut self) -> Result<bitbox02::memory::Platform, ()> {
            Ok(self.platform)
        }

        fn get_device_name(&mut self) -> String {
            self.device_name
                .clone()
                .unwrap_or_else(|| "My BitBox".into())
        }

        fn set_device_name(&mut self, name: &str) -> Result<(), bitbox02::memory::Error> {
            self.device_name = Some(name.into());
            Ok(())
        }

        fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
            self.mnemonic_passphrase_enabled
        }

        fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
            self.mnemonic_passphrase_enabled = enabled;
            Ok(())
        }

        fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
            self.seed_birthdate = timestamp;
            Ok(())
        }

        fn get_seed_birthdate(&mut self) -> u32 {
            self.seed_birthdate
        }

        fn is_seeded(&mut self) -> bool {
            self.is_seeded
        }

        fn is_initialized(&mut self) -> bool {
            self.initialized
        }

        fn set_initialized(&mut self) -> Result<(), ()> {
            self.initialized = true;
            Ok(())
        }

        fn get_encrypted_seed_and_hmac(&mut self) -> Result<alloc::vec::Vec<u8>, ()> {
            self.encrypted_seed_and_hmac.clone().ok_or(())
        }

        fn set_encrypted_seed_and_hmac(&mut self, data: &[u8]) -> Result<(), ()> {
            // 96 is the max space allocated in BitBox02's memory for this.
            if data.len() > 96 {
                return Err(());
            }
            self.encrypted_seed_and_hmac = Some(data.to_vec());
            self.is_seeded = true;
            Ok(())
        }

        fn reset_hww(&mut self) -> Result<(), ()> {
            self.initialized = false;
            self.is_seeded = false;
            self.mnemonic_passphrase_enabled = false;
            self.seed_birthdate = 0;
            self.encrypted_seed_and_hmac = None;
            self.device_name = None;
            Ok(())
        }

        fn get_unlock_attempts(&mut self) -> u8 {
            self.unlock_attempts
        }

        fn increment_unlock_attempts(&mut self) {
            self.unlock_attempts += 1;
        }

        fn reset_unlock_attempts(&mut self) {
            self.unlock_attempts = 0;
        }

        fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
            if self.salt_root.iter().all(|&b| b == 0xff) {
                Err(())
            } else {
                Ok(zeroize::Zeroizing::new(self.salt_root.to_vec()))
            }
        }

        fn get_attestation_pubkey_and_certificate(
            &mut self,
            pubkey_out: &mut [u8; 64],
            certificate_out: &mut [u8; 64],
            root_pubkey_identifier_out: &mut [u8; 32],
        ) -> Result<(), ()> {
            match (
                self.attestation_device_pubkey,
                self.attestation_certificate,
                self.attestation_root_pubkey_identifier,
            ) {
                (Some(pubkey), Some(certificate), Some(root_id)) => {
                    *pubkey_out = pubkey;
                    *certificate_out = certificate;
                    *root_pubkey_identifier_out = root_id;
                    Ok(())
                }
                _ => Err(()),
            }
        }

        fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
            self.attestation_bootloader_hash
        }
    }

    pub struct TestingHal<'a> {
        pub ui: crate::workflow::testing::TestingWorkflows<'a>,
        pub sd: TestingSd,
        pub random: TestingRandom,
        pub securechip: TestingSecureChip,
        pub memory: TestingMemory,
    }

    impl TestingHal<'_> {
        pub fn new() -> Self {
            Self {
                ui: crate::workflow::testing::TestingWorkflows::new(),
                sd: TestingSd::new(),
                random: TestingRandom::new(),
                securechip: TestingSecureChip::new(),
                memory: TestingMemory::new(),
            }
        }
    }

    impl super::Hal for TestingHal<'_> {
        fn ui(&mut self) -> &mut impl super::Ui {
            &mut self.ui
        }
        fn sd(&mut self) -> &mut impl super::Sd {
            &mut self.sd
        }
        fn random(&mut self) -> &mut impl super::Random {
            &mut self.random
        }
        fn securechip(&mut self) -> &mut impl super::SecureChip {
            &mut self.securechip
        }
        fn memory(&mut self) -> &mut impl super::Memory {
            &mut self.memory
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::hal::{Random, Sd};
        use hex_lit::hex;

        use util::bb02_async::block_on;

        // Quick check if our mock TestingSd implementation makes sense.
        #[test]
        fn test_sd_list_write_read_erase() {
            let mut sd = TestingSd::new();
            assert_eq!(block_on(sd.list_subdir(None)), Ok(vec![]));
            assert_eq!(block_on(sd.list_subdir(Some("dir1"))), Ok(vec![]));

            assert!(block_on(sd.load_bin("file1.txt", "dir1")).is_err());
            assert!(block_on(sd.write_bin("file1.txt", "dir1", b"data")).is_ok());
            assert_eq!(block_on(sd.list_subdir(None)), Ok(vec!["dir1".into()]));
            assert_eq!(
                block_on(sd.list_subdir(Some("dir1"))),
                Ok(vec!["file1.txt".into()])
            );
            assert_eq!(
                block_on(sd.load_bin("file1.txt", "dir1"))
                    .unwrap()
                    .as_slice(),
                b"data"
            );
            assert!(block_on(sd.write_bin("file1.txt", "dir1", b"replaced data")).is_ok());
            assert_eq!(
                block_on(sd.load_bin("file1.txt", "dir1"))
                    .unwrap()
                    .as_slice(),
                b"replaced data"
            );
            assert!(block_on(sd.erase_file_in_subdir("doesnt-exist.txt", "dir1")).is_err());
            assert!(block_on(sd.erase_file_in_subdir("file1.txt", "dir1")).is_ok());
            assert_eq!(block_on(sd.list_subdir(Some("dir1"))), Ok(vec![]));
        }

        #[test]
        fn test_random() {
            let mut random = TestingRandom::new();
            let first = random.random_32_bytes();
            let second = random.random_32_bytes();
            assert_eq!(
                first.as_slice(),
                &hex!("b40711a88c7039756fb8a73827eabe2c0fe5a0346ca7e0a104adc0fc764f528d"),
            );
            assert_eq!(
                second.as_slice(),
                &hex!("433ebf5bc03dffa38536673207a21281612cef5faa9bc7a4d5b9be2fdb12cf1a"),
            );
        }
    }
}
