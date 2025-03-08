use std::fs::File;
use std::io::{Result, Write};

const E: [char; 4] = ['x', 'y', 'z', 'w']; // element name
const V: [&str; 4] = ["1", "2", "3", "4"]; // element value
const V_RHS: [&str; 3] = ["11", "12", "13"]; // Right hand side element value

fn write_swizzle_head(out: &mut impl Write) -> Result<()> {
    writeln!(out, "// Generated by swizzlegen. Do not edit.")?;
    Ok(())
}

fn write_loops_vec4<W, F4>(out: &mut W, size: usize, vec4fn: F4) -> Result<()>
where
    W: Write,
    F4: Fn(&mut W, usize, usize, usize, usize) -> Result<()>,
{
    for e0 in 0..size {
        for e1 in 0..size {
            for e2 in 0..size {
                for e3 in 0..size {
                    if size == 4 && e0 == 0 && e1 == 1 && e2 == 2 && e3 == 3 {
                        continue;
                    }
                    vec4fn(out, e0, e1, e2, e3)?;
                }
            }
        }
    }
    Ok(())
}

fn write_loops_vec3<W, F3>(out: &mut W, size: usize, vec3fn: F3) -> Result<()>
where
    W: Write,
    F3: Fn(&mut W, usize, usize, usize) -> Result<()>,
{
    for e0 in 0..size {
        for e1 in 0..size {
            for e2 in 0..size {
                if size == 3 && e0 == 0 && e1 == 1 && e2 == 2 {
                    continue;
                }
                vec3fn(out, e0, e1, e2)?;
            }
        }
    }
    Ok(())
}

fn write_loops_vec2<W, F2>(out: &mut W, size: usize, vec2fn: F2) -> Result<()>
where
    W: Write,
    F2: Fn(&mut W, usize, usize) -> Result<()>,
{
    for e0 in 0..size {
        for e1 in 0..size {
            if size == 2 && e0 == 0 && e1 == 1 {
                continue;
            }
            vec2fn(out, e0, e1)?;
        }
    }
    Ok(())
}

