// SPDX-License-Identifier: Apache-2.0

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

impl crate::hal::Sd for TestingSd {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::Sd;
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
}
