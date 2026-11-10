//! Multiplication between boxed integers in Montgomery form (i.e. Montgomery multiplication).
//!
//! Some parts adapted from `monty.rs` in `num-bigint`:
//! <https://github.com/rust-num/num-bigint/blob/2cea7f4/src/biguint/monty.rs>
//!
//! Originally (c) 2014 The Rust Project Developers, dual licensed Apache 2.0+MIT.

use super::{BoxedMontyForm, BoxedMontyParams};
use crate::{
    AmmMultiplier, BoxedUint, Limb, MontyMultiplier, Mul, MulAssign, Square, SquareAssign,
    WideWord, Word, Zero,
    modular::mul::{almost_montgomery_mul, montgomery_multiply_inner},
    word,
};
use alloc::boxed::Box;

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

impl BoxedMontyForm {
    /// Multiplies by `rhs`.
    #[must_use]
    pub fn mul(&self, rhs: &Self) -> Self {
        debug_assert_eq!(&self.params, &rhs.params);
        let mut out = BoxedUint::zero_with_precision(self.bits_precision());
        montgomery_mul(
            &self.montgomery_form,
            &rhs.montgomery_form,
            &mut out,
            self.params.modulus(),
            self.params.mod_neg_inv(),
        );
        Self {
            montgomery_form: out,
            params: self.params.clone(),
        }
    }

    /// Computes the (reduced) square.
    #[must_use]
    pub fn square(&self) -> Self {
        if BoxedMontyMultiplier::has_dedicated_square(self.montgomery_form.nlimbs()) {
            let mut out = self.clone();
            BoxedMontyMultiplier::from(&self.params).square_assign(&mut out.montgomery_form);
            return out;
        }

        let mut out = BoxedUint::zero_with_precision(self.bits_precision());
        montgomery_mul(
            &self.montgomery_form,
            &self.montgomery_form,
            &mut out,
            self.params.modulus(),
            self.params.mod_neg_inv(),
        );
        Self {
            montgomery_form: out,
            params: self.params.clone(),
        }
    }
}

impl Mul<&BoxedMontyForm> for &BoxedMontyForm {
    type Output = BoxedMontyForm;
    fn mul(self, rhs: &BoxedMontyForm) -> BoxedMontyForm {
        self.mul(rhs)
    }
}

impl Mul<BoxedMontyForm> for &BoxedMontyForm {
    type Output = BoxedMontyForm;
    #[allow(clippy::op_ref)]
    fn mul(self, rhs: BoxedMontyForm) -> BoxedMontyForm {
        self * &rhs
    }
}

impl Mul<&BoxedMontyForm> for BoxedMontyForm {
    type Output = BoxedMontyForm;
    #[allow(clippy::op_ref)]
    fn mul(self, rhs: &BoxedMontyForm) -> BoxedMontyForm {
        &self * rhs
    }
}

impl Mul<BoxedMontyForm> for BoxedMontyForm {
    type Output = BoxedMontyForm;
    fn mul(self, rhs: BoxedMontyForm) -> BoxedMontyForm {
        &self * &rhs
    }
}

impl MulAssign<BoxedMontyForm> for BoxedMontyForm {
    fn mul_assign(&mut self, rhs: BoxedMontyForm) {
        Self::mul_assign(self, &rhs);
    }
}

impl MulAssign<&BoxedMontyForm> for BoxedMontyForm {
    fn mul_assign(&mut self, rhs: &BoxedMontyForm) {
        *self = Self::mul(self, rhs);
    }
}

impl Square for BoxedMontyForm {
    fn square(&self) -> Self {
        BoxedMontyForm::square(self)
    }
}

impl SquareAssign for BoxedMontyForm {
    fn square_assign(&mut self) {
        *self = Self::square(self);
    }
}

/// Montgomery multiplier with a pre-allocated internal buffer to avoid additional allocations.
#[derive(Debug, Clone)]
pub struct BoxedMontyMultiplier<'a> {
    product: BoxedUint,
    square: Box<[Limb]>,
    modulus: &'a BoxedUint,
    mod_neg_inv: Limb,
}

