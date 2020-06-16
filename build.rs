use std::env;

fn main() {
    let force_scalar_math = env::var("CARGO_FEATURE_SCALAR_MATH").is_ok();

    let target_feature_sse2 = env::var("CARGO_CFG_TARGET_FEATURE")
        .map_or(false, |cfg| cfg.split(',').find(|&f| f == "sse2").is_some());

    if target_feature_sse2 && !force_scalar_math {
        println!("cargo:rustc-cfg=vec3sse2");
        println!("cargo:rustc-cfg=vec3align16sse2");
    } else {
        println!("cargo:rustc-cfg=vec3align16f32");
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
