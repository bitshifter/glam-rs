# glam

[![Build Status](https://travis-ci.org/bitshifter/glam-rs.svg?branch=master)](https://travis-ci.org/bitshifter/glam-rs)
[![Coverage Status](https://coveralls.io/repos/github/bitshifter/glam-rs/badge.svg?branch=master)](https://coveralls.io/github/bitshifter/glam-rs?branch=master)

A simple and fast 3D math library for games and graphics.

# Development status

`glam` is in a pre-alpha stage. Base functionality has been implemented and the look and feel of the
API has solidified.

Only single precision floating point arithmetic (i.e. `f32`) is currently supported.

At this point I'm looking to see if people are interested in using it and how they find it. So it is
possible that future versions may include API changes or even changes in the conventions listed
below.

# Features

* vectors: `Vec3`, `Vec3`, `Vec4`
* square matrices: `Mat2`, `Mat3`, `Mat4`
* a quaternion type: `Quat`
* an angle type: `Angle`

## SIMD

The `Vec3`, `Vec4` and `Quat` types use SSE2 on x86/x86_64 architectures. `Mat3` and `Mat4` also use
SSE2 for some functionality such as inverse and transpose. Not everything has a SIMD implementation
yet.

Due to the use of SIMD, vector elements may only be get and set via accessor methods, e.g.
`Vec3::get_x()` and `Vec3::set_x()`. If getting or setting more than one element it is more
efficient to convert from tuples or arrays, e.g. `(x, y, z) = v.into()`.

## Default features

* `approx` - implementations of the `AbsDiffEq` and `UlpsEq` traits for all `glam` types
* `rand` - implementations of `Distribution` trait for all `glam` types
* `serde` - implementations of `Serialize` and `Deserialize` for all `glam` types. Note that
  serialization should work between builds of `glam` with and without SIMD enabled

## Feature gates

* `no-simd` - compiles with SIMD support disabled

# Conventions

## Row vectors

`glam` interprets vectors as row matrices (also known as "row vectors") meaning when transforming a
vector with a matrix the matrix goes on the right, e.g. `v' = vM`.  DirectX uses row vectors, OpenGL
uses column vectors. There are pros and cons to both, the main advantage of row vectors is
multiplication reads from left to right.

For example:

```
result = input * local_to_object * object_to_world
```

Input is in local space, it get transformed into object space before the final transform into
world space.

## Row-major order

Matrices are stored in row major format.

## Co-ordinate system

When relevant, a left-handed co-ordinate system is used:

* `+X` - right
* `+Y` - up
* `+Z` - forward

The co-ordinate system primary affects functions that deal with Euler angle rotations.

Rotations follow the left-hand rule.

# Design Philosophy

The design of this library is guided by a desire for simplicity and good performance.

* Only `f32` arithmetic is supported for now (no `f64`)
* No traits or generics
* Dependencies are optional (e.g. mint, rand and serde)
* Follows a Rust style, e.g. methods instead of free functions
* Aiming for 100% test coverage for both SIMD and scalar code paths
* Common functionality is benchmarked

# Future work

* Experiment with replacing SSE2 code with `f32x4` from the `packed_simd` library
  - this will mean other architectures get SIMD support
* Experiment with a using a 4x3 matrix as a 3D transform type that can be more efficient than `Mat4`
  for certain operations like inverse and multiplies

# Naming

`glam` is a play on the name of the popular C++ library `glm`.
