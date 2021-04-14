use crate::core::{
    storage::Vector3x3,
    traits::{
        matrix::{FloatMatrix3x3, Matrix3x3, MatrixConst},
        vector::{FloatVector3, SignedVector, Vector},
    },
};
use crate::{Mat3, Mat4, Quat, Vec3};

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(not(feature = "std"))]
use num_traits::Float;

macro_rules! define_affine3_struct {
    ($affine3:ident, $transform:ident, $translate:ident) => {
        /// A 3D affine transform, which can represent translation, rotation, scaling and shear.
        #[derive(Copy, Clone)]
        pub struct $affine3 {
            pub(crate) transform: $transform,
            pub(crate) translate: $translate,
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
                transform: $transform::ZERO,
                translate: $translate::ZERO,
            };

            /// The identity transform.
            ///
            /// Multiplying a vector with this returns the same vector.
            pub const IDENTITY: Self = Self {
                transform: $transform::IDENTITY,
                translate: $translate::ZERO,
            };

            /// Creates a transformation matrix that changes scale.
            /// Note that if any scale is zero the transform will be non-invertible.
            #[inline(always)]
            pub fn from_scale(scale: $vec3) -> Self {
                Self {
                    transform: $transform::from_diagonal(scale.0),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation matrix from the given `rotation` quaternion.
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_quat(rotation: Quat) -> Self {
                Self {
                    // TODO: unnecessary into
                    transform: $transform::from_quaternion(rotation.0.into()),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation matrix containing a 3D rotation around a normalized
            /// rotation `axis` of `angle` (in radians).
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_axis_angle(axis: $vec3, angle: f32) -> Self {
                Self {
                    transform: $transform::from_axis_angle(axis.0, angle),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation matrix containing a 3D rotation around the x axis of
            /// `angle` (in radians).
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_rotation_x(angle: f32) -> Self {
                Self {
                    transform: $transform::from_rotation_x(angle),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
            /// `angle` (in radians).
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline]
            pub fn from_rotation_y(angle: f32) -> Self {
                Self {
                    transform: $transform::from_rotation_y(angle),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
            /// `angle` (in radians).
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline]
            pub fn from_rotation_z(angle: f32) -> Self {
                Self {
                    transform: $transform::from_rotation_z(angle),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation from the given 3D `translation`.
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_translation(translation: $vec3) -> Self {
                Self {
                    transform: $transform::IDENTITY,
                    translate: translation.0.into(),
                }
            }

            /// Creates an affine transformation from a 3x3 matrix (expressing scale, shear and rotation)
            #[inline(always)]
            pub fn from_mat3(mat3: $mat3) -> Self {
                Self {
                    transform: mat3.0.into(),
                    translate: $translate::ZERO,
                }
            }

            /// Creates an affine transformation from a 3x3 matrix (expressing scale, shear and rotation)
            /// and a translation vector.
            ///
            /// Equivalent to `Affine3D::from_translation(translation) * Affine3D::from_mat3(mat3)`
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_mat3_translation(mat3: $mat3, translation: $vec3) -> Self {
                Self {
                    transform: mat3.0.into(),
                    translate: translation.0.into(),
                }
            }

            /// Creates an affine transformation from the given 3D `scale`, `rotation` and
            /// `translation`.
            ///
            /// Equivalent to `Affine3D::from_translation(translation) * Affine3D::from_quat(rotation) * Affine3D::from_scale(scale)`
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_scale_rotation_translation(
                scale: $vec3,
                rotation: $quat,
                translation: $vec3,
            ) -> Self {
                let rotation = $transform::from_quaternion(rotation.0.into());
                Self {
                    transform: $transform::from_cols(
                        rotation.x_axis.mul_scalar(scale.x),
                        rotation.y_axis.mul_scalar(scale.y),
                        rotation.z_axis.mul_scalar(scale.z),
                    ),
                    translate: translation.0.into(),
                }
            }

            /// Creates an affine transformation from the given 3D `rotation` and `translation`.
            ///
            /// Equivalent to `Affine3D::from_translation(translation) * Affine3D::from_quat(rotation)`
            ///
            /// The result can be used to transform 3D points and vectors.
            /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
            #[inline(always)]
            pub fn from_rotation_translation(rotation: $quat, translation: $vec3) -> Self {
                let rotation = $transform::from_quaternion(rotation.0.into());
                Self {
                    transform: $transform::from_cols(
                        rotation.x_axis,
                        rotation.y_axis,
                        rotation.z_axis,
                    ),
                    translate: translation.0.into(),
                }
            }

            /// The given `Mat4` must be an affine transform,
            /// i.e. contain no persepctive transform.
            #[inline]
            pub fn from_mat4(m: $mat4) -> Self {
                Self {
                    transform: $transform::from_cols(
                        m.x_axis.0.into(),
                        m.y_axis.0.into(),
                        m.z_axis.0.into(),
                    ),
                    translate: m.w_axis.0.into(),
                }
            }

            /// The first column.
            #[inline(always)]
            fn x_col(&self) -> $vec3 {
                $vec3(self.transform.x_axis.into())
            }

            /// The second column.
            #[inline(always)]
            fn y_col(&self) -> $vec3 {
                $vec3(self.transform.y_axis.into())
            }

            /// The third column.
            #[inline(always)]
            fn z_col(&self) -> $vec3 {
                $vec3(self.transform.z_axis.into())
            }

            /// The translation expressed by this transform.
            /// The translation is applied last, so is separatable from scale, shear and rotation.
            #[inline(always)]
            pub fn translation(&self) -> $vec3 {
                $vec3(self.translate.into())
            }

            /// Set the translation part of this transform.
            /// The translation is applied last, so is separatable from scale, shear and rotation.
            #[inline(always)]
            pub fn set_translation(&mut self, translation: $vec3) {
                self.translate = translation.0.into();
            }

            /// The scale, shear and rotation expressed by this transform.
            #[inline(always)]
            pub fn mat3(&self) -> $mat3 {
                $mat3(self.transform.into())
            }

            /// Set the scale, shear and rotation expressed by this transform.
            #[inline(always)]
            pub fn set_mat3(&mut self, m: $mat3) {
                self.transform = m.0.into();
            }

            /// Extracts `scale`, `rotation` and `translation` from `self`.
            ///
            /// The transform is expected to be non-degenerate and without shearing, or the output will be invalid.
            #[inline(always)]
            pub fn to_scale_rotation_translation(&self) -> ($vec3, $quat, $vec3) {
                // TODO: migrate to core module
                let det = self.determinant();
                glam_assert!(det != 0.0);

                let scale = $vec3::new(
                    self.transform.x_axis.length() * det.signum(),
                    self.transform.y_axis.length(),
                    self.transform.z_axis.length(),
                );

                glam_assert!(scale.cmpne($vec3::ZERO).all());

                let inv_scale = scale.recip();

                let rotation = $quat::from_rotation_mat3(&$mat3::from_cols(
                    $vec3(self.transform.x_axis.mul_scalar(inv_scale.x).into()),
                    $vec3(self.transform.y_axis.mul_scalar(inv_scale.y).into()),
                    $vec3(self.transform.z_axis.mul_scalar(inv_scale.z).into()),
                ));

                (scale, rotation, self.translation())
            }

            /// If this is zero, or close to zero, it is a singular transform (zero-scale on at least one axis).
            #[inline]
            fn determinant(&self) -> f32 {
                self.transform.determinant()
            }

            #[inline]
            fn look_to_lh(eye: $vec3, dir: $vec3, up: $vec3) -> Self {
                let f = dir.normalize();
                let s = up.cross(f).normalize();
                let u = f.cross(s);
                Self {
                    transform: $transform::from_cols(
                        $vec3::new(s.x, u.x, f.x).0.into(),
                        $vec3::new(s.y, u.y, f.y).0.into(),
                        $vec3::new(s.z, u.z, f.z).0.into(),
                    ),
                    translate: $vec3::new(-s.dot(eye), -u.dot(eye), -f.dot(eye)).0.into(),
                }
            }

            /// Creates a left-handed view transform using a camera position, an up direction, and a focal
            /// point.
            /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
            #[inline]
            pub fn look_at_lh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                glam_assert!(up.is_normalized());
                Self::look_to_lh(eye, center - eye, up)
            }

            /// Creates a right-handed view transform using a camera position, an up direction, and a focal
            /// point.
            /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
            #[inline]
            pub fn look_at_rh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                glam_assert!(up.is_normalized());
                Self::look_to_lh(eye, eye - center, up)
            }

            /// Transforms the given 3D points, applying shear, scale, rotation and translatio.
            #[inline(always)]
            pub fn transform_point3(&self, other: $vec3) -> $vec3 {
                $vec3(
                    (self
                        .transform
                        .x_axis
                        .mul_scalar(other.0.x)
                        .add(self.transform.y_axis.mul_scalar(other.0.y))
                        .add(self.transform.z_axis.mul_scalar(other.0.z))
                        .add(self.translate))
                    .into(),
                )
            }

            /// Transforms the give 3D vector, applying shear, scale and rotation (but NOT translation).
            ///
            /// To also apply translation, use [`Self::transform_point3`] instead.
            #[inline(always)]
            pub fn transform_vector3(&self, other: $vec3) -> $vec3 {
                $vec3(
                    (self
                        .transform
                        .x_axis
                        .mul_scalar(other.0.x)
                        .add(self.transform.y_axis.mul_scalar(other.0.y))
                        .add(self.transform.z_axis.mul_scalar(other.0.z)))
                    .into(),
                )
            }

            /// Returns `true` if, and only if, all elements are finite.
            /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
            #[inline]
            pub fn is_finite(&self) -> bool {
                self.transform.is_finite() && self.translate.is_finite()
            }

            /// Returns `true` if any elements are `NaN`.
            #[inline]
            pub fn is_nan(&self) -> bool {
                self.transform.is_nan() || self.translate.is_nan()
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
            pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
                self.transform.abs_diff_eq(&other.transform, max_abs_diff)
                    && self.translate.abs_diff_eq(other.translate, max_abs_diff)
            }

            /// Return the inverse of this transform.
            pub fn inverse(&self) -> Self {
                // // invert 3x3 matrix:
                // let x_row = self.y_col.cross(self.z_col);
                // let y_row = self.z_col.cross(self.x_col);
                // let z_row = self.x_col.cross(self.y_col);
                // let det = self.z_col.dot(z_row);
                // let inv_det = det.recip();
                // let x_row = x_row * inv_det;
                // let y_row = y_row * inv_det;
                // let z_row = z_row * inv_det;

                // let x_col = Vec3A::new(x_row.x, y_row.x, z_row.x);
                // let y_col = Vec3A::new(x_row.y, y_row.y, z_row.y);
                // let z_col = Vec3A::new(x_row.z, y_row.z, z_row.z);

                let transform = self.transform.inverse();
                // transform negative translation by the 3x3 inverse:
                let translate = transform.mul_vector(self.translate).neg();

                Self {
                    transform,
                    translate,
                }
            }
        }
    };
}

macro_rules! impl_affine3_traits {
    ($t:ty, $mat3:ident, $mat4:ident, $vec3:ident, $affine3:ident, $transform:ident, $translate:ident) => {
        impl Default for $affine3 {
            #[inline(always)]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl core::fmt::Debug for $affine3 {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct(stringify!($affine3d))
                    .field("transform", &$mat3(self.transform.into()))
                    .field("translate", &$vec3(self.translate.into()))
                    .finish()
            }
        }

        // impl From<$affine3> for $mat4 {
        //     #[inline]
        //     fn from(m: $affine3) -> $mat4 {
        //         $mat4::from_cols(
        //             m.transform.x_axis.extend(0.0),
        //             m.transform.y_axis.extend(0.0),
        //             m.transform.z_axis.extend(0.0),
        //             m.translate.extend(1.0),
        //         )
        //     }
        // }

        impl core::ops::Mul for $affine3 {
            type Output = $affine3;

            #[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
            #[inline(always)]
            fn mul(self, rhs: $affine3) -> Self::Output {
                use crate::core::traits::vector::Vector4;
                #[cfg(target_arch = "x86")]
                use core::arch::x86::*;
                #[cfg(target_arch = "x86_64")]
                use core::arch::x86_64::*;

                unsafe {
                    // TODO: optimize
                    let lhs_x_col: __m128 = self.transform.x_axis;
                    let lhs_y_col: __m128 = self.transform.y_axis;
                    let lhs_z_col: __m128 = self.transform.z_axis;
                    let lhs_w_col: __m128 = self.translate;

                    let rhs_x_col: __m128 = rhs.transform.x_axis;
                    let rhs_y_col: __m128 = rhs.transform.y_axis;
                    let rhs_z_col: __m128 = rhs.transform.z_axis;
                    let rhs_w_col: __m128 = rhs.translate;

                    // Based on https://github.com/microsoft/DirectXMath XMMatrixMultiply

                    let v_x = _mm_mul_ps(rhs_x_col.splat_x(), lhs_x_col);
                    let v_y = _mm_mul_ps(rhs_x_col.splat_y(), lhs_y_col);
                    let v_z = _mm_mul_ps(rhs_x_col.splat_z(), lhs_z_col);
                    let out_x_col = _mm_add_ps(_mm_add_ps(v_x, v_z), v_y);

                    let v_x = _mm_mul_ps(rhs_y_col.splat_x(), lhs_x_col);
                    let v_y = _mm_mul_ps(rhs_y_col.splat_y(), lhs_y_col);
                    let v_z = _mm_mul_ps(rhs_y_col.splat_z(), lhs_z_col);
                    let out_y_col = _mm_add_ps(_mm_add_ps(v_x, v_z), v_y);

                    let v_x = _mm_mul_ps(rhs_z_col.splat_x(), lhs_x_col);
                    let v_y = _mm_mul_ps(rhs_z_col.splat_y(), lhs_y_col);
                    let v_z = _mm_mul_ps(rhs_z_col.splat_z(), lhs_z_col);
                    let out_z_col = _mm_add_ps(_mm_add_ps(v_x, v_z), v_y);

                    let v_x = _mm_mul_ps(rhs_w_col.splat_x(), lhs_x_col);
                    let v_y = _mm_mul_ps(rhs_w_col.splat_y(), lhs_y_col);
                    let v_z = _mm_mul_ps(rhs_w_col.splat_z(), lhs_z_col);
                    let out_w_col = _mm_add_ps(_mm_add_ps(v_x, v_z), _mm_add_ps(v_y, lhs_w_col));

                    Self {
                        transform: $transform::from_cols(out_x_col, out_y_col, out_z_col),
                        translate: out_w_col,
                    }
                }
            }

            #[cfg(not(all(target_feature = "sse2", not(feature = "scalar-math"))))]
            #[inline(always)]
            fn mul(self, rhs: $affine3) -> Self::Output {
                Self {
                    transform: $transform::mul(rhs.transform),
                    translate: self.transform_point3(rhs.translate.into()).into(),
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

type TransformF32 = Vector3x3<__m128>;
type TranslateF32 = __m128;

define_affine3_struct!(Affine3, TransformF32, TranslateF32);
impl_affine3_methods!(
    f32,
    Mat3,
    Mat4,
    Quat,
    Vec3,
    Affine3,
    TransformF32,
    TranslateF32
);
impl_affine3_traits!(f32, Mat3, Mat4, Vec3, Affine3, TransformF32, TranslateF32);
