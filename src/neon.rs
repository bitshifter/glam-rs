use core::arch::aarch64::*;

union UnionCast {
    u32x4: [u32; 4],
    f32x4: [f32; 4],
    v: float32x4_t,
    u: uint32x4_t,
}

#[inline]
pub const fn f32x4_from_array(f32x4: [f32; 4]) -> float32x4_t {
    unsafe { UnionCast { f32x4 }.v }
}

#[inline]
pub(crate) const fn u32x4_from_array(u32x4: [u32; 4]) -> uint32x4_t {
    unsafe { UnionCast { u32x4 }.u }
}

const U32X4_NEG_ZERO: uint32x4_t = u32x4_from_array([0x8000_0000; 4]);
const F32X4_PI: float32x4_t = f32x4_from_array([core::f32::consts::PI; 4]);
const F32X4_FRAC_PI_2: float32x4_t = f32x4_from_array([core::f32::consts::FRAC_PI_2; 4]);
const F32X4_SIN_COEFFICIENTS0: float32x4_t =
    f32x4_from_array([-0.16666667, 0.008_333_331, -0.00019840874, 2.752_556_2e-6]);
const F32X4_SIN_COEFFICIENTS1: float32x4_t = f32x4_from_array([
    -2.388_985_9e-8,
    -0.16665852,      /*Est1*/
    0.008_313_95,     /*Est2*/
    -0.000_185_246_7, /*Est3*/
]);
const F32X4_ONE: float32x4_t = f32x4_from_array([1.0; 4]);
const F32X4_TAU: float32x4_t = f32x4_from_array([core::f32::consts::TAU; 4]);
const F32X4_FRAC_1_TAU: float32x4_t = f32x4_from_array([0.159_154_94; 4]);

// #[inline]
// pub(crate) unsafe fn dot3_in_x(lhs: float32x4_t, rhs: float32x4_t) -> float32x4_t {
//     let x2_y2_z2_w2 = vmulq_f32(lhs, rhs);
//     let y2 = vdupq_laneq_f32(x2_y2_z2_w2, 1);
//     let z2 = vdupq_laneq_f32(x2_y2_z2_w2, 2);
//     let x2y2 = vaddq_f32(x2_y2_z2_w2, y2);
//     vaddq_f32(x2y2, z2)
// }

#[inline]
pub(crate) unsafe fn dot3(lhs: float32x4_t, rhs: float32x4_t) -> f32 {
    let x2_y2_z2_w2 = vmulq_f32(lhs, rhs);
    let x2_y2_z2 = vsetq_lane_f32(0.0, x2_y2_z2_w2, 3);
    vaddvq_f32(x2_y2_z2)
    // let dot = dot3_in_x(lhs, rhs);
    // vdups_laneq_f32(dot, 0)
}

#[inline]
pub(crate) unsafe fn dot3_into_f32x4(lhs: float32x4_t, rhs: float32x4_t) -> float32x4_t {
    let dot = dot3(lhs, rhs);
    vld1q_dup_f32(&dot as *const f32)
    // let dot = dot3_in_x(lhs, rhs);
    // vdupq_laneq_f32(dot, 0)
}

#[inline]
pub(crate) unsafe fn dot4(lhs: float32x4_t, rhs: float32x4_t) -> f32 {
    let x2_y2_z2_w2 = vmulq_f32(lhs, rhs);
    // TODO: horizontal add - might perform bad?
    vaddvq_f32(x2_y2_z2_w2)
}

#[inline]
pub(crate) unsafe fn dot4_into_f32x4(lhs: float32x4_t, rhs: float32x4_t) -> float32x4_t {
    let dot = dot4(lhs, rhs);
    vld1q_dup_f32(&dot as *const f32)
}

/// Returns a vector whose components are the corresponding components of Angles modulo 2PI.
#[inline]
pub(crate) unsafe fn f32x4_mod_angles(angles: float32x4_t) -> float32x4_t {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorModAngles`
    // Modulo the range of the given angles such that -XM_PI <= Angles < XM_PI
    let mut v = vmulq_f32(angles, F32X4_FRAC_1_TAU);
    v = vrndnq_f32(v);
    vmlsq_f32(angles, v, F32X4_TAU)
}

/// Computes the sine of the angle in each lane of `v`. Values outside
/// the bounds of PI may produce an increasing error as the input angle
/// drifts from `[-PI, PI]`.
#[inline]
pub(crate) unsafe fn f32x4_sin(v: float32x4_t) -> float32x4_t {
    // Based on https://github.com/microsoft/DirectXMath `XMVectorSin`

    // 11-degree minimax approximation

    // Force the value within the bounds of pi
    let mut x = f32x4_mod_angles(v);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    let sign = vandq_u32(vreinterpretq_u32_f32(x), U32X4_NEG_ZERO);
    let c = vorrq_u32(vreinterpretq_u32_f32(F32X4_PI), sign); // pi when x >= 0, -pi when x < 0
    let absx = vabsq_f32(x);
    let rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    let comp = vcleq_f32(absx, F32X4_FRAC_PI_2);
    x = vbslq_f32(comp, x, rflx);

    let x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const SC1: float32x4_t = F32X4_SIN_COEFFICIENTS1;
    const SC0: float32x4_t = F32X4_SIN_COEFFICIENTS0;
    let mut v_constants = vdupq_lane_f32(vget_high_f32(SC0), 1);
    let mut result = vmlaq_lane_f32(v_constants, x2, vget_low_f32(SC1), 0);

    v_constants = vdupq_lane_f32(vget_high_f32(SC0), 0);
    result = vmlaq_f32(v_constants, result, x2);

    v_constants = vdupq_lane_f32(vget_low_f32(SC0), 1);
    result = vmlaq_f32(v_constants, result, x2);

    v_constants = vdupq_lane_f32(vget_low_f32(SC0), 0);
    result = vmlaq_f32(v_constants, result, x2);

    result = vmlaq_f32(F32X4_ONE, result, x2);
    result = vmulq_f32(result, x);

    result
}

#[test]
fn test_neon_f32x4_sin() {
    use crate::Vec4;
    use core::f32::consts::PI;

    fn test_neon_f32x4_sin_angle(a: f32) {
        let v = unsafe { f32x4_sin(vdupq_n_f32(a)) };
        let v = Vec4(v);
        let a_sin = a.sin();
        // dbg!((a, a_sin, v));
        assert!(v.abs_diff_eq(Vec4::splat(a_sin), 1e-6));
    }

    let mut a = -PI;
    let end = PI;
    let step = PI / 8192.0;

    while a <= end {
        test_neon_f32x4_sin_angle(a);
        a += step;
    }
}
