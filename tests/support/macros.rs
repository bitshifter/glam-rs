#![allow(unused_macros)]

#[macro_export]
macro_rules! glam_test {
    ($name:ident, $block:block) => {
        #[cfg_attr(not(target_family = "wasm"), test)]
        #[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
        fn $name() {
            $block
        }
    };
}

#[macro_export]
macro_rules! should_panic {
    ($block:block) => {{
        #[cfg(all(feature = "std", panic = "unwind"))]
        assert!(std::panic::catch_unwind(|| $block).is_err());
    }};
}

#[macro_export]
macro_rules! should_glam_assert {
    ($block:block) => {{
        #[cfg(any(feature = "glam-assert", feature = "debug-glam-assert"))]
        should_panic!($block);
    }};
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        #[allow(unused_imports)]
        use $crate::support::FloatCompare;
        let eps = f32::EPSILON;
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
        use $crate::support::FloatCompare;
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
    ($a:expr, $b:expr, $eps:expr, $ctx:expr) => {{
        use $crate::support::FloatCompare;
        let (a, b) = (&$a, &$b);
        let eps = $eps;
        assert!(
            a.approx_eq(b, $eps),
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`), \
             additional context: {}",
            *a,
            *b,
            eps,
            a.abs_diff(b),
            $ctx
        );
    }};
}

/// Test vector normalization for float vector
#[macro_export]
macro_rules! impl_vec_float_normalize_tests {
    ($t:ident, $vec:ident) => {
        /// Works for vec2, vec3, vec4
        fn from_x_y(x: $t, y: $t) -> $vec {
            let mut v = $vec::ZERO;
            v.x = x;
            v.y = y;
            v
        }

        glam_test!(test_normalize, {
            assert_eq!(from_x_y(-42.0, 0.0).normalize(), from_x_y(-1.0, 0.0));
            assert_eq!(
                from_x_y($t::MAX.sqrt(), 0.0).normalize(),
                from_x_y(1.0, 0.0)
            );
            // assert_eq!(from_x_y($t::MAX, 0.0).normalize(), from_x_y(1.0, 0.0)); // normalize fails for huge vectors and returns zero

            // We expect not to be able to normalize small numbers:
            should_glam_assert!({ from_x_y(0.0, 0.0).normalize() });
            should_glam_assert!({ from_x_y($t::MIN_POSITIVE, 0.0).normalize() });

            // We expect not to be able to normalize non-finite vectors:
            should_glam_assert!({ from_x_y($t::INFINITY, 0.0).normalize() });
            should_glam_assert!({ from_x_y($t::NAN, 0.0).normalize() });
        });

        #[cfg(not(any(feature = "debug-glam-assert", feature = "glam-assert")))]
        glam_test!(test_normalize_no_glam_assert, {
            // We expect not to be able to normalize small numbers:
            assert!(!from_x_y(0.0, 0.0).normalize().is_finite());
            assert!(!from_x_y($t::MIN_POSITIVE, 0.0).normalize().is_finite());

            // We expect not to be able to normalize non-finite vectors:
            assert!(!from_x_y($t::INFINITY, 0.0).normalize().is_finite());
            assert!(!from_x_y($t::NAN, 0.0).normalize().is_finite());
        });

        glam_test!(test_try_normalize, {
            assert_eq!(
                from_x_y(-42.0, 0.0).try_normalize(),
                Some(from_x_y(-1.0, 0.0))
            );
            assert_eq!(
                from_x_y($t::MAX.sqrt(), 0.0).try_normalize(),
                Some(from_x_y(1.0, 0.0))
            );

            // We expect `try_normalize` to return None when inputs are very small:
            assert_eq!(from_x_y(0.0, 0.0).try_normalize(), None);
            assert_eq!(from_x_y($t::MIN_POSITIVE, 0.0).try_normalize(), None);

            // We expect `try_normalize` to return None when inputs are non-finite:
            assert_eq!(from_x_y($t::INFINITY, 0.0).try_normalize(), None);
            assert_eq!(from_x_y($t::NAN, 0.0).try_normalize(), None);

            // We expect `try_normalize` to return None when inputs are very large:
            assert_eq!(from_x_y($t::MAX, 0.0).try_normalize(), None);
            assert_eq!(from_x_y($t::MAX, $t::MAX).try_normalize(), None);
        });

        glam_test!(test_normalize_or, {
            assert_eq!(
                from_x_y(-42.0, 0.0).normalize_or($vec::Y),
                from_x_y(-1.0, 0.0)
            );
            assert_eq!(
                from_x_y($t::MAX.sqrt(), 0.0).normalize_or($vec::Y),
                from_x_y(1.0, 0.0)
            );

            // We expect `normalize_or` to return the fallback value when inputs are very small:
            assert_eq!(from_x_y(0.0, 0.0).normalize_or($vec::Y), $vec::Y);
            assert_eq!(
                from_x_y($t::MIN_POSITIVE, 0.0).normalize_or($vec::Y),
                $vec::Y
            );

            // We expect `normalize` to return zero when inputs are non-finite:
            assert_eq!(from_x_y($t::INFINITY, 0.0).normalize_or($vec::Y), $vec::Y);
            assert_eq!(from_x_y($t::NAN, 0.0).normalize_or($vec::Y), $vec::Y);

            // We expect `normalize` to return zero when inputs are very large:
            assert_eq!(from_x_y($t::MAX, 0.0).normalize_or($vec::Y), $vec::Y);
            assert_eq!(from_x_y($t::MAX, $t::MAX).normalize_or($vec::Y), $vec::Y);
        });

        glam_test!(test_normalize_or_zero, {
            assert_eq!(
                from_x_y(-42.0, 0.0).normalize_or_zero(),
                from_x_y(-1.0, 0.0)
            );
            assert_eq!(
                from_x_y($t::MAX.sqrt(), 0.0).normalize_or_zero(),
                from_x_y(1.0, 0.0)
            );

            // We expect `normalize_or_zero` to return zero when inputs are very small:
            assert_eq!(from_x_y(0.0, 0.0).normalize_or_zero(), $vec::ZERO);
            assert_eq!(
                from_x_y($t::MIN_POSITIVE, 0.0).normalize_or_zero(),
                $vec::ZERO
            );

            // We expect `normalize_or_zero` to return zero when inputs are non-finite:
            assert_eq!(from_x_y($t::INFINITY, 0.0).normalize_or_zero(), $vec::ZERO);
            assert_eq!(from_x_y($t::NAN, 0.0).normalize_or_zero(), $vec::ZERO);

            // We expect `normalize_or_zero` to return zero when inputs are very large:
            assert_eq!(from_x_y($t::MAX, 0.0).normalize_or_zero(), $vec::ZERO);
            assert_eq!(from_x_y($t::MAX, $t::MAX).normalize_or_zero(), $vec::ZERO);
        });

        glam_test!(test_normalize_and_length, {
            assert_eq!(
                from_x_y(-42.0, 0.0).normalize_and_length(),
                (from_x_y(-1.0, 0.0), 42.0)
            );
            assert_eq!(
                from_x_y($t::MAX.sqrt(), 0.0).normalize_and_length(),
                (from_x_y(1.0, 0.0), $t::MAX.sqrt())
            );

            // We expect `normalize_and_length` to return zero length when inputs are very small:
            assert_eq!(from_x_y(0.0, 0.0).normalize_and_length(), ($vec::X, 0.0));
            assert_eq!(
                from_x_y($t::MIN_POSITIVE, 0.0).normalize_and_length(),
                ($vec::X, 0.0)
            );

            // We expect `normalize_and_length` to return zero length when inputs are non-finite:
            assert_eq!(
                from_x_y($t::INFINITY, 0.0).normalize_and_length(),
                ($vec::X, 0.0)
            );
            assert_eq!(
                from_x_y($t::NAN, 0.0).normalize_and_length(),
                ($vec::X, 0.0)
            );

            // We expect `normalize_and_length` to return zero length when inputs are very large:
            assert_eq!(
                from_x_y($t::MAX, 0.0).normalize_and_length(),
                ($vec::X, 0.0)
            );
            assert_eq!(from_x_y($t::MAX, $t::MAX).normalize_or_zero(), $vec::ZERO);
        });
    };
}

/// Useful test vectors
#[macro_export]
macro_rules! vec3_float_test_vectors {
    ($vec3:ident) => {
        [
            $vec3::X,
            $vec3::Y,
            $vec3::Z,
            -$vec3::X,
            -$vec3::Y,
            -$vec3::Z,
            $vec3::new(1.0, 1e-3, 0.0),
            $vec3::new(1.0, 1e-4, 0.0),
            $vec3::new(1.0, 1e-5, 0.0),
            $vec3::new(1.0, 1e-6, 0.0),
            $vec3::new(1.0, 1e-7, 0.0),
            $vec3::new(1.0, 1e-14, 0.0),
            $vec3::new(1.0, 1e-15, 0.0),
            $vec3::new(1.0, 1e-16, 0.0),
            $vec3::new(0.1, 0.2, 0.3),
            $vec3::new(0.2, 0.3, 0.4),
            $vec3::new(4.0, -5.0, 6.0),
            $vec3::new(-2.0, 0.5, -1.0),
            // Pathological cases from <https://graphics.pixar.com/library/OrthonormalB/paper.pdf>:
            $vec3::new(0.00038527316, 0.00038460016, -0.99999988079),
            $vec3::new(-0.00019813581, -0.00008946839, -0.99999988079),
        ]
    };
}

#[macro_export]
macro_rules! vec2_float_test_vectors {
    ($vec2:ident) => {
        [
            $vec2::X,
            $vec2::Y,
            -$vec2::X,
            -$vec2::Y,
            $vec2::new(1.0, 1e-3),
            $vec2::new(1.0, 1e-4),
            $vec2::new(1.0, 1e-5),
            $vec2::new(1.0, 1e-6),
            $vec2::new(1.0, 1e-7),
            $vec2::new(1.0, 1e-14),
            $vec2::new(1.0, 1e-15),
            $vec2::new(1.0, 1e-16),
            $vec2::new(0.1, 0.2),
            $vec2::new(0.2, 0.3),
            $vec2::new(4.0, -5.0),
            $vec2::new(-2.0, 0.5),
            // Pathological cases from <https://graphics.pixar.com/library/OrthonormalB/paper.pdf>:
            $vec2::new(0.00038527316, 0.00038460016),
            $vec2::new(-0.00019813581, -0.00008946839),
        ]
    };
}

/// Unified eq/hash test for vectors of any dimension
#[macro_export]
macro_rules! impl_vec_eq_hash_tests {
    ($test_name:ident, $t:ident, $new:ident, $($a_val:expr),+ ; $($c_val:expr),+) => {
        glam_test!($test_name, {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $new($(($a_val) as $t),+);
            let b = $new($(($a_val) as $t),+);
            let c = $new($(($c_val) as $t),+);

            let mut hasher = DefaultHasher::new();
            a.hash(&mut hasher);
            let a_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            let b_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            c.hash(&mut hasher);
            let c_hashed = hasher.finish();

            assert_eq!(a, b);
            assert_eq!(a_hashed, b_hashed);
            assert_ne!(a, c);
            assert_ne!(a_hashed, c_hashed);
        });
    };
}

/// Helper to generate nested for loops from variable names
#[macro_export]
macro_rules! impl_vec_nested_for {
    ($range:expr, $body:tt, $var:ident) => {
        for $var in $range $body
    };
    ($range:expr, $body:tt, $var:ident, $($rest:ident),+) => {
        for $var in $range {
            impl_vec_nested_for!($range, $body, $($rest),+)
        }
    };
}

/// Scalar shift op tests, generates all 8 shift_by_* modules at once.
#[macro_export]
macro_rules! impl_vec_scalar_shift_op_test {
    ($test_name:ident, $vec:ident, $t_min:literal, $t_max:literal, $($var:ident),+) => {
        mod shift_by_i8 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, i8, $($var),+);
        }
        mod shift_by_i16 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, i16, $($var),+);
        }
        mod shift_by_i32 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, i32, $($var),+);
        }
        mod shift_by_i64 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, i64, $($var),+);
        }
        mod shift_by_u8 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, u8, $($var),+);
        }
        mod shift_by_u16 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, u16, $($var),+);
        }
        mod shift_by_u32 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, u32, $($var),+);
        }
        mod shift_by_u64 {
            use glam::$vec;
            impl_vec_scalar_shift_op_test_inner!($test_name, $vec, $t_min, $t_max, u64, $($var),+);
        }
    };
}

/// The actual single `glam_test!` body for a scalar shift op test.
/// Takes just the rhs type; 0 and 2 are implied.
#[macro_export]
macro_rules! impl_vec_scalar_shift_op_test_inner {
    ($test_name:ident, $vec:ident, $t_min:literal, $t_max:literal,
     $rhs_ty:ty, $($var:ident),+) => {
        glam_test!($test_name, {
            impl_vec_nested_for!($t_min..$t_max, {
                for rhs in (0 as $rhs_ty)..(2 as $rhs_ty) {
                    let lhs = $vec::new($($var),+);
                    assert_eq!(lhs << rhs, $vec::new($($var << rhs),+));
                    assert_eq!(lhs >> rhs, $vec::new($($var >> rhs),+));

                    assert_eq!(&lhs << rhs, $vec::new($($var << rhs),+));
                    assert_eq!(&lhs >> rhs, $vec::new($($var >> rhs),+));

                    assert_eq!(lhs << &rhs, $vec::new($($var << rhs),+));
                    assert_eq!(lhs >> &rhs, $vec::new($($var >> rhs),+));

                    assert_eq!(&lhs << &rhs, $vec::new($($var << rhs),+));
                    assert_eq!(&lhs >> &rhs, $vec::new($($var >> rhs),+));

                    let mut a = lhs;
                    a <<= rhs;
                    assert_eq!(a, lhs << rhs);

                    let mut a = lhs;
                    a <<= &rhs;
                    assert_eq!(a, lhs << rhs);

                    let mut a = lhs;
                    a >>= rhs;
                    assert_eq!(a, lhs >> rhs);

                    let mut a = lhs;
                    a >>= &rhs;
                    assert_eq!(a, lhs >> rhs);
                }
            }, $($var),+)
        });
    };
}

/// Shared vec-vec shift op test (dimension parameterized by two variable lists)
#[macro_export]
macro_rules! impl_vec_shift_op_test {
    ($test_name:ident, $vec:ident, $rhs_ty:ident, $t_min:literal, $t_max:literal,
     $($var1:ident),+ ; $($var2:ident),+) => {
        glam_test!($test_name, {
            impl_vec_nested_for!($t_min..$t_max, {
                let lhs = $vec::new($($var1),+);
                impl_vec_nested_for!($t_min..$t_max, {
                    let rhs = $rhs_ty::new($($var2),+);
                    assert_eq!(lhs << rhs, $vec::new($($var1 << $var2),+));
                    assert_eq!(lhs >> rhs, $vec::new($($var1 >> $var2),+));

                    assert_eq!(&lhs << rhs, $vec::new($($var1 << $var2),+));
                    assert_eq!(&lhs >> rhs, $vec::new($($var1 >> $var2),+));

                    assert_eq!(lhs << &rhs, $vec::new($($var1 << $var2),+));
                    assert_eq!(lhs >> &rhs, $vec::new($($var1 >> $var2),+));

                    assert_eq!(&lhs << &rhs, $vec::new($($var1 << $var2),+));
                    assert_eq!(&lhs >> &rhs, $vec::new($($var1 >> $var2),+));
                }, $($var2),+)
            }, $($var1),+)
        });
    };
}

/// Shared scalar bit op test (dimension parameterized by variable list)
#[macro_export]
macro_rules! impl_vec_scalar_bit_op_tests {
    ($test_name:ident, $vec:ident, $t_min:literal, $t_max:literal, $($var:ident),+) => {
        glam_test!($test_name, {
            impl_vec_nested_for!($t_min..$t_max, {
                for rhs in $t_min..$t_max {
                    let lhs = $vec::new($($var),+);
                    assert_eq!(lhs & rhs, $vec::new($($var & rhs),+));
                    assert_eq!(lhs | rhs, $vec::new($($var | rhs),+));
                    assert_eq!(lhs ^ rhs, $vec::new($($var ^ rhs),+));

                    assert_eq!(&lhs & rhs, $vec::new($($var & rhs),+));
                    assert_eq!(&lhs | rhs, $vec::new($($var | rhs),+));
                    assert_eq!(&lhs ^ rhs, $vec::new($($var ^ rhs),+));

                    assert_eq!(lhs & &rhs, $vec::new($($var & rhs),+));
                    assert_eq!(lhs | &rhs, $vec::new($($var | rhs),+));
                    assert_eq!(lhs ^ &rhs, $vec::new($($var ^ rhs),+));

                    assert_eq!(&lhs & &rhs, $vec::new($($var & rhs),+));
                    assert_eq!(&lhs | &rhs, $vec::new($($var | rhs),+));
                    assert_eq!(&lhs ^ &rhs, $vec::new($($var ^ rhs),+));

                    let mut a = lhs;
                    a &= rhs;
                    assert_eq!(a, lhs & rhs);

                    let mut a = lhs;
                    a &= &rhs;
                    assert_eq!(a, lhs & rhs);

                    let mut a = lhs;
                    a |= rhs;
                    assert_eq!(a, lhs | rhs);

                    let mut a = lhs;
                    a |= &rhs;
                    assert_eq!(a, lhs | rhs);

                    let mut a = lhs;
                    a ^= rhs;
                    assert_eq!(a, lhs ^ rhs);

                    let mut a = lhs;
                    a ^= &rhs;
                    assert_eq!(a, lhs ^ rhs);
                }
            }, $($var),+)
        });
    };
}

/// Shared vec-vec bit op test (dimension parameterized by two variable lists)
#[macro_export]
macro_rules! impl_vec_bit_op_tests {
    ($test_name:ident, $vec:ident, $t_min:literal, $t_max:literal,
     $($var1:ident),+ ; $($var2:ident),+) => {
        glam_test!($test_name, {
            impl_vec_nested_for!($t_min..$t_max, {
                let lhs = $vec::new($($var1),+);
                assert_eq!(!lhs, $vec::new($(!$var1),+));
                assert_eq!(!&lhs, $vec::new($(!$var1),+));

                impl_vec_nested_for!($t_min..$t_max, {
                    let rhs = $vec::new($($var2),+);
                    assert_eq!(lhs & rhs, $vec::new($($var1 & $var2),+));
                    assert_eq!(lhs | rhs, $vec::new($($var1 | $var2),+));
                    assert_eq!(lhs ^ rhs, $vec::new($($var1 ^ $var2),+));

                    assert_eq!(&lhs & rhs, $vec::new($($var1 & $var2),+));
                    assert_eq!(&lhs | rhs, $vec::new($($var1 | $var2),+));
                    assert_eq!(&lhs ^ rhs, $vec::new($($var1 ^ $var2),+));

                    assert_eq!(lhs & &rhs, $vec::new($($var1 & $var2),+));
                    assert_eq!(lhs | &rhs, $vec::new($($var1 | $var2),+));
                    assert_eq!(lhs ^ &rhs, $vec::new($($var1 ^ $var2),+));

                    assert_eq!(&lhs & &rhs, $vec::new($($var1 & $var2),+));
                    assert_eq!(&lhs | &rhs, $vec::new($($var1 | $var2),+));
                    assert_eq!(&lhs ^ &rhs, $vec::new($($var1 ^ $var2),+));

                    let mut a = lhs;
                    a &= rhs;
                    assert_eq!(a, lhs & rhs);

                    let mut a = lhs;
                    a &= &rhs;
                    assert_eq!(a, lhs & rhs);

                    let mut a = lhs;
                    a |= rhs;
                    assert_eq!(a, lhs | rhs);

                    let mut a = lhs;
                    a |= &rhs;
                    assert_eq!(a, lhs | rhs);

                    let mut a = lhs;
                    a ^= rhs;
                    assert_eq!(a, lhs ^ rhs);

                    let mut a = lhs;
                    a ^= &rhs;
                    assert_eq!(a, lhs ^ rhs);
                }, $($var2),+)
            }, $($var1),+)
        });
    };
}

#[macro_export]
macro_rules! test_matrix_minor {
    ($n:expr, $minor:expr, $input:expr, $i:expr, $j:expr) => {
        let mut yy = 0;
        for y in 0..$n {
            if y != $j {
                let mut xx = 0;
                for x in 0..$n {
                    if x != $i {
                        assert_eq!($minor.col(xx)[yy], $input.col(x)[y]);
                        xx += 1;
                    }
                }
                yy += 1;
            }
        }
    };
}

/// try_from with MAX overflow error checks
macro_rules! impl_try_from_pair_with_max_error {
    ($feature:literal, $src:ident, $tgt:ident, $max:path, 2) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!($src::new(1, 2), $src::try_from($tgt::new(1, 2)).unwrap());
            assert!($src::try_from($tgt::new($max, 2)).is_err());
            assert!($src::try_from($tgt::new(1, $max)).is_err());
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, $max:path, 3) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3),
                $src::try_from($tgt::new(1, 2, 3)).unwrap()
            );
            assert!($src::try_from($tgt::new($max, 2, 3)).is_err());
            assert!($src::try_from($tgt::new(1, $max, 3)).is_err());
            assert!($src::try_from($tgt::new(1, 2, $max)).is_err());
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, $max:path, 4) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3, 4),
                $src::try_from($tgt::new(1, 2, 3, 4)).unwrap()
            );
            assert!($src::try_from($tgt::new($max, 2, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, $max, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, $max, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, 3, $max)).is_err());
        }
    };
}

/// from (infallible) test
macro_rules! impl_from_pair_infallible {
    ($feature:literal, $src:ident, $tgt:ident, 2) => {
        #[cfg(feature = $feature)]
        assert_eq!($src::new(1, 2), $src::from($tgt::new(1, 2)));
    };
    ($feature:literal, $src:ident, $tgt:ident, 3) => {
        #[cfg(feature = $feature)]
        assert_eq!($src::new(1, 2, 3), $src::from($tgt::new(1, 2, 3)));
    };
    ($feature:literal, $src:ident, $tgt:ident, 4) => {
        #[cfg(feature = $feature)]
        assert_eq!($src::new(1, 2, 3, 4), $src::from($tgt::new(1, 2, 3, 4)));
    };
}

/// try_from with negative value error checks
macro_rules! impl_try_from_pair_with_negative_error {
    ($feature:literal, $src:ident, $tgt:ident, 2) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!($src::new(1, 2), $src::try_from($tgt::new(1, 2)).unwrap());
            assert!($src::try_from($tgt::new(-1, 2)).is_err());
            assert!($src::try_from($tgt::new(1, -2)).is_err());
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, 3) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3),
                $src::try_from($tgt::new(1, 2, 3)).unwrap()
            );
            assert!($src::try_from($tgt::new(-1, 2, 3)).is_err());
            assert!($src::try_from($tgt::new(1, -2, 3)).is_err());
            assert!($src::try_from($tgt::new(1, 2, -3)).is_err());
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, 4) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3, 4),
                $src::try_from($tgt::new(1, 2, 3, 4)).unwrap()
            );
            assert!($src::try_from($tgt::new(-1, 2, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, -2, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, -3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, 3, -4)).is_err());
        }
    };
}

/// try_from without error tests - for same-width or narrower types
macro_rules! impl_try_from_pair_no_error {
    ($feature:literal, $src:ident, $tgt:ident, 2) => {
        #[cfg(feature = $feature)]
        assert_eq!($src::new(1, 2), $src::try_from($tgt::new(1, 2)).unwrap());
    };
    ($feature:literal, $src:ident, $tgt:ident, 3) => {
        #[cfg(feature = $feature)]
        assert_eq!(
            $src::new(1, 2, 3),
            $src::try_from($tgt::new(1, 2, 3)).unwrap()
        );
    };
    ($feature:literal, $src:ident, $tgt:ident, 4) => {
        #[cfg(feature = $feature)]
        assert_eq!(
            $src::new(1, 2, 3, 4),
            $src::try_from($tgt::new(1, 2, 3, 4)).unwrap()
        );
    };
}

/// try_from with both negative and MAX overflow error checks
macro_rules! impl_try_from_pair_with_negmax_error {
    ($feature:literal, $src:ident, $tgt:ident, $max:path, 2) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!($src::new(1, 2), $src::try_from($tgt::new(1, 2)).unwrap());
            assert!($src::try_from($tgt::new(-1, 2)).is_err());
            assert!($src::try_from($tgt::new(1, -2)).is_err());
            assert!($src::try_from($tgt::new($max, 2)).is_err());
            assert!($src::try_from($tgt::new(1, $max)).is_err());
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, $max:path, 3) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3),
                $src::try_from($tgt::new(1, 2, 3)).unwrap()
            );
            assert!($src::try_from($tgt::new(-1, 2, 3)).is_err());
            assert!($src::try_from($tgt::new(1, -2, 3)).is_err());
            assert!($src::try_from($tgt::new(1, 2, -3)).is_err());
            assert!($src::try_from($tgt::new($max, 2, 3)).is_err());
            assert!($src::try_from($tgt::new(1, $max, 3)).is_err());
            assert!($src::try_from($tgt::new(1, 2, $max)).is_err());
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, $max:path, 4) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3, 4),
                $src::try_from($tgt::new(1, 2, 3, 4)).unwrap()
            );
            assert!($src::try_from($tgt::new(-1, 2, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, -2, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, -3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, 3, -4)).is_err());
            assert!($src::try_from($tgt::new($max, 2, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, $max, 3, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, $max, 4)).is_err());
            assert!($src::try_from($tgt::new(1, 2, 3, $max)).is_err());
        }
    };
}

/// try_from with MAX overflow error checks, conditional on target platform.
/// Only expects overflow when `core::mem::size_of::<$cond_type>() > 4`.
/// This handles isize/usize which are 32-bit on wasm32 and 64-bit on most other targets.
macro_rules! impl_try_from_pair_with_sizeof_max_error {
    ($feature:literal, $src:ident, $tgt:ident, $cond_type:ty, $max:path, 2) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!($src::new(1, 2), $src::try_from($tgt::new(1, 2)).unwrap());
            if core::mem::size_of::<$cond_type>() > 4 {
                assert!($src::try_from($tgt::new($max, 2)).is_err());
                assert!($src::try_from($tgt::new(1, $max)).is_err());
            }
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, $cond_type:ty, $max:path, 3) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3),
                $src::try_from($tgt::new(1, 2, 3)).unwrap()
            );
            if core::mem::size_of::<$cond_type>() > 4 {
                assert!($src::try_from($tgt::new($max, 2, 3)).is_err());
                assert!($src::try_from($tgt::new(1, $max, 3)).is_err());
                assert!($src::try_from($tgt::new(1, 2, $max)).is_err());
            }
        }
    };
    ($feature:literal, $src:ident, $tgt:ident, $cond_type:ty, $max:path, 4) => {
        #[cfg(feature = $feature)]
        {
            assert_eq!(
                $src::new(1, 2, 3, 4),
                $src::try_from($tgt::new(1, 2, 3, 4)).unwrap()
            );
            if core::mem::size_of::<$cond_type>() > 4 {
                assert!($src::try_from($tgt::new($max, 2, 3, 4)).is_err());
                assert!($src::try_from($tgt::new(1, $max, 3, 4)).is_err());
                assert!($src::try_from($tgt::new(1, 2, $max, 4)).is_err());
                assert!($src::try_from($tgt::new(1, 2, 3, $max)).is_err());
            }
        }
    };
}

// Helper macros for wrapping and saturating arithmetic tests.
// The top-level `impl_vec_wrapping_saturating_tests!` dispatches by dim and mode.

macro_rules! impl_vec_wrapping_test {
    (2, $vec:ident, $t:ty) => {
        glam_test!(test_wrapping_add, {
            assert_eq!(
                $vec::new(<$t>::MAX, 5).wrapping_add($vec::new(1, 3)),
                $vec::new(<$t>::MIN, 8),
            );
        });
        glam_test!(test_wrapping_sub, {
            assert_eq!(
                $vec::new(<$t>::MAX, 5).wrapping_sub($vec::new(1, 3)),
                $vec::new(<$t>::MAX - 1, 2)
            );
        });
        glam_test!(test_wrapping_mul, {
            assert_eq!(
                $vec::new(<$t>::MAX, 5).wrapping_mul($vec::new(3, 3)),
                $vec::new(<$t>::MAX.wrapping_mul(3), 15)
            );
        });
        glam_test!(test_wrapping_div, {
            assert_eq!(
                $vec::new(<$t>::MAX, 5).wrapping_div($vec::new(3, 3)),
                $vec::new(<$t>::MAX / 3, 1)
            );
        });
    };
    (3, $vec:ident, $t:ty) => {
        glam_test!(test_wrapping_add, {
            assert_eq!(
                $vec::new(<$t>::MAX, 0, 5).wrapping_add($vec::new(1, 2, 3)),
                $vec::new(<$t>::MIN, 2, 8),
            );
        });
        glam_test!(test_wrapping_sub, {
            assert_eq!(
                $vec::new(<$t>::MAX, 4, 5).wrapping_sub($vec::new(1, 2, 3)),
                $vec::new(<$t>::MAX - 1, 2, 2)
            );
        });
        glam_test!(test_wrapping_mul, {
            assert_eq!(
                $vec::new(<$t>::MAX, 2, 5).wrapping_mul($vec::new(3, 2, 3)),
                $vec::new(<$t>::MAX.wrapping_mul(3), 4, 15)
            );
        });
        glam_test!(test_wrapping_div, {
            assert_eq!(
                $vec::new(<$t>::MAX, 4, 5).wrapping_div($vec::new(3, 2, 3)),
                $vec::new(<$t>::MAX / 3, 2, 1)
            );
        });
    };
    (4, $vec:ident, $t:ty) => {
        glam_test!(test_wrapping_add, {
            assert_eq!(
                $vec::new(<$t>::MAX, 0, 5, 4).wrapping_add($vec::new(1, 2, 3, 2)),
                $vec::new(<$t>::MIN, 2, 8, 6),
            );
        });
        glam_test!(test_wrapping_sub, {
            assert_eq!(
                $vec::new(<$t>::MAX, 4, 5, 4).wrapping_sub($vec::new(1, 2, 3, 2)),
                $vec::new(<$t>::MAX - 1, 2, 2, 2)
            );
        });
        glam_test!(test_wrapping_mul, {
            assert_eq!(
                $vec::new(<$t>::MAX, 2, 5, 4).wrapping_mul($vec::new(3, 2, 3, 2)),
                $vec::new(<$t>::MAX.wrapping_mul(3), 4, 15, 8)
            );
        });
        glam_test!(test_wrapping_div, {
            assert_eq!(
                $vec::new(<$t>::MAX, 4, 5, 4).wrapping_div($vec::new(3, 2, 3, 2)),
                $vec::new(<$t>::MAX / 3, 2, 1, 2)
            );
        });
    };
}

macro_rules! impl_signed_saturating_tests {
    (2, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal) => {
        glam_test!(test_saturating_add, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN).saturating_add($vec::new(1, -1)),
                $vec::new(<$scalar>::MAX, <$scalar>::MIN)
            );
        });
        glam_test!(test_saturating_sub, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MAX).saturating_sub($vec::new(1, -1)),
                $vec::new(<$scalar>::MIN, <$scalar>::MAX)
            );
        });
        glam_test!(test_saturating_mul, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN).saturating_mul($vec::new(2, 2)),
                $vec::new(<$scalar>::MAX, <$scalar>::MIN)
            );
        });
        glam_test!(test_saturating_div, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN).saturating_div($vec::new(2, 2)),
                $vec::new(<$scalar>::MAX / 2, <$scalar>::MIN / 2)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_add_unsigned, {
            assert_eq!($vec::MAX.checked_add_unsigned($paired_vec::ONE), None);
            assert_eq!(
                $vec::NEG_ONE.checked_add_unsigned($paired_vec::ONE),
                Some($vec::ZERO)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_sub_unsigned, {
            assert_eq!($vec::MIN.checked_sub_unsigned($paired_vec::ONE), None);
            assert_eq!(
                $vec::ZERO.checked_sub_unsigned($paired_vec::ONE),
                Some($vec::NEG_ONE)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_add_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX).wrapping_add_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MIN)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_sub_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MIN).wrapping_sub_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_add_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX).saturating_add_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_sub_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MIN).saturating_sub_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MIN)
            );
        });
    };
    (3, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal) => {
        glam_test!(test_saturating_add, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0).saturating_add($vec::new(1, -1, 0)),
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0)
            );
        });
        glam_test!(test_saturating_sub, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MAX, 0).saturating_sub($vec::new(1, -1, 0)),
                $vec::new(<$scalar>::MIN, <$scalar>::MAX, 0)
            );
        });
        glam_test!(test_saturating_mul, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0).saturating_mul($vec::new(2, 2, 0)),
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0)
            );
        });
        glam_test!(test_saturating_div, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0).saturating_div($vec::new(2, 2, 3)),
                $vec::new(<$scalar>::MAX / 2, <$scalar>::MIN / 2, 0)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_add_unsigned, {
            assert_eq!($vec::MAX.checked_add_unsigned($paired_vec::ONE), None);
            assert_eq!(
                $vec::NEG_ONE.checked_add_unsigned($paired_vec::ONE),
                Some($vec::ZERO)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_sub_unsigned, {
            assert_eq!($vec::MIN.checked_sub_unsigned($paired_vec::ONE), None);
            assert_eq!(
                $vec::ZERO.checked_sub_unsigned($paired_vec::ONE),
                Some($vec::NEG_ONE)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_add_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
                    .wrapping_add_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MIN, <$scalar>::MIN)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_sub_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MIN, <$scalar>::MIN)
                    .wrapping_sub_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_add_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
                    .saturating_add_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_sub_unsigned, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MIN, <$scalar>::MIN)
                    .saturating_sub_unsigned($paired_vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MIN, <$scalar>::MIN)
            );
        });
    };
    (4, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal) => {
        glam_test!(test_saturating_add, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0, 0)
                    .saturating_add($vec::new(1, -1, 0, 0)),
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0, 0)
            );
        });
        glam_test!(test_saturating_sub, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MAX, 0, 0)
                    .saturating_sub($vec::new(1, -1, 0, 0)),
                $vec::new(<$scalar>::MIN, <$scalar>::MAX, 0, 0)
            );
        });
        glam_test!(test_saturating_mul, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0, 0)
                    .saturating_mul($vec::new(2, 2, 0, 0)),
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0, 0)
            );
        });
        glam_test!(test_saturating_div, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MIN, 0, 0)
                    .saturating_div($vec::new(2, 2, 3, 4)),
                $vec::new(<$scalar>::MAX / 2, <$scalar>::MIN / 2, 0, 0)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_add_unsigned, {
            assert_eq!($vec::MAX.checked_add_unsigned($paired_vec::ONE), None);
            assert_eq!(
                $vec::NEG_ONE.checked_add_unsigned($paired_vec::ONE),
                Some($vec::ZERO)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_sub_unsigned, {
            assert_eq!($vec::MIN.checked_sub_unsigned($paired_vec::ONE), None);
            assert_eq!(
                $vec::ZERO.checked_sub_unsigned($paired_vec::ONE),
                Some($vec::NEG_ONE)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_add_unsigned, {
            assert_eq!(
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
                .wrapping_add_unsigned($paired_vec::ONE),
                $vec::new(
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN
                )
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_sub_unsigned, {
            assert_eq!(
                $vec::new(
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN
                )
                .wrapping_sub_unsigned($paired_vec::ONE),
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_add_unsigned, {
            assert_eq!(
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
                .saturating_add_unsigned($paired_vec::ONE),
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_sub_unsigned, {
            assert_eq!(
                $vec::new(
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN
                )
                .saturating_sub_unsigned($paired_vec::ONE),
                $vec::new(
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN
                )
            );
        });
    };
}

macro_rules! impl_unsigned_saturating_tests {
    (2, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal) => {
        glam_test!(test_saturating_add, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
                    .saturating_add($vec::new(1, <$scalar>::MAX)),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
            );
        });
        glam_test!(test_saturating_sub, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MAX).saturating_sub($vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MAX - 1)
            );
        });
        glam_test!(test_saturating_mul, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
                    .saturating_mul($vec::new(2, <$scalar>::MAX)),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
            );
        });
        glam_test!(test_saturating_div, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
                    .saturating_div($vec::new(2, <$scalar>::MAX)),
                $vec::new(<$scalar>::MAX / 2, 1)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_add_signed, {
            assert_eq!($vec::MAX.checked_add_signed($paired_vec::ONE), None);
            assert_eq!(
                $vec::ONE.checked_add_signed($paired_vec::NEG_ONE),
                Some($vec::ZERO)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_add_signed, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX).wrapping_add_signed($paired_vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MIN)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_add_signed, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX).saturating_add_signed($paired_vec::ONE),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX)
            );
        });
    };
    (3, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal) => {
        glam_test!(test_saturating_add, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0).saturating_add($vec::new(
                    1,
                    <$scalar>::MAX,
                    0
                )),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0)
            );
        });
        glam_test!(test_saturating_sub, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MAX, 0).saturating_sub($vec::new(1, 1, 0)),
                $vec::new(<$scalar>::MIN, <$scalar>::MAX - 1, 0)
            );
        });
        glam_test!(test_saturating_mul, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0).saturating_mul($vec::new(
                    2,
                    <$scalar>::MAX,
                    0
                )),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0)
            );
        });
        glam_test!(test_saturating_div, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0).saturating_div($vec::new(
                    2,
                    <$scalar>::MAX,
                    3
                )),
                $vec::new(<$scalar>::MAX / 2, 1, 0)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_add_signed, {
            assert_eq!($vec::MAX.checked_add_signed($paired_vec::ONE), None);
            assert_eq!(
                $vec::ONE.checked_add_signed($paired_vec::NEG_ONE),
                Some($vec::ZERO)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_add_signed, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
                    .wrapping_add_signed($paired_vec::ONE),
                $vec::new(<$scalar>::MIN, <$scalar>::MIN, <$scalar>::MIN)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_add_signed, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
                    .saturating_add_signed($paired_vec::ONE),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, <$scalar>::MAX)
            );
        });
    };
    (4, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal) => {
        glam_test!(test_saturating_add, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0, 0).saturating_add($vec::new(
                    1,
                    <$scalar>::MAX,
                    0,
                    0
                )),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0, 0)
            );
        });
        glam_test!(test_saturating_sub, {
            assert_eq!(
                $vec::new(<$scalar>::MIN, <$scalar>::MAX, 0, 0)
                    .saturating_sub($vec::new(1, 1, 0, 0)),
                $vec::new(<$scalar>::MIN, <$scalar>::MAX - 1, 0, 0)
            );
        });
        glam_test!(test_saturating_mul, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0, 0).saturating_mul($vec::new(
                    2,
                    <$scalar>::MAX,
                    0,
                    0
                )),
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0, 0)
            );
        });
        glam_test!(test_saturating_div, {
            assert_eq!(
                $vec::new(<$scalar>::MAX, <$scalar>::MAX, 0, 0).saturating_div($vec::new(
                    2,
                    <$scalar>::MAX,
                    3,
                    4
                )),
                $vec::new(<$scalar>::MAX / 2, 1, 0, 0)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_checked_add_signed, {
            assert_eq!($vec::MAX.checked_add_signed($paired_vec::ONE), None);
            assert_eq!(
                $vec::ONE.checked_add_signed($paired_vec::NEG_ONE),
                Some($vec::ZERO)
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_wrapping_add_signed, {
            assert_eq!(
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
                .wrapping_add_signed($paired_vec::ONE),
                $vec::new(
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN,
                    <$scalar>::MIN
                )
            );
        });
        #[cfg(feature = $paired_feature)]
        glam_test!(test_saturating_add_signed, {
            assert_eq!(
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
                .saturating_add_signed($paired_vec::ONE),
                $vec::new(
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX,
                    <$scalar>::MAX
                )
            );
        });
    };
}

macro_rules! impl_vec_wrapping_saturating_tests {
    ($dim:tt, $vec:ident, $t:ty, wrapping) => {
        impl_vec_wrapping_test!($dim, $vec, $t);
    };
    ($dim:tt, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal, signed) => {
        impl_vec_wrapping_test!($dim, $vec, $scalar);
        impl_signed_saturating_tests!($dim, $vec, $scalar, $paired_vec, $paired_feature);
    };
    ($dim:tt, $vec:ident, $scalar:ty, $paired_vec:ident, $paired_feature:literal, unsigned) => {
        impl_vec_wrapping_test!($dim, $vec, $scalar);
        impl_unsigned_saturating_tests!($dim, $vec, $scalar, $paired_vec, $paired_feature);
    };
}
