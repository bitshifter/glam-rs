#![allow(clippy::excessive_precision)]
#![cfg(feature = "f16")]

#[macro_use]
mod support;

use glam::{hvec4, BVec4, HVec4};
use half::f16;

fn f16x(v: f32) -> f16 {
    f16::from_f32(v)
}

glam_test!(test_hvec4_consts, {
    assert_eq!(
        HVec4::ZERO,
        hvec4(f16x(0.0), f16x(0.0), f16x(0.0), f16x(0.0))
    );
    assert_eq!(
        HVec4::ONE,
        hvec4(f16x(1.0), f16x(1.0), f16x(1.0), f16x(1.0))
    );
    assert_eq!(HVec4::X, hvec4(f16x(1.0), f16x(0.0), f16x(0.0), f16x(0.0)));
    assert_eq!(HVec4::Y, hvec4(f16x(0.0), f16x(1.0), f16x(0.0), f16x(0.0)));
    assert_eq!(HVec4::Z, hvec4(f16x(0.0), f16x(0.0), f16x(1.0), f16x(0.0)));
    assert_eq!(HVec4::W, hvec4(f16x(0.0), f16x(0.0), f16x(0.0), f16x(1.0)));
    assert_eq!(HVec4::MIN, hvec4(f16::MIN, f16::MIN, f16::MIN, f16::MIN));
    assert_eq!(HVec4::MAX, hvec4(f16::MAX, f16::MAX, f16::MAX, f16::MAX));
});

glam_test!(test_hvec4_new, {
    let v = hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    assert_eq!(v.x, f16x(1.0));
    assert_eq!(v.y, f16x(2.0));
    assert_eq!(v.z, f16x(3.0));
    assert_eq!(v.w, f16x(4.0));

    let t = (f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    let v = HVec4::from(t);
    assert_eq!(t, v.into());

    let a = [f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0)];
    let v = HVec4::from(a);
    let a1: [f16; 4] = v.into();
    assert_eq!(a, a1);
    assert_eq!(a, v.to_array());
    assert_eq!(a, *v.as_ref());

    assert_eq!(
        HVec4::new(f16x(1.0), f16x(0.0), f16x(0.0), f16x(0.0)),
        BVec4::new(true, false, false, false).into()
    );

    assert_eq!(
        hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0)),
        hvec3_extend()
    );
});

fn hvec3_extend() -> HVec4 {
    glam::hvec3(f16x(1.0), f16x(2.0), f16x(3.0)).extend(f16x(4.0))
}

glam_test!(test_hvec4_fmt, {
    let a = hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    assert_eq!(
        format!("{:?}", a),
        format!("HVec4({:?}, {:?}, {:?}, {:?})", a.x, a.y, a.z, a.w)
    );
    assert_eq!(format!("{}", a), "[1, 2, 3, 4]");
});

glam_test!(test_hvec4_splat_map_with, {
    assert_eq!(HVec4::splat(f16x(1.0)), HVec4::ONE);
    let v = hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    assert_eq!(v.map(|n| n + f16x(1.0)), v + HVec4::ONE);
    assert_eq!(HVec4::X, HVec4::ZERO.with_x(f16x(1.0)));
    assert_eq!(HVec4::Y, HVec4::ZERO.with_y(f16x(1.0)));
    assert_eq!(HVec4::Z, HVec4::ZERO.with_z(f16x(1.0)));
    assert_eq!(HVec4::W, HVec4::ZERO.with_w(f16x(1.0)));
});

