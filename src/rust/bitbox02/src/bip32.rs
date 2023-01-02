// Copyright 2023 Shift Crypto AG
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

use alloc::vec;
use alloc::vec::Vec;
use bitbox02_sys::BIP32_SERIALIZED_LEN;

pub fn derive_xpub(xpub78: &[u8], keypath: &[u32]) -> Result<Vec<u8>, ()> {
    let mut xpub = vec![0u8; BIP32_SERIALIZED_LEN as _];
    match unsafe {
        bitbox02_sys::bip32_derive_xpub(
            xpub78.as_ptr(),
            keypath.as_ptr(),
            keypath.len(),
            xpub.as_mut_ptr(),
        )
    } {
        true => Ok(xpub),
        false => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_xpub() {
        // xpub661MyMwAqRbcGpuMRXa55WgyqinF4dpxvqQK63xBHtnH5yK4e3cTLqbX9CP4mEMHUbqsjSQ8y3hhbAzuMhpn8eEiLNVSWYaVSbKMAtUPyYH
        let xpub = b"\x04\x88\xb2\x1e\x00\x00\x00\x00\x00\x00\x00\x00\x00\xe5\x67\x65\x23\x1c\x63\xfd\x41\xe0\x42\xbe\x95\xd0\x17\x81\x75\x23\x49\xc6\x6b\x10\x0c\x50\xdb\x84\x90\x95\xa7\x4e\x9f\x69\x6f\x02\xfb\xca\x9a\xde\xb7\xdb\xc9\x62\xfa\xa0\xf6\x0e\x32\x8f\x11\xfe\x84\xec\xc5\x3f\xf6\x22\xe9\x9d\x13\xa4\x60\xa8\x47\x84\x54\xa7";

        assert_eq!(derive_xpub(xpub, &[]).unwrap(), xpub.to_vec());
        // xpub6CYiDoWMtLVQNrc4tbAvuRk5wjsp6MFgtYEdBUV7TGLUjutavHdEKLu9KpTpRxEZULbSwM1UQPaQpqAhmWYvngXCGHGE7hSZFNofeSRzmk5
        let expected = b"\x04\x88\xb2\x1e\x03\x79\xd7\xa1\x2b\x00\x00\x00\x02\x00\x43\x25\x50\x64\xb5\x0c\x27\x32\x98\x22\x4a\xf7\xb1\x18\x7b\x27\xd4\x14\x00\x04\x71\x84\x64\x2a\x6f\x46\xe0\x95\x90\xe5\xc7\x02\xf1\xf4\x18\xcc\xc3\x19\x2d\x1b\xa9\x6b\xfe\x40\x96\x57\x8a\x25\x7c\x73\x5b\x92\x7c\x4b\x1e\x55\x2f\x7e\x1a\x03\x4b\x56\xf3\x85";
        assert_eq!(derive_xpub(xpub, &[0, 1, 2]).unwrap(), expected.to_vec());
    }
}
