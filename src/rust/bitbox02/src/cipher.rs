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

use alloc::vec::Vec;

pub fn aes_hmac_encrypt(input: &[u8], key: &[u8]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut output = zeroize::Zeroizing::new(vec![0u8; input.len() + 64]);
    let mut output_size: usize = output.len();
    match unsafe {
        bitbox02_sys::cipher_aes_hmac_encrypt(
            input.as_ptr(),
            input.len() as _,
            output.as_mut_ptr(),
            &mut output_size,
            key.as_ptr(),
        )
    } {
        true => {
            output.truncate(output_size);
            Ok(output)
        }
        false => Err(()),
    }
}
