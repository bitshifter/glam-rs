use crate::core::storage::Columns3;
use crate::{DMat2, DMat3, DVec2, Mat2, Mat3, Vec2};
use core::ops::{Deref, DerefMut};

#[cfg(not(feature = "std"))]
use num_traits::Float;

macro_rules! define_affine2_struct {
    ($affine2:ident, $transform:ident, $translate:ident) => {
        /// A 2D affine transform, which can represent translation, rotation, scaling and shear.
        #[derive(Copy, Clone)]
        pub struct $affine2 {
            pub matrix2: $transform,
            pub translation: $translate,
        }
    };
}

macro_rules! impl_affine2_methods {
    ($t:ty, $mat2:ident, $mat3:ident, $vec2:ident, $affine2:ident, $transform:ident, $translate:ident) => {
        impl $affine2 {
            /// The degenerate zero transform.
            ///
            /// This transforms any finite vector and point to zero.
            /// The zero transform is non-invertible.
            pub const ZERO: Self = Self {
                matrix2: $transform::ZERO,
                translation: $translate::ZERO,
            };

            /// The identity transform.
            ///
            /// Multiplying a vector with this returns the same vector.
            pub const IDENTITY: Self = Self {
                matrix2: $transform::IDENTITY,
                translation: $translate::ZERO,
            };

            /// Creates an affine transform that changes scale.
            /// Note that if any scale is zero the transform will be non-invertible.
            #[inline(always)]
            pub fn from_scale(scale: $vec2) -> Self {
                Self {
                    matrix2: $transform::from_diagonal(scale),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform from the given rotation `angle`.
            #[inline(always)]
            pub fn from_angle(angle: $t) -> Self {
                Self {
                    matrix2: $transform::from_angle(angle),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transformation from the given 2D `translation`.
            #[inline(always)]
            pub fn from_translation(translation: $vec2) -> Self {
                Self {
                    matrix2: $transform::IDENTITY,
                    translation,
                }
            }

            /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and
            /// rotation)
            #[inline(always)]
            pub fn from_mat2(matrix2: $mat2) -> Self {
                Self {
                    matrix2,
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
            /// and a translation vector.
            ///
            /// Equivalent to `Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`
            #[inline(always)]
            pub fn from_mat2_translation(matrix2: $mat2, translation: $vec2) -> Self {
                Self {
                    matrix2,
                    translation,
                }
            }

            /// Creates an affine transform from the given 2D `scale`, rotation `angle` (in
            /// radians) and `translation`.
            ///
            /// Equivalent to `Affine2::from_translation(translation) *
            /// Affine2::from_angle(angle) * Affine2::from_scale(scale)`
            #[inline]
            pub fn from_scale_angle_translation(
                scale: $vec2,
                angle: $t,
                translation: $vec2,
            ) -> Self {
                let rotation = $transform::from_angle(angle);
                Self {
                    matrix2: $transform::from_cols(
                        rotation.x_axis * scale.x,
                        rotation.y_axis * scale.y,
                    ),
                    translation,
                }
            }

            /// Creates an affine transform from the given 2D rotation `angle` (in radians) and
            /// `translation`.
            ///
            /// Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`
            #[inline(always)]
            pub fn from_angle_translation(angle: $t, translation: $vec2) -> Self {
                Self {
                    matrix2: $transform::from_angle(angle),
                    translation,
                }
            }

            /// The given `Mat3` must be an affine transform,
            #[inline]
            pub fn from_mat3(m: $mat3) -> Self {
                Self {
                    matrix2: $transform::from_cols(m.x_axis.into(), m.y_axis.into()),
                    translation: m.z_axis.into(),
                }
            }

            /// Transforms the given 2D point, applying shear, scale, rotation and translation.
            #[inline(always)]
            pub fn transform_point2(&self, other: $vec2) -> $vec2 {
                self.matrix2 * other + self.translation
            }

            /// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
            /// translation).
            ///
            /// To also apply translation, use [`Self::transform_point2`] instead.
            #[inline(always)]
            pub fn transform_vector2(&self, other: $vec2) -> $vec2 {
                self.matrix2 * other
            }

            /// Returns `true` if, and only if, all elements are finite.
            ///
            /// If any element is either `NaN`, positive or negative infinity, this will return
            /// `false`.
            #[inline]
            pub fn is_finite(&self) -> bool {
                self.matrix2.is_finite() && self.translation.is_finite()
            }

            /// Returns `true` if any elements are `NaN`.
            #[inline]
            pub fn is_nan(&self) -> bool {
                self.matrix2.is_nan() || self.translation.is_nan()
            }

            /// Returns true if the absolute difference of all elements between `self` and `other`
            /// is less than or equal to `max_abs_diff`.
            ///
            /// This can be used to compare if two 3x4 matrices contain similar elements. It works
            /// best when comparing with a known value. The `max_abs_diff` that should be used used
            /// depends on the values being compared against.
            ///
            /// For more see
            /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
            #[inline]
            pub fn abs_diff_eq(&self, other: Self, max_abs_diff: $t) -> bool {
                self.matrix2.abs_diff_eq(&other.matrix2, max_abs_diff)
                    && self
                        .translation
                        .abs_diff_eq(other.translation, max_abs_diff)
            }

            /// Return the inverse of this transform.
            ///
            /// Note that if the transform is not invertible the result will be invalid.
            #[inline]
            pub fn inverse(&self) -> Self {
                let matrix2 = self.matrix2.inverse();
                // transform negative translation by the 2x2 inverse:
                let translation = -(matrix2 * self.translation);

                Self {
                    matrix2,
                    translation,
                }
            }
        }
    };
}

macro_rules! impl_affine2_traits {
    ($t:ty, $mat2:ident, $mat3:ident, $vec2:ident, $affine2:ident, $transform:ident, $translate:ident, $deref:ident) => {
        impl Default for $affine2 {
            #[inline(always)]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl Deref for $affine2 {
            type Target = $deref;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $affine2 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        impl PartialEq for $affine2 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.matrix2.eq(&other.matrix2) && self.translation.eq(&other.translation)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl core::fmt::Debug for $affine2 {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct(stringify!($affine2))
                    .field("matrix2", &self.matrix2)
                    .field("translation", &self.translation)
                    .finish()
            }
        }

        impl From<$affine2> for $mat3 {
            #[inline]
            fn from(m: $affine2) -> $mat3 {
                Self::from_cols(
                    m.matrix2.x_axis.extend(0.0),
                    m.matrix2.y_axis.extend(0.0),
                    m.translation.extend(1.0),
                )
            }
        }

        impl core::ops::Mul for $affine2 {
            type Output = $affine2;

            #[inline(always)]
            fn mul(self, other: $affine2) -> Self::Output {
                Self {
                    matrix2: self.matrix2 * other.matrix2,
                    translation: self.matrix2 * other.translation + self.translation,
                }
            }
        }

        impl core::ops::Mul<$mat3> for $affine2 {
            type Output = $mat3;

            #[inline(always)]
            fn mul(self, other: $mat3) -> Self::Output {
                $mat3::from(self) * other
            }
        }

        impl core::ops::Mul<$affine2> for $mat3 {
            type Output = $mat3;

            #[inline(always)]
            fn mul(self, other: $affine2) -> Self::Output {
                self * $mat3::from(other)
            }
        }

        impl<'a> core::iter::Product<&'a Self> for $affine2 {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::IDENTITY, |a, &b| a * b)
            }
        }
    };
}

type TransformF32 = Mat2;
type TranslateF32 = Vec2;
type DerefTargetF32 = Columns3<crate::Vec2>;

define_affine2_struct!(Affine2, TransformF32, TranslateF32);
impl_affine2_methods!(f32, Mat2, Mat3, Vec2, Affine2, TransformF32, TranslateF32);
impl_affine2_traits!(
    f32,
    Mat2,
    Mat3,
    Vec2,
    Affine2,
    TransformF32,
    TranslateF32,
    DerefTargetF32
);

type TransformF64 = DMat2;
type TranslateF64 = DVec2;
type DerefTargetF64 = Columns3<DVec2>;

define_affine2_struct!(DAffine2, TransformF64, TranslateF64);
impl_affine2_methods!(
    f64,
    DMat2,
    DMat3,
    DVec2,
    DAffine2,
    TransformF64,
    TranslateF64
);
impl_affine2_traits!(
    f64,
    DMat2,
    DMat3,
    DVec2,
    DAffine2,
    TransformF64,
    TranslateF64,
    DerefTargetF64
);

#[cfg(any(feature = "scalar-math", target_arch = "spriv"))]
mod const_test_affine2 {
    const_assert_eq!(4, core::mem::align_of::<super::Affine2>());
    const_assert_eq!(24, core::mem::size_of::<super::Affine2>());
}

#[cfg(not(any(feature = "scalar-math", target_arch = "spriv")))]
mod const_test_affine2 {
    const_assert_eq!(16, core::mem::align_of::<super::Affine2>());
    const_assert_eq!(32, core::mem::size_of::<super::Affine2>());
}

mod const_test_daffine2 {
    const_assert_eq!(8, core::mem::align_of::<super::DAffine2>());
    const_assert_eq!(48, core::mem::size_of::<super::DAffine2>());
}
