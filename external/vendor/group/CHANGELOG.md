# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0] - 2026-06-01
### Added
- `group::CurveAffine`, an affine-representation trait that the curve-specific
  affine traits are now built on top of.
- `group::Group::mul_by_generator`, with a default implementation. Implementors
  can override it to take advantage of precomputed tables.
- `group::Group::try_random<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error>`,
  a new trait method that must be implemented by downstreams. It samples a
  non-identity group element using a fallible RNG and propagates the RNG's error.

### Changed
- MSRV is now 1.85.0.
- Bumped dependencies to `ff 0.14`, `rand_core 0.10`.
- `group::Group::random(rng: impl RngCore) -> Self` has been changed to
  `Group::random<R: Rng + ?Sized>(rng: &mut R) -> Self`, to enable passing a
  trait object as the RNG. It now has a default implementation in terms of
  `Group::try_random`.
- The curve-related traits have been refactored around the new `CurveAffine`
  trait:
  - `group::Curve::AffineRepr` has been renamed to `Curve::Affine`.
  - All of the trait methods and associated types on the following traits have
    been removed (use `group::Curve::Affine` or the `group::CurveAffine` trait
    instead; trait implementors must implement `group::CurveAffine` instead
    using the same logic):
    - `group::cofactor::CofactorCurve`
    - `group::cofactor::CofactorCurveAffine`
    - `group::prime::PrimeCurve`
    - `group::prime::PrimeCurveAffine`
  - `group::cofactor::CofactorCurveAffine` and `group::prime::PrimeCurveAffine`
    now have blanket implementations for all types `C: group::CurveAffine` where
    `C::Curve` implements `CofactorCurve` or `PrimeCurve` respectively.

## [0.13.0] - 2022-12-06
### Changed
- Bumped `ff` to `0.13`

## [0.12.1] - 2022-10-13
### Added
- `group::{WnafBase, WnafScalar}` structs for caching precomputations of both
  bases and scalars, for improved many-base many-scalar multiplication
  performance.
- `impl memuse::DynamicUsage for group::{Wnaf WnafBase, WnafScalar}`, behind the
  new `wnaf-memuse` feature flag, to enable the heap usage of these types to be
  measured at runtime.

### Changed
- Removed temporary allocations from `Wnaf` internals for improved performance.

## [0.12.0] - 2022-05-04
### Changed
- MSRV is now 1.56.0.
- Bumped `ff` to `0.12`

## [0.11.0] - 2021-09-02
### Fixed
- The affine scalar multiplication bounds on the following traits had typos that
  prevented multiplying by `&Self::Scalar`, which has now been fixed:
  - `group::cofactor::{CofactorCurve::Affine, CofactorCurveAffine}`
  - `group::prime::{PrimeCurve::Affine, PrimeCurveAffine}`

### Added
- `Copy + Send + Sync + 'static` bounds on `group::GroupEncoding::Repr`.

### Changed
- Bumped `ff` to 0.11.

## [0.10.0] - 2021-06-01
### Added
- `group::ff`, which re-exports the `ff` crate to make version-matching easier.

### Changed
- MSRV is now 1.51.0.
- Bumped `ff` to 0.10.

### Removed
- `group::cofactor::CofactorGroup::is_torsion_free` provided implementation
  (trait implementors must now implement this method themselves). This avoids
  a hard dependency on the `ff/bits` feature flag.

## [0.9.0] - 2021-01-06
### Changed
- Bumped dependencies to `ff 0.9`, `rand_core 0.6`, `rand 0.8`.

## [0.8.0] - 2020-09-08
### Added
- `no_std` support.

### Changed
- MSRV is now 1.44.0.
- Bumped `ff` to 0.8.
- `group::{wnaf, Wnaf, WnafGroup}` are now gated behind the (default-enabled)
  `alloc` feature flag. The `byteorder` dependency is now optional.
- `group::tests` is now gated behind the `tests` feature flag. The `rand` and
  `rand_xorshift` dependencies are now optional.

### Removed
- `fmt::Display` bound from the following traits:
  - `group::Group`
  - `group::cofactor::CofactorCurveAffine`
  - `group::prime::PrimeCurveAffine`
