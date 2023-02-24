// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{LVec2, LVec3, LVec4, Vec2Swizzles};

impl Vec2Swizzles for LVec2 {
    type Vec3 = LVec3;

    type Vec4 = LVec4;

    #[inline]
    fn xx(self) -> LVec2 {
        LVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> LVec2 {
        LVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn yx(self) -> LVec2 {
        LVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> LVec2 {
        LVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn xxx(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xyx(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> LVec3 {
        LVec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yxx(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yyx(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> LVec3 {
        LVec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xxxx(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> LVec4 {
        LVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> LVec4 {
        LVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> LVec4 {
        LVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> LVec4 {
        LVec4::new(self.y, self.y, self.y, self.y)
    }
}
