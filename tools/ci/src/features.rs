macro_rules! deps {
    () => {
        "arbitrary approx-05 bytemuck encase-012 encase-013 float_eq mint-05 rand-010 rkyv-08 bytecheck serde speedy-08 zerocopy-08 debug-glam-assert"
    };
}

pub(crate) const FEATURE_SETS: &[&str] = &[
    concat!("std ", deps!()),
    concat!("std all-types scalar-math ", deps!()),
    "std all-types cuda bytemuck",
    "std all-types scalar-math cuda bytemuck",
    "std all-types libm bytemuck",
    "std all-types scalar-math libm bytemuck",
    concat!("libm ", deps!()),
    concat!("libm all-types ", deps!()),
    concat!("libm all-types scalar-math ", deps!()),
];

// MSRV reduced set. `scalar-math`, `cuda`, `libm` and `nostd-libm` are added by
// the individual checks in tools/ci/src/commands/msrv.rs.
pub(crate) const MSRV_FEATURES: &str = "all-types glam-assert debug-glam-assert";

// All optional deps used by clippy, doc, and coverage
pub(crate) const ALL_FEATURES: &str = deps!();

// core-simd profile features (no zerocopy as it doesn't compile with core-simd)
pub(crate) const CORE_SIMD_FEATURES: &str = "core-simd arbitrary approx-05 bytemuck encase-012 encase-013 float_eq mint-05 rand-010 rkyv-08 bytecheck serde speedy-08 debug-glam-assert";

// A small subset of the CI tests, used by the reduced pre-push check.
pub(crate) const PRE_PUSH_FEATURE_SETS: &[&str] = &[
    concat!("std ", deps!()),
    concat!("std all-types scalar-math ", deps!()),
];

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
