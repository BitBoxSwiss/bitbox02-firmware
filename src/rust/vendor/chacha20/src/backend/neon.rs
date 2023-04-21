//! The ChaCha20 core function. Defined in RFC 8439 Section 2.3.
//!
//! <https://tools.ietf.org/html/rfc8439#section-2.3>
//!
//! NEON-optimized implementation for aarch64 CPUs adapted from the SUPERCOP `dolbeau`
//! backend (public domain).

use crate::{rounds::Rounds, BLOCK_SIZE, CONSTANTS, IV_SIZE, KEY_SIZE};
use core::{convert::TryInto, marker::PhantomData};

/// Size of buffers passed to `generate` and `apply_keystream` for this backend, which
/// operates on four blocks in parallel for optimal performance.
pub(crate) const BUFFER_SIZE: usize = BLOCK_SIZE * 4;

use core::arch::aarch64::*;

/// A composite type storing four consecutive ChaCha20 words from a single block.
#[derive(Clone, Copy)]
struct StateWord(uint32x4_t);

impl StateWord {
    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert_eq!(bytes.len(), 16);

        // We read the bytes into a [u32; 4] because vld1q_u32 requires aligned memory.
        StateWord(vld1q_u32(
            [
                u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
                u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
                u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
                u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            ]
            .as_ptr(),
        ))
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn store(self, bytes: &mut [u8]) {
        debug_assert_eq!(bytes.len(), 16);

        // We write the word into a [u32; 4] because vst1q_u32 requires aligned memory.
        let mut out = [0u32; 4];
        vst1q_u32(out.as_mut_ptr(), self.0);

        for (word, chunk) in core::array::IntoIter::new(out).zip(bytes.chunks_exact_mut(4)) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "neon")]
    unsafe fn xor(self, rhs: Self) -> Self {
        StateWord(veorq_u32(self.0, rhs.0))
    }
}

/// A composite type storing the equivalent ChaCha20 word from four consecutive blocks.
#[derive(Clone, Copy)]
struct QuadWord(uint32x4_t);

impl QuadWord {
    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn from_u32(value: u32) -> Self {
        QuadWord(vdupq_n_u32(value))
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "neon")]
    unsafe fn add(self, rhs: Self) -> Self {
        QuadWord(vaddq_u32(self.0, rhs.0))
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "neon")]
    unsafe fn xor(self, rhs: Self) -> Self {
        QuadWord(veorq_u32(self.0, rhs.0))
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "neon")]
    unsafe fn rol<const BY: i32, const REST: i32>(self) -> Self {
        QuadWord(vsliq_n_u32(vshrq_n_u32(self.0, REST), self.0, BY))
    }

    #[inline]
    #[must_use]
    #[target_feature(enable = "neon")]
    unsafe fn rol_16(self) -> Self {
        QuadWord(vreinterpretq_u32_u16(vrev32q_u16(vreinterpretq_u16_u32(
            self.0,
        ))))
    }
}

/// The state required to process four blocks in parallel.
///
/// Unlike the AVX2 backend, where each state word holds a quarter of a single block, here
/// each quad word holds a specific ChaCha word from all four blocks.
struct State {
    x_0: QuadWord,
    x_1: QuadWord,
    x_2: QuadWord,
    x_3: QuadWord,
    x_4: QuadWord,
    x_5: QuadWord,
    x_6: QuadWord,
    x_7: QuadWord,
    x_8: QuadWord,
    x_9: QuadWord,
    x_10: QuadWord,
    x_11: QuadWord,
    x_12: QuadWord,
    x_13: QuadWord,
    x_14: QuadWord,
    x_15: QuadWord,
}

