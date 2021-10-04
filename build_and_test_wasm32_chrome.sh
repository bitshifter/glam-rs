#!/bin/sh

set -e

RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack test --headless --chrome --features=glam-simd128
wasm-pack test --headless --chrome
