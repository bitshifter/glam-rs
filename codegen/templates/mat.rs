// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if scalar_t == "f32" %}
    {% set vecn_t = "Vec" ~ dim %}
    {% if dim == 3 and is_align %}
        {% set self_t = "Mat3A" %}
        {% set col_t = "Vec3A" %}
    {% else %}
        {% set self_t = "Mat" ~ dim %}
        {% set col_t = vecn_t %}
    {% endif %}
    {% set quat_t = "Quat" %}
    {% set affine2_t = "Affine2" %}
    {% set affine3_t = "Affine3A" %}
    {% set vec2_t = "Vec2" %}
    {% set vec3_t = "Vec3" %}
    {% set vec3a_t = "Vec3A" %}
    {% set vec4_t = "Vec4" %}
    {% set mat2_t = "Mat2" %}
    {% set mat3_t = "Mat3" %}
    {% set mat3a_t = "Mat3A" %}
    {% set mat4_t = "Mat4" %}
{% elif scalar_t == "f64" %}
    {% set vecn_t = "DVec" ~ dim %}
    {% set self_t = "DMat" ~ dim %}
    {% set col_t = vecn_t %}
    {% set quat_t = "DQuat" %}
    {% set affine2_t = "DAffine2" %}
    {% set affine3_t = "DAffine3" %}
    {% set vec2_t = "DVec2" %}
    {% set vec3_t = "DVec3" %}
    {% set vec4_t = "DVec4" %}
    {% set mat2_t = "DMat2" %}
    {% set mat3_t = "DMat3" %}
    {% set mat4_t = "DMat4" %}
{% endif %}

{% if dim == 2 %}
    {% set deref_t = "Columns2::<" ~ col_t ~ ">" %}
    {% set inner_t = "Columns2::<XY<" ~ scalar_t ~ ">>" %}
{% elif dim == 3 %}
    {% set deref_t = "Columns3::<" ~ col_t ~ ">" %}
    {% set inner_t = "Columns3::<XYZ<" ~ scalar_t ~ ">>" %}
{% elif dim == 4 %}
    {% set deref_t = "Columns4::<" ~ col_t ~ ">" %}
    {% set inner_t = "Columns4::<XYZW<" ~ scalar_t ~ ">>" %}
{% endif %}

{% if self_t == "Mat2" %}
    {% if not is_scalar %}
        {% set is_simd = true %}
        {% if is_sse2 %}
            {% set inner_t = "__m128" %}
        {% elif is_wasm32 %}
            {% set inner_t = "v128" %}
        {% endif %}
    {% endif %}
{% elif self_t == "Mat3A" %}
    {% if is_sse2 %}
        {% set inner_t = "Columns3::<__m128>" %}
    {% elif is_wasm32 %}
        {% set inner_t = "Columns3::<v128>" %}
    {% else %}
        {% set inner_t = "Columns3::<XYZF32A16>" %}
    {% endif %}
{% elif self_t == "Mat4" %}
    {% if is_sse2 %}
        {% set inner_t = "Columns4::<__m128>" %}
    {% elif is_wasm32 %}
        {% set inner_t = "Columns4::<v128>" %}
    {% endif %}
{% endif %}

{% set size = dim * dim %}
{% set nxn = dim ~ "x" ~ dim %}

{% set components = ["x", "y", "z", "w"] | slice(end = dim) %}
{% set axes = ["x_axis", "y_axis", "z_axis", "w_axis"] | slice(end = dim) %}

use crate::{
    core::{
        storage::*,
        traits::{
            matrix::{FloatMatrix{{ nxn }}, Matrix{{ nxn }}, MatrixConst},
            {% if dim == 4 %}
                projection::ProjectionMatrix,
            {% endif %}
        },
    },
{% if scalar_t == "f32" %}
    DMat{{ dim }},
{% elif scalar_t == "f64" %}
    Mat{{ dim }},
{% endif %}
{% if dim == 2 %}
    {{ mat3_t }}, {{ vec2_t }},
{% elif dim == 3 %}
    EulerRot,
    {{ mat2_t }}, {{ mat4_t }}, {{ quat_t }}, {{ vec2_t }}, {{ col_t }},
    {% if is_align %}
        {{ mat3_t }}, {{ vec3_t }},
    {% elif scalar_t == "f32" %}
        {{ mat3a_t }}, {{ vec3a_t }},
    {% endif %}
{% elif dim == 4 %}
    EulerRot,
    {{ mat3_t }}, {{ quat_t }}, {{ vec3_t }}, {{ col_t }},
    {% if scalar_t == "f32" %}
        {{ vec3a_t }},
    {% endif %}
{% endif %}
};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

