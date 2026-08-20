#![allow(clippy::excessive_precision)]
#![cfg(feature = "f16")]

#[macro_use]
mod support;

use glam::{hvec2, hvec3, BVec3, HVec3};
use half::f16;

fn f16x(v: f32) -> f16 {
    f16::from_f32(v)
}

fn f16_abs(x: f16) -> f16 {
    f16x(x.to_f32().abs())
}

glam_test!(test_hvec3_consts, {
    assert_eq!(HVec3::ZERO, hvec3(f16x(0.0), f16x(0.0), f16x(0.0)));
    assert_eq!(HVec3::ONE, hvec3(f16x(1.0), f16x(1.0), f16x(1.0)));
    assert_eq!(HVec3::X, hvec3(f16x(1.0), f16x(0.0), f16x(0.0)));
    assert_eq!(HVec3::Y, hvec3(f16x(0.0), f16x(1.0), f16x(0.0)));
    assert_eq!(HVec3::Z, hvec3(f16x(0.0), f16x(0.0), f16x(1.0)));
    assert_eq!(HVec3::MIN, hvec3(f16::MIN, f16::MIN, f16::MIN));
    assert_eq!(HVec3::MAX, hvec3(f16::MAX, f16::MAX, f16::MAX));
});

glam_test!(test_hvec3_new, {
    let v = hvec3(f16x(1.0), f16x(2.0), f16x(3.0));
    assert_eq!(v.x, f16x(1.0));
    assert_eq!(v.y, f16x(2.0));
    assert_eq!(v.z, f16x(3.0));

    let t = (f16x(1.0), f16x(2.0), f16x(3.0));
    let v = HVec3::from(t);
    assert_eq!(t, v.into());

    let a = [f16x(1.0), f16x(2.0), f16x(3.0)];
    let v = HVec3::from(a);
    let a1: [f16; 3] = v.into();
    assert_eq!(a, a1);
    assert_eq!(a, v.to_array());
    assert_eq!(a, *v.as_ref());

    assert_eq!(
        HVec3::new(f16x(1.0), f16x(0.0), f16x(0.0)),
        BVec3::new(true, false, false).into()
    );

    assert_eq!(
        hvec3(f16x(1.0), f16x(2.0), f16x(3.0)),
        hvec2(f16x(1.0), f16x(2.0)).extend(f16x(3.0))
    );
});

glam_test!(test_hvec3_fmt, {
    let a = hvec3(f16x(1.0), f16x(2.0), f16x(3.0));
    assert_eq!(
        format!("{:?}", a),
        format!("HVec3({:?}, {:?}, {:?})", a.x, a.y, a.z)
    );
    assert_eq!(format!("{}", a), "[1, 2, 3]");
});

glam_test!(test_hvec3_splat_map_with, {
    assert_eq!(HVec3::splat(f16x(1.0)), HVec3::ONE);
    let v = hvec3(f16x(1.0), f16x(2.0), f16x(3.0));
    assert_eq!(v.map(|n| n + f16x(1.0)), v + HVec3::ONE);
    assert_eq!(HVec3::X, HVec3::ZERO.with_x(f16x(1.0)));
    assert_eq!(HVec3::Y, HVec3::ZERO.with_y(f16x(1.0)));
    assert_eq!(HVec3::Z, HVec3::ZERO.with_z(f16x(1.0)));
});

glam_test!(test_hvec3_ops, {
    let a = hvec3(f16x(1.0), f16x(2.0), f16x(3.0));
    let b = hvec3(f16x(3.0), f16x(4.0), f16x(5.0));
    assert_eq!(a + b, hvec3(f16x(4.0), f16x(6.0), f16x(8.0)));
    assert_eq!(a - b, hvec3(f16x(-2.0), f16x(-2.0), f16x(-2.0)));
    assert_eq!(a * b, hvec3(f16x(3.0), f16x(8.0), f16x(15.0)));
    assert_eq!(a / b, hvec3(f16x(1.0 / 3.0), f16x(0.5), f16x(0.6)));
    assert_eq!(-a, hvec3(f16x(-1.0), f16x(-2.0), f16x(-3.0)));
    assert_eq!(a * f16x(2.0), hvec3(f16x(2.0), f16x(4.0), f16x(6.0)));
    assert_eq!(f16x(2.0) * a, hvec3(f16x(2.0), f16x(4.0), f16x(6.0)));
    assert_eq!(a % b, hvec3(f16x(1.0), f16x(2.0), f16x(3.0)));
});

