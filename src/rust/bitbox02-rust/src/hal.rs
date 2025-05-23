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

use alloc::string::String;
use alloc::vec::Vec;

pub trait Sd {
    fn sdcard_inserted(&mut self) -> bool;
    fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()>;
    fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()>;
    fn load_bin(&mut self, filename: &str, dir: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()>;
    fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()>;
}

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    fn ui(&mut self) -> &mut impl Ui;
    fn sd(&mut self) -> &mut impl Sd;
}

pub struct BitBox02Sd;

impl Sd for BitBox02Sd {
    #[inline(always)]
    fn sdcard_inserted(&mut self) -> bool {
        bitbox02::sd::sdcard_inserted()
    }

    #[inline(always)]
    fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()> {
        bitbox02::sd::list_subdir(subdir)
    }

    #[inline(always)]
    fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()> {
        bitbox02::sd::erase_file_in_subdir(filename, dir)
    }

    #[inline(always)]
    fn load_bin(&mut self, filename: &str, dir: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        bitbox02::sd::load_bin(filename, dir)
    }

    #[inline(always)]
    fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()> {
        bitbox02::sd::write_bin(filename, dir, data)
    }
}

pub struct BitBox02Hal {
    ui: RealWorkflows,
    sd: BitBox02Sd,
}

impl BitBox02Hal {
    pub const fn new() -> Self {
        Self {
            ui: crate::workflow::RealWorkflows,
            sd: BitBox02Sd,
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
}

#[cfg(feature = "testing")]
pub mod testing {
    use alloc::collections::BTreeMap;
    use alloc::string::String;
    use alloc::vec::Vec;

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
        fn sdcard_inserted(&mut self) -> bool {
            self.inserted.unwrap()
        }

        fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()> {
            match subdir {
                Some(key) => Ok(self
                    .files
                    .get(key)
                    .map(|files| files.keys().cloned().collect())
                    .unwrap_or_default()),
                None => Ok(self.files.keys().cloned().collect()),
            }
        }

        fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()> {
            self.files
                .get_mut(dir)
                .and_then(|files| files.remove(filename).map(|_| ()))
                .ok_or(())
        }

        fn load_bin(
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

        fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()> {
            self.files
                .entry(dir.into())
                .or_default()
                .insert(filename.into(), data.to_vec());
            Ok(())
        }
    }

    pub struct TestingHal<'a> {
        pub ui: crate::workflow::testing::TestingWorkflows<'a>,
        pub sd: TestingSd,
    }

    impl TestingHal<'_> {
        pub fn new() -> Self {
            Self {
                ui: crate::workflow::testing::TestingWorkflows::new(),
                sd: TestingSd::new(),
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::hal::Sd;

        // Quick check if our mock TestingSd implementation makes sense.
        #[test]
        fn test_sd_list_write_read_erase() {
            let mut sd = TestingSd::new();
            assert_eq!(sd.list_subdir(None), Ok(vec![]));
            assert_eq!(sd.list_subdir(Some("dir1")), Ok(vec![]));

            assert!(sd.load_bin("file1.txt", "dir1").is_err());
            assert!(sd.write_bin("file1.txt", "dir1", b"data").is_ok());
            assert_eq!(sd.list_subdir(None), Ok(vec!["dir1".into()]));
            assert_eq!(sd.list_subdir(Some("dir1")), Ok(vec!["file1.txt".into()]));
            assert_eq!(
                sd.load_bin("file1.txt", "dir1").unwrap().as_slice(),
                b"data"
            );
            assert!(sd.write_bin("file1.txt", "dir1", b"replaced data").is_ok());
            assert_eq!(
                sd.load_bin("file1.txt", "dir1").unwrap().as_slice(),
                b"replaced data"
            );
            assert!(sd.erase_file_in_subdir("doesnt-exist.txt", "dir1").is_err());
            assert!(sd.erase_file_in_subdir("file1.txt", "dir1").is_ok());
            assert_eq!(sd.list_subdir(Some("dir1")), Ok(vec![]));
        }
    }
}
