{% import "macros.rs" as macros %}

// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if dim == 2 %}
    {% set deref_t = "XY::<" ~ scalar_t ~ ">" %}
{% elif dim == 3 %}
    {% set deref_t = "XYZ::<" ~ scalar_t ~ ">" %}
{% elif dim == 4 %}
    {% set deref_t = "XYZW::<" ~ scalar_t ~ ">" %}
{% endif %}

{% if is_scalar %}
    {% if scalar_t == "f32" and is_align %}
        {% set inner_t = "XYZF32A16" %}
    {% else %}
        {% set inner_t = deref_t %}
    {% endif %}
    {% set mask_t = "BVec" ~ dim %}
{% else %}
    {% set is_simd = true %}
    {% if is_sse2 %}
        {% set inner_t = "__m128" %}
    {% elif is_wasm32 %}
        {% set inner_t = "v128" %}
    {% endif %}
    {% set mask_t = "BVec" ~ dim ~ "A" %}
{% endif %}

{% set bytes = dim * 4 %}

{% if scalar_t == "f32" or scalar_t == "f64" %}
    {% set is_signed = true %}
    {% set is_float = true %}
    {% set trait_t = "FloatVector" ~ dim %}
    {% if scalar_t == "f32" %}
        {% if dim == 3 and is_simd or is_align %}
            {% set self_t = "Vec3A" %}
            {% set bytes = 16 %}
        {% else %}
            {% set self_t = "Vec" ~ dim %}
        {% endif %}
        {% set vec2_t = "Vec2" %}
        {% set vec3_t = "Vec3" %}
        {% set vec3a_t = "Vec3A" %}
        {% set vec4_t = "Vec4" %}
    {% elif scalar_t == "f64" %}
        {% set self_t = "DVec" ~ dim %}
        {% set vec2_t = "DVec2" %}
        {% set vec3_t = "DVec3" %}
        {% set vec4_t = "DVec4" %}
        {% set bytes = dim * 8 %}
    {% endif %}
{% elif scalar_t == "i32" %}
    {% set is_signed = true %}
    {% set is_float = false %}
    {% set trait_t = "SignedVector" ~ dim %}
    {% set self_t = "IVec" ~ dim %}
    {% set vec2_t = "IVec2" %}
    {% set vec3_t = "IVec3" %}
    {% set vec4_t = "IVec4" %}
{% else %}
    {% set is_signed = false %}
    {% set is_float = false %}
    {% set trait_t = "Vector" ~ dim %}
    {% set self_t = "UVec" ~ dim %}
    {% set vec2_t = "UVec2" %}
    {% set vec3_t = "UVec3" %}
    {% set vec4_t = "UVec4" %}
{% endif %}

{% if scalar_t == "f64" or dim == 4 %}
    {% set cuda_align = 16 %}
{% elif dim == 2 %}
    {% set cuda_align = 8 %}
{% endif %}

{% set components = ["x", "y", "z", "w"] | slice(end = dim) %}
{% set unit_x = [1, 0, 0, 0] %}
{% set unit_y = [0, 1, 0, 0] %}
{% set unit_z = [0, 0, 1, 0] %}
{% set unit_w = [0, 0, 0, 1] %}
{% set identity = [unit_x, unit_y, unit_z, unit_w] %}