{% if is_sse2 %}
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
{% elif is_wasm32 %}
use core::arch::wasm32::*;
{% endif %}

/// Creates a {{ nxn }} matrix from column vectors.
#[inline(always)]
pub fn {{ self_t | lower }}(
    {% for axis in axes %}
        {{ axis }}: {{ col_t }},
    {% endfor %}
) -> {{ self_t }} {
    {{ self_t }}::from_cols({{ axes | join(sep=",") }})
}

/// A {{ nxn }} column major matrix.
{%- if dim == 3 %}
///
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations. If you are primarily dealing with 2D affine transformations the
/// [`{{ affine2_t }}`](crate::{{ affine2_t }}) type is much faster and more space efficient than
/// using a 3x3 matrix.
///
/// Linear transformations including 3D rotation and scale can be created using methods
/// such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
/// [`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
/// [`Self::from_rotation_z()`].
///
/// The resulting matrices can be use to transform 3D vectors using regular vector
/// multiplication.
///
/// Affine transformations including 2D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
/// [`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
///
/// The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
/// are provided for performing affine transforms on 2D vectors and points. These multiply
/// 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
/// vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
{%- elif dim == 4 %}
///
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and
/// perspective projections. If you are primarily dealing with 3D affine transformations
/// considering using [`{{affine3_t}}`](crate::{{ affine3_t }}) which is faster than a 4x4 matrix
/// for some affine operations.
///
/// Affine transformations including 3D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
/// [`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
///
/// Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
/// left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
/// systems. The resulting matrix is also an affine transformation.
///
/// The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
/// are provided for performing affine transformations on 3D vectors and points. These
/// multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
/// for vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
///
/// Perspective projections can be created using methods such as
/// [`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
/// [`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
/// [`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
/// [`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
///
/// The resulting perspective project can be use to transform 3D vectors as points with
/// perspective correction using the [`Self::project_point3()`] convenience method.
{%- endif %}
#[derive(Clone, Copy)]
{%- if self_t == "Mat4" and is_scalar %}
#[cfg_attr(
    any(
        not(any(feature = "scalar-math", target_arch = "spirv")),
        feature = "cuda"),
    repr(C, align(16))
)]
#[cfg_attr(
    all(
        any(feature = "scalar-math", target_arch = "spirv"),
        not(feature = "cuda")),
    repr(transparent)
)]
{%- elif self_t == "Mat2" and is_scalar %}
#[cfg_attr(not(any(feature = "scalar-math", target_arch = "spirv")), repr(C, align(16)))]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(all(any(feature = "scalar-math", target_arch = "spirv"), not(feature = "cuda")), repr(transparent))]
{%- elif self_t == "DMat2" or self_t == "DMat4" %}
#[cfg_attr(feature = "cuda", repr(align(16)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
{% else %}
#[repr(transparent)]
{% endif -%}
pub struct {{ self_t }}(pub(crate) {{ inner_t }});

impl {{ self_t }} {
    /// A {{ nxn }} matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self({{ inner_t }}::ZERO);

    /// A {{ nxn }} identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self({{ inner_t }}::IDENTITY);

    /// All NAN:s.
    pub const NAN: Self = Self(<{{ inner_t }} as crate::core::traits::scalar::NanConstEx>::NAN);

    /// Creates a {{ nxn }} matrix from two column vectors.
    #[inline(always)]
    pub fn from_cols(
        {% for axis in axes %}
            {{ axis }}: {{ col_t }},
        {% endfor %}
    ) -> Self {
        Self({{ inner_t }}::from_cols(
            {% for axis in axes %}
                {{ axis }}.0,
            {% endfor %}
            )
        )
    }

    /// Creates a {{ nxn }} matrix from a `[{{ scalar_t }}; {{ size }}]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline(always)]
    pub fn from_cols_array(m: &[{{ scalar_t }}; {{ size }}]) -> Self {
        Self({{ inner_t }}::from_cols_array(m))
    }

