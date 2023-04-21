//! The ChaCha20 core function. Defined in RFC 8439 Section 2.3.
//!
//! <https://tools.ietf.org/html/rfc8439#section-2.3>
//!
//! AVX2-optimized implementation for x86/x86-64 CPUs adapted from the SUPERCOP
//! `goll_gueron` backend (public domain) described in:
//!
//! Goll, M., and Gueron,S.: Vectorization of ChaCha Stream Cipher. Cryptology ePrint Archive,
//! Report 2013/759, November, 2013, <https://eprint.iacr.org/2013/759.pdf>

use super::autodetect::BUFFER_SIZE;
use crate::{rounds::Rounds, BLOCK_SIZE, CONSTANTS, IV_SIZE, KEY_SIZE};
use core::{convert::TryInto, marker::PhantomData};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// The number of blocks processed per invocation by this backend.
const BLOCKS: usize = 4;

/// Helper union for accessing per-block state.
///
/// ChaCha20 block state is stored in four 32-bit words, so we can process two blocks in
/// parallel. We store the state words as a union to enable cheap transformations between
/// their interpretations.
///
/// Additionally, we process four blocks at a time to take advantage of ILP.
#[derive(Clone, Copy)]
union StateWord {
    blocks: [__m128i; BLOCKS],
    avx: [__m256i; BLOCKS / 2],
}

impl StateWord {
    #[inline]
    #[must_use]
    #[target_feature(enable = "avx2")]
    unsafe fn add_epi32(self, rhs: Self) -> Self {
        StateWord {
            avx: [
                _mm256_add_epi32(self.avx[0], rhs.avx[0]),
                _mm256_add_epi32(self.avx[1], rhs.avx[1]),
            ],
        }
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "avx2")]
    unsafe fn xor(self, rhs: Self) -> Self {
        StateWord {
            avx: [
                _mm256_xor_si256(self.avx[0], rhs.avx[0]),
                _mm256_xor_si256(self.avx[1], rhs.avx[1]),
            ],
        }
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "avx2")]
    unsafe fn shuffle_epi32<const MASK: i32>(self) -> Self {
        StateWord {
            avx: [
                _mm256_shuffle_epi32(self.avx[0], MASK),
                _mm256_shuffle_epi32(self.avx[1], MASK),
            ],
        }
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "avx2")]
    unsafe fn rol<const BY: i32, const REST: i32>(self) -> Self {
        StateWord {
            avx: [
                _mm256_xor_si256(
                    _mm256_slli_epi32(self.avx[0], BY),
                    _mm256_srli_epi32(self.avx[0], REST),
                ),
                _mm256_xor_si256(
                    _mm256_slli_epi32(self.avx[1], BY),
                    _mm256_srli_epi32(self.avx[1], REST),
                ),
            ],
        }
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "avx2")]
    unsafe fn rol_8(self) -> Self {
        StateWord {
            avx: [
                _mm256_shuffle_epi8(
                    self.avx[0],
                    _mm256_set_epi8(
                        14, 13, 12, 15, 10, 9, 8, 11, 6, 5, 4, 7, 2, 1, 0, 3, 14, 13, 12, 15, 10,
                        9, 8, 11, 6, 5, 4, 7, 2, 1, 0, 3,
                    ),
                ),
                _mm256_shuffle_epi8(
                    self.avx[1],
                    _mm256_set_epi8(
                        14, 13, 12, 15, 10, 9, 8, 11, 6, 5, 4, 7, 2, 1, 0, 3, 14, 13, 12, 15, 10,
                        9, 8, 11, 6, 5, 4, 7, 2, 1, 0, 3,
                    ),
                ),
            ],
        }
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "avx2")]
    unsafe fn rol_16(self) -> Self {
        StateWord {
            avx: [
                _mm256_shuffle_epi8(
                    self.avx[0],
                    _mm256_set_epi8(
                        13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8,
                        11, 10, 5, 4, 7, 6, 1, 0, 3, 2,
                    ),
                ),
                _mm256_shuffle_epi8(
                    self.avx[1],
                    _mm256_set_epi8(
                        13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8,
                        11, 10, 5, 4, 7, 6, 1, 0, 3, 2,
                    ),
                ),
            ],
        }
    }
}

struct State {
    a: StateWord,
    b: StateWord,
    c: StateWord,
    d: StateWord,
}

