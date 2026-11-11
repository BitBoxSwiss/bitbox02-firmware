use crate::{Curve, CurveAffine, Group, GroupEncoding};

/// This trait represents an element of a prime-order cryptographic group.
pub trait PrimeGroup: Group + GroupEncoding {}

/// Efficient representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait PrimeCurve: Curve + PrimeGroup {}

/// Affine representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait PrimeCurveAffine: CurveAffine {}

impl<C: CurveAffine> PrimeCurveAffine for C where C::Curve: PrimeCurve {}