impl State {
    /// Stores this state in the given output.
    #[inline]
    unsafe fn store(self, output: &mut [u8]) {
        for (i, (a, b, c, d)) in core::array::IntoIter::new([
            (self.x_0, self.x_1, self.x_2, self.x_3),
            (self.x_4, self.x_5, self.x_6, self.x_7),
            (self.x_8, self.x_9, self.x_10, self.x_11),
            (self.x_12, self.x_13, self.x_14, self.x_15),
        ])
        .enumerate()
        {
            let start = 16 * i;
            let (q_0, q_1, q_2, q_3) = transpose(a, b, c, d);

            for (j, q) in core::array::IntoIter::new([q_0, q_1, q_2, q_3]).enumerate() {
                let offset = 64 * j;
                let chunk = &mut output[start + offset..start + offset + 16];
                q.store(chunk);
            }
        }
    }

    /// XORs this state into the given output.
    #[inline]
    unsafe fn xor(self, output: &mut [u8]) {
        for (i, (a, b, c, d)) in core::array::IntoIter::new([
            (self.x_0, self.x_1, self.x_2, self.x_3),
            (self.x_4, self.x_5, self.x_6, self.x_7),
            (self.x_8, self.x_9, self.x_10, self.x_11),
            (self.x_12, self.x_13, self.x_14, self.x_15),
        ])
        .enumerate()
        {
            let start = 16 * i;
            let (q_0, q_1, q_2, q_3) = transpose(a, b, c, d);

            for (j, q) in core::array::IntoIter::new([q_0, q_1, q_2, q_3]).enumerate() {
                let offset = 64 * j;
                let chunk = &mut output[start + offset..start + offset + 16];
                let m = StateWord::from_bytes(chunk);
                q.xor(m).store(chunk);
            }
        }
    }
}

/// The ChaCha20 core function (NEON-optimized implementation for aarch64).
#[derive(Clone)]
pub struct Core<R: Rounds> {
    x_0: QuadWord,
    x_1: QuadWord,
    x_2: QuadWord,
    x_3: QuadWord,
    x_4: QuadWord,
    x_5: QuadWord,
    x_6: QuadWord,
    x_7: QuadWord,
    x_8: QuadWord,
    x_9: QuadWord,
    x_10: QuadWord,
    x_11: QuadWord,
    x_14: QuadWord,
    x_15: QuadWord,
    rounds: PhantomData<R>,
}

impl<R: Rounds> Core<R> {
    /// Initialize core function with the given key size, IV, and number of rounds.
    #[inline]
    pub fn new(key: &[u8; KEY_SIZE], iv: [u8; IV_SIZE]) -> Self {
        let key = [
            u32::from_le_bytes(key[0..4].try_into().unwrap()),
            u32::from_le_bytes(key[4..8].try_into().unwrap()),
            u32::from_le_bytes(key[8..12].try_into().unwrap()),
            u32::from_le_bytes(key[12..16].try_into().unwrap()),
            u32::from_le_bytes(key[16..20].try_into().unwrap()),
            u32::from_le_bytes(key[20..24].try_into().unwrap()),
            u32::from_le_bytes(key[24..28].try_into().unwrap()),
            u32::from_le_bytes(key[28..32].try_into().unwrap()),
        ];
        let iv = [
            u32::from_le_bytes(iv[..4].try_into().unwrap()),
            u32::from_le_bytes(iv[4..].try_into().unwrap()),
        ];

        unsafe { Self::from_key_and_iv(key, iv) }
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn from_key_and_iv(key: [u32; KEY_SIZE / 4], iv: [u32; IV_SIZE / 4]) -> Self {
        Self {
            x_0: QuadWord::from_u32(CONSTANTS[0]),
            x_1: QuadWord::from_u32(CONSTANTS[1]),
            x_2: QuadWord::from_u32(CONSTANTS[2]),
            x_3: QuadWord::from_u32(CONSTANTS[3]),
            x_4: QuadWord::from_u32(key[0]),
            x_5: QuadWord::from_u32(key[1]),
            x_6: QuadWord::from_u32(key[2]),
            x_7: QuadWord::from_u32(key[3]),
            x_8: QuadWord::from_u32(key[4]),
            x_9: QuadWord::from_u32(key[5]),
            x_10: QuadWord::from_u32(key[6]),
            x_11: QuadWord::from_u32(key[7]),
            x_14: QuadWord::from_u32(iv[0]),
            x_15: QuadWord::from_u32(iv[1]),
            rounds: PhantomData,
        }
    }

    #[inline]
    pub fn generate(&self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);

        unsafe {
            let (x_12, x_13) = counter_setup(counter);
            let state = State {
                x_0: self.x_0,
                x_1: self.x_1,
                x_2: self.x_2,
                x_3: self.x_3,
                x_4: self.x_4,
                x_5: self.x_5,
                x_6: self.x_6,
                x_7: self.x_7,
                x_8: self.x_8,
                x_9: self.x_9,
                x_10: self.x_10,
                x_11: self.x_11,
                x_12,
                x_13,
                x_14: self.x_14,
                x_15: self.x_15,
            };
            let state = self.rounds(state);
            state.store(output);
        }
    }

