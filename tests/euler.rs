#[macro_use]
mod support;

/// Helper to get the 'canonical' version of a `Quat`. We define the canonical of quat `q` as:
/// * `q`, if q.w > epsilon
/// * `-q`, if q.w < -epsilon
/// * `(0, 0, 0, 1)` otherwise
/// The rationale is that q and -q represent the same rotation, and any (_, _, _, 0) represent no rotation at all.
trait CanonicalQuat: Copy {
    fn canonical(self) -> Self;
}

macro_rules! impl_canonical_quat {
    ($t:ty) => {
        impl CanonicalQuat for $t {
            fn canonical(self) -> Self {
                match self {
                    _ if self.w >= 1e-5 => self,
                    _ if self.w <= -1e-5 => -self,
                    _ => <$t>::from_xyzw(0.0, 0.0, 0.0, 1.0),
                }
            }
        }
    };
}

impl_canonical_quat!(glam::Quat);
impl_canonical_quat!(glam::DQuat);

/// Helper to set some alternative epsilons based on the floating point type used
trait EulerEpsilon {
    /// epsilon for comparing quaterions built from eulers and axis-angles
    const Q_EPS: f32;

    /// epsilon for comparing quaternion round-tripped through eulers (quat -> euler -> quat)
    const E_EPS: f32;
}

impl EulerEpsilon for f32 {
    const Q_EPS: f32 = 1e-5;
    const E_EPS: f32 = 1e-5;
}

impl EulerEpsilon for f64 {
    const Q_EPS: f32 = 1e-8;
    const E_EPS: f32 = 1e-8;
}

macro_rules! impl_3axis_test {
    ($name:ident, $t:ty, $quat:ident, $euler:path, $U:path, $V:path, $W:path, $vec:ident) => {
        glam_test!($name, {
            let euler = $euler;
            assert!($U != $W); // First and last axis must be different for three axis
            for u in (-180..=180).step_by(15) {
                for v in (-180..=180).step_by(15) {
                    for w in (-180..=180).step_by(15) {
                        let u1 = (u as $t).to_radians();
                        let v1 = (v as $t).to_radians();
                        let w1 = (w as $t).to_radians();

                        let q1: $quat = ($quat::from_axis_angle($U, u1)
                            * $quat::from_axis_angle($V, v1)
                            * $quat::from_axis_angle($W, w1))
                        .normalize();

                        // Test if the rotation is the expected
                        let q2: $quat = $quat::from_euler(euler, u1, v1, w1).normalize();
                        assert_approx_eq!(q1.canonical(), q2.canonical(), <$t>::Q_EPS);

                        // Test quat reconstruction from angles
                        let (u2, v2, w2) = q2.to_euler(euler);
                        let q3 = $quat::from_euler(euler, u2, v2, w2).normalize();
                        assert_approx_eq!(
                            q2.canonical(),
                            q3.canonical(),
                            <$t>::E_EPS,
                            format!(
                                "angles {:?} -> {:?}",
                                (u, v, w),
                                (u2.to_degrees(), v2.to_degrees(), w2.to_degrees())
                            )
                        );
                    }
                }
            }
        });
    };
}

macro_rules! impl_all_quat_tests_three_axis {
    ($t:ty, $q:ident, $v:ident) => {
        impl_3axis_test!(test_euler_zyx, $t, $q, ER::ZYX, $v::Z, $v::Y, $v::X, $v);
        impl_3axis_test!(test_euler_zxy, $t, $q, ER::ZXY, $v::Z, $v::X, $v::Y, $v);
        impl_3axis_test!(test_euler_yxz, $t, $q, ER::YXZ, $v::Y, $v::X, $v::Z, $v);
        impl_3axis_test!(test_euler_yzx, $t, $q, ER::YZX, $v::Y, $v::Z, $v::X, $v);
        impl_3axis_test!(test_euler_xyz, $t, $q, ER::XYZ, $v::X, $v::Y, $v::Z, $v);
        impl_3axis_test!(test_euler_xzy, $t, $q, ER::XZY, $v::X, $v::Z, $v::Y, $v);
    };
}

mod euler {
    use super::{CanonicalQuat, EulerEpsilon};
    use glam::*;

    // fn test_random_angles<F: Fn(&Mat4, &Fn(&Mat4, EulerRot) -> Mat4, EulerRot) -> Mat4>(rot_euler_rot: &F, order: EulerRot)
    // {
    //     use rand::{Rng, SeedableRng};
    //     use rand_xoshiro::Xoshiro256Plus;
    //     let mut rng = Xoshiro256Plus::seed_from_u64(0);
    //     let i = rng.gen_range(-f32::consts::PI, f32::consts::PI);
    //     let j = rng.gen_range(-f32::consts::PI, f32::consts::PI);
    //     let k = rng.gen_range(-f32::consts::PI, f32::consts::PI);
    //     for _ in 0..100000 {
    //         let m = Mat4::from_euler(order, i, j, k);

