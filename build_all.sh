#!/bin/sh

cargo test --features "bytemuck mint rand serde debug-glam-assert transform-types" && \
cargo test --features "scalar-math bytemuck mint rand serde debug-glam-assert transform-types" && \
cargo test --no-default-features --features "libm scalar-math bytemuck mint rand serde debug-glam-assert transform-types"
