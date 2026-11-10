#![no_std]
// Catch documentation errors caused by code changes.
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

// Re-export ff to make version-matching easier.
pub use ff;

use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use ff::PrimeField;
use rand_core::{Rng, TryRng};
use subtle::{Choice, CtOption};

pub mod cofactor;
pub mod prime;
#[cfg(feature = "tests")]
pub mod tests;

#[cfg(feature = "alloc")]
mod wnaf;
#[cfg(feature = "alloc")]
pub use self::wnaf::{Wnaf, WnafBase, WnafGroup, WnafScalar};

/// A helper trait for types with a group operation.
pub trait GroupOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + AddAssign<Rhs> + SubAssign<Rhs>
{
}

impl<T, Rhs, Output> GroupOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + AddAssign<Rhs> + SubAssign<Rhs>
{
}

/// A helper trait for references with a group operation.
pub trait GroupOpsOwned<Rhs = Self, Output = Self>: for<'r> GroupOps<&'r Rhs, Output> {}
impl<T, Rhs, Output> GroupOpsOwned<Rhs, Output> for T where T: for<'r> GroupOps<&'r Rhs, Output> {}

/// A helper trait for types implementing group scalar multiplication.
pub trait ScalarMul<Rhs, Output = Self>: Mul<Rhs, Output = Output> + MulAssign<Rhs> {}

impl<T, Rhs, Output> ScalarMul<Rhs, Output> for T where T: Mul<Rhs, Output = Output> + MulAssign<Rhs>
{}

/// A helper trait for references implementing group scalar multiplication.
pub trait ScalarMulOwned<Rhs, Output = Self>: for<'r> ScalarMul<&'r Rhs, Output> {}
impl<T, Rhs, Output> ScalarMulOwned<Rhs, Output> for T where T: for<'r> ScalarMul<&'r Rhs, Output> {}

/// This trait represents an element of a cryptographic group.
pub trait Group:
    Clone
    + Copy
    + fmt::Debug
    + Eq
    + Sized
    + Send
    + Sync
    + 'static
    + Sum
    + for<'a> Sum<&'a Self>
    + Neg<Output = Self>
    + GroupOps
    + GroupOpsOwned
    + ScalarMul<<Self as Group>::Scalar>
    + ScalarMulOwned<<Self as Group>::Scalar>
{
    /// Scalars modulo the order of this group's scalar field.
    type Scalar: PrimeField;

    /// Returns an element chosen uniformly at random from the non-identity elements of
    /// this group using a user-provided infallible RNG.
    ///
    /// This is a convenience wrapper around [`Group::try_random`] for RNGs that cannot
    /// fail. Use [`Group::try_random`] if your RNG may fail (for example, an OS-backed
    /// entropy source).
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let Ok(out) = Self::try_random(rng);
        out
    }

    /// Returns an element chosen uniformly at random from the non-identity elements of
    /// this group using a user-provided fallible RNG.
    ///
    /// Returns `Err` propagating the RNG's error if the underlying RNG fails to produce
    /// the randomness required to sample an element. Implementors of `Group` must
    /// provide this method; [`Group::random`] is derived from it for infallible RNGs.
    fn try_random<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error>;

    /// Returns the additive identity, also known as the "neutral element".
    fn identity() -> Self;

    /// Returns a fixed generator of the prime-order subgroup.
    fn generator() -> Self;

    /// Determines if this point is the identity.
    fn is_identity(&self) -> Choice;

    /// Doubles this element.
    #[must_use]
    fn double(&self) -> Self;

    /// Multiply by the generator of the prime-order subgroup.
    #[must_use]
    fn mul_by_generator(scalar: &Self::Scalar) -> Self {
        Self::generator() * scalar
    }
}

/// Efficient representation of an elliptic curve point.
pub trait Curve: Group + GroupOps<Self::Affine> + GroupOpsOwned<Self::Affine> {
    /// The affine representation for this elliptic curve.
    type Affine: CurveAffine<Curve = Self, Scalar = Self::Scalar>;