    //         // Add a small random error to the elements of m
    //         for c in 0..3 {
    //             let col = m.col_mut(c);
    //             for r in 0..3 {
    //                 col[r] = rng.get_range(-1e-7, 1e-7);
    //             }
    //         }

    //         // Extract Euler angles from m, convert the Euler angles back to a matrix n and verify that
    //         // the entries in m and n do not differ too much.
    //         test_matrix(m, &matrix_euler_matrix, order);
    //     }
    // }

    fn test_mat4_euler_mat4(order: EulerRot, a: i32, b: i32, c: i32) {
        println!("test_mat4_euler_mat4: {order:?} ({a}, {b}, {c})");
        let a = (a as f32).to_radians();
        let b = (b as f32).to_radians();
        let c = (c as f32).to_radians();
        let m = Mat4::from_euler(order, a, b, c);
        let (i, j, k) = m.to_euler(order);
        let n = Mat4::from_euler(order, i, j, k);
        assert_approx_eq!(m, n, 2e-6);
    }

    fn test_mat3_euler_mat3(order: EulerRot, a: i32, b: i32, c: i32) {
        println!("test_mat3_euler_mat3: {order:?} ({a}, {b}, {c})");
        let a = (a as f32).to_radians();
        let b = (b as f32).to_radians();
        let c = (c as f32).to_radians();
        let m = Mat3::from_euler(order, a, b, c);
        let (i, j, k) = m.to_euler(order);
        let n = Mat3::from_euler(order, i, j, k);
        assert_approx_eq!(m, n, 2e-6);
    }

    fn test_quat_euler_quat(order: EulerRot, a: i32, b: i32, c: i32) {
        println!("test_quat_euler_quat: {order:?} ({a}, {b}, {c})");
        let a = (a as f32).to_radians();
        let b = (b as f32).to_radians();
        let c = (c as f32).to_radians();
        let m = Quat::from_euler(order, a, b, c);
        let (i, j, k) = m.to_euler(order);
        let n = Quat::from_euler(order, i, j, k);
        assert_approx_eq!(m.canonical(), n.canonical(), 2e-6);
    }

    fn test_order_angles<F: Fn(EulerRot, i32, i32, i32)>(order: EulerRot, test: &F) {
        const STEP: usize = 90;
        for i in (0..360).step_by(STEP) {
            for j in (0..360).step_by(STEP) {
                for k in (0..360).step_by(STEP) {
                    test(order, i, j, k);
                }
            }
        }
    }

    fn test_all_orders<F: Fn(EulerRot)>(test: &F) {
        test(EulerRot::XYZ);
        test(EulerRot::XZY);
        test(EulerRot::YZX);
        test(EulerRot::YXZ);
        test(EulerRot::ZXY);
        test(EulerRot::ZYX);

        test(EulerRot::XZX);
        test(EulerRot::XYX);
        test(EulerRot::YXY);
        test(EulerRot::YZY);
        test(EulerRot::ZYZ);
        test(EulerRot::ZXZ);

        test(EulerRot::XYZEx);
        test(EulerRot::XZYEx);
        test(EulerRot::YZXEx);
        test(EulerRot::YXZEx);
        test(EulerRot::ZXYEx);
        test(EulerRot::ZYXEx);

        test(EulerRot::XZXEx);
        test(EulerRot::XYXEx);
        test(EulerRot::YXYEx);
        test(EulerRot::YZYEx);
        test(EulerRot::ZYZEx);
        test(EulerRot::ZXZEx);
    }

    #[test]
    fn test_all_mat4_euler_orders() {
        let test = |order| test_order_angles(order, &test_mat4_euler_mat4);
        test_all_orders(&test);
    }

    #[test]
    fn test_all_mat3_euler_orders() {
        let test = |order| test_order_angles(order, &test_mat3_euler_mat3);
        test_all_orders(&test);
    }

    #[test]
    fn test_all_quat_euler_orders() {
        let test = |order| test_order_angles(order, &test_quat_euler_quat);
        test_all_orders(&test);
    }

    mod quat {
        use super::*;
        type ER = EulerRot;

        impl_all_quat_tests_three_axis!(f32, Quat, Vec3);
    }

    mod dquat {
        use super::*;
        type ER = EulerRot;

        impl_all_quat_tests_three_axis!(f64, DQuat, DVec3);
    }
}