glam_test!(test_hvec4_ops, {
    let a = hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    let b = hvec4(f16x(3.0), f16x(4.0), f16x(5.0), f16x(6.0));
    assert_eq!(a + b, hvec4(f16x(4.0), f16x(6.0), f16x(8.0), f16x(10.0)));
    assert_eq!(a - b, hvec4(f16x(-2.0), f16x(-2.0), f16x(-2.0), f16x(-2.0)));
    assert_eq!(a * b, hvec4(f16x(3.0), f16x(8.0), f16x(15.0), f16x(24.0)));
    assert_eq!(
        a / b,
        hvec4(f16x(1.0 / 3.0), f16x(0.5), f16x(0.6), f16x(4.0 / 6.0))
    );
    assert_eq!(-a, hvec4(f16x(-1.0), f16x(-2.0), f16x(-3.0), f16x(-4.0)));
    assert_eq!(
        a * f16x(2.0),
        hvec4(f16x(2.0), f16x(4.0), f16x(6.0), f16x(8.0))
    );
    assert_eq!(
        f16x(2.0) * a,
        hvec4(f16x(2.0), f16x(4.0), f16x(6.0), f16x(8.0))
    );
});

glam_test!(test_hvec4_dot, {
    let x = hvec4(f16x(1.0), f16x(0.0), f16x(0.0), f16x(0.0));
    let y = hvec4(f16x(0.0), f16x(1.0), f16x(0.0), f16x(0.0));
    assert_eq!(f16x(1.0), x.dot(x));
    assert_eq!(f16x(0.0), x.dot(y));
});

glam_test!(test_hvec4_length, {
    let v = hvec4(f16x(2.0), f16x(3.0), f16x(6.0), f16x(0.0));
    assert_eq!(f16x(49.0), v.length_squared());
    assert_eq!(f16x(7.0), v.length());
    assert_eq!(f16x(1.0 / 7.0), v.length_recip());
    assert_eq!(f16x(7.0), v.distance(HVec4::ZERO));
    assert_eq!(f16x(49.0), v.distance_squared(HVec4::ZERO));
});

glam_test!(test_hvec4_normalize, {
    let v = hvec4(f16x(2.0), f16x(3.0), f16x(6.0), f16x(0.0));
    assert!(v.normalize().is_normalized());
    assert_eq!(v.normalize(), v.normalize_or_zero());
    let (n, l) = v.normalize_and_length();
    assert_eq!(n, v.normalize());
    assert_eq!(l, f16x(7.0));
    assert_eq!(HVec4::ZERO.try_normalize(), None);
});

glam_test!(test_hvec4_minmax, {
    let a = hvec4(f16x(2.0), f16x(3.0), f16x(1.0), f16x(4.0));
    let b = hvec4(f16x(1.0), f16x(4.0), f16x(2.0), f16x(3.0));
    assert_eq!(a.min(b), hvec4(f16x(1.0), f16x(3.0), f16x(1.0), f16x(3.0)));
    assert_eq!(a.max(b), hvec4(f16x(2.0), f16x(4.0), f16x(2.0), f16x(4.0)));
    assert_eq!(a.min_element(), f16x(1.0));
    assert_eq!(a.max_element(), f16x(4.0));
    assert_eq!(a.min_position(), 2);
    assert_eq!(a.max_position(), 3);
    assert_eq!(a.element_sum(), f16x(10.0));
    assert_eq!(a.element_product(), f16x(24.0));
});

glam_test!(test_hvec4_abs, {
    let a = hvec4(f16x(-1.5), f16x(2.5), f16x(-3.5), f16x(4.5));
    assert_eq!(a.abs(), hvec4(f16x(1.5), f16x(2.5), f16x(3.5), f16x(4.5)));
    assert_eq!(
        a.signum(),
        hvec4(f16x(-1.0), f16x(1.0), f16x(-1.0), f16x(1.0))
    );
    assert_eq!(a.is_negative_bitmask(), 0b0101);
    assert!(a.is_finite());
});

