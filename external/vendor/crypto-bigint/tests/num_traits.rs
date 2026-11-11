//! Tests for the `num_traits` additive and multiplicative identities.

use core::fmt::Debug;
use crypto_bigint::{I256, Limb, U256, Wrapping, const_monty_params, modular::ConstMontyForm};
use num_traits::{One, Zero};

const_monty_params!(
    Modulus,
    U256,
    "0000000000000000000000000000000000000000000000000000000000000013"
);

type FieldElement = ConstMontyForm<Modulus, { U256::LIMBS }>;

fn assert_one<T>(one: T, non_one: T)
where
    T: One + Eq + Debug,
{
    assert_eq!(T::one(), one);
    assert!(one.is_one());
    assert!(!non_one.is_one());
    assert_eq!(T::one() * T::one(), one);

    let mut value = non_one;
    value.set_one();
    assert_eq!(value, one);
}

fn assert_zero_one<T>(zero: T, one: T)
where
    T: Zero + One + Eq + Debug,
{
    assert_eq!(T::zero(), zero);
    assert!(zero.is_zero());
    assert!(!one.is_zero());

    let mut value = T::one();
    value.set_zero();
    assert_eq!(value, zero);

    assert_one(one, zero);
}

#[test]
fn zero_one() {
    assert_zero_one(Limb::ZERO, Limb::ONE);
    assert_zero_one(U256::ZERO, U256::ONE);
    assert_zero_one(I256::ZERO, I256::ONE);
    assert_zero_one(Wrapping(U256::ZERO), Wrapping(U256::ONE));
    assert_zero_one(FieldElement::ZERO, FieldElement::ONE);

    #[cfg(feature = "alloc")]
    assert_zero_one(
        crypto_bigint::BoxedUint::zero(),
        crypto_bigint::BoxedUint::one(),
    );

    assert_one(U256::ONE.to_nz().unwrap(), U256::from(3u8).to_nz().unwrap());
    assert_one(
        U256::ONE.to_odd().unwrap(),
        U256::from(3u8).to_odd().unwrap(),
    );
}
