/*!
# glam

`glam` is a simple and fast linear algebra library for games and graphics.

## Features

* [`f32`](mod@f32) types
  * vectors: [`Vec2`], [`Vec3`], [`Vec3A`] and [`Vec4`]
  * square matrices: [`Mat2`], [`Mat3`], [`Mat3A`] and [`Mat4`]
  * a quaternion type: [`Quat`]
  * affine transformation types: [`Affine2`] and [`Affine3A`]
* [`f64`](mod@f64) types
  * vectors: [`DVec2`], [`DVec3`] and [`DVec4`]
  * square matrices: [`DMat2`], [`DMat3`] and [`DMat4`]
  * a quaternion type: [`DQuat`]
  * affine transformation types: [`DAffine2`] and [`DAffine3`]
* [`i32`](mod@i32) types
  * vectors: [`IVec2`], [`IVec3`] and [`IVec4`]
* [`u32`](mod@u32) types
  * vectors: [`UVec2`], [`UVec3`] and [`UVec4`]
* [`bool`](mod@bool) types
  * vectors: [`BVec2`], [`BVec3`] and [`BVec4`]

## SIMD

`glam` is built with SIMD in mind. Many `f32` types use 128-bit SIMD vector types for storage
and/or implementation. The use of SIMD generally enables better performance than using primitive
numeric types such as `f32`.

Some `glam` types use SIMD for storage meaning they are 16 byte aligned, these types include
`Mat2`, `Mat3A`, `Mat4`, `Quat`, `Vec3A`, `Vec4`, `Affine2` an `Affine3A`. Types
with an `A` suffix are a SIMD alternative to a scalar type, e.g. `Vec3` uses `f32` storage and
`Vec3A` uses SIMD storage.

When SIMD is not available on the target the types will maintain 16 byte alignment and internal
padding so that object sizes and layouts will not change between architectures. There are scalar
math fallback implementations exist when SIMD is not available. It is intended to add support for
other SIMD architectures once they appear in stable Rust.

Currently only SSE2 on x86/x86_64 is supported as this is what stable Rust supports.

## Vec3A and Mat3A

`Vec3A` is a SIMD optimized version of the `Vec3` type, which due to 16 byte alignment results
in `Vec3A` containing 4 bytes of padding making it 16 bytes in size in total. `Mat3A` is composed
of three `Vec3A` columns.

| Type       | `f32` bytes | Align bytes | Size bytes | Padding |
|:-----------|------------:|------------:|-----------:|--------:|
|[`Vec3`]    |           12|            4|          12|        0|
|[`Vec3A`]   |           12|           16|          16|        4|
|[`Mat3`]    |           36|            4|          36|        0|
|[`Mat3A`]   |           36|           16|          48|       12|

Despite this wasted space the SIMD implementations tend to outperform `f32` implementations in
[**mathbench**](https://github.com/bitshifter/mathbench-rs) benchmarks.

`glam` treats [`Vec3`] as the default 3D vector type and [`Vec3A`] a special case for optimization.
When methods need to return a 3D vector they will generally return [`Vec3`].

There are [`From`] trait implementations for converting from [`Vec4`] to a [`Vec3A`] and between
[`Vec3`] and [`Vec3A`] (and vice versa).

```
use glam::{Vec3, Vec3A, Vec4};

let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

// Convert from `Vec4` to `Vec3A`, this is a no-op if SIMD is supported.
let v3a = Vec3A::from(v4);
assert_eq!(Vec3A::new(1.0, 2.0, 3.0), v3a);

// Convert from `Vec3A` to `Vec3`.
let v3 = Vec3::from(v3a);
assert_eq!(Vec3::new(1.0, 2.0, 3.0), v3);

// Convert from `Vec3` to `Vec3A`.
let v3a = Vec3A::from(v3);
assert_eq!(Vec3A::new(1.0, 2.0, 3.0), v3a);
```

## Affine2 and Affine3A

`Affine2` and `Affine3A` are composed of a linear transform matrix and a vector translation. The
represent 2D and 3D affine transformations which are commonly used in games.

The table below shows the performance advantage of `Affine2` over `Mat3A` and `Mat3A` over `Mat3`.

| operation          | `Mat3`      | `Mat3A`    | `Affine2`  |
|--------------------|-------------|------------|------------|
| inverse            | 11.4±0.09ns | 7.1±0.09ns | 5.4±0.06ns |
| mul self           | 10.5±0.04ns | 5.2±0.05ns | 4.0±0.05ns |
| transform point2   |  2.7±0.02ns | 2.7±0.03ns | 2.8±0.04ns |
| transform vector2  |  2.6±0.01ns | 2.6±0.03ns | 2.3±0.02ns |

Performance is much closer between `Mat4` and `Affine3A` with the affine type being faster to
invert.

| operation          | `Mat4`      | `Affine3A`  |
|--------------------|-------------|-------------|
| inverse            | 15.9±0.11ns | 10.8±0.06ns |
| mul self           |  7.3±0.05ns |  7.0±0.06ns |
| transform point3   |  3.6±0.02ns |  4.3±0.04ns |
| transform point3a  |  3.0±0.02ns |  3.0±0.04ns |
| transform vector3  |  4.1±0.02ns |  3.9±0.04ns |
| transform vector3a |  2.8±0.02ns |  2.8±0.02ns |

Benchmarks were taken on an Intel Core i7-4710HQ.

## Linear algebra conventions

`glam` interprets vectors as column matrices (also known as column vectors) meaning when
transforming a vector with a matrix the matrix goes on the left.

```
use glam::{Mat3, Vec3};
let m = Mat3::IDENTITY;
let x = Vec3::X;
let v = m * x;
assert_eq!(v, x);
```

Matrices are stored in memory in column-major order.

All angles are in radians. Rust provides the `f32::to_radians()` and `f64::to_radians()` methods to
convert from degrees.

## Direct element access

Because some types may internally be implemented using SIMD types, direct access to vector elements
is supported by implementing the [`Deref`] and [`DerefMut`] traits.

```
use glam::Vec3A;
let mut v = Vec3A::new(1.0, 2.0, 3.0);
assert_eq!(3.0, v.z);
v.z += 1.0;
assert_eq!(4.0, v.z);
```

[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html

## Vector swizzles

`glam` vector types have functions allowing elements of vectors to be reordered, this includes
creating a vector of a different size from the vectors elements.

The swizzle functions are implemented using traits to add them to each vector type. This is
primarily because there are a lot of swizzle functions which can obfuscate the other vector
functions in documentation and so on. The traits are [`Vec2Swizzles`], [`Vec3Swizzles`] and
[`Vec4Swizzles`].

Note that the [`Vec3Swizzles`] implementation for [`Vec3A`] will return a [`Vec3A`] for 3 element
swizzles, all other implementations will return [`Vec3`].

```
use glam::{swizzles::*, Vec2, Vec3, Vec3A, Vec4};

let v = Vec4::new(1.0, 2.0, 3.0, 4.0);

// Reverse elements of `v`, if SIMD is supported this will use a vector shuffle.
let wzyx = v.wzyx();
assert_eq!(Vec4::new(4.0, 3.0, 2.0, 1.0), wzyx);

// Swizzle the yzw elements of `v` into a `Vec3`
let yzw = v.yzw();
assert_eq!(Vec3::new(2.0, 3.0, 4.0), yzw);

// To swizzle a `Vec4` into a `Vec3A` swizzle the `Vec4` first then convert to
// `Vec3A`. If SIMD is supported this will use a vector shuffle. The last
// element of the shuffled `Vec4` is ignored by the `Vec3A`.
let yzw = Vec3A::from(v.yzwx());
assert_eq!(Vec3A::new(2.0, 3.0, 4.0), yzw);

// You can swizzle from a `Vec4` to a `Vec2`
let xy = v.xy();
assert_eq!(Vec2::new(1.0, 2.0), xy);

// And back again
let yyxx = xy.yyxx();
assert_eq!(Vec4::new(2.0, 2.0, 1.0, 1.0), yyxx);
```

## SIMD and scalar consistency

`glam` types implement `serde` `Serialize` and `Deserialize` traits to ensure
that they will serialize and deserialize exactly the same whether or not
SIMD support is being used.

The SIMD versions implement the `core::fmt::Debug` and `core::fmt::Display`
traits so they print the same as the scalar version.

```
use glam::Vec4;
let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
assert_eq!(format!("{}", a), "[1, 2, 3, 4]");
```

## Feature gates

All `glam` dependencies are optional, however some are required for tests
and benchmarks.

* `std` - the default feature, has no dependencies.
* `approx` - traits and macros for approximate float comparisons
* `bytemuck` - for casting into slices of bytes
* `libm` - required to compile with `no_std`
* `mint` - for interoperating with other 3D math libraries
* `num-traits` - required to compile `no_std`, will be included when enabling
  the `libm` feature
* `rand` - implementations of `Distribution` trait for all `glam` types.
* `serde` - implementations of `Serialize` and `Deserialize` for all `glam`
  types. Note that serialization should work between builds of `glam` with and without SIMD enabled
* `scalar-math` - disables SIMD support and uses native alignment for all types.
* `debug-glam-assert` - adds assertions in debug builds which check the validity of parameters
  passed to `glam` to help catch runtime errors.
* `glam-assert` - adds assertions to all builds which check the validity of parameters passed to
  `glam` to help catch runtime errors.

## Minimum Supported Rust Version (MSRV)

The minimum supported Rust version is `1.45.0`.

*/
#![doc(html_root_url = "https://docs.rs/glam/0.15.2")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(target_arch = "spirv", feature(asm, register_attr, repr_simd))]
// This would require renaming a lot of stuff, disabling for now.
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
mod macros;