glam_test!(test_hvec3_dot_cross, {
    let x = hvec3(f16x(1.0), f16x(0.0), f16x(0.0));
    let y = hvec3(f16x(0.0), f16x(1.0), f16x(0.0));
    let z = hvec3(f16x(0.0), f16x(0.0), f16x(1.0));
    assert_eq!(f16x(1.0), x.dot(x));
    assert_eq!(f16x(0.0), x.dot(y));
    assert_eq!(x.cross(y), z);
    assert_eq!(y.cross(z), x);
    assert_eq!(z.cross(x), y);
});

glam_test!(test_hvec3_length, {
    let v = hvec3(f16x(2.0), f16x(3.0), f16x(6.0));
    assert_eq!(f16x(49.0), v.length_squared());
    assert_eq!(f16x(7.0), v.length());
    assert_eq!(f16x(1.0 / 7.0), v.length_recip());
    assert_eq!(f16x(7.0), v.distance(HVec3::ZERO));
    assert_eq!(f16x(49.0), v.distance_squared(HVec3::ZERO));
});

glam_test!(test_hvec3_normalize, {
    let v = hvec3(f16x(2.0), f16x(3.0), f16x(6.0));
    assert!(v.normalize().is_normalized());
    assert_eq!(v.normalize(), v.normalize_or_zero());
    let (n, l) = v.normalize_and_length();
    assert_eq!(n, v.normalize());
    assert_eq!(l, f16x(7.0));
    assert_eq!(HVec3::ZERO.try_normalize(), None);
    assert_eq!(v.try_normalize(), Some(v.normalize()));
});

glam_test!(test_hvec3_minmax, {
    let a = hvec3(f16x(2.0), f16x(3.0), f16x(1.0));
    let b = hvec3(f16x(1.0), f16x(4.0), f16x(2.0));
    assert_eq!(a.min(b), hvec3(f16x(1.0), f16x(3.0), f16x(1.0)));
    assert_eq!(a.max(b), hvec3(f16x(2.0), f16x(4.0), f16x(2.0)));
    assert_eq!(a.min_element(), f16x(1.0));
    assert_eq!(a.max_element(), f16x(3.0));
    assert_eq!(a.min_position(), 2);
    assert_eq!(a.max_position(), 1);
    assert_eq!(a.element_sum(), f16x(6.0));
    assert_eq!(a.element_product(), f16x(6.0));
});

glam_test!(test_hvec3_abs, {
    let a = hvec3(f16x(-1.5), f16x(2.5), f16x(-3.5));
    assert_eq!(a.abs(), hvec3(f16x(1.5), f16x(2.5), f16x(3.5)));
    assert_eq!(a.signum(), hvec3(f16x(-1.0), f16x(1.0), f16x(-1.0)));
    assert_eq!(a.is_negative_bitmask(), 0b101);
    assert!(a.is_finite());
    assert!(!hvec3(f16::NAN, f16x(1.0), f16x(2.0)).is_finite());
});

glam_test!(test_hvec3_rounding, {
    let v = hvec3(f16x(1.5), f16x(-1.5), f16x(2.5));
    assert_eq!(v.round(), hvec3(f16x(2.0), f16x(-2.0), f16x(3.0)));
    assert_eq!(v.floor(), hvec3(f16x(1.0), f16x(-2.0), f16x(2.0)));
    assert_eq!(v.ceil(), hvec3(f16x(2.0), f16x(-1.0), f16x(3.0)));
    assert_eq!(v.trunc(), hvec3(f16x(1.0), f16x(-1.0), f16x(2.0)));
});

glam_test!(test_hvec3_lerp, {
    let a = hvec3(f16x(0.0), f16x(0.0), f16x(0.0));
    let b = hvec3(f16x(2.0), f16x(4.0), f16x(6.0));
    assert_eq!(a.lerp(b, f16x(0.5)), hvec3(f16x(1.0), f16x(2.0), f16x(3.0)));
    assert_eq!(a.midpoint(b), hvec3(f16x(1.0), f16x(2.0), f16x(3.0)));
    assert!(a.abs_diff_eq(b, f16x(7.0)));
    assert!(!(a.abs_diff_eq(b, f16x(5.0))));
});

