// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if is_scalar %}
    {% set self_t = "BVec" ~ dim %}
    {% set scalar_t = "bool" %}
    {% set align = 1 %}
{% else %}
    {% set self_t = "BVec" ~ dim ~ "A" %}
    {% set scalar_t = "u32" %}
    {% set align = 16 %}
    {% if is_sse2 %}
        {% set simd_t = "__m128" %}
    {% elif is_wasm32 %}
        {% set simd_t = "v128" %}
    {% endif %}
{% endif %}

{% set components = ["x", "y", "z", "w"] | slice(end = dim) %}

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

{% if is_sse2 %}
union UnionCast {
    a: [u32; 4],
    v: {{ self_t }}
}
{% endif %}

{% if is_scalar %}
/// A {{ dim }}-dimensional boolean vector.
{%- else %}
/// A {{ dim }}-dimensional SIMD vector mask.
///
/// This type is {{ align }} byte aligned and is backed by a SIMD vector. If SIMD is not available
/// `{{ self_t }}` will be a type alias for `BVec{{ dim }}`.
{%- endif %}
#[derive(Clone, Copy)]
{%- if is_scalar %}
pub struct {{ self_t }}
{
{% for c in components %}
    pub {{ c }}: bool,
{%- endfor %}
}
{%- else %}
#[repr(transparent)]
pub struct {{ self_t }}(pub(crate) {{ simd_t }});
{% endif %}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

const FALSE: {{ self_t }} = {{ self_t }}::new(
    {% for c in components %}
        false,
    {%- endfor %}
);

impl {{ self_t }} {

    /// Creates a new vector mask.
    #[inline(always)]
    pub const fn new(
        {% for c in components %}
            {{ c }}: bool,
        {% endfor %}
    ) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }},
                {%- endfor %}
            }
        {% elif is_sse2 %}
            unsafe {
                UnionCast { a: [
                    MASK[x as usize],
                    MASK[y as usize],
                    MASK[z as usize],
                    {% if dim == 3 %}
                        0,
                    {% elif dim == 4 %}
                        MASK[w as usize],
                    {% endif %}
                ] }.v
            }
        {% elif is_wasm32 %}
            Self(u32x4(
                MASK[x as usize],
                MASK[y as usize],
                MASK[z as usize],
                {% if dim == 3 %}
                    0,
                {% elif dim == 4 %}
                    MASK[w as usize],
                {% endif %}
            ))
        {% endif %}
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        {% if is_scalar %}
            {% for c in components %}
                (self.{{ c }} as u32) << {{ loop.index0 }} {% if not loop.last %} | {% endif %}
            {% endfor %}
        {% elif is_sse2 %}
            {% if dim == 3 %}
                unsafe { (_mm_movemask_ps(self.0) as u32) & 0x7 }
            {% elif dim == 4 %}
                unsafe { _mm_movemask_ps(self.0) as u32 }
            {% endif %}
        {% elif is_wasm32 %}
            {% if dim == 3 %}
                (u32x4_bitmask(self.0) & 0x7) as u32
            {% elif dim == 4 %}
                u32x4_bitmask(self.0) as u32
            {% endif %}
        {% endif %}
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }} {% if not loop.last %} || {% endif %}
            {%- endfor %}
        {% elif is_sse2 %}
            {% if dim == 3 %}
                unsafe { (_mm_movemask_ps(self.0) & 0x7) != 0 }
            {% elif dim == 4 %}
                unsafe { _mm_movemask_ps(self.0) != 0 }
            {% endif %}
        {% elif is_wasm32 %}
            {% if dim == 3 %}
                (u32x4_bitmask(self.0) & 0x7) != 0
            {% elif dim == 4 %}
                u32x4_bitmask(self.0) != 0
            {% endif %}
        {% endif %}
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }} {% if not loop.last %} && {% endif %}
            {%- endfor %}
        {% elif is_sse2 %}
            {% if dim == 3 %}
                unsafe { (_mm_movemask_ps(self.0) & 0x7) == 0x7 }
            {% elif dim == 4 %}
                unsafe { _mm_movemask_ps(self.0) == 0xf }
            {% endif %}
        {% elif is_wasm32 %}
            {% if dim == 3 %}
                (u32x4_bitmask(self.0) & 0x7) == 0x7
            {% elif dim == 4 %}
                u32x4_bitmask(self.0) == 0xf
            {% endif %}
        {% endif %}
    }

    #[inline]
    fn into_bool_array(self) -> [bool; {{ dim }}] {
        {% if is_scalar %}
            [
                {% for c in components %}
                    self.{{ c }},
                {%- endfor %}
            ]
        {% else %}
            {% set bits = [1, 2, 4, 8] | slice(end = dim) %}
            let bitmask = self.bitmask();
            [
                {% for b in bits %}
                    (bitmask & {{ b }}) != 0,
                {%- endfor %}
            ]
        {% endif %}
    }

    #[inline]
    fn into_u32_array(self) -> [u32; {{ dim }}] {
        {% if is_scalar %}
            [
                {% for c in components %}
                    MASK[self.{{ c }} as usize],
                {%- endfor %}
            ]
        {% else %}
            let bitmask = self.bitmask();
            [
                {% for c in components %}
                    MASK[((bitmask >> {{ loop.index0 }}) & 1) as usize],
                {%- endfor %}
            ]
        {% endif %}
    }
}


impl Default for {{ self_t }} {
    #[inline]
    fn default() -> Self {
        FALSE
    }
}

impl PartialEq for {{ self_t }} {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.bitmask().eq(&rhs.bitmask())
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
    fn bitand(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }} && rhs.{{ c }},
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_and_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(v128_and(self.0, rhs.0))
        {% endif %}
    }
}

impl BitAndAssign for {{ self_t }} {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }} || rhs.{{ c }},
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_or_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(v128_or(self.0, rhs.0))
        {% endif %}
    }
}

impl BitOrAssign for {{ self_t }} {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl Not for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: !self.{{ c }},
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe {
                _mm_andnot_ps(self.0, _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff)))
            })
        {% elif is_wasm32 %}
            Self(v128_not(self.0))
        {% endif %}
    }
}

{% if not is_scalar %}
impl From<{{ self_t }}> for {{ simd_t }} {
    #[inline]
    fn from(t: {{ self_t }}) -> Self {
        t.0
    }
}
{% endif %}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
{%- if dim == 2 %}
        write!(f, "{}({:#x}, {:#x})", stringify!({{ self_t }}), arr[0], arr[1])
{% elif dim == 3 %}
        write!(f, "{}({:#x}, {:#x}, {:#x})", stringify!({{ self_t }}), arr[0], arr[1], arr[2])
{% elif dim == 4 %}
        write!(f, "{}({:#x}, {:#x}, {:#x}, {:#x})", stringify!({{ self_t }}), arr[0], arr[1], arr[2], arr[3])
{% endif %}
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
{%- if dim == 2 %}
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
        mask.into_bool_array()
    }
}

impl From<{{ self_t }}> for [u32; {{ dim }}] {
    #[inline]
    fn from(mask: {{ self_t }}) -> Self {
        mask.into_u32_array()
    }
}
