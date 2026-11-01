//! Signed-zero results of the `scalar-math` selection operations.
//!
//! See <https://github.com/bitshifter/glam-rs/discussions/909> for the report
//! that prompted these tests.
//!
//! The scalar implementation of `min`, `max`, `clamp` and friends compares
//! elements with `<` or `>`, so equal values (including `+0.0` and `-0.0`)
//! select the right-hand side. That is well-defined Rust, but optimizing the
//! comparisons to architecture min/max instructions gives a different zero
//! sign because those instructions have their own equality rules:
//! * AArch64 `FMIN`/`FMINNM` return `-0.0` for equal operands, while
//!   `FMAX`/`FMAXNM` return `+0.0`.
//! * x86 `MINPS`/`MAXPS` return the second operand for equal operands.
//!
//! This was the cause of the signed-zero non-determinism reported for
//! `scalar-math` + `libm`. It was an LLVM miscompile, not a property of the
//! scalar implementation: see LLVM issue
//! <https://github.com/llvm/llvm-project/issues/93414>, fixed by
//! <https://github.com/llvm/llvm-project/pull/210077> in LLVM 23 and shipped
//! in Rust 1.99.0.
//!
//! For example, with Rust 1.96.0 on AArch64 `Vec4::splat(-0.0).min(Vec4::ZERO)`
//! returned `-0.0` instead of `+0.0`.
//!
//! The tests pin the scalar comparison semantics so a future compiler
//! regression is visible rather than silently changing a sign bit. They only
//! run with the `scalar-math` feature and must be built with optimizations
//! enabled (`cargo test --release`); an unoptimized build follows the source
//! comparisons.

#![cfg(feature = "scalar-math")]

#[macro_use]
mod support;

macro_rules! impl_signed_zero_tests {
    ($module:ident, $vec:ident, $zero_bits:expr, $neg_zero_bits:expr) => {
        mod $module {
            use core::hint::black_box;
            use glam::$vec;

            // Keep one operand opaque, otherwise the compiler can constant fold
            // the whole expression. The other operand stays a constant, which
            // is what triggers the min/max instruction selection.
            fn opaque(value: $vec) -> $vec {
                black_box(value)
            }

            glam_test!(test_min_with_constant_positive_zero, {
                let neg_zero = opaque(<$vec>::splat(-0.0));
                assert_eq!(
                    neg_zero.min(<$vec>::ZERO).x.to_bits(),
                    $zero_bits,
                    "min(-0.0, +0.0) must return the right-hand +0.0"
                );
            });

            glam_test!(test_max_with_constant_negative_zero, {
                let positive_zero = opaque(<$vec>::ZERO);
                let neg_zero = <$vec>::splat(-0.0);
                assert_eq!(
                    positive_zero.max(neg_zero).x.to_bits(),
                    $neg_zero_bits,
                    "max(+0.0, -0.0) must return the right-hand -0.0"
                );
            });

            glam_test!(test_clamp_with_constant_bounds, {
                let neg_zero = opaque(<$vec>::splat(-0.0));
                assert_eq!(
                    neg_zero.clamp(-<$vec>::ONE, <$vec>::ZERO).x.to_bits(),
                    $zero_bits,
                    "clamp(-0.0, -1.0, +0.0) must return +0.0"
                );
            });

            glam_test!(test_saturate_negative_zero, {
                let neg_zero = opaque(<$vec>::splat(-0.0));
                assert_eq!(
                    neg_zero.saturate().x.to_bits(),
                    $zero_bits,
                    "saturate(-0.0) must return +0.0"
                );
            });

            glam_test!(test_select_negative_zero, {
                let neg_zero = opaque(<$vec>::splat(-0.0));
                let selected = <$vec>::select(neg_zero.cmplt(<$vec>::ZERO), neg_zero, <$vec>::ZERO);
                assert_eq!(
                    selected.x.to_bits(),
                    $zero_bits,
                    "select(-0.0 < 0.0, -0.0, +0.0) must return +0.0"
                );
            });

            glam_test!(test_min_element_negative_zero, {
                let mut value = <$vec>::ZERO;
                value.x = black_box(-0.0);
                assert_eq!(
                    value.min_element().to_bits(),
                    $zero_bits,
                    "min_element([-0.0, +0.0, ...]) must return +0.0"
                );
            });

            glam_test!(test_max_element_negative_zero, {
                let mut value = <$vec>::splat(-0.0);
                value.x = black_box(0.0);
                assert_eq!(
                    value.max_element().to_bits(),
                    $neg_zero_bits,
                    "max_element([+0.0, -0.0, ...]) must return -0.0"
                );
            });
        }
    };
}

impl_signed_zero_tests!(vec2, Vec2, 0x0000_0000, 0x8000_0000);
impl_signed_zero_tests!(vec3, Vec3, 0x0000_0000, 0x8000_0000);
impl_signed_zero_tests!(vec3a, Vec3A, 0x0000_0000, 0x8000_0000);
impl_signed_zero_tests!(vec4, Vec4, 0x0000_0000, 0x8000_0000);

#[cfg(feature = "f64")]
impl_signed_zero_tests!(dvec2, DVec2, 0x0000_0000_0000_0000, 0x8000_0000_0000_0000);
#[cfg(feature = "f64")]
impl_signed_zero_tests!(dvec3, DVec3, 0x0000_0000_0000_0000, 0x8000_0000_0000_0000);
#[cfg(feature = "f64")]
impl_signed_zero_tests!(dvec4, DVec4, 0x0000_0000_0000_0000, 0x8000_0000_0000_0000);
