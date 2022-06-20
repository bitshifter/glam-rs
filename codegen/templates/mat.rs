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

{% if self_t == "Mat2" %}
    {% if not is_scalar %}
        {% set is_simd = true %}
        {% if is_sse2 %}
            {% set simd_t = "__m128" %}
        {% elif is_wasm32 %}
            {% set simd_t = "v128" %}
        {% endif %}
    {% endif %}
{% endif %}

{% set size = dim * dim %}
{% set nxn = dim ~ "x" ~ dim %}

{% set components = ["x", "y", "z", "w"] | slice(end = dim) %}
{% set axes = ["x_axis", "y_axis", "z_axis", "w_axis"] | slice(end = dim) %}

use crate::{
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
    swizzles::*,
};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

{% if is_sse2 %}
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
{% elif is_wasm32 %}
use core::arch::wasm32::*;
{% endif %}

#[cfg(not(feature = "std"))]
use num_traits::Float;


{% if self_t == "Mat2" and is_sse2 %}
union UnionCast {
    a: [f32; 4],
    v: {{ self_t }}
}
{% endif %}

/// Creates a {{ nxn }} matrix from column vectors.
#[inline(always)]
pub const fn {{ self_t | lower }}(
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
{%- elif self_t == "Mat2" and is_scalar %}
#[cfg_attr(not(any(feature = "scalar-math", target_arch = "spirv")), repr(C, align(16)))]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
{%- elif self_t == "DMat2" or self_t == "DMat4" %}
#[cfg_attr(feature = "cuda", repr(align(16)))]
{%- endif %}
{%- if self_t == "Mat2" and not is_scalar %}
#[repr(transparent)]
pub struct {{ self_t }}(pub(crate) {{ simd_t }});
{% else %}
pub struct {{ self_t }}
{
    {% for axis in axes %}
        pub {{ axis }}: {{ col_t }},
    {%- endfor %}
}
{% endif %}

impl {{ self_t }} {
    /// A {{ nxn }} matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self::from_cols(
        {% for axis in axes %}
            {{ col_t }}::ZERO,
        {%- endfor %}
    );

    /// A {{ nxn }} identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(
        {% for i in range(end = dim) %}
            {{ col_t }}::{{ components[i] | upper }},
        {%- endfor %}
    );

    /// All NAN:s.
    pub const NAN: Self = Self::from_cols(
        {% for axis in axes %}
            {{ col_t }}::NAN,
        {%- endfor %}
    );

    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    const fn new(
        {% for i in range(end = dim) %}
            {%- for j in range(end = dim) %}
                m{{ i }}{{ j }}: {{ scalar_t }},
            {%- endfor %}
        {%- endfor %}
    ) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            unsafe { UnionCast { a: [m00, m01, m10, m11] }.v }
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(f32x4(m00, m01, m10, m11))
        {% else %}
        Self {
            {% for i in range(end = dim) %}
                {{ axes[i] }}: {{ col_t}}::new(
                    {% for j in range(end = dim) %}
                        m{{ i }}{{ j }},
                    {% endfor %}
                ),
            {%- endfor %}
        }
        {% endif %}
    }

    /// Creates a {{ nxn }} matrix from two column vectors.
    #[inline(always)]
    pub const fn from_cols(
        {% for axis in axes %}
            {{ axis }}: {{ col_t }},
        {% endfor %}
    ) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            unsafe { UnionCast { a: [x_axis.x, x_axis.y, y_axis.x, y_axis.y] }.v }
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(f32x4(x_axis.x, x_axis.y, y_axis.x, y_axis.y))
        {% else %}
        Self {
            {% for axis in axes %}
                {{ axis }},
            {%- endfor %}
        }
        {% endif %}
    }

    /// Creates a {{ nxn }} matrix from a `[{{ scalar_t }}; {{ size }}]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline]
    pub const fn from_cols_array(m: &[{{ scalar_t }}; {{ size }}]) -> Self {
        Self::new(
            {% for i in range(end = size) %}
                m[{{ i }}],
            {%- endfor %}
        )
    }

    /// Creates a `[{{ scalar_t }}; {{ size }}]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array(&self) -> [{{ scalar_t }}; {{ size }}] {
        [
            {% for axis in axes %}
                {% for c in components %}
                    self.{{ axis }}.{{ c }},
                {%- endfor %}
            {%- endfor %}
        ]
    }

    /// Creates a {{ nxn }} matrix from a `[[{{ scalar_t }}; {{ dim }}]; {{ dim }}]` {{ dim }}D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    pub const fn from_cols_array_2d(m: &[[{{ scalar_t }}; {{ dim }}]; {{ dim }}]) -> Self {
        Self::from_cols(
            {% for i in range(end = dim) %}
                {{ col_t }}::from_array(m[{{ i }}]),
            {%- endfor %}
        )
    }

    /// Creates a `[[{{ scalar_t }}; {{ dim }}]; {{ dim }}]` {{ dim }}D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array_2d(&self) -> [[{{ scalar_t }}; {{ dim }}]; {{ dim }}] {
        [
            {% for axis in axes %}
                self.{{ axis }}.to_array(),
            {%- endfor %}
        ]
    }

    /// Creates a {{ nxn }} matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline]
    pub fn from_diagonal(diagonal: {{ vecn_t }}) -> Self {
        Self::new(
            {% for i in range(end = dim) %}
                {% for j in range(end = dim) %}
                    {% if i == j %}
                        diagonal.{{ components[i] }},
                    {% else %}
                        0.0,
                    {% endif %}
                {%- endfor %}
            {%- endfor %}
        )
    }

{% if dim == 2 %}
    /// Creates a {{ nxn }} matrix containing the combining non-uniform `scale` and rotation of
    /// `angle` (in radians).
    #[inline]
    pub fn from_scale_angle(scale: {{ col_t }}, angle: {{ scalar_t }}) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(cos * scale.x, sin * scale.x, -sin * scale.y, cos * scale.y)
    }

    /// Creates a {{ nxn }} matrix containing a rotation of `angle` (in radians).
    #[inline]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(cos, sin, -sin, cos)
    }

    /// Creates a {{ nxn }} matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline]
    pub fn from_mat3(m: {{ mat3_t }}) -> Self {
        Self::from_cols(m.x_axis.xy(), m.y_axis.xy())
    }
{% elif dim == 3 %}
    /// Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
    pub fn from_mat4(m: {{ mat4_t }}) -> Self {
        {% if self_t == "Mat3A" %}
            Self::from_cols(
                m.x_axis.into(),
                m.y_axis.into(),
                m.z_axis.into(),
            )
        {% else %}
            Self::from_cols(
                m.x_axis.xyz(),
                m.y_axis.xyz(),
                m.z_axis.xyz(),
            )
        {% endif %}
    }

    /// Creates a 3D rotation matrix from the given quaternion.
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_quat(rotation: {{ quat_t }}) -> Self {
        glam_assert!(rotation.is_normalized());

        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self::from_cols(
            {{ col_t }}::new(1.0 - (yy + zz), xy + wz, xz - wy),
            {{ col_t }}::new(xy - wz, 1.0 - (xx + zz), yz + wx),
            {{ col_t }}::new(xz + wy, yz - wx, 1.0 - (xx + yy)),
        )
    }

    /// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    /// radians).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_axis_angle(axis: {{ vec3_t }}, angle: {{ scalar_t }}) -> Self {
        {# TODO: make common with dim == 4 #}
        glam_assert!(axis.is_normalized());

        let (sin, cos) = angle.sin_cos();
        let (xsin, ysin, zsin) = axis.mul(sin).into();
        let (x, y, z) = axis.into();
        let (x2, y2, z2) = axis.mul(axis).into();
        let omc = 1.0 - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        Self::from_cols(
            {{ col_t }}::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            {{ col_t }}::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            {{ col_t }}::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        )
    }

    #[inline]
    /// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    /// radians).
    pub fn from_euler(order: EulerRot, a: {{ scalar_t }}, b: {{ scalar_t }}, c: {{ scalar_t }}) -> Self {
        let quat = {{ quat_t }}::from_euler(order, a, b, c);
        Self::from_quat(quat)
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: {{ scalar_t }}) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::X,
            {{ col_t }}::new(0.0, cosa, sina),
            {{ col_t }}::new(0.0, -sina, cosa),
        )
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: {{ scalar_t }}) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::new(cosa, 0.0, -sina),
            {{ col_t }}::Y,
            {{ col_t }}::new(sina, 0.0, cosa),
        )
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: {{ scalar_t }}) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::new(cosa, sina, 0.0),
            {{ col_t }}::new(-sina, cosa, 0.0),
            {{ col_t }}::Z,
        )
    }

    /// Creates an affine transformation matrix from the given 2D `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    pub fn from_translation(translation: {{ vec2_t }}) -> Self {
        Self::from_cols(
            {{ col_t }}::X,
            {{ col_t }}::Y,
            {{ col_t }}::new(translation.x, translation.y, 1.0))
    }

    /// Creates an affine transformation matrix from the given 2D rotation `angle` (in
    /// radians).
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::new(cos, sin, 0.0),
            {{ col_t }}::new(-sin, cos, 0.0),
            {{ col_t }}::Z,
        )
    }

    /// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    /// radians) and `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    pub fn from_scale_angle_translation(scale: {{ vec2_t }}, angle: {{ scalar_t }}, translation: {{ vec2_t }}) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::new(cos * scale.x, sin * scale.x, 0.0),
            {{ col_t }}::new(-sin * scale.y, cos * scale.y, 0.0),
            {{ col_t }}::new(translation.x, translation.y, 1.0),
        )
    }

    /// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    /// # Panics
    ///
    /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[inline]
    pub fn from_scale(scale: {{ vec2_t }}) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne({{ vec2_t }}::ZERO).any());

        Self::from_cols(
            {{ col_t }}::new(scale.x, 0.0, 0.0),
            {{ col_t }}::new(0.0, scale.y, 0.0),
            {{ col_t }}::Z,
        )
    }

    /// Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    pub fn from_mat2(m: {{ mat2_t }}) -> Self {
        Self::from_cols((m.x_axis, 0.0).into(), (m.y_axis, 0.0).into(), {{ col_t }}::Z)
    }

