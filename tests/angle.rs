use approx::assert_ulps_eq;
use glam::f32::{deg, rad, Angle};
#[cfg(feature = "rand")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "rand")]
use rand_xoshiro::Xoshiro256Plus;
use std::f32::consts;

#[test]
fn test_angle() {
    assert_eq!(Angle::from_radians(0.0), Angle::default());

    let a = Angle::from_radians(consts::PI);
    let b = Angle::from_degrees(90.0);
    assert_eq!(a.radians(), consts::PI);
    assert_eq!(a.to_degrees(), consts::PI.to_degrees());
    assert_eq!(b.to_degrees(), 90.0);
    assert_eq!(b.radians(), 90.0_f32.to_radians());
    assert_eq!(a.sin_cos(), consts::PI.sin_cos());
    assert_eq!(a, rad(consts::PI));
    assert_eq!(b, deg(90.0));

    assert_ulps_eq!(rad(0.0f32.acos()), Angle::acos(0.0));
    assert_ulps_eq!(rad(1.0f32.acos()), Angle::acos(1.0));
    assert_ulps_eq!(rad((-1.0f32).acos()), Angle::acos(-1.0));

    // Angle::acos limits to [-1.0, 1.0]
    assert_ulps_eq!(rad(1.0f32.acos()), Angle::acos(2.0));
    assert_ulps_eq!(rad((-1.0f32).acos()), Angle::acos(-2.0));

    assert_eq!(rad(consts::PI * 2.0), rad(consts::PI) * 2.0);
    assert_eq!(rad(2.0 * consts::PI), 2.0 * rad(consts::PI));

    assert_eq!(rad(consts::PI / 2.0), rad(consts::PI) / 2.0);

    let mut a = rad(consts::PI);
    a *= 2.0;
    assert_eq!(rad(consts::PI * 2.0), a);
    a /= 2.0;
    assert_eq!(rad(consts::PI), a);
    assert_eq!(rad(consts::PI * 2.0), a + a);
    assert_eq!(rad(0.0), a - a);
    a += a;
    assert_eq!(rad(consts::PI * 2.0), a);
    a -= a;
    assert_eq!(rad(0.0), a);

    assert_eq!(rad(-consts::PI), -rad(consts::PI));
}

#[cfg(feature = "rand")]
#[test]
fn test_angle_rnd() {
    let mut rng = Xoshiro256Plus::seed_from_u64(0);
    let a: Angle = rng.gen();
    assert!(a >= rad(0.0));
    assert!(a < rad(std::f32::consts::PI * 2.0));
}
