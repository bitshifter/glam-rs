# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]
### Added
* Added `Vec2Mask::new`, `Vec3Mask::new` and `Vec4Mask::new` methods.
* Added method documentation for `Vec4` and `Vec4Mask` types.

## [0.7.1] - 2019-07-08
### Fixed
* The SSE2 implementation of `Vec4` `dot` was missing a shuffle, meaning the
  `dot`, `length`, `length_squared`, `length_reciprocal` and `normalize`
  methods were sometimes incorrect.
### Added
* Added the `glam_assert` macro which behaves like Rust's `debug_assert` but
  can be enabled separately to `debug_assert`. This is used to perform
  asserts on correctness.
* Added `is_normalized` method to `Vec2`, `Vec3` and `Vec4`.
### Changed
* Replaced usage of `std::mem::uninitialized` with `std::mem::MaybeUninit`. This
  change requires stable Rust 1.36.
* Renamed `Vec2b` to `Vec2Mask`, `Vec3b` to `Vec3Mask` and `Vec4b` to
  `Vec4Mask`. Old names are aliased to the new name and deprecated.
* Deprecate `VecNMask` `mask` method, use new `bitmask` method instead
* Made fallback version of `VecNMask` types the same size and alignment as the
  SIMD versions.
* Added `Default` support to `VecNMask` types, will add more common traits in
  the future.
* Added `#[inline]` to `mat2`, `mat3` and `mat4` functions.

## [0.7.0] - 2019-06-28
### Added
* Added `Mat2` into `[f32; 4]`, `Mat3` into `[f32; 9]` and `Mat4` into
  `[f32; 16]`.
### Changed
* Removed `impl Mul<&Vec2> for Mat2` and `impl Mul<&Vec3> for Vec3` as these
  don't exist for any other types.

## [0.6.1] - 2019-06-22
### Changed
* `Mat2` now uses a `Vec4` internally which gives it some performance
   improvements when SSE2 is available.

## 0.6.0 - 2019-06-13
### Changed
* Switched from row vectors to column vectors
  * Vectors are now on the right of multiplications with matrices and quaternions.


[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Unreleased]: https://github.com/bitshifter/glam-rs/compare/0.7.1...HEAD
[0.7.1]: https://github.com/bitshifter/glam-rs/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/bitshifter/glam-rs/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/bitshifter/glam-rs/compare/0.6.0...0.6.1

