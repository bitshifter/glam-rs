#[cfg(feature = "transform-types")]
mod transform {
    use approx::assert_ulps_eq;
    use glam::f32::*;

    #[test]
    fn test_identity() {
        let tr = TransformRT::identity();
        assert_eq!(tr.rotation, Quat::identity());
        assert_eq!(tr.translation, Vec3::zero());

        let srt = TransformSRT::identity();
        assert_eq!(srt.scale, Vec3::one());
        assert_eq!(srt.rotation, Quat::identity());
        assert_eq!(srt.translation, Vec3::zero());

        assert_eq!(srt, tr.into());
    }

    #[test]
    fn test_new() {
        let t = Vec3::new(1.0, 2.0, 3.0);
        let r = Quat::from_rotation_y(deg(90.0));
        let s = Vec3::new(-1.0, -2.0, -3.0);

        let tr = TransformRT::new(r, t);
        assert_eq!(tr.rotation, r);
        assert_eq!(tr.translation, t);

        let srt = TransformSRT::new(s, r, t);
        assert_eq!(srt.scale, s);
        assert_eq!(srt.rotation, r);
        assert_eq!(srt.translation, t);

        assert_eq!(tr, tr);
        assert_eq!(srt, srt);
        assert_eq!(srt, TransformSRT::from_transform_rt(s, &tr));
    }

    #[test]
    fn test_mul() {
        let tr = TransformRT::new(Quat::from_rotation_z(deg(-90.0)), Vec3::unit_x());
        let v0 = Vec3::unit_y();
        let v1 = v0 * tr;
        assert_ulps_eq!(v1, Vec3::unit_x() * 2.0);
        assert_ulps_eq!(v1, v0 * &tr);
        let inv_tr = tr.inverse();
        let v2 = v1 * inv_tr;
        assert_ulps_eq!(v0, v2);

        assert_eq!(tr * TransformRT::identity(), tr);
        assert_eq!(tr * inv_tr, TransformRT::identity());

        assert_eq!(tr * TransformSRT::identity(), TransformSRT::from(tr));
        assert_eq!(TransformSRT::identity() * tr, TransformSRT::from(tr));
        assert_eq!(tr * &TransformSRT::identity(), TransformSRT::from(tr));
        assert_eq!(TransformSRT::identity() * &tr, TransformSRT::from(tr));

        let s = Vec3::splat(2.0);
        let r = Quat::from_rotation_y(deg(180.0));
        let t = -Vec3::unit_y();
        let srt = TransformSRT::new(s, r, t);
        let v0 = Vec3::unit_x();
        let v1 = v0 * srt;
        assert_ulps_eq!(v1, ((v0 * s) * r) + t);
        assert_ulps_eq!(v1, v0 * &srt);
        let inv_srt = srt.inverse();
        let v2 = v1 * inv_srt;
        assert_ulps_eq!(v0, v2);

        assert_eq!(srt * TransformSRT::identity(), srt);
        assert_eq!(srt * inv_srt, TransformSRT::identity());

        // negative scale mul test
        let s = Vec3::splat(-2.0);
        let srt = TransformSRT::new(s, r, t);
        let inv_srt = srt.inverse();
        assert_eq!(srt * inv_srt, TransformSRT::identity());
    }
}