impl<'a> From<&'a BoxedMontyParams> for BoxedMontyMultiplier<'a> {
    #[inline]
    fn from(params: &'a BoxedMontyParams) -> BoxedMontyMultiplier<'a> {
        BoxedMontyMultiplier::new(params.modulus(), params.mod_neg_inv())
    }
}

impl<'a> MontyMultiplier<'a> for BoxedMontyMultiplier<'a> {
    type Monty = BoxedMontyForm;

    /// Performs a Montgomery multiplication, assigning a fully reduced result to `lhs`.
    #[inline]
    fn mul_assign(&mut self, lhs: &mut BoxedMontyForm, rhs: &BoxedMontyForm) {
        self.mul_assign(&mut lhs.montgomery_form, &rhs.montgomery_form);
    }

    /// Performs a Montgomery squaring, assigning a fully reduced result to `lhs`.
    #[inline]
    fn square_assign(&mut self, lhs: &mut BoxedMontyForm) {
        self.square_assign(&mut lhs.montgomery_form);
    }
}

impl<'a> AmmMultiplier<'a> for BoxedMontyMultiplier<'a> {
    #[inline]
    fn mul_amm_assign(&mut self, a: &mut BoxedUint, b: &BoxedUint) {
        self.mul_amm_assign(a, b);
    }

    #[inline]
    fn square_amm_assign(&mut self, a: &mut BoxedUint) {
        self.square_amm_assign(a);
    }
}

impl<'a> BoxedMontyMultiplier<'a> {
    /// Sizes supported by the dedicated squaring kernel.
    const fn has_dedicated_square(nlimbs: usize) -> bool {
        Limb::BITS == 64 && matches!(nlimbs, 24 | 27 | 32 | 35)
    }

    /// Create a new Montgomery multiplier.
    pub(super) fn new(modulus: &'a BoxedUint, mod_neg_inv: Limb) -> Self {
        Self {
            product: BoxedUint::zero_with_precision(modulus.bits_precision()),
            square: if Self::has_dedicated_square(modulus.nlimbs()) {
                vec![Limb::ZERO; modulus.nlimbs() * 2].into_boxed_slice()
            } else {
                Box::default()
            },
            modulus,
            mod_neg_inv,
        }
    }

    /// Perform a Montgomery multiplication, assigning a fully reduced result to `a`.
    pub(super) fn mul_assign(&mut self, a: &mut BoxedUint, b: &BoxedUint) {
        debug_assert_eq!(a.bits_precision(), self.modulus.bits_precision());
        debug_assert_eq!(b.bits_precision(), self.modulus.bits_precision());

        self.product.set_zero();
        montgomery_mul(a, b, &mut self.product, self.modulus, self.mod_neg_inv);
        a.limbs.copy_from_slice(&self.product.limbs);
        debug_assert!(*a < self.modulus);
    }

    /// Perform a Montgomery squaring, assigning a fully reduced result to `a`.
    pub(super) fn square_assign(&mut self, a: &mut BoxedUint) {
        debug_assert_eq!(a.bits_precision(), self.modulus.bits_precision());

        if self.square.is_empty() {
            self.product.set_zero();
            montgomery_mul(a, a, &mut self.product, self.modulus, self.mod_neg_inv);
            a.limbs.copy_from_slice(&self.product.limbs);
        } else {
            let carry = self.square_inner(a);
            a.sub_assign_mod_with_carry(carry, self.modulus, self.modulus);
        }

        debug_assert!(*a < self.modulus);
    }

    /// Perform an "Almost Montgomery Multiplication", assigning the product to `a`.
    ///
    /// NOTE: the resulting output will be reduced to the *bit length* of the modulus, but not fully reduced and may
    /// exceed the modulus. A final reduction is required to ensure AMM results are fully reduced, and should not be
    /// exposed outside the internals of this crate.
    pub(super) fn mul_amm_assign(&mut self, a: &mut BoxedUint, b: &BoxedUint) {
        debug_assert_eq!(a.bits_precision(), self.modulus.bits_precision());
        debug_assert_eq!(b.bits_precision(), self.modulus.bits_precision());

        self.product.set_zero();
        almost_montgomery_mul(
            a.as_uint_ref(),
            b.as_uint_ref(),
            self.product.as_mut_uint_ref(),
            self.modulus.as_uint_ref(),
            self.mod_neg_inv,
        );
        a.limbs.copy_from_slice(&self.product.limbs);
    }

