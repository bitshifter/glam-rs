#![allow(clippy::excessive_precision)]
#![cfg(feature = "f16")]

#[macro_use]
mod support;

use glam::{hvec2, BVec2, HVec2};
use half::f16;

fn f16x(v: f32) -> f16 {
    f16::from_f32(v)
}

fn f16_abs(x: f16) -> f16 {
    f16x(x.to_f32().abs())
}

glam_test!(test_hvec2_consts, {
    assert_eq!(HVec2::ZERO, hvec2(f16x(0.0), f16x(0.0)));
    assert_eq!(HVec2::ONE, hvec2(f16x(1.0), f16x(1.0)));
    assert_eq!(HVec2::NEG_ONE, hvec2(f16x(-1.0), f16x(-1.0)));
    assert_eq!(HVec2::X, hvec2(f16x(1.0), f16x(0.0)));
    assert_eq!(HVec2::Y, hvec2(f16x(0.0), f16x(1.0)));
    assert_eq!(HVec2::NEG_X, hvec2(f16x(-1.0), f16x(0.0)));
    assert_eq!(HVec2::NEG_Y, hvec2(f16x(0.0), f16x(-1.0)));
    assert_eq!(HVec2::MIN, hvec2(f16::MIN, f16::MIN));
    assert_eq!(HVec2::MAX, hvec2(f16::MAX, f16::MAX));
});

glam_test!(test_hvec2_new, {
    let v = hvec2(f16x(1.0), f16x(2.0));
    assert_eq!(v.x, f16x(1.0));
    assert_eq!(v.y, f16x(2.0));

    let t = (f16x(1.0), f16x(2.0));
    let v = HVec2::from(t);
    assert_eq!(t, v.into());

    let a = [f16x(1.0), f16x(2.0)];
    let v = HVec2::from(a);
    let a1: [f16; 2] = v.into();
    assert_eq!(a, a1);
    assert_eq!(a, v.to_array());
    assert_eq!(a, *v.as_ref());

    assert_eq!(
        HVec2::new(f16x(1.0), f16x(0.0)),
        BVec2::new(true, false).into()
    );
    assert_eq!(
        HVec2::new(f16x(0.0), f16x(1.0)),
        BVec2::new(false, true).into()
    );
});

glam_test!(test_hvec2_fmt, {
    let a = hvec2(f16x(1.0), f16x(2.0));
    assert_eq!(format!("{:?}", a), format!("HVec2({:?}, {:?})", a.x, a.y));
    assert_eq!(format!("{}", a), "[1, 2]");
});

glam_test!(test_hvec2_splat_map_with, {
    assert_eq!(HVec2::splat(f16x(1.0)), HVec2::ONE);
    let v = hvec2(f16x(1.0), f16x(2.0));
    assert_eq!(v.map(|n| n + f16x(3.0)), v + HVec2::splat(f16x(3.0)));
    assert_eq!(v.map(|_| f16x(0.0)), HVec2::ZERO);
    assert_eq!(HVec2::X, HVec2::ZERO.with_x(f16x(1.0)));
    assert_eq!(HVec2::Y, HVec2::ZERO.with_y(f16x(1.0)));
    assert_eq!(hvec2(f16x(1.0), f16x(2.0)), v.extend(f16x(3.0)).truncate());
});

glam_test!(test_hvec2_ops, {
    let a = hvec2(f16x(1.0), f16x(2.0));
    let b = hvec2(f16x(3.0), f16x(4.0));
    assert_eq!(a + b, hvec2(f16x(4.0), f16x(6.0)));
    assert_eq!(a - b, hvec2(f16x(-2.0), f16x(-2.0)));
    assert_eq!(a * b, hvec2(f16x(3.0), f16x(8.0)));
    assert_eq!(a / b, hvec2(f16x(1.0 / 3.0), f16x(0.5)));
    assert_eq!(-a, hvec2(f16x(-1.0), f16x(-2.0)));
    assert_eq!(a * f16x(2.0), hvec2(f16x(2.0), f16x(4.0)));
    assert_eq!(f16x(2.0) * a, hvec2(f16x(2.0), f16x(4.0)));
});

