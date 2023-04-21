//! The ChaCha20 core function. Defined in RFC 8439 Section 2.3.
//!
//! <https://tools.ietf.org/html/rfc8439#section-2.3>
//!
//! Portable implementation which does not rely on architecture-specific
//! intrinsics.

use crate::{rounds::Rounds, BLOCK_SIZE, CONSTANTS, IV_SIZE, KEY_SIZE};
use core::{convert::TryInto, marker::PhantomData};

/// Size of buffers passed to `generate` and `apply_keystream` for this backend
#[allow(dead_code)]
pub(crate) const BUFFER_SIZE: usize = BLOCK_SIZE;

/// Number of 32-bit words in the ChaCha20 state
const STATE_WORDS: usize = 16;

/// The ChaCha20 core function.
// TODO(tarcieri): zeroize?
#[derive(Clone)]
#[allow(dead_code)]
pub struct Core<R: Rounds> {
    /// Internal state of the core function
    state: [u32; STATE_WORDS],

    /// Number of rounds to perform
    rounds: PhantomData<R>,
}

#[allow(dead_code)]
impl<R: Rounds> Core<R> {
    /// Initialize core function with the given key, IV, and number of rounds
    pub fn new(key: &[u8; KEY_SIZE], iv: [u8; IV_SIZE]) -> Self {
        let state = [
            CONSTANTS[0],
            CONSTANTS[1],
            CONSTANTS[2],
            CONSTANTS[3],
            u32::from_le_bytes(key[..4].try_into().unwrap()),
            u32::from_le_bytes(key[4..8].try_into().unwrap()),
            u32::from_le_bytes(key[8..12].try_into().unwrap()),
            u32::from_le_bytes(key[12..16].try_into().unwrap()),
            u32::from_le_bytes(key[16..20].try_into().unwrap()),
            u32::from_le_bytes(key[20..24].try_into().unwrap()),
            u32::from_le_bytes(key[24..28].try_into().unwrap()),
            u32::from_le_bytes(key[28..32].try_into().unwrap()),
            0,
            0,
            u32::from_le_bytes(iv[0..4].try_into().unwrap()),
            u32::from_le_bytes(iv[4..].try_into().unwrap()),
        ];

        Self {
            state,
            rounds: PhantomData,
        }
    }

    /// Generate output, overwriting data already in the buffer
    #[inline]
    pub fn generate(&mut self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);
        self.counter_setup(counter);

        let mut state = self.state;
        self.rounds(&mut state);

        for (i, chunk) in output.chunks_mut(4).enumerate() {
            chunk.copy_from_slice(&state[i].to_le_bytes());
        }
    }

    /// Apply generated keystream to the output buffer
    #[inline]
    #[cfg(feature = "cipher")]
    pub fn apply_keystream(&mut self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);
        self.counter_setup(counter);

        let mut state = self.state;
        self.rounds(&mut state);

        for (i, chunk) in output.chunks_mut(4).enumerate() {
            for (a, b) in chunk.iter_mut().zip(&state[i].to_le_bytes()) {
                *a ^= *b;
            }
        }
    }

    #[inline]
    fn counter_setup(&mut self, counter: u64) {
        self.state[12] = (counter & 0xffff_ffff) as u32;
        self.state[13] = ((counter >> 32) & 0xffff_ffff) as u32;
    }

    #[inline]
    fn rounds(&mut self, state: &mut [u32; STATE_WORDS]) {
        for _ in 0..(R::COUNT / 2) {
            // column rounds
            quarter_round(0, 4, 8, 12, state);
            quarter_round(1, 5, 9, 13, state);
            quarter_round(2, 6, 10, 14, state);
            quarter_round(3, 7, 11, 15, state);

            // diagonal rounds
            quarter_round(0, 5, 10, 15, state);
            quarter_round(1, 6, 11, 12, state);
            quarter_round(2, 7, 8, 13, state);
            quarter_round(3, 4, 9, 14, state);
        }

        for (s1, s0) in state.iter_mut().zip(&self.state) {
            *s1 = s1.wrapping_add(*s0);
        }
    }
}

/// The ChaCha20 quarter round function
#[inline]
pub(crate) fn quarter_round(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    state: &mut [u32; STATE_WORDS],
) {
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(16);

    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(12);

    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(8);

    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(7);
}
