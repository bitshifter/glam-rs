#[macro_use]
mod support;

macro_rules! impl_vec4_tests {
    ($t:ident, $new:ident, $vec4:ident, $mask:ident) => {
        #[test]
        fn test_new() {
            let v = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);

            assert_eq!(v.x, 1 as $t);
            assert_eq!(v.y, 2 as $t);
            assert_eq!(v.z, 3 as $t);
            assert_eq!(v.w, 4 as $t);

            let t = (1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let v = $vec4::from(t);
            assert_eq!(t, v.into());

            let a = [1 as $t, 2 as $t, 3 as $t, 4 as $t];
            let v = $vec4::from(a);
            let a1: [$t; 4] = v.into();
            assert_eq!(a, a1);

            let v = $vec4::new(t.0, t.1, t.2, t.3);
            assert_eq!(t, v.into());

            assert_eq!(
                $vec4::new(1 as $t, 0 as $t, 0 as $t, 0 as $t),
                $vec4::unit_x()
            );
            assert_eq!(
                $vec4::new(0 as $t, 1 as $t, 0 as $t, 0 as $t),
                $vec4::unit_y()
            );
            assert_eq!(
                $vec4::new(0 as $t, 0 as $t, 1 as $t, 0 as $t),
                $vec4::unit_z()
            );
            assert_eq!(
                $vec4::new(0 as $t, 0 as $t, 0 as $t, 1 as $t),
                $vec4::unit_w()
            );
        }

        #[test]
        fn test_fmt() {
            let a = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert_eq!(
                format!("{:?}", a),
                format!(
                    "{}({:?}, {:?}, {:?}, {:?})",
                    stringify!($vec4),
                    a.x,
                    a.y,
                    a.z,
                    a.w
                )
            );
            // assert_eq!(
            //     format!("{:#?}", a),
            //     "$vec4(\n    1.0,\n    2.0,\n    3.0,\n    4.0\n)"
            // );
            assert_eq!(format!("{}", a), "[1, 2, 3, 4]");
        }

        #[test]
        fn test_zero() {
            let v = $vec4::zero();
            assert_eq!((0 as $t, 0 as $t, 0 as $t, 0 as $t), v.into());
            assert_eq!(v, $vec4::default());
        }

        #[test]
        fn test_splat() {
            let v = $vec4::splat(1 as $t);
            assert_eq!($vec4::one(), v);
        }

        #[test]
        fn test_accessors() {
            let mut a = $vec4::zero();
            a.x = 1 as $t;
            a.y = 2 as $t;
            a.z = 3 as $t;
            a.w = 4 as $t;
            assert_eq!(1 as $t, a.x);
            assert_eq!(2 as $t, a.y);
            assert_eq!(3 as $t, a.z);
            assert_eq!(4 as $t, a.w);
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), a.into());

            let mut a = $vec4::zero();
            a[0] = 1 as $t;
            a[1] = 2 as $t;
            a[2] = 3 as $t;
            a[3] = 4 as $t;
            assert_eq!(1 as $t, a[0]);
            assert_eq!(2 as $t, a[1]);
            assert_eq!(3 as $t, a[2]);
            assert_eq!(4 as $t, a[3]);
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), a.into());
        }

        #[test]
        fn test_dot_unsigned() {
            let x = $new(1 as $t, 0 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t, 0 as $t);
            let w = $new(0 as $t, 0 as $t, 0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(0 as $t, y.dot(z));
            assert_eq!(0 as $t, z.dot(w));
        }

        #[test]
        fn test_ops() {
            let a = $new(2 as $t, 4 as $t, 8 as $t, 16 as $t);
            assert_eq!((4 as $t, 8 as $t, 16 as $t, 32 as $t), (a + a).into());
            assert_eq!((0 as $t, 0 as $t, 0 as $t, 0 as $t), (a - a).into());
            assert_eq!((4 as $t, 16 as $t, 64 as $t, 256 as $t), (a * a).into());
            assert_eq!((4 as $t, 8 as $t, 16 as $t, 32 as $t), (a * 2 as $t).into());
            assert_eq!((4 as $t, 8 as $t, 16 as $t, 32 as $t), (2 as $t * a).into());
            assert_eq!((1 as $t, 1 as $t, 1 as $t, 1 as $t), (a / a).into());
            assert_eq!((1 as $t, 2 as $t, 4 as $t, 8 as $t), (a / 2 as $t).into());
            assert_eq!((8 as $t, 4 as $t, 2 as $t, 1 as $t), (16 as $t / a).into());
        }

        #[test]
        fn test_assign_ops() {
            let a = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let mut b = a;
            b += a;
            assert_eq!((2 as $t, 4 as $t, 6 as $t, 8 as $t), b.into());
            b -= a;
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), b.into());
            b *= a;
            assert_eq!((1 as $t, 4 as $t, 9 as $t, 16 as $t), b.into());
            b /= a;
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), b.into());
            b *= 2 as $t;
            assert_eq!((2 as $t, 4 as $t, 6 as $t, 8 as $t), b.into());
            b /= 2 as $t;
            assert_eq!((1 as $t, 2 as $t, 3 as $t, 4 as $t), b.into());
        }

        #[test]
        fn test_min_max() {
            let a = $new(4 as $t, 6 as $t, 2 as $t, 8 as $t);
            let b = $new(5 as $t, 3 as $t, 7 as $t, 1 as $t);
            assert_eq!((4 as $t, 3 as $t, 2 as $t, 1 as $t), a.min(b).into());
            assert_eq!((4 as $t, 3 as $t, 2 as $t, 1 as $t), b.min(a).into());
            assert_eq!((5 as $t, 6 as $t, 7 as $t, 8 as $t), a.max(b).into());
            assert_eq!((5 as $t, 6 as $t, 7 as $t, 8 as $t), b.max(a).into());
        }

        #[test]
        fn test_hmin_hmax() {
            let a = $new(3 as $t, 4 as $t, 1 as $t, 2 as $t);
            assert_eq!(1 as $t, a.min_element());
            assert_eq!(4 as $t, a.max_element());
            assert_eq!(
                3 as $t,
                $new(1 as $t, 2 as $t, 3 as $t, 4 as $t)
                    .truncate()
                    .max_element()
            );
            assert_eq!(
                2 as $t,
                $new(4 as $t, 3 as $t, 2 as $t, 1 as $t)
                    .truncate()
                    .min_element()
            );
        }

        #[test]
        fn test_eq() {
            let a = $new(1 as $t, 1 as $t, 1 as $t, 1 as $t);
            let b = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert!(a.cmpeq(a).all());
            assert!(b.cmpeq(b).all());
            assert!(a.cmpne(b).any());
            assert!(b.cmpne(a).any());
            assert!(b.cmpeq(a).any());
        }

        #[test]
        fn test_cmp() {
            assert!(!$mask::default().any());
            assert!(!$mask::default().all());
            assert_eq!($mask::default().bitmask(), 0x0);
            let a = $new(1 as $t, 1 as $t, 1 as $t, 1 as $t);
            let b = $new(2 as $t, 2 as $t, 2 as $t, 2 as $t);
            let c = $new(1 as $t, 1 as $t, 2 as $t, 2 as $t);
            let d = $new(2 as $t, 1 as $t, 1 as $t, 2 as $t);
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
            assert!(a == a);
            assert!(a < b);
            assert!(b > a);
        }

        #[test]
        fn test_slice() {
            let a = [1 as $t, 2 as $t, 3 as $t, 4 as $t];
            let b = $vec4::from_slice_unaligned(&a);
            let c: [$t; 4] = b.into();
            assert_eq!(a, c);
            let mut d = [0 as $t, 0 as $t, 0 as $t, 0 as $t];
            b.write_to_slice_unaligned(&mut d[..]);
            assert_eq!(a, d);
        }

        #[test]
        fn test_mask_as_ref() {
            assert_eq!(
                $mask::new(false, false, false, false).as_ref(),
                &[0, 0, 0, 0]
            );
            assert_eq!(
                $mask::new(false, false, true, true).as_ref(),
                &[0, 0, !0, !0]
            );
            assert_eq!(
                $mask::new(true, true, false, false).as_ref(),
                &[!0, !0, 0, 0]
            );
            assert_eq!(
                $mask::new(false, true, false, true).as_ref(),
                &[0, !0, 0, !0]
            );
            assert_eq!(
                $mask::new(true, false, true, false).as_ref(),
                &[!0, 0, !0, 0]
            );
            assert_eq!(
                $mask::new(true, true, true, true).as_ref(),
                &[!0, !0, !0, !0]
            );
        }

        #[test]
        fn test_mask_from() {
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(false, false, false, false)),
                [0, 0, 0, 0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(false, false, true, true)),
                [0, 0, !0, !0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(true, true, false, false)),
                [!0, !0, 0, 0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(false, true, false, true)),
                [0, !0, 0, !0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(true, false, true, false)),
                [!0, 0, !0, 0]
            );
            assert_eq!(
                Into::<[u32; 4]>::into($mask::new(true, true, true, true)),
                [!0, !0, !0, !0]
            );
        }

        #[test]
        fn test_mask_bitmask() {
            assert_eq!($mask::new(false, false, false, false).bitmask(), 0b0000);
            assert_eq!($mask::new(false, false, true, true).bitmask(), 0b1100);
            assert_eq!($mask::new(true, true, false, false).bitmask(), 0b0011);
            assert_eq!($mask::new(false, true, false, true).bitmask(), 0b1010);
            assert_eq!($mask::new(true, false, true, false).bitmask(), 0b0101);
            assert_eq!($mask::new(true, true, true, true).bitmask(), 0b1111);
        }

        #[test]
        fn test_mask_any() {
            assert_eq!($mask::new(false, false, false, false).any(), false);
            assert_eq!($mask::new(true, false, false, false).any(), true);
            assert_eq!($mask::new(false, true, false, false).any(), true);
            assert_eq!($mask::new(false, false, true, false).any(), true);
            assert_eq!($mask::new(false, false, false, true).any(), true);
        }

        #[test]
        fn test_mask_all() {
            assert_eq!($mask::new(true, true, true, true).all(), true);
            assert_eq!($mask::new(false, true, true, true).all(), false);
            assert_eq!($mask::new(true, false, true, true).all(), false);
            assert_eq!($mask::new(true, true, false, true).all(), false);
            assert_eq!($mask::new(true, true, true, false).all(), false);
        }

        #[test]
        fn test_mask_select() {
            let a = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let b = $vec4::new(5 as $t, 6 as $t, 7 as $t, 8 as $t);
            assert_eq!(
                $vec4::select($mask::new(true, true, true, true), a, b),
                $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t),
            );
            assert_eq!(
                $vec4::select($mask::new(true, false, true, false), a, b),
                $vec4::new(1 as $t, 6 as $t, 3 as $t, 8 as $t),
            );
            assert_eq!(
                $vec4::select($mask::new(false, true, false, true), a, b),
                $vec4::new(5 as $t, 2 as $t, 7 as $t, 4 as $t),
            );
            assert_eq!(
                $vec4::select($mask::new(false, false, false, false), a, b),
                $vec4::new(5 as $t, 6 as $t, 7 as $t, 8 as $t),
            );
        }

        #[test]
        fn test_mask_and() {
            assert_eq!(
                ($mask::new(false, false, false, false) & $mask::new(false, false, false, false))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, true, true, true) & $mask::new(true, true, true, true)).bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) & $mask::new(false, true, false, true))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, false, true, true) & $mask::new(true, true, true, false))
                    .bitmask(),
                0b0101,
            );

            let mut mask = $mask::new(true, true, false, false);
            mask &= $mask::new(true, false, true, false);
            assert_eq!(mask.bitmask(), 0b0001);
        }

        #[test]
        fn test_mask_or() {
            assert_eq!(
                ($mask::new(false, false, false, false) | $mask::new(false, false, false, false))
                    .bitmask(),
                0b0000,
            );
            assert_eq!(
                ($mask::new(true, true, true, true) | $mask::new(true, true, true, true)).bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) | $mask::new(false, true, false, true))
                    .bitmask(),
                0b1111,
            );
            assert_eq!(
                ($mask::new(true, false, true, false) | $mask::new(true, false, true, false))
                    .bitmask(),
                0b0101,
            );

            let mut mask = $mask::new(true, true, false, false);
            mask |= $mask::new(true, false, true, false);
            assert_eq!(mask.bitmask(), 0b0111);
        }

        #[test]
        fn test_mask_not() {
            assert_eq!((!$mask::new(false, false, false, false)).bitmask(), 0b1111);
            assert_eq!((!$mask::new(true, true, true, true)).bitmask(), 0b0000);
            assert_eq!((!$mask::new(true, false, true, false)).bitmask(), 0b1010);
            assert_eq!((!$mask::new(false, true, false, true)).bitmask(), 0b0101);
        }

        #[test]
        fn test_mask_fmt() {
            let a = $mask::new(true, false, true, false);

            assert_eq!(format!("{}", a), "[true, false, true, false]");
            assert_eq!(
                format!("{:?}", a),
                format!("{}(0xffffffff, 0x0, 0xffffffff, 0x0)", stringify!($mask))
            );
        }

        #[test]
        fn test_mask_eq() {
            let a = $mask::new(true, false, true, false);
            let b = $mask::new(true, false, true, false);
            let c = $mask::new(false, true, true, false);

            assert_eq!(a, b);
            assert_eq!(b, a);
            assert_ne!(a, c);
            assert_ne!(b, c);

            assert!(a > c);
            assert!(c < a);
        }

        #[test]
        fn test_mask_hash() {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $mask::new(true, false, true, false);
            let b = $mask::new(true, false, true, false);
            let c = $mask::new(false, true, true, false);

            let mut hasher = DefaultHasher::new();
            a.hash(&mut hasher);
            let a_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            let b_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            c.hash(&mut hasher);
            let c_hashed = hasher.finish();

            assert_eq!(a, b);
            assert_eq!(a_hashed, b_hashed);
            assert_ne!(a, c);
            assert_ne!(a_hashed, c_hashed);
        }

        #[test]
        fn test_to_from_slice() {
            let v = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let mut a = [0 as $t, 0 as $t, 0 as $t, 0 as $t];
            v.write_to_slice_unaligned(&mut a);
            assert_eq!(v, $vec4::from_slice_unaligned(&a));
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let one = $vec4::one();
            assert_eq!(vec![one, one].iter().sum::<$vec4>(), one + one);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $vec4::new(2 as $t, 2 as $t, 2 as $t, 2 as $t);
            assert_eq!(vec![two, two].iter().product::<$vec4>(), two * two);
        }
    };
}

