#[macro_use]
mod support;

macro_rules! impl_mat3_tests {
    ($newmat3:ident, $mat3:ident, $newvec3:ident, $vec3:ident, $vec2:ident, $t:ident) => {
        const IDENTITY: [[$t; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        const MATRIX: [[$t; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

        const ZERO: [[$t; 3]; 3] = [[0.0; 3]; 3];

        #[test]
        fn test_mat3_identity() {
            let identity = $mat3::identity();
            assert_eq!(IDENTITY, identity.to_cols_array_2d());
            assert_eq!($mat3::from_cols_array_2d(&IDENTITY), identity);
            assert_eq!(identity, identity * identity);
            assert_eq!(identity, $mat3::default());
        }

        #[test]
        fn test_mat3_zero() {
            assert_eq!($mat3::from_cols_array_2d(&ZERO), $mat3::zero());
        }

        #[test]
        fn test_mat3_accessors() {
            let mut m = $mat3::zero();
            m.x_axis = $vec3::new(1.0, 2.0, 3.0);
            m.y_axis = $vec3::new(4.0, 5.0, 6.0);
            m.z_axis = $vec3::new(7.0, 8.0, 9.0);
            assert_eq!($mat3::from_cols_array_2d(&MATRIX), m);
            assert_eq!($vec3::new(1.0, 2.0, 3.0), m.x_axis);
            assert_eq!($vec3::new(4.0, 5.0, 6.0), m.y_axis);
            assert_eq!($vec3::new(7.0, 8.0, 9.0), m.z_axis);
        }

        #[test]
        fn test_mat3_from_axes() {
            let a = $mat3::from_cols_array_2d(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
            assert_eq!(MATRIX, a.to_cols_array_2d());
            let b = $mat3::from_cols(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            assert_eq!(a, b);
            let c = $newmat3(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            assert_eq!(a, c);
            let d = b.to_cols_array();
            let f = $mat3::from_cols_array(&d);
            assert_eq!(b, f);
        }

        #[test]
        fn test_from_rotation() {
            let rot_x1 = $mat3::from_rotation_x(deg(180.0));
            let rot_x2 = $mat3::from_axis_angle($vec3::unit_x(), deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2);
            let rot_y1 = $mat3::from_rotation_y(deg(180.0));
            let rot_y2 = $mat3::from_axis_angle($vec3::unit_y(), deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2);
            let rot_z1 = $mat3::from_rotation_z(deg(180.0));
            let rot_z2 = $mat3::from_axis_angle($vec3::unit_z(), deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2);
        }

        #[test]
        fn test_mat3_mul() {
            let mat_a = $mat3::from_axis_angle($vec3::unit_z(), deg(90.0));
            assert_approx_eq!($newvec3(-1.0, 0.0, 0.0), mat_a * $vec3::unit_y());
            assert_approx_eq!($newvec3(-1.0, 0.0, 0.0), mat_a.mul_vec3($vec3::unit_y()));
        }

        #[test]
        fn test_mat3_transform2d() {
            let mat_b = $mat3::from_scale_angle_translation(
                $vec2::new(0.5, 1.5),
                $t::to_radians(90.0),
                $vec2::new(1.0, 2.0),
            );
            let result2 = mat_b.transform_vector2($vec2::unit_y());
            assert_approx_eq!($vec2::new(-1.5, 0.0), result2, 1.0e-6);
            assert_approx_eq!(result2, (mat_b * $vec2::unit_y().extend(0.0)).truncate());

            let result2 = mat_b.transform_point2($vec2::unit_y());
            assert_approx_eq!($vec2::new(-0.5, 2.0), result2, 1.0e-6);
            assert_approx_eq!(result2, (mat_b * $vec2::unit_y().extend(1.0)).truncate());
        }

        #[test]
        fn test_from_ypr() {
            let zero = deg(0.0);
            let yaw = deg(30.0);
            let pitch = deg(60.0);
            let roll = deg(90.0);
            let y0 = $mat3::from_rotation_y(yaw);
            let y1 = $mat3::from_rotation_ypr(yaw, zero, zero);
            assert_approx_eq!(y0, y1);

            let x0 = $mat3::from_rotation_x(pitch);
            let x1 = $mat3::from_rotation_ypr(zero, pitch, zero);
            assert_approx_eq!(x0, x1);

            let z0 = $mat3::from_rotation_z(roll);
            let z1 = $mat3::from_rotation_ypr(zero, zero, roll);
            assert_approx_eq!(z0, z1);

            let yx0 = y0 * x0;
            let yx1 = $mat3::from_rotation_ypr(yaw, pitch, zero);
            assert_approx_eq!(yx0, yx1);

            let yxz0 = y0 * x0 * z0;
            let yxz1 = $mat3::from_rotation_ypr(yaw, pitch, roll);
            assert_approx_eq!(yxz0, yxz1, 1e-6);
        }

        #[test]
        fn test_from_scale() {
            let m = $mat3::from_scale($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!(m * $vec3::new(1.0, 1.0, 1.0), $vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!($vec3::unit_x() * 2.0, m.x_axis);
            assert_approx_eq!($vec3::unit_y() * 4.0, m.y_axis);
            assert_approx_eq!($vec3::unit_z() * 8.0, m.z_axis);
        }

        #[test]
        fn test_mat3_transpose() {
            let m = $newmat3(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            let mt = m.transpose();
            assert_eq!(mt.x_axis, $newvec3(1.0, 4.0, 7.0));
            assert_eq!(mt.y_axis, $newvec3(2.0, 5.0, 8.0));
            assert_eq!(mt.z_axis, $newvec3(3.0, 6.0, 9.0));
        }

        #[test]
        fn test_mat3_det() {
            assert_eq!(0.0, $mat3::zero().determinant());
            assert_eq!(1.0, $mat3::identity().determinant());
            assert_eq!(1.0, $mat3::from_rotation_x(deg(90.0)).determinant());
            assert_eq!(1.0, $mat3::from_rotation_y(deg(180.0)).determinant());
            assert_eq!(1.0, $mat3::from_rotation_z(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0 * 2.0,
                $mat3::from_scale($newvec3(2.0, 2.0, 2.0)).determinant()
            );
        }

        #[test]
        fn test_mat3_inverse() {
            // assert_eq!(None, $mat3::zero().inverse());
            let inv = $mat3::identity().inverse();
            // assert_ne!(None, inv);
            assert_approx_eq!($mat3::identity(), inv);

            let rotz = $mat3::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            // assert_ne!(None, rotz_inv);
            // let rotz_inv = rotz_inv.unwrap();
            assert_approx_eq!($mat3::identity(), rotz * rotz_inv);
            assert_approx_eq!($mat3::identity(), rotz_inv * rotz);

            let scale = $mat3::from_scale($newvec3(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            // assert_ne!(None, scale_inv);
            // let scale_inv = scale_inv.unwrap();
            assert_approx_eq!($mat3::identity(), scale * scale_inv);
            assert_approx_eq!($mat3::identity(), scale_inv * scale);

            let m = scale * rotz;
            let m_inv = m.inverse();
            // assert_ne!(None, m_inv);
            // let m_inv = m_inv.unwrap();
            assert_approx_eq!($mat3::identity(), m * m_inv);
            assert_approx_eq!($mat3::identity(), m_inv * m);
            assert_approx_eq!(m_inv, rotz_inv * scale_inv);
        }

        #[test]
        fn test_mat3_ops() {
            let m0 = $mat3::from_cols_array_2d(&MATRIX);
            let m0x2 = $mat3::from_cols_array_2d(&[
                [2.0, 4.0, 6.0],
                [8.0, 10.0, 12.0],
                [14.0, 16.0, 18.0],
            ]);
            assert_eq!(m0x2, m0 * 2.0);
            assert_eq!(m0x2, 2.0 * m0);
            assert_eq!(m0x2, m0 + m0);
            assert_eq!($mat3::zero(), m0 - m0);
            assert_approx_eq!(m0, m0 * $mat3::identity());
            assert_approx_eq!(m0, $mat3::identity() * m0);
        }

        #[test]
        fn test_mat3_fmt() {
            let a = $mat3::from_cols_array_2d(&MATRIX);
            assert_eq!(format!("{}", a), "[[1, 2, 3], [4, 5, 6], [7, 8, 9]]");
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let id = $mat3::identity();
            assert_eq!(vec![id, id].iter().sum::<$mat3>(), id + id);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $mat3::identity() + $mat3::identity();
            assert_eq!(vec![two, two].iter().product::<$mat3>(), two * two);
        }

        #[test]
        fn test_mat3_is_finite() {
            use std::$t::INFINITY;
            use std::$t::NAN;
            use std::$t::NEG_INFINITY;
            assert!($mat3::identity().is_finite());
            assert!(!($mat3::identity() * INFINITY).is_finite());
            assert!(!($mat3::identity() * NEG_INFINITY).is_finite());
            assert!(!($mat3::identity() * NAN).is_finite());
        }
    };
}

mod mat3 {
    use super::support::deg;
    use glam::{mat3, vec3, vec3a, Mat3, Vec2, Vec3, Vec3A};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(36, mem::size_of::<Mat3>());
        assert_eq!(4, mem::align_of::<Mat3>());
    }

    #[test]
    fn test_mul_vec3a() {
        let mat_a = Mat3::from_axis_angle(Vec3::unit_z(), deg(90.0));
        assert_approx_eq!(vec3a(-1.0, 0.0, 0.0), mat_a * Vec3A::unit_y());
        assert_approx_eq!(vec3a(-1.0, 0.0, 0.0), mat_a.mul_vec3a(Vec3A::unit_y()));
    }

    impl_mat3_tests!(mat3, Mat3, vec3, Vec3, Vec2, f32);
}

mod dmat3 {
    use super::support::deg;
    use glam::{dmat3, dvec3, DMat3, DVec2, DVec3};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(72, mem::size_of::<DMat3>());
        assert_eq!(8, mem::align_of::<DMat3>());
    }

    impl_mat3_tests!(dmat3, DMat3, dvec3, DVec3, DVec2, f64);
}
