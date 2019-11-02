// #[cfg(target_arch = "x86")]
// use std::arch::x86::*;
// #[cfg(target_arch = "x86_64")]
// use std::arch::x86_64::*;

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// use super::x86_utils::UnionCast;

// macro_rules! _ps_const_ty {
//     ($name:ident, $field:ident, $x:expr) => {
//         #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//         const $name: UnionCast = UnionCast {
//             $field: [$x, $x, $x, $x],
//         };
//     };
// }

// _ps_const_ty!(PS_INV_SIGN_MASK, u32x4, !0x8000_0000);
// _ps_const_ty!(PS_SIGN_MASK, u32x4, 0x8000_0000);

// _ps_const_ty!(PS_1_0, f32x4, 1.0);
// _ps_const_ty!(PS_0_5, f32x4, 0.5);

// _ps_const_ty!(PI32_1, i32x4, 1);
// _ps_const_ty!(PI32_INV_1, i32x4, !1);
// _ps_const_ty!(PI32_2, i32x4, 2);
// _ps_const_ty!(PI32_4, i32x4, 4);

// _ps_const_ty!(PS_MINUS_CEPHES_DP1, f32x4, -0.785_156_25);
// _ps_const_ty!(PS_MINUS_CEPHES_DP2, f32x4, -2.418_756_5e-4);
// _ps_const_ty!(PS_MINUS_CEPHES_DP3, f32x4, -3.774_895e-8);
// _ps_const_ty!(PS_SINCOF_P0, f32x4, -1.951_529_6e-4);
// _ps_const_ty!(PS_SINCOF_P1, f32x4, 8.332_161e-3);
// _ps_const_ty!(PS_SINCOF_P2, f32x4, -1.666_665_5e-1);
// _ps_const_ty!(PS_COSCOF_P0, f32x4, 2.443_315_7e-5);
// _ps_const_ty!(PS_COSCOF_P1, f32x4, -1.388_731_6E-3);
// _ps_const_ty!(PS_COSCOF_P2, f32x4, 4.166_664_6e-2);
// _ps_const_ty!(PS_CEPHES_FOPI, f32x4, 1.273_239_5); // 4 / M_PI

#[inline]
pub(crate) fn scalar_sin_cos(x: f32) -> (f32, f32) {
    // // expect sse2 to be available on all x86 builds
    // #[cfg(target_feature = "sse2")]
    // unsafe {
    //     let (sinx, cosx) = sin_cos_sse2(_mm_set1_ps(x));
    //     (_mm_cvtss_f32(sinx), _mm_cvtss_f32(cosx))
    // }
    // #[cfg(not(target_feature = "sse2"))]
    x.sin_cos()
}

// Based on http://gruntthepeon.free.fr/ssemath/sse_mathfun.h
// #[cfg(target_feature = "sse2")]
// unsafe fn sin_cos_sse2(x: __m128) -> (__m128, __m128) {
//     let mut sign_bit_sin = x;
//     // take the absolute value
//     let mut x = _mm_and_ps(x, PS_INV_SIGN_MASK.m128);
//     // extract the sign bit (upper one)
//     sign_bit_sin = _mm_and_ps(sign_bit_sin, PS_SIGN_MASK.m128);

//     // scale by 4/Pi
//     let mut y = _mm_mul_ps(x, PS_CEPHES_FOPI.m128);

//     // store the integer part of y in emm2
//     let mut emm2 = _mm_cvttps_epi32(y);

//     // j=(j+1) & (~1) (see the cephes sources)
//     emm2 = _mm_add_epi32(emm2, PI32_1.m128i);
//     emm2 = _mm_and_si128(emm2, PI32_INV_1.m128i);
//     y = _mm_cvtepi32_ps(emm2);

//     let mut emm4 = emm2;

