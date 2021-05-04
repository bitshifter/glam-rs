#[cfg(feature = "transform-types")]
#[macro_use]
mod support;

#[cfg(feature = "transform-types")]
mod transform {
    use glam::*;

    #[test]
    fn test_identity() {
        let tr = Isometry3A::IDENTITY;
        assert_eq!(tr.rotation, Quat::IDENTITY);
        assert_eq!(tr.translation, Vec3A::ZERO);

        let srt = Transform3A::IDENTITY;
        assert_eq!(srt.scale, Vec3A::ONE);
        assert_eq!(srt.rotation, Quat::IDENTITY);
        assert_eq!(srt.translation, Vec3A::ZERO);

        assert_eq!(srt, tr.into());

        assert_eq!(Isometry3A::IDENTITY, Isometry3A::default());
        assert_eq!(Transform3A::IDENTITY, Transform3A::default());
    }

    #[test]
    fn test_new() {
        let t = Vec3::new(1.0, 2.0, 3.0);
        let r = Quat::from_rotation_y(90.0_f32.to_radians());
        let s = Vec3::new(-1.0, -2.0, -3.0);

        let tr = Isometry3A::from_rotation_translation(r, t);
        assert_eq!(tr.rotation, r);
        assert_eq!(tr.translation, t.into());

        let srt = Transform3A::from_scale_rotation_translation(s, r, t);
        assert_eq!(srt.scale, s.into());
        assert_eq!(srt.rotation, r);
        assert_eq!(srt.translation, t.into());

        assert_eq!(tr, tr);
        assert_eq!(srt, srt);
        assert_eq!(srt, Transform3A::from_scale_isometry(s, &tr));
    }

    #[test]
    fn test_mul() {
        let tr = Isometry3A::from_rotation_translation(
            Quat::from_rotation_z(-90.0_f32.to_radians()),
            Vec3::X,
        );
        let v0 = Vec3A::Y;
        let v1 = tr.transform_point3a(v0);
        assert_approx_eq!(v1, Vec3A::X * 2.0);
        assert_approx_eq!(v1, tr.transform_point3a(v0));
        let inv_tr = tr.inverse();
        let v2 = inv_tr.transform_point3a(v1);
        assert_approx_eq!(v0, v2);

        assert_eq!(tr * Isometry3A::IDENTITY, tr);
        assert_approx_eq!(tr * inv_tr, Isometry3A::IDENTITY);

        assert_eq!(tr * Transform3A::IDENTITY, Transform3A::from(tr));
        assert_eq!(Transform3A::IDENTITY * tr, Transform3A::from(tr));

        let s = Vec3::splat(2.0);
        let r = Quat::from_rotation_y(180.0_f32.to_radians());
        let t = -Vec3::Y;
        let srt = Transform3A::from_scale_rotation_translation(s, r, t);
        let v0 = Vec3A::X;
        let v1 = srt.transform_point3a(v0);
        assert_approx_eq!(v1, (r * (v0 * Vec3A::from(s))) + Vec3A::from(t));
        assert_approx_eq!(v1, srt.transform_point3a(v0));
        let inv_srt = srt.inverse();
        let v2 = inv_srt.transform_point3a(v1);
        assert_approx_eq!(v0, v2);

        assert_eq!(srt * Transform3A::IDENTITY, srt);
        assert_eq!(srt * inv_srt, Transform3A::IDENTITY);

        // negative scale mul test
        let s = Vec3::splat(-2.0);
        let srt = Transform3A::from_scale_rotation_translation(s, r, t);
        let inv_srt = srt.inverse();
        assert_eq!(srt * inv_srt, Transform3A::IDENTITY);
    }
}
