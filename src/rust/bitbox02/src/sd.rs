// SPDX-License-Identifier: Apache-2.0

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use bitbox02_sys::SD_MAX_FILE_SIZE;
use util::strings::str_to_cstr_vec;

#[cfg(any(feature = "testing", feature = "simulator-graphical"))]
pub fn format() -> bool {
    unsafe { bitbox02_sys::sd_format() }
}

pub fn sdcard_inserted() -> bool {
    unsafe { bitbox02_sys::sd_card_inserted() }
}

struct SdList(bitbox02_sys::sd_list_t);

impl Drop for SdList {
    fn drop(&mut self) {
        unsafe { bitbox02_sys::sd_free_list(&mut self.0) }
    }
}

pub fn list_subdir(subdir: Option<&str>) -> Result<Vec<String>, ()> {
    let mut list = SdList(bitbox02_sys::sd_list_t {
        num_files: 0,
        files: core::ptr::null_mut(),
    });
    let c_subdir = subdir.map(|s| str_to_cstr_vec(s).unwrap());
    match unsafe {
        bitbox02_sys::sd_list_subdir(
            &mut list.0,
            match c_subdir.as_ref() {
                Some(s) => s.as_ptr().cast(),
                None => core::ptr::null(),
            },
        )
    } {
        true => (0..list.0.num_files)
            .map(|i| unsafe {
                let ptr = *list.0.files.add(i);
                util::strings::str_from_null_terminated_ptr(ptr).map(String::from)
            })
            .collect(),
        false => Err(()),
    }
}

pub fn erase_file_in_subdir(filename: &str, dir: &str) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::sd_erase_file_in_subdir(
            str_to_cstr_vec(filename).unwrap().as_ptr().cast(),
            str_to_cstr_vec(dir).unwrap().as_ptr().cast(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn load_bin(filename: &str, dir: &str) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut contents = zeroize::Zeroizing::new([0u8; SD_MAX_FILE_SIZE as _]);
    let mut contents_len: usize = 0;
    match unsafe {
        bitbox02_sys::sd_load_bin(
            str_to_cstr_vec(filename).unwrap().as_ptr().cast(),
            str_to_cstr_vec(dir).unwrap().as_ptr().cast(),
            contents.as_mut_ptr(),
            &mut contents_len,
        )
    } {
        true => Ok(zeroize::Zeroizing::new(contents[..contents_len].to_vec())),
        false => Err(()),
    }
}

pub fn write_bin(filename: &str, dir: &str, data: &[u8]) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::sd_write_bin(
            str_to_cstr_vec(filename).unwrap().as_ptr().cast(),
            str_to_cstr_vec(dir).unwrap().as_ptr().cast(),
            data.as_ptr(),
            data.len() as _,
            true,
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_write_read_erase() {
        format();

        assert_eq!(list_subdir(None), Ok(vec![]));
        assert_eq!(list_subdir(Some("dir1")), Ok(vec![]));

        assert!(load_bin("file1.txt", "dir1").is_err());
        assert!(write_bin("file1.txt", "dir1", b"data").is_ok());
        assert_eq!(list_subdir(None), Ok(vec!["dir1".into()]));
        assert_eq!(list_subdir(Some("dir1")), Ok(vec!["file1.txt".into()]));
        assert_eq!(load_bin("file1.txt", "dir1").unwrap().as_slice(), b"data");
        assert!(write_bin("file1.txt", "dir1", b"replaced data").is_ok());
        assert_eq!(
            load_bin("file1.txt", "dir1").unwrap().as_slice(),
            b"replaced data"
        );
        assert!(erase_file_in_subdir("doesnt-exist.txt", "dir1").is_err());
        assert!(erase_file_in_subdir("file1.txt", "dir1").is_ok());
        assert_eq!(list_subdir(Some("dir1")), Ok(vec![]));
    }
}