    #[inline]
    #[cfg(feature = "cipher")]
    pub fn apply_keystream(&self, counter: u64, output: &mut [u8]) {
        debug_assert_eq!(output.len(), BUFFER_SIZE);

        unsafe {
            let (x_12, x_13) = counter_setup(counter);
            let state = State {
                x_0: self.x_0,
                x_1: self.x_1,
                x_2: self.x_2,
                x_3: self.x_3,
                x_4: self.x_4,
                x_5: self.x_5,
                x_6: self.x_6,
                x_7: self.x_7,
                x_8: self.x_8,
                x_9: self.x_9,
                x_10: self.x_10,
                x_11: self.x_11,
                x_12,
                x_13,
                x_14: self.x_14,
                x_15: self.x_15,
            };
            let state = self.rounds(state);
            state.xor(output);
        }
    }

    #[inline]
    #[target_feature(enable = "neon")]
    unsafe fn rounds(&self, mut state: State) -> State {
        let x_12_orig = state.x_12;
        let x_13_orig = state.x_13;

        for _ in 0..(R::COUNT / 2) {
            state = double_quarter_round(state);
        }

        State {
            x_0: state.x_0.add(self.x_0),
            x_1: state.x_1.add(self.x_1),
            x_2: state.x_2.add(self.x_2),
            x_3: state.x_3.add(self.x_3),
            x_4: state.x_4.add(self.x_4),
            x_5: state.x_5.add(self.x_5),
            x_6: state.x_6.add(self.x_6),
            x_7: state.x_7.add(self.x_7),
            x_8: state.x_8.add(self.x_8),
            x_9: state.x_9.add(self.x_9),
            x_10: state.x_10.add(self.x_10),
            x_11: state.x_11.add(self.x_11),
            x_12: state.x_12.add(x_12_orig),
            x_13: state.x_13.add(x_13_orig),
            x_14: state.x_14.add(self.x_14),
            x_15: state.x_15.add(self.x_15),
        }
    }
}

/// Prepares the following structure:
///
/// ```text
/// c_0 = counter + 0
/// c_1 = counter + 1
/// c_2 = counter + 2
/// c_3 = counter + 3
///
/// (
///     [lo(c_0), lo(c_1), lo(c_2), lo(c_3)],
///     [hi(c_0), hi(c_1), hi(c_2), hi(c_3)],
/// )
/// ```
#[inline]
#[target_feature(enable = "neon")]
unsafe fn counter_setup(counter: u64) -> (QuadWord, QuadWord) {
    // add_01 = [0, 1]
    // add_23 = [2, 3]
    let add_01 = vcombine_u64(vcreate_u64(0), vcreate_u64(1));
    let add_23 = vcombine_u64(vcreate_u64(2), vcreate_u64(3));

    // c = [counter, counter]
    let c = vdupq_n_u64(counter);

    // c_01 = [counter + 0, counter + 1]
    //      = [lo(c_0), hi(c_0), lo(c_1), hi(c_1)]
    // c_23 = [counter + 2, counter + 3]
    //      = [lo(c_2), hi(c_2), lo(c_3), hi(c_3)]
    let c_01 = vreinterpretq_u32_u64(vaddq_u64(c, add_01));
    let c_23 = vreinterpretq_u32_u64(vaddq_u64(c, add_23));

    (
        // [lo(c_0), lo(c_1), lo(c_2), lo(c_3)]
        QuadWord(vuzp1q_u32(c_01, c_23)),
        // [hi(c_0), hi(c_1), hi(c_2), hi(c_3)]
        QuadWord(vuzp2q_u32(c_01, c_23)),
    )
}

