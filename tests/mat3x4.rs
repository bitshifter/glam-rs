#[macro_use]
mod support;

macro_rules! impl_mat3x4_tests {
    ($t:ident, $mat3x4:ident, $quat:ident, $vec4:ident, $vec3:ident) => {
        use core::$t::INFINITY;
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_mat3x4_identity() {
            assert_eq!(
                $mat3x4::IDENTITY,
                $mat3x4::from_rows_array(&[
                    1., 0., 0., 0., //
                    0., 1., 0., 0., //
                    0., 0., 1., 0., //
                ])
            );
            assert_eq!($mat3x4::IDENTITY, $mat3x4::IDENTITY * $mat3x4::IDENTITY);
            assert_eq!($mat3x4::IDENTITY, $mat3x4::default());
        }

        #[test]
        fn test_mat3x4_zero() {
            assert_eq!(
                $mat3x4::ZERO,
                $mat3x4::from_rows_array(&[
                    0., 0., 0., 0., //
                    0., 0., 0., 0., //
                    0., 0., 0., 0., //
                ])
            );
        }

        #[test]
        fn test_mat3x4_accessors() {
            let m = $mat3x4::from_rows_array_2d(&[
                [0.0, 1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0, 7.0],
                [8.0, 9.0, 10., 11.],
            ]);
            assert_eq!(m.row(0), $vec4::new(0.0, 1.0, 2.0, 3.0));
            assert_eq!(m.row(1), $vec4::new(4.0, 5.0, 6.0, 7.0));
            assert_eq!(m.row(2), $vec4::new(8.0, 9.0, 10., 11.));

            assert_eq!(m.col(0), $vec3::new(0.0, 4.0, 8.0));
            assert_eq!(m.col(1), $vec3::new(1.0, 5.0, 9.0));
            assert_eq!(m.col(2), $vec3::new(2.0, 6.0, 10.));
            assert_eq!(m.col(3), $vec3::new(3.0, 7.0, 11.));
        }

        #[test]
        fn test_mat3x4_translation() {
            let translate = $mat3x4::from_translation($vec3::new(1.0, 2.0, 3.0));
            assert_eq!(
                $mat3x4::from_cols(
                    $vec3::new(1.0, 0.0, 0.0),
                    $vec3::new(0.0, 1.0, 0.0),
                    $vec3::new(0.0, 0.0, 1.0),
                    $vec3::new(1.0, 2.0, 3.0)
                ),
                translate
            );
        }

        #[test]
        fn test_from_rotation() {
            let eps = 2.0 * core::f32::EPSILON;
            let rot_x1 = $mat3x4::from_rotation_x(deg(180.0));
            let rot_x2 = $mat3x4::from_axis_angle($vec3::X, deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2, eps);
            let rot_y1 = $mat3x4::from_rotation_y(deg(180.0));
            let rot_y2 = $mat3x4::from_axis_angle($vec3::Y, deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2, eps);
            let rot_z1 = $mat3x4::from_rotation_z(deg(180.0));
            let rot_z2 = $mat3x4::from_axis_angle($vec3::Z, deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2, eps);
        }

        #[test]
        fn test_mat3x4_mul() {
            let m = $mat3x4::from_axis_angle($vec3::Z, deg(90.0));
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(-1.0, 0.0, 0.0), result3);
            assert_approx_eq!(result3, m * $vec3::Y.extend(0.0));
            let result4 = m * $vec4::Y;
            assert_approx_eq!($vec3::new(-1.0, 0.0, 0.0), result4);

            let m = $mat3x4::from_scale_rotation_translation(
                $vec3::new(0.5, 1.5, 2.0),
                $quat::from_rotation_x(deg(90.0)),
                $vec3::new(1.0, 2.0, 3.0),
            );
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(0.0, 0.0, 1.5), result3, 1.0e-6);
            assert_approx_eq!(result3, m * $vec3::Y.extend(0.0));

            let result3 = m.transform_point3($vec3::Y);
            assert_approx_eq!($vec3::new(1.0, 2.0, 4.5), result3, 1.0e-6);
            assert_approx_eq!(result3, m * $vec3::Y.extend(1.0));
        }

        #[test]
        fn test_from_scale() {
            let m = $mat3x4::from_scale($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!($vec3::X * 2.0, m.x_col());
            assert_approx_eq!($vec3::Y * 4.0, m.y_col());
            assert_approx_eq!($vec3::Z * 8.0, m.z_col());
            assert_approx_eq!($vec3::ZERO, m.w_col());
            assert_approx_eq!(
                m.transform_point3($vec3::new(1.0, 1.0, 1.0)),
                $vec3::new(2.0, 4.0, 8.0)
            );
        }

        #[test]
        fn test_mat3x4_det() {
            assert_eq!(0.0, $mat3x4::ZERO.determinant());
            assert_eq!(1.0, $mat3x4::IDENTITY.determinant());
            assert_eq!(1.0, $mat3x4::from_rotation_x(deg(90.0)).determinant());
            assert_eq!(1.0, $mat3x4::from_rotation_y(deg(180.0)).determinant());
            assert_eq!(1.0, $mat3x4::from_rotation_z(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0 * 2.0,
                $mat3x4::from_scale($vec3::new(2.0, 2.0, 2.0)).determinant()
            );
        }

        #[test]
        fn test_mat3x4_inverse() {
            let inv = $mat3x4::IDENTITY.inverse();
            assert_approx_eq!($mat3x4::IDENTITY, inv);

            let rotz = $mat3x4::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            assert_approx_eq!($mat3x4::IDENTITY, rotz * rotz_inv);
            assert_approx_eq!($mat3x4::IDENTITY, rotz_inv * rotz);

            let trans = $mat3x4::from_translation($vec3::new(1.0, 2.0, 3.0));
            let trans_inv = trans.inverse();
            assert_approx_eq!($mat3x4::IDENTITY, trans * trans_inv);
            assert_approx_eq!($mat3x4::IDENTITY, trans_inv * trans);

            let scale = $mat3x4::from_scale($vec3::new(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($mat3x4::IDENTITY, scale * scale_inv);
            assert_approx_eq!($mat3x4::IDENTITY, scale_inv * scale);

            let m = scale * rotz * trans;
            let m_inv = m.inverse();
            assert_approx_eq!($mat3x4::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($mat3x4::IDENTITY, m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rotz_inv * scale_inv, 1.0e-6);
        }

        #[test]
        fn test_mat3x4_decompose() {
            // identity
            let (out_scale, out_rotation, out_translation) =
                $mat3x4::IDENTITY.to_scale_rotation_translation();
            assert_approx_eq!($vec3::ONE, out_scale);
            assert!(out_rotation.is_near_identity());
            assert_approx_eq!($vec3::ZERO, out_translation);

            // no scale
            let in_scale = $vec3::ONE;
            let in_translation = $vec3::new(-2.0, 4.0, -0.125);
            let in_rotation = $quat::from_rotation_ypr(
                $t::to_radians(-45.0),
                $t::to_radians(180.0),
                $t::to_radians(270.0),
            );
            let in_mat =
                $mat3x4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat3x4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            // positive scale
            let in_scale = $vec3::new(1.0, 2.0, 4.0);
            let in_mat =
                $mat3x4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat3x4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            // negative scale
            let in_scale = $vec3::new(-4.0, 1.0, 2.0);
            let in_mat =
                $mat3x4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat3x4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(4.0, -1.0, -2.0);
            let in_mat =
                $mat3x4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            // out_scale and out_rotation are different but they produce the same matrix
            // assert_approx_eq!(in_scale, out_scale, 1e-6);
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat3x4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );
        }

        #[test]
        fn test_mat3x4_look_at() {
            let eye = $vec3::new(0.0, 0.0, -5.0);
            let center = $vec3::new(0.0, 0.0, 0.0);
            let up = $vec3::new(1.0, 0.0, 0.0);
            let lh = $mat3x4::look_at_lh(eye, center, up);
            let rh = $mat3x4::look_at_rh(eye, center, up);
            let point = $vec3::new(1.0, 0.0, 0.0);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let id = $mat3x4::IDENTITY;
            assert_eq!(vec![id, id].iter().sum::<$mat3x4>(), id + id);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $mat3x4::IDENTITY + $mat3x4::IDENTITY;
            assert_eq!(vec![two, two].iter().product::<$mat3x4>(), two * two);
        }

        #[test]
        fn test_mat3x4_is_finite() {
            assert!($mat3x4::IDENTITY.is_finite());
            assert!(!($mat3x4::IDENTITY * INFINITY).is_finite());
            assert!(!($mat3x4::IDENTITY * NEG_INFINITY).is_finite());
            assert!(!($mat3x4::IDENTITY * NAN).is_finite());
        }
    };
}

mod mat3x4 {
    use super::support::deg;
    use glam::{Mat3x4, Quat, Vec3, Vec4};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(48, mem::size_of::<Mat3x4>());
        if cfg!(feature = "scalar-math") {
            assert_eq!(4, mem::align_of::<Mat3x4>());
        } else {
            assert_eq!(16, mem::align_of::<Mat3x4>());
        }
    }

    impl_mat3x4_tests!(f32, Mat3x4, Quat, Vec4, Vec3);
}
