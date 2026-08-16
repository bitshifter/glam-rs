use crate::{Affine2, Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
#[cfg(feature = "f64")]
use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

macro_rules! impl_approx_as_ref {
    ($prim:ident, $type:ty) => {
        impl AbsDiffEq for $type {
            type Epsilon = <$prim as AbsDiffEq>::Epsilon;
            fn default_epsilon() -> Self::Epsilon {
                $prim::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.as_ref().abs_diff_eq(other.as_ref(), epsilon)
            }
        }

        impl RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                $prim::default_max_relative()
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.as_ref()
                    .relative_eq(other.as_ref(), epsilon, max_relative)
            }
        }

        impl UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                $prim::default_max_ulps()
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.as_ref().ulps_eq(other.as_ref(), epsilon, max_ulps)
            }
        }
    };
}

macro_rules! impl_approx_xzy_axes {
    ($prim:ident, $type:ty) => {
        impl AbsDiffEq for $type {
            type Epsilon = <$prim as AbsDiffEq>::Epsilon;
            fn default_epsilon() -> Self::Epsilon {
                $prim::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                AbsDiffEq::abs_diff_eq(&self.x_axis, &other.x_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.y_axis, &other.y_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.z_axis, &other.z_axis, epsilon)
            }
        }

        impl RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                $prim::default_max_relative()
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                RelativeEq::relative_eq(&self.x_axis, &other.x_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.y_axis, &other.y_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.z_axis, &other.z_axis, epsilon, max_relative)
            }
        }

        impl UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                $prim::default_max_ulps()
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                UlpsEq::ulps_eq(&self.x_axis, &other.x_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.y_axis, &other.y_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.z_axis, &other.z_axis, epsilon, max_ulps)
            }
        }
    };
}

macro_rules! impl_approx_xzyw_axes {
    ($prim:ident, $type:ty) => {
        impl AbsDiffEq for $type {
            type Epsilon = <$prim as AbsDiffEq>::Epsilon;
            fn default_epsilon() -> Self::Epsilon {
                $prim::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                AbsDiffEq::abs_diff_eq(&self.x_axis, &other.x_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.y_axis, &other.y_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.z_axis, &other.z_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.w_axis, &other.w_axis, epsilon)
            }
        }

        impl RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                $prim::default_max_relative()
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                RelativeEq::relative_eq(&self.x_axis, &other.x_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.y_axis, &other.y_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.z_axis, &other.z_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.w_axis, &other.w_axis, epsilon, max_relative)
            }
        }

        impl UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                $prim::default_max_ulps()
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                UlpsEq::ulps_eq(&self.x_axis, &other.x_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.y_axis, &other.y_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.z_axis, &other.z_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.w_axis, &other.w_axis, epsilon, max_ulps)
            }
        }
    };
}

impl_approx_as_ref!(f32, Mat2);
impl_approx_as_ref!(f32, Mat3);
impl_approx_as_ref!(f32, Mat4);
impl_approx_as_ref!(f32, Quat);
impl_approx_as_ref!(f32, Vec2);
impl_approx_as_ref!(f32, Vec3);
impl_approx_as_ref!(f32, Vec4);
impl_approx_as_ref!(f32, Vec3A);

impl_approx_xzy_axes!(f32, Affine2);
impl_approx_xzyw_axes!(f32, Affine3);
impl_approx_xzyw_axes!(f32, Affine3A);
impl_approx_xzy_axes!(f32, Mat3A);

#[cfg(feature = "f64")]
impl_approx_xzy_axes!(f64, DAffine2);
#[cfg(feature = "f64")]
impl_approx_xzyw_axes!(f64, DAffine3);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DMat2);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DMat3);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DMat4);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DQuat);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DVec2);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DVec3);
#[cfg(feature = "f64")]
impl_approx_as_ref!(f64, DVec4);

#[cfg(feature = "f16")]
mod f16 {
    //! Approximate comparison implementations for half-precision types.
    //!
    //! The `approx` crate only implements its traits for `f32`/`f64` (and
    //! integer) scalars, so these are implemented manually for `f16` types.
    //! Comparisons are performed component-wise and ULP distances are computed
    //! from the `u16` bit representation of `f16` values.
    use crate::{HQuat, HVec2, HVec3, HVec4};
    use approx::{AbsDiffEq, RelativeEq, UlpsEq};
    use half::f16;