#[inline]
#[target_feature(enable = "neon")]
unsafe fn double_quarter_round(mut state: State) -> State {
    vec4_quarter_round(
        &mut state.x_0,
        &mut state.x_4,
        &mut state.x_8,
        &mut state.x_12,
    );
    vec4_quarter_round(
        &mut state.x_1,
        &mut state.x_5,
        &mut state.x_9,
        &mut state.x_13,
    );
    vec4_quarter_round(
        &mut state.x_2,
        &mut state.x_6,
        &mut state.x_10,
        &mut state.x_14,
    );
    vec4_quarter_round(
        &mut state.x_3,
        &mut state.x_7,
        &mut state.x_11,
        &mut state.x_15,
    );
    vec4_quarter_round(
        &mut state.x_0,
        &mut state.x_5,
        &mut state.x_10,
        &mut state.x_15,
    );
    vec4_quarter_round(
        &mut state.x_1,
        &mut state.x_6,
        &mut state.x_11,
        &mut state.x_12,
    );
    vec4_quarter_round(
        &mut state.x_2,
        &mut state.x_7,
        &mut state.x_8,
        &mut state.x_13,
    );
    vec4_quarter_round(
        &mut state.x_3,
        &mut state.x_4,
        &mut state.x_9,
        &mut state.x_14,
    );

    state
}

#[inline]
#[target_feature(enable = "neon")]
unsafe fn vec4_quarter_round(
    a: &mut QuadWord,
    b: &mut QuadWord,
    c: &mut QuadWord,
    d: &mut QuadWord,
) {
    // a += b; d ^= a; d <<<= (16, 16, 16, 16);
    *a = a.add(*b);
    *d = d.xor(*a).rol_16();

    // c += d; b ^= c; b <<<= (12, 12, 12, 12);
    *c = c.add(*d);
    *b = b.xor(*c).rol::<12, 20>();

    // a += b; d ^= a; d <<<= (8, 8, 8, 8);
    *a = a.add(*b);
    *d = d.xor(*a).rol::<8, 24>();

    // c += d; b ^= c; b <<<= (7, 7, 7, 7);
    *c = c.add(*d);
    *b = b.xor(*c).rol::<7, 25>();
}