{% elif dim == 4 %}
    fn quat_to_axes(rotation: {{ quat_t }}) -> ({{ col_t }}, {{ col_t }}, {{ col_t }}) {
        glam_assert!(rotation.is_normalized());

        let (x, y, z, w) = rotation.into();
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let x_axis = {{ col_t }}::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0);
        let y_axis = {{ col_t }}::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0);
        let z_axis = {{ col_t }}::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0);
        (x_axis, y_axis, z_axis)
    }

    /// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_scale_rotation_translation(
        scale: {{ vec3_t }},
        rotation: {{ quat_t }},
        translation: {{ vec3_t }},
    ) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);
        Self::from_cols(
            x_axis.mul(scale.x),
            y_axis.mul(scale.y),
            z_axis.mul(scale.z),
            {{ col_t }}::from((translation, 1.0)),
        )
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_rotation_translation(rotation: {{ quat_t }}, translation: {{ vec3_t }}) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, {{ col_t }}::from((translation, 1.0)))
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
    /// expected to be a 3D affine transformation matrix otherwise the output will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero or if the resulting scale vector
    /// contains any zero elements when `glam_assert` is enabled.
    #[inline]
    pub fn to_scale_rotation_translation(&self) -> ({{ vec3_t }}, {{ quat_t }}, {{ vec3_t }}) {
        let det = self.determinant();
        glam_assert!(det != 0.0);

        let scale = {{ vec3_t }}::new(
            self.x_axis.length() * det.signum(),
            self.y_axis.length(),
            self.z_axis.length(),
        );

        glam_assert!(scale.cmpne({{ vec3_t }}::ZERO).all());

        let inv_scale = scale.recip();

        let rotation = {{ quat_t }}::from_rotation_axes(
            self.x_axis.mul(inv_scale.x).xyz(),
            self.y_axis.mul(inv_scale.y).xyz(),
            self.z_axis.mul(inv_scale.z).xyz(),
        );

        let translation = self.w_axis.xyz();

        (scale, rotation, translation)
    }

    /// Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_quat(rotation: {{ quat_t }}) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, {{ col_t }}::W)
    }

    /// Creates an affine transformation matrix from the given 3x3 linear transformation
    /// matrix.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_mat3(m: {{ mat3_t }}) -> Self {
        Self::from_cols(
            {{ col_t }}::from((m.x_axis, 0.0)),
            {{ col_t }}::from((m.y_axis, 0.0)),
            {{ col_t }}::from((m.z_axis, 0.0)),
            {{ col_t }}::W,
        )
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_translation(translation: {{ vec3_t }}) -> Self {
        Self::from_cols(
            {{ col_t }}::X,
            {{ col_t }}::Y,
            {{ col_t }}::Z,
            {{ col_t }}::new(translation.x, translation.y, translation.z, 1.0),
        )
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
    #[inline]
    pub fn from_axis_angle(axis: {{ vec3_t }}, angle: {{ scalar_t }}) -> Self {
        {# TODO: make common with dim == 3 #}
        glam_assert!(axis.is_normalized());

        let (sin, cos) = angle.sin_cos();
        let axis_sin = axis.mul(sin);
        let axis_sq = axis.mul(axis);
        let omc = 1.0 - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;
        Self::from_cols(
            {{ col_t }}::new(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
                0.0,
            ),
            {{ col_t }}::new(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                0.0,
            ),
            {{ col_t }}::new(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                0.0,
            ),
            {{ col_t }}::W,
        )
    }

    #[inline]
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
    #[inline]
    pub fn from_rotation_x(angle: {{ scalar_t }}) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::X,
            {{ col_t }}::new(0.0, cosa, sina, 0.0),
            {{ col_t }}::new(0.0, -sina, cosa, 0.0),
            {{ col_t }}::W,
        )
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_y(angle: {{ scalar_t }}) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::new(cosa, 0.0, -sina, 0.0),
            {{ col_t }}::Y,
            {{ col_t }}::new(sina, 0.0, cosa, 0.0),
            {{ col_t }}::W,
        )
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_z(angle: {{ scalar_t }}) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            {{ col_t }}::new(cosa, sina, 0.0, 0.0),
            {{ col_t }}::new(-sina, cosa, 0.0, 0.0),
            {{ col_t }}::Z,
            {{ col_t }}::W,
        )
    }

    /// Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    ///
    /// # Panics
    ///
    /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[inline]
    pub fn from_scale(scale: {{ vec3_t }}) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne({{ vec3_t }}::ZERO).any());

        Self::from_cols(
            {{ col_t }}::new(scale.x, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, scale.y, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, scale.z, 0.0),
            {{ col_t }}::W,
        )
    }
{% endif %}

    /// Creates a {{ nxn }} matrix from the first {{ size }} values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than {{ size }} elements long.
    #[inline]
    pub const fn from_cols_slice(slice: &[{{ scalar_t }}]) -> Self {
        Self::new(
            {% for i in range(end = size) %}
                slice[{{ i }}],
            {%- endfor %}
        )
    }

    /// Writes the columns of `self` to the first {{ size }} elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than {{ size }} elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [{{ scalar_t }}]) {
        {% for i in range(end = dim) %}
            {%- for j in range(end = dim) %}
                slice[{{ i * dim + j }}] = self.{{ axes[i] }}.{{ components[j] }};
            {%- endfor %}
        {%- endfor %}
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
        {% for axis in axes %}
            self.{{ axis }}.is_finite() {% if not loop.last %} && {% endif %}
        {% endfor %}
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        {% for axis in axes %}
            self.{{ axis }}.is_nan() {% if not loop.last %} || {% endif %}
        {% endfor %}
    }

    /// Returns the transpose of `self`.
    #[must_use]
    #[inline]
    pub fn transpose(&self) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_00) })
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(i32x4_shuffle::<0, 2, 5, 7>(self.0, self.0))
        {% elif self_t == "Mat3A" and is_sse2 %}
            unsafe {
                let tmp0 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b01_00_01_00);
                let tmp1 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b11_10_11_10);

                Self {
                    x_axis: Vec3A(_mm_shuffle_ps(tmp0, self.z_axis.0, 0b00_00_10_00)),
                    y_axis: Vec3A(_mm_shuffle_ps(tmp0, self.z_axis.0, 0b01_01_11_01)),
                    z_axis: Vec3A(_mm_shuffle_ps(tmp1, self.z_axis.0, 0b10_10_10_00)),
                }
            }
        {% elif self_t == "Mat3A" and is_wasm32 %}
            let tmp0 = i32x4_shuffle::<0, 1, 4, 5>(self.x_axis.0, self.y_axis.0);
            let tmp1 = i32x4_shuffle::<2, 3, 6, 7>(self.x_axis.0, self.y_axis.0);

            Self {
                x_axis: Vec3A(i32x4_shuffle::<0, 2, 4, 4>(tmp0, self.z_axis.0)),
                y_axis: Vec3A(i32x4_shuffle::<1, 3, 5, 5>(tmp0, self.z_axis.0)),
                z_axis: Vec3A(i32x4_shuffle::<0, 2, 6, 6>(tmp1, self.z_axis.0)),
            }
        {% elif self_t == "Mat4" and is_sse2 %}
            unsafe {
                // Based on https://github.com/microsoft/DirectXMath `XMMatrixTranspose`
                let tmp0 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b01_00_01_00);
                let tmp1 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b11_10_11_10);
                let tmp2 = _mm_shuffle_ps(self.z_axis.0, self.w_axis.0, 0b01_00_01_00);
                let tmp3 = _mm_shuffle_ps(self.z_axis.0, self.w_axis.0, 0b11_10_11_10);

                Self {
                    x_axis: Vec4(_mm_shuffle_ps(tmp0, tmp2, 0b10_00_10_00)),
                    y_axis: Vec4(_mm_shuffle_ps(tmp0, tmp2, 0b11_01_11_01)),
                    z_axis: Vec4(_mm_shuffle_ps(tmp1, tmp3, 0b10_00_10_00)),
                    w_axis: Vec4(_mm_shuffle_ps(tmp1, tmp3, 0b11_01_11_01)),
                }
            }
        {% elif self_t == "Mat4" and is_wasm32 %}
            // Based on https://github.com/microsoft/DirectXMath `XMMatrixTranspose`
            let tmp0 = i32x4_shuffle::<0, 1, 4, 5>(self.x_axis.0, self.y_axis.0);
            let tmp1 = i32x4_shuffle::<2, 3, 6, 7>(self.x_axis.0, self.y_axis.0);
            let tmp2 = i32x4_shuffle::<0, 1, 4, 5>(self.z_axis.0, self.w_axis.0);
            let tmp3 = i32x4_shuffle::<2, 3, 6, 7>(self.z_axis.0, self.w_axis.0);

            Self {
                x_axis: Vec4(i32x4_shuffle::<0, 2, 4, 6>(tmp0, tmp2)),
                y_axis: Vec4(i32x4_shuffle::<1, 3, 5, 7>(tmp0, tmp2)),
                z_axis: Vec4(i32x4_shuffle::<0, 2, 4, 6>(tmp1, tmp3)),
                w_axis: Vec4(i32x4_shuffle::<1, 3, 5, 7>(tmp1, tmp3)),
            }
        {% else %}
            Self {
                {% for i in range(end = dim) %}
                    {{ axes[i] }}: {{ col_t }}::new(
                        {% for j in range(end = dim) %}
                            self.{{ axes[j] }}.{{ components[i] }},
                        {% endfor %}
                    ),
                {%- endfor %}
            }
        {% endif %}
    }

    /// Returns the determinant of `self`.
    {%- if dim < 3 %}
    #[inline]
    {%- endif %}
    pub fn determinant(&self) -> {{ scalar_t }} {
        {% if self_t == "Mat2" and is_sse2 %}
            unsafe {
                let abcd = self.0;
                let dcba = _mm_shuffle_ps(abcd, abcd, 0b00_01_10_11);
                let prod = _mm_mul_ps(abcd, dcba);
                let det = _mm_sub_ps(prod, _mm_shuffle_ps(prod, prod, 0b01_01_01_01));
                _mm_cvtss_f32(det)
            }
        {% elif self_t == "Mat2" and is_wasm32 %}
            let abcd = self.0;
            let dcba = i32x4_shuffle::<3, 2, 5, 4>(abcd, abcd);
            let prod = f32x4_mul(abcd, dcba);
            let det = f32x4_sub(prod, i32x4_shuffle::<1, 1, 5, 5>(prod, prod));
            f32x4_extract_lane::<0>(det)
        {% elif self_t == "Mat4" and is_sse2 %}
            unsafe {
                // Based on https://github.com/g-truc/glm `glm_mat4_determinant_lowp`
                let swp2a = _mm_shuffle_ps(self.z_axis.0, self.z_axis.0, 0b00_01_01_10);
                let swp3a = _mm_shuffle_ps(self.w_axis.0, self.w_axis.0, 0b11_10_11_11);
                let swp2b = _mm_shuffle_ps(self.z_axis.0, self.z_axis.0, 0b11_10_11_11);
                let swp3b = _mm_shuffle_ps(self.w_axis.0, self.w_axis.0, 0b00_01_01_10);
                let swp2c = _mm_shuffle_ps(self.z_axis.0, self.z_axis.0, 0b00_00_01_10);
                let swp3c = _mm_shuffle_ps(self.w_axis.0, self.w_axis.0, 0b01_10_00_00);

                let mula = _mm_mul_ps(swp2a, swp3a);
                let mulb = _mm_mul_ps(swp2b, swp3b);
                let mulc = _mm_mul_ps(swp2c, swp3c);
                let sube = _mm_sub_ps(mula, mulb);
                let subf = _mm_sub_ps(_mm_movehl_ps(mulc, mulc), mulc);

                let subfaca = _mm_shuffle_ps(sube, sube, 0b10_01_00_00);
                let swpfaca = _mm_shuffle_ps(self.y_axis.0, self.y_axis.0, 0b00_00_00_01);
                let mulfaca = _mm_mul_ps(swpfaca, subfaca);

                let subtmpb = _mm_shuffle_ps(sube, subf, 0b00_00_11_01);
                let subfacb = _mm_shuffle_ps(subtmpb, subtmpb, 0b11_01_01_00);
                let swpfacb = _mm_shuffle_ps(self.y_axis.0, self.y_axis.0, 0b01_01_10_10);
                let mulfacb = _mm_mul_ps(swpfacb, subfacb);

                let subres = _mm_sub_ps(mulfaca, mulfacb);
                let subtmpc = _mm_shuffle_ps(sube, subf, 0b01_00_10_10);
                let subfacc = _mm_shuffle_ps(subtmpc, subtmpc, 0b11_11_10_00);
                let swpfacc = _mm_shuffle_ps(self.y_axis.0, self.y_axis.0, 0b10_11_11_11);
                let mulfacc = _mm_mul_ps(swpfacc, subfacc);

                let addres = _mm_add_ps(subres, mulfacc);
                let detcof = _mm_mul_ps(addres, _mm_setr_ps(1.0, -1.0, 1.0, -1.0));

                crate::sse2::dot4(self.x_axis.0, detcof)
            }
        {% elif self_t == "Mat4" and is_wasm32 %}
            // Based on https://github.com/g-truc/glm `glm_mat4_determinant`
            let swp2a = i32x4_shuffle::<2, 1, 1, 0>(self.z_axis.0, self.z_axis.0);
            let swp3a = i32x4_shuffle::<3, 3, 2, 3>(self.w_axis.0, self.w_axis.0);
            let swp2b = i32x4_shuffle::<3, 3, 2, 3>(self.z_axis.0, self.z_axis.0);
            let swp3b = i32x4_shuffle::<2, 1, 1, 0>(self.w_axis.0, self.w_axis.0);
            let swp2c = i32x4_shuffle::<2, 1, 0, 0>(self.z_axis.0, self.z_axis.0);
            let swp3c = i32x4_shuffle::<0, 0, 2, 1>(self.w_axis.0, self.w_axis.0);

            let mula = f32x4_mul(swp2a, swp3a);
            let mulb = f32x4_mul(swp2b, swp3b);
            let mulc = f32x4_mul(swp2c, swp3c);
            let sube = f32x4_sub(mula, mulb);
            let subf = f32x4_sub(i32x4_shuffle::<6, 7, 2, 3>(mulc, mulc), mulc);

            let subfaca = i32x4_shuffle::<0, 0, 1, 2>(sube, sube);
            let swpfaca = i32x4_shuffle::<1, 0, 0, 0>(self.y_axis.0, self.y_axis.0);
            let mulfaca = f32x4_mul(swpfaca, subfaca);

            let subtmpb = i32x4_shuffle::<1, 3, 4, 4>(sube, subf);
            let subfacb = i32x4_shuffle::<0, 1, 1, 3>(subtmpb, subtmpb);
            let swpfacb = i32x4_shuffle::<2, 2, 1, 1>(self.y_axis.0, self.y_axis.0);
            let mulfacb = f32x4_mul(swpfacb, subfacb);

            let subres = f32x4_sub(mulfaca, mulfacb);
            let subtmpc = i32x4_shuffle::<2, 2, 4, 5>(sube, subf);
            let subfacc = i32x4_shuffle::<0, 2, 3, 3>(subtmpc, subtmpc);
            let swpfacc = i32x4_shuffle::<3, 3, 3, 2>(self.y_axis.0, self.y_axis.0);
            let mulfacc = f32x4_mul(swpfacc, subfacc);

            let addres = f32x4_add(subres, mulfacc);
            let detcof = f32x4_mul(addres, f32x4(1.0, -1.0, 1.0, -1.0));

            crate::wasm32::dot4(self.x_axis.0, detcof)
        {% elif dim == 2 %}
            self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
        {% elif dim == 3 %}
            self.z_axis.dot(self.x_axis.cross(self.y_axis))
        {% elif dim == 4 %}
            let (m00, m01, m02, m03) = self.x_axis.into();
            let (m10, m11, m12, m13) = self.y_axis.into();
            let (m20, m21, m22, m23) = self.z_axis.into();
            let (m30, m31, m32, m33) = self.w_axis.into();

            let a2323 = m22 * m33 - m23 * m32;
            let a1323 = m21 * m33 - m23 * m31;
            let a1223 = m21 * m32 - m22 * m31;
            let a0323 = m20 * m33 - m23 * m30;
            let a0223 = m20 * m32 - m22 * m30;
            let a0123 = m20 * m31 - m21 * m30;

            m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
                - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
                + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
                - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
        {% endif %}
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[must_use]
    pub fn inverse(&self) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            unsafe {
                const SIGN: __m128 = const_f32x4!([1.0, -1.0, -1.0, 1.0]);
                let abcd = self.0;
                let dcba = _mm_shuffle_ps(abcd, abcd, 0b00_01_10_11);
                let prod = _mm_mul_ps(abcd, dcba);
                let sub = _mm_sub_ps(prod, _mm_shuffle_ps(prod, prod, 0b01_01_01_01));
                let det = _mm_shuffle_ps(sub, sub, 0b00_00_00_00);
                let tmp = _mm_div_ps(SIGN, det);
                glam_assert!(Mat2(tmp).is_finite());
                let dbca = _mm_shuffle_ps(abcd, abcd, 0b00_10_01_11);
                Self(_mm_mul_ps(dbca, tmp))
            }
        {% elif self_t == "Mat2" and is_wasm32 %}
            const SIGN: v128 = const_f32x4!([1.0, -1.0, -1.0, 1.0]);
            let abcd = self.0;
            let dcba = i32x4_shuffle::<3, 2, 5, 4>(abcd, abcd);
            let prod = f32x4_mul(abcd, dcba);
            let sub = f32x4_sub(prod, i32x4_shuffle::<1, 1, 5, 5>(prod, prod));
            let det = i32x4_shuffle::<0, 0, 4, 4>(sub, sub);
            let tmp = f32x4_div(SIGN, det);
            glam_assert!(Mat2(tmp).is_finite());
            let dbca = i32x4_shuffle::<3, 1, 6, 4>(abcd, abcd);
            Self(f32x4_mul(dbca, tmp))
        {% elif self_t == "Mat4" and is_sse2 %}
            unsafe {
                // Based on https://github.com/g-truc/glm `glm_mat4_inverse`
                let fac0 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b11_11_11_11);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b10_10_10_10);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b10_10_10_10);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b11_11_11_11);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac1 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b11_11_11_11);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b01_01_01_01);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b01_01_01_01);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b11_11_11_11);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac2 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b10_10_10_10);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b01_01_01_01);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b01_01_01_01);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b10_10_10_10);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac3 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b11_11_11_11);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b00_00_00_00);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b00_00_00_00);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b11_11_11_11);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac4 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b10_10_10_10);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b00_00_00_00);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b00_00_00_00);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b10_10_10_10);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac5 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b01_01_01_01);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b00_00_00_00);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b00_00_00_00);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b01_01_01_01);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let sign_a = _mm_set_ps(1.0, -1.0, 1.0, -1.0);
                let sign_b = _mm_set_ps(-1.0, 1.0, -1.0, 1.0);

                let temp0 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b00_00_00_00);
                let vec0 = _mm_shuffle_ps(temp0, temp0, 0b10_10_10_00);

                let temp1 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b01_01_01_01);
                let vec1 = _mm_shuffle_ps(temp1, temp1, 0b10_10_10_00);

                let temp2 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b10_10_10_10);
                let vec2 = _mm_shuffle_ps(temp2, temp2, 0b10_10_10_00);

                let temp3 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b11_11_11_11);
                let vec3 = _mm_shuffle_ps(temp3, temp3, 0b10_10_10_00);

                let mul00 = _mm_mul_ps(vec1, fac0);
                let mul01 = _mm_mul_ps(vec2, fac1);
                let mul02 = _mm_mul_ps(vec3, fac2);
                let sub00 = _mm_sub_ps(mul00, mul01);
                let add00 = _mm_add_ps(sub00, mul02);
                let inv0 = _mm_mul_ps(sign_b, add00);

                let mul03 = _mm_mul_ps(vec0, fac0);
                let mul04 = _mm_mul_ps(vec2, fac3);
                let mul05 = _mm_mul_ps(vec3, fac4);
                let sub01 = _mm_sub_ps(mul03, mul04);
                let add01 = _mm_add_ps(sub01, mul05);
                let inv1 = _mm_mul_ps(sign_a, add01);

                let mul06 = _mm_mul_ps(vec0, fac1);
                let mul07 = _mm_mul_ps(vec1, fac3);
                let mul08 = _mm_mul_ps(vec3, fac5);
                let sub02 = _mm_sub_ps(mul06, mul07);
                let add02 = _mm_add_ps(sub02, mul08);
                let inv2 = _mm_mul_ps(sign_b, add02);

                let mul09 = _mm_mul_ps(vec0, fac2);
                let mul10 = _mm_mul_ps(vec1, fac4);
                let mul11 = _mm_mul_ps(vec2, fac5);
                let sub03 = _mm_sub_ps(mul09, mul10);
                let add03 = _mm_add_ps(sub03, mul11);
                let inv3 = _mm_mul_ps(sign_a, add03);

                let row0 = _mm_shuffle_ps(inv0, inv1, 0b00_00_00_00);
                let row1 = _mm_shuffle_ps(inv2, inv3, 0b00_00_00_00);
                let row2 = _mm_shuffle_ps(row0, row1, 0b10_00_10_00);

                let dot0 = crate::sse2::dot4(self.x_axis.0, row2);
                glam_assert!(dot0 != 0.0);

                let rcp0 = _mm_set1_ps(dot0.recip());

                Self {
                    x_axis: Vec4(_mm_mul_ps(inv0, rcp0)),
                    y_axis: Vec4(_mm_mul_ps(inv1, rcp0)),
                    z_axis: Vec4(_mm_mul_ps(inv2, rcp0)),
                    w_axis: Vec4(_mm_mul_ps(inv3, rcp0)),
                }
            }
        {% elif self_t == "Mat4" and is_wasm32 %}
            // Based on https://github.com/g-truc/glm `glm_mat4_inverse`
            let fac0 = {
                let swp0a = i32x4_shuffle::<3, 3, 7, 7>(self.w_axis.0, self.z_axis.0);
                let swp0b = i32x4_shuffle::<2, 2, 6, 6>(self.w_axis.0, self.z_axis.0);

                let swp00 = i32x4_shuffle::<2, 2, 6, 6>(self.z_axis.0, self.y_axis.0);
                let swp01 = i32x4_shuffle::<0, 0, 4, 6>(swp0a, swp0a);
                let swp02 = i32x4_shuffle::<0, 0, 4, 6>(swp0b, swp0b);
                let swp03 = i32x4_shuffle::<3, 3, 7, 7>(self.z_axis.0, self.y_axis.0);

                let mul00 = f32x4_mul(swp00, swp01);
                let mul01 = f32x4_mul(swp02, swp03);
                f32x4_sub(mul00, mul01)
            };
            let fac1 = {
                let swp0a = i32x4_shuffle::<3, 3, 7, 7>(self.w_axis.0, self.z_axis.0);
                let swp0b = i32x4_shuffle::<1, 1, 5, 5>(self.w_axis.0, self.z_axis.0);

                let swp00 = i32x4_shuffle::<1, 1, 5, 5>(self.z_axis.0, self.y_axis.0);
                let swp01 = i32x4_shuffle::<0, 0, 4, 6>(swp0a, swp0a);
                let swp02 = i32x4_shuffle::<0, 0, 4, 6>(swp0b, swp0b);
                let swp03 = i32x4_shuffle::<3, 3, 7, 7>(self.z_axis.0, self.y_axis.0);

                let mul00 = f32x4_mul(swp00, swp01);
                let mul01 = f32x4_mul(swp02, swp03);
                f32x4_sub(mul00, mul01)
            };
            let fac2 = {
                let swp0a = i32x4_shuffle::<2, 2, 6, 6>(self.w_axis.0, self.z_axis.0);
                let swp0b = i32x4_shuffle::<1, 1, 5, 5>(self.w_axis.0, self.z_axis.0);

                let swp00 = i32x4_shuffle::<1, 1, 5, 5>(self.z_axis.0, self.y_axis.0);
                let swp01 = i32x4_shuffle::<0, 0, 4, 6>(swp0a, swp0a);
                let swp02 = i32x4_shuffle::<0, 0, 4, 6>(swp0b, swp0b);
                let swp03 = i32x4_shuffle::<2, 2, 6, 6>(self.z_axis.0, self.y_axis.0);

                let mul00 = f32x4_mul(swp00, swp01);
                let mul01 = f32x4_mul(swp02, swp03);
                f32x4_sub(mul00, mul01)
            };
            let fac3 = {
                let swp0a = i32x4_shuffle::<3, 3, 7, 7>(self.w_axis.0, self.z_axis.0);
                let swp0b = i32x4_shuffle::<0, 0, 4, 4>(self.w_axis.0, self.z_axis.0);

                let swp00 = i32x4_shuffle::<0, 0, 4, 4>(self.z_axis.0, self.y_axis.0);
                let swp01 = i32x4_shuffle::<0, 0, 4, 6>(swp0a, swp0a);
                let swp02 = i32x4_shuffle::<0, 0, 4, 6>(swp0b, swp0b);
                let swp03 = i32x4_shuffle::<3, 3, 7, 7>(self.z_axis.0, self.y_axis.0);

                let mul00 = f32x4_mul(swp00, swp01);
                let mul01 = f32x4_mul(swp02, swp03);
                f32x4_sub(mul00, mul01)
            };
            let fac4 = {
                let swp0a = i32x4_shuffle::<2, 2, 6, 6>(self.w_axis.0, self.z_axis.0);
                let swp0b = i32x4_shuffle::<0, 0, 4, 4>(self.w_axis.0, self.z_axis.0);

                let swp00 = i32x4_shuffle::<0, 0, 4, 4>(self.z_axis.0, self.y_axis.0);
                let swp01 = i32x4_shuffle::<0, 0, 4, 6>(swp0a, swp0a);
                let swp02 = i32x4_shuffle::<0, 0, 4, 6>(swp0b, swp0b);
                let swp03 = i32x4_shuffle::<2, 2, 6, 6>(self.z_axis.0, self.y_axis.0);

                let mul00 = f32x4_mul(swp00, swp01);
                let mul01 = f32x4_mul(swp02, swp03);
                f32x4_sub(mul00, mul01)
            };
            let fac5 = {
                let swp0a = i32x4_shuffle::<1, 1, 5, 5>(self.w_axis.0, self.z_axis.0);
                let swp0b = i32x4_shuffle::<0, 0, 4, 4>(self.w_axis.0, self.z_axis.0);

                let swp00 = i32x4_shuffle::<0, 0, 4, 4>(self.z_axis.0, self.y_axis.0);
                let swp01 = i32x4_shuffle::<0, 0, 4, 6>(swp0a, swp0a);
                let swp02 = i32x4_shuffle::<0, 0, 4, 6>(swp0b, swp0b);
                let swp03 = i32x4_shuffle::<1, 1, 5, 5>(self.z_axis.0, self.y_axis.0);

                let mul00 = f32x4_mul(swp00, swp01);
                let mul01 = f32x4_mul(swp02, swp03);
                f32x4_sub(mul00, mul01)
            };
            let sign_a = f32x4(-1.0, 1.0, -1.0, 1.0);
            let sign_b = f32x4(1.0, -1.0, 1.0, -1.0);

            let temp0 = i32x4_shuffle::<0, 0, 4, 4>(self.y_axis.0, self.x_axis.0);
            let vec0 = i32x4_shuffle::<0, 2, 6, 6>(temp0, temp0);

            let temp1 = i32x4_shuffle::<1, 1, 5, 5>(self.y_axis.0, self.x_axis.0);
            let vec1 = i32x4_shuffle::<0, 2, 6, 6>(temp1, temp1);

            let temp2 = i32x4_shuffle::<2, 2, 6, 6>(self.y_axis.0, self.x_axis.0);
            let vec2 = i32x4_shuffle::<0, 2, 6, 6>(temp2, temp2);

            let temp3 = i32x4_shuffle::<3, 3, 7, 7>(self.y_axis.0, self.x_axis.0);
            let vec3 = i32x4_shuffle::<0, 2, 6, 6>(temp3, temp3);

            let mul00 = f32x4_mul(vec1, fac0);
            let mul01 = f32x4_mul(vec2, fac1);
            let mul02 = f32x4_mul(vec3, fac2);
            let sub00 = f32x4_sub(mul00, mul01);
            let add00 = f32x4_add(sub00, mul02);
            let inv0 = f32x4_mul(sign_b, add00);

            let mul03 = f32x4_mul(vec0, fac0);
            let mul04 = f32x4_mul(vec2, fac3);
            let mul05 = f32x4_mul(vec3, fac4);
            let sub01 = f32x4_sub(mul03, mul04);
            let add01 = f32x4_add(sub01, mul05);
            let inv1 = f32x4_mul(sign_a, add01);

            let mul06 = f32x4_mul(vec0, fac1);
            let mul07 = f32x4_mul(vec1, fac3);
            let mul08 = f32x4_mul(vec3, fac5);
            let sub02 = f32x4_sub(mul06, mul07);
            let add02 = f32x4_add(sub02, mul08);
            let inv2 = f32x4_mul(sign_b, add02);

            let mul09 = f32x4_mul(vec0, fac2);
            let mul10 = f32x4_mul(vec1, fac4);
            let mul11 = f32x4_mul(vec2, fac5);
            let sub03 = f32x4_sub(mul09, mul10);
            let add03 = f32x4_add(sub03, mul11);
            let inv3 = f32x4_mul(sign_a, add03);

            let row0 = i32x4_shuffle::<0, 0, 4, 4>(inv0, inv1);
            let row1 = i32x4_shuffle::<0, 0, 4, 4>(inv2, inv3);
            let row2 = i32x4_shuffle::<0, 2, 4, 6>(row0, row1);

            let dot0 = crate::wasm32::dot4(self.x_axis.0, row2);
            glam_assert!(dot0 != 0.0);

            let rcp0 = f32x4_splat(dot0.recip());

            Self {
                x_axis: Vec4(f32x4_mul(inv0, rcp0)),
                y_axis: Vec4(f32x4_mul(inv1, rcp0)),
                z_axis: Vec4(f32x4_mul(inv2, rcp0)),
                w_axis: Vec4(f32x4_mul(inv3, rcp0)),
            }
        {% elif dim == 2 %}
            let inv_det = {
                let det = self.determinant();
                glam_assert!(det != 0.0);
                det.recip()
            };
            Self::new(
                self.y_axis.y * inv_det,
                self.x_axis.y * -inv_det,
                self.y_axis.x * -inv_det,
                self.x_axis.x * inv_det,
            )
        {% elif dim == 3 %}
            let tmp0 = self.y_axis.cross(self.z_axis);
            let tmp1 = self.z_axis.cross(self.x_axis);
            let tmp2 = self.x_axis.cross(self.y_axis);
            let det = self.z_axis.dot(tmp2);
            glam_assert!(det != 0.0);
            let inv_det = {{ col_t }}::splat(det.recip());
            Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
        {% elif dim == 4 %}
            let (m00, m01, m02, m03) = self.x_axis.into();
            let (m10, m11, m12, m13) = self.y_axis.into();
            let (m20, m21, m22, m23) = self.z_axis.into();
            let (m30, m31, m32, m33) = self.w_axis.into();

            let coef00 = m22 * m33 - m32 * m23;
            let coef02 = m12 * m33 - m32 * m13;
            let coef03 = m12 * m23 - m22 * m13;

            let coef04 = m21 * m33 - m31 * m23;
            let coef06 = m11 * m33 - m31 * m13;
            let coef07 = m11 * m23 - m21 * m13;

            let coef08 = m21 * m32 - m31 * m22;
            let coef10 = m11 * m32 - m31 * m12;
            let coef11 = m11 * m22 - m21 * m12;

            let coef12 = m20 * m33 - m30 * m23;
            let coef14 = m10 * m33 - m30 * m13;
            let coef15 = m10 * m23 - m20 * m13;

            let coef16 = m20 * m32 - m30 * m22;
            let coef18 = m10 * m32 - m30 * m12;
            let coef19 = m10 * m22 - m20 * m12;

            let coef20 = m20 * m31 - m30 * m21;
            let coef22 = m10 * m31 - m30 * m11;
            let coef23 = m10 * m21 - m20 * m11;

            let fac0 = {{ col_t }}::new(coef00, coef00, coef02, coef03);
            let fac1 = {{ col_t }}::new(coef04, coef04, coef06, coef07);
            let fac2 = {{ col_t }}::new(coef08, coef08, coef10, coef11);
            let fac3 = {{ col_t }}::new(coef12, coef12, coef14, coef15);
            let fac4 = {{ col_t }}::new(coef16, coef16, coef18, coef19);
            let fac5 = {{ col_t }}::new(coef20, coef20, coef22, coef23);

            let vec0 = {{ col_t }}::new(m10, m00, m00, m00);
            let vec1 = {{ col_t }}::new(m11, m01, m01, m01);
            let vec2 = {{ col_t }}::new(m12, m02, m02, m02);
            let vec3 = {{ col_t }}::new(m13, m03, m03, m03);

            let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
            let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
            let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
            let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

            let sign_a = {{ col_t }}::new(1.0, -1.0, 1.0, -1.0);
            let sign_b = {{ col_t }}::new(-1.0, 1.0, -1.0, 1.0);

            let inverse = Self::from_cols(
                inv0.mul(sign_a),
                inv1.mul(sign_b),
                inv2.mul(sign_a),
                inv3.mul(sign_b),
            );

            let col0 = {{ col_t }}::new(
                inverse.x_axis.x,
                inverse.y_axis.x,
                inverse.z_axis.x,
                inverse.w_axis.x,
            );

            let dot0 = self.x_axis.mul(col0);
            let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

            glam_assert!(dot1 != 0.0);

            let rcp_det = dot1.recip();
            inverse.mul(rcp_det)
        {% endif %}
    }

