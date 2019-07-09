use glam::*;
#[cfg(feature = "rand")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "rand")]
use rand_xoshiro::Xoshiro256Plus;

#[test]
fn test_vec2_align() {
    use std::mem;
    assert_eq!(8, mem::size_of::<Vec2>());
    assert_eq!(4, mem::align_of::<Vec2>());
    assert_eq!(8, mem::size_of::<Vec2Mask>());
    assert_eq!(4, mem::align_of::<Vec2Mask>());
}

#[test]
fn test_vec2_new() {
    let v = vec2(1.0, 2.0);

    assert_eq!(v.x(), 1.0);
    assert_eq!(v.y(), 2.0);

    let t = (1.0, 2.0);
    let v = Vec2::from(t);
    assert_eq!(t, v.into());

    let a = [1.0, 2.0];
    let v = Vec2::from(a);
    let a1: [f32; 2] = v.into();
    assert_eq!(a, a1);

    let v = Vec2::new(t.0, t.1);
    assert_eq!(t, v.into());

    assert_eq!(Vec2::new(1.0, 0.0), Vec2::unit_x());
    assert_eq!(Vec2::new(0.0, 1.0), Vec2::unit_y());
}

#[test]
fn test_vec2_fmt() {
    let a = Vec2::new(1.0, 2.0);
    assert_eq!(format!("{:?}", a), "Vec2(1.0, 2.0)");
    // assert_eq!(format!("{:#?}", a), "Vec2(\n    1.0,\n    2.0\n)");
    assert_eq!(format!("{}", a), "(1, 2)");
}

#[test]
fn test_vec2_zero() {
    let v = Vec2::zero();
    assert_eq!(vec2(0.0, 0.0), v);
    assert_eq!(v, Vec2::default());
}

#[test]
fn test_vec2_splat() {
    let v = Vec2::splat(1.0);
    assert_eq!(vec2(1.0, 1.0), v);
}

#[test]
fn test_vec2_accessors() {
    let mut a = vec2(0.0, 0.0);
    a.set_x(1.0);
    a.set_y(2.0);
    assert_eq!(1.0, a.x());
    assert_eq!(2.0, a.y());
}

#[test]
fn test_vec2_funcs() {
    let x = vec2(1.0, 0.0);
    let y = vec2(0.0, 1.0);
    assert_eq!(1.0, x.dot(x));
    assert_eq!(0.0, x.dot(y));
    assert_eq!(-1.0, x.dot(-x));
    assert_eq!(4.0, (2.0 * x).length_squared());
    assert_eq!(9.0, (-3.0 * y).length_squared());
    assert_eq!(2.0, (-2.0 * x).length());
    assert_eq!(3.0, (3.0 * y).length());
    assert_eq!(x, (2.0 * x).normalize());
    assert_eq!(1.0 * 3.0 + 2.0 * 4.0, vec2(1.0, 2.0).dot(vec2(3.0, 4.0)));
    assert_eq!(2.0 * 2.0 + 3.0 * 3.0, vec2(2.0, 3.0).length_squared());
    assert_eq!((2.0_f32 * 2.0 + 3.0 * 3.0).sqrt(), vec2(2.0, 3.0).length());
    assert_eq!(
        1.0 / (2.0_f32 * 2.0 + 3.0 * 3.0).sqrt(),
        vec2(2.0, 3.0).length_reciprocal()
    );
    assert!(vec2(2.0, 3.0).normalize().is_normalized());
    assert_eq!(
        vec2(2.0, 3.0) / (2.0_f32 * 2.0 + 3.0 * 3.0).sqrt(),
        vec2(2.0, 3.0).normalize()
    );
}

#[test]
fn test_vec2_ops() {
    let a = vec2(1.0, 2.0);
    assert_eq!(vec2(2.0, 4.0), (a + a));
    assert_eq!(vec2(0.0, 0.0), (a - a));
    assert_eq!(vec2(1.0, 4.0), (a * a));
    assert_eq!(vec2(2.0, 4.0), (a * 2.0));
    assert_eq!(vec2(1.0, 1.0), (a / a));
    assert_eq!(vec2(0.5, 1.0), (a / 2.0));
    assert_eq!(vec2(-1.0, -2.0), (-a));
}

