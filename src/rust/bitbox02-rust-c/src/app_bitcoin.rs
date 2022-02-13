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

use core::fmt::Write;

use util::c_types::size_t;

use bitbox02_rust::apps::bitcoin::keypath;
use bitbox02_rust::apps::bitcoin::util::format_amount;

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
