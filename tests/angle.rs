use glam::f32::{deg, rad, Angle};
use std::f32::consts;

#[test]
fn test_angle() {
    let a = Angle::from_radians(consts::PI);
    let b = Angle::from_degrees(90.0);
    assert_eq!(a.as_radians(), consts::PI);
    assert_eq!(a.as_degrees(), consts::PI.to_degrees());
    assert_eq!(b.as_degrees(), 90.0);
    assert_eq!(b.as_radians(), 90.0_f32.to_radians());
    assert_eq!(a.sin_cos(), consts::PI.sin_cos());
    assert_eq!(a, rad(consts::PI));
    assert_eq!(b, deg(90.0));
}
