use core::simd::{num::SimdFloat, *};

#[cfg(not(feature = "std"))]
pub(crate) use libm_math::*;

#[cfg(feature = "std")]
pub(crate) use std_math::*;

#[cfg(feature = "std")]
mod std_math {
    use core::simd::f32x4;
    use std::simd::StdFloat;

    #[inline]
    pub fn round3(v: f32x4) -> f32x4 {
        v.round()
    }

    #[inline]
    pub fn floor3(v: f32x4) -> f32x4 {
        v.floor()
    }

    #[inline]
    pub fn ceil3(v: f32x4) -> f32x4 {
        v.ceil()
    }

    #[inline]
    pub fn trunc3(v: f32x4) -> f32x4 {
        v.trunc()
    }

    #[inline]
    pub fn mul_add3(v: f32x4, a: f32x4, b: f32x4) -> f32x4 {
        v.mul_add(a, b)
    }

    #[inline]
    pub fn round4(v: f32x4) -> f32x4 {
        v.round()
    }

    #[inline]
    pub fn floor4(v: f32x4) -> f32x4 {
        v.floor()
    }

    #[inline]
    pub fn ceil4(v: f32x4) -> f32x4 {
        v.ceil()
    }

    #[inline]
    pub fn trunc4(v: f32x4) -> f32x4 {
        v.trunc()
    }

    #[inline]
    pub fn mul_add4(v: f32x4, a: f32x4, b: f32x4) -> f32x4 {
        v.mul_add(a, b)
    }

    #[inline]
    pub fn sqrt3(v: f32x4) -> f32x4 {
        v.sqrt()
    }

    #[inline]
    pub fn sqrt4(v: f32x4) -> f32x4 {
        v.sqrt()
    }
}

#[cfg(not(feature = "std"))]
mod libm_math {
    use crate::f32::math;
    use core::simd::f32x4;

    #[inline]
    pub fn round3(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::round(v[0]),
            math::round(v[1]),
            math::round(v[2]),
            v[3],
        ])
    }

    #[inline]
    pub fn floor3(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::floor(v[0]),
            math::floor(v[1]),
            math::floor(v[2]),
            v[3],
        ])
    }

    #[inline]
    pub fn ceil3(v: f32x4) -> f32x4 {
        f32x4::from_array([math::ceil(v[0]), math::ceil(v[1]), math::ceil(v[2]), v[3]])
    }

    #[inline]
    pub fn trunc3(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::trunc(v[0]),
            math::trunc(v[1]),
            math::trunc(v[2]),
            v[3],
        ])
    }

    #[inline]
    pub fn mul_add3(v: f32x4, a: f32x4, b: f32x4) -> f32x4 {
        f32x4::from_array([
            math::mul_add(v[0], a[0], b[0]),
            math::mul_add(v[1], a[1], b[1]),
            math::mul_add(v[2], a[2], b[2]),
            v[3],
        ])
    }

    #[inline]
    pub fn round4(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::round(v[0]),
            math::round(v[1]),
            math::round(v[2]),
            math::round(v[3]),
        ])
    }

    #[inline]
    pub fn floor4(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::floor(v[0]),
            math::floor(v[1]),
            math::floor(v[2]),
            math::floor(v[3]),
        ])
    }

    #[inline]
    pub fn ceil4(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::ceil(v[0]),
            math::ceil(v[1]),
            math::ceil(v[2]),
            math::ceil(v[3]),
        ])
    }

    #[inline]
    pub fn trunc4(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::trunc(v[0]),
            math::trunc(v[1]),
            math::trunc(v[2]),
            math::trunc(v[3]),
        ])
    }

    #[inline]
    pub fn mul_add4(v: f32x4, a: f32x4, b: f32x4) -> f32x4 {
        f32x4::from_array([
            math::mul_add(v[0], a[0], b[0]),
            math::mul_add(v[1], a[1], b[1]),
            math::mul_add(v[2], a[2], b[2]),
            math::mul_add(v[3], a[3], b[3]),
        ])
    }

    #[inline]
    pub fn sqrt3(v: f32x4) -> f32x4 {
        f32x4::from_array([math::sqrt(v[0]), math::sqrt(v[1]), math::sqrt(v[2]), v[3]])
    }

    #[inline]
    pub fn sqrt4(v: f32x4) -> f32x4 {
        f32x4::from_array([
            math::sqrt(v[0]),
            math::sqrt(v[1]),
            math::sqrt(v[2]),
            math::sqrt(v[3]),
        ])
    }
}

/// Calculates the vector 3 dot product and returns answer in x lane of f32x4.
#[inline(always)]
pub(crate) fn dot3_in_x(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let x2_y2_z2_w2 = lhs * rhs;
    let y2_0_0_0 = simd_swizzle!(x2_y2_z2_w2, [1, 0, 0, 0]);
    let z2_0_0_0 = simd_swizzle!(x2_y2_z2_w2, [2, 0, 0, 0]);
    let x2y2_0_0_0 = x2_y2_z2_w2 + y2_0_0_0;
    x2y2_0_0_0 + z2_0_0_0
}

/// Calculates the vector 4 dot product and returns answer in x lane of f32x4.
#[inline(always)]
pub(crate) fn dot4_in_x(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let x2_y2_z2_w2 = lhs * rhs;
    let z2_w2_0_0 = simd_swizzle!(x2_y2_z2_w2, [2, 3, 0, 0]);
    let x2z2_y2w2_0_0 = x2_y2_z2_w2 + z2_w2_0_0;
    let y2w2_0_0_0 = simd_swizzle!(x2z2_y2w2_0_0, [1, 0, 0, 0]);
    x2z2_y2w2_0_0 + y2w2_0_0_0
}

#[inline]
pub(crate) fn dot3(lhs: f32x4, rhs: f32x4) -> f32 {
    dot3_in_x(lhs, rhs)[0]
}

#[inline]
pub(crate) fn dot3_into_f32x4(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let dot_in_x = dot3_in_x(lhs, rhs);
    simd_swizzle!(dot_in_x, [0, 0, 0, 0])
}

#[inline]
pub(crate) fn dot4(lhs: f32x4, rhs: f32x4) -> f32 {
    dot4_in_x(lhs, rhs)[0]
}

#[inline]
pub(crate) fn dot4_into_f32x4(lhs: f32x4, rhs: f32x4) -> f32x4 {
    let dot_in_x = dot4_in_x(lhs, rhs);
    simd_swizzle!(dot_in_x, [0, 0, 0, 0])
}

#[inline(always)]
pub(crate) fn f32x4_bitand(a: f32x4, b: f32x4) -> f32x4 {
    let a = a.to_bits();
    let b = b.to_bits();
    f32x4::from_bits(a & b)
}

#[inline(always)]
pub(crate) fn f32x4_bitxor(a: f32x4, b: f32x4) -> f32x4 {
    let a = a.to_bits();
    let b = b.to_bits();
    f32x4::from_bits(a ^ b)
}
