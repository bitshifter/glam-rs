#[macro_use]
mod support;

macro_rules! impl_mat4_tests {
    ($newmat4:ident, $newvec4:ident, $newvec3:ident, $mat4:ident, $quat:ident, $vec4:ident, $vec3:ident, $t:ident) => {
        const IDENTITY: [[$t; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        const MATRIX: [[$t; 4]; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        const ZERO: [[$t; 4]; 4] = [[0.0; 4]; 4];

        #[test]
        fn test_mat4_identity() {
            let identity = $mat4::identity();
            assert_eq!(IDENTITY, identity.to_cols_array_2d());
            assert_eq!($mat4::from_cols_array_2d(&IDENTITY), identity);
            assert_eq!(identity, identity * identity);
            assert_eq!(identity, $mat4::default());
        }

        #[test]
        fn test_mat4_zero() {
            assert_eq!($mat4::from_cols_array_2d(&ZERO), $mat4::zero());
        }

        #[test]
        fn test_mat4_accessors() {
            let mut m = $mat4::zero();
            m.x_axis = $vec4::new(1.0, 2.0, 3.0, 4.0);
            m.y_axis = $vec4::new(5.0, 6.0, 7.0, 8.0);
            m.z_axis = $vec4::new(9.0, 10.0, 11.0, 12.0);
            m.w_axis = $vec4::new(13.0, 14.0, 15.0, 16.0);
            assert_eq!($mat4::from_cols_array_2d(&MATRIX), m);
            assert_eq!($vec4::new(1.0, 2.0, 3.0, 4.0), m.x_axis);
            assert_eq!($vec4::new(5.0, 6.0, 7.0, 8.0), m.y_axis);
            assert_eq!($vec4::new(9.0, 10.0, 11.0, 12.0), m.z_axis);
            assert_eq!($vec4::new(13.0, 14.0, 15.0, 16.0), m.w_axis);
        }

        #[test]
        fn test_mat4_from_axes() {
            let a = $mat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            assert_eq!(MATRIX, a.to_cols_array_2d());
            let b = $mat4::from_cols(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            assert_eq!(a, b);
            let c = $newmat4(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            assert_eq!(a, c);
            let d = b.to_cols_array();
            let f = $mat4::from_cols_array(&d);
            assert_eq!(b, f);
        }

        #[test]
        fn test_mat4_translation() {
            let translate = $mat4::from_translation($newvec3(1.0, 2.0, 3.0));
            assert_eq!(
                $mat4::from_cols(
                    $newvec4(1.0, 0.0, 0.0, 0.0),
                    $newvec4(0.0, 1.0, 0.0, 0.0),
                    $newvec4(0.0, 0.0, 1.0, 0.0),
                    $newvec4(1.0, 2.0, 3.0, 1.0)
                ),
                translate
            );
        }

        #[test]
        fn test_from_rotation() {
            let rot_x1 = $mat4::from_rotation_x(deg(180.0));
            let rot_x2 = $mat4::from_axis_angle($vec3::unit_x(), deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2);
            let rot_y1 = $mat4::from_rotation_y(deg(180.0));
            let rot_y2 = $mat4::from_axis_angle($vec3::unit_y(), deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2);
            let rot_z1 = $mat4::from_rotation_z(deg(180.0));
            let rot_z2 = $mat4::from_axis_angle($vec3::unit_z(), deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2);
        }

        #[test]
        fn test_mat4_mul() {
            let mat_a = $mat4::from_axis_angle($vec3::unit_z(), deg(90.0));
            let result3 = mat_a.transform_vector3($vec3::unit_y());
            assert_approx_eq!($newvec3(-1.0, 0.0, 0.0), result3);
            assert_approx_eq!(
                result3,
                (mat_a * $vec3::unit_y().extend(0.0)).truncate().into()
            );
            let result4 = mat_a * $vec4::unit_y();
            assert_approx_eq!($newvec4(-1.0, 0.0, 0.0, 0.0), result4);
            assert_approx_eq!(result4, mat_a * $vec4::unit_y());

            let mat_b = $mat4::from_scale_rotation_translation(
                $vec3::new(0.5, 1.5, 2.0),
                $quat::from_rotation_x(deg(90.0)),
                $vec3::new(1.0, 2.0, 3.0),
            );
            let result3 = mat_b.transform_vector3($vec3::unit_y());
            assert_approx_eq!($newvec3(0.0, 0.0, 1.5), result3, 1.0e-6);
            assert_approx_eq!(
                result3,
                (mat_b * $vec3::unit_y().extend(0.0)).truncate().into()
            );

            let result3 = mat_b.transform_point3($vec3::unit_y());
            assert_approx_eq!($newvec3(1.0, 2.0, 4.5), result3, 1.0e-6);
            assert_approx_eq!(
                result3,
                (mat_b * $vec3::unit_y().extend(1.0)).truncate().into()
            );
        }

        #[test]
        fn test_from_ypr() {
            let zero = deg(0.0);
            let yaw = deg(30.0);
            let pitch = deg(60.0);
            let roll = deg(90.0);
            let y0 = $mat4::from_rotation_y(yaw);
            let y1 = $mat4::from_rotation_ypr(yaw, zero, zero);
            assert_approx_eq!(y0, y1);

            let x0 = $mat4::from_rotation_x(pitch);
            let x1 = $mat4::from_rotation_ypr(zero, pitch, zero);
            assert_approx_eq!(x0, x1);

            let z0 = $mat4::from_rotation_z(roll);
            let z1 = $mat4::from_rotation_ypr(zero, zero, roll);
            assert_approx_eq!(z0, z1);

            let yx0 = y0 * x0;
            let yx1 = $mat4::from_rotation_ypr(yaw, pitch, zero);
            assert_approx_eq!(yx0, yx1);

            let yxz0 = y0 * x0 * z0;
            let yxz1 = $mat4::from_rotation_ypr(yaw, pitch, roll);
            assert_approx_eq!(yxz0, yxz1, 1e-6);
        }

        #[test]
        fn test_from_scale() {
            let m = $mat4::from_scale($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!($vec4::unit_x() * 2.0, m.x_axis);
            assert_approx_eq!($vec4::unit_y() * 4.0, m.y_axis);
            assert_approx_eq!($vec4::unit_z() * 8.0, m.z_axis);
            assert_approx_eq!($vec4::unit_w(), m.w_axis);
            assert_approx_eq!(
                m.transform_point3($vec3::new(1.0, 1.0, 1.0)),
                $vec3::new(2.0, 4.0, 8.0)
            );
        }

        #[test]
        fn test_mat4_transpose() {
            let m = $newmat4(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            let mt = m.transpose();
            assert_eq!(mt.x_axis, $newvec4(1.0, 5.0, 9.0, 13.0));
            assert_eq!(mt.y_axis, $newvec4(2.0, 6.0, 10.0, 14.0));
            assert_eq!(mt.z_axis, $newvec4(3.0, 7.0, 11.0, 15.0));
            assert_eq!(mt.w_axis, $newvec4(4.0, 8.0, 12.0, 16.0));
        }

        #[test]
        fn test_mat4_det() {
            assert_eq!(0.0, $mat4::zero().determinant());
            assert_eq!(1.0, $mat4::identity().determinant());
            assert_eq!(1.0, $mat4::from_rotation_x(deg(90.0)).determinant());
            assert_eq!(1.0, $mat4::from_rotation_y(deg(180.0)).determinant());
            assert_eq!(1.0, $mat4::from_rotation_z(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0 * 2.0,
                $mat4::from_scale($newvec3(2.0, 2.0, 2.0)).determinant()
            );
        }

        #[test]
        fn test_mat4_inverse() {
            // assert_eq!(None, $mat4::zero().inverse());
            let inv = $mat4::identity().inverse();
            // assert_ne!(None, inv);
            assert_approx_eq!($mat4::identity(), inv);

            let rotz = $mat4::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            // assert_ne!(None, rotz_inv);
            // let rotz_inv = rotz_inv.unwrap();
            assert_approx_eq!($mat4::identity(), rotz * rotz_inv);
            assert_approx_eq!($mat4::identity(), rotz_inv * rotz);

            let trans = $mat4::from_translation($newvec3(1.0, 2.0, 3.0));
            let trans_inv = trans.inverse();
            // assert_ne!(None, trans_inv);
            // let trans_inv = trans_inv.unwrap();
            assert_approx_eq!($mat4::identity(), trans * trans_inv);
            assert_approx_eq!($mat4::identity(), trans_inv * trans);

            let scale = $mat4::from_scale($newvec3(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            // assert_ne!(None, scale_inv);
            // let scale_inv = scale_inv.unwrap();
            assert_approx_eq!($mat4::identity(), scale * scale_inv);
            assert_approx_eq!($mat4::identity(), scale_inv * scale);

            let m = scale * rotz * trans;
            let m_inv = m.inverse();
            // assert_ne!(None, m_inv);
            // let m_inv = m_inv.unwrap();
            assert_approx_eq!($mat4::identity(), m * m_inv, 1.0e-5);
            assert_approx_eq!($mat4::identity(), m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rotz_inv * scale_inv, 1.0e-6);
        }

        #[test]
        fn test_mat4_decompose() {
            // identity
            let (out_scale, out_rotation, out_translation) =
                $mat4::identity().to_scale_rotation_translation();
            assert_approx_eq!($vec3::one(), out_scale);
            assert!(out_rotation.is_near_identity());
            assert_approx_eq!($vec3::zero(), out_translation);

            // no scale
            let in_scale = $vec3::one();
            let in_translation = $vec3::new(-2.0, 4.0, -0.125);
            let in_rotation = $quat::from_rotation_ypr(
                $t::to_radians(-45.0),
                $t::to_radians(180.0),
                $t::to_radians(270.0),
            );
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            // positive scale
            let in_scale = $vec3::new(1.0, 2.0, 4.0);
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            // negative scale
            let in_scale = $vec3::new(-4.0, 1.0, 2.0);
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(4.0, -1.0, -2.0);
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            // out_scale and out_rotation are different but they produce the same matrix
            // assert_approx_eq!(in_scale, out_scale, 1e-6);
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );
        }

        #[test]
        fn test_mat4_look_at() {
            let eye = $vec3::new(0.0, 0.0, -5.0);
            let center = $vec3::new(0.0, 0.0, 0.0);
            let up = $vec3::new(1.0, 0.0, 0.0);
            let lh = $mat4::look_at_lh(eye, center, up);
            let rh = $mat4::look_at_rh(eye, center, up);
            let point = $vec3::new(1.0, 0.0, 0.0);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));
        }

        #[test]
        fn test_mat4_perspective_gl_rh() {
            let projection = $mat4::perspective_rh_gl($t::to_radians(90.0), 2.0, 5.0, 15.0);

            let original = $vec3::new(5.0, 5.0, -15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected);

            let original = $vec3::new(5.0, 5.0, -5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -5.0, 5.0), projected);
        }

        #[test]
        fn test_mat4_perspective_lh() {
            let projection = $mat4::perspective_lh($t::to_radians(90.0), 2.0, 5.0, 15.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected);
        }

        #[test]
        fn test_mat4_perspective_infinite_lh() {
            let projection = $mat4::perspective_infinite_lh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 10.0, 15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected);
        }

        #[test]
        fn test_mat4_perspective_infinite_reverse_lh() {
            let projection = $mat4::perspective_infinite_reverse_lh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, 15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, 5.0), projected);
        }

        #[test]
        fn test_mat4_perspective_rh() {
            let projection = $mat4::perspective_rh($t::to_radians(90.0), 2.0, 5.0, 15.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -30.0, -15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -15.0, -5.0), projected);
        }

        #[test]
        fn test_mat4_perspective_infinite_rh() {
            let projection = $mat4::perspective_infinite_rh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -20.0, -15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -10.0, -5.0), projected);
        }

        #[test]
        fn test_mat4_perspective_infinite_reverse_rh() {
            let projection = $mat4::perspective_infinite_reverse_rh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, -15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, -5.0), projected);
        }

        #[test]
        fn test_mat4_orthographic_gl_rh() {
            let projection = $mat4::orthographic_rh_gl(-10.0, 10.0, -5.0, 5.0, 0.0, -10.0);
            let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, -2.0, 1.0));
        }

        #[test]
        fn test_mat4_orthographic_rh() {
            let projection = $mat4::orthographic_rh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
            let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));

            let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));
        }

        #[test]
        fn test_mat4_orthographic_lh() {
            let projection = $mat4::orthographic_lh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
            let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));

            let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));
        }

        #[test]
        fn test_mat4_ops() {
            let m0 = $mat4::from_cols_array_2d(&MATRIX);
            let m0x2 = $mat4::from_cols_array_2d(&[
                [2.0, 4.0, 6.0, 8.0],
                [10.0, 12.0, 14.0, 16.0],
                [18.0, 20.0, 22.0, 24.0],
                [26.0, 28.0, 30.0, 32.0],
            ]);
            assert_eq!(m0x2, m0 * 2.0);
            assert_eq!(m0x2, 2.0 * m0);
            assert_eq!(m0x2, m0 + m0);
            assert_eq!($mat4::zero(), m0 - m0);
            assert_approx_eq!(m0, m0 * $mat4::identity());
            assert_approx_eq!(m0, $mat4::identity() * m0);
        }

        #[test]
        fn test_mat4_fmt() {
            let a = $mat4::from_cols_array_2d(&MATRIX);
            assert_eq!(
                format!("{}", a),
                "[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]"
            );
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let id = $mat4::identity();
            assert_eq!(vec![id, id].iter().sum::<$mat4>(), id + id);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $mat4::identity() + $mat4::identity();
            assert_eq!(vec![two, two].iter().product::<$mat4>(), two * two);
        }

        #[test]
        fn test_mat4_is_finite() {
            use std::$t::INFINITY;
            use std::$t::NAN;
            use std::$t::NEG_INFINITY;
            assert!($mat4::identity().is_finite());
            assert!(!($mat4::identity() * INFINITY).is_finite());
            assert!(!($mat4::identity() * NEG_INFINITY).is_finite());
            assert!(!($mat4::identity() * NAN).is_finite());
        }
    };
}

mod mat3 {
    use super::support::deg;
    use glam::{mat4, vec3, vec4, Mat4, Quat, Vec3, Vec4};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(64, mem::size_of::<Mat4>());
        if cfg!(feature = "scalar-math") {
            assert_eq!(4, mem::align_of::<Mat4>());
        } else {
            assert_eq!(16, mem::align_of::<Mat4>());
        }
    }

    impl_mat4_tests!(mat4, vec4, vec3, Mat4, Quat, Vec4, Vec3, f32);
}

mod dmat3 {
    use super::support::deg;
    use glam::{dmat4, dvec3, dvec4, DMat4, DQuat, DVec3, DVec4};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(128, mem::size_of::<DMat4>());
        assert_eq!(8, mem::align_of::<DMat4>());
    }

    impl_mat4_tests!(dmat4, dvec4, dvec3, DMat4, DQuat, DVec4, DVec3, f64);
}
