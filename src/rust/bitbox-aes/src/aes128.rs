// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use cipher::{
    Block, BlockBackend, BlockCipher, BlockClosure, BlockEncrypt, BlockSizeUser, Key, KeyInit,
    KeySizeUser, ParBlocksSizeUser,
    consts::{U1, U16},
    inout::InOut,
};
use core::cell::RefCell;
use zeroize::Zeroizing;

pub struct Aes128 {
    backend: RefCell<Box<Backend>>,
}

struct Backend {
    cipher: ctaes::Aes128,
    block: Zeroizing<[u8; 16]>,
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
            backend: RefCell::new(Box::new(Backend {
                cipher: ctaes::Aes128::new(key.as_slice().try_into().unwrap()),
                block: Zeroizing::new([0; 16]),
            })),
        }
    }
}

impl BlockSizeUser for Aes128 {
    type BlockSize = U16;
}

impl BlockCipher for Aes128 {}

impl BlockEncrypt for Aes128 {
    fn encrypt_with_backend(&self, f: impl BlockClosure<BlockSize = Self::BlockSize>) {
        f.call(&mut **self.backend.borrow_mut());
    }
}

impl BlockSizeUser for Backend {
    type BlockSize = U16;
}

impl ParBlocksSizeUser for Backend {
    type ParBlocksSize = U1;
}

impl BlockBackend for Backend {
    fn proc_block(&mut self, mut block: InOut<'_, '_, Block<Self>>) {
        self.cipher
            .encrypt_to_slice(block.get_in(), self.block.as_mut_slice());
        block.get_out().copy_from_slice(self.block.as_slice());
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
