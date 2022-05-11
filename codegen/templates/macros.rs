{% macro make_tuple_t(t, n) %}
    (
        {%- for i in range(end=n) -%}
            {{ t }},
        {%- endfor -%}
    )
{% endmacro make_tuple_t %}
