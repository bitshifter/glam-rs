// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{ULVec2, ULVec3, ULVec4, Vec2Swizzles};

impl Vec2Swizzles for ULVec2 {
    type Vec3 = ULVec3;

    type Vec4 = ULVec4;

    #[inline]
    fn xx(self) -> ULVec2 {
        ULVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> ULVec2 {
        ULVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn yx(self) -> ULVec2 {
        ULVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> ULVec2 {
        ULVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn xxx(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xyx(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> ULVec3 {
        ULVec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yxx(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yyx(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> ULVec3 {
        ULVec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xxxx(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> ULVec4 {
        ULVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> ULVec4 {
        ULVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> ULVec4 {
        ULVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> ULVec4 {
        ULVec4::new(self.y, self.y, self.y, self.y)
    }
}
