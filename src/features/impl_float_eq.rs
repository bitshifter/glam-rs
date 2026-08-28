use crate::{Affine2, Affine3, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
#[cfg(feature = "f64")]
use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
use float_eq::{
    AssertFloatEq, AssertFloatEqAll, DebugUlpsDiff, FloatEq, FloatEqAll, FloatEqDebugUlpsDiff,
    FloatEqUlpsTol, UlpsTol,
};

// Every glam type here can be viewed as a fixed size array of primitives, and
// `float_eq` already implements all of its traits for `[T; N]`. So each impl
// just borrows the values as an array (via `to_array` or `to_cols_array`) and
// defers to that. The tolerance and debug types are the array's, which keeps
// the per-field comparisons lining up with the components of the type.
macro_rules! impl_float_eq {
    ($prim:ident, $type:ty, $n:literal, $to_array:ident) => {
        impl FloatEqUlpsTol for $type {
            type UlpsTol = <[$prim; $n] as FloatEqUlpsTol>::UlpsTol;
        }

        impl FloatEqDebugUlpsDiff for $type {
            type DebugUlpsDiff = <[$prim; $n] as FloatEqDebugUlpsDiff>::DebugUlpsDiff;
        }

        impl FloatEq for $type {
            type Tol = <[$prim; $n] as FloatEq>::Tol;

            fn eq_abs(&self, other: &Self, tol: &Self::Tol) -> bool {
                self.$to_array().eq_abs(&other.$to_array(), tol)
            }

            fn eq_rmax(&self, other: &Self, tol: &Self::Tol) -> bool {
                self.$to_array().eq_rmax(&other.$to_array(), tol)
            }

            fn eq_rmin(&self, other: &Self, tol: &Self::Tol) -> bool {
                self.$to_array().eq_rmin(&other.$to_array(), tol)
            }

            fn eq_r1st(&self, other: &Self, tol: &Self::Tol) -> bool {
                self.$to_array().eq_r1st(&other.$to_array(), tol)
            }

            fn eq_r2nd(&self, other: &Self, tol: &Self::Tol) -> bool {
                self.$to_array().eq_r2nd(&other.$to_array(), tol)
            }

            fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Self::Tol>) -> bool {
                self.$to_array().eq_ulps(&other.$to_array(), tol)
            }
        }

        impl FloatEqAll for $type {
            type AllTol = <[$prim; $n] as FloatEqAll>::AllTol;

            fn eq_abs_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.$to_array().eq_abs_all(&other.$to_array(), tol)
            }

            fn eq_rmax_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.$to_array().eq_rmax_all(&other.$to_array(), tol)
            }

            fn eq_rmin_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.$to_array().eq_rmin_all(&other.$to_array(), tol)
            }

            fn eq_r1st_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.$to_array().eq_r1st_all(&other.$to_array(), tol)
            }

            fn eq_r2nd_all(&self, other: &Self, tol: &Self::AllTol) -> bool {
                self.$to_array().eq_r2nd_all(&other.$to_array(), tol)
            }

            fn eq_ulps_all(&self, other: &Self, tol: &UlpsTol<Self::AllTol>) -> bool {
                self.$to_array().eq_ulps_all(&other.$to_array(), tol)
            }
        }

        impl AssertFloatEq for $type {
            type DebugAbsDiff = <[$prim; $n] as AssertFloatEq>::DebugAbsDiff;
            type DebugTol = <[$prim; $n] as AssertFloatEq>::DebugTol;

            fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
                self.$to_array().debug_abs_diff(&other.$to_array())
            }

            fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
                self.$to_array().debug_ulps_diff(&other.$to_array())
            }

            fn debug_abs_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                self.$to_array().debug_abs_tol(&other.$to_array(), tol)
            }

            fn debug_rmax_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                self.$to_array().debug_rmax_tol(&other.$to_array(), tol)
            }

            fn debug_rmin_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                self.$to_array().debug_rmin_tol(&other.$to_array(), tol)
            }

            fn debug_r1st_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                self.$to_array().debug_r1st_tol(&other.$to_array(), tol)
            }

            fn debug_r2nd_tol(&self, other: &Self, tol: &Self::Tol) -> Self::DebugTol {
                self.$to_array().debug_r2nd_tol(&other.$to_array(), tol)
            }

            fn debug_ulps_tol(
                &self,
                other: &Self,
                tol: &UlpsTol<Self::Tol>,
            ) -> UlpsTol<Self::DebugTol> {
                self.$to_array().debug_ulps_tol(&other.$to_array(), tol)
            }
        }

        impl AssertFloatEqAll for $type {
            type AllDebugTol = <[$prim; $n] as AssertFloatEqAll>::AllDebugTol;

            fn debug_abs_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.$to_array().debug_abs_all_tol(&other.$to_array(), tol)
            }

            fn debug_rmax_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.$to_array().debug_rmax_all_tol(&other.$to_array(), tol)
            }

            fn debug_rmin_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.$to_array().debug_rmin_all_tol(&other.$to_array(), tol)
            }

            fn debug_r1st_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.$to_array().debug_r1st_all_tol(&other.$to_array(), tol)
            }

            fn debug_r2nd_all_tol(&self, other: &Self, tol: &Self::AllTol) -> Self::AllDebugTol {
                self.$to_array().debug_r2nd_all_tol(&other.$to_array(), tol)
            }

            fn debug_ulps_all_tol(
                &self,
                other: &Self,
                tol: &UlpsTol<Self::AllTol>,
            ) -> UlpsTol<Self::AllDebugTol> {
                self.$to_array().debug_ulps_all_tol(&other.$to_array(), tol)
            }
        }
    };
}