use crate::{
    core::{
        storage::*,
        traits::vector::*,
    },
    {{ mask_t }},

    {% if self_t != vec2_t %}
        {{ vec2_t }},
    {% endif %}
    {% if self_t != vec3_t %}
        {{ vec3_t }},
    {% endif %}
    {% if self_t == "Vec3" or self_t == "Vec4" %}
        {{ vec3a_t }},
    {% endif %}
    {% if dim > 2 and self_t != vec4_t %}
        {{ vec4_t }},
    {% endif %}
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

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

/// Creates a {{ dim }}-dimensional vector.
#[inline(always)]
pub fn {{ self_t | lower }}(
    {% for c in components %}
        {{ c }}: {{ scalar_t }},
    {% endfor %}
) -> {{ self_t }} {
    {{ self_t }}::new({{ components | join(sep=",") }})
}

{% if self_t == "Vec3A" %}
/// A 3-dimensional vector with SIMD support.
///
/// This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
/// better performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
{% elif self_t == "Vec4" and is_simd %} 
/// A 4-dimensional vector with SIMD support.
///
/// This type uses 16 byte aligned SIMD vector type for storage.
{% else %}
/// A {{ dim }}-dimensional vector.
{%- endif %}
#[derive(Clone, Copy)]
{%- if self_t == "Vec4" and is_scalar %}
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
{%- elif dim != 3 and is_scalar %}
#[cfg_attr(feature = "cuda", repr(C, align({{ cuda_align }})))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
{% else %}
#[repr(transparent)]
{% endif -%}
pub struct {{ self_t }}(pub(crate) {{ inner_t }});

impl {{ self_t }} {
    /// All zeroes.
    pub const ZERO: Self = Self({{ inner_t }}::ZERO);

    /// All ones.
    pub const ONE: Self = Self({{ inner_t }}::ONE);

{% if is_float %}
    /// All NAN.
    pub const NAN: Self = Self(<{{ inner_t }} as crate::core::traits::scalar::NanConstEx>::NAN);
{% endif %}

{% for c in components %}
    {% set C = c | upper %}
    /// `[{{ identity[loop.index0] | slice(end=dim) | join(sep=", ") }}]`: a unit-length vector pointing along the positive {{ C }} axis.
    pub const {{ C }}: Self = Self(<{{ inner_t }} as Vector{{ dim }}Const>::{{ C }});
{% endfor %}

    /// The unit axes.
    pub const AXES: [Self; {{ dim }}] = [
        {% for c in components %}
            Self::{{ c | upper }},
        {% endfor %}
    ];

    /// Creates a new vector.
    #[inline(always)]
    pub fn new(
        {% for c in components %}
            {{ c }}: {{ scalar_t }},
        {% endfor %}
    ) -> Self {
        Self(Vector{{ dim }}::new({{ components | join(sep=",") }}))
    }

{% if dim == 2 %}
    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline(always)]
    pub fn extend(self, z: {{ scalar_t }}) -> {{ vec3_t }} {
        {{ vec3_t }}::new(self.x, self.y, z)
    }
{% elif dim == 3 %}
    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline(always)]
    pub fn extend(self, w: {{ scalar_t }}) -> {{ vec4_t }} {
        {{ vec4_t }}::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using `self.xy()` or `{{ vec2_t }}::from()`.
    #[inline(always)]
    pub fn truncate(self) -> {{ vec2_t }} {
        {{ vec2_t }}(Vector3::into_xy(self.0))
    }
{% elif dim == 4 %}
    /// Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to `{{ vec3_t }}` may also be performed by using `self.xyz()` or `{{ vec3_t }}::from()`.
{%- if scalar_t == "f32" %}
    ///
    /// To truncate to `Vec3A` use `Vec3A::from()`.
{%- endif %}
    #[inline(always)]
    pub fn truncate(self) -> {{ vec3_t }} {
        {{ vec3_t }}::new(self.x, self.y, self.z)
    }
{% endif %}

    /// `[{{ components | join(sep=", ") }}]`
    #[inline(always)]
    pub fn to_array(&self) -> [{{ scalar_t }}; {{ dim }}] {
        [
            {% for c in components %}
                self.{{ c }},
            {% endfor %}
        ]
    }

    /// Creates a vector with all elements set to `v`.
    #[inline(always)]
    pub fn splat(v: {{ scalar_t }}) -> Self {
        Self({{ inner_t }}::splat(v))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline(always)]
    pub fn select(mask: {{ mask_t }}, if_true: Self, if_false: Self) -> Self {
        Self({{ inner_t }}::select(mask.0, if_true.0, if_false.0))
    }

    /// Computes the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> {{ scalar_t }} {
        <{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::dot(self.0, other.0)
    }

    {% if dim == 3 %}
    /// Computes the cross product of `self` and `other`.
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        Self(self.0.cross(other.0))
    }
    {% endif %}

    /// Returns a vector containing the minimum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns a vector containing the maximum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(<{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::clamp(self.0, min.0, max.0))
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline(always)]
    pub fn min_element(self) -> {{ scalar_t }} {
        <{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::min_element(self.0)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline(always)]
    pub fn max_element(self) -> {{ scalar_t }} {
        <{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::max_element(self.0)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpeq(self, other: Self) -> {{ mask_t }} {
        {{ mask_t }}(self.0.cmpeq(other.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpne(self, other: Self) -> {{ mask_t }} {
        {{ mask_t }}(self.0.cmpne(other.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpge(self, other: Self) -> {{ mask_t }} {
        {{ mask_t }}(self.0.cmpge(other.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpgt(self, other: Self) -> {{ mask_t }} {
        {{ mask_t }}(self.0.cmpgt(other.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmple(self, other: Self) -> {{ mask_t }} {
        {{ mask_t }}(self.0.cmple(other.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmplt(self, other: Self) -> {{ mask_t }} {
        {{ mask_t }}(self.0.cmplt(other.0))
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn from_slice(slice: &[{{ scalar_t }}]) -> Self {
        Self(<{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first {{ dim }} elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [{{ scalar_t }}]) {
        <{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::write_to_slice_unaligned(self.0, slice)
    }

{% if is_signed %}
    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(<{{ inner_t }} as SignedVector{{ dim }}<{{ scalar_t }}>>::abs(self.0))
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        Self(<{{ inner_t }} as SignedVector{{ dim }}<{{ scalar_t }}>>::signum(self.0))
    }
{% endif %}

{% if is_float %}
    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline(always)]
    pub fn is_finite(self) -> bool {
        {{ trait_t }}::is_finite(self.0)
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline(always)]
    pub fn is_nan(self) -> bool {
        {{ trait_t }}::is_nan(self.0)
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline(always)]
    pub fn is_nan_mask(self) -> {{ mask_t }} {
        {{ mask_t }}({{ trait_t }}::is_nan_mask(self.0))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline(always)]
    pub fn length(self) -> {{ scalar_t }} {
        {{ trait_t }}::length(self.0)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline(always)]
    pub fn length_squared(self) -> {{ scalar_t }} {
        {{ trait_t }}::length_squared(self.0)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline(always)]
    pub fn length_recip(self) -> {{ scalar_t }} {
        {{ trait_t }}::length_recip(self.0)
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Self) -> {{ scalar_t }} {
        (self - other).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Self) -> {{ scalar_t }} {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        Self({{ trait_t }}::normalize(self.0))
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero`].
    #[must_use]
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize`].
    #[must_use]
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            self * rcp
        } else {
            Self::ZERO
        }
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline(always)]
    pub fn is_normalized(self) -> bool {
        {{ trait_t }}::is_normalized(self.0)
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        let other_len_sq_rcp = other.dot(other).recip();
        glam_assert!(other_len_sq_rcp.is_finite());
        other * self.dot(other) * other_len_sq_rcp
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `other`, in other words the result of `self - self.project_onto(other)`.
    ///
    /// `other` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `other` has a length of zero when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        glam_assert!(other.is_normalized());
        other * self.dot(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `other`, in other words the result of `self - self.project_onto(other)`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from_normalized(self, other: Self) -> Self {
        self - self.project_onto_normalized(other)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline(always)]
    pub fn round(self) -> Self {
        Self({{ trait_t }}::round(self.0))
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline(always)]
    pub fn floor(self) -> Self {
        Self({{ trait_t }}::floor(self.0))
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline(always)]
    pub fn ceil(self) -> Self {
        Self({{ trait_t }}::ceil(self.0))
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline(always)]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline(always)]
    pub fn exp(self) -> Self {
        Self({{ trait_t }}::exp(self.0))
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline(always)]
    pub fn powf(self, n: {{ scalar_t }}) -> Self {
        Self({{ trait_t }}::powf(self.0, n))
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline(always)]
    pub fn recip(self) -> Self {
        Self({{ trait_t }}::recip(self.0))
    }

    /// Performs a linear interpolation between `self` and `other` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `other`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(self, other: Self, s: {{ scalar_t }}) -> Self {
        self + ((other - self) * s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `other` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline(always)]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: {{ scalar_t }}) -> bool {
        {{ trait_t }}::abs_diff_eq(self.0, other.0, max_abs_diff)
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(self, min: {{ scalar_t }}, max: {{ scalar_t }}) -> Self {
        glam_assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`
    pub fn clamp_length_max(self, max: {{ scalar_t }}) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: {{ scalar_t }}) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self({{ trait_t }}::mul_add(self.0, a.0, b.0))
    }

{% if dim == 2 %}
    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
    /// create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
    #[inline(always)]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        Self({{ trait_t }}::from_angle(angle))
    }

    /// Returns the angle (in radians) between `self` and `other`.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline(always)]
    pub fn angle_between(self, other: Self) -> {{ scalar_t }} {
        self.0.angle_between(other.0)
    }
{% elif dim == 3 %}
    /// Returns the angle (in radians) between two vectors.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline(always)]
    pub fn angle_between(self, other: Self) -> {{ scalar_t }} {
        self.0.angle_between(other.0)
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit-length.
    /// For that use [`Self::any_orthonormal_vector`] instead.
    #[inline]
    pub fn any_orthogonal_vector(&self) -> Self {
        // This can probably be optimized
        if self.x.abs() > self.y.abs() {
            Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
        } else {
            Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
        }
    }

    /// Returns any unit-length vector that is orthogonal to the given one.
    /// The input vector must be finite and non-zero.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_vector(&self) -> Self {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_{{ scalar_t }}).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Given a unit-length vector return two other vectors that together form an orthonormal
    /// basis.  That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_pair(&self) -> (Self, Self) {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_{{ scalar_t }}).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }
{% endif %}
{% endif %}

{% if is_signed and dim == 2 %}
    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline(always)]
    pub fn perp(self) -> Self {
        Self(self.0.perp())
    }

    /// The perpendicular dot product of `self` and `other`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline(always)]
    pub fn perp_dot(self, other: Self) -> {{ scalar_t }} {
        self.0.perp_dot(other.0)
    }

    /// Returns `other` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[must_use]
    #[inline(always)]
    pub fn rotate(self, other: Self) -> Self {
        Self(self.0.rotate(other.0))
    }
{% endif %}

{% if scalar_t != "f32" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec2(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x as f32, self.y as f32)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec3(&self) -> crate::Vec3 {
        crate::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec3a(&self) -> crate::Vec3A {
        crate::Vec3A::new(self.x as f32, self.y as f32, self.z as f32)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec4(&self) -> crate::Vec4 {
        crate::Vec4::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }
    {% endif %}
{% endif %}
{% if scalar_t != "f64" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec2(&self) -> crate::DVec2 {
        crate::DVec2::new(self.x as f64, self.y as f64)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec4(&self) -> crate::DVec4 {
        crate::DVec4::new(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }
    {% endif %}
{% endif %}
{% if scalar_t != "i32" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec2(&self) -> crate::IVec2 {
        crate::IVec2::new(self.x as i32, self.y as i32)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec4(&self) -> crate::IVec4 {
        crate::IVec4::new(self.x as i32, self.y as i32, self.z as i32, self.w as i32)
    }
    {% endif %}
{% endif %}
{% if scalar_t != "u32" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `u32`.
    #[inline(always)]
    pub fn as_uvec2(&self) -> crate::UVec2 {
        crate::UVec2::new(self.x as u32, self.y as u32)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `u32`.
    #[inline(always)]
    pub fn as_uvec3(&self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `u32`.
    #[inline(always)]
    pub fn as_uvec4(&self) -> crate::UVec4 {
        crate::UVec4::new(self.x as u32, self.y as u32, self.z as u32, self.w as u32)
    }
    {% endif %}
{% endif %}
}

impl Default for {{ self_t }} {
    #[inline(always)]
    fn default() -> Self {
        Self({{ inner_t }}::ZERO)
    }
}

impl PartialEq for {{ self_t }} {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl Div<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: {{ self_t }}) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn div_assign(&mut self, other: {{ self_t }}) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: {{ scalar_t }}) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline(always)]
    fn div_assign(&mut self, other: {{ scalar_t }}) {
        self.0 = self.0.div_scalar(other)
    }
}

impl Div<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline(always)]
    fn div(self, other: {{ self_t }}) -> {{ self_t }} {
        {{ self_t }}({{ inner_t }}::splat(self).div(other.0))
    }
}

impl Mul<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: {{ self_t }}) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn mul_assign(&mut self, other: {{ self_t }}) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: {{ scalar_t }}) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline(always)]
    fn mul_assign(&mut self, other: {{ scalar_t }}) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline(always)]
    fn mul(self, other: {{ self_t }}) -> {{ self_t }} {
        {{ self_t }}({{ inner_t }}::splat(self).mul(other.0))
    }
}

impl Add<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: {{ self_t }}) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn add_assign(&mut self, other: {{ self_t }}) {
        self.0 = self.0.add(other.0)
    }
}

impl Add<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: {{ scalar_t }}) -> Self {
        Self(self.0.add_scalar(other))
    }
}

impl AddAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline(always)]
    fn add_assign(&mut self, other: {{ scalar_t }}) {
        self.0 = self.0.add_scalar(other)
    }
}

impl Add<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline(always)]
    fn add(self, other: {{ self_t }}) -> {{ self_t }} {
        {{ self_t }}({{ inner_t }}::splat(self).add(other.0))
    }
}

impl Sub<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: {{ self_t }}) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn sub_assign(&mut self, other: {{ self_t }}) {
        self.0 = self.0.sub(other.0)
    }
}

impl Sub<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: {{ scalar_t }}) -> Self {
        Self(self.0.sub_scalar(other))
    }
}

impl SubAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline(always)]
    fn sub_assign(&mut self, other: {{ scalar_t }}) {
        self.0 = self.0.sub_scalar(other)
    }
}

impl Sub<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline(always)]
    fn sub(self, other: {{ self_t }}) -> {{ self_t }} {
        {{ self_t }}({{ inner_t }}::splat(self).sub(other.0))
    }
}

impl Rem<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: {{ self_t }}) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl RemAssign<{{ self_t }}> for {{ self_t }} {
    #[inline(always)]
    fn rem_assign(&mut self, other: {{ self_t }}) {
        self.0 = self.0.rem(other.0)
    }
}

impl Rem<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: {{ scalar_t }}) -> Self {
        Self(self.0.rem_scalar(other))
    }
}

impl RemAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline(always)]
    fn rem_assign(&mut self, other: {{ scalar_t }}) {
        self.0 = self.0.rem_scalar(other)
    }
}

impl Rem<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline(always)]
    fn rem(self, other: {{ self_t }}) -> {{ self_t }} {
        {{ self_t }}({{ inner_t }}::splat(self).rem(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline(always)]
    fn as_ref(&self) -> &[{{ scalar_t }}; {{ dim }}] {
        unsafe { &*(self as *const {{ self_t }} as *const [{{ scalar_t }}; {{ dim }}]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [{{ scalar_t }}; {{ dim }}] {
        unsafe { &mut *(self as *mut {{ self_t }} as *mut [{{ scalar_t }}; {{ dim }}]) }
    }
}

impl<'a> Sum<&'a Self> for {{ self_t }} {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for {{ self_t }} {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

{% if is_signed %}
impl Neg for {{ self_t }} {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}
{% endif %}

{% if not is_float %}
impl Eq for {{ self_t }} {}

#[cfg(not(target_arch = "spirv"))]
impl core::hash::Hash for {{ self_t }} {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let inner: &[{{ scalar_t }}; {{ dim }}] = self.as_ref();
        inner.hash(state);
    }
}

impl Not for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        {{ self_t }}({{ inner_t }}::not(self.0))
    }
}

impl BitAnd for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        {{ self_t }}({{ inner_t }}::vector_bitand(self.0, rhs.0))
    }
}

impl BitOr for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        {{ self_t }}({{ inner_t }}::vector_bitor(self.0, rhs.0))
    }
}

impl BitXor for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        {{ self_t }}({{ inner_t }}::vector_bitxor(self.0, rhs.0))
    }
}

impl BitAnd<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: {{ scalar_t }}) -> Self::Output {
        {{ self_t }}({{ inner_t }}::scalar_bitand(self.0, rhs))
    }
}

impl BitOr<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: {{ scalar_t }}) -> Self::Output {
        {{ self_t }}({{ inner_t }}::scalar_bitor(self.0, rhs))
    }
}

impl BitXor<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: {{ scalar_t }}) -> Self::Output {
        {{ self_t }}({{ inner_t }}::scalar_bitxor(self.0, rhs))
    }
}

{% for rhs_t in ["i8", "i16", "i32", "u8", "u16", "u32"] %}
    impl Shl<{{ rhs_t }}> for {{ self_t }} {
        type Output = Self;

        #[inline(always)]
        fn shl(self, rhs: {{ rhs_t }}) -> Self::Output {
            {{ self_t }}({{ inner_t }}::scalar_shl(self.0, rhs))
        }
    }

    impl Shr<{{ rhs_t }}> for {{ self_t }} {
        type Output = Self;

        #[inline(always)]
        fn shr(self, rhs: {{ rhs_t }}) -> Self::Output {
            {{ self_t }}({{ inner_t }}::scalar_shr(self.0, rhs))
        }
    }
{% endfor %}

{% for rhs_t in ["crate::IVec" ~ dim, "crate::UVec" ~ dim] %}
        impl Shl<{{ rhs_t }}> for {{ self_t }} {
            type Output = Self;

            #[inline(always)]
            fn shl(self, rhs: {{ rhs_t }}) -> Self::Output {
                {{ self_t }}({{ inner_t }}::vector_shl(self.0, rhs.0))
            }
        }

        impl Shr<{{ rhs_t }}> for {{ self_t }} {
            type Output = Self;

            #[inline(always)]
            fn shr(self, rhs: {{ rhs_t }}) -> Self::Output {
                {{ self_t }}({{ inner_t }}::vector_shr(self.0, rhs.0))
            }
        }
{% endfor %}

{% endif %}

impl Index<usize> for {{ self_t }} {
    type Output = {{ scalar_t }};
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            {% for c in components %}
                {{ loop.index0 }} => &self.{{ c }},
            {% endfor %}
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for {{ self_t }} {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            {% for c in components %}
                {{ loop.index0 }} => &mut self.{{ c }},
            {% endfor %}
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        {% if dim == 2 %}
            write!(f, "[{}, {}]", self.x, self.y)
        {% elif dim == 3 %}
            write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
        {% elif dim == 4 %}
            write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
        {% endif %}
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for {{ self_t }} {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!({{ self_t }}))
            {% for c in components %}
                .field(&self.{{ c }})
            {% endfor %}
            .finish()
    }
}

impl From<{{ self_t }}> for {{ inner_t }} {
    #[inline(always)]
    fn from(t: {{ self_t }}) -> Self {
        t.0
    }
}

impl From<{{ inner_t }}> for {{ self_t }} {
    #[inline(always)]
    fn from(t: {{ inner_t }}) -> Self {
        Self(t)
    }
}

impl From<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline(always)]
    fn from(a: [{{ scalar_t }}; {{ dim }}]) -> Self {
        Self(<{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::from_array(a))
    }
}

impl From<{{ self_t }}> for [{{ scalar_t }}; {{ dim }}] {
    #[inline(always)]
    fn from(v: {{ self_t }}) -> Self {
        v.into_array()
    }
}

impl From<{{ macros::make_tuple_t(t=scalar_t, n=dim) }}> for {{ self_t }} {
    #[inline(always)]
    fn from(t: {{ macros::make_tuple_t(t=scalar_t, n=dim) }}) -> Self {
        Self(<{{ inner_t }} as Vector{{ dim }}<{{ scalar_t }}>>::from_tuple(t))
    }
}

impl From<{{ self_t }}> for {{ macros::make_tuple_t(t=scalar_t, n=dim) }} {
    #[inline(always)]
    fn from(v: {{ self_t }}) -> Self {
        Vector{{ dim }}::into_tuple(v.0)
    }
}

{% if self_t == "Vec3A" %}
impl From<Vec3> for Vec3A {
    #[inline(always)]
    fn from(v: Vec3) -> Self {
        Self(v.0.into())
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3A` from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on `x86_64` this conversion is a noop.
    #[inline(always)]
    fn from(v: Vec4) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(v.0.into())
    }
}
{% elif self_t == "Vec3" %}
impl From<Vec3A> for Vec3 {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        Self(v.0.into())
    }
}
{% elif self_t == "Vec4" %}
impl From<(Vec3A, f32)> for Vec4 {
    #[inline(always)]
    fn from((v, w): (Vec3A, f32)) -> Self {
        v.extend(w)
    }
}

impl From<(f32, Vec3A)> for Vec4 {
    #[inline(always)]
    fn from((x, v): (f32, Vec3A)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}
{% endif %}

{% if dim == 3 %}
impl From<({{ vec2_t }}, {{ scalar_t }})> for {{ self_t }} {
    #[inline(always)]
    fn from((v, z): ({{ vec2_t }}, {{ scalar_t }})) -> Self {
        Self::new(v.x, v.y, z)
    }
}
{% elif dim == 4 %}
impl From<({{ vec3_t }}, {{ scalar_t }})> for {{ self_t }} {
    #[inline(always)]
    fn from((v, w): ({{ vec3_t }}, {{ scalar_t }})) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }
}

impl From<({{ scalar_t }}, {{ vec3_t }})> for {{ self_t }} {
    #[inline(always)]
    fn from((x, v): ({{ scalar_t }}, {{ vec3_t }})) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

impl From<({{ vec2_t }}, {{ scalar_t }}, {{ scalar_t }})> for {{ self_t }} {
    #[inline(always)]
    fn from((v, z, w): ({{ vec2_t }}, {{ scalar_t }}, {{ scalar_t }})) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

impl From<({{ vec2_t }}, {{ vec2_t }})> for {{ self_t }} {
    #[inline(always)]
    fn from((v, u): ({{ vec2_t }}, {{ vec2_t }})) -> Self {
        Self::new(v.x, v.y, u.x, u.y)
    }
}
{% endif %}

impl Deref for {{ self_t }} {
    type Target = {{ deref_t }};
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_{{ components | join(sep="") }}()
    }
}

impl DerefMut for {{ self_t }} {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_{{ components | join(sep="") }}()
    }
}
