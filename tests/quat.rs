use approx::assert_ulps_eq;
use glam::f32::{deg, quat, Quat, Vec3};
use std::mem;

#[test]
fn test_quat_rotation() {
    let zero = deg(0.0);
    let yaw = deg(30.0);
    let pitch = deg(60.0);
    let roll = deg(90.0);
    let y0 = Quat::from_rotation_y(yaw);
    let (axis, angle) = y0.get_axis_angle();
    assert_ulps_eq!(axis, Vec3::unit_y());
    assert_ulps_eq!(angle, yaw);
    let y1 = Quat::from_rotation_ypr(yaw, zero, zero);
    assert_ulps_eq!(y0, y1);
    let y2 = Quat::from_axis_angle(Vec3::unit_y(), yaw);
    assert_ulps_eq!(y0, y2);

    let x0 = Quat::from_rotation_x(pitch);
    let (axis, angle) = x0.get_axis_angle();
    assert_ulps_eq!(axis, Vec3::unit_x());
    assert_ulps_eq!(angle, pitch);
    let x1 = Quat::from_rotation_ypr(zero, pitch, zero);
    assert_ulps_eq!(x0, x1);
    let x2 = Quat::from_axis_angle(Vec3::unit_x(), pitch);
    assert_ulps_eq!(x0, x2);

    let z0 = Quat::from_rotation_z(roll);
    let (axis, angle) = z0.get_axis_angle();
    assert_ulps_eq!(axis, Vec3::unit_z());
    assert_ulps_eq!(angle, roll);
    let z1 = Quat::from_rotation_ypr(zero, zero, roll);
    assert_ulps_eq!(z0, z1);
    let z2 = Quat::from_axis_angle(Vec3::unit_z(), roll);
    assert_ulps_eq!(z0, z2);

    let yx0 = y0 * x0;
    let yx1 = Quat::from_rotation_ypr(yaw, pitch, zero);
    assert_ulps_eq!(yx0, yx1);

    let yxz0 = y0 * x0 * z0;
    let yxz1 = Quat::from_rotation_ypr(yaw, pitch, roll);
    assert_ulps_eq!(yxz0, yxz1);
}

#[test]
fn test_quat_new() {
    let ytheta = deg(45.0);
    let q0 = Quat::from_rotation_y(ytheta);

    assert_eq!(mem::size_of_val(&q0), 16);
    assert_eq!(mem::align_of_val(&q0), 16);

    let t1 = (
        0.0,
        (ytheta.as_radians() * 0.5).sin(),
        0.0,
        (ytheta.as_radians() * 0.5).cos(),
    );
    assert_eq!(q0, t1.into());
    let q1 = Quat::from(t1);
    assert_eq!(t1, q1.into());
    let q1 = Quat::from(&t1);
    assert_eq!(t1, (&q1).into());

    assert_eq!(q0, quat(t1.0, t1.1, t1.2, t1.3));

    let a1 = [
        0.0,
        (ytheta.as_radians() * 0.5).sin(),
        0.0,
        (ytheta.as_radians() * 0.5).cos(),
    ];
    assert_eq!(q0, a1.into());
    let q1 = Quat::from(a1);
    let a2: [f32; 4] = q1.into();
    assert_eq!(a1, a2);
    let q1 = Quat::from(&a1);
    let a2: [f32; 4] = (&q1).into();
    assert_eq!(a1, a2);
}

#[test]
fn test_quat_fmt() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(format!("{:?}", a), "Quat(1.0, 2.0, 3.0, 4.0)");
    // assert_eq!(
    //     format!("{:#?}", a),
    //     "Quat(\n    1.0,\n    2.0,\n    3.0,\n    4.0\n)"
    // );
    assert_eq!(format!("{}", a), "(1, 2, 3, 4)");
}

#[test]
fn test_quat_identity() {
    let identity = Quat::identity();
    assert_eq!(identity, Quat::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(identity, identity * identity);
    let q = Quat::from_rotation_ypr(deg(10.0), deg(-10.0), deg(45.0));
    assert_eq!(q, q * identity);
    assert_eq!(q, identity * q);
}

#[test]
fn test_quat_slice() {
    let a = [1.0, 2.0, 3.0, 4.0];
    let b = Quat::from_slice_unaligned(&a);
    let c: [f32; 4] = b.into();
    assert_eq!(a, c);
    let mut d = [0.0, 0.0, 0.0, 0.0];
    b.write_to_slice_unaligned(&mut d[..]);
    assert_eq!(a, d);
}

#[cfg(feature = "serde")]
#[test]
fn test_quat_serde() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Quat>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
}