    /// Creates a `[{{ scalar_t }}; {{ size }}]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline(always)]
    pub fn to_cols_array(&self) -> [{{ scalar_t }}; {{ size }}] {
        self.0.to_cols_array()
    }

    /// Creates a {{ nxn }} matrix from a `[[{{ scalar_t }}; {{ dim }}]; {{ dim }}]` {{ dim }}D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline(always)]
    pub fn from_cols_array_2d(m: &[[{{ scalar_t }}; {{ dim }}]; {{ dim }}]) -> Self {
        Self({{ inner_t }}::from_cols_array_2d(m))
    }

    /// Creates a `[[{{ scalar_t }}; {{ dim }}]; {{ dim }}]` {{ dim }}D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline(always)]
    pub fn to_cols_array_2d(&self) -> [[{{ scalar_t }}; {{ dim }}]; {{ dim }}] {
        self.0.to_cols_array_2d()
    }

    /// Creates a {{ nxn }} matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline(always)]
    pub fn from_diagonal(diagonal: {{ vecn_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self({{ inner_t }}::from_diagonal(diagonal.0.into()))
    }

{% if dim == 2 %}
    /// Creates a {{ nxn }} matrix containing the combining non-uniform `scale` and rotation of
    /// `angle` (in radians).
    #[inline(always)]
    pub fn from_scale_angle(scale: {{ col_t }}, angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_scale_angle(scale.0, angle))
    }

    /// Creates a {{ nxn }} matrix containing a rotation of `angle` (in radians).
    #[inline(always)]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_angle(angle))
    }

    /// Creates a {{ nxn }} matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline(always)]
    pub fn from_mat3(m: {{ mat3_t }}) -> Self {
        Self::from_cols({{ col_t }}(m.x_axis.0.into()), {{ col_t }}(m.y_axis.0.into()))
    }
{% elif dim == 3 %}
    /// Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
    pub fn from_mat4(m: {{ mat4_t }}) -> Self {
        #[allow(clippy::useless_conversion)]
        Self::from_cols(
            {{ col_t }}(m.x_axis.0.into()),
            {{ col_t }}(m.y_axis.0.into()),
            {{ col_t }}(m.z_axis.0.into()),
        )
    }

    /// Creates a 3D rotation matrix from the given quaternion.
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_quat(rotation: {{ quat_t }}) -> Self {
        // TODO: SIMD?
        #[allow(clippy::useless_conversion)]
        Self({{ inner_t }}::from_quaternion(rotation.0.into()))
    }

    /// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    /// radians).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_axis_angle(axis: {{ vec3_t }}, angle: {{ scalar_t }}) -> Self {
        Self(FloatMatrix3x3::from_axis_angle(axis.0, angle))
    }

    #[inline(always)]
    /// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    /// radians).
    pub fn from_euler(order: EulerRot, a: {{ scalar_t }}, b: {{ scalar_t }}, c: {{ scalar_t }}) -> Self {
        let quat = {{ quat_t }}::from_euler(order, a, b, c);
        Self::from_quat(quat)
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[inline(always)]
    pub fn from_rotation_x(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_rotation_x(angle))
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[inline(always)]
    pub fn from_rotation_y(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_rotation_y(angle))
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[inline(always)]
    pub fn from_rotation_z(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_rotation_z(angle))
    }

    /// Creates an affine transformation matrix from the given 2D `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_translation(translation: {{ vec2_t }}) -> Self {
        Self(Matrix3x3::from_translation(translation.0))
    }

    /// Creates an affine transformation matrix from the given 2D rotation `angle` (in
    /// radians).
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        Self(FloatMatrix3x3::from_angle(angle))
    }

    /// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    /// radians) and `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_scale_angle_translation(scale: {{ vec2_t }}, angle: {{ scalar_t }}, translation: {{ vec2_t }}) -> Self {
        Self(FloatMatrix3x3::from_scale_angle_translation(
                scale.0,
                angle,
                translation.0,
        ))
    }

    /// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    /// # Panics
    ///
    /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_scale(scale: {{ vec2_t }}) -> Self {
        Self(Matrix3x3::from_scale(scale.0))
    }

    /// Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_mat2(m: {{ mat2_t }}) -> Self {
        Self::from_cols((m.x_axis, 0.0).into(), (m.y_axis, 0.0).into(), {{ col_t }}::Z)
    }

