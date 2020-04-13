//! The ChaCha20 block function. Defined in RFC 8439 Section 2.3.
//!
//! <https://tools.ietf.org/html/rfc8439#section-2.3>
//!
//! AVX2-optimized implementation for x86/x86-64 CPUs adapted from the SUPERCOP
//! `goll_gueron` backend (public domain) described in:
//!
//! Goll, M., and Gueron,S.: Vectorization of ChaCha Stream Cipher. Cryptology ePrint Archive,
//! Report 2013/759, November, 2013, <https://eprint.iacr.org/2013/759.pdf>

use crate::{rounds::Rounds, BLOCK_SIZE, CONSTANTS, IV_SIZE, KEY_SIZE};
use core::{convert::TryInto, marker::PhantomData};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Size of buffers passed to `generate` and `apply_keystream` for this
/// backend, which operates on two blocks in parallel for optimal performance.
pub(crate) const BUFFER_SIZE: usize = BLOCK_SIZE * 2;

/// The ChaCha20 block function (AVX2 accelerated implementation for x86/x86_64)
// TODO(tarcieri): zeroize?
#[derive(Clone)]
pub(crate) struct Block<R: Rounds> {
    v0: __m256i,
    v1: __m256i,
    v2: __m256i,
    iv: [i32; 2],
    rounds: PhantomData<R>,
}

