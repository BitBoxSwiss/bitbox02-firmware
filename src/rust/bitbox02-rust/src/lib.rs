// SPDX-License-Identifier: Apache-2.0

// Since we are targeting embedded we exclude the standard library by default
#![no_std]
// When compiling for testing we allow certain warnings.
#![cfg_attr(test, allow(unused_imports, dead_code))]

mod pb {
    include!("./shiftcrypto.bitbox02.rs");
}
mod pb_backup {
    include!("./shiftcrypto.bitbox02.backups.rs");
}

#[macro_use]
pub mod general;
pub mod async_usb;
pub mod attestation;
pub mod backup;
pub mod bb02_async;
mod bip32;
pub mod bip39;
pub mod communication_mode;
pub mod hal;
pub mod hash;
pub mod hww;
pub mod keystore;
#[cfg(all(
    feature = "firmware",
    not(any(feature = "c-unit-testing", feature = "simulator-graphical"))
))]
pub mod main_loop;
pub mod reset;
pub mod salt;
pub mod secp256k1;
#[cfg(feature = "app-u2f")]
mod u2f;
mod version;
pub mod workflow;
#[cfg(any(feature = "app-bitcoin", feature = "app-litecoin"))]
mod xpubcache;

// for `format!`
#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate bitbox_aes;

// Manually link fatfs
extern crate fatfs_sys;

//
// C interface
//

/// `private_key_out` must be 32 bytes.
#[unsafe(no_mangle)]
pub extern "C" fn rust_noise_generate_static_private_key(
    mut private_key_out: util::bytes::BytesMut,
) {
    let key = bitbox02_noise::generate_static_private_key::<hww::noise::BB02Random32>();
    private_key_out.as_mut().copy_from_slice(&key[..]);
}
