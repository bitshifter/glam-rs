#!/bin/bash

set -ex

# Supported dependencies
DEPENDENCIES="arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy zerocopy debug-glam-assert"

# Set of features to build & test.
FEATURE_SETS=(
  # std
  "std $DEPENDENCIES"
  "std all-types scalar-math $DEPENDENCIES"
  "std all-types cuda"
  "std all-types scalar-math cuda"
  "std all-types libm"
  "std all-types scalar-math libm"
  # no_std
  "libm $DEPENDENCIES"
  "libm all-types $DEPENDENCIES"
  "libm all-types scalar-math $DEPENDENCIES"
)

rustc --version

for features in "${FEATURE_SETS[@]}"
do
  :
  cargo build --tests --no-default-features --features="$features"
  cargo test --no-default-features --features="$features"
done

RUSTFLAGS='-C target-feature=+fma' cargo check

cargo check -p glam-no_std
