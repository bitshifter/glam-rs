#![allow(clippy::excessive_precision)]
#![cfg(feature = "f16")]

#[macro_use]
mod support;

use glam::{hquat, EulerRot, HQuat, HVec3};
use half::f16;

fn f16x(v: f32) -> f16 {
    f16::from_f32(v)
}

fn f16_abs(x: f16) -> f16 {
    f16x(x.to_f32().abs())
}

glam_test!(test_hquat_consts, {
    assert_eq!(
        HQuat::IDENTITY,
        hquat(f16x(0.0), f16x(0.0), f16x(0.0), f16x(1.0))
    );
    assert_eq!(HQuat::IDENTITY, HQuat::default());
    assert!(HQuat::NAN.is_nan());
});

glam_test!(test_hquat_new, {
    let q = hquat(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    assert_eq!(q.x, f16x(1.0));
    assert_eq!(q.y, f16x(2.0));
    assert_eq!(q.z, f16x(3.0));
    assert_eq!(q.w, f16x(4.0));
    assert_eq!(
        q,
        HQuat::from_array([f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0)])
    );
    assert_eq!(
        q,
        HQuat::from_xyzw(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0))
    );
    assert_eq!(q.xyz(), HVec3::new(f16x(1.0), f16x(2.0), f16x(3.0)));
    assert_eq!(q.to_array(), [f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0)]);
    assert_eq!(
        q,
        HQuat::from_vec4(glam::hvec4(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0)))
    );
});

glam_test!(test_hquat_fmt, {
    let q = hquat(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    assert_eq!(
        format!("{:?}", q),
        format!("HQuat({:?}, {:?}, {:?}, {:?})", q.x, q.y, q.z, q.w)
    );
});

glam_test!(test_hquat_identity_rotation, {
    let q = HQuat::IDENTITY;
    let v = HVec3::new(f16x(1.0), f16x(2.0), f16x(3.0));
    assert_eq!(q.mul_vec3(v), v);
    assert!(q.is_normalized());
    assert!(q.is_near_identity());
    assert_eq!(q.conjugate(), q);
    assert_eq!(q.inverse(), q);
});

glam_test!(test_hquat_rotation_x, {
    let angle = f16x(core::f32::consts::FRAC_PI_2);
    let q = HQuat::from_rotation_x(angle);
    let v = HVec3::new(f16x(0.0), f16x(1.0), f16x(0.0));
    let rotated = q.mul_vec3(v);
    assert!(
        rotated.abs_diff_eq(HVec3::new(f16x(0.0), f16x(0.0), f16x(1.0)), f16x(1e-3)),
        "rotated: {:?}",
        rotated
    );
    assert!(q.is_normalized());
    assert_eq!(q, HQuat::from_rotation_x(angle).normalize());
});

glam_test!(test_hquat_rotation_y_z, {
    let angle = f16x(core::f32::consts::FRAC_PI_2);
    let qy = HQuat::from_rotation_y(angle);
    let v = HVec3::new(f16x(1.0), f16x(0.0), f16x(0.0));
    let rotated = qy.mul_vec3(v);
    assert!(
        rotated.abs_diff_eq(HVec3::new(f16x(0.0), f16x(0.0), f16x(-1.0)), f16x(1e-3)),
        "rotated: {:?}",
        rotated
    );

    let qz = HQuat::from_rotation_z(angle);
    let v = HVec3::new(f16x(1.0), f16x(0.0), f16x(0.0));
    let rotated = qz.mul_vec3(v);
    assert!(
        rotated.abs_diff_eq(HVec3::new(f16x(0.0), f16x(1.0), f16x(0.0)), f16x(1e-3)),
        "rotated: {:?}",
        rotated
    );
});

glam_test!(test_hquat_from_axis_angle, {
    let axis = HVec3::Y;
    let angle = f16x(core::f32::consts::FRAC_PI_2);
    let q = HQuat::from_axis_angle(axis, angle);
    let (a, ang) = q.to_axis_angle();
    assert!(a.abs_diff_eq(axis, f16x(1e-3)), "axis: {:?}", a);
    assert!(f16_abs(ang - angle) < f16x(1e-3), "angle: {:?}", ang);
    assert_eq!(q, HQuat::from_scaled_axis(axis * angle));
});

glam_test!(test_hquat_mul_quat, {
    let qx = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_2));
    let qy = HQuat::from_rotation_y(f16x(core::f32::consts::FRAC_PI_2));
    let v = HVec3::new(f16x(1.0), f16x(1.0), f16x(0.0));
    let r1 = (qy * qx).mul_vec3(v);
    let r2 = qy.mul_vec3(qx.mul_vec3(v));
    assert!(r1.abs_diff_eq(r2, f16x(1e-3)), "{} vs {}", r1, r2);
    assert_eq!(qx * HQuat::IDENTITY, qx);
    assert_eq!(HQuat::IDENTITY * qx, qx);
});

