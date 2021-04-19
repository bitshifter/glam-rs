#[macro_use]
mod support;

macro_rules! impl_affine3d_tests {
    ($t:ident, $affine3d:ident, $quat:ident, $vec3:ident) => {
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_affine3d_identity() {
            assert_eq!(
                $affine3d::IDENTITY,
                $affine3d::IDENTITY * $affine3d::IDENTITY
            );
            assert_eq!($affine3d::IDENTITY, $affine3d::default());
        }

        #[test]
        fn test_affine3d_zero() {
            assert_eq!(
                $affine3d::ZERO.transform_point3($vec3::new(1., 2., 3.)),
                $vec3::ZERO
            );
        }

        #[test]
        fn test_affine3d_translation() {
            let translate = $affine3d::from_translation($vec3::new(1.0, 2.0, 3.0));
            assert_eq!(translate.translation, $vec3::new(1.0, 2.0, 3.0).into());
            assert_eq!(
                translate.transform_point3($vec3::new(2.0, 3.0, 4.0)),
                $vec3::new(3.0, 5.0, 7.0),
            );
        }

        #[test]
        fn test_from_rotation() {
            let eps = 2.0 * core::f32::EPSILON;
            let rot_x1 = $affine3d::from_rotation_x(deg(180.0));
            let rot_x2 = $affine3d::from_axis_angle($vec3::X, deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2, eps);
            let rot_y1 = $affine3d::from_rotation_y(deg(180.0));
            let rot_y2 = $affine3d::from_axis_angle($vec3::Y, deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2, eps);
            let rot_z1 = $affine3d::from_rotation_z(deg(180.0));
            let rot_z2 = $affine3d::from_axis_angle($vec3::Z, deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2, eps);
        }

        #[test]
        fn test_affine3d_mul() {
            let m = $affine3d::from_axis_angle($vec3::Z, deg(90.0));
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(-1.0, 0.0, 0.0), result3);

            let m = $affine3d::from_scale_rotation_translation(
                $vec3::new(0.5, 1.5, 2.0),
                $quat::from_rotation_x(deg(90.0)),
                $vec3::new(1.0, 2.0, 3.0),
            );
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(0.0, 0.0, 1.5), result3, 1.0e-6);

            let result3 = m.transform_point3($vec3::Y);
            assert_approx_eq!($vec3::new(1.0, 2.0, 4.5), result3, 1.0e-6);
        }

        #[test]
        fn test_from_scale() {
            let m = $affine3d::from_scale($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!(
                m.transform_point3($vec3::new(1.0, 1.0, 1.0)),
                $vec3::new(2.0, 4.0, 8.0)
            );
        }

        #[test]
        fn test_affine3d_inverse() {
            let inv = $affine3d::IDENTITY.inverse();
            assert_approx_eq!($affine3d::IDENTITY, inv);

            let rotz = $affine3d::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            assert_approx_eq!($affine3d::IDENTITY, rotz * rotz_inv);
            assert_approx_eq!($affine3d::IDENTITY, rotz_inv * rotz);

            let trans = $affine3d::from_translation($vec3::new(1.0, 2.0, 3.0));
            let trans_inv = trans.inverse();
            assert_approx_eq!($affine3d::IDENTITY, trans * trans_inv);
            assert_approx_eq!($affine3d::IDENTITY, trans_inv * trans);

            let scale = $affine3d::from_scale($vec3::new(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($affine3d::IDENTITY, scale * scale_inv);
            assert_approx_eq!($affine3d::IDENTITY, scale_inv * scale);

            let m = scale * rotz * trans;
            let m_inv = m.inverse();
            assert_approx_eq!($affine3d::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine3d::IDENTITY, m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rotz_inv * scale_inv, 1.0e-6);

            // Make sure we can invert a shear matrix:
            let m = $affine3d::from_axis_angle($vec3::X, 0.5)
                * $affine3d::from_scale($vec3::new(1.0, 0.5, 2.0))
                * $affine3d::from_axis_angle($vec3::X, -0.5);
            let m_inv = m.inverse();
            assert_approx_eq!($affine3d::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine3d::IDENTITY, m_inv * m, 1.0e-5);
        }

        #[test]
        fn test_affine3d_decompose() {
            // identity
            let (out_scale, out_rotation, out_translation) =
                $affine3d::IDENTITY.to_scale_rotation_translation();
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
                $affine3d::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3d::from_scale_rotation_translation(
                    out_scale,
                    out_rotation,
                    out_translation
                ),
                1e-6
            );

            // positive scale
            let in_scale = $vec3::new(1.0, 2.0, 4.0);
            let in_mat =
                $affine3d::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3d::from_scale_rotation_translation(
                    out_scale,
                    out_rotation,
                    out_translation
                ),
                1e-6
            );

            // negative scale
            let in_scale = $vec3::new(-4.0, 1.0, 2.0);
            let in_mat =
                $affine3d::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3d::from_scale_rotation_translation(
                    out_scale,
                    out_rotation,
                    out_translation
                ),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(4.0, -1.0, -2.0);
            let in_mat =
                $affine3d::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            // out_scale and out_rotation are different but they produce the same matrix
            // assert_approx_eq!(in_scale, out_scale, 1e-6);
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3d::from_scale_rotation_translation(
                    out_scale,
                    out_rotation,
                    out_translation
                ),
                1e-6
            );
        }

        #[test]
        fn test_affine3d_look_at() {
            let eye = $vec3::new(0.0, 0.0, -5.0);
            let center = $vec3::new(0.0, 0.0, 0.0);
            let up = $vec3::new(1.0, 0.0, 0.0);
            let lh = $affine3d::look_at_lh(eye, center, up);
            let rh = $affine3d::look_at_rh(eye, center, up);
            let point = $vec3::new(1.0, 0.0, 0.0);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let ident = $affine3d::IDENTITY;
            assert_eq!(
                vec![ident, ident].iter().product::<$affine3d>(),
                ident * ident
            );
        }

        #[test]
        fn test_affine3d_is_finite() {
            assert!($affine3d::from_scale($vec3::new(1.0, 1.0, 1.0)).is_finite());
            assert!($affine3d::from_scale($vec3::new(0.0, 1.0, 1.0)).is_finite());
            assert!(!$affine3d::from_scale($vec3::new(1.0, NAN, 1.0)).is_finite());
            assert!(!$affine3d::from_scale($vec3::new(1.0, 1.0, NEG_INFINITY)).is_finite());
        }
    };
}

mod affine3 {
    use super::support::deg;
    use glam::{Affine3, Quat, Vec3};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(64, mem::size_of::<Affine3>());
        assert_eq!(16, mem::align_of::<Affine3>());
    }

    impl_affine3d_tests!(f32, Affine3, Quat, Vec3);
}

mod daffine3 {
    use super::support::deg;
    use glam::{DAffine3, DQuat, DVec3};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(96, mem::size_of::<DAffine3>());
        assert_eq!(8, mem::align_of::<DAffine3>());
    }

    impl_affine3d_tests!(f64, DAffine3, DQuat, DVec3);
}
