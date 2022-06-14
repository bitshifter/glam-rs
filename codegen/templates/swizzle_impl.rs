// Generated from {{template_path}} template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

{# component indices #}
{% set indices = [0, 1, 2, 3] %}

{# vector dimensions #}
{% set dimensions = [2, 3, 4] %}

{# element names #}
{% set e = ["x", "y", "z", "w"] %}

{# shuffle bits (sse2) #}
{% set b = ["00", "01", "10", "11"] %}

{# low index (wasm32) #}
{% set l = ["0", "1", "2", "3"] %}

{# high index (wasm32) #}
{% set h = ["4", "5", "6", "7"] %}

use crate::{
    {{vec2_t}}, {{vec3_t}}, {{vec4_t}},
};
use super::Vec{{ dim }}Swizzles;

{% if is_sse2 %}
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
{% elif is_wasm32 %}
use core::arch::wasm32::*;
{% endif %}

impl Vec{{ dim }}Swizzles for {{ self_t }} {
    {% if dim != 2 %}
    type Vec2 = {{ vec2_t }};
    {% endif %}
    {% if dim != 3 %}
    type Vec3 = {{ vec3_t }};
    {% endif %}
    {% if dim != 4 %}
    type Vec4 = {{ vec4_t }};
    {% endif %}

    {% for i in dimensions %}
        {% for j0 in indices | slice(end=dim) %}
            {% for j1 in indices | slice(end=dim) %}
                {% if i == 2 %}
                    {% set skip = dim == 2 and j0 == "x" and j1 == "y" %}
                    {% if not skip %}
    #[inline]
    fn {{ e[j0] }}{{ e[j1] }}(self) -> {{ vec2_t }} {
            {{ vec2_t }} { x: self.{{ e[j0] }}, y: self.{{ e[j1] }} }
    }
                    {% endif %}
                {% else %}
                    {% for j2 in indices | slice(end=dim) %}
                        {% if i == 3 %}
                            {% set skip = dim == 3 and j0 == "x" and j1 == "y" and j2 == "z" %}
                            {% if not skip %}
    #[inline]
    fn {{ e[j0] }}{{ e[j1] }}{{ e[j2] }}(self) -> {{ vec3_t }} {
            {% if vec3_t == "Vec3A" and is_sse2 %}
                {{ vec3_t }}((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_{{ b[j2] }}_{{ b[j1] }}_{{ b[j0] }}) }).into())
            {% elif vec3_t == "Vec3A" and is_wasm32 %}
                {{ vec3_t }}(i32x4_shuffle::<{{ l[j0] }}, {{ l[j1] }}, {{ h[j2] }}, {{ h[0] }}>(self.0, self.0).into())
            {% else %}
                {{ vec3_t }} { x: self.{{ e[j0] }}, y: self.{{ e[j1] }}, z: self.{{ e[j2] }} }
            {% endif %}
    }
                            {% endif %}
                        {% else %}
                            {% for j3 in indices | slice(end=dim) %}
                                {% set skip = dim == 4 and j0 == "x" and j1 == "y" and j2 == "z" and j3 == "w" %}
                                {% if not skip %}
    #[inline]
    fn {{ e[j0] }}{{ e[j1] }}{{ e[j2] }}{{ e[j3] }}(self) -> {{ vec4_t }} {
            {% if is_sse2 %}
                {{ vec4_t }}(unsafe { _mm_shuffle_ps(self.0, self.0, 0b{{ b[j3] }}_{{ b[j2] }}_{{ b[j1] }}_{{ b[j0] }}) })
            {% elif is_wasm32 %}
                {{ vec4_t}}(i32x4_shuffle::<{{ l[j0] }}, {{ l[j1] }}, {{ h[j2] }}, {{ h[j3] }}>(self.0, self.0))
            {% else %}
                {{ vec4_t }}::new(self.{{ e[j0] }}, self.{{ e[j1] }}, self.{{ e[j2] }}, self.{{ e[j3] }})
            {% endif %}
    }
                                {% endif %}
                            {% endfor %}
                        {% endif %}
                    {% endfor %}
                {% endif %}
            {% endfor %}
        {% endfor %}
    {% endfor %}
}

