#[macro_use]
mod support;

macro_rules! impl_float_tests {
    ($t:ident) => {
        glam_test!(test_lerp, {
            let a = 0.;
            let b = 10.;
            assert_eq!($t::lerp(a, b, 0.), a);
            assert_eq!($t::lerp(a, b, 0.5), 5.);
            assert_eq!($t::lerp(a, b, 1.), b);
            assert_eq!($t::lerp(a, a, 0.), a);
            assert_eq!($t::lerp(a, a, 1.), a);
        });

        glam_test!(test_lerp_big_difference, {
            // `lerp` uses the form `self * (1 - s) + rhs * s`, which guarantees `rhs` at `s == 1`
            // even when the inputs differ greatly in magnitude.
            let a = -16.0e30;
            let b = 16.0;
            assert_eq!($t::lerp(a, b, 0.), a);
            assert_eq!($t::lerp(a, b, 1.), b);
        });

        glam_test!(test_inverse_lerp, {
            let a = 0.;
            let b = 10.;
            assert_eq!($t::inverse_lerp(a, b, 0.), 0.);
            assert_eq!($t::inverse_lerp(a, b, 5.), 0.5);
            assert_eq!($t::inverse_lerp(a, b, 10.), 1.);
            assert_eq!($t::inverse_lerp(a, b, 15.), 1.5);
            assert!($t::inverse_lerp(a, a, 0.).is_nan());
            assert!($t::inverse_lerp(a, a, 1.).is_infinite());
        });

        glam_test!(test_smoothstep, {
            assert_eq!((-1. as $t).smoothstep(0., 1.), 0.);
            assert_eq!((0. as $t).smoothstep(0., 1.), 0.);
            assert_eq!((0.5 as $t).smoothstep(0., 1.), 0.5);
            assert_eq!((1. as $t).smoothstep(0., 1.), 1.);
            assert_eq!((2. as $t).smoothstep(0., 1.), 1.);
        });

        glam_test!(test_remap, {
            assert_eq!($t::remap(0., 0., 2., 0., 20.), 0.);
            assert_eq!($t::remap(1., 0., 2., 0., 20.), 10.);
            assert_eq!($t::remap(2., 0., 2., 0., 20.), 20.);
            assert_eq!($t::remap(-5., -10., 30., 60., 20.), 55.);
            // Pins the precise `lerp`: a monotone form would return 0 here.
            assert_eq!(20., $t::remap(2., 0., 2., -1e30, 20.));
            // When one of the input ranges is degenerate `inverse_lerp` is infinite or NaN, which
            // `lerp` (using the precise form) propagates as NaN.
            assert!($t::remap(0., 0., 0., 0., 1.).is_nan());
            assert!($t::remap(1., 0., 0., 0., 1.).is_nan());
        });

        glam_test!(test_fract_gl, {
            assert_approx_eq!(1.35.fract_gl(), 0.35);
            assert_approx_eq!((-1.5).fract_gl(), 0.5);
            assert_approx_eq!((-2000000.123).fract_gl(), 0.877, 0.002);
            assert_approx_eq!(1000000.123.fract_gl(), 0.123, 0.002);
        });

        glam_test!(test_move_towards, {
            assert_eq!($t::move_towards(-1., 1., 0.), -1.);
            assert_eq!($t::move_towards(-1., 1., 1.), 0.);
            assert_eq!($t::move_towards(-1., 1., 2.), 1.);
            assert_eq!($t::move_towards(-1., 1., 3.), 1.);
            assert_eq!($t::move_towards(1., -1., 1.), 0.);
            assert_eq!($t::move_towards(1., -1., 3.), -1.);
            assert_eq!($t::move_towards(1., 1., 1.), 1.);
        });
    };
}

mod float32 {
    use glam::FloatExt;

    impl_float_tests!(f32);
}

#[cfg(feature = "f64")]
mod float64 {
    use glam::FloatExt;

    impl_float_tests!(f64);
}