glam_test!(test_hquat_conjugate_inverse, {
    let q = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_3));
    let inv = q.inverse();
    assert!(q.mul_quat(inv).is_near_identity());
    assert!(inv.mul_quat(q).is_near_identity());
    assert_eq!(q.conjugate().length_squared(), q.length_squared());
    let qn = q.normalize();
    assert!(qn.is_normalized());
    assert!(f16_abs(qn.length() - f16x(1.0)) < f16x(1e-3));
});

glam_test!(test_hquat_dot_length, {
    let q = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_2));
    assert_eq!(q.dot(q), q.length_squared());
    assert!(f16_abs(q.dot(q) - f16x(1.0)) < f16x(1e-3));
    assert!(f16_abs(q.length() - f16x(1.0)) < f16x(1e-3));
    assert!(f16_abs(q.length_recip() - f16x(1.0)) < f16x(1e-3));
});

glam_test!(test_hquat_angle_between, {
    let q1 = HQuat::from_rotation_x(f16x(0.0));
    let q2 = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_2));
    assert!(f16_abs(q1.angle_between(q2) - f16x(core::f32::consts::FRAC_PI_2)) < f16x(1e-3));
    assert!(f16_abs(q1.angle_between(q1)) < f16x(1e-4));
});

glam_test!(test_hquat_lerp_slerp, {
    let q1 = HQuat::IDENTITY;
    let q2 = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_2));
    let l = q1.lerp(q2, f16x(0.5));
    let mid = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_4));
    assert!(l.abs_diff_eq(mid, f16x(1e-3)), "l: {:?}", l);
    let s = q1.slerp(q2, f16x(0.5));
    assert!(
        f16_abs(s.length_squared() - f16x(1.0)) < f16x(1e-3),
        "s: {:?}",
        s
    );
    assert!(s.abs_diff_eq(mid, f16x(1e-3)), "s: {:?}", s);
    let sl = q1.slerp_long(q2, f16x(0.5));
    assert!(
        f16_abs(sl.length_squared() - f16x(1.0)) < f16x(1e-3),
        "sl: {:?}",
        sl
    );
    assert!(sl.abs_diff_eq(mid, f16x(1e-3)), "sl: {:?}", sl);
    assert!(q1.lerp(q1, f16x(0.5)).is_near_identity());
});

glam_test!(test_hquat_rotate_towards, {
    let q1 = HQuat::IDENTITY;
    let q2 = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_2));
    let r = q1.rotate_towards(q2, f16x(core::f32::consts::FRAC_PI_4));
    assert!(r.abs_diff_eq(
        HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_4)),
        f16x(1e-3)
    ));
});

glam_test!(test_hquat_from_euler, {
    let q = HQuat::from_euler(EulerRot::XYZ, f16x(0.1), f16x(0.2), f16x(0.3));
    let (x, y, z) = q.to_euler(EulerRot::XYZ);
    assert!(f16_abs(x - f16x(0.1)) < f16x(1e-3), "x: {:?}", x);
    assert!(f16_abs(y - f16x(0.2)) < f16x(1e-3), "y: {:?}", y);
    assert!(f16_abs(z - f16x(0.3)) < f16x(1e-3), "z: {:?}", z);

    // identity maps to zero euler angles
    let (x, y, z) = HQuat::IDENTITY.to_euler(EulerRot::XYZ);
    assert!(f16_abs(x) < f16x(1e-4));
    assert!(f16_abs(y) < f16x(1e-4));
    assert!(f16_abs(z) < f16x(1e-4));
});

