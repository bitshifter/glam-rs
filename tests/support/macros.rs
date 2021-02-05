#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        #[allow(unused_imports)]
        use crate::support::FloatCompare;
        let eps = core::f32::EPSILON;
        let (a, b) = (&$a, &$b);
        assert!(
            a.approx_eq(b, eps),
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            a.abs_diff(b)
        );
    }};
    ($a:expr, $b:expr, $eps:expr) => {{
        use crate::support::FloatCompare;
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            a.approx_eq(b, $eps),
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            a.abs_diff(b)
        );
    }};
}

/// Test vector normalization for float vector
#[macro_export]
macro_rules! impl_vec_float_normalize_tests {
    ($t:ident, $vec:ident) => {
        use core::$t::MIN_POSITIVE;

        #[test]
        fn test_normalize() {
            assert_eq!((-42.0 * $vec::X).normalize(), -$vec::X);

            // We expect not to be able to normalize small numbers:
            assert!(!$vec::default().normalize().is_finite());
            assert!(!(MIN_POSITIVE * $vec::X).normalize().is_finite());

            // We expect not to be able to normalize non-finite values:
            assert!(!(INFINITY * $vec::X).normalize().is_finite());
            assert!(!(NAN * $vec::X).normalize().is_finite());
        }

        #[test]
        fn test_normalize_or_zero() {
            assert_eq!((-42.0 * $vec::X).normalize(), -$vec::X);

            // We expect `normalize_or_zero` to return zero when inputs are very small:
            assert_eq!($vec::default().normalize_or_zero(), $vec::default());
            assert_eq!(
                (MIN_POSITIVE * $vec::X).normalize_or_zero(),
                $vec::default()
            );

            // We expect `normalize_or_zero` to return zero when inputs are non-finite:
            assert_eq!((INFINITY * $vec::X).normalize_or_zero(), $vec::default());
            assert_eq!((NAN * $vec::X).normalize_or_zero(), $vec::default());
        }
    };
}
