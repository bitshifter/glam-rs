// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if not is_scalar %}
    {% set is_simd = true %}
    {% if is_sse2 %}
        {% set simd_t = "__m128" %}
    {% elif is_wasm32 %}
        {% set simd_t = "v128" %}
    {% endif %}
{% endif %}

{% if scalar_t == "f32" %}
    {% set self_t = "Quat" %}
    {% set affine3_t = "Affine3A" %}
    {% set vec2_t = "Vec2" %}
    {% set vec3_t = "Vec3" %}
    {% set vec4_t = "Vec4" %}
    {% set mat3_t = "Mat3" %}
    {% set mat4_t = "Mat4" %}
{% elif scalar_t == "f64" %}
    {% set self_t = "DQuat" %}
    {% set affine3_t = "DAffine3" %}
    {% set vec2_t = "DVec2" %}
    {% set vec3_t = "DVec3" %}
    {% set vec4_t = "DVec4" %}
    {% set mat3_t = "DMat3" %}
    {% set mat4_t = "DMat4" %}
{% endif %}

use crate::{
    FloatEx,
    euler::{EulerFromQuaternion, EulerRot, EulerToQuaternion},
    {% if scalar_t == "f32" %}
        DQuat, Mat3, Mat4, Vec2, Vec3, Vec3A, Vec4,
    {% elif scalar_t == "f64" %}
        DMat3, DMat4, DVec2, DVec3, DVec4, Quat,
    {% endif %}
};

#[cfg(not(feature = "std"))]
use num_traits::Float;

{% if is_sse2 %}
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
{% elif is_wasm32 %}
use core::arch::wasm32::*;
{% endif %}

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, Deref, Div, Mul, MulAssign, Neg, Sub};

/// Creates a quaternion from `x`, `y`, `z` and `w` values.
///
/// This should generally not be called manually unless you know what you are doing. Use
/// one of the other constructors instead such as `identity` or `from_axis_angle`.
#[inline]
pub fn {{ self_t | lower }}(x: {{ scalar_t }}, y: {{ scalar_t }}, z: {{ scalar_t }}, w: {{ scalar_t }}) -> {{ self_t }} {
    {{ self_t }}::from_xyzw(x, y, z, w)
}

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
{%- if is_simd %}
///
/// This type is 16 byte aligned.
{%- endif %}
#[derive(Clone, Copy)]
{%- if is_scalar %}
{%- if scalar_t == "f32" %}
#[cfg_attr(not(any(feature = "scalar-math", target_arch = "spirv")), repr(C, align(16)))]
{%- endif %}
pub struct {{ self_t }}{
    x: {{ scalar_t }},
    y: {{ scalar_t }},
    z: {{ scalar_t }},
    w: {{ scalar_t }},
}
{%- else %}
#[repr(transparent)]
pub struct {{ self_t }}(pub(crate) {{ simd_t }});
{%- endif %}

