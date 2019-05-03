use approx::assert_ulps_eq;
use glam::f32::*;

#[test]
fn test_transform_identity() {
    let tr = TransformRT::identity();
    assert_eq!(tr.rotation, Quat::identity());
    assert_eq!(tr.translation, Vec3::zero());

    let trs = TransformSRT::identity();
    assert_eq!(trs.scale, Vec3::one());
    assert_eq!(trs.rotation, Quat::identity());
    assert_eq!(trs.translation, Vec3::zero());

    assert_eq!(trs, tr.into());
}

#[test]
fn test_transform_new() {
    let t = Vec3::new(1.0, 2.0, 3.0);
    let r = Quat::from_rotation_y(deg(90.0));
    let s = Vec3::new(-1.0, -2.0, -3.0);

    let tr = TransformRT::new(r, t);
    assert_eq!(tr.rotation, r);
    assert_eq!(tr.translation, t);

    let trs = TransformSRT::new(s, r, t);
    assert_eq!(trs.scale, s);
    assert_eq!(trs.rotation, r);
    assert_eq!(trs.translation, t);

    assert_eq!(tr, tr);
    assert_eq!(trs, trs);
    assert_eq!(trs, TransformSRT::from_transform_rt(s, &tr));
}

#[test]
fn test_transform_mul() {
    let tr = TransformRT::new(Quat::from_rotation_z(deg(-90.0)), Vec3::unit_x());
    let v0 = Vec3::unit_y();
    let v1 = v0* tr;
    assert_ulps_eq!(v1, Vec3::unit_x() * 2.0);
    let inv_tr = tr.inverse();
    let v2 = v1 * inv_tr;
    assert_ulps_eq!(v0, v2);

    assert_eq!(tr * TransformRT::identity(), tr);
    assert_eq!(tr * inv_tr, TransformRT::identity());

    let s = Vec3::splat(2.0);
    let r = Quat::from_rotation_y(deg(180.0));
    let t = -Vec3::unit_y();
    let trs = TransformSRT::new(s, r, t);
    let v0 = Vec3::unit_x();
    let v1 = v0 * trs;
    assert_ulps_eq!(v1, ((v0 * s) * r) + t);
    let inv_trs = trs.inverse();
    let v2 = v1 * inv_trs;
    assert_ulps_eq!(v0, v2);
    
    assert_eq!(trs * TransformSRT::identity(), trs);
    assert_eq!(trs * inv_trs, TransformSRT::identity());
}