fn write_test_vec4(
    out: &mut impl Write,
    t: &str,
    vec4t: &str,
    vec3t: &str,
    vec2t: &str,
) -> Result<()> {
    const SIZE: usize = 4;

    write!(
        out,
        r#"
glam_test!(test_{vec4t}_swizzles, {{
    let v = {vec4t}(1_{t}, 2_{t}, 3_{t}, 4_{t});
    let rhs3 = {vec3t}(11_{t}, 12_{t}, 13_{t});
    let rhs2 = {vec2t}(11_{t}, 12_{t});
"#,
    )?;

    writeln!(out, "    assert_eq!(v, v.xyzw());")?;

    write_test_loops(out, SIZE, t, vec4t, vec3t, vec2t)?;

    writeln!(out, "}});")?;

    Ok(())
}

fn write_test_vec3(
    out: &mut impl Write,
    t: &str,
    vec4t: &str,
    vec3t: &str,
    vec2t: &str,
) -> Result<()> {
    const SIZE: usize = 3;

    write!(
        out,
        r#"
glam_test!(test_{vec3t}_swizzles, {{
    let v = {vec3t}(1_{t}, 2_{t}, 3_{t});
    let rhs2 = {vec2t}(11_{t}, 12_{t});
"#,
    )?;

    writeln!(out, "    assert_eq!(v, v.xyz());")?;

    write_test_loops(out, SIZE, t, vec4t, vec3t, vec2t)?;

    writeln!(out, "}});")?;

    Ok(())
}

fn write_test_vec2(
    out: &mut impl Write,
    t: &str,
    vec4t: &str,
    vec3t: &str,
    vec2t: &str,
) -> Result<()> {
    const SIZE: usize = 2;

    write!(
        out,
        r#"
glam_test!(test_{}_swizzles, {{
    let v = {}(1_{}, 2_{});
"#,
        vec2t, vec2t, t, t,
    )?;

    writeln!(out, "    assert_eq!(v, v.xy());")?;

    write_test_loops(out, SIZE, t, vec4t, vec3t, vec2t)?;

    writeln!(out, "}});")?;

    Ok(())
}

fn write_test_loops(
    out: &mut impl Write,
    size: usize,
    t: &str,
    vec4t: &str,
    vec3t: &str,
    vec2t: &str,
) -> Result<()> {
    write_loops_vec4(out, size, |out, e0, e1, e2, e3| {
        writeln!(
            out,
            "    assert_eq!(v.{}{}{}{}(), {}({}_{}, {}_{}, {}_{}, {}_{}));",
            E[e0], E[e1], E[e2], E[e3], vec4t, V[e0], t, V[e1], t, V[e2], t, V[e3], t
        )
    })?;
    write_loops_vec3(out, size, |out, e0, e1, e2| {
        writeln!(
            out,
            "    assert_eq!(v.{}{}{}(), {}({}_{}, {}_{}, {}_{}));",
            E[e0], E[e1], E[e2], vec3t, V[e0], t, V[e1], t, V[e2], t,
        )
    })?;
    if size == 4 {
        // with_XXX swizzles
        write_loops_vec3(out, size, |out, e0, e1, e2| {
            if e0 == e1 || e0 == e2 || e1 == e2 {
                return Ok(());
            }
            // Calculate the expected value for a position
            let result = |pos: usize| {
                // If any e matches this position the resulting value there should be from the rhs
                for (i, e) in [e0, e1, e2].into_iter().enumerate() {
                    if e == pos {
                        return V_RHS[i];
                    }
                }
                // Unchanged
                V[pos]
            };
            writeln!(
                out,
                "    assert_eq!(v.with_{}{}{}(rhs3), {vec4t}({}_{t}, {}_{t}, {}_{t}, {}_{t}));",
                E[e0],
                E[e1],
                E[e2],
                result(0),
                result(1),
                result(2),
                result(3),
            )
        })?;
    }
    write_loops_vec2(out, size, |out, e0, e1| {
        writeln!(
            out,
            "    assert_eq!(v.{}{}(), {}({}_{}, {}_{}));",
            E[e0], E[e1], vec2t, V[e0], t, V[e1], t,
        )
    })?;
    if size >= 3 {
        // with_XX swizzles
        write_loops_vec2(out, size, |out, e0, e1| {
            if e0 == e1 {
                return Ok(());
            }
            // Calculate the expected value for a position
            let result = |pos: usize| {
                // If any e matches this position the resulting value there should be from the rhs
                for (i, e) in [e0, e1].into_iter().enumerate() {
                    if e == pos {
                        return V_RHS[i];
                    }
                }
                // Unchanged
                V[pos]
            };
            write!(out, "    assert_eq!(v.with_{}{}(rhs2), ", E[e0], E[e1],)?;
            if size == 3 {
                writeln!(
                    out,
                    "{vec3t}({}_{t}, {}_{t}, {}_{t}));",
                    result(0),
                    result(1),
                    result(2),
                )?;
            } else {
                writeln!(
                    out,
                    "{vec4t}({}_{t}, {}_{t}, {}_{t}, {}_{t}));",
                    result(0),
                    result(1),
                    result(2),
                    result(3),
                )?;
            }

            Ok(())
        })?;
    }
    Ok(())
}

fn write_swizzle_tests_preamble(filename: &str) -> Result<impl Write> {
    let mut out = File::create(filename)?;
    write_swizzle_head(&mut out)?;
    writeln!(
        &mut out,
        "#[macro_use]\n\
        mod support;\n\
        use glam::*;"
    )?;
    Ok(out)
}

fn write_swizzle_tests_scalar(scalar_t: &str, prefix: &str) -> Result<()> {
    let vec4_t = format!("{prefix}vec4");
    let vec3_t = format!("{prefix}vec3");
    let vec2_t = format!("{prefix}vec2");
    let mut out = write_swizzle_tests_preamble(&format!("../tests/swizzles_{scalar_t}.rs"))?;
    write_test_vec4(&mut out, scalar_t, &vec4_t, &vec3_t, &vec2_t)?;
    if scalar_t == "f32" {
        write_test_vec3(&mut out, "f32", "vec4", "vec3a", "vec2")?;
    }
    write_test_vec3(&mut out, scalar_t, &vec4_t, &vec3_t, &vec2_t)?;
    write_test_vec2(&mut out, scalar_t, &vec4_t, &vec3_t, &vec2_t)?;
    Ok(())
}

fn write_swizzle_tests() -> Result<()> {
    write_swizzle_tests_scalar("f32", "")?;
    write_swizzle_tests_scalar("f64", "d")?;

    write_swizzle_tests_scalar("i64", "i64")?;
    write_swizzle_tests_scalar("i32", "i")?;
    write_swizzle_tests_scalar("i16", "i16")?;
    write_swizzle_tests_scalar("i8", "i8")?;

    write_swizzle_tests_scalar("u64", "u64")?;
    write_swizzle_tests_scalar("u32", "u")?;
    write_swizzle_tests_scalar("u16", "u16")?;
    write_swizzle_tests_scalar("u8", "u8")?;

    write_swizzle_tests_scalar("usize", "usize")?;

    Ok(())
}

fn main() -> Result<()> {
    // Change into `./codegen` dir for convenience to the user
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;

    write_swizzle_tests()
}