impl<R: Rounds> Block<R> {
    /// Initialize block function with the given key size, IV, and number of rounds
    #[inline]
    pub(crate) fn new(key: &[u8; KEY_SIZE], iv: [u8; IV_SIZE]) -> Self {
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
    pub(crate) fn generate(&self, counter: u64, output: &mut [u8]) {
        unsafe {
            let (mut v0, mut v1, mut v2) = (self.v0, self.v1, self.v2);
            let mut v3 = iv_setup(self.iv, counter);
            self.rounds(&mut v0, &mut v1, &mut v2, &mut v3);
            store(v0, v1, v2, v3, output);
        }
    }

    #[inline]
    #[cfg(feature = "stream-cipher")]
    #[allow(clippy::cast_ptr_alignment)] // loadu/storeu support unaligned loads/stores
    pub(crate) fn apply_keystream(&self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);

        unsafe {
            let (mut v0, mut v1, mut v2) = (self.v0, self.v1, self.v2);
            let mut v3 = iv_setup(self.iv, counter);
            self.rounds(&mut v0, &mut v1, &mut v2, &mut v3);

            for (chunk, a) in output[..BLOCK_SIZE].chunks_mut(0x10).zip(&[v0, v1, v2, v3]) {
                let b = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
                let out = _mm_xor_si128(_mm256_castsi256_si128(*a), b);
                _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, out);
            }

            for (chunk, a) in output[BLOCK_SIZE..].chunks_mut(0x10).zip(&[v0, v1, v2, v3]) {
                let b = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
                let out = _mm_xor_si128(_mm256_extractf128_si256(*a, 1), b);
                _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, out);
            }
        }
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn rounds(
        &self,
        v0: &mut __m256i,
        v1: &mut __m256i,
        v2: &mut __m256i,
        v3: &mut __m256i,
    ) {
        let v3_orig = *v3;

        for _ in 0..(R::COUNT / 2) {
            double_quarter_round(v0, v1, v2, v3);
        }

        *v0 = _mm256_add_epi32(*v0, self.v0);
        *v1 = _mm256_add_epi32(*v1, self.v1);
        *v2 = _mm256_add_epi32(*v2, self.v2);
        *v3 = _mm256_add_epi32(*v3, v3_orig);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
#[allow(clippy::cast_ptr_alignment)] // loadu supports unaligned loads
unsafe fn key_setup(key: &[u8; KEY_SIZE]) -> (__m256i, __m256i, __m256i) {
    let v0 = _mm_loadu_si128(CONSTANTS.as_ptr() as *const __m128i);
    let v1 = _mm_loadu_si128(key.as_ptr().offset(0x00) as *const __m128i);
    let v2 = _mm_loadu_si128(key.as_ptr().offset(0x10) as *const __m128i);

    (
        _mm256_broadcastsi128_si256(v0),
        _mm256_broadcastsi128_si256(v1),
        _mm256_broadcastsi128_si256(v2),
    )
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iv_setup(iv: [i32; 2], counter: u64) -> __m256i {
    let s3 = _mm_set_epi32(
        iv[0],
        iv[1],
        ((counter >> 32) & 0xffff_ffff) as i32,
        (counter & 0xffff_ffff) as i32,
    );

    _mm256_add_epi64(
        _mm256_broadcastsi128_si256(s3),
        _mm256_set_epi64x(0, 1, 0, 0),
    )
}

#[inline]
#[target_feature(enable = "avx2")]
#[allow(clippy::cast_ptr_alignment)] // storeu supports unaligned stores
unsafe fn store(v0: __m256i, v1: __m256i, v2: __m256i, v3: __m256i, output: &mut [u8]) {
    debug_assert_eq!(output.len(), BUFFER_SIZE);

    for (chunk, v) in output[..BLOCK_SIZE].chunks_mut(0x10).zip(&[v0, v1, v2, v3]) {
        _mm_storeu_si128(
            chunk.as_mut_ptr() as *mut __m128i,
            _mm256_castsi256_si128(*v),
        );
    }

    for (chunk, v) in output[BLOCK_SIZE..].chunks_mut(0x10).zip(&[v0, v1, v2, v3]) {
        _mm_storeu_si128(
            chunk.as_mut_ptr() as *mut __m128i,
            _mm256_extractf128_si256(*v, 1),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn double_quarter_round(
    v0: &mut __m256i,
    v1: &mut __m256i,
    v2: &mut __m256i,
    v3: &mut __m256i,
) {
    add_xor_rot(v0, v1, v2, v3);
    rows_to_cols(v0, v1, v2, v3);
    add_xor_rot(v0, v1, v2, v3);
    cols_to_rows(v0, v1, v2, v3);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn rows_to_cols(_v0: &mut __m256i, v1: &mut __m256i, v2: &mut __m256i, v3: &mut __m256i) {
    // b = ROR256_V1(b); c = ROR256_V2(c); d = ROR256_V3(d);
    *v1 = _mm256_shuffle_epi32(*v1, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
    *v2 = _mm256_shuffle_epi32(*v2, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
    *v3 = _mm256_shuffle_epi32(*v3, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cols_to_rows(_v0: &mut __m256i, v1: &mut __m256i, v2: &mut __m256i, v3: &mut __m256i) {
    // b = ROR256_V3(b); c = ROR256_V2(c); d = ROR256_V1(d);
    *v1 = _mm256_shuffle_epi32(*v1, 0b_10_01_00_11); // _MM_SHUFFLE(2, 1, 0, 3)
    *v2 = _mm256_shuffle_epi32(*v2, 0b_01_00_11_10); // _MM_SHUFFLE(1, 0, 3, 2)
    *v3 = _mm256_shuffle_epi32(*v3, 0b_00_11_10_01); // _MM_SHUFFLE(0, 3, 2, 1)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn add_xor_rot(v0: &mut __m256i, v1: &mut __m256i, v2: &mut __m256i, v3: &mut __m256i) {
    // a = ADD256_32(a,b); d = XOR256(d,a); d = ROL256_16(d);
    *v0 = _mm256_add_epi32(*v0, *v1);
    *v3 = _mm256_xor_si256(*v3, *v0);
    *v3 = _mm256_shuffle_epi8(
        *v3,
        _mm256_set_epi8(
            13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8, 11, 10, 5,
            4, 7, 6, 1, 0, 3, 2,
        ),
    );

    // c = ADD256_32(c,d); b = XOR256(b,c); b = ROL256_12(b);
    *v2 = _mm256_add_epi32(*v2, *v3);
    *v1 = _mm256_xor_si256(*v1, *v2);
    *v1 = _mm256_xor_si256(_mm256_slli_epi32(*v1, 12), _mm256_srli_epi32(*v1, 20));

    // a = ADD256_32(a,b); d = XOR256(d,a); d = ROL256_8(d);
    *v0 = _mm256_add_epi32(*v0, *v1);
    *v3 = _mm256_xor_si256(*v3, *v0);
    *v3 = _mm256_shuffle_epi8(
        *v3,
        _mm256_set_epi8(
            14, 13, 12, 15, 10, 9, 8, 11, 6, 5, 4, 7, 2, 1, 0, 3, 14, 13, 12, 15, 10, 9, 8, 11, 6,
            5, 4, 7, 2, 1, 0, 3,
        ),
    );

    // c = ADD256_32(c,d); b = XOR256(b,c); b = ROL256_7(b);
    *v2 = _mm256_add_epi32(*v2, *v3);
    *v1 = _mm256_xor_si256(*v1, *v2);
    *v1 = _mm256_xor_si256(_mm256_slli_epi32(*v1, 7), _mm256_srli_epi32(*v1, 25));
}
