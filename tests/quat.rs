#[macro_use]
mod support;

macro_rules! impl_quat_tests {
    ($t:ident, $const_new:ident, $new:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec3:ident, $vec4:ident) => {
        use core::$t::INFINITY;
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_const() {
            const Q: $quat = $const_new!([1.0, 2.0, 3.0, 4.0]);
            assert_eq!($quat::from_xyzw(1.0, 2.0, 3.0, 4.0), Q);
        }

        #[test]
        fn test_new() {
            let ytheta = deg(45.0);
            let q0 = $quat::from_rotation_y(ytheta);

            let t1 = (0.0, (ytheta * 0.5).sin(), 0.0, (ytheta * 0.5).cos());
            assert_eq!(q0, t1.into());
            let q1 = $quat::from(t1);
            assert_eq!(t1, q1.into());

            assert_eq!(q0, $new(t1.0, t1.1, t1.2, t1.3));

            let a1 = [0.0, (ytheta * 0.5).sin(), 0.0, (ytheta * 0.5).cos()];
            assert_eq!(q0, a1.into());
            let q1 = $quat::from(a1);
            let a2: [$t; 4] = q1.into();
            assert_eq!(a1, a2);
        }

        #[test]
        fn test_funcs() {
            let q0 = $quat::from_rotation_ypr(deg(45.0), deg(180.0), deg(90.0));
            assert!(q0.is_normalized());
            assert_approx_eq!(q0.length_squared(), 1.0);
            assert_approx_eq!(q0.length(), 1.0);
            assert_approx_eq!(q0.length_recip(), 1.0);
            assert_approx_eq!(q0, q0.normalize());

            assert_approx_eq!(q0.dot(q0), 1.0);
            assert_approx_eq!(q0.dot(q0), 1.0);

            let q1 = $quat::from($vec4::from(q0) * 2.0);
            assert!(!q1.is_normalized());
            assert_approx_eq!(q1.length_squared(), 4.0, 1.0e-6);
            assert_approx_eq!(q1.length(), 2.0);
            assert_approx_eq!(q1.length_recip(), 0.5);
            assert_approx_eq!(q0, q1.normalize());
            assert_approx_eq!(q0.dot(q1), 2.0, 1.0e-6);
        }

        #[test]
        fn test_rotation() {
            let zero = deg(0.0);
            let yaw = deg(30.0);
            let pitch = deg(60.0);
            let roll = deg(90.0);
            let y0 = $quat::from_rotation_y(yaw);
            assert!(y0.is_normalized());
            let (axis, angle) = y0.to_axis_angle();
            assert_approx_eq!(axis, $vec3::Y, 1.0e-6);
            assert_approx_eq!(angle, yaw);
            let y1 = $quat::from_rotation_ypr(yaw, zero, zero);
            assert_approx_eq!(y0, y1);
            let y2 = $quat::from_axis_angle($vec3::Y, yaw);
            assert_approx_eq!(y0, y2);
            let y3 = $quat::from_rotation_mat3(&$mat3::from_rotation_y(yaw));
            assert_approx_eq!(y0, y3);
            let y4 = $quat::from_rotation_mat3(&$mat3::from_quat(y0));
            assert_approx_eq!(y0, y4);

            let x0 = $quat::from_rotation_x(pitch);
            assert!(x0.is_normalized());
            let (axis, angle) = x0.to_axis_angle();
            assert_approx_eq!(axis, $vec3::X);
            assert_approx_eq!(angle, pitch);
            let x1 = $quat::from_rotation_ypr(zero, pitch, zero);
            assert_approx_eq!(x0, x1);
            let x2 = $quat::from_axis_angle($vec3::X, pitch);
            assert_approx_eq!(x0, x2);
            let x3 = $quat::from_rotation_mat4(&$mat4::from_rotation_x(deg(180.0)));
            assert!(x3.is_normalized());
            assert_approx_eq!($quat::from_rotation_x(deg(180.0)), x3);

            let z0 = $quat::from_rotation_z(roll);
            assert!(z0.is_normalized());
            let (axis, angle) = z0.to_axis_angle();
            assert_approx_eq!(axis, $vec3::Z);
            assert_approx_eq!(angle, roll);
            let z1 = $quat::from_rotation_ypr(zero, zero, roll);
            assert_approx_eq!(z0, z1);
            let z2 = $quat::from_axis_angle($vec3::Z, roll);
            assert_approx_eq!(z0, z2);
            let z3 = $quat::from_rotation_mat4(&$mat4::from_rotation_z(roll));
            assert_approx_eq!(z0, z3);

            let yx0 = y0 * x0;
            assert!(yx0.is_normalized());
            let yx1 = $quat::from_rotation_ypr(yaw, pitch, zero);
            assert_approx_eq!(yx0, yx1);

            let yxz0 = y0 * x0 * z0;
            assert!(yxz0.is_normalized());
            let yxz1 = $quat::from_rotation_ypr(yaw, pitch, roll);
            assert_approx_eq!(yxz0, yxz1);

            // use the conjugate of z0 to remove the rotation from yxz0
            let yx2 = yxz0 * z0.conjugate();
            assert_approx_eq!(yx0, yx2);

            let yxz2 = $quat::from_rotation_mat4(&$mat4::from_quat(yxz0));
            assert_approx_eq!(yxz0, yxz2);

            // if near identity, just returns x axis and 0 rotation
            let (axis, angle) = $quat::IDENTITY.to_axis_angle();
            assert_eq!(axis, $vec3::X);
            assert_eq!(angle, rad(0.0));
        }

        #[test]
        fn test_mul_vec3() {
            let qrz = $quat::from_rotation_z(deg(90.0));
            assert_approx_eq!($vec3::Y, qrz * $vec3::X);
            assert_approx_eq!($vec3::Y, qrz.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Y, -qrz * $vec3::X);
            assert_approx_eq!($vec3::Y, qrz.neg().mul_vec3($vec3::X));
            assert_approx_eq!(-$vec3::X, qrz * $vec3::Y);
            assert_approx_eq!(-$vec3::X, qrz.mul_vec3($vec3::Y));
            assert_approx_eq!(-$vec3::X, -qrz * $vec3::Y);
            assert_approx_eq!(-$vec3::X, qrz.neg().mul_vec3($vec3::Y));

            // check vec3 * mat3 is the same
            let mrz = $mat3::from_quat(qrz);
            assert_approx_eq!($vec3::Y, mrz * $vec3::X);
            assert_approx_eq!($vec3::Y, mrz.mul_vec3($vec3::X));
            // assert_approx_eq!($vec3::Y, -mrz * $vec3::X);
            assert_approx_eq!(-$vec3::X, mrz * $vec3::Y);
            assert_approx_eq!(-$vec3::X, mrz.mul_vec3($vec3::Y));

            let qrx = $quat::from_rotation_x(deg(90.0));
            assert_approx_eq!($vec3::X, qrx * $vec3::X);
            assert_approx_eq!($vec3::X, qrx.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::X, -qrx * $vec3::X);
            assert_approx_eq!($vec3::X, qrx.neg().mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, qrx * $vec3::Y);
            assert_approx_eq!($vec3::Z, qrx.mul_vec3($vec3::Y));
            assert_approx_eq!($vec3::Z, -qrx * $vec3::Y);
            assert_approx_eq!($vec3::Z, qrx.neg().mul_vec3($vec3::Y));

            // check vec3 * mat3 is the same
            let mrx = $mat3::from_quat(qrx);
            assert_approx_eq!($vec3::X, mrx * $vec3::X);
            assert_approx_eq!($vec3::X, mrx.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, mrx * $vec3::Y);
            assert_approx_eq!($vec3::Z, mrx.mul_vec3($vec3::Y));

            let qrxz = qrz * qrx;
            assert_approx_eq!($vec3::Y, qrxz * $vec3::X);
            assert_approx_eq!($vec3::Y, qrxz.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, qrxz * $vec3::Y);
            assert_approx_eq!($vec3::Z, qrxz.mul_vec3($vec3::Y));

            let mrxz = mrz * mrx;
            assert_approx_eq!($vec3::Y, mrxz * $vec3::X);
            assert_approx_eq!($vec3::Y, mrxz.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, mrxz * $vec3::Y);
            assert_approx_eq!($vec3::Z, mrxz.mul_vec3($vec3::Y));

            let qrzx = qrx * qrz;
            assert_approx_eq!($vec3::Z, qrzx * $vec3::X);
            assert_approx_eq!($vec3::Z, qrzx.mul_vec3($vec3::X));
            assert_approx_eq!(-$vec3::X, qrzx * $vec3::Y);
            assert_approx_eq!(-$vec3::X, qrzx.mul_vec3($vec3::Y));

            let mrzx = qrx * qrz;
            assert_approx_eq!($vec3::Z, mrzx * $vec3::X);
            assert_approx_eq!($vec3::Z, mrzx.mul_vec3($vec3::X));
            assert_approx_eq!(-$vec3::X, mrzx * $vec3::Y);
            assert_approx_eq!(-$vec3::X, mrzx.mul_vec3($vec3::Y));
        }
        #[test]
        fn test_lerp() {
            let q0 = $quat::from_rotation_y(deg(0.0));
            let q1 = $quat::from_rotation_y(deg(90.0));
            assert_approx_eq!(q0, q0.lerp(q1, 0.0));
            assert_approx_eq!(q1, q0.lerp(q1, 1.0));
            assert_approx_eq!($quat::from_rotation_y(deg(45.0)), q0.lerp(q1, 0.5));
        }

        #[test]
        fn test_slerp() {
            let q0 = $quat::from_rotation_y(deg(0.0));
            let q1 = $quat::from_rotation_y(deg(90.0));
            assert_approx_eq!(q0, q0.slerp(q1, 0.0), 1.0e-3);
            assert_approx_eq!(q1, q0.slerp(q1, 1.0), 1.0e-3);
            assert_approx_eq!($quat::from_rotation_y(deg(45.0)), q0.slerp(q1, 0.5), 1.0e-3);
        }

        #[test]
        fn test_slerp_constant_speed() {
            let step = 0.01;
            let mut s = 0.0;
            while s <= 1.0 {
                let q0 = $quat::from_rotation_y(deg(0.0));
                let q1 = $quat::from_rotation_y(deg(90.0));
                assert_approx_eq!(
                    $quat::from_rotation_y(deg(s * 90.0)),
                    q0.slerp(q1, s),
                    1.0e-3
                );
                s += step;
            }
        }

        #[test]
        fn test_fmt() {
            let a = $quat::IDENTITY;
            assert_eq!(
                format!("{:?}", a),
                format!("{}(0.0, 0.0, 0.0, 1.0)", stringify!($quat))
            );
            // assert_eq!(
            //     format!("{:#?}", a),
            //     "$quat(\n    1.0,\n    2.0,\n    3.0,\n    4.0\n)"
            // );
            assert_eq!(format!("{}", a), "[0, 0, 0, 1]");
        }

        #[test]
        fn test_identity() {
            let identity = $quat::IDENTITY;
            assert!(identity.is_near_identity());
            assert!(identity.is_normalized());
            assert_eq!(identity, $quat::from_xyzw(0.0, 0.0, 0.0, 1.0));
            assert_eq!(identity, identity * identity);
            let q = $quat::from_rotation_ypr(deg(10.0), deg(-10.0), deg(45.0));
            assert_eq!(q, q * identity);
            assert_eq!(q, identity * q);
            assert_eq!(identity, $quat::default());
        }

        #[test]
        fn test_slice() {
            let a: [$t; 4] = $quat::from_rotation_ypr(deg(30.0), deg(60.0), deg(90.0)).into();
            let b = $quat::from_slice_unaligned(&a);
            let c: [$t; 4] = b.into();
            assert_eq!(a, c);
            let mut d = [0.0, 0.0, 0.0, 0.0];
            b.write_to_slice_unaligned(&mut d[..]);
            assert_eq!(a, d);
        }

        #[test]
        fn test_elements() {
            let x = 1.0;
            let y = 2.0;
            let z = 3.0;
            let w = 4.0;

            let a = $quat::from_xyzw(x, y, z, w);
            assert!(a.x == x);
            assert!(a.y == y);
            assert!(a.z == z);
            assert!(a.w == w);
        }

        #[test]
        fn test_addition() {
            let a = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            let b = $quat::from_xyzw(5.0, 6.0, 7.0, -9.0);
            assert_eq!(a + b, $quat::from_xyzw(6.0, 8.0, 10.0, -5.0));
        }

        #[test]
        fn test_subtraction() {
            let a = $quat::from_xyzw(6.0, 8.0, 10.0, -5.0);
            let b = $quat::from_xyzw(5.0, 6.0, 7.0, -9.0);
            assert_eq!(a - b, $quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        }

        #[test]
        fn test_scalar_multiplication() {
            let a = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            assert_eq!(a * 2.0, $quat::from_xyzw(2.0, 4.0, 6.0, 8.0));
        }

        #[test]
        fn test_scalar_division() {
            let a = $quat::from_xyzw(2.0, 4.0, 6.0, 8.0);
            assert_eq!(a / 2.0, $quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let two = $new(2.0, 2.0, 2.0, 2.0);
            assert_eq!(vec![two, two].iter().sum::<$quat>(), two + two);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $new(2.0, 2.0, 2.0, 2.0).normalize();
            assert_eq!(vec![two, two].iter().product::<$quat>(), two * two);
        }

        #[test]
        fn test_is_finite() {
            assert!($quat::from_xyzw(0.0, 0.0, 0.0, 0.0).is_finite());
            assert!($quat::from_xyzw(-1e-10, 1.0, 1e10, 42.0).is_finite());
            assert!(!$quat::from_xyzw(INFINITY, 0.0, 0.0, 0.0).is_finite());
            assert!(!$quat::from_xyzw(0.0, NAN, 0.0, 0.0).is_finite());
            assert!(!$quat::from_xyzw(0.0, 0.0, NEG_INFINITY, 0.0).is_finite());
            assert!(!$quat::from_xyzw(0.0, 0.0, 0.0, NAN).is_finite());
        }

        #[test]
        fn test_rotation_arc() {
            let eps = 2.0 * core::$t::EPSILON.sqrt();

            for &from in &vec3_float_test_vectors!($vec3) {
                let from = from.normalize();

                {
                    let q = $quat::from_rotation_arc(from, from);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                {
                    let q = $quat::from_rotation_arc_colinear(from, from);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                {
                    let to = -from;
                    let q = $quat::from_rotation_arc(from, to);
                    assert!(q.is_normalized());
                    assert!((q * from - to).length() < eps);
                }

                {
                    let to = -from;
                    let q = $quat::from_rotation_arc_colinear(from, to);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                for &to in &vec3_float_test_vectors!($vec3) {
                    let to = to.normalize();

                    let q = $quat::from_rotation_arc(from, to);
                    assert!(q.is_normalized());
                    assert!((q * from - to).length() < eps);

                    let q = $quat::from_rotation_arc_colinear(from, to);
                    assert!(q.is_normalized());
                    let transformed = q * from;
                    assert!(
                        (transformed - to).length() < eps || (-transformed - to).length() < eps
                    );
                }
            }
        }
    };
}

mod quat {
    use crate::support::{deg, rad};
    use core::ops::Neg;
    use glam::{const_quat, quat, Mat3, Mat4, Quat, Vec3, Vec3A, Vec4};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(16, mem::size_of::<Quat>());
        if cfg!(feature = "scalar-math") {
            assert_eq!(4, mem::align_of::<Quat>());
        } else {
            assert_eq!(16, mem::align_of::<Quat>());
        }
    }

    #[test]
    fn test_mul_vec3a() {
        let qrz = Quat::from_rotation_z(deg(90.0));
        assert_approx_eq!(Vec3A::Y, qrz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, qrz.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Y, -qrz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, qrz.neg().mul_vec3a(Vec3A::X));
        assert_approx_eq!(-Vec3A::X, qrz * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, qrz.mul_vec3a(Vec3A::Y));
        assert_approx_eq!(-Vec3A::X, -qrz * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, qrz.neg().mul_vec3a(Vec3A::Y));

        // check vec3 * mat3 is the same
        let mrz = Mat3::from_quat(qrz);
        assert_approx_eq!(Vec3A::Y, mrz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, mrz.mul_vec3a(Vec3A::X));
        // assert_approx_eq!(Vec3A::Y, -mrz * Vec3A::X);
        assert_approx_eq!(-Vec3A::X, mrz * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, mrz.mul_vec3a(Vec3A::Y));

        let qrx = Quat::from_rotation_x(deg(90.0));
        assert_approx_eq!(Vec3A::X, qrx * Vec3A::X);
        assert_approx_eq!(Vec3A::X, qrx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::X, -qrx * Vec3A::X);
        assert_approx_eq!(Vec3A::X, qrx.neg().mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, qrx * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, qrx.mul_vec3a(Vec3A::Y));
        assert_approx_eq!(Vec3A::Z, -qrx * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, qrx.neg().mul_vec3a(Vec3A::Y));

        // check vec3 * mat3 is the same
        let mrx = Mat3::from_quat(qrx);
        assert_approx_eq!(Vec3A::X, mrx * Vec3A::X);
        assert_approx_eq!(Vec3A::X, mrx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, mrx * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, mrx.mul_vec3a(Vec3A::Y));

        let qrxz = qrz * qrx;
        assert_approx_eq!(Vec3A::Y, qrxz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, qrxz.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, qrxz * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, qrxz.mul_vec3a(Vec3A::Y));

        let mrxz = mrz * mrx;
        assert_approx_eq!(Vec3A::Y, mrxz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, mrxz.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, mrxz * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, mrxz.mul_vec3a(Vec3A::Y));

        let qrzx = qrx * qrz;
        assert_approx_eq!(Vec3A::Z, qrzx * Vec3A::X);
        assert_approx_eq!(Vec3A::Z, qrzx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(-Vec3A::X, qrzx * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, qrzx.mul_vec3a(Vec3A::Y));

        let mrzx = qrx * qrz;
        assert_approx_eq!(Vec3A::Z, mrzx * Vec3A::X);
        assert_approx_eq!(Vec3A::Z, mrzx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(-Vec3A::X, mrzx * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, mrzx.mul_vec3a(Vec3A::Y));
    }

    #[test]
    fn test_as() {
        use glam::DQuat;
        assert_approx_eq!(
            DQuat::from_rotation_ypr(1.0, 2.0, 3.0),
            Quat::from_rotation_ypr(1.0, 2.0, 3.0).as_f64()
        );
        assert_approx_eq!(
            Quat::from_rotation_ypr(1.0, 2.0, 3.0),
            DQuat::from_rotation_ypr(1.0, 2.0, 3.0).as_f32()
        );
    }

    impl_quat_tests!(f32, const_quat, quat, Mat3, Mat4, Quat, Vec3, Vec4);
}

mod dquat {
    use crate::support::{deg, rad};
    use core::ops::Neg;
    use glam::{const_dquat, dquat, DMat3, DMat4, DQuat, DVec3, DVec4};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(32, mem::size_of::<DQuat>());
        assert_eq!(8, mem::align_of::<DQuat>());
    }

    impl_quat_tests!(f64, const_dquat, dquat, DMat3, DMat4, DQuat, DVec3, DVec4);
}