glam_test!(test_hvec3_clamp_length, {
    let v = hvec3(f16x(2.0), f16x(3.0), f16x(6.0));
    assert_eq!(
        v.clamp_length(f16x(7.0), f16x(7.0)),
        v.normalize() * f16x(7.0)
    );
    assert_eq!(v.clamp_length_max(f16x(7.0)), v);
    assert_eq!(v.clamp_length_min(f16x(7.0)), v);
});

glam_test!(test_hvec3_reflect, {
    let v = hvec3(f16x(1.0), f16x(-1.0), f16x(0.0));
    let n = hvec3(f16x(0.0), f16x(1.0), f16x(0.0));
    assert_eq!(v.reflect(n), hvec3(f16x(1.0), f16x(1.0), f16x(0.0)));
});

glam_test!(test_hvec3_project, {
    let a = hvec3(f16x(2.0), f16x(0.0), f16x(0.0));
    let b = hvec3(f16x(1.0), f16x(1.0), f16x(0.0));
    assert_eq!(a.project_onto(b), hvec3(f16x(1.0), f16x(1.0), f16x(0.0)));
    assert_eq!(a.reject_from(b), hvec3(f16x(1.0), f16x(-1.0), f16x(0.0)));
});

glam_test!(test_hvec3_any_orthogonal, {
    let v = hvec3(f16x(1.0), f16x(2.0), f16x(3.0)).normalize();
    let o = v.any_orthonormal_vector();
    assert!(f16_abs(o.length() - f16x(1.0)) < f16x(1e-3), "o: {:?}", o);
    assert!(f16_abs(v.dot(o)) < f16x(1e-3));
    let (a, b) = v.any_orthonormal_pair();
    assert!(f16_abs(a.length() - f16x(1.0)) < f16x(1e-3));
    assert!(f16_abs(b.length() - f16x(1.0)) < f16x(1e-3));
    assert!(f16_abs(v.dot(a)) < f16x(1e-3));
    assert!(f16_abs(v.dot(b)) < f16x(1e-3));
});

glam_test!(test_hvec3_as_conversions, {
    let v = hvec3(f16x(1.0), f16x(2.0), f16x(3.0));
    assert_eq!(v.as_vec3(), glam::vec3(1.0, 2.0, 3.0));
    #[cfg(feature = "f64")]
    assert_eq!(v.as_dvec3(), glam::dvec3(1.0, 2.0, 3.0));
    #[cfg(feature = "i8")]
    assert_eq!(v.as_i8vec3(), glam::i8vec3(1, 2, 3));
    #[cfg(feature = "u8")]
    assert_eq!(v.as_u8vec3(), glam::u8vec3(1, 2, 3));
    #[cfg(feature = "i16")]
    assert_eq!(v.as_i16vec3(), glam::i16vec3(1, 2, 3));
    #[cfg(feature = "u16")]
    assert_eq!(v.as_u16vec3(), glam::u16vec3(1, 2, 3));
    #[cfg(feature = "i32")]
    assert_eq!(v.as_ivec3(), glam::ivec3(1, 2, 3));
    #[cfg(feature = "u32")]
    assert_eq!(v.as_uvec3(), glam::uvec3(1, 2, 3));
    #[cfg(feature = "i64")]
    assert_eq!(v.as_i64vec3(), glam::i64vec3(1, 2, 3));
    #[cfg(feature = "u64")]
    assert_eq!(v.as_u64vec3(), glam::u64vec3(1, 2, 3));
    #[cfg(feature = "isize")]
    assert_eq!(v.as_isizevec3(), glam::isizevec3(1, 2, 3));
    #[cfg(feature = "usize")]
    assert_eq!(v.as_usizevec3(), glam::usizevec3(1, 2, 3));
});

glam_test!(test_hvec3_from_vec3, {
    assert_eq!(
        HVec3::from(glam::Vec3::new(1.0, 2.0, 3.0)),
        hvec3(f16x(1.0), f16x(2.0), f16x(3.0))
    );
    #[cfg(feature = "i32")]
    assert_eq!(
        HVec3::from(glam::IVec3::new(1, 2, 3)),
        hvec3(f16x(1.0), f16x(2.0), f16x(3.0))
    );
    #[cfg(feature = "u32")]
    assert_eq!(
        HVec3::from(glam::UVec3::new(1, 2, 3)),
        hvec3(f16x(1.0), f16x(2.0), f16x(3.0))
    );
    assert_eq!(
        HVec3::from(BVec3::new(true, false, true)),
        hvec3(f16x(1.0), f16x(0.0), f16x(1.0))
    );
});
