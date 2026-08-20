//! Math functions for `f16` that are not provided by the `half` crate.
//!
//! These functions convert to `f32` and round-trip through the `f32` math
//! implementations. This avoids a dependency on `libm` or `num-traits` and
//! works in `no_std` environments.

use half::f16;

#[inline(always)]
pub(crate) fn abs(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::abs(f32::from(a)))
}

#[inline(always)]
pub(crate) fn acos_approx(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::acos_approx(f32::from(a)))
}

#[inline(always)]
pub(crate) fn atan2(a: f16, b: f16) -> f16 {
    f16::from_f32(crate::f32::math::atan2(f32::from(a), f32::from(b)))
}

#[inline(always)]
pub(crate) fn ceil(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::ceil(f32::from(a)))
}

#[inline(always)]
pub(crate) fn copysign(a: f16, b: f16) -> f16 {
    f16::from_f32(crate::f32::math::copysign(f32::from(a), f32::from(b)))
}

#[inline(always)]
pub(crate) fn cos(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::cos(f32::from(a)))
}

#[inline(always)]
pub(crate) fn div_euclid(a: f16, b: f16) -> f16 {
    f16::from_f32(crate::f32::math::div_euclid(f32::from(a), f32::from(b)))
}

#[inline(always)]
pub(crate) fn exp(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::exp(f32::from(a)))
}

#[inline(always)]
pub(crate) fn exp2(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::exp2(f32::from(a)))
}

#[inline(always)]
pub(crate) fn floor(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::floor(f32::from(a)))
}

#[inline(always)]
pub(crate) fn ln(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::ln(f32::from(a)))
}

#[inline(always)]
pub(crate) fn log2(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::log2(f32::from(a)))
}

#[inline(always)]
pub(crate) fn mul_add(a: f16, b: f16, c: f16) -> f16 {
    f16::from_f32(crate::f32::math::mul_add(
        f32::from(a),
        f32::from(b),
        f32::from(c),
    ))
}

#[inline(always)]
pub(crate) fn powf(a: f16, b: f16) -> f16 {
    f16::from_f32(crate::f32::math::powf(f32::from(a), f32::from(b)))
}

#[inline(always)]
pub(crate) fn rem_euclid(a: f16, b: f16) -> f16 {
    f16::from_f32(crate::f32::math::rem_euclid(f32::from(a), f32::from(b)))
}

#[inline(always)]
pub(crate) fn round(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::round(f32::from(a)))
}

#[inline(always)]
pub(crate) fn signum(a: f16) -> f16 {
    if a.is_nan() {
        f16::NAN
    } else {
        f16::from_f32(crate::f32::math::signum(f32::from(a)))
    }
}

#[inline(always)]
pub(crate) fn sin(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::sin(f32::from(a)))
}

#[inline(always)]
pub(crate) fn sin_cos(a: f16) -> (f16, f16) {
    let (s, c) = crate::f32::math::sin_cos(f32::from(a));
    (f16::from_f32(s), f16::from_f32(c))
}

#[inline(always)]
pub(crate) fn sqrt(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::sqrt(f32::from(a)))
}

#[allow(unused)]
#[inline(always)]
pub(crate) fn tan(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::tan(f32::from(a)))
}

#[inline(always)]
pub(crate) fn trunc(a: f16) -> f16 {
    f16::from_f32(crate::f32::math::trunc(f32::from(a)))
}