impl {{ self_t }} {
    /// All zeros.
    const ZERO: Self = Self
        {% if is_scalar %}
            { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        {% else %}
            (const_f32x4!([0.0; 4]));
        {% endif %}

    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self
        {% if is_scalar %}
            { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        {% else %}
            (const_f32x4!([0.0, 0.0, 0.0, 1.0]));
        {% endif %}

    /// All NANs.
    pub const NAN: Self = Self
        {% if is_scalar %}
            { x: {{ scalar_t }}::NAN, y: {{ scalar_t }}::NAN, z: {{ scalar_t }}::NAN, w: {{ scalar_t }}::NAN, };
        {% else %}
            (const_f32x4!([{{ scalar_t }}::NAN; 4]));
        {% endif %}

    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing.
    /// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    pub fn from_xyzw(x: {{ scalar_t }}, y: {{ scalar_t }}, z: {{ scalar_t }}, w: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self { x, y, z, w }
        {% elif is_sse2 %}
            Self(unsafe { _mm_setr_ps(x, y, z, w) })
        {% elif is_wasm32 %}
            Self(f32x4(x, y, z, w))
        {% endif %}
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    pub fn from_array(a: [{{ scalar_t }}; 4]) -> Self {
        {% if is_sse2 %}
            Self(unsafe { _mm_loadu_ps(a.as_ptr()) })
        {% else %}
            Self::from_xyzw(a[0], a[1], a[2], a[3])
        {% endif %}
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    pub fn from_vec4(v: {{ vec4_t }}) -> Self {
        {% if is_scalar %}
            Self { x: v.x, y: v.y, z: v.z, w: v.w }
        {% else %}
            Self(v.0)
        {% endif %}
    }

    /// Creates a rotation quaternion from a slice.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn from_slice(slice: &[{{ scalar_t }}]) -> Self {
        {% if is_sse2 %}
            assert!(slice.len() >= 4);
            Self(unsafe { _mm_loadu_ps(slice.as_ptr()) })
        {% else %}
            Self::from_xyzw(slice[0], slice[1], slice[2], slice[3])
        {% endif %}
    }

    /// Writes the quaternion to an unaligned slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [{{ scalar_t }}]) {
        {% if is_sse2 %}
            assert!(slice.len() >= 4);
            unsafe { _mm_storeu_ps(slice.as_mut_ptr(), self.0) }
        {% else %}
            slice[0] = self.x;
            slice[1] = self.y;
            slice[2] = self.z;
            slice[3] = self.w;
        {% endif %}
    }

    /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    /// The axis must be normalized (unit-length).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_axis_angle(axis: {{ vec3_t }}, angle: {{ scalar_t }}) -> Self {
        glam_assert!(axis.is_normalized());
        let (s, c) = (angle * 0.5).sin_cos();
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    /// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[inline]
    pub fn from_scaled_axis(v: {{ vec3_t }}) -> Self {
        let length = v.length();
        if length == 0.0 {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: {{ scalar_t }}) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(s, 0.0, 0.0, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: {{ scalar_t }}) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(0.0, s, 0.0, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: {{ scalar_t }}) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(0.0, 0.0, s, c)
    }

    #[inline]
    /// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    pub fn from_euler(euler: EulerRot, a: {{ scalar_t }}, b: {{ scalar_t }}, c: {{ scalar_t }}) -> Self {
        euler.new_quat(a, b, c)
    }

    /// From the columns of a 3x3 rotation matrix.
    #[inline]
    pub(crate) fn from_rotation_axes(x_axis: {{ vec3_t }}, y_axis: {{ vec3_t }}, z_axis: {{ vec3_t }}) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XM$quaternionRotationMatrix`
        // TODO: sse2 version
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= 0.0 {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = 1.0 - m22;
            if dif10 <= 0.0 {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = 0.5 / four_xsq.sqrt();
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / four_ysq.sqrt();
                Self::from_xyzw(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = 1.0 + m22;
            if sum10 <= 0.0 {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = 0.5 / four_zsq.sqrt();
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / four_wsq.sqrt();
                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Creates a quaternion from a 3x3 rotation matrix.
    #[inline]
    pub fn from_mat3(mat: &{{ mat3_t }}) -> Self {
        Self::from_rotation_axes(
            mat.x_axis,
            mat.y_axis,
            mat.z_axis,
        )
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[inline]
    pub fn from_mat4(mat: &{{ mat4_t }}) -> Self {
        Self::from_rotation_axes(
            mat.x_axis.truncate(),
            mat.y_axis.truncate(),
            mat.z_axis.truncate(),
        )
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `from_rotation_arc(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc(from: {{ vec3_t }}, to: {{ vec3_t }}) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPS: {{ scalar_t }} = 1.0 - 2.0 * core::{{ scalar_t }}::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPS {
            // 0° singulary: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPS {
            // 180° singulary: from ≈ -to
            use core::{{ scalar_t }}::consts::PI; // half a turn = 𝛕/2 = 180°
            Self::from_axis_angle(from.any_orthonormal_vector(), PI)
        } else {
            let c = from.cross(to);
            Self::from_xyzw(c.x, c.y, c.z, 1.0 + dot).normalize()
        }
    }

