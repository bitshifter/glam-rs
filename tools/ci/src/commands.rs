pub mod check_features;
pub mod clippy;
pub mod codegen;
pub mod doc;
pub mod fmt;
pub mod lints;
pub mod test_features;

const DEPENDENCIES: &str = "arbitrary approx bytemuck encase mint rand rkyv bytecheck serde speedy zerocopy debug-glam-assert";

fn feature_sets() -> Vec<String> {
    vec![
        format!("std {DEPENDENCIES}"),
        format!("std all-types scalar-math {DEPENDENCIES}"),
        "std all-types cuda".into(),
        "std all-types scalar-math cuda".into(),
        "std all-types libm".into(),
        "std all-types scalar-math libm".into(),
        format!("libm {DEPENDENCIES}"),
        format!("libm all-types {DEPENDENCIES}"),
        format!("libm all-types scalar-math {DEPENDENCIES}"),
    ]
}