/// The ChaCha20 core function (AVX2 accelerated implementation for x86/x86_64)
// TODO(tarcieri): zeroize?
#[derive(Clone)]
pub(crate) struct Core<R: Rounds> {
    v0: StateWord,
    v1: StateWord,
    v2: StateWord,
    iv: [i32; 2],
    rounds: PhantomData<R>,
}

impl<R: Rounds> Core<R> {
    /// Initialize core function with the given key size, IV, and number of rounds
    #[inline]
    pub fn new(key: &[u8; KEY_SIZE], iv: [u8; IV_SIZE]) -> Self {
        let (v0, v1, v2) = unsafe { key_setup(key) };
        let iv = [
            i32::from_le_bytes(iv[4..].try_into().unwrap()),
            i32::from_le_bytes(iv[..4].try_into().unwrap()),
        ];

        Self {
            v0,
            v1,
            v2,
            iv,
            rounds: PhantomData,
        }
    }

    #[inline]
    pub fn generate(&self, counter: u64, output: &mut [u8]) {
        unsafe {
            let state = State {
                a: self.v0,
                b: self.v1,
                c: self.v2,
                d: iv_setup(self.iv, counter),
            };
            let state = self.rounds(state);
            store(state.a, state.b, state.c, state.d, output);
        }
    }

    #[inline]
    #[cfg(feature = "cipher")]
    #[allow(clippy::cast_ptr_alignment)] // loadu/storeu support unaligned loads/stores
    pub fn apply_keystream(&self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);

