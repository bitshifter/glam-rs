# glam-rs

[![Build Status](https://travis-ci.org/bitshifter/glam-rs.svg?branch=master)](https://travis-ci.org/bitshifter/glam-rs)
[![Coverage Status](https://coveralls.io/repos/github/bitshifter/glam-rs/badge.svg?branch=master)](https://coveralls.io/github/bitshifter/glam-rs?branch=master)

Experimenting with vector math libraries.

This will change a lot, don't use it :)

Design goals:
* A simple and fast 3D math library for games and graphics
* Row vectors instead of column vectors
* Implemented with SIMD (only SSE2 for now)
* No generics necessary - only f32 supported (although f64 should be feasible)
* No traits necessary
* Vector types are always 16 byte aligned
* Dependencies are optional (e.g. mint, rand and serde)

Potential goals:
* Experimental fast-math scalar implementation

Rejected goals:
* Initially supporting having f32 and sse2 implementations available, mostly for ease of testing and benchmarking without recompiling, but this is starting to make the default use case (users won't switch between f32 and sse2 at runtime) more complicated.

