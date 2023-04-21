//! The ChaCha20 core function. Defined in RFC 8439 Section 2.3.
//!
//! <https://tools.ietf.org/html/rfc8439#section-2.3>
//!
//! SSE2-optimized implementation for x86/x86-64 CPUs.

use super::autodetect::BUFFER_SIZE;
use crate::{rounds::Rounds, BLOCK_SIZE, CONSTANTS, IV_SIZE, KEY_SIZE};
use core::{convert::TryInto, marker::PhantomData};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// The ChaCha20 core function (SSE2 accelerated implementation for x86/x86_64)
// TODO(tarcieri): zeroize?
#[derive(Clone)]
pub struct Core<R: Rounds> {
    v0: __m128i,
    v1: __m128i,
    v2: __m128i,
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
        debug_assert_eq!(output.len(), BUFFER_SIZE);

        for (i, chunk) in output.chunks_exact_mut(BLOCK_SIZE).enumerate() {
            unsafe {
                let (mut v0, mut v1, mut v2) = (self.v0, self.v1, self.v2);
                let mut v3 = iv_setup(self.iv, counter.checked_add(i as u64).unwrap());
                self.rounds(&mut v0, &mut v1, &mut v2, &mut v3);
                store(v0, v1, v2, v3, chunk)
            }
        }
    }

    #[inline]
    #[cfg(feature = "cipher")]
    #[allow(clippy::cast_ptr_alignment)] // loadu/storeu support unaligned loads/stores
    pub fn apply_keystream(&self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);

        for (i, chunk) in output.chunks_exact_mut(BLOCK_SIZE).enumerate() {
            unsafe {
                let (mut v0, mut v1, mut v2) = (self.v0, self.v1, self.v2);
                let mut v3 = iv_setup(self.iv, counter.checked_add(i as u64).unwrap());
                self.rounds(&mut v0, &mut v1, &mut v2, &mut v3);

                for (ch, a) in chunk.chunks_exact_mut(0x10).zip(&[v0, v1, v2, v3]) {
                    let b = _mm_loadu_si128(ch.as_ptr() as *const __m128i);
                    let out = _mm_xor_si128(*a, b);
                    _mm_storeu_si128(ch.as_mut_ptr() as *mut __m128i, out);
                }
            }
        }
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn rounds(
        &self,
        v0: &mut __m128i,
        v1: &mut __m128i,
        v2: &mut __m128i,
        v3: &mut __m128i,
    ) {
        let v3_orig = *v3;

        for _ in 0..(R::COUNT / 2) {
            double_quarter_round(v0, v1, v2, v3);
        }

        *v0 = _mm_add_epi32(*v0, self.v0);
        *v1 = _mm_add_epi32(*v1, self.v1);
        *v2 = _mm_add_epi32(*v2, self.v2);
        *v3 = _mm_add_epi32(*v3, v3_orig);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
#[allow(clippy::cast_ptr_alignment)] // loadu supports unaligned loads
unsafe fn key_setup(key: &[u8; KEY_SIZE]) -> (__m128i, __m128i, __m128i) {
    let v0 = _mm_loadu_si128(CONSTANTS.as_ptr() as *const __m128i);
    let v1 = _mm_loadu_si128(key.as_ptr().offset(0x00) as *const __m128i);
    let v2 = _mm_loadu_si128(key.as_ptr().offset(0x10) as *const __m128i);
    (v0, v1, v2)
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn iv_setup(iv: [i32; 2], counter: u64) -> __m128i {
    _mm_set_epi32(
        iv[0],
        iv[1],
        ((counter >> 32) & 0xffff_ffff) as i32,
        (counter & 0xffff_ffff) as i32,
    )
}

#[inline]
#[target_feature(enable = "sse2")]
#[allow(clippy::cast_ptr_alignment)] // storeu supports unaligned stores
unsafe fn store(v0: __m128i, v1: __m128i, v2: __m128i, v3: __m128i, output: &mut [u8]) {
    _mm_storeu_si128(output.as_mut_ptr().offset(0x00) as *mut __m128i, v0);
    _mm_storeu_si128(output.as_mut_ptr().offset(0x10) as *mut __m128i, v1);
    _mm_storeu_si128(output.as_mut_ptr().offset(0x20) as *mut __m128i, v2);
    _mm_storeu_si128(output.as_mut_ptr().offset(0x30) as *mut __m128i, v3);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn double_quarter_round(a: &mut __m128i, b: &mut __m128i, c: &mut __m128i, d: &mut __m128i) {
    add_xor_rot(a, b, c, d);
    rows_to_cols(a, b, c, d);
    add_xor_rot(a, b, c, d);
    cols_to_rows(a, b, c, d);
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
#[target_feature(enable = "sse2")]
unsafe fn rows_to_cols(a: &mut __m128i, _b: &mut __m128i, c: &mut __m128i, d: &mut __m128i) {
    // c >>>= 32; d >>>= 64; a >>>= 96;
    *c = _mm_shuffle_epi32(*c, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
    *d = _mm_shuffle_epi32(*d, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
    *a = _mm_shuffle_epi32(*a, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
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
#[target_feature(enable = "sse2")]
unsafe fn cols_to_rows(a: &mut __m128i, _b: &mut __m128i, c: &mut __m128i, d: &mut __m128i) {
    // c <<<= 32; d <<<= 64; a <<<= 96;
    *c = _mm_shuffle_epi32(*c, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
    *d = _mm_shuffle_epi32(*d, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
    *a = _mm_shuffle_epi32(*a, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn add_xor_rot(a: &mut __m128i, b: &mut __m128i, c: &mut __m128i, d: &mut __m128i) {
    // a += b; d ^= a; d <<<= (16, 16, 16, 16);
    *a = _mm_add_epi32(*a, *b);
    *d = _mm_xor_si128(*d, *a);
    *d = _mm_xor_si128(_mm_slli_epi32(*d, 16), _mm_srli_epi32(*d, 16));

    // c += d; b ^= c; b <<<= (12, 12, 12, 12);
    *c = _mm_add_epi32(*c, *d);
    *b = _mm_xor_si128(*b, *c);
    *b = _mm_xor_si128(_mm_slli_epi32(*b, 12), _mm_srli_epi32(*b, 20));

    // a += b; d ^= a; d <<<= (8, 8, 8, 8);
    *a = _mm_add_epi32(*a, *b);
    *d = _mm_xor_si128(*d, *a);
    *d = _mm_xor_si128(_mm_slli_epi32(*d, 8), _mm_srli_epi32(*d, 24));

    // c += d; b ^= c; b <<<= (7, 7, 7, 7);
    *c = _mm_add_epi32(*c, *d);
    *b = _mm_xor_si128(*b, *c);
    *b = _mm_xor_si128(_mm_slli_epi32(*b, 7), _mm_srli_epi32(*b, 25));
}

#[cfg(all(test, target_feature = "sse2"))]
mod tests {
    use super::*;
    use crate::rounds::R20;
    use crate::{backend::soft, BLOCK_SIZE};
    use core::convert::TryInto;

    // random inputs for testing
    const R_CNT: u64 = 0x9fe625b6d23a8fa8u64;
    const R_IV: [u8; IV_SIZE] = [0x2f, 0x96, 0xa8, 0x4a, 0xf8, 0x92, 0xbc, 0x94];
    const R_KEY: [u8; KEY_SIZE] = [
        0x11, 0xf2, 0x72, 0x99, 0xe1, 0x79, 0x6d, 0xef, 0xb, 0xdc, 0x6a, 0x58, 0x1f, 0x1, 0x58,
        0x94, 0x92, 0x19, 0x69, 0x3f, 0xe9, 0x35, 0x16, 0x72, 0x63, 0xd1, 0xd, 0x94, 0x6d, 0x31,
        0x34, 0x11,
    ];

    #[test]
    fn init_and_store() {
        unsafe {
            let (v0, v1, v2) = key_setup(&R_KEY);

            let v3 = iv_setup(
                [
                    i32::from_le_bytes(R_IV[4..].try_into().unwrap()),
                    i32::from_le_bytes(R_IV[..4].try_into().unwrap()),
                ],
                R_CNT,
            );

            let vs = [v0, v1, v2, v3];

            let mut output = [0u8; BLOCK_SIZE];
            store(vs[0], vs[1], vs[2], vs[3], &mut output);

            let expected = [
                1634760805, 857760878, 2036477234, 1797285236, 2574447121, 4016929249, 1483398155,
                2488795423, 1063852434, 1914058217, 2483933539, 288633197, 3527053224, 2682660278,
                1252562479, 2495386360,
            ];

            for (i, chunk) in output.chunks(4).enumerate() {
                assert_eq!(expected[i], u32::from_le_bytes(chunk.try_into().unwrap()));
            }
        }
    }

    #[test]
    fn init_and_double_round() {
        unsafe {
            let (mut v0, mut v1, mut v2) = key_setup(&R_KEY);

            let mut v3 = iv_setup(
                [
                    i32::from_le_bytes(R_IV[4..].try_into().unwrap()),
                    i32::from_le_bytes(R_IV[..4].try_into().unwrap()),
                ],
                R_CNT,
            );

            double_quarter_round(&mut v0, &mut v1, &mut v2, &mut v3);

            let mut output = [0u8; BLOCK_SIZE];
            store(v0, v1, v2, v3, &mut output);

            let expected = [
                562456049, 3130322832, 1534507163, 1938142593, 1427879055, 3727017100, 1549525649,
                2358041203, 1010155040, 657444539, 2865892668, 2826477124, 737507996, 3254278724,
                3376929372, 928763221,
            ];

            for (i, chunk) in output.chunks(4).enumerate() {
                assert_eq!(expected[i], u32::from_le_bytes(chunk.try_into().unwrap()));
            }
        }
    }

    #[test]
    fn generate_vs_scalar_impl() {
        let mut soft_result = [0u8; soft::BUFFER_SIZE];
        soft::Core::<R20>::new(&R_KEY, R_IV).generate(R_CNT, &mut soft_result);

        let mut simd_result = [0u8; BUFFER_SIZE];
        Core::<R20>::new(&R_KEY, R_IV).generate(R_CNT, &mut simd_result);

        assert_eq!(&soft_result[..], &simd_result[..soft::BUFFER_SIZE])
    }
}