    #[inline]
    fn abs_diff(a: f16, b: f16) -> f16 {
        let d = a - b;
        if d < f16::ZERO {
            -d
        } else {
            d
        }
    }

    #[inline]
    fn abs(a: f16) -> f16 {
        if a < f16::ZERO {
            -a
        } else {
            a
        }
    }

    #[inline]
    fn ulps_diff(a: f16, b: f16) -> u32 {
        // Order the sign-magnitude `u16` bits monotonically so that the ULP
        // distance between two values is the absolute difference of the
        // ordered representations.
        let ai = a.to_bits() as i32;
        let bi = b.to_bits() as i32;
        let a_ordered = if ai & 0x8000 != 0 { 0x8000 - ai } else { ai };
        let b_ordered = if bi & 0x8000 != 0 { 0x8000 - bi } else { bi };
        (a_ordered - b_ordered).unsigned_abs()
    }

    macro_rules! impl_approx {
        ($type:ty, $($comp:ident),+) => {
            impl AbsDiffEq for $type {
                type Epsilon = f16;
                #[inline]
                fn default_epsilon() -> f16 {
                    f16::from_f32(1e-3)
                }
                #[inline]
                fn abs_diff_eq(&self, other: &Self, epsilon: f16) -> bool {
                    $(abs_diff(self.$comp, other.$comp) <= epsilon)&&+
                }
            }

            impl RelativeEq for $type {
                #[inline]
                fn default_max_relative() -> f16 {
                    f16::from_f32(1e-3)
                }
                #[inline]
                fn relative_eq(&self, other: &Self, epsilon: f16, max_relative: f16) -> bool {
                    $(
                        abs_diff(self.$comp, other.$comp)
                            <= max_relative * abs(self.$comp).max(abs(other.$comp)).max(epsilon)
                    )&&+
                }
            }

            impl UlpsEq for $type {
                #[inline]
                fn default_max_ulps() -> u32 {
                    4
                }
                #[inline]
                fn ulps_eq(&self, other: &Self, epsilon: f16, max_ulps: u32) -> bool {
                    $(
                        abs_diff(self.$comp, other.$comp) <= epsilon
                            || ulps_diff(self.$comp, other.$comp) <= max_ulps
                    )&&+
                }
            }
        };
    }

    impl_approx!(HVec2, x, y);
    impl_approx!(HVec3, x, y, z);
    impl_approx!(HVec4, x, y, z, w);
    impl_approx!(HQuat, x, y, z, w);
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;

    /// Helper trait to get a typed `1.0` scalar, since `half::f16` values
    /// cannot be constructed from untyped float literals.
    trait TestScalar {
        const ONE: Self;
    }
    impl TestScalar for f32 {
        const ONE: Self = 1.0;
    }
    impl TestScalar for f64 {
        const ONE: Self = 1.0;
    }
    #[cfg(feature = "f16")]
    impl TestScalar for half::f16 {
        const ONE: Self = half::f16::from_f32_const(1.0);
    }

    macro_rules! impl_approx_test {
        ($prim:ident, $type:ident, $ones:expr) => {
            let one_eps = $ones * $type::default_epsilon();
            let two_eps = one_eps + one_eps;

            let one_ulp = $ones * $prim::from_bits($prim::to_bits(<$prim as TestScalar>::ONE) + 1);
            let four_ulp =
                $ones * $prim::from_bits($prim::to_bits(<$prim as TestScalar>::ONE) + 16);

            approx::assert_abs_diff_eq!($ones, $ones);
            approx::assert_abs_diff_eq!($ones, $ones + one_eps);
            approx::assert_abs_diff_eq!($ones, $ones - one_eps);

            approx::assert_abs_diff_ne!($ones, $ones + two_eps);
            approx::assert_abs_diff_ne!($ones, $ones - two_eps);

            approx::assert_relative_eq!($ones, $ones);
            approx::assert_relative_ne!($ones, $ones - $ones);

            // defaults to 4 ulps and I have no idea how to pass other parameters to this macro :)
            approx::assert_ulps_eq!($ones, one_ulp);
            approx::assert_ulps_ne!($ones, four_ulp);
        };
        ($prim:ident, $type:ident) => {
            impl_approx_test!($prim, $type, $type::ONE)
        };
    }

