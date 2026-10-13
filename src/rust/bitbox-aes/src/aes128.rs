// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use cipher::{
    Block, BlockCipherEncBackend, BlockCipherEncClosure, BlockCipherEncrypt, BlockSizeUser, InOut,
    Key, KeyInit, KeySizeUser, ParBlocksSizeUser,
    consts::{U1, U16},
};
use core::cell::RefCell;
use zeroize::Zeroizing;

pub struct Aes128 {
    backend: Box<Backend>,
}

struct Backend {
    cipher: ctaes::Aes128,
    block: RefCell<Zeroizing<[u8; 16]>>,
}

impl Drop for Backend {
    fn drop(&mut self) {
        // ctaes stores only the expanded key here and does not wipe it on drop.
        const { assert!(!core::mem::needs_drop::<ctaes::Aes128>()) };
        unsafe {
            util::rust_util_zero(
                core::ptr::from_mut(&mut self.cipher).cast(),
                core::mem::size_of::<ctaes::Aes128>(),
            );
        }
    }
}

impl KeySizeUser for Aes128 {
    type KeySize = U16;
}

impl KeyInit for Aes128 {
    fn new(key: &Key<Self>) -> Self {
        Self {
            backend: Box::new(Backend {
                cipher: ctaes::Aes128::new(key.as_ref()),
                block: RefCell::new(Zeroizing::new([0; 16])),
            }),
        }
    }
}

impl BlockSizeUser for Aes128 {
    type BlockSize = U16;
}

impl BlockCipherEncrypt for Aes128 {
    fn encrypt_with_backend(&self, f: impl BlockCipherEncClosure<BlockSize = Self::BlockSize>) {
        f.call(self.backend.as_ref());
    }
}

impl BlockSizeUser for Backend {
    type BlockSize = U16;
}

impl ParBlocksSizeUser for Backend {
    type ParBlocksSize = U1;
}

impl BlockCipherEncBackend for Backend {
    fn encrypt_block(&self, mut block: InOut<'_, '_, Block<Self>>) {
        let mut scratch = self.block.borrow_mut();
        self.cipher
            .encrypt_to_slice(block.get_in(), scratch.as_mut_slice());
        block.get_out().copy_from_slice(scratch.as_slice());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;

    #[test]
    fn test_encrypt_block() {
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let plain = hex!("00112233445566778899aabbccddeeff");
        let expected = hex!("69c4e0d86a7b0430d8cdb78070b4c55a");
        let cipher = Aes128::new(&key.into());
        let mut block = plain.into();
        cipher.encrypt_block(&mut block);
        assert_eq!(block.as_slice(), expected);
        let mut output = Block::<Aes128>::default();
        cipher.encrypt_block_b2b(&plain.into(), &mut output);
        assert_eq!(output.as_slice(), expected);
    }
}