{% if dim == 3 %}
    /// Transforms the given 2D vector as a point.
    ///
    /// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline]
    pub fn transform_point2(&self, rhs: {{ vec2_t }}) -> {{ vec2_t }} {
        {{ mat2_t }}::from_cols(self.x_axis.xy(), self.y_axis.xy()) * rhs + self.z_axis.xy()
    }

    /// Rotates the given 2D vector.
    ///
    /// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline]
    pub fn transform_vector2(&self, rhs: {{ vec2_t }}) -> {{ vec2_t }} {
        {{ mat2_t }}::from_cols(self.x_axis.xy(), self.y_axis.xy()) * rhs
    }

{% elif dim == 4 %}
    #[inline]
    fn look_to_lh(eye: {{ vec3_t }}, dir: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        Self::from_cols(
            {{ col_t }}::new(s.x, u.x, f.x, 0.0),
            {{ col_t }}::new(s.y, u.y, f.y, 0.0),
            {{ col_t }}::new(s.z, u.z, f.z, 0.0),
            {{ col_t }}::new(-s.dot(eye), -u.dot(eye), -f.dot(eye), 1.0),
        )
    }

    /// Creates a left-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn look_at_lh(eye: {{ vec3_t }}, center: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center.sub(eye), up)
    }

    /// Creates a right-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn look_at_rh(eye: {{ vec3_t }}, center: {{ vec3_t }}, up: {{ vec3_t }}) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, eye.sub(center), up)
    }

    /// Creates a right-handed perspective projection matrix with [-1,1] depth range.
    /// This is the same as the OpenGL `gluPerspective` function.
    /// See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
    #[inline]
    pub fn perspective_rh_gl(
        fov_y_radians: {{ scalar_t }},
        aspect_ratio: {{ scalar_t }},
        z_near: {{ scalar_t }},
        z_far: {{ scalar_t }},
    ) -> Self {
        let inv_length = 1.0 / (z_near - z_far);
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        let a = f / aspect_ratio;
        let b = (z_near + z_far) * inv_length;
        let c = (2.0 * z_near * z_far) * inv_length;
        Self::from_cols(
            {{ col_t }}::new(a, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, f, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, b, -1.0),
            {{ col_t }}::new(0.0, 0.0, c, 0.0),
        )
    }

    /// Creates a left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    /// enabled.
    #[inline]
    pub fn perspective_lh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}, z_far: {{ scalar_t }}) -> Self {
        glam_assert!(z_near > 0.0 && z_far > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        Self::from_cols(
            {{ col_t }}::new(w, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, h, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, r, 1.0),
            {{ col_t }}::new(0.0, 0.0, -r * z_near, 0.0),
        )
    }

    /// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
    /// enabled.
    #[inline]
    pub fn perspective_rh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}, z_far: {{ scalar_t }}) -> Self {
        glam_assert!(z_near > 0.0 && z_far > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);
        Self::from_cols(
            {{ col_t }}::new(w, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, h, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, r, -1.0),
            {{ col_t }}::new(0.0, 0.0, r * z_near, 0.0),
        )
    }

    /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    pub fn perspective_infinite_lh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}) -> Self {
        glam_assert!(z_near > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::from_cols(
            {{ col_t }}::new(w, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, h, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, 1.0, 1.0),
            {{ col_t }}::new(0.0, 0.0, -z_near, 0.0),
        )
    }

    /// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
    ///
    /// # Panics
    ///
    /// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
    #[inline]
    pub fn perspective_infinite_reverse_lh(
        fov_y_radians: {{ scalar_t }},
        aspect_ratio: {{ scalar_t }},
        z_near: {{ scalar_t }},
    ) -> Self {
        glam_assert!(z_near > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::from_cols(
            {{ col_t }}::new(w, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, h, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, 0.0, 1.0),
            {{ col_t }}::new(0.0, 0.0, z_near, 0.0),
        )
    }

    /// Creates an infinite right-handed perspective projection matrix with
    /// `[0,1]` depth range.
    #[inline]
    pub fn perspective_infinite_rh(fov_y_radians: {{ scalar_t }}, aspect_ratio: {{ scalar_t }}, z_near: {{ scalar_t }}) -> Self {
        glam_assert!(z_near > 0.0);
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        Self::from_cols(
            {{ col_t }}::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, f, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, -1.0, -1.0),
            {{ col_t }}::new(0.0, 0.0, -z_near, 0.0),
        )
    }

    /// Creates an infinite reverse right-handed perspective projection matrix
    /// with `[0,1]` depth range.
    #[inline]
    pub fn perspective_infinite_reverse_rh(
        fov_y_radians: {{ scalar_t }},
        aspect_ratio: {{ scalar_t }},
        z_near: {{ scalar_t }},
    ) -> Self {
        glam_assert!(z_near > 0.0);
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        Self::from_cols(
            {{ col_t }}::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, f, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, 0.0, -1.0),
            {{ col_t }}::new(0.0, 0.0, z_near, 0.0),
        )
    }

    /// Creates a right-handed orthographic projection matrix with `[-1,1]` depth
    /// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    /// See
    /// <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
    #[inline]
    pub fn orthographic_rh_gl(
        left: {{ scalar_t }},
        right: {{ scalar_t }},
        bottom: {{ scalar_t }},
        top: {{ scalar_t }},
        near: {{ scalar_t }},
        far: {{ scalar_t }},
    ) -> Self {
        let a = 2.0 / (right - left);
        let b = 2.0 / (top - bottom);
        let c = -2.0 / (far - near);
        let tx = -(right + left) / (right - left);
        let ty = -(top + bottom) / (top - bottom);
        let tz = -(far + near) / (far - near);

        Self::from_cols(
            {{ col_t }}::new(a, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, b, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, c, 0.0),
            {{ col_t }}::new(tx, ty, tz, 1.0),
        )
    }

    /// Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
    #[inline]
    pub fn orthographic_lh(
        left: {{ scalar_t }},
        right: {{ scalar_t }},
        bottom: {{ scalar_t }},
        top: {{ scalar_t }},
        near: {{ scalar_t }},
        far: {{ scalar_t }},
    ) -> Self {
        let rcp_width = 1.0 / (right - left);
        let rcp_height = 1.0 / (top - bottom);
        let r = 1.0 / (far - near);
        Self::from_cols(
            {{ col_t }}::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, r, 0.0),
            {{ col_t }}::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                -r * near,
                1.0,
            ),
        )
    }

    /// Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
    #[inline]
    pub fn orthographic_rh(
        left: {{ scalar_t }},
        right: {{ scalar_t }},
        bottom: {{ scalar_t }},
        top: {{ scalar_t }},
        near: {{ scalar_t }},
        far: {{ scalar_t }},
    ) -> Self {
        let rcp_width = 1.0 / (right - left);
        let rcp_height = 1.0 / (top - bottom);
        let r = 1.0 / (near - far);
        Self::from_cols(
            {{ col_t }}::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            {{ col_t }}::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            {{ col_t }}::new(0.0, 0.0, r, 0.0),
            {{ col_t }}::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                r * near,
                1.0,
            ),
        )
    }

    /// Transforms the given 3D vector as a point, applying perspective correction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    /// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
    ///
    /// This method assumes that `self` contains a projective transform.
    #[inline]
    pub fn project_point3(&self, rhs: {{ vec3_t }}) -> {{ vec3_t }} {
        let mut res = self.x_axis.mul(rhs.x);
        res = self.y_axis.mul(rhs.y).add(res);
        res = self.z_axis.mul(rhs.z).add(res);
        res = self.w_axis.add(res);
        res = res.mul(res.wwww().recip());
        res.xyz()
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
    pub fn transform_point3(&self, rhs: {{ vec3_t }}) -> {{ vec3_t }} {
        glam_assert!(self.row(3) == {{ vec4_t }}::W);
        let mut res = self.x_axis.mul(rhs.x);
        res = self.y_axis.mul(rhs.y).add(res);
        res = self.z_axis.mul(rhs.z).add(res);
        res = self.w_axis.add(res);
        res.xyz()
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
    pub fn transform_vector3(&self, rhs: {{ vec3_t }}) -> {{ vec3_t }} {
        glam_assert!(self.row(3) == {{ vec4_t }}::W);
        let mut res = self.x_axis.mul(rhs.x);
        res = self.y_axis.mul(rhs.y).add(res);
        res = self.z_axis.mul(rhs.z).add(res);
        res.xyz()
    }

{% endif %}

{% if self_t == "Mat4" %}
    /// Transforms the given `Vec3A` as 3D point.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `1.0`.
    #[inline]
    pub fn transform_point3a(&self, rhs: Vec3A) -> Vec3A {
        {% if is_scalar %}
            self.transform_point3(rhs.into()).into()
        {% else %}
            let mut res = self.x_axis.mul(rhs.xxxx());
            res = self.y_axis.mul(rhs.yyyy()).add(res);
            res = self.z_axis.mul(rhs.zzzz()).add(res);
            res = self.w_axis.add(res);
            res.into()
        {% endif %}
    }

    /// Transforms the give `Vec3A` as 3D vector.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `0.0`.
    #[inline]
    pub fn transform_vector3a(&self, rhs: Vec3A) -> Vec3A {
        {% if is_scalar %}
            self.transform_vector3(rhs.into()).into()
        {% else %}
            let mut res = self.x_axis.mul(rhs.xxxx());
            res = self.y_axis.mul(rhs.yyyy()).add(res);
            res = self.z_axis.mul(rhs.zzzz()).add(res);
            res.into()
        {% endif %}
    }
{% endif %}

    /// Transforms a {{ dim }}D vector.
    #[inline]
    pub fn mul_vec{{ dim }}(&self, rhs: {{ vecn_t }}) -> {{ vecn_t }} {
        {% if self_t == "Mat2" and is_sse2 %}
            unsafe {
                use core::mem::MaybeUninit;
                use crate::Align16;
                let abcd = self.0;
                let xxyy = _mm_set_ps(rhs.y, rhs.y, rhs.x, rhs.x);
                let axbxcydy = _mm_mul_ps(abcd, xxyy);
                let cydyaxbx = _mm_shuffle_ps(axbxcydy, axbxcydy, 0b01_00_11_10);
                let result = _mm_add_ps(axbxcydy, cydyaxbx);
                let mut out: MaybeUninit<Align16<Vec2>> = MaybeUninit::uninit();
                _mm_store_ps(out.as_mut_ptr().cast(), result);
                out.assume_init().0
            }
        {% elif self_t == "Mat2" and is_wasm32 %}
            use core::mem::MaybeUninit;
            let abcd = self.0;
            let xxyy = f32x4(rhs.x, rhs.x, rhs.y, rhs.y);
            let axbxcydy = f32x4_mul(abcd, xxyy);
            let cydyaxbx = i32x4_shuffle::<2, 3, 4, 5>(axbxcydy, axbxcydy);
            let result = f32x4_add(axbxcydy, cydyaxbx);
            let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
            unsafe {
                v128_store(out.as_mut_ptr(), result);
                *(&out.assume_init() as *const v128 as *const Vec2)
            }
        {% elif dim == 2 %}
            #[allow(clippy::suspicious_operation_groupings)]
            {{ col_t }}::new(
                (self.x_axis.x * rhs.x) + (self.y_axis.x * rhs.y),
                (self.x_axis.y * rhs.x) + (self.y_axis.y * rhs.y),
            )
        {% elif self_t == "Mat3A" %}
            {# use the Vec3A implementation #}
            self.mul_vec3a(rhs.into()).into()
        {% elif dim == 3 %}
            let mut res = self.x_axis.mul(rhs.x);
            res = res.add(self.y_axis.mul(rhs.y));
            res = res.add(self.z_axis.mul(rhs.z));
            res
        {% elif dim == 4 %}
            {% if is_scalar %}
                let mut res = self.x_axis.mul(rhs.x);
                res = res.add(self.y_axis.mul(rhs.y));
                res = res.add(self.z_axis.mul(rhs.z));
                res = res.add(self.w_axis.mul(rhs.w));
                res
            {% else %}
                {# use swizzles if simd #}
                let mut res = self.x_axis.mul(rhs.xxxx());
                res = res.add(self.y_axis.mul(rhs.yyyy()));
                res = res.add(self.z_axis.mul(rhs.zzzz()));
                res = res.add(self.w_axis.mul(rhs.wwww()));
                res
            {% endif %}
        {% endif %}
    }

{% if self_t == "Mat3" %}
    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, rhs: Vec3A) -> Vec3A {
        self.mul_vec3(rhs.into()).into()
    }
{% elif self_t == "Mat3A" %}
    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, rhs: Vec3A) -> Vec3A {
        let mut res = self.x_axis.mul(rhs.xxx());
        res = res.add(self.y_axis.mul(rhs.yyy()));
        res = res.add(self.z_axis.mul(rhs.zzz()));
        res
    }
{% endif %}

    /// Multiplies two {{ nxn }} matrices.
    #[inline]
    pub fn mul_mat{{ dim }}(&self, rhs: &Self) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            unsafe {
                let abcd = self.0;
                let rhs = rhs.0;
                let xxyy0 = _mm_shuffle_ps(rhs, rhs, 0b01_01_00_00);
                let xxyy1 = _mm_shuffle_ps(rhs, rhs, 0b11_11_10_10);
                let axbxcydy0 = _mm_mul_ps(abcd, xxyy0);
                let axbxcydy1 = _mm_mul_ps(abcd, xxyy1);
                let cydyaxbx0 = _mm_shuffle_ps(axbxcydy0, axbxcydy0, 0b01_00_11_10);
                let cydyaxbx1 = _mm_shuffle_ps(axbxcydy1, axbxcydy1, 0b01_00_11_10);
                let result0 = _mm_add_ps(axbxcydy0, cydyaxbx0);
                let result1 = _mm_add_ps(axbxcydy1, cydyaxbx1);
                Self(_mm_shuffle_ps(result0, result1, 0b01_00_01_00))
            }
        {% elif self_t == "Mat2" and is_wasm32 %}
            let abcd = self.0;
            let rhs = rhs.0;
            let xxyy0 = i32x4_shuffle::<0, 0, 5, 5>(rhs, rhs);
            let xxyy1 = i32x4_shuffle::<2, 2, 7, 7>(rhs, rhs);
            let axbxcydy0 = f32x4_mul(abcd, xxyy0);
            let axbxcydy1 = f32x4_mul(abcd, xxyy1);
            let cydyaxbx0 = i32x4_shuffle::<2, 3, 4, 5>(axbxcydy0, axbxcydy0);
            let cydyaxbx1 = i32x4_shuffle::<2, 3, 4, 5>(axbxcydy1, axbxcydy1);
            let result0 = f32x4_add(axbxcydy0, cydyaxbx0);
            let result1 = f32x4_add(axbxcydy1, cydyaxbx1);
            Self(i32x4_shuffle::<0, 1, 4, 5>(result0, result1))
        {% else %}
            Self::from_cols(
                {% for axis in axes %}
                    self.mul(rhs.{{ axis }}),
                {%- endfor %}
            )
        {% endif %}
    }

    /// Adds two {{ nxn }} matrices.
    #[inline]
    pub fn add_mat{{ dim }}(&self, rhs: &Self) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            Self(unsafe { _mm_add_ps(self.0, rhs.0) })
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(f32x4_add(self.0, rhs.0))
        {% else %}
            Self::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.add(rhs.{{ axis }}),
                {%- endfor %}
            )
        {% endif %}
    }

    /// Subtracts two {{ nxn }} matrices.
    #[inline]
    pub fn sub_mat{{ dim }}(&self, rhs: &Self) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            Self(unsafe { _mm_sub_ps(self.0, rhs.0) })
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(f32x4_sub(self.0, rhs.0))
        {% else %}
            Self::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.sub(rhs.{{ axis }}),
                {%- endfor %}
            )
        {% endif %}
    }

    /// Multiplies a {{ nxn }} matrix by a scalar.
    #[inline]
    pub fn mul_scalar(&self, rhs: {{ scalar_t }}) -> Self {
        {% if self_t == "Mat2" and is_sse2 %}
            Self(unsafe { _mm_mul_ps(self.0, _mm_set_ps1(rhs)) })
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(f32x4_mul(self.0, f32x4_splat(rhs)))
        {% else %}
            Self::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.mul(rhs),
                {%- endfor %}
            )
        {% endif %}
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two matrices contain similar elements. It works best
    /// when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: {{ scalar_t }}) -> bool {
        {% for axis in axes %}
            self.{{ axis }}.abs_diff_eq(rhs.{{ axis }}, max_abs_diff)
                {% if not loop.last %} && {% endif %}
        {% endfor %}
    }

    {% if scalar_t == "f32" %}
        #[inline]
        pub fn as_dmat{{ dim }}(&self) -> DMat{{ dim }} {
            DMat{{ dim }}::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.as_dvec{{ dim }}(),
                {% endfor %}
            )
        }
    {% elif scalar_t == "f64" %}
        #[inline]
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
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.add_mat{{ dim }}(&rhs)
    }
}

