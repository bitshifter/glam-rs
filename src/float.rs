use core::ops::{Div, Neg};

/// Trait that provides all the math methods that we need from std.
/// This is private because it's too easy to silently end up using the std methods silently if both
/// std and libm are enabled.
pub(crate) trait Float: Copy + PartialEq + Neg<Output = Self> + Div<Output = Self> {
    const ONE: Self;
    const NAN: Self;
    #[inline]
    fn abs(self) -> Self {
        // libm doesn't define abs, so proviae a default implementation here
        if self.is_sign_positive() {
            return self;
        }
        if self.is_sign_negative() {
            return -self;
        }
        Self::NAN
    }
    fn acos_clamped(self) -> Self;
    #[inline(always)]
    fn acos_approx(self) -> Self {
        Self::acos_clamped(self)
    }
    fn asin_clamped(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn tan(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn sqrt(self) -> Self;
    #[inline]
    #[deny(clippy::eq_op)]
    fn is_nan(self) -> bool {
        self != self
    }
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn copysign(self, sign: Self) -> Self;
    fn round(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn exp(self) -> Self;
    fn powf(self, n: Self) -> Self;
    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else {
            Self::ONE.copysign(self)
        }
    }
}

#[inline(always)]
fn is_sign_negative_f32(v: f32) -> bool {
    let bits: u32 = v.to_bits();
    bits >> 31 != 0
}

#[inline(always)]
fn is_sign_positive_f32(v: f32) -> bool {
    let bits: u32 = v.to_bits();
    bits >> 31 == 0
}

#[inline(always)]
fn is_sign_negative_f64(v: f64) -> bool {
    let bits: u64 = v.to_bits();
    bits >> 63 != 0
}

#[inline(always)]
fn is_sign_positive_f64(v: f64) -> bool {
    let bits: u64 = v.to_bits();
    bits >> 63 == 0
}

/// Returns a very close approximation of `self.clamp(-1.0, 1.0).acos()`.
#[inline]
fn acos_approx_f32(v: f32) -> f32 {
    // Based on https://github.com/microsoft/DirectXMath `XMScalarAcos`
    // Clamp input to [-1,1].
    let nonnegative = v >= 0.0;
    let x = abs(v);
    let mut omx = 1.0 - x;
    if omx < 0.0 {
        omx = 0.0;
    }
    let root = sqrt(omx);

    // 7-degree minimax approximation
    #[allow(clippy::approx_constant)]
    let mut result =
        ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x + 0.030_891_88) * x
            - 0.050_174_303)
            * x
            + 0.088_978_99)
            * x
            - 0.214_598_8)
            * x
            + 1.570_796_3;
    result *= root;

    // acos(x) = pi - acos(-x) when x < 0
    if nonnegative {
        result
    } else {
        core::f32::consts::PI - result
    }
}

#[cfg(feature = "libm")]
impl Float for f32 {
    const ONE: Self = 1.0;
    const NAN: Self = f32::NAN;
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f32(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f32(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        libm::copysignf(self, sign)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        libm::ceilf(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        libm::floorf(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        libm::roundf(self)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        libm::expf(self)
    }
    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        libm::powf(self, n)
    }
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        libm::fmaf(self, a, b)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        libm::acosf(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        acos_approx_f32(self)
    }
    #[inline(always)]
    fn asin_clamped(self) -> Self {
        libm::asinf(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cosf(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        libm::sinf(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincosf(self)
    }
    #[inline(always)]
    fn tan(self) -> Self {
        libm::tanf(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
}

#[cfg(feature = "libm")]
impl Float for f64 {
    const ONE: Self = 1.0;
    const NAN: Self = f64::NAN;
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f64(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f64(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        libm::copysign(self, sign)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        libm::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        libm::floor(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        libm::round(self)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        libm::exp(self)
    }
    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        libm::pow(self, n)
    }
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        libm::fma(self, a, b)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        libm::acos(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn asin_clamped(self) -> Self {
        libm::asin(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        libm::atan2(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cos(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        libm::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincos(self)
    }
    #[inline(always)]
    fn tan(self) -> Self {
        libm::tan(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
}

#[cfg(not(feature = "libm"))]
impl Float for f32 {
    const ONE: Self = 1.0;
    const NAN: Self = f32::NAN;
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f32(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f32(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        f32::copysign(self, sign)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        f32::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        f32::floor(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        f32::round(self)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        f32::exp(self)
    }
    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        f32::powf(self, n)
    }
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        f32::acos(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        acos_approx_f32(self)
    }
    #[inline(always)]
    fn asin_clamped(self) -> Self {
        f32::asin(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        f32::atan2(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        f32::cos(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        f32::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        f32::sin_cos(self)
    }
    #[inline(always)]
    fn tan(self) -> Self {
        f32::tan(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

#[cfg(not(feature = "libm"))]
impl Float for f64 {
    const ONE: Self = 1.0;
    const NAN: Self = f64::NAN;
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f64(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f64(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        f64::copysign(self, sign)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        f64::ceil(self)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        f64::floor(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        f64::round(self)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        f64::exp(self)
    }
    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        f64::powf(self, n)
    }
    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        f64::mul_add(self, a, b)
    }
    #[inline(always)]
    fn asin_clamped(self) -> Self {
        f64::asin(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        f64::atan2(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        f64::cos(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        f64::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        f64::sin_cos(self)
    }
    #[inline(always)]
    fn tan(self) -> Self {
        f64::tan(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        f64::acos(self)
    }
}

#[inline(always)]
pub(crate) fn abs<T: Float>(f: T) -> T {
    Float::abs(f)
}

#[inline(always)]
pub(crate) fn acos_approx<T: Float>(f: T) -> T {
    Float::acos_approx(f)
}

#[inline(always)]
pub(crate) fn asin_clamped<T: Float>(f: T) -> T {
    Float::asin_clamped(f)
}

#[inline(always)]
pub(crate) fn atan2<T: Float>(f: T, other: T) -> T {
    Float::atan2(f, other)
}

#[inline(always)]
pub(crate) fn sin<T: Float>(f: T) -> T {
    Float::sin(f)
}

// #[inline(always)]
// pub(crate) fn cos<T: Float>(f: T) -> T {
//     Float::cos(f)
// }

#[inline(always)]
pub(crate) fn sin_cos<T: Float>(f: T) -> (T, T) {
    Float::sin_cos(f)
}

#[inline(always)]
pub(crate) fn tan<T: Float>(f: T) -> T {
    Float::tan(f)
}

#[inline(always)]
pub(crate) fn sqrt<T: Float>(f: T) -> T {
    Float::sqrt(f)
}

#[inline(always)]
pub(crate) fn copysign<T: Float>(f: T, sign: T) -> T {
    Float::copysign(f, sign)
}

#[inline(always)]
pub(crate) fn signum<T: Float>(f: T) -> T {
    Float::signum(f)
}

#[inline(always)]
pub(crate) fn round<T: Float>(f: T) -> T {
    Float::round(f)
}

#[inline(always)]
pub(crate) fn ceil<T: Float>(f: T) -> T {
    Float::ceil(f)
}

#[inline(always)]
pub(crate) fn floor<T: Float>(f: T) -> T {
    Float::floor(f)
}

#[inline(always)]
pub(crate) fn exp<T: Float>(f: T) -> T {
    Float::exp(f)
}

#[inline(always)]
pub(crate) fn powf<T: Float>(f: T, n: T) -> T {
    Float::powf(f, n)
}

#[inline(always)]
pub(crate) fn mul_add<T: Float>(a: T, b: T, c: T) -> T {
    Float::mul_add(a, b, c)
}
