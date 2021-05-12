use crate::core::storage::Columns4;
use crate::{DMat3, DMat4, DQuat, DVec3, Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A};
use core::ops::{Deref, DerefMut};

#[cfg(not(feature = "std"))]
use num_traits::Float;

macro_rules! define_affine3_struct {
    ($affine3:ident, $transform:ident, $translate:ident) => {
        /// A 3D affine transform, which can represent translation, rotation, scaling and shear.
        ///
        /// The type is composed of a 3x3 matrix containing a linear transformation (e.g. scale,
        /// rotation, shear, reflection) and a 3D vector translation.
        #[derive(Copy, Clone)]
        pub struct $affine3 {
            pub matrix3: $transform,
            pub translation: $translate,
        }
    };
}

macro_rules! impl_affine3_methods {
    ($t:ty, $mat3:ident, $mat4:ident, $quat:ident, $vec3:ident, $affine3:ident, $transform:ident, $translate:ident) => {
        impl $affine3 {
            /// The degenerate zero transform.
            ///
            /// This transforms any finite vector and point to zero.
            /// The zero transform is non-invertible.
            pub const ZERO: Self = Self {
                matrix3: $transform::ZERO,
                translation: $translate::ZERO,
            };

            /// The identity transform.
            ///
            /// Multiplying a vector with this returns the same vector.
            pub const IDENTITY: Self = Self {
                matrix3: $transform::IDENTITY,
                translation: $translate::ZERO,
            };

            /// Creates an affine transform that changes scale.
            /// Note that if any scale is zero the transform will be non-invertible.
            #[inline(always)]
            pub fn from_scale(scale: $vec3) -> Self {
                Self {
                    matrix3: $transform::from_diagonal(scale),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform from the given `rotation` quaternion.
            #[inline(always)]
            pub fn from_quat(rotation: $quat) -> Self {
                Self {
                    matrix3: $transform::from_quat(rotation),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform containing a 3D rotation around a normalized
            /// rotation `axis` of `angle` (in radians).
            #[inline(always)]
            pub fn from_axis_angle(axis: $vec3, angle: $t) -> Self {
                Self {
                    matrix3: $transform::from_axis_angle(axis, angle),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform containing a 3D rotation around the x axis of
            /// `angle` (in radians).
            #[inline(always)]
            pub fn from_rotation_x(angle: $t) -> Self {
                Self {
                    matrix3: $transform::from_rotation_x(angle),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform containing a 3D rotation around the y axis of
            /// `angle` (in radians).
            #[inline]
            pub fn from_rotation_y(angle: $t) -> Self {
                Self {
                    matrix3: $transform::from_rotation_y(angle),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform containing a 3D rotation around the z axis of
            /// `angle` (in radians).
            #[inline]
            pub fn from_rotation_z(angle: $t) -> Self {
                Self {
                    matrix3: $transform::from_rotation_z(angle),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transformation from the given 3D `translation`.
            #[inline(always)]
            pub fn from_translation(translation: $vec3) -> Self {
                Self {
                    matrix3: $transform::IDENTITY,
                    translation: translation.into(),
                }
            }

            /// Creates an affine transform from a 3x3 matrix (expressing scale, shear and
            /// rotation)
            #[inline(always)]
            pub fn from_mat3(mat3: $mat3) -> Self {
                Self {
                    matrix3: mat3.into(),
                    translation: $translate::ZERO,
                }
            }

            /// Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
            /// and a translation vector.
            ///
            /// Equivalent to `Affine3::from_translation(translation) * Affine3::from_mat3(mat3)`
            #[inline(always)]
            pub fn from_mat3_translation(mat3: $mat3, translation: $vec3) -> Self {
                Self {
                    matrix3: mat3.into(),
                    translation: translation.into(),
                }
            }

            /// Creates an affine transform from the given 3D `scale`, `rotation` and
            /// `translation`.
            ///
            /// Equivalent to `Affine3::from_translation(translation) *
            /// Affine3::from_quat(rotation) * Affine3::from_scale(scale)`
            #[inline(always)]
            pub fn from_scale_rotation_translation(
                scale: $vec3,
                rotation: $quat,
                translation: $vec3,
            ) -> Self {
                let rotation = $transform::from_quat(rotation);
                Self {
                    matrix3: $transform::from_cols(
                        rotation.x_axis * scale.x,
                        rotation.y_axis * scale.y,
                        rotation.z_axis * scale.z,
                    ),
                    translation: translation.into(),
                }
            }

            /// Creates an affine transform from the given 3D `rotation` and `translation`.
            ///
            /// Equivalent to `Affine3::from_translation(translation) * Affine3::from_quat(rotation)`
            #[inline(always)]
            pub fn from_rotation_translation(rotation: $quat, translation: $vec3) -> Self {
                Self {
                    matrix3: $transform::from_quat(rotation.into()),
                    translation: translation.into(),
                }
            }

            /// The given `Mat4` must be an affine transform,
            /// i.e. contain no perspective transform.
            #[inline]
            pub fn from_mat4(m: $mat4) -> Self {
                Self {
                    matrix3: $transform::from_cols(
                        m.x_axis.into(),
                        m.y_axis.into(),
                        m.z_axis.into(),
                    ),
                    translation: m.w_axis.into(),
                }
            }

            /// Extracts `scale`, `rotation` and `translation` from `self`.
            ///
            /// The transform is expected to be non-degenerate and without shearing, or the output
            /// will be invalid.
            #[inline(always)]
            pub fn to_scale_rotation_translation(&self) -> ($vec3, $quat, $vec3) {
                // TODO: migrate to core module
                let det = self.matrix3.determinant();
                glam_assert!(det != 0.0);

                let scale = $vec3::new(
                    self.matrix3.x_axis.length() * det.signum(),
                    self.matrix3.y_axis.length(),
                    self.matrix3.z_axis.length(),
                );

                glam_assert!(scale.cmpne($vec3::ZERO).all());

                let inv_scale = scale.recip();

                let rotation = $quat::from_mat3(&$mat3::from_cols(
                    (self.matrix3.x_axis * inv_scale.x).into(),
                    (self.matrix3.y_axis * inv_scale.y).into(),
                    (self.matrix3.z_axis * inv_scale.z).into(),
                ));

                (scale, rotation, self.translation.into())
            }

            #[inline]
            fn look_to_lh(eye: $vec3, dir: $vec3, up: $vec3) -> Self {
                let f = dir.normalize();
                let s = up.cross(f).normalize();
                let u = f.cross(s);
                Self {
                    matrix3: $transform::from_cols(
                        $vec3::new(s.x, u.x, f.x).into(),
                        $vec3::new(s.y, u.y, f.y).into(),
                        $vec3::new(s.z, u.z, f.z).into(),
                    ),
                    translation: $translate::new(-s.dot(eye), -u.dot(eye), -f.dot(eye)),
                }
            }

            /// Creates a left-handed view transform using a camera position, an up direction, and
            /// a focal point.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
            #[inline]
            pub fn look_at_lh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                glam_assert!(up.is_normalized());
                Self::look_to_lh(eye, center - eye, up)
            }

            /// Creates a right-handed view transform using a camera position, an up direction, and
            /// a focal point.
            ///
            /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
            #[inline]
            pub fn look_at_rh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                glam_assert!(up.is_normalized());
                Self::look_to_lh(eye, eye - center, up)
            }

            /// Transforms the given 3D points, applying shear, scale, rotation and translation.
            #[inline(always)]
            pub fn transform_point3(&self, other: $vec3) -> $vec3 {
                ((self.matrix3.x_axis * other.x)
                    + (self.matrix3.y_axis * other.y)
                    + (self.matrix3.z_axis * other.z)
                    + self.translation)
                    .into()
            }

            /// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
            /// translation).
            ///
            /// To also apply translation, use [`Self::transform_point3`] instead.
            #[inline(always)]
            pub fn transform_vector3(&self, other: $vec3) -> $vec3 {
                ((self.matrix3.x_axis * other.x)
                    + (self.matrix3.y_axis * other.y)
                    + (self.matrix3.z_axis * other.z))
                    .into()
            }

            /// Returns `true` if, and only if, all elements are finite.
            ///
            /// If any element is either `NaN`, positive or negative infinity, this will return
            /// `false`.
            #[inline]
            pub fn is_finite(&self) -> bool {
                self.matrix3.is_finite() && self.translation.is_finite()
            }

            /// Returns `true` if any elements are `NaN`.
            #[inline]
            pub fn is_nan(&self) -> bool {
                self.matrix3.is_nan() || self.translation.is_nan()
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
                self.matrix3.abs_diff_eq(other.matrix3, max_abs_diff)
                    && self
                        .translation
                        .abs_diff_eq(other.translation, max_abs_diff)
            }

            /// Return the inverse of this transform.
            ///
            /// Note that if the transform is not invertible the result will be invalid.
            pub fn inverse(&self) -> Self {
                let matrix3 = self.matrix3.inverse();
                // transform negative translation by the 3x3 inverse:
                let translation = -(matrix3 * self.translation);

                Self {
                    matrix3,
                    translation,
                }
            }
        }
    };
}

macro_rules! impl_affine3_traits {
    ($t:ty, $mat3:ident, $mat4:ident, $vec3:ident, $vec4:ident, $affine3:ident, $transform:ident, $translate:ident, $deref:ident) => {
        impl Default for $affine3 {
            #[inline(always)]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl Deref for $affine3 {
            type Target = $deref;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }

        impl DerefMut for $affine3 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut Self as *mut Self::Target) }
            }
        }

        impl PartialEq for $affine3 {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.matrix3.eq(&other.matrix3) && self.translation.eq(&other.translation)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl core::fmt::Debug for $affine3 {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct(stringify!($affine3))
                    .field("matrix3", &self.matrix3)
                    .field("translation", &self.translation)
                    .finish()
            }
        }

        impl From<$affine3> for $mat4 {
            #[inline]
            fn from(m: $affine3) -> $mat4 {
                $mat4::from_cols(
                    m.matrix3.x_axis.extend(0.0),
                    m.matrix3.y_axis.extend(0.0),
                    m.matrix3.z_axis.extend(0.0),
                    m.translation.extend(1.0),
                )
            }
        }

        impl core::ops::Mul for $affine3 {
            type Output = $affine3;

            #[inline(always)]
            fn mul(self, other: $affine3) -> Self::Output {
                Self {
                    matrix3: self.matrix3 * other.matrix3,
                    translation: self.matrix3 * other.translation + self.translation,
                }
            }
        }

        impl core::ops::Mul<$mat4> for $affine3 {
            type Output = $mat4;

            #[inline(always)]
            fn mul(self, rhs: $mat4) -> Self::Output {
                $mat4::from(self) * rhs
            }
        }

        impl core::ops::Mul<$affine3> for $mat4 {
            type Output = $mat4;

            #[inline(always)]
            fn mul(self, rhs: $affine3) -> Self::Output {
                self * $mat4::from(rhs)
            }
        }

        impl<'a> core::iter::Product<&'a Self> for $affine3 {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::IDENTITY, |a, &b| a * b)
            }
        }
    };
}

type TransformF32 = Mat3A;
type TranslateF32 = Vec3A;
type DerefTargetF32 = Columns4<crate::Vec3A>;

define_affine3_struct!(Affine3A, TransformF32, TranslateF32);
impl_affine3_methods!(
    f32,
    Mat3,
    Mat4,
    Quat,
    Vec3,
    Affine3A,
    TransformF32,
    TranslateF32
);
impl_affine3_traits!(
    f32,
    Mat3,
    Mat4,
    Vec3,
    Vec4,
    Affine3A,
    TransformF32,
    TranslateF32,
    DerefTargetF32
);

impl Affine3A {
    /// Transforms the given `Vec3A`, applying shear, scale, rotation and translation.
    #[inline(always)]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        self.matrix3 * other + self.translation
    }

    /// Transforms the given `Vec3A`, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3`] instead.
    #[inline(always)]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
        self.matrix3 * other
    }
}

type TransformF64 = DMat3;
type TranslateF64 = DVec3;
type DerefTargetF64 = Columns4<DVec3>;

define_affine3_struct!(DAffine3, TransformF64, TranslateF64);
impl_affine3_methods!(
    f64,
    DMat3,
    DMat4,
    DQuat,
    DVec3,
    DAffine3,
    TransformF64,
    TranslateF64
);
impl_affine3_traits!(
    f64,
    DMat3,
    DMat4,
    DVec3,
    DVec4,
    DAffine3,
    TransformF64,
    TranslateF64,
    DerefTargetF64
);

mod const_test_affine3a {
    const_assert_eq!(16, core::mem::align_of::<super::Affine3A>());
    const_assert_eq!(64, core::mem::size_of::<super::Affine3A>());
} 

mod const_test_daffine3 {
    const_assert_eq!(8, core::mem::align_of::<super::DAffine3>());
    const_assert_eq!(96, core::mem::size_of::<super::DAffine3>());
}
