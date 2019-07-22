use approx::assert_ulps_eq;
use glam::*;
#[cfg(feature = "rand")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "rand")]
use rand_xoshiro::Xoshiro256Plus;

#[test]
fn test_vec4_align() {
    use std::mem;
    assert_eq!(16, mem::size_of::<Vec4>());
    assert_eq!(16, mem::size_of::<Vec4Mask>());
    if cfg!(feature = "scalar-math") {
        assert_eq!(4, mem::align_of::<Vec4>());
        assert_eq!(4, mem::align_of::<Vec4Mask>());
    } else {
        assert_eq!(16, mem::align_of::<Vec4>());
        assert_eq!(16, mem::align_of::<Vec4Mask>());
    }
}

#[test]
fn test_vec4_new() {
    let v = vec4(1.0, 2.0, 3.0, 4.0);

    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 3.0);
    assert_eq!(v.w(), 4.0);

    let t = (1.0, 2.0, 3.0, 4.0);
    let v = Vec4::from(t);
    assert_eq!(t, v.into());

    let a = [1.0, 2.0, 3.0, 4.0];
    let v = Vec4::from(a);
    let a1: [f32; 4] = v.into();
    assert_eq!(a, a1);

    let v = Vec4::new(t.0, t.1, t.2, t.3);
    assert_eq!(t, v.into());

    assert_eq!(Vec4::new(1.0, 0.0, 0.0, 0.0), Vec4::unit_x());
    assert_eq!(Vec4::new(0.0, 1.0, 0.0, 0.0), Vec4::unit_y());
    assert_eq!(Vec4::new(0.0, 0.0, 1.0, 0.0), Vec4::unit_z());
    assert_eq!(Vec4::new(0.0, 0.0, 0.0, 1.0), Vec4::unit_w());
}

#[test]
fn test_vec4_fmt() {
    let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(format!("{:?}", a), "Vec4(1.0, 2.0, 3.0, 4.0)");
    // assert_eq!(
    //     format!("{:#?}", a),
    //     "Vec4(\n    1.0,\n    2.0,\n    3.0,\n    4.0\n)"
    // );
    assert_eq!(format!("{}", a), "(1, 2, 3, 4)");
}

#[test]
fn test_vec4_zero() {
    let v = Vec4::zero();
    assert_eq!((0.0, 0.0, 0.0, 0.0), v.into());
    assert_eq!(v, Vec4::default());
}

#[test]
fn test_vec4_splat() {
    let v = Vec4::splat(1.0);
    assert_eq!((1.0, 1.0, 1.0, 1.0), v.into());
}

#[test]
fn test_vec4_accessors() {
    let mut a = vec4(0.0, 0.0, 0.0, 0.0);
    a.set_x(1.0);
    a.set_y(2.0);
    a.set_z(3.0);
    a.set_w(4.0);
    assert_eq!(1.0, a.x());
    assert_eq!(2.0, a.y());
    assert_eq!(3.0, a.z());
    assert_eq!(4.0, a.w());
    assert_eq!((1.0, 2.0, 3.0, 4.0), a.into());
}

