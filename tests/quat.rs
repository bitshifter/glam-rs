use approx::assert_ulps_eq;
use glam::f32::{deg, Quat};

#[test]
fn test_from_ypr() {
    let zero = deg(0.0);
    let yaw = deg(30.0);
    let pitch = deg(60.0);
    let roll = deg(90.0);
    let y0 = Quat::from_rotation_y(yaw);
    let y1 = Quat::from_ypr(yaw, zero, zero);
    assert_ulps_eq!(y0, y1);

    let x0 = Quat::from_rotation_x(pitch);
    let x1 = Quat::from_ypr(zero, pitch, zero);
    assert_ulps_eq!(x0, x1);

    let z0 = Quat::from_rotation_z(roll);
    let z1 = Quat::from_ypr(zero, zero, roll);
    assert_ulps_eq!(z0, z1);

    // TODO: this order is not what I expected... check Quat::from_ypr
    let yx0 = x0 * y0;
    let yx1 = Quat::from_ypr(yaw, pitch, zero);
    assert_ulps_eq!(yx0, yx1);

    let yxz0 = z0 * x0 * y0;
    let yxz1 = Quat::from_ypr(yaw, pitch, roll);
    assert_ulps_eq!(yxz0, yxz1);
}
