// SPDX-License-Identifier: Apache-2.0

use blake2::Blake2bVarCore;
use digest::{
    Output, Update,
    block_api::{Buffer, UpdateCore, VariableOutputCore},
};

pub(super) struct Blake2b {
    core: Blake2bVarCore,
    buffer: Buffer<Blake2bVarCore>,
    output_len: usize,
}

impl Blake2b {
    pub(super) fn new(output_len: usize) -> Self {
        assert!((1..=64).contains(&output_len));
        Self {
            core: Blake2bVarCore::new(output_len).unwrap(),
            buffer: Buffer::<Blake2bVarCore>::default(),
            output_len,
        }
    }

    pub(super) fn finalize(mut self, output: &mut [u8]) {
        assert_eq!(output.len(), self.output_len);
        let mut full = Output::<Blake2bVarCore>::default();
        self.core
            .finalize_variable_core(&mut self.buffer, &mut full);
        output.copy_from_slice(&full[..self.output_len]);
    }
}

impl Update for Blake2b {
    fn update(&mut self, data: &[u8]) {
        let core = &mut self.core;
        self.buffer
            .digest_blocks(data, |blocks| core.update_blocks(blocks));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use blake2::digest::{
        Digest,
        consts::{U20, U28, U32},
    };

    #[test]
    fn test_runtime_output_lengths_and_chunking() {
        for input_len in [0, 1, 127, 128, 129, 255, 256, 257, 1024] {
            let input: Vec<u8> = (0..input_len).map(|n| (n % 251) as u8).collect();
            for (output_len, expected) in [
                (20, blake2::Blake2b::<U20>::digest(&input).to_vec()),
                (28, blake2::Blake2b::<U28>::digest(&input).to_vec()),
                (32, blake2::Blake2b::<U32>::digest(&input).to_vec()),
            ] {
                for chunk_len in [1, 7, 127, 128, 129, 1025] {
                    let mut hasher = Blake2b::new(output_len);
                    hasher.update(&[]);
                    for chunk in input.chunks(chunk_len) {
                        hasher.update(chunk);
                    }
                    hasher.update(&[]);
                    let mut actual = alloc::vec![0u8; output_len];
                    hasher.finalize(&mut actual);
                    assert_eq!(actual, expected);
                }
            }
        }
    }
}