{% elif dim == 4 %}
    /// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_scale_rotation_translation(
        scale: {{ vec3_t }},
        rotation: {{ quat_t }},
        translation: {{ vec3_t }},
    ) -> Self {
        Self({{ inner_t }}::from_scale_quaternion_translation(
            scale.0,
            rotation.0,
            translation.0,
        ))
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_rotation_translation(rotation: {{ quat_t }}, translation: {{ vec3_t }}) -> Self {
        Self({{ inner_t }}::from_quaternion_translation(
            rotation.0,
            translation.0,
        ))
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
    /// expected to be a 3D affine transformation matrix otherwise the output will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero or if the resulting scale vector
    /// contains any zero elements when `glam_assert` is enabled.
    #[inline(always)]
    pub fn to_scale_rotation_translation(&self) -> ({{ vec3_t }}, {{ quat_t }}, {{ vec3_t }}) {
        let (scale, rotation, translation) = self.0.to_scale_quaternion_translation();
        ({{ vec3_t }}(scale), {{ quat_t }}(rotation), {{ vec3_t }}(translation))
    }

    /// Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_quat(rotation: {{ quat_t }}) -> Self {
        Self({{ inner_t }}::from_quaternion(rotation.0))
    }

    /// Creates an affine transformation matrix from the given 3x3 linear transformation
    /// matrix.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_mat3(m: {{ mat3_t }}) -> Self {
        Self::from_cols(
            (m.x_axis, 0.0).into(),
            (m.y_axis, 0.0).into(),
            (m.z_axis, 0.0).into(),
            {{ vec4_t }}::W,
        )
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_translation(translation: {{ vec3_t }}) -> Self {
        Self({{ inner_t }}::from_translation(translation.0))
    }

    /// Creates an affine transformation matrix containing a 3D rotation around a normalized
    /// rotation `axis` of `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_axis_angle(axis: {{ vec3_t }}, angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_axis_angle(axis.0, angle))
    }

    #[inline(always)]
    /// Creates a affine transformation matrix containing a rotation from the given euler
    /// rotation sequence and angles (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    pub fn from_euler(order: EulerRot, a: {{ scalar_t }}, b: {{ scalar_t }}, c: {{ scalar_t }}) -> Self {
        let quat = {{ quat_t }}::from_euler(order, a, b, c);
        Self::from_quat(quat)
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_rotation_x(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_rotation_x(angle))
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_rotation_y(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_rotation_y(angle))
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_rotation_z(angle: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::from_rotation_z(angle))
    }

    /// Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_scale(scale: {{ vec3_t }}) -> Self {
        Self({{ inner_t }}::from_scale(scale.0))
    }
{% endif %}

    /// Creates a {{ nxn }} matrix from the first {{ size }} values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than {{ size }} elements long.
    #[inline(always)]
    pub fn from_cols_slice(slice: &[{{ scalar_t }}]) -> Self {
        Self({{ inner_t }}::from_cols_slice(slice))
    }

    /// Writes the columns of `self` to the first {{ size }} elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than {{ size }} elements long.
    #[inline(always)]
    pub fn write_cols_to_slice(self, slice: &mut [{{ scalar_t }}]) {
        {{ inner_t }}::write_cols_to_slice(&self.0, slice)
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than {{ dim - 1 }}.
    #[inline]
    pub fn col(&self, index: usize) -> {{ col_t }} {
        match index {
            {% for axis in axes %}
                {{ loop.index0 }} => self.{{ axis }},
            {%- endfor %}
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns a mutable reference to the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than {{ dim - 1 }}.
    #[inline]
    pub fn col_mut(&mut self, index: usize) -> &mut {{ col_t }} {
        match index {
            {% for axis in axes %}
                {{ loop.index0 }} => &mut self.{{ axis }},
            {%- endfor %}
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns the matrix row for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than {{ dim - 1 }}.
    #[inline]
    pub fn row(&self, index: usize) -> {{ col_t }} {
        match index {
            {% for i in range(end=dim) %}
                {{ i }} => {{ col_t }}::new(
                    {% for axis in axes %}
                        self.{{ axis }}.{{ components[i] }},
                    {%- endfor %}
                ),
            {%- endfor %}
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        // TODO
        self.x_axis.is_finite() && self.y_axis.is_finite()
        {% for axis in axes | slice(start=2) %}
            && self.{{ axis }}.is_finite()
        {% endfor %}
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan()
        {% for axis in axes | slice(start=2) %}
            || self.{{ axis }}.is_nan()
        {% endfor %}
    }

    /// Returns the transpose of `self`.
    #[must_use]
    #[inline(always)]
    pub fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }

    /// Returns the determinant of `self`.
    #[inline(always)]
    pub fn determinant(&self) -> {{ scalar_t }} {
        self.0.determinant()
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

{% if dim == 3 %}
    /// Transforms the given 2D vector as a point.
    ///
    /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `1`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline(always)]
    pub fn transform_point2(&self, other: {{ vec2_t }}) -> {{ vec2_t }} {
        {{ mat2_t }}::from_cols({{ vec2_t }}(self.x_axis.0.into()), {{ vec2_t }}(self.y_axis.0.into())) * other
            + {{ vec2_t }}(self.z_axis.0.into())
    }

    /// Rotates the given 2D vector.
    ///
    /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline(always)]
    pub fn transform_vector2(&self, other: {{ vec2_t }}) -> {{ vec2_t }} {
        {{ mat2_t }}::from_cols({{ vec2_t }}(self.x_axis.0.into()), {{ vec2_t }}(self.y_axis.0.into())) * other
    }

{% elif dim == 4 %}
    /// Creates a left-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn look_at_lh(eye: {{ vec3_t }}, center: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        Self({{ inner_t }}::look_at_lh(eye.0, center.0, up.0))
    }

    /// Creates a right-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn look_at_rh(eye: {{ vec3_t }}, center: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        Self({{ inner_t }}::look_at_rh(eye.0, center.0, up.0))
    }

    /// Creates a right-handed perspective projection matrix with [-1,1] depth range.
    /// This is the same as the OpenGL `gluPerspective` function.
    /// See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
    #[inline(always)]
    pub fn perspective_rh_gl(
        fov_y_radians: {{ scalar_t }},
        aspect_ratio: {{ scalar_t }},
        z_near: {{ scalar_t }},
        z_far: {{ scalar_t }},
    ) -> Self {
        Self({{ inner_t }}::perspective_rh_gl(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }

    /// Creates a left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    /// enabled.
    #[inline(always)]
    pub fn perspective_lh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}, z_far: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::perspective_lh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }

    /// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    /// enabled.
    #[inline(always)]
    pub fn perspective_rh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}, z_far: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::perspective_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }

    /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline(always)]
    pub fn perspective_infinite_lh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::perspective_infinite_lh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }

    /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline(always)]
    pub fn perspective_infinite_reverse_lh(
        fov_y_radians: {{ scalar_t }},
        aspect_ratio: {{ scalar_t }},
        z_near: {{ scalar_t }},
    ) -> Self {
        Self({{ inner_t }}::perspective_infinite_reverse_lh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }

    /// Creates an infinite right-handed perspective projection matrix with
    /// `[0,1]` depth range.
    #[inline(always)]
    pub fn perspective_infinite_rh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::perspective_infinite_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }

    /// Creates an infinite reverse right-handed perspective projection matrix
    /// with `[0,1]` depth range.
    #[inline(always)]
    pub fn perspective_infinite_reverse_rh(
        fov_y_radians: {{ scalar_t }},
        aspect_ratio: {{ scalar_t }},
        z_near: {{ scalar_t }},
    ) -> Self {
        Self({{ inner_t }}::perspective_infinite_reverse_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
        ))
    }

    /// Creates a right-handed orthographic projection matrix with `[-1,1]` depth
    /// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    /// See
    /// <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
    #[inline(always)]
    pub fn orthographic_rh_gl(
        left: {{ scalar_t }},
        right: {{ scalar_t }},
        bottom: {{ scalar_t }},
        top: {{ scalar_t }},
        near: {{ scalar_t }},
        far: {{ scalar_t }},
    ) -> Self {
        Self({{ inner_t }}::orthographic_rh_gl(
            left, right, bottom, top, near, far,
        ))
    }

    /// Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
    #[inline(always)]
    pub fn orthographic_lh(
        left: {{ scalar_t }},
        right: {{ scalar_t }},
        bottom: {{ scalar_t }},
        top: {{ scalar_t }},
        near: {{ scalar_t }},
        far: {{ scalar_t }},
    ) -> Self {
        Self({{ inner_t }}::orthographic_lh(left, right, bottom, top, near, far))
    }

    /// Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
    #[inline(always)]
    pub fn orthographic_rh(
        left: {{ scalar_t }},
        right: {{ scalar_t }},
        bottom: {{ scalar_t }},
        top: {{ scalar_t }},
        near: {{ scalar_t }},
        far: {{ scalar_t }},
    ) -> Self {
        Self({{ inner_t }}::orthographic_rh(left, right, bottom, top, near, far))
    }

    /// Transforms the given 3D vector as a point, applying perspective correction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    /// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
    ///
    /// This method assumes that `self` contains a projective transform.
    #[inline]
    pub fn project_point3(&self, other: {{ vec3_t }}) -> {{ vec3_t }} {
        {{ vec3_t }}(self.0.project_point3(other.0))
    }

    /// Transforms the given 3D vector as a point.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    /// `1.0`.
    ///
    /// This method assumes that `self` contains a valid affine transform. It does not perform
    /// a persective divide, if `self` contains a perspective transform, or if you are unsure,
    /// the [`Self::project_point3()`] method should be used instead.
    ///
    /// # Panics
    ///
    /// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
    #[inline]
    pub fn transform_point3(&self, other: {{ vec3_t }}) -> {{ vec3_t }} {
        glam_assert!(self.row(3) == {{ vec4_t }}::W);
        {{ vec3_t }}(self.0.transform_point3(other.0))
    }

    /// Transforms the give 3D vector as a direction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    /// `0.0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// # Panics
    ///
    /// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
    #[inline]
    pub fn transform_vector3(&self, other: {{ vec3_t }}) -> {{ vec3_t }} {
        glam_assert!(self.row(3) == {{ vec4_t }}::W);
        {{ vec3_t }}(self.0.transform_vector3(other.0))
    }