glam_test!(test_hquat_from_rotation_axes, {
    let q = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_3));
    let (x_axis, y_axis, z_axis) = (
        q.mul_vec3(HVec3::X),
        q.mul_vec3(HVec3::Y),
        q.mul_vec3(HVec3::Z),
    );
    let q2 = HQuat::from_rotation_axes(x_axis, y_axis, z_axis);
    let v = HVec3::new(f16x(1.0), f16x(2.0), f16x(3.0));
    assert!(q.mul_vec3(v).abs_diff_eq(q2.mul_vec3(v), f16x(1e-3)));
});

glam_test!(test_hquat_from_rotation_arc, {
    let from = HVec3::X;
    let to = HVec3::Y;
    let q = HQuat::from_rotation_arc(from, to);
    assert!(q.mul_vec3(from).abs_diff_eq(to, f16x(1e-3)));
    let qc = HQuat::from_rotation_arc_colinear(from, to);
    assert!(qc.mul_vec3(from).abs_diff_eq(to, f16x(1e-3)));
    let q2d = HQuat::from_rotation_arc_2d(from.truncate(), to.truncate());
    assert!(q2d.mul_vec3(from).abs_diff_eq(to, f16x(1e-3)));
});

glam_test!(test_hquat_to_scaled_axis, {
    let axis = HVec3::Y;
    let angle = f16x(core::f32::consts::FRAC_PI_2);
    let q = HQuat::from_axis_angle(axis, angle);
    let s = q.to_scaled_axis();
    assert!(s.abs_diff_eq(axis * angle, f16x(1e-3)), "s: {:?}", s);
});

glam_test!(test_hquat_abs_diff_eq, {
    let q = HQuat::from_rotation_x(f16x(0.5));
    assert!(q.abs_diff_eq(q, f16x(0.0)));
    let q2 = q * HQuat::from_rotation_x(f16x(0.001));
    assert!(q.abs_diff_eq(q2, f16x(0.01)));
    assert!(!q.abs_diff_eq(q2, f16x(1e-6)));
});

glam_test!(test_hquat_finite, {
    assert!(HQuat::IDENTITY.is_finite());
    assert!(!HQuat::NAN.is_finite());
    assert!(HQuat::IDENTITY.is_normalized());
});

glam_test!(test_hquat_as_conversions, {
    let q = HQuat::from_rotation_x(f16x(core::f32::consts::FRAC_PI_2));
    let qf32 = q.as_quat();
    let v = HVec3::new(f16x(0.0), f16x(1.0), f16x(0.0));
    let expected = q.mul_vec3(v);
    let r32 = qf32.mul_vec3(glam::Vec3::new(0.0, 1.0, 0.0));
    let r32_h = HVec3::from(r32);
    assert!(
        r32_h.abs_diff_eq(expected, f16x(1e-3)),
        "{} vs {}",
        r32_h,
        expected
    );
    #[cfg(feature = "f64")]
    {
        let qf64 = q.as_dquat();
        let r64 = qf64.mul_vec3(glam::DVec3::new(0.0, 1.0, 0.0));
        let r32_f64 = glam::DVec3::from(r32_h.as_vec3());
        assert!(r32_f64.angle_between(r64) < 1e-3, "{} vs {}", r32_f64, r64);
    }
});

glam_test!(test_hquat_slice, {
    let mut slice = [f16x(0.0); 4];
    let q = hquat(f16x(1.0), f16x(2.0), f16x(3.0), f16x(4.0));
    q.write_to_slice(&mut slice);
    assert_eq!(slice, q.to_array());
    assert_eq!(q, HQuat::from_slice(&slice));
});

glam_test!(test_hquat_mul_vec3_identity_roundtrip, {
    let q = HQuat::from_rotation_z(f16x(core::f32::consts::FRAC_PI_4));
    let v = HVec3::new(f16x(1.0), f16x(0.0), f16x(0.0));
    let rotated = q.mul_vec3(v);
    let back = q.inverse().mul_vec3(rotated);
    assert!(back.abs_diff_eq(v, f16x(1e-3)), "back: {:?}", back);
});
