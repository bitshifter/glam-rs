use anyhow::bail;
use clap::{arg, command};
use rustfmt_wrapper::rustfmt;
use serde::ser::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const GLAM_ROOT: &str = "..";

fn is_modified(repo: &git2::Repository, output_path: &str) -> anyhow::Result<bool> {
    match repo.status_file(Path::new(output_path)) {
        Ok(status) => {
            return Ok(status.is_wt_modified());
            // if status.is_wt_modified() {
            //     bail!("{} is already modified, revert or stash your changes.", output_path);
            // }
        }
        Err(e) => {
            if e.code() == git2::ErrorCode::NotFound {
                return Ok(false);
            } else {
                bail!("git file status failed for {}: {}", output_path, e);
            }
        }
    }
}

fn generate_file(
    tera: &tera::Tera,
    context: &tera::Context,
    template_path: &str,
) -> anyhow::Result<String> {
    let buffer = match tera.render(template_path, &context) {
        Ok(output) => output,
        Err(e) => {
            bail!("tera error encountered: {}", e);
        }
    };

    Ok(buffer)
}

struct ContextBuilder(tera::Context);

impl ContextBuilder {
    pub fn new() -> Self {
        Self(tera::Context::new())
    }

    fn new_tvecn_swizzle_impl(dim: u32, prefix: &str) -> Self {
        ContextBuilder::new()
            .with_template("swizzle_impl.rs")
            .target_scalar()
            .with_key_val("vec2_t", &format!("{}Vec2", prefix))
            .with_key_val("vec3_t", &format!("{}Vec3", prefix))
            .with_key_val("vec4_t", &format!("{}Vec4", prefix))
            .with_self_t(&format!("{}Vec{}", prefix, dim))
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

    fn new_taffinen(dim: u32, scalar_t: &str) -> Self {
        ContextBuilder::new()
            .with_template("affine.rs")
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

    pub fn new_bvecn(dim: u32) -> Self {
        ContextBuilder::new()
            .with_template("vec_mask.rs")
            .target_scalar()
            .with_dimension(dim)
    }

    pub fn new_bvec2() -> Self {
        Self::new_bvecn(2)
    }

    pub fn new_bvec3() -> Self {
        Self::new_bvecn(3)
    }

    pub fn new_bvec4() -> Self {
        Self::new_bvecn(4)
    }

    pub fn new_vecn(dim: u32) -> Self {
        ContextBuilder::new()
            .with_template("vec.rs")
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
        Self::new_vecn(4).with_scalar_t("f32")
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

    pub fn new_quat() -> Self {
        ContextBuilder::new()
            .with_template("quat.rs")
            .target_scalar()
            .with_scalar_t("f32")
    }

    pub fn new_dquat() -> Self {
        Self::new_quat().with_scalar_t("f64")
    }

    fn new_tmatn(dim: u32, scalar_t: &str) -> Self {
        ContextBuilder::new()
            .with_template("mat.rs")
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

    pub fn target_sse2(mut self) -> Self {
        self.0.insert("is_sse2", &true);
        self.0.insert("is_wasm32", &false);
        self.0.insert("is_scalar", &false);
        self
    }

    pub fn target_wasm32(mut self) -> Self {
        self.0.insert("is_sse2", &false);
        self.0.insert("is_wasm32", &true);
        self.0.insert("is_scalar", &false);
        self
    }

    pub fn target_scalar(mut self) -> Self {
        self.0.insert("is_sse2", &false);
        self.0.insert("is_wasm32", &false);
        self.0.insert("is_scalar", &true);
        self
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

fn build_output_pairs() -> HashMap<&'static str, tera::Context> {
    HashMap::from([
        (
            "src/swizzles/vec_traits.rs",
            ContextBuilder::new()
                .with_template("swizzle_traits.rs")
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
            "src/swizzles/scalar/vec4_impl.rs",
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
            "src/bool/sse2/bvec3a.rs",
            ContextBuilder::new_bvec3().target_sse2().build(),
        ),
        (
            "src/bool/wasm32/bvec3a.rs",
            ContextBuilder::new_bvec3().target_wasm32().build(),
        ),
        (
            "src/bool/sse2/bvec4a.rs",
            ContextBuilder::new_bvec4().target_sse2().build(),
        ),
        (
            "src/bool/wasm32/bvec4a.rs",
            ContextBuilder::new_bvec4().target_wasm32().build(),
        ),
        ("src/f32/vec2.rs", ContextBuilder::new_vec2().build()),
        ("src/f32/vec3.rs", ContextBuilder::new_vec3().build()),
        (
            "src/f32/scalar/vec3a.rs",
            ContextBuilder::new_vec3a().build(),
        ),
        (
            "src/f32/sse2/vec3a.rs",
            ContextBuilder::new_vec3a().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/vec3a.rs",
            ContextBuilder::new_vec3a().target_wasm32().build(),
        ),
        ("src/f32/scalar/vec4.rs", ContextBuilder::new_vec4().build()),
        (
            "src/f32/sse2/vec4.rs",
            ContextBuilder::new_vec4().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/vec4.rs",
            ContextBuilder::new_vec4().target_wasm32().build(),
        ),
        ("src/f64/dvec2.rs", ContextBuilder::new_dvec2().build()),
        ("src/f64/dvec3.rs", ContextBuilder::new_dvec3().build()),
        ("src/f64/dvec4.rs", ContextBuilder::new_dvec4().build()),
        ("src/i32/ivec2.rs", ContextBuilder::new_ivec2().build()),
        ("src/i32/ivec3.rs", ContextBuilder::new_ivec3().build()),
        ("src/i32/ivec4.rs", ContextBuilder::new_ivec4().build()),
        ("src/u32/uvec2.rs", ContextBuilder::new_uvec2().build()),
        ("src/u32/uvec3.rs", ContextBuilder::new_uvec3().build()),
        ("src/u32/uvec4.rs", ContextBuilder::new_uvec4().build()),
        ("src/f32/scalar/quat.rs", ContextBuilder::new_quat().build()),
        (
            "src/f32/sse2/quat.rs",
            ContextBuilder::new_quat().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/quat.rs",
            ContextBuilder::new_quat().target_wasm32().build(),
        ),
        ("src/f64/dquat.rs", ContextBuilder::new_dquat().build()),
        ("src/f32/scalar/mat2.rs", ContextBuilder::new_mat2().build()),
        (
            "src/f32/sse2/mat2.rs",
            ContextBuilder::new_mat2().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/mat2.rs",
            ContextBuilder::new_mat2().target_wasm32().build(),
        ),
        ("src/f64/dmat2.rs", ContextBuilder::new_dmat2().build()),
        ("src/f32/mat3.rs", ContextBuilder::new_mat3().build()),
        (
            "src/f32/scalar/mat3a.rs",
            ContextBuilder::new_mat3a().build(),
        ),
        (
            "src/f32/sse2/mat3a.rs",
            ContextBuilder::new_mat3a().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/mat3a.rs",
            ContextBuilder::new_mat3a().target_wasm32().build(),
        ),
        ("src/f32/scalar/mat4.rs", ContextBuilder::new_mat4().build()),
        (
            "src/f32/sse2/mat4.rs",
            ContextBuilder::new_mat4().target_sse2().build(),
        ),
        (
            "src/f32/wasm32/mat4.rs",
            ContextBuilder::new_mat4().target_wasm32().build(),
        ),
        ("src/f64/dmat3.rs", ContextBuilder::new_dmat3().build()),
        ("src/f64/dmat4.rs", ContextBuilder::new_dmat4().build()),
    ])
}

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(arg!([GLOB]))
        .arg(arg!(-f - -force))
        .arg(arg!(-s - -stdout))
        .arg(arg!(-n - -nofmt))
        .get_matches();

    let force = matches.is_present("force");
    let stdout = matches.is_present("stdout");
    let fmt_output = !matches.is_present("nofmt");
    let output_path_glob = matches.value_of("GLOB");

    if stdout && output_path_glob.is_none() {
        bail!("specify a single file to output to stdout.");
    }

    let glob = if let Some(output_path_glob) = output_path_glob {
        match globset::Glob::new(output_path_glob) {
            Ok(glob) => Some(glob.compile_matcher()),
            Err(e) => bail!("failed to compile glob: {}", e),
        }
    } else {
        None
    };
    let tera = match tera::Tera::new("templates/**/*.rs") {
        Ok(t) => t,
        Err(e) => bail!("Parsing error(s): {}", e),
    };

    let repo = match git2::Repository::open(GLAM_ROOT) {
        Ok(repo) => repo,
        Err(e) => bail!("failed to open git repo: {}", e),
    };
    let workdir = repo.workdir().unwrap();

    let output_pairs = build_output_pairs();

    let mut output_paths = vec![];
    if let Some(glob) = glob {
        for k in output_pairs.keys() {
            if glob.is_match(k) {
                output_paths.push(k)
            }
        }
        if output_paths.is_empty() {
            bail!("no matching output paths found for '{}'.", glob.glob());
        };
    } else {
        for k in output_pairs.keys() {
            output_paths.push(k)
        }
    };

    output_paths.sort();

    for output_path in output_paths {
        println!("generating {}", output_path);
        let context = output_pairs.get(output_path).unwrap();
        let template_path = context.get("template_path").unwrap().as_str().unwrap();

        if !(force || stdout) && is_modified(&repo, output_path)? {
            bail!(
                "{} is already modified, use  `-f` to force overwrite or revert local changes.",
                output_path
            );
        }

        let mut output_str = generate_file(&tera, &context, template_path)?;

        if fmt_output {
            match rustfmt(&output_str) {
                Ok(output) => output_str = output,
                Err(e) => {
                    bail!("rustfmt error encountered: {}", e);
                }
            }
        }

        if stdout {
            print!("{}", output_str);
            continue;
        }

        let full_output_path = workdir.join(output_path);
        let mut output_file = match File::create(&full_output_path) {
            Ok(file) => file,
            Err(e) => {
                bail!("failed to create {}: {}", full_output_path.display(), e);
            }
        };

        match write!(output_file, "{}", output_str) {
            Err(e) => {
                bail!("failed to write output: {}", e);
            }
            Ok(()) => (),
        }
    }

    Ok(())
}