{% endif %}

{% if self_t == "Mat4" %}
    /// Transforms the given `Vec3A` as 3D point.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `1.0`.
    #[inline(always)]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        #[allow(clippy::useless_conversion)]
        Vec3A(self.0.transform_float4_as_point3(other.0.into()).into())
    }

    /// Transforms the give `Vec3A` as 3D vector.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `0.0`.
    #[inline(always)]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
        #[allow(clippy::useless_conversion)]
        Vec3A(self.0.transform_float4_as_vector3(other.0.into()).into())
    }
{% endif %}

{% if self_t == "Mat3A" %}
    /// Transforms a `Vec3`.
    #[inline(always)]
    pub fn mul_vec3(&self, other: Vec3) -> Vec3 {
        Vec3(self.0.mul_vector(other.0.into()).into())
    }

    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, other: Vec3A) -> Vec3A {
        Vec3A(self.0.mul_vector(other.0))
    }
{% else %}
    /// Transforms a {{ dim }}D vector.
    #[inline(always)]
    pub fn mul_vec{{ dim }}(&self, other: {{ col_t }}) -> {{ col_t }} {
        {{ col_t }}(self.0.mul_vector(other.0))
    }
{% endif %}

{% if self_t == "Mat3" %}
    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, other: Vec3A) -> Vec3A {
        self.mul_vec3(other.into()).into()
    }
{% endif %}

    /// Multiplies two {{ nxn }} matrices.
    #[inline(always)]
    pub fn mul_mat{{ dim }}(&self, other: &Self) -> Self {
        Self(self.0.mul_matrix(&other.0))
    }

    /// Adds two {{ nxn }} matrices.
    #[inline(always)]
    pub fn add_mat{{ dim }}(&self, other: &Self) -> Self {
        Self(self.0.add_matrix(&other.0))
    }

    /// Subtracts two {{ nxn }} matrices.
    #[inline(always)]
    pub fn sub_mat{{ dim }}(&self, other: &Self) -> Self {
        Self(self.0.sub_matrix(&other.0))
    }

    /// Multiplies a {{ nxn }} matrix by a scalar.
    #[inline(always)]
    pub fn mul_scalar(&self, other: {{ scalar_t }}) -> Self {
        Self(self.0.mul_scalar(other))
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two matrices contain similar elements. It works best
    /// when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline(always)]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: {{ scalar_t }}) -> bool {
        self.0.abs_diff_eq(&other.0, max_abs_diff)
    }

    {% if scalar_t == "f32" %}
        #[inline(always)]
        pub fn as_dmat{{ dim }}(&self) -> DMat{{ dim }} {
            DMat{{ dim }}::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.as_dvec{{ dim }}(),
                {% endfor %}
            )
        }
    {% elif scalar_t == "f64" %}
        #[inline(always)]
        pub fn as_mat{{ dim }}(&self) -> Mat{{ dim }} {
            Mat{{ dim }}::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.as_vec{{ dim }}(),
                {% endfor %}
            )
        }
    {% endif %}
}

