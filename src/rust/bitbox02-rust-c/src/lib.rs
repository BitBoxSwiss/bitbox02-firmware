// Copyright 2020 Shift Cryptosecurity AG
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

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod alloc;

mod util;

#[cfg(feature = "firmware")]
mod async_usb;
#[cfg(feature = "firmware")]
mod bip39;
#[cfg(feature = "bitbox02-noise")]
mod noise;
#[cfg(feature = "firmware")]
mod p256;
#[cfg(feature = "sha2")]
mod sha2;
#[cfg(feature = "app-u2f")]
mod u2f;
#[cfg(feature = "firmware")]
mod workflow;

#[cfg(feature = "firmware")]
mod der;

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen and over RTT. If we compile with
// `panic=abort` this code will never get executed.
#[cfg(not(test))]
#[cfg(not(feature = "testing"))]
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "firmware")]
    ::util::log::log!("{}", info);
    #[cfg(feature = "firmware")]
    bitbox02_rust::print_screen!(0, "Error: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_rtt_init() {
    ::util::log::rtt_init()
}

#[no_mangle]
pub extern "C" fn rust_rtt_flush() {
    ::util::log::rtt_flush();
}

/// # Safety
///
/// The pointer `ptr` must point to a null terminated string
#[no_mangle]
#[cfg_attr(not(feature = "rtt"), allow(unused))]
pub unsafe extern "C" fn rust_log(ptr: *const ::util::c_types::c_char) {
    #[cfg(feature = "rtt")]
    {
        if ptr.is_null() {
            panic!("`ptr` must be a valid pointer");
        }
        let s = unsafe { core::ffi::CStr::from_ptr(ptr as _) };
        let s = unsafe { core::str::from_utf8_unchecked(s.to_bytes()) };
        ::util::log::rtt_target::rprintln!("{}", s);
    }
}

#[no_mangle]
pub extern "C" fn rust_cipher_encrypt(
    iv: crate::util::Bytes,
    key: crate::util::Bytes,
    plain: crate::util::Bytes,
    mut out: crate::util::BytesMut,
    out_len: &mut usize,
) {
    let enc = bitbox_aes::encrypt_with_hmac(
        &iv.as_ref().try_into().unwrap(),
        key.as_ref(),
        plain.as_ref(),
    );
    out.as_mut()[..enc.len()].copy_from_slice(&enc);
    *out_len = enc.len();
}

#[no_mangle]
pub extern "C" fn rust_cipher_decrypt(
    key: crate::util::Bytes,
    cipher: crate::util::Bytes,
    mut out: crate::util::BytesMut,
    out_len: &mut usize,
) -> bool {
    match bitbox_aes::decrypt_with_hmac(key.as_ref(), cipher.as_ref()) {
        Ok(dec) => {
            out.as_mut()[..dec.len()].copy_from_slice(&dec);
            *out_len = dec.len();
            true
        }
        Err(_) => false,
    }
}

/// # Safety
///
/// keypath pointer has point to a buffer of length `keypath_len` uint32 elements.
#[cfg(feature = "firmware")]
#[no_mangle]
pub unsafe extern "C" fn rust_secp256k1_get_private_key(
    keypath: *const u32,
    keypath_len: usize,
    mut out: crate::util::BytesMut,
) -> bool {
    match bitbox02_rust::keystore::secp256k1_get_private_key(core::slice::from_raw_parts(
        keypath,
        keypath_len,
    )) {
        Ok(private_key) => {
            out.as_mut().copy_from_slice(&private_key);
            true
        }
        Err(()) => false,
    }
}

/// # Safety
///
/// The pointer `data` must point to a buffer of length `len`.
#[no_mangle]
#[allow(static_mut_refs)]
#[cfg_attr(not(feature = "rtt"), allow(unused))]
pub unsafe extern "C" fn rust_rtt_ch1_write(data: *const u8, len: usize) {
    #[cfg(feature = "rtt")]
    {
        let buf = unsafe { core::slice::from_raw_parts(data, len) };
        let channel = unsafe { ::util::log::CH1_UP.as_mut().unwrap() };
        let mut written = 0;
        while written < len {
            written += channel.write(buf);
        }
    }
}

/// # Safety
///
/// The pointer `data` must point to a buffer of length `len`.
#[no_mangle]
#[allow(static_mut_refs)]
#[cfg_attr(not(feature = "rtt"), allow(unused))]
pub unsafe extern "C" fn rust_rtt_ch0_read(data: *mut u8, len: usize) -> usize {
    #[cfg(feature = "rtt")]
    {
        let buf = unsafe { core::slice::from_raw_parts_mut(data, len) };
        let channel = unsafe { ::util::log::CH0_DOWN.as_mut().unwrap() };
        channel.read(buf)
    }
    #[cfg(not(feature = "rtt"))]
    0
}
