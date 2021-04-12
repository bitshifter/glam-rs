use glam::{DQuat, Quat};

#[macro_use]
mod support;

/// Helper to calculate the inner angle in the range [0, 2*PI)
trait AngleDiff {
    type Output;
    fn angle_diff(self, other: Self) -> Self::Output;
}
#[macro_export]
macro_rules! impl_angle_diff {
    ($t:ty, $pi:expr) => {
        impl AngleDiff for $t {
            type Output = $t;
            fn angle_diff(self, other: $t) -> $t {
                const PI2: $t = $pi + $pi;
                let s = self.rem_euclid(PI2);
                let o = other.rem_euclid(PI2);
                if s > o {
                    (s - o).min(PI2 + o - s)
                } else {
                    (o - s).min(PI2 + s - o)
                }
            }
        }
    };
}
impl_angle_diff!(f32, std::f32::consts::PI);
impl_angle_diff!(f64, std::f64::consts::PI);

#[macro_export]
macro_rules! assert_approx_angle {
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = ($a, $b);
        let eps = $eps;
        let diff = a.angle_diff(b);
        assert!(
            diff < $eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            a,
            b,
            eps,
            diff
        );
    }};
}

trait EqApprox {
    type EPS;
    fn eq_approx(self, other: Self, axis_eps: Self::EPS, rot_axis: Self::EPS) -> bool;
}

macro_rules! impl_eq_approx {
    ($t:ty, $quat:ident, $pi:expr) => {
        impl EqApprox for $quat {
            type EPS = $t;
            fn eq_approx(self, other: Self, axis_eps: Self::EPS, rot_axis: Self::EPS) -> bool {
                let (s_axis, s_angle) = self.to_axis_angle();
                let (o_axis, o_angle) = other.to_axis_angle();
                if s_angle.abs() < rot_axis && o_angle.abs() < rot_axis {
                    // No rotation
                    true
                } else {
                    let a = s_axis.angle_between(o_axis);
                    if a < axis_eps {
                        // Same axes
                        (s_angle - o_angle).abs() < rot_axis
                    } else if ($pi - a).abs() < axis_eps {
                        // Inverted axes (180°)
                        (s_angle + o_angle).abs() < rot_axis
                    } else {
                        // Other
                        false
                    }
                }
            }
        }
    };
}
impl_eq_approx!(f32, Quat, std::f32::consts::PI);
impl_eq_approx!(f64, DQuat, std::f64::consts::PI);

