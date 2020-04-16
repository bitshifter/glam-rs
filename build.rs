use std::{collections::HashSet, env};

fn main() {
    let force_scalar_math = env::var("CARGO_FEATURE_SCALAR_MATH").is_ok();
    let force_packed_vec3 = env::var("CARGO_FEATURE_PACKED_VEC3").is_ok();

    let cfg_target_feature: Option<String> = env::var("CARGO_CFG_TARGET_FEATURE").ok();

    // TODO: probably excessive making a hashmap
    let target_features: HashSet<&str> = cfg_target_feature
        .as_ref()
        .map_or(HashSet::new(), |v| v.split(',').collect());
    let target_feature_sse2 = target_features.contains("sse2");

    if target_feature_sse2 && !force_scalar_math && !force_packed_vec3 {
        println!("cargo:rustc-cfg=vec3sse2");
    } else {
        if !force_scalar_math && !force_packed_vec3 {
            // simd not available but not explicitly disabled so maintain 16 byte alignment
            println!("cargo:rustc-cfg=vec3f32_align16");
        }
        println!("cargo:rustc-cfg=vec3f32");
    }

    if target_feature_sse2 && !force_scalar_math {
        println!("cargo:rustc-cfg=vec4sse2");
    } else {
        if !force_scalar_math {
            // simd not available but not explicitly disabled so maintain 16 byte alignment
            println!("cargo:rustc-cfg=vec4f32_align16");
        }
        println!("cargo:rustc-cfg=vec4f32");
    }
}