glam_test!(test_hvec4_rounding, {
    let v = hvec4(f16x(1.5), f16x(-1.5), f16x(2.5), f16x(-2.5));
    assert_eq!(
        v.round(),
        hvec4(f16x(2.0), f16x(-2.0), f16x(3.0), f16x(-3.0))
    );
    assert_eq!(
        v.floor(),
        hvec4(f16x(1.0), f16x(-2.0), f16x(2.0), f16x(-3.0))
    );
    assert_eq!(
        v.ceil(),
        hvec4(f16x(2.0), f16x(-1.0), f16x(3.0), f16x(-2.0))
    );
    assert_eq!(
        v.trunc(),
        hvec4(f16x(1.0), f16x(-1.0), f16x(2.0), f16x(-2.0))
    );
});

glam_test!(test_hvec4_lerp, {
    let a = hvec4(f16x(0.0), f16x(0.0), f16x(0.0), f16x(0.0));
    let b = hvec4(f16x(2.0), f16x(4.0), f16x(6.0), f16x(8.0));
    assert_eq!(
        a.lerp(b, f16x(0.5)),
        hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0))
    );
    assert_eq!(
        a.midpoint(b),
        hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0))
    );
    assert!(a.abs_diff_eq(b, f16x(9.0)));
    assert!(!(a.abs_diff_eq(b, f16x(7.0))));
});

glam_test!(test_hvec4_clamp_length, {
    let v = hvec4(f16x(2.0), f16x(3.0), f16x(6.0), f16x(0.0));
    assert_eq!(
        v.clamp_length(f16x(7.0), f16x(7.0)),
        v.normalize() * f16x(7.0)
    );
    assert_eq!(v.clamp_length_max(f16x(7.0)), v);
    assert_eq!(v.clamp_length_min(f16x(7.0)), v);
});

glam_test!(test_hvec4_as_conversions, {
    let v = hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    assert_eq!(v.as_vec4(), glam::vec4(1.0, 2.0, 3.0, 4.0));
    #[cfg(feature = "f64")]
    assert_eq!(v.as_dvec4(), glam::dvec4(1.0, 2.0, 3.0, 4.0));
    #[cfg(feature = "i8")]
    assert_eq!(v.as_i8vec4(), glam::i8vec4(1, 2, 3, 4));
    #[cfg(feature = "u8")]
    assert_eq!(v.as_u8vec4(), glam::u8vec4(1, 2, 3, 4));
    #[cfg(feature = "i16")]
    assert_eq!(v.as_i16vec4(), glam::i16vec4(1, 2, 3, 4));
    #[cfg(feature = "u16")]
    assert_eq!(v.as_u16vec4(), glam::u16vec4(1, 2, 3, 4));
    #[cfg(feature = "i32")]
    assert_eq!(v.as_ivec4(), glam::ivec4(1, 2, 3, 4));
    #[cfg(feature = "u32")]
    assert_eq!(v.as_uvec4(), glam::uvec4(1, 2, 3, 4));
    #[cfg(feature = "i64")]
    assert_eq!(v.as_i64vec4(), glam::i64vec4(1, 2, 3, 4));
    #[cfg(feature = "u64")]
    assert_eq!(v.as_u64vec4(), glam::u64vec4(1, 2, 3, 4));
    #[cfg(feature = "isize")]
    assert_eq!(v.as_isizevec4(), glam::isizevec4(1, 2, 3, 4));
    #[cfg(feature = "usize")]
    assert_eq!(v.as_usizevec4(), glam::usizevec4(1, 2, 3, 4));
});

glam_test!(test_hvec4_from_vec4, {
    assert_eq!(
        HVec4::from(glam::Vec4::new(1.0, 2.0, 3.0, 4.0)),
        hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0))
    );
    #[cfg(feature = "i32")]
    assert_eq!(
        HVec4::from(glam::IVec4::new(1, 2, 3, 4)),
        hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0))
    );
    #[cfg(feature = "u32")]
    assert_eq!(
        HVec4::from(glam::UVec4::new(1, 2, 3, 4)),
        hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0))
    );
    assert_eq!(
        HVec4::from(BVec4::new(true, false, true, false)),
        hvec4(f16x(1.0), f16x(0.0), f16x(1.0), f16x(0.0))
    );
});