impl AddAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add_mat{{ dim }}(&rhs);
    }
}

impl Sub<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_mat{{ dim }}(&rhs)
    }
}

impl SubAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub_mat{{ dim }}(&rhs);
    }
}

impl Neg for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        {% if self_t == "Mat2" and is_sse2 %}
            Self(unsafe { _mm_xor_ps(self.0, _mm_set1_ps(-0.0)) })
        {% elif self_t == "Mat2" and is_wasm32 %}
            Self(f32x4_neg(self.0))
        {% else %}
            Self::from_cols(
                {% for axis in axes %}
                    self.{{ axis }}.neg(),
                {%- endfor %}
            )
        {% endif %}
    }
}

impl Mul<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat{{ dim }}(&rhs)
    }
}

impl MulAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul_mat{{ dim }}(&rhs);
    }
}

impl Mul<{{ col_t }}> for {{ self_t }} {
    type Output = {{ col_t }};
    #[inline]
    fn mul(self, rhs: {{ col_t }}) -> Self::Output {
        {% if self_t == "Mat3A" %}
            self.mul_vec3a(rhs)
        {% else %}
            self.mul_vec{{ dim }}(rhs)
        {% endif %}
    }
}

impl Mul<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn mul(self, rhs: {{ self_t }}) -> Self::Output {
        rhs.mul_scalar(self)
    }
}

