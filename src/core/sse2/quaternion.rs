#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use super::float::*;
use crate::const_m128;
use crate::core::{
    storage::XYZ,
    traits::{quaternion::Quaternion, scalar::*, vector::*},
};

impl Quaternion<f32> for __m128 {
    type SIMDVector3 = __m128;

    #[inline(always)]
    fn conjugate(self) -> Self {
        const SIGN: __m128 = const_m128!([-0.0, -0.0, -0.0, 0.0]);
        unsafe { _mm_xor_ps(self, SIGN) }
    }

    #[inline]
    fn lerp(self, end: Self, s: f32) -> Self {
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        unsafe {
            const NEG_ZERO: __m128 = const_m128!([-0.0; 4]);
            let start = self;
            let end = end;
            let dot = Vector4::dot_into_vec(start, end);
            // Calculate the bias, if the dot product is positive or zero, there is no bias
            // but if it is negative, we want to flip the 'end' rotation XYZW components
            let bias = _mm_and_ps(dot, NEG_ZERO);
            let interpolated = _mm_add_ps(
                _mm_mul_ps(_mm_sub_ps(_mm_xor_ps(end, bias), start), _mm_set_ps1(s)),
                start,
            );
            FloatVector4::normalize(interpolated)
        }
    }

    #[inline]
    fn slerp(self, end: Self, s: f32) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(FloatVector4::is_normalized(self));
        glam_assert!(FloatVector4::is_normalized(end));

        const DOT_THRESHOLD: f32 = 0.9995;

        let dot = Vector4::dot(self, end);

        if dot > DOT_THRESHOLD {
            // assumes lerp returns a normalized quaternion
            self.lerp(end, s)
        } else {
            // assumes scalar_acos clamps the input to [-1.0, 1.0]
            let theta = dot.acos_approx();

            let x = 1.0 - s;
            let y = s;
            let z = 1.0;

            unsafe {
                let tmp = _mm_mul_ps(_mm_set_ps1(theta), _mm_set_ps(0.0, z, y, x));
                let tmp = m128_sin(tmp);

                let scale1 = _mm_shuffle_ps(tmp, tmp, 0b00_00_00_00);
                let scale2 = _mm_shuffle_ps(tmp, tmp, 0b01_01_01_01);
                let theta_sin = _mm_shuffle_ps(tmp, tmp, 0b10_10_10_10);

                let theta_sin_recip = _mm_rcp_ps(theta_sin);

                self.mul(scale1).add(end.mul(scale2)).mul(theta_sin_recip)
            }
        }
    }

    #[inline]
    fn mul_quaternion(self, other: Self) -> Self {
        unsafe {
            let lhs = self;
            let rhs = other;

            const MASK_PNPN: __m128 = const_m128!([0.0, -0.0, 0.0, -0.0]);
            const MASK_PPNN: __m128 = const_m128!([0.0, 0.0, -0.0, -0.0]);
            const MASK_NPPN: __m128 = const_m128!([-0.0, 0.0, 0.0, -0.0]);

            let lhs_xxxx = _mm_shuffle_ps(lhs, lhs, 0b00_00_00_00);
            let lhs_yyyy = _mm_shuffle_ps(lhs, lhs, 0b01_01_01_01);
            let lhs_zzzz = _mm_shuffle_ps(lhs, lhs, 0b10_10_10_10);
            let lhs_wwww = _mm_shuffle_ps(lhs, lhs, 0b11_11_11_11);
            let rhs_wzyx = _mm_shuffle_ps(rhs, rhs, 0b00_01_10_11);
            let rhs_zwxy = _mm_shuffle_ps(rhs, rhs, 0b01_00_11_10);
            let rhs_yxwz = _mm_shuffle_ps(rhs, rhs, 0b10_11_00_01);
            let wwww_xyzw = _mm_mul_ps(lhs_wwww, rhs);
            let xxxx_wzyx = _mm_mul_ps(lhs_xxxx, rhs_wzyx);
            let yyyy_zwxy = _mm_mul_ps(lhs_yyyy, rhs_zwxy);
            let zzzz_yxwz = _mm_mul_ps(lhs_zzzz, rhs_yxwz);
            let xxxx_wnzynx = _mm_xor_ps(xxxx_wzyx, MASK_PNPN);
            let yyyy_zwnxny = _mm_xor_ps(yyyy_zwxy, MASK_PPNN);
            let zzzz_nyxwnz = _mm_xor_ps(zzzz_yxwz, MASK_NPPN);

            _mm_add_ps(
                _mm_add_ps(wwww_xyzw, xxxx_wnzynx),
                _mm_add_ps(yyyy_zwnxny, zzzz_nyxwnz),
            )
        }
    }

    #[inline]
    fn mul_vector3(self, other: XYZ<f32>) -> XYZ<f32> {
        self.mul_float4_as_vector3(other.into()).into()
    }

    #[inline]
    fn mul_float4_as_vector3(self, other: __m128) -> __m128 {
        glam_assert!(FloatVector4::is_normalized(self));
        unsafe {
            const TWO: __m128 = const_m128!([2.0; 4]);
            let w = _mm_shuffle_ps(self, self, 0b11_11_11_11);
            let b = self;
            let b2 = Vector3::dot_into_vec(b, b);
            other
                .mul(w.mul(w).sub(b2))
                .add(b.mul(Vector3::dot_into_vec(other, b).mul(TWO)))
                .add(b.cross(other).mul(w.mul(TWO)))
        }
    }
}