impl_float_eq!(f32, Vec2, 2, to_array);
impl_float_eq!(f32, Vec3, 3, to_array);
impl_float_eq!(f32, Vec3A, 3, to_array);
impl_float_eq!(f32, Vec4, 4, to_array);
impl_float_eq!(f32, Quat, 4, to_array);
impl_float_eq!(f32, Mat2, 4, to_cols_array);
impl_float_eq!(f32, Mat3, 9, to_cols_array);
impl_float_eq!(f32, Mat3A, 9, to_cols_array);
impl_float_eq!(f32, Mat4, 16, to_cols_array);
impl_float_eq!(f32, Affine2, 6, to_cols_array);
impl_float_eq!(f32, Affine3, 12, to_cols_array);
impl_float_eq!(f32, Affine3A, 12, to_cols_array);

#[cfg(feature = "f64")]
impl_float_eq!(f64, DVec2, 2, to_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DVec3, 3, to_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DVec4, 4, to_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DQuat, 4, to_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DMat2, 4, to_cols_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DMat3, 9, to_cols_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DMat4, 16, to_cols_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DAffine2, 6, to_cols_array);
#[cfg(feature = "f64")]
impl_float_eq!(f64, DAffine3, 12, to_cols_array);

#[cfg(test)]
mod test {
    use crate::*;
    use float_eq::{assert_float_eq, assert_float_ne};

    // Perturbs a value by scaling it, which nudges every component by the same
    // number of ulps away from the original.
    macro_rules! impl_float_eq_asserts {
        ($eps:expr, $ones:expr, $near:expr, $far:expr) => {{
            assert_float_eq!($ones, $ones, abs_all <= 0.0);
            assert_float_eq!($ones, $ones, ulps_all <= 0);

            assert_float_eq!($ones, $near, abs_all <= 4.0 * $eps);
            assert_float_eq!($ones, $near, rmax_all <= 4.0 * $eps);
            assert_float_eq!($ones, $near, ulps_all <= 4);

            assert_float_ne!($ones, $far, abs_all <= 4.0 * $eps);
            assert_float_ne!($ones, $far, rmax_all <= 4.0 * $eps);
            assert_float_ne!($ones, $far, ulps_all <= 4);
        }};
    }

    macro_rules! impl_float_eq_test {
        ($prim:ident, $type:ident, $ones:expr) => {{
            let ones: $type = $ones;
            let eps = $prim::EPSILON;
            let near = ones * (1.0 + 2.0 * eps);
            let far = ones * (1.0 + 64.0 * eps);
            impl_float_eq_asserts!(eps, ones, near, far);
        }};
        ($prim:ident, $type:ident) => {
            impl_float_eq_test!($prim, $type, $type::ONE)
        };
    }

    // Affine types don't multiply by a scalar, so build the perturbed values
    // from their column arrays instead.
    macro_rules! impl_affine_float_eq_test {
        ($prim:ident, $type:ident, $n:literal) => {{
            let eps = $prim::EPSILON;
            let ones = <$type>::from_cols_array(&[1.0; $n]);
            let near = <$type>::from_cols_array(&[1.0 + 2.0 * eps; $n]);
            let far = <$type>::from_cols_array(&[1.0 + 64.0 * eps; $n]);
            impl_float_eq_asserts!(eps, ones, near, far);
        }};
    }

    #[test]
    fn test_float_eq_f32() {
        const ONES: [f32; 16] = [1.0; 16];

        impl_float_eq_test!(f32, Vec2);
        impl_float_eq_test!(f32, Vec3);
        impl_float_eq_test!(f32, Vec3A);
        impl_float_eq_test!(f32, Vec4);
        impl_float_eq_test!(f32, Quat, Quat::from_slice(&ONES));
        impl_float_eq_test!(f32, Mat2, Mat2::from_cols_slice(&ONES));
        impl_float_eq_test!(f32, Mat3, Mat3::from_cols_slice(&ONES));
        impl_float_eq_test!(f32, Mat3A, Mat3A::from_cols_slice(&ONES));
        impl_float_eq_test!(f32, Mat4, Mat4::from_cols_slice(&ONES));

        impl_affine_float_eq_test!(f32, Affine2, 6);
        impl_affine_float_eq_test!(f32, Affine3, 12);
        impl_affine_float_eq_test!(f32, Affine3A, 12);
    }

    #[cfg(feature = "f64")]
    #[test]
    fn test_float_eq_f64() {
        const ONES: [f64; 16] = [1.0; 16];

        impl_float_eq_test!(f64, DVec2);
        impl_float_eq_test!(f64, DVec3);
        impl_float_eq_test!(f64, DVec4);
        impl_float_eq_test!(f64, DQuat, DQuat::from_slice(&ONES));
        impl_float_eq_test!(f64, DMat2, DMat2::from_cols_slice(&ONES));
        impl_float_eq_test!(f64, DMat3, DMat3::from_cols_slice(&ONES));
        impl_float_eq_test!(f64, DMat4, DMat4::from_cols_slice(&ONES));

        impl_affine_float_eq_test!(f64, DAffine2, 6);
        impl_affine_float_eq_test!(f64, DAffine3, 12);
    }
}
