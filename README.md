# glam-rs

[![Build Status](https://travis-ci.org/bitshifter/glam-rs.svg?branch=master)](https://travis-ci.org/bitshifter/glam-rs)
[![Coverage Status](https://coveralls.io/repos/github/bitshifter/glam-rs/badge.svg?branch=master)](https://coveralls.io/github/bitshifter/glam-rs?branch=master)

Experimenting with vector math libraries.

A simple and fast 3D math library for games and graphics.

This will change a lot, don't use it :)

Design decisions:
* Only f32 implementations for now (no f64)
* All Vec types have SSE2 version, everything is 16byte aligned (even Vec2)
* Row vectors, multiplications are applied from left to right
	* vector1 = vector0 * transform
* Rotation is left handed (+ve rotation is counter clockwise around axis)
* Coordinate space is left handed, with +Z forward, +Y up and +X right - only really relevant when creating rotations from euler angles
* Types so far:
  * Angle, Vec2, Vec3, Vec4, Quat, Mat4, TransformRT, TransformSRT
* Dependencies are optional (e.g. mint, rand and serde)
* Idiomatic Rust, e.g. methods instead of free functions

Design goals:
* Row vectors instead of column vectors (for mul operator order)
* Implemented with SIMD (only SSE2 for now)
* No generics necessary - only f32 supported (although f64 should be feasible)
* No traits necessary

Potential goals:
* Experimental fast-math scalar implementation

Rejected goals:
* Initially supporting having f32 and sse2 implementations available, mostly for ease of testing and benchmarking without recompiling, but this is starting to make the default use case (users won't switch between f32 and sse2 at runtime) more complicated.

Inspired by a bunch of different math libraries in different ways:
* cgmath
* nalgebra_glm
* DirectXMath
* DirectXTK SimpleMath
* RealtimeMath