        unsafe {
            let state = State {
                a: self.v0,
                b: self.v1,
                c: self.v2,
                d: iv_setup(self.iv, counter),
            };
            let state = self.rounds(state);

            for i in 0..BLOCKS {
                for (chunk, a) in output[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE]
                    .chunks_mut(0x10)
                    .zip(
                        [state.a, state.b, state.c, state.d]
                            .iter()
                            .map(|s| s.blocks[i]),
                    )
                {
                    let b = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
                    let out = _mm_xor_si128(a, b);
                    _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, out);
                }
            }
        }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn rounds(&self, mut state: State) -> State {
        let d_orig = state.d;

        for _ in 0..(R::COUNT / 2) {
            state = double_quarter_round(state);
        }

        State {
            a: state.a.add_epi32(self.v0),
            b: state.b.add_epi32(self.v1),
            c: state.c.add_epi32(self.v2),
            d: state.d.add_epi32(d_orig),
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
#[allow(clippy::cast_ptr_alignment)] // loadu supports unaligned loads
unsafe fn key_setup(key: &[u8; KEY_SIZE]) -> (StateWord, StateWord, StateWord) {
    let v0 = _mm_loadu_si128(CONSTANTS.as_ptr() as *const __m128i);
    let v1 = _mm_loadu_si128(key.as_ptr().offset(0x00) as *const __m128i);
    let v2 = _mm_loadu_si128(key.as_ptr().offset(0x10) as *const __m128i);

    (
        StateWord {
            blocks: [v0, v0, v0, v0],
        },
        StateWord {
            blocks: [v1, v1, v1, v1],
        },
        StateWord {
            blocks: [v2, v2, v2, v2],
        },
    )
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iv_setup(iv: [i32; 2], counter: u64) -> StateWord {
    let s3 = _mm_set_epi32(
        iv[0],
        iv[1],
        ((counter >> 32) & 0xffff_ffff) as i32,
        (counter & 0xffff_ffff) as i32,
    );

    StateWord {
        blocks: [
            s3,
            _mm_add_epi64(s3, _mm_set_epi64x(0, 1)),
            _mm_add_epi64(s3, _mm_set_epi64x(0, 2)),
            _mm_add_epi64(s3, _mm_set_epi64x(0, 3)),
        ],
    }
}

#[inline]
#[target_feature(enable = "avx2")]
#[allow(clippy::cast_ptr_alignment)] // storeu supports unaligned stores
unsafe fn store(v0: StateWord, v1: StateWord, v2: StateWord, v3: StateWord, output: &mut [u8]) {
    debug_assert_eq!(output.len(), BUFFER_SIZE);

    for i in 0..BLOCKS {
        for (chunk, v) in output[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE]
            .chunks_mut(0x10)
            .zip([v0, v1, v2, v3].iter().map(|s| s.blocks[i]))
        {
            _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, v);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn double_quarter_round(state: State) -> State {
    let state = add_xor_rot(state);
    cols_to_rows(add_xor_rot(rows_to_cols(state)))
}

/// The goal of this function is to transform the state words from:
/// ```text
/// [a0, a1, a2, a3]    [ 0,  1,  2,  3]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c0, c1, c2, c3]    [ 8,  9, 10, 11]
/// [d0, d1, d2, d3]    [12, 13, 14, 15]
/// ```
///
/// to:
/// ```text
/// [a0, a1, a2, a3]    [ 0,  1,  2,  3]
/// [b1, b2, b3, b0] == [ 5,  6,  7,  4]
/// [c2, c3, c0, c1]    [10, 11,  8,  9]
/// [d3, d0, d1, d2]    [15, 12, 13, 14]
/// ```
///
/// so that we can apply [`add_xor_rot`] to the resulting columns, and have it compute the
/// "diagonal rounds" (as defined in RFC 7539) in parallel. In practice, this shuffle is
/// non-optimal: the last state word to be altered in `add_xor_rot` is `b`, so the shuffle
/// blocks on the result of `b` being calculated.
///
/// We can optimize this by observing that the four quarter rounds in `add_xor_rot` are
/// data-independent: they only access a single column of the state, and thus the order of
/// the columns does not matter. We therefore instead shuffle the other three state words,
/// to obtain the following equivalent layout:
/// ```text
/// [a3, a0, a1, a2]    [ 3,  0,  1,  2]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c1, c2, c3, c0]    [ 9, 10, 11,  8]
/// [d2, d3, d0, d1]    [14, 15, 12, 13]
/// ```
///
/// See https://github.com/sneves/blake2-avx2/pull/4 for additional details. The earliest
/// known occurrence of this optimization is in floodyberry's SSE4 ChaCha code from 2014:
/// - https://github.com/floodyberry/chacha-opt/blob/0ab65cb99f5016633b652edebaf3691ceb4ff753/chacha_blocks_ssse3-64.S#L639-L643
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn rows_to_cols(state: State) -> State {
    // c = ROR256_B(c); d = ROR256_C(d); a = ROR256_D(a);
    let c = state.c.shuffle_epi32::<0b_00_11_10_01>(); // _MM_SHUFFLE(0, 3, 2, 1)
    let d = state.d.shuffle_epi32::<0b_01_00_11_10>(); // _MM_SHUFFLE(1, 0, 3, 2)
    let a = state.a.shuffle_epi32::<0b_10_01_00_11>(); // _MM_SHUFFLE(2, 1, 0, 3)

    State {
        a,
        b: state.b,
        c,
        d,
    }
}

/// The goal of this function is to transform the state words from:
/// ```text
/// [a3, a0, a1, a2]    [ 3,  0,  1,  2]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c1, c2, c3, c0]    [ 9, 10, 11,  8]
/// [d2, d3, d0, d1]    [14, 15, 12, 13]
/// ```
///
/// to:
/// ```text
/// [a0, a1, a2, a3]    [ 0,  1,  2,  3]
/// [b0, b1, b2, b3] == [ 4,  5,  6,  7]
/// [c0, c1, c2, c3]    [ 8,  9, 10, 11]
/// [d0, d1, d2, d3]    [12, 13, 14, 15]
/// ```
///
/// reversing the transformation of [`rows_to_cols`].
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cols_to_rows(state: State) -> State {
    // c = ROR256_D(c); d = ROR256_C(d); a = ROR256_B(a);
    let c = state.c.shuffle_epi32::<0b_10_01_00_11>(); // _MM_SHUFFLE(2, 1, 0, 3)
    let d = state.d.shuffle_epi32::<0b_01_00_11_10>(); // _MM_SHUFFLE(1, 0, 3, 2)
    let a = state.a.shuffle_epi32::<0b_00_11_10_01>(); // _MM_SHUFFLE(0, 3, 2, 1)

    State {
        a,
        b: state.b,
        c,
        d,
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn add_xor_rot(state: State) -> State {
    // a = ADD256_32(a,b); d = XOR256(d,a); d = ROL256_16(d);
    let a = state.a.add_epi32(state.b);
    let d = state.d.xor(a).rol_16();

    // c = ADD256_32(c,d); b = XOR256(b,c); b = ROL256_12(b);
    let c = state.c.add_epi32(d);
    let b = state.b.xor(c).rol::<12, 20>();

    // a = ADD256_32(a,b); d = XOR256(d,a); d = ROL256_8(d);
    let a = a.add_epi32(b);
    let d = d.xor(a).rol_8();

    // c = ADD256_32(c,d); b = XOR256(b,c); b = ROL256_7(b);
    let c = c.add_epi32(d);
    let b = b.xor(c).rol::<7, 25>();

    State { a, b, c, d }
}
