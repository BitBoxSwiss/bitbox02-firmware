// Copyright 2020 Shift Crypto AG
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

use core::convert::TryInto;
use core::fmt::Write;

use util::c_types::size_t;

use bitbox02_rust::apps::bitcoin::util::format_amount;
use bitbox02_rust::apps::bitcoin::{bip143, keypath};

/// # Safety
/// `keypath` must be not NULL and contain `keypath_len` u32 elements.
#[no_mangle]
pub unsafe extern "C" fn rust_bitcoin_keypath_validate_account(
    keypath: *const u32,
    keypath_len: size_t,
    expected_purpose: u32,
    expected_coin: u32,
) -> bool {
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    bitbox02_rust::apps::bitcoin::keypath::validate_account(
        keypath,
        expected_purpose,
        expected_coin,
    )
    .is_ok()
}

/// # Safety
/// `keypath` must be not NULL and contain `keypath_len` u32 elements.
#[no_mangle]
pub unsafe extern "C" fn rust_bitcoin_keypath_validate_account_multisig(
    keypath: *const u32,
    keypath_len: size_t,
    expected_coin: u32,
    script_type: i32,
) -> bool {
    let script_type = match keypath::MultisigScriptType::from_i32(script_type) {
        Some(script_type) => script_type,
        None => return false,
    };
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    keypath::validate_account_multisig(keypath, expected_coin, script_type).is_ok()
}

/// # Safety
/// `keypath` must be not NULL and contain `keypath_len` u32 elements.
#[no_mangle]
pub unsafe extern "C" fn rust_bitcoin_keypath_validate_address_multisig(
    keypath: *const u32,
    keypath_len: size_t,
    expected_coin: u32,
    script_type: i32,
) -> bool {
    let script_type = match keypath::MultisigScriptType::from_i32(script_type) {
        Some(script_type) => script_type,
        None => return false,
    };
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    keypath::validate_address_multisig(keypath, expected_coin, script_type).is_ok()
}

/// # Safety
/// `keypath` must be not NULL and contain `keypath_len` u32 elements.
#[no_mangle]
pub unsafe extern "C" fn rust_bitcoin_keypath_validate_account_simple(
    keypath: *const u32,
    keypath_len: size_t,
    expected_coin: u32,
    script_type: i32,
) -> bool {
    let script_type = match keypath::SimpleType::from_i32(script_type) {
        Some(script_type) => script_type,
        None => return false,
    };
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    keypath::validate_account_simple(keypath, expected_coin, script_type).is_ok()
}

/// # Safety
/// `keypath` must be not NULL and contain `keypath_len` u32 elements.
#[no_mangle]
pub unsafe extern "C" fn rust_bitcoin_keypath_validate_address_simple(
    keypath: *const u32,
    keypath_len: size_t,
    expected_coin: u32,
    script_type: i32,
) -> bool {
    let script_type = match keypath::SimpleType::from_i32(script_type) {
        Some(script_type) => script_type,
        None => return false,
    };
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    keypath::validate_address_simple(keypath, expected_coin, script_type).is_ok()
}

/// `out` should be at least 31+len(unit) bytes.
#[no_mangle]
pub extern "C" fn rust_bitcoin_util_format_amount(
    satoshi: u64,
    unit: crate::util::CStr,
    mut out: crate::util::CStrMut,
) {
    let result = format_amount(satoshi, unit.as_ref());
    out.write_str(&result).unwrap();
}

#[repr(C)]
pub struct Bip143Args {
    version: u32,
    hash_prevouts: *const u8,
    hash_sequence: *const u8,
    outpoint_hash: *const u8,
    outpoint_index: u32,
    sighash_script: crate::util::Bytes,
    prevout_value: u64,
    sequence: u32,
    hash_outputs: *const u8,
    locktime: u32,
    sighash_flags: u32,
}

/// # Safety
/// The *const u8 buffers must be valid 32 byte buffers.
#[no_mangle]
pub unsafe extern "C" fn rust_bitcoin_bip143_sighash(
    args: &Bip143Args,
    mut hash_out: crate::util::BytesMut,
) {
    let hash = bip143::sighash(&bip143::Args {
        version: args.version,
        hash_prevouts: core::slice::from_raw_parts(args.hash_prevouts, 32)
            .try_into()
            .unwrap(),
        hash_sequence: core::slice::from_raw_parts(args.hash_sequence, 32)
            .try_into()
            .unwrap(),
        outpoint_hash: core::slice::from_raw_parts(args.outpoint_hash, 32)
            .try_into()
            .unwrap(),
        outpoint_index: args.outpoint_index,
        sighash_script: args.sighash_script.as_ref(),
        prevout_value: args.prevout_value,
        sequence: args.sequence,
        hash_outputs: core::slice::from_raw_parts(args.hash_outputs, 32)
            .try_into()
            .unwrap(),
        locktime: args.locktime,
        sighash_flags: args.sighash_flags,
    });
    hash_out.as_mut().copy_from_slice(&hash[..]);
}