    /// Converts a batch of projective elements into affine elements. This function will
    /// panic if `p.len() != q.len()`.
    fn batch_normalize(p: &[Self], q: &mut [Self::Affine]) {
        assert_eq!(p.len(), q.len());

        for (p, q) in p.iter().zip(q.iter_mut()) {
            *q = p.to_affine();
        }
    }

    /// Converts this element into its affine representation.
    fn to_affine(&self) -> Self::Affine;
}

/// Affine representation of an elliptic curve point.
pub trait CurveAffine:
    GroupEncoding
    + Copy
    + fmt::Debug
    + Eq
    + Send
    + Sync
    + 'static
    + Neg<Output = Self>
    + Mul<<Self::Curve as Group>::Scalar, Output = Self::Curve>
    + for<'r> Mul<&'r <Self::Curve as Group>::Scalar, Output = Self::Curve>
{
    /// The efficient representation for this elliptic curve.
    type Curve: Curve<Affine = Self, Scalar = Self::Scalar>;

    /// Scalars modulo the order of this group's scalar field.
    ///
    /// This associated type is temporary, and will be removed once downstream users have
    /// migrated to using `Curve` as the primary generic bound.
    type Scalar: PrimeField;

    /// Returns the additive identity.
    fn identity() -> Self;

    /// Returns a fixed generator of unknown exponent.
    fn generator() -> Self;

    /// Determines if this point represents the additive identity.
    fn is_identity(&self) -> Choice;

    /// Converts this affine point to its efficient representation.
    fn to_curve(&self) -> Self::Curve;
}

pub trait GroupEncoding: Sized {
    /// The encoding of group elements.
    ///
    /// The `Default` implementation is not required to return a valid point encoding. The
    /// bound is present to enable encodings to be constructed generically:
    /// ```
    /// # use group::GroupEncoding;
    /// # use subtle::CtOption;
    /// # struct G;
    /// # impl GroupEncoding for G {
    /// #     type Repr = [u8; 0];
    /// #     fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> { unimplemented!() }
    /// #     fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> { unimplemented!() }
    /// #     fn to_bytes(&self) -> Self::Repr { unimplemented!() }
    /// # }
    /// # let buf = &[0u8; 0][..];
    /// let mut encoding = <G as GroupEncoding>::Repr::default();
    /// encoding.as_mut().copy_from_slice(buf);
    /// ```
    ///
    /// It is recommended that the default should be the all-zeroes encoding.
    type Repr: Copy + Default + Send + Sync + 'static + AsRef<[u8]> + AsMut<[u8]>;

    /// Attempts to deserialize a group element from its encoding.
    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self>;

    /// Attempts to deserialize a group element, not checking if the element is valid.
    ///
    /// **This is dangerous to call unless you trust the bytes you are reading; otherwise,
    /// API invariants may be broken.** Please consider using
    /// [`GroupEncoding::from_bytes`] instead.
    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self>;

    /// Converts this element into its byte encoding. This may or may not support
    /// encoding the identity.
    // TODO: Figure out how to handle identity encoding generically.
    fn to_bytes(&self) -> Self::Repr;
}

/// Affine representation of a point on an elliptic curve that has a defined uncompressed
/// encoding.
pub trait UncompressedEncoding: Sized {
    type Uncompressed: Default + AsRef<[u8]> + AsMut<[u8]>;

    /// Attempts to deserialize an element from its uncompressed encoding.
    fn from_uncompressed(bytes: &Self::Uncompressed) -> CtOption<Self>;

    /// Attempts to deserialize an uncompressed element, not checking if the element is in
    /// the correct subgroup.
    ///
    /// **This is dangerous to call unless you trust the bytes you are reading; otherwise,
    /// API invariants may be broken.** Please consider using
    /// [`UncompressedEncoding::from_uncompressed`] instead.
    fn from_uncompressed_unchecked(bytes: &Self::Uncompressed) -> CtOption<Self>;

    /// Converts this element into its uncompressed encoding, so long as it's not
    /// the point at infinity.
    fn to_uncompressed(&self) -> Self::Uncompressed;
}
