# glam code generation

A tool to generate `glam`'s source code.

## Overview

`glam` is not a generic library, in part because some types are backed by SIMD
when it is available and also to avoid code complexity. However to support a
number of different number types combined with a number of different SIMD
architectures there is a lot of duplicate code and comments. Previously macros
were used to avoid a lot of the duplication. The down side of this was macros
were also quite complex and `glam` users often needed to navigate the macros
when viewing source or debugging. Contributors also had to find their way around
the macros to fix bugs or add features to `glam`.

This tool replaces the majority of macros in `glam` by instead generating all of
the different containers and SIMD implementations offline. These generated files
are committed to the `glam` repo and this is what is compiled, read and debugged
by `glam` users.

## Tera templates

Source files are generated using [`Tera`] templates. These kinds of templates
are typically used to generate static web pages, but here we are using them to
generate Rust source. The main advantages are a lot of the template looks like
Rust code. The templates are fairly declarative and easier to follow than the
macros (in my opinion). I try to stick to basic features of the [templating
language] to keep things simple.

[`Tera`]: https://tera.netlify.app/
[templating language]: https://tera.netlify.app/docs/

### Control variables

There `codegen` program maps a template to an output file. There are a small
number of variables that are usually set before the template is rendered to
control output:

* `scalar_t` - the container's scalar type, e.g. `f32`, `f64`, `i32`, `u32`
* `dim` - the container's dimension, e.g. `2`, `3` or `4`
* `is_scalar` - generate regular Rust code using arithmetic operators
* `is_sse2` - generate code using `sse2` intrinsics
* `is_wasm32` - generate code using `wasm32` `simd128` intrinsics

The templates for swizzles and vector masks behave slightly differently but
otherwise this is the common setup.

### Template files

The following templates are used:

* `affine.rs` - generates 2D and 3D affine transformation types
* `mat.rs` - generates all matrix types
* `quat.rs` - generates all quaternion types
* `vec.rs` - generates all vector types
* `vec_mask.rs` - generates all vector mask types
* `swizzle_traits.rs` - generates swizzle traits
* `swizzle_impl.rs` - generates impls of swizzle traits for all vector types

### Output module structure

Not all types have sse2 or wasm32 implementations, if a type does have these the
scalar and SIMD types will be in their respective modules. For example the `f32`
`Vec3` type has no SIMD implementation, but `Vec4` does, the generated source
files are structured like so:

* `src/f32/vec3.rs`
* `src/f32/scalar/vec4.rs`
* `src/f32/sse2/vec4.rs`
* `src/f32/wasm32/vec4.rs`

The appropriate implementation module will be included when `glam` is compiled.

### Template prelude

Each template starts with setting up a number of common variables based on the
inputs from the `codegen` program. Commonly used variables are:

* `self_t` - the name of the type being generated
* `inner_t` - the inner storage type used by this type (e.g. `__m128` or
  `core::storage::XYZ<f32>`)
* `deref_t` - the type used by the `Deref` and `DerefMut` implementation - not
  always the same as `inner_t`
* `col_t` - the column type - used by matrix and affine transform templates
* `quat_t` - the `scalar_t` quaternion type name
* `affine2_t` - the `scalar_t` 2D affine transform type name
* `affine3_t` - the `scalar_t` 3D affine transform type name
* `vec2_t` - the `scalar_t` 2D vector type name
* `vec3_t` - the `scalar_t` 3D vector type name
* `vec4_t` - the `scalar_t` 4D vector type name
* `mat2_t` - the `scalar_t` 2D matrix type name
* `mat3_t` - the `scalar_t` 3D matrix type name
* `mat4_t` - the `scalar_t` 4D matrix type name

## Running `codegen`

`codegen` will generate all files by default or if a glob pattern is specified,
only output files that match the glob.

The following steps will be performed for each output file:

* Check `git` status to see if the output is already modified. This is to avoid
  overriding changes by accident. The `-f` flag can be used to force override.
* Render the `Tera` template in memory
* Rust format the template output
* Write the formatted output to the output file path

### Template errors

The `Tera` error output leaves a little to be desired. If the template fails to
render with no error it is usually because a variable that doesn't exist is
being referenced somewhere in the template.

### Rust format errors

If the Rust format step fails it is usually because the template generated
invalid Rust. The `-n` flag can be used to skip the Rust format step so that the
file may be compiled which typically shows what the problem is.

### Workflow

When working on a template it is often easiest to generate one or two output
files using a glob pattern to check that template changes are working as
expected. When finished editing generate everything (most files should not
change unexpectedly) and run the test suite using `build_and_test_features.sh`
and `build_and_test_wasm32_chrome.sh`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0])
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or [http://opensource.org/licenses/MIT])

at your option.