impl Mul<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: {{ scalar_t }}) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl MulAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline]
    fn mul_assign(&mut self, rhs: {{ scalar_t }}) {
        *self = self.mul_scalar(rhs);
    }
}

{% if self_t == "Mat3" %}
impl Mul<Vec3A> for Mat3 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        self.mul_vec3a(rhs)
    }
}

impl From<Mat3A> for Mat3 {
    #[inline]
    fn from(m: Mat3A) -> Self {
        Self {
            x_axis: m.x_axis.into(),
            y_axis: m.y_axis.into(),
            z_axis: m.z_axis.into(),
        }
    }
}
{% elif self_t == "Mat3A" %}
impl Mul<Vec3> for Mat3A {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3a(rhs.into()).into()
    }
}

impl From<Mat3> for Mat3A {
    #[inline]
    fn from(m: Mat3) -> Self {
        Self {
            x_axis: m.x_axis.into(),
            y_axis: m.y_axis.into(),
            z_axis: m.z_axis.into(),
        }
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
    fn eq(&self, rhs: &Self) -> bool {
        {% for axis in axes %}
            self.{{ axis }}.eq(&rhs.{{ axis }}) {% if not loop.last %} && {% endif %}
        {% endfor %}
    }
}

{% if not is_align %}
#[cfg(not(target_arch = "spirv"))]
impl AsRef<[{{ scalar_t }}; {{ size }}]> for {{ self_t }} {
    #[inline]
    fn as_ref(&self) -> &[{{ scalar_t }}; {{ size }}] {
        unsafe { &*(self as *const Self as *const [{{ scalar_t }}; {{ size }}]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[{{ scalar_t }}; {{ size }}]> for {{ self_t }} {
    #[inline]
    fn as_mut(&mut self) -> &mut [{{ scalar_t }}; {{ size }}] {
        unsafe { &mut *(self as *mut Self as *mut [{{ scalar_t }}; {{ size }}]) }
    }
}
{% endif %}

{% if self_t == "Mat2" and not is_scalar %}
impl core::ops::Deref for Mat2 {
    type Target = crate::deref::Columns2<Vec2>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl core::ops::DerefMut for Mat2 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}
{% endif %}

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