impl Default for {{ self_t }} {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0.add_matrix(&other.0))
    }
}

impl AddAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add_matrix(&other.0);
    }
}

impl Sub<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub_matrix(&other.0))
    }
}

impl SubAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub_matrix(&other.0);
    }
}

impl Neg for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(self.0.neg_matrix())
    }
}

impl Mul<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0.mul_matrix(&other.0))
    }
}

impl MulAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul_matrix(&other.0);
    }
}

impl Mul<{{ col_t }}> for {{ self_t }} {
    type Output = {{ col_t }};
    #[inline(always)]
    fn mul(self, other: {{ col_t }}) -> Self::Output {
        {{ col_t }}(self.0.mul_vector(other.0))
    }
}

impl Mul<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline(always)]
    fn mul(self, other: {{ self_t }}) -> Self::Output {
        {{ self_t }}(other.0.mul_scalar(self))
    }
}

impl Mul<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: {{ scalar_t }}) -> Self::Output {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline(always)]
    fn mul_assign(&mut self, other: {{ scalar_t }}) {
        self.0 = self.0.mul_scalar(other);
    }
}

{% if self_t == "Mat3" %}
impl Mul<Vec3A> for Mat3 {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Vec3A {
        self.mul_vec3a(other)
    }
}

impl From<Mat3A> for Mat3 {
    #[inline(always)]
    fn from(m: Mat3A) -> Self {
        Self(m.0.into())
    }
}
{% elif self_t == "Mat3A" %}
impl Mul<Vec3> for Mat3A {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        #[allow(clippy::useless_conversion)]
        self.mul_vec3(other.into()).into()
    }
}