    /// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    /// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    /// degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_rotation_arc_colinear(from: {{ vec3_t }}, to: {{ vec3_t }}) -> Self {
        if from.dot(to) < 0.0 {
            Self::from_rotation_arc(from, -to)
        } else {
            Self::from_rotation_arc(from, to)
        }
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    /// around the z axis. Will rotate at most 180 degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc_2d(from: {{ vec2_t }}, to: {{ vec2_t }}) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPSILON: {{ scalar_t }} = 1.0 - 2.0 * core::{{ scalar_t }}::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPSILON {
            // 0° singulary: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPSILON {
            // 180° singulary: from ≈ -to
            const COS_FRAC_PI_2: {{ scalar_t }} = 0.0;
            const SIN_FRAC_PI_2: {{ scalar_t }} = 1.0;
            // rotation around z by PI radians
            Self::from_xyzw(0.0, 0.0, SIN_FRAC_PI_2, COS_FRAC_PI_2)
        } else {
            // vector3 cross where z=0
            let z = from.x * to.y - to.x * from.y;
            let w = 1.0 + dot;
            // calculate length with x=0 and y=0 to normalize
            let len_rcp = 1.0 / (z * z + w * w).sqrt();
            Self::from_xyzw(0.0, 0.0, z * len_rcp, w * len_rcp)
        }
    }

    /// Returns the rotation axis and angle (in radians) of `self`.
    #[inline]
    pub fn to_axis_angle(self) -> ({{ vec3_t }}, {{ scalar_t }}) {
        const EPSILON: {{ scalar_t }} = 1.0e-8;
        const EPSILON_SQUARED: {{ scalar_t }} = EPSILON * EPSILON;
        let w = self.w;
        let angle = w.acos_approx() * 2.0;
        let scale_sq = {{ scalar_t }}::max(1.0 - w * w, 0.0);
        if scale_sq >= EPSILON_SQUARED {
            ({{ vec3_t }}::new(self.x, self.y, self.z) * scale_sq.sqrt().recip(), angle)
        } else {
            ({{ vec3_t }}::X, angle)
        }
    }

    /// Returns the rotation axis scaled by the rotation in radians.
    #[inline]
    pub fn to_scaled_axis(self) -> {{ vec3_t }} {
        let (axis, angle) = self.to_axis_angle();
        axis * angle
    }

    /// Returns the rotation angles for the given euler rotation sequence.
    #[inline]
    pub fn to_euler(self, euler: EulerRot) -> ({{ scalar_t }}, {{ scalar_t }}, {{ scalar_t }}) {
        euler.convert_quat(self)
    }

    /// `[x, y, z, w]`
    #[inline]
    pub fn to_array(&self) -> [{{ scalar_t }}; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part of the quaternion.
    #[inline]
    pub fn xyz(self) -> {{ vec3_t }} {
        {{ vec3_t }}::new(self.x, self.y, self.z)
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[must_use]
    #[inline]
    pub fn conjugate(self) -> Self {
        {% if is_scalar %}
            Self { x: -self.x, y: -self.y, z: -self.z, w: self.w }
        {% elif is_sse2 %}
            const SIGN: __m128 = const_f32x4!([-0.0, -0.0, -0.0, 0.0]);
            Self(unsafe { _mm_xor_ps(self.0, SIGN) })
        {% elif is_wasm32 %}
            const SIGN: v128 = const_f32x4!([-1.0, -1.0, -1.0, 1.0]);
            Self(f32x4_mul(self.0, SIGN))
        {% endif %}
    }

    /// Returns the inverse of a normalized quaternion.
    ///
    /// Typically quaternion inverse returns the conjugate of a normalized quaternion.
    /// Because `self` is assumed to already be unit length this method *does not* normalize
    /// before returning the conjugate.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn inverse(self) -> Self {
        glam_assert!(self.is_normalized());
        self.conjugate()
    }

    /// Computes the dot product of `self` and `rhs`. The dot product is
    /// equal to the cosine of the angle between two quaternion rotations.
    #[inline]
    pub fn dot(self, rhs: Self) -> {{ scalar_t }} {
        {{ vec4_t }}::from(self).dot({{ vec4_t }}::from(rhs))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    pub fn length(self) -> {{ scalar_t }} {
        {{ vec4_t }}::from(self).length()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `length()` as it avoids a square
    /// root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    pub fn length_squared(self) -> {{ scalar_t }} {
        {{ vec4_t }}::from(self).length_squared()
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> {{ scalar_t }} {
        {{ vec4_t }}::from(self).length_recip()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        Self::from_vec4({{ vec4_t }}::from(self).normalize())
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        {{ vec4_t }}::from(self).is_finite()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        {{ vec4_t }}::from(self).is_nan()
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        {{ vec4_t }}::from(self).is_normalized()
    }

    #[inline]
    pub fn is_near_identity(self) -> bool {
        // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
        let threshold_angle = 0.002_847_144_6;
        // Because of floating point precision, we cannot represent very small rotations.
        // The closest f32 to 1.0 that is not 1.0 itself yields:
        // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //
        // An error threshold of 1.e-6 is used by default.
        // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //
        // We don't really care about the angle value itself, only if it's close to 0.
        // This will happen whenever quat.w is close to 1.0.
        // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        // the shortest path.
        let positive_w_angle = self.w.abs().acos_approx() * 2.0;
        positive_w_angle < threshold_angle
    }

    /// Returns the angle (in radians) for the minimal rotation
    /// for transforming this quaternion into another.
    ///
    /// Both quaternions must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn angle_between(self, rhs: Self) -> {{ scalar_t }} {
        glam_assert!(self.is_normalized() && rhs.is_normalized());
        self.dot(rhs).abs().acos_approx() * 2.0
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two quaternions contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: {{ scalar_t }}) -> bool {
        {{ vec4_t }}::from(self).abs_diff_eq({{ vec4_t }}::from(rhs), max_abs_diff)
    }

    /// Performs a linear interpolation between `self` and `rhs` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `rhs`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[doc(alias = "mix")]
    pub fn lerp(self, end: Self, s: {{ scalar_t }}) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        {% if is_scalar %}
            let start = self;
            let dot = start.dot(end);
            let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
            let interpolated = start.add(end.mul(bias).sub(start).mul(s));
            interpolated.normalize()
        {% elif is_sse2 %}
            const NEG_ZERO: __m128 = const_f32x4!([-0.0; 4]);
            let start = self.0;
            let end = end.0;
            unsafe {
                let dot = crate::sse2::dot4_into_m128(start, end);
                // Calculate the bias, if the dot product is positive or zero, there is no bias
                // but if it is negative, we want to flip the 'end' rotation XYZW components
                let bias = _mm_and_ps(dot, NEG_ZERO);
                let interpolated = _mm_add_ps(
                    _mm_mul_ps(_mm_sub_ps(_mm_xor_ps(end, bias), start), _mm_set_ps1(s)),
                    start,
                );
                {{ self_t }}(interpolated).normalize()
            }
        {% elif is_wasm32 %}
            const NEG_ZERO: v128 = const_f32x4!([-0.0; 4]);
            let start = self.0;
            let end = end.0;
            let dot = crate::wasm32::dot4_into_v128(start, end);
            // Calculate the bias, if the dot product is positive or zero, there is no bias
            // but if it is negative, we want to flip the 'end' rotation XYZW components
            let bias = v128_and(dot, NEG_ZERO);
            let interpolated = f32x4_add(
                f32x4_mul(f32x4_sub(v128_xor(end, bias), start), f32x4_splat(s)),
                start,
            );
            {{ self_t }}(interpolated).normalize()
        {% endif %}
    }

    /// Performs a spherical linear interpolation between `self` and `end`
    /// based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn slerp(self, mut end: Self, s: {{ scalar_t }}) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        const DOT_THRESHOLD: {{ scalar_t }} = 0.9995;

        // Note that a rotation can be represented by two quaternions: `q` and
        // `-q`. The slerp path between `q` and `end` will be different from the
        // path between `-q` and `end`. One path will take the long way around and
        // one will take the short way. In order to correct for this, the `dot`
        // product between `self` and `end` should be positive. If the `dot`
        // product is negative, slerp between `self` and `-end`.
        let mut dot = self.dot(end);
        if dot < 0.0 {
            end = -end;
            dot = -dot;
        }

        if dot > DOT_THRESHOLD {
            // assumes lerp returns a normalized quaternion
            self.lerp(end, s)
        } else {
            let theta = dot.acos_approx();
            {% if is_scalar %}
                let scale1 = (theta * (1.0 - s)).sin();
                let scale2 = (theta * s).sin();
                let theta_sin = theta.sin();

                self.mul(scale1)
                    .add(end.mul(scale2))
                    .mul(theta_sin.recip())
            {% elif is_sse2 %}
                let x = 1.0 - s;
                let y = s;
                let z = 1.0;

                unsafe {
                    let tmp = _mm_mul_ps(_mm_set_ps1(theta), _mm_set_ps(0.0, z, y, x));
                    let tmp = crate::sse2::m128_sin(tmp);

                    let scale1 = _mm_shuffle_ps(tmp, tmp, 0b00_00_00_00);
                    let scale2 = _mm_shuffle_ps(tmp, tmp, 0b01_01_01_01);
                    let theta_sin = _mm_shuffle_ps(tmp, tmp, 0b10_10_10_10);

                    Self(_mm_div_ps(
                        _mm_add_ps(_mm_mul_ps(self.0, scale1), _mm_mul_ps(end.0, scale2)),
                        theta_sin,
                    ))
                }
            {% elif is_wasm32 %}
                // TODO: v128_sin is broken
                // let x = 1.0 - s;
                // let y = s;
                // let z = 1.0;
                // let w = 0.0;
                // let tmp = f32x4_mul(f32x4_splat(theta), f32x4(x, y, z, w));
                // let tmp = v128_sin(tmp);
                let x = (theta * (1.0 - s)).sin();
                let y = (theta * s).sin();
                let z = theta.sin();
                let w = 0.0;
                let tmp = f32x4(x, y, z, w);

                let scale1 = i32x4_shuffle::<0, 0, 4, 4>(tmp, tmp);
                let scale2 = i32x4_shuffle::<1, 1, 5, 5>(tmp, tmp);
                let theta_sin = i32x4_shuffle::<2, 2, 6, 6>(tmp, tmp);

                Self(f32x4_div(
                    f32x4_add(f32x4_mul(self.0, scale1), f32x4_mul(end.0, scale2)),
                    theta_sin,
                ))
            {% endif %}
        }
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn mul_vec3(self, rhs: {{ vec3_t }}) -> {{ vec3_t }} {
        glam_assert!(self.is_normalized());
        {% if is_scalar %}
            let w = self.w;
            let b = {{ vec3_t }}::new(self.x, self.y, self.z);
            let b2 = b.dot(b);
            rhs
                .mul(w * w - b2)
                .add(b.mul(rhs.dot(b) * 2.0))
                .add(b.cross(rhs).mul(w * 2.0))
        {% else %}
            self.mul_vec3a(rhs.into()).into()
        {% endif %}
    }

    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn mul_quat(self, rhs: Self) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(rhs.is_normalized());

        {% if is_scalar %}
            let (x0, y0, z0, w0) = self.into();
            let (x1, y1, z1, w1) = rhs.into();
            Self::from_xyzw(
                w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
                w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
                w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
                w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
            )
        {% elif is_sse2 %}
            // Based on https://github.com/nfrechette/rtm `rtm::quat_mul`
            const CONTROL_WZYX: __m128 = const_f32x4!([1.0, -1.0, 1.0, -1.0]);
            const CONTROL_ZWXY: __m128 = const_f32x4!([1.0, 1.0, -1.0, -1.0]);
            const CONTROL_YXWZ: __m128 = const_f32x4!([-1.0, 1.0, 1.0, -1.0]);

            let lhs = self.0;
            let rhs = rhs.0;

            unsafe {
                let r_xxxx = _mm_shuffle_ps(lhs, lhs, 0b00_00_00_00);
                let r_yyyy = _mm_shuffle_ps(lhs, lhs, 0b01_01_01_01);
                let r_zzzz = _mm_shuffle_ps(lhs, lhs, 0b10_10_10_10);
                let r_wwww = _mm_shuffle_ps(lhs, lhs, 0b11_11_11_11);

                let lxrw_lyrw_lzrw_lwrw = _mm_mul_ps(r_wwww, rhs);
                let l_wzyx = _mm_shuffle_ps(rhs, rhs, 0b00_01_10_11);

                let lwrx_lzrx_lyrx_lxrx = _mm_mul_ps(r_xxxx, l_wzyx);
                let l_zwxy = _mm_shuffle_ps(l_wzyx, l_wzyx, 0b10_11_00_01);

                let lwrx_nlzrx_lyrx_nlxrx = _mm_mul_ps(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX);

                let lzry_lwry_lxry_lyry = _mm_mul_ps(r_yyyy, l_zwxy);
                let l_yxwz = _mm_shuffle_ps(l_zwxy, l_zwxy, 0b00_01_10_11);

                let lzry_lwry_nlxry_nlyry = _mm_mul_ps(lzry_lwry_lxry_lyry, CONTROL_ZWXY);

                let lyrz_lxrz_lwrz_lzrz = _mm_mul_ps(r_zzzz, l_yxwz);
                let result0 = _mm_add_ps(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

                let nlyrz_lxrz_lwrz_wlzrz = _mm_mul_ps(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ);
                let result1 = _mm_add_ps(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);

                Self(_mm_add_ps(result0, result1))
            }
        {% elif is_wasm32 %}
            let lhs = self.0;
            let rhs = rhs.0;

            const CONTROL_WZYX: v128 = const_f32x4!([1.0, -1.0, 1.0, -1.0]);
            const CONTROL_ZWXY: v128 = const_f32x4!([1.0, 1.0, -1.0, -1.0]);
            const CONTROL_YXWZ: v128 = const_f32x4!([-1.0, 1.0, 1.0, -1.0]);

            let r_xxxx = i32x4_shuffle::<0, 0, 4, 4>(lhs, lhs);
            let r_yyyy = i32x4_shuffle::<1, 1, 5, 5>(lhs, lhs);
            let r_zzzz = i32x4_shuffle::<2, 2, 6, 6>(lhs, lhs);
            let r_wwww = i32x4_shuffle::<3, 3, 7, 7>(lhs, lhs);

            let lxrw_lyrw_lzrw_lwrw = f32x4_mul(r_wwww, rhs);
            let l_wzyx = i32x4_shuffle::<3, 2, 5, 4>(rhs, rhs);

            let lwrx_lzrx_lyrx_lxrx = f32x4_mul(r_xxxx, l_wzyx);
            let l_zwxy = i32x4_shuffle::<1, 0, 7, 6>(l_wzyx, l_wzyx);

            let lwrx_nlzrx_lyrx_nlxrx = f32x4_mul(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX);

            let lzry_lwry_lxry_lyry = f32x4_mul(r_yyyy, l_zwxy);
            let l_yxwz = i32x4_shuffle::<3, 2, 5, 4>(l_zwxy, l_zwxy);

            let lzry_lwry_nlxry_nlyry = f32x4_mul(lzry_lwry_lxry_lyry, CONTROL_ZWXY);

            let lyrz_lxrz_lwrz_lzrz = f32x4_mul(r_zzzz, l_yxwz);
            let result0 = f32x4_add(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

            let nlyrz_lxrz_lwrz_wlzrz = f32x4_mul(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ);
            let result1 = f32x4_add(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);

            Self(f32x4_add(result0, result1))
        {% endif %}
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[inline]
    pub fn from_affine3(a: &crate::{{ affine3_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self::from_rotation_axes(
            a.matrix3.x_axis.into(),
            a.matrix3.y_axis.into(),
            a.matrix3.z_axis.into(),
        )
    }

{% if scalar_t == "f32" %}
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    #[inline]
    pub fn mul_vec3a(self, rhs: Vec3A) -> Vec3A {
        {% if is_scalar %}
            self.mul_vec3(rhs.into()).into()
        {% elif is_sse2 %}
            unsafe {
                const TWO: __m128 = const_f32x4!([2.0; 4]);
                let w = _mm_shuffle_ps(self.0, self.0, 0b11_11_11_11);
                let b = self.0;
                let b2 = crate::sse2::dot3_into_m128(b, b);
                Vec3A(_mm_add_ps(
                    _mm_add_ps(
                        _mm_mul_ps(rhs.0, _mm_sub_ps(_mm_mul_ps(w, w), b2)),
                        _mm_mul_ps(b, _mm_mul_ps(crate::sse2::dot3_into_m128(rhs.0, b), TWO)),
                    ),
                    _mm_mul_ps(Vec3A(b).cross(rhs).into(), _mm_mul_ps(w, TWO)),
                ))
            }
        {% elif is_wasm32 %}
            const TWO: v128 = const_f32x4!([2.0; 4]);
            let w = i32x4_shuffle::<3, 3, 7, 7>(self.0, self.0);
            let b = self.0;
            let b2 = crate::wasm32::dot3_into_v128(b, b);
            Vec3A(f32x4_add(
                f32x4_add(
                    f32x4_mul(rhs.0, f32x4_sub(f32x4_mul(w, w), b2)),
                    f32x4_mul(b, f32x4_mul(crate::wasm32::dot3_into_v128(rhs.0, b), TWO)),
                ),
                f32x4_mul(Vec3A(b).cross(rhs).into(), f32x4_mul(w, TWO)),
            ))
        {% endif %}
    }

    #[inline]
    pub fn as_f64(self) -> DQuat {
        DQuat::from_xyzw(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }

{% elif scalar_t == "f64" %}
    #[inline]
    pub fn as_f32(self) -> Quat {
        Quat::from_xyzw(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }
{% endif %}
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for {{ self_t }} {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!({{ self_t }}))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for {{ self_t }} {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl Add<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    /// Adds two quaternions.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication.
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_vec4({{ vec4_t }}::from(self) + {{ vec4_t }}::from(rhs))
    }
}

impl Sub<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    /// Subtracts the `rhs` quaternion from `self`.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_vec4({{ vec4_t }}::from(self) - {{ vec4_t }}::from(rhs))
    }
}

impl Mul<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    /// Multiplies a quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    fn mul(self, rhs: {{ scalar_t }}) -> Self {
        Self::from_vec4({{ vec4_t }}::from(self) * rhs)
    }
}

impl Div<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    /// Divides a quaternion by a scalar value.
    /// The quotient is not guaranteed to be normalized.
    #[inline]
    fn div(self, rhs: {{ scalar_t }}) -> Self {
        Self::from_vec4({{ vec4_t }}::from(self) / rhs)
    }
}

impl Mul<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_quat(rhs)
    }
}

