use subtle::{Choice, CtOption};

use crate::{prime::PrimeGroup, Curve, CurveAffine, Group, GroupEncoding, GroupOps, GroupOpsOwned};

/// This trait represents an element of a cryptographic group with a large prime-order
/// subgroup and a comparatively-small cofactor.
pub trait CofactorGroup:
    Group
    + GroupEncoding
    + GroupOps<<Self as CofactorGroup>::Subgroup>
    + GroupOpsOwned<<Self as CofactorGroup>::Subgroup>
{
    /// The large prime-order subgroup in which cryptographic operations are performed.
    /// If `Self` implements `PrimeGroup`, then `Self::Subgroup` may be `Self`.
    type Subgroup: PrimeGroup<Scalar = Self::Scalar> + Into<Self>;

    /// Maps `self` to the prime-order subgroup by multiplying this element by some
    /// `k`-multiple of the cofactor.
    ///
    /// The value `k` does not vary between inputs for a given implementation, but may
    /// vary between different implementations of `CofactorGroup` because some groups have
    /// more efficient methods of clearing the cofactor when `k` is allowed to be
    /// different than `1`.
    ///
    /// If `Self` implements [`PrimeGroup`], this returns `self`.
    fn clear_cofactor(&self) -> Self::Subgroup;

    /// Returns `self` if it is contained in the prime-order subgroup.
    ///
    /// If `Self` implements [`PrimeGroup`], this returns `Some(self)`.
    fn into_subgroup(self) -> CtOption<Self::Subgroup>;

    /// Determines if this element is of small order.
    ///
    /// Returns:
    /// - `true` if `self` is in the torsion subgroup.
    /// - `false` if `self` is not in the torsion subgroup.
    fn is_small_order(&self) -> Choice {
        self.clear_cofactor().is_identity()
    }

    /// Determines if this element is "torsion free", i.e., is contained in the
    /// prime-order subgroup.
    ///
    /// Returns:
    /// - `true` if `self` has trivial torsion and is in the prime-order subgroup.
    /// - `false` if `self` has non-zero torsion component and is not in the prime-order
    ///   subgroup.
    fn is_torsion_free(&self) -> Choice;
}

/// Efficient representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait CofactorCurve: Curve + CofactorGroup {}

/// Affine representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait CofactorCurveAffine: CurveAffine {}

impl<C: CurveAffine> CofactorCurveAffine for C where C::Curve: CofactorCurve {}
