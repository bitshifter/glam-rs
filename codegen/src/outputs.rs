use serde::ser::Serialize;
use std::collections::HashMap;

struct ContextBuilder(tera::Context);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Target {
    Scalar,
    Sse2,
    Wasm32,
    Neon,
    CoreSimd,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self(tera::Context::new())
    }

    fn new_tvecn_swizzle_impl(dim: u32, prefix: &str) -> Self {
        ContextBuilder::new()
            .with_template("swizzle_impl.rs.tera")
            .target_scalar()
            .with_key_val("vec2_t", &format!("{prefix}Vec2"))
            .with_key_val("vec3_t", &format!("{prefix}Vec3"))
            .with_key_val("vec4_t", &format!("{prefix}Vec4"))
            .with_self_t(&format!("{prefix}Vec{dim}"))
            .with_dimension(dim)
    }

    pub fn new_vec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "")
    }

    pub fn new_vec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "")
    }

    pub fn new_vec3a_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "")
            .with_key_val("vec3_t", "Vec3A")
            .with_self_t("Vec3A")
    }

    pub fn new_vec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "")
    }

    pub fn new_dvec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "D")
    }

    pub fn new_dvec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "D")
    }

    pub fn new_dvec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "D")
    }

    pub fn new_i16vec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "I16")
    }

    pub fn new_i16vec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "I16")
    }

    pub fn new_i16vec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "I16")
    }

    pub fn new_u16vec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "U16")
    }

    pub fn new_u16vec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "U16")
    }

    pub fn new_u16vec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "U16")
    }

    pub fn new_ivec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "I")
    }

    pub fn new_ivec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "I")
    }

    pub fn new_ivec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "I")
    }

    pub fn new_uvec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "U")
    }

    pub fn new_uvec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "U")
    }

    pub fn new_uvec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "U")
    }

    pub fn new_i64vec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "I64")
    }

    pub fn new_i64vec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "I64")
    }

    pub fn new_i64vec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "I64")
    }

    pub fn new_u64vec2_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(2, "U64")
    }

    pub fn new_u64vec3_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(3, "U64")
    }

    pub fn new_u64vec4_swizzle_impl() -> Self {
        Self::new_tvecn_swizzle_impl(4, "U64")
    }

    fn new_taffinen(dim: u32, scalar_t: &str) -> Self {
        ContextBuilder::new()
            .with_template("affine.rs.tera")
            .target_scalar()
            .with_scalar_t(scalar_t)
            .with_dimension(dim)
            .with_is_align(false)
    }

    pub fn new_affine2() -> Self {
        Self::new_taffinen(2, "f32")
    }

    pub fn new_affine3a() -> Self {
        Self::new_taffinen(3, "f32").with_is_align(true)
    }

    pub fn new_daffine2() -> Self {
        Self::new_taffinen(2, "f64")
    }

    pub fn new_daffine3() -> Self {
        Self::new_taffinen(3, "f64")
    }

    pub fn new_bvecn(dim: u32, scalar_t: &str) -> Self {
        ContextBuilder::new()
            .with_template("vec_mask.rs.tera")
            .with_scalar_t(scalar_t)
            .target_scalar()
            .with_dimension(dim)
    }

    pub fn new_float(scalar_t: &str) -> Self {
        ContextBuilder::new()
            .with_template("float.rs.tera")
            .with_scalar_t(scalar_t)
    }

    pub fn new_bvec2() -> Self {
        Self::new_bvecn(2, "bool")
    }

    pub fn new_bvec3() -> Self {
        Self::new_bvecn(3, "bool")
    }

    pub fn new_bvec4() -> Self {
        Self::new_bvecn(4, "bool")
    }

    pub fn new_bvec3a() -> Self {
        Self::new_bvecn(3, "u32")
    }

    pub fn new_bvec4a() -> Self {
        Self::new_bvecn(4, "u32")
    }

    pub fn new_vecn(dim: u32) -> Self {
        ContextBuilder::new()
            .with_template("vec.rs.tera")
            .target_scalar()
            .with_dimension(dim)
            .with_is_align(false)
    }

    pub fn new_vec2() -> Self {
        Self::new_vecn(2).with_scalar_t("f32")
    }

    pub fn new_vec3() -> Self {
        Self::new_vecn(3).with_scalar_t("f32")
    }

    pub fn new_vec3a() -> Self {
        Self::new_vecn(3).with_scalar_t("f32").with_is_align(true)
    }

    pub fn new_vec4() -> Self {
        Self::new_vecn(4).with_scalar_t("f32").with_is_align(true)
    }

    pub fn new_dvec2() -> Self {
        Self::new_vecn(2).with_scalar_t("f64")
    }

    pub fn new_dvec3() -> Self {
        Self::new_vecn(3).with_scalar_t("f64")
    }

    pub fn new_dvec4() -> Self {
        Self::new_vecn(4).with_scalar_t("f64")
    }

    pub fn new_i16vec2() -> Self {
        Self::new_vecn(2).with_scalar_t("i16")
    }

    pub fn new_i16vec3() -> Self {
        Self::new_vecn(3).with_scalar_t("i16")
    }

    pub fn new_i16vec4() -> Self {
        Self::new_vecn(4).with_scalar_t("i16")
    }

    pub fn new_u16vec2() -> Self {
        Self::new_vecn(2).with_scalar_t("u16")
    }

    pub fn new_u16vec3() -> Self {
        Self::new_vecn(3).with_scalar_t("u16")
    }

    pub fn new_u16vec4() -> Self {
        Self::new_vecn(4).with_scalar_t("u16")
    }

    pub fn new_ivec2() -> Self {
        Self::new_vecn(2).with_scalar_t("i32")
    }

    pub fn new_ivec3() -> Self {
        Self::new_vecn(3).with_scalar_t("i32")
    }

    pub fn new_ivec4() -> Self {
        Self::new_vecn(4).with_scalar_t("i32")
    }

    pub fn new_uvec2() -> Self {
        Self::new_vecn(2).with_scalar_t("u32")
    }

    pub fn new_uvec3() -> Self {
        Self::new_vecn(3).with_scalar_t("u32")
    }

    pub fn new_uvec4() -> Self {
        Self::new_vecn(4).with_scalar_t("u32")
    }

    pub fn new_i64vec2() -> Self {
        Self::new_vecn(2).with_scalar_t("i64")
    }

    pub fn new_i64vec3() -> Self {
        Self::new_vecn(3).with_scalar_t("i64")
    }

    pub fn new_i64vec4() -> Self {
        Self::new_vecn(4).with_scalar_t("i64")
    }

    pub fn new_u64vec2() -> Self {
        Self::new_vecn(2).with_scalar_t("u64")
    }

    pub fn new_u64vec3() -> Self {
        Self::new_vecn(3).with_scalar_t("u64")
    }

    pub fn new_u64vec4() -> Self {
        Self::new_vecn(4).with_scalar_t("u64")
    }

    pub fn new_quat() -> Self {
        ContextBuilder::new()
            .with_template("quat.rs.tera")
            .target_scalar()
            .with_scalar_t("f32")
    }

    pub fn new_dquat() -> Self {
        Self::new_quat().with_scalar_t("f64")
    }

    fn new_tmatn(dim: u32, scalar_t: &str) -> Self {
        ContextBuilder::new()
            .with_template("mat.rs.tera")
            .target_scalar()
            .with_scalar_t(scalar_t)
            .with_dimension(dim)
    }

    pub fn new_mat2() -> Self {
        Self::new_tmatn(2, "f32")
    }

    pub fn new_dmat2() -> Self {
        Self::new_tmatn(2, "f64")
    }

    pub fn new_mat3() -> Self {
        Self::new_tmatn(3, "f32")
    }

    pub fn new_mat3a() -> Self {
        Self::new_tmatn(3, "f32").with_is_align(true)
    }

    pub fn new_dmat3() -> Self {
        Self::new_tmatn(3, "f64")
    }

    pub fn new_mat4() -> Self {
        Self::new_tmatn(4, "f32")
    }

    pub fn new_dmat4() -> Self {
        Self::new_tmatn(4, "f64")
    }

    pub fn with_template(mut self, template_path: &str) -> Self {
        self.0.insert("template_path", template_path);
        self
    }

    pub fn with_scalar_t(mut self, scalar_t: &str) -> Self {
        self.0.insert("scalar_t", scalar_t);
        self
    }

    pub fn with_target(mut self, target: Target) -> Self {
        self.0.insert("is_sse2", &(target == Target::Sse2));
        self.0.insert("is_coresimd", &(target == Target::CoreSimd));
        self.0.insert("is_wasm32", &(target == Target::Wasm32));
        self.0.insert("is_neon", &(target == Target::Neon));
        self.0.insert("is_scalar", &(target == Target::Scalar));
        self
    }

    pub fn target_sse2(self) -> Self {
        self.with_target(Target::Sse2)
    }

    pub fn target_neon(self) -> Self {
        self.with_target(Target::Neon)
    }

    pub fn target_wasm32(self) -> Self {
        self.with_target(Target::Wasm32)
    }

    pub fn target_scalar(self) -> Self {
        self.with_target(Target::Scalar)
    }

    pub fn target_coresimd(self) -> Self {
        self.with_target(Target::CoreSimd)
    }

    fn with_self_t(mut self, self_t: &str) -> Self {
        self.0.insert("self_t", self_t);
        self
    }

    fn with_dimension(mut self, dim: u32) -> Self {
        self.0.insert("dim", &dim);
        self
    }

    fn with_is_align(mut self, is_align: bool) -> Self {
        self.0.insert("is_align", &is_align);
        self
    }

    fn with_key_val<T: Serialize + ?Sized, S: Into<String>>(mut self, key: S, val: &T) -> Self {
        self.0.insert(key, val);
        self
    }

    pub fn build(self) -> tera::Context {
        self.0
    }
}

