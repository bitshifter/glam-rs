#!/bin/sh

CARGO='rustup run 1.53.0 cargo'
$CARGO test --features "bytecheck bytemuck mint rand rkyv serde debug-glam-assert transform-types" && \
$CARGO test --features "scalar-math bytecheck bytemuck mint rand rkyv serde debug-glam-assert transform-types" && \
$CARGO test --no-default-features --features "libm scalar-math bytecheck bytemuck mint rand rkyv serde debug-glam-assert transform-types" && \
$CARGO bench --no-run
