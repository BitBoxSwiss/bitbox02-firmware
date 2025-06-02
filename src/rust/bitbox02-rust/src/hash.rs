// Copyright 2025 Shift Crypto AG
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

/// Implements the digest traits for Sha512 backing it with the wally_sha512 C function. This is
/// done to avoid using a second sha512 implementation like `sha2::Sha512`, which bloats the binary
/// by an additional ~12.7kB (at the time of writing).
///
/// This implementation accumulates the data to be hashed in heap, it does **not** hash in a
/// streaming fashion, even when using `update()`.
#[derive(Default, Clone)]
pub struct Sha512 {
    message: Vec<u8>,
}

impl digest::HashMarker for Sha512 {}

impl digest::OutputSizeUser for Sha512 {
    type OutputSize = digest::typenum::U64;
}

impl digest::FixedOutput for Sha512 {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        // use digest::Digest;
        // out.copy_from_slice(&sha2::Sha512::digest(&self.message));
        out.copy_from_slice(&bitbox02::sha512(&self.message));
    }
}

impl digest::Update for Sha512 {
    fn update(&mut self, data: &[u8]) {
        self.message.extend(data);
    }
}

impl digest::Reset for Sha512 {
    fn reset(&mut self) {
        self.message = vec![];
    }
}

impl digest::core_api::BlockSizeUser for Sha512 {
    type BlockSize = digest::typenum::U128;
}