#[macro_export]
macro_rules! impl_three_axis_test {
    ($name:ident, $t:ty, $quat:ident, $euler:path, $U:path, $V:path, $W:path, $vec:ident) => {
        #[test]
        fn $name() {
            let euler = $euler;
            assert!($U != $W); // First and last axis must be different for three axis
            for u in -179..=179 {
                if u % 2 != 0 {
                    continue;
                }
                for v in -89..=89 {
                    if v % 2 != 0 {
                        continue;
                    }
                    for w in -179..=179 {
                        if w % 2 != 0 {
                            continue;
                        }
                        let u1 = (u as $t).to_radians();
                        let v1 = (v as $t).to_radians();
                        let w1 = (w as $t).to_radians();

                        let q1: $quat = ($quat::from_axis_angle($U, u1)
                            * $quat::from_axis_angle($V, v1)
                            * $quat::from_axis_angle($W, w1))
                        .normalize();

                        // Test if the rotation is the expected
                        let q2: $quat = euler.to_quat(u1, v1, w1).normalize();
                        assert_approx_eq!(q1, q2, 1e-5);

                        // Test angle reconstruction
                        let (u2, v2, w2) = euler.from_quat(q1);
                        let q3 = euler.to_quat(u2, v2, w2).normalize();

                        assert_approx_angle!(u1, u2, 1e-4 as $t);
                        assert_approx_angle!(v1, v2, 1e-4 as $t);
                        assert_approx_angle!(w1, w2, 1e-4 as $t);

                        assert_approx_eq!(q1 * $vec::X, q3 * $vec::X, 1e-4);
                        assert_approx_eq!(q1 * $vec::Y, q3 * $vec::Y, 1e-4);
                        assert_approx_eq!(q1 * $vec::Z, q3 * $vec::Z, 1e-4);
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_two_axis_test {
    ($name:ident, $t:ty, $quat:ident, $euler:path, $U:path, $V:path, $W:path, $vec:ident) => {
        #[test]
        fn $name() {
            #[allow(deprecated)]
            let euler = $euler;
            assert!($U == $W); // First and last axis must be different for three axis
            for u in -179..=179 {
                if u % 2 != 0 {
                    continue;
                }
                for v in -89..=89 {
                    if v % 2 != 0 {
                        continue;
                    }
                    for w in -179..=179 {
                        if w % 2 != 0 {
                            continue;
                        }
                        let u1 = (u as $t).to_radians();
                        let v1 = (v as $t).to_radians();
                        let w1 = (w as $t).to_radians();

                        let q1: $quat = ($quat::from_axis_angle($U, u1)
                            * $quat::from_axis_angle($V, v1)
                            * $quat::from_axis_angle($W, w1))
                        .normalize();

                        // Test if the rotation is the expected
                        let q2: $quat = euler.to_quat(u1, v1, w1).normalize();
                        assert_approx_eq!(q1, q2, 1e-5);

                        // Test angle reconstruction
                        let (u2, v2, w2) = euler.from_quat(q1);
                        let _q3 = euler.to_quat(u2, v2, w2).normalize();

                        //

                        // assert_approx_angle!(u1, u2, 1e-4 as $t);
                        // assert_approx_angle!(v1, v2, 1e-4 as $t);
                        // assert_approx_angle!(w1, w2, 1e-4 as $t);

                        // assert_approx_eq!(q1 * $vec::X, q3 * $vec::X, 1e-4);
                        // assert_approx_eq!(q1 * $vec::Y, q3 * $vec::Y, 1e-4);
                        // assert_approx_eq!(q1 * $vec::Z, q3 * $vec::Z, 1e-4);
                    }
                }
            }
        }
    };
}

macro_rules! impl_all_quat_tests_three_axis {
    ($t:ty, $q:ident, $v:ident) => {
        impl_three_axis_test!(test_zyx, $t, $q, EulerRot::ZYXi, $v::Z, $v::Y, $v::X, $v);
        impl_three_axis_test!(test_zxy, $t, $q, EulerRot::ZXYi, $v::Z, $v::X, $v::Y, $v);
        impl_three_axis_test!(test_yxz, $t, $q, EulerRot::YXZi, $v::Y, $v::X, $v::Z, $v);
        impl_three_axis_test!(test_yzx, $t, $q, EulerRot::YZXi, $v::Y, $v::Z, $v::X, $v);
        impl_three_axis_test!(test_xyz, $t, $q, EulerRot::XYZi, $v::X, $v::Y, $v::Z, $v);
        impl_three_axis_test!(test_xzy, $t, $q, EulerRot::XZYi, $v::X, $v::Z, $v::Y, $v);
    };
}

macro_rules! impl_all_quat_tests_two_axis {
    ($t:ty, $q:ident, $v:ident) => {
        impl_two_axis_test!(test_zyz, $t, $q, EulerRot::ZYZi, $v::Z, $v::Y, $v::Z, $v);
        impl_two_axis_test!(test_zxz, $t, $q, EulerRot::ZXZi, $v::Z, $v::X, $v::Z, $v);
        impl_two_axis_test!(test_yxy, $t, $q, EulerRot::YXYi, $v::Y, $v::X, $v::Y, $v);
        impl_two_axis_test!(test_yzy, $t, $q, EulerRot::YZYi, $v::Y, $v::Z, $v::Y, $v);
        impl_two_axis_test!(test_xyx, $t, $q, EulerRot::XYXi, $v::X, $v::Y, $v::X, $v);
        impl_two_axis_test!(test_xzx, $t, $q, EulerRot::XZXi, $v::X, $v::Z, $v::X, $v);
    };
}

#[cfg(test)]
mod quat {
    use super::AngleDiff;
    use glam::*;

    impl_all_quat_tests_three_axis!(f32, Quat, Vec3);

    impl_all_quat_tests_two_axis!(f32, Quat, Vec3);
}

#[cfg(test)]
mod dquat {
    use super::AngleDiff;
    use glam::*;

    impl_all_quat_tests_three_axis!(f64, DQuat, DVec3);
    impl_all_quat_tests_two_axis!(f64, DQuat, DVec3);
}