    /// Perform a squaring using "Almost Montgomery Multiplication", assigning the result to `a`.
    ///
    /// NOTE: the resulting output will be reduced to the *bit length* of the modulus, but not fully reduced and may
    /// exceed the modulus. A final reduction is required to ensure AMM results are fully reduced, and should not be
    /// exposed outside the internals of this crate.
    pub(super) fn square_amm_assign(&mut self, a: &mut BoxedUint) {
        debug_assert_eq!(a.bits_precision(), self.modulus.bits_precision());

        if self.square.is_empty() {
            self.product.set_zero();
            almost_montgomery_mul(
                a.as_uint_ref(),
                a.as_uint_ref(),
                self.product.as_mut_uint_ref(),
                self.modulus.as_uint_ref(),
                self.mod_neg_inv,
            );
            a.limbs.copy_from_slice(&self.product.limbs);
        } else {
            let carry = self.square_inner(a);
            a.as_mut_uint_ref().conditional_borrowing_sub_assign(
                self.modulus.as_uint_ref(),
                word::choice_from_lsb(carry.0),
            );
        }
    }

    /// Use dedicated squaring for the ordinary and blinded CRT widths used by RSA-3072 and RSA-4096
    /// on 64-bit targets.
    ///
    /// With `R = 2^precision` and `a < R`, the unreduced result is below `R + modulus`.
    ///
    /// The returned carry is its bit above the storage precision. AMM callers subtract the modulus
    /// only on overflow; fully reduced callers also compare to the modulus.
    fn square_inner(&mut self, a: &mut BoxedUint) -> Limb {
        match self.modulus.nlimbs() {
            24 => self.square_inner_sized::<24>(a),
            27 => self.square_inner_sized::<27>(a),
            32 => self.square_inner_sized::<32>(a),
            35 => self.square_inner_sized::<35>(a),
            _ => unreachable!("unsupported dedicated squaring precision"),
        }
    }