//     /* get the swap sign flag for the sine */
//     let mut emm0 = _mm_and_si128(emm2, PI32_4.m128i);
//     emm0 = _mm_slli_epi32(emm0, 29);
//     let swap_sign_bit_sin = _mm_castsi128_ps(emm0);

//     /* get the polynom selection mask for the sine*/
//     emm2 = _mm_and_si128(emm2, PI32_2.m128i);
//     emm2 = _mm_cmpeq_epi32(emm2, _mm_setzero_si128());
//     let poly_mask = _mm_castsi128_ps(emm2);

//     /* The magic pass: "Extended precision modular arithmetic"
//     x = ((x - y * DP1) - y * DP2) - y * DP3; */
//     let mut xmm1 = PS_MINUS_CEPHES_DP1.m128;
//     let mut xmm2 = PS_MINUS_CEPHES_DP2.m128;
//     let mut xmm3 = PS_MINUS_CEPHES_DP3.m128;
//     xmm1 = _mm_mul_ps(y, xmm1);
//     xmm2 = _mm_mul_ps(y, xmm2);
//     xmm3 = _mm_mul_ps(y, xmm3);
//     x = _mm_add_ps(x, xmm1);
//     x = _mm_add_ps(x, xmm2);
//     x = _mm_add_ps(x, xmm3);

//     emm4 = _mm_sub_epi32(emm4, PI32_2.m128i);
//     emm4 = _mm_andnot_si128(emm4, PI32_4.m128i);
//     emm4 = _mm_slli_epi32(emm4, 29);
//     let sign_bit_cos = _mm_castsi128_ps(emm4);

//     sign_bit_sin = _mm_xor_ps(sign_bit_sin, swap_sign_bit_sin);

//     // Evaluate the first polynom  (0 <= x <= Pi/4)
//     let z = _mm_mul_ps(x, x);
//     y = PS_COSCOF_P0.m128;

//     y = _mm_mul_ps(y, z);
//     y = _mm_add_ps(y, PS_COSCOF_P1.m128);
//     y = _mm_mul_ps(y, z);
//     y = _mm_add_ps(y, PS_COSCOF_P2.m128);
//     y = _mm_mul_ps(y, z);
//     y = _mm_mul_ps(y, z);
//     let tmp = _mm_mul_ps(z, PS_0_5.m128);
//     y = _mm_sub_ps(y, tmp);
//     y = _mm_add_ps(y, PS_1_0.m128);

//     // Evaluate the second polynom  (Pi/4 <= x <= 0)
//     let mut y2 = PS_SINCOF_P0.m128;
//     y2 = _mm_mul_ps(y2, z);
//     y2 = _mm_add_ps(y2, PS_SINCOF_P1.m128);
//     y2 = _mm_mul_ps(y2, z);
//     y2 = _mm_add_ps(y2, PS_SINCOF_P2.m128);
//     y2 = _mm_mul_ps(y2, z);
//     y2 = _mm_mul_ps(y2, x);
//     y2 = _mm_add_ps(y2, x);

//     // select the correct result from the two polynoms
//     xmm3 = poly_mask;
//     let ysin2 = _mm_and_ps(xmm3, y2);
//     let ysin1 = _mm_andnot_ps(xmm3, y);
//     y2 = _mm_sub_ps(y2, ysin2);
//     y = _mm_sub_ps(y, ysin1);

//     xmm1 = _mm_add_ps(ysin1, ysin2);
//     xmm2 = _mm_add_ps(y, y2);

//     // update the sign
//     (
//         _mm_xor_ps(xmm1, sign_bit_sin),
//         _mm_xor_ps(xmm2, sign_bit_cos),
//     )
// }

#[inline]
pub fn scalar_acos(value: f32) -> f32 {
    // from DirectXMath XMScalarAcos
    // Clamp input to [-1,1].
    let nonnegative = value >= 0.0;
    let x = value.abs();
    let mut omx = 1.0 - x;
    if omx < 0.0 {
        omx = 0.0;
    }
    let root = omx.sqrt();

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
        std::f32::consts::PI - result
    }
}

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        assert_approx_eq!($a, $b, core::f32::EPSILON);
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            (a - b).abs() <= eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (a - b).abs()
        );
    }};
}