impl MulAssign<{{ self_t }}> for {{ self_t }} {
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul_quat(rhs);
    }
}

impl Mul<{{ vec3_t }}> for {{ self_t }} {
    type Output = {{ vec3_t }};
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: {{ vec3_t }}) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Neg for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Default for {{ self_t }} {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for {{ self_t }} {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        {{ vec4_t }}::from(*self).eq(&{{ vec4_t }}::from(*rhs))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[{{ scalar_t }}; 4]> for {{ self_t }} {
    #[inline]
    fn as_ref(&self) -> &[{{ scalar_t }}; 4] {
        unsafe { &*(self as *const Self as *const [{{ scalar_t }}; 4]) }
    }
}

impl<'a> Sum<&'a Self> for {{ self_t }} {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for {{ self_t }} {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

{% if scalar_t == "f32" %}
impl Mul<Vec3A> for Quat {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Self::Output {
        self.mul_vec3a(rhs)
    }
}
{% endif %}

impl From<{{ self_t }}> for {{ vec4_t }} {
    #[inline]
    fn from(q: {{ self_t }}) -> Self {
        {% if is_scalar %}
            Self::new(q.x, q.y, q.z, q.w)
        {% else %}
            Self(q.0)
        {% endif %}
    }
}

impl From<{{ self_t }}> for ({{ scalar_t }}, {{ scalar_t }}, {{ scalar_t }}, {{ scalar_t }}) {
    #[inline]
    fn from(q: {{ self_t }}) -> Self {
        {% if is_scalar %}
            (q.x, q.y, q.z, q.w)
        {% else %}
            {{ vec4_t }}::from(q).into()
        {% endif %}
    }
}

impl From<{{ self_t }}> for [{{ scalar_t }}; 4] {
    #[inline]
    fn from(q: {{ self_t }}) -> Self {
        {% if is_scalar %}
            [q.x, q.y, q.z, q.w]
        {% else %}
            {{ vec4_t }}::from(q).into()
        {% endif %}
    }
}

{% if not is_scalar %}
impl From<{{ self_t }}> for {{ simd_t }} {
    // TODO: write test
    #[inline]
    fn from(q: {{ self_t }}) -> Self {
        {% if is_scalar %}
            Self { x: q.x, y: q.y, z: q.z, w: q.w }
        {% else %}
            q.0
        {% endif %}
    }
}
{% endif %}

impl Deref for {{ self_t }} {
    type Target = crate::deref::XYZW<{{ scalar_t }}>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

