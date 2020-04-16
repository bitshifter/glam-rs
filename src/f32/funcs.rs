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

#[cfg(vec4sse2)]
pub(crate) mod sse2 {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    use crate::f32::x86_utils::UnionCast;

    macro_rules! _ps_const_ty {
        ($name:ident, $field:ident, $x:expr) => {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            const $name: UnionCast = UnionCast {
                $field: [$x, $x, $x, $x],
            };
        };

        ($name:ident, $field:ident, $x:expr, $y:expr, $z:expr, $w:expr) => {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            const $name: UnionCast = UnionCast {
                $field: [$x, $y, $z, $w],
            };
        };
    }

    _ps_const_ty!(PS_INV_SIGN_MASK, u32x4, !0x8000_0000);
    _ps_const_ty!(PS_SIGN_MASK, u32x4, 0x8000_0000);
    _ps_const_ty!(PS_NO_FRACTION, f32x4, 8388608.0);

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

    _ps_const_ty!(g_XMNegativeZero, u32x4, 0x80000000);
    _ps_const_ty!(g_XMPi, f32x4, std::f32::consts::PI);
    _ps_const_ty!(g_XMHalfPi, f32x4, std::f32::consts::FRAC_PI_2);
    _ps_const_ty!(g_XMSinCoefficients0 , f32x4, -0.16666667, 0.0083333310, -0.00019840874, 2.7525562e-06);
    _ps_const_ty!(g_XMSinCoefficients1 , f32x4, -2.3889859e-08, -0.16665852 /*Est1*/, 0.0083139502 /*Est2*/, -0.00018524670 /*Est3*/);
    _ps_const_ty!(g_XMOne, f32x4, 1.0);
    _ps_const_ty!(g_XMTwoPi, f32x4, std::f32::consts::PI * 2.0);
    _ps_const_ty!(g_XMReciprocalTwoPi , f32x4, 0.159154943);

    macro_rules! XM_PERMUTE_PS {
        ($v:expr, $c:expr) => {
            _mm_shuffle_ps($v, $v, $c)
        }
    }

    // XMVectorNegativeMultiplySubtract
    macro_rules! XM_FMADD_PS {
        ($a:expr, $b:expr, $c:expr) => {
            _mm_fmadd_ps($a, $b, $c)
        }
    }

    #[inline]
    pub(crate) unsafe fn m128_round(v: __m128) -> __m128 {
        // From DirectXMath XMVectorRound.
        let sign = _mm_and_ps(v, PS_SIGN_MASK.m128);
        let s_magic = _mm_or_ps(PS_NO_FRACTION.m128, sign);
        let r1 = _mm_add_ps(v, s_magic);
        let r1 = _mm_sub_ps(r1, s_magic);
        let r2 = _mm_and_ps(v, PS_INV_SIGN_MASK.m128);
        let mask = _mm_cmple_ps(r2, PS_NO_FRACTION.m128);
        let r2 = _mm_andnot_ps(mask, v);
        let r1 = _mm_and_ps(r1, mask);
        _mm_xor_ps(r1, r2)
    }

    #[inline]
    pub(crate) unsafe fn m128_floor(v: __m128) -> __m128 {
        // From DirectXMath XMVectorFloor
        // To handle NAN, INF and numbers greater than 8388608, use masking
        let test = _mm_and_si128(_mm_castps_si128(v), PS_INV_SIGN_MASK.m128i);
        let test = _mm_cmplt_epi32(test, PS_NO_FRACTION.m128i);
        // Truncate
        let vint = _mm_cvttps_epi32(v);
        let result = _mm_cvtepi32_ps(vint);
        let larger = _mm_cmpgt_ps(result, v);
        // 0 -> 0, 0xffffffff -> -1.0f
        let larger = _mm_cvtepi32_ps(_mm_castps_si128(larger));
        let result = _mm_add_ps(result, larger);
        // All numbers less than 8388608 will use the round to int
        let result = _mm_and_ps(result, _mm_castsi128_ps(test));
        // All others, use the ORIGINAL value
        let test = _mm_andnot_si128(test, _mm_castps_si128(v));
        _mm_or_ps(result, _mm_castsi128_ps(test))
    }