#[cfg(test)]
macro_rules! assert_relative_eq {
    ($a:expr, $b:expr) => {{
        assert_relative_eq!($a, $b, core::f32::EPSILON);
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        let diff = (a - b).abs();
        let largest = a.abs().max(b.abs());
        assert!(
            diff <= largest * eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            largest * eps,
            diff
        );
    }};
}

#[test]
fn test_scalar_acos() {
    fn test_scalar_acos_angle(a: f32) {
        // 1e-6 is the lowest epsilon that will pass
        assert_relative_eq!(scalar_acos(a), a.acos(), 1e-6);
        // assert_approx_eq!(scalar_acos(a), a.acos(), 1e-6);
    }

    // test 1024 floats between -1.0 and 1.0 inclusive
    const MAX_TESTS: u32 = 1024 / 2;
    const SIGN: u32 = 0x80_00_00_00;
    const PTVE_ONE: u32 = 0x3f_80_00_00; // 1.0_f32.to_bits();
    const NGVE_ONE: u32 = SIGN | PTVE_ONE;
    const STEP_SIZE: usize = (PTVE_ONE / MAX_TESTS) as usize;
    for f in (SIGN..=NGVE_ONE).step_by(STEP_SIZE).map(|i| f32::from_bits(i)) {
        test_scalar_acos_angle(f);
    }
    for f in (0..=PTVE_ONE).step_by(STEP_SIZE).map(|i| f32::from_bits(i)) {
        test_scalar_acos_angle(f);
    }

    // input is clamped to -1.0..1.0
    assert_approx_eq!(scalar_acos(2.0), 0.0);
    assert_approx_eq!(scalar_acos(-2.0), std::f32::consts::PI);
}

#[test]
fn test_scalar_sin_cos() {
    fn test_scalar_sin_cos_angle(a: f32) {
        let (s1, c1) = scalar_sin_cos(a);
        let (s2, c2) = a.sin_cos();
        dbg!(a);
        assert_approx_eq!(s1, s2);
        assert_approx_eq!(c1, c2);
    }

    // test 1024 floats between -PI and PI inclusive
    const MAX_TESTS: u32 = 1024 / 2;
    const SIGN: u32 = 0x80_00_00_00;
    let ptve_pi = std::f32::consts::PI.to_bits();
    let ngve_pi = SIGN | ptve_pi;
    let step_pi = (ptve_pi / MAX_TESTS) as usize;
    for f in (SIGN..=ngve_pi).step_by(step_pi).map(|i| f32::from_bits(i)) {
        test_scalar_sin_cos_angle(f);
    }
    for f in (0..=ptve_pi).step_by(step_pi).map(|i| f32::from_bits(i)) {
        test_scalar_sin_cos_angle(f);
    }

    // test 1024 floats between -INF and +INF exclusive
    let ptve_inf = std::f32::INFINITY.to_bits();
    let ngve_inf = std::f32::NEG_INFINITY.to_bits();
    let step_inf = (ptve_inf / MAX_TESTS) as usize;
    for f in (SIGN..ngve_inf).step_by(step_inf).map(|i| f32::from_bits(i)) {
        test_scalar_sin_cos_angle(f);
    }
    for f in (0..ptve_inf).step_by(step_inf).map(|i| f32::from_bits(i)) {
        test_scalar_sin_cos_angle(f);
    }

    // +inf and -inf should return NaN
    let (s, c) = scalar_sin_cos(std::f32::INFINITY);
    assert!(s.is_nan());
    assert!(c.is_nan());

    let (s, c) = scalar_sin_cos(std::f32::NEG_INFINITY);
    assert!(s.is_nan());
    assert!(c.is_nan());
}