impl From<Mat3> for Mat3A {
    #[inline(always)]
    fn from(m: Mat3) -> Self {
        Self(m.0.into())
    }
}
{% endif %}

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

impl PartialEq for {{ self_t }} {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x_axis.eq(&other.x_axis) && self.y_axis.eq(&other.y_axis)
        {% for axis in axes | slice(start=2) %}
            && self.{{ axis }}.eq(&other.{{ axis }})
        {% endfor %}
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[{{ scalar_t }}; {{ size }}]> for {{ self_t }} {
    #[inline(always)]
    fn as_ref(&self) -> &[{{ scalar_t }}; {{ size }}] {
        unsafe { &*(self as *const Self as *const [{{ scalar_t }}; {{ size }}]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[{{ scalar_t }}; {{ size }}]> for {{ self_t }} {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [{{ scalar_t }}; {{ size }}] {
        unsafe { &mut *(self as *mut Self as *mut [{{ scalar_t }}; {{ size }}]) }
    }
}

impl Deref for {{ self_t }} {
    type Target = {{ deref_t }};
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for {{ self_t }} {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for {{ self_t }} {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(stringify!({{ self_t }}))
            {% for axis in axes %}
                .field("{{ axis }}", &self.{{ axis }})
            {% endfor %}
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        {% if dim == 2 %}
            write!(f, "[{}, {}]", self.x_axis, self.y_axis)
        {% elif dim == 3 %}
            write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
        {% elif dim == 4 %}
            write!(f, "[{}, {}, {}, {}]", self.x_axis, self.y_axis, self.z_axis, self.w_axis)
        {% endif %}
    }
}