glam_test!(test_hvec2_dot, {
    let x = hvec2(f16x(1.0), f16x(0.0));
    let y = hvec2(f16x(0.0), f16x(1.0));
    assert_eq!(f16x(1.0), x.dot(x));
    assert_eq!(f16x(0.0), x.dot(y));
    assert_eq!(
        hvec2(f16x(8.0), f16x(8.0)),
        hvec2(f16x(1.0), f16x(2.0)).dot_into_vec(hvec2(f16x(4.0), f16x(2.0)))
    );
});

glam_test!(test_hvec2_length, {
    assert_eq!(f16x(4.0), (hvec2(f16x(2.0), f16x(0.0))).length_squared());
    assert_eq!(f16x(2.0), (hvec2(f16x(2.0), f16x(0.0))).length());
    assert_eq!(f16x(0.5), (hvec2(f16x(2.0), f16x(0.0))).length_recip());
    assert_eq!(f16x(13.0), hvec2(f16x(2.0), f16x(3.0)).length_squared());
    assert_eq!(
        f16x(5.0),
        hvec2(f16x(2.0), f16x(3.0)).distance(hvec2(f16x(6.0), f16x(6.0)))
    );
});

glam_test!(test_hvec2_normalize, {
    let v = hvec2(f16x(3.0), f16x(4.0));
    assert!(v.normalize().is_normalized());
    assert_eq!(v.normalize(), v.normalize_or(HVec2::ZERO));
    assert_eq!(v.normalize(), v.normalize_or_zero());
    let (n, l) = v.normalize_and_length();
    assert_eq!(n, v.normalize());
    assert_eq!(l, f16x(5.0));
    assert_eq!(HVec2::ZERO.try_normalize(), None);
    assert_eq!(v.try_normalize(), Some(v.normalize()));
});

glam_test!(test_hvec2_minmax, {
    let a = hvec2(f16x(2.0), f16x(3.0));
    let b = hvec2(f16x(1.0), f16x(4.0));
    assert_eq!(a.min(b), hvec2(f16x(1.0), f16x(3.0)));
    assert_eq!(a.max(b), hvec2(f16x(2.0), f16x(4.0)));
    assert_eq!(
        a.clamp(hvec2(f16x(0.5), f16x(0.5)), hvec2(f16x(2.5), f16x(2.5))),
        hvec2(f16x(2.0), f16x(2.5))
    );
    assert_eq!(a.min_element(), f16x(2.0));
    assert_eq!(a.max_element(), f16x(3.0));
    assert_eq!(a.min_position(), 0);
    assert_eq!(a.max_position(), 1);
    assert_eq!(a.element_sum(), f16x(5.0));
    assert_eq!(a.element_product(), f16x(6.0));
});

glam_test!(test_hvec2_abs_signum, {
    let a = hvec2(f16x(-1.5), f16x(2.5));
    assert_eq!(a.abs(), hvec2(f16x(1.5), f16x(2.5)));
    assert_eq!(a.signum(), hvec2(f16x(-1.0), f16x(1.0)));
    assert_eq!(
        a.copysign(hvec2(f16x(1.0), f16x(-1.0))),
        hvec2(f16x(1.5), f16x(-2.5))
    );
    assert_eq!(a.is_negative_bitmask(), 0b01);
    assert_eq!(a.is_negative_mask(), BVec2::new(true, false));
    assert!(a.is_finite());
    assert!(!HVec2::NAN.is_finite());
    assert!(!a.is_nan());
});

