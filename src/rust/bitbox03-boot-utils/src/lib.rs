// SPDX-License-Identifier: Apache-2.0

#![no_std]

use bitbox_boot_utils::{
    FLASH_PAGE_SIZE, IMAGE_SIGNATURE_COUNT, IMMUTABLE_PAGE_MAGIC, IMMUTABLE_PAGE_VERSION,
    P256_PUBLIC_KEY_LEN,
};
#[doc(hidden)]
#[cfg(feature = "rtt")]
pub use log;
#[doc(hidden)]
#[cfg(feature = "rtt")]
pub use rtt_target;

#[macro_export]
macro_rules! rtt_logger_init {
    () => {{
        #[cfg(feature = "rtt")]
        {
            let channels = $crate::rtt_target::rtt_init! {
                up: {
                    0: {
                        size: 1024,
                        mode: $crate::rtt_target::ChannelMode::NoBlockSkip,
                        name: "Terminal",
                        section: ".segger_rtt_buf",
                    }
                    1: {
                        size: 1024,
                        mode: $crate::rtt_target::ChannelMode::NoBlockSkip,
                        name: "API Response",
                        section: ".segger_rtt_buf",
                    }
                }
                down: {
                    0: {
                        size: 16,
                        mode: $crate::rtt_target::ChannelMode::NoBlockSkip,
                        name: "Terminal",
                        section: ".segger_rtt_buf",
                    }
                    1: {
                        size: 1024,
                        mode: $crate::rtt_target::ChannelMode::NoBlockSkip,
                        name: "API Request",
                        section: ".segger_rtt_buf",
                    }
                }
                section_cb: ".segger_rtt"
                reuse_if_initialized: true
            };
            $crate::rtt_target::set_print_channel(channels.up.0);
            $crate::rtt_target::init_logger_with_level($crate::log::LevelFilter::Trace);
        }
    }};
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ImmutablePage {
    pub magic: u32,
    pub version: u32,
    pub root_pubkeys: [[u8; P256_PUBLIC_KEY_LEN]; IMAGE_SIGNATURE_COUNT],
    pub attestation_present: u8,
    pub _reserved0: [u8; 3],
    pub io_protection_key: [u8; 32],
    pub attestation_device_pubkey: [u8; 64],
    pub attestation_certificate: [u8; 64],
    pub attestation_root_pubkey_identifier: [u8; 32],
    pub attestation_bootloader_hash: [u8; 32],
}

const _: [(); FLASH_PAGE_SIZE - core::mem::size_of::<ImmutablePage>()] =
    [(); FLASH_PAGE_SIZE - core::mem::size_of::<ImmutablePage>()];

impl ImmutablePage {
    pub fn blank() -> Self {
        Self {
            magic: IMMUTABLE_PAGE_MAGIC,
            version: IMMUTABLE_PAGE_VERSION,
            root_pubkeys: [[0; P256_PUBLIC_KEY_LEN]; IMAGE_SIGNATURE_COUNT],
            attestation_present: 0,
            _reserved0: [0; 3],
            io_protection_key: [0; 32],
            attestation_device_pubkey: [0; 64],
            attestation_certificate: [0; 64],
            attestation_root_pubkey_identifier: [0; 32],
            attestation_bootloader_hash: [0; 32],
        }
    }

    pub fn from_address(address: usize) -> Result<Self, ()> {
        let bytes = unsafe { &*(address as *const [u8; FLASH_PAGE_SIZE]) };
        Self::from_page_bytes(bytes)
    }

    pub fn from_page_bytes(bytes: &[u8; FLASH_PAGE_SIZE]) -> Result<Self, ()> {
        let page = unsafe { core::ptr::read_unaligned(bytes.as_ptr().cast::<Self>()) };
        if page.is_valid() { Ok(page) } else { Err(()) }
    }

    pub fn to_page_bytes(self) -> [u8; FLASH_PAGE_SIZE] {
        let mut page = [0xff; FLASH_PAGE_SIZE];
        let bytes = self.as_bytes();
        page[..bytes.len()].copy_from_slice(bytes);
        page
    }

    fn is_valid(&self) -> bool {
        self.magic == IMMUTABLE_PAGE_MAGIC && self.version == IMMUTABLE_PAGE_VERSION
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const ImmutablePage).cast::<u8>(),
                core::mem::size_of::<ImmutablePage>(),
            )
        }
    }
}

pub const fn build_immutable_page_bytes(
    root_pubkeys: [[u8; P256_PUBLIC_KEY_LEN]; IMAGE_SIGNATURE_COUNT],
) -> [u8; FLASH_PAGE_SIZE] {
    let mut page = [0xff; FLASH_PAGE_SIZE];
    let magic = IMMUTABLE_PAGE_MAGIC.to_le_bytes();
    let version = IMMUTABLE_PAGE_VERSION.to_le_bytes();
    let mut index = 0usize;
    while index < 4 {
        page[index] = magic[index];
        page[4 + index] = version[index];
        index += 1;
    }

    let mut page_index = 8usize;
    let mut key_index = 0usize;
    while key_index < IMAGE_SIGNATURE_COUNT {
        let mut byte_index = 0usize;
        while byte_index < P256_PUBLIC_KEY_LEN {
            page[page_index] = root_pubkeys[key_index][byte_index];
            page_index += 1;
            byte_index += 1;
        }
        key_index += 1;
    }

    while page_index < core::mem::size_of::<ImmutablePage>() {
        page[page_index] = 0;
        page_index += 1;
    }

    page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_page_roundtrip() {
        let mut immutable = ImmutablePage::blank();
        immutable.root_pubkeys[0] = [0x11; P256_PUBLIC_KEY_LEN];
        immutable.io_protection_key = [0x22; 32];

        let page = immutable.to_page_bytes();
        let decoded = ImmutablePage::from_page_bytes(&page).unwrap();

        assert_eq!(decoded.root_pubkeys[0], [0x11; P256_PUBLIC_KEY_LEN]);
        assert_eq!(decoded.io_protection_key, [0x22; 32]);
    }
}