    /// Square into reusable scratch space, then fuse pairs of Montgomery cancellation rows.
    ///
    /// The public limb count is a compile-time constant to eliminate bounds checks and enable loop
    /// optimization at these sizes.
    #[allow(clippy::cast_possible_truncation, reason = "TODO")]
    fn square_inner_sized<const N: usize>(&mut self, a: &mut BoxedUint) -> Limb {
        let limbs: &[Limb; N] = a.as_limbs().try_into().expect("input precision mismatch");
        let modulus: &[Limb; N] = self
            .modulus
            .as_limbs()
            .try_into()
            .expect("modulus precision mismatch");
        assert_eq!(self.square.len(), 2 * N);
        self.square.fill(Limb::ZERO);

        // Compute each off-diagonal product once. The carry goes into a limb that has not yet been
        // written by a previous row.
        for i in 1..limbs.len() {
            let mut carry = Limb::ZERO;
            for (out, rhs) in self.square[i..2 * i].iter_mut().zip(&limbs[..i]) {
                (*out, carry) = limbs[i].carrying_mul_add(*rhs, *out, carry);
            }
            self.square[2 * i] = carry;
        }

        // Double the off-diagonal terms and add the diagonal. Keeping pairs together avoids a
        // parity branch for each output limb.
        let mut carry = Limb::ZERO;
        let mut hi_bit = Limb::ZERO;
        for (pair, limb) in self.square.chunks_exact_mut(2).zip(limbs) {
            let low = pair[0].shl(1).bitor(hi_bit);
            let high = pair[1].shl(1).bitor(pair[0].shr(Limb::HI_BIT));
            hi_bit = pair[1].shr(Limb::HI_BIT);
            (pair[0], carry) = limb.carrying_mul_add(*limb, low, carry);
            (pair[1], carry) = high.overflowing_add(carry);
        }

        // Cancel two low limbs per pass, combining two independent products per column.
        //
        // Write B = 2^Word::BITS. The carry is at most 2*B - 2: a <= B^2 - B and b <= B^2 - 1,
        // hence floor((a+b)/B) <= 2*B - 2.
        //
        // The initial carry has the same bound: second <= B^2 - 1 and product <= B^2 - 2*B + 1.
        // Thus every product-plus-carry fits `WideWord`.
        let nlimbs = N;
        let mut meta_carry = Limb::ZERO;
        let mut i = 0;

        while i + 1 < nlimbs {
            let window = &mut self.square[i..i + nlimbs + 2];
            let q0 = window[0].0.wrapping_mul(self.mod_neg_inv.0);

            // q0 cancels the first limb. Include that row's carry in the next
            // column before choosing q1, which cancels the second limb.
            let first =
                WideWord::from(q0) * WideWord::from(modulus[0].0) + WideWord::from(window[0].0);
            let second = WideWord::from(q0) * WideWord::from(modulus[1].0)
                + WideWord::from(window[1].0)
                + (first >> Word::BITS);

            let q1 = (second as Word).wrapping_mul(self.mod_neg_inv.0);
            let product = WideWord::from(q1) * WideWord::from(modulus[0].0);
            let low = WideWord::from(second as Word) + WideWord::from(product as Word);
            let mut carry = (second >> Word::BITS) + (product >> Word::BITS) + (low >> Word::BITS);

            for j in 2..nlimbs {
                let a =
                    WideWord::from(q0) * WideWord::from(modulus[j].0) + WideWord::from(window[j].0);
                let b = WideWord::from(q1) * WideWord::from(modulus[j - 1].0) + carry;
                let low = WideWord::from(a as Word) + WideWord::from(b as Word);
                window[j] = Limb(low as Word);
                carry = (a >> Word::BITS) + (b >> Word::BITS) + (low >> Word::BITS);
            }

            // Only the q1 row remains in this column. The previous pass's high carry belongs here,
            // at the end of the q0 row. The next column has no product term and leaves at most one
            // overflow bit.
            let product = WideWord::from(q1) * WideWord::from(modulus[nlimbs - 1].0) + carry;

            let low = WideWord::from(product as Word)
                + WideWord::from(window[nlimbs].0)
                + WideWord::from(meta_carry.0);
            window[nlimbs] = Limb(low as Word);

            let high = WideWord::from(window[nlimbs + 1].0)
                + (product >> Word::BITS)
                + (low >> Word::BITS);

            window[nlimbs + 1] = Limb(high as Word);
            meta_carry = Limb((high >> Word::BITS) as Word);
            i += 2;
        }

        if i < nlimbs {
            let (lower, upper) = self.square[i..].split_at_mut(nlimbs);
            let u = lower[0].wrapping_mul(self.mod_neg_inv);
            let (_, mut carry) = u.carrying_mul_add(modulus[0], lower[0], Limb::ZERO);
            for (limb, m) in lower[1..].iter_mut().zip(&modulus[1..]) {
                (*limb, carry) = u.carrying_mul_add(*m, *limb, carry);
            }
            (upper[0], meta_carry) = upper[0].carrying_add(carry, meta_carry);
        }

        a.limbs.copy_from_slice(&self.square[nlimbs..]);
        meta_carry
    }
}

#[cfg(feature = "zeroize")]
impl Drop for BoxedMontyMultiplier<'_> {
    fn drop(&mut self) {
        self.product.zeroize();
        self.square.zeroize();
    }
}

/// Computes Montgomery multiplication of `x` and `y` into `out`, that is
/// `out = x * y * 2^(-n*W) mod m` assuming `k = -1/m mod 2^W`,
/// where `W` is the bit size of the limb, and `n * W` is the full bit size of the integer.
///
/// NOTE: `out` is assumed to be pre-zeroized.
#[inline]
pub(crate) fn montgomery_mul(
    x: &BoxedUint,
    y: &BoxedUint,
    out: &mut BoxedUint,
    modulus: &BoxedUint,
    mod_neg_inv: Limb,
) {
    let carry = montgomery_multiply_inner(
        x.as_limbs(),
        y.as_limbs(),
        out.as_mut_limbs(),
        modulus.as_limbs(),
        mod_neg_inv,
    );
    out.sub_assign_mod_with_carry(carry, modulus, modulus);
}

#[cfg(test)]
mod tests {
    use super::{BoxedMontyForm, BoxedMontyMultiplier, BoxedMontyParams, BoxedUint};
    use crate::{Limb, SquareAssign};
    use num_bigint::BigUint;
    use rand_core::{Rng, SeedableRng};

