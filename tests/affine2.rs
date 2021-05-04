#[cfg(feature = "transform-types")]
#[macro_use]
mod support;

#[cfg(feature = "transform-types")]
macro_rules! impl_affine2_tests {
    ($t:ident, $affine2:ident, $vec2:ident) => {
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_affine2_identity() {
            assert_eq!($affine2::IDENTITY, $affine2::IDENTITY * $affine2::IDENTITY);
            assert_eq!($affine2::IDENTITY, $affine2::default());
        }

        #[test]
        fn test_affine2_zero() {
            assert_eq!(
                $affine2::ZERO.transform_point2($vec2::new(1., 2.)),
                $vec2::ZERO
            );
        }

        #[test]
        fn test_affine2_translation() {
            let translate = $affine2::from_translation($vec2::new(1.0, 2.0));
            assert_eq!(translate.translation, $vec2::new(1.0, 2.0).into());
            assert_eq!(
                translate.transform_point2($vec2::new(2.0, 3.0)),
                $vec2::new(3.0, 5.0),
            );
        }

        #[test]
        fn test_affine2_mul() {
            let m = $affine2::from_angle(deg(90.0));
            let result3 = m.transform_vector2($vec2::Y);
            assert_approx_eq!($vec2::new(-1.0, 0.0), result3);

            let m = $affine2::from_scale_angle_translation(
                $vec2::new(0.5, 1.5),
                deg(90.0),
                $vec2::new(1.0, 2.0),
            );
            let result3 = m.transform_vector2($vec2::Y);
            assert_approx_eq!($vec2::new(-1.5, 0.0), result3, 1.0e-6);

            let result3 = m.transform_point2($vec2::Y);
            assert_approx_eq!($vec2::new(-0.5, 2.0), result3, 1.0e-6);
        }

        #[test]
        fn test_from_scale() {
            let m = $affine2::from_scale($vec2::new(2.0, 4.0));
            assert_approx_eq!(
                m.transform_point2($vec2::new(1.0, 1.0)),
                $vec2::new(2.0, 4.0)
            );
        }

        #[test]
        fn test_affine2_inverse() {
            let inv = $affine2::IDENTITY.inverse();
            assert_approx_eq!($affine2::IDENTITY, inv);

            let rot = $affine2::from_angle(deg(90.0));
            let rot_inv = rot.inverse();
            assert_approx_eq!($affine2::IDENTITY, rot * rot_inv);
            assert_approx_eq!($affine2::IDENTITY, rot_inv * rot);

            let trans = $affine2::from_translation($vec2::new(1.0, 2.0));
            let trans_inv = trans.inverse();
            assert_approx_eq!($affine2::IDENTITY, trans * trans_inv);
            assert_approx_eq!($affine2::IDENTITY, trans_inv * trans);

            let scale = $affine2::from_scale($vec2::new(4.0, 5.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($affine2::IDENTITY, scale * scale_inv);
            assert_approx_eq!($affine2::IDENTITY, scale_inv * scale);

            let m = scale * rot * trans;
            let m_inv = m.inverse();
            assert_approx_eq!($affine2::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine2::IDENTITY, m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rot_inv * scale_inv, 1.0e-6);

            // Make sure we can invert a shear matrix:
            let m = $affine2::from_angle(0.5)
                * $affine2::from_scale($vec2::new(1.0, 0.5))
                * $affine2::from_angle(-0.5);
            let m_inv = m.inverse();
            assert_approx_eq!($affine2::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine2::IDENTITY, m_inv * m, 1.0e-5);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let ident = $affine2::IDENTITY;
            assert_eq!(
                vec![ident, ident].iter().product::<$affine2>(),
                ident * ident
            );
        }

        #[test]
        fn test_affine2_is_finite() {
            assert!($affine2::from_scale($vec2::new(1.0, 1.0)).is_finite());
            assert!($affine2::from_scale($vec2::new(0.0, 1.0)).is_finite());
            assert!(!$affine2::from_scale($vec2::new(1.0, NAN)).is_finite());
            assert!(!$affine2::from_scale($vec2::new(1.0, NEG_INFINITY)).is_finite());
        }
    };
}

#[cfg(feature = "transform-types")]
mod affine2 {
    use super::support::deg;
    use glam::{Affine2, Vec2};

    #[test]
    fn test_align() {
        use std::mem;
        // TODO: With SSE2 Mat2/Affine2 is 16 byte aligned. Should this be true when no SSE2?
        if cfg!(all(target_feature = "sse2", not(feature = "scalar-math"))) {
            assert_eq!(32, mem::size_of::<Affine2>());
            assert_eq!(16, mem::align_of::<Affine2>());
        } else {
            assert_eq!(24, mem::size_of::<Affine2>());
            assert_eq!(4, mem::align_of::<Affine2>());
        }
    }

    impl_affine2_tests!(f32, Affine2, Vec2);
}

#[cfg(feature = "transform-types")]
mod daffine2 {
    use super::support::deg;
    use glam::{DAffine2, DVec2};

    #[test]
    fn test_align() {
        use std::mem;
        assert_eq!(48, mem::size_of::<DAffine2>());
        assert_eq!(8, mem::align_of::<DAffine2>());
    }

    impl_affine2_tests!(f64, DAffine2, DVec2);
}