#[inline]
#[target_feature(enable = "neon")]
unsafe fn transpose(
    a: QuadWord,
    b: QuadWord,
    c: QuadWord,
    d: QuadWord,
) -> (StateWord, StateWord, StateWord, StateWord) {
    // Input:
    //
    // a = [a_0, a_1, a_2, a_3]
    // b = [b_0, b_1, b_2, b_3]
    // c = [c_0, c_1, c_2, c_3]
    // d = [d_0, d_1, d_2, d_3]

    // t0dq = (
    //     [a_0, b_0, a_2, b_2],
    //     [a_1, b_1, a_3, b_3],
    // )
    // t1dq = (
    //     [c_0, d_0, c_2, d_2],
    //     [c_1, d_1, c_3, d_3],
    // )
    let t0dq = (vtrn1q_u32(a.0, b.0), vtrn2q_u32(a.0, b.0));
    let t1dq = (vtrn1q_u32(c.0, d.0), vtrn2q_u32(c.0, d.0));

    // Output:
    // (
    //     [a_0, b_0, c_0, d_0],
    //     [a_1, b_1, c_1, d_1],
    //     [a_2, b_2, c_2, d_2],
    //     [a_3, b_3, c_3, d_3],
    // )
    (
        StateWord(vreinterpretq_u32_u64(vcombine_u64(
            vget_low_u64(vreinterpretq_u64_u32(t0dq.0)),
            vget_low_u64(vreinterpretq_u64_u32(t1dq.0)),
        ))),
        StateWord(vreinterpretq_u32_u64(vcombine_u64(
            vget_low_u64(vreinterpretq_u64_u32(t0dq.1)),
            vget_low_u64(vreinterpretq_u64_u32(t1dq.1)),
        ))),
        StateWord(vreinterpretq_u32_u64(vcombine_u64(
            vget_high_u64(vreinterpretq_u64_u32(t0dq.0)),
            vget_high_u64(vreinterpretq_u64_u32(t1dq.0)),
        ))),
        StateWord(vreinterpretq_u32_u64(vcombine_u64(
            vget_high_u64(vreinterpretq_u64_u32(t0dq.1)),
            vget_high_u64(vreinterpretq_u64_u32(t1dq.1)),
        ))),
    )
}

#[cfg(all(test, target_feature = "neon"))]
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
        let core = Core::<R20>::new(&R_KEY, R_IV);

        unsafe {
            let (x_12, x_13) = counter_setup(R_CNT);
            let state = State {
                x_0: core.x_0,
                x_1: core.x_1,
                x_2: core.x_2,
                x_3: core.x_3,
                x_4: core.x_4,
                x_5: core.x_5,
                x_6: core.x_6,
                x_7: core.x_7,
                x_8: core.x_8,
                x_9: core.x_9,
                x_10: core.x_10,
                x_11: core.x_11,
                x_12,
                x_13,
                x_14: core.x_14,
                x_15: core.x_15,
            };

            let mut output = [0u8; BUFFER_SIZE];
            state.store(&mut output);

            let expected = [
                0x6170_7865,
                0x3320_646e,
                0x7962_2d32,
                0x6b20_6574,
                0x9972_f211,
                0xef6d_79e1,
                0x586a_dc0b,
                0x9458_011f,
                0x3f69_1992,
                0x7216_35e9,
                0x940d_d163,
                0x1134_316d,
                0xd23a_8fa8,
                0x9fe6_25b6,
                0x4aa8_962f,
                0x94bc_92f8,
            ];

            for (i, chunk) in output[..BLOCK_SIZE].chunks(4).enumerate() {
                assert_eq!(expected[i], u32::from_le_bytes(chunk.try_into().unwrap()));
            }
        }
    }

    #[test]
    fn init_and_double_round() {
        let core = Core::<R20>::new(&R_KEY, R_IV);

        unsafe {
            let (x_12, x_13) = counter_setup(R_CNT);
            let state = State {
                x_0: core.x_0,
                x_1: core.x_1,
                x_2: core.x_2,
                x_3: core.x_3,
                x_4: core.x_4,
                x_5: core.x_5,
                x_6: core.x_6,
                x_7: core.x_7,
                x_8: core.x_8,
                x_9: core.x_9,
                x_10: core.x_10,
                x_11: core.x_11,
                x_12,
                x_13,
                x_14: core.x_14,
                x_15: core.x_15,
            };

            let state = double_quarter_round(state);

            let mut output = [0u8; BUFFER_SIZE];
            state.store(&mut output);

            let expected = [
                562456049, 3130322832, 1534507163, 1938142593, 1427879055, 3727017100, 1549525649,
                2358041203, 1010155040, 657444539, 2865892668, 2826477124, 737507996, 3254278724,
                3376929372, 928763221,
            ];

            for (i, chunk) in output[..BLOCK_SIZE].chunks(4).enumerate() {
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