pub fn build_output_pairs() -> HashMap<&'static str, tera::Context> {
    HashMap::from([
        (
            "src/swizzles/vec_traits.rs",
            ContextBuilder::new()
                .with_template("swizzle_traits.rs.tera")
                .build(),
        ),
        (
            "src/swizzles/vec2_impl.rs",
            ContextBuilder::new_vec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/vec3_impl.rs",
            ContextBuilder::new_vec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/scalar/vec3a_impl.rs",
            ContextBuilder::new_vec3a_swizzle_impl().build(),
        ),
        (
            "src/swizzles/neon/vec3a_impl.rs",
            ContextBuilder::new_vec3a_swizzle_impl().build(),
        ),
        (
            "src/swizzles/sse2/vec3a_impl.rs",
            ContextBuilder::new_vec3a_swizzle_impl()
                .target_sse2()
                .build(),
        ),
        (
            "src/swizzles/wasm32/vec3a_impl.rs",
            ContextBuilder::new_vec3a_swizzle_impl()
                .target_wasm32()
                .build(),
        ),
        (
            "src/swizzles/coresimd/vec3a_impl.rs",
            ContextBuilder::new_vec3a_swizzle_impl()
                .target_coresimd()
                .build(),
        ),
        (
            "src/swizzles/scalar/vec4_impl.rs",
            ContextBuilder::new_vec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/neon/vec4_impl.rs",
            ContextBuilder::new_vec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/sse2/vec4_impl.rs",
            ContextBuilder::new_vec4_swizzle_impl()
                .target_sse2()
                .build(),
        ),
        (
            "src/swizzles/wasm32/vec4_impl.rs",
            ContextBuilder::new_vec4_swizzle_impl()
                .target_wasm32()
                .build(),
        ),
        (
            "src/swizzles/coresimd/vec4_impl.rs",
            ContextBuilder::new_vec4_swizzle_impl()
                .target_coresimd()
                .build(),
        ),
        (
            "src/swizzles/dvec2_impl.rs",
            ContextBuilder::new_dvec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/dvec3_impl.rs",
            ContextBuilder::new_dvec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/dvec4_impl.rs",
            ContextBuilder::new_dvec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/i16vec2_impl.rs",
            ContextBuilder::new_i16vec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/i16vec3_impl.rs",
            ContextBuilder::new_i16vec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/i16vec4_impl.rs",
            ContextBuilder::new_i16vec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/u16vec2_impl.rs",
            ContextBuilder::new_u16vec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/u16vec3_impl.rs",
            ContextBuilder::new_u16vec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/u16vec4_impl.rs",
            ContextBuilder::new_u16vec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/ivec2_impl.rs",
            ContextBuilder::new_ivec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/ivec3_impl.rs",
            ContextBuilder::new_ivec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/ivec4_impl.rs",
            ContextBuilder::new_ivec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/uvec2_impl.rs",
            ContextBuilder::new_uvec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/uvec3_impl.rs",
            ContextBuilder::new_uvec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/uvec4_impl.rs",
            ContextBuilder::new_uvec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/i64vec2_impl.rs",
            ContextBuilder::new_i64vec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/i64vec3_impl.rs",
            ContextBuilder::new_i64vec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/i64vec4_impl.rs",
            ContextBuilder::new_i64vec4_swizzle_impl().build(),
        ),
        (
            "src/swizzles/u64vec2_impl.rs",
            ContextBuilder::new_u64vec2_swizzle_impl().build(),
        ),
        (
            "src/swizzles/u64vec3_impl.rs",
            ContextBuilder::new_u64vec3_swizzle_impl().build(),
        ),
        (
            "src/swizzles/u64vec4_impl.rs",
            ContextBuilder::new_u64vec4_swizzle_impl().build(),
        ),
        ("src/f32/affine2.rs", ContextBuilder::new_affine2().build()),
        (
            "src/f32/affine3a.rs",
            ContextBuilder::new_affine3a().build(),
        ),
        (
            "src/f64/daffine2.rs",
            ContextBuilder::new_daffine2().build(),
        ),
        (
            "src/f64/daffine3.rs",
            ContextBuilder::new_daffine3().build(),
        ),
        ("src/bool/bvec2.rs", ContextBuilder::new_bvec2().build()),
        ("src/bool/bvec3.rs", ContextBuilder::new_bvec3().build()),
        ("src/bool/bvec4.rs", ContextBuilder::new_bvec4().build()),
        (
            "src/bool/scalar/bvec3a.rs",
            ContextBuilder::new_bvec3a().build(),
        ),
        (
            "src/bool/sse2/bvec3a.rs",
            ContextBuilder::new_bvec3a().target_sse2().build(),
        ),
        (
            "src/bool/wasm32/bvec3a.rs",
            ContextBuilder::new_bvec3a().target_wasm32().build(),
        ),
        (
            "src/bool/neon/bvec3a.rs",
            ContextBuilder::new_bvec3a().target_neon().build(),
        ),
        (
            "src/bool/coresimd/bvec3a.rs",
            ContextBuilder::new_bvec3a().target_coresimd().build(),
        ),
        (
            "src/bool/scalar/bvec4a.rs",
            ContextBuilder::new_bvec4a().build(),
        ),
        (
            "src/bool/sse2/bvec4a.rs",
            ContextBuilder::new_bvec4a().target_sse2().build(),
        ),
        (
            "src/bool/wasm32/bvec4a.rs",
            ContextBuilder::new_bvec4a().target_wasm32().build(),
        ),
        (
            "src/bool/neon/bvec4a.rs",
            ContextBuilder::new_bvec4a().target_neon().build(),
        ),
        (
            "src/bool/coresimd/bvec4a.rs",
            ContextBuilder::new_bvec4a().target_coresimd().build(),
        ),
        ("src/f32/vec2.rs", ContextBuilder::new_vec2().build()),
        ("src/f32/vec3.rs", ContextBuilder::new_vec3().build()),
        (
            "src/f32/scalar/vec3a.rs",
            ContextBuilder::new_vec3a().build(),
        ),
        (
            "src/f32/neon/vec3a.rs",
            ContextBuilder::new_vec3a().target_neon().build(),
        ),
        (
            "src/f32/sse2/vec3a.rs",
            ContextBuilder::new_vec3a().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/vec3a.rs",
            ContextBuilder::new_vec3a().target_wasm32().build(),
        ),
        (
            "src/f32/coresimd/vec3a.rs",
            ContextBuilder::new_vec3a().target_coresimd().build(),
        ),
        ("src/f32/scalar/vec4.rs", ContextBuilder::new_vec4().build()),
        (
            "src/f32/neon/vec4.rs",
            ContextBuilder::new_vec4().target_neon().build(),
        ),
        (
            "src/f32/sse2/vec4.rs",
            ContextBuilder::new_vec4().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/vec4.rs",
            ContextBuilder::new_vec4().target_wasm32().build(),
        ),
        (
            "src/f32/coresimd/vec4.rs",
            ContextBuilder::new_vec4().target_coresimd().build(),
        ),
        ("src/f64/dvec2.rs", ContextBuilder::new_dvec2().build()),
        ("src/f64/dvec3.rs", ContextBuilder::new_dvec3().build()),
        ("src/f64/dvec4.rs", ContextBuilder::new_dvec4().build()),
        ("src/i16/i16vec2.rs", ContextBuilder::new_i16vec2().build()),
        ("src/i16/i16vec3.rs", ContextBuilder::new_i16vec3().build()),
        ("src/i16/i16vec4.rs", ContextBuilder::new_i16vec4().build()),
        ("src/u16/u16vec2.rs", ContextBuilder::new_u16vec2().build()),
        ("src/u16/u16vec3.rs", ContextBuilder::new_u16vec3().build()),
        ("src/u16/u16vec4.rs", ContextBuilder::new_u16vec4().build()),
        ("src/i32/ivec2.rs", ContextBuilder::new_ivec2().build()),
        ("src/i32/ivec3.rs", ContextBuilder::new_ivec3().build()),
        ("src/i32/ivec4.rs", ContextBuilder::new_ivec4().build()),
        ("src/u32/uvec2.rs", ContextBuilder::new_uvec2().build()),
        ("src/u32/uvec3.rs", ContextBuilder::new_uvec3().build()),
        ("src/u32/uvec4.rs", ContextBuilder::new_uvec4().build()),
        ("src/i64/i64vec2.rs", ContextBuilder::new_i64vec2().build()),
        ("src/i64/i64vec3.rs", ContextBuilder::new_i64vec3().build()),
        ("src/i64/i64vec4.rs", ContextBuilder::new_i64vec4().build()),
        ("src/u64/u64vec2.rs", ContextBuilder::new_u64vec2().build()),
        ("src/u64/u64vec3.rs", ContextBuilder::new_u64vec3().build()),
        ("src/u64/u64vec4.rs", ContextBuilder::new_u64vec4().build()),
        ("src/f32/scalar/quat.rs", ContextBuilder::new_quat().build()),
        (
            "src/f32/neon/quat.rs",
            ContextBuilder::new_quat().target_neon().build(),
        ),
        (
            "src/f32/sse2/quat.rs",
            ContextBuilder::new_quat().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/quat.rs",
            ContextBuilder::new_quat().target_wasm32().build(),
        ),
        (
            "src/f32/coresimd/quat.rs",
            ContextBuilder::new_quat().target_coresimd().build(),
        ),
        ("src/f64/dquat.rs", ContextBuilder::new_dquat().build()),
        ("src/f32/scalar/mat2.rs", ContextBuilder::new_mat2().build()),
        (
            "src/f32/neon/mat2.rs",
            ContextBuilder::new_mat2().target_neon().build(),
        ),
        (
            "src/f32/sse2/mat2.rs",
            ContextBuilder::new_mat2().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/mat2.rs",
            ContextBuilder::new_mat2().target_wasm32().build(),
        ),
        (
            "src/f32/coresimd/mat2.rs",
            ContextBuilder::new_mat2().target_coresimd().build(),
        ),
        ("src/f64/dmat2.rs", ContextBuilder::new_dmat2().build()),
        ("src/f32/mat3.rs", ContextBuilder::new_mat3().build()),
        (
            "src/f32/scalar/mat3a.rs",
            ContextBuilder::new_mat3a().build(),
        ),
        (
            "src/f32/neon/mat3a.rs",
            ContextBuilder::new_mat3a().target_neon().build(),
        ),
        (
            "src/f32/sse2/mat3a.rs",
            ContextBuilder::new_mat3a().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/mat3a.rs",
            ContextBuilder::new_mat3a().target_wasm32().build(),
        ),
        (
            "src/f32/coresimd/mat3a.rs",
            ContextBuilder::new_mat3a().target_coresimd().build(),
        ),
        ("src/f32/scalar/mat4.rs", ContextBuilder::new_mat4().build()),
        (
            "src/f32/neon/mat4.rs",
            ContextBuilder::new_mat4().target_neon().build(),
        ),
        (
            "src/f32/sse2/mat4.rs",
            ContextBuilder::new_mat4().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/mat4.rs",
            ContextBuilder::new_mat4().target_wasm32().build(),
        ),
        (
            "src/f32/coresimd/mat4.rs",
            ContextBuilder::new_mat4().target_coresimd().build(),
        ),
        ("src/f64/dmat3.rs", ContextBuilder::new_dmat3().build()),
        ("src/f64/dmat4.rs", ContextBuilder::new_dmat4().build()),
        ("src/f32/float.rs", ContextBuilder::new_float("f32").build()),
        ("src/f64/float.rs", ContextBuilder::new_float("f64").build()),
    ])
}