    #[inline]
    pub(crate) unsafe fn m128_ceil(v: __m128) -> __m128 {
        // From DirectXMath XMVectorCeil
        // To handle NAN, INF and numbers greater than 8388608, use masking
        let test = _mm_and_si128(_mm_castps_si128(v), PS_INV_SIGN_MASK.m128i);
        let test = _mm_cmplt_epi32(test, PS_NO_FRACTION.m128i);
        // Truncate
        let vint = _mm_cvttps_epi32(v);
        let result = _mm_cvtepi32_ps(vint);
        let smaller = _mm_cmplt_ps(result, v);
        // 0 -> 0, 0xffffffff -> -1.0f
        let smaller = _mm_cvtepi32_ps(_mm_castps_si128(smaller));
        let result = _mm_sub_ps(result, smaller);
        // All numbers less than 8388608 will use the round to int
        let result = _mm_and_ps(result, _mm_castsi128_ps(test));
        // All others, use the ORIGINAL value
        let test = _mm_andnot_si128(test, _mm_castps_si128(v));
        _mm_or_ps(result, _mm_castsi128_ps(test))
    }

    #[inline]
    #[allow(non_snake_case)]
    pub(crate) const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
        // core::arch::x86_64::_MM_SHUFFLE requires nightly
        ((z << 6) | (y << 4) | (x << 2) | w) as i32
    }

    /// From DirectXMath: XMVectorModAngles
    /// 
    /// Returns a vector whose components are the corresponding components of Angles modulo 2PI.
    pub(crate) unsafe fn m128_mod_angles(angles: __m128) -> __m128 {
        let v = _mm_mul_ps(angles, g_XMReciprocalTwoPi.m128);
        let v = m128_round(v);
        XM_FMADD_PS!(g_XMTwoPi.m128, v, angles) // XMVectorNegativeMultiplySubtract
    }

    /// From DirectXMath: XMVectorSin
    pub(crate) unsafe fn m128_sin(v: __m128) -> __m128 {
        // Note that XMVectorSin clamps the value here with XMVectorModAngles,
        // but this adds about 15% overhead.
        //
        // // Force the value within the bounds of pi
        // let mut x = m128_mod_angles(v);

        let mut x = v;

        // Map in [-pi/2,pi/2] with sin(y) = sin(x).
        let sign = _mm_and_ps(x, g_XMNegativeZero.m128);
        // pi when x >= 0, -pi when x < 0
        let c = _mm_or_ps(g_XMPi.m128, sign);
        // |x|
        let absx = _mm_andnot_ps(sign, x);
        let rflx = _mm_sub_ps(c, x);
        let comp = _mm_cmple_ps(absx, g_XMHalfPi.m128);
        let select0 = _mm_and_ps(comp, x);
        let select1 = _mm_andnot_ps(comp, rflx);
        x = _mm_or_ps(select0, select1);

        let x2 = _mm_mul_ps(x, x);

        // Compute polynomial approximation
        let sc1 = g_XMSinCoefficients1;
        let v_constants_b = XM_PERMUTE_PS!(sc1.m128, _MM_SHUFFLE(0, 0, 0, 0));

        let sc0 = g_XMSinCoefficients0.m128;
        let mut v_constants = XM_PERMUTE_PS!(sc0, _MM_SHUFFLE(3, 3, 3, 3));
        let mut result = XM_FMADD_PS!(v_constants_b, x2, v_constants);

        v_constants = XM_PERMUTE_PS!(sc0, _MM_SHUFFLE(2, 2, 2, 2));
        result = XM_FMADD_PS!(result, x2, v_constants);

        v_constants = XM_PERMUTE_PS!(sc0, _MM_SHUFFLE(1, 1, 1, 1));
        result = XM_FMADD_PS!(result, x2, v_constants);

        v_constants = XM_PERMUTE_PS!(sc0, _MM_SHUFFLE(0, 0, 0, 0));
        result = XM_FMADD_PS!(result, x2, v_constants);

        result = XM_FMADD_PS!(result, x2, g_XMOne.m128);
        result = _mm_mul_ps(result, x);

        result
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
    for f in (SIGN..=NGVE_ONE)
        .step_by(STEP_SIZE)
        .map(|i| f32::from_bits(i))
    {
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
    for f in (SIGN..ngve_inf)
        .step_by(step_inf)
        .map(|i| f32::from_bits(i))
    {
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
