// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

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