#[test]
fn test_vec4_funcs() {
    let x = vec4(1.0, 0.0, 0.0, 0.0);
    let y = vec4(0.0, 1.0, 0.0, 0.0);
    let z = vec4(0.0, 0.0, 1.0, 0.0);
    let w = vec4(0.0, 0.0, 0.0, 1.0);
    assert_eq!(1.0, x.dot(x));
    assert_eq!(0.0, x.dot(y));
    assert_eq!(-1.0, z.dot(-z));
    assert_eq!(4.0, (2.0 * x).length_squared());
    assert_eq!(9.0, (-3.0 * y).length_squared());
    assert_eq!(16.0, (4.0 * z).length_squared());
    assert_eq!(64.0, (8.0 * w).length_squared());
    assert_eq!(2.0, (-2.0 * x).length());
    assert_eq!(3.0, (3.0 * y).length());
    assert_eq!(4.0, (-4.0 * z).length());
    assert_eq!(5.0, (-5.0 * w).length());
    assert_eq!(x, (2.0 * x).normalize());
    assert_eq!(
        1.0 * 5.0 + 2.0 * 6.0 + 3.0 * 7.0 + 4.0 * 8.0,
        vec4(1.0, 2.0, 3.0, 4.0).dot(vec4(5.0, 6.0, 7.0, 8.0))
    );
    assert_eq!(
        2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0,
        vec4(2.0, 3.0, 4.0, 5.0).length_squared()
    );
    assert_eq!(
        (2.0_f32 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
        vec4(2.0, 3.0, 4.0, 5.0).length()
    );
    assert_eq!(
        1.0 / (2.0_f32 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
        vec4(2.0, 3.0, 4.0, 5.0).length_reciprocal()
    );
    assert!(vec4(2.0, 3.0, 4.0, 5.0).normalize().is_normalized());
    assert_ulps_eq!(
        vec4(2.0, 3.0, 4.0, 5.0) / (2.0_f32 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
        vec4(2.0, 3.0, 4.0, 5.0).normalize()
    );
}

#[test]
fn test_vec4_ops() {
    let a = vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!((2.0, 4.0, 6.0, 8.0), (a + a).into());
    assert_eq!((0.0, 0.0, 0.0, 0.0), (a - a).into());
    assert_eq!((1.0, 4.0, 9.0, 16.0), (a * a).into());
    assert_eq!((2.0, 4.0, 6.0, 8.0), (a * 2.0).into());
    assert_eq!((2.0, 4.0, 6.0, 8.0), (2.0 * a).into());
    assert_eq!((1.0, 1.0, 1.0, 1.0), (a / a).into());
    assert_eq!((0.5, 1.0, 1.5, 2.0), (a / 2.0).into());
    // is this a sensible operator?
    // assert_eq!((1.0, 0.5, 1.0/3.0, 0.25), (1.0 / a).into());
    assert_eq!((-1.0, -2.0, -3.0, -4.0), (-a).into());
}

#[test]
fn test_vec4_assign_ops() {
    let a = vec4(1.0, 2.0, 3.0, 4.0);
    let mut b = a;
    b += a;
    assert_eq!((2.0, 4.0, 6.0, 8.0), b.into());
    b -= a;
    assert_eq!((1.0, 2.0, 3.0, 4.0), b.into());
    b *= a;
    assert_eq!((1.0, 4.0, 9.0, 16.0), b.into());
    b /= a;
    assert_eq!((1.0, 2.0, 3.0, 4.0), b.into());
    b *= 2.0;
    assert_eq!((2.0, 4.0, 6.0, 8.0), b.into());
    b /= 2.0;
    assert_eq!((1.0, 2.0, 3.0, 4.0), b.into());
}

#[test]
fn test_vec4_min_max() {
    let a = vec4(-1.0, 2.0, -3.0, 4.0);
    let b = vec4(1.0, -2.0, 3.0, -4.0);
    assert_eq!((-1.0, -2.0, -3.0, -4.0), a.min(b).into());
    assert_eq!((-1.0, -2.0, -3.0, -4.0), b.min(a).into());
    assert_eq!((1.0, 2.0, 3.0, 4.0), a.max(b).into());
    assert_eq!((1.0, 2.0, 3.0, 4.0), b.max(a).into());
}

#[test]
fn test_vec4_hmin_hmax() {
    let a = vec4(-1.0, 4.0, -3.0, 2.0);
    assert_eq!(-3.0, a.min_element());
    assert_eq!(4.0, a.max_element());
    assert_eq!(3.0, vec4(1.0, 2.0, 3.0, 4.0).truncate().max_element());
    assert_eq!(-3.0, vec4(-1.0, -2.0, -3.0, -4.0).truncate().min_element());
}

#[test]
fn test_vec4_eq() {
    let a = vec4(1.0, 1.0, 1.0, 1.0);
    let b = vec4(1.0, 2.0, 3.0, 4.0);
    assert!(a.cmpeq(a).all());
    assert!(b.cmpeq(b).all());
    assert!(a.cmpne(b).any());
    assert!(b.cmpne(a).any());
    assert!(b.cmpeq(a).any());
}

#[test]
fn test_vec4_cmp() {
    assert!(!Vec4Mask::default().any());
    assert!(!Vec4Mask::default().all());
    assert_eq!(Vec4Mask::default().bitmask(), 0x0);
    let a = vec4(-1.0, -1.0, -1.0, -1.0);
    let b = vec4(1.0, 1.0, 1.0, 1.0);
    let c = vec4(-1.0, -1.0, 1.0, 1.0);
    let d = vec4(1.0, -1.0, -1.0, 1.0);
    assert_eq!(a.cmplt(a).bitmask(), 0x0);
    assert_eq!(a.cmplt(b).bitmask(), 0xf);
    assert_eq!(a.cmplt(c).bitmask(), 0xc);
    assert_eq!(c.cmple(a).bitmask(), 0x3);
    assert_eq!(a.cmplt(d).bitmask(), 0x9);
    assert!(a.cmplt(b).all());
    assert!(a.cmplt(c).any());
    assert!(a.cmple(b).all());
    assert!(a.cmple(a).all());
    assert!(b.cmpgt(a).all());
    assert!(b.cmpge(a).all());
    assert!(b.cmpge(b).all());
    assert!(!(a.cmpge(c).all()));
    assert!(c.cmple(c).all());
    assert!(c.cmpge(c).all());
    assert!(a.cmpeq(a).all());
    assert!(!a.cmpeq(b).all());
    assert!(a.cmpeq(c).any());
    assert!(!a.cmpne(a).all());
    assert!(a.cmpne(b).all());
    assert!(a.cmpne(c).any());
}

#[cfg(feature = "rand")]
#[test]
fn test_vec4_rand() {
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: (f32, f32, f32, f32) = rng1.gen();
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Vec4 = rng2.gen();
    assert_eq!(a, b.into());
}

#[test]
fn test_vec4_slice() {
    let a = [1.0, 2.0, 3.0, 4.0];
    let b = Vec4::from_slice_unaligned(&a);
    let c: [f32; 4] = b.into();
    assert_eq!(a, c);
    let mut d = [0.0, 0.0, 0.0, 0.0];
    b.write_to_slice_unaligned(&mut d[..]);
    assert_eq!(a, d);
}

#[test]
fn test_vec4_sign() {
    assert_eq!(Vec4::zero().sign(), Vec4::one());
    assert_eq!(Vec4::one().sign(), Vec4::one());
    assert_eq!((-Vec4::one()).sign(), -Vec4::one());
}

// #[test]
// fn dup_element() {
//     let a = vec4(1.0, 2.0, 3.0, 4.0);
//     assert_eq!(vec4(1.0, 1.0, 1.0, 1.0), a.dup_x());
//     assert_eq!(vec4(2.0, 2.0, 2.0, 2.0), a.dup_y());
//     assert_eq!(vec4(3.0, 3.0, 3.0, 3.0), a.dup_z());
//     assert_eq!(vec4(4.0, 4.0, 4.0, 4.0), a.dup_w());
// }

#[test]
fn test_vec4mask_bitmask() {
    assert_eq!(Vec4Mask::new(false, false, false, false).bitmask(), 0b0000);
    assert_eq!(Vec4Mask::new(false, false, true, true).bitmask(),   0b1100);
    assert_eq!(Vec4Mask::new(true, true, false, false).bitmask(),   0b0011);
    assert_eq!(Vec4Mask::new(false, true, false, true).bitmask(),   0b1010);
    assert_eq!(Vec4Mask::new(true, false, true, false).bitmask(),   0b0101);
    assert_eq!(Vec4Mask::new(true, true, true, true).bitmask(),     0b1111);
}

#[test]
fn test_vec4mask_any() {
    assert_eq!(Vec4Mask::new(false, false, false, false).any(), false);
    assert_eq!(Vec4Mask::new(true, false, false, false).any(), true);
    assert_eq!(Vec4Mask::new(false, true, false, false).any(), true);
    assert_eq!(Vec4Mask::new(false, false, true, false).any(), true);
    assert_eq!(Vec4Mask::new(false, false, false, true).any(), true);
}

#[test]
fn test_vec4mask_all() {
    assert_eq!(Vec4Mask::new(true, true, true, true).all(), true);
    assert_eq!(Vec4Mask::new(false, true, true, true).all(), false);
    assert_eq!(Vec4Mask::new(true, false, true, true).all(), false);
    assert_eq!(Vec4Mask::new(true, true, false, true).all(), false);
    assert_eq!(Vec4Mask::new(true, true, true, false).all(), false);
}

#[test]
fn test_vec4mask_select() {
    let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert_eq!(
        Vec4Mask::new(true, true, true, true).select(a, b),
        Vec4::new(1.0, 2.0, 3.0, 4.0),
    );
    assert_eq!(
        Vec4Mask::new(true, false, true, false).select(a, b),
        Vec4::new(1.0, 6.0, 3.0, 8.0),
    );
    assert_eq!(
        Vec4Mask::new(false, true, false, true).select(a, b),
        Vec4::new(5.0, 2.0, 7.0, 4.0),
    );
    assert_eq!(
        Vec4Mask::new(false, false, false, false).select(a, b),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
    );
}

#[test]
fn test_vec4mask_and() {
    assert_eq!(
        (
            Vec4Mask::new(false, false, false, false)
            & Vec4Mask::new(false, false, false, false)
        ).bitmask(),
        0b0000,
    );
    assert_eq!(
        (
            Vec4Mask::new(true, true, true, true)
            & Vec4Mask::new(true, true, true, true)
        ).bitmask(),
        0b1111,
    );
    assert_eq!(
        (
            Vec4Mask::new(true, false, true, false)
            & Vec4Mask::new(false, true, false, true)
        ).bitmask(),
        0b0000,
    );
    assert_eq!(
        (
            Vec4Mask::new(true, false, true, true)
            & Vec4Mask::new(true, true, true, false)
        ).bitmask(),
        0b0101,
    );

    let mut mask = Vec4Mask::new(true, true, false, false);
    mask &= Vec4Mask::new(true, false, true, false);
    assert_eq!(mask.bitmask(), 0b0001);
}

#[test]
fn test_vec4mask_or() {
    assert_eq!(
        (
            Vec4Mask::new(false, false, false, false)
            | Vec4Mask::new(false, false, false, false)
        ).bitmask(),
        0b0000,
    );
    assert_eq!(
        (
            Vec4Mask::new(true, true, true, true)
            | Vec4Mask::new(true, true, true, true)
        ).bitmask(),
        0b1111,
    );
    assert_eq!(
        (
            Vec4Mask::new(true, false, true, false)
            | Vec4Mask::new(false, true, false, true)
        ).bitmask(),
        0b1111,
    );
    assert_eq!(
        (
            Vec4Mask::new(true, false, true, false)
            | Vec4Mask::new(true, false, true, false)
        ).bitmask(),
        0b0101,
    );

    let mut mask = Vec4Mask::new(true, true, false, false);
    mask |= Vec4Mask::new(true, false, true, false);
    assert_eq!(mask.bitmask(), 0b0111);
}

#[test]
fn test_vec4mask_not() {
    assert_eq!((!Vec4Mask::new(false, false, false, false)).bitmask(), 0b1111);
    assert_eq!((!Vec4Mask::new(true, true, true, true)).bitmask(), 0b0000);
    assert_eq!((!Vec4Mask::new(true, false, true, false)).bitmask(), 0b1010);
    assert_eq!((!Vec4Mask::new(false, true, false, true)).bitmask(), 0b0101);
}

#[cfg(feature = "serde")]
#[test]
fn test_vec4_serde() {
    let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Vec4>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
}
