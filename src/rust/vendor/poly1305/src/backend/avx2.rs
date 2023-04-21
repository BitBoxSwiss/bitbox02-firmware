//! AVX2 implementation of the Poly1305 state machine.

// The State struct and its logic was originally derived from Goll and Gueron's AVX2 C
// code:
//     [Vectorization of Poly1305 message authentication code](https://ieeexplore.ieee.org/document/7113463)
//
// which was sourced from Bhattacharyya and Sarkar's modified variant:
//     [Improved SIMD Implementation of Poly1305](https://eprint.iacr.org/2019/842)
//     https://github.com/Sreyosi/Improved-SIMD-Implementation-of-Poly1305
//
// The logic has been extensively rewritten and documented, and several bugs in the
// original C code were fixed.
//
// Note that State only implements the original Goll-Gueron algorithm, not the
// optimisations provided by Bhattacharyya and Sarkar. The latter require the message
// length to be known, which is incompatible with the streaming API of UniversalHash.

use core::convert::TryInto;
use universal_hash::generic_array::GenericArray;

use crate::{Block, Key, Tag};

mod helpers;
use self::helpers::*;

#[derive(Copy, Clone)]
struct Initialized {
    p: Aligned4x130,
    m: SpacedMultiplier4x130,
    r4: PrecomputedMultiplier,
}

#[derive(Clone)]
pub(crate) struct State {
    k: AdditionKey,
    r1: PrecomputedMultiplier,
    r2: PrecomputedMultiplier,
    initialized: Option<Initialized>,
    cached_blocks: [Block; 4],
    num_cached_blocks: usize,
    partial_block: Option<Block>,
}

impl State {
    /// Initialize Poly1305 [`State`] with the given key
    pub(crate) fn new(key: &Key) -> Self {
        // Prepare addition key and polynomial key.
        let (k, r1) = unsafe { prepare_keys(key) };

        // Precompute R^2.
        let r2 = (r1 * r1).reduce();

        State {
            k,
            r1,
            r2: r2.into(),
            initialized: None,
            cached_blocks: [Block::default(); 4],
            num_cached_blocks: 0,
            partial_block: None,
        }
    }

    /// Reset internal state
    pub(crate) fn reset(&mut self) {
        self.initialized = None;
        self.num_cached_blocks = 0;
    }

    /// Compute a Poly1305 block
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn compute_block(&mut self, block: &Block, partial: bool) {
        // We can cache a single partial block.
        if partial {
            assert!(self.partial_block.is_none());
            self.partial_block = Some(*block);
            return;
        }

        self.cached_blocks[self.num_cached_blocks].copy_from_slice(block);
        if self.num_cached_blocks < 3 {
            self.num_cached_blocks += 1;
            return;
        } else {
            self.num_cached_blocks = 0;
        }

        if let Some(inner) = &mut self.initialized {
            // P <-- R^4 * P + blocks
            inner.p =
                (&inner.p * inner.r4).reduce() + Aligned4x130::from_blocks(&self.cached_blocks);
        } else {
            // Initialize the polynomial.
            let p = Aligned4x130::from_blocks(&self.cached_blocks);

            // Initialize the multiplier (used to merge down the polynomial during
            // finalization).
            let (m, r4) = SpacedMultiplier4x130::new(self.r1, self.r2);

            self.initialized = Some(Initialized { p, m, r4 })
        }
    }

    /// Finalize output producing a [`Tag`]
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn finalize(&mut self) -> Tag {
        assert!(self.num_cached_blocks < 4);
        let mut data = &self.cached_blocks[..];

        // T ← R◦T
        // P = T_0 + T_1 + T_2 + T_3
        let mut p = self
            .initialized
            .take()
            .map(|inner| (inner.p * inner.m).sum().reduce());

        if self.num_cached_blocks >= 2 {
            // Compute 32 byte block (remaining data < 64 bytes)
            let mut c = Aligned2x130::from_blocks(data[..2].try_into().unwrap());
            if let Some(p) = p {
                c = c + p;
            }
            p = Some(c.mul_and_sum(self.r1, self.r2).reduce());
            data = &data[2..];
            self.num_cached_blocks -= 2;
        }

        if self.num_cached_blocks == 1 {
            // Compute 16 byte block (remaining data < 32 bytes)
            let mut c = Aligned130::from_block(&data[0]);
            if let Some(p) = p {
                c = c + p;
            }
            p = Some((c * self.r1).reduce());
            self.num_cached_blocks -= 1;
        }

        if let Some(block) = &self.partial_block {
            // Compute last block (remaining data < 16 bytes)
            let mut c = Aligned130::from_partial_block(block);
            if let Some(p) = p {
                c = c + p;
            }
            p = Some((c * self.r1).reduce());
        }

        // Compute tag: p + k mod 2^128
        let mut tag = GenericArray::<u8, _>::default();
        let tag_int = if let Some(p) = p {
            self.k + p
        } else {
            self.k.into()
        };
        tag_int.write(tag.as_mut_slice());

        Tag::new(tag)
    }
}
