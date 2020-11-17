#[macro_use]
mod support;

macro_rules! impl_mat2_tests {
    ($newmat2:ident, $mat2:ident, $newvec2:ident, $vec2:ident, $t:ident) => {
        const IDENTITY: [[$t; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];

        const MATRIX: [[$t; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];

        const ZERO: [[$t; 2]; 2] = [[0.0; 2]; 2];

        #[test]
        fn test_mat2_identity() {
            let identity = $mat2::identity();
            assert_eq!(IDENTITY, identity.to_cols_array_2d());
            assert_eq!($mat2::from_cols_array_2d(&IDENTITY), identity);
            assert_eq!(identity, identity * identity);
            assert_eq!(identity, $mat2::default());
        }

        #[test]
        fn test_mat2_zero() {
            assert_eq!($mat2::from_cols_array_2d(&ZERO), $mat2::zero());
        }

        #[test]
        fn test_mat2_accessors() {
            let mut m = $mat2::zero();
            m.x_axis = $vec2::new(1.0, 2.0);
            m.y_axis = $vec2::new(3.0, 4.0);
            assert_eq!($mat2::from_cols_array_2d(&MATRIX), m);
            assert_eq!($vec2::new(1.0, 2.0), m.x_axis);
            assert_eq!($vec2::new(3.0, 4.0), m.y_axis);
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
            let res_a = mat_a * $vec2::unit_y();
            assert_approx_eq!($newvec2(-1.0, 0.0), res_a);
            let res_b = mat_a * $vec2::unit_x();
            assert_approx_eq!($newvec2(0.0, 1.0), res_b);
        }

        #[test]
        fn test_from_scale() {
            let m = $mat2::from_scale($vec2::new(2.0, 4.0));
            assert_approx_eq!(m * $vec2::new(1.0, 1.0), $vec2::new(2.0, 4.0));
            assert_approx_eq!($vec2::unit_x() * 2.0, m.x_axis);
            assert_approx_eq!($vec2::unit_y() * 4.0, m.y_axis);

            let rot = $mat2::from_scale_angle($vec2::new(4.0, 2.0), deg(180.0));
            assert_approx_eq!($vec2::unit_x() * -4.0, rot * $vec2::unit_x(), 1.0e-6);
            assert_approx_eq!($vec2::unit_y() * -2.0, rot * $vec2::unit_y(), 1.0e-6);
        }

        #[test]
        fn test_mat2_transpose() {
            let m = $newmat2($newvec2(1.0, 2.0), $newvec2(3.0, 4.0));
            let mt = m.transpose();
            assert_eq!(mt.x_axis, $newvec2(1.0, 3.0));
            assert_eq!(mt.y_axis, $newvec2(2.0, 4.0));
        }

        #[test]
        fn test_mat2_det() {
            assert_eq!(0.0, $mat2::zero().determinant());
            assert_eq!(1.0, $mat2::identity().determinant());
            assert_eq!(1.0, $mat2::from_angle(deg(90.0)).determinant());
            assert_eq!(1.0, $mat2::from_angle(deg(180.0)).determinant());
            assert_eq!(1.0, $mat2::from_angle(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0,
                $mat2::from_scale($newvec2(2.0, 2.0)).determinant()
            );
            assert_eq!(
                1.0 * 4.0 - 2.0 * 3.0,
                $mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]).determinant()
            );
        }

        #[test]
        fn test_mat2_inverse() {
            let inv = $mat2::identity().inverse();
            assert_approx_eq!($mat2::identity(), inv);

            let rot = $mat2::from_angle(deg(90.0));
            let rot_inv = rot.inverse();
            assert_approx_eq!($mat2::identity(), rot * rot_inv);
            assert_approx_eq!($mat2::identity(), rot_inv * rot);

            let scale = $mat2::from_scale($newvec2(4.0, 5.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($mat2::identity(), scale * scale_inv);
            assert_approx_eq!($mat2::identity(), scale_inv * scale);

            let m = scale * rot;
            let m_inv = m.inverse();
            assert_approx_eq!($mat2::identity(), m * m_inv);
            assert_approx_eq!($mat2::identity(), m_inv * m);
            assert_approx_eq!(m_inv, rot_inv * scale_inv);
        }

        #[test]
        fn test_mat2_ops() {
            let m0 = $mat2::from_cols_array_2d(&MATRIX);
            assert_eq!(
                $mat2::from_cols_array_2d(&[[2.0, 4.0], [6.0, 8.0]]),
                m0 * 2.0
            );
            assert_eq!(
                $mat2::from_cols_array_2d(&[[2.0, 4.0], [6.0, 8.0]]),
                2.0 * m0
            );
            assert_eq!(
                $mat2::from_cols_array_2d(&[[2.0, 4.0], [6.0, 8.0]]),
                m0 + m0
            );
            assert_eq!($mat2::zero(), m0 - m0);
            assert_approx_eq!(
                $mat2::from_cols_array_2d(&[[1.0, 2.0], [3.0, 4.0]]),
                m0 * $mat2::identity()
            );
            assert_approx_eq!(
                $mat2::from_cols_array_2d(&[[1.0, 2.0], [3.0, 4.0]]),
                $mat2::identity() * m0
            );
        }

        #[test]
        fn test_mat2_fmt() {
            let a = $mat2::from_cols_array_2d(&MATRIX);
            assert_eq!(format!("{}", a), "[[1, 2], [3, 4]]");
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let id = $mat2::identity();
            assert_eq!(vec![id, id].iter().sum::<$mat2>(), id + id);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $mat2::identity() + $mat2::identity();
            assert_eq!(vec![two, two].iter().product::<$mat2>(), two * two);
        }

        #[test]
        fn test_mat2_is_finite() {
            use std::$t::INFINITY;
            use std::$t::NAN;
            use std::$t::NEG_INFINITY;
            assert!($mat2::identity().is_finite());
            assert!(!($mat2::identity() * INFINITY).is_finite());
            assert!(!($mat2::identity() * NEG_INFINITY).is_finite());
            assert!(!($mat2::identity() * NAN).is_finite());
        }
    };
}

mod mat2 {
    use super::support::deg;
    use glam::{mat2, vec2, Mat2, Vec2};

    #[test]
    fn test_mat2_align() {
        use std::mem;
        assert_eq!(16, mem::size_of::<Mat2>());
        if cfg!(feature = "scalar-math") {
            assert_eq!(4, mem::align_of::<Mat2>());
        } else {
            assert_eq!(16, mem::align_of::<Mat2>());
        }
    }

    impl_mat2_tests!(mat2, Mat2, vec2, Vec2, f32);
}

mod dmat2 {
    use super::support::deg;
    use glam::{dmat2, dvec2, DMat2, DVec2};

    #[test]
    fn test_mat2_align() {
        use std::mem;
        assert_eq!(32, mem::size_of::<DMat2>());
        assert_eq!(8, mem::align_of::<DMat2>());
    }

    impl_mat2_tests!(dmat2, DMat2, dvec2, DVec2, f64);
}
