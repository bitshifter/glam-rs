#[cfg(feature = "libm")]
mod libm_math {
    #[inline(always)]
    pub(crate) fn abs(f: f64) -> f64 {
        libm::fabs(f)
    }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f64) -> f64 {
        libm::acos(f.clamp(-1.0, 1.0))
    }

    #[inline(always)]
    pub(crate) fn asin_clamped(f: f64) -> f64 {
        libm::asin(f.clamp(-1.0, 1.0))
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f64, other: f64) -> f64 {
        libm::atan2(f, other)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f64) -> f64 {
        libm::sin(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f64) -> (f64, f64) {
        libm::sincos(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f64) -> f64 {
        libm::tan(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f64) -> f64 {
        libm::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f64, sign: f64) -> f64 {
        libm::copysign(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f64) -> f64 {
        if f.is_nan() {
            f64::NAN
        } else {
            copysign(1.0, f)
        }
    }

    #[inline(always)]
    pub(crate) fn round(f: f64) -> f64 {
        libm::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f64) -> f64 {
        libm::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f64) -> f64 {
        libm::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f64) -> f64 {
        libm::floor(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f64) -> f64 {
        libm::exp(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f64, n: f64) -> f64 {
        libm::pow(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 {
        libm::fma(a, b, c)
    }
}

#[cfg(not(feature = "libm"))]
mod std_math {
    #[inline(always)]
    pub(crate) fn abs(f: f64) -> f64 {
        f64::abs(f)
    }

    #[inline(always)]
    pub(crate) fn acos_approx(f: f64) -> f64 {
        f64::acos(f64::clamp(f, -1.0, 1.0))
    }

    #[inline(always)]
    pub(crate) fn asin_clamped(f: f64) -> f64 {
        f64::asin(f64::clamp(f, -1.0, 1.0))
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f64, other: f64) -> f64 {
        f64::atan2(f, other)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f64) -> f64 {
        f64::sin(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f64) -> (f64, f64) {
        f64::sin_cos(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f64) -> f64 {
        f64::tan(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f64) -> f64 {
        f64::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f64, sign: f64) -> f64 {
        f64::copysign(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f64) -> f64 {
        f64::signum(f)
    }

    #[inline(always)]
    pub(crate) fn round(f: f64) -> f64 {
        f64::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f64) -> f64 {
        f64::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f64) -> f64 {
        f64::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f64) -> f64 {
        f64::floor(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f64) -> f64 {
        f64::exp(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f64, n: f64) -> f64 {
        f64::powf(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f64, b: f64, c: f64) -> f64 {
        f64::mul_add(a, b, c)
    }
}

#[cfg(feature = "libm")]
pub(crate) use libm_math::*;

#[cfg(not(feature = "libm"))]
pub(crate) use std_math::*;
