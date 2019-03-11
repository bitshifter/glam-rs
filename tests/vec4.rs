use glam::*;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::mem;

#[test]
fn test_vec4_new() {
    let v = vec4(1.0, 2.0, 3.0, 4.0);

    assert_eq!(mem::size_of_val(&v), 16);
    assert_eq!(mem::align_of_val(&v), 16);

    assert_eq!(v.get_x(), 1.0);
    assert_eq!(v.get_y(), 2.0);
    assert_eq!(v.get_z(), 3.0);
    assert_eq!(v.get_w(), 4.0);

    let t = (1.0, 2.0, 3.0, 4.0);
    let v = Vec4::from(t);
    assert_eq!(t, v.into());

    let a = [1.0, 2.0, 3.0, 4.0];
    let v = Vec4::from(a);
    let a1: [f32; 4] = v.into();
    assert_eq!(a, a1);

    let v = Vec4::new(t.0, t.1, t.2, t.3);
    assert_eq!(t, v.into());
}

#[test]
fn test_vec4_zero() {
    let v = Vec4::zero();
    assert_eq!((0.0, 0.0, 0.0, 0.0), v.into());
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
    assert_eq!(1.0, a.get_x());
    assert_eq!(2.0, a.get_y());
    assert_eq!(3.0, a.get_z());
    assert_eq!(4.0, a.get_w());
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

// #[test]
// fn test_vec4_hmin_hmax() {
//     let a = vec4(-1.0, 2.0, -3.0);
//     assert_eq!(-3.0, a.hmin());
//     assert_eq!(2.0, a.hmax());
// }

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
    let a = vec4(-1.0, -1.0, -1.0, -1.0);
    let b = vec4(1.0, 1.0, 1.0, 1.0);
    let c = vec4(-1.0, -1.0, 1.0, 1.0);
    let d = vec4(1.0, -1.0, -1.0, 1.0);
    assert_eq!(a.cmplt(a).mask(), 0x0);
    assert_eq!(a.cmplt(b).mask(), 0xf);
    assert_eq!(a.cmplt(c).mask(), 0xc);
    assert_eq!(c.cmple(a).mask(), 0x3);
    assert_eq!(a.cmplt(d).mask(), 0x9);
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
}

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
    let b = Vec4::load_from_slice(&a);
    let c: [f32; 4] = b.into();
    assert_eq!(a, c);
    let mut d = [0.0, 0.0, 0.0, 0.0];
    b.store_to_slice(&mut d[..]);
    assert_eq!(a, d);
}

#[test]
fn dup_element() {
    let a = vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(vec4(1.0, 1.0, 1.0, 1.0), a.dup_x());
    assert_eq!(vec4(2.0, 2.0, 2.0, 2.0), a.dup_y());
    assert_eq!(vec4(3.0, 3.0, 3.0, 3.0), a.dup_z());
    assert_eq!(vec4(4.0, 4.0, 4.0, 4.0), a.dup_w());
}

