// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if is_scalar %}
    {% set self_t = "BVec" ~ dim %}
    {% set scalar_t = "bool" %}
    {% set align = 1 %}
    {% set bytes = dim %}
    {% if dim == 2 %}
        {% set inner_t = "crate::XY::<bool>" %}
    {% elif dim == 3 %}
        {% set inner_t = "crate::XYZ::<bool>" %}
    {% elif dim == 4 %}
        {% set inner_t = "crate::XYZW::<bool>" %}
    {% endif %}
{% else %}
    {% set self_t = "BVec" ~ dim ~ "A" %}
    {% set scalar_t = "u32" %}
    {% set align = 16 %}
    {% set bytes = align %}
    {% if is_sse2 %}
        {% set inner_t = "__m128" %}
    {% elif is_wasm32 %}
        {% set inner_t = "v128" %}
    {% endif %}
{% endif %}

{% if dim == 2 %}
    {% set trait_t = "MaskVector2" %}
{% elif dim == 3 %}
    {% set trait_t = "MaskVector3" %}
{% elif dim == 4 %}
    {% set trait_t = "MaskVector4" %}
{% endif %}

use crate::core::traits::vector::{
    MaskVector, {{ trait_t }}, MaskVectorConst,
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

{% if is_sse2 %}
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
{% elif is_wasm32 %}
use core::arch::wasm32::*;
{% endif %}

{% if is_scalar %}
/// A {{ dim }}-dimensional boolean vector.
{% else %}
/// A {{ dim }}-dimensional SIMD vector mask.
///
/// This type is {{ align }} byte aligned and is backed by a SIMD vector. If SIMD is not available
/// `{{ self_t }}` will be a type alias for `BVec{{ dim }}`.
{% endif %}
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct {{ self_t }}(pub(crate) {{ inner_t }});

impl {{ self_t }} {
    /// Creates a new vector mask.
    #[inline]
{%- if dim == 2 %}
    pub fn new(x: bool, y: bool) -> Self {
        Self(MaskVector2::new(x, y))
    }
{%- elif dim == 3 %}
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(MaskVector3::new(x, y, z))
    }
{%- elif dim == 4 %}
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self(MaskVector4::new(x, y, z, w))
    }
{% endif %}

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        {{ trait_t }}::bitmask(self.0)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        {{ trait_t }}::any(self.0)
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        {{ trait_t }}::all(self.0)
    }
}


impl Default for {{ self_t }} {
    #[inline]
    fn default() -> Self {
        Self({{ inner_t }}::FALSE)
    }
}

impl PartialEq for {{ self_t }} {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bitmask().eq(&other.bitmask())
    }
}

impl Eq for {{ self_t }} {}

impl hash::Hash for {{ self_t }} {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(MaskVector::bitand(self.0, other.0))
    }
}

impl BitAndAssign for {{ self_t }} {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitand(self.0, other.0);
    }
}

impl BitOr for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(MaskVector::bitor(self.0, other.0))
    }
}

impl BitOrAssign for {{ self_t }} {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitor(self.0, other.0);
    }
}

impl Not for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(MaskVector::not(self.0))
    }
}

impl From<{{ self_t }}> for {{ inner_t }} {
    #[inline]
    fn from(t: {{ self_t }}) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_u32_array();
{% if dim == 2 %}
        write!(f, "{}({:#x}, {:#x})", stringify!({{ self_t }}), arr[0], arr[1])
{% elif dim == 3 %}
        write!(f, "{}({:#x}, {:#x}, {:#})", stringify!({{ self_t }}), arr[0], arr[1], arr[2])
{% elif dim == 4 %}
        write!(f, "{}({:#x}, {:#x}, {:#}, {:#})", stringify!({{ self_t }}), arr[0], arr[1], arr[2], arr[3])
{% endif %}
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_bool_array();
{% if dim == 2 %}
        write!(f, "[{}, {}]", arr[0], arr[1])
{% elif dim == 3 %}
        write!(f, "[{}, {}, {}]", arr[0], arr[1], arr[2])
{% elif dim == 4 %}
        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
{% endif %}
    }
}

impl From<{{ self_t }}> for [bool; {{ dim }}] {
    #[inline]
    fn from(mask: {{ self_t }}) -> Self {
        mask.0.into_bool_array()
    }
}

impl From<{{ self_t }}> for [u32; {{ dim }}] {
    #[inline]
    fn from(mask: {{ self_t }}) -> Self {
        mask.0.into_u32_array()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline]
    fn as_ref(&self) -> &[{{ scalar_t }}; {{ dim }}] {
        unsafe { &*(self as *const Self as *const [{{ scalar_t }}; {{ dim }}]) }
    }
}