macro_rules! impl_vec4_signed_tests {
    ($t:ident, $new:ident, $vec4:ident, $mask:ident) => {
        impl_vec4_tests!($t, $new, $vec4, $mask);

        #[test]
        fn test_neg() {
            let a = $new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            assert_eq!((-1 as $t, -2 as $t, -3 as $t, -4 as $t), (-a).into());
        }

        #[test]
        fn test_dot_signed() {
            let x = $new(1 as $t, 0 as $t, 0 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t, 0 as $t, 0 as $t);
            let z = $new(0 as $t, 0 as $t, 1 as $t, 0 as $t);
            let w = $new(0 as $t, 0 as $t, 0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(0 as $t, x.dot(-z));
            assert_eq!(-1 as $t, w.dot(-w));
        }
    };
}

macro_rules! impl_vec4_float_tests {
    ($t:ident, $new:ident, $vec4:ident, $mask:ident) => {
        impl_vec4_signed_tests!($t, $new, $vec4, $mask);

        use core::$t::INFINITY;
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_funcs() {
            let x = $new(1.0, 0.0, 0.0, 0.0);
            let y = $new(0.0, 1.0, 0.0, 0.0);
            let z = $new(0.0, 0.0, 1.0, 0.0);
            let w = $new(0.0, 0.0, 0.0, 1.0);
            assert_eq!(4.0, (2.0 * x).length_squared());
            assert_eq!(9.0, (-3.0 * y).length_squared());
            assert_eq!(16.0, (4.0 * z).length_squared());
            assert_eq!(64.0, (8.0 * w).length_squared());
            assert_eq!(2.0, (-2.0 * x).length());
            assert_eq!(3.0, (3.0 * y).length());
            assert_eq!(4.0, (-4.0 * z).length());
            assert_eq!(5.0, (-5.0 * w).length());
            assert_eq!(2.0, x.distance_squared(y));
            assert_eq!(13.0, (2.0 * x).distance_squared(-3.0 * z));
            assert_eq!((2.0 as $t).sqrt(), w.distance(y));
            assert_eq!(5.0, (3.0 * x).distance(-4.0 * y));
            assert_eq!(13.0, (-5.0 * w).distance(12.0 * y));
            assert_eq!(x, (2.0 * x).normalize());
            assert_eq!(
                1.0 * 5.0 + 2.0 * 6.0 + 3.0 * 7.0 + 4.0 * 8.0,
                $new(1.0, 2.0, 3.0, 4.0).dot($new(5.0, 6.0, 7.0, 8.0))
            );
            assert_eq!(
                2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0,
                $new(2.0, 3.0, 4.0, 5.0).length_squared()
            );
            assert_eq!(
                (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
                $new(2.0, 3.0, 4.0, 5.0).length()
            );
            assert_eq!(
                1.0 / (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
                $new(2.0, 3.0, 4.0, 5.0).length_recip()
            );
            assert!($new(2.0, 3.0, 4.0, 5.0).normalize().is_normalized());
            assert_approx_eq!(
                $new(2.0, 3.0, 4.0, 5.0)
                    / (2.0 as $t * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt(),
                $new(2.0, 3.0, 4.0, 5.0).normalize()
            );
            assert_eq!(
                $new(0.5, 0.25, 0.125, 0.0625),
                $new(2.0, 4.0, 8.0, 16.0).recip()
            );
        }

        #[test]
        fn test_signum() {
            assert_eq!($vec4::zero().signum(), $vec4::one());
            assert_eq!(-$vec4::zero().signum(), -$vec4::one());
            assert_eq!($vec4::one().signum(), $vec4::one());
            assert_eq!((-$vec4::one()).signum(), -$vec4::one());
            assert_eq!($vec4::splat(INFINITY).signum(), $vec4::one());
            assert_eq!($vec4::splat(NEG_INFINITY).signum(), -$vec4::one());
            assert!($vec4::splat(NAN).signum().is_nan_mask().all());
        }

        #[test]
        fn test_abs() {
            assert_eq!($vec4::zero().abs(), $vec4::zero());
            assert_eq!($vec4::one().abs(), $vec4::one());
            assert_eq!((-$vec4::one()).abs(), $vec4::one());
        }

        #[test]
        fn test_round() {
            assert_eq!($vec4::new(1.35, 0.0, 0.0, 0.0).round().x, 1.0);
            assert_eq!($vec4::new(0.0, 1.5, 0.0, 0.0).round().y, 2.0);
            assert_eq!($vec4::new(0.0, 0.0, -15.5, 0.0).round().z, -16.0);
            assert_eq!($vec4::new(0.0, 0.0, 0.0, 0.0).round().z, 0.0);
            assert_eq!($vec4::new(0.0, 21.1, 0.0, 0.0).round().y, 21.0);
            assert_eq!($vec4::new(0.0, 0.0, 0.0, 11.123).round().w, 11.0);
            assert_eq!($vec4::new(0.0, 0.0, 11.501, 0.0).round().z, 12.0);
            assert_eq!(
                $vec4::new(NEG_INFINITY, INFINITY, 1.0, -1.0).round(),
                $vec4::new(NEG_INFINITY, INFINITY, 1.0, -1.0)
            );
            assert!($vec4::new(NAN, 0.0, 0.0, 1.0).round().x.is_nan());
        }

        #[test]
        fn test_floor() {
            assert_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1.999).floor(),
                $vec4::new(1.0, 1.0, -2.0, 1.0)
            );
            assert_eq!(
                $vec4::new(INFINITY, NEG_INFINITY, 0.0, 0.0).floor(),
                $vec4::new(INFINITY, NEG_INFINITY, 0.0, 0.0)
            );
            assert!($vec4::new(0.0, NAN, 0.0, 0.0).floor().y.is_nan());
            assert_eq!(
                $vec4::new(-0.0, -2000000.123, 10000000.123, 1000.9).floor(),
                $vec4::new(-0.0, -2000001.0, 10000000.0, 1000.0)
            );
        }

        #[test]
        fn test_ceil() {
            assert_eq!(
                $vec4::new(1.35, 1.5, -1.5, 1234.1234).ceil(),
                $vec4::new(2.0, 2.0, -1.0, 1235.0)
            );
            assert_eq!(
                $vec4::new(INFINITY, NEG_INFINITY, 0.0, 0.0).ceil(),
                $vec4::new(INFINITY, NEG_INFINITY, 0.0, 0.0)
            );
            assert!($vec4::new(0.0, 0.0, NAN, 0.0).ceil().z.is_nan());
            assert_eq!(
                $vec4::new(-1234.1234, -2000000.123, 1000000.123, 1000.9).ceil(),
                $vec4::new(-1234.0, -2000000.0, 1000001.0, 1001.0)
            );
        }

        #[test]
        fn test_lerp() {
            let v0 = $vec4::new(-1.0, -1.0, -1.0, -1.0);
            let v1 = $vec4::new(1.0, 1.0, 1.0, 1.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
            assert_approx_eq!($vec4::zero(), v0.lerp(v1, 0.5));
        }

        #[test]
        fn test_is_finite() {
            assert!($vec4::new(0.0, 0.0, 0.0, 0.0).is_finite());
            assert!($vec4::new(-1e-10, 1.0, 1e10, 42.0).is_finite());
            assert!(!$vec4::new(INFINITY, 0.0, 0.0, 0.0).is_finite());
            assert!(!$vec4::new(0.0, NAN, 0.0, 0.0).is_finite());
            assert!(!$vec4::new(0.0, 0.0, NEG_INFINITY, 0.0).is_finite());
            assert!(!$vec4::new(0.0, 0.0, 0.0, NAN).is_finite());
        }

        #[test]
        fn test_powf() {
            assert_eq!(
                $vec4::new(2.0, 4.0, 8.0, 16.0).powf(2.0),
                $vec4::new(4.0, 16.0, 64.0, 256.0)
            );
        }

        #[test]
        fn test_exp() {
            assert_eq!(
                $vec4::new(1.0, 2.0, 3.0, 4.0).exp(),
                $vec4::new(
                    (1.0 as $t).exp(),
                    (2.0 as $t).exp(),
                    (3.0 as $t).exp(),
                    (4.0 as $t).exp()
                )
            );
        }
    };
}

mod vec4 {
    use glam::{vec4, Vec4, Vec4Mask};

    #[test]
    fn test_align() {
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

    #[cfg(vec4_sse2)]
    #[test]
    fn test_m128() {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        #[repr(C, align(16))]
        struct F32x4_A16([f32; 4]);

        let v0 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let m0: __m128 = v0.into();
        let mut a0 = F32x4_A16([0.0, 0.0, 0.0, 0.0]);
        unsafe {
            _mm_store_ps(a0.0.as_mut_ptr(), m0);
        }
        assert_eq!([1.0, 2.0, 3.0, 4.0], a0.0);
        let v1 = Vec4::from(m0);
        assert_eq!(v0, v1);

        #[repr(C, align(16))]
        struct U32x4_A16([u32; 4]);

        let v0 = Vec4Mask::new(true, false, true, false);
        let m0: __m128 = v0.into();
        let mut a0 = U32x4_A16([1, 2, 3, 4]);
        unsafe {
            _mm_store_ps(a0.0.as_mut_ptr() as *mut f32, m0);
        }
        assert_eq!([0xffffffff, 0, 0xffffffff, 0], a0.0);
    }

    impl_vec4_float_tests!(f32, vec4, Vec4, Vec4Mask);
}

mod dvec4 {
    use glam::{dvec4, DVec4, UVec4Mask};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(32, mem::size_of::<DVec4>());
        assert_eq!(16, mem::size_of::<UVec4Mask>());
        assert_eq!(8, mem::align_of::<DVec4>());
        assert_eq!(4, mem::align_of::<UVec4Mask>());
    }

    impl_vec4_float_tests!(f64, dvec4, DVec4, UVec4Mask);
}

mod ivec4 {
    use glam::{ivec4, IVec4, UVec4Mask};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(16, mem::size_of::<IVec4>());
        assert_eq!(16, mem::size_of::<UVec4Mask>());
        assert_eq!(4, mem::align_of::<IVec4>());
        assert_eq!(4, mem::align_of::<UVec4Mask>());
    }

    impl_vec4_signed_tests!(i32, ivec4, IVec4, UVec4Mask);
}

mod uvec4 {
    use glam::{uvec4, UVec4, UVec4Mask};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(16, mem::size_of::<UVec4>());
        assert_eq!(16, mem::size_of::<UVec4Mask>());
        assert_eq!(4, mem::align_of::<UVec4>());
        assert_eq!(4, mem::align_of::<UVec4Mask>());
    }

    impl_vec4_tests!(u32, uvec4, UVec4, UVec4Mask);
}
