#!/bin/bash

set -e

# Set of features to build & test.
FEATURE_SETS=(
  # std
  "std"
  "std approx bytemuck mint rand serde debug-glam-assert"
  "std scalar-math approx bytemuck mint rand serde debug-glam-assert"
  "std cuda"
  "std scalar-math cuda"
  # no_std
  "libm"
  "libm scalar-math approx bytemuck mint rand serde debug-glam-assert"
)

rustc --version

for features in "${FEATURE_SETS[@]}"
do
   :
   echo cargo build --tests --no-default-features --features=\"$features\"
   cargo build --tests --no-default-features --features="$features"
   echo cargo test --no-default-features --features=\"$features\"
   cargo test --no-default-features --features="$features"
done

echo RUSTFLAGS='-C target-feature=+fma' cargo check
RUSTFLAGS='-C target-feature=+fma' cargo check

echo cargo check test_no_std
pushd test_no_std && cargo check
