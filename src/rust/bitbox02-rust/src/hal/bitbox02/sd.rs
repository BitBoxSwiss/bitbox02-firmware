// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use alloc::vec::Vec;

use futures_lite::future::yield_now;

use crate::hal::Sd;

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