    macro_rules! impl_affine_approx_test {
        ($prim:ident, $type:ident, $from:ident, $mat:ident, $ones:expr) => {
            let ones = $mat::from($ones);
            let one_eps = ones * $mat::default_epsilon();
            let two_eps = one_eps + one_eps;

            let one_ulp = ones * $prim::from_bits($prim::to_bits(<$prim as TestScalar>::ONE) + 1);
            let four_ulp = ones * $prim::from_bits($prim::to_bits(<$prim as TestScalar>::ONE) + 16);

            approx::assert_abs_diff_eq!($type::$from(ones), $type::$from(ones));
            approx::assert_abs_diff_eq!($type::$from(ones), $type::$from(ones + one_eps));
            approx::assert_abs_diff_eq!($type::$from(ones), $type::$from(ones - one_eps));

            approx::assert_abs_diff_ne!($type::$from(ones), $type::$from(ones + two_eps));
            approx::assert_abs_diff_ne!($type::$from(ones), $type::$from(ones - two_eps));

            approx::assert_relative_eq!($type::$from(ones), $type::$from(ones));
            approx::assert_relative_ne!($type::$from(ones), $type::$from(ones - ones));

            // defaults to 4 ulps and I have no idea how to pass other parameters to this macro :)
            approx::assert_ulps_eq!($type::$from(ones), $type::$from(one_ulp));
            approx::assert_ulps_ne!($type::$from(ones), $type::$from(four_ulp));
        };
    }
    #[test]
    fn test_approx() {
        const ONESF32: [f32; 16] = [1.0; 16];

        impl_approx_test!(f32, Vec2);
        impl_approx_test!(f32, Vec3);
        impl_approx_test!(f32, Vec3A);
        impl_approx_test!(f32, Vec4);
        impl_approx_test!(f32, Quat, Quat::from_slice(&ONESF32));
        impl_approx_test!(f32, Mat2, Mat2::from_cols_slice(&ONESF32));
        impl_approx_test!(f32, Mat3, Mat3::from_cols_slice(&ONESF32));
        impl_approx_test!(f32, Mat3A, Mat3A::from_cols_slice(&ONESF32));
        impl_approx_test!(f32, Mat4, Mat4::from_cols_slice(&ONESF32));
        impl_affine_approx_test!(
            f32,
            Affine2,
            from_mat3,
            Mat3,
            Affine2::from_cols_slice(&ONESF32)
        );
        impl_affine_approx_test!(
            f32,
            Affine3,
            from_mat4,
            Mat4,
            Affine3::from_cols_slice(&ONESF32)
        );
        impl_affine_approx_test!(
            f32,
            Affine3A,
            from_mat4,
            Mat4,
            Affine3A::from_cols_slice(&ONESF32)
        );

        #[cfg(feature = "f64")]
        {
            const ONESF64: [f64; 16] = [1.0; 16];
            impl_approx_test!(f64, DVec2);
            impl_approx_test!(f64, DVec3);
            impl_approx_test!(f64, DVec4);
            impl_approx_test!(f64, DQuat, DQuat::from_slice(&ONESF64));
            impl_approx_test!(f64, DMat2, DMat2::from_cols_slice(&ONESF64));
            impl_approx_test!(f64, DMat3, DMat3::from_cols_slice(&ONESF64));
            impl_approx_test!(f64, DMat4, DMat4::from_cols_slice(&ONESF64));
            impl_affine_approx_test!(
                f64,
                DAffine2,
                from_mat3,
                DMat3,
                DAffine2::from_cols_slice(&ONESF64)
            );
            impl_affine_approx_test!(
                f64,
                DAffine3,
                from_mat4,
                DMat4,
                DAffine3::from_cols_slice(&ONESF64)
            );
        }

        #[cfg(feature = "f16")]
        {
            use half::f16;
            const ONESF16: [f16; 16] = [f16::from_f32_const(1.0); 16];
            impl_approx_test!(f16, HVec2);
            impl_approx_test!(f16, HVec3);
            impl_approx_test!(f16, HVec4);
            impl_approx_test!(f16, HQuat, HQuat::from_slice(&ONESF16));
        }
    }
}
