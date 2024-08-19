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

#[cfg(target_arch = "arm")]
pub fn mcu_32_bytes(out: &mut [u8; 32]) {
    unsafe { bitbox02_sys::random_32_bytes_mcu(out.as_mut_ptr()) }
}

#[cfg(not(target_arch = "arm"))]
pub fn mcu_32_bytes(out: &mut [u8; 32]) {
    extern "C" {
        fn rand() -> util::c_types::c_int;
    }

    for elem in out.iter_mut() {
        // Not uniform, but it's only for tests...
        *elem = unsafe { rand() as _ };
    }
}

#[cfg(feature = "testing")]
pub fn mock_reset() {
    unsafe {
        bitbox02_sys::random_mock_reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcu_32_bytes() {
        let mut result = [0; 32];
        mcu_32_bytes(&mut result);
        assert!([0; 32] != result);
    }
}
