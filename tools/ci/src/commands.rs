pub mod check_features;
pub mod ci;
pub mod clippy;
pub mod codegen;
pub mod core_simd;
pub mod coverage_llvm;
pub mod coverage_tarpaulin;
pub mod doc;
pub mod fmt;
pub mod lints;
pub mod msrv;
pub mod pre_push;
pub mod test_features;
pub mod wasm32;
pub mod wasm32_chrome;
pub mod wasm32_firefox;
pub mod wasm32_node;
pub mod wasm64;

macro_rules! deps {
    () => {
        "arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy zerocopy debug-glam-assert"
    };
}

pub(crate) const FEATURE_SETS: &[&str] = &[
    concat!("std ", deps!()),
    concat!("std all-types scalar-math ", deps!()),
    "std all-types cuda",
    "std all-types scalar-math cuda",
    "std all-types libm",
    "std all-types scalar-math libm",
    concat!("libm ", deps!()),
    concat!("libm all-types ", deps!()),
    concat!("libm all-types scalar-math ", deps!()),
];

// MSRV reduced set — some optional deps need a newer rustc
pub(crate) const MSRV_FEATURES: &str = "all-types arbitrary approx mint speedy debug-glam-assert";

// All optional deps used by clippy and doc
pub(crate) const OPTIONAL_DEPS: &str = deps!();

// core-simd profile features (no zerocopy as it doesn't compile with core-simd)
pub(crate) const CORE_SIMD_FEATURES: &str =
    "core-simd arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy debug-glam-assert";

pub fn resolve_sets(index: Option<usize>) -> &'static [&'static str] {
    match index {
        Some(i) => {
            if i == 0 || i > FEATURE_SETS.len() {
                panic!(
                    "feature set index {i} is out of range (1-{})",
                    FEATURE_SETS.len()
                );
            }
            &FEATURE_SETS[i - 1..i]
        }
        None => FEATURE_SETS,
    }
}

pub fn print_feature_sets() {
    for (i, features) in FEATURE_SETS.iter().enumerate() {
        println!("  {}. {features}", i + 1);
    }
}
