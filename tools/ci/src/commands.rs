pub mod check_features;
pub mod clippy;
pub mod codegen;
pub mod doc;
pub mod fmt;
pub mod lints;
pub mod test_features;

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
        print!("  {}. {features}\n", i + 1);
    }
}
