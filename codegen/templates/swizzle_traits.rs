// Generated from {{template_path}} template. Edit the template, not the generated file.

{% set components = ["x", "y", "z", "w"] %}
{% set dimensions = [2, 3, 4] %}
{% for dim in dimensions %}
    {% set ret2 = "Self::Vec2" %}
    {% set ret3 = "Self::Vec3" %}
    {% set ret4 = "Self::Vec4" %}

    {% if dim == 4 %}
        {% set ret4 = "Self" %}
    {% elif dim == 3 %}
        {% set ret3 = "Self" %}
    {% elif dim == 2 %}
        {% set ret2 = "Self" %}
    {% endif %}

    pub trait Vec{{ dim }}Swizzles: Sized + Copy + Clone {
        {% if dim != 2 %}
        type Vec2;
        {% endif %}
        {% if dim != 3 %}
        type Vec3;
        {% endif %}
        {% if dim != 4 %}
        type Vec4;
        {% endif %}

        {% if dim == 4 %}
        #[inline]
        fn xyzw(self) -> Self {
            self
        }
        {% elif dim == 3 %}
        #[inline]
        fn xyz(self) -> Self {
            self
        }
        {% elif dim == 2 %}
        #[inline]
        fn xy(self) -> Self {
            self
        }
        {% endif %}

        {% for i in dimensions %}
            {% for e0 in components | slice(end=dim) %}
                {% for e1 in components | slice(end=dim) %}
                    {% if i == 2 %}
                        {% set skip = dim == 2 and e0 == "x" and e1 == "y" %}
                        {% if not skip %}
                            fn {{e0}}{{e1}}(self) -> {{ret2}};
                        {% endif %}
                    {% else %}
                        {% for e2 in components | slice(end=dim) %}
                            {% if i == 3 %}
                                {% set skip = dim == 3 and e0 == "x" and e1 == "y" and e2 == "z" %}
                                {% if not skip %}
                                    fn {{e0}}{{e1}}{{e2}}(self) -> {{ret3}};
                                {% endif %}
                            {% else %}
                                {% for e3 in components | slice(end=dim) %}
                                    {% set skip = dim == 4 and e0 == "x" and e1 == "y" and e2 == "z" and e3 == "w" %}
                                    {% if not skip %}
                                        fn {{e0}}{{e1}}{{e2}}{{e3}}(self) -> {{ret4}};
                                    {% endif %}
                                {% endfor %}
                            {% endif %}
                        {% endfor %}
                    {% endif %}
                {% endfor %}
            {% endfor %}
        {% endfor %}
    }
{% endfor %}
