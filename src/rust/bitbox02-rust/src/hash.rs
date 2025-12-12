// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

/// Implements the digest traits for Sha512 backing it with bitcoin::hashes. This is done to avoid
/// using a second sha512 implementation like `sha2::Sha512`, which bloats the binary by an
/// additional ~12.7kB (at the time of writing).
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
        use bitcoin::hashes::Hash;
        out.copy_from_slice(bitcoin::hashes::sha512::Hash::hash(&self.message).as_byte_array())
    }
}

impl digest::Update for Sha512 {
    fn update(&mut self, data: &[u8]) {
        self.message.extend(data);
    }
}

impl digest::Reset for Sha512 {
    fn reset(&mut self) {
        self.message.clear()
    }
}

impl digest::core_api::BlockSizeUser for Sha512 {
    type BlockSize = digest::typenum::U128;
}

impl digest::core_api::UpdateCore for Sha512 {
    fn update_blocks(&mut self, blocks: &[digest::core_api::Block<Self>]) {
        self.message
            .extend(blocks.iter().flat_map(|b| b.iter().copied()))
    }
}
