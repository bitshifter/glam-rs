extern crate glam;

macro_rules! impl_vec3_tests {
    ($mod_name:ident) => {
        mod $mod_name {
            use glam::$mod_name::*;
            use std::mem;

            #[test]
            fn test_vec3_new() {
                let v = vec3(1.0, 2.0, 3.0);

                assert_eq!(mem::size_of_val(&v), 16);
                assert_eq!(mem::align_of_val(&v), 16);

                assert_eq!(v.get_x(), 1.0);
                assert_eq!(v.get_y(), 2.0);
                assert_eq!(v.get_z(), 3.0);

                let t = (1.0, 2.0, 3.0);
                let v = Vec3::from(t);
                assert_eq!(t, v.into());

                let a = [1.0, 2.0, 3.0];
                let v = Vec3::from(a);
                let a1: [f32; 3] = v.into();
                assert_eq!(a, a1);

                let v = Vec3::new(t.0, t.1, t.2);
                assert_eq!(t, v.into());
            }

            #[test]
            fn test_vec3_zero() {
                let v = Vec3::zero();
                assert_eq!((0.0, 0.0, 0.0), v.into());
            }

            #[test]
            fn test_vec3_splat() {
                let v = Vec3::splat(1.0);
                assert_eq!((1.0, 1.0, 1.0), v.into());
            }

            #[test]
            fn test_vec3_min_max() {
                let a = vec3(-1.0, 2.0, -3.0);
                let b = vec3(1.0, -2.0, 3.0);
                assert_eq!((-1.0, -2.0, -3.0), a.min(b).into());
                assert_eq!((-1.0, -2.0, -3.0), b.min(a).into());
                assert_eq!((1.0, 2.0, 3.0), a.max(b).into());
                assert_eq!((1.0, 2.0, 3.0), b.max(a).into());
            }

            #[test]
            fn test_vec3_hmin_hmax() {
                let a = vec3(-1.0, 2.0, -3.0);
                assert_eq!(-3.0, a.hmin());
                assert_eq!(2.0, a.hmax());
            }

            #[test]
            fn test_vec3_partial_eq() {
                let a = vec3(1.0, 1.0, 1.0);
                let b = vec3(1.0, 2.0, 3.0);
                assert!(a == a);
                assert!(b == b);
                assert!(a != b);
                assert!(b != a);
            }

            #[test]
            fn test_vec3_partial_ord() {
                let a = vec3(-1.0, -1.0, -1.0);
                let b = vec3(1.0, 1.0, 1.0);
                let c = vec3(1.0, -1.0, 1.0);
                assert!(a < b);
                assert!(a <= b);
                assert!(a <= a);
                assert!(b > a);
                assert!(b >= a);
                assert!(b >= b);
                assert!(!(a < c));
                assert!(!(a <= c));
                assert!(!(a >= c));
                assert!(c <= c);
                assert!(c >= c);
            }
        }
    };
}

impl_vec3_tests!(f32);

#[cfg(target_feature = "sse2")]
impl_vec3_tests!(sse2);
