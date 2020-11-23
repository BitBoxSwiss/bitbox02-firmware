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

/// Safety: keypath must be a valid buffer of `usize` elements of `uint32_t`-sized elements.
#[no_mangle]
pub unsafe extern "C" fn rust_bip32_to_string(
    keypath: *const u32,
    keypath_len: size_t,
    mut out: crate::util::CStrMut,
) {
    let keypath_string = util::bip32::to_string(core::slice::from_raw_parts(keypath, keypath_len));
    out.write_str(keypath_string.as_str()).unwrap()
}