#[test]
fn test_vec2_assign_ops() {
    let a = vec2(1.0, 2.0);
    let mut b = a;
    b += a;
    assert_eq!(vec2(2.0, 4.0), b);
    b -= a;
    assert_eq!(vec2(1.0, 2.0), b);
    b *= a;
    assert_eq!(vec2(1.0, 4.0), b);
    b /= a;
    assert_eq!(vec2(1.0, 2.0), b);
    b *= 2.0;
    assert_eq!(vec2(2.0, 4.0), b);
    b /= 2.0;
    assert_eq!(vec2(1.0, 2.0), b);
}

#[test]
fn test_vec2_min_max() {
    let a = vec2(-1.0, 2.0);
    let b = vec2(1.0, -2.0);
    assert_eq!(vec2(-1.0, -2.0), a.min(b));
    assert_eq!(vec2(-1.0, -2.0), b.min(a));
    assert_eq!(vec2(1.0, 2.0), a.max(b));
    assert_eq!(vec2(1.0, 2.0), b.max(a));
}

#[test]
fn test_vec2_hmin_hmax() {
    let a = vec2(-1.0, 2.0);
    assert_eq!(-1.0, a.min_element());
    assert_eq!(2.0, a.max_element());
}

#[test]
fn test_vec2_eq() {
    let a = vec2(1.0, 1.0);
    let b = vec2(1.0, 2.0);
    assert!(a.cmpeq(a).all());
    assert!(b.cmpeq(b).all());
    assert!(a.cmpne(b).any());
    assert!(b.cmpne(a).any());
    assert!(b.cmpeq(a).any());
}

#[test]
fn test_vec2_cmp() {
    assert!(!Vec2Mask::default().any());
    assert!(!Vec2Mask::default().all());
    assert_eq!(Vec2Mask::default().bitmask(), 0x0);
    let a = vec2(-1.0, -1.0);
    let b = vec2(1.0, 1.0);
    let c = vec2(-1.0, -1.0);
    let d = vec2(1.0, -1.0);
    assert_eq!(a.cmplt(a).bitmask(), 0x0);
    assert_eq!(a.cmplt(b).bitmask(), 0x3);
    assert_eq!(a.cmplt(d).bitmask(), 0x1);
    assert_eq!(c.cmple(a).bitmask(), 0x3);
    assert!(a.cmplt(b).all());
    assert!(a.cmplt(d).any());
    assert!(a.cmple(b).all());
    assert!(a.cmple(a).all());
    assert!(b.cmpgt(a).all());
    assert!(b.cmpge(a).all());
    assert!(b.cmpge(b).all());
    assert!(!(a.cmpge(d).all()));
    assert!(c.cmple(c).all());
    assert!(c.cmpge(c).all());
}

#[test]
fn test_extend_truncate() {
    let a = vec2(1.0, 2.0);
    let b = a.extend(3.0);
    assert_eq!(vec3(1.0, 2.0, 3.0), b);
}

#[test]
fn test_vec2b() {
    // make sure the unused 'w' value doesn't break Vec2b behaviour
    let a = Vec3::zero();
    let mut b = a.truncate();
    b.set_x(1.0);
    b.set_y(1.0);
    assert!(!b.cmpeq(Vec2::zero()).any());
    assert!(b.cmpeq(Vec2::splat(1.0)).all());
}

#[cfg(feature = "rand")]
#[test]
fn test_vec2_rand() {
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: (f32, f32) = rng1.gen();
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Vec2 = rng2.gen();
    assert_eq!(a, b.into());
}

#[cfg(feature = "serde")]
#[test]
fn test_vec2_serde() {
    let a = Vec2::new(1.0, 2.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Vec2>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec2>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec2>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
}
