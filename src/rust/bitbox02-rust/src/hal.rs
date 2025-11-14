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
}

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    fn ui(&mut self) -> &mut impl Ui;
    fn sd(&mut self) -> &mut impl Sd;
    fn random(&mut self) -> &mut impl Random;
    fn securechip(&mut self) -> &mut impl SecureChip;
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
}

pub struct BitBox02Hal {
    ui: RealWorkflows,
    sd: BitBox02Sd,
    random: BitBox02Random,
    securechip: BitBox02SecureChip,
}

impl BitBox02Hal {
    pub const fn new() -> Self {
        Self {
            ui: crate::workflow::RealWorkflows,
            sd: BitBox02Sd,
            random: BitBox02Random,
            securechip: BitBox02SecureChip,
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
}

#[cfg(feature = "testing")]
pub mod testing {
    use alloc::boxed::Box;
    use alloc::collections::{BTreeMap, VecDeque};
    use alloc::string::String;
    use alloc::vec::Vec;

    use bitcoin::hashes::{Hash, sha256};

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
        // Count how man seceurity events happen. The numbers were obtained by reading the security
        // event counter slot (0xE0C5) on a real device. We can use this to assert how many events
        // were used in unit tests. The number is relevant due to Optiga's throttling mechanism.
        event_counter: u32,
    }

    impl TestingSecureChip {
        pub fn new() -> Self {
            TestingSecureChip { event_counter: 0 }
        }

        /// Resets the event counter.
        pub fn event_counter_reset(&mut self) {
            self.event_counter = 0;
            // TODO: remove once all unit tests use the SecureChip HAL.
            bitbox02::securechip::fake_event_counter_reset()
        }

        /// Retrieves the event counter.
        pub fn get_event_counter(&self) -> u32 {
            // TODO: remove fake_event_counter() once all unit tests use the SecureChip HAL.
            bitbox02::securechip::fake_event_counter() + self.event_counter
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
            _challenge: &[u8; 32],
            _signature: &mut [u8; 64],
        ) -> Result<(), ()> {
            self.event_counter += 1;
            todo!()
        }
    }

    pub struct TestingHal<'a> {
        pub ui: crate::workflow::testing::TestingWorkflows<'a>,
        pub sd: TestingSd,
        pub random: TestingRandom,
        pub securechip: TestingSecureChip,
    }

    impl TestingHal<'_> {
        pub fn new() -> Self {
            Self {
                ui: crate::workflow::testing::TestingWorkflows::new(),
                sd: TestingSd::new(),
                random: TestingRandom::new(),
                securechip: TestingSecureChip::new(),
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
