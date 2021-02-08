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
        use core::$t::MAX;
        use core::$t::MIN_POSITIVE;

        #[test]
        fn test_normalize() {
            let x = $vec::X;

            assert_eq!((-42.0 * x).normalize(), -x);
            assert_eq!((MAX.sqrt() * x).normalize(), x);
            // assert_eq!((MAX * x).normalize(), x); // normalize fails for huge vectors and returns zero

            // We expect not to be able to normalize small numbers:
            assert!(!$vec::ZERO.normalize().is_finite());
            assert!(!(MIN_POSITIVE * x).normalize().is_finite());

            // We expect not to be able to normalize non-finite vectors:
            assert!(!(INFINITY * x).normalize().is_finite());
            assert!(!(NAN * x).normalize().is_finite());
        }

        #[test]
        fn test_normalize_or_zero() {
            let x = $vec::X;
            let y = $vec::Y;

            assert_eq!((-42.0 * x).normalize(), -x);
            assert_eq!((MAX.sqrt() * x).normalize_or_zero(), x);

            // We expect `normalize_or_zero` to return zero when inputs are very small:
            assert_eq!($vec::ZERO.normalize_or_zero(), $vec::ZERO);
            assert_eq!((MIN_POSITIVE * x).normalize_or_zero(), $vec::ZERO);

            // We expect `normalize_or_zero` to return zero when inputs are non-finite:
            assert_eq!((INFINITY * x).normalize_or_zero(), $vec::ZERO);
            assert_eq!((NAN * x).normalize_or_zero(), $vec::ZERO);

            // We expect `normalize_or_zero` to return zero when inputs are very large:
            assert_eq!((MAX * x).normalize_or_zero(), $vec::ZERO);
            assert_eq!((MAX * (x + y)).normalize_or_zero(), $vec::ZERO);
        }
    };
}
