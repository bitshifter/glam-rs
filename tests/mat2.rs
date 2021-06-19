#[macro_use]
mod support;

macro_rules! impl_mat2_tests {
    ($t:ident, $const_new:ident, $newmat2:ident, $mat2:ident, $mat3:ident, $newvec2:ident, $vec2:ident) => {
        const IDENTITY: [[$t; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];

        const MATRIX: [[$t; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];

        #[test]
        fn test_const() {
            const M0: $mat2 = $const_new!([0.0; 4]);
            const M1: $mat2 = $const_new!([1.0, 2.0, 3.0, 4.0]);
            const M2: $mat2 = $const_new!([1.0, 2.0], [3.0, 4.0]);
            assert_eq!($mat2::ZERO, M0);
            assert_eq!($mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]), M1);
            assert_eq!($mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]), M2);
        }

        #[test]
        fn test_mat2_identity() {
            assert_eq!($mat2::IDENTITY, $mat2::from_cols_array(&[1., 0., 0., 1.]));
            let identity = $mat2::IDENTITY;
            assert_eq!(IDENTITY, identity.to_cols_array_2d());
            assert_eq!($mat2::from_cols_array_2d(&IDENTITY), identity);
            assert_eq!(identity, identity * identity);
            assert_eq!(identity, $mat2::default());
        }

        #[test]
        fn test_mat2_zero() {
            assert_eq!($mat2::ZERO, $mat2::from_cols_array(&[0., 0., 0., 0.]));
        }

        #[test]
        fn test_mat2_accessors() {
            let mut m = $mat2::ZERO;
            m.x_axis = $vec2::new(1.0, 2.0);
            m.y_axis = $vec2::new(3.0, 4.0);
            assert_eq!($mat2::from_cols_array_2d(&MATRIX), m);
            assert_eq!($vec2::new(1.0, 2.0), m.x_axis);
            assert_eq!($vec2::new(3.0, 4.0), m.y_axis);

            assert_eq!($vec2::new(1.0, 2.0), m.col(0));
            assert_eq!($vec2::new(3.0, 4.0), m.col(1));

            assert_eq!($newvec2(1.0, 3.0), m.row(0));
            assert_eq!($newvec2(2.0, 4.0), m.row(1));

            *m.col_mut(0) = m.col(0).yx();
            *m.col_mut(1) = m.col(1).yx();
            assert_eq!($vec2::new(2.0, 1.0), m.col(0));
            assert_eq!($vec2::new(4.0, 3.0), m.col(1));

            should_panic!({ $mat2::ZERO.col(2) });
            should_panic!({
                let mut m = $mat2::ZERO;
                m.col_mut(2);
            });
            should_panic!({ $mat2::ZERO.row(2) });
        }

        #[test]
        fn test_mat2_from_axes() {
            let a = $mat2::from_cols_array_2d(&[[1.0, 2.0], [3.0, 4.0]]);
            assert_eq!(MATRIX, a.to_cols_array_2d());
            let b = $mat2::from_cols($newvec2(1.0, 2.0), $newvec2(3.0, 4.0));
            assert_eq!(a, b);
            let c = $newmat2($newvec2(1.0, 2.0), $newvec2(3.0, 4.0));
            assert_eq!(a, c);
            let d = b.to_cols_array();
            let f = $mat2::from_cols_array(&d);
            assert_eq!(b, f);
        }

        #[test]
        fn test_mat2_mul() {
            let mat_a = $mat2::from_angle(deg(90.0));
            let res_a = mat_a * $vec2::Y;
            assert_approx_eq!($newvec2(-1.0, 0.0), res_a);
            let res_b = mat_a * $vec2::X;
            assert_approx_eq!($newvec2(0.0, 1.0), res_b);
        }

        #[test]
        fn test_from_scale_angle() {
            let rot = $mat2::from_scale_angle($vec2::new(4.0, 2.0), deg(180.0));
            assert_approx_eq!($vec2::X * -4.0, rot * $vec2::X, 1.0e-6);
            assert_approx_eq!($vec2::Y * -2.0, rot * $vec2::Y, 1.0e-6);
        }

        #[test]
        fn test_from_diagonal() {
            let m = $mat2::from_diagonal($vec2::new(2 as $t, 4 as $t));
            assert_eq!(
                $mat2::from_cols_array_2d(&[[2 as $t, 0 as $t], [0 as $t, 4 as $t]]),
                m
            );
            assert_approx_eq!(m * $vec2::new(1.0, 1.0), $vec2::new(2.0, 4.0));
            assert_approx_eq!($vec2::X * 2.0, m.x_axis);
            assert_approx_eq!($vec2::Y * 4.0, m.y_axis);
        }

        #[test]
        fn test_from_mat3() {
            let m3 =
                $mat3::from_cols_array_2d(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
            let m2 = $mat2::from(m3);
            assert_eq!($mat2::from_cols_array_2d(&[[1.0, 2.0], [4.0, 5.0]]), m2);
        }

        #[test]
        fn test_mat2_transpose() {
            let m = $newmat2($newvec2(1.0, 2.0), $newvec2(3.0, 4.0));
            let mt = m.transpose();
            assert_eq!($newvec2(1.0, 3.0), mt.x_axis);
            assert_eq!($newvec2(2.0, 4.0), mt.y_axis);
        }

        #[test]
        fn test_mat2_det() {
            assert_eq!(0.0, $mat2::ZERO.determinant());
            assert_eq!(1.0, $mat2::IDENTITY.determinant());
            assert_eq!(1.0, $mat2::from_angle(deg(90.0)).determinant());
            assert_eq!(1.0, $mat2::from_angle(deg(180.0)).determinant());
            assert_eq!(1.0, $mat2::from_angle(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0,
                $mat2::from_diagonal($newvec2(2.0, 2.0)).determinant()
            );
            assert_eq!(
                1.0 * 4.0 - 2.0 * 3.0,
                $mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]).determinant()
            );
        }

        #[test]
        fn test_mat2_inverse() {
            let inv = $mat2::IDENTITY.inverse();
            assert_approx_eq!($mat2::IDENTITY, inv);

            let rot = $mat2::from_angle(deg(90.0));
            let rot_inv = rot.inverse();
            assert_approx_eq!($mat2::IDENTITY, rot * rot_inv);
            assert_approx_eq!($mat2::IDENTITY, rot_inv * rot);

            let scale = $mat2::from_diagonal($newvec2(4.0, 5.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($mat2::IDENTITY, scale * scale_inv);
            assert_approx_eq!($mat2::IDENTITY, scale_inv * scale);

            let m = scale * rot;
            let m_inv = m.inverse();
            assert_approx_eq!($mat2::IDENTITY, m * m_inv);
            assert_approx_eq!($mat2::IDENTITY, m_inv * m);
            assert_approx_eq!(m_inv, rot_inv * scale_inv);

            should_glam_assert!({ $mat2::ZERO.inverse() });
        }

        #[test]
        fn test_mat2_ops() {
            let m0 = $mat2::from_cols_array_2d(&MATRIX);
            let m0x2 = $mat2::from_cols_array_2d(&[[2.0, 4.0], [6.0, 8.0]]);
            assert_eq!(m0x2, m0 * 2.0);
            assert_eq!(m0x2, 2.0 * m0);
            assert_eq!(m0x2, m0 + m0);
            assert_eq!($mat2::ZERO, m0 - m0);
            assert_approx_eq!(m0, m0 * $mat2::IDENTITY);
            assert_approx_eq!(m0, $mat2::IDENTITY * m0);

            let mut m1 = m0;
            m1 *= 2.0;
            assert_eq!(m0x2, m1);

            let mut m1 = m0;
            m1 += m0;
            assert_eq!(m0x2, m1);

            let mut m1 = m0;
            m1 -= m0;
            assert_eq!($mat2::ZERO, m1);

            let mut m1 = $mat2::IDENTITY;
            m1 *= m0;
            assert_approx_eq!(m0, m1);
        }

        #[test]
        fn test_mat2_fmt() {
            let a = $mat2::from_cols_array_2d(&MATRIX);
            assert_eq!(format!("{}", a), "[[1, 2], [3, 4]]");
        }

        #[test]
        fn test_mat2_to_from_slice() {
            const MATRIX1D: [$t; 4] = [1.0, 2.0, 3.0, 4.0];
            let m = $mat2::from_cols_slice(&MATRIX1D);
            assert_eq!($mat2::from_cols_array(&MATRIX1D), m);
            let mut out: [$t; 4] = Default::default();
            m.write_cols_to_slice(&mut out);
            assert_eq!(MATRIX1D, out);

            should_panic!({ $mat2::from_cols_slice(&[0.0]) });
            should_panic!({ $mat2::IDENTITY.write_cols_to_slice(&mut [0.0]) });
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let id = $mat2::IDENTITY;
            assert_eq!(vec![id, id].iter().sum::<$mat2>(), id + id);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $mat2::IDENTITY + $mat2::IDENTITY;
            assert_eq!(vec![two, two].iter().product::<$mat2>(), two * two);
        }

        #[test]
        fn test_mat2_is_finite() {
            use std::$t::INFINITY;
            use std::$t::NAN;
            use std::$t::NEG_INFINITY;
            assert!($mat2::IDENTITY.is_finite());
            assert!(!($mat2::IDENTITY * INFINITY).is_finite());
            assert!(!($mat2::IDENTITY * NEG_INFINITY).is_finite());
            assert!(!($mat2::IDENTITY * NAN).is_finite());
        }
    };
}

mod mat2 {
    use super::support::deg;
    use glam::{const_mat2, mat2, swizzles::*, vec2, Mat2, Mat3, Vec2};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(16, mem::size_of::<Mat2>());
        if cfg!(feature = "scalar-math") {
            assert_eq!(4, mem::align_of::<Mat2>());
        } else {
            assert_eq!(16, mem::align_of::<Mat2>());
        }
    }

    #[test]
    fn test_as() {
        use glam::DMat2;
        assert_eq!(
            DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]),
            Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]).as_f64()
        );
        assert_eq!(
            Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]),
            DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]).as_f32()
        );
    }

    impl_mat2_tests!(f32, const_mat2, mat2, Mat2, Mat3, vec2, Vec2);
}

mod dmat2 {
    use super::support::deg;
    use glam::{const_dmat2, dmat2, dvec2, swizzles::*, DMat2, DMat3, DVec2};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(32, mem::size_of::<DMat2>());
        assert_eq!(8, mem::align_of::<DMat2>());
    }

    impl_mat2_tests!(f64, const_dmat2, dmat2, DMat2, DMat3, dvec2, DVec2);
}
