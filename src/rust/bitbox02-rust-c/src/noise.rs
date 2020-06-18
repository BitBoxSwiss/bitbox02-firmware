// Copyright 2020 Shift Cryptosecurity AG
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

/// `private_key_out` must be 32 bytes.
#[no_mangle]
pub extern "C" fn rust_noise_generate_static_private_key(
    mut private_key_out: crate::util::BytesMut,
) {
    let key =
        bitbox02_noise::generate_static_private_key::<bitbox02_rust::hww::noise::BB02Random32>();
    private_key_out.as_mut().copy_from_slice(&key[..]);
}