glam_test!(test_hvec2_rounding, {
    let v = hvec2(f16x(1.5), f16x(-1.5));
    assert_eq!(v.round(), hvec2(f16x(2.0), f16x(-2.0)));
    assert_eq!(v.floor(), hvec2(f16x(1.0), f16x(-2.0)));
    assert_eq!(v.ceil(), hvec2(f16x(2.0), f16x(-1.0)));
    assert_eq!(v.trunc(), hvec2(f16x(1.0), f16x(-1.0)));
    assert_eq!(v.fract(), hvec2(f16x(0.5), f16x(-0.5)));
    assert_eq!(v.saturate(), hvec2(f16x(1.0), f16x(0.0)));
});

glam_test!(test_hvec2_exp_ops, {
    let v = hvec2(f16x(1.0), f16x(2.0));
    assert_eq!(v.exp(), hvec2(f16::E, f16x(7.389056)));
    assert_eq!(v.exp2(), hvec2(f16x(2.0), f16x(4.0)));
    let ln = hvec2(f16x(1.0), f16x(2.0)).ln();
    assert!(
        ln.abs_diff_eq(hvec2(f16x(0.0), f16::LN_2), f16x(1e-3)),
        "ln: {:?}",
        ln
    );
    assert_eq!(
        hvec2(f16x(4.0), f16x(8.0)).log2(),
        hvec2(f16x(2.0), f16x(3.0))
    );
    assert_eq!(v.powf(f16x(2.0)), hvec2(f16x(1.0), f16x(4.0)));
    assert_eq!(
        hvec2(f16x(4.0), f16x(9.0)).sqrt(),
        hvec2(f16x(2.0), f16x(3.0))
    );
    assert_eq!(v.recip(), hvec2(f16x(1.0), f16x(0.5)));
    assert_eq!(v.sin_cos(), (v.sin(), v.cos()));
});

glam_test!(test_hvec2_lerp, {
    let a = hvec2(f16x(0.0), f16x(0.0));
    let b = hvec2(f16x(2.0), f16x(4.0));
    assert_eq!(a.lerp(b, f16x(0.5)), hvec2(f16x(1.0), f16x(2.0)));
    let m = a.move_towards(b, f16x(1.0));
    assert!(
        m.abs_diff_eq(hvec2(f16x(0.44726563), f16x(0.89453125)), f16x(1e-3)),
        "m: {:?}",
        m
    );
    assert_eq!(a.midpoint(b), hvec2(f16x(1.0), f16x(2.0)));
    assert!(a.abs_diff_eq(b, f16x(5.0)));
    assert!(!(a.abs_diff_eq(b, f16x(3.0))));
});

glam_test!(test_hvec2_clamp_length, {
    let v = hvec2(f16x(3.0), f16x(4.0));
    assert_eq!(v.clamp_length(f16x(5.0), f16x(5.0)), v);
    assert_eq!(v.clamp_length_max(f16x(5.0)), v);
    assert_eq!(v.clamp_length_min(f16x(5.0)), v);
});

glam_test!(test_hvec2_reflect, {
    let v = hvec2(f16x(1.0), f16x(-1.0));
    let n = hvec2(f16x(0.0), f16x(1.0));
    assert_eq!(v.reflect(n), hvec2(f16x(1.0), f16x(1.0)));
    // `refract` requires `self` and `normal` to be normalized, so normalize
    // the incident vector (mirroring the `f32` tests).
    let incident = v.normalize();
    assert!(incident
        .refract(n, f16x(1.0))
        .abs_diff_eq(incident, f16x(1e-3)));
});

glam_test!(test_hvec2_angle, {
    let x = hvec2(f16x(1.0), f16x(0.0));
    let y = hvec2(f16x(0.0), f16x(1.0));
    let v = hvec2(f16x(0.0), f16x(1.0));
    assert_eq!(v.to_angle(), f16x(core::f32::consts::FRAC_PI_2));
    assert!(f16_abs(v.angle_to(y)) < f16x(1e-4));
    assert_eq!(v.perp(), hvec2(f16x(-1.0), f16x(0.0)));
    assert_eq!(x.perp_dot(y), f16x(1.0));
    let r = v.rotate_angle(f16x(core::f32::consts::FRAC_PI_2));
    assert!(
        r.abs_diff_eq(hvec2(f16x(-1.0), f16x(0.0)), f16x(1e-3)),
        "r: {:?}",
        r
    );
    let a = HVec2::from_angle(f16x(core::f32::consts::FRAC_PI_2));
    assert!(a.is_normalized());
});