    /// Regression test for RustCrypto/crypto-bigint#441
    #[test]
    fn square() {
        let x = 0x20u128;
        let modulus = 0xB44677037A7DBDE04814256570DCBD8Du128;

        let boxed_modulus = BoxedUint::from(modulus);
        let boxed_params = BoxedMontyParams::new(boxed_modulus.to_odd().unwrap());
        let boxed_monty = BoxedMontyForm::new(BoxedUint::from(x), &boxed_params);
        let boxed_square = boxed_monty.square();

        // TODO(tarcieri): test for correct output
        assert!(boxed_square.as_montgomery() < boxed_square.params().modulus());

        // Check mul_assign
        let mut boxed_mut = boxed_monty.clone();
        boxed_mut *= &boxed_monty;
        assert_eq!(boxed_mut, boxed_square);

        // Check square_assign
        let mut boxed_mut = boxed_monty.clone();
        boxed_mut.square_assign();
        assert_eq!(boxed_mut, boxed_square);
    }

    /// Exercise full-width carries, unreduced AMM inputs, short moduli, and both sides of the
    /// Karatsuba thresholds, including RSA's widened CRT operands.
    #[test]
    #[allow(clippy::integer_division_remainder_used, reason = "test oracle")]
    fn multiplier_square() {
        let mut rng = chacha20::ChaCha8Rng::from_seed([42; 32]);

        for nlimbs in [1, 2, 3, 4, 8, 16, 17, 19, 24, 27, 31, 32, 33, 35, 64, 65] {
            let bits = u32::try_from(nlimbs).unwrap() * Limb::BITS;

            for kind in 0..20 {
                let mut bytes = vec![0; nlimbs * Limb::BYTES];
                rng.fill_bytes(&mut bytes);

                match kind {
                    0 | 1 => {
                        bytes.fill(0);
                        bytes[0] = if kind == 0 { 1 } else { 3 };
                    }
                    2 => bytes.fill(0xff),
                    3 => bytes[nlimbs * Limb::BYTES / 2..].fill(0),
                    _ => (),
                }

                bytes[0] |= 1;

                let modulus = BoxedUint::from_le_slice(&bytes, bits).unwrap();
                let params = BoxedMontyParams::new(modulus.to_odd().unwrap());
                let m = BigUint::from_bytes_le(&bytes);
                let r = BigUint::from(1u8) << bits;
                let mut multiplier = BoxedMontyMultiplier::from(&params);
                let mut reference = BoxedMontyMultiplier::from(&params);

                for input in 0..12 {
                    rng.fill_bytes(&mut bytes);
                    let mut a = match input {
                        0 => BoxedUint::zero_with_precision(bits),
                        1 => BoxedUint::one_with_precision(bits),
                        2 => BoxedUint::max(bits),
                        3 => modulus.clone(),
                        4 => modulus.wrapping_sub(Limb::ONE),
                        _ => BoxedUint::from_le_slice(&bytes, bits).unwrap(),
                    };

                    // Reuse dirty buffers and feed AMM results back into squaring.
                    for _ in 0..4 {
                        let before = BigUint::from_bytes_le(&a.to_le_bytes());
                        let mut expected = a.clone();
                        reference.mul_amm_assign(&mut expected, &a);
                        multiplier.square_amm_assign(&mut a);
                        assert_eq!(a, expected, "AMM: {nlimbs} limbs, modulus {kind}");
                        assert_eq!(
                            (BigUint::from_bytes_le(&a.to_le_bytes()) * &r) % &m,
                            (&before * &before) % &m,
                        );
                    }

                    let mut a = a.rem_vartime(params.modulus().as_nz_ref());
                    for _ in 0..4 {
                        let before = BigUint::from_bytes_le(&a.to_le_bytes());
                        let mut expected = a.clone();
                        reference.mul_assign(&mut expected, &a);
                        multiplier.square_assign(&mut a);
                        assert_eq!(a, expected, "reduced: {nlimbs} limbs, modulus {kind}");
                        assert!(a < modulus);
                        assert_eq!(
                            (BigUint::from_bytes_le(&a.to_le_bytes()) * &r) % &m,
                            (&before * &before) % &m,
                        );
                    }
                }
            }
        }
    }
}
