use std::io::{self, Write};

pub fn print_vec4(out: &mut impl Write) -> std::io::Result<()> {
    const E: [char; 4] = ['x', 'y', 'z', 'w'];
    const B: [&str; 4] = ["00", "01", "10", "11"];

    writeln!(out, "impl Vec4 {{")?;

    // to vec4
    for e0 in 0..4 {
        for e1 in 0..4 {
            for e2 in 0..4 {
                for e3 in 0..4 {
                    writeln!(out, "    #[inline]")?;
                    writeln!(
                        out,
                        "    pub fn {}{}{}{}(self) -> Vec4 {{",
                        E[e0], E[e1], E[e2], E[e3]
                    )?;
                    writeln!(
                        out,
                        "        Vec4(unsafe {{ _mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_{}) }})",
                        B[e0], B[e1], B[e2], B[e3]
                    )?;
                    writeln!(out, "    }}\n")?;
                }
            }
        }
    }

    // to vec3
    for e0 in 0..4 {
        for e1 in 0..4 {
            for e2 in 0..4 {
                writeln!(out, "    #[inline]")?;
                writeln!(
                    out,
                    "    pub fn {}{}{}(self) -> Vec3 {{",
                    E[e0], E[e1], E[e2]
                )?;
                writeln!(
                    out,
                    "        Vec3::from(Vec4(unsafe {{ _mm_shuffle_ps(self.0, self.0, 0b{}_{}_{}_00)}}))",
                    B[e0], B[e1], B[e2]
                )?;
                writeln!(out, "    }}\n")?;
            }
        }
    }

    // to vec2
    for e0 in 0..4 {
        for e1 in 0..4 {
            writeln!(out, "    #[inline]")?;
            writeln!(out, "    pub fn {}{}(self) -> Vec2 {{", E[e0], E[e1])?;
            writeln!(
                out,
                "        Vec2::from(Vec4(unsafe {{ _mm_shuffle_ps(self.0, self.0, 0b{}_{}_00_00)}}))",
                B[e0], B[e1]
            )?;
            writeln!(out, "    }}\n")?;
        }
    }

    writeln!(out, "}}")
}

fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print_vec4(&mut handle)
}