#[macro_use]
mod vec;

#[doc(hidden)]
pub mod cast;

mod affine2;
mod affine3;
mod core;
mod euler;
mod features;
mod mat2;
mod mat3;
mod mat4;
mod quat;
mod vec2;
mod vec3;
mod vec4;
mod vec_mask;

#[cfg(feature = "spirv-std")]
mod spirv;

#[cfg(feature = "transform-types")]
mod transform;

#[doc(hidden)]
pub use self::core::storage::{XY, XYZ, XYZW};

/** `bool` vector mask types. */
pub mod bool {
    pub use super::vec_mask::{BVec2, BVec3, BVec3A, BVec4, BVec4A};
}
pub use self::bool::*;

/** `f32` vector, quaternion and matrix types. */
pub mod f32 {
    pub use super::affine2::Affine2;
    pub use super::affine3::Affine3A;
    pub use super::mat2::{mat2, Mat2};
    pub use super::mat3::{mat3, mat3a, Mat3, Mat3A};
    pub use super::mat4::{mat4, Mat4};
    pub use super::quat::{quat, Quat};
    pub use super::vec2::{vec2, Vec2};
    pub use super::vec3::{vec3, vec3a, Vec3, Vec3A};
    pub use super::vec4::{vec4, Vec4};

    #[cfg(feature = "transform-types")]
    #[allow(deprecated)]
    pub use super::transform::{TransformRT, TransformSRT};
}
pub use self::f32::*;

/** `f64` vector, quaternion and matrix types. */
pub mod f64 {
    pub use super::affine2::DAffine2;
    pub use super::affine3::DAffine3;
    pub use super::mat2::{dmat2, DMat2};
    pub use super::mat3::{dmat3, DMat3};
    pub use super::mat4::{dmat4, DMat4};
    pub use super::quat::{dquat, DQuat};
    pub use super::vec2::{dvec2, DVec2};
    pub use super::vec3::{dvec3, DVec3};
    pub use super::vec4::{dvec4, DVec4};
}
pub use self::f64::*;

/** `i32` vector types. */
pub mod i32 {
    pub use super::vec2::{ivec2, IVec2};
    pub use super::vec3::{ivec3, IVec3};
    pub use super::vec4::{ivec4, IVec4};
}
pub use self::i32::*;

/** `u32` vector types. */
pub mod u32 {
    pub use super::vec2::{uvec2, UVec2};
    pub use super::vec3::{uvec3, UVec3};
    pub use super::vec4::{uvec4, UVec4};
}
pub use self::u32::*;

/** Traits adding swizzle methods to all vector types. */
pub mod swizzles;

pub use self::swizzles::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles};

/** Rotation Helper */
pub use euler::EulerRot;