glam_test!(test_hvec2_project, {
    let a = hvec2(f16x(2.0), f16x(0.0));
    let b = hvec2(f16x(1.0), f16x(1.0));
    assert_eq!(a.project_onto(b), hvec2(f16x(1.0), f16x(1.0)));
    assert_eq!(a.reject_from(b), hvec2(f16x(1.0), f16x(-1.0)));
    assert_eq!(
        a.project_onto_normalized(b.normalize()),
        hvec2(f16x(1.0), f16x(1.0))
    );
    assert_eq!(
        a.reject_from_normalized(b.normalize()),
        hvec2(f16x(1.0), f16x(-1.0))
    );
});

glam_test!(test_hvec2_as_conversions, {
    let v = hvec2(f16x(1.0), f16x(2.0));
    assert_eq!(v.as_vec2(), glam::vec2(1.0, 2.0));
    #[cfg(feature = "f64")]
    assert_eq!(v.as_dvec2(), glam::dvec2(1.0, 2.0));
    #[cfg(feature = "i8")]
    assert_eq!(v.as_i8vec2(), glam::i8vec2(1, 2));
    #[cfg(feature = "u8")]
    assert_eq!(v.as_u8vec2(), glam::u8vec2(1, 2));
    #[cfg(feature = "i16")]
    assert_eq!(v.as_i16vec2(), glam::i16vec2(1, 2));
    #[cfg(feature = "u16")]
    assert_eq!(v.as_u16vec2(), glam::u16vec2(1, 2));
    #[cfg(feature = "i32")]
    assert_eq!(v.as_ivec2(), glam::ivec2(1, 2));
    #[cfg(feature = "u32")]
    assert_eq!(v.as_uvec2(), glam::uvec2(1, 2));
    #[cfg(feature = "i64")]
    assert_eq!(v.as_i64vec2(), glam::i64vec2(1, 2));
    #[cfg(feature = "u64")]
    assert_eq!(v.as_u64vec2(), glam::u64vec2(1, 2));
    #[cfg(feature = "isize")]
    assert_eq!(v.as_isizevec2(), glam::isizevec2(1, 2));
    #[cfg(feature = "usize")]
    assert_eq!(v.as_usizevec2(), glam::usizevec2(1, 2));
});

glam_test!(test_hvec2_from_vec2, {
    assert_eq!(
        HVec2::from(glam::Vec2::new(1.0, 2.0)),
        hvec2(f16x(1.0), f16x(2.0))
    );
    #[cfg(feature = "i32")]
    assert_eq!(
        HVec2::from(glam::IVec2::new(1, 2)),
        hvec2(f16x(1.0), f16x(2.0))
    );
    #[cfg(feature = "u32")]
    assert_eq!(
        HVec2::from(glam::UVec2::new(1, 2)),
        hvec2(f16x(1.0), f16x(2.0))
    );
    assert_eq!(
        HVec2::from(BVec2::new(true, false)),
        hvec2(f16x(1.0), f16x(0.0))
    );
});

glam_test!(test_hvec2_div_euclid, {
    let a = hvec2(f16x(5.0), f16x(-5.0));
    let b = hvec2(f16x(3.0), f16x(3.0));
    assert_eq!(a.div_euclid(b), hvec2(f16x(1.0), f16x(-2.0)));
    assert_eq!(a.rem_euclid(b), hvec2(f16x(2.0), f16x(1.0)));
});

glam_test!(test_hvec2_finite, {
    assert!(hvec2(f16x(1.0), f16x(2.0)).is_finite());
    assert!(!hvec2(f16::NAN, f16x(1.0)).is_finite());
    assert!(!hvec2(f16::INFINITY, f16x(1.0)).is_finite());
});
