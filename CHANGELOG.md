# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.7.1] - Unreleased
### Changed
#### Renamed VecNb to VecNMask and made size consistent without SSE2
* Renamed `Vec2b` to `Vec2Mask`, `Vec3b` to `Vec3Mask` and `Vec4b` to
  `Vec4Mask`. Old names are aliased to the new name and deprecated.
* Deprecate `VecNMask` `mask` method, use new `bitmask` method instead
* Made fallback version of `VecNMask` types the same size and alignment as the
  SIMD versions.
* Added `Default` support to `VecNMask` types, will add more common traits in
  the future.
#### Inline more functions
* Added `#[inline]` to `mat2`, `mat3` and `mat4` functions.
## [0.7.0] - 2019-06-28
### Added
#### Support converting matrices into flat arrays
* Added `Mat2` into `[f32; 4]`, `Mat3` into `[f32; 9]` and `Mat4` into
  `[f32; 16]`.
### Changed
#### Removed inconsistent `std::op::Mul` by reference support
* Removed `impl Mul<&Vec2> for Mat2` and `impl Mul<&Vec3> for Vec3` as these
  don't exist for any other types.

## [0.6.1] - 2019-06-22
### Changed
#### Mat2 optimizations
* `Mat2` now uses a `Vec4` internally which gives it some performance
   improvements when SSE2 is available.

## [0.6.0] - 2019-06-13
### Changed
#### Switched from row vectors to column vectors
* Vectors are now on the right of multiplications with matrices and quaternions.


[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
